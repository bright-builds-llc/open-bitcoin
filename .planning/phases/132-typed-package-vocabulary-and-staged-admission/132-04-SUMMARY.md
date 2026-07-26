---
phase: 132-typed-package-vocabulary-and-staged-admission
plan: "04"
subsystem: mempool
tags: [rust, mempool, package-admission, partial-acceptance, dry-run, sparse-overlay, parity]
requires:
  - phase: 132-typed-package-vocabulary-and-staged-admission
    plan: "01"
    provides: Checked opaque package shape and submission refinement
  - phase: 132-typed-package-vocabulary-and-staged-admission
    plan: "02"
    provides: Pre-script candidate preparation and guarded revision-bound apply
  - phase: 132-typed-package-vocabulary-and-staged-admission
    plan: "03"
    provides: Sparse prospective mempool overlay and atomic patch preparation
provides:
  - Request-ordered individual-first package classification with exact and witness-alias handling
  - Residual reconsideration over one discardable prospective working view
  - Distinct non-mutating dry-run and checked submission capabilities
  - One guarded package apply returning the committed lifecycle delta
affects: [phase-132-package-policy, mempool, package-relay, rpc-package-surfaces]
tech-stack:
  added: []
  patterns:
    - Prepare every package transition against an immutable-base sparse view
    - Keep pre-script policy before late script execution for singleton and residual paths
    - Commit only one complete revision-bound patch after the full package report succeeds
key-files:
  created:
    - packages/open-bitcoin-mempool/src/pool/package_admission.rs
    - packages/open-bitcoin-mempool/src/pool/prospective/limits.rs
    - packages/open-bitcoin-mempool/src/pool/tests/package_admission_cases.rs
  modified:
    - packages/open-bitcoin-mempool/src/package.rs
    - packages/open-bitcoin-mempool/src/lib.rs
    - packages/open-bitcoin-mempool/src/pool.rs
    - packages/open-bitcoin-mempool/src/pool/candidate.rs
    - packages/open-bitcoin-mempool/src/pool/prospective.rs
    - packages/open-bitcoin-mempool/src/pool/tests.rs
    - docs/parity/source-breadcrumbs.json
    - docs/metrics/lines-of-code.md
key-decisions:
  - "Package submission consumes only the checked SubmissionPackage accessors, while dry-run accepts the broader checked WellFormedPackage."
  - "Every member is classified in request order; a hard member failure suppresses only residual retry and does not stop later singleton evaluation."
  - "Only a successful or aggregate-fee residual result replaces individual fee groups; failed residual work retains valid singleton report facts."
  - "Candidate preparation reads through a narrow mempool-view trait so children can consume parents staged in the sparse prospective overlay."
patterns-established:
  - "Individual-first package admission: exact/alias classification, singleton preparation, reconsiderable residual grouping, then one report."
  - "Capability-separated execution: dry-run drops the owned transition; submission alone may call guarded apply."
requirements-addressed: [PACK-02, PACK-03, PACK-04, PACK-05]
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 132-2026-07-25T18-13-00
generated_at: 2026-07-26T05:54:39Z
duration: 38m
completed: 2026-07-26
---

# Phase 132 Plan 04: Individual-First Package Admission Summary

**Request-ordered package admission now preserves partial successes, retries only reconsiderable members as one residual group, and separates non-mutating dry-run from one guarded checked submission apply.**

## Performance

- **Duration:** 38m
- **Started:** 2026-07-26T05:16:43Z
- **Completed:** 2026-07-26T05:54:39Z
- **Tasks:** 2
- **Files modified:** 11

## Accomplishments

- Added the exact dry-run and submission command/result capabilities, with submission accepting only the opaque checked `SubmissionPackage`.
- Implemented request-ordered exact-existing, same-txid/different-witness, singleton success, reconsiderable, and hard-failure classification without global all-or-nothing behavior.
- Composed missing or aggregate-fee-reconsiderable members in one discardable residual view, preserving successful singleton parents and valid fee-group report facts when later work fails.
- Kept script checks after pre-script policy and limit checks, with fault and behavior tests proving failures never leak prospective membership into live state.
- Added one guarded submit apply and a fully non-mutating dry-run path, including stale, no-op, late-failure, and dry-run/submit report-equivalence regressions.

