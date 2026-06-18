---
phase: 80-opt-in-soak-uat-and-release-boundaries
plan: 03
subsystem: verification-release-boundaries
tags: [verification, parity, release-boundaries, v1.7, bun]

requires:
  - phase: 80 plan 01
    provides: Phase 80 UAT matrix and release-boundary framing
  - phase: 80 plan 02
    provides: v1.7 parity closeout roots and non-claim register
provides:
  - deterministic Phase 80 release-boundary checker
  - fixture-root regression tests for v1.7 claim and verifier drift
  - repo-native verifier wiring after Phase 79
  - passed Phase 80 verification evidence
affects: [phase-80, verification, parity, release-readiness]

tech-stack:
  added: []
  patterns:
    - Bun TypeScript checker with exported validation function and CLI wrapper
    - fixture-root tests assert exact failure lines without nested subprocess capture

key-files:
  created:
    - scripts/check-phase80-opt-in-soak-uat-release-boundaries.ts
    - .planning/phases/80-opt-in-soak-uat-and-release-boundaries/80-VERIFICATION.md
    - .planning/phases/80-opt-in-soak-uat-and-release-boundaries/80-03-SUMMARY.md
  modified:
    - scripts/check-phase80-opt-in-soak-uat-release-boundaries.test.ts
    - scripts/verify.sh
    - docs/metrics/lines-of-code.md

key-decisions:
  - "Keep Phase 80 verification local and deterministic by checking existing docs, parity roots, source anchors, and verifier wiring instead of adding public-network, service-manager, process-table, or large-disk gates."
  - "Expose the checker validation as a pure function so fixture tests can assert concrete failure lines while the CLI still uses OPEN_BITCOIN_PHASE80_REPO_ROOT and exits non-zero on failures."
  - "Require the complete Phase 75 through Phase 80 verifier command sequence so removing either Phase 80 verifier command is a checker failure."

patterns-established:
  - "Release-boundary closeout checks should guard exact current claim-bearing files and historical evidence roots without broad scans for generic production/mainnet terms."
  - "Default verifier boundary checks should reject concrete forbidden command strings and prove checker ordering in `scripts/verify.sh`."

