---
phase: 75-multi-day-soak-runner-and-evidence-ledger
reviewed: 2026-06-15T04:57:06Z
depth: standard
files_reviewed: 2
files_reviewed_list:
  - packages/open-bitcoin-cli/src/operator/soak/runtime.rs
  - packages/open-bitcoin-cli/src/operator/soak/tests/runtime.rs
findings:
  critical: 0
  warning: 0
  info: 0
  total: 0
status: clean
---

# Phase 75: Code Review Clean Re-Review 2

**Reviewed:** 2026-06-15T04:57:06Z
**Depth:** standard
**Files Reviewed:** 2
**Status:** clean

## Summary

Reviewed `packages/open-bitcoin-cli/src/operator/soak/runtime.rs` and `packages/open-bitcoin-cli/src/operator/soak/tests/runtime.rs` after commit `94eacba` (`fix(75): allow stopping active resumed soak runs`), focused on the warning from `75-REVIEW-CLEAN.md` and on start/resume/stop regressions.

Material guidance used: repo-local `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md`, `standards/index.md`, `standards/core/architecture.md`, `standards/core/code-shape.md`, `standards/core/operability.md`, `standards/core/verification.md`, `standards/core/testing.md`, and `standards/languages/rust.md`. No project skill indexes were present under `.claude/skills/` or `.agents/skills/`.

The previous warning is resolved. `write_operator_stop` now checks terminal state using `has_terminal_stop_and_verdict(latest_invocation_events(&read.events))`, so a historical terminal verdict no longer blocks stopping a later active resumed invocation. The new regression test `soak_runtime_stop_accepts_active_resume_after_historical_terminal_verdict` covers the `Started -> historical OperatorStop/Verdict -> Resume -> Checkpoint -> operator stop` sequence and verifies the new stop/verdict is appended at the current invocation.

Start/resume/stop semantics were re-reviewed for regressions:

- Start still records the index, writes a `Started` marker, checkpoints, and emits stop/verdict reports for bounded completion.
- Resume still preserves the original run start time and elapsed deadline while appending a fresh `Resume` marker for the current invocation.
- External stop detection still checks for terminal events after sleep and after status collection before appending a checkpoint.
- Stop now rejects only terminal state in the latest invocation, while the existing terminal-verdict rejection remains covered for non-resumed completed runs.

All reviewed files meet quality standards. No issues found.

## Verification

Focused verification passed:

```text
cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --all-features --lib soak_runtime_
```

Result: 13 passed, 0 failed.

---

_Reviewed: 2026-06-15T04:57:06Z_
_Reviewer: the agent (gsd-code-reviewer)_
_Depth: standard_
