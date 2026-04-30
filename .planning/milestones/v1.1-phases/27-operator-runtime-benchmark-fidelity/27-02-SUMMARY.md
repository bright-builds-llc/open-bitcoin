---
phase: 27-operator-runtime-benchmark-fidelity
plan: "02"
subsystem: operator-runtime-dashboard-benchmark
requirements-completed: [VER-06]
generated_by: gsd-execute-phase
lifecycle_mode: yolo
phase_lifecycle_id: 27-2026-04-28T20-48-00
generated_at: 2026-04-28T21:27:00.000Z
completed: 2026-04-28
---

# Phase 27 Plan 02 Summary

## One-Liner

The operator-runtime dashboard benchmark now collects through
`collect_dashboard_snapshot()`, and the benchmark metadata/docs now describe the
operator-runtime group as runtime-collected evidence instead of shared-snapshot
projection.

## What Was Built

- Added a dashboard fixture holder that keeps the tempdir-backed runtime state
  alive for the entire dashboard benchmark call, so collected metrics history is
  still present when `collect_dashboard_snapshot()` runs.
- Updated `operator-runtime.dashboard-projection` to collect the shared runtime
  snapshot before projecting `DashboardState` and rendering the text snapshot.
- Refreshed the operator-runtime case fixture labels to:
  - `runtime_collected_status_snapshot`
  - `runtime_collected_dashboard_snapshot`
- Updated `packages/open-bitcoin-bench/src/registry.rs` so the operator-runtime
  group now describes runtime-collected status and dashboard snapshots.
- Updated `docs/parity/benchmarks.md` and
  `docs/parity/catalog/operator-runtime-release-hardening.md` so the public
  benchmark narrative matches the new fidelity level.

## Task Commits

1. **Task 1: route the dashboard benchmark through the real dashboard collector
   and refresh the metadata** — Pending the wrapper-owned Phase 27 finalization
   commit.

## Verification

Passed:

- `bash scripts/run-benchmarks.sh --smoke --output-dir packages/target/benchmark-reports`
- `bun scripts/check-benchmark-report.ts --report=packages/target/benchmark-reports/open-bitcoin-bench-smoke.json`

## Deviations from Plan

- None.

## Self-Check: PASSED

- The dashboard benchmark now hits the real dashboard collection path.
- The smoke report and docs both describe the operator-runtime cases with
  truthful runtime-collected fixture metadata.
