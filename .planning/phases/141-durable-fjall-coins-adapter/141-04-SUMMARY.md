---
phase: 141-durable-fjall-coins-adapter
plan: 04
subsystem: storage
tags: [schema-2, leftover-migrate, coins-hydrate, undo-records, chain-meta, rust]

requires:
  - phase: 141-durable-fjall-coins-adapter
    provides: FjallCoinsView BatchWrite and fail-closed coins reads
  - phase: 141-durable-fjall-coins-adapter
    provides: MemoryChainstateStore view-backed surface and standalone undo codec
provides:
  - "SchemaVersion::CURRENT = 2 with one-way leftover snapshot migrate into C/B, undo:, and chain_meta"
  - "hydrate_chainstate_for_open as DurableSyncRuntime::open UTXO authority"
  - "Schema 2 leftover plus empty coins fails closed without remigrate"
  - "Two-element H plus missing B returns InterruptedWrite on hydrate"
affects:
  - persist-cutover
  - replay-blocks
  - phase-142

tech-stack:
  added: []
  patterns:
    - "Store schema 2 is independent of leftover/wallet/mempool blob schema 1 readability"
    - "Leftover snapshot stays on disk after migrate; coins are reopen UTXO truth"
    - "Transitional persist_progress dual-write: leftover snapshot plus seed_coins_from_leftover_for_reopen"

key-files:
  created:
    - packages/open-bitcoin-node/src/storage/fjall_store/coins.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/tests/coins_migration.rs
    - packages/open-bitcoin-node/src/storage/snapshot_codec/chain_meta.rs
  modified:
    - packages/open-bitcoin-node/src/storage.rs
    - packages/open-bitcoin-node/src/storage/fjall_store.rs
    - packages/open-bitcoin-node/src/sync.rs
    - packages/open-bitcoin-node/src/sync/runtime_state.rs
    - scripts/check-phase135-snapshot-recovery.ts
    - docs/parity/source-breadcrumbs.json

key-decisions:
  - "Combined 141-04 RED and GREEN into one hook-passing feat commit because pre-commit runs verify.sh"
  - "persist_progress still writes leftover snapshots and also seeds coins so schema-2 reopen can hydrate until Phase 142 write-site cutover"
  - "Phase 135 snapshot-recovery checks now require store CURRENT = 2 and DurableSyncRuntime hydrate_chainstate_for_open"
  - "COIN-01 and CSOBS-03 remain Pending until phase verification"

patterns-established:
  - "Pattern 1: ensure_schema migrates leftover once on schema 1 plus empty coins, then writes schema 2; schema 2 leftover plus empty coins is Corruption RestoreFromBackup"
  - "Pattern 2: hydrate_chainstate_for_open calls head_blocks before any C scan; leftover utxos are ignored after coins exist"

requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 141-2026-09-04T17-37-21
generated_at: 2026-09-04T23:35:15Z

duration: 118min
completed: 2026-09-04
---

# Phase 141 Plan 04: Schema 1→2 Migration and Leftover Non-Authority Summary

**Schema 2 store with one-way leftover UTXO migrate into coins/undo/chain_meta, and DurableSyncRuntime::open hydrating coins rather than leftover snapshots**

## Performance

- **Duration:** 118 min
- **Started:** 2026-09-04T21:37:04Z
- **Completed:** 2026-09-04T23:35:15Z
- **Tasks:** 2
- **Files modified:** 25

## Accomplishments

- `SchemaVersion::CURRENT` is 2. Fresh open writes schema 2 with empty coins and no leftover snapshot.
- Schema 1 leftover snapshot migrates once into `C`/`B` coins, `undo:` records, and `chain_meta`, then writes schema 2. The leftover `"snapshot"` blob stays on disk.
- Schema 2 treats coins as UTXO truth. Leftover utxos are not copied after coins exist. Schema 2 leftover plus empty coins fails closed without remigrate. Schema 1 leftover plus non-empty coins is Corruption.
- `DurableSyncRuntime::open` hydrates `MemoryChainstateStore` from `hydrate_chainstate_for_open`. Two-element `H` plus missing `B` returns `InterruptedWrite` and never `Ok(Some(snapshot))`.
- Blob schema 1 leftover/wallet/mempool JSON still decodes. `save_block` / `load_block` stay on `block:` hex keys. `ReplayBlocks` is not implemented.

## Task Commits

Each task was committed atomically:

1. **Task 1: Write failing schema-2, migrate, leftover-non-authority, and blob-read tests** — RED verified locally by writing the locked test names first; atomic RED commit skipped because `.githooks/pre-commit` runs workspace `verify.sh`.
2. **Task 2: Implement schema 1→2 migrate, Fjall undo, and coins hydrate** - `0c14e1a9` (feat)

**Plan metadata:** (pending docs commit)

## Files Created/Modified

