---
phase: 130-resource-time-and-fee-primitives
plan: "03"
subsystem: mempool
tags: [rust, mempool, metadata, explicit-contexts, privacy]
requires:
  - phase: 130-resource-time-and-fee-primitives
    provides: Typed resource and fee-role primitives from Plans 130-01 and 130-02
provides:
  - Canonical typed acceptance time, origin, and relay-intent metadata on every mempool entry
  - Explicit admission, pressure, block, and reorg operation contexts
  - Fail-closed legacy admission adapters with named production migration and removal owners
  - Deterministic metadata preservation and retry-eligibility regressions
affects: [phase-131, phase-134, phase-135, phase-136, mempool-policy, node-admission]
tech-stack:
  added: []
  patterns:
    - Effectful adapters supply immutable policy facts to the pure mempool core
    - Legacy compatibility facts use explicit unknown variants and fail closed
key-files:
  created:
    - packages/open-bitcoin-mempool/src/context.rs
    - packages/open-bitcoin-mempool/src/pool/admission.rs
    - packages/open-bitcoin-mempool/src/pool/tests/context_cases.rs
  modified:
    - packages/open-bitcoin-mempool/src/types.rs
    - packages/open-bitcoin-mempool/src/pool.rs
    - packages/open-bitcoin-mempool/src/pool/admission_outcome.rs
    - packages/open-bitcoin-node/src/mempool.rs
    - packages/open-bitcoin-node/src/storage/mempool_snapshot.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - "Represent missing legacy metadata only as LegacyUnknown, RecoveryUnknown, and NotRequested; never infer local origin or current time."
  - "Require local origin, requested relay intent, and current authoritative membership together for retry eligibility."
  - "Keep no-context admission only as a deprecated fail-closed adapter owned for production migration by Plan 130-05 and removal by Plan 130-11."
patterns-established:
  - "Canonical entry metadata is copied only during a successful first admission and remains immutable on duplicate attempts."
  - "Operation-specific contexts require all relevant time, occupancy, capacity, or height facts without ambient clock or randomness access."
