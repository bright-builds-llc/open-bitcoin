# Phase 139: Coins-View, Cache Contract, and Engine Apply - Research

**Researched:** 2026-08-30
**Domain:** In-memory Knots-shaped coins overlay (DIRTY/FRESH cache algebra + engine apply)
**Confidence:** HIGH

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

#### View/cache layering contract

- **D-01:** Use the Knots-shaped stack: a pure `CoinsView` trait, a resident
  `CoinsCache<V>` as the only DIRTY/FRESH overlay, and `MemoryCoinsView` as
  the in-memory parent. Do not keep `Chainstate.utxos: HashMap` as the apply
  target. Do not make `CoinsCache` prepare-only or eager-flush after every
  block.
- **D-02:** `CoinsView` exposes `get_coin`, `have_coin`, `best_block`, and
  `batch_write`. Optional cursor types may be sketched for later rescan, but
  Phase 139 must not require a production cursor walk.
- **D-03:** `CoinsCache` is the live UTXO working set. `MemoryCoinsView` is
  parent truth and is written only through `Flush` / `Sync` / `batch_write`.
  The cache must not know whether the parent is memory or a future
  `FjallCoinsView`.
- **D-04:** `CoinsCache` is not `Clone`. Prepare isolation is a child cache,
  not a cloned map or cloned `Chainstate`.
- **D-05:** `Chainstate` keeps `active_chain`, undo, confirmation counts, and
  a `CoinsCache<MemoryCoinsView>` handle. It stops owning the live UTXO map.
- **D-06:** Do not port `CCoinsViewBacked`, the flagged linked-list sentinel,
  `CCoinsViewMemPool`, dual `CoinsViews`, or assumeutxo snapshot chainstate.
- **D-07:** Place new types in `open-bitcoin-chainstate` as
  `coins.rs` / `coins/cache.rs` / `coins/memory.rs`. Do not put Fjall,
  `std::fs`, Tokio, or a wall clock in this crate.

#### DIRTY/FRESH occupancy vs HaveCoin

- **D-08:** Keep four lookup facts, matching pinned Knots `CCoinsViewCache`:
  map occupancy (includes spent DIRTY tombstones), `have_coin_in_cache`
  (local unspent only, no parent call), `have_coin` / `get_coin` (FetchCoin
  then unspent), and parent-view truth (consulted only on occupancy miss).
- **D-09:** A spent-in-cache coin is a cache hit that answers
  `have_coin = false` without asking the parent. Parent must not resurrect a
  spent outpoint.
- **D-10:** `Coin` stays a live output. Spentness and DIRTY/FRESH live on
  `CoinsCacheEntry`, not on `Coin`.
- **D-11:** Support the five valid Knots combinations: unspent clean, unspent
  DIRTY, unspent FRESH|DIRTY, spent DIRTY, and spent FRESH-not-DIRTY. Encode
  `SanityCheck`-style invalid combos as typed construction failures or
  invariant tests, not silent states.
- **D-12:** Port dirty/fresh transitions as unit-tested cache algebra:
  reorg re-add of a dirty spent coin must not become FRESH; FRESH+spent may
  be dropped from the child; FRESH+spent against a FRESH parent may delete
  the parent instead of writing a tombstone. Do not invent write-through
  "every mutation is parent truth."
- **D-13:** Engine helpers must stop treating `HashMap::contains_key` /
  `remove` as spendability. Occupancy is not HaveCoin.

#### Prepare/commit without cloning

- **D-14:** `prepare_connect` / `prepare_reorg` use a peek-only child
  `CoinsCache` over the live cache. Apply connect, disconnect, and a whole
  reorg only on the child. Commit flushes dirty entries, including spent
  tombstones, into the parent cache. Drop the child on any prepare error.
- **D-15:** Child reads must peek, not `FetchCoin` into the parent. A failed
  prepare must leave parent occupancy, flags, and best-block unchanged.
- **D-16:** Child→parent commit is always an emptying Flush of the child,
  not Phase 140 disk Sync and not a durability proof.
- **D-17:** `PreparedChainstateConnect` / `PreparedChainstateReorg` own the
  child's overlay plus staged `BlockUndo` / `ChainPosition`. Do not put undo
  inside the cache. Do not clone `self.chainstate` to isolate prepare.
- **D-18:** Keep the existing two-phase manager lifecycle:
  prepare chainstate → prepare mempool → commit both. Isolation must still
  hold across that borrow/commit window.
- **D-19:** `connect_block` / `disconnect_tip` / `reorg` in the engine apply
  against the view/cache. They must not clone the full UTXO map into
  `next_utxos`.

#### Existing snapshot/HashMap compatibility

- **D-20:** Hydrate `from_snapshot` into `MemoryCoinsView` + cache. Keep
  `snapshot()`, `from_snapshot()`, and `utxos()` as hydrate/export/test
  helpers. Engine tests may keep `from_snapshot` and `utxos().len()`.
- **D-21:** Do not delete `ChainstateSnapshot` or stop
  `ManagedChainstate::persist` / `MemoryChainstateStore::save_snapshot` in
  this phase. Snapshot-blob persist remains live until Phases 141–142.
- **D-22:** `snapshot()` may still materialize a full map at export/persist
  edges. That is allowed leftover write amplification. The forbidden clone
  is connect/prepare/reorg apply.
- **D-23:** A cache hit is not a durability proof. Do not add operator
  "durably persisted" claims from cache occupancy.

### Claude's Discretion

- Exact module split inside `coins/` as long as D-07 crate boundaries hold.
- Whether DIRTY/FRESH are bitflags or a five-state enum, provided D-08–D-12
  facts and transitions remain.
- Cursor type presence or absence in Phase 139.
- How `utxos()` is implemented as a view-backed test convenience.

### Deferred Ideas (OUT OF SCOPE)

- Flush policy `None` / `IfNeeded` / `Periodic` / `Always` and cache-size
  OK/LARGE/CRITICAL — Phase 140
