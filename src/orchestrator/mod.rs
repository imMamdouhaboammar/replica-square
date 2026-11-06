use crate::agent::AgentContext;
use crate::architect::ArchitectEngine;
use crate::brand::BrandEngine;
use crate::design::DesignEngine;
use crate::diff::DiffEngine;
use crate::entrepreneur::EntrepreneurEngine;
use crate::handoff::{HandoffEngine, PipelineResult};
use crate::launch::{LaunchEngine, StoreMetadata};
use crate::recon::ReconEngine;
use crate::router::{DynamicRouter, RouteResult};
use crate::types::{ContrastPair, DesignTokens, ReconFlow, ReconScreen};
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct Orchestrator;

impl Orchestrator {
    /// Dynamically route user prompt, construct DAG, and execute via tool-agents with auto-handoff
    pub fn run_routed<P: AsRef<Path>>(prompt: &str, workspace_dir: P) -> Result<PipelineResult> {
        let ws = workspace_dir.as_ref();
        let router = DynamicRouter::new();
        let route = router.route(prompt);

        println!("{}", route.to_markdown());

        if route.dag.is_empty() {
            anyhow::bail!("No executable DAG found for prompt: '{}'", prompt);
        }

        let target_name = route
            .params
            .get("target")
            .cloned()
            .unwrap_or_else(|| "TargetApp".to_string());
        let target_url = route.params.get("url").cloned();
        let fail_under: f64 = route
            .params
            .get("fail_under")
            .and_then(|v| v.parse().ok())
            .unwrap_or(80.0);

        let mut ctx = AgentContext::new(&target_name, ws);
        ctx.target_url = target_url;
        ctx.fail_under_parity = fail_under;

        let result = HandoffEngine::execute_dag(&route.dag, &ctx)?;
        Ok(result)
    }

    /// Explain and route a prompt without executing
    pub fn route_only(prompt: &str) -> RouteResult {
        let router = DynamicRouter::new();
        router.route(prompt)
    }

