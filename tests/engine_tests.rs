use replica_square::architect::ArchitectEngine;
use replica_square::brand::BrandEngine;
use replica_square::design::DesignEngine;
use replica_square::diff::DiffEngine;
use replica_square::launch::{AppStoreListing, GooglePlayListing, LaunchEngine, StoreMetadata};
use replica_square::recon::ReconEngine;
use replica_square::types::{ContrastPair, DesignTokens, ReconMap};
use std::collections::HashMap;
use std::fs;

#[test]
fn test_wcag_relative_luminance_and_contrast() {
    // Black on white should have maximum contrast ~21:1
    let ratio_bw = DesignEngine::contrast_ratio("#000000", "#ffffff").expect("Contrast failed");
    assert!((ratio_bw - 21.0).abs() < 0.1);

    // White on white should be 1:1
    let ratio_ww = DesignEngine::contrast_ratio("#ffffff", "#ffffff").expect("Contrast failed");
    assert!((ratio_ww - 1.0).abs() < 0.01);

    // Common UI pair: #0f172a on #ffffff
    let ratio_text = DesignEngine::contrast_ratio("#0f172a", "#ffffff").expect("Contrast failed");
    assert!(ratio_text > 15.0);
}

#[test]
fn test_design_tokens_audit() {
    let mut colors = HashMap::new();
    colors.insert("text".to_string(), "#000000".to_string());
    colors.insert("bg".to_string(), "#ffffff".to_string());
    colors.insert("bad-text".to_string(), "#777777".to_string());
    colors.insert("bad-bg".to_string(), "#666666".to_string());

    let tokens = DesignTokens {
        colors,
        typography: HashMap::new(),
        spacing: HashMap::new(),
        pairs: vec![
            ContrastPair {
                fg_name: "text".to_string(),
                bg_name: "bg".to_string(),
                level: Some("normal".to_string()),
            },
            ContrastPair {
                fg_name: "bad-text".to_string(),
                bg_name: "bad-bg".to_string(),
                level: Some("normal".to_string()),
            },
        ],
    };

    let results = DesignEngine::check_tokens(&tokens);
    assert_eq!(results.len(), 2);
    assert!(results[0].normal_aa_pass);
    assert!(!results[1].normal_aa_pass);
}

#[test]
fn test_feature_parity_weighting() {
    let tmp_dir = std::env::temp_dir().join("test_replica_diff");
    fs::create_dir_all(&tmp_dir).unwrap();
    let csv_file = tmp_dir.join("features.csv");

    // Must = 3 weight, Should = 2 weight, Could = 1 weight
    // Total possible weight: 3 + 2 + 1 = 6
    // If Must is Yes (3*1 = 3), Should is Partial (2*0.5 = 1), Could is No (1*0 = 0)
    // Total earned credit = 4.0 / 6.0 = 66.7%
    let csv_content = "feature,area,priority,original,clone,notes\n\
        Feature A,Core,must,yes,yes,Done\n\
        Feature B,Core,should,yes,partial,WIP\n\
        Feature C,Core,could,yes,no,Todo\n\
        Feature D,Extra,must,no,yes,Replica extension (ignored in parity)\n\
        Feature E,Extra,must,yes,skip,Skipped license\n";

    fs::write(&csv_file, csv_content).unwrap();

    let report = DiffEngine::calculate_feature_parity(&csv_file).expect("Parity calculation failed");
    assert!((report.feature_score - 66.7).abs() < 0.2);
    assert_eq!(report.must_completed, 1);
    assert_eq!(report.must_total, 1);
    assert_eq!(report.should_total, 1);
}

