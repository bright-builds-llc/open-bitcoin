---
phase: 72-operator-observability-and-support-evidence
plan: 01
subsystem: operator-status
tags: [status, dashboard, rpc, observability, parity]

requires: [72-CONTEXT, 72-RESEARCH, 72-UI-SPEC]
provides:
  - CLI human status projection for Phase 72 full-sync truth fields
  - Dashboard row projection for best-known tip, stay-current, no-progress, reorg, reconcile, pressure, and validated active-chain progress
  - RPC regression coverage proving Open Bitcoin sync status preserves durable evidence while baseline getblockchaininfo omits support-only fields
affects: [phase-72, operator-status, dashboard, rpc-status, parity-breadcrumbs]

tech-stack:
  added: []
  patterns:
    - Shared sync truth text helper consumed by CLI status and dashboard renderers
    - Exact renderer tests for available and unavailable Phase 72 evidence paths
    - Baseline RPC compatibility regression for support-only field exclusions

key-files:
  created:
    - packages/open-bitcoin-cli/src/operator/sync_truth_render.rs
    - .planning/phases/72-operator-observability-and-support-evidence/72-01-SUMMARY.md
  modified:
    - packages/open-bitcoin-cli/src/operator.rs
    - packages/open-bitcoin-cli/src/operator/status/render.rs
    - packages/open-bitcoin-cli/src/operator/status/render/tests.rs
    - packages/open-bitcoin-cli/src/operator/dashboard/model.rs
    - packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs
    - packages/open-bitcoin-rpc/src/dispatch/tests.rs
    - docs/parity/source-breadcrumbs.json
    - docs/metrics/lines-of-code.md

key-decisions:
  - "Use one shared CLI helper for human and dashboard sync-truth text so validated active-chain progress cannot drift between local projections."
  - "Keep Phase 72 support-only evidence out of baseline-compatible getblockchaininfo and prove that with a JSON shape regression."

patterns-established:
  - "Status renderer tests assert exact Phase 72 labels and unavailable reasons instead of only checking field presence."
  - "Open Bitcoin-specific RPC status may serialize the durable evidence contract while baseline RPC tests guard Knots-compatible output shape."

requirements-completed: [OBS-01, OBS-03]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 72-2026-06-13T16-25-04
generated_at: 2026-06-13T18:21:53Z

duration: 45min
completed: 2026-06-13
---

# Phase 72 Plan 01: Status Surface Full-Sync Truth Summary

**CLI, dashboard, and Open Bitcoin RPC status now expose the same durable full-sync evidence while baseline getblockchaininfo stays support-field free.**

## Performance

- **Duration:** 45 min
- **Started:** 2026-06-13T17:36:25Z
- **Completed:** 2026-06-13T18:21:53Z
- **Tasks:** 3
- **Files modified:** 8
- **Files created:** 2

## Accomplishments

- Added shared `sync_truth_render` helpers for best-known tip, stay-current, no-progress diagnosis, reorg, reconcile, and validated active-chain progress text.
- Extended human CLI status output and dashboard sync rows with Phase 72 evidence labels and exact unavailable-reason handling.
- Added RPC tests proving `open_bitcoin_sync_status` preserves durable Phase 72 fields and `getblockchaininfo` does not expose support-only fields.

## Task Commits

Task commits are pending the wrapper-owned final commit after full phase verification.

1. **Task 1: CLI status full-sync truth rendering** - `pending final wrapper commit`
2. **Task 2: Dashboard full-sync truth rows** - `pending final wrapper commit`
3. **Task 3: RPC durable status and baseline compatibility tests** - `pending final wrapper commit`

## Files Created/Modified

- `packages/open-bitcoin-cli/src/operator/sync_truth_render.rs` - Shared text projection helpers for typed sync-truth fields.
- `packages/open-bitcoin-cli/src/operator.rs` - Registers the shared sync-truth rendering module.
- `packages/open-bitcoin-cli/src/operator/status/render.rs` - Adds Phase 72 human status lines and uses the shared progress formatter.
- `packages/open-bitcoin-cli/src/operator/status/render/tests.rs` - Adds exact available/unavailable CLI renderer coverage.
- `packages/open-bitcoin-cli/src/operator/dashboard/model.rs` - Adds dashboard rows for Phase 72 sync evidence and shared progress formatting.
- `packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs` - Adds exact dashboard row and unavailable-reason coverage.
- `packages/open-bitcoin-rpc/src/dispatch/tests.rs` - Adds Open Bitcoin-specific durable status and baseline getblockchaininfo regression tests.
- `docs/parity/source-breadcrumbs.json` - Adds the new Open Bitcoin-only helper module breadcrumb.
- `docs/metrics/lines-of-code.md` - Refreshed by the repo LOC generator after source/test changes.

## Decisions Made

- Shared the text formatting helper between CLI and dashboard instead of duplicating validated active-chain formatting in each renderer.
- Preserved baseline RPC compatibility by adding assertions against serialized support-only field names in `getblockchaininfo`.

## Deviations from Plan

- Added `packages/open-bitcoin-cli/src/operator/sync_truth_render.rs` to avoid renderer-local formatting drift. The new first-party Rust source file includes an explicit `none` parity breadcrumb and is registered in `docs/parity/source-breadcrumbs.json`.

## Issues Encountered

- The first executor agent stalled after producing code changes but before writing the summary. The orchestrator closed that agent, reviewed the partial diff, completed the shared-renderer cleanup locally, and reran focused checks.
- A full filtered `open-bitcoin-cli` package test run hung inside an empty binary test harness after the matching library tests passed. It was interrupted, and the bounded `--lib` focused run completed cleanly.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --lib phase72 --all-features -- --nocapture`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --lib phase72 --all-features -- --nocapture`
- `rg -n "Sync best-known tip|Best-known tip|open_bitcoin_sync_status_returns_phase72_durable_truth_contract|phase72_cli_status_renders_full_sync_truth_contract|phase72_dashboard_projects_full_sync_truth_contract|get_blockchain_info_does_not_expose_phase72_support_fields|validated_active_chain_work" packages/open-bitcoin-cli/src/operator/status/render.rs packages/open-bitcoin-cli/src/operator/status/render/tests.rs packages/open-bitcoin-cli/src/operator/dashboard/model.rs packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs packages/open-bitcoin-rpc/src/dispatch/tests.rs packages/open-bitcoin-cli/src/operator/sync_truth_render.rs`

## User Setup Required

None - all checks are deterministic and local.

## Next Phase Readiness

Plan 72-02 can derive support evidence from the same durable status fields that now project through CLI, dashboard, and Open Bitcoin RPC surfaces.

## Self-Check: PASSED

- Summary file exists.
- Focused Plan 01 tests pass.
- Shared sync-truth helper has an explicit parity breadcrumb.
- Baseline RPC output remains guarded against support-only evidence fields.

*Phase: 72-operator-observability-and-support-evidence*
*Completed: 2026-06-13*
