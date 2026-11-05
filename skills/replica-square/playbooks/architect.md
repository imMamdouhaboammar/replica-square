# Playbook: `replica-square architect`

> **Role:** Technical Architecture Generator  
> **Inputs:** `replica/recon.md`, `replica/features.csv`  
> **Outputs:** `replica/architecture.md`, `replica/schema.sql`  
> **Gate:** Run after `recon` completes (and optionally after `entrepreneur`).

---

## Stack Selection

| Layer | Default | Acceptable Swap | Never Use |
|-------|---------|----------------|-----------|
| API | Rust (Axum) or TypeScript (Hono) | Go (Gin), Python (FastAPI) | PHP monolith, Rails |
| Database | PostgreSQL 16 | SQLite (prototype only) | MySQL, MongoDB as primary |
| Auth | Supabase Auth or Auth.js | Clerk, JWT-only | Roll-your-own bcrypt |
| Background jobs | Tokio tasks (Rust) or BullMQ (TS) | Celery | Cron-only |
| File storage | S3-compatible (R2 / MinIO) | Supabase Storage | Local disk in production |
| Cache | Redis | Upstash | In-process HashMap as cache |
| Search | Typesense | Meilisearch, Postgres FTS | Elasticsearch unless corpus > 100M docs |
| Realtime | Supabase Realtime | Ably, Socket.io | Polling loops |

Record final stack choices in `replica/architecture.md` with one-sentence rationale each.

---

## Schema Rules (Non-Negotiable)

1. **UUID PKs** — `id UUID PRIMARY KEY DEFAULT gen_random_uuid()`
2. **Audit timestamps** — every table has `created_at TIMESTAMPTZ DEFAULT now()` and `updated_at TIMESTAMPTZ DEFAULT now()`
3. **Owner column** — every user-scoped table has `user_id UUID REFERENCES users(id) ON DELETE CASCADE`
4. **Money as integer cents** — `price_cents INTEGER NOT NULL CHECK (price_cents >= 0)`, never `DECIMAL` or `FLOAT` for money
5. **Times as TIMESTAMPTZ UTC** — never `TIMESTAMP WITHOUT TIME ZONE`
6. **Status enums as check constraints** — `CHECK (status IN ('pending','confirmed','cancelled'))`, not PG ENUM types (migration-safe)
7. **Soft delete pattern** — `deleted_at TIMESTAMPTZ DEFAULT NULL` where needed; never physical deletes on user data
8. **FK on delete** — always specify `ON DELETE CASCADE` or `ON DELETE SET NULL`; never leave FK behavior implicit
9. **RLS policies** — every table gets `ENABLE ROW LEVEL SECURITY` + at least one policy for `SELECT`
10. **Indexes** — foreign keys always have a supporting index; add GIN index for `tsvector` columns

---

## Schema File Format (`replica/schema.sql`)

```sql
-- ============================================================
-- TABLE: <entity_name>
-- ============================================================
CREATE TABLE <entity_name> (
    id           UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    -- ... columns ...
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT now()
);

ALTER TABLE <entity_name> ENABLE ROW LEVEL SECURITY;

CREATE POLICY "Users see own rows" ON <entity_name>
    FOR SELECT USING (user_id = auth.uid());

CREATE INDEX idx_<entity_name>_user_id ON <entity_name>(user_id);
```

---

## API Contract Format

Each endpoint entry in `replica/architecture.md`:

```
METHOD /path/to/resource
  Description : one line
  Auth        : Bearer JWT | None | API Key
  Input       : { field: type } or N/A
  Output      : { field: type } or 204
  Flow        : guard → validate → DB query → emit event → respond
  Errors      : 400 validation, 401 auth, 403 permission, 404 not found, 409 conflict
```

Group by resource: `/users`, `/bookings`, `/event-types`, `/integrations`, etc.

---

## Webhooks & Background Jobs

For every async operation identified in `features.csv`:

```markdown
### Job: send_booking_reminder
- Trigger: booking.confirmed event
- Schedule: T-24h and T-1h before start_at
- Input: booking_id
- Idempotency: job_id = sha256(booking_id + scheduled_at)
- Failure: retry 3× exponential backoff (1s, 4s, 16s), then dead-letter
- Output: notification row inserted, booking.reminded_at updated
```

---

## Risk Register

Address each of the following before finalising `architecture.md`:

| Risk | Mitigation |
|------|-----------|
| Idempotency | All mutation endpoints accept and log a client-supplied `Idempotency-Key` header |
| Timezone DST | Store all times as UTC TIMESTAMPTZ; convert only at display layer |
| Concurrency / double-booking | Postgres advisory lock or `SELECT ... FOR UPDATE SKIP LOCKED` on booking slots |
| Rate limits | Per-user rate limit on mutation endpoints (100 req/min); 429 response with `Retry-After` |
| N+1 queries | All list endpoints use JOIN or batch loader; no loop-inside-loop queries |
| Secret leakage | All secrets in env vars; never committed; rotated on breach |
| Data migration | Schema migrations in numbered files (`0001_initial.sql`); always reversible |

---

## Build Order Rule

Always implement in this sequence:
1. **Vertical slice first** — pick one `must` feature, build it end-to-end (DB → API → minimal UI)
2. **All `must` features** — complete before any `should`
3. **`should` features** — complete before any `could`
4. **`could` features** — only after MVP is validated

This maps directly to `features.csv` priority column.

---

## CLI Invocation

```bash
replica-square architect \
  --recon replica/recon.md \
  --features replica/features.csv \
  --stack rust \
  --out-dir replica/
```

---

## Output Contract

| Artifact | Required | Contents |
|----------|----------|---------|
| `replica/architecture.md` | ✅ | Stack table, API contracts, webhooks, risk register, build order |
| `replica/schema.sql` | ✅ | Complete DDL with RLS, indexes, check constraints |

---

## Handoff

After this playbook completes:
- Run `replica-square design replica/tokens.json` to validate color palette
- Run `replica-square diff replica/features.csv` to get initial parity score
- Block on: `schema.sql` must contain at least the core user-scoped entity table
