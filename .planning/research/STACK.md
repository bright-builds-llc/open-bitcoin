# Stack Research

**Domain:** Bitcoin chainstate durability (coins DB, cache flush, chainstate manager)
**Researched:** 2026-08-29
**Confidence:** HIGH

## Recommendation

v2.3 should add **no new production crates**. Disk-backed coins, cache-flush policy, and fuller chainstate-manager behavior fit the existing Rust workspace, first-party UTXO types, and Fjall `3.1.4` store already used by `FjallNodeStore`.

The milestone is an internal persistence-shape change:

- Stop treating a pretty-printed JSON `ChainstateSnapshot` blob as coin truth.
- Add a first-party coins-view/cache in `open-bitcoin-chainstate` that mirrors Knots `CCoinsView` / `CCoinsViewCache` semantics (`DIRTY` / `FRESH`, `Flush` vs `Sync`).
- Persist per-outpoint coin records plus best-block / head-blocks metadata through the existing Fjall database.
- Move `ManagedChainstate` from "clone the whole UTXO map and rewrite `snapshot`" to a manager that owns cache lifecycle, flush points, and restart-safe reopen.
- Make block serving and operator evidence ask Fjall whether the `block:` payload key exists; do not infer availability from the header/index snapshot.

Keep Fjall at the repo pin. Do not add LevelDB, RocksDB, or rust-bitcoin. Those would duplicate a store and domain model the project already owns.

## Recommended Stack

### Core Technologies

| Technology | Version | Purpose | Why Recommended |
|------------|---------|---------|-----------------|
| Rust | `1.94.1`, edition 2024 | Pure coins-view/cache, flush-policy state machines, and typed recovery outcomes | Already pinned by `rust-toolchain.toml` and `packages/Cargo.toml`. Enums can make `Flush` vs `Sync`, cache-size state, and incomplete-flush recovery explicit. |
| Open Bitcoin workspace crates | `0.1.0` | Own UTXO types, coin codec, node store adapter, and manager orchestration | Preserves the production-path ownership rule and the functional-core / imperative-shell split. `open-bitcoin-chainstate` has no I/O deps today; keep it that way. |
| Bitcoin Knots | `29.3.knots20260210` | External behavior contract for coins cache, coins DB writes, and flush policy | Vendored `coins.h` / `coins.cpp`, `txdb.h` / `txdb.cpp`, `validation.cpp` (`FlushStateToDisk`), and `node/blockstorage.cpp` (block/undo flush before coins) are the parity roots. |
| Fjall | `3.1.4`, existing, `default-features = false` | Durable per-coin records, atomic batches, prefix/range cursors, and persist modes | Already backs headers, block payloads, chainstate snapshots, mempool, and runtime markers. `Database::batch()`, `Keyspace::{get,insert,remove,contains_key,prefix,range}`, and `persist(Buffer \| SyncAll)` cover the Knots `CCoinsViewDB` surface without a second database. |
| Tokio | `1.52.1`, existing in `open-bitcoin-rpc` only | Optional periodic flush wakeup in the daemon shell | Knots schedules `PERIODIC` writes on an injected clock, not inside the coins cache. Do not add Tokio to `open-bitcoin-chainstate` or `open-bitcoin-node`. |
| serde / serde_json | `1.0.228` / `1.0.149`, existing | Versioned metadata and operator/report shapes | Keep for tip metadata, recovery markers, and evidence. Do **not** encode the live UTXO set as pretty JSON. |

### First-Party Modules to Extend

