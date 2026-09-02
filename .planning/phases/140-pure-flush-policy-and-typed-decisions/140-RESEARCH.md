# Phase 140: Pure Flush Policy and Typed Decisions - Research

**Researched:** 2026-09-01
**Domain:** Pure-core Knots `FlushStateToDisk` / `GetCoinsCacheSizeState` decision machine and typed recovery-marker sketch
**Confidence:** HIGH

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

#### Decision outcome vocabulary

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

#### Injected facts vs computed classification

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

#### Flush vs Sync mapping

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

#### Recovery types and wiring boundary

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

### Deferred Ideas (OUT OF SCOPE)

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
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| FLUSH-01 | Node flushes coins using IfNeeded, Periodic, and Always policy, including disk-space refusal, from injected cache, time, and disk facts. | This phase delivers the **decision** surface only: `decide_flush(FlushPolicyInput) -> FlushDecision` with the pinned Knots `FlushStateToDisk` boolean split (`fCacheLarge` Periodic-only, `fCacheCritical` IfNeeded, `fPeriodicWrite`, `empty_cache`). Phase 142 executes the write-kind. Disk refusal is `RefuseDiskSpace`, not an adapter `io::Error`. |
| MGR-03 | Flush and recovery decisions are a typed pure-core state machine; adapters perform I/O. | `FlushDecision` plus a thin `RecoveryDecision` from injected `head_blocks` count. No Fjall, `std::fs`, Tokio, clock, or RNG in `open-bitcoin-chainstate`. Do not implement `ReplayBlocks` or fail-closed I/O. |
</phase_requirements>

## Summary

Phase 140 is a pure-core decision crate change. Implement Knots `FlushStateToDisk` / `GetCoinsCacheSizeState` as one injectable function in `open-bitcoin-chainstate` `coins/flush.rs`. The shell supplies occupancy bytes, already-jittered `next_write`, `now`, `memory_pressure`, disk-free bytes, and entry count. Core returns one `FlushDecision` and never reads a clock, allocator, or disk. Phase 139 already owns Flush-empties / Sync-retains algebra; this phase only chooses which later manager call to make.

The planner-critical trap is simplifying LARGE into “any Large writes.” Pinned Knots `fCacheLarge` is **Periodic-only**. `IfNeeded` + LARGE without CRITICAL or `memory_pressure` is `None`. Disk-space refusal is checked only after a write would otherwise happen, and it replaces the write (`RefuseDiskSpace`), including in `Always`. `None` mode classifies cache-size for observability and never writes or refuses. `FlushForPrune` stays unimplemented.

`RecoveryDecision` is a count-only sketch of Knots `GetHeadBlocks()` / `ReplayBlocks()` cardinality (empty / 1 / 2 / other). Do not implement replay. Do not touch `ManagedChainstate::persist` or `DurableSyncRuntime::persist_progress`.

**Primary recommendation:** Add `coins/flush.rs` with a shared-payload `FlushDecision::{None, Flush, Sync, RefuseDiskSpace}` plus `decide_flush` / `decide_recovery`, pin the 90% / 10 MiB / `192 * entry_count` constants, and cover the Knots boolean split with unit tests that inject facts.

## Project Constraints (from .cursor/rules/)

`.cursor/rules/` is absent in this repo. Treat the following as the local constraint set the planner must honor:

- Functional core / imperative shell: no Fjall, `std::fs`, Tokio, wall clock, or RNG in `open-bitcoin-chainstate`. [VERIFIED: `standards/core/architecture.md`, `scripts/check-pure-core-deps.sh`, `scripts/pure-core-crates.txt`]
- New modules use `foo.rs` + `foo/` (already the `coins.rs` / `coins/` layout). [VERIFIED: `standards/languages/rust.md`]
- No `unwrap()` in production; crate already `deny(clippy::unwrap_used)` outside tests. [VERIFIED: `packages/open-bitcoin-chainstate/src/lib.rs`]
- No rust-bitcoin, LevelDB, or new production crates. [VERIFIED: `.planning/REQUIREMENTS.md`, `packages/open-bitcoin-chainstate/Cargo.toml`]
- Prefix optional internals with `maybe_`; use `let...else`; keep files under the 628-line refactor trigger. [VERIFIED: `standards/core/code-shape.md`]
- Unit-test pure logic with Arrange / Act / Assert, one concern per test. [VERIFIED: `standards/core/testing.md`]
- New first-party Rust files under `packages/open-bitcoin-*/src` or `tests` need a parity breadcrumb mapping in `docs/parity/source-breadcrumbs.json`. [VERIFIED: `AGENTS.md`, `scripts/check-parity-breadcrumbs.ts`]
- Repo verification contract is `bash scripts/verify.sh`; ad-hoc Cargo goes through `bun run scripts/command-timings.ts run --key <stable-key> -- <command>`. [VERIFIED: `AGENTS.md`]
- `standards-overrides.md` has no active real overrides (placeholder row only). [VERIFIED: `standards-overrides.md`]

