# Feature Research

**Domain:** Bitcoin chainstate durability and honest historical availability
**Milestone:** Open Bitcoin v2.3 Chainstate Durability and Historical Serving
**Baseline:** Bitcoin Knots `29.3.knots20260210`
**Researched:** 2026-08-29
**Confidence:** HIGH — pinned Knots coins/flush/manager/serve-path sources plus current Open Bitcoin snapshot and block-serving code

## Scope Decision

v2.3 is a storage-first durability milestone, not a prune/archive or IBD-speed milestone.

Knots already separates three things that Open Bitcoin currently collapses:

1. **Coins truth** lives in a disk-backed `CCoinsViewDB` under `chainstate/`, with an in-memory `CCoinsViewCache` that is flushed on policy, not after every connect.
2. **Chainstate-manager behavior** owns that coins view, flush points, `LoadChainTip` from the coins best-block, and interrupted-flush replay. Dual IBD/snapshot chainstates exist only for assumeutxo.
3. **Historical availability** is a payload-present contract: `BLOCK_HAVE_DATA` plus a successful `ReadBlock`/`ReadRawBlock`. Index membership is not enough. `pruned` is a prune-mode fact, not a synonym for "we do not have the bytes."

Open Bitcoin already has the pure-core UTXO connect/disconnect/reorg engine, node-side snapshot persistence, durable sync, and default-off block serving with `pruned`/`unavailable`/`unvalidated` labels. Those are foundations, not new v2.3 features. The current durability model still persists a full `ChainstateSnapshot` after every connect, and availability can be asserted from a boolean or active-chain membership without proving the payload is present.

Do **not** treat prune-mode product behavior, archive-node serving, assumeutxo/assumevalid, compact filters, BIP37, public defaults, or production readiness as table stakes. Honest availability is a truth contract over what is stored today.

## Feature Landscape

### Table Stakes (Users Expect These)

