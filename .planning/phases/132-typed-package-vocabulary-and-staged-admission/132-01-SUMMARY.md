---
phase: 132-typed-package-vocabulary-and-staged-admission
plan: "01"
subsystem: mempool
tags: [rust, package-policy, transaction-identity, fee-groups, bitcoin-knots-parity]

requires: []
provides:
  - Opaque checked package shape and submission-refinement types
  - Knots-compatible permutation-independent package fingerprints
  - Ordered impossible-state-free package reports and effective-fee groups
affects: [132-02, 132-03, 132-04, package-admission, mempool-policy]

tech-stack:
  added: []
  patterns:
    - Private invariant-bearing storage with checked public constructors
    - Request-ordered sum-type results with derived status and fee-group validation

key-files:
  created:
    - packages/open-bitcoin-mempool/src/package.rs
    - packages/open-bitcoin-mempool/src/package/shape.rs
    - packages/open-bitcoin-mempool/src/package/report.rs
    - packages/open-bitcoin-mempool/src/package/tests.rs
  modified:
    - packages/open-bitcoin-mempool/src/lib.rs
    - docs/parity/source-breadcrumbs.json
    - docs/metrics/lines-of-code.md

key-decisions:
  - "Effective-fee group identity exists only on newly evaluated or fee-reconsiderable typed outcomes; exact members and witness aliases cannot carry group membership."
  - "Amount makes negative and Bitcoin-range-invalid aggregate fees unrepresentable; EffectiveFeeGroup additionally rejects empty/duplicate membership, zero or out-of-range vsize, and inconsistent rates."
  - "Package and report breadcrumb registrations use narrow dedicated groups so exact Knots anchors do not rewrite unrelated existing mempool-policy headers."

patterns-established:
  - "Package capability pattern: validate raw transactions into WellFormedPackage, then refine only through SubmissionPackage::try_from_package."
  - "Report alignment pattern: derive status and effective-fee membership from input-index-aligned sum types inside PackageReport::try_new."

requirements-completed: []
requirements-addressed: [PACK-01, PACK-03, PACK-07]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 132-2026-07-25T18-13-00
generated_at: 2026-07-25T23:06:50Z

duration: 1h 37m
completed: 2026-07-25
---

# Phase 132 Plan 01: Opaque Package Vocabulary and Transactional Results Summary

**Opaque package/refinement capabilities with Knots-compatible fingerprints and input-aligned, checked package reports whose fee evidence cannot represent empty, aliased, or misordered membership**

## Performance

- **Duration:** 1h 37m
- **Started:** 2026-07-25T21:29:54Z
- **Completed:** 2026-07-25T23:06:50Z
- **Tasks:** 2
- **Files modified:** 7

## Accomplishments

- Rejects malformed package count, weight, identity, topology, zero-input, and cross-member-conflict shapes before contextual or script validation.
- Refines only singleton or direct-parent-plus-final-child submissions through a private checked boundary, with E0451 privacy proof.
- Matches the pinned Knots sorted-wtxid package fingerprint without changing request-ordered member storage.
- Validates ordered member identities, deterministic complete/partial/failed status, non-empty fee groups, fee arithmetic, and eligible group references.
- Proves report and fee-group construction/mutation opacity through E0451 and E0616 doctests.

## Task Commits

Each TDD task was committed as a RED/GREEN pair:

1. **Task 1 RED: Package vocabulary tests** - `e7ba7842`
2. **Task 1 GREEN: Opaque package shape, refinement, and fingerprint** - `01f29e17`
3. **Task 2 RED: Ordered report and fee-group tests** - `0fb08256`
4. **Task 2 GREEN: Checked report sum types and fee groups** - `509a59c4`

## Files Created/Modified

- `packages/open-bitcoin-mempool/src/package.rs` - Private ordered package storage, cached identities, fingerprinting, and submission capability.
- `packages/open-bitcoin-mempool/src/package/shape.rs` - Context-free shape checks and chainstate-backed submission refinement.
- `packages/open-bitcoin-mempool/src/package/report.rs` - Ordered member-result sum types, checked reports, and non-empty effective-fee groups.
- `packages/open-bitcoin-mempool/src/package/tests.rs` - Boundary, fixed-vector, alignment, fee-group, and opacity regressions.
- `packages/open-bitcoin-mempool/src/lib.rs` - Public package and report exports.
- `docs/parity/source-breadcrumbs.json` - Exact Knots source registrations for package policy and report vocabulary.
- `docs/metrics/lines-of-code.md` - Hook-regenerated tracked line-count report.

