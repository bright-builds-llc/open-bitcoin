---
phase: 92-address-advertisement-and-discovery-boundaries
plan: 06
subsystem: networking/rpc
status: complete
completed_at: 2026-06-26T08:45:48Z
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 92-2026-06-26T03-52-33
generated_at: 2026-06-26T08:45:48Z
requirements: [ADDR-01, ADDR-02]
dependency_graph:
  requires: [92-04, 92-05]
  provides:
    - runtime listener evidence handoff into local advertisement policy
    - bounded Phase 92 address evidence in Open Bitcoin RPC status
    - baseline getnetworkinfo non-drift regression coverage
  affects:
    - packages/open-bitcoin-network/src/lib.rs
    - packages/open-bitcoin-rpc/src/context.rs
    - packages/open-bitcoin-rpc/src/context/address_boundary.rs
    - packages/open-bitcoin-rpc/src/context/network.rs
    - packages/open-bitcoin-rpc/src/dispatch/tests.rs
    - packages/open-bitcoin-rpc/src/inbound_listener/tests.rs
tech_stack:
  added: []
  patterns:
    - runtime evidence to pure policy adapter
    - bounded RPC status projection
    - baseline RPC non-drift regression
key_files:
  created:
    - .planning/phases/92-address-advertisement-and-discovery-boundaries/92-06-SUMMARY.md
    - packages/open-bitcoin-rpc/src/context/address_boundary.rs
  modified:
    - docs/metrics/lines-of-code.md
    - packages/open-bitcoin-network/src/lib.rs
    - packages/open-bitcoin-rpc/src/context.rs
    - packages/open-bitcoin-rpc/src/context/network.rs
    - packages/open-bitcoin-rpc/src/dispatch/tests.rs
    - packages/open-bitcoin-rpc/src/inbound_listener/tests.rs
decisions:
  - Use only configured listener addresses plus runtime-bound listener evidence for local advertisement decisions.
  - Keep `LocalPeerConfig.address` at the default zero address and let only policy-selected candidates affect version sender/getaddr behavior.
  - Re-export existing `addr` payload structs so public wire-message tests can construct `WireNetworkMessage::Addr`.
metrics:
  tasks_completed: 2
  task_commits: 2
  duration: 33m
---

# Phase 92 Plan 06: Runtime Address Evidence RPC Summary

Runtime listener evidence now feeds pure local-advertisement policy, and Open Bitcoin RPC status exposes bounded Phase 92 address evidence while baseline `getnetworkinfo` stays unchanged.

## Objective

Connect the pure address-boundary decisions from Plans 92-04 and 92-05 to the opt-in listener/runtime path and prove the RPC extension exposes only bounded evidence.

## Completed Tasks

| Task | Name | Commit | Result |
| ---- | ---- | ------ | ------ |
| 1 | Feed listener evidence into local advertisement policy | cf071a4 | Added runtime-to-policy handoff, invalid bound endpoint suppression, loopback/public listener tests, and a split helper module to stay under production file-length limits. |
| 2 | Project address evidence through Open Bitcoin RPC status | b796267 | Added end-to-end `openbitcoinnetworkstatus` coverage for local advertisements, learned addresses, getaddr serving/suppression, raw-detail redaction, and `getnetworkinfo` non-drift. |

## Implementation Notes

- `ManagedRpcContext` now retains inbound listener config and converts listener evidence into `LocalAdvertisementDecision` values through the pure `select_local_advertisement_candidates` path.
- Invalid runtime bound endpoint strings become `advertise_suppressed` evidence with `invalid_port` or `unsupported_address_network`; they do not panic and do not fall back to configured public addresses.
- `current_inbound_status` consumes `self.network.address_boundary_info()` and fills the Phase 92 fields on `InboundPeerServingStatus`.
- `openbitcoinnetworkstatus` tests now assert bounded labels/counts and absence of raw permission class names, raw config strings, peer IDs, and raw address bytes in the Phase 92 address-evidence subset.
- `getnetworkinfo` tests reject listener/admission, permission, and address-boundary fields so baseline RPC shape does not drift.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc inbound_listener --no-fail-fast`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc open_bitcoin_network_status --no-fail-fast`
- `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-targets --all-features -- -D warnings`
- `cargo fmt --manifest-path packages/Cargo.toml --all`
- `cargo clippy --manifest-path packages/Cargo.toml --all-targets --all-features -- -D warnings`
- `cargo build --manifest-path packages/Cargo.toml --all-targets --all-features`
- `cargo test --manifest-path packages/Cargo.toml --all-features`
- `bash scripts/verify.sh` via normal git hooks for both task commits

