---
phase: 148-fjall-payload-unlink-and-have-pruned
verified: 2026-09-27T09:36:53Z
status: gaps_found
score: 8/9 must-haves verified
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: 148-2026-09-27T02-46-06
generated_at: 2026-09-27T09:36:53Z
lifecycle_validated: true
overrides_applied: 0
gaps:
  - truth: "After a flush that deleted a hash, that hash is gone from blocks_by_hash before the mutate closure returns."
    status: failed
    reason: "A durable paired delete can commit and then the flush returns an error. Eviction runs only on Ok, so the body stays in blocks_by_hash. A later retry sees both mates already absent and does not report the hash as deleted, so the cache entry is never removed. Inventory treats cache presence as payload presence."
    artifacts:
      - path: "packages/open-bitcoin-node/src/network/runtime_authority/prune_flush.rs"
        issue: "flush_and_evict_pruned_blocks removes blocks_by_hash entries only after flush_applying_plan returns Ok."
      - path: "packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs"
        issue: "complete_coins_write drops deleted_block_hashes when returning RefuseDiskSpace, and later coins or chain-meta errors do the same after apply_prune_plan has committed."
      - path: "packages/open-bitcoin-node/src/chainstate/flush_lifecycle/prune_apply.rs"
        issue: "AlreadyAbsent is not added to deleted_block_hashes and does not call on_deleted, so a retry cannot evict a hash the failed flush already unlinked."
    missing:
      - "Surface hashes whose paired delete already committed when the flush later returns an error."
      - "Remove those hashes from blocks_by_hash before the mutate closure returns the error."
      - "Cover the error-after-unlink path with a test that the cache entry is gone and that a following AlreadyAbsent retry does not put it back."
---

# Phase 148: Fjall Payload Unlink and Have-Pruned Verification Report

**Phase Goal:** Eligible heights lose paired block and undo payloads on disk, and have-pruned is recorded only after that delete is durable.
**Verified:** 2026-09-27T09:36:53Z
**Status:** gaps_found
**Re-verification:** No — initial verification

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
| --- | ------- | ---------- | -------------- |
| 1 | Pruning a height removes that height's Fjall block payload and undo together; no `blk`/`rev` flat-file store is introduced. | ✓ VERIFIED | `commit_paired_delete` removes `block_key` and `undo_key` in one `SyncAll` batch. `prune.rs` does not call `save_block` or `save_undo` and does not introduce flat files. |
| 2 | The node records have-pruned only after a non-empty durable delete batch succeeds, never from prune config alone. | ✓ VERIFIED | `HAVE_PRUNED_KEY` is inserted only in the tombstone batch, and only after a live-mate pre-check. Empty plans and `sync_prune_intent` do not insert it. Both mates already absent returns `AlreadyAbsent` with no write. |
| 3 | Restart after an interrupted prune finishes the partial delete or refuses closed without inventing blocks or triggering reindex. | ✓ VERIFIED | `initialize` calls `resume_prune_intent` before `ReadyToFlush`. Finish reuses `commit_paired_delete`. Refusal is `StorageError::Corruption` with `StorageRecoveryAction::Repair` and does not write payload bytes. |
| 4 | Heights covered by the Phase 147 lock buffer remain present after a prune attempt that would otherwise delete them. | ✓ VERIFIED | `classify_prune_height` calls `height_forbidden_by_any_lock` (`PRUNE_LOCK_BUFFER` is 10) before the keep window and skips the height without failing the rest of the plan. |
| 5 | One mate already absent is tombstoned without writing an empty block or undo. Both mates already absent does not write have-pruned. | ✓ VERIFIED | Live-mate path always tombstones both keys. Early return when both probes are false writes nothing. Tests cover block-only, undo-only, and both-absent. |
| 6 | The delete batch does not insert or remove the coins best-block key. | ✓ VERIFIED | The Sync batch touches the block key, undo key, `have_pruned`, and `prune_intent` only. `encode_best_block_key` is absent from `prune.rs`. |
| 7 | A non-empty plan persists the ordered prefix, deletes eligible pairs, and only then flushes coins. An empty plan does not write have-pruned. | ✓ VERIFIED | `execute_flush_applying_plan` calls `persist_ordered_prefix`, then `apply_prune_plan`, then `complete_coins_write`. An empty plan skips `apply_prune_plan`. |
| 8 | After a deleted hash, `Chainstate::forget_undo` drops that undo, including when a later height errors, so the next flush cannot write the key back. | ✓ VERIFIED | `flush_applying_plan` records `on_deleted` hashes and calls `forget_undo` before returning the flush `Result`. |
| 9 | After a flush that deleted a hash, that hash is gone from `blocks_by_hash` before the mutate closure returns. | ✗ FAILED | Success path evicts. If the unlink commits and the flush then errors, `deleted_block_hashes` is discarded and the cache entry stays. A retry will not evict it. |

