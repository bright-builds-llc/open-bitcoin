---
phase: 19-ratatui-node-dashboard
plan: "02"
subsystem: dashboard-interactive-surface
requirements-completed: [DASH-01, DASH-02, DASH-03, SYNC-06]
generated_by: gsd-execute-phase
lifecycle_mode: yolo
phase_lifecycle_id: 19-2026-04-27T09-02-20
generated_at: 2026-04-27T09:29:09Z
tags:
  - dashboard
  - ratatui
  - actions
  - metrics
  - service
key_files:
  created:
    - packages/open-bitcoin-cli/src/operator/dashboard/model.rs
    - packages/open-bitcoin-cli/src/operator/dashboard/action.rs
    - packages/open-bitcoin-cli/src/operator/dashboard/app.rs
  modified:
    - packages/open-bitcoin-node/src/metrics.rs
    - packages/open-bitcoin-node/src/storage/fjall_store.rs
    - packages/open-bitcoin-cli/src/operator/status/render.rs
    - docs/architecture/status-snapshot.md
metrics:
  completed_date: "2026-04-27"
  files_created: 3
  files_modified: 4
---

# Phase 19 Plan 02 Summary

## One-Liner

The dashboard now has a real Ratatui app loop, projection model, bounded metric charts, and confirmation-gated service actions built on the shared snapshot surface.

## What Was Built

- Added `DashboardState`, section rows, chart models, and action entries in [`packages/open-bitcoin-cli/src/operator/dashboard/model.rs`](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-cli/src/operator/dashboard/model.rs).
- Kept charts on the shared status contract by extending `MetricsStatus` with bounded `samples` and by loading those samples from [`packages/open-bitcoin-node/src/storage/fjall_store.rs`](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/storage/fjall_store.rs).
- Added a real interactive Ratatui dashboard loop in [`packages/open-bitcoin-cli/src/operator/dashboard/app.rs`](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-cli/src/operator/dashboard/app.rs) with:
  - 1-second default refresh cadence via `tick_ms`
  - compact section layout
  - bounded sparklines for sync, peers, mempool, disk usage, and RPC health
  - restrained color usage for title, section labels, charts, and action prompts
- Added a confirmation state machine in [`packages/open-bitcoin-cli/src/operator/dashboard/action.rs`](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-cli/src/operator/dashboard/action.rs) so install/uninstall/enable/disable actions cannot execute until explicitly confirmed.
- Updated [`packages/open-bitcoin-cli/src/operator/status/render.rs`](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-cli/src/operator/status/render.rs) and [`docs/architecture/status-snapshot.md`](/Users/peterryszkiewicz/Repos/open-bitcoin/docs/architecture/status-snapshot.md) so the snapshot contract clearly exposes bounded metric history to dashboard consumers.

## Deviations from Plan

- The chart implementation required promoting metric samples into the shared `MetricsStatus` contract instead of reading storage directly from the dashboard. That is a deliberate contract-preserving change, not a dashboard-local shortcut.
- Two compile issues surfaced during verification and were fixed in place:
  - corrected the `MetricsAvailability` import path
  - made the interactive action match exhaustive for all destructive service actions

## Self-Check: PASSED

- Dashboard model and action unit tests passed:
  - `operator::dashboard::model::tests::dashboard_projection_includes_required_sections_and_charts`
  - `operator::dashboard::action::tests::{pending,cancelled,confirmed}_service_action_*`
- Full CLI package verification passed.
- Full repo `scripts/verify.sh` passed.
