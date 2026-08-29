# Architecture Research

**Domain:** Open Bitcoin chainstate durability integration
**Researched:** 2026-08-29
**Confidence:** HIGH for current Open Bitcoin core/shell seams and pinned Knots coins/manager APIs; MEDIUM for the exact Fjall per-coin key schema and crash-atomic batch contract until a later phase researches those details

## Recommendation

Keep coins policy, cache flags, and flush *decisions* in `open-bitcoin-chainstate`. Keep Fjall, filesystem, clocks, and persist execution in `open-bitcoin-node`. Do not put Fjall or filesystem calls in the chainstate crate.

Replace the current “clone the whole UTXO map, then write one `snapshot` blob” path with a Knots-shaped layered view: a typed coins-view contract, an in-memory dirty/fresh cache, a Fjall-backed disk view, and a manager that chooses flush points and rebuilds the cache on restart. Block-serving availability must become a payload-present fact, not a hardcoded `durable_availability: true`.

v2.3 is a single active chainstate. Do not port assumeutxo dual-chainstate, prune product modes, or LevelDB.

## Standard Architecture

### System Overview

```
┌──────────────────────────────────────────────────────────────────────────┐
│ Effectful entry points                                                    │
│ open-bitcoind | DurableSyncRuntime | inbound getdata | RPC/CLI/status     │
└───────────────────────────────┬──────────────────────────────────────────┘
                                │ typed commands + injected time/size/mode
┌───────────────────────────────▼──────────────────────────────────────────┐
│ Node shell: ManagedChainstate + DurableSyncRuntime                       │
│ ┌────────────────────────┐  ┌─────────────────────────────────────────┐  │
│ │ Flush orchestration    │  │ Honest availability                     │  │
│ │ flush points           │  │ payload-present? → Available            │  │
│ │ restart cache init     │  │ missing payload  → Unavailable/NotFound │  │
│ │ persist execution      │  │ never claim Available without bytes     │  │
│ └───────────┬────────────┘  └──────────────────┬──────────────────────┘  │
└─────────────┼──────────────────────────────────┼─────────────────────────┘
              │ decisions (pure)                 │ load_block / has_payload
┌─────────────▼──────────────────────────────────┼─────────────────────────┐
│ Pure core: open-bitcoin-chainstate             │                         │
│ CoinsView contract | CoinsCache | FlushPolicy  │                         │
│ connect / disconnect / reorg against the view  │                         │
│ (no Fjall, no fs, no clock)                    │                         │
└─────────────┬──────────────────────────────────┼─────────────────────────┘
              │ BatchWrite / GetCoin             │
┌─────────────▼──────────────────────────────────▼─────────────────────────┐
│ Fjall (existing store, new coins keys)                                    │
│ chainstate: per-outpoint coins + best-block + head-blocks                │
│ block_index: per-hash block payloads (already stored)                    │
│ headers / runtime / schema: unchanged roles                              │
└──────────────────────────────────────────────────────────────────────────┘
```

### Component Responsibilities