Features operators and contributors assume exist once this milestone claims chainstate durability. Missing these = the node still has snapshot-only coin truth or can lie about stored blocks.

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| Disk-backed coins database | A node that restarts must recover spendable UTXOs without replaying the whole chain from a dumped snapshot | HIGH | Knots `CCoinsViewDB` stores per-outpoint coins (`DB_COIN`/`C`), best-block (`DB_BEST_BLOCK`/`B`), and in-progress heads (`DB_HEAD_BLOCKS`/`H`). Reuse the existing Fjall adapter and Coin/undo types; replace snapshot-as-sole-truth. Do not claim Knots LevelDB `chainstate/` file compatibility. |
| In-memory coins cache over disk | Connect/disconnect cannot hit disk for every input/output and cannot persist the whole UTXO set after every block | HIGH | Knots `CCoinsViewCache` layers on `CCoinsViewErrorCatcher` → `CCoinsViewDB`. Entries carry DIRTY/FRESH; `Flush()` writes then wipes; `Sync()` writes dirty and drops spent while retaining unspent. Open Bitcoin already has the pure UTXO engine; the shell must stop treating `ManagedChainstate::persist()` as a full-snapshot write after every connect. |
| Cache-flush policy | Operators expect bounded cache memory and crash-bounded replay, not an unbounded dirty cache or a write after every block | HIGH | Knots `FlushStateMode`: `NONE` (prune check only), `IF_NEEDED` (CRITICAL over limit or OS memory pressure), `PERIODIC` (LARGE ≥90%/within 10 MiB, or randomized 50–70 min write), `ALWAYS` (shutdown/resize). `ALWAYS`/`LARGE`/`CRITICAL` call `Flush()`; periodic writes call `Sync()`. Order is block/undo files → block index → coins. Disk-space checks precede writes. |
| Crash-safe coins batch and interrupted-flush recovery | A crash mid-flush must not silently invent a consistent tip | HIGH | Knots `BatchWrite` first erases `B` and writes `H=[new,old]`, then writes dirty coins in ~64 MiB batches, then erases `H` and writes `B`. `ReplayBlocks()` rolls back the old branch and rolls forward to the new head using stored blocks/undo. `GetHeadBlocks()` empty means consistent. Open Bitcoin already persists undo payloads; recovery must consume them instead of a last-good full snapshot only. |
| Manager-owned durable coins lifecycle | Callers must not pick a store, cache, and flush independently | HIGH | Knots `Chainstate` owns `CoinsViews`, `CChain`, flush, `LoadChainTip()`, and `ReplayBlocks()`. `CanFlushToDisk()` is false until the cache is initialized after DB health checks. v2.3 "fuller manager" means this single active chainstate lifecycle, not assumeutxo dual-chainstate or `MaybeRebalanceCaches()`. |
| Restart from coins best-block | After restart, tip and UTXO view must come from durable coins, not a cloned in-memory snapshot | HIGH | Knots `LoadChainTip()` sets the active chain from `CoinsTip().GetBestBlock()`. `LoadChainstate` opens the coins DB, verifies, inits cache, replays interrupted flushes, then loads the tip. Existing snapshot decode can remain a migration/compat path, not the live truth. |
| Honest stored-block availability | Serving or reporting a stored block when the payload is absent is a lie | MEDIUM | Knots serves only when `nStatus & BLOCK_HAVE_DATA`, then `ReadBlock`/`ReadRawBlock` succeeds. Current Open Bitcoin can mark `Available` from `has_local_data \|\| durable_availability` and labels active non-tip missing data as `Pruned` even without prune mode. v2.3 must treat payload-present as the only serve/report gate. |
| Clean refuse when payload is absent | Peers and operators need a refusal, not a hang, empty body, or invented block | MEDIUM | Knots P2P: no `HAVE_DATA` → silent return (do not send the block). `HAVE_DATA` but read fails → disconnect. RPC: "pruned data" / "not fully downloaded" / "Block not found on disk". Open Bitcoin already has `unavailable`/`pruned`/`unvalidated` labels and `lookup_block` miss → `LookupUnavailable`. Wire those to payload-present facts; do not emit `Pruned` unless prune mode actually deleted files. |
| Operator evidence for persistence and availability truth | Operators must see flush/recovery and "we have the bytes" vs "we do not" | MEDIUM | Surface cache size state, last flush mode/reason, coins best-block, interrupted-flush/replay outcome, and per-request availability labels through the existing status/RPC/CLI/dashboard/metrics/log/support contract. Keep aggregates sanitized; do not dump coin keys or raw blocks into support bundles. |
| Parity roots and no-claim guardrails | The project core value requires auditable Knots behavior and scoped claims | MEDIUM | Cite `coins.h`/`coins.cpp`, `txdb.h`/`txdb.cpp`, `validation.cpp` (`FlushStateToDisk`, `ReplayBlocks`, `LoadChainTip`), `node/chainstate.cpp`, `node/blockstorage.cpp`, and `net_processing.cpp` serve-path. Keep prune/archive, assumeutxo, compact filters, public defaults, and production readiness deferred. |

### Differentiators (Competitive Advantage)

Features that set Open Bitcoin apart. Not required for Knots-observable durability, but valuable because they make the same contract safer and auditable.

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| Typed flush and recovery state machine | Makes IF_NEEDED/PERIODIC/ALWAYS and interrupted-flush replay reviewable without tracing LevelDB | HIGH | Pure transitions take cache usage, configured cap, injected time, disk-space facts, and flush mode; adapters perform I/O. Matches functional-core / imperative-shell. |
| Payload-present availability facts | Prevents index membership or a stale `durable_availability` flag from authorizing a serve | MEDIUM | Model `payload_present`, `index_known`, `validated_on_active_chain` as distinct facts. `Available` requires payload_present. Reuse existing labels; stop treating `Pruned` as "missing and not tip." |
| Cache-size and flush-reason evidence | Operators can tell "we have not written lately" from "we cannot flush" | LOW | Expose Knots-equivalent `OK` / `LARGE` / `CRITICAL` plus last flush reason (`needed`, `periodic`, `always`, `failed_disk`). Fixed low-cardinality metrics only. |
| Fail-closed coins read errors | A disk read error must not look like a spent or missing coin | MEDIUM | Knots `CCoinsViewErrorCatcher` runs callbacks on LevelDB read errors rather than returning "not found." Map this onto existing typed recovery (`FreeDisk`, corruption) instead of silent empty coins. |
| Explicit claim taxonomy | Prevents "durable coins" from being read as archive-node or assumeutxo | LOW | Status/docs distinguish `disk_backed_coins`, `cache_flush_policy`, `honest_stored_availability` from deferred `prune_mode`, `archive_serving`, `assumeutxo`. |

### Anti-Features (Commonly Requested, Often Problematic)

Features that seem good but create problems. Treat as future or out of scope, not v2.3 table stakes.

