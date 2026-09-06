# Phase 142: Manager Flush Lifecycle and Restart - Research

**Researched:** 2026-09-06
**Domain:** Chainstate manager flush lifecycle, ordered persist, interrupted-flush replay, same-datadir restart
**Confidence:** HIGH

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
### Crash-loss window
- **D-01:** Replace every-connect leftover snapshot persist with Knots-shaped
  IfNeeded / Periodic / Always. In-process memory stays current between
  flushes. A crash may lose unflushed cache — that is the allowed window.
- **D-02:** Do not keep silent every-connect snapshot dumps as the durability
  path. Do not flush coins after every connect just to avoid the window.
- **D-03:** `FlushMode::Always` on clean shutdown is mandatory so a clean
  restart should not lose the in-memory working set.

### Cache-byte defaults and accounting
- **D-04:** Keep 140 D-06–D-10: core `decide_flush` stays unpinned. The
  shell injects `cache_bytes`, `cache_byte_limit`, `mempool_leftover_bytes`,
  `now`, already-jittered `next_write`, `memory_pressure`, disk-free bytes,
  and entry count.
- **D-05:** Adapter defaults for those injected limits follow Knots-shaped
  first-party values: 450 MiB default kernel cache, 4 MiB min dbcache, 8 MiB
  coins-DB cap. Occupancy accounting stays first-party estimated bytes, not
  Fjall item count or LevelDB `SizeEstimate`.
- **D-06:** After a successful Periodic write, the shell resamples 50–70
  minute jitter and injects the new `next_write`. Do not sample RNG or
  clocks in `open-bitcoin-chainstate`.

### Interrupted-flush recovery
- **D-07:** Two-element `H` plus missing `B` is an interrupted flush. When
  stored undo and block bodies exist for the interrupted window, replay:
  roll the old branch back and the new branch forward (`ReplayBlocks`
  shape). Then finish or clear markers so reopen is consistent.
- **D-08:** If undo or block bodies are missing, fail closed as a typed
  interrupted-write / recovery error. Do not invent a consistent tip, do
  not silently drop `H`, and do not auto-reindex or destructive-repair
  (FUT-23 stays out of scope).
- **D-09:** Any other `H` count, mismatched `H[0]`, or decode failure stays
  inconsistent and fail-closed (141 D-10). Empty `H` plus present `B` is
  consistent and does not replay.
- **D-10:** Classify interrupted `H` before the schema-2 leftover-plus-empty
  check (141-REVIEW WR-02). A crash after seed writes `H` but before the
  first `C`/`B` must surface as interrupted flush, not leftover-empty
  corruption. Remap interrupted-write by typed error, not Display text
  (141-REVIEW IN-01).

### Manager ownership and CanFlush
- **D-11:** One manager owns the single active chainstate sequence: coins
  database init → marker health-check / `decide_recovery` → optional replay
  or fail-closed → cache init → CanFlush-style readiness. Do not report
  ready-to-flush before that sequence completes.
- **D-12:** Dual snapshot/IBD chainstate and assumeutxo cache split stay
  out of scope. `MemoryChainstateStore` remains the in-memory test parent.
- **D-13:** Production live cache parent is `FjallCoinsView` (or a thin
  manager-owned wrapper around it). `Chainstate` must stop using
  `MemoryCoinsView` as the only production parent. `open-bitcoin-chainstate`
  stays I/O-free: the Fjall parent lives in `open-bitcoin-node`.

### Ordered flush and persist cutover
- **D-14:** One shell-owned flush entry point executes `decide_flush` and
  then writes in Knots `FlushStateToDisk` order: block and undo files,
  then block index, then coins (`Flush` or `Sync` as the decision says).
  Do not re-choose Flush vs Sync in the manager (140 D-11).
- **D-15:** If block, undo, or index flush fails, abort before coins write
  and before advancing coins best-block or progress credit. Failed coins
  writes fail closed and stop progress credit.
- **D-16:** After connect/reorg, call `FlushMode::IfNeeded`. On injected
  ticks, call `FlushMode::Periodic`. On clean shutdown, call
  `FlushMode::Always`. `FlushMode::None` stays observability-only.
- **D-17:** Cut over `ManagedChainstate::persist` and
  `DurableSyncRuntime::persist_progress` off leftover snapshot *writes* as
  live truth. Progress credit and restart tip come from durable coins
  `B` / coins best-block. `ChainstateSnapshot` may remain a hydrate /
  export / test helper; a leftover `"snapshot"` blob must not hydrate the
  live UTXO view (141 D-15 / D-18).
- **D-18:** Flip the Phase 140/141 leftover-write guard tests when the
  cutover lands. Those guards exist to fail this phase if snapshot dumps
  remain the durability path.

### Claude's Discretion
- Exact manager type/module names (`ManagedChainstate` extension vs a
  dedicated flush-lifecycle type) as long as one owner implements D-11.
- How `Chainstate` becomes generic over a `CoinsView` parent without
  pulling Fjall into the core crate.
- Exact `ReplayBlocks` helper shape and crash-simulation fixture style,
  provided D-07/D-08 hold.
- Whether leftover snapshot *files* are deleted, left unread, or only
  stopped being written — as long as they are not live UTXO truth.
- Internal typed facts the later Phase 144 operator surfaces can project;
  do not build those surfaces here.

