# Reference: Evidence Contract

## Purpose

Every claim in a Replica-Square recon must be traceable to a verifiable public source. This document defines what qualifies as evidence, how to record it, and what REA (Reverse Engineering Architecture) compliance requires.

---

## Evidence JSON Schema

Each entry in `recon-map.json`'s `evidence` array:

```json
{
  "id": "S05",
  "source_type": "screen",
  "uri": "https://calendly.com/event_types/new",
  "sha256_digest": "44fbc761a8c9d3fa5b2e1d8a7c4f3b2e1a9c8d7f6e5b4a3c2d1e0f9a8b7c6d5",
  "observed_at": "2026-01-15T09:30:00Z",
  "observations": [
    "Duration selector: 15, 30, 60, custom minutes",
    "Buffer time field: before and after event",
    "Availability: select days of week + time ranges"
  ]
}
```

### Field Definitions

| Field | Type | Required | Rule |
|-------|------|----------|------|
| `id` | string | ✅ | Format: `S##` (screen), `F##` (flow), `API##`, `RV##` (review), `DOC##` (doc page) |
| `source_type` | enum | ✅ | `screen` \| `flow` \| `api_response` \| `review` \| `marketing_page` \| `help_doc` \| `app_store_listing` \| `video_frame` |
| `uri` | string | ✅ | Public URL that resolves to the evidence. Use Wayback Machine URL if original may 404. |
| `sha256_digest` | string | ✅ | SHA-256 hex of the saved bytes (HTML, PNG, JSON) at time of capture |
| `observed_at` | ISO 8601 | ✅ | Timestamp of observation |
| `observations` | string[] | ✅ | Verbatim observable facts. No interpretation. No "it seems like". |

---

## Evidence vs Inference

| Category | Definition | Example |
|----------|-----------|---------|
| **Evidence** | Directly observed and recorded from a public source | "Duration field shows values: 15, 30, 60, custom" |
| **Inference** | Logically derived from evidence | "The API likely stores duration as integer minutes" |
| **Speculation** | No supporting evidence | "They probably use PostgreSQL" |

Rules:
- `observations[]` in evidence entries: **evidence only** (verbatim, observed)
- `features.csv` notes column: may include inferences, clearly tagged as `[inferred]`
- `recon.md` body: separate `## Inferred Data Model` section clearly labelled "inferred"
- **Never** present inferences as direct observations in evidence JSON

---

## SHA-256 Anchoring

Every evidence entry requires a SHA-256 digest to prove the artifact existed at observation time.

Compute digest:

```bash
# For saved HTML
sha256sum saved_page.html

# For PNG screenshot
sha256sum screenshot.png

# For JSON API response
sha256sum api_response.json

# macOS alternative
shasum -a 256 file
```

Store the raw file in a local `evidence/` directory (gitignored). Record only the hex digest in `recon-map.json`.

If a live URL can be verified at review time, the digest is secondary confirmation. If the URL changes or 404s, the digest is the only proof.

---

## REA Compliance Rules

Replica-Square evidence meets REA (Reverse Engineering Architecture) standards when:

1. **Every** feature row in `features.csv` has a `source_id` that exists in `recon-map.json`
2. **Every** `recon-map.json` entry has a non-empty `sha256_digest`
3. **Every** quote in `feedback.md` has a matching entry in `reviews.csv` with a URL
4. **Zero** observations in evidence entries contain the word "probably", "likely", "seems", "assume", or "maybe"
5. **All** `observed_at` timestamps are in the past relative to the report date

---

## Evidence Quality Gates

Before submitting a recon report, verify:

```bash
# Gate 1: All features have source_ids
python3 -c "
import csv, json, sys
features = list(csv.DictReader(open('replica/features.csv')))
evidence_ids = {e['id'] for e in json.load(open('replica/recon-map.json'))['evidence']}
missing = [f['id'] for f in features if f['source_id'] not in evidence_ids and f['status'] != 'skip']
if missing: print('FAIL: missing source_ids for', missing); sys.exit(1)
else: print('PASS: all features anchored')
"
```

---

## What Does NOT Qualify as Evidence

| Disqualified | Reason |
|-------------|--------|
| Competitor's private admin panel (requires login beyond free tier) | Cannot be publicly verified |
| Reverse-engineered binary or obfuscated JS bundle | Legal risk; not clean-room |
| Screenshots from third-party blog posts | Secondary source; may be outdated |
| AI-generated descriptions of competitor features | Not observable reality |
| Paraphrased reviews | Must be verbatim |
| Personal memory ("I used it and remember...") | Not reproducible |
