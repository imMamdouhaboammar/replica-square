/// Auto-Handoff Mechanism — artifact-gate-driven agent chaining.
///
/// After each agent completes, the HandoffEngine:
///  1. Inspects the neural graph to find newly-unblocked agents
///  2. Checks gate artifacts on disk (skips agents whose outputs exist)
///  3. Dispatches the next agent(s) in topological order
///  4. Applies the Fable Circuit Breaker (halts if failure_streak ≥ 2)
///  5. Writes a live handoff trace to `replica/handoff-trace.md`
use crate::agent::{agent_for, AgentContext, AgentOutput, SignalEvent, ToolAgent};
use crate::neural::{NeuralGraph, NodeState};
use crate::router::{AgentKind, DagNode};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

// ─── Handoff record ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandoffRecord {
    pub from_agent: String,
    pub to_agent: String,
    pub reason: HandoffReason,
    pub artifacts_carried: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HandoffReason {
    /// Normal completion — gate artifacts produced, downstream unblocked
    NormalCompletion,
    /// Skipped — gate artifacts already existed on disk
    GateAlreadySatisfied,
    /// Warning passthrough — upstream warned but downstream proceeds
    WarningPassthrough,
}

// ─── Circuit breaker state ────────────────────────────────────────────────────

#[derive(Debug, Default)]
struct CircuitBreaker {
    failure_streak: usize,
}

impl CircuitBreaker {
    const THRESHOLD: usize = 2;

    fn record_success(&mut self) {
        self.failure_streak = 0;
    }

    fn record_failure(&mut self) {
        self.failure_streak += 1;
    }

    fn is_open(&self) -> bool {
        self.failure_streak >= Self::THRESHOLD
    }
}

// ─── Execution summary ────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct PipelineResult {
    pub success: bool,
    pub agents_run: usize,
    pub agents_skipped: usize,
    pub agents_failed: usize,
    pub handoffs: Vec<HandoffRecord>,
    pub outputs: HashMap<String, AgentOutput>,
    pub circuit_breaker_tripped: bool,
    pub final_graph_md: String,
}

// ─── HandoffEngine ────────────────────────────────────────────────────────────

pub struct HandoffEngine;

impl HandoffEngine {
    /// Execute a DAG of agents with auto-handoff, gate checks, and circuit breaker.
    ///
    /// `dag` is the topologically-ordered list produced by the Router.
    /// Agents are run in dependency order; skippable nodes are skipped if gates pass.
    pub fn execute_dag(
        dag: &[DagNode],
        ctx: &AgentContext,
    ) -> Result<PipelineResult> {
        fs::create_dir_all(&ctx.replica_dir)?;

        let mut graph = NeuralGraph::full_pipeline();
        let mut circuit = CircuitBreaker::default();
        let mut handoffs: Vec<HandoffRecord> = vec![];
        let mut outputs: HashMap<String, AgentOutput> = HashMap::new();
        let mut agents_run = 0usize;
        let mut agents_skipped = 0usize;
        let mut agents_failed = 0usize;
        let mut circuit_tripped = false;

        // Build a lookup map from id → DagNode
        let node_map: HashMap<String, &DagNode> = dag.iter()
            .map(|n| (n.id.clone(), n))
            .collect();

        // Process in topological order (dag is already sorted by router)
        let ordered_ids: Vec<String> = dag.iter().map(|n| n.id.clone()).collect();

        'outer: for agent_id in &ordered_ids {
            let dag_node = match node_map.get(agent_id) {
                Some(n) => n,
                None => continue,
            };

            // ── Circuit breaker ──────────────────────────────────────────
            if circuit.is_open() {
                eprintln!("⚡ [CircuitBreaker] Halted at `{agent_id}` — failure streak ≥ {}.", CircuitBreaker::THRESHOLD);
                circuit_tripped = true;
                break 'outer;
            }

            // ── Predecessor check ────────────────────────────────────────
            if !graph.predecessors_satisfied(agent_id) {
                // A predecessor failed without warning passthrough — skip downstream
                eprintln!("⏭️  [Handoff] Skipping `{agent_id}` — predecessor not satisfied.");
                graph.node_states.insert(agent_id.clone(), NodeState::Skipped);
                agents_skipped += 1;
                continue;
            }

            // ── Build the agent ──────────────────────────────────────────
            let agent_kind = kind_from_id(agent_id);
            let agent: Box<dyn ToolAgent> = agent_for(&agent_kind);

            // ── Gate check (skip if skippable and artifacts exist) ───────
            if dag_node.skippable && agent.gate_satisfied(ctx) {
                eprintln!("⏭️  [Gate] `{agent_id}` gate satisfied — skipping.");
                graph.node_states.insert(agent_id.clone(), NodeState::Skipped);

                // Emit skip signal so downstream unblocks correctly
                use crate::agent::NeuralSignal;
                graph.receive(NeuralSignal {
                    from_agent: agent_id.clone(),
                    event: SignalEvent::Skipped,
                    payload: serde_json::json!({ "reason": "gate_already_satisfied" }),
                });

                agents_skipped += 1;
                record_handoff(&mut handoffs, agent_id, dag, HandoffReason::GateAlreadySatisfied, ctx);
                continue;
            }

            // ── Dispatch ─────────────────────────────────────────────────
            eprintln!("🚀 [Handoff] Dispatching `{}`  — {}", agent_id, dag_node.label);
            graph.mark_running(agent_id);

            match agent.run(ctx) {
                Ok(output) => {
                    let signal = output.signal.clone();
                    let event = signal.event.clone();

                    eprintln!("   ✓ `{agent_id}` — {}", output.summary);
                    graph.receive(signal);

                    if event == SignalEvent::Failed {
                        circuit.record_failure();
                        agents_failed += 1;
                    } else {
                        circuit.record_success();
                        agents_run += 1;
                        // Emit handoff trace for each downstream
                        record_handoff(&mut handoffs, agent_id, dag, HandoffReason::NormalCompletion, ctx);
                    }

                    outputs.insert(agent_id.clone(), output);
                }
                Err(e) => {
                    eprintln!("   ✗ `{agent_id}` FAILED: {e}");
                    circuit.record_failure();
                    agents_failed += 1;

                    use crate::agent::NeuralSignal;
                    graph.receive(NeuralSignal {
                        from_agent: agent_id.clone(),
                        event: SignalEvent::Failed,
                        payload: serde_json::json!({ "error": e.to_string() }),
                    });
                }
            }

            // ── Write live trace ─────────────────────────────────────────
            let trace = render_trace(&handoffs, &graph);
            let _ = fs::write(ctx.replica_dir.join("handoff-trace.md"), trace);
        }

