---
phase: 78-progress-guarantees-and-stall-diagnosis
plan: 04
subsystem: operator-status-support
tags: [rust, typescript, operator-status, dashboard, support-evidence, live-smoke]

requires:
  - phase: 78
    plan: 02
    provides: "Runtime progress-credit and stall diagnosis evidence on shared SyncStatus"
  - phase: 78
    plan: 03
    provides: "Soak checkpoints and reports carry shared progress guarantee evidence"
provides:
  - "CLI human status renders Phase 78 progress credit, expected window, threshold, peer contribution, and stall diagnosis fields"
  - "Dashboard Sync and Peers rows render the same shared Phase 78 fields and unavailable reasons"
  - "Support evidence and live-smoke summaries project compact Phase 78 progress/stall evidence"
affects: [operator-status, dashboard, support-evidence, live-smoke, phase-78]

tech-stack:
  added: []
  patterns:
    - "Operator renderers consume shared SyncStatus evidence through text helpers instead of reclassifying counters"
    - "Large operator projection sections can move into child modules while parent files keep documented label constants"
    - "Support evidence keeps Phase 78 status projection compact and excludes raw status or daemon bodies"

key-files:
  created:
    - packages/open-bitcoin-cli/src/operator/status/render/progress_guarantee.rs
    - packages/open-bitcoin-cli/src/operator/dashboard/model/sync_section.rs
    - packages/open-bitcoin-cli/src/operator/support/progress_guarantee.rs
    - .planning/phases/78-progress-guarantees-and-stall-diagnosis/78-04-SUMMARY.md
  modified:
    - packages/open-bitcoin-cli/src/operator/sync_truth_render.rs
    - packages/open-bitcoin-cli/src/operator/status/render.rs
    - packages/open-bitcoin-cli/src/operator/status/render/tests.rs
    - packages/open-bitcoin-cli/src/operator/dashboard/model.rs
    - packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs
    - packages/open-bitcoin-cli/src/operator/support.rs
    - packages/open-bitcoin-cli/src/operator/support/evidence.rs
    - packages/open-bitcoin-cli/src/operator/support/render.rs
    - packages/open-bitcoin-cli/src/operator/support/tests.rs
    - packages/open-bitcoin-cli/src/operator/soak/tests.rs
    - scripts/run-live-mainnet-smoke.ts
    - scripts/test-run-live-mainnet-smoke.sh
    - docs/parity/source-breadcrumbs.json
    - docs/metrics/lines-of-code.md

key-decisions:
  - "Kept CLI/dashboard wording as direct projections of typed SyncStatus fields, using shared enum serialization for labels."
  - "Added compact support summaries for progress guarantee and stall diagnosis under full_sync_evidence rather than storing raw status bodies."
  - "Extended live-smoke polling and final-status summaries with flat camelCase Phase 78 fields for fixture and support-bundle consumers."

patterns-established:
  - "Phase 78 status lines use `Sync progress credit:`, `Sync expected progress window:`, `Sync no-progress threshold:`, `Sync last useful work:`, `Sync last peer contribution:`, and `Sync stalled subsystem:`."
  - "Dashboard row labels mirror the status fields: `Progress credit`, `Expected progress window`, `No-progress threshold`, `Last useful work`, `Last peer contribution`, and `Stalled subsystem`."
  - "Support full-sync evidence summarizes progress with `credit=`, `last_useful_work=`, `expected_window=`, and `threshold=` and summarizes stalls with `stalled_subsystem=`, `confidence=`, `basis=`, and `next_action=`."

requirements-completed: [PROG-02, PROG-03]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 78-2026-06-16T14-21-42
generated_at: 2026-06-17T09:21:12Z

duration: 1h 08m
completed: 2026-06-17
---

# Phase 78-04: Operator Status, Support, and Live-Smoke Progress Evidence Summary

**Operator-facing status, dashboard, support, and live-smoke outputs now surface shared Phase 78 progress guarantees and stall diagnosis evidence.**

## Performance

- **Duration:** 1h 08m
- **Completed:** 2026-06-17T09:21:12Z
- **Tasks:** 3
- **Files modified:** 16

## Accomplishments

- Added shared render helpers for progress credit, expected progress window, no-progress threshold, last useful work, last peer contribution, and stall diagnosis.
- Added human status lines and dashboard rows that render available values and explicit unavailable reasons from shared `SyncStatus`.
- Added compact support evidence summaries and live-smoke JSON/Markdown projections for Phase 78 fields.

