---
phase: 70-reorg-peer-rotation-and-no-progress-recovery
plan: 02
subsystem: sync-runtime
tags: [sync, reorg, storage-recovery, runtime-status, deterministic-tests]

requires: [70-01]
provides:
  - Typed internal branch and reorg reconcile outcomes
  - Durable latest reorg and reconcile progress projection
  - Storage-first blockers for missing active bodies, missing undo, and malformed chainstate
  - Deterministic branch competition and reorg coverage
affects: [phase-70, sync-runtime, chainstate, durable-status, storage]

tech-stack:
  added: []
  patterns:
    - Runtime summary carries bounded reconcile outcomes into durable status projection
    - Existing ManagedPeerNetwork reorg path remains the only reorg execution path
    - Storage blockers are typed before peer retry guidance

key-files:
  created:
    - .planning/phases/70-reorg-peer-rotation-and-no-progress-recovery/70-02-SUMMARY.md
    - packages/open-bitcoin-node/src/sync/reconcile_status.rs
    - packages/open-bitcoin-node/src/sync/session.rs
    - packages/open-bitcoin-node/src/sync/types/error.rs
  modified:
    - .planning/ROADMAP.md
    - .planning/STATE.md
    - docs/metrics/lines-of-code.md
    - docs/parity/source-breadcrumbs.json
    - packages/open-bitcoin-node/src/storage/fjall_store.rs
    - packages/open-bitcoin-node/src/sync.rs
    - packages/open-bitcoin-node/src/sync/block_reconcile.rs
    - packages/open-bitcoin-node/src/sync/runtime_state.rs
    - packages/open-bitcoin-node/src/sync/tests.rs
    - packages/open-bitcoin-node/src/sync/types.rs
    - packages/open-bitcoin-node/src/sync/types/summary.rs

key-decisions:
  - "Keep reorg execution delegated to ManagedPeerNetwork::reorg_to_branch and project only bounded evidence."
  - "Record reconcile progress in runtime state so peer-loop outcomes are not discarded before summary projection."
  - "Map missing undo during active-chain reorg to storage corruption repair because it proves local chainstate evidence is incomplete."
  - "Split large sync type/session helpers into child modules to keep production Rust files under the repo file-length limit."

patterns-established:
  - "SyncReconcileProgress is the internal runtime outcome; SyncReconcileProgressStatus is the public bounded projection."
  - "latest_reorg persists as the latest durable evidence only, with no event history."
  - "Phase 70 branch and storage blockers use deterministic local fixtures, not public-network checks."

requirements-completed: [REC-01, REC-02]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 70-2026-06-12T14-56-46
generated_at: 2026-06-12T20:17:45Z

duration: 102min
completed: 2026-06-12
---

# Phase 70 Plan 02: Branch/Reorg Runtime and Storage Blockers Summary

**Typed branch/reorg outcomes now flow through runtime summaries into durable status, with deterministic storage-blocker coverage.**

## Performance

- **Duration:** 102 min
- **Started:** 2026-06-12T18:35:00Z
- **Completed:** 2026-06-12T20:17:45Z
- **Tasks:** 2
- **Files modified:** 11
- **Files created:** 4

## Accomplishments

- Replaced bool-style best-chain reconciliation with `SyncReconcileProgress` outcomes for no change, active-chain extension, missing branch bodies, side-branch preservation, and persisted reorgs.
- Preserved the existing `reorg_to_branch` adapter as the only reorg execution path and built `SyncReorgEvidence` from the returned transition.
- Recorded reconcile outcomes in the runtime and projected them into `SyncStatus.reconcile_progress` and `SyncStatus.latest_reorg`.
- Preserved prior durable `latest_reorg` evidence when later summaries have no new reorg.
- Mapped missing active-chain block bodies and missing undo data to storage repair blockers.
- Added deterministic Phase 70 tests for branch waiting, persisted reorg evidence, side-branch preservation, missing active bodies, missing undo, and malformed stored chainstate.
- Split error, reconcile-status, and session helpers into child modules to keep production Rust files below the line limit.
- Refreshed parity breadcrumbs and the tracked LOC report for the new modules.

## Task Commits

Implementation and summary are in the pending `70-02` commit prepared after verification.

## Files Created/Modified

