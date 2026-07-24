---
phase: 130-resource-time-and-fee-primitives
plan: "08"
subsystem: storage-recovery
tags: [rust, mempool, snapshot, recovery, metadata, serde]
requires:
  - phase: 130-resource-time-and-fee-primitives
    provides: Canonical MempoolEntryMetadata and AdmissionContext constructors from Plans 130-03 through 130-07
provides:
  - MempoolSnapshotRecord carrying exact MempoolEntryMetadata through capture and recovery replay
  - AdmissionContext::recovery for durable restore without clock or origin substitution
  - All-or-none optional wire fields for accepted_at/origin/relay without a global schema bump
  - Fail-closed legacy decode and typed mempool corruption for partial or invalid metadata
affects: [phase-135, durable-recovery, mempool-snapshot, FEEP-03, FEEP-05]
tech-stack:
  added: []
  patterns:
    - All-or-none optional serde DTO fields with maybe_ internals and stable external names
    - Managed recovery consumes MempoolTransition deltas then seeds no-socket fanout
key-files:
  created:
    - packages/open-bitcoin-node/src/storage/snapshot_codec/mempool.rs
  modified:
    - packages/open-bitcoin-mempool/src/context.rs
    - packages/open-bitcoin-node/src/storage/mempool_snapshot.rs
    - packages/open-bitcoin-node/src/network/recovery.rs
    - packages/open-bitcoin-node/src/storage/snapshot_codec.rs
    - packages/open-bitcoin-node/src/storage/snapshot_codec/tests.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - "Keep SchemaVersion::CURRENT unchanged and encode metadata as three optional mempool-record fields."
  - "All-absent decodes to LegacyUnknown, RecoveryUnknown, and NotRequested; any partial set is StorageError::Corruption in Mempool."
  - "Known capture and recovery pass metadata through AdmissionContext::recovery without substituting restart time or local origin."
patterns-established:
  - "Durable mempool metadata is atomic: encode all three fields or omit all three."
  - "Legacy recovery remains retry-ineligible until explicit known facts are present."
requirements-completed: []
requirements-addressed: [FEEP-03, FEEP-05]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 130-2026-07-23T14-26-46
generated_at: 2026-07-24T03:01:25Z
duration: 61 min
completed: 2026-07-24
---

# Phase 130 Plan 08: Legacy Snapshot Compatibility Summary

**Legacy and known mempool snapshots now preserve truthful acceptance time, origin, and relay intent across capture and recovery without bumping the repository-wide schema**

## Performance

- **Duration:** 61 min
- **Started:** 2026-07-24T02:00:22Z
- **Completed:** 2026-07-24T03:01:25Z
- **Tasks:** 2
- **Files modified:** 12

## Accomplishments

- Added `metadata: MempoolEntryMetadata` to snapshot records and copied exact entry facts in `from_mempool`.
- Replayed recovery through `AdmissionContext::recovery` and consumed transition deltas in managed recovery while retaining typed drop classifications and no-socket fanout seeding.
- Implemented all-or-none optional wire fields (`accepted_at_unix_seconds`, `origin`, `relay_requested`) with fail-closed legacy decode and typed corruption for partial or invalid metadata.
- Kept `SchemaVersion::CURRENT` at `1` and proved literal pre-Phase-130 JSON still decodes as retry-ineligible legacy metadata.
- Passed focused snapshot/recovery suites, the exact timed workspace all-target gate at both task boundaries, full normal-hook verification, and final plan verification.

## TDD Execution

- **Task 1 RED:** Recovery metadata tests failed while capture still forced legacy-unknown and replay still used `AdmissionContext::legacy_unknown()`; intentional RED could not commit through hook-owned full verify.
- **Task 1 GREEN:** Capture copied exact metadata, replay used `AdmissionContext::recovery`, and managed recovery applied deltas plus fanout seeding.
- **Task 2 RED/GREEN:** Codec tests for known round-trip, legacy fail-closed decode, partial corruption, invalid origin, and unchanged schema were implemented with the all-or-none decoder and landed with the Task 2 feat commit.

## Task Commits

1. **Task 1: Preserve metadata through snapshot capture and replay** - `def8a2df` (feat)
2. **Task 2: Add all-or-none backward-compatible metadata decoding** - `60abc7f3` (feat)

**Plan metadata:** `21535884` (docs: complete plan)

