---
phase: 75-multi-day-soak-runner-and-evidence-ledger
reviewed: 2026-06-15T04:31:52Z
depth: standard
files_reviewed: 6
files_reviewed_list:
  - packages/open-bitcoin-cli/src/operator/soak/runtime.rs
  - packages/open-bitcoin-cli/src/operator/soak/runtime/helpers.rs
  - packages/open-bitcoin-cli/src/operator/soak/tests/runtime.rs
  - packages/open-bitcoin-cli/tests/operator_binary.rs
  - scripts/check-phase75-soak-runner.ts
  - scripts/check-phase75-soak-runner.test.ts
findings:
  critical: 0
  warning: 1
  info: 0
  total: 1
status: issues_found
---

# Phase 75: Code Review Final Report

**Reviewed:** 2026-06-15T04:31:52Z
**Depth:** standard
**Files Reviewed:** 6
**Status:** issues_found

## Summary

Re-reviewed the final Phase 75 soak stop/race fix in commit `60c397e` (`fix(75): guard soak runner after external stop`). Material guidance used: repo-local `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md`, `standards/index.md`, `standards/core/architecture.md`, `standards/core/code-shape.md`, `standards/core/verification.md`, `standards/core/testing.md`, `standards/languages/rust.md`, and `standards/languages/typescript-javascript.md`.

WR-05 is resolved for the covered start-loop race where an external `soak stop` appends a terminal stop/verdict while the runner is asleep. The new guard re-reads the ledger before checkpoint append and before final stop/verdict append, and the added regression test covers that path.

However, the same guard now treats historical terminal stop/verdict pairs as current-run external stops. That breaks the D-11 same-run resume path for prior `operator_stop`, `resource_stop`, and `recovery_stop` outcomes.

Targeted verification passed:

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --all-features --lib soak_runtime_runner_returns_existing_terminal_verdict_after_external_stop`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --all-features --lib soak_runtime_`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --all-features --test operator_binary open_bitcoin_soak`
- `bun test scripts/check-phase75-soak-runner.test.ts`
- `bun run scripts/check-phase75-soak-runner.ts`

## Warnings

### WR-01: Same-run resume exits on historical terminal stop/verdict

**File:** `packages/open-bitcoin-cli/src/operator/soak/runtime.rs:279`

**Issue:** `run_bounded_soak_loop` now calls `existing_terminal_result` unconditionally after every sleep. That helper checks whether the ledger contains any stop and any verdict, not whether those terminal events were written after the current invocation started. For a permitted same-run resume after `operator_stop`, `resource_stop`, or `recovery_stop`, `validate_resume_plan` allows the resume, then the loop appends `Resume` and immediately returns the old projection because the historical stop/verdict pair is still present. The resume command reports the prior terminal outcome and performs no new checkpointing or soak work.

**Fix:** Capture the sequence number of the current invocation marker (`Started` or `Resume`) from `append_event`, and only treat terminal stop/verdict pairs with sequences greater than that marker as external stops for this invocation. Add a regression test that writes a prior `resource_stop` or `operator_stop` stop/verdict, runs the loop in `Resume` mode, and asserts the resumed invocation appends new checkpoint evidence instead of returning immediately.

```rust
let invocation_marker_sequence = ledger
    .append_event(
        invocation_started_at,
        SoakLedgerEvent::Resume {
            interrupted_prior_run,
        },
    )
    .map_err(runtime_error)?
    .sequence;

if let Some(result) =
    existing_terminal_result_after_sequence(layout, run_id, invocation_marker_sequence)?
{
    return Ok(result);
}
```

---

_Reviewed: 2026-06-15T04:31:52Z_
_Reviewer: the agent (gsd-code-reviewer)_
_Depth: standard_
