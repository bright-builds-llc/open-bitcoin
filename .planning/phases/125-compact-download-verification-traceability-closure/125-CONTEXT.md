---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 125-2026-07-17T13-21-01
generated_at: 2026-07-17T13:21:01.182Z
---

# Phase 125: Compact Download Verification Traceability Closure - Context

**Gathered:** 2026-07-17
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Phase 125 restores lifecycle-valid verification ownership for `RCN-04`, `RCN-05`, and `RCN-06` by mapping each requirement explicitly to the already completed Phase 115 compact-download request, response, and validation-handoff evidence. It adds a deterministic repository guard that rejects active requirements which are assigned and summary-complete but absent from every lifecycle-valid phase verification artifact.

This is verification-traceability, checker, and milestone-projection work. It must not change compact-download runtime behavior, rewrite Phase 115 implementation history, duplicate runtime claims, absorb Phase 126 compact-relay hardening, or claim final v2.1 archive readiness.
</domain>

<decisions>
## Implementation Decisions

### Verification Ownership

- **D-01:** Phase 125 is the canonical lifecycle-valid closure owner for `RCN-04`, `RCN-05`, and `RCN-06`, matching the active roadmap and requirements traceability map.
- **D-02:** Keep Phase 115 verification and summaries immutable as historical implementation evidence. Do not retroactively add requirement IDs to `115-VERIFICATION.md`.
- **D-03:** Create a Phase 125 verification artifact that names all three requirement IDs explicitly and maps each to the relevant Phase 115 summary, observable truth, production symbol, and focused test evidence.
- **D-04:** Phase 125 claims verification-traceability closure and deterministic enforcement only. It must not describe the Phase 115 runtime behavior as newly implemented or independently duplicate its substantive runtime claims.

### Deterministic Verification-Orphan Guard

- **D-05:** Implement a reusable Bun/TypeScript checker over the active milestone rather than a v2.1-only requirement manifest.
- **D-06:** Derive the checked corpus from non-deferred active requirements and their exactly-one traceability assignments. A requirement becomes subject to the orphan gate when an in-scope phase summary lists it in `requirements-completed`.
- **D-07:** Treat a requirement as covered only when its exact ID appears in a lifecycle-valid `*-VERIFICATION.md` artifact belonging to a roadmap-listed active-milestone phase. Archived milestone token collisions and invalid-lifecycle verification artifacts must not satisfy coverage.
- **D-08:** Fail closed on malformed or duplicate active ownership and lifecycle metadata. Ignore genuinely pending, unsummarized, deferred, and archived requirements so incomplete future work does not break normal iteration.
- **D-09:** Add focused temporary-filesystem fixtures and one real-repository smoke test covering complete corpus success, independent missing-token failures, pending/deferred exclusions, duplicate ownership, archived collisions, malformed metadata, and invalid-lifecycle masking.
- **D-10:** Wire the checker test and checker into both the visible command-order block and executable steps in `scripts/verify.sh`.

### Milestone Projection Sequencing

- **D-11:** Preserve Phase 125 as the exactly-one canonical owner for the three requirements while retaining Phase 115 as the linked implementation-evidence source.
- **D-12:** Correct stale project and state wording that still claims archive readiness from the superseded Phase 124 audit, but keep `RCN-04`, `RCN-05`, and `RCN-06` pending until Phase 125 verification and lifecycle validation pass.
- **D-13:** Promote the three requirements and Phase 125 to complete only after the new verification artifact, focused checker suite, repo-native verification, and lifecycle validation are clean.
- **D-14:** Refresh the canonical milestone audit to show the three orphan gaps closed by Phase 125 without weakening or duplicating Phase 115 runtime evidence. Keep the audit non-passed and archival routing blocked while Phase 126 requirements remain pending.
- **D-15:** Leave compact-relay residual hardening, final metadata reconciliation, the final passed audit, and archive routing to Phase 126.

### Scope Isolation

- **D-16:** No Rust production or test changes are expected. If planning discovers a runtime defect rather than a traceability defect, stop and route it to Phase 126 or a new gap phase instead of expanding Phase 125.
- **D-17:** Preserve the deterministic, public-network-free default verification boundary and all existing no-claim constraints.

### Folded Todos

No pending todos matched Phase 125.

### the agent's Discretion

The planner may choose checker module and helper names, fixture organization, and the precise split between checker implementation, metadata projection, and final verification plans. Prefer explicit parsers, fixed failure messages, small fixture builders, and localized planning-artifact edits.

</decisions>

<canonical-refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Project Rules And Phase Scope

- `AGENTS.md` — repo-local GSD, verification, generated-artifact, Bun scripting, and parity rules.
- `AGENTS.bright-builds.md` — managed Bright Builds workflow and cross-cutting standards.
- `standards-overrides.md` — active local exceptions; currently no substantive override applies.
- `standards/core/architecture.md` — functional-core and imperative-shell boundaries.
- `standards/core/code-shape.md` — TypeScript and script readability constraints.
- `standards/core/testing.md` — focused Arrange/Act/Assert test expectations.
- `standards/core/verification.md` — repo-native verification and clean commit gate.
- `standards/languages/typescript-javascript.md` — Bun/TypeScript automation conventions.
- `.planning/ROADMAP.md` — Phase 125 goal, dependencies, requirements, and success criteria.
- `.planning/REQUIREMENTS.md` — active `RCN-04`, `RCN-05`, and `RCN-06` ownership and status.
- `.planning/PROJECT.md` — current v2.1 claim and stale archive-readiness narrative.
- `.planning/STATE.md` — active milestone routing and lifecycle state.
- `.planning/v2.1-MILESTONE-AUDIT.md` — authoritative orphan findings and evidence matrix.

