---
phase: 130-resource-time-and-fee-primitives
plan: "02"
subsystem: mempool
tags: [rust, mempool, fee-policy, package-policy, typed-invariants]
requires:
  - phase: 130-resource-time-and-fee-primitives
    provides: Typed transaction virtual size and resource-accounting primitives from Plan 130-01
provides:
  - Compile-time-distinct static, incremental, rolling, and effective fee-rate roles
  - Pure effective admission derivation from static and rolling floors only
  - Separate package member-static and aggregate-rolling applicability decisions
  - Role-correct ordinary admission, replacement bumping, and pressure evidence
affects: [phase-131, phase-132, phase-137, mempool-policy, rpc-evidence]
tech-stack:
  added: []
  patterns:
    - Role-neutral fee arithmetic wrapped by semantic policy newtypes
    - Derived effective policy state instead of mutable canonical duplication
key-files:
  created:
    - packages/open-bitcoin-mempool/src/fee.rs
    - packages/open-bitcoin-mempool/src/pool/tests/fee_cases.rs
  modified:
    - packages/open-bitcoin-mempool/src/pool.rs
    - packages/open-bitcoin-mempool/src/pool/lifecycle.rs
    - packages/open-bitcoin-mempool/src/types.rs
    - packages/open-bitcoin-node/src/network.rs
    - packages/open-bitcoin-rpc/src/dispatch/node.rs
key-decisions:
  - "Keep FeeRate role-neutral for wallet arithmetic while requiring semantic wrappers at mempool policy boundaries."
  - "Initialize the rolling floor to zero and derive effective admission from static and rolling values at each decision or summary boundary."
  - "Represent package member-static and eligible aggregate-rolling obligations as independent decisions without a generic exception switch."
patterns-established:
  - "Fee role wrappers expose explicit constructors and arithmetic accessors but no cross-role From conversions."
  - "Pressure evidence carries typed fee roles; node and RPC adapters project role-preserving numeric fields."
requirements-completed: []
requirements-addressed: [FEEP-02]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 130-2026-07-23T14-26-46
generated_at: 2026-07-23T19:15:53Z
duration: 28 min
completed: 2026-07-23
---

# Phase 130 Plan 02: Fee Role and Admission Floor Summary

**Compile-time-distinct fee roles with pure effective-floor derivation, separate package obligations, and role-correct admission, replacement, and pressure evidence**

## Performance

- **Duration:** 28 min
- **Started:** 2026-07-23T18:47:36Z
- **Completed:** 2026-07-23T19:15:53Z
- **Tasks:** 2
- **Files modified:** 16

## Accomplishments

- Added static relay, incremental relay, rolling mempool, and effective admission newtypes without cross-role conversions.
- Proved rolling below, equal to, and above static; zero rolling; incremental exclusion; and separate package member/aggregate obligations through 11 discoverable fee tests.
- Enforced ordinary admission at `max(static, rolling)` while retaining incremental relay fee exclusively for replacement bump calculations.
- Migrated pressure, node, RPC, integration-test, and wallet arithmetic callers to typed virtual-size and role-preserving fee surfaces.

## Task Commits

1. **Task 1: Define semantic fee roles and floor derivation** - `628558c4`
2. **Task 2: Apply fee roles to admission, replacement, and pressure evidence** - `b6787dbf`

## Files Created/Modified

- `packages/open-bitcoin-mempool/src/fee.rs` - Fee arithmetic, semantic role wrappers, effective derivation, and package floor assessment.
- `packages/open-bitcoin-mempool/src/pool/tests/fee_cases.rs` - Named boundary tests for derivation, package obligations, admission, replacement, and evidence.
- `packages/open-bitcoin-mempool/src/types.rs` - Role-typed policy configuration and typed fee arithmetic callers.
- `packages/open-bitcoin-mempool/src/pool.rs` - Zero rolling baseline, derived ordinary admission, and incremental-only replacement bumps.
- `packages/open-bitcoin-mempool/src/pool/lifecycle.rs` - Typed static, incremental, rolling, and derived effective pressure evidence.
- `packages/open-bitcoin-node/src/network.rs` - Role-preserving node status projection.
- `packages/open-bitcoin-node/src/network/types.rs` - Explicit numeric adapter fields for all four fee roles.
- `packages/open-bitcoin-rpc/src/dispatch/node.rs` - Static/effective RPC projection aligned with pinned Knots meanings.
- `packages/open-bitcoin-wallet/src/wallet/build.rs` - Typed transaction-vsize inputs for role-neutral wallet fee arithmetic.
- `docs/parity/source-breadcrumbs.json` - Production and focused test registration under `mempool-policy`.
- `docs/metrics/lines-of-code.md` - Hook-regenerated tracked LOC freshness.

## Decisions Made

- `FeeRate` remains a role-neutral arithmetic value because wallet fee selection is not a mempool floor; policy configuration and state use semantic wrappers.
- `EffectiveAdmissionFeeRate` has no public constructor and is produced only by the static-plus-rolling derivation function.
- Phase 130 exposes rolling state read-only with a zero baseline; bump and decay behavior remains owned by Phase 131.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

- Both TDD tasks were proven red before implementation and green afterward. Intentionally failing intermediate states were not committed because sequential execution required normal repository hooks and prohibited `--no-verify`; each completed task was committed atomically after its blocking checks passed.
- The metadata hook correctly rejected early FEEP-02 completion because Phase 130 has no lifecycle-valid `VERIFICATION.md` yet. The requirement remains addressed but pending until phase verification.

## Authentication Gates

None.

## Known Stubs

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan 130-03 can add canonical entry metadata and explicit policy inputs on top of role-correct fee and resource contracts.
- Phase 131 can implement rolling bump/decay behind `RollingMempoolFeeRate` without changing ordinary admission semantics.
- Phase 132 can consume the separate member-static and aggregate-rolling package contract without a generic exception bypass.
- No blockers remain.

## Self-Check: PASSED

- Created files and summary exist.
- Task commits `628558c4` and `b6787dbf` exist.
- Eleven named fee-role tests, parity breadcrumbs, the full mempool library suite, and the timed workspace all-target compile gate pass.
- Source scans find every required fee role and no cross-role `From` conversions, old fee field names, or mutable effective state in `Mempool` or `PolicyConfig`.

---
*Phase: 130-resource-time-and-fee-primitives*
*Completed: 2026-07-23*