| Feature | Why Requested | Why Problematic | Alternative |
|---------|---------------|-----------------|-------------|
| Prune-mode product behavior | Disk savings and Knots `-prune` parity | Needs height windows, file unlinking, `m_have_pruned`, prune locks, and `NODE_NETWORK_LIMITED` serving limits. Collapses the honesty contract into a product mode too early | Keep `pruned` as a reserved label; report `unavailable` when the payload is absent. Implement prune later on top of honest availability. |
| Archive-node or production-scale historical serving | "Serve any old block" sounds like completeness | Production claim boundary already defers archive-scale serving. v2.3 only tells the truth about what is stored | Serve when the payload is present; refuse cleanly otherwise. No archive or public-default claim. |
| assumeutxo / snapshot chainstate / dual-chainstate | Faster IBD | Knots `ChainstateManager` snapshot/IBD pair, `ActivateSnapshot`, `MaybeCompleteSnapshotValidation`, and `MaybeRebalanceCaches` are a separate consensus-trust and cache-split surface | Single active chainstate. Keep assumeutxo out. |
| assumevalid / IBD snapshot shortcuts | Skip script checks or import a UTXO dump to sync faster | v2.3 is durability, not a sync-speed milestone. Shortcuts change the validation claim | Continue validating and connecting; persist coins after policy flushes. |
| Persist a full UTXO snapshot after every connect | Seems crash-safe and simple | That is the current model. It does not scale, hides flush policy, and is not Knots-observable durability | Disk-backed per-outpoint coins plus Flush/Sync policy. |
| Knots LevelDB `chainstate/` binary compatibility | Drop-in datadir reuse | Different engine (Fjall vs LevelDB), key layout, and crash markers. Quietly reading or writing Knots files is a migration/corruption hazard | Own the coins schema. Keep Core/Knots datadir mutation dry-run-first and deferred. |
| Claiming `Available` from index or `durable_availability` alone | Avoids a disk read on the serve path | Current gap: `inventory.rs` can mark Available when `durable_availability` is true even if `blocks_by_hash` lacks the payload; `lookup_block` then fails. That is the lie this milestone exists to close | Payload-present check is the serve/report gate. Cache may accelerate a proven-present payload; it may not invent presence. |
| Labeling missing active-history as `Pruned` without prune mode | Reuses an existing status label | Knots `IsBlockPruned` requires `m_have_pruned && !(BLOCK_HAVE_DATA) && nTx > 0`. Open Bitcoin currently labels active non-tip missing data `Pruned` | Use `Unavailable` unless prune mode has actually deleted files. |
| Compact-filter or BIP37 serving | Historical lookup without full blocks | Separate protocol/privacy/DoS surface; locked out of v2.3 | Keep filter serving deferred. Honest block availability does not imply filters. |
| Public serving/relay defaults or production readiness | Makes the feature easy to demonstrate | Violates shipped activation and v1.8 claim gates | Keep block serving default-off and production claims deferred. |
| Mempool disconnected-transaction repair during reorg | Knots keeps a disconnect pool | Catalogued chainstate gap, but not required to persist coins or tell the truth about stored blocks | Leave mempool reorg repair on its existing v2.0/v2.2 path unless a later requirement names it. |
| Automatic destructive reindex or coins repair | Recovers from a bad flush | Current recovery is diagnosis-only. Hidden datadir mutation is a production-adjacent anti-feature | Fail closed; tell the operator to retry, free disk, or plan an explicit later repair. |

## Feature Dependencies

```text
Existing pure-core UTXO connect/disconnect/reorg + undo
└──requires──> Disk-backed per-outpoint coins database
                ├──requires──> In-memory dirty/fresh coins cache
                │               └──requires──> Flush/Sync policy (IF_NEEDED / PERIODIC / ALWAYS)
                │                               └──requires──> Crash-safe batch + interrupted-flush replay
                └──requires──> Manager-owned coins view, CanFlush, LoadChainTip
                                └──requires──> Restart from coins best-block

Existing node snapshot adapter + Fjall store
└──enhances──> Disk-backed coins (storage engine exists; schema/truth changes)
Existing undo payloads
└──requires──> Interrupted-flush rollback/roll-forward

Existing default-off block serving + status labels
└──requires──> Payload-present availability facts
                └──requires──> Serve or report only when payload is present
                                └──requires──> Clean refuse + operator/parity evidence

Flush policy ──depends-on──> Cache-size accounting and disk-space facts
Honest availability ──conflicts-with──> Pruned label without prune mode
Honest availability ──conflicts-with──> Available from index/`durable_availability` alone
assumeutxo dual-chainstate ──conflicts-with──> Single active chainstate manager
Prune/archive product modes ──conflicts-with──> v2.3 storage-first scope
```

