/// Tool-Agent layer — each of the 7 pipeline tools is wrapped as a typed Agent
/// that exposes a uniform `run(ctx) → AgentOutput` interface. The orchestrator
/// dispatches agents; the handoff mechanism inspects their outputs to decide
/// what fires next.
use crate::architect::ArchitectEngine;
use crate::brand::BrandEngine;
use crate::design::DesignEngine;
use crate::diff::DiffEngine;
use crate::entrepreneur::EntrepreneurEngine;
use crate::launch::{LaunchEngine, StoreMetadata};
use crate::recon::ReconEngine;
use crate::router::AgentKind;
use crate::types::{ContrastPair, DesignTokens, ReconFlow, ReconScreen};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

// ─── Agent execution context ─────────────────────────────────────────────────

/// Shared context passed into every agent.run() call.
#[derive(Debug, Clone)]
pub struct AgentContext {
    /// Target product name (e.g. "Calendly")
    pub target_name: String,
    /// Target product URL if known
    pub target_url: Option<String>,
    /// Root workspace directory (project root)
    pub workspace: PathBuf,
    /// `replica/` output directory
    pub replica_dir: PathBuf,
    /// Feature parity failure threshold (default 80.0)
    pub fail_under_parity: f64,
    /// Brand names to avoid in sweep
    pub avoid_names: Vec<String>,
    /// Domains to avoid in sweep
    pub avoid_domains: Vec<String>,
    /// Hex colors to avoid in sweep
    pub avoid_colors: Vec<String>,
    /// Store metadata for launch audit (optional)
    pub store_metadata: Option<StoreMetadata>,
}

impl AgentContext {
    pub fn new(target_name: &str, workspace: &Path) -> Self {
        let replica_dir = workspace.join("replica");
        Self {
            target_name: target_name.to_string(),
            target_url: None,
            workspace: workspace.to_path_buf(),
            replica_dir,
            fail_under_parity: 80.0,
            avoid_names: vec![target_name.to_string()],
            avoid_domains: vec![],
            avoid_colors: vec![],
            store_metadata: None,
        }
    }
}

// ─── Agent output ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentOutput {
    pub agent: String,
    pub success: bool,
    /// Wall-clock execution time in milliseconds
    pub elapsed_ms: u64,
    /// Key-value artifact paths produced
    pub artifacts: HashMap<String, String>,
    /// Human-readable summary
    pub summary: String,
    /// Downstream signal emitted into the neural graph
    pub signal: NeuralSignal,
}

/// Signal emitted on the neural bus after an agent completes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuralSignal {
    pub from_agent: String,
    pub event: SignalEvent,
    /// Structured payload (agent-specific metrics)
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SignalEvent {
    /// Agent completed; downstream may proceed
    Complete,
    /// Agent failed; downstream is blocked
    Failed,
    /// Agent was skipped (gate artifacts already exist)
    Skipped,
    /// Agent emits a warning but continues
    Warning,
}

// ─── Agent trait ──────────────────────────────────────────────────────────────

pub trait ToolAgent: Send + Sync {
    fn kind(&self) -> AgentKind;

    /// Check if the gate artifacts for this agent already exist.
    /// If true and the DAG node is skippable, the orchestrator skips execution.
    fn gate_satisfied(&self, ctx: &AgentContext) -> bool;

    /// Execute the agent. Must be pure: same context → same outputs.
    fn run(&self, ctx: &AgentContext) -> Result<AgentOutput>;
}

// ─── Helper ──────────────────────────────────────────────────────────────────

fn ok_output(
    kind: &AgentKind,
    elapsed_ms: u64,
    artifacts: HashMap<String, String>,
    summary: String,
    payload: serde_json::Value,
) -> AgentOutput {
    AgentOutput {
        agent: kind.id().to_string(),
        success: true,
        elapsed_ms,
        artifacts,
        summary,
        signal: NeuralSignal {
            from_agent: kind.id().to_string(),
            event: SignalEvent::Complete,
            payload,
        },
    }
}

