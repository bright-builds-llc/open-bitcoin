---
phase: 137-rpc-and-sanitized-operator-evidence
plan: 05
subsystem: status-rpc
tags: [mempool, status-snapshot, json-rpc, pressure, checkpoint, recovery, retry, admission]

requires:
  - phase: 137-rpc-and-sanitized-operator-evidence
    provides: Identifier-free MempoolResourcesGroup and MempoolFeeFloorsGroup on MempoolStatus
  - phase: 134-authoritative-cross-cache-lifecycle-integration
    provides: Lifecycle pressure_removals and retry_clears evidence
  - phase: 135-snapshot-schema-checkpointing-and-recovery
    provides: CheckpointEvidenceSnapshot and ManagedMempoolRecoverySummary
  - phase: 136-receive-independent-maintenance-and-transport-receipts
    provides: Unbroadcast retry eligibility and fanout queued_transactions
provides:
  - Identifier-free pressure, eviction, checkpoint, recovery, retry, and admission groups on MempoolStatus
  - Count-only recovery mapping that drops MempoolRecoveryRecord identities
  - ManagedNetworkOperatorSnapshot publication of those groups
  - openbitcoinnetworkstatus.mempool publication of retry and admission
affects:
  - 137-07 dashboard and CLI collector copy of snapshot groups
  - 137-08 metrics and structured logs for the same labels

tech-stack:
  added: []
  patterns:
    - Identifier-free snapshot groups assembled under the existing operator read guard
    - Retry and admission stay independent axes; Phase 105 deferred counters are not reused

key-files:
  created:
    - packages/open-bitcoin-node/src/network/operator_snapshot.rs
  modified:
    - packages/open-bitcoin-node/src/status/mempool_groups.rs
    - packages/open-bitcoin-node/src/status.rs
    - packages/open-bitcoin-node/src/status/tests/mempool_groups.rs
    - packages/open-bitcoin-node/src/network.rs
    - packages/open-bitcoin-node/src/network/types.rs
    - packages/open-bitcoin-rpc/src/dispatch/node.rs
    - packages/open-bitcoin-rpc/src/context/inbound_status.rs
    - packages/open-bitcoin-rpc/src/dispatch/tests/network_status.rs
    - docs/parity/source-breadcrumbs.json

key-decisions:
  - "Assemble operator snapshot groups in network/operator_snapshot.rs so network.rs stays under the 628-line gate."
  - "Publish attempted as 0 because leftover_unattempted is tick-local, not a stored live counter."
  - "Map emitted to the existing relay announce outcome counter; do not invent a second authority."
  - "retry.cleared copies lifecycle retry_clears; admission.cleared copies removed_members."
  - "relay_disabled is boolean-as-0/1; still-present local membership stays on admission.still_present."

patterns-established:
  - "Pattern 1: Recovery records stay authority-local; shared evidence copies counts only."
  - "Pattern 2: Checkpoint publishes dirty_generation_present, not the generation integer."

requirements-completed: [MPOBS-02, MPOBS-03]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 137-2026-08-19T21-52-42
generated_at: 2026-08-19T23:17:51Z

duration: 7min
completed: 2026-08-19
---

# Phase 137 Plan 05: Remaining Mempool Snapshot Groups Summary

**MempoolStatus now carries identifier-free pressure, eviction, checkpoint, recovery, retry, and admission groups, with recovery txids dropped and retry kept distinct from rebroadcast deferred.**

## Performance

- **Duration:** 7 min
- **Started:** 2026-08-19T23:10:36Z
- **Completed:** 2026-08-19T23:17:51Z
- **Tasks:** 2
- **Files modified:** 10

## Accomplishments

- Added pressure, eviction, checkpoint, and recovery groups with occupancy decay labels and count-only recovery mapping.
- Added retry and admission aggregates and published all new groups through `ManagedNetworkOperatorSnapshot` and `openbitcoinnetworkstatus`.
- Kept D-13/D-15/D-17/D-18 separations: retry ≠ deferred counters, pressure ≠ evicted, admission ≠ retry clear, no propagation fields.

## Task Commits

Each task was committed atomically:

1. **Task 1 RED: failing pressure/checkpoint/recovery tests** - `0ac1c2e5` (test)
2. **Task 1 GREEN: implement remaining D-10 groups** - `c6dc12b5` (feat)
3. **Task 2 RED: failing retry/admission tests** - `983c12ef` (test)
4. **Task 2 GREEN: publish retry and admission groups** - `f2003a7d` (feat)

