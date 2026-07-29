---
phase: 134-authoritative-cross-cache-lifecycle-integration
plan: "21"
subsystem: mempool-lifecycle
tags: [rust, atomic-commit, cross-cache, stale-revision]
requires:
  - phase: 134-authoritative-cross-cache-lifecycle-integration
    provides: "Plans 17 and 20 prepared closed projections and atomic mempool commit"
provides:
  - "Core-first atomic aggregate lifecycle commit"
  - "Obsolete validated transition compatibility removal"
  - "Eight-domain stale failure snapshot proof through production dispatcher"
affects: [phase-134-plan-22, phase-134-verification, mempool-lifecycle]
tech-stack:
  added: []
  patterns:
    - "One fallible core commit before seven infallible dependent applies"
    - "Typed complete aggregate snapshot for stale cross-cache atomicity"
key-files:
  created: []
  modified:
    - packages/open-bitcoin-mempool/src/lib.rs
    - packages/open-bitcoin-mempool/src/pool.rs
    - packages/open-bitcoin-mempool/src/pool/prepared_lifecycle.rs
    - packages/open-bitcoin-mempool/src/pool/tests/prepared_lifecycle_cases.rs
    - packages/open-bitcoin-mempool/src/pool/tests/prepared_maintenance_cases.rs
    - packages/open-bitcoin-node/src/network/lifecycle_projection/authority.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/lifecycle.rs
    - packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases.rs
    - docs/metrics/lines-of-code.md
key-decisions:
  - "Core commit is the sole fallible aggregate mutation and executes before dependent target application."
  - "The old validated transition surface was removed in the same commit that migrated the live node."
  - "Stale atomicity is proved through the production dispatcher with a complete eight-domain snapshot."
  - "MPLIFE-01 through MPLIFE-04 remain pending until phase re-verification."
patterns-established:
  - "Aggregate commit: consume the sealed prepared transition, commit core, then project and apply seven dependent targets."
  - "Stale regression: compare typed complete aggregate snapshots before and after the production dispatch failure."
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 134-2026-07-28T01-41-12
generated_at: 2026-07-29T19:18:50Z
duration: 2h 39m
completed: 2026-07-29
---

# Phase 134 Plan 21: Atomic Aggregate Lifecycle Commit Summary

**The node now commits authoritative mempool state before applying seven closed dependent projections, with production-dispatch proof that stale work leaves every aggregate domain unchanged**

## Performance

- **Duration:** 2h 39m
- **Started:** 2026-07-29T16:40:00Z
- **Completed:** 2026-07-29T19:18:50Z
- **Tasks:** 2
- **Files modified:** 9

## Accomplishments

- Replaced the node's validate-then-apply aggregate flow with one consuming `commit_sealed_lifecycle` boundary whose sole fallible mutation is the authoritative mempool commit.
- Removed `ValidatedMempoolTransition` and its public validate/apply compatibility methods after migrating all live node callers.
- Applied compact relay, serving, fanout, peer lifecycle, unbroadcast, persistence, and evidence projections only after the core commit succeeds.
- Added a typed complete aggregate snapshot covering core state plus all seven dependent domains.
- Proved through the production lifecycle dispatcher that stale work preserves the exact eight-domain snapshot and that a freshly prepared command subsequently succeeds once.

## Task Commits

Each task was committed atomically:

1. **Task 1: Integrate the atomic aggregate lifecycle commit** - `953e4361` (fix)
2. **Task 2: Prove complete stale aggregate atomicity** - `c94a30af` (test)

## Files Created/Modified

- `packages/open-bitcoin-mempool/src/lib.rs` - Removes the obsolete validated-transition export.
- `packages/open-bitcoin-mempool/src/pool.rs` - Removes obsolete compatibility methods from the public mempool surface.
- `packages/open-bitcoin-mempool/src/pool/prepared_lifecycle.rs` - Retains the consuming atomic prepared commit as the only fallible core mutation boundary.
- `packages/open-bitcoin-mempool/src/pool/tests/prepared_lifecycle_cases.rs` - Migrates prepared-transition coverage to the consuming commit API.
- `packages/open-bitcoin-mempool/src/pool/tests/prepared_maintenance_cases.rs` - Migrates maintenance coverage away from the removed compatibility API.
- `packages/open-bitcoin-node/src/network/lifecycle_projection/authority.rs` - Seals the prepared transition and commits core before applying seven dependent projections.
- `packages/open-bitcoin-node/src/network/runtime_authority/lifecycle.rs` - Propagates aggregate commit failure through the production dispatcher.
- `packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases.rs` - Adds the complete typed snapshot and stale-then-fresh production-dispatch regression.
- `docs/metrics/lines-of-code.md` - Records hook-refreshed tracked source metrics.

