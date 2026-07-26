---
phase: 132-typed-package-vocabulary-and-staged-admission
plan: "06"
subsystem: mempool
tags: [rust, mempool, package-rbf, replacement-policy, feerate-diagram, prospective-delta, lifecycle, parity]
requires:
  - phase: 132-typed-package-vocabulary-and-staged-admission
    plan: "03"
    provides: Sparse prospective mempool composition and typed removal facts
  - phase: 132-typed-package-vocabulary-and-staged-admission
    plan: "04"
    provides: Individual-first package evaluation and residual retry
  - phase: 132-typed-package-vocabulary-and-staged-admission
    plan: "05"
    provides: Checked aggregate fee groups, final trimming, and authoritative lifecycle assembly
provides:
  - Exact bounded two-member one-parent/one-child package replacement policy
  - Conservative pre-union conflict-descendant counting with a hard 100-candidate limit
  - Checked incremental fee, strict parent/package feerate, and feerate-diagram dominance rules
  - One atomic prospective transition for replacement removals and accepted additions
  - Truthful direct/descendant replacement lifecycle facts with mutation-free failure paths
affects: [phase-132-ephemeral-policy, mempool, package-relay, replacement-lifecycle]
tech-stack:
  added: []
  patterns:
    - Bound conflict graph work before descendant-union or diagram allocation
    - Prepare residual children through a read-only candidate overlay before any mempool mutation
    - Compose replacement removals and package additions in one checked sparse sub-delta
key-files:
  created:
    - packages/open-bitcoin-mempool/src/policy/replacement.rs
    - packages/open-bitcoin-mempool/src/policy/replacement/diagram.rs
    - packages/open-bitcoin-mempool/src/pool/package_admission/residual.rs
  modified:
    - packages/open-bitcoin-mempool/src/package/report.rs
    - packages/open-bitcoin-mempool/src/policy.rs
    - packages/open-bitcoin-mempool/src/pool/package_admission.rs
    - packages/open-bitcoin-mempool/src/pool/prospective.rs
    - packages/open-bitcoin-mempool/src/pool/tests/package_policy_cases.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - "A singleton direct conflict is reconsiderable only so the exact residual package can apply the bounded package-RBF policy; arbitrary singleton or larger-package replacement remains rejected."
  - "Each direct conflict contributes its full cached descendant count to the conservative running bound, including overlap, before any removal union or feerate diagram is allocated."
  - "Residual candidates are prepared against a read-only parent-aware view, then replacement removals and both additions are composed exactly once before post-removal limit and script checks."
  - "Transient replacement candidates removed by final trim never become lifecycle admissions or pressure removals; only base members receive committed replacement-removal facts."
patterns-established:
  - "Package replacement economics are pure data-in/data-out policy over a narrow mempool view."
  - "Late residual failure rewrites the whole fee group consistently and drops the owned prospective transition."
requirements-addressed: [PACK-07]
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 132-2026-07-25T18-13-00
generated_at: 2026-07-26T07:40:21Z
duration: 52m
completed: 2026-07-26
---

# Phase 132 Plan 06: Bounded Package Replacement Summary

**Exact limited 1P1C package replacement now enforces a pre-allocation 100-candidate bound, checked incremental and diagram economics, and one atomic removal/addition lifecycle transition.**

## Performance

- **Duration:** 52m
- **Started:** 2026-07-26T06:48:24Z
- **Completed:** 2026-07-26T07:40:21Z
- **Tasks:** 2
- **Files modified:** 15

## Accomplishments

- Added pure typed package-RBF policy for the exact two-member parent-child residual shape, including no-new-mempool-ancestor eligibility.
- Checked the conservative sum of every direct conflict's cached descendant count before allocating the deterministic deduplicated removal union or feerate diagram.
- Enforced checked original-plus-incremental fees, package feerate strictly above the new parent, and strict cumulative feerate-diagram improvement.
- Integrated replacement after aggregate fee assessment through a read-only residual preparation view and one checked prospective sub-delta containing all removals and additions.
- Added public-path parity fixtures for dry-run/submit equality, direct and descendant lifecycle roles, insufficient-fee rollback, late script rollback, composition failure injection, and final-trim truthfulness.

