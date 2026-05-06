---
status: complete
phase: 25-migration-source-selection-hardening
source:
  - 25-01-SUMMARY.md
  - 25-02-SUMMARY.md
  - 25-03-SUMMARY.md
started: 2026-05-06T11:02:17.026Z
updated: 2026-05-06T11:12:06.809Z
---

## Current Test

[testing complete]

## Tests

### 1. Supported custom source datadir produces a concrete dry-run plan
expected: Create a temporary Core or Knots-like source datadir outside the default home roots with source evidence such as `bitcoin.conf`, `.cookie`, and `wallets/main/wallet.dat`. Then run either `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --network regtest --datadir <temp-target-datadir> migrate plan --source-datadir <temp-source-datadir>` or `bazel run //packages/open-bitcoin-cli:open_bitcoin -- --network regtest --datadir <temp-target-datadir> migrate plan --source-datadir <temp-source-datadir>`. The command should succeed, print `Migration plan (dry run only)`, include the explicit source datadir plus its `bitcoin.conf` and wallet path in the review evidence, keep service review manual when the service cannot be confidently tied to that selected source, and avoid printing secret cookie or wallet file contents.
result: skipped
reason: "defer"

### 2. Bare explicit source datadir stays in manual review
expected: Create a temporary existing directory outside the default home roots, but do not add `bitcoin.conf`, `.cookie`, or wallet evidence to it. Then run either `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --network regtest --datadir <temp-target-datadir> migrate plan --source-datadir <bare-temp-source-datadir>` or `bazel run //packages/open-bitcoin-cli:open_bitcoin -- --network regtest --datadir <temp-target-datadir> migrate plan --source-datadir <bare-temp-source-datadir>`. The command should not auto-select that bare directory as a supported migration source; it should report manual review guidance explaining that the explicit path does not yet expose source config, cookie, or wallet evidence.
result: skipped
reason: "defer"

## Summary

total: 2
passed: 0
issues: 0
pending: 0
skipped: 2
blocked: 0

## Gaps
