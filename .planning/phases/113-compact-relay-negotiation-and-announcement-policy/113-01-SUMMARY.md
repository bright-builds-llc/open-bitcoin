---
phase: 113-compact-relay-negotiation-and-announcement-policy
plan: 01
subsystem: network-peer-policy
tags: [rust, bip152, sendcmpct, compact-relay, peer-state, parity]

requires:
  - phase: 112-bip152-wire-codec-and-message-semantics
    provides: BIP152 `SendCompactMessage` decoding and `WireNetworkMessage::SendCompact`
provides:
  - Typed per-peer compact relay negotiation state for BIP152 sendcmpct messages
  - PeerManager sendcmpct routing that mutates only compact relay peer state and returns no actions
  - Deterministic tests for high-bandwidth, low-bandwidth, last-supported preference, unsupported-version evidence, and transaction-relay isolation
affects: [phase-113-plan-02-compact-announcement-policy, phase-114-compact-reconstruction, phase-116-operator-evidence]

tech-stack:
  added: []
  patterns:
    - Pure compact relay negotiation state in open-bitcoin-network peer policy
    - Unsupported BIP152 versions stored as fixed typed evidence without disconnecting
    - Wrapper-owned final git mutation with task commits recorded as pending-final-commit

key-files:
  created:
    - packages/open-bitcoin-network/src/peer/compact_relay.rs
    - .planning/phases/113-compact-relay-negotiation-and-announcement-policy/113-01-SUMMARY.md
  modified:
    - packages/open-bitcoin-network/src/peer.rs
    - packages/open-bitcoin-network/src/peer/tests.rs
    - packages/open-bitcoin-network/src/lib.rs
    - docs/parity/source-breadcrumbs.json

key-decisions:
  - "Compact relay capability and high-/low-bandwidth preferences live in a dedicated `CompactRelayPeerState` instead of transaction relay or permission state."
  - "Unsupported sendcmpct versions record `maybe_unsupported_version` and only set Unsupported capability when no supported version 2 preference has been recorded."
  - "sendcmpct handling keeps `announcement_eligibility` Unknown; Plan 113-02 owns explicit announcement decision recording."
  - "No git commits were created because the wrapper reserves final git mutation for the orchestrator."

patterns-established:
  - "Version 2 sendcmpct is the only positive compact relay capability signal."
  - "Repeated supported sendcmpct messages use last-supported preference semantics and clear the opposite bandwidth preference."
  - "Transaction relay and wtxidrelay messages do not activate compact relay capability."

requirements-completed: [CMP-04, CMP-06]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 113-2026-07-04T22-53-48
generated_at: 2026-07-04T23:18:29Z

duration: 4m 4s
completed: 2026-07-04
---

# Phase 113 Plan 01: Compact Relay Negotiation State Summary

**BIP152 sendcmpct now updates explicit per-peer compact relay capability and bandwidth preference state without coupling to transaction relay, permissions, or announcement eligibility.**

## Performance

- **Duration:** 4m 4s
- **Started:** 2026-07-04T23:14:25Z
- **Completed:** 2026-07-04T23:18:29Z
- **Tasks:** 2 completed
- **Files modified:** 6 including this summary

## Accomplishments

- Added `CompactRelayPeerState` with typed capability, high-bandwidth preference, low-bandwidth preference, announcement eligibility, and unsupported-version evidence.
- Implemented `apply_send_compact` as a pure state transition using `BIP152_COMPACT_BLOCKS_VERSION`, including last-supported high/low preference semantics.
- Wired `PeerManager::handle_message` so `WireNetworkMessage::SendCompact(message)` updates only the matched peer's compact relay state and emits no peer actions.
- Added deterministic tests proving version 2 high/low behavior, high-to-low and low-to-high preference clearing, unsupported-version preservation, and transaction-relay isolation.
- Registered `packages/open-bitcoin-network/src/peer/compact_relay.rs` in `docs/parity/source-breadcrumbs.json`.

## Task Changes

No commits were created. The parent wrapper owns verification-first final git mutation, so commit fields are recorded as `pending-final-commit`.

