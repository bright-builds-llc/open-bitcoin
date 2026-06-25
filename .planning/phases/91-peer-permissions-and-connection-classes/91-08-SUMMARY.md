---
phase: 91-peer-permissions-and-connection-classes
plan: 08
subsystem: network-tests
tags: [rust, p2p, peer-permissions, relay, safeguards]

requires:
  - phase: 91-01
    provides: "Inactive relay, mempool, bloom, and blockfilter permission labels"
provides:
  - "Negative peer message tests for inactive relay, forcerelay, and mempool permissions"
  - "Negative peer message tests for inactive bloomfilter, blockfilters, all expansion, and compact-block inventory"
  - "Service-bit regression coverage proving Phase 91 permissions do not advertise filter support"
affects:
  - 91-09-operator-docs-parity-roots-and-uat-commands
  - 91-10-deterministic-phase-checker-and-verifier-wiring

tech-stack:
  added: []
  patterns:
    - "Test inactive permission labels beside existing PeerManager message handling without adding production branches"
    - "Use --lib focused Cargo test filters to avoid unrelated integration-test binary startup stalls"

key-files:
  created:
    - .planning/phases/91-peer-permissions-and-connection-classes/91-08-SUMMARY.md
  modified:
    - packages/open-bitcoin-network/src/peer/tests.rs

key-decisions:
  - "Relay, forcerelay, and mempool permissions remain inactive labels and do not alter WtxidRelay, Inv, Tx, or GetData behavior."
  - "Bloomfilter, blockfilters, and all-expansion permissions do not change local service bits."
  - "CompactBlock inventory remains ignored by existing peer inventory handling and is not activated by permission labels."

requirements-completed: [PERM-03]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 91-2026-06-25T13-36-41
generated_at: 2026-06-25T18:56:56Z

duration: 11min
completed: 2026-06-25
---

# Phase 91 Plan 08: Relay, Mempool, Filter, and Compact-Block Negative Safeguards Summary

**PeerManager tests now prove Phase 91 relay-like and filter-like permission labels remain inert evidence instead of activating deferred network behavior.**

## Accomplishments

- Added a relay/mempool safeguard test that parses `in,relay,forcerelay,mempool`, verifies inactive labels, and asserts existing `WtxidRelay`, transaction `Inv`, `Tx`, and `GetData` behavior is unchanged.
- Added a filter/compact-block safeguard test that parses `in,all`, verifies inactive bloom/blockfilter labels, asserts local service bits remain `NETWORK | WITNESS`, and proves compact-block inventory stays ignored.
- Kept the production `PeerManager`, `WireNetworkMessage`, and service-bit model unchanged.

## Task Commits

1. **Task 1: Prove relay and mempool permissions stay inactive** - `2341d04` (`test`)
2. **Task 2: Prove bloom, blockfilter, and compact-block permissions stay inactive** - `2341d04` (`test`)

## Files Created/Modified

- `packages/open-bitcoin-network/src/peer/tests.rs` - Adds negative permission-safeguard tests and local permission decision helpers.

## Verification Results

- `cargo fmt --all` - passed
- `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-network --all-targets --all-features -- -D warnings` - passed
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network --no-run` - passed
- `timeout 90s cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network --lib relay_permission -- --nocapture` - passed
- `timeout 90s cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network --lib filter_permission -- --nocapture` - passed
- `timeout 90s cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network relay_permission -- --nocapture` - timed out locally at package test-binary execution, matching the Phase 91 integration-test runner stall; the focused `--lib` filter passed.
- `timeout 90s cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network filter_permission -- --nocapture` - ran the unit test successfully, then timed out while Cargo advanced to unrelated integration-test binaries; the focused `--lib` filter passed.
- `rg -n "inactive_relay|inactive_forcerelay|inactive_mempool|WtxidRelay|WireNetworkMessage::Tx|GetData" packages/open-bitcoin-network/src/peer/tests.rs` - passed
- `! rg -n "Mempool|Bloom|FilterLoad|compact filter|compact block relay" packages/open-bitcoin-network/src/peer.rs packages/open-bitcoin-network/src/message.rs` - passed
- `rg -n "inactive_bloomfilter|inactive_blockfilters|ServiceFlags::NETWORK \\| ServiceFlags::WITNESS|CompactBlock|all" packages/open-bitcoin-network/src/peer/tests.rs` - passed
- `! rg -n "NODE_BLOOM|BIP37|BIP157|FilterLoad|CFilters|CompactBlockRelay" packages/open-bitcoin-network/src/peer.rs packages/open-bitcoin-network/src/message.rs` - passed
- `git diff --check` - passed

## Deviations from Plan

- Used `cargo test --lib` for the executable filter checks after unscoped package-level filtered tests hit the known local integration-test binary startup stall.

## Next Phase Readiness

Plan 91-09 can document the no-claim boundary with deterministic evidence that relay-like, mempool, bloom/filter, and compact-block permissions are inactive labels only.
