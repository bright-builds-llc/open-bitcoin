---
phase: 78-progress-guarantees-and-stall-diagnosis
plan: 05
subsystem: sync-runtime-tests
tags: [rust, sync, tests, progress-guarantee, stall-diagnosis]

requires:
  - phase: 78
    plan: 02
    provides: "Runtime progress-credit and stall diagnosis evidence on shared SyncStatus"
provides:
  - "Deterministic Rust tests prove header/download evidence does not advance progress credit without durable active-chain progress"
  - "Deterministic Rust tests cover stale in-flight cleanup, peer rotation/backoff, branch competition, at-tip waiting, validation stalls, storage precedence, operator stop, and local shutdown classifications"
  - "Synthetic soak fixtures cover progress-credit false-positive prevention without public peers or multi-day wall-clock sleeps"
affects: [sync-runtime, sync-status, synthetic-soak, phase-78]

tech-stack:
  added: []
  patterns:
    - "Phase 78 tests assert typed status fields directly instead of renderer output"
    - "Serialized enum label assertions guard the external JSON vocabulary for progress and stall evidence"
    - "Prior persisted credit is used to prove repeated evidence does not fabricate new progress credit"

key-files:
  created:
    - .planning/phases/78-progress-guarantees-and-stall-diagnosis/78-05-SUMMARY.md
  modified:
    - packages/open-bitcoin-node/src/sync/tests.rs
    - packages/open-bitcoin-node/src/sync/tests/soak.rs

key-decisions:
  - "Used existing scripted peer and synthetic soak fixtures rather than adding public-network or wall-clock soak tests."
  - "Compared serialized labels for `header_download`, `block_download`, `in_flight_request`, and stall subsystem labels so the tests guard the machine contract."
  - "Kept peer rotation assertions scoped to existing retry/backoff behavior and avoided peer-governance scope such as bans, reputation, addrman, or compact blocks."

patterns-established:
  - "False-progress tests persist a prior useful-work credit, then assert subsequent header/download/in-flight evidence leaves `progress_credit` unavailable and preserves `last_useful_work`."
  - "Stall classification tests assert both no-progress diagnosis and final `StalledSubsystem` where peer failure precedence intentionally affects the stall diagnosis."
  - "Local operator pause and shutdown are checked as distinct `operator_stop` and `local_shutdown` stall subsystem labels."

requirements-completed: [PROG-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 78-2026-06-16T14-21-42
generated_at: 2026-06-17T10:40:16Z

duration: 1h 15m
completed: 2026-06-17
---

# Phase 78-05: Deterministic Progress Guarantee Test Summary

**Phase 78 now has deterministic Rust coverage for false-progress prevention and stall diagnosis behavior without public peers, service managers, or long wall-clock runs.**

## Performance

- **Duration:** 1h 15m
- **Completed:** 2026-06-17T10:40:16Z
- **Tasks:** 3
- **Files modified:** 2

## Accomplishments

- Added `phase78_header_and_download_activity_do_not_credit_soak_progress` to synthetic soak fixtures.
- Added runtime tests for branch competition, current-at-tip credit, stale in-flight cleanup, no-credit peer rotation, validation stalls, storage/resource precedence, operator stop, and local shutdown.
- Added helper assertions for progress credit, last useful work, last peer contribution, stall diagnosis, rejected activity, and serialized status labels.

## Task Commits

This plan will be committed as one atomic implementation commit with this summary artifact.

## Files Created/Modified

- `packages/open-bitcoin-node/src/sync/tests.rs` - Added Phase 78 runtime status and peer-rotation classification tests.
- `packages/open-bitcoin-node/src/sync/tests/soak.rs` - Added synthetic soak false-progress coverage.
- `.planning/phases/78-progress-guarantees-and-stall-diagnosis/78-05-SUMMARY.md` - Records plan evidence and outcomes.

## Decisions Made

- Tested status fields directly from `DurableSyncRuntime::durable_sync_state_for_summary` and persisted runtime metadata instead of testing renderer strings.
- Used stale-tip time evidence to prevent accidental `current_at_best_known_tip` credit in stale in-flight tests while preserving the intended no-progress branch.
- Treated storage/resource recovery category as higher-priority evidence than peer retry/backoff, matching the Phase 78 classifier precedence.

## Deviations from Plan

### Auto-fixed Issues

**1. False-progress fixtures initially produced valid at-tip credit**
- **Found during:** `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase78_ --all-features`
- **Issue:** Some repeated active-chain fixtures were still current at the best-known tip, which is valid `current_at_best_known_tip` credit.
- **Fix:** Adjusted fixtures to represent either an unconnected better header or a stale best-known tip, depending on the no-progress branch under test.
- **Verification:** The filtered Phase 78 test run passed.

**2. Validation stall evidence includes recovery category**
- **Found during:** Focused Phase 78 test run.
- **Issue:** The validation-stall test expected only peer failure evidence, but production evidence also records `recovery_category=invalid_peer_data`.
- **Fix:** Updated the expected basis to match the production bounded evidence.
- **Verification:** `phase78_validation_stall_classifies_validation_subsystem` passed.

## Issues Encountered

- None remaining.

## Verification

- `cargo fmt --manifest-path packages/Cargo.toml --all`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase78_ --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase78_header_and_download_activity_do_not_credit_soak_progress --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase78_current_at_tip_credits_stay_current_useful_work --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase78_stale_inflight_cleanup_preserves_prior_credit_and_rotates_peer --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase78_no_credit_peer_rotation_keeps_last_peer_contribution_without_credit --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase78_validation_stall_classifies_validation_subsystem --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase78_storage_resource_pressure_outranks_peer_retry_advice --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase78_operator_stop_and_shutdown_classify_local_subsystems --all-features`
- Acceptance `rg` scans for required test names, labels, rejected activity anchors, and negative peer-governance scope.
- `cargo clippy --manifest-path packages/Cargo.toml --all-targets --all-features -- -D warnings`
- `cargo build --manifest-path packages/Cargo.toml --all-targets --all-features`
- `cargo test --manifest-path packages/Cargo.toml --all-features`

## User Setup Required

None.

## Next Phase Readiness

Plan 78-06 can close Phase 78 with documentation, parity roots, deterministic checker wiring, and final verification evidence now that PROG-04 has deterministic Rust coverage.

---
*Phase: 78-progress-guarantees-and-stall-diagnosis*
*Completed: 2026-06-17*
