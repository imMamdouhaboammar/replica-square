mod architect;
mod brand;
mod design;
mod diff;
mod entrepreneur;
mod launch;
mod mcp;
mod orchestrator;
mod recon;
mod types;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "replica",
    author = "Replica Agent Contributors",
    version = "0.1.0",
    about = "Autonomous clean-room agentic reverse-engineering and replica engine"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run complete autonomous end-to-end reconstruction pipeline
    Run {
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

    /// Start Model Context Protocol (MCP) server over stdio
    Mcp,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { target, url, workspace, fail_under } => {
            orchestrator::Orchestrator::run_full_pipeline(&target, url, workspace, fail_under)?;
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
        Commands::Mcp => {
            eprintln!("Starting Replica Agent Model Context Protocol (MCP) server over stdio...");
            mcp::McpServer::run().await?;
        }
    }

    Ok(())
}
