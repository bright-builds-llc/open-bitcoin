---
phase: 142
fixed_at: 2026-09-13T20:00:38Z
review_path: .planning/phases/142-manager-flush-lifecycle-and-restart/142-REVIEW.md
iteration: 1
findings_in_scope: 6
fixed: 6
skipped: 0
status: all_fixed
---

# Phase 142: Code Review Fix Report

**Fixed at:** 2026-09-13T20:00:38Z
**Source review:** .planning/phases/142-manager-flush-lifecycle-and-restart/142-REVIEW.md
**Iteration:** 1

**Summary:**
- Findings in scope: 6
- Fixed: 6
- Skipped: 0

Overlapping files were split by primary finding so the tree stayed compiling under pre-commit `verify.sh`. `coins_view.rs` in WR-01 also carries `batch_write_sync` and fail-closed `collect_unspent_hint`. `chainstate.rs` in WR-02 also uses `store.disk_free_bytes()`. `coins.rs` / `cache.rs` in WR-03 also change the hint trait to `Result`. `coins_flush.rs` in WR-06 also probes `store.datadir()` instead of `"."`.

Info findings IN-01, IN-02, and IN-03 were out of `critical_warning` scope and were left as-is.

## Fixed Issues

### WR-01: Replay persist clobbers interrupted old-head marker

**Files modified:** `packages/open-bitcoin-node/src/storage/coins_view.rs`, `packages/open-bitcoin-node/src/storage/coins_view/tests.rs`, `docs/metrics/lines-of-code.md`
**Commit:** cd85feef
**Status:** fixed: requires human verification
**Applied fix:** `old_tip_for_heads()` keeps `MarkerState::Interrupted { old }` when `B` is missing, so replay persist writes `H=[new, old]` instead of `H=[new, 0]`. Added `replay_partial_commit_preserves_interrupted_old_head`.

### WR-02: `persist()` swallows every flush error

**Files modified:** `packages/open-bitcoin-node/src/chainstate.rs`, `packages/open-bitcoin-node/src/chainstate/tests.rs`, `packages/open-bitcoin-node/src/network/lifecycle_projection.rs`, `packages/open-bitcoin-node/src/network/lifecycle_projection/authority.rs`, `packages/open-bitcoin-node/src/network/mempool_lifecycle.rs`, `scripts/check-phase134-apply-boundaries/aggregate-roots.ts`, `scripts/check-phase134-apply-boundaries/reachability.ts`, `docs/metrics/lines-of-code.md`
**Commit:** e0888dfd
**Status:** fixed: requires human verification
**Applied fix:** `persist()` returns `Result<(), StorageError>`. Connect, disconnect, reorg, and prepared-commit map that error. Prepared connect still uses the checker-required `chainstate.commit_prepared_connect(prepared_chainstate);` fixture, then `persist_result.map_err(...)?` after the mempool transaction. Phase 134 checkers strip that persist-result map before after-transaction checks. `authority.rs` stays at 627 lines.

### WR-03: `FlushDecision::Sync` does not use `PersistMode::Sync`

**Files modified:** `packages/open-bitcoin-chainstate/src/coins.rs`, `packages/open-bitcoin-chainstate/src/coins/cache.rs`, `docs/metrics/lines-of-code.md`
**Commit:** 80c8d4ce
**Status:** fixed: requires human verification
**Applied fix:** `CoinsView::batch_write_sync` defaults to `batch_write`. `CoinsCache::sync` calls it. `FjallCoinsView` overrides with `PersistMode::Sync`.

### WR-04: Production disk-space guard is never armed

**Files modified:** `MODULE.bazel.lock`, `packages/Cargo.lock`, `packages/open-bitcoin-node/BUILD.bazel`, `packages/open-bitcoin-node/Cargo.toml`, `packages/open-bitcoin-node/src/chainstate/fjall_store.rs`, `packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs`, `packages/open-bitcoin-node/src/chainstate/flush_lifecycle/tests.rs`, `packages/open-bitcoin-node/src/storage/fjall_store.rs`, `docs/metrics/lines-of-code.md`
**Commit:** d089fcc1
**Status:** fixed
**Applied fix:** `probe_disk_free_bytes` uses `fs4::available_space` and fail-closes to `0`. `u64::MAX` remains only as the unprobed `FlushPersistSink` default. `FjallNodeStore` keeps the datadir path and implements `disk_free_bytes`. Added Bazel `@crate_index//:fs4` on the node crate.

### WR-05: `collect_unspent_hint` fails open to an empty parent UTXO set

**Files modified:** `packages/open-bitcoin-chainstate/src/coins/memory.rs`, `packages/open-bitcoin-chainstate/src/coins/tests.rs`, `packages/open-bitcoin-chainstate/src/engine.rs`, `packages/open-bitcoin-node/src/mempool.rs`, `packages/open-bitcoin-node/src/network.rs`, `packages/open-bitcoin-node/src/network/admission_bridge/local_package.rs`, `packages/open-bitcoin-node/src/network/admission_bridge/package.rs`, `packages/open-bitcoin-node/src/network/inventory.rs`, `packages/open-bitcoin-node/src/network/lifecycle_projection/recovery.rs`, `packages/open-bitcoin-node/src/network/recovery.rs`, `packages/open-bitcoin-node/src/network/runtime_authority.rs`, `packages/open-bitcoin-node/src/network/runtime_authority/recovery.rs`, `packages/open-bitcoin-node/src/network/tests/block_connect_disposition.rs`, `packages/open-bitcoin-node/src/network/tests/compact_cleanup_cases.rs`, `packages/open-bitcoin-node/src/network/tests/compact_timeout_cases.rs`, `packages/open-bitcoin-node/src/network/tests/recovery_cases.rs`, `packages/open-bitcoin-node/src/network/tests/runtime_projection.rs`, `docs/metrics/lines-of-code.md`
**Commit:** 5cd0de57
**Status:** fixed: requires human verification
**Applied fix:** Hint collection and admission snapshots return `Result`. Scan/decode errors map to `CoinsStorage` instead of an empty parent UTXO set. Callers propagate or map the error at the admission / recovery boundary.

### WR-06: Failed Periodic resample leaves `next_write` in the past

**Files modified:** `packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush.rs`, `packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush/tests.rs`, `docs/metrics/lines-of-code.md`
**Commit:** 2f7d75af
**Status:** fixed: requires human verification
**Applied fix:** Getrandom failure after a Periodic write sets `next_write` to `now + PERIODIC_WRITE_MIN_SECS`. `set_coins_next_write` errors return to the worker instead of leaving a due timestamp. The worker also probes `store.datadir()` (WR-04 call-site half).

---

_Fixed: 2026-09-13T20:00:38Z_
_Fixer: Claude (gsd-code-fixer)_
_Iteration: 1_
