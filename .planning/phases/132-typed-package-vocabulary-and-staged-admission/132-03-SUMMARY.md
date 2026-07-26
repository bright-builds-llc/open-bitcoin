---
phase: 132-typed-package-vocabulary-and-staged-admission
plan: "03"
subsystem: mempool
tags: [rust, mempool, sparse-overlay, pressure-trim, resource-accounting, parity]
requires:
  - phase: 132-typed-package-vocabulary-and-staged-admission
    plan: "02"
    provides: Revision-bound sparse patch contract and test-only full-state oracle
  - phase: 131-rolling-fee-expiry-and-descendant-eviction-core
    provides: Descendant-score pressure ordering and rolling-fee bump behavior
provides:
  - Overlay-first prospective mempool view owning only touched transition facts
  - Checked resource subtraction and replacement primitives that fail on underflow
  - Prospective descendant-package pressure selection, removal, and rolling bump
  - Generated-graph and failure oracles proving sparse composition and rollback
affects: [phase-132-package-admission, mempool, package-policy, pressure]
tech-stack:
  added: []
  patterns:
    - Borrow immutable live state and own only touched prospective facts
    - Clone the sparse overlay for atomic composition and publish only successful changes
    - Run descendant-score pressure selection once over the final prospective view
key-files:
  created:
    - packages/open-bitcoin-mempool/src/pool/prospective.rs
    - packages/open-bitcoin-mempool/src/pool/tests/prospective_oracle_cases.rs
    - packages/open-bitcoin-mempool/src/pool/tests/prospective_failure_cases.rs
  modified:
    - packages/open-bitcoin-mempool/src/resource.rs
    - packages/open-bitcoin-mempool/src/pool.rs
    - packages/open-bitcoin-mempool/src/pool/pressure.rs
    - packages/open-bitcoin-mempool/src/pool/tests/pressure_internal_cases.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - "Prospective state borrows the immutable base and resolves removals and touched updates before base entries or spenders."
  - "Checked composition and pressure trim operate on a cloned sparse overlay and replace the caller view only after every fallible step succeeds."
  - "Pressure selection is generic over the prospective view while retaining Phase 131 minimum descendant-score and txid ordering."
  - "Full-state materialization and canonical recomputation remain test-only independent oracles."
patterns-established:
  - "Sparse prospective transition: base borrow plus touched entry, removal, spent-index, topology, resource, rolling, and lifecycle facts."
  - "Atomic prospective mutation: prepare against a sparse working clone, then publish the complete successful overlay."
requirements-addressed: [PACK-05]
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 132-2026-07-25T18-13-00
generated_at: 2026-07-26T05:08:20Z
duration: 1h 3m
completed: 2026-07-26
---

# Phase 132 Plan 03: Sparse Prospective Overlay and Pressure Trim Summary

**A checked sparse mempool overlay now composes touched topology and resource facts, applies Phase 131 descendant pressure once, and proves equivalence to test-only canonical recomputation.**

## Performance

- **Duration:** 1h 3m
- **Started:** 2026-07-26T04:05:19Z
- **Completed:** 2026-07-26T05:08:20Z
- **Tasks:** 2
- **Files modified:** 10

## Accomplishments

- Added overlay-first entry and spent-outpoint lookup over an immutable base, with owned sparse additions, removals, topology replacements, resource deltas, rolling state, and lifecycle facts.
- Added checked resource-ledger removal and replacement operations that report typed underflow instead of wrapping, saturating, or partially changing state.
- Reused Phase 131 descendant-score and txid ordering against the prospective view, removing complete descendant packages and bumping rolling state from actual removed fee/vsize facts.
- Proved sparse behavior with generated graph, conflict, arithmetic, rollback, and 25-member pressure oracles; the bounded case performs zero production full-state clones/recomputes and exactly one final trim.

## Task Commits

Each task was committed atomically:

1. **Task 1: Implement checked sparse overlay and touched-state accounting** - `d77d484e`
2. **Task 2: Run Phase 131 pressure selection and trim over the overlay exactly once** - `7cf3b167`

## Files Created/Modified

- `packages/open-bitcoin-mempool/src/resource.rs` - Checked entry/spent-index subtraction and replacement with typed underflow.
- `packages/open-bitcoin-mempool/src/pool.rs` - Prospective module registration and pressure-module production wiring.
- `packages/open-bitcoin-mempool/src/pool/prospective.rs` - Borrowed-base sparse view, checked composition, patch preparation, and test-only oracle instrumentation.
- `packages/open-bitcoin-mempool/src/pool/pressure.rs` - Prospective descendant-score selector and atomic trim-to-capacity implementation.
- `packages/open-bitcoin-mempool/src/pool/tests.rs` - Focused prospective and pressure test-module registration.
- `packages/open-bitcoin-mempool/src/pool/tests/prospective_oracle_cases.rs` - Generated graph, composition, 25-member pressure, and no-full-clone oracles.
- `packages/open-bitcoin-mempool/src/pool/tests/prospective_failure_cases.rs` - Identity, spent-index, arithmetic, topology, and rollback failures.
- `packages/open-bitcoin-mempool/src/pool/tests/pressure_internal_cases.rs` - Prospective pressure success, empty, fail-closed, and malformed-descendant cases.
- `docs/parity/source-breadcrumbs.json` - Registered the new prospective production and oracle sources.
- `docs/metrics/lines-of-code.md` - Refreshed tracked worktree LOC evidence.

