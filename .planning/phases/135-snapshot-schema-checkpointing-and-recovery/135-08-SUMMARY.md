---
phase: 135-snapshot-schema-checkpointing-and-recovery
plan: "08"
subsystem: storage-verification
tags: [rust, typescript, snapshot, serde, merkle, mutation-testing]
requires:
  - phase: 135-01
    provides: bounded versioned mempool snapshot codec
  - phase: 135-06
    provides: legacy confirmation migration and startup recovery ordering
  - phase: 135-07
    provides: Phase 135 source-corpus mutation checker
provides:
  - fail-closed allocation-bounded raw JSON key preflight before Serde
  - Merkle-authenticated legacy confirmation evidence migration
  - function-scoped mutation guard for preflight-before-deserializer ordering
affects: [phase-135-verification, MPDUR-01, MPDUR-02]
tech-stack:
  added: []
  patterns:
    - stateful lexical JSON preflight with fixed low-cardinality errors
    - authenticate persisted block bodies against header Merkle commitments before migration
    - body-scoped ordered source invariants with exact mutation diagnostics
key-files:
  created: []
  modified:
    - packages/open-bitcoin-node/src/storage/snapshot_codec/mempool/decode/key_preflight.rs
    - packages/open-bitcoin-node/src/storage/snapshot_codec/tests/mempool_limits.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/mempool.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/tests/snapshot_persistence/confirmation_migration.rs
    - scripts/check-phase135-snapshot-recovery.ts
    - scripts/check-phase135-snapshot-recovery.test.ts
    - docs/metrics/lines-of-code.md
key-decisions:
  - "Classify malformed lexical JSON as structural corruption while retaining a distinct fixed resource-bound diagnostic for oversized raw keys."
  - "Reuse block_merkle_root and reject both mutated trees and header/body commitment mismatches before deriving confirmation counts."
  - "Inspect only decode_bounded_versioned and require the exact preflight call to precede exact Serde construction."
patterns-established:
  - "Pre-Serde trust boundaries are guarded by executable-order mutation tests, not corpus-wide identifier presence."
  - "Migration evidence is authenticated against every commitment available in the persisted source artifact before it can change durable state."
requirements-completed: [MPDUR-01, MPDUR-02]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 135-2026-08-02T17-41-48
generated_at: 2026-08-03T20:26:36Z
duration: 25m
completed: 2026-08-03
---

# Phase 135 Plan 08: Snapshot Durability Gap Closure Summary

Fail-closed JSON key scanning, Merkle-authenticated confirmation migration, and mutation-enforced decoder ordering close the remaining snapshot recovery audit gaps.

## Performance

- **Duration:** 25m
- **Started:** 2026-08-03T20:01:36Z
- **Completed:** 2026-08-03T20:26:36Z
- **Tasks:** 3
- **Files modified:** 7

## Accomplishments

- Replaced the raw-key heuristic with a stateful lexical scanner that rejects malformed escapes and unterminated strings before Serde while bounding raw key bytes without constraining value strings.
- Authenticated legacy active-chain block bodies against their header Merkle roots, rejecting mutated trees and mismatched bodies before confirmation evidence is migrated or persisted.
- Strengthened the Phase 135 checker to require the exact preflight call inside `decode_bounded_versioned` before deserializer construction, with exact single-diagnostic mutations for deletion and reordering.
- Preserved the phase's bounded scope: no schema version change, no parity-status promotion, and no Phase 136-138 implementation claims.

## Task Commits

Each task was committed atomically:

1. **Task 1: Fail closed on malformed raw snapshot keys** - `2d9ac20d` (`fix`)
2. **Task 2: Authenticate migrated confirmation evidence** - `38e70604` (`fix`)
3. **Task 3: Enforce snapshot preflight ordering** - `631ba614` (`fix`)

## Files Created/Modified

