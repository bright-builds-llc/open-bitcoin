---
phase: 135-snapshot-schema-checkpointing-and-recovery
plan: "06"
subsystem: rpc-runtime
tags: [rust, mempool, startup-recovery, checkpointing, fjall, sync-durability]
requires:
  - phase: 135-03
    provides: staged mempool recovery authority and consuming atomic install
  - phase: 135-05
    provides: bounded snapshot loads and single-flight periodic/shutdown checkpoint coordination
provides:
  - policy-bounded staged snapshot recovery before durable runtime publication
  - private five-minute daemon checkpoint coordination for durable runtimes
  - producer-quiesced exact-current Sync shutdown before clean-marker persistence
affects: [135-07, phase-135-verification, daemon-restart, mempool-durability]
tech-stack:
  added: []
  patterns: [pre-publication-recovery-install, private-periodic-worker, quiesce-settle-mark-clean]
key-files:
  created:
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/checkpoint.rs
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/checkpoint.rs
  modified:
    - packages/open-bitcoin-rpc/src/context/mempool_recovery.rs
    - packages/open-bitcoin-rpc/src/context/network.rs
    - packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/mempool.rs
    - packages/open-bitcoin-node/src/network/recovery.rs
key-decisions:
  - "Durable startup derives snapshot limits from the exact PolicyConfig used by the runtime and installs one prepared recovery before publishing the managed handle."
  - "Checkpoint cadence remains a private 300-second daemon constant; transient and no-store runtimes start no worker."
  - "Daemon shutdown joins every mutation producer, settles exact-current Sync durability, and only then persists the clean marker; any typed failure stops the sequence."
patterns-established:
  - "Validate-stage-install startup: bounded external bytes become a prepared recovery outside authority, then one consuming install occurs before concurrency."
  - "Quiesce-settle-mark-clean shutdown: producer joins precede checkpoint settlement, and clean evidence follows durable success only."
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 135-2026-08-02T17-41-48
generated_at: 2026-08-03T02:43:00Z
duration: 1h 23m
completed: 2026-08-02
---

# Phase 135 Plan 06: Startup Recovery and Daemon Checkpoint Composition Summary

**Policy-bounded staged startup recovery plus a private five-minute checkpoint worker whose producer-quiesced shutdown requires exact-current Sync durability before recording clean state**

## Performance

- **Duration:** 1h 23m
- **Started:** 2026-08-03T01:20:15Z
- **Completed:** 2026-08-03T02:42:45Z
- **Tasks:** 2
- **Files modified:** 34

## Accomplishments

- Migrated both durable RPC constructor families to policy-derived bounded snapshot loading, startup-time staging, and one consuming recovery install before any shared managed handle reaches HTTP, inbound, sync, metrics, or checkpoint producers.
- Preserved current-v2 and safe legacy-v1 recovery while mapping limit, schema, decode, and identity failures to typed unavailable evidence without overwriting corrupt or oversized stored bytes.
- Deleted the deprecated zero-argument snapshot loader and both no-time live-recovery adapters, together with their obsolete helper chain and narrow compatibility allowances.
- Added one private durable-runtime checkpoint worker with an injected wait/time seam, a fixed 300-second interval, single-flight periodic coordination, and fixed low-cardinality failure classes.
- Reordered daemon shutdown so HTTP and mutation producers stop first, the checkpoint worker settles/forces exact-current Sync durability second, and `mark_clean_shutdown(PersistMode::Sync)` runs only after success.

## Task Commits

Each task was committed atomically:

1. **Task 1: Route startup load through staged recovery and atomic install** - `758aec10` (feat)
2. **Task 2: Drive private periodic checkpoints and exact-current clean shutdown** - `28efc3ca` (feat)

The plan summary is recorded by the final documentation commit.

## Files Created/Modified

