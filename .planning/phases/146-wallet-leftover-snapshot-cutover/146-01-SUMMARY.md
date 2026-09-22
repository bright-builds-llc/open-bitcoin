---
phase: 146-wallet-leftover-snapshot-cutover
plan: "01"
subsystem: wallet
tags: [wallet-rescan, coins, chain_meta, leftover-snapshot, has_block, SNAP-01]

requires:
  - phase: 141-durable-fjall-coins-adapter
    provides: durable coins C/B, seed_coins_from_snapshot, chain_meta
  - phase: 142-manager-flush-lifecycle-and-restart
    provides: leftover snapshot non-authoritative for persist_progress
  - phase: 143-honest-stored-block-availability
    provides: has_block payload-present probe
provides:
  - wallet_scan_chainstate_snapshot coins + chain_meta assembly
  - WalletRescanRuntime cut off load_chainstate_snapshot
  - has_block fail-closed chunk gate with missing block payload
  - disagreeing leftover reopen and missing-payload tests
affects:
  - 146-wallet-leftover-snapshot-cutover
  - 147-prune-policy
  - 148-fjall-unlink

tech-stack:
  added: []
  patterns:
    - "Wallet scan authority = scan_coin_records + load_chain_meta_for_open + has_block; never leftover snapshot"
    - "Missing payload marks rescan job failed with UnavailableNamespace::BlockIndex; no Pruned"

key-files:
  created: []
  modified:
    - packages/open-bitcoin-node/src/storage/fjall_store/coins.rs
    - packages/open-bitcoin-node/src/sync/wallet_rescan.rs
    - packages/open-bitcoin-node/src/sync/tests/wallet_rescan_runtime.rs
    - packages/open-bitcoin-bench/src/cases/wallet_rescan.rs
    - docs/metrics/lines-of-code.md

key-decisions:
  - "Keep feeding assembled ChainstateSnapshot into rescan_chainstate; do not invent a narrower wallet scan-input type"
  - "Combined Task 1 and Task 2 into one hook-passing feat commit because pre-commit runs full verify.sh"
  - "wallet-rescan bench plants coins via seed_coins_from_snapshot and save_block so has_block gate passes"

patterns-established:
  - "Pattern: wallet_scan_chainstate_snapshot fails closed on empty chain_meta or coins-B vs tip mismatch"
  - "Pattern: advance_wallet_rescan gates every chunk height with has_block before rescan_chainstate"

requirements-completed: []  # SNAP-01 stays Pending until lifecycle-valid phase verification
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 146-2026-09-21T20-57-36
generated_at: 2026-09-22T00:12:56Z

duration: 79min
completed: 2026-09-22
---

# Phase 146 Plan 01: WalletRescanRuntime Coins-Backed Cutover Summary

**WalletRescanRuntime now builds scan input from durable coins + `chain_meta` and fails closed on missing block payloads, so leftover `"snapshot"` bytes cannot change wallet tip or balances.**

## Performance

- **Duration:** 79 min
- **Started:** 2026-09-21T22:53:48Z
- **Completed:** 2026-09-22T00:12:56Z
- **Tasks:** 2
- **Files modified:** 5

## Accomplishments

- Added `FjallNodeStore::wallet_scan_chainstate_snapshot` that never reads leftover `"snapshot"`.
- Replaced `required_chainstate_snapshot` so wallet rescan uses coins + `chain_meta` only.
- Gated each rescan chunk with `has_block`, failing closed with `missing block payload`.
- Proved disagreeing leftover reopen preserves coins tip/UTXOs and leftover blob remains on disk.
- Updated the wallet-rescan bench fixture to plant coins and payloads after the cutover.

## Task Commits

Each task was committed atomically:

1. **Task 1 + Task 2 (combined):** `8c5fec01` (feat) — coins-backed cutover, has_block gate, reopen/missing-payload tests, and bench fixture update

**Plan metadata:** `46a83ccd` (docs: complete plan)

_Note: TDD RED/GREEN and the two plan tasks were combined into one hook-passing feat commit because pre-commit always runs `bash scripts/verify.sh`._

## Files Created/Modified

