---
phase: 134-authoritative-cross-cache-lifecycle-integration
plan: "22"
subsystem: testing
tags: [typescript, lexical-analysis, call-graph, mutation-testing, apply-boundary]

# Dependency graph
requires:
  - phase: 134-17
    provides: Snapshot complete-or-abort lifecycle handling
  - phase: 134-19
    provides: Bounded accepted-package and retirement-aware lifecycle work
  - phase: 134-21
    provides: Atomic mempool commit migration and removal of validated compatibility APIs
provides:
  - Fail-closed transitive analysis for protected Phase 134 apply boundaries
  - Exact atomic-core, infallible-apply, and pure-call symbol classifications
  - Independent helper-indirection, cycle, ordering, and removed-API mutation coverage
affects: [134-23, 134-24, phase-134-verification, MPLIFE]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - Deterministic exact-symbol call-graph traversal with fail-closed classification
    - Visited fully qualified symbols terminate helper cycles without hiding violations

key-files:
  created:
    - scripts/check-phase134-authoritative-lifecycle.test/apply-helpers.ts
  modified:
    - scripts/check-phase134-apply-boundaries.ts
    - scripts/check-phase134-authoritative-lifecycle.test.ts
    - docs/metrics/lines-of-code.md

key-decisions:
  - "Treat ManagedPeerNetwork::commit_sealed_lifecycle and ManagedPeerNetwork::apply_prepared_lifecycle as the aggregate root because Plan 21 placed the fallible atomic core commit one level above the infallible dependent reducer."
  - "Classify callable behavior only by exact fully qualified symbol; unresolved or unknown repo-owned calls fail closed."
  - "Track visited fully qualified symbols so recursive helper cycles terminate deterministically while each reachable body remains inspected."
  - "Keep MPLIFE-01 through MPLIFE-04 pending until phase re-verification."

patterns-established:
  - "Transitive apply guards: every reachable repo-owned helper is inspected or rejected as unclassified."
  - "Atomic core ordering: the sole fallible commit occurs exactly once before any dependent target mutation."

requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 134-2026-07-28T01-41-12
generated_at: 2026-07-29T20:11:49Z

# Metrics
duration: 27m
completed: 2026-07-29
---

# Phase 134 Plan 22: Transitive Apply Boundary Summary

**A deterministic exact-symbol call graph now proves protected lifecycle apply paths transitively, with one ordered atomic mempool exception and mutation tests that expose hidden helper bypasses.**

## Performance

- **Duration:** 27m
- **Started:** 2026-07-29T19:44:27Z
- **Completed:** 2026-07-29T20:11:49Z
- **Tasks:** 1
- **Files modified:** 4

## Accomplishments

- Replaced the lexical-only apply check with a deterministic call graph spanning the node targets and reachable mempool, peer lifecycle, scheduler, and orphanage sources.
- Added explicit `ATOMIC_CORE_COMMIT`, `INFALLIBLE_APPLY_CALLEES`, and `PURE_CALL_ALLOWLIST` exact-symbol sets, with fail-closed handling for unknown helpers and unresolved overloads.
- Proved the atomic mempool core commit occurs exactly once before dependent mutations and rejected every removed validated lifecycle API.
- Added ten helper-focused cases covering direct and nested hidden I/O, indirect mutation, unknown calls, removed APIs, ordering, duplicate commit, recursive cycles, and live positive controls.

## Task Commits

Each task was committed atomically:

1. **Task 1: Make apply-boundary checking transitive and fail closed** - `ee2b6a14` (test)

The TDD RED signal was captured before implementation: all eight negative helper mutations failed while the existing and control cases stayed green. Repository policy requires green pre-commit gates, so RED and GREEN were retained in the single atomic task commit.

## Files Created/Modified

- `scripts/check-phase134-apply-boundaries.ts` - Traverses exact-symbol repo call graphs, enforces explicit classifications, and validates atomic core ordering.
- `scripts/check-phase134-authoritative-lifecycle.test.ts` - Integrates the split helper-focused mutation suite with the existing lifecycle guard tests.
- `scripts/check-phase134-authoritative-lifecycle.test/apply-helpers.ts` - Owns independent indirect-helper mutations and positive controls.
- `docs/metrics/lines-of-code.md` - Tracked LOC snapshot refreshed by the normal repository hook.

## Decisions Made

