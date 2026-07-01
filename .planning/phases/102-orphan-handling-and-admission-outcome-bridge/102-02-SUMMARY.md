---
phase: 102-orphan-handling-and-admission-outcome-bridge
plan: 02
subsystem: network transaction relay
tags:
  - rust
  - transaction-relay
  - orphanage
  - peer-manager
  - resource-governance
requires:
  - phase: 102-01
    provides: mempool admission outcomes for missing-parent bridge decisions
  - phase: 101-02
    provides: typed transaction download scheduler actions and request caps
  - phase: 94-01
    provides: transaction request resource-governance limits
provides:
  - bounded in-memory transaction orphanage with expiry, deterministic eviction, and reconsideration actions
  - scheduler-backed orphan parent requests using TxRelayId::Txid without direct socket writes
  - PeerManager API for orphan parent requests that preserves existing transaction request caps
  - deterministic fake-time tests for orphan staging, eviction, expiry, reconsideration, cleanup, and request suppression
affects:
  - network transaction relay
  - mempool admission bridge
  - peer resource governance
tech-stack:
  added: []
  patterns:
    - pure functional-core orphan state returning typed actions
    - scheduler-mediated parent fetches through existing transaction download state
key-files:
  created:
    - packages/open-bitcoin-network/src/peer/transaction_relay/orphanage.rs
    - packages/open-bitcoin-network/src/peer/transaction_relay/tests/orphanage_cases.rs
  modified:
    - packages/open-bitcoin-network/src/lib.rs
    - packages/open-bitcoin-network/src/peer.rs
    - packages/open-bitcoin-network/src/peer/inventory_state.rs
    - packages/open-bitcoin-network/src/peer/tests.rs
    - packages/open-bitcoin-network/src/peer/transaction_relay.rs
    - packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs
    - packages/open-bitcoin-network/src/peer/transaction_relay/tests.rs
    - packages/open-bitcoin-network/src/peer/transaction_relay/tests/scheduler_cases.rs
    - packages/open-bitcoin-network/src/peer/transaction_relay/tests/scheduler_cases/edge_cases.rs
    - packages/open-bitcoin-network/src/peer/transaction_relay/tests/scheduler_cases/received_cases.rs
    - docs/parity/source-breadcrumbs.json
    - docs/metrics/lines-of-code.md
key-decisions:
  - "Bounded orphan staging remains pure network state and returns typed actions instead of mutating mempool or socket state."
  - "Orphan parent requests reuse Phase 101 transaction download scheduler caps, duplicate suppression, and local-fact suppression."
  - "PeerManager request routing lives behind the inventory-state extension so peer.rs stays under the repo file-length guard."
patterns-established:
  - "Use TxRelayId::Txid for missing-parent fetches and preserve request-governance accounting."
  - "Use injected unix timestamps for orphan expiry tests instead of wall-clock reads or sleeps."
requirements-completed:
  - DL-03
  - DL-04
  - DL-05
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 102-2026-06-30T14-54-50
generated_at: 2026-07-01T03:11:55Z
duration: 1h 35m
completed: 2026-07-01
---

# Phase 102 Plan 02: Bounded Orphan Staging and Peer Request Logic Summary

**Bounded fake-time orphan staging plus scheduler-backed parent txid requests and peer cleanup hooks.**

## Performance

- **Duration:** 1h 35m
- **Started:** 2026-07-01T01:40:00Z
- **Completed:** 2026-07-01T03:11:55Z
- **Tasks:** 2
- **Files modified:** 14

## Accomplishments

- Added `TxOrphanage` with Phase 102 bounds: total cap 100, per-peer cap 25, 20-minute injected-time TTL, and 32 reconsiderations per accepted parent.
- Added deterministic orphan actions and evidence labels for parent requests, eviction, expiry, reconsideration, reconsideration outcomes, and peer cleanup.
- Routed orphan parent fetches through `TxDownloadScheduler::request_parent`, preserving already-have, recent-reject, mempool-known, duplicate in-flight, and request-cap suppression.
- Exposed `PeerManager::request_orphan_parent` as a typed wrapper returning `PeerAction::TransactionRelay` actions and `NetworkError::UnknownPeer` for unknown peers.
- Added parity breadcrumbs for the new orphanage source and tests, and refreshed the tracked LOC report.

