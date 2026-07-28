---
phase: 134-authoritative-cross-cache-lifecycle-integration
plan: "05"
subsystem: node-network-lifecycle
tags: [rust, lifecycle-authority, reconciliation, bounded-evidence, transaction-relay]

requires:
  - phase: 134-authoritative-cross-cache-lifecycle-integration
    plan: "04"
    provides: prepared infallible compact, serving, fanout, and peer lifecycle target operations
provides:
  - sole mutex-guarded lifecycle command dispatcher with authority-epoch and core-revision validation
  - ordered all-target lifecycle application with aggregate generations, unbroadcast membership, persistence, and evidence
  - bounded seven-target reconciliation with production-safe labels/counts and test-only exact identities
affects: [phase-134-production-routing, transaction-relay, compact-download, lifecycle-verification]

tech-stack:
  added: []
  patterns:
    - validate authority epoch and canonical revision before consuming a private lifecycle capability
    - apply the canonical transition and every dependent projection in one fixed infallible order
    - expose bounded audit reconciliation without admitting repair or normal-path invocation

key-files:
  created:
    - packages/open-bitcoin-node/src/network/lifecycle_projection/authority.rs
    - packages/open-bitcoin-node/src/network/lifecycle_projection/reconciliation.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/lifecycle.rs
    - packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases.rs
    - packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/reconciliation.rs
    - packages/open-bitcoin-network/src/peer/transaction_lifecycle/reconciliation.rs
  modified:
    - packages/open-bitcoin-node/src/network/lifecycle_projection.rs
    - packages/open-bitcoin-node/src/network/compact_receive_candidates.rs
    - packages/open-bitcoin-node/src/network/relay_serving.rs
    - packages/open-bitcoin-network/src/peer/transaction_lifecycle.rs
    - scripts/check-phase134-apply-boundaries.ts
    - docs/parity/source-breadcrumbs.json

key-decisions:
  - "Make the validated lifecycle capability private and consumable so stale authority epoch or canonical revision fails before any target mutation."
  - "Apply canonical core, compact, serving, fanout, peer, unbroadcast, persistence, and evidence projections in one exact order under the sole authority lock."
  - "Keep reconciliation read-only and bounded: production exposes only seven fixed labels and capped counts, while exact identities remain test-only."

patterns-established:
  - "Authority dispatch: one private handle closure validates once and consumes one complete projection capability."
  - "Audit oracle: fixed-cardinality target summaries compare canonical truth to every dependent cache without repairing state."

requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 134-2026-07-28T01-41-12
generated_at: 2026-07-28T10:32:17Z

duration: 1h 41m
completed: 2026-07-28
---

# Phase 134 Plan 05: Authoritative Lifecycle Dispatcher and Reconciliation Summary

**A single epoch-validated lifecycle dispatcher now commits canonical and dependent transaction state atomically, with a bounded seven-target audit oracle that cannot leak identities in production.**

## Performance

- **Duration:** 1h 41m
- **Started:** 2026-07-28T08:51:08Z
- **Completed:** 2026-07-28T10:32:17Z
- **Tasks:** 2
- **Files modified:** 25

## Accomplishments

- Moved authority epoch, lifecycle and dirty generations, bounded unbroadcast membership, persistence projection, and low-cardinality evidence under `ManagedPeerNetwork`.
- Added the sole private `apply_lifecycle_command` closure, separating fallible epoch/revision validation from an infallible aggregate apply in the required core, compact, serving, fanout, peer, unbroadcast, persistence, and evidence order.
- Proved stale epoch and revision rejection is mutation-free, non-empty deltas advance generations exactly once, empty deltas do not advance them, and unbroadcast eligibility follows final authoritative membership plus admission source.
- Added complete expected-projection fixtures and deliberate divergence coverage across serving, fanout, peer, compact, unbroadcast, persistence, and evidence targets.
- Added bounded production reconciliation reports containing only fixed labels and capped counts; exact mismatching identities exist only in test builds, and normal lifecycle apply never calls reconciliation.

## Task Commits

Each task was committed atomically:

