---
phase: 97-inbound-metrics-sample-production
reviewed: 2026-06-28T18:20:03Z
depth: standard
files_reviewed: 33
files_reviewed_list:
  - packages/open-bitcoin-node/src/metrics.rs
  - packages/open-bitcoin-node/src/metrics/tests.rs
  - packages/open-bitcoin-node/src/status/inbound.rs
  - packages/open-bitcoin-node/src/status/inbound/tests.rs
  - packages/open-bitcoin-node/src/status/tests.rs
  - packages/open-bitcoin-node/src/sync.rs
  - packages/open-bitcoin-node/src/sync/metrics.rs
  - packages/open-bitcoin-node/src/sync/runtime_state.rs
  - packages/open-bitcoin-node/src/sync/tests.rs
  - packages/open-bitcoin-rpc/src/config.rs
  - packages/open-bitcoin-rpc/src/config/loader.rs
  - packages/open-bitcoin-rpc/src/config/loader/open_bitcoin_runtime.rs
  - packages/open-bitcoin-rpc/src/context.rs
  - packages/open-bitcoin-rpc/src/context/inbound_status.rs
  - packages/open-bitcoin-rpc/src/context/network.rs
  - packages/open-bitcoin-rpc/src/context/wallet_state.rs
  - packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/inbound_metrics.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests.rs
  - packages/open-bitcoin-rpc/src/dispatch/node.rs
  - packages/open-bitcoin-rpc/src/method/node.rs
  - packages/open-bitcoin-cli/src/operator/dashboard/model.rs
  - packages/open-bitcoin-cli/src/operator/dashboard/model/metrics.rs
  - packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs
  - packages/open-bitcoin-cli/src/operator/status.rs
  - packages/open-bitcoin-cli/src/operator/status/tests.rs
  - packages/open-bitcoin-cli/src/operator/status/render/tests.rs
  - packages/open-bitcoin-cli/src/operator/support/tests.rs
  - docs/architecture/operator-observability.md
  - docs/operator/runtime-guide.md
  - scripts/check-phase97-inbound-metrics.ts
  - scripts/check-phase97-inbound-metrics.test.ts
  - scripts/verify.sh
findings:
  critical: 0
  warning: 0
  info: 0
  total: 0
status: passed
---

# Phase 97: Code Review Report

**Reviewed:** 2026-06-28T18:20:03Z
**Depth:** standard
**Files Reviewed:** 33
**Status:** passed

## Summary

Reviewed the Phase 97 inbound metrics implementation against the repo-local guidance in `AGENTS.md`, `AGENTS.bright-builds.md`, `standards/core/verification.md`, `standards/core/testing.md`, `standards/core/code-shape.md`, `standards/languages/rust.md`, and `standards/languages/typescript-javascript.md`. The fixed `MetricKind` projection, Fjall append call, sync-disabled inbound metrics worker, RPC/live-status metrics projection, bounded dashboard chart list, extracted file-length modules, status/support serialization tests, checker, and verifier wiring are present.

The prior warning is resolved. The documented regtest inbound listener review path now has an active metrics producer even when daemon sync is disabled.

The final file-length cleanup was reviewed after the warning fix; `bash scripts/check-file-lengths.sh` passed with all production Rust files below the configured limit.

## Resolution

WR-01 was fixed by adding `start_inbound_metrics_worker` and `persist_inbound_metrics_once` in `open-bitcoind`, sharing the opened Fjall store with `ManagedRpcContext::from_runtime_config_with_store`, exposing retained metrics through `openbitcoinnetworkstatus`, and mapping those metrics into live CLI status snapshots. Targeted regression checks passed:

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc open_bitcoind_inbound_metrics_worker_persists_sync_disabled_inbound_samples -- --nocapture`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli fake_live_rpc_maps_metrics_from_open_bitcoin_network_status -- --nocapture`
- `bun test scripts/check-phase97-inbound-metrics.test.ts`
- `bun run scripts/check-phase97-inbound-metrics.ts`

## Resolved Warnings

### WR-01: Documented inbound-only review path never produces retained inbound metric samples

**File:** `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs:306`
**Issue:** `start_daemon_sync_worker` returns `Ok(None)` whenever `runtime.sync.is_enabled()` is false, and the only production call to `set_inbound_metric_status_provider` lives below that guard. The Phase 97 runtime guide documents `-regtest` inbound listener/status/dashboard/support commands without enabling daemon sync, so those workflows can expose current inbound status but never append retained inbound metric samples to Fjall. The Phase 97 checker still passes because it verifies string presence, not that the documented inbound-only runtime path has a sampler.
**Fix:** Start a dedicated bounded metrics sampler for the inbound listener path, or explicitly scope the Phase 97 docs/checker to sync-enabled runtime only. A code fix should append status-derived inbound samples on the configured retention interval even when daemon sync is disabled:

```rust
fn start_inbound_metrics_worker(
    runtime: &RuntimeConfig,
    shared_context: Arc<tokio::sync::Mutex<ManagedRpcContext>>,
) -> Result<Option<InboundMetricsWorker>, DaemonSyncPreflightError> {
    if !runtime.inbound.enabled {
        return Ok(None);
    }

    let Some(data_dir) = runtime.maybe_data_dir.as_ref() else {
        return Ok(None);
    };

    let store = FjallNodeStore::open(data_dir).map_err(|error| {
        DaemonSyncPreflightError::new(format!(
            "open-bitcoind inbound metrics failed to open durable store at \"{}\": {error}",
            data_dir.display()
        ))
    })?;

    let retention = MetricRetentionPolicy::default();
    let persist_mode = runtime.sync.runtime.persist_mode;
    let (shutdown_sender, shutdown_receiver) = mpsc::channel();
    let join_handle = thread::spawn(move || loop {
        let timestamp = current_timestamp_unix_seconds();
        let inbound = shared_context.blocking_lock().current_inbound_status();
        let timestamp = u64::try_from(timestamp).unwrap_or(0);
        let samples = inbound_metric_samples(&inbound, timestamp);
        if !samples.is_empty() {
            let _ = store.append_metric_samples(&samples, retention, timestamp, persist_mode);
        }
        if shutdown_receiver
            .recv_timeout(Duration::from_secs(retention.sample_interval_seconds.max(1)))
            .is_ok()
        {
            break;
        }
    });

    Ok(Some(InboundMetricsWorker {
        join_handle,
        shutdown_sender,
    }))
}
```

Add an integration-style test that starts the regtest inbound listener path with sync disabled, records inbound evidence, advances one metrics tick, and asserts `FjallNodeStore::load_metrics_snapshot()` contains at least one `MetricKind::Inbound*` sample. Update `scripts/check-phase97-inbound-metrics.ts` to require that non-sync sampler or change `docs/operator/runtime-guide.md` so the advertised commands match the actual sync-only producer.

---

_Reviewed: 2026-06-28T17:19:06Z_
_Reviewer: the agent (gsd-code-reviewer)_
_Depth: standard_
