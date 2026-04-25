---
status: complete
phase: 01-workspace-baseline-and-guardrails
source:
  - .planning/phases/01-workspace-baseline-and-guardrails/01-01-SUMMARY.md
  - .planning/phases/01-workspace-baseline-and-guardrails/01-02-SUMMARY.md
  - .planning/phases/01-workspace-baseline-and-guardrails/01-03-SUMMARY.md
  - .planning/phases/01-workspace-baseline-and-guardrails/01-04-SUMMARY.md
started: 2026-04-25T12:02:41Z
updated: 2026-04-25T12:13:35Z
---

## Current Test

[testing complete]

## Tests

### 1. Pinned Knots Baseline
expected: From the repo root, `git submodule status packages/bitcoin-knots` reports a materialized submodule and `git -C packages/bitcoin-knots describe --tags --exact-match` reports `v29.3.knots20260210`.
result: pass

### 2. Root Workspace and Bazel Targets
expected: `cargo metadata --manifest-path packages/Cargo.toml --format-version 1 --no-deps` succeeds and the repo-root Bazel labels `//:core` and `//:node` resolve/build successfully through the first-party package targets.
result: pass

### 3. Repo Verification Contract
expected: Running `bash scripts/verify.sh` from the repo root exits successfully and visibly runs the repo-owned guardrails: pure-core policy, file-length and panic checks, format, clippy, build, tests, benchmark smoke, Bazel smoke build, and pure-core coverage.
result: pass

### 4. Pure-Core Dependency Guardrail
expected: `bash scripts/check-pure-core-deps.sh` exits successfully on the current tree, and `scripts/pure-core-crates.txt` lists the crates that must remain free of forbidden I/O, runtime, randomness, and third-party network dependencies.
result: pass

### 5. Parity Ledger and Contributor Guidance
expected: `docs/parity/index.json` records the Knots baseline and deviation ledger scaffold, while `README.md`, `CONTRIBUTING.md`, and `AGENTS.md` explain submodule sync, repo verification, and intentional parity-difference updates.
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
