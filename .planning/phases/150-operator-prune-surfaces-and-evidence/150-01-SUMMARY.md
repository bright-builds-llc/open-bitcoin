---
phase: 150-operator-prune-surfaces-and-evidence
plan: "01"
subsystem: chainstate
tags: [prune, getblockchaininfo, pruneblockchain, parity-breadcrumbs]

requires:
  - phase: 147-pure-prune-policy-and-lock-windows
    provides: PruneMode with Disabled, ManualOnly, and Automatic target MiB
provides:
  - I/O-free GetPruneHeight over a height-1 completeness slice
  - getblockchaininfo quartet projection from PruneMode plus that helper
  - pruneblockchain height-or-timestamp pre-step
  - operator-prune-projection parity breadcrumb group
affects:
  - 150-02 JSONC prune config
  - 150-04 status snapshot quartet
  - 150-05 pruneblockchain RPC

tech-stack:
  added: []
  patterns:
    - "Configured prune status is a pure projection; disabled clears optional keys even when a last pruned height is supplied"

key-files:
  created:
    - packages/open-bitcoin-chainstate/src/prune/status.rs
    - packages/open-bitcoin-chainstate/src/prune/tests/status.rs
  modified:
    - packages/open-bitcoin-chainstate/src/prune.rs
    - packages/open-bitcoin-chainstate/src/prune/tests.rs
    - docs/parity/source-breadcrumbs.json
    - docs/metrics/lines-of-code.md

key-decisions:
  - "Tests and implementation ship in one hook-passing feat commit because pre-commit runs verify.sh"
  - "Timestamp-window underflow returns i64::MAX so the header search fails closed"
  - "OPER-01 stays pending until lifecycle-valid Phase 150 verification"

patterns-established:
  - "maybe_pruneheight is GetPruneHeight plus one, or 0 when the helper is empty, and is absent only when mode is disabled"
  - "operator-prune-projection cites blockchain.cpp and chain.h without adding those paths to chainstate-prune"

requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 150-2026-09-28T17-08-23
generated_at: 2026-09-29T06:09:00Z

duration: 1h 17m
completed: 2026-09-29
---

# Phase 150 Plan 01: Pure Prune Status Projection Summary

**I/O-free Knots GetPruneHeight, the getblockchaininfo quartet, and the pruneblockchain timestamp pre-step in open-bitcoin-chainstate**

## Performance

- **Duration:** 1h 17m
- **Started:** 2026-09-29T04:51:10Z
- **Completed:** 2026-09-29T06:08:23Z
- **Tasks:** 1
- **Files modified:** 6

## Accomplishments

- Disabled projection is `pruned: false` and omits pruneheight, automatic pruning, and the byte target.
- Manual-only with nothing pruned reports pruneheight 0 and `automatic_pruning: false` and omits the target. Automatic 550 MiB reports `576716800` bytes.
- A hole under a complete tip returns last pruned height 11. Arguments above `1000000000` subtract 7200 seconds before the earliest header match. Height 0 stays the zero sentinel.

## Task Commits

Each task was committed atomically:

1. **Task 1: Project the quartet and GetPruneHeight** - `d0e9d510` (feat)

## Files Created/Modified

- `packages/open-bitcoin-chainstate/src/prune/status.rs` - Pure height helper, quartet projection, and timestamp pre-step
- `packages/open-bitcoin-chainstate/src/prune/tests/status.rs` - Quartet, hole, and timestamp tests
- `packages/open-bitcoin-chainstate/src/prune.rs` - Re-exports the projection API
- `packages/open-bitcoin-chainstate/src/prune/tests.rs` - Declares the status test module
- `docs/parity/source-breadcrumbs.json` - `operator-prune-projection` group citing `blockchain.cpp` and `chain.h`
- `docs/metrics/lines-of-code.md` - Hook-refreshed line count

## Decisions Made

- Tests and implementation are one commit. A failing-test-only commit cannot pass pre-commit `bash scripts/verify.sh`.
- Checked timestamp subtraction that cannot fit in `i64` returns `i64::MAX`, so the header search reports no block.
- OPER-01 stays pending. This plan is the projection only. Phase verification activates the requirement.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - Missing Critical] Fail closed when the timestamp window underflows**
- **Found during:** Task 1 (Project the quartet and GetPruneHeight)
- **Issue:** Workspace coverage rejects the unchecked failure of `checked_sub(7200)`. Every raw value above `1000000000` fits, so that arm never ran.
- **Fix:** `adjusted_prune_timestamp` uses `checked_sub` and returns `i64::MAX` on failure. The header search then finds no block. A unit test covers `i64::MIN`.
- **Files modified:** `packages/open-bitcoin-chainstate/src/prune/status.rs`, `packages/open-bitcoin-chainstate/src/prune/tests/status.rs`
- **Verification:** `cargo llvm-cov -p open-bitcoin-chainstate --lib` no longer lists `prune/status.rs`. Focused status tests pass.
- **Committed in:** `d0e9d510`

**2. [Rule 1 - Bug] Return the overflowed info height with `?`**
- **Found during:** Task 1 commit hook
- **Issue:** A `let...else` around `checked_add(1)` failed `clippy::question_mark`. Returning the `Option` directly left an uncovered region.
- **Fix:** `info_pruneheight` uses `checked_add(1)?`, which is `None` only when `last + 1` overflows `u32`. The `u32::MAX` test hits that arm.
- **Files modified:** `packages/open-bitcoin-chainstate/src/prune/status.rs`
- **Verification:** Package clippy with `-D warnings` passed, and the status coverage report no longer lists this file.
- **Committed in:** `d0e9d510`

---

**Total deviations:** 2 auto-fixed (1 missing critical, 1 bug)
**Impact on plan:** Both stay inside the planned checked arithmetic. Timestamp arguments above the threshold still subtract 7200 seconds. Height overflow still omits pruneheight.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Ready for plan 150-02. The chainstate projection is public from `prune` and does not read Fjall, change RPC, or delete payloads.

## Self-Check: PASSED

- FOUND: packages/open-bitcoin-chainstate/src/prune/status.rs
- FOUND: packages/open-bitcoin-chainstate/src/prune/tests/status.rs
- FOUND: d0e9d510

---
*Phase: 150-operator-prune-surfaces-and-evidence*
*Completed: 2026-09-29*
