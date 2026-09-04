---
phase: 141-durable-fjall-coins-adapter
plan: 03
subsystem: storage
tags: [chainstate-store, undo-codec, memory-coins-view, leftover-snapshot, rust]

requires:
  - phase: 141-durable-fjall-coins-adapter
    provides: Fallible CoinsView heads and MemoryCoinsView parent used by the in-memory store
provides:
  - "ChainstateStore view-backed surface: get_coin, have_coin, batch_write, best_block, head_blocks, load_undo, save_undo"
  - "MemoryChainstateStore coins and undo maps that stay independent of leftover snapshot writes after hydrate"
  - "Standalone encode_block_undo / decode_block_undo that omit leftover utxos"
affects:
  - 141-04
  - persist-cutover
  - fjall-undo-records

tech-stack:
  added: []
  patterns:
    - "Leftover save_snapshot only mutates maybe_snapshot; coins and undo live on store maps"
    - "BlockUndoDto encodes as its own versioned JSON record, not ChainstateSnapshotDto"

key-files:
  created: []
  modified:
    - packages/open-bitcoin-node/src/chainstate.rs
    - packages/open-bitcoin-node/src/chainstate/tests.rs
    - packages/open-bitcoin-node/src/storage/snapshot_codec.rs
    - packages/open-bitcoin-node/src/storage/snapshot_codec/tests.rs
    - packages/open-bitcoin-chainstate/src/coins/memory.rs
    - docs/metrics/lines-of-code.md

key-decisions:
  - "Combined 141-03 RED and GREEN into one hook-passing feat commit because pre-commit runs verify.sh"
  - "MemoryCoinsView gained Debug/Clone/Default/PartialEq/Eq so MemoryChainstateStore can keep those derives"
  - "encode_block_undo and decode_block_undo allow(dead_code) until Plan 04 writes undo: records"
  - "SchemaVersion::CURRENT stays 1; COIN-01 remains Pending until phase verification"

patterns-established:
  - "Pattern 1: from_snapshot hydrates MemoryCoinsView and undo_by_block once; later leftover snapshots do not clear view truth"
  - "Pattern 2: Undo codec reuses BlockUndoDto + encode_versioned/decode_versioned and fails closed as Corruption"

requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 141-2026-09-04T17-37-21
generated_at: 2026-09-04T21:23:47Z

duration: 37min
completed: 2026-09-04
---

# Phase 141 Plan 03: Undo Codec Extract and Memory View-Backed Store Summary

**View-backed MemoryChainstateStore plus standalone BlockUndo encode/decode, with leftover persist still writing snapshot blobs**

## Performance

- **Duration:** 37 min
- **Started:** 2026-09-04T20:46:27Z
- **Completed:** 2026-09-04T21:23:47Z
- **Tasks:** 2
- **Files modified:** 6

## Accomplishments

- `ChainstateStore` now exposes `get_coin`, `have_coin`, `batch_write`, `best_block`, `head_blocks`, `load_undo`, and `save_undo` while leftover `load_snapshot` / `save_snapshot` remain.
- `MemoryChainstateStore` hydrates `MemoryCoinsView` and `undo_by_block` from a snapshot once. A later empty leftover snapshot does not clear coins or undo.
- `encode_block_undo` / `decode_block_undo` round-trip `BlockUndoDto` JSON that contains `transactions` and omits `"utxos"`.
- `ManagedChainstate::persist` still calls `save_snapshot(self.chainstate.snapshot())`.

## Task Commits

Each task was committed atomically:

1. **Task 1: Write failing MemoryChainstateStore surface and undo-codec tests** — RED verified locally by writing the locked test names first; atomic RED commit skipped because `.githooks/pre-commit` runs workspace `verify.sh`.
2. **Task 2: Implement MemoryChainstateStore surface and undo encode/decode** - `da33cc34` (feat)

**Plan metadata:** pending `docs(141-03): complete undo codec and memory view-backed store plan`

## Files Created/Modified

