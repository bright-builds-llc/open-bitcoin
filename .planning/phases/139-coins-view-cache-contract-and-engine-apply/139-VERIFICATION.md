---
phase: 139-coins-view-cache-contract-and-engine-apply
verified: 2026-08-31T04:35:55Z
status: passed
score: 8/8 must-haves verified
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: 139-2026-08-30T15-42-10
generated_at: 2026-08-31T04:35:55Z
lifecycle_validated: true
overrides_applied: 0
---

# Phase 139: Coins-View, Cache Contract, and Engine Apply Verification Report

**Phase Goal:** Connect and disconnect mutate an in-memory DIRTY/FRESH coins overlay instead of cloning or rewriting the whole UTXO set.
**Verified:** 2026-08-31T04:35:55Z
**Status:** passed
**Re-verification:** No — initial verification

## Goal Achievement

CACHE-01 is the overlay + apply contract. `persist()` still writes the leftover snapshot blob (D-21). That leftover write is not treated as CACHE-01 completion, and it is not a Phase 139 gap.

### Observable Truths

| #   | Truth   | Status     | Evidence       |
| --- | ------- | ---------- | -------------- |
| 1   | Contributors can apply connect and disconnect against a typed `CoinsView` / `CoinsCache` overlay with DIRTY/FRESH transitions, without cloning the full UTXO map. | ✓ VERIFIED | `Chainstate` owns `coins: CoinsCache<MemoryCoinsView>`. `stage_connect_block_with_current_time` / `stage_disconnect_tip` / `stage_reorg` mutate a local `CoinsOverlay`; `apply_flushed` / `absorb_overlay` write dirty entries only. Production `engine.rs` / `apply.rs` have no `next_utxos` or `self.utxos.clone()`. |
| 2   | A cache hit is distinguishable from parent-view truth; HaveCoin and in-cache occupancy remain separate facts. | ✓ VERIFIED | `contains_in_cache` is overlay occupancy including tombstones; `have_coin_in_cache` is overlay unspent only; trait `have_coin`/`get_coin` return spent occupancy as false/None without parent consult. `four_lookup_facts_are_distinct_for_spent_dirty_tombstone` and `spent_dirty_tombstone_is_occupancy_hit_with_have_coin_false` assert the four facts. |
| 3   | Prepare/commit uses a child cache flushed into the parent cache, and a failed prepare leaves the parent view unchanged. | ✓ VERIFIED | Node `Prepared*` hold `StagedChainstateConnect` / `StagedChainstateReorg`. Prepare calls `stage_*` on `&self`. Commit absorbs Flush then leftover persist. Isolation tests assert failed prepare does not change tip, `utxos()`, or `have_coin_in_cache`, and does not FetchCoin into the live overlay. |
| 4   | The chainstate core remains I/O-free: no Fjall, filesystem, or clock appears in `open-bitcoin-chainstate`. | ✓ VERIFIED | Crate `Cargo.toml` depends only on `open-bitcoin-consensus` and `open-bitcoin-primitives`. `rg` over the crate finds no `fjall`, `std::fs`, `tokio`, `Instant`, or `std::time`. `bash scripts/check-pure-core-deps.sh` exits 0. |
| 5   | Illegal DIRTY/FRESH/spent combinations cannot be constructed as silent states. | ✓ VERIFIED | Five-state `CoinsCacheFlags` plus infallible constructors. `try_from_flags` returns `InvalidCacheEntry` for FRESH-only unspent, spent-clean, and spent FRESH\|DIRTY. |
| 6   | AddCoin / SpendCoin / BatchWrite / Flush / Sync follow the Knots DIRTY/FRESH algebra. | ✓ VERIFIED | `fresh = !is_dirty` on spent occupancy re-add; FRESH spend erases; `FreshFlagMisapplied` on child FRESH into live unspent parent; FRESH parent + child spent erases parent; Flush clears after `parent.batch_write`; Sync retains `UnspentClean`. Tests live in `coins/tests/algebra.rs`. |
| 7   | `from_snapshot` hydrates `MemoryCoinsView`; `snapshot()` / `utxos()` export unspent coins only. | ✓ VERIFIED | `from_snapshot` builds `CoinsCache::from_parent(MemoryCoinsView::from_coins(...))` with empty overlay. `collect_unspent` / `overlay_unspent_into` drop spent tombstones. |
| 8   | Commit Flushes dirty overlay entries, then leftover `persist()` still writes the snapshot blob; mempool uses `have_coin`; no durability claims. | ✓ VERIFIED | `commit_prepared_connect` absorbs then `self.store.save_snapshot(self.chainstate.snapshot())`. Mempool orphan check calls `have_coin`, not `utxos().contains_key`. No `durably persisted`, `IfNeeded`, or `FjallCoinsView` copy. |

**Score:** 8/8 truths verified

### Deferred Items

