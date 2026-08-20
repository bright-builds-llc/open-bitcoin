---
phase: 137-rpc-and-sanitized-operator-evidence
plan: 03
subsystem: rpc
tags: [package-rpc, testmempoolaccept, submitpackage, knots-json, baseline-parity]

# Dependency graph
requires:
  - phase: 137-rpc-and-sanitized-operator-evidence
    provides: Plan 01 crate-private Knots projectors over PackageReport
  - phase: 137-rpc-and-sanitized-operator-evidence
    provides: Plan 02 dry_run_local_package and submit_local_package handle methods
provides:
  - BaselineParity testmempoolaccept and submitpackage method registration
  - Knots RPC codes -8/-22/-25 for count, decode, and submitpackage topology
  - Dispatch that dry-runs or submits through Plan 01 projectors and Plan 02 handle methods
affects: [137-06 Open Bitcoin package extension, 137-05 CLI forwarding]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - Package RPC dispatch lives in dispatch/package.rs, not dispatch/node.rs
    - RPC-level errors are count/decode/topology only; member policy stays in the result body

key-files:
  created:
    - packages/open-bitcoin-rpc/src/method/package.rs
    - packages/open-bitcoin-rpc/src/dispatch/package.rs
    - packages/open-bitcoin-rpc/src/dispatch/tests/package_methods.rs
  modified:
    - packages/open-bitcoin-rpc/src/error.rs
    - packages/open-bitcoin-rpc/src/method.rs
    - packages/open-bitcoin-rpc/src/method/tests.rs
    - packages/open-bitcoin-rpc/src/dispatch.rs
    - packages/open-bitcoin-rpc/src/dispatch/tests.rs
    - packages/open-bitcoin-rpc/src/context/network.rs
    - docs/parity/source-breadcrumbs.json

key-decisions:
  - "Keep testmempoolaccept and submitpackage as BaselineParity node methods with Knots names"
  - "Map empty/count to -8, hex/decode to -22, and submitpackage topology to -25"
  - "Fail closed on explicit maxfeerate, maxburnamount, and non-empty ignore_rejects"
  - "Resolve RelayIntent from activation the same way local sendrawtransaction does"

patterns-established:
  - "Pattern 1: Knots package request types live in method/package.rs beside method/node.rs"
  - "Pattern 2: Dispatch calls Plan 02 handle methods then Plan 01 projectors"

requirements-completed: [MPOBS-01]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 137-2026-08-19T21-52-42
generated_at: 2026-08-19T23:09:42Z

# Metrics
duration: 8min
completed: 2026-08-19
---

# Phase 137 Plan 03: Package RPC Registration and Dispatch Summary

**Knots-named testmempoolaccept and submitpackage are BaselineParity node methods with -8/-22/-25 error boundaries, dry-run that does not mutate, and Knots JSON without Open Bitcoin keys**

## Performance

- **Duration:** 8 min
- **Started:** 2026-08-19T23:02:15Z
- **Completed:** 2026-08-19T23:09:42Z
- **Tasks:** 2
- **Files modified:** 10

## Accomplishments
- Registered `testmempoolaccept` and `submitpackage` with Knots parameter names and `deny_unknown_fields`
- Added Knots RPC codes `-8`, `-22`, and `-25` plus matching `RpcFailure` constructors
- Dispatched dry-run through `dry_run_local_package` and submit through `submit_local_package`, then Plan 01 projectors
- Kept member policy in the HTTP 200 result body and left `getrawmempool` unregistered

## Task Commits

Each task was committed atomically:

1. **Task 1: Add Knots error codes and BaselineParity method registration** - `859a8804` (test)
2. **Task 1: Add Knots error codes and BaselineParity method registration** - `5e375330` (feat)
3. **Task 2: Dispatch decode, count, topology, project, and handle calls** - `39ff4ed9` (test)
4. **Task 2: Dispatch decode, count, topology, project, and handle calls** - `8f282178` (feat)

