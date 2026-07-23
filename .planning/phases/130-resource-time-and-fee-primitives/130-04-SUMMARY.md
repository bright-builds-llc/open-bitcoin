---
phase: 130-resource-time-and-fee-primitives
plan: "04"
subsystem: mempool
tags: [rust, mempool, lifecycle, deterministic-deltas, typed-invariants]
requires:
  - phase: 130-resource-time-and-fee-primitives
    provides: Typed resources, fee roles, canonical metadata, and explicit contexts from Plans 130-01 through 130-03
provides:
  - Cache-agnostic committed lifecycle deltas separate from admission-attempt outcomes
  - Independent typed removal causes and direct-versus-descendant roles
  - Deterministic identity ordering, deduplication, final membership, and retry-clear precedence
  - Transition-returning admission and block cleanup paths with named compatibility owners
affects: [phase-131, phase-132, phase-134, phase-136, mempool-policy, node-lifecycle]
tech-stack:
  added: []
  patterns:
    - Checked lifecycle-delta builder with typed contradiction errors
    - Attempt-versus-commit separation through MempoolTransition
key-files:
  created:
    - packages/open-bitcoin-mempool/src/pool/tests/lifecycle_delta_cases.rs
  modified:
    - packages/open-bitcoin-mempool/src/pool/lifecycle.rs
    - packages/open-bitcoin-mempool/src/pool/admission.rs
    - packages/open-bitcoin-mempool/src/pool/admission_outcome.rs
    - packages/open-bitcoin-mempool/src/pool.rs
    - packages/open-bitcoin-node/src/network/mempool_lifecycle.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - "Keep validation and admission-attempt vocabulary in MempoolOutcome while returning committed facts separately in MempoolTransition."
  - "Resolve retry-clear evidence with fixed LifecycleRemoval > TransportWritten > EligibleServe precedence and exactly one fact per identity."
  - "Model removal cause independently from direct-versus-descendant role, with deterministic cause precedence and direct-role upgrades."
patterns-established:
  - "Every affected lifecycle identity has one deterministic final membership fact, and conflicting txid/wtxid pairs fail closed."
  - "Deprecated compatibility projections derive only from semantic transitions and name their later migration or removal owners."
requirements-completed: []
requirements-addressed: [FEEP-05]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 130-2026-07-23T14-26-46
generated_at: 2026-07-23T21:51:00Z
duration: 1h 23m
completed: 2026-07-23
---

# Phase 130 Plan 04: Deterministic Committed Lifecycle Deltas Summary

**Cache-agnostic committed deltas now preserve typed removal cause and role, final membership, retry-clear precedence, and attempt-versus-commit separation**

## Performance

- **Duration:** 1h 23m
- **Started:** 2026-07-23T20:27:41Z
- **Completed:** 2026-07-23T21:51:00Z
- **Tasks:** 2
- **Files modified:** 17

## Accomplishments

- Added a checked `MempoolLifecycleDelta` builder with deterministic admitted ordering, identity-sorted removals/final state/retry clears, and typed identity-conflict rejection.
- Split replacement, expiry, pressure, block confirmation, block conflict, and reorg causes from direct and descendant roles.
- Enforced exactly one retry-clear fact per identity with `LifecycleRemoval > TransportWritten > EligibleServe` precedence in both insertion orders.
- Added transition-returning admission and block-cleanup APIs that produce committed facts from prospective and committed state rather than reclassifying outcome vectors.
- Proved admission, replacement, pressure descendant removal, block confirmation/conflict, and empty noncommitting attempts through 16 discoverable lifecycle-delta tests.
- Migrated parity and in-crate callers while retaining only deprecated transition-derived node and recovery projections with named later owners.

## Task Commits

1. **Task 1: Define lifecycle facts, labels, and invariants** - `6a7feddf` (feat)
2. **Task 2: Produce lifecycle deltas from committed mempool transitions** - `7669b758` (feat)

Both TDD tasks were proven red before implementation. Their completed green behavior was committed atomically after the exact timed all-target workspace gate and repository hooks passed.

## Files Created/Modified

