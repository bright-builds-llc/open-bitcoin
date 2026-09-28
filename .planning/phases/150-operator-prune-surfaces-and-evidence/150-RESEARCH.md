# Phase 150: Operator Prune Surfaces and Evidence - Research

**Researched:** 2026-09-28
**Domain:** Knots-aligned operator prune status, manual prune, prune locks, and sanitized support evidence
**Confidence:** HIGH
**Lifecycle:** yolo
**phase_lifecycle_id:** 150-2026-09-28T17-08-23

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

### Prune status fields

- **D-01:** OPER-01 is the Knots `getblockchaininfo` quartet: `pruned`, `pruneheight`, `automatic_pruning`, and `prune_target_size`. Project the same facts on the Open Bitcoin status snapshot, the CLI status view, and the terminal dashboard.
- **D-02:** `getblockchaininfo.pruned` means prune mode is on (`PruneMode::ManualOnly` or `PruneMode::Automatic`). It is not the Phase 149 block-availability label. Configured mode alone does not set durable have-pruned and does not relabel a missing payload as `Pruned`.
- **D-03:** `automatic_pruning` is true only for `PruneMode::Automatic`. `prune_target_size` is the automatic target in bytes and is present only when automatic pruning is on. Manual-only reports `automatic_pruning: false` and omits `prune_target_size`. Disabled omits the prune-only keys and reports `pruned: false`.
- **D-04:** `pruneheight` is the Knots lowest-height complete block still stored, present only when prune mode is on. Research must quote the pinned Knots `getblockchaininfo` / prune-height helper before planning the integer. Do not substitute the support-evidence last-delete height for this field.
- **D-05:** Keep Phase 149 `pruned_count` and `BlockServingDataAvailability::Pruned` on the earned rule: have-pruned is set, the payload is gone, and the block is validated on the active chain. Status JSON must expose configured-mode fields and the earned count as separate facts.

### Config so those reads are real

- **D-06:** Wire the existing `parse_prune_arg` integer into Open Bitcoin JSONC as `prune`. `0` is disabled, `1` is manual-only, and `N >= 550` is an automatic target in MiB. Values `2..=549` and negatives stay typed refusals. Default remains `0`.
- **D-07:** Do not add a second constructor or a parallel byte-target field. Phase 147 left JSONC for this phase; the integer contract stays the one `parse_prune_arg` already implements.
- **D-08:** The running node, RPC, CLI status, and dashboard all read that resolved mode. Tests may still call `set_prune_mode` directly.

### Manual prune request

- **D-09:** OPER-02 is Knots `pruneblockchain <height>` on RPC, plus a CLI command that requests the same manual prune. Dashboard shows the height or typed refusal and does not offer a destructive prune control.
- **D-10:** Disabled mode returns the existing typed refusal and does not delete payloads or set have-pruned. A target inside the 288-block keep window is refused without clamping. Success returns the resulting prune height after the Phase 148 durable delete path runs.
- **D-11:** Manual prune does not invent an automatic byte-budget plan. Automatic mode may still accept `pruneblockchain` the way Knots does once the manual height check passes; research quotes that Knots rule before planning.

### Prune locks

- **D-12:** LOCK-02 lists and sets named locks through RPC and CLI. A lock is the existing `PruneLockInfo`: name plus inclusive `height_first` and `height_last`. The same name replaces the previous range.
- **D-13:** Setting includes a clear-by-name so a lock is not one-way. Dashboard lists locks and does not edit them.
- **D-14:** Locks persist in node storage and survive restart. They feed the existing pure lock-buffer planner. This phase does not change the 10-block buffer formula.

### Sanitized support evidence

- **D-15:** OPER-03 adds a prune section to the existing support-evidence JSON and Markdown bundle.
- **D-16:** The section reports prune counts and the last prune height: how many successful prune batches have committed, how many heights lost payloads, and the highest height deleted by the last successful batch. Before any successful delete, counts are zero and last prune height is absent.
- **D-17:** The section does not include datadir paths, Fjall key names, filesystem paths, block hashes, or raw lock names that embed paths. Follow the existing support-bundle redaction pattern.

### Boundaries carried forward

- **D-18:** No archive serving, assumeutxo, BIP37, compact-filter serving, public serving or relay defaults, public-network CI, production full-node claims, or production-funds wallet claims.
- **D-19:** Phase 151 owns parity catalog docs and no-claim checkers. This phase still adds parity breadcrumbs for new first-party Rust under `packages/open-bitcoin-*/src` and `packages/open-bitcoin-*/tests`.
- **D-20:** Verification remains `bash scripts/verify.sh`. Default verification stays deterministic and public-network-free. No new production crate or third-party library.

### Claude's Discretion

- Exact dashboard section layout, as long as the four status facts and the lock list are visible and no control deletes payloads.
- CLI subcommand spelling, as long as RPC stays `pruneblockchain`, list, set, and clear, and the CLI calls that same behavior.
- Whether last-prune counters live beside the have-pruned marker, as long as they advance only after a durable delete succeeds.
- JSONC file placement for `prune`, as long as it uses `parse_prune_arg` and defaults to disabled.

### Deferred Ideas (OUT OF SCOPE)

