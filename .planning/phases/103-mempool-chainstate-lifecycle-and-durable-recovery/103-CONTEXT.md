---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 103-2026-07-01T12-38-00
generated_at: 2026-07-01T12:38:00.304Z
---

# Phase 103: Mempool Chainstate Lifecycle and Durable Recovery - Context

**Gathered:** 2026-07-01
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Phase 103 makes accepted mempool state coherent across chainstate transitions, runtime relay caches, restart, and repair boundaries. It owns MEM-03 through MEM-06: truthful mempool pressure/trimming/fee-floor evidence, block-connect removal of confirmed and conflicting transactions, bounded block-disconnect or reorg reconsideration, and durable accepted-transaction persistence/recovery.

This phase may add pure mempool lifecycle APIs, managed-network chainstate hooks, storage codecs/namespaces, deterministic recovery tests, and parity evidence for the lifecycle boundary. It must not implement relay serving, transaction fanout, rebroadcast policy, broad RPC/operator/support observability, final release closeout, compact block relay, package relay, bloom/filter serving, public relay defaults, public-network relay CI, production full-node readiness, or production-funds wallet support.

</domain>

<decisions>
## Implementation Decisions

### Mempool Pressure And Trimming Evidence

- **D-01:** MEM-03 should be represented as typed, low-cardinality pressure and trimming evidence in the mempool/runtime contract, not as prose-only operator copy. At minimum, expose transaction count, virtual size, configured capacity, min relay fee floor, incremental relay fee floor, evicted/trimmed outcomes, and whether full Knots trimming parity remains deferred.
- **D-02:** Existing `PolicyConfig::max_mempool_virtual_size`, `FeeRate`, `MempoolOutcome::Evicted`, `AdmissionResult::evicted`, and `ManagedMempoolInfo` are the starting points. Prefer adding small typed summaries rather than scattering derived capacity math across RPC, CLI, or peer code.
- **D-03:** If Knots behavior is not fully implemented, docs and tests must say so explicitly. Do not claim full dynamic rolling minimum fee, package/cluster mempool, or production public relay parity unless this phase actually implements and verifies it.

### Block Connect Cleanup

- **D-04:** Block connect must remove confirmed transactions and mempool conflicts from the pure mempool state and from relay-serving caches owned by the managed runtime. Cleanup should preserve coherent `entries`, `spent_outpoints`, child/parent links, totals, and stored `transactions_by_txid` / `transactions_by_wtxid` indexes.
- **D-05:** The pure mempool crate should own transaction graph cleanup and return structured lifecycle outcomes. The managed shell should call that API after successful block connection and translate cleanup into relay/download/orphan cache maintenance. Peer/socket code should not mutate mempool state directly.
- **D-06:** Tests must cover confirmed transaction removal, descendant/conflict removal, replacement/eviction interaction, relay cache removal, and no stale request/cache state after block connect.

### Disconnect And Reorg Reconsideration

- **D-07:** Block disconnect or reorg handling should reconsider eligible disconnected transactions through a bounded candidate flow. Reuse the Phase 102 `MempoolOutcome` and orphan bridge where possible so candidates become accepted, rejected, orphaned, evicted, or expired through one stable outcome vocabulary.
- **D-08:** Reconsideration must be deterministic and bounded. It must not recurse unboundedly, add sleeps, use public-network behavior, or imply package relay. If descendants or package-shaped cases are deferred, record the deferred boundary in parity docs and status evidence.
- **D-09:** Managed `reorg_to_branch` and chainstate disconnect APIs are the integration points. The planner should preserve existing chainstate semantics and avoid making the chainstate engine aware of mempool internals.

### Durable Mempool Persistence And Recovery

- **D-10:** Add dedicated durable mempool storage rather than folding accepted transactions into generic runtime metadata. Use a stable codec/schema with save, load, remove, restart, stale-record, corruption, and schema-mismatch behavior tested through the existing Fjall adapter style.
- **D-11:** Persist only accepted mempool transaction state and enough metadata to reconstruct the pure mempool under current policy/chainstate constraints. On load, records that are confirmed, stale, policy-incompatible, corrupt, or schema-incompatible must be repaired, dropped with typed evidence, or surfaced as a typed storage recovery error.
- **D-12:** Durable recovery must align with existing `StorageNamespace`, `SchemaVersion`, `StorageError`, `StorageRecoveryAction`, and `FjallNodeStore` patterns. Operator-destructive repair remains out of scope unless a future phase explicitly plans it.

### Parity, Tests, And Guardrails

