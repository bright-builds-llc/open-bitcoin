---
phase: 141-durable-fjall-coins-adapter
plan: 01
subsystem: storage
tags: [coins-view, coins-codec, fjall, coins-storage, head-blocks, rust]

requires:
  - phase: 139-coins-view-cache-contract-and-engine-apply
    provides: CoinsView/CoinsCache/MemoryCoinsView overlay that a later disk parent can sit behind
  - phase: 140-pure-flush-policy-and-typed-decisions
    provides: I/O-free flush/recovery decisions that must not treat disk-read failure as a miss
provides:
  - "Fallible CoinsView::best_block and head_blocks returning Result"
  - "ChainstateError::CoinsStorage distinct from MissingCoin"
  - "StorageNamespace::Coins plus an opened Fjall coins keyspace"
  - "C/B/H compact key/value codec including created_height and created_median_time_past"
affects:
  - 141-02
  - fjall-coins-view
  - schema-migration

tech-stack:
  added: []
  patterns:
    - "Parse compact coin bytes at the node boundary with Reader::finish()"
    - "CoinsCache::from_parent stays infallible; overlay best-block starts None and reads fall through to parent"

key-files:
  created:
    - packages/open-bitcoin-node/src/storage/coins_codec.rs
    - packages/open-bitcoin-node/src/storage/coins_codec/tests.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/coins_access.rs
  modified:
    - packages/open-bitcoin-chainstate/src/coins.rs
    - packages/open-bitcoin-chainstate/src/coins/cache.rs
    - packages/open-bitcoin-chainstate/src/coins/memory.rs
    - packages/open-bitcoin-chainstate/src/coins/tests.rs
    - packages/open-bitcoin-chainstate/src/engine.rs
    - packages/open-bitcoin-chainstate/src/error.rs
    - packages/open-bitcoin-node/src/storage.rs
    - packages/open-bitcoin-node/src/storage/fjall_store.rs
    - docs/parity/source-breadcrumbs.json
    - docs/metrics/lines-of-code.md

key-decisions:
  - "Combined 141-01 RED and GREEN into one hook-passing feat commit because pre-commit runs verify.sh"
  - "CoinsCache::from_parent does not probe parent.best_block; overlay starts with maybe_best_block None"
  - "FjallNodeStore database() and coins_keyspace() live in fjall_store/coins_access.rs so fjall_store.rs stays under 628 lines"
  - "SchemaVersion::CURRENT stays 1; COIN-01 and CSOBS-03 remain Pending until phase verification"

patterns-established:
  - "Pattern 1: Compact coin value is compact-size code, i64 LE sats, compact-size script, i64 LE MTP, then Reader::finish()"
  - "Pattern 2: Parent CoinsView disk errors propagate as CoinsStorage; never Ok(None) or MissingCoin"

requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 141-2026-09-04T17-37-21
generated_at: 2026-09-04T19:41:20Z

duration: 42min
completed: 2026-09-04
---

# Phase 141 Plan 01: Coins Namespace, Compact Codec, and Fallible Heads Summary

**Fallible `CoinsView` heads, typed `CoinsStorage` errors, Knots-shaped `C`/`B`/`H` keys, and a compact height+MTP codec behind `StorageNamespace::Coins`**

## Performance

- **Duration:** 42 min
- **Started:** 2026-09-04T18:58:44Z
- **Completed:** 2026-09-04T19:41:20Z
- **Tasks:** 2
- **Files modified:** 13

## Accomplishments

- `CoinsView::best_block` and `head_blocks` now return `Result` so a later disk parent can surface `ChainstateError::CoinsStorage` instead of pretending the tip is missing.
- `CoinsCache` propagates parent `CoinsStorage` on get/have/best/heads; `from_parent` stays infallible by not probing `parent.best_block()` at construction.
- Compact coin values round-trip `created_height` and `created_median_time_past`; trailing bytes, empty bytes, and `head_blocks` count 1 fail closed as `StorageError::Corruption` on `Coins`.
- `FjallNodeStore` opens the dedicated `coins` keyspace. `SchemaVersion::CURRENT` remains `1`.

## Task Commits

Each task was committed atomically:

1. **Task 1: Write failing fallible-heads, CoinsStorage, codec, and namespace tests** — RED verified locally (`70 passed; 1 failed` on `coins::tests`, compile ok). Atomic RED commit skipped because `.githooks/pre-commit` runs workspace `verify.sh`.
2. **Task 2: Implement fallible views, compact codec, and coins keyspace open** - `10688c09` (feat)

