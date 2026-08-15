---
phase: 136-receive-independent-maintenance-and-transport-receipts
plan: "02"
subsystem: network
tags: [unbroadcast, insert-gate, reconciliation, subset-oracle, transport-written]

# Dependency graph
requires:
  - phase: 134-authoritative-cross-cache-lifecycle-integration
    provides: prepare_unbroadcast_projection and expected_unbroadcast_members equality oracle
  - phase: 136-receive-independent-maintenance-and-transport-receipts
    provides: Plan 01 cycle, budgets, and leftover cursor over a caller-supplied BTreeSet
provides:
  - insert-only-on-new-admission unbroadcast projection
  - subset reconciliation oracle that accepts TransportWritten-cleared still-present members
affects: [phase-136-03, phase-136-04, IBR-01, IBR-04, TransportWritten]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - unbroadcast insert is delta().admitted intersect is_retry_eligible(true)
    - expected_unbroadcast_members is unbroadcast intersect retry-eligible canonical members
    - EligibleServe is not a membership mutation in this projection

key-files:
  created:
    - packages/open-bitcoin-node/src/network/tests/unbroadcast_projection_cases.rs
  modified:
    - packages/open-bitcoin-node/src/network/lifecycle_projection/authority.rs
    - packages/open-bitcoin-node/src/network/lifecycle_projection/reconciliation.rs
    - packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/reconciliation.rs
    - packages/open-bitcoin-node/src/network/tests.rs
    - docs/parity/source-breadcrumbs.json
    - docs/metrics/lines-of-code.md

key-decisions:
  - "Insert unbroadcast members only when delta().admitted contains the identity and metadata is retry-eligible."
  - "expected_unbroadcast_members is the intersection of the live set with retry-eligible canonical members."
  - "A still-present TransportWritten-cleared member is not a reconciliation mismatch."
  - "Keep IBR-01 and IBR-04 Pending until lifecycle-valid phase verification."

patterns-established:
  - "Unbroadcast membership is sticky after first local+requested admission until teardown or retry_clears."
  - "Reconciliation flags illegal extras only; missing retry-eligible members are accepted as cleared."

requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 136-2026-08-15T21-24-17
generated_at: 2026-08-15T23:54:34Z

# Metrics
duration: 21min
completed: 2026-08-15
---

# Phase 136 Plan 02: Insert-On-Admission and Subset Unbroadcast Oracle Summary

**Insert-only-on-new-admission unbroadcast projection plus a subset reconciliation oracle so a later still-present package or maintenance fact cannot re-insert a TransportWritten-cleared parent.**

## Performance

- **Duration:** 21 min
- **Started:** 2026-08-15T23:33:17Z
- **Completed:** 2026-08-15T23:54:34Z
- **Tasks:** 2
- **Files modified:** 7

## Accomplishments

- `prepare_unbroadcast_projection` inserts only `delta().admitted` ∩ retry-eligible members. Teardown and `retry_clears` remain the only exits.
- A later child-package admission whose AlreadyPresent parent is absent from `delta().admitted` leaves that parent cleared; the newly admitted local+requested child is inserted.
- Empty-admitted maintenance facts do not re-insert a previously cleared still-present identity.
- `expected_unbroadcast_members` is now the intersection of the live unbroadcast set with retry-eligible canonical members. Cleared still-present members are not mismatches; foreign and ineligible extras still fail closed.
- No `MempoolRetryClearCause::EligibleServe` rows are emitted by this projection.

## Task Commits

Each task was committed atomically:

1. **Task 1: Insert only newly admitted retry-eligible members** - `42b8f86d` (feat)
2. **Task 2: Rewrite expected unbroadcast to a subset oracle** - `118448c6` (feat)

**Plan metadata:** pending docs commit

_Note: TDD RED commits were not created because `.githooks/pre-commit` runs `bash scripts/verify.sh`, which requires a green tree. Tests were written first, then the production gate and oracle were implemented in the same task commits._

## Files Created/Modified

- `packages/open-bitcoin-node/src/network/lifecycle_projection/authority.rs` - D-04 insert gate on `delta().admitted` ∩ `is_retry_eligible(true)`
- `packages/open-bitcoin-node/src/network/tests/unbroadcast_projection_cases.rs` - Five insert-gate tests and local admission helpers
- `packages/open-bitcoin-node/src/network/tests.rs` - Registers `unbroadcast_projection_cases`
- `packages/open-bitcoin-node/src/network/lifecycle_projection/reconciliation.rs` - Subset oracle for expected unbroadcast membership
- `packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/reconciliation.rs` - Renamed missing/swap tests and added ineligible-extra coverage
- `docs/parity/source-breadcrumbs.json` - New `node-unbroadcast-projection` group
- `docs/metrics/lines-of-code.md` - Hook-regenerated LOC report

## Decisions Made

- Insert only on new local+requested admission. Metadata eligibility alone must not recreate unbroadcast membership after a successful TX write or lifecycle removal.
- Reconciliation reports illegal extras only. A still-present retry-eligible member missing from the set is accepted as TransportWritten-cleared.
- A swap that removes a legal member and inserts a foreign extra counts as one mismatch (the extra).
- Do not emit `EligibleServe` retry-clear rows from this projection; classify-time serve is not a membership mutation.
- Keep IBR-01 and IBR-04 Pending until Phase 136 has lifecycle-valid VERIFICATION.md.

## Deviations from Plan

### Other Deviations

**1. TDD RED commits omitted**
- Pre-commit runs full `bash scripts/verify.sh`. Failing tests cannot be committed. Tests were written first, then the insert gate and subset oracle were implemented in the same feat commits.

**2. [Rule 3 - Blocking] Left IBR-01 and IBR-04 Pending**
- Marking them Complete fails `check-active-milestone-verification-traceability` because Phase 136 has no lifecycle-valid VERIFICATION.md yet. Same pattern as Phase 136 Plan 01 and Phase 135 MPDUR.

---

**Total deviations:** 2 process notes (RED commits, requirement traceability)
**Impact on plan:** D-04 insert/clear projection and IBR-04 still-present-clear survival match the plan. No GETDATA, PeerEmission, timer, or EligibleServe emission was added.

## Issues Encountered

- The new `node-unbroadcast-projection` breadcrumb group failed `--check` until the test file was staged, because the checker walks `git ls-files`. Staging before commit resolved it.
- Current `final_present()` is already admitted-only, so several insert-gate tests would have passed before the explicit `delta().admitted.contains` guard. The guard is still required as the D-04 contract.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan 03/04 can rely on a TransportWritten-cleared parent staying out of `unbroadcast_members` across later package and maintenance projections.
- Reconciliation will not force re-insertion of cleared still-present members.
- No timer, receipt, or fanout wiring was added; those remain later Phase 136 plans.
- No blockers.

---
*Phase: 136-receive-independent-maintenance-and-transport-receipts*
*Completed: 2026-08-15*

## Self-Check: PASSED