        let final_graph_md = graph.render_status();
        let success = !graph.has_hard_failure() && !circuit_tripped;

        Ok(PipelineResult {
            success,
            agents_run,
            agents_skipped,
            agents_failed,
            handoffs,
            outputs,
            circuit_breaker_tripped: circuit_tripped,
            final_graph_md,
        })
    }
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

fn kind_from_id(id: &str) -> AgentKind {
    match id {
        "recon"        => AgentKind::Recon,
        "entrepreneur" => AgentKind::Entrepreneur,
        "architect"    => AgentKind::Architect,
        "design"       => AgentKind::Design,
        "diff"         => AgentKind::Diff,
        "sweep"        => AgentKind::Sweep,
        "launch"       => AgentKind::Launch,
        _              => AgentKind::Recon, // safe fallback
    }
}

fn record_handoff(
    handoffs: &mut Vec<HandoffRecord>,
    from_id: &str,
    dag: &[DagNode],
    reason: HandoffReason,
    ctx: &AgentContext,
) {
    // Find all downstream nodes that depend on from_id
    for node in dag {
        if node.depends_on.contains(&from_id.to_string()) {
            let artifacts: Vec<String> = ctx.replica_dir
                .read_dir()
                .map(|rd| {
                    rd.filter_map(|e| e.ok())
                      .map(|e| e.file_name().to_string_lossy().to_string())
                      .collect()
                })
                .unwrap_or_default();

            handoffs.push(HandoffRecord {
                from_agent: from_id.to_string(),
                to_agent: node.id.clone(),
                reason: reason.clone(),
                artifacts_carried: artifacts,
            });
        }
    }
}

fn render_trace(handoffs: &[HandoffRecord], graph: &NeuralGraph) -> String {
    let mut md = String::new();
    md.push_str("# 🔄 Auto-Handoff Trace\n\n");
    md.push_str(&graph.render_status());
    md.push('\n');
    md.push_str("## Handoff Chain\n\n");

    if handoffs.is_empty() {
        md.push_str("*No handoffs recorded yet.*\n");
    } else {
        for (i, h) in handoffs.iter().enumerate() {
            let reason = match h.reason {
                HandoffReason::NormalCompletion    => "✅ Normal completion",
                HandoffReason::GateAlreadySatisfied => "⏭️  Gate satisfied (skipped)",
                HandoffReason::WarningPassthrough  => "⚠️  Warning passthrough",
            };
            md.push_str(&format!(
                "{}. `{}` → `{}`  \n   Reason: {}  \n   Artifacts: {}\n\n",
                i + 1,
                h.from_agent,
                h.to_agent,
                reason,
                if h.artifacts_carried.is_empty() {
                    "none".to_string()
                } else {
                    h.artifacts_carried.join(", ")
                }
            ));
        }
    }

    md
}