## Decisions Made

- The prospective view owns only touched facts and consults removals and overlay updates before falling back to the immutable base.
- Composition and pressure trim use a sparse working clone. The caller-visible overlay is replaced only after all checked resource, topology, spent-index, and rolling changes succeed.
- The pressure selector accepts the prospective view directly, preserving minimum descendant score followed by txid as the deterministic tie-break.
- A private `trim_to_size` compatibility seam retains the established Phase 131 static evidence contract while the new public-in-module prospective entry point delegates to it; this does not restore full-state cloning.
- Canonical state and resource recomputation are compiled only for tests and remain independent of production transition preparation.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Split prospective failure and internal pressure tests into focused modules**

- **Found during:** Tasks 1 and 2 code-shape pass
- **Issue:** Keeping every planned oracle and failure case in the single prospective oracle file would exceed the repository file-length guidance and mix success oracles with distinct failure concerns.
- **Fix:** Kept generated equivalence and bounded-work cases in `prospective_oracle_cases.rs`, moved checked failure cases to `prospective_failure_cases.rs`, and extended the existing internal pressure module for selector-specific behavior.
- **Files modified:** `packages/open-bitcoin-mempool/src/pool/tests.rs`, `packages/open-bitcoin-mempool/src/pool/tests/prospective_oracle_cases.rs`, `packages/open-bitcoin-mempool/src/pool/tests/prospective_failure_cases.rs`, `packages/open-bitcoin-mempool/src/pool/tests/pressure_internal_cases.rs`
- **Verification:** All three focused test modules remain below 500 lines; `pressure.rs` is 111 lines. The 600-line `prospective.rs` remains one cohesive bounded transition unit, and targeted coverage reported no missing lines in the new prospective/resource/pressure paths.
- **Committed in:** `d77d484e`, `7cf3b167`

**2. [Rule 3 - Blocking] Preserved the Phase 131 static pressure compatibility seam**

- **Found during:** Task 2 normal pre-commit hook
- **Issue:** The Phase 131 regression checker intentionally anchors the established pressure implementation at a private function named `trim_to_size`; replacing that name caused the otherwise-green hook to reject the refactor.
- **Fix:** Made `trim_prospective_to_capacity` delegate to a private `trim_to_size` implementation that operates exclusively on `ProspectiveMempool`.
- **Files modified:** `packages/open-bitcoin-mempool/src/pool/pressure.rs`
- **Verification:** The Phase 131 checker and all 13 mutation tests passed, followed by the complete repository verifier.
- **Committed in:** `7cf3b167`

**3. [Rule 3 - Blocking] Recoverably quarantined a stalled ignored Cargo cache**

- **Found during:** Task 1 normal pre-commit hook
- **Issue:** Filesystem enumeration under the pre-existing ignored `packages/target` cache stalled repository Bun checks for minutes before any source verification could proceed.
- **Fix:** Moved the cache intact to `/tmp/open-bitcoin-stalled-target.ZmWhcU/target` and ran all Cargo work against the isolated `CARGO_TARGET_DIR=/tmp/open-bitcoin-phase132-03.COCKIE`. The cache was not deleted and is recoverable.
- **Files modified:** None
- **Verification:** Bun fixture checks returned to sub-second startup, and both normal task hooks completed the full verifier successfully.
- **Committed in:** Not applicable; ignored local cache handling only.

**Total deviations:** 3 auto-fixed (3 blocking)

**Impact on plan:** The fixes preserved repository verification and code-shape contracts without broadening runtime behavior or introducing a production full-state materialization path.

## Issues Encountered

- The first Task 2 commit attempt stopped at the Phase 131 static checker because the pressure helper name had changed. The compatibility seam above fixed the evidence contract without changing prospective semantics.
- The ignored Cargo cache stall was isolated from source work through a recoverable move and a checkout-independent target directory. A small benchmark-report subtree was recreated by the successful verifier under the ignored target path.

## Verification

- Targeted prospective oracle and pressure regression suites passed.
- The complete `open-bitcoin-mempool` library suite passed 228 tests; its five compile-fail doctests also passed.
- Targeted `cargo llvm-cov` reported no missing lines in `prospective.rs`, `resource.rs`, or `pressure.rs`.
- Acceptance scans found every checked subtraction, sparse overlay, pressure selector, rolling-bump, and patch seam, with no production `entries.clone()` or `recompute_state` use in `pressure.rs`.
- The Task 1 normal pre-commit hook passed `bash scripts/verify.sh` in 2m 54s.
- The Task 2 normal pre-commit hook passed `bash scripts/verify.sh` in 3m 1.8s, including format, Clippy, all-target build, full tests/doctests, benchmark smoke, Bazel, parity guardrails, and coverage.

## Authentication Gates

None.

## Known Stubs

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan 04 package orchestration can stage up to 25 prepared members into one prospective view, invoke the final pressure trim once, and consume one owned revision-bound patch.
- Checked topology, resource, spent-index, rolling, and lifecycle facts are ready for package-level late-script and atomic-apply integration.
- No implementation or external-service blockers remain.

## Self-Check: PASSED

- Summary file exists and its lifecycle and requirement metadata match the originating plan.
- Task commits `d77d484e` and `7cf3b167` exist in repository history.
- Summary diff is whitespace-clean, and the parent-owned `STATE.md` and `ROADMAP.md` remain unstaged.
