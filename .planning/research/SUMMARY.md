# Project Research Summary

**Project:** Open Bitcoin
**Milestone:** v2.3 Chainstate Durability and Historical Serving
**Domain:** Bitcoin chainstate durability (coins DB, cache flush, manager, honest availability)
**Researched:** 2026-08-29
**Confidence:** HIGH

## Executive Summary

v2.3 is a storage-first durability milestone, not a prune/archive product or IBD-speed milestone. Experts (pinned Bitcoin Knots `29.3.knots20260210`) keep spendable UTXO truth in a disk-backed coins view, overlay an in-memory dirty/fresh cache, flush on policy rather than after every connect, and treat historical serving as a payload-present fact. Open Bitcoin already has the pure UTXO engine, Fjall storage, default-off block serving, and `pruned`/`unavailable` labels. The current defect is collapsing those layers: `ManagedChainstate` still rewrites a pretty-printed JSON `ChainstateSnapshot` after every mutation, and inventory can mark a block `Available` from a boolean or index membership without proving the body exists.

The recommended approach is an internal persistence-shape change with **no new production crates**. Extend `open-bitcoin-chainstate` with a pure `CoinsView` / `CoinsCache` / `FlushPolicy` (I/O-free). Extend `FjallNodeStore` on Fjall `3.1.4` with per-outpoint coins, `best_block`, and two-element `head_blocks`. Grow `ManagedChainstate` into a single-active-chainstate manager that inits DB → health-check → cache, flushes `IfNeeded` / `Periodic` / `Always`, and restarts from coins best-block. Honest availability is a truth contract over what is stored today: serve or report `Available` only when the `block:` payload is present; refuse cleanly with `Unavailable` when it is not. Do not add LevelDB, RocksDB, rust-bitcoin, assumeutxo/assumevalid, prune-mode file managers, or archive-node serving.

The main risks are dual truth (snapshot blob plus coins DB), treating a cache hit as durable, incremental writes without an interrupted-flush protocol, flushing coins before block/undo/index, and claim creep that reads "durable coins" as archive-node or assumeutxo. Mitigate by naming one durable coin source of truth before the first production write, porting Knots DIRTY/FRESH as typed cache algebra, requiring `head_blocks` around 64 MiB batches, advancing coins tip only after matching payload and undo are durable, and enforcing a bounded claim taxonomy (`disk_backed_coins`, `cache_flush_policy`, `chainstate_manager_flush_lifecycle`, `honest_payload_availability`). Phase numbering continues from 139. Do not invent prune or assumeutxo phases.

## Key Findings

### Recommended Stack

See [STACK.md](./STACK.md). Add **no new production crates** and do **not** bump Fjall. Disk-backed coins, flush policy, and manager behavior fit the existing Rust workspace, first-party UTXO types, and the Fjall store already used by `FjallNodeStore`.

**Core technologies:**
- Rust `1.94.1` / edition 2024: typed cache flags, flush vs sync, and recovery outcomes — already pinned; keep `open-bitcoin-chainstate` free of Fjall, Tokio, and clocks.
- Workspace crates `0.1.0` (`chainstate`, `codec`, `node`, `rpc`): own Coin/outpoint types, compact coin records, Fjall adapter, and manager orchestration — preserves production-path ownership and functional-core / imperative-shell.
- Bitcoin Knots `29.3.knots20260210`: external behavior contract — cite `coins.h`/`coins.cpp`, `txdb.h`/`txdb.cpp`, `validation.cpp` (`FlushStateToDisk`), `node/blockstorage.cpp`.
- Fjall `3.1.4` (existing pin, `default-features = false`): per-coin records, atomic batches, prefix/range cursors, `persist(Buffer|SyncAll)` — no capability gap that justifies LevelDB or a second store.
- Existing Tokio `1.52.1` in `open-bitcoin-rpc` only, plus `getrandom 0.3.4` in the shell: periodic 50–70 minute flush wakeup with injected jitter — do not add Tokio to chainstate or node.
- serde / serde_json (existing): versioned metadata and operator evidence — never encode the live UTXO set as pretty JSON.