- `packages/open-bitcoin-mempool/src/pool/lifecycle.rs` - Lifecycle facts, labels, checked builder, compatibility summaries, and block transition production.
- `packages/open-bitcoin-mempool/src/pool/admission.rs` - Prospective-state lifecycle assembly and transition-returning admission entry point.
- `packages/open-bitcoin-mempool/src/pool/admission_outcome.rs` - Attempt outcome plus committed delta construction without outcome reclassification.
- `packages/open-bitcoin-mempool/src/pool.rs` - Public `MempoolTransition` and role-aware legacy trim identities.
- `packages/open-bitcoin-mempool/src/pool/tests/lifecycle_delta_cases.rs` - Ordering, deduplication, precedence, identity conflict, admission, replacement, pressure, block, and empty-attempt regressions.
- `packages/open-bitcoin-mempool/src/pool/tests/lifecycle_cases.rs` - Cause/role-aware compatibility summary assertions.
- `packages/open-bitcoin-mempool/tests/parity.rs` - Public lifecycle contract migration and parity assertions.
- `packages/open-bitcoin-node/src/network/mempool_lifecycle.rs` - Explicit named compatibility boundary pending Plan 130-07.
- `packages/open-bitcoin-node/src/storage/mempool_snapshot.rs` - Explicit named recovery compatibility boundary pending Plans 130-08/130-11.
- `scripts/check-phase103-mempool-lifecycle.ts` - Updated historical guard for the new cause/role symbols.
- `docs/parity/source-breadcrumbs.json` - Lifecycle-delta test and pinned removal-reason source registration.
- `docs/metrics/lines-of-code.md` - Hook-managed LOC freshness.

## Decisions Made

- `MempoolOutcome` remains attempt vocabulary; `MempoolLifecycleDelta` records only committed facts.
- Removal cause precedence is deterministic, while a direct role independently upgrades a duplicate descendant role.
- Legacy vsize trimming emits truthful `Pressure` facts without changing `RollingMempoolFeeRate` or pulling Phase 131 enforcement forward.
- Complete serving, fanout, persistence, compact-reconstruction, RPC, metric, and log projection remains deferred to Phase 134.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Migrated the historical Phase 103 lifecycle guard to cause/role vocabulary**
- **Found during:** Task 1 verified commit
- **Issue:** The default verifier still required the removed `MempoolLifecycleRemovalReason` symbol and rejected the planned cause/role split.
- **Fix:** Updated the checker to require both `MempoolRemovalCause` and `MempoolRemovalRole`, and expanded its mutation test to prove each symbol remains mandatory.
- **Files modified:** `scripts/check-phase103-mempool-lifecycle.ts`, `scripts/check-phase103-mempool-lifecycle.test.ts`
- **Verification:** The focused Bun checker suite passed 8/8 tests, and both task commits passed the full repository hook.
- **Committed in:** `6a7feddf`

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** The fix preserved an existing historical guard across the required public API migration; no deferred pressure policy or cross-cache projection was added.

## Issues Encountered

- Initial verified Task 1 commits exposed a stale lifecycle breadcrumb header, the historical Phase 103 symbol guard, a large typed error variant, and uncovered deterministic branches. Breadcrumbs, guard tests, error shape, and focused coverage were corrected before the successful normal commit.
- Task 2's verified commit exposed `-D deprecated` failures in the explicitly retained node block and snapshot compatibility callers. Narrow owner-labeled allowances now preserve the compile-safe intermediate boundary until Plans 130-07, 130-08, and 130-11 migrate them.
- One exact focused pressure command initially matched zero tests; the corrected fully qualified name executed one test, and final named discovery proved all 16 lifecycle-delta tests are compiled and nonzero.
- The metadata hook correctly rejected early FEEP-05 completion because Phase 130 has no lifecycle-valid `VERIFICATION.md` yet. FEEP-05 remains addressed and pending until phase verification.

## Authentication Gates

None.

## Known Stubs

None.

## Threat Flags

None - the change adds pure typed lifecycle contracts and no network endpoint, authentication path, file-access pattern, or persistence schema.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan 130-05 can migrate managed peer/local admission to consume `MempoolTransition::delta` directly.
- Plan 130-07 has explicit block-lifecycle compatibility seams to replace with context-aware delta consumption.
- Phase 134 retains complete cross-cache projection ownership; no cache-specific effects were added here.
- No blockers remain.

## Self-Check: PASSED

- Summary and lifecycle-delta test file exist.
- Task commits `6a7feddf` and `7669b758` exist.
- Sixteen named lifecycle-delta tests are discoverable.
- Lifecycle ID, yolo mode, FEEP-05, and verification claims match the committed plan work.

---
*Phase: 130-resource-time-and-fee-primitives*
*Completed: 2026-07-23*
