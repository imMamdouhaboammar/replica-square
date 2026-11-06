use crate::types::{BrandFinding, CleanRoomReport};
use anyhow::Result;
use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

pub struct BrandEngine;

impl BrandEngine {
    /// Normalize hex string
    pub fn norm_hex(h: &str) -> String {
        let clean = h.trim().trim_start_matches('#').to_lowercase();
        if clean.len() == 3 {
            let mut s = String::new();
            for c in clean.chars() {
                s.push(c);
                s.push(c);
            }
            format!("#{}", s)
        } else {
            format!("#{}", clean)
        }
    }

    /// Run clean-room rebrand sweep across a codebase
    pub fn sweep_codebase<P: AsRef<Path>>(
        root: P,
        avoid_names: &[String],
        avoid_domains: &[String],
        avoid_colors: &[String],
    ) -> Result<CleanRoomReport> {
        let skip_dirs: HashSet<&str> = [
            ".git",
            "node_modules",
            ".next",
            ".nuxt",
            "dist",
            "build",
            "target",
            "out",
            ".turbo",
            ".cache",
            "coverage",
            "__pycache__",
            ".venv",
            "venv",
            "replica",
        ]
        .into_iter()
        .collect();

        let binary_exts: HashSet<&str> = [
            "png", "jpg", "jpeg", "gif", "webp", "ico", "pdf", "zip", "woff", "woff2", "ttf",
            "mp4", "mov", "dylib", "so", "exe", "lock",
        ]
        .into_iter()
        .collect();

        let hex_regex = Regex::new(r"#(?:[0-9a-fA-F]{6}|[0-9a-fA-F]{3})\b").unwrap();
        let norm_colors: HashSet<String> = avoid_colors.iter().map(|c| Self::norm_hex(c)).collect();

        let mut findings = Vec::new();

        for entry in walkdir::WalkDir::new(root.as_ref())
            .into_iter()
            .filter_entry(|e| !skip_dirs.contains(e.file_name().to_str().unwrap_or("")))
            .filter_map(|e| e.ok())
        {
            if !entry.file_type().is_file() {
                continue;
            }

            let path = entry.path();
            if let Some(ext) = path.extension().and_then(|x| x.to_str()) {
                if binary_exts.contains(ext.to_lowercase().as_str()) {
                    continue;
                }
            }

            // Also check path/filename itself
            let path_str = path.to_string_lossy();
            for name in avoid_names {
                if !name.is_empty() && path_str.to_lowercase().contains(&name.to_lowercase()) {
                    findings.push(BrandFinding {
                        file_path: path_str.to_string(),
                        line_number: 0,
                        match_type: "name_in_filepath".to_string(),
                        matched_term: name.clone(),
                        line_snippet: path_str.to_string(),
                    });
                }
            }

            // Read file lines
            if let Ok(content) = fs::read_to_string(path) {
                for (idx, line) in content.lines().enumerate() {
                    let line_lower = line.to_lowercase();

                    // Check names
                    for name in avoid_names {
                        if !name.is_empty() && line_lower.contains(&name.to_lowercase()) {
                            findings.push(BrandFinding {
                                file_path: path_str.to_string(),
                                line_number: idx + 1,
                                match_type: "original_brand_name".to_string(),
                                matched_term: name.clone(),
                                line_snippet: line.trim().to_string(),
                            });
                        }
                    }

                    // Check domains
                    for domain in avoid_domains {
                        if !domain.is_empty() && line_lower.contains(&domain.to_lowercase()) {
                            findings.push(BrandFinding {
                                file_path: path_str.to_string(),
                                line_number: idx + 1,
                                match_type: "original_domain".to_string(),
                                matched_term: domain.clone(),
                                line_snippet: line.trim().to_string(),
                            });
                        }
                    }

                    // Check colors
                    for cap in hex_regex.find_iter(line) {
                        let h = Self::norm_hex(cap.as_str());
                        if norm_colors.contains(&h) {
                            findings.push(BrandFinding {
                                file_path: path_str.to_string(),
                                line_number: idx + 1,
                                match_type: "original_brand_color".to_string(),
                                matched_term: h,
                                line_snippet: line.trim().to_string(),
                            });
                        }
                    }
                }
            }
        }

        let total_violations = findings.len();
        let passed = total_violations == 0;

        Ok(CleanRoomReport {
            passed,
            total_violations,
            findings,
            avoided_names: avoid_names.to_vec(),
            avoided_domains: avoid_domains.to_vec(),
            avoided_colors: avoid_colors.to_vec(),
        })
    }

    /// Render Markdown Clean-Room Clearance
    pub fn render_markdown(report: &CleanRoomReport) -> String {
        let mut md = String::new();
        md.push_str("# Clean-Room Trademark & Brand Clearance Report\n\n");
        if report.passed {
            md.push_str("### 🟢 PASSED: Codebase is 100% Clean-Room Cleared\n\n");
            md.push_str("No references to competitor brand names, registered domains, or trademark colors were found.\n\n");
        } else {
            md.push_str(&format!(
                "### 🔴 BLOCKED: {} Violations Found\n\n",
                report.total_violations
            ));
            md.push_str("Clean-room clearance cannot pass until all original brand references and colors are purged.\n\n");
            md.push_str("| File | Line | Type | Matched Term | Code Snippet |\n");
            md.push_str("|:---|:---|:---|:---|:---|\n");
            for f in &report.findings {
                md.push_str(&format!(
                    "| `{}` | {} | `{}` | `{}` | `{}` |\n",
                    f.file_path,
                    f.line_number,
                    f.match_type,
                    f.matched_term,
                    f.line_snippet.replace('|', "\\|")
                ));
            }
        }

        md
    }
}