Leftover snapshot-blob persist is intentional D-21 leftover, not a failed Phase 139 truth.

| # | Item | Addressed In | Evidence |
|---|------|-------------|----------|
| 1 | Per-outpoint durable coins replace leftover snapshot-blob persist as live UTXO truth | Phase 141 | Goal: persist spendable UTXOs as per-outpoint Fjall records; leftover snapshot becomes non-authoritative |
| 2 | Restart tip/UTXO view comes from coins best-block, not leftover snapshot | Phase 142 | Success criteria: after same-datadir restart, tip and UTXO view come from durable coins best-block |

### Required Artifacts

| Artifact | Expected    | Status | Details |
| -------- | ----------- | ------ | ------- |
| `packages/open-bitcoin-chainstate/src/coins.rs` | CoinsView trait, CoinsBatch, five-state flags | ✓ VERIFIED | 139 lines; `pub trait CoinsView`, `CoinsCacheFlags`, `CoinsCacheEntry` constructors and `try_from_flags` |
| `packages/open-bitcoin-chainstate/src/coins/cache.rs` | Resident CoinsCache + CoinsOverlay algebra | ✓ VERIFIED | 425 lines; peek/fetch, add/spend, BatchWrite, Flush, Sync, absorb |
| `packages/open-bitcoin-chainstate/src/coins/memory.rs` | In-memory parent truth | ✓ VERIFIED | `MemoryCoinsView` stores unspent only; `batch_write` insert/delete |
| `packages/open-bitcoin-chainstate/src/coins/tests.rs` | Four-fact and five-state tests | ✓ VERIFIED | `spent_dirty_tombstone_is_occupancy_hit_with_have_coin_false` present |
| `packages/open-bitcoin-chainstate/src/coins/tests/algebra.rs` | Knots algebra tests | ✓ VERIFIED | `reorg_readd_of_dirty_spent_coin_is_not_fresh` lives here (plan-allowed split) |
| `packages/open-bitcoin-chainstate/src/engine/apply.rs` | Overlay spend/add/restore/context helpers | ✓ VERIFIED | Calls `overlay.spend_coin` / `add_coin` / `peek_coin`; no `HashMap::contains_key` |
| `packages/open-bitcoin-chainstate/src/engine.rs` | Cache-backed Chainstate + stage/commit | ✓ VERIFIED | `coins: CoinsCache<MemoryCoinsView>`; no Clone; 615 lines |
| `packages/open-bitcoin-chainstate/src/engine/stage.rs` | Absorb / reorg preview | ✓ VERIFIED | `absorb_staged_*` and `install_staged_reorg_preview` clone overlay metadata only |
| `packages/open-bitcoin-node/src/chainstate.rs` | Prepared overlay isolation + leftover persist | ✓ VERIFIED | `staged: StagedChainstateConnect`; no `self.chainstate.clone()` |
| `packages/open-bitcoin-node/src/network/mempool_lifecycle.rs` | have_coin orphan classification | ✓ VERIFIED | `have_coin(&input.previous_output).unwrap_or_default()` |

gsd-tools artifact/key-link pattern misses (not code gaps):

- Plan 02 `contains: reorg_readd_of_dirty_spent_coin_is_not_fresh` looked only in `coins/tests.rs`; the test is in `coins/tests/algebra.rs` as the plan allowed.
- Plan 03 `contains: fn spend_coin` looked for a local function name; apply helpers call `overlay.spend_coin`.
- Plan 04 persist pattern omitted `self.store.`; the live call is `self.store.save_snapshot(self.chainstate.snapshot())`.

### Key Link Verification

| From | To  | Via | Status | Details |
| ---- | --- | --- | ------ | ------- |
| `coins/cache.rs` | `coins.rs` | `impl<V: CoinsView> CoinsView for CoinsCache<V>` | ✓ WIRED | Peek-only trait reads; occupancy short-circuits parent |
| `lib.rs` | `coins.rs` | `pub use coins::` | ✓ WIRED | Re-exports view/cache types; `CoinsOverlay` via `coins::` |
| `coins/cache.rs` | `error.rs` | `FreshFlagMisapplied` | ✓ WIRED | `apply_child_write` returns the typed error |
| `coins/cache.rs` | `coins.rs` | `fn flush` → `parent.batch_write` then clear | ✓ WIRED | Flush empties; Sync retains unspent clean |
| `engine.rs` | `coins/cache.rs` | `commit_staged_connect` / `apply_flushed` | ✓ WIRED | Dirty batch into live cache only after success |
| `engine.rs` | `types.rs` | `MemoryCoinsView::from_coins` | ✓ WIRED | `from_snapshot` hydrates parent truth |
| `node/chainstate.rs` | `engine.rs` | `stage_connect_block_with_current_time` | ✓ WIRED | Prepare stages; commit absorbs |
| `authority.rs` | `node/chainstate.rs` | `commit_prepared_connect` | ✓ WIRED | Inside mempool transition closure |
| `node/chainstate.rs` | `MemoryChainstateStore::save_snapshot` | leftover snapshot persist | ✓ WIRED | `self.store.save_snapshot(self.chainstate.snapshot())` |

