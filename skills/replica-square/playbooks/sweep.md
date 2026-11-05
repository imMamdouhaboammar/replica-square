# Playbook: `replica-square sweep`

> **Role:** Clean-Room Compliance Gate  
> **Input:** Code root directory  
> **Output:** `replica/cleanroom-report.md`  
> **Gate:** MUST run before `launch`. Pipeline MUST NOT proceed if exit code is 1.

---

## What This Tool Checks

| Check Type | What it looks for |
|-----------|-------------------|
| Brand names | Exact strings from `--avoid` list (case-insensitive) in file content and file/dir names |
| Domains | Strings from `--domains` list in URLs, import paths, comments, strings |
| Hex colors | Exact or normalized hex values from `--colors` list |
| Trademarks | Avoid list items in strings, comments, metadata, and binary file names |

---

## Directories Always Skipped

The following are excluded unconditionally from every sweep:

- `.git/`
- `node_modules/`
- `target/` (Rust build artifacts)
- `dist/`
- `build/`
- `replica/` ← planning artifacts legitimately mention target names; never flag them
- Binary files (detected by null byte in first 8KB)
- Files > 10MB

---

## Color Normalization

All hex colors are normalized before comparison:
- Short form: `#06f` → `#0066ff`
- Uppercase: `#006BFF` → `#006bff`
- With alpha: `#006bff80` → compare against `#006bff` (alpha ignored)

---

## CLI Invocation

```bash
replica-square sweep . \
  --avoid "Calendly,Cal.com" \
  --domains "calendly.com,cal.com" \
  --colors "#006bff,#0ae8f0,#1a1a2e"
```

Flags:
- `.` — scan from current directory recursively
- `--avoid` — comma-separated competitor/target brand names
- `--domains` — comma-separated domains to flag
- `--colors` — comma-separated hex colors to flag (normalized before comparison)

**Exit code 1** if any violations found.

---

## Violation Classification

| Severity | Type | Example |
|----------|------|---------|
| ERROR | Brand name in source code string | `let name = "Calendly Pro"` |
| ERROR | Domain in production import | `import { x } from "api.calendly.com"` |
| ERROR | Exact hex color match | `color: #006bff` in a CSS file |
| WARNING | Brand name in comment | `// inspired by Calendly's approach` |
| WARNING | Domain in a comment/doc | `// see https://calendly.com/docs` |
| INFO | Brand name in test fixture | `const mockName = "CalendlyMock"` |

The tool exits 1 on any ERROR. It exits 0 with WARNINGs (but they appear in the report).

---

## Output: `replica/cleanroom-report.md`

```markdown
# Clean-Room Compliance Report

## Summary
- Scanned: 847 files
- Violations: 2 errors, 3 warnings
- **Status: ❌ FAIL — resolve errors before launch**

## Errors (Must Fix)
| File | Line | Match | Type |
|------|------|-------|------|
| src/api/client.rs | 142 | `"calendly.com"` | Domain in production string |
| src/config.rs | 87 | `#006bff` | Exact brand color |

## Warnings (Review)
| File | Line | Match | Type |
|------|------|-------|------|
| docs/ARCHITECTURE.md | 23 | `Calendly` | Brand name in doc |
...

## Skipped Directories
- replica/ (planning artifacts)
- target/ (build artifacts)
- .git/
```

---

## Failure Protocol

When exit code is 1 (errors found):

1. Open each file+line in the ERROR table
2. For string literals: replace with generic variable name or your product's name
3. For colors: replace with your design token (run `replica-square design` to verify new palette)
4. For domains: remove or replace with mock/stub URL
5. Re-run sweep until exit code 0
6. NEVER commit a build that has not passed sweep

---

## CI Pipeline Integration

Add to CI before any deploy step:

```yaml
- name: Clean-room compliance sweep
  run: |
    ./target/release/replica-square sweep . \
      --avoid "${{ vars.AVOID_NAMES }}" \
      --domains "${{ vars.AVOID_DOMAINS }}" \
      --colors "${{ vars.AVOID_COLORS }}"
```

Gate the deploy job on this step's exit code.

---

## Handoff

After this playbook completes:
- If PASS (exit 0): run `replica-square launch replica/listing.json`
- If FAIL (exit 1): fix all ERROR rows, re-run sweep
- Block on: exit code 0