| Component | Change | Responsibility | Typical implementation |
|-----------|--------|----------------|------------------------|
| `CoinsView` contract | New | Get/Have coin, best-block, batch write, optional cursor | Pure trait in `open-bitcoin-chainstate` |
| `CoinsCache` | New | Dirty/fresh overlay, `Flush` vs `Sync`, cache-size facts | Pure cache over a parent view |
| `FlushPolicy` | New | Decide none / if-needed / periodic / always, empty-cache vs retain | Pure function of injected time, size, mode |
| `Chainstate` engine | Modify | Apply connect/disconnect/reorg to a view instead of cloning `HashMap` | Keep consensus/undo rules; stop owning the live UTXO map |
| `MemoryCoinsView` | New (replaces snapshot store role) | In-memory parent for tests and prepare overlays | Hash map + best-block; no I/O |
| `FjallCoinsView` | New | Disk-backed parent: per-outpoint coins, best-block, head-blocks | `open-bitcoin-node` adapter only |
| `ManagedChainstate` | Modify | Own cache lifecycle, flush points, restart init | Shell orchestration; execute policy, do not invent policy |
| `ChainstateStore` | Modify | Evolve from full-snapshot load/save to view attach + flush | Keep test memory impl; stop treating snapshot as durable truth |
| `FjallNodeStore` | Modify | Add coins keyspace ops; keep `save_block` / `load_block` | Do not write a complete UTXO blob after every connect |
| `DurableSyncRuntime` | Modify | Flush coins + headers on progress; hydrate cache, not full map | Attach Fjall view; do not reload entire UTXO into RAM |
| `ManagedPeerNetwork` | Modify | Consume manager + payload-present facts | Keep `MemoryCoinsView` in hermetic tests |
| Block serving / inbound wire | Modify | Label Available only when payload exists | Gate before serve; `NotFound` when lookup misses |
| Wallet rescan | Modify | Read coins through a view or cursor, not a full snapshot blob | Keep chunking; stop requiring `load_chainstate_snapshot()` |
| RPC / CLI / status / parity | Modify | Project flush, recovery, and honest availability | One authoritative snapshot; no second coins truth |

## New vs Modified

### New (add these)

| Piece | Lives in | Why new |
|-------|----------|---------|
| `CoinsView` / `CoinsCacheEntry` / batch-write cursor types | `open-bitcoin-chainstate` | Current engine has no view layer; Knots `CCoinsView` / `CCoinsViewCache` is the missing contract |
| Flush policy types (`FlushMode`, cache-size state, flush vs sync) | `open-bitcoin-chainstate` | Policy must stay pure and unit-testable |
| `MemoryCoinsView` | `open-bitcoin-chainstate` or node test adapter | Tests and prepare overlays need a parent that is not Fjall |
| `FjallCoinsView` + coins codec | `open-bitcoin-node` `storage/` | Disk parent; Fjall stays in the shell |
| Cache-lifecycle commands on the manager | `open-bitcoin-node` `chainstate.rs` | Restart-safe init, flush points, shutdown flush |
| Payload-present availability fact | node/network + RPC inbound | Today durable serving hardcodes `durable_availability: true` |

### Modified (do not fork)

| Piece | What changes | What stays |
|-------|--------------|------------|
| `Coin`, `TxUndo`, `BlockUndo`, `ChainPosition` | Serialization/metadata only as needed for disk coins | Domain meaning stays |
| `Chainstate` connect/disconnect/reorg | Apply against a cache/view | Contextual validation, undo order, BIP30 overwrite reject |
| `ChainstateSnapshot` | Test/migration/export helper, not durable truth | Keep for fixtures and one-time snapshot→coins migration |
| `ManagedChainstate` persist-after-every-mutation | Become policy-driven flush | prepare/commit remains; implement with a child cache, not a full clone |
| `MemoryChainstateStore` | Become or wrap `MemoryCoinsView` | Hermetic tests keep an in-memory parent |
| `FjallNodeStore::save_chainstate_snapshot` | Retire as the live write path | Keep decode for migration from existing datadirs |
| `DurableSyncRuntime::persist_progress` | Flush coins cache + headers, not a UTXO blob | Still persist headers/runtime metadata |
| `DurableSyncRuntime::open` | Attach disk view + init cache after DB health check | Still load headers; still seed `ManagedPeerNetwork` |
| `managed_block_serve_input` | Availability from payload-present, not a bool default | Existing status/eligibility classifiers stay |
| `docs/parity/catalog/chainstate.md` | Close the known disk-backed / manager gap | Keep existing connect/disconnect/reorg claims |

### Do not add

| Anti-feature | Why |
|--------------|-----|
| Fjall, `std::fs`, Tokio, or wall clock in `open-bitcoin-chainstate` | Bright Builds functional-core rule; quality gate |
| LevelDB / `CCoinsViewDB` port | Storage decision is Fjall |
| Assumeutxo / second `Chainstate` / snapshot chain | Out of scope for v2.3 |
| Prune or archive product modes | Later milestone; keep the `Pruned` *label* only as “active but payload absent” if still needed |
| A second coins authority beside `ManagedChainstate` | Same failure mode v2.2 already forbade for mempool |

