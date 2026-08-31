---
phase: 139-coins-view-cache-contract-and-engine-apply
plan: 04
subsystem: chainstate
tags: [prepare-commit, staged-overlay, have-coin, leftover-persist, mempool-lifecycle]

requires:
  - phase: 139-03
    provides: "stage_connect / commit_staged_connect, stage_reorg / commit_staged_reorg, have_coin, snapshot / from_snapshot"
provides:
  - "Prepared* own StagedChainstateConnect / StagedChainstateReorg instead of cloned Chainstate"
  - "commit_prepared_* absorb Flush then leftover snapshot-blob persist"
  - "mempool missing-input classification uses have_coin"
affects:
  - 139-05
  - flush-policy
  - fjall-coins

tech-stack:
  added: []
  patterns:
    - "Prepare stages a detached overlay; commit absorbs Flush so the mempool window stays infallible"
    - "ManagedChainstate Clone rematerializes via from_snapshot leftover only"

key-files:
  created:
    - packages/open-bitcoin-chainstate/src/engine/stage.rs
    - packages/open-bitcoin-node/src/chainstate/tests.rs
  modified:
    - packages/open-bitcoin-node/src/chainstate.rs
    - packages/open-bitcoin-node/src/network/mempool_lifecycle.rs
    - packages/open-bitcoin-chainstate/src/engine.rs
    - packages/open-bitcoin-chainstate/src/coins/cache.rs
    - docs/parity/source-breadcrumbs.json
    - docs/metrics/lines-of-code.md

key-decisions:
  - "Absorb staged overlays on commit so D-18 mempool commits stay infallible"
  - "Keep ManagedChainstate Clone via from_snapshot leftover; do not Clone Chainstate"
  - "persist() still writes the snapshot blob (D-21)"

patterns-established:
  - "Pattern 6: Prepared* hold staged overlays; live cache stays cold until absorb"
  - "Pattern 7: Orphan parent checks peek have_coin, not UTXO map occupancy"

requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 139-2026-08-30T15-42-10
generated_at: 2026-08-31T04:16:29Z

duration: 77min
completed: 2026-08-31
---

# Phase 139 Plan 04: Manager Prepare/Commit Overlay and Leftover Persist Summary

**ManagedChainstate prepare now owns staged overlays, commit absorbs Flush then leftover snapshot-blob persist, and mempool orphan checks peek via have_coin**

## Performance

- **Duration:** 1h 17min
- **Started:** 2026-08-31T02:59:53Z
- **Completed:** 2026-08-31T04:16:29Z
- **Tasks:** 2
- **Files modified:** 11

## Accomplishments

- Retargeted `prepare_connect_block` / `prepare_reorg` onto Plan 03 `stage_*` overlays so prepare no longer clones `Chainstate`.
- Made `commit_prepared_*` absorb Flush into the live cache, then `persist()` still writes the snapshot blob.
- Switched mempool missing-input classification to `have_coin` and proved a failed prepare leaves the live cache cold across the two-phase window.

## Task Commits

Each task was committed atomically:

1. **Task 1: Retarget prepare onto staged overlays** - `66c0005a` (feat)
2. **Task 2: Switch mempool have_coin and prove two-phase isolation** - `9a5042fc` (feat)

**Plan metadata:** this docs commit

## Files Created/Modified

- `packages/open-bitcoin-node/src/chainstate.rs` - `Prepared*` own staged overlays; absorb on commit; leftover snapshot Clone and persist
- `packages/open-bitcoin-node/src/chainstate/tests.rs` - prepare isolation, persist blob, and failed-prepare-before-mempool tests
- `packages/open-bitcoin-node/src/network/mempool_lifecycle.rs` - orphan parent check uses `have_coin`
- `packages/open-bitcoin-chainstate/src/engine/stage.rs` - `absorb_staged_*` and `install_staged_reorg_preview`
- `packages/open-bitcoin-chainstate/src/engine.rs` - drop `Clone` on `Chainstate`; export stage absorb helpers
- `packages/open-bitcoin-chainstate/src/coins/cache.rs` - `absorb_batch_write` overwrites FRESH-vs-live-unspent without failing
- `packages/open-bitcoin-chainstate/src/coins/tests/algebra.rs` - absorb skip/parent-miss/fresh-parent-erase coverage
- `packages/open-bitcoin-chainstate/src/engine/tests/apply_non_coinbase_transaction_returns_fee_and_records_undo.rs` - absorb/preview coverage; snapshot rematerialize instead of `clone()`
- `docs/parity/source-breadcrumbs.json` - register `engine/stage.rs` and `chainstate/tests.rs`
- `docs/metrics/lines-of-code.md` - hook-refreshed LOC report

## Decisions Made

