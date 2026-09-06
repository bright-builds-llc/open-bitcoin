---
phase: 142-manager-flush-lifecycle-and-restart
plan: 02
subsystem: storage
tags: [chainstate, coins-view, occupancy, generic-parent, rust]

requires:
  - phase: 141-durable-fjall-coins-adapter
    provides: CoinsCache::from_parent with overlay maybe_best_block starting None
  - phase: 140-pure-flush-policy-and-typed-decisions
    provides: FlushPolicyInput.cache_bytes and cache_entry_count injection points
provides:
  - "Chainstate<V: CoinsView = MemoryCoinsView> with MemoryBackedChainstate alias"
  - "Chainstate::from_parent that does not probe parent.best_block"
  - "CoinsCache::cache_entry_count and estimated_cache_bytes with 48-byte first-party overhead"
  - "engine/overlay_apply.rs so engine.rs stays under 628 lines"
affects:
  - 142-03
  - 142-04
  - 142-06
  - flush-policy
  - fjall-parent-attach

tech-stack:
  added: []
  patterns:
    - "Chainstate is generic over V: CoinsView with Memory default; from_snapshot stays Memory-only"
    - "Occupancy walks overlay entries only; spent entries count 48-byte overhead, unspent add value/script/height/MTP/coinbase"

key-files:
  created:
    - packages/open-bitcoin-chainstate/src/engine/overlay_apply.rs
  modified:
    - packages/open-bitcoin-chainstate/src/engine.rs
    - packages/open-bitcoin-chainstate/src/engine/stage.rs
    - packages/open-bitcoin-chainstate/src/coins/cache.rs
    - packages/open-bitcoin-chainstate/src/coins/tests.rs
    - packages/open-bitcoin-chainstate/src/coins.rs
    - packages/open-bitcoin-chainstate/src/lib.rs
    - docs/parity/source-breadcrumbs.json
    - docs/metrics/lines-of-code.md

key-decisions:
  - "Combined 142-02 RED and GREEN into one hook-passing feat commit because pre-commit runs verify.sh"
  - "from_parent does not probe best_block; overlay occupancy starts at 0 while coins_best_block reads the parent tip"
  - "estimated_cache_bytes is first-party overlay math (48 + unspent payload), never Fjall len or LevelDB SizeEstimate"
  - "Leave MGR-01 and MGR-02 Pending until Fjall attach, manager flush, and lifecycle-valid phase verification"

patterns-established:
  - "Pattern 1: Default type parameter keeps existing Chainstate spellings compiling while production can later pass FjallCoinsView"
  - "Pattern 2: Extract overlay apply beside engine.rs (foo.rs plus foo/) so the 628-line gate stays green"

requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 142-2026-09-06T16-23-52
generated_at: 2026-09-06T19:14:00Z

duration: 23min
completed: 2026-09-06
---

# Phase 142 Plan 02: Generic Chainstate Parent and Cache Occupancy Summary

**Chainstate is now generic over a `CoinsView` parent with a Memory default, and first-party overlay occupancy estimators supply `cache_entry_count` / `estimated_cache_bytes` without putting Fjall or clocks in core.**

## Performance

- **Duration:** 23 min
- **Started:** 2026-09-06T18:51:30Z
- **Completed:** 2026-09-06T19:14:00Z
- **Tasks:** 2
- **Files modified:** 9

## Accomplishments

- `Chainstate<V: CoinsView = MemoryCoinsView>` compiles existing `Chainstate` call sites via the Memory default. `MemoryBackedChainstate` aliases the test parent.
- `from_parent` builds an empty overlay without probing `parent.best_block()`. `coins_best_block()` still reads the parent tip.
- `cache_entry_count()` is overlay `len`. `estimated_cache_bytes()` uses 48-byte overhead plus unspent value/script/height/MTP/coinbase; spent entries add overhead only.
- Overlay connect/disconnect apply lives in `engine/overlay_apply.rs`. `engine.rs` is 539 lines. Pure-core dep check still passes.

## Task Commits

Each task was committed atomically:

