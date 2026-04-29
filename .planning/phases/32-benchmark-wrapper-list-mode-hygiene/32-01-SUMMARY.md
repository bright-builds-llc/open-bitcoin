---
phase: 32-benchmark-wrapper-list-mode-hygiene
plan: "01"
status: completed
generated_by: gsd-execute-phase
lifecycle_mode: yolo
phase_lifecycle_id: 32-2026-04-29T22-28-38
generated_at: 2026-04-29T22:35:43.739Z
completed: 2026-04-29
---

# Phase 32 Plan 01 Summary

## One-Liner

The benchmark wrapper now initializes its shared Cargo command before the
`--list` fast path, so `bash scripts/run-benchmarks.sh --list` works again and
the repo-native verification contract directly guards that path.

## What Was Built

- Updated `scripts/run-benchmarks.sh` so the shared `cargo run` invocation is
  constructed before the list-mode branch instead of after it.
- Kept the existing `--list cannot be combined with run options` guard and the
  existing smoke/full argument behavior unchanged.
- Updated `scripts/verify.sh` to run `bash scripts/run-benchmarks.sh --list`
  before the existing smoke benchmark wrapper call, giving the shipped wrapper
  surface a durable regression guard.

## Task Commits

1. **Task 1: repair benchmark wrapper list mode and add a durable guard** —
   Pending the wrapper-owned Phase 32 finalization commit.

## Verification

Passed:

- `bash scripts/run-benchmarks.sh --list`
- `bash scripts/run-benchmarks.sh --smoke --output-dir packages/target/benchmark-reports`

## Deviations from Plan

- None. The control-flow repair stayed as narrow as planned, and the
  verify-time `--list` invocation was enough to cover the actual wrapper
  surface without adding a separate harness.

## Self-Check: PASSED

- `--list` no longer aborts on an uninitialized array.
- The smoke path still writes the normal benchmark report files after the
  wrapper repair.