### Immutable Phase 115 Runtime Evidence

- `.planning/phases/115-missing-transaction-round-trip-fallback-and-validation-handoff/115-CONTEXT.md` — locked compact-download behavior and scope.
- `.planning/phases/115-missing-transaction-round-trip-fallback-and-validation-handoff/115-01-SUMMARY.md` — `RCN-04` request scheduler implementation evidence.
- `.planning/phases/115-missing-transaction-round-trip-fallback-and-validation-handoff/115-02-SUMMARY.md` — `RCN-05` response handling implementation evidence.
- `.planning/phases/115-missing-transaction-round-trip-fallback-and-validation-handoff/115-03-SUMMARY.md` — `RCN-06` validation handoff implementation evidence.
- `.planning/phases/115-missing-transaction-round-trip-fallback-and-validation-handoff/115-VERIFICATION.md` — passed behavioral truths and focused command evidence.
- `packages/open-bitcoin-network/src/compact_download.rs` — request scheduling, response handling, completion, fallback, and action mapping.
- `packages/open-bitcoin-network/src/compact_download/tests.rs` — focused scheduler, response, and received-block tests.
- `packages/open-bitcoin-network/src/peer/compact_download_state.rs` — peer-state integration.
- `packages/open-bitcoin-network/src/peer/message_dispatch.rs` — compact message dispatch.
- `packages/open-bitcoin-network/src/peer/tests.rs` — peer-path completion evidence.

### Traceability And Checker Precedents

- `.planning/phases/81-milestone-audit-traceability-closure/81-CONTEXT.md` — earlier audit traceability closure precedent.
- `.planning/phases/98-traceability-reconciliation/98-CONTEXT.md` — historical evidence versus canonical closure ownership precedent.
- `.planning/phases/124-milestone-closeout-reconciliation/124-CONTEXT.md` — staged evidence-first metadata projection and audit policy.
- `scripts/check-phase98-traceability-reconciliation.ts` — focused traceability checker pattern.
- `scripts/check-phase98-traceability-reconciliation.test.ts` — temporary fixture and mutation-test pattern.
- `scripts/check-phase124-milestone-closeout-reconciliation.ts` — staged milestone projection checks.
- `scripts/check-phase124-milestone-closeout-reconciliation.test.ts` — real-corpus and synthetic stage tests.
- `scripts/check-phase117-parity-uat-release-boundary.ts` — current v2.1 requirement/evidence boundary checks.
- `scripts/check-phase117-parity-uat-release-boundary.test.ts` — v2.1 fixture and verifier-order checks.
- `scripts/verify.sh` — repo-native verification contract and checker ordering.

</canonical-refs>

<code-context>
## Existing Code Insights

### Reusable Assets

- Phase 98 already demonstrates canonical closure ownership while preserving older phases as implementation evidence.
- Phase 124 checkers already parse requirements, traceability rows, staged counts, audit state, and lifecycle provenance.
- Phase 117 already defines the v2.1 requirement surface and checks `scripts/verify.sh` ordering.
- Existing Bun checker tests use temporary repositories, focused mutation cases, stable failure messages, and real-corpus smoke tests.

### Established Patterns

- Repo-owned substantial automation is TypeScript executed by Bun.
- Checkers are deterministic and filesystem-local; public-network, wall-clock, and service-manager work stays outside default verification.
- Requirement ownership is exact-once in `.planning/REQUIREMENTS.md`, while older phases may remain cited as historical implementation evidence.
- Final phase verification records lifecycle provenance and exact commands only after the repo-native verifier passes.

### Integration Points

- Add a Phase 125 checker and paired fixture tests under `scripts/`.
- Wire both commands into the documented and executable order in `scripts/verify.sh`.
- Add lifecycle-valid `125-VERIFICATION.md` mapping the three IDs to Phase 115 evidence.
- Update active requirements, roadmap, project, state, and canonical audit through staged projection.
- Keep the existing Phase 115 runtime and verification artifacts unchanged.

</code-context>

<specifics>
## Specific Ideas

- Exact-token matching should prevent `RCN-04` in unrelated archived prose from masking a missing active verification reference.
- Summary completion should activate the guard before whole-phase completion so multi-plan phases cannot hide verification orphans.
- The Phase 125 verification matrix should use one row per requirement and distinguish implementation source, behavioral truth, focused tests, and closure owner.
- The canonical audit should say the orphan gaps were traceability defects closed by Phase 125, not runtime defects reimplemented by Phase 125.

</specifics>

<deferred>
## Deferred Ideas

- Compact receive empty-candidate bypass removal, Knots-aligned compact nonce generation, parity closure, and residual runtime hardening remain Phase 126 scope.
- Final v2.1 passed-audit projection and `/gsd-complete-milestone v2.1` archive routing remain Phase 126 scope.

</deferred>

***

*Phase: 125-compact-download-verification-traceability-closure*
*Context gathered: 2026-07-17*
