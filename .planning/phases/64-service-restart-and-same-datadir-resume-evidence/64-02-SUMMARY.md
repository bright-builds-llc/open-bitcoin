---
phase: 64-service-restart-and-same-datadir-resume-evidence
plan: "02"
subsystem: operator-status-dashboard
tags:
  - rust
  - cli
  - dashboard
  - service-restart
requires:
  - .planning/phases/64-service-restart-and-same-datadir-resume-evidence/64-02-PLAN.md
provides:
  - human status restart/resume rendering
  - dashboard restart/resume service rows
  - service restart evidence guidance
affects:
  - open-bitcoin status human output
  - operator dashboard service section
  - open-bitcoin service restart output
requirements_completed:
  - SVC-03
  - RR-03
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 64-2026-06-08T03-22-46
generated_at: 2026-06-08T03:43:38.403Z
---

# Phase 64 Plan 02 Summary

Rendered service restart/resume evidence consistently across human status,
dashboard rows, and service restart guidance.

## What Changed

- Appended `restart_resume=` to the human Service status line.
- Added dashboard rows for restart/resume, prior shutdown, resume progress,
  stale in-flight state, and resume action.
- Added a successful `service restart` guidance line pointing operators to
  `open-bitcoin status --format json` with the same datadir.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli service_restart_resume_status_render --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli dashboard_service_restart_resume --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli service_restart_calls_manager_and_renders_commands --all-features`
