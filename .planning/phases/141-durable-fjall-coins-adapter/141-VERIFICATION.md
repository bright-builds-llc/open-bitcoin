---
phase: 141-durable-fjall-coins-adapter
verified: 2026-09-05T00:40:10Z
status: passed
score: 12/12 must-haves verified
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: 141-2026-09-04T17-37-21
generated_at: 2026-09-05T00:40:10Z
lifecycle_validated: true
overrides_applied: 0
---

# Phase 141: Durable Fjall Coins Adapter Verification Report

**Phase Goal:** Spendable UTXOs live as per-outpoint Fjall records with best-block and interrupted-flush markers; snapshot blobs are no longer live coin truth.
**Verified:** 2026-09-05T00:40:10Z
**Status:** passed
**Re-verification:** No — initial verification

## Goal Achievement

The codebase delivers the phase goal. Live UTXO truth on disk is the dedicated `coins` keyspace (`C` + best-block `B` + interrupted-flush `H`). Schema 1 leftover `"snapshot"` blobs migrate once into those records. Schema 2 reopen hydrates from coins, not leftover `utxos`. Leftover persist *writes* remain in place (D-19). `ReplayBlocks` is not implemented.

### Observable Truths

| # | Truth | Status | Evidence |
| --- | --- | --- | --- |
| 1 | The node stores and loads coins as per-outpoint durable records with coins best-block and two-element `head_blocks` markers. | ✓ VERIFIED | `FjallCoinsView` writes `C` keys, `H=[new,old]`, then `B=new`. `classify_markers` treats empty `H` + present `B` as consistent. Tests `batch_write_writes_h_then_coins_then_b` and `two_element_h_without_b_is_interrupted_write` exist. |
| 2 | A leftover snapshot blob is non-authoritative after the explicit one-way migration; reopen does not treat it as UTXO truth. | ✓ VERIFIED | `hydrate_chainstate_for_open` scans `C` keys for utxos. `load_hydrate_chain_meta` may read leftover *metadata only*. Test `schema_two_leftover_utxos_do_not_hydrate_after_coins_exist` overwrites leftover with `[0xdd]` and asserts hydrate still has `[0xaa]`. Schema 2 leftover + empty coins is `Corruption` / `RestoreFromBackup`. |
| 3 | A coins disk-read error fails closed as a typed storage or recovery error, never as spent or missing. | ✓ VERIFIED | `map_optional_read` / `map_storage` turn Fjall `Err` and decode failure into `ChainstateError::CoinsStorage`. `get_coin` never `ok().flatten()`. Tests `get_coin_decode_failure_is_coins_storage_not_none`, `have_coin_backend_error_is_not_false`, and `coins_cache_propagates_parent_coins_storage_on_get_have_best_and_heads`. |
| 4 | Schema mismatch fails closed, and LevelDB or rust-bitcoin are not introduced. | ✓ VERIFIED | `ensure_schema_and_migrate_coins` maps unknown versions to `StorageError::schema_mismatch`. Test `schema_three_is_schema_mismatch`. No `rusty-leveldb`, `rust-bitcoin`, `rocksdb`, or `leveldb` in `packages/Cargo.toml` or node/chainstate crate manifests. Fjall stays `3.1.4`. |
| 5 | `CoinsView::best_block` and `head_blocks` return `Result` so a disk parent can surface `CoinsStorage`. | ✓ VERIFIED | Trait in `coins.rs` is `Result<Option<BlockHash>, ChainstateError>` / `Result<Vec<BlockHash>, ChainstateError>`. Memory, cache, and `FjallCoinsView` implement those signatures. |
| 6 | Coin keys are `C` + 32-byte txid + 4-byte LE vout; `B`/`H` are single-byte keys; compact values round-trip height and MTP. | ✓ VERIFIED | `encode_coin_key` writes 37 bytes with `vout.to_le_bytes()`. Compact layout is compact-size code, i64 LE sats, compact-size script, i64 LE MTP, `Reader::finish()`. Tests in `coins_codec/tests.rs`. |
| 7 | `StorageNamespace::Coins` is `"coins"`; `FjallNodeStore` opens that keyspace; `SchemaVersion::CURRENT` is 2. | ✓ VERIFIED | `as_str() == "coins"`. `open` and `open_without_ensure_schema_for_test` call `open_keyspace(..., StorageNamespace::Coins)`. `CURRENT = Self(2)`. |
| 8 | `FjallCoinsView` implements Knots two-phase BatchWrite with first-party 64 MiB encoded-byte accounting. | ✓ VERIFIED | Erase `B`, write `H`, dirty `C` inserts/deletes via `estimated_encoded_bytes`, Buffered partials, erase `H` + write `B` as Flush. No `WriteBatch::len()` / `batch.len()`. `DEFAULT_COINS_DB_BATCH_BYTES = 64 << 20`. |
| 9 | Leftover persist writes were not cut over. `ReplayBlocks` is not implemented. `save_block` / `load_block` stay on `block:` hex keys. | ✓ VERIFIED | `ManagedChainstate::persist` is still `save_snapshot(self.chainstate.snapshot())`. `persist_progress` still calls `save_chainstate_snapshot` (plus transitional `seed_coins_from_leftover_for_reopen`). Repo `ReplayBlocks` appears only as a forbidden-string assertion in flush tests. `save_block` / `load_block` use `block_key` and do not mention `StorageNamespace::Coins`. |
| 10 | `hydrate_chainstate_for_open` consults `FjallCoinsView::head_blocks` before any `C`-prefix scan; two-element `H` plus missing `B` is `InterruptedWrite`. | ✓ VERIFIED | First statements in `hydrate_chainstate_for_open` construct the view and match `head_blocks()`. `map_heads_error` maps `"interrupted write"` to `StorageError::InterruptedWrite { namespace: Coins, action: Reindex }`. Test `hydrate_two_element_h_without_b_fails_closed`. |
| 11 | `ChainstateStore` exposes the view-backed surface; `MemoryChainstateStore` serves coins/undo from its own maps; standalone undo codec omits `"utxos"`. | ✓ VERIFIED | Trait has `get_coin` / `have_coin` / `best_block` / `head_blocks` / `batch_write` / `load_undo` / `save_undo` plus leftover snapshot load/save. `from_snapshot` hydrates `MemoryCoinsView` and `undo_by_block` once; later `save_snapshot` does not clear them. `encode_block_undo` uses `BlockUndoDto` only. |
| 12 | `open-bitcoin-chainstate` remains I/O-free. | ✓ VERIFIED | Crate `Cargo.toml` depends only on `open-bitcoin-consensus` and `open-bitcoin-primitives`. Production coins/error sources have no `fjall`, `std::fs`, Tokio, `Instant`, or `SystemTime`. `CoinsCache` has no Fjall types. |

