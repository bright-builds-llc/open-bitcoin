---
phase: 102-orphan-handling-and-admission-outcome-bridge
plan: 01
subsystem: mempool
tags: [rust, mempool, admission, parity, testing]

requires:
  - phase: 101-transaction-inventory-download-scheduling
    provides: transaction inventory scheduling and bounded deferred-surface guardrails
provides:
  - Stable `MempoolOutcome` admission contract with low-cardinality labels and rejection categories
  - Outcome-producing pure mempool and managed-node submission methods
  - Outcome mapping and no-partial-mutation regression coverage for rejection and eviction paths
  - Parity breadcrumb coverage for new mempool outcome source and test files
affects: [phase-102, mempool, transaction-relay, orphan-handling]

tech-stack:
  added: []
  patterns:
    - typed admission outcome contract
    - pure mempool admission wrapper preserving existing admission result API
    - snapshot-based mutation guards for rejection paths

key-files:
  created:
    - packages/open-bitcoin-mempool/src/outcome.rs
    - packages/open-bitcoin-mempool/src/pool/admission_outcome.rs
    - packages/open-bitcoin-mempool/src/pool/tests/outcome_cases.rs
  modified:
    - packages/open-bitcoin-mempool/src/lib.rs
    - packages/open-bitcoin-mempool/src/pool.rs
    - packages/open-bitcoin-mempool/src/pool/tests.rs
    - packages/open-bitcoin-node/src/mempool.rs
    - docs/parity/source-breadcrumbs.json
    - docs/metrics/lines-of-code.md

key-decisions:
  - "Preserved the existing `accept_transaction` API and added `accept_transaction_outcome` as an outcome bridge."
  - "Moved outcome mapping into `pool/admission_outcome.rs` to keep `pool.rs` under the repo production file-length gate."
  - "Kept admission mutation snapshotting test-only instead of widening production mempool internals."

patterns-established:
  - "Mempool outcome labels and rejection categories are fixed string APIs through `as_str` methods."
  - "Admission failures that are expected submission outcomes become typed `MempoolOutcome` values rather than display-string parsing."
  - "No-partial-mutation tests compare accepted txids, graph links, spent outpoints, and total virtual size before and after rejection paths."

requirements-completed: [MEM-01, MEM-02]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 102-2026-06-30T14-54-50
generated_at: 2026-07-01T01:35:51Z

duration: 1h 17m
completed: 2026-07-01
---

# Phase 102 Plan 01: Mempool Admission Outcome Contract Summary

**Typed mempool admission outcomes with stable labels, orphan parent lists, replacement/eviction distinction, and mutation guards for rejected submissions**

## Performance

- **Duration:** 1h 17m
- **Started:** 2026-07-01T00:18:45Z
- **Completed:** 2026-07-01T01:35:51Z
- **Tasks:** 2
- **Files modified:** 9

## Accomplishments

- Added `MempoolOutcome`, `MempoolOutcomeLabel`, and `MempoolRejectionCategory` as the shared typed admission result surface.
- Added `Mempool::accept_transaction_outcome` and `ManagedMempool::submit_transaction_outcome` while preserving the existing `AdmissionResult` path.
- Mapped duplicate, orphan, replacement, rejection, and candidate-eviction admission paths to stable typed outcomes.
- Added focused tests for outcome labels, rejection categories, orphan missing parent txids, replacement-vs-eviction distinction, and no-partial-mutation rejection behavior.
- Updated parity breadcrumbs for new mempool source/test files and refreshed the tracked LOC report.

## Task Commits

Each task was committed atomically:

1. **Task 1: Outcome contract and admission bridge** - `e174207d` (`feat`)
2. **Task 2: No-partial-mutation regressions** - `4b49972c` (`test`)

## Files Created/Modified

- `packages/open-bitcoin-mempool/src/outcome.rs` - Stable admission outcome labels, rejection categories, variants, and accessors.
- `packages/open-bitcoin-mempool/src/pool/admission_outcome.rs` - Outcome wrapper around pure mempool admission and missing-parent collection.
- `packages/open-bitcoin-mempool/src/lib.rs` - Public outcome module and re-exports.
- `packages/open-bitcoin-mempool/src/pool.rs` - `accept_transaction_outcome` bridge.
- `packages/open-bitcoin-mempool/src/pool/tests.rs` - Test fixture exports and outcome test module registration.
- `packages/open-bitcoin-mempool/src/pool/tests/outcome_cases.rs` - Outcome mapping coverage and admission snapshot mutation guards.
- `packages/open-bitcoin-node/src/mempool.rs` - Managed mempool outcome submission wrapper.
- `docs/parity/source-breadcrumbs.json` - Breadcrumb coverage for new mempool outcome source/test files.
- `docs/metrics/lines-of-code.md` - Refreshed tracked generated LOC report.

## Decisions Made