- `packages/open-bitcoin-rpc/src/context/mempool_recovery.rs` - Composes policy limits, bounded Fjall load, explicit startup time, prepared recovery, and consuming install.
- `packages/open-bitcoin-rpc/src/context/network.rs` - Reuses the exact runtime policy and completes recovery before publishing either durable constructor family.
- `packages/open-bitcoin-node/src/storage/fjall_store/mempool.rs` - Keeps only the explicit bounded snapshot load API.
- `packages/open-bitcoin-node/src/network/recovery.rs` and `packages/open-bitcoin-node/src/network/runtime_authority/recovery.rs` - Remove deprecated no-time recovery and obsolete live-mutation helpers.
- `packages/open-bitcoin-rpc/src/bin/open_bitcoind/checkpoint.rs` - Owns the private periodic worker, fixed failure taxonomy, shutdown settlement, and clean-marker ordering.
- `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/checkpoint.rs` - Proves cadence, no-store behavior, retry after periodic failure, producer barriers, exact-current save/skip, and failure ordering.
- `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs` - Starts the durable checkpoint worker and propagates producer-join/checkpoint shutdown failures before a clean claim.
- `packages/open-bitcoin-rpc/Cargo.toml`, `packages/open-bitcoin-rpc/BUILD.bazel`, and `MODULE.bazel.lock` - Promote the existing mempool dependency to production and keep Cargo/Bazel metadata aligned.
- `docs/parity/source-breadcrumbs.json` - Registers a dedicated daemon mempool checkpoint breadcrumb group.
- `scripts/check-phase97-inbound-metrics.ts`, `scripts/check-phase102-orphan-admission-bridge/bridge.ts`, `scripts/check-phase103-mempool-lifecycle.ts`, and `scripts/check-phase108-durable-mempool-relay-state-recovery.ts` - Track the final staged startup API and removed adapters without weakening prior phase guards.
- `scripts/panic-sites.allowlist` and `docs/metrics/lines-of-code.md` - Refresh exact generated verification evidence.

## Decisions Made

- Derived encoded bytes, record count, per-transaction bytes, and unbroadcast count with checked arithmetic from the same policy instance used to construct the runtime; derivation failure is typed startup unavailability rather than an unbounded fallback.
- Kept periodic checkpoint errors non-fatal to daemon operation but visible through fixed classes and coordinator state; shutdown checkpoint or clean-marker errors remain fatal to the clean claim.
- Used one owned thread and channel for the private worker instead of adding a scheduler, public configuration, RPC/CLI control, or another authority owner.
- Treated a sync mutation-producer join panic as a typed shutdown failure so a failed producer join can never be followed by a clean marker.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - Missing Critical] Removed the obsolete recovery helper chain exposed by adapter deletion**

- **Found during:** Task 1 legacy adapter inventory
- **Issue:** Deleting the three named compatibility adapters left production-only mutable recovery helpers and projections with no valid caller, preserving an accidental live-mutation path contrary to staged recovery ownership.
- **Fix:** Removed the orphaned helper chain, related re-exports, and stale parity wording while retaining only `prepare_mempool_recovery_at` plus consuming `install_mempool_recovery`.
- **Files modified:** Node network recovery/authority modules, dependent relay/admission modules, snapshot DTO/codec modules, and `docs/parity/catalog/p2p.md`.
- **Verification:** Repository-wide legacy-call search returned no definitions or callers; clippy, all workspace tests, architecture checks, and the full verifier passed.
- **Committed in:** `758aec10`

**2. [Rule 3 - Blocking] Updated prior-phase guards for the final staged startup API**

- **Found during:** Task 1 repository verification
- **Issue:** Phase 97, 102, 103, and 108 checkers encoded the transitional deprecated adapters and rejected their planned removal.
- **Fix:** Narrowly migrated checker anchors and mutation fixtures to the final bounded load, prepared recovery, and consuming install sequence; removed Plan 03/05 deprecated-call allowances.
- **Files modified:** `scripts/check-phase97-inbound-metrics*`, `scripts/check-phase102-orphan-admission-bridge*`, `scripts/check-phase103-mempool-lifecycle*`, and `scripts/check-phase108-durable-mempool-relay-state-recovery*`.
- **Verification:** Every affected checker and mutation suite passed, followed by the complete repository verifier.
- **Committed in:** `758aec10`

**3. [Rule 3 - Blocking] Refreshed exact panic and source metrics evidence**

- **Found during:** Task 1 and Task 2 verification
- **Issue:** Removing legacy source and adding daemon modules invalidated exact result-site and deterministic LOC evidence.
- **Fix:** Narrowed `scripts/panic-sites.allowlist` to the surviving classified sites and regenerated the index-backed LOC report after staging each task corpus.
- **Files modified:** `scripts/panic-sites.allowlist`, `docs/metrics/lines-of-code.md`.
- **Verification:** Panic-site classification, LOC freshness, Bright Builds checks, and both commit hooks passed.
- **Committed in:** `758aec10`, `28efc3ca`

**4. [Rule 3 - Blocking] Promoted the existing mempool crate to a production RPC dependency**