### Deferred Ideas (OUT OF SCOPE)
- Honest payload-present availability and reserved `Pruned` label —
  Phase 143
- Status / RPC / CLI / dashboard / metrics / logs / support flush
  evidence — Phase 144
- Parity-root closeout and no-claim guardrails — Phase 145
- Prune/archive product modes, assumeutxo, dual-chainstate, LevelDB
  import, automatic destructive reindex, public defaults, production
  readiness — FUT-18 through FUT-26

None of these were folded into Phase 142.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| MGR-01 | One manager owns coins-database init, health-check, cache init, and CanFlush-style readiness for the single active chainstate. | Knots `CompleteChainstateInitialization` order (InitCoinsDB → ReplayBlocks → InitCoinsCache → `CanFlushToDisk`). Implement as one node-shell owner around `decide_recovery` + cache attach. |
| MGR-02 | After restart, tip and UTXO view come from durable coins best-block, not a leftover snapshot blob. | Cut `DurableSyncRuntime::open` off `hydrate`→`MemoryCoinsView` full-map load; attach `FjallCoinsView` + empty cache; tip from `B`. Stop leftover writes in `persist` / `persist_progress`. |
| FLUSH-02 | A mid-flush crash is recovered by interrupted-flush replay using stored undo and block bodies, or fails closed without inventing a consistent tip. | Make two-element `H` observable (today it fail-closes at store open). Replay Knots `ReplayBlocks` shape when undo+bodies exist; else typed `InterruptedWrite`. |
</phase_requirements>

## Summary

Phase 142 is a wiring and cutover phase, not a new storage engine. Policy (`decide_flush` / `decide_recovery`), cache Flush/Sync algebra, and the Fjall two-phase `H`/`B` parent already exist. The live node still dumps a leftover `ChainstateSnapshot` after every mutation, hydrates production apply through `MemoryCoinsView`, and fail-closes interrupted `H` at `FjallNodeStore::open` before a manager can replay.

The planner must deliver one shell-owned manager that (1) opens coins, classifies markers, replays or fails closed, then inits an empty cache over `FjallCoinsView` and reports CanFlush; (2) executes `decide_flush` at IfNeeded / Periodic / Always call sites in Knots block→undo→index→coins order; (3) stops leftover snapshot *writes* as live truth so restart and progress credit follow coins `B`.

**Primary recommendation:** Extend `ManagedChainstate` with a `chainstate/flush_lifecycle.rs` owner; make `Chainstate<V: CoinsView>` so production uses `FjallCoinsView` without Fjall entering the core crate; change interrupted `H` from an open-time hard fail into an observable marker the manager consumes.

## Project Constraints (from .cursor/rules/)

`.cursor/rules/` does not exist in this repository. Enforce the Bright Builds and repo-local sources instead:

- Functional core / imperative shell: no Fjall, `std::fs`, Tokio, `SystemTime`, or RNG in `open-bitcoin-chainstate`. [VERIFIED: `standards/core/architecture.md`, `scripts/check-pure-core-deps.sh`, `.planning/ARCHITECTURE.md`]
- `foo.rs` plus `foo/` for new multi-file modules. [VERIFIED: `standards/languages/rust.md`]
- Production-file length gate is **628** physical lines (`FILE_LINE_LIMIT`). [VERIFIED: `scripts/bright-builds-check.ts`]
- New first-party Rust sources need parity breadcrumbs via `docs/parity/source-breadcrumbs.json`. [VERIFIED: `AGENTS.md`]
- No rust-bitcoin, LevelDB, new production crates, or Fjall bump. [VERIFIED: `.planning/PROJECT.md`, `.planning/research/STACK.md`]
- `standards-overrides.md` has no active local exceptions (placeholder table only). [VERIFIED: `standards-overrides.md`]

## Standard Stack

### Core

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| Rust | `1.94.1` / edition 2024 | Typed manager, generic `Chainstate<V>`, replay apply | Pinned by `rust-toolchain.toml`; local `rustc`/`cargo` match. [VERIFIED: local `rustc --version`] |
| `open-bitcoin-chainstate` | workspace `0.1.0` | `decide_flush`, `decide_recovery`, `CoinsCache` Flush/Sync | Policy and cache algebra already shipped in Phases 139–140. [VERIFIED: `packages/open-bitcoin-chainstate/src/coins/flush.rs`] |
| `open-bitcoin-node` | workspace `0.1.0` | Manager, Fjall parent, ordered flush I/O | Shell owns persist execution. [VERIFIED: `.planning/ARCHITECTURE.md`] |
| Fjall | `3.1.4`, `default-features = false` | Coins `C`/`B`/`H`, undo, headers, block payloads | Existing pin; do not bump. [VERIFIED: `packages/open-bitcoin-node/Cargo.toml`] |
| Bitcoin Knots | `29.3.knots20260210` | `FlushStateToDisk`, `ReplayBlocks`, `CanFlushToDisk` | Behavioral baseline. [VERIFIED: GitHub tag `v29.3.knots20260210`; local `packages/bitcoin-knots/src/validation.cpp` present] |

