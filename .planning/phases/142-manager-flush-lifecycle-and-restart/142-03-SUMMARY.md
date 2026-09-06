---
phase: 142-manager-flush-lifecycle-and-restart
plan: 03
subsystem: storage
tags: [interrupted-write, replay, coins-view, persist-sync, rust]

requires:
  - phase: 142-manager-flush-lifecycle-and-restart
    provides: MarkerState::Interrupted with head_blocks Ok([new, old])
  - phase: 142-manager-flush-lifecycle-and-restart
    provides: Chainstate generic CoinsView parent and cache occupancy
provides:
  - "replay_interrupted_flush apply-only ReplayBlocks owner"
  - "Bodies-present two-element H replays to B=new and empty H"
  - "Missing body or undo returns InterruptedWrite and keeps H"
  - "FjallCoinsView::batch_write_with_persist_mode for Sync marker finish"
affects:
  - 142-04
  - 142-05
  - flush-replay
  - canflush-init

tech-stack:
  added: []
  patterns:
    - "Replay lives in the node crate; flush.rs stays count-only"
    - "PersistMode::Sync finishes interrupted H; ordinary batch_write still uses Flush"

key-files:
  created:
    - packages/open-bitcoin-node/src/chainstate/replay.rs
    - packages/open-bitcoin-node/src/chainstate/replay/tests.rs
  modified:
    - packages/open-bitcoin-node/src/chainstate.rs
    - packages/open-bitcoin-node/src/storage/coins_view.rs
    - packages/open-bitcoin-chainstate/src/coins/cache.rs
    - packages/open-bitcoin-chainstate/src/coins/tests.rs
    - docs/parity/source-breadcrumbs.json
    - docs/metrics/lines-of-code.md

key-decisions:
  - "Combined 142-03 RED and GREEN into one hook-passing feat commit because pre-commit runs verify.sh"
  - "batch_write_with_persist_mode allows present H so replay can finish markers; ordinary batch_write still refuses H"
  - "CoinsCache::set_best_block and into_dirty_parent_write extract Sync writes without calling cache.flush"
  - "Leave FLUSH-02 Pending until lifecycle-valid phase verification"

patterns-established:
  - "Pattern 1: Apply-only spend/add overwrite replays interrupted H; missing body/undo fail-closes without dropping H"
  - "Pattern 2: Marker finish uses PersistMode::Sync through batch_write_with_persist_mode, not the Flush default"

requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 142-2026-09-06T16-23-52
generated_at: 2026-09-06T20:22:36Z

duration: 53min
completed: 2026-09-06
---

# Phase 142 Plan 03: Interrupted-Flush Replay Summary

**Two-element interrupted `H` now replays from stored undo and block bodies to `B=new`, or fail-closes as `InterruptedWrite` without dropping `H`.**

## Performance

- **Duration:** 53 min
- **Started:** 2026-09-06T19:28:40Z
- **Completed:** 2026-09-06T20:22:36Z
- **Tasks:** 2
- **Files modified:** 8

## Accomplishments

- `replay_interrupted_flush` consumes observable two-element `H`, walks headers, and apply-only rolls the old branch back and the new branch forward.
- Successful replay flushes the dirty overlay with `PersistMode::Sync`, clears `H`, and writes `B=new`.
- Missing block body or old-tip undo returns `StorageError::InterruptedWrite { namespace: Coins, action: Reindex }` and leaves `H` in place.
- First-flush `H=[new, [0;32]]` skips rollback. Empty `H` plus present `B` is a no-op.
- `flush.rs` still has no `ReplayBlocks` or `BlockHash`. Replay does not call `connect_block`.

## Task Commits

Each task was committed atomically:

1. **Task 1: Write failing replay success and fail-closed tests** — RED verified by construction (new replay tests). Atomic RED commit skipped because `.githooks/pre-commit` runs workspace `verify.sh`.
2. **Task 2: Implement apply-only replay and Sync marker finish** - `affbaef4` (feat)

**Plan metadata:** included in the docs(142-03) complete-plan commit

## Files Created/Modified

