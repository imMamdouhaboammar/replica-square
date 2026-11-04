use crate::types::FeedbackTheme;
use anyhow::{Context, Result};
use chrono::{Duration, NaiveDate, Utc};
use serde::Deserialize;
use std::collections::HashSet;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Clone, Deserialize)]
pub struct RawReview {
    pub source: String,
    pub url: String,
    pub date: Option<String>,
    pub rating: Option<f64>,
    pub text: String,
}

pub struct EntrepreneurEngine;

impl EntrepreneurEngine {
    /// Ingest reviews CSV and cluster into actionable feedback themes
    pub fn analyze_reviews<P: AsRef<Path>>(csv_path: P) -> Result<Vec<FeedbackTheme>> {
        let file = File::open(&csv_path).with_context(|| format!("Opening reviews CSV {:?}", csv_path.as_ref()))?;
        let mut rdr = csv::Reader::from_reader(file);

        let mut reviews: Vec<RawReview> = Vec::new();
        for res in rdr.deserialize() {
            let row: RawReview = res?;
            if !row.url.trim().is_empty() && !row.text.trim().is_empty() {
                reviews.push(row);
            }
        }

        // Standard thematic clusters
        let themes_def: Vec<(&str, &str, Vec<&str>, &str)> = vec![
            (
                "pricing",
                "High Pricing & Paywalled Core Features",
                vec!["price", "expensive", "paywall", "cost", "subscription", "charge", "tier", "billing"],
                "Introduce transparent flat pricing or a generous free tier with zero feature gates.",
            ),
            (
                "sync_reliability",
                "Sync Failures & Data Conflicts",
                vec!["sync", "delay", "calendar", "conflict", "offline", "disconnect", "lost", "duplicate"],
                "Implement offline-first CRDTs or strict database transaction locks with webhook idempotency.",
            ),
            (
                "performance",
                "Sluggish Load Times & Clunky Navigation",
                vec!["slow", "lag", "bloat", "crash", "freeze", "loading", "heavy", "clunky", "memory"],
                "Build on Next.js 15 Server Components with zero JavaScript client runtime for static screens.",
            ),
            (
                "ui_complexity",
                "Overwhelming UI & Buried Settings",
                vec!["confusing", "complex", "buried", "settings", "menu", "ugly", "cluttered", "hard to find"],
                "Radically simplify layout: flat hierarchy, one primary action per screen, no nested modal hell.",
            ),
            (
                "export_lockin",
                "Vendor Lock-in & Poor Data Export",
                vec!["export", "lock-in", "csv", "backup", "import", "migrate", "trap", "download"],
                "Offer 1-click full JSON/CSV data export and native standard schema migrations.",
            ),
        ];

        let mut theme_results = Vec::new();
        let now = Utc::now();

        for (theme_id, title, keywords, counter) in themes_def {
            let mut matches = Vec::new();
            let mut total_score = 0.0;
            let mut sources = HashSet::new();

            for r in &reviews {
                let text_lower = r.text.to_lowercase();
                let has_keyword = keywords.iter().any(|k| text_lower.contains(k));

                if has_keyword {
                    sources.insert(r.source.clone());

                    // Rating weight: 1 star = 1.0, 5 star = 0.1, unrated = 0.6
                    let mut weight = match r.rating.map(|x| x.round() as i64) {
                        Some(1) => 1.0,
                        Some(2) => 0.8,
                        Some(3) => 0.5,
                        Some(4) => 0.2,
                        Some(5) => 0.1,
                        _ => 0.6,
                    };

                    // Age decay: > 18 months halves the weight
                    if let Some(ref d_str) = r.date {
                        if let Ok(nd) = NaiveDate::parse_from_str(d_str.trim(), "%Y-%m-%d") {
                            let dt = nd.and_hms_opt(0, 0, 0).unwrap().and_utc();
                            if now.signed_duration_since(dt) > Duration::days(540) {
                                weight *= 0.5;
                            }
                        }
                    }

                    total_score += weight;
                    if matches.len() < 3 {
                        matches.push(format!("\"{}\" ({}, {})", r.text.trim(), r.source, r.url));
                    }
                }
            }

            if total_score > 0.0 {
                let score_f64: f64 = total_score;
                theme_results.push(FeedbackTheme {
                    theme_id: theme_id.to_string(),
                    title: title.to_string(),
                    keywords: keywords.into_iter().map(String::from).collect(),
                    total_mentions: matches.len(),
                    dissatisfaction_score: (score_f64 * 100.0).round() / 100.0,
                    sample_quotes: matches,
                    sources: sources.into_iter().collect(),
                    recommended_feature_counter: counter.to_string(),
                });
            }
        }

        // Sort descending by dissatisfaction score
        theme_results.sort_by(|a, b| b.dissatisfaction_score.partial_cmp(&a.dissatisfaction_score).unwrap());
        Ok(theme_results)
    }

    /// Render Entrepreneur Markdown
    pub fn render_markdown(themes: &[FeedbackTheme]) -> String {
        let mut md = String::new();
        md.push_str("# Entrepreneur Feedback & Strategic Edge Report\n\n");
        md.push_str("> Grounded in real customer reviews, complaints, and incumbent weaknesses.\n\n");

        md.push_str("| Rank | Theme | Mentions | Pain Score | Counter-Strategy |\n");
        md.push_str("|:---|:---|:---|:---|:---|\n");
        for (i, t) in themes.iter().enumerate() {
            md.push_str(&format!(
                "| #{} | **{}** | {} | `{}` | {} |\n",
                i + 1,
                t.title,
                t.total_mentions,
                t.dissatisfaction_score,
                t.recommended_feature_counter
            ));
        }
        md.push_str("\n");

        md.push_str("## Detailed Evidence Breakdown\n\n");
        for t in themes {
            md.push_str(&format!("### {} (`{}`)\n", t.title, t.theme_id));
            md.push_str(&format!("- **Pain Score**: {}\n", t.dissatisfaction_score));
            md.push_str(&format!("- **Sources**: {}\n", t.sources.join(", ")));
            md.push_str(&format!("- **Our Counter-Feature**: {}\n", t.recommended_feature_counter));
            md.push_str("- **Customer Quotes**:\n");
            for q in &t.sample_quotes {
                md.push_str(&format!("  * {}\n", q));
            }
            md.push_str("\n");
        }

        md
    }
}
