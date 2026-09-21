# Feature Research

**Domain:** Bitcoin Knots-aligned prune-mode product behavior
**Milestone:** Open Bitcoin v2.4 Prune-Mode Product Behavior
**Baseline:** Bitcoin Knots `29.3.knots20260210`
**Researched:** 2026-09-21
**Confidence:** HIGH — pinned Knots prune/blockstorage/validation/net/RPC/wallet sources plus current Open Bitcoin availability and wallet-rescan code

## Scope Decision

v2.4 is a **bounded-disk full-node** milestone on top of shipped v2.3 durability and honest availability. It is not an archive-node, assumeutxo, filter-serving, or production-readiness milestone.

Knots already separates three facts that Open Bitcoin must keep distinct:

1. **Prune mode** (`IsPruneMode` / `m_prune_mode`) is a startup product setting from `-prune`: `0` off, `1` manual (`PRUNE_TARGET_MANUAL`), `>=550` MiB automatic target (`ParsePruneOption` in `node/blockmanager_args.cpp`; floor `MIN_DISK_SPACE_FOR_BLOCK_FILES` in `validation.h`).
2. **Have-ever-pruned** (`m_have_pruned`, disk flag `prunedblockfiles`) is set only when prune actually queued files for deletion (`FlushStateToDisk` in `validation.cpp`). `IsBlockPruned` requires `m_have_pruned && !(BLOCK_HAVE_DATA) && nTx > 0` (`node/blockstorage.cpp`).
3. **Payload absence** without that prune fact is not "pruned." Open Bitcoin v2.3 already serves/reports `Available` only when bytes exist and refuses as `Unavailable` otherwise (`HAVL-01`/`HAVL-02`). Production inventory must not inject `BlockServingDataAvailability::Pruned` until a real delete happened.

**Already built (do not re-list as new table stakes):** disk-backed coins, typed flush, single-chainstate restart from coins best-block, honest payload-present availability, default-off block serving / compact relay / bounded tx relay, package/mempool policy, terminal operator surfaces.

**v2.3 debt in scope as first cutover:** wallet rescan still reads leftover `ChainstateSnapshot` bytes (`WalletRescanRuntime::required_chainstate_snapshot` → `load_chainstate_snapshot`). Chainstate restart does not. Closing that before unlink prevents prune from resurrecting snapshot-as-truth.

## Feature Landscape

### Table Stakes (Users Expect These)

