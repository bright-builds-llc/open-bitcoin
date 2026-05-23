---
status: complete
phase: 40-live-mainnet-smoke-docs-and-parity-closeout
source: [.planning/phases/40-live-mainnet-smoke-docs-and-parity-closeout/40-01-SUMMARY.md]
started: 2026-05-23T02:33:51.446Z
updated: 2026-05-23T02:40:41.277Z
---

## Current Test

[testing complete]

## Tests

### 1. Offline Live-Smoke Regression
expected: Running `bash scripts/test-run-live-mainnet-smoke.sh` from the repo root completes successfully without public-network access. The script may be quiet on success because it asserts generated reports and preflight failure messaging internally.
result: pass

### 2. Opt-In Live Mainnet Smoke Command
expected: Running `bun run scripts/run-live-mainnet-smoke.ts --datadir=/tmp/open-bitcoin-mainnet-smoke-uat --timeout-seconds=60 --poll-seconds=5` from the repo root builds the daemon and CLI, launches `open-bitcoind` only in explicit `mainnet-ibd` mode, probes `getblockchaininfo`, and exits with a written report. In a network-limited environment, the report records `no_progress` with zero-outbound-peer guidance instead of failing opaquely.
result: pass

### 3. Generated Evidence Reports
expected: After the live smoke command runs, `packages/target/live-mainnet-smoke-reports/open-bitcoin-live-mainnet-smoke.json` and `.md` exist. The reports include run provenance, daemon or sync status details, the observed progress outcome, and actionable guidance when no outbound peers or no header/block movement were observed.
result: pass

### 4. Docs And Parity Closeout Truth
expected: `README.md`, `docs/operator/runtime-guide.md`, `docs/parity/checklist.md`, `docs/parity/index.json`, `docs/parity/release-readiness.md`, and `docs/parity/deviations-and-unknowns.md` describe the opt-in live-smoke workflow as shipped, keep it outside default verification, and still avoid production-node, production-funds, packaged-service, or full Knots-equivalence claims.
result: pass

## Summary

total: 4
passed: 4
issues: 0
pending: 0
skipped: 0
blocked: 0

## Gaps

[none yet]
