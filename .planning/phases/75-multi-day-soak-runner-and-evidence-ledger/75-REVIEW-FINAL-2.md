---
phase: 75-multi-day-soak-runner-and-evidence-ledger
reviewed: 2026-06-15T04:40:28Z
depth: standard
files_reviewed: 2
files_reviewed_list:
  - packages/open-bitcoin-cli/src/operator/soak/runtime.rs
  - packages/open-bitcoin-cli/src/operator/soak/tests/runtime.rs
findings:
  critical: 0
  warning: 2
  info: 0
  total: 2
status: issues_found
---

# Phase 75: Code Review Final Re-Review 2

**Reviewed:** 2026-06-15T04:40:28Z
**Depth:** standard
**Files Reviewed:** 2
**Status:** issues_found

## Summary

Reviewed the requested final sequence-aware external-stop guard and resume regression fix in `packages/open-bitcoin-cli/src/operator/soak/runtime.rs` and `packages/open-bitcoin-cli/src/operator/soak/tests/runtime.rs`.

Material guidance used: repo-local `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md`, `standards/index.md`, `standards/core/architecture.md`, `standards/core/code-shape.md`, `standards/core/verification.md`, `standards/core/testing.md`, and `standards/languages/rust.md`.

The warning from `75-REVIEW-FINAL.md` is resolved for the covered historical-terminal resume regression. `run_bounded_soak_loop` now captures the current `Started` or `Resume` event sequence and `existing_terminal_result_after_sequence` only treats later terminal stop/verdict pairs as external stops for that invocation. The added regression test proves a resume after a historical `operator_stop` appends new resume/checkpoint/final evidence instead of returning the old terminal projection.

Focused verification passed:

```text
cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --all-features --lib soak_runtime_
```

Result: 10 passed, 0 failed.

Two warning-level edge cases remain in start/resume/stop semantics.

## Warnings

### WR-01: External stop during status collection can still be followed by a runner checkpoint

**File:** `packages/open-bitcoin-cli/src/operator/soak/runtime.rs:281`

**Issue:** The external-stop guard runs before `collector.collect()`, but status collection can do effectful work and take time. If `soak stop` appends `Stop` and `Verdict` after the guard at lines 281-284 but before the runner appends the checkpoint at lines 288-290, the runner still writes a checkpoint using its stale in-memory sequence. That can leave checkpoint evidence after a terminal verdict, and can duplicate event sequences because the runner's `SoakLedger` does not learn about the stop command's appended events.

**Fix:** Re-check for a later terminal stop/verdict after status collection and immediately before checkpoint append. For full multi-process safety, protect the read/check/append sequence with a ledger-level lock or an append helper that derives the next sequence while holding exclusive access.

```rust
let snapshot = collector.collect();
if let Some(result) =
    existing_terminal_result_after_sequence(layout, run_id, invocation_marker_sequence)?
{
    return Ok(result);
}
let status = checkpoint_status_from_snapshot(&snapshot);
ledger
    .append_event(checkpoint_at, SoakLedgerEvent::Checkpoint { status })
    .map_err(runtime_error)?;
```

Add a regression collector that writes `write_operator_stop` from `collect()` before returning a snapshot, then assert the runner returns the external stop projection without appending a post-terminal checkpoint or duplicate sequence.

### WR-02: Interrupted resumed invocation can be misclassified from an older terminal verdict

**File:** `packages/open-bitcoin-cli/src/operator/soak/runtime.rs:372`

**Issue:** `validate_resume_plan` classifies resume state using `latest_verdict(&read.events)` and `has_terminal_stop_and_verdict(&read.events)` across the whole ledger. A run can have an older same-run terminal stop, then a later `Resume` and checkpoint, then the resumed invocation can terminate unexpectedly before writing its own stop/verdict. The next resume still sees the older `operator_stop`, `resource_stop`, or `recovery_stop` verdict and an older terminal pair, so it returns `interrupted_prior_run: false` instead of recording interrupted recovery evidence for the latest invocation.

**Fix:** Classify resumability from the events at or after the latest invocation marker (`Started` or `Resume`), while still preserving the first `Started` event for original bounds and elapsed-budget calculations.

```rust
let latest_invocation_index = read
    .events
    .iter()
    .rposition(|envelope| {
        matches!(
            &envelope.event,
            SoakLedgerEvent::Started { .. } | SoakLedgerEvent::Resume { .. }
        )
    })
    .unwrap_or(0);
let invocation_events = &read.events[latest_invocation_index..];

let interrupted_prior_run = if !has_terminal_stop_and_verdict(invocation_events) {
    true
} else {
    match latest_verdict(invocation_events) {
        Some(SoakOutcomeLabel::OperatorStop)
        | Some(SoakOutcomeLabel::ResourceStop)
        | Some(SoakOutcomeLabel::RecoveryStop) => false,
        Some(SoakOutcomeLabel::UnexpectedTermination) | None => true,
        Some(SoakOutcomeLabel::CleanCompletion) => {
            return Err(OperatorRuntimeError::InvalidRequest {
                message: format!(
                    "soak run {run_id} latest verdict clean_completion cannot be resumed"
                ),
            });
        }
        Some(SoakOutcomeLabel::DiagnosedBlocker) => {
            return Err(OperatorRuntimeError::InvalidRequest {
                message: format!(
                    "soak run {run_id} ended with diagnosed_blocker and cannot be resumed as the same run"
                ),
            });
        }
    }
};
```

Add a regression ledger with `Started`, historical `operator_stop` stop/verdict, `Resume { interrupted_prior_run: false }`, and a checkpoint without a later terminal pair. `validate_resume_plan` should return `interrupted_prior_run: true` and the next sequence after the checkpoint.

---

_Reviewed: 2026-06-15T04:40:28Z_
_Reviewer: the agent (gsd-code-reviewer)_
_Depth: standard_