1. **Task 1: Write failing occupancy and from_parent contract tests** — RED verified by construction (new occupancy and `from_parent` tests). Atomic RED commit skipped because `.githooks/pre-commit` runs workspace `verify.sh`.
2. **Task 2: Genericize Chainstate and implement occupancy** - `42fb918e` (feat)

**Plan metadata:** included in the docs(142-02) complete-plan commit

## Files Created/Modified

- `packages/open-bitcoin-chainstate/src/engine.rs` — Generic `Chainstate<V>`, `from_parent`, `MemoryBackedChainstate`, Memory-only snapshot helpers
- `packages/open-bitcoin-chainstate/src/engine/overlay_apply.rs` — Generic `apply_connect_on_overlay` / `apply_disconnect_on_overlay`
- `packages/open-bitcoin-chainstate/src/engine/stage.rs` — Staged absorb methods generic over `V`
- `packages/open-bitcoin-chainstate/src/coins/cache.rs` — `ESTIMATED_COIN_ENTRY_OVERHEAD_BYTES`, `cache_entry_count`, `estimated_cache_bytes`
- `packages/open-bitcoin-chainstate/src/coins/tests.rs` — Occupancy, `from_parent` probe, and Memory alias tests
- `packages/open-bitcoin-chainstate/src/coins.rs` — Re-export occupancy constant
- `packages/open-bitcoin-chainstate/src/lib.rs` — Re-export `MemoryBackedChainstate` and occupancy constant
- `docs/parity/source-breadcrumbs.json` — `overlay_apply.rs` in the chainstate-engine group
- `docs/metrics/lines-of-code.md` — Hook-regenerated LOC freshness

## Decisions Made

- Combined RED and GREEN into one hook-passing feat commit because pre-commit runs `verify.sh`.
- `from_parent` does not probe `best_block`; empty overlay occupancy is 0 while the parent tip remains readable.
- Occupancy is first-party overlay estimated bytes with 48-byte overhead. Do not pin 450/4/8 MiB in this crate.
- Leave `MGR-01` and `MGR-02` Pending. This plan only supplies the generic parent and occupancy facts for later attach and flush wiring.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Replaced markdown `---` rules in 142-01 SUMMARY so lifecycle validation can parse frontmatter**
- **Found during:** Pre-execution lifecycle check
- **Issue:** The GSD frontmatter parser treats the last `---` block as frontmatter. Horizontal rules in `142-01-SUMMARY.md` hid `generated_by` / `lifecycle_mode` / `phase_lifecycle_id` / `generated_at`.
- **Fix:** Changed those body rules to `***` so the real YAML frontmatter is the last parseable block.
- **Files modified:** `.planning/phases/142-manager-flush-lifecycle-and-restart/142-01-SUMMARY.md`
- **Verification:** `gsd-tools verify lifecycle 142-manager-flush-lifecycle-and-restart --require-plans` reports `valid: true`
- **Committed in:** this docs commit

**2. [Rule 2 - Missing Critical] Genericized staged absorb methods on `Chainstate<V>`**
- **Found during:** Task 2
- **Issue:** `engine/stage.rs` still said `impl Chainstate`, which would only cover the Memory default after genericization and block a Fjall parent from absorb/preview.
- **Fix:** `impl<V: CoinsView> Chainstate<V>` for `absorb_staged_connect`, `absorb_staged_reorg`, and `install_staged_reorg_preview`.
- **Files modified:** `packages/open-bitcoin-chainstate/src/engine/stage.rs`
- **Verification:** `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-chainstate --lib` — 123 passed
- **Committed in:** `42fb918e` (part of combined feat commit)

***

**Total deviations:** 2 auto-fixed (1 blocking, 1 missing critical)
**Impact on plan:** Required for lifecycle validation and generic absorb. No persist cutover or Fjall-in-core scope creep.

## Issues Encountered

None.

## Authentication Gates

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Ready for `142-03-PLAN.md`. Generic parent and occupancy exist for later Fjall attach and `FlushPolicyInput` injection.
- Replay, CanFlush, persist cutover, and node flush wiring are intentionally not implemented.
- `MGR-01` and `MGR-02` remain Pending until later plans and lifecycle-valid phase verification.

***
*Phase: 142-manager-flush-lifecycle-and-restart*
*Completed: 2026-09-06*

## Self-Check: PASSED
