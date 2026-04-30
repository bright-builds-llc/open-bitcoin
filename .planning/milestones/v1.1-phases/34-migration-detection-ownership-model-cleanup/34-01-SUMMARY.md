---
phase: 34-migration-detection-ownership-model-cleanup
plan: "01"
status: completed
generated_by: gsd-execute-phase
lifecycle_mode: yolo
phase_lifecycle_id: 34-2026-04-30T07-38-33
generated_at: 2026-04-30T07:55:55Z
completed: 2026-04-30
---

# Phase 34 Plan 01 Summary

## One-Liner

The shared detector now returns service definitions as scan-level evidence via a
typed `DetectionScan`, and status/runtime consumers no longer depend on
installation-local cloned service lists.

## What Was Built

- Added `DetectionScan` in `packages/open-bitcoin-cli/src/operator/detect.rs`
  with explicit `installations` and `service_candidates` collections.
- Removed `service_candidates` from `DetectedInstallation`, keeping the
  installation shape focused on installation-local datadir/config/cookie/wallet
  evidence.
- Updated runtime status wiring so `StatusDetectionEvidence` carries the
  scan-level service list explicitly instead of reconstructing it from detected
  installations.
- Added focused status coverage proving detected service candidates still drive
  truthful fallback service status when no platform manager is inspected.
- Updated detector, dashboard, onboarding, wallet, and nearby status fixtures so
  they no longer model service definitions as installation-owned data.

## Task Commits

1. **Task 1: introduce the scan-level detection ownership model** — Pending the
   wrapper-owned Phase 34 finalization commit.

## Verification

Passed:

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli detect::tests`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli status::tests`

## Deviations from Plan

- None. The data-shape repair stayed in the shared detector/runtime boundary and
  did not widen onboarding, wallet, or status behavior beyond the ownership
  cleanup.

## Self-Check: PASSED

- Service definitions are now scan-level evidence instead of a per-installation
  field.
- Status fallback truth remains explicit after adopting the tightened model.
- Installation-local fixtures no longer imply that every service belongs to
  every detected install.
