---
phase: 135-snapshot-schema-checkpointing-and-recovery
plan: "01"
subsystem: storage
tags: [rust, mempool, snapshot, serde, fjall, recovery]
requires:
  - phase: 134-authoritative-cross-cache-lifecycle-integration
    provides: Authoritative mempool generations, unbroadcast membership, and prepared snapshot capability
provides:
  - Invariant-bearing mempool-local v2 source envelope
  - Bounded fail-closed v2 decoder and exact v1 migration
  - Canonical transaction identity validation with low-cardinality failures
affects: [135-02, 135-03, 135-04, 135-05, 135-06]
tech-stack:
  added: []
  patterns: [local payload versioning, bounded deserialization, source-only persistence]
key-files:
  created:
    - packages/open-bitcoin-node/src/storage/mempool_snapshot/tests.rs
    - packages/open-bitcoin-node/src/storage/snapshot_codec/tests/mempool_limits.rs
  modified:
    - packages/open-bitcoin-node/src/storage/mempool_snapshot.rs
    - packages/open-bitcoin-node/src/storage/snapshot_codec/mempool.rs
    - packages/open-bitcoin-node/src/storage/snapshot_codec/tests.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/tests.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - "Keep SchemaVersion::CURRENT at 1 and place the v2 discriminator inside only the mempool payload."
  - "Persist only witness transactions, acceptance time, capture provenance, and exact unbroadcast identities; recompute or discard compatibility authority."
  - "Retain exact v1 encoding only for the explicitly typed LegacyV1 capture facade until Plan 04 supplies production v2 provenance."
patterns-established:
  - "Decode bounds run before transaction parsing, with byte, record, per-transaction, total-transaction, and unbroadcast caps."
  - "Snapshot failures expose stable low-cardinality classes and never include raw persisted bytes."
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 135-2026-08-02T17-41-48
generated_at: 2026-08-02T20:24:37Z
duration: 42min
completed: 2026-08-02
---

# Phase 135 Plan 01: Source-Only Snapshot Schema Summary

**Mempool-local v2 persistence now stores canonical source truth with bounded fail-closed decoding and an explicit legacy v1 migration.**

## Performance

- **Duration:** 42 min
- **Started:** 2026-08-02T19:42:39Z
- **Completed:** 2026-08-02T20:24:37Z
- **Tasks:** 2
- **Files modified:** 11

## Accomplishments

- Added an invariant-bearing v2 envelope with capture generation/time, known acceptance times, and an exact bounded unbroadcast identity set.
- Added local v2 DTOs inside the unchanged global schema v1 wrapper; current snapshots serialize no fee, virtual size, topology, rolling state, origin, or relay metadata.
- Added an exact, deny-unknown-fields v1 migration that verifies txid/wtxid, discards fee/vsize/origin/relay authority, preserves known or explicitly unknown age, and restores no unbroadcast members.
- Added byte, record, unbroadcast, per-transaction, and total transaction-byte limits plus low-cardinality whole-snapshot failures.
- Added deterministic tests for round trips, legacy migration, truncation, unknown versions, bounds, duplicates, identity mismatch, future acceptance time, and foreign unbroadcast identities.

## Task Commits

Each task was committed atomically:

1. **Task 1: Define the invariant-bearing source envelope and failure vocabulary** - `7c90625e` (feat)
2. **Task 2: Implement bounded v2 encoding and explicit v1 migration** - `8e38bfb0` (feat)

## Files Created/Modified

- `packages/open-bitcoin-node/src/storage/mempool_snapshot.rs` - Versioned domain envelope, provenance, invariants, bounds, and typed failures.
- `packages/open-bitcoin-node/src/storage/mempool_snapshot/tests.rs` - Extracted domain and replay tests with Phase 135 invariant coverage.
- `packages/open-bitcoin-node/src/storage/snapshot_codec/mempool.rs` - Source-only v2 codec, exact v1 migration, canonical identity checks, and decode limits.
- `packages/open-bitcoin-node/src/storage/snapshot_codec/tests.rs` - Current-v2 and legacy-v1 fixtures and round-trip assertions.
- `packages/open-bitcoin-node/src/storage/snapshot_codec/tests/mempool_limits.rs` - Focused malformed, bounded, identity, and transitional compatibility tests.
- `packages/open-bitcoin-node/src/storage/fjall_store/tests.rs` - Deterministic current-v2 persistence fixture.
- `packages/open-bitcoin-node/src/network/tests/recovery_cases.rs` - Checked compatibility record fixtures.
- `packages/open-bitcoin-node/src/network/tests/mempool_lifecycle_cases.rs` - Checked compatibility record fixtures.
- `docs/parity/source-breadcrumbs.json` - Registered new mempool snapshot test modules against pinned Knots anchors.
- `docs/metrics/lines-of-code.md` - Refreshed tracked generated LOC evidence.

## Decisions Made