### Supporting

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `getrandom` | `0.3.4` | 50–70 minute Periodic jitter | Sample in node or `open-bitcoind` shell only. Already a node + rpc dep. [VERIFIED: `packages/open-bitcoin-node/Cargo.toml`] |
| Tokio | `1.52.1` (rpc only) | Periodic flush wakeup | Drive ticks from `open-bitcoind`, next to the mempool checkpoint worker. Do not add Tokio to node or chainstate. [VERIFIED: `.planning/research/STACK.md`, `scripts/check-pure-core-deps.sh`] |
| Existing `PersistMode` | node crate | Fjall durability of a coins final-`B` batch | `Flush` for Periodic/IfNeeded journal-to-OS; `Sync` for Always / marker finish. Distinct from cache Flush vs Sync. [VERIFIED: 141 D-08, `coins_view.rs`] |
| serde / serde_json | existing | Leftover snapshot decode only | Never encode live UTXO truth. [VERIFIED: 141 D-12 / D-17] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| `Chainstate<V: CoinsView>` | `Box<dyn CoinsView>` | Trait object avoids generic bleed into `ManagedPeerNetwork`, but loses static Flush/Sync and clones poorly. Prefer generics. |
| Replay apply-only (Knots `RollforwardBlock`) | Re-run `connect_block` | Full connect re-validates a mixed coins set and can fail; Knots replay is spend/add with overwrite. Use apply-only. |
| Leave leftover files unread | Delete leftover `"snapshot"` | Deletion is optional (discretion). Stopping writes + ignoring on hydrate is enough for MGR-02. |

**Installation:** No new Cargo packages.

```bash
# No dependency changes. Materialize Knots only if a planner/executor needs local C++ line reads.
git submodule update --init --recursive
```

**Version verification:** Rust `1.94.1` (2026-03-25), Fjall `3.1.4` from `packages/open-bitcoin-node/Cargo.toml` on 2026-09-06. Do not bump Fjall.

## Architecture Patterns

### Recommended Project Structure

```
packages/open-bitcoin-chainstate/src/
├── coins.rs                 # Keep CoinsView; export replay types if added
├── coins/flush.rs           # Unchanged policy; do NOT put ReplayBlocks here
├── coins/cache.rs           # Add entry_count + estimated_cache_bytes
├── engine.rs                # Chainstate<V: CoinsView>; extract if near 628
└── engine/apply.rs          # Already generic over V: CoinsView

packages/open-bitcoin-node/src/
├── chainstate.rs            # ManagedChainstate owner; persist becomes IfNeeded
├── chainstate/flush_lifecycle.rs   # NEW: CanFlush, decide_flush exec, ordered write
├── chainstate/replay.rs     # NEW: load undo/bodies; apply-only rollback/rollforward
├── storage/coins_view.rs    # Observable Interrupted marker; typed error remap
├── storage/fjall_store/coins.rs    # H-before-leftover-empty; stop fail-at-open
├── sync.rs                  # Open attaches Fjall parent (extract if near 628)
└── sync/runtime_state.rs    # persist_progress cutover (already 625 lines)

packages/open-bitcoin-rpc/src/bin/open_bitcoind/
├── checkpoint.rs            # Pattern to copy: Periodic tick + Always on shutdown
└── coins_flush.rs           # NEW thin worker: Periodic ticks + Always then mark_clean_shutdown
```

### Pattern 1: Knots init / CanFlush sequence

**What:** One owner runs coins DB open → marker health-check → replay-or-fail-closed → cache init → ready-to-flush. [CITED: `https://github.com/bitcoinknots/bitcoin/blob/v29.3.knots20260210/src/node/chainstate.cpp` `CompleteChainstateInitialization`]

**When to use:** `DurableSyncRuntime::open_with_runtime_activation` and any production `ManagedChainstate` construction over Fjall.

**Knots order (single active chainstate only):**

1. `InitCoinsDB`
2. `ReplayBlocks` (no-op when `H` empty)
3. `InitCoinsCache`
4. `assert(CanFlushToDisk())` — `m_coins_views && m_coins_views->m_cacheview`
5. `LoadChainTip` from coins best-block when `B` is not null

**Open Bitcoin mapping:**

```
FjallNodeStore::open (schema 2, do not hard-fail interrupted H)
  → inspect markers → decide_recovery(count)
  → InterruptedTwoHeads: load undo+bodies or StorageError::InterruptedWrite
  → replay on a temporary CoinsCache<FjallCoinsView>, then Flush, clear H, write B
  → CoinsCache::from_parent(FjallCoinsView)   // empty overlay; do not probe B (141)
  → ManagerReadiness::ReadyToFlush
```

Do not report CanFlush, do not call `decide_flush`, and do not credit progress before `ReadyToFlush`.

### Pattern 2: One flush entry point, policy-selected write-kind

**What:** Shell builds `FlushPolicyInput` from injected facts, calls `decide_flush`, then executes the returned write-kind. [VERIFIED: `packages/open-bitcoin-chainstate/src/coins/flush.rs`]

**When to use:** After connect/reorg (`IfNeeded`), on daemon ticks (`Periodic`), on clean shutdown (`Always`).

```rust
// Source: packages/open-bitcoin-chainstate/src/coins/flush.rs
match decide_flush(input) {
    FlushDecision::None(_) => { /* observability only */ }
    FlushDecision::RefuseDiskSpace(_) => { /* fail closed; no coins write */ }
    FlushDecision::Flush(_) => ordered_write_then(cache.flush())?,
    FlushDecision::Sync(_) => ordered_write_then(cache.sync())?,
}
```

