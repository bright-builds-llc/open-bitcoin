---
phase: 130-resource-time-and-fee-primitives
plan: "06"
subsystem: node-admission
tags: [rust, mempool, metadata, explicit-time, runtime-authority]
requires:
  - phase: 130-resource-time-and-fee-primitives
    provides: Explicit-time managed admission and semantic lifecycle deltas from Plan 130-05
provides:
  - Explicit acceptance time and typed relay intent at every node-owned local admission caller
  - Deterministic admission, serving, compact-receive, and lifecycle fixtures
  - Fail-closed RPC compatibility boundary retained for Plan 130-11
affects: [phase-130-plan-11, phase-134, node-admission, rpc-admission]
tech-stack:
  added: []
  patterns:
    - Node fixtures supply deterministic admission facts through the managed authority
    - Relay intent is requested only for fixtures exercising local relay behavior
key-files:
  created: []
  modified:
    - packages/open-bitcoin-node/src/network/tests.rs
    - packages/open-bitcoin-node/src/network/tests/admission_bridge_cases.rs
    - packages/open-bitcoin-node/src/network/tests/compact_receive_cases.rs
    - packages/open-bitcoin-node/src/network/tests/mempool_lifecycle_cases.rs
    - packages/open-bitcoin-node/src/network/tests/relay_serving_cases.rs
key-decisions:
  - "Use RelayIntent::Requested only for local relay and serving fixtures; use NotRequested for admission, compact-candidate, status, and lifecycle setup."
  - "Treat deterministic fixture timestamps as explicit admission facts while leaving live RPC clock sampling and compatibility removal to Plan 130-11."
  - "Preserve lifecycle-delta-driven cache effects without restoring outcome-vector reclassification."
patterns-established:
  - "Node-owned local admission tests call submit_local_transaction_outcome_at with nonzero deterministic time and typed intent."
requirements-completed: []
requirements-addressed: [FEEP-03, FEEP-04, FEEP-05]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 130-2026-07-23T14-26-46
generated_at: 2026-07-24T00:11:24Z
duration: 14 min
completed: 2026-07-24
---

# Phase 130 Plan 06: Node Admission Caller Migration Summary

**Every node-owned local admission fixture now supplies deterministic nonzero acceptance time and typed relay intent while the sole old RPC caller remains behind the Plan-130-11-owned fail-closed adapter**

## Performance

- **Duration:** 14 min
- **Started:** 2026-07-23T23:57:31Z
- **Completed:** 2026-07-24T00:11:24Z
- **Tasks:** 1
- **Files modified:** 6

## Accomplishments

- Migrated admission, serving, compact-receive, lifecycle, status, and in-memory relay fixtures from no-time compatibility calls to `submit_local_transaction_outcome_at`.
- Supplied `RelayIntent::Requested` only where local relay behavior is under test and `NotRequested` for non-relay fixture setup.
- Replaced the node-side fail-closed compatibility regression with exact known-time, local-origin, and not-requested metadata assertions.
- Preserved existing attempt outcomes, lifecycle-delta cache consequences, compact candidates, serving classifications, and duplicate metadata immutability.
- Audited old and explicit authority callers across the workspace and retained the sole production old caller in `open-bitcoin-rpc/src/context/network.rs` for Plan 130-11.
- Passed 24 focused admission tests, all 471 node tests plus the node doctest, the timed workspace all-target compile gate, formatting, diff checks, and the full repository verifier.

## Task Commits

1. **Task 1: Migrate node callers and preserve RPC compatibility** - `f27cdfc8` (feat)

The TDD RED regression proved that a no-time node call stored `LegacyUnknown`; the GREEN migration stored exact `Known(PolicyTime(45))`, `Local`, and `NotRequested` metadata. The completed task was committed atomically after all normal hooks passed.

## Files Created/Modified

- `packages/open-bitcoin-node/src/network/tests.rs` - Explicit requested/not-requested time facts for broad managed-network fixtures.
- `packages/open-bitcoin-node/src/network/tests/admission_bridge_cases.rs` - Explicit-time outcome fixtures and exact local metadata assertions.
- `packages/open-bitcoin-node/src/network/tests/compact_receive_cases.rs` - Non-relay explicit-time mempool candidate setup.
- `packages/open-bitcoin-node/src/network/tests/mempool_lifecycle_cases.rs` - Non-relay explicit-time lifecycle setup without cache reclassification.
- `packages/open-bitcoin-node/src/network/tests/relay_serving_cases.rs` - Relay-requested explicit-time serving fixtures.
- `docs/metrics/lines-of-code.md` - Hook-regenerated tracked LOC freshness.

## Decisions Made

- Relay and serving fixtures request relay because relay behavior is part of their contract; fixtures that only need canonical mempool membership do not.
- Deterministic test timestamps are authoritative fixture facts, not substitutes for a production clock.
- The deprecated no-time methods remain unchanged in the admission bridge and runtime authority because Plan 130-11 must migrate the RPC shell and remove both compatibility layers atomically.

## ASVS Mitigations

- **ASVS-130-V1/V13:** Inventoried old and `_at` callers across node and RPC targets and passed the exact timed workspace `cargo check --workspace --all-targets` compatibility gate.
- **ASVS-130-V4/V8:** Kept transaction detail inside existing direct responses, preserved exact privacy-sensitive metadata, and added no shared identity-bearing evidence.
- **ASVS-130-V11:** Replaced sentinel-free node calls with typed relay intent and deterministic nonzero time while preserving outcome and lifecycle assertions.

## Deviations from Plan

None - plan implementation executed exactly as written.

## Issues Encountered

- The plan's written negative scan used non-recursive `--glob '!network/…'` exclusions, which still matched the two required compatibility files from the repository root. Verification used the semantically equivalent recursive exclusions `!**/network/admission_bridge.rs` and `!**/network/runtime_authority.rs`; this proved no node-owned caller remains while preserving the required definitions.
- The initially attempted `cargo fmt` argument order passed `--check` to rustfmt instead of Cargo. The corrected `cargo fmt --manifest-path packages/Cargo.toml --all --check` command passed.

## Authentication Gates

None.

## Known Stubs

None.

## Threat Flags

None - the change migrates deterministic in-crate callers and introduces no network endpoint, authentication path, file-access pattern, schema, dependency, or transport behavior.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan 130-11 retains one clearly audited RPC caller to migrate before removing the no-time admission adapters.
- Plan 130-07 can proceed with explicit block/reorg contexts independently of this caller migration.
- No blockers remain.

## Self-Check: PASSED

- Summary file and task commit `f27cdfc8` exist.
- Node source scans find no old local admission caller outside the retained compatibility definitions.
- Stub and threat-surface scans found no blocking placeholder or new security-relevant surface.
- Focused admission tests, all node tests, the timed workspace all-target gate, and the full repository verifier passed.

---
*Phase: 130-resource-time-and-fee-primitives*
*Completed: 2026-07-24*
