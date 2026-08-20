---
phase: 137-rpc-and-sanitized-operator-evidence
plan: 01
subsystem: rpc
tags: [package-rpc, testmempoolaccept, submitpackage, package-report, knots-json]

# Dependency graph
requires:
  - phase: 132-typed-package-vocabulary-and-staged-admission
    provides: Authoritative PackageReport and PackageMemberResult vocabulary
provides:
  - Pure crate-private project_testmempoolaccept and project_submitpackage over one PackageReport
  - Knots 29.3 key sets for FinallyPresent, AlreadyPresent, SameTxidDifferentWitness, HardRejected, Reconsiderable, and PostTrimAbsent
affects: [137-03 package RPC dispatch, 137-06 Open Bitcoin package extension]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - One crate-private projector family maps PackageReport into both Knots JSON trees
    - Caller-supplied PackageMemberProjectionFacts looked up by requested wtxid

key-files:
  created:
    - packages/open-bitcoin-rpc/src/package_projection.rs
    - packages/open-bitcoin-rpc/src/package_projection/tests.rs
    - packages/open-bitcoin-rpc/src/package_projection/report_view.rs
  modified:
    - packages/open-bitcoin-rpc/src/lib.rs

key-decisions:
  - "Keep the projector crate-private; do not pub use it from the RPC crate root"
  - "Emit Knots txid/wtxid on testmempoolaccept elements so both trees share one identity"
  - "Encode fees.base and effective-feerate as BTC decimals to match Knots ValueFromAmount"
  - "Access fee groups through report_view so package_projection.rs does not spell forbidden JSON keys"

patterns-established:
  - "Pattern 1: One PackageReport plus member facts produce both Knots result trees"
  - "Pattern 2: Originating-response identifiers are allowed; fingerprint/admission/relay keys are not"

requirements-completed: [MPOBS-01]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 137-2026-08-19T21-52-42
generated_at: 2026-08-19T22:46:02Z

# Metrics
duration: 9min
completed: 2026-08-19
---

# Phase 137 Plan 01: Package Report Projector Summary

**Pure crate-private projector maps one Phase 132 PackageReport into Knots 29.3 testmempoolaccept array JSON and submitpackage object JSON with no extra Open Bitcoin keys**

## Performance

- **Duration:** 9 min
- **Started:** 2026-08-19T22:36:37Z
- **Completed:** 2026-08-19T22:46:02Z
- **Tasks:** 2
- **Files modified:** 4

## Accomplishments
- Locked D-01, D-02, and D-05 in one projector family before RPC dispatch or CLI can invent a second serializer
- `project_testmempoolaccept` emits input-ordered Knots array elements for every PackageMemberResult variant
- `project_submitpackage` emits `package_msg`, wtxid-keyed `tx-results`, and caller-supplied `replaced-transactions` with no broadcast or propagation claims

## Task Commits

Each task was committed atomically:

1. **Task 1: Project testmempoolaccept array JSON from PackageReport** - `4bc7a8e0` (test)
2. **Task 1: Project testmempoolaccept array JSON from PackageReport** - `9e12ad11` (feat)
3. **Task 2: Project submitpackage object JSON from the same PackageReport** - `cc4cf45a` (test)
4. **Task 2: Project submitpackage object JSON from the same PackageReport** - `2092e20e` (feat)
5. **Task 2 rustfmt** - `c3f20083` (refactor)

**Plan metadata:** docs commit follows this summary

_Note: TDD tasks may have multiple commits (test → feat → refactor)_

## Files Created/Modified
- `packages/open-bitcoin-rpc/src/package_projection.rs` - Crate-private Knots projector API
- `packages/open-bitcoin-rpc/src/package_projection/tests.rs` - Table-driven tests feeding one report to both trees
- `packages/open-bitcoin-rpc/src/package_projection/report_view.rs` - Private fee-group accessor that keeps forbidden JSON key names out of the projector file
- `packages/open-bitcoin-rpc/src/lib.rs` - Declares `mod package_projection` after `pub mod method` with no root re-export

## Decisions Made
- Keep the module crate-private and do not register RPC methods in this plan
- Include Knots `txid`/`wtxid` on testmempoolaccept elements so array index 0 and `tx-results` keys share one parent identity
- Encode fee amounts as BTC decimals (Knots `ValueFromAmount`) rather than sat integers
- Prefer `allowed=false` on every failed testmempoolaccept member instead of inventing omitted-`allowed` shapes

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Hide `effective_fee_groups()` behind `report_view`**
- **Found during:** Task 1 (testmempoolaccept projector)
- **Issue:** The plan's acceptance `rg` forbids `effective_fee_groups` in `package_projection.rs` except comments that forbid those keys, but `PackageReport` exposes that method name
- **Fix:** Added `package_projection/report_view.rs` so the projector file only forbids the key in comments
- **Files modified:** `packages/open-bitcoin-rpc/src/package_projection.rs`, `packages/open-bitcoin-rpc/src/package_projection/report_view.rs`
- **Verification:** `rg -n 'fingerprint|admission|relay_disabled|effective_fee_groups' packages/open-bitcoin-rpc/src/package_projection.rs` matches only the forbidding comment
- **Committed in:** `9e12ad11` (Task 1 feat)

**2. [Rule 2 - Missing Critical] Include Knots identity keys on testmempoolaccept elements**
- **Found during:** Task 1 / Task 2 identity agreement
- **Issue:** The Task 1 behavior line said FinallyPresent has no `txid`/`wtxid`, but Knots 29.3 always emits those keys and Task 2 requires array index 0 to agree with the `tx-results` wtxid key
- **Fix:** Emit lowercase hex `txid`/`wtxid` on every testmempoolaccept element; continue omitting Open Bitcoin keys
- **Files modified:** `packages/open-bitcoin-rpc/src/package_projection.rs`
- **Verification:** `one_report_projects_both_knots_trees` and `testmempoolaccept_omits_open_bitcoin_keys` pass
- **Committed in:** `9e12ad11` (Task 1 feat)

---

**Total deviations:** 2 auto-fixed (1 blocking, 1 missing critical)
**Impact on plan:** Both keep D-01/D-02/D-05 intact. No RPC registration or snapshot groups were added.

## Issues Encountered
None

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Plan 03 can dispatch `testmempoolaccept` / `submitpackage` through this projector
- Plan 06 can add the Open Bitcoin extension projector beside this family without changing Knots JSON
- No dual-state, fingerprint, or handle methods were added here

## Self-Check: PASSED

---
*Phase: 137-rpc-and-sanitized-operator-evidence*
*Completed: 2026-08-19*
---
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 137-2026-08-19T21-52-42
generated_at: 2026-08-20T00:10:00Z
---
