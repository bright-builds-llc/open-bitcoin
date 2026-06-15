---
phase: 75-multi-day-soak-runner-and-evidence-ledger
reviewed: 2026-06-15T04:19:27Z
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

# Phase 75: Code Review Rerun Report

**Reviewed:** 2026-06-15T04:19:27Z
**Depth:** standard
**Files Reviewed:** 6
**Status:** issues_found

## Summary

Re-reviewed the Phase 75 code-review fixes for WR-01 through WR-04 in the scoped files. Material guidance used: repo-local `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md`, `standards/index.md`, `standards/core/verification.md`, `standards/core/testing.md`, `standards/core/code-shape.md`, `standards/languages/rust.md`, and `standards/languages/typescript-javascript.md`.

WR-01, WR-02, and WR-03 are resolved. WR-04 is resolved for the already-terminal ledger case covered by the original finding, but the real sleeping runner introduced by WR-01 leaves an in-progress stop race: `soak stop` can now append an operator-stop verdict while the original `soak start` process keeps running and later appends more events.

Targeted verification passed:

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --lib soak_runtime_ --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --test operator_binary open_bitcoin_soak --all-features`
- `bun test scripts/check-phase75-soak-runner.test.ts`
- `bun run scripts/check-phase75-soak-runner.ts`

## Prior Fix Verification

- WR-01: Resolved. `SystemSoakClock::sleep_until` now sleeps until the scheduled wall-clock timestamp, while `SoakTestClock` still fast-forwards deterministic tests.
- WR-02: Resolved. Resume plans now preserve the first `Started` timestamp, and the bounded loop computes the deadline from that original run start.
- WR-03: Resolved. `SoakStopCondition::OperatorStop` is now evaluated through the shared outcome classifier and has table-test coverage.
- WR-04: Partially resolved. The direct already-terminal overwrite path is rejected, but in-progress stop coordination is still unsafe.

## Warnings

### WR-05: Running soak loop can append after an operator stop verdict

**File:** `packages/open-bitcoin-cli/src/operator/soak/runtime.rs:277`

**Issue:** `run_bounded_soak_loop` keeps an in-memory `SoakLedger` and never re-reads the ledger after `sleep_until`. With the production clock now actually sleeping, another process can run `soak stop`, pass the line 379 terminal check, and append an `operator_stop` stop/verdict while the original runner is asleep. When the runner wakes, lines 277-308 append another checkpoint and terminal stop/verdict from stale state, so operator-stop evidence can be followed or superseded by a later runner verdict under the same run id.

**Fix:** Make `soak stop` a coordinated stop request for in-progress runs, and have the runner consume that request before appending the next checkpoint or final verdict. If terminal ledger events remain the stop-command mechanism, guard every runner append by re-reading the ledger under a lock and refusing to append after any terminal stop/verdict.

```rust
if has_terminal_stop_and_verdict(&SoakLedger::read_events(&paths.events_path)?.events) {
    return write_report_projection(layout, run_id);
}

if stop_request_exists(layout, run_id)? {
    final_outcome = Some(SoakOutcomeLabel::OperatorStop);
}
```

---

_Reviewed: 2026-06-15T04:19:27Z_
_Reviewer: the agent (gsd-code-reviewer)_
_Depth: standard_
