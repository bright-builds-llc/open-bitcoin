---
phase: 113-compact-relay-negotiation-and-announcement-policy
plan: 02
subsystem: network-peer-policy
tags: [rust, bip152, compact-relay, announcement-policy, peer-state, parity]

requires:
  - phase: 113-compact-relay-negotiation-and-announcement-policy
    provides: Plan 113-01 typed compact relay peer negotiation state and sendcmpct routing
provides:
  - Pure compact announcement decision API with typed actions, fixed reasons, and deterministic eligibility
  - PeerManager compact announcement entrypoint that consumes stored peer negotiation state and records decision-derived eligibility
  - Gate-specific tests for activation, negotiation, high-bandwidth, header continuity, block availability, resource limits, and unsupported-version preservation
affects: [phase-113-plan-03-announcement-evidence, phase-114-compact-reconstruction, phase-116-operator-evidence]

tech-stack:
  added: []
  patterns:
    - Pure data-in/data-out compact announcement policy in open-bitcoin-network
    - Decision-derived CompactAnnouncementEligibility recorded only through explicit announcement decisions
    - Wrapper-owned final git mutation with task commits recorded as pending-final-commit

key-files:
  created:
    - .planning/phases/113-compact-relay-negotiation-and-announcement-policy/113-02-SUMMARY.md
  modified:
    - packages/open-bitcoin-network/src/peer/compact_relay.rs
    - packages/open-bitcoin-network/src/peer.rs
    - packages/open-bitcoin-network/src/peer/tests.rs
    - packages/open-bitcoin-network/src/lib.rs

key-decisions:
  - "Compact announcement decisions return typed actions and low-cardinality reasons rather than booleans or node-shell messages."
  - "Eligibility is derived from `CompactAnnouncementDecision` and recorded on `CompactRelayPeerState` only through `record_announcement_decision`."
  - "Unsupported sendcmpct evidence remains stored but does not override a prior supported version 2 high-bandwidth preference."
  - "No git commits were created because the wrapper reserves final git mutation for the orchestrator."

patterns-established:
  - "Local compact relay activation is the first announcement gate."
  - "High-bandwidth preference is checked before header continuity."
  - "Block availability and resource gates suppress rather than falling back to headers or inventory."

requirements-completed: [CMP-04, CMP-05]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 113-2026-07-04T22-53-48
generated_at: 2026-07-04T23:23:56Z

duration: 4m 4s
completed: 2026-07-04
---

# Phase 113 Plan 02: Compact Announcement Decision Policy Summary

**Compact relay announcement policy now derives typed announce, fallback, or suppress decisions from local activation, stored peer negotiation state, header continuity, block availability, and resource gates.**

## Performance

- **Duration:** 4m 4s
- **Started:** 2026-07-04T23:19:52Z
- **Completed:** 2026-07-04T23:23:56Z
- **Tasks:** 2 completed
- **Files modified:** 5 including this summary

## Accomplishments

- Added `CompactAnnouncementInput`, `PeerCompactAnnouncementInput`, `CompactAnnouncementDecision`, `CompactAnnouncementAction`, and `CompactAnnouncementReason`.
- Implemented `decide_compact_announcement` with the required gate order: activation, negotiation, unsupported-version-only state, high-bandwidth preference, previous/current header continuity, block status, resource gate, then compact announce.
- Added deterministic eligibility mapping from decision reasons to `CompactAnnouncementEligibility` and `CompactAnnouncementEligibilityReason`.
- Added `CompactRelayPeerState::record_announcement_decision`, which updates only `announcement_eligibility`.
- Added `PeerManager::decide_compact_announcement_for_peer`, which reads stored peer compact relay state and `remote_prefers_headers`, calls the pure decision API, records eligibility, and returns unknown-peer errors for missing peers.
- Added unit coverage for all planned compact announcement gates, eligibility refresh across high -> low -> high sendcmpct toggles, and supported preference preservation after unsupported sendcmpct evidence.

## Task Changes

No commits were created. The parent wrapper owns verification-first final git mutation, so commit fields are recorded as `pending-final-commit`.

1. **Task 1: Add pure compact announcement decision types** - `pending-final-commit`
   - Added the pure announcement policy types, fixed reason labels, gate-ordered `decide_compact_announcement`, and eligibility recording on `CompactRelayPeerState`.
   - Added compact announcement unit tests covering the happy path, eligibility mappings, unsupported-without-supported preference, supported preference preservation, high-bandwidth-before-header ordering, status/resource suppression, and record-only eligibility mutation.

