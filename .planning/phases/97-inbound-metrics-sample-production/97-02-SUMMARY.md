---
phase: 97-inbound-metrics-sample-production
plan: 02
subsystem: sync-runtime
tags: [rust, metrics, fjall, daemon, status]
requires:
  - phase: 97-inbound-metrics-sample-production
    plan: 01
    provides: Pure inbound metric sample mapper.
provides:
  - Runtime append path that persists sync and inbound samples together.
  - Daemon provider hook from ManagedRpcContext current inbound status to DurableSyncRuntime.
  - Sync-disabled inbound listener metrics worker that writes retained samples on the default retention interval.
affects: [durable-sync-runtime, open-bitcoind, metrics-history]
tech-stack:
  added: []
  patterns: [provider-hook, single-append-path, unavailable-status-empty-samples]
key-files:
  created:
    - packages/open-bitcoin-node/src/sync/metrics.rs
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/inbound_metrics.rs
    - packages/open-bitcoin-rpc/src/context/inbound_status.rs
  modified:
    - packages/open-bitcoin-node/src/sync.rs
    - packages/open-bitcoin-node/src/sync/runtime_state.rs
    - packages/open-bitcoin-node/src/sync/tests.rs
    - packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs
key-decisions:
  - "Extend the existing metric sample vector before one Fjall append call instead of adding a second metrics store."
  - "Use try_lock on the shared RPC context and return unavailable inbound status when the sync provider context is busy."
  - "Share one opened Fjall store between RPC context, daemon sync, and inbound metrics workers to avoid test and runtime store-lock contention."
requirements-completed: [INB-05, DOS-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 97-2026-06-28T16-11-36
generated_at: 2026-06-28T17:15:16Z
duration: 28min
completed: 2026-06-28
---

# Phase 97 Plan 02: Runtime Metrics Persistence Summary

**Runtime metrics persistence now appends retained inbound samples through the existing Fjall metrics history path.**

## Accomplishments

- Added `DurableSyncRuntime::set_inbound_metric_status_provider`.
- Moved metrics persistence into `sync/metrics.rs` and extended the append batch from sync samples plus status-derived inbound samples.
- Wired `open-bitcoind` so the daemon sync worker samples `ManagedRpcContext::current_inbound_status()` through a non-blocking provider.
- Added `open_bitcoind/inbound_metrics.rs` for the dedicated sync-disabled inbound listener metrics worker.
- Moved inbound status projection into `context/inbound_status.rs` while keeping the public context method unchanged.
- Added store-aware RPC context construction so live status and background workers can share the same Fjall metrics history.
- Added runtime tests for available inbound samples and unavailable inbound status.
- Added an `open-bitcoind` regression test proving a sync-disabled inbound listener run persists retained inbound samples.
- Split persistence, worker, and status projection code into focused modules to keep production Rust files under the repo file-length gate.

## Task Commits

Deferred until the wrapper-level clean verification gate.

## Files Created/Modified

- `packages/open-bitcoin-node/src/sync.rs` - Adds the inbound status provider hook.
- `packages/open-bitcoin-node/src/sync/metrics.rs` - Extends the existing metrics append batch.
- `packages/open-bitcoin-node/src/sync/runtime_state.rs` - Exposes summary construction to the metrics module.
- `packages/open-bitcoin-node/src/sync/tests.rs` - Adds retained metrics persistence tests.
- `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs` - Wires daemon sync runtime to the shared RPC context provider and starts the sync-disabled inbound metrics worker.
- `packages/open-bitcoin-rpc/src/bin/open_bitcoind/inbound_metrics.rs` - Persists retained inbound samples when inbound is enabled and sync is disabled.
- `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests.rs` - Verifies sync-disabled inbound metrics persistence.
- `packages/open-bitcoin-rpc/src/context.rs`, `packages/open-bitcoin-rpc/src/context/network.rs`, `packages/open-bitcoin-rpc/src/context/inbound_status.rs`, and `packages/open-bitcoin-rpc/src/context/wallet_state.rs` - Share a pre-opened Fjall store with context metrics status and project current inbound status.

## Deviations from Plan

- Runtime retention tests were added in `packages/open-bitcoin-node/src/sync/tests.rs` for the sync append contract, and an `open_bitcoind/tests.rs` worker test was added after review to cover the documented sync-disabled inbound listener path.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node persist_metrics_appends_inbound_status_samples_with_sync_samples -- --nocapture`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node persist_metrics_omits_inbound_samples_when_status_unavailable -- --nocapture`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc open_bitcoind_inbound_metrics_worker_persists_sync_disabled_inbound_samples -- --nocapture`

## User Setup Required

None.
