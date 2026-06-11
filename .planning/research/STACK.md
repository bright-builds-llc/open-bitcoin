# Stack Research: v1.6 Mainnet Full-Sync Completion

**Domain:** explicit opt-in `open-bitcoind` mainnet full sync-to-tip and stay-current operation
**Researched:** 2026-06-11
**Confidence:** HIGH for stack direction, MEDIUM for exact performance tuning until a long-run sync profile exists

## Findings

v1.6 does not need a broad stack replacement. The existing Rust `1.94.1` workspace, first-party `open-bitcoin-*` crates, Bazel/Bzlmod smoke build, Bun-backed TypeScript automation, Fjall durable metadata store, and local operator surfaces are the right base for full mainnet sync-to-tip.

The required stack change is mostly first-party design: the current runtime persists headers, downloaded blocks, runtime metadata, metrics, and a whole `ChainstateSnapshot`. That is enough for bounded evidence and restart/resume review, but not enough for a truthful mainnet-scale sync-to-tip claim. v1.6 should evolve storage, validation, sync scheduling, and verification around durable incremental state rather than adding an existing Rust Bitcoin implementation.

Materially reviewed local guidance and sources: `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md`, `.planning/PROJECT.md`, `.planning/MILESTONES.md`, `.planning/milestones/v1.5-REQUIREMENTS.md`, `.planning/STACK.md`, `.planning/ARCHITECTURE.md`, `docs/parity/release-readiness.md`, package manifests, and the current sync, storage, chainstate, and P2P modules. The pinned canonical Bright Builds standards pages were not available from this environment during research.

## Recommended Stack Changes

### First-Party Full-Sync Design

- Keep the functional-core / imperative-shell split. Consensus, chainstate rules, header validation, reorg selection, and transaction validation should stay in pure first-party crates. Sockets, files, clocks, service managers, durable storage, logs, metrics, and runtime orchestration should stay in `open-bitcoin-node`, `open-bitcoin-rpc`, and `open-bitcoin-cli` shell surfaces.
- Promote v1.6 from bounded progress loops to a durable sync controller with explicit states for headers catch-up, block download, block connect, near-tip follow, no-progress recovery, peer rotation, operator pause/shutdown, and clean stay-current idle.
- Keep `open-bitcoind` explicit opt-in. Full sync-to-tip should be source-built operator evidence and not a default daemon behavior unless the operator enables it.

### Durable Storage

- Keep Fjall as the durable key-value engine for metadata, headers, indexes, UTXO entries, undo metadata, runtime state, bounded metrics, and wallet snapshots. Do not swap storage engines preemptively.
- Add first-party mainnet-scale storage layout instead of storing whole chainstate snapshots for every connect:
  - UTXO keyspace keyed by `OutPoint`, with compact first-party encoding for `Coin`.
  - Undo keyspace keyed by block hash or height/hash pair for disconnect and reorg.
  - Active-chain index keyed by height and block hash for tip, locators, and reorg common-ancestor lookup.
  - Header index keyed by hash plus best-chain height projection and cumulative work.
  - Block index metadata keyed by hash, including height when known, status flags, file offset or payload location, validation/connect status, and pruning eligibility if pruning is later designed.
- Add a first-party append-only block body store for full block payloads if profiling confirms large Fjall values are not appropriate for mainnet block files. Prefer segmented `blocks` and `undo` files with Fjall metadata over introducing RocksDB or LevelDB. Keep the format Open Bitcoin-owned and documented, with reindex or repair paths before any production claim.
- Introduce a schema version bump and deterministic recovery behavior for the v1.6 storage layout. Existing v1.5 datadirs should fail or upgrade through typed recovery/reindex guidance, not silent best-effort interpretation.
- Replace whole-snapshot chainstate persistence in the sync path with write batches and checkpoints. The pure core can produce connect/disconnect effects; the shell adapter should apply them durably with bounded cache flushes.

### Validation

- Complete first-party validation needed for active mainnet sync: contextual header difficulty/work, median-time-past, historical activation rules, coinbase height and maturity, BIP30-style overwrite handling, subsidy plus fees, full spend-context validation, witness/script flags by height, and reorg disconnect/reconnect behavior.
- Preserve parity breadcrumbs against the pinned Knots baseline for any new validation files. Do not use an external Rust Bitcoin consensus or chainstate library in production code.
- Add typed validation outcomes that feed peer attribution, durable block status, support bundles, and operator recovery. Invalid headers or blocks should not be credited as useful progress.

### Networking And Runtime

- Reuse the existing P2P model: `PeerManager`, `HeaderStore`, `DurableSyncRuntime`, `SyncTransport`, and compatibility harnesses. The main change is making the sync scheduler sustained and parallel enough for mainnet, not replacing the protocol stack.
- Keep outbound-only scope. v1.6 should not add inbound serving, address relay, transaction relay, compact block relay, or broad production-node peer policy.
- Add durable peer scoring and rotation sufficient for long runs: stale peers, incompatible peers, repeated `notfound`, malformed payloads, invalid data, slow peers, storage pressure, and no-progress windows should all be typed and bounded.
- If concurrent peer I/O is needed, reuse the existing Tokio runtime already present in `open-bitcoin-rpc`/`open-bitcoind` or add Tokio narrowly to `open-bitcoin-node` as shell orchestration. Do not introduce a second async runtime. A blocking worker-thread design is also acceptable if it meets throughput and shutdown requirements.
- Keep DNS and manual-peer handling first-party and deterministic under test, with injected resolvers/transports for hermetic coverage.

### Performance And Observability

