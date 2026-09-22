---
phase: 146-wallet-leftover-snapshot-cutover
reviewed: 2026-09-22T05:07:00Z
depth: standard
files_reviewed: 9
files_reviewed_list:
  - packages/open-bitcoin-bench/src/cases/wallet_rescan.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/coins.rs
  - packages/open-bitcoin-node/src/sync/tests/wallet_rescan_runtime.rs
  - packages/open-bitcoin-node/src/sync/wallet_rescan.rs
  - packages/open-bitcoin-rpc/src/context/network.rs
  - packages/open-bitcoin-rpc/src/context/rescan.rs
  - packages/open-bitcoin-rpc/src/context/tests.rs
  - packages/open-bitcoin-rpc/src/context/tests/construction.rs
  - scripts/check-phase135-snapshot-recovery.ts
findings:
  critical: 1
  warning: 2
  info: 2
  total: 5
status: issues
---

# Phase 146: Code Review Report

**Reviewed:** 2026-09-22T05:07:00Z
**Depth:** standard
**Files Reviewed:** 9
**Status:** issues

## Summary

Phase 146 cutover largely lands: `WalletRescanRuntime` and durable RPC startup seed from `wallet_scan_chainstate_snapshot` (coins + `chain_meta`, never leftover `"snapshot"`), `has_block` fail-closes missing payloads on the durable rescan paths, disagreeing leftover regressions exist, and source-string guards block `have_pruned` / `NODE_NETWORK_LIMITED` invention.

One critical correctness gap remains: the new wallet-scan helper does not fail closed on interrupted two-head coins markers the way `hydrate_chainstate_for_open` does, so a crash mid-coins-write can still feed wallet/RPC scan truth. Two warnings cover incomplete D-04 payload gating for out-of-range creating heights and inconsistent job failure marking when `has_block` itself errors.

## Critical Issues

### CR-01: `wallet_scan_chainstate_snapshot` fails open on interrupted coins writes

**File:** `packages/open-bitcoin-node/src/storage/fjall_store/coins.rs:105-133`
**Issue:** `hydrate_chainstate_for_open` (lines 74–78) rejects `RecoveryDecision::InterruptedTwoHeads`. The new wallet-scan helper never calls `head_blocks()` / `decide_recovery`. During an interrupted write, `FjallCoinsView::best_block` returns `None`, so the tip-agreement check is skipped, `scan_coin_records()` still returns whatever coin rows exist, and callers (`WalletRescanRuntime`, durable RPC seed in `network.rs`) can treat a torn coins keyspace as scan authority. That can credit wrong wallet UTXOs/balances after a crash.
**Fix:** Fail closed the same way hydrate does before scanning coins:

```rust
pub fn wallet_scan_chainstate_snapshot(
    &self,
) -> Result<Option<ChainstateSnapshot>, StorageError> {
    let view = FjallCoinsView::from_store(self);
    let heads = view.head_blocks().map_err(map_heads_error)?;
    if matches!(
        decide_recovery(heads.len()),
        RecoveryDecision::InterruptedTwoHeads
    ) {
        return Err(interrupted_coins_write());
    }

    if self.coins_keyspace_is_empty()? {
        return Ok(None);
    }
    // ... existing chain_meta + tip agreement logic ...
}
```

## Warnings

### WR-01: D-04 payload gate only covers the active scan window, not every admitted UTXO creating height

**File:** `packages/open-bitcoin-node/src/sync/wallet_rescan.rs:121-137` and `packages/open-bitcoin-rpc/src/context/rescan.rs:165-182`
**Issue:** `has_block` is checked only for heights in `[next_height, chunk_end]` / `[start_height, stop_height]`. `partial_chainstate_snapshot` then admits every coin with `created_height <= through_height`, and `rescan_chainstate` replaces the full wallet UTXO set from that set. On chunk resume or `rescanblockchain` with `start_height > 0`, UTXOs whose creating height is outside the gated window are still applied from coins without a creating-payload probe. That violates CONTEXT D-04 (“Coins best-block alone does not authorize an entry whose creating payload is missing”) when early payloads were never present or were removed between chunks.
**Fix:** Before `rescan_chainstate`, require payload presence for every height that contributes an admitted UTXO (or every `active_chain` height `<= through_height`), not only the current window:

```rust
for position in chainstate.active_chain.iter().filter(|position| {
    position.height <= chunk_end_height
}) {
    if !self.store.has_block(position.block_hash)? {
        // mark_failed + return Unavailable as today
    }
}
```

(Or filter partial UTXOs to `created_height` inside the gated window and merge incrementally instead of full replace.)

### WR-02: `has_block` storage errors leave rescan jobs Pending while `false` marks Failed

**File:** `packages/open-bitcoin-node/src/sync/wallet_rescan.rs:124-135` and `packages/open-bitcoin-rpc/src/context/rescan.rs:168-180`
**Issue:** When `has_block` returns `Ok(false)`, both paths `mark_failed` and persist. When `has_block` returns `Err(...)`, the error propagates without marking the job failed. A backend/read failure can leave the job `Pending`/`Scanning`, so `resume_pending_jobs` / reopen keeps retrying the same torn state without a durable failure record.
**Fix:** On `has_block` error, mark and persist failure before returning, mirroring the missing-payload branch:

```rust
match self.store.has_block(position.block_hash) {
    Ok(true) => {}
    Ok(false) => {
        job.mark_failed(format!(
            "missing block payload at height {} hash {:?}",
            position.height, position.block_hash
        ));
        registry.save_rescan_job(&self.store, job.clone(), self.persist_mode)?;
        return Err(/* Unavailable BlockIndex */);
    }
    Err(error) => {
        job.mark_failed(format!(
            "block payload probe failed at height {}: {error}",
            position.height
        ));
        registry.save_rescan_job(&self.store, job.clone(), self.persist_mode)?;
        return Err(error.into());
    }
}
```

## Info

### IN-01: Tip mismatch / empty `chain_meta` mapped to `UnavailableNamespace`

**File:** `packages/open-bitcoin-node/src/storage/fjall_store/coins.rs:115-128`
**Issue:** Coins/meta inconsistency returns `StorageError::UnavailableNamespace { Chainstate }`, which reads like a missing keyspace rather than corrupt/disagreeing durable state. Callers and operators may mis-diagnose reopen failures.
**Fix:** Prefer a `Corruption` (or dedicated inconsistency) variant with a detail such as `"wallet_scan: coins best-block disagrees with chain_meta tip"`.

### IN-02: Duplicate `partial_chainstate_snapshot` helpers

**File:** `packages/open-bitcoin-node/src/sync/wallet_rescan.rs:169-197` and `packages/open-bitcoin-rpc/src/context/rescan.rs:220-248`
**Issue:** Identical height/UTXO/undo filtering logic lives in both crates. Drift risk if D-04 filtering rules change in only one place.
**Fix:** Share one pure helper (e.g. on `ChainstateSnapshot` in the chainstate core) and call it from both shells.

---

_Reviewed: 2026-09-22T05:07:00Z_
_Reviewer: Claude (gsd-code-reviewer)_
_Depth: standard_