## Task Commits

Each task was committed atomically:

1. **Task 1: Implement individual-first package classification and capability types** - `a60ccd73`
2. **Task 2: Prove residual discard and one guarded submit boundary** - `63ec324b`

## Files Created/Modified

- `packages/open-bitcoin-mempool/src/package.rs` - Dry-run/submission capabilities and crate-private identity-preserving member iteration.
- `packages/open-bitcoin-mempool/src/lib.rs` - Public package command and result exports.
- `packages/open-bitcoin-mempool/src/pool.rs` - Package admission registration and candidate input lookup over a narrow view.
- `packages/open-bitcoin-mempool/src/pool/candidate.rs` - Candidate preparation generalized across live and prospective mempool views.
- `packages/open-bitcoin-mempool/src/pool/package_admission.rs` - Individual-first state machine, residual composition, report construction, dry-run, and guarded submit.
- `packages/open-bitcoin-mempool/src/pool/prospective.rs` - Focused limits child-module registration.
- `packages/open-bitcoin-mempool/src/pool/prospective/limits.rs` - Prospective admission-fee and ancestor/descendant policy checks.
- `packages/open-bitcoin-mempool/src/pool/tests.rs` - Package admission regression-module registration.
- `packages/open-bitcoin-mempool/src/pool/tests/package_admission_cases.rs` - Ordered, residual, alias, script, limit, non-mutation, and atomic-submit regressions.
- `docs/parity/source-breadcrumbs.json` - Registered all new package admission and prospective-limit sources.
- `docs/metrics/lines-of-code.md` - Refreshed tracked source metrics through normal hooks.

## Decisions Made

- The package evaluator owns a `ProspectiveMempool` and advances it only after a singleton passes preparation, fee, limits, and script checks.
- Hard failure marks the package partial/failed and suppresses residual aggregation, while request-order evaluation continues for later independent singleton members.
- Residual evaluation clones the current prospective view, stages all reconsiderable members, applies aggregate fee and late-script checks, and publishes the working view only after complete success.
- Dry-run exposes no patch or lifecycle capability. Submission consumes the checked refinement through `package()` and `kind()` and has the sole `apply_prepared` call site.
- Lifecycle output records only members newly made present by this package; exact-existing and witness-alias observations remain report-only.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Generalized candidate preparation over a narrow mempool view**

- **Found during:** Task 1
- **Issue:** Plan 02 candidate preparation accepted only `&Mempool`, so a residual child could not resolve an unconfirmed parent staged in the evolving sparse overlay without cloning live state.
- **Fix:** Added the private `CandidateMempoolView` contract and implemented it for both `Mempool` and `ProspectiveMempool`; input-context and candidate preparation now read through that contract.
- **Files modified:** `packages/open-bitcoin-mempool/src/pool.rs`, `packages/open-bitcoin-mempool/src/pool/candidate.rs`, `packages/open-bitcoin-mempool/src/pool/prospective/limits.rs`
- **Verification:** The parent-below-floor plus child aggregate-fee package succeeds as one residual group and commits both members.
- **Committed in:** `a60ccd73`

**2. [Rule 3 - Blocking] Extracted prospective policy checks to satisfy the production file limit**

- **Found during:** Task 1 normal pre-commit hook
- **Issue:** Adding package-facing fee and ancestor/descendant checks raised `prospective.rs` to 684 lines, above the repository's 628-line production limit.
- **Fix:** Extracted the cohesive checks and candidate-view implementation into `prospective/limits.rs`, leaving `prospective.rs` at 602 lines.
- **Files modified:** `packages/open-bitcoin-mempool/src/pool/prospective.rs`, `packages/open-bitcoin-mempool/src/pool/prospective/limits.rs`, `docs/parity/source-breadcrumbs.json`
- **Verification:** The file-length checker passed for all 313 production Rust files, and all prospective limit directions/kinds are covered.
- **Committed in:** `a60ccd73`

