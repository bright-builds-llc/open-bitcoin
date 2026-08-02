---
phase: 135-snapshot-schema-checkpointing-and-recovery
plan: "02"
subsystem: mempool-recovery
tags: [rust, mempool, recovery, topology, snapshots, parity]
requires:
  - phase: 135-01
    provides: validated source-only v2 mempool snapshot envelope and compatibility decoding
  - phase: 135-04
    provides: authority-owned generation, capture time, trigger, and exact unbroadcast membership
  - phase: 134-authoritative-cross-cache-lifecycle-integration
    provides: authoritative mempool lifecycle, chainstate, policy, and projection boundaries
provides:
  - bounded deterministic parent-before-child snapshot topology
  - side-effect-free staged replay into a fresh mempool candidate
  - final-membership recovery classifications with exact unbroadcast restoration
affects: [135-03, 135-05, mempool-recovery, startup-recovery]
tech-stack:
  added: []
  patterns: [bounded-kahn-topology, side-effect-free-staging, final-membership-classification]
key-files:
  created:
    - packages/open-bitcoin-node/src/network/recovery/topology.rs
    - packages/open-bitcoin-node/src/network/recovery/staging.rs
    - packages/open-bitcoin-node/src/network/tests/recovery_cases/staging.rs
  modified:
    - packages/open-bitcoin-node/src/network/recovery.rs
    - packages/open-bitcoin-node/src/storage/mempool_snapshot.rs
    - packages/open-bitcoin-node/src/storage/snapshot_codec/mempool.rs
    - packages/open-bitcoin-mempool/src/pool.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - "Build recovery edges only for canonical parents present in the validated snapshot; current chainstate remains authoritative for external prevouts."
  - "Classify recovered and evicted records only after expiry and capacity processing establish final staged membership."
  - "Preserve only acceptance time and exact surviving unbroadcast membership while resetting every rolling-fee field to a fresh-pool restart baseline."
patterns-established:
  - "Bounded topology: checked vertex and edge accounting plus lexicographic BTreeSet ordering make adversarial replay deterministic and component-local."
  - "Prepared recovery: a non-Clone opaque candidate contains all final runtime truth without mutating live authority during preparation."
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 135-2026-08-02T17-41-48
generated_at: 2026-08-02T22:37:16Z
duration: 55min
completed: 2026-08-02
---

# Phase 135 Plan 02: Deterministic Recovery Topology and Staged Replay Summary

**Bounded parent-before-child replay now produces one isolated fresh mempool candidate with final typed outcomes, preserved ages, and exact surviving unbroadcast truth.**

## Performance

- **Duration:** 55 min
- **Started:** 2026-08-02T21:42:13Z
- **Completed:** 2026-08-02T22:37:16Z
- **Tasks:** 2
- **Files modified:** 17

## Accomplishments

- Added a std-only bounded Kahn topology that derives canonical identities once, orders ready records lexicographically by txid/wtxid, and isolates failed or cyclic in-snapshot components without rejecting independent records or snapshot-external prevouts.
- Completed the source-only schema cutover across recovery, snapshot codec, authority capture, and tests so historical fee, size, origin, relay, and stored-identity fields cannot regain authority.
- Added non-Clone `PreparedMempoolRecovery`, built without live mutations from current chainstate, consensus, policy, preserved acceptance times, and injected startup time.
- Classified confirmed, duplicate, proven missing-parent, policy-incompatible, expired, evicted, and recovered outcomes from staged final truth, including descendant propagation and exact capacity/expiry behavior.
- Restored unbroadcast membership only for persisted final survivors and rebuilt the final pool from source records so rolling rate, decay gate, and decay time match a fresh restart baseline.

## Task Commits

Each task was committed atomically:

1. **Task 1: Implement bounded deterministic dependency analysis** - `1c27e003` (feat)
2. **Task 2: Stage policy replay and derive final recovery truth** - `0771c16d` (feat)

## Files Created/Modified

