---
phase: 108-durable-mempool-relay-state-recovery
plan: 01
subsystem: network-recovery
tags:
  - mempool
  - relay-serving
  - relay-fanout
  - startup-recovery
requires:
  - phase: 103-mempool-chainstate-lifecycle-and-durable-recovery
    provides: durable Open Bitcoin mempool snapshot replay
  - phase: 107-runtime-relay-activation-and-download-eligibility-integration
    provides: coherent managed relay activation and eligibility state
provides:
  - Managed durable mempool snapshot recovery API
  - No-socket relay fanout identity seeding for recovered transactions
  - Startup snapshot load hook for managed RPC context construction
  - Sanitized storage-error recovery category evidence
affects:
  - packages/open-bitcoin-node/src/network/recovery.rs
  - packages/open-bitcoin-node/src/network/relay_fanout.rs
  - packages/open-bitcoin-rpc/src/context/network.rs
  - packages/open-bitcoin-node/src/network/tests/recovery_cases.rs
  - packages/open-bitcoin-rpc/src/context/tests.rs
tech-stack:
  added: []
  patterns:
    - Reuse durable snapshot replay before mutating managed relay indexes
    - Seed relay fanout identity without enqueueing or draining peer fanout
    - Keep startup recovery non-fatal and observable through sanitized status evidence
key-files:
  created:
    - packages/open-bitcoin-node/src/network/recovery.rs
    - packages/open-bitcoin-node/src/network/tests/recovery_cases.rs
  modified:
    - packages/open-bitcoin-node/src/network.rs
    - packages/open-bitcoin-node/src/network/relay_fanout.rs
    - packages/open-bitcoin-node/src/network/relay_serving.rs
    - packages/open-bitcoin-node/src/network/tests.rs
    - packages/open-bitcoin-rpc/src/context/network.rs
    - packages/open-bitcoin-rpc/src/context/tests.rs
    - docs/parity/source-breadcrumbs.json
    - docs/metrics/lines-of-code.md
key-decisions:
  - "Recovered accepted records re-enter the existing managed transaction store and relay-serving cache instead of a recovery-only cache."
  - "Recovered fanout state seeds txid/wtxid identity only; it does not enqueue, drain, translate, or emit peer socket actions."
  - "Startup snapshot load failures are recorded as typed recovery categories and do not expose raw storage detail strings."
requirements-completed:
  - MEM-06
  - REL-01
  - REL-02
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 108-2026-07-03T14-09-06
generated_at: 2026-07-03T15:32:37Z
completed: 2026-07-03
---

# Phase 108 Plan 01 Summary

Managed durable mempool recovery now rehydrates local mempool, relay-serving, and fanout identity state without startup socket I/O.

## Accomplishments

- Added `ManagedMempoolRecoverySummary` and `ManagedPeerNetwork::recover_mempool_snapshot`.
- Replayed `MempoolSnapshot` records through the existing mempool recovery classifier and only stored accepted recovered transactions in managed relay state.
- Added `ManagedRelayFanoutState::seed_recovered_transaction`, which records txid/wtxid identity without enqueueing fanout actions.
- Wired managed RPC context construction to load an existing Fjall mempool snapshot and recover it during startup.
- Recorded mempool recovery storage-open/load errors as sanitized `SyncRecoveryCategory` evidence.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node recovery -- --nocapture` - passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc managed_rpc_context_loads_durable_mempool_snapshot_on_startup -- --nocapture` - passed.
- `bun run scripts/check-parity-breadcrumbs.ts --check` - passed.

## Deviations

- The generated plan named two RPC startup tests; the implemented coverage uses one startup snapshot-load regression plus node recovery tests for accepted and non-accepted records.
- Storage action is not exposed as a public recovery field; the operator-facing unavailable state remains bounded to the sanitized recovery category and fixed unavailable reason.

## Residual Boundaries

No public relay default, socket fanout, `inv` emission, compact block relay, package relay, bloom/filter serving, destructive repair, source datadir mutation, compaction, reindex, store surgery, or automatic support upload was added.
