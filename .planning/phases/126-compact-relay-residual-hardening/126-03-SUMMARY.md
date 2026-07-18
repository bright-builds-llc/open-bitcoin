---
phase: 126-compact-relay-residual-hardening
plan: "03"
subsystem: verification
tags: [bip152, lifecycle, candidate-evidence, bun, rust]
requires:
  - phase: 126-02
    provides: compact relay runtime/parity checker and four-state closeout guard
provides:
  - executor-owned candidate command evidence
  - full repository verification against the six-pending projection
  - explicit independent-verifier provenance boundary
affects: [126-verification, 126-04, milestone-audit]
tech-stack:
  added: []
  patterns: [candidate evidence before independent verification, generator-only LOC refresh]
key-files:
  created:
    - .planning/phases/126-compact-relay-residual-hardening/126-PRE-PROMOTION-EVIDENCE.md
  modified:
    - docs/metrics/lines-of-code.md
    - docs/parity/catalog/mempool-policy.md
key-decisions:
  - "Keep executor candidate evidence distinct from independent gsd-verifier ownership."
  - "Retain all six Phase 126 requirements pending until the verifier revision gate passes."
patterns-established:
  - "Candidate proof records exact rerun history without promoting lifecycle metadata."
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 126-2026-07-18T16-09-20
generated_at: 2026-07-18T20:40:43Z
duration: 7m
completed: 2026-07-18
---

# Phase 126 Plan 03: Pre-Promotion Candidate Evidence Summary

Executor-owned command evidence now proves the compact-relay candidate against
the full repository contract while preserving the independent verifier gate
and all six pending requirements.

## Performance

- **Duration:** 7m
- **Started:** 2026-07-18T20:33:53Z
- **Completed:** 2026-07-18T20:40:43Z
- **Tasks:** 1
- **Files modified:** 3

## Accomplishments

- Passed the Phase 126 mutation/live checker, Phase 124 candidate-state checker,
  active-milestone traceability checker, parity breadcrumb checker, and
  require-plans lifecycle gate.
- Refreshed `docs/metrics/lines-of-code.md` only through the repository-owned
  generator after the verifier requested it.
- Passed the full `bash scripts/verify.sh` contract in 3m 19.503s after
  correcting one committed no-claim wording conflict.
- Created `126-PRE-PROMOTION-EVIDENCE.md` with truthful executor attribution,
  exact lifecycle identity, `candidate_passed`, exact command results, and an
  explicit independent-verifier disclaimer.
- Preserved the candidate projection at `33/39` requirements, `16/17` phases,
  `gaps_found`, and `/gsd-execute-phase 126`.

## Task Commits

| Task | Commit | Result |
| --- | --- | --- |
| Blocking verification fix | `74f826e3` | Restored checker-recognized no-claim grammar for package relay |
| 1. Prove the six-pending candidate corpus | `db41fd50` | Recorded candidate evidence and refreshed generated LOC |

## Files Created/Modified

- `.planning/phases/126-compact-relay-residual-hardening/126-PRE-PROMOTION-EVIDENCE.md`
  - Records exact candidate command results and the independent-verifier
    provenance boundary.
- `docs/metrics/lines-of-code.md` - Generator-refreshed report covering 560
  files and 236,187 lines.
- `docs/parity/catalog/mempool-policy.md` - Uses the established `does not add`
  no-claim grammar recognized by the Phase 103 guard.

## Verification

| Check | Result |
| --- | --- |
| Phase 126 mutation suite | 13 passed, 0 failed |
| Phase 126 live checker | Passed |
| Phase 124 candidate-state checker | Passed |
| Active milestone traceability checker | Passed |
| Parity breadcrumb checker | 383 Rust files verified |
| Lifecycle require-plans gate | Valid |
| Phase 103 mutation suite after wording fix | 8 passed, 0 failed |
| Phase 103 live checker | Passed |
| Full repository verifier | Passed in 3m 19.503s |
| Final focused candidate rerun | Passed |
| Diff checks | Passed |

The full verifier covered deterministic checker suites, pure-core dependency
and panic-site guards, file-length policy, Cargo formatting, Clippy with
warnings denied, all-target/all-feature build and tests, benchmark smoke,
Bazel smoke/provenance, and pure-core coverage.

## Decisions Made

- The pre-promotion artifact is executor evidence only. It is input to—not a
  substitute for—the fresh `gsd-verifier`.
- `126-VERIFICATION.md` remains exclusively owned by `gsd-verifier`; this plan
  neither created nor edited it.
- `CMP-05`, `RCN-02`, `RCN-03`, `GOV-04`, `BOUND-01`, and `HARD-05` remain
  unchecked/Pending until Plan 04 promotion is authorized by passed,
  lifecycle-valid independent verification.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Corrected no-claim grammar rejected by the Phase 103 guard**

- **Found during:** Task 1 full repository verification
- **Issue:** The committed Plan 02 sentence `This candidate adds no package
  relay` truthfully denied the feature but did not use a marker recognized by
  the established Phase 103 no-claim parser, causing two mutation cases to
  fail.
- **Fix:** Changed the sentence to `This candidate does not add package relay`
  without altering the scoped claim.
- **Files modified:** `docs/parity/catalog/mempool-policy.md`
- **Verification:** Phase 103 mutation suite passed 8/8, its live checker
  passed, and the full repository verifier passed.
- **Committed in:** `74f826e3`

**2. [Rule 1 - Bug] Corrected legacy state counter and Roadmap row output**

- **Found during:** Plan metadata update
- **Issue:** The state progress helper counted two legacy aggregate summaries as
  plans and produced an impossible `59/58`; the Roadmap helper also removed the
  Phase 126 date and table spacing, while the metric helper failed to recognize
  the existing Performance Metrics section.
- **Fix:** Reconciled active-milestone progress to `57/58` (98%), restored the
  Phase 126 `3/4` In Progress row and date, and appended the Plan 03 metric.
- **Files modified:** `.planning/STATE.md`, `.planning/ROADMAP.md`
- **Verification:** Phase 124 candidate-state and active-milestone traceability
  checkers passed against the corrected metadata.
- **Committed in:** Plan metadata commit

**Total deviations:** 2 auto-fixed bugs.

**Impact on plan:** The fix restored the existing deterministic no-claim
contract without expanding scope or changing candidate metadata.

## Issues Encountered

- The first full-verifier attempt reported the tracked LOC report as stale.
  The prescribed generator refreshed it to 236,187 lines.
- The next attempt exposed the Phase 103 no-claim grammar bug documented above.
  After the focused fix, the complete verifier passed.

## Authentication Gates

None.

## Known Stubs

None.

## Threat Flags

None. This plan added only planning evidence and a generated metrics report;
it introduced no endpoint, authentication path, schema change, or runtime
filesystem trust boundary.

## Next Phase Readiness

- A fresh `gsd-verifier` must now independently inspect Phase 126 and
  exclusively create lifecycle-valid `126-VERIFICATION.md`.
- Plan 126-04 remains blocked until verification status is `passed` and
  `verify lifecycle 126 --require-plans --require-verification --raw` returns
  `valid`.
- All six requirements intentionally remain pending and no archive promotion
  has occurred.

## Self-Check: PASSED

- The pre-promotion evidence and Plan 03 summary exist.
- Task commits `74f826e3` and `db41fd50` exist in repository history.
- Both artifacts carry the exact Plan 126 lifecycle identity and only the
  expected YAML frontmatter delimiters.
- `requirements-completed` is empty and no `126-VERIFICATION.md` was created.
- No known stub prevents the plan objective.