Features operators and peers assume once this milestone claims Knots-aligned `-prune`. Missing these = the node either cannot delete old blocks safely or lies about why data is gone.

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| `-prune` product modes and target | Operators configure disk budget the same way as Knots | MEDIUM | Knots: `-prune=0` disable; `=1` manual via `pruneblockchain`; `>=550` MiB automatic (`init.cpp` help + `ParsePruneOption`). Automatic target is bytes on disk for blk+rev; refuse below 550 MiB. Depends on durable coins so tip/UTXO survive after deletes. |
| Height windows before delete | Peers and reorg safety require a recent block window | MEDIUM | Knots keeps at least `MIN_BLOCKS_TO_KEEP` (288) from tip (`validation.h` / `GetPruneRange`). Automatic prune also waits until chain height > `PruneAfterHeight()` (mainnet 100000 in `kernel/chainparams.cpp`). Manual `pruneblockchain` clamps to `chainHeight - MIN_BLOCKS_TO_KEEP` and refuses short chains. |
| Block-file prune + unlink | Disk savings are the product; clearing an index bit without deleting files is not prune | HIGH | `FindFilesToPrune` / `FindFilesToPruneManual` → `PruneOneBlockFile` clears `BLOCK_HAVE_DATA`/`BLOCK_HAVE_UNDO` and file positions → flush block index → `UnlinkPrunedFiles` removes `blk`/`rev` (`node/blockstorage.cpp`, `validation.cpp`). Whole files, not arbitrary per-block holes. Depends on honest availability so post-delete serves refuse correctly. |
| Persist `m_have_pruned` / `prunedblockfiles` | Restart and peers must know pruning has occurred | MEDIUM | First non-empty prune set writes flag and sets `m_have_pruned` (`validation.cpp`). Load reads flag (`LoadBlockIndexDB`). Leaving prune after have-pruned requires `-reindex` (`node/chainstate.cpp`). |
| Prune locks that keep files | Indexes and operators must pin height ranges so unlink does not destroy required history | HIGH | `m_prune_locks` + `DoPruneLocksForbidPruning` with `PRUNE_LOCK_BUFFER` (10) (`blockstorage.cpp`/`.h`). Knots exposes `listprunelocks` / `setprunelock` (`rpc/blockchain.cpp`). Locks durable in block tree DB. Without locks, later compact-filter/index work cannot retain ranges. |
| `NODE_NETWORK_LIMITED` advertisement and serve limits | Pruned nodes must not claim full historical serving | HIGH | Default local services start as `NODE_NETWORK_LIMITED \| NODE_WITNESS`; non-prune path adds `NODE_NETWORK` (`init.cpp`). Prune mode never adds `NODE_NETWORK`. Serve path ignores (and disconnects ordinary peers for) requests deeper than `NODE_NETWORK_LIMITED_MIN_BLOCKS` (288) + 2 buffer (`net_processing.cpp` `ProcessGetBlockData`). BIP159 semantics in `protocol.h`. Depends on opt-in block serving already shipped. |
| P2P refuse when data gone or below limited window | Peers must not hang waiting for deleted history | MEDIUM | No `BLOCK_HAVE_DATA` → silent non-serve; read-after-prune race → disconnect (`ProcessGetBlockData`). `getblocks` stops on pruned/too-old (`net_processing.cpp` ~4104–4106). Limited-threshold ignore is separate from payload absence. |
| Emit `Pruned` only after real delete; else `Unavailable` | Operators must not confuse corruption/missing bytes with prune product behavior | MEDIUM | Knots `IsBlockPruned` needs `m_have_pruned`. RPC: `"Block not available (pruned data)"` vs `"Block not available (not fully downloaded)"` (`rpc/blockchain.cpp`). Open Bitcoin already maps `Pruned`/`Unavailable` labels but production inventory only emits `Unavailable` for missing payload (`inventory.rs`; tests forbid injecting `Pruned`). Wire `Pruned` only when unlink + have-pruned facts are true — fulfills reserved `HAVL-02`. |
| Operator prune surfaces | Operators need to see mode, height, and trigger manual prune | MEDIUM | `getblockchaininfo`: `pruned`, `pruneheight` (last pruned + 1), `automatic_pruning`, `prune_target_size` (`rpc/blockchain.cpp`). `pruneblockchain` requires prune mode; irreversible local delete. Align Open Bitcoin status/RPC/CLI/dashboard fields to the same vocabulary. |
| Wallet leftover-snapshot cutover before first unlink | Rescan must not reconstruct UTXOs from a blob prune did not delete | MEDIUM | Current `WalletRescanRuntime` still requires `load_chainstate_snapshot()` leftover bytes. Coins path already treats leftover as non-authoritative. Cutover first (candidate Phase 146) so rescan uses durable coins / block payloads only. After have-pruned, Knots refuses rescan past prune height (`wallet/rpc/transactions.cpp`: "Can't rescan beyond pruned data…"). |
| Parity roots and no-claim guardrails | Core value requires auditable Knots behavior and scoped claims | LOW | Cite `node/blockstorage.cpp`/`.h`, `node/blockmanager_args.cpp`, `validation.cpp` (`FlushStateToDisk`, `GetPruneRange`, `PruneAndFlush`), `net_processing.cpp` (limited serve), `protocol.h` BIP159, `rpc/blockchain.cpp`, wallet prune refusal. Keep archive, assumeutxo, BIP37, public defaults, production claims deferred. |

### Differentiators (Competitive Advantage)

Features that set Open Bitcoin apart. Not required for Knots-observable prune, but valuable because they make the same contract safer and auditable.

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| Typed prune decision state machine | Makes height-window, lock, target-bytes, and unlink ordering reviewable without tracing C++ flush | HIGH | Pure transitions take tip height, prune target, file usage facts, lock ranges, and have-pruned; shell unlinks and persists flags. Matches functional-core / imperative-shell. |
| Explicit `Pruned` vs `Unavailable` vs `Available` facts | Prevents "missing bytes" from being marketed as prune mode | LOW | Extend v2.3 presence facts with `have_pruned` / `deleted_by_prune`. Status already reserved `BlockServingDataAvailability::Pruned`. |
| Sanitized prune-event evidence | Operators can prove a delete happened without dumping file paths into support noise | MEDIUM | Low-cardinality counters: files considered, files unlinked, last prune height, lock-blocked skips, limited-serve refusals. Reuse CSOBS-style sanitization. |
| Fail-closed snapshot cutover as phase-0 gate | Stops prune from amplifying v2.3 debt | MEDIUM | Refuse rescan that would read leftover snapshot; migrate or ignore leftover for wallet the way coins restart already does. |
| Claim taxonomy in operator copy | Prevents "we prune" from being read as archive or production-ready | LOW | Distinguish `prune_mode`, `have_pruned`, `node_network_limited` from deferred `archive_serving`, `assumeutxo`, `production_ready`. |

