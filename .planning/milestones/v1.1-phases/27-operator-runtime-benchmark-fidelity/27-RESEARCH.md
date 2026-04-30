---
generated_by: gsd-plan-phase
lifecycle_mode: yolo
phase_lifecycle_id: 27-2026-04-28T20-48-00
generated_at: 2026-04-28T21:02:00.000Z
---

# Phase 27 Research

## Gap Being Closed

### `VER-06` fidelity follow-up

- The roadmap now reserves Phase 27 for the remaining operator-runtime benchmark
  fidelity gap.
- The original milestone audit identified the problem clearly: the
  operator-runtime benchmark group still exercised `sample_status_snapshot()`
  fixtures instead of the real runtime collection path.

## Current Code Facts

1. `packages/open-bitcoin-bench/src/cases/operator_runtime.rs` currently imports
   `sample_status_snapshot()` from `runtime_fixtures.rs` and benchmarks status
   render plus dashboard projection from that synthetic snapshot.
2. `packages/open-bitcoin-cli/src/operator/status.rs` exposes public
   `StatusRequest`, `StatusCollectorInput`, `StatusDetectionEvidence`,
   `StatusRpcClient`, and `collect_status_snapshot()` types and functions that
   the benchmark crate can call directly.
3. `packages/open-bitcoin-cli/src/operator/dashboard/mod.rs` exposes public
   `DashboardRuntimeContext`, `DashboardServiceRuntime`, and
   `collect_dashboard_snapshot()` helpers for the dashboard runtime path.
4. `packages/open-bitcoin-bench/src/runtime_fixtures.rs` already provides
   deterministic temp-store helpers and metrics snapshot builders, so the
   benchmark can seed local `FjallNodeStore` state without touching real host
   data.
5. `scripts/check-benchmark-report.ts` only enforces stable case IDs, group IDs,
   schema/profile shape, and non-empty measurement metadata, so the fixture text
   can be updated without changing the validator schema.

## Constraints From Guidance

- `AGENTS.md` says to use `bash scripts/verify.sh` as the repo-native
  verification contract.
- Bright Builds verification guidance says to sync first, prefer repo-owned
  verification entrypoints, and keep the verification burden proportional to the
  changed surfaces.
- Bright Builds testing guidance says pure or business logic should be unit
  tested, but the benchmark runner already exercises registered cases in tests,
  so Phase 27 should add only the focused checks that materially improve
  confidence.
- Rust guidance says to keep helpers readable, guard-oriented, and inside the
  touched module layout when a new module split is unnecessary.

## Chosen Implementation Strategy

### Status benchmark fidelity

- Replace `sample_status_snapshot()` usage with a deterministic fixture that:
  - creates temp dirs for datadir, logs, and metrics
  - seeds metrics history into a temp `FjallNodeStore`
  - builds a public `StatusCollectorInput`
  - injects a tiny fake `StatusRpcClient`
  - injects a fake running `ServiceManager`
- Keep the case focused on rendering cost, but collect the snapshot through the
  real shared status collector first.

### Dashboard benchmark fidelity

- Route the dashboard case through `collect_dashboard_snapshot()` instead of
  direct `DashboardState::from_snapshot(sample_status_snapshot())`.
- Keep the existing assertions around sections, charts, actions, and text
  headings, but ensure at least one collected chart has real metric points.

### Metadata and docs

- Update the `operator-runtime` benchmark group description and notes in
  `registry.rs` to reflect runtime-collected status/dashboard evidence.
- Update `docs/parity/benchmarks.md` and the parity catalog entry so they no
  longer describe the operator-runtime group as projection from a shared sample
  snapshot.

### Verification and closeout

- Run the `open-bitcoin-bench` crate tests.
- Run the benchmark smoke wrapper plus the report validator.
- Refresh `docs/metrics/lines-of-code.md` before the repo-native verify gate.
- Mark `VER-06` complete in `.planning/REQUIREMENTS.md` and close Phase 27 in
  `.planning/ROADMAP.md`.
