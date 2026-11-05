mod agent;
mod architect;
mod brand;
mod design;
mod diff;
mod e2e;
mod entrepreneur;
mod handoff;
mod har;
mod launch;
mod mcp;
mod mock;
mod neural;
mod orchestrator;
mod recon;
mod router;
mod types;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "replica-square",
    author = "Replica-Square Contributors",
    version = "2.0.0",
    about = "Fully agentic clean-room reverse-engineering and product-replica engine"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Analyze natural language prompt and generate dynamic execution DAG
    Route {
        /// User request or intention in natural language
        prompt: String,

        /// Output format as JSON instead of Markdown
        #[arg(long)]
        json: bool,
    },

    /// Run complete autonomous end-to-end reconstruction pipeline
    Run {
        /// Optional natural language prompt to dynamically route and execute
        #[arg(short, long)]
        prompt: Option<String>,

        /// Name of the target application to reverse engineer and clone
        #[arg(short, long, default_value = "TargetApp")]
        target: String,

        /// Optional target URL or repository surface
        #[arg(short, long)]
        url: Option<String>,

        /// Working directory where artifacts will be harvested and stored
        #[arg(short, long, default_value = ".")]
        workspace: PathBuf,

        /// Minimum feature parity score percentage required to pass gate
        #[arg(long, default_value = "80.0")]
        fail_under: f64,
    },

    /// Run reconnaissance & harvest cryptographic evidence ledger
    Recon {
        /// Target surface or directory to inspect
        target: String,

        /// Output directory for recon artifacts
        #[arg(short, long, default_value = "replica")]
        out: PathBuf,
    },

    /// Generate PostgreSQL schema and API contract specifications
    Architect {
        /// Path to recon-map.json
        #[arg(default_value = "replica/recon-map.json")]
        recon_path: PathBuf,
    },

    /// Validate WCAG 2.2 color contrast and export design tokens
    Design {
        /// Path to tokens.json
        #[arg(default_value = "replica/tokens.json")]
        tokens_path: PathBuf,
    },

    /// Analyze customer reviews and mine competitor weaknesses
    Entrepreneur {
        /// Path to reviews.csv
        #[arg(default_value = "replica/reviews.csv")]
        csv_path: PathBuf,
    },

    /// Calculate functional feature parity and visual layout similarity
    Diff {
        /// Path to features.csv matrix
        #[arg(default_value = "replica/features.csv")]
        features_path: PathBuf,

        /// Optional original screenshot for visual layout comparison
        #[arg(long)]
        original_png: Option<PathBuf>,

        /// Optional clone screenshot for visual layout comparison
        #[arg(long)]
        clone_png: Option<PathBuf>,

        /// Minimum overall parity score percentage (exit 1 if below)
        #[arg(long)]
        fail_under: Option<f64>,
    },

    /// Perform clean-room trademark and brand IP sweep
    Sweep {
        /// Codebase directory to inspect
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Names to avoid (comma-separated, case-insensitive)
        #[arg(short, long, value_delimiter = ',')]
        avoid: Vec<String>,

        /// Domains to avoid (comma-separated)
        #[arg(short, long, value_delimiter = ',')]
        domains: Vec<String>,

        /// Hex colors to avoid (comma-separated, e.g. #006bff)
        #[arg(short, long, value_delimiter = ',')]
        colors: Vec<String>,
    },

    /// Audit App Store and Google Play listing metadata
    Launch {
        /// Path to listing.json metadata
        #[arg(default_value = "replica/listing.json")]
        listing_path: PathBuf,
    },

    /// Parse HAR network capture and infer API endpoints, auth, and payload schemas
    Har {
        /// Path to .har network capture file
        path: PathBuf,

        /// Output directory for inferred API specs and evidence
        #[arg(short, long, default_value = "replica")]
        out: PathBuf,
    },

    /// Run or export autonomous HTTP mock server based on synthesized API contracts
    Mock {
        /// Local TCP port for embedded mock server
        #[arg(short, long, default_value = "3000")]
        port: u16,

        /// Export standalone Bun/TypeScript mock server without starting listener
        #[arg(short, long)]
        export_only: bool,
    },

    /// Generate automated Playwright E2E and Axe-Core accessibility test specs
    TestGen {
        /// Target base URL for test runner
        #[arg(short, long, default_value = "http://localhost:3000")]
        base_url: String,

        /// Output file path for Playwright test spec
        #[arg(short, long, default_value = "e2e/flows.spec.ts")]
        out: PathBuf,
    },

    /// Start Model Context Protocol (MCP) server over stdio
    Mcp,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Route { prompt, json } => {
            let res = orchestrator::Orchestrator::route_only(&prompt);
            if json {
                println!("{}", serde_json::to_string_pretty(&res)?);
            } else {
                println!("{}", res.to_markdown());
            }
        }
        Commands::Run { prompt, target, url, workspace, fail_under } => {
            if let Some(p) = prompt {
                orchestrator::Orchestrator::run_routed(&p, workspace)?;
            } else {
                orchestrator::Orchestrator::run_full_pipeline(&target, url, workspace, fail_under)?;
            }
        }
        Commands::Recon { target, out } => {
            std::fs::create_dir_all(&out)?;
            let evidence = recon::ReconEngine::harvest_evidence(&target)?;
            println!("🔍 Discovered {} cryptographic evidence items", evidence.len());
            let recon_map = recon::ReconEngine::build_recon_map(&target, None, vec![], vec![], vec![], evidence);
            let md = recon::ReconEngine::render_markdown(&recon_map);
            std::fs::write(out.join("recon.md"), md)?;
            println!("✅ Reconnaissance map written to {}", out.join("recon.md").display());
        }
        Commands::Architect { recon_path } => {
            let data = std::fs::read_to_string(&recon_path)?;
            let map: types::ReconMap = serde_json::from_str(&data)?;
            let plan = architect::ArchitectEngine::plan_from_recon(&map);
            let sql = architect::ArchitectEngine::render_sql(&plan);
            let md = architect::ArchitectEngine::render_markdown(&plan);
            std::fs::write("replica/schema.sql", sql)?;
            std::fs::write("replica/architecture.md", md)?;
            println!("✅ Generated replica/schema.sql and replica/architecture.md");
        }
        Commands::Design { tokens_path } => {
            let data = std::fs::read_to_string(&tokens_path)?;
            let tokens: types::DesignTokens = serde_json::from_str(&data)?;
            let results = design::DesignEngine::check_tokens(&tokens);
            let tailwind = design::DesignEngine::export_tailwind(&tokens);
            std::fs::write("replica/tailwind.tokens.js", tailwind)?;

            println!("🎨 Checked {} color contrast pairs:", results.len());
            let mut failed = 0;
            for r in &results {
                let status = if r.normal_aa_pass { "PASS" } else { failed += 1; "FAIL" };
                println!("  [{}] {} on {}: {:.2}:1", status, r.fg_color, r.bg_color, r.ratio);
            }
            if failed > 0 {
                std::process::exit(1);
            }
        }
        Commands::Entrepreneur { csv_path } => {
            let themes = entrepreneur::EntrepreneurEngine::analyze_reviews(&csv_path)?;
            let md = entrepreneur::EntrepreneurEngine::render_markdown(&themes);
            std::fs::write("replica/feedback.md", md)?;
            println!("💡 Identified {} customer pain themes. Saved to replica/feedback.md", themes.len());
        }
        Commands::Diff { features_path, original_png, clone_png, fail_under } => {
            let mut report = diff::DiffEngine::calculate_feature_parity(&features_path)?;
            if let (Some(orig), Some(clone)) = (original_png, clone_png) {
                let visual = diff::DiffEngine::compute_visual_layout_parity(orig, clone)?;
                report.visual_score = Some(visual);
                report.overall_score = ((0.8 * report.feature_score + 0.2 * visual) * 10.0).round() / 10.0;
            }
            let md = diff::DiffEngine::render_markdown(&report);
            std::fs::write("replica/parity.md", md)?;
            println!("📊 Overall Parity: {:.1}% (Features: {:.1}%)", report.overall_score, report.feature_score);

            if let Some(threshold) = fail_under {
                if report.overall_score < threshold {
                    eprintln!("❌ Parity score {:.1}% is below required threshold {:.1}%", report.overall_score, threshold);
                    std::process::exit(1);
                }
            }
        }
        Commands::Sweep { path, avoid, domains, colors } => {
            let report = brand::BrandEngine::sweep_codebase(&path, &avoid, &domains, &colors)?;
            let md = brand::BrandEngine::render_markdown(&report);
            std::fs::write("replica/cleanroom-report.md", md)?;
            if report.passed {
                println!("🟢 Clean-room sweep passed: 0 brand/IP violations.");
            } else {
                eprintln!("🔴 Clean-room sweep failed with {} violations!", report.total_violations);
                std::process::exit(1);
            }
        }
        Commands::Launch { listing_path } => {
            let data = std::fs::read_to_string(&listing_path)?;
            let meta: launch::StoreMetadata = serde_json::from_str(&data)?;
            let audit = launch::LaunchEngine::audit_listing(&meta);
            let md = launch::LaunchEngine::render_markdown(&audit);
            std::fs::write("replica/listing-audit.md", md)?;
            if audit.passed {
                println!("🟢 Store listing metadata passed audit ({} warnings).", audit.warnings.len());
            } else {
                eprintln!("🔴 Store listing audit failed with {} errors!", audit.errors.len());
                std::process::exit(1);
            }
        }
        Commands::Har { path, out } => {
            std::fs::create_dir_all(&out)?;
            let report = har::HarFlowEngine::parse_har_file(&path)?;
            let md = har::HarFlowEngine::render_markdown(&report);
            let json = serde_json::to_string_pretty(&report)?;
            std::fs::write(out.join("har-catalog.md"), md)?;
            std::fs::write(out.join("har-report.json"), json)?;
            println!("🌐 Inferred {} API endpoints from {} network entries", report.endpoints.len(), report.total_entries);
            println!("✅ Reports written to {}/har-catalog.md and har-report.json", out.display());
        }
        Commands::Mock { port, export_only } => {
            let recon_path = std::path::Path::new("replica/recon-map.json");
            let routes = if recon_path.exists() {
                let data = std::fs::read_to_string(recon_path)?;
                if let Ok(map) = serde_json::from_str::<types::ReconMap>(&data) {
                    let plan = architect::ArchitectEngine::plan_from_recon(&map);
                    plan.routes
                } else {
                    vec![]
                }
            } else {
                vec![]
            };

            let out_ts = std::path::Path::new("replica/mock-server.ts");
            std::fs::create_dir_all("replica")?;
            mock::MockEngine::export_bun_mock_server(&routes, out_ts, port)?;
            println!("📦 Exported standalone Bun mock server to {}", out_ts.display());

            if !export_only {
                mock::MockEngine::run_embedded_server(routes, port).await?;
            }
        }
        Commands::TestGen { base_url, out } => {
            let recon_path = std::path::Path::new("replica/recon-map.json");
            let (flows, screens) = if recon_path.exists() {
                let data = std::fs::read_to_string(recon_path)?;
                if let Ok(map) = serde_json::from_str::<types::ReconMap>(&data) {
                    (map.flows, map.screens)
                } else {
                    (vec![], vec![])
                }
            } else {
                (vec![], vec![])
            };

            e2e::E2eGenerator::write_spec_file(&flows, &screens, &base_url, &out)?;
            println!("🧪 Generated Playwright E2E and Axe-Core accessibility spec: {}", out.display());
        }
        Commands::Mcp => {
            eprintln!("Starting Replica Agent Model Context Protocol (MCP) server over stdio...");
            mcp::McpServer::run().await?;
        }
    }

    Ok(())
}
