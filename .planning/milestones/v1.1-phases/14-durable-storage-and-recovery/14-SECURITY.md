---
phase: 14-durable-storage-and-recovery
slug: durable-storage-and-recovery
status: verified
threats_open: 0
asvs_level: 1
created: 2026-05-02
generated_by: gsd-secure-phase
lifecycle_mode: yolo
phase_lifecycle_id: 14-2026-04-26T20-34-48
generated_at: 2026-05-03T03:20:55Z
---

# Phase 14 - Security

Per-phase security contract: threat register, accepted risks, and audit trail.

## Scope

Phase 14 adds the first durable `fjall` storage adapter, node-owned snapshot
DTO encoding, schema validation, corruption handling, recovery markers, and
restart persistence evidence for node runtime state. The phase changes
`open-bitcoin-node`, `open-bitcoin-network` header-store rebuild helpers,
parity breadcrumbs, and storage ADR evidence. It does not add public network
surfaces, authentication changes, wallet export/import endpoints, or service
automation.

The phase plans did not include an explicit `<threat_model>` block, so this
audit derived the threat register from the phase-local decisions in
`14-CONTEXT.md`, `14-DISCUSSION-LOG.md`, `14-RESEARCH.md`, `14-03-PLAN.md`,
and `14-VERIFICATION.md`, then verified those threats against implementation and
targeted tests only. No unrelated vulnerability scan was performed.

## Trust Boundaries

| Boundary | Description | Data Crossing |
|----------|-------------|---------------|
| Node runtime to Fjall datadir | The node shell persists and reopens runtime state through a local durable store. | Schema metadata, header/block-index snapshots, chainstate, wallet data, metrics, runtime metadata, recovery markers. |
| Snapshot DTO layer to pure domain types | Node-owned JSON DTOs serialize and rebuild pure chainstate, header, and wallet state. | Versioned storage payloads and reconstructed domain snapshots. |
| Operator to recovery contract | Storage failures must surface actionable recovery guidance instead of silent reset or panic. | `StorageError`, `StorageRecoveryAction`, clean-shutdown status, interrupted-write markers. |
| Node shell to pure core crates | Database and filesystem effects must remain outside consensus, chainstate, wallet, mempool, codec, and primitive crates. | Storage dependencies, adapter code, import boundaries. |

## Threat Register

| Threat ID | Category | Component | Disposition | Mitigation | Status |
|-----------|----------|-----------|-------------|------------|--------|
| T-14-01-01 | Tampering | Schema metadata compatibility | mitigate | `FjallNodeStore::open` forces `ensure_schema()`, and `validate_schema_version` rejects mismatched versions with `StorageError::SchemaMismatch` instead of reopening against an incompatible layout. Evidence: [fjall_store.rs](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/storage/fjall_store.rs:56), [fjall_store.rs](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/storage/fjall_store.rs:419), [storage.rs](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/storage.rs:156), [tests.rs](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/storage/fjall_store/tests.rs:540). Exact test passed: `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node storage::fjall_store::tests::incompatible_schema_version_returns_schema_mismatch -- --exact`. | closed |
| T-14-01-02 | Tampering | Persisted snapshot and recovery-marker payloads | mitigate | Versioned decoders map malformed JSON and invalid payloads into typed corruption errors with repair guidance, and header rebuild paths reject inconsistent stored content instead of trusting it blindly. Evidence: [snapshot_codec.rs](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/storage/snapshot_codec.rs:227), [snapshot_codec.rs](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/storage/snapshot_codec.rs:244), [fjall_store.rs](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/storage/fjall_store.rs:159), [storage.rs](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/storage.rs:208), [tests.rs](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/storage/fjall_store/tests.rs:570), [tests.rs](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/storage/fjall_store/tests.rs:640). Exact tests passed: `... malformed_snapshot_maps_to_corruption -- --exact` and `... malformed_recovery_marker_maps_to_runtime_corruption -- --exact`. | closed |
| T-14-01-03 | Denial of Service | Interrupted-write and recovery-state handling | mitigate | Interrupted writes persist explicit recovery markers and last recovery action, operator-visible actions expose `Restart`, `Reindex`, `Repair`, and `RestoreFromBackup`, and clean shutdown clears stale markers before the next run. Evidence: [storage.rs](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/storage.rs:81), [fjall_store.rs](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/storage/fjall_store.rs:372), [fjall_store.rs](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/storage/fjall_store.rs:399), [tests.rs](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/storage/fjall_store/tests.rs:602). Exact tests passed: `... recovery_marker_round_trips_and_clean_shutdown_clears_it -- --exact` and `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node storage::tests::storage_recovery_actions_have_operator_messages -- --exact`. | closed |
| T-14-01-04 | Tampering | Pure-core storage boundary | mitigate | The concrete `fjall` adapter and snapshot codecs live under `open-bitcoin-node::storage`, and the repo’s pure-core dependency guard passed, preventing database and filesystem effects from leaking into pure crates. Evidence: [storage.rs](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/storage.rs:1), [14-CONTEXT.md](/Users/peterryszkiewicz/Repos/open-bitcoin/.planning/milestones/v1.1-phases/14-durable-storage-and-recovery/14-CONTEXT.md:30), [14-RESEARCH.md](/Users/peterryszkiewicz/Repos/open-bitcoin/.planning/milestones/v1.1-phases/14-durable-storage-and-recovery/14-RESEARCH.md:24). Exact check passed: `bash scripts/check-pure-core-deps.sh`. | closed |

Status: open, closed.
Disposition: mitigate (implementation required), accept (documented risk), transfer (third-party).

## Accepted Risks Log

No accepted risks.

## Unregistered Flags

No unregistered flags.

| Summary | Threat Flags Result | Mapping |
|---------|---------------------|---------|
| `14-01-SUMMARY.md` | No `## Threat Flags` section present. | None |
| `14-02-SUMMARY.md` | No `## Threat Flags` section present. | None |
| `14-03-SUMMARY.md` | No `## Threat Flags` section present. | None |
| `14-04-SUMMARY.md` | No `## Threat Flags` section present. | None |

## Security Audit Trail

| Audit Date | Threats Total | Closed | Open | Run By |
|------------|---------------|--------|------|--------|
| 2026-05-02 | 4 | 4 | 0 | gsd-secure-phase |

## Verification Evidence

| Command | Result |
|---------|--------|
| `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node storage::fjall_store::tests::incompatible_schema_version_returns_schema_mismatch -- --exact` | Passed. |
| `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node storage::fjall_store::tests::malformed_snapshot_maps_to_corruption -- --exact` | Passed. |
| `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node storage::fjall_store::tests::malformed_recovery_marker_maps_to_runtime_corruption -- --exact` | Passed. |
| `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node storage::fjall_store::tests::recovery_marker_round_trips_and_clean_shutdown_clears_it -- --exact` | Passed. |
| `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node storage::tests::storage_recovery_actions_have_operator_messages -- --exact` | Passed. |
| `bash scripts/check-pure-core-deps.sh` | Passed. |

## Standards Inputs

Materially applied local `AGENTS.md`, `AGENTS.bright-builds.md`,
`standards-overrides.md`, Bright Builds pinned `standards/index.md`,
`standards/core/verification.md`, `standards/core/testing.md`, the repo-local
Phase 14 context/research/discussion artifacts, and the `gsd-secure-phase`
workflow. ASVS Level: 1.

## Sign-Off

- [x] All threats have a disposition (mitigate / accept / transfer)
- [x] Accepted risks documented in Accepted Risks Log
- [x] `threats_open: 0` confirmed
- [x] `status: verified` set in frontmatter

Approval: verified 2026-05-02