1. **Task 1: Implement aggregate ownership and sole command application** - `3492acdb`
2. **Task 2: Add bounded reconciliation and complete assertion fixtures** - `a5e8e13a`

## Files Created/Modified

- `packages/open-bitcoin-node/src/network/lifecycle_projection/authority.rs` - Owns validation, generation, unbroadcast, persistence, evidence, and ordered aggregate application.
- `packages/open-bitcoin-node/src/network/runtime_authority/lifecycle.rs` - Implements the sole private handle-level lifecycle command dispatcher.
- `packages/open-bitcoin-node/src/network/lifecycle_projection/reconciliation.rs` - Produces bounded fixed-target audit summaries.
- `packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases.rs` and `lifecycle_projection_cases/reconciliation.rs` - Define complete projection expectations and authority/reconciliation behavior coverage.
- `packages/open-bitcoin-network/src/peer/transaction_lifecycle/reconciliation.rs` - Exposes bounded peer-core lifecycle snapshot and mismatch accounting.
- `packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs` - Accounts for lifecycle scheduler divergence.
- `packages/open-bitcoin-node/src/network/relay_serving.rs`, `relay_fanout/reconciliation.rs`, and `compact_receive_candidates.rs` - Supply exact read-only target membership comparisons.
- `scripts/check-phase134-apply-boundaries.ts` - Extends exact infallible target-body enforcement to unbroadcast, persistence, and evidence.
- `docs/parity/source-breadcrumbs.json` - Registers the new first-party Rust sources against the pinned Knots anchors.
- `docs/metrics/lines-of-code.md` - Refreshed by the mandatory repository hook.

## Decisions Made

- The private `ValidatedLifecycleProjection` is the only input accepted by aggregate application, preventing callers from bypassing authority epoch and mempool revision validation.
- Compact application remains before serving retirement so a replacement transaction body cannot lose its final retrievable copy.
- Reconciliation is observation-only: it accepts bounded caps, reports exactly seven stable target labels, and has no admission, pressure, expiry, block, reorg, aggregate-apply, or effect-completion call site.
- Peer reconciliation lives beside the peer lifecycle authority so node-level audits consume bounded domain facts instead of inspecting peer internals.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - Missing Critical] Carried admission source metadata into authoritative membership**

- **Found during:** Task 1
- **Issue:** Correct unbroadcast eligibility requires distinguishing Local and Requested admissions after canonical commit, but prepared final members did not retain their source.
- **Fix:** Added the validated admission source to prepared mempool members and used it with final authoritative membership to admit or clear unbroadcast entries.
- **Files modified:** `packages/open-bitcoin-mempool/src/pool/prepared_lifecycle.rs`, `packages/open-bitcoin-node/src/network/lifecycle_projection/authority.rs`
- **Verification:** Authority lifecycle tests prove Local/Requested admission and exact identity clearing; all mandatory Rust gates passed.
- **Committed in:** `3492acdb`

**2. [Rule 1 - Bug] Cleared stale compact extras for final-present transactions**

- **Found during:** Task 2 reconciliation coverage
- **Issue:** A newly admitted final-present transaction could retain a stale compact-extra copy, making compact reconciliation report an unnecessary duplicate.
- **Fix:** Remove the exact compact-extra entry when applying a final-present compact admission while preserving replacement retention ordering.
- **Files modified:** `packages/open-bitcoin-node/src/network/compact_receive_candidates.rs`
- **Verification:** The final-present compact regression test and complete seven-target reconciliation suite pass.
- **Committed in:** `a5e8e13a`

**3. [Rule 2 - Missing Critical] Added bounded peer-core reconciliation facts**

- **Found during:** Task 2
- **Issue:** A complete peer-target audit could not detect orphan, candidate, package, known-transaction, request, and scheduler divergence through the existing public capability.
- **Fix:** Added a bounded peer lifecycle snapshot and mismatch counter, including scheduler mismatch accounting, and composed them into the node's fixed peer target report.
- **Files modified:** `packages/open-bitcoin-network/src/peer/transaction_lifecycle.rs`, `packages/open-bitcoin-network/src/peer/transaction_lifecycle/reconciliation.rs`, `packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs`
- **Verification:** Clean and deliberately divergent peer-core audit tests pass, and targeted LLVM coverage reports no uncovered added lines.
- **Committed in:** `a5e8e13a`