| Crate / module | New responsibility | Integration point |
| --- | --- | --- |
| `open-bitcoin-chainstate` coins view | Pure `CoinsView` / `CoinsViewCache` with `GetCoin`, `HaveCoin`, `AddCoin`, `SpendCoin`, `BatchWrite`, `Flush`, `Sync`, `Uncache`, cache-size accounting, and `DIRTY`/`FRESH` flags | Replace `Chainstate.utxos: HashMap<OutPoint, Coin>` as the live spend view. The engine should apply connect/disconnect against the cache, not clone the whole map on every block. |
| `open-bitcoin-chainstate` flush policy | Pure `FlushStateMode { None, IfNeeded, Periodic, Always }` and `CoinsCacheSizeState { Ok, Large, Critical }` | Mirror Knots `validation.h` / `validation.cpp`. Inputs are injected: now, cache bytes, cache limit, mempool leftover bytes, and whether a write is due. Do not read clocks or disk here. Omit prune-triggered flush (`fFlushForPrune`); that product mode is out of scope. |
| `open-bitcoin-codec` coin records | First-party compact outpoint keys and coin values, including Open Bitcoin's extra `created_median_time_past` field | Reuse existing compact-size / script / output encoders. Do not invent a rust-bitcoin `TxOutCompression` port and do not persist coins as `CoinDto` JSON. |
| `open-bitcoin-node` `FjallNodeStore` | Disk-backed coins adapter: per-outpoint get/put/erase, `best_block`, two-element `head_blocks`, batched dirty writes, payload-presence checks | Add a `coins` keyspace (or an equivalent prefix inside the existing `chainstate` keyspace). Reuse `PersistMode::{Buffered,Flush,Sync}` already mapped to Fjall `None` / `Buffer` / `SyncAll`. |
| `open-bitcoin-node` `ManagedChainstate` | Fuller manager: init coins DB, verify store health, then init cache; flush on policy, not after every connect; fail closed on incomplete `head_blocks` | Today's `persist()` calls `save_snapshot` after every connect/disconnect/reorg. That is the snapshot-only contract to replace. Keep `MemoryChainstateStore` as an in-memory view for tests. |
| `open-bitcoin-node` sync runtime | Call manager flush (`IfNeeded` after connect, `Periodic` on ticks, `Always` on clean shutdown) instead of `save_chainstate_snapshot` of the full UTXO map | `DurableSyncRuntime::persist_progress` currently writes the JSON snapshot with `config.persist_mode` (default `PersistMode::Flush`). |
| `open-bitcoin-node` / `open-bitcoin-network` serving | Honest availability: `Available` only when `block:` payload exists | `serve_managed_block_request` already refuses a missing lookup. `inventory.rs` still treats `has_local_data \|\| durable_availability` as enough and labels a missing non-tip active block `Pruned`. Change the facts, not the stack. |
| `open-bitcoin-rpc` daemon shell | Drive periodic flush ticks with injected time and sampled 50–70 minute jitter | Use existing Tokio `time` / `test-util`. Sample jitter with existing `getrandom 0.3.4` in the shell. |

### Supporting Libraries and Standard-Library Approaches

| Library / facility | Version | Purpose | When to Use |
|--------------------|---------|---------|-------------|
| `std::collections::HashMap` plus an explicit dirty/fresh index | Rust `1.94.1` | In-memory coins cache | Prefer a dirty-entry list or flagged-entry set so `BatchWrite` does not scan the whole cache. Do not add `indexmap`, `hashbrown`, or a pool allocator crate to copy Knots `PoolAllocator`. |
| First-party compact codec | workspace `0.1.0` | On-disk coin and outpoint encoding | Encode `Coin { output, is_coinbase, created_height, created_median_time_past }`. Knots on-disk `Coin` omits MTP; copying that format would drop an in-scope Open Bitcoin field. |
| Existing `PersistMode` | node crate | Durability of a flush batch | `Buffered` for in-flush partial batches, `Flush` for journal-to-OS, `Sync` for shutdown / schema / recovery-marker writes. Fjall documents that `persist` affects durability, not consistency; the `head_blocks` protocol is what makes a multi-batch flush restart-safe. |
| `getrandom` | `0.3.4`, existing | 50–70 minute periodic-write jitter | Sample in the node/RPC shell and pass the next-write deadline into pure policy. Match Knots `DATABASE_WRITE_INTERVAL_MIN/MAX` (`50min` / `70min`). |
| `FjallNodeStore::contains_key` / `get` / `size_of` | Fjall `3.1.4` | Honest block-payload presence | Prefer existence of the `block:` key over header-index membership. Load the payload only when serving. |
| Existing `StorageRecoveryAction` | node crate | Operator-visible recovery | Map incomplete `head_blocks` or schema mismatch to `Reindex` / `Repair` / `RestoreFromBackup`. Do not add automatic destructive repair. |

