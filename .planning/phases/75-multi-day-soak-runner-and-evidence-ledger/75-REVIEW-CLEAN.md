---
phase: 75-multi-day-soak-runner-and-evidence-ledger
reviewed: 2026-06-15T04:49:49Z
depth: standard
files_reviewed: 2
files_reviewed_list:
  - packages/open-bitcoin-cli/src/operator/soak/runtime.rs
  - packages/open-bitcoin-cli/src/operator/soak/tests/runtime.rs
findings:
  critical: 0
  warning: 1
  info: 0
  total: 1
status: issues_found
---

# Phase 75: Code Review Clean Re-Review

**Reviewed:** 2026-06-15T04:49:49Z
**Depth:** standard
**Files Reviewed:** 2
**Status:** issues_found

## Summary

Reviewed `packages/open-bitcoin-cli/src/operator/soak/runtime.rs` and `packages/open-bitcoin-cli/src/operator/soak/tests/runtime.rs` after commit `9aa9479` (`fix(75): close soak stop resume edge cases`), focused on the two warnings from `75-REVIEW-FINAL-2.md` and start/resume/stop regressions.

Material guidance used: repo-local `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md`, `standards/index.md`, `standards/core/architecture.md`, `standards/core/code-shape.md`, `standards/core/verification.md`, `standards/core/testing.md`, and `standards/languages/rust.md`. No project skill indexes were present under `.claude/skills/` or `.agents/skills/`.

The two prior warnings are covered in the current implementation:

- External stop during status collection: `run_bounded_soak_loop` now re-checks for a terminal stop/verdict after `collector.collect()` and before appending a checkpoint, and `soak_runtime_runner_returns_external_stop_written_during_collect` verifies no post-terminal checkpoint is appended.
- Interrupted resumed invocation classification: `validate_resume_plan` now classifies from `latest_invocation_events`, and `soak_runtime_resume_plan_treats_latest_unterminated_invocation_as_interrupted` verifies a later unterminated resume is treated as interrupted despite an older operator-stop verdict.

Focused verification passed:

```text
cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --all-features --lib soak_runtime_
```

Result: 12 passed, 0 failed.

One warning-level stop-path regression remains.

## Warnings

### WR-01: Stop rejects an active resumed invocation after a historical terminal verdict

**File:** `packages/open-bitcoin-cli/src/operator/soak/runtime.rs:434`

**Issue:** `write_operator_stop` still checks `has_terminal_stop_and_verdict(&read.events)` across the entire ledger. A same-run resume after `operator_stop`, `resource_stop`, or `recovery_stop` is now valid, but that resumed invocation inherits the older terminal stop/verdict in the ledger. If the operator runs `soak stop` while the resumed invocation is active, this whole-ledger check rejects it as "already has a terminal verdict" even though the latest `Resume` invocation has not written a terminal verdict.

**Fix:** Apply the same latest-invocation slicing used by `validate_resume_plan` before deciding whether the current invocation is already terminal, and add a regression test with `Started`, historical `OperatorStop` stop/verdict, `Resume`, `Checkpoint`, then `write_operator_stop`.

```rust
let invocation_events = latest_invocation_events(&read.events);
if has_terminal_stop_and_verdict(invocation_events) {
    return Err(OperatorRuntimeError::InvalidRequest {
        message: format!("soak run {run_id} already has a terminal verdict"),
    });
}
```

---

_Reviewed: 2026-06-15T04:49:49Z_
_Reviewer: the agent (gsd-code-reviewer)_
_Depth: standard_
