---
phase: 125-compact-download-verification-traceability-closure
plan: "01"
subsystem: verification-traceability
tags: [bun, typescript, lifecycle, requirements, verification]
requires:
  - phase: 124-milestone-closeout-reconciliation
    provides: v2.1 audit findings identifying the three verification-orphan gaps
provides:
  - Active-milestone requirement activation and exact-once ownership parsing
  - Lifecycle-valid active-phase verification coverage enforcement
  - Focused mutation fixtures plus a strict staged real-repository smoke test
affects:
  - 125-04-lifecycle-valid-evidence-and-requirement-promotion
  - 126-compact-relay-residual-hardening
tech-stack:
  added: []
  patterns:
    - Functional-core parsing with a thin filesystem and CLI shell
    - Exact-token evidence matching after lifecycle provenance validation
key-files:
  created:
    - scripts/check-active-milestone-verification-traceability.ts
    - scripts/check-active-milestone-verification-traceability.test.ts
  modified:
    - docs/metrics/lines-of-code.md
key-decisions:
  - "Requirement activation requires one unique active checklist entry, exactly one traceability row, an active-roadmap owner, and summary completion."
  - "Only passed, lifecycle-validated verification whose mode and lifecycle identity match its phase CONTEXT can satisfy coverage."
  - "Invalid lifecycle evidence still exposes the activated orphan and cannot authorize coverage."
patterns-established:
  - "Verification traceability derives its corpus from active planning metadata rather than a milestone-specific ID manifest."
  - "Archived artifacts, deferred IDs, unsummarized requirements, and near-token collisions cannot mask an active orphan."
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 125-2026-07-17T13-21-01
generated_at: 2026-07-17T15:30:00Z
duration: 11min
completed: 2026-07-17
---

# Phase 125 Plan 01: Active-Milestone Verification-Orphan Checker Summary

**A deterministic Bun checker now exposes summary-complete active requirements that lack lifecycle-valid verification evidence while excluding genuinely pending, deferred, archived, and malformed ownership states.**

## Performance

- **Duration:** 11 min
- **Started:** 2026-07-17T15:19:00Z
- **Completed:** 2026-07-17T15:30:00Z
- **Tasks:** 2
- **Files modified:** 3

## Accomplishments

- Derived the active phase corpus, active requirements, and exact-once owners from `ROADMAP.md` and `REQUIREMENTS.md` without a v2.1-specific ID manifest.
- Required passed lifecycle provenance before verification text can cover an activated requirement, with exact token boundaries that reject `RCN-040` for `RCN-04`.
- Added 18 focused Bun tests covering independent orphans, exclusions, ownership defects, malformed metadata, invalid-lifecycle masking, inline/block summary lists, and the real repository's pre/post-Phase-125 stages.
- Refreshed and verified the intentionally tracked worktree LOC report.

## Task Commits

Each TDD task was committed atomically:

1. **Task 1 RED: complete active-corpus fixture** - `d994bac6` (test)
2. **Task 1 GREEN: active verification traceability checker** - `7d877490` (feat)
3. **Task 2 RED: activation, exclusion, ownership, and lifecycle mutations** - `1ec2aa80` (test)
4. **Task 2 GREEN: valid-ownership activation gate and LOC refresh** - `863e671b` (fix)

## Files Created/Modified

- `scripts/check-active-milestone-verification-traceability.ts` - Reusable active-milestone parser, lifecycle validator, coverage decision core, filesystem shell, and CLI.
- `scripts/check-active-milestone-verification-traceability.test.ts` - Temporary-repository fixture matrix and strict staged real-repository smoke test.
- `docs/metrics/lines-of-code.md` - Regenerated worktree LOC report after both TypeScript files reached their final state.

## Decisions Made

- Invalid or duplicate ownership fails as an ownership defect but does not activate a second orphan; only exactly-once active ownership enters the orphan gate.
- Invalid verification lifecycle metadata fails independently and cannot suppress the uncovered requirement diagnostic.
- Empty future phase directories are allowed because roadmap planning may precede artifacts; only discovered summary or verification artifacts require a valid phase CONTEXT.

## Deviations from Plan

None - plan functionality and file scope were executed as written.

## Issues Encountered

- The first normal-hook RED commit attempt ran the repository verifier and failed on the pre-existing transient message `P124 gap-closure Phase 125 plans missing **Plans:** 0 plans`. The shared uncommitted ROADMAP had already enumerated four Phase 125 plans, while Plan 03 owns making the Phase 124 checker stage-aware.
- The Phase 125 orchestrator classified that mismatch as an expected transient stage and explicitly authorized `git commit --no-verify` for this plan's intermediate atomic commits only. Every such commit was still preceded by its applicable RED/GREEN test evidence and `git diff --check`; the complete repository verifier remains the orchestrator's gate after Plans 03 and 04 close the transient stage.

## Known Stubs

None. Empty collections and nullable parser results are deliberate typed outcomes, not unwired data sources or placeholder behavior.

## User Setup Required

None - the checker is filesystem-local and requires no credentials, services, network access, or environment configuration.

## Next Phase Readiness

- Plan 04 can add lifecycle-valid Phase 125 verification evidence; the staged real-repository smoke test will then require zero failures automatically.
- Plans 03 and 04 must reconcile the transient Phase 124 verifier stage before the orchestrator's final repository-wide verification and strict finalization.

## Self-Check: PASSED

- All three created/modified implementation artifacts and this summary exist.
- All four TDD task commits resolve in repository history.
- Lifecycle metadata matches `125-2026-07-17T13-21-01`, with `requirements-completed: []`.
- The 18-test fixture suite, LOC freshness check, diff check, Phase 115 immutability check, and no-Rust-change check passed.