- `packages/open-bitcoin-node/src/chainstate.rs` — Grown `ChainstateStore` + `MemoryChainstateStore` coins/undo maps; `persist` unchanged
- `packages/open-bitcoin-node/src/chainstate/tests.rs` — View-vs-leftover, `batch_write`, and `save_undo` tests
- `packages/open-bitcoin-node/src/storage/snapshot_codec.rs` — `encode_block_undo` / `decode_block_undo`
- `packages/open-bitcoin-node/src/storage/snapshot_codec/tests.rs` — Undo JSON omits `"utxos"`
- `packages/open-bitcoin-chainstate/src/coins/memory.rs` — `Debug` / `Clone` / `Default` / `PartialEq` / `Eq` on `MemoryCoinsView`
- `docs/metrics/lines-of-code.md` — Hook-regenerated LOC freshness

## Decisions Made

- Combined RED and GREEN into one hook-passing feat commit because pre-commit runs `verify.sh`.
- Derive `MemoryCoinsView` so `MemoryChainstateStore` can keep `Clone` / `Default` / `PartialEq` without reconstructing maps by hand.
- Leave `encode_block_undo` / `decode_block_undo` `allow(dead_code)` until Plan 04 writes `undo:` Fjall keys.
- Leave `COIN-01` Pending; this plan does not bump schema or migrate leftover snapshots.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Combined RED and GREEN because pre-commit runs verify.sh**
- **Found during:** Task 1
- **Issue:** A RED-only commit cannot pass `.githooks/pre-commit`, which runs full `bash scripts/verify.sh`.
- **Fix:** Implemented GREEN in the same working tree, then made one feat commit after `chainstate::tests` (9 passed) and `encode_block_undo` (passed).
- **Files modified:** `packages/open-bitcoin-node/src/chainstate.rs`, `packages/open-bitcoin-node/src/chainstate/tests.rs`, `packages/open-bitcoin-node/src/storage/snapshot_codec.rs`, `packages/open-bitcoin-node/src/storage/snapshot_codec/tests.rs`, `packages/open-bitcoin-chainstate/src/coins/memory.rs`
- **Verification:** `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib chainstate::tests` — 9 passed; undo codec test passed; `verify.sh` completed in 17m 56s.
- **Committed in:** `da33cc34`

**2. [Rule 3 - Blocking] Derived MemoryCoinsView so MemoryChainstateStore stays Clone/Default/Eq**
- **Found during:** Task 2
- **Issue:** `MemoryCoinsView` had no derives; adding it as a field would drop `Clone` used by `ManagedChainstate` and many network tests.
- **Fix:** Added `#[derive(Debug, Clone, Default, PartialEq, Eq)]` on `MemoryCoinsView`.
- **Files modified:** `packages/open-bitcoin-chainstate/src/coins/memory.rs`
- **Verification:** Existing `MemoryChainstateStore::default()` call sites compile; store tests pass.
- **Committed in:** `da33cc34`

**3. [Rule 3 - Blocking] Allowed dead_code on unused undo codec until Plan 04**
- **Found during:** Task 2 commit (`verify.sh` `-D dead-code`)
- **Issue:** `pub(crate) encode_block_undo` / `decode_block_undo` are only called from tests until Fjall `undo:` writes land.
- **Fix:** `#[allow(dead_code)]` with Plan 04 comments, matching `coins_access.rs`.
- **Files modified:** `packages/open-bitcoin-node/src/storage/snapshot_codec.rs`
- **Verification:** `cargo check -p open-bitcoin-node --lib` and `verify.sh` completed.
- **Committed in:** `da33cc34`

---

**Total deviations:** 3 auto-fixed (3 blocking)
**Impact on plan:** All three were required to land the planned contract under repo hooks and existing `Clone`/`Default` store call sites. No schema bump, no Fjall `undo:` keys, no persist write-site cutover.

## Issues Encountered

- First commit attempt failed `verify.sh` on unused undo codec (`-D dead-code`). Fixed with `allow(dead_code)` and recommitted.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan 04 can persist `undo:` records through `encode_block_undo` / `decode_block_undo` and implement Fjall migrate.
- `SchemaVersion::CURRENT` remains `1`. `ManagedChainstate::persist` still writes leftover snapshots.
- Do not implement `ReplayBlocks` or cut over `persist_progress` in this phase.

## Self-Check: PASSED