Do not re-derive Flush vs Sync in the manager (140 D-11 / 142 D-14). Cache `Flush` empties; `Sync` retains unspent. [VERIFIED: `CoinsCache::flush` / `sync` in `cache.rs`]

**Two different Flush/Sync vocabularies:**

| Vocabulary | Meaning | Owner |
|------------|---------|--------|
| `FlushDecision::Flush` / `Sync` | Empty cache vs keep unspent | `decide_flush` → `CoinsCache` |
| `PersistMode::Flush` / `Sync` | Fjall journal-to-OS vs `SyncAll` | Adapter final-`B` batch |

`FjallCoinsView::persist_mode_for_final_best_block()` is hardcoded `PersistMode::Flush` today. [VERIFIED: `coins_view.rs`] Pass `PersistMode::Sync` for Always / marker-finish (141 D-08). That is not re-choosing cache write-kind.

### Pattern 3: Ordered persist before coins `B` advances

**What:** Knots `FlushStateToDisk` writes block+undo, then block index, then coins. Failed index write is fatal before coins. [CITED: Knots `validation.cpp` `FlushStateToDisk` around the `write block and undo` / `write block index` / `CoinsTip().Flush|Sync` sequence]

**Open Bitcoin mapping (one function):**

1. Ensure block payloads for the flush window are durable (`save_block` / `load_block`). Connect already calls `save_block` in `record_block_disposition`. [VERIFIED: `sync/block_response.rs`]
2. Persist undo records (`save_undo`) for that window.
3. Persist headers + block-index projection (`save_header_entries`).
4. Only then `CoinsCache::flush` or `sync` (advances `H`/`B`).

If step 1–3 fails: abort, do not write coins, do not advance `B`, do not credit progress (D-15).

### Pattern 4: Observable interrupted `H`, then replay-or-fail-closed

**What:** Two-element `H` plus missing `B` is interrupted. Knots `ReplayBlocks` rolls the old branch back (`DisconnectBlock`, idempotent) and the new branch forward (`RollforwardBlock`: SpendCoin + `AddCoins(..., check=true)`), then `SetBestBlock` + `Flush`. [CITED: Knots `validation.cpp` `ReplayBlocks` / `RollforwardBlock`]

**Critical current-code gap:** `FjallCoinsView::classify_markers` turns `(2, None)` into `StorageError::InterruptedWrite`. `head_blocks()` never returns the two hashes. `FjallNodeStore::open` → `ensure_schema_two` → `head_blocks()` fail-closes before a manager exists. [VERIFIED: `coins_view.rs`, `fjall_store/coins.rs`, `fjall_store.rs`]

**Do this instead:**

1. `classify_markers` returns a first-class `Interrupted { new, old }` state (plus existing Empty / Consistent).
2. `head_blocks()` returns those two hashes so `decide_recovery(2)` is `InterruptedTwoHeads`.
3. Store open succeeds; leftover-empty still runs **after** interrupted classification and **must skip** when `H` is two-element (preserve WR-02).
4. Manager loads undo + block bodies for the old→fork and fork→new walks. Missing any required body/undo → `StorageError::InterruptedWrite` (do not drop `H`).
5. Replay on a **temporary** cache over `FjallCoinsView` (Knots uses a cache over `CoinsDB` before `InitCoinsCache`). Apply-only; do not call full `connect_block`.
6. Flush replay cache, erase `H`, write `B = new`. Then init the live empty cache.

`H[1] == [0;32]` is the first-flush case (Fjall already uses that when `B` was missing). [VERIFIED: `coins_view.rs` `old_tip` fallback] No rollback; rollforward only if bodies exist.

`decide_recovery` stays count-only. A Phase 140 test forbids `ReplayBlocks` / `BlockHash` in `flush.rs`. [VERIFIED: `coins/tests/flush.rs` `decide_recovery_does_not_mention_block_hash_or_replay_in_source`] Put replay I/O and hash walks in the node crate.

### Pattern 5: Generic `Chainstate` without Fjall in core

**What:** Production parent is `FjallCoinsView` in the node crate. Core stays I/O-free. [VERIFIED: `open-bitcoin-chainstate/Cargo.toml` has only consensus + primitives]

**Do this:**

```rust
pub struct Chainstate<V: CoinsView> {
    active_chain: Vec<ChainPosition>,
    coins: CoinsCache<V>,
    undo_by_block: HashMap<BlockHash, BlockUndo>,
    maybe_confirmed_txid_counts: Option<HashMap<Txid, u32>>,
}

pub type MemoryBackedChainstate = Chainstate<MemoryCoinsView>;
```

- `apply.rs` is already `apply_connect_transactions<V: CoinsView>`. [VERIFIED: `engine/apply.rs`]
- `engine.rs` still hardcodes `CoinsCache<MemoryCoinsView>` on `Chainstate` and `apply_*_on_overlay`. [VERIFIED: `engine.rs` lines 40, 382, 441]
- `from_snapshot` stays a `MemoryCoinsView` constructor for tests/export.
- Production constructor: `Chainstate::from_parent(fjall_view, active_chain, undo_map, counts)` with empty cache overlay (141: `from_parent` does not probe `best_block`).
- Tests keep `MemoryChainstateStore` + `MemoryBackedChainstate` (D-12).