## Task Commits

Each task was committed atomically:

1. **Task 1: Add bounded orphanage contracts and deterministic pure tests** - `9192102e` (`feat`)
2. **Task 2: Route orphan parent requests through scheduler caps and PeerManager** - `dff72cfa` (`feat`)

## Files Created/Modified

- `packages/open-bitcoin-network/src/peer/transaction_relay/orphanage.rs` - Pure bounded orphan staging state and action contracts.
- `packages/open-bitcoin-network/src/peer/transaction_relay/tests/orphanage_cases.rs` - Fake-time orphanage cap, expiry, reconsideration, outcome, and cleanup tests.
- `packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs` - Scheduler parent-request entrypoint using existing suppression and in-flight state.
- `packages/open-bitcoin-network/src/peer/transaction_relay.rs` - Orphanage module registration and re-exports.
- `packages/open-bitcoin-network/src/peer.rs` - Public `request_orphan_parent` wrapper and mempool-known local fact storage.
- `packages/open-bitcoin-network/src/peer/inventory_state.rs` - PeerManager transaction relay accessors and orphan parent routing implementation.
- `packages/open-bitcoin-network/src/peer/tests.rs` - PeerManager orphan parent request, cap, suppression, governance, and unknown-peer tests.
- `packages/open-bitcoin-network/src/peer/transaction_relay/tests.rs` - Test module wiring for orphanage and scheduler cases.
- `packages/open-bitcoin-network/src/peer/transaction_relay/tests/scheduler_cases.rs` - Duplicate pending parent request regression coverage.
- `packages/open-bitcoin-network/src/peer/transaction_relay/tests/scheduler_cases/edge_cases.rs` - Scheduler coverage adjustments committed with Task 1.
- `packages/open-bitcoin-network/src/peer/transaction_relay/tests/scheduler_cases/received_cases.rs` - Scheduler coverage adjustments committed with Task 1.
- `packages/open-bitcoin-network/src/lib.rs` - Network crate module exposure touched by Task 1.
- `docs/parity/source-breadcrumbs.json` - Breadcrumb coverage for orphanage source and tests.
- `docs/metrics/lines-of-code.md` - Regenerated tracked LOC metric.

## Decisions Made

- Keep orphanage state deterministic and in-memory only; it emits actions and candidates but does not perform I/O, storage, logging, status projection, random eviction, recursion, or mempool mutation.
- Keep orphan parent requests in the existing transaction download scheduler instead of adding a parallel request path, so Phase 94 and Phase 101 governance remains authoritative.
- Move PeerManager transaction relay accessors and routing internals into `peer/inventory_state.rs` so the public wrapper can stay compact and `peer.rs` remains under the production file-length limit.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Preserved hook requirement during TDD RED**
- **Found during:** Task 1 and Task 2 TDD flow
- **Issue:** The formal TDD RED commits could not be recorded because repo hooks require passing commits.
- **Fix:** Ran the failing RED checks locally, then committed the passing implementation and tests atomically per task.
- **Files modified:** No additional files beyond task implementation.
- **Verification:** Task-focused tests failed before implementation and passed after implementation.
- **Committed in:** `9192102e`, `dff72cfa`

