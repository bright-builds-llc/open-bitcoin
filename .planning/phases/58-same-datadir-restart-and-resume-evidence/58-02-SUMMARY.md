---
phase: 58-same-datadir-restart-and-resume-evidence
plan: 02
subsystem: live-smoke
tags: [typescript, bun, restart, same-datadir, smoke-report]
requires:
  - phase: 58-same-datadir-restart-and-resume-evidence
    provides: deterministic same-datadir durable resume tests
provides:
  - opt-in `--restart-after-progress` live-smoke flow
  - compact `result.restartResumeEvidence` schema
  - mocked two-session restart fixture coverage
affects: [operator-uat, resume-evidence, recovery-diagnosis]
tech-stack:
  added: []
  patterns: [single-session helper, compact restart evidence schema, mocked daemon restart fixture]
key-files:
  created: []
  modified:
    - scripts/run-live-mainnet-smoke.ts
    - scripts/test-run-live-mainnet-smoke.sh
key-decisions:
  - "Normal reports explicitly emit `result.restartResumeEvidence: null`."
  - "Restart mode relaunches a second daemon with a new RPC port and the same selected datadir."
  - "Restart evidence stays compact and excludes raw tails, raw endpoint tables, snapshots arrays, and manual peers."
patterns-established:
  - "Restart UAT remains opt-in through `--restart-after-progress` and stays outside `scripts/verify.sh`."
requirements-completed: [RESUME-01, RESUME-02]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 58-2026-06-05T12-58-05
generated_at: 2026-06-05T13:50:35Z
duration: 42min
completed: 2026-06-05
---

# Phase 58: Plan 02 Summary

**Opt-in two-session same-datadir restart smoke evidence with compact JSON and Markdown report fields.**

## Performance

- **Duration:** 42 min
- **Started:** 2026-06-05T13:08:00Z
- **Completed:** 2026-06-05T13:33:22Z
- **Tasks:** 2
- **Files modified:** 2

## Accomplishments

- Added `--restart-after-progress` parsing and help text.
- Added `result.restartResumeEvidence` with same-datadir checks, restart status, before/after summaries, duplicate-connect verdict, peer outcome summary, and post-restart delta.
- Added `maybeLastSuccessfulProgressUnixSeconds` parsing for status snapshots and final status summaries.
- Added a mocked restart fixture proving the daemon starts twice and the report uses restart evidence as the restart proof.
- Added code-review regression guards for unchanged-height hash mismatch, post-restart status failure, and actual per-session command reporting.

## Task Commits

Task commits are deferred to the final strict yolo push gate for this run. No code is committed until phase verification and repo verification pass.

## Files Created/Modified

- `scripts/run-live-mainnet-smoke.ts` - Adds restart flag, reusable daemon session runner, compact restart evidence, actual per-session command reporting, and Markdown rendering.
- `scripts/test-run-live-mainnet-smoke.sh` - Adds normal-report null evidence assertions, deterministic two-session restart fixture coverage, and restart regression cases.

## Decisions Made

- Reused the existing smoke runner process model with a session helper instead of introducing another orchestration script.
- Passed restart mode when the fresh post-restart status snapshot preserves durable heights and hashes, without requiring fresh public-network progress after restart.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Same-height restart hash mismatch could pass**

- **Found during:** Code review after Task 2.
- **Issue:** Restart completion required non-decreasing heights but did not reject a changed downloaded or connected hash at the same height.
- **Fix:** Required hash stability when downloaded or connected heights are unchanged and surfaced the mismatch as `duplicate_connect_suspected`.
- **Files modified:** `scripts/run-live-mainnet-smoke.ts`, `scripts/test-run-live-mainnet-smoke.sh`
- **Verification:** `bash scripts/test-run-live-mainnet-smoke.sh` includes a same-height/different-hash restart fixture that fails.

**2. [Rule 1 - Bug] Post-restart status failure used misleading wording**

- **Found during:** Code review after Task 2.
- **Issue:** Missing post-restart snapshots could be described as no pre-restart progress.
- **Fix:** Classified second-session status failure as post-restart runtime failure and preserved restart-specific failure messages.
- **Files modified:** `scripts/run-live-mainnet-smoke.ts`, `scripts/test-run-live-mainnet-smoke.sh`
- **Verification:** `bash scripts/test-run-live-mainnet-smoke.sh` includes a second-session status failure fixture.

**3. [Rule 1 - Bug] Report commands could show preview RPC ports**

- **Found during:** Code review after Task 2.
- **Issue:** Top-level report commands were built from preview command specs while actual sessions allocated fresh RPC ports.
- **Fix:** Reported actual first-session commands at the existing top-level fields and added `daemon_sessions` plus Markdown rows for every actual session.
- **Files modified:** `scripts/run-live-mainnet-smoke.ts`, `scripts/test-run-live-mainnet-smoke.sh`
- **Verification:** `bash scripts/test-run-live-mainnet-smoke.sh` asserts `daemon_sessions` in JSON and `Daemon Sessions` in Markdown.

**Total deviations:** 3 auto-fixed (Rule 1 bugs).
**Impact on plan:** Fixes tightened restart evidence correctness without broadening scope.

## Issues Encountered

None.

## Verification

- `bash scripts/test-run-live-mainnet-smoke.sh` - passed, including code-review regression cases.
- `bun run scripts/run-live-mainnet-smoke.ts --help` - passed and prints `--restart-after-progress`.
- `rg -n "restartAfterProgress|--restart-after-progress|restartResumeEvidence|type RestartStatus|type DuplicateConnectVerdict|maybeLastSuccessfulProgressUnixSeconds" scripts/run-live-mainnet-smoke.ts` - found the schema/flag contract.
- `rg -n -- "--restart-after-progress|restartResumeEvidence|restartStatus|duplicateConnectVerdict|maybePostRestartProgressDelta|requestedPathMatched|resolvedPathMatched" scripts/test-run-live-mainnet-smoke.sh` - found restart fixture assertions.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Plan 58-03 can add typed recovery diagnosis and operator/parity docs on top of `result.restartResumeEvidence`.

---
*Phase: 58-same-datadir-restart-and-resume-evidence*
*Completed: 2026-06-05*
