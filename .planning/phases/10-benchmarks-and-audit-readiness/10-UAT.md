---
status: complete
phase: 10-benchmarks-and-audit-readiness
source:
  - .planning/phases/10-benchmarks-and-audit-readiness/10-01-SUMMARY.md
  - .planning/phases/10-benchmarks-and-audit-readiness/10-02-SUMMARY.md
  - .planning/phases/10-benchmarks-and-audit-readiness/10-03-SUMMARY.md
  - .planning/phases/10-benchmarks-and-audit-readiness/10-04-SUMMARY.md
  - .planning/phases/10-benchmarks-and-audit-readiness/10-05-SUMMARY.md
started: 2026-04-26T13:16:19Z
updated: 2026-04-26T13:52:36Z
---

## Current Test

[testing complete]

## Tests

### 1. Benchmark Crate and Registry
expected: From the repo root, `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-bench --all-features` passes, and `bash scripts/run-benchmarks.sh --list` prints the seven benchmark groups: consensus script, block/transaction codec, chainstate, mempool policy, network wire/sync, wallet, and RPC/CLI. The benchmark crate is a first-party workspace member and root Bazel exposes `//:bench`.
result: pass

### 2. Smoke Benchmark Report Generation
expected: `bash scripts/run-benchmarks.sh --smoke --output-dir packages/target/benchmark-reports` passes and writes `open-bitcoin-bench-smoke.json` plus `open-bitcoin-bench-smoke.md`. The JSON report has schema version 1, mode `smoke:1`, seven benchmark groups, no timing threshold status, and `optional_knots_source: null` unless explicit Knots metadata paths are provided.
result: pass

### 3. Bounded Runner and Optional Knots Metadata
expected: The benchmark runner exposes bounded smoke and full modes without wall-clock pass/fail thresholds. Tests for conflicting modes, iteration caps, and optional Knots path recording pass, and explicit `--knots-json` or `--knots-bin` inputs are report metadata only rather than default execution dependencies.
result: pass

### 4. Repo Verify and CI Benchmark Wiring
expected: `bash scripts/verify.sh` passes from the repo root, defaults `OPEN_BITCOIN_BENCHMARK_REPORT_DIR` to `packages/target/benchmark-reports`, runs the bounded smoke benchmark wrapper, and includes `//:bench` in the Bazel smoke build. `.github/workflows/ci.yml` uploads `benchmark-reports` as an artifact without making benchmark timing thresholds release gates.
result: pass

### 5. Parity Checklist Status
expected: `docs/parity/index.json` and `docs/parity/checklist.md` expose one checklist covering all 11 in-scope surfaces, use only the locked status taxonomy (`planned`, `in_progress`, `done`, `deferred`, `out_of_scope`), and mark `benchmarks-audit-readiness` as `done` with evidence links to the benchmark, verify, and release-readiness artifacts.
result: pass

### 6. Deviations and Unknowns Audit View
expected: `docs/parity/deviations-and-unknowns.md` lists intentional deviations, deferred surfaces, suspected unknowns, folded todo audit risks, and follow-up triggers without broadening implementation scope or hiding unresolved audit concerns.
result: pass

### 7. Release Readiness Handoff
expected: `docs/parity/release-readiness.md` provides a reviewer-facing milestone handoff with readiness verdict, complete surfaces, deferrals, known gaps or unknowns, verification commands, benchmark evidence paths, CI artifact notes, reviewer inspection checklist, and stale planning bookkeeping notes.
result: pass

## Summary

total: 7
passed: 7
issues: 0
pending: 0
skipped: 0
blocked: 0

## Gaps

[none yet]