### Development Tools

| Tool | Purpose | Notes |
|------|---------|-------|
| `bash scripts/verify.sh` | Repo-native format, lint, build, test, coverage, architecture, parity-breadcrumb, and Bazel contract | Remains the deterministic default gate. Do not add public-network or full-mainnet UTXO-set checks to the default gate. |
| Existing Rust unit tests + temporary Fjall stores | Cache flag combinations, flush vs sync, two-phase `head_blocks`, restart reopen, missing-payload serving | Inject clocks and cache limits. Use `simulate_crash` style fixtures in the adapter (Knots `CoinsViewOptions.simulate_crash_ratio`) without process `_Exit`. |
| Pinned Knots tests | Source of parity cases | Mine `coins.cpp` flag/flush behavior, `txdb.cpp` `BatchWrite`, `validation.cpp` `FlushStateToDisk` / `GetCoinsCacheSizeState`, and `test/chainstate_write_tests.cpp` for the 50–70 minute window. |
| Existing `open-bitcoin-bench` | Detect full-map clone and full-snapshot encode regressions | Add connect/flush workloads that dirty a bounded coin set; do not add Criterion. |

## Detailed Stack Decisions

### Disk-backed coins: extend Fjall, do not add LevelDB

Knots `CCoinsViewDB` is a LevelDB wrapper (`txdb.h`) with:

- per-outpoint `DB_COIN` records
- `DB_BEST_BLOCK` when consistent
- `DB_HEAD_BLOCKS = [new_tip, old_tip]` while a flush is in progress
- batched dirty writes, default `nDefaultDbBatchSize = 64 << 20` (64 MiB)
- cursor iteration over coin keys

Fjall `3.1.4` already provides the required primitives on the same `Database` the node opens today:

- atomic `WriteBatch` across keyspaces
- per-key get / insert / remove / `contains_key`
- prefix and range iterators for a coins cursor
- `persist(PersistMode::{Buffer,SyncAll})`
- `disk_space()` for low-disk classification already mapped to `StorageRecoveryAction::FreeDisk`

There is no documented capability gap that justifies `rusty-leveldb`, `leveldb-sys`, RocksDB, or a second process-local store. A second database would split crash ordering between coins and block payloads, which is the opposite of fuller manager behavior.

**Layout recommendation:** add a dedicated `coins` keyspace (new `StorageNamespace::Coins`) rather than stuffing millions of outpoints next to the current `chainstate` `"snapshot"` key. Keep tip/undo/active-chain metadata in `chainstate` as small records, not as a cloned UTXO map.

**Record format:** first-party compact binary. The current `encode_chainstate_snapshot` path uses `serde_json::to_vec_pretty`. That is acceptable for bounded mempool/runtime metadata and unacceptable as coin truth: every connect currently rewrites the entire set. Knots writes only dirty coins.

**Schema:** `SchemaVersion::CURRENT` is `1`. Introducing a coins keyspace and retiring snapshot-as-coin-truth is a breaking store layout. Bump the schema, fail closed on mismatch (existing `StorageError::schema_mismatch`), and treat leftover `"snapshot"` blobs as non-authoritative once coins records exist. Do not silently convert a JSON snapshot into per-coin rows without an explicit, testable migration step in this milestone.

**Crash protocol:** implement Knots `CCoinsViewDB::BatchWrite` in the adapter:

