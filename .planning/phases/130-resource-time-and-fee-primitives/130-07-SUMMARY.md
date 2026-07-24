---
phase: 130-resource-time-and-fee-primitives
plan: "07"
subsystem: node-lifecycle
tags: [rust, mempool, block-lifecycle, reorg, runtime-authority]
requires:
  - phase: 130-resource-time-and-fee-primitives
    provides: Canonical metadata, explicit contexts, and committed lifecycle deltas from Plans 130-03 through 130-06
provides:
  - Explicit block connection time and height through the pure mempool transition
  - Exact reorg event time, reorg origin, and non-requested relay intent on reaccepted entries
  - Delta-driven current cache cleanup preserving typed cause, role, identity, and final membership
  - Complete block and reorg caller migration through the sole runtime authority
affects: [phase-131, phase-134, block-lifecycle, reorg-recovery, runtime-authority]
tech-stack:
  added: []
  patterns:
    - Shell-supplied block and reorg contexts through ManagedNetworkHandle
    - Current cache effects consume semantic lifecycle deltas without outcome reclassification
key-files:
  created: []
  modified:
    - packages/open-bitcoin-mempool/src/context.rs
    - packages/open-bitcoin-mempool/src/pool/lifecycle.rs
    - packages/open-bitcoin-node/src/network/mempool_lifecycle.rs
    - packages/open-bitcoin-node/src/network/runtime_authority.rs
    - packages/open-bitcoin-node/src/network/tests/mempool_lifecycle_cases.rs
    - packages/open-bitcoin-node/src/sync/block_reconcile.rs
key-decisions:
  - "Use the stored-block receive timestamp and connected height for block lifecycle, while direct local blocks use explicit header time and connected height."
  - "Use one reorg operation time for replacement-branch cleanup and every disconnected transaction reacceptance."
  - "Apply every reorg admission transition through the current delta bridge, including noncommitting attempts."
  - "Keep complete cross-cache lifecycle projection deferred to Phase 134."
patterns-established:
  - "Block and reorg lifecycle APIs require typed immutable contexts; no no-time block or reorg adapter remains."
  - "Reorg metadata is always Known(event time), Reorg, and NotRequested."
requirements-completed: []
requirements-addressed: [FEEP-03, FEEP-04, FEEP-05]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 130-2026-07-23T14-26-46
generated_at: 2026-07-24T01:09:15Z
duration: 47 min
completed: 2026-07-24
---

# Phase 130 Plan 07: Explicit Block and Reorg Lifecycle Integration Summary

**Block and reorg transitions now carry exact typed event facts through the sole runtime authority and project current cache cleanup from committed lifecycle deltas**

## Performance

- **Duration:** 47 min
- **Started:** 2026-07-24T00:22:34Z
- **Completed:** 2026-07-24T01:09:15Z
- **Tasks:** 2
- **Files modified:** 16

## Accomplishments

- Routed stored-block timestamp/height and direct-block header-time/height into the pure block lifecycle transition.
- Removed no-context block transition adapters and migrated all library, integration-test, benchmark, binary, and doctest targets.
- Preserved block confirmation versus conflict causes and direct versus descendant roles while current txid, wtxid, serving, fanout, and compact-partial cleanup consumes delta identities and final membership.
- Forwarded the existing sync reconciliation timestamp unchanged through `ManagedNetworkHandle` into replacement-block cleanup and disconnected transaction reacceptance.
- Stored exact `Known(80)`, `Reorg`, and `NotRequested` metadata in the managed reorg regression without adding local relay intent.
- Applied every reorg attempt through `MempoolTransition` and its semantic delta while retaining ordered typed attempt outcomes.
- Passed focused block/reorg suites, the exact timed workspace all-target gate at both task boundaries, full normal-hook verification, and final plan verification.

## TDD Execution