## Task Commits

Each task was committed atomically:

1. **Task 1: Implement bounded limited package-RBF policy and diagram dominance** - `d31c3079`
2. **Task 2: Stage replacement removals and additions as one coherent package transition** - `d348c9ee`

## Files Created/Modified

- `packages/open-bitcoin-mempool/src/policy/replacement.rs` - Exact eligibility, conservative conflict bound, removal union, incremental fee, and strict package-parent rate policy.
- `packages/open-bitcoin-mempool/src/policy/replacement/diagram.rs` - Checked old/new cumulative feerate curves and strict dominance comparison.
- `packages/open-bitcoin-mempool/src/policy/replacement/tests.rs` - Typed topology, bound, overlap, arithmetic, fee, rate, and diagram policy regressions.
- `packages/open-bitcoin-mempool/src/policy/replacement/diagram/tests.rs` - Curve construction, ordering, partial-prefix, lookup, and overflow coverage.
- `packages/open-bitcoin-mempool/src/pool/package_admission/residual.rs` - Read-only residual preparation, replacement decision mapping, coherent sub-delta composition, and post-removal checks.
- `packages/open-bitcoin-mempool/src/pool/package_admission.rs` - Singleton conflict deferral into exact residual replacement evaluation.
- `packages/open-bitcoin-mempool/src/pool/prospective.rs` - Checked transition constructor combining entry upserts and typed removals.
- `packages/open-bitcoin-mempool/src/package/report.rs` - Typed reconsiderable and hard replacement result vocabulary.
- `packages/open-bitcoin-mempool/src/pool/tests/package_policy_cases.rs` - End-to-end package-RBF lifecycle, rollback, dry-run, composition, and trim fixtures.
- `docs/parity/source-breadcrumbs.json` - Registered policy, diagram, tests, and residual orchestration against pinned Knots sources.
- `docs/metrics/lines-of-code.md` - Refreshed tracked source metrics through normal hooks.

## Decisions Made

- Package conflicts do not mutate or stage through the singleton path; they defer to residual evaluation so only the exact bounded exception can replace live entries.
- The child is prepared against an in-memory map of already prepared package entries rather than a staged mempool edit, preserving the no-mutation boundary before policy approval.
- Spent-outpoint index entries are validated against visible members before fee assessment so corrupted prospective facts remain typed internal invariants rather than policy rejections.
- Limit and script failures reclassify the whole residual group and remove all stale individual fee groups, keeping report invariants and base state coherent.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Completed workspace coverage for the pure policy boundary**

- **Found during:** Task 1 normal-hook verification
- **Issue:** The first hook run identified uncovered typed diagnostic, arithmetic, diagram, and lookup branches.
- **Fix:** Added focused unit fixtures for every checked replacement and diagram branch before retrying the atomic Task 1 commit.
- **Files modified:** `packages/open-bitcoin-mempool/src/policy/replacement/tests.rs`, `packages/open-bitcoin-mempool/src/policy/replacement/diagram/tests.rs`, `packages/open-bitcoin-mempool/src/package/tests.rs`
- **Verification:** The second normal hook passed the complete workspace coverage gate.
- **Committed in:** `d31c3079`

**2. [Rule 1 - Bug] Reclassified the complete residual after late script or limit failure**

- **Found during:** Task 2 late-script rollback test
- **Issue:** Reclassifying only the failing child left a singleton fee group with no eligible report member, causing report construction to fail with an internal invariant.
- **Fix:** Late limit and script failures now hard-reject the entire residual and remove superseded individual fee groups before final report construction.
- **Files modified:** `packages/open-bitcoin-mempool/src/pool/package_admission/residual.rs`, `packages/open-bitcoin-mempool/src/pool/tests/package_policy_cases.rs`
- **Verification:** The injected late-script rollback returns a typed report with an empty delta and an unchanged complete mempool snapshot.
- **Committed in:** `d348c9ee`