- **D-13:** Unit tests should lead with pure mempool lifecycle behavior, then add managed-network integration tests for block connect and reorg cache coherence, then Fjall adapter restart/recovery tests. Use Arrange, Act, Assert comments for non-trivial tests.
- **D-14:** New Rust source or test files under first-party packages need parity breadcrumbs in `docs/parity/source-breadcrumbs.json`, citing Knots anchors where defensible. Use explicit `none` only for Open Bitcoin-only storage/support infrastructure.
- **D-15:** If parity docs or verifier wiring change, add a deterministic Phase 103 checker with fixture tests and wire it into `bash scripts/verify.sh` after Phase 102. The checker should reject claims that relay serving/fanout, rebroadcast, compact blocks, package relay, bloom/filter serving, public relay defaults, public-network CI, production readiness, or production-funds wallet use are complete in Phase 103.
- **D-16:** Verification stays local and deterministic. The phase closeout target remains `bash scripts/verify.sh`; no public-network relay, service-manager, wall-clock soak, destructive repair, or production-deployment gate belongs in default verification.

### the agent's Discretion

The planner may choose exact type names, storage key shape, module split, and plan granularity. Prefer small pure lifecycle APIs in `open-bitcoin-mempool`, thin managed shell hooks in `open-bitcoin-node`, and storage codec helpers that match existing Fjall/snapshot patterns. Keep later RPC/operator/support presentation out of this phase unless required to truthfully expose MEM-03 pressure status.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Project And v2.0 Scope

- `.planning/PROJECT.md` - Open Bitcoin parity, architecture, dependency, verification, and v2.0 transaction relay boundaries.
- `.planning/REQUIREMENTS.md` - MEM-03 through MEM-06 are owned by Phase 103; REL-*, OBS-*, and BOUND-* are later phases.
- `.planning/ROADMAP.md` - Phase 103 purpose, scope, success criteria, and verification contract.
- `.planning/STATE.md` - Current milestone state, Phase 102 completion notes, deterministic verification caveats, and repo-local UAT command reminders.
- `.planning/phases/100-relay-activation-boundary-and-permission-semantics/100-CONTEXT.md` - Locked default-off relay activation, permission-effect, low-cardinality evidence, and no-claim decisions.
- `.planning/phases/101-transaction-inventory-identity-and-download-scheduling/101-CONTEXT.md` - Locked transaction identity, scheduler, request cleanup, and typed action decisions.
- `.planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-CONTEXT.md` - Locked outcome/orphan bridge and managed admission decisions that Phase 103 builds on.
- `.planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-04-SUMMARY.md` - Phase 102 parity checker and verifier wiring pattern.

### Open Bitcoin Code And Tests

- `packages/open-bitcoin-mempool/src/pool.rs` - Pure mempool admission, replacement, trimming, graph state, and current entry indexes.
- `packages/open-bitcoin-mempool/src/outcome.rs` - Stable `MempoolOutcome` labels and rejection categories for lifecycle/reconsideration outcomes.
- `packages/open-bitcoin-mempool/src/types.rs` - `PolicyConfig`, `MempoolEntry`, `AdmissionResult`, fee rates, and aggregate stats.
- `packages/open-bitcoin-mempool/src/pool/tests.rs` - Current admission, replacement, limit, trimming, and invariant tests.
- `packages/open-bitcoin-mempool/src/pool/tests/outcome_cases.rs` - Current outcome and no-partial-mutation regression coverage.
- `packages/open-bitcoin-node/src/network.rs` - `ManagedPeerNetwork`, block connect/reorg hooks, mempool info, and runtime transaction indexes.
- `packages/open-bitcoin-node/src/network/admission_bridge.rs` - Phase 102 managed bridge between peer transactions, mempool outcomes, orphan staging, and parent requests.
- `packages/open-bitcoin-node/src/network/action_translation.rs` - Transaction request cleanup and targeted `getdata` translation patterns.
- `packages/open-bitcoin-node/src/network/tests.rs` - Existing managed-network block, mempool, transaction relay, and in-memory integration tests.
- `packages/open-bitcoin-node/src/network/tests/admission_bridge_cases.rs` - Phase 102 peer/local admission and orphan lifecycle integration tests.
- `packages/open-bitcoin-network/src/peer/transaction_relay/orphanage.rs` - Bounded orphan state and reconsideration vocabulary.
- `packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs` - Transaction download request, already-have, recent-reject, in-flight, timeout, fallback, and cleanup scheduler.
- `packages/open-bitcoin-node/src/storage.rs` - Storage namespaces, schema versions, recovery markers, typed storage errors, and recovery actions.
- `packages/open-bitcoin-node/src/storage/fjall_store.rs` - Fjall keyspace, schema, save/load/remove, runtime metadata, and recovery marker patterns.
- `packages/open-bitcoin-node/src/storage/snapshot_codec.rs` - Existing stable codec style for durable snapshots.
- `docs/parity/catalog/mempool-policy.md` - Mempool policy parity catalog and Knots anchors.
- `docs/parity/catalog/p2p.md` - P2P relay/orphan parity catalog and v2.0 deferred-boundary wording.
- `docs/parity/index.json` - Machine-readable parity surface registry.
- `docs/parity/source-breadcrumbs.json` - Required source breadcrumb registry for new/touched first-party Rust files.
- `scripts/check-phase102-orphan-admission-bridge.ts` - Deterministic phase-checker pattern to reuse for Phase 103 if docs/checkers change.
- `scripts/verify.sh` - Repo-native verification contract and checker ordering.

