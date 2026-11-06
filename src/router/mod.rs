/// Dynamic Intent Router — classifies raw user prompts into typed pipeline intents
/// and constructs a Directed Acyclic Graph (DAG) of agent execution nodes.
///
/// Design: RegexSet-based, zero-LLM, deterministic. Each pattern cluster maps to
/// a PhaseIntent variant. The router produces an ordered Vec<DagNode> that the
/// Orchestrator and auto-handoff mechanism consume.
use regex::{Regex, RegexSet};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ─── Intent taxonomy ────────────────────────────────────────────────────────

/// Top-level classified intent from a user prompt.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PhaseIntent {
    /// Full pipeline: recon → entrepreneur → architect → design → diff → sweep → launch
    FullPipeline,
    /// Run only the evidence-harvesting recon agent
    Recon,
    /// Run only the customer-grievance analysis agent
    Entrepreneur,
    /// Run only the technical architecture generation agent
    Architect,
    /// Run only the WCAG design-token validation agent
    Design,
    /// Run only the feature-parity scoring agent
    Diff,
    /// Run only the clean-room brand sweep agent
    Sweep,
    /// Run only the app store listing linter agent
    Launch,
    /// Resume from a specific phase (artifact gate detected mid-pipeline)
    Resume(String),
    /// No replica-related intent; router rejects
    Unknown,
}

// ─── DAG node ───────────────────────────────────────────────────────────────

/// A single node in the execution DAG.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DagNode {
    /// Unique node identifier
    pub id: String,
    /// Human label for tracing
    pub label: String,
    /// Which agent to invoke
    pub agent: AgentKind,
    /// IDs of nodes that must complete before this one runs
    pub depends_on: Vec<String>,
    /// Whether this node may be skipped if its gate artifacts already exist
    pub skippable: bool,
}

/// Canonical agent identifiers — mirrors the 7 tool-agents.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentKind {
    Recon,
    Entrepreneur,
    Architect,
    Design,
    Diff,
    Sweep,
    Launch,
}

impl AgentKind {
    pub fn id(&self) -> &'static str {
        match self {
            AgentKind::Recon => "recon",
            AgentKind::Entrepreneur => "entrepreneur",
            AgentKind::Architect => "architect",
            AgentKind::Design => "design",
            AgentKind::Diff => "diff",
            AgentKind::Sweep => "sweep",
            AgentKind::Launch => "launch",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            AgentKind::Recon => "Evidence Harvester",
            AgentKind::Entrepreneur => "Market Intelligence",
            AgentKind::Architect => "Technical Architect",
            AgentKind::Design => "WCAG Token Auditor",
            AgentKind::Diff => "Parity Scorer",
            AgentKind::Sweep => "Clean-Room Gate",
            AgentKind::Launch => "Store Listing Linter",
        }
    }
}

// ─── Routing result ──────────────────────────────────────────────────────────

/// Full routing result returned to the caller.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteResult {
    pub intent: PhaseIntent,
    pub dag: Vec<DagNode>,
    /// Extracted parameters from the prompt (target name, URL, threshold, …)
    pub params: HashMap<String, String>,
    /// Human-readable explanation of routing decision
    pub explanation: String,
}

// ─── Router ──────────────────────────────────────────────────────────────────

pub struct DynamicRouter {
    full_pipeline_patterns: RegexSet,
    recon_patterns: RegexSet,
    entrepreneur_patterns: RegexSet,
    architect_patterns: RegexSet,
    design_patterns: RegexSet,
    diff_patterns: RegexSet,
    sweep_patterns: RegexSet,
    launch_patterns: RegexSet,
    target_extractor: Regex,
    url_extractor: Regex,
    threshold_extractor: Regex,
}

