---
phase: 78-progress-guarantees-and-stall-diagnosis
plan: 02
subsystem: sync-runtime
tags: [rust, sync-runtime, progress-guarantees, stall-diagnosis, structured-logs]

requires:
  - phase: 78
    plan: 01
    provides: "Phase 78 shared status DTO fields"
provides:
  - "Pure progress-credit and stall-diagnosis classifiers"
  - "Durable runtime projection for Phase 78 progress-guarantee status fields"
  - "Status-derived structured log evidence for progress guarantees"
affects: [sync-runtime, node-status, structured-logs, phase-78]

tech-stack:
  added: []
  patterns:
    - "Progress credit is classified from validated active-chain height/hash/work or explicit current-at-best-known-tip evidence"
    - "Rejected activity records headers, block responses, messages, retries, report projections, and in-flight work without crediting progress"
    - "Runtime structured logs derive compact labels from shared status fields after projection"

key-files:
  created:
    - packages/open-bitcoin-node/src/sync/progress/guarantee.rs
    - packages/open-bitcoin-node/src/sync/progress/tests.rs
    - packages/open-bitcoin-node/src/sync/runtime_state/recovery.rs
  modified:
    - packages/open-bitcoin-node/src/sync/progress.rs
    - packages/open-bitcoin-node/src/sync/runtime_state.rs
    - packages/open-bitcoin-node/src/sync.rs
    - packages/open-bitcoin-node/src/sync/tests.rs
    - docs/metrics/lines-of-code.md
    - docs/parity/source-breadcrumbs.json

key-decisions:
  - "Header-only and block-response-only activity no longer sets useful-progress timestamps in durable runtime status."
  - "Previous durable last-useful-work credit is carried forward when the current run has no new active-chain credit."
  - "Storage/resource recovery evidence outranks peer retry evidence in the typed stall diagnosis."
  - "The existing runtime recovery-category helper was moved to a child module to keep runtime_state.rs below the repo line-count guard."

patterns-established:
  - "ProgressGuaranteeInput centralizes typed runtime facts for all six Phase 78 status fields."
  - "Runtime logs use shared status fields as their source of truth instead of summary counters or renderer strings."

requirements-completed: [PROG-01, PROG-02, PROG-03]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 78-2026-06-16T14-21-42
generated_at: 2026-06-17T06:08:41Z

duration: 32m
completed: 2026-06-17
---

# Phase 78-02: Runtime Progress Evidence Projection Summary

**Durable sync status now projects Phase 78 progress guarantees from validated active-chain and stay-current evidence rather than header, block-response, or retry counters.**

## Performance

- **Duration:** 32m
- **Completed:** 2026-06-17T06:08:41Z
- **Tasks:** 3
- **Files modified:** 8

## Accomplishments

- Added pure progress-guarantee classifiers for credited progress, last useful work, expected progress windows, no-progress thresholds, latest peer contribution, and typed stall diagnosis.
- Replaced runtime `made_useful_progress` input with `made_validated_durable_progress` based on validated active-chain height/hash/work compared to prior durable credit.
- Populated `progress_credit`, `expected_progress_window`, `no_progress_threshold`, `last_useful_work`, `last_peer_contribution`, and `stall_diagnosis` in `DurableSyncRuntime::durable_sync_state_from_summary`.
- Set `last_successful_progress_unix_seconds` from `last_useful_work.source_unix_seconds` or carried prior durable status, never from header-only or block-response-only peer counters.
- Added a compact `write_progress_guarantee_log` line derived from projected status fields, with log-write failures added as health signals before persistence.
- Added focused classifier and projection tests covering false-progress rejection, at-tip credit, previous-credit carry-forward, and storage-first stall diagnosis.

## Files Created/Modified

- `packages/open-bitcoin-node/src/sync/progress.rs` - Phase 78 input contract and wrapper helpers beside existing no-progress classification.
- `packages/open-bitcoin-node/src/sync/progress/guarantee.rs` - Pure progress-credit, threshold, peer-contribution, and stall-diagnosis implementation.
- `packages/open-bitcoin-node/src/sync/progress/tests.rs` - Moved existing no-progress tests plus new Phase 78 classifier coverage.
- `packages/open-bitcoin-node/src/sync/runtime_state.rs` - Durable runtime projection and status-derived progress-guarantee log writer.
- `packages/open-bitcoin-node/src/sync/runtime_state/recovery.rs` - Existing durable recovery-category projection moved out of the line-count constrained runtime file.
- `packages/open-bitcoin-node/src/sync.rs` - Sync persistence path writes the compact Phase 78 status-derived log after projection.
- `packages/open-bitcoin-node/src/sync/tests.rs` - Runtime projection regression coverage and stricter headers-only stay-current expectation.
- `docs/metrics/lines-of-code.md` - Regenerated after adding new Rust modules.
- `docs/parity/source-breadcrumbs.json` - Breadcrumb registry entries for the new Rust modules.

## Decisions Made

- Kept summary-only `SyncRunSummary::sync_status` conservative; durable runtime projection is the first layer that has enough evidence to make Phase 78 fields available.
- Treated repeated active-chain evidence as no new progress unless the node is explicitly current at the best-known validated tip.
- Logged only bounded labels and status-derived counters for Phase 78 evidence; raw peer tables and local sensitive data are not emitted.

## Deviations from Plan

The plan listed `progress.rs`, `runtime_state.rs`, and `sync.rs` as primary files. To satisfy the repo line-count guard, the detailed pure classifier implementation and an existing recovery helper were placed in child modules while the required entry points remain in the planned files.

## Issues Encountered

- The first classifier compile pass needed an explicit lifetime on the latest-peer-failure helper.
- Moving the exact `ProgressGuaranteeInput` contract exposed an otherwise unused `progress_signal` and `maybe_stop_reason`; both now participate in rejected retry evidence or no-progress classification.
- The older headers-only stay-current regression expected `InitialCatchUp`; Phase 78 correctly tightens that to `NoProgress` because header evidence is no longer useful active-chain progress.
- The first `bash scripts/verify.sh` run found a stale LOC report; regenerating `docs/metrics/lines-of-code.md` resolved it.

## Verification

- `cargo fmt --manifest-path packages/Cargo.toml --all`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib phase78_progress_guarantee_classifier --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase78_progress_guarantee_projection --all-features`
- `cargo check --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-targets --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib phase69_headers_only_tip_does_not_report_current --all-features`
- `bash scripts/verify.sh`
- Acceptance greps for the new helpers, runtime projection assignments, retry-backoff conversion, structured log keys, and removal of header/block counters from useful-progress projection.

## User Setup Required

None.

## Next Phase Readiness

Runtime status now exposes typed progress/stall evidence for downstream CLI/RPC surfaces and later deterministic verification plans.

---
*Phase: 78-progress-guarantees-and-stall-diagnosis*
*Completed: 2026-06-17*
