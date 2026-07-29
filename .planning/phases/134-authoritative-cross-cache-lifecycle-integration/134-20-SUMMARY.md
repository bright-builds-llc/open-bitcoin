---
phase: 134-authoritative-cross-cache-lifecycle-integration
plan: "20"
subsystem: mempool
tags: [rust, prepared-lifecycle, atomic-commit, revision-guard]
requires:
  - phase: 134-authoritative-cross-cache-lifecycle-integration
    provides: "Plan 01 prepared mempool transitions and revision capabilities"
provides:
  - "Atomic consuming commit boundary for prepared mempool transitions"
  - "Stale-revision rejection before authoritative mempool mutation"
  - "Mempool-owned singleton, package, expiry, and block lifecycle facades routed through atomic commit"
affects: [phase-134-plan-21, phase-134-verification, mempool-lifecycle]
tech-stack:
  added: []
  patterns:
    - "Validate revision and apply under one mutable consuming call"
    - "Project facade results before consuming the prepared capability"
key-files:
  created: []
  modified:
    - packages/open-bitcoin-mempool/src/pool/prepared_lifecycle.rs
    - packages/open-bitcoin-mempool/src/pool/tests/prepared_lifecycle_cases.rs
    - packages/open-bitcoin-mempool/src/pool/admission.rs
    - packages/open-bitcoin-mempool/src/pool/package_admission.rs
    - packages/open-bitcoin-mempool/src/pool/expiry.rs
    - packages/open-bitcoin-mempool/src/pool/lifecycle.rs
key-decisions:
  - "Revision validation and patch application execute inside one mutable consuming boundary."
  - "Legacy validated transition APIs remain available and covered through Plan 20 for Plan 21 node migration."
  - "MPLIFE-01 through MPLIFE-04 remain pending until phase re-verification."
patterns-established:
  - "Atomic prepared commit: stale returns before mutation; current and no-op capabilities return their exact lifecycle delta."
  - "Facade migration: capture read-only result projection, then consume the prepared transition exactly once."
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 134-2026-07-28T01-41-12
generated_at: 2026-07-29T16:37:00Z
duration: 2h 35m
completed: 2026-07-29
---

# Phase 134 Plan 20: Atomic Mempool Prepared Commit Summary

**Prepared mempool transitions now reject stale revisions and apply current state under one consuming mutable commit boundary used by every mempool-owned facade**

## Performance

- **Duration:** 2h 35m
- **Started:** 2026-07-29T14:01:46Z
- **Completed:** 2026-07-29T16:37:00Z
- **Tasks:** 2
- **Files modified:** 7

## Accomplishments

- Added `Mempool::commit_prepared_mempool_transition`, which consumes the prepared capability, rejects a stale base revision before mutation, and applies a current patch or no-op under the same mutable call.
- Proved exact current deltas, no-op revision stability, stale-state failure atomicity, and affine replay rejection.
- Retained and covered the legacy validated transition API so Plan 21 can migrate node-owned callers independently.
- Routed singleton admission, package submission, expiry, and connected-block lifecycle facades through the atomic commit boundary.
- Proved the migrated facades contain no internal validate/apply pair and preserved their existing result projections.

## Task Commits

Each task was committed atomically:

1. **Task 1: Add atomic prepared mempool commit** - `3462614d` (fix)
2. **Task 2: Route mempool-owned facades through atomic commit** - `004ad12f` (fix)

## Files Created/Modified

- `packages/open-bitcoin-mempool/src/pool/prepared_lifecycle.rs` - Adds the consuming atomic commit boundary while retaining Plan 21 compatibility APIs.
- `packages/open-bitcoin-mempool/src/pool/tests/prepared_lifecycle_cases.rs` - Covers current, stale, no-op, replay, and retained legacy transition paths.
- `packages/open-bitcoin-mempool/src/pool/admission.rs` - Commits singleton admission through the atomic boundary.
- `packages/open-bitcoin-mempool/src/pool/package_admission.rs` - Commits package submission through the atomic boundary.
- `packages/open-bitcoin-mempool/src/pool/expiry.rs` - Commits prepared expiry through the atomic boundary.
- `packages/open-bitcoin-mempool/src/pool/lifecycle.rs` - Commits connected-block lifecycle work through the atomic boundary.
- `docs/metrics/lines-of-code.md` - Hook-refreshed tracked source metrics.