- `FjallCoinsView`, per-outpoint keys, `head_blocks`, leftover-snapshot
  non-authority, coins disk-read fail-closed — Phase 141
- Manager init, CanFlush readiness, restart from coins best-block,
  `persist_progress` cutover — Phase 142
- Honest payload-present availability — Phase 143
- Operator flush/recovery evidence — Phase 144
- Parity-root closeout and no-claim guardrails — Phase 145
- Wallet rescan / confirmation migration off the snapshot blob — later
  consumer follow-through, not Phase 139
- Assumeutxo dual-chainstate, prune/archive product modes, LevelDB,
  rust-bitcoin — out of v2.3
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| CACHE-01 | Node overlays an in-memory DIRTY/FRESH coins cache so connect and disconnect do not persist the whole UTXO set after every block. | This phase delivers the overlay, four lookup facts, and child-cache prepare/commit. Persist still writes today's snapshot blob (D-21). Write amplification of *apply* ends; write amplification of *persist* ends in Phases 141–142. Standard Stack + Architecture Patterns 1–4 + Code Examples encode the Knots algebra to implement. |
</phase_requirements>

## Summary

Phase 139 replaces `Chainstate.utxos: HashMap` and `next_utxos = self.utxos.clone()` with a Knots-shaped layered view: a pure `CoinsView` trait, a resident `CoinsCache<MemoryCoinsView>` working set, and DIRTY/FRESH cache-entry algebra. Connect, disconnect, and reorg mutate the cache (or a peek-only child overlay) instead of cloning the full map. Prepare/commit keeps the existing two-phase manager API but isolates with a child overlay that Flushes into the live cache on commit and is dropped on failure.

Pinned Knots `29.3.knots20260210` `CCoinsViewCache` is the behavioral spec for FetchCoin, AddCoin, SpendCoin, BatchWrite, HaveCoin vs HaveCoinInCache, and SanityCheck's five valid flag combinations. Open Bitcoin must not copy Knots' spent-null `Coin`, linked-list sentinel, `CCoinsViewBacked`, or parent-populating FetchCoin during prepare. `Coin` stays a live output; spentness lives on `CoinsCacheEntry`. The chainstate crate stays I/O-free. Snapshot helpers and `ManagedChainstate::persist` stay as leftover export/persist edges.

**Primary recommendation:** Implement `coins.rs` + `coins/cache.rs` + `coins/memory.rs` first with unit-tested Knots algebra, then retarget engine apply helpers onto cache `get_coin` / `have_coin` / `add_coin` / `spend_coin`, then change manager prepare to own a detached child overlay that Flushes on commit.

## Project Constraints (from .cursor/rules/)

No `.cursor/rules/` directory exists in this repo. Actionable constraints come from repo-local `AGENTS.md` / Bright Builds pages already loaded:

- Functional core / imperative shell: no Fjall, `std::fs`, Tokio, or clock in `open-bitcoin-chainstate`. [VERIFIED: standards/core/architecture.md; scripts/check-pure-core-deps.sh]
- No `unwrap()` in production; `expect()` only with a strong panic-impossible belief. [VERIFIED: user code-styling rule + crate `deny(clippy::unwrap_used)`]
- New first-party Rust sources need parity breadcrumbs via `docs/parity/source-breadcrumbs.json` and `bun run scripts/check-parity-breadcrumbs.ts --write`. [VERIFIED: docs/parity/source-breadcrumbs.json]
- `bash scripts/verify.sh` is the completion contract; do not run overlapping Cargo jobs during research/planning. [VERIFIED: AGENTS.md]
- File-length gate fails at 629 physical lines. `engine.rs` is already 584 lines — extract apply helpers before growing it. [VERIFIED: standards/core/code-shape.md; packages/open-bitcoin-chainstate/src/engine.rs]

## Standard Stack

This phase adds **no new crates**. The coins overlay is first-party Rust in the existing pure-core crate.

### Core

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| Rust edition 2024 | 1.94.1 (`rust-toolchain.toml`) | Language / toolchain | Pinned repo source of truth [VERIFIED: rust-toolchain.toml; AGENTS.md] |
| `std::collections::HashMap` | std | Cache overlay + MemoryCoinsView parent map | Existing chainstate pattern; no extra hasher crate [VERIFIED: engine.rs] |
| `open-bitcoin-primitives` | workspace path | `OutPoint`, `BlockHash`, `TransactionOutput` | Existing domain types [VERIFIED: packages/open-bitcoin-chainstate/Cargo.toml] |
| `open-bitcoin-consensus` | workspace path | Contextual validation already used by connect | Do not re-validate inside the cache [VERIFIED: engine.rs] |

### Supporting

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| Existing `ChainstateError` | crate-local | Typed view/cache failures | Extend; do not add a second error type [VERIFIED: error.rs] |
| Existing `Coin` / `TxUndo` / `BlockUndo` | crate-local | Live output + undo payloads | Keep domain meaning; do not move spentness onto `Coin` [VERIFIED: types.rs; D-10] |
| `scripts/check-pure-core-deps.sh` | repo script | Forbid tokio/reqwest/rustls/rand and `std::fs`/`std::net`/`std::env`/`std::process`/`std::thread` | Run after crate edits [VERIFIED: scripts/check-pure-core-deps.sh] |
| `docs/parity/source-breadcrumbs.json` | repo mapping | Breadcrumb comments on new `coins*` / apply files | Add files to `chainstate-engine` group citing `coins.h` / `coins.cpp` [VERIFIED: source-breadcrumbs.json] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| First-party `CoinsView` | rust-bitcoin / bitcoincore-rpc coin types | Forbidden by project dependency policy [VERIFIED: AGENTS.md PROJECT constraints] |
| Bitflags crate | Five-state enum (recommended) | Extra dep for two bits; enum makes D-11 illegal states unrepresentable [VERIFIED: standards/core/architecture.md] |
| `CCoinsViewBacked` pointer stack | `CoinsCache<V: CoinsView>` generic | Locked: do not port Backed [VERIFIED: D-06; coins.h:342-358] |
| Linked-list dirty cursor | `HashMap` scan of dirty entries on Flush | D-06 forbids the sentinel list; a `Vec` of dirty outpoints is enough at fixture scale [VERIFIED: D-06; coins.h:111-126] |

