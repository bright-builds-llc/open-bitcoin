---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 141-2026-09-04T17-37-21
generated_at: 2026-09-04T17:37:29.235Z
---

# Phase 141: Durable Fjall Coins Adapter - Context

**Gathered:** 2026-09-04
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Spendable UTXOs live as per-outpoint Fjall records with best-block and
interrupted-flush markers; snapshot blobs are no longer live coin truth.

This phase delivers COIN-01 and CSOBS-03 only: a dedicated `coins`
keyspace, compact per-outpoint codec (including MTP), `best_block` and
two-element `head_blocks`, 64 MiB dirty-batch writes, explicit one-way
migration from the leftover `"snapshot"` blob, schema bump with
fail-closed mismatch, and typed fail-closed coins disk-read errors.
`save_block` / `load_block` stay unchanged. `open-bitcoin-chainstate`
stays I/O-free.

This phase does not own manager init / CanFlush readiness, IfNeeded /
Periodic / Always call-site wiring, persist_progress cutover off leftover
writes, interrupted-flush replay, honest payload-present availability,
operator flush evidence, or prune / archive / assumeutxo / LevelDB /
rust-bitcoin / production-readiness claims. Phase 142 executes recovery
and manager lifecycle against the markers this phase persists.

</domain>

<decisions>
## Implementation Decisions

### Per-coin key schema

- **D-01:** Use Knots-shaped first-party prefixes inside a dedicated coins
  keyspace: `C` + 32-byte txid + 4-byte little-endian vout for a coin,
  `B` for coins best-block, and `H` for `head_blocks`. Values are compact
  binary, not pretty JSON. Do not require LevelDB `chainstate/`
  byte-identical keys.
- **D-02:** Erase is a missing key, not a stored spent tombstone on disk.
  Spent DIRTY entries that survive cache Flush become deletes in the
  parent batch. A successful `get_coin` miss is `Ok(None)`.
- **D-03:** Do not introduce LevelDB, rusty-leveldb, RocksDB, or
  rust-bitcoin. Reuse Fjall `3.1.4` `batch` / `persist` / `contains_key`
  and existing `PersistMode::{Buffered,Flush,Sync}`.

### Dedicated coins keyspace

- **D-04:** Add `StorageNamespace::Coins` (`"coins"`). Do not stuff
  millions of outpoints into the existing `chainstate` `"snapshot"`
  neighborhood.
- **D-05:** Keep small tip / undo / active-chain metadata in the existing
  `chainstate` keyspace as small records, not as a cloned UTXO map.
- **D-06:** `FjallCoinsView` lives in `open-bitcoin-node` storage and
  implements the Phase 139 `CoinsView` trait. `CoinsCache` must stay
  ignorant of whether the parent is `MemoryCoinsView` or
  `FjallCoinsView`. Do not put Fjall, `std::fs`, Tokio, or a clock in
  `open-bitcoin-chainstate`.

### Dirty-batch accounting and head_blocks

- **D-07:** Bound dirty coin writes at `64 << 20` bytes of first-party
  estimated encoded payload (`nDefaultDbBatchSize`). Fjall batch `len()`
  is item count, not LevelDB `SizeEstimate`; do not treat count as size.
- **D-08:** Encode `head_blocks` as a two-element `[new, old]`
  `BlockHash` vector. Implement Knots `CCoinsViewDB::BatchWrite` in the
  adapter:
  1. Erase `B`, write `H = [new, old]`.
  2. Write dirty coins in 64 MiB-capped batches (`Buffered` for in-flush
     partials).
  3. Erase `H`, write `B = new` (`Flush` for journal-to-OS; `Sync` for
     shutdown / schema / recovery-marker writes).
- **D-09:** On reopen, empty `H` plus present `B` is consistent. A
  two-element `H` with missing `B` is an interrupted-flush marker and
  must fail closed as a typed storage or recovery error in this phase.
  Do not implement `ReplayBlocks` here. Phase 142 chooses
  replay-versus-fail-closed execution when bodies or undo are present.
- **D-10:** Any other `H` count, mismatched `H[0]`, or decode failure is
  inconsistent and fails closed. Do not auto-repair. Fjall persist
  changes durability, not consistency; `head_blocks` is the application
  consistency marker.

### Compact codec home

- **D-11:** Place the compact per-outpoint codec at
  `packages/open-bitcoin-node/src/storage/coins_codec.rs`. Parse bytes
  into domain `Coin` at the Fjall boundary. Include
  `created_height` and `created_median_time_past`.
- **D-12:** Do not reuse `encode_chainstate_snapshot` pretty JSON as coin
  truth. Snapshot JSON remains a leftover migration source only.

### Undo records vs leftover snapshot DTO