#[allow(dead_code)]
fn err_output(kind: &AgentKind, elapsed_ms: u64, err: &str) -> AgentOutput {
    AgentOutput {
        agent: kind.id().to_string(),
        success: false,
        elapsed_ms,
        artifacts: HashMap::new(),
        summary: format!("FAILED: {err}"),
        signal: NeuralSignal {
            from_agent: kind.id().to_string(),
            event: SignalEvent::Failed,
            payload: serde_json::json!({ "error": err }),
        },
    }
}

// ─── 1. ReconAgent ───────────────────────────────────────────────────────────

pub struct ReconAgent;

impl ToolAgent for ReconAgent {
    fn kind(&self) -> AgentKind { AgentKind::Recon }

    fn gate_satisfied(&self, ctx: &AgentContext) -> bool {
        ctx.replica_dir.join("recon.md").exists()
            && ctx.replica_dir.join("features.csv").exists()
            && ctx.replica_dir.join("recon-map.json").exists()
    }

    fn run(&self, ctx: &AgentContext) -> Result<AgentOutput> {
        let t = Instant::now();
        fs::create_dir_all(&ctx.replica_dir)?;

        let evidence = ReconEngine::harvest_evidence(&ctx.workspace)?;

        let screens = vec![
            ReconScreen {
                screen_id: "S-HOME".into(), title: "Main Dashboard".into(),
                route_path: "/".into(), viewport: "1440x900".into(),
                interactive_elements: vec!["Create Event".into(), "Share Link".into()],
                network_calls: vec!["GET /api/events".into()],
                states: vec!["empty".into(), "populated".into()],
            },
        ];
        let flows = vec![
            ReconFlow {
                flow_id: "F-BOOKING".into(), name: "Guest Booking Journey".into(),
                step_screens: vec!["S-HOME".into()],
                happy_path: vec!["Open link".into(), "Pick slot".into(), "Confirm".into()],
                edge_cases: vec!["Concurrency clash".into()],
            },
        ];
        let constraints = vec![
            "Zero double-booking (serializable tx)".into(),
            "UTC-only timestamps".into(),
        ];

        let recon_map = ReconEngine::build_recon_map(
            &ctx.target_name,
            ctx.target_url.clone(),
            screens,
            flows,
            constraints,
            evidence,
        );

        let recon_json = serde_json::to_string_pretty(&recon_map)?;
        let recon_md = ReconEngine::render_markdown(&recon_map);
        let features_csv = ReconEngine::generate_features_csv(&recon_map);

        fs::write(ctx.replica_dir.join("recon-map.json"), &recon_json)?;
        fs::write(ctx.replica_dir.join("recon.md"), &recon_md)?;
        fs::write(ctx.replica_dir.join("features.csv"), &features_csv)?;

        let elapsed_ms = t.elapsed().as_millis() as u64;
        let mut artifacts = HashMap::new();
        artifacts.insert("recon_md".into(), ctx.replica_dir.join("recon.md").display().to_string());
        artifacts.insert("recon_map".into(), ctx.replica_dir.join("recon-map.json").display().to_string());
        artifacts.insert("features_csv".into(), ctx.replica_dir.join("features.csv").display().to_string());

        Ok(ok_output(
            &self.kind(), elapsed_ms, artifacts,
            format!("Recon complete: {} evidence items", recon_map.evidence.len()),
            serde_json::json!({ "evidence_count": recon_map.evidence.len(), "screen_count": recon_map.screens.len() }),
        ))
    }
}

// ─── 2. EntrepreneurAgent ─────────────────────────────────────────────────────

pub struct EntrepreneurAgent;

impl ToolAgent for EntrepreneurAgent {
    fn kind(&self) -> AgentKind { AgentKind::Entrepreneur }

    fn gate_satisfied(&self, ctx: &AgentContext) -> bool {
        ctx.replica_dir.join("feedback.md").exists()
    }