## Standard Stack

### Core

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| Rust | `1.94.1`, edition 2024 | Enums, newtypes, checked/saturating arithmetic | Pinned by `rust-toolchain.toml` and `packages/Cargo.toml`. [VERIFIED: `rustc --version`] |
| `open-bitcoin-chainstate` `0.1.0` | workspace | Home of `decide_flush` / `RecoveryDecision` | Already I/O-free; depends only on `open-bitcoin-consensus` and `open-bitcoin-primitives`. [VERIFIED: `packages/open-bitcoin-chainstate/Cargo.toml`] |
| Bitcoin Knots | `29.3.knots20260210` | Behavioral contract for mode, LARGE/CRITICAL math, Flush vs Sync, coins disk guard, head-marker cardinality | Official `validation.h` / `validation.cpp` / `txdb.cpp` / `util/fs_helpers.cpp` / `util/mempressure.h`. [CITED: github.com/bitcoinknots/bitcoin/blob/v29.3.knots20260210] |

### Supporting

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `std` only (`u64`, enums, `PartialOrd`) | Rust `1.94.1` | Injected-fact comparisons | Always. Do not add serde, Tokio, getrandom, or Fjall to this crate. |
| Existing `CoinsCache::flush` / `CoinsCache::sync` | Phase 139 | Algebra a later manager executes | Do not change. Policy only names which one. [VERIFIED: `packages/open-bitcoin-chainstate/src/coins/cache.rs`] |
| Mempool `PolicyTime` pattern | workspace | Precedent for injected time | Copy the *shape* (pure newtype, no `Instant::now()`). Do **not** depend on `open-bitcoin-mempool`. [VERIFIED: `packages/open-bitcoin-mempool/src/context.rs`] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Shared-payload `FlushDecision` enum | Separate write / refuse / size / reason predicates | Forbidden by D-01 / D-02. Adapters would recompose and Phase 144 would invent a second policy. |
| Local `FlushPolicyTime(u64)` seconds | Import mempool `PolicyTime(i64)` | Couples chainstate to mempool. Discretion allows a local newtype. |
| D-09 `free < 192 * n` | Knots `CheckDiskSpace` (`free >= 50 MiB + additional`) | Locked D-09 omits the 50 MiB floor. Document as a known difference; do not add it here. |
| Count-only `RecoveryDecision` | Hash-bearing replay plan | Phase 141 owns `head_blocks` encoding; Phase 142 owns replay vs fail-closed. |

**Installation:** none. Do not add Cargo packages.

**Version verification:** `rustc 1.94.1 (e408947bf 2026-03-25)`, `cargo 1.94.1`, Bun `1.3.14`, Bazel `8.6.0` are present on the research machine. [VERIFIED: local `--version`]

## Architecture Patterns

### Recommended Project Structure

```
packages/open-bitcoin-chainstate/src/
├── coins.rs                 # add `mod flush;` and re-export public types
├── coins/
│   ├── cache.rs             # leave Flush/Sync algebra unchanged
│   ├── memory.rs            # unchanged
│   ├── flush.rs             # NEW: modes, facts, decide_flush, decide_recovery
│   └── tests.rs             # add `mod flush;`
│       ├── algebra.rs       # existing Flush/Sync algebra tests — do not retarget
│       └── flush.rs         # NEW: policy matrix + recovery-count tests
├── error.rs                 # do not add disk-refusal as MissingCoin / apply error
└── lib.rs                   # re-export flush types next to CoinsCache
```