- **Task 1 RED:** The managed block regression failed because the bridge accepted no `BlockLifecycleContext` and returned the deprecated summary projection.
- **Task 1 GREEN:** The bridge and pure transition accepted explicit context, returned typed delta facts, cleaned current caches from final-absent removals, and removed no-context block APIs.
- **Task 2 RED:** The managed reorg regression failed because `reorg_to_branch` accepted no `ReorgLifecycleContext`.
- **Task 2 GREEN:** Sync forwarded exact event time, reaccepted entries stored reorg metadata, replacement blocks shared the same operation time, and every attempt consumed its delta.
- **REFACTOR:** Block/reorg orchestration moved into `network/mempool_lifecycle.rs` to preserve the production file-length contract.

## Task Commits

1. **Task 1: Apply block lifecycle through explicit context and delta** - `6cc5c51c` (feat)
2. **Task 2: Reaccept disconnected transactions with explicit reorg facts** - `5960e7f4` (feat)

## Files Created/Modified

- `packages/open-bitcoin-mempool/src/context.rs` - Trusted reorg admission constructor.
- `packages/open-bitcoin-mempool/src/pool/lifecycle.rs` - Context-required block transition and removed no-time adapters.
- `packages/open-bitcoin-mempool/src/pool/tests/context_cases.rs` - Reorg constructor metadata coverage.
- `packages/open-bitcoin-mempool/src/pool/tests/lifecycle_cases.rs` - Explicit block context migration.
- `packages/open-bitcoin-mempool/src/pool/tests/lifecycle_delta_cases.rs` - Typed block delta context migration.
- `packages/open-bitcoin-mempool/src/pool/tests/resource_cases.rs` - Resource-oracle block transition migration.
- `packages/open-bitcoin-mempool/tests/parity.rs` - Public explicit block transition coverage.
- `packages/open-bitcoin-node/src/network.rs` - Thin network root after lifecycle extraction.
- `packages/open-bitcoin-node/src/network/mempool_lifecycle.rs` - Block/reorg context orchestration and delta-driven current cleanup.
- `packages/open-bitcoin-node/src/network/runtime_authority.rs` - Typed reorg context through the sole authority.
- `packages/open-bitcoin-node/src/network/tests/mempool_lifecycle_cases.rs` - Exact context, cause/role, cache, and reorg metadata regressions.
- `packages/open-bitcoin-node/src/sync/block_reconcile.rs` - Exact reconciliation timestamp handoff.
- `packages/open-bitcoin-node/src/sync/tests.rs` - Explicit sync reorg context regression.
- `scripts/check-phase103-mempool-lifecycle.ts` - Historical lifecycle guard migrated to the stronger reorg test.
- `scripts/check-phase103-mempool-lifecycle.test.ts` - Mutation coverage for the migrated guard.
- `docs/metrics/lines-of-code.md` - Hook-managed LOC freshness.

## Decisions Made

- Stored blocks use the existing receive/reconciliation timestamp rather than block-header time; direct local blocks use explicit header time because no outer receive time exists.
- A genuine reorg uses one immutable `ReorgLifecycleContext` for all replacement-block cleanup and disconnected transaction attempts.
- Reorg reacceptance never infers local origin or requested relay intent.
- Complete persistence, retry, serving, fanout, peer, compact, and evidence projection remains owned by Phase 134.

## ASVS Mitigations

- **ASVS-130-V1/V13:** Inventoried every changed block/reorg public caller and passed the exact timed `cargo check --workspace --all-targets` gate after each task.
- **ASVS-130-V5/V11:** Required closed typed contexts, removed no-time block/reorg entry points, and asserted exact time, height, cause, role, and metadata.
- **ASVS-130-V8:** Reorg entries remain `Reorg`/`NotRequested`; no local or requested relay fact is fabricated.
- **ASVS-130-V10 boundary:** Added no dependency, dynamic loading, scheduler, transport, or executable-content path.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Migrated omitted pure-core block transition callers**
- **Found during:** Task 1 all-target caller audit
- **Issue:** The context-required transition removed no-time public methods still used by mempool unit, resource, parity, and lifecycle tests omitted from the task file list.
- **Fix:** Migrated every caller to a concrete block plus `BlockLifecycleContext` and made the transaction-only helper private.
- **Files modified:** `packages/open-bitcoin-mempool/src/pool/lifecycle.rs`, `packages/open-bitcoin-mempool/src/pool/tests/lifecycle_cases.rs`, `packages/open-bitcoin-mempool/src/pool/tests/lifecycle_delta_cases.rs`, `packages/open-bitcoin-mempool/src/pool/tests/resource_cases.rs`, `packages/open-bitcoin-mempool/tests/parity.rs`
- **Verification:** Exact timed workspace all-target check passed.
- **Committed in:** `6cc5c51c`

