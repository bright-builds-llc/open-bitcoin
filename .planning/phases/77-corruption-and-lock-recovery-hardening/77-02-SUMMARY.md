---
phase: 77-corruption-and-lock-recovery-hardening
plan: 02
subsystem: recovery
tags: [rust, fjall, storage, lock-probe, recovery]

requires:
  - phase: 77-corruption-and-lock-recovery-hardening
    provides: Plan 77-01 recovery evidence DTOs and pure classifier contract
provides:
  - Probe-only Fjall lock evidence collector
  - Typed Fjall lock-open failure mapping
  - Deterministic recovery fixtures for lock, stale lock, schema, corruption, partial write, and backend open failures
affects:
  - storage
  - recovery
  - status
  - support
  - parity-breadcrumbs

tech-stack:
  added: []
  patterns:
    - Probe-only filesystem evidence uses metadata, `File::open`, advisory `try_lock`, and `unlock` without opening or mutating the Fjall store.
    - Real Fjall adapter lock failures map typed `fjall::Error::Locked` before falling back to backend message classification.

key-files:
  created:
    - packages/open-bitcoin-node/src/storage/lock_probe.rs
  modified:
    - packages/open-bitcoin-node/src/storage.rs
    - packages/open-bitcoin-node/src/storage/fjall_store.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/tests.rs
    - docs/parity/source-breadcrumbs.json

key-decisions:
  - "Implemented `probe_fjall_lock` as a read-only filesystem/advisory-lock probe with explicit unavailable evidence instead of opening Fjall."
  - "Kept stale lock evidence as an unlocked persistent lock artifact, not proof of owner death or permission to delete lock files."
  - "Mapped typed `fjall::Error::Locked` to `database locked by another process` with restart guidance while preserving existing storage-pressure fallback mapping."

patterns-established:
  - "Lock evidence details are stable strings consumed by deterministic tests and later status/support projections."
  - "Fjall recovery fixtures use tiny temp datadirs and real adapter behavior without public network, service managers, process scans, sleeps, or large disk allocation."

requirements-completed: [REC-05, REC-06, REC-08]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 77-2026-06-15T18-33-03
generated_at: 2026-06-15T22:28:41Z

duration: 9 min
completed: 2026-06-15
---

# Phase 77 Plan 02: Probe-Only Fjall Lock Evidence and Backend Mapping Summary

**Fjall lock diagnosis now separates absent, stale, active-contention, and unavailable evidence without opening or mutating the store, while real lock-open failures map through typed recovery evidence.**

## Performance

- **Duration:** 9 min
- **Started:** 2026-06-15T22:19:56Z
- **Completed:** 2026-06-15T22:28:41Z
- **Tasks:** 2
- **Files modified:** 5

## Accomplishments

- Added `probe_fjall_lock` and `FJALL_LOCK_FILE_NAME` under `packages/open-bitcoin-node/src/storage/lock_probe.rs`.
- Re-exported the probe from `storage.rs` and added a parity breadcrumb for the new Rust source file.
- Added lock probe tests for missing datadir, no lock artifact, stale lock artifact, and active Fjall lock contention.
- Matched `fjall::Error::Locked` explicitly so real second-open failures produce stable lock-contention recovery evidence.
- Added deterministic `fjall_recovery_evidence_` fixtures for lock contention, path-as-file backend open failure, schema mismatch, corruption marker, and partial write classifier causes.

## Task Commits

1. **Task 1 RED: lock probe evidence tests** - `b390805` (test)
2. **Task 1 GREEN: probe-only lock evidence** - `cdc16a3` (feat)
3. **Task 2 RED: Fjall recovery evidence fixtures** - `1d6eeec` (test)
4. **Task 2 GREEN: typed Fjall lock mapping** - `1017a1e` (fix)

## Files Created/Modified

- `packages/open-bitcoin-node/src/storage/lock_probe.rs` - Probe-only Fjall lock evidence collector using metadata and advisory locking only.
- `packages/open-bitcoin-node/src/storage.rs` - Re-exported the lock probe and lock filename constant.
- `packages/open-bitcoin-node/src/storage/fjall_store.rs` - Added explicit typed mapping for `fjall::Error::Locked`.
- `packages/open-bitcoin-node/src/storage/fjall_store/tests.rs` - Added lock probe and real Fjall recovery evidence fixtures.
- `docs/parity/source-breadcrumbs.json` - Added the new lock probe source file to the storage contract group with no Knots source anchor.

## Decisions Made

- The probe returns `FieldAvailability::Unavailable` for missing or non-directory datadirs, and available `LockEvidence` for no-artifact, stale-artifact, active-contention, and probe-unavailable lock-file cases.
- The probe does not call `FjallNodeStore::open`, `Database::builder`, create, delete, clear, repair, reindex, relocate, or scan processes.
- Backend errors other than typed `Locked` still use `StorageRecoveryAction::for_backend_message` so Phase 76 storage-pressure mapping remains intact.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

- The first Task 2 RED draft used `expect_err`, which required `FjallNodeStore: Debug` and failed for the wrong reason. The test was corrected before the RED commit, then failed as intended on the raw `FjallError: Locked` message.

## Known Stubs

None.

## Authentication Gates

None.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib lock_probe_ --all-features`
- `bun run scripts/check-parity-breadcrumbs.ts --check`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib fjall_recovery_evidence_ --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib storage_recovery_category_maps_schema_corruption_lock_and_backend_states --all-features`

## Diff Review

- `probe_fjall_lock` negative scan found no `FjallNodeStore::open`, `Database::builder`, create/delete, marker clear/write, repair, reindex, relocation, public-network, service-manager, or process-scan behavior.
- Positive probe scan found only `fs::metadata`, `File::open`, `try_lock`, and `unlock` as the lock evidence primitives.
- Production file line counts stayed below the repo trigger: `storage.rs` 502, `lock_probe.rs` 103, `fjall_store.rs` 599.
- Repo-wide `bash scripts/verify.sh` was not run here per serialized wave instructions; this plan ran the required focused verification commands and the orchestrator owns final full verification.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Ready for Plan 77-03 to project the shared recovery evidence into status collection without opening Fjall stores from probe-only inspection paths.

## Self-Check: PASSED

- Summary file exists.
- Created lock probe module exists.
- Task commits found: `b390805`, `cdc16a3`, `1d6eeec`, `1017a1e`.
- No failed self-check marker remains.

---
*Phase: 77-corruption-and-lock-recovery-hardening*
*Completed: 2026-06-15*
