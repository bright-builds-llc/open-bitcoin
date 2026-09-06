---
phase: 142-manager-flush-lifecycle-and-restart
plan: 05
subsystem: storage
tags: [flush-lifecycle, ifneeded, periodic, always, persist-cutover, rust]

requires:
  - phase: 142-manager-flush-lifecycle-and-restart
    provides: FlushLifecycle initialize and ordered execute_flush
  - phase: 140-pure-flush-policy-and-typed-decisions
    provides: FlushMode IfNeeded Periodic Always decide_flush
provides:
  - "ManagedChainstate owns a required FlushLifecycle; persist() always execute_flush(IfNeeded) with live coins_mut"
  - "open-bitcoind coins flush worker drives Periodic and Always through ManagedNetworkHandle"
  - "Flipped persist leftover-write guard; persist_progress leftover snapshot writes remain"
affects:
  - 142-06
  - persist-progress-cutover
  - canflush-init

tech-stack:
  added: []
  patterns:
    - "Required FlushLifecycle on ManagedChainstate; persist always IfNeeded, never Option-gated"
    - "Periodic/Always go through ManagedNetworkHandle the same way checkpoint.rs does"

key-files:
  created:
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush.rs
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush/tests.rs
  modified:
    - packages/open-bitcoin-node/src/chainstate.rs
    - packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs
    - packages/open-bitcoin-node/src/chainstate/tests.rs
    - packages/open-bitcoin-chainstate/src/engine.rs
    - packages/open-bitcoin-node/src/network/runtime_authority.rs
    - packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs
    - packages/open-bitcoin-chainstate/src/coins/tests/flush.rs
    - docs/parity/source-breadcrumbs.json
    - docs/metrics/lines-of-code.md

key-decisions:
  - "Combined 142-05 RED and GREEN into one hook-passing feat commit because pre-commit runs verify.sh"
  - "FlushLifecycle is a required ManagedChainstate field; persist() always execute_flush(IfNeeded)"
  - "Periodic/Always go through ManagedNetworkHandle; the worker does not own a second lifecycle"
  - "Leave MGR-01 Pending until persist_progress cutover and lifecycle-valid phase verification"

patterns-established:
  - "Pattern 1: persist() always calls self.flush_lifecycle.execute_flush(IfNeeded) with coins_mut() and a defined flush window"
  - "Pattern 2: Daemon Periodic/Always use handle.flush_coins and set_coins_next_write; Always precedes the clean marker"

requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 142-2026-09-06T16-23-52
generated_at: 2026-09-06T22:35:35Z

duration: 60min
completed: 2026-09-06
---

# Phase 142 Plan 05: IfNeeded / Periodic / Always Call Sites Summary

**`ManagedChainstate` now owns a required `FlushLifecycle`, `persist()` always runs `execute_flush(IfNeeded)` on the live `CoinsCache`, and `open-bitcoind` drives Periodic plus Always through `ManagedNetworkHandle` without leftover persist snapshot dumps.**

## Performance

- **Duration:** 60 min
- **Started:** 2026-09-06T21:35:45Z
- **Completed:** 2026-09-06T22:35:35Z
- **Tasks:** 2
- **Files modified:** 16

## Accomplishments

- `ManagedChainstate` stores `flush_lifecycle: FlushLifecycle` (not `Option`). `from_store` always constructs `FlushLifecycle::ready(...)`.
- `persist()` always calls `self.flush_lifecycle.execute_flush(IfNeeded)` with `chainstate.coins_mut()` and a defined flush window. Connect/reorg/commit still call `persist()` only.
- `open_bitcoind/coins_flush.rs` copies the checkpoint thread+channel pattern. Periodic ticks and Always shutdown go through `ManagedNetworkHandle::flush_coins`. After a Periodic coins write, the worker resamples 50–70 minute jitter via `set_coins_next_write`.
- Shutdown Always-then-joins the coins worker before `worker.shutdown_and_mark_clean()?` so the Phase 135 clean-marker order stays intact.
- Flipped `managed_chainstate_persist_calls_decide_flush_and_does_not_write_snapshot`. `persist_progress` leftover snapshot writes remain for Plan 06.

## Task Commits

Each task was committed atomically:

1. **Task 1: Write failing persist-guard and worker tests** — RED verified by construction (flipped persist leftover-write guard plus coins-flush worker tests). Atomic RED commit skipped because `.githooks/pre-commit` runs workspace `verify.sh`.
2. **Task 2: Implement IfNeeded persist and the coins-flush worker** - `be23ebf9` (feat)

**Plan metadata:** included in the docs(142-05) complete-plan commit

## Files Created/Modified

- `packages/open-bitcoin-node/src/chainstate.rs` — required `FlushLifecycle`, `persist()` always `execute_flush(IfNeeded)`
- `packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs` — `FlushLifecycle::ready`, `&mut self` sink, `MemoryChainstateStore` persist
- `packages/open-bitcoin-node/src/chainstate/tests.rs` — required-owner, IfNeeded-only connect, store-backed persist asserts
- `packages/open-bitcoin-chainstate/src/engine.rs` — `Chainstate::coins_mut`
- `packages/open-bitcoin-chainstate/src/coins/tests.rs` — `coins_mut` coverage
- `packages/open-bitcoin-chainstate/src/coins/tests/flush.rs` — flipped persist leftover-write guard
- `packages/open-bitcoin-node/src/network.rs` and `runtime_authority.rs` — `chainstate_mut`, `flush_coins`, `set_coins_next_write`
- `packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush.rs` — Periodic/Always worker over `ManagedNetworkHandle`
- `packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush/tests.rs` — source, jitter, and no-second-lifecycle tests
- `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs` — start worker beside checkpoint; Always then clean marker
- `docs/parity/source-breadcrumbs.json` — `rpc-coins-flush-daemon`
- `docs/metrics/lines-of-code.md` — hook-regenerated LOC freshness

