---
phase: 132-typed-package-vocabulary-and-staged-admission
plan: "05"
subsystem: mempool
tags: [rust, mempool, package-policy, fee-groups, truc, pressure-trim, lifecycle, parity]
requires:
  - phase: 132-typed-package-vocabulary-and-staged-admission
    plan: "01"
    provides: Checked opaque package reports and effective fee groups
  - phase: 132-typed-package-vocabulary-and-staged-admission
    plan: "02"
    provides: Pre-script candidate facts and guarded apply
  - phase: 132-typed-package-vocabulary-and-staged-admission
    plan: "03"
    provides: Sparse prospective view, pressure trimming, and lifecycle fact storage
  - phase: 132-typed-package-vocabulary-and-staged-admission
    plan: "04"
    provides: Individual-first package evaluation and residual retry
provides:
  - Checked request-ordered effective fee groups with distinct static and rolling obligations
  - Pinned Reject, Accept, and Enforce TRUC fee-policy modes
  - One post-script final pressure trim followed by authoritative result rewriting
  - Final-membership lifecycle deltas containing only real admitted and removed identities
affects: [phase-132-replacement-policy, phase-132-ephemeral-policy, mempool, package-relay]
tech-stack:
  added: []
  patterns:
    - Check ordinary member static floors before aggregate rolling-floor sponsorship
    - Keep incremental relay fee confined to replacement and pressure roles
    - Derive reports and lifecycle facts only after one final prospective capacity trim
key-files:
  created:
    - packages/open-bitcoin-mempool/src/pool/package_admission/finalization.rs
    - packages/open-bitcoin-mempool/src/pool/package_admission/test_support.rs
    - packages/open-bitcoin-mempool/src/pool/tests/package_policy_cases.rs
  modified:
    - packages/open-bitcoin-mempool/src/fee.rs
    - packages/open-bitcoin-mempool/src/types.rs
    - packages/open-bitcoin-mempool/src/pool/candidate.rs
    - packages/open-bitcoin-mempool/src/pool/package_admission.rs
    - packages/open-bitcoin-mempool/src/pool/prospective.rs
    - packages/open-bitcoin-mempool/src/pool/tests/package_admission_cases.rs
    - docs/parity/source-breadcrumbs.json
    - docs/metrics/lines-of-code.md
key-decisions:
  - "Every ordinary eligible member independently meets the static floor; only the checked aggregate modified fee may satisfy the rolling floor."
  - "TRUC defaults to Accept; Reject is hard non-standard, while Enforce version 3 bypasses only the static-floor check."
  - "A package transition is prepared whenever staged membership or rolling-fee state changed, even when final admitted membership is empty."
  - "Post-trim lifecycle removals are filtered to identities that actually existed in the immutable base, preventing requested witness aliases or transient candidates from becoming removal facts."
patterns-established:
  - "Effective fee groups retain checked base fee, modified fee, virtual size, effective rate, and exact request-ordered eligible wtxids."
  - "Finalization order: late scripts, one pressure trim, ordered report rewrite, lifecycle assembly, then guarded apply."
requirements-addressed: [PACK-03, PACK-06, PACK-07]
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 132-2026-07-25T18-13-00
generated_at: 2026-07-26T06:41:12Z
duration: 47m
completed: 2026-07-26
---

# Phase 132 Plan 05: Effective Fee and Final Membership Summary

**Package admission now enforces checked member-static and aggregate-rolling fee groups, applies one final capacity trim, and reports only authoritative final membership and lifecycle identities.**

## Performance

- **Duration:** 47m
- **Started:** 2026-07-26T05:54:39Z
- **Completed:** 2026-07-26T06:41:12Z
- **Tasks:** 2
- **Files modified:** 13

## Accomplishments

- Added checked `CandidateFees`, ordered `PackageFeeMember` assessment, and non-empty opaque effective fee groups whose base, modified, virtual-size, and effective-rate facts use checked arithmetic.
- Preserved exact fee-role separation: ordinary members meet static individually, the group meets rolling in aggregate, and incremental relay fee remains absent from admission-floor evaluation.
- Added the pinned three-state TRUC contract with default `Accept`, hard `Reject`, and an `Enforce` version-3 exception limited to the static check.
- Finalized successful package work with exactly one post-script capacity trim, then rewrote new, exact-existing, and witness-alias results from authoritative prospective membership.
- Built lifecycle admissions and removals from real final identities only, including pressure-removal facts for base members and no false alias, transient, or post-trim admission facts.