    fn run(&self, ctx: &AgentContext) -> Result<AgentOutput> {
        let t = Instant::now();
        let reviews_path = ctx.replica_dir.join("reviews.csv");
        if !reviews_path.exists() {
            let sample_csv = "source,url,date,rating,text\n\
                trustpilot,https://trustpilot.com/review/1,2026-05-10,1.0,\"Too expensive and hid basic features behind enterprise paywall\"\n\
                reddit,https://reddit.com/r/saas/comments/x,2026-07-15,2.0,\"Sync errors kept double booking my clients, support was useless\"\n\
                app-store,https://apps.apple.com/app/id1,2026-08-01,1.0,\"The interface is clunky and buried all the calendar settings\"\n";
            fs::write(&reviews_path, sample_csv)?;
        }
        let reviews_path_str = reviews_path.to_string_lossy().to_string();

        let feedback = EntrepreneurEngine::analyze_reviews(&reviews_path_str)?;
        let feedback_md = EntrepreneurEngine::render_markdown(&feedback);
        fs::write(ctx.replica_dir.join("feedback.md"), &feedback_md)?;

        let elapsed_ms = t.elapsed().as_millis() as u64;
        let theme_count = feedback.len();
        let mut artifacts = HashMap::new();
        artifacts.insert("feedback_md".into(), ctx.replica_dir.join("feedback.md").display().to_string());

        Ok(ok_output(
            &self.kind(), elapsed_ms, artifacts,
            format!("Entrepreneur analysis: {theme_count} grievance themes identified"),
            serde_json::json!({ "theme_count": theme_count }),
        ))
    }
}

// ─── 3. ArchitectAgent ───────────────────────────────────────────────────────

pub struct ArchitectAgent;

impl ToolAgent for ArchitectAgent {
    fn kind(&self) -> AgentKind { AgentKind::Architect }

    fn gate_satisfied(&self, ctx: &AgentContext) -> bool {
        ctx.replica_dir.join("architecture.md").exists()
            && ctx.replica_dir.join("schema.sql").exists()
    }

    fn run(&self, ctx: &AgentContext) -> Result<AgentOutput> {
        let t = Instant::now();
        let recon_map_path = ctx.replica_dir.join("recon-map.json");
        let recon_map: crate::types::ReconMap = if recon_map_path.exists() {
            let raw = fs::read_to_string(&recon_map_path)?;
            serde_json::from_str(&raw)?
        } else {
            ReconEngine::build_recon_map(&ctx.target_name, ctx.target_url.clone(), vec![], vec![], vec![], vec![])
        };

        let plan = ArchitectEngine::plan_from_recon(&recon_map);
        let arch_md = ArchitectEngine::render_markdown(&plan);
        let schema_sql = ArchitectEngine::render_sql(&plan);

        fs::write(ctx.replica_dir.join("architecture.md"), &arch_md)?;
        fs::write(ctx.replica_dir.join("schema.sql"), &schema_sql)?;

        let elapsed_ms = t.elapsed().as_millis() as u64;
        let table_count = plan.tables.len();
        let route_count = plan.routes.len();
        let mut artifacts = HashMap::new();
        artifacts.insert("architecture_md".into(), ctx.replica_dir.join("architecture.md").display().to_string());
        artifacts.insert("schema_sql".into(), ctx.replica_dir.join("schema.sql").display().to_string());

        Ok(ok_output(
            &self.kind(), elapsed_ms, artifacts,
            format!("Architecture generated: {table_count} tables, {route_count} API routes"),
            serde_json::json!({ "table_count": table_count, "route_count": route_count }),
        ))
    }
}

// ─── 4. DesignAgent ──────────────────────────────────────────────────────────

pub struct DesignAgent;

impl ToolAgent for DesignAgent {
    fn kind(&self) -> AgentKind { AgentKind::Design }

    fn gate_satisfied(&self, ctx: &AgentContext) -> bool {
        ctx.replica_dir.join("tailwind.tokens.js").exists()
    }

