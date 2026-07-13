---
phase: 120-compact-download-timeout-and-misbehavior-runtime-bridge
plan: 02
subsystem: network
tags: [compact-blocks, misbehavior, disconnect, peer-policy, gov-02, blocktxn]

requires:
  - phase: 120-compact-download-timeout-and-misbehavior-runtime-bridge
    provides: Peer-scoped expire API and ManagedPeerNetwork timeout forwarder (Plan 01)
  - phase: 115-missing-transaction-round-trip-fallback-and-validation-handoff
    provides: Typed CompactBlockTxnHandleOutcome and CompactBlockTxnMisbehavior variants
  - phase: 96-peer-policy-runtime-bridge
    provides: MisbehaviorDecision / record_peer_policy_misbehavior bridge
provides:
  - DisconnectReason::CompactBlockMisbehavior and CompactBlockHeaderViolation
  - CompactBlockInitOutcome::Misbehavior for Invalid reconstruction (not Fallback)
  - compact_block_txn_actions escalation to PeerAction::Disconnect for typed Misbehavior/UnexpectedBlockHash
  - ManagedPeerNetwork peer-policy recording on compact disconnect
  - GOV-02 live-path proofs in compact_misbehavior_cases
affects:
  - 120-03 ReceivedBlock multi-peer volatile clear
  - Phase 121 DurableSyncRuntime metrics (explicitly untouched)

tech-stack:
  added: []
  patterns:
    - Pure compact outcomes escalate via PeerAction::Disconnect; shell records MisbehaviorDecision
    - NoMatchingInFlight stays empty-action suppress; collision Failed stays GetData Fallback

key-files:
  created:
    - packages/open-bitcoin-node/src/network/tests/compact_misbehavior_cases.rs
  modified:
    - packages/open-bitcoin-network/src/error.rs
    - packages/open-bitcoin-network/src/compact_download.rs
    - packages/open-bitcoin-network/src/peer/compact_download_state.rs
    - packages/open-bitcoin-network/src/compact_download/tests.rs
    - packages/open-bitcoin-network/src/peer/tests.rs
    - packages/open-bitcoin-network/src/compatibility.rs
    - packages/open-bitcoin-node/src/network/inventory.rs
    - packages/open-bitcoin-node/src/network/action_translation.rs
    - packages/open-bitcoin-node/src/network.rs
    - packages/open-bitcoin-node/src/network/tests.rs
    - docs/parity/source-breadcrumbs.json

key-decisions:
  - "Invalid init uses DisconnectReason::CompactBlockHeaderViolation → MisbehaviorKind::HeaderViolation"
  - "Typed blocktxn Misbehavior/UnexpectedBlockHash use CompactBlockMisbehavior → MalformedMessage"
  - "Peer-policy score set to discourage_threshold (50) with MisbehaviorResponse::Disconnect"
  - "NoMatchingInFlight remains empty Vec; ShortIdCollision Failed remains Fallback GetData"

patterns-established:
  - "Pattern: compact disconnect reasons drive both NetworkError mapping and MisbehaviorDecision kind"
  - "Pattern: live GOV-02 proofs assert Err disconnect path plus peer-policy evidence, not empty Ok silence"

requirements-completed: [GOV-02]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 120-2026-07-13T20-01-34
generated_at: 2026-07-13T22:59:58Z

duration: 43min
completed: 2026-07-13
---

# Phase 120 Plan 02: Compact Misbehavior Escalation Summary

**Escalated typed compact misbehavior and Invalid init from silent empty PeerAction lists to Knots-aligned Disconnect plus peer-policy evidence, while keeping NoMatchingInFlight suppressible and collision Failed as GetData Fallback.**

## Performance

- **Duration:** 43 min
- **Started:** 2026-07-13T22:16:41Z
- **Completed:** 2026-07-13T22:59:58Z
- **Tasks:** 2/2
- **Files modified:** 12

## Accomplishments

- Added `DisconnectReason::CompactBlockMisbehavior` / `CompactBlockHeaderViolation` with matching `NetworkError` variants and inventory mapping.
- Changed Invalid compact reconstruction init from Fallback to `CompactBlockInitOutcome::Misbehavior`; collision Failed stays Fallback.
- Rewrote `compact_block_txn_actions` so Misbehavior/UnexpectedBlockHash emit Disconnect; only NoMatchingInFlight stays empty.
- Wired `process_actions` Disconnect to `record_peer_policy_misbehavior` with MalformedMessage or HeaderViolation.
- Added ManagedPeerNetwork GOV-02 live proofs covering duplicate/OOB/invalid/stray/collision paths.

## Task Commits