## Decisions Made

- The atomic operation owns both the revision comparison and authoritative mutation; stale transitions return without invoking patch application.
- Facades extract immutable result/report projections before consuming the prepared transition, preserving their public return contracts.
- The legacy validation/apply surface remains public and fully covered in Plan 20 because node-owned migration and removal belong to Plan 21.
- MPLIFE-01 through MPLIFE-04 remain pending for the phase verifier; this gap plan does not claim requirement completion.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Consolidated the affine replay proof to satisfy the production file-length boundary**

- **Found during:** Task 1 final Bright Builds gate
- **Issue:** The first correct implementation made `prepared_lifecycle.rs` 638 lines, exceeding the repository limit of 628.
- **Fix:** Merged the new replay compile-fail proof into the existing prepared-capability doctest, reducing the production file to 627 lines without weakening the ownership check.
- **Files modified:** `packages/open-bitcoin-mempool/src/pool/prepared_lifecycle.rs`
- **Committed in:** `3462614d`

**2. [Rule 2 - Missing Critical Functionality] Restored coverage for retained compatibility branches**

- **Found during:** Task 1 and Task 2 normal commit hooks
- **Issue:** Migrating behavior tests and facade call sites left the retained legacy stale-validation and no-op apply branches uncovered, causing the repository coverage gate to fail.
- **Fix:** Added focused regressions for stale legacy validation and exact no-op legacy application without changing production behavior.
- **Files modified:** `packages/open-bitcoin-mempool/src/pool/tests/prepared_lifecycle_cases.rs`
- **Committed in:** `3462614d`, `004ad12f`

**Total deviations:** 2 auto-fixed (1 blocking, 1 missing critical verification)
**Impact on plan:** Both fixes preserve the planned atomic boundary and keep the explicitly retained Plan 21 compatibility API verifiable.

## Issues Encountered

- Both TDD RED stages were executed and observed, but separate failing-test commits were omitted because repository instructions require formatting, warnings-denied Clippy, all-target build, and all-feature tests to pass before every Rust commit.
- The Task 1 normal hook initially failed only at coverage for the retained stale validator; the added focused regression passed and the retried hook completed successfully.
- The Task 2 normal hook initially failed only at coverage for the retained no-op apply branch; the added focused regression passed and the retried hook completed successfully.
- Multiple Cargo and hook binaries paused at macOS `_dyld_start`. Samples showed the affected processes entirely at the dynamic-loader boundary, including the focused parity harness and ordered `operator_flows`; runs were preserved and all suites completed.

## Threat Model Closure

- Stale prepared transitions cannot mutate membership, rolling-fee state, resource totals, or revision state because the revision check precedes application inside the mutable boundary.
- Prepared capabilities remain affine and consuming; the compile-fail proof prevents replay at compile time.
- No network endpoint, authentication path, file-access boundary, dependency, or storage schema was introduced.

## Known Stubs

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- All mempool-owned facades now use the atomic prepared commit boundary.
- Plan 21 can migrate node-owned validate/apply callers and remove the retained compatibility API.
- MPLIFE-01 through MPLIFE-04 remain pending until formal phase re-verification.

## Self-Check: PASSED

- Summary and every modified source file exist.
- Task commits `3462614d` and `004ad12f` exist in repository history.
- MPLIFE-01 through MPLIFE-04 remain pending in both the checklist and traceability table.

*Phase: 134-authoritative-cross-cache-lifecycle-integration*
*Completed: 2026-07-29*
