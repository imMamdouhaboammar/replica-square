# Playbook: `replica-square mock`

> **Role:** Autonomous HTTP Mock Server  
> **Inputs:** `replica/recon-map.json` / `replica/architecture.md`  
> **Outputs:** Live HTTP Mock Server on `http://127.0.0.1:3000`, `replica/mock-server.ts`  
> **Gate:** Run as soon as `architect` completes. Enables day-1 frontend and test development.

---

## Why This Tool Is Critical

A classic failure in software replication is waiting for the complete backend database and migrations to be built before testing frontend screens.

`replica-square mock` solves this by spinning up a zero-dependency async HTTP mock server matching the exact synthesized API routes (`ApiRoute`s).

---

## Capabilities

- **Zero-Dependency Native Server**: Runs directly inside the compiled Rust binary over Tokio.
- **Universal CORS**: Handles `OPTIONS` preflight and `Access-Control-Allow-Origin: *`.
- **Typed Responses**: Serves typed mock JSON responses matching `output_schema`.
- **Standalone Bun Export**: Generates `replica/mock-server.ts` so developers can run it independently via `bun run replica/mock-server.ts`.

---

## CLI Invocation

```bash
# Start embedded live mock server
replica-square mock --port 3000

# Export standalone Bun script only (without binding port)
replica-square mock --port 3000 --export-only
```

---

## Testing Against the Mock

```bash
# Test mock endpoint
curl -i http://localhost:3000/api/v1/events

# Response:
# HTTP/1.1 200 OK
# Content-Type: application/json
# Access-Control-Allow-Origin: *
# {"status":"ok","replica_mock":true}
```

---

## Handoff

- Point the frontend clone prototype or Playwright E2E runner (`BASE_URL=http://localhost:3000`) directly at the mock server.
- Run `replica-square test-gen` to generate test specs targeting the mock.