**Stack additions (modules, not crates):** dedicated `coins` keyspace (`StorageNamespace::Coins`); first-party compact outpoint/coin codec including `created_median_time_past`; `PersistMode` mapped as today (`Buffered` / `Flush` / `Sync`); schema bump with fail-closed mismatch; leftover `"snapshot"` blobs become non-authoritative after an explicit, testable migration.

### Expected Features

See [FEATURES.md](./FEATURES.md). Table stakes are the v2.3 claim. Missing any of them means the node still has snapshot-only coin truth or can lie about stored blocks.

**Must have (table stakes):**
- Disk-backed per-outpoint coins database with `best_block` and interrupted-flush markers — restart without replaying the chain from a dumped snapshot.
- In-memory coins cache with DIRTY/FRESH and Flush vs Sync — stop persisting the whole UTXO set after every connect.
- Cache-flush policy (`IfNeeded` / `Periodic` / `Always`) with disk-space refusal — bounded cache and crash-bounded replay.
- Crash-safe batch plus interrupted-flush recovery (replay or fail-closed) using stored undo/block bodies — a mid-flush crash must not invent a consistent tip.
- Manager-owned coins lifecycle: init DB, verify, init cache, `CanFlush`, restart from coins best-block — callers must not pick store, cache, and flush independently.
- Honest stored-block availability and clean refuse — payload-present is the only serve/report gate; emit `Unavailable` when bytes are absent, not fake `Pruned`.
- Operator evidence plus parity roots and no-claim guardrails — flush/recovery and "we have the bytes" vs "we do not," without archive or production claims.

**Should have (competitive):**
- Typed flush/recovery state machine — reviewable IF_NEEDED/PERIODIC/ALWAYS without tracing LevelDB.
- Distinct `payload_present` / `index_known` / `validated_on_active_chain` facts — prevents index or `durable_availability` from authorizing a serve.
- Cache-size and flush-reason evidence (`OK` / `LARGE` / `CRITICAL`, last reason).
- Fail-closed coins read errors — a disk read error must not look like a spent or missing coin.
- Explicit claim taxonomy — `disk_backed_coins` is not archive-node or assumeutxo.

**Defer (later milestones, not v2.3 phases):**
- Prune-mode product behavior (`-prune`, file unlink, `m_have_pruned`, `NODE_NETWORK_LIMITED`).
- Archive-node or production-scale historical serving.
- assumeutxo / assumevalid / IBD snapshot shortcuts and dual-chainstate.
- Compact-filter and BIP37 serving.
- Knots/Core `chainstate/` LevelDB binary import.
- Automatic destructive reindex/repair.
- Public serving defaults, public-network CI, production full-node readiness, production-funds wallet use.

### Architecture Approach

See [ARCHITECTURE.md](./ARCHITECTURE.md). Keep coins policy, cache flags, and flush *decisions* in `open-bitcoin-chainstate`. Keep Fjall, filesystem, clocks, and persist *execution* in `open-bitcoin-node`. Replace "clone the whole UTXO map, then write one `snapshot` blob" with a Knots-shaped layered view: typed `CoinsView`, in-memory dirty/fresh cache, Fjall-backed disk parent, and a manager that chooses flush points and rebuilds an empty cache on restart. One active chainstate only. `ChainstateSnapshot` becomes a test/migration helper, not live truth.

**Major components:**
1. `CoinsView` / `CoinsCache` / `FlushPolicy` (pure core) — Get/Have/Add/Spend, DIRTY/FRESH, Flush vs Sync, injected-fact flush decisions.
2. `MemoryCoinsView` (pure or test adapter) — in-memory parent for tests and prepare overlays; no I/O.
3. `FjallCoinsView` + coin codec (node shell) — per-outpoint get/batch, `best_block`, `head_blocks`; parse at the storage boundary.
4. `ManagedChainstate` (node shell) — cache lifecycle, ordered flush, shutdown `Always`, restart from coins best-block.
5. `DurableSyncRuntime` — attach disk view on open; flush dirty coins + headers on progress; stop `save_chainstate_snapshot` as the live path.
6. Honest availability (inventory / serve / RPC) — `Available` only when `load_block` / `contains_key` on `block:` succeeds.
7. Wallet rescan and leftover snapshot consumers — read through a view/cursor so they cannot resurrect snapshot-as-truth.