**3. [Rule 1 - Bug] Retained individual fee groups when residual work fails**

- **Found during:** Task 2 fail-closed residual coverage
- **Issue:** Individual reconsiderable fee groups were removed before residual composition, so a later residual policy, limit, or script failure left an otherwise valid member result referencing a missing fee group.
- **Fix:** Individual groups are now replaced only when residual evaluation produces a successful or aggregate-fee group; failed residual work preserves the original singleton group facts.
- **Files modified:** `packages/open-bitcoin-mempool/src/pool/package_admission.rs`, `packages/open-bitcoin-mempool/src/pool/tests/package_admission_cases.rs`
- **Verification:** Residual policy, limit, and script failures produce valid opaque reports and leave the complete live snapshot unchanged.
- **Committed in:** `a60ccd73`

**4. [Rule 3 - Blocking] Added exhaustive fail-closed coverage for the repository gate**

- **Found during:** Task 1 normal pre-commit hook
- **Issue:** The implementation passed behavior, Clippy, build, Bazel, and full tests, but the required pure-core coverage gate exposed unexecuted composition, residual, limit, and invariant paths.
- **Fix:** Added focused fault and policy regressions for singleton/residual composition errors, no-patch submit, every prospective limit dimension, missing members, late scripts, and report/group invariants.
- **Files modified:** `packages/open-bitcoin-mempool/src/pool/package_admission.rs`, `packages/open-bitcoin-mempool/src/pool/tests/package_admission_cases.rs`
- **Verification:** Focused `cargo llvm-cov` reported no uncovered lines, followed by two successful normal hooks.
- **Committed in:** `a60ccd73`, `63ec324b`

**Total deviations:** 4 auto-fixed (1 bug, 3 blocking)

**Impact on plan:** All fixes were necessary for correct residual reporting, prospective parent visibility, repository code-shape, and mandatory verification. No adapter effects, new dependencies, or external surfaces were introduced.

## Issues Encountered

- The first hook attempt identified stale generated breadcrumb blocks for the two new Rust files; the repository writer refreshed them before verification resumed.
- Clippy required boxing the large accepted singleton variant because the sparse prospective view made the enum disproportionately large.
- The first full verifier rerun stopped at the production file-length gate, and the next stopped at the pure-core coverage gate. The extracted limits module and focused fail-closed regressions resolved both without weakening checks.

## Verification

- `phase132-package-classify`: 24 focused tests passed before the Task 1 commit.
- `phase132-partial-package`: 25 focused tests passed after the atomic-submit regression.
- `phase132-dry-run`: both non-mutating dry-run regressions passed.
- Complete `open-bitcoin-mempool` tests, parity tests, and compile-fail doctests passed.
- Focused `cargo llvm-cov` reported no uncovered package-admission or prospective-limit lines.
- Acceptance scans found all four capability types, immutable refinement accessors, ordered classification states, distinct pre-script/late-script calls, both public APIs, and exactly one `apply_prepared` site.
- Capability and implementation scans found no relay, persistence, storage, managed-network, compact, orphan, metric, or logging effect surface.
- The Task 1 normal hook passed `bash scripts/verify.sh` in 3m 33.174s.
- The Task 2 normal hook passed `bash scripts/verify.sh` in 3m 20.172s.

## Authentication Gates

None.

## Known Stubs

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plans 05-07 can extend package policy decisions over the same opaque report and one guarded transition boundary.
- Checked submission, residual fee groups, partial acceptance, and dry-run semantics are available without node, RPC, storage, or network dependencies.
- No implementation or external-service blockers remain.

## Self-Check: PASSED

- Summary file exists and its lifecycle and requirement metadata match the originating plan.
- Task commits `a60ccd73` and `63ec324b` exist in repository history.
- Summary diff is whitespace-clean, and the parent-owned `STATE.md` and `ROADMAP.md` remain unstaged.