**Pitfall:** `ManagedChainstate<S: Clone>` clones by `Chainstate::from_snapshot(...)`, which rebuilds a `MemoryCoinsView`. Production must not Clone through that path or Fjall parent is discarded. [VERIFIED: `chainstate.rs` `Clone` impl]

### Anti-Patterns to Avoid

- **Flush coins after every `persist_progress`:** `should_persist_progress` fires on every extended active-chain connect. [VERIFIED: `sync/types.rs`] Putting coins `Always`/`Flush` there reintroduces every-connect dumps (D-02). Headers/runtime may persist; coins only via `decide_flush`.
- **Hydrate production by `scan_coin_records()` into `MemoryCoinsView`:** That reloads the full UTXO set and keeps Memory as the live parent (violates D-13 / MGR-02). `hydrate_chainstate_for_open` may remain a test/export helper.
- **Fail-at-open on interrupted `H`:** Blocks FLUSH-02. Make the marker observable, then replay or fail closed in the manager.
- **Remap via `detail.contains("interrupted write")`:** Still live in `map_heads_error`. [VERIFIED: `fjall_store/coins.rs`] Carry `StorageError::InterruptedWrite` or add `ChainstateError::InterruptedWrite { heads }` (IN-01 / D-10).
- **Replay with `connect_block`:** Mixed mid-flush coins are not a consistent old tip. Use Knots apply-only overwrite.
- **Put ReplayBlocks in `flush.rs`:** Forbidden by existing source test; also would pull hashes into the count-only sketch.
- **Hold `ManagedNetworkHandle` across Fjall:** Capture flush facts, persist, then receipt (PITFALLS.md).
- **Credit progress from in-memory tip:** `connected_block()` reads live chain tip. [VERIFIED: `runtime_state.rs`] Credit only after coins `B` advances.
- **Prune unlink / assumeutxo / dual chainstate / LevelDB:** Out of scope.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Flush vs Sync / LARGE / disk refusal | New policy predicates | `decide_flush` | Already locked 140 D-11–D-18. |
| Marker count classification | Ad-hoc `H.len()` matches in three places | `decide_recovery` plus richer shell checks | Count-only core; hashes/bodies in shell. |
| Cache empty vs retain | Custom clear flags | `CoinsCache::flush` / `sync` | Algebra already unit-tested. |
| Two-phase coins persist | Loop of `put_bytes` | `FjallCoinsView::batch_write` | `H` then coins then `B`, 64 MiB batches, crash seam. |
| Undo / block I/O | New keyspaces | `save_undo` / `load_undo`, `save_block` / `load_block` | Already durable records. |
| Periodic jitter | `rand` in chainstate | `getrandom` in shell | Core forbids `rand` (`check-pure-core-deps.sh`). |
| Interrupted operator mapping | Display-string match | `StorageError::InterruptedWrite` | `classify_recovery` already matches the typed variant. |
| Periodic + shutdown worker | New scheduler crate | Copy `open_bitcoind/checkpoint.rs` thread+channel pattern | Existing daemon ownership. |

**Key insight:** The missing work is ownership and cutover, not new persistence primitives. Reimplementing flush policy or `H`/`B` will fork truth.

## Runtime State Inventory

This phase migrates live coin truth off leftover snapshot writes.

| Category | Items Found | Action Required |
|----------|-------------|------------------|
| Stored data | Schema-2 Fjall datadirs: leftover `chainstate/"snapshot"` blob; coins `C`/`B`/`H`; `undo:{hex}`; `chain_meta`. [VERIFIED: `fjall_store/coins.rs`] | Code: stop leftover *writes*. Reopen from `B` + `FjallCoinsView`. Leftover files may remain unread (discretion). No silent remigrate (schema-2 leftover+empty stays fail-closed when `H` is not interrupted). |
| Live service config | None — verified by searching node/rpc for external flush dashboards or n8n-style service config. Manager state is in-process. | None |
| OS-registered state | None — no systemd/launchd unit names for coins flush. Daemon checkpoint is an in-process thread. [VERIFIED: `open_bitcoind/checkpoint.rs`] | None. New coins-flush worker is also in-process. |
| Secrets/env vars | None — no coins-flush secret keys. `PersistMode` is config, not a renamed env. | None |
| Build artifacts | None — no installed package name change. Fjall datadirs are user data, not build artifacts. | None |

**After every file in the repo is updated:** existing schema-2 datadirs may still contain a leftover `"snapshot"` blob. Reopen must ignore it as UTXO truth. A datadir with two-element `H` and missing `B` must replay or fail closed, not open as leftover-empty.

## Common Pitfalls

### Pitfall 1: persist_progress still writes leftover or seeds the full map

**What goes wrong:** Dual truth; reopen hydrates the blob or a seeded full UTXO set; WR-04 crash window returns.

**Why it happens:** `persist_progress` currently `save_header_entries` → `seed_coins_from_snapshot` → `save_chainstate_snapshot`. [VERIFIED: `runtime_state.rs` 88–104] Guards assert that path.

**How to avoid:** Replace snapshot seed/write with headers/runtime persist + optional `decide_flush(IfNeeded)` only if this is the connect-site owner. Prefer IfNeeded on `commit_prepared_*` and keep `persist_progress` off coins dumps.

**Warning signs:** `save_chainstate_snapshot` or `seed_coins_from_snapshot` remain on the progress path after D-18 flips.

