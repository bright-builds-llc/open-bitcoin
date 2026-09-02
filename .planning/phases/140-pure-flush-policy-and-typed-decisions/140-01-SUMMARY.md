---
phase: 140-pure-flush-policy-and-typed-decisions
plan: 01
subsystem: chainstate
tags: [flush-policy, coins-cache, cache-size, decide-flush, rust]

requires:
  - phase: 139-coins-view-cache-contract-and-engine-apply
    provides: CoinsView/CoinsCache overlay algebra that later managers will Flush or Sync
provides:
  - "I/O-free FlushMode, FlushPolicyInput, FlushDecision, and FlushPolicyTime types"
  - "pub(crate) classify_cache_size with Knots 90% and 10 MiB LARGE math"
  - "decide_flush skeleton that always returns FlushDecision::None with classified facts"
affects:
  - 140-02
  - flush-write-kind
  - recovery-decision

tech-stack:
  added: []
  patterns:
    - "Injected FlushPolicyTime unix seconds; no Instant or SystemTime in core"
    - "u128 90% math with saturating total space so u64::MAX leftover cannot panic"

key-files:
  created:
    - packages/open-bitcoin-chainstate/src/coins/flush.rs
    - packages/open-bitcoin-chainstate/src/coins/tests/flush.rs
  modified:
    - packages/open-bitcoin-chainstate/src/coins.rs
    - packages/open-bitcoin-chainstate/src/coins/tests.rs
    - docs/parity/source-breadcrumbs.json
    - docs/metrics/lines-of-code.md

key-decisions:
  - "FlushPolicyTime is a u64 unix-seconds newtype; core never samples a clock"
  - "classify_cache_size is pub(crate) and is not re-exported from coins.rs"
  - "Plan 01 decide_flush always returns FlushDecision::None with LastFlushReason::None"
  - "Purity-test needles use concat! so check-pure-core-deps does not false-positive on std::fs or tokio"

patterns-established:
  - "Pattern 1: Shell injects occupancy, leftover, time, pressure, and disk-free facts; core classifies"
  - "Pattern 2: Shared FlushDecisionFacts on every exclusive FlushDecision variant"

requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 140-2026-09-02T02-14-08
generated_at: 2026-09-02T06:39:43Z

duration: 133min
completed: 2026-09-02
---

# Phase 140 Plan 01: Flush Types, Cache-Size Classification, and decide_flush Skeleton Summary

**I/O-free flush-policy vocabulary with Knots `max(90%, total - 10 MiB)` cache-size classification through a None-returning `decide_flush` skeleton**

## Performance

- **Duration:** 2h 13m
- **Started:** 2026-09-02T04:26:39Z
- **Completed:** 2026-09-02T06:39:43Z
- **Tasks:** 2
- **Files modified:** 6

## Accomplishments

- Added `FlushMode`, `FlushPolicyInput`, `FlushDecision`, `FlushPolicyTime`, and Knots LARGE constants in `coins/flush.rs` with no clock, filesystem, Fjall, or Tokio.
- `classify_cache_size` uses saturating total space and `u128` 90% math: 50 MiB binds at 90%, 200 MiB binds at `total - 10 MiB`, leftover adds to total, and `u64::MAX` leftover does not panic.
- None-mode `decide_flush` always returns `FlushDecision::None` with classified `CoinsCacheSizeState` and `LastFlushReason::None`, even when `disk_free_bytes` is 0.

## Task Commits

Each task was committed atomically:

1. **Task 1: Write failing None-mode classification and purity tests** — RED verified locally (`6 passed; 5 failed`, compile ok). Atomic RED commit skipped because `.githooks/pre-commit` runs workspace `cargo test`.
2. **Task 2: Implement classify_cache_size and the None-returning decide_flush skeleton** - `40fd4133` (feat)

**Plan metadata:** pending `docs(140-01): complete flush types and classification plan`

_Note: TDD tasks may have multiple commits (test → feat → refactor). This plan shipped one hook-passing feat commit after RED evidence._

## Files Created/Modified