- `packages/open-bitcoin-node/src/storage/fjall_store/coins.rs` — `wallet_scan_chainstate_snapshot`
- `packages/open-bitcoin-node/src/sync/wallet_rescan.rs` — coins-backed required snapshot + has_block gate
- `packages/open-bitcoin-node/src/sync/tests/wallet_rescan_runtime.rs` — disagreeing leftover and missing-payload tests; resume test plants payloads
- `packages/open-bitcoin-bench/src/cases/wallet_rescan.rs` — coins + payload plant for bench case
- `docs/metrics/lines-of-code.md` — hook-regenerated LOC freshness

## Decisions Made

- Keep `rescan_chainstate(&ChainstateSnapshot)` signature; shell assembles from coins/`chain_meta` (D-02/D-07).
- Combine Task 1 and Task 2 into one feat commit under full pre-commit verify (matches Phases 140–145).
- Fix wallet-rescan bench as Rule 3 so verify can pass after the has_block gate.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] BlockHash Display in fail message**
- **Found during:** Task 1 (commit verify)
- **Issue:** `format!("{}")` on `BlockHash` does not compile (`Display` missing).
- **Fix:** Use `{:?}` while keeping the `missing block payload` substring.
- **Files modified:** `packages/open-bitcoin-node/src/sync/wallet_rescan.rs`
- **Verification:** Compiles under clippy/`verify.sh`
- **Committed in:** `8c5fec01`

**2. [Rule 3 - Blocking] Wallet-rescan bench still planted leftover-only truth**
- **Found during:** Task 1 (commit verify)
- **Issue:** `open-bitcoin-bench` `wallet-rescan.runtime-rescan` saved leftover snapshot without coins/`save_block`, so the new has_block gate failed the bench suite inside `verify.sh`.
- **Fix:** Seed coins via `seed_coins_from_snapshot` and `save_block` for every active_chain position.
- **Files modified:** `packages/open-bitcoin-bench/src/cases/wallet_rescan.rs`
- **Verification:** `cargo test -p open-bitcoin-bench --lib cases::tests::stateful_core_cases_are_repeatable` passed; full verify passed on commit.
- **Committed in:** `8c5fec01`

**3. [Rule 3 - Blocking] Combined Task 1 and Task 2 into one feat commit**
- **Found during:** Task execution under sequential hooks
- **Issue:** Pre-commit runs full `bash scripts/verify.sh` (~11–12 min). A RED-only or Task-1-without-test-updates commit cannot pass after the has_block gate, and split commits repeatedly burned full verify cycles on fmt/clippy/bench regressions.
- **Fix:** Ship production cutover, runtime tests, and bench plant in one hook-passing feat commit (same pattern as Phases 140–145 RED+GREEN combines).
- **Files modified:** all Task 1/2 files listed above
- **Verification:** Commit `8c5fec01` completed with hooks
- **Committed in:** `8c5fec01`

---

**4. [Rule 2 - Missing Critical] Leave SNAP-01 Pending until phase verification**
- **Found during:** Plan metadata commit
- **Issue:** Marking SNAP-01 Complete in REQUIREMENTS.md without lifecycle-valid VERIFICATION.md failed `check-active-milestone-verification-traceability`.
- **Fix:** Revert SNAP-01 to Pending; keep SUMMARY `requirements-completed` empty (Phases 140–145 pattern).
- **Files modified:** `.planning/REQUIREMENTS.md`, `146-01-SUMMARY.md`
- **Verification:** Will confirm on docs commit verify
- **Committed in:** `46a83ccd`

---

**Total deviations:** 4 auto-fixed (1 bug, 2 blocking, 1 missing-critical)
**Impact on plan:** Required for compile correctness and verify passage; no prune/have-pruned scope creep.

## Issues Encountered

- Multiple verify iterations failed on Display formatting, clippy `collapsible_if` (let-chain form required), rustfmt import layout, and the bench fixture before the combined commit succeeded.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Ready for Plan 02 (remaining Phase 146 seams such as durable RPC rescan seed path if planned).
- Leftover `"snapshot"` blob still on disk by design (D-05); unlink/have-pruned stay deferred to later phases.

## Self-Check: PASSED

- FOUND: `.planning/phases/146-wallet-leftover-snapshot-cutover/146-01-SUMMARY.md`
- FOUND: commit `8c5fec01`
- FOUND: `fn wallet_scan_chainstate_snapshot` in coins.rs
- FOUND: zero `load_chainstate_snapshot` in wallet_rescan.rs
- FOUND: disagreeing leftover and missing-payload tests

---
*Phase: 146-wallet-leftover-snapshot-cutover*
*Completed: 2026-09-22*