requirements-completed: []
requirements-addressed: [FEEP-03, FEEP-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 130-2026-07-23T14-26-46
generated_at: 2026-07-23T20:05:53Z
duration: 35 min
completed: 2026-07-23
---

# Phase 130 Plan 03: Canonical Entry Metadata and Explicit Contexts Summary

**Typed acceptance time, origin, and relay intent now persist on canonical entries through explicit effect-free admission, pressure, block, and reorg contracts**

## Performance

- **Duration:** 35 min
- **Started:** 2026-07-23T19:30:58Z
- **Completed:** 2026-07-23T20:05:53Z
- **Tasks:** 1
- **Files modified:** 17

## Accomplishments

- Added closed time, origin, relay-intent, metadata, and operation-context types without default-current-time, wall-clock, or randomness paths.
- Preserved exact local, peer, reorg, and known-recovery metadata while making legacy recovery and retry eligibility fail closed.
- Added context-aware result and outcome admission APIs; existing no-context APIs now document Plans 130-05 and 130-11 as migration/removal owners.
- Registered ten discoverable metadata/context regressions and complete Knots entry/persistence breadcrumbs.
- Migrated every direct constructor and workspace caller or retained it behind the documented fail-closed compatibility boundary; the timed all-target workspace compile gate passed.

## Task Commits

1. **Task 1: Add canonical metadata and narrow mempool contexts** - `ca90316e` (feat)

The TDD RED run failed on the intentionally absent context types, metadata field, and context-aware APIs. The completed GREEN implementation and regression suite were committed atomically after all repository hooks passed.

## Files Created/Modified

- `packages/open-bitcoin-mempool/src/context.rs` - Canonical metadata, retry eligibility, and admission/pressure/block/reorg contexts.
- `packages/open-bitcoin-mempool/src/pool/admission.rs` - Context-aware admission plus deprecated fail-closed migration adapters.
- `packages/open-bitcoin-mempool/src/pool/tests/context_cases.rs` - Metadata preservation, duplicate immutability, compatibility, and retry regressions.
- `packages/open-bitcoin-mempool/src/types.rs` - Canonical `MempoolEntry::metadata` field and constructor contract.
- `packages/open-bitcoin-mempool/src/pool.rs` - Admission module registration and retained policy/state helpers.
- `packages/open-bitcoin-mempool/src/pool/admission_outcome.rs` - Context propagation through stable admission outcomes.
- `packages/open-bitcoin-bench/src/cases/mempool.rs` - Explicit fail-closed deterministic benchmark context.
- `packages/open-bitcoin-mempool/tests/parity.rs` - Explicit context at the integration-test boundary.
- `packages/open-bitcoin-node/src/mempool.rs` - Documented temporary production compatibility boundary.
- `packages/open-bitcoin-node/src/network/compact_receive_candidates.rs` - Explicit context in compact-candidate test setup.
- `packages/open-bitcoin-node/src/storage/mempool_snapshot.rs` - Explicit legacy-unknown recovery classification pending Plan 130-08.
- `docs/parity/source-breadcrumbs.json` - Entry-context production and test registrations.
- `docs/metrics/lines-of-code.md` - Hook-regenerated tracked LOC freshness.

## Decisions Made

- Missing legacy metadata remains explicit and non-retryable rather than being upgraded to a guessed local origin or acceptance time.
- Retry eligibility is one closed conjunction: local origin, relay requested, and current authoritative membership.
- Context-aware APIs are the canonical path; compatibility APIs inject only `AdmissionContext::legacy_unknown()` and name their later owners.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Extracted admission methods to preserve the production file-size contract**
- **Found during:** Task 1 pre-commit verification
- **Issue:** Adding the required context-aware and compatibility methods pushed `pool.rs` to 677 lines, above the repository's enforced 628-line production limit.
- **Fix:** Moved the coherent admission API implementation into `pool/admission.rs`, kept `pool.rs` as the state/policy root, and registered the new file in parity breadcrumbs.
- **Files modified:** `packages/open-bitcoin-mempool/src/pool.rs`, `packages/open-bitcoin-mempool/src/pool/admission.rs`, `docs/parity/source-breadcrumbs.json`
- **Verification:** The normal pre-commit verifier passed the file-size, parity, clippy, test, coverage, and Bazel gates.
- **Committed in:** `ca90316e`

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** The extraction preserves the exact planned behavior and public API while satisfying a hard repository structure rule; no later lifecycle or pressure behavior was pulled forward.

## Issues Encountered

- The first verified commit attempt exposed the `pool.rs` file-size violation and was corrected by the admission-module extraction.
- The next verified attempt exposed uncovered compatibility adapters and the timestamp accessor. Focused fail-closed adapter regressions and an accessor assertion closed those gaps; the final normal commit passed the complete hook.
- The metadata hook rejected early FEEP-03/FEEP-04 completion because Phase 130 has no lifecycle-valid `VERIFICATION.md` yet. Both requirements remain addressed but pending until phase verification.

## Authentication Gates

None.

## Known Stubs

None.

## Threat Flags

None - the change adds typed pure-domain contracts and no new network endpoint, authentication path, file-access pattern, or persistence schema.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan 130-04 can build deterministic lifecycle deltas on canonical metadata and explicit operation inputs.
- Plans 130-05, 130-08, and 130-11 have named seams for production context migration, durable metadata recovery, and compatibility-adapter removal.
- No blockers remain.

## Self-Check: PASSED

- Created source, test, and summary files exist.
- Task commit `ca90316e` exists.
- Lifecycle mode, lifecycle ID, addressed requirement IDs, verification claims, and changed-file metrics match the committed work.

---
*Phase: 130-resource-time-and-fee-primitives*
*Completed: 2026-07-23*