**4. [Rule 3 - Blocking] Split reconciliation sources and tests to satisfy enforced code shape**

- **Found during:** Task 2 commit verification
- **Issue:** Complete reconciliation logic and coverage pushed peer and fanout modules beyond the repository's strict production-file limit.
- **Fix:** Extracted focused peer, fanout, and lifecycle reconciliation modules plus the peer reconciliation test module, then registered each new source breadcrumb.
- **Files modified:** `packages/open-bitcoin-network/src/peer/transaction_lifecycle/reconciliation.rs`, `packages/open-bitcoin-network/src/peer/tests/transaction_lifecycle_cases/reconciliation.rs`, `packages/open-bitcoin-node/src/network/relay_fanout/reconciliation.rs`, `packages/open-bitcoin-node/src/network/lifecycle_projection/reconciliation.rs`, `docs/parity/source-breadcrumbs.json`
- **Verification:** Bright Builds code-shape checks, parity breadcrumbs, the complete Rust gates, and the normal full repository hook passed.
- **Committed in:** `a5e8e13a`

**5. [Rule 3 - Blocking] Deferred requirement activation to lifecycle verification**

- **Found during:** Plan metadata preparation
- **Issue:** Intermediate Phase 134 summaries cannot activate `MPLIFE-01`, `MPLIFE-02`, or `MPLIFE-03` before the phase has a lifecycle-valid verification artifact.
- **Fix:** Preserved the requirements as pending and left `requirements-completed` empty, matching the active Phase 134 traceability contract.
- **Files modified:** `.planning/phases/134-authoritative-cross-cache-lifecycle-integration/134-05-SUMMARY.md`
- **Verification:** The repository's active milestone traceability checker passes with requirements pending.
- **Committed in:** Plan metadata commit

**Total deviations:** 5 auto-fixed (2 missing critical, 1 bug, 2 blocking)
**Impact on plan:** All adjustments were necessary for correct final-state ownership, complete bounded auditing, and repository policy compliance; no new runtime surface or architectural scope was added.

## Issues Encountered

- The first Task 2 hook identified a missing exact breadcrumb for the extracted fanout reconciliation module; regenerating and verifying the registry resolved it.
- The next hook reached final pure-core LLVM coverage and exposed unexecuted peer reconciliation and scheduler branches. Focused peer-core audit fixtures covered those paths; one initial single-child candidate fixture was corrected to the required two-child reconsideration case before the complete test suite and exact coverage check passed.
- Bright Builds then identified the peer lifecycle test file above its strict length limit. Moving the focused audit tests into a registered child module resolved the policy failure.

## Known Stubs

- Lifecycle effect command variants remain typed dispatcher arms reserved for Plan 08, exactly as required by this plan. They do not create a second mutable authority path and do not block the Plan 05 goal.
- Empty arrays and nullable values found by the stub scan are local parser accumulators in `scripts/check-phase134-apply-boundaries.ts`, not runtime or user-visible placeholders.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plans 06 and 07 can route production lifecycle callers through the sole validated dispatcher without reopening target mutation APIs.
- Plan 08 can implement the retained effect command arms inside the existing authority boundary.
- The bounded audit oracle is ready for startup and verification use without becoming a cache-repair mechanism.
- `MPLIFE-01`, `MPLIFE-02`, and `MPLIFE-03` remain pending until Phase 134 verification produces the lifecycle-valid completion artifact.
- No blockers remain.

## Self-Check: PASSED

- Summary and all six key created files exist.
- Task commits `3492acdb` and `a5e8e13a` are present in repository history.
- Stub and threat-surface scans found no goal-blocking placeholder or unplanned trust-boundary surface.
- `git diff --check` passes and Markdown frontmatter uses exactly two delimiters.

***

*Phase: 134-authoritative-cross-cache-lifecycle-integration*
*Completed: 2026-07-28*
