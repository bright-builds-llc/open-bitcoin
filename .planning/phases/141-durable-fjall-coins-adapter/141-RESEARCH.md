# Phase 141: Durable Fjall Coins Adapter - Research

**Researched:** 2026-09-04
**Domain:** Fjall per-outpoint coins persistence, Knots `CCoinsViewDB` BatchWrite, schema 1→2 migration
**Confidence:** HIGH

<user_constraints>
## User Constraints (from CONTEXT.md)

**CRITICAL:** If CONTEXT.md exists from /gsd-discuss-phase, copy locked decisions here verbatim. These MUST be honored by the planner.

### Locked Decisions

#### Per-coin key schema

- **D-01:** Use Knots-shaped first-party prefixes inside a dedicated coins keyspace: `C` + 32-byte txid + 4-byte little-endian vout for a coin, `B` for coins best-block, and `H` for `head_blocks`. Values are compact binary, not pretty JSON. Do not require LevelDB `chainstate/` byte-identical keys.
- **D-02:** Erase is a missing key, not a stored spent tombstone on disk. Spent DIRTY entries that survive cache Flush become deletes in the parent batch. A successful `get_coin` miss is `Ok(None)`.
- **D-03:** Do not introduce LevelDB, rusty-leveldb, RocksDB, or rust-bitcoin. Reuse Fjall `3.1.4` `batch` / `persist` / `contains_key` and existing `PersistMode::{Buffered,Flush,Sync}`.

#### Dedicated coins keyspace

- **D-04:** Add `StorageNamespace::Coins` (`"coins"`). Do not stuff millions of outpoints into the existing `chainstate` `"snapshot"` neighborhood.
- **D-05:** Keep small tip / undo / active-chain metadata in the existing `chainstate` keyspace as small records, not as a cloned UTXO map.
- **D-06:** `FjallCoinsView` lives in `open-bitcoin-node` storage and implements the Phase 139 `CoinsView` trait. `CoinsCache` must stay ignorant of whether the parent is `MemoryCoinsView` or `FjallCoinsView`. Do not put Fjall, `std::fs`, Tokio, or a clock in `open-bitcoin-chainstate`.

#### Dirty-batch accounting and head_blocks

- **D-07:** Bound dirty coin writes at `64 << 20` bytes of first-party estimated encoded payload (`nDefaultDbBatchSize`). Fjall batch `len()` is item count, not LevelDB `SizeEstimate`; do not treat count as size.
- **D-08:** Encode `head_blocks` as a two-element `[new, old]` `BlockHash` vector. Implement Knots `CCoinsViewDB::BatchWrite` in the adapter:
  1. Erase `B`, write `H = [new, old]`.
  2. Write dirty coins in 64 MiB-capped batches (`Buffered` for in-flush partials).
  3. Erase `H`, write `B = new` (`Flush` for journal-to-OS; `Sync` for shutdown / schema / recovery-marker writes).
- **D-09:** On reopen, empty `H` plus present `B` is consistent. A two-element `H` with missing `B` is an interrupted-flush marker and must fail closed as a typed storage or recovery error in this phase. Do not implement `ReplayBlocks` here. Phase 142 chooses replay-versus-fail-closed execution when bodies or undo are present.
- **D-10:** Any other `H` count, mismatched `H[0]`, or decode failure is inconsistent and fails closed. Do not auto-repair. Fjall persist changes durability, not consistency; `head_blocks` is the application consistency marker.

#### Compact codec home

- **D-11:** Place the compact per-outpoint codec at `packages/open-bitcoin-node/src/storage/coins_codec.rs`. Parse bytes into domain `Coin` at the Fjall boundary. Include `created_height` and `created_median_time_past`.
- **D-12:** Do not reuse `encode_chainstate_snapshot` pretty JSON as coin truth. Snapshot JSON remains a leftover migration source only.

#### Undo records vs leftover snapshot DTO

- **D-13:** Persist undo as its own durable records (`save_undo` / `load_undo`) in the `chainstate` keyspace. A disconnect must not require reloading a full UTXO blob. Do not keep undo live inside the leftover snapshot DTO after migration.
- **D-14:** `save_block` / `load_block` stay the payload home and stay unchanged. Phase 142 flushes block / undo / index before coins; this phase only makes undo independently readable.
- **D-15:** `ChainstateSnapshot` remains a hydrate / export / test helper. After the one-way migration, reopen must not treat the leftover `"snapshot"` blob as UTXO truth.

#### Schema bump and one-way migration

- **D-16:** Bump `SchemaVersion::CURRENT` from `1` to `2`. This is a breaking store layout. Fail closed on mismatch through the existing `StorageError::schema_mismatch` path.
- **D-17:** Do not invent per-namespace schema versions. Wallet, mempool, and runtime snapshot codecs stay readable under schema 2; only coin-truth layout changes. One store-level version covers the cutover.
- **D-18:** Explicit one-way migration, not dual-read and not silent convert:
  - Fresh datadir writes schema 2 with an empty coins keyspace and no snapshot blob.
  - Schema 1 plus leftover `"snapshot"` and empty coins: migrate once into per-outpoint coins plus undo records, then write schema 2.
  - Schema 2: coins keyspace is UTXO truth. A leftover `"snapshot"` blob is non-authoritative and must not hydrate the live view.
  - Schema 2 plus empty coins plus leftover snapshot: fail closed. Do not remigrate.
  - Any other version: `schema_mismatch`.
- **D-19:** Do not cut over `ManagedChainstate::persist` or `DurableSyncRuntime::persist_progress` off leftover snapshot *writes*. Phase 142 owns that write-site cutover and manager flush wiring. This phase makes reopen / load treat coins as truth after migration.

#### Fail-closed disk reads

- **D-20:** A coins disk-read I/O error, corruption, or decode failure is a typed `StorageError` / recovery error. Never map it to spent, missing, or `Ok(None)`. `Ok(None)` is only a successful absence after a completed lookup.
- **D-21:** Do not swallow Fjall errors inside `FjallCoinsView::get_coin` / `have_coin` / `best_block` / `head_blocks`. Surface them through the `CoinsView` `Result` so a later manager cannot treat a read failure as an empty UTXO set.
- **D-22:** `MemoryChainstateStore` stays the in-memory test parent and must implement the same view-backed store surface (`get_coin`, `batch_write`, `best_block`, `head_blocks`, `load_undo`, `save_undo`). Tests may use a `cfg(test)` simulate-crash seam without process `_Exit`.