2. **Task 2: Consume announcement policy from PeerManager tests** - `pending-final-commit`
   - RED test run failed as expected because `PeerManager::decide_compact_announcement_for_peer` did not exist.
   - GREEN implementation added the PeerManager entrypoint and re-exported the policy API through `peer.rs` and `lib.rs`.
   - Added the plan-specified `phase113_compact_announcement_*` tests for compact action, fallback, suppression, recorded eligibility, high/low/high refresh, and unsupported evidence preservation.

**Plan metadata:** `pending-final-commit`

## Files Created/Modified

- `packages/open-bitcoin-network/src/peer/compact_relay.rs` - Pure compact announcement policy, reason/action types, eligibility mapping, decision recording, and unit tests.
- `packages/open-bitcoin-network/src/peer.rs` - PeerManager compact announcement entrypoint and peer-module re-exports.
- `packages/open-bitcoin-network/src/peer/tests.rs` - Phase 113 PeerManager compact announcement gate tests.
- `packages/open-bitcoin-network/src/lib.rs` - Crate-root re-exports for compact announcement policy consumers.
- `.planning/phases/113-compact-relay-negotiation-and-announcement-policy/113-02-SUMMARY.md` - Execution summary.

## Decisions Made

- Kept compact announcement policy in `peer/compact_relay.rs` as a pure function, preserving the functional-core boundary and avoiding socket, storage, payload-building, or validation side effects.
- Preserved gate-specific reasons during headers/inventory fallback instead of replacing them with generic fallback labels, so later evidence can report the first failed compact gate.
- Recorded announcement eligibility only after an explicit decision, preventing sendcmpct handling from leaving stale `Eligible` state across high/low/high preference toggles.

## Deviations from Plan

None in behavior or scope. The no-commit behavior is a wrapper instruction, not a plan deviation; task and metadata commit fields are therefore `pending-final-commit`.

## Issues Encountered

- The Task 2 RED run failed for the intended missing `PeerManager::decide_compact_announcement_for_peer` API before implementation.
- The first Task 1 targeted run passed after a combined no-commit test/implementation edit and emitted temporary dead-code warnings before the PeerManager consumption path was added. Final clippy passed with `-D warnings`.

## Known Stubs

None found.

## Threat Flags

None. The only new trust-boundary behavior is the planned pure decision from negotiated peer state plus local block/resource facts into typed compact announcement action and eligibility. No new network endpoint, auth path, file access pattern, schema boundary, runtime socket/storage effect, compact payload construction, reconstruction, package relay, bloom/filter serving, public default, public CI, or production-readiness surface was introduced.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network phase113_compact_announcement -- --nocapture` - failed before implementation on the missing PeerManager API, as expected for RED.
- `cargo fmt --manifest-path packages/Cargo.toml --all` - applied formatting.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network compact_announcement -- --nocapture` - passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network phase113_compact_announcement -- --nocapture` - passed.
- `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-network --all-targets --all-features -- -D warnings` - passed.
- `git diff --check` - passed.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Ready for `113-03-PLAN.md` to build the remaining compact relay announcement policy/evidence integration on top of the typed decision surface. Compact payload construction, short-ID reconstruction, missing transaction scheduling, `getblocktxn`/`blocktxn` round trips, validation handoff, package relay, bloom/filter serving, public defaults, public-network CI, and production-readiness claims remain deferred.

## Self-Check: PASSED

- Summary file created at `.planning/phases/113-compact-relay-negotiation-and-announcement-policy/113-02-SUMMARY.md`.
- Acceptance probes confirmed `decide_compact_announcement`, `CompactAnnouncementDecision.eligibility`, `CompactRelayPeerState::record_announcement_decision`, `PeerManager::decide_compact_announcement_for_peer`, crate-root exports, and all plan-specified PeerManager test names.
- Stub scan found no placeholder, TODO/FIXME, or hardcoded empty UI/data stub markers in `packages/open-bitcoin-network/src/peer/compact_relay.rs`.
- No git commits, pushes, broad staging, or `gsd-tools commit` commands were run; commit fields remain `pending-final-commit` for the parent wrapper.

*Phase: 113-compact-relay-negotiation-and-announcement-policy*
*Completed: 2026-07-04*