## Files Created/Modified

- `packages/open-bitcoin-mempool/src/context.rs` - Added `AdmissionContext::recovery`.
- `packages/open-bitcoin-node/src/storage/mempool_snapshot.rs` - Snapshot records carry metadata; capture/replay preserve known or fail-closed legacy facts.
- `packages/open-bitcoin-node/src/network/recovery.rs` - Managed recovery uses recovery context and transition deltas.
- `packages/open-bitcoin-node/src/storage/snapshot_codec/mempool.rs` - Mempool DTO encode/decode with all-or-none optional metadata.
- `packages/open-bitcoin-node/src/storage/snapshot_codec.rs` - Re-exports mempool codec after file-length split.
- `packages/open-bitcoin-node/src/storage/snapshot_codec/tests.rs` - Known round-trip, legacy, partial, invalid-origin, and schema-stability tests.
- `docs/parity/source-breadcrumbs.json` - Registered `snapshot_codec/mempool.rs` under node-mempool-storage.

## Decisions Made

- Keep the repository-wide schema version unchanged and attach optional per-record metadata fields instead.
- Classify missing legacy metadata only as `LegacyUnknown` + `RecoveryUnknown` + `NotRequested`; never infer local origin or restart time.
- Reject every partial metadata combination and invalid origin label as mempool namespace corruption before mempool mutation.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - Missing critical functionality] Cover `AdmissionContext::recovery` in mempool-crate tests**
- **Found during:** Task 1 commit
- **Issue:** Hook coverage flagged uncovered `recovery` lines because node tests do not contribute to mempool crate coverage.
- **Fix:** Extended `admission_context_constructors_map_trusted_source_facts` to exercise `AdmissionContext::recovery`.
- **Files modified:** `packages/open-bitcoin-mempool/src/pool/tests/context_cases.rs`
- **Verification:** Coverage gate passed in Task 1 hook verify.
- **Committed in:** `def8a2df`

**2. [Rule 3 - Blocking issue] Split snapshot codec after 628-line production limit**
- **Found during:** Task 2 commit
- **Issue:** Adding metadata DTO logic pushed `snapshot_codec.rs` to 732 lines.
- **Fix:** Extracted mempool DTO/encode/decode into `snapshot_codec/mempool.rs` and registered parity breadcrumbs.
- **Files modified:** `packages/open-bitcoin-node/src/storage/snapshot_codec.rs`, `packages/open-bitcoin-node/src/storage/snapshot_codec/mempool.rs`, `docs/parity/source-breadcrumbs.json`
- **Verification:** File-length check and mempool_snapshot suite passed.
- **Committed in:** `60abc7f3`

**3. [Rule 1 - Bug] Clippy `type_complexity` on encode helper return type**
- **Found during:** Task 2 commit
- **Issue:** `Result<(Option<i64>, Option<MempoolOriginDto>, Option<bool>), _>` tripped `-D clippy::type_complexity`.
- **Fix:** Introduced `EncodedEntryMetadataFields` alias.
- **Files modified:** `packages/open-bitcoin-node/src/storage/snapshot_codec/mempool.rs`
- **Verification:** `cargo clippy -p open-bitcoin-node --lib -- -D warnings` and full hook verify passed.
- **Committed in:** `60abc7f3`

---

**Total deviations:** 3 auto-fixed (1 Rule 1, 1 Rule 2, 1 Rule 3)
**Impact on plan:** Required for coverage, production file-length, and clippy gates. No scope creep into Phase 135 topology/checkpoint work.

## Issues Encountered

- Intentional TDD RED commits cannot land through this repo's hook-owned full verify; RED evidence was retained in local test runs and documented here.
- Plan acceptance grep path named `snapshot_codec.rs` for optional fields; after the file-length split the fields live in `snapshot_codec/mempool.rs` within the same module.

## Next Phase Readiness

- Phase 135 can add mempool-local versioning, topological replay, checkpoint cadence, and crash-window semantics while preserving this truthful legacy classification.
- FEEP-03 and recovery-related FEEP-05 are truthful for current durable snapshots under the no-global-schema-bump decision.

## Self-Check: PASSED

- FOUND: `.planning/phases/130-resource-time-and-fee-primitives/130-08-SUMMARY.md`
- FOUND: `def8a2df`
- FOUND: `60abc7f3`
