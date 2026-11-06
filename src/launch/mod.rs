use crate::types::{ListingAudit, ListingIssue};
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppStoreListing {
    pub name: Option<String>,
    pub subtitle: Option<String>,
    pub promotional_text: Option<String>,
    pub description: Option<String>,
    pub keywords: Option<String>,
    pub whats_new: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GooglePlayListing {
    pub title: Option<String>,
    pub short_description: Option<String>,
    pub full_description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreMetadata {
    pub avoid: Option<Vec<String>>,
    pub app_store: Option<AppStoreListing>,
    pub google_play: Option<GooglePlayListing>,
}

pub struct LaunchEngine;

impl LaunchEngine {
    /// Lint store listing metadata against Apple and Google rules
    pub fn audit_listing(meta: &StoreMetadata) -> ListingAudit {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        let claims_regex = Regex::new(
            r"(?i)(#\s?1\b|\bnumber one\b|\bno\.? ?1\b|\bbest\b|\btop[- ]rated\b|\b(app|game) of the year\b|\bfree\b|\bdiscount\b|\bsale\b|\b\d+%\s*off\b)"
        ).unwrap();

        let avoid_list = meta.avoid.clone().unwrap_or_default();

        // 1. Audit App Store
        if let Some(ref ios) = meta.app_store {
            let fields: Vec<(&str, &Option<String>, usize)> = vec![
                ("name", &ios.name, 30),
                ("subtitle", &ios.subtitle, 30),
                ("promotional_text", &ios.promotional_text, 170),
                ("description", &ios.description, 4000),
                ("keywords", &ios.keywords, 100),
                ("whats_new", &ios.whats_new, 4000),
            ];

            for (name, val_opt, max_len) in fields {
                if let Some(ref text) = val_opt {
                    let char_len = text.chars().count();
                    if char_len > max_len {
                        errors.push(ListingIssue {
                            store: "App Store".to_string(),
                            field: name.to_string(),
                            severity: "error".to_string(),
                            message: format!(
                                "Exceeds limit: {} / {} characters",
                                char_len, max_len
                            ),
                        });
                    }

                    // Check avoided names
                    for av in &avoid_list {
                        if !av.is_empty() && text.to_lowercase().contains(&av.to_lowercase()) {
                            errors.push(ListingIssue {
                                store: "App Store".to_string(),
                                field: name.to_string(),
                                severity: "error".to_string(),
                                message: format!("Contains original competitor name {:?}", av),
                            });
                        }
                    }

                    // Check ranking claims in title/subtitle
                    if (name == "name" || name == "subtitle") && claims_regex.is_match(text) {
                        warnings.push(ListingIssue {
                            store: "App Store".to_string(),
                            field: name.to_string(),
                            severity: "warning".to_string(),
                            message:
                                "Contains unverified ranking/price claim (e.g. #1, best, free)"
                                    .to_string(),
                        });
                    }
                }
            }

            // Keyword optimization check
            if let Some(ref kw) = ios.keywords {
                if kw.contains(", ") {
                    warnings.push(ListingIssue {
                        store: "App Store".to_string(),
                        field: "keywords".to_string(),
                        severity: "warning".to_string(),
                        message:
                            "Contains space after comma ', '. Remove spaces to save characters."
                                .to_string(),
                    });
                }
            }
        }

        // 2. Audit Google Play
        if let Some(ref android) = meta.google_play {
            let fields: Vec<(&str, &Option<String>, usize)> = vec![
                ("title", &android.title, 30),
                ("short_description", &android.short_description, 80),
                ("full_description", &android.full_description, 4000),
            ];

            for (name, val_opt, max_len) in fields {
                if let Some(ref text) = val_opt {
                    let char_len = text.chars().count();
                    if char_len > max_len {
                        errors.push(ListingIssue {
                            store: "Google Play".to_string(),
                            field: name.to_string(),
                            severity: "error".to_string(),
                            message: format!(
                                "Exceeds limit: {} / {} characters",
                                char_len, max_len
                            ),
                        });
                    }

                    // Check avoided names
                    for av in &avoid_list {
                        if !av.is_empty() && text.to_lowercase().contains(&av.to_lowercase()) {
                            errors.push(ListingIssue {
                                store: "Google Play".to_string(),
                                field: name.to_string(),
                                severity: "error".to_string(),
                                message: format!("Contains original competitor name {:?}", av),
                            });
                        }
                    }

                    if (name == "title" || name == "short_description")
                        && claims_regex.is_match(text)
                    {
                        warnings.push(ListingIssue {
                            store: "Google Play".to_string(),
                            field: name.to_string(),
                            severity: "warning".to_string(),
                            message: "Google Play policy prohibits ranking or promo claims in title/short desc.".to_string(),
                        });
                    }
                }
            }
        }

        let passed = errors.is_empty();
        ListingAudit {
            passed,
            errors,
            warnings,
        }
    }

    /// Render Listing Audit Markdown
    pub fn render_markdown(audit: &ListingAudit) -> String {
        let mut md = String::new();
        md.push_str("# Store Listing Compliance Audit\n\n");
        if audit.passed {
            md.push_str("### 🟢 PASSED: Listing metadata is compliant with App Store & Google Play guidelines.\n\n");
        } else {
            md.push_str(&format!(
                "### 🔴 FAILED: {} Rejection Risks Found\n\n",
                audit.errors.len()
            ));
            md.push_str("Fix all errors before submitting to store review.\n\n");
        }

        if !audit.errors.is_empty() {
            md.push_str("## Rejection Errors (Must Fix)\n\n");
            md.push_str("| Store | Field | Issue |\n");
            md.push_str("|:---|:---|:---|\n");
            for err in &audit.errors {
                md.push_str(&format!(
                    "| {} | `{}` | {} |\n",
                    err.store, err.field, err.message
                ));
            }
            md.push('\n');
        }

        if !audit.warnings.is_empty() {
            md.push_str("## Policy & Optimization Warnings\n\n");
            md.push_str("| Store | Field | Suggestion |\n");
            md.push_str("|:---|:---|:---|\n");
            for w in &audit.warnings {
                md.push_str(&format!(
                    "| {} | `{}` | {} |\n",
                    w.store, w.field, w.message
                ));
            }
            md.push('\n');
        }

        md
    }
}
