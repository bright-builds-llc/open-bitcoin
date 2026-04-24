---
phase: 10-benchmarks-and-audit-readiness
plan: 03
subsystem: benchmarks
tags: [bash, ci, benchmarks, parity, audit]

requires:
  - phase: 10-02
    provides: Executable benchmark CLI with smoke/full modes and JSON/Markdown reports
provides:
  - Repo-owned benchmark wrapper script
  - Bounded benchmark smoke invocation from repo verification
  - CI benchmark report artifact upload
  - Reviewer-facing benchmark parity documentation
affects: [benchmarking, parity, audit-readiness, ci, verification]

tech-stack:
  added: []
  patterns:
    - Bash-array CLI forwarding for benchmark wrapper arguments
    - Threshold-free benchmark smoke reports in repo verification
    - CI artifact retention for generated benchmark evidence

key-files:
  created:
    - scripts/run-benchmarks.sh
    - docs/parity/benchmarks.md
    - .planning/phases/10-benchmarks-and-audit-readiness/10-03-SUMMARY.md
  modified:
    - scripts/verify.sh
    - .github/workflows/ci.yml

key-decisions:
  - "Use scripts/run-benchmarks.sh as the contributor-facing benchmark entrypoint and forward only planned options through Bash arrays."
  - "Keep benchmark reports as audit and trend evidence rather than release timing gates."
  - "Make Knots JSON/bin inputs optional metadata enrichment while preserving mapping-only as the default comparison."

patterns-established:
  - "OPEN_BITCOIN_BENCHMARK_REPORT_DIR controls benchmark report output across local verification and CI."
  - "CI uploads benchmark reports as a separate benchmark-reports artifact with missing files ignored."

requirements-completed: [PAR-02]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 10-2026-04-24T10-47-33
generated_at: 2026-04-24T12:12:50Z

duration: 5 min
completed: 2026-04-24
---

# Phase 10 Plan 03: Benchmark Verification and Audit Wiring Summary

Repo-owned benchmark execution now feeds local verification, CI artifact retention, and reviewer-facing parity documentation without timing release gates.

## Performance

- **Duration:** 5 min
- **Started:** 2026-04-24T12:07:55Z
- **Completed:** 2026-04-24T12:12:50Z
- **Tasks:** 2
- **Files modified:** 5

## Accomplishments

- Added `scripts/run-benchmarks.sh` with safe Bash-array forwarding for `--list`, smoke/full runs, iteration counts, output directories, and optional Knots metadata paths.
- Updated `scripts/verify.sh` to create `OPEN_BITCOIN_BENCHMARK_REPORT_DIR`, run bounded `--smoke` reports, and include `//:bench` in the Bazel smoke build.
- Added CI retention for generated benchmark reports as a separate `benchmark-reports` artifact.
- Documented benchmark scope, groups, Knots mapping, local commands, report paths, and non-goals in `docs/parity/benchmarks.md`.

## Task Commits

Each task was committed atomically:

1. **Task 1: Add benchmark wrapper script with bounded smoke defaults** - `d8a79e3` (feat)
2. **Task 2: Add verify, CI, and reviewer documentation wiring** - `0a66639` (feat)

## Files Created/Modified

- `scripts/run-benchmarks.sh` - Repo-owned benchmark wrapper with `OPEN_BITCOIN_BENCHMARK_REPORT_DIR` defaulting and array-based Cargo invocation.
- `scripts/verify.sh` - Runs threshold-free benchmark smoke reports and builds `//:bench`.
- `.github/workflows/ci.yml` - Exports the benchmark report directory and uploads `benchmark-reports`.
- `docs/parity/benchmarks.md` - Explains benchmark groups, mapping-only default comparison, local runs, reports, and non-goals.
- `.planning/phases/10-benchmarks-and-audit-readiness/10-03-SUMMARY.md` - Plan execution record.

## Decisions Made

- Use `scripts/run-benchmarks.sh` as the contributor-facing wrapper instead of asking contributors to remember the Cargo manifest and package flags.
- Keep local verification bounded to `--smoke`; full mode stays opt-in with explicit iterations.
- Preserve mapping-only as the default Knots comparison and treat explicit Knots JSON/bin paths as report metadata enrichment.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

- `shfmt -d scripts/verify.sh` would re-indent the entire pre-existing file. To avoid unrelated formatting churn, only the new wrapper was kept `shfmt`-clean, and `scripts/verify.sh` was validated with `bash -n` plus full repo verification.
- Bazel emitted existing third-party C warning noise from dependency build scripts during `//:bench`; the build completed successfully.

## Known Stubs

None. Stub scanning found only shell control variables initialized to empty strings in `scripts/verify.sh`, not placeholder behavior or unwired data.

## Threat Flags

None. The shell argument boundary, bounded smoke invocation, CI artifact path, and benchmark documentation claims were covered by the plan threat model.

## Verification

- `bash -n scripts/run-benchmarks.sh`
- `shfmt -d scripts/run-benchmarks.sh`
- `bash scripts/run-benchmarks.sh --list`
- `bash scripts/run-benchmarks.sh --smoke --output-dir packages/target/benchmark-reports`
- Plan marker check for wrapper options and forbidden shell-evaluation patterns
- Plan marker check for benchmark report groups, docs sections, and CI artifact settings
- `bash -n scripts/verify.sh`
- `bash scripts/verify.sh`
- Git hooks ran `bash scripts/verify.sh` during both task commits

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Plan 10-04 can consume checked-in benchmark documentation and generated smoke reports from `packages/target/benchmark-reports` without requiring default Knots execution.

## Self-Check: PASSED

- Found summary, wrapper script, and benchmark docs on disk.
- Found task commits `d8a79e3` and `0a66639` in git history.

---
*Phase: 10-benchmarks-and-audit-readiness*
*Completed: 2026-04-24*
