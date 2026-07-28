---
phase: 134-authoritative-cross-cache-lifecycle-integration
plan: "02"
subsystem: node-lifecycle-authority
tags: [rust, lifecycle-command, projection-plan, authority-epoch, mempool]

requires:
  - phase: 134-authoritative-cross-cache-lifecycle-integration
    plan: "01"
    provides: opaque prepared mempool transitions and immutable canonical lifecycle facts
provides:
  - exhaustive typed command vocabulary for lifecycle mutation and effect preparation/completion
  - checked non-interchangeable authority epochs and lifecycle generations
  - sealed authority-bound plan with mandatory core and seven concrete projection targets
  - preparation-only ManagedMempool accessors with no production caller routing
affects: [phase-134-node-projector, peer-lifecycle, relay-projection, persistence-effects]

tech-stack:
  added: []
  patterns:
    - exhaustive typed lifecycle command
    - closed authority-bound projection aggregate
    - checked facts-derived projection preflight

key-files:
  created:
    - packages/open-bitcoin-node/src/network/lifecycle_projection.rs
  modified:
    - packages/open-bitcoin-node/src/mempool.rs
    - packages/open-bitcoin-node/src/network.rs
    - docs/parity/source-breadcrumbs.json
    - docs/metrics/lines-of-code.md

key-decisions:
  - "Use one exhaustive LifecycleCommand family for lifecycle mutation, effect preparation, relay preparation, and receipt completion."
  - "Require authority epoch, core capability, and all seven concrete projection targets in one private checked constructor."
  - "Keep ManagedMempool additions preparation-only and preserve the existing Arc<Mutex<AuthoritativeNetwork>> runtime authority without routing callers."

patterns-established:
  - "Closed plan: an authority-bound lifecycle capability cannot omit compact, serving, fanout, peer, unbroadcast, persistence, or evidence projection."
  - "Facts-derived preflight: projection shape validation finishes before any authoritative mutation or effect execution."

requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 134-2026-07-28T01-41-12
generated_at: 2026-07-28T06:04:38Z

duration: 32min
completed: 2026-07-28
---

# Phase 134 Plan 02: Lifecycle Command and Closed Projection Contract Summary

**An exhaustive sealed command family and authority-bound nine-field projection plan now make lifecycle target omissions and alternate effect mutation paths structurally difficult to represent.**

## Performance

- **Duration:** 32 min
- **Started:** 2026-07-28T05:32:34Z
- **Completed:** 2026-07-28T06:04:38Z
- **Tasks:** 2
- **Files modified:** 5

## Accomplishments

- Defined typed singleton/package admission, pressure, expiry, connected-block, reorg, maintenance, snapshot/relay preparation, and peer/snapshot completion commands in one sealed vocabulary.
- Added checked, non-interchangeable `AuthorityEpoch` and `LifecycleGeneration` newtypes while keeping lifecycle facts, projection plans, owned effects, requests, and receipts as distinct concrete types.
- Closed `LifecycleProjectionPlan` over authority epoch, the opaque core transition, and mandatory compact, serving, fanout, peer, unbroadcast, persistence, and evidence targets.
- Added preparation-only `ManagedMempool` accessors for singleton, package, expiry, and connected-block transitions without routing production callers or changing the shared runtime authority.
- Passed focused lifecycle tests and the complete repository contract, including formatting, warnings-denied clippy, all-target build, all-feature tests, Bright Builds, parity breadcrumbs, Bazel smoke verification, and coverage.

## Task Commits

Each task was committed atomically:

1. **Task 1: Define exhaustive lifecycle and effect command variants** - `2a670d82`
2. **Task 2: Define the closed authority-bound projection plan** - `74b509f7`

## Files Created/Modified

- `packages/open-bitcoin-node/src/network/lifecycle_projection.rs` - Checked authority identities, exhaustive command family, private requests/receipts, closed plan, concrete target contracts, and construction tests.
- `packages/open-bitcoin-node/src/mempool.rs` - Thin preparation-only node facade accessors for core mempool transitions.
- `packages/open-bitcoin-node/src/network.rs` - Registered the sealed lifecycle projection module without exposing another dispatcher.
- `docs/parity/source-breadcrumbs.json` - Registered the lifecycle contract against the pinned Knots mempool, validation, persistence, and network-processing anchors.
- `docs/metrics/lines-of-code.md` - Refreshed by the mandatory repository hook.

## Decisions Made

- All lifecycle mutation and later effect prepare/complete work is represented by `LifecycleCommand`; no callback, heterogeneous batch, actor, mailbox, or parallel effect entrypoint was introduced.
- The seven targets use distinct concrete wrapper types over one checked lifecycle shape, minimizing duplicate validation while preserving compile-time non-interchangeability for later plans.
- The new `ManagedMempool` APIs only prepare opaque core capabilities. Validation, consumption, target mutation, I/O, and production routing remain owned by later Phase 134 plans.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - Missing Critical] Registered and breadcrumbed the new Rust module in Task 1**

- **Found during:** Task 1
- **Issue:** The task file list named only the new Rust source, but repository instructions require every first-party Rust file to be registered and breadcrumbed in the same commit.
- **Fix:** Registered `lifecycle_projection` in `network.rs` and added its pinned Knots anchor group to the parity registry.
- **Files modified:** `packages/open-bitcoin-node/src/network.rs`, `docs/parity/source-breadcrumbs.json`
- **Verification:** `bun scripts/check-parity-breadcrumbs.ts` verified all 706 Rust files and both normal commit hooks passed.
- **Committed in:** `2a670d82`

**2. [Rule 3 - Blocking] Deferred requirement activation to lifecycle verification**

- **Found during:** Plan metadata preparation
- **Issue:** Intermediate Phase 134 summaries cannot activate `MPLIFE-01` or `MPLIFE-02` before the phase has a lifecycle-valid verification artifact.
- **Fix:** Preserved both requirements as pending and left `requirements-completed` empty, matching Plan 134-01 and the active milestone traceability contract.
- **Files modified:** `.planning/phases/134-authoritative-cross-cache-lifecycle-integration/134-02-SUMMARY.md`
- **Verification:** The normal repository hook's active milestone traceability checker passed with requirements pending.
- **Committed in:** Plan metadata commit

**Total deviations:** 2 auto-fixed (1 missing critical, 1 blocking)
**Impact on plan:** Both adjustments enforce repository registration and lifecycle traceability rules; runtime scope and contract behavior are unchanged.

## Issues Encountered

- Both TDD RED runs failed on the intended missing contracts before implementation. GREEN tests then passed, and neither task required a verification retry.

## Known Stubs

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plans 134-03 through 134-05 can replace each target's checked shape contract with exact prepared operations without changing the aggregate field set.
- Later effect plans can implement snapshot/relay preparation and receipt completion through the existing command family rather than adding mutation entrypoints.
- No blockers remain.

## Self-Check: PASSED

- Summary and all key created/modified files exist.
- Task commits `2a670d82` and `74b509f7` are present in repository history.
- Changed Rust files contain no tracked stub patterns, and Markdown frontmatter uses exactly two delimiters.

***

*Phase: 134-authoritative-cross-cache-lifecycle-integration*
*Completed: 2026-07-28*
