---
phase: 134-authoritative-cross-cache-lifecycle-integration
plan: "14"
subsystem: lifecycle-effect-authority
tags: [rust, lifecycle-authority, affine-receipts, bounded-ledgers, concurrency, security]

requires:
  - phase: 134-13
    provides: authoritative lifecycle parity closeout and explicit pending requirement gate
provides:
  - unique process-local authority incarnations for every live ManagedNetworkHandle
  - family-specific exact pending and completed receipt accounting
  - atomic exact achieved-truth recording before freshness-sensitive consequences
  - adversarial cross-handle, binding-mismatch, stale-state, and replay regressions
affects: [phase-134-verification, phase-135, lifecycle-effects, receipt-authority]

tech-stack:
  added: []
  patterns:
    - live handles install a monotonic process-local authority incarnation before sharing the authority mutex
    - peer and snapshot ledgers validate and consume complete immutable family bindings
    - only exact pending receipts can become achieved; freshness controls consequences after accounting

key-files:
  created:
    - .planning/phases/134-authoritative-cross-cache-lifecycle-integration/134-14-SUMMARY.md
  modified:
    - packages/open-bitcoin-node/src/network.rs
    - packages/open-bitcoin-node/src/network/lifecycle_effects.rs
    - packages/open-bitcoin-node/src/network/lifecycle_projection.rs
    - packages/open-bitcoin-node/src/network/relay_serving.rs
    - packages/open-bitcoin-node/src/network/runtime_authority.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/lifecycle.rs
    - packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/effects.rs
    - packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/effects/contracts.rs
    - docs/metrics/lines-of-code.md

key-decisions:
  - "Install a unique non-initial authority epoch at the ManagedNetworkHandle construction boundary while retaining fixture-friendly initial epochs before handle installation."
  - "Store pending and completed truth as complete family-specific keys; use the monotonic family ID only as bounded eviction-order metadata."
  - "Reject foreign or immutable-mismatched receipts with a typed lifecycle error; reserve AchievedButStale for exact pending work whose current targets advanced."
  - "Keep MPLIFE-01 through MPLIFE-04 pending until independent phase re-verification."

patterns-established:
  - "Exact receipt accounting: validate the full authority, generation, family identity, target, and session/persistence binding before any ledger mutation."
  - "Achieved-before-freshness: atomically move an exact key from pending to completed, then classify Applied versus AchievedButStale."

requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 134-2026-07-28T01-41-12
generated_at: 2026-07-29T01:24:23Z

duration: 58m
completed: 2026-07-29
---

# Phase 134 Plan 14: Exact Effect Receipt Authority Summary

**Unique handle incarnations and exact family-bound ledgers now prevent cross-handle or malformed receipts from consuming work, while exact stale achievements preserve newer lifecycle state.**

## Performance

- **Duration:** 58m
- **Started:** 2026-07-29T00:26:08Z
- **Completed:** 2026-07-29T01:24:23Z
- **Tasks:** 2
- **Files modified:** 9

## Accomplishments

- Added a process-local, monotonic, overflow-checked authority-incarnation allocator and installed a distinct non-initial epoch for each live `ManagedNetworkHandle`.
- Replaced raw-ID pending/completed truth with complete peer and snapshot keys binding authority, lifecycle or persistence generation, effect identity, and family-specific target/session identity.
- Made exact pending completion atomically enter bounded completed accounting before current freshness is evaluated.
- Rejected cross-handle and every immutable binding mismatch without changing ledgers, dirty state, unbroadcast state, peer provenance, session state, or relay evidence.
- Proved exact current, exact stale, and exact replay outcomes across 23 focused lifecycle-effect tests.

## Task Commits

Each task was committed atomically:

1. **Task 1: Make pending effect identity authority-incarnation complete** - `0a647068` (fix)
2. **Task 2: Record exact achieved truth before freshness-sensitive lifecycle mutation** - `dcefcc64` (fix)

## Files Created/Modified

- `packages/open-bitcoin-node/src/network.rs` - Owns process-local authority-incarnation allocation.
- `packages/open-bitcoin-node/src/network/relay_serving.rs` - Installs a new incarnation at the live-handle boundary.
- `packages/open-bitcoin-node/src/network/runtime_authority.rs` - Calls incarnation installation from the actual shared-handle constructor.
- `packages/open-bitcoin-node/src/network/lifecycle_effects.rs` - Defines exact peer/snapshot keys and bounded atomic ledgers.
- `packages/open-bitcoin-node/src/network/runtime_authority/lifecycle.rs` - Rejects non-exact receipts and classifies freshness only after achieved accounting.
- `packages/open-bitcoin-node/src/network/lifecycle_projection.rs` - Provides the typed foreign-or-mismatched receipt error.
- `packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/effects.rs` - Covers current, stale, replay, and newer-state preservation behavior.
- `packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/effects/contracts.rs` - Covers unique handles, cross-handle collisions, every immutable mismatch, and ledger bounds.
- `docs/metrics/lines-of-code.md` - Regenerated by normal verification hooks.

