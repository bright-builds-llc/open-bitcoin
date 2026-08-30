---
phase: 139-coins-view-cache-contract-and-engine-apply
plan: 02
subsystem: chainstate
tags: [coins-cache, dirty-fresh, add-coin, spend-coin, batch-write, flush, sync, overlay]

requires:
  - phase: 139-01
    provides: "CoinsView peek contract, five-state CoinsCacheEntry, MemoryCoinsView, CoinsCache lookup"
provides:
  - "CoinsOverlay detached peek/add/spend plus into_dirty_batch"
  - "AddCoin FRESH rule, SpendCoin erase vs tombstone, Knots BatchWrite"
  - "Flush empties dirty occupancy; Sync retains unspent clean"
affects:
  - 139-03
  - engine-apply
  - prepare-commit

tech-stack:
  added: []
  patterns:
    - "Shared add/spend helpers take a peek-beneath closure so overlay occupancy is first"
    - "Flush clears the child after parent.batch_write; Sync drops spent and keeps UnspentClean"

key-files:
  created:
    - packages/open-bitcoin-chainstate/src/coins/tests/algebra.rs
  modified:
    - packages/open-bitcoin-chainstate/src/coins/cache.rs
    - packages/open-bitcoin-chainstate/src/coins.rs
    - packages/open-bitcoin-chainstate/src/coins/tests.rs
    - docs/parity/source-breadcrumbs.json
    - docs/metrics/lines-of-code.md

key-decisions:
  - "CoinsCache stores occupancy in CoinsOverlay so prepare can own a detached overlay without a parent pointer"
  - "AddCoin/SpendCoin peek via CoinsView::get_coin, never parent.fetch_coin"
  - "FRESH+spent against a FRESH parent erases the parent entry instead of writing a tombstone"
  - "Leave requirements-completed empty until phase verification exists"

patterns-established:
  - "Pattern 3: Detached CoinsOverlay peeks the parent cache without warming it"
  - "Pattern 1: Flush is child-emptying BatchWrite; Sync retains unspent clean in memory"

requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 139-2026-08-30T15-42-10
generated_at: 2026-08-30T20:59:20Z

duration: 130min
completed: 2026-08-30
---

# Phase 139 Plan 02: DIRTY/FRESH Mutation Algebra and Peek Overlay Summary

**Knots AddCoin/SpendCoin/BatchWrite/Flush/Sync algebra on a detached CoinsOverlay that peeks without populating the parent**

## Performance

- **Duration:** 2h 10m
- **Started:** 2026-08-30T18:49:12Z
- **Completed:** 2026-08-30T20:59:20Z
- **Tasks:** 2
- **Files modified:** 6

## Accomplishments

- Ported AddCoin FRESH (`fresh = !is_dirty` on spent occupancy), SpendCoin FRESH-erase vs DIRTY tombstone, and BatchWrite including `FreshFlagMisapplied`.
- Child `CoinsOverlay::peek_coin` uses `parent.get_coin` only; failed prepare cannot warm parent occupancy.
- Flush writes dirty entries including spent tombstones then clears the child; Sync writes dirty entries, drops spent keys, and keeps unspent as `UnspentClean`.

## Task Commits

Each task was committed atomically:

1. **Task 1: Write failing AddCoin, SpendCoin, BatchWrite, Flush, Sync, and peek tests** - `2e0eb8a6` (test)
2. **Task 2: Implement overlay mutation, BatchWrite, Flush, and Sync** - `0cd4a0bc` (feat)

**Plan metadata:** this docs commit

_Note: TDD RED was verified locally (11 failing `coins::tests::algebra` tests, compile-clean) before GREEN. A red-only commit cannot land because pre-commit runs `bash scripts/verify.sh`._

## Files Created/Modified

- `packages/open-bitcoin-chainstate/src/coins/cache.rs` - `CoinsOverlay`, add/spend helpers, Knots BatchWrite, Flush, Sync
- `packages/open-bitcoin-chainstate/src/coins.rs` - export `CoinsOverlay`; `Clone` plus `is_dirty`/`is_fresh`/`is_spent` on entries
- `packages/open-bitcoin-chainstate/src/coins/tests.rs` - `mod algebra` and shared second-outpoint fixture
- `packages/open-bitcoin-chainstate/src/coins/tests/algebra.rs` - named Knots algebra and peek-isolation tests
- `docs/parity/source-breadcrumbs.json` - algebra tests added to `chainstate-engine`
- `docs/metrics/lines-of-code.md` - hook-refreshed LOC report

## Decisions Made

- Put shared AddCoin/SpendCoin bodies on peek-closure helpers so `CoinsCache` checks overlay first and peeks `parent.get_coin` on miss.
- Encode Flush as emptying `parent.batch_write` of dirty occupancy, including spent tombstones; Sync is in-memory retain-unspent only.
- Child dirty Spent against a FRESH parent deletes the parent entry (D-12); do not write a spent tombstone into a FRESH parent.

