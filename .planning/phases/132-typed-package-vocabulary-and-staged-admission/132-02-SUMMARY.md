---
phase: 132-typed-package-vocabulary-and-staged-admission
plan: "02"
subsystem: mempool
tags: [rust, mempool, staged-admission, sparse-patch, revision-guard, parity]
requires:
  - phase: 130-resource-time-and-fee-primitives
    provides: Typed resource, time, fee-role, and lifecycle-delta primitives
  - phase: 131-rolling-fee-expiry-and-descendant-eviction-core
    provides: Rolling-fee, expiry, pressure, and descendant-eviction behavior
provides:
  - Shared non-mutating pre-script candidate preparation and contextual script checking
  - Monotonic mempool revision covering membership, indexes, resources, and rolling state
  - Revision-bound sparse patches with stale-base rejection before mutation
  - Complete regression and oracle coverage for legacy lifecycle outcomes
affects: [phase-132-package-admission, mempool, package-policy, relay-admission]
tech-stack:
  added: []
  patterns:
    - Prepare every fallible transition artifact before applying an infallible sparse patch
    - Bind prepared transitions to an exact monotonic state revision
    - Keep full-state materialization test-only as an independent oracle
key-files:
  created:
    - packages/open-bitcoin-mempool/src/pool/candidate.rs
    - packages/open-bitcoin-mempool/src/pool/patch.rs
    - packages/open-bitcoin-mempool/src/pool/patch/graph.rs
    - packages/open-bitcoin-mempool/src/pool/oracle.rs
    - packages/open-bitcoin-mempool/src/pool/tests/revision_cases.rs
  modified:
    - packages/open-bitcoin-mempool/src/pool.rs
    - packages/open-bitcoin-mempool/src/pool/admission.rs
    - packages/open-bitcoin-mempool/src/pool/expiry.rs
    - packages/open-bitcoin-mempool/src/pool/lifecycle.rs
    - packages/open-bitcoin-node/src/network.rs
key-decisions:
  - "Candidate preparation owns pre-script facts only; contextual scripts remain a separate late checker."
  - "Prepared transitions carry exact sparse map, topology, resource, rolling, revision, and lifecycle edits rather than a full mempool state."
  - "Patch application checks revision equality first, performs no fallible work, and assigns the next revision last."
  - "Full-state materialization remains test-only as an independent sparse-patch oracle."
patterns-established:
  - "Prepare-check-apply: finish policy, arithmetic, allocation, lifecycle composition, and revision calculation before touching live state."
  - "Revision coverage: every decision-relevant mutation advances exactly once; true no-ops do not advance."
requirements-addressed: [PACK-05]
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 132-2026-07-25T18-13-00
generated_at: 2026-07-26T03:06:10Z
duration: 5h 32m
completed: 2026-07-26
---

# Phase 132 Plan 02: Staged Admission and Revision-Bound Sparse Patches Summary

**Shared pre-script candidate preparation now feeds revision-bound sparse mempool patches that reject stale bases before mutation while preserving legacy outcomes and lifecycle ordering.**

## Performance

- **Duration:** 5h 32m
- **Started:** 2026-07-25T21:34:42Z
- **Completed:** 2026-07-26T03:06:10Z
- **Tasks:** 2
- **Files modified:** 30

## Accomplishments

- Extracted a reusable non-mutating candidate preparer and a separate contextual script checker, preserving the single-transaction failure order while establishing the late-script seam needed by package admission.
- Added a complete monotonic revision across membership, indexes, accounted resources, rolling-fee state, expiry, pressure, and connected-block cleanup.
- Replaced full-state prepared transitions with owned sparse patches guarded by exact base revision; stale or exhausted transitions fail before live mutation.
- Added test-only full-state and pressure oracles plus focused regression coverage for stale membership, rolling-only changes, revision overflow, no-op behavior, lifecycle deltas, and legacy outcomes.

## Task Commits

Each task was committed atomically:

1. **Task 1: Extract pre-script candidate preparation and the separate script checker** - `f505cc78`
2. **Task 2: Bind prepared transitions to a complete monotonic mempool revision** - `d7d8febd`

## Files Created/Modified

- `packages/open-bitcoin-mempool/src/pool/candidate.rs` - Shared pre-script candidate facts and contextual script checker.
- `packages/open-bitcoin-mempool/src/pool/patch.rs` - Sparse patch contract, resource-delta preparation, and guarded apply seam.
- `packages/open-bitcoin-mempool/src/pool/patch/graph.rs` - Prospective topology, aggregate, limit, closure, and eviction preparation.
- `packages/open-bitcoin-mempool/src/pool/oracle.rs` - Test-only complete-state materializer for independent patch verification.
- `packages/open-bitcoin-mempool/src/pool.rs` - Monotonic revision ownership and rolling-only mutation integration.
- `packages/open-bitcoin-mempool/src/pool/admission.rs` - Ordered prepare, policy, script, patch-build, and apply flow.
- `packages/open-bitcoin-mempool/src/pool/expiry.rs` - Sparse expiry-removal patch preparation.
- `packages/open-bitcoin-mempool/src/pool/lifecycle.rs` - Sparse connected-block and conflict cleanup.
- `packages/open-bitcoin-mempool/src/pool/tests/revision_cases.rs` - Revision, stale-base, ordering, overflow, and no-op regressions.
- `packages/open-bitcoin-node/src/network.rs` - Fallible rolling-state propagation through the managed network boundary.
- `packages/open-bitcoin-rpc/src/dispatch/tests.rs` - Updated RPC behavior coverage for fallible mempool operations.
- `docs/parity/source-breadcrumbs.json` - Registered candidate, patch, oracle, and regression sources.
- `docs/metrics/lines-of-code.md` - Refreshed tracked worktree LOC evidence.

