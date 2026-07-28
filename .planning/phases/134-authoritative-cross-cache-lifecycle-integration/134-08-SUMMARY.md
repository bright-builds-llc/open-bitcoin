---
phase: 134-authoritative-cross-cache-lifecycle-integration
plan: "08"
subsystem: node-network-lifecycle
tags: [rust, lifecycle-authority, effects, receipts, bounded-ledgers]

requires:
  - phase: 134-authoritative-cross-cache-lifecycle-integration
    plan: "07"
    provides: authoritative maintenance and reorg lifecycle projection
provides:
  - family-specific peer relay and snapshot write effect contracts
  - bounded pending and completion ledgers with deterministic eviction
  - shared authoritative prepare and completion dispatch through LifecycleCommand
affects: [phase-134-production-routing, peer-relay, mempool-snapshots, lifecycle-verification]

tech-stack:
  added: []
  patterns:
    - consume non-clone family capabilities to mint identity-complete success receipts
    - record achieved effect identity before freshness classification
    - expose thin public facades that construct LifecycleCommand and invoke the sole dispatcher

key-files:
  created:
    - packages/open-bitcoin-node/src/network/lifecycle_effects.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/effects.rs
    - packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/effects.rs
  modified:
    - packages/open-bitcoin-node/src/network.rs
    - packages/open-bitcoin-node/src/network/lifecycle_projection.rs
    - packages/open-bitcoin-node/src/network/runtime_authority.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/lifecycle.rs
    - packages/open-bitcoin-rpc/src/dispatch.rs
    - docs/parity/source-breadcrumbs.json

key-decisions:
  - "Allocate an effect ID only after its bounded reservation succeeds so pressure does not consume identity space."
  - "Check completed ledgers before freshness and retain achieved stale completions while preserving every newer authoritative field."
  - "Keep effect-facing methods as command-construction facades and map lifecycle-effect authority failures to the existing internal RPC error boundary."

patterns-established:
  - "Family-safe completion: peer and snapshot capabilities, receipts, IDs, and identity tuples cannot cross-complete."
  - "Bounded effect truth: peer pending/completed ledgers cap at 128; snapshot pending caps at 1 and completed identity caps at 2."

requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 134-2026-07-28T01-41-12
generated_at: 2026-07-28T14:40:10Z

duration: 1h 17m
completed: 2026-07-28
---

# Phase 134 Plan 08: Authoritative Lifecycle Effect Dispatcher Summary

**Peer relay and mempool snapshot effects now use family-safe, bounded capabilities and receipts whose preparation and completion both pass through the sole authoritative lifecycle dispatcher.**

## Performance

- **Duration:** 1h 17m
- **Started:** 2026-07-28T13:23:03Z
- **Completed:** 2026-07-28T14:40:10Z
- **Tasks:** 2
- **Files modified:** 13

## Accomplishments

- Added distinct peer and snapshot effect IDs, identities, non-`Clone` capabilities, consuming success receipts, and fixed-cap pending/completed ledgers.
- Embedded bounded peer-session and snapshot-write effect state in `ManagedPeerNetwork` without a generic bus, durable journal, outbox, schema, or unbounded history.
- Routed relay preparation, snapshot preparation, peer completion, and snapshot completion through `LifecycleCommand` and `apply_lifecycle_command`.
- Classified completions as exact `Applied`, `AchievedButStale`, or `AlreadyApplied` outcomes, with duplicate detection preceding freshness checks.
- Preserved newer dirty generations, peer provenance, sessions, and unbroadcast state when an achieved external effect completes against stale authority.
- Added contract, cap, eviction, current/stale/duplicate, public-facade, and compile-fail wrong-family coverage.

## Task Commits

Each task was committed atomically:

1. **Task 1: Define family identities, exact caps, capabilities, receipts, and ledgers** - `eca802c6`
2. **Task 2: Route effect preparation and completion through apply_lifecycle_command** - `fc66c20b`

## Files Created/Modified

- `packages/open-bitcoin-node/src/network/lifecycle_effects.rs` - Defines family-specific effect identities, capabilities, receipts, completion outcomes, exact caps, and bounded ledgers.
- `packages/open-bitcoin-node/src/network/lifecycle_projection.rs` - Carries real effect receipts and preparation inputs through shared lifecycle command variants.
- `packages/open-bitcoin-node/src/network/runtime_authority/effects.rs` - Provides thin public prepare/complete facades and compile-fail family-safety proofs.
- `packages/open-bitcoin-node/src/network/runtime_authority/lifecycle.rs` - Reserves effect capabilities and records current, stale, or duplicate completion through the sole dispatcher.
- `packages/open-bitcoin-node/src/network/runtime_authority.rs` - Registers effect facades and the typed lifecycle-effect authority failure.
- `packages/open-bitcoin-node/src/network.rs` - Owns and reexports the bounded effect state.
- `packages/open-bitcoin-node/src/network/relay_serving.rs` - Initializes the new bounded authority state at the existing construction boundary.
- `packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/effects.rs` - Proves family contracts, exact caps, deterministic eviction, facade routing, and completion semantics.
- `packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases.rs` - Registers the effect contract scenarios.
- `packages/open-bitcoin-node/src/network/lifecycle_projection/tests.rs` - Updates command fixtures for the identity-complete effect inputs and receipts.
- `packages/open-bitcoin-rpc/src/dispatch.rs` - Preserves exhaustive authority error projection at the RPC boundary.
- `docs/parity/source-breadcrumbs.json` - Registers new source and test paths against the pinned Knots anchors.
- `docs/metrics/lines-of-code.md` - Refreshed by the mandatory normal repository hooks.