**Score:** 8/9 truths verified

The review warning in `148-REVIEW.md` (WR-01) is a goal gap. Decision D-14 and plan 04 require a durable delete to leave the block cache before a later presence probe. The success-path test `flush_applying_prune_plan_drops_only_the_deleted_hash` does not cover the error path. Disk pairing, have-pruned timing, and restart finish-or-refuse still hold, so UNLK-01, UNLK-02, and UNLK-03 stay satisfied. The failed truth is the in-memory eviction contract.

Production `flush_coins` still passes an empty plan. That is the planned boundary: this phase does not call `plan_automatic_prune` from the flush loop. Phase 150 owns operator invocation. It is not a second gap.

### Required Artifacts

| Artifact | Expected | Status | Details |
| -------- | ----------- | ------ | ------- |
| `packages/open-bitcoin-node/src/storage/fjall_store/prune.rs` | Paired delete, `prune_intent`, and have-pruned marker | ✓ VERIFIED | `commit_paired_delete` and `resume_prune_intent` are substantive and called from the flush owner and startup. |
| `packages/open-bitcoin-node/src/storage/fjall_store/coins.rs` | `has_undo` `contains_key` probe and `pub(super) undo_key` | ✓ VERIFIED | Both exist. `prune.rs` uses `undo_key`. |
| `docs/parity/source-breadcrumbs.json` | `node-fjall-prune-unlink` breadcrumb group | ✓ VERIFIED | Group cites `validation.cpp`, `blockstorage.cpp`, `blockstorage.h`, and `validation.h`. `prune_flush.rs` is in `node-network-runtime-authority`. |
| `packages/open-bitcoin-node/src/chainstate/flush_lifecycle/prune_apply.rs` | Height classification and per-height unlink loop | ✓ VERIFIED | `classify_prune_height` and `apply_prune_plan` are called from `execute_flush_applying_plan`. |
| `packages/open-bitcoin-chainstate/src/engine.rs` | `forget_undo` map removal | ✓ VERIFIED | Removes only the named hash from `undo_by_block`. |
| `packages/open-bitcoin-node/src/chainstate.rs` | `flush_applying_plan` on the existing flush owner | ✓ VERIFIED | Calls `execute_flush_applying_plan` and `forget_undo`. |
| `packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs` | `initialize` calls resume before `ReadyToFlush` | ✓ VERIFIED | `resume_prune_intent(store, &[])?` precedes the readiness assignment. |
| `packages/open-bitcoin-node/src/network/runtime_authority/prune_flush.rs` | Flush plus `blocks_by_hash` eviction | ⚠️ PARTIAL | Eviction is wired on `Ok` only. The error path leaves the deleted body cached. |
| `packages/open-bitcoin-node/src/network/runtime_authority.rs` | `flush_coins` delegates to the evicting path with an empty plan | ✓ VERIFIED | Delegates to `flush_and_evict_pruned_blocks` with `PrunePlan::default()`. |

### Key Link Verification

| From | To | Via | Status | Details |
| ---- | --- | --- | ------ | ------- |
| `fjall_store/prune.rs` | Fjall `Database::batch` | One Sync batch removes both keys and inserts `have_pruned` | ✓ WIRED | `db.batch().durability(Some(SyncAll))` then `commit`. |
| `fjall_store.rs` | `fjall_store/prune.rs` | `mod prune` | ✓ WIRED | Module is declared and re-exported. |
| `prune.rs` | `coins.rs` | `undo_key` | ✓ WIRED | `use super::coins::undo_key`. |
| `flush_lifecycle.rs` | `prune_apply.rs` | `apply_prune_plan` after `persist_ordered_prefix` | ✓ WIRED | Call is in `execute_flush_applying_plan`. |
| `chainstate/fjall_store.rs` | `commit_paired_delete` | `FlushPersistSink::commit_paired_unlink` | ✓ WIRED | Fjall store override delegates. The trait default returns `AlreadyAbsent` and is not the production path. |
| `chainstate.rs` | `Chainstate::forget_undo` | Callback after each `DeletedLiveMate`, including a later error | ✓ WIRED | `forget_undo` runs before `result` is returned. |
| `flush_lifecycle.rs` | `resume_prune_intent` | `initialize`, before `ReadyToFlush`, empty lock slice | ✓ WIRED | Error prevents `ReadyToFlush`. |
| `prune.rs` resume | `commit_paired_delete` | Finish reuses the paired delete; refuse returns Repair corruption | ✓ WIRED | `fail_closed` uses `corruption`, which sets `StorageRecoveryAction::Repair`. |
| `runtime_authority.rs` | `prune_flush.rs` | `flush_coins` calls `flush_and_evict_pruned_blocks` | ✓ WIRED | Empty plan on the production flush. |
| `prune_flush.rs` | `flush_applying_plan` | Mutate closure, then `blocks_by_hash.remove` for each deleted hash | ✗ PARTIAL | Remove runs only when the flush returns `Ok`. |

