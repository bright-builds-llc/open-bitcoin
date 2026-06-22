---
phase: 86-service-operation-expectations
plan: 03
subsystem: automation
tags:
  - service-expectations
  - verifier
  - bun
requires:
  - phase: 86-service-operation-expectations
    provides: Canonical service expectations and parity roots
provides:
  - Deterministic Phase 86 checker tests
  - Narrow Phase 86 checker implementation
  - Default verifier wiring after Phase 85
affects:
  - scripts/check-phase86-service-operation-expectations.test.ts
  - scripts/check-phase86-service-operation-expectations.ts
  - scripts/verify.sh
tech-stack:
  added: []
  patterns:
    - Fixed-file Bun checker mirroring Phase 85 verification style
key-files:
  created:
    - scripts/check-phase86-service-operation-expectations.test.ts
    - scripts/check-phase86-service-operation-expectations.ts
  modified:
    - scripts/verify.sh
key-decisions:
  - "The Phase 86 checker stays fixed-file and does not become the later broad production-claim scanner."
  - "Verifier text checks strip the legacy VERIFY_COMMAND_ORDER heredoc before validating executed Phase 86 order."
patterns-established:
  - "Phase-specific parity docs are guarded by fixture tests plus a pure exported checker function."
requirements-completed:
  - SVC-01
  - SVC-02
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 86-2026-06-22T19-33-52
generated_at: 2026-06-22T20:25:43Z
duration: 9min
completed: 2026-06-22
---

# Phase 86 Plan 03: Deterministic Checker Summary

**Phase 86 service expectations now have deterministic fixture tests, a narrow checker, and default verifier wiring after Phase 85.**

## Performance

- **Duration:** 9 min
- **Started:** 2026-06-22T20:16:26Z
- **Completed:** 2026-06-22T20:25:43Z
- **Tasks:** 3
- **Files modified:** 3

## Accomplishments

- Added `scripts/check-phase86-service-operation-expectations.test.ts` with the required eleven fixture tests and Arrange/Act/Assert sections.
- Added `scripts/check-phase86-service-operation-expectations.ts` exporting `checkPhase86ServiceOperationExpectations`.
- Wired the Phase 86 test and checker into `scripts/verify.sh` immediately after Phase 85 in both visible command order and executed `run_step` order.

## Task Commits

Plan work is being committed once at the wrapper finalization gate after full Phase 86 verification passes.

## Files Created/Modified

- `scripts/check-phase86-service-operation-expectations.test.ts` - Fixture tests for document, parity-root, pointer, and verifier drift.
- `scripts/check-phase86-service-operation-expectations.ts` - Fixed-target Phase 86 checker and CLI entrypoint.
- `scripts/verify.sh` - Default verifier command order and execution wiring for Phase 86.

## Decisions Made

The checker follows the existing Phase 85 structure instead of introducing a shared abstraction. This keeps Phase 86 scoped and avoids broad repository scanning or live operator command execution.

## Deviations from Plan

None - plan executed exactly as written.

**Total deviations:** 0 auto-fixed.
**Impact on plan:** No scope change.

## Issues Encountered

The first focused run showed two checker text-contract mismatches: restart/resume proof wording belonged to the restart section, and several default-boundary phrases wrap across lines in the canonical document. The checker now validates those with section-appropriate normalized text checks.

## Verification

- `bun test scripts/check-phase86-service-operation-expectations.test.ts`
- `bun --check scripts/check-phase86-service-operation-expectations.ts`
- `bun run scripts/check-phase86-service-operation-expectations.ts`
- `git diff --check -- scripts/check-phase86-service-operation-expectations.ts scripts/check-phase86-service-operation-expectations.test.ts scripts/verify.sh`
- `rg -n "OPEN_BITCOIN_PHASE86_REPO_ROOT|v1-8-service-operation-expectations|v1_8_service_operation_expectations|SVC-01|SVC-02|check-phase85-operator-runbooks|check-phase86-service-operation-expectations|service-operation-expectations.md|VERIFY_COMMAND_ORDER|real-service-manager-free" scripts/check-phase86-service-operation-expectations.ts scripts/check-phase86-service-operation-expectations.test.ts scripts/verify.sh`

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Ready for `86-04`: run focused checks, refresh generated LOC if needed, run `bash scripts/verify.sh`, and record phase-level verification evidence.

---
*Phase: 86-service-operation-expectations*
*Completed: 2026-06-22*
