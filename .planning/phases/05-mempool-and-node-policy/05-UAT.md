---
status: complete
phase: 05-mempool-and-node-policy
source:
  - .planning/phases/05-mempool-and-node-policy/05-01-SUMMARY.md
  - .planning/phases/05-mempool-and-node-policy/05-02-SUMMARY.md
  - .planning/phases/05-mempool-and-node-policy/05-03-SUMMARY.md
started: 2026-04-25T17:14:50Z
updated: 2026-04-25T19:15:02Z
---

## Current Test

[testing complete]

## Tests

### 1. Pure Mempool Crate and Policy Types
expected: From the repo root, `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-mempool default_policy_matches_the_targeted_phase_defaults` and `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-mempool mempool_entry_starts_with_self_only_metrics` pass, and `packages/open-bitcoin-core/src/lib.rs` exposes `open_bitcoin_mempool` as `mempool`.
result: pass

### 2. Standard Admission and Rejection
expected: `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-mempool accepts_standard_confirmed_spend`, `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-mempool rejects_non_standard_output_scripts`, and `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-mempool duplicate_transactions_and_missing_inputs_are_rejected` pass, proving the public mempool engine admits valid confirmed spends and rejects duplicates, missing inputs, and non-standard outputs before mutating state.
result: pass

### 3. RBF Replacement Policy
expected: `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-mempool replacement_requires_a_fee_bump`, `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-mempool replacement_requires_opt_in_signal_when_policy_demands_it`, and `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-mempool replacements_respect_disabled_policy` pass, covering fee-bump replacement, opt-in signaling, and disabled replacement behavior.
result: pass

### 4. Ancestor, Descendant, and Eviction Accounting
expected: `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-mempool tracks_parent_child_and_ancestor_descendant_metrics`, `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-mempool rejects_entries_that_exceed_ancestor_limits`, and `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-mempool evicts_lowest_descendant_score_package_when_size_limit_is_exceeded` pass, showing relationship metrics, limit checks, and size-limit trimming are deterministic and explicit.
result: pass

### 5. Public Mempool Parity Fixtures
expected: `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-mempool --test parity` passes, covering standard admission, non-standard rejection, fee-bump replacement, ancestor-limit handling, and size-limit eviction through the public mempool API.
result: pass

### 6. Managed Node Mempool Wrapper
expected: `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node mempool` passes, covering `ManagedMempool` submission against `ManagedChainstate` without re-implementing pure-core policy logic in the runtime shell.
result: pass

### 7. Repo Verification and Parity Documentation
expected: `docs/parity/catalog/mempool-policy.md`, `docs/parity/index.json`, and the Phase 5 verification report show mempool policy as verified with explicit known gaps, and `bash scripts/verify.sh` succeeds with the mempool crate included in pure-core verification, coverage, Bazel smoke build, and parity/report checks.
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
