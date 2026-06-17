---
phase: 78-progress-guarantees-and-stall-diagnosis
plan: 07
subsystem: cli-rpc-status
tags: [rust, cli-status, rpc-status, sync-status, progress-guarantees]

requires:
  - phase: 78
    plan: 01
    provides: "Phase 78 shared SyncStatus DTO fields"
provides:
  - "Downstream CLI and RPC SyncStatus constructors with explicit Phase 78 unavailable evidence"
  - "Fixture coverage proving exact unavailable reasons for additive progress-guarantee fields"
  - "Negative acceptance evidence that downstream fixtures do not fabricate progress credit"
affects: [cli-status, rpc-status, soak-status, phase-78]

tech-stack:
  added: []
  patterns:
    - "Downstream status constructors use FieldAvailability::unavailable with shared Phase 78 reasons until runtime projection supplies evidence"
    - "Legacy operator and RPC fixtures preserve additive field absence as explicit unavailable evidence"

key-files:
  created:
    - .planning/phases/78-progress-guarantees-and-stall-diagnosis/78-07-SUMMARY.md
  modified:
    - packages/open-bitcoin-cli/src/operator/status/sync_state.rs
    - packages/open-bitcoin-cli/src/operator/runtime/support.rs
    - packages/open-bitcoin-cli/src/operator/status/tests.rs
    - packages/open-bitcoin-cli/src/operator/status/render/tests.rs
    - packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs
    - packages/open-bitcoin-cli/src/operator/support/tests.rs
    - packages/open-bitcoin-cli/src/operator/soak/tests/runtime.rs
    - packages/open-bitcoin-cli/tests/operator_binary.rs
    - packages/open-bitcoin-rpc/src/dispatch/tests.rs

key-decisions:
  - "Kept all downstream Phase 78 fields unavailable in constructor fixtures; real evidence remains owned by durable runtime projection."
  - "Accepted that 78-01 already absorbed the planned constructor fallout to keep all-target clippy and verification green after the shared DTO expansion."

patterns-established:
  - "Dependent plans may summarize and verify work absorbed by an earlier compile-preserving plan when the acceptance contract is still independently checked."

requirements-completed: [PROG-01, PROG-02, PROG-03, PROG-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 78-2026-06-16T14-21-42
generated_at: 2026-06-17T07:02:16Z

duration: 12m
completed: 2026-06-17
---

# Phase 78-07: Downstream CLI/RPC Status Constructor Summary

**CLI and RPC status fixtures preserve Phase 78 progress-guarantee fields as explicit unavailable evidence until runtime projection supplies real values.**

## Performance

- **Duration:** 12m
- **Completed:** 2026-06-17T07:02:16Z
- **Tasks:** 1
- **Files modified:** 1 summary artifact; downstream code changes were already committed with 78-01

## Accomplishments

- Verified every downstream CLI/RPC `SyncStatus` constructor compiles with the six additive Phase 78 fields.
- Verified legacy operator and RPC fixtures use the exact shared unavailable reasons for progress credit, last useful work, and stall diagnosis.
- Verified no downstream CLI/RPC fixture fabricates progress credit from headers, downloaded block bodies, report generation, retries, or in-flight requests.

## Task Commits

The downstream constructor updates were intentionally absorbed by the 78-01 commit because full-workspace clippy could not pass with missing additive `SyncStatus` fields. This plan records the independent 78-07 verification and summary artifact for the formal dependency graph.

## Files Created/Modified

- `.planning/phases/78-progress-guarantees-and-stall-diagnosis/78-07-SUMMARY.md` - Completion evidence for the downstream CLI/RPC constructor plan.
- `packages/open-bitcoin-cli/src/operator/status/sync_state.rs` - Already contains unavailable Phase 78 fields for RPC-derived and unavailable sync status.
- `packages/open-bitcoin-cli/src/operator/runtime/support.rs` - Already contains unavailable Phase 78 fields in sync-control runtime fixtures.
- `packages/open-bitcoin-cli/src/operator/status/tests.rs` - Already asserts the exact unavailable reasons.
- `packages/open-bitcoin-cli/src/operator/support/tests.rs` - Already asserts the exact unavailable reasons.
- `packages/open-bitcoin-rpc/src/dispatch/tests.rs` - Already contains unavailable Phase 78 fields in RPC status fixtures.

## Decisions Made

- Treated this as a no-code plan execution because the required downstream constructor edits had already landed under 78-01 to preserve compile correctness after the shared status schema expanded.
- Kept the 78-07 verification separate so later plans can depend on a concrete summary and acceptance evidence.

## Deviations from Plan

The planned code edits were not repeated in this execution because they were already made in 78-01. The acceptance surface was still run exactly for 78-07.

## Issues Encountered

None.

## Verification

- `rg -n "progress_credit|expected_progress_window|no_progress_threshold|last_useful_work|last_peer_contribution|stall_diagnosis" packages/open-bitcoin-cli/src/operator/status/sync_state.rs packages/open-bitcoin-cli/src/operator/runtime/support.rs packages/open-bitcoin-rpc/src/dispatch/tests.rs`
- `rg -n "progress credit evidence unavailable|last useful work unavailable|stall diagnosis unavailable" packages/open-bitcoin-cli/src/operator/status/tests.rs packages/open-bitcoin-cli/src/operator/support/tests.rs packages/open-bitcoin-rpc/src/dispatch/tests.rs`
- `rg -n "headers_received.*progress_credit|blocks_received.*progress_credit|report generation.*progress_credit" packages/open-bitcoin-cli packages/open-bitcoin-rpc -g'*.rs'` returned no matches
- `cargo check --manifest-path packages/Cargo.toml -p open-bitcoin-cli -p open-bitcoin-rpc --all-targets --all-features`

## User Setup Required

None.

## Next Phase Readiness

Plan 78-03 can now depend on a completed downstream constructor summary while it carries Phase 78 progress and stall evidence through soak checkpoints and reports.

---
*Phase: 78-progress-guarantees-and-stall-diagnosis*
*Completed: 2026-06-17*
