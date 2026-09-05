---
phase: 141-durable-fjall-coins-adapter
plan: 02
subsystem: storage
tags: [fjall-coins-view, batch-write, head-blocks, fail-closed, rust]

requires:
  - phase: 141-durable-fjall-coins-adapter
    provides: Fallible CoinsView heads, CoinsStorage, C/B/H codec, and opened coins keyspace
provides:
  - "FjallCoinsView implements CoinsView with Knots two-phase H/B BatchWrite"
  - "First-party estimated_encoded_bytes 64 MiB dirty-batch splits (Buffered partials, Flush-final B)"
  - "Fail-closed Fjall get/contains_key/decode mapping to CoinsStorage"
  - "Two-element H plus missing B is InterruptedWrite; other H counts are Corruption"
affects:
  - 141-03
  - 141-04
  - persist-cutover
  - manager-flush

tech-stack:
  added: []
  patterns:
    - "Application consistency is H then coins then B, not a single Fjall batch"
    - "Bound dirty writes with estimated_encoded_bytes; never Fjall WriteBatch::len() as size"

key-files:
  created:
    - packages/open-bitcoin-node/src/storage/coins_view.rs
    - packages/open-bitcoin-node/src/storage/coins_view/tests.rs
  modified:
    - packages/open-bitcoin-node/src/storage.rs
    - docs/parity/source-breadcrumbs.json
    - docs/metrics/lines-of-code.md

key-decisions:
  - "Combined 141-02 RED and GREEN into one hook-passing feat commit because pre-commit runs verify.sh"
  - "Split after adding an item when accumulated encoded bytes exceed the cap (Knots SizeEstimate-after-write)"
  - "Fjall WriteBatch is named OwnedWriteBatch at the crate root; coins_view uses that type for commit_batch"
  - "SchemaVersion::CURRENT stays 1; COIN-01 and CSOBS-03 remain Pending until phase verification"

patterns-established:
  - "Pattern 1: Erase B and write H=[new,old], then dirty C inserts/deletes, then erase H and write B"
  - "Pattern 2: map_optional_read never turns StorageError into Ok(None); successful miss is the only None"

requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 141-2026-09-04T17-37-21
generated_at: 2026-09-04T20:24:16Z

duration: 27min
completed: 2026-09-04
---

# Phase 141 Plan 02: FjallCoinsView BatchWrite and Fail-Closed Reads Summary

**FjallCoinsView persists dirty coins with Knots two-phase H/B BatchWrite, first-party encoded-byte caps, erase-as-delete, and fail-closed disk reads**

## Performance

- **Duration:** 27 min
- **Started:** 2026-09-04T19:57:30Z
- **Completed:** 2026-09-04T20:24:16Z
- **Tasks:** 2
- **Files modified:** 5

## Accomplishments

- `FjallCoinsView` implements `CoinsView` in the node shell: dirty unspent coins write `C` keys, spent DIRTY deletes the key, and a successful miss stays `Ok(None)`.
- BatchWrite follows Knots order: erase `B`, write `H=[new, old]`, commit 64 MiB-capped coin batches as `Buffered`, then erase `H` and write `B=new` as `Flush`.
- Two-element `H` with missing `B` is `InterruptedWrite`; other `H` counts and decode failures are `Corruption`. A `cfg(test)` crash seam leaves `H` and fails closed without `ReplayBlocks`.

## Task Commits

Each task was committed atomically:

1. **Task 1: Write failing BatchWrite, accounting, and fail-closed Fjall tests** — RED verified by writing the locked test names first; atomic RED commit skipped because `.githooks/pre-commit` runs workspace `verify.sh`.
2. **Task 2: Implement BatchWrite protocol, 64 MiB accounting, and fail-closed reads** - `6b59740c` (feat)

**Plan metadata:** pending `docs(141-02): complete FjallCoinsView BatchWrite plan`

## Files Created/Modified

- `packages/open-bitcoin-node/src/storage.rs` — `pub mod coins_view;`
- `packages/open-bitcoin-node/src/storage/coins_view.rs` — `FjallCoinsView`, `CoinsView` impl, marker classification, capped BatchWrite
- `packages/open-bitcoin-node/src/storage/coins_view/tests.rs` — H/B order, erase-as-delete, 80-byte split, fail-closed reads, crash seam
- `docs/parity/source-breadcrumbs.json` — `node-coins-adapter` files include `coins_view.rs` and tests
- `docs/metrics/lines-of-code.md` — hook-regenerated LOC freshness

## Decisions Made

- Combined RED and GREEN into one hook-passing feat commit because pre-commit runs `verify.sh`.
- Flush a partial batch after adding the item that pushes encoded bytes over the cap, matching Knots `SizeEstimate` after write.
- Use `fjall::OwnedWriteBatch` because Fjall 3.1.4 does not export `WriteBatch` at the crate root.
- Leave `COIN-01` and `CSOBS-03` Pending; this plan implements the disk parent but does not bump schema or cut over persist writes.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Combined RED and GREEN because pre-commit runs verify.sh**
- **Found during:** Task 1
- **Issue:** A RED-only commit cannot pass `.githooks/pre-commit`, which runs full `bash scripts/verify.sh`.
- **Fix:** Implemented GREEN in the same working tree, then made one feat commit after `storage::coins_view` passed (11 tests).
- **Files modified:** `packages/open-bitcoin-node/src/storage.rs`, `packages/open-bitcoin-node/src/storage/coins_view.rs`, `packages/open-bitcoin-node/src/storage/coins_view/tests.rs`, `docs/parity/source-breadcrumbs.json`
- **Verification:** `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib storage::coins_view` — 11 passed; `verify.sh` completed in 19m 38s.
- **Committed in:** `6b59740c`

**2. [Rule 3 - Blocking] Named Fjall batch type `OwnedWriteBatch`**
- **Found during:** Task 2 compile
- **Issue:** `fjall::WriteBatch` is not a public crate-root name in Fjall 3.1.4 (`pub use batch::WriteBatch as OwnedWriteBatch`).
- **Fix:** `commit_batch` takes `fjall::OwnedWriteBatch`.
- **Files modified:** `packages/open-bitcoin-node/src/storage/coins_view.rs`
- **Verification:** crate compiled; BatchWrite tests passed.
- **Committed in:** `6b59740c`

---

**Total deviations:** 2 auto-fixed (2 blocking)
**Impact on plan:** Both fixes were required to land the planned contract under repo hooks and the Fjall 3.1.4 public API. No schema bump, no `ReplayBlocks`, no persist write-site cutover.

## Issues Encountered

None beyond the Fjall `OwnedWriteBatch` export name and the known hook-vs-RED commit constraint.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan 03 can grow `ChainstateStore` / undo records against `FjallCoinsView` without changing leftover persist writes.
- `SchemaVersion::CURRENT` remains `1`. `save_chainstate_snapshot` remains the leftover persist path.
- Do not implement `ReplayBlocks` or cut over `persist_progress` in this phase.

## Self-Check: PASSED

***
*Phase: 141-durable-fjall-coins-adapter*
*Completed: 2026-09-04*
