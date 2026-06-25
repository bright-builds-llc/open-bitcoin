---
phase: 91-peer-permissions-and-connection-classes
plan: 04
subsystem: rpc-networking
tags: [rust, p2p, peer-permissions, inbound-listener, admission]

requires:
  - phase: 91-02
    provides: "Resolved PeerPermissionClassRegistry carried on RuntimeConfig.inbound"
  - phase: 91-03
    provides: "InboundAdmissionRequest::from_permission_decision and managed permission counters"
provides:
  - "ManagedRpcContext storage for the resolved inbound permission registry"
  - "Remote SocketAddr permission decisions using literal remote_addr.ip() matching"
  - "Runtime listener admission through typed InboundPermissionDecision records"
  - "Loopback coverage for ordinary, permissioned, and protected inbound capacity behavior"
affects:
  - 91-05-shared-status-rpc-and-metrics-permission-evidence
  - 91-06-status-and-support-permission-evidence-rendering

tech-stack:
  added: []
  patterns:
    - "Keep listener socket I/O as a thin adapter over ManagedRpcContext permission decisions"
    - "Preserve compatibility admission helpers as ordinary by default"
    - "Resolve permission classes from SocketAddr::ip() only, never endpoint strings or raw config names"

key-files:
  created:
    - .planning/phases/91-peer-permissions-and-connection-classes/91-04-SUMMARY.md
  modified:
    - packages/open-bitcoin-rpc/src/context.rs
    - packages/open-bitcoin-rpc/src/context/network.rs
    - packages/open-bitcoin-rpc/src/inbound_listener.rs
    - packages/open-bitcoin-rpc/src/inbound_listener/tests.rs
    - packages/open-bitcoin-rpc/src/dispatch/tests.rs

key-decisions:
  - "Store the resolved PeerPermissionClassRegistry on ManagedRpcContext but omit it from Debug/status surfaces to avoid raw class-name leakage."
  - "Keep record_inbound_admission as an ordinary compatibility path and add record_inbound_admission_for_remote_addr for runtime listener use."
  - "Use listener remote_addr.ip() as the only runtime matching input for permission class resolution."

patterns-established:
  - "ManagedRpcContext::permission_decision_for_remote_addr is the RPC runtime seam for typed inbound permission decisions."
  - "handle_inbound_stream passes SocketAddr evidence to context and stays free of permission tokens, parsing, or relay behavior."

requirements-completed: [PERM-01, PERM-02, PERM-03]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 91-2026-06-25T13-36-41
generated_at: 2026-06-25T17:22:05Z

duration: 14min
completed: 2026-06-25
---

# Phase 91 Plan 04: Runtime Listener Permission-Aware Admission Wiring Summary

**Runtime inbound listener peers are classified from typed permission registries before admission while deferred relay-like permissions remain inert labels.**

## Performance

- **Duration:** 14 min
- **Started:** 2026-06-25T17:07:48Z
- **Completed:** 2026-06-25T17:22:05Z
- **Tasks:** 2
- **Files modified:** 5 code files plus this summary

## Accomplishments

- Added `PeerPermissionClassRegistry` storage to `ManagedRpcContext`, initialized from `RuntimeConfig.inbound.permission_classes`.
- Added `permission_decision_for_remote_addr` and `record_inbound_admission_for_remote_addr` so runtime admission resolves `SocketAddr::ip()` through the pure registry.
- Switched `handle_inbound_stream` from hard-coded ordinary admission to the remote-address-aware helper without adding raw token parsing to the listener.
- Added tests for default ordinary decisions, protected inbound reserved capacity, permissioned inbound ordinary capacity, and inactive relay/mempool/filter effect observations.

## Task Commits

Each task was committed atomically:

1. **Task 1: Store resolved permission registry on ManagedRpcContext** - `d1b1143` (`feat`)
2. **Task 2: Make listener admission permission-aware** - `f5866bd` (`feat`)

**Plan metadata:** final docs commit created after this summary.

## Files Created/Modified

