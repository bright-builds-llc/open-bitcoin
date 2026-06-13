---
phase: 71-resource-bounds-and-durable-restart-resume
plan: 03
subsystem: storage-recovery
tags: [storage, recovery-category, resource-exhaustion, no-progress]

requires: [71-CONTEXT, 71-RESEARCH]
provides:
  - `StorageRecoveryAction::FreeDisk`
  - Backend storage-pressure detection through `for_backend_message`
  - Error-detail mapping from low disk/storage pressure to `resource_exhaustion`
  - No-progress guidance that includes freeing disk space
affects: [phase-71, storage, sync-recovery, no-progress]

tech-stack:
  added: []
  patterns:
    - Storage backend messages choose typed recovery actions before renderer guidance
    - Low disk and storage pressure map to existing `resource_exhaustion`
    - Storage/resource no-progress guidance stays in shared sync progress code

key-files:
  created:
    - .planning/phases/71-resource-bounds-and-durable-restart-resume/71-03-SUMMARY.md
  modified:
    - packages/open-bitcoin-node/src/storage.rs
    - packages/open-bitcoin-node/src/storage/fjall_store.rs
    - packages/open-bitcoin-node/src/sync/types/recovery.rs
    - packages/open-bitcoin-node/src/sync/progress.rs

key-decisions:
  - "Represent low disk/storage pressure as `StorageRecoveryAction::FreeDisk` mapped to `SyncRecoveryCategory::ResourceExhaustion`."
  - "Preserve existing schema mismatch, corruption, lock contention, and backend failure precedence."

patterns-established:
  - "`StorageRecoveryAction::for_backend_message` detects storage-pressure phrases and otherwise keeps restart guidance."
  - "`recovery_category_from_error_detail` detects low disk/storage pressure before generic backend failure."

requirements-completed: [RES-03]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 71-2026-06-13T10-34-37
generated_at: 2026-06-13T11:23:00Z

duration: 25min
completed: 2026-06-13
---

# Phase 71 Plan 03: Storage-Pressure Recovery Guidance Summary

**Low disk and storage pressure now produce typed `FreeDisk`/`resource_exhaustion` guidance while storage-first recovery precedence remains intact.**

## Performance

- **Duration:** 25 min
- **Started:** 2026-06-13T10:58:00Z
- **Completed:** 2026-06-13T11:23:00Z
- **Tasks:** 2
- **Files modified:** 4
- **Files created:** 1

## Accomplishments

- Added `StorageRecoveryAction::FreeDisk`, serialized as `free_disk`.
- Mapped `FreeDisk` to `SyncRecoveryCategory::ResourceExhaustion`.
- Added exact operator guidance: `Free disk space for the selected datadir, then retry sync.`
- Added `StorageRecoveryAction::for_backend_message` for `no space left on device`, `ENOSPC`, `disk full`, `low disk`, and `storage pressure`.
- Updated Fjall backend errors to choose recovery action from the backend message.
- Updated sync recovery-detail classification so storage-pressure detail maps to `ResourceExhaustion` before generic backend failure.
- Updated storage/resource no-progress guidance to include freeing disk.

## Task Commits

Task commits are pending the wrapper-owned final commit after full phase verification.

1. **Task 1: Storage FreeDisk action and Fjall backend mapping** - `pending final wrapper commit`
2. **Task 2: Shared recovery-detail and no-progress guidance** - `pending final wrapper commit`

## Files Created/Modified

- `packages/open-bitcoin-node/src/storage.rs` - Added `FreeDisk`, `for_backend_message`, pressure-signal detection, and regression coverage.
- `packages/open-bitcoin-node/src/storage/fjall_store.rs` - Routed backend failures through `StorageRecoveryAction::for_backend_message`.
- `packages/open-bitcoin-node/src/sync/types/recovery.rs` - Added low-disk/storage-pressure recovery-detail classification.
- `packages/open-bitcoin-node/src/sync/progress.rs` - Updated shared storage/resource no-progress next action.

## Decisions Made

- Reused the existing `resource_exhaustion` category for low disk and storage pressure to avoid unnecessary stable-label expansion.
- Kept precise free-disk guidance in typed recovery/action code instead of renderer-local strings.

## Deviations from Plan

None.

## Issues Encountered

- None after local takeover. Targeted storage and recovery tests passed.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node storage_recovery_category_maps_low_disk_and_storage_pressure --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node sync_recovery_category_from_error_detail_maps_known_detail_facts --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node sync::progress::tests --all-features`
- Acceptance `rg` checks for `FreeDisk`, `free_disk`, `for_backend_message`, low-disk/storage-pressure phrases, Fjall backend action selection, `ResourceExhaustion`, and shared no-progress free-disk guidance.

## User Setup Required

None - all checks are deterministic and local.

## Next Phase Readiness

Plan 71-04 can document and checker-gate `StorageRecoveryAction::FreeDisk`, exact guidance text, and storage-pressure default-verification coverage.

## Self-Check: PASSED

- Summary file exists.
- Required Plan 03 mapping and guidance tests pass.
- No hidden storage mutation, repair, migration, or public-network scope was added.

*Phase: 71-resource-bounds-and-durable-restart-resume*
*Completed: 2026-06-13*