- Parity catalog docs and no-claim checkers — Phase 151 (GRD-01).
- `-pruneduringinit`, txindex-plus-prune, archive serving, assumeutxo, BIP37, compact filters, and public defaults stay outside v2.4.
- A dashboard control that deletes block payloads. Status and last-outcome display only.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| OPER-01 | Operator can read pruned, prune height, automatic pruning, and prune target from RPC, CLI, and dashboard. | Quote `getblockchaininfo` and implement `project_prune_status` from `PruneMode` plus `GetPruneHeight`. Wire JSONC `prune` through `parse_prune_arg` into the running node. Copy the quartet into `OpenBitcoinStatusSnapshot` so CLI status and the dashboard render it. Keep `pruned_count` on the earned label path. |
| OPER-02 | Operator can request a manual prune when prune mode is on. | `pruneblockchain` calls `plan_manual_prune` then `flush_applying_prune_plan`. Disabled and keep-window targets refuse before any delete. Return the Knots `pruneblockchain` integer (`GetPruneHeight`, not the `+ 1` info field). Automatic mode is accepted. Dashboard displays the outcome and has no delete action. |
| OPER-03 | Support evidence reports prune counts and the last prune height without raw storage paths. | Add a sibling prune section to the support bundle. Counts and last-batch max height advance only after a durable `DeletedLiveMate`. Redact paths, Fjall key names, block hashes, and lock names. Do not reuse `getblockchaininfo.pruneheight`. |
| LOCK-02 | Operator can list and set prune locks. | RPC `listprunelocks`, `setprunelock`, and `clearprunelock` on the existing `PruneLockInfo`. Same name replaces. Clear of a missing name returns success false. Persist in the Fjall block-index namespace and pass the loaded slice into startup `resume_prune_intent`. Dashboard lists only. |
</phase_requirements>

## Summary

Phase 150 exposes facts and controls that Phases 147–149 already computed. It does not change the keep window, the 10-block lock buffer, the paired-delete batch, or the earned `Pruned` label. The planning hazard is three different integers that all sound like "prune height," plus a production startup path that still leaves prune mode disabled and resumes interrupted deletes with an empty lock slice.

`getblockchaininfo.pruned` is the configured mode bit. `getblockchaininfo.pruneheight` is the Knots helper below (last contiguous pruned height plus one, or `0` when nothing is pruned). `pruneblockchain` returns that helper without the plus one, or `-1` when nothing is pruned. Support evidence's last prune height is the highest height in the last successful delete batch, and it is omitted until such a batch exists. `pruned_count` stays the earned availability counter.

**Primary recommendation:** Add one pure `project_prune_status` function in `open-bitcoin-chainstate`, persist locks and last-batch counters in the existing Fjall block-index namespace, and have RPC, CLI, dashboard, and support evidence read those projections. Call `plan_manual_prune` and then the Phase 148 flush that commits paired deletes. Do not clamp the 288-block window. Do not open a second database from the CLI while the node holds Fjall.

## Project Constraints

`.cursor/rules/` is absent. These repo rules bind the plan. [VERIFIED: AGENTS.md, standards/index.md, standards/core/architecture.md, standards/core/code-shape.md, standards/languages/rust.md, standards/core/testing.md]

- Functional core stays I/O-free. `plan_manual_prune`, lock-buffer checks, and the new status integer projection live in `open-bitcoin-chainstate`. Fjall, RPC, CLI, dashboard, and process startup stay in the node and operator shells.
- New first-party Rust under `packages/open-bitcoin-*/src` and `packages/open-bitcoin-*/tests` needs a parity breadcrumb block registered in `docs/parity/source-breadcrumbs.json`. Use the explicit `none` reason only when no Knots anchor exists. `scripts/check-parity-breadcrumbs.ts` fails a missing or stale block. [VERIFIED: scripts/check-parity-breadcrumbs.ts]
- Do not add a production crate or a third-party library (D-20). Do not use rust-bitcoin.
- Prefer `foo.rs` plus `foo/` for new modules. Prefer `let...else`. Prefix optional locals with `maybe_`. No `unwrap()`.
- `bun scripts/bright-builds-check.ts file-lengths` fails at 629 physical lines. `packages/open-bitcoin-node/src/network/runtime_authority.rs` is 626 lines and `packages/open-bitcoin-node/src/status.rs` is 622. Put new functions in existing sibling modules such as `network/runtime_authority/prune_flush.rs` and a new `status/prune_operator.rs`. Do not grow those two files past the checker.
- Verification command is `bash scripts/verify.sh`. It is deterministic and public-network-free.
- Phase 151 writes parity catalog docs. This phase still records the intentional keep-window refusal (no Knots clamp) in code comments and tests, not in `docs/parity/`.

## Standard Stack

No new crates. Use the code that already implements the policy.

### Core

| Library / crate | Version | Purpose | Why Standard |
|-----------------|---------|---------|--------------|
| `open-bitcoin-chainstate` prune module | workspace, Rust 1.94.1 | `parse_prune_arg`, `PruneMode`, `plan_manual_prune`, `PruneLockInfo`, 288 keep, 10-block buffer | Locked by Phases 147 and D-07/D-14. [VERIFIED: mode.rs, plan.rs, locks.rs, rustc 1.94.1] |
| `open-bitcoin-node` Fjall store | Fjall already in the node crate | Paired payload+undo delete, `have_pruned`, `prune_intent`, new lock and counter keys | Phase 148 durable path. [VERIFIED: storage/fjall_store/prune.rs] |
| `open-bitcoin-rpc` JSON-RPC registry | existing Axum/serde | `getblockchaininfo`, `pruneblockchain`, list, set, clear | Baseline methods keep Knots names. [VERIFIED: method.rs, blockchain.cpp] |
| `open-bitcoin-cli` operator status, dashboard, support | clap, Ratatui, existing snapshot | Read-only dashboard, CLI that calls the same behavior, redacted support bundle | Existing operator surfaces. [VERIFIED: operator.rs, dashboard/model.rs, support.rs] |

### Supporting

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `serde` / `serde_json` | existing | Optional `getblockchaininfo` keys via `skip_serializing_if` | Omit `pruneheight`, `automatic_pruning`, and `prune_target_size` when their presence rules say so |
| jsonc-parser | existing | `open-bitcoin.jsonc` | Parse the new `prune` integer; `OpenBitcoinConfig` already has `#[serde(default, deny_unknown_fields)]` |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Existing `parse_prune_arg` | A JSONC boolean plus a byte field | Forbidden by D-07. |
| Existing `plan_manual_prune` + Phase 148 flush | Direct `commit_paired_delete` from RPC | Skips keep-window re-check, lock re-check, cache eviction, and `prune_intent`. |
| Existing `PruneLockInfo` | Knots `desc`, `temporary`, and `"*"` | D-12 locks the existing struct. Knots `temporary` defaults to true and skips disk. That contradicts D-14. |
| Knots clamp of near-tip `pruneblockchain` | Open Bitcoin refusal | Already locked by Phase 147 PRUN-03 and D-10. |