## Task Commits

This plan will be committed as one atomic implementation commit with this summary artifact.

## Files Created/Modified

- `packages/open-bitcoin-cli/src/operator/sync_truth_render.rs` - Added shared Phase 78 text helpers.
- `packages/open-bitcoin-cli/src/operator/status/render.rs` - Wires Phase 78 human status lines through a child projection module.
- `packages/open-bitcoin-cli/src/operator/status/render/progress_guarantee.rs` - Builds the Phase 78 human status lines.
- `packages/open-bitcoin-cli/src/operator/status/render/tests.rs` - Covers available and unavailable status rendering.
- `packages/open-bitcoin-cli/src/operator/dashboard/model.rs` - Keeps dashboard label constants and delegates Sync-and-Peers rows.
- `packages/open-bitcoin-cli/src/operator/dashboard/model/sync_section.rs` - Builds Sync-and-Peers dashboard rows including Phase 78 fields.
- `packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs` - Covers available and unavailable dashboard rows.
- `packages/open-bitcoin-cli/src/operator/support/evidence.rs` - Adds progress guarantee and stall diagnosis summaries to full-sync evidence.
- `packages/open-bitcoin-cli/src/operator/support/progress_guarantee.rs` - Builds compact support summaries for Phase 78 fields.
- `packages/open-bitcoin-cli/src/operator/support/render.rs` - Renders support Markdown lines for progress guarantee and stall diagnosis.
- `packages/open-bitcoin-cli/src/operator/support/tests.rs` - Covers support JSON, Markdown, and raw-body exclusion.
- `scripts/run-live-mainnet-smoke.ts` - Preserves Phase 78 fields in polling snapshots, final status, and Markdown.
- `scripts/test-run-live-mainnet-smoke.sh` - Adds fixture fields and assertions for live-smoke Phase 78 projection.
- `docs/parity/source-breadcrumbs.json` - Registers new Open Bitcoin-only Rust modules.
- `docs/metrics/lines-of-code.md` - Refreshed tracked LOC report.

## Decisions Made

- Rendered typed labels directly through `serde_json` label serialization instead of string parsing or counter-derived reclassification.
- Kept support output to compact summaries and explicit unavailable reasons; raw status snapshots, daemon logs, credentials, and live-smoke inputs remain excluded.
- Split large projection blocks into child modules to preserve the repo file-length guard while keeping parent files as the documented integration points.

## Deviations from Plan

### Auto-fixed Issues

**1. Production file-length guard required projection module splits**
- **Found during:** Line-count review before commit gate.
- **Issue:** Adding Phase 78 helper rows pushed existing production render files over the 628-line limit.
- **Fix:** Moved human status Phase 78 lines, dashboard Sync-and-Peers rows, and support progress summaries into child modules.
- **Verification:** `wc -l` confirmed touched production files are below the guard; focused Rust tests passed.

**2. New Rust modules required parity breadcrumb registration**
- **Found during:** New file review after module split.
- **Issue:** New first-party Rust source files under `packages/open-bitcoin-cli/src` must be listed in `docs/parity/source-breadcrumbs.json`.
- **Fix:** Added the new modules to existing Open Bitcoin-only operator groups.
- **Verification:** `bun run scripts/check-parity-breadcrumbs.ts` passed.

## Issues Encountered

- `docs/metrics/lines-of-code.md` needed regeneration after adding Rust modules.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --lib phase78_progress_guarantee --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --lib support_phase78_progress_guarantee --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --lib phase78 --all-features`
- `cargo check --manifest-path packages/Cargo.toml -p open-bitcoin-cli --all-targets --all-features`
- `bash scripts/test-run-live-mainnet-smoke.sh`
- `bun run scripts/check-parity-breadcrumbs.ts`
- Acceptance `rg` scans from 78-04 for status, dashboard, support, live-smoke, and negative raw-evidence patterns.

## User Setup Required

None.

## Next Phase Readiness

Plan 78-05 can focus on deterministic sync behavior tests knowing the shared evidence contract now reaches operator status, dashboard, support, live-smoke, and soak report surfaces.

---
*Phase: 78-progress-guarantees-and-stall-diagnosis*
*Completed: 2026-06-17*
