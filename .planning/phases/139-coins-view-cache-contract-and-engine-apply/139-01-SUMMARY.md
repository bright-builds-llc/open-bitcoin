---
phase: 139-coins-view-cache-contract-and-engine-apply
plan: 01
subsystem: chainstate
tags: [coins-view, coins-cache, dirty-fresh, utxo, rust]

requires: []
provides:
  - "CoinsView trait with peek-only get_coin/have_coin/best_block/batch_write"
  - "Five-state CoinsCacheEntry constructors and InvalidCacheEntry"
  - "MemoryCoinsView parent truth and resident CoinsCache overlay lookup"
affects:
  - 139-02
  - engine-apply
  - cache-flush

tech-stack:
  added: []
  patterns:
    - "Five-state DIRTY/FRESH enum instead of bitflags"
    - "Peek-only CoinsView; fetch_coin inserts only into self"

key-files:
  created:
    - packages/open-bitcoin-chainstate/src/coins.rs
    - packages/open-bitcoin-chainstate/src/coins/cache.rs
    - packages/open-bitcoin-chainstate/src/coins/memory.rs
    - packages/open-bitcoin-chainstate/src/coins/tests.rs
  modified:
    - packages/open-bitcoin-chainstate/src/error.rs
    - packages/open-bitcoin-chainstate/src/lib.rs
    - docs/parity/source-breadcrumbs.json
    - docs/metrics/lines-of-code.md

key-decisions:
  - "Encode DIRTY/FRESH as a five-state enum so invalid SanityCheck masks cannot be constructed"
  - "CoinsView::get_coin/have_coin never insert; CoinsCache::fetch_coin is the only self-populating lookup"
  - "Gate insert_entry_for_test with cfg(test) so production lib deny(dead_code) stays clean"
  - "CoinsCache::batch_write installs overlay entries only; Flush algebra stays in Plan 02"

patterns-established:
  - "Pattern 1: Layered coins view with MemoryCoinsView parent and CoinsCache overlay"
  - "Pattern 2: Occupancy, have_coin_in_cache, peek HaveCoin, and parent truth are four distinct facts"

requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 139-2026-08-30T15-42-10
generated_at: 2026-08-30T18:06:02Z

duration: 70min
completed: 2026-08-30
---

# Phase 139 Plan 01: CoinsView Contract and Lookup Facts Summary

**Knots-shaped CoinsView/CoinsCache overlay with four lookup facts and five valid DIRTY/FRESH states**

## Performance

- **Duration:** 70 min
- **Started:** 2026-08-30T16:56:19Z
- **Completed:** 2026-08-30T18:06:02Z
- **Tasks:** 2
- **Files modified:** 8

## Accomplishments

- Added `CoinsView`, `CoinsBatch`, `CoinsCacheEntry`, and the five-state `CoinsCacheFlags` enum in `open-bitcoin-chainstate`.
- `MemoryCoinsView` stores only live unspent coins; `CoinsCache` overlays occupancy including spent DIRTY tombstones without cloning a live UTXO map.
- Peek `get_coin`/`have_coin` never insert; `fetch_coin` inserts `UnspentClean` into self only; spent overlay occupancy returns `have_coin = false` without consulting the parent.

## Task Commits

Each task was committed atomically:

1. **Task 1: Write failing lookup, five-state, and MemoryCoinsView tests** - `e885a0e8` (test)
2. **Task 2: Implement MemoryCoinsView, CoinsCacheEntry, and peek/fetch lookup** - `55373085` (feat)

**Plan metadata:** `docs(139-01): complete coins view lookup contract plan`

_Note: TDD RED was verified locally (7 failing `coins::` tests) before GREEN. A red-only commit cannot land because pre-commit runs `bash scripts/verify.sh`._

## Files Created/Modified

- `packages/open-bitcoin-chainstate/src/coins.rs` - `CoinsView` trait, `CoinsBatch`, five-state flags, `CoinsCacheEntry` constructors
- `packages/open-bitcoin-chainstate/src/coins/cache.rs` - resident `CoinsCache` overlay with peek/fetch lookup
- `packages/open-bitcoin-chainstate/src/coins/memory.rs` - in-memory parent truth
- `packages/open-bitcoin-chainstate/src/coins/tests.rs` - four-fact, five-state, fetch, and batch_write coverage
- `packages/open-bitcoin-chainstate/src/error.rs` - `InvalidCacheEntry` and `FreshFlagMisapplied`
- `packages/open-bitcoin-chainstate/src/lib.rs` - `pub mod coins` and re-exports
- `docs/parity/source-breadcrumbs.json` - coins files added to `chainstate-engine`
- `docs/metrics/lines-of-code.md` - hook-refreshed LOC report

## Decisions Made