    fn run(&self, ctx: &AgentContext) -> Result<AgentOutput> {
        let t = Instant::now();
        let tokens_path = ctx.replica_dir.join("tokens.json");

        let tokens: DesignTokens = if tokens_path.exists() {
            let raw = fs::read_to_string(&tokens_path)?;
            serde_json::from_str(&raw)?
        } else {
            default_tokens()
        };

        let contrast_results = DesignEngine::check_tokens(&tokens);
        let tailwind_js = DesignEngine::export_tailwind(&tokens);
        let tokens_json = serde_json::to_string_pretty(&tokens)?;

        fs::write(ctx.replica_dir.join("tokens.json"), &tokens_json)?;
        fs::write(ctx.replica_dir.join("tailwind.tokens.js"), &tailwind_js)?;

        let aa_failures = contrast_results.iter().filter(|r| !r.normal_aa_pass).count();
        let elapsed_ms = t.elapsed().as_millis() as u64;
        let mut artifacts = HashMap::new();
        artifacts.insert("tailwind_js".into(), ctx.replica_dir.join("tailwind.tokens.js").display().to_string());
        artifacts.insert("tokens_json".into(), ctx.replica_dir.join("tokens.json").display().to_string());

        let event = if aa_failures > 0 { SignalEvent::Warning } else { SignalEvent::Complete };
        Ok(AgentOutput {
            agent: self.kind().id().to_string(),
            success: true,
            elapsed_ms,
            artifacts,
            summary: format!("Design audit: {}/{} pairs pass WCAG AA", contrast_results.len() - aa_failures, contrast_results.len()),
            signal: NeuralSignal {
                from_agent: self.kind().id().to_string(),
                event,
                payload: serde_json::json!({ "aa_failures": aa_failures, "total_pairs": contrast_results.len() }),
            },
        })
    }
}

fn default_tokens() -> DesignTokens {
    let mut colors = std::collections::HashMap::new();
    colors.insert("text-primary".into(), "#0f172a".into());
    colors.insert("bg-surface".into(), "#ffffff".into());
    colors.insert("brand-primary".into(), "#7c3aed".into());
    colors.insert("text-inverse".into(), "#ffffff".into());
    DesignTokens {
        colors,
        typography: Default::default(),
        spacing: Default::default(),
        pairs: vec![
            ContrastPair { fg_name: "text-primary".into(), bg_name: "bg-surface".into(), level: Some("normal".into()) },
            ContrastPair { fg_name: "text-inverse".into(), bg_name: "brand-primary".into(), level: Some("normal".into()) },
        ],
    }
}

// ─── 5. DiffAgent ────────────────────────────────────────────────────────────

pub struct DiffAgent;

impl ToolAgent for DiffAgent {
    fn kind(&self) -> AgentKind { AgentKind::Diff }

    fn gate_satisfied(&self, ctx: &AgentContext) -> bool {
        ctx.replica_dir.join("parity.md").exists()
    }

    fn run(&self, ctx: &AgentContext) -> Result<AgentOutput> {
        let t = Instant::now();
        let csv_path = ctx.replica_dir.join("features.csv");
        let report = DiffEngine::calculate_feature_parity(&csv_path)?;
        let parity_md = DiffEngine::render_markdown(&report);
        fs::write(ctx.replica_dir.join("parity.md"), &parity_md)?;

        let elapsed_ms = t.elapsed().as_millis() as u64;
        let score = report.overall_score;
        let event = if score < ctx.fail_under_parity { SignalEvent::Warning } else { SignalEvent::Complete };
        let mut artifacts = HashMap::new();
        artifacts.insert("parity_md".into(), ctx.replica_dir.join("parity.md").display().to_string());

        Ok(AgentOutput {
            agent: self.kind().id().to_string(),
            success: true,
            elapsed_ms,
            artifacts,
            summary: format!("Parity score: {score:.1}% (threshold: {:.1}%)", ctx.fail_under_parity),
            signal: NeuralSignal {
                from_agent: self.kind().id().to_string(),
                event,
                payload: serde_json::json!({
                    "parity_score": score,
                    "threshold": ctx.fail_under_parity,
                    "must_completed": report.must_completed,
                    "must_total": report.must_total,
                }),
            },
        })
    }
}

// ─── 6. SweepAgent ───────────────────────────────────────────────────────────

pub struct SweepAgent;

impl ToolAgent for SweepAgent {
    fn kind(&self) -> AgentKind { AgentKind::Sweep }

    fn gate_satisfied(&self, ctx: &AgentContext) -> bool {
        ctx.replica_dir.join("cleanroom-report.md").exists()
    }