### Claude's Discretion

- Exact binary layout of compact coin values (varint vs fixed fields)
  provided MTP and height round-trip and decode failures fail closed.
- Exact `chainstate` undo key spelling (`undo:` + block hash or
  equivalent) provided undo is not inside the leftover snapshot DTO.
- Whether `FjallCoinsView` is a standalone type or a focused module
  beside `FjallNodeStore`, provided Fjall stays in the node shell.
- How `ChainstateStore` grows from `{load_snapshot, save_snapshot}` to
  the view-backed surface without breaking Phase 139/140 compile of
  leftover persist write sites.

### Deferred Ideas (OUT OF SCOPE)

- Manager init, CanFlush readiness, IfNeeded-after-connect / Periodic-tick
  / Always-on-shutdown wiring, ordered block/undo/index-then-coins flush,
  jitter resampling, dbcache default budgets, crash-loss window,
  interrupted-flush replay vs fail-closed when bodies or undo are
  missing, persist_progress write-site cutover — Phase 142
- Honest payload-present availability — Phase 143
- Operator flush/recovery evidence and last-flush-reason surfaces —
  Phase 144
- Parity-root closeout and no-claim guardrails — Phase 145
- Wallet rescan / confirmation migration off the snapshot blob as a
  leftover consumer rewrite — later manager/consumer follow-through
  once coins truth exists
- Assumeutxo dual-chainstate, prune/archive product modes, LevelDB,
  rust-bitcoin, Knots `chainstate/` file compatibility — out of v2.3
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| COIN-01 | Node persists spendable UTXOs as per-outpoint durable coin records with coins best-block and interrupted-flush markers, not as live snapshot-blob truth. | Dedicated `coins` keyspace; `C`/`B`/`H` keys; compact `coins_codec.rs`; Knots two-phase `BatchWrite`; schema 1→2 one-way migration; leftover `"snapshot"` ignored after coins exist. |
| CSOBS-03 | A coins disk-read error fails closed as a typed storage or recovery error, not as spent or missing. | Fjall `get`/`contains_key` `Err` and decode failure map to `StorageError` / `ChainstateError` storage variant; never `Ok(None)` or `MissingCoin`. Make `CoinsView::best_block` / `head_blocks` fallible. |
</phase_requirements>

<research_summary>
## Summary

Phase 141 is a node-shell persistence-shape change on the existing Fjall `3.1.4` pin. Do not add crates. Spendable UTXOs move from the leftover pretty-JSON `"snapshot"` blob into a dedicated `coins` keyspace with Knots-shaped `C`/`B`/`H` prefixes, compact per-outpoint values (height + MTP), and the two-phase `BatchWrite` protocol. `open-bitcoin-chainstate` stays I/O-free: it already has `CoinsView` / `CoinsCache` / `MemoryCoinsView` / `decide_recovery`. The disk parent is `FjallCoinsView` in `open-bitcoin-node` storage.

The current `ensure_schema` path treats any version other than `SchemaVersion::CURRENT` as `schema_mismatch`. Bumping `CURRENT` from `1` to `2` therefore requires rewriting open/migrate so schema `1` plus leftover snapshot can migrate once before the store is considered current. `DurableSyncRuntime::persist_progress` and `ManagedChainstate::persist` keep writing leftover snapshots (D-19). Reopen / load after coins exist must not copy leftover `utxos` into the live view (D-15, D-18).

**Primary recommendation:** Add `StorageNamespace::Coins`, `coins_codec.rs`, and a standalone `FjallCoinsView` that implements `CoinsView` with fallible `best_block` / `head_blocks`. Implement first-party 64 MiB encoded-byte batching and the `H`/`B` write protocol. Run an explicit one-way schema 1→2 migration on open. Fail closed on coins I/O, decode, interrupted `H`, and schema 2 + empty coins + leftover snapshot. Do not implement `ReplayBlocks`, manager flush wiring, or persist write-site cutover.
</research_summary>

## Project Constraints (from `.cursor/rules/` and repo guidance)

No `.cursor/rules/` directory exists in this repo. [VERIFIED: workspace glob]

Honor these repo-local / Bright Builds constraints with the same authority as locked CONTEXT decisions:

- Functional core / imperative shell: Fjall, `std::fs`, Tokio, and clocks stay out of `open-bitcoin-chainstate`. [VERIFIED: `standards/core/architecture.md`, `scripts/pure-core-crates.txt`, `scripts/check-pure-core-deps.sh`]
- `open-bitcoin-chainstate` depends only on `open-bitcoin-consensus` and `open-bitcoin-primitives`. [VERIFIED: `packages/open-bitcoin-chainstate/Cargo.toml`]
- No rust-bitcoin / LevelDB on the production path. [VERIFIED: `AGENTS.md`, `.planning/PROJECT.md`]
- New first-party Rust sources under `packages/open-bitcoin-*/src` need parity breadcrumbs via `docs/parity/source-breadcrumbs.json`. [VERIFIED: `AGENTS.md`]
- Prefer `foo.rs` plus `foo/` over `foo/mod.rs`. [VERIFIED: `standards/languages/rust.md`]
- Unit-test pure codec and fail-closed mapping. Arrange / Act / Assert; one concern per test. [VERIFIED: `standards/core/testing.md`]
- Repo-native verification is `bash scripts/verify.sh`. Ad-hoc Cargo goes through `bun run scripts/command-timings.ts run --key <stable-key> -- <command>`. [VERIFIED: `AGENTS.md`]
- Do not use `unwrap()` in production Rust. [VERIFIED: user code-styling rule; crate deny lists]

<standard_stack>
## Standard Stack

No new production crates. Do not bump Fjall.

### Core

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| Rust | `1.94.1`, edition 2024 | Typed keys, `Result` fail-closed mapping | Pinned by `rust-toolchain.toml` / `packages/Cargo.toml`. [VERIFIED: repo pins] |
| Fjall | `3.1.4`, `default-features = false` | Dedicated `coins` keyspace, `WriteBatch`, `get` / `contains_key` / `prefix`, `persist` | Already backs `FjallNodeStore`. `WriteBatch::len()` is item count only. [VERIFIED: `packages/open-bitcoin-node/Cargo.toml`, Fjall `3.1.4` source] |
| `open-bitcoin-chainstate` `0.1.0` | workspace | `CoinsView`, `CoinsCache`, `Coin`, `decide_recovery` | I/O-free parent contract. [VERIFIED: `coins.rs`, `Cargo.toml`] |
| `open-bitcoin-codec` `0.1.0` | workspace | `Reader`, `write_*_le`, `read_compact_size` / `write_compact_size` | First-party compact primitives for `coins_codec.rs`. [VERIFIED: `packages/open-bitcoin-codec/src`] |
| Bitcoin Knots | `29.3.knots20260210` | `CCoinsViewDB` prefixes and `BatchWrite` | Vendored `txdb.h` / `txdb.cpp`. [VERIFIED: local submodule] |

