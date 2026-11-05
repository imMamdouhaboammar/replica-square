---
name: replica-square
description: >
  Fully agentic, clean-room reverse-engineering and software reconstruction engine.
  Orchestrates seven autonomous tools (recon, entrepreneur, architect, design, diff, sweep, launch)
  to study any app, synthesize a production-grade clone architecture, verify parity, and clear
  all trademark and store-listing gates — entirely without touching proprietary code or assets.
  Triggered by: "clone this app", "reverse engineer X", "build my own version of X",
  "replica recon", "replica architect", "parity check", "clean-room sweep", "app store audit",
  or "run the full replica pipeline".
  Do NOT use for general code review, single-file edits, or non-reconstruction tasks.
version: "2.0.0"
author: "Replica-Square Contributors"
license: "MIT"
repository: "https://github.com/imMamdouhaboammar/replica-square"
hosts:
  - antigravity
  - claude
  - codex
  - cursor
  - universal
metadata:
  engine: "Rust 1.99 (native binary + JSON-RPC 2.0 MCP stdio server)"
  tools: 7
  playbooks: 7
  tests: 7
---

# Replica-Square: Fully Agentic Clean-Room Reconstruction Engine

> Point it at any app or natural language prompt. It dynamically routes intent, harvests verifiable evidence, plans a clean-room rebuild via autonomous tool-agents, propagates signals across a neural graph, and auto-handoffs across gates — autonomously.

---

## Dynamic Agentic Routing & Auto-Handoff Invariant

$$\text{Intent (Any Prompt)} \xrightarrow{\text{Dynamic Router}} \text{Execution DAG} \xrightarrow{\text{Tool-Agents}} \text{Neural Signal Bus} \xrightarrow{\text{Auto-Handoff}} \text{Production Release}$$

---

## The Seven Tool-Agents

| Agent | CLI | Playbook | What it does |
|:---|:---|:---|:---|
| **recon** | `replica-square recon` | [`playbooks/recon.md`](playbooks/recon.md) | Maps screens, flows, data model, and cryptographic evidence ledger |
| **entrepreneur** | `replica-square entrepreneur` | [`playbooks/entrepreneur.md`](playbooks/entrepreneur.md) | Mines user pain, ranks themes, formulates counter-features |
| **architect** | `replica-square architect` | [`playbooks/architect.md`](playbooks/architect.md) | Synthesizes PostgreSQL schema, API contracts, build order |
| **design** | `replica-square design` | [`playbooks/design.md`](playbooks/design.md) | WCAG 2.2 color validation, Tailwind token export |
| **diff** | `replica-square diff` | [`playbooks/diff.md`](playbooks/diff.md) | 80/20 feature-visual parity scoring |
| **sweep** | `replica-square sweep` | [`playbooks/sweep.md`](playbooks/sweep.md) | Trademark, domain, and hex-color clean-room sweep |
| **launch** | `replica-square launch` | [`playbooks/launch.md`](playbooks/launch.md) | App Store and Google Play metadata linter |

---

## Neural Connections & Auto-Handoff Architecture

```mermaid
flowchart TD
    PROMPT["User Prompt (Natural Language)"] --> ROUTER["Dynamic Agentic Router<br/>(RegexSet + Parameter Extraction)"]
    ROUTER --> DAG["Directed Acyclic Graph (DAG)"]

    subgraph ENGINE ["Auto-Handoff Engine & Neural Bus"]
        DAG --> RECON["ReconAgent<br/>Evidence Harvester"]
        RECON -->|"Complete (⚠️ passthrough)"| ENTRE["EntrepreneurAgent<br/>Market Pain Miner"]
        RECON -->|"Complete (Strict Gate)"| ARCH["ArchitectAgent<br/>PostgreSQL & API Spec"]
        ARCH -->|"Complete"| DESIGN["DesignAgent<br/>WCAG Token Auditor"]
        ARCH -->|"Complete"| DIFF["DiffAgent<br/>Parity Scorer"]
        DESIGN -->|"Complete (⚠️ passthrough)"| SWEEP["SweepAgent<br/>Clean-Room Clearance"]
        DIFF -->|"Complete (⚠️ passthrough)"| SWEEP
        SWEEP -->|"Complete (Strict Gate)"| LAUNCH["LaunchAgent<br/>Store Listing Linter"]

        CIRCUIT["⚡ Fable Circuit Breaker (Halt if failures ≥ 2)"]
    end

    LAUNCH --> OUT["✅ Production-Ready Deliverable"]
    ENGINE -.->|"Live State & Signals"| TRACE["replica/handoff-trace.md"]
```

