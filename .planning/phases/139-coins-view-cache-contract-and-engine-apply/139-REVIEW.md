---
phase: 139-coins-view-cache-contract-and-engine-apply
reviewed: 2026-08-31T04:35:00Z
depth: standard
files_reviewed: 13
files_reviewed_list:
  - packages/open-bitcoin-chainstate/src/coins.rs
  - packages/open-bitcoin-chainstate/src/coins/cache.rs
  - packages/open-bitcoin-chainstate/src/coins/memory.rs
  - packages/open-bitcoin-chainstate/src/engine.rs
  - packages/open-bitcoin-chainstate/src/engine/apply.rs
  - packages/open-bitcoin-chainstate/src/engine/stage.rs
  - packages/open-bitcoin-chainstate/src/error.rs
  - packages/open-bitcoin-chainstate/src/lib.rs
  - packages/open-bitcoin-chainstate/src/coins/tests/algebra.rs
  - packages/open-bitcoin-node/src/chainstate.rs
  - packages/open-bitcoin-node/src/chainstate/tests.rs
  - packages/open-bitcoin-node/src/network/mempool_lifecycle.rs
  - packages/open-bitcoin-chainstate/src/engine/tests/apply_non_coinbase_transaction_returns_fee_and_records_undo.rs
findings:
  critical: 0
  warning: 2
  info: 2
  total: 4
status: issues
---

# Phase 139: Code Review Report

**Reviewed:** 2026-08-31T04:35:00Z
**Depth:** standard
**Files Reviewed:** 13
**Status:** issues

## Summary

The DIRTY/FRESH overlay, four lookup facts, and prepare isolation match the Phase 139 contract. `CoinsCacheEntry` makes the invalid SanityCheck masks unrepresentable. Engine connect/disconnect/reorg mutate a child `CoinsOverlay` and only commit through `batch_write` / `absorb_batch_write`. Child reads use `CoinsView::get_coin` (peek), not `fetch_coin`. A failed `stage_*` / `prepare_*` never borrows the live cache mutably. `open-bitcoin-chainstate` has no Fjall, filesystem, Tokio, or clock imports, and production sources do not `unwrap`. `ManagedChainstate::persist` still writes the leftover snapshot blob with no durability claim.

Two correctness gaps remain: `batch_write` is not transactional on `FreshFlagMisapplied`, and `maybe_best_block: None` cannot clear coins best-block when the active chain becomes empty.

## Warnings

### WR-01: Failed `batch_write` leaves live overlay partially applied

**File:** `packages/open-bitcoin-chainstate/src/coins/cache.rs:412-423`
**Issue:** `CoinsCache::batch_write` applies child entries in place and returns `FreshFlagMisapplied` without restoring prior overlay occupancy. `Chainstate::apply_flushed` then skips metadata (`active_chain`, undo, confirmation counts). A failed `commit_staged_connect` / `commit_staged_reorg` can therefore leave extra DIRTY coins on the live cache while tip and undo stay on the previous block. HashMap iteration order makes which writes land before the error non-deterministic. The engine test that injects a FRESH overwrite (`commit_staged_connect_rejects_fresh_flag_misapplied_to_live_cache`) asserts only the error variant, not isolation. Manager commit uses infallible `absorb_batch_write`, so the two-phase mempool window is not affected; the Result commit path used by `connect_block` / `disconnect_tip` / `reorg` is.

**Fix:** Apply against a cloned overlay and publish only on success:

```rust
fn batch_write(
    &mut self,
    writes: CoinsBatch,
    maybe_best_block: Option<BlockHash>,
) -> Result<(), ChainstateError> {
    let mut next_overlay = self.overlay.clone();
    for (outpoint, child_entry) in writes.entries {
        apply_child_write(&mut next_overlay, outpoint, child_entry)?;
    }
    if let Some(best_block) = maybe_best_block {
        next_overlay.maybe_best_block = Some(best_block);
    }
    self.overlay = next_overlay;
    Ok(())
}
```

Also assert `have_coin_in_cache`, `utxos()`, `tip()`, and `coins_best_block()` are unchanged after the injected FreshFlagMisapplied commit.

### WR-02: Empty-chain disconnect cannot clear `coins_best_block`

**File:** `packages/open-bitcoin-chainstate/src/engine.rs:230-236`
**Issue:** `batch_write` / `absorb_batch_write` treat `maybe_best_block: None` as “leave the current tip hash”. Knots `BatchWrite` always assigns `hashBlock`, including null. After disconnecting the last block (or a reorg that empties `next_active_chain`), the engine passes `None`, so `coins_best_block()` still reports the disconnected tip. Clearing only `overlay.maybe_best_block` would not be enough: `CoinsCache::best_block` falls through to `MemoryCoinsView`, which still holds the `from_snapshot` / `from_parent` tip because Phase 139 never Flushes that parent on apply.

**Fix:** Give the cache an explicit clear that sets both overlay and parent best-block to `None`, and call it when the post-disconnect chain is empty:

```rust
impl<V: CoinsView> CoinsCache<V> {
    pub fn clear_best_block(&mut self) {
        self.overlay.maybe_best_block = None;
    }
}

impl MemoryCoinsView {
    pub fn clear_best_block(&mut self) {
        self.maybe_best_block = None;
    }
}

// In disconnect_tip / commit_staged_reorg when next_active_chain is empty:
self.coins.clear_best_block();
self.coins.parent_mut().clear_best_block(); // or a cache method that forwards
```

If adding `parent_mut` is too wide, specialize the clear on `CoinsCache<MemoryCoinsView>` so emptying the chain updates both layers.

## Info

### IN-01: `spend_coin_into` writes a throwaway `UnspentClean` before the tombstone

**File:** `packages/open-bitcoin-chainstate/src/coins/cache.rs:188-192`
**Issue:** On a parent peek hit the helper inserts `unspent_clean` and immediately overwrites it with `spent_dirty`. End state matches Knots FetchCoin-then-SpendCoin, but the first insert is dead work and would be misleading if a later edit split the two writes.
**Fix:** Insert `CoinsCacheEntry::spent_dirty()` only.

### IN-02: Mempool `have_coin` errors classify as missing parents

**File:** `packages/open-bitcoin-node/src/network/mempool_lifecycle.rs:348-352`
**Issue:** `.unwrap_or_default()` turns any `have_coin` `Err` into `false`, so the input is treated as a missing orphan parent. Today `MemoryCoinsView` never errors, so this is not a Phase 139 behavior bug. When Phase 141 adds a fallible coins view, this fail-open path will hide read failures.
**Fix:** Keep `unwrap_or_default` only while the view is infallible; switch to fail-closed (`?` or a typed internal error) when `have_coin` can fail.

---

_Reviewed: 2026-08-31T04:35:00Z_
_Reviewer: Claude (gsd-code-reviewer)_
_Depth: standard_