Acceptance scans found the required handoff/projection/status test patterns. Two planned negative scans had expected false positives: `interface_rpc.py` appears in a required parity breadcrumb, and `context/network.rs` still contains the required default zero `LocalPeerConfig.address.address_bytes` plus peer-id helper parameters outside the address-evidence status boundary.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Split context helper to satisfy production file-length gate**
- **Found during:** Task 1
- **Issue:** The first implementation kept the listener-to-address-policy helper in `context/network.rs`, and the normal commit hook rejected the file for exceeding the production Rust line limit.
- **Fix:** Moved the helper into `packages/open-bitcoin-rpc/src/context/address_boundary.rs` and wired it through `context.rs`.
- **Files modified:** `packages/open-bitcoin-rpc/src/context.rs`, `packages/open-bitcoin-rpc/src/context/address_boundary.rs`, `packages/open-bitcoin-rpc/src/context/network.rs`
- **Verification:** Focused inbound listener tests, full Rust pre-commit sequence, parity breadcrumb check, and `bash scripts/verify.sh`.
- **Commit:** cf071a4

**2. [Rule 3 - Blocking] Re-exported existing addr payload structs for public wire-message tests**
- **Found during:** Task 2
- **Issue:** `WireNetworkMessage::Addr` is public, but its `AddressList`/`AddressAnnouncement` payload structs were not re-exported, preventing RPC tests from constructing learned-address evidence.
- **Fix:** Re-exported the existing payload structs from `open-bitcoin-network`.
- **Files modified:** `packages/open-bitcoin-network/src/lib.rs`, `packages/open-bitcoin-rpc/src/dispatch/tests.rs`
- **Verification:** Focused RPC status tests, scoped RPC clippy, full Rust pre-commit sequence, and `bash scripts/verify.sh`.
- **Commit:** b796267

### Execution Notes

- TDD RED failures were observed for Task 1 before the implementation. Task 2 projection logic already existed from Plan 92-05, so this plan added the missing RPC regression coverage and the minimal export needed to drive it.
- `docs/metrics/lines-of-code.md` was refreshed by normal repo hooks and committed with each task.

## Issues Encountered

- The plan's negative grep for forbidden discovery terms matched the existing parity breadcrumb `interface_rpc.py`, not runtime discovery code.
- The plan's negative grep for raw/high-cardinality status evidence matched existing non-status context helpers and the required default zero local peer address. The new RPC test serializes only the Phase 92 address-evidence subset and verifies it omits raw addresses, peer IDs, raw config strings, and raw permission class names.

## Auth Gates

None.

## Known Stubs

None. Stub-pattern scan found no TODO/placeholders or empty-data UI/status stubs in files created or modified by this plan.

## Threat Flags

None. This plan wires existing opt-in listener evidence into existing pure policy and existing RPC status surfaces; it does not add new network listeners, authentication paths, file access patterns, or schema trust boundaries.

## Orchestrator Notes

- `.planning/STATE.md`, `.planning/ROADMAP.md`, and `.planning/REQUIREMENTS.md` were intentionally not updated because the orchestrator owns those writes after execution waves complete.
- The pre-existing `.planning/config.json` working-tree change was left untouched and uncommitted.

## Self-Check: PASSED

- Summary file exists at `.planning/phases/92-address-advertisement-and-discovery-boundaries/92-06-SUMMARY.md`.
- Task commits `cf071a4` and `b796267` exist in git history.
- `.planning/STATE.md`, `.planning/ROADMAP.md`, and `.planning/REQUIREMENTS.md` have no diff from this executor.