**Installation:** none. Do not add packages.

**Version verification:** `rustc 1.94.1 (e408947bf 2026-03-25)`, `cargo 1.94.1`, `bun 1.4.2` on this machine. [VERIFIED: shell]

## Quoted Knots Contracts

### `ParsePruneOption`

`packages/bitcoin-knots/src/node/blockmanager_args.cpp` lines 20–35. [VERIFIED: source read]

```cpp
util::Result<uint64_t> ParsePruneOption(const int64_t nPruneArg, const std::string_view opt_name)
{
    if (nPruneArg < 0) {
        return util::Error{strprintf(_("%s cannot be configured with a negative value."), opt_name)};
    } else if (nPruneArg == 0) {
        return 0;
    } else if (nPruneArg == 1) {
        return BlockManager::PRUNE_TARGET_MANUAL;
    }
    const uint64_t nPruneTarget{uint64_t(nPruneArg) * 1024 * 1024};
    if (nPruneTarget < MIN_DISK_SPACE_FOR_BLOCK_FILES) {
        return util::Error{... below the minimum of 550 MiB ...};
    }
    return nPruneTarget;
}
```

`MIN_DISK_SPACE_FOR_BLOCK_FILES` is `550 * 1024 * 1024` (`validation.h` line 82). `PRUNE_TARGET_MANUAL` is `std::numeric_limits<uint64_t>::max()` (`blockstorage.h` line 375). `m_prune_mode` is `opts.prune_target > 0` (`blockstorage.cpp` line 1315), so both manual-only and automatic are prune mode. Disabled is target `0`.

`parse_prune_arg` already matches this integer contract and stores MiB, not the Knots sentinel. [VERIFIED: packages/open-bitcoin-chainstate/src/prune/mode.rs] `prune_target_size` on the wire is bytes: `target_mib * 1024 * 1024`. Do not emit MiB.

### `getblockchaininfo` quartet

Always push `pruned` (`blockman.IsPruneMode()`). Only inside prune mode, push `pruneheight` and `automatic_pruning`. Push `prune_target_size` only when automatic. [VERIFIED: blockchain.cpp lines 1802–1811]

```cpp
obj.pushKV("pruned", chainman.m_blockman.IsPruneMode());
if (chainman.m_blockman.IsPruneMode()) {
    const auto prune_height{GetPruneHeight(chainman.m_blockman, active_chainstate.m_chain)};
    obj.pushKV("pruneheight", prune_height ? prune_height.value() + 1 : 0);
    const bool automatic_pruning{chainman.m_blockman.GetPruneTarget() != BlockManager::PRUNE_TARGET_MANUAL};
    obj.pushKV("automatic_pruning", automatic_pruning);
    if (automatic_pruning) {
        obj.pushKV("prune_target_size", chainman.m_blockman.GetPruneTarget());
    }
}
```

Help text for `pruneheight` is "height of the last block pruned, plus one (only present if pruning is enabled)" (`blockchain.cpp` line 1763). The key is absent when pruning is disabled. When pruning is enabled and `GetPruneHeight` is empty, the value is `0`, not absent.

`automatic_pruning` is `GetPruneTarget() != PRUNE_TARGET_MANUAL`. That is `PruneMode::Automatic` only. Manual-only is `automatic_pruning: false` and omits `prune_target_size`. This matches D-03.

### `GetPruneHeight` — the integer planner must implement

`packages/bitcoin-knots/src/rpc/blockchain.cpp` lines 1026–1049. [VERIFIED: source read]

The comment says "height of highest block that has been pruned, or `std::nullopt` if no blocks have been pruned." The algorithm:

1. Start at `chain[1]`. Genesis is not a prune candidate because it has no undo data.
2. If there is no block after genesis or no tip, return nullopt.
3. If the tip itself lacks `BLOCK_HAVE_MASK` (both block data and undo), return the tip height.
4. Otherwise walk back from the tip, with `chain[1]` as the floor, to the earliest block that still begins a contiguous suffix of complete blocks (`GetFirstBlock` with `BLOCK_HAVE_MASK`).
5. If that block is height 1, return nullopt.
6. Otherwise return the previous block's height (the last pruned block in that suffix).

`getblockchaininfo.pruneheight` is that value plus one, or `0` when the helper returns nullopt.

D-04's phrase "lowest-height complete block still stored" matches this plus-one result only for one contiguous pruned prefix under a complete tip. It does not match a hole. Example: heights 1–10 stored, 11 missing, 12..tip stored. The walk stops at 12, last pruned height is 11, and `pruneheight` is 12. A `min(stored complete heights)` scan would report 1 and is wrong. Plan the quoted helper, not a minimum scan, and not the support-evidence last-delete height.

Open Bitcoin "complete" means the active-chain hash at that height has both payload and undo (`has_block` and `has_undo`). Start the walk at height 1. If the tip hash lacks either mate, `pruneheight = tip + 1`.

### `pruneblockchain` return and refusal

`packages/bitcoin-knots/src/rpc/blockchain.cpp` lines 1213–1271. RPC result help says "Height of the last block pruned". The return is `GetPruneHeight(...).value_or(-1)` after `PruneBlockFilesManual`. It does not add one. [VERIFIED: source read]

Use this integer as D-10's "resulting prune height." Do not return `getblockchaininfo.pruneheight`.

