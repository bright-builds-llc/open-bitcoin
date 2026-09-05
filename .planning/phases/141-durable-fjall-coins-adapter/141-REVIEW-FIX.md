---
phase: 141
fixed_at: 2026-09-05T22:31:16Z
review_path: .planning/phases/141-durable-fjall-coins-adapter/141-REVIEW.md
iteration: 1
findings_in_scope: 5
fixed: 5
skipped: 0
status: all_fixed
---

# Phase 141: Code Review Fix Report

**Fixed at:** 2026-09-05T22:31:16Z
**Source review:** .planning/phases/141-durable-fjall-coins-adapter/141-REVIEW.md
**Iteration:** 1

**Summary:**
- Findings in scope: 5
- Fixed: 5
- Skipped: 0

## Fixed Issues

### CR-01: Seed merge resurrects spent coins on reopen

**Files modified:** `packages/open-bitcoin-node/src/storage/fjall_store/coins.rs`, `packages/open-bitcoin-node/src/storage/fjall_store/tests/coins_migration.rs`, `docs/metrics/lines-of-code.md`
**Commit:** 18d9ad22
**Status:** fixed: requires human verification
**Applied fix:** Leftover seed now replaces the UTXO set. `write_migrated_coins_with_tip` marks on-disk `C` keys that are missing from leftover as `spent_dirty` before BatchWrite, so a later spend/persist cannot resurrect those coins on hydrate.

### WR-01: Runtime open hydrates leftover snapshot slot, not the view-backed maps

**Files modified:** `packages/open-bitcoin-node/src/sync.rs`, `packages/open-bitcoin-node/src/sync/tests/restart_chainstate.rs`, `docs/metrics/lines-of-code.md`
**Commit:** b4ccc9d3
**Status:** fixed
**Applied fix:** `DurableSyncRuntime::open` hydrates through `MemoryChainstateStore::from_snapshot` instead of `save_snapshot`, so view-backed coins/undo maps match the coins-truth snapshot.

### WR-02: Schema-2 leftover-plus-empty check shadows interrupted `H`

**Files modified:** `packages/open-bitcoin-node/src/storage/fjall_store/coins.rs`, `packages/open-bitcoin-node/src/storage/fjall_store/tests/coins_migration.rs`, `packages/open-bitcoin-node/src/storage/coins_view/tests.rs`, `docs/metrics/lines-of-code.md`
**Commit:** 635223d5
**Status:** fixed: requires human verification
**Applied fix:** `ensure_schema_two` classifies `head_blocks` before leftover-plus-empty. Two-element `H` without `B` is `InterruptedWrite` even when leftover is present and `C`/`B` are missing. The partial-crash coins-view test now expects that same fail-closed open.

### WR-03: Schema 1 migrate is not crash-resumable after coins exist

**Files modified:** `packages/open-bitcoin-node/src/storage/fjall_store/coins.rs`, `packages/open-bitcoin-node/src/storage/fjall_store/tests/coins_migration.rs`, `docs/metrics/lines-of-code.md`
**Commit:** 3369ee74
**Status:** fixed: requires human verification
**Applied fix:** Schema 1 leftover plus consistent coins `B` (empty `H`, matching leftover tip and UTXO set) finishes undo/`chain_meta` and bumps schema 2. Leftover plus conflicting coins still fail closed as `RestoreFromBackup`.

### WR-04: Leftover-then-seed persist order can fail closed or hydrate stale coins

**Files modified:** `packages/open-bitcoin-node/src/storage/fjall_store/coins.rs`, `packages/open-bitcoin-node/src/sync/runtime_state.rs`, `packages/open-bitcoin-node/src/sync/tests/restart_chainstate.rs`, `packages/open-bitcoin-node/src/storage/fjall_store/tests/coins_migration.rs`, `docs/metrics/lines-of-code.md`
**Commit:** 485e5296
**Status:** fixed: requires human verification
**Applied fix:** `persist_progress` seeds coins from the in-memory snapshot (replace) before writing leftover. Leftover dual-write remains (D-19). A coins-only persist now reopens from coins instead of leftover-plus-empty fail-closed.

---

_Fixed: 2026-09-05T22:31:16Z_
_Fixer: Claude (gsd-code-fixer)_
_Iteration: 1_