## Recommended Project Structure

```
packages/open-bitcoin-chainstate/src/
├── engine.rs                 # Modify: apply connect/disconnect/reorg to a view
├── types.rs                  # Keep Coin / undo / position; snapshot becomes helper
├── coins.rs                  # New: CoinsView, cache entry flags, cursor types
├── coins/
│   ├── cache.rs              # New: dirty/fresh cache, Flush vs Sync
│   ├── memory.rs             # New: MemoryCoinsView
│   └── flush.rs              # New: FlushMode, cache-size state, decide_flush
└── error.rs                  # Modify: view/cache/flush typed errors

packages/open-bitcoin-node/src/
├── chainstate.rs             # Modify: manager orchestration, cache lifecycle
├── storage.rs                # Modify: namespace/schema notes for coins keys
├── storage/
│   ├── fjall_store.rs        # Modify: coins get/batch/best-block; keep save_block
│   ├── coins_codec.rs        # New: per-outpoint encode/decode
│   └── snapshot_codec.rs     # Modify: migration read of legacy snapshot blob
├── network/
│   ├── inventory.rs          # Modify: payload-present availability
│   └── block_serving.rs      # Keep classifiers; feed honest facts
└── sync/
    ├── runtime_state.rs      # Modify: persist_progress flushes coins
    └── wallet_rescan.rs      # Modify: read through view/cursor

packages/open-bitcoin-rpc/src/context/inbound_wire.rs
                              # Modify: refuse when load_block is None
docs/parity/catalog/chainstate.md
                              # Modify: disk coins + manager + availability
```

### Structure Rationale

- **`open-bitcoin-chainstate/coins*`:** Mirrors Knots `coins.h` / `coins.cpp` without importing their I/O. Policy and cache flags live next to the UTXO engine that already cites those breadcrumbs.
- **`open-bitcoin-node/storage`:** Already owns Fjall keyspaces, `PersistMode`, schema, and recovery markers. Coins keys belong here, not in a new crate.
- **`ManagedChainstate` stays in the node crate:** It is already the imperative shell around the pure engine. Growing it into a flush manager is a modification, not a new service.
- **RPC inbound stays a lookup adapter:** `DurableBlockSource::load_block` already exists. Honesty is feeding it a true presence fact *before* labeling Available, then still refusing on `None`.

## Architectural Patterns

### Pattern 1: Layered coins view (Knots `CoinsViews`, Fjall instead of LevelDB)

**What:** A parent view is the durable coin set. A child cache answers hits in memory and records dirty/fresh mutations. `Flush` writes dirty coins and empties the cache. `Sync` writes dirty coins and keeps unspent entries cached.

**When to use:** Every production connect/disconnect/reorg, and every test that claims restart safety.

**Trade-offs:** Extra types and a batch-write protocol. Avoids cloning the whole UTXO set per block and rewriting one giant snapshot blob. Matches pinned Knots `CCoinsView` / `CCoinsViewCache` / `CoinsViews` in `validation.h`.

**Example:**

```rust
pub trait CoinsView {
    fn get_coin(&self, outpoint: &OutPoint) -> Result<Option<Coin>, ChainstateError>;
    fn have_coin(&self, outpoint: &OutPoint) -> Result<bool, ChainstateError>;
    fn best_block(&self) -> Option<BlockHash>;
    fn batch_write(
        &mut self,
        writes: CoinsBatch,
        best_block: BlockHash,
    ) -> Result<(), ChainstateError>;
}

pub struct CoinsCache<V> {
    parent: V,
    entries: HashMap<OutPoint, CoinsCacheEntry>,
    best_block: Option<BlockHash>,
}
```

The parent in production is `FjallCoinsView`. The parent in tests is `MemoryCoinsView`. `prepare_connect` uses a child cache over the live cache, then commits by `Flush` into the parent cache — not by cloning `Chainstate`.

### Pattern 2: Injected-fact flush policy

