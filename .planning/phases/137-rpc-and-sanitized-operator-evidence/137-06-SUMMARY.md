---
phase: 137-rpc-and-sanitized-operator-evidence
plan: 06
subsystem: rpc
tags: [openbitcoinpackage, package-cli, dual-state, fingerprint, dry-run, submit]

# Dependency graph
requires:
  - phase: 137-rpc-and-sanitized-operator-evidence
    provides: Plan 01 PackageReport projectors and member facts
  - phase: 137-rpc-and-sanitized-operator-evidence
    provides: Plan 03 BaselineParity testmempoolaccept and submitpackage registration
provides:
  - OpenBitcoinExtension method openbitcoinpackage with required mode dry-run|submit
  - project_open_bitcoin_package typed DTO with fingerprint, ordered members, and dual-state
  - open-bitcoin package dry-run|submit CLI that calls openbitcoinpackage
affects:
  - 137-10 relay-disabled proof and remaining submit fanout receipts
  - 137-07 dashboard must not copy the typed package report

# Tech tracking
tech-stack:
  added: []
  patterns:
    - One extension method with required mode instead of two method names
    - Typed dual-state projector stays separate from Knots testmempoolaccept/submitpackage JSON

key-files:
  created:
    - packages/open-bitcoin-cli/src/operator/package.rs
  modified:
    - packages/open-bitcoin-rpc/src/method.rs
    - packages/open-bitcoin-rpc/src/method/package.rs
    - packages/open-bitcoin-rpc/src/package_projection.rs
    - packages/open-bitcoin-rpc/src/dispatch/package.rs
    - packages/open-bitcoin-cli/src/operator.rs
    - packages/open-bitcoin-cli/src/operator/runtime.rs
    - packages/open-bitcoin-cli/src/client.rs

key-decisions:
  - "Keep openbitcoinpackage as one OpenBitcoinExtension method with required mode dry-run|submit"
  - "Map AlreadyPresent to still-present on both dry-run and submit; FinallyPresent to accepted; rejected/absent to cleared"
  - "Walk submit relay from lifecycle/fanout facts, not only RelayIntent"
  - "Leave BaselineParity Knots methods unextended and keep open-bitcoin-cli as a forwarder"

patterns-established:
  - "Pattern 1: Knots JSON stays on testmempoolaccept/submitpackage; typed fingerprint and dual-state live only on openbitcoinpackage"
  - "Pattern 2: open-bitcoin package is the originating human/JSON workflow; open-bitcoin-cli does not grow a package subcommand"

requirements-completed: [MPOBS-01, MPOBS-03]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 137-2026-08-19T21-52-42
generated_at: 2026-08-19T23:27:23Z

# Metrics
duration: 9min
completed: 2026-08-19
---

# Phase 137 Plan 06: RPC and Sanitized Operator Evidence Summary

**Typed `openbitcoinpackage` extension plus `open-bitcoin package {dry-run,submit}` expose fingerprint, input-ordered members, and dual-state tokens without claiming public relay**

## Performance

- **Duration:** 9 min
- **Started:** 2026-08-19T23:18:34Z
- **Completed:** 2026-08-19T23:27:23Z
- **Tasks:** 2
- **Files modified:** 25

## Accomplishments

- Registered `SupportedMethod::OpenBitcoinPackage` as `OpenBitcoinExtension` / `Node` with required `mode` and `deny_unknown_fields`.
- Projected fingerprint, `complete`/`partial`/`failed` status, input-ordered members, and `effective_fee_groups` from the same `PackageReport` family.
- Locked admission mapping: `FinallyPresent` → `accepted`, `AlreadyPresent` → `still-present`, rejected/absent → `cleared` on both dry-run and submit.
- Added `open-bitcoin package dry-run|submit` with UI-SPEC disclaimers and hex arity 1..=25; `open-bitcoin-cli` still only forwards Knots names.

## Task Commits

Each task was committed atomically:

1. **Task 1 RED: failing openbitcoinpackage tests** - `6cb8e324` (test)
2. **Task 1 GREEN: implement openbitcoinpackage extension RPC** - `1b0a7b64` (feat)
3. **Task 2 RED: failing open-bitcoin package CLI tests** - `7a102c15` (test)
4. **Task 2 GREEN: implement package dry-run and submit CLI** - `fbc01a87` (feat)

**Plan metadata:** `docs(137-06)` commit for this SUMMARY only; STATE.md and ROADMAP.md were not updated.

_Note: TDD tasks produced RED then GREEN commits; no refactor commit was needed._

