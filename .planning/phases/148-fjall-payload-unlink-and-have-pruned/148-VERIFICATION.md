---
phase: 148-fjall-payload-unlink-and-have-pruned
verified: 2026-09-27T19:19:09Z
status: passed
score: 9/9 must-haves verified
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: 148-2026-09-27T02-46-06
generated_at: 2026-09-27T19:19:09Z
lifecycle_validated: true
overrides_applied: 0
re_verification:
  previous_status: gaps_found
  previous_score: 8/9
  gaps_closed:
    - "After a flush that deleted a hash, that hash is gone from blocks_by_hash before the mutate closure returns."
  gaps_remaining: []
  regressions: []
---

# Phase 148: Fjall Payload Unlink and Have-Pruned Verification Report

**Phase Goal:** Eligible heights lose paired block and undo payloads on disk, and have-pruned is recorded only after that delete is durable.
**Verified:** 2026-09-27T19:19:09Z
**Status:** passed
**Re-verification:** Yes — after gap closure

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
| --- | --- | --- | --- |
| 1 | Pruning a height removes that height's Fjall block payload and undo together; no `blk`/`rev` flat-file store is introduced. | ✓ VERIFIED | `commit_paired_delete` removes `block_key` and `undo_key` in one `SyncAll` batch. `prune.rs` has no `save_block`, `save_undo`, or flat-file writes. `paired_delete_removes_both_mates_and_records_have_pruned` passed. |
| 2 | The node records have-pruned only after a non-empty durable delete batch succeeds, never from prune config alone. | ✓ VERIFIED | `HAVE_PRUNED_KEY` is inserted only in the tombstone batch after a live-mate pre-check. `sync_prune_intent` does not insert it. An empty plan skips `apply_prune_plan`. Both mates already absent returns `AlreadyAbsent` with no write. |
| 3 | Restart after an interrupted prune finishes the partial delete or refuses closed without inventing blocks or triggering reindex. | ✓ VERIFIED | `initialize` calls `resume_prune_intent(store, &[])` before `ReadyToFlush`. Finish reuses `commit_paired_delete`. Refusal is `StorageError::Corruption` with `StorageRecoveryAction::Repair`. `fail_closed` writes no payload bytes. Resume tests for both mates, one mate, both absent, hash mismatch, keep window, lock, empty chain, and a coins best-block other than the tip passed. |
| 4 | Heights covered by the Phase 147 lock buffer remain present after a prune attempt that would otherwise delete them. | ✓ VERIFIED | `classify_prune_height` calls `height_forbidden_by_any_lock` before the keep window. That function expands the lock by `PRUNE_LOCK_BUFFER` (10). A skipped lock leaves both keys and still deletes other eligible heights. `lock_forbidden_height_keeps_both_mates` passed. |
| 5 | One mate already absent is tombstoned without writing an empty block or undo. Both mates already absent does not write have-pruned. | ✓ VERIFIED | A live-mate path tombstones both keys. Both probes false returns before any batch. Store tests cover block-only, undo-only, and both-absent. `initialize_intent_with_one_mate_removes_the_remaining_key` passed. |
| 6 | The delete batch does not insert or remove the coins best-block key. | ✓ VERIFIED | The Sync batch touches the block key, undo key, `have_pruned`, and `prune_intent` only. `encode_best_block_key` is absent from `prune.rs`. `paired_delete_leaves_coins_best_block_bytes_unchanged` passed. |
| 7 | A non-empty plan persists the ordered prefix, deletes eligible pairs, and only then flushes coins. An empty plan does not write have-pruned. | ✓ VERIFIED | `execute_flush_applying_plan` calls `persist_ordered_prefix`, then `apply_prune_plan`, then `complete_coins_write`. An empty plan skips `apply_prune_plan`. `applying_plan_ordering_writes_coins_after_unlink` and `empty_plan_on_none_mode_does_not_unlink` passed. |
| 8 | After a deleted hash, `Chainstate::forget_undo` drops that undo, including when a later height errors, so the next flush cannot write the key back. | ✓ VERIFIED | `flush_applying_plan` records `on_deleted` hashes and calls `forget_undo` before returning the flush `Result`. `forget_undo` removes only that hash from `undo_by_block`. `second_flush_does_not_restore_deleted_undo` passed. |
| 9 | After a flush that deleted a hash, that hash is gone from `blocks_by_hash` before the mutate closure returns, including when that flush returns an error. An `AlreadyAbsent` retry does not put the cache entry back. | ✓ VERIFIED | `FlushApplyError` carries the hashes the callback already recorded. `flush_and_evict_pruned_blocks` removes them on both `Ok` and `Err` before the closure returns the error. `AlreadyAbsent` is not added to that list. `error_after_unlink_drops_deleted_hash_and_retry_leaves_it_gone` passed. |