**2. [Rule 3 - Blocking] Extracted block/reorg orchestration below the production file limit**
- **Found during:** Task 1 normal-hook verification
- **Issue:** Context construction pushed `network.rs` ten lines above the enforced 628-line limit.
- **Fix:** Moved the coherent block-connect, stored-block, and reorg methods into the existing `network/mempool_lifecycle.rs` child module.
- **Files modified:** `packages/open-bitcoin-node/src/network.rs`, `packages/open-bitcoin-node/src/network/mempool_lifecycle.rs`
- **Verification:** The 298-file production Rust length check and full repository verifier passed.
- **Committed in:** `6cc5c51c`

**3. [Rule 3 - Blocking] Migrated the historical Phase 103 lifecycle guard**
- **Found during:** Task 2 normal-hook verification
- **Issue:** The guard required the superseded reorg test name and rejected the stronger explicit-time regression.
- **Fix:** Required the new behavior test and preserved its missing-test mutation.
- **Files modified:** `scripts/check-phase103-mempool-lifecycle.ts`, `scripts/check-phase103-mempool-lifecycle.test.ts`
- **Verification:** Focused guard tests passed 8/8, the live guard passed, and the full verifier passed.
- **Committed in:** `5960e7f4`

**4. [Rule 3 - Blocking] Added direct reorg constructor coverage**
- **Found during:** Task 2 normal-hook coverage gate
- **Issue:** Const inlining left the new pure constructor lines uncovered despite the managed integration regression.
- **Fix:** Extended the focused constructor test to assert exact known time, reorg origin, and non-requested relay intent.
- **Files modified:** `packages/open-bitcoin-mempool/src/pool/tests/context_cases.rs`
- **Verification:** Focused constructor test, exact timed workspace all-target check, zero-uncovered-line gate, and full verifier passed.
- **Committed in:** `5960e7f4`

---

**Total deviations:** 4 auto-fixed (4 blocking)
**Impact on plan:** All changes were required for the explicit public API migration and repository contracts; no Phase 131 fee behavior or Phase 134 cross-cache expansion was added.

## Issues Encountered

- Normal hooks exposed the file-length, historical-guard, and direct-coverage blockers. Each was fixed and the commits were retried without bypassing hooks.
- FEEP-03, FEEP-04, and FEEP-05 remain addressed rather than prematurely completed until Phase 130 receives lifecycle-valid verification.

## Authentication Gates

None.

## Known Stubs

None.

## Threat Flags

None - the change strengthens typed in-memory lifecycle boundaries and introduces no network endpoint, authentication path, file-access pattern, durable schema, dependency, scheduler, or transport behavior.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan 130-08 can preserve known metadata through snapshot compatibility.
- Plan 130-11 can remove the remaining no-time local admission compatibility boundary independently.
- Phase 131 retains rolling-fee and pressure behavior; Phase 134 retains complete cross-cache projection.
- No blockers remain.

## Self-Check: PASSED

- Summary and all key modified files exist.
- Task commits `6cc5c51c` and `5960e7f4` exist.
- Lifecycle ID, yolo mode, addressed requirements, verification claims, and changed-file metrics match the committed work.
- Stub and threat-surface scans found no blocking placeholder or new security-relevant surface.

---
*Phase: 130-resource-time-and-fee-primitives*
*Completed: 2026-07-24*
