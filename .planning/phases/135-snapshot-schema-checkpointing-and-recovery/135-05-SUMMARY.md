---
phase: 135-snapshot-schema-checkpointing-and-recovery
plan: "05"
subsystem: node-runtime
tags: [rust, mempool, fjall, checkpointing, single-flight, sync-durability]
requires:
  - phase: 135-03
    provides: CurrentV2 source-only snapshots and staged recovery authority
  - phase: 135-04
    provides: affine checkpoint capabilities, receipts, and exact durability evidence
provides:
  - explicitly bounded node-owned mempool snapshot loads
  - Sync-only Fjall snapshot execution returning retained achieved receipts
  - single-flight periodic and shutdown checkpoint coordination with bounded coalesced follow-up
affects: [135-06, mempool-recovery, rpc-startup, operator-checkpoint-evidence]
tech-stack:
  added: []
  patterns: [single-flight-state-machine, receipt-preserving-completion, outside-lock-sync-io]
key-files:
  created:
    - packages/open-bitcoin-node/src/network/checkpoint.rs
    - packages/open-bitcoin-node/src/network/checkpoint/tests.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/tests/snapshot_persistence/load_limits.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/tests/snapshot_persistence/write_execution_failures.rs
  modified:
    - packages/open-bitcoin-node/src/storage/fjall_store/mempool.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/effects.rs
    - packages/open-bitcoin-node/src/network/lifecycle_effects/checkpoint.rs
    - packages/open-bitcoin-node/src/network.rs
key-decisions:
  - "Checkpoint execution always uses Fjall SyncAll and returns the achieved receipt before authority completion is attempted."
  - "The coordinator keeps one flight claimed across prepare, outside-lock persistence, completion, and at most one immediate follow-up write."
  - "A completion-dispatch failure stores the original non-Clone receipt in AchievedAwaitingCompletion and retries it before any new capture."
patterns-established:
  - "Receipt-preserving terminal transition: achieved external effects remain owned until idempotent authority completion succeeds."
  - "Bounded coalescing: concurrent triggers return Coalesced, while one stale completion may start only one immediate follow-up."
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 135-2026-08-02T17-41-48
generated_at: 2026-08-03T01:10:00Z
duration: 1h 25m
completed: 2026-08-02
---

# Phase 135 Plan 05: Sync Checkpoint Execution and Single-Flight Coordination Summary

**Bounded snapshot loads, SyncAll persistence with retained achieved receipts, and a single-flight coordinator that coalesces concurrent and stale checkpoint work**

## Performance

- **Duration:** 1h 25m
- **Started:** 2026-08-02T23:44:00Z
- **Completed:** 2026-08-03T01:09:11Z
- **Tasks:** 3
- **Files modified:** 24

## Accomplishments

- Added a caller-owned `MempoolSnapshotDecodeLimits` contract and migrated every node-owned load to the bounded decoder while keeping the deprecated zero-argument adapter isolated to the two Plan 06 RPC startup callers.
- Made prepared checkpoint execution unconditionally synchronous, sampled terminal time exactly once, exact-aborted only before achievement, and returned `SnapshotWriteReceipt` without consuming it in authority completion.
- Removed the legacy snapshot terminal aliases and retained only typed `acknowledge_write(completed_at, strength)`, `abort_snapshot_write(abort)`, and `complete_snapshot_write(receipt)` transitions.
- Added `MempoolCheckpointCoordinator` with private `Idle`, `Persisting`, and `AchievedAwaitingCompletion` states, clean periodic skip, concurrent coalescing, receipt-first retry, and at most one immediate stale follow-up.
- Proved authority mutation remains available during blocked persistence, stale durable high-water advances without clearing newer dirty work, exact loss ranges remain truthful, and shutdown succeeds only at exact current durability.

## Task Commits

Each task was committed atomically:

1. **Task 1: Require bounded snapshot loads and migrate every node caller** - `3f42bfa5` (feat)
2. **Task 2: Preserve achieved Sync receipts across completion failure** - `cefd9a90` (feat)
3. **Task 3: Implement the single-flight coalescing coordinator** - `1da20952` (feat)

