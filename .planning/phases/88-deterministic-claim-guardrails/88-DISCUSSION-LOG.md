# Phase 88: Deterministic Claim Guardrails - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution
> agents. Decisions are captured in CONTEXT.md — this log preserves the
> alternatives considered.

**Date:** 2026-06-23T20:39:39.056Z
**Phase:** 88-Deterministic Claim Guardrails
**Mode:** Yolo
**Areas discussed:** Claim Scan Boundary, Evidence Gates, Deferred-Surface Promotion Guardrails, Verifier Integration And Regression Tests

---

## Claim Scan Boundary

| Option | Description | Selected |
|--------|-------------|----------|
| Current release docs only | Scan README, release-readiness, production claim boundary, support matrix, deviations, parity checklist/readme/index, and verifier script. Low false positives, but may miss operator or catalog overclaims. | |
| Release plus public operator surface | Scan release roots plus runtime guide and relevant parity catalog pages. Catches public-facing readiness drift while avoiding whole-docs-tree noise. | ✓ |
| Whole docs tree scanner | Broadest coverage, but likely false positives against historical scoped claims, planning files, valid no-claim prose, and examples. | |

**User's choice:** Auto-selected release plus public operator surface.
**Notes:** Advisor output recommended curated release/operator scanning with allow
rules for explicit no-claim, deferred, historical, opt-in UAT, future-gate, and
outside-default-verification wording.

---

## Evidence Gates

| Option | Description | Selected |
|--------|-------------|----------|
| Hybrid existing-root gate parser plus curated scanner | Uses production-claim-boundary, support-matrix, and release-readiness as sources of truth, with row parsing where possible and sentence scanning for prose drift. | ✓ |
| Exact phrase denylist | Simple fixture coverage for known bad phrases, but weak against paraphrases and evidence-gate semantics. | |
| New machine-readable claim-gate registry | Clean schema, but conflicts with current no-new-v1.8-manifest direction and duplicates docs/parity roots. | |
| Whole-repo all-doc scanner | Broad catch-all but noisy across historical planning and parity archives. | |

**User's choice:** Auto-selected hybrid existing-root gate parser plus curated scanner.
**Notes:** The selected approach keeps canonical docs authoritative and avoids a
new evidence registry while still catching obvious overclaims.

---

## Deferred-Surface Promotion Guardrails

| Option | Description | Selected |
|--------|-------------|----------|
| Targeted release-doc claim scanner with allowed no-claim contexts | Catches implied promotions across release/operator docs and allows explicit deferred/no-claim wording. | ✓ |
| Canonical matrix promotion gate | Low false positives in tables, but misses prose outside canonical matrices. | |
| Registry-driven deferred-surface rules | Evolvable single inventory, but risks creating a second source of truth. | |
| Exact denylist only | Simple supplemental smoke coverage, but misses variants or blocks valid no-claim text. | |

**User's choice:** Auto-selected targeted scanner with allowed no-claim contexts.
**Notes:** The checker should fail promotion predicates like production-ready,
supported, default-verified, release-blocking, proven, GA, and certified when
attached to Phase 82 deferred surfaces.

---

## Verifier Integration And Regression Tests

| Option | Description | Selected |
|--------|-------------|----------|
| New Phase 88 Bun checker plus Bun fixture test wired after Phase 87 | Matches the Phase 82-87 pattern, keeps Bun as repo-owned automation, supports fixture temp roots and executed verifier order checks. | ✓ |
| Extend Phase 87 checker | Fewer files, but blurs completed Phase 87 scope and makes REL-02/REL-03/REL-04 failures harder to trace. | |
| Run checker only in default verify and leave tests targeted | Shorter default verification, but weakens the default regression gate. | |
| Shell or rg guard in verify.sh | Minimal plumbing, but brittle and poor for structured docs/JSON. | |

**User's choice:** Auto-selected new Phase 88 Bun checker plus fixture tests.
**Notes:** Wire both test and checker after Phase 87 in the heredoc and executed
`run_step` sequence. Use `OPEN_BITCOIN_PHASE88_REPO_ROOT` fixture overrides and
validate executable verifier text, not only heredoc text.

---

## the agent's Discretion

- Planner may choose exact plan splits.
- Executor may factor local helpers if it improves clarity without creating a
  separate claim-gate registry.
- Executor decides the precise scoped-context matcher, as long as valid
  no-claim/deferred text is accepted and positive promotion text fails.

## Deferred Ideas

None.
