---
status: complete
phase: 32-benchmark-wrapper-list-mode-hygiene
source:
  - .planning/milestones/v1.1-phases/32-benchmark-wrapper-list-mode-hygiene/32-01-SUMMARY.md
  - .planning/milestones/v1.1-phases/32-benchmark-wrapper-list-mode-hygiene/32-02-SUMMARY.md
started: 2026-05-07T12:03:48Z
updated: 2026-05-07T12:21:18Z
---

## Current Test

[testing complete]

## Tests

### 1. Benchmark List Mode
expected: From the repo root, `bash scripts/run-benchmarks.sh --list` exits successfully and prints the registered benchmark groups instead of failing with an uninitialized `cargo_args[@]` or other Bash array error.
result: pass
evidence: "`bash scripts/run-benchmarks.sh --list` exited 0 and printed benchmark groups including `consensus-script`, `operator-runtime`, `wallet-rescan`, and `rpc-cli`."

### 2. Cargo Benchmark List Equivalent
expected: The repo-local Cargo workflow `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-bench --bin open-bitcoin-bench -- --list` exits successfully and shows the same benchmark registry surface that the wrapper delegates to.
result: pass
evidence: "`cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-bench --bin open-bitcoin-bench -- --list` exited 0 and printed the same benchmark group list as the wrapper."

### 3. Smoke Benchmark Path Still Writes Reports
expected: From the repo root, `bash scripts/run-benchmarks.sh --smoke --output-dir packages/target/benchmark-reports` exits successfully and writes the normal smoke benchmark JSON and Markdown reports.
result: pass
evidence: "`bash scripts/run-benchmarks.sh --smoke --output-dir packages/target/benchmark-reports` exited 0 and wrote `open-bitcoin-bench-smoke.json` plus `open-bitcoin-bench-smoke.md`."

### 4. Repo Verification Guards List Mode
expected: `bash scripts/verify.sh` exits successfully and includes the benchmark wrapper list-mode guard before the existing smoke benchmark wrapper check, so future regressions in `--list` fail through the repo-native verification contract.
result: pass
evidence: "`bash scripts/verify.sh` exited 0 in 1m 51.604s, ran `open-bitcoin-bench --list`, ran the smoke benchmark wrapper, validated the smoke report, and completed the Bazel smoke build."

## Summary

total: 4
passed: 4
issues: 0
pending: 0
skipped: 0
blocked: 0

## Gaps

[none yet]
