---
phase: 139-coins-view-cache-contract-and-engine-apply
plan: 03
subsystem: chainstate
tags: [coins-cache, coins-overlay, engine-apply, stage-commit, from-snapshot]

requires:
  - phase: 139-02
    provides: "CoinsOverlay peek/add/spend, BatchWrite, Flush, Sync"
provides:
  - "Chainstate owns CoinsCache<MemoryCoinsView> and applies on a child CoinsOverlay"
  - "stage_connect / commit_staged_connect and stage_reorg / commit_staged_reorg"
  - "from_snapshot / snapshot / utxos / have_coin export unspent coins only"
affects:
  - 139-04
  - prepare-commit
  - open-bitcoin-node

tech-stack:
  added: []
  patterns:
    - "Connect, disconnect, and reorg stage on a dropped child overlay and Flush only on success"
    - "snapshot() and utxos() rematerialize unspent coins from parent plus overlay"

key-files:
  created:
    - packages/open-bitcoin-chainstate/src/engine/apply.rs
  modified:
    - packages/open-bitcoin-chainstate/src/engine.rs
    - packages/open-bitcoin-chainstate/src/engine/tests.rs
    - packages/open-bitcoin-chainstate/src/engine/tests/apply_non_coinbase_transaction_returns_fee_and_records_undo.rs
    - packages/open-bitcoin-chainstate/src/engine/tests/derives_contexts_from_chainstate_metadata.rs
    - packages/open-bitcoin-chainstate/src/engine/tests/disconnect_tip_skips_unspendable_outputs_and_reports_missing_created_out.rs
    - packages/open-bitcoin-chainstate/src/lib.rs
    - packages/open-bitcoin-chainstate/src/coins/cache.rs
    - packages/open-bitcoin-chainstate/src/coins/memory.rs
    - docs/parity/source-breadcrumbs.json
    - docs/metrics/lines-of-code.md

key-decisions:
  - "Apply mutates a child CoinsOverlay; batch_write runs only inside commit/disconnect success"
  - "from_snapshot hydrates MemoryCoinsView then CoinsCache::from_parent with an empty overlay"
  - "Keep a snapshot-backed Clone shim so the workspace stays hook-green until Plan 04 rewires prepare"
  - "Leave requirements-completed empty until phase verification exists"

patterns-established:
  - "Pattern 4: Stage on overlay, Flush on success, drop overlay on error"
  - "Pattern 5: Engine helpers treat have_coin/spend_coin/add_coin as spendability, not HashMap occupancy"

requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 139-2026-08-30T15-42-10
generated_at: 2026-08-31T02:35:00Z

duration: 303min
completed: 2026-08-31
---

# Phase 139 Plan 03: Engine Apply on Overlay Without next_utxos Clone Summary

**Chainstate now owns `CoinsCache<MemoryCoinsView>` and applies connect, disconnect, and reorg on a child overlay that Flushes only on success**

## Performance

- **Duration:** 5h 3min
- **Started:** 2026-08-30T21:31:40Z
- **Completed:** 2026-08-31T02:35:00Z
- **Tasks:** 3
- **Files modified:** 10

## Accomplishments

- Extracted overlay spend/add/restore/context helpers into `engine/apply.rs` so `engine.rs` stays under the 628-line gate.
- Replaced the live `HashMap` apply target with `CoinsCache<MemoryCoinsView>` and deleted every `next_utxos` / `self.utxos.clone()` path.
- Public `connect_block_with_current_time` is stage-then-commit; failed connect leaves live occupancy, flags, and best-block unchanged.

## Task Commits

Each task was committed atomically:

1. **Task 1: Extract engine/apply.rs before growing engine.rs** - `7e623c25` (refactor)
2. **Task 2: Retarget Chainstate onto CoinsCache with stage/commit overlay apply** - `38d643c9` (feat)
3. **Task 3: Rewrite Chainstate struct-literal fixtures to from_snapshot** - `28c7c989` (test)

