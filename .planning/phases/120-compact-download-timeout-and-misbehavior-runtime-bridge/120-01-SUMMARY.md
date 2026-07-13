---
phase: 120-compact-download-timeout-and-misbehavior-runtime-bridge
plan: 01
subsystem: network
tags: [compact-blocks, timeout, getdata, peer-manager, managed-peer-network, rcn-07, gov-03]

requires:
  - phase: 115-missing-transaction-round-trip-fallback-and-validation-handoff
    provides: PeerManager::expire_compact_download_timeouts and COMPACT_BLOCK_DOWNLOAD_TIMEOUT_SECONDS
  - phase: 119-compact-receive-mempool-candidate-injection
    provides: live CompactBlock receive path and ManagedPeerNetwork test helpers
provides:
  - Peer-scoped PeerManager::expire_compact_download_timeouts returning Vec<(PeerId, PeerAction)>
  - ManagedPeerNetwork::expire_compact_download_timeouts forwarder that keeps Send GetData and records Timeout cleanup
  - receive_message / receive_sync_message caller-clocked compact timeout piggyback
  - ManagedPeerNetwork compact_timeout live-path proofs
affects:
  - 120-02 misbehavior escalation
  - 120-03 ReceivedBlock multi-peer volatile clear
  - DurableSyncRuntime targeted_outbound routing (pre-existing gap; not closed here)

tech-stack:
  added: []
  patterns:
    - Caller-clocked ManagedPeerNetwork expire forwarder mirroring TX clock injection without TX TransactionRelay filter
    - receive_* piggyback merge of peer-scoped timeout GetData into outbound / targeted_outbound

key-files:
  created:
    - packages/open-bitcoin-node/src/network/tests/compact_timeout_cases.rs
  modified:
    - packages/open-bitcoin-network/src/peer/compact_download_state.rs
    - packages/open-bitcoin-network/src/peer/tests.rs
    - packages/open-bitcoin-node/src/network/action_translation.rs
    - packages/open-bitcoin-node/src/network.rs
    - packages/open-bitcoin-node/src/network/tests.rs
    - packages/open-bitcoin-rpc/src/context/network.rs
    - docs/parity/source-breadcrumbs.json

key-decisions:
  - "PeerManager expire returns peer-scoped Vec<(PeerId, PeerAction)> like TX expire"
  - "ManagedPeerNetwork compact expire keeps PeerAction::Send; never copies TX TransactionRelay filter"
  - "receive_message now returns ManagedSyncMessageResult so other-peer timeout GetData is preserved"
  - "Timeout tick piggybacks on receive_* message timestamps; no Tokio timer"

patterns-established:
  - "Pattern: shell expire forwarder records CompactDownloadCleanupCause::Timeout with expired pair count"
  - "Pattern: merge same-peer expire messages into outbound and other-peer into targeted_outbound"

requirements-completed: [RCN-07, GOV-03]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 120-2026-07-13T20-01-34
generated_at: 2026-07-13T21:48:56Z

duration: 39min
completed: 2026-07-13
---

# Phase 120 Plan 01: Compact Download Timeout Runtime Bridge Summary

**Wired peer-scoped compact-download timeout expiry onto ManagedPeerNetwork so live receive traffic emits full-block GetData fallbacks and Timeout cleanup evidence.**

## Performance

- **Duration:** 39 min
- **Started:** 2026-07-13T21:09:54Z
- **Completed:** 2026-07-13T21:48:56Z
- **Tasks:** 2/2
- **Files modified:** 11

## Accomplishments

- Changed `PeerManager::expire_compact_download_timeouts` to return peer-scoped `(PeerId, PeerAction)` pairs and updated Phase 115 coverage.
- Added `ManagedPeerNetwork::expire_compact_download_timeouts` that translates `PeerAction::Send` into wire GetData and records `CompactDownloadCleanupCause::Timeout`.
- Piggybacked the expire tick on both `receive_message` and `receive_sync_message` using the message timestamp; preserved other-peer GetData via `ManagedSyncMessageResult`.
- Added `compact_timeout_cases` proving public expire, live-path GetData + Timeout evidence, other-peer preservation, and volatile-only cleanup.

