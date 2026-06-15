---
phase: 75-multi-day-soak-runner-and-evidence-ledger
plan: 02
subsystem: operator-soak-runner
tags: [rust, operator-cli, soak, ledger, reports, tdd]

requires:
  - phase: 75-multi-day-soak-runner-and-evidence-ledger
    provides: Plan 75-01 soak bounds, run-id, ledger, outcome, and report projection contracts
provides:
  - Operator-facing `open-bitcoin soak start`, `resume`, `stop`, and `report` commands
  - Ledger-backed bounded soak observe loop with checkpoint, stop, verdict, and report writes
  - Same-run resume matrix for clean, operator, resource, recovery, and interrupted outcomes
  - Binary-level soak flow coverage for durable run artifacts and report projection semantics
affects: [phase-75, operator-cli, support-evidence, status-snapshot]

tech-stack:
  added: []
  patterns:
    - Clap parser contracts map to Plan 75-01 soak domain types at the operator boundary
    - Bounded soak loop polls shared status snapshots and writes ledger/report projections
    - Same-run resume validates datadir-owned run index before appending evidence

key-files:
  created:
    - .planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-02-SUMMARY.md
  modified:
    - packages/open-bitcoin-cli/src/operator.rs
    - packages/open-bitcoin-cli/src/operator/runtime.rs
    - packages/open-bitcoin-cli/src/operator/soak.rs
    - packages/open-bitcoin-cli/src/operator/soak/ledger.rs
    - packages/open-bitcoin-cli/src/operator/soak/outcome.rs
    - packages/open-bitcoin-cli/src/operator/support.rs
    - packages/open-bitcoin-cli/src/operator/tests.rs
    - packages/open-bitcoin-cli/tests/operator_binary.rs

key-decisions:
  - "Require explicit datadir and network resolution before soak command execution."
  - "Use the existing status collector as soak checkpoint evidence instead of reclassifying daemon sync state locally."
  - "Keep report writes projection-only; `soak report` rewrites reports without appending ledger events."

patterns-established:
  - "Soak command output exposes only ledger/report paths, run id, latest sequence, and final outcome."
  - "Resume reconstructs bounds from the Started event and records interrupted recovery explicitly."

requirements-completed: [SOAK-01, SOAK-02, SOAK-03]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 75-2026-06-14T22-59-23
generated_at: 2026-06-15T02:00:08Z

duration: 29 min
completed: 2026-06-15
---

# Phase 75 Plan 02: Soak Operator Runner Summary

**Operator soak commands with bounded status checkpoints, datadir-owned resume identity, and durable report artifacts**

## Performance

- **Duration:** 29 min
- **Started:** 2026-06-15T01:31:28Z
- **Completed:** 2026-06-15T02:00:08Z
- **Tasks:** 3
- **Files modified:** 8

## Accomplishments

- Added the `open-bitcoin soak` Clap contract for start, resume, stop, and report flows with positive elapsed/checkpoint/disk bounds.
- Wired soak commands into the operator runtime using shared status snapshot collection for checkpoint evidence.
- Implemented bounded ledger-backed start/resume loops, operator stop, projection-only report rewrite, and binary flow coverage.

## Task Commits

Each task was committed atomically, with RED commits for TDD tests:

1. **Task 1: Add the `open-bitcoin soak` Clap contract**
   - `40f95b7` (`test`) add failing soak CLI parser tests
   - `52befcb` (`feat`) add soak operator parser contract
2. **Task 2: Dispatch soak commands to the ledger-backed runner**
   - `883d129` (`test`) add failing soak runtime tests
   - `47313d0` (`feat`) execute soak commands with ledger-backed runner
3. **Task 3: Add operator binary soak flow coverage**
   - `ffba207` (`test`) add soak operator binary flows

## Files Created/Modified

- `packages/open-bitcoin-cli/src/operator.rs` - Adds `OperatorCommand::Soak` and the start/resume/stop/report Clap contracts.
- `packages/open-bitcoin-cli/src/operator/tests.rs` - Adds parser coverage for soak start bounds, zero-bound rejection, and run-id commands.
- `packages/open-bitcoin-cli/src/operator/runtime.rs` - Dispatches soak commands and exposes shared status runtime parts within the operator module.
- `packages/open-bitcoin-cli/src/operator/soak.rs` - Adds soak execution, bounded loop, output rendering, resume validation, stop/report helpers, and runtime tests.
- `packages/open-bitcoin-cli/src/operator/soak/ledger.rs` - Adds append-resume support and datadir access for existing run ledgers.
- `packages/open-bitcoin-cli/src/operator/soak/outcome.rs` - Marks test-only constructors as test-only for clean integration builds.
- `packages/open-bitcoin-cli/src/operator/support.rs` - Marks test-only evidence reexports as test-only for clean integration builds.
- `packages/open-bitcoin-cli/tests/operator_binary.rs` - Adds binary tests for durable soak artifacts, operator stop, report projection, and clean-resume refusal.