**Plan metadata:** this docs commit

## Files Created/Modified

- `packages/open-bitcoin-chainstate/src/engine/apply.rs` - overlay+parent apply helpers and connect/disconnect transaction walks
- `packages/open-bitcoin-chainstate/src/engine.rs` - cache-backed Chainstate, stage/commit APIs, `apply_flushed`
- `packages/open-bitcoin-chainstate/src/lib.rs` - export `StagedChainstateConnect` and `StagedChainstateReorg`
- `packages/open-bitcoin-chainstate/src/coins/cache.rs` - `overlay_unspent_into` and `collect_unspent`
- `packages/open-bitcoin-chainstate/src/coins/memory.rs` - `unspent_coins` for snapshot export
- `packages/open-bitcoin-chainstate/src/engine/tests.rs` - HashMap adapters for helper unit tests
- `packages/open-bitcoin-chainstate/src/engine/tests/disconnect_tip_skips_unspendable_outputs_and_reports_missing_created_out.rs` - `from_snapshot` fixtures and reorg error coverage
- `packages/open-bitcoin-chainstate/src/engine/tests/derives_contexts_from_chainstate_metadata.rs` - bind owned `utxos()` before iterating
- `packages/open-bitcoin-chainstate/src/engine/tests/apply_non_coinbase_transaction_returns_fee_and_records_undo.rs` - isolation, Clone/Debug, and FreshFlagMisapplied flush coverage
- `docs/parity/source-breadcrumbs.json` - register `engine/apply.rs` under `chainstate-engine`
- `docs/metrics/lines-of-code.md` - hook-refreshed LOC report

## Decisions Made

- Stage connect/disconnect/reorg on a local `CoinsOverlay` and call `batch_write` only after the apply succeeds, so a mid-block `MissingCoin` cannot leave partial spends on the resident cache.
- Hydrate `from_snapshot` through `MemoryCoinsView::from_coins` plus `CoinsCache::from_parent`; `snapshot()` / `utxos()` rematerialize unspent coins only.
- Keep a snapshot-backed `Clone` impl so `open-bitcoin-node` still compiles until Plan 04 rewires prepare. This is a hook-green shim, not the D-04 end state.
- Add `collect_unspent` / `overlay_unspent_into` so export does not require a second live HashMap on Chainstate.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Use cargo fmt, not system rustfmt**
- **Found during:** Task 1 commit
- **Issue:** System `rustfmt` disagreed with the workspace `cargo fmt` contract and failed the hook.
- **Fix:** Format only with `cargo fmt --manifest-path packages/Cargo.toml --all`.
- **Files modified:** Task 1 engine sources
- **Verification:** subsequent hook-backed commit
- **Committed in:** `7e623c25`

**2. [Rule 3 - Blocking] Gate leftover HashMap adapters as test-only**
- **Found during:** Task 1 clippy
- **Issue:** Temporary HashMap adapters were unused in the production lib after helpers moved to overlay+parent.
- **Fix:** Keep adapters in `engine/tests.rs` so helper unit tests still compile, and keep production `engine.rs` under 628 lines.
- **Files modified:** `packages/open-bitcoin-chainstate/src/engine.rs`, `engine/tests.rs`
- **Verification:** `cargo clippy -p open-bitcoin-chainstate --lib --all-targets -- -D warnings`
- **Committed in:** `7e623c25` (extract), `28c7c989` (adapters in tests)

**3. [Rule 2 - Missing Critical] Cover spend-present and flush/reorg error branches**
- **Found during:** Task 1 and Task 2 commits (llvm-cov uncovered-line gate)
- **Issue:** Helper error paths, `batch_write ?`, `stage_reorg` disconnect/connect `?`, and snapshot-Clone were untested.
- **Fix:** Extended helper error coverage; added FreshFlagMisapplied commit coverage via `apply_flushed`; added missing-undo and missing-coin reorg tests; asserted `clone()` / `Debug` / `have_coin`.
- **Files modified:** engine tests under `packages/open-bitcoin-chainstate/src/engine/tests/`
- **Verification:** hook-backed `bash scripts/verify.sh`
- **Committed in:** `7e623c25`, `38d643c9`, `28c7c989`

