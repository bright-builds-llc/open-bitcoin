---
phase: 101-transaction-inventory-identity-and-download-scheduling
plan: 02
subsystem: network
tags: [rust, transaction-relay, peer-manager, managed-network, getdata]
requires:
  - 101-01
provides:
  - scheduler-backed PeerManager transaction request state
  - typed transaction relay peer actions for inv, notfound, tx, timeout, and disconnect paths
  - managed network translation from request actions to targeted getdata messages
  - duplicate suppression, fallback, cleanup, and compatibility tests
affects:
  - 101-transaction-inventory-identity-and-download-scheduling
  - open-bitcoin-network
  - open-bitcoin-node
tech-stack:
  added: []
  patterns:
    - pure scheduler owned by PeerManager and driven by caller-provided timestamps
    - typed transaction relay actions translated at the managed network adapter
    - split managed action translation helper module for file-length compliance
key-files:
  created:
    - packages/open-bitcoin-node/src/network/action_translation.rs
  modified:
    - packages/open-bitcoin-network/src/peer.rs
    - packages/open-bitcoin-network/src/peer/inventory_state.rs
    - packages/open-bitcoin-network/src/peer/tests.rs
    - packages/open-bitcoin-network/src/peer/transaction_relay.rs
    - packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs
    - packages/open-bitcoin-network/src/peer/transaction_relay/tests/scheduler_cases/edge_cases.rs
    - packages/open-bitcoin-node/src/network.rs
    - packages/open-bitcoin-node/src/network/tests.rs
    - docs/parity/source-breadcrumbs.json
    - docs/metrics/lines-of-code.md
key-decisions:
  - "Replace PeerState transaction request sets with one PeerManager-owned TxDownloadScheduler."
  - "Keep block request state separate from transaction relay scheduling."
  - "Translate only request and fallback actions into wire getdata messages; suppressions and cleanup stay internal evidence."
  - "Preserve the existing ReceivedTransaction admission bridge until Phase 102."
patterns-established:
  - "PeerAction::TransactionRelay carries typed scheduler output across open-bitcoin-network."
  - "ManagedSyncMessageResult.targeted_outbound carries fallback getdata messages for alternate peers."
  - "Disconnect, notfound, and timeout cleanup can schedule alternate-peer downloads without socket or mempool side effects in the scheduler."
requirements-completed: [INV-01, INV-02, INV-03, INV-04, DL-01, DL-02]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 101-2026-06-29T21-00-59
generated_at: 2026-06-30T00:42:40Z
duration: 1h 10m
completed: 2026-06-30
---

# Phase 101 Plan 02: PeerManager and Managed Network Integration Summary

**Scheduler-backed transaction request state is now wired through PeerManager and translated by the managed network bridge without adding mempool admission semantics.**

## Performance

- **Duration:** 1h 10m
- **Started:** 2026-06-29T23:32:21Z
- **Completed:** 2026-06-30T00:42:40Z
- **Tasks:** 2
- **Files modified:** 13

## Accomplishments

- Replaced bare PeerState `requested_txids` and `requested_wtxids` transaction request sets with a single PeerManager-owned `TxDownloadScheduler`.
- Added `PeerAction::TransactionRelay(TxDownloadAction)` and PeerManager APIs for recent rejects, request snapshots, timeout expiry, and disconnect cleanup.
- Routed transaction `inv`, `getdata`, `notfound`, and `tx` paths through typed `TxRelayId` identities while preserving separate block request accounting.
- Added managed network translation from request and fallback actions to `WireNetworkMessage::GetData`, including targeted alternate-peer outbound messages.
- Covered txid and wtxid requests, identity mismatch, duplicate suppression, recent rejects, already-have suppression, timeout fallback, `notfound` fallback, disconnect fallback, and received-transaction cleanup.

## Task Commits

1. **Task 1 and Task 2: Wire scheduler-backed transaction relay state through PeerManager and ManagedPeerNetwork** - `a512cf47` (`feat`)

## Files Created/Modified

- `packages/open-bitcoin-node/src/network/action_translation.rs` - Private helper translating request-capable transaction relay actions into targeted `getdata` messages.
- `packages/open-bitcoin-node/src/network.rs` - Managed result shape, action processing, timeout expiry, and disconnect cleanup integration.
- `packages/open-bitcoin-node/src/network/tests.rs` - Managed network request, duplicate suppression, timeout fallback, `notfound` fallback, and disconnect fallback tests.
- `packages/open-bitcoin-network/src/peer.rs` - PeerManager scheduler ownership, transaction relay actions, request snapshots, recent rejects, expiry, and cleanup APIs.
- `packages/open-bitcoin-network/src/peer/inventory_state.rs` - Typed `inv`, `getdata`, `notfound`, and received transaction scheduler integration.
- `packages/open-bitcoin-network/src/peer/tests.rs` - PeerManager integration tests for typed transaction relay behavior.
- `packages/open-bitcoin-network/src/peer/transaction_relay.rs` - Action helper coverage for peer ids and request inventory.
- `packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs` - Cleanup and request-state behavior exercised by PeerManager integration.
- `packages/open-bitcoin-network/src/peer/transaction_relay/tests/scheduler_cases/edge_cases.rs` - Coverage guard for cleanup and fallback branches.
- `docs/parity/source-breadcrumbs.json` - Registered the new managed network action translation helper.
- `docs/metrics/lines-of-code.md` - Refreshed tracked generated LOC report from repo hooks.