| Condition | Knots behavior | Open Bitcoin plan |
|-----------|----------------|-------------------|
| Not prune mode | `RPC_MISC_ERROR` (-1): "Cannot prune blocks because node is not in prune mode." No delete. | `ManualPruneRefusal::Disabled` before any store call. Map to `RpcErrorCode::MiscError`. |
| Height 0 | Return `0` immediately. No prune. | Match this early return. `plan_manual_prune(0)` is not a no-op on a tall chain. |
| Negative | `RPC_INVALID_PARAMETER` (-8): "Negative block height." | Accept `i64`. Map to `RpcErrorCode::InvalidParameter`. |
| Value `> 1_000_000_000` | Treat as a Unix timestamp. Subtract `TIMESTAMP_WINDOW` (7200 seconds, `chain.h` lines 29 and 37). `FindEarliestAtLeast` that adjusted time. Missing block: `RPC_INVALID_PARAMETER`. | Pure pre-step over injected `(height, time)` pairs, then `plan_manual_prune` on the resolved height. Do not put header I/O in chainstate. |
| `tip < PruneAfterHeight` | `RPC_MISC_ERROR`: "Blockchain is too short for pruning." | `ManualPruneRefusal::ChainTooShort`. Keep the Phase 147 `<` comparison (not `<=`). `MiscError`. |
| Target above tip | `RPC_INVALID_PARAMETER`: "Blockchain is shorter than the attempted prune height." | `ManualPruneRefusal::TargetAboveTip`. `InvalidParameter`. |
| Target inside `tip - 288` | Knots logs and clamps to `chainHeight - MIN_BLOCKS_TO_KEEP`. `MIN_BLOCKS_TO_KEEP` is 288 (`validation.h` line 71). | Refuse `ManualPruneRefusal::TargetInsideKeepWindow` with no clamp (D-10, PRUN-03). `InvalidParameter`. |
| Checks pass | `PruneBlockFilesManual` → `FlushStateToDisk(..., nManualPruneHeight)`. Return last pruned height or `-1`. | `plan_manual_prune`, then `flush_applying_prune_plan`. Return the helper value or `-1`. |

`PruneBlockFilesManual` (`validation.cpp` lines 5027–5033) only flushes. It does not check manual versus automatic. `IsPruneMode()` is the only mode gate, and it is true for both `prune=1` and an automatic byte target (D-11). `plan_manual_prune` already refuses only `PruneMode::Disabled`. Do not call `plan_automatic_prune` from this RPC. `GetPruneRange` (`validation.cpp` lines 6910–6934) still clamps the file scan to `min(target, tip - 288)`. Open Bitcoin's planner refuses that clamp instead of applying it. Do not add a second clamp in the shell.

### `PruneLockInfo` and lock RPC

Struct at `blockstorage.h` lines 104–116. [VERIFIED: source read]

```cpp
struct PruneLockInfo {
    std::string desc;
    uint64_t height_first{std::numeric_limits<uint64_t>::max()};
    uint64_t height_last{std::numeric_limits<uint64_t>::max()};
    bool temporary{true};
    // serialized: desc, height_first, height_last. temporary is not serialized.
};
```

Open Bitcoin's struct is already the D-12 subset: `name`, `height_first: u32`, `height_last: u32` (`locks.rs`). Do not add `desc`, `temporary`, or `"*"`.

Knots RPC names (`blockchain.cpp` registration lines 4129–4131): `listprunelocks`, `setprunelock`, `pruneblockchain`. `setprunelock` both writes and deletes. Delete is a null height; missing name returns success false (`PruneLockExists && DeletePruneLock`). `id == "*"` deletes every lock. `temporary` defaults true, and `WritePruneLock` returns without writing when `temporary` is true (`blockstorage.cpp` lines 102–104). `UpdatePruneLock` replaces by name.

Discretion asks for list, set, and clear as separate RPC behavior. Use these names:

- `listprunelocks` — array of `{ "name", "height_first", "height_last" }`. No `desc`, `temporary`, or path-like extras.
- `setprunelock` — `name`, `height_first`, `height_last`. Same name replaces. Refuse `height_last < height_first`, empty name, and a `height_last` that would overflow `height_last + PRUNE_LOCK_BUFFER` in `height_forbidden_by_lock` (that add is not saturating). Do not change the formula.
- `clearprunelock` — clear by name. Missing name returns `{ "success": false }` and writes nothing. No `"*"` clear-all.

Every set and clear syncs to Fjall before success (D-14). Do not offer Knots' `sync: false` or `temporary: true` defaults.

`DoPruneLocksForbidPruning` (`blockstorage.cpp` lines 222 and 317–324) is the already-ported buffer. `PRUNE_LOCK_BUFFER` is 10. Leave `height_forbidden_by_lock` unchanged.

## Architecture Patterns

### Recommended module placement

```text
packages/open-bitcoin-chainstate/src/prune/status.rs   # pure quartet + GetPruneHeight
packages/open-bitcoin-node/src/storage/fjall_store/    # lock map + counter record beside have_pruned
packages/open-bitcoin-node/src/network/runtime_authority/prune_flush.rs
                                                       # public manual-prune request; file is the live caller
packages/open-bitcoin-node/src/status/prune_operator.rs
packages/open-bitcoin-rpc/src/method/prune.rs
packages/open-bitcoin-rpc/src/dispatch/prune.rs
packages/open-bitcoin-cli/src/operator/                    # Prune subcommand calling those RPCs
packages/open-bitcoin-rpc/src/config/open_bitcoin.rs       # top-level prune: i64
```

### Pattern 1: One pure projection for the quartet

**What:** Chainstate maps `PruneMode` plus the `GetPruneHeight` input facts into the JSON shape.
**When to use:** RPC `getblockchaininfo`, the status snapshot, CLI render, and dashboard rows all call it.
**Example:**

```rust
pub struct PruneStatusProjection {
    pub pruned: bool,
    pub maybe_pruneheight: Option<u32>,
    pub maybe_automatic_pruning: Option<bool>,
    pub maybe_prune_target_size: Option<u64>,
}

/// `maybe_last_pruned_height` is `GetPruneHeight` (None if nothing is pruned).
/// Disabled clears every optional key. Manual-only sets automatic to false and
/// omits the target. Automatic sets the target in bytes.
pub fn project_prune_status(
    mode: PruneMode,
    maybe_last_pruned_height: Option<u32>,
) -> PruneStatusProjection { /* ... */ }
```