**4. [Rule 3 - Blocking] Snapshot-backed Clone shim until Plan 04**
- **Found during:** Task 2
- **Issue:** Plan forbids snapshot-Clone so prepare cannot keep cloning, but workspace hooks compile `open-bitcoin-node`, which still does `self.chainstate.clone()`.
- **Fix:** Manual `Clone` via `from_snapshot(self.snapshot())`. Plan 04 must rewire prepare and can drop this impl.
- **Files modified:** `packages/open-bitcoin-chainstate/src/engine.rs`
- **Verification:** node crate compiles in the hook-backed workspace verify
- **Committed in:** `38d643c9`

**5. [Rule 2 - Missing Critical] Export unspent coins without a second live HashMap**
- **Found during:** Task 2
- **Issue:** `snapshot()` / `utxos()` need an owned unspent map after the field change.
- **Fix:** Added `CoinsOverlay::overlay_unspent_into`, `MemoryCoinsView::unspent_coins`, and `CoinsCache<MemoryCoinsView>::collect_unspent`.
- **Files modified:** `packages/open-bitcoin-chainstate/src/coins/cache.rs`, `coins/memory.rs`
- **Verification:** crate tests plus hook-backed verify
- **Committed in:** `38d643c9`

**6. [Rule 3 - Blocking] Leave requirements-completed empty until phase verification**
- **Found during:** Plan 01 docs commit (same traceability gate)
- **Issue:** Listing CACHE-01 in SUMMARY `requirements-completed` fails `check-active-milestone-verification-traceability` before a lifecycle-valid `139-VERIFICATION.md` exists.
- **Fix:** Keep `requirements-completed: []`.
- **Files modified:** this SUMMARY
- **Verification:** same checker that failed Plan 01 when CACHE-01 was listed
- **Committed in:** this docs commit

---

**Total deviations:** 6 auto-fixed (4 blocking, 2 missing-critical)
**Impact on plan:** Required for hook-passing verification and workspace compile. No node prepare rewrite (Plan 04). CACHE-01 remains pending until phase verification.

## Issues Encountered

- Task 1 hook failed first on system rustfmt vs `cargo fmt`, then clippy unused adapters, then llvm-cov on helper error paths.
- First Task 2 hook failed llvm-cov on standalone `)?;` after `batch_write` / `stage_reorg` apply calls. Collapsed flush into `apply_flushed` and added reorg plus FreshFlagMisapplied tests.
- A duplicate Task 2 commit attempt then failed llvm-cov on the snapshot-Clone impl because those assertions still lived only in unstaged Task 3 tests.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan 04 can wrap `StagedChainstateConnect` / `StagedChainstateReorg` and stop cloning the live `Chainstate`.
- Drop the snapshot-Clone shim once node prepare no longer calls `chainstate.clone()`.
- `have_coin_in_cache` is available for prepare peek-isolation assertions.

---
*Phase: 139-coins-view-cache-contract-and-engine-apply*
*Completed: 2026-08-31*

## Self-Check: PASSED

- FOUND: packages/open-bitcoin-chainstate/src/engine.rs
- FOUND: packages/open-bitcoin-chainstate/src/engine/apply.rs
- FOUND: packages/open-bitcoin-chainstate/src/lib.rs
- FOUND: packages/open-bitcoin-chainstate/src/engine/tests.rs
- FOUND: packages/open-bitcoin-chainstate/src/engine/tests/disconnect_tip_skips_unspendable_outputs_and_reports_missing_created_out.rs
- FOUND: .planning/phases/139-coins-view-cache-contract-and-engine-apply/139-03-SUMMARY.md
- FOUND: 7e623c25
- FOUND: 38d643c9
- FOUND: 28c7c989