**Installation:** none. Do not add Cargo dependencies to `open-bitcoin-chainstate`.

**Version verification:** Toolchain pin is `1.94.1` in `rust-toolchain.toml`. Crate deps are path-only (`open-bitcoin-consensus`, `open-bitcoin-primitives`). [VERIFIED: packages/open-bitcoin-chainstate/Cargo.toml]

## Architecture Patterns

### Recommended Project Structure

```
packages/open-bitcoin-chainstate/src/
├── coins.rs                 # CoinsView, CoinsBatch, CoinsCacheEntry, flag type, re-exports
├── coins/
│   ├── cache.rs             # CoinsCache: FetchCoin, peek, AddCoin, SpendCoin, Flush, Sync, BatchWrite
│   └── memory.rs            # MemoryCoinsView: HashMap + best_block
├── engine.rs                # Chainstate fields + connect/disconnect/reorg orchestration (keep thin)
├── engine/
│   ├── apply.rs             # NEW extract: spend/add/restore/context against CoinsCache (file-length)
│   └── tests.rs             # existing fixtures; keep from_snapshot / utxos()
├── types.rs                 # Coin / undo / snapshot unchanged as domain/DTO
├── error.rs                 # add FreshFlagMisapplied + cache construction errors
└── lib.rs                   # pub mod coins; re-export CoinsView, CoinsCache, MemoryCoinsView

packages/open-bitcoin-node/src/chainstate.rs
                              # prepare_* owns detached overlay; commit Flushes; persist() still snapshot()
```

Do **not** add `coins/flush.rs` in this phase (Phase 140). Do **not** add `FjallCoinsView` (Phase 141).

### Pattern 1: Layered coins view (locked D-01)

**What:** Parent view is coin truth. Cache answers occupancy hits in memory and records DIRTY/FRESH mutations. `Flush` batch-writes dirty entries (including spent tombstones) then empties the child. `Sync` batch-writes dirty entries, drops spent, keeps unspent clean. [VERIFIED: coins.h:441-455; coins.cpp:250-272; .planning/research/ARCHITECTURE.md Pattern 1]

**When to use:** Every connect/disconnect/reorg and every prepare/commit.

**Phase 139 parent is always `MemoryCoinsView`.** Cache must not mention Fjall. [VERIFIED: D-03; D-07]

```rust
// Source: .planning/research/ARCHITECTURE.md Pattern 1, aligned to D-02
pub trait CoinsView {
    fn get_coin(&self, outpoint: &OutPoint) -> Result<Option<Coin>, ChainstateError>;
    fn have_coin(&self, outpoint: &OutPoint) -> Result<bool, ChainstateError>;
    fn best_block(&self) -> Option<BlockHash>;
    fn batch_write(
        &mut self,
        writes: CoinsBatch,
        best_block: Option<BlockHash>,
    ) -> Result<(), ChainstateError>;
}
```

`CoinsView::get_coin` / `have_coin` **never insert** into the view being asked. That is the parent-safe peek contract. `CoinsCache` working-set fill is a separate `&mut self` `fetch_coin`. [VERIFIED: D-15; coins.cpp:48-64 shows Knots FetchCoin inserts into *self*; child must not call that on the parent]

Do not add `GetHeadBlocks`, `Cursor`, `EstimateSize`, or `HaveInputs` as required trait methods. [VERIFIED: D-02; D-06]

### Pattern 2: Four lookup facts (locked D-08–D-09)

**What:** Occupancy, in-cache unspent, HaveCoin, and parent truth are different questions. [VERIFIED: coins.h:399-404; coins.cpp:66-70, 163-171]

| Fact | Knots API | Open Bitcoin method | Parent call? | Spent tombstone |
|------|-----------|---------------------|--------------|-----------------|
| Map occupancy | `cacheCoins.find` | `contains_in_cache` / `occupancy` | No | Yes (hit) |
| In-cache unspent | `HaveCoinInCache` | `have_coin_in_cache` | No | No (`false`) |
| HaveCoin / GetCoin | `HaveCoin` / `GetCoin` | `have_coin` / `get_coin` on the *current* cache | Only on occupancy miss (`fetch_coin`) | Hit, `have_coin=false` |
| Parent-view truth | `base->GetCoin` | `parent.get_coin` | Yes | Must not resurrect |

`HaveCoinInCache` is `find && !IsSpent` with **no** base call. [VERIFIED: coins.cpp:168-171]

`GetCoin` / `HaveCoin` FetchCoin then treat spent as absent. [VERIFIED: coins.cpp:66-70, 163-166]

### Pattern 3: Detached child overlay for prepare (locked D-14–D-18)

**What:** Prepare applies on a child overlay. Child FetchCoin peeks the live cache without inserting into it. Prepared owns the overlay plus staged chain/undo metadata. Commit is an emptying Flush into the live cache. Drop overlay on error.

**Why not `CoinsCache<&CoinsCache<V>>`:** `prepare_*` is `&self` and the result must live across mempool prepare (`connect_local_block` does chainstate prepare → mempool prepare → commit both). A child that borrows the live cache cannot be stored in `Prepared*` across that window without lifetime infection of `ManagedPeerNetwork`. [VERIFIED: packages/open-bitcoin-node/src/network/mempool_lifecycle.rs:48-67; chainstate.rs:147-166]

**Prescribed shape:**

