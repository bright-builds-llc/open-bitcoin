# Phase 82: Production Claim Boundary - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution
> agents. Decisions are captured in CONTEXT.md - this log preserves the
> alternatives considered.

**Date:** 2026-06-21T12:38:38.140Z
**Phase:** 82-Production Claim Boundary
**Mode:** Yolo
**Areas discussed:** Production Vocabulary, Evidence Gate Model, Deferred Surface Inventory, Documentation Shape, Verification And Traceability

---

## Production Vocabulary

| Option | Description | Selected |
| --- | --- | --- |
| Five-term controlled vocabulary | Define supported, preview, opt-in UAT, unsupported, and deferred as the only support terms used for v1.8 release language. | yes |
| Existing prose only | Continue using the historical release-boundary paragraphs without a new support glossary. | |
| Marketing-style readiness label | Let production-readiness wording vary by document and rely on context. | |

**User's choice:** Auto-selected five-term controlled vocabulary.
**Notes:** This best satisfies PROD-01 and keeps v1.8 as boundary-setting rather
than a production readiness claim.

---

## Evidence Gate Model

| Option | Description | Selected |
| --- | --- | --- |
| Claim-to-evidence matrix | Map allowed statements to support term, evidence sources, verification command, UAT status, residual risk, and next gate. | yes |
| Narrative checklist | Describe the gates in prose without a structured traceability table. | |
| Immediate broad default checker | Build the full production-claim scanner in Phase 82. | |

**User's choice:** Auto-selected claim-to-evidence matrix.
**Notes:** The matrix closes Phase 82 traceability while keeping broad default
guardrails scoped to Phase 88 unless a narrow consistency check is needed.

---

## Deferred Surface Inventory

| Option | Description | Selected |
| --- | --- | --- |
| Preserve full non-claim inventory | Carry forward every production-adjacent deferred surface from v1.8 requirements and previous release-boundary docs. | yes |
| Summarize by category | Collapse deferred surfaces into broad categories such as networking, wallet, packaging, and UI. | |
| Mention only Phase 82 surfaces | Limit the inventory to production terminology and omit later-phase surfaces. | |

**User's choice:** Auto-selected preserve full non-claim inventory.
**Notes:** The exact list prevents accidental broadening before support matrix,
upgrade, runbook, service, release-readiness, and guardrail phases build on it.

---

## Documentation Shape

| Option | Description | Selected |
| --- | --- | --- |
| Canonical boundary doc plus links | Add one production boundary document and link it from README, runtime guide, release readiness, checklist, parity README, parity index, and deviations register. | yes |
| Release-readiness only | Put all new Phase 82 content into `docs/parity/release-readiness.md`. | |
| README-first narrative | Make README the primary production-boundary source. | |

**User's choice:** Auto-selected canonical boundary doc plus links.
**Notes:** A single doc avoids duplicate matrices while preserving reviewer
discoverability from existing parity roots.

---

## Verification And Traceability

| Option | Description | Selected |
| --- | --- | --- |
| Docs and parity metadata first | Run focused checks during iteration and final `bash scripts/verify.sh`; add automation only if needed for traceability. | yes |
| Add a Phase 82 checker by default | Create a new deterministic checker even if docs and parity roots are sufficient. | |
| Defer all verification changes | Write docs only and leave all traceability checks to Phase 88. | |

**User's choice:** Auto-selected docs and parity metadata first.
**Notes:** This keeps Phase 82 small and auditable, while still allowing a narrow
Bun checker if planning finds that docs alone cannot satisfy PROD-02 or PROD-03.

---

## the agent's Discretion

- Planner may choose the smallest set of docs, parity roots, and optional local
  checks that satisfies PROD-01 through PROD-04.
- Executor may avoid Rust changes unless a narrow behavior/status wording issue
  is discovered.
- Executor may keep broad claim-scanner work deferred to Phase 88.

## Deferred Ideas

- Full default-verification production-claim scanner belongs to Phase 88.
- Production full-node readiness claim remains future work after all gates pass.
