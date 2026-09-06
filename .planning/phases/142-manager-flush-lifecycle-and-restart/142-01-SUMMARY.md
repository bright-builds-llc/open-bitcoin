---
phase: 142-manager-flush-lifecycle-and-restart
plan: 01
subsystem: storage
tags: [interrupted-write, head-blocks, leftover-empty, typed-remap, coins-view, rust]

requires:
  - phase: 140-pure-flush-policy-and-typed-decisions
    provides: decide_recovery count-only InterruptedTwoHeads sketch
  - phase: 141-durable-fjall-coins-adapter
    provides: Fjall C/B/H markers, leftover-empty fail-closed, Display-text map_heads_error
provides:
  - "MarkerState::Interrupted { new, old } with head_blocks Ok([new, old])"
  - "FjallNodeStore::open succeeds on two-element H without B"
  - "Leftover-empty skipped when decide_recovery is InterruptedTwoHeads"
  - "ChainstateError::InterruptedWrite typed remap without Display substring search"
  - "hydrate_chainstate_for_open still returns StorageError::InterruptedWrite for interrupted H"
affects:
  - 142-02
  - 142-03
  - flush-replay
  - leftover-empty

tech-stack:
  added: []
  patterns:
    - "Classify two-element H before leftover-empty; leftover-empty only when H is not InterruptedTwoHeads"
    - "Remap interrupted writes by ChainstateError::InterruptedWrite, never detail.contains Display text"

key-files:
  created: []
  modified:
    - packages/open-bitcoin-chainstate/src/error.rs
    - packages/open-bitcoin-node/src/storage/coins_view.rs
    - packages/open-bitcoin-node/src/storage/coins_view/tests.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/coins.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/tests/coins_migration.rs
    - docs/metrics/lines-of-code.md

key-decisions:
  - "Combined 142-01 RED and GREEN into one hook-passing feat commit because pre-commit runs verify.sh"
  - "classify_markers (2, None) is Ok(Interrupted); store open succeeds; hydrate still fail-closes"
  - "map_heads_error matches ChainstateError::InterruptedWrite; leftover-empty skipped when InterruptedTwoHeads"
  - "Leave FLUSH-02 Pending until replay and lifecycle-valid phase verification"

patterns-established:
  - "Pattern 1: Two-element H without B is an observable marker at head_blocks, not an open-time hard fail"
  - "Pattern 2: Typed InterruptedWrite crosses the chainstate/storage boundary without Display flattening"

requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 142-2026-09-06T16-23-52
generated_at: 2026-09-06T18:23:00Z

duration: 21min
completed: 2026-09-06
---

# Phase 142 Plan 01: Observable Interrupted H and Typed Remap Summary

**Two-element `H` without `B` is now an observable interrupted marker: store open succeeds, `head_blocks` returns both hashes, leftover-empty cannot shadow it, and remap is typed `InterruptedWrite`.**

## Performance

- **Duration:** 21 min
- **Started:** 2026-09-06T18:01:44Z
- **Completed:** 2026-09-06T18:23:00Z
- **Tasks:** 2
- **Files modified:** 6

## Accomplishments

- `classify_markers` returns `MarkerState::Interrupted { new, old }` for two-element `H` plus missing `B`. `head_blocks()` returns those hashes; `best_block()` returns `None`.
- `FjallNodeStore::open` succeeds on interrupted `H`. `ensure_schema_two` skips leftover-empty when `decide_recovery(heads.len())` is `InterruptedTwoHeads`.
- `hydrate_chainstate_for_open` still returns `StorageError::InterruptedWrite` for interrupted `H` so leftover scanners do not invent a tip.
- `map_storage` / `map_heads_error` carry `ChainstateError::InterruptedWrite`. The Display substring `detail.contains("interrupted write")` is gone.

## Task Commits

Each task was committed atomically:

1. **Task 1: Write failing interrupted-marker and typed-remap tests** — RED verified by construction (new tests plus `InterruptedWrite` variant). Atomic RED commit skipped because `.githooks/pre-commit` runs workspace `verify.sh`.
2. **Task 2: Implement Interrupted marker, leftover-empty skip, and typed remap** - `d86db036` (feat)

**Plan metadata:** `cc914a8d` (docs: complete plan)

## Files Created/Modified

- `packages/open-bitcoin-chainstate/src/error.rs` — `InterruptedWrite { heads }` with Display `interrupted coins write`
- `packages/open-bitcoin-node/src/storage/coins_view.rs` — Observable `Interrupted` marker; typed `map_storage`
- `packages/open-bitcoin-node/src/storage/coins_view/tests.rs` — Two-hash `head_blocks` coverage; existing crash-open tests follow the new contract
- `packages/open-bitcoin-node/src/storage/fjall_store/coins.rs` — `decide_recovery` before leftover-empty; typed `map_heads_error`
- `packages/open-bitcoin-node/src/storage/fjall_store/tests/coins_migration.rs` — Leftover-plus-H opens; source remap guard
- `docs/metrics/lines-of-code.md` — Hook-regenerated LOC freshness

## Decisions Made

- Combined RED and GREEN into one hook-passing feat commit because pre-commit runs `verify.sh`.
- Two-element `H` without `B` is observable at store open and `head_blocks`; hydrate remains fail-closed so leftover scanners cannot invent a tip.
- Remap interrupted writes by typed `ChainstateError::InterruptedWrite`, never Display text.
- Leave `FLUSH-02` Pending; this plan only makes the interrupted marker observable for later replay.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Updated existing fail-closed-at-open tests to the observable-marker contract**
- **Found during:** Task 2
- **Issue:** `two_element_h_without_b_is_interrupted_write`, `simulate_crash_after_partial_leaves_h_and_fails_closed`, and `map_storage_preserves_storage_detail` still expected open/`head_blocks`/`best_block` to error and `map_storage` to flatten into `CoinsStorage`.
- **Fix:** Those tests now assert `head_blocks` returns two hashes, `best_block` is `None`, store open succeeds, and `map_storage` yields `ChainstateError::InterruptedWrite`. `batch_write` still refuses a present `H` as typed interrupt.
- **Files modified:** `packages/open-bitcoin-node/src/storage/coins_view/tests.rs`
- **Verification:** `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib storage::` — 142 passed
- **Committed in:** `d86db036` (part of combined feat commit)

***

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** Required for GREEN under the new D-09/D-10 contract. No scope creep; leftover-write guards and `flush.rs` stay unflipped.

## Issues Encountered

None.

## Authentication Gates

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Ready for `142-02-PLAN.md`. Interrupted `H` is observable at store open and `head_blocks`.
- Replay, CanFlush, and persist cutover are intentionally not implemented.
- `FLUSH-02` remains Pending until later plans replay or fail closed from stored undo/bodies.

***
*Phase: 142-manager-flush-lifecycle-and-restart*
*Completed: 2026-09-06*

## Self-Check: PASSED
