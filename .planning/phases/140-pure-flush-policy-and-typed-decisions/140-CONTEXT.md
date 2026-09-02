---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 140-2026-09-02T02-14-08
generated_at: 2026-09-02T02:14:23.875Z
---

# Phase 140: Pure Flush Policy and Typed Decisions - Context

**Gathered:** 2026-09-01
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Flush and recovery decisions are a typed, injectable policy the later
manager can execute without clocks or I/O in core.

This phase delivers FLUSH-01 and MGR-03 only: `decide_flush` from injected
cache-size, time, disk-space, and mode facts; first-class disk-space
refusal; OK / LARGE / CRITICAL classification; distinct Flush vs Sync
decisions; and a sketched I/O-free `RecoveryDecision` from injected marker
facts. `open-bitcoin-chainstate` stays I/O-free.

This phase does not persist per-outpoint coins, cut over
`persist_progress` off the leftover snapshot blob, own manager
init/restart, implement interrupted-flush replay, wire IfNeeded / Periodic
/ Always call sites against snapshot persist, change serving honesty, or
claim prune / archive / assumeutxo / production readiness. `FlushForPrune`
is not implemented. `None` is observability only.

</domain>

<decisions>
## Implementation Decisions

### Decision outcome vocabulary

- **D-01:** One `decide_flush` returns a single enum-first `FlushDecision`.
  Adapters match-and-execute. They must not recompose write-kind, empty-cache,
  refusal, cache-size, or reason from split predicates.
- **D-02:** Encode disk-space refusal as its own variant (for example
  `RefuseDiskSpace`) so a write and a refusal cannot coexist. Do not use a
  parallel `disk_ok` flag on a write decision.
- **D-03:** The same decision value carries classified
  `CoinsCacheSizeState` (`Ok` / `Large` / `Critical`) and a last-flush-reason
  enum (`None` / `Needed` / `Periodic` / `Always` / `FailedDisk` or equivalent).
  Phase 144 reports these fields; do not invent a second derivation later.
- **D-04:** `classify_cache_size` may exist as a helper (Knots
  `GetCoinsCacheSizeState` split), but it is not a manager-composed public
  contract. Refusal is not a separate predicate the shell must remember.
- **D-05:** `empty_cache` is implied by write-kind once Phase 139 algebra
  is reused: `Flush` empties, `Sync` retains unspent. Do not add a third
  write-kind. Prefer an enum `None | Flush | Sync` over a redundant
  `empty_cache: bool` that can disagree with the write.

### Injected facts vs computed classification

- **D-06:** The shell injects occupancy facts: cache bytes, cache-byte
  limit, and mempool leftover bytes. Policy computes total space as
  `cache_limit + max(0, mempool_leftover)` and classifies:
  CRITICAL when `cache_bytes > total`; LARGE when
  `cache_bytes > max((9 * total) / 10, total - 10 MiB)`; otherwise OK.
  Pin the 90% and 10 MiB constants as first-party values. Do not read a
  clock, allocator, or disk in core.
- **D-07:** The shell injects `now` and an already-jittered `next_write`.
  Policy computes `periodic_due = now >= next_write`. Do not inject
  last-flush plus 50–70 minute window bounds, and do not sample jitter or
  RNG in this crate. Phase 142 owns resampling after a successful write.
- **D-08:** Inject `memory_pressure: bool` for Knots
  `SystemNeedsMemoryReleased()`. Do not add an OS-pressure crate or read
  host memory from core.
- **D-09:** Inject disk-free bytes and cache entry count (or an already
  computed required-guard). Policy refuses when
  `free_bytes < 48 * 2 * 2 * entry_count` (Knots coins-write guard). Check
  refusal only after the policy has decided a write would otherwise happen.
- **D-10:** Do not pin 450 MiB `DEFAULT_KERNEL_CACHE`, 4 MiB min dbcache,
  or 8 MiB coins-DB cap in this phase. Those defaults and the crash-loss
  window stay a Phase 142 research flag. Phase 140 tests inject limits.

### Flush vs Sync mapping

- **D-11:** Lock the pinned Knots `FlushStateToDisk` mapping in
  `decide_flush`. Phase 142 executes the returned write-kind; it does not
  re-choose Flush vs Sync.
