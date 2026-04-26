# Storage Decision ADR

## Decision: fjall

Open Bitcoin will use `fjall` as the v1.1 default durable storage decision target. The decision is intentionally made before adding a concrete adapter so Phase 14 can implement storage behind typed contracts and measured recovery tests rather than choosing a database opportunistically during real sync work.

RocksDB remains a fallback only if Rust-native options fail measured storage and recovery checks for the v1.1 workload. That fallback requires explicit evidence because it adds native dependency and Bazel integration cost.

## Comparison

| Engine | Chainstate | Headers | Block index | Wallet | Metrics | Recovery | Bazel | Dependency |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `fjall` | LSM-shaped keyspaces fit high-write chainstate and UTXO updates. | Prefix/range iteration fits header indexes. | Separate keyspaces can isolate block index metadata. | Key-value namespaces can hold wallet state without changing wallet domain types. | Bounded series can live in a metrics namespace. | Phase 14 must prove schema version, corruption, restart, interrupted write, reindex, and repair behavior. | Rust-native dependency should be simpler than native C++ bindings. | Preferred default because it avoids RocksDB's native footprint while preserving a RocksDB-like shape. |
| `redb` | B-tree/MVCC model may be simpler and strongly crash-safe, but has a different write profile. | Works for ordered header records. | Works for metadata tables. | Works for wallet records. | Works for bounded series. | Strong fallback if simple ACID behavior outperforms LSM tradeoffs. | Rust-native dependency should be manageable. | Fallback if Phase 14 evidence favors simpler transactions over LSM behavior. |
| `rocksdb` | Mature LSM storage with a long operational history. | Strong fit for ordered headers and indexes. | Strong fit for block index metadata. | Usable for wallet state. | Usable for metrics. | Recovery tooling is mature, but Rust integration still needs Open Bitcoin-specific tests. | Native C++ build cost and Bzlmod integration need explicit justification. | Fallback only after Rust-native options fail measured storage and recovery checks. |

## Contract Shape

Phase 13 defines storage namespaces, schema versions, persist modes, recovery actions, and typed errors in `packages/open-bitcoin-node/src/storage.rs`. Those contracts are adapter-facing and must stay free of filesystem calls, concrete database imports, or public network behavior.

## Phase 14 Obligations

Before real sync or wallet runtime state depends on a durable adapter, Phase 14 must prove:

- `schema version` mismatch produces a typed recovery error.
- `corruption` is detected and mapped to operator guidance.
- `restart` preserves headers, block index metadata, chainstate, wallet, runtime, metrics, and schema records.
- `interrupted write` behavior is tested with isolated stores.
- `reindex` is exposed as an explicit recovery action.
- `repair` is exposed as an explicit recovery action.

Phase 14 may add the selected storage dependency only after these obligations are represented in tests and adapter code.

## Phase 14 Evidence

Phase 14 implements the initial `fjall` adapter in `packages/open-bitcoin-node/src/storage/fjall_store.rs`. The adapter stores schema-versioned JSON snapshots in separate `headers`, `block_index`, `chainstate`, `wallet`, `metrics`, `runtime`, and `schema` keyspaces and keeps database effects inside `open-bitcoin-node`.

The restart and recovery tests cover reopening persisted chainstate, header/block-index, wallet, metrics, and runtime records; incompatible schema versions; malformed stored records; interrupted-write recovery markers; explicit reindex guidance; and clean-shutdown marker clearing.
