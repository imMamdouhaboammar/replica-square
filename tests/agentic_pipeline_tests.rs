use replica_square::agent::{agent_for, AgentContext, SignalEvent};
use replica_square::handoff::HandoffEngine;
use replica_square::neural::NeuralGraph;
use replica_square::orchestrator::Orchestrator;
use replica_square::router::{AgentKind, DynamicRouter, PhaseIntent};
use std::fs;
use std::path::PathBuf;

#[test]
fn test_dynamic_router_intents() {
    let router = DynamicRouter::new();

    // 1. Full pipeline
    let r1 = router.route("I want to clone Calendly and build a full replica");
    assert_eq!(r1.intent, PhaseIntent::FullPipeline);
    assert_eq!(r1.dag.len(), 7);
    assert_eq!(r1.params.get("target"), Some(&"Calendly".to_string()));

    // 2. Recon only
    let r2 = router.route("Can you map screens and inventory features for Linear?");
    assert_eq!(r2.intent, PhaseIntent::Recon);
    assert_eq!(r2.dag.len(), 1);

    // 3. Entrepreneur
    let r3 = router.route("Analyze user complaints and reviews for competitor gaps");
    assert_eq!(r3.intent, PhaseIntent::Entrepreneur);

    // 4. Architect
    let r4 = router.route("Generate SQL schema and API route contracts for my clone");
    assert_eq!(r4.intent, PhaseIntent::Architect);

    // 5. Design
    let r5 = router.route("Audit WCAG contrast ratio on design tokens");
    assert_eq!(r5.intent, PhaseIntent::Design);

    // 6. Diff
    let r6 = router.route("Calculate feature parity score for our replica build");
    assert_eq!(r6.intent, PhaseIntent::Diff);

    // 7. Sweep
    let r7 = router.route("Run clean-room brand and trademark sweep across codebase");
    assert_eq!(r7.intent, PhaseIntent::Sweep);

    // 8. Launch
    let r8 = router.route("Audit store metadata and App Store listing keywords");
    assert_eq!(r8.intent, PhaseIntent::Launch);
}

#[test]
fn test_tool_agents_execution() {
    let pid = std::process::id();
    let temp_ws = PathBuf::from(format!("/tmp/test_agent_exec_{}", pid));
    let _ = fs::remove_dir_all(&temp_ws);
    fs::create_dir_all(&temp_ws).unwrap();

    let ctx = AgentContext::new("TestApp", &temp_ws);

    // Test ReconAgent
    let recon_agent = agent_for(&AgentKind::Recon);
    assert_eq!(recon_agent.kind(), AgentKind::Recon);
    assert!(!recon_agent.gate_satisfied(&ctx));

    let recon_out = recon_agent.run(&ctx).expect("ReconAgent run failed");
    assert!(recon_out.success);
    assert_eq!(recon_out.signal.event, SignalEvent::Complete);
    assert!(recon_agent.gate_satisfied(&ctx));

    // Test ArchitectAgent after Recon
    let arch_agent = agent_for(&AgentKind::Architect);
    assert!(!arch_agent.gate_satisfied(&ctx));

    let arch_out = arch_agent.run(&ctx).expect("ArchitectAgent run failed");
    assert!(arch_out.success);
    assert!(arch_agent.gate_satisfied(&ctx));

    // Test DesignAgent
    let design_agent = agent_for(&AgentKind::Design);
    let design_out = design_agent.run(&ctx).expect("DesignAgent run failed");
    assert!(design_out.success);
    assert!(design_agent.gate_satisfied(&ctx));

    // Clean up
    let _ = fs::remove_dir_all(&temp_ws);
}

#[test]
fn test_neural_graph_signal_propagation() {
    let mut graph = NeuralGraph::full_pipeline();

    // Initially, only recon is ready
    assert!(graph.predecessors_satisfied("recon"));
    assert!(!graph.predecessors_satisfied("architect"));
    assert!(!graph.predecessors_satisfied("sweep"));
    assert!(!graph.predecessors_satisfied("launch"));

    // Complete recon
    graph.receive(replica_square::agent::NeuralSignal {
        from_agent: "recon".into(),
        event: SignalEvent::Complete,
        payload: serde_json::json!({}),
    });

    // Now architect and entrepreneur are unblocked
    assert!(graph.predecessors_satisfied("architect"));
    assert!(graph.predecessors_satisfied("entrepreneur"));
    assert!(!graph.predecessors_satisfied("design"));

    // Complete architect
    graph.receive(replica_square::agent::NeuralSignal {
        from_agent: "architect".into(),
        event: SignalEvent::Complete,
        payload: serde_json::json!({}),
    });

    // Design and diff are unblocked
    assert!(graph.predecessors_satisfied("design"));
    assert!(graph.predecessors_satisfied("diff"));

    // Sweep still blocked until BOTH design and diff complete
    assert!(!graph.predecessors_satisfied("sweep"));

    graph.receive(replica_square::agent::NeuralSignal {
        from_agent: "design".into(),
        event: SignalEvent::Complete,
        payload: serde_json::json!({}),
    });
    assert!(!graph.predecessors_satisfied("sweep"));

    graph.receive(replica_square::agent::NeuralSignal {
        from_agent: "diff".into(),
        event: SignalEvent::Complete,
        payload: serde_json::json!({}),
    });
    assert!(graph.predecessors_satisfied("sweep"));

    // Complete sweep -> unblocks launch
    graph.receive(replica_square::agent::NeuralSignal {
        from_agent: "sweep".into(),
        event: SignalEvent::Complete,
        payload: serde_json::json!({}),
    });
    assert!(graph.predecessors_satisfied("launch"));
}

