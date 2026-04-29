---
phase: 31-migration-source-specific-service-review-truth
plan: "02"
subsystem: migration-service-review-closeout
requirements-completed: [MIG-02, MIG-04]
generated_by: gsd-execute-phase
lifecycle_mode: yolo
phase_lifecycle_id: 31-2026-04-29T16-42-33
generated_at: 2026-04-29T18:00:14.646Z
completed: 2026-04-29
---

# Phase 31 Plan 02 Summary

## One-Liner

The selected-source migration service-review repair is now documented, passes
the repo-native verification contract, and has the closeout artifacts needed to
re-close the Phase 31 blocker in the planning ledgers.

## What Was Built

- Updated `docs/operator/runtime-guide.md` so the migration planner now
  explicitly says source-specific service review paths only appear when a
  detected service definition can be tied to the selected source install;
  otherwise service cutover review stays manual.
- Refreshed `docs/metrics/lines-of-code.md` after the Phase 31 code and docs
  changes.
- Prepared the Phase 31 closeout artifacts so `MIG-02` and `MIG-04` can be
  re-closed from passing selected-source and ambiguous-service evidence.

## Task Commits

1. **Task 1: verify and close the service-review blocker** — Pending the
   wrapper-owned Phase 31 finalization commit.

## Verification

Passed:

- `bun run scripts/generate-loc-report.ts --source=worktree --output=docs/metrics/lines-of-code.md`
- `bash scripts/verify.sh`

## Deviations from Plan

- The first repo-native verification run failed because
  `packages/open-bitcoin-cli/src/operator/migration/planning.rs` landed exactly
  on the repo's 628-line ceiling. The fix stayed behavior-preserving: move the
  service-review ambiguity constant and helper into the new
  `service_evidence.rs` module, rerun `cargo fmt`, refresh the LOC report, and
  rerun the full verification contract cleanly.

## Self-Check: PASSED

- Operator docs now describe the same selected-source service-review rule the
  planner actually enforces.
- The Phase 31 artifacts and verification evidence are ready for truthful
  roadmap and requirements closeout.
