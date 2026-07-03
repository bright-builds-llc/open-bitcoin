---
phase: 107-runtime-relay-activation-and-download-eligibility-integration
plan: 01
subsystem: network
tags:
  - relay
  - transaction-download
  - peer-manager
  - scheduler

requires:
  - phase: 100-relay-activation-boundary-and-permission-semantics
    provides: Relay activation and peer eligibility classifier inputs
  - phase: 101-transaction-inventory-identity-and-download-scheduling
    provides: Transaction announcement scheduler, request state, and cleanup behavior
provides:
  - Default-off PeerManager relay download policy
  - Pure PeerManager-to-relay-eligibility adapter
  - Scheduler eligibility gate before transaction download state mutation
  - Typed low-cardinality transaction download suppression evidence
affects:
  - transaction relay scheduling
  - peer inventory handling
  - orphan parent requests
  - parity breadcrumbs

tech-stack:
  added: []
  patterns:
    - Pure relay eligibility adapter at the PeerManager boundary
    - Typed scheduler input for relay eligibility on announcements and parent requests
    - Fixed suppression labels for public evidence

key-files:
  created:
    - packages/open-bitcoin-network/src/peer/relay_download.rs
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
    - docs/parity/source-breadcrumbs.json
    - docs/metrics/lines-of-code.md

key-decisions:
  - "Gate transaction downloads in the pure scheduler before candidate or in-flight request state can be inserted."
  - "Keep PeerManager relay download default-off and require explicit RelayDownloadPolicy activation for scheduling."
  - "Use fixed suppression labels instead of peer ids, endpoints, permission strings, or transaction material."
  - "Register relay_download.rs under the existing transaction-relay-download parity breadcrumb group."

patterns-established:
  - "PeerManager computes relay download eligibility from connection class and permission effects before scheduler calls."
  - "Ineligible relay decisions suppress without mutating candidate, in-flight, or already-have scheduler state."
  - "Tests cover disabled, outbound, ordinary inbound, protected-only inbound, permissioned inbound, and fallback announcer behavior."

requirements-completed:
  - ACT-02
  - INV-02
  - INV-03
  - DL-01
  - DL-02
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 107-2026-07-03T02-54-20
generated_at: 2026-07-03T04:06:48Z

duration: 17min
completed: 2026-07-03
---

# Phase 107 Plan 01: Runtime Relay Activation and Download Eligibility Integration Summary

**Default-off relay download eligibility now gates transaction getdata scheduling before scheduler state mutation.**

## Performance

- **Duration:** 17 min
- **Started:** 2026-07-03T03:50:06Z
- **Completed:** 2026-07-03T04:06:48Z
- **Tasks:** 2
- **Files modified:** 13, including this summary

## Accomplishments

- Added typed relay eligibility suppressions for disabled relay, generic ineligible relay, inbound serving requirements, permission requirements, and protected-only inbound peers.
- Extended scheduler inputs so transaction announcements and orphan parent requests carry `RelayEligibilityDecision` before any candidate or in-flight request state is created.
- Added `RelayDownloadPolicy` and PeerManager eligibility wiring for default-off activation, outbound peers, ordinary inbound peers, protected-only inbound peers, and permissioned inbound peers.
- Preserved existing txid/wtxid duplicate, fallback, expiry, notfound, disconnect, and received-transaction cleanup behavior under the new eligibility gate.

## Task Commits

No commits were created. The execution context explicitly instructed this executor not to commit; the parent workflow owns final commit and push after whole-phase verification is clean.

1. **Task 1: Add typed scheduler eligibility suppression before mutation** - complete, not committed here.
2. **Task 2: Wire PeerManager relay eligibility into transaction download entry points** - complete, not committed here.

## Files Created/Modified

- `packages/open-bitcoin-network/src/peer/relay_download.rs` - New pure PeerManager relay download eligibility adapter and policy surface.
- `packages/open-bitcoin-network/src/peer.rs` - Adds relay download policy state and re-exports while keeping the production file under the length guard.
- `packages/open-bitcoin-network/src/peer/inventory_state.rs` - Wires relay eligibility into inventory announcements and orphan parent request scheduling.
- `packages/open-bitcoin-network/src/peer/transaction_relay.rs` - Adds public typed suppression reasons and stable action labels.
- `packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs` - Gates announcements and parent requests on relay eligibility before request-state mutation.
- `packages/open-bitcoin-network/src/peer/transaction_relay/tests*.rs` - Adds scheduler regression coverage for typed suppressions and mutation-free ineligible paths.
- `packages/open-bitcoin-network/src/peer/tests.rs` - Adds PeerManager matrix coverage for default-off, outbound, inbound, protected-only, permissioned, and fallback announcer behavior.
- `packages/open-bitcoin-network/src/lib.rs` - Re-exports `RelayDownloadPolicy` and `TxParentRequestInput`.
- `docs/parity/source-breadcrumbs.json` - Registers the new relay download module with parity breadcrumb checking.
- `docs/metrics/lines-of-code.md` - Refreshed the tracked LOC artifact for the current worktree.