    fn run(&self, ctx: &AgentContext) -> Result<AgentOutput> {
        let t = Instant::now();
        let report = BrandEngine::sweep_codebase(
            &ctx.workspace,
            &ctx.avoid_names,
            &ctx.avoid_domains,
            &ctx.avoid_colors,
        )?;
        let sweep_md = BrandEngine::render_markdown(&report);
        fs::write(ctx.replica_dir.join("cleanroom-report.md"), &sweep_md)?;

        let elapsed_ms = t.elapsed().as_millis() as u64;
        let violations = report.total_violations;
        let event = if !report.passed { SignalEvent::Failed } else { SignalEvent::Complete };
        let mut artifacts = HashMap::new();
        artifacts.insert("cleanroom_report".into(), ctx.replica_dir.join("cleanroom-report.md").display().to_string());

        Ok(AgentOutput {
            agent: self.kind().id().to_string(),
            success: report.passed,
            elapsed_ms,
            artifacts,
            summary: if report.passed {
                "Clean-room sweep PASSED — zero violations".into()
            } else {
                format!("Clean-room sweep BLOCKED — {violations} violations found")
            },
            signal: NeuralSignal {
                from_agent: self.kind().id().to_string(),
                event,
                payload: serde_json::json!({ "passed": report.passed, "violations": violations }),
            },
        })
    }
}

// ─── 7. LaunchAgent ──────────────────────────────────────────────────────────

pub struct LaunchAgent;

impl ToolAgent for LaunchAgent {
    fn kind(&self) -> AgentKind { AgentKind::Launch }

    fn gate_satisfied(&self, ctx: &AgentContext) -> bool {
        ctx.replica_dir.join("listing-audit.md").exists()
    }

    fn run(&self, ctx: &AgentContext) -> Result<AgentOutput> {
        let t = Instant::now();

        // Load from listing.json if it exists, otherwise use context metadata or default
        let store_meta = if let Some(meta) = &ctx.store_metadata {
            meta.clone()
        } else {
            let listing_path = ctx.replica_dir.join("listing.json");
            if listing_path.exists() {
                let raw = fs::read_to_string(&listing_path)?;
                serde_json::from_str(&raw)?
            } else {
                StoreMetadata { avoid: Some(ctx.avoid_names.clone()), app_store: None, google_play: None }
            }
        };

        let audit = LaunchEngine::audit_listing(&store_meta);
        let audit_md = LaunchEngine::render_markdown(&audit);
        fs::write(ctx.replica_dir.join("listing-audit.md"), &audit_md)?;

        let elapsed_ms = t.elapsed().as_millis() as u64;
        let errors = audit.errors.len();
        let warnings = audit.warnings.len();
        let event = if !audit.passed { SignalEvent::Failed } else { SignalEvent::Complete };
        let mut artifacts = HashMap::new();
        artifacts.insert("listing_audit".into(), ctx.replica_dir.join("listing-audit.md").display().to_string());

        Ok(AgentOutput {
            agent: self.kind().id().to_string(),
            success: audit.passed,
            elapsed_ms,
            artifacts,
            summary: format!("Launch audit: {errors} errors, {warnings} warnings"),
            signal: NeuralSignal {
                from_agent: self.kind().id().to_string(),
                event,
                payload: serde_json::json!({ "passed": audit.passed, "errors": errors, "warnings": warnings }),
            },
        })
    }
}

// ─── Agent registry ──────────────────────────────────────────────────────────

/// Return the canonical agent for a given AgentKind.
pub fn agent_for(kind: &AgentKind) -> Box<dyn ToolAgent> {
    match kind {
        AgentKind::Recon        => Box::new(ReconAgent),
        AgentKind::Entrepreneur => Box::new(EntrepreneurAgent),
        AgentKind::Architect    => Box::new(ArchitectAgent),
        AgentKind::Design       => Box::new(DesignAgent),
        AgentKind::Diff         => Box::new(DiffAgent),
        AgentKind::Sweep        => Box::new(SweepAgent),
        AgentKind::Launch       => Box::new(LaunchAgent),
    }
}
