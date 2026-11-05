# Playbook: `replica-square test-gen`

> **Role:** Contract-Driven Playwright Test Generator  
> **Input:** `replica/recon-map.json`  
> **Output:** `e2e/flows.spec.ts`  
> **Gate:** Run after `recon` has mapped user flows (`F01`, `F02`...).

---

## Why This Tool Is Critical

Manually writing end-to-end tests is slow and prone to brittle CSS selector coupling.

`replica-square test-gen` takes the verified screen and flow inventory (`ReconFlow`) and generates battle-tested, production-ready TypeScript specifications for Playwright.

---

## Invariants Enforced in Generated Specs

1. **Zero Console Errors**: Fails tests immediately if any unhandled browser exception or warning is logged.
2. **Zero 5xx Responses**: Fails tests if any network request returns a server error status $\ge 500$.
3. **Automated Axe-Core WCAG Audits**: Every screen visit automatically executes `new AxeBuilder({ page }).analyze()` and asserts 0 violations.
4. **Multi-Tenant Isolation Test**: Verifies that User A's session context cannot see User B's state.
5. **Concurrency & Double-Submit Protection**: Exercises edge cases documented in `ReconFlow.edge_cases`.

---

## CLI Invocation

```bash
replica-square test-gen --base-url "http://localhost:3000" --out "e2e/flows.spec.ts"
```

---

## Running the Generated Tests

```bash
# Install Playwright dependencies (using Bun)
bun add -d @playwright/test @axe-core/playwright
bunx playwright install chromium

# Run the test suite
bunx playwright test
```

---

## Handoff

- Tests must pass green before `replica-square sweep` and `launch` clearance.
- Include test execution results in `replica/parity.md` verification report.
