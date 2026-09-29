---
phase: 150-operator-prune-surfaces-and-evidence
plan: "05"
subsystem: rpc
tags: [prune, rpc, locks, pruneblockchain, parity-breadcrumbs]

requires:
  - phase: 150-02
    provides: prune mode stored on the network handle
  - phase: 150-03
    provides: durable prune lock map under prune_locks
  - phase: 150-04
    provides: ManualPruneSurface on the operator status snapshot
provides:
  - RPC listprunelocks, setprunelock, and clearprunelock
  - RPC pruneblockchain that refuses disabled and keep-window targets before delete
  - Production caller of flush_applying_prune_plan
affects:
  - 150-06 dashboard read of locks and the last manual outcome
  - 150-07 CLI routing onto these RPC methods

tech-stack:
  added: []
  patterns:
    - "Lock replace writes the whole name-ordered map with SyncAll before success"
    - "pruneblockchain checks disabled mode first, returns 0 for height zero, and returns GetPruneHeight or -1 after flush"

key-files:
  created:
    - packages/open-bitcoin-rpc/src/method/prune.rs
    - packages/open-bitcoin-rpc/src/context/prune.rs
    - packages/open-bitcoin-rpc/src/dispatch/prune/tests.rs
  modified:
    - packages/open-bitcoin-rpc/src/dispatch/prune.rs
    - packages/open-bitcoin-rpc/src/method.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/prune_flush.rs
    - docs/parity/source-breadcrumbs.json

key-decisions:
  - "Tests and implementation ship in one hook-passing feat commit per task because pre-commit runs verify.sh"
  - "Disabled pruneblockchain is refused before height zero, matching Knots order"
  - "A legal prune on the memory chainstate fixture returns -1 because that sink reports AlreadyAbsent and GetPruneHeight stays empty"
  - "OPER-02 and LOCK-02 stay pending until lifecycle-valid Phase 150 verification"

patterns-established:
  - "Clear of a missing lock name returns success false and does not call sync_prune_locks"
  - "The RPC client serializes the new methods; the operator prune subcommand stays in plan 07"

requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 150-2026-09-28T17-08-23
generated_at: 2026-09-29T14:05:00Z

duration: 1h 37m
completed: 2026-09-29
---

# Phase 150 Plan 05: Manual Prune and Lock Commands Summary

**RPC lists, replaces, and clears durable prune locks, and `pruneblockchain` refuses a disabled node or a keep-window target before any delete**

## Performance

- **Duration:** 1h 37m
- **Started:** 2026-09-29T12:27:42Z
- **Completed:** 2026-09-29T14:05:00Z
- **Tasks:** 2
- **Files modified:** 21

## Accomplishments

- `listprunelocks` on an empty record returns `[]`. Setting `"wallet"` at 10..=20 and then 12..=18 leaves one row, and that row is still there after the store reopens.
- `clearprunelock` of a present name returns `{ "success": true }`. A missing name returns `{ "success": false }` and does not write. An empty name, a reversed range, and `height_last > u32::MAX - 10` return `InvalidParameter` and leave the saved map unchanged.
- Disabled `pruneblockchain` returns `MiscError` (`Cannot prune blocks because node is not in prune mode.`) and `load_have_pruned` stays false. A target inside the 288-block keep window returns `InvalidParameter` and the cached tip payload stays. Height `0` returns JSON `0`. A legal height and automatic mode both return `-1` on the memory fixture, which is `GetPruneHeight` of `None`, not the info-field plus one.

## Task Commits

Each task was committed atomically:

1. **Task 1: List, set, and clear durable prune locks** - `5c6e1c4e` (feat)
2. **Task 2: Request manual prune** - `f1f6c7fe` (feat)

## Files Created/Modified

- `packages/open-bitcoin-rpc/src/method/prune.rs` - Request and lock-row types for the four methods
- `packages/open-bitcoin-rpc/src/context/prune.rs` - Lock sync, prune-after height, and the flush wrapper
- `packages/open-bitcoin-rpc/src/dispatch/prune.rs` - `pruneblockchain` plus list, replace, and clear
- `packages/open-bitcoin-rpc/src/dispatch/prune/tests.rs` - Replace, clear-missing, refusals, and keep-window coverage
- `packages/open-bitcoin-rpc/src/dispatch/prune/status.rs` - Status reads the last manual outcome
- `packages/open-bitcoin-node/src/network/runtime_authority/prune_flush.rs` - `flush_applying_prune_plan` is a public production caller
- `packages/open-bitcoin-node/src/storage/fjall_store/prune/records.rs` - `sync_prune_locks` is public for the RPC caller
- `docs/parity/source-breadcrumbs.json` - `rpc-prune-commands` cites `blockchain.cpp` and `blockstorage.h`

## Decisions Made

