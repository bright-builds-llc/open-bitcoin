---
phase: 33-operator-surface-truth-and-coverage-cleanup
plan: "01"
status: completed
generated_by: gsd-execute-phase
lifecycle_mode: yolo
phase_lifecycle_id: 33-2026-04-30T05-02-20
generated_at: 2026-04-30T05:12:28.878Z
completed: 2026-04-30
---

# Phase 33 Plan 01 Summary

## One-Liner

The operator CLI now stops advertising a dead `status --watch` flag, and all
remaining unmanaged service preview hints point operators at the real
preview-by-default `service install` flow.

## What Was Built

- Removed the unused `watch` field from `StatusArgs`, keeping `open-bitcoin
  status` as the truthful one-shot snapshot surface already implemented in the
  runtime.
- Added a guard test in `packages/open-bitcoin-cli/src/operator/tests.rs` that
  proves `open-bitcoin status --watch` is now rejected instead of silently
  parsing and doing nothing.
- Updated the shared `ServiceError::NotInstalled` message plus unmanaged
  launchd/systemd diagnostics to describe the real contract:
  `open-bitcoin service install` previews, and `--apply` mutates.
- Added focused service tests that lock in the repaired preview hints across the
  shared renderer and both platform adapters.

## Task Commits

1. **Task 1: remove dead status watch surface and normalize service preview
   hints** — Pending the wrapper-owned Phase 33 finalization commit.

## Verification

Passed:

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli operator::service::tests`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli operator::tests`

## Deviations from Plan

- None. The cleanup stayed narrow: it removed the misleading flag instead of
  adding a new watch loop, and it repaired the stale preview text without
  changing the underlying service contract.

## Self-Check: PASSED

- The public `status` clap surface now matches the existing one-shot runtime.
- Shared CLI and platform-backed service hints now align with the documented
  preview-by-default `service install` flow.
