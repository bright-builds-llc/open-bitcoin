---
phase: 150-operator-prune-surfaces-and-evidence
plan: "04"
subsystem: rpc
tags: [prune, getblockchaininfo, status-snapshot, parity-breadcrumbs]

requires:
  - phase: 150-01
    provides: I/O-free getblockchaininfo quartet projection
  - phase: 150-02
    provides: prune mode stored on the network handle
  - phase: 150-03
    provides: durable prune locks and support counts
provides:
  - getblockchaininfo quartet with Knots presence rules
  - PruneOperatorStatus on RPC status and the shared CLI snapshot
  - Stopped collector mode from JSONC without inventing pruneheight 0
affects:
  - 150-05 pruneblockchain RPC
  - 150-06 dashboard and CLI status lines
  - 150-08 support-bundle prune section

tech-stack:
  added: []
  patterns:
    - "getblockchaininfo calls project_prune_status; disabled mode skips the active-chain walk"
    - "Live snapshot facts stay on PruneOperatorStatus, not inside chainstate_durability"

key-files:
  created:
    - packages/open-bitcoin-node/src/status/prune_operator.rs
    - packages/open-bitcoin-node/src/status/wallet_freshness.rs
    - packages/open-bitcoin-rpc/src/dispatch/prune/status.rs
    - packages/open-bitcoin-cli/src/operator/status/tests/prune_snapshot.rs
  modified:
    - packages/open-bitcoin-rpc/src/method/node.rs
    - packages/open-bitcoin-rpc/src/dispatch/node.rs
    - packages/open-bitcoin-node/src/status.rs
    - packages/open-bitcoin-cli/src/operator/status.rs
    - docs/parity/source-breadcrumbs.json

key-decisions:
  - "Tests and implementation ship in one hook-passing feat commit per task because pre-commit runs verify.sh"
  - "Disabled getblockchaininfo skips the completeness walk because the projection discards the height"
  - "The active-chain walk lives under dispatch/prune/status.rs so it can cite blockchain.cpp without duplicating the rpc-surface breadcrumb pattern"
  - "OPER-01 stays pending until lifecycle-valid Phase 150 verification"

patterns-established:
  - "maybe_last_prune_height serializes as last_prune_height and is omitted when None"
  - "A stopped collector reads parse_prune_arg from loaded JSONC and marks height, locks, manual prune, and counts Unavailable"

requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 150-2026-09-28T17-08-23
generated_at: 2026-09-29T12:05:00Z

duration: 2h 16m
completed: 2026-09-29
---

# Phase 150 Plan 04: Quartet on RPC and the Status Snapshot Summary

**`getblockchaininfo` publishes the Knots prune quartet, and the shared status snapshot carries live locks, the manual outcome, and support counts beside that mode**

## Performance

- **Duration:** 2h 16m
- **Started:** 2026-09-29T09:49:07Z
- **Completed:** 2026-09-29T12:05:00Z
- **Tasks:** 2
- **Files modified:** 40

## Accomplishments

- Disabled `getblockchaininfo` adds only `pruned: false`. Manual-only with nothing pruned reports `pruneheight` 0 and `automatic_pruning: false` and omits the byte target. Automatic 550 MiB reports `576716800`. A hole at height 11 under a complete tip reports `pruneheight` 12.
- `PruneOperatorStatus` sits on RPC status and `OpenBitcoinStatusSnapshot`. Support counts omit `last_prune_height` when none has been recorded and emit it for `Some(40)`.
- A stopped collector with JSONC `prune: 550` sets `pruned` true, automatic pruning, and the byte target, and leaves `pruneheight`, locks, the manual outcome, and counts `Unavailable`. It does not report height 0. Missing config and `prune: 0` stay disabled.

## Task Commits

Each task was committed atomically:

1. **Task 1: Serialize the quartet on getblockchaininfo** - `fc4b5cc2` (feat)
2. **Task 2: Put operator prune facts on the shared snapshot** - `dc2a717d` (feat)

## Files Created/Modified

- `packages/open-bitcoin-rpc/src/method/node.rs` - Quartet fields on `getblockchaininfo` and `prune` on network status
- `packages/open-bitcoin-rpc/src/dispatch/node.rs` - Both blockchain info constructors call `project_prune_status`
- `packages/open-bitcoin-rpc/src/dispatch/prune/status.rs` - Active-chain completeness walk and the live operator object
- `packages/open-bitcoin-node/src/status/prune_operator.rs` - Snapshot prune facts, including the renamed last height
- `packages/open-bitcoin-node/src/status/wallet_freshness.rs` - Wallet types moved so `status.rs` stays at or below 628 lines
- `packages/open-bitcoin-node/src/status.rs` - `OpenBitcoinStatusSnapshot.prune`
- `packages/open-bitcoin-cli/src/operator/status.rs` - Live copy and stopped JSONC projection
- `packages/open-bitcoin-cli/src/operator/status/tests/prune_snapshot.rs` - Stopped 550 MiB collector does not invent height 0
- `docs/parity/source-breadcrumbs.json` - `rpc-prune-status` plus the none group for the moved snapshot files