- `packages/open-bitcoin-node/src/storage/snapshot_codec/mempool/decode/key_preflight.rs` - Stateful root/object/array lexical scanner with checked cursor movement and fixed diagnostics.
- `packages/open-bitcoin-node/src/storage/snapshot_codec/tests/mempool_limits.rs` - Public decoder regressions for malformed escaped keys and structural truncation classification.
- `packages/open-bitcoin-node/src/storage/fjall_store/mempool.rs` - Header/body Merkle authentication before legacy confirmation counting.
- `packages/open-bitcoin-node/src/storage/fjall_store/tests/snapshot_persistence/confirmation_migration.rs` - Same-header/different-body and mutated-tree persistence regressions.
- `scripts/check-phase135-snapshot-recovery.ts` - Function-scoped exact call-order invariant.
- `scripts/check-phase135-snapshot-recovery.test.ts` - Deleted/reordered preflight mutations with exact bounded-load diagnostics.
- `docs/metrics/lines-of-code.md` - Repository-generated current LOC report.

## Decisions Made

- Structural lexical defects map to the existing structural-corruption class; only actual raw key-size violations map to resource-bound exhaustion.
- Confirmation migration validates header identity, then Merkle commitment and mutation state, then derives transaction confirmation counts. No migrated snapshot is saved when any authentication step fails.
- The source checker uses comment/string-masked function bodies and exact executable markers so helper definitions or mentions elsewhere cannot satisfy the invariant.

## Verification Evidence

- Task 1 RED: five focused malformed-key regressions failed before implementation; GREEN: all five passed.
- Task 1 broader snapshot codec suite: 39 passed, 0 failed.
- Task 2 RED: the two new Merkle-authentication regressions failed while the three existing migration tests passed; GREEN: all five passed.
- Task 3 RED: only the deleted-call and reordered-call mutations failed; GREEN: 52 passed, 0 failed, and the live checker reported the Phase 135 invariants verified.
- Parity breadcrumbs: 761 Rust files verified.
- Bright Builds: 1,041 file-length inputs scanned, 0 findings; lesson audit checks also reported 0 findings.
- Full repository contract: `bash scripts/verify.sh` passed in 12m 45.509s, including repository checkers, formatting, clippy, all-target build/tests, benchmark smoke, Bazel build/provenance, and coverage.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Refreshed the tracked LOC report before the full verifier rerun**

- **Found during:** Final verification
- **Issue:** The first verifier attempt stopped at its freshness gate because source and mutation-test changes made `docs/metrics/lines-of-code.md` stale.
- **Fix:** Ran the verifier's exact repository-owned LOC generation command, then reran the complete verification contract successfully.
- **Files modified:** `docs/metrics/lines-of-code.md`
- **Commit:** `631ba614`

## Issues Encountered

- The initial full-verifier attempt exited before substantive checks on the expected stale-LOC gate. Regeneration resolved the blocker; the subsequent complete run passed.
- A Rust compile during focused migration testing was quiet for several minutes; process liveness confirmed active `rustc` work, so it was polled without interruption and completed successfully.

## Known Stubs

None. All changed runtime and checker paths are wired to their production data sources and execution paths.

## Security and Threat Review

- The raw snapshot decoder now fails closed before allocation-heavy Serde processing on malformed key syntax.
- Legacy confirmation migration no longer trusts a block body based on its storage key and header hash alone.
- No new endpoint, authentication path, file-access pattern, schema version, or external trust boundary was introduced.

## User Setup Required

None - no external service configuration or credentials are required.

## Next Phase Readiness

- Plan 135-08 implementation is complete and ready for independent phase verification.
- Phase status, requirement promotion, roadmap state, and the phase verification artifact remain owned by the parent orchestrator.

## Self-Check: PASSED

All seven modified implementation artifacts and this summary exist, and task commits `2d9ac20d`, `38e70604`, and `631ba614` resolve in repository history. The only empty collection introduced is the test harness's intentional temporary-directory tracker; it is not a runtime or UI stub.
