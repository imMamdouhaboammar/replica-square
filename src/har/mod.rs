/// HAR Flow Engine — Native HTTP Archive (HAR 1.2) parser and network flow inferrer.
///
/// Parses raw browser/proxy network recordings (.har files) to reconstruct:
///  - Unique API endpoints and methods (REST / GraphQL / RPC)
///  - Request and response JSON schema inference
///  - Authentication mechanisms (Bearer, Cookie, Custom headers)
///  - Query parameters and header contracts
///  - Cryptographic evidence anchoring (SHA-256 of request/response payloads)
///
/// Runs entirely in Rust with 0 LLM context token consumption.
use crate::types::{ApiRoute, EvidenceItem};
use anyhow::{Context, Result};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferredEndpoint {
    pub method: String,
    pub path: String,
    pub host: String,
    pub status_codes: Vec<u16>,
    pub auth_type: Option<String>,
    pub query_params: Vec<String>,
    pub request_headers: Vec<String>,
    pub sample_request_schema: Option<Value>,
    pub sample_response_schema: Option<Value>,
    pub call_count: usize,
    pub sha256_digest: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarAnalysisReport {
    pub total_entries: usize,
    pub api_entries: usize,
    pub static_assets_skipped: usize,
    pub endpoints: Vec<InferredEndpoint>,
    pub auth_summary: HashMap<String, usize>,
    pub evidence: Vec<EvidenceItem>,
}

pub struct HarFlowEngine;

impl HarFlowEngine {
    /// Parse a HAR file and infer API routes and evidence ledger
    pub fn parse_har_file<P: AsRef<Path>>(har_path: P) -> Result<HarAnalysisReport> {
        let content = fs::read_to_string(har_path.as_ref())
            .with_context(|| format!("Reading HAR file {:?}", har_path.as_ref()))?;

        let har_json: Value =
            serde_json::from_str(&content).context("Parsing HAR JSON structure")?;

        Self::analyze_har_value(&har_json)
    }

    /// Analyze a parsed HAR JSON value
    pub fn analyze_har_value(har: &Value) -> Result<HarAnalysisReport> {
        let entries = har
            .get("log")
            .and_then(|l| l.get("entries"))
            .and_then(|e| e.as_array())
            .cloned()
            .unwrap_or_default();

        let total_entries = entries.len();
        let mut static_assets_skipped = 0;
        let mut api_entries = 0;

        // Group endpoints by method + normalized path
        let mut endpoint_map: BTreeMap<(String, String), InferredEndpoint> = BTreeMap::new();
        let mut auth_summary: HashMap<String, usize> = HashMap::new();
        let mut evidence_items: Vec<EvidenceItem> = Vec::new();

        for (idx, entry) in entries.iter().enumerate() {
            let request = match entry.get("request") {
                Some(r) => r,
                None => continue,
            };

            let url_str = request.get("url").and_then(|u| u.as_str()).unwrap_or("");
            let method = request
                .get("method")
                .and_then(|m| m.as_str())
                .unwrap_or("GET")
                .to_uppercase();

            // Skip static asset files (css, js, png, jpg, woff, etc.)
            if Self::is_static_asset(url_str) {
                static_assets_skipped += 1;
                continue;
            }

            api_entries += 1;

            let (host, path, query_keys) = Self::parse_url_components(url_str);

            let status_code = entry
                .get("response")
                .and_then(|r| r.get("status"))
                .and_then(|s| s.as_u64())
                .unwrap_or(200) as u16;

            // Extract auth type
            let auth = Self::detect_auth(request);
            if let Some(ref a) = auth {
                *auth_summary.entry(a.clone()).or_insert(0) += 1;
            }

            // Extract headers
            let req_headers = request
                .get("headers")
                .and_then(|h| h.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|h| h.get("name").and_then(|n| n.as_str()))
                        .map(|s| s.to_string())
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();

            // Extract request JSON schema if available
            let req_schema = request
                .get("postData")
                .and_then(|p| p.get("text"))
                .and_then(|t| t.as_str())
                .and_then(|text| serde_json::from_str::<Value>(text).ok())
                .map(|val| Self::infer_json_schema(&val));

            // Extract response JSON schema if available
            let res_schema = entry
                .get("response")
                .and_then(|r| r.get("content"))
                .and_then(|c| c.get("text"))
                .and_then(|t| t.as_str())
                .and_then(|text| serde_json::from_str::<Value>(text).ok())
                .map(|val| Self::infer_json_schema(&val));

            // Compute SHA-256 for evidence ledger
            let mut hasher = Sha256::new();
            hasher.update(url_str.as_bytes());
            hasher.update(method.as_bytes());
            if let Some(pd) = request
                .get("postData")
                .and_then(|p| p.get("text"))
                .and_then(|t| t.as_str())
            {
                hasher.update(pd.as_bytes());
            }
            let digest = hex::encode(hasher.finalize());

            let key = (method.clone(), path.clone());
            let endpoint = endpoint_map.entry(key).or_insert_with(|| InferredEndpoint {
                method: method.clone(),
                path: path.clone(),
                host: host.clone(),
                status_codes: Vec::new(),
                auth_type: auth.clone(),
                query_params: Vec::new(),
                request_headers: Vec::new(),
                sample_request_schema: req_schema.clone(),
                sample_response_schema: res_schema.clone(),
                call_count: 0,
                sha256_digest: digest.clone(),
            });

            endpoint.call_count += 1;
            if !endpoint.status_codes.contains(&status_code) {
                endpoint.status_codes.push(status_code);
            }
            for qk in query_keys {
                if !endpoint.query_params.contains(&qk) {
                    endpoint.query_params.push(qk);
                }
            }
            for rh in req_headers {
                if !endpoint.request_headers.contains(&rh) {
                    endpoint.request_headers.push(rh);
                }
            }

            // Create cryptographic evidence entry
            evidence_items.push(EvidenceItem {
                id: format!("HAR-{:04}", idx + 1),
                source_type: "har_network_capture".to_string(),
                uri_or_path: url_str.to_string(),
                sha256_digest: digest,
                timestamp: Utc::now(),
                observations: vec![
                    format!("Method: {}", method),
                    format!("Status: {}", status_code),
                    format!("Auth: {}", auth.as_deref().unwrap_or("None")),
                ],
            });
        }

        let endpoints: Vec<InferredEndpoint> = endpoint_map.into_values().collect();

        Ok(HarAnalysisReport {
            total_entries,
            api_entries,
            static_assets_skipped,
            endpoints,
            auth_summary,
            evidence: evidence_items,
        })
    }

    /// Convert InferredEndpoints into production ApiRoutes
    #[allow(dead_code)]
    pub fn to_api_routes(report: &HarAnalysisReport) -> Vec<ApiRoute> {
        report
            .endpoints
            .iter()
            .map(|ep| {
                let auth_req = ep.auth_type.is_some();
                let input_schema = ep
                    .sample_request_schema
                    .as_ref()
                    .map(|s| serde_json::to_string(s).unwrap_or_else(|_| "{}".to_string()))
                    .unwrap_or_else(|| "{}".to_string());

                let output_schema = ep
                    .sample_response_schema
                    .as_ref()
                    .map(|s| serde_json::to_string(s).unwrap_or_else(|_| "{}".to_string()))
                    .unwrap_or_else(|| "{}".to_string());

                ApiRoute {
                    method: ep.method.clone(),
                    path: ep.path.clone(),
                    description: format!(
                        "Inferred from {} capture(s) on host {}",
                        ep.call_count, ep.host
                    ),
                    auth_required: auth_req,
                    input_schema,
                    output_schema,
                    flow_id: "INFERRED-HAR".to_string(),
                }
            })
            .collect()
    }

    /// Render human-readable Markdown specification of captured endpoints
    pub fn render_markdown(report: &HarAnalysisReport) -> String {
        let mut md = String::new();
        md.push_str("# 🌐 Network Flow & Inferred API Catalog\n\n");
        md.push_str(&format!(
            "- **Total Entries Processed:** {}\n",
            report.total_entries
        ));
        md.push_str(&format!(
            "- **API Endpoints Identified:** {}\n",
            report.endpoints.len()
        ));
        md.push_str(&format!(
            "- **Static Assets Excluded:** {}\n\n",
            report.static_assets_skipped
        ));

        md.push_str("## Inferred API Endpoints\n\n");
        md.push_str("| Method | Path | Status Codes | Auth | Calls | SHA-256 |\n");
        md.push_str("|:---|:---|:---|:---|:---:|:---:|\n");

        for ep in &report.endpoints {
            let statuses = ep
                .status_codes
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join(", ");

            let short_hash = if ep.sha256_digest.len() >= 8 {
                &ep.sha256_digest[0..8]
            } else {
                &ep.sha256_digest
            };

            md.push_str(&format!(
                "| `{}` | `{}` | {} | {} | {} | `{}` |\n",
                ep.method,
                ep.path,
                statuses,
                ep.auth_type.as_deref().unwrap_or("None"),
                ep.call_count,
                short_hash
            ));
        }

        md.push_str("\n## Detailed Endpoint Schemas\n\n");
        for ep in &report.endpoints {
            md.push_str(&format!("### `{} {}`\n\n", ep.method, ep.path));
            if !ep.query_params.is_empty() {
                md.push_str(&format!(
                    "- **Query Parameters:** `{}`\n",
                    ep.query_params.join("`, `")
                ));
            }
            if let Some(ref req_s) = ep.sample_request_schema {
                md.push_str(&format!(
                    "- **Request Body Schema:**\n```json\n{}\n```\n",
                    serde_json::to_string_pretty(req_s).unwrap_or_default()
                ));
            }
            if let Some(ref res_s) = ep.sample_response_schema {
                md.push_str(&format!(
                    "- **Response Schema:**\n```json\n{}\n```\n",
                    serde_json::to_string_pretty(res_s).unwrap_or_default()
                ));
            }
            md.push('\n');
        }

        md
    }

    fn is_static_asset(url: &str) -> bool {
        let clean = url.split('?').next().unwrap_or(url).to_lowercase();
        let exts = [
            ".css", ".js", ".png", ".jpg", ".jpeg", ".gif", ".svg", ".ico", ".woff", ".woff2",
            ".ttf", ".eot", ".map", ".webp", ".mp4", ".mp3",
        ];
        exts.iter().any(|ext| clean.ends_with(ext))
    }

    fn parse_url_components(url_str: &str) -> (String, String, Vec<String>) {
        let no_scheme = url_str
            .strip_prefix("https://")
            .or_else(|| url_str.strip_prefix("http://"))
            .unwrap_or(url_str);

        let parts: Vec<&str> = no_scheme.splitn(2, '/').collect();
        let host = parts.first().copied().unwrap_or("").to_string();
        let path_and_query = parts.get(1).copied().unwrap_or("");

        let pq_parts: Vec<&str> = path_and_query.splitn(2, '?').collect();
        let path = format!("/{}", pq_parts.first().copied().unwrap_or(""));

        let query_keys = pq_parts
            .get(1)
            .map(|q| {
                q.split('&')
                    .filter_map(|pair| pair.split('=').next())
                    .map(|k| k.to_string())
                    .collect()
            })
            .unwrap_or_default();

        (host, path, query_keys)
    }

    fn detect_auth(request: &Value) -> Option<String> {
        let headers = request.get("headers").and_then(|h| h.as_array())?;
        for h in headers {
            let name = h
                .get("name")
                .and_then(|n| n.as_str())
                .unwrap_or("")
                .to_lowercase();
            let val = h.get("value").and_then(|v| v.as_str()).unwrap_or("");

            if name == "authorization" {
                if val.to_lowercase().starts_with("bearer ") {
                    return Some("Bearer JWT".to_string());
                } else if val.to_lowercase().starts_with("basic ") {
                    return Some("HTTP Basic".to_string());
                } else {
                    return Some("Custom Authorization".to_string());
                }
            } else if name == "x-api-key" || name == "apikey" {
                return Some("API Key Header".to_string());
            } else if name == "cookie"
                && (val.contains("session") || val.contains("token") || val.contains("auth"))
            {
                return Some("Session Cookie".to_string());
            }
        }
        None
    }

    fn infer_json_schema(val: &Value) -> Value {
        match val {
            Value::Null => serde_json::json!({ "type": "null" }),
            Value::Bool(_) => serde_json::json!({ "type": "boolean" }),
            Value::Number(n) => {
                if n.is_i64() || n.is_u64() {
                    serde_json::json!({ "type": "integer" })
                } else {
                    serde_json::json!({ "type": "number" })
                }
            }
            Value::String(_) => serde_json::json!({ "type": "string" }),
            Value::Array(arr) => {
                let item_type = if let Some(first) = arr.first() {
                    Self::infer_json_schema(first)
                } else {
                    serde_json::json!({ "type": "unknown" })
                };
                serde_json::json!({
                    "type": "array",
                    "items": item_type
                })
            }
            Value::Object(map) => {
                let mut props = serde_json::Map::new();
                for (k, v) in map {
                    props.insert(k.clone(), Self::infer_json_schema(v));
                }
                serde_json::json!({
                    "type": "object",
                    "properties": props
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_har_analysis_and_schema_inference() {
        let sample_har = serde_json::json!({
            "log": {
                "entries": [
                    {
                        "request": {
                            "method": "GET",
                            "url": "https://api.calendly.com/v2/users/me",
                            "headers": [
                                { "name": "Authorization", "value": "Bearer token123" }
                            ]
                        },
                        "response": {
                            "status": 200,
                            "content": {
                                "text": "{\"resource\":{\"name\":\"John Doe\",\"email\":\"john@example.com\",\"active\":true}}"
                            }
                        }
                    },
                    {
                        "request": {
                            "method": "POST",
                            "url": "https://api.calendly.com/v2/event_types",
                            "headers": [
                                { "name": "Authorization", "value": "Bearer token123" },
                                { "name": "Content-Type", "value": "application/json" }
                            ],
                            "postData": {
                                "text": "{\"name\":\"30 Minute Meeting\",\"duration\":30}"
                            }
                        },
                        "response": {
                            "status": 201,
                            "content": {
                                "text": "{\"id\":\"evt_123\",\"created_at\":\"2026-01-01T00:00:00Z\"}"
                            }
                        }
                    },
                    {
                        "request": {
                            "method": "GET",
                            "url": "https://assets.calendly.com/styles/main.css",
                            "headers": []
                        },
                        "response": {
                            "status": 200,
                            "content": { "text": "body { margin: 0; }" }
                        }
                    }
                ]
            }
        });

        let report = HarFlowEngine::analyze_har_value(&sample_har).expect("HAR analysis failed");

        assert_eq!(report.total_entries, 3);
        assert_eq!(report.static_assets_skipped, 1);
        assert_eq!(report.endpoints.len(), 2);

        let get_me = report
            .endpoints
            .iter()
            .find(|e| e.path == "/v2/users/me")
            .unwrap();
        assert_eq!(get_me.method, "GET");
        assert_eq!(get_me.auth_type, Some("Bearer JWT".to_string()));

        let routes = HarFlowEngine::to_api_routes(&report);
        assert_eq!(routes.len(), 2);
    }
}
