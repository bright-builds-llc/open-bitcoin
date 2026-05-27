---
status: complete
phase: 47-operator-sync-truth-surfaces
source:
  - 47-SUMMARY.md
started: 2026-05-27T11:24:12Z
updated: 2026-05-27T11:27:10Z
---

## Current Test

[testing complete]

## Tests

### 1. Shared Sync Status Fields
expected: Operator-facing status surfaces expose progress_signal and last_successful_progress_unix_seconds, with explicit unavailable values when durable sync truth is absent.
result: pass
evidence: `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node status::tests::`; `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli operator::status::`

### 2. Progress Signal And Last Progress Projection
expected: Durable sync summaries derive the correct progress signal and most recent successful progress timestamp across header, block download, waiting, failure, awaiting-peer, and steady states.
result: pass
evidence: `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node sync::tests::`

### 3. Aligned Metrics And Structured Logs
expected: Metrics and structured logs report header_height, downloaded_block_height, connected_block_height, compatibility sync_height, progress_signal, and last_successful_progress_unix_seconds without collapsing downloaded and connected chainstate truth.
result: pass
evidence: `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node sync::tests::`; `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node metrics::tests::metric_kind_names_are_stable`

### 4. Status Dashboard And RPC Alignment
expected: CLI status, terminal dashboard, and RPC fallback surfaces agree on durable sync truth; dashboard rows include signal, last progress, recovery guidance, latest error, and connected/downloaded height separation.
result: pass
evidence: `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli operator::dashboard::model::tests::`; `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli operator::status::`; `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc dispatch::tests::blockchain_info_uses_durable_sync_truth_when_available`

### 5. Operator Docs And Contracts
expected: Operator-facing docs describe progress_signal, last_successful_progress_unix_seconds, estimated lag, downloaded versus connected heights, and metrics/log semantics using repo-local Cargo and Bazel command surfaces where applicable.
result: pass
evidence: `rg -n "progress_signal|last_successful_progress_unix_seconds|downloaded_block_height|connected_block_height|sync_progress|estimated.*lag|header_height|sync_height|Bazel|cargo run" docs README.md packages/open-bitcoin-cli README.md`

## Summary

total: 5
passed: 5
issues: 0
pending: 0
skipped: 0
blocked: 0

## Gaps

[none yet]
