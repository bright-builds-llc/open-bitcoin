---
phase: 90-inbound-listener-and-admission-policy
plan: 06
subsystem: rpc
tags: [rust, rpc, inbound, status, p2p]

requires:
  - phase: 90-04
    provides: Runtime listener activation and managed inbound admission evidence
  - phase: 90-05
    provides: Shared InboundPeerServingStatus contract
provides:
  - Open Bitcoin RPC extension method `openbitcoinnetworkstatus`
  - RPC dispatch projection of managed inbound admission evidence into `FieldAvailability<InboundPeerServingStatus>`
  - Baseline `getnetworkinfo` regression coverage preventing detailed inbound field drift
affects:
  - phase-90-cli-status
  - phase-90-support-evidence
  - phase-90-final-verification

tech-stack:
  added: []
  patterns:
    - Open Bitcoin extension methods are marked with `MethodOrigin::OpenBitcoinExtension`
    - Detailed inbound evidence is serialized through the shared node status contract
    - Baseline-compatible RPC responses stay separate from Open Bitcoin-owned status extensions

key-files:
  created:
    - .planning/phases/90-inbound-listener-and-admission-policy/90-06-SUMMARY.md
  modified:
    - packages/open-bitcoin-rpc/src/context/network.rs
    - packages/open-bitcoin-rpc/src/method.rs
    - packages/open-bitcoin-rpc/src/method/node.rs
    - packages/open-bitcoin-rpc/src/method/tests.rs
    - packages/open-bitcoin-rpc/src/dispatch.rs
    - packages/open-bitcoin-rpc/src/dispatch/node.rs
    - packages/open-bitcoin-rpc/src/dispatch/tests.rs

key-decisions:
  - "Added `openbitcoinnetworkstatus` as an Open Bitcoin node extension, not a baseline parity method."
  - "Kept detailed listener/admission fields out of `getnetworkinfo` and exposed them only through the Open Bitcoin-owned RPC method."
  - "Projected managed admission counters into the shared `InboundPeerServingStatus` contract rather than a dispatch-local DTO."

patterns-established:
  - "RPC methods with Open Bitcoin-only behavior are normalized and dispatched separately from Knots-shaped methods."
  - "Inbound status RPC serialization consumes `FieldAvailability<InboundPeerServingStatus>` directly."

requirements-completed: [INB-03, INB-05]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 90-2026-06-25T04-23-47
generated_at: 2026-06-25T08:16:37Z

duration: 21 min
completed: 2026-06-25
---

# Phase 90 Plan 06: RPC Inbound Network Status Summary

**Open Bitcoin network status RPC extension with shared inbound admission evidence and baseline getnetworkinfo shape protection**

## Performance

- **Duration:** 21 min
- **Started:** 2026-06-25T07:54:44Z
- **Completed:** 2026-06-25T08:16:37Z
- **Tasks:** 2
- **Files modified:** 7

## Accomplishments

- Added `openbitcoinnetworkstatus` to the supported RPC method registry as `MethodOrigin::OpenBitcoinExtension` and `MethodScope::Node`.
- Added request/response contracts for `OpenBitcoinNetworkStatus`, returning `FieldAvailability<InboundPeerServingStatus>`.
- Wired dispatch to return current managed inbound admission evidence and preserve the shared unavailable reason when no inbound evidence exists.
- Added regression coverage proving `getnetworkinfo` keeps baseline count fields and does not serialize detailed listener/admission field names.

## Task Commits

1. **Task 1 RED: method normalization tests** - `86e2310` (test)
2. **Task 1 GREEN: method contract implementation** - `249baed` (feat)
3. **Task 2 RED: dispatch/status tests** - `5b55afe` (test)
4. **Task 2 GREEN: dispatch inbound network status** - `f6874e4` (feat)

## Files Created/Modified

- `packages/open-bitcoin-rpc/src/method.rs` - Adds supported method normalization and Open Bitcoin extension metadata.
- `packages/open-bitcoin-rpc/src/method/node.rs` - Adds request/response DTOs for the new status method.
- `packages/open-bitcoin-rpc/src/method/tests.rs` - Covers method list, origin/scope, and parameter rejection.
- `packages/open-bitcoin-rpc/src/context/network.rs` - Projects managed inbound admission counters into the shared inbound status contract.
- `packages/open-bitcoin-rpc/src/dispatch.rs` - Dispatches `OpenBitcoinNetworkStatus`.
- `packages/open-bitcoin-rpc/src/dispatch/node.rs` - Serializes the context-derived inbound status response.
- `packages/open-bitcoin-rpc/src/dispatch/tests.rs` - Covers available/unavailable status and baseline `getnetworkinfo` shape.

## Decisions Made

- Kept `getnetworkinfo` limited to baseline-compatible count fields: `connections`, `connections_in`, and `connections_out`.
- Used the Plan 05 `InboundPeerServingStatus` contract directly in the RPC response instead of adding a parallel RPC-specific shape.
- Kept daemon/http shared-context rewiring out of this plan because the user constrained implementation to the 90-06 owned files.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Kept dispatcher exhaustive after adding the method variant**
- **Found during:** Task 1 GREEN
- **Issue:** Adding `MethodCall::OpenBitcoinNetworkStatus` made `dispatch.rs` non-exhaustive, so method-only tests could not compile before Task 2 wiring.
- **Fix:** Added a temporary dispatcher arm in Task 1 and replaced it with the real status response in Task 2.
- **Files modified:** `packages/open-bitcoin-rpc/src/dispatch.rs`
- **Verification:** `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc network_status -- --nocapture` passed after Task 2.
- **Committed in:** `249baed`, replaced in `f6874e4`

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** Narrow Rust exhaustiveness repair only. No new baseline fields or deferred relay/permission behavior were added.

## Issues Encountered

- Cargo initially waited on the shared artifact lock while other Phase 90 work was active.
- Concurrent support-bundle changes appeared in `packages/open-bitcoin-cli/` during execution and were left untouched by this plan. They were later committed by their owning executor.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc network_status -- --nocapture` passed with 4 matching tests.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc open_bitcoin_network_status -- --nocapture` passed with 3 matching tests.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc get_network_info -- --nocapture` passed with 1 matching test.
- `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-targets --all-features -- -D warnings` passed.
- `rustfmt --edition 2024 --check ...` passed for the seven owned Rust files.
- Required `rg` scans for method normalization and inbound status projection passed.

## Known Stubs

- `packages/open-bitcoin-rpc/src/context/network.rs`: `bound_endpoints` is currently emitted as an empty vector when deriving available status from managed admission evidence. The Plan 04 listener worker owns endpoint evidence in a separate runtime context, and daemon/http shared-context rewiring is outside the 90-06 owned file list.

## Threat Flags

None. The new RPC method is the planned Open Bitcoin extension surface and `getnetworkinfo` drift is covered by regression tests.

## Authentication Gates

None.

## User Setup Required

None - no external service configuration required.

## State Updates

Skipped intentionally. The orchestrator explicitly owns `.planning/STATE.md` and `.planning/ROADMAP.md` for this parallel phase run.

## Next Phase Readiness

Ready for downstream Phase 90 status/support work with one noted follow-up: endpoint evidence needs shared daemon/http context wiring before RPC can report actual bound listener endpoints.

## Self-Check: PASSED

- Found all seven owned RPC files.
- Found `.planning/phases/90-inbound-listener-and-admission-policy/90-06-SUMMARY.md`.
- Found commits `86e2310`, `249baed`, `5b55afe`, and `f6874e4`.

---
*Phase: 90-inbound-listener-and-admission-policy*
*Completed: 2026-06-25*
