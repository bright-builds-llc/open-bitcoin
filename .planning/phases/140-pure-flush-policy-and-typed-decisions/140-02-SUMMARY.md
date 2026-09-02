---
phase: 140-pure-flush-policy-and-typed-decisions
plan: 02
subsystem: chainstate
tags: [flush-policy, decide-flush, write-kind, disk-refusal, rust]

requires:
  - phase: 140-pure-flush-policy-and-typed-decisions
    provides: I/O-free FlushMode, FlushPolicyInput, FlushDecision, and classify_cache_size skeleton
provides:
  - "Knots FlushStateToDisk mapping: Always/Periodic LARGE-or-CRITICAL flush, Periodic OK due sync, IfNeeded CRITICAL-or-pressure flush"
  - "First-class RefuseDiskSpace after should-write using 192 bytes per cache entry"
  - "IfNeeded LARGE without pressure stays None; None mode never writes or refuses"
affects:
  - 140-03
  - flush-write-kind
  - persist-wiring-guard

tech-stack:
  added: []
  patterns:
    - "Private FlushWriteKind plus maybe_intended_write; adapters match FlushDecision only"
    - "disk_guard_fails after intended write; no disk_ok flag and no 50 MiB CheckDiskSpace floor"

key-files:
  created: []
  modified:
    - packages/open-bitcoin-chainstate/src/coins/flush.rs
    - packages/open-bitcoin-chainstate/src/coins/tests/flush.rs
    - docs/metrics/lines-of-code.md

key-decisions:
  - "Periodic LARGE/CRITICAL flushes even when not due; IfNeeded LARGE without pressure returns None"
  - "RefuseDiskSpace replaces an intended Flush or Sync only; guard is free_bytes < 192 * entry_count"
  - "Combined 140-02 RED and GREEN into one hook-passing feat commit because pre-commit runs cargo test"

patterns-established:
  - "Pattern 3: classify then maybe_intended_write then disk_guard_fails; write-kind is the enum variant"
  - "Pattern 4: IfNeeded LARGE is not a write trigger; fCacheLarge stays Periodic-only"

requirements-completed: [FLUSH-01]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 140-2026-09-02T02-14-08
generated_at: 2026-09-02T08:21:18Z

duration: 79min
completed: 2026-09-02
---

# Phase 140 Plan 02: Knots Flush, Sync, and Disk-Refusal Matrix Summary

**Knots `FlushStateToDisk` write-kind mapping with Periodic-only LARGE flush and first-class `RefuseDiskSpace` after should-write**

## Performance

- **Duration:** 1h 19m
- **Started:** 2026-09-02T07:01:58Z
- **Completed:** 2026-09-02T08:21:18Z
- **Tasks:** 2
- **Files modified:** 3

## Accomplishments

- Replaced the Plan 01 `decide_flush` skeleton with `maybe_intended_write` plus post-should-write `disk_guard_fails`.
- Periodic LARGE/CRITICAL returns `Flush` even when not due; Periodic OK due returns `Sync`; IfNeeded LARGE without pressure returns `None`.
- Disk refusal uses `192 * entry_count` with a strict `<` comparison, never coexists with a write, and is skipped when no write is intended.

## Task Commits

Each task was committed atomically:

1. **Task 1: Write failing Flush, Sync, and RefuseDiskSpace matrix tests** — RED verified locally (`19 passed; 9 failed`, compile ok). Atomic RED commit skipped because `.githooks/pre-commit` runs workspace `cargo test`.
2. **Task 2: Implement maybe_intended_write and post-should-write disk refusal** - `d1ac9965` (feat)

**Plan metadata:** pending `docs(140-02): complete Knots flush write-kind and disk-refusal plan`

_Note: TDD tasks may have multiple commits (test → feat → refactor). This plan shipped one hook-passing feat commit after RED evidence._

## Files Created/Modified

- `packages/open-bitcoin-chainstate/src/coins/flush.rs` - Private `FlushWriteKind`, `maybe_intended_write`, `disk_guard_fails`, and Knots `decide_flush` mapping
- `packages/open-bitcoin-chainstate/src/coins/tests/flush.rs` - Mode × cache × pressure × due × disk matrix
- `docs/metrics/lines-of-code.md` - Hook-regenerated LOC freshness

## Decisions Made

- `fCacheLarge` is Periodic-only: Periodic LARGE/CRITICAL flushes even when `periodic_due` is false; IfNeeded LARGE without pressure is `None`.
- Disk refusal runs only after an intended write and uses `COIN_WRITE_GUARD_BYTES_PER_ENTRY.saturating_mul(entry_count)` with no `50 MiB` addend.
- Combined RED and GREEN into one hook-passing feat commit because pre-commit `verify.sh` includes workspace `cargo test`.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Combined RED and GREEN into one hook-passing feat commit**
- **Found during:** Task 1 commit
- **Issue:** `.githooks/pre-commit` runs `bash scripts/verify.sh`, including workspace `cargo test`. A RED-only commit cannot pass hooks, and `--no-verify` is disallowed.
- **Fix:** Kept Task 1 RED evidence (`coins::tests::flush` compiled and failed 9 write/refuse cases; 19 Plan 01 and None-path tests still passed). Implemented GREEN, then committed both tasks as `d1ac9965`.
- **Files modified:** `packages/open-bitcoin-chainstate/src/coins/flush.rs`, `packages/open-bitcoin-chainstate/src/coins/tests/flush.rs`
- **Verification:** RED log exit 101 (`19 passed; 9 failed`), then GREEN 28/28 and `verify.sh` 0
- **Committed in:** `d1ac9965` (Task 2 feat commit)

**2. [Rule 3 - Blocking] Re-ran workspace rustfmt after a bare rustfmt reordered imports**
- **Found during:** Task 2 commit (first hook run failed at rustfmt check)
- **Issue:** `rustfmt` without the workspace crate config reordered `use crate::coins::{...}` away from the repo rustfmt grouping.
- **Fix:** Ran `cargo fmt --manifest-path packages/Cargo.toml -p open-bitcoin-chainstate` and recommitted.
- **Files modified:** `packages/open-bitcoin-chainstate/src/coins/tests/flush.rs`
- **Verification:** Second pre-commit `verify.sh` completed in 1h 1m
- **Committed in:** `d1ac9965`

---

**Total deviations:** 2 auto-fixed (2 blocking)
**Impact on plan:** Required for hook-compatible TDD and rustfmt. No scope creep into FlushForPrune, persist, or RecoveryDecision.

## Issues Encountered

- First commit attempt failed at rustfmt import grouping after a bare `rustfmt` pass.
- Pre-commit `verify.sh` ran 1h 1m, including slow merged doctest compilation; advisory stall evidence was written and the job completed without interruption.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan 03 can add the I/O-free `RecoveryDecision` sketch, crate-root exports, and leftover-persist guard without retargeting `persist_progress`.
- `FlushForPrune`, coins-cache algebra, and node persist remain untouched as specified.
- MGR-03 stays pending until Plan 03 ships recovery types.

---
*Phase: 140-pure-flush-policy-and-typed-decisions*
*Completed: 2026-09-02*

## Self-Check: PASSED
