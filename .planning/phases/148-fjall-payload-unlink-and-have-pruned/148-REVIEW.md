---
phase: 148-fjall-payload-unlink-and-have-pruned
reviewed: 2026-09-27T18:52:16Z
depth: standard
files_reviewed: 17
files_reviewed_list:
  - docs/metrics/lines-of-code.md
  - docs/parity/source-breadcrumbs.json
  - packages/open-bitcoin-chainstate/src/engine.rs
  - packages/open-bitcoin-node/src/chainstate.rs
  - packages/open-bitcoin-node/src/chainstate/fjall_store.rs
  - packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs
  - packages/open-bitcoin-node/src/chainstate/flush_lifecycle/prune_apply.rs
  - packages/open-bitcoin-node/src/chainstate/flush_lifecycle/tests/execute_flush.rs
  - packages/open-bitcoin-node/src/chainstate/flush_lifecycle/tests/initialize.rs
  - packages/open-bitcoin-node/src/network/runtime_authority.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/prune_flush.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/tests.rs
  - packages/open-bitcoin-node/src/storage/fjall_store.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/coins.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/prune.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/tests.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/tests/prune_unlink.rs
findings:
  critical: 0
  warning: 1
  info: 1
  total: 2
status: issues_found
---

# Phase 148: Code Review Report

**Reviewed:** 2026-09-27T18:52:16Z
**Depth:** standard
**Files Reviewed:** 17
**Status:** issues_found

## Summary

Reviewed the paired Sync delete, flush-owner height classification, startup finish-or-refuse, and block-cache eviction, including the error path landed in `cfc8d3a5`. A flush that has already committed a live-mate delete now returns those hashes on `FlushApplyError`, calls `forget_undo`, and removes them from `blocks_by_hash` before the mutate closure returns the error. `AlreadyAbsent` stays out of that list. The previous cache-eviction warning is closed.

One warning remains in restart recovery. `prune_intent` is synced against the in-memory chain, but `resume_prune_intent` judges that intent against `chain_meta` and the coins best-block from the previous coins flush. Those durable facts are written only after the intent, and ordinary `IfNeeded` persists often skip them. A crash in that window can refuse every later open.

## Warnings

### WR-01: Interrupted prune can refuse startup against a stale chain

**File:** `packages/open-bitcoin-node/src/storage/fjall_store/prune.rs:104` and `packages/open-bitcoin-node/src/storage/fjall_store/prune.rs:141-186`
**Issue:** `commit_paired_delete` Sync-inserts `prune_intent` before the tombstone batch. That insert happens inside `apply_prune_plan`, which runs before `complete_coins_write` persists coins and `chain_meta` (`flush_lifecycle.rs:319-325`, `flush_lifecycle.rs:471-475`). `connect_block` persists with `FlushMode::IfNeeded` (`chainstate.rs:465-469`), and `IfNeeded` skips the coins write unless the cache is critical or the process is under memory pressure, so `chain_meta` can lag the in-memory tip by many blocks. `load_chain_meta_for_open` treats a missing key as an empty chain (`coins.rs:387-391`).

Restart then calls `resume_prune_intent` before `ReadyToFlush`. It takes `active_chain.last()` as the tip and refuses with repair corruption when that tip's 288-block keep window contains the intent, when the hash is not on that older chain, or when coins best-block is not that tip. The refusal leaves the intent in place. The node never becomes ready, so it cannot persist the chain that made the height eligible. A height the flush already classified as ready can therefore fail closed on every later open. `initialize` is live; only a non-empty plan writes the intent, and production `flush_coins` still passes an empty plan.

**Fix:** Durably record the chain that classified the height before `sync_prune_intent`, using Sync durability, and resume against that record. Coins best-block is still updated later, so equality with the tip is the wrong crash-window check: allow a best-block that is absent or is the previous flush's tip on that same chain, and refuse when it is some other hash.

```rust
// Sync the classifying chain before the intent, not after the delete.
store.save_chain_meta(active_chain, PersistMode::Sync)?;
store.sync_prune_intent(PruneIntent { height, block_hash })?;

// Resume: keep-window and hash checks use that synced chain.
// Coins best-block may still be the previous tip in this crash window.
let best_is_tip = maybe_best_block == Some(tip.block_hash);
let best_is_ancestor = maybe_best_block
    .is_some_and(|hash| active_chain.iter().any(|position| position.block_hash == hash));
if maybe_best_block.is_some() && !best_is_tip && !best_is_ancestor {
    return Err(fail_closed("coins best-block is not on the active chain"));
}
```

## Info

### IN-01: Default unlink reports success and deletes nothing

**File:** `packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs:93-99`
**Issue:** `FlushPersistSink::commit_paired_unlink` defaults to `PairedDeleteOutcome::AlreadyAbsent`. A sink that does not override it, unlike `FjallNodeStore` and `FjallChainstateStore`, turns a non-empty plan into a successful no-op with no `have_pruned` write and no cache eviction.
**Fix:** Leave the method required, with no default, so a new sink cannot compile until it implements the paired delete.

---

_Reviewed: 2026-09-27T18:52:16Z_
_Reviewer: Claude (gsd-code-reviewer)_
_Depth: standard_
