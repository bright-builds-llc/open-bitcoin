---
status: complete
phase: 38-block-download-connect-and-restart-recovery
source:
  - .planning/phases/38-block-download-connect-and-restart-recovery/38-01-SUMMARY.md
started: 2026-05-14T12:21:09.256Z
updated: 2026-05-15T10:06:21.000Z
---

## Current Test

[testing complete]

## Tests

### 1. Bounded Missing-Block Requests
expected: Run `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network request_missing_blocks -- --nocapture`. The filtered network tests pass. The output should show the request planner can skip locally known block hashes, track requested block inventory, respect in-flight capacity, and stop issuing `getdata` once capacity is filled.
result: pass
evidence: "3 filtered peer tests passed: skips known hashes/tracks requested inventory, respects capacity/skip-only path, and stops once capacity is filled."

### 2. Restart Reconnects Persisted Blocks
expected: Run `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node restart_reconnects_persisted_blocks_before_re_requesting_them -- --nocapture`. The filtered node test passes. The restarted runtime reconnects the already stored block, advances `best_block_height`, and does not send a duplicate `getdata` request for that block.
result: pass
evidence: "1 filtered node test passed."

### 3. Restart Reorgs To Best Local Branch
expected: Run `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node restart_reorgs_to_best_available_branch_when_blocks_are_already_local -- --nocapture`. The filtered node test passes. After restart, the runtime activates the better durable branch from local blocks and reports matching best header and best block height.
result: pass
evidence: "1 filtered node test passed."

### 4. Best-Chain Entry Ordering
expected: Run `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network best_chain_entries_follow_the_selected_tip_order -- --nocapture`. The filtered header-store test passes. The returned best-chain entries follow the selected tip from genesis through the winning branch and omit the losing branch.
result: pass
evidence: "1 filtered header-store test passed."

### 5. Repo-Native Verification Contract
expected: Run `bash scripts/verify.sh`. The repo-native verification contract passes, including formatting/lint/build/test coverage, panic-site checks, benchmark smoke, Bazel smoke build, and tracked generated LOC freshness.
result: pass
evidence: "Repo-native verifier passed in 2m 37.517s, including hook setup check, LOC freshness, parity breadcrumbs, pure-core policy, file-length gate, panic-site check, Cargo build/test coverage, benchmark smoke, and Bazel smoke build."

## Summary

total: 5
passed: 5
issues: 0
pending: 0
skipped: 0
blocked: 0

## Gaps

[none yet]