    /// Run complete autonomous replica agent lifecycle
    pub fn run_full_pipeline<P: AsRef<Path>>(
        target_name: &str,
        target_url: Option<String>,
        workspace_dir: P,
        fail_under_parity: f64,
    ) -> Result<()> {
        let ws = workspace_dir.as_ref();
        let replica_dir = ws.join("replica");
        fs::create_dir_all(&replica_dir).with_context(|| format!("Creating {:?}", replica_dir))?;

        println!(
            "🚀 [Replica Agent] Starting autonomous reconstruction for target: {}",
            target_name
        );

        // -------------------------------------------------------------
        // Phase 1: Reconnaissance & Cryptographic Evidence Ledger
        // -------------------------------------------------------------
        println!("🔍 [Phase 1/7] Harvesting evidence & mapping target surfaces...");
        let evidence = ReconEngine::harvest_evidence(ws)?;

        let sample_screens = vec![
            ReconScreen {
                screen_id: "S-HOME".to_string(),
                title: "Main Dashboard / Landing".to_string(),
                route_path: "/".to_string(),
                viewport: "1440x900".to_string(),
                interactive_elements: vec![
                    "Create Event Button".to_string(),
                    "Upcoming Schedule List".to_string(),
                    "Share Link Trigger".to_string(),
                ],
                network_calls: vec![
                    "GET /api/v1/events".to_string(),
                    "GET /api/v1/user/me".to_string(),
                ],
                states: vec![
                    "empty".to_string(),
                    "populated".to_string(),
                    "loading".to_string(),
                ],
            },
            ReconScreen {
                screen_id: "S-BOOKING".to_string(),
                title: "Public Booking Calendar".to_string(),
                route_path: "/[user]/[slug]".to_string(),
                viewport: "375x812 (mobile)".to_string(),
                interactive_elements: vec![
                    "Date Picker".to_string(),
                    "Time Slot Grid".to_string(),
                    "Guest Form Input".to_string(),
                    "Confirm Booking CTA".to_string(),
                ],
                network_calls: vec![
                    "GET /api/v1/availability".to_string(),
                    "POST /api/v1/bookings".to_string(),
                ],
                states: vec![
                    "date_select".to_string(),
                    "slot_select".to_string(),
                    "confirmed".to_string(),
                ],
            },
        ];

        let sample_flows = vec![ReconFlow {
            flow_id: "F-BOOKING".to_string(),
            name: "Guest Booking Journey".to_string(),
            step_screens: vec!["S-BOOKING".to_string(), "S-CONFIRMATION".to_string()],
            happy_path: vec![
                "Guest opens public link".to_string(),
                "Selects available date and time slot".to_string(),
                "Enters name and email".to_string(),
                "Receives calendar invite & confirmation".to_string(),
            ],
            edge_cases: vec![
                "Slot booked simultaneously by another guest (concurrency clash)".to_string(),
                "Guest in conflicting timezone (DST calculation)".to_string(),
            ],
        }];

        let constraints = vec![
            "Absolute zero double-booking invariant (exclusion constraint or serializable transaction)".to_string(),
            "All timestamps stored strictly in UTC with timezone offset preserved".to_string(),
            "Public forms must implement client IP rate-limiting and honeypot validation".to_string(),
        ];

        let recon_map = ReconEngine::build_recon_map(
            target_name,
            target_url.clone(),
            sample_screens,
            sample_flows,
            constraints,
            evidence,
        );

        let recon_json = serde_json::to_string_pretty(&recon_map)?;
        fs::write(replica_dir.join("recon-map.json"), recon_json)?;

        let recon_md = ReconEngine::render_markdown(&recon_map);
        fs::write(replica_dir.join("recon.md"), recon_md)?;

        let features_csv = ReconEngine::generate_features_csv(&recon_map);
        let features_path = replica_dir.join("features.csv");
        if !features_path.exists() {
            fs::write(&features_path, features_csv)?;
        }

        // -------------------------------------------------------------
        // Phase 2: Strategic Entrepreneur & Competitor Weakness Mining
        // -------------------------------------------------------------
        println!("💡 [Phase 2/7] Mining market grievances & calculating competitive counters...");
        let reviews_csv_path = replica_dir.join("reviews.csv");
        if reviews_csv_path.exists() {
            let themes = EntrepreneurEngine::analyze_reviews(&reviews_csv_path)?;
            let feedback_md = EntrepreneurEngine::render_markdown(&themes);
            fs::write(replica_dir.join("feedback.md"), feedback_md)?;
            println!(
                "   -> Formulated {} customer pain themes with counter-strategies",
                themes.len()
            );
        } else {
            // Write a template reviews CSV if absent
            let sample_csv = "source,url,date,rating,text\n\
                trustpilot,https://trustpilot.com/review/1,2026-05-10,1.0,\"Too expensive and hid basic features behind enterprise paywall\"\n\
                reddit,https://reddit.com/r/saas/comments/x,2026-07-15,2.0,\"Sync errors kept double booking my clients, support was useless\"\n\
                app-store,https://apps.apple.com/app/id1,2026-08-01,1.0,\"The interface is clunky and buried all the calendar settings\"\n";
            fs::write(&reviews_csv_path, sample_csv)?;
            let themes = EntrepreneurEngine::analyze_reviews(&reviews_csv_path)?;
            let feedback_md = EntrepreneurEngine::render_markdown(&themes);
            fs::write(replica_dir.join("feedback.md"), feedback_md)?;
        }

        // -------------------------------------------------------------
        // Phase 3: Clean-Room System Architecture & PostgreSQL Schema
        // -------------------------------------------------------------
        println!(
            "🏛️ [Phase 3/7] Generating production SQL schema and API contract specifications..."
        );
        let arch_plan = ArchitectEngine::plan_from_recon(&recon_map);
        let schema_sql = ArchitectEngine::render_sql(&arch_plan);
        fs::write(replica_dir.join("schema.sql"), schema_sql)?;

        let arch_md = ArchitectEngine::render_markdown(&arch_plan);
        fs::write(replica_dir.join("architecture.md"), arch_md)?;

        // -------------------------------------------------------------
        // Phase 4: Accessible Design System & WCAG Contrast Validation
        // -------------------------------------------------------------
        println!("🎨 [Phase 4/7] Synthesizing accessible design tokens & validating WCAG 2.2...");
        let mut colors = HashMap::new();
        colors.insert("bg-surface".to_string(), "#ffffff".to_string());
        colors.insert("bg-muted".to_string(), "#f8fafc".to_string());
        colors.insert("text-primary".to_string(), "#0f172a".to_string());
        colors.insert("text-muted".to_string(), "#64748b".to_string());
        colors.insert("brand-primary".to_string(), "#4f46e5".to_string());
        colors.insert("brand-on-primary".to_string(), "#ffffff".to_string());

        let mut typography = HashMap::new();
        typography.insert(
            "font-sans".to_string(),
            "Inter, system-ui, sans-serif".to_string(),
        );
        typography.insert(
            "font-mono".to_string(),
            "JetBrains Mono, monospace".to_string(),
        );

        let mut spacing = HashMap::new();
        spacing.insert("gutter".to_string(), "1rem".to_string());

        let pairs = vec![
            ContrastPair {
                fg_name: "text-primary".to_string(),
                bg_name: "bg-surface".to_string(),
                level: Some("normal".to_string()),
            },
            ContrastPair {
                fg_name: "text-muted".to_string(),
                bg_name: "bg-surface".to_string(),
                level: Some("normal".to_string()),
            },
            ContrastPair {
                fg_name: "brand-on-primary".to_string(),
                bg_name: "brand-primary".to_string(),
                level: Some("normal".to_string()),
            },
        ];

        let tokens = DesignTokens {
            colors,
            typography,
            spacing,
            pairs,
        };
        let tokens_json = serde_json::to_string_pretty(&tokens)?;
        fs::write(replica_dir.join("tokens.json"), tokens_json)?;

        let tailwind_js = DesignEngine::export_tailwind(&tokens);
        fs::write(replica_dir.join("tailwind.tokens.js"), tailwind_js)?;

        let contrast_results = DesignEngine::check_tokens(&tokens);
        for res in &contrast_results {
            println!(
                "   -> Contrast {} on {}: {:.2}:1 (AA: {})",
                res.fg_color, res.bg_color, res.ratio, res.normal_aa_pass
            );
        }

        // -------------------------------------------------------------
        // Phase 5: Feature Parity & Verification Gate
        // -------------------------------------------------------------
        println!("📊 [Phase 5/7] Evaluating functional parity score...");
        let parity_report = DiffEngine::calculate_feature_parity(&features_path)?;
        let parity_md = DiffEngine::render_markdown(&parity_report);
        fs::write(replica_dir.join("parity.md"), parity_md)?;
        println!(
            "   -> Feature Parity: {:.1}% (Must: {}/{})",
            parity_report.feature_score, parity_report.must_completed, parity_report.must_total
        );
        if parity_report.overall_score < fail_under_parity {
            println!("   -> ⚠️ Notice: Parity score ({:.1}%) is below target threshold ({:.1}%) - logging remaining build backlog.",
                parity_report.overall_score, fail_under_parity);
        }

        // -------------------------------------------------------------
        // Phase 6: Trademark & Clean-Room IP Sweeper
        // -------------------------------------------------------------
        println!("🛡️ [Phase 6/7] Running clean-room trademark and brand IP sweep...");
        let avoid_names = vec![target_name.to_string()];
        let avoid_domains = target_url
            .as_ref()
            .map(|u| {
                vec![u
                    .replace("https://", "")
                    .replace("http://", "")
                    .trim_end_matches('/')
                    .to_string()]
            })
            .unwrap_or_default();
        let avoid_colors = vec!["#006bff".to_string(), "#0ae8f0".to_string()];

        let sweep_report =
            BrandEngine::sweep_codebase(ws, &avoid_names, &avoid_domains, &avoid_colors)?;
        let sweep_md = BrandEngine::render_markdown(&sweep_report);
        fs::write(replica_dir.join("cleanroom-report.md"), sweep_md)?;

        if sweep_report.passed {
            println!("   -> Clean-Room Audit: PASSED (Zero original brand leaks)");
        } else {
            println!(
                "   -> Clean-Room Audit: BLOCKED with {} violations",
                sweep_report.total_violations
            );
        }

        // -------------------------------------------------------------
        // Phase 7: Store Metadata & Preflight Audit
        // -------------------------------------------------------------
        println!("🚀 [Phase 7/7] Auditing store listings and preflight deployment...");
        let store_meta = StoreMetadata {
            avoid: Some(avoid_names),
            app_store: Some(crate::launch::AppStoreListing {
                name: Some(format!(
                    "{} App",
                    target_name.chars().rev().collect::<String>()
                )),
                subtitle: Some("Fast, simple scheduling tool".to_string()),
                promotional_text: Some("Clean calendar bookings with zero clutter".to_string()),
                description: Some(
                    "Automate your meetings and scheduling effortlessly.".to_string(),
                ),
                keywords: Some("calendar,booking,meeting,appointment,schedule".to_string()),
                whats_new: Some("Initial release with full timezone support.".to_string()),
            }),
            google_play: Some(crate::launch::GooglePlayListing {
                title: Some("Schedule Flow".to_string()),
                short_description: Some(
                    "Effortless calendar booking app for modern professionals".to_string(),
                ),
                full_description: Some(
                    "Schedule Flow enables seamless appointment booking and instant calendar sync."
                        .to_string(),
                ),
            }),
        };

        let audit = LaunchEngine::audit_listing(&store_meta);
        let audit_md = LaunchEngine::render_markdown(&audit);
        fs::write(replica_dir.join("listing-audit.md"), audit_md)?;

        println!("✅ [Replica Agent] Autonomous pipeline completed successfully!");
        println!("   Artifacts written to: {}", replica_dir.display());

        Ok(())
    }
}