impl DynamicRouter {
    pub fn new() -> Self {
        Self {
            full_pipeline_patterns: RegexSet::new([
                r"(?i)\b(clone|replicate|rebuild|replica)\b.{0,60}\b(full|complete|end.to.end|pipeline|all)\b",
                r"(?i)\b(build|create|make)\b.{0,40}\b(clone|replica|copy|alternative)\b.{0,40}\b(of|for|from)\b",
                r"(?i)\b(reverse.engineer|re-engineer)\b.{0,60}\band\b.{0,30}\b(build|create|ship)\b",
                r"(?i)\breplica.square\s+run\b",
                r"(?i)\bfull\s+(agentic\s+)?(pipeline|workflow|run)\b",
            ]).unwrap(),

            recon_patterns: RegexSet::new([
                r"(?i)\b(recon|reconnaissance|survey|map|inventory)\b.{0,50}\b(screens?|flows?|features?|app|product)\b",
                r"(?i)\b(screens?|flows?|features?)\b.{0,50}\b(inventory|map|survey|recon)\b",
                r"(?i)\bwhat\s+screens?\s+does\b",
                r"(?i)\bhow\s+does\s+\w+\s+(work|handle|manage)\b",
                r"(?i)\b(analyse|analyze|study|understand)\b.{0,40}\b(features?|flows?|ux|interface)\b",
                r"(?i)\breplica.square\s+recon\b",
            ]).unwrap(),

            entrepreneur_patterns: RegexSet::new([
                r"(?i)\b(reviews?|feedback|complaints?|grievances?)\b.{0,40}\b(analyse|analyze|mine|score|rank)\b",
                r"(?i)\b(analyse|analyze|mine|score|rank)\b.{0,40}\b(reviews?|feedback|complaints?|grievances?)\b",
                r"(?i)\bwhat\s+(users?|customers?|people)\s+(hate|dislike|complain|want|miss)\b",
                r"(?i)\b(app.store|google.play|g2|capterra)\s+reviews?\b",
                r"(?i)\bcompetitor\s+(weakness|gap|pain.point)\b",
                r"(?i)\breplica.square\s+entrepreneur\b",
            ]).unwrap(),

            architect_patterns: RegexSet::new([
                r"(?i)\b(schema|database|db|sql|postgres)\b.{0,40}\b(design|generate|create|produce|spec)\b",
                r"(?i)\b(design|generate|create|produce)\b.{0,40}\b(schema|database|db|sql|postgres)\b",
                r"(?i)\b(api|routes?|endpoints?)\b.{0,40}\b(design|contracts?|specs?|generate)\b",
                r"(?i)\b(design|generate|contract|spec)\b.{0,40}\b(api|routes?|endpoints?)\b",
                r"(?i)\b(architecture|tech.stack|system.design)\b",
                r"(?i)\breplica.square\s+architect\b",
            ]).unwrap(),

            design_patterns: RegexSet::new([
                r"(?i)\b(wcag|a11y|accessibility|contrast.ratio)\b",
                r"(?i)\b(color.tokens?|design.tokens?|tailwind.config)\b",
                r"(?i)\b(palette|brand.colors?)\b.{0,40}\b(audit|check|validate)\b",
                r"(?i)\b(audit|check|validate)\b.{0,40}\b(palette|brand.colors?|tokens?|contrast)\b",
                r"(?i)\breplica.square\s+design\b",
            ]).unwrap(),

            diff_patterns: RegexSet::new([
                r"(?i)\b(parity|coverage|completeness)\b.{0,40}\b(score|check|calculate|measure)\b",
                r"(?i)\b(score|check|calculate|measure)\b.{0,40}\b(parity|coverage|completeness)\b",
                r"(?i)\bhow\s+(complete|far|close)\b.{0,30}\b(clone|replica|build)\b",
                r"(?i)\bfeature.parity\b",
                r"(?i)\breplica.square\s+diff\b",
            ]).unwrap(),

            sweep_patterns: RegexSet::new([
                r"(?i)\b(sweep|scan|audit|check)\b.{0,40}\b(brand|trademark|ip|copyright)\b",
                r"(?i)\b(brand|trademark|ip|copyright)\b.{0,40}\b(sweep|scan|audit|check)\b",
                r"(?i)\bclean.room\b.{0,30}\b(check|audit|verify|scan|sweep)\b",
                r"(?i)\b(trademark|brand.leak|ip.leak)\b",
                r"(?i)\breplica.square\s+sweep\b",
            ]).unwrap(),

            launch_patterns: RegexSet::new([
                r"(?i)\b(app.store|google.play)\b.{0,40}\b(listing|metadata|description|keywords?)\b",
                r"(?i)\b(listing|store.metadata)\b.{0,30}\b(audit|lint|check|validate)\b",
                r"(?i)\b(audit|lint|check|validate)\b.{0,30}\b(listing|store.metadata)\b",
                r"(?i)\breplica.square\s+launch\b",
            ]).unwrap(),

            target_extractor: Regex::new(
                r#"(?i)(?:clone|replicate|rebuild|analyze|study|replica of|target[:\s]+)\s*["\']?([A-Z][a-zA-Z0-9\s\-\.]{1,40}?)["\']?(?:\s|,|$)"#
            ).unwrap(),

            url_extractor: Regex::new(
                r#"https?://[^\s"']{4,100}"#
            ).unwrap(),

            threshold_extractor: Regex::new(
                r"(?i)(?:fail.under|threshold|minimum|at.least)\s*(\d{1,3})%?|(\d{1,3})%?\s*(?:fail.under|threshold|minimum|at.least)"
            ).unwrap(),
        }
    }

