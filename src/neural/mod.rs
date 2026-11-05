/// Neural Connection Graph — models the signal bus between tool-agents.
///
/// Each agent emits a `NeuralSignal` on completion. The graph maintains:
///  - Directed edges (producer → consumer)
///  - Signal history (append-only event log)
///  - Activation state per node (Idle | Running | Complete | Failed | Skipped)
///  - Provides query methods used by the handoff mechanism
use crate::agent::{NeuralSignal, SignalEvent};
use crate::router::AgentKind;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ─── Node state ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeState {
    Idle,
    Running,
    Complete,
    Warning,  // completed with warnings — downstream may still proceed
    Failed,
    Skipped,
}

#[allow(dead_code)]
impl NodeState {
    /// Returns true if downstream edges are unblocked by this state.
    pub fn is_unblocked(&self) -> bool {
        matches!(self, NodeState::Complete | NodeState::Warning | NodeState::Skipped)
    }

    pub fn is_terminal(&self) -> bool {
        matches!(self, NodeState::Complete | NodeState::Warning | NodeState::Failed | NodeState::Skipped)
    }
}

// ─── Graph edge ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuralEdge {
    pub from: String,  // agent id
    pub to: String,    // agent id
    /// Whether a Warning signal from `from` still allows `to` to run
    pub warning_passthrough: bool,
}

// ─── Signal log entry ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalLogEntry {
    pub timestamp: DateTime<Utc>,
    pub signal: NeuralSignal,
}

// ─── Neural Graph ─────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct NeuralGraph {
    pub edges: Vec<NeuralEdge>,
    pub node_states: HashMap<String, NodeState>,
    pub signal_log: Vec<SignalLogEntry>,
}

#[allow(dead_code)]
impl NeuralGraph {
    /// Build the canonical full-pipeline neural graph.
    ///
    /// Topology (matches DAG in router):
    ///   recon ──→ entrepreneur
    ///   recon ──→ architect ──→ design ──→ sweep ──→ launch
    ///                       ──→ diff   ──→ sweep
    pub fn full_pipeline() -> Self {
        let edges = vec![
            NeuralEdge { from: "recon".into(),        to: "entrepreneur".into(), warning_passthrough: true  },
            NeuralEdge { from: "recon".into(),        to: "architect".into(),    warning_passthrough: false },
            NeuralEdge { from: "architect".into(),    to: "design".into(),       warning_passthrough: false },
            NeuralEdge { from: "architect".into(),    to: "diff".into(),         warning_passthrough: false },
            NeuralEdge { from: "design".into(),       to: "sweep".into(),        warning_passthrough: true  },
            NeuralEdge { from: "diff".into(),         to: "sweep".into(),        warning_passthrough: true  },
            NeuralEdge { from: "sweep".into(),        to: "launch".into(),       warning_passthrough: false },
        ];

        let agents = ["recon", "entrepreneur", "architect", "design", "diff", "sweep", "launch"];
        let node_states = agents.iter()
            .map(|&a| (a.to_string(), NodeState::Idle))
            .collect();

        NeuralGraph { edges, node_states, signal_log: vec![] }
    }

    /// Build a minimal graph for a single-agent run.
    pub fn single(kind: &AgentKind) -> Self {
        let id = kind.id().to_string();
        let mut node_states = HashMap::new();
        node_states.insert(id, NodeState::Idle);
        NeuralGraph { edges: vec![], node_states, signal_log: vec![] }
    }

    /// Process an incoming signal: update node state and append to log.
    pub fn receive(&mut self, signal: NeuralSignal) {
        let new_state = match signal.event {
            SignalEvent::Complete => NodeState::Complete,
            SignalEvent::Failed   => NodeState::Failed,
            SignalEvent::Skipped  => NodeState::Skipped,
            SignalEvent::Warning  => NodeState::Warning,
        };
        self.node_states.insert(signal.from_agent.clone(), new_state);
        self.signal_log.push(SignalLogEntry {
            timestamp: Utc::now(),
            signal,
        });
    }

    /// Mark a node as Running (called before dispatching an agent).
    pub fn mark_running(&mut self, agent_id: &str) {
        self.node_states.insert(agent_id.to_string(), NodeState::Running);
    }

    /// Returns true if all direct predecessors of `agent_id` are unblocked.
    pub fn predecessors_satisfied(&self, agent_id: &str) -> bool {
        let required_predecessors: Vec<&NeuralEdge> = self.edges.iter()
            .filter(|e| e.to == agent_id)
            .collect();

        if required_predecessors.is_empty() {
            return true; // root node
        }

        required_predecessors.iter().all(|edge| {
            match self.node_states.get(&edge.from) {
                Some(NodeState::Complete) | Some(NodeState::Skipped) => true,
                Some(NodeState::Warning) => edge.warning_passthrough,
                _ => false,
            }
        })
    }