```rust
// Apply-time only (stack borrow of live cache as peek parent):
fn apply_connect_on_child(
    parent: &CoinsCache<MemoryCoinsView>,
    overlay: &mut CoinsOverlay,
    block: &Block,
    ...
) -> Result<StagedConnect, ChainstateError>;

// Stored in Prepared* (no parent borrow):
struct CoinsOverlay {
    entries: HashMap<OutPoint, CoinsCacheEntry>,
    best_block: Option<BlockHash>,
}

pub(crate) struct PreparedChainstateConnect {
    overlay: CoinsOverlay,
    undo: BlockUndo,
    position: ChainPosition,
    next_active_chain: Vec<ChainPosition>,
    next_confirmed_txid_counts: Option<HashMap<Txid, u32>>,
}
```

On commit: `live.batch_write(overlay.into_dirty_batch(), overlay.best_block)` then install staged chain/undo/counts. That is Knots `Flush` (`will_erase=true`) of the child into the parent cache. [VERIFIED: coins.cpp:250-258; D-16]

### Pattern 4: Engine apply without `next_utxos.clone()` (locked D-13, D-19)

**What:** `connect_block` currently clones the full map, mutates `next_utxos`, then assigns on success. [VERIFIED: engine.rs:158-200] After this phase, apply spends/adds through cache methods. For atomicity without a full-map clone, even the public `Chainstate::connect_block` / `disconnect_tip` / `reorg` apply on a local child overlay and Flush on success. Failed apply drops the overlay so live occupancy/flags/best-block stay unchanged — same isolation prepare needs.

Replace these engine helpers:

| Current helper | Forbidden use | Replacement |
|----------------|---------------|-------------|
| `utxos.get` / `next_utxos.get` | Occupancy as spendability | `cache.get_coin` / `have_coin` (FetchCoin on *this* cache) |
| `next_utxos.remove` (`remove_spent_input`) | Remove occupancy | `cache.spend_coin` (FRESH erase vs DIRTY tombstone) |
| `utxos.contains_key` (`add_transaction_outputs`, `restore_non_coinbase_inputs`) | Occupancy as overwrite | `cache.have_coin` for BIP30/live-unspent; occupancy only when testing tombstone re-add |

Keep consensus order: spend inputs before add outputs on connect; remove created outputs before restore undo on disconnect. [VERIFIED: docs/parity/catalog/chainstate.md; engine.rs:162-186, 255-262]

### Anti-Patterns to Avoid

- **Fjall / `std::fs` / Tokio / `Instant` in `open-bitcoin-chainstate`:** Architecture-policy and D-07. `check-pure-core-deps.sh` does **not** list `fjall` or `std::time` in its forbidden set — do not treat a green checker as permission. [VERIFIED: scripts/check-pure-core-deps.sh:28-29]
- **`Chainstate.clone()` / `utxos.clone()` for prepare or apply:** Anti-Pattern 3. [VERIFIED: .planning/research/ARCHITECTURE.md; D-04; D-17; D-19]
- **Write-through every mutation into `MemoryCoinsView`:** Destroys DIRTY/FRESH and FRESH-spend-erase. [VERIFIED: D-12; PITFALLS.md Pitfall 3]
- **Porting `CoinsViews` dual-chainstate / `CCoinsViewBacked` / linked-list sentinel:** D-06. [VERIFIED: coins.h:342-358, 111-126; validation.h:481-503]
- **`HashMap::contains_key` as HaveCoin:** Occupancy includes spent DIRTY tombstones. [VERIFIED: D-13; coins.cpp:168-171]
- **Spentness on `Coin`:** Knots nulls `CTxOut`; Open Bitcoin `Coin` stays a live output. [VERIFIED: D-10; types.rs:16-21; coins.h:78-83]
- **Closing the catalog disk-backed gap:** Leave `docs/parity/catalog/chainstate.md` known-gap sentence in place. [VERIFIED: 139-CONTEXT.md canonical refs]
- **Cache hit as durability / persist skip:** `persist()` still `save_snapshot`. [VERIFIED: D-21; D-23; node chainstate.rs:242-244]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| DIRTY/FRESH transitions | Ad-hoc bools or "write every mutation" | Port Knots `AddCoin` / `SpendCoin` / `BatchWrite` as unit-tested algebra | FRESH misapply is a consensus-class bug [VERIFIED: coins.cpp:86-99, 214-219; PITFALLS.md Pitfall 3] |
| Prepare isolation | `self.chainstate.clone()` | Detached child overlay + Flush | Clone copies the UTXO map [VERIFIED: D-04; D-17; node chainstate.rs:155] |
| Spent tombstones | Delete keys on every spend | FRESH erase vs DIRTY `Clear` | Reorg re-add of DIRTY spent must stay non-FRESH [VERIFIED: coins.cpp:86-99, 143-148] |
| Parent populate during peek | Child `get_coin` → parent `fetch_coin` | Parent `CoinsView::get_coin` is peek-only | Knots 29.3 `GetCoin` on a cache parent *does* FetchCoin into the parent; D-15 forbids that [VERIFIED: coins.cpp:48-69; D-15] |
| Cursor / dirty list | Port `CoinsCachePair` sentinel | Collect dirty entries into `CoinsBatch` at Flush | D-06 [VERIFIED: coins.h:264-306] |
| New error type tree | `CoinsError` crate | Extend `ChainstateError` | Existing engine/node `?` paths [VERIFIED: error.rs] |
| rust-bitcoin `OutPoint`/`TxOut` | Third-party coin types | First-party `Coin` / `OutPoint` | Project dependency policy [VERIFIED: AGENTS.md] |

**Key insight:** The hard part is not a HashMap overlay. It is keeping occupancy, HaveCoin, FRESH, and parent truth as four facts so a reorg cannot resurrect a spent outpoint or drop spentness before Flush.

## Runtime State Inventory

This phase refactors the in-memory apply path. It does not rename persisted keys or cut over durable coins truth.

