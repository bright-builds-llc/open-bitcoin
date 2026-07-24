---
phase: 130-resource-time-and-fee-primitives
plan: "09"
subsystem: rpc-evidence
tags: [rust, mempool, rpc, getmempoolinfo, resource-accounting, fee-roles, privacy]
requires:
  - phase: 130-resource-time-and-fee-primitives
    provides: Typed resource ledger, fee roles, and safe RPC admission timing from Plans 130-01/02/08/11
provides:
  - Authoritative ManagedMempoolInfo projection with capacity enforcement and all four fee roles
  - Truthful getmempoolinfo bytes/usage/maxmempool and fee-floor mappings plus Open Bitcoin extensions
  - Unequal-value integration fixtures proving incremental exclusion and identity-free shared evidence
affects: [phase-131, phase-130-12, operator-rpc, FEEP-01, FEEP-02, FEEP-03, FEEP-05]
tech-stack:
  added: []
  patterns:
    - Fixed MempoolCapacityEnforcement::LegacyVsize label on aggregate pressure and RPC evidence
    - Knots fields keep baseline meanings; Open Bitcoin extensions carry rolling/effective/enforcement detail
key-files:
  created: []
  modified:
    - packages/open-bitcoin-mempool/src/pool/lifecycle.rs
    - packages/open-bitcoin-mempool/src/pool.rs
    - packages/open-bitcoin-node/src/network/types.rs
    - packages/open-bitcoin-node/src/network.rs
    - packages/open-bitcoin-rpc/src/method/node.rs
    - packages/open-bitcoin-rpc/src/dispatch/node.rs
    - packages/open-bitcoin-rpc/src/dispatch/tests.rs
    - packages/open-bitcoin-cli/tests/operator_binary.rs
key-decisions:
  - "Keep getmempoolinfo.bytes=vsize, usage=accounted memory, maxmempool=accounted capacity, and mempoolminfee=effective max(static, rolling)."
  - "Serialize capacityenforcement as fixed legacy_vsize during Phase 130 without claiming accounted-capacity enforcement."
  - "Expose rollingmempoolfee, effectiveadmissionfee, and incrementalrelayfee as distinct exact fields so incremental never contaminates mempoolminfee."
patterns-established:
  - "One authoritative ManagedMempoolInfo snapshot projects typed pressure resources and fees into RPC aggregates."
  - "Shared mempool info remains identity-free; detailed member identities stay on authenticated direct responses."
requirements-completed: []
requirements-addressed: [FEEP-01, FEEP-02, FEEP-03, FEEP-05]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 130-2026-07-23T14-26-46
generated_at: 2026-07-24T05:21:31Z
duration: 47 min
completed: 2026-07-24
---

# Phase 130 Plan 09: Authoritative Mempool Resource and Fee Evidence Summary

**Operator `getmempoolinfo` now exposes exact distinct resource and fee meanings with truthful Phase-130 legacy-vsize enforcement and identity-free shared evidence**

## Performance

- **Duration:** 47 min
- **Started:** 2026-07-24T04:34:09Z
- **Completed:** 2026-07-24T05:21:31Z
- **Tasks:** 1
- **Files modified:** 18

## Accomplishments

- Added `MempoolCapacityEnforcement::LegacyVsize` to pressure summaries and projected it through `ManagedMempoolInfo` and RPC `capacityenforcement`.
- Mapped Knots-compatible `bytes`/`usage`/`maxmempool`/`mempoolminfee`/`minrelaytxfee`/`incrementalrelayfee` and Open Bitcoin extensions `rollingmempoolfee`/`effectiveadmissionfee`.
- Added node and RPC unequal-value fixtures with nontrivial script payload, non-default capacity/static/incremental/rolling values, and forbidden-identity serialization checks.
- Migrated CLI, bench, and operator-binary fake RPC callers to the final typed response shape and covered the rolling-fee setter in pure-core tests for the coverage gate.
- Passed focused node/RPC suites, timed workspace `--all-targets` check, and full normal-hook verification.

## TDD Execution

- **Task 1 RED/GREEN:** Focused unequal-value and fee-equation tests were authored with the authoritative projection and caller migration so the workspace remained compilable at the commit boundary; RED-only commits could not land through hook-owned full verify.