**Plan metadata:** pending docs commit for this SUMMARY

_Note: TDD tasks produced RED then GREEN commits; no refactor commit was needed._

## Files Created/Modified

- `packages/open-bitcoin-node/src/status/mempool_groups.rs` - Remaining D-10 groups, retry/admission, and count-only mappers
- `packages/open-bitcoin-node/src/status.rs` - New `MempoolStatus` fields with unavailable serde defaults
- `packages/open-bitcoin-node/src/status/tests/mempool_groups.rs` - Named group, snapshot, and operator-eligible tests
- `packages/open-bitcoin-node/src/network/operator_snapshot.rs` - Identifier-free snapshot assembly
- `packages/open-bitcoin-node/src/network/types.rs` - Snapshot group fields and getters
- `packages/open-bitcoin-node/src/network.rs` - Moved snapshot assembly out of the 628-line file
- `packages/open-bitcoin-rpc/src/dispatch/node.rs` - Copy new groups onto status RPC mempool
- `packages/open-bitcoin-rpc/src/context/inbound_status.rs` - `operator_network()` accessor for group getters
- `packages/open-bitcoin-rpc/src/dispatch/tests/network_status.rs` - Status RPC retry/admission coverage
- `docs/parity/source-breadcrumbs.json` - Register `operator_snapshot.rs`

## Decisions Made

- New snapshot assembly lives in `network/operator_snapshot.rs` so `network.rs` stays at 612 lines.
- `attempted` is `0` with rustdoc because leftover-unattempted exists only on a maintenance tick outcome.
- `emitted` copies the existing relay announce outcome counter rather than inventing a drain authority.
- `retry.cleared` is `retry_clears`; `admission.cleared` is lifecycle `removed_members`.
- `relay_disabled` is 0/1 from `ManagedNetworkInfo.relay`; still-present local membership stays on admission.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Split snapshot assembly out of network.rs**
- **Found during:** Task 2 (publish groups)
- **Issue:** `network.rs` was already at the 628-line gate; adding snapshot fields inline would exceed it.
- **Fix:** Moved `operator_snapshot()` into `network/operator_snapshot.rs`.
- **Files modified:** `packages/open-bitcoin-node/src/network.rs`, `packages/open-bitcoin-node/src/network/operator_snapshot.rs`
- **Verification:** named retry and operator-snapshot tests pass
- **Committed in:** `f2003a7d`

**2. [Rule 3 - Blocking] Added operator snapshot accessor on RPC context**
- **Found during:** Task 2 (publish groups)
- **Issue:** `AuthoritativeOperatorSnapshot` did not expose the new group getters, and `inbound_status.rs` was not in the task file list.
- **Fix:** Added `operator_network()` so dispatch can copy pressure/eviction/checkpoint/recovery/retry/admission.
- **Files modified:** `packages/open-bitcoin-rpc/src/context/inbound_status.rs`
- **Verification:** `openbitcoinnetworkstatus_includes_retry_and_admission_groups` passes
- **Committed in:** `f2003a7d`

**3. [Rule 2 - Missing Critical] Registered operator_snapshot.rs in breadcrumb map**
- **Found during:** Task 2
- **Issue:** New first-party Rust files require a parity-breadcrumb mapping.
- **Fix:** Added the path to the existing node-network-adapter group.
- **Files modified:** `docs/parity/source-breadcrumbs.json`
- **Verification:** file-header breadcrumbs present; mapping lists the new path
- **Committed in:** `f2003a7d`

---

**Total deviations:** 3 auto-fixed (2 blocking, 1 missing critical)
**Impact on plan:** Required for compile, file-length policy, and breadcrumb policy. No scope creep.

## Issues Encountered

- `leftover_unattempted` is not stored on the live network, so `retry.attempted` is published as zero until a later phase owns a live counter.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan 07 can copy the new `mempool.*` groups into dashboard/CLI.
- Plan 08 can emit the same fixed labels on metrics and logs.
- CLI `MempoolStatus` literals remain for Plan 07 because Rust 1.94.1 default field values are unstable.

## Self-Check: PASSED

---
*Phase: 137-rpc-and-sanitized-operator-evidence*
*Completed: 2026-08-19*
