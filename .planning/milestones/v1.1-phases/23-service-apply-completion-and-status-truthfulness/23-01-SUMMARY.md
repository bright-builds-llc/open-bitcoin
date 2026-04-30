---
phase: 23-service-apply-completion-and-status-truthfulness
plan: "01"
subsystem: service-apply
requirements-completed: [SVC-01, SVC-02, SVC-03, SVC-05]
generated_by: gsd-execute-phase
lifecycle_mode: yolo
phase_lifecycle_id: 23-2026-04-28T17-18-36
generated_at: 2026-04-28T17:24:34.432Z
completed: 2026-04-28
---

# Phase 23 Plan 01 Summary

## One-Liner

`open-bitcoin service install --apply` now completes the real launchd or
systemd registration sequence after writing the service file, and the dry-run
preview still shows the exact command path apply mode follows.

## What Was Built

- Added shared preview-command helpers for launchd and systemd install flows so
  dry-run output and apply behavior stay aligned.
- Updated `LaunchdAdapter::install()` to run `launchctl enable` plus
  `launchctl bootstrap` after the plist write succeeds.
- Updated `SystemdAdapter::install()` to run `systemctl --user daemon-reload`
  plus `systemctl --user enable` after the unit write succeeds.
- Extended the hermetic service tests with dry-run command assertions for both
  platforms.

## Task Commits

1. **Task 1: complete install apply semantics without breaking dry-run
   truthfulness** — Pending the wrapper-owned Phase 23 finalization commit.

## Verification

Passed:

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli service::tests -- --nocapture`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli -- --nocapture`

## Deviations from Plan

- The systemd uninstall path also gained a post-remove `daemon-reload` step so
  service-manager state stays consistent after file removal.

## Self-Check: PASSED

- Install apply no longer stops after a file write on either supported service
  manager.
- Dry-run previews remain the operator-facing source of truth for the command
  sequence apply mode will execute.
