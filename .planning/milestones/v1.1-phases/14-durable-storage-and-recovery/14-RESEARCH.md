---
phase: 14-durable-storage-and-recovery
generated_by: gsd-phase-researcher
lifecycle_mode: yolo
phase_lifecycle_id: 14-2026-04-26T20-34-48
generated_at: 2026-04-26T20:34:48.510Z
---

# Phase 14 Research

## Findings

- Phase 13 selected `fjall` as the v1.1 storage decision target and explicitly deferred concrete adapter implementation to this phase.
- The current node crate already owns effectful runtime wrappers and can take the database dependency without contaminating pure core crates.
- `fjall` 3.1.4 is a Rust-native, log-structured key-value store with multiple keyspaces and explicit persist modes. It supports the namespace shape described in the Phase 13 ADR.
- Existing snapshot types are cloneable but not serializable. A node-owned DTO layer avoids pushing serde or database concerns into pure domain crates.
- Current managed in-memory chainstate and wallet stores are useful patterns but do not return storage errors. The durable adapter should expose explicit fallible load/save methods while leaving existing pure wrappers intact.

## Recommended Shape

1. Add `fjall = "3.1.4"` to `open-bitcoin-node`.
2. Split `storage.rs` into a module entry plus child modules for durable fjall storage and snapshot DTO encoding.
3. Add `FjallNodeStore` with keyspaces for schema, headers, block index, chainstate, wallet, metrics, and runtime.
4. Add DTO conversion functions that serialize/deserialize current domain snapshots to JSON bytes with schema and corruption checks.
5. Add isolated restart, schema mismatch, corruption, interrupted-write marker, reindex, and repair tests.
6. Export only node-shell storage types; pure crates remain database-free.

## Risks

- `fjall` adds a larger dependency graph and Bazel generated lock metadata; `bash scripts/verify.sh` must prove Bzlmod still resolves.
- JSON snapshot encoding is not the final sync hot path. Treat it as a schema-versioned boundary and keep later binary/columnar optimizations behind the same adapter.
- Wallet descriptors should be restored from `original_text` so private keys remain represented exactly as the current wallet parser expects.
