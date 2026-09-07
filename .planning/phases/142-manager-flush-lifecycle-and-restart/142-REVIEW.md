---
phase: 142-manager-flush-lifecycle-and-restart
reviewed: 2026-09-07T02:15:00Z
depth: standard
files_reviewed: 112
files_reviewed_list:
  - docs/parity/source-breadcrumbs.json
  - packages/open-bitcoin-chainstate/src/coins.rs
  - packages/open-bitcoin-chainstate/src/coins/cache.rs
  - packages/open-bitcoin-chainstate/src/coins/memory.rs
  - packages/open-bitcoin-chainstate/src/coins/tests.rs
  - packages/open-bitcoin-chainstate/src/coins/tests/flush.rs
  - packages/open-bitcoin-chainstate/src/engine.rs
  - packages/open-bitcoin-chainstate/src/engine/overlay_apply.rs
  - packages/open-bitcoin-chainstate/src/engine/stage.rs
  - packages/open-bitcoin-chainstate/src/error.rs
  - packages/open-bitcoin-chainstate/src/lib.rs
  - packages/open-bitcoin-node/src/chainstate.rs
  - packages/open-bitcoin-node/src/chainstate/fjall_store.rs
  - packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs
  - packages/open-bitcoin-node/src/chainstate/flush_lifecycle/tests.rs
  - packages/open-bitcoin-node/src/chainstate/flush_lifecycle/tests/error_map.rs
  - packages/open-bitcoin-node/src/chainstate/replay.rs
  - packages/open-bitcoin-node/src/chainstate/replay/tests.rs
  - packages/open-bitcoin-node/src/chainstate/tests.rs
  - packages/open-bitcoin-node/src/lib.rs
  - packages/open-bitcoin-node/src/mempool.rs
  - packages/open-bitcoin-node/src/network.rs
  - packages/open-bitcoin-node/src/network/action_translation.rs
  - packages/open-bitcoin-node/src/network/admission_bridge.rs
  - packages/open-bitcoin-node/src/network/admission_bridge/local_package.rs
  - packages/open-bitcoin-node/src/network/admission_bridge/package.rs
  - packages/open-bitcoin-node/src/network/admission_bridge/singleton.rs
  - packages/open-bitcoin-node/src/network/announcement_transport.rs
  - packages/open-bitcoin-node/src/network/block_relay_evidence.rs
  - packages/open-bitcoin-node/src/network/checkpoint.rs
  - packages/open-bitcoin-node/src/network/compact_receive_candidates.rs
  - packages/open-bitcoin-node/src/network/inbound.rs
  - packages/open-bitcoin-node/src/network/inventory.rs
  - packages/open-bitcoin-node/src/network/lifecycle_projection.rs
  - packages/open-bitcoin-node/src/network/lifecycle_projection/authority.rs
  - packages/open-bitcoin-node/src/network/lifecycle_projection/reconciliation.rs
  - packages/open-bitcoin-node/src/network/lifecycle_projection/recovery.rs
  - packages/open-bitcoin-node/src/network/mempool_lifecycle.rs
  - packages/open-bitcoin-node/src/network/operator_snapshot.rs
  - packages/open-bitcoin-node/src/network/peer_network_clone.rs
  - packages/open-bitcoin-node/src/network/peer_policy.rs
  - packages/open-bitcoin-node/src/network/recovery.rs
  - packages/open-bitcoin-node/src/network/relay_fanout.rs
  - packages/open-bitcoin-node/src/network/relay_fanout/lifecycle.rs
  - packages/open-bitcoin-node/src/network/relay_serving.rs
  - packages/open-bitcoin-node/src/network/runtime_authority.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/effects.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/error.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/lifecycle.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/local_package.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/maintenance.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/recovery.rs
  - packages/open-bitcoin-node/src/network/tests/mempool_lifecycle_cases/reorg_reject_evidence.rs
  - packages/open-bitcoin-node/src/storage.rs
  - packages/open-bitcoin-node/src/storage/coins_view.rs
  - packages/open-bitcoin-node/src/storage/coins_view/tests.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/coins.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/mempool.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/tests/coins_migration.rs
  - packages/open-bitcoin-node/src/sync.rs
  - packages/open-bitcoin-node/src/sync/open_runtime.rs
  - packages/open-bitcoin-node/src/sync/runtime_state.rs
  - packages/open-bitcoin-node/src/sync/runtime_state/helpers.rs
  - packages/open-bitcoin-node/src/sync/session.rs
  - packages/open-bitcoin-node/src/sync/session/emission_terminal.rs
  - packages/open-bitcoin-node/src/sync/tests.rs
  - packages/open-bitcoin-node/src/sync/tests/block_response/connection_progress.rs
  - packages/open-bitcoin-node/src/sync/tests/reorg_reconciliation.rs
  - packages/open-bitcoin-node/src/sync/tests/restart_chainstate.rs
  - packages/open-bitcoin-node/src/sync/tests/soak.rs
  - packages/open-bitcoin-node/src/sync/tests/stay_current_progress.rs
  - packages/open-bitcoin-node/src/sync/tests/support_blocks.rs
  - packages/open-bitcoin-node/src/sync/tests/support_runtime.rs
  - packages/open-bitcoin-node/src/sync/tests/synthetic_long_chain.rs
  - packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/checkpoint.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush/tests.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/inbound_metrics.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/inbound_startup.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/retry.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/daemon_sync.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/retry.rs
  - packages/open-bitcoin-rpc/src/context.rs
  - packages/open-bitcoin-rpc/src/context/inbound_status.rs
  - packages/open-bitcoin-rpc/src/context/inbound_wire.rs
  - packages/open-bitcoin-rpc/src/context/mempool_recovery.rs
  - packages/open-bitcoin-rpc/src/context/network.rs
  - packages/open-bitcoin-rpc/src/context/peer_policy.rs
  - packages/open-bitcoin-rpc/src/context/rescan.rs
  - packages/open-bitcoin-rpc/src/context/resource_governance.rs
  - packages/open-bitcoin-rpc/src/context/wallet_state.rs
  - packages/open-bitcoin-rpc/src/dispatch.rs
  - packages/open-bitcoin-rpc/src/dispatch/node.rs
  - packages/open-bitcoin-rpc/src/dispatch/package.rs
  - packages/open-bitcoin-rpc/src/dispatch/wallet.rs
  - packages/open-bitcoin-rpc/src/http.rs
  - packages/open-bitcoin-rpc/src/http/request.rs
  - packages/open-bitcoin-rpc/src/inbound_listener.rs
  - packages/open-bitcoin-rpc/src/inbound_listener/connection_runtime.rs
  - packages/open-bitcoin-rpc/src/inbound_listener/resource_runtime.rs
  - packages/open-bitcoin-rpc/src/inbound_listener/tests.rs
  - packages/open-bitcoin-rpc/tests/black_box_parity/phase127_composition.rs
  - scripts/check-phase123-runtime-timing-evidence-integrity.test.ts
  - scripts/check-phase123-runtime-timing-evidence-integrity/checks.ts
  - scripts/check-phase123-runtime-timing-evidence-integrity/constants.ts
  - scripts/check-phase127-authoritative-network-state-unification.test.ts
  - scripts/check-phase127-authoritative-network-state-unification.ts
  - scripts/check-phase128-production-compact-announcement-transport.ts
  - scripts/check-phase134-authoritative-lifecycle.ts
  - scripts/check-phase135-snapshot-recovery.test.ts
  - scripts/check-phase135-snapshot-recovery.ts
