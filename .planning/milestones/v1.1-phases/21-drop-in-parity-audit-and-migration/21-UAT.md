---
status: complete
phase: 21-drop-in-parity-audit-and-migration
source: [21-01-SUMMARY.md, 21-02-SUMMARY.md, 21-03-SUMMARY.md]
started: 2026-05-04T09:28:02.107Z
updated: 2026-05-04T09:29:24.687Z
---

## Current Test

[testing complete]

## Tests

### 1. Migration Command Discovery and Help
expected: `open-bitcoin migrate plan --help` should exist under the operator CLI, describe a dry-run migration planner, and expose the dedicated `--source-datadir` selector instead of hiding migration behind onboarding text.
result: pass

### 2. Explanation-First Migration Plan
expected: Running `open-bitcoin migrate plan` against a detected source install should print explanation-first output covering benefits, tradeoffs, unsupported surfaces, rollback and backup expectations, relevant intentional-difference notices, and grouped follow-up actions for config, datadir/files, service, wallet, and operator steps.
result: pass

### 3. Secret-Safe Read-Only Planning
expected: The dry-run planner should leave source files unchanged and must redact sensitive material such as cookie contents or wallet secrets instead of echoing them into terminal output.
result: pass

### 4. Ambiguous Source Detection
expected: If the source install is ambiguous or incomplete, the planner should stop short of a confident migration recipe and clearly mark the situation as manual review required rather than pretending the source was identified.
result: pass

### 5. Migration Audit and Contributor Docs
expected: Contributor-facing docs should point to `open-bitcoin migrate plan` and the drop-in migration parity page, with intentional differences recorded in the parity ledger and wording that avoids claiming full replacement parity.
result: pass

## Summary

total: 5
passed: 5
issues: 0
pending: 0
skipped: 0
blocked: 0

## Gaps

[none yet]
