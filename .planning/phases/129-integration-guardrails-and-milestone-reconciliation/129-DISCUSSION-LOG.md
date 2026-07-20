# Phase 129: Integration Guardrails and Milestone Reconciliation - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-07-20
**Phase:** 129-integration-guardrails-and-milestone-reconciliation
**Mode:** Yolo
**Areas discussed:** Aggregate guard shape, Flow coverage strategy, Phase 124 stage machine evolution, Requirement closure and audit rerun, Runtime scope

---

## Aggregate Guard Shape

| Option | Description | Selected |
|--------|-------------|----------|
| New Phase 129 checker pair | New `check-phase129-...ts` + `.test.ts`, wired after 128 and before the final 117 no-claim gate | ✓ |
| Extend Phase 127/128 checkers | Fold aggregate assertions into the existing phase-local guards | |
| Extend Phase 124 reconciliation checker | Add flow/seam assertions to the closeout stage machine | |

**Choice rationale:** Matches the phase-owned checker convention, keeps 124 a lifecycle/stage machine, keeps 117 the last no-claim gate. Research confirmed the `orderedLines` subsequence assertions in upstream checkers tolerate the insertion.

---

## Flow Coverage Strategy

| Option | Description | Selected |
|--------|-------------|----------|
| Compose existing seam guards plus cross-phase assertions | Reuse exported 127/128 check functions where practical; add FLOW-01..04 naming and archive-contract checks they cannot express | ✓ |
| Duplicate all anchors in the new checker | Re-implement every seam anchor independently | |
| New Rust end-to-end super-test | One integration test spanning all four flows | |

**Choice rationale:** The six seams and the four flows' Rust test anchors already exist (127 covers FLOW-01/04, 128 covers FLOW-02/03). The aggregate guard's marginal value is naming the flows, binding them together, and guarding the reconciled milestone contract — not re-proving seams.

---

## Phase 124 Stage Machine Evolution

| Option | Description | Selected |
|--------|-------------|----------|
| Add explicit archive-ready post-129 stage | Extend post-audit gap-planning checker with a new fail-closed stage accepting the reconciled end-state | ✓ |
| Loosen existing post-audit assertions | Make gaps_found/29-39 assertions optional | |
| Retire the post-audit checker after 129 | Delete the stage instead of evolving it | |

**Choice rationale:** The current stage hard-requires `status: gaps_found`, `29/39`, and Phase 129 pending, so reconciliation would fail verification without evolution. Explicit distinct states follow the Phase 126/128 fail-closed closeout precedent; loosening or deleting would reopen the drift the audit caught.

---

## Requirement Closure And Audit Rerun

| Option | Description | Selected |
|--------|-------------|----------|
| Independent verifier closes all 10; in-place audit rerun to passed | gsd-verifier re-attests 7 Complete and newly closes OBS-01/BOUND-02/HARD-05; update `.planning/v2.1-MILESTONE-AUDIT.md` in place per v1.0/v1.9/v2.0 precedent | ✓ |
| Companion rerun audit file | Keep gaps_found audit as baseline, add `v2.1-MILESTONE-AUDIT-RERUN.md` (v1.1 precedent) | |
| Close only the 3 Pending requirements | Skip re-attesting the 7 already Complete | |

**Choice rationale:** In-place supersede is the dominant precedent and keeps a single canonical audit. The roadmap success criterion explicitly requires independent closure of all 10 reassigned requirements. Archival itself (`/gsd-complete-milestone v2.1`) stays out of phase scope.

---

## Runtime Scope

| Option | Description | Selected |
|--------|-------------|----------|
| Guards only; fix runtime only on proven defect | No new runtime features; minimal fail-closed fix if flow verification exposes a real truthfulness defect | ✓ |
| Proactive OBS-01 re-plumbing | Add new status fields or re-source facets | |

**Choice rationale:** Research verified all six OBS-01 facets already project from the single authoritative `ManagedNetworkHandle`; the audit's OBS-01 finding was the pre-127 second network, which no longer exists. The fallback-counter snapshot semantics were flagged as a truthfulness nuance to check, not a wiring gap.

---

## Claude's Discretion

- Exact stage names and fixtures for the Phase 124 evolution.
- Import-vs-reassert composition for 127/128 anchors in the new checker.
- Plan split between guard work and reconciliation work.
- Minimal new Rust flow-test anchors, if any, beyond the existing corpus.

## Deferred Ideas

- `/gsd-complete-milestone v2.1` archival run after Phase 129 routes there.
- Refactoring the 1,505-line `check-phase124-milestone-gap-closure.ts` (non-blocking debt).
- All v2.1-deferred surfaces (package relay, filters, public defaults, production readiness, production-funds, packaging, GUI, hosted services).
