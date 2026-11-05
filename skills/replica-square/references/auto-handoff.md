# Reference: Auto-Handoff Mechanism & Circuit Breaker

## Overview

The Auto-Handoff Mechanism (`HandoffEngine`) provides autonomous coordination between tool-agents without human intervention or fragile script orchestration.

```
Incoming Intent → Dynamic Router → Topological DAG
      ↓
[Gate Check] ── Artifacts Exist? ── (Yes) ──→ Skip & Unblock Downstream
      ↓ (No)
[Dispatch Agent]
      ↓
[Receive Signal] ──→ Update Neural Graph
      ↓
[Evaluate Circuit Breaker] ── Streak ≥ 2? ── (Yes) ──→ Hard Halt (Fail-Safe)
      ↓ (No)
[Auto-Handoff] ── Dispatch Next Unblocked Node
```

---

## 1. Disk Artifact Gate Checks

Before dispatching any node marked `skippable`, the HandoffEngine verifies if its canonical output files exist:

| Agent | Required Gate Artifacts |
|:---|:---|
| `recon` | `replica/recon.md`, `replica/features.csv`, `replica/recon-map.json` |
| `entrepreneur` | `replica/feedback.md` |
| `architect` | `replica/architecture.md`, `replica/schema.sql` |
| `design` | `replica/tailwind.tokens.js` |
| `diff` | `replica/parity.md` |
| `sweep` | `replica/cleanroom-report.md` |
| `launch` | `replica/listing-audit.md` |

If all gate artifacts exist, the agent is marked `Skipped`, emits `SignalEvent::Skipped`, and immediately unblocks its downstream dependents.

---

## 2. Fable Circuit Breaker Protocol

To prevent runaway agent loops or persistent failure cascades, the engine implements the **Fable Circuit Breaker**:

- **Failure Streak Threshold:** `2` consecutive failures across any agents.
- **Trip Condition:** If `failure_streak >= 2`, the engine halts all execution immediately.
- **Safety Guarantee:** Halts speculative edits, preserves current workspace artifacts, and writes incident diagnostics to `replica/handoff-trace.md`.

---

## 3. Dynamic Intent Router Integration

The HandoffEngine consumes the execution DAG constructed by the `DynamicRouter`:

```bash
# Analyze prompt and preview dynamic execution DAG
replica-square route "Reverse engineer Cal.com and check feature parity"

# Execute routed pipeline with auto-handoff
replica-square run --prompt "Build a full clean-room clone of Calendly"
```

---

## 4. Handoff Audit Trail

Every handoff event records:
1. `from_agent`: Predecessor agent
2. `to_agent`: Downstream agent
3. `reason`: `NormalCompletion`, `GateAlreadySatisfied`, or `WarningPassthrough`
4. `artifacts_carried`: Explicit list of on-disk artifacts passed across the boundary