## Decisions Made

- Candidate preparation does not accept script flags and cannot invoke scripts; the shared script checker consumes the prepared context only after legacy pre-script policy stages succeed.
- `MempoolPatch` contains only touched entry, spent-index, topology, resource, rolling, revision, and lifecycle facts. No production transition owns or materializes a full `MempoolState`.
- Revision comparison is the first operation in `apply_prepared`; all remaining application work is infallible, and the next revision is assigned last.
- `patch.rs` was split at the graph-preparation boundary so sparse patch/application ownership remains cohesive and production files stay below the repository size limit.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - Missing Critical] Added total ordering for sparse spent-index keys**

- **Found during:** Task 2
- **Issue:** The required deterministic `BTreeMap<OutPoint, Option<Txid>>` patch shape needed `OutPoint` ordering.
- **Fix:** Derived `Ord` and `PartialOrd` for `OutPoint`.
- **Files modified:** `packages/open-bitcoin-primitives/src/transaction.rs`
- **Verification:** Clippy, all-target build/tests, Bazel, and full coverage passed.
- **Committed in:** `d7d8febd`

**2. [Rule 3 - Blocking] Propagated fallible rolling mutations through downstream callers**

- **Found during:** Task 2
- **Issue:** Checked next-revision preparation makes rolling-fee updates fallible, so node, network, and RPC callers could no longer ignore completion errors.
- **Fix:** Propagated `Result` through managed mempool/network seams and updated behavior tests.
- **Files modified:** `packages/open-bitcoin-node/src/mempool.rs`, `packages/open-bitcoin-node/src/network.rs`, `packages/open-bitcoin-node/src/network/tests.rs`, `packages/open-bitcoin-rpc/src/dispatch/tests.rs`
- **Verification:** Node 490/490, network 468/468, and RPC 154/154 tests passed in the full verifier.
- **Committed in:** `d7d8febd`

**3. [Rule 2 - Missing Critical] Added independent test-only state and pressure oracles**

- **Found during:** Task 2
- **Issue:** Sparse application needed an independent full-state comparison and complete legacy pressure branch coverage without allowing production full-state materialization.
- **Fix:** Added test-only oracle modules and internal regression cases covering patch helpers, pressure branches, and complete snapshots.
- **Files modified:** `packages/open-bitcoin-mempool/src/pool/oracle.rs`, `packages/open-bitcoin-mempool/src/pool/tests/oracle_internal_cases.rs`, `packages/open-bitcoin-mempool/src/pool/tests/patch_internal_cases.rs`, `packages/open-bitcoin-mempool/src/pool/tests/pressure_internal_cases.rs`
- **Verification:** Mempool 201/201 unit tests and 5/5 public parity tests passed; targeted coverage reported no uncovered mempool lines.
- **Committed in:** `d7d8febd`

**4. [Rule 3 - Blocking] Split graph preparation out of the sparse patch module**

- **Found during:** Task 2 simplification pass
- **Issue:** The initial patch module exceeded the repository production-file guidance and mixed graph analysis with patch application.
- **Fix:** Moved prospective topology, aggregates, limits, closure, and eviction selection into `patch/graph.rs`; `patch.rs` retains sparse layout, resource deltas, and application.
- **Files modified:** `packages/open-bitcoin-mempool/src/pool/patch.rs`, `packages/open-bitcoin-mempool/src/pool/patch/graph.rs`
- **Verification:** Production file-length check passed with `patch.rs` at 248 lines and `patch/graph.rs` at 287 lines before generated breadcrumb refresh.
- **Committed in:** `d7d8febd`

**Total deviations:** 4 auto-fixed (2 missing critical, 2 blocking)

**Impact on plan:** Each change was required for correctness, deterministic evidence, downstream compilation, or repository code-shape compliance. The public behavior and planned architecture boundary did not expand.

## Issues Encountered

- The first Task 2 commit attempt was stopped by the normal hook because four generated in-file parity breadcrumb blocks were stale after the module split. Running `bun run scripts/check-parity-breadcrumbs.ts --write` refreshed the blocks; the checker then verified all 420 Rust files.
- The successful commit hook took 1h 2m 6s because several deterministic phase fixtures and Rust test harnesses had long startup intervals. The verifier emitted liveness markers and captured advisory stall evidence automatically; no process was interrupted and every gate passed.

## Verification

- `bun run scripts/command-timings.ts run --key phase132-revision-guard -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-mempool revision_cases`
- `bun run scripts/command-timings.ts run --key phase132-single-regression -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-mempool --lib`
- Targeted `cargo llvm-cov -p open-bitcoin-mempool --show-missing-lines --text` with no uncovered-line report.
- `bash scripts/verify.sh` passed before commit in 14m 7s.
- The normal Task 2 pre-commit hook reran `bash scripts/verify.sh` and passed in 1h 2m 6s, including format, Clippy, all-target build, full tests/doctests, benchmark smoke, Bazel, and 100% coverage.

## Authentication Gates

None.

## Known Stubs

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Package orchestration can reuse `prepare_candidate` and defer `check_candidate_scripts` until its D-13 late-script stage.
- Sparse revision-bound application is available to reject stale package work without exposing revision counters publicly.
- No blockers or manual setup remain for the next Phase 132 plan.

## Self-Check: PASSED

- Summary file exists and its lifecycle/requirement-addressed metadata matches the originating plan.
- Task commits `f505cc78` and `d7d8febd` exist in repository history.
- Summary diff is whitespace-clean, and the parent-owned `STATE.md` and `ROADMAP.md` remain unstaged.