### Critical Pitfalls

See [PITFALLS.md](./PITFALLS.md). Highest-risk ordering mistake: implementing "chainstate manager" or operator evidence before a single durable coin source of truth and a cache-vs-disk distinction.

1. **Dual truth (snapshot blob plus coins DB)** — Pick one durable coin source before the first production write. After cutover, reopen must not hydrate `ManagedChainstate` from a leftover blob. Wallet rescan, confirmation migration, and `persist_progress` move together.
2. **Cache hit treated as durable coin truth** — Progress credit, restart, and "durably persisted" fields move only on committed coins `best_block`. Cache occupancy, dirty/fresh, and durable tip are three facts.
3. **Incremental coins writes without `head_blocks`** — Require the two-phase marker around 64 MiB batches. Recovery finishes the transition from durable undo/blocks or fails closed. Do not auto-repair. Fjall persist changes durability, not consistency.
4. **Flushing coins out of order with block/undo/index** — One shell-owned sequence: block+undo payloads, then index, then coins. Coins best-block advances only after matching payload and undo are durable. Failed coins writes fail closed.
5. **DIRTY/FRESH mis-applied so spentness never reaches disk** — Port Knots cache-entry transitions as typed state with reorg unit tests. Do not invent write-through-every-mutation and call it flush policy.
6. **Serving or labeling a block whose payload is absent** — `load_block` (or equivalent probe) is the only serve/available proof. Do not set `durable_availability` from coins tip or header index. Do not emit `Pruned` as a synonym for missing bytes.
7. **I/O or clocks in `open-bitcoin-chainstate`, or claim creep** — Core stays I/O-free. Capture a flush delta under the lock; persist after release. No LevelDB, rust-bitcoin, assumeutxo, prune unlink, or archive-node language. Keep historical `.planning/phases/` tracked.

## Implications for Roadmap

Based on combined research, suggested phase structure starting conceptually at **Phase 139+**. The roadmapper assigns exact numbers. Do **not** add prune, assumeutxo, or archive-serving phases.

### Phase 139+: Coins-view and cache contract
**Rationale:** Every later phase depends on a cache-vs-disk distinction. If flush policy is written against today's `save_snapshot`, later work keeps snapshot semantics. Pure-core first; no Fjall yet.
**Delivers:** `CoinsView`, `CoinsCacheEntry` DIRTY/FRESH, batch-write types, `MemoryCoinsView`, HaveCoin vs in-cache. Engine tests may still use snapshot helpers.
**Addresses:** In-memory coins cache foundation; fail-closed typed errors; no I/O in core.
**Avoids:** Pitfalls 2, 3, 8 (cache-as-durable, DIRTY/FRESH, I/O in engine). Names one source of truth: the view, not the blob.

### Phase 140+: Flush policy (pure)
**Rationale:** Policy must exist before the manager can stop persisting after every connect. Unit-testable without a store.
**Delivers:** `FlushMode { None, IfNeeded, Periodic, Always }`, `CoinsCacheSizeState { Ok, Large, Critical }`, `decide_flush` from injected now/size/mode/memory-pressure. `None` is observability only — no prune hook implementation.
**Addresses:** Cache-flush policy (decision surface); typed flush state machine (P2).
**Avoids:** Clock in core; `fFlushForPrune`; treating Flush and Sync as the same.