- Use a five-state enum (not a bitflags crate) so FRESH-only unspent, spent-clean, and spent FRESH|DIRTY are unrepresentable in infallible constructors.
- Keep `Coin` as a live output; spentness lives on `CoinsCacheEntry`.
- `CoinsCache` is not `Clone`. Prepare isolation remains a later child overlay, not a cloned map.
- `insert_entry_for_test` is `pub(crate)` and `#[cfg(test)]` so Plan 02 can own `add_coin`/`spend_coin` without a production-only helper.
- `CoinsCache::batch_write` installs overlay entries and optional best-block; Knots Flush/FRESH-parent-delete algebra stays in Plan 02.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Gate test-only tombstone insert behind cfg(test)**
- **Found during:** Task 1 commit (pre-commit verify)
- **Issue:** `insert_entry_for_test` is unused in the production lib, so `deny(dead_code)` failed `cargo check`.
- **Fix:** Marked the helper `#[cfg(test)]`.
- **Files modified:** `packages/open-bitcoin-chainstate/src/coins/cache.rs`
- **Verification:** `cargo check -p open-bitcoin-chainstate --lib` and later `verify.sh` both passed
- **Committed in:** `55373085` (Task 2)

**2. [Rule 2 - Missing Critical] Add coverage for valid try_from_flags, fetch miss/hit, and batch_write**
- **Found during:** Task 1 commit (llvm-cov uncovered-line gate)
- **Issue:** Valid `try_from_flags` arms, `fetch_coin` occupancy hit and parent miss, cache `have_coin`/`best_block`/`batch_write`, and MemoryCoinsView unspent insert / best-block update were untested.
- **Fix:** Added focused unit tests for those branches.
- **Files modified:** `packages/open-bitcoin-chainstate/src/coins/tests.rs`
- **Verification:** `cargo test -p open-bitcoin-chainstate --lib coins::` (13 passed) and `verify.sh` coverage
- **Committed in:** `e885a0e8` (Task 1)

**3. [Process] Land Task 1 after GREEN because hooks run full verify**
- **Found during:** Task 1 commit
- **Issue:** TDD asked for a failing-test commit, but `.githooks/pre-commit` runs `bash scripts/verify.sh`, which cannot succeed on a red suite.
- **Fix:** Verified RED locally (7 failures, compile-clean), implemented GREEN, then committed tests/errors first and implementation second. Both commits are hook-green.
- **Files modified:** none beyond the planned task files
- **Verification:** Local RED cargo test, then two successful hook-backed commits
- **Committed in:** `e885a0e8`, `55373085`

**4. [Rule 3 - Blocking] Refresh breadcrumbs to the chainstate-engine group set**
- **Found during:** Task 1
- **Issue:** New coins files were added to the existing `chainstate-engine` group, which also cites validation and blockstorage anchors.
- **Fix:** Ran `bun run scripts/check-parity-breadcrumbs.ts --write` so comments match the group mapping.
- **Files modified:** `packages/open-bitcoin-chainstate/src/coins.rs`, `coins/cache.rs`, `coins/memory.rs`, `coins/tests.rs`
- **Verification:** `bun run scripts/check-parity-breadcrumbs.ts --check`
- **Committed in:** `e885a0e8`, `55373085`

**5. [Rule 3 - Blocking] Leave requirements-completed empty until phase verification**
- **Found during:** docs commit
- **Issue:** Listing CACHE-01 in SUMMARY `requirements-completed` fails `check-active-milestone-verification-traceability` because no lifecycle-valid 139-VERIFICATION.md exists yet.
- **Fix:** Keep `requirements-completed: []` so CACHE-01 stays pending until phase verification.
- **Files modified:** `.planning/phases/139-coins-view-cache-contract-and-engine-apply/139-01-SUMMARY.md`
- **Verification:** retry of hook-backed docs commit
- **Committed in:** this docs commit

***

**Total deviations:** 5 auto-fixed (3 blocking, 1 missing-critical, 1 process)
**Impact on plan:** Required for hook-passing verification and typed construction. No scope creep into add/spend/Flush or engine apply. CACHE-01 remains pending until phase verification.

## Issues Encountered

- First hook-backed commit failed on production `dead_code` for `insert_entry_for_test`.
- Second hook-backed attempt failed llvm-cov uncovered lines in `try_from_flags`, `fetch_coin`, cache `batch_write`, and MemoryCoinsView insert/best-block update. Coverage tests were added before retrying.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan 02 can add `add_coin` / `spend_coin` / Flush algebra on top of the peek/fetch contract.
- `FreshFlagMisapplied` is registered and unused until BatchWrite FRESH-misapply tests.
- Engine apply and node prepare remain untouched, as required.

***
*Phase: 139-coins-view-cache-contract-and-engine-apply*
*Completed: 2026-08-30*

## Self-Check: PASSED

- FOUND: packages/open-bitcoin-chainstate/src/coins.rs
- FOUND: packages/open-bitcoin-chainstate/src/coins/cache.rs
- FOUND: packages/open-bitcoin-chainstate/src/coins/memory.rs
- FOUND: packages/open-bitcoin-chainstate/src/coins/tests.rs
- FOUND: e885a0e8
- FOUND: 55373085