findings:
  critical: 0
  warning: 6
  info: 3
  total: 9
status: issues
---

# Phase 142: Code Review Report

**Reviewed:** 2026-09-07T02:15:00Z
**Depth:** standard
**Files Reviewed:** 112
**Status:** issues

## Summary

Phase 142's flush lifecycle, interrupted-H replay, generic `Chainstate<V: CoinsView>`, and persist-progress cutover are coherent. Typed `InterruptedWrite` remap, leftover-empty skip on two-element `H`, `from_coins_cache` open, and coins-B progress gating look correct. Most of the git-range ripple is generic type plumbing and is not independently buggy.

The remaining issues sit on crash-consistency and silent-failure paths: replay persist can rewrite `H=[new, old]` to `H=[new, 0]` when `B` is missing; `persist()` discards flush errors; `FlushDecision::Sync` never reaches `PersistMode::Sync`; disk-space probing is a stub; admission UTXO scans fail open; and a `getrandom` miss after Periodic write can flush every second.

This review is advisory and does not block the phase.

## Warnings

### WR-01: Replay persist clobbers interrupted old-head marker

**File:** `packages/open-bitcoin-node/src/storage/coins_view.rs:196-214`
**Issue:** `batch_write_with_persist_mode` (replay's finish path) sets `allow_existing_heads = true`, then derives `old_tip` only from current `B`. Interrupted replay has `H=[new, old]` and no `B`, so `old_tip` becomes the zero hash. The first buffered commit writes `H=[new, 0]`, destroying the original old head. A crash after that commit makes the next replay treat the write as a first-flush (`old == 0`), skip rollback, and apply-only roll forward. On a reorg interrupt that leaves stale old-branch UTXOs that the new branch does not overwrite.
**Fix:** When `B` is missing, keep the existing two-element `H` pair until the final marker finish. Example:

```rust
let old_tip = match self.coins_get(&encode_best_block_key()).map_err(map_storage)? {
    Some(bytes) => decode_best_block_value(&bytes).map_err(map_storage)?,
    None => match self.classify_markers().map_err(map_storage)? {
        MarkerState::Interrupted { old, .. } => old,
        _ => BlockHash::from_byte_array([0_u8; 32]),
    },
};
```

Alternatively, if `allow_existing_heads` and `H` is already present, do not rewrite `H` until the final `H`-clear / `B`-write batch.

### WR-02: `persist()` swallows every flush error

**File:** `packages/open-bitcoin-node/src/chainstate.rs:416-425`
**Issue:** Connect, disconnect, reorg, and prepared-commit all call `persist()`, which does `let _ = self.flush_with_mode(...)`. A failed coins write (interrupted `H` already present, backend error, or `persist_chain_meta` after a successful coins write) is dropped. The in-memory tip still advances. Later `IfNeeded` / Periodic / Always attempts then hit `allow_existing_heads = false` and fail again, still silently on the connect path. After coins succeed and meta fails, reopen hydrates a stale `active_chain` against a newer coins `B`.
**Fix:** Propagate or at least log the `StorageError`. Minimum connect-path change:

```rust
fn persist(&mut self) -> Result<(), StorageError>
where
    S: FlushPersistSink,
{
    self.flush_with_mode(
        FlushMode::IfNeeded,
        FlushPolicyTime::from_unix_seconds(0),
        u64::MAX,
    )
    .map(|_| ())
}
```

Then have `connect_block` / `commit_prepared_*` map that error instead of ignoring it.

### WR-03: `FlushDecision::Sync` does not use `PersistMode::Sync`

**File:** `packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs:192-194`
**Issue:** `execute_flush` maps Sync to `cache.sync()`. `CoinsCache::sync` (`cache.rs:383-386`) calls `parent.batch_write`, and `FjallCoinsView::batch_write` always uses `persist_mode_for_final_best_block()` (`Flush`). Replay is the only path that calls `batch_write_with_persist_mode(..., PersistMode::Sync)`. Periodic-due Sync writes therefore get the same Fjall `Buffer` durability as Flush. Overlay retention differs (`flush` clears, `sync` keeps clean unspent), but fsync does not.
**Fix:** Thread persist mode through the cache write, or special-case Fjall on the Sync branch:

```rust
CoinsWriteKind::Sync => {
    cache.sync_with_persist_mode(PersistMode::Sync).map_err(map_chainstate)?
}
```

`CoinsView` needs a persist-mode write, or `FjallCoinsView` must be flushed via `batch_write_with_persist_mode` after `into_dirty_parent_write`.

### WR-04: Production disk-space guard is never armed

**File:** `packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs:236-239`
**Issue:** `probe_disk_free_bytes` ignores `datadir` and returns `u64::MAX`. The daemon worker (`coins_flush.rs:149`) injects `probe_disk_free_bytes(Path::new("."))`, so even a later real `statvfs` would probe cwd, not the datadir. `RefuseDiskSpace` cannot fire on Periodic / Always / `persist()` (`persist` also hardcodes `u64::MAX`).
**Fix:** Probe the actual datadir from a crate that may use `statvfs` (or a safe std equivalent), and pass that value into `flush_coins` / `execute_flush`. Keep `u64::MAX` only for explicit unprobed test paths.

```rust
let disk_free_bytes = probe_disk_free_bytes(store.datadir());
handle.flush_coins(mode, now, disk_free_bytes)
```

### WR-05: `collect_unspent_hint` fails open to an empty parent UTXO set

**File:** `packages/open-bitcoin-node/src/storage/coins_view.rs:317-331`
**Issue:** Any prefix-scan or decode error returns `HashMap::new()` instead of failing. `admission_snapshot` merges this hint with the overlay. A transient decode error makes confirmed parent UTXOs disappear from admission, so spends of those coins look missing. Not a double-spend (safer than inventing coins), but it silently changes mempool admission after a corrupt or unreadable record.
**Fix:** Fail closed. Change the hint to `Result`, or treat scan failure as `ChainstateError::CoinsStorage` at the admission boundary rather than an empty map.

```rust
fn collect_unspent_hint(&self) -> HashMap<OutPoint, Coin> {
    // Prefer a Result-returning collect used by admission_snapshot.
}
```

Until the trait can change, log and surface storage corruption instead of returning empty.

### WR-06: Failed Periodic resample leaves `next_write` in the past

**File:** `packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush.rs:155-169`
**Issue:** After a Periodic coins write, `resample_periodic_next_write` errors (getrandom) are ignored, and `set_coins_next_write` errors are ignored. `next_write` stays due. The worker ticks every 1s (`TICK_SECS`), so Periodic keeps writing every second until resample succeeds. `drive_periodic` also only `eprintln`s flush failures and continues.
**Fix:** On resample failure, set a conservative fallback `next_write` (e.g. `now + PERIODIC_WRITE_MIN_SECS`) and surface the error. Do not return early leaving a due timestamp.

```rust
let next_write = resample_periodic_next_write(now)
    .unwrap_or_else(|_| FlushPolicyTime::from_unix_seconds(now.saturating_add(PERIODIC_WRITE_MIN_SECS)));
let _ = handle.set_coins_next_write(next_write);
```

Prefer returning the `set_coins_next_write` error to the worker loop.

## Info

### IN-01: Stale leftover-write comment after persist cutover

**File:** `packages/open-bitcoin-node/src/storage/fjall_store/coins.rs:206-207`
**Issue:** Comment still says production persist writes leftover only and that Phase 142 owns write-site cutover. Plan 06 removed leftover snapshot writes from `persist_progress`.
**Fix:** Update the comment to say leftover blobs may remain unread and are not production persist authority.

### IN-02: Replay maps `vout` overflow to `u32::MAX`

**File:** `packages/open-bitcoin-node/src/chainstate/replay.rs:201-203`
**Issue:** `u32::try_from(vout).unwrap_or(u32::MAX)` would alias a theoretically huge output index to `MAX` instead of fail-closing. Not reachable for Bitcoin-sized transactions.
**Fix:** Return `interrupted_write()` on conversion failure.

### IN-03: Coins flush errors drop authority detail

**File:** `packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush.rs:38-41`
**Issue:** `From<ManagedNetworkAuthorityError>` always becomes `CoinsFlushError::Authority`. Periodic logs a generic string; Always maps to `ShutdownCheckpoint`. Operators lose the underlying storage message.
**Fix:** Keep the source string on the error variant, same as `ManagedNetworkAuthorityError::LifecycleEffect`.

---

_Reviewed: 2026-09-07T02:15:00Z_
_Reviewer: Claude (gsd-code-reviewer)_
_Depth: standard_