**Score:** 9/9 truths verified

The previous gap is closed in the current code. `apply_prune_plan` calls `on_deleted` only after `commit_paired_unlink` returns `DeletedLiveMate`. `flush_applying_plan` keeps that list even when `execute_flush_applying_plan` later returns `Err`. `flush_and_evict_pruned_blocks` drains it with `blocks_by_hash.remove` before `outcome.map_err`. The focused test plants a real Fjall payload, forces disk-space refusal after the unlink (`disk_free_bytes` 0, non-empty plan, one overlay coin), and asserts the hash is gone from the cache and both mates are gone from disk. The retry uses `u64::MAX`, returns an empty `deleted_block_hashes`, and leaves the cache entry absent. An unrelated cached hash stays.

Production `flush_coins` still passes `PrunePlan::default()`. Decision D-15 leaves operator invocation to Phase 150. An empty plan writes no intent and deletes nothing, which decision D-06 requires.

### Review warning WR-01

`148-REVIEW.md` warns that a crash after `sync_prune_intent` and before `persist_chain_meta` can make a later open refuse against a stale chain. That warning does not fail success criterion 3.

Criterion 3 and decision D-08 accept two outcomes: finish that height's paired delete, or refuse closed. The resume path does both. When the durable chain hash, keep window, lock set, or coins best-block disagrees, `fail_closed` returns corruption with `StorageRecoveryAction::Repair` and `initialize` leaves readiness short of `ReadyToFlush`. The refusal string states that resume stopped closed and did not reindex. `corruption()` in `fjall_store.rs` sets `Repair`. Resume does not call `save_block` or `save_undo`. The injected tests for hash mismatch, keep window, lock, empty chain, and a coins best-block other than the tip all passed with that repair refusal.

Finishing the delete when the durable chain still places the height inside the keep window, or when the hash is absent from that chain, would delete a height the durable record does not authorize. The plan 03 must-have already requires refusal in those cases. The ordinary flush still supplies an empty plan, so it does not write `prune_intent`.

### Required Artifacts

