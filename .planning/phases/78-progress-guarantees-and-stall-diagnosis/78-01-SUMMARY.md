---
phase: 78-progress-guarantees-and-stall-diagnosis
plan: 01
subsystem: node-status
tags: [rust, serde, sync-status, progress-guarantees, stall-diagnosis]

requires:
  - phase: 70
    provides: "existing no-progress and reconcile status fields"
  - phase: 71
    provides: "shared status snapshot fixture coverage"
provides:
  - "Phase 78 progress-credit and stall-diagnosis DTO contracts"
  - "Additive SyncStatus fields with legacy-safe unavailable defaults"
  - "Node status contract coverage for Phase 78 labels and defaults"
affects: [node-status, sync-runtime, cli-status, rpc-status, phase-78]

tech-stack:
  added: []
  patterns:
    - "Shared status extensions use FieldAvailability plus serde defaults"
    - "Progress-credit labels are explicit serde renames in source"
    - "Status extension defaults live with the DTO module to keep root status files below production line-count limits"

key-files:
  created:
    - packages/open-bitcoin-node/src/status/progress_guarantee.rs
  modified:
    - packages/open-bitcoin-node/src/status.rs
    - packages/open-bitcoin-node/src/status/tests.rs
    - packages/open-bitcoin-node/src/lib.rs
    - packages/open-bitcoin-node/src/sync/types/summary.rs
    - packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs
    - packages/open-bitcoin-cli/src/operator/runtime/support.rs
    - packages/open-bitcoin-cli/src/operator/soak/tests/runtime.rs
    - packages/open-bitcoin-cli/src/operator/status/render/tests.rs
    - packages/open-bitcoin-cli/src/operator/status/sync_state.rs
    - packages/open-bitcoin-cli/src/operator/status/tests.rs
    - packages/open-bitcoin-cli/src/operator/support/tests.rs
    - packages/open-bitcoin-cli/tests/operator_binary.rs
    - packages/open-bitcoin-rpc/src/dispatch/tests.rs
    - docs/metrics/lines-of-code.md
    - docs/parity/source-breadcrumbs.json

key-decisions:
  - "Credited progress is limited to validated durable active-chain progress and current-at-best-known-tip evidence."
  - "Header, block-download, retry, in-flight, report, and peer-message activity are modeled only as rejected activity or peer contribution, never as progress credit."
  - "Summary-only status projection leaves every Phase 78 field unavailable until runtime evidence is supplied by downstream plans."

patterns-established:
  - "New status DTO modules carry explicit parity breadcrumb comments and source-breadcrumb registry entries."
  - "Legacy SyncStatus compatibility tests assert exact unavailable reasons for additive fields."

