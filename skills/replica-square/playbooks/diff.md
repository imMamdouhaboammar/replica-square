# Playbook: `replica-square diff`

> **Role:** Parity Scorer  
> **Input:** `replica/features.csv` (required), optional screenshot PNGs  
> **Output:** `replica/parity.md`  
> **Gate:** Run after `recon` has populated `features.csv` with at least 10 rows.

---

## Feature Parity Formula

$$
\text{Feature Score} = \frac{\sum_{i} w_i \cdot c_i}{\sum_{i} w_i} \times 100
$$

Where:

| Variable | Meaning | Value |
|----------|---------|-------|
| $w_i$ | Priority weight of feature $i$ | `must`=3, `should`=2, `could`=1 |
| $c_i$ | Credit for feature $i$ | `yes`=1.0, `partial`=0.5, `no`=0.0, `skip`=excluded |

Features with `status = skip` are excluded from both numerator and denominator.

---

## Visual Parity (Optional)

When `--original-png` and `--clone-png` are provided, the engine computes an edge density score:

1. Convert both images to grayscale
2. Normalize to 480px width (preserve aspect ratio)
3. Apply Sobel edge detection
4. Divide into a 4×4 grid (16 cells)
5. For each cell, compute edge pixel density: `edge_pixels / total_pixels`
6. Visual score: `1 - mean(|orig_density[i] - clone_density[i]|)` × 100

---

## Combined Score

$$
\text{Parity Score} = 0.80 \times \text{Feature Score} + 0.20 \times \text{Visual Score}
$$

If no screenshots are provided, `Parity Score = Feature Score` (visual weight ignored, not penalised).

---

## CLI Invocation

```bash
# Feature parity only
replica-square diff replica/features.csv --fail-under 80

# Feature + visual parity
replica-square diff replica/features.csv \
  --fail-under 80 \
  --original-png reference/calendly-dashboard.png \
  --clone-png screenshots/my-app-dashboard.png
```

**Exit code 1** if Parity Score < `--fail-under` threshold (default: 80).

---

## Output: `replica/parity.md`

```markdown
# Feature Parity Report

## Summary
| Metric | Score |
|--------|-------|
| Feature Score | 85.3% |
| Visual Score  | 72.1% |
| **Combined**  | **82.7%** |
| Threshold     | 80% |
| **Status**    | ✅ PASS |

## Feature Breakdown
| Priority | Total | Yes | Partial | No | Skipped | Weight |
|----------|-------|-----|---------|----|---------|----|
| must (×3) | 12 | 10 | 1 | 1 | 0 | 36/39 |
| should (×2) | 8 | 5 | 2 | 1 | 0 | 14/18 |
| could (×1) | 5 | 3 | 0 | 2 | 0 | 3/5 |

## Missing P0 (must + no) — Build Backlog
| ID | Feature | Size | Suggested Sprint |
|----|---------|------|-----------------|
| F007 | Recurring events | L | Sprint 2 |
...

## Visual Grid Delta
| Cell | Original Density | Clone Density | Delta |
|------|-----------------|--------------|-------|
| R0C0 | 0.182 | 0.174 | 0.008 |
...
```

---

## Failure Protocol

When Parity Score < threshold:

1. Extract all rows where `priority = must AND status = no` → these are the P0 missing features
2. Write as build backlog table in `parity.md` under `## Missing P0`
3. Add estimated size (from `features.csv` size column if present)
4. Do NOT proceed to `launch` until a subsequent run passes the threshold

---

## Interpreting Partial (0.5)

Use `partial` when:
- The feature exists but has a known gap (e.g. recurring events work but daily/weekly only, not custom intervals)
- The feature is hidden behind a higher pricing tier (confirmed exists but gated)
- The feature has been announced on the roadmap but not yet shipped

Do NOT use `partial` speculatively. Must be backed by evidence in `recon-map.json`.

---

## Handoff

After this playbook completes:
- If PASS: proceed to `replica-square sweep` and then `replica-square launch`
- If FAIL: return to implementation, address `## Missing P0` items, then re-run diff
- Block on: at least 10 scored features (non-skip) in `features.csv`