| Artifact | Expected | Status | Details |
| --- | --- | --- | --- |
| `packages/open-bitcoin-node/src/storage/fjall_store/prune.rs` | Paired delete, `prune_intent`, and have-pruned marker | ✓ VERIFIED | `commit_paired_delete` and `resume_prune_intent` are substantive and called from the flush owner and startup. |
| `packages/open-bitcoin-node/src/storage/fjall_store/coins.rs` | `has_undo` `contains_key` probe and `pub(super) undo_key` | ✓ VERIFIED | `has_undo` probes `contains_key`. `prune.rs` uses `undo_key`. |
| `docs/parity/source-breadcrumbs.json` | `node-fjall-prune-unlink` breadcrumb group | ✓ VERIFIED | Group cites `validation.cpp`, `blockstorage.cpp`, `blockstorage.h`, and `validation.h`. `prune_flush.rs` is in `node-network-runtime-authority`. |
| `packages/open-bitcoin-node/src/chainstate/flush_lifecycle/prune_apply.rs` | Height classification and per-height unlink loop | ✓ VERIFIED | `classify_prune_height` and `apply_prune_plan` are called from `execute_flush_applying_plan`. |
| `packages/open-bitcoin-chainstate/src/engine.rs` | `forget_undo` map removal | ✓ VERIFIED | Removes only the named hash from `undo_by_block`. |
| `packages/open-bitcoin-node/src/chainstate.rs` | Flush error that still carries hashes whose paired delete committed | ✓ VERIFIED | `FlushApplyError` wraps `StorageError` plus `deleted_block_hashes`. `flush_applying_plan` fills that list before mapping the error. |
| `packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs` | `initialize` calls resume before `ReadyToFlush` | ✓ VERIFIED | `resume_prune_intent(store, &[])?` precedes the readiness assignment. |
| `packages/open-bitcoin-node/src/network/runtime_authority/prune_flush.rs` | Cache removal on both `Ok` and `Err` before the closure returns | ✓ VERIFIED | Eviction runs on `failure.deleted_block_hashes` as well as the success list, then the error is returned. |
| `packages/open-bitcoin-node/src/network/runtime_authority.rs` | `flush_coins` delegates to the evicting path with an empty plan | ✓ VERIFIED | Delegates to `flush_and_evict_pruned_blocks` with `PrunePlan::default()`. |
| `packages/open-bitcoin-node/src/network/runtime_authority/tests.rs` | Error-after-unlink cache test and `AlreadyAbsent` retry | ✓ VERIFIED | `error_after_unlink_drops_deleted_hash_and_retry_leaves_it_gone` passed against a temp Fjall store. |

### Key Link Verification

| From | To | Via | Status | Details |
| --- | --- | --- | --- | --- |
| `fjall_store/prune.rs` | Fjall `Database::batch` | One Sync batch removes both keys and inserts `have_pruned` | ✓ WIRED | `db.batch().durability(Some(SyncAll))` then `commit`. |
| `fjall_store.rs` | `fjall_store/prune.rs` | `mod prune` | ✓ WIRED | Module is declared and `HAVE_PRUNED_KEY` is re-exported. |
| `prune.rs` | `coins.rs` | `undo_key` | ✓ WIRED | `use super::coins::undo_key`. |
| `flush_lifecycle.rs` | `prune_apply.rs` | `apply_prune_plan` after `persist_ordered_prefix` | ✓ WIRED | Call is in `execute_flush_applying_plan`. |
| `chainstate/fjall_store.rs` | `commit_paired_delete` | `FlushPersistSink::commit_paired_unlink` | ✓ WIRED | `FjallChainstateStore` delegates to the `FjallNodeStore` override. |
| `chainstate.rs` | `Chainstate::forget_undo` | Callback after each `DeletedLiveMate`, including a later error | ✓ WIRED | `forget_undo` runs before `result` is mapped to `FlushApplyError`. |
| `flush_lifecycle.rs` | `resume_prune_intent` | `initialize`, before `ReadyToFlush`, empty lock slice | ✓ WIRED | Error prevents `ReadyToFlush`. |
| `prune.rs` resume | `commit_paired_delete` | Finish reuses the paired delete; refuse returns Repair corruption | ✓ WIRED | `fail_closed` uses `corruption`, which sets `StorageRecoveryAction::Repair`. |
| `runtime_authority.rs` | `prune_flush.rs` | `flush_coins` calls `flush_and_evict_pruned_blocks` | ✓ WIRED | Empty plan on the production flush. |
| `chainstate.rs` | `prune_flush.rs` | `flush_applying_plan` returns deleted hashes on `Err`; eviction runs before the error leaves the closure | ✓ WIRED | `FlushApplyError.deleted_block_hashes` is removed inside `flush_and_evict_pruned_blocks` before `map_err`. |
| `prune_flush.rs` | `blocks_by_hash` | `remove` before the mutate closure returns | ✓ WIRED | `network.blocks_by_hash.remove(hash)` runs for the error list and the success list. |

### Data-Flow Trace (Level 4)

