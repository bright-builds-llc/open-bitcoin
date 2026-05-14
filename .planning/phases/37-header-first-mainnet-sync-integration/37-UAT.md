---
status: complete
phase: 37-header-first-mainnet-sync-integration
source:
  - .planning/phases/37-header-first-mainnet-sync-integration/37-01-SUMMARY.md
started: 2026-05-14T00:02:35Z
updated: 2026-05-14T00:21:18Z
---

## Current Test

[testing complete]

## Tests

### 1. Header-First Sync Continues Without Block Requests
expected: |
  Run:
  cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node sync::tests::sync_once_continues_header_batches_when_peer_advertises_more_work -- --exact

  The test should pass. It should prove the sync runtime keeps sending getheaders when a peer advertises more header work, advances best_header_height to 2, keeps best_block_height at 0, and does not request blocks during Phase 37 header-first sync.
result: pass

### 2. Durable Header Progress Survives Restart
expected: |
  Run:
  cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node sync::tests::runtime_seeds_headers_from_durable_store_on_restart -- --exact

  The test should pass. It should prove a reopened runtime seeds from persisted headers and reports best_header_height 1 without replaying imported batches or implying block progress.
result: pass

### 3. Invalid Contextual Headers Become Typed Peer Failures
expected: |
  Run:
  cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node sync::tests::contextual_invalid_headers_fail_with_typed_invalid_data -- --exact

  The test should pass. It should prove invalid contextual headers are rejected as typed invalid-data peer outcomes, with the operator-facing signal "sync peer sent invalid data: inspect peer compatibility".
result: pass

### 4. Competing Header Branch Can Win After Restart
expected: |
  Run:
  cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node sync::tests::competing_header_branch_wins_after_restart_when_it_extends_farther -- --exact

  The test should pass. It should prove a farther valid competing header branch can take over after restart and the runtime snapshot reports best_header_height 3.
result: pass

### 5. Repo-Native Verification Contract Remains Green
expected: |
  Run:
  bash scripts/verify.sh

  The command should pass. It should cover the repo-owned verification contract for Phase 37, including formatting/lint/build/test coverage, parity breadcrumb checks, file-length and panic-site policy checks, the benchmark smoke, and the Bazel smoke build.
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