### Data-Flow Trace (Level 4)

These artifacts are domain/apply types, not UI. Trace is connect/prepare → overlay → Flush → live cache → leftover snapshot export.

| Artifact | Data Variable | Source | Produces Real Data | Status |
| -------- | ------------- | ------ | ------------------ | ------ |
| `CoinsCache` overlay | `overlay.entries` | `add_coin` / `spend_coin` / `fetch_coin` / `batch_write` | Yes — DIRTY/FRESH entries including spent tombstones | ✓ FLOWING |
| `Chainstate.coins` | `CoinsCache<MemoryCoinsView>` | `from_snapshot` parent + Flush/absorb | Yes — parent unspent plus flushed overlay | ✓ FLOWING |
| `StagedChainstateConnect.overlay` | detached dirty set | `apply_connect_on_overlay` | Yes — dropped on Err, flushed only on commit | ✓ FLOWING |
| `ManagedChainstate::persist` | `ChainstateSnapshot.utxos` | `snapshot()` / `collect_unspent` | Yes — rematerialized unspent map (leftover export, D-22) | ✓ FLOWING |

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
| -------- | ------- | ------ | ------ |
| Pure-core crate has no Fjall/fs/clock deps | `bash scripts/check-pure-core-deps.sh` | exit 0 | ✓ PASS |
| Production apply has no full-map clone | `rg next_utxos\|self.utxos.clone` in `engine.rs` / `apply.rs` | no matches | ✓ PASS |
| Child peek never calls `parent.fetch_coin` | `rg parent.fetch_coin` under `src/coins` | no matches | ✓ PASS |
| Prepare does not clone live Chainstate | `rg self.chainstate.clone` under `open-bitcoin-node` | no matches | ✓ PASS |
| Persist leftover snapshot blob still exists | persist body in `node/chainstate.rs:249-251` | `save_snapshot(self.chainstate.snapshot())` | ✓ PASS |

Step 7b cargo suites were not re-run here (spot-check budget; hooks already compiled this tree during plan commits). Isolation tests exist with Arrange/Act/Assert and assert live occupancy, tip, and `have_coin_in_cache`.

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
| ----------- | ---------- | ----------- | ------ | -------- |
| CACHE-01 | 139-01, 139-02, 139-03, 139-04 | Node overlays an in-memory DIRTY/FRESH coins cache so connect and disconnect do not persist the whole UTXO set after every block | ✓ SATISFIED | Overlay + apply contract is live: connect/disconnect/reorg/prepare mutate a child overlay and Flush on success. CACHE-01 is not claimed via leftover snapshot persist. `persist()` still `save_snapshot` per D-21 until Phases 141–142. |

No orphaned Phase 139 requirement IDs. REQUIREMENTS.md maps only CACHE-01 to this phase. FLUSH-01, COIN-01, MGR-*, HAVL-*, CSOBS-*, and CSVFY-* belong to later phases.

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
| ---- | ---- | ------- | -------- | ------ |
| `packages/open-bitcoin-chainstate/src/engine/tests.rs` | 45–75 | Test adapters still named `next_utxos` | ℹ️ Info | Test-only HashMap wrappers around overlay helpers. Production apply does not clone a live map. |
| `packages/open-bitcoin-chainstate/src/engine/tests/apply_non_coinbase_transaction_returns_fee_and_records_undo.rs` | 536–551 | `connect_block_does_not_clone_utxo_map` asserts export length, not a clone ban | ℹ️ Info | Weak name vs assertion; production `connect_block_with_current_time` is stage-then-commit with no clone. |
| `packages/open-bitcoin-node/src/chainstate.rs` | 249–251 | Leftover snapshot persist | ℹ️ Info | Intentional D-21 leftover. Not CACHE-01 completion and not a Phase 139 gap. |

No TODO/FIXME/placeholder stubs in the coins, engine apply, or node prepare surfaces. `CoinsCache` and `Chainstate` are not `Clone`. `CoinsOverlay` Clone is the allowed dirty-set copy for reorg preview.

### Human Verification Required

None. This phase is a pure-core overlay/apply contract plus node prepare isolation. Observable truths are checkable from types, wiring, and existing unit tests.

### Gaps Summary

No goal-blocking gaps. Tool pattern misses on split test files and `overlay.spend_coin` / `self.store.save_snapshot` do not change the contract.

CACHE-01 is satisfied as the overlay + apply contract. Do not treat leftover snapshot persist as the coins write that completes CACHE-01; D-21 left that blob in place until later durability phases.

---

_Verified: 2026-08-31T04:35:55Z_
_Verifier: Claude (gsd-verifier)_
