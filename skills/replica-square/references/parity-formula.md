# Reference: Parity Formula Derivation

## Overview

The Replica-Square parity score measures how completely a clone application implements the original product's observable feature set, weighted by feature criticality and optionally adjusted by visual structural similarity.

---

## Feature Parity (80% weight)

### Priority Weight Rationale

| Priority | Weight | Rationale |
|----------|--------|----------|
| `must` | 3 | Core differentiating features — without these, the product is a different category |
| `should` | 2 | Standard-tier features — users expect them; absence causes churn |
| `could` | 1 | Nice-to-have features — differentiators but not table stakes |

The 3:2:1 ratio reflects the economic reality that losing a `must` feature has ~3× the negative impact of losing a `could` feature on user retention (derived from review corpus grievance scoring).

### Credit Values

| `status` column | Credit $c_i$ | When to use |
|-----------------|-------------|-------------|
| `yes` | 1.0 | Feature is fully implemented and observable |
| `partial` | 0.5 | Feature exists but with a known, documented gap |
| `no` | 0.0 | Feature is absent from the clone |
| `skip` | excluded | Feature is out of scope (see skip-list rules) |

### Formula

$$
\text{Feature Score} = \frac{\sum_{i \in \text{scored}} w_i \cdot c_i}{\sum_{i \in \text{scored}} w_i} \times 100
$$

Scored features: all rows where `status ∈ {yes, partial, no}`.

### Example Calculation

| Feature | Priority | Weight | Status | Credit | Weighted |
|---------|----------|--------|--------|--------|---------|
| Event type creation | must | 3 | yes | 1.0 | 3.0 |
| Calendar integration | must | 3 | yes | 1.0 | 3.0 |
| Team scheduling | must | 3 | partial | 0.5 | 1.5 |
| Recurring events | must | 3 | no | 0.0 | 0.0 |
| SMS reminders | should | 2 | yes | 1.0 | 2.0 |
| Zapier integration | should | 2 | no | 0.0 | 0.0 |
| Custom branding | could | 1 | yes | 1.0 | 1.0 |

$$
\text{Feature Score} = \frac{3.0 + 3.0 + 1.5 + 0.0 + 2.0 + 0.0 + 1.0}{3 + 3 + 3 + 3 + 2 + 2 + 1} \times 100 = \frac{10.5}{17} \times 100 \approx 61.8\%
$$

---

## Visual Parity (20% weight, optional)

Visual parity measures layout and structural similarity — not pixel perfection. It answers: "Does the clone have a similar information density and structural composition to the original?"

### Algorithm

1. Load original and clone images
2. Convert to grayscale (Luma channel)
3. Normalize both to 480px width (preserve aspect ratio via bicubic interpolation)
4. Apply Sobel edge detection (3×3 kernel) → edge magnitude map
5. Divide into a 4×4 grid (16 cells of equal size)
6. Per cell: `density = (pixels with edge magnitude > threshold) / total_pixels`
7. Per-cell delta: `|original_density - clone_density|`
8. Visual score: `(1 - mean(all_deltas)) × 100`

### Why 480px normalization?

Prevents resolution differences from inflating or deflating scores. 480px is the standard mobile viewport width — the common denominator for both web and app screenshots.

### Why edge density?

Edge density correlates with information density and structural complexity without being sensitive to color (which is a design choice, not a structural one). A dashboard with many charts has high edge density; a splash screen has low edge density.

### Why 4×4 grid?

Coarse enough to capture macro-layout (navbar, sidebar, content area, footer) without penalizing micro-layout differences (button placement, padding). Finer grids (8×8) produce noisy scores sensitive to small UI differences.

---

## Combined Score Formula

$$
\text{Parity Score} = \begin{cases}
0.80 \times F + 0.20 \times V & \text{if screenshots provided} \\
F & \text{if no screenshots}
\end{cases}
$$

Where $F$ = Feature Score (%), $V$ = Visual Score (%).

### Why 80/20 split?

- Features are verifiable from `features.csv` with deterministic, REA-anchored evidence
- Visual parity is more subjective and can be gamed (similar screenshots ≠ similar functionality)
- 80% feature weight ensures the score reflects working software, not styled mockups

### Default threshold: 80%

A score ≥ 80% means the clone covers the dominant portion of the original's weighted feature value. Below 80% means significant `must` features are missing — the product is not yet a viable substitute.

---

## Skip Row Rules

A feature is excluded from scoring (not counted in denominator or numerator) when:

1. User explicitly listed it in `SKIP_LIST` during `recon` pre-flight
2. Feature requires proprietary third-party licensing (e.g., a patented algorithm)
3. Feature is infrastructure-only (no user-visible manifestation)
4. Feature cannot be verified from public evidence (not observed, not inferable)

**Skipped features are NOT counted as 0 (failures) — they are removed from the formula entirely.** This prevents skipped out-of-scope items from artificially lowering the score.
