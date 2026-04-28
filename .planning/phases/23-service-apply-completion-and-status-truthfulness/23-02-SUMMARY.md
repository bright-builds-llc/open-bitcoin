---
phase: 23-service-apply-completion-and-status-truthfulness
plan: "02"
subsystem: service-status
requirements-completed: [SVC-04, DASH-03, SVC-05]
generated_by: gsd-execute-phase
lifecycle_mode: yolo
phase_lifecycle_id: 23-2026-04-28T17-18-36
generated_at: 2026-04-28T17:24:34.432Z
completed: 2026-04-28
---

# Phase 23 Plan 02 Summary

## One-Liner

Service status and dashboard projections now preserve real manager-reported
enablement instead of inferring it from `ServiceLifecycleState`, so the operator
surface can represent failed-enabled and running-disabled combinations honestly.

## What Was Built

- Added `maybe_enabled: Option<bool>` to `ServiceStateSnapshot` so service
  adapters can carry manager-reported startup enablement through the shared
  runtime path.
- Taught the launchd adapter to query `launchctl print-disabled` and the
  systemd adapter to query `systemctl --user is-enabled`.
- Updated `collect_service_status()` to prefer explicit manager evidence over
  the old enum-only heuristic.
- Updated the CLI service-status renderer to print explicit installed, enabled,
  and running booleans alongside the lifecycle label and diagnostics.
- Added focused service and status tests for parser coverage, failed-enabled
  projection, running-disabled projection, and service-status rendering.

## Task Commits

1. **Task 1: preserve manager truth across CLI status and dashboard surfaces** —
   Pending the wrapper-owned Phase 23 finalization commit.

## Verification

Passed:

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli service::tests -- --nocapture`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli status::tests -- --nocapture`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli -- --nocapture`

## Deviations from Plan

- No dashboard code changes were needed because the existing confirmation-gated
  shared service runtime path already inherited the corrected behavior.

## Self-Check: PASSED

- Service truth now comes from manager evidence when available.
- The dashboard service-action closure landed by fixing the shared runtime path
  instead of introducing a second action implementation.