**Plan metadata:** pending `docs(141-01): complete coins namespace and compact codec plan`

## Files Created/Modified

- `packages/open-bitcoin-chainstate/src/coins.rs` — Fallible `best_block` / `head_blocks` on `CoinsView`
- `packages/open-bitcoin-chainstate/src/coins/memory.rs` — Memory heads return `Ok(Vec::new())`; best-block is `Ok(maybe_best_block)`
- `packages/open-bitcoin-chainstate/src/coins/cache.rs` — Overlay-then-parent best-block; parent `head_blocks` propagation
- `packages/open-bitcoin-chainstate/src/coins/tests.rs` — `FailingCoinsView` and fail-closed / heads tests
- `packages/open-bitcoin-chainstate/src/error.rs` — `CoinsStorage { detail }` with `coins storage error: {detail}`
- `packages/open-bitcoin-chainstate/src/engine.rs` — `coins_best_block` forwards the `Result`
- `packages/open-bitcoin-node/src/storage.rs` — `StorageNamespace::Coins` (`"coins"`); `CURRENT` still `1`
- `packages/open-bitcoin-node/src/storage/coins_codec.rs` — `C`+txid+LE vout keys and compact height+MTP values
- `packages/open-bitcoin-node/src/storage/coins_codec/tests.rs` — Key, round-trip, and fail-closed decode tests
- `packages/open-bitcoin-node/src/storage/fjall_store.rs` — Opens the coins keyspace
- `packages/open-bitcoin-node/src/storage/fjall_store/coins_access.rs` — `database()` / `coins_keyspace()` for Plan 02
- `docs/parity/source-breadcrumbs.json` — `node-coins-adapter` group

## Decisions Made

- Combined RED and GREEN into one hook-passing feat commit because pre-commit runs `verify.sh`.
- Keep `CoinsCache::from_parent` infallible: overlay `maybe_best_block` starts `None` and `best_block()` falls through to the parent.
- Extract `database()` and `coins_keyspace()` into `fjall_store/coins_access.rs` so `fjall_store.rs` stays under the 628-line production gate.
- Leave `COIN-01` and `CSOBS-03` Pending; this plan only starts those requirements.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Extracted unused Plan 02 accessors so fjall_store.rs stays under 628 lines**
- **Found during:** Task 2 commit (`verify.sh` file-length check)
- **Issue:** Adding `coins` keyspace plus `database()` / `coins_keyspace()` made `fjall_store.rs` 629 lines.
- **Fix:** Moved the two `pub(crate)` accessors into `fjall_store/coins_access.rs`.
- **Files modified:** `packages/open-bitcoin-node/src/storage/fjall_store.rs`, `packages/open-bitcoin-node/src/storage/fjall_store/coins_access.rs`, `docs/parity/source-breadcrumbs.json`
- **Verification:** Production file-length check passed (374 files, limit 628).
- **Committed in:** `10688c09`

**2. [Rule 3 - Blocking] Combined RED and GREEN because pre-commit runs verify.sh**
- **Found during:** Task 1
- **Issue:** A RED-only commit cannot pass `.githooks/pre-commit`, which runs full `bash scripts/verify.sh`.
- **Fix:** Verified RED locally, implemented GREEN, then made one feat commit.
- **Files modified:** same as Task 1 + Task 2
- **Verification:** `coins::tests` 71 passed; `storage::coins_codec` 6 passed; `verify.sh` completed.
- **Committed in:** `10688c09`

---

**Total deviations:** 2 auto-fixed (2 blocking)
**Impact on plan:** Both fixes were required to land the planned contract under repo hooks and the 628-line gate. No scope creep; no `FjallCoinsView`, schema bump, or persist cutover.

## Issues Encountered

- System `rustfmt` reordered imports differently than pinned toolchain `cargo fmt --all`. Used `cargo fmt --manifest-path packages/Cargo.toml --all` for the hook-passing style.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan 02 can implement `FjallCoinsView` on `FjallNodeStore::coins_keyspace()` with the locked `C`/`B`/`H` codec.
- `database()` and `coins_keyspace()` are `pub(crate)` and `allow(dead_code)` until Plan 02 calls them.
- Do not bump `SchemaVersion::CURRENT` until Plan 04.

## Self-Check: PASSED

---
*Phase: 141-durable-fjall-coins-adapter*
*Completed: 2026-09-04*