### Phase 141+: Engine apply on a view
**Rationale:** Connect/disconnect/reorg must mutate a cache, not clone `HashMap`. Prepare/commit becomes a child cache flushed into the parent cache. Still no disk.
**Delivers:** Engine apply against `CoinsCache`; undo and chain metadata remain first-class; `prepare_*` / `commit_*` without full-map clone.
**Addresses:** Stop snapshot-shaped connect; enable later incremental flush.
**Avoids:** Anti-pattern "clone `Chainstate` to prepare a connect"; write-through every mutation.

### Phase 142+: Durable Fjall coins adapter
**Rationale:** First shell I/O change. Storage engine exists; schema and truth change. Must land before manager rewrite so reopen has a real parent view.
**Delivers:** `coins` keyspace, compact codec (including MTP), per-outpoint get/put/erase, `best_block`, two-element `head_blocks`, 64 MiB dirty batches, schema bump, fail-closed mismatch, explicit one-way migration from legacy `"snapshot"` blob. `save_block` / `load_block` unchanged.
**Uses:** Fjall `3.1.4` `batch` / `persist` / `contains_key` / prefix/range; existing `PersistMode`.
**Avoids:** Pitfalls 4 and 5 (no heads marker; dual write as live truth). LevelDB / rust-bitcoin stay out.

### Phase 143+: Manager flush lifecycle and restart
**Rationale:** "Fuller manager" is this single-chainstate lifecycle, not assumeutxo. Wiring must consume committed-best-block and the ordered flush protocol.
**Delivers:** Init coins DB → health check → init cache (`CanFlush` false until then). Flush on `IfNeeded` after connect/reorg, `Periodic` on injected ticks, `Always` on shutdown. Block/undo/index then coins. `DurableSyncRuntime::open` attaches `FjallCoinsView` and hydrates an empty cache from coins best-block. `persist_progress` stops writing the full UTXO blob. Same-datadir reopen, progress credit only after flush receipt, wallet-rescan/migration callers pointed at the new truth or a maintained projection.
**Implements:** Knots `CoinsViews` / `LoadChainTip` / `FlushStateToDisk` subset for one active chainstate.
**Avoids:** Pitfalls 1, 5, 9 (flush order; leftover blob hydrate; restart/rescan/progress break). No dual-chainstate, no `EmplaceCoinInternalDANGER`.

### Phase 144+: Honest availability
**Rationale:** Honesty is payload presence, not coins durability. Can start as soon as `load_block` is the presence oracle and must not ship *after* operator evidence that would re-document the lie. Does not require the coins adapter, but must not infer availability from coins tip.
**Delivers:** Inventory/serve/RPC classify `Available` only after a `block:` payload probe. Missing payload → clean refuse (`Unavailable` / `LookupUnavailable` / `NotFound`). Reserve `Pruned` for later prune-mode product work; do not emit it as "index says yes, body says no" or "active and not tip."
**Addresses:** Honest stored-block availability; clean refuse.
**Avoids:** Pitfall 6. No archive-node serving, no public-default change.

### Phase 145+: Operator evidence, leftover consumers, and claim guardrails
**Rationale:** Evidence last as enforcement; taxonomy must already be in requirements so mid-phase PRs cannot drift. Consumers that still load the blob (wallet rescan, confirmation migration, any RPC that materializes `ChainstateSnapshot.utxos`) must move so snapshot-as-truth cannot resurrect.
**Delivers:** Status/RPC/CLI/dashboard/metrics/logs/support: cache-size state, last flush mode/reason, coins best-block, interrupted-flush/replay outcome, per-request availability labels. Update `docs/parity/catalog/chainstate.md` and breadcrumbs. Narrow no-claim checkers: in-scope `disk_backed_coins` / `cache_flush_policy` / `chainstate_manager_flush_lifecycle` / `honest_payload_availability`; still-deferred assumeutxo, prune_mode, archive_node, public/default serving, production readiness. Historical `.planning/phases/` stay tracked. Default `verify.sh` stays deterministic.
**Addresses:** Operator evidence; parity roots; no-claim guardrails; leftover snapshot consumers.
**Avoids:** Pitfalls 7 and 10 (smuggled assumeutxo/LevelDB/prune; claim creep; phase-dir deletion).

