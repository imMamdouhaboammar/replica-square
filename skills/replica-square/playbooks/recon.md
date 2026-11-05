# Playbook: `replica-square recon`

> **Role:** Evidence Harvester  
> **Outputs:** `replica/recon.md`, `replica/features.csv`, `replica/recon-map.json`  
> **Gate:** Must complete before `architect` or `entrepreneur` runs.

---

## Pre-flight Questions (ask once, record answers)

| # | Question | Key |
|---|----------|-----|
| 1 | Which product are you cloning? (exact name, URL) | `TARGET` |
| 2 | Which platform slice? (web / iOS / Android / API / all) | `PLATFORM` |
| 3 | Which user segment is in scope? (free tier / paid / admin / all) | `SEGMENT` |
| 4 | What is the purpose of this replica? (competitor analysis / greenfield build / parity test) | `PURPOSE` |
| 5 | Are there any features explicitly OUT OF SCOPE? | `SKIP_LIST` |

Do NOT proceed until all 5 answers are recorded in `replica/recon.md` under a `## Scope` header.

---

## Source Table

Exhaust all sources below before writing the recon map. Mark each row ✅ Done / ⚠️ Partial / ❌ Unavailable.

| Priority | Source | How to access |
|----------|--------|---------------|
| P0 | Live product (free tier) | Create a free account, screen-record onboarding |
| P0 | Pricing page | Public URL — capture field names and tier limits |
| P0 | Help / docs center | Sitemap crawl or search "site:help.target.com" |
| P1 | App Store listing | iOS App Store public page |
| P1 | Google Play listing | Play Store public page |
| P1 | Public changelog / releases | Product blog or `/releases` path |
| P1 | Marketing site | Hero copy, feature bullets, comparison tables |
| P2 | YouTube demo videos | Official channel + community walkthroughs |
| P2 | G2 / Capterra feature list | Public profile tabs |
| P2 | Public API docs | developer.target.com or Postman workspace |
| P3 | Community / Reddit | r/<product> posts about features and gaps |

---

## Execution Steps

### Step 1 — Screen Inventory
Enumerate every distinct screen. Assign IDs: `S01`, `S02`, …

```
ID     Screen Name              Auth Required   Notes
S01    Onboarding: welcome       No
S02    Onboarding: account form  No
S03    Dashboard                 Yes             Primary nav hub
...
```

Rule: A screen is distinct if its URL path, sheet, or modal has a unique title or primary action.

### Step 2 — Flow Inventory
Group screens into user flows. Assign IDs: `F01`, `F02`, …

```
F01  Signup → Onboarding → Dashboard   S01, S02, S03
F02  Create new booking                S05, S06, S07
...
```

Each flow must have a start screen, end screen, and linear or branching path.

### Step 3 — Component Inventory
For each screen list UI components:

```
S03  Dashboard:
  - NavBar (logo, primary links, user avatar)
  - CalendarWidget (month view, event chips)
  - QuickActions (3 CTA buttons)
  - RecentActivity (paginated list)
```

Do NOT invent components. Only list what is visibly present in recordings or screenshots.

### Step 4 — Inferred Data Model
From visible forms, filters, and displayed attributes infer entities:

```
Entity: Booking
  id, host_user_id, invitee_email, start_at (timestamptz), duration_minutes (int),
  status (pending|confirmed|cancelled), meeting_type_id, notes
```

Flag each field as `observed` (visible in UI) or `inferred` (logically required but not shown).

### Step 5 — Feature Matrix
Produce `replica/features.csv` with these columns:

```
id,name,description,priority,status,source_id,notes
F001,Event type creation,"User creates reusable meeting type with duration+buffer",must,yes,S05,"observed: /event-types"
F002,Team scheduling,"Round-robin or collective availability",should,yes,S08,"inferred from pricing page"
F003,SMS reminders,Automated SMS before event,could,yes,S09,"App Store screenshot"
```

Priority values: `must` | `should` | `could`  
Status values: `yes` | `partial` | `no` | `skip`

### Step 6 — Apply Skip-List Rules
Remove a feature from scoring (set `status = skip`) when ANY of the following is true:
- Feature requires proprietary third-party licensing
- Feature is clearly server-infrastructure (CDN, caching strategy) with no user-visible manifestation
- User explicitly placed it in `SKIP_LIST` during pre-flight
- Feature cannot be observed or inferred from public evidence

### Step 7 — Sizing
Assign T-shirt size to each feature for build estimation:

| Size | Rough effort | Examples |
|------|-------------|---------|
| S | < 1 day | Static field, label, toggle |
| M | 1–3 days | CRUD form, list with filter |
| L | 3–10 days | Calendar widget, file upload, real-time collab |
| XL | 10+ days | Payment flow, video conferencing, AI pipeline |

### Step 8 — Evidence Anchoring
Every feature row in `features.csv` must have a `source_id` tracing to an entry in `recon-map.json`.

`recon-map.json` schema:
```json
{
  "target": "Calendly",
  "platform": "web",
  "created_at": "2026-01-15T10:00:00Z",
  "evidence": [
    {
      "id": "S05",
      "source_type": "screen",
      "uri": "https://calendly.com/event_types",
      "sha256_digest": "44fbc761...",
      "observed_at": "2026-01-15T09:30:00Z",
      "observations": ["Create event type button", "Duration selector 15/30/60/custom"]
    }
  ]
}
```

Compute SHA-256 of any saved HTML/screenshot bytes and record it. This is the REA evidence anchor.

### Step 9 — Build `recon.md`
Structure:

```markdown
# Recon: <TARGET>

## Scope
- Target: ...
- Platform: ...
- Segment: ...
- Purpose: ...
- Skip list: ...

## Screen Inventory
<table>

## Flow Inventory
<table>

## Component Inventory
<grouped by screen>

## Inferred Data Model
<entity list>

## Evidence Index
<links to recon-map.json entries>
```

---

## CLI Invocation

```bash
replica-square recon \
  --target "Calendly" \
  --url "https://calendly.com" \
  --platform web \
  --out-dir replica/
```

---

## Output Contract

| Artifact | Required | Schema |
|----------|----------|--------|
| `replica/recon.md` | ✅ | Markdown; headers: Scope, Screens, Flows, Components, Data Model, Evidence |
| `replica/features.csv` | ✅ | CSV; columns: id, name, description, priority, status, source_id, notes |
| `replica/recon-map.json` | ✅ | JSON; keys: target, platform, created_at, evidence[] |

---

## Handoff

After this playbook completes:
- Run `replica-square entrepreneur replica/reviews.csv` (if review data collected)
- Run `replica-square architect` (always next)
- Block on: at least 10 features in `features.csv`, all P0 sources ✅