| Artifact | Data Variable | Source | Produces Real Data | Status |
| --- | --- | --- | --- | --- |
| `prune_flush.rs` | `deleted` from `Ok` or `FlushApplyError` | `on_deleted` runs only after `PairedDeleteOutcome::DeletedLiveMate` | Yes. The error-path test observes both mates absent and the cache entry gone. | ✓ FLOWING |
| `inventory.rs` | `payload_present` | `blocks_by_hash.contains_key` OR durable `has_block` | After a committed delete, the evictor removes the cache entry on success and on error, so cache presence no longer reports the deleted body. | ✓ FLOWING |
| `resume_prune_intent` | `prune_intent` bytes | `maybe_prune_intent` reads the block-index key written by `sync_prune_intent` | Yes. Finish and refuse tests plant the 36-byte intent and observe a completed delete or a repair refusal. | ✓ FLOWING |

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
| --- | --- | --- | --- |
| Error-after-unlink eviction and `AlreadyAbsent` retry | `cargo test -p open-bitcoin-node --lib error_after_unlink_drops_deleted_hash_and_retry_leaves_it_gone` | 1 passed, plus the success-path eviction and empty-plan cache tests in the same run | ✓ PASS |
| Paired delete, have-pruned timing, and resume finish-or-refuse | Same harness: `initialize_intent*`, `have_pruned_is_inserted_only_in_the_tombstone_batch`, `sync_prune_intent_does_not_insert_have_pruned`, `truncated_prune_intent_is_corruption_with_repair` | 10 passed in that run | ✓ PASS |
| Lock skip, keep window, coins best-block preservation, forget-undo | Follow-up `cargo test` across `open-bitcoin-node` and `open-bitcoin-chainstate` | 12 passed, including `lock_forbidden_height_keeps_both_mates`, `initialize_best_block_other_than_tip_refuses_repair`, and `forget_undo_removes_only_the_named_hash` | ✓ PASS |

Both commands ran through `bun run scripts/command-timings.ts run`, one after the other. 13 node tests passed in 3.96s, then 1 chainstate test and 11 node tests passed in 3.59s.

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
| --- | --- | --- | --- | --- |
| UNLK-01 | 148-01, 148-02, 148-04, 148-05 | A prune of a height removes that height's block payload and undo together. | ✓ SATISFIED | One Sync batch tombstones both keys. Lock and keep-window skips leave both keys. Error-path eviction drops the cache entry after that delete commits. |
| UNLK-02 | 148-01, 148-02 | The node records have-pruned only after that delete is durable. | ✓ SATISFIED | The marker insert is in the committed tombstone batch. Config, empty plans, and intent-only commits leave it unset. |
| UNLK-03 | 148-03 | Restart after an interrupted prune finishes or refuses the partial delete without inventing blocks or reindexing. | ✓ SATISFIED | Finish and refuse tests cover one mate, both absent, hash mismatch, keep window, lock, empty chain, and a coins best-block that is not the tip. Refusal is Repair. |

REQUIREMENTS.md maps UNLK-01, UNLK-02, and UNLK-03 to Phase 148. Every ID declared in plan frontmatter is one of those three. No phase-148 requirement is unclaimed. This report does not mark the requirement checkboxes complete.

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
| --- | --- | --- | --- | --- |
| `packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs` | 92-99 | Default `commit_paired_unlink` returns `AlreadyAbsent` | ℹ️ Info | `FjallNodeStore` and `FjallChainstateStore` override it. A future sink that forgets the override would skip deletes and still return success. |
| `packages/open-bitcoin-node/src/network/runtime_authority/prune_flush.rs` | 45 | `allow(dead_code)` on the non-empty-plan wrapper outside tests | ℹ️ Info | Planned. `flush_coins` passes an empty plan until a later caller supplies one. |

### Human Verification Required

None. Disk pairing, have-pruned timing, resume finish-or-refuse, lock skips, and error-path cache eviction are covered by the focused tests above.

### Gaps Summary

No remaining gaps. The earlier report failed because a committed delete followed by a flush error left the body in `blocks_by_hash`, and a later `AlreadyAbsent` retry did not evict it. The current flush attaches those hashes to `FlushApplyError`, forgets their undo, and removes them before the mutate closure returns the error. The retry test leaves the entry gone.

***

_Verified: 2026-09-27T19:19:09Z_
_Verifier: Claude (gsd-verifier)_
