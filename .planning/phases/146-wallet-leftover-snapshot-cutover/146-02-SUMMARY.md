---
phase: 146-wallet-leftover-snapshot-cutover
plan: "02"
subsystem: wallet
tags: [wallet-rescan, durable-rpc, coins, leftover-snapshot, has_block, SNAP-01]

requires:
  - phase: 146-wallet-leftover-snapshot-cutover
    provides: wallet_scan_chainstate_snapshot and WalletRescanRuntime has_block gate
  - phase: 141-durable-fjall-coins-adapter
    provides: seed_coins_from_snapshot and durable coins truth
  - phase: 143-honest-stored-block-availability
    provides: has_block payload-present probe
provides:
  - Durable ManagedRpcContext seeds MemoryChainstateStore from wallet_scan_chainstate_snapshot
  - Durable named-wallet rescanblockchain has_block fail-closed gate
  - durable_rescan_ignores_disagreeing_leftover_snapshot regression
affects:
  - 146-wallet-leftover-snapshot-cutover
  - 147-prune-policy

tech-stack:
  added: []
  patterns:
    - "Durable RPC open authority = wallet_scan_chainstate_snapshot; never leftover load or hydrate leftover-meta tip"
    - "Durable RPC rescanblockchain gates every in-range height with has_block before rescan_chainstate"

key-files:
  created: []
  modified:
    - packages/open-bitcoin-rpc/src/context/network.rs
    - packages/open-bitcoin-rpc/src/context/rescan.rs
    - packages/open-bitcoin-rpc/src/context/tests.rs
    - packages/open-bitcoin-rpc/src/context/tests/construction.rs
    - scripts/check-phase135-snapshot-recovery.ts
    - docs/metrics/lines-of-code.md

key-decisions:
  - "Combined Task 1 and Task 2 into one hook-passing feat commit because pre-commit runs full verify.sh"
  - "Durable connect_local_block persists block payloads so existing RPC rescan tests stay coherent under has_block"
  - "Phase 135 startup checker now requires wallet_scan_chainstate_snapshot on from_runtime_config_with_store"

patterns-established:
  - "Pattern: durable RPC MemoryChainstateStore seed uses Plan 01 wallet_scan helper, not hydrate leftover-meta fallback"
  - "Pattern: durable named-wallet rescan marks job Failed and returns missing block payload on absent payload"

requirements-completed: []  # SNAP-01 stays Pending until lifecycle-valid phase verification
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 146-2026-09-21T20-57-36
generated_at: 2026-09-22T03:18:54Z

duration: 70min
completed: 2026-09-22
---

# Phase 146 Plan 02: Durable RPC Rescan Leftover-Seed Cutover Summary

**Durable RPC now seeds chain truth from coins + `chain_meta` and fails closed on missing block payloads, so leftover `"snapshot"` bytes cannot drive operator `rescanblockchain` tip or balances.**

## Performance

- **Duration:** 70 min
- **Started:** 2026-09-22T02:07:59Z
- **Completed:** 2026-09-22T03:18:54Z
- **Tasks:** 2
- **Files modified:** 6

## Accomplishments

- Replaced durable RPC `load_chainstate_snapshot_with_confirmation_migration` seed with `wallet_scan_chainstate_snapshot`.
- Gated durable named-wallet `rescan_wallet_range` with `has_block`, failing closed with `missing block payload`.
- Persisted block payloads on durable `connect_local_block` so in-memory test connects remain payload-honest.
- Proved disagreeing leftover cannot change wallet tip/UTXOs; leftover blob remains on disk.
- Retargeted the Phase 135 startup checker and coins-seeded the mempool-before-chainstate regression.

## Task Commits

Each task was committed atomically:

1. **Task 1 + Task 2 (combined):** `d90dfb80` (feat) — durable RPC seed cutover, has_block gate, regressions, P135 checker, and related fixture fixes

**Plan metadata:** `dffa0c75` (docs: complete plan)

_Note: TDD RED/GREEN and the two plan tasks were combined into one hook-passing feat commit because pre-commit always runs `bash scripts/verify.sh`._

## Files Created/Modified

- `packages/open-bitcoin-rpc/src/context/network.rs` — coins-backed durable seed; durable `connect_local_block` saves payloads
- `packages/open-bitcoin-rpc/src/context/rescan.rs` — `has_block` fail-closed gate on durable named-wallet rescan
- `packages/open-bitcoin-rpc/src/context/tests/construction.rs` — disagreeing leftover and missing-payload regressions
- `packages/open-bitcoin-rpc/src/context/tests.rs` — leftover-empty-coins open; mempool recovery plants coins
- `scripts/check-phase135-snapshot-recovery.ts` — startup seed needle is `wallet_scan_chainstate_snapshot`
- `docs/metrics/lines-of-code.md` — hook-regenerated LOC freshness

