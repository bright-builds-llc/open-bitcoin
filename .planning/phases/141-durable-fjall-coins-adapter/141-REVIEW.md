---
phase: 141-durable-fjall-coins-adapter
reviewed: 2026-09-05T01:15:00Z
depth: standard
files_reviewed: 21
files_reviewed_list:
  - packages/open-bitcoin-chainstate/src/coins.rs
  - packages/open-bitcoin-chainstate/src/coins/cache.rs
  - packages/open-bitcoin-chainstate/src/coins/memory.rs
  - packages/open-bitcoin-chainstate/src/coins/tests.rs
  - packages/open-bitcoin-chainstate/src/engine.rs
  - packages/open-bitcoin-chainstate/src/error.rs
  - packages/open-bitcoin-node/src/chainstate.rs
  - packages/open-bitcoin-node/src/chainstate/tests.rs
  - packages/open-bitcoin-node/src/storage.rs
  - packages/open-bitcoin-node/src/storage/coins_codec.rs
  - packages/open-bitcoin-node/src/storage/coins_codec/tests.rs
  - packages/open-bitcoin-node/src/storage/coins_view.rs
  - packages/open-bitcoin-node/src/storage/coins_view/tests.rs
  - packages/open-bitcoin-node/src/storage/fjall_store.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/coins.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/coins_access.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/tests/coins_migration.rs
  - packages/open-bitcoin-node/src/storage/snapshot_codec.rs
  - packages/open-bitcoin-node/src/storage/snapshot_codec/chain_meta.rs
  - packages/open-bitcoin-node/src/sync.rs
  - packages/open-bitcoin-node/src/sync/runtime_state.rs
findings:
  critical: 1
  warning: 4
  info: 3
  total: 8
status: issues_found
---

# Phase 141: Code Review Report

**Reviewed:** 2026-09-05T01:15:00Z
**Depth:** standard
**Files Reviewed:** 21
**Status:** issues_found

## Summary

Reviewed the Phase 141 coins adapter: compact `C`/`B`/`H` codec, `FjallCoinsView` two-phase BatchWrite, fallible `CoinsView` heads, `MemoryChainstateStore` view surface, schema 1→2 migrate, and `hydrate_chainstate_for_open`.

Fail-closed disk reads on the Fjall parent are sound: `get_coin` / `have_coin` / `best_block` / `head_blocks` map I/O and decode failures to `CoinsStorage`, never `Ok(None)` or `MissingCoin`. Two-element `H` plus missing `B` is `InterruptedWrite` on the view. Schema 2 leftover-plus-empty-coins refuses remigrate. Compact codec round-trips height and MTP and rejects trailing bytes.

The production reopen path has a leftover dual-truth hole: `seed_coins_from_leftover_for_reopen` merges leftover UTXOs into the coins keyspace and never deletes spent `C` keys, so `hydrate_chainstate_for_open` can resurrect spent coins. `DurableSyncRuntime::open` also installs the hydrate snapshot through leftover `save_snapshot` instead of `from_snapshot`. Interrupted-`H` classification is shadowed when leftover is present and coins look empty. Schema 1 migrate is not crash-resumable after the first `C`/`B` write.

## Critical Issues

### CR-01: Seed merge resurrects spent coins on reopen

**File:** `packages/open-bitcoin-node/src/storage/fjall_store/coins.rs:149-201`
**Issue:** `seed_coins_from_leftover_for_reopen` is the transitional write that makes schema-2 reopen possible. It copies leftover UTXOs through `write_migrated_coins_with_tip`, which only inserts `unspent_dirty` entries and then `FjallCoinsView::batch_write`. Knots BatchWrite applies dirty keys only; it does not replace the UTXO set. Coins that were seeded earlier and later spent (or dropped from the leftover snapshot) stay on disk. `hydrate_chainstate_for_open` then `scan_coin_records()` over every `C` prefix key, so `DurableSyncRuntime::open` hydrates a superset of the leftover snapshot. After one persist, a spend, and another persist, reopen can treat spent outpoints as live.
**Fix:** Treat leftover as a full UTXO set. Delete `C` keys that are not in leftover (or wipe the coins prefix, then write). Keep `H`/`B` BatchWrite for the replacement set.

```rust
fn write_migrated_coins_with_tip(
    &self,
    utxos: &HashMap<OutPoint, Coin>,
    tip: BlockHash,
) -> Result<(), StorageError> {
    let mut view = FjallCoinsView::from_store(self);
    let mut entries = HashMap::new();
    for guard in self.coins.prefix([DB_COIN]) {
        let (key_bytes, _) = guard
            .into_inner()
            .map_err(|error| backend_failure(StorageNamespace::Coins, error))?;
        let outpoint = decode_coin_key(key_bytes.as_ref())?;
        if !utxos.contains_key(&outpoint) {
            entries.insert(outpoint, CoinsCacheEntry::spent_dirty());
        }
    }
    for (outpoint, coin) in utxos {
        entries.insert(
            outpoint.clone(),
            CoinsCacheEntry::unspent_dirty(coin.clone()),
        );
    }
    view.batch_write(CoinsBatch { entries }, Some(tip))
        .map_err(map_heads_error)
}
```

## Warnings

### WR-01: Runtime open hydrates leftover snapshot slot, not the view-backed maps