## Decisions Made

- Use the resolved operator network as a required soak bound; soak commands fail before execution when the network is absent.
- Use no public-network scripts, DNS seed resolution, daemon spawning, or service-manager calls from the soak runner or tests.
- Keep `soak report` idempotent over the ledger: report files are rewritten, but ledger line count is unchanged.

## Verification

- RED Task 1: `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --lib soak_cli_ --all-features` failed on missing soak parser types and `OperatorCommand::Soak`.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --lib soak_cli_ --all-features` passed: 3 tests.
- RED Task 2: `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --lib soak_runtime_ --all-features` failed on unresolved bounded-loop, resume, stop, report, collector, and clock helpers.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --lib soak_runtime_ --all-features` passed: 5 tests.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --lib soak_ --all-features` passed: 22 tests.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --test operator_binary open_bitcoin_soak_ --all-features` passed: 4 tests.
- Plan acceptance `rg` checks for parser symbols, flags, value enums, dispatch, output labels, JSON fields, runtime loop, resume matrix, and binary tests passed.
- Forbidden soak runner search for `run-live-mainnet-smoke|manual-peer|systemctl|launchctl` returned no matches in `packages/open-bitcoin-cli/src/operator/soak.rs`.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Added temporary runtime exhaustiveness handling during parser task**
- **Found during:** Task 1 (Add the `open-bitcoin soak` Clap contract)
- **Issue:** Adding `OperatorCommand::Soak` made runtime command matches non-exhaustive, so parser-focused tests could not compile.
- **Fix:** Added a narrow `UnsupportedCommand` arm during Task 1, then replaced it with real soak dispatch during Task 2.
- **Files modified:** `packages/open-bitcoin-cli/src/operator/runtime.rs`
- **Verification:** `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --lib soak_cli_ --all-features` passed.
- **Committed in:** `52befcb`, replaced by `47313d0`

**2. [Rule 2 - Missing Critical] Added append-resume ledger constructor**
- **Found during:** Task 2 (Dispatch soak commands to the ledger-backed runner)
- **Issue:** Plan 75-01 ledger creation always started at sequence 1, which would reset sequence numbers for resume and stop appends.
- **Fix:** Added `SoakLedger::resume` with explicit next-sequence preservation.
- **Files modified:** `packages/open-bitcoin-cli/src/operator/soak/ledger.rs`
- **Verification:** `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --lib soak_runtime_ --all-features` passed.
- **Committed in:** `47313d0`

**3. [Rule 3 - Blocking] Cleaned warning-only surfaces for integration build**
- **Found during:** Task 3 (Add operator binary soak flow coverage)
- **Issue:** Integration-test compilation surfaced unused reexport and dead-code warnings that would become clippy failures under the repo verifier.
- **Fix:** Gated test-only support evidence reexports and outcome constructors with `#[cfg(test)]`.
- **Files modified:** `packages/open-bitcoin-cli/src/operator/support.rs`, `packages/open-bitcoin-cli/src/operator/soak/outcome.rs`
- **Verification:** `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --test operator_binary open_bitcoin_soak_ --all-features` passed without those warnings.
- **Committed in:** `ffba207`

---

**Total deviations:** 3 auto-fixed (2 blocking, 1 missing critical)
**Impact on plan:** All fixes were required for compile correctness, resume durability, or clean verifier readiness. No public-network, daemon-spawn, DNS, source-datadir mutation, or service-manager behavior was added.

## Issues Encountered

- The soak runner implementation made `packages/open-bitcoin-cli/src/operator/soak.rs` large. Splitting was considered, but the plan acceptance checks inspect symbols directly in `soak.rs`; the file-size risk is contained to this operator module and can be revisited after Phase 75 acceptance criteria settle.
- Some focused test builds took over a minute due to Rust integration-test rebuilds; all completed successfully.

## Known Stubs

None.

## Threat Flags

None - new CLI argument, status collection, and datadir ledger write surfaces were covered by the plan threat model. Command output is restricted to paths, run id, latest sequence, and final outcome.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Ready for Plan 75-03. The operator command now writes the Plan 75-01 ledger/report artifacts and gives later plans a durable soak lifecycle to extend with richer resource, recovery, and support evidence.

## Self-Check: PASSED

- Summary file exists: `.planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-02-SUMMARY.md`
- Task commit exists: `40f95b7`
- Task commit exists: `52befcb`
- Task commit exists: `883d129`
- Task commit exists: `47313d0`
- Task commit exists: `ffba207`

---
*Phase: 75-multi-day-soak-runner-and-evidence-ledger*
*Completed: 2026-06-15*
