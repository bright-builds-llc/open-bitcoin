---
status: complete
phase: 14-durable-storage-and-recovery
source:
  - 14-01-SUMMARY.md
  - 14-02-SUMMARY.md
  - 14-03-SUMMARY.md
  - 14-04-SUMMARY.md
started: 2026-05-03T02:41:22Z
updated: 2026-05-03T03:04:49Z
---

## Current Test

[testing complete]

## Tests

### 1. Durable snapshot survives restart
expected: Run `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node storage::fjall_store::tests::fjall_store_reopens_saved_snapshots_and_metadata -- --exact`. It should pass and prove headers, block-index metadata, chainstate, wallet state, runtime metadata, metrics, and schema data all survive closing and reopening the Fjall store.
result: pass

### 2. Incompatible schema is rejected safely
expected: Run `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node storage::fjall_store::tests::incompatible_schema_version_returns_schema_mismatch -- --exact`. It should fail safely with `StorageError::SchemaMismatch` plus recovery guidance instead of panicking or silently resetting the store.
result: pass

### 3. Corrupt snapshot records surface repair guidance
expected: Run `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node storage::fjall_store::tests::malformed_snapshot_maps_to_corruption -- --exact`. It should detect malformed persisted snapshot data and map it to a typed corruption error with repair-oriented operator guidance.
result: pass

### 4. Corrupt recovery markers surface runtime corruption guidance
expected: Run `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node storage::fjall_store::tests::malformed_recovery_marker_maps_to_runtime_corruption -- --exact`. It should detect malformed recovery-marker data and return a typed runtime corruption error with explicit recovery guidance.
result: pass

### 5. Interrupted-write markers clear after clean shutdown
expected: Run `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node storage::fjall_store::tests::recovery_marker_round_trips_and_clean_shutdown_clears_it -- --exact`. It should preserve recovery-marker state across restart, expose reindex guidance for interrupted writes, and clear the marker after a clean shutdown path.
result: pass

### 6. Recovery actions have operator-facing messages
expected: Run `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node storage::tests::storage_recovery_actions_have_operator_messages -- --exact`. It should confirm `Restart`, `Reindex`, `Repair`, and `RestoreFromBackup` each expose explicit operator-facing guidance instead of vague generic errors.
result: pass

### 7. Pure core crates stay database-free
expected: Run `bash scripts/check-pure-core-deps.sh`. It should pass, proving the `fjall` dependency and durable-storage effects stay inside `open-bitcoin-node` rather than leaking into pure core crates.
result: pass

## Summary

total: 7
passed: 7
issues: 0
pending: 0
skipped: 0
blocked: 0

## Gaps