requirements-completed: [PROG-01, PROG-02, PROG-03, PROG-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 78-2026-06-16T14-21-42
generated_at: 2026-06-17T05:35:58Z

duration: 2h 34m
completed: 2026-06-17
---

# Phase 78-01: Progress Guarantee Status Contract Summary

**Shared node status now has typed progress-credit, progress-window, peer-contribution, threshold, and stall-diagnosis contracts with legacy-safe defaults.**

## Performance

- **Duration:** 2h 34m
- **Started:** 2026-06-17T03:02:00Z
- **Completed:** 2026-06-17T05:35:58Z
- **Tasks:** 2
- **Files modified:** 19

## Accomplishments

- Added `progress_guarantee.rs` with Phase 78 DTOs and explicit serde labels for every new enum.
- Extended `SyncStatus` with six additive `FieldAvailability` fields and stable unavailable reasons.
- Updated node status fixtures, summary-only status projection, public re-exports, and parity breadcrumbs.
- Added minimal CLI/RPC constructor defaults required for the workspace to keep compiling after the shared `SyncStatus` expansion.
- Moved Phase 78 defaults and `NoProgressDiagnosis` into the progress-guarantee module so production files stay under the repo line-count guard.
- Added `phase78_progress_guarantee_status_contract` covering labels, legacy defaults, and the no-header/download-credit invariant.

## Task Commits

Implementation is staged for a single local plan commit after summary and GSD state updates. The stalled executor did not produce task commits.

## Files Created/Modified

- `packages/open-bitcoin-node/src/status/progress_guarantee.rs` - Progress-credit, progress-window, threshold, peer-contribution, and stall-diagnosis DTOs.
- `packages/open-bitcoin-node/src/status.rs` - New module export, serde defaults, and `SyncStatus` fields.
- `packages/open-bitcoin-node/src/status/tests.rs` - Contract and fixture coverage for Phase 78 status additions.
- `packages/open-bitcoin-node/src/lib.rs` - Public re-exports for the new status DTOs.
- `packages/open-bitcoin-node/src/sync/types/summary.rs` - Summary-only status projection defaults for the new fields.
- `packages/open-bitcoin-cli/src/operator/status/sync_state.rs` - Live RPC and unavailable sync-status constructors now fill the new unavailable fields.
- `packages/open-bitcoin-cli/src/operator/runtime/support.rs` - Sync-control runtime fixture now fills the new unavailable fields.
- `packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs` - Dashboard status fixtures now fill the new unavailable fields.
- `packages/open-bitcoin-cli/src/operator/status/render/tests.rs` - Status renderer fixture now fills the new unavailable fields.
- `packages/open-bitcoin-cli/src/operator/status/tests.rs` - Status collector fixture now fills the new unavailable fields.
- `packages/open-bitcoin-cli/src/operator/soak/tests/runtime.rs` - Soak runtime fixture now fills the new unavailable fields.
- `packages/open-bitcoin-cli/src/operator/support/tests.rs` - Support bundle fixture now fills the new unavailable fields.
- `packages/open-bitcoin-cli/tests/operator_binary.rs` - Operator binary fixture now fills the new unavailable fields.
- `packages/open-bitcoin-rpc/src/dispatch/tests.rs` - RPC dispatch fixtures now fill the new unavailable fields.
- `docs/metrics/lines-of-code.md` - Regenerated by the repo verification hook after file-count changes.
- `docs/parity/source-breadcrumbs.json` - Breadcrumb registry coverage for the new Rust source.

## Decisions Made

- Used explicit `#[serde(rename = "...")]` labels on the new enums so both runtime JSON and source-level acceptance checks expose the contract.
- Kept Phase 78 unavailable defaults in `status/progress_guarantee.rs` with the DTOs instead of the root `status.rs`; this preserves the thin-entrypoint module shape and satisfies the 628-line production file limit.
- Kept runtime evidence unavailable in `SyncRunSummary::sync_status`; populating real evidence is intentionally left to downstream Phase 78 runtime plans.
- Modeled rejected progress activity separately from credited progress so status consumers cannot treat header-only or download-only work as forward progress.

## Deviations from Plan

The first executor agent stalled without source changes, so the plan was executed directly in the main workspace. Both plan tasks were implemented as one local batch to avoid committing half-wired shared status shapes.

Full-workspace clippy requires downstream `SyncStatus` struct literals to compile even though CLI/RPC rendering work is planned in `78-07`, so this plan added unavailable defaults to existing CLI/RPC constructors and fixtures. No downstream renderer behavior or runtime evidence projection was added.

The first commit attempt failed the repository line-count guard after `status.rs` and `operator/runtime/support.rs` exceeded 628 lines. The fix moved Phase 78 defaults into `status/progress_guarantee.rs` and compacted the local CLI runtime test fixture while preserving the same status values.

## Issues Encountered

- The first filtered test run buffered its final harness output after the binary completed; polling plus a process check confirmed the test itself passed.
- Source-level acceptance checks initially could not see labels generated by `serde(rename_all)`, so labels were made explicit and reverified.
- The required all-target clippy run exposed missing `SyncStatus` fields in downstream CLI/RPC constructors; those were patched with unavailable defaults before full verification.
- The commit hook line-count guard failed on `status.rs` and `operator/runtime/support.rs`; both files are now below the 628-line limit.

## Verification

- `cargo fmt --manifest-path packages/Cargo.toml --all`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib phase78_progress_guarantee_status_contract --all-features`
- `cargo check --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-targets --all-features`
- `cargo clippy --manifest-path packages/Cargo.toml --all-targets --all-features -- -D warnings`
- `cargo build --manifest-path packages/Cargo.toml --all-targets --all-features`
- `cargo test --manifest-path packages/Cargo.toml --all-features`
- Acceptance greps for new DTOs, `SyncStatus` fields, public exports, and parity breadcrumb entry.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

The shared status contract is ready for Plan 78-02 to populate runtime evidence and Plan 78-07 to update downstream CLI/RPC status constructors.

---
*Phase: 78-progress-guarantees-and-stall-diagnosis*
*Completed: 2026-06-17*
