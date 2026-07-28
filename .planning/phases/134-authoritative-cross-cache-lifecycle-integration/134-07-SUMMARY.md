---
phase: 134-authoritative-cross-cache-lifecycle-integration
plan: "07"
subsystem: node-network-lifecycle
tags: [rust, lifecycle-authority, maintenance, reorg, reconciliation]

requires:
  - phase: 134-authoritative-cross-cache-lifecycle-integration
    plan: "06"
    provides: authoritative singleton and package admission through the sole lifecycle dispatcher
provides:
  - expiry and connected-block maintenance routed through complete lifecycle projection
  - sequential per-step reorg removal and reconsideration commands
  - generation and seven-target reconciliation coverage for legacy lifecycle behavior
affects: [phase-134-production-routing, mempool-maintenance, reorg-composition, lifecycle-verification]

tech-stack:
  added: []
  patterns:
    - prepare each maintenance transition once and dispatch one exact typed lifecycle command
    - apply reorg steps sequentially so every preparation observes prior committed membership
    - preserve rejected reorg reconsiderations as pure no-op outward outcomes

key-files:
  created:
    - packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/maintenance.rs
  modified:
    - packages/open-bitcoin-node/src/network/mempool_lifecycle.rs
    - packages/open-bitcoin-node/src/network/tests/mempool_lifecycle_cases.rs
    - packages/open-bitcoin-node/src/network/tests/mempool_lifecycle_cases/reorg_reject_evidence.rs
    - docs/parity/source-breadcrumbs.json

key-decisions:
  - "Use the command variant as an explicit maintenance input: ConnectedBlock for ordinary connects, Expiry for expiry, and ReorgStep for every replacement block or disconnected transaction."
  - "Project reorg preparation failures into exact pure no-op outcomes instead of invoking a compatibility facade that could become a second mutation authority."
  - "Prove sequential composition with a disconnected parent and child in separate blocks so the child can succeed only after the parent step commits."

patterns-established:
  - "Maintenance facade: prepare immutable core facts, prepare all dependent projections, dispatch once, and return the committed typed delta."
  - "Reorg ordering: replacement blocks execute in branch order, then disconnected blocks execute oldest-first and transactions retain block order."

requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 134-2026-07-28T01-41-12
generated_at: 2026-07-28T13:09:00Z

duration: 1h 5m
completed: 2026-07-28
---

# Phase 134 Plan 07: Authoritative Maintenance and Reorg Lifecycle Summary

**Expiry, connected-block consequences, pressure-producing admission, and sequential reorg steps now share one complete lifecycle reducer that reconciles canonical membership across every dependent cache.**

## Performance

- **Duration:** 1h 5m
- **Started:** 2026-07-28T12:03:54Z
- **Completed:** 2026-07-28T13:09:00Z
- **Tasks:** 3
- **Files modified:** 10

## Accomplishments

- Replaced expiry and connected-block cache-specific mutation loops with prepared `Expiry` and `ConnectedBlock` commands through the sole lifecycle dispatcher.
- Preserved exact confirmation, conflict, expiry, pressure, descendant-first removal, rolling-fee, compact-victim, and outward outcome behavior while advancing generation once per non-empty transition.
- Routed every replacement block and successfully prepared disconnected transaction through an individual `ReorgStep`, removing the old reorg admission mutation plus duplicate projection path.
- Added a sequence-sensitive parent/child reorg fixture that checks canonical membership, all dependent targets, evidence, generation, and zero reconciliation mismatches after every step.
- Strengthened legacy maintenance and reorg cases with lifecycle/dirty generation and complete seven-target reconciliation assertions.

## Task Commits

Each task was committed atomically:

1. **Task 1: Route expiry, pressure, and connected-block changes** - `4256c254`
2. **Task 2: Preserve legacy maintenance behavior on the shared path** - `702f37bd`
3. **Task 3: Preserve sequential reorg composition** - `119bab0b`

## Files Created/Modified

