use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Global Priority for features
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Priority {
    Must,
    Should,
    Could,
}

impl Priority {
    pub fn weight(&self) -> f64 {
        match self {
            Priority::Must => 3.0,
            Priority::Should => 2.0,
            Priority::Could => 1.0,
        }
    }

    pub fn from_str_loose(s: &str) -> Self {
        match s.trim().to_lowercase().as_str() {
            "must" | "p0" => Priority::Must,
            "should" | "p1" => Priority::Should,
            _ => Priority::Could,
        }
    }
}

/// Completion status of a clone feature
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FeatureStatus {
    Yes,
    Partial,
    No,
    Skip,
}

impl FeatureStatus {
    pub fn credit(&self) -> f64 {
        match self {
            FeatureStatus::Yes => 1.0,
            FeatureStatus::Partial => 0.5,
            FeatureStatus::No => 0.0,
            FeatureStatus::Skip => 0.0,
        }
    }

    pub fn from_str_loose(s: &str) -> Self {
        match s.trim().to_lowercase().as_str() {
            "yes" | "done" | "true" => FeatureStatus::Yes,
            "partial" | "wip" | "in_progress" => FeatureStatus::Partial,
            "skip" | "ignored" => FeatureStatus::Skip,
            _ => FeatureStatus::No,
        }
    }
}

/// A single row in the feature matrix
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct FeatureRow {
    pub feature: String,
    pub area: String,
    pub priority: Priority,
    pub original: bool,
    pub clone: FeatureStatus,
    pub notes: String,
}

/// Evidence item anchoring claims to verifiable artifacts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceItem {
    pub id: String,
    pub source_type: String, // e.g. "dom_snapshot", "network_har", "binary_symbol", "screenshot"
    pub uri_or_path: String,
    pub sha256_digest: String,
    pub timestamp: DateTime<Utc>,
    pub observations: Vec<String>,
}

/// Reconnaissance Screen
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReconScreen {
    pub screen_id: String,
    pub title: String,
    pub route_path: String,
    pub viewport: String,
    pub interactive_elements: Vec<String>,
    pub network_calls: Vec<String>,
    pub states: Vec<String>,
}

/// Reconnaissance Flow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReconFlow {
    pub flow_id: String,
    pub name: String,
    pub step_screens: Vec<String>,
    pub happy_path: Vec<String>,
    pub edge_cases: Vec<String>,
}

/// Complete Reconnaissance Map
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReconMap {
    pub target_name: String,
    pub target_url: Option<String>,
    pub analyzed_at: DateTime<Utc>,
    pub screens: Vec<ReconScreen>,
    pub flows: Vec<ReconFlow>,
    pub constraints: Vec<String>,
    pub evidence: Vec<EvidenceItem>,
}

/// Database column definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbColumn {
    pub name: String,
    pub data_type: String,
    pub is_primary_key: bool,
    pub is_nullable: bool,
    pub default_value: Option<String>,
    pub check_constraint: Option<String>,
    pub references: Option<String>,
}

/// Database table definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbTable {
    pub name: String,
    pub columns: Vec<DbColumn>,
    pub indexes: Vec<String>,
    pub rls_policies: Vec<String>,
}

/// API Route definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiRoute {
    pub method: String,
    pub path: String,
    pub description: String,
    pub auth_required: bool,
    pub input_schema: String,
    pub output_schema: String,
    pub flow_id: String,
}

/// Architecture specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchitecturePlan {
    pub web_framework: String,
    pub styling_system: String,
    pub database_engine: String,
    pub orm_layer: String,
    pub auth_provider: String,
    pub tables: Vec<DbTable>,
    pub routes: Vec<ApiRoute>,
    pub build_order: Vec<String>,
    pub risk_mitigations: HashMap<String, String>,
}

/// Design Token structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignTokens {
    pub colors: HashMap<String, String>,
    pub typography: HashMap<String, String>,
    pub spacing: HashMap<String, String>,
    pub pairs: Vec<ContrastPair>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContrastPair {
    pub fg_name: String,
    pub bg_name: String,
    pub level: Option<String>, // "normal", "large", "ui"
}

/// WCAG Contrast Ratio Result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContrastResult {
    pub fg_color: String,
    pub bg_color: String,
    pub ratio: f64,
    pub normal_aa_pass: bool,
    pub normal_aaa_pass: bool,
    pub large_aa_pass: bool,
    pub large_aaa_pass: bool,
}

/// Entrepreneur Feedback theme
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackTheme {
    pub theme_id: String,
    pub title: String,
    pub keywords: Vec<String>,
    pub total_mentions: usize,
    pub dissatisfaction_score: f64,
    pub sample_quotes: Vec<String>,
    pub sources: Vec<String>,
    pub recommended_feature_counter: String,
}

/// Trademark & Clean-room finding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrandFinding {
    pub file_path: String,
    pub line_number: usize,
    pub match_type: String, // "name", "domain", "color"
    pub matched_term: String,
    pub line_snippet: String,
}

/// Brand Clearance Report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanRoomReport {
    pub passed: bool,
    pub total_violations: usize,
    pub findings: Vec<BrandFinding>,
    pub avoided_names: Vec<String>,
    pub avoided_domains: Vec<String>,
    pub avoided_colors: Vec<String>,
}

/// Parity evaluation report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParityScoreReport {
    pub feature_score: f64,
    pub visual_score: Option<f64>,
    pub overall_score: f64,
    pub must_completed: usize,
    pub must_total: usize,
    pub should_completed: usize,
    pub should_total: usize,
    pub could_completed: usize,
    pub could_total: usize,
    pub missing_musts: Vec<String>,
}

/// Store listing linter issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListingIssue {
    pub store: String,
    pub field: String,
    pub severity: String, // "error", "warning"
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListingAudit {
    pub passed: bool,
    pub errors: Vec<ListingIssue>,
    pub warnings: Vec<ListingIssue>,
}
