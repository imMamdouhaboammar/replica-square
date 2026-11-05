# Playbook: `replica-square launch`

> **Role:** App Store Listing Linter  
> **Input:** `replica/listing.json`  
> **Output:** `replica/listing-audit.md`  
> **Gate:** Run only after `sweep` passes (exit 0).

---

## Listing File Schema

Create `replica/listing.json` before invoking the tool:

```json
{
  "avoid": ["Calendly", "Cal.com", "Acuity"],
  "app_store": {
    "name": "BookFlow — Smart Scheduler",
    "subtitle": "Calendar links for your team",
    "promo_text": "New: AI-powered availability detection",
    "description": "BookFlow makes scheduling painless...",
    "keywords": "scheduler,calendar,booking,meetings,availability",
    "category": "Business"
  },
  "google_play": {
    "title": "BookFlow — Smart Scheduler",
    "short_description": "Shareable calendar links your clients love",
    "full_description": "BookFlow makes scheduling painless...",
    "category": "Business"
  }
}
```

---

## Field Limits

### App Store Connect

| Field | Limit | Violation |
|-------|-------|----------|
| `name` | 30 chars | ERROR if over |
| `subtitle` | 30 chars | ERROR if over |
| `promo_text` | 170 chars | ERROR if over |
| `description` | 4000 chars | ERROR if over |
| `keywords` | 100 chars total | ERROR if over |

### Google Play Console

| Field | Limit | Violation |
|-------|-------|----------|
| `title` | 30 chars | ERROR if over |
| `short_description` | 80 chars | ERROR if over |
| `full_description` | 4000 chars | ERROR if over |

---

## Content Checks

| Check | Severity | Rule |
|-------|----------|------|
| Competitor name in any field | ERROR | No string from `avoid` list may appear in any listing field |
| "#1", "best", "top-rated", "award-winning" in `name` or `subtitle` | WARNING | Apple/Google reject ranking/superlative claims in title fields |
| Price claim in `name` or `subtitle` | WARNING | "Free", "Paid", "$X" in title fields triggers review rejection |
| Keyword separator waste | WARNING | Keywords string uses `, ` (comma + space) wastes chars; use `,` only |
| Keywords contain duplicates | WARNING | Duplicate keywords waste the 100-char budget |
| `name` ≠ `title` across stores | WARNING | Minor inconsistency is acceptable, but flag if semantically different |

---

## Anti-Claim Regex Patterns

The engine checks `name` and `subtitle` against:

```
\b(#1|number one|best|top-rated|award.winning|most popular|fastest|cheapest|free forever)\b
\b(free|paid|\$\d|\d+%)\b
```

These match Apple App Store Review Guideline 2.3.1 and Google Play Policy banned phrases.

---

## CLI Invocation

```bash
replica-square launch replica/listing.json
```

**Exit code 1** if any ERROR-level violation exists.

---

## Output: `replica/listing-audit.md`

```markdown
# App Store Listing Audit

## Summary
- Errors: 0
- Warnings: 2
- **Status: ✅ PASS (warnings — review before submit)**

## Field Length Check
| Store | Field | Chars Used | Limit | Status |
|-------|-------|-----------|-------|--------|
| App Store | name | 28 | 30 | ✅ |
| App Store | subtitle | 29 | 30 | ✅ |
| App Store | keywords | 97 | 100 | ✅ |
| Google Play | title | 28 | 30 | ✅ |

## Warnings
1. App Store > keywords: comma+space separators waste 4 chars. Use comma-only.
2. Google Play > short_description: contains "simple" — consider more specific benefit language.

## Competitor Name Check
- ✅ No strings from avoid list found in any listing field.
```

---

## Trademark Notice

**The tool does NOT perform trademark searches.** Before submitting:

1. Search the target market's trademark registry (USPTO for US, EUIPO for EU)
2. Verify your app name is not a registered mark in the `Business` or `Productivity` classes
3. Run a domain check: does `yourdomain.com` resolve to a live competing service?
4. Document your trademark review in `docs/legal/trademark-check.md`

This is the agent's responsibility; the tool only catches mechanical listing errors.

---

## Failure Protocol

When exit code is 1 (competitor name found):

1. Identify which field contains the competitor string
2. Replace with your product name or a generic term
3. Never use the original app's name as a differentiating phrase ("Better than [X]" is still a violation)
4. Re-run `replica-square launch` until exit code 0

---

## Handoff

After this playbook completes:
- Paste `app_store.name`, `app_store.subtitle`, and `app_store.keywords` directly into App Store Connect
- Paste `google_play.title` and `google_play.short_description` into Google Play Console
- Attach `replica/listing-audit.md` to the app release notes for reviewer audit trail
- Block on: exit code 0