## Decisions Made

- Fee-group association is structurally absent from exact-present and witness-alias variants, rather than represented by optional state.
- Post-trim absence retains a typed prior success so final absence cannot erase whether the member had been new, exact-present, or a witness alias.
- Package status is derived from aligned member variants and checked against the supplied status instead of trusted as an independent flag.
- A temporary sorted wtxid copy feeds the fingerprint; the private member vector always remains in request order.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Used narrow breadcrumb groups for exact source anchors**

- **Found during:** Task 1
- **Issue:** Adding the new files to the broad existing `mempool-policy` group made the checker require the package-specific anchor set on 18 unrelated existing Rust files.
- **Fix:** Added dedicated `mempool-package-policy` and `mempool-package-report` groups with the exact plan-required Knots anchors.
- **Files modified:** `docs/parity/source-breadcrumbs.json`
- **Verification:** `bun run scripts/check-parity-breadcrumbs.ts` verifies 412 Rust files.
- **Committed in:** `e7ba7842`, `0fb08256`

**2. [Rule 3 - Blocking] Added complete pure-core line coverage for new invariant APIs**

- **Found during:** Task 1 pre-commit verification
- **Issue:** The strict repository hook rejected uncovered read-only accessors, typed error formatting, and defensive invariant branches.
- **Fix:** Added behavioral tests for every observable accessor, error variant, report status, group-validation branch, and internal empty-package defense; simplified the unreachable report index fallback by iterating aligned private storage directly.
- **Files modified:** `packages/open-bitcoin-mempool/src/package/tests.rs`, `packages/open-bitcoin-mempool/src/package/shape.rs`, `packages/open-bitcoin-mempool/src/package/report.rs`
- **Verification:** `cargo llvm-cov` reports no uncovered lines for the new pure-core files; `bash scripts/verify.sh` passes.
- **Committed in:** `e7ba7842`, `0fb08256`, `509a59c4`

**3. [Rule 3 - Blocking] Staged addressed requirements for phase-level verification**

- **Found during:** Summary metadata commit
- **Issue:** Active-milestone traceability rejects `requirements-completed` activation before Phase 132 has a lifecycle-valid phase verification artifact.
- **Fix:** Followed the repository's established lifecycle pattern: keep `requirements-completed: []` and record PACK-01, PACK-03, and PACK-07 under `requirements-addressed` until phase verification promotes them.
- **Files modified:** `.planning/phases/132-typed-package-vocabulary-and-staged-admission/132-01-SUMMARY.md`
- **Verification:** `bun test scripts/check-active-milestone-verification-traceability.test.ts` and the normal metadata commit hook.

**Total deviations:** 3 auto-fixed blocking issues.
**Impact on plan:** The changes were required by repository parity, coverage, and lifecycle contracts; none expanded runtime scope.

## Issues Encountered

- The checkout-default Cargo test artifact stalled at macOS `_dyld_start`, matching the documented host-cache lesson. After capturing process evidence, verification moved to an isolated `CARGO_TARGET_DIR`; no security metadata or project behavior was changed.
- The first Task 1 commit hook correctly failed on uncovered new pure-core lines. Focused regression coverage resolved the failure, and every subsequent normal hook and the final verifier passed.
- The first summary metadata hook correctly rejected premature requirement completion. The summary now records those requirements as addressed for the phase verifier to promote.

## Known Stubs

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Later Phase 132 admission stages can accept only checked `SubmissionPackage` capabilities and return `PackageReport` values aligned to the original package order.
- No blockers or threat flags remain; this plan adds no network endpoint, authentication path, filesystem trust boundary, or schema change.

## Self-Check: PASSED

- All seven created or modified implementation/evidence files and this summary exist.
- Task commits `e7ba7842`, `01f29e17`, `0fb08256`, and `509a59c4` resolve as commits.
- The summary has exactly one YAML frontmatter block and no whitespace errors.

*Phase: 132-typed-package-vocabulary-and-staged-admission*
*Completed: 2026-07-25*
