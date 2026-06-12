---
phase: 70-reorg-peer-rotation-and-no-progress-recovery
plan: 01
subsystem: status
tags: [sync-status, reorg, reconcile, serde, fixtures]

requires: []
provides:
  - Additive latest reorg evidence status contract
  - Additive branch/reorg reconcile progress status contract
  - Legacy-safe serde defaults for Phase 70 status fields
  - Downstream CLI, RPC, dashboard, and support fixture compatibility
affects: [phase-70, sync-runtime, operator-status, rpc-status]

tech-stack:
  added: []
  patterns:
    - Shared status structs expose bounded evidence only
    - New status fields use serde defaults for legacy JSON compatibility

key-files:
  created:
    - .planning/phases/70-reorg-peer-rotation-and-no-progress-recovery/70-01-SUMMARY.md
  modified:
    - packages/open-bitcoin-node/src/status.rs
    - packages/open-bitcoin-node/src/status/tests.rs
    - packages/open-bitcoin-node/src/sync/types/summary.rs
    - packages/open-bitcoin-cli/src/operator/status/sync_state.rs
    - packages/open-bitcoin-cli/src/operator/status/tests.rs
    - packages/open-bitcoin-cli/src/operator/status/render/tests.rs
    - packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs
    - packages/open-bitcoin-cli/src/operator/runtime/support.rs
    - packages/open-bitcoin-rpc/src/dispatch/tests.rs
    - docs/metrics/lines-of-code.md

key-decisions:
  - "Expose latest reorg evidence as a single bounded record, not an unbounded history."
  - "Expose reconcile progress as a typed serde enum with bounded hashes and counts only."
  - "Keep Phase 70 unavailable reason strings in the shared status contract so support fixtures do not add renderer-local wording."

patterns-established:
  - "Additive status fields default through FieldAvailability::unavailable for legacy JSON."
  - "Downstream status fixtures compile through shared status additions without owning runtime semantics."

requirements-completed: [REC-02]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 70-2026-06-12T14-56-46
generated_at: 2026-06-12T18:34:13Z

duration: 73min
completed: 2026-06-12
---

# Phase 70 Plan 01: Reorg Status Contract Summary

**Bounded latest reorg evidence and typed reconcile progress status fields with legacy-safe defaults and downstream fixture compatibility**

## Performance

- **Duration:** 73 min
- **Started:** 2026-06-12T17:21:00Z
- **Completed:** 2026-06-12T18:34:13Z
- **Tasks:** 2
- **Files modified:** 10

## Accomplishments

- Added `SyncReorgEvidence` with bounded common-ancestor, disconnect/connect count, final active tip, and persisted verdict fields.
- Added `SyncReconcileProgressStatus` as a tagged serde enum for no-change, active-chain extension, branch body wait, side-branch preservation, and persisted reorg states.
- Added `SyncStatus.latest_reorg` and `SyncStatus.reconcile_progress` with legacy-safe unavailable defaults.
- Updated CLI, RPC, dashboard, support, and sync summary fixtures to compile through the additive fields without adding runtime projection behavior.
- Refreshed the tracked LOC artifact after the status/test additions.

## Task Commits

Each planned task was implemented in the plan commit after executor handoff:

1. **Task 1: Add bounded reorg and reconcile status fields** - `58aed15` (feat)
2. **Task 2: Update downstream status fixtures for additive fields** - `58aed15` (feat)

**Plan metadata:** committed separately by the GSD metadata flow.

## Files Created/Modified

- `packages/open-bitcoin-node/src/status.rs` - Public Phase 70 status contract and default unavailable reasons.
- `packages/open-bitcoin-node/src/status/tests.rs` - Legacy JSON defaulting and bounded serialization tests.
- `packages/open-bitcoin-node/src/sync/types/summary.rs` - Summary projection fixture defaults for additive fields.
- `packages/open-bitcoin-cli/src/operator/status/sync_state.rs` - CLI RPC/unavailable status construction defaults.
- `packages/open-bitcoin-cli/src/operator/status/tests.rs` - Status fixture compatibility updates.
- `packages/open-bitcoin-cli/src/operator/status/render/tests.rs` - Render fixture compatibility updates.
- `packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs` - Dashboard fixture compatibility updates.
- `packages/open-bitcoin-cli/src/operator/runtime/support.rs` - Support fixture compatibility through shared reason constants.
- `packages/open-bitcoin-rpc/src/dispatch/tests.rs` - RPC runtime metadata fixture compatibility updates.
- `docs/metrics/lines-of-code.md` - Regenerated tracked LOC report.

## Decisions Made

- Used a single latest reorg evidence record instead of an unbounded reorg history to keep operator status bounded.
- Used a typed tagged enum for reconcile progress so later runtime work can project states without renderer-local string contracts.
- Added shared unavailable reason constants for Phase 70 reconcile status so support fixtures can preserve the exact value without local wording.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Took over after executor verification stall**
- **Found during:** Plan execution handoff
- **Issue:** The executor completed the diff but did not commit or write the summary because a broad Cargo test command remained active in a long macOS dynamic-loader delay.
- **Fix:** Interrupted the executor, reviewed the diff, reran focused acceptance checks, ran the full workspace Cargo test outside the hook, and retried the implementation commit with a longer hook wait.
- **Files modified:** No additional implementation files beyond the plan scope.
- **Verification:** Focused plan tests passed; full commit hook passed before `58aed15`.
- **Committed in:** `58aed15`

**2. [Rule 2 - Missing Critical] Shared Phase 70 unavailable reason constants**
- **Found during:** Acceptance criteria check for renderer-local wording
- **Issue:** A support fixture needed the exact reconcile unavailable value but the plan also required no local `reconcile progress` wording in support rendering code.
- **Fix:** Added shared status constants and referenced the reconcile constant from the support test fixture.
- **Files modified:** `packages/open-bitcoin-node/src/status.rs`, `packages/open-bitcoin-cli/src/operator/runtime/support.rs`
- **Verification:** `rg` acceptance checks and focused Cargo tests passed.
- **Committed in:** `58aed15`

**Total deviations:** 2 auto-fixed (1 blocking, 1 missing critical)
**Impact on plan:** No scope expansion; both fixes preserved the planned public status contract and kept runtime behavior unchanged.

## Issues Encountered

- Full `cargo test --manifest-path packages/Cargo.toml --workspace --all-features` showed intermittent long macOS dynamic-loader startup delays for test binaries. The exact full workspace test passed when allowed to continue, and the implementation commit hook also passed after a longer wait.

## Verification

- `cargo fmt --manifest-path packages/Cargo.toml --all`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase70_sync_reorg_evidence --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --lib status --all-features`
- `cargo test --manifest-path packages/Cargo.toml --workspace --all-features`
- Commit hook for `58aed15`, which ran `bash scripts/verify.sh`

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Phase 70-02 can project real branch/reorg reconciliation evidence into the additive status fields without changing the public status contract introduced here.

## Self-Check: PASSED

- Summary file exists.
- Implementation commit exists: `58aed15`.
- Key modified files are present on disk.
- Acceptance criteria and focused verification passed.

*Phase: 70-reorg-peer-rotation-and-no-progress-recovery*
*Completed: 2026-06-12*
