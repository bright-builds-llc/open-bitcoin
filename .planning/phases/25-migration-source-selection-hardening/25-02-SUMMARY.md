---
phase: 25-migration-source-selection-hardening
plan: "02"
subsystem: planner-hardening
requirements-completed: [MIG-02, MIG-04]
generated_by: gsd-execute-phase
lifecycle_mode: yolo
phase_lifecycle_id: 25-2026-04-28T20-22-00
generated_at: 2026-04-28T20:44:00.000Z
completed: 2026-04-28
---

# Phase 25 Plan 02 Summary

## One-Liner

The migration planner remains conservative even with the new explicit-path
support: it now requires real source config, cookie, or wallet evidence before
auto-selecting an explicit source datadir into a concrete plan.

## What Was Built

- Added a small planner guard in
  `packages/open-bitcoin-cli/src/operator/migration/planning.rs` that rejects
  explicit path matches lacking source config, cookie, or wallet evidence.
- Preserved the manual-review fallback path with clearer operator guidance when
  the explicit directory exists but does not yet look like a supported Core or
  Knots datadir.
- Added a focused planner unit test for that unsupported explicit-path case.

## Task Commits

1. **Task 1: keep explicit source selection conservative and evidence-driven** —
   Pending the wrapper-owned Phase 25 finalization commit.

## Verification

Passed:

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli migration::tests -- --nocapture`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli -- --nocapture`

## Deviations from Plan

- None.

## Self-Check: PASSED

- A bare existing directory no longer counts as a valid migration source.
- The phase widened source selection without weakening the Phase 21 dry-run-only
  and manual-review safety posture.