## Deviations from Plan

### Auto-fixed Issues

**1. [Process] Land Task 1 after GREEN because hooks run full verify**
- **Found during:** Task 1 commit
- **Issue:** TDD asked for a failing-test commit, but `.githooks/pre-commit` runs `bash scripts/verify.sh`.
- **Fix:** Verified RED locally (11 algebra failures, Plan 01 tests still green), implemented GREEN, then committed tests first and implementation second.
- **Files modified:** none beyond the planned task files
- **Verification:** Local RED cargo test, then two successful hook-backed commits
- **Committed in:** `2e0eb8a6`, `0cd4a0bc`

**2. [Rule 3 - Blocking] Implement Default for CoinsOverlay**
- **Found during:** Task 1 commit (clippy `new_without_default`)
- **Issue:** Public `CoinsOverlay::new()` requires `Default`.
- **Fix:** Added `impl Default for CoinsOverlay` and used `CoinsOverlay::default()` in the peek test.
- **Files modified:** `packages/open-bitcoin-chainstate/src/coins/cache.rs`, `coins/tests/algebra.rs`
- **Verification:** `cargo clippy -p open-bitcoin-chainstate --lib -- -D warnings`
- **Committed in:** `0cd4a0bc` (Default), `2e0eb8a6` (test call)

**3. [Rule 2 - Missing Critical] Cover overlay occupancy add/spend and BatchWrite overwrite arms**
- **Found during:** Task 1 commit (llvm-cov uncovered-line gate)
- **Issue:** Overlay-unspent AddCoin, spent-occupancy SpendCoin, overlay mutation helpers, and dirty-unspent BatchWrite overwrite were untested.
- **Fix:** Added focused algebra tests for those branches.
- **Files modified:** `packages/open-bitcoin-chainstate/src/coins/tests/algebra.rs`
- **Verification:** `cargo test -p open-bitcoin-chainstate --lib coins::` (30 passed) and hook-backed verify
- **Committed in:** `2e0eb8a6`

**4. [Rule 3 - Blocking] Split algebra tests to stay under the file-length gate**
- **Found during:** Task 1
- **Issue:** Eleven Arrange/Act/Assert algebra tests would push `coins/tests.rs` past 628 lines.
- **Fix:** Added `coins/tests/algebra.rs` and `mod algebra;`, and registered the file in `chainstate-engine` breadcrumbs.
- **Files modified:** `packages/open-bitcoin-chainstate/src/coins/tests.rs`, `coins/tests/algebra.rs`, `docs/parity/source-breadcrumbs.json`
- **Verification:** file-length check in verify.sh
- **Committed in:** `2e0eb8a6`

**5. [Rule 3 - Blocking] Leave requirements-completed empty until phase verification**
- **Found during:** Plan 01 docs commit (same traceability gate)
- **Issue:** Listing CACHE-01 in SUMMARY `requirements-completed` fails `check-active-milestone-verification-traceability` before a lifecycle-valid `139-VERIFICATION.md` exists.
- **Fix:** Keep `requirements-completed: []`.
- **Files modified:** this SUMMARY
- **Verification:** same checker that failed Plan 01 when CACHE-01 was listed
- **Committed in:** this docs commit

---

**Total deviations:** 5 auto-fixed (3 blocking, 1 missing-critical, 1 process)
**Impact on plan:** Required for hook-passing verification. No scope creep into engine apply or node prepare. CACHE-01 remains pending until phase verification.

## Issues Encountered

- First hook-backed Task 1 attempt failed clippy `new_without_default` on `CoinsOverlay`.
- Second hook-backed Task 1 attempt failed llvm-cov on overlay-unspent AddCoin, spent-occupancy SpendCoin, and dirty-unspent BatchWrite overwrite.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan 03 can retarget engine apply onto `add_coin` / `spend_coin` / Flush.
- Detached `CoinsOverlay` is ready for prepare isolation without cloning `CoinsCache`.
- Engine and node files remain unchanged, as required.

---
*Phase: 139-coins-view-cache-contract-and-engine-apply*
*Completed: 2026-08-30*

## Self-Check: PASSED

- FOUND: packages/open-bitcoin-chainstate/src/coins.rs
- FOUND: packages/open-bitcoin-chainstate/src/coins/cache.rs
- FOUND: packages/open-bitcoin-chainstate/src/coins/tests.rs
- FOUND: packages/open-bitcoin-chainstate/src/coins/tests/algebra.rs
- FOUND: .planning/phases/139-coins-view-cache-contract-and-engine-apply/139-02-SUMMARY.md
- FOUND: 2e0eb8a6
- FOUND: 0cd4a0bc