### Pitfall 2: Interrupted `H` still fail-closes at store open

**What goes wrong:** FLUSH-02 cannot run; replay never sees hashes.

**Why it happens:** `classify_markers` errors on `(2, None)`; `ensure_schema_two` maps that to `InterruptedWrite` before manager init. [VERIFIED: `coins_view.rs`, `coins.rs`]

**How to avoid:** Observable marker + leftover-empty skip when interrupted. Fail closed only after the manager proves undo/bodies are missing, or after replay fails.

**Warning signs:** `FjallNodeStore::open` still returns `InterruptedWrite` for a fixture that has undo+bodies.

### Pitfall 3: Leftover-empty shadows interrupted `H` again

**What goes wrong:** Crash after `H` and before `C`/`B` opens as `RestoreFromBackup` leftover-empty.

**Why it happens:** `coins_keyspace_is_empty` ignores `H` (only `B` + `C` prefix). [VERIFIED: `coins.rs` `coins_keyspace_is_empty`] WR-02 was fixed by classifying `head_blocks()` first while that call still *errored*. If `head_blocks()` starts returning `Ok([new,old])`, leftover-empty will fire unless the skip is explicit.

**How to avoid:** Keep interrupted classification first; leftover-empty only when `H` is empty.

**Warning signs:** `leftover_plus_two_element_h_without_b_is_interrupted_write` fails or starts expecting leftover-empty.

### Pitfall 4: Display-text remap (IN-01)

**What goes wrong:** `map_heads_error` matches `detail.contains("interrupted write")`. A Display change turns interrupted flush into `Corruption`/`Repair`. [VERIFIED: `coins.rs` 388–404]

**How to avoid:** Typed `ChainstateError` variant or match `StorageError::InterruptedWrite` before flattening through `error.to_string()`.

**Warning signs:** New `CoinsStorage { detail }` arms; tests that assert Display substrings.

### Pitfall 5: Cache occupancy has no estimator

**What goes wrong:** Shell cannot inject `cache_bytes` / `entry_count`; IfNeeded never fires; Periodic LARGE path is untestable.

**Why it happens:** `CoinsCache` exposes occupancy as map hits, not bytes. [VERIFIED: grep — no `cache_bytes` / `estimated` on cache]

**How to avoid:** Add first-party `cache_entry_count()` and `estimated_cache_bytes()` on the overlay (script/value sizes + fixed per-entry overhead). Do not use Fjall `len()` or LevelDB `SizeEstimate` (D-05).

**Warning signs:** Hardcoded `cache_bytes: 0` in production `FlushPolicyInput`.

### Pitfall 6: File-length overflow on hot files

**What goes wrong:** 628-line gate fails.

**Why it happens:** Near-limit files: `runtime_state.rs` 625, `engine.rs` 615, `sync.rs` 611. [VERIFIED: `wc -l`]

**How to avoid:** New `chainstate/flush_lifecycle.rs` and `chainstate/replay.rs`. Shrink `persist_progress` in place or extract. Do not grow `engine.rs` by inlining generics — extract `apply_*_on_overlay`.

**Warning signs:** Edits that only append to `runtime_state.rs` / `engine.rs` / `sync.rs`.

### Pitfall 7: Progress credit from connected tip

**What goes wrong:** Operators and tests treat connect as durable; restart looks like data loss (allowed window becomes a surprise).

**Why it happens:** `connected_block()` uses `maybe_chain_tip`; `made_validated_durable_progress` runs after `persist_progress` which today dumps the full map.

**How to avoid:** Credit only when coins `B` equals the claimed tip (or a flush receipt names that hash). Keep connected tip as in-memory fact.

**Warning signs:** Restart tests that expect unflushed IfNeeded connects to survive.

### Pitfall 8: Claim creep and leftover consumers

**What goes wrong:** Docs imply archive/assumeutxo; wallet rescan still materializes leftover UTXOs as truth.

**Why it happens:** `hydrate_chainstate_for_open` still builds a full `ChainstateSnapshot` for open. Wallet rescan was deferred by 141 as a leftover consumer.

**How to avoid:** Production open must not use leftover UTXOs. Do not rewrite wallet-rescan product surfaces here, but do not hydrate them from leftover. Phase 143/144 own availability and operator evidence.

**Warning signs:** README “full chainstate manager”; rescan tests that pass only because leftover was written.

## Code Examples

Verified patterns from this repo and pinned Knots:

### Execute decide_flush without re-choosing write-kind

```rust
// Source: packages/open-bitcoin-chainstate/src/coins/flush.rs
pub fn decide_flush(input: FlushPolicyInput) -> FlushDecision { /* Flush | Sync | None | RefuseDiskSpace */ }

// Shell (new flush_lifecycle.rs)
let decision = decide_flush(FlushPolicyInput {
    mode, // IfNeeded | Periodic | Always
    cache_bytes: cache.estimated_cache_bytes(),
    cache_byte_limit: injected_limit, // default 450 MiB − 8 MiB coins-DB cap
    mempool_leftover_bytes,
    now: FlushPolicyTime::from_unix_seconds(unix_seconds),
    next_write: already_jittered,
    memory_pressure, // injected bool; default false
    disk_free_bytes, // Fjall/OS probe in shell
    cache_entry_count: cache.cache_entry_count(),
});
```