### Anti-Features (Commonly Requested, Often Problematic)

Features that seem good but create problems. Treat as out of scope for v2.4, not table stakes.

| Feature | Why Requested | Why Problematic | Alternative |
|---------|---------------|-----------------|-------------|
| Emitting `Pruned` without a real file delete | Reuses a familiar label for any missing payload | Violates Knots `IsBlockPruned` and shipped `HAVL-02`. Trains operators to distrust availability labels | Keep `Unavailable` until `m_have_pruned` and unlink actually cleared `HAVE_DATA` |
| Manual `rm` of blk files / fake prune | "Disk savings without product work" | Leaves index/`HAVE_DATA` inconsistent; no `m_have_pruned`; peers and RPC lie or crash | Implement `PruneOneBlockFile` + `UnlinkPrunedFiles` path only |
| Archive-node / production-scale historical serving | Opposite of prune: keep and serve everything | Opposite operator problem; production claim boundary (FUT-19) | Serve when bytes exist; refuse otherwise; no archive claim |
| assumeutxo / dual chainstate | Faster IBD while pruning | Second chainstate, snapshot activation, and prune-budget split (`GetPruneTargetForChainstate`) are a separate milestone (FUT-21). v2.3 kept one active chainstate on purpose | Single chainstate prune only |
| BIP37 bloom serving | Historical SPV lookup on a pruned node | Privacy/DoS surface; deferred with compact filters (FUT-20 BIP37-out) | Later BIP157/158 after prune retention design; no BIP37 |
| Compact-filter serving in the same milestone | Light clients on pruned history | Needs prune-lock retention design; mixing it into first prune ship conflates two products | v2.5 candidate after prune locks exist |
| Knots/Core LevelDB `chainstate/` import | Drop-in datadir | Different engine/schema; migration must stay dry-run-first (FUT-22) | Own coins schema; detection-only migration |
| Automatic destructive reindex to "fix" prune | Recover from bad deletes | Hidden datadir mutation; fail-closed recovery is the shipped contract (FUT-23) | Diagnose; require explicit later repair plan |
| Public relay/serving defaults or public-network CI gates | Demo prune on mainnet by default | Violates activation and v1.8 claim gates (FUT-24/25) | Keep opt-in activation; deterministic verify stays hermetic |
| Production readiness / production-funds wallet | "Prune means ready for money" | Durable coins + prune do not satisfy v1.8 evidence gates (FUT-26) | Keep production claims deferred |
| `-txindex` with prune | Convenient historical tx lookup | Knots marks `-prune` incompatible with `-txindex` (`init.cpp` help) | Do not enable txindex under prune mode |
| Guaranteeing re-fetch of every pruned block | Soften irreversibility | `getblockfrompeer` is best-effort and tip-constrained in prune mode; local delete remains irreversible | Document irreversibility; optional later peer-refetch scope |

## Feature Dependencies

```text
v2.3 durable per-outpoint coins + typed flush
└──requires──> Wallet leftover-snapshot cutover (Phase 146 / first prune phase)
                 └──requires──> -prune modes + height windows (MIN_BLOCKS_TO_KEEP / PruneAfterHeight)
                                  ├──requires──> File select + PruneOneBlockFile + UnlinkPrunedFiles
                                  │                ├──requires──> Persist m_have_pruned / prunedblockfiles
                                  │                │                └──requires──> Emit Pruned only when have_pruned + deleted
                                  │                └──requires──> Prune locks (keep windows)
                                  └──requires──> NODE_NETWORK_LIMITED advertise + serve depth limits
                                                   └──enhances──> P2P/RPC refuse strings aligned to Knots

v2.3 honest payload-present availability ──requires──> Pruned/Unavailable split above
Existing default-off block serving ──enhances──> Limited-window serving under prune

Archive serving ──conflicts──> Prune file deletion
assumeutxo dual chainstate ──conflicts──> Single-chainstate prune budget (defer together)
BIP37 / compact filters ──conflicts──> Unscoped historical serve without prune locks
Fake Pruned label ──conflicts──> HAVL-02 honesty contract
```