Also edit `docs/parity/source-breadcrumbs.json` `chainstate-engine.files` to include:

- `packages/open-bitcoin-chainstate/src/coins/flush.rs`
- `packages/open-bitcoin-chainstate/src/coins/tests/flush.rs`

Reuse the existing `chainstate-engine` breadcrumbs (`coins.h`, `coins.cpp`, `validation.cpp`, `node/blockstorage.cpp`, `node/chainstate.cpp`). Do not create a second group. [VERIFIED: `docs/parity/source-breadcrumbs.json`, `scripts/check-parity-breadcrumbs.ts`]

Do **not** edit:

- `packages/open-bitcoin-node/src/chainstate.rs` `persist()` (`save_snapshot` leftover)
- `packages/open-bitcoin-node/src/sync/runtime_state.rs` `persist_progress()`
- `CoinsCache::flush` / `CoinsCache::sync`

### Pattern 1: Injected-fact flush policy

**What:** One pure function returns a single `FlushDecision`. Facts in, decision out.
**When to use:** This is the entire Phase 140 production surface.
**Example:**

```rust
// Discretion recommendation. Mapping locked by D-06..D-18.
// Knots source: validation.cpp GetCoinsCacheSizeState + FlushStateToDisk
// https://github.com/bitcoinknots/bitcoin/blob/v29.3.knots20260210/src/validation.cpp

pub fn decide_flush(input: FlushPolicyInput) -> FlushDecision {
    let cache_size = classify_cache_size(
        input.cache_bytes,
        input.cache_byte_limit,
        input.mempool_leftover_bytes,
    );
    let maybe_write = intended_write(input.mode, cache_size, input.periodic_due(), input.memory_pressure);
    let Some((kind, reason)) = maybe_write else {
        return FlushDecision::None(FlushDecisionFacts {
            cache_size,
            reason: LastFlushReason::None,
        });
    };
    if disk_guard_fails(input.disk_free_bytes, input.cache_entry_count) {
        return FlushDecision::RefuseDiskSpace(FlushDecisionFacts {
            cache_size,
            reason: LastFlushReason::FailedDisk,
        });
    }
    match kind {
        FlushWriteKind::Flush => FlushDecision::Flush(FlushDecisionFacts { cache_size, reason }),
        FlushWriteKind::Sync => FlushDecision::Sync(FlushDecisionFacts { cache_size, reason }),
    }
}
```

### Pattern 2: Shared facts, exclusive outcomes

**What:** Cache-size and last-flush-reason live on every decision via one `FlushDecisionFacts` payload. Write-kind is the enum variant. `RefuseDiskSpace` cannot carry a write.
**When to use:** Always. Satisfies D-01, D-02, D-03, D-05.
**Do not** add `empty_cache: bool` or `disk_ok: bool`.

### Pattern 3: Recovery cardinality sketch

**What:** `decide_recovery(head_marker_count: usize) -> RecoveryDecision` mirrors Knots `GetHeadBlocks()` / `ReplayBlocks()` count split without hashes or I/O.
**When to use:** MGR-03 only. Phase 142 binds this to stored markers.

Knots: missing `DB_HEAD_BLOCKS` → empty vector → consistent; `size != 2` → `"unknown inconsistent state"`; `size == 2` → replay candidate. [CITED: github.com/bitcoinknots/bitcoin/blob/v29.3.knots20260210/src/txdb.cpp, validation.cpp `ReplayBlocks`]

D-20 requires one-element to be first-class, not lumped into “other.”

### Anti-Patterns to Avoid