### Dependency Notes

- **Disk-backed coins require the existing UTXO engine, not a new connect path:** `ManagedChainstate` already connect/disconnect/reorgs and persists. v2.3 changes *what* is persisted and *when*, not the pure-core spend rules.
- **Flush policy requires a cache that is not the durable store:** If every connect still writes a full snapshot, `IF_NEEDED`/`PERIODIC`/`ALWAYS` have nothing to decide.
- **Interrupted-flush replay requires undo and block bodies:** Knots `ReplayBlocks()` reads stored blocks and applies `DisconnectBlock`/`RollforwardBlock` on a coins cache over the DB. Snapshot-only recovery cannot reconstruct a partial batch.
- **LoadChainTip requires coins best-block to be the chain authority:** Tip height/hash in operator status must follow the flushed coins view, not an in-memory snapshot that raced ahead of disk.
- **Honest availability depends on the existing serve gate, not a new P2P protocol:** Labels and `lookup_block` already exist. The missing contract is: do not classify `Available` or allow storage read unless the payload is actually present.
- **`Pruned` without prune mode conflicts with honesty:** Keep the label; do not emit it as a stand-in for missing bytes.
- **Existing snapshot persistence enhances migration, it does not satisfy the milestone:** A versioned snapshot codec can remain for upgrade/compat, but live coins truth must be the disk-backed view.

## MVP Definition

### Launch With (v2.3)

Minimum shippable boundary for the milestone claim.

- [ ] Disk-backed per-outpoint coins database for the active chainstate, with best-block and interrupted-flush markers
- [ ] In-memory coins cache with Flush vs Sync semantics; stop persisting a full UTXO snapshot after every connect
- [ ] Cache-flush policy equivalent to Knots `IF_NEEDED` / `PERIODIC` / `ALWAYS`, including disk-space refusal
- [ ] Interrupted-flush detection and replay (or fail-closed recovery) using existing undo/block bodies
- [ ] Manager-owned coins view, flush points, `CanFlush`-style readiness, and restart from coins best-block
- [ ] Payload-present availability: serve or report a stored block only when the bytes are present
- [ ] Clean refuse with `unavailable` (not fake `pruned`) when the payload is absent
- [ ] Operator status/metrics/logs/support evidence for flush/recovery and availability truth
- [ ] Parity breadcrumbs, deterministic tests, UAT commands, and no-claim guardrails

### Add After Validation (v2.3.x)

Features to add once the durable coins view and honesty gate work.

- [ ] Cache-pressure and long-chain flush harnesses under injected time and disk-full facts — add once unit transitions and runtime wiring agree
- [ ] Opt-in restart/resume UAT that proves coins best-block continuity without snapshot-only restore — never a default `verify.sh` gate
- [ ] Performance baselines for cache hit/miss and flush duration — add before any performance or production-scale claim

### Future Consideration (later milestones)

- [ ] Prune-mode product behavior (`-prune`, file unlink, `m_have_pruned`, prune locks, `NODE_NETWORK_LIMITED` serving window)
- [ ] Archive-node or production-scale historical serving
- [ ] assumeutxo, assumevalid, and IBD snapshot shortcuts
- [ ] Compact-filter and BIP37 serving
- [ ] Knots/Core `chainstate/` binary import
- [ ] Automatic destructive reindex/repair
- [ ] Public serving defaults, public-network CI, production full-node readiness, production-funds wallet use

## Feature Prioritization Matrix

