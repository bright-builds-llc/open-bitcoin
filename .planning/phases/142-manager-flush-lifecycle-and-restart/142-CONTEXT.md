---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 142-2026-09-06T16-23-52
generated_at: 2026-09-06T16:24:01.047Z
---

# Phase 142: Manager Flush Lifecycle and Restart - Context

**Gathered:** 2026-09-06
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

One manager owns coins-database init, health-check, cache init, CanFlush-style
readiness, ordered flush points, interrupted-flush recovery, and same-datadir
restart from durable coins best-block.

This phase delivers MGR-01, MGR-02, and FLUSH-02 only. After connect/reorg the
manager asks `decide_flush(IfNeeded)`; on injected ticks it asks
`decide_flush(Periodic)`; on shutdown it asks `decide_flush(Always)`. A write
runs block, undo, and index before coins. A mid-flush crash is recovered by
replay from stored undo and block bodies, or fails closed without inventing a
consistent tip. `persist_progress` no longer rewrites the full UTXO snapshot
as live truth.

This phase does not own honest payload-present availability (Phase 143),
operator flush/recovery evidence surfaces (Phase 144), or parity/no-claim
closeout (Phase 145). It does not add prune/archive modes, assumeutxo,
dual-chainstate cache split, LevelDB/Core `chainstate/` import, automatic
destructive reindex, public defaults, or production-readiness claims.

</domain>

<decisions>
## Implementation Decisions

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

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase and requirement
- `.planning/ROADMAP.md` — Phase 142 goal, MGR-01, MGR-02, FLUSH-02,
  success criteria, and research flags (crash-loss window, cache-byte
  defaults, replay vs fail-closed)
- `.planning/REQUIREMENTS.md` — MGR-01, MGR-02, FLUSH-02 wording; FUT-21
  through FUT-23 stay deferred
- `.planning/PROJECT.md` — Functional-core / no rust-bitcoin / Fjall-in-shell
  constraints and v2.3 storage-first scope
- `.planning/phases/139-coins-view-cache-contract-and-engine-apply/139-CONTEXT.md`
  — Locked Flush vs Sync algebra and two-phase manager lifecycle
- `.planning/phases/140-pure-flush-policy-and-typed-decisions/140-CONTEXT.md`
  — Locked `decide_flush` / `decide_recovery`; D-07 resampling; D-10
  unpinned defaults; D-21 replay left for this phase; D-22 leftover
  write-site freeze
- `.planning/phases/141-durable-fjall-coins-adapter/141-CONTEXT.md`
  — Coins keyspace, `H`/`B` protocol, D-09 fail-closed-this-phase, D-19
  write-site deferral to this phase
- `.planning/phases/141-durable-fjall-coins-adapter/141-REVIEW.md` —
  WR-02 interrupted-`H` shadowed by leftover-empty; IN-01 Display-text
  remap

### Milestone research
- `.planning/research/ARCHITECTURE.md` — Manager owns flush I/O; policy
  stays injected-fact core
- `.planning/research/STACK.md` — 450 MiB / 4 MiB / 8 MiB defaults,
  50–70 minute jitter, `PersistMode`, no new production crates
- `.planning/research/PITFALLS.md` — Flush order, cache hit ≠ durable,
  interrupted-flush protocol, leftover snapshot as truth
- `.planning/research/SUMMARY.md` — Phase 142 owns manager wiring,
  replay, and persist cutover

### Existing apply surface
- `packages/open-bitcoin-chainstate/src/coins/flush.rs` — `decide_flush`,
  `decide_recovery`, cache-size classification
- `packages/open-bitcoin-chainstate/src/engine.rs` — Still
  `CoinsCache<MemoryCoinsView>` in production apply
- `packages/open-bitcoin-node/src/chainstate.rs` — `ManagedChainstate::persist`
  still `save_snapshot`
- `packages/open-bitcoin-node/src/sync/runtime_state.rs` —
  `persist_progress` leftover dual-write (D-19 comment)
- `packages/open-bitcoin-node/src/storage/coins_view.rs` —
  `FjallCoinsView::batch_write` two-phase `H`/`B`
