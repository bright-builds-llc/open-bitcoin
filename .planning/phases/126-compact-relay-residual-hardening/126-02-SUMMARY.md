---
phase: 126-compact-relay-residual-hardening
plan: "02"
subsystem: verification
tags: [bip152, parity, mutation-testing, lifecycle, bun]
requires:
  - phase: 126-01
    provides: fail-closed compact receive routing and lazy fallible announcement entropy
  - phase: 124
    provides: milestone closeout reconciliation and archive-stage validation
provides:
  - exact Knots anchors for live compact receive candidates and randomized announcement nonces
  - deterministic mutation-tested runtime and parity regression guard
  - four explicit Phase 126 closeout lifecycle states
affects: [126-03, 126-04, phase-124-closeout, milestone-audit]
tech-stack:
  added: []
  patterns: [fixed-corpus structural guard, one-concern mutations, explicit lifecycle state machine]
key-files:
  created:
    - scripts/check-phase126-compact-relay-residual-hardening.ts
    - scripts/check-phase126-compact-relay-residual-hardening.test.ts
  modified:
    - docs/parity/index.json
    - docs/parity/source-breadcrumbs.json
    - docs/parity/catalog/p2p.md
    - docs/parity/catalog/mempool-policy.md
    - scripts/check-phase124-milestone-gap-closure.ts
    - scripts/check-phase124-milestone-gap-closure.test.ts
    - scripts/check-phase124-milestone-closeout-reconciliation.fixtures.ts
    - scripts/verify.sh
    - docs/metrics/lines-of-code.md
key-decisions:
  - "Guard compact receive facts, nonce entropy, achieved-effect evidence, dependency agreement, and parity roots with a fixed local corpus."
  - "Run the Phase 126 guard after Phase 124 plus active traceability and before the unchanged Phase 117 final no-claim gate."
  - "Model candidate, verified pre-promotion, promoted pre-summary, and archive-ready as the only legal Phase 126 closeout states."
requirements-completed: []
duration: 21m
completed: 2026-07-18
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 126-2026-07-18T16-09-20
generated_at: 2026-07-18T20:27:00Z
---

# Phase 126 Plan 02: Compact Relay Audit and Closeout Guard Summary

Exact Knots parity roots, a local mutation-tested compact-relay guard, and a four-state closeout model now protect the hardened runtime without promoting any milestone requirement.

## Performance

- **Duration:** 21m
- **Started:** 2026-07-18T20:06:08Z
- **Completed:** 2026-07-18T20:27:00Z
- **Tasks:** 3
- **Files modified:** 11

## Accomplishments

- Anchored compact receive candidate supply to Knots mempool and recent-extra handling and anchored compact announcement nonces to `FastRandomContext().rand64()` and `CBlockHeaderAndShortTxIDs`.
- Added a filesystem-local Phase 126 guard with 13 passing mutation cases covering factless dispatch, explicit live facts, lazy OS entropy, safe entropy failure, achieved-effect evidence, dependency agreement, exact parity roots, and deterministic verifier order.
- Extended the Phase 124 compatibility layer with candidate, verified-pre-promotion, promoted-pre-summary, and archive-ready Phase 126 states plus one-concern rejection cases.
- Preserved the truthful candidate projection: all six Phase 126 requirements remain pending and the milestone audit remains `gaps_found`.

## Task Commits

| Task | Commit | Result |
| --- | --- | --- |
| 1. Refresh compact relay parity anchors and breadcrumbs | `256f3acd` | Added exact receive-candidate, nonce, runtime-evidence, and no-claim anchors |
| 2. Add deterministic compact relay residual hardening guard (RED) | `2aaad76b` | Added the initially failing mutation suite |
| 2. Add deterministic compact relay residual hardening guard (GREEN) | `ccf718af` | Implemented the local checker and ordered verifier wiring |
| 3. Encode the four legal Phase 126 closeout states | `22c4a785` | Added legal-state fixtures and mixed-state rejection coverage |

## Files Created/Modified