`maybe_pruneheight` is `Some(last + 1)` or `Some(0)`, and it is `None` only when mode is disabled. `pruned_count` is not an input.

### Pattern 2: Shell resolves, core decides, Phase 148 deletes

**What:** RPC parses the height or timestamp, loads tip, prune-after height, mode, present heights, header times, and locks, then calls `plan_manual_prune`. On `Ok`, call `flush_applying_prune_plan` with that plan and the same lock slice. On `Err`, return the mapped RPC error and do not touch storage.
**When to use:** `pruneblockchain` only.
**Why this seam:** `flush_applying_prune_plan` (`prune_flush.rs`) is the production wrapper that flushes, applies the plan, and evicts deleted hashes from `blocks_by_hash`. It is `#[cfg_attr(not(test), allow(dead_code))]` today. This phase is the production caller. Remove that allowance once the RPC path calls it. `runtime_authority.rs` cannot absorb the new function.

`apply_prune_plan` re-checks locks and the keep window and skips those heights instead of failing the batch (`prune_apply.rs`). A lock that forbids every candidate still returns the Knots result height (`-1` or the previous last pruned height) after a successful no-delete flush. It must not set have-pruned or advance support counters. `AlreadyAbsent` is the same: not a lost payload.

### Pattern 3: Config integer in, typed mode out

**What:** Add `prune: i64` to `OpenBitcoinConfig` with default `0`. [VERIFIED: the struct has `#[serde(default, deny_unknown_fields)]` and only two struct literals, both in `config/open_bitcoin.rs`]
**When to use:** JSONC load. Run `parse_prune_arg` at the boundary. Store `PruneMode` on `RuntimeConfig`. On node startup, call the existing `ManagedNetworkHandle::set_prune_mode`.
**Today's gap:** Production startup never calls `set_prune_mode`. The field is constructed as `PruneMode::Disabled` in `network/relay_serving.rs`. RPC `set_prune_mode` is `#[cfg(test)]` only (`context.rs`). D-08 is not true until startup applies the resolved mode.
**Placement recommendation:** Top-level `prune` on `open-bitcoin.jsonc`. That is the D-06 key. Invalid integers (`2..=549`, negatives) are config errors, not a silent disable.

### Pattern 4: Live reads go through RPC

**What:** `collect_status_snapshot` uses `getblockchaininfo` when live RPC is on, and it does not open Fjall (`operator/status.rs`). Probe-only support says it does not open Fjall (`support.rs`).
**When to use:** CLI status, dashboard, and support evidence while a node may hold the database.
**Plan:** Extend `GetBlockchainInfoResponse` with the quartet and read it in the live collector. Add the lock list and the support counters to an RPC the live collector already trusts, or to `getblockchaininfo` only for the quartet and to `listprunelocks` plus a small counter read for the rest. Do not open Fjall from the CLI while `open-bitcoind` has the store. Stopped status reads the mode from the already-loaded JSONC resolution. Stopped `pruneheight`, lock list, and counters stay unavailable with an explicit reason. Do not fill them with `0` or the last-delete height, because that lies when the store was not read.

### Pattern 5: Durable locks before resume

`FlushLifecycle::initialize` calls `resume_prune_intent(store, &[])` with the comment "Locks are not durable until Phase 150" (`flush_lifecycle.rs` lines 169–170). After locks persist, load them in `initialize` and pass that slice. An empty slice lets a resumed intent delete a now-locked height. The in-process flush path already re-checks the caller-supplied slice. [VERIFIED: STATE.md Phase 148 decision, flush_lifecycle.rs, prune.rs `finish_intent`]

Store one block-index record, for example key `prune_locks`, whose value is the name-to-range map. Keep names inside the value. Do not use the lock name as a filesystem path. Replace-by-name is a map insert. Clear-by-name is a map remove. Sync the batch before RPC success.

### Pattern 6: Counters advance only after a live delete

Discretion allows the counters beside `have_pruned`. Put one summary record in the same block-index namespace:

- `successful_batch_count: u64`
- `pruned_height_count: u64`
- `maybe_last_prune_height: Option<u32>`

Increment the height count by the number of `DeletedLiveMate` outcomes in the plan. Increment the batch count by one only when that number is at least one. Set `maybe_last_prune_height` to the maximum of those deleted heights (this batch only, not the historical max). Write the summary in the same sync batch as the deletes when the store API allows it; otherwise sync it immediately after the plan commits and before RPC returns. `AlreadyAbsent`, skipped locks, skipped keep-window heights, and empty plans do not move the summary. `finish_intent` on restart counts as a successful batch of one only when that finish's `commit_paired_delete` returns `DeletedLiveMate`.

Support evidence reads this summary. If the summary was not loaded (probe-only bundle), the prune section is unavailable with a reason. Loaded-and-never-deleted is count `0` and an absent last height (D-16).

### Pattern 7: Dashboard and support stay read-only

Append a `Prune` dashboard section. Do not insert it before `sections[2]` (`Mempool and Wallet`). `dashboard_model_chainstate_durability_rows_preserve_unavailable_reason_without_sensitive_text` looks at `sections[2]` and forbids the substring `pruned` inside the six durability rows. A separate section is safe. Show the four status facts and the lock list (`name`, `height_first`, `height_last`). Add no `ActionEntry` that deletes payloads. The existing service start/stop actions stay. Show the last `pruneblockchain` outcome as text when the status snapshot has it.

Support markdown gets a `## Prune` section sibling to chainstate durability. `support_bundle_chainstate_durability_forbids_prune_and_getblock_copy` scans only the durability section. Do not put the new facts inside `chainstate_durability`.

### RPC registration shape

Follow `SupportedMethod` plus `method/node.rs` plus `dispatch.rs`. `pruneblockchain`, `listprunelocks`, and `setprunelock` are `MethodOrigin::BaselineParity` even though the lock JSON is the smaller D-12 shape. `clearprunelock` is an Open Bitcoin extension of Knots' delete-by-null-height. Register all four in `SupportedMethod::all` or the black-box unknown-method test stays the only guard and operators cannot call them.