- `packages/open-bitcoin-node/src/sync/types.rs` - Added the internal reconcile progress enum and summary field.
- `packages/open-bitcoin-node/src/sync/block_reconcile.rs` - Returned typed outcomes, preserved missing-body safety, built reorg evidence, and mapped missing undo to storage repair.
- `packages/open-bitcoin-node/src/sync.rs` - Captured reconcile outcomes in runtime state and persisted progress only for actual header/block/reorg progress.
- `packages/open-bitcoin-node/src/sync/reconcile_status.rs` - Projected internal reconcile outcomes into public durable status fields.
- `packages/open-bitcoin-node/src/sync/runtime_state.rs` - Applied reconcile status projection during durable state construction.
- `packages/open-bitcoin-node/src/sync/session.rs` - Moved session helper methods out of the root sync runtime file.
- `packages/open-bitcoin-node/src/sync/types/error.rs` - Moved sync runtime error definitions and conversions out of the root types file.
- `packages/open-bitcoin-node/src/sync/tests.rs` - Added required Phase 70 deterministic branch/reorg and storage-blocker tests.
- `packages/open-bitcoin-node/src/storage/fjall_store.rs` - Exposed a test-only raw write helper to crate tests for malformed chainstate coverage.
- `docs/parity/source-breadcrumbs.json` - Added parity breadcrumbs for new first-party Rust modules.
- `docs/metrics/lines-of-code.md` - Regenerated the tracked LOC report.

## Decisions Made

- Captured reconcile progress in `DurableSyncRuntime` instead of trying to thread it through every peer-progress return value.
- Treated `ChainstateError::MissingUndo` during reorg disconnect as storage corruption repair because the active-chain undo evidence is local durable state.
- Kept branch/reorg status bounded to counts, heights, hashes, and persisted verdicts; no raw block or undo payloads are exposed.
- Split modules rather than compressing code to satisfy the production file-length gate.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] File-length hook failure from partial executor work**

- **Found during:** Plan execution handoff
- **Issue:** `sync.rs` and `sync/types.rs` exceeded the production Rust file-length limit after the partial reconcile implementation.
- **Fix:** Extracted sync runtime errors, reconcile status projection, and session helpers into child modules and updated parity breadcrumbs.
- **Files modified:** `packages/open-bitcoin-node/src/sync/types/error.rs`, `packages/open-bitcoin-node/src/sync/reconcile_status.rs`, `packages/open-bitcoin-node/src/sync/session.rs`, `docs/parity/source-breadcrumbs.json`
- **Verification:** `bash scripts/check-file-lengths.sh` and `bash scripts/verify.sh` passed.

**2. [Rule 2 - Missing Critical] Missing undo needed storage-first recovery mapping**

- **Found during:** Storage-blocker test implementation
- **Issue:** Missing undo data from active-chain reorg disconnect would otherwise flow through the generic network error conversion.
- **Fix:** Converted `ChainstateError::MissingUndo` at the reorg boundary into `SyncRuntimeError::Storage(StorageError::Corruption { namespace: Chainstate, action: Repair })`.
- **Files modified:** `packages/open-bitcoin-node/src/sync/block_reconcile.rs`
- **Verification:** `phase70_missing_undo_data_is_storage_blocker` and full verifier passed.

**Total deviations:** 2 auto-fixed
**Impact on plan:** Scope stayed within REC-01 and REC-02. The extra modules are structural splits needed by the repo quality gates.

## Issues Encountered

- `bash scripts/verify.sh` initially failed because `docs/metrics/lines-of-code.md` was stale after adding Rust modules. Regenerated it with `bun run scripts/generate-loc-report.ts --source=worktree --output=docs/metrics/lines-of-code.md`, then reran verification successfully.

## Verification

- `cargo fmt --manifest-path packages/Cargo.toml --all`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase70_ --all-features`
- `cargo clippy --manifest-path packages/Cargo.toml --all-targets --all-features -- -D warnings`
- `cargo build --manifest-path packages/Cargo.toml --all-targets --all-features`
- `cargo test --manifest-path packages/Cargo.toml --all-features`
- `bash scripts/verify.sh`

## User Setup Required

None - all coverage is deterministic and local.

## Next Phase Readiness

Phase 70-03 can build peer attribution, stale in-flight release, and rotation behavior on top of typed branch/reorg reconcile outcomes.

## Self-Check: PASSED

- Summary file exists.
- Internal reconcile enum exists in `sync/types.rs`.
- Required Phase 70 branch/reorg and storage-blocker tests pass.
- Full repo verification passed.

*Phase: 70-reorg-peer-rotation-and-no-progress-recovery*
*Completed: 2026-06-12*