- `packages/open-bitcoin-node/src/storage.rs` — `CURRENT = 2` and `blob_schema_is_readable` for versions 1 and 2
- `packages/open-bitcoin-node/src/storage/fjall_store/coins.rs` — migrate, hydrate, undo, leftover seed helper
- `packages/open-bitcoin-node/src/storage/fjall_store/tests/coins_migration.rs` — schema 2 / leftover-non-authority / interrupted-H tests
- `packages/open-bitcoin-node/src/storage/snapshot_codec/chain_meta.rs` — `encode_chain_meta` / `decode_chain_meta`
- `packages/open-bitcoin-node/src/sync.rs` — open hydrates coins
- `packages/open-bitcoin-node/src/sync/runtime_state.rs` — leftover persist plus transitional coins seed
- `scripts/check-phase135-snapshot-recovery.ts` — store schema 2 and coins hydrate locks

## Decisions Made

- Combined RED and GREEN because pre-commit runs full `verify.sh`.
- Production `persist_progress` still writes leftover snapshots (D-19) and additionally seeds coins so schema-2 reopen can hydrate. Phase 142 owns leftover write-site removal.
- Phase 135 mutation checks now treat store `CURRENT = 2` as required and lock `hydrate_chainstate_for_open` on `DurableSyncRuntime::open`. Leftover confirmation migration remains on schema 1 migrate and on the RPC leftover load path.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Combined RED+GREEN into one hook-passing feat commit**
- **Found during:** Task 1
- **Issue:** `.githooks/pre-commit` runs `bash scripts/verify.sh`, so a failing RED commit cannot land.
- **Fix:** Implemented tests and production code together, then committed once.
- **Files modified:** plan implementation files
- **Verification:** `coins_migration` 11/11 and workspace `verify.sh` via pre-commit
- **Committed in:** `0c14e1a9`

**2. [Rule 3 - Blocking] Transitional coins seed after leftover persist**
- **Found during:** Task 2
- **Issue:** Schema 2 leftover plus empty coins fails closed, so leftover-only persist made reopen fixtures and bench/RPC restart paths fail.
- **Fix:** `persist_progress` still writes leftover and now calls `seed_coins_from_leftover_for_reopen`. Restart fixtures that write leftover directly also seed.
- **Files modified:** `runtime_state.rs`, `coins.rs`, bench/RPC restart fixtures
- **Verification:** node lib 779 passed; bench, RPC, and `open-bitcoind` daemon tests passed
- **Committed in:** `0c14e1a9`

**3. [Rule 3 - Blocking] Phase 135 checks required leftover confirmation-migration on open and store schema 1**
- **Found during:** Task 2 commit hook
- **Issue:** Historical Phase 135 corpus failed after `CURRENT = 2` and `hydrate_chainstate_for_open`.
- **Fix:** Require store schema 2 and coins hydrate on runtime open; leftover confirmation migration stays on schema 1 migrate and RPC leftover load.
- **Files modified:** `scripts/check-phase135-snapshot-recovery.ts`, `.test.ts`
- **Verification:** `bun test scripts/check-phase135-snapshot-recovery.test.ts` 84/84
- **Committed in:** `0c14e1a9`

**4. [Rule 1 - Bug] Hydrate leftover+empty now decodes leftover before fail-closed**
- **Found during:** Task 2 reopen regressions
- **Issue:** Malformed leftover planted on schema 2 empty coins returned coins `RestoreFromBackup` instead of leftover decode `Chainstate` / `Repair`.
- **Fix:** Decode leftover first; malformed leftover keeps the existing chainstate corruption; valid leftover plus empty coins still fails closed.
- **Files modified:** `packages/open-bitcoin-node/src/storage/fjall_store/coins.rs`
- **Verification:** `phase70_malformed_stored_chainstate_is_storage_blocker` and leftover-empty fail-closed tests
- **Committed in:** `0c14e1a9`

**5. [Rule 3 - Blocking] Missing active-chain body no longer blocks coins hydrate**
- **Found during:** Task 2
- **Issue:** `phase70_missing_active_chain_block_body_blocks_runtime_open` expected leftover confirmation-migration failure; open no longer loads leftover utxos.
- **Fix:** Updated the test to expect a successful hydrate from coins/chain_meta (headers 3, blocks 2).
- **Files modified:** `packages/open-bitcoin-node/src/sync/tests/reorg_reconciliation.rs`
- **Verification:** that test plus reopen height tests
- **Committed in:** `0c14e1a9`

---

**Total deviations:** 5 auto-fixed (1 bug, 4 blocking)
**Impact on plan:** Persist leftover writes remain. Dual-write seed is transitional until Phase 142. No `ReplayBlocks`. No persist write-site cutover off leftover.

## Issues Encountered

- Reopen tests that persist leftover more than once needed seed to overwrite coins/chain_meta on each persist, not only when coins are empty.
- Bench leftover fixtures lack full active-chain block bodies, so seed must not run leftover confirmation migration. Daemon leftover confirmation stays an explicit leftover-load call.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Schema 2 + leftover-non-authority hydrate is in place for phase verification.
- Phase 142 still owns persist write-site cutover off leftover snapshots and `ReplayBlocks`.
- `COIN-01` and `CSOBS-03` stay Pending until phase verification.

## Self-Check: PASSED

## Self-Check: PASSED

---
*Phase: 141-durable-fjall-coins-adapter*
*Completed: 2026-09-04*