## Task Commits

Each task was committed atomically:

1. **Task 1: Generalize fee roles to non-empty ordered effective groups** - `98fe1195`
2. **Task 2: Trim once and rewrite ordered results from final prospective membership** - `ceaba3d0`

## Files Created/Modified

- `packages/open-bitcoin-mempool/src/fee.rs` - Checked candidate fee facts, package fee members, group assessment, TRUC/static decisions, and aggregate rolling evaluation.
- `packages/open-bitcoin-mempool/src/types.rs` - Three-state `TrucPolicy` and pinned `Accept` default.
- `packages/open-bitcoin-mempool/src/pool/candidate.rs` - Production base/modified candidate fee facts.
- `packages/open-bitcoin-mempool/src/pool/package_admission.rs` - Fee-group classification, late scripts, one final trim, rewrite delegation, and guarded patch selection.
- `packages/open-bitcoin-mempool/src/pool/package_admission/finalization.rs` - Post-trim report rewriting and lifecycle assembly from final real identities.
- `packages/open-bitcoin-mempool/src/pool/package_admission/test_support.rs` - Objective stage-order and trim probes.
- `packages/open-bitcoin-mempool/src/pool/prospective.rs` - Read-only removal/base/change facts needed by finalization.
- `packages/open-bitcoin-mempool/src/pool/tests/package_policy_cases.rs` - Fee boundaries, TRUC modes, stage ordering, pressure rewrite, alias identity, and dry-run/submit equivalence.
- `packages/open-bitcoin-mempool/src/pool/tests/package_admission_cases.rs` - Residual rolling/static fixture migration and exhaustive fee-group branch coverage.
- `docs/parity/source-breadcrumbs.json` - Registered package-policy and extracted finalization sources.
- `docs/metrics/lines-of-code.md` - Refreshed tracked source metrics through normal hooks.

## Decisions Made

- Static assessment uses each member's modified fee independently, preventing a high-fee sibling from sponsoring an ordinary below-static member.
- Rolling assessment uses the checked aggregate modified fee and aggregate virtual size; equality passes, while one satoshi below is reconsiderable.
- Version 3 in `Enforce` mode bypasses only static assessment and still participates in checked aggregate rolling assessment.
- Final capacity trimming runs only after at least one candidate completed late scripts; a script or pre-script failure cannot trim or apply that work.
- Patch preparation is based on prospective state change rather than admitted count so a new-only pressure eviction still commits its rolling-fee transition without inventing membership.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Migrated residual fee fixtures to separated static and rolling policy**

- **Found during:** Task 1 full regression
- **Issue:** Older residual fixtures relied on a combined admission floor, so the new independent static check hard-rejected cases intended to exercise rolling reconsideration.
- **Fix:** Used zero static plus positive rolling for the residual-rolling scenarios and changed the one genuine below-static expectation to `HardRejected`.
- **Files modified:** `packages/open-bitcoin-mempool/src/pool/tests/package_admission_cases.rs`
- **Verification:** All 26 package-admission regressions and the complete mempool suite passed.
- **Committed in:** `98fe1195`

**2. [Rule 1 - Bug] Removed stale singleton groups after residual hard rejection**

- **Found during:** Task 1 coverage completion
- **Issue:** Residual hard-result rewriting left individual reconsiderable fee groups behind, causing report validation to reject groups with no eligible members.
- **Fix:** The residual hard branch now removes the replaced singleton group facts before constructing the report.
- **Files modified:** `packages/open-bitcoin-mempool/src/pool/package_admission.rs`, `packages/open-bitcoin-mempool/src/pool/tests/package_admission_cases.rs`
- **Verification:** The injected residual hard-policy regression returns typed hard results without mutation and workspace coverage is complete.
- **Committed in:** `98fe1195`

