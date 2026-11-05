use crate::brand::BrandEngine;
use crate::design::DesignEngine;
use crate::diff::DiffEngine;
use crate::launch::{LaunchEngine, StoreMetadata};
use crate::types::DesignTokens;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::io::{self, BufRead, Write};

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub id: Option<Value>,
    pub method: String,
    pub params: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Value>,
}

pub struct McpServer;

impl McpServer {
    pub async fn run() -> Result<()> {
        let stdin = io::stdin();
        let mut stdout = io::stdout();
        let reader = stdin.lock();

        for line in reader.lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }

            if let Ok(req) = serde_json::from_str::<JsonRpcRequest>(&line) {
                let resp = Self::handle_request(req).await;
                let out = serde_json::to_string(&resp)?;
                writeln!(stdout, "{}", out)?;
                stdout.flush()?;
            }
        }

        Ok(())
    }

    async fn handle_request(req: JsonRpcRequest) -> JsonRpcResponse {
        match req.method.as_str() {
            "initialize" => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: req.id,
                result: Some(json!({
                    "protocolVersion": "2024-11-05",
                    "capabilities": {
                        "tools": {}
                    },
                    "serverInfo": {
                        "name": "replica-square",
                        "version": "2.0.0"
                    }
                })),
                error: None,
            },
            "tools/list" => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: req.id,
                result: Some(json!({
                    "tools": [
                        {
                            "name": "replica_route",
                            "description": "Analyzes natural language prompt to classify phase intent and construct execution DAG",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "prompt": { "type": "string" }
                                },
                                "required": ["prompt"]
                            }
                        },
                        {
                            "name": "replica_orchestrate",
                            "description": "Executes dynamic multi-agent DAG with neural signal bus and auto-handoff mechanism",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "prompt": { "type": "string" },
                                    "workspace_path": { "type": "string" }
                                },
                                "required": ["prompt"]
                            }
                        },
                        {
                            "name": "replica_recon",
                            "description": "Reverse engineers any target application into screens, flows, and verifiable evidence ledger",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "target_name": { "type": "string" },
                                    "target_url": { "type": "string" },
                                    "artifact_path": { "type": "string" }
                                },
                                "required": ["target_name"]
                            }
                        },
                        {
                            "name": "replica_architect",
                            "description": "Synthesizes PostgreSQL schema with RLS, API contracts, and vertical-slice build order",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "recon_json": { "type": "object" }
                                }
                            }
                        },
                        {
                            "name": "replica_design",
                            "description": "Validates WCAG 2.2 color contrast ratios and exports Tailwind CSS configuration",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "tokens_json": { "type": "object" }
                                },
                                "required": ["tokens_json"]
                            }
                        },
                        {
                            "name": "replica_entrepreneur",
                            "description": "Analyzes real user reviews to rank customer grievances and formulate competitive counters",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "reviews_csv_path": { "type": "string" }
                                },
                                "required": ["reviews_csv_path"]
                            }
                        },
                        {
                            "name": "replica_diff",
                            "description": "Calculates functional feature parity and visual layout similarity (0-100%)",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "features_csv_path": { "type": "string" },
                                    "original_png_path": { "type": "string" },
                                    "clone_png_path": { "type": "string" }
                                },
                                "required": ["features_csv_path"]
                            }
                        },
                        {
                            "name": "replica_brand_sweep",
                            "description": "Performs recursive clean-room trademark and brand IP sweep across codebase",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "code_root": { "type": "string" },
                                    "avoid_names": { "type": "array", "items": { "type": "string" } },
                                    "avoid_domains": { "type": "array", "items": { "type": "string" } },
                                    "avoid_colors": { "type": "array", "items": { "type": "string" } }
                                },
                                "required": ["code_root", "avoid_names"]
                            }
                        },
                        {
                            "name": "replica_launch_audit",
                            "description": "Audits store listing metadata against Apple App Store and Google Play compliance rules",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "listing_metadata": { "type": "object" }
                                },
                                "required": ["listing_metadata"]
                            }
                        }
                    ]
                })),
                error: None,
            },
            "tools/call" => {
                let params = req.params.unwrap_or(json!({}));
                let name = params.get("name").and_then(|v| v.as_str()).unwrap_or("");
                let args = params.get("arguments").cloned().unwrap_or(json!({}));

                let result = match name {
                    "replica_route" => {
                        let prompt = args.get("prompt").and_then(|v| v.as_str()).unwrap_or("");
                        let route = crate::router::DynamicRouter::new().route(prompt);
                        json!({
                            "status": "success",
                            "intent": route.intent,
                            "dag": route.dag,
                            "params": route.params,
                            "explanation": route.explanation,
                            "markdown": route.to_markdown()
                        })
                    }
                    "replica_orchestrate" => {
                        let prompt = args.get("prompt").and_then(|v| v.as_str()).unwrap_or("");
                        let ws = args.get("workspace_path").and_then(|v| v.as_str()).unwrap_or(".");
                        match crate::orchestrator::Orchestrator::run_routed(prompt, ws) {
                            Ok(res) => json!({
                                "status": "success",
                                "pipeline_success": res.success,
                                "agents_run": res.agents_run,
                                "agents_skipped": res.agents_skipped,
                                "agents_failed": res.agents_failed,
                                "circuit_breaker_tripped": res.circuit_breaker_tripped,
                                "handoffs_count": res.handoffs.len(),
                                "neural_graph_markdown": res.final_graph_md
                            }),
                            Err(e) => json!({ "error": e.to_string() }),
                        }
                    }
                    "replica_design" => {
                        if let Ok(tokens) = serde_json::from_value::<DesignTokens>(args.get("tokens_json").cloned().unwrap_or(json!({}))) {
                            let contrast_results = DesignEngine::check_tokens(&tokens);
                            let tailwind = DesignEngine::export_tailwind(&tokens);
                            json!({
                                "status": "success",
                                "contrast_results": contrast_results,
                                "tailwind_config": tailwind
                            })
                        } else {
                            json!({ "error": "Invalid tokens_json structure" })
                        }
                    }
                    "replica_brand_sweep" => {
                        let root = args.get("code_root").and_then(|v| v.as_str()).unwrap_or(".");
                        let names: Vec<String> = args.get("avoid_names")
                            .and_then(|v| v.as_array())
                            .map(|a| a.iter().filter_map(|x| x.as_str().map(String::from)).collect())
                            .unwrap_or_default();
                        let domains: Vec<String> = args.get("avoid_domains")
                            .and_then(|v| v.as_array())
                            .map(|a| a.iter().filter_map(|x| x.as_str().map(String::from)).collect())
                            .unwrap_or_default();
                        let colors: Vec<String> = args.get("avoid_colors")
                            .and_then(|v| v.as_array())
                            .map(|a| a.iter().filter_map(|x| x.as_str().map(String::from)).collect())
                            .unwrap_or_default();

                        match BrandEngine::sweep_codebase(root, &names, &domains, &colors) {
                            Ok(rep) => json!({
                                "status": "success",
                                "clean_room_passed": rep.passed,
                                "violations_count": rep.total_violations,
                                "markdown": BrandEngine::render_markdown(&rep)
                            }),
                            Err(e) => json!({ "error": e.to_string() }),
                        }
                    }
                    "replica_diff" => {
                        let csv_path = args.get("features_csv_path").and_then(|v| v.as_str()).unwrap_or("");
                        match DiffEngine::calculate_feature_parity(csv_path) {
                            Ok(mut rep) => {
                                if let (Some(orig), Some(clone)) = (
                                    args.get("original_png_path").and_then(|v| v.as_str()),
                                    args.get("clone_png_path").and_then(|v| v.as_str()),
                                ) {
                                    if let Ok(v_score) = DiffEngine::compute_visual_layout_parity(orig, clone) {
                                        rep.visual_score = Some(v_score);
                                        rep.overall_score = ((0.8 * rep.feature_score + 0.2 * v_score) * 10.0).round() / 10.0;
                                    }
                                }
                                json!({
                                    "status": "success",
                                    "report": rep,
                                    "markdown": DiffEngine::render_markdown(&rep)
                                })
                            }
                            Err(e) => json!({ "error": e.to_string() }),
                        }
                    }
                    "replica_launch_audit" => {
                        if let Ok(meta) = serde_json::from_value::<StoreMetadata>(args.get("listing_metadata").cloned().unwrap_or(json!({}))) {
                            let audit = LaunchEngine::audit_listing(&meta);
                            json!({
                                "status": "success",
                                "passed": audit.passed,
                                "errors": audit.errors,
                                "warnings": audit.warnings,
                                "markdown": LaunchEngine::render_markdown(&audit)
                            })
                        } else {
                            json!({ "error": "Invalid listing_metadata" })
                        }
                    }
                    _ => json!({ "error": format!("Tool '{}' not recognized", name) }),
                };

                JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: req.id,
                    result: Some(json!({ "content": [{ "type": "text", "text": serde_json::to_string_pretty(&result).unwrap() }] })),
                    error: None,
                }
            }
            _ => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: req.id,
                result: None,
                error: Some(json!({ "code": -32601, "message": "Method not found" })),
            },
        }
    }
}