- **Found during:** Task 2 compilation
- **Issue:** The production checkpoint clock constructs `PolicyTime`, but `open-bitcoin-mempool` was available to the RPC crate only as a dev dependency.
- **Fix:** Moved the existing path dependency to production, added the matching Bazel direct dependency, and refreshed the Bzlmod lock fingerprint.
- **Files modified:** `packages/open-bitcoin-rpc/Cargo.toml`, `packages/open-bitcoin-rpc/BUILD.bazel`, `MODULE.bazel.lock`.
- **Verification:** Cargo clippy/build/test and Bazel build/run gates passed.
- **Committed in:** `28efc3ca`

**5. [Rule 2 - Missing Critical] Made producer join failure block clean shutdown**

- **Found during:** Task 2 shutdown ordering implementation
- **Issue:** The existing sync worker logged a join panic and continued, which could let final checkpoint and clean-marker persistence proceed without proof that a mutation producer had quiesced.
- **Fix:** Changed sync-worker shutdown to return fixed-class `ProducerJoin` failure and propagated it before checkpoint settlement.
- **Files modified:** `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs`, `packages/open-bitcoin-rpc/src/bin/open_bitcoind/checkpoint.rs`.
- **Verification:** Source-order and barrier tests prove producer joins precede settlement; all 27 daemon tests and the full verifier passed.
- **Committed in:** `28efc3ca`

**6. [Rule 3 - Blocking] Preserved staged requirement activation until phase verification**

- **Found during:** Final summary creation
- **Issue:** Activating MPDUR-02, MPDUR-03, and MPDUR-04 from this intermediate plan summary would preempt Phase 135's lifecycle-valid verification and reconciliation plans.
- **Fix:** Followed the established Phase 135 summary convention and left `requirements-completed` empty for final phase verification.
- **Files modified:** `.planning/phases/135-snapshot-schema-checkpointing-and-recovery/135-06-SUMMARY.md`.
- **Verification:** Matches the staged lifecycle treatment in Plan 05 and leaves phase-level requirement completion to the parent orchestrator.
- **Committed in:** final documentation commit

**Total deviations:** 6 auto-fixed (2 missing critical, 4 blocking)
**Impact on plan:** Each deviation closed a correctness, ownership, verification, or build-integration gap required by the planned composition. No public surface or architectural scope was added.

## Issues Encountered

- TDD RED runs failed on the expected missing policy-limit and checkpoint-worker symbols. Repository commit rules require green verification, so each task retained one atomic feature commit after its RED/GREEN cycle rather than committing an intentionally failing tree.
- The first Task 2 commit attempt stopped in the hook because the two new source headers inherited the broad RPC breadcrumb group. A dedicated `rpc-mempool-checkpoint-daemon` group restored the intended Knots persistence anchors; canonical generation then passed for 754 Rust files.
- A Phase 127 checker recursively inspected the new no-store test and interpreted direct transient-handle construction as a production-authority mutation. The test now obtains a valid handle from a durable fixture while still passing `None` to prove no-store worker suppression.

## Verification

- Focused RPC startup context suite - 20 passed, 0 failed
- Focused daemon checkpoint suite - 10 passed, 0 failed
- Complete `open-bitcoind` binary suite - 27 passed, 0 failed
- `cargo fmt --all` - passed
- `cargo clippy --all-targets --all-features -- -D warnings` - passed
- `cargo build --all-targets --all-features` - passed
- `cargo test --all-features` - passed, including doctests
- Parity breadcrumbs - 754 Rust files verified
- Phase 127 mutation suite - 15 passed, 0 failed
- `bun scripts/bright-builds-check.ts all` - passed
- `bash scripts/verify.sh` - passed in 8m 34.453s after implementation
- Task 1 commit hook - complete verifier passed in 11m 55.879s
- Task 2 commit hook - complete verifier passed in 7m 32.687s
- `git diff --check` - passed

## Known Stubs

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Durable constructors now recover before concurrency, and supported daemon runtimes periodically checkpoint and can claim clean shutdown only after exact-current Sync durability.
- Plans 07 and 08 can add integration/release evidence against final production composition without relying on transitional recovery or load adapters.
- MPDUR-02, MPDUR-03, and MPDUR-04 remain intentionally unactivated until Phase 135's lifecycle-valid verification and reconciliation close them.
- No blockers remain.

## Self-Check

PASSED

- Summary and both declared created Rust files exist at their recorded paths.
- Task commits `758aec10` and `28efc3ca` exist in repository history.
- Stub scan found no TODO, FIXME, placeholder, coming-soon, or UI-flow empty-value stubs in the plan's changed Rust and TypeScript files; typed unavailable evidence is intentional fail-closed behavior.
- Frontmatter contains exactly one opening and one closing standalone delimiter.
- `git diff --check` passed.

***

*Phase: 135-snapshot-schema-checkpointing-and-recovery*
*Completed: 2026-08-02*