- Preserved the older `accept_transaction` API to avoid forcing callers to migrate before later Phase 102 plans are ready.
- Returned expected submission states such as duplicate, orphan, rejected, and candidate-evicted as `Ok(MempoolOutcome)` from the new outcome wrapper; serialization/internal failures can still propagate as `MempoolError`.
- Used a child helper module for outcome mapping so the production file-length verifier stays satisfied without changing mempool ownership boundaries.
- Kept `MempoolAdmissionSnapshot` in tests, using private field access from the descendant test module rather than adding a production getter for `spent_outpoints`.

## Verification

- `cargo fmt --manifest-path packages/Cargo.toml --all --check` passed.
- `cargo clippy --manifest-path packages/Cargo.toml --all-targets --all-features -- -D warnings` passed.
- `cargo build --manifest-path packages/Cargo.toml --all-targets --all-features` passed.
- `cargo test --manifest-path packages/Cargo.toml --all-features` passed.
- `timeout 120s cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-mempool --lib outcome -- --nocapture` passed: 12 tests.
- `timeout 120s cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-mempool --lib no_partial_mutation -- --nocapture` passed: 6 tests.
- `bun run scripts/check-parity-breadcrumbs.ts --check` passed during commit verification: 315 Rust files verified.
- `bun run scripts/generate-loc-report.ts --source=worktree --output=docs/metrics/lines-of-code.md --check` passed.
- `bash scripts/verify.sh` passed in commit hooks for both task commits; the Task 2 hook completed in 4m 17.149s.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Split outcome mapping out of `pool.rs`**
- **Found during:** Task 1
- **Issue:** The first implementation pushed `pool.rs` over the repo production Rust file-length gate.
- **Fix:** Added `packages/open-bitcoin-mempool/src/pool/admission_outcome.rs` and kept the public wrapper in `pool.rs`.
- **Files modified:** `packages/open-bitcoin-mempool/src/pool.rs`, `packages/open-bitcoin-mempool/src/pool/admission_outcome.rs`, `docs/parity/source-breadcrumbs.json`
- **Verification:** `bash scripts/check-file-lengths.sh` passed in `bash scripts/verify.sh`.
- **Committed in:** `e174207d`

**2. [Rule 3 - Blocking] Added focused coverage for new outcome branches**
- **Found during:** Task 1
- **Issue:** Coverage verification identified unexercised outcome/accessor and helper branches.
- **Fix:** Added focused outcome label, accessor, category, orphan, replacement, and eviction tests.
- **Files modified:** `packages/open-bitcoin-mempool/src/pool/tests/outcome_cases.rs`
- **Verification:** `cargo llvm-cov --manifest-path packages/Cargo.toml --package open-bitcoin-mempool --show-missing-lines --text` produced no uncovered-line section for the focused mempool check before commit; `bash scripts/verify.sh` passed.
- **Committed in:** `e174207d`

**3. [Rule 2 - Repo Instruction] Used canonical mempool-policy breadcrumbs**
- **Found during:** Task 1
- **Issue:** The plan referenced exact Knots files, but repo tooling canonicalized new mempool files into the existing `mempool-policy` breadcrumb group.
- **Fix:** Registered new Rust files under the canonical `mempool-policy` source anchors.
- **Files modified:** `docs/parity/source-breadcrumbs.json`
- **Verification:** `bun run scripts/check-parity-breadcrumbs.ts --check` passed.
- **Committed in:** `e174207d`

**Total deviations:** 3 auto-fixed (1 missing critical/repo instruction, 2 blocking)
**Impact on plan:** No scope creep. The changes were required to satisfy repo verification, breadcrumb policy, and the plan's intended outcome coverage.

## Issues Encountered

- TDD RED tests were observed locally but not committed because repo hooks require passing verification before every commit. The final task commits preserve the repo's passing-state commit invariant.
- The first Task 1 commit command reported a stale `HEAD` ref after the long hook, but the commit had landed as `e174207d`; follow-up `git status --short` was clean and `git log` confirmed the commit.

## Known Stubs

None - stub/placeholder scan returned no matches across the files created or modified by this plan.

## Threat Flags

None - this plan added no new network endpoints, auth paths, file access patterns, schema changes, or trust-boundary crossings.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Later Phase 102 plans can consume `MempoolOutcome::Orphaned { missing_parents, .. }` instead of matching `MempoolError` display text.
- Replacement and candidate eviction are now distinguishable for relay/orphan bridge decisions.
- Rejected admission paths have regression coverage proving no partial mutation of entries, parent/child links, spent-outpoint indexes, or total virtual size.

## Self-Check

PASSED

- Summary and key created files exist.
- Task commits `e174207d` and `4b49972c` exist in git history.
- Markdown frontmatter uses only the opening and closing `---` delimiters.

*Phase: 102-orphan-handling-and-admission-outcome-bridge*
*Completed: 2026-07-01*