- The production aggregate root is the pair `ManagedPeerNetwork::commit_sealed_lifecycle` and `ManagedPeerNetwork::apply_prepared_lifecycle`: Plan 21 intentionally placed the revision-checking atomic commit in the first method and kept the seven dependent applies in the second.
- Exact fully qualified symbols are the only classification keys. Suffix, receiver-free, and module-prefix allowances remain prohibited.
- A visited set keyed by fully qualified symbol terminates recursive cycles; visiting a cycle does not suppress inspection or a violation found in any reachable body.
- MPLIFE-01 through MPLIFE-04 remain pending until Plan 24 phase re-verification.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Matched the checker root to the Plan 21 production split**

- **Found during:** Task 1 (Make apply-boundary checking transitive and fail closed)
- **Issue:** The plan described the atomic commit as occurring inside `apply_prepared_lifecycle`, but Plan 21 moved it into `commit_sealed_lifecycle` immediately above the infallible reducer.
- **Fix:** Analyze both functions as one explicit aggregate root while retaining exact once-before-dependent-mutation ordering.
- **Files modified:** `scripts/check-phase134-apply-boundaries.ts`
- **Verification:** The live checker and legitimate core-first control pass; late and duplicate atomic-commit mutations fail.
- **Committed in:** `ee2b6a14`

**2. [Rule 3 - Blocking] Preserved Rust lifetime syntax during lexical masking**

- **Found during:** Task 1 (Make apply-boundary checking transitive and fail closed)
- **Issue:** Treating `'_` as the start of a character literal could unbalance source extraction and prevent deterministic traversal.
- **Fix:** Made masking lifetime-aware so Rust lifetime tokens remain structurally neutral without weakening string and character masking.
- **Files modified:** `scripts/check-phase134-apply-boundaries.ts`
- **Verification:** The full live source corpus extracts successfully and all 99 focused tests pass.
- **Committed in:** `ee2b6a14`

**Total deviations:** 2 auto-fixed blocking issues.

**Impact on plan:** Both changes were required to inspect the actual Plan 21 production topology correctly; no public runtime behavior or architecture changed.

## Issues Encountered

- The normal commit hook ran the full repository contract for 8m 21s. It completed successfully without interruption.

## Verification Evidence

- Ordered Rust gate: `cargo fmt --all` passed.
- Ordered Rust gate: `cargo clippy --all-targets --all-features -- -D warnings` passed.
- Ordered Rust gate: `cargo build --all-targets --all-features` passed.
- Ordered Rust gate: `cargo test --all-features` passed.
- Focused lifecycle suite: 99 passed, 0 failed, 200 assertions.
- Live apply-boundary checker passed and printed the exact protected classifications.
- `git diff --check` passed.
- `bun scripts/bright-builds-check.ts all` passed before the task commit.
- Normal repository hook: `bash scripts/verify.sh` passed in 8m 21s.

## Threat Model Closure

| Threat | Mitigation evidence |
| --- | --- |
| T-134-22-01 | Reachable repo-owned helpers are traversed by exact symbol; nested, unknown, cyclic, and hidden-I/O mutations fail. |
| T-134-22-02 | Only `Mempool::commit_prepared_mempool_transition` is atomic-core classified, exactly once before dependent mutation; removed APIs fail. |
| T-134-22-03 | Pure and infallible classifications use exact fully qualified symbols and unresolved calls fail closed. |
| T-134-22-04 | Helper mutations live in an independent fixture module with negative cases and live positive controls. |

No new network endpoint, authentication path, schema boundary, or runtime file-access surface was introduced.

## Known Stubs

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan 134-23 can guard the canonical claim surfaces with the stronger transitive apply proof available.
- MPLIFE-01 through MPLIFE-04 remain pending until the complete phase verification contract in Plan 134-24.

## Self-Check

PASSED:

- All three plan-owned source/test artifacts and this summary exist.
- Task commit `ee2b6a14` exists in repository history.
- `STATE.md` points to Plan 23 and `ROADMAP.md` reports 22/24 with Phase 134 still in progress.
- MPLIFE-01 through MPLIFE-04 remain unchecked and `Pending` in `REQUIREMENTS.md`.
- No source stub markers, whitespace errors, or untracked generated artifacts remain.

***

*Phase: 134-authoritative-cross-cache-lifecycle-integration*
*Completed: 2026-07-29*