- `packages/open-bitcoin-node/src/network/recovery/topology.rs` - Canonical identity validation, bounded graph construction, deterministic Kahn ordering, and component-local failure classification.
- `packages/open-bitcoin-node/src/network/recovery/staging.rs` - Side-effect-free replay, expiry/capacity finalization, exact unbroadcast intersection, and opaque prepared candidate.
- `packages/open-bitcoin-node/src/network/recovery.rs` - Preparation entrypoint and exhaustive recovery summary, including `DroppedExpired` evidence.
- `packages/open-bitcoin-node/src/network/tests/recovery_cases.rs` - Recovery topology, independent salvage, external-prevout, and final classification coverage.
- `packages/open-bitcoin-node/src/network/tests/recovery_cases/staging.rs` - Focused expiry, capacity, metadata, chainstate, and live-authority isolation coverage.
- `packages/open-bitcoin-node/src/storage/mempool_snapshot.rs` - Source-only record contract and complete typed recovery status vocabulary.
- `packages/open-bitcoin-node/src/storage/snapshot_codec/mempool.rs` - Compatibility decoding that validates historical fields without restoring their authority.
- `packages/open-bitcoin-node/src/network/runtime_authority/lifecycle.rs` - Canonical source-record construction after removal of transitional record members.
- `packages/open-bitcoin-mempool/src/pool.rs` - Read-only rolling decay-gate and last-update evidence needed to prove restart state.
- `packages/open-bitcoin-mempool/src/pool/tests/rolling_fee_cases.rs` - Direct fresh-pool rolling baseline coverage.
- `scripts/check-phase103-mempool-lifecycle.ts` and its mutation tests - Updated live recovery ownership anchors while preserving the original lifecycle constraints.
- `docs/parity/source-breadcrumbs.json` - Registered the new recovery topology and staged test sources against pinned Knots anchors.
- `docs/metrics/lines-of-code.md` - Refreshed tracked generated LOC evidence.

## Decisions Made

- Treated absent snapshot parents as external prevouts until current chainstate admission proves them missing; topology never invents chainstate truth.
- Preserved duplicate input records for exhaustive typed outcomes while allowing only one canonical identity to participate in the staged graph and pool.
- Rebuilt accepted survivors into a second fresh mempool after final expiry/capacity selection, avoiding accidental retention of historical rolling pressure.
- Kept `PreparedMempoolRecovery` non-Clone and uninstalled; Plan 03 owns the single atomic swap into live runtime authority.
- Left `requirements-completed` empty under the established Phase 135 staged-summary convention; MPDUR-02 and MPDUR-03 activate only after lifecycle-valid phase verification.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Repaired the stale Phase 103 lifecycle checker after recovery ownership moved**

- **Found during:** Task 1 full verification
- **Issue:** The live architecture check still required the removed `replay_into_mempool` symbol and old replay fixtures, so it rejected the planned topology/recovery ownership split.
- **Fix:** Updated only the checker corpus, recovery symbols, and behavior anchors while preserving its requirement, Knots-anchor, breadcrumb, ordering, and claim guards.
- **Files modified:** `scripts/check-phase103-mempool-lifecycle.ts`, `scripts/check-phase103-mempool-lifecycle.test.ts`
- **Verification:** The live checker and all 8 mutation cases pass; the complete repository verifier also passes.
- **Committed in:** `1c27e003`

**2. [Rule 3 - Blocking] Split staged recovery tests at the managed source-length boundary**

- **Found during:** Task 2 Bright Builds verification
- **Issue:** Required recovery coverage increased `recovery_cases.rs` to 764 lines, above the managed 628-line limit.
- **Fix:** Moved the staged expiry and chainstate cases into a focused child module with exact parity breadcrumbs, leaving the parent at 620 lines.
- **Files modified:** `packages/open-bitcoin-node/src/network/tests/recovery_cases.rs`, `packages/open-bitcoin-node/src/network/tests/recovery_cases/staging.rs`, `docs/parity/source-breadcrumbs.json`
- **Verification:** Bright Builds file-shape checks and parity breadcrumb validation pass.
- **Committed in:** `0771c16d`

