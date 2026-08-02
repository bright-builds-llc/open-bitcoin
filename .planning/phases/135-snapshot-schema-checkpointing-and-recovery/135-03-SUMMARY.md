---
phase: 135-snapshot-schema-checkpointing-and-recovery
plan: "03"
subsystem: mempool-recovery
tags: [rust, mempool, recovery, lifecycle-authority, snapshots, parity]
requires:
  - phase: 135-02
    provides: non-Clone staged final recovery truth
  - phase: 135-04
    provides: authority-owned generation, capture time, checkpoint evidence, and exact unbroadcast membership
  - phase: 134-authoritative-cross-cache-lifecycle-integration
    provides: sole lifecycle dispatcher and authoritative derived-projection boundaries
provides:
  - startup-only consuming recovery command through ManagedNetworkHandle
  - atomic canonical and derived projection rebuild with clean captured-generation evidence
  - deterministic node recovery caller migration and connected-block regression coverage
affects: [135-05, 135-06, mempool-recovery, lifecycle-authority, rpc-startup]
tech-stack:
  added: []
  patterns: [epoch-bound-preparation, consuming-lifecycle-command, validate-before-replace]
key-files:
  created:
    - packages/open-bitcoin-node/src/network/lifecycle_projection/recovery.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/recovery.rs
    - packages/open-bitcoin-node/src/network/tests/recovery_cases/metadata.rs
  modified:
    - packages/open-bitcoin-node/src/network/lifecycle_projection.rs
    - packages/open-bitcoin-node/src/network/lifecycle_projection/authority.rs
    - packages/open-bitcoin-node/src/network/recovery.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/effects.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/lifecycle.rs
    - packages/open-bitcoin-node/src/network/tests/recovery_cases.rs
    - packages/open-bitcoin-node/src/network/tests/recovery_cases/staging.rs
    - packages/open-bitcoin-node/src/network/tests/mempool_lifecycle_cases/connected_block_removal.rs
    - packages/open-bitcoin-rpc/src/context/mempool_recovery.rs
key-decisions:
  - "Prepared recovery is epoch-bound, non-Clone, validated completely, and consumed only by LifecycleCommand::InstallRecovery."
  - "Recovery initializes current and last-durable generation from captured generation without fabricating checkpoint trigger, strength, or completion metadata."
  - "Legacy no-time recovery remains only for two RPC startup calls until Plan 06, with statement-scoped deprecation allowances."
patterns-established:
  - "Recovery authority: prepare outside the authority guard, bind to the observed epoch, and consume once through the sole managed dispatcher."
  - "Atomic installation: validate the complete candidate and empty effect ledgers before replacing canonical state and every derived projection."
requirements-completed: []
generated_by: gsd-executor
lifecycle_mode: yolo
phase_lifecycle_id: 135-2026-08-02T17-41-48
generated_at: 2026-08-02T23:35:13Z
duration: 52min
completed: 2026-08-02
---

# Phase 135 Plan 03: Atomic Recovery Authority Summary

**Epoch-bound prepared mempool recovery now installs once through the lifecycle dispatcher, rebuilding every dependent projection with clean captured-generation evidence.**

## Performance

- **Duration:** 52 min
- **Started:** 2026-08-02T22:43:54Z
- **Completed:** 2026-08-02T23:35:13Z
- **Tasks:** 2
- **Files modified:** 20

## Accomplishments

- Added `LifecycleCommand::InstallRecovery` as the only consuming path from a non-Clone `PreparedMempoolRecovery` into live authority, with epoch freshness, generation/time, projection-shape, and pending-effect validation before mutation.
- Replaced canonical mempool truth and all serving, relay, peer, unbroadcast, generation, checkpoint, and recovery-evidence projections in one bounded authority transition without seeding outbound relay work.
- Established restart evidence at the captured generation: current and last-durable generations agree, dirty and in-flight checkpoint state are empty, and the first later mutation advances exactly once and opens the loss interval.
- Proved five injected validation failures plus stale and nonfresh candidates leave the exact live aggregate unchanged.
- Migrated every node recovery test caller, including all three recovered connected-block removal paths, to deterministic staged preparation and installation while retaining only the two explicitly planned RPC startup compatibility calls.
- Added exact post-recovery and first-post-mutation reconciliation, unbroadcast membership, connected-block cleanup, replacement identity, and all seven recovery-summary classification assertions.