### Bitcoin Knots Anchors

- `packages/bitcoin-knots/src/txmempool.h` - Mempool state, entry/index ownership, conflict tracking, descendants, and size policy anchors.
- `packages/bitcoin-knots/src/txmempool.cpp` - Mempool acceptance, trimming, replacement, removal, and rolling fee behavior anchors.
- `packages/bitcoin-knots/src/validation.cpp` - Block connect/disconnect, mempool removal, disconnected transaction handling, and validation/mempool interaction anchors.
- `packages/bitcoin-knots/src/node/txdownloadman.h` - Transaction request/cleanup contract relevant to relay-cache coherence.
- `packages/bitcoin-knots/src/node/txdownloadman_impl.cpp` - Accepted/rejected cleanup, already-have/recent-reject, and request-state cleanup anchors.
- `packages/bitcoin-knots/src/net_processing.cpp` - P2P transaction relay, mempool interaction, orphan handling, and block/reorg processing hazards.
- `packages/bitcoin-knots/src/policy/policy.h` - Standardness and relay policy declarations.
- `packages/bitcoin-knots/src/policy/rbf.cpp` - Replacement policy implementation.
- `packages/bitcoin-knots/test/functional/mempool_limit.py` - Mempool trimming and fee-floor behavior.
- `packages/bitcoin-knots/test/functional/mempool_persist.py` - Mempool persistence and restart behavior.
- `packages/bitcoin-knots/test/functional/mempool_reorg.py` - Disconnected block transaction reconsideration behavior.
- `packages/bitcoin-knots/test/functional/mempool_accept.py` - Admission policy, validation, and rejection behavior.
- `packages/bitcoin-knots/test/functional/p2p_orphan_handling.py` - Orphan and parent request behavior carried forward from Phase 102.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- `Mempool` already owns accepted entries, spent outpoint indexes, virtual-size totals, replacement sets, trimming, and policy limits.
- `MempoolOutcome` already gives local and peer callers a shared low-cardinality outcome vocabulary.
- `ManagedPeerNetwork` already owns chainstate, managed mempool, peer manager, orphanage, block cache, and in-memory txid/wtxid transaction indexes, making it the right shell for block-connect cleanup and reorg bridges.
- `TxDownloadScheduler`, `TxOrphanage`, and action translation already provide request cleanup and reconsideration vocabulary that should be reused instead of creating socket-side mempool effects.
- `FjallNodeStore`, `StorageNamespace`, `SchemaVersion`, `StorageError`, and `snapshot_codec` already provide durable storage and recovery patterns for a mempool persistence namespace.

### Established Patterns

- Pure lifecycle and policy logic belongs in `open-bitcoin-mempool`; peer relay/download decisions belong in `open-bitcoin-network`; managed mutation, durable storage, and runtime orchestration belong in `open-bitcoin-node`.
- Phase checkers are Bun/TypeScript fixed-corpus scripts with fixture mutation tests and explicit verifier-order checks.
- Evidence labels must be fixed and low-cardinality. Do not expose raw transaction hex, txids, wtxids, peer endpoints, permission strings, raw config, credentials, or dynamic labels in shared docs/status/support planning.
- Default verification must remain deterministic and local through `bash scripts/verify.sh`.

### Integration Points

- Add pure lifecycle helpers near `packages/open-bitcoin-mempool/src/pool.rs`, likely split into a child file if `pool.rs` would grow too large.
- Call lifecycle helpers from `ManagedPeerNetwork::connect_local_block`, `connect_stored_block`, and `reorg_to_branch` after successful chainstate changes.
- Extend `FjallNodeStore` with mempool save/load/remove APIs and codec helpers without weakening existing chainstate, wallet, metrics, runtime, or schema isolation.
- Update parity breadcrumbs, docs, and checker roots only for files actually touched by the implementation.

</code_context>

<specifics>
## Specific Ideas

- Favor a `MempoolLifecycleOutcome` or similarly concrete type for block-connect/reorg removal and reconsideration evidence.
- Favor explicit stale/corrupt/schema recovery outcomes for durable mempool records instead of silently dropping records without evidence.
- Treat rolling fee-floor parity as a likely gap unless implemented with tests; document it truthfully rather than letting `min_relay_feerate` imply full Knots trimming behavior.
- Preserve repo-local UAT command lessons if operator-facing guidance changes: use explicit Cargo and Bazel command forms, not a bare `open-bitcoin` alias.

</specifics>

<deferred>
## Deferred Ideas

Relay serving, fanout, rebroadcast, RPC methods beyond narrow truth fields, CLI/dashboard rendering, metrics/log/support bundle redaction, final parity/UAT/release-boundary closeout, compact block relay, package relay, bloom/filter serving, public relay defaults, public-network relay CI, production full-node readiness, and production-funds wallet use remain outside Phase 103.

</deferred>

***

*Phase: 103-mempool-chainstate-lifecycle-and-durable-recovery*
*Context gathered: 2026-07-01*