### Dependency Notes

- **Wallet snapshot cutover before unlink:** If rescan still hydrates from leftover snapshot blobs, deleting blk/rev does not remove the alternate UTXO truth those blobs carry. Coins restart already ignores leftover as authoritative; wallet must match before first delete.
- **Honest availability before `Pruned` wiring:** v2.3 made missing bytes `Unavailable`. Prune adds the only legitimate path to `Pruned`.
- **Durable coins before disk-saving claims:** Tip and UTXO must restart from coins best-block after blk/rev unlink; otherwise prune is unsafe.
- **Prune locks before compact filters:** Filters that need undo/history need lock ranges; do not ship filters in v2.4.
- **`NODE_NETWORK_LIMITED` after have-pruned path exists:** Advertising limited services without real delete semantics still misrepresents peers if the node claims full history another way; couple flag changes to prune mode startup like Knots.

## MVP Definition

### Launch With (v2.4)

Minimum viable Knots-aligned prune product for the single active chainstate.

- [ ] Wallet leftover-snapshot cutover — remove snapshot-as-truth before any unlink
- [ ] `-prune` 0 / 1 / >=550 MiB modes with height windows (`MIN_BLOCKS_TO_KEEP`, `PruneAfterHeight`)
- [ ] Automatic and manual file selection, index clear, blk/rev unlink, persist `m_have_pruned`
- [ ] Prune locks that can forbid deleting locked height ranges
- [ ] `NODE_NETWORK_LIMITED` (no `NODE_NETWORK` in prune mode) and serve-depth refusal
- [ ] `Pruned` only after real delete; `Unavailable` when bytes missing without prune
- [ ] Operator surfaces: prune mode / pruneheight / manual `pruneblockchain` (or equivalent) + sanitized evidence
- [ ] Parity breadcrumbs and no-claim guardrails (no archive / assumeutxo / BIP37 / public defaults / production)

### Add After Validation (v2.4.x / adjacent)

- [ ] Full Knots `listprunelocks` / `setprunelock` operator parity if MVP ships locks only via index adapters
- [ ] `-pruneduringinit` temporary target during IBD (`blockmanager_args.cpp`) — useful, not required to prove product prune
- [ ] Wallet import/rescan refusal strings matching Knots once have-pruned is live
- [ ] `ScanAndUnlinkAlreadyPrunedFiles` restart cleanup for zero-size files after crash mid-unlink

### Future Consideration (later milestones)

- [ ] Compact-filter serving with prune-aware locks (FUT-20 without BIP37)
- [ ] assumeutxo / dual chainstate and prune-budget split (FUT-21)
- [ ] Archive-node product mode (FUT-19)
- [ ] Public defaults, public-network CI, production readiness (FUT-24–26)

## Feature Prioritization Matrix

| Feature | User Value | Implementation Cost | Priority |
|---------|------------|---------------------|----------|
| Wallet leftover-snapshot cutover | HIGH | MEDIUM | P1 |
| `-prune` modes + height windows | HIGH | MEDIUM | P1 |
| File unlink + `m_have_pruned` | HIGH | HIGH | P1 |
| `Pruned` only after real delete | HIGH | MEDIUM | P1 |
| `NODE_NETWORK_LIMITED` advertise + serve limits | HIGH | HIGH | P1 |
| Prune locks | HIGH | HIGH | P1 |
| Operator prune RPC/status evidence | HIGH | MEDIUM | P1 |
| Typed prune state machine + evidence | MEDIUM | HIGH | P2 |
| `-pruneduringinit` | MEDIUM | MEDIUM | P2 |
| Compact filters | MEDIUM | HIGH | P3 |
| assumeutxo dual chainstate | MEDIUM | HIGH | P3 |
| Archive serving | LOW (conflicts) | HIGH | P3 (anti) |
| Fake `Pruned` / manual rm | LOW (harmful) | LOW | Anti |

