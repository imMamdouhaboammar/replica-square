# Playbook: `replica-square entrepreneur`

> **Role:** Market Intelligence Engine  
> **Input:** `replica/reviews.csv`  
> **Outputs:** `replica/feedback.md`, `replica/fixes.md`, updated rows in `replica/features.csv`  
> **Gate:** Run after `recon` completes. Output feeds `architect` and `diff`.

---

## Review Data Collection

Build `replica/reviews.csv` before invoking the tool. Use only public, non-authenticated sources.

**CSV schema:**
```
source,url,date,rating,text
app_store,https://apps.apple.com/...,2026-01-10,2,"Keeps crashing when I add a third invitee"
google_play,https://play.google.com/...,2026-01-08,1,"No SMS reminders on free tier, deal breaker"
g2,https://www.g2.com/products/...,2025-12-20,3,"Integration with Zoom breaks randomly"
reddit,https://reddit.com/r/...,2026-01-05,0,"Switched because recurring events are buggy"
hackernews,https://news.ycombinator.com/item?id=...,2025-11-30,0,"The API rate limits are absurd"
```

Rating: numeric 1–5 for rated sources; `0` for unrated (Reddit, HN) — treated as signal, not score.

---

## Sources (Priority Order)

| Priority | Source | Access method |
|----------|--------|--------------|
| P0 | App Store RSS | `https://itunes.apple.com/rss/customerreviews/id=<APP_ID>/json` |
| P0 | Google Play | Scrape public reviews (HTML, no auth) |
| P1 | G2 | Public reviews tab on product profile |
| P1 | Capterra | Public reviews tab |
| P2 | Reddit | `https://www.reddit.com/search/?q=%22<product>+sucks%22+OR+%22<product>+alternative%22` |
| P2 | Hacker News | `https://hn.algolia.com/?query=<product>&dateRange=pastYear` |
| P3 | Product roadmap board | Upvote counts on feature requests |
| P3 | Trustpilot / Sitejabber | Public reviews |

**Minimum corpus:** 50 reviews across at least 3 sources before proceeding.

---

## Fabrication Prohibition

**NEVER** invent, paraphrase, or synthesize a fake quote. Every quoted string in `feedback.md` must:
1. Appear verbatim in the source text (spelling errors preserved)
2. Have a URL in `reviews.csv` that resolves to that text
3. Have a date stamp

If fewer than 10 genuine negative reviews exist, write: `Insufficient negative signal — corpus too small.` and stop.

---

## Scoring Algorithm

Each review is scored by the Rust engine using Bayesian grievance scoring with time-decay:

```
base_score = (5 - rating) / 4         # 0.0 (5-star) to 1.0 (1-star); 0.5 for unrated
time_weight = 0.5^((now - date) / 540d) # 18-month half-life
score = base_score × time_weight
```

Themes are clustered by keyword overlap. Each theme accumulates total score across all matching reviews.

---

## CLI Invocation

```bash
replica-square entrepreneur replica/reviews.csv
```

The engine writes `replica/feedback.md` and `replica/fixes.md` automatically.

---

## Output: `replica/feedback.md`

Structure:

```markdown
# Market Feedback Analysis: <TARGET>

## Corpus
- Total reviews: N
- Sources: App Store (N), G2 (N), Reddit (N), HN (N)
- Date range: YYYY-MM-DD → YYYY-MM-DD

## Top Hated (What Users Despise)
1. <Theme> — score: X.XX, evidence: N reviews
   > "verbatim quote" — [App Store, 2026-01-10](url), ★2

## Top Missing (Gaps Users Want)
1. <Feature gap> — score: X.XX, evidence: N reviews

## Unsolved (Known Bugs / Broken Promises)
1. <Issue> — score: X.XX, evidence: N reviews
   > "verbatim quote" — [Reddit, 2026-01-05](url)
```

---

## Output: `replica/fixes.md`

Top 5–8 items ranked by: `evidence_weight × (1 / fix_cost_estimate)`

```markdown
# Fix Plan

| Rank | Problem | Evidence Score | Fix Cost | Build Priority |
|------|---------|---------------|----------|---------------|
| 1 | SMS reminders on free tier | 4.72 | S | P0 |
| 2 | Recurring event bugs | 3.91 | M | P0 |
...

## Positioning Angle
Based on the gaps above, Replica-Square's differentiated angle:
"[Target] users are most frustrated by [top theme]. Build with [top missing feature] as day-1 capability."
```

---

## `features.csv` Update Rule

For each identified gap in `fixes.md` that is NOT already in `features.csv`:
- Add a new row with `priority = must` or `should` based on evidence score
- Set `status = no` (gap = not in original = opportunity)
- Set `source_id` to the review entry ID in `recon-map.json`

Do NOT modify existing rows already in `features.csv` — append only.

---

## Handoff

After this playbook completes:
- Run `replica-square architect` — feedback themes inform API surface and schema priorities
- Review `fixes.md` with the team to validate fix cost estimates before architect runs
- Block on: at least 3 themes populated in `feedback.md`