- **Clock in `decide_flush`:** `Instant::now()` / `SystemTime::now()` makes core impure. Inject `FlushPolicyTime`. `scripts/check-pure-core-deps.sh` does **not** ban `std::time`; still forbid it here. [VERIFIED: `scripts/check-pure-core-deps.sh` forbidden list is `std::fs|std::net|std::env|std::process|std::thread|tokio|reqwest|rustls|rand`]
- **LARGE writes under IfNeeded:** Knots `fCacheLarge = mode == PERIODIC && cache_state >= LARGE`. IfNeeded LARGE without CRITICAL/pressure is `None`. [CITED: validation.cpp L3117–L3126]
- **Disk refuse as `ChainstateError`:** Refusal is a decision, not `MissingCoin` or apply failure. [VERIFIED: CONTEXT D-02, `error.rs`]
- **Retarget leftover snapshot persist:** `persist()` and `persist_progress` still write the JSON blob. Do not call `decide_flush` from them. [VERIFIED: `chainstate.rs` L249–251, `runtime_state.rs` L88–101]
- **`empty_cache` bool:** Phase 139 `flush()` empties; `sync()` retains unspent clean. A bool can disagree. [VERIFIED: `cache.rs` L341–360]
- **Port `fFlushForPrune`:** Knots unlinks files after `FindFilesToPrune`. Out of v2.3. Keep the prune flag conceptually false. [CITED: validation.cpp L3085–L3161]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Wall clock | `Instant::now()` in chainstate | Injected `FlushPolicyTime` | Tests and architecture policy |
| Periodic jitter | `getrandom` / 50–70 min window in core | Shell-supplied `next_write` | D-07; Phase 142 resamples |
| Host memory pressure | `util/mempressure` port / OS crate | Injected `memory_pressure: bool` | D-08; Knots `SystemNeedsMemoryReleased()` is OS I/O |
| Disk free | `std::fs::space` / Fjall `disk_space()` | Injected `disk_free_bytes` | D-09 / D-23 |
| Flush vs Sync algebra | New write path | Existing `CoinsCache::flush` / `sync` | Already unit-tested in `algebra.rs` |
| Interrupted flush | `ReplayBlocks` / undo I/O | Count-only `RecoveryDecision` | D-20 / D-21 |
| Cache-byte defaults | 450 / 4 / 8 MiB constants | Test-injected limits | D-10 |
| Last-flush-reason later | Phase 144 re-derives reason | Carry `LastFlushReason` on every decision | D-03 |

**Key insight:** The hard part is the Knots boolean split, not a new storage engine. Hand-rolling “Large always flushes” or putting the clock in core would make Phase 142 execute the wrong write-kind.

## Common Pitfalls

### Pitfall 1: IfNeeded + LARGE writes

**What goes wrong:** Policy treats LARGE like CRITICAL and returns `Flush` after every connect once the cache is 90% or within 10 MiB.
**Why it happens:** FEATURES/STACK prose says “LARGE or CRITICAL” without repeating that `fCacheLarge` is Periodic-only.
**How to avoid:** Implement the Knots split verbatim. Unit-test IfNeeded + LARGE + no pressure → `None`.
**Warning signs:** A test named `large_always_flushes` with no mode dimension.

### Pitfall 2: LARGE threshold is `max(90%, total - 10 MiB)`, not “90% or 10 MiB”

**What goes wrong:** Tests only use a 50 MiB budget (where 90% wins) and miss that a 200 MiB budget becomes LARGE only above `total - 10 MiB` (190 MiB), not at 180 MiB.
**Why it happens:** The Knots enum comment says “>= 90% capacity” but the code is `cacheSize > max((9 * total) / 10, total - 10 MiB)` with strict `>`. [CITED: validation.cpp L3056–L3067]
**How to avoid:** Two classification tests: small budget (90% binds) and large budget (headroom binds). Use `>` not `>=`.
**Warning signs:** A single 90% fixture; `>=` in the helper.

### Pitfall 3: Disk guard runs when no write is intended

**What goes wrong:** `None` mode or IfNeeded+LARGE returns `RefuseDiskSpace` because free bytes are low.
**Why it happens:** Checking the guard before mode logic. Knots `CheckDiskSpace` runs only inside `if (should_write)`.
**How to avoid:** D-09 / D-17: classify first, decide intended write, then maybe refuse.
**Warning signs:** `None` + low disk test expects refusal.

### Pitfall 4: Silent 50 MiB Knots floor