    /// Classify the intent of a raw user prompt and produce a DAG.
    pub fn route(&self, prompt: &str) -> RouteResult {
        let intent = self.classify(prompt);
        let params = self.extract_params(prompt);
        let dag = self.build_dag(&intent);
        let explanation = self.explain(&intent, prompt);

        RouteResult {
            intent,
            dag,
            params,
            explanation,
        }
    }

    fn classify(&self, prompt: &str) -> PhaseIntent {
        // Priority order: most specific first
        if self.full_pipeline_patterns.is_match(prompt) {
            return PhaseIntent::FullPipeline;
        }
        if self.sweep_patterns.is_match(prompt) {
            return PhaseIntent::Sweep;
        }
        if self.launch_patterns.is_match(prompt) {
            return PhaseIntent::Launch;
        }
        if self.diff_patterns.is_match(prompt) {
            return PhaseIntent::Diff;
        }
        if self.design_patterns.is_match(prompt) {
            return PhaseIntent::Design;
        }
        if self.architect_patterns.is_match(prompt) {
            return PhaseIntent::Architect;
        }
        if self.entrepreneur_patterns.is_match(prompt) {
            return PhaseIntent::Entrepreneur;
        }
        if self.recon_patterns.is_match(prompt) {
            return PhaseIntent::Recon;
        }

        // Broad catch: any mention of a replica/clone/rebuild falls into FullPipeline
        let broad = RegexSet::new([
            r"(?i)\b(clone|replicate|rebuild|replica)\b",
            r"(?i)\b(build.my.own|build.a.copy|create.alternative)\b",
        ])
        .unwrap();
        if broad.is_match(prompt) {
            return PhaseIntent::FullPipeline;
        }

        PhaseIntent::Unknown
    }

    fn extract_params(&self, prompt: &str) -> HashMap<String, String> {
        let mut params = HashMap::new();

        if let Some(cap) = self.target_extractor.captures(prompt) {
            params.insert("target".to_string(), cap[1].trim().to_string());
        }
        if let Some(m) = self.url_extractor.find(prompt) {
            params.insert("url".to_string(), m.as_str().to_string());
        }
        if let Some(cap) = self.threshold_extractor.captures(prompt) {
            let val = cap
                .get(1)
                .or_else(|| cap.get(2))
                .map(|m| m.as_str())
                .unwrap_or("80");
            params.insert("fail_under".to_string(), val.to_string());
        }

        params
    }

    fn build_dag(&self, intent: &PhaseIntent) -> Vec<DagNode> {
        match intent {
            PhaseIntent::FullPipeline => full_pipeline_dag(),
            PhaseIntent::Recon => single_node_dag(AgentKind::Recon),
            PhaseIntent::Entrepreneur => vec![
                node(AgentKind::Recon, vec![], true),
                node(AgentKind::Entrepreneur, vec!["recon".into()], false),
            ],
            PhaseIntent::Architect => vec![
                node(AgentKind::Recon, vec![], true),
                node(AgentKind::Architect, vec!["recon".into()], false),
            ],
            PhaseIntent::Design => single_node_dag(AgentKind::Design),
            PhaseIntent::Diff => vec![
                node(AgentKind::Recon, vec![], true),
                node(AgentKind::Diff, vec!["recon".into()], false),
            ],
            PhaseIntent::Sweep => single_node_dag(AgentKind::Sweep),
            PhaseIntent::Launch => vec![
                node(AgentKind::Sweep, vec![], true),
                node(AgentKind::Launch, vec!["sweep".into()], false),
            ],
            PhaseIntent::Resume(phase) => resume_dag(phase),
            PhaseIntent::Unknown => vec![],
        }
    }

    fn explain(&self, intent: &PhaseIntent, _prompt: &str) -> String {
        match intent {
            PhaseIntent::FullPipeline =>
                "Full 7-phase pipeline: recon → entrepreneur → architect → design → diff → sweep → launch".into(),
            PhaseIntent::Recon => "Single-phase: evidence harvesting recon only".into(),
            PhaseIntent::Entrepreneur => "Two-phase: recon (skippable) → market intelligence".into(),
            PhaseIntent::Architect => "Two-phase: recon (skippable) → technical architecture".into(),
            PhaseIntent::Design => "Single-phase: WCAG token audit only".into(),
            PhaseIntent::Diff => "Two-phase: recon (skippable) → parity scoring".into(),
            PhaseIntent::Sweep => "Single-phase: clean-room brand sweep only".into(),
            PhaseIntent::Launch => "Two-phase: sweep (skippable) → store listing lint".into(),
            PhaseIntent::Resume(p) => format!("Resuming pipeline from phase: {p}"),
            PhaseIntent::Unknown => "No replica-related intent detected — router rejected".into(),
        }
    }
}