### Phase Ordering Rationale

- Pure view + cache + policy (139–141) before Fjall so architecture checks and hermetic engine tests stay green, and so flush policy has a cache that is not the durable store.
- Fjall adapter (142) before manager (143) so restart has a disk parent and cutover has one source of truth.
- Manager (143) before or with leftover consumers so `DurableSyncRuntime::open` does not keep hydrating `MemoryChainstateStore` from a blob.
- Honest availability (144) before operator evidence (145) so docs cannot re-document `durable_availability: true`.
- Claim guardrails last as checkers, but the taxonomy is a requirements input from day one.
- Coins durability and historical serving are different products: do not "make serving honest" by inferring availability from coins.

### Research Flags

Phases likely needing deeper research during planning:
- **Phase 142+ (Fjall coins adapter):** Exact per-coin key schema, 64 MiB size accounting without LevelDB `SizeEstimate`, `head_blocks` encoding on Fjall, and whether undo stays in the snapshot DTO or becomes its own record. Architecture confidence is MEDIUM here. Prefer STACK's dedicated `coins` keyspace over stuffing millions of outpoints into the existing `chainstate` `"snapshot"` neighborhood.
- **Phase 142+ / 143+ (schema and migration):** When the full `ChainstateSnapshot` stops being live truth; dual-read generation vs hard cutover; `SchemaVersion` bump vs multi-namespace migration so wallet/mempool/runtime snapshots are not invalidated accidentally.
- **Phase 143+ (flush thresholds and crash-loss window):** STACK pins Knots numbers (450 MiB default, 4 MiB min, 8 MiB coins-DB cap, LARGE ≥90% or within 10 MiB, 50–70 min jitter). Architecture asked not to copy `dbcache` until phase research. Requirements must name the allowed crash-loss window (periodic + clean shutdown vs every-connect).
- **Phase 143+ (interrupted-flush replay vs fail-closed):** Knots `ReplayBlocks` when heads are a two-element vector. Open Bitcoin may fail closed if bodies/undo are missing, but must not pretend the coins DB is consistent.
- **Phase 144+ (`durable_availability` flag shape and `Pruned` wording):** FEATURES/STACK say emit `Unavailable` and reserve `Pruned`. PITFALLS notes v2.1 already used `Pruned` as a local missing-payload label on non-tip active hashes. Requirements must pick one meaning so help text cannot be read as prune-mode.

Phases with standard patterns (skip research-phase unless a design hole appears):
- **Phase 139+ (coins-view/cache contract):** Knots `CCoinsView` / `CCoinsViewCache` and current engine seams are well documented.
- **Phase 140+ (pure flush policy):** `FlushStateMode` and Flush vs Sync are pinned in `validation.h` / `coins.cpp`.
- **Phase 141+ (engine apply on a view):** Existing connect/disconnect/reorg rules stay; only the live map ownership changes.
- **Phase 145+ (operator evidence and claim checkers):** Same evidence/no-claim pattern as v2.1/v2.2; narrow the catalog sentence that currently groups assumeutxo with coins.

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| Stack | HIGH | Local pins (`rust-toolchain.toml`, `Cargo.lock` Fjall `3.1.4`), Fjall 3.1.4 docs, Knots coins/txdb/validation, existing `FjallNodeStore`. Do not bump to crates.io `3.1.5`. |
| Features | HIGH | Pinned Knots coins/flush/manager/serve-path plus current snapshot and inventory code. Exact RPC error-string parity is MEDIUM and not table stakes. |
| Architecture | HIGH for seams; MEDIUM for Fjall schema | Current core/shell layout and Knots APIs are verified. Per-coin key layout, undo relocation, and cache-byte defaults need phase research. |
| Pitfalls | HIGH for hazards; MEDIUM for crash-window | Snapshot/serving seams and Knots flush/consistency are verified. Exact Fjall interrupted-flush encoding and promised loss window wait on requirements. |

**Overall confidence:** HIGH

### Gaps to Address