1. **Task 1: Create typed compact relay peer state** - `pending-final-commit`
   - RED tests failed as expected for missing compact relay state types and APIs.
   - GREEN implementation added the typed pure state module, default unknown state, supported/unsupported transitions, exports, and parity breadcrumb registration.

2. **Task 2: Apply sendcmpct messages to peer state** - `pending-final-commit`
   - RED tests failed as expected for missing `PeerState.compact_relay` and peer sendcmpct routing.
   - GREEN implementation added the peer state field, initialized it for new peers, and routed `SendCompact(message)` through `apply_send_compact`.

**Plan metadata:** `pending-final-commit`

## Files Created/Modified

- `packages/open-bitcoin-network/src/peer/compact_relay.rs` - Pure compact relay negotiation state, outcome/reason types, and unit tests.
- `packages/open-bitcoin-network/src/peer.rs` - Stores per-peer compact relay state and applies decoded sendcmpct messages.
- `packages/open-bitcoin-network/src/peer/tests.rs` - Adds Phase 113 sendcmpct and transaction-relay-isolation tests.
- `packages/open-bitcoin-network/src/lib.rs` - Re-exports compact relay state and reason types.
- `docs/parity/source-breadcrumbs.json` - Registers the new compact relay peer-state source file.

## Decisions Made

- Kept unsupported-version observations as evidence instead of disconnecting or clearing a valid version 2 preference, matching the phase contract.
- Kept announcement eligibility unchanged by sendcmpct so Plan 113-02 can record it only from explicit announcement decisions.
- Used `git add -N` only for the new Rust source so `scripts/check-parity-breadcrumbs.ts --check` could see it through `git ls-files`; no commit, push, or broad staging command was run.

## Deviations from Plan

None - plan executed exactly as written. The no-commit behavior is a wrapper instruction, not a plan deviation.

## Issues Encountered

- The RED test pass failed for the intended missing API and peer-state fields before implementation.
- `scripts/check-parity-breadcrumbs.ts --check` scans tracked paths via `git ls-files`, so the new `compact_relay.rs` file needed `git add -N` intent-to-add for verification. No commit was created.

## Known Stubs

None found.

## Threat Flags

None. The only new trust-boundary behavior is the planned peer `sendcmpct` message to typed per-peer negotiation state transition; no network endpoint, auth path, file access path, schema boundary, compact payload serving, reconstruction, operator evidence, or public-default surface was introduced.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network compact_relay -- --nocapture` - passed after formatting.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network phase113_sendcmpct -- --nocapture` - passed after formatting.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network phase113_transaction_relay_messages_do_not_activate_compact_relay_state -- --nocapture` - passed after formatting.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network phase113_unsupported_sendcmpct_does_not_clear_existing_version2_capability -- --nocapture` - passed.
- `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-network --all-targets --all-features -- -D warnings` - passed.
- `bun run scripts/check-parity-breadcrumbs.ts --check` - passed for 353 Rust files.
- `cargo fmt --manifest-path packages/Cargo.toml --all` - applied formatting.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Ready for `113-02-PLAN.md` to derive and record compact announcement eligibility from explicit announcement decisions. Compact relay negotiation state is now available on each peer, while compact reconstruction, missing-transaction scheduling, fallback validation, operator evidence, package relay, filter serving, public defaults, and production-readiness claims remain deferred.

## Self-Check: PASSED

- Summary file created at `.planning/phases/113-compact-relay-negotiation-and-announcement-policy/113-01-SUMMARY.md`.
- Required source files exist, including `packages/open-bitcoin-network/src/peer/compact_relay.rs`.
- Acceptance probes confirmed `CompactRelayPeerState`, `apply_send_compact`, `BIP152_COMPACT_BLOCKS_VERSION`, `PeerState.compact_relay`, `WireNetworkMessage::SendCompact(message)`, crate-root re-export, and parity breadcrumb registry entries.
- No git commits or pushes were created; commit fields remain `pending-final-commit` for the parent wrapper.

*Phase: 113-compact-relay-negotiation-and-announcement-policy*
*Completed: 2026-07-04*