---

## Three Constitutional Invariants

1. **Clean-Room Invariant**: Never copy source code, trademarks, assets, proprietary APIs, or copyrighted content. Study observable behavior and public contracts only.
2. **Evidence-First Invariant**: Every architectural claim cites a SHA-256 digest, URL, or DOM/network trace. Never speculate without evidence.
3. **Gate-Must-Pass Invariant**: The pipeline is blocked at parity < 80%, any sweep violation, or any store listing error. No exceptions.

---

## Quick Start

### 1. Dynamic Routing & Autonomous Execution
```bash
# Analyze prompt and inspect execution DAG
replica-square route "Clone Calendly and check parity at 85%"

# Run autonomous pipeline with dynamic routing and auto-handoff
replica-square run --prompt "Build a full clean-room clone of Calendly"
```

### 2. Full Standard Pipeline
```bash
replica-square run --target "Calendly" --url https://calendly.com --fail-under 80
```

### 3. Individual Tool Calls
```bash
replica-square recon https://app.example.com --out replica/
replica-square entrepreneur replica/reviews.csv
replica-square architect replica/recon-map.json
replica-square design replica/tokens.json
replica-square diff replica/features.csv --fail-under 80
replica-square sweep . --avoid "Competitor" --domains competitor.com --colors "#006bff"
replica-square launch replica/listing.json
```

### 4. As MCP Server (for AI Agents)
```bash
replica-square mcp
```

Tools exposed over JSON-RPC 2.0 stdio:
- `replica_route`: Analyzes natural language prompt and returns DAG
- `replica_orchestrate`: Executes dynamic multi-agent DAG with neural signal bus
- `replica_recon`, `replica_architect`, `replica_design`, `replica_entrepreneur`, `replica_diff`, `replica_brand_sweep`, `replica_launch_audit`

---

## Reference Index

| Path | Contents |
|:---|:---|
| [`references/neural-connections.md`](references/neural-connections.md) | Neural topology, signal protocol, and warning passthroughs |
| [`references/auto-handoff.md`](references/auto-handoff.md) | Auto-handoff engine, artifact gates, and circuit breaker |
| [`references/pipeline-state.md`](references/pipeline-state.md) | SOT state machine and artifact naming |
| [`references/evidence-contract.md`](references/evidence-contract.md) | Evidence schema, SHA-256 anchoring, REA compliance |
| [`references/parity-formula.md`](references/parity-formula.md) | 80/20 parity formula derivation and weighting |
| [`references/legal-boundaries.md`](references/legal-boundaries.md) | Clean-room legal scope and skip-row rules |
| [`playbooks/recon.md`](playbooks/recon.md) | Recon tool — step-by-step agent instructions |
| [`playbooks/entrepreneur.md`](playbooks/entrepreneur.md) | Entrepreneur tool — review mining and counter-feature synthesis |
| [`playbooks/architect.md`](playbooks/architect.md) | Architect tool — schema DDL and API contract generation |
| [`playbooks/design.md`](playbooks/design.md) | Design tool — WCAG 2.2 and token export |
| [`playbooks/diff.md`](playbooks/diff.md) | Diff tool — parity scoring methodology |
| [`playbooks/sweep.md`](playbooks/sweep.md) | Sweep tool — trademark clean-room certification |
| [`playbooks/launch.md`](playbooks/launch.md) | Launch tool — store metadata compliance |
| [`evals/triggers.md`](evals/triggers.md) | Positive, negative, and pressure-test cases |
