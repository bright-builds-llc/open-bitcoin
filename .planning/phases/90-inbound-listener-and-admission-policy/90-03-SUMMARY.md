---
phase: 90-inbound-listener-and-admission-policy
plan: 03
subsystem: networking
tags: [rust, p2p, inbound, peer-manager, admission-policy]

requires:
  - phase: 90-01
    provides: Pure inbound listener and admission policy contracts
provides:
  - Typed inbound peer records stored on the existing PeerManager lifecycle
  - Self-connection nonce rejection evidence and distinct disconnect reason
  - ManagedPeerNetwork admission policy wiring before peer insertion
  - Bounded managed admission counters for accepted, rejected, reserved, cap, and duplicate outcomes
  - Count projection preserving separate inbound and outbound peer accounting
affects:
  - phase-90-runtime-listener
  - phase-90-peer-status
  - phase-90-support-evidence
  - phase-91-peer-permissions

tech-stack:
  added: []
  patterns:
    - Existing PeerManager lifecycle extended with optional typed inbound metadata
    - ManagedPeerNetwork fills pure admission policy inputs before insertion
    - Rejection evidence accumulated separately from baseline connected-peer counts

key-files:
  created: []
  modified:
    - packages/open-bitcoin-network/src/peer.rs
    - packages/open-bitcoin-network/src/peer/tests.rs
    - packages/open-bitcoin-node/src/network.rs
    - packages/open-bitcoin-node/src/network/tests.rs
    - packages/open-bitcoin-node/src/lib.rs
    - packages/open-bitcoin-network/src/error.rs
    - packages/open-bitcoin-network/src/compatibility.rs
    - packages/open-bitcoin-node/src/network/inventory.rs

key-decisions:
  - "Kept inbound handshake on the existing PeerAction version/verack path rather than adding an inbound-only handshake engine."
  - "Stored Phase 90 inbound metadata as optional PeerState evidence so outbound peers keep their existing shape."
  - "Filled admission counters, duplicate endpoint keys, peer IDs, and local nonce inside ManagedPeerNetwork before invoking the pure InboundAdmissionPolicy."
  - "Added a distinct self-connection disconnect/network error reason after self-check showed duplicate-version reporting would hide the stable self_connection cause."

patterns-established:
  - "PeerManager::add_inbound_peer remains a compatibility wrapper over typed inbound records."
  - "ManagedPeerNetwork::admit_inbound_peer returns the pure InboundAdmissionDecision and only inserts admitted records."
  - "ManagedInboundAdmissionInfo provides bounded counters for later status/support rendering."