- Use `absorb_staged_connect` / `absorb_staged_reorg` on the manager commit path so D-18 stays infallible: mempool still commits after the closure even if `R` is `Err`. `commit_staged_*` remain Result-returning for engine callers.
- `install_prepared_reorg_preview` flushes a clone of the staged overlay plus metadata; it does not `preview().clone()`.
- Keep leftover snapshot rematerialization only on `ManagedChainstate` (`from_snapshot(self.chainstate.snapshot())`) for `reorg_to_branch` staging. Do not impl `Clone` on `Chainstate`.
- Leave `persist()` on `save_snapshot(self.chainstate.snapshot())` (D-21). No Fjall coins, flush policy, or manager-restart work.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Combine Task 1 tests with implementation**
- **Found during:** Task 1 TDD RED
- **Issue:** Isolation tests already passed against the snapshot-clone shim (characterization, not a true RED). Separate test-only commits failed because pre-commit `verify.sh` compiles the whole tree and llvm-cov flagged unstaged absorb/stage implementation.
- **Fix:** Land tests and overlay retarget in one feat commit. Document as a TDD process deviation, not a behavior skip.
- **Files modified:** `packages/open-bitcoin-node/src/chainstate.rs`, `chainstate/tests.rs`, `engine/stage.rs`
- **Verification:** hook-backed `bash scripts/verify.sh`
- **Committed in:** `66c0005a`

**2. [Rule 2 - Missing Critical] Infallible absorb instead of Result `commit_staged_*`**
- **Found during:** Task 1
- **Issue:** Plan key-link text says commit calls `commit_staged_*`, but those return `Result` and D-18 requires `commit_prepared_*` to stay infallible inside the mempool transition closure.
- **Fix:** Added `absorb_staged_connect` / `absorb_staged_reorg` that Flush then install metadata without returning `Err`.
- **Files modified:** `packages/open-bitcoin-chainstate/src/engine/stage.rs`, `packages/open-bitcoin-node/src/chainstate.rs`
- **Verification:** isolation plus persist tests; hook-backed verify
- **Committed in:** `66c0005a`

**3. [Rule 1 - Bug] Absorb overwrites FRESH-vs-live-unspent occupancy**
- **Found during:** Task 1 llvm-cov / absorb correctness
- **Issue:** Child-overlay Flush can present FRESH against a live unspent parent entry; Result `batch_write` would return `FreshFlagMisapplied` and break D-18.
- **Fix:** `absorb_batch_write` overwrites dirty without FRESH instead of failing.
- **Files modified:** `packages/open-bitcoin-chainstate/src/coins/cache.rs`, algebra tests
- **Verification:** `absorb_batch_write_clears_fresh_on_live_unspent_occupancy`; hook-backed verify
- **Committed in:** `66c0005a`

**4. [Rule 3 - Blocking] Clippy `manual_unwrap_or_default` on have_coin match**
- **Found during:** Task 2 commit
- **Issue:** The plan's `match have_coin { Ok(present) => present, Err(_) => false }` trips `-D clippy::manual_unwrap_or_default`.
- **Fix:** Use `.unwrap_or_default()` (bool default is `false`). This is not `unwrap()` and does not panic.
- **Files modified:** `packages/open-bitcoin-node/src/network/mempool_lifecycle.rs`
- **Verification:** hook-backed `bash scripts/verify.sh`
- **Committed in:** `9a5042fc`

**5. [Rule 3 - Blocking] Import network address types from core primitives**
- **Found during:** Task 2 compile
- **Issue:** `NetworkAddress` and `NetworkMagic` are not in the `open_bitcoin_network` crate root.
- **Fix:** Import them from `open_bitcoin_core::primitives`, matching `network/tests.rs`.
- **Files modified:** `packages/open-bitcoin-node/src/chainstate/tests.rs`
- **Verification:** `cargo test -p open-bitcoin-node --lib`
- **Committed in:** `9a5042fc`

**6. [Rule 3 - Blocking] Leave requirements-completed empty until phase verification**
- **Found during:** Plan 03 lesson
- **Issue:** Listing CACHE-01 in SUMMARY `requirements-completed` fails `check-active-milestone-verification-traceability` before a lifecycle-valid `139-VERIFICATION.md` exists.
- **Fix:** Keep `requirements-completed: []`.
- **Files modified:** this SUMMARY
- **Verification:** same checker that failed Plan 01 when CACHE-01 was listed
- **Committed in:** this docs commit

---

**Total deviations:** 6 auto-fixed (4 blocking, 1 bug, 1 missing-critical)
**Impact on plan:** Overlay absorb is required for D-18. TDD process adapted to hook coverage. CACHE-01 remains pending until phase verification. No Fjall/flush-policy scope creep.

## Issues Encountered

- Task 1 isolation tests passed against the leftover clone shim, so RED was characterization rather than a failing contract.
- First Task 2 commit failed clippy on the plan's `match have_coin` form; `unwrap_or_default` satisfied `-D warnings` without using `unwrap()`.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Prepare isolation is in place; leftover snapshot persist and `ManagedChainstate` snapshot Clone remain until flush/Fjall/manager-restart work.
- Two-phase mempool order is unchanged: prepare chainstate, prepare mempool, commit both.
- Do not treat a cache hit as durably persisted.

---
*Phase: 139-coins-view-cache-contract-and-engine-apply*
*Completed: 2026-08-31*

## Self-Check: PASSED

- FOUND: packages/open-bitcoin-node/src/chainstate.rs
- FOUND: packages/open-bitcoin-node/src/chainstate/tests.rs
- FOUND: packages/open-bitcoin-node/src/network/mempool_lifecycle.rs
- FOUND: packages/open-bitcoin-chainstate/src/engine/stage.rs
- FOUND: packages/open-bitcoin-chainstate/src/engine.rs
- FOUND: packages/open-bitcoin-chainstate/src/coins/cache.rs
- FOUND: .planning/phases/139-coins-view-cache-contract-and-engine-apply/139-04-SUMMARY.md
- FOUND: 66c0005a
- FOUND: 9a5042fc