| Category | Items Found | Action Required |
|----------|-------------|------------------|
| Stored data | Fjall `chainstate/snapshot` blob via `ManagedChainstate::persist` → `save_snapshot(self.chainstate.snapshot())` | Code edit only: `snapshot()` may rematerialize from cache+parent. No data migration. Do not stop writing the blob (D-21). [VERIFIED: node chainstate.rs:242-244] |
| Live service config | None — no n8n/Datadog/service-name coupling to UTXO map internals | None — verified by repo search of persist/snapshot seams |
| OS-registered state | None — no systemd/launchd unit embeds the UTXO map type | None |
| Secrets/env vars | None — no env key named after `utxos` / `CoinsCache` | None |
| Build artifacts | Local `target/` Cargo artifacts will rebuild; no installed package name change | Rebuild via later verify; do not treat stale `target/` as source of truth |

**After every file in the repo is updated:** existing datadirs still have a snapshot blob. Phase 139 must keep reading/writing that blob. Runtime coins truth remains the in-memory cache+parent until Phase 141–142.

## Common Pitfalls

### Pitfall 1: Child FetchCoin populates the live cache

**What goes wrong:** Child miss calls parent `GetCoin`, parent `FetchCoin` inserts a clean entry into the live cache. Failed prepare leaves parent occupancy/flags changed. [VERIFIED: coins.cpp:48-64, 66-69; D-15]

**Why it happens:** Knots 29.3 `CCoinsViewCache::GetCoin` always FetchCoins into *that* cache. A child layered on a cache parent therefore warms the parent. Pinned Knots has no `PeekCoin`. [VERIFIED: coins.h / coins.cpp; grep PeekCoin = no matches]

**How to avoid:** `CoinsView::get_coin` is peek-only. `CoinsCache::fetch_coin` inserts only into `self`. Child fetch on miss calls `parent.get_coin()` / `parent.peek_coin()`, never `parent.fetch_coin()`.

**Warning signs:** Unit test that FetchCoins an untouched outpoint on a child and then sees `have_coin_in_cache` / occupancy on the parent.

### Pitfall 2: Occupancy used as spendability

**What goes wrong:** `contains_key` is true for a DIRTY spent tombstone, so connect refuses a valid re-add or restore, or treats a spent coin as spendable. [VERIFIED: D-13; engine.rs:445-446, 503-504; mempool_lifecycle.rs:348-352]

**Why it happens:** Today's HashMap only stores live coins, so occupancy == HaveCoin. After tombstones exist, that equality dies.

**How to avoid:** Engine add/restore/spend go through `have_coin` / `spend_coin` / `add_coin`. `utxos()` exports **unspent only**. Mempool `utxos().contains_key` stays correct if and only if `utxos()` is unspent-only; prefer adding `Chainstate::have_coin` and switching that one call site.

**Warning signs:** Tests that insert a spent DIRTY entry and then fail BIP30/overwrite checks via `contains_key`.

### Pitfall 3: FRESH on reorg re-add of a DIRTY spent coin

**What goes wrong:** Disconnect spends a coin (DIRTY tombstone). Reconnect adds it again and marks FRESH. A later spend erases the FRESH entry. Flush never writes spentness. Restart (later phases) resurrects the coin. [VERIFIED: coins.cpp:86-99; PITFALLS.md Pitfall 3]

**How to avoid:** Port the Knots comment as a unit test: `fresh = !existing.IsDirty()` when overwriting a spent occupancy. Do not mark FRESH when `possible_overwrite` is true.

**Warning signs:** Child Flush of a re-added-then-respent coin produces no dirty spent record.

### Pitfall 4: FRESH flag misapplied into a live parent

**What goes wrong:** Child marks FRESH for a coin that is unspent in the parent. `BatchWrite` throws `FRESH flag misapplied to coin that exists in parent cache`. [VERIFIED: coins.cpp:214-219]

**How to avoid:** FRESH means "parent has no unspent coin." Child AddCoin after a peek miss may be FRESH; after a peek hit of unspent, it must not.

**Warning signs:** Flush/commit panics or returns `FreshFlagMisapplied`.

### Pitfall 5: FRESH+spent against FRESH parent must delete, not tombstone

**What goes wrong:** Child spends a FRESH parent entry and writes a tombstone into the parent. Parent now holds a spent coin that the grandparent never had. [VERIFIED: coins.cpp:222-226; D-12]

**How to avoid:** Port `BatchWrite` branch: if parent entry is FRESH and child coin is spent, erase parent entry.

### Pitfall 6: In-place connect loses apply atomicity