### Supporting

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| Existing `PersistMode` | node crate | `Buffered` → no Fjall persist; `Flush` → `PersistMode::Buffer`; `Sync` → `PersistMode::SyncAll` | Partial coin batches use `Buffered`. Final `B` write uses `Flush`. Schema / migration / recovery markers use `Sync`. [VERIFIED: `fjall_store.rs` `fjall_persist_mode`] |
| Existing `StorageError` | node crate | `BackendFailure`, `Corruption`, `InterruptedWrite`, `schema_mismatch` | Coins I/O → `BackendFailure`; decode → `Corruption`; two-element `H` + missing `B` → `InterruptedWrite`. [VERIFIED: `storage.rs`] |
| serde / serde_json | `1.0.228` / `1.0.149` | Leftover snapshot DTO and small undo/meta JSON if reused | Migration source and optional small `chainstate` records only. Never live coin values. [VERIFIED: `snapshot_codec.rs`] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Fjall `3.1.4` coins keyspace | LevelDB / rusty-leveldb | Forbidden (D-03). Only later Knots datadir import. |
| First-party compact coin values | serde JSON `CoinDto` per outpoint | Forbidden as live truth (D-12). JSON stays leftover migration only. |
| First-party compact coin values | rust-bitcoin / Knots `TxOutCompression` | Forbidden. Workspace already owns `TransactionOutput` encode shape. |
| `coins_codec.rs` in node | `open-bitcoin-codec` public `encode_tx_out` | STACK suggested codec-crate home; CONTEXT D-11 locked node `coins_codec.rs`. Call public codec primitives from that file. |
| Fjall `OptimisticTxDatabase` | Existing `db.batch()` | Coins flush is write-batch + application `H`/`B`, not interactive txn. [VERIFIED: STACK.md] |

**Installation:** none. Keep Fjall `3.1.4`.

**Version verification:** Fjall pin `3.1.4` in `packages/open-bitcoin-node/Cargo.toml` and lockfile. Do not upgrade to crates.io `3.1.5`. [VERIFIED: `Cargo.toml`]
</standard_stack>

<architecture_patterns>
## Architecture Patterns

### Recommended Project Structure

```
packages/open-bitcoin-chainstate/src/
├── coins.rs                 # Extend CoinsView: fallible best_block + head_blocks
├── coins/memory.rs          # MemoryCoinsView implements the same trait
├── coins/cache.rs           # Propagate parent Result; stay I/O-free
└── error.rs                 # Add CoinsStorage { detail } — not MissingCoin

packages/open-bitcoin-node/src/
├── storage.rs               # StorageNamespace::Coins, SchemaVersion::CURRENT = 2
├── storage/
│   ├── coins_codec.rs       # Keys + coin / heads / best-block encode-decode
│   ├── coins_view.rs        # Standalone FjallCoinsView: CoinsView + BatchWrite
│   ├── fjall_store.rs       # Open coins keyspace; raw byte get/put; migrate on open
│   ├── fjall_store/coins.rs # save_undo/load_undo, migrate_schema_1_coins, crash seam
│   └── snapshot_codec.rs    # Leftover decode only; optional undo DTO reuse
└── chainstate.rs            # Grow ChainstateStore; keep save_snapshot/load_snapshot
```

### Pattern 1: Parse at the Fjall boundary

**What:** Raw Fjall bytes become `Coin` / `BlockHash` in `coins_codec.rs`. The engine only sees domain types.

**When to use:** Every coins read/write.

**Example:**

```rust
// Source: standards/core/architecture.md (parse at boundaries)
// Home locked by D-11.
pub fn decode_coin(bytes: &[u8]) -> Result<Coin, StorageError> {
    let mut reader = open_bitcoin_codec::primitives::Reader::new(bytes);
    let coin = parse_coin_fields(&mut reader)?;
    reader.finish().map_err(|error| corruption(StorageNamespace::Coins, error))?;
    Ok(coin)
}
```

### Pattern 2: Knots two-phase BatchWrite on Fjall batches

**What:** Application consistency is `H` then coins then `B`. Each Fjall `WriteBatch` is atomic, but a 64 MiB-capped flush is a sequence of batches. Persist changes durability, not consistency. [VERIFIED: Knots `txdb.cpp` `CCoinsViewDB::BatchWrite`; Fjall `Database::persist` docs in `db.rs`]

**When to use:** `FjallCoinsView::batch_write` only. `MemoryCoinsView::batch_write` stays an in-memory map update with no `H` protocol.

**Example:**

```rust
// Source: packages/bitcoin-knots/src/txdb.cpp BatchWrite
// 1) Erase B, write H = [new, old]          PersistMode::Buffered
// 2) Dirty coins, split at DEFAULT_COINS_DB_BATCH_BYTES
//    spent DIRTY -> batch.remove(C key)
//    unspent DIRTY -> batch.insert(C key, compact value)
//    partial commits: PersistMode::Buffered
// 3) Erase H, write B = new                 PersistMode::Flush (or Sync if requested)
```

On reopen, do **not** resume an in-flight `H` the way Knots `BatchWrite` does when `old_tip` is null and `old_heads.size()==2`. D-09 fail-closes that marker this phase. If `H` is already present when `batch_write` starts, return `InterruptedWrite` and do not write more coins.

### Pattern 3: One-way schema gate on store open

**What:** `ensure_schema` today writes `CURRENT` if missing and otherwise requires `actual == CURRENT`. After `CURRENT = 2`, a schema-1 datadir would fail closed before migration unless open special-cases version `1`. [VERIFIED: `fjall_store.rs` `ensure_schema` / `validate_schema_version`]

**When to use:** `FjallNodeStore::open` only. Wallet / mempool / runtime codecs stay unchanged.

**Example:**

