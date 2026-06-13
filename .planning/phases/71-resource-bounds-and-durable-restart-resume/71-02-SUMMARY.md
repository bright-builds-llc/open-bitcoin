---
phase: 71-resource-bounds-and-durable-restart-resume
plan: 02
subsystem: sync-runtime
tags: [sync, restart-resume, resource-bounds, synthetic-long-chain]

requires: [71-CONTEXT, 71-RESEARCH]
provides:
  - Deterministic same-datadir restart/resume matrix coverage
  - Synthetic 48-block long-chain resource-bound coverage
  - Reopen proof that connected blocks are not requested again
affects: [phase-71, sync-runtime, durable-storage, verification]

tech-stack:
  added: []
  patterns:
    - Restart/resume tests use real `FjallNodeStore` reopen and scripted transport
    - Synthetic long-chain tests stress bounded config without public peers or DNS seeds
    - Stale in-flight evidence is checked through shared no-progress status fields

key-files:
  created:
    - .planning/phases/71-resource-bounds-and-durable-restart-resume/71-02-SUMMARY.md
  modified:
    - packages/open-bitcoin-node/src/sync/tests.rs

key-decisions:
  - "Use exactly 48 synthetic blocks for the deterministic long-chain fixture."
  - "Prove same-datadir resume through real store reopen, not service text or public-network timing."

patterns-established:
  - "Phase 71 restart cases are named in one matrix test: clean shutdown, unclean shutdown, mid-download interruption, mid-connect interruption, and stale in-flight after reopen."
  - "Synthetic long-chain proof asserts configured peer, in-flight, message, round, metrics, and log bounds."

requirements-completed: [RES-01, RES-02, RES-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 71-2026-06-13T10-34-37
generated_at: 2026-06-13T11:23:00Z

duration: 25min
completed: 2026-06-13
---

# Phase 71 Plan 02: Restart/Resume Matrix and Synthetic Long-Chain Summary

**Sync runtime tests now prove same-datadir resume across interruption modes and a deterministic 48-block long-chain resource-bound path without public-network access.**

## Performance

- **Duration:** 25 min
- **Started:** 2026-06-13T10:58:00Z
- **Completed:** 2026-06-13T11:23:00Z
- **Tasks:** 2
- **Files modified:** 1
- **Files created:** 1

## Accomplishments

- Added `phase71_same_datadir_resume_matrix_covers_clean_unclean_mid_download_mid_connect_and_stale_inflight`.
- Covered clean shutdown, unclean shutdown, mid-download interruption, mid-connect interruption, and stale in-flight after reopen in one deterministic matrix.
- Added `phase71_synthetic_long_chain_exercises_resource_bounds_without_public_network`.
- Proved the synthetic path uses 48 local blocks, manual local peers, no DNS seeds, bounded in-flight/message/round settings, default metrics retention, and default structured log retention.
- Verified reopen preserves connected progress and does not request an already connected block again.

## Task Commits

Task commits are pending the wrapper-owned final commit after full phase verification.

1. **Task 1: Same-datadir restart/resume matrix** - `pending final wrapper commit`
2. **Task 2: Synthetic long-chain resource-bound proof** - `pending final wrapper commit`

## Files Created/Modified

- `packages/open-bitcoin-node/src/sync/tests.rs` - Added the Phase 71 restart/resume matrix and synthetic long-chain resource-bound tests.

## Decisions Made

- Kept the tests in the existing sync fixture module to reuse `DurableSyncRuntime`, `FjallNodeStore`, `ScriptedTransport`, and existing block builders.
- Avoided public peers, DNS seeds, sleeps, service managers, and timing thresholds in default verification coverage.

## Deviations from Plan

None.

## Issues Encountered

- None after local takeover. Both targeted Phase 71 sync tests passed.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase71_same_datadir_resume_matrix --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase71_synthetic_long_chain --all-features`
- Acceptance `rg` checks for matrix case labels, exact test names, `48`, `target_outbound_peers: 2`, `max_blocks_in_flight_total: 4`, `max_messages_per_peer: 12`, and `max_rounds: 32`.

## User Setup Required

None - all checks are deterministic and local.

## Next Phase Readiness

Plan 71-04 can document and checker-gate the exact restart/resume and synthetic long-chain test names.

## Self-Check: PASSED

- Summary file exists.
- Required Plan 02 tests exist and pass.
- The tests do not require public-network access.

*Phase: 71-resource-bounds-and-durable-restart-resume*
*Completed: 2026-06-13*