### Knots ReplayBlocks shape (apply-only)

```text
// Source: https://github.com/bitcoinknots/bitcoin/blob/v29.3.knots20260210/src/validation.cpp
// ReplayBlocks:
//   heads empty → consistent return
//   heads.size != 2 → unknown inconsistent state
//   pindexNew = index[heads[0]]; pindexOld = index[heads[1]] (0 allowed)
//   rollback old → fork via DisconnectBlock (idempotent)
//   rollforward fork+1 → new via RollforwardBlock (SpendCoin + AddCoins overwrite)
//   cache.SetBestBlock(new); cache.Flush()
```

### Leftover-write guards that must flip (D-18)

```rust
// Source: packages/open-bitcoin-chainstate/src/coins/tests/flush.rs
// TODAY: persist_src.contains("save_snapshot") && !contains("decide_flush")
// AFTER: persist calls decide_flush / FlushMode; no live save_snapshot

// Source: packages/open-bitcoin-node/src/storage/fjall_store/tests/coins_migration.rs
// TODAY: runtime_state contains save_chainstate_snapshot; seed before leftover
// AFTER: no save_chainstate_snapshot on persist_progress
```

### Typed interrupted remap (replace IN-01)

```rust
// Source: packages/open-bitcoin-node/src/storage.rs — keep this variant
StorageError::InterruptedWrite {
    namespace: StorageNamespace::Coins,
    action: StorageRecoveryAction::Reindex, // diagnostic; do not auto-reindex
}

// Do not keep:
// if detail.contains("interrupted write") { interrupted_coins_write() }
```

### Shell jitter after Periodic success

```rust
// Source: Knots validation.cpp DATABASE_WRITE_INTERVAL_MIN/MAX = 50min/70min
// Sample in shell with getrandom 0.3.4; inject next_write. Never Instant::now() in core.
const PERIODIC_WRITE_MIN_SECS: u64 = 50 * 60;
const PERIODIC_WRITE_MAX_SECS: u64 = 70 * 60;
```

### Cache defaults (shell constants, not flush.rs)