## Task Commits

Each task was committed atomically:

1. **Task 1: Install staged recovery atomically through lifecycle authority** - `1a4e098d` (feat)
2. **Task 2: Migrate node recovery callers and enforce reconciliation** - `64f32419` (feat)

## Files Created/Modified

- `packages/open-bitcoin-node/src/network/lifecycle_projection/recovery.rs` - Complete candidate validation and bounded atomic replacement of canonical and derived recovery state.
- `packages/open-bitcoin-node/src/network/runtime_authority/recovery.rs` - Epoch-bound preparation and consuming managed-handle installation facade.
- `packages/open-bitcoin-node/src/network/tests/recovery_cases/metadata.rs` - Recovery metadata, generation, checkpoint, and validation-failure coverage.
- `packages/open-bitcoin-node/src/network/lifecycle_projection.rs` and `authority.rs` - Recovery command dispatch plus independent current and last-durable generation evidence.
- `packages/open-bitcoin-node/src/network/lifecycle_effects.rs` and `checkpoint.rs` - Read-only pending-effect checks used to protect startup installation.
- `packages/open-bitcoin-node/src/network/recovery.rs` and `recovery/staging.rs` - Epoch ownership on prepared recovery and narrowly retained deprecated compatibility adapter.
- `packages/open-bitcoin-node/src/network/runtime_authority.rs`, `effects.rs`, and `lifecycle.rs` - Managed command routing and exact installed projection inputs.
- `packages/open-bitcoin-node/src/network/tests/recovery_cases.rs` and `staging.rs` - Deterministic staged caller migration, exhaustive classifications, and reconciliation assertions.
- `packages/open-bitcoin-node/src/network/tests/mempool_lifecycle_cases.rs` and `connected_block_removal.rs` - Three recovered connected-block mutation paths using staged installation.
- `packages/open-bitcoin-rpc/src/context/mempool_recovery.rs` - Exactly two statement-scoped compatibility allowances pending Plan 06 migration.
- `docs/parity/source-breadcrumbs.json` - Registered the three new first-party Rust files against pinned Knots evidence.
- `docs/metrics/lines-of-code.md` - Refreshed tracked generated LOC evidence.

## Decisions Made

- Bound prepared recovery to the lifecycle epoch observed before expensive staging, then rejected installation if authority changed before consumption.
- Required complete projection validation and empty effect ledgers before any live mutation, so every failure preserves byte-for-byte-equivalent aggregate state.
- Recorded the captured generation as both current and last durable without inventing checkpoint trigger, strength, completion, dirty, or in-flight evidence.
- Kept legacy no-time recovery exclusively for the two existing RPC startup callers, marked it deprecated, and confined warning allowances to those two statements with explicit Plan 06 removal reasons.
- Left `requirements-completed` empty under the established Phase 135 staged-summary convention; the phase verifier owns final MPDUR requirement activation.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Split recovery projection and authority facades at managed source-length limits**

- **Found during:** Task 1 Bright Builds verification
- **Issue:** The complete recovery install logic pushed the existing lifecycle projection and runtime authority modules beyond the managed 628-line source limit.
- **Fix:** Extracted focused `lifecycle_projection/recovery.rs` and `runtime_authority/recovery.rs` child modules and registered their parity breadcrumbs without changing ownership boundaries.
- **Files modified:** `packages/open-bitcoin-node/src/network/lifecycle_projection.rs`, `packages/open-bitcoin-node/src/network/lifecycle_projection/recovery.rs`, `packages/open-bitcoin-node/src/network/runtime_authority.rs`, `packages/open-bitcoin-node/src/network/runtime_authority/recovery.rs`, `docs/parity/source-breadcrumbs.json`
- **Verification:** Bright Builds file-shape checks, parity breadcrumb validation, clippy, all-feature tests, and the repository verifier pass.
- **Committed in:** `1a4e098d`

**2. [Rule 2 - Missing Critical] Added pending-effect introspection before startup replacement**

