# Reference: Pipeline State & Artifact Conventions

## Artifact Directory Layout

All Replica-Square artifacts live in `replica/` relative to the project root. This directory is:
- Created automatically by `replica-square run`
- Never committed to source control (add `replica/` to `.gitignore` for most projects)
- Never swept by `replica-square sweep` (planning artifacts legitimately reference target names)

```
replica/
├── recon.md              # Human-readable recon report
├── recon-map.json        # Machine-readable evidence map
├── features.csv          # Feature matrix (scored parity input)
├── reviews.csv           # Raw review corpus (entrepreneur input)
├── feedback.md           # Grievance analysis output
├── fixes.md              # Prioritized fix plan
├── architecture.md       # Technical architecture plan
├── schema.sql            # PostgreSQL DDL
├── tokens.json           # Design token source
├── tokens-audit.md       # WCAG 2.2 contrast audit
├── tailwind.tokens.js    # Build-ready token export
├── parity.md             # Feature + visual parity score
├── cleanroom-report.md   # Brand sweep results
└── listing-audit.md      # App store listing lint results
```

---

## Pipeline State Machine

```
IDLE
  ↓ replica-square recon
RECON_COMPLETE        — gate: recon.md, features.csv, recon-map.json exist
  ↓ replica-square entrepreneur (optional)
ENTREPRENEUR_COMPLETE — gate: feedback.md, fixes.md exist
  ↓ replica-square architect
ARCHITECT_COMPLETE    — gate: architecture.md, schema.sql exist
  ↓ replica-square design
DESIGN_COMPLETE       — gate: tailwind.tokens.js exists, design exit=0
  ↓ replica-square diff
DIFF_COMPLETE         — gate: parity.md exists, parity ≥ threshold
  ↓ replica-square sweep
SWEEP_COMPLETE        — gate: cleanroom-report.md exists, sweep exit=0
  ↓ replica-square launch
LAUNCH_COMPLETE       — gate: listing-audit.md exists, launch exit=0
```

Agents MUST NOT skip a gate. Verify file existence before invoking the next tool.

---

## State Verification (Shell)

```bash
# Check gate: RECON_COMPLETE
test -f replica/recon.md && test -f replica/features.csv && test -f replica/recon-map.json \
  && echo "GATE: recon complete" || echo "GATE: recon NOT complete"

# Check gate: SWEEP_COMPLETE
test -f replica/cleanroom-report.md \
  && grep -q "Status: ✅ PASS" replica/cleanroom-report.md \
  && echo "GATE: sweep passed" || echo "GATE: sweep NOT passed"
```

---

## Artifact Naming Conventions

| Artifact | Naming rule | Notes |
|----------|------------|-------|
| `recon.md` | Fixed name | Always `replica/recon.md` |
| `features.csv` | Fixed name | Shared across all pipeline stages |
| `recon-map.json` | Fixed name | Evidence index; append-only after creation |
| `schema.sql` | Fixed name | Accumulates tables across runs |
| `tokens.json` | Fixed name | Edited by user; source of truth for design |

Evidence IDs in `recon-map.json` follow format `S##` for screens, `F##` for flows, `API##` for API endpoints, `RV##` for reviews.

---

## Shared Artifact Read/Write Rules

| Agent stage | Reads | Writes | Appends |
|------------|-------|--------|---------|
| recon | — | recon.md, recon-map.json | — |
| recon | — | features.csv | — |
| entrepreneur | reviews.csv | feedback.md, fixes.md | features.csv (new rows only) |
| architect | recon.md, features.csv | architecture.md, schema.sql | — |
| design | tokens.json | tailwind.tokens.js, tokens-audit.md | — |
| diff | features.csv | parity.md | — |
| sweep | all src/ | cleanroom-report.md | — |
| launch | listing.json | listing-audit.md | — |

**Append-only rule:** `entrepreneur` may only append rows to `features.csv`; it must never modify or delete existing rows.