## Task Commits

Each task was committed atomically:

1. **Task 1: Peer-scoped expire API + ManagedPeerNetwork forwarder** - `0a06b473` (feat)
2. **Task 2: receive_* piggyback tick + live-path timeout tests** - `7447cdfe` (feat)

**Plan metadata:** pending final docs commit

## Files Created/Modified

- `packages/open-bitcoin-network/src/peer/compact_download_state.rs` - peer-scoped expire return type
- `packages/open-bitcoin-network/src/peer/tests.rs` - Phase 115 expire assertion updated for peer pairs
- `packages/open-bitcoin-node/src/network/action_translation.rs` - ManagedPeerNetwork expire forwarder + Timeout evidence
- `packages/open-bitcoin-node/src/network.rs` - receive_* expire piggyback; `receive_message` returns `ManagedSyncMessageResult`
- `packages/open-bitcoin-node/src/network/tests/compact_timeout_cases.rs` - live-path timeout proofs
- `packages/open-bitcoin-node/src/network/tests.rs` (+ sibling test modules) - `.outbound` after receive_message API change
- `packages/open-bitcoin-rpc/src/context/network.rs` - maps receive_message result to same-peer outbound Vec
- `docs/parity/source-breadcrumbs.json` - `node-compact-download-timeout` breadcrumb group

## Decisions Made

- Kept `COMPACT_BLOCK_DOWNLOAD_TIMEOUT_SECONDS` at 60; no retune to Knots IBD stall math.
- Preferred changing `receive_message` to return `ManagedSyncMessageResult` over silently dropping other-peer timeout GetData.
- RPC `receive_network_message` continues exposing same-peer `Vec` outbound only; full peer-targeted routing at the RPC inbound shell remains outside this plan's scope (ManagedPeerNetwork path preserves pairs).

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] receive_message return-type callers**
- **Found during:** Task 2
- **Issue:** Changing `receive_message` / `receive_wire_message` to `ManagedSyncMessageResult` broke test bindings and RPC's `receive_network_message` Vec contract.
- **Fix:** Updated test call sites to use `.outbound`; RPC wrapper returns `.outbound` for same-peer encoding.
- **Files modified:** network test modules, `packages/open-bitcoin-rpc/src/context/network.rs`
- **Verification:** `cargo test -p open-bitcoin-node compact_timeout`; `cargo check -p open-bitcoin-rpc`
- **Committed in:** `7447cdfe`

---

**Total deviations:** 1 auto-fixed (Rule 3)
**Impact on plan:** Necessary to preserve other-peer GetData without leaving the tree uncompilable. No Phase 121 scope creep.

## Issues Encountered

- Pre-commit rustfmt rejected import order / assertion formatting in `compact_timeout_cases.rs`; fixed with `cargo fmt` and recommitted successfully.

## User Setup Required

None.

## Next Phase Readiness

- Plan 02 can escalate compact misbehavior beyond empty PeerAction mapping.
- Plan 03 can wire ReceivedBlock multi-peer volatile clear / remaining breadcrumb sweep.
- Note: DurableSyncRuntime still sends only `sync_result.outbound` today; routing `targeted_outbound` at the sync session layer remains a follow-up outside Plan 01.

## Self-Check: PASSED

- FOUND: `packages/open-bitcoin-node/src/network/tests/compact_timeout_cases.rs`
- FOUND: `packages/open-bitcoin-node/src/network/action_translation.rs` (`expire_compact_download_timeouts`)
- FOUND: commit `0a06b473`
- FOUND: commit `7447cdfe`
- FOUND: both receive paths call `expire_compact_download_timeouts`
- FOUND: focused tests `phase115_expire_compact_download_timeouts` and `compact_timeout` passed