- **Found during:** Task 1 recovery invariant review
- **Issue:** Atomic startup installation could not prove that checkpoint or peer-effect queues were empty before replacing their projection owners.
- **Fix:** Added read-only `has_pending` evidence and rejected installation before mutation when either ledger contains work.
- **Files modified:** `packages/open-bitcoin-node/src/network/lifecycle_effects.rs`, `packages/open-bitcoin-node/src/network/lifecycle_effects/checkpoint.rs`, `packages/open-bitcoin-node/src/network/lifecycle_projection/recovery.rs`
- **Verification:** Injected failure coverage proves pending-effect rejection leaves the exact live aggregate unchanged.
- **Committed in:** `1a4e098d`

**3. [Rule 3 - Blocking] Confined deprecation allowances to the two planned RPC startup calls**

- **Found during:** Task 2 mandatory clippy verification
- **Issue:** Deprecating the compatibility adapters as planned caused `cargo clippy -- -D warnings` to reject the two callers intentionally deferred to Plan 06.
- **Fix:** Added statement-scoped `#[allow(deprecated)]` attributes with explicit Plan 06 migration reasons; no broader module or crate allowance was introduced.
- **Files modified:** `packages/open-bitcoin-rpc/src/context/mempool_recovery.rs`
- **Verification:** Workspace clippy passes with exactly two legacy calls and exactly two scoped allowances remaining.
- **Committed in:** `64f32419`

**4. [Rule 3 - Blocking] Split recovery metadata tests at the managed source-length boundary**

- **Found during:** Task 2 Bright Builds verification
- **Issue:** The required recovery reconciliation and metadata assertions exceeded the managed source-length limit in `recovery_cases.rs`.
- **Fix:** Extracted a focused `recovery_cases/metadata.rs` child module and registered its parity breadcrumb.
- **Files modified:** `packages/open-bitcoin-node/src/network/tests/recovery_cases.rs`, `packages/open-bitcoin-node/src/network/tests/recovery_cases/metadata.rs`, `docs/parity/source-breadcrumbs.json`
- **Verification:** Bright Builds file-shape checks, parity validation, all focused recovery suites, and the complete verifier pass.
- **Committed in:** `64f32419`

**Total deviations:** 4 auto-fixed (1 missing critical, 3 blocking)

**Impact on plan:** The deviations enforce atomicity evidence, managed code shape, strict-warning compatibility, and parity traceability without widening the recovery architecture or public surface.

## Verification

- `cargo fmt --all` - passed in required pre-commit order for both tasks
- `cargo clippy --all-targets --all-features -- -D warnings` - passed
- `cargo build --all-targets --all-features` - passed
- `cargo test --all-features` - passed
- Focused `recovery_cases` suite - 14 passed, 0 failed
- Focused lifecycle reconciliation suite - 9 passed, 0 failed
- Recovered connected-block removal scenarios - 3 passed, 0 failed
- Timed workspace all-target/all-feature check - passed with both deferred RPC callers compiled
- `bun scripts/bright-builds-check.ts all` - passed
- `bun run scripts/check-parity-breadcrumbs.ts --check` - passed
- `bash scripts/verify.sh` - passed after both tasks; normal pre-commit hooks independently passed the full repository contract
- `git diff --check` - passed

## Known Stubs

None. The modified-file stub scan found no placeholder text or empty values flowing to runtime or operator output.

## Authentication Gates

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan 05 can build shutdown and crash-window behavior on exact current-versus-last-durable generation evidence.
- Plan 06 has exactly two deprecated RPC startup calls to migrate before removing the legacy no-time adapters and their scoped allowances.
- No blocker remains for subsequent Phase 135 plans.

## Self-Check

PASSED

- Summary and all three created Rust files exist at their declared paths.
- Task commits `1a4e098d` and `64f32419` are present in repository history.
- Summary frontmatter contains exactly one opening and closing delimiter pair.
- Modified-file stub and threat-surface scans found no goal-blocking stubs or unplanned trust-boundary surface.
- `git diff --check` passed.

***

*Phase: 135-snapshot-schema-checkpointing-and-recovery*
*Completed: 2026-08-02*