The plan summary is recorded by the final documentation commit.

## Files Created/Modified

- `packages/open-bitcoin-node/src/storage/fjall_store/mempool.rs` - Defines explicit load limits and the Sync-only prepared snapshot executor returning an achieved receipt.
- `packages/open-bitcoin-node/src/storage/fjall_store/tests/snapshot_persistence/load_limits.rs` - Proves bounded decode rejection and byte-identical retained storage.
- `packages/open-bitcoin-node/src/storage/fjall_store/tests/snapshot_persistence/write_execution_failures.rs` - Proves abort-dispatch failure remains typed without false persistence evidence or byte mutation.
- `packages/open-bitcoin-node/src/network/lifecycle_effects/checkpoint.rs` - Owns the final typed snapshot acknowledgement and abort contracts.
- `packages/open-bitcoin-node/src/network/runtime_authority/effects.rs` - Exposes the final typed terminal methods and returns completion source plus retained receipt to the coordinator.
- `packages/open-bitcoin-node/src/network/checkpoint.rs` - Implements the single-flight periodic and shutdown checkpoint shell.
- `packages/open-bitcoin-node/src/network/checkpoint/tests.rs` - Covers clean skip, concurrency, bounded follow-up, stale evidence, encode/write/completion failures, duplicate completion, and shutdown exactness.
- `packages/open-bitcoin-node/src/network.rs` - Exports the checkpoint coordinator, typed outcome, and typed error.
- `scripts/check-phase134-authoritative-lifecycle.ts` - Recognizes the final receipt-retaining completion dispatcher without weakening its lifecycle mutation checks.
- `docs/parity/source-breadcrumbs.json` - Registers the new checkpoint and split storage test paths.
- `docs/metrics/lines-of-code.md` - Tracks verifier-generated source metrics after the new modules.

## Decisions Made

- Kept cadence out of the public coordinator API; its evidence interval is an internal implementation detail and callers provide only injected time.
- Held the coordinator flight, but neither the coordinator mutex nor the authority guard, across encoding and Fjall SyncAll so concurrent triggers coalesce while unrelated authority mutation proceeds.
- Counted two writes as the hard per-call maximum: the captured write plus one immediate coalesced follow-up. Continued mutation remains dirty for the next tick.
- Reused Phase 134's sole authority and affine receipt ledger rather than introducing a scheduler, journal, actor, or second mutation owner.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Split snapshot persistence coverage to satisfy managed file limits**

- **Found during:** Tasks 1 and 2 verification
- **Issue:** Consolidated bounded-load and failure-injection cases would push `snapshot_persistence.rs` beyond the repository's strict 628-line production Rust limit.
- **Fix:** Added focused `load_limits.rs` and `write_execution_failures.rs` child test modules with required inline and registry breadcrumbs.
- **Files modified:** `packages/open-bitcoin-node/src/storage/fjall_store/tests/snapshot_persistence.rs`, both new child modules, and `docs/parity/source-breadcrumbs.json`.
- **Verification:** Bright Builds file-length checks, parity breadcrumb validation, focused storage tests, and the full verifier passed.
- **Committed in:** `3f42bfa5`, `cefd9a90`

**2. [Rule 3 - Blocking] Preserved RPC compile compatibility around the bounded load cutover**

- **Found during:** Task 1 workspace verification
- **Issue:** Deprecating the zero-argument load adapter caused the two intentionally unmigrated RPC startup families to emit warnings under `-D warnings`, and the new public limit type required storage re-exports.
- **Fix:** Re-exported the explicit limit contract, kept unchanged legacy decode only in the deprecated adapter, and narrowly annotated the two RPC compatibility calls until Plan 06 derives policy limits.
- **Files modified:** `packages/open-bitcoin-node/src/storage/fjall_store.rs`, `packages/open-bitcoin-node/src/storage/snapshot_codec.rs`, and `packages/open-bitcoin-rpc/src/context/mempool_recovery.rs`.
- **Verification:** The all-target/all-feature workspace check and complete verifier passed with both RPC constructor families compiling.
- **Committed in:** `3f42bfa5`