## Decisions Made

- Each task is one feat commit. A failing-test-only commit cannot pass pre-commit `bash scripts/verify.sh`.
- Disabled mode does not walk the active chain. The projection clears the height anyway.
- The walk lives in `dispatch/prune/status.rs`. A file directly under `dispatch/` would also match the `rpc-surface` pattern and fail the breadcrumb checker.
- OPER-01 stays pending. Traceability rejects a Complete flip before `150-VERIFICATION.md` exists.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Keep the completeness walk off the `dispatch/*.rs` breadcrumb pattern**
- **Found during:** Task 1 (Serialize the quartet on getblockchaininfo)
- **Issue:** `dispatch/node.rs` was already 616 lines with the walk. A sibling `dispatch/prune_status.rs` matched both `rpc-surface` and a new `rpc-prune-status` group.
- **Fix:** The walk and the live snapshot fill live in `dispatch/prune/status.rs`, registered only as `rpc-prune-status`. `dispatch/prune.rs` stays on the existing `rpc-surface` group.
- **Files modified:** `packages/open-bitcoin-rpc/src/dispatch/prune.rs`, `packages/open-bitcoin-rpc/src/dispatch/prune/status.rs`
- **Committed in:** `dc2a717d` (the walk first landed inside `dispatch/node.rs` in `fc4b5cc2`, then moved in the snapshot commit)

**2. [Rule 3 - Blocking] Let RPC tests drop payload and undo on the node handle**
- **Found during:** Task 1 hole test
- **Issue:** `#[cfg(test)]` methods on `open-bitcoin-node` are not compiled when `open-bitcoin-rpc` tests depend on that crate.
- **Fix:** `forget_block_payload_and_undo_for_test` stays in the library build with `allow(dead_code)` outside tests. The hole test uses it to remove height 11 bytes without scanning for the minimum stored height.
- **Files modified:** `packages/open-bitcoin-node/src/network/runtime_authority/prune_flush.rs`, `packages/open-bitcoin-node/src/chainstate.rs`
- **Committed in:** `fc4b5cc2`

**3. [Rule 3 - Blocking] Re-export the projection from the chainstate crate root**
- **Found during:** Task 1 compile
- **Issue:** `project_prune_status` and `get_prune_height` were public on `prune` but not on the chainstate crate root the RPC shell already uses.
- **Fix:** The crate root re-exports them with `PruneStatusProjection`.
- **Files modified:** `packages/open-bitcoin-chainstate/src/lib.rs`
- **Committed in:** `fc4b5cc2`

**4. [Rule 1 - Bug] Phase 127 key pins must include `prune`**
- **Found during:** Task 2 commit hook
- **Issue:** The composition checker and the network-status schema tests still required the pre-prune key list.
- **Fix:** Those expected lists insert `"prune"` and nothing else. Durability JSON is still forbidden from containing a pruned key.
- **Files modified:** `scripts/check-phase127-authoritative-network-state-unification.ts`, `packages/open-bitcoin-rpc/src/dispatch/tests/network_status_schema.rs`
- **Committed in:** `dc2a717d`

**5. [Rule 3 - Blocking] Keep the stopped-collector test under the file-length gate**
- **Found during:** Task 2 commit hook
- **Issue:** Adding the test to `snapshot.rs` made that file 632 lines.
- **Fix:** The test lives in `prune_snapshot.rs`. `snapshot.rs` stays at 575 lines.
- **Files modified:** `packages/open-bitcoin-cli/src/operator/status/tests/prune_snapshot.rs`, `packages/open-bitcoin-cli/src/operator/status/tests.rs`
- **Committed in:** `dc2a717d`

---

**Total deviations:** 5 auto-fixed (4 blocking, 1 bug)
**Impact on plan:** The quartet and the stopped 550 MiB rule are unchanged. The extra files exist so breadcrumbs and the 628-line gate stay green.

## Issues Encountered

None

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Ready for 150-05. `getblockchaininfo` and the status snapshot expose configured mode. Manual prune still records `ManualPruneSurface::None`. Dashboard rows and the support markdown section remain later plans. OPER-01 stays pending until phase verification.

## Self-Check: PASSED

- FOUND: packages/open-bitcoin-rpc/src/dispatch/prune/status.rs
- FOUND: packages/open-bitcoin-node/src/status/prune_operator.rs
- FOUND: packages/open-bitcoin-cli/src/operator/status/tests/prune_snapshot.rs
- FOUND: fc4b5cc2
- FOUND: dc2a717d

---
*Phase: 150-operator-prune-surfaces-and-evidence*
*Completed: 2026-09-29*
