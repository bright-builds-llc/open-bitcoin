---
phase: 31-migration-source-specific-service-review-truth
plan: "01"
subsystem: migration-service-review
requirements-completed: [MIG-02, MIG-04]
generated_by: gsd-execute-phase
lifecycle_mode: yolo
phase_lifecycle_id: 31-2026-04-29T16-42-33
generated_at: 2026-04-29T17:56:45.242Z
completed: 2026-04-29
---

# Phase 31 Plan 01 Summary

## One-Liner

`open-bitcoin migrate plan --source-datadir ...` now keeps service cutover review
truthful by showing only service definitions that can be tied to the selected
source install and falling back to explicit manual review when service ownership
is ambiguous.

## What Was Built

- Added a migration-local read-only helper in
  `packages/open-bitcoin-cli/src/operator/migration/service_evidence.rs` that
  inspects launchd and systemd service-definition arguments for `datadir` and
  config ownership evidence.
- Updated `packages/open-bitcoin-cli/src/operator/migration/planning.rs` so the
  selected-source summary, service action group, and service-surface deviation
  relevance only treat matched service definitions as source-specific.
- Added explicit `service_review_ambiguous` fallback handling so the planner
  keeps the selected source install but tells operators to review service
  ownership manually when a detected service cannot be tied to that install.
- Added focused planner regressions for matched-source inclusion and
  ambiguous-service fallback, then tightened the operator binary coverage so the
  explicit custom-source flow no longer inherits an unrelated service review
  path.

## Task Commits

1. **Task 1: scope migration service review to the selected source** — Pending
   the wrapper-owned Phase 31 finalization commit.

## Verification

Passed:

- `cargo fmt --manifest-path packages/Cargo.toml --all`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli planner_limits_service_review_to_selected_source_installation -- --nocapture`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli planner_uses_manual_service_review_when_service_ownership_is_ambiguous -- --nocapture`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --test operator_binary -- --nocapture open_bitcoin_migrate_plan_selects_explicit_custom_source_outside_default_roots`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --test operator_binary -- --nocapture open_bitcoin_migrate_plan_is_dry_run_only_for_detected_source_install`

## Deviations from Plan

- The implementation stayed migration-local instead of rewriting the broader
  detection data model. This still closes the blocker because the false service
  association only leaked through migration planning, and the narrower helper
  keeps the fix isolated to the selected-source review path.

## Self-Check: PASSED

- Selected-source migration review no longer shows scan-wide service definitions
  as though they belong to every install.
- Ambiguous service ownership now degrades to an explicit manual-review step
  rather than silently showing the wrong cutover path.