**What:** A pure function returns `FlushDecision { write: Flush \| Sync \| None, empty_cache: bool }` from `FlushMode`, cache-size state, last-flush age, and prune-for-later=false.

**When to use:** After connect/reorg, on periodic sync ticks, on clean shutdown, and before restart-sensitive availability claims.

**Trade-offs:** Callers must inject time and size. That is required: the core must not read a clock. Knots `FlushStateToDisk` uses `FlushStateMode::{NONE, IF_NEEDED, PERIODIC, ALWAYS}` and empties the cache only when the mode is ALWAYS or the cache is large/critical (or prune — out of scope). Prefer `Sync` for periodic writes so the working set stays warm.

**Example:**

```rust
pub enum FlushMode {
    None,
    IfNeeded,
    Periodic,
    Always,
}

pub fn decide_flush(input: FlushPolicyInput) -> FlushDecision {
    // empty_cache only for Always or CacheSizeState::{Large, Critical}
}
```

The shell maps `PersistMode::{Buffered, Flush, Sync}` onto Fjall durability *after* the policy decides *whether* to write.

### Pattern 3: Restart-safe cache lifecycle

**What:** Knots initializes the disk view first, verifies it, then `InitCoinsCache`, then `LoadChainTip` from the coins view’s best block. Cache is created only after the database is healthy. Shutdown `ForceFlush` then `ResetCoinsViews`.

**When to use:** `DurableSyncRuntime::open`, clean shutdown, and crash recovery.

**Trade-offs:** Startup is more steps than today’s “decode one snapshot into a HashMap.” It is the only way a crash during cache residency does not invent a fake tip.

Open Bitcoin mapping:

1. Open Fjall and verify schema / recovery markers.
2. Attach `FjallCoinsView` (do not create the cache yet).
3. If coins best-block is null, start empty; else treat that hash as tip identity.
4. Init `CoinsCache` with a configured byte budget.
5. Rebuild active-chain metadata from headers/block-index plus coins best-block. Do not load every coin into RAM.
6. On progress: policy → `Flush`/`Sync` → Fjall persist.
7. On clean shutdown: `Always` flush, then drop the cache.

### Pattern 4: Parse at the storage boundary

**What:** Raw Fjall bytes become `Coin` / `BlockHash` in the adapter. The engine only sees domain types.

**When to use:** Every coins read/write.

**Trade-offs:** A coins codec to maintain. Prevents snapshot-DTO leakage into connect/reorg. Matches `standards/core/architecture.md`.

## Data Flow

### Request Flow

```
Peer getdata / RPC / sync connect
    ↓
ManagedPeerNetwork / DurableSyncRuntime
    ↓
ManagedChainstate (shell)
    ↓  injected time, cache size, persist mode
FlushPolicy (pure) → FlushDecision
    ↓
CoinsCache (pure) Get/Add/Spend or Flush/Sync
    ↓
FjallCoinsView or MemoryCoinsView
    ↓
Fjall keyspaces  |  in-memory test map
```

### State Management

```
Live truth: CoinsCache over FjallCoinsView + coins best-block
Active-chain metadata: headers / ChainPosition list (already persisted separately)
Block payloads: Fjall BlockIndex per-hash keys (already exist)
Serving label: Available only if load_block(hash) is Some
```

`ChainstateSnapshot` is no longer live truth. It may still be built for tests, wallet-rescan chunks, or one-time migration.

### Key Data Flows

1. **Connect a block:** Validate with coins from the cache (parent miss → Fjall get). Spend inputs, add outputs, write undo, set cache best-block to the new tip. Do **not** persist yet unless policy says so. `prepare_connect` uses a child cache; `commit` flushes that child into the live cache.

2. **Flush / Sync:** Cache emits a dirty batch + best-block. `FjallCoinsView` writes per-outpoint keys and best-block (and head-blocks if a write is two-phase). `Flush` then empties the cache; `Sync` drops spent entries and keeps unspent hits.

3. **Restart:** Open Fjall → attach coins view → init empty cache → tip from coins best-block, not from a reconstructed full UTXO map. Unflushed cache entries from a crash are gone; durable tip is the last successful batch.

