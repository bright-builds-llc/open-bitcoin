---
phase: 01-workspace-baseline-and-guardrails
plan: 03
subsystem: testing
tags: [verification, coverage, ci, shell]
requires:
  - phase: 01-02
    provides: repo-root Bazel surface and first-party crate targets
provides:
  - pure-core dependency and import policy checker
  - repo-native verification script
  - GitHub Actions workflow mirroring local verification
affects: [ci, contributor-workflow, pure-core-architecture]
tech-stack:
  added: [cargo-llvm-cov, GitHub Actions verification workflow]
  patterns: [pure-core allowlist gate, single repo verification entrypoint]
key-files:
  created:
    - scripts/pure-core-crates.txt
    - scripts/check-pure-core-deps.sh
    - scripts/verify.sh
    - .github/workflows/ci.yml
    - packages/Cargo.lock
  modified:
    - packages/open-bitcoin-core/src/lib.rs
key-decisions:
  - "Made the pure-core boundary enforceable through both dependency graph checks and forbidden-import scans."
  - "Defined bash scripts/verify.sh as the single contributor-facing verification contract."
patterns-established:
  - "Architecture policy checks run before formatting, lint, build, test, and coverage."
  - "CI mirrors the same verification script used locally instead of reconstructing partial checks."
requirements-completed: [ARCH-02, ARCH-04, VER-01, VER-02]
generated_by: gsd-execute-plan
lifecycle_mode: interactive
phase_lifecycle_id: 01-2026-04-11T11-36-20
generated_at: 2026-04-11T12:10:11Z
duration: 1 min
completed: 2026-04-11
---

# Phase 1 Plan 03: Workspace, Baseline, and Guardrails Summary

**Established a hard-failing pure-core policy gate, a single repo verification script, and CI that mirrors the same contract.**

## Performance

- **Duration:** 1 min
- **Started:** 2026-04-11T12:09:58Z
- **Completed:** 2026-04-11T12:10:11Z
- **Tasks:** 3
- **Files modified:** 6

## Accomplishments
- Added a pure-core allowlist plus dependency/import checker
- Created `scripts/verify.sh` as the single repo-native verification contract
- Added CI that runs the same verification contract and a Bazel smoke build

## Task Commits

Each task was committed atomically:

1. **Task 1: Implement pure-core architecture policy checks** - `c0f5c7b` (feat)
2. **Task 2: Create the repo-native verification entrypoint** - `3a8ee4f` (feat)
3. **Task 3: Mirror the verification contract in GitHub Actions** - `dc86a3e` (ci)

**Plan metadata:** pending in docs commit after state and roadmap updates

## Files Created/Modified
- `scripts/pure-core-crates.txt` - pure-core crate allowlist for policy checks
- `scripts/check-pure-core-deps.sh` - hard-failing dependency/import guard for pure-core crates
- `scripts/verify.sh` - single repo-native verification command
- `.github/workflows/ci.yml` - CI workflow that runs the same verification contract
- `packages/Cargo.lock` - initial workspace lockfile for reproducible cargo verification
- `packages/open-bitcoin-core/src/lib.rs` - minimal smoke-testable function and unit test for the coverage gate

## Decisions Made
- Used a repo-owned shell script for the architecture policy gate instead of adding a heavyweight lint framework immediately.
- Made CI call `bash scripts/verify.sh` directly so local and remote verification stay aligned.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Added a tiny covered smoke test so the pure-core coverage gate emits real data**
- **Found during:** Task 2 (Create the repo-native verification entrypoint)
- **Issue:** `cargo llvm-cov` failed with `no coverage data found` because the pure-core crate had no executable covered code yet.
- **Fix:** Added `crate_ready()` plus a unit test in `packages/open-bitcoin-core/src/lib.rs`, then reran `scripts/verify.sh` to confirm 100% line coverage.
- **Files modified:** `packages/open-bitcoin-core/src/lib.rs`, `packages/Cargo.lock`
- **Verification:** `bash scripts/verify.sh` completed successfully and reported 100.00% coverage for `open-bitcoin-core`
- **Committed in:** `3a8ee4f` (Task 2 commit)

---

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** No scope creep. The smoke test exists only to make the planned coverage gate meaningful on an otherwise empty pure-core crate.

## Issues Encountered
- `cargo-llvm-cov` initially prompted for `llvm-tools-preview`, so the Rust LLVM tools component had to be installed once before the verification script became fully non-interactive.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- The repo now has enforceable local and CI verification gates for the workspace.
- Phase 1 is ready for parity/deviation ledger seeding and contributor-doc alignment in Plan 04.

---
*Phase: 01-workspace-baseline-and-guardrails*
*Completed: 2026-04-11*
