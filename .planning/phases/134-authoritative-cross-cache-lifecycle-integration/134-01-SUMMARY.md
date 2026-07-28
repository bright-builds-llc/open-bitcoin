---
phase: 134-authoritative-cross-cache-lifecycle-integration
plan: "01"
subsystem: mempool
tags: [rust, mempool, lifecycle, revision-capability, package-admission]

requires:
  - phase: 130-resource-time-fee-primitives
    provides: typed lifecycle deltas, canonical identities, and revisioned mempool state
  - phase: 132-typed-package-staged-admission
    provides: staged singleton/package admission and immutable package reports
provides:
  - opaque non-Clone prepared and validated mempool transition capabilities
  - immutable canonical lifecycle facts with bodies, membership, and graph order
  - prepare-first compatibility facades for admission, pressure, expiry, and block lifecycle
  - sequence-sensitive reorg preparation proof
affects: [phase-134-node-projector, cross-cache-lifecycle, relay-projection, durable-mempool]

tech-stack:
  added: []
  patterns:
    - sealed prepare-validate-consume capability
    - immutable transition facts before authoritative mutation
    - transaction-graph-derived descendant-first teardown

key-files:
  created:
    - packages/open-bitcoin-mempool/src/pool/prepared_lifecycle.rs
    - packages/open-bitcoin-mempool/src/pool/tests/prepared_maintenance_cases.rs
  modified:
    - packages/open-bitcoin-mempool/src/pool/admission.rs
    - packages/open-bitcoin-mempool/src/pool/package_admission.rs
    - packages/open-bitcoin-mempool/src/pool/expiry.rs
    - packages/open-bitcoin-mempool/src/pool/lifecycle.rs
    - packages/open-bitcoin-mempool/src/pool/tests/prepared_lifecycle_cases.rs
    - docs/parity/source-breadcrumbs.json

key-decisions:
  - "Keep MempoolPatch private behind one opaque non-Clone capability and separate revision validation from infallible consumption."
  - "Derive teardown order from canonical removed transaction inputs so descendants always precede ancestors independently of removal cause or role."
  - "Retain existing mutating APIs as compatibility facades that prepare, validate, and consume exactly once."

patterns-established:
  - "Prepare-first transition: all fallible policy, script, identity, and patch checks finish before mutation."
  - "Sequential lifecycle step: consume transition N before preparing transition N+1 against the new revision."

requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 134-2026-07-28T01-41-12
generated_at: 2026-07-28T05:22:16Z

duration: 1h 10m
completed: 2026-07-28
---

# Phase 134 Plan 01: Prepared Mempool Lifecycle Summary

**Sealed revision-bound capabilities now preflight admission, pressure, expiry, block cleanup, and sequential reorg steps while exposing canonical lifecycle facts before mutation.**

## Performance

- **Duration:** 1h 10m
- **Started:** 2026-07-28T04:11:55Z
- **Completed:** 2026-07-28T05:22:16Z
- **Tasks:** 3
- **Files modified:** 13

## Accomplishments

- Added opaque, non-`Clone` prepared and validated transition capabilities whose immutable facts include canonical identities, transaction bodies, final membership, admission order, and teardown order without exposing `MempoolPatch`.
- Routed singleton admission, package admission, expiry, and connected-block compatibility APIs through prepare → revision validation → consuming apply, including no-op and stale-revision truth.
- Proved partial package survivors remain parent-first, teardown is graph-derived descendant-first across replacement/pressure/expiry/block conflict, and reorg reconsideration cannot prepare before the preceding removal is consumed.
- Passed the full repository verification contract, including formatting, warnings-denied clippy, all-target build, all-feature tests, Bright Builds checks, Bazel smoke verification, and zero uncovered pure-core lines.

## Task Commits

Each task was committed atomically:

1. **Task 1: Implement the sealed prepared transition capability** - `db1f4cc7`
2. **Task 2: Route singleton and package facades through capabilities** - `4ac9656b`
3. **Task 3: Extend preparation to maintenance and reorg steps** - `92ef7d22`

## Files Created/Modified

- `packages/open-bitcoin-mempool/src/pool/prepared_lifecycle.rs` - Opaque transition capabilities, immutable preflighted facts, revision validation, and graph ordering.
- `packages/open-bitcoin-mempool/src/pool/admission.rs` - Singleton compatibility facade using one prepared capability.
- `packages/open-bitcoin-mempool/src/pool/package_admission.rs` - Package compatibility facade using one prepared capability.
- `packages/open-bitcoin-mempool/src/pool/expiry.rs` - Pure expiry preparation plus consuming compatibility facade.
- `packages/open-bitcoin-mempool/src/pool/lifecycle.rs` - Pure connected-block preparation plus consuming compatibility facade.
- `packages/open-bitcoin-mempool/src/pool/tests/prepared_lifecycle_cases.rs` - Admission, package, revision, partial-survivor, and replacement ordering proofs.
- `packages/open-bitcoin-mempool/src/pool/tests/prepared_maintenance_cases.rs` - Expiry, pressure, block, and sequential reorg preparation proofs.
- `docs/parity/source-breadcrumbs.json` - Registered the prepared lifecycle surface against pinned Knots validation and mempool anchors.