**Score:** 12/12 truths verified

### Required Artifacts

| Artifact | Expected | Status | Details |
| -------- | ----------- | ------ | ------- |
| `packages/open-bitcoin-node/src/storage/coins_codec.rs` | C/B/H key encoders and compact coin / heads / best-block codec | ✓ VERIFIED | Exists, substantive (`DB_COIN`, `encode_coin_key`, `decode_coin_value`, `Reader::finish`). Wired from `FjallCoinsView` and migrate/hydrate. |
| `packages/open-bitcoin-chainstate/src/error.rs` | `CoinsStorage` disk-read error distinct from `MissingCoin` | ✓ VERIFIED | Variant + Display `coins storage error: {detail}`. Used by cache parent propagation and `FjallCoinsView::map_storage`. |
| `packages/open-bitcoin-node/src/storage.rs` | `StorageNamespace::Coins` and `CURRENT = 2` | ✓ VERIFIED | `Coins => "coins"`, `blob_schema_is_readable` accepts 1\|2, `CURRENT = Self(2)`. |
| `packages/open-bitcoin-node/src/storage/coins_view.rs` | `FjallCoinsView` CoinsView impl and BatchWrite protocol | ✓ VERIFIED | `impl CoinsView for FjallCoinsView`. Wired from store `coins_view()`, hydrate, and schema-1 migrate. |
| `packages/open-bitcoin-node/src/storage/coins_view/tests.rs` | BatchWrite, 64 MiB split, fail-closed read, interrupted H tests | ✓ VERIFIED | Locked test names present, including `batch_write_writes_h_then_coins_then_b`. |
| `packages/open-bitcoin-node/src/chainstate.rs` | Grown `ChainstateStore` + `MemoryChainstateStore` | ✓ VERIFIED | `fn load_undo` / `fn save_undo` / `fn get_coin` / `fn batch_write`. `persist` still leftover. |
| `packages/open-bitcoin-node/src/storage/snapshot_codec.rs` | Standalone `encode_block_undo` / `decode_block_undo` | ✓ VERIFIED | `BlockUndoDto` + `encode_versioned` / `decode_versioned`. Does not call `encode_chainstate_snapshot`. |
| `packages/open-bitcoin-node/src/storage/fjall_store/coins.rs` | `save_undo` / `load_undo`, schema 1 migrate, `hydrate_chainstate_for_open` | ✓ VERIFIED | All three present. `ensure_schema` in `fjall_store.rs` delegates to `ensure_schema_and_migrate_coins`. |
| `packages/open-bitcoin-node/src/sync.rs` | `DurableSyncRuntime::open` uses coins hydrate | ✓ VERIFIED | `open_with_runtime_activation` calls `hydrate_chainstate_for_open`. No `load_chainstate_snapshot_with_confirmation_migration`. |

