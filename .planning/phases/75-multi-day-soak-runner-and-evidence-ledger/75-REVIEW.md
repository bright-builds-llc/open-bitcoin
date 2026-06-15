---
phase: 75-multi-day-soak-runner-and-evidence-ledger
reviewed: 2026-06-15T03:55:08Z
depth: standard
files_reviewed: 33
files_reviewed_list:
  - README.md
  - docs/architecture/operator-observability.md
  - docs/architecture/status-snapshot.md
  - docs/metrics/lines-of-code.md
  - docs/operator/runtime-guide.md
  - docs/parity/README.md
  - docs/parity/catalog/chainstate.md
  - docs/parity/catalog/operator-runtime-release-hardening.md
  - docs/parity/catalog/p2p.md
  - docs/parity/checklist.md
  - docs/parity/index.json
  - docs/parity/source-breadcrumbs.json
  - packages/open-bitcoin-cli/src/operator.rs
  - packages/open-bitcoin-cli/src/operator/runtime.rs
  - packages/open-bitcoin-cli/src/operator/soak.rs
  - packages/open-bitcoin-cli/src/operator/soak/ledger.rs
  - packages/open-bitcoin-cli/src/operator/soak/outcome.rs
  - packages/open-bitcoin-cli/src/operator/soak/report.rs
  - packages/open-bitcoin-cli/src/operator/soak/runtime.rs
  - packages/open-bitcoin-cli/src/operator/soak/runtime/helpers.rs
  - packages/open-bitcoin-cli/src/operator/soak/tests.rs
  - packages/open-bitcoin-cli/src/operator/soak/tests/runtime.rs
  - packages/open-bitcoin-cli/src/operator/support.rs
  - packages/open-bitcoin-cli/src/operator/support/render.rs
  - packages/open-bitcoin-cli/src/operator/support/tests.rs
  - packages/open-bitcoin-cli/src/operator/tests.rs
  - packages/open-bitcoin-cli/tests/operator_binary.rs
  - packages/open-bitcoin-node/src/sync/tests.rs
  - packages/open-bitcoin-node/src/sync/tests/soak.rs
  - scripts/check-panic-sites.sh
  - scripts/check-phase75-soak-runner.test.ts
  - scripts/check-phase75-soak-runner.ts
  - scripts/verify.sh
findings:
  critical: 0
  warning: 4
  info: 0
  total: 4
status: issues_found
---

# Phase 75: Code Review Report

**Reviewed:** 2026-06-15T03:55:08Z
**Depth:** standard
**Files Reviewed:** 33
**Status:** issues_found

## Summary

Reviewed the Phase 75 soak runner, durable ledger/report projections, support-bundle soak evidence, deterministic checker, docs/parity entries, and verification wiring. Material guidance used: repo-local `AGENTS.md`, `AGENTS.bright-builds.md`, `standards/index.md`, `standards/core/verification.md`, `standards/core/testing.md`, `standards/core/code-shape.md`, `standards/languages/rust.md`, and `standards/languages/typescript-javascript.md`.

The support evidence redaction path looks intentionally projection-based and the Phase 75 checker/test pass locally, but the runner has blocking behavior issues around elapsed-time execution, resume duration accounting, operator-stop handling, and terminal ledger mutation.

## Warnings

### WR-01: Production elapsed-time soak runs do not wait

**File:** `packages/open-bitcoin-cli/src/operator/soak/runtime.rs:173`

**Issue:** `SystemSoakClock::sleep_until` is an empty method, while `run_bounded_soak_loop` advances `checkpoint_at` through the configured elapsed-time window on every iteration. A documented three-day run such as `--elapsed-time-seconds 259200 --checkpoint-interval-seconds 300` will append all checkpoints and final verdict in a tight loop instead of sampling wall-clock behavior over multiple days.

**Fix:** Implement real sleeping for the production clock and keep deterministic fast-forwarding only in `SoakTestClock`. Also adjust binary tests that currently use elapsed-time completion with `60` seconds so they do not become slow; use a fast stop condition for CLI smoke tests and cover elapsed-time scheduling in unit tests.