## Decisions Made

- `MempoolPatch` remains crate-private and boxed inside the prepared core transition, keeping the public capability small and preventing projection code from bypassing validation.
- Final-present ordering follows committed admission order; teardown ordering is independently derived from removed transaction dependencies, so semantic cause/role sorting cannot accidentally dictate destruction order.
- Existing public mutation methods remain compatible but contain no parallel implementation path: each extracts its facade result, validates the exact base revision, and consumes once.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Boxed the prepared patch variant**

- **Found during:** Task 1
- **Issue:** Warnings-denied clippy rejected the capability enum because the patch variant made it excessively large.
- **Fix:** Boxed the private `MempoolPatch` payload without changing capability ownership or public facts.
- **Files modified:** `packages/open-bitcoin-mempool/src/pool/prepared_lifecycle.rs`
- **Verification:** Task 1 clippy, build, tests, and full repository hook passed.
- **Committed in:** `db1f4cc7`

**2. [Rule 3 - Blocking] Split maintenance tests at the repository file-length limit**

- **Found during:** Task 3
- **Issue:** Bright Builds rejected the combined prepared lifecycle test file at 744 lines against the 628-line limit.
- **Fix:** Moved maintenance/reorg cases into a focused sibling module while preserving the plan's test filter.
- **Files modified:** `packages/open-bitcoin-mempool/src/pool/tests.rs`, `packages/open-bitcoin-mempool/src/pool/tests/prepared_maintenance_cases.rs`
- **Verification:** Bright Builds reported zero file-length findings and the focused filter ran all 13 cases.
- **Committed in:** `92ef7d22`

**3. [Rule 3 - Blocking] Synchronized parity breadcrumbs after adding the mempool anchor**

- **Found during:** Task 3 commit verification
- **Issue:** The parity registry included `txmempool.cpp`, but two existing source breadcrumb blocks still listed only `validation.cpp`.
- **Fix:** Updated both source blocks to match the registered Knots anchors.
- **Files modified:** `packages/open-bitcoin-mempool/src/pool/prepared_lifecycle.rs`, `packages/open-bitcoin-mempool/src/pool/tests/prepared_lifecycle_cases.rs`
- **Verification:** `bun scripts/check-parity-breadcrumbs.ts` verified all 705 Rust files.
- **Committed in:** `92ef7d22`

**4. [Rule 3 - Blocking] Removed unreachable teardown error branches**

- **Found during:** Task 3 commit coverage gate
- **Issue:** The initial DFS carried missing-entry and cycle errors that cannot occur for validated Bitcoin transaction dependencies, leaving uncovered pure-core lines.
- **Fix:** Derived edges directly from validated removed transaction bodies and retained deterministic visited-set traversal.
- **Files modified:** `packages/open-bitcoin-mempool/src/pool/prepared_lifecycle.rs`
- **Verification:** Focused package coverage and the full repository hook both reported zero uncovered lines.
- **Committed in:** `92ef7d22`

**5. [Rule 3 - Blocking] Deferred requirement activation to lifecycle verification**

- **Found during:** Plan metadata commit
- **Issue:** Marking `MPLIFE-02` and `MPLIFE-03` complete in Plan 01 activated them before Phase 134 has a lifecycle-valid verification artifact, which the active-milestone traceability guard rejects.
- **Fix:** Kept both requirements pending and left this intermediate summary's `requirements-completed` list empty, matching established multi-plan phase practice.
- **Files modified:** `.planning/REQUIREMENTS.md`, `.planning/phases/134-authoritative-cross-cache-lifecycle-integration/134-01-SUMMARY.md`
- **Verification:** Active milestone traceability checker passes; the final Phase 134 verification plan remains responsible for requirement activation.
- **Committed in:** Plan metadata commit

**Total deviations:** 5 auto-fixed (5 blocking)
**Impact on plan:** All fixes enforced repository quality gates or simplified unreachable logic; capability behavior and plan scope were unchanged.

## Issues Encountered

- The Task 3 normal commit hook required two retries: first for synchronized breadcrumb metadata, then for unreachable coverage branches. Both root causes were corrected and the final unmodified hook completed successfully.

## Known Stubs

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- The mempool core now exposes the sealed preflight boundary required for authoritative node-side dependent cache projection.
- Later Phase 134 plans can prepare relay, serving, persistence, orphan, and observability projections exclusively from `PreparedLifecycleFacts`, then consume the validated transition once.
- No blockers remain.

## Self-Check: PASSED

- Summary and key created files exist.
- Task commits `db1f4cc7`, `4ac9656b`, and `92ef7d22` are present in repository history.

***

*Phase: 134-authoritative-cross-cache-lifecycle-integration*
*Completed: 2026-07-28*