gsd-tools `verify artifacts` on all four plans: 10/10 passed.

### Key Link Verification

| From | To | Via | Status | Details |
| ---- | --- | --- | ------ | ------- |
| `coins.rs` | `error.rs` | fallible `best_block` / `head_blocks` return `ChainstateError` | ✓ WIRED | `fn head_blocks` present; signatures return `Result<..., ChainstateError>`. |
| `coins_codec.rs` | codec primitives | `Reader::new` then `finish` after compact fields | ✓ WIRED | `decode_coin_value` / `decode_best_block_value` / `decode_head_blocks_value`. |
| `coins_view.rs` | `coins.rs` | `impl CoinsView for FjallCoinsView` | ✓ WIRED | Full trait impl. |
| `coins_view.rs` | `coins_codec.rs` | `estimated_encoded_bytes` | ✓ WIRED | Used for marker + dirty-item accounting. |
| `chainstate.rs` | `MemoryCoinsView` | `fn batch_write` delegates | ✓ WIRED | Memory store coins methods forward to `self.coins`. |
| `snapshot_codec.rs` | `BlockUndoDto` | `fn encode_block_undo` | ✓ WIRED | Encode/decode extracted from leftover snapshot DTO. |
| `fjall_store.rs` | `fjall_store/coins.rs` | `migrate_schema_1_coins` | ✓ WIRED | `ensure_schema` → `ensure_schema_and_migrate_coins` → schema-1 branch. |
| `sync.rs` | `fjall_store/coins.rs` | `hydrate_chainstate_for_open` | ✓ WIRED | Open path only. |
| `fjall_store/coins.rs` | `coins_view.rs` | `head_blocks` before C scan | ✓ WIRED | First call in `hydrate_chainstate_for_open`. |

gsd-tools `verify key-links` on all four plans: 9/9 verified.

### Data-Flow Trace (Level 4)

