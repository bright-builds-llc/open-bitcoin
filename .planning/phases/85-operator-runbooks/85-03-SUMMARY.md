---
phase: 85-operator-runbooks
plan: 03
subsystem: automation
tags: [bun, checker, verify-sh, parity-guard]
requires:
  - phase: 85-operator-runbooks
    provides: canonical runbook and parity root registration
provides:
  - Deterministic Phase 85 checker
  - Fixture tests for runbook drift and verifier-order drift
  - scripts/verify.sh wiring after Phase 84
affects: [phase85, verification, parity-ledger]
tech-stack:
  added: []
  patterns: [fixed-target Bun checker, fixture repository roots, executed verifier-order validation]
key-files:
  created:
    - scripts/check-phase85-operator-runbooks.ts
    - scripts/check-phase85-operator-runbooks.test.ts
  modified:
    - scripts/verify.sh
key-decisions:
  - "Keep the checker narrow to Phase 85 target files instead of scanning all docs."
  - "Validate executed verifier order after stripping the legacy VERIFY_COMMAND_ORDER heredoc."
patterns-established:
  - "Phase 85 checker rejects missing RUN-02 monitoring/opt-in evidence and unsafe proof language."
requirements-completed: [RUN-01, RUN-02, RUN-03]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 85-2026-06-22T11-57-13
generated_at: 2026-06-22T15:15:40Z
duration: 36min
completed: 2026-06-22
---

# Phase 85: Operator Runbooks Plan 03 Summary

**Focused Bun checker now guards the Phase 85 runbook, parity roots, entrypoint links, and verifier wiring.**

## Performance

- **Duration:** 36 min
- **Started:** 2026-06-22T14:39:00Z
- **Completed:** 2026-06-22T15:15:40Z
- **Tasks:** 3
- **Files modified:** 3

## Accomplishments

- Added `scripts/check-phase85-operator-runbooks.test.ts` with nine deterministic fixture tests.
- Added `scripts/check-phase85-operator-runbooks.ts` using fixed target files and pure validation helpers.
- Wired Phase 85 test and checker commands into both the visible and executed `scripts/verify.sh` order immediately after Phase 84.

## Task Commits

Intermediate task commits were intentionally deferred. The wrapper requires a clean Phase 85 verification result and final full `bash scripts/verify.sh` pass before git finalization.

## Files Created/Modified

- `scripts/check-phase85-operator-runbooks.test.ts` - Fixture tests for required content, unsafe claims, parity roots, and verifier-order drift.
- `scripts/check-phase85-operator-runbooks.ts` - Narrow deterministic checker for Phase 85 targets.
- `scripts/verify.sh` - Adds Phase 85 checker execution after Phase 84.

## Decisions Made

The checker validates only the Phase 85 document set, including runbook, parity roots, pointer docs, operator entrypoints, catalog row, and `scripts/verify.sh`. Broad all-doc production-claim scanning remains Phase 88 scope.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

`bun --check scripts/check-phase85-operator-runbooks.ts` invokes enough script behavior to require real `scripts/verify.sh` wiring. The command was therefore run successfully after Task 3 wired the verifier, matching the revised dependency order.

## Verification

Passed:

```bash
bun test scripts/check-phase85-operator-runbooks.test.ts
bun --check scripts/check-phase85-operator-runbooks.ts
bun run scripts/check-phase85-operator-runbooks.ts
rg -n "bun test scripts/check-phase85-operator-runbooks.test.ts|bun run scripts/check-phase85-operator-runbooks.ts" scripts/verify.sh
git diff --check -- scripts/check-phase85-operator-runbooks.ts scripts/check-phase85-operator-runbooks.test.ts scripts/verify.sh
```

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Wave 4 can refresh generated LOC, rerun focused Phase 85 checks, and execute the full repo-native verifier.

---
*Phase: 85-operator-runbooks*
*Completed: 2026-06-22*