**What goes wrong:** Implementer copies `CheckDiskSpace` (`free >= 50 MiB + additional`) and disagrees with D-09, or documents D-09 as full Knots parity.
**Why it happens:** `CheckDiskSpace` in `util/fs_helpers.cpp` adds `min_disk_space = 52428800`. [CITED: github.com/bitcoinknots/bitcoin/blob/v29.3.knots20260210/src/util/fs_helpers.cpp]
**How to avoid:** Phase 140 refuses iff `free_bytes < 192 * entry_count`. Record the 50 MiB floor as a known difference. Phase 142 may inject a precomputed required-guard that includes it.
**Warning signs:** A `50 * 1024 * 1024` constant in `flush.rs`.

### Pitfall 5: Overflow on `9 * total`

**What goes wrong:** `cache_byte_limit.saturating_add(leftover) * 9` wraps or panics in debug.
**Why it happens:** Knots uses `int64_t`; tests inject huge limits.
**How to avoid:** Compute `(u128::from(total) * 9 / 10)` then saturate to `u64`. Use `saturating_add` / `saturating_mul` for the disk guard (`192 * entry_count`).
**Warning signs:** Bare `9 * total` on `u64`.

### Pitfall 6: Wiring leftover persist

**What goes wrong:** `ManagedChainstate::persist` or `persist_progress` starts calling `decide_flush` and is treated as durability proof.
**Why it happens:** FLUSH-01 wording says “Node flushes.” The phase boundary is the decision machine only.
**How to avoid:** D-22. Leave snapshot writes untouched. A snapshot write is not a flush-policy proof.
**Warning signs:** `open-bitcoin-node` diffs in this phase.

### Pitfall 7: Recovery sketch grows into ReplayBlocks

**What goes wrong:** `decide_recovery` takes block hashes and invents rollback/roll-forward.
**Why it happens:** Knots `ReplayBlocks` is adjacent in `validation.cpp`.
**How to avoid:** Inject only a count. Variants: empty / one / two / other. No undo, no fail-closed I/O.
**Warning signs:** `BlockHash` in `flush.rs`.

## Code Examples

Verified patterns from official Knots `v29.3.knots20260210` and local Phase 139 algebra.

### Cache-size classification (lock this math)

```rust
// Source: Knots validation.cpp GetCoinsCacheSizeState
// https://github.com/bitcoinknots/bitcoin/blob/v29.3.knots20260210/src/validation.cpp
// nTotalSpace = max_coins_cache_size_bytes + max(0, max_mempool - mempool_usage)
// D-06 injects leftover already: total = cache_limit + leftover (leftover is u64 => already >= 0)

pub const LARGE_CACHE_NUMERATOR: u64 = 9;
pub const LARGE_CACHE_DENOMINATOR: u64 = 10;
pub const LARGE_CACHE_HEADROOM_BYTES: u64 = 10 * 1024 * 1024;
pub const COIN_WRITE_GUARD_BYTES_PER_ENTRY: u64 = 48 * 2 * 2; // 192

fn classify_cache_size(
    cache_bytes: u64,
    cache_byte_limit: u64,
    mempool_leftover_bytes: u64,
) -> CoinsCacheSizeState {
    let total = cache_byte_limit.saturating_add(mempool_leftover_bytes);
    if cache_bytes > total {
        return CoinsCacheSizeState::Critical;
    }
    let ninety = u64::try_from(u128::from(total) * 9 / 10).unwrap_or(u64::MAX);
    let headroom = total.saturating_sub(LARGE_CACHE_HEADROOM_BYTES);
    let large_threshold = ninety.max(headroom);
    if cache_bytes > large_threshold {
        CoinsCacheSizeState::Large
    } else {
        CoinsCacheSizeState::Ok
    }
}
```

`classify_cache_size` should be `pub(crate)` (D-04: not a manager-composed public contract). Cover it through `decide_flush` plus same-module tests.

### Intended write (Knots boolean split)

```text
Knots FlushStateToDisk (fFlushForPrune forced false):

fCacheLarge      = mode == PERIODIC && cache_state >= LARGE   // LARGE and CRITICAL
fCacheCritical   = mode == IF_NEEDED && (CRITICAL || memory_pressure)
fPeriodicWrite   = mode == PERIODIC && now >= next_write
should_write     = ALWAYS || fCacheLarge || fCacheCritical || fPeriodicWrite
empty_cache      = ALWAYS || fCacheLarge || fCacheCritical
write            = empty_cache ? Flush : Sync

Source: https://github.com/bitcoinknots/bitcoin/blob/v29.3.knots20260210/src/validation.cpp
```

