---
phase: 134-authoritative-cross-cache-lifecycle-integration
plan: "11"
subsystem: node-lifecycle-verification
tags: [rust, mempool, lifecycle-projection, reconciliation, failure-injection]

requires:
  - phase: 134-06
    provides: authoritative lifecycle projection plan and fixed-target reconciliation
  - phase: 134-07
    provides: ordered lifecycle consequences and exact projection application
  - phase: 134-09
    provides: receipt-bound peer effect completion
  - phase: 134-10
    provides: truthful outside-authority snapshot execution and completion
provides:
  - complete deterministic transition and effect scenario matrix across every lifecycle target
  - fixed-seed independent oracle for seven reconciliation targets
  - test-only preflight failure injection proving complete aggregate immutability
  - bounded identifier-free lifecycle evidence assertions
affects: [134-12, 134-13, phase-135]

tech-stack:
  added: []
  patterns:
    - expected projections assert canonical membership, all dependent targets, generation, evidence, and reconciliation together
    - independent std-only ordered collections model lifecycle operations without production helpers
    - scoped test-only guards inject failures strictly before authoritative mutation

key-files:
  created:
    - packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/effects/contracts.rs
    - packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/oracle.rs
  modified:
    - packages/open-bitcoin-node/src/network/lifecycle_projection.rs
    - packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases.rs
    - packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/admission.rs
    - packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/admission/partial_package.rs
    - packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/maintenance.rs
    - packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/effects.rs
    - packages/open-bitcoin-node/src/network/tests/announcement_transport_cases.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/tests/snapshot_persistence.rs
    - docs/parity/source-breadcrumbs.json
    - docs/metrics/lines-of-code.md

key-decisions:
  - "Treat prepared admitted and teardown order as the exact consequence-order contract while canonical delta membership remains unordered."
  - "Keep the independent model inside the oracle test module using only std ordered collections, then compare its target vectors with production reconciliation outside that module."
  - "Compile failure injection only under cfg(test), scope it per thread with a restoring guard, and invoke it only during preparation."
  - "Keep MPLIFE-02, MPLIFE-03, and MPLIFE-04 pending for phase-level verification."

patterns-established:
  - "Complete projection assertion: each scenario proves canonical, serving, fanout, peer, compact, unbroadcast, persistence generation, evidence, and clean reconciliation as one contract."
  - "Independent corruption oracle: mutate one target at a time and require an exact one-slot reconciliation vector."
  - "Failure atomicity: every injected preparation-stage failure compares the complete aggregate debug snapshot before and after."

requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 134-2026-07-28T01-41-12
generated_at: 2026-07-28T19:27:27Z

duration: 1h 34m
completed: 2026-07-28
---

# Phase 134 Plan 11: Lifecycle Matrix and Independent Oracle Summary

**Eleven deterministic lifecycle scenarios now prove exact cross-cache state, effect completion truth, generation invariants, and seven-target reconciliation through an independent fixed-seed oracle with preflight failure atomicity.**

## Performance

- **Duration:** 1h 34m
- **Started:** 2026-07-28T17:53:15Z
- **Completed:** 2026-07-28T19:27:27Z
- **Tasks:** 3
- **Files modified:** 12

## Accomplishments

- Completed the admission, package, replacement, pressure, expiry, connected-block, reorg, and failed-admission rows with exact prepared ordering plus complete projection and generation assertions.
- Completed stale, duplicate, partial-I/O, and snapshot-write failure coverage with exact `Applied`, `AchievedButStale`, and `AlreadyApplied` outcomes and no achieved credit for failed or unsent work.
- Added a bounded fixed-seed std-only model for admission, package, replacement, pressure, expiry, block, and reorg operations; each deliberate corruption produces the exact expected production reconciliation target.
- Added eleven `cfg(test)` preparation failure points and proved every injected error leaves the complete aggregate snapshot unchanged.
- Proved lifecycle evidence is fixed-size, bounded, and free of transaction, package, peer, path, and string identifiers.

## Task Commits

Each task was committed atomically:

1. **Task 1: Complete the eight lifecycle-transition matrix rows** - `7ef81ceb` (test)
2. **Task 2: Complete stale, duplicate, and partial-I/O matrix rows** - `0f6bbb86` (test)
3. **Task 3: Add independent generated reconciliation and failure injection** - `0ba1f885` (test)

## Files Created/Modified