**3. [Rule 3 - Blocking] Added exhaustive checked-error and classifier coverage**

- **Found during:** Task 1 normal pre-commit hook
- **Issue:** The required workspace coverage gate exposed unchecked diagnostic, empty-group, residual outcome, report-conversion, and legacy prospective-fee branches.
- **Fix:** Added focused checked-error diagnostics, residual outcomes, zero-vsize conversion, and prospective fee-guard regressions without weakening the coverage gate.
- **Files modified:** `packages/open-bitcoin-mempool/src/pool/package_admission.rs`, `packages/open-bitcoin-mempool/src/pool/tests/package_admission_cases.rs`, `packages/open-bitcoin-mempool/src/pool/tests/package_policy_cases.rs`
- **Verification:** Both Task 1 and Task 2 normal hooks completed with no uncovered workspace lines.
- **Committed in:** `98fe1195`

**4. [Rule 2 - Missing Critical] Extracted finalization modules to satisfy repository code-shape limits**

- **Found during:** Task 2 file-length audit
- **Issue:** Final-membership rewriting and lifecycle assembly raised `package_admission.rs` above the hard 628-line production ceiling, while new prospective accessors reached the boundary.
- **Fix:** Extracted cohesive finalization and test-probe child modules, compacted the prospective accessors, and registered both sources for parity breadcrumbs.
- **Files modified:** `packages/open-bitcoin-mempool/src/pool/package_admission.rs`, `packages/open-bitcoin-mempool/src/pool/package_admission/finalization.rs`, `packages/open-bitcoin-mempool/src/pool/package_admission/test_support.rs`, `packages/open-bitcoin-mempool/src/pool/prospective.rs`, `docs/parity/source-breadcrumbs.json`
- **Verification:** All 315 production Rust files pass the file-length gate; the affected roots are 598, 627, 103, and 67 lines.
- **Committed in:** `ceaba3d0`

**Total deviations:** 4 auto-fixed (1 bug, 1 missing-critical code-shape fix, 2 blocking fixes)

**Impact on plan:** The deviations preserve the locked policy semantics while making residual tests accurate, reports internally consistent, coverage exhaustive, and production modules compliant. No new dependencies or external effect surfaces were added.

## Issues Encountered

- The first Task 1 hook exposed coverage gaps; focused branch tests and classifier extraction resolved them while retaining the full coverage requirement.
- A test-only mutable binding triggered production Clippy because the mutation existed only under `cfg(test)`; immutable shadow bindings removed the warning.
- The first Task 2 hook found stale breadcrumb headers for extracted modules; the repository breadcrumb writer synchronized their managed anchor blocks before the successful rerun.

## Verification

- `phase132-package-fees`: all package fee and TRUC policy tests passed.
- `phase132-final-rewrite`: 15 package-policy tests passed, including new-only, exact-existing, and witness-alias pressure rewrites.
- `phase132-lifecycle-regression`: all 16 lifecycle-delta regressions passed.
- All 26 package-admission regressions and the complete mempool suite passed.
- Parity breadcrumbs verified for 429 Rust files.
- The production file-length gate passed for all 315 Rust files.
- Both normal hooks passed formatting, Clippy with warnings denied, all-target builds, full tests and doctests, Bazel smoke builds, and workspace line coverage.
- Task 1 normal hook completed in 3m 1.264s; Task 2 normal hook completed in 3m 16.077s.
- Acceptance scans found checked fee facts, pinned TRUC modes, checked group construction, late-script seams, one final trim, final-membership rewriting, and lifecycle record calls.

## Authentication Gates

None.

## Known Stubs

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan 06 can add exact limited-replacement accounting on top of final authoritative membership and the separated incremental fee role.
- Plan 07 can replace the named TRUC and ephemeral stage hooks without changing report/lifecycle finalization.
- No implementation, verification, or external-service blockers remain.

## Self-Check: PASSED

- Summary file exists with the required generator, lifecycle, and requirement metadata.
- Task commits `98fe1195` and `ceaba3d0` exist in repository history.
- All created source files exist and are parity-registered.
- Summary diff is whitespace-clean, and parent-owned `STATE.md` and `ROADMAP.md` remain unstaged.
