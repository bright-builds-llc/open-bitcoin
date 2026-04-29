---
phase: 28-service-log-path-truth-and-operator-docs-alignment
plan: "01"
subsystem: service-log-path-runtime
requirements-completed: [SVC-03, SVC-04]
generated_by: gsd-execute-phase
lifecycle_mode: yolo
phase_lifecycle_id: 28-2026-04-29T13-37-12
generated_at: 2026-04-29T13:50:24.398Z
completed: 2026-04-29
---

# Phase 28 Plan 01 Summary

## One-Liner

The shared service runtime now derives a concrete `open-bitcoin.log` file from
the selected operator log directory, preserves it in launchd and systemd
generated artifacts, and keeps `open-bitcoin service status` truthful about that
path.

## What Was Built

- Added `service_log_path_from_log_dir()` in
  `packages/open-bitcoin-cli/src/operator/service.rs` so the service and
  dashboard runtime paths stop treating the resolved log directory like a log
  file.
- Extended `ServiceStateSnapshot` and `render_service_state_snapshot()` so the
  service surface now preserves either a concrete log path or an explicit
  unavailable reason instead of silently dropping the line.
- Updated `packages/open-bitcoin-cli/src/operator/service/launchd.rs` to recover
  `StandardOutPath` from the installed plist and
  `packages/open-bitcoin-cli/src/operator/service/systemd.rs` to generate
  `StandardOutput=append:...` or `StandardError=append:...` plus recover that
  file-backed path from the installed unit.
- Kept dashboard service actions on the shared execution path by routing the
  derived concrete service log file through `runtime.rs` into the existing
  dashboard service runtime.
- Added focused generator, parser, and status-rendering coverage and updated the
  operator-runtime benchmark fixture for the expanded service snapshot contract.

## Task Commits

1. **Task 1: restore service log-path truth across preview, apply, and
   `service status`** — Pending the wrapper-owned Phase 28 finalization commit.

## Verification

Passed:

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli service::tests -- --nocapture`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli status::tests -- --nocapture`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-bench -- --nocapture`

## Deviations from Plan

- None.

## Self-Check: PASSED

- Launchd and systemd now preserve one concrete service log file path instead of
  drifting between preview and apply behavior.
- `open-bitcoin service status` now stays explicit even when an installed
  service definition does not expose a file-backed path.
