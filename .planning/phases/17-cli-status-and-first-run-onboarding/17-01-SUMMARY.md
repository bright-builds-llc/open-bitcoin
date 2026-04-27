---
phase: 17-cli-status-and-first-run-onboarding
plan: "01"
subsystem: cli-operator-contracts
tags: [cli, operator, config, detection, onboarding, status, breadcrumbs]

requires:
  - phase: 13-operator-runtime-foundations
    provides: "operator CLI routing and config/status foundation contracts"
  - phase: 17-cli-status-and-first-run-onboarding
    provides: "Phase 17 context, research, and execution plan"
provides:
  - "typed operator contracts for config precedence, read-only detection, onboarding write decisions, runtime outcomes, and status collector inputs"
  - "parity breadcrumb coverage for new operator Rust source and test files"
  - "operator contract tests preserving open-bitcoin-cli compatibility routing"
affects: [CLI-03, CLI-05, CLI-06, operator, status, onboarding]

tech-stack:
  added: []
  patterns:
    - "contract-first operator modules under packages/open-bitcoin-cli/src/operator"
    - "read-only detection evidence separated from filesystem mutation"
    - "onboarding write decisions distinguish no-write, proposed write, and approved write"

key-files:
  created:
    - packages/open-bitcoin-cli/src/operator/config.rs
    - packages/open-bitcoin-cli/src/operator/config/tests.rs
    - packages/open-bitcoin-cli/src/operator/detect.rs
    - packages/open-bitcoin-cli/src/operator/detect/tests.rs
    - packages/open-bitcoin-cli/src/operator/onboarding.rs
    - packages/open-bitcoin-cli/src/operator/onboarding/tests.rs
    - packages/open-bitcoin-cli/src/operator/runtime.rs
    - packages/open-bitcoin-cli/src/operator/status.rs
    - packages/open-bitcoin-cli/src/operator/status/tests.rs
  modified:
    - packages/open-bitcoin-cli/src/operator.rs
    - packages/open-bitcoin-cli/src/operator/tests.rs
    - docs/parity/source-breadcrumbs.json
    - docs/metrics/lines-of-code.md

key-decisions:
  - "Deferred direct OpenBitcoinStatusSnapshot binding to Plan 17-04, where the CLI dependency on open-bitcoin-node is introduced."
  - "Kept detection contracts read-only and credential-safe by carrying paths and source names, never secret values."
  - "Used explicit none breadcrumbs for Open Bitcoin-only support contracts without defensible Knots source anchors."

patterns-established:
  - "Operator-facing contract modules expose typed evidence and decisions before shell adapters are wired."
  - "Config precedence source names remain stable snake_case values for user-facing output and tests."
  - "New breadcrumb-scoped Rust files are registered in docs/parity/source-breadcrumbs.json and generated with top-of-file parity blocks."

requirements-completed: [CLI-03, CLI-05, CLI-06]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 17-2026-04-26T23-56-00
generated_at: 2026-04-27T01:04:07Z

duration: 24min
completed: 2026-04-27
---

# Phase 17 Plan 01 Summary

**Typed operator contracts for config precedence, read-only detection, onboarding writes, runtime outcomes, and status collection inputs**

## Performance

- **Duration:** 24 min
- **Started:** 2026-04-27T00:40:44Z
- **Completed:** 2026-04-27T01:04:07Z
- **Tasks:** 3
- **Files modified:** 12

## Accomplishments

- Added operator child modules for config, detection, onboarding, runtime, and status contracts.
- Preserved `open-bitcoin-cli` compatibility routing while adding contract tests for `open-bitcoin` operator routing.
- Registered parity breadcrumbs for all new operator Rust source/test files and generated the required top-of-file blocks.

## Task Commits

Each task was committed atomically:

1. **Task 1: Create config and detection contracts** - `f3afa87` (feat)
2. **Task 2: Create status, onboarding, and runtime contracts** - `91e6224` (feat)
3. **Task 3: Register breadcrumbs and contract tests** - `3a583b2` (test)

## Files Created/Modified

- `packages/open-bitcoin-cli/src/operator.rs` - Declares the new operator child modules.
- `packages/open-bitcoin-cli/src/operator/config.rs` - Defines config source precedence and path-resolution evidence contracts.
- `packages/open-bitcoin-cli/src/operator/detect.rs` - Defines read-only Core/Knots detection evidence contracts.
- `packages/open-bitcoin-cli/src/operator/onboarding.rs` - Defines onboarding prompt answers, plans, messages, and write decisions.
- `packages/open-bitcoin-cli/src/operator/runtime.rs` - Defines typed operator command outcome and runtime error contracts.
- `packages/open-bitcoin-cli/src/operator/status.rs` - Defines status requests, collector inputs, detection evidence, and live RPC input contracts.
- `packages/open-bitcoin-cli/src/operator/tests.rs` - Adds cross-module route and contract coverage.
- `docs/parity/source-breadcrumbs.json` - Registers all new operator source/test files.
- `docs/metrics/lines-of-code.md` - Updated by the repository metrics hook.

## Decisions Made

- Deferred final shared status snapshot binding until Plan 17-04 so this plan does not add the `open-bitcoin-node` production dependency early.
- Modeled onboarding writes as explicit no-write, proposed-write, and approved-write states to keep interactive and non-interactive behavior idempotent.
- Kept detection contracts free of mutation-capable operations and credential values.

## Deviations from Plan

None - plan scope was executed as written. The executor process did not write the summary after committing its task changes, so this summary was recovered by the orchestrator from the plan, commits, and targeted verification output.

## Issues Encountered

- The executor session was closed after the task commits landed but before summary generation. Worktree state and targeted checks were reviewed before writing this recovered summary.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli operator::` passed.
- `bun run scripts/check-parity-breadcrumbs.ts --check` passed.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

The shared operator contract surface is ready for Plan 17-02 config resolution and Plan 17-03 read-only installation detection. Plan 17-04 still owns binding status rendering to the shared node snapshot after adding the required production dependency.

## Self-Check: PASSED

- Plan acceptance criteria are satisfied.
- Targeted tests and breadcrumb verification pass.
- No unrelated worktree changes were present when this summary was created.

---
*Phase: 17-cli-status-and-first-run-onboarding*
*Completed: 2026-04-27*