- **Fjall coins layout:** STACK recommends a dedicated `coins` keyspace; ARCHITECTURE sometimes reuses `StorageNamespace::Chainstate`. Plan the dedicated keyspace unless a later phase proves otherwise.
- **Coin codec home:** STACK puts compact records in `open-bitcoin-codec`; ARCHITECTURE sketches `open-bitcoin-node/src/storage/coins_codec.rs`. Prefer codec-crate encode/decode with the node adapter calling it, so the engine never sees raw bytes.
- **Undo location:** Knots keeps undo with block files; Open Bitcoin keeps `undo_by_block` inside `ChainstateSnapshot`. Decide before incremental coins: persist undo as its own record so disconnect and `ReplayBlocks` do not reload a UTXO blob.
- **Allowed crash-loss window:** Product choice. Today's every-connect snapshot dump is not Knots periodic write. Requirements should state periodic + shutdown durability, not silent every-connect persist.
- **Interrupted-flush success vs fail-closed:** Requirements should allow fail-closed when undo/payload is missing, and forbid "looks consistent" on a two-head or unknown-head state.
- **`Pruned` label:** Align FEATURES (reserve until prune mode) with existing v2.1 classifier language in requirements and help text.
- **Exact Knots RPC missing-block strings:** Status labels are table stakes; string-for-string RPC parity only if a later requirement names it.
- **Local Knots tree:** ARCHITECTURE cited GitHub for some Knots files because the submodule was not materialized in that session. Planning should `git submodule update --init --recursive` before phase research that quotes `txdb.cpp` line-level behavior.

## Sources

### Primary (HIGH confidence)
- `packages/Cargo.toml`, `rust-toolchain.toml`, `packages/Cargo.lock` — Rust `1.94.1`, Fjall `3.1.4`, serde, getrandom pins
- `packages/open-bitcoin-chainstate` — in-memory `HashMap` UTXO truth, no I/O deps
- `packages/open-bitcoin-node/src/chainstate.rs`, `storage.rs`, `storage/fjall_store.rs`, `storage/snapshot_codec.rs` — snapshot persist, namespaces, `SchemaVersion::CURRENT = 1`
- `packages/open-bitcoin-node/src/sync.rs`, `sync/runtime_state.rs` — snapshot hydrate on open; `persist_progress` writes headers + full snapshot
- `packages/open-bitcoin-node/src/network/inventory.rs`, `network/block_serving.rs` — `durable_availability` / `Pruned` classification gap
- `packages/open-bitcoin-rpc/src/context/inbound_wire.rs` — lookup miss → NotFound
- `packages/bitcoin-knots/src/coins.h`, `coins.cpp`, `txdb.h`, `txdb.cpp`, `validation.h`, `validation.cpp`, `node/blockstorage.cpp`, `node/caches.h`, `kernel/caches.h` — cache flags, BatchWrite heads, FlushStateToDisk order, 50–70 min window, 450 MiB default
- [Fjall 3.1.4 `Database` / `Keyspace` / `PersistMode`](https://docs.rs/fjall/3.1.4/fjall/) — batch, persist, get/contains_key/prefix/range
- `standards/core/architecture.md`, `.planning/PROJECT.md`, `docs/parity/catalog/chainstate.md`, `docs/parity/production-claim-boundary.md`

### Secondary (MEDIUM confidence)
- Snapshot codec retirement vs dual-write timing — needs a requirements generation gate
- Interrupted-flush replay vs fail-closed when bodies/undo are missing
- Exact Fjall `head_blocks` encoding and cache-byte accounting vs Knots allocator sizes
- [crates.io fjall latest `3.1.5`](https://docs.rs/crate/fjall/latest) — newer than the pin; do not upgrade in v2.3
- Exact RPC error-string parity for missing blocks

### Tertiary (LOW confidence)
- ARCHITECTURE local `packages/bitcoin-knots` file-path reads (submodule not materialized in that research session); use the pinned GitHub tag or a materialized checkout during phase research

---
*Research completed: 2026-08-29*
*Ready for roadmap: yes*