- **D-13:** Persist undo as its own durable records (`save_undo` /
  `load_undo`) in the `chainstate` keyspace. A disconnect must not
  require reloading a full UTXO blob. Do not keep undo live inside the
  leftover snapshot DTO after migration.
- **D-14:** `save_block` / `load_block` stay the payload home and stay
  unchanged. Phase 142 flushes block / undo / index before coins; this
  phase only makes undo independently readable.
- **D-15:** `ChainstateSnapshot` remains a hydrate / export / test helper.
  After the one-way migration, reopen must not treat the leftover
  `"snapshot"` blob as UTXO truth.

### Schema bump and one-way migration

- **D-16:** Bump `SchemaVersion::CURRENT` from `1` to `2`. This is a
  breaking store layout. Fail closed on mismatch through the existing
  `StorageError::schema_mismatch` path.
- **D-17:** Do not invent per-namespace schema versions. Wallet,
  mempool, and runtime snapshot codecs stay readable under schema 2;
  only coin-truth layout changes. One store-level version covers the
  cutover.
- **D-18:** Explicit one-way migration, not dual-read and not silent
  convert:
  - Fresh datadir writes schema 2 with an empty coins keyspace and no
    snapshot blob.
  - Schema 1 plus leftover `"snapshot"` and empty coins: migrate once
    into per-outpoint coins plus undo records, then write schema 2.
  - Schema 2: coins keyspace is UTXO truth. A leftover `"snapshot"`
    blob is non-authoritative and must not hydrate the live view.
  - Schema 2 plus empty coins plus leftover snapshot: fail closed. Do
    not remigrate.
  - Any other version: `schema_mismatch`.
- **D-19:** Do not cut over `ManagedChainstate::persist` or
  `DurableSyncRuntime::persist_progress` off leftover snapshot *writes*.
  Phase 142 owns that write-site cutover and manager flush wiring. This
  phase makes reopen / load treat coins as truth after migration.

### Fail-closed disk reads

- **D-20:** A coins disk-read I/O error, corruption, or decode failure
  is a typed `StorageError` / recovery error. Never map it to spent,
  missing, or `Ok(None)`. `Ok(None)` is only a successful absence after
  a completed lookup.
- **D-21:** Do not swallow Fjall errors inside `FjallCoinsView::get_coin`
  / `have_coin` / `best_block` / `head_blocks`. Surface them through the
  `CoinsView` `Result` so a later manager cannot treat a read failure as
  an empty UTXO set.
- **D-22:** `MemoryChainstateStore` stays the in-memory test parent and
  must implement the same view-backed store surface (`get_coin`,
  `batch_write`, `best_block`, `head_blocks`, `load_undo`, `save_undo`).
  Tests may use a `cfg(test)` simulate-crash seam without process
  `_Exit`.

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

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase and requirement
- `.planning/ROADMAP.md` — Phase 141 goal, COIN-01, CSOBS-03, success
  criteria, and the research flags for key schema, coins keyspace,
  64 MiB batches, `head_blocks`, codec home, undo location, and schema
  bump
- `.planning/REQUIREMENTS.md` — COIN-01 and CSOBS-03 wording; FLUSH-02 /
  MGR-01 / MGR-02 stay Phase 142
- `.planning/PROJECT.md` — Functional-core / no rust-bitcoin /
  Fjall-in-shell constraints and v2.3 storage-first scope
- `.planning/phases/139-coins-view-cache-contract-and-engine-apply/139-CONTEXT.md`
  — Locked `CoinsView` / `CoinsCache` / leftover snapshot persist
- `.planning/phases/140-pure-flush-policy-and-typed-decisions/140-CONTEXT.md`
  — Locked Flush vs Sync algebra and the explicit Phase 141 deferral of
  Fjall coins / `head_blocks`

### Milestone research
- `.planning/research/SUMMARY.md` — Durable Fjall coins adapter
  deliverable (roadmap Phase 141; older research numbering said 142+):
  coins keyspace, compact codec including MTP, `best_block`, two-element
  `head_blocks`, 64 MiB batches, schema bump, one-way migration
- `.planning/research/STACK.md` — Dedicated `coins` keyspace, first-party
  64 MiB accounting, `head_blocks` write protocol, schema 1→2, undo as
  own record
- `.planning/research/ARCHITECTURE.md` — `FjallCoinsView` in node
  `storage/`, `coins_codec.rs`, Anti-Patterns (Fjall in core; dual truth)
- `.planning/research/FEATURES.md` — Knots `CCoinsViewDB` `DB_COIN` /
  `DB_BEST_BLOCK` / `DB_HEAD_BLOCKS` and two-phase `BatchWrite`