4. **Persist progress today vs after:** Today `persist_progress` encodes the entire `ChainstateSnapshot` under `chainstate/snapshot` after header/connect work. After: flush dirty coins + save headers/runtime. Stop rewriting the whole UTXO set on every connected block.

5. **Honest serve:** Inventory gate asks “is the payload stored?” (`blocks_by_hash` **or** `FjallNodeStore::load_block`). Only then `BlockServingDataAvailability::Available`. RPC `resolve_block_intent` already NotFounds on `None`; the lie is labeling Available first via `durable_availability: true` in `gate_inventory_for_durable_serving`.

6. **Wallet rescan:** Today `required_chainstate_snapshot()` loads the full blob and filters by height. After: walk a coins cursor or height-indexed reads. Do not reintroduce a full-map snapshot as the rescan API.

## Integration Points

### Internal Boundaries

| Boundary | Communication | Notes |
|----------|---------------|-------|
| `Chainstate` ↔ `CoinsView` | Direct trait calls | Engine must not assume a `HashMap` or a snapshot |
| `CoinsCache` ↔ parent view | `get_coin` / `batch_write` | Parent may be memory or Fjall; cache must not know which |
| `FlushPolicy` ↔ `ManagedChainstate` | Pure decision in, persist out | Inject last-flush time, cache bytes, `FlushMode` |
| `ManagedChainstate` ↔ `FjallCoinsView` | Shell-owned adapter | Only place Fjall coins I/O happens |
| `DurableSyncRuntime` ↔ manager | Flush on progress / shutdown | Replace `save_chainstate_snapshot` as the live path |
| `ManagedPeerNetwork` ↔ manager | Connect/reorg/tip/snapshot-for-status | `AuthoritativeNetwork` can stay generic over a store, but production must not hydrate a full in-memory UTXO map from Fjall |
| Block serving ↔ `DurableBlockSource` | `load_block` | Presence fact must match this lookup |
| Wallet rescan ↔ coins view | Cursor / height reads | Snapshot helper only if built from the view, not from the legacy blob |
| Status / RPC / CLI ↔ one snapshot | Existing `OpenBitcoinStatusSnapshot` | Add flush/recovery/availability fields; do not invent a second tip |

### External Services

| Service | Integration Pattern | Notes |
|---------|---------------------|-------|
| Fjall | Existing `FjallNodeStore` keyspaces | Reuse `StorageNamespace::Chainstate`; change value layout from one `snapshot` key to per-coin keys + metadata |
| Bitcoin Knots `29.3.knots20260210` | Behavioral reference only | Cite `coins.h`, `coins.cpp`, `validation.cpp`, `node/chainstate.cpp`, `node/blockstorage.cpp` |
| LevelDB | Do not integrate | Intentional stack exception vs Knots |

### Current seams that make snapshot-style persist visible

| Seam | Today | After |
|------|-------|-------|
| `Chainstate` fields | `utxos: HashMap<OutPoint, Coin>` plus undo + chain | View/cache handle; undo + chain metadata remain first-class |
| `ManagedChainstate::persist` | `save_snapshot(self.chainstate.snapshot())` after every mutation | Policy-driven flush; memory store used only in tests |
| `DurableSyncRuntime::open` | Load snapshot blob into `MemoryChainstateStore` | Attach `FjallCoinsView`, then init cache |
| `persist_progress` | `save_chainstate_snapshot` of the full map | Coins flush + headers + runtime metadata |
| `gate_inventory_for_durable_serving` | `durable_availability: true` | Payload-present from store or local cache |
| `serve_inventory` | In-memory `blocks_by_hash` only | Same honesty rule if that path can report Available |
| Confirmation migration | Rebuild counts from stored active-chain blocks | Keep as a one-time adapter; do not depend on a UTXO blob |

## Scaling Considerations

This milestone scales with **UTXO set size and IBD write amplification**, not user count.

