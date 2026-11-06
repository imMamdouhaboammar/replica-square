use crate::types::{EvidenceItem, ReconFlow, ReconMap, ReconScreen};
use anyhow::{Context, Result};
use chrono::Utc;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;

pub struct ReconEngine;

impl ReconEngine {
    /// Compute sha256 of file bytes to produce verifiable REA evidence
    pub fn compute_sha256(data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hex::encode(hasher.finalize())
    }

    /// Scan a directory or file and harvest evidence
    pub fn harvest_evidence<P: AsRef<Path>>(path: P) -> Result<Vec<EvidenceItem>> {
        let p = path.as_ref();
        let mut evidence = Vec::new();

        if p.is_file() {
            let data = fs::read(p).with_context(|| format!("Reading file {:?}", p))?;
            let digest = Self::compute_sha256(&data);
            evidence.push(EvidenceItem {
                id: format!("ev-{}", &digest[..12]),
                source_type: "file_artifact".to_string(),
                uri_or_path: p.to_string_lossy().to_string(),
                sha256_digest: format!("sha256:{}", digest),
                timestamp: Utc::now(),
                observations: vec![format!(
                    "Directly harvested from file with size {} bytes",
                    data.len()
                )],
            });
        } else if p.is_dir() {
            for entry in walkdir::WalkDir::new(p)
                .max_depth(3)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                if entry.file_type().is_file() {
                    if let Ok(data) = fs::read(entry.path()) {
                        let digest = Self::compute_sha256(&data);
                        evidence.push(EvidenceItem {
                            id: format!("ev-{}", &digest[..12]),
                            source_type: "directory_artifact".to_string(),
                            uri_or_path: entry.path().to_string_lossy().to_string(),
                            sha256_digest: format!("sha256:{}", digest),
                            timestamp: Utc::now(),
                            observations: vec![format!(
                                "Artifact discovered: {}",
                                entry.file_name().to_string_lossy()
                            )],
                        });
                    }
                }
            }
        }

        Ok(evidence)
    }

    /// Build a synthesized ReconMap from analyzed data or defaults
    pub fn build_recon_map(
        target_name: &str,
        target_url: Option<String>,
        screens: Vec<ReconScreen>,
        flows: Vec<ReconFlow>,
        constraints: Vec<String>,
        evidence: Vec<EvidenceItem>,
    ) -> ReconMap {
        ReconMap {
            target_name: target_name.to_string(),
            target_url,
            analyzed_at: Utc::now(),
            screens,
            flows,
            constraints,
            evidence,
        }
    }

    /// Render ReconMap to Markdown according to clean-room standards
    pub fn render_markdown(map: &ReconMap) -> String {
        let mut md = String::new();
        md.push_str(&format!("# Reconnaissance Map: {}\n\n", map.target_name));
        if let Some(ref url) = map.target_url {
            md.push_str(&format!("**Target URL / Surface**: {}\n", url));
        }
        md.push_str(&format!(
            "**Analyzed At**: {}\n\n",
            map.analyzed_at.to_rfc3339()
        ));

        md.push_str("## 1. Screens & Surfaces\n\n");
        md.push_str("| Screen ID | Title | Route Path | Interactive Elements | States |\n");
        md.push_str("|:---|:---|:---|:---|:---|\n");
        for s in &map.screens {
            md.push_str(&format!(
                "| `{}` | {} | `{}` | {} | {} |\n",
                s.screen_id,
                s.title,
                s.route_path,
                s.interactive_elements.join(", "),
                s.states.join(", ")
            ));
        }
        md.push('\n');

        md.push_str("## 2. User Flows\n\n");
        for f in &map.flows {
            md.push_str(&format!("### Flow: {} (`{}`)\n", f.name, f.flow_id));
            md.push_str(&format!(
                "- **Screens Involved**: {}\n",
                f.step_screens.join(" -> ")
            ));
            md.push_str("- **Happy Path**:\n");
            for step in &f.happy_path {
                md.push_str(&format!("  1. {}\n", step));
            }
            if !f.edge_cases.is_empty() {
                md.push_str("- **Edge Cases & Failure Modes**:\n");
                for ec in &f.edge_cases {
                    md.push_str(&format!("  * {}\n", ec));
                }
            }
            md.push('\n');
        }

        md.push_str("## 3. Hard Architectural Constraints\n\n");
        for c in &map.constraints {
            md.push_str(&format!("- {}\n", c));
        }
        md.push('\n');

        md.push_str("## 4. Cryptographic Evidence Ledger\n\n");
        md.push_str("| Evidence ID | Source | SHA-256 Digest | Key Observation |\n");
        md.push_str("|:---|:---|:---|:---|\n");
        for ev in &map.evidence {
            md.push_str(&format!(
                "| `{}` | `{}` | `{}` | {} |\n",
                ev.id,
                ev.source_type,
                ev.sha256_digest,
                ev.observations.first().cloned().unwrap_or_default()
            ));
        }

        md
    }

    /// Generate clean-room CSV feature matrix template from ReconMap
    pub fn generate_features_csv(map: &ReconMap) -> String {
        let mut wtr = csv::Writer::from_writer(vec![]);
        wtr.write_record(["feature", "area", "priority", "original", "clone", "notes"])
            .unwrap();

        for s in &map.screens {
            for elem in &s.interactive_elements {
                wtr.write_record([
                    elem.as_str(),
                    s.title.as_str(),
                    "must",
                    "yes",
                    "no",
                    "Extracted from recon reconnaissance",
                ])
                .unwrap();
            }
        }

        for f in &map.flows {
            wtr.write_record([
                format!("Complete {}", f.name).as_str(),
                "Core User Flow",
                "must",
                "yes",
                "no",
                "Primary end-to-end journey",
            ])
            .unwrap();
        }

        String::from_utf8(wtr.into_inner().unwrap()).unwrap()
    }
}