    /// Returns all agent IDs that are ready to run (predecessors satisfied, not yet started).
    pub fn ready_agents(&self) -> Vec<String> {
        self.node_states.iter()
            .filter(|(id, state)| {
                **state == NodeState::Idle && self.predecessors_satisfied(id)
            })
            .map(|(id, _)| id.clone())
            .collect()
    }

    /// Returns true when all nodes have reached a terminal state.
    pub fn is_complete(&self) -> bool {
        self.node_states.values().all(|s| s.is_terminal())
    }

    /// Returns true if any node is in Failed state (and not all failures are recoverable).
    pub fn has_hard_failure(&self) -> bool {
        self.node_states.values().any(|s| *s == NodeState::Failed)
    }

    /// Render the current graph state as a Markdown status table.
    pub fn render_status(&self) -> String {
        let mut md = String::new();
        md.push_str("## 🧠 Neural Graph Status\n\n");
        md.push_str("| Agent | State | Last Signal |\n");
        md.push_str("|-------|-------|-------------|\n");

        let ordered = ["recon", "entrepreneur", "architect", "design", "diff", "sweep", "launch"];
        for id in &ordered {
            if let Some(state) = self.node_states.get(*id) {
                let icon = match state {
                    NodeState::Idle     => "⬜",
                    NodeState::Running  => "🔄",
                    NodeState::Complete => "✅",
                    NodeState::Warning  => "⚠️",
                    NodeState::Failed   => "❌",
                    NodeState::Skipped  => "⏭️",
                };
                let last = self.signal_log.iter().rev()
                    .find(|e| e.signal.from_agent == *id)
                    .map(|e| format!("`{:?}` @ {}", e.signal.event, e.timestamp.format("%H:%M:%S")))
                    .unwrap_or_else(|| "—".into());
                md.push_str(&format!("| `{id}` | {icon} `{state:?}` | {last} |\n"));
            }
        }

        md.push('\n');

        // Render edges
        md.push_str("**Connections:**\n```\n");
        for edge in &self.edges {
            let pass = if edge.warning_passthrough { " (⚠️ passthrough)" } else { "" };
            md.push_str(&format!("  {} ──→ {}{}\n", edge.from, edge.to, pass));
        }
        md.push_str("```\n");
        md
    }

    /// Export the full signal log as JSON.
    pub fn export_signal_log(&self) -> String {
        serde_json::to_string_pretty(&self.signal_log).unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::SignalEvent;

    fn signal(from: &str, event: SignalEvent) -> NeuralSignal {
        NeuralSignal {
            from_agent: from.to_string(),
            event,
            payload: serde_json::json!({}),
        }
    }

    #[test]
    fn test_root_node_ready_immediately() {
        let g = NeuralGraph::full_pipeline();
        let ready = g.ready_agents();
        assert!(ready.contains(&"recon".to_string()));
    }

    #[test]
    fn test_downstream_blocked_before_recon() {
        let g = NeuralGraph::full_pipeline();
        // architect requires recon complete
        assert!(!g.predecessors_satisfied("architect"));
    }

    #[test]
    fn test_architect_unblocked_after_recon_complete() {
        let mut g = NeuralGraph::full_pipeline();
        g.receive(signal("recon", SignalEvent::Complete));
        assert!(g.predecessors_satisfied("architect"));
    }

    #[test]
    fn test_sweep_requires_both_design_and_diff() {
        let mut g = NeuralGraph::full_pipeline();
        g.receive(signal("recon", SignalEvent::Complete));
        g.receive(signal("architect", SignalEvent::Complete));
        g.receive(signal("design", SignalEvent::Complete));
        // diff not done yet — sweep should be blocked
        assert!(!g.predecessors_satisfied("sweep"));
        g.receive(signal("diff", SignalEvent::Complete));
        assert!(g.predecessors_satisfied("sweep"));
    }

    #[test]
    fn test_warning_passthrough_on_entrepreneur() {
        let mut g = NeuralGraph::full_pipeline();
        g.receive(signal("recon", SignalEvent::Warning));
        // entrepreneur has warning_passthrough=true from recon
        assert!(g.predecessors_satisfied("entrepreneur"));
        // architect has warning_passthrough=false from recon
        assert!(!g.predecessors_satisfied("architect"));
    }

    #[test]
    fn test_completion_detection() {
        let mut g = NeuralGraph::full_pipeline();
        assert!(!g.is_complete());
        for id in ["recon", "entrepreneur", "architect", "design", "diff", "sweep", "launch"] {
            g.receive(signal(id, SignalEvent::Complete));
        }
        assert!(g.is_complete());
    }
}