requirements-completed: [INB-03, INB-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 90-2026-06-25T04-23-47
generated_at: 2026-06-25T06:59:41Z

duration: 40 min
completed: 2026-06-25
---

# Phase 90 Plan 03: Peer Manager Inbound Admission Summary

**Typed inbound peer lifecycle and managed cap-aware admission over the existing peer manager**

## Performance

- **Duration:** 40 min
- **Started:** 2026-06-25T06:19:30Z
- **Completed:** 2026-06-25T06:59:41Z
- **Tasks:** 2
- **Files modified:** 8

## Accomplishments

- Extended `PeerState` with optional inbound record metadata and self-connection rejection evidence.
- Added `PeerManager::add_inbound_peer_record` while preserving `add_inbound_peer` as a compatibility wrapper.
- Kept inbound version/verack negotiation on the existing `PeerAction::Send` flow for `version`, `wtxidrelay`, `verack`, and `sendheaders`.
- Added `ManagedPeerNetwork::admit_inbound_peer` to apply `InboundAdmissionPolicy` before peer insertion.
- Added `ManagedInboundAdmissionInfo` with bounded accepted/rejected/reserved/cap/duplicate/self/shutdown counters.
- Preserved baseline `ManagedNetworkInfo` count projection with separate inbound and outbound counts.

## Task Commits

1. **Task 1 RED: peer inbound state tests** - `c584e23` (test)
2. **Task 1 GREEN: typed inbound peer state** - `ebec21a` (feat)
3. **Task 2 RED: managed admission tests** - `96597f7` (test)
4. **Task 2 GREEN: managed inbound admission** - `5c365c9` (feat)
5. **Corrective fix: distinct self-connection reason** - `76a62cd` (fix)

## Files Created/Modified

- `packages/open-bitcoin-network/src/peer.rs` - Stores typed inbound records, exposes endpoint/peer/counter helpers, and rejects inbound self-connections by nonce.
- `packages/open-bitcoin-network/src/peer/tests.rs` - Covers typed inbound record insertion, compatibility helper behavior, self-connection rejection, and inbound handshake actions.
- `packages/open-bitcoin-node/src/network.rs` - Applies admission policy before insertion and projects managed inbound admission evidence.
- `packages/open-bitcoin-node/src/network/tests.rs` - Covers admitted, cap-rejected, reserved-slot, duplicate, and inbound/outbound count behavior.
- `packages/open-bitcoin-node/src/lib.rs` - Re-exports managed network info and inbound admission evidence for downstream plans.
- `packages/open-bitcoin-network/src/error.rs` - Adds distinct self-connection disconnect and network error labels.
- `packages/open-bitcoin-network/src/compatibility.rs` - Classifies self-connection as a version-stage rejection in compatibility diagnostics.
- `packages/open-bitcoin-node/src/network/inventory.rs` - Converts self-connection disconnect actions into the matching network error.

## Decisions Made

- Reused `InboundPeerRecord` from Plan 01 instead of adding a second peer table.
- Counted only active inbound records in managed network projection; rejected candidates remain evidence, not baseline connections.
- Preserved outbound sync accounting by reading outbound counts into admission records without sharing inbound caps with outbound target logic.
- Added the self-connection reason to shared error handling because otherwise managed callers would receive a misleading duplicate-version error.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network inbound -- --nocapture` passed with 22 matching unit tests plus filtered integration/property binaries.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network self_connection -- --nocapture` passed with 2 matching unit tests plus filtered integration/property binaries.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node inbound -- --nocapture` passed with 6 matching unit tests.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node network_info -- --nocapture` passed with 1 matching unit test.
- `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-network --all-targets --all-features -- -D warnings` passed.
- `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-targets --all-features -- -D warnings` passed.
- `rg -n "noban|forcerelay|mempool propagation|whitebind|whitelist|NetPermission|compact block|address relay|ban" ...` returned no matches in the plan-owned node files.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Reported self-connections with a distinct disconnect reason**
- **Found during:** Summary self-check after Task 2
- **Issue:** Peer state recorded `self_connection`, but the disconnect action still used the existing duplicate-version reason, which would make managed callers see the wrong cause.
- **Fix:** Added `DisconnectReason::SelfConnection` and `NetworkError::SelfConnection`, mapped them through compatibility diagnostics and node disconnect conversion, and tightened the peer test to assert the exact reason.
- **Files modified:** `packages/open-bitcoin-network/src/error.rs`, `packages/open-bitcoin-network/src/compatibility.rs`, `packages/open-bitcoin-network/src/peer.rs`, `packages/open-bitcoin-network/src/peer/tests.rs`, `packages/open-bitcoin-node/src/network/inventory.rs`
- **Verification:** Re-ran network self-connection/inbound tests, node inbound/network-info tests, and network/node clippy.
- **Committed in:** `76a62cd`

**Total deviations:** 1 auto-fixed (1 bug)
**Impact on plan:** The fix is a narrow correctness change required for T-90-03-01. It does not add relay, permission, eviction, ban, or address-relay behavior.

## Issues Encountered

- Cargo commands intermittently waited on artifact/package locks while other Phase 90 executor work was active. Verification completed after waiting.
- Concurrent out-of-scope RPC config changes appeared during execution and were left unstaged/unmodified by this plan.

## Known Stubs

None - stub and placeholder scans found no matches in the files created or modified for this plan.

## Authentication Gates

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Ready for runtime listener and status/support plans. Downstream work can use `PeerState::maybe_inbound_record`, `PeerManager::inbound_admission_counters`, `ManagedPeerNetwork::admit_inbound_peer`, and `ManagedInboundAdmissionInfo` without changing baseline `getnetworkinfo` count semantics.

## Self-Check: PASSED

- Found all modified source and test files.
- Found `.planning/phases/90-inbound-listener-and-admission-policy/90-03-SUMMARY.md`.
- Found commits `c584e23`, `ebec21a`, `96597f7`, `5c365c9`, and `76a62cd`.

---

*Phase: 90-inbound-listener-and-admission-policy*
*Completed: 2026-06-25*