impl Default for DynamicRouter {
    fn default() -> Self {
        Self::new()
    }
}

// ─── DAG construction helpers ────────────────────────────────────────────────

fn node(agent: AgentKind, depends_on: Vec<String>, skippable: bool) -> DagNode {
    DagNode {
        id: agent.id().to_string(),
        label: agent.label().to_string(),
        depends_on,
        skippable,
        agent,
    }
}

fn single_node_dag(agent: AgentKind) -> Vec<DagNode> {
    vec![node(agent, vec![], false)]
}

fn full_pipeline_dag() -> Vec<DagNode> {
    vec![
        node(AgentKind::Recon, vec![], false),
        node(AgentKind::Entrepreneur, vec!["recon".into()], true), // optional
        node(AgentKind::Architect, vec!["recon".into()], false),
        node(AgentKind::Design, vec!["architect".into()], false),
        node(AgentKind::Diff, vec!["architect".into()], false),
        // sweep and launch both wait for diff + design
        node(
            AgentKind::Sweep,
            vec!["diff".into(), "design".into()],
            false,
        ),
        node(AgentKind::Launch, vec!["sweep".into()], false),
    ]
}

fn resume_dag(from_phase: &str) -> Vec<DagNode> {
    let all = full_pipeline_dag();
    let start_idx = all.iter().position(|n| n.id == from_phase).unwrap_or(0);
    // Return subgraph from start_idx; mark earlier nodes skippable
    all.into_iter()
        .enumerate()
        .map(|(i, mut n)| {
            if i < start_idx {
                n.skippable = true;
            }
            n
        })
        .collect()
}

// ─── Render helpers ───────────────────────────────────────────────────────────

impl RouteResult {
    /// Render as a Markdown routing trace suitable for agent logs.
    pub fn to_markdown(&self) -> String {
        let mut md = String::new();
        md.push_str("# 🧭 Router Trace\n\n");
        md.push_str(&format!("**Intent:** `{:?}`\n\n", self.intent));
        md.push_str(&format!("**Explanation:** {}\n\n", self.explanation));

        if !self.params.is_empty() {
            md.push_str("**Extracted Parameters:**\n");
            for (k, v) in &self.params {
                md.push_str(&format!("- `{k}`: `{v}`\n"));
            }
            md.push('\n');
        }

        md.push_str("**Execution DAG:**\n\n");
        md.push_str("```\n");
        for (i, node) in self.dag.iter().enumerate() {
            let skip_tag = if node.skippable { " [skippable]" } else { "" };
            let dep_tag = if node.depends_on.is_empty() {
                String::new()
            } else {
                format!(" ← depends on [{}]", node.depends_on.join(", "))
            };
            md.push_str(&format!(
                "{:>2}. {:12} │ {}{}{}\n",
                i + 1,
                node.id,
                node.label,
                dep_tag,
                skip_tag
            ));
        }
        md.push_str("```\n");
        md
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_pipeline_route() {
        let router = DynamicRouter::new();
        let r = router.route("Clone Calendly for my startup");
        assert_eq!(r.intent, PhaseIntent::FullPipeline);
        assert_eq!(r.dag.len(), 7);
    }

    #[test]
    fn test_recon_only_route() {
        let router = DynamicRouter::new();
        let r = router.route("What screens does Linear have?");
        assert_eq!(r.intent, PhaseIntent::Recon);
    }

    #[test]
    fn test_unknown_rejected() {
        let router = DynamicRouter::new();
        let r = router.route("Fix a bug in my authentication module");
        assert_eq!(r.intent, PhaseIntent::Unknown);
        assert!(r.dag.is_empty());
    }

    #[test]
    fn test_param_extraction() {
        let router = DynamicRouter::new();
        let r = router.route("Clone Calendly from https://calendly.com with 85% threshold");
        assert!(r.params.contains_key("url"));
        assert!(r.params.contains_key("fail_under"));
        assert_eq!(r.params["fail_under"], "85");
    }

    #[test]
    fn test_dag_dependency_order() {
        let router = DynamicRouter::new();
        let r = router.route("Full pipeline run for Notion");
        // sweep must depend on diff and design
        let sweep = r.dag.iter().find(|n| n.id == "sweep").unwrap();
        assert!(sweep.depends_on.contains(&"diff".to_string()));
        assert!(sweep.depends_on.contains(&"design".to_string()));
        // launch depends on sweep
        let launch = r.dag.iter().find(|n| n.id == "launch").unwrap();
        assert!(launch.depends_on.contains(&"sweep".to_string()));
    }
}
