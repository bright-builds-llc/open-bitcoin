# Phase 98: Traceability Reconciliation - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md - this log preserves the alternatives considered.

**Date:** 2026-06-28T19:20:44.567Z
**Phase:** 98-traceability-reconciliation
**Mode:** Yolo
**Areas discussed:** Canonical requirement ownership, artifact reconciliation, deterministic checker and verification

---

## Canonical Requirement Ownership

| Option | Description | Selected |
|--------|-------------|----------|
| Phase 90 remains canonical owner for INB-01 through INB-04 | Preserve the original implementation phase as owner, leaving the audit's stale ownership finding unresolved. | |
| Phase 98 becomes canonical closure for INB-01 through INB-04 and BOUND-06 | Treat Phase 90 as historical implementation evidence and Phase 98 as the reconciliation owner that closes stale requirement status. | yes |
| Broadly rewrite all prior phase summaries as if the new ownership always existed | Creates historical churn and risks obscuring the evidence trail. | |

**User's choice:** Auto-selected recommended option: Phase 98 becomes canonical closure for `INB-01` through `INB-04` and `BOUND-06`.
**Notes:** This matches the Phase 98 roadmap scope and the Phase 95 checker’s target requirement assignment map. Phase 97 remains canonical for `INB-05` and `DOS-04`; Phase 96 remains canonical for `EVICT-03`, `EVICT-04`, and `DOS-03`.

---

## Artifact Reconciliation

| Option | Description | Selected |
|--------|-------------|----------|
| Update only requirements and roadmap | Fixes the most visible tables but leaves audit, verification, and release-readiness artifacts inconsistent. | |
| Reconcile requirements, roadmap, state, audit, selected verification reports, and release-readiness docs | Closes the full traceability loop required by BOUND-06 without expanding runtime scope. | yes |
| Rebuild the whole v1.9 planning history | Too broad for Phase 98 and likely to create avoidable churn in historical artifacts. | |

**User's choice:** Auto-selected recommended option: reconcile the full set of current traceability artifacts while preserving historical evidence.
**Notes:** The key distinction is evidence versus canonical ownership. Historical summaries and verification reports can remain as evidence records when they are annotated clearly.

---

## Deterministic Checker And Verification

| Option | Description | Selected |
|--------|-------------|----------|
| Rely on manual audit review only | Leaves stale-count regressions easy to reintroduce and does not match recent phase checker patterns. | |
| Add a focused Phase 98 Bun checker and fixture test, then wire it after Phase 97 in `scripts/verify.sh` | Matches repo-owned verification style and makes traceability closure repeatable. | yes |
| Scan the entire repository for traceability prose | Too brittle and likely to fail on historical archives or unrelated mentions. | |

**User's choice:** Auto-selected recommended option: add a focused Phase 98 checker over curated files.
**Notes:** The checker should follow Phase 95 through Phase 97 patterns: explicit file inputs, stable assertions, fixture mutation tests, and default `bash scripts/verify.sh` integration.

---

## Claude's Discretion

- Exact checker helper names, fixture construction, and failure-message wording.
- The precise wording of historical verification ownership notes.
- Whether to update historical summaries directly or rely on verification notes, as long as the final audit sees exact, current ownership.

## Deferred Ideas

- Transaction relay, compact block relay, mempool propagation, full address relay, public inbound defaults, public-network CI, production service operation, and production full-node readiness remain outside Phase 98.
