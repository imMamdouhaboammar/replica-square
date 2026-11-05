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
