use crate::types::{FeatureStatus, ParityScoreReport, Priority};
use anyhow::{Context, Result};
use image::{GrayImage, ImageReader};
use std::fs::File;
use std::path::Path;

pub struct DiffEngine;

impl DiffEngine {
    /// Calculate functional feature parity score from features CSV
    pub fn calculate_feature_parity<P: AsRef<Path>>(csv_path: P) -> Result<ParityScoreReport> {
        let file = File::open(&csv_path)
            .with_context(|| format!("Opening features CSV {:?}", csv_path.as_ref()))?;
        let mut rdr = csv::Reader::from_reader(file);

        let mut total_possible_weight = 0.0;
        let mut total_earned_credit = 0.0;

        let mut must_completed = 0;
        let mut must_total = 0;
        let mut should_completed = 0;
        let mut should_total = 0;
        let mut could_completed = 0;
        let mut could_total = 0;
        let mut missing_musts = Vec::new();

        for result in rdr.records() {
            let record = result?;
            if record.len() < 5 {
                continue;
            }

            let feature_name = record.get(0).unwrap_or("").trim();
            let area = record.get(1).unwrap_or("").trim();
            let priority_str = record.get(2).unwrap_or("").trim();
            let original_str = record.get(3).unwrap_or("").trim().to_lowercase();
            let clone_str = record.get(4).unwrap_or("").trim();

            let priority = Priority::from_str_loose(priority_str);
            let clone_status = FeatureStatus::from_str_loose(clone_str);

            // Skip rows marked skip or where original is "no" (replica unique extensions)
            if clone_status == FeatureStatus::Skip
                || original_str == "no"
                || original_str == "false"
            {
                continue;
            }

            let weight = priority.weight();
            let credit = clone_status.credit();

            total_possible_weight += weight;
            total_earned_credit += weight * credit;

            match priority {
                Priority::Must => {
                    must_total += 1;
                    if clone_status == FeatureStatus::Yes {
                        must_completed += 1;
                    } else {
                        missing_musts.push(format!("{} (in {})", feature_name, area));
                    }
                }
                Priority::Should => {
                    should_total += 1;
                    if clone_status == FeatureStatus::Yes {
                        should_completed += 1;
                    }
                }
                Priority::Could => {
                    could_total += 1;
                    if clone_status == FeatureStatus::Yes {
                        could_completed += 1;
                    }
                }
            }
        }

        let feature_score = if total_possible_weight > 0.0 {
            (total_earned_credit / total_possible_weight) * 100.0
        } else {
            100.0
        };

        Ok(ParityScoreReport {
            feature_score: (feature_score * 10.0).round() / 10.0,
            visual_score: None,
            overall_score: (feature_score * 10.0).round() / 10.0,
            must_completed,
            must_total,
            should_completed,
            should_total,
            could_completed,
            could_total,
            missing_musts,
        })
    }