**3. [Rule 2 - Missing Critical] Exposed every rolling-fee restart field for direct verification**

- **Found during:** Task 2 full verification and coverage
- **Issue:** Recovery could verify the numeric rolling floor but had no read-only evidence for the decay gate or last update time, leaving the complete restart invariant unprovable and the new accessors uncovered.
- **Fix:** Added clock-free read-only accessors and a direct fresh-pool test proving the open restart gate and epoch update time.
- **Files modified:** `packages/open-bitcoin-mempool/src/pool.rs`, `packages/open-bitcoin-mempool/src/pool/tests/rolling_fee_cases.rs`
- **Verification:** The rolling baseline test, all workspace tests, coverage gate, and complete repository verifier pass.
- **Committed in:** `0771c16d`

**4. [Rule 3 - Blocking] Preserved staged requirement activation until phase verification**

- **Found during:** Final metadata preparation
- **Issue:** Marking MPDUR-02 and MPDUR-03 complete from this implementation plan would activate them before Plan 03 installs the candidate and Phase 135 establishes lifecycle-valid parity evidence.
- **Fix:** Followed the existing Phase 135 summary convention and left `requirements-completed` empty for the phase verifier to close.
- **Files modified:** `.planning/phases/135-snapshot-schema-checkpointing-and-recovery/135-02-SUMMARY.md`
- **Verification:** Summary metadata retains the originating lifecycle identifiers and does not advance phase state.
- **Committed in:** final documentation commit

**Total deviations:** 4 auto-fixed (1 missing critical, 3 blocking)

**Impact on plan:** The repairs keep architecture enforcement, managed file shape, rolling-state evidence, and staged requirement traceability correct without expanding the recovery architecture.

## Issues Encountered

- The first complete verifier run exposed the stale Phase 103 symbol corpus; the checker was narrowed to the new recovery modules and passed its complete mutation suite.
- The Task 2 verifier exposed the managed file-length boundary and uncovered rolling-state accessors. Focused extraction and direct behavioral coverage resolved both before commit.
- An initial rolling baseline assertion assumed a closed decay gate; direct inspection of the documented fresh-pool state showed that restart truth is an open gate at epoch time, and the test was corrected to the actual invariant.

## Verification

- `cargo fmt --all` - passed
- `cargo clippy --all-targets --all-features -- -D warnings` - passed
- `cargo build --all-targets --all-features` - passed
- `cargo test --all-features` - passed
- Focused `open-bitcoin-node` recovery suite - 11 passed, 0 failed
- Phase 103 lifecycle checker - live check and 8 mutation cases passed
- `bun scripts/bright-builds-check.ts all` - passed
- `bun run scripts/check-parity-breadcrumbs.ts --check` - passed
- `bash scripts/verify.sh` - passed after each task; both normal pre-commit hooks independently passed the full repository contract in 4m53s and 4m14s
- `git diff --check` - passed

## Known Stubs

None. The stub scan found only ordinary typed empty collection initializers in the Phase 103 checker.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan 03 can consume the non-Clone `PreparedMempoolRecovery` once and atomically install its complete final pool, outcomes, unbroadcast set, generation/time, and projection inputs.
- Plan 05 and phase verification can build on the final typed recovery truth without consulting historical pressure or relay metadata.
- MPDUR-02 and MPDUR-03 remain intentionally unactivated until staged installation and lifecycle-valid verification are complete.
- No blocker remains for subsequent Phase 135 plans.

## Self-Check

PASSED

- Summary and all three created Rust files exist at their declared paths.
- Task commits `1c27e003` and `0771c16d` are present in repository history.
- Summary frontmatter contains exactly one opening and closing delimiter pair.
- Stub and threat-surface scans found no goal-blocking stubs or unplanned trust-boundary surface.
- `git diff --check` passed.

***

*Phase: 135-snapshot-schema-checkpointing-and-recovery*
*Completed: 2026-08-02*
