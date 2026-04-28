---
phase: 19-ratatui-node-dashboard
plan: "01"
subsystem: dashboard-runtime-bootstrap
requirements-completed: [DASH-01, SYNC-06]
generated_by: gsd-execute-phase
lifecycle_mode: yolo
phase_lifecycle_id: 19-2026-04-27T09-02-20
generated_at: 2026-04-27T09:29:09Z
tags:
  - dashboard
  - cli
  - snapshot
  - status
key_files:
  created:
    - packages/open-bitcoin-cli/src/operator/dashboard/mod.rs
  modified:
    - packages/open-bitcoin-cli/src/operator.rs
    - packages/open-bitcoin-cli/src/operator/runtime.rs
    - packages/open-bitcoin-cli/Cargo.toml
    - packages/open-bitcoin-cli/BUILD.bazel
    - docs/parity/source-breadcrumbs.json
metrics:
  completed_date: "2026-04-27"
  files_created: 1
  files_modified: 5
---

# Phase 19 Plan 01 Summary

## One-Liner

`open-bitcoin dashboard` now dispatches through the operator runtime into a snapshot-first dashboard shell instead of the old deferred placeholder.

## What Was Built

- Added `operator::dashboard` as a real module and routed `OperatorCommand::Dashboard` through `run_dashboard(...)` in [`packages/open-bitcoin-cli/src/operator/runtime.rs`](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-cli/src/operator/runtime.rs).
- Introduced `DashboardRuntimeContext` and `DashboardServiceRuntime` in [`packages/open-bitcoin-cli/src/operator/dashboard/mod.rs`](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-cli/src/operator/dashboard/mod.rs) so the dashboard reuses the existing status and service runtime contracts instead of inventing a second runtime model.
- Kept data flow snapshot-first with `collect_dashboard_snapshot(...)`, which calls `collect_status_snapshot(...)` exactly once per refresh path.
- Added deterministic non-interactive rendering for human and JSON output. JSON serializes the shared `OpenBitcoinStatusSnapshot`; human fallback prints stable sections for node, sync, wallet, service, logs, charts, and actions.
- Wired the Ratatui/Crossterm dependencies into both Cargo and Bazel and added parity manifest coverage for the new dashboard sources.

## Deviations from Plan

- The runtime bootstrap and dependency wiring landed together because the new dashboard shell could not compile or route without the terminal dependencies already present in the crate build graph.
- This closeout resumed from an existing dirty worktree, so there is no plan-local commit table for this summary.

## Self-Check: PASSED

- `cargo test --package open-bitcoin-cli --all-features` passed.
- `cargo fmt --all`, `cargo clippy --package open-bitcoin-cli --all-targets --all-features -- -D warnings`, and `cargo build --package open-bitcoin-cli --all-features` passed.
- `bash scripts/verify.sh` passed after refreshing generated LOC and parity tracking artifacts for the new dashboard files.
