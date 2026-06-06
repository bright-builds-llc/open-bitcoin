---
phase: 62-long-run-sync-truth-surfaces
plan: 01
subsystem: observability
tags: [rust, sync-status, durable-sync, structured-logs, metrics]

requires:
  - phase: 61-resource-bounds-and-recovery-taxonomy
    provides: typed recovery category, recovery action, resource pressure, and bounded structured-log expectations
provides:
  - Phase 62 shared sync truth contract fields for configured targets, attempt counters, and latest stop reason
  - Durable sync projection from runtime config and summary facts into shared status
  - Bounded structured sync log cycle facts for targets, attempts, stop reason, recovery category, progress, and counters
  - Deterministic Rust coverage for legacy status defaults, summary projection, bounded logs, and status/log target agreement
affects: [operator-status, dashboard, rpc, support-bundles, live-smoke, phase-62]

tech-stack:
  added: []
  patterns:
    - Additive serde-defaulted FieldAvailability status fields for older persisted metadata compatibility
    - Runtime-stamped summary projections for config-derived durable sync facts
    - Bounded structured-log records split by fact family when a single line would exceed the Phase 61 cap

key-files:
  created:
    - .planning/phases/62-long-run-sync-truth-surfaces/62-01-SUMMARY.md
  modified:
    - packages/open-bitcoin-node/src/status.rs
    - packages/open-bitcoin-node/src/sync/types.rs
    - packages/open-bitcoin-node/src/sync/types/summary.rs
    - packages/open-bitcoin-node/src/sync/runtime_state.rs
    - packages/open-bitcoin-node/src/sync/tests.rs
    - packages/open-bitcoin-cli/src/operator/status/sync_state.rs
    - packages/open-bitcoin-cli/src/operator/status/tests.rs
    - packages/open-bitcoin-cli/src/operator/status/render.rs
    - packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs
    - packages/open-bitcoin-cli/src/operator/runtime/support.rs
    - packages/open-bitcoin-rpc/src/dispatch/tests.rs

key-decisions:
  - "Keep configured target, attempt counter, and stop-reason facts in the shared Rust status contract as FieldAvailability values with explicit unavailable defaults."
  - "Stamp durable summaries with runtime config target header height at status, metric, and structured-log projection boundaries without widening the assigned write scope into sync.rs."
  - "Split Phase 62 structured cycle facts across two bounded sync info records to preserve the existing 192-character message cap."

patterns-established:
  - "Missing additive status fields default through serde functions with stable unavailable reasons for older RuntimeMetadata compatibility."
  - "Metrics remain numeric-only while structured logs carry compact machine-stable labels for non-numeric cycle facts."

requirements-completed: [OBS-01, OBS-02]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 62-2026-06-06T19-46-48
generated_at: 2026-06-06T21:12:12Z

duration: 19m 4s
completed: 2026-06-06
---

# Phase 62 Plan 01: Sync Truth Contract Summary

**Typed sync status contract with durable target/attempt/stop-reason projection and bounded structured cycle facts**

## Performance

- **Duration:** 19m 4s
- **Started:** 2026-06-06T20:53:08Z
- **Completed:** 2026-06-06T21:12:12Z
- **Tasks:** 2
- **Files modified:** 12

## Accomplishments

- Added `SyncConfiguredTargets`, `SyncAttemptCounters`, and `SyncStopReasonStatus` to the shared status contract with explicit serde defaults for older persisted JSON.
- Projected configured targets, attempt counters, latest stop reason, and runtime `max_sync_rounds` through summary and durable sync state.
- Kept metrics numeric-only and added bounded structured-log cycle labels for configured targets, attempts, progress counters, stop reason, and recovery category.
- Added deterministic Phase 62 Rust tests for contract defaults, summary projection, bounded log labels, and durable status/log target agreement.

## Task Commits