- `docs/parity/index.json` - Records exact Knots sources, tests, runtime evidence, and bounded rationale for both compact-relay surfaces.
- `docs/parity/source-breadcrumbs.json` - Keeps the modified compact receive seams tied to exact Knots source roots.
- `docs/parity/catalog/p2p.md` - Documents the Phase 126 receive and randomized-nonce candidate anchors without broader relay claims.
- `docs/parity/catalog/mempool-policy.md` - Documents live mempool and bounded recent-extra candidate supply.
- `scripts/check-phase126-compact-relay-residual-hardening.ts` - Enforces the fixed runtime, dependency, parity, and verifier corpus locally.
- `scripts/check-phase126-compact-relay-residual-hardening.test.ts` - Mutates each promised boundary and asserts stable Phase 126 diagnostics.
- `scripts/check-phase124-milestone-gap-closure.ts` - Recognizes exactly four coherent Phase 126 closeout states.
- `scripts/check-phase124-milestone-gap-closure.test.ts` - Covers legal states and rejects mixed lifecycle projections.
- `scripts/check-phase124-milestone-closeout-reconciliation.fixtures.ts` - Builds the Phase 126 candidate-through-archive fixture corpus.
- `scripts/verify.sh` - Runs the Phase 126 test and live check after active traceability and before Phase 117.
- `docs/metrics/lines-of-code.md` - Retains generated LOC freshness after the closeout checker expansion.

## Verification

| Check | Result |
| --- | --- |
| Phase 126 mutation suite | 13 passed, 0 failed |
| Phase 126 live checker | Passed |
| Phase 124 gap-closure suite | 41 passed, 0 failed |
| Phase 124 closeout reconciliation suite | 65 passed, 0 failed |
| Phase 124 live closeout checker | Passed in candidate state |
| Active milestone traceability suite and live checker | 21 passed, 0 failed; live check passed |
| Phase 117 final no-claim suite and live checker | 23 passed, 0 failed; live check passed |
| Parity breadcrumb checker | 383 Rust files verified |
| Phase lifecycle validation | Valid |
| Workspace formatting | Passed |
| Workspace Clippy, all targets/features | Passed with warnings denied |
| Workspace build, all targets/features | Passed |
| Workspace tests, all features | Passed |
| Bash syntax, file-length policy, and diff checks | Passed |

## Decisions Made

- The Phase 126 checker reads only a fixed repository corpus and scans itself for subprocess or network tokens, keeping default verification deterministic and public-network-free.
- Runtime truth is guarded at the adapter seams: generic dispatch must fail closed, while both managed receive paths must route through explicit live snapshot construction.
- Compact success provenance and achieved-effect evidence remain coupled to the actual emitted message rather than the selected policy action.
- Phase 117 remains the final phase checker and final no-claim gate.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Executed Task 3 early to unblock Plan 01 metadata**

- **Found during:** Phase 126 Plan 01 metadata closeout
- **Issue:** The existing Phase 124 compatibility layer could not accept the new Phase 126 candidate projection needed for coherent planning metadata.
- **Fix:** Executed the planned four-state closeout compatibility task early, then resumed Plan 02 Tasks 1 and 2 without changing Task 3.
- **Files modified:** `scripts/check-phase124-milestone-gap-closure.ts`, `scripts/check-phase124-milestone-gap-closure.test.ts`, `scripts/check-phase124-milestone-closeout-reconciliation.fixtures.ts`, `docs/metrics/lines-of-code.md`
- **Verification:** All 41 gap-closure mutations and all 65 closeout reconciliation cases pass; the live repository passes as a candidate.
- **Committed in:** `22c4a785`

**Total deviations:** 1 auto-fixed blocking issue.

**Impact on plan:** Task order changed only to unblock truthful metadata; scope and final behavior are unchanged.

## Issues Encountered

- The Phase 117 claim guard caught an initially over-broad catalog phrase during Task 1. The wording was narrowed to an explicit no-claim boundary before the task commit.
- The first parity mutation targeted an unrelated earlier JSON occurrence. The fixture was corrected to mutate the named compact-relay surface directly before the GREEN commit.

## Authentication Gates

None.

## Known Stubs

None. Empty arrays and nullable values in the checker corpus are test harness state or explicit parser outcomes, not unwired production data.

## Threat Flags

None. The new checker performs bounded local file reads only and introduces no endpoint, authentication path, schema change, or new runtime filesystem trust boundary.

## Next Phase Readiness

- Plan 126-03 can run the repository verification contract with the Phase 126 guard in its final ordered position.
- Plan 126-04 can promote requirements only after lifecycle-valid verification and a refreshed passing audit.
- No active blocker is recorded; six Phase 126 requirements intentionally remain pending.

## Self-Check: PASSED

- All created and modified key files exist.
- Task commits `256f3acd`, `2aaad76b`, `ccf718af`, and `22c4a785` exist in repository history.
- The summary has exactly two standalone YAML frontmatter delimiters.
- No known stub prevents the plan objective.