## Decisions Made

- Effect IDs advance only after the corresponding bounded ledger accepts a reservation; rejected work does not consume an ID.
- Completed identity is checked and recorded before epoch, generation, or session freshness, so achieved external truth is retained even when it cannot mutate newer authority.
- Snapshot completion clears dirty state only when the exact captured persistence generation remains current.
- Public effect methods remain ergonomic facades over `LifecycleCommand`; they contain no independent `mutate` or `try_mutate` closure.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Initialized effect ledgers at the existing network construction boundary**

- **Found during:** Task 1 compilation
- **Issue:** Adding bounded ledgers and peer-session generation to `ManagedPeerNetwork` required its existing relay-serving constructor to initialize the new fields.
- **Fix:** Initialized all ledgers with their exact fixed caps and started the peer-session generation at its defined initial value.
- **Files modified:** `packages/open-bitcoin-node/src/network/relay_serving.rs`
- **Verification:** Task 1 mandatory Rust gates, focused contracts, Bright Builds, and the full normal hook passed.
- **Committed in:** `eca802c6`

**2. [Rule 3 - Blocking] Preserved exhaustive RPC projection for the new authority error**

- **Found during:** Task 2 mandatory Clippy
- **Issue:** The new `ManagedNetworkAuthorityError::LifecycleEffect` variant made the existing exhaustive RPC authority-error match incomplete.
- **Fix:** Mapped lifecycle-effect preparation failures to the existing redacted internal RPC failure boundary.
- **Files modified:** `packages/open-bitcoin-rpc/src/dispatch.rs`
- **Verification:** Task 2 mandatory Rust gates, focused completion suite, Bright Builds, and the full normal hook passed.
- **Committed in:** `fc66c20b`

**3. [Rule 3 - Blocking] Canonicalized parity breadcrumbs before both normal-hook commits**

- **Found during:** Task 1 and Task 2 normal-hook verification
- **Issue:** New first-party Rust paths required canonical inline breadcrumb expansions in addition to registry entries.
- **Fix:** Ran the repository breadcrumb writer and committed the canonical pinned-Knots anchor blocks.
- **Files modified:** `packages/open-bitcoin-node/src/network/lifecycle_effects.rs`, `packages/open-bitcoin-node/src/network/runtime_authority/effects.rs`, `packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/effects.rs`, `docs/parity/source-breadcrumbs.json`
- **Verification:** `bun scripts/check-parity-breadcrumbs.ts` and both normal hooks passed.
- **Committed in:** `eca802c6`, `fc66c20b`

**4. [Rule 3 - Blocking] Deferred requirement activation to lifecycle verification**

- **Found during:** Plan metadata preparation
- **Issue:** Intermediate Phase 134 summaries cannot activate `MPLIFE-01` or `MPLIFE-04` before the phase has a lifecycle-valid verification artifact.
- **Fix:** Preserved both requirements as pending and left `requirements-completed` empty.
- **Files modified:** `.planning/phases/134-authoritative-cross-cache-lifecycle-integration/134-08-SUMMARY.md`
- **Verification:** Documentation reconciliation remains consistent with Phase 134 requirements pending.
- **Committed in:** Plan metadata commit

**Total deviations:** 4 auto-fixed (4 blocking)
**Impact on plan:** The adjustments satisfy existing construction, parity, exhaustive-error, and traceability contracts without adding transport I/O, scheduling, recovery, schema, or a second mutation authority.

## Issues Encountered

- Task 2 Clippy exposed test-only ledger inspection helpers in production builds; gating them with `#[cfg(test)]` retained focused evidence without widening the runtime API.
- The new authority module initially exceeded the Bright Builds file-length limit; moving its effect error conversion beside the effect facades restored the existing module boundary.
- The full normal Task 2 hook took 20m 47s because macOS delayed several test-binary startups in dyld; liveness checks showed a single Cargo verifier, and every suite eventually passed.
- TDD RED states were not committed separately because the mandatory normal hook requires a fully green repository; each task was committed atomically after its RED/GREEN cycle and complete verification.

## Known Stubs

None. The modified production and test files contain no goal-blocking placeholder, empty runtime projection, TODO, or FIXME.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Production socket and Fjall executors can consume the bounded owned capabilities and return family-specific receipts without introducing a second authority path.
- Current, stale, duplicate, cap, eviction, and family-safety behavior is locked down before external I/O routing begins.
- `MPLIFE-01` and `MPLIFE-04` remain pending until Phase 134 verification produces the lifecycle-valid completion artifact.
- No blockers remain.

## Self-Check: PASSED

- Summary and every created effect source/test file exist.
- Task commits `eca802c6` and `fc66c20b` are present in repository history.
- Focused contract and completion suites, all mandatory Rust gates, Bright Builds checks, parity breadcrumbs, apply-boundary checker, and both full normal hooks pass.
- Compile-fail doctests prove peer and snapshot families cannot cross-complete.
- Stub and threat-surface scans found no goal-blocking placeholder or unplanned trust-boundary surface.
- `git diff --check` passes and Markdown frontmatter uses exactly two delimiters.

***

*Phase: 134-authoritative-cross-cache-lifecycle-integration*
*Completed: 2026-07-28*