#[test]
fn test_auto_handoff_engine_execution() {
    let pid = std::process::id();
    let temp_ws = PathBuf::from(format!("/tmp/test_handoff_engine_{}", pid));
    let _ = fs::remove_dir_all(&temp_ws);
    fs::create_dir_all(&temp_ws).unwrap();

    let router = DynamicRouter::new();
    let route = router.route("Replicate Notion completely from scratch");
    assert_eq!(route.intent, PhaseIntent::FullPipeline);

    let ctx = AgentContext::new("Notion", &temp_ws);
    let result = HandoffEngine::execute_dag(&route.dag, &ctx).expect("Handoff execution failed");

    assert!(result.success, "Pipeline should succeed");
    assert!(result.agents_run >= 5, "At least 5 agents should have executed");
    assert!(!result.handoffs.is_empty(), "Handoff records should be logged");
    assert!(temp_ws.join("replica").join("handoff-trace.md").exists());
    assert!(temp_ws.join("replica").join("recon.md").exists());
    assert!(temp_ws.join("replica").join("architecture.md").exists());
    assert!(temp_ws.join("replica").join("tokens.json").exists());

    // Clean up
    let _ = fs::remove_dir_all(&temp_ws);
}

#[test]
fn test_orchestrator_routed_execution() {
    let pid = std::process::id();
    let temp_ws = PathBuf::from(format!("/tmp/test_orch_routed_{}", pid));
    let _ = fs::remove_dir_all(&temp_ws);
    fs::create_dir_all(&temp_ws).unwrap();

    let res = Orchestrator::run_routed("Build a clean replica of Calendly with 80% parity", &temp_ws)
        .expect("Routed execution failed");

    assert!(res.success);
    assert_eq!(res.agents_failed, 0);

    // Clean up
    let _ = fs::remove_dir_all(&temp_ws);
}

#[test]
fn test_elite_breakthrough_engines() {
    let pid = std::process::id();
    let temp_ws = PathBuf::from(format!("/tmp/test_elite_breakthroughs_{}", pid));
    let _ = fs::remove_dir_all(&temp_ws);
    fs::create_dir_all(&temp_ws).unwrap();

    // 1. Test OpportunityMatrix
    let themes = vec![
        replica_square::types::FeedbackTheme {
            theme_id: "pricing".to_string(),
            title: "Too expensive".to_string(),
            keywords: vec!["price".to_string()],
            total_mentions: 42,
            dissatisfaction_score: 8.5,
            sample_quotes: vec!["Hate paywall".to_string()],
            sources: vec!["G2".to_string()],
            recommended_feature_counter: "Flat fee tier".to_string(),
        },
        replica_square::types::FeedbackTheme {
            theme_id: "sync_reliability".to_string(),
            title: "Double booking sync errors".to_string(),
            keywords: vec!["sync".to_string()],
            total_mentions: 55,
            dissatisfaction_score: 9.2,
            sample_quotes: vec!["Lost client due to double booking".to_string()],
            sources: vec!["Trustpilot".to_string()],
            recommended_feature_counter: "Serializable tx locks".to_string(),
        },
    ];

    let opps = replica_square::entrepreneur::EntrepreneurEngine::calculate_opportunity_matrix(&themes);
    assert_eq!(opps.len(), 2);
    // Pricing: S effort (cost 1.0) -> ROI = 8.5 * 10 / 1.0 = 85.0
    // Sync: XL effort (cost 6.0) -> ROI = 9.2 * 10 / 6.0 = 15.33
    assert_eq!(opps[0].theme_id, "pricing");
    assert!(opps[0].opportunity_roi > opps[1].opportunity_roi);

    // 2. Test MockEngine Bun Export
    let routes = vec![replica_square::types::ApiRoute {
        method: "GET".to_string(),
        path: "/api/v1/events".to_string(),
        description: "List events".to_string(),
        auth_required: true,
        input_schema: "{}".to_string(),
        output_schema: "{\"events\":[]}".to_string(),
        flow_id: "F01".to_string(),
    }];

    let mock_out = temp_ws.join("mock-server.ts");
    replica_square::mock::MockEngine::export_bun_mock_server(&routes, &mock_out, 3000)
        .expect("Export Bun mock server failed");
    assert!(mock_out.exists());
    let mock_content = fs::read_to_string(&mock_out).unwrap();
    assert!(mock_content.contains("Bun.serve"));
    assert!(mock_content.contains("/api/v1/events"));

    // 3. Test E2E Spec Writer
    let flows = vec![replica_square::types::ReconFlow {
        flow_id: "F-BOOK".to_string(),
        name: "Book Meeting".to_string(),
        step_screens: vec!["S-CALENDAR".to_string()],
        happy_path: vec!["Pick date".to_string(), "Submit details".to_string()],
        edge_cases: vec!["Double submit".to_string()],
    }];
    let screens = vec![replica_square::types::ReconScreen {
        screen_id: "S-CALENDAR".to_string(),
        title: "Calendar View".to_string(),
        route_path: "/calendar".to_string(),
        viewport: "1440x900".to_string(),
        interactive_elements: vec!["Slot button".to_string()],
        network_calls: vec!["POST /api/v1/bookings".to_string()],
        states: vec!["ready".to_string()],
    }];

    let spec_out = temp_ws.join("e2e").join("flows.spec.ts");
    replica_square::e2e::E2eGenerator::write_spec_file(&flows, &screens, "http://localhost:3000", &spec_out)
        .expect("Write spec failed");
    assert!(spec_out.exists());
    let spec_content = fs::read_to_string(&spec_out).unwrap();
    assert!(spec_content.contains("AxeBuilder"));
    assert!(spec_content.contains("Flow: Book Meeting (F-BOOK)"));

    // Clean up
    let _ = fs::remove_dir_all(&temp_ws);
}