`GetBlockchainInfoResponse` gains `pruned: bool` always, plus the three optional fields with `skip_serializing_if`. Default disabled mode serializes `"pruned": false` and nothing else prune-related.

### Anti-Patterns to Avoid

- **Using `pruned_count` as `pruned` or as `pruneheight`.** Mode can be on with count 0. Count can be positive from an earlier delete while a read path is about configuration. D-02 and D-05.
- **Returning the info-field plus-one from `pruneblockchain`.** Knots returns the unadjusted helper.
- **Clamping a keep-window target "to match Knots RPC."** That reopens PRUN-03.
- **Calling `commit_paired_delete` from the RPC crate.** Bypass of the flush owner.
- **Leaving `initialize` on `&[]` after locks exist.**
- **Opening the datadir from the CLI to paint the dashboard.** Two Fjall openers.
- **Copying Knots `temporary: true`.** Those locks vanish on restart and `WritePruneLock` skips them.
- **Printing lock names or datadir in the support prune section.**

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| `0` / `1` / `>= 550` parsing | A second parser or byte field | `parse_prune_arg` | D-07. Negatives and `2..=549` already refuse. |
| Manual candidate heights | A byte-budget walk or a clamp | `plan_manual_prune` | D-10 and D-11. |
| 288-block window | A new constant | `MIN_BLOCKS_TO_KEEP` in `prune/range.rs` | Knots `validation.h`. |
| 10-block buffer | A new formula | `height_forbidden_by_lock` | D-14. |
| Paired delete and have-pruned | A direct key erase in RPC | `flush_applying_prune_plan` | Phase 148 durability and cache eviction. |
| Lock storage | A new database, a path, or a file | One Fjall block-index record | Existing engine. Names are data, not paths. |
| Pruneheight | `min(stored heights)` or last-delete height | `GetPruneHeight` plus one, or 0 | Quoted helper. |
| Support redaction | A new redaction dialect | `support_status_for_bundle` plus a prune-section scrub | Paths, hashes, and lock names stay out. |

**Key insight:** The policy and the delete path exist. This phase is projection, one RPC control path, and durability of locks and counters that the earlier phases deliberately left empty.

## Common Pitfalls

### Pitfall 1: Three prune heights

**What goes wrong:** RPC info, RPC prune return, and support evidence show the same number.
**Why it happens:** D-04, D-10, and D-16 all say "prune height."
**How to avoid:** Project them in three fields with the quoted formulas. Test a hole (complete tip suffix above a gap) and a first successful batch separately.
**Warning signs:** A test expects `pruneblockchain` to equal `getblockchaininfo.pruneheight`, or support last height to equal either one before a delete.

### Pitfall 2: Exact `getblockchaininfo` key snapshot

**What goes wrong:** `phase127_composition.rs` asserts sorted result keys and does not include `pruned`.
**Why it happens:** The response object today is chain, blocks, headers, bestblockhash, mediantime, verificationprogress, initialblockdownload, warnings.
**How to avoid:** Default mode adds only `pruned: false`. Update that sorted-key assertion. Do not add `pruneheight` on the disabled default. `ResultHasKeys` in `fixtures.rs` is a subset and can keep passing.
**Warning signs:** Phase 127 black-box fails on key equality after the response struct grows.

### Pitfall 3: Substring `pruned` in older sections

**What goes wrong:** Durability dashboard rows or durability support JSON start containing `pruned`.
**Why it happens:** Someone nests the quartet under `chainstate_durability` or `pruned_count`.
**How to avoid:** New snapshot field and new support/dashboard sections. Update the durability tests only if a new field is forced into those six rows.
**Warning signs:** `forbids_prune_and_getblock_copy` or the dashboard durability test fails without a durability change.

### Pitfall 4: Mode never reaches the running node

**What goes wrong:** JSONC `prune` parses, but `getblockchaininfo.pruned` stays false and `pruneblockchain` returns the disabled refusal.
**Why it happens:** Startup does not call `set_prune_mode`.
**How to avoid:** One `RuntimeConfig` field set from `parse_prune_arg`, applied when the network is built. CLI status reads that same resolution when RPC is down, and the live node when RPC is up.
**Warning signs:** A test sets the config file, starts the context, and still sees `PruneMode::Disabled`.

### Pitfall 5: Resume deletes through a lock

**What goes wrong:** A lock saved by `setprunelock` does not protect an in-flight `prune_intent` after restart.
**Why it happens:** `initialize` passes `&[]`.
**How to avoid:** Load locks before `resume_prune_intent`. Test a lock covering the intent height and expect fail-closed, which `finish_intent` already returns when `height_forbidden_by_any_lock` is true.
**Warning signs:** Restart tests still construct `resume_prune_intent(store, &[])` as the production path.

### Pitfall 6: Counter lies on empty or probe-only reads

**What goes wrong:** Support evidence shows zeros when the store was not read, or increments on a refused prune.
**Why it happens:** Default `0` is also the legal "no delete yet" value.
**How to avoid:** Unavailable versus loaded-zero are different. Increment only on `DeletedLiveMate`.
**Warning signs:** Disabled `pruneblockchain` changes the counter, or a probe-only bundle prints `successful_batch_count: 0` without opening the node.

### Pitfall 7: Snapshot struct literals

**What goes wrong:** Adding `prune` to `OpenBitcoinStatusSnapshot` breaks about 48 struct literals across node and CLI tests.
**Why it happens:** The struct does not use functional update.
**How to avoid:** Give the new field a small default helper and update every literal in one task. `OpenBitcoinConfig` itself is cheap (only the `Default` impl). `RuntimeConfig` callers that use `..RuntimeConfig::default()` stay source-compatible if the new field is in `Default`.
**Warning signs:** A compile error storm after the snapshot field lands.

### Pitfall 8: File-length checker at 629