1. **Task 1: Add Phase 62 fields to the shared status contract** - `08af31d` (feat)
2. **Task 2: Project compact cycle facts into status, metrics, and structured logs** - `7270c2b` (feat)

## TDD Evidence

- **Task 1 RED:** `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase62_sync_truth_contract --all-features` failed on missing `SyncConfiguredTargets`, `SyncAttemptCounters`, `SyncStopReasonStatus`, `SyncStatus` fields, and `SyncRunSummary::maybe_target_header_height`.
- **Task 1 GREEN:** same filter passed with 2 tests after adding the contract, defaults, projections, and constructor updates.
- **Task 2 RED:** `phase62_structured_logs_keep_bounded_cycle_facts` failed on missing `target_outbound_peers=4`; `phase62_status_and_structured_logs_agree_on_configured_targets` failed on missing `target_header_height=840123`.
- **Task 2 GREEN:** both Task 2 filters passed after extending structured-log projection.

## Verification

- `cargo check --manifest-path packages/Cargo.toml --workspace --all-targets --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase62_sync_truth_contract --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase62_structured_logs_keep_bounded_cycle_facts --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase62_status_and_structured_logs_agree_on_configured_targets --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node sync_summary_projects_metric_samples --all-features`
- `cargo fmt --manifest-path packages/Cargo.toml --all`
- `cargo clippy --manifest-path packages/Cargo.toml --all-targets --all-features -- -D warnings`
- `cargo build --manifest-path packages/Cargo.toml --all-targets --all-features`
- `cargo test --manifest-path packages/Cargo.toml --all-features`

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - Missing Critical Boundedness] Split oversized Phase 62 log facts into two bounded records**
- **Found during:** Task 2
- **Issue:** The plan asked for one compact summary record containing every Phase 62 label, but the required label names alone exceed the existing 192-character structured-message cap verified by Phase 61 tests.
- **Fix:** Emitted two sync `Info` summary records: one for configured targets, attempts, stop reason, and recovery category; one for progress/counter facts. Tests assert every required label is present and every record remains at or below 192 characters.
- **Files modified:** `packages/open-bitcoin-node/src/sync/types/summary.rs`, `packages/open-bitcoin-node/src/sync/tests.rs`
- **Verification:** `phase62_structured_logs_keep_bounded_cycle_facts`, full `cargo test --all-features`
- **Committed in:** `7270c2b`

**Total deviations:** 1 auto-fixed (Rule 2)
**Impact on plan:** Preserves the plan's machine-stable log facts and Phase 61 bounded-retention invariant without adding unbounded arrays or new observability storage.

## Issues Encountered

- `cargo fmt --all` from the repo root failed because this repository's Cargo workspace manifest is `packages/Cargo.toml`; reran successfully as `cargo fmt --manifest-path packages/Cargo.toml --all`.
- `.planning/STATE.md` and `.planning/config.json` were already modified in the worktree context; they were left unstaged and unmodified by this executor per the orchestrator scope instruction.

## Known Stubs

None. Stub scan found only format strings and intentional `FieldAvailability::unavailable(...)` reasons used as operator-visible missing-data semantics.

## Threat Surface

No new network endpoint, authentication path, file-access pattern, or schema trust boundary was introduced. The plan mitigated the existing durable runtime -> status and status -> logs/metrics boundaries with typed fields, explicit unavailable defaults, and bounded log records.

## User Setup Required

None.

## Next Phase Readiness

Plan 62-02 can render and compare the shared truth fields without re-inferring configured targets, attempt counters, or stop reasons from human text. Metrics remain the same five numeric sync series.

## Self-Check: PASSED

- FOUND: `.planning/phases/62-long-run-sync-truth-surfaces/62-01-SUMMARY.md`
- FOUND: `08af31d` (`feat(62-01): add sync truth status contract`)
- FOUND: `7270c2b` (`feat(62-01): project bounded sync cycle log facts`)

---
*Phase: 62-long-run-sync-truth-surfaces*
*Completed: 2026-06-06*
