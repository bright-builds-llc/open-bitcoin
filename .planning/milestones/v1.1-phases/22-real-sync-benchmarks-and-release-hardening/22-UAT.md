---
status: complete
phase: 22-real-sync-benchmarks-and-release-hardening
source: [22-01-SUMMARY.md, 22-02-SUMMARY.md, 22-03-SUMMARY.md]
started: 2026-05-04T11:13:52Z
updated: 2026-05-05T00:46:29Z
---

## Current Test

[testing complete]

## Tests

### 1. Operator Guide Handoff
expected: The root README and parity docs should now hand operators off to `docs/operator/runtime-guide.md` as the practical workflow entrypoint. You should see source-built install, onboarding, service, status, dashboard, migration, benchmark, and known-limitation guidance described as current shipped surfaces rather than future-only placeholders.
result: pass

### 2. Source-Built Install And Onboarding Docs
expected: `docs/operator/runtime-guide.md` should give a coherent source-built path with `git submodule update --init --recursive`, `bun --version`, `bash scripts/install-git-hooks.sh`, workspace Cargo build, and `bash scripts/verify.sh`, plus clear onboarding behavior such as `--approve-write`, `--detect-existing`, JSONC ownership, and the rule that onboarding does not rewrite `bitcoin.conf`.
result: pass

### 3. Service, Status, And Dashboard Runtime Surface
expected: The runtime guide should describe the shipped operator commands for `open-bitcoin service`, `open-bitcoin status`, `open-bitcoin dashboard`, and `open-bitcoin sync status|pause|resume`, including dry-run versus `--apply` behavior, RPC bootstrap expectations, and truthful stopped-node or `Unavailable` reporting when live data is missing.
result: pass

### 4. Smoke Benchmark Workflow And Report Semantics
expected: The benchmark docs should present `bash scripts/run-benchmarks.sh --smoke` as the default bounded offline path, document that smoke uses the `debug` profile and stays threshold-free, and show the expanded Phase 22 runtime groups such as `sync-runtime`, `storage-recovery`, `operator-runtime`, and `wallet-rescan` with reports written under `packages/target/benchmark-reports`.
result: pass

### 5. Benchmark Report Validator
expected: Running `bun scripts/check-benchmark-report.ts --report=packages/target/benchmark-reports/open-bitcoin-bench-smoke.json` after a smoke benchmark run should validate the report structure, required benchmark groups, required Phase 22 runtime case ids, allowed durability metadata, and the expected smoke `debug` profile contract.
result: pass

### 6. Release-Readiness And Parity Closeout
expected: `docs/parity/release-readiness.md`, `docs/parity/index.json`, and the Phase 22 parity catalog should treat `real-sync-benchmarks` and `operator-runtime-release-hardening` as completed evidence surfaces, keep packaged installs, migration apply mode, default public-network verification, and hosted dashboard work explicitly deferred, and point reviewers to repo-owned evidence instead of checked-in benchmark artifacts.
result: pass

## Summary

total: 6
passed: 6
issues: 0
pending: 0
skipped: 0
blocked: 0

## Gaps

[none yet]
