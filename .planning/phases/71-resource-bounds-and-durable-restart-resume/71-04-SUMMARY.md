---
phase: 71-resource-bounds-and-durable-restart-resume
plan: 04
subsystem: docs-verification
tags: [operator-docs, architecture-docs, parity-docs, verification, resource-bounds]

requires: [71-01, 71-02, 71-03, 71-CONTEXT, 71-RESEARCH]
provides:
  - Phase 71 operator resource and restart/resume documentation
  - Architecture documentation for resource pressure, restart/resume, and storage-pressure recovery contracts
  - Parity boundary documentation for outbound opt-in full-sync review
  - Deterministic Phase 71 checker wired into repo-native verification
affects: [phase-71, docs, scripts, default-verification]

tech-stack:
  added: []
  patterns:
    - Bun TypeScript source/docs/test checker matching Phase 70 checker style
    - Verification wiring after the previous phase checker
    - Tracked LOC freshness update after adding the checker

key-files:
  created:
    - scripts/check-phase71-resource-restart.ts
    - .planning/phases/71-resource-bounds-and-durable-restart-resume/71-04-SUMMARY.md
  modified:
    - docs/operator/runtime-guide.md
    - docs/architecture/status-snapshot.md
    - docs/architecture/operator-observability.md
    - docs/architecture/storage-decision.md
    - docs/parity/catalog/p2p.md
    - docs/parity/catalog/chainstate.md
    - scripts/verify.sh
    - docs/metrics/lines-of-code.md
    - packages/open-bitcoin-node/src/sync/tests.rs

key-decisions:
  - "Keep Phase 71 default verification deterministic and public-network-free."
  - "Document storage pressure as diagnosis only, with no automatic datadir mutation."
  - "Preserve the outbound opt-in full-sync review boundary and avoid production-node claims."

patterns-established:
  - "Phase-specific checkers require exact plan, source, test, doc, and verify-script needles."
  - "Phase documentation names deterministic test anchors for resource/restart claims."

requirements-completed: [RES-01, RES-02, RES-03, RES-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 71-2026-06-13T10-34-37
generated_at: 2026-06-13T12:56:37Z

duration: 70min
completed: 2026-06-13
---

# Phase 71 Plan 04: Documentation and Default Verification

**Phase 71 now has documented resource/restart evidence and a deterministic checker in the repo-native verification contract.**

## Performance

- **Duration:** 70 min
- **Started:** 2026-06-13T11:46:00Z
- **Completed:** 2026-06-13T12:56:37Z
- **Tasks:** 3
- **Files modified:** 9
- **Files created:** 2

## Accomplishments

- Added `### Phase 71 resource bounds and restart/resume proof` to the operator runtime guide with exact resource-bound, same-datadir resume, test-anchor, and free-disk guidance.
- Updated architecture docs to name `SyncResourcePressure`, `SyncRecoveryCategory::ResourceExhaustion`, `StorageRecoveryAction::FreeDisk`, `MetricRetentionPolicy`, and `LogRetentionPolicy`.
- Updated parity docs to keep Phase 71 scoped to outbound explicit opt-in full-sync review and list deferred production surfaces.
- Added `scripts/check-phase71-resource-restart.ts` with deterministic plan/source/test/doc/verify-script assertions.
- Wired the Phase 71 checker after the Phase 70 checker in `scripts/verify.sh`.
- Refreshed `docs/metrics/lines-of-code.md`.
- Updated the Phase 70 storage/resource blocker assertion to match the new explicit free-disk operator action.

## Task Commits

Task commits are pending the wrapper-owned final commit after full phase verification.

1. **Task 1: Document Phase 71 resource, restart, and storage-pressure evidence** - `pending final wrapper commit`
2. **Task 2: Add the deterministic Phase 71 checker** - `pending final wrapper commit`
3. **Task 3: Wire checker into repo verification and refresh generated LOC** - `pending final wrapper commit`

## Files Created/Modified

- `docs/operator/runtime-guide.md` - Added Phase 71 operator proof and recovery guidance.
- `docs/architecture/status-snapshot.md` - Documented Phase 71 status/test anchors and storage-pressure category/action.
- `docs/architecture/operator-observability.md` - Documented retention-policy names and compact resource vocabulary.
- `docs/architecture/storage-decision.md` - Added storage-pressure evidence contract.
- `docs/parity/catalog/p2p.md` - Added Phase 71 outbound opt-in boundary and deferred surfaces.
- `docs/parity/catalog/chainstate.md` - Added storage-pressure parity claim and deferred surfaces.
- `scripts/check-phase71-resource-restart.ts` - Added deterministic Phase 71 checker.
- `scripts/verify.sh` - Wired Phase 71 checker after Phase 70.
- `docs/metrics/lines-of-code.md` - Refreshed generated LOC artifact.
- `packages/open-bitcoin-node/src/sync/tests.rs` - Updated storage/resource blocker expectation to the new guidance.

## Decisions Made

- Reused the Phase 70 checker structure for Phase 71 so default verification stays a simple local text/source contract.
- Kept public-network smoke, manual peers, restart-after-progress, and service-manager commands out of `scripts/verify.sh`.
- Treated `StorageRecoveryAction::FreeDisk` as operator guidance only, not an automatic repair or mutation path.

## Deviations from Plan

None.

## Issues Encountered

- The first full verification run failed the file-length gate because a new inline CLI test pushed `operator/runtime/support.rs` over 628 lines. The test was compacted to assert the formatter directly, and the file-length gate passed.
- The second full verification run exposed a stale Phase 70 expected string for storage/resource blockers. The assertion now matches the explicit free-disk guidance.

## Verification

- `rg` acceptance checks for operator, architecture, parity, checker, verify-script, and LOC needles
- `bun --check scripts/check-phase71-resource-restart.ts`
- `bun run scripts/check-phase71-resource-restart.ts`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli phase71_ --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase71_ --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node storage_recovery_category_maps_low_disk_and_storage_pressure --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node sync_recovery_category --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase70_no_progress_status_projects_storage_or_resource_blocker --all-features`
- `bash scripts/verify.sh`

## User Setup Required

None - all checks are deterministic and local.

## Next Phase Readiness

Phase 71 has complete docs, checker wiring, and default verification evidence for RES-01 through RES-04.

## Self-Check: PASSED

- Summary file exists.
- Phase 71 checker runs in default verification after Phase 70.
- Full `bash scripts/verify.sh` passed.
- No public-network, service-manager, production-wallet, migration-apply, packaging, GUI, hosted-dashboard, or broad production-node claim was added.

*Phase: 71-resource-bounds-and-durable-restart-resume*
*Completed: 2026-06-13*