## Decisions Made

- Transaction download state is centralized in `PeerManager` so request pressure and fallback behavior use one scheduler-owned source of truth.
- `PeerAction::ReceivedTransaction(Transaction)` remains in place for the existing admission bridge; Phase 101 only changes download identity and scheduling behavior.
- Managed network translation is intentionally narrow: only `RequestGetData` and `FallbackRequest` leave the network adapter as wire messages.
- Fallback requests are returned as `(PeerId, WireNetworkMessage)` pairs so alternate-peer scheduling does not depend on the peer that triggered timeout, `notfound`, or disconnect cleanup.

## Verification

- `cargo fmt --manifest-path packages/Cargo.toml --all`
- `timeout 120s cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network --lib peer_manager_transaction_relay -- --nocapture`
- `timeout 120s cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network --lib transaction_relay -- --nocapture`
- `timeout 120s cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib managed_network_transaction_relay -- --nocapture`
- `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-network -p open-bitcoin-node --all-targets --all-features -- -D warnings`
- `bash scripts/check-file-lengths.sh`
- `bun run scripts/check-parity-breadcrumbs.ts --check`
- Pure-core `cargo llvm-cov` slice reported no uncovered `inventory_state` or `transaction_relay` lines.
- Commit hook ran `bash scripts/verify.sh` successfully for `a512cf47`.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Split managed action translation out of network.rs**
- **Found during:** file-length verification
- **Issue:** Keeping transaction relay action translation inline would leave `packages/open-bitcoin-node/src/network.rs` over the production Rust file-length limit.
- **Fix:** Added `packages/open-bitcoin-node/src/network/action_translation.rs` and re-exported it through the existing network module.
- **Verification:** `bash scripts/check-file-lengths.sh`
- **Committed in:** `a512cf47`

**2. [Rule 1 - Bug] Restored pure-core coverage for transaction relay cleanup helpers**
- **Found during:** coverage verification
- **Issue:** A helper for mixed block and transaction request cleanup introduced uncovered lines in `inventory_state`.
- **Fix:** Inlined the remaining block cleanup behavior and kept transaction cleanup under the scheduler APIs.
- **Verification:** Pure-core `cargo llvm-cov` slice reported no uncovered `inventory_state` or `transaction_relay` lines.
- **Committed in:** `a512cf47`

**3. [Rule 3 - Blocking] Consolidated interrupted executor output into one implementation commit**
- **Found during:** subagent commit handoff
- **Issue:** The subagent implemented the wave but failed before producing a clean commit and summary.
- **Fix:** Main execution reviewed, completed verification fixes, staged the final diff, and committed the plan as one coherent implementation commit.
- **Verification:** Full commit hook passed `bash scripts/verify.sh`.
- **Committed in:** `a512cf47`

**Total deviations:** 3 auto-fixed.
**Impact on plan:** No scope expansion; the changes were required for repo verification, coverage, and recoverable execution.

## Issues Encountered

- The initial executor handoff left source changes staged without a valid commit. The main workflow recovered by re-running focused tests, clippy, file-length, parity breadcrumb, coverage, and full verifier checks before committing.
- The full verifier for the implementation commit completed successfully in 5m 58.366s.

## Auth Gates

None.

## Known Stubs

None. Stub scan found no `TODO`, `FIXME`, placeholder text, empty hardcoded UI data, or unwired mock data in files created or modified by this plan.

## Threat Flags

None. This plan changed in-memory transaction download scheduling and managed network message translation only; it did not add new auth paths, persistence schemas, public defaults, RPC methods, service-manager behavior, or production relay claims.

## User Setup Required

None.

## Next Phase Readiness

Plan 03 can document the bounded Phase 101 parity claim, add the deterministic checker, wire it into the repo verifier, and record phase-level verification.

## Phase State

Per orchestrator instruction, this executor did not advance phase state, update roadmap progress, mark requirements complete, or transition to the next phase.

## Self-Check: PASSED

- Found summary file: `.planning/phases/101-transaction-inventory-identity-and-download-scheduling/101-02-SUMMARY.md`
- Found task commit: `a512cf47`
- Verified summary uses standalone `---` only for frontmatter delimiters.
