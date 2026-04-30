---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 14-2026-04-26T20-34-48
generated_at: 2026-04-26T20:34:48.510Z
---

# Phase 14: Durable Storage and Recovery - Context

**Gathered:** 2026-04-26
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Implement the first durable storage adapter and recovery contract for node-owned runtime state. This phase persists current in-memory snapshots and storage metadata across restart, but does not implement real network sync, metrics/log writers, service installation, dashboard rendering, or wallet runtime expansion.

</domain>

<decisions>
## Implementation Decisions

### Storage engine and ownership
- **D-01:** Use `fjall` as the concrete Phase 14 storage dependency, following the Phase 13 ADR.
- **D-02:** Keep all filesystem and database effects in `open-bitcoin-node`; pure chainstate, consensus, wallet, network, mempool, codec, and primitive crates must stay database-free.
- **D-03:** Use node-owned snapshot DTOs at the storage boundary instead of adding serialization derives or database-specific concerns to pure domain types.

### Persisted state
- **D-04:** Persist schema metadata, active chain/header/block-index metadata, UTXO state, undo/reorg metadata, wallet snapshot state, runtime metadata, and a metrics placeholder namespace.
- **D-05:** Store Phase 14 records as schema-versioned JSON values in fjall keyspaces. This is inspectable and sufficient for deterministic restart/recovery tests; later phases may replace hot-path values with more compact encodings behind the same boundary.
- **D-06:** Header/block-index persistence may start from exported header entries and active-chain positions; real peer header sync integration remains Phase 15.

### Recovery behavior
- **D-07:** Opening a store with an incompatible schema version must return `StorageError::SchemaMismatch`.
- **D-08:** Malformed persisted records must return `StorageError::Corruption` with an operator recovery action.
- **D-09:** Recovery helpers must expose restart, reindex, repair, and restore-from-backup guidance without mutating real user data in tests.

### Tests and verification
- **D-10:** Tests must use isolated temporary directories and must not touch real operator datadirs or service-manager state.
- **D-11:** Restart tests must close and reopen the fjall store to prove persistence, not just inspect an in-memory cache.
- **D-12:** The repo-owned verification path remains the authority before commit.

### the agent's Discretion
- Exact DTO names, key names, and helper layout are discretionary if they keep storage effects in the shell crate, preserve typed errors, and satisfy the Phase 14 success criteria.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase scope
- `.planning/ROADMAP.md` - Phase 14 goal, dependencies, requirements, and success criteria.
- `.planning/REQUIREMENTS.md` - DB-02, DB-03, DB-04, DB-05 definitions.
- `.planning/PROJECT.md` - parity, functional-core, and dependency constraints.
- `.planning/phases/13-operator-runtime-foundations/13-CONTEXT.md` - Phase 13 storage decisions.
- `docs/architecture/storage-decision.md` - fjall decision and Phase 14 obligations.

### Existing code boundaries
- `packages/open-bitcoin-node/src/storage.rs` - storage namespace, schema, persistence, and recovery contracts.
- `packages/open-bitcoin-node/src/chainstate.rs` - managed in-memory chainstate adapter and snapshot store pattern.
- `packages/open-bitcoin-node/src/wallet.rs` - managed in-memory wallet adapter and snapshot store pattern.
- `packages/open-bitcoin-network/src/header_store.rs` - header metadata model for sync state.
- `packages/open-bitcoin-chainstate/src/types.rs` - chainstate snapshot, UTXO, and undo models.
- `packages/open-bitcoin-wallet/src/wallet.rs` - wallet snapshot model.
- `docs/parity/source-breadcrumbs.json` - required source breadcrumb manifest for new Rust files.

### Standards
- `AGENTS.md` - Repo-local workflow, parity, verification, and GSD rules.
- `AGENTS.bright-builds.md` - Bright Builds workflow and standards routing.
- `standards-overrides.md` - Local standards exceptions.
- `../coding-and-architecture-requirements/standards/index.md`
- `../coding-and-architecture-requirements/standards/core/architecture.md`
- `../coding-and-architecture-requirements/standards/core/code-shape.md`
- `../coding-and-architecture-requirements/standards/core/testing.md`
- `../coding-and-architecture-requirements/standards/core/verification.md`
- `../coding-and-architecture-requirements/standards/languages/rust.md`

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `ChainstateSnapshot`, `WalletSnapshot`, and `HeaderEntry` already express most state Phase 14 needs to preserve.
- `StorageNamespace`, `SchemaVersion`, `PersistMode`, `StorageError`, and `StorageRecoveryAction` already provide the typed operator contract.
- `ManagedChainstate` and `ManagedWallet` prove the store-wrapper pattern for in-memory state.

### Established Patterns
- New Rust files require top-of-file parity breadcrumbs and entries in `docs/parity/source-breadcrumbs.json`.
- Runtime effects belong in `open-bitcoin-node`.
- Tests use focused Arrange / Act / Assert structure.

### Integration Points
- `open-bitcoin-node::storage` should export durable store types.
- `open-bitcoin-network::HeaderStore` may need small pure helper methods to expose/rebuild header entries.
- Bazel must include any new external crate labels used by the node crate.

</code_context>

<deferred>
## Deferred Ideas

- Real peer header/block sync persistence belongs to Phase 15.
- Metrics/log writer retention enforcement belongs to Phase 16.
- Rich status rendering and onboarding config writes belong to Phase 17.
- Service lifecycle-managed datadir paths belong to Phase 18.

</deferred>