- `packages/open-bitcoin-node/src/network/lifecycle_projection.rs` - Adds test-only, preparation-stage failure points with thread-scoped restoration.
- `packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases.rs` - Registers the oracle and centralizes complete projection assertions used across the matrix.
- `packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/admission.rs` - Strengthens exact full-package admission ordering and target assertions.
- `packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/admission/partial_package.rs` - Covers partial packages, replacement, pressure eviction, and failed admission with exact consequences.
- `packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/maintenance.rs` - Proves expiry, connected-block, and sequential reorg generation truth.
- `packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/effects.rs` - Covers exact current, stale, duplicate, and failed snapshot/peer completion behavior.
- `packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/effects/contracts.rs` - Holds bounded capability, receipt, and completed-ledger contract tests.
- `packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/oracle.rs` - Implements the independent generated model, exact target corruption vectors, failure injection, and evidence-schema assertions.
- `packages/open-bitcoin-node/src/network/tests/announcement_transport_cases.rs` - Proves only the successfully written prefix receives achieved-effect credit.
- `packages/open-bitcoin-node/src/storage/fjall_store/tests/snapshot_persistence.rs` - Proves write failure preserves pre-existing persisted state and returns no receipt.
- `docs/parity/source-breadcrumbs.json` - Registers both new Rust test modules under the lifecycle projection contract.
- `docs/metrics/lines-of-code.md` - Refreshes the intentionally tracked generated LOC evidence through normal hooks.

## Decisions Made

- Used prepared `admitted_order` and `teardown_order` as the exact ordering authority; unordered lifecycle delta membership remains a canonical fact set.
- Kept the independent model free of production preparation, application, and reconciliation helpers while separately requiring its target vectors to match production reconciliation output.
- Kept all failure hooks absent from production builds and confined them to preflight preparation, so normal apply paths remain reconciliation- and failure-hook-free.
- Left `MPLIFE-02`, `MPLIFE-03`, and `MPLIFE-04` pending for phase-level verification, as required.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Split lifecycle effect contract tests to satisfy the Bright Builds file-size guard**

- **Found during:** Task 2 mandatory Bright Builds verification
- **Issue:** Exact effect-completion coverage expanded `effects.rs` beyond the managed production/test file-length boundary.
- **Fix:** Extracted the existing capability, receipt, and bounded-ledger contract tests into `effects/contracts.rs` and registered the new path with canonical lifecycle breadcrumbs.
- **Files modified:** `packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/effects.rs`, `packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/effects/contracts.rs`, `docs/parity/source-breadcrumbs.json`
- **Verification:** Focused effect, announcement, and storage tests, all ordered Rust gates, Bright Builds checks, breadcrumb checks, and the normal full repository hook pass.
- **Committed in:** `0f6bbb86`

**Total deviations:** 1 auto-fixed blocking issue.
**Impact on plan:** The structural split preserved behavior, reduced file size, and added no runtime scope.

## Issues Encountered

- An initial Task 3 formatting invocation omitted the workspace manifest path and failed before changing files; the exact required command was rerun successfully.
- The first oracle compile exposed two test-fixture integer type mismatches, and Clippy identified one manual divisibility check. Both were corrected before the final ordered gate sequence.
- Cargo commands occasionally waited behind an active workspace check; processes were inspected and allowed to finish so all Cargo work remained serialized.

## Verification

- Final sequential lifecycle matrix: 44 tests passed with `--test-threads=1`.
- Final sequential independent oracle: 4 tests passed with `--test-threads=1`.
- Every task-specific ordered `cargo fmt`, Clippy with `-D warnings`, all-target build, and all-feature test gate passed.
- `bun scripts/bright-builds-check.ts all` passed for each task.
- Parity breadcrumbs verify 730 Rust files, and the Phase 134 apply-boundary checker confirms all seven target applies remain structurally infallible.
- All three normal task commit hooks passed the full `bash scripts/verify.sh` contract, including coverage and Bazel smoke builds.
- Final `git diff --check` and clean-tree checks passed before summary creation.

## Known Stubs

None.

## Threat Flags

None. The only new failure-control surface is compiled exclusively for tests; no network endpoint, authentication path, file-access boundary, or schema change was introduced.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan 12 can consume complete executable evidence for the authoritative lifecycle matrix and reconciliation oracle.
- Phase 135 persistence schema, recovery, cadence, and crash-window behavior remain outside this plan.
- `MPLIFE-02`, `MPLIFE-03`, and `MPLIFE-04` remain pending until phase-level verification reconciles the complete lifecycle surface.

## Self-Check: PASSED

All key files and task commits were found, the summary uses only the two frontmatter delimiters, and final plan-level verification passed.

*Phase: 134-authoritative-cross-cache-lifecycle-integration*
*Completed: 2026-07-28*