## Decisions Made

- The core mempool commit is the only fallible aggregate mutation and must complete before any dependent projection is constructed or applied.
- The seven dependent target applications remain infallible and execute in the mandatory compact, serving, fanout, peer lifecycle, unbroadcast, persistence, and evidence order.
- The obsolete validated-transition surface was removed atomically with live caller migration so no split validate/apply path remains available.
- Stale atomicity is measured through a typed exact snapshot of all eight domains at the production dispatcher boundary, rather than inferred from isolated cache assertions.
- MPLIFE-01 through MPLIFE-04 remain pending for formal phase re-verification; this gap plan does not claim requirement completion.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Split the aggregate boundary from the exact infallible apply target**

- **Found during:** Task 1 normal commit hook
- **Issue:** The first implementation placed the fallible core commit and `?` propagation inside the checker-designated exact apply function, violating its required infallible seven-target shape.
- **Fix:** Kept `commit_sealed_lifecycle` as the fallible core-first aggregate boundary and introduced a private prepared projection consumed by an infallible `apply_prepared_lifecycle` containing exactly the seven ordered target applications.
- **Files modified:** `packages/open-bitcoin-node/src/network/lifecycle_projection/authority.rs`
- **Committed in:** `953e4361`

**2. [Rule 3 - Blocking] Migrated legacy maintenance coverage to the consuming prepared API**

- **Found during:** Task 1 compilation
- **Issue:** A maintenance test still called the removed validate/apply compatibility surface and prevented the workspace from compiling.
- **Fix:** Reworked the caller to commit its prepared transition through the surviving consuming mempool boundary.
- **Files modified:** `packages/open-bitcoin-mempool/src/pool/tests/prepared_maintenance_cases.rs`
- **Committed in:** `953e4361`

**3. [Rule 3 - Blocking] Corrected regression-test assumptions about production result types**

- **Found during:** Task 2 focused compilation
- **Issue:** The first test draft required `LifecycleCommandResult: Debug` through `expect_err` and referenced a non-existent `admitted_order` delta field.
- **Fix:** Matched the result explicitly and asserted against the public `admitted` delta field.
- **Files modified:** `packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases.rs`
- **Committed in:** `c94a30af`

**4. [Rule 3 - Blocking] Preserved mutation-test anchor uniqueness**

- **Found during:** Task 2 normal commit hook
- **Issue:** The complete snapshot helper duplicated seven exact checker assertion strings, so Phase 134 mutation fixtures no longer removed the unique production anchor they are designed to test.
- **Fix:** Expressed equivalent test-only snapshots through direct and fully qualified access without duplicating the checker needles.
- **Files modified:** `packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases.rs`
- **Committed in:** `c94a30af`

**Total deviations:** 4 auto-fixed blocking issues.
**Impact on plan:** The fixes preserve the planned core-first boundary, strengthen compatibility removal, and keep the structural mutation suite authoritative.

## Issues Encountered

- Normal commit hooks exercised the complete repository verifier, Bazel smoke build, coverage gate, and Phase 134 mutation suite; both final task hooks completed successfully.
- Long-running macOS build processes were polled rather than interrupted, consistent with the repository's quiet-command and dynamic-loader guidance.

## Threat Model Closure

- Stale revision failure occurs before any authoritative or dependent mutation and is proven against an exact snapshot of all eight aggregate domains.
- Dependent target projections are not constructed or applied until the authoritative core commit succeeds.
- No network endpoint, authentication path, file-access boundary, dependency, or storage schema was introduced.

## Known Stubs

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- The aggregate lifecycle path has one consuming core-first commit boundary and no obsolete validate/apply compatibility route.
- Plan 22 can build on exact stale-failure atomicity across the core and all seven dependent projections.
- MPLIFE-01 through MPLIFE-04 remain pending until formal phase re-verification.

## Self-Check: PASSED

- Summary and every modified source file exist.
- Task commits `953e4361` and `c94a30af` exist in repository history.
- MPLIFE-01 through MPLIFE-04 remain pending in both the checklist and traceability table.

*Phase: 134-authoritative-cross-cache-lifecycle-integration*
*Completed: 2026-07-29*