- Each task is one feat commit. A failing-test-only commit cannot pass pre-commit `bash scripts/verify.sh`.
- Disabled mode is checked before the height-zero return, matching Knots `pruneblockchain`. Height zero returns JSON `0` only after that check, and it does not call `plan_manual_prune`.
- A legal plan on the RPC memory chainstate returns `-1`. That store's paired unlink reports `AlreadyAbsent`, so `GetPruneHeight` stays empty. The return is not the plus-one info field, which would be `0`.
- OPER-02 and LOCK-02 stay pending. Traceability rejects a Complete flip before `150-VERIFICATION.md` exists.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Publish `sync_prune_locks` for the RPC crate**
- **Found during:** Task 1 (List, set, and clear durable prune locks)
- **Issue:** The lock writer was `pub(crate)` on `FjallNodeStore`, so `context/prune.rs` could not sync a map.
- **Fix:** The method is `pub`. The lock name stays inside the value and is not a path.
- **Files modified:** `packages/open-bitcoin-node/src/storage/fjall_store/prune/records.rs`
- **Committed in:** `5c6e1c4e`

**2. [Rule 3 - Blocking] Keep the method catalog in step with `SupportedMethod`**
- **Found during:** Task 1 commit hook
- **Issue:** The documentation reconciliation checker still required 23 serde names and omitted the lock methods.
- **Fix:** The catalog, README, and checker now include the baseline lock methods and the `clearprunelock` extension. Task 2 added `pruneblockchain` and moved the pin to 27 names.
- **Files modified:** `docs/parity/catalog/rpc-cli-config.md`, `README.md`, `scripts/check-current-documentation-reconciliation.ts`, `scripts/check-current-documentation-reconciliation.test.ts`
- **Committed in:** `5c6e1c4e` and `f1f6c7fe`

**3. [Rule 3 - Blocking] Serialize the new methods in the baseline RPC client**
- **Found during:** Task 1 commit hook
- **Issue:** `open-bitcoin-cli` matches every `MethodCall`. The new variants failed the workspace check.
- **Fix:** The client forwards the JSON bodies. This is not an `OperatorCommand::Prune` route. Plan 07 owns that command.
- **Files modified:** `packages/open-bitcoin-cli/src/client.rs`
- **Committed in:** `5c6e1c4e` and `f1f6c7fe`

**4. [Rule 3 - Blocking] Re-export the timestamp resolver from the chainstate crate root**
- **Found during:** Task 2
- **Issue:** `resolve_manual_prune_argument` was public on the prune module and not on the crate root the RPC shell already uses.
- **Fix:** The crate root re-exports the argument types and the resolver.
- **Files modified:** `packages/open-bitcoin-chainstate/src/lib.rs`
- **Committed in:** `f1f6c7fe`

**5. [Rule 1 - Bug] Pad coinbase height script numbers whose high bit is set**
- **Found during:** Task 2 keep-window test
- **Issue:** The RPC chain fixture encoded height `128` as a negative script number, so `connect_local_block` returned `bad-cb-height` before the tip could reach the regtest prune-after height of 1000.
- **Fix:** A high bit on the last encoded byte gains a trailing `0x00`, matching the script-number sign rule. Heights `0..=127` stay the same encoding.
- **Files modified:** `packages/open-bitcoin-rpc/src/dispatch/tests/chain_fixtures.rs`
- **Committed in:** `f1f6c7fe`

**6. [Rule 2 - Missing Critical] Store the last manual outcome on the RPC context**
- **Found during:** Task 2
- **Issue:** Plan 04 hard-coded `ManualPruneSurface::None`. The status read had no writer, and `runtime_authority.rs` had to stay at 626 lines.
- **Fix:** The context records `Height` or `Refused`, and `operator_prune_status` reads that field. The dashboard still has no delete control.
- **Files modified:** `packages/open-bitcoin-rpc/src/context.rs`, `packages/open-bitcoin-rpc/src/context/network.rs`, `packages/open-bitcoin-rpc/src/dispatch/prune/status.rs`
- **Committed in:** `f1f6c7fe`


**Total deviations:** 6 auto-fixed (4 blocking, 1 missing critical, 1 bug)
**Impact on plan:** Lock replace, clear-by-name, and the manual-prune refusals are unchanged. The extra catalog and client arms keep the existing exhaustive RPC registry compiling.

## Issues Encountered

None

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Ready for 150-06. Locks and the last manual outcome are on the status snapshot the dashboard will read. Plan 07 owns `open-bitcoin prune`. OPER-02 and LOCK-02 stay pending until phase verification.

## Self-Check: PASSED

- FOUND: packages/open-bitcoin-rpc/src/method/prune.rs
- FOUND: packages/open-bitcoin-rpc/src/context/prune.rs
- FOUND: packages/open-bitcoin-rpc/src/dispatch/prune/tests.rs
- FOUND: 5c6e1c4e
- FOUND: f1f6c7fe

---
*Phase: 150-operator-prune-surfaces-and-evidence*
*Completed: 2026-09-29*