**Priority key:**
- P1: Must have for v2.4 launch
- P2: Should have once P1 is solid
- P3: Future consideration / explicit anti-feature

## Competitor Feature Analysis

| Feature | Bitcoin Knots 29.3 (pinned) | Open Bitcoin today (post-v2.3) | Our v2.4 approach |
|---------|----------------------------|--------------------------------|-------------------|
| `-prune` target / manual | `ParsePruneOption`; auto >=550 MiB; `pruneblockchain` | Not implemented | Match Knots modes on single chainstate |
| Height keep window | `MIN_BLOCKS_TO_KEEP` = 288; `PruneAfterHeight` | N/A | Same constants for in-scope nets |
| File deletion | `PruneOneBlockFile` + `UnlinkPrunedFiles` | Stores blocks; no prune unlink | Implement unlink; do not fake labels |
| `m_have_pruned` | Disk flag `prunedblockfiles` | Absent | Persist and gate `IsBlockPruned` equivalent |
| Prune locks | `m_prune_locks` + RPCs | Absent | Ship locks with prune; filters later |
| Service flags | Prune stays `NODE_NETWORK_LIMITED` only | Serving exists; limited flag not prune-tied | Advertise limited under prune mode |
| Missing payload label | `IsBlockPruned` vs not-fully-downloaded | `Unavailable` only in production path; `Pruned` reserved | Wire `Pruned` only after delete |
| Wallet vs leftover snapshot | N/A (no OB snapshot debt) | Rescan still reads leftover snapshot | Cut over first phase |
| Archive / assumeutxo / BIP37 | Separate features | Explicitly deferred | Keep anti-features |

## Sources

- Pinned Knots `packages/bitcoin-knots/src/node/blockstorage.cpp` / `blockstorage.h` — `PruneOneBlockFile`, `FindFilesToPrune`, `UnlinkPrunedFiles`, `IsBlockPruned`, `m_have_pruned`, prune locks
- Pinned Knots `packages/bitcoin-knots/src/node/blockmanager_args.cpp` — `ParsePruneOption` (`0` / `1` / MiB target)
- Pinned Knots `packages/bitcoin-knots/src/validation.h` / `validation.cpp` — `MIN_BLOCKS_TO_KEEP`, `MIN_DISK_SPACE_FOR_BLOCK_FILES`, flush-triggered prune, `GetPruneRange`, `PruneAndFlush`
- Pinned Knots `packages/bitcoin-knots/src/init.cpp` — `-prune` help, default `NODE_NETWORK_LIMITED`, add `NODE_NETWORK` only when not pruning
- Pinned Knots `packages/bitcoin-knots/src/net_processing.cpp` — `NODE_NETWORK_LIMITED_MIN_BLOCKS` (288), serve-depth disconnect, `getblocks` prune stop
- Pinned Knots `packages/bitcoin-knots/src/protocol.h` — BIP159 `NODE_NETWORK_LIMITED`
- Pinned Knots `packages/bitcoin-knots/src/rpc/blockchain.cpp` — `getblockchaininfo` prune fields, `pruneblockchain`, `listprunelocks` / `setprunelock`, "pruned data" vs "not fully downloaded"
- Pinned Knots `packages/bitcoin-knots/src/wallet/rpc/transactions.cpp` / `backup.cpp` — rescan/import refusals when pruned
- Pinned Knots `packages/bitcoin-knots/src/node/chainstate.cpp` — cannot leave prune after `m_have_pruned` without `-reindex`
- Open Bitcoin `packages/open-bitcoin-network/src/block_serving.rs` — `Pruned` reserved; missing payload is `Unavailable`
- Open Bitcoin `packages/open-bitcoin-node/src/network/inventory.rs` — production path emits `Unavailable`, not `Pruned`
- Open Bitcoin `packages/open-bitcoin-node/src/sync/wallet_rescan.rs` — still requires leftover chainstate snapshot
- Open Bitcoin `.planning/milestones/v2.3-REQUIREMENTS.md` — `HAVL-02`, `FUT-18`
- Open Bitcoin `.planning/reports/NEXT-MILESTONE-CANDIDATES.md` — v2.4 scope selection

---
*Feature research for: Open Bitcoin v2.4 prune-mode product behavior*
*Researched: 2026-09-21*
*Baseline: Bitcoin Knots 29.3.knots20260210*
