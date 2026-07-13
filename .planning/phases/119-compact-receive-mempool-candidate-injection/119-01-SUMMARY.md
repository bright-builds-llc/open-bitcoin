---
phase: 119-compact-receive-mempool-candidate-injection
plan: 01
subsystem: network
tags: [compact-block, mempool, bip152, reconstruction, peer-manager]

requires:
  - phase: 114-compact-block-reconstruction-from-mempool-state
    provides: PartialCompactBlock::on_mempool_transaction_removed and CompactBlockReceiveFacts
  - phase: 115-missing-transaction-round-trip-fallback-and-validation-handoff
    provides: compact download in-flight state and handle_compact_block_download
provides:
  - PeerManager::on_mempool_transaction_removed forwarder over all in-flight partials
  - CompactExtraTxnBuffer with Knots-aligned slot/byte/per-tx bounds
  - mempool_compact_candidate_owned and compact_extra_owned snapshot helpers
affects:
  - 119-02 ManagedPeerNetwork CompactBlock intercept
  - 119-03 mempool lifecycle hook and runtime tests

tech-stack:
  added: []
  patterns:
    - PeerManager wtxid forwarder without open-bitcoin-mempool coupling
    - Node-owned Knots-shaped CompactExtraTxnBuffer ring (AddToCompactExtraTransactions)

key-files:
  created:
    - packages/open-bitcoin-node/src/network/compact_receive_candidates.rs
  modified:
    - packages/open-bitcoin-network/src/peer/compact_download_state.rs
    - packages/open-bitcoin-network/src/peer/message_dispatch.rs
    - packages/open-bitcoin-network/src/peer/tests.rs
    - packages/open-bitcoin-node/src/network.rs
    - docs/parity/source-breadcrumbs.json

key-decisions:
  - "PeerManager forwarder walks compact_download_states[*].in_flight[*].partial by wtxid only (D-07/D-08)"
  - "Empty-facts CompactBlock in message_dispatch kept callable with non-production annotation (D-03)"
  - "CompactExtraTxnBuffer uses virtual size as Knots-aligned byte-budget approximation (D-05 / RESEARCH A1)"

patterns-established:
  - "Pattern: PeerManager mempool lifecycle forwarder clears volatile partial slots without mempool types"
  - "Pattern: Node shell owns CompactExtraTxnBuffer; network crate stays mempool-free"

requirements-completed: [RCN-02, GOV-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 119-2026-07-13T16-08-52
generated_at: 2026-07-13T18:20:00Z

duration: 7min
completed: 2026-07-13
---

# Phase 119 Plan 01: Forwarder and Extra Buffer Foundation Summary

**PeerManager mempool-removal forwarder plus Knots-shaped CompactExtraTxnBuffer and mempool snapshot helpers for Phase 119 candidate injection.**

## Performance

- **Duration:** 7 min
- **Started:** 2026-07-13T18:13:06Z
- **Completed:** 2026-07-13T18:19:53Z
- **Tasks:** 2
- **Files modified:** 6

## Accomplishments

- Added `PeerManager::on_mempool_transaction_removed` that walks every peer's in-flight partial compact state and clears matching wtxid slots without importing mempool.
- Annotated the empty-facts `CompactBlock` branch in `message_dispatch` as non-production for live shell receive (D-03).
- Shipped `CompactExtraTxnBuffer` with Knots defaults (32768 slots / 10MB / 100KB per-tx gate), ring overwrite, byte-budget eviction, and `mempool_compact_candidate_owned` / `compact_extra_owned` helpers.

## Task Commits

Each task was committed atomically:

1. **Task 1: PeerManager mempool-removal forwarder** - `d591797d` (test) → `42885519` (feat)
2. **Task 2: CompactExtraTxnBuffer and mempool snapshot helpers** - `c644da44` (test) → `80ac4cf0` (feat)

**Plan metadata:** _(pending final docs commit)_

_Note: TDD tasks used RED → GREEN commits_

## Files Created/Modified

- `packages/open-bitcoin-network/src/peer/compact_download_state.rs` — PeerManager mempool-removal forwarder
- `packages/open-bitcoin-network/src/peer/message_dispatch.rs` — empty-facts CompactBlock non-production annotation
- `packages/open-bitcoin-network/src/peer/tests.rs` — forwarder unit test with one matched + one missing short-id
- `packages/open-bitcoin-node/src/network/compact_receive_candidates.rs` — buffer + snapshot helpers + unit tests
- `packages/open-bitcoin-node/src/network.rs` — register `compact_receive_candidates` module
- `docs/parity/source-breadcrumbs.json` — `node-compact-receive-candidates` breadcrumb entry

## Decisions Made

- Forwarder only clears matching volatile slots; no timeouts, no `on_compact_download_block_connected`, no mempool crate dependency in network.
- Extra-buffer aggregate accounting uses virtual size as an acceptable Knots-aligned approximation (not RecursiveDynamicUsage identity).
- ManagedPeerNetwork field wiring deferred to Plan 02 as specified.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None

## User Setup Required

None

## Known Stubs

None — stubs used only during TDD RED and were replaced by the GREEN implementation.

## Threat Flags

None — no new trust-boundary surface beyond the plan threat model (buffer DoS caps mitigated; empty-facts path accepted with annotation).

## Next Phase Readiness

Ready for Plan 02: shell CompactBlock intercept, ManagedPeerNetwork buffer field, and live candidate injection into `handle_compact_block_download`.

## Verification Evidence

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network --lib peer_manager_on_mempool_transaction_removed` — pass
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib compact_receive_candidates` — pass (6 tests)
- `packages/open-bitcoin-network/Cargo.toml` has no `open-bitcoin-mempool` dependency

## Self-Check: PASSED

- Created/modified key files present on disk
- Commits `d591797d`, `42885519`, `c644da44`, `80ac4cf0` present in git log
- Acceptance must_haves verified (forwarder, Knots bounds, empty-facts annotation, no network→mempool dep)