```rust
// Source: .planning/research/STACK.md citing Knots kernel/caches.h / node/caches.h
const DEFAULT_KERNEL_CACHE_BYTES: u64 = 450 * 1024 * 1024;
const MIN_DBCACHE_BYTES: u64 = 4 * 1024 * 1024;
const COINS_DB_CACHE_CAP_BYTES: u64 = 8 * 1024 * 1024;
// coins-tip limit = max(MIN_DBCACHE, DEFAULT_KERNEL − COINS_DB_CAP)
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Every-connect leftover JSON snapshot | Knots Periodic 50–70 min + IfNeeded CRITICAL + Always shutdown | Knots hourly flush era; Open Bitcoin still on leftover writes | Phase 142 must adopt the crash-loss window, not keep dumps |
| `ReplayBlocks` uninterruptible | Bitcoin #30155 proposed resumable replay; closed — hourly flush made long replays rare | 2025 close | Do **not** port interruptible replay. Full replay then clear markers. |
| `CanFlushToDisk` = coins views + cache present | Same predicate | Knots current | Readiness is a flag after init, not “store opened” |
| Snapshot hydrate into `MemoryCoinsView` | Disk parent + empty cache + tip from `B` | This phase | Stops loading the full UTXO set on restart |

**Deprecated/outdated:**

- `ManagedChainstate::persist` → `save_snapshot` as durability
- `persist_progress` leftover dual-write / `seed_coins_from_snapshot` as live truth
- `map_heads_error` Display-text remap
- Production `Chainstate` hardcoded to `MemoryCoinsView`
- Treating `FjallNodeStore::open` `InterruptedWrite` as the finished recovery story

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | Fjall `Database::disk_space()` is the shell probe for `disk_free_bytes` (not yet called in node). [ASSUMED: STACK.md Fjall API; no local `disk_space` call found] | Standard Stack / Pattern 2 | Planner may need `std::fs::metadata` on the datadir in the node crate instead. Either is shell-only. |
| A2 | Default `memory_pressure = false` is acceptable until an OS probe exists. [ASSUMED: 140 D-08 injects a bool; no probe exists] | Pattern 2 | IfNeeded will not Flush on host memory pressure; CRITICAL still Flushes. |

**If this table were empty:** all claims would be verified. These two are the only assumed injection sources.

## Open Questions

1. **Who owns IfNeeded after connect — `commit_prepared_*` or `persist_progress`?**
   - What we know: Both currently persist. `should_persist_progress` is every accepted connect/reorg. D-16 says IfNeeded after connect/reorg.
   - What's unclear: Putting IfNeeded in both double-asks policy (harmless) vs splitting headers persist from coins flush (clearer).
   - Recommendation: `commit_prepared_connect` / `commit_prepared_reorg` / `connect_block` / `disconnect_tip` / `reorg` call the one manager flush with `IfNeeded`. `persist_progress` persists headers/runtime only and credits from coins `B`.

2. **Wallet rescan leftover consumer**
   - What we know: 141 deferred rescan rewrite. Tests still `seed_coins_from_leftover_for_reopen`.
   - What's unclear: How many rescan tests break if leftover writes stop.
   - Recommendation: Keep `seed_*` as a test helper; production persist must not call it. Do not expand Phase 142 into rescan product work.

3. **Delete vs leave leftover snapshot files**
   - Discretion. Recommendation: leave unread; stop writing. Deletion is optional cleanup, not required for MGR-02.

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| Rust / Cargo | All implementation | ✓ | 1.94.1 | — |
| Bun | Breadcrumb / verify scripts | ✓ | local bun | — |
| Fjall 3.1.4 | Coins parent | ✓ | Cargo pin | — |
| `getrandom` 0.3.4 | Periodic jitter | ✓ | node + rpc deps | — |
| Tokio (rpc) | Periodic tick | ✓ | rpc crate only | Thread+channel like `checkpoint.rs` |
| Bitcoin Knots tree | Line-level C++ cites | ✓ | `packages/bitcoin-knots/src/validation.cpp` present | GitHub tag already fetched this session |
| Fjall `disk_space` | Injected `disk_free_bytes` | ✗ wired | API claimed by STACK | Node `std::fs` datadir free-space probe |

**Missing dependencies with no fallback:** none.

**Missing dependencies with fallback:** disk-free probe not wired; use datadir filesystem stat in the node shell.

**Step 2.6 note:** No new external services. Phase is code/config plus existing Fjall datadirs.

## Security Domain

`security_enforcement` is not `false` in `.planning/config.json` (absent = enabled).

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V2 Authentication | no | Not in scope |
| V3 Session Management | no | Not in scope |
| V4 Access Control | no | Single-process datadir lock already exists; no new authz |
| V5 Input Validation | yes | Decode `H`/`B`/undo/block bodies fail closed; reject unexpected `H` counts |
| V6 Cryptography | no | No new crypto; first-party hashes only |

### Known Threat Patterns for chainstate flush / restart

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Mixed old/new coins treated as consistent tip | Tampering / elevation of privilege over consensus | Two-element `H` + missing `B` is interrupted; replay or fail closed; never invent `B` |
| Leftover snapshot hydrates stale or attacker-written UTXOs | Tampering | Leftover non-authoritative; tip from coins `B` |
| Display-text remap hides interrupted flush | Information disclosure / denial of recovery | Typed `InterruptedWrite` (D-10 / IN-01) |
| Auto-reindex / destructive repair | Denial of service / destruction | FUT-23 out of scope; `Reindex` action is diagnostic only |
| Serve bodies inferred from coins | Spoofing | Phase 143; do not set `durable_availability` from `B` |
| High-cardinality UTXO dumps in metrics | Information disclosure | Phase 144; this phase may keep internal receipts only |

## Sources

### Primary (HIGH confidence)

- `packages/open-bitcoin-chainstate/src/coins/flush.rs` — `decide_flush`, `decide_recovery`, injected facts
- `packages/open-bitcoin-chainstate/src/coins/cache.rs` — `flush` / `sync` / `from_parent`
- `packages/open-bitcoin-chainstate/src/engine.rs` — `Chainstate` still `CoinsCache<MemoryCoinsView>`
- `packages/open-bitcoin-node/src/chainstate.rs` — `persist` → `save_snapshot`
- `packages/open-bitcoin-node/src/sync.rs` — open hydrates `MemoryChainstateStore::from_snapshot`
- `packages/open-bitcoin-node/src/sync/runtime_state.rs` — leftover dual-write `persist_progress`
- `packages/open-bitcoin-node/src/storage/coins_view.rs` — `classify_markers` fail-closed interrupted
- `packages/open-bitcoin-node/src/storage/fjall_store/coins.rs` — WR-02 order, IN-01 remap, hydrate scan
- `packages/open-bitcoin-node/src/storage/fjall_store/tests/coins_migration.rs` — leftover-write + interrupted-H tests
- `packages/open-bitcoin-chainstate/src/coins/tests/flush.rs` — leftover-write guards + `flush.rs` must stay count-only
- Knots `v29.3.knots20260210` `validation.cpp` `FlushStateToDisk`, `ReplayBlocks`, `RollforwardBlock`, `DATABASE_WRITE_INTERVAL_{MIN,MAX}`
- Knots `v29.3.knots20260210` `node/chainstate.cpp` `CompleteChainstateInitialization` (InitCoinsDB → ReplayBlocks → InitCoinsCache → CanFlush)
- `scripts/check-pure-core-deps.sh`, `scripts/bright-builds-check.ts` (628-line limit)
- `.planning/phases/141-durable-fjall-coins-adapter/141-REVIEW.md` WR-02 / IN-01
- `.planning/research/{ARCHITECTURE,STACK,PITFALLS,SUMMARY}.md`

### Secondary (MEDIUM confidence)

- Fjall `3.1.4` `disk_space()` as the disk-free probe — documented in milestone STACK, not called in node today
- Bitcoin Core #30155 closed (interruptible replay not needed) — [CITED: github.com/bitcoin/bitcoin/pull/30155]

### Tertiary (LOW confidence)

- None material. Local Knots tree is present; GitHub tag was also fetched this session.

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH — local pins and Cargo.toml verified
- Architecture: HIGH — live call sites and Knots init/replay/flush order verified
- Pitfalls: HIGH — leftover guards, IN-01, fail-at-open, file-length, and leftover-empty shadow are in-repo

**Research date:** 2026-09-06
**Valid until:** 2026-10-06 (stable internal architecture; re-check if Phase 141 follow-up commits change `classify_markers` or leftover guards)