## Decisions Made

- Combined RED and GREEN into one hook-passing feat commit because pre-commit runs `verify.sh`.
- `FlushLifecycle` is required on `ManagedChainstate`. `persist()` always calls `execute_flush(IfNeeded)` — no "when installed" hedge.
- Periodic/Always go through `ManagedNetworkHandle` like `checkpoint.rs`. The worker does not construct a second lifecycle.
- Leave `MGR-01` Pending until `persist_progress` cutover and lifecycle-valid phase verification.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] FlushPersistSink methods take `&mut self`**
- **Found during:** Task 2
- **Issue:** In-memory undo persist needs a mutable store; `&self` could not `save_undo`.
- **Fix:** Sink methods take `&mut self`.
- **Files modified:** `packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs`
- **Verification:** Node `chainstate` tests including undo persist
- **Committed in:** `be23ebf9` (part of combined feat commit)

**2. [Rule 3 - Blocking] `ChainstateStore` is a `FlushPersistSink` supertrait**
- **Found during:** Task 2
- **Issue:** Existing `impl<S: ChainstateStore>` bounds would not compile after the sink split.
- **Fix:** Made `ChainstateStore: FlushPersistSink` so current store impls stay valid.
- **Files modified:** `packages/open-bitcoin-node/src/chainstate.rs`, `flush_lifecycle.rs`
- **Verification:** `cargo test -p open-bitcoin-node --lib chainstate`
- **Committed in:** `be23ebf9` (part of combined feat commit)

**3. [Rule 3 - Blocking] Did not add `ManagedChainstate<S, V>`**
- **Found during:** Task 2
- **Issue:** `Chainstate<V>` lacks the Debug/Eq derives `ManagedChainstate<S>` already uses.
- **Fix:** Kept `ManagedChainstate<S>` with default `Chainstate` and used `coins_mut()` on that engine.
- **Files modified:** `packages/open-bitcoin-node/src/chainstate.rs`, `packages/open-bitcoin-chainstate/src/engine.rs`
- **Verification:** Targeted chainstate and node tests
- **Committed in:** `be23ebf9` (part of combined feat commit)

**4. [Rule 3 - Blocking] Mapped `getrandom` errors through `to_string()`**
- **Found during:** Task 2
- **Issue:** getrandom 0.3 `Error` is not `std::error::Error`, so `std::io::Error::other(error)` does not compile.
- **Fix:** `std::io::Error::other(error.to_string())`.
- **Files modified:** `packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush.rs`
- **Verification:** `open-bitcoind -- coins_flush` tests
- **Committed in:** `be23ebf9` (part of combined feat commit)

**5. [Rule 3 - Blocking] Explicit `#[path = "coins_flush/tests.rs"]`**
- **Found during:** Task 2
- **Issue:** Plain `mod tests` resolved to the existing `open_bitcoind/tests.rs`.
- **Fix:** `#[cfg(test)] #[path = "coins_flush/tests.rs"] mod tests;`
- **Files modified:** `packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush.rs`
- **Verification:** Three `coins_flush` tests ran
- **Committed in:** `be23ebf9` (part of combined feat commit)

**6. [Rule 3 - Blocking] Restored Phase 135 shutdown strings**
- **Found during:** Task 2 (pre-commit `verify.sh`)
- **Issue:** A helper hid `if let Some(worker) = maybe_checkpoint_worker` and `worker.shutdown_and_mark_clean()?`.
- **Fix:** Inline Always then checkpoint clean in `open-bitcoind.rs` and compact preflight impls so the file stays under 628 lines.
- **Files modified:** `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs`
- **Verification:** `bun test scripts/check-phase135-snapshot-recovery.test.ts`
- **Committed in:** `be23ebf9` (part of combined feat commit)

**7. [Rule 2 - Missing Critical] `coins_mut` unit coverage**
- **Found during:** Task 2 (pre-commit llvm-cov)
- **Issue:** Pure-core coverage reported `engine.rs` 151–153 uncovered because only the node crate called `coins_mut`.
- **Fix:** Added `coins_mut_exposes_the_same_live_cache` in chainstate coins tests.
- **Files modified:** `packages/open-bitcoin-chainstate/src/coins/tests.rs`
- **Verification:** Full pre-commit `verify.sh` llvm-cov
- **Committed in:** `be23ebf9` (part of combined feat commit)

***

**Total deviations:** 7 auto-fixed (6 blocking, 1 missing critical)
**Impact on plan:** Required for compile, Phase 135 checker, file-length, and coverage. No persist_progress cutover. `MGR-01` stays Pending.

## Issues Encountered

Pre-commit `verify.sh` rejected the first attempts for the Phase 135 shutdown-string checker, the 628-line production cap (must stay *below* 628), and uncovered `coins_mut` lines. Each was fixed before the feat commit landed.

## Authentication Gates

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Ready for `142-06-PLAN.md`. IfNeeded persist and daemon Periodic/Always are wired.
- `persist_progress` leftover snapshot writes are intentionally still present.
- `MGR-01` remains Pending until later plans and lifecycle-valid phase verification.

***
*Phase: 142-manager-flush-lifecycle-and-restart*
*Completed: 2026-09-06*

## Self-Check: PASSED