| Scale | Architecture adjustments |
|-------|--------------------------|
| Fixtures / regtest | `MemoryCoinsView` + cache; snapshot helpers in tests are fine |
| Short public-mainnet review | Disk coins + periodic `Sync`; do not reload the full set on restart |
| Full mainnet UTXO | Per-outpoint Fjall keys + bounded cache; snapshot blob is a non-starter |

### Scaling Priorities

1. **First bottleneck:** Full-map clone on every `connect_block` / `prepare_connect` and a complete snapshot write on persist. Fix with a cache overlay and incremental batch write.
2. **Second bottleneck:** Loading the entire UTXO set into `MemoryChainstateStore` at `DurableSyncRuntime::open`. Fix with disk parent + empty cache + best-block tip.
3. **Third bottleneck:** Serving labels that assume durable payloads exist. That does not scale to honest historical getdata; fix before operator evidence.

Cache byte budget and Fjall batch atomicity need phase-level research (MEDIUM). Do not pick Knots `dbcache` defaults until that research runs.

## Anti-Patterns

### Anti-Pattern 1: Fjall or filesystem in `open-bitcoin-chainstate`

**What people do:** Implement `FjallCoinsView` next to the engine “for convenience.”

**Why it's wrong:** Breaks functional core / imperative shell, architecture-policy checks, and hermetic engine tests.

**Do this instead:** Pure `CoinsView` trait + `CoinsCache` + `MemoryCoinsView`. Fjall adapter in `open-bitcoin-node/src/storage`.

### Anti-Pattern 2: Keep the snapshot blob as live truth

**What people do:** Write per-coin keys *and* still `save_chainstate_snapshot` after every connect.

**Why it's wrong:** Two truths, double write amplification, restart will pick the wrong one.

**Do this instead:** One durable coins view. Snapshot encode stays for migration and tests only.

### Anti-Pattern 3: Clone `Chainstate` to prepare a connect

**What people do:** Keep `prepare_connect_block` as `self.chainstate.clone()`.

**Why it's wrong:** Clone copies the entire UTXO map. That is the snapshot architecture.

**Do this instead:** Child `CoinsCache` over the live cache; commit is `Flush` into the parent cache.

### Anti-Pattern 4: Hardcode durable availability

**What people do:** `managed_block_serve_input(..., durable_availability: true)` because a later `load_block` can NotFound.

**Why it's wrong:** Status/eligibility already reported Available. Operators and peers see a lie; `LookupUnavailable` is a correction, not honesty.

**Do this instead:** `Available` only when `blocks_by_hash` or `load_block` has the payload. Missing payload on an active hash is Unavailable (or the existing `Pruned` label if the classifier still needs that distinction). Refuse with `NotFound`.

### Anti-Pattern 5: Port Knots `CoinsViews` dual-chainstate

**What people do:** Add background/snapshot chainstate because `validation.h` has it.

**Why it's wrong:** Assumeutxo is explicitly out of scope. Dual tips would fork manager work.

**Do this instead:** One active chainstate, one coins view stack, one best-block.

### Anti-Pattern 6: Put the clock in flush policy

**What people do:** `Instant::now()` inside `decide_flush`.

**Why it's wrong:** Core becomes impure; tests need real time.

**Do this instead:** Inject `now` and `last_flush` from the shell, same as mempool rolling-fee time.

### Anti-Pattern 7: Claim a block available because the header or coins tip exists

**What people do:** Treat coins best-block or header index as “we can serve this block.”

**Why it's wrong:** Headers and coins can exist without the block payload. v2.3 availability is payload-present.

**Do this instead:** Serving reads `FjallNodeStore::load_block` (or the in-memory cache of a stored payload). Coins tip answers “what UTXO set is this,” not “can I send `block`.”

## Suggested Build Order

Dependency order for roadmap phases starting at Phase 139:

1. **Typed coins-view contracts** — `CoinsView`, cache-entry dirty/fresh flags, batch-write cursor, best-block, `MemoryCoinsView`. No Fjall. Engine tests can still use today’s snapshot helpers.

2. **Flush policy** — Pure `FlushMode` / cache-size / `Flush` vs `Sync` decisions with injected time and size. Unit tests only.

