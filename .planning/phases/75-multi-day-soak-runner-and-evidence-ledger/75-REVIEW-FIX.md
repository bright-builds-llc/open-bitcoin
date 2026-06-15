---
phase: 75-multi-day-soak-runner-and-evidence-ledger
fixed_at: 2026-06-15T04:12:41Z
review_path: .planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-REVIEW.md
iteration: 1
findings_in_scope: 4
fixed: 4
skipped: 0
status: all_fixed
---

# Phase 75: Code Review Fix Report

**Fixed at:** 2026-06-15T04:12:41Z
**Source review:** `.planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-REVIEW.md`
**Iteration:** 1

**Summary:**
- Findings in scope: 4
- Fixed: 4
- Skipped: 0

## Fixed Issues

### WR-01: Production elapsed-time soak runs do not wait

**Files modified:** `packages/open-bitcoin-cli/src/operator/soak/runtime.rs`, `packages/open-bitcoin-cli/tests/operator_binary.rs`
**Commit:** 93aea9b
**Applied fix:** Implemented real production clock sleeping and moved CLI smoke starts to seeded target-height completion so tests stay fast.

### WR-02: Resume starts a fresh elapsed-time budget instead of preserving same-run duration

**Files modified:** `packages/open-bitcoin-cli/src/operator/soak/runtime.rs`, `packages/open-bitcoin-cli/src/operator/soak/tests/runtime.rs`
**Commit:** 958ac04
**Applied fix:** Preserved the first `Started` timestamp in resume plans and computed elapsed deadlines from the original run start, with tests for remaining-budget and already-expired resumes.

### WR-03: `operator-stop` stop condition is parsed but ignored

**Files modified:** `packages/open-bitcoin-cli/src/operator/soak/runtime/helpers.rs`, `packages/open-bitcoin-cli/src/operator/soak/tests/runtime.rs`
**Commit:** 9490efe
**Applied fix:** Evaluated `SoakStopCondition::OperatorStop` through the shared outcome classifier and added operator-stop evidence to the stop-condition table test.

### WR-04: `soak stop` can overwrite a terminal ledger outcome

**Files modified:** `packages/open-bitcoin-cli/src/operator/soak/runtime.rs`, `packages/open-bitcoin-cli/src/operator/soak/tests/runtime.rs`, `packages/open-bitcoin-cli/tests/operator_binary.rs`, `scripts/check-phase75-soak-runner.ts`, `scripts/check-phase75-soak-runner.test.ts`
**Commit:** 5e6d68a
**Applied fix:** Rejected operator stop appends when the ledger already has a terminal stop/verdict pair, added unit and binary rejection coverage, and updated Phase 75 checker anchors.

## Skipped Issues

None.

## Verification

- `cargo fmt --manifest-path packages/Cargo.toml --all`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --lib soak_runtime_ --all-features`
- `bun run scripts/check-phase75-soak-runner.ts`

---

_Fixed: 2026-06-15T04:12:41Z_
_Fixer: the agent (gsd-code-fixer)_
_Iteration: 1_