- **D-12:** `Always` → `Flush`.
- **D-13:** `Periodic` + LARGE or CRITICAL → `Flush`, even when
  `periodic_due` is false (`fCacheLarge` is Periodic-only).
- **D-14:** `Periodic` + OK + `periodic_due` → `Sync` (keep the working
  set warm).
- **D-15:** `IfNeeded` + CRITICAL, or `IfNeeded` + `memory_pressure` →
  `Flush`.
- **D-16:** `IfNeeded` + LARGE without CRITICAL or pressure → `None`.
  LARGE is not a write trigger under IfNeeded.
- **D-17:** `None` mode never writes and never refuses for disk. It may
  still classify cache-size for observability. No prune-hook implementation.
- **D-18:** Disk-space refusal precedes any Flush or Sync outcome. If a
  write would happen and the guard fails, return `RefuseDiskSpace` rather
  than a write.
- **D-19:** Do not implement `FlushForPrune`, `PruneAndFlush`, or file
  unlink. `fFlushForPrune` stays false.

### Recovery types and wiring boundary

- **D-20:** Add a thin I/O-free `RecoveryDecision` from injected marker
  facts so MGR-03 is a typed machine, not flush-only: heads empty
  (consistent), one-element, two-element, or any other count / inconsistent.
  Do not implement `ReplayBlocks`, undo replay, or fail-closed I/O.
- **D-21:** Keep replay-versus-fail-closed unresolved. The sketch names
  marker facts; Phase 142 chooses the executable recovery rule after
  `head_blocks` exist.
- **D-22:** Do not change `ManagedChainstate::persist`,
  `DurableSyncRuntime::persist_progress`, or snapshot-blob write sites.
  Do not wire IfNeeded / Periodic / Always call sites against leftover
  persist. A snapshot write is not a flush-policy durability proof.
- **D-23:** Place new types in `open-bitcoin-chainstate` as
  `coins/flush.rs`. Do not put Fjall, `std::fs`, Tokio, or a wall clock
  in this crate.

### Claude's Discretion

- Exact `FlushDecision` / `RecoveryDecision` variant names and whether
  cache-size and reason live on every variant or on a shared payload,
  provided D-02 (refuse cannot coexist with a write) holds.
- Whether `classify_cache_size` is `pub` or crate-private.
- Exact last-flush-reason spelling as long as Phase 144 can map
  needed / periodic / always / failed_disk without a second policy.
- How injected times are represented (`u64` millis, a pure `PolicyTime`
  newtype) as long as core does not call `Instant::now()`.

### Folded Todos

None — `todo match-phase 140` returned no matches.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase and requirement
- `.planning/ROADMAP.md` — Phase 140 goal, FLUSH-01, MGR-03, success
  criteria, and later-phase ownership for Fjall, manager replay, and
  operator evidence
- `.planning/REQUIREMENTS.md` — FLUSH-01 and MGR-03 wording; FLUSH-02
  stays Phase 142
- `.planning/PROJECT.md` — Functional-core / no rust-bitcoin / Fjall-in-shell
  constraints and v2.3 storage-first scope
- `.planning/phases/139-coins-view-cache-contract-and-engine-apply/139-CONTEXT.md`
  — Locked Flush vs Sync algebra, leftover snapshot persist, and the
  explicit Phase 140 deferral of flush policy

### Milestone research
- `.planning/research/ARCHITECTURE.md` — Pattern 2 injected-fact flush
  policy, `coins/flush.rs` home, Anti-Pattern "clock in `decide_flush`"
- `.planning/research/STACK.md` — Mode table (None / IfNeeded / Periodic /
  Always), LARGE/CRITICAL math, disk guard `48 * 2 * 2 * entry_count`,
  50–70 minute jitter owned by the later manager
- `.planning/research/FEATURES.md` — Knots `FlushStateMode` and
  ALWAYS/LARGE/CRITICAL → Flush, periodic time-due → Sync
- `.planning/research/PITFALLS.md` — Cache hit is not durability; flush
  order and interrupted-flush protocol wait for later phases
- `.planning/research/SUMMARY.md` — Phase 140+ delivers FlushMode,
  CoinsCacheSizeState, and `decide_flush`; `None` is observability only

### Existing apply surface
- `packages/open-bitcoin-chainstate/src/coins.rs` — Module root; add
  `mod flush` here
