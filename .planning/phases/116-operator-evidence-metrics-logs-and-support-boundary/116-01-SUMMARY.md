---
phase: 116-operator-evidence-metrics-logs-and-support-boundary
plan: 116-01
subsystem: shared-status-rpc
tags:
  - operator-evidence
  - rpc
  - block-relay
  - status
requires: []
provides:
  - Shared `BlockRelayEvidenceStatus` contract for downstream operator surfaces.
  - Managed network projection for block-serving and compact-relay aggregate evidence.
  - Open Bitcoin RPC `block_relay` surface on `openbitcoinnetworkstatus`.
affects:
  - node-status
  - managed-network
  - rpc-surface
  - parity-breadcrumbs
tech-stack:
  added: []
  patterns:
    - Shared status contracts stay aggregate-only and redact peer or block identifiers by construction.
    - Open Bitcoin-specific RPC surfaces carry operator evidence without changing baseline-compatible RPC methods.
key-files:
  created:
    - packages/open-bitcoin-node/src/status/block_relay_evidence.rs
    - packages/open-bitcoin-node/src/status/block_relay_evidence/tests.rs
    - packages/open-bitcoin-node/src/network/block_relay_evidence.rs
  modified:
    - packages/open-bitcoin-node/src/status.rs
    - packages/open-bitcoin-node/src/status/block_serving.rs
    - packages/open-bitcoin-node/src/network.rs
    - packages/open-bitcoin-rpc/src/context/network.rs
    - packages/open-bitcoin-rpc/src/dispatch/node.rs
    - packages/open-bitcoin-rpc/src/dispatch/tests.rs
    - packages/open-bitcoin-rpc/src/method/node.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - "`BlockRelayEvidenceStatus` composes existing block-serving evidence with compact-relay counter groups so every downstream surface reads one typed contract."
  - "Managed runtime projection collapses peer and block details into fixed aggregate counts before serialization."
  - "Only `openbitcoinnetworkstatus` gained block-relay evidence; baseline-compatible RPC shapes stayed unchanged."
requirements-completed:
  - OBS-01
generated_by: gsd-executor
lifecycle_mode: yolo
phase_lifecycle_id: 116-2026-07-06T03-46-36
generated_at: 2026-07-06T05:03:00Z
---

# Phase 116 Plan 01: Shared Block-Relay Status Contract Summary

Phase 116-01 established the shared block-relay truth surface that later CLI, dashboard, metrics, logs, and support work now consume.

## Accomplishments

- Added `BlockRelayEvidenceStatus` and its compact-relay counter groups with explicit unavailable semantics and aggregate-only in-flight fields.
- Wired `ManagedPeerNetwork::block_relay_evidence_status()` so runtime block serving, compact announcement, reconstruction, fallback, and cleanup paths project into one shared status contract.
- Exposed `block_relay` on `OpenBitcoinNetworkStatusResponse` through `openbitcoinnetworkstatus` while leaving baseline-compatible RPC methods unchanged.
- Extended parity breadcrumbs so the new node status and RPC evidence files remain auditable.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node block_relay -- --nocapture`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc open_bitcoin_network_status -- --nocapture`

## Deviations from Plan

None observed beyond standard implementation wiring.

## Self-Check

- Complete: shared status contract, managed runtime projection, and RPC exposure are present in the working tree.
- Passed: downstream Phase 116 work and later focused tests depend on this contract successfully.