```text
missing schema     → write 2, empty coins, no snapshot
actual == 1
  leftover snapshot + empty coins → migrate coins+undo, write schema 2 (Sync)
  no leftover snapshot            → write schema 2, empty coins
actual == 2
  leftover snapshot + empty coins (no B, no C keys) → fail closed, do not remigrate
  otherwise → coins are UTXO truth; leftover blob ignored
other              → StorageError::schema_mismatch
```

### Pattern 4: Grow `ChainstateStore` without retargeting leftover writes

**What:** Keep `load_snapshot` / `save_snapshot` so `ManagedChainstate::persist` and `persist_progress` still compile and still write leftover blobs (D-19). Add view-backed methods. After coins exist, `DurableSyncRuntime::open` must not copy leftover `utxos` into `MemoryChainstateStore`.

**When to use:** Node store trait + open hydrate only. Do not genericize `Chainstate` off `MemoryCoinsView` this phase (manager attach is Phase 142).

**Recommended hydrate after coins exist:**

1. Load UTXOs by scanning `C`-prefix keys into a `MemoryCoinsView` (same RAM cost as today's snapshot hydrate; Phase 142 attaches `FjallCoinsView` as the live parent).
2. Load undo from `undo:` records.
3. Load `active_chain` / confirmation counts from leftover snapshot metadata **or** a small `chain_meta` record written during migration (D-05). Never copy leftover `utxos`.
4. If leftover snapshot is present and coins are empty (no `B`, no `C` keys) under schema 2: fail closed.

Accepted seam until Phase 142: leftover persist writes may be newer than coins. Reopen follows coins. Tests must not expect leftover-only post-migration connects to survive reopen.

### Anti-Patterns to Avoid

- **Fjall or `std::fs` in `open-bitcoin-chainstate`:** Architecture-policy / pure-core dep check fails. Keep the adapter in node storage.
- **Dual truth:** Writing coins *and* hydrating reopen from leftover `utxos`. Leftover writes may continue; leftover UTXO *reads* after coins exist must not.
- **`WriteBatch::len()` as 64 MiB:** That is item count. [VERIFIED: Fjall `batch/mod.rs` `len()`]
- **Resuming Knots `BatchWrite` when `H` is already two-element:** That is `ReplayBlocks` / in-flush resume. D-09 fail-closes this phase.
- **`best_block() -> Option` swallowing Fjall errors:** Today's trait is infallible. D-21 requires `Result`. Change the trait; `MemoryCoinsView` returns `Ok`.
- **Mapping Fjall `Err` or decode failure to `Ok(None)` / `MissingCoin`:** That is CSOBS-03. Knots `CCoinsViewErrorCatcher` aborts rather than return not-found. [VERIFIED: `coins.cpp` `ExecuteBackedWrapper`]
- **String `&str` keys for `C` records:** Current `put_bytes` takes `&str`. Coin keys are raw bytes and are not UTF-8. Add raw `&[u8]` helpers. [VERIFIED: `fjall_store.rs` `put_bytes`; Fjall `Keyspace::get` takes `AsRef<[u8]>`]
- **Deleting leftover snapshot on migrate:** Leave the blob; it becomes non-authoritative. Wallet rescan still reads it until a later consumer rewrite.
- **LevelDB / rust-bitcoin / Fjall bump:** Locked out.
</architecture_patterns>

<dont_hand_roll>
## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Durable KV + atomic multi-key write | Custom file format / second LSM | Fjall `3.1.4` `Database::batch()`, `Keyspace::{get,insert,remove,contains_key,prefix}` | Already opened by `FjallNodeStore`; cross-keyspace batches already used for headers. [VERIFIED: `fjall_store.rs` `save_header_entries`] |
| Bitcoin compact-size / LE integers | New varint crate | `open_bitcoin_codec::{read,write}_compact_size`, `primitives::{Reader, write_u32_le, write_i64_le}` | Canonical compact-size and fail-closed `Reader::finish`. [VERIFIED: codec crate] |
| Leftover snapshot parse | New JSON schema | Existing `decode_chainstate_snapshot` | Migration source only (D-12). [VERIFIED: `snapshot_codec.rs`] |
| Persist mode mapping | New durability enum | Existing `PersistMode` → Fjall `None` / `Buffer` / `SyncAll` | Already tested. [VERIFIED: `fjall_persist_mode`] |
| Schema mismatch / recovery typing | New error crate | `StorageError::{schema_mismatch, Corruption, InterruptedWrite, BackendFailure}` | Open and recovery classifiers already consume these. [VERIFIED: `storage.rs`] |
| Temp Fjall fixtures | New test harness | Existing `temp_store_path` / `write_raw_for_test` / `write_schema_version_for_test` | Adapter tests already plant raw keys. [VERIFIED: `fjall_store/tests.rs`] |
| Cache DIRTY/FRESH algebra | Rewrite cache in the adapter | Existing `CoinsCache::flush` / `sync` | Adapter is a parent; do not change Phase 139 algebra. [VERIFIED: `cache.rs`] |

**Key insight:** Hand-roll only the first-party pieces Fjall and Knots LevelDB cannot share: encoded-byte batch accounting, `C`+txid+LE-vout keys, MTP in the coin value, and the application `H`/`B` marker. Do not hand-roll a store, a JSON UTXO codec, or `ReplayBlocks`.
</dont_hand_roll>

## Runtime State Inventory

This phase is a store-layout migration (schema 1→2, leftover snapshot → coins). Runtime state after a source rename is not the issue; **existing Fjall datadirs** are.

| Category | Items Found | Action Required |
|----------|-------------|------------------|
| Stored data | Operator / test Fjall datadirs with `schema/schema_version` = `1` and optional `chainstate/snapshot` pretty-JSON blob containing `utxos` + `undo_by_block`. [VERIFIED: `fjall_store.rs` `SNAPSHOT_KEY`, `SCHEMA_VERSION_KEY`] | **Data migration on open:** schema 1 + leftover snapshot + empty coins → write `C`/`B` + `undo:` records, then schema 2. Schema 2 leftover blob stays on disk but must not hydrate UTXOs. |
| Live service config | None — verified: no n8n / Datadog / external UI store for coins keys. | None |
| OS-registered state | None — verified: Fjall lives under the node datadir; no launchd/systemd unit embeds schema 1. | None |
| Secrets/env vars | None — verified: schema version is a Fjall key, not an env/SOPS name. | None |
| Build artifacts | Cargo `target/` and Bazel outputs do not store coin keys. | None — rebuild picks up `CURRENT = 2` |

**Nothing found in category:** Live service config, OS-registered state, secrets/env vars, and build artifacts — verified by repo search and current Fjall adapter layout.

After every file in the repo is updated, **schema-1 datadirs still have the old blob**. Open must migrate them. Tests that plant `write_schema_version_for_test(1)` plus a leftover snapshot are the fixture for that path.

<common_pitfalls>
## Common Pitfalls

### Pitfall 1: Bumping `CURRENT` without a schema-1 open path

**What goes wrong:** Every existing datadir fails `schema_mismatch` on first open after the bump.

**Why it happens:** `validate_schema_version` requires `actual == CURRENT`. [VERIFIED: `fjall_store.rs`]

**How to avoid:** Special-case version `1` in `ensure_schema` / a `migrate_coins_layout` helper before the equality check. Fresh missing schema still writes `2`.

**Warning signs:** `incompatible_schema_version_returns_schema_mismatch` still uses `CURRENT + 1` (good). A test that opens a schema-1 leftover store starts failing with mismatch instead of migrating.

### Pitfall 2: Treating Fjall `len()` as 64 MiB

**What goes wrong:** A batch of many tiny coins never splits, or a few large scripts split too early / never.

**Why it happens:** Fjall `WriteBatch::len()` is `data.len()` (item count). Knots uses LevelDB `SizeEstimate`. [VERIFIED: Fjall `batch/mod.rs`; Knots `txdb.cpp` line 130]

**How to avoid:** Accumulate `key.len() + value.len()` (deletes: `key.len()` only) including the `H`/`B` marker writes. Commit when adding the next item would exceed `DEFAULT_COINS_DB_BATCH_BYTES` (`64 << 20`).

**Warning signs:** Tests that insert two 40 MiB values never create a partial batch, or tests that insert 100 empty coins split immediately.

### Pitfall 3: `Ok(None)` on Fjall `Err` or decode failure

**What goes wrong:** A disk error looks like a spent/missing coin; a later manager treats the UTXO set as empty.

**Why it happens:** `get` returns `Result<Option<_>>`. Easy to write `ok().flatten()`. Knots comments that returning false on LevelDB exception is misinterpreted as not-found. [VERIFIED: `coins.cpp` `ExecuteBackedWrapper`]

**How to avoid:** `Fjall get Err` → `StorageError::BackendFailure`. Decode / trailing bytes → `Corruption`. Map into `ChainstateError::CoinsStorage`, never `MissingCoin`. `contains_key` errors fail closed, not `false`.

**Warning signs:** `have_coin` returns `Ok(false)` in a test that injects backend failure.

### Pitfall 4: Leftover snapshot still hydrates `DurableSyncRuntime::open`

**What goes wrong:** After migration, reopen still installs leftover `utxos` into `MemoryChainstateStore`. Coins are dead on restart.

**Why it happens:** `open` currently `load_chainstate_snapshot_with_confirmation_migration` then `save_snapshot`. [VERIFIED: `sync.rs`]

**How to avoid:** After coins exist, build the in-memory store from coins + undo records. Ignore leftover `utxos`. Do not change `persist_progress` writes.

**Warning signs:** A test that migrates, then overwrites leftover `utxos` without touching coins, still sees the leftover set after reopen.

### Pitfall 5: UTF-8 `&str` coin keys

**What goes wrong:** Insert/get panic or corrupt lookups because txid bytes are not UTF-8.

**Why it happens:** Today's helpers take `&str` (`SNAPSHOT_KEY`, `block:{hex}`).

**How to avoid:** Add `put_raw_bytes` / `get_raw_bytes` / `remove_raw_bytes` on `&[u8]`. Hex is only for `undo:` / `block:` string keys.

**Warning signs:** Tests only use ASCII txids; a `0xff`-filled txid fails.

### Pitfall 6: Auto-repair or remigration

**What goes wrong:** Schema 2 + leftover + empty coins silently remigrates a stale blob over a wiped coins keyspace.

**Why it happens:** D-18 fail-closed case looks like "we should migrate again."

**How to avoid:** That combination is `Corruption` / `InterruptedWrite` with `Reindex` or `RestoreFromBackup`. No remigrate. No auto-repair.

**Warning signs:** A planted leftover blob under schema 2 with empty coins opens successfully.
</common_pitfalls>

<code_examples>
## Code Examples

### Per-coin key schema (locked D-01)

```rust
// Source: D-01; Knots txdb.cpp uses 'C' + hash + VARINT(vout).
// Open Bitcoin uses 4-byte LE vout — intentional, not LevelDB-identical.
pub const DB_COIN: u8 = b'C';
pub const DB_BEST_BLOCK: u8 = b'B';
pub const DB_HEAD_BLOCKS: u8 = b'H';
pub const DEFAULT_COINS_DB_BATCH_BYTES: usize = 64 << 20;

pub fn encode_coin_key(outpoint: &OutPoint) -> [u8; 37] {
    let mut key = [0_u8; 37];
    key[0] = DB_COIN;
    key[1..33].copy_from_slice(outpoint.txid.as_bytes());
    key[33..37].copy_from_slice(&outpoint.vout.to_le_bytes());
    key
}

pub fn encode_best_block_key() -> [u8; 1] {
    [DB_BEST_BLOCK]
}

pub fn encode_head_blocks_key() -> [u8; 1] {
    [DB_HEAD_BLOCKS]
}
```

### Compact coin value (discretion recommendation)

Use Knots-shaped `code` plus first-party TxOut and MTP. Fail closed on short reads, non-canonical compact-size, invalid amount/script, and trailing bytes.

```rust
// Source: Knots coins.h Coin serialize:
//   VARINT((coinbase ? 1 : 0) | (height << 1)) then TxOut
// Open Bitcoin adds created_median_time_past (D-11).
// TxOut: i64 LE sats + compact-size script (existing transaction.rs shape).
//
// Layout:
//   compact-size code = (created_height << 1) | (is_coinbase as u64)
//   i64 LE output.value.to_sats()
//   compact-size script length + script bytes
//   i64 LE created_median_time_past
//   Reader::finish() required

fn encode_coin_value(coin: &Coin) -> Result<Vec<u8>, StorageError> { /* ... */ }
fn decode_coin_value(bytes: &[u8]) -> Result<Coin, StorageError> { /* ... */ }
```

`created_height` is `u32`. If `(height as u64) << 1` would lose the coinbase bit, fail closed (`Corruption`) rather than truncate.

`head_blocks` value: compact-size count + `count * 32` hash bytes. Empty / missing key = count 0. Present value must decode to exactly two hashes or fail closed (D-10). `B` value is 32 raw bytes.

### First-party 64 MiB accounting

```rust
// Source: Knots txdb.h nDefaultDbBatchSize = 64 << 20
// Fjall WriteBatch::len() is item count — do not use it as size.
// [VERIFIED: fjall-3.1.4/src/batch/mod.rs]

fn estimated_encoded_bytes(key: &[u8], maybe_value: Option<&[u8]>) -> usize {
    key.len() + maybe_value.map_or(0, <[u8]>::len)
}
```

Count marker writes (`erase B`, `write H`, `erase H`, `write B`) in the same accumulator. When the next item would exceed the cap, `commit` the current batch with `Buffered` and start a new batch.

### Fail-closed get (CSOBS-03)

```rust
// Source: Fjall Keyspace::get returns Result<Option<UserValue>>
// [VERIFIED: fjall-3.1.4/src/keyspace/mod.rs]
// Knots CCoinsViewErrorCatcher must not return not-found on read error.

fn get_coin(&self, outpoint: &OutPoint) -> Result<Option<Coin>, ChainstateError> {
    let key = encode_coin_key(outpoint);
    let maybe_bytes = self
        .coins
        .get(key)
        .map_err(|error| map_backend(error))?; // never Ok(None)
    let Some(bytes) = maybe_bytes else {
        return Ok(None); // successful absence only
    };
    decode_coin_value(bytes.as_ref())
        .map(Some)
        .map_err(map_corruption) // never Ok(None)
}
```

### Undo key (discretion recommendation)

Match existing `block:` hex spelling in the `chainstate` keyspace:

```rust
// Source: fjall_store.rs block_key — "block:" + 64 hex chars
fn undo_key(block_hash: BlockHash) -> String {
    // "undo:" + lowercase hex of block_hash.as_bytes()
}
```

Value: reuse `snapshot_codec` `BlockUndoDto` JSON for this small record (not the leftover full snapshot). Do not store undo inside `"snapshot"` after migration.

### `CoinsView` trait change required by D-21

```rust
// Source: packages/open-bitcoin-chainstate/src/coins.rs today:
//   fn best_block(&self) -> Option<BlockHash>;  // infallible — cannot surface Fjall errors
// D-21 requires Result on get_coin / have_coin / best_block / head_blocks.

pub trait CoinsView {
    fn get_coin(&self, outpoint: &OutPoint) -> Result<Option<Coin>, ChainstateError>;
    fn have_coin(&self, outpoint: &OutPoint) -> Result<bool, ChainstateError>;
    fn best_block(&self) -> Result<Option<BlockHash>, ChainstateError>;
    fn head_blocks(&self) -> Result<Vec<BlockHash>, ChainstateError>;
    fn batch_write(
        &mut self,
        writes: CoinsBatch,
        maybe_best_block: Option<BlockHash>,
    ) -> Result<(), ChainstateError>;
}
```

Add `ChainstateError::CoinsStorage { detail: String }`. Never reuse `MissingCoin` for disk failure. `MemoryCoinsView::best_block` returns `Ok(self.maybe_best_block)`; `head_blocks` returns `Ok(Vec::new())`. `CoinsCache` uses overlay then `self.parent.best_block()?`. `FjallCoinsView::batch_write` requires `Some(new_tip)` (Knots `assert(!hashBlock.IsNull())`); `MemoryCoinsView` may still accept `None`.

### `cfg(test)` crash seam (D-22)

```rust
// Source: Knots CoinsViewOptions.simulate_crash_ratio + _Exit
// Open Bitcoin: no process abort.

#[cfg(test)]
fn maybe_simulate_crash(&self) -> Result<(), StorageError> {
    if self.simulate_crash_after_partial {
        return Err(StorageError::InterruptedWrite {
            namespace: StorageNamespace::Coins,
            action: StorageRecoveryAction::Reindex,
        });
    }
    Ok(())
}
```

Call after the first partial `Buffered` commit, before the final `H` erase / `B` write.
</code_examples>

<sota_updates>
## State of the Art (2024-2026)

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Pretty-JSON `chainstate/snapshot` as UTXO truth | Per-outpoint Fjall `coins` records + `B`/`H` | Phase 141 | Leftover blob is migration/export only after schema 2 |
| Milestone research: codec crate vs node `coins_codec.rs` | CONTEXT D-11 locks node `coins_codec.rs` | 2026-09-04 discuss | Do not add `encode_tx_out` to the codec crate this phase |
| Milestone research: `StorageNamespace::Chainstate` neighborhood | Dedicated `StorageNamespace::Coins` | CONTEXT D-04 | Do not stuff outpoints next to `"snapshot"` |
| Knots LevelDB `SizeEstimate` | First-party encoded-byte sum | Locked D-07 | Never use Fjall `len()` as bytes |
| Knots `ReplayBlocks` on two-element `H` | Fail closed `InterruptedWrite` | Locked D-09 | Phase 142 owns replay-versus-fail-closed |
| Knots `CCoinsViewErrorCatcher` abort | Typed `StorageError` / `CoinsStorage` | CSOBS-03 | Same invariant: error ≠ missing |

**Deprecated/outdated for this phase:**

- Live `encode_chainstate_snapshot` as coin truth
- Infallible `CoinsView::best_block`
- `ensure_schema` equality-only check once `CURRENT` becomes `2` (must gain a schema-1 migrate branch)
- LevelDB / rust-bitcoin as "parity shortcuts"

**Do not copy from Knots this phase:** `ReplayBlocks`, `ResizeCache`, `NeedsUpgrade` (`DB_COINS` / `'c'`), `EmplaceCoinInternalDANGER`, prune unlink, assumeutxo dual-chainstate.
</sota_updates>

## Discretion Recommendations

These are the CONTEXT freedom areas. Planner should treat them as the default design unless a task finds a compile blocker.

| Area | Recommendation | Why |
|------|----------------|-----|
| Coin value layout | Compact-size `code = (height << 1) \| coinbase`, then i64 LE value, compact-size script, i64 LE MTP, `Reader::finish()` | Knots-shaped height/coinbase; first-party TxOut (no `TxOutCompression`); MTP required; trailing bytes fail closed. |
| Undo key | `undo:` + 64 lowercase hex chars in `chainstate` | Matches existing `block:` helper in `fjall_store.rs`. |
| Undo value | Existing `BlockUndoDto` JSON via extracted encode/decode in `snapshot_codec.rs` | Small record, already tested; not the leftover full snapshot. |
| `FjallCoinsView` shape | Standalone type in `storage/coins_view.rs`; `FjallNodeStore::coins_view()` constructs it | Clear `CoinsView` impl; Fjall stays in the shell; store keeps keyspace ownership. |
| `ChainstateStore` growth | Add view-backed methods; keep `load_snapshot` / `save_snapshot` | Leftover persist write sites keep compiling (D-19). |
| Production `Chainstate` parent | Stay `CoinsCache<MemoryCoinsView>` this phase | Manager attach is Phase 142. Open hydrates Memory from a coins scan after migration. |
| `H` encoding | Compact-size count + 32-byte hashes | Detects count ≠ 2 (D-10) instead of a raw 64-byte blob that cannot represent other counts. |

## Research Flags (resolved)

| Flag | Resolution | Confidence |
|------|------------|------------|
| Exact per-coin key schema | `C` + 32-byte txid + 4-byte LE vout (37 bytes). `B` / `H` are single-byte keys. Not Knots VARINT(vout); not LevelDB-identical. | HIGH — D-01 + Knots `CoinEntry` contrast [VERIFIED: `txdb.cpp`] |
| Dedicated coins keyspace | `StorageNamespace::Coins` / `"coins"`. Update `as_str`, `open`, `keyspace()`, namespace test. | HIGH — D-04 + current enum [VERIFIED: `storage.rs`] |
| 64 MiB dirty-batch accounting | `DEFAULT_COINS_DB_BATCH_BYTES = 64 << 20`. Sum encoded key+value bytes. Do not use `WriteBatch::len()`. | HIGH — D-07 + Fjall source [VERIFIED: `batch/mod.rs`] |
| `head_blocks` + BatchWrite | Two-element `[new, old]`. Protocol D-08. Reopen: empty `H`+`B` consistent; two-element `H`+missing `B` → `InterruptedWrite`; other → fail closed. No `ReplayBlocks`. | HIGH — D-08–D-10 + `txdb.cpp` [VERIFIED] |
| Compact codec home | `storage/coins_codec.rs`. Include height + MTP. Use codec primitives; `Reader::finish()`. | HIGH — D-11 + codec `Reader` [VERIFIED] |
| Undo vs leftover DTO | `save_undo` / `load_undo` in `chainstate` as `undo:{hex}`. Migration copies `undo_by_block` out of leftover. | HIGH — D-13 + existing `block_key` pattern [VERIFIED] |
| Schema 1→2 | Rewrite `ensure_schema` to migrate version `1`. Fail closed on other versions and on schema 2 + empty coins + leftover snapshot. One store-level version. | HIGH — D-16–D-18 + current `validate_schema_version` [VERIFIED] |
| Fail-closed disk reads | `get`/`contains_key` `Err` and decode → typed error. `Ok(None)` only after successful miss. Fallible `CoinsView` heads/best-block. | HIGH — D-20/D-21 + Knots error catcher + Fjall `get` [VERIFIED] |

<open_questions>
## Open Questions

1. **Post-migration leftover writes vs coins-on-reopen**
   - What we know: D-19 keeps `persist` / `persist_progress` writing leftover snapshots. D-15/D-18 forbid leftover UTXO hydrate after coins exist.
   - What's unclear: none as a product decision — the window is locked. In-process Memory stays current; reopen follows coins (migration-time or explicit `batch_write` only) until Phase 142 wires flush writes.
   - Recommendation: Document in plan success checks. Do not dual-read leftover `utxos` to "fix" the window.

2. **`active_chain` / confirmation counts on reopen**
   - What we know: leftover snapshot still holds `active_chain` and `maybe_confirmed_txid_counts`. Coins keyspace does not.
   - What's unclear: whether Phase 141 should persist a small `chain_meta` record during migration or read leftover metadata only (never `utxos`).
   - Recommendation: Prefer a small `chainstate` `chain_meta` record written during migration (D-05). If that slips, leftover-metadata-only read is acceptable; leftover `utxos` are not.

3. **`Reader` / `write_i64_le` public path**
   - What we know: `open_bitcoin_codec::primitives` is a public module; node already depends on `open-bitcoin-codec`. `encode_outputs` is private in `transaction.rs`.
   - What's unclear: none — call public primitives from `coins_codec.rs`.
   - Recommendation: Do not add a new codec-crate `encode_tx_out` this phase.
</open_questions>

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| Rustc / Cargo | Build + unit tests | ✓ | `1.94.1` via `rust-toolchain.toml` | — |
| Fjall | Coins keyspace | ✓ (workspace pin) | `3.1.4` | — |
| Bun | `command-timings.ts` / verify | ✓ (repo pin `.bun-version`) | repo pin | — |
| Bitcoin Knots submodule | Parity breadcrumbs / cite | ✓ (files read this session) | `29.3.knots20260210` | `git submodule update --init --recursive` |
| LevelDB / rust-bitcoin | — | must stay absent | — | Forbidden |

**Missing dependencies with no fallback:** none.

**Missing dependencies with fallback:** none.

**Step 2.6:** External tools are the existing Rust/Fjall/Bun toolchain. No new services.

## Security Domain

`security_enforcement` is not set to `false` in `.planning/config.json`. Include this section.

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V2 Authentication | no | Not a user-auth phase |
| V3 Session Management | no | Not a session phase |
| V4 Access Control | no | Single-process datadir; existing Fjall lock stays |
| V5 Input Validation | yes | `coins_codec` + `Reader::finish`; schema version parse; fail-closed decode |
| V6 Cryptography | no | Do not hand-roll hashes/scripts; reuse first-party `Txid` / `ScriptBuf` |

### Known Threat Patterns for Fjall coins

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Disk-read error treated as spent | Tampering / Elevation | CSOBS-03: typed `StorageError` / `CoinsStorage`, never `Ok(None)` |
| Leftover snapshot after cutover | Tampering | Schema 2 ignores leftover `utxos`; schema 2 + empty coins + leftover fails closed |
| Mid-flush mixed old/new coins | Tampering | `H`/`B` two-phase protocol; two-element `H` fail-closed this phase |
| Schema downgrade / unknown version | Tampering | `schema_mismatch`; no silent convert |
| Auto-repair / remigrate | Tampering | D-10 / D-18; diagnostic recovery only |
| LevelDB or rust-bitcoin import | Spoofing of consensus types | Dependency policy; do not add crates |
| High-cardinality coin dumps in tests/metrics | Information disclosure | Test fixtures only; no support-bundle coin dumps this phase |

<sources>
## Sources

### Primary (HIGH confidence)

- `packages/bitcoin-knots/src/txdb.h` — `nDefaultDbBatchSize = 64 << 20`, `GetHeadBlocks`, `CCoinsViewDB`
- `packages/bitcoin-knots/src/txdb.cpp` — `DB_COIN`/`DB_BEST_BLOCK`/`DB_HEAD_BLOCKS`, `CoinEntry` VARINT vout, two-phase `BatchWrite`, crash `_Exit` hook
- `packages/bitcoin-knots/src/coins.h` — `Coin` VARINT height/coinbase serialize; `CCoinsViewErrorCatcher`
- `packages/bitcoin-knots/src/coins.cpp` — `ExecuteBackedWrapper` abort-on-read-error (error ≠ not-found)
- `packages/bitcoin-knots/src/validation.cpp` — `ReplayBlocks` cite only (Phase 142)
- `packages/open-bitcoin-node/src/storage.rs` — namespaces, `SchemaVersion::CURRENT = 1`, `StorageError`
- `packages/open-bitcoin-node/src/storage/fjall_store.rs` — keyspaces, `SNAPSHOT_KEY`, `ensure_schema`, `PersistMode` map, `&str` put/get
- `packages/open-bitcoin-node/src/storage/snapshot_codec.rs` — leftover `CoinDto` / `undo_by_block`
- `packages/open-bitcoin-node/src/chainstate.rs` — `ChainstateStore` `{load,save}_snapshot`, `persist` leftover write
- `packages/open-bitcoin-node/src/sync.rs` / `sync/runtime_state.rs` — leftover hydrate / `persist_progress`
- `packages/open-bitcoin-chainstate/src/coins.rs` — `CoinsView` (infallible `best_block`)
- `packages/open-bitcoin-chainstate/src/types.rs` — `Coin` height + MTP
- `packages/open-bitcoin-chainstate/src/error.rs` — `MissingCoin` must not be the disk-error path
- Fjall `3.1.4` `src/batch/mod.rs` — `len()` item count; `insert` / `remove` / `commit` / `durability`
- Fjall `3.1.4` `src/keyspace/mod.rs` — `get` / `contains_key` / `prefix` return `Result`
- Fjall `3.1.4` `src/db.rs` — `persist` poisons on flush failure
- `.planning/phases/141-durable-fjall-coins-adapter/141-CONTEXT.md` — D-01–D-22
- `standards/core/architecture.md`, `standards/languages/rust.md`, `standards/core/testing.md`
- `scripts/pure-core-crates.txt`, `scripts/check-pure-core-deps.sh`

### Secondary (MEDIUM confidence)

- `.planning/research/{STACK,ARCHITECTURE,FEATURES,PITFALLS,SUMMARY}.md` — milestone guidance; Phase 141 CONTEXT overrides codec-home and keyspace where they disagreed
- Wallet rescan still calling `load_chainstate_snapshot` — leftover consumer, deferred

### Tertiary (LOW confidence)

- None for this phase's locked flags
</sources>

<metadata>
## Metadata

**Research scope:**
- Core technology: Fjall `3.1.4` coins keyspace + Knots `CCoinsViewDB` BatchWrite
- Ecosystem: existing node store, chainstate `CoinsView`, first-party codec primitives
- Patterns: parse-at-boundary, two-phase `H`/`B`, one-way schema 1→2
- Pitfalls: dual truth, `len()` as size, infallible `best_block`, schema bump without migrate branch

**Confidence breakdown:**
- Standard stack: HIGH — repo pins and Fjall 3.1.4 source
- Architecture: HIGH — CONTEXT locks plus live `FjallNodeStore` / `CoinsView` seams
- Pitfalls: HIGH — verified against current `ensure_schema`, `open`, and Knots `txdb.cpp`
- Code examples: HIGH — derived from locked keys + verified APIs

**Research date:** 2026-09-04
**Valid until:** 2026-10-04 (30 days — Fjall pin and Knots baseline are stable)
</metadata>

---

*Phase: 141-durable-fjall-coins-adapter*
*Research completed: 2026-09-04*
*Ready for planning: yes*

## RESEARCH COMPLETE

**Phase:** 141 - Durable Fjall Coins Adapter
**Confidence:** HIGH

### Key Findings

- Use dedicated `StorageNamespace::Coins` with keys `C`+txid+LE vout, `B`, and `H`. Bound batches by first-party encoded bytes (`64 << 20`), never Fjall `len()`.
- Implement Knots two-phase `BatchWrite` in `FjallCoinsView`. Two-element `H` + missing `B` fails closed this phase — no `ReplayBlocks`.
- Bumping `SchemaVersion::CURRENT` to `2` requires a schema-1 migrate branch; today's equality check would reject every existing datadir.
- Make `CoinsView::best_block` / `head_blocks` fallible. Disk I/O and decode errors become `StorageError` / `ChainstateError::CoinsStorage`, never `Ok(None)` or `MissingCoin`.
- Keep leftover snapshot *writes*. After coins exist, reopen must not hydrate leftover `utxos`. That write/read split is the locked Phase 142 seam.

### File Created

`.planning/phases/141-durable-fjall-coins-adapter/141-RESEARCH.md`

### Confidence Assessment

| Area | Level | Reason |
|------|-------|--------|
| Standard Stack | HIGH | Existing Fjall `3.1.4` pin and first-party codec primitives |
| Architecture | HIGH | CONTEXT D-01–D-22 plus live store / view seams |
| Pitfalls | HIGH | Verified `ensure_schema`, leftover `open`, and Knots `BatchWrite` |

### Open Questions

- Accepted leftover-write vs coins-on-reopen window until Phase 142.
- Prefer small `chain_meta` for `active_chain` on migrate; leftover metadata-only is the fallback.

### Ready for Planning

Research complete. Planner can now create PLAN.md files. Do not commit; the orchestrator commits docs.