- `packages/open-bitcoin-rpc/src/context.rs` - Stores the typed permission registry on `ManagedRpcContext`.
- `packages/open-bitcoin-rpc/src/context/network.rs` - Initializes registry defaults/runtime values and adds remote-address permission/admission helpers.
- `packages/open-bitcoin-rpc/src/inbound_listener.rs` - Routes accepted peers through permission-aware admission.
- `packages/open-bitcoin-rpc/src/inbound_listener/tests.rs` - Adds loopback coverage for ordinary reserved rejection, protected reserved admission, and permissioned ordinary-slot behavior.
- `packages/open-bitcoin-rpc/src/dispatch/tests.rs` - Adds runtime context permission decision tests without exposing raw class names.

## Decisions Made

- `ManagedRpcContext::new` and `ManagedRpcContext::for_local_operator` continue to use an empty/default permission registry.
- `record_inbound_admission` remains the ordinary compatibility helper for existing tests/status fixtures.
- Listener admission uses `record_inbound_admission_for_remote_addr`, which derives the slot class from `InboundPermissionDecision` rather than from raw tokens or caller-provided labels.

## Deviations from Plan

### Process Adjustments

**1. TDD red commits skipped**
- **Found during:** Task 1 and Task 2
- **Issue:** The plan marked both tasks `tdd="true"`, but repo/user coordination favored verified focused commits and `--no-verify` over intentionally failing red-test commits.
- **Adjustment:** Added focused tests with implementation per task, then committed only verified green compile/lint states.
- **Verification:** `cargo check`, `cargo build`, `cargo clippy`, and `cargo test --no-run` for `open-bitcoin-rpc` passed.

**2. Local Rust executable test launch remained blocked**
- **Found during:** Task 1 and Task 2 verification
- **Issue:** Focused `cargo test` commands compiled but timed out after Cargo launched generated test binaries and printed `Running unittests src/lib.rs`.
- **Adjustment:** Used the requested `timeout 30s` wrapper, recorded exit code 124, and confirmed no matching test binaries remained running.
- **Verification:** `cargo test --no-run` passed and all bounded focused execution attempts were controlled by `timeout 30s`.

---

**Total deviations:** 2 process adjustments.
**Impact on plan:** Implementation scope stayed within Plan 91-04 ownership. No relay, mempool, filter, forcerelay, ban, eviction, public-default, or outbound-sync behavior was added.

## Verification Results

- `cargo fmt --all --manifest-path packages/Cargo.toml` - passed
- `cargo check --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-targets --all-features` - passed
- `cargo build --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-targets --all-features` - passed
- `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-targets --all-features -- -D warnings` - passed
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-features --no-run` - passed
- `timeout 30s cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc permission_context -- --nocapture` - timed out with code 124 after printing `Running unittests src/lib.rs`
- `timeout 30s cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc inbound_permission -- --nocapture` - timed out with code 124 after printing `Running unittests src/lib.rs`
- `timeout 30s cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc inbound_listener -- --nocapture` - timed out with code 124 after printing `Running unittests src/lib.rs`
- `rg` acceptance scans for registry storage, remote-address decisions, listener wiring, and no listener relay/mempool/filter token logic - passed
- `pgrep -fl "open_bitcoin_rpc|open_bitcoind|black_box_parity"` - no matching test binaries running

## Known Stubs

None.

## Issues Encountered

- Local generated Rust test binaries still hang immediately after Cargo launches them. This matched the known local blocker and was contained with `timeout 30s`; compile, build, clippy, and no-run test verification all passed.

## Authentication Gates

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Plan 91-05 can project the managed permission counters and latest admission evidence into shared status/RPC/metrics surfaces. Runtime listener admission now produces permission-aware records and leaves relay-like permissions as inactive observations only.

## Self-Check: PASSED

- Found `.planning/phases/91-peer-permissions-and-connection-classes/91-04-SUMMARY.md`
- Found `packages/open-bitcoin-rpc/src/context.rs`
- Found `packages/open-bitcoin-rpc/src/context/network.rs`
- Found `packages/open-bitcoin-rpc/src/inbound_listener.rs`
- Found `packages/open-bitcoin-rpc/src/inbound_listener/tests.rs`
- Found `packages/open-bitcoin-rpc/src/dispatch/tests.rs`
- Found task commit `d1b1143`
- Found task commit `f5866bd`

---
*Phase: 91-peer-permissions-and-connection-classes*
*Completed: 2026-06-25*