## Decisions Made

- Combine Task 1 and Task 2 into one feat commit under full pre-commit verify (matches Phase 146 Plan 01).
- Persist payloads from durable `connect_local_block` so hermetic RPC rescan tests do not skip the has_block gate.
- Update Phase 135 checker to the SNAP-01 seed seam rather than restore leftover confirmation migration at RPC open.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Combined Task 1 and Task 2 into one feat commit**
- **Found during:** Task execution under sequential hooks
- **Issue:** Pre-commit runs full `bash scripts/verify.sh`; a RED-only or Task-1-without-tests commit cannot pass after the has_block gate.
- **Fix:** Ship production cutover, regressions, checker, and fixture fixes in one hook-passing feat commit.
- **Files modified:** all Task 1/2 files listed above
- **Verification:** Commit `d90dfb80` completed with hooks
- **Committed in:** `d90dfb80`

**2. [Rule 3 - Blocking] Durable connect_local_block now saves payloads**
- **Found during:** Task 1 (range-rescan coherence)
- **Issue:** Existing durable `rescanblockchain` tests connect blocks only into memory; has_block would fail closed without Fjall payloads.
- **Fix:** When wallet state is DurableNamedRegistry, `connect_local_block` also `save_block`.
- **Files modified:** `packages/open-bitcoin-rpc/src/context/network.rs`
- **Verification:** `rescanblockchain_accepts_ranges_and_records_partial_freshness` passes
- **Committed in:** `d90dfb80`

**3. [Rule 3 - Blocking] Phase 135 startup checker still required leftover confirmation migration**
- **Found during:** commit verify
- **Issue:** Checker asserted `load_chainstate_snapshot_with_confirmation_migration()?` inside `from_runtime_config_with_store`.
- **Fix:** Require `wallet_scan_chainstate_snapshot()?` and order against that needle; keep confirmation migration on schema-1 leftover migrate.
- **Files modified:** `scripts/check-phase135-snapshot-recovery.ts`
- **Verification:** `bun test scripts/check-phase135-snapshot-recovery.test.ts` — 84 pass
- **Committed in:** `d90dfb80`

**4. [Rule 1 - Bug] Mempool recovery regression still planted leftover-only chain truth**
- **Found during:** commit verify
- **Issue:** `managed_rpc_context_loads_chainstate_before_replaying_mempool_snapshot` used `save_chainstate_snapshot`, so post-cutover open seeded empty chainstate and dropped the tx as MissingParent.
- **Fix:** Plant via `seed_coins_from_snapshot` so wallet_scan seeds the parent UTXO set.
- **Files modified:** `packages/open-bitcoin-rpc/src/context/tests.rs`
- **Verification:** focused cargo test passes; full verify passed on commit
- **Committed in:** `d90dfb80`

**5. [Rule 2 - Missing Critical] Leave SNAP-01 Pending until phase verification**
- **Found during:** SUMMARY authorship
- **Issue:** Marking SNAP-01 Complete without lifecycle-valid VERIFICATION.md fails milestone traceability.
- **Fix:** Keep SUMMARY `requirements-completed` empty (Plan 01 pattern).
- **Files modified:** `146-02-SUMMARY.md`
- **Verification:** deferred to Plan 03 / phase verification
- **Committed in:** docs commit

---

**Total deviations:** 5 auto-fixed (1 bug, 3 blocking, 1 missing-critical)
**Impact on plan:** Required for verify passage and SNAP-01 coherence; no prune/have-pruned scope creep.

## Issues Encountered

- First commit attempts failed on Phase 135 checker, rustfmt layout, and the mempool chainstate plant before the combined feat commit succeeded.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Ready for Plan 03 (phase verification / remaining SNAP-01 closeout).
- Leftover `"snapshot"` blob still on disk by design (D-05); unlink/have-pruned stay deferred.

## Self-Check: PASSED

- FOUND: `.planning/phases/146-wallet-leftover-snapshot-cutover/146-02-SUMMARY.md`
- FOUND: commit `d90dfb80`
- FOUND: `wallet_scan_chainstate_snapshot` in network.rs durable seed
- FOUND: `has_block` and `missing block payload` in rescan.rs
- FOUND: `fn durable_rescan_ignores_disagreeing_leftover_snapshot` in construction.rs

---
*Phase: 146-wallet-leftover-snapshot-cutover*
*Completed: 2026-09-22*