**File:** `packages/open-bitcoin-node/src/sync.rs:130-133`
**Issue:** `hydrate_chainstate_for_open` returns coins-truth, but open does `MemoryChainstateStore::default()` then `save_snapshot(snapshot)`. `save_snapshot` only writes `maybe_snapshot` and does not populate `coins` or `undo_by_block` (that split is intentional for later leftover persists). `ManagedChainstate::from_store` still builds the engine from `load_snapshot()`, so apply works, but the view-backed store surface is empty after a successful coins hydrate. That is leftover-vs-view dual-truth on the in-memory store Plan 03 introduced `from_snapshot` to prevent.
**Fix:** Hydrate once through `from_snapshot`:

```rust
let memory_store = match store.hydrate_chainstate_for_open()? {
    Some(snapshot) => MemoryChainstateStore::from_snapshot(snapshot),
    None => MemoryChainstateStore::default(),
};
```

### WR-02: Schema-2 leftover-plus-empty check shadows interrupted `H`

**File:** `packages/open-bitcoin-node/src/storage/fjall_store/coins.rs:110-116`
**Issue:** `ensure_schema_two` returns leftover-empty `RestoreFromBackup` when leftover exists and `coins_keyspace_is_empty` is true. Emptiness is only `B` absent and no `C` prefix; it ignores `H`. `FjallNodeStore::open` runs this before `hydrate_chainstate_for_open`, which is the path that classifies two-element `H` plus missing `B` as `InterruptedWrite`. A crash after `persist_progress` writes leftover and `seed` writes `H` but before the first `C`/`B` therefore opens as leftover-empty corruption, not interrupted flush. Phase 142 replay cannot see the marker.
**Fix:** Classify markers (or at least presence of `H`) before the leftover-empty fail-closed. If `head_blocks` is a two-element interrupted flush, return `InterruptedWrite` even when leftover is present and `C`/`B` are missing.

```rust
fn ensure_schema_two(&self) -> Result<(), StorageError> {
    match FjallCoinsView::from_store(self).head_blocks() {
        Ok(_) => {}
        Err(error) => return Err(map_heads_error(error)),
    }
    let leftover_present = self.leftover_snapshot_present()?;
    let coins_empty = self.coins_keyspace_is_empty()?;
    if leftover_present && coins_empty {
        return Err(leftover_empty_coins_corruption());
    }
    Ok(())
}
```

### WR-03: Schema 1 migrate is not crash-resumable after coins exist

**File:** `packages/open-bitcoin-node/src/storage/fjall_store/coins.rs:98-107`
**Issue:** `ensure_schema_one` migrates leftover into `C`/`B`, undo, and `chain_meta`, then writes schema 2. If the process dies after `write_migrated_coins_*` and before `write_schema_version`, reopen still sees schema 1 plus leftover plus non-empty coins and returns `mixed_schema_one_corruption` (`RestoreFromBackup`). The leftover blob is still intact, but the store will not finish the one-way migrate. Tests lock this as corruption; it is still a datadir-bricking crash window on the upgrade path.
**Fix:** If schema 1 leftover is present and coins already have a consistent `B` (empty `H`), treat that as an interrupted migrate: finish undo/`chain_meta` if needed and bump schema 2. Keep leftover-plus-conflicting-coins as corruption.

### WR-04: Leftover-then-seed persist order can fail closed or hydrate stale coins

**File:** `packages/open-bitcoin-node/src/sync/runtime_state.rs:88-97`
**Issue:** `persist_progress` writes leftover first, then seeds coins. Schema 2 treats leftover as non-authoritative. A crash after leftover and before seed on a store whose coins are still empty fails closed as leftover-plus-empty (valid leftover cannot remigrate). A crash after leftover and before seed when prior coins exist reopens from the older coins set and ignores the newer leftover. Dual-write makes coins truth and leftover newer at the same time.
**Fix:** Seed coins (replace, not merge) before leftover, or write both in one Sync batch and treat leftover-plus-empty as remigrate-only when `H` is absent and leftover decodes. Phase 142 persist cutover should not leave this window.

## Info

### IN-01: Interrupted-write remap depends on Display text

**File:** `packages/open-bitcoin-node/src/storage/fjall_store/coins.rs:340-355`
**Issue:** `map_heads_error` flattens `ChainstateError::CoinsStorage` and recovers `InterruptedWrite` with `detail.contains("interrupted write")`. That matches today's `StorageError` Display (`"interrupted write in coins; ..."`), but any Display change silently turns interrupted flush into `Corruption`/`Repair`.
**Fix:** Carry a typed flag through `CoinsStorage`, or match on `StorageError::InterruptedWrite` before flattening.

### IN-02: Missing `chain_meta` falls back to leftover snapshot metadata

**File:** `packages/open-bitcoin-node/src/storage/fjall_store/coins.rs:269-277`
**Issue:** UTXOs come from the coins scan, but `load_hydrate_chain_meta` reads leftover `active_chain` / confirmation counts when `chain_meta` is absent. That is not UTXO dual-read, but leftover can still supply tip metadata that disagrees with coins `B`.
**Fix:** Fail closed if coins are non-empty and `chain_meta` is missing, or require `chain_meta` on every successful seed/migrate.

### IN-03: Memory parent applies non-dirty batch entries; Fjall parent skips them

**File:** `packages/open-bitcoin-chainstate/src/coins/memory.rs:56-75`
**Issue:** `FjallCoinsView::batch_write` skips `!entry.is_dirty()`. `MemoryCoinsView::batch_write` applies every entry, so `spent_fresh` becomes a delete in memory and a no-op on disk. Production `CoinsCache::flush` only sends dirty entries, so the live path matches.
**Fix:** Skip non-dirty entries in `MemoryCoinsView::batch_write` so both parents honor the same BatchWrite contract.

---

_Reviewed: 2026-09-05T01:15:00Z_
_Reviewer: Claude (gsd-code-reviewer)_
_Depth: standard_
