---
phase: 64-service-restart-and-same-datadir-resume-evidence
plan: "01"
subsystem: service-restart-resume-status
tags:
  - rust
  - status
  - durable-sync
  - service-restart
requires:
  - .planning/phases/64-service-restart-and-same-datadir-resume-evidence/64-01-PLAN.md
provides:
  - shared service restart/resume status contract
  - selected-datadir runtime metadata projection
  - clean and unclean prior-shutdown evidence
  - stale in-flight verdict projection
affects:
  - open-bitcoin-node status contracts
  - open-bitcoin-cli status collection
requirements_completed:
  - SVC-03
  - RR-03
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 64-2026-06-08T03-22-46
generated_at: 2026-06-08T03:43:38.403Z
---

# Phase 64 Plan 01 Summary

Added the shared `ServiceRestartResumeStatus` contract and projected it from the
selected datadir's durable runtime metadata.

## What Changed

- Added `ServicePriorShutdownStatus`, `ServiceStaleInflightStatus`,
  `ServiceResumeProgressStatus`, and `ServiceRestartResumeStatus`.
- Extended `ServiceStatus` with additive `restart_resume` serde defaults for
  older JSON compatibility.
- Added `durable_runtime_metadata()` and reused it for durable sync state.
- Projected datadir, same-datadir verdict, prior shutdown, progress,
  stale in-flight status, recovery category, and next action into service status.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node service_restart_resume_status_contract --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli service_restart_resume --all-features`