Each task was committed atomically:

1. **Task 1: Escalate compact_block_*_actions + Invalid init** - `2614c779` (feat)
2. **Task 2: Peer-policy recording + ManagedPeerNetwork GOV-02 proofs** - `6f44ff0e` (feat)

**Plan metadata:** pending final docs commit

## Files Created/Modified

- `packages/open-bitcoin-network/src/error.rs` - CompactBlockMisbehavior/HeaderViolation disconnect + network errors
- `packages/open-bitcoin-network/src/compact_download.rs` - Invalid → Misbehavior init outcome
- `packages/open-bitcoin-network/src/peer/compact_download_state.rs` - action escalation + mut state accessor
- `packages/open-bitcoin-network/src/compact_download/tests.rs` - Invalid misbehavior vs collision Fallback split
- `packages/open-bitcoin-network/src/peer/tests.rs` - PeerManager disconnect/silent/Fallback proofs
- `packages/open-bitcoin-node/src/network/action_translation.rs` - peer-policy recording on compact disconnect
- `packages/open-bitcoin-node/src/network/inventory.rs` - disconnect_network_error mapping
- `packages/open-bitcoin-node/src/network/tests/compact_misbehavior_cases.rs` - live GOV-02 proofs
- `docs/parity/source-breadcrumbs.json` - `node-compact-misbehavior-escalation` group

## Decisions Made

- Split header Invalid init into `CompactBlockHeaderViolation` so peer-policy can record `HeaderViolation` separately from blocktxn `MalformedMessage`.
- Record explicit `MisbehaviorResponse::Disconnect` at `discourage_threshold` (50) rather than inventing a compact-only ban book.
- Exposed `compact_download_peer_state_mut` / `peer_manager_mut` so live DuplicateResponse proofs can clear the getblocktxn-in-flight flag without a parallel policy path.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - Critical] Separate HeaderViolation disconnect reason**
- **Found during:** Task 2
- **Issue:** A single `CompactBlockMisbehavior` reason could not distinguish Invalid-header init from blocktxn faults for peer-policy kind mapping.
- **Fix:** Added `DisconnectReason::CompactBlockHeaderViolation` (+ NetworkError) used by Invalid init actions.
- **Files modified:** `error.rs`, `compact_download_state.rs`, `inventory.rs`, `action_translation.rs`, tests
- **Verification:** `cargo test -p open-bitcoin-node compact_misbehavior`
- **Committed in:** `6f44ff0e`

**2. [Rule 3 - Blocking] Coverage on mut accessor**
- **Found during:** Task 2 commit (verify.sh coverage)
- **Issue:** `compact_download_peer_state_mut` was uncovered when only node tests called it through a dependency build.
- **Fix:** PeerManager phase120 tests now call the public mut accessor so network-crate coverage includes those lines.
- **Files modified:** `packages/open-bitcoin-network/src/peer/tests.rs`
- **Verification:** pre-commit `verify.sh` coverage gate
- **Committed in:** `6f44ff0e`

**3. [Rule 3 - Blocking] TDD RED/GREEN collapsed**
- **Found during:** Task 1
- **Issue:** Enum exhaustiveness for `CompactBlockInitOutcome` / `DisconnectReason` made a compile-failing RED commit impractical.
- **Fix:** Implemented production mapping and focused tests in one feat commit; behavior still matches `<behavior>` criteria.
- **Impact:** No behavior gap; commit history has one feat commit per task rather than test→feat pairs.

---

**Total deviations:** 3 auto-fixed (Rules 2–3)
**Impact on plan:** Necessary for GOV-02 kind discrimination and coverage/compile hygiene. No Phase 121 scope creep.

## Issues Encountered

- Pre-commit full `verify.sh` is long-running (~12m); Task 2 first commit attempt failed coverage before the mut-accessor test fix.

## User Setup Required

None.

## Next Phase Readiness

- Plan 03 can wire ReceivedBlock multi-peer volatile clear / remaining breadcrumb sweep.
- GOV-02 misbehavior escalation is closed on the ManagedPeerNetwork live path.
- DurableSyncRuntime `persist_metrics` / `block_relay_log_record` remain untouched (Phase 121).

## Self-Check: PASSED

- FOUND: `packages/open-bitcoin-node/src/network/tests/compact_misbehavior_cases.rs`
- FOUND: `PeerAction::Disconnect` escalation in `compact_download_state.rs`
- FOUND: Invalid init does not map to Fallback in `compact_download.rs`
- FOUND: commit `2614c779`
- FOUND: commit `6f44ff0e`
- FOUND: focused tests `compact_block` (network) and `compact_misbehavior` (node) passed