- `packages/open-bitcoin-node/src/network/mempool_lifecycle.rs` - Prepares and dispatches typed expiry, connected-block, and sequential reorg lifecycle commands.
- `packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/maintenance.rs` - Covers exact expiry/conflict removal, empty maintenance, and step-sensitive reorg projection behavior.
- `packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases.rs` - Registers the maintenance scenario module.
- `packages/open-bitcoin-node/src/network/tests/mempool_lifecycle_cases.rs` - Provides shared authoritative generation and reconciliation assertions.
- `packages/open-bitcoin-node/src/network/tests/mempool_lifecycle_cases/compact_cache_and_fee.rs` - Preserves compact partial-slot and rolling-fee behavior on the shared path.
- `packages/open-bitcoin-node/src/network/tests/mempool_lifecycle_cases/connected_block_removal.rs` - Verifies confirmation/conflict cache cleanup and exact generation counts.
- `packages/open-bitcoin-node/src/network/tests/mempool_lifecycle_cases/expiry.rs` - Verifies expiry removal and reconciled generation advancement.
- `packages/open-bitcoin-node/src/network/tests/mempool_lifecycle_cases/reorg_reject_evidence.rs` - Preserves reorg timing, metadata, reject evidence, generation, and reconciliation.
- `docs/parity/source-breadcrumbs.json` - Registers the maintenance scenario against the pinned Knots lifecycle anchors.
- `docs/metrics/lines-of-code.md` - Refreshed by the mandatory normal repository hooks.

## Decisions Made

- Ordinary block connects retain the `ConnectedBlock` command identity, while replacement-branch blocks use `ReorgStep`; both share the same complete prepared-projection helper.
- Reorg preparation errors are converted into exact duplicate, orphaned, evicted, or rejected no-op outcomes without calling the mutable compatibility admission facade.
- Disconnected blocks are reconsidered oldest-first and their transactions remain in block order, ensuring parent state is committed before dependent child preparation.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Canonicalized the new test breadcrumb before the first task commit**

- **Found during:** Task 1 normal-hook verification
- **Issue:** The registry included the new maintenance file, but its inline breadcrumb block omitted two canonical anchors required by the repository hook.
- **Fix:** Ran the repository breadcrumb writer and committed its canonical four-anchor block; registration moved into Task 1 so the atomic commit could pass.
- **Files modified:** `packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/maintenance.rs`, `docs/parity/source-breadcrumbs.json`
- **Verification:** `bun scripts/check-parity-breadcrumbs.ts` and all three normal hooks passed.
- **Committed in:** `4256c254`

**2. [Rule 3 - Blocking] Deferred requirement activation to lifecycle verification**

- **Found during:** Plan metadata preparation
- **Issue:** Intermediate Phase 134 summaries cannot activate `MPLIFE-01`, `MPLIFE-02`, or `MPLIFE-03` before the phase has a lifecycle-valid verification artifact.
- **Fix:** Preserved all three requirements as pending and left `requirements-completed` empty, matching the active Phase 134 traceability contract.
- **Files modified:** `.planning/phases/134-authoritative-cross-cache-lifecycle-integration/134-07-SUMMARY.md`
- **Verification:** The full repository hooks and current documentation reconciliation remain green with requirements pending.
- **Committed in:** Plan metadata commit

**Total deviations:** 2 auto-fixed (2 blocking)
**Impact on plan:** Both adjustments were required by repository parity and traceability policy; no lifecycle scope, transport, scheduler, recovery, or public interface was added.

## Issues Encountered

- The Task 1 RED scenario exposed stale unbroadcast membership after direct expiry and connected-block mutations; complete projection dispatch removed the stale identity.
- The first sequence-sensitive reorg fixture placed parent and child in one three-transaction block, which the test merkle helper classified as mutated. Splitting them across consecutive blocks made the ordering dependency explicit and mirrors the production oldest-first reconsideration loop.
- TDD RED states were not committed separately because the mandatory normal hook requires a fully green repository; each task was committed atomically after its RED/GREEN cycle and all gates passed.

## Known Stubs

None. The modified production and test files contain no goal-blocking placeholder, empty runtime projection, TODO, or FIXME.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan 08 can build snapshot/recovery routing on a maintenance and admission surface that already shares the sole lifecycle authority.
- Generation/evidence and seven-target reconciliation fixtures now cover admission, expiry, pressure, connected blocks, and sequential reorg composition.
- `MPLIFE-01`, `MPLIFE-02`, and `MPLIFE-03` remain pending until Phase 134 verification produces the lifecycle-valid completion artifact.
- No blockers remain.

## Self-Check: PASSED

- Summary, maintenance scenario, and every modified lifecycle file exist.
- Task commits `4256c254`, `702f37bd`, and `119bab0b` are present in repository history.
- Focused maintenance, legacy lifecycle, and reorg suites pass; all mandatory Rust gates, Bright Builds checks, parity breadcrumbs, and all three full normal hooks pass.
- Stub and threat-surface scans found no goal-blocking placeholder or unplanned trust-boundary surface.
- `git diff --check` passes and Markdown frontmatter uses exactly two delimiters.

***

*Phase: 134-authoritative-cross-cache-lifecycle-integration*
*Completed: 2026-07-28*