| Feature | User Value | Implementation Cost | Priority |
|---------|------------|---------------------|----------|
| Disk-backed coins database | HIGH | HIGH | P1 |
| In-memory coins cache (Flush/Sync) | HIGH | HIGH | P1 |
| Cache-flush policy | HIGH | HIGH | P1 |
| Crash-safe batch + interrupted-flush recovery | HIGH | HIGH | P1 |
| Manager-owned coins lifecycle + LoadChainTip | HIGH | HIGH | P1 |
| Payload-present availability gate | HIGH | MEDIUM | P1 |
| Clean refuse when payload absent | HIGH | MEDIUM | P1 |
| Operator persistence/availability evidence | HIGH | MEDIUM | P1 |
| Parity roots and no-claim guardrails | HIGH | MEDIUM | P1 |
| Typed flush state machine / cache-size evidence | MEDIUM | MEDIUM | P2 |
| Fail-closed coins read-error callbacks | MEDIUM | MEDIUM | P2 |
| Long-chain flush / restart UAT harnesses | MEDIUM | HIGH | P2 |
| Prune-mode product behavior | LOW for this milestone | HIGH | P3 |
| assumeutxo / dual-chainstate | LOW for this milestone | HIGH | P3 |
| Archive-node historical serving | LOW for this milestone | HIGH | P3 |

**Priority key:**
- P1: Must have for the v2.3 claim
- P2: Should have once the durable view and honesty gate work
- P3: Nice to have later; not v2.3 table stakes

## Competitor Feature Analysis

| Feature | Pinned Knots `29.3.knots20260210` | Open Bitcoin through v2.2 | Recommended v2.3 approach |
|---------|-----------------------------------|---------------------------|---------------------------|
| Coins persistence | Per-outpoint LevelDB `CCoinsViewDB` under `chainstate/` | Full `ChainstateSnapshot` saved after every connect/reorg | Disk-backed per-outpoint coins on Fjall; snapshot becomes upgrade/compat, not live truth |
| Coins cache | `CCoinsViewCache` with DIRTY/FRESH; `Flush` vs `Sync` | In-memory `Chainstate` cloned for prepare/commit; persist writes the whole snapshot | Cache dirty coins in the shell; flush on policy |
| Flush policy | `FlushStateMode` NONE/IF_NEEDED/PERIODIC/ALWAYS; 50–70 min jitter; LARGE/CRITICAL thresholds | Persist immediately on every mutation | Implement the Knots mode/threshold/order contract without prune-file unlinking |
| Crash recovery | `DB_HEAD_BLOCKS` two-phase write + `ReplayBlocks()` | Last complete snapshot or typed storage recovery blocker | Two-phase markers + replay/fail-closed using stored undo/blocks |
| Manager | `Chainstate` + `ChainstateManager` (active vs optional assumeutxo snapshot) | `ManagedChainstate<S>` over `ChainstateStore` | Fuller single-chainstate manager: coins DB, cache init, flush, LoadChainTip. No snapshot chainstate |
| Block availability | `BLOCK_HAVE_DATA` then successful read; `IsBlockPruned` only if prune has run | Labels exist; `Available` can follow `durable_availability`; missing active history can be `Pruned` | Payload-present gate; `Unavailable` when bytes are absent; reserve `Pruned` |
| Missing-block serve | Silent if no `HAVE_DATA`; disconnect if `HAVE_DATA` but read fails | `lookup_block` miss → `LookupUnavailable`; may have already classified Available | Classify from payload-present; refuse cleanly; do not serve a body you do not have |
| RPC historical read | "pruned data" / "not fully downloaded" / "Block not found on disk" | Operator status labels; no Knots RPC error-string claim required | Honest labels and refuse; exact RPC string parity only if a later requirement names it |

## Sources

### Pinned Knots coins, flush, and manager

- [`packages/bitcoin-knots/src/coins.h`](../../packages/bitcoin-knots/src/coins.h) — `Coin`, DIRTY/FRESH cache entries, `CCoinsView` / `CCoinsViewCache` (`Flush`, `Sync`, `BatchWrite`), `GetHeadBlocks`, `CCoinsViewErrorCatcher`
- [`packages/bitcoin-knots/src/txdb.h`](../../packages/bitcoin-knots/src/txdb.h) and [`txdb.cpp`](../../packages/bitcoin-knots/src/txdb.cpp) — `CCoinsViewDB`, `DB_COIN`/`DB_BEST_BLOCK`/`DB_HEAD_BLOCKS`, two-phase `BatchWrite`, default 64 MiB batch
- [`packages/bitcoin-knots/src/validation.h`](../../packages/bitcoin-knots/src/validation.h) — `FlushStateMode`, `CoinsViews`, `CoinsCacheSizeState`, `Chainstate::FlushStateToDisk` / `ForceFlushStateToDisk` / `LoadChainTip` / `ReplayBlocks`, `ChainstateManager` active vs snapshot roles
- [`packages/bitcoin-knots/src/validation.cpp`](../../packages/bitcoin-knots/src/validation.cpp) — `GetCoinsCacheSizeState` (CRITICAL over cap, LARGE ≥90% or within 10 MiB), `FlushStateToDisk` write order and 50–70 min periodic jitter, `ReplayBlocks` old/new head recovery
- [`packages/bitcoin-knots/src/node/chainstate.h`](../../packages/bitcoin-knots/src/node/chainstate.h) — `LoadChainstate` / `VerifyLoadedChainstate` status taxonomy
- [`packages/bitcoin-knots/src/node/caches.h`](../../packages/bitcoin-knots/src/node/caches.h) — `-dbcache` minimum 4 MiB