- Extend current bounded metrics, structured logs, status, dashboard, RPC sync status, live-smoke reports, support bundles, and service evidence instead of adding Prometheus, OpenTelemetry, or a hosted dashboard.
- Add sync-to-tip fields: header tip, connected tip, chainwork, validated block rate, bytes downloaded, block/connect backlog, peer contribution by phase, UTXO/cache flush stats, store write/flush latency, reorg depth, no-progress windows, and stay-current lag.
- Extend benchmark smoke reports with deterministic storage/connect scenarios and long-chain synthetic sync. Benchmark reports should remain audit/trend evidence, not timing gates.

## Dependency Stance

- Recommended third-party dependency additions for v1.6: none by default.
- Acceptable narrow change: reuse the already-pinned Tokio version in a node-shell sync worker if the implementation chooses async multi-peer orchestration. Keep it out of pure-core crates.
- Keep `secp256k1` as the cryptographic dependency already used by consensus and wallet code. Continue first-party hashing, codec, primitives, consensus, chainstate, mempool, wallet, network, node, RPC, CLI, harness, and benchmark code.
- Do not add existing Rust Bitcoin libraries in the production path. That includes libraries for block/transaction types, consensus verification, chainstate, P2P, wallet, or address logic.
- Do not add RocksDB, LevelDB, SQLite, external indexers, Electrum, Esplora, ZMQ, Prometheus, or OpenTelemetry unless a later profiling or product milestone proves the current first-party/Fjall design cannot meet a specific requirement.

## Integration Points

- `packages/open-bitcoin-node/src/sync.rs`: evolve `DurableSyncRuntime` from bounded `sync_once` and `sync_until_idle` work into a resumable full-sync controller with durable phase transitions and stay-current behavior.
- `packages/open-bitcoin-node/src/sync/block_reconcile.rs`: expand best-chain block download/connect/reorg reconciliation for mainnet-scale backlog, durable statuses, and restart-safe in-flight cleanup.
- `packages/open-bitcoin-node/src/storage/fjall_store.rs` and `packages/open-bitcoin-node/src/storage.rs`: add v1.6 schema, UTXO/undo/active-chain/block metadata stores, typed recovery, and reindex/repair hooks.
- `packages/open-bitcoin-chainstate/src/engine.rs` and `packages/open-bitcoin-node/src/chainstate.rs`: replace whole-snapshot persistence with first-party durable coin-view/write-batch integration while preserving pure validation rules.
- `packages/open-bitcoin-network/src/header_store.rs` and `packages/open-bitcoin-network/src/peer.rs`: keep header-chain, locator, block request, and peer attribution behavior deterministic while adding long-run scheduler inputs.
- `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs`: own opt-in daemon activation, lifecycle, shutdown, and Tokio or worker-thread orchestration.
- `scripts/run-live-mainnet-smoke.ts`, `docs/operator/runtime-guide.md`, support bundle code, and status/dashboard/RPC projections: extend from bounded evidence to full-sync and stay-current evidence.
- `docs/parity/index.json`, `docs/parity/checklist.md`, `docs/parity/release-readiness.md`, and a new v1.6 threat model/release-boundary checker: record the new claim without promoting deferred surfaces.

## What Not To Add

- No existing Rust Bitcoin libraries in production code.
- No inbound serving, address advertisement, transaction relay, compact block relay, mempool propagation claim, production-funds wallet claim, migration apply mode, GUI, hosted dashboard, signed packaging, Windows service support, or broad production-node wording.
- No public-network or real service-manager work in `bash scripts/verify.sh`.
- No checked-in live-mainnet block data, generated full-sync reports, support bundles, or timing-threshold release gates.
- No storage-engine migration away from Fjall without a measured blocker and a separate migration/recovery design.
- No hidden mutation of Core or Knots datadirs. v1.6 should continue using Open Bitcoin-owned datadirs for full-sync evidence.

## Verification Implications

- Keep `bash scripts/verify.sh` deterministic and public-network-free. It should continue to cover formatting, linting, tests, architecture policy, panic-site checks, parity breadcrumbs, benchmark smoke/report validation, and Bazel smoke builds.
- Add hermetic v1.6 tests for:
  - incremental durable UTXO and undo write batches;
  - block connect/disconnect/reorg across restart;
  - header reorg and best-chain selection;
  - duplicate, malformed, invalid, non-extending, disconnected, and `notfound` block responses;
  - storage schema mismatch, corruption, lock contention, repair, and reindex guidance;
  - no duplicate block requests or connects after crash/restart;
  - long synthetic chain sync with bounded memory and metrics/log retention.
- Extend parity fixtures and compatibility harnesses against the pinned Knots baseline for the new validation and P2P behaviors. New first-party Rust files under `packages/open-bitcoin-*/src` or tests need parity breadcrumb entries.
- Add opt-in operator UAT for actual mainnet full sync-to-tip and stay-current review. The generated evidence should stay local, redacted, and outside git, with JSON/Markdown summaries containing start height, final connected tip, tip freshness, peer contribution, restart/resume checks, resource pressure, and next action.
- Add deterministic v1.6 release-boundary checks so docs and parity roots cannot claim production-node, inbound, relay, wallet, packaging, migration apply, GUI, hosted dashboard, or default public-network verification scope.

## Bottom Line

The v1.6 stack should remain first-party Rust with Fjall-backed durable metadata and existing operator surfaces. The meaningful additions are an Open Bitcoin-owned mainnet-scale chainstate/block storage design, a sustained outbound sync controller, complete first-party validation for active-chain connection, richer bounded observability, and a split verification contract: deterministic local proof by default, explicit opt-in full-mainnet evidence for reviewers.

---
*Stack research for: Open Bitcoin v1.6 Mainnet Full-Sync Completion*
*Researched: 2026-06-11*