- Kept the global storage wrapper at schema version 1; only the mempool payload carries format version 2.
- Made canonical witness transaction bytes the identity source and treated v1 fee/vsize/origin/relay values as non-authoritative compatibility inputs.
- Required current acceptance time to be known and no later than capture time.
- Preserved a narrow `LegacyV1` encoding facade because Plan 01 intentionally cannot fabricate generation/time/unbroadcast provenance and Plan 04 owns production capture migration. Invariant-bearing current snapshots always encode v2.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Extracted snapshot tests to satisfy the managed file-length gate**

- **Found during:** Task 1 and Task 2 verification
- **Issue:** The domain and codec test files exceeded the 628-line Bright Builds limit after adding required coverage.
- **Fix:** Extracted focused tests into `mempool_snapshot/tests.rs` and `snapshot_codec/tests/mempool_limits.rs`, registered exact Knots breadcrumbs, and refreshed LOC evidence.
- **Files modified:** Domain/codec test modules, `docs/parity/source-breadcrumbs.json`, `docs/metrics/lines-of-code.md`
- **Verification:** `bun scripts/bright-builds-check.ts all` and `bun run scripts/check-parity-breadcrumbs.ts --check` pass.
- **Committed in:** `7c90625e`, `8e38bfb0`

**2. [Rule 3 - Blocking] Preserved an exact typed legacy encoder until authoritative capture migration**

- **Found during:** Task 2 implementation
- **Issue:** The plan required all encoding to be v2 but also prohibited fabricating provenance and deferred the only production capture migration to Plan 04.
- **Fix:** Current invariant-bearing snapshots always encode v2; only explicit `LegacyV1` snapshots retain the exact prior v1 encoding path until Plan 04 removes it.
- **Files modified:** `packages/open-bitcoin-node/src/storage/snapshot_codec/mempool.rs`, codec tests
- **Verification:** Dedicated tests distinguish v2 and legacy output shapes; all lifecycle persistence tests and full verification pass.
- **Committed in:** `8e38bfb0`

**3. [Rule 1 - Bug] Migrated Fjall equality fixture to current source truth**

- **Found during:** Task 2 envelope-suite verification
- **Issue:** Three persistence tests expected discarded legacy fee authority to round-trip unchanged.
- **Fix:** Converted the shared fixture to deterministic current-v2 provenance and source-only compatibility values.
- **Files modified:** `packages/open-bitcoin-node/src/storage/fjall_store/tests.rs`
- **Verification:** All six filtered Fjall snapshot persistence cases pass within the envelope suite.
- **Committed in:** `8e38bfb0`

**4. [Rule 3 - Blocking] Retained the Phase 130 partial-metadata structural anchor**

- **Found during:** Task 2 full verification
- **Issue:** Low-cardinality error mapping removed a historical structural-checker phrase even though partial metadata still failed closed.
- **Fix:** Restored the compatibility anchor at the typed rejection branch without exposing detailed runtime diagnostics.
- **Files modified:** `packages/open-bitcoin-node/src/storage/snapshot_codec/mempool.rs`
- **Verification:** All 25 Phase 130 checker mutation tests pass.
- **Committed in:** `8e38bfb0`

**Total deviations:** 4 auto-fixed (1 bug, 3 blocking)

**Impact on plan:** All changes preserve the locked source-only and fail-closed contract. The only temporary surface is the narrowly typed legacy encoder scheduled for removal by Plan 04.

## Issues Encountered

- The Task 2 read-first path `packages/open-bitcoin-core/src/consensus.rs` does not exist; `open-bitcoin-core/src/lib.rs` reexports the dedicated `open-bitcoin-consensus` crate, so canonical transaction functions were resolved through that established interface.
- TDD RED runs failed on the intended missing envelope/limits APIs. Separate failing-test commits were not created because the mandatory normal pre-commit hook runs the full repository and the user explicitly prohibited bypassing it; each task instead landed as one verified atomic green commit.

## Verification

- `phase135-01-envelope`: 37 passed, 0 failed (includes domain, codec, and Fjall snapshot matches).
- `phase135-01-codec -- --test-threads=1`: 27 passed, 0 failed.
- `bun scripts/bright-builds-check.ts all`: zero findings.
- `bun run scripts/check-parity-breadcrumbs.ts --check`: 738 Rust files verified.
- `bash scripts/verify.sh`: passed after the final implementation change in 4m26s; normal commit hooks passed the same full contract for both task commits.
- `git diff --check`: passed.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plans 02 and 03 can consume the validated canonical record and envelope contracts.
- Plan 04 must replace the sole production `LegacyV1` capture with explicit authority-owned generation, capture time, and unbroadcast membership, then remove the transitional legacy encoder.
- No blocker remains for subsequent Phase 135 plans.

## Self-Check: PASSED

- Created files exist.
- Task commits `7c90625e` and `8e38bfb0` are present in repository history.
- Summary frontmatter has exactly one opening and closing delimiter pair.

*Phase: 135-snapshot-schema-checkpointing-and-recovery*
*Completed: 2026-08-02*
