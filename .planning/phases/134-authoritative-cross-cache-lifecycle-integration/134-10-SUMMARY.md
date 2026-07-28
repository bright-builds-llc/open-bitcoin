---
phase: 134-authoritative-cross-cache-lifecycle-integration
plan: "10"
subsystem: node-storage-lifecycle
tags: [rust, fjall, mempool, lifecycle-effects, snapshot-receipts]

requires:
  - phase: 134-08
    provides: prepared snapshot capabilities, receipts, and shared lifecycle completion dispatch
provides:
  - outside-authority Fjall executor for prepared current-schema mempool snapshots
  - snapshot receipts minted only after encoding and requested persistence succeed
  - deterministic generation-safe completion and no-credit failure coverage
affects: [134-11, 134-12, 134-13, phase-135]

tech-stack:
  added: []
  patterns:
    - owned prepared snapshot crosses the authority boundary into the Fjall shell
    - storage success precedes consuming receipt acknowledgement
    - generation-aware completion distinguishes current, stale, and duplicate achieved effects

key-files:
  created: []
  modified:
    - packages/open-bitcoin-node/src/storage/fjall_store/mempool.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/tests/snapshot_persistence.rs
    - packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/effects.rs
    - docs/metrics/lines-of-code.md

key-decisions:
  - "Keep current-schema snapshot execution beside Fjall save_mempool_snapshot and outside lifecycle authority."
  - "Mint SnapshotWriteReceipt only after encoding and the caller-requested PersistMode complete successfully."
  - "Use public Plan 08 facades for end-to-end persistence/completion coverage while retaining exact dispatcher-state assertions for generation invariants."

patterns-established:
  - "Prepare-execute-complete: authority emits owned work, Fjall persists outside the lock, and only achieved storage returns a receipt for shared completion."
  - "Stale achieved truth: an older successful save is recorded as achieved but cannot clear a newer dirty generation."

requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 134-2026-07-28T01-41-12
generated_at: 2026-07-28T17:35:33Z

duration: 1h 10m
completed: 2026-07-28
---

# Phase 134 Plan 10: Truthful Fjall Snapshot Execution Summary

**Prepared current-schema mempool snapshots now persist through an outside-authority Fjall executor that creates completion receipts only after achieved storage and preserves newer dirty truth across stale completion.**

## Performance

- **Duration:** 1h 10m
- **Started:** 2026-07-28T16:25:45Z
- **Completed:** 2026-07-28T17:35:33Z
- **Tasks:** 2
- **Files modified:** 4

## Accomplishments

- Added `FjallNodeStore::execute_prepared_mempool_snapshot_write` beside the existing save primitive so owned snapshot work crosses out of authority before encoding and durable I/O.
- Ensured the executor consumes its capability into a `SnapshotWriteReceipt` only after the requested `PersistMode` succeeds; encoding and injected backend failures return no receipt.
- Proved exact-current completion clears only the matching pending/dirty generation, stale success preserves newer dirty state, and duplicate completion is mutation-free.
- Exercised the public Plan 08 prepare/complete facades with a temporary Fjall store across current, stale, duplicate, cap, encode-failure, and write-failure scenarios.

## Task Commits

Each task was committed atomically:

1. **Task 1: Execute the prepared current-schema snapshot beside the Fjall save primitive** - `f0e9566f` (feat)
2. **Task 2: Prove generation-safe snapshot completion and failures** - `cbffce41` (test)

## Files Created/Modified

- `packages/open-bitcoin-node/src/storage/fjall_store/mempool.rs` - Executes owned prepared snapshots through the existing current-schema save path and acknowledges only achieved storage.
- `packages/open-bitcoin-node/src/storage/fjall_store/tests/snapshot_persistence.rs` - Verifies temporary-store success, requested durability, backend failure, and released pending capacity.
- `packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/effects.rs` - Proves current, stale, duplicate, cap, encode-failure, and public-facade persistence/completion truth.
- `docs/metrics/lines-of-code.md` - Refreshes the intentionally tracked generated LOC evidence through normal hooks.

## Decisions Made

- Kept encoding and Fjall persistence entirely outside lifecycle authority; the executor has no authority lock or mutation access.
- Reused the existing current-schema `MempoolSnapshot` codec and `save_mempool_snapshot` path without adding Phase 135 schema, cadence, coordinator, recovery, journal, or crash-durability behavior.
- Kept failure injection private to tests so the production API exposes only the truthful consume-save-acknowledge boundary.
- Left `MPLIFE-04` pending for phase-level reconciliation and verification, as required.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Factored lifecycle-effect test setup to satisfy the Bright Builds file-size guard**

- **Found during:** Task 2 mandatory Bright Builds verification
- **Issue:** The added scenario coverage expanded `effects.rs` to 673 lines, exceeding its managed 628-line limit.
- **Fix:** Extracted shared local-spend and snapshot-preparation helpers while preserving the exact behavioral assertions, bringing the file back to 628 lines.
- **Files modified:** `packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/effects.rs`
- **Verification:** Both focused suites, all Rust gates, Bright Builds checks, and the normal full repository hook pass.
- **Committed in:** `cbffce41`

**Total deviations:** 1 auto-fixed blocking issue.
**Impact on plan:** The refactor reduced duplication without changing scope or behavior.

## Verification

- Focused Fjall snapshot persistence suite: 6 tests passed.
- Focused lifecycle snapshot effect suite: 16 tests passed.
- Both task-specific ordered `cargo fmt`, Clippy with `-D warnings`, all-target build, and all-feature test gates passed.
- `bun scripts/bright-builds-check.ts all` passed after the size-limit simplification.
- Both normal task commit hooks passed the full `bash scripts/verify.sh` repository contract.
- Final focused suites and `git diff --check` passed.

## Known Stubs

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan 11 can build its deterministic scenario matrix on a real outside-authority snapshot executor with truthful current, stale, duplicate, and failed completion evidence.
- Phase 135 snapshot schema, cadence, recovery, and crash-window behavior remain deliberately absent.
- `MPLIFE-04` remains pending until phase-level verification reconciles the complete lifecycle surface.

## Self-Check: PASSED

All key files and task commits were found, the summary uses only the two frontmatter delimiters, and `git diff --check` passes.

*Phase: 134-authoritative-cross-cache-lifecycle-integration*
*Completed: 2026-07-28*
