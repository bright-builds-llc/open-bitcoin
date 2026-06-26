---
phase: 93-eviction-ban-and-misbehavior-policy
plan: 02
subsystem: operator-status
tags: [rust, status, rpc, cli, metrics, support-bundle]

provides:
  - Shared inbound peer-policy status fields
  - Managed peer-policy projection from PeerManager evidence
  - RPC, operator status, dashboard metric, and support-bundle rendering
  - Support-bundle redaction for peer-policy latest-event text
affects: [phase-93-docs-checker, phase-93-verification]

tech-stack:
  added: []
  patterns:
    - Shared status contracts remain the source of truth for CLI and support renderers
    - Low-cardinality metric names and bounded latest-event labels only

key-files:
  created: []
  modified:
    - packages/open-bitcoin-node/src/status/inbound.rs
    - packages/open-bitcoin-node/src/network.rs
    - packages/open-bitcoin-node/src/network/inbound.rs
    - packages/open-bitcoin-node/src/metrics.rs
    - packages/open-bitcoin-rpc/src/context/network.rs
    - packages/open-bitcoin-rpc/src/dispatch/tests.rs
    - packages/open-bitcoin-cli/src/operator/status/render/inbound.rs
    - packages/open-bitcoin-cli/src/operator/support/render/inbound.rs
    - packages/open-bitcoin-cli/src/operator/support/redaction.rs
    - packages/open-bitcoin-cli/src/operator/status/tests.rs
    - packages/open-bitcoin-cli/src/operator/status/render/tests.rs
    - packages/open-bitcoin-cli/src/operator/support/tests.rs
    - packages/open-bitcoin-cli/src/operator/dashboard/model.rs

requirements-completed: [EVICT-02, EVICT-03, EVICT-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 93-2026-06-26T13-15-10
generated_at: 2026-06-26T13:55:51Z

completed: 2026-06-26
---

# Phase 93 Plan 02: Peer Policy Status Summary

Implemented shared peer-policy status, RPC projection, metrics, human status rendering, and support-bundle rendering/redaction.

## Accomplishments

- Extended `InboundPeerServingStatus` with peer-policy counters and `latest_peer_policy_decision`.
- Added `ManagedPeerPolicyInfo` to project bounded eviction, ban, unban, and misbehavior evidence without raw peer ids or raw ban scopes.
- Exposed peer-policy evidence through `openbitcoinnetworkstatus`, operator status, support Markdown/JSON, and dashboard metric labels.
- Added redaction for raw-looking peer-policy text in support bundles.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node managed_peer_policy_info --no-fail-fast`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node inbound_status_ --no-fail-fast`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node inbound_metric_kinds_are_low_cardinality_counters --no-fail-fast`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc open_bitcoin_network_status --no-fail-fast`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli phase93_peer_policy --no-fail-fast`

## Deviations

- Task-level commits were intentionally deferred. The invoked wrapper commits only after phase verification passes.

## Self-Check: PASSED

- Status schema defaults preserve legacy JSON deserialization.
- Focused node, RPC, CLI status, and support tests passed.
- Support-bundle rendering uses bounded labels and redacts raw-looking peer-policy material.

---
*Phase: 93-eviction-ban-and-misbehavior-policy*
*Completed: 2026-06-26*
