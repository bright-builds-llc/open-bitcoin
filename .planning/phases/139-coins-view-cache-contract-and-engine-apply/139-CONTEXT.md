---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 139-2026-08-30T15-42-10
generated_at: 2026-08-30T15:44:48.589Z
---

# Phase 139: Coins-View, Cache Contract, and Engine Apply - Context

**Gathered:** 2026-08-30
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Connect and disconnect mutate an in-memory DIRTY/FRESH coins overlay instead
of cloning or rewriting the whole UTXO set.

This phase delivers CACHE-01 only: a typed `CoinsView` / `CoinsCache` overlay,
distinguishable cache-hit vs parent-view vs HaveCoin facts, and
prepare/commit via a child cache flushed into the parent cache. A failed
prepare leaves the parent view unchanged. `open-bitcoin-chainstate` stays
I/O-free.

This phase does not persist per-outpoint coins, decide IfNeeded/Periodic/Always
flush policy, own manager init/restart, change serving honesty, or claim
prune/archive/assumeutxo/production readiness. Node `persist()` may still write
today's snapshot blob until Phases 141–142 cut over durable coins truth.

</domain>

<decisions>
## Implementation Decisions

### View/cache layering contract

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

### DIRTY/FRESH occupancy vs HaveCoin

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

### Prepare/commit without cloning

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

### Existing snapshot/HashMap compatibility

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

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase and requirement
- `.planning/ROADMAP.md` — Phase 139 goal, CACHE-01, success criteria, and
  the locked merge of view/cache plus engine apply
- `.planning/REQUIREMENTS.md` — CACHE-01 wording and later-phase ownership
  for FLUSH, COIN, MGR, HAVL, CSOBS, CSVFY
- `.planning/PROJECT.md` — Functional-core / no rust-bitcoin / Fjall-in-shell
  constraints and v2.3 storage-first scope

### Milestone research
- `.planning/research/ARCHITECTURE.md` — Pattern 1 layered coins view,
  Anti-Patterns 1/3/5, suggested build order steps 1–3, `CoinsView` sketch
- `.planning/research/FEATURES.md` — Knots `CCoinsViewCache` DIRTY/FRESH,
  Flush vs Sync, and the current full-snapshot persist gap
- `.planning/research/PITFALLS.md` — Cache occupancy ≠ durability, FRESH
  misapply on reorg re-add, and "do not keep snapshot as live truth" (the
  last is a later-phase cutover, not a Phase 139 persist stop)

### Existing apply surface
- `packages/open-bitcoin-chainstate/src/engine.rs` — `Chainstate` still owns
  `utxos: HashMap` and clones it in `connect_block`
- `packages/open-bitcoin-chainstate/src/types.rs` — `Coin` and
  `ChainstateSnapshot` helper shapes
- `packages/open-bitcoin-node/src/chainstate.rs` — `prepare_connect_block` /
  `prepare_reorg` clone `self.chainstate`; `persist()` still
  `save_snapshot`

### Knots anchors
- `packages/bitcoin-knots/src/coins.h` — `CCoinsView`, `CCoinsViewCache`,
  `CCoinsCacheEntry` DIRTY/FRESH, `HaveCoin` vs `HaveCoinInCache`, Flush /
  Sync / BatchWrite
- `packages/bitcoin-knots/src/coins.cpp` — FetchCoin, AddCoin, SpendCoin,
  FRESH spend-erase and dirty-spent re-add
- `packages/bitcoin-knots/src/validation.h` — `CoinsViews`, `CoinsTip`,
  `InitCoinsCache` (cite only; dual-chainstate and flush policy stay out)

### Architecture standards
- `standards/core/architecture.md` — Functional core / imperative shell
- `.planning/ARCHITECTURE.md` — Crate boundaries; chainstate stays pure
- `docs/parity/catalog/chainstate.md` — Existing connect/disconnect claims
  and the known disk-backed / manager gap (do not close the disk gap here)

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `Coin`, `TxUndo`, `BlockUndo`, `ChainPosition` — keep domain meaning;
  spentness does not move onto `Coin`
- `ChainstateError` — extend with typed view/cache failures; do not map a
  missing parent coin to a silent spent
- `ChainstateSnapshot` — remains the hydrate/export DTO
- `PreparedChainstateConnect` / `PreparedChainstateReorg` — keep the
  two-phase manager API; change the isolation mechanism underneath
- `MemoryChainstateStore` — still snapshot-backed in this phase

### Established Patterns
- Functional-core engine with snapshot/clone isolation — replace the clone
  with a child cache, keep the prepare/commit call shape
- Mempool already uses prepare-then-commit patches; coins isolation must
  remain compatible with `connect_local_block`'s prepare-chainstate →
  prepare-mempool → commit-both sequence
- Parity breadcrumbs are required on new first-party Rust sources
- Architecture-policy checks fail if Fjall or clocks enter
  `open-bitcoin-chainstate`

### Integration Points
- `Chainstate::connect_block` / `disconnect_tip` / `reorg` become
  view/cache apply
- `ManagedChainstate::prepare_*` / `commit_*` stop cloning `Chainstate`
- `ManagedChainstate::persist` stays snapshot-blob until later phases
- Engine unit tests hydrate through `from_snapshot` and may still assert
  `utxos()`
- `DurableSyncRuntime`, wallet rescan, and RPC snapshot consumers are
  out of scope except that they must keep compiling against the helper DTO

</code_context>

<specifics>
## Specific Ideas

- Match pinned Knots `CCoinsView` / `CCoinsViewCache` / `HaveCoin` vs
  `HaveCoinInCache` rather than inventing a smaller overlay
- Child prepare isolation should follow the intent of Core's peek/overlay
  work: reads during prepare must not populate the parent
- FRESH-flag comments in Knots `AddCoin` / `SpendCoin` become unit tests,
  not prose-only breadcrumbs

</specifics>

<deferred>
## Deferred Ideas

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

</deferred>

---

*Phase: 139-coins-view-cache-contract-and-engine-apply*
*Context gathered: 2026-08-30*