    /// Compare two screenshots and compute structural layout similarity (0.0 to 100.0)
    pub fn compute_visual_layout_parity<P: AsRef<Path>>(
        orig_path: P,
        clone_path: P,
    ) -> Result<f64> {
        let img1 = ImageReader::open(orig_path.as_ref())?.decode()?.to_luma8();
        let img2 = ImageReader::open(clone_path.as_ref())?.decode()?.to_luma8();

        let (w1, h1) = img1.dimensions();
        let (w2, h2) = img2.dimensions();

        // Scale to common width (480px default standard)
        let target_width = 480u32;
        let scale1 = target_width as f64 / w1 as f64;
        let scale2 = target_width as f64 / w2 as f64;

        let target_h1 = (h1 as f64 * scale1).round() as u32;
        let target_h2 = (h2 as f64 * scale2).round() as u32;

        let r_img1 = image::imageops::resize(
            &img1,
            target_width,
            target_h1,
            image::imageops::FilterType::Triangle,
        );
        let r_img2 = image::imageops::resize(
            &img2,
            target_width,
            target_h2,
            image::imageops::FilterType::Triangle,
        );

        let common_height = target_h1.min(target_h2);
        if common_height == 0 {
            return Ok(0.0);
        }

        // Divide into 16x16 grid cells
        let cell_size = 16u32;
        let cols = target_width / cell_size;
        let rows = common_height / cell_size;

        if cols == 0 || rows == 0 {
            return Ok(100.0);
        }

        let mut total_cells_scored = 0;
        let mut similarity_sum = 0.0;

        for r in 0..rows {
            for c in 0..cols {
                let x0 = c * cell_size;
                let y0 = r * cell_size;

                let d1 = Self::cell_edge_density(&r_img1, x0, y0, cell_size);
                let d2 = Self::cell_edge_density(&r_img2, x0, y0, cell_size);

                // If both cells are completely blank, skip padding
                if d1 > 0.01 || d2 > 0.01 {
                    total_cells_scored += 1;
                    let diff = (d1 - d2).abs();
                    let max_d = d1.max(d2);
                    let cell_sim = if max_d > 0.0 {
                        1.0 - (diff / max_d)
                    } else {
                        1.0
                    };
                    similarity_sum += cell_sim;
                }
            }
        }

        let layout_score = if total_cells_scored > 0 {
            (similarity_sum / total_cells_scored as f64) * 100.0
        } else {
            100.0
        };

        // Penalize severe height mismatch
        let height_ratio = common_height as f64 / target_h1.max(target_h2) as f64;
        let final_visual = (layout_score * height_ratio * 10.0).round() / 10.0;

        Ok(final_visual)
    }

    fn cell_edge_density(img: &GrayImage, x0: u32, y0: u32, size: u32) -> f64 {
        let mut edges = 0u32;
        let (width, height) = img.dimensions();

        for dy in 0..(size - 1) {
            for dx in 0..(size - 1) {
                let x = x0 + dx;
                let y = y0 + dy;
                if x + 1 < width && y + 1 < height {
                    let p = img.get_pixel(x, y).0[0] as i32;
                    let pr = img.get_pixel(x + 1, y).0[0] as i32;
                    let pb = img.get_pixel(x, y + 1).0[0] as i32;

                    let grad = (p - pr).abs() + (p - pb).abs();
                    if grad > 25 {
                        edges += 1;
                    }
                }
            }
        }

        edges as f64 / (size * size) as f64
    }

    /// Render Parity Markdown
    pub fn render_markdown(report: &ParityScoreReport) -> String {
        let mut md = String::new();
        md.push_str("# Replica Parity Scorecard\n\n");
        md.push_str(&format!(
            "### Overall Parity: **{:.1}%**\n\n",
            report.overall_score
        ));
        md.push_str(&format!(
            "- **Feature Functional Parity**: {:.1}%\n",
            report.feature_score
        ));
        if let Some(v) = report.visual_score {
            md.push_str(&format!("- **Visual Layout Parity**: {:.1}%\n", v));
            md.push_str("- **Formula**: `80% Features + 20% Layout`\n");
        }
        md.push('\n');

        md.push_str("## Breakdown by Priority\n\n");
        md.push_str("| Priority | Completed | Total | Completion % |\n");
        md.push_str("|:---|:---|:---|:---|\n");
        md.push_str(&format!(
            "| **Must (P0)** | {} | {} | {:.1}% |\n",
            report.must_completed,
            report.must_total,
            if report.must_total > 0 {
                (report.must_completed as f64 / report.must_total as f64) * 100.0
            } else {
                100.0
            }
        ));
        md.push_str(&format!(
            "| **Should (P1)** | {} | {} | {:.1}% |\n",
            report.should_completed,
            report.should_total,
            if report.should_total > 0 {
                (report.should_completed as f64 / report.should_total as f64) * 100.0
            } else {
                100.0
            }
        ));
        md.push_str(&format!(
            "| **Could (P2)** | {} | {} | {:.1}% |\n",
            report.could_completed,
            report.could_total,
            if report.could_total > 0 {
                (report.could_completed as f64 / report.could_total as f64) * 100.0
            } else {
                100.0
            }
        ));
        md.push('\n');

        if !report.missing_musts.is_empty() {
            md.push_str("### ⚠️ Blocking Must-Haves (P0 Missing)\n");
            for m in &report.missing_musts {
                md.push_str(&format!("- {}\n", m));
            }
        }

        md
    }
}