- `.planning/research/PITFALLS.md` — Dual truth, cache-hit-as-durable,
  incremental writes without heads, no auto-repair

### Existing apply surface
- `packages/open-bitcoin-chainstate/src/coins.rs` — `CoinsView`,
  `CoinsBatch`; parent must stay I/O-free
- `packages/open-bitcoin-chainstate/src/coins/cache.rs` — `flush()` /
  `sync()` algebra the adapter must honor as a parent
- `packages/open-bitcoin-chainstate/src/types.rs` — `Coin` already
  carries `created_height` and `created_median_time_past`
- `packages/open-bitcoin-node/src/storage.rs` — `StorageNamespace`,
  `SchemaVersion::CURRENT = 1`, `PersistMode`, `StorageError`
- `packages/open-bitcoin-node/src/storage/fjall_store.rs` —
  `SNAPSHOT_KEY = "snapshot"`, existing keyspaces, schema mismatch
- `packages/open-bitcoin-node/src/storage/snapshot_codec.rs` — Leftover
  pretty-JSON snapshot encode/decode; migration source only
- `packages/open-bitcoin-node/src/chainstate.rs` —
  `ManagedChainstate::persist` still `save_snapshot`; do not retarget
  writes in this phase
- `packages/open-bitcoin-node/src/sync/runtime_state.rs` —
  `persist_progress` still writes the leftover snapshot; do not cut over
  writes in this phase

### Knots anchors
- `packages/bitcoin-knots/src/txdb.h` — `CCoinsViewDB`,
  `nDefaultDbBatchSize = 64 << 20`, `GetHeadBlocks`
- `packages/bitcoin-knots/src/txdb.cpp` — `DB_COIN` / `DB_BEST_BLOCK` /
  `DB_HEAD_BLOCKS`, two-phase `BatchWrite`
- `packages/bitcoin-knots/src/coins.h` / `packages/bitcoin-knots/src/coins.cpp`
  — Cache Flush/Sync already ported; disk parent is this phase
- `packages/bitcoin-knots/src/txdb.cpp` `ReplayBlocks` — cite only;
  executable recovery stays Phase 142

### Architecture standards
- `standards/core/architecture.md` — Functional core / imperative shell;
  parse at boundaries
- `standards/languages/rust.md` — `foo.rs` plus `foo/` module layout
- `standards/core/testing.md` — Unit-test pure codec and fail-closed
  mapping
- `.planning/ARCHITECTURE.md` — Crate boundaries; Fjall stays in node
- `docs/parity/catalog/chainstate.md` — Known disk-backed gap (this
  phase closes coins persistence, not manager flush or serving honesty)

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `CoinsView` / `CoinsCache` / `MemoryCoinsView` — Disk parent plugs in
  behind the same trait; cache algebra stays unchanged
- `FjallNodeStore` — Add a coins keyspace beside existing namespaces;
  reuse `PersistMode` and schema-mismatch
- `Coin` — Already has height and MTP; codec must round-trip both
- `BlockUndo` / `TxUndo` — Move to own records; keep domain types
- `StorageError::schema_mismatch` / `StorageRecoveryAction` — Fail closed
  without automatic destructive repair
- `ChainstateSnapshot` — Leftover migration source and test helper

### Established Patterns
- Functional-core view + shell Fjall adapter (mempool snapshot already
  follows this split)
- Parse-at-boundary codecs in `open-bitcoin-node/src/storage/`
- Parity breadcrumbs required on new first-party Rust sources
- Architecture-policy checks fail if Fjall or clocks enter
  `open-bitcoin-chainstate`
- Schema mismatch already fails closed at store open

### Integration Points
- New `StorageNamespace::Coins` and `coins_codec.rs`
- `FjallCoinsView` implements `CoinsView` for production parent reads
- `ChainstateStore` grows view-backed methods; leftover
  `load_snapshot` / `save_snapshot` remain for Phase 142 write-site
  cutover
- `DurableSyncRuntime::open` may still compile against leftover persist
  writes, but must not treat a leftover blob as UTXO truth after
  coins exist
- Engine unit tests keep `MemoryCoinsView`; adapter tests use temporary
  Fjall stores

</code_context>

<specifics>
## Specific Ideas

- Match pinned Knots `CCoinsViewDB` prefixes and two-phase `BatchWrite`
  rather than inventing a single atomic Fjall batch as the only
  consistency story
- Treat leftover `"snapshot"` as a one-shot migration source, then
  non-authoritative leftover — never as a dual-read generation
- CSOBS-03 is a storage-boundary mapping: disk-read error ≠ spent

</specifics>

<deferred>
## Deferred Ideas

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

</deferred>

---

*Phase: 141-durable-fjall-coins-adapter*
*Context gathered: 2026-09-04*