- `packages/open-bitcoin-chainstate/src/coins/cache.rs` — Existing
  `flush()` empties; `sync()` retains unspent clean
- `packages/open-bitcoin-chainstate/src/error.rs` — Extend only if policy
  construction needs a typed error; do not map disk refusal through
  `MissingCoin`
- `packages/open-bitcoin-node/src/chainstate.rs` — `persist()` still
  `save_snapshot`; do not retarget in this phase
- `packages/open-bitcoin-node/src/sync/runtime_state.rs` —
  `persist_progress` still writes the leftover snapshot; do not cut over

### Knots anchors
- `packages/bitcoin-knots/src/validation.h` — `FlushStateMode`,
  `CoinsCacheSizeState`
- `packages/bitcoin-knots/src/validation.cpp` — `GetCoinsCacheSizeState`
  (CRITICAL over cap, LARGE ≥90% or within 10 MiB), `FlushStateToDisk`
  (`fCacheLarge` Periodic-only, `fCacheCritical` IfNeeded, `fPeriodicWrite`,
  `empty_cache ? Flush() : Sync()`, coins disk-space guard
  `48 * 2 * 2 * GetCacheSize()`)
- `packages/bitcoin-knots/src/coins.h` / `packages/bitcoin-knots/src/coins.cpp`
  — `CCoinsViewCache::Flush` vs `Sync` algebra already ported in Phase 139

### Architecture standards
- `standards/core/architecture.md` — Functional core / imperative shell;
  parse at boundaries
- `.planning/ARCHITECTURE.md` — Crate boundaries; chainstate stays pure
- `docs/parity/catalog/chainstate.md` — Known disk-backed / manager gap
  (do not close the disk gap here)

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `CoinsCache::flush` / `CoinsCache::sync` — Policy selects which one a
  later manager calls; do not change the algebra
- `CoinsCacheFlags` / DIRTY/FRESH overlay — Occupancy facts for later
  byte accounting; Phase 140 may take injected bytes rather than walking
  the map itself
- `ChainstateError` — Add a typed policy/construction error only if
  needed; disk-space refusal is a decision, not an apply error
- `MemoryCoinsView` — Remains the in-memory parent; no Fjall parent here

### Established Patterns
- Functional-core decisions from injected facts (mempool `PolicyTime`,
  not `Instant::now()`)
- Phase 139 child→parent Flush is cache algebra, not this disk policy
- Parity breadcrumbs required on new first-party Rust sources
- Architecture-policy checks fail if Fjall or clocks enter
  `open-bitcoin-chainstate`

### Integration Points
- New `coins/flush.rs` exported from `coins.rs`
- Unit tests live under `coins/tests/` beside the existing algebra tests
- `ManagedChainstate` and `DurableSyncRuntime` compile unchanged
- Engine connect/reorg paths stay on the Phase 139 overlay; they do not
  call `decide_flush` yet

</code_context>

<specifics>
## Specific Ideas

- Match pinned Knots `FlushStateToDisk` boolean split (`fCacheLarge`,
  `fCacheCritical`, `fPeriodicWrite`, `empty_cache`) rather than a
  simplified "any Large writes" rule
- Disk-space refusal is the Knots fatal "Disk space is too low!" path
  expressed as a typed outcome, not an adapter `io::Error`
- `RecoveryDecision` is a marker-fact sketch so MGR-03 is auditable now;
  do not pretend `head_blocks` exist until Phase 141

</specifics>

<deferred>
## Deferred Ideas

- `FjallCoinsView`, per-outpoint keys, `head_blocks` encoding, leftover
  snapshot non-authority, coins disk-read fail-closed — Phase 141
- Manager init, CanFlush readiness, IfNeeded-after-connect / Periodic-tick
  / Always-on-shutdown wiring, ordered block/undo/index-then-coins flush,
  jitter resampling, dbcache default budgets, crash-loss window,
  interrupted-flush replay vs fail-closed — Phase 142
- Honest payload-present availability — Phase 143
- Operator flush/recovery evidence and last-flush-reason surfaces —
  Phase 144
- Parity-root closeout and no-claim guardrails — Phase 145
- Assumeutxo dual-chainstate, prune/archive product modes, LevelDB,
  rust-bitcoin — out of v2.3

</deferred>

---

*Phase: 140-pure-flush-policy-and-typed-decisions*
*Context gathered: 2026-09-01*