### Pinned Knots honest availability

- [`packages/bitcoin-knots/src/chain.h`](../../packages/bitcoin-knots/src/chain.h) — `BLOCK_HAVE_DATA`, `BLOCK_HAVE_UNDO`
- [`packages/bitcoin-knots/src/node/blockstorage.h`](../../packages/bitcoin-knots/src/node/blockstorage.h) and [`blockstorage.cpp`](../../packages/bitcoin-knots/src/node/blockstorage.cpp) — `ReadBlock`/`ReadRawBlock`, `IsBlockPruned` (`m_have_pruned && !HAVE_DATA && nTx > 0`), `CheckBlockDataAvailability`
- [`packages/bitcoin-knots/src/net_processing.cpp`](../../packages/bitcoin-knots/src/net_processing.cpp) — getdata serve: require `HAVE_DATA`, then read; silent if absent; disconnect on read failure; prune-mode `getblocks` stops when data is gone
- [`packages/bitcoin-knots/src/rpc/blockchain.cpp`](../../packages/bitcoin-knots/src/rpc/blockchain.cpp) — `CheckBlockDataAvailability` / `GetBlockChecked` refuse strings

### Existing Open Bitcoin surfaces this milestone must reuse

- [`docs/parity/catalog/chainstate.md`](../../docs/parity/catalog/chainstate.md) — shipped UTXO/undo/reorg snapshot slice; known gap: disk-backed coins, cache-flush, assumeutxo, full manager
- [`docs/parity/checklist.md`](../../docs/parity/checklist.md) — `chainstate` row: disk-backed coins, cache-flush, assumeutxo, and full manager remain outside the current slice
- [`docs/parity/production-claim-boundary.md`](../../docs/parity/production-claim-boundary.md) — archive-node and production-scale historical serving stay deferred
- [`packages/open-bitcoin-node/src/chainstate.rs`](../../packages/open-bitcoin-node/src/chainstate.rs) — `ManagedChainstate` persists a full snapshot after every connect/disconnect/reorg
- [`packages/open-bitcoin-network/src/block_serving.rs`](../../packages/open-bitcoin-network/src/block_serving.rs) — existing status labels and serve-gate vocabulary
- [`packages/open-bitcoin-node/src/network/inventory.rs`](../../packages/open-bitcoin-node/src/network/inventory.rs) — current `durable_availability` / `Pruned` classification gap
- [`.planning/PROJECT.md`](../PROJECT.md) — v2.3 target features and locked out-of-scope list

## Confidence and Open Questions

| Area | Confidence | Remaining decision |
|------|------------|-------------------|
| Knots coins DB + Flush/Sync + two-phase heads | HIGH | Choose Fjall key layout equivalent to `C`/`B`/`H`; do not import LevelDB files |
| Flush thresholds and 50–70 min jitter | HIGH | Inject time and jitter in tests; exact allocator bytes need not match C++ |
| Single-chainstate manager vs assumeutxo dual-chainstate | HIGH | No product decision inside v2.3: one active chainstate only |
| Honest availability vs prune labels | HIGH | Emit `Unavailable` for missing payloads; reserve `Pruned` until a prune-mode milestone |
| Exact RPC error-string parity for missing blocks | MEDIUM | Status labels are table stakes; Knots RPC wording only if requirements name it |
| Whether interrupted-flush replay must succeed or may fail closed | MEDIUM | Knots replays when heads are a 2-element vector; Open Bitcoin may fail closed if bodies/undo are missing, but must not pretend the coins DB is consistent |
| Snapshot codec retirement vs dual-write | MEDIUM | Requirements should say when the full `ChainstateSnapshot` stops being live truth |

---
*Feature research for: Bitcoin chainstate durability and honest historical availability*
*Researched: 2026-08-29*
