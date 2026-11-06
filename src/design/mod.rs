use crate::types::{ContrastPair, ContrastResult, DesignTokens};
use anyhow::{anyhow, Result};

pub struct DesignEngine;

impl DesignEngine {
    /// Parse hex string (#RGB or #RRGGBB) to (r, g, b)
    pub fn parse_hex(hex_str: &str) -> Result<(u8, u8, u8)> {
        let clean = hex_str.trim().trim_start_matches('#');
        let full = match clean.len() {
            3 => {
                let mut s = String::new();
                for c in clean.chars() {
                    s.push(c);
                    s.push(c);
                }
                s
            }
            6 => clean.to_string(),
            _ => return Err(anyhow!("Invalid hex length: {}", hex_str)),
        };

        let r = u8::from_str_radix(&full[0..2], 16)?;
        let g = u8::from_str_radix(&full[2..4], 16)?;
        let b = u8::from_str_radix(&full[4..6], 16)?;
        Ok((r, g, b))
    }

    /// Calculate relative luminance according to WCAG 2.2 standard
    pub fn relative_luminance(r: u8, g: u8, b: u8) -> f64 {
        let channel = |c: u8| -> f64 {
            let v = (c as f64) / 255.0;
            if v <= 0.04045 {
                v / 12.92
            } else {
                ((v + 0.055) / 1.055).powf(2.4)
            }
        };

        0.2126 * channel(r) + 0.7152 * channel(g) + 0.0722 * channel(b)
    }

    /// Calculate contrast ratio between two colors
    pub fn contrast_ratio(hex_a: &str, hex_b: &str) -> Result<f64> {
        let (r1, g1, b1) = Self::parse_hex(hex_a)?;
        let (r2, g2, b2) = Self::parse_hex(hex_b)?;

        let l1 = Self::relative_luminance(r1, g1, b1);
        let l2 = Self::relative_luminance(r2, g2, b2);

        let (lighter, darker) = if l1 > l2 { (l1, l2) } else { (l2, l1) };
        Ok((lighter + 0.05) / (darker + 0.05))
    }

    /// Check contrast pairs against WCAG 2.2 guidelines
    pub fn check_tokens(tokens: &DesignTokens) -> Vec<ContrastResult> {
        let mut results = Vec::new();

        let pairs: Vec<ContrastPair> = if tokens.pairs.is_empty() {
            // Auto-detect pairs if none specified
            let mut detected = Vec::new();
            for fg_k in tokens.colors.keys() {
                if fg_k.contains("text") || fg_k.contains("fg") || fg_k.starts_with("on-") {
                    for bg_k in tokens.colors.keys() {
                        if bg_k.contains("bg")
                            || bg_k.contains("background")
                            || bg_k.contains("surface")
                        {
                            detected.push(ContrastPair {
                                fg_name: fg_k.clone(),
                                bg_name: bg_k.clone(),
                                level: Some("normal".to_string()),
                            });
                        }
                    }
                }
            }
            detected
        } else {
            tokens.pairs.clone()
        };

        for p in pairs {
            if let (Some(fg), Some(bg)) =
                (tokens.colors.get(&p.fg_name), tokens.colors.get(&p.bg_name))
            {
                if let Ok(ratio) = Self::contrast_ratio(fg, bg) {
                    results.push(ContrastResult {
                        fg_color: format!("{} ({})", p.fg_name, fg),
                        bg_color: format!("{} ({})", p.bg_name, bg),
                        ratio,
                        normal_aa_pass: ratio >= 4.5,
                        normal_aaa_pass: ratio >= 7.0,
                        large_aa_pass: ratio >= 3.0,
                        large_aaa_pass: ratio >= 4.5,
                    });
                }
            }
        }

        results
    }

    /// Export tokens to Tailwind CSS config JavaScript format
    pub fn export_tailwind(tokens: &DesignTokens) -> String {
        let mut out = String::new();
        out.push_str("/** @type {import('tailwindcss').Config} */\n");
        out.push_str("module.exports = {\n");
        out.push_str("  theme: {\n");
        out.push_str("    extend: {\n");
        out.push_str("      colors: {\n");
        for (k, v) in &tokens.colors {
            out.push_str(&format!("        '{}': '{}',\n", k, v));
        }
        out.push_str("      },\n");
        out.push_str("    },\n");
        out.push_str("  },\n");
        out.push_str("};\n");
        out
    }
}
