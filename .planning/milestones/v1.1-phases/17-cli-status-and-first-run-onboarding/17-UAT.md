---
status: complete
phase: 17-cli-status-and-first-run-onboarding
source:
  - 17-01-SUMMARY.md
  - 17-02-SUMMARY.md
  - 17-03-SUMMARY.md
  - 17-04-SUMMARY.md
  - 17-05-SUMMARY.md
started: 2026-05-03T14:00:51Z
updated: 2026-05-03T14:28:25Z
---

## Current Test

[testing complete]

## Tests

### 1. Operator Binary Help Surface
expected: Running the repo operator binary help shows a separate `open-bitcoin` operator command surface with operator-oriented subcommands such as `status`, `config`, and `onboard`.
result: pass

### 2. Stopped-Node Human Status
expected: Running `open-bitcoin status --format human --no-color` against a fresh temp datadir succeeds and prints support-oriented labels including Daemon, Version, Datadir, Config, Network, Chain, Sync, Peers, Mempool, Wallet, Service, Logs, Metrics, and Health, with no ANSI color sequences.
result: pass

### 3. Stopped-Node JSON Status
expected: Running `open-bitcoin status --format json` against a fresh temp datadir succeeds and returns stable top-level sections including node, config, service, sync, peers, mempool, wallet, logs, metrics, health_signals, and build. The node state should be `stopped`, and unavailable live fields should be explicit instead of omitted.
result: pass

### 4. Config Paths Source Report
expected: Running `open-bitcoin config paths --format human` reports Config, Bitcoin config, Datadir, Logs, and Metrics paths plus the source precedence string `cli_flags > environment > open_bitcoin_jsonc`.
result: pass

### 5. First-Run Onboarding Creates Open Bitcoin JSONC Only
expected: Running non-interactive onboarding with `--approve-write --detect-existing` against a temp datadir writes `open-bitcoin.jsonc`, includes Open Bitcoin-owned onboarding and runtime sections, reports detected existing-install evidence when present, and does not create or modify `bitcoin.conf`.
result: pass

### 6. Onboarding Rerun Is Idempotent and Force Is Explicit
expected: Rerunning the same non-interactive onboarding command without `--force-overwrite` leaves the existing `open-bitcoin.jsonc` unchanged and reports that it was left unchanged. Running again with `--force-overwrite` succeeds explicitly, while still avoiding any `bitcoin.conf` write.
result: pass

### 7. Live RPC Status (If Available)
expected: When pointed at a reachable local regtest-style RPC endpoint, `open-bitcoin status --format json` reports a running node with live sync, peers, mempool, and wallet fields populated from RPC instead of the stopped-node unavailable values.
result: pass

## Summary

total: 7
passed: 7
issues: 0
pending: 0
skipped: 0
blocked: 0

## Gaps