**Plan metadata:** docs commit follows this summary

_Note: TDD tasks may have multiple commits (test → feat → refactor)_

## Files Created/Modified
- `packages/open-bitcoin-rpc/src/method/package.rs` - Knots-named package request types
- `packages/open-bitcoin-rpc/src/dispatch/package.rs` - Count, decode, topology, project, and handle dispatch
- `packages/open-bitcoin-rpc/src/dispatch/tests/package_methods.rs` - Named package RPC behavior tests
- `packages/open-bitcoin-rpc/src/error.rs` - InvalidParameter, DeserializationError, VerifyError
- `packages/open-bitcoin-rpc/src/method.rs` - Method enum, normalize, and MethodCall variants
- `packages/open-bitcoin-rpc/src/method/tests.rs` - Registration and request-shape tests
- `packages/open-bitcoin-rpc/src/dispatch.rs` - Match arms for the new MethodCall variants
- `packages/open-bitcoin-rpc/src/dispatch/tests.rs` - Registers package_methods tests
- `packages/open-bitcoin-rpc/src/context/network.rs` - Thin wrappers that resolve RelayIntent then call Plan 02 handle methods
- `docs/parity/source-breadcrumbs.json` - Breadcrumb coverage for the new dispatch test file

## Decisions Made
- Keep both methods on `MethodOrigin::BaselineParity` and `MethodScope::Node`
- Use `-8` for unsupported explicit Knots fee/ignore options instead of growing sendrawtransaction helpers
- Build `PackageMemberProjectionFacts` from decoded txid/wtxid/vsize plus input-minus-output or singleton fee-group fees
- Collect `replaced-transactions` from lifecycle replacement removals only, not pressure evictions

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Exhaustive dispatch match during Task 1**
- **Found during:** Task 1 (method registration)
- **Issue:** Adding `MethodCall` variants made `dispatch.rs` non-exhaustive before Task 2 implemented handlers
- **Fix:** Temporary internal-error stub arms, replaced by real handlers in Task 2
- **Files modified:** `packages/open-bitcoin-rpc/src/dispatch.rs`
- **Verification:** Task 1 registration tests passed; Task 2 replaced the stubs
- **Committed in:** `5e375330` (Task 1 feat), replaced in `8f282178`

**2. [Rule 2 - Missing Critical] Context wrappers for Plan 02 handle methods**
- **Found during:** Task 2 (dispatch)
- **Issue:** Dispatch cannot call `ManagedNetworkHandle` methods without a context facade that also samples `RelayIntent`
- **Fix:** Added `ManagedRpcContext::dry_run_local_package` and `submit_local_package` that resolve activation-enabled → `Requested`
- **Files modified:** `packages/open-bitcoin-rpc/src/context/network.rs`
- **Verification:** Dry-run size and submitpackage success tests pass
- **Committed in:** `8f282178` (Task 2 feat)

**3. [Rule 3 - Blocking] Project projection errors without Display**
- **Found during:** Task 2 (GREEN compile)
- **Issue:** `PackageProjectionError` has no `Display`, so `error.to_string()` failed to compile
- **Fix:** Map projector failures with `format!("{error:?}")` into an internal RPC error
- **Files modified:** `packages/open-bitcoin-rpc/src/dispatch/package.rs`
- **Verification:** Timed `testmempoolaccept_` and `submitpackage_` filters pass
- **Committed in:** `8f282178` (Task 2 feat)

---

**Total deviations:** 3 auto-fixed (2 blocking, 1 missing critical)
**Impact on plan:** Required for compile, handle access, and fail-closed dispatch. No getrawmempool, no Open Bitcoin keys, and no sendrawtransaction shape change.

## Issues Encountered
None

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Plan 05/CLI can forward the Knots method names
- Plan 06 can add the Open Bitcoin package extension beside these BaselineParity handlers
- Dry-run remains non-mutating; topology stays `-25`; member policy stays in-body

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
