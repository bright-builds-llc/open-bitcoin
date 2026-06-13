---
phase: 72-operator-observability-and-support-evidence
plan: 03
subsystem: operator-observability
tags: [metrics, structured-logs, live-smoke, full-sync-evidence]

requires: [72-CONTEXT, 72-RESEARCH, 72-02-SUMMARY]
provides:
  - Bounded validated active-chain height metric samples
  - Structured sync logs with compact full-sync truth dimensions
  - Live-smoke schema version 2 final-status evidence fields
  - Deterministic live-smoke fixture assertions for full-sync truth fields
affects: [phase-72, node-metrics, sync-logs, live-smoke]

tech-stack:
  added: []
  patterns:
    - Metrics remain numeric and bounded
    - Structured log additions use compact key-value labels only
    - Live-smoke final-status evidence is mapped from typed durable status JSON

key-files:
  created:
    - .planning/phases/72-operator-observability-and-support-evidence/72-03-SUMMARY.md
  modified:
    - packages/open-bitcoin-node/src/metrics.rs
    - packages/open-bitcoin-node/src/sync/types/summary.rs
    - scripts/run-live-mainnet-smoke.ts
    - scripts/test-run-live-mainnet-smoke.sh

key-decisions:
  - "Added `validated_active_chain_height` as a numeric metric instead of adding labels or freeform metric dimensions."
  - "Kept structured sync logs compact: validated active-chain, resource pressure, peer contribution, stop reason, and recovery category are emitted as bounded labels."
  - "Kept live-smoke schema version 2 because the new final_status keys are additive and fixture covered."

patterns-established:
  - "Live-smoke final-status Markdown and JSON now share typed full-sync truth fields from durable status metadata."
  - "Fixture assertions fail with a Phase 72-specific message when final-status evidence is missing."

requirements-completed: [OBS-01, OBS-02, OBS-03]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 72-2026-06-13T16-25-04
generated_at: 2026-06-13T18:52:00Z

duration: 17min
completed: 2026-06-13
---

# Phase 72 Plan 03: Metrics, Logs, and Live-Smoke Evidence Summary

**Metrics, structured logs, and live-smoke reports now expose compact full-sync truth dimensions that correlate with operator status and support evidence.**

## Performance

- **Duration:** 17 min
- **Started:** 2026-06-13T18:35:51Z
- **Completed:** 2026-06-13T18:52:00Z
- **Tasks:** 2
- **Files modified:** 4
- **Files created:** 1

## Accomplishments

- Added `MetricKind::ValidatedActiveChainHeight` and sample emission from `SyncRunSummary::metric_samples`.
- Extended sync structured log records with validated active-chain height/work, resource pressure, peer contribution, latest stop reason, and recovery category.
- Added a focused Rust regression test for Phase 72 metric/log truth dimensions.
- Extended live-smoke `final_status` JSON and Markdown with validated active-chain, best-known tip, stay-current, no-progress, reorg, reconcile, and peer contribution evidence.
- Extended deterministic live-smoke fixtures and JSON/Markdown assertions while keeping default verification public-network-free.

## Task Commits

Task commits are pending the wrapper-owned final commit after full phase verification.

1. **Task 1: Add bounded metric and structured-log full-sync dimensions** - `pending final wrapper commit`
2. **Task 2: Extend live-smoke final status with full-sync truth fields** - `pending final wrapper commit`

## Files Created/Modified

- `packages/open-bitcoin-node/src/metrics.rs` - Adds the validated active-chain height metric kind and stable-name coverage.
- `packages/open-bitcoin-node/src/sync/types/summary.rs` - Emits the new metric and compact structured log dimensions.
- `scripts/run-live-mainnet-smoke.ts` - Adds typed final-status evidence fields and Markdown rows.
- `scripts/test-run-live-mainnet-smoke.sh` - Adds deterministic fixture data and assertions for the new evidence fields.

## Decisions Made

- Preserved numeric-only metrics and avoided freeform labels in `MetricSample`.
- Mapped live-smoke fields only from typed durable status JSON rather than human CLI or Markdown text.
- Kept schema version 2 because the report changes are additive and covered by deterministic fixtures.

## Deviations from Plan

None.

## Issues Encountered

- The live-smoke shell harness has many fixture scenarios and runs for over a minute. A traced run was used to verify the earlier silent failure was resolved, then the normal harness passed.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase72_summary_metrics_and_logs_carry_full_sync_truth_dimensions --all-features -- --nocapture`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node metric_kind_names_are_stable --all-features -- --nocapture`
- `bash scripts/test-run-live-mainnet-smoke.sh`
- Plan 72-03 `rg` acceptance checks for metric names, structured log labels, live-smoke JSON keys, Markdown rows, and schema version 2.
- `node "$HOME/.codex/get-shit-done/bin/gsd-tools.cjs" verify key-links .planning/phases/72-operator-observability-and-support-evidence/72-03-PLAN.md`

## User Setup Required

None - all checks are deterministic and local.

## Next Phase Readiness

Plan 72-04 can document the full evidence contract, add aggregate enforcement, and run repo-native verification.

## Self-Check: PASSED

- Summary file exists.
- Focused Plan 03 Rust tests pass.
- Live-smoke fixture harness passes.
- Evidence remains compact, typed, and public-network-free by default.

*Phase: 72-operator-observability-and-support-evidence*
*Completed: 2026-06-13*
