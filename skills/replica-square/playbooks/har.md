# Playbook: `replica-square har`

> **Role:** Network Flow & Protocol Inferrer  
> **Input:** `.har` (HTTP Archive 1.2) network recording file  
> **Outputs:** `replica/har-catalog.md`, `replica/har-report.json`, evidence ledger items  
> **Gate:** Optional pre-flight or companion to `recon`. Bypasses manual API reverse-engineering.

---

## What This Tool Does

Unlike manual DevTools inspection or dumping raw PCAP/HAR bytes into an LLM context (which exhausts tokens and hallucinates), `replica-square har`:

1. **Parses HAR 1.2 natively in Rust** with 0 LLM context token consumption.
2. **Filters noise automatically**: Drops static CSS, JS, fonts, images, and analytics beacons.
3. **Infers JSON schemas**: Deduces request and response payload shapes.
4. **Detects authentication**: Pinpoints Bearer JWTs, session cookies, and API key headers.
5. **Anchors cryptographic evidence**: Hashes every request/response with SHA-256 for REA compliance.

---

## How to Capture a HAR File

1. Open target app in Chrome / Brave / Safari DevTools (`Inspect → Network`).
2. Check `Preserve log`.
3. Complete the key user flows (e.g., signup, create event, checkout).
4. Right-click any network request → **"Save all as HAR with content"**.
5. Save as `target-flow.har`.

---

## CLI Invocation

```bash
replica-square har target-flow.har --out replica/
```

---

## Output Contract

| Artifact | Contents |
|:---|:---|
| `replica/har-catalog.md` | Markdown API catalog with methods, paths, status codes, query parameters, auth types, and sample JSON schemas |
| `replica/har-report.json` | Structured JSON representation with full endpoint inventory and evidence ledger |

---

## Handoff

- Feed inferred endpoints directly into `replica-square architect` to generate production PostgreSQL DDL and typed API route handlers.
- Feed routes into `replica-square mock` to start an embedded mock server immediately.