requirements-completed: [VER-05, VER-06, VER-07, REL-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 80-2026-06-17T22-54-57
generated_at: 2026-06-18T05:28:37Z

duration: 40m
completed: 2026-06-18
---

# Phase 80 Plan 03: Boundary Checker and Verification Summary

**Deterministic v1.7 release-boundary checker wired into local repo verification after Phase 79**

## Performance

- **Duration:** 40m
- **Started:** 2026-06-18T03:15:26Z
- **Completed:** 2026-06-18T03:55:18Z
- **Tasks:** 2
- **Files modified:** 5

## Accomplishments

- Added `scripts/check-phase80-opt-in-soak-uat-release-boundaries.ts` to validate Phase 80 requirements, UAT matrix anchors, v1.7 parity roots, support/soak source anchors, forbidden manifest paths, default verifier exclusions, and Phase 75-80 checker ordering.
- Added fixture-root regression coverage for positive and negative UAT anchors, parity roots, verify ordering/forbidden strings, broad claim drift, and forbidden manifest creation.
- Wired the Phase 80 test/checker pair immediately after Phase 79 in `scripts/verify.sh`.
- Regenerated tracked LOC metrics and recorded passed Phase 80 verification evidence.

## Task Commits

1. **Task 1 RED: failing Phase 80 checker tests** - `6474502` (test)
2. **Task 1 GREEN: deterministic Phase 80 checker** - `00850ef` (feat)
3. **Task 2: verifier wiring and verification evidence** - `e77ea20` (chore)
4. **Code review fix: require Phase 80 verifier commands** - `f887b73` (fix)

## Files Created/Modified

- `scripts/check-phase80-opt-in-soak-uat-release-boundaries.ts` - Deterministic checker for v1.7 release-boundary and verifier drift.
- `scripts/check-phase80-opt-in-soak-uat-release-boundaries.test.ts` - Fixture-root regression tests with required positive and negative cases.
- `scripts/verify.sh` - Runs the Phase 80 checker test and checker immediately after Phase 79.
- `docs/metrics/lines-of-code.md` - Refreshed tracked LOC report for the two new TypeScript checker files and verifier wiring.
- `.planning/phases/80-opt-in-soak-uat-and-release-boundaries/80-VERIFICATION.md` - Records focused checks and full `bash scripts/verify.sh` success.
- `.planning/phases/80-opt-in-soak-uat-and-release-boundaries/80-03-SUMMARY.md` - Records this execution.

## Decisions Made

- Used a pure exported checker function plus CLI wrapper rather than nested CLI assertions in tests, because Bun fixture tests need deterministic access to concrete failure lines.
- Kept `scripts/verify.sh` local and bounded; no public-network sync, manual peer, service-manager, process-table, current-tip, multi-day sleep, large-disk, or wallet-mutation command was added.
- Left `.planning/STATE.md` and `.planning/ROADMAP.md` untouched and unstaged because the orchestrator owns those files after phase execution.

## Verification

- `bun test scripts/check-phase80-opt-in-soak-uat-release-boundaries.test.ts` - passed, 6 tests.
- `bun --check scripts/check-phase80-opt-in-soak-uat-release-boundaries.ts` - passed.
- `bun run scripts/check-phase80-opt-in-soak-uat-release-boundaries.ts` - passed against the real worktree.
- `bun run scripts/check-parity-breadcrumbs.ts --check` - passed, 268 Rust files verified.
- `bash scripts/verify.sh` - passed end to end in 23m 13.609s.
- Simplification pass confirmed no Rust source/test files were added, no new v1.7 evidence manifest exists, the Phase 80 UAT matrix has exactly four workflows, and `scripts/verify.sh` has no forbidden default-verification strings.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Avoided nested Bun subprocess output loss in fixture tests**
- **Found during:** Task 1 (Add deterministic Phase 80 checker and fixture tests)
- **Issue:** Negative fixture tests could observe the checker exit status but not its failure output when the checker was spawned as a nested Bun subprocess under `bun:test`.
- **Fix:** Exported the checker validation as a pure function returning failure lines; the CLI wrapper still reads `OPEN_BITCOIN_PHASE80_REPO_ROOT`, prints concrete failures, and exits non-zero.
- **Files modified:** `scripts/check-phase80-opt-in-soak-uat-release-boundaries.ts`, `scripts/check-phase80-opt-in-soak-uat-release-boundaries.test.ts`
- **Verification:** `bun test scripts/check-phase80-opt-in-soak-uat-release-boundaries.test.ts` and `bun run scripts/check-phase80-opt-in-soak-uat-release-boundaries.ts` passed.
- **Committed in:** `00850ef`

***

**Total deviations:** 1 auto-fixed (Rule 3 blocking)
**Impact on plan:** No scope expansion. The checker remains local, fixture-rooted, and CLI-compatible while tests assert exact failure lines deterministically.

### Code Review Follow-up

**1. [Advisory review fix] Required Phase 80 verifier commands**
- **Found during:** Phase-level code review after plan execution
- **Issue:** The first checker implementation failed when one Phase 80 verifier command was removed, but it still passed when both Phase 80 commands were removed from `scripts/verify.sh`.
- **Fix:** Switched verify-order validation to require the complete Phase 75 through Phase 80 command sequence and added `fails_when_phase80_verify_commands_are_missing`.
- **Files modified:** `scripts/check-phase80-opt-in-soak-uat-release-boundaries.ts`, `scripts/check-phase80-opt-in-soak-uat-release-boundaries.test.ts`
- **Verification:** `bun test scripts/check-phase80-opt-in-soak-uat-release-boundaries.test.ts`, `bun --check scripts/check-phase80-opt-in-soak-uat-release-boundaries.ts`, `bun run scripts/check-phase80-opt-in-soak-uat-release-boundaries.ts`, and the commit hook's `bash scripts/verify.sh` passed.
- **Committed in:** `f887b73`

## Issues Encountered

- The broad placeholder simplification scan matched an existing sentence that says service/status/dashboard docs are "not as placeholders." That was not a functional stub or v1.7 placeholder claim, so no out-of-scope documentation edit was made.

## Known Stubs

None. Stub scan matches were limited to checker forbidden-string guards and shell variable initializers; no UI/data-flow stub or placeholder implementation was introduced.

## Threat Flags

None. This plan introduced a local repository checker and verifier wiring only. The plan threat mitigations for default-verifier drift and release-wording drift are covered by the checker and `scripts/verify.sh` ordering.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Phase 80 now has deterministic closure evidence: the checker guards v1.7 claim drift and verifier boundaries, `scripts/verify.sh` runs Phase 75 through Phase 80 checks in order, and `80-VERIFICATION.md` records a passed full repo verification. Future production-adjacent claims still require separate scoped plans.

## Self-Check: PASSED

- Found created files: `scripts/check-phase80-opt-in-soak-uat-release-boundaries.ts`, `.planning/phases/80-opt-in-soak-uat-and-release-boundaries/80-VERIFICATION.md`, and `.planning/phases/80-opt-in-soak-uat-and-release-boundaries/80-03-SUMMARY.md`.
- Found task commits: `6474502`, `00850ef`, `e77ea20`, and post-review fix `f887b73`.
- Confirmed `.planning/STATE.md` remains unstaged and orchestrator-owned.

***
*Phase: 80-opt-in-soak-uat-and-release-boundaries*
*Completed: 2026-06-18*