- `packages/open-bitcoin-chainstate/src/coins/flush.rs` - Flush policy types, `classify_cache_size`, and None-returning `decide_flush`
- `packages/open-bitcoin-chainstate/src/coins/tests/flush.rs` - None-mode classification, purity, periodic-due, and accessor coverage
- `packages/open-bitcoin-chainstate/src/coins.rs` - `mod flush` and public re-exports (not crate-root)
- `packages/open-bitcoin-chainstate/src/coins/tests.rs` - `mod flush`
- `docs/parity/source-breadcrumbs.json` - `chainstate-engine` files for flush sources
- `docs/metrics/lines-of-code.md` - Hook-regenerated LOC freshness

## Decisions Made

- `FlushPolicyTime` is a `u64` unix-seconds newtype with `from_unix_seconds` / `unix_seconds`; `periodic_due` is `now >= next_write`.
- `classify_cache_size` stays `pub(crate)` so Plan 02 composes write-kind from `decide_flush`, not a second public classifier.
- Plan 01 `decide_flush` ignores mode, pressure, disk-free, and entry count and always returns `FlushDecision::None`.
- Coverage tests construct `Flush`, `Sync`, and `RefuseDiskSpace` only to prove shared-fact accessors; production still never returns those variants.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Split purity-test needles so pure-core import scan can pass**
- **Found during:** Task 1 commit (pre-commit `check-pure-core-deps.sh`)
- **Issue:** The source-purity test listed contiguous `std::fs` and `tokio` strings, which the pure-core import scanner treats as forbidden imports in `open-bitcoin-chainstate`.
- **Fix:** Build those needles with `concat!("std::", "fs")` and `concat!("tok", "io")`.
- **Files modified:** `packages/open-bitcoin-chainstate/src/coins/tests/flush.rs`
- **Verification:** `bash scripts/check-pure-core-deps.sh` exits 0; purity test still fails if `flush.rs` contains the real tokens.
- **Committed in:** `40fd4133` (Task 2 feat commit)

**2. [Rule 3 - Blocking] Cover `unix_seconds`, `periodic_due`, and every `FlushDecision` accessor arm**
- **Found during:** Task 2 commit (pre-commit `cargo llvm-cov` uncovered `flush.rs` lines 33–35, 95–96, 118–120)
- **Issue:** New production helpers were untested; the coverage gate failed the hook.
- **Fix:** Added unit tests for injected unix seconds, `periodic_due` at and before `next_write`, and `cache_size()` / `reason()` on all four decision variants.
- **Files modified:** `packages/open-bitcoin-chainstate/src/coins/tests/flush.rs`
- **Verification:** `cargo llvm-cov --package open-bitcoin-chainstate --lib` no longer lists `flush.rs`; pre-commit `verify.sh` completed.
- **Committed in:** `40fd4133` (Task 2 feat commit)

**3. [Rule 3 - Blocking] Combined RED and GREEN into one hook-passing feat commit**
- **Found during:** Task 1 commit
- **Issue:** `.githooks/pre-commit` runs `bash scripts/verify.sh`, including workspace `cargo test`. A RED-only commit cannot pass hooks, and `--no-verify` is disallowed.
- **Fix:** Kept Task 1 RED evidence (`coins::tests::flush` compiled and failed 5 LARGE/CRITICAL cases). Implemented GREEN, then committed both tasks as `40fd4133`.
- **Files modified:** same Task 1 and Task 2 files
- **Verification:** RED log exit 101, then GREEN 15/15 and `verify.sh` 0
- **Committed in:** `40fd4133`

***

**Total deviations:** 3 auto-fixed (3 blocking)
**Impact on plan:** Required for hook-compatible TDD in this repo. No scope creep into write-kind, disk refusal, or recovery.

## Issues Encountered

- First RED commit failed at `check-pure-core-deps.sh` on the purity test's own forbidden-token list.
- First GREEN commit failed at `cargo llvm-cov` on untested `unix_seconds`, `periodic_due`, and or-pattern accessor arms.
- Rust-analyzer `cargo check --workspace` held the artifact lock for ~19 minutes before the RED test compile started.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan 02 can TDD IfNeeded / Periodic / Always write-kind and `RefuseDiskSpace` against the existing `decide_flush` API.
- `lib.rs` crate-root exports, `RecoveryDecision`, leftover persist, and `empty_cache` remain untouched as specified.
- FLUSH-01 and MGR-03 stay pending until Plans 02 and 03 close write-kind, refusal, and recovery types.

***
*Phase: 140-pure-flush-policy-and-typed-decisions*
*Completed: 2026-09-02*

## Self-Check: PASSED