1. Erase `best_block`, write `head_blocks = [new, old]`.
2. Write dirty coins in batches bounded by 64 MiB of estimated payload (first-party size accounting; Fjall batches expose `len()`, not LevelDB `SizeEstimate`).
3. Erase `head_blocks`, write `best_block = new`.
4. On reopen, if `head_blocks` is a two-element vector and `best_block` is missing, resume only when `head_blocks[0]` matches the flush target; otherwise return a typed `Reindex` recovery error.

A single Fjall batch is atomic, but large flushes still need this protocol because Knots (and this milestone) must be able to split a 64 MiB-capped write sequence. Fjall's own docs state persist changes durability, not consistency; the head-blocks keys are the application consistency marker.

### Cache-flush policy: first-party, injected time, no prune product

Implement the Knots policy in pure code, then let the node shell apply it:

| Mode | When the shell asks | What the cache does |
|------|---------------------|---------------------|
| `None` | Observability / future prune hook only | Do not write coins. Do not implement file pruning. |
| `IfNeeded` | After connect / reorg | Write if cache is `Critical` (over budget). Knots also flushes when `SystemNeedsMemoryReleased()`; treat that as an injected boolean from the shell, not a new crate. |
| `Periodic` | Daemon tick | Write if cache is `Large` (≥ 90% of budget, or within 10 MiB of the limit) **or** `now >= next_write`. Periodic writes `Sync` (keep unspent cache entries). Large/critical writes `Flush` (wipe cache). |
| `Always` | Clean shutdown, explicit force flush | `Flush` the cache. |

Knots numbers to pin in first-party constants (do not pull them from a C++ crate):

- default kernel/dbcache budget: `450 MiB` (`DEFAULT_KERNEL_CACHE`)
- minimum dbcache: `4 MiB`
- coins-DB cache cap: `8 MiB` (Fjall block cache is store-level; do not pretend to resize LevelDB)
- coins-tip cache gets the remainder after the small index/coins-db reservation
- large-flush warning threshold: Knots `WARN_FLUSH_COINS_SIZE`
- disk-space guard: `48 * 2 * 2 * cache_entry_count` bytes before a coins write
- periodic window: 50–70 minutes, sampled once per successful write

`ManagedChainstate` must stop calling `save_snapshot` inside `connect_block` / `disconnect_tip` / `reorg`. Connect updates the cache and records undo; flush is a separate manager step. `prepare_*` / `commit_*` can stay, but commit should not imply a full UTXO rewrite.

Do not port `PruneAndFlush`, `FindFilesToPrune`, or `UnlinkPrunedFiles`. `blockstorage.cpp` flush that **is** in scope is the ordering constraint: flush block/undo payloads before coins, then treat a missing block body as unavailable. Open Bitcoin already stores blocks under `block:` keys; the manager should persist those payloads (existing `save_block`) before a coins `best_block` advance.

### Fuller chainstate-manager behavior

Knots splits this across `CoinsViews`, `Chainstate`, and `ChainstateManager`:

- `CoinsViews`: disk view → read-error catcher → tip cache
- `InitCoinsDB` then health check then `InitCoinsCache` (`CanFlushToDisk` is false until the cache exists)
- `CoinsTip()` is the only spend/connect view
- `FlushStateToDisk` writes block/undo, then block index, then coins
- assumeutxo / background snapshot chainstates exist on `ChainstateManager` and are **out of scope**

Open Bitcoin should grow `ManagedChainstate` to own that active-chainstate subset only:

1. Open Fjall coins + existing block/header namespaces.
2. If `head_blocks` is inconsistent, do not install a cache and do not flush.
3. Install the cache only after the disk view is readable.
4. Serve `GetCoin` through the cache, falling back to Fjall on miss.
5. Persist undo per block as its own record (today undo lives inside the JSON snapshot). A disconnect must not require reloading a full UTXO blob.
6. Keep one active chainstate. Do not add a second snapshot chainstate or `EmplaceCoinInternalDANGER`.