- `packages/open-bitcoin-node/src/storage/fjall_store/coins.rs` —
  `hydrate_chainstate_for_open`, `seed_coins_from_snapshot`, schema 2
- `packages/open-bitcoin-node/src/sync.rs` — Runtime open hydrates via
  leftover-era snapshot helper
- `packages/open-bitcoin-node/src/recovery.rs` — Operator
  `classify_recovery` mapping for `InterruptedWrite`

### Knots anchors
- `packages/bitcoin-knots/src/validation.cpp` — `FlushStateToDisk`,
  `GetCoinsCacheSizeState`, `ReplayBlocks`, `DATABASE_WRITE_INTERVAL`
- `packages/bitcoin-knots/src/node/chainstate.cpp` — Manager init /
  health-check / cache-init / CanFlush
- `packages/bitcoin-knots/src/node/blockstorage.cpp` — Block/undo flush
  before coins
- `packages/bitcoin-knots/src/txdb.cpp` — `CCoinsViewDB::BatchWrite`
  already ported in Phase 141
- `packages/bitcoin-knots/src/coins.h` / `packages/bitcoin-knots/src/coins.cpp`
  — `Flush` vs `Sync` algebra already ported in Phase 139

### Architecture standards
- `standards/core/architecture.md` — Functional core / imperative shell
- `.planning/ARCHITECTURE.md` — Crate boundaries; chainstate stays pure
- `docs/parity/catalog/chainstate.md` — Known disk-backed / manager gap
  this phase starts to close
- `docs/parity/source-breadcrumbs.json` — Required breadcrumbs on new
  first-party Rust sources

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `decide_flush` / `decide_recovery` — Manager executes these; it does
  not reimplement policy
- `CoinsCache::flush` / `CoinsCache::sync` — Write-kind already mapped
- `FjallCoinsView::batch_write` — Two-phase `H`/`B` coins persist
- `save_undo` / `load_undo` / `save_block` / `load_block` — Replay inputs
- `StorageError::InterruptedWrite` — Fail-closed typed error to keep
- Phase 140/141 leftover-write guard tests — Flip when cutover lands

### Established Patterns
- Functional-core decisions from injected facts; I/O stays in
  `open-bitcoin-node`
- Single active chainstate; `MemoryChainstateStore` for tests
- Fail closed on schema mismatch, decode failure, and inconsistent
  `head_blocks`
- Parity breadcrumbs required on new first-party Rust sources
- Architecture-policy checks fail if Fjall or clocks enter
  `open-bitcoin-chainstate`

### Integration Points
- `ManagedChainstate::persist` / `commit_prepared_*` — IfNeeded flush
  instead of snapshot dump
- `DurableSyncRuntime::persist_progress` — Stop leftover snapshot as
  live truth; credit coins best-block
- `DurableSyncRuntime::open_with_runtime_activation` / store open —
  Init, recover, attach `FjallCoinsView`, report CanFlush
- Daemon/RPC shell — Periodic tick + Always on shutdown
- Network lifecycle authority — Connect/reorg commit still two-phase;
  durability is the manager flush, not the snapshot blob

</code_context>

<specifics>
## Specific Ideas

Yolo discuss locked the ROADMAP research flags to Knots-shaped manager
behavior: periodic-plus-shutdown crash-loss (not every-connect dumps),
shell-injected Knots-aligned cache defaults, and replay-when-bodies-exist
else fail-closed. Fix 141-REVIEW WR-02/IN-01 as part of making replay
see the interrupted marker.

</specifics>

<deferred>
## Deferred Ideas

- Honest payload-present availability and reserved `Pruned` label —
  Phase 143
- Status / RPC / CLI / dashboard / metrics / logs / support flush
  evidence — Phase 144
- Parity-root closeout and no-claim guardrails — Phase 145
- Prune/archive product modes, assumeutxo, dual-chainstate, LevelDB
  import, automatic destructive reindex, public defaults, production
  readiness — FUT-18 through FUT-26

None of these were folded into Phase 142.

</deferred>

---

*Phase: 142-manager-flush-lifecycle-and-restart*
*Context gathered: 2026-09-06*