#[test]
fn test_clean_room_brand_sweep() {
    let tmp_dir = std::env::temp_dir().join(format!("test_replica_sweep_{}", std::process::id()));
    let _ = fs::remove_dir_all(&tmp_dir);
    fs::create_dir_all(&tmp_dir).unwrap();

    let clean_file = tmp_dir.join("clean.ts");
    fs::write(&clean_file, "export const bookingTitle = 'Schedule Meeting';").unwrap();

    let report1 = BrandEngine::sweep_codebase(&tmp_dir, &["OldApp".to_string()], &["oldapp.com".to_string()], &["#006bff".to_string()]).unwrap();
    assert!(report1.passed, "Findings in clean run: {:?}", report1.findings);
    assert_eq!(report1.total_violations, 0);

    // Add dirty file with competitor brand leak
    let dirty_file = tmp_dir.join("dirty.ts");
    fs::write(&dirty_file, "import { OldAppButton } from 'oldapp.com'; const color = '#006bff';").unwrap();

    let report2 = BrandEngine::sweep_codebase(&tmp_dir, &["OldApp".to_string()], &["oldapp.com".to_string()], &["#006bff".to_string()]).unwrap();
    assert!(!report2.passed);
    assert!(report2.total_violations >= 2);

    let _ = fs::remove_dir_all(&tmp_dir);
}

#[test]
fn test_store_listing_linter() {
    // Valid listing
    let meta_valid = StoreMetadata {
        avoid: Some(vec!["Competitor".to_string()]),
        app_store: Some(AppStoreListing {
            name: Some("BookFlow".to_string()),
            subtitle: Some("Simple meeting calendar".to_string()),
            promotional_text: Some("Smart scheduling".to_string()),
            description: Some("Effortless bookings".to_string()),
            keywords: Some("calendar,schedule,meetings".to_string()),
            whats_new: Some("New features".to_string()),
        }),
        google_play: Some(GooglePlayListing {
            title: Some("BookFlow".to_string()),
            short_description: Some("Fast calendar booking app".to_string()),
            full_description: Some("Full booking description".to_string()),
        }),
    };

    let audit_valid = LaunchEngine::audit_listing(&meta_valid);
    assert!(audit_valid.passed);
    assert_eq!(audit_valid.errors.len(), 0);

    // Invalid listing: name > 30 chars, competitor leak, ranking claim
    let meta_invalid = StoreMetadata {
        avoid: Some(vec!["Competitor".to_string()]),
        app_store: Some(AppStoreListing {
            name: Some("This Super Long App Name Exceeds The Maximum Limit Of 30 Chars".to_string()),
            subtitle: Some("The #1 Competitor Alternative".to_string()),
            promotional_text: None,
            description: None,
            keywords: None,
            whats_new: None,
        }),
        google_play: None,
    };

    let audit_invalid = LaunchEngine::audit_listing(&meta_invalid);
    assert!(!audit_invalid.passed);
    assert!(audit_invalid.errors.iter().any(|e| e.field == "name"));
    assert!(audit_invalid.errors.iter().any(|e| e.message.contains("Competitor")));
    assert!(audit_invalid.warnings.iter().any(|w| w.message.contains("#1")));
}

#[test]
fn test_recon_hash_integrity() {
    let bytes = b"Replica clean room reverse engineering evidence payload";
    let hash = ReconEngine::compute_sha256(bytes);
    assert_eq!(hash.len(), 64);
    assert_eq!(hash, "44fbc7610f8cf81025f245bb5aeb654b34954042d6db1998aec04694b4d20c29");
}

#[test]
fn test_architect_sql_generation() {
    let recon = ReconMap {
        target_name: "TestTarget".to_string(),
        target_url: None,
        analyzed_at: chrono::Utc::now(),
        screens: vec![],
        flows: vec![],
        constraints: vec![],
        evidence: vec![],
    };

    let plan = ArchitectEngine::plan_from_recon(&recon);
    let sql = ArchitectEngine::render_sql(&plan);

    assert!(sql.contains("CREATE TABLE IF NOT EXISTS public.users"));
    assert!(sql.contains("ENABLE ROW LEVEL SECURITY"));
    assert!(sql.contains("PRIMARY KEY DEFAULT gen_random_uuid()"));
}