- `packages/open-bitcoin-node/src/chainstate/replay.rs` — `replay_interrupted_flush` apply-only rollback/rollforward and Sync finish
- `packages/open-bitcoin-node/src/chainstate/replay/tests.rs` — Bodies-present, first-flush, missing-body, missing-undo, no-op, and source guards
- `packages/open-bitcoin-node/src/chainstate.rs` — `mod replay` plus `pub use replay_interrupted_flush`
- `packages/open-bitcoin-node/src/storage/coins_view.rs` — `batch_write_with_persist_mode` allows present `H` for marker finish
- `packages/open-bitcoin-chainstate/src/coins/cache.rs` — `set_best_block` and `into_dirty_parent_write`
- `packages/open-bitcoin-chainstate/src/coins/tests.rs` — Coverage for the new cache extractors
- `docs/parity/source-breadcrumbs.json` — `replay.rs` and `replay/tests.rs` in `node-chainstate-adapter`
- `docs/metrics/lines-of-code.md` — Hook-regenerated LOC freshness

## Decisions Made

- Combined RED and GREEN into one hook-passing feat commit because pre-commit runs `verify.sh`.
- `batch_write_with_persist_mode` may write while `H` is present so replay can finish markers. Ordinary `batch_write` still refuses a present `H`.
- Replay extracts dirty writes through `CoinsCache::into_dirty_parent_write` and Sync-finishes them; it does not call `cache.flush()` (Flush persist mode).
- Leave `FLUSH-02` Pending until lifecycle-valid phase verification.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - Missing Critical] Exported `replay_interrupted_flush` so the production lib is not dead-code**
- **Found during:** Task 2 (pre-commit clippy)
- **Issue:** Private `mod replay` left the public function unused in the non-test lib, so clippy `-D dead-code` failed.
- **Fix:** `pub use replay::replay_interrupted_flush` from `chainstate.rs`.
- **Files modified:** `packages/open-bitcoin-node/src/chainstate.rs`
- **Verification:** `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib -- -D warnings`
- **Committed in:** `affbaef4` (part of combined feat commit)

**2. [Rule 2 - Missing Critical] Allowed present `H` on `batch_write_with_persist_mode`**
- **Found during:** Task 2
- **Issue:** Ordinary `batch_write` refuses a present `H`. Replay must finish that interrupted write.
- **Fix:** `batch_write_with_persist_mode` passes `allow_existing_heads = true`; `batch_write` still refuses `H`.
- **Files modified:** `packages/open-bitcoin-node/src/storage/coins_view.rs`
- **Verification:** Replay success and fail-closed tests — 7 passed
- **Committed in:** `affbaef4` (part of combined feat commit)

**3. [Rule 2 - Missing Critical] Added `CoinsCache` extractors for Sync marker finish**
- **Found during:** Task 2
- **Issue:** `CoinsCache::flush` always calls parent `batch_write` (Flush persist mode). Replay needs the dirty batch plus parent for `PersistMode::Sync`.
- **Fix:** `set_best_block` and `into_dirty_parent_write` on `CoinsCache`, with a crate-local coverage test.
- **Files modified:** `packages/open-bitcoin-chainstate/src/coins/cache.rs`, `packages/open-bitcoin-chainstate/src/coins/tests.rs`
- **Verification:** `into_dirty_parent_write_returns_parent_and_dirty_overlay` passed; verify.sh coverage accepted the new lines
- **Committed in:** `affbaef4` (part of combined feat commit)

***

**Total deviations:** 3 auto-fixed (3 missing critical)
**Impact on plan:** Required for clippy, marker finish, and coverage. No persist cutover, CanFlush, or `flush.rs` ReplayBlocks.

## Issues Encountered

Pre-commit `verify.sh` rejected the first attempts for stale breadcrumb order, rustfmt import grouping, unused private replay exports, and uncovered `CoinsCache` helper lines. Each was fixed before the feat commit landed.

## Authentication Gates

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Ready for `142-04-PLAN.md`. Interrupted flush can replay or fail closed from stored undo and bodies.
- CanFlush, persist cutover, and manager flush wiring are intentionally not implemented.
- `FLUSH-02` remains Pending until later plans and lifecycle-valid phase verification.

***
*Phase: 142-manager-flush-lifecycle-and-restart*
*Completed: 2026-09-06*

## Self-Check: PASSED