**3. [Rule 3 - Blocking] Updated the Phase 134 lifecycle checker for the final receipt-retaining terminal path**

- **Found during:** Task 2 verification
- **Issue:** The live checker recognized only the pre-cutover snapshot completion spelling and would reject the final `dispatch_checkpoint_completion` path even though it preserves the same exact lifecycle authority boundary.
- **Fix:** Updated the checker and its mutation fixture to recognize the final typed dispatcher and freshness guard without broadening accepted mutation paths.
- **Files modified:** `scripts/check-phase134-authoritative-lifecycle.ts`, `scripts/check-phase134-authoritative-lifecycle.test.ts`.
- **Verification:** All 248 Phase 134 mutation tests and the live checker passed repeatedly, including in the repository verifier.
- **Committed in:** `cefd9a90`

**4. [Rule 3 - Blocking] Preserved staged requirement activation until phase verification**

- **Found during:** Final summary creation
- **Issue:** Marking MPDUR-04 complete in this intermediate plan summary would activate a phase-level requirement before Phase 135's lifecycle-valid final verification plans close it.
- **Fix:** Followed the established Phase 135 staged-summary convention and left `requirements-completed` empty for final phase verification to activate.
- **Files modified:** `.planning/phases/135-snapshot-schema-checkpointing-and-recovery/135-05-SUMMARY.md`.
- **Verification:** Summary structure matches prior Phase 135 lifecycle summaries; final metadata hook verifies active-milestone traceability.
- **Committed in:** final documentation commit

**Total deviations:** 4 auto-fixed (4 blocking)
**Impact on plan:** All deviations preserved compile compatibility, managed file shape, architecture checking, or lifecycle-valid requirement staging. No product or architectural scope was added.

## Issues Encountered

- The first focused breadcrumb check could not see untracked new Rust files; staging only those new paths made the checker validate the real planned corpus.
- Clippy rejected an `unreachable!` branch in the coordinator claim transition. The state replacement was simplified into a total match with no panic-like path.
- Each feature commit intentionally replayed the repository's full verification hook; all completed successfully.

## Verification

- `cargo fmt --all` - passed
- `cargo clippy --all-targets --all-features -- -D warnings` - passed
- `cargo build --all-targets --all-features` - passed
- `cargo test --all-features` - passed, including 636 node tests and doctests
- Focused bounded-load snapshot persistence tests - passed
- Focused prepared-write and lifecycle-effect suites - passed
- Focused checkpoint filter - 11 passed, 0 failed
- `cargo check --workspace --all-targets --all-features` - passed, including the two deprecated RPC compatibility callers
- Parity breadcrumbs - 752 Rust files verified
- Phase 134 checker - 248 mutation tests passed and live checker passed
- `bun scripts/bright-builds-check.ts all` - passed
- `bash scripts/verify.sh` - passed in 7m 17s before Task 3 commit; every task commit hook independently passed the full contract
- `git diff --check` - passed

## Known Stubs

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan 06 can derive startup decode limits for both RPC constructor families, migrate them to `load_mempool_snapshot_with_limits`, and delete the deprecated zero-argument adapter.
- The coordinator is ready for runtime composition: periodic callers can skip or coalesce deterministically, and shutdown callers can require exact current durability after producer quiescence.
- MPDUR-04 remains intentionally unactivated until Phase 135's final lifecycle-valid verification closes the requirement.
- No blockers remain.

## Self-Check

PASSED

- Summary and all four declared created Rust files exist at their recorded paths.
- Task commits `3f42bfa5`, `cefd9a90`, and `1da20952` exist in repository history.
- Stub scan found no TODO, FIXME, placeholder, coming-soon, unavailable, or UI-flow empty-value stubs in the plan's created and modified files; the matched TypeScript empty arrays are legitimate local accumulators.
- Frontmatter contains exactly one opening and one closing standalone delimiter.
- `git diff --check` passed.

***

*Phase: 135-snapshot-schema-checkpointing-and-recovery*
*Completed: 2026-08-02*
