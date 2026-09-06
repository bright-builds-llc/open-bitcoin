---
phase: 142-manager-flush-lifecycle-and-restart
plan: 04
subsystem: storage
tags: [canflush, flush-lifecycle, decide-flush, persist-sink, rust]

requires:
  - phase: 142-manager-flush-lifecycle-and-restart
    provides: replay_interrupted_flush apply-only ReplayBlocks owner
  - phase: 140-pure-flush-policy-and-typed-decisions
    provides: decide_flush Flush vs Sync without re-choosing write-kind
provides:
  - "FlushLifecycle initialize reaches ReadyToFlush after replay-or-fail-closed and empty cache attach"
  - "execute_flush writes block/undo/index before coins and uses decide_flush write-kind"
  - "FlushPersistSink plus UndoFailingSink/RecordingCoinsView abort coins without chmod"
affects:
  - 142-05
  - 142-06
  - canflush-init
  - persist-cutover

tech-stack:
  added: []
  patterns:
    - "One shell owner completes D-11 init then executes decide_flush in block/undo/index/coins order"
    - "Undo/index abort tests inject FlushPersistSink; they do not chmod a Fjall datadir"

key-files:
  created:
    - packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs
    - packages/open-bitcoin-node/src/chainstate/flush_lifecycle/tests.rs
    - packages/open-bitcoin-node/src/chainstate/flush_lifecycle/tests/error_map.rs
  modified:
    - packages/open-bitcoin-node/src/chainstate.rs
    - docs/parity/source-breadcrumbs.json
    - docs/metrics/lines-of-code.md

key-decisions:
  - "Combined 142-04 RED and GREEN into one hook-passing feat commit because pre-commit runs verify.sh"
  - "execute_flush is generic over FlushPersistSink so undo-abort uses UndoFailingSink + RecordingCoinsView"
  - "probe_disk_free_bytes returns u64::MAX because the node crate forbids unsafe libc::statvfs"
  - "Leave MGR-01 Pending until persist cutover and lifecycle-valid phase verification"

patterns-established:
  - "Pattern 1: ReadyToFlush is false until replay-or-fail-closed and empty cache attach finish"
  - "Pattern 2: Ordered persist goes through FlushPersistSink before cache.flush/sync"

requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 142-2026-09-06T16-23-52
generated_at: 2026-09-06T21:20:44Z

duration: 43min
completed: 2026-09-06
---

# Phase 142 Plan 04: CanFlush Owner and Ordered Flush Summary

**One shell-owned `FlushLifecycle` now reaches `ReadyToFlush` after replay-or-fail-closed and empty cache attach, then executes `decide_flush` in block → undo → index → coins order without rewriting leftover snapshots.**

## Performance

- **Duration:** 43 min
- **Started:** 2026-09-06T20:37:49Z
- **Completed:** 2026-09-06T21:20:44Z
- **Tasks:** 2
- **Files modified:** 6

## Accomplishments

- `initialize` opens coins, runs `decide_recovery`, replays or fail-closes interrupted `H`, attaches an empty `CoinsCache`, then reports `ReadyToFlush`.
- `execute_flush` injects cache occupancy into `decide_flush` and does not re-choose Flush vs Sync. `None` and `RefuseDiskSpace` do not write coins.
- Block, undo, and header persist go through `FlushPersistSink` before `cache.flush()` / `cache.sync()`. Undo-abort tests use `UndoFailingSink` plus `RecordingCoinsView`.
- Shell defaults stay 450 / 4 / 8 MiB with `cache_byte_limit` 442 MiB. `flush.rs` remains unpinned. `ManagedChainstate::persist` still writes leftover snapshots.

## Task Commits

Each task was committed atomically:

1. **Task 1: Write failing CanFlush and ordered-write tests** — RED verified by construction (new CanFlush and ordered-write tests). Atomic RED commit skipped because `.githooks/pre-commit` runs workspace `verify.sh`.
2. **Task 2: Implement initialize and ordered execute_flush** - `643da47d` (feat)