**2. [Rule 3 - Blocking] Added coverage for orphanage and scheduler branches required by hook**
- **Found during:** Task 1 and Task 2 commit hooks
- **Issue:** The repo coverage gate reported missing branch coverage after the initial implementation.
- **Fix:** Added additional deterministic orphanage cases and `orphan_parent_request_suppresses_duplicate_pending_parent`.
- **Files modified:** `packages/open-bitcoin-network/src/peer/transaction_relay/tests/orphanage_cases.rs`, `packages/open-bitcoin-network/src/peer/transaction_relay/tests.rs`, `packages/open-bitcoin-network/src/peer/transaction_relay/tests/scheduler_cases.rs`
- **Verification:** Filtered pure-core `cargo llvm-cov` run reported `coverage gate clean: no Uncovered Lines`; `bash scripts/verify.sh` passed in both task commit hooks.
- **Committed in:** `9192102e`, `dff72cfa`

**3. [Rule 3 - Blocking] Kept peer.rs under the production file-length guard**
- **Found during:** Task 2 commit hook
- **Issue:** Adding the PeerManager wrapper and test-facing accessors pushed `peer.rs` over the repo production file-length limit.
- **Fix:** Moved transaction request snapshot, mempool-known note, and orphan parent routing internals into `peer/inventory_state.rs`, leaving the public `peer.rs` wrapper compact.
- **Files modified:** `packages/open-bitcoin-network/src/peer.rs`, `packages/open-bitcoin-network/src/peer/inventory_state.rs`
- **Verification:** Hook reported `Production Rust file-length check passed: 247 file(s) checked, limit 628 lines.`
- **Committed in:** `dff72cfa`

**4. [Rule 3 - Blocking] Re-ran a transient Phase 101 checker timeout**
- **Found during:** Task 1 commit hook
- **Issue:** The first hook attempt hit a Phase 101 checker timeout before a task commit could be recorded.
- **Fix:** Re-ran after the inline fixes; the checker completed normally in the passing hook run.
- **Files modified:** None.
- **Verification:** `bash scripts/verify.sh` passed in the task commit hook.
- **Committed in:** `9192102e`

**Total deviations:** 4 auto-fixed (4 Rule 3)
**Impact on plan:** All fixes were required to satisfy repository verification and did not add behavior beyond the bounded orphan staging and scheduler-backed parent request scope.

## Issues Encountered

- The plan acceptance grep included `mempool` in a no-direct-mutation scan while the same task required `MempoolKnown` and `mempool_known` local-fact suppression. The exact pure orphanage/scheduler scan now finds only `mempool_known` local-fact references in `scheduler.rs`; no `WireNetworkMessage::GetData`, `submit_transaction`, `socket`, or `Tcp` direct path was introduced in `orphanage.rs` or `scheduler.rs`.

## Verification

- `cargo fmt --manifest-path packages/Cargo.toml --all`
- `cargo clippy --manifest-path packages/Cargo.toml --all-targets --all-features -- -D warnings`
- `cargo build --manifest-path packages/Cargo.toml --all-targets --all-features`
- `cargo test --manifest-path packages/Cargo.toml --all-features`
- `timeout 120s cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network --lib orphanage -- --nocapture`
- `timeout 120s cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network --lib orphan_parent_request -- --nocapture`
- `timeout 120s cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network --lib peer_manager_transaction_relay -- --nocapture`
- `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-network --all-targets --all-features -- -D warnings`
- `bun run scripts/check-parity-breadcrumbs.ts --check`
- Filtered pure-core `cargo llvm-cov --show-missing-lines --text` with no `Uncovered Lines:`
- `bash scripts/verify.sh` passed in the Task 1 and Task 2 commit hooks; Task 2 hook completed in 4m 19.914s.

## Stub Scan

None found. The touched files were scanned for placeholder text, TODO/FIXME markers, and hardcoded empty/null UI-flow stubs.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

DL-03, DL-04, and DL-05 are satisfied at the pure network boundary. Missing-parent transactions can now be staged and bounded, parent requests are scheduler-governed, reconsideration is deterministic and capped, and disconnect/expiry/eviction paths produce typed evidence for the later admission bridge work.

*Phase: 102-orphan-handling-and-admission-outcome-bridge*
*Completed: 2026-07-01*

## Self-Check: PASSED

- Verified key created/modified files exist.
- Verified task commits `9192102e` and `dff72cfa` exist in git history.
