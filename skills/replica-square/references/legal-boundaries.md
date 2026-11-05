# Reference: Legal Boundaries for Clean-Room Replication

## What This Document Governs

Replica-Square is a clean-room analysis and build-planning tool. This document defines what agents may and may not include in a recon, design, or build based on legal and ethical boundaries.

---

## ✅ What CAN Be Replicated (Safe)

| Category | Examples | Basis |
|----------|---------|-------|
| **UX flows and interaction patterns** | Multi-step onboarding, wizard forms, drag-and-drop scheduling | UX patterns are not copyrightable (Baker v. Selden doctrine) |
| **Feature concepts** | "Users can create recurring events", "30-min buffer after meetings" | Ideas are not copyrightable |
| **Information architecture** | Sidebar nav with sections, dashboard with quick-actions | General UI structure is functional, not expressive |
| **API surface design** | REST endpoints: `POST /bookings`, `GET /users/:id/availability` | API contracts are functional interfaces, not creative expression |
| **Data model shape** | `booking { id, host_id, start_at, duration_minutes, status }` | Database schema derived from observed UI is fair analysis |
| **Business model** | Freemium with seat-based pricing, upgrade prompts at key flows | Business models are not protectable |
| **Pricing structure** | Free tier: 1 calendar, Paid: unlimited | Factual pricing information is public |

---

## ❌ What CANNOT Be Replicated (Prohibited)

| Category | Examples | Reason |
|----------|---------|--------|
| **Source code** | Copying JS, Python, or any source file from a competitor's repo | Copyright violation |
| **Compiled binary content** | Decompiling and copying application logic | Copyright + likely CFAA |
| **Trademarks** | Using the original app's name, logo, or brand colors in your product | Trademark infringement |
| **Trade secrets** | Proprietary algorithms disclosed under NDA | Trade secret misappropriation |
| **Licensed content** | Icon sets, fonts, or libraries under non-permissive licenses | License violation |
| **User data** | Scraping or importing the original app's user accounts, emails, or records | GDPR/CCPA/CFAA violations |
| **Private APIs** | Accessing undocumented internal endpoints requiring reverse-engineering | CFAA / ToS violation |
| **Terms of Service violations** | Automated scraping prohibited by ToS | Breach of contract, potential CFAA |
| **The user network** | Attempting to migrate or import the original app's users | Privacy law violations |

---

## Brand Color Rule

The `replica-square sweep` tool flags competitor hex colors. This boundary exists because:

- Using a competitor's exact brand palette in a competing product can constitute **trade dress infringement** (Lanham Act § 43(a))
- Even if colors are individually unprotectable, the specific combination used in a commercial context can be protectable

**Safe practice:** Derive your palette from your own brand brief. Run `replica-square design` to verify WCAG 2.2 compliance of your palette.

---

## Terms of Service (ToS) Flag Protocol

If during `recon` the agent discovers that:

1. The target product's ToS explicitly prohibits automated access, scraping, or competitive analysis
2. Any source URL requires login to a paid account beyond the free tier

Stop and record in `replica/recon.md`:

```markdown
## ToS Flag
- Source: [URL]
- Issue: ToS Section X prohibits [specific action]
- Decision: [Skip this source / Proceed with manual observation only / Escalate to legal]
```

Do NOT proceed with that source without a decision. Default: skip the source, mark it ⚠️ in the Source Table.

---

## Skip-Row Rules for `features.csv`

Set `status = skip` and document the reason in `notes` when:

| Condition | Example note |
|-----------|-------------|
| Feature requires a patented algorithm | `[skip] US patent 10,XXX,XXX covers this algorithm` |
| Feature data cannot be observed publicly | `[skip] requires paid enterprise account; not in scope` |
| Feature requires accessing user-generated private data | `[skip] user calendar data not publicly accessible` |
| ToS explicitly prohibits replication of this feature | `[skip] ToS Section 4.2 prohibits scraping availability data` |

---

## Clean-Room Process Requirement

For the recon output to be legally defensible as clean-room work:

1. No person who has seen the original source code may contribute to this recon or the resulting build
2. All observations come from the public-facing product only
3. All evidence is SHA-256 anchored (not from memory or second-hand accounts)
4. The `replica/` planning artifacts and resulting codebase contain zero strings, colors, or structures from the `--avoid` list (verified by `replica-square sweep`)

---

## Disclaimer

Replica-Square is a productivity tool. This reference provides general guidance, not legal advice. For any replication involving significant commercial investment or regulatory-sensitive industries (healthcare, finance, education), consult a qualified IP attorney before proceeding.

The agents using this tool are responsible for compliance with applicable laws in their jurisdiction.
