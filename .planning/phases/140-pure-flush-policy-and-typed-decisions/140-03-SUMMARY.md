---
phase: 140-pure-flush-policy-and-typed-decisions
plan: 03
subsystem: chainstate
tags: [recovery-decision, decide-recovery, crate-exports, persist-guard, rust]

requires:
  - phase: 140-pure-flush-policy-and-typed-decisions
    provides: Knots FlushStateToDisk mapping with Periodic-only LARGE flush and first-class RefuseDiskSpace
provides:
  - "Count-only RecoveryDecision: ConsistentEmptyHeads / OneHead / InterruptedTwoHeads / InconsistentOtherCount"
  - "Crate-root re-exports of flush and recovery types next to CoinsCache"
  - "Leftover persist guard: ManagedChainstate::persist and persist_progress still write snapshots"
affects:
  - 141
  - 142
  - head-blocks-encoding
  - replay-versus-fail-closed

tech-stack:
  added: []
  patterns:
    - "RecoveryDecision is injected-count only; no BlockHash, undo, or ReplayBlocks"
    - "include_str persist guards prove leftover snapshot writes are not flush-policy durability"

key-files:
  created: []
  modified:
    - packages/open-bitcoin-chainstate/src/coins/flush.rs
    - packages/open-bitcoin-chainstate/src/coins.rs
    - packages/open-bitcoin-chainstate/src/lib.rs
    - packages/open-bitcoin-chainstate/src/coins/tests/flush.rs
    - docs/metrics/lines-of-code.md

key-decisions:
  - "RecoveryDecision maps count 0/1/2/other; one-element is first-class, not InconsistentOtherCount"
  - "Crate root re-exports flush and recovery types; classify_cache_size stays crate-private"
  - "Combined 140-03 RED and GREEN into one hook-passing feat commit because pre-commit runs cargo test"
  - "Leave MGR-03 Pending and requirements-completed empty until lifecycle-valid phase verification"

patterns-established:
  - "Pattern 5: decide_recovery is a four-arm count match; Phase 142 owns replay versus fail-closed"
  - "Pattern 6: leftover persist stays snapshot-blob; tests fail if decide_flush appears in persist files"

requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 140-2026-09-02T02-14-08
generated_at: 2026-09-02T09:35:40Z

duration: 28min
completed: 2026-09-02
---

# Phase 140 Plan 03: RecoveryDecision Sketch, Crate Exports, and Leftover-Persist Guard Summary

**Count-only RecoveryDecision at the chainstate crate root, with leftover snapshot persist proven not to be flush-policy durability**

## Performance

- **Duration:** 28 min
- **Started:** 2026-09-02T09:07:23Z
- **Completed:** 2026-09-02T09:35:40Z
- **Tasks:** 2
- **Files modified:** 5

## Accomplishments

- Added I/O-free `RecoveryDecision` and `decide_recovery` as a four-arm count match: empty / one / two / other.
- Re-exported flush and recovery types from `open-bitcoin-chainstate` next to `CoinsCache`.
- Guarded leftover persist so `ManagedChainstate::persist` and `DurableSyncRuntime::persist_progress` still write snapshot blobs and do not call `decide_flush`.

## Task Commits

Each task was committed atomically:

1. **Task 1: Write failing recovery-count and leftover-persist guard tests** — RED verified locally (`32 passed; 4 failed`, compile ok). Atomic RED commit skipped because `.githooks/pre-commit` runs workspace `cargo test`.
2. **Task 2: Implement decide_recovery and crate-root re-exports** - `944611a3` (feat)

**Plan metadata:** pending `docs(140-03): complete RecoveryDecision sketch and leftover-persist guard`

_Note: TDD tasks may have multiple commits (test → feat → refactor). This plan shipped one hook-passing feat commit after RED evidence._

## Files Created/Modified

- `packages/open-bitcoin-chainstate/src/coins/flush.rs` - Count-only `RecoveryDecision` and `decide_recovery`
- `packages/open-bitcoin-chainstate/src/coins.rs` - Re-exports `RecoveryDecision` and `decide_recovery`
- `packages/open-bitcoin-chainstate/src/lib.rs` - Crate-root flush and recovery re-exports
- `packages/open-bitcoin-chainstate/src/coins/tests/flush.rs` - Recovery cardinality, crate-root, and leftover-persist guard tests
- `docs/metrics/lines-of-code.md` - Hook-regenerated LOC freshness