**What goes wrong:** A small edit to `runtime_authority.rs` (626) or `status.rs` (622) fails `file-lengths`.
**Why it happens:** Those files are already at the ceiling.
**How to avoid:** New code goes in sibling modules. Move lines out before adding any.
**Warning signs:** `bright-builds-check` file-lengths fails inside `verify.sh`.

## Code Examples

### Disabled, manual-only, and automatic JSON

```json
{ "pruned": false }
```

```json
{ "pruned": true, "pruneheight": 0, "automatic_pruning": false }
```

```json
{ "pruned": true, "pruneheight": 12, "automatic_pruning": true, "prune_target_size": 576716800 }
```

`576716800` is `550 * 1024 * 1024`. `pruneheight: 12` is last pruned height 11 plus one. Source of the presence rules: `blockchain.cpp` lines 1802–1811. [VERIFIED]

### `pruneblockchain` numeric result

After a delete whose `GetPruneHeight` helper returns 11, the RPC result is `11`. The info field becomes `12`. If the helper is nullopt, the RPC result is `-1` and the info field is `0` while mode stays on. Source: `blockchain.cpp` lines 1270–1271 and 1804–1805. [VERIFIED]

### Suggested CLI shape

RPC names stay `pruneblockchain`, `listprunelocks`, `setprunelock`, and `clearprunelock`. CLI spelling is discretionary; this spelling calls those methods and does not grow a second planner:

```text
open-bitcoin prune run <height-or-timestamp>
open-bitcoin prune lock list
open-bitcoin prune lock set --name <name> --height-first <n> --height-last <n>
open-bitcoin prune lock clear --name <name>
```

Status and dashboard keep showing the quartet. They do not grow a destructive `prune run` keybinding.

### Parity breadcrumbs

New chainstate/node/rpc files that quote the helpers above should list:

- `packages/bitcoin-knots/src/rpc/blockchain.cpp`
- `packages/bitcoin-knots/src/node/blockstorage.h`
- `packages/bitcoin-knots/src/node/blockstorage.cpp`
- `packages/bitcoin-knots/src/node/blockmanager_args.cpp`
- `packages/bitcoin-knots/src/validation.h`
- `packages/bitcoin-knots/src/validation.cpp`

CLI dashboard and support files that only render an Open Bitcoin snapshot keep the existing `none` reason. Register every new file in `docs/parity/source-breadcrumbs.json`, then `bun run scripts/check-parity-breadcrumbs.ts --write` if the checker owns the comment block.

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Knots `blk`/`rev` file prune | Fjall paired key delete | v2.4 Phase 148 | `GetPruneHeight` walks payload+undo presence, not file numbers |
| Knots RPC clamps near-tip manual prune | Open Bitcoin refuses | Phase 147 PRUN-03 | Intentional. Do not "fix" it in this phase |
| Knots lock RPC has desc, temporary, `"*"` | D-12 name + inclusive heights, durable, clear-by-name | Phase 150 context | Smaller JSON. Document the difference in Phase 151, not here |
| `initialize` passes no locks | Load the durable map | This phase | Required once operators can set locks |
| `flush_applying_prune_plan` is test-only | RPC manual prune calls it | This phase | Removes the production `dead_code` allowance |

**Deprecated/outdated:**

- Treating D-04's "lowest stored complete block" as `min(height)` without the tip-suffix walk. Use the quoted helper.
- Phase 147 comments that say lock persistence and list/set are out of scope. They are this phase's job. Do not leave that comment on the public struct once persistence exists.

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | `clearprunelock` is a separate method rather than Knots' null-height `setprunelock` | Architecture, locks | Discretion says list, set, and clear. If the planner prefers null-height delete on `setprunelock`, keep the success-false-if-missing rule and still sync. |
| A2 | Top-level JSONC `prune` is the right placement | Pattern 3 | Discretion allows another object key. Behavior must still be `parse_prune_arg` and default 0. |
| A3 | Unix timestamps above `1_000_000_000` are in scope for `pruneblockchain` | Quoted Knots `pruneblockchain` | D-09 says `<height>`. The quoted function also accepts a timestamp. Omitting it is an operator-visible difference Phase 151 would have to record. Implementing it as a pure pre-step matches the quoted RPC. |
| A4 | Probe-only support should mark prune evidence unavailable instead of emitting loaded-zero | Pattern 6 | D-16's zero is for "no successful delete," which requires a read. Emitting zero without a read under-reports. |

## Open Questions (RESOLVED)

1. **Where the live collector reads lock rows and counters** — RESOLVED
   - What we know: Live CLI status does not open Fjall. `getblockchaininfo` is the live blockchain read. Locks have their own Knots RPC.
   - What's unclear: Whether counters ride on `getblockchaininfo` (they must not: that object is the quartet only) or on a dedicated read the collector already performs.
   - Recommendation: Keep counters off `getblockchaininfo`. Have the live collector call `listprunelocks` for the dashboard list and read the summary from the node through whatever status RPC already returns `OpenBitcoinStatusSnapshot` fields. Stopped collector uses JSONC for mode only.
   - Resolution: Live locks and counters ride on the status snapshot, not on `getblockchaininfo` and not on a CLI Fjall open. Plan 04 fills `OpenBitcoinNetworkStatusResponse.prune` from `load_prune_locks` and `load_prune_support_summary`, and the live collector copies that object onto the snapshot. The stopped collector reads JSONC mode only and marks height, locks, manual prune, and support counts Unavailable. `getblockchaininfo` stays the quartet.

2. **Summary-record crash window** — RESOLVED
   - What we know: Deletes commit per height inside `commit_paired_delete`. A plan is many heights.
   - What's unclear: Whether one Fjall batch can carry the last height's delete and the summary.
   - Recommendation: Prefer one sync batch. If the API cannot do that, sync the summary before RPC returns and test a crash between delete and summary as under-count, not as a second delete.
   - Resolution: The summary is a separate write after `DeletedLiveMate` heights, before flush returns. A crash in between under-counts and must not delete again. Plan 03 calls `record_successful_prune_batch` once after those per-height deletes. A later finish of the same heights is `AlreadyAbsent` and does not increment the batch or height counters again.

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| rustc / cargo | First-party build and tests | ✓ | 1.94.1 | — |
| bun | Breadcrumb checker and `verify.sh` script host | ✓ | 1.4.2 | — |
| Pinned Knots tree | Quotes and breadcrumbs | ✓ | submodule path readable | `git submodule update --init --recursive` if a file is missing |
| Public network | None | n/a | — | Must stay unused |

