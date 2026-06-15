---
phase: 75-multi-day-soak-runner-and-evidence-ledger
plan: 03
subsystem: deterministic-soak-coverage
tags: [rust, sync-runtime, operator-cli, soak, ledger, synthetic-tests]

requires:
  - phase: 75-multi-day-soak-runner-and-evidence-ledger
    provides: Plan 75-01 soak ledger, outcome, and report projection contracts
  - phase: 71-resource-bounds-and-durable-restart-resume
    provides: Durable sync runtime reopen and synthetic long-chain test patterns
provides:
  - Deterministic 96-block synthetic sync soak coverage with no public peers or DNS seeds
  - Same-datadir reopen proof that connected progress is preserved and not requested again
  - Shared status evidence coverage for resource-stop recovery and no-progress diagnosis
  - Typed ledger replay coverage for interrupted resume, clean-completion resume refusal, and resource-stop reports
affects: [phase-75, sync-runtime, operator-cli, soak-ledger, parity-breadcrumbs]

tech-stack:
  added: []
  patterns:
    - Child sync test module reusing parent scripted runtime fixtures through `super::*`
    - Typed soak ledger event histories with explicit synthetic timestamps

key-files:
  created:
    - packages/open-bitcoin-node/src/sync/tests/soak.rs
    - .planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-03-SUMMARY.md
  modified:
    - packages/open-bitcoin-node/src/sync/tests.rs
    - packages/open-bitcoin-cli/src/operator/soak/tests.rs
    - docs/parity/source-breadcrumbs.json

key-decisions:
  - "Keep long-run soak proof in deterministic Rust tests using scripted loopback transports and resolvers."
  - "Replay soak ledgers through typed constructors instead of raw JSON fixtures."
  - "Use shared sync status fields for resource-stop evidence rather than soak-local reclassification."

patterns-established:
  - "Phase 75 synthetic sync soak uses a 96-block fixture chunked through bounded peer scripts."
  - "Interrupted resume evidence is represented by a typed Resume event with interrupted_prior_run=true."

requirements-completed: [SOAK-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 75-2026-06-14T22-59-23
generated_at: 2026-06-15T02:17:55Z

duration: 13 min
completed: 2026-06-15
---

# Phase 75 Plan 03: Synthetic Soak Coverage Summary

**Deterministic synthetic soak proof for long-run sync, durable reopen, resource-stop status evidence, and ledger replay outcomes**

## Performance

- **Duration:** 13 min
- **Started:** 2026-06-15T02:04:48Z
- **Completed:** 2026-06-15T02:17:55Z
- **Tasks:** 2
- **Files modified:** 5

## Accomplishments

- Added `phase75_synthetic_soak_*` node tests covering 96-block target-height sync, same-datadir reopen/resume evidence, and shared resource-stop status projection.
- Added `soak_synthetic_*` CLI tests covering interrupted-run replay, clean-completion resume refusal, and resource-stop report preservation.
- Registered the new sync soak test module in parity breadcrumbs.

## Task Commits

Each task was committed atomically:

1. **Task 1: Add synthetic long-run sync soak tests** - `7c945e3` (`test`)
2. **Task 2: Add synthetic ledger replay tests for interruption and resume evidence** - `b11db0d` (`test`)

## Files Created/Modified

- `packages/open-bitcoin-node/src/sync/tests.rs` - Registers the `soak` child test module.
- `packages/open-bitcoin-node/src/sync/tests/soak.rs` - Adds the Phase 75 deterministic sync soak tests.
- `packages/open-bitcoin-cli/src/operator/soak/tests.rs` - Adds synthetic typed ledger replay tests.
- `docs/parity/source-breadcrumbs.json` - Adds breadcrumb coverage for the new sync soak test module.
- `.planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-03-SUMMARY.md` - Records execution results.

## Decisions Made

- Used a child sync test module so the 96-block soak fixture stays separate from the large existing sync test file while still reusing established private scripted fixtures.
- Kept all synthetic timestamps explicit in test constants and event records.
- Used `SoakLedger`, `SoakLedgerEvent`, `SoakLedgerEventEnvelope`, and report projections rather than hand-written JSON strings.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase75_synthetic_soak_ --all-features` passed: 3 tests.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --lib soak_synthetic_ --all-features` passed: 3 tests.
- `bun run scripts/check-parity-breadcrumbs.ts --check` passed for 246 Rust files.
- Acceptance `rg` checks for `mod soak;`, deterministic sync bounds, all new test names, explicit ledger timestamps/outcomes, and forbidden sync-test terms passed.
- `cargo fmt --manifest-path packages/Cargo.toml --all` passed.

## Deviations from Plan

None - plan outputs were implemented as specified.

## Issues Encountered

- The TDD-marked tasks added coverage over existing runtime and ledger behavior. The first focused test runs passed, so each task landed as a single test commit rather than a RED/GREEN production-code pair.

## Known Stubs

None.

## Threat Flags

None - this plan added deterministic tests and parity metadata only; no new network endpoint, auth path, schema boundary, or production file-access surface was introduced.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Ready for Plan 75-04. SOAK-04 now has replayable local proof for sync control flow and ledger replay without public-network access, service-manager action, or wall-clock multi-day waits.

## Self-Check: PASSED

- Summary file exists: `.planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-03-SUMMARY.md`
- Task commit exists: `7c945e3`
- Task commit exists: `b11db0d`

---
*Phase: 75-multi-day-soak-runner-and-evidence-ledger*
*Completed: 2026-06-15*