| Mode | Cache | Pressure | periodic_due | Outcome | Reason |
|------|-------|----------|--------------|---------|--------|
| None | any | any | any | `None` (never refuse) | `None` |
| Always | any | any | any | `Flush` or `RefuseDiskSpace` | `Always` / `FailedDisk` |
| Periodic | Large or Critical | any | false | `Flush` | `Periodic` |
| Periodic | Ok | any | true | `Sync` | `Periodic` |
| Periodic | Ok | any | false | `None` | `None` |
| IfNeeded | Critical | any | any | `Flush` | `Needed` |
| IfNeeded | Ok or Large | true | any | `Flush` | `Needed` |
| IfNeeded | Large | false | any | `None` | `None` |
| IfNeeded | Ok | false | any | `None` | `None` |

`CoinsCacheSizeState` ordering may be `Ok = 0, Large = 1, Critical = 2` to match Knots so `>= Large` includes Critical under Periodic. [CITED: validation.h `CoinsCacheSizeState`]

### Disk guard (D-09, not full CheckDiskSpace)

```rust
// Knots coins-write additional_bytes = 48 * 2 * 2 * GetCacheSize()
// CheckDiskSpace also requires +50 MiB — do not add that constant here.
fn disk_guard_fails(disk_free_bytes: u64, cache_entry_count: u64) -> bool {
    let required = COIN_WRITE_GUARD_BYTES_PER_ENTRY.saturating_mul(cache_entry_count);
    disk_free_bytes < required
}
```