```rust
impl SoakClock for SystemSoakClock {
    fn now_unix_seconds(&mut self) -> u64 {
        current_unix_seconds()
    }

    fn sleep_until(&mut self, scheduled_unix_seconds: u64) {
        let now = current_unix_seconds();
        if scheduled_unix_seconds > now {
            std::thread::sleep(std::time::Duration::from_secs(
                scheduled_unix_seconds - now,
            ));
        }
    }
}
```

### WR-02: Resume starts a fresh elapsed-time budget instead of preserving same-run duration

**File:** `packages/open-bitcoin-cli/src/operator/soak/runtime.rs:230`

**Issue:** `run_bounded_soak_loop` always sets `started_at = clock.now_unix_seconds()` and then computes `deadline = started_at + bounds.elapsed_time_seconds`. `execute_soak_resume` passes the original bounds back into that same loop, so an interrupted same-run resume gets a brand-new elapsed-time window rather than the remaining time from the first `started` event. A run interrupted near the end of a three-day soak can therefore run another full three days under the same run id.

**Fix:** Carry the original run start time into the resume plan and compute the elapsed deadline from the first `Started` event for resume mode. If the original deadline has already passed, append a resume checkpoint and terminal verdict immediately instead of extending the run.

```rust
let invocation_started_at = clock.now_unix_seconds();
let run_started_at = match mode {
    SoakLoopMode::Start => invocation_started_at,
    SoakLoopMode::Resume { .. } => first_started_at(layout, run_id)?
        .unwrap_or(invocation_started_at),
};
let deadline = run_started_at.saturating_add(bounds.elapsed_time_seconds);
let mut checkpoint_at = invocation_started_at.min(deadline);
```

### WR-03: `operator-stop` stop condition is parsed but ignored

**File:** `packages/open-bitcoin-cli/src/operator/soak/runtime/helpers.rs:58`

**Issue:** `SoakStopCondition::OperatorStop` is grouped with no-op match arms in `evaluate_stop_outcome`. The lower-level classifier can already identify `operator_stop` and `operator_cancellation`, but a run configured with `--stop-condition operator-stop` will not stop on that evidence until the default deadline fallback.

**Fix:** Evaluate `OperatorStop` the same way resource and recovery stop conditions are evaluated, and add a unit case to `soak_runtime_target_height_resource_recovery_and_status_verdict_stop_conditions`.

```rust
SoakStopCondition::OperatorStop => {
    let outcome = outcome_for_snapshot(snapshot);
    if matches!(outcome, SoakOutcomeLabel::OperatorStop) {
        return Some(outcome);
    }
}
```

### WR-04: `soak stop` can overwrite a terminal ledger outcome

**File:** `packages/open-bitcoin-cli/src/operator/soak/runtime.rs:348`

**Issue:** `write_operator_stop` reads the existing ledger and blindly appends a new `operator_stop` stop/verdict pair. If the run already ended with `clean_completion`, `diagnosed_blocker`, `resource_stop`, `recovery_stop`, or `unexpected_termination`, the report projection will treat the later operator-stop verdict as final. That mutates historical evidence and also changes later resume eligibility because `validate_resume_plan` keys off the latest verdict.

**Fix:** Reject `soak stop` when the ledger already has a terminal stop/verdict pair. If a separate in-progress stop request is still needed, model it as an explicit stop-request file or locked ledger transition so a running process cannot race the append sequence.

```rust
let read = SoakLedger::read_events(&paths.events_path).map_err(runtime_error)?;
if has_terminal_stop_and_verdict(&read.events) {
    return Err(OperatorRuntimeError::InvalidRequest {
        message: format!("soak run {run_id} already has a terminal verdict"),
    });
}
```

---

_Reviewed: 2026-06-15T03:55:08Z_
_Reviewer: the agent (gsd-code-reviewer)_
_Depth: standard_
