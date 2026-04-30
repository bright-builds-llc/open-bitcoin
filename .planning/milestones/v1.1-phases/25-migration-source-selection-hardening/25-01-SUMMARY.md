---
phase: 25-migration-source-selection-hardening
plan: "01"
subsystem: migration-source-detection
requirements-completed: [MIG-02, MIG-04]
generated_by: gsd-execute-phase
lifecycle_mode: yolo
phase_lifecycle_id: 25-2026-04-28T20-22-00
generated_at: 2026-04-28T20:44:00.000Z
completed: 2026-04-28
---

# Phase 25 Plan 01 Summary

## One-Liner

`open-bitcoin migrate plan` now seeds an explicit `--source-datadir` into its
read-only detection roots, so supported custom-location source installs can
produce concrete dry-run plans even when they live outside the default home
paths.

## What Was Built

- Added a command-scoped detection helper in
  `packages/open-bitcoin-cli/src/operator/runtime.rs`.
- Kept status, onboarding, service, dashboard, and wallet commands on the
  existing detection path while giving migration its own augmented root set.
- For migration only, appended the explicit `--source-datadir` into the shared
  detector's candidate datadirs before calling `detect_existing_installations()`.
- Added an operator-binary regression for a custom source datadir outside the
  default `HOME/.bitcoin` roots and proved the planner still stays dry-run and
  secret-safe.

## Task Commits

1. **Task 1: let migration detection see explicit custom source datadirs** —
   Pending the wrapper-owned Phase 25 finalization commit.

## Verification

Passed:

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --test operator_binary -- --nocapture open_bitcoin_migrate_plan_selects_explicit_custom_source_outside_default_roots`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli -- --nocapture`

## Deviations from Plan

- None.

## Self-Check: PASSED

- The fix reuses the existing detector instead of inventing a second migration
  scanner.
- Only the migration command gets the explicit-path augmentation, so the gap
  closure stays narrow.