## Decisions Made

- Kept `AuthorityEpoch::INITIAL` available for unshared fixtures, but required the live handle constructor to replace it with a process-unique incarnation.
- Used checked monotonic allocation and process abort for impossible poison/exhaustion states, satisfying fail-visible identity safety without introducing recoverable identifier reuse.
- Kept peer and snapshot families separate and affine; no generic effect bus, durable outbox, or second state authority was introduced.
- Classified a receipt as `AlreadyApplied` only from its exact completed key, and as `AchievedButStale` only after its exact pending key was recorded achieved.
- Left `MPLIFE-01` through `MPLIFE-04` unchecked for the independent phase verifier.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Wired incarnation installation through the actual handle constructor**

- **Found during:** Task 1
- **Issue:** The plan named `network.rs` and `relay_serving.rs`, but the production `Arc<Mutex<...>>` construction boundary also lives in `runtime_authority.rs`.
- **Fix:** Added the constructor call in `runtime_authority.rs` so every production handle installs its unique epoch before sharing.
- **Files modified:** `packages/open-bitcoin-node/src/network/runtime_authority.rs`
- **Verification:** Two independently constructed handles produced distinct non-initial epochs with colliding local effect IDs.
- **Committed in:** `0a647068`

**2. [Rule 2 - Missing Critical] Added a typed rejection for non-exact receipts**

- **Found during:** Task 2
- **Issue:** No existing lifecycle error represented a foreign or immutable-mismatched receipt, and treating it as achieved-stale was incorrect.
- **Fix:** Added `InvalidEffectReceipt` and returned it only when no exact pending or completed binding exists.
- **Files modified:** `packages/open-bitcoin-node/src/network/lifecycle_projection.rs`, `packages/open-bitcoin-node/src/network/runtime_authority/lifecycle.rs`
- **Verification:** The TDD regression failed before the change and then passed for every peer and snapshot binding dimension.
- **Committed in:** `dcefcc64`

**Total deviations:** 2 auto-fixed (1 blocking, 1 missing critical)
**Impact on plan:** Both changes were required to place the planned guarantees at the real construction and error boundaries; no architectural scope was added.

## Issues Encountered

- The first Task 1 commit attempt was rejected by the existing Phase 134 structural guard because it encoded earlier receipt field-count and bounded-order anchors. The implementation retained exact pending/completed sets while preserving monotonic raw IDs only as bounded eviction-order metadata; all 88 guard tests and the live checker then passed.
- The first Task 2 Clippy pass found snapshot receipt getters used only by tests after exact-key validation removed redundant production checks. Gating those accessors with `cfg(test)` resolved the warning without changing behavior.
- Full normal hooks each took about seven minutes and included quiet macOS test-harness intervals; timing heartbeats confirmed liveness, so no Cargo work was interrupted or overlapped.

## Verification

- TDD RED: both foreign-binding dispatcher regressions failed because mismatches were incorrectly classified achieved-stale.
- Focused GREEN: 23 lifecycle-effect tests passed, including cross-handle rejection, all immutable mismatches, exact current/stale transitions, and replay.
- `bun test scripts/check-phase134-authoritative-lifecycle.test.ts`: 88 passed, 0 failed.
- `bun run scripts/check-phase134-authoritative-lifecycle.ts`: passed.
- Ordered `cargo fmt`, Clippy with `-D warnings`, all-target/all-feature build, and all-feature tests passed for both tasks.
- `bun scripts/bright-builds-check.ts all`: 0 findings.
- Both normal task commit hooks passed the complete `bash scripts/verify.sh` contract, including Cargo tests/doc-tests/coverage, Bazel builds/tests/provenance, benchmark smoke, checkers, breadcrumbs, and LOC freshness.
- `git diff --check` passed.

## Known Stubs

None.

## Threat Flags

None. The authority-incarnation and receipt-completion trust boundaries were explicitly covered by T-134-14-01 through T-134-14-04; no unplanned endpoint, authentication path, file-access boundary, schema boundary, or dependency was introduced.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan 14 closes the receipt-authority and exact-consumption gap for Phase 134.
- Later Phase 134 gap plans may build on exact receipt accounting without accepting foreign or malformed bindings as achieved.
- `MPLIFE-01` through `MPLIFE-04` intentionally remain pending until independent phase-level re-verification.

## Self-Check: PASSED

The summary and both task commits exist, all nine modified files exist, frontmatter uses exactly two delimiters, no goal-blocking stubs or unplanned threat surface were found, and requirement promotion remains deferred.

*Phase: 134-authoritative-cross-cache-lifecycle-integration*
*Completed: 2026-07-29*