| Artifact | Data Variable | Source | Produces Real Data | Status |
| -------- | ------------- | ------ | ------------------ | ------ |
| `FjallCoinsView::get_coin` | `Coin` | Fjall `coins.get(encode_coin_key)` → `decode_coin_value` | Yes — compact bytes from the coins keyspace | ✓ FLOWING |
| `hydrate_chainstate_for_open` | `snapshot.utxos` | `scan_coin_records()` over `C` prefix | Yes — leftover `utxos` are not copied after coins exist | ✓ FLOWING |
| `DurableSyncRuntime::open` | `MemoryChainstateStore` leftover DTO | `hydrate_chainstate_for_open()` then `save_snapshot` | Yes — DTO is a hydrate helper filled from coins, not leftover disk `utxos` | ✓ FLOWING |
| `migrate_schema_1_coins` | `C` / `undo:` / `chain_meta` | leftover snapshot loaded once on schema 1 + empty coins | Yes — leftover blob remains; schema 2 does not remigrate | ✓ FLOWING |

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
| -------- | ------- | ------ | ------ |
| Schema current is 2 | `rg 'pub const CURRENT: Self = Self\(2\)' packages/open-bitcoin-node/src/storage.rs` | Match at `SchemaVersion` | ✓ PASS |
| Leftover persist writes remain | `rg 'save_chainstate_snapshot\|save_snapshot\(self.chainstate.snapshot\(\)\)' packages/open-bitcoin-node/src/sync/runtime_state.rs packages/open-bitcoin-node/src/chainstate.rs` | Both leftover write sites present | ✓ PASS |
| Open hydrates coins, not leftover snapshot | `rg 'hydrate_chainstate_for_open\|load_chainstate_snapshot_with_confirmation_migration' packages/open-bitcoin-node/src/sync.rs` | Only `hydrate_chainstate_for_open` | ✓ PASS |
| No ReplayBlocks / no rust-bitcoin | `rg ReplayBlocks\|rusty-leveldb\|rust-bitcoin\|rocksdb packages/` | Only a forbidden-string test mention of `ReplayBlocks` | ✓ PASS |
| Fail-closed decode mapping | `rg 'ok\(\)\.flatten\(\)' packages/open-bitcoin-node/src/storage/coins_view.rs` | No matches | ✓ PASS |

Targeted cargo suites were not re-run in this pass (verification stays file/wiring-first; SUMMARY-claimed GREEN runs are not trusted as evidence). Locked test names exist in-tree for codec, view, store, and migrate suites.

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
| ----------- | ---------- | ----------- | ------ | -------- |
| **COIN-01** | 141-01, 141-02, 141-03, 141-04 | Node persists spendable UTXOs as per-outpoint durable coin records with coins best-block and interrupted-flush markers, not as live snapshot-blob truth. | ✓ SATISFIED | Dedicated `coins` keyspace, compact `C`/`B`/`H`, BatchWrite, schema 1→2 migrate, leftover non-authority hydrate. REQUIREMENTS.md maps COIN-01 → Phase 141. |
| **CSOBS-03** | 141-01, 141-02, 141-04 | A coins disk-read error fails closed as a typed storage or recovery error, not as spent or missing. | ✓ SATISFIED | `CoinsStorage`, `map_optional_read`, planted-byte decode test, interrupted `H` → `InterruptedWrite`, cache parent propagation. REQUIREMENTS.md maps CSOBS-03 → Phase 141. |

No orphaned Phase 141 requirements. PLAN 03 lists only COIN-01; CSOBS-03 is claimed by plans 01, 02, and 04.

### Locked Decisions D-01..D-22