`ChainstateStore` should change from `{ load_snapshot, save_snapshot }` to a view-backed store (`get_coin`, `batch_write`, `best_block`, `head_blocks`, `load_undo`, `save_undo`, `has_block_payload`). `MemoryChainstateStore` implements the same trait in RAM for tests.

### Honest availability: no new stack

`serve_managed_block_request` already takes `lookup_block: FnOnce(BlockHash) -> Option<Block>` and returns `LookupUnavailable` when the payload is missing. The defect is upstream classification in `inventory.rs`:

- `has_local_data` is an in-memory map
- `durable_availability` is a boolean that can be true without a payload get
- active + not tip + missing data is labeled `Pruned`

v2.3 should set `data_availability` from an actual store probe (`contains_key` / `get` on `block:{hex}`). Missing payload is `Unavailable`, not `Pruned`. Keep the `Pruned` enum for later product work; do not report it from “index says yes, body says no.”

Operator status, RPC, and support evidence should use the same probe. Do not add a search index, object store, or archive-mode crate.

## Installation

No Cargo packages or system services should be added for v2.3.

```bash
# Materialize the pinned behavioral baseline if needed.
git submodule update --init --recursive

# Verify the unchanged dependency/toolchain contract after implementation.
# Do not treat this research step as a reason to run verify now.
bash scripts/verify.sh
```

If implementation proposes a dependency despite this recommendation, require a written capability gap against Fjall `3.1.4` (`batch`, `persist`, `contains_key`, prefix/range), a maintenance/security review, Cargo and Bazel wiring, and evidence that a first-party adapter is less safe.

Do not bump Fjall to crates.io `3.1.5` (current docs.rs latest as of this research) as part of this milestone. The lockfile pin is `3.1.4` (`b62b25b4d815ae178d7d9e4aa32ee59f072efd5431c736abede1e6ee13c8c453`).

## Alternatives Considered

| Recommended | Alternative | When to Use Alternative |
|-------------|----------------|-------------------------|
| Fjall `3.1.4` coins keyspace | rusty-leveldb / LevelDB / Bitcoin `dbwrapper` | Only if a later milestone requires byte-identical Knots `chainstate/` files for datadir migration. Migration apply is explicitly out of scope. |
| Fjall `3.1.4` coins keyspace | RocksDB / `rust-rocksdb` | Only if measured flush/read latency on a full UTXO set proves Fjall cannot meet an explicit budget **and** RocksDB passes dependency review. Do not preempt that with a rewrite. |
| First-party compact coin codec | serde JSON `UtxoRecordDto` per coin | Never for live coin truth. JSON may remain for small metadata and tests. |
| First-party compact coin codec | rust-bitcoin / bitcoinconsensus codecs | Forbidden on the production path. The workspace already owns `Coin`, `OutPoint`, scripts, and compact-size. |
| `HashMap` + dirty index | `indexmap`, `hashbrown`, `lru` | Only after a cache-size benchmark shows standard-library maps cannot meet the 450 MiB accounting model. Knots pool-allocator tricks are not required for correctness. |
| Injected periodic tick in RPC shell | New scheduler / job crate | Never for one interval. Tokio `time` is already in `open-bitcoin-rpc`. |
| `contains_key` on `block:` | Infer availability from header/index snapshot | Never. That is the dishonest path this milestone exists to close. |
| One active `ManagedChainstate` | Knots dual IBD + assumeutxo chainstates | Later milestone. `EmplaceCoinInternalDANGER` and snapshot-base chainstates stay unused. |

## What NOT to Use