## Decisions Made

- Eligibility is enforced inside `TxDownloadScheduler` and the PeerManager scheduler entry points, not only at a later managed-network translation boundary, so ineligible peers cannot leave stale scheduler state.
- `PeerManager::new` and `with_max_blocks_in_flight` remain default-off. Activation requires an explicit `RelayDownloadPolicy`.
- The new suppression labels remain fixed and low-cardinality: `relay_disabled`, `not_relay_eligible`, `inbound_serving_required`, `permission_required`, and `protected_not_relay`.
- `relay_download.rs` uses the existing `network-transaction-relay-download` breadcrumb group so the repo checker has one deterministic parity group for this behavior.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Kept `peer.rs` below the production file-length guard**
- **Found during:** Task 2
- **Issue:** Adding policy state and relay download wiring to `peer.rs` would have pushed the file toward the repo file-length guard.
- **Fix:** Moved public transaction request maintenance methods into `peer/inventory_state.rs` and kept the new eligibility adapter in `peer/relay_download.rs`.
- **Files modified:** `packages/open-bitcoin-network/src/peer.rs`, `packages/open-bitcoin-network/src/peer/inventory_state.rs`, `packages/open-bitcoin-network/src/peer/relay_download.rs`
- **Verification:** `bash scripts/check-file-lengths.sh`
- **Committed in:** Not committed by this executor per execution context.

**2. [Rule 3 - Blocking] Used the repo's existing parity breadcrumb group**
- **Found during:** Task 2
- **Issue:** The parity checker expects Rust files to match registered breadcrumb groups exactly; a one-off breadcrumb set for the new module would not be tracked consistently.
- **Fix:** Added `packages/open-bitcoin-network/src/peer/relay_download.rs` to the existing `network-transaction-relay-download` group.
- **Files modified:** `packages/open-bitcoin-network/src/peer/relay_download.rs`, `docs/parity/source-breadcrumbs.json`
- **Verification:** `bun run scripts/check-parity-breadcrumbs.ts --check`
- **Committed in:** Not committed by this executor per execution context.

**Total deviations:** 2 auto-fixed blocking issues.
**Impact on plan:** Both changes preserve the plan goal and repo verification contracts without expanding relay behavior beyond pure eligibility gating.

## Issues Encountered

- The TDD red run failed as intended before implementation because scheduler tests referenced the new `TxParentRequestInput`, relay eligibility input field, and suppression variants.
- `docs/metrics/lines-of-code.md` was regenerated from the worktree. Because the generator relies on tracked files, the newly created untracked Rust file may be counted only after the parent workflow stages or commits it and refreshes the report.

## Known Stubs

None. A targeted scan of created and modified files found no TODO, FIXME, placeholder, coming-soon text, or empty hardcoded UI/data stubs.

## Threat Flags

None. This plan adds pure in-process policy and scheduler gating only; it does not add network endpoints, auth paths, filesystem access, schema changes, service-bit changes, compact block behavior, package relay, bloom/filter serving, or public relay defaults.

## Verification

- `cargo fmt --manifest-path packages/Cargo.toml --all` - passed
- `cargo fmt --manifest-path packages/Cargo.toml --all --check` - passed
- `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-network --all-targets --all-features -- -D warnings` - passed
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network --lib transaction_relay -- --nocapture` - passed, 64 passed
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network --lib peer_manager_transaction_relay -- --nocapture` - passed, 14 passed
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network --lib` - passed, 263 passed
- `bun run scripts/check-parity-breadcrumbs.ts --check` - passed
- `bash scripts/check-file-lengths.sh` - passed
- `rg -n "relay_disabled|not_relay_eligible|inbound_serving_required|permission_required|protected_not_relay" packages/open-bitcoin-network/src/peer/transaction_relay.rs` - passed
- `bun run scripts/generate-loc-report.ts --source=worktree --output=docs/metrics/lines-of-code.md --check` - passed
- `git diff --check` - passed

## User Setup Required

None.

## Next Phase Readiness

Plan 107-01 leaves the pure runtime boundary ready for later plans to integrate managed-network translation and operator evidence. The remaining parent workflow should stage the new Rust file before any final LOC freshness pass so generated metrics include it.

## Self-Check: PASSED

- Created summary file: `.planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-01-SUMMARY.md`
- Created implementation file: `packages/open-bitcoin-network/src/peer/relay_download.rs`
- Verified required labels are present in `packages/open-bitcoin-network/src/peer/transaction_relay.rs`
- Verified parity breadcrumb registration includes `packages/open-bitcoin-network/src/peer/relay_download.rs`
- No commits were created, matching the execution context.

*Phase: 107-runtime-relay-activation-and-download-eligibility-integration*
*Completed: 2026-07-03*
