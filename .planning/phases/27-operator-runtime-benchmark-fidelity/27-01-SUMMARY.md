---
phase: 27-operator-runtime-benchmark-fidelity
plan: "01"
subsystem: operator-runtime-status-benchmark
requirements-completed: [VER-06]
generated_by: gsd-execute-phase
lifecycle_mode: yolo
phase_lifecycle_id: 27-2026-04-28T20-48-00
generated_at: 2026-04-28T21:26:00.000Z
completed: 2026-04-28
---

# Phase 27 Plan 01 Summary

## One-Liner

The operator-runtime status benchmark now builds a deterministic tempdir-backed
runtime fixture and collects its snapshot through `collect_status_snapshot()`
before rendering human and JSON output.

## What Was Built

- Replaced the direct `sample_status_snapshot()` dependency in
  `packages/open-bitcoin-bench/src/cases/operator_runtime.rs` with an
  `OperatorRuntimeFixture` that creates isolated config, log, metrics, and
  binary paths under a temp dir.
- Added a tiny deterministic `RunningStatusRpcClient` so the benchmark exercises
  the live shared collector path without requiring a real daemon or public
  network.
- Seeded local metrics history through `FjallNodeStore` and
  `sample_metrics_storage_snapshot()` so the collected snapshot includes the
  dashboard-facing metric series.
- Kept the benchmark assertions focused on the shared operator surface:
  `Daemon: running`, wallet freshness, metrics availability, and stable JSON
  sections.

## Task Commits

1. **Task 1: route the operator-runtime status benchmark through the real status
   collector** — Pending the wrapper-owned Phase 27 finalization commit.

## Verification

Passed:

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-bench -- --nocapture`

## Deviations from Plan

- None.

## Self-Check: PASSED

- The benchmark now measures runtime-collected status output instead of a
  hand-built shared snapshot.
- The implementation stays deterministic and tempdir-backed, so it fits the
  existing offline benchmark smoke path.