| Avoid | Why | Use Instead |
|-------|-----|-------------|
| Third-party Rust Bitcoin domain libraries | Violates the production-path ownership policy and would re-encode `Coin` without `created_median_time_past` | Existing `open-bitcoin-primitives` / `chainstate` / `codec` types |
| LevelDB, rusty-leveldb, `dbwrapper` ports | No capability gap over Fjall for per-key coins, batches, and cursors; splits crash domains | Extend `FjallNodeStore` with a coins keyspace |
| RocksDB or a second LSM | Extra native toolchain, extra lock/datadir story, no required API | Same Fjall `Database` the node already opens |
| Fjall `OptimisticTxDatabase` / `SingleWriterTxDatabase` | Coins flush is write-batch + application `head_blocks`, not interactive read-modify-write | Existing `db.batch()` |
| Pretty JSON as UTXO truth | Rewrites the whole set on every persist; unbounded encode cost | Compact per-outpoint records + dirty-set flush |
| Tokio, wall clocks, or `getrandom` in `open-bitcoin-chainstate` | Breaks functional-core tests and architecture policy | Inject `now`, jitter, and persist mode from the shell |
| Prune-mode file managers / `fFlushForPrune` | Out of milestone scope; current code already mis-labels missing bodies as `Pruned` | Payload-present → `Available`, else `Unavailable` |
| assumeutxo / assumevalid / snapshot-load APIs | Out of scope; Knots `EmplaceCoinInternalDANGER` is snapshot-only | Single active chainstate from genesis connect + coins DB |
| Bumping Fjall to `3.1.5` “while we are here” | Invented upgrade; lockfile pin is the source of truth | Stay on `3.1.4` unless a reproduced API bug requires a documented bump |

## Stack Patterns by Variant

**If implementing the coins cache (pure core):**

- Use a view trait with an in-memory cache adapter and a test memory backend.
- Track `DIRTY` / `FRESH` exactly as Knots documents the five valid combinations.
- `Flush` wipes the cache after a successful `BatchWrite`; `Sync` writes dirty entries and drops spent ones.
- Never open Fjall from `open-bitcoin-chainstate`.

**If implementing the coins DB (node shell):**

- One Fjall `Database`, new `coins` keyspace, existing `PersistMode` mapping.
- Two-phase `head_blocks` around 64 MiB batches.
- Init DB → health check → init cache. No flush until `CanFlushToDisk`.
- Bump `SchemaVersion` and fail closed.

**If implementing flush policy:**

- Pure decision function: `(mode, cache_state, now, next_write, memory_pressure) -> FlushAction { None, Sync, Flush }`.
- Shell executes the action, updates `next_write`, and records evidence.
- After connect: `IfNeeded`. On timer: `Periodic`. On shutdown: `Always`.

**If implementing honest availability:**

- Probe `block:` payload keys.
- Serve only after `get` returns bytes that decode as a block.
- Report `Unavailable` when the index or tip mentions a hash whose payload is missing.
- Do not introduce prune-mode serving behavior.

**If a later milestone needs Knots datadir import:**

- That is the only plausible reason to revisit LevelDB or a Knots-compatible coin record layout.
- It is not a v2.3 stack change.

## Version Compatibility

| Package A | Compatible With | Notes |
|-----------|-----------------|-------|
| Rust `1.94.1` / edition 2024 | All workspace crates `0.1.0` | `rust-toolchain.toml` and `packages/Cargo.toml` remain the sources of truth. |
| `open-bitcoin-chainstate` `0.1.0` | `open-bitcoin-consensus`, `open-bitcoin-primitives` only | Do not add Fjall, Tokio, or serde to this crate. |
| `open-bitcoin-codec` `0.1.0` | `open-bitcoin-primitives` only | Add coin/outpoint record encoding here, not in the node JSON snapshot codec. |
| `open-bitcoin-node` `0.1.0` | Fjall `3.1.4`, serde `1.0.228`, serde_json `1.0.149`, getrandom `0.3.4` | Extend `FjallNodeStore` and `ManagedChainstate`; bump `SchemaVersion` when coins records become authoritative. |
| `open-bitcoin-rpc` `0.1.0` | Tokio `1.52.1` with `time` and `test-util` | Sufficient for periodic flush wakeups; no new timer framework. |
| Fjall `3.1.4` | Existing `PersistMode` mapping (`Buffered` → no persist, `Flush` → `Buffer`, `Sync` → `SyncAll`) | `SyncData` exists on Fjall but is unused; do not remap unless flush evidence shows `SyncAll` is too strong for periodic writes. |
| Bitcoin Knots `29.3.knots20260210` | v2.3 coins/flush fixtures | Do not mix Core 30+ assumeutxo or later flush defaults into this pin. |