**What goes wrong:** Removing `next_utxos.clone()` and mutating the live cache directly means a mid-block `MissingCoin` / validation error leaves partial spends in the live overlay. [VERIFIED: engine.rs:158-200 — today's clone is the atomicity mechanism]

**How to avoid:** Public `connect_block` / `disconnect_tip` / `reorg` also apply on a local child overlay and Flush only on success. Prepare reuses that helper.

**Warning signs:** A failing connect test changes `have_coin_in_cache` or `best_block` on the original `Chainstate`.

### Pitfall 7: `Chainstate: Clone` compile break on the network crate

**What goes wrong:** `CoinsCache` is not `Clone` (D-04). `Chainstate` currently `#[derive(Clone)]`. `ManagedChainstate` and `ManagedPeerNetwork` derive `Clone`. `reorg_to_branch` does `let mut staged = self.clone()` then `install_prepared_reorg_preview` which does `prepared.preview().clone()`. [VERIFIED: engine.rs:29-30; node chainstate.rs:44-45, 73-74, 225-226; network.rs:113-115; mempool_lifecycle.rs:156-159]

**How to avoid:** Do not impl `Clone` on `CoinsCache`. Drop derive `Clone` on `Chainstate`. Keep prepare off `Chainstate.clone()`. For `ManagedPeerNetwork::clone()` / preview install, use `snapshot()` / `from_snapshot()` as a D-22 leftover export-edge clone, **or** change `preview()` / `install_prepared_reorg_preview` to Flush the prepared overlay into the staged manager without cloning the live cache. Do not use that leftover clone inside `prepare_*`.

**Warning signs:** `prepare_connect_block` still contains `self.chainstate.clone()`.

### Pitfall 8: `utxos()` cannot return `&HashMap` after the split

**What goes wrong:** Live coins live in parent + overlay. There is no single map to borrow. [VERIFIED: engine.rs:76-78; D-05]

**How to avoid:** Change `utxos()` to return an owned `HashMap<OutPoint, Coin>` of **unspent** coins collected from parent + overlay. Update `.len()` / `==` tests (they still work). Do not keep a second materialized live map on `Chainstate`.

### Pitfall 9: File-length overflow on `engine.rs`

**What goes wrong:** `engine.rs` is 584 lines; the Bright Builds file-length gate is 629. View-apply edits will fail the gate. [VERIFIED: engine.rs line count; standards/core/code-shape.md]

**How to avoid:** Extract apply helpers into `engine/apply.rs` (or equivalent) in the same plan that retargets them onto the cache. Prefer `engine.rs` + `engine/apply.rs` over `engine/mod.rs`.

### Pitfall 10: Closing persist or catalog gaps in this phase

**What goes wrong:** Stopping `persist()` or rewriting `docs/parity/catalog/chainstate.md` to claim disk-backed coins. [VERIFIED: D-21; catalog known gaps]

**How to avoid:** `persist()` still `save_snapshot`. Catalog gap stays. CACHE-01 is the overlay + apply contract, not durable per-outpoint coins.

## Code Examples

Verified patterns from pinned Knots `29.3.knots20260210` and current Open Bitcoin seams.

### FetchCoin (populate self only)

```cpp
// Source: packages/bitcoin-knots/src/coins.cpp:48-64
CCoinsMap::iterator CCoinsViewCache::FetchCoin(const COutPoint &outpoint) const {
    const auto [ret, inserted] = cacheCoins.try_emplace(outpoint);
    if (inserted) {
        if (auto coin{base->GetCoin(outpoint)}) {
            ret->second.coin = std::move(*coin);
            if (ret->second.coin.IsSpent()) {
                CCoinsCacheEntry::SetFresh(*ret, m_sentinel);
            }
        } else {
            cacheCoins.erase(ret);
            return cacheCoins.end();
        }
    }
    return ret;
}
```

Rust port: `fetch_coin(&mut self, outpoint)` uses `parent.get_coin` (peek). On parent miss, erase the tentative occupancy and return miss. On parent spent (should not happen for `MemoryCoinsView` / `Coin` live-output), treat as FRESH spent occupancy. Do not implement Knots' `const` + `mutable cacheCoins`; use `&mut self`.

### AddCoin FRESH rule (reorg re-add)

```cpp
// Source: packages/bitcoin-knots/src/coins.cpp:72-103
// If the coin exists in this cache as a spent coin and is DIRTY, then
// its spentness hasn't been flushed to the parent cache. We're
// re-adding the coin to this cache now but we can't mark it as FRESH.
fresh = !it->second.IsDirty();
it->second.coin = std::move(coin);
CCoinsCacheEntry::SetDirty(*it, m_sentinel);
if (fresh) CCoinsCacheEntry::SetFresh(*it, m_sentinel);
```

Engine connect uses `possible_overwrite = false` and keeps BIP30 `OutputOverwrite` on unspent HaveCoin. Do not port Knots `AddCoins` coinbase-always-overwrite. [VERIFIED: engine.rs:503-505; coins.cpp:119-127]

### SpendCoin FRESH erase vs DIRTY tombstone

```cpp
// Source: packages/bitcoin-knots/src/coins.cpp:130-149
if (it->second.IsFresh()) {
    cacheCoins.erase(it);
} else {
    CCoinsCacheEntry::SetDirty(*it, m_sentinel);
    it->second.coin.Clear();
}
```

Rust: FRESH → remove occupancy. Else → mark DIRTY and set entry spent (`maybe_coin = None`), keep occupancy. Engine `remove_spent_input` must return the live `Coin` for undo (Knots `moveout`). Missing unspent → `ChainstateError::MissingCoin`, not Knots' silent `false`. [VERIFIED: engine.rs:404-412]

### BatchWrite child → parent (commit algebra)

```cpp
// Source: packages/bitcoin-knots/src/coins.cpp:183-247
// Skip non-dirty.
// Parent miss + child FRESH+spent → ignore.
// Parent miss + otherwise → insert, DIRTY, copy FRESH from child.
// Parent hit + child FRESH + parent unspent → logic_error FRESH misapplied.
// Parent FRESH + child spent → erase parent.
// Else overwrite coin, SetDirty; do NOT mark parent FRESH.
```

Map `logic_error` to `ChainstateError::FreshFlagMisapplied { outpoint }`. Never `unwrap()`.

### HaveCoin vs HaveCoinInCache

```cpp
// Source: packages/bitcoin-knots/src/coins.cpp:163-171
bool CCoinsViewCache::HaveCoin(const COutPoint &outpoint) const {
    CCoinsMap::const_iterator it = FetchCoin(outpoint);
    return (it != cacheCoins.end() && !it->second.coin.IsSpent());
}
bool CCoinsViewCache::HaveCoinInCache(const COutPoint &outpoint) const {
    CCoinsMap::const_iterator it = cacheCoins.find(outpoint);
    return (it != cacheCoins.end() && !it->second.coin.IsSpent());
}
```

### SanityCheck valid mask

```cpp
// Source: packages/bitcoin-knots/src/coins.cpp:315-325
// attr: DIRTY=1, FRESH=2, spent=4
// invalid: 2 (FRESH-only unspent), 4 (spent clean), 7 (spent FRESH|DIRTY)
assert(attr != 2 && attr != 4 && attr != 7);
```

Valid: `0` unspent clean, `1` unspent DIRTY, `3` unspent FRESH\|DIRTY, `5` spent DIRTY, `6` spent FRESH-not-DIRTY. [VERIFIED: coins.h:101-106]

### Current forbidden apply clone (must delete)

```rust
// Source: packages/open-bitcoin-chainstate/src/engine.rs:158-200
let mut next_utxos = self.utxos.clone();
// ... mutate next_utxos ...
self.utxos = next_utxos;
```

```rust
// Source: packages/open-bitcoin-node/src/chainstate.rs:155-166
let mut chainstate = self.chainstate.clone();
let position = chainstate.connect_block_with_current_time(...)?;
Ok(PreparedChainstateConnect { chainstate, position })
```

### Two-phase commit window (must keep)

```rust
// Source: packages/open-bitcoin-node/src/network/mempool_lifecycle.rs:48-67
let prepared_chainstate = self.chainstate.prepare_connect_block(...)?;
let prepared_lifecycle = self.mempool.mempool().prepare_connected_block_transition(...)?;
self.commit_connected_block_lifecycle_transaction(..., prepared_chainstate, sealed_lifecycle)
```

Commit of chainstate happens inside `commit_prepared_mempool_transition_with` so both commit or neither. [VERIFIED: lifecycle_projection/authority.rs:544-565] Isolation: live cache unchanged until that closure runs `commit_prepared_connect`.

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Full UTXO `HashMap` + clone per connect | `CCoinsView` + `CCoinsViewCache` DIRTY/FRESH | Bitcoin Core coins cache (historical); Knots still this in 29.3 | Apply is incremental; persist is policy-driven |
| `GetCoin` on cache parent populates parent | Core peek/overlay work (not in pinned Knots 29.3) | Post-29.3 Core; D-15 locks peek for prepare | Failed prepare must not warm parent |
| `Flush` empties; `Sync` retains unspent | Same in Knots 29.3 | `coins.h` Flush/Sync | Phase 139 uses Flush for child commit; Sync is cache algebra only |
| Snapshot blob after every mutation | Still Open Bitcoin today | Until 141–142 | Phase 139 does not stop `persist()` |

**Deprecated/outdated:**

- Treating `Chainstate.utxos` as the apply target. [VERIFIED: D-01]
- `CCoinsViewBacked` as a required layer. [VERIFIED: D-06]
- Knots spent-null `Coin::Clear()` as the domain `Coin` model. [VERIFIED: D-10]
- Linked-list flagged-entry cursor. [VERIFIED: D-06]
- `AccessCoin` / `coinEmpty` sentinel. Use `Option<Coin>`. [VERIFIED: coins.cpp:152-160]

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | `ManagedPeerNetwork::clone()` for `reorg_to_branch` may rematerialize coins via `snapshot()`/`from_snapshot()` (D-22 leftover) without violating D-04/D-17, as long as `prepare_*` never clones. | Pitfall 7 | If discuss-phase intended *zero* full-map clones anywhere in node, reorg staging must be rewritten in this phase (larger plan). |
| A2 | Implementing `Sync` as in-memory cache algebra (retain unspent) in `coins/cache.rs` is in scope as the Knots pair of `Flush`, but must not be wired to disk or persist. | Pattern 1 | If planner treats Sync as Phase 140-only, skip the method and keep only Flush. |

**If this table is empty:** All claims in this research were verified or cited — no user confirmation needed.

These two items are planning-scope choices, not product-behavior forks. Default: accept A1 leftover snapshot clone for network staging; implement Flush+Sync algebra but call only Flush from prepare/commit.

## Open Questions

1. **`reorg_to_branch` network clone vs overlay-only preview**
   - What we know: `reorg_to_branch` clones `ManagedPeerNetwork`, installs a cloned preview `Chainstate`, applies mempool, then commits. [VERIFIED: mempool_lifecycle.rs:138-185]
   - What's unclear: Whether Phase 139 must eliminate that network-level snapshot rematerialization or only the prepare/apply clones (D-17/D-19).
   - Recommendation: Default A1 — keep staging compile via snapshot leftover; do not rewrite mempool reorg orchestration beyond `Prepared*` / `install_*` / `commit_*` API internals.

2. **`utxos()` return type**
   - What we know: Callers use `.len()`, `==`, and `contains_key`. Signature is `&HashMap`. [VERIFIED: engine tests; parity.rs; mempool_lifecycle.rs:351]
   - What's unclear: Discretion allows any view-backed helper.
   - Recommendation: Return owned unspent `HashMap`. Add `Chainstate::have_coin` and switch the mempool call site so occupancy cannot leak later.

## Environment Availability

Step 2.6: code/config-only phase. No new external services. Existing toolchain is the contract.

| Dependency | Required By | Available | Version | Fallback |
|------------|-------------|-----------|---------|----------|
| Rust / Cargo | crate edits | ✓ (do not run overlapping builds in this research) | 1.94.1 pin | — |
| Bun | breadcrumb `--write` | ✓ (repo-canonical) | `.bun-version` | — |
| Bitcoin Knots submodule | parity citations | ✓ files read this session | `29.3.knots20260210` | — |
| Fjall | not this phase | n/a | — | Must stay out of chainstate |
| New crates | none | n/a | — | Do not add |

**Missing dependencies with no fallback:** none.

**Missing dependencies with fallback:** none.

## Security Domain

`security_enforcement` is not `false` in `.planning/config.json`. Overlay algebra is consensus-adjacent integrity, not a new auth surface.

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V2 Authentication | no | — |
| V3 Session Management | no | — |
| V4 Access Control | no | — |
| V5 Input Validation | yes | Typed `OutPoint` / `Coin`; `possible_overwrite=false` keeps BIP30 `OutputOverwrite`; missing parent → `MissingCoin`, never silent spent |
| V6 Cryptography | no | No new hash/script crypto; reuse consensus crate |

### Known Threat Patterns for coins cache

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Parent resurrect of spent outpoint | Tampering | Occupancy hit on spent DIRTY answers HaveCoin=false; no parent fetch [VERIFIED: D-09] |
| FRESH misapply drops spentness | Tampering | Port BatchWrite + AddCoin re-add tests; typed `FreshFlagMisapplied` [VERIFIED: coins.cpp:214-219] |
| Failed prepare mutates tip | Tampering | Peek-only child; drop overlay on error [VERIFIED: D-14; D-15] |
| Cache hit claimed durable | Information disclosure / spoofing | D-23; persist still snapshot; no operator durability copy |
| I/O in core | Elevation / supply-chain | D-07; no Fjall/fs/clock; `check-pure-core-deps.sh` |

## Sources

### Primary (HIGH confidence)

- `packages/bitcoin-knots/src/coins.h` — `CCoinsView`, `CCoinsViewCache`, `CCoinsCacheEntry` DIRTY/FRESH, HaveCoin vs HaveCoinInCache, Flush/Sync/BatchWrite, SanityCheck comment
- `packages/bitcoin-knots/src/coins.cpp` — FetchCoin, AddCoin, SpendCoin, BatchWrite, Flush, Sync, HaveCoin, HaveCoinInCache, SanityCheck
- `packages/bitcoin-knots/src/validation.h` — `CoinsViews`, `CoinsTip`, `InitCoinsCache` (cite only; dual-chainstate out of scope)
- `packages/open-bitcoin-chainstate/src/engine.rs` — `utxos` HashMap, `next_utxos.clone()`, spend/add/restore helpers
- `packages/open-bitcoin-node/src/chainstate.rs` — prepare clone, persist snapshot
- `packages/open-bitcoin-node/src/network/mempool_lifecycle.rs` — prepare chainstate → prepare mempool → commit; `utxos().contains_key`
- `.planning/phases/139-coins-view-cache-contract-and-engine-apply/139-CONTEXT.md` — locked D-01–D-23
- `.planning/research/ARCHITECTURE.md` — Pattern 1, anti-patterns 1/3/5, build order 1–3
- `.planning/research/FEATURES.md` — Flush vs Sync, snapshot persist gap
- `.planning/research/PITFALLS.md` — occupancy ≠ durability, FRESH misapply
- `standards/core/architecture.md` — functional core; illegal states unrepresentable
- `docs/parity/catalog/chainstate.md` — existing claims; disk/manager gap stays
- `docs/parity/source-breadcrumbs.json` — `chainstate-engine` group
- `scripts/check-pure-core-deps.sh` — forbidden deps/imports
- `.planning/config.json` — `nyquist_validation: false`

### Secondary (MEDIUM confidence)

- `.planning/research/ARCHITECTURE.md` suggested `coins/flush.rs` — treat as Phase 140; do not add in 139
- Bitcoin Core post-29.3 peek/overlay intent — not present in pinned Knots sources; D-15 is the lock [CITED: 139-CONTEXT.md specifics; VERIFIED absence of PeekCoin in knots coins.*]

### Tertiary (LOW confidence)

- None used as implementation authority.

## Metadata

**Confidence breakdown:**

- Standard stack: HIGH — no new libraries; crate and toolchain pins verified
- Architecture: HIGH — Knots APIs and current Open Bitcoin seams read in this session; prepare borrow window verified in `connect_local_block`
- Pitfalls: HIGH — FRESH/occupancy/clone/atomicity issues are in pinned source and current engine/node code

**Research date:** 2026-08-30
**Valid until:** 2026-09-29 (stable; pinned Knots tag + locked CONTEXT)

## Planner implementation notes

Wave-oriented build order (research steps 1–3, merged as this phase):

1. **Cache contract** — `coins.rs` / `coins/cache.rs` / `coins/memory.rs` + error variants + breadcrumbs. Unit tests: four lookup facts, five valid states, invalid construction, AddCoin re-add, SpendCoin FRESH erase, BatchWrite FRESH-misapply / FRESH-parent-delete, child peek does not populate parent, Flush empties child.
2. **Engine apply** — Extract `engine/apply.rs`. `Chainstate` owns `CoinsCache<MemoryCoinsView>`. `from_snapshot` hydrates `MemoryCoinsView` with snapshot coins (cache starts empty; FetchCoin fills clean unspent). `snapshot()` / `utxos()` export unspent. `connect_block` / `disconnect_tip` / `reorg` apply on a local overlay and Flush on success. Same-crate struct-literal tests that set `utxos:` must use `from_snapshot` or a `pub(crate)` constructor.
3. **Manager prepare/commit** — `Prepared*` own overlay + staged undo/position/chain/counts. `prepare_*` stop `self.chainstate.clone()`. `commit_*` Flush overlay then install metadata, then existing `persist()`. Failed prepare drops overlay. `install_prepared_reorg_preview` must not `preview().clone()` of a live `CoinsCache`. Keep `connect_local_block` call order.

**Recommended flag encoding:** five-state enum (not bitflags), because D-11 + "make illegal states unrepresentable." Map to Knots attr bits in comments/tests.

**Recommended cursor:** omit production `CoinsViewCursor` (D-02 / discretion).

**Recommended `utxos()`:** owned unspent `HashMap` collected from parent + overlay.

**Exports from `lib.rs`:** `CoinsView`, `CoinsCache`, `MemoryCoinsView`, `CoinsCacheEntry`, plus existing `Chainstate` / snapshot types.

**Tests that construct `Chainstate { utxos, ... }`:** `disconnect_tip_skips_unspendable_outputs_and_reports_missing_created_out.rs` — rewrite to `from_snapshot`. [VERIFIED: that file lines 48-65]

**Do not run overlapping Cargo builds during planning/execution without the cooperative lock.** Verify later with `bun run scripts/command-timings.ts run --key <key> -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-chainstate` and node chainstate/network tests, then `bash scripts/verify.sh`.
