---
status: complete
phase: 04-chainstate-and-utxo-engine
source:
  - .planning/phases/04-chainstate-and-utxo-engine/04-01-SUMMARY.md
  - .planning/phases/04-chainstate-and-utxo-engine/04-02-SUMMARY.md
  - .planning/phases/04-chainstate-and-utxo-engine/04-03-SUMMARY.md
started: 2026-04-25T16:15:28Z
updated: 2026-04-25T17:02:12Z
---

## Current Test

[testing complete]

## Tests

### 1. Pure Chainstate Crate and Model Wiring
expected: From the repo root, both `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-chainstate coin_converts_to_spent_output_without_losing_metadata` and `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-chainstate snapshot_tip_returns_the_last_active_position` pass, and `packages/open-bitcoin-core/src/lib.rs` exposes `open_bitcoin_chainstate` as `chainstate`.
result: pass

### 2. Connect/Disconnect State Transitions
expected: Both `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-chainstate derives_contexts_from_chainstate_metadata` and `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-chainstate connect_and_disconnect_round_trip_utxos_and_tip` pass, proving the chainstate engine derives consensus contexts from UTXO metadata and can connect then disconnect a direct tip without losing the expected UTXO state.
result: pass

### 3. Reorg and Best-Tip Selection
expected: Both `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-chainstate reorg_prefers_heavier_branch_and_preserves_expected_utxos` and `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-chainstate reorg_and_tip_preference_cover_remaining_decision_branches` pass, showing candidate tips are selected by deterministic work/height/hash rules and reorgs preserve the expected UTXO set.
result: pass

### 4. Chainstate Parity Edge Cases
expected: `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-chainstate --test parity` passes, covering connect/disconnect/reorg parity outcomes, unspendable outputs staying out of the spendable UTXO view, and BIP30-style output overwrite rejection through the public chainstate API.
result: pass

### 5. Node-Side Persistence Boundary
expected: `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node chainstate` passes, covering `ManagedChainstate`, `ChainstateStore`, and `MemoryChainstateStore` snapshot persistence while keeping storage concerns outside the pure chainstate crate.
result: pass

### 6. Repo Verification and Parity Documentation
expected: `docs/parity/catalog/chainstate.md`, `docs/parity/index.json`, and the Phase 4 verification report show chainstate as verified with explicit known gaps, and `bash scripts/verify.sh` succeeds with the chainstate crate included in pure-core verification, coverage, Bazel smoke build, and parity/report checks.
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