### Data-Flow Trace (Level 4)

| Artifact | Data Variable | Source | Produces Real Data | Status |
| -------- | ------------- | ------ | ------------------ | ------ |
| `prune_flush.rs` | `execution.deleted_block_hashes` | `apply_prune_plan` pushes hashes only for `DeletedLiveMate` after `commit_paired_delete` | Yes on the success path | ✓ FLOWING |
| `prune_flush.rs` error return | discarded hash list | `complete_coins_write` returns `Err` without `FlushExecution` | Deleted hashes exist in the callback vec and in the discarded argument, but the evictor never sees them | ⚠️ HOLLOW |
| `inventory.rs` | `payload_present` | `blocks_by_hash.contains_key` OR durable `has_block` | A cached body with a deleted key still reports available | ⚠️ HOLLOW on the error path |

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
| -------- | ------- | ------ | ------ |
| Paired delete and resume seams | Existing unit tests in `prune_unlink.rs`, `execute_flush.rs`, and `initialize.rs` | Not executed here. Compile-and-run exceeds the spot-check budget. Code and assertions were read. | ? SKIP |
| Cache eviction after an error-after-unlink flush | No test invokes a non-empty plan and then forces `RefuseDiskSpace` or a coins error | Code inspection shows the cache entry remains | ✗ FAIL |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
| ----------- | ---------- | ----------- | ------ | -------- |
| UNLK-01 | 148-01, 148-02, 148-04 | A prune of a height removes that height's block payload and undo together. | ✓ SATISFIED | One Sync batch tombstones both keys. Lock and keep-window skips leave both keys. The cache gap does not leave one mate on disk. |
| UNLK-02 | 148-01, 148-02 | The node records have-pruned only after that delete is durable. | ✓ SATISFIED | Marker insert is in the committed tombstone batch. Config, empty plans, and intent-only commits leave it unset. |
| UNLK-03 | 148-03 | Restart after an interrupted prune finishes or refuses the partial delete without inventing blocks or reindexing. | ✓ SATISFIED | Finish and refuse tests cover one mate, both absent, hash mismatch, keep window, lock, and a coins best-block that is not the tip. Refusal is Repair, not Reindex. |

REQUIREMENTS.md maps UNLK-01, UNLK-02, and UNLK-03 to Phase 148. Every ID declared in plan frontmatter is one of those three. No phase-148 requirement is unclaimed.

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
| ---- | ---- | ------- | -------- | ------ |
| `packages/open-bitcoin-node/src/network/runtime_authority/prune_flush.rs` | 27-34 | Evicts only after `Ok` | 🛑 Blocker | Same as the failed truth. WR-01 stands. |
| `packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs` | 92-99 | Default `commit_paired_unlink` returns `AlreadyAbsent` | ℹ️ Info | Production `FjallNodeStore` overrides it. A future sink that forgets the override would skip deletes without error. |
| `packages/open-bitcoin-node/src/network/runtime_authority/prune_flush.rs` | 41 | `allow(dead_code)` on the non-empty-plan wrapper outside tests | ℹ️ Info | Planned. `flush_coins` passes an empty plan until a later caller supplies one. |

### Human Verification Required

None. The failed cache path is visible in the flush result handling and does not need a running node.

### Gaps Summary

Disk unlink, have-pruned durability, lock-buffer skips, and interrupted-prune resume match the phase goal and UNLK-01 through UNLK-03. The phase still misses plan 04 and decision D-14 on the error path.

`apply_prune_plan` commits a paired delete, then `complete_coins_write` can return `Err` (disk-space refusal, coins flush/sync failure, or chain-meta persist failure) without the deleted hash list. `flush_and_evict_pruned_blocks` therefore never removes that hash from `blocks_by_hash`. `forget_undo` does run on that path, so undo will not be rewritten, but the block body stays cached. Inventory sets `payload_present` from cache presence or the durable key, so the pruned body can still look present. A later flush of the same height gets `AlreadyAbsent` and does not evict the entry.

---

_Verified: 2026-09-27T09:36:53Z_
_Verifier: Claude (gsd-verifier)_