3. **Engine apply on a view** — Refactor connect/disconnect/reorg to mutate a `CoinsCache`. Keep undo and chain metadata. Replace prepare/commit clone with a child cache. Still no disk.

4. **Durable Fjall adapter** — Per-outpoint coins, best-block, head-blocks, schema bump, one-way migration from the legacy `snapshot` blob. `save_block` / `load_block` unchanged. No manager rewrite yet.

5. **Manager orchestration** — `ManagedChainstate` owns cache init, flush points, shutdown flush, and restart from coins best-block. `DurableSyncRuntime::open` and `persist_progress` switch off snapshot-blob writes.

6. **Availability truth** — Payload-present fact into `managed_block_serve_input` and inbound `DurableBlock` gating. Active-but-missing-bytes refuses cleanly. Do not wait until docs to stop the hardcoded `true`.

7. **Operator and parity evidence** — Status/RPC/CLI/dashboard/metrics/logs: flush, recovery, honest serve labels. Update `docs/parity/catalog/chainstate.md` and breadcrumbs. Keep public/production claims deferred.

8. **Consumers that still load the blob** — Wallet rescan, confirmation migration leftovers, and any RPC that materializes `ChainstateSnapshot.utxos` must read the view/cursor so they cannot resurrect snapshot-as-truth.

Steps 1–3 stay in the pure crate. Step 4 is the first Fjall change. Step 5 is the first runtime wiring. Step 6 can start as soon as `load_block` is the presence oracle — it does not need the coins adapter, but it must not ship *after* operator evidence that would re-document the lie.

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| Current Open Bitcoin layout | HIGH | Read engine, `ManagedChainstate`, Fjall snapshot persist, sync open/persist, inventory, inbound wire |
| Knots coins / manager APIs | HIGH | Official `v29.3.knots20260210` `coins.h` and `validation.h`; published Flush vs Sync behavior |
| Fjall per-coin schema / atomic batch | MEDIUM | Needs phase research; do not copy LevelDB layout |
| Exact flush thresholds / cache bytes | MEDIUM | Knots `dbcache` and periodic intervals should be researched before copying numbers |
| Local `packages/bitcoin-knots` tree | LOW for file-path reads | Submodule was not materialized in this session; citations use the pinned GitHub tag |

## Sources

- Open Bitcoin engine and types: `packages/open-bitcoin-chainstate/src/engine.rs`, `types.rs`, `lib.rs`
- Node manager and snapshot store: `packages/open-bitcoin-node/src/chainstate.rs`
- Fjall snapshot + per-hash blocks: `packages/open-bitcoin-node/src/storage.rs`, `storage/fjall_store.rs`
- Sync hydrate/persist: `packages/open-bitcoin-node/src/sync.rs`, `sync/runtime_state.rs`
- Serving honesty gap: `packages/open-bitcoin-node/src/network/inventory.rs` (`durable_availability: true`); `packages/open-bitcoin-rpc/src/context/inbound_wire.rs`
- Architecture rules: `standards/core/architecture.md`, `.planning/ARCHITECTURE.md`, `.planning/PROJECT.md`
- Parity gap: `docs/parity/catalog/chainstate.md` (disk-backed coins and full manager still listed as known gaps)
- Pinned Knots coins API: https://github.com/bitcoinknots/bitcoin/blob/v29.3.knots20260210/src/coins.h
- Pinned Knots chainstate / flush API: https://github.com/bitcoinknots/bitcoin/blob/v29.3.knots20260210/src/validation.h (`CoinsViews`, `CoinsTip`, `FlushStateMode`, `FlushStateToDisk`, `InitCoinsCache`)
- Knots periodic flush retains cache via `Sync`: https://github.com/bitcoinknots/bitcoin/commit/2cafbe3796cd811826c8aa48ac26bd4bda2895a3
- Knots post-IBD `CoinsTip().Sync()`: https://github.com/bitcoinknots/bitcoin/commit/2f1bf089f7540356daf8c88ab81341d1248ad81b

---
*Architecture research for: Open Bitcoin chainstate durability integration*
*Researched: 2026-08-29*
