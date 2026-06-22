---
phase: 85-operator-runbooks
plan: 01
subsystem: docs
tags: [operator-runbooks, parity, support-evidence, recovery]
requires:
  - phase: 82-production-claim-boundary
    provides: v1.8 support terms and deferred production-surface boundaries
  - phase: 83-support-matrix-and-issue-evidence
    provides: issue evidence and redaction rules
  - phase: 84-upgrade-and-rollback-policy
    provides: no-hidden-mutation rollback and compatibility boundaries
provides:
  - Canonical Phase 85 operator runbook for RUN-01, RUN-02, and RUN-03
affects: [phase85, operator-docs, parity-ledger, support-bundles]
tech-stack:
  added: []
  patterns: [canonical parity runbook, field-level evidence tables]
key-files:
  created:
    - docs/parity/operator-runbooks.md
  modified: []
key-decisions:
  - "Use docs/parity/operator-runbooks.md as the single canonical runbook."
  - "Keep all long-running public-network, stay-current, and multi-day evidence opt-in and outside default verification."
patterns-established:
  - "Operator procedures distinguish review-only evidence from local state mutation."
  - "Recovery guidance uses safe_retry, read_only_inspection, backup_then_rebuild, and stop_and_escalate only."
requirements-completed: [RUN-01, RUN-02, RUN-03]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 85-2026-06-22T11-57-13
generated_at: 2026-06-22T15:03:45Z
duration: 24min
completed: 2026-06-22
---

# Phase 85: Operator Runbooks Plan 01 Summary

**Canonical source-built operator runbook with preflight, no-progress, recovery, escalation, and redacted support-bundle timeline guidance.**

## Performance

- **Duration:** 24 min
- **Started:** 2026-06-22T14:39:00Z
- **Completed:** 2026-06-22T15:03:45Z
- **Tasks:** 3
- **Files modified:** 1

## Accomplishments

- Created `docs/parity/operator-runbooks.md` with surface id `v1-8-operator-runbooks`.
- Added production-boundary preflight evidence, repo-local Cargo/Bazel status commands, and review-only mutation boundaries.
- Added long-run monitoring, no-progress diagnosis, recovery/stop decisions, escalation thresholds, support-bundle timeline, and privacy boundaries.

## Task Commits

Intermediate task commits were intentionally deferred. The wrapper requires a clean Phase 85 verification result and final full `bash scripts/verify.sh` pass before git finalization.

## Files Created/Modified

- `docs/parity/operator-runbooks.md` - Canonical Phase 85 runbook for RUN-01, RUN-02, and RUN-03.

## Decisions Made

The runbook uses existing v1.3 through v1.7 field vocabulary rather than introducing a new incident schema. It also keeps public-network, stay-current, and multi-day reports explicitly opt-in so default verification remains deterministic.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

Two acceptance-check string issues were fixed while creating the runbook: `elapsed time` needed exact lowercase wording, and the default verifier boundary sentence needed to stay on one line for fixed-string validation.

## Verification

Passed:

```bash
bash -lc 'set -euo pipefail; for needle in ...; do rg -F -- "$needle" docs/parity/operator-runbooks.md >/dev/null; done; git diff --check -- docs/parity/operator-runbooks.md'
```

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Wave 2 can register and link the canonical runbook through parity roots and operator entrypoints. No blockers remain from Wave 1.

---
*Phase: 85-operator-runbooks*
*Completed: 2026-06-22*