## Decisions Made

- One-element head-marker count is first-class `OneHead`, not `InconsistentOtherCount { count: 1 }` (D-20).
- Replay versus fail-closed stays unnamed beyond the sketch variants (D-21); no `ReplayBlocks`.
- Leftover persist remains `save_snapshot` / `save_chainstate_snapshot` and is not a flush-policy proof (D-22).
- Combined RED and GREEN into one hook-passing feat commit because pre-commit `verify.sh` includes workspace `cargo test`.
- Leave MGR-03 Pending until lifecycle-valid phase verification exists.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Combined RED and GREEN into one hook-passing feat commit**
- **Found during:** Task 1 commit
- **Issue:** `.githooks/pre-commit` runs `bash scripts/verify.sh`, including workspace `cargo test`. A RED-only commit cannot pass hooks, and `--no-verify` is disallowed.
- **Fix:** Kept Task 1 RED evidence (`coins::tests::flush` compiled and failed 4 recovery/export cases; 32 Plan 01/02 and persist-guard tests still passed). Implemented GREEN, then committed both tasks as `944611a3`.
- **Files modified:** `packages/open-bitcoin-chainstate/src/coins/flush.rs`, `packages/open-bitcoin-chainstate/src/coins.rs`, `packages/open-bitcoin-chainstate/src/lib.rs`, `packages/open-bitcoin-chainstate/src/coins/tests/flush.rs`
- **Verification:** RED log exit 101 (`32 passed; 4 failed`), then GREEN 36/36, persist/purity greps empty as required, `check-pure-core-deps.sh` 0, and `verify.sh` 0 in 24m 24s
- **Committed in:** `944611a3` (Task 2 feat commit)

**2. [Rule 3 - Blocking] Crate-root export test used include_str during RED, then crate:: constructions after GREEN**
- **Found during:** Task 1
- **Issue:** Writing `crate::decide_recovery` before crate-root exports would fail compilation, contradicting "RED, not compile errors."
- **Fix:** Task 1 asserted `lib.rs` `pub use coins` names via `include_str!`. Task 2 replaced that with constructions through `crate::FlushPolicyTime`, `crate::decide_flush`, and `crate::decide_recovery`.
- **Files modified:** `packages/open-bitcoin-chainstate/src/coins/tests/flush.rs`
- **Verification:** RED crate-root assertion failed; GREEN constructed crate-root types
- **Committed in:** `944611a3`

**3. [Rule 3 - Blocking] Leave MGR-03 Pending and requirements-completed empty**
- **Found during:** Plan metadata commit
- **Issue:** `requirements mark-complete MGR-03` failed `check-active-milestone-verification-traceability` because no lifecycle-valid `140-VERIFICATION.md` covers MGR-03.
- **Fix:** Keep `requirements-completed: []` and leave MGR-03 Pending until phase verification. Do not invent a verification report in this plan.
- **Files modified:** `.planning/phases/140-pure-flush-policy-and-typed-decisions/140-03-SUMMARY.md`, `.planning/REQUIREMENTS.md`
- **Verification:** First docs commit hook failed on MGR-03 activation; retry keeps MGR-03 Pending
- **Committed in:** pending docs commit

---

**Total deviations:** 3 auto-fixed (3 blocking)
**Impact on plan:** Required for hook-compatible TDD, compiling RED, and milestone traceability. No scope creep into ReplayBlocks, Fjall coins view, or persist retargeting.

## Issues Encountered

- Pre-commit `verify.sh` ran 24m 24s, including slow merged doctest compilation; timing heartbeats continued and the job completed without interruption.
- The first docs commit failed the active-milestone traceability checker after flipping MGR-03 Complete. MGR-03 stays Pending until phase verification.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Phase 141 can own `head_blocks` encoding without this sketch inventing hashes or a tip.
- Phase 142 can choose replay versus fail-closed; `RecoveryDecision` only names marker-count facts.
- `ManagedChainstate::persist` and `persist_progress` remain leftover snapshot writes.
- MGR-03 stays Pending until a lifecycle-valid `140-VERIFICATION.md` exists.

---
*Phase: 140-pure-flush-policy-and-typed-decisions*
*Completed: 2026-09-02*

## Self-Check: PASSED