## Sources

All decisive stack and version claims were verified from local pins or official docs. Web-only “latest crate” mentions are labeled.

- `rust-toolchain.toml` and `packages/Cargo.toml` — Rust `1.94.1`, edition 2024, workspace `0.1.0`. **HIGH confidence.**
- `packages/open-bitcoin-node/Cargo.toml` and `packages/Cargo.lock` — Fjall `3.1.4`, serde `1.0.228`, serde_json `1.0.149`, getrandom `0.3.4`. **HIGH confidence.**
- `packages/open-bitcoin-rpc/Cargo.toml` — Tokio `1.52.1`, axum `0.8.9`. **HIGH confidence.**
- `packages/open-bitcoin-chainstate/Cargo.toml` and `src/engine.rs` — in-memory `HashMap` UTXO truth, no I/O deps. **HIGH confidence.**
- `packages/open-bitcoin-node/src/chainstate.rs` — `ManagedChainstate` snapshot persist after every mutation. **HIGH confidence.**
- `packages/open-bitcoin-node/src/storage.rs` and `storage/fjall_store.rs` — namespaces, `SchemaVersion::CURRENT = 1`, `PersistMode` mapping, `save_chainstate_snapshot` / `save_block` / `load_block`. **HIGH confidence.**
- `packages/open-bitcoin-node/src/storage/snapshot_codec.rs` — JSON pretty snapshot as current coin persistence. **HIGH confidence.**
- `packages/open-bitcoin-node/src/network/inventory.rs` and `network/block_serving.rs` — serving lookup vs pruned/unavailable classification. **HIGH confidence.**
- `packages/bitcoin-knots/src/coins.h`, `coins.cpp` — `CCoinsView` / cache flags / `Flush` vs `Sync`. **HIGH confidence.**
- `packages/bitcoin-knots/src/txdb.h`, `txdb.cpp` — LevelDB coins DB, 64 MiB batches, `best_block` / `head_blocks`. **HIGH confidence.**
- `packages/bitcoin-knots/src/validation.h`, `validation.cpp` — `CoinsViews`, `FlushStateMode`, cache-size state, 50–70 minute write interval, flush-before-coins ordering. **HIGH confidence.**
- `packages/bitcoin-knots/src/kernel/caches.h`, `node/caches.h` — 450 MiB default, 8 MiB coins-DB cap, 4 MiB minimum. **HIGH confidence.**
- `packages/bitcoin-knots/src/node/blockstorage.cpp` — `FlushChainstateBlockFile` / block+undo flush; prune helpers noted only to exclude them. **HIGH confidence.**
- [Fjall `3.1.4` `Database`](https://docs.rs/fjall/3.1.4/fjall/struct.Database.html), [`Keyspace`](https://docs.rs/fjall/3.1.4/fjall/struct.Keyspace.html), [`PersistMode`](https://docs.rs/fjall/3.1.4/fjall/enum.PersistMode.html) — batch, persist, get/contains_key/prefix/range. **HIGH confidence.**
- [crates.io / docs.rs fjall latest `3.1.5`](https://docs.rs/crate/fjall/latest) — newer than the repo pin; do not upgrade in this milestone. **MEDIUM confidence** (registry latest can move; pin is authoritative).
- `standards/core/architecture.md` — functional core / imperative shell. **HIGH confidence.**

---
*Stack research for: Bitcoin chainstate durability (coins DB, cache flush, chainstate manager)*
*Researched: 2026-08-29*