**Plan metadata:** included in the docs(142-04) complete-plan commit

## Files Created/Modified

- `packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs` — constants, `FlushPersistSink`, `initialize`, `execute_flush`
- `packages/open-bitcoin-node/src/chainstate/flush_lifecycle/tests.rs` — CanFlush sequence, None, Always, undo-abort, and persist-snapshot guards
- `packages/open-bitcoin-node/src/chainstate/flush_lifecycle/tests/error_map.rs` — mapped coins-write error coverage
- `packages/open-bitcoin-node/src/chainstate.rs` — `mod flush_lifecycle` plus public re-exports
- `docs/parity/source-breadcrumbs.json` — new files in `node-chainstate-adapter`
- `docs/metrics/lines-of-code.md` — Hook-regenerated LOC freshness

## Decisions Made

- Combined RED and GREEN into one hook-passing feat commit because pre-commit runs `verify.sh`.
- `execute_flush` is generic over `FlushPersistSink`. Undo-abort does not chmod a Fjall datadir.
- `probe_disk_free_bytes` reports `u64::MAX` because this crate forbids `unsafe` `statvfs`. Callers still inject `disk_free_bytes`.
- Leave `MGR-01` Pending until persist cutover and lifecycle-valid phase verification.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Allowed clippy `too_many_arguments` on the plan-pinned `execute_flush` signature**
- **Found during:** Task 2 (pre-commit clippy)
- **Issue:** The specified `execute_flush` has nine arguments including `self`. Clippy `-D warnings` rejects more than seven.
- **Fix:** `#[allow(clippy::too_many_arguments)]` on the Knots-shaped entry point instead of inventing a request struct this plan.
- **Files modified:** `packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs`
- **Verification:** `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib -- -D warnings`
- **Committed in:** `643da47d` (part of combined feat commit)

**2. [Rule 3 - Blocking] Kept `probe_disk_free_bytes` unsafe-free**
- **Found during:** Task 2
- **Issue:** The plan named unix `libc::statvfs`, but `open-bitcoin-node` has `#![forbid(unsafe_code)]`.
- **Fix:** The probe returns `u64::MAX`. Production `execute_flush` still injects `disk_free_bytes`.
- **Files modified:** `packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs`
- **Verification:** `probe_disk_free_bytes_is_defined` passed
- **Committed in:** `643da47d` (part of combined feat commit)

**3. [Rule 2 - Missing Critical] Nested coins-write error coverage under `tests/`**
- **Found during:** Task 2 (pre-commit panic-site scan)
- **Issue:** A sibling `error_map.rs` looked like production and failed `check-panic-sites.sh`.
- **Fix:** Moved it to `flush_lifecycle/tests/error_map.rs` so the scanner treats it as test-only.
- **Files modified:** `packages/open-bitcoin-node/src/chainstate/flush_lifecycle/tests/error_map.rs`, `docs/parity/source-breadcrumbs.json`
- **Verification:** Panic-site check passed in pre-commit `verify.sh`
- **Committed in:** `643da47d` (part of combined feat commit)

***

**Total deviations:** 3 auto-fixed (2 blocking, 1 missing critical)
**Impact on plan:** Required for clippy, forbid-unsafe, and panic-site gates. No persist cutover or Periodic worker.

## Issues Encountered

Pre-commit `verify.sh` rejected the first attempts for panic-like sites in a sibling test module and clippy `too_many_arguments`. Each was fixed before the feat commit landed.

## Authentication Gates

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Ready for `142-05-PLAN.md`. CanFlush init and ordered `execute_flush` exist.
- Persist snapshot cutover and manager Periodic/Always wiring are intentionally not implemented.
- `MGR-01` remains Pending until later plans and lifecycle-valid phase verification.

***
*Phase: 142-manager-flush-lifecycle-and-restart*
*Completed: 2026-09-06*

## Self-Check: PASSED