**Missing dependencies with no fallback:** none.

**Missing dependencies with fallback:** none.

External service probes are unnecessary. This phase uses the existing toolchain and the in-repo Knots tree.

## Security Domain

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V2 Authentication | yes | New methods sit on the existing RPC server. Do not add an unauthenticated listener or a dashboard action that deletes without going through that RPC. |
| V3 Session Management | no | No new session type. |
| V4 Access Control | yes | Mutating commands are RPC and CLI-over-RPC. Dashboard list and status are read-only. Disabled mode and keep-window checks run before any delete. |
| V5 Input Validation | yes | `parse_prune_arg` for config. `i64` height, negative refusal, timestamp pre-step, `plan_manual_prune` for the rest. Lock name non-empty; `height_last >= height_first`; refuse buffer overflow. |
| V6 Cryptography | no | No new crypto. Do not hand-roll hashing of lock names. |

### Known Threat Patterns for operator prune surfaces

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Manual prune while disabled, or inside the keep window | Tampering | Typed refusal before `flush_applying_prune_plan`. Tests assert `have_pruned` stays false. |
| Restarted intent ignores a saved lock | Tampering | Load locks in `initialize` before `resume_prune_intent`. |
| Support bundle leaks datadir, Fjall key names, block hashes, or lock names | Information disclosure | Prune section is counts and last height only. Redaction pass strips the rest. Dashboard may show lock names; support evidence may not. |
| Lock name treated as a path | Tampering / information disclosure | Store the name inside one record. Never join it onto the datadir. |
| Config `2..=549` silently becomes disabled | Tampering | Surface `PruneModeParseError` at JSONC load. |
| `pruned: true` reported because a payload is missing | Spoofing | `pruned` is mode-only. Missing payload without have-pruned stays `Unavailable` (LABL-01). |

## Sources

### Primary (HIGH confidence)

- `packages/bitcoin-knots/src/rpc/blockchain.cpp` — `GetPruneHeight`, `getblockchaininfo` keys, `pruneblockchain`, `listprunelocks`, `setprunelock`
- `packages/bitcoin-knots/src/node/blockmanager_args.cpp` — `ParsePruneOption`
- `packages/bitcoin-knots/src/node/blockstorage.h` — `PruneLockInfo`, `PRUNE_TARGET_MANUAL`, `IsPruneMode`, `GetPruneTarget`
- `packages/bitcoin-knots/src/node/blockstorage.cpp` — `WritePruneLock`, `UpdatePruneLock`, `DeletePruneLock`, `PRUNE_LOCK_BUFFER`, `m_prune_mode{opts.prune_target > 0}`
- `packages/bitcoin-knots/src/validation.h` — `MIN_BLOCKS_TO_KEEP` 288, `MIN_DISK_SPACE_FOR_BLOCK_FILES`
- `packages/bitcoin-knots/src/validation.cpp` — `PruneBlockFilesManual`, `GetPruneRange`
- `packages/bitcoin-knots/src/chain.h` — `TIMESTAMP_WINDOW` = `MAX_FUTURE_BLOCK_TIME` = 7200
- `packages/bitcoin-knots/src/rpc/protocol.h` — `RPC_MISC_ERROR` -1, `RPC_INVALID_PARAMETER` -8
- Open Bitcoin prune, flush, status, RPC, CLI, and config files cited above

### Secondary (MEDIUM confidence)

- `.planning/STATE.md` Phase 147–149 decisions confirming the no-clamp rule and the empty startup lock slice
- `.planning/phases/150-operator-prune-surfaces-and-evidence/150-CONTEXT.md` locked decisions

### Tertiary (LOW confidence)

- None. The timestamp-in-scope recommendation is a reading of the quoted RPC, recorded as assumption A3 because D-09's wording says height.

## Metadata

**Confidence breakdown:**

- Standard stack: HIGH — no new libraries; existing functions were read.
- Architecture: HIGH — Knots formulas and the Open Bitcoin seams were read in this session. A3 and A4 are explicit discretion calls.
- Pitfalls: HIGH — the exact-key test, file lengths, empty lock slice, and dead_code flush were read off the current tree.

**Research date:** 2026-09-28
**Valid until:** 2026-10-28 (stable pinned Knots tree; re-check if the submodule pointer moves)

## Planning Notes

- Nyquist validation is off in `.planning/config.json` (`workflow.nyquist_validation: false`). No separate test-framework section.
- Success checks the planner can turn into tasks:
  1. Disabled JSONC and default runtime: `getblockchaininfo` is `{ ..., "pruned": false }` with no prune-only keys. `pruned_count` stays 0.
  2. Manual-only, nothing deleted: `pruned: true`, `pruneheight: 0`, `automatic_pruning: false`, no `prune_target_size`.
  3. Automatic 550: `prune_target_size` is `576716800`.
  4. A hole under a complete tip: info `pruneheight` is one past the last incomplete height in the tip suffix, not the lowest stored height.
  5. `pruneblockchain` in disabled mode does not delete and does not set have-pruned.
  6. A target inside the keep window is a typed refusal, not a clamped delete.
  7. A legal target returns `GetPruneHeight` (not plus one) after the durable flush.
  8. Automatic mode accepts that same manual RPC.
  9. Set, list, replace, and clear survive a reopen, and `initialize` feeds the loaded locks into resume.
  10. Support JSON and Markdown show batch count, height count, and last batch height, and contain no datadir, Fjall key, block hash, or lock name.
  11. Dashboard shows the quartet and the lock list and has no payload-deleting action.
  12. `bash scripts/verify.sh` is the completion command.
