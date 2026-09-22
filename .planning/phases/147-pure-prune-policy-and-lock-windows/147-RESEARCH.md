# Phase 147: Pure Prune Policy and Lock Windows - Research

**Researched:** 2026-09-22
**Domain:** Knots-aligned prune mode / height-window / lock-buffer decisions in `open-bitcoin-chainstate` (I/O-free)
**Confidence:** HIGH

## Summary

Phase 147 adds typed prune-mode shapes and pure planners in `open-bitcoin-chainstate` that mirror Bitcoin Knots `29.3.knots20260210` for `-prune` parsing, `GetPruneRange` keep windows, automatic prune-after gating, manual short-chain refusal, and `DoPruneLocksForbidPruning` buffers — without Fjall, filesystem, RPC, unlink, or `have_pruned`. The existing `decide_flush` / `FlushDecision` pattern is the template: injected facts in, exclusive typed outcomes out. Do not extend `FlushMode` or `ChainstateError` for these policy outcomes; introduce prune-local decision and parse enums beside a new `prune/` module.

Planning risk is off-by-one. Knots last prunable height is `tip - 288` (inclusive). Automatic prune is a no-op when `tip <= nPruneAfterHeight`. Manual RPC errors when `tip < nPruneAfterHeight` and **clamps** targets inside the keep window; Open Bitcoin locks a **typed refusal** for that keep-window case (D-07 / PRUN-03) while still using the same Knots comparison (`height > tip - 288`). Lock protection for height-granular payloads is the inclusive range `[height_first - 10, height_last + 10]` (with Knots' low-height special case). Automatic byte-budget selection walks oldest-first and stops once remaining usage is under target; map that onto ascending injected per-height sizes and omit Knots `BLOCKFILE_CHUNK_SIZE + UNDOFILE_CHUNK_SIZE` as a documented layout difference.

**Primary recommendation:** Add `packages/open-bitcoin-chainstate/src/prune/` with `PruneMode` parse + `plan_automatic_prune` / `plan_manual_prune` / `heights_forbidden_by_prune_locks`, unit-tested against the Knots comparisons below, and register a new `chainstate-prune` breadcrumb group.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

#### Typed prune mode

- **D-01:** Prune mode is a typed enum in `open-bitcoin-chainstate`:
  disabled, manual-only, or automatic with a target in MiB. The external
  Knots integer contract is `0` disabled, `1` manual-only
  (`PRUNE_TARGET_MANUAL`), and `N >= 550` automatic target MiB. The decision
  core does not keep a raw integer as its working state.
- **D-02:** An automatic target below 550 MiB is a typed config refusal.
  Values other than `0`, `1`, or `>= 550` are the same refusal. 550 MiB is
  `MIN_DISK_SPACE_FOR_BLOCK_FILES`, not a tunable default.
- **D-03:** The default mode is disabled. Enabling prune is explicit. This
  phase does not parse JSONC, clap flags, or RPC. It exposes the typed
  shapes and pure decisions that a later operator surface (Phase 150) will
  feed. Success criterion 1 is satisfied by those typed config/policy
  shapes, not by a new operator command.

#### Automatic height window

- **D-04:** Automatic planning keeps the last 288 blocks
  (`MIN_BLOCKS_TO_KEEP`). Heights inside that trailing window are never
  prune candidates.
- **D-05:** Automatic planning does not start before the network
  prune-after height. That height is an injected chain-params input, not
  a mainnet constant baked into the function and not an operator override.
  Before the tip reaches that height, the automatic plan is empty. That
  empty plan is not a failure.
- **D-06:** The pure planner may return a candidate height list only from
  caller-injected facts: tip height, prune-after height, mode, and
  per-height payload byte sizes. It compares injected usage with the
  automatic target and stops at the keep window, prune-after gate, and
  lock protections. It does not stat disk, list Fjall keys, or choose
  files by walking storage.

#### Manual keep-window refusal

- **D-07:** Manual prune takes an explicit target height. A target inside
  the 288-block keep window is a typed refusal. The comparison must match
  pinned Knots `GetPruneRange` / manual prune height checks. Do not invent
  a looser or stricter window.
- **D-08:** Manual mode does not invent an automatic byte-budget plan.
  When the target height is legal, the plan is the heights up to that
  target that also sit outside the keep window, at or after the
  prune-after rule Knots uses for manual prune, and outside lock
  protections. Researcher must cite the Knots manual-versus-automatic
  difference rather than assuming they share one formula.

#### Lock buffer

- **D-09:** A prune lock is an injected inclusive height range plus a
  caller-supplied name or id. This phase does not persist locks, list
  them, or add an operator setter. Phase 150 owns LOCK-02.
- **D-10:** Protected heights are the locked range expanded by
  `PRUNE_LOCK_BUFFER = 10` in the direction and inclusivity Knots uses in
  `DoPruneLocksForbidPruning`. Both automatic and manual plans must omit
  those heights. A plan that would otherwise delete a protected height
  must leave it out, not fail the whole prune solely because one lock
  overlaps part of the range, unless Knots itself refuses the call.
  Researcher pins that Knots behavior before the planner codes it.
- **D-11:** Lock decisions are pure. No Fjall, filesystem, or RPC access
  in `open-bitcoin-chainstate`.

#### Plan result and later phases

- **D-12:** The phase output is a pure plan or a typed refusal: eligible
  heights, an empty not-yet plan, or a refusal reason (below-minimum
  target, manual height inside the keep window, prune disabled when a
  manual prune is requested). It does not delete payloads, set
  have-pruned, or emit `Pruned`.
- **D-13:** Do not extend `FlushMode` or perform flush in this phase.
  Phase 148 owns unlink order, including any flush-before-delete fact.
  A non-empty candidate list is enough for that later phase to consume.
- **D-14:** Single active chainstate only. No assumeutxo snapshot-base
  prune-start branch and no second chainstate.

#### Operator, parity, and verification boundary

- **D-15:** Do not add prune RPC, CLI, or dashboard fields. Phase 150 owns
  operator surfaces. Do not advertise `NODE_NETWORK_LIMITED` (Phase 149).
- **D-16:** New first-party Rust source and tests under
  `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests` get
  parity breadcrumbs through `docs/parity/source-breadcrumbs.json`. Cite
  the pinned Knots prune anchors where a defensible source line exists.
  Full v2.4 prune parity docs and the no-claim checker stay in Phase 151
  (GRD-01).
- **D-17:** Verification remains `bash scripts/verify.sh`. Default
  verification stays deterministic and public-network-free. No new
  production crate or third-party library.

### Claude's Discretion

- Module split inside `open-bitcoin-chainstate` (`prune.rs` versus a
  `prune/` folder) as long as the crate stays I/O-free and the public
  decision API is typed.
- Exact integer comparisons for "last 288" and the 10-block buffer, once
  research quotes the pinned Knots functions. The constants 288, 10, and
  550 MiB are locked; off-by-one interpretation is not.
- Whether per-height sizes are a `BTreeMap` or a sorted slice, as long as
  the planner stays pure and does not read storage.
- Fixture style for tip, prune-after, locks, and injected byte sizes.

### Deferred Ideas (OUT OF SCOPE)

- Fjall block and undo key deletion, have-pruned, and interrupted-prune
  recovery — Phase 148 (UNLK-01..03).
- `NODE_NETWORK_LIMITED` advertisement and honest `Pruned` versus
  `Unavailable` labels — Phase 149 (SERV-01..03, LABL-01).
- RPC, CLI, dashboard prune status, manual prune commands, and lock
  list/set — Phase 150 (OPER-01..03, LOCK-02).
- Full prune parity docs and no-claim checkers — Phase 151 (GRD-01).
- Temporary IBD prune target (`-pruneduringinit`) — FUT-27.
- Combining txindex with prune — out of scope; Knots marks those modes
  incompatible, and this milestone does not add txindex.
- Flush-before-unlink orchestration (`fFlushForPrune`) — Phase 148.

None — discussion stayed within phase scope.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| PRUN-01 | Operator can disable prune, select manual-only prune, or set an automatic target of at least 550 MiB | `parse_prune_arg` / `PruneMode` mapping from Knots `ParsePruneOption`; default `Disabled` |
| PRUN-02 | Automatic prune keeps the last 288 blocks and does not start before the network prune-after height | `GetPruneRange` max = `tip - 288`; `FindFilesToPrune` early return when `tip <= PruneAfterHeight()` → empty plan |
| PRUN-03 | Manual prune refuses a target inside the 288-block keep window | Same Knots comparison `height > tip - 288`; Open Bitcoin returns typed refusal (Knots RPC clamps — document, do not copy clamp) |
| LOCK-01 | A prune lock keeps the locked height range, plus a 10-block buffer, from deletion | `DoPruneLocksForbidPruning` / header note → omit protected heights; skip, do not fail entire plan |
</phase_requirements>

## Project Constraints (from .cursor/rules/)

No `.cursor/rules/` directory is present in this workspace. Constraints below come from `AGENTS.md` Repo-Local Guidance, `AGENTS.bright-builds.md`, `standards-overrides.md` (no active overrides), and managed standards pages `standards/core/architecture.md` + `standards/core/code-shape.md`:

- Functional core / imperative shell: prune decisions stay pure; no Fjall/filesystem in `open-bitcoin-chainstate`.
- Parse at boundaries into domain types (`PruneMode`), then operate on those types.
- Prefer early returns / `let...else`; no `unwrap` in production path; prefix `Option` bindings with `maybe_`.
- No new production crate or third-party library (D-17 / stack research).
- New Rust sources under `packages/open-bitcoin-*/src` need parity breadcrumbs via `docs/parity/source-breadcrumbs.json`.
- Verification contract remains `bash scripts/verify.sh` (do not run it in this research pass).

## Knots Comparison Answers (planning questions)

All citations are from the pinned submodule `packages/bitcoin-knots` at tag `v29.3.knots20260210`.

### 1. Integer mapping for prune=0 / 1 / >=550 / refuse 2..549

From `ParsePruneOption` in `packages/bitcoin-knots/src/node/blockmanager_args.cpp`:

| Input `nPruneArg` | Knots result | Lines |
|-------------------|--------------|-------|
| `< 0` | Error: cannot be negative | 23–24 |
| `0` | Disabled → return `0` | 25–26 |
| `1` | Manual → `BlockManager::PRUNE_TARGET_MANUAL` (`uint64_t::max`) | 27–28; `blockstorage.h:375` |
| `>= 2` | `nPruneTarget = nPruneArg * 1024 * 1024` bytes; if `< MIN_DISK_SPACE_FOR_BLOCK_FILES` → error | 30–33 |
| `>= 550` | Automatic target bytes = `N * 1024 * 1024` | 30–34 |

`MIN_DISK_SPACE_FOR_BLOCK_FILES = 550 * 1024 * 1024` (`validation.h:82`). So **`2..=549` all refuse** with *"Prune configured below the minimum of 550 MiB"*; **`550` is the first accepted automatic target**. `IsPruneMode` is `prune_target > 0` (`blockstorage.cpp:1315`), so both manual and automatic are prune mode.

Open Bitcoin: map to `PruneMode::{Disabled, ManualOnly, Automatic { target_mib }}` — never store `PRUNE_TARGET_MANUAL` raw integer in the decision core (D-01).

### 2. Keep-window comparison (inclusive / exclusive)

`GetPruneRange` (`validation.cpp:6910–6934`):

```text
max_prune = max(0, Height() - MIN_BLOCKS_TO_KEEP)   // MIN_BLOCKS_TO_KEEP = 288
prune_end = min(last_height_can_prune, max_prune)
return {prune_start, prune_end}
```

File eligibility (`FindFilesToPrune` / Manual, `blockstorage.cpp:353`, `441`):

```text
skip if nHeightLast > last_block_can_prune   // so last_block_can_prune is inclusive
```

**Verified rule:** last prunable height = `tip - 288` (inclusive). Heights `tip - 287 ..= tip` (288 heights) are never prune candidates.

Example: tip `1000` → `max_prune = 712`. Height `712` may be pruned; `713` may not.

Manual RPC keep check (`rpc/blockchain.cpp:1265–1267`):

```text
if (height > chainHeight - MIN_BLOCKS_TO_KEEP) { clamp to chainHeight - MIN_BLOCKS_TO_KEEP; }
```

Same boundary: inside keep window ⇔ `height > tip - 288` ⇔ `height >= tip - 287`.

### 3. Manual vs automatic `GetPruneRange`; keep-window behavior

Both paths call `GetPruneRange`:

- Manual: `FindFilesToPruneManual(..., nManualPruneHeight, ...)` → `GetPruneRange(chain, nManualPruneHeight)` (`blockstorage.cpp:348`).
- Automatic: `FindFilesToPrune(..., last_prune, ...)` where `last_prune` starts as tip height (`validation.cpp:3092`, `3104`) → `GetPruneRange(chain, last_prune)` (`blockstorage.cpp:404`).

**Manual keep-window in Knots RPC:** does **not** error — it **clamps** and logs *"Attempt to prune blocks close to the tip..."* (`rpc/blockchain.cpp:1265–1267`), then proceeds. `GetPruneRange` would also cap via `min(requested, tip - 288)`.

**Open Bitcoin (locked):** typed **refusal** when the requested manual target is inside the keep window (D-07, PRUN-03, success criterion 3). Use the Knots comparison; do not invent a different window; do not silently clamp in the pure API.

Other manual refusals to model (for later Phase 150 strings; pure reasons now):

| Condition | Knots | Open Bitcoin Phase 147 |
|-----------|-------|------------------------|
| Not prune mode | RPC error `"Cannot prune blocks because node is not in prune mode."` | Refusal: prune disabled |
| `tip < PruneAfterHeight()` | RPC error `"Blockchain is too short for pruning."` (`rpc/blockchain.cpp:1261–1262`) | Refusal: chain too short (D-08) |
| Target inside keep window | Clamp | **Refusal** (locked) |
| Target `> tip` | RPC error `"Blockchain is shorter..."` | Refusal (recommended; Knots-aligned) |

### 4. Automatic prune before `nPruneAfterHeight`

`FindFilesToPrune` (`blockstorage.cpp:400–402`):

```text
if (Height() <= PruneAfterHeight()) { return; }  // empty set — no error
```

**Automatic before/at prune-after = no-op empty plan, not an error** (matches D-05). Pruning may start only when `tip > prune_after_height`.

**Manual differs:** RPC uses `chainHeight < PruneAfterHeight()` → **error**, not empty plan (`rpc/blockchain.cpp:1261–1262`). When `tip == prune_after_height`, automatic is still no-op (`<=`), but manual RPC is allowed (`not <`).

Injected `nPruneAfterHeight` values from `kernel/chainparams.cpp` (adapters supply these; pure core must not hardcode):

| Network | `nPruneAfterHeight` | Lines |
|---------|---------------------|-------|
| Mainnet | `100000` | 133 |
| Testnet3 | `1000` | 291 |
| Testnet4 | `1000` | 390 |
| Signet | `1000` | 527 |
| Regtest | `100` if `fastprune` else `1000` | 603 |

### 5. Lock buffer direction, inclusivity, overlap behavior

Constants / types:

- `PRUNE_LOCK_BUFFER = 10` (`blockstorage.cpp:222`)
- `PruneLockInfo { height_first, height_last, ... }` inclusive lock range (`blockstorage.h:104–108`)

`DoPruneLocksForbidPruning` (`blockstorage.cpp:317–332`):

```text
lock_height = (height_first <= 11) ? 1 : (height_first - PRUNE_LOCK_BUFFER - 1)
lock_height_last = height_last + PRUNE_LOCK_BUFFER
forbid file if it overlaps (lock_height, lock_height_last]
  i.e. not (nHeightFirst > lock_height_last || nHeightLast <= lock_height)
```

Public note (`blockstorage.h:268–270`): only blocks **before** `(height_first - PRUNE_LOCK_BUFFER - 1)` and **after** `(height_last + PRUNE_LOCK_BUFFER)` are pruned.

**Height-granular protected set (normal case `height_first > 11`):**

`[height_first - 10, height_last + 10]` inclusive.

Derivation: forbidden when `H > lock_height && H <= lock_height_last` with `lock_height = height_first - 11` ⇒ `H >= height_first - 10` and `H <= height_last + 10`.

**Partial overlap:** `continue` — skip that file/candidate; **do not fail** the prune call (`blockstorage.cpp:357`, `445`). Matches D-10: omit protected heights; do not abort the whole plan.

### 6. How Knots chooses files for a byte target (height analogue)

`FindFilesToPrune` (`blockstorage.cpp:406–452`):

1. `nCurrentUsage = CalculateCurrentUsage()`.
2. `nBuffer = BLOCKFILE_CHUNK_SIZE (16 MiB) + UNDOFILE_CHUNK_SIZE (1 MiB)` — file-layout headroom.
3. Only enter selection if `nCurrentUsage + nBuffer >= target`.
4. Iterate `fileNumber` from `0` upward (**oldest first**).
5. Skip empty / out-of-range / lock-forbidden files (`continue`).
6. Else prune file, subtract its size from `nCurrentUsage`, continue.
7. **Break** when `nCurrentUsage + nBuffer < target` (stop once under target accounting for buffer).

**Pure height analogue for Phase 147:**

- Inject total usage and per-height sizes (ascending heights = oldest first).
- Eligible set: heights in `[0 ..= tip - 288]` (and `>= prune_start`), not lock-protected, mode automatic.
- Walk ascending; accumulate deleted bytes; stop when `remaining_usage <= target_bytes` (or `<` — pick one and test; Knots stops when `usage + buffer < target`).
- **Omit** the 17 MiB chunk buffer from the pure planner (Fjall has no blk/rev chunks). Document as layout difference for Phase 151. Optional injected `usage_buffer_bytes` is unnecessary this phase (D-06 lists tip, prune-after, mode, per-height sizes).

Manual does **not** use a byte budget — only height + locks (`FindFilesToPruneManual`).

### 7. Module home and error / decision types

**Home:** `packages/open-bitcoin-chainstate/src/prune/` (recommended over a single fat `prune.rs`):

```text
prune/
  mod.rs          # re-exports
  mode.rs         # PruneMode, parse_prune_arg, MIN_* constants
  range.rs        # last_prunable_height, keep-window predicates (GetPruneRange analogue)
  locks.rs        # PruneLockInfo fact, protected height check
  plan.rs         # plan_automatic / plan_manual → PrunePlan | refusal
  tests/          # fixtures: tip, prune-after, locks, sizes
```

Wire `pub mod prune;` from `lib.rs` and re-export the public decision API (same style as `coins` / `decide_flush`).

**Do not extend `ChainstateError`** for policy refusals — that enum is apply/connect/coins-storage oriented (`error.rs`). Follow `FlushDecision` / `RecoveryDecision`: exclusive outcome enums, e.g.:

- `PruneModeParseError::{Negative, BelowMinimum { requested_mib }}`
- `ManualPruneRefusal::{Disabled, ChainTooShort { tip, prune_after }, TargetInsideKeepWindow { tip, target, last_prunable }, TargetAboveTip { ... }}`
- `PrunePlan { heights: Vec<u32> /* or BTreeSet */ }` including empty automatic “not yet” plans as success with empty heights

Config parse: `Result<PruneMode, PruneModeParseError>`. Planning: `Result<PrunePlan, ManualPruneRefusal>` for manual; automatic returns `PrunePlan` (possibly empty) without error for prune-after / under-target.

### 8. Parity breadcrumbs

**Add a new group** `chainstate-prune` in `docs/parity/source-breadcrumbs.json` (do not overload `chainstate-engine`, which is coins/engine-focused). List every new `packages/open-bitcoin-chainstate/src/prune/**/*.rs` and matching tests.

**Knots breadcrumb paths to cite:**

- `packages/bitcoin-knots/src/validation.h` — `MIN_BLOCKS_TO_KEEP`, `MIN_DISK_SPACE_FOR_BLOCK_FILES`
- `packages/bitcoin-knots/src/validation.cpp` — `GetPruneRange`
- `packages/bitcoin-knots/src/node/blockmanager_args.cpp` — `ParsePruneOption`
- `packages/bitcoin-knots/src/node/blockstorage.h` — `PruneLockInfo`, `PRUNE_TARGET_MANUAL`, lock buffer note
- `packages/bitcoin-knots/src/node/blockstorage.cpp` — `PRUNE_LOCK_BUFFER`, `DoPruneLocksForbidPruning`, `FindFilesToPrune`, `FindFilesToPruneManual`
- Optional comment-only: `packages/bitcoin-knots/src/rpc/blockchain.cpp` — manual clamp vs OB refuse; `packages/bitcoin-knots/src/kernel/chainparams.cpp` — prune-after values (injected, not baked in)

File headers should repeat the same breadcrumb block pattern as `coins/flush.rs`.

### 9. Must NOT implement in this phase

| Item | Owner |
|------|-------|
| Fjall block/undo unlink | Phase 148 |
| `have_pruned` / `prunedblockfiles` | Phase 148 |
| `fFlushForPrune` / `FlushMode` changes | Phase 148 (D-13) |
| Emit `BlockServingDataAvailability::Pruned` | Phase 149 |
| `NODE_NETWORK_LIMITED` advertisement / serve window | Phase 149 |
| RPC / CLI / dashboard prune commands or status fields | Phase 150 |
| Persist / list / set prune locks | Phase 150 (LOCK-02) |
| Full prune parity docs + no-claim checkers | Phase 151 |
| `blk`/`rev` files, second chainstate, assumeutxo snapshot prune-start | Out of scope (D-14) |
| `-pruneduringinit`, txindex+prune | FUT-27 / out of scope |
| New production crate or third-party crate | D-17 |

## Standard Stack

### Core

| Library / facility | Version | Purpose | Why Standard |
|--------------------|---------|---------|--------------|
| Rust | `1.94.1` / edition 2024 | Typed enums + pure planners | Pinned by `rust-toolchain.toml` `[VERIFIED: rust-toolchain.toml / AGENTS.md]` |
| `open-bitcoin-chainstate` | workspace `0.1.0` | I/O-free prune decisions | Existing functional-core home; matches flush pattern `[VERIFIED: lib.rs, coins/flush.rs]` |
| Bitcoin Knots sources | `29.3.knots20260210` | Behavioral contract | Vendored submodule anchors `[VERIFIED: packages/bitcoin-knots]` |
| `std` (`BTreeMap` / slice) | std | Injected per-height sizes | No new deps `[ASSUMED]` sufficient for planner inputs |

### Supporting

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| Existing unit-test layout under `coins/tests/flush/` | — | Fixture style model | Mirror for `prune/tests/` |
| `docs/parity/source-breadcrumbs.json` | — | Auditable Knots anchors | Every new prune `.rs` file |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| `prune/` folder | Single `prune.rs` | Folder scales for mode/range/locks/plan; single file OK only if &lt; ~300 lines |
| New `Prune*Decision` enums | Extend `ChainstateError` | Error enum is wrong layer; flush already uses decision enums |
| Clamp manual keep-window like Knots RPC | Typed refusal | Locked by D-07 / PRUN-03 |
| Encode Knots chunk buffer in planner | Injected sizes only | Chunk sizes are blk/rev layout; Phase 151 documents difference |

**Installation:** none — no new packages.

**Version verification:** No new crates. Rust pin confirmed via repo `rust-toolchain.toml` / AGENTS.md (`1.94.1`). Knots pin confirmed via submodule status `v29.3.knots20260210`.

## Architecture Patterns

### Recommended Project Structure

```text
packages/open-bitcoin-chainstate/src/
├── coins/flush.rs          # pattern to copy (unchanged)
├── prune/
│   ├── mod.rs
│   ├── mode.rs             # PruneMode + parse
│   ├── range.rs            # tip−288, prune-after gates
│   ├── locks.rs            # buffer expand + forbid predicate
│   ├── plan.rs             # automatic + manual planners
│   └── tests/
└── lib.rs                  # pub mod prune; re-exports
```

### Pattern 1: Injected facts → exclusive decision (flush analogue)

**What:** Pure function takes a struct of already-sampled facts; returns an enum that cannot mix success with refusal incorrectly.
**When to use:** All prune planning and mode parse.
**Example:**

```rust
// Pattern source: packages/open-bitcoin-chainstate/src/coins/flush.rs (decide_flush)
pub fn decide_flush(input: FlushPolicyInput) -> FlushDecision { /* ... */ }

// Phase 147 analogue:
pub fn plan_automatic_prune(input: AutomaticPruneInput) -> PrunePlan { /* empty or heights */ }
pub fn plan_manual_prune(input: ManualPruneInput) -> Result<PrunePlan, ManualPruneRefusal> { /* ... */ }
```

### Pattern 2: Parse integers at the boundary

**What:** `parse_prune_arg(i64) -> Result<PruneMode, PruneModeParseError>` once; planners accept only `PruneMode`.
**When to use:** Any future JSONC/clap/RPC path (Phase 150) must call this — not reimplement MiB math.

### Anti-Patterns to Avoid

- **Raw `u64` prune target in core:** Violates D-01; confuses `PRUNE_TARGET_MANUAL` with a huge MiB budget.
- **Baking mainnet `100000` into planners:** Violates D-05; inject prune-after.
- **Silent clamp for manual keep-window:** Violates PRUN-03 / D-07.
- **Failing whole plan on lock overlap:** Knots skips; D-10 requires omit.
- **Touching `FlushMode` or emitting `Pruned`:** Phase 148/149 work.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Keep-window math | Invent tip−287 vs tip−288 | Quote `GetPruneRange` | Off-by-one reorg hazard |
| Mode parsing | Ad-hoc `if n >= 550` without bytes | Mirror `ParsePruneOption` then map to enum | `2..=549` refusal is byte-floor based |
| Lock buffer | ±10 both sides naively without −1 quirk | `DoPruneLocksForbidPruning` formula | Knots uses `height_first - 11` / `height_last + 10` |
| File walking | Fjall keyscan in chainstate | Injected sizes + height plan | Core must stay I/O-free |

**Key insight:** Behavioral parity is the comparisons and refusals, not `blk?????.dat` selection units.

## Common Pitfalls

### Pitfall 1: tip − 287 vs tip − 288

**What goes wrong:** Pruning one height too close to tip breaks undo/reorg safety.
**Why it happens:** “Keep 288” misread as `tip - 287` inclusive last prune.
**How to avoid:** Unit test `last_prunable(1000) == 712`; property `tip - last_prunable == 288` when `tip >= 288`.
**Warning signs:** Tests that prune `tip - 10` and call it success.

### Pitfall 2: Copying Knots manual clamp

**What goes wrong:** Operator thinks they pruned to tip−10; actually clamped (Knots) or OB must refuse (locked).
**Why it happens:** Reading RPC without CONTEXT D-07.
**How to avoid:** Test that `target == tip - 287` is `TargetInsideKeepWindow`; `target == tip - 288` is allowed.

### Pitfall 3: Treating automatic prune-after empty plan as error

**What goes wrong:** Startup/IBD spam or aborted sync when tip ≤ prune-after.
**Why it happens:** Confusing automatic `<=` no-op with manual `<` error.
**How to avoid:** Separate automatic vs manual APIs; empty `PrunePlan` is `Ok`.

### Pitfall 4: Lock buffer off-by-one (`−10` vs `−11`)

**What goes wrong:** Deletes one height inside Knots' protected skirt.
**Why it happens:** Header says “buffer 10” without reading `height_first - PRUNE_LOCK_BUFFER - 1`.
**How to avoid:** Fixture lock `{first: 100, last: 100}` forbids `[90, 110]` inclusive.

### Pitfall 5: Setting have-pruned or Pruned from policy alone

**What goes wrong:** Label/serving lies before Phase 148 deletes.
**Why it happens:** Temptation to “prepare” flags in the policy phase.
**How to avoid:** Planner returns heights only; no durable flag types mutated here.

## Code Examples

### Mode parse (Knots mapping)

```rust
// Source comparisons: packages/bitcoin-knots/src/node/blockmanager_args.cpp:20-34
//                     packages/bitcoin-knots/src/validation.h:82
pub const MIN_DISK_SPACE_FOR_BLOCK_FILES_MIB: u64 = 550;
pub const MIN_BLOCKS_TO_KEEP: u32 = 288;
pub const PRUNE_LOCK_BUFFER: u32 = 10;

pub enum PruneMode {
    Disabled,
    ManualOnly,
    Automatic { target_mib: u64 },
}

pub fn parse_prune_arg(n_prune_arg: i64) -> Result<PruneMode, PruneModeParseError> {
    if n_prune_arg < 0 {
        return Err(PruneModeParseError::Negative);
    }
    if n_prune_arg == 0 {
        return Ok(PruneMode::Disabled);
    }
    if n_prune_arg == 1 {
        return Ok(PruneMode::ManualOnly);
    }
    let requested_mib = n_prune_arg as u64;
    if requested_mib < MIN_DISK_SPACE_FOR_BLOCK_FILES_MIB {
        return Err(PruneModeParseError::BelowMinimum { requested_mib });
    }
    Ok(PruneMode::Automatic {
        target_mib: requested_mib,
    })
}
```

### Keep window + automatic prune-after

```rust
// Source: validation.cpp GetPruneRange; blockstorage.cpp FindFilesToPrune early return
pub fn last_prunable_height(tip: u32) -> u32 {
    tip.saturating_sub(MIN_BLOCKS_TO_KEEP)
}

pub fn automatic_prune_allowed(tip: u32, prune_after_height: u32) -> bool {
    tip > prune_after_height
}

pub fn manual_target_inside_keep_window(tip: u32, target: u32) -> bool {
    // Knots RPC: height > chainHeight - MIN_BLOCKS_TO_KEEP
    target > tip.saturating_sub(MIN_BLOCKS_TO_KEEP)
}
```

### Lock protection (height analogue)

```rust
// Source: blockstorage.cpp:317-332 and blockstorage.h:268-270
pub fn height_forbidden_by_lock(height: u64, height_first: u64, height_last: u64) -> bool {
    let lock_height = if height_first <= u64::from(PRUNE_LOCK_BUFFER) + 1 {
        1
    } else {
        height_first - u64::from(PRUNE_LOCK_BUFFER) - 1
    };
    let lock_height_last = height_last.saturating_add(u64::from(PRUNE_LOCK_BUFFER));
    height > lock_height && height <= lock_height_last
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| No prune policy in OB | Pure height/lock/mode decisions | v2.4 Phase 147 | Unlink (148) can consume plans safely |
| Knots file-number prune units | Height + injected sizes | Open Bitcoin Fjall layout | Document in Phase 151; do not invent blk/rev |
| Knots RPC clamp near tip | OB typed refusal | Locked D-07 | Clearer operator/API contract |

**Deprecated/outdated:** Treating “missing payload” as pruned (forbidden until Phase 149 after real delete).

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | Sorted slice or `BTreeMap<u32, u64>` are equally fine for injected sizes | Discretion / Standard Stack | Low — API shape only |
| A2 | Omitting Knots 17 MiB chunk buffer from automatic stop condition is correct for Fjall | §6 height analogue | Medium — may prune slightly more aggressively than Knots for same MiB target; Phase 151 must document |
| A3 | Manual `TargetAboveTip` should be a typed refusal in Phase 147 (Knots RPC errors) | §3 | Low — can defer string parity to Phase 150 |

**If this table is empty:** N/A — three discretionary assumptions remain for planner confirmation.

## Open Questions

1. **Automatic stop predicate: `remaining <= target` vs `remaining < target`?**
   - What we know: Knots breaks when `usage + buffer < target`.
   - What's unclear: Without buffer, exact equality edge for height sums.
   - Recommendation: Stop when `remaining_usage <= target_bytes` after subtracting a candidate; unit-test equality.

2. **Should `parse_prune_arg` accept `u64` MiB directly as a second constructor for Phase 150 JSONC?**
   - What we know: Knots argv is signed `int64` MiB count.
   - Recommendation: Keep `i64` parse for Knots parity; add `PruneMode::automatic_mib(u64) -> Result<...>` for already-validated non-negative paths if needed.

3. **Prune-start for single chainstate always 0?**
   - What we know: `GetPruneRange` sets `prune_start` non-zero only for snapshot/background dual-chainstate (out of scope D-14).
   - Recommendation: Hardcode start `0` / omit snapshot branch; document intentionally.

## Environment Availability

Step 2.6: SKIPPED (no new external dependencies). Phase is pure Rust in an existing crate; no new CLIs, databases, or services. Existing toolchain (Rust `1.94.1`, Bun for breadcrumb checker) is already the repo contract — do not install anything for this phase.

## Security Domain

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|------------------|
| V2 Authentication | no | N/A — no operator surface this phase |
| V3 Session Management | no | N/A |
| V4 Access Control | no | N/A — locks are injected facts, not auth |
| V5 Input Validation | yes | `parse_prune_arg` refuses negative / `2..=549`; manual target bounds |
| V6 Cryptography | no | N/A — no new crypto |

### Known Threat Patterns for prune policy

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Prune inside tip keep window | Tampering / Denial of availability | Hard ceiling `tip - 288`; refuse illegal manual targets |
| Delete under active lock | Tampering | Expand lock by buffer; omit heights; never ignore locks |
| Config below 550 MiB “automatic” | Elevation of misconfig | Typed parse refusal |
| Policy phase setting have-pruned | Spoofing (label lie) | Do not mutate durable flags here |

## Sources

### Primary (HIGH confidence)

- `packages/bitcoin-knots/src/validation.h:70-82` — `MIN_BLOCKS_TO_KEEP`, `MIN_DISK_SPACE_FOR_BLOCK_FILES`
- `packages/bitcoin-knots/src/validation.cpp:6910-6934` — `GetPruneRange`
- `packages/bitcoin-knots/src/validation.cpp:3089-3106` — manual vs automatic flush prune branch
- `packages/bitcoin-knots/src/node/blockmanager_args.cpp:20-34` — `ParsePruneOption`
- `packages/bitcoin-knots/src/node/blockstorage.h:104-116, 268-272, 375` — `PruneLockInfo`, lock note, `PRUNE_TARGET_MANUAL`
- `packages/bitcoin-knots/src/node/blockstorage.cpp:222, 317-459` — buffer, forbid, FindFilesToPrune*
- `packages/bitcoin-knots/src/rpc/blockchain.cpp:1231-1271` — manual RPC refusals / clamp
- `packages/bitcoin-knots/src/kernel/chainparams.cpp` — `nPruneAfterHeight` per network
- `packages/open-bitcoin-chainstate/src/coins/flush.rs` — I/O-free decision pattern
- `.planning/phases/147-.../147-CONTEXT.md` — locked D-01..D-17

### Secondary (MEDIUM confidence)

- `.planning/research/{STACK,FEATURES,PITFALLS,ARCHITECTURE}.md` — milestone framing; verified against Knots above, not used as substitute for C++ lines

### Tertiary (LOW confidence)

- None material; A2 chunk-buffer omission flagged for Phase 151 documentation

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH — no new deps; existing crate + Knots pin verified in-tree
- Architecture: HIGH — flush analogue + CONTEXT module discretion; dual-chainstate branch explicitly out of scope
- Pitfalls: HIGH — keep window, lock buffer, clamp-vs-refuse, prune-after asymmetry all line-cited

**Research date:** 2026-09-22
**Valid until:** 2026-10-22 (stable Knots pin; re-verify if submodule moves)