**3. [Rule 1 - Bug] Preserved corrupted spent-index invariant detection**

- **Found during:** Task 2 full mempool regression
- **Issue:** A stale spent-outpoint index could be mistaken for a package conflict and reclassified as a bounded replacement policy failure.
- **Fix:** Conflict discovery now verifies every prospective spender resolves to a visible entry before aggregate fee or replacement decisions.
- **Files modified:** `packages/open-bitcoin-mempool/src/pool/package_admission/residual.rs`
- **Verification:** All four existing overlay-corruption regressions again return internal invariants without mutation; the full mempool suite passes.
- **Committed in:** `d348c9ee`

**4. [Rule 2 - Missing Critical] Extracted residual orchestration for repository code-shape compliance**

- **Found during:** Task 2 simplification and file-length pass
- **Issue:** Inline replacement orchestration raised `package_admission.rs` beyond the enforced 628-line production limit, and the transition constructor moved `prospective.rs` to the boundary.
- **Fix:** Extracted cohesive residual evaluation into a breadcrumbed child module and compacted the prospective constructor without weakening checks.
- **Files modified:** `packages/open-bitcoin-mempool/src/pool/package_admission.rs`, `packages/open-bitcoin-mempool/src/pool/package_admission/residual.rs`, `packages/open-bitcoin-mempool/src/pool/prospective.rs`, `docs/parity/source-breadcrumbs.json`
- **Verification:** The normal hook passed the production file-length gate for all 318 checked Rust files.
- **Committed in:** `d348c9ee`

**Total deviations:** 4 auto-fixed (2 bugs, 1 missing-critical code-shape fix, 1 blocking coverage fix)

**Impact on plan:** All fixes strengthen atomicity, invariant preservation, coverage, and repository compliance without broadening replacement beyond the planned limited 1P1C exception.

### Test Boundary Note

- Task 1 kept direct pure-policy and diagram fixtures beside their internal modules so conservative pre-allocation behavior could be observed without prematurely wiring production admission.
- Task 2 added the requested public-path `package_rbf` matrix in `package_policy_cases.rs` together with production integration. No Task 2 production integration entered the Task 1 commit.

## Issues Encountered

- The first Task 1 normal hook stopped on coverage only; focused branch fixtures closed the gaps and the retry passed.
- The first Task 2 full mempool run exposed stale-index classification drift; validating spender identities restored the prior invariant contract.
- The composition failure path is unreachable from a valid opaque package by construction, so a scoped test-only duplicate-entry injection proves the checked sub-delta fails before live mutation.

## Verification

- `phase132-package-rbf-policy`: pure replacement policy and diagram tests passed.
- `phase132-package-rbf-integration`: all 5 end-to-end `package_rbf` fixtures passed.
- `phase132-rbf-single-regression`: existing single replacement and parity regressions passed.
- Complete mempool verification passed with 293 unit tests, 5 parity tests, and 5 compile-fail doctests.
- Required pre-commit sequence passed: format, Clippy with warnings denied, all-target/all-feature build, and full workspace tests.
- Parity breadcrumbs verified for 434 Rust files.
- Production file-length verification passed for all 318 checked Rust files.
- Both normal hooks passed the complete repository verifier, Bazel smoke builds, and workspace coverage.
- Task 1 normal hook completed in 3m 5.796s; Task 2 normal hook completed in 3m 26.531s.

## Authentication Gates

None.

## Known Stubs

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan 07 can add ephemeral package policy after the now-explicit replacement stage without changing the coherent prospective transition boundary.
- The package engine now has checked static/rolling, limits, replacement, scripts, trim, report, and lifecycle seams with no remaining Plan 06 blockers.

## Self-Check: PASSED

- Summary file exists with the exact Plan 06 generator, lifecycle, and requirements metadata.
- Task commits `d31c3079` and `d348c9ee` exist in repository history.
- All created policy, diagram, test, and residual source files exist and are parity-registered.
- Summary diff is whitespace-clean, and parent-owned `STATE.md` and `ROADMAP.md` remain unstaged.