| ID | Decision | Status |
| --- | --- | --- |
| D-01 | Knots-shaped `C` + txid + LE vout; `B`; `H`; compact values | ✓ Honored |
| D-02 | Erase is delete; successful miss is `Ok(None)` | ✓ Honored |
| D-03 | No LevelDB / rusty-leveldb / RocksDB / rust-bitcoin; Fjall 3.1.4 | ✓ Honored |
| D-04 | `StorageNamespace::Coins` (`"coins"`) | ✓ Honored |
| D-05 | Undo / chain_meta stay in `chainstate` keyspace; `chain_meta` has no `utxos` | ✓ Honored |
| D-06 | `FjallCoinsView` in node shell; cache ignorant of Fjall; core I/O-free | ✓ Honored |
| D-07 | 64 MiB first-party `estimated_encoded_bytes`; no `WriteBatch::len()` as size | ✓ Honored |
| D-08 | Two-phase BatchWrite: erase `B` / write `H`, dirty coins, erase `H` / write `B` | ✓ Honored |
| D-09 | Two-element `H` + missing `B` fails closed; no `ReplayBlocks` | ✓ Honored |
| D-10 | Other `H` counts / decode failure are `Corruption`; no auto-repair | ✓ Honored |
| D-11 | Compact codec in `coins_codec.rs` includes height + MTP | ✓ Honored |
| D-12 | Leftover pretty-JSON snapshot is not live coin truth | ✓ Honored |
| D-13 | Undo is its own `undo:` + hex record | ✓ Honored |
| D-14 | `save_block` / `load_block` unchanged on `block:` hex keys | ✓ Honored |
| D-15 | After migrate, leftover `"snapshot"` is not UTXO truth | ✓ Honored |
| D-16 | `SchemaVersion::CURRENT = 2`; other store versions `schema_mismatch` | ✓ Honored |
| D-17 | Blob schemas 1 and 2 remain readable | ✓ Honored |
| D-18 | One-way migrate; schema 2 leftover + empty coins fails closed | ✓ Honored |
| D-19 | Leftover persist *writes* not cut over | ✓ Honored — leftover write sites remain; transitional coins *seed* is extra, not a cutover |
| D-20 | Disk-read / decode failure is typed, never spent/missing | ✓ Honored |
| D-21 | Fjall errors surface through `CoinsView` `Result` | ✓ Honored |
| D-22 | `MemoryChainstateStore` implements the view-backed surface; crash seam is `cfg(test)` | ✓ Honored |

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
| ---- | ---- | ------- | -------- | ------ |
| `packages/open-bitcoin-node/src/sync/runtime_state.rs` | 95–97 | Transitional `seed_coins_from_leftover_for_reopen` after leftover persist | ℹ️ Info | Dual-write so schema-2 reopen can hydrate. Leftover write remains. Phase 142 owns write-site cutover. |
| `packages/open-bitcoin-node/src/storage/fjall_store/coins.rs` | 147–185 | `seed_coins_from_leftover_for_reopen` copies leftover utxos into coins on persist | ℹ️ Info | Write-path helper, not reopen authority. Hydrate still ignores leftover `utxos` after coins exist. |
| `packages/open-bitcoin-node/src/storage/fjall_store/coins_access.rs` | 9–16 | `#[allow(dead_code)]` on `database()` / `coins_keyspace()` | ℹ️ Info | Both accessors are used by `FjallCoinsView::from_store`. Allow is stale, not a stub. |

No blocker stubs. No `TODO` / `FIXME` / `not implemented` in the coins adapter production files. No `ReplayBlocks` implementation.

### Confirmation-Bias Notes

- Persist still writes leftover snapshots. That is locked D-19, not incomplete COIN-01. Reopen authority is coins.
- `persist_progress_source_still_writes_leftover_snapshot` is an `include_str` source lock. Behavioral leftover-non-authority is covered by `schema_two_leftover_utxos_do_not_hydrate_after_coins_exist`.
- `have_coin_backend_error_is_not_false` unit-tests the mapper rather than injecting a live Fjall I/O error. Planted corrupt `C` bytes still prove `get_coin` fail-closed.

### Human Verification Required

None. This phase is a storage adapter with unit-locked migrate / hydrate / fail-closed / leftover-write contracts. No visual, operator-flow, or external-service checks are required to confirm the goal.

### Gaps Summary

No actionable gaps. Phase 142 still owns persist write-site cutover off leftover snapshots, manager flush wiring, and `ReplayBlocks` execution. Those are explicit deferrals, not Phase 141 failures.

---

_Verified: 2026-09-05T00:40:10Z_
_Verifier: Claude (gsd-verifier)_