### Recommended types (discretion)

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FlushPolicyTime(u64); // unix seconds; no Instant::now()

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlushMode { None, IfNeeded, Periodic, Always }

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CoinsCacheSizeState { Ok = 0, Large = 1, Critical = 2 }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LastFlushReason { None, Needed, Periodic, Always, FailedDisk }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FlushDecisionFacts {
    pub cache_size: CoinsCacheSizeState,
    pub reason: LastFlushReason,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlushDecision {
    None(FlushDecisionFacts),
    Flush(FlushDecisionFacts),
    Sync(FlushDecisionFacts),
    RefuseDiskSpace(FlushDecisionFacts),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryDecision {
    ConsistentEmptyHeads,
    OneHead,
    InterruptedTwoHeads,
    InconsistentOtherCount { count: usize },
}
```

`RefuseDiskSpace` facts.reason is always `FailedDisk`. Accessors `cache_size()` / `reason()` on `FlushDecision` keep Phase 144 from matching twice.

### Existing algebra the manager will call later (do not change)

```341:360:packages/open-bitcoin-chainstate/src/coins/cache.rs
    pub fn flush(&mut self) -> Result<(), ChainstateError> {
        let batch = self.overlay.dirty_batch();
        let maybe_best_block = self.overlay.maybe_best_block;
        self.parent.batch_write(batch, maybe_best_block)?;
        self.overlay.clear();
        Ok(())
    }

    pub fn sync(&mut self) -> Result<(), ChainstateError> {
        let batch = self.overlay.dirty_batch();
        let maybe_best_block = self.overlay.maybe_best_block;
        self.parent.batch_write(batch, maybe_best_block)?;
        self.overlay.entries.retain(|_, entry| {
            let Some(coin) = entry.maybe_coin().cloned() else {
                return false;
            };
            *entry = CoinsCacheEntry::unspent_clean(coin);
            true
        });
        Ok(())
    }
```

### Recovery cardinality

```rust
// Source: Knots txdb.cpp GetHeadBlocks + validation.cpp ReplayBlocks
// empty → consistent; size == 2 → interrupted; else unknown inconsistent.
// D-20 splits size == 1 out as its own variant.

pub fn decide_recovery(head_marker_count: usize) -> RecoveryDecision {
    match head_marker_count {
        0 => RecoveryDecision::ConsistentEmptyHeads,
        1 => RecoveryDecision::OneHead,
        2 => RecoveryDecision::InterruptedTwoHeads,
        count => RecoveryDecision::InconsistentOtherCount { count },
    }
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| `ManagedChainstate::persist` writes a full snapshot after every mutation | Policy decides None / Flush / Sync / Refuse from injected facts | Phase 140 decides; Phase 142 executes | This phase must not cut over persist |
| Milestone research Pattern 2 returned `{ write, empty_cache: bool }` | Enum `None \| Flush \| Sync \| RefuseDiskSpace` | Locked D-05 2026-09-01 | Write-kind implies empty-cache |
| ARCHITECTURE suggested injecting `last_flush` | Inject already-jittered `next_write` | Locked D-07 | No 50–70 min math in core |
| Knots `CheckDiskSpace` = 50 MiB + `192 * n` | D-09 = `free < 192 * n` | Locked this phase | Known difference; do not “fix” it here |

**Deprecated/outdated:**

- “LARGE always flushes” — false for IfNeeded. [CITED: validation.cpp]
- Milestone SUMMARY Phase 140+/141+ numbering — the live roadmap merged view/cache + engine apply into Phase 139; **this** Phase 140 is flush policy. [VERIFIED: `.planning/ROADMAP.md`]
- Putting `FlushPolicy` next to Fjall — decisions stay in chainstate. [VERIFIED: `.planning/research/ARCHITECTURE.md`]

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | `FlushPolicyTime(u64)` unix seconds is enough for `now >= next_write` (Knots uses `NodeClock` time_points). | Discretion / types | Phase 142 could prefer millis; conversion stays in the shell. Low risk: comparison is ordinal. |
| A2 | Carrying `LastFlushReason::FailedDisk` without also storing the would-have-written mode is enough for Phase 144. | D-03 / types | Phase 144 might want “failed during Periodic.” Input `FlushMode` is still available at the call site then; do not add a second reason field now. |

No other `[ASSUMED]` claims. Knots mapping, disk-guard formula, and leftover persist seams were read from official sources or the live tree.

## Open Questions

1. **Should Phase 142’s injected required-guard add Knots’ 50 MiB `min_disk_space`?**
   - What we know: D-09 locks `free < 192 * n` for Phase 140. Knots `CheckDiskSpace` also requires 50 MiB. [CITED: `util/fs_helpers.cpp`]
   - What's unclear: whether operator-visible refusal should match Knots fatality on a 40 MiB-free disk with zero cache entries.
   - Recommendation: Keep Phase 140 on D-09. Note the difference in plan “known Knots difference.” Let Phase 142 choose a precomputed guard if desired.

2. **Does `check-pure-core-deps.sh` need a `std::time` ban for this crate?**
   - What we know: current forbidden imports omit `std::time`. [VERIFIED: `scripts/check-pure-core-deps.sh`]
   - What's unclear: whether to extend the repo-wide checker in this phase.
   - Recommendation: Do not expand the global checker. Add a focused unit or comment-level grep in flush tests (`flush.rs` must not contain `Instant`, `SystemTime`, `std::fs`, `fjall`, `tokio`).

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| Rustc / Cargo | Implement + unit test | ✓ | 1.94.1 | — |
| Bun | Breadcrumb checker / verify | ✓ | 1.3.14 | — |
| Bazel | Default `verify.sh` smoke | ✓ | 8.6.0 | — |
| `packages/bitcoin-knots` submodule on disk | Line-level local reads | ✗ | — | Official GitHub tag `v29.3.knots20260210` (used this session) |
| Fjall / Tokio / getrandom | This phase | n/a | — | Must **not** be added to chainstate |

**Missing dependencies with no fallback:** none for implementation. Local Knots checkout is optional; citations used the pinned GitHub tag.

**Missing dependencies with fallback:** bitcoin-knots submodule — use the pinned GitHub sources already quoted above.

## Security Domain

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V2 Authentication | no | No operator or peer auth in this phase |
| V3 Session Management | no | — |
| V4 Access Control | no | — |
| V5 Input Validation | yes | Parse injected facts into `FlushPolicyTime` / `u64` / enums; saturating / `u128` arithmetic so huge limits cannot panic or wrap into a write |
| V6 Cryptography | no | No hashing or keys. Do not add rust-bitcoin |

### Known Threat Patterns for flush policy

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Core reads disk/clock and becomes non-hermetic | Information Disclosure / Tampering | Inject facts; architecture-policy + no Fjall/`std::fs` in crate |
| Overflow turns CRITICAL into OK and skips a required Flush | Tampering | `u128` 90% math; saturating add/mul |
| Disk-full write proceeds because refusal is an adapter error | Denial of Service / Tampering | First-class `RefuseDiskSpace`; adapters must not write on that variant |
| Prune unlink smuggled in as “Knots FlushStateToDisk” | Tampering / Repudiation | `fFlushForPrune` stays false; no file unlink |
| Snapshot persist treated as flush proof | Spoofing | D-22: do not wire leftover persist |

## Sources

### Primary (HIGH confidence)

- [CITED: github.com/bitcoinknots/bitcoin/blob/v29.3.knots20260210/src/validation.h] — `FlushStateMode`, `CoinsCacheSizeState`, `FlushStateToDisk`, `GetCoinsCacheSizeState`
- [CITED: github.com/bitcoinknots/bitcoin/blob/v29.3.knots20260210/src/validation.cpp] — `GetCoinsCacheSizeState` math; `FlushStateToDisk` `fCacheLarge` / `fCacheCritical` / `fPeriodicWrite` / `empty_cache`; `ReplayBlocks` count split; `DATABASE_WRITE_INTERVAL` 50–70 min (Phase 142)
- [CITED: github.com/bitcoinknots/bitcoin/blob/v29.3.knots20260210/src/txdb.cpp] — `GetHeadBlocks()` empty-on-miss; `DB_HEAD_BLOCKS` two-element write
- [CITED: github.com/bitcoinknots/bitcoin/blob/v29.3.knots20260210/src/util/fs_helpers.cpp] — `CheckDiskSpace` = 50 MiB + additional
- [CITED: github.com/bitcoinknots/bitcoin/blob/v29.3.knots20260210/src/util/mempressure.h] — `SystemNeedsMemoryReleased()` declaration
- [VERIFIED: `packages/open-bitcoin-chainstate/src/coins.rs`, `coins/cache.rs`, `error.rs`, `lib.rs`] — module home, Flush/Sync algebra, no I/O deps
- [VERIFIED: `packages/open-bitcoin-node/src/chainstate.rs` L249–251, `sync/runtime_state.rs` L88–101] — leftover snapshot persist
- [VERIFIED: `docs/parity/source-breadcrumbs.json`, `scripts/check-parity-breadcrumbs.ts`] — explicit file list, not a glob
- [VERIFIED: `scripts/check-pure-core-deps.sh`, `scripts/pure-core-crates.txt`] — chainstate is pure-core
- [VERIFIED: `.planning/phases/140-pure-flush-policy-and-typed-decisions/140-CONTEXT.md`] — locked D-01..D-23
- [VERIFIED: `standards/core/architecture.md`, `code-shape.md`, `testing.md`, `languages/rust.md`]

### Secondary (MEDIUM confidence)

- [CITED: `.planning/research/STACK.md`, `FEATURES.md`, `ARCHITECTURE.md`] — milestone-level flush table; superseded on `empty_cache: bool` and `last_flush` injection by CONTEXT D-05 / D-07
- [CITED: github.com/bitcoinknots/bitcoin/pull/261] — `SystemNeedsMemoryReleased` is OS memory-pressure (inject bool; do not port)

### Tertiary (LOW confidence)

- Local `packages/bitcoin-knots` tree was not materialized this session; all Knots line behavior was fetched from the pinned GitHub tag rather than the submodule path.

## Metadata

**Confidence breakdown:**

- Standard stack: HIGH — no new crates; pins and Cargo.toml verified
- Architecture: HIGH — `coins/flush.rs` home, leftover persist seams, breadcrumb mapping, and Knots boolean split verified
- Pitfalls: HIGH — IfNeeded+LARGE, disk-guard order, 50 MiB difference, and persist retarget are sourced from Knots + live code

**Research date:** 2026-09-01
**Valid until:** 2026-10-01 (Knots pin and crate seams are stable)

---
*Phase: 140-pure-flush-policy-and-typed-decisions*
*Researched: 2026-09-01*