## Files Created/Modified

- `packages/open-bitcoin-rpc/src/method.rs` - `OpenBitcoinPackage` in every `SupportedMethod` match
- `packages/open-bitcoin-rpc/src/method/package.rs` - `OpenBitcoinPackageRequest` with `mode` and `rawtxs`
- `packages/open-bitcoin-rpc/src/method/tests.rs` - extension origin and unknown-field coverage
- `packages/open-bitcoin-rpc/src/package_projection.rs` - typed projector and dual-state tokens
- `packages/open-bitcoin-rpc/src/package_projection/tests.rs` - fingerprint, dual-state, and omitted-key tests
- `packages/open-bitcoin-rpc/src/dispatch.rs` / `dispatch/package.rs` - dry-run vs submit dispatch
- `packages/open-bitcoin-rpc/src/dispatch/tests/package_methods.rs` - mempool-size and still-present tests
- `packages/open-bitcoin-rpc/src/context/network.rs` - `package_relay_disabled()` helper
- `packages/open-bitcoin-cli/src/operator/package.rs` - clap command, human render, HTTP call
- `packages/open-bitcoin-cli/src/operator.rs` / `runtime.rs` - `Package(PackageArgs)` wiring
- `packages/open-bitcoin-cli/src/client.rs` - forward `testmempoolaccept`, `submitpackage`, and `openbitcoinpackage`
- `packages/open-bitcoin-cli/src/operator/status.rs` plus fixtures - Plan 05 `MempoolStatus` compile unblock

## Decisions Made

- One method `openbitcoinpackage` with required `mode` rather than two extension names.
- Admission tokens come only from `PackageMemberResult`; submit relay walks the D-15 ladder after `relay_disabled`.
- `open-bitcoin-cli` serializes the new `MethodCall` variants but does not add a package subcommand.
- File-header breadcrumbs were added on `operator/package.rs`; `source-breadcrumbs.json` was left unchanged per the plan.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Wired new MethodCall variants in the baseline CLI client**
- **Found during:** Task 2 (package CLI)
- **Issue:** `method_call_to_json` did not cover `TestMempoolAccept`, `SubmitPackage`, or `OpenBitcoinPackage`, so `open-bitcoin-cli` failed to compile.
- **Fix:** Forward each request through the existing `to_json_value` serializer. No package parser was added.
- **Files modified:** `packages/open-bitcoin-cli/src/client.rs`
- **Verification:** `package_` CLI tests pass
- **Committed in:** `fbc01a87`

**2. [Rule 3 - Blocking] Updated CLI MempoolStatus literals after Plan 05**
- **Found during:** Task 2 (package CLI)
- **Issue:** Plan 05 added required `MempoolStatus` groups; operator collectors and fixtures still used the old two-field struct and omitted `OpenBitcoinNetworkStatusResponse.mempool`.
- **Fix:** Use `MempoolStatus::from_transactions_and_relay` / `MempoolStatus::default()` and clone relay before moving mempool.
- **Files modified:** `packages/open-bitcoin-cli/src/operator/status.rs` and CLI status/support/dashboard/soak fixtures
- **Verification:** `open-bitcoin-cli` package tests compile and pass
- **Committed in:** `fbc01a87`

---

**Total deviations:** 2 auto-fixed (2 blocking)
**Impact on plan:** Required to compile the CLI crate. No scope creep; Knots methods stay unextended.

## Issues Encountered

- Submit dispatch currently fills `PackageSubmitRelayFacts.served` from `delta.retry_clears` `EligibleServe` and `admitted` from `delta.admitted`. Queued, leftover-unattempted, emitted, requested, suppressed, and unbroadcast stay default-false until per-member fanout APIs are available to the RPC crate. The projector ladder already accepts those facts. Plan 10 owns relay-disabled proof.

## Known Stubs

- `packages/open-bitcoin-rpc/src/dispatch/package.rs` `submit_dual_state`: fanout receipts other than `served`/`admitted` are not yet populated. Intentional incomplete wiring; tokens exist and map when facts are supplied.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan 10 can prove `relay_disabled` and finish submit fanout receipt wiring on the existing ladder.
- Dashboard and `openbitcoinnetworkstatus` must not echo the typed package report.
- Baseline `testmempoolaccept` / `submitpackage` remain Knots-only on `open-bitcoin-cli`.

## Self-Check: PASSED

---
*Phase: 137-rpc-and-sanitized-operator-evidence*
*Completed: 2026-08-19*
