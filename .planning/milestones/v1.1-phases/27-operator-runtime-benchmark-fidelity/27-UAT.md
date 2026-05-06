---
status: complete
phase: 27-operator-runtime-benchmark-fidelity
source:
  - 27-01-SUMMARY.md
  - 27-02-SUMMARY.md
  - 27-03-SUMMARY.md
started: 2026-05-06T11:40:36Z
updated: 2026-05-06T11:47:08Z
---

## Current Test

[testing complete]

## Tests

### 1. Runtime-Collected Status Benchmark
expected: Running `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-bench -- --nocapture` succeeds. The operator-runtime status benchmark exercises the shared runtime status collector with deterministic tempdir-backed config, log, metrics, and binary paths, and its rendered evidence includes the running daemon state, wallet freshness, metrics availability, and stable JSON sections.
result: pass

### 2. Runtime-Collected Dashboard Benchmark
expected: Running `bash scripts/run-benchmarks.sh --smoke --output-dir packages/target/benchmark-reports` and `bun scripts/check-benchmark-report.ts --report=packages/target/benchmark-reports/open-bitcoin-bench-smoke.json` succeeds. The smoke report records the operator-runtime dashboard benchmark with the `runtime_collected_dashboard_snapshot` fixture label and the operator-runtime group narrative describes runtime-collected status and dashboard evidence.
result: pass

### 3. Repo-Native Verification Closure
expected: Running `bash scripts/verify.sh` succeeds after regenerating the tracked LOC report as needed. The roadmap, requirements ledger, phase summaries, and verification report agree that Phase 27 closed `VER-06` with all three plans complete.
result: pass

## Summary

total: 3
passed: 3
issues: 0
pending: 0
skipped: 0
blocked: 0

## Gaps

[none yet]
