---
phase: 34-migration-detection-ownership-model-cleanup
plan: "02"
status: completed
generated_by: gsd-execute-phase
lifecycle_mode: yolo
phase_lifecycle_id: 34-2026-04-30T07-38-33
generated_at: 2026-04-30T07:55:55Z
completed: 2026-04-30
---

# Phase 34 Plan 02 Summary

## One-Liner

Migration planning now consumes the tightened scan-level detection model
directly while preserving the existing selected-source service-review truth and
manual-review fallback.

## What Was Built

- Updated `packages/open-bitcoin-cli/src/operator/migration.rs` and
  `packages/open-bitcoin-cli/src/operator/migration/planning.rs` so migration
  planning consumes `DetectionScan` instead of a bare vector of installations.
- Changed `associate_service_candidates()` in
  `packages/open-bitcoin-cli/src/operator/migration/service_evidence.rs` to
  match a selected installation against the scan-level service list explicitly.
- Preserved the existing selected-source and ambiguous-service summary behavior
  in `packages/open-bitcoin-cli/src/operator/migration/tests.rs`.
- Kept the custom-source operator-binary migration regression green so
  `open-bitcoin migrate plan --source-datadir <custom-path>` remains truthful
  after the ownership-model cleanup.
- Split label and path-rendering helpers into
  `packages/open-bitcoin-cli/src/operator/migration/planning/labels.rs` so the
  planner stays under the repo's production Rust file-length limit without
  changing operator-visible behavior.

## Task Commits

1. **Task 1: adopt the tightened ownership model in migration** — Pending the
   wrapper-owned Phase 34 finalization commit.

## Verification

Passed:

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli migration::tests`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --test operator_binary open_bitcoin_migrate_plan_selects_explicit_custom_source_outside_default_roots`

## Deviations from Plan

- The first repo-native verification pass showed `migration/planning.rs` had
  grown to 632 lines. The fix stayed narrow: extract label helpers into
  `planning/labels.rs` and register the nested file in
  `docs/parity/source-breadcrumbs.json`.

## Self-Check: PASSED

- Selected-source migration planning still shows only the relevant service
  review actions.
- Ambiguous service ownership still degrades to explicit manual review.
- The migration planner now receives scan-level service evidence through an
  explicit typed boundary instead of a misleading cloned field.
