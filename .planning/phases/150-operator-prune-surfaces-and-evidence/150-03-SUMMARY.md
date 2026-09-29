---
phase: 150-operator-prune-surfaces-and-evidence
plan: "03"
subsystem: chainstate
tags: [prune, fjall, locks, support-summary, resume, parity-breadcrumbs]

requires:
  - phase: 148-fjall-payload-unlink-and-have-pruned
    provides: prune_intent resume and paired payload delete
  - phase: 147-pure-prune-policy-and-lock-windows
    provides: PruneLockInfo and the 10-block lock buffer
provides:
  - Durable prune lock map under the fixed block-index key prune_locks
  - Support summary of batch count, height count, and this-batch max height
  - Startup resume that loads locks before it can delete
affects:
  - 150-04 status snapshot support counts
  - 150-05 prune lock RPC
  - 150-08 support-bundle prune section

tech-stack:
  added: []
  patterns:
    - "Lock names are length-prefixed bytes inside one block-index value, never a key and never a path"
    - "Support counters move only for DeletedLiveMate heights, and last prune height is that batch's maximum only"

key-files:
  created:
    - packages/open-bitcoin-node/src/storage/fjall_store/prune/records.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/tests/prune_records.rs
  modified:
    - packages/open-bitcoin-node/src/storage/fjall_store/prune.rs
    - packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs
    - packages/open-bitcoin-node/src/chainstate/flush_lifecycle/prune_apply.rs
    - packages/open-bitcoin-node/src/chainstate/flush_lifecycle/tests/initialize.rs
    - packages/open-bitcoin-node/src/chainstate/fjall_store.rs
    - packages/open-bitcoin-chainstate/src/prune/locks.rs
    - docs/parity/source-breadcrumbs.json

key-decisions:
  - "Tests and implementation ship in one hook-passing feat commit per task because pre-commit runs verify.sh"
  - "Support counters move only for DeletedLiveMate heights; a crash before the summary write under-counts and a later AlreadyAbsent finish does not increment"
  - "FjallChainstateStore delegates the summary write because the live flush sink is that wrapper"
  - "LOCK-02 and OPER-03 stay pending until lifecycle-valid Phase 150 verification"

patterns-established:
  - "sync_prune_locks replaces the whole map in one SyncAll batch under prune_locks"
  - "record_successful_prune_batch of an empty slice does not create prune_summary"

requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 150-2026-09-28T17-08-23
generated_at: 2026-09-29T09:26:18Z

duration: 1h 16m
completed: 2026-09-29
---

# Phase 150 Plan 03: Durable Locks, Counters, and Resume Summary

**Durable prune locks and live-delete support counts are loaded before startup resume can delete another height**

## Performance

- **Duration:** 1h 16m
- **Started:** 2026-09-29T08:09:57Z
- **Completed:** 2026-09-29T09:26:18Z
- **Tasks:** 2
- **Files modified:** 11

## Accomplishments

- A saved lock map reopens with the same names and inclusive heights. Replacing one name keeps a single row, and an empty sync stores an empty map.
- `record_successful_prune_batch(&[])` does not create the summary. `&[4, 9]` then `&[2]` yields batch count 2, height count 3, and last height `Some(2)`.
- `FlushLifecycle::initialize` loads locks and passes them to `resume_prune_intent`. A saved lock that covers the intent height returns `prune_intent height is forbidden by a prune lock` and leaves the payload in place.
- Batch and height counters move only for `DeletedLiveMate`. A later `AlreadyAbsent` finish of that height does not increment them again.

## Task Commits

Each task was committed atomically:

1. **Task 1: Persist the lock map and the support summary** - `2549f401` (feat)
2. **Task 2: Resume with loaded locks and count only live deletes** - `8b41d8bc` (feat)

## Files Created/Modified

- `packages/open-bitcoin-node/src/storage/fjall_store/prune/records.rs` - Little-endian lock map and support summary under fixed block-index keys
- `packages/open-bitcoin-node/src/storage/fjall_store/tests/prune_records.rs` - Reopen, replace, clear, empty-batch, and corruption tests
- `packages/open-bitcoin-node/src/storage/fjall_store/prune.rs` - Records module and resume recording after a live delete
- `packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs` - Startup loads locks; the Fjall sink records live deleted heights
- `packages/open-bitcoin-node/src/chainstate/flush_lifecycle/prune_apply.rs` - One summary write of `DeletedLiveMate` heights after the plan loop
- `packages/open-bitcoin-node/src/chainstate/fjall_store.rs` - Production chainstate sink delegates that summary write
- `packages/open-bitcoin-node/src/chainstate/flush_lifecycle/tests/initialize.rs` - Saved-lock refusal, absent-record resume, and AlreadyAbsent recount
- `packages/open-bitcoin-chainstate/src/prune/locks.rs` - Docs now describe the durable operator lock
- `docs/parity/source-breadcrumbs.json` - `node-prune-records` cites blockstorage.cpp and blockstorage.h

## Decisions Made

- Tests and implementation ship in one hook-passing feat commit per task because pre-commit runs `bash scripts/verify.sh`.
- Support counters move only for `DeletedLiveMate` heights. A crash after the delete batch and before `record_successful_prune_batch` under-counts, and a later `AlreadyAbsent` finish does not increment again.
- `FjallChainstateStore` delegates the summary write because the live flush sink is that wrapper, not `FjallNodeStore` alone.
- LOCK-02 and OPER-03 stay pending until lifecycle-valid Phase 150 verification. This plan does not add RPC.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - Missing Critical] Record live deletes through the production chainstate sink**
- **Found during:** Task 2 (Resume with loaded locks and count only live deletes)
- **Issue:** `apply_prune_plan` writes the summary through `FlushPersistSink`. The live flush path uses `FjallChainstateStore`, whose default method would have returned `Ok` without recording.
- **Fix:** Override `record_successful_prune_batch` on `FjallChainstateStore` and delegate to the inner Fjall store method.
- **Files modified:** `packages/open-bitcoin-node/src/chainstate/fjall_store.rs`
- **Verification:** `flush_lifecycle::tests` including the applying-plan cases passed.
- **Committed in:** `8b41d8bc` (Task 2 commit)


**Total deviations:** 1 auto-fixed (1 missing critical)
**Impact on plan:** The delegation is required so in-process prune counts the same live deletes that startup resume counts. No RPC or lock-formula change.

## Issues Encountered

None

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Ready for 150-04 status snapshot support counts. Lock list/set RPC remains later. LOCK-02 and OPER-03 stay pending until phase verification.

## Self-Check: PASSED

- FOUND: packages/open-bitcoin-node/src/storage/fjall_store/prune/records.rs
- FOUND: packages/open-bitcoin-node/src/storage/fjall_store/tests/prune_records.rs
- FOUND: 2549f401
- FOUND: 8b41d8bc

---
*Phase: 150-operator-prune-surfaces-and-evidence*
*Completed: 2026-09-29*