## Task Commits

1. **Task 1: Project authoritative resource and fee evidence** - `a3e75105` (feat)

**Plan metadata:** `6019a384` (docs: complete plan)

## Files Created/Modified

- `packages/open-bitcoin-mempool/src/pool/lifecycle.rs` - Capacity-enforcement enum and pressure-summary field.
- `packages/open-bitcoin-mempool/src/pool.rs` - Public rolling-fee setter and exports.
- `packages/open-bitcoin-mempool/src/pool/tests/fee_cases.rs` - Uses setter for coverage.
- `packages/open-bitcoin-mempool/src/pool/tests/lifecycle_cases.rs` - Asserts legacy-vsize enforcement label.
- `packages/open-bitcoin-mempool/tests/parity.rs` - Constructs and labels capacity enforcement.
- `packages/open-bitcoin-node/src/mempool.rs` / `network.rs` / `network/types.rs` - Authoritative projection and rolling-fee seam.
- `packages/open-bitcoin-node/src/network/tests.rs` - Unequal resource/fee role fixture.
- `packages/open-bitcoin-rpc/src/method/node.rs` / `dispatch/node.rs` - Response fields and mappings.
- `packages/open-bitcoin-rpc/src/dispatch/tests.rs` - Identity-free unequal-value RPC evidence test.
- `packages/open-bitcoin-cli` / `open-bitcoin-bench` fixtures - Final response constructors and fake RPC JSON.

## Decisions Made

- Preserve Knots field meanings and carry Phase-130 truth about legacy-vsize enforcement through a dedicated extension field.
- Keep `mempoolminfee` equal to derived effective admission and independent from incremental relay fee.
- Install rolling floors for evidence fixtures through an explicit setter rather than leaving private-field test mutations as the only path.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Remove `ConsensusParams::clone()` under clippy deny-warnings**
- **Found during:** Task 1
- **Issue:** RPC fixture used `.clone()` on a `Copy` type and failed pre-commit clippy.
- **Fix:** Pass `consensus` by copy.
- **Files modified:** `packages/open-bitcoin-rpc/src/dispatch/tests.rs`
- **Verification:** `cargo clippy -p open-bitcoin-rpc --all-targets -- -D warnings`
- **Committed in:** `a3e75105`

**2. [Rule 2 - Missing critical functionality] Migrate operator-binary fake getmempoolinfo JSON**
- **Found during:** Task 1
- **Issue:** CLI binary tests deserialized live-ish fake RPC JSON missing the new fields.
- **Fix:** Extend the fake `getmempoolinfo` payload with incremental/rolling/effective/enforcement fields.
- **Files modified:** `packages/open-bitcoin-cli/tests/operator_binary.rs`
- **Verification:** Focused `operator_binary` status/support tests
- **Committed in:** `a3e75105`

**3. [Rule 2 - Missing critical functionality] Cover rolling-fee setter and enforcement label for llvm-cov**
- **Found during:** Task 1
- **Issue:** New production lines were uncovered and failed the pure-core coverage gate.
- **Fix:** Route fee-case rolling installs through `set_rolling_mempool_fee_rate` and assert `capacity_enforcement.as_str()`.
- **Files modified:** `packages/open-bitcoin-mempool/src/pool/tests/fee_cases.rs`, `lifecycle_cases.rs`, `tests/parity.rs`
- **Verification:** Focused mempool pressure tests and full verify coverage step
- **Committed in:** `a3e75105`
***
**Total deviations:** 3 auto-fixed (1 Rule 1, 2 Rule 2)
**Impact on plan:** Required for correctness, caller completeness, and the repository coverage gate. No scope creep.

## Issues Encountered

None beyond the auto-fixed pre-commit findings above.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan 130-12 can document stable RPC/resource/fee anchors and release-boundary language.
- Plan 130-13 can mutation-test the unequal-value and no-claim guards and run the full repository gate.
- Phase 131 remains the owner of accounted-capacity enforcement changes.

## Self-Check: PASSED

- FOUND: `.planning/phases/130-resource-time-and-fee-primitives/130-09-SUMMARY.md`
- FOUND: commit `a3e75105`
***
*Phase: 130-resource-time-and-fee-primitives*
*Completed: 2026-07-24*
