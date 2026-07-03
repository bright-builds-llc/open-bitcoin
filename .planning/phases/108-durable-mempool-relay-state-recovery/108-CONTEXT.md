---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 108-2026-07-03T14-09-06
generated_at: 2026-07-03T14:09:06.388Z
---

# Phase 108: Durable Mempool Relay State Recovery - Context

**Gathered:** 2026-07-03
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Phase 108 makes accepted mempool transactions recovered from durable storage rejoin the same relay-serving, fanout, lifecycle cleanup, and sanitized operator evidence paths as live accepted transactions. The phase owns restart replay, stale or invalid recovered records, relay cache rehydration, cleanup after block/reorg/replacement/eviction/expiry boundaries, and deterministic evidence. It does not add public relay by default, compact block relay, package relay, bloom/filter serving, production service operation, public-network CI, production full-node readiness, production-funds wallet safety, or propagation guarantees.

</domain>

<decisions>
## Implementation Decisions

### Recovery Replay Into Relay State

- **D-01:** Durable mempool recovery must replay accepted records through a managed recovery API that repopulates the pure mempool, relay-serving cache, fanout identity indexes, and sanitized evidence together. Do not treat `MempoolSnapshot::replay_into_mempool` as complete until accepted recovered records are also serveable or suppressible through the same typed relay policy used by live accepted transactions.
- **D-02:** Recovered accepted records should be recorded as accepted serving state only after they pass current chainstate, policy, and consensus checks. Confirmed, duplicate, missing-parent, rejected, stale, evicted, corrupt, or schema-incompatible records must produce typed recovery evidence and must not remain serveable or queued for fanout.
- **D-03:** Recovery must not perform socket I/O, public fanout, or `inv` emission during startup replay. Recovered records may seed serveable state and aggregate counters, but actual peer messages should occur only through existing post-recovery relay/fanout paths when a normal runtime event drains them.

### Lifecycle Coherence After Restart

- **D-04:** Block connect, conflict removal, replacement, eviction, expiry, and reorg reconsideration must clean recovered serving and fanout state through the same cleanup paths used for live mempool outcomes. No stale recovered transaction should remain in `RelayServingCache`, fanout queues, recent-reject state, or txid/wtxid indexes after it is confirmed, replaced, evicted, expired, or rejected.
- **D-05:** Reorg reconsideration should keep Phase 103's deterministic bounded flow. Disconnected transactions may re-enter the mempool and relay-serving state only through the stable `MempoolOutcome` vocabulary; package relay and unbounded descendant recursion remain out of scope.
- **D-06:** Restart recovery should preserve txid/wtxid identity consistency. A recovered transaction must serve as `MSG_TX` or `MSG_WTX` according to the requesting peer's negotiated relay mode and must report identity mismatch through the existing low-cardinality serve outcome.

### Operator Evidence And Redaction

- **D-07:** RPC, CLI, dashboard, metrics, logs, and support bundles should project recovered relay state through the existing `RelayEvidenceStatus`, `RelayEvidenceCounters`, `ManagedRelayServingInfo`, `ManagedRelayFanoutInfo`, and support redaction helpers. Add only fixed, aggregate recovery labels or counters where the current status contract cannot represent recovery safely.
- **D-08:** Evidence must distinguish live admission from recovery replay without implying public propagation. Acceptable public/operator vocabulary is bounded labels and counts such as `recovered`, `dropped_confirmed`, `dropped_duplicate`, `dropped_missing_parent`, `dropped_policy_incompatible`, `dropped_evicted`, `served`, `suppressed`, `announced`, `evicted`, and `expired`.
- **D-09:** Support, logs, metrics, status, and docs must not expose raw transaction hex, txids, wtxids, peer ids, endpoints, permission strings, class names, credentials, dynamic metric labels, raw structured-log bodies, or free-form rejection text.

### Storage, Corruption, And Repair

- **D-10:** Recovery should use the existing Open Bitcoin-owned mempool snapshot codec and Fjall namespace. Schema mismatch, malformed snapshots, partial writes, and corrupt records should map to existing `StorageError`, `StorageRecoveryAction`, or tightly scoped mempool recovery evidence instead of silent drops.
- **D-11:** Destructive repair, source datadir mutation, automatic support upload, compaction, reindexing, or manual store surgery remains out of scope. If a corrupt snapshot cannot be recovered safely, the runtime should surface typed diagnosis and drop or quarantine only through an explicit safe path.
- **D-12:** Snapshot persistence should remain accepted-mempool-state-only. Do not persist peer-specific queues, peer ids, raw permission class names, raw endpoint material, or fanout state that would become stale across restart.

### Tests, Docs, And Guardrails

- **D-13:** Tests should lead with pure snapshot replay and mempool lifecycle cases, then managed-network restart/recovery integration tests proving serving/fanout cache rehydration and cleanup. Use deterministic temp stores, fake chainstate, and fake peers; no sleeps, public-network relay, service-manager checks, or wall-clock soak belong in default verification.
- **D-14:** Add deterministic checker coverage if docs, parity roots, verifier order, or evidence registries change. The checker should guard MEM-04, MEM-05, MEM-06, REL-01, and REL-02 ownership for recovered state and reject positive claims for public relay by default, compact block relay, package relay, bloom/filter serving, public-network relay CI, production service operation, production full-node readiness, production-funds wallet safety, production-funds wallet use, and guaranteed public propagation.
- **D-15:** New or touched first-party Rust source or test files under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests` need parity breadcrumbs through `docs/parity/source-breadcrumbs.json`, citing Knots anchors where defensible and using explicit `none` only for Open Bitcoin-only support infrastructure.

### the agent's Discretion

The planner may choose exact type names, module split, recovery evidence counters, and whether recovery replay lives beside `mempool_snapshot`, `ManagedPeerNetwork`, or a focused child module. Prefer small pure recovery functions, thin managed shell wiring, and reuse of the existing relay-serving/fanout/status contracts over a separate recovery-only relay model.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Project And Phase Scope

- `.planning/PROJECT.md` - v2.0 relay/mempool boundary, architecture constraints, dependency policy, and deferred production/public relay claims.
- `.planning/REQUIREMENTS.md` - MEM-04, MEM-05, MEM-06, REL-01, and REL-02 are pending under Phase 108.
- `.planning/ROADMAP.md` - Phase 108 purpose, scope, success criteria, verification contract, dependency on Phase 107, and deferred scope.
- `.planning/STATE.md` - Current milestone state, Phase 108 pending position, repo-local UAT command reminders, and deterministic verification caveats.
- `AGENTS.md` - Repo-local verification, parity breadcrumb, GSD, Rust, and Bright Builds guidance.

### Prior Locked Decisions

- `.planning/phases/100-relay-activation-boundary-and-permission-semantics/100-CONTEXT.md` - Default-off relay activation, peer eligibility, scoped permissions, low-cardinality evidence, and no-claim guardrails.
- `.planning/phases/101-transaction-inventory-identity-and-download-scheduling/101-CONTEXT.md` - Txid/wtxid identity, typed request state, scheduler actions, cleanup, and deterministic fake-clock tests.
- `.planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-CONTEXT.md` - Stable `MempoolOutcome`, orphan staging, parent request, managed admission bridge, and low-cardinality outcome vocabulary.
- `.planning/phases/103-mempool-chainstate-lifecycle-and-durable-recovery/103-CONTEXT.md` - Durable mempool snapshots, replay statuses, block/reorg lifecycle cleanup, and storage recovery decisions.
- `.planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-CONTEXT.md` - Relay-serving cache, fanout queues, lifecycle cleanup, negotiated identity, local submission evidence, and rebroadcast-deferred boundary.
- `.planning/phases/105-operator-rpc-metrics-logs-and-support-evidence/105-CONTEXT.md` - Shared sanitized status, metrics, logs, CLI/dashboard/support projections, and redaction constraints.
- `.planning/phases/106-parity-traceability-uat-and-release-boundary-guardrails/106-CONTEXT.md` - Deterministic checker, parity-root, UAT, and release-boundary guardrail pattern.
- `.planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-CONTEXT.md` - Runtime activation propagation, download eligibility gate, sanitized status contract reuse, and Phase 108 handoff boundary.

### Open Bitcoin Code And Tests

- `packages/open-bitcoin-node/src/storage/mempool_snapshot.rs` - `MempoolSnapshot`, `MempoolRecoveryStatus`, and current replay into pure mempool state.
- `packages/open-bitcoin-node/src/storage/fjall_store/mempool.rs` - Fjall save/load/clear adapter for accepted mempool snapshots.
- `packages/open-bitcoin-node/src/storage/snapshot_codec.rs` - Versioned snapshot codec and mempool snapshot DTO conversion.
- `packages/open-bitcoin-node/src/storage/fjall_store/tests.rs` - Existing snapshot persistence, corruption, and recovery-marker tests.
- `packages/open-bitcoin-node/src/mempool.rs` - `ManagedMempool` submission wrapper and mempool access seam.
- `packages/open-bitcoin-node/src/network.rs` - `ManagedPeerNetwork`, transaction indexes, chainstate/mempool ownership, and managed runtime shell.
- `packages/open-bitcoin-node/src/network/inventory.rs` - Managed transaction storage, serving, and transaction index cleanup helpers.
- `packages/open-bitcoin-node/src/network/relay_serving.rs` - `RelayServingCache`, serve request classification, eligibility context, and serving evidence.
- `packages/open-bitcoin-node/src/network/relay_fanout.rs` - `ManagedRelayFanoutState`, fanout queue cleanup, local submission evidence, and aggregate relay evidence projection.
- `packages/open-bitcoin-node/src/network/mempool_lifecycle.rs` - Block connect and reorg cleanup hooks that must also cover recovered state.
- `packages/open-bitcoin-node/src/network/admission_bridge.rs` - Peer/local transaction admission bridge and `apply_admitted_outcome` integration point.
- `packages/open-bitcoin-node/src/network/tests/relay_serving_cases.rs` - Existing managed serving cases for accepted, stale, confirmed, replaced, evicted, expired, and notfound outcomes.
- `packages/open-bitcoin-node/src/network/tests/relay_fanout_cases.rs` - Existing managed fanout and lifecycle cleanup cases.
- `packages/open-bitcoin-node/src/network/tests/mempool_lifecycle_cases.rs` - Existing Phase 103 lifecycle cleanup tests.
- `packages/open-bitcoin-network/src/peer/transaction_relay/serving.rs` - Pure serve outcome/status classification and identity behavior.
- `packages/open-bitcoin-network/src/peer/transaction_relay/fanout.rs` - Pure fanout action and cleanup vocabulary.
- `packages/open-bitcoin-mempool/src/outcome.rs` - Stable mempool outcome labels used by peer and local submissions.
- `packages/open-bitcoin-mempool/src/pool.rs` - Pure mempool admission, replacement, trimming, graph state, and entry indexes.
- `packages/open-bitcoin-mempool/src/pool/lifecycle.rs` - Pure mempool lifecycle cleanup summaries.
- `packages/open-bitcoin-rpc/src/dispatch/node.rs` - `sendrawtransaction`, `getmempoolinfo`, `getnetworkinfo`, and `openbitcoinnetworkstatus` projections.
- `packages/open-bitcoin-rpc/src/method/node.rs` - Baseline-shaped RPC response contracts plus Open Bitcoin network status response.
- `packages/open-bitcoin-cli/src/operator/status/render/relay.rs` - Human status relay evidence rendering.
- `packages/open-bitcoin-cli/src/operator/dashboard/model/relay.rs` - Dashboard relay projection from shared status.
- `packages/open-bitcoin-cli/src/operator/support/redaction.rs` - Support redaction for relay/mempool evidence.
- `packages/open-bitcoin-cli/src/operator/support/render/relay.rs` - Support Markdown relay/mempool evidence rendering.
- `docs/parity/source-breadcrumbs.json` - Required breadcrumb registry for new or touched first-party Rust files.
- `scripts/check-phase103-mempool-lifecycle.ts` - Durable mempool/lifecycle checker pattern.
- `scripts/check-phase104-relay-serving-fanout.ts` - Relay serving/fanout checker pattern.
- `scripts/check-phase105-operator-relay-evidence.ts` - Operator evidence and redaction checker pattern.
- `scripts/check-phase107-runtime-relay-activation-download-eligibility.ts` - Phase 107 no-claim and verifier-order checker pattern.
- `scripts/verify.sh` - Repo-native verification contract and checker order.

### Docs, Parity, And Operator Surfaces

- `docs/architecture/status-snapshot.md` - Shared status snapshot contract and relay/mempool evidence boundaries.
- `docs/architecture/operator-observability.md` - Low-cardinality metrics/logs/support constraints for relay/mempool material.
- `docs/operator/runtime-guide.md` - Repo-local Cargo/Bazel UAT command style and Phase 107 relay no-claim wording to update for Phase 108 if operator docs change.
- `docs/parity/catalog/p2p.md` - P2P relay, serving, fanout, and deferred-scope parity catalog.
- `docs/parity/catalog/mempool-policy.md` - Mempool policy, lifecycle, and durable persistence parity catalog.
- `docs/parity/catalog/rpc-cli-config.md` - RPC/CLI baseline behavior and Open Bitcoin operator extension catalog.
- `docs/parity/checklist.md` - Human-readable parity checklist entries.
- `docs/parity/index.json` - Machine-readable parity surface registry and evidence roots.

### Bitcoin Knots Anchors

- `packages/bitcoin-knots/src/node/mempool_persist.cpp` - Baseline mempool persistence and restart behavior.
- `packages/bitcoin-knots/src/node/mempool_persist.h` - Mempool persistence contract declarations.
- `packages/bitcoin-knots/test/functional/mempool_persist.py` - Functional mempool persistence and recovery expectations.
- `packages/bitcoin-knots/src/txmempool.h` - Mempool state, entry/index ownership, conflicts, descendants, and policy structures.
- `packages/bitcoin-knots/src/txmempool.cpp` - Mempool acceptance, trimming, replacement, removal, and rolling fee behavior.
- `packages/bitcoin-knots/src/validation.cpp` - Block connect/disconnect, mempool removal, disconnected transaction handling, and validation/mempool interaction anchors.
- `packages/bitcoin-knots/src/net_processing.cpp` - P2P transaction relay, request serving, fanout, suppression, and mempool interaction anchors.
- `packages/bitcoin-knots/src/node/txdownloadman.h` - Transaction request, in-flight, cleanup, and announcement state contract.
- `packages/bitcoin-knots/src/node/txdownloadman_impl.cpp` - Request/fallback, already-have/recent-reject, accepted/rejected cleanup, and peer cleanup anchors.
- `packages/bitcoin-knots/src/protocol.h` - Inventory type and wire message anchors.
- `packages/bitcoin-knots/test/functional/p2p_getdata.py` - Peer transaction and inventory serving behavior.
- `packages/bitcoin-knots/test/functional/p2p_tx_download.py` - Transaction announcement, request, and cleanup behavior.
- `packages/bitcoin-knots/test/functional/mempool_accept.py` - Admission policy, validation, and rejection behavior.
- `packages/bitcoin-knots/test/functional/mempool_reorg.py` - Disconnected block transaction reconsideration behavior.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- `MempoolSnapshot::replay_into_mempool` already returns stable recovery statuses, but currently only repopulates the pure mempool.
- `FjallNodeStore::save_mempool_snapshot`, `load_mempool_snapshot`, and `clear_mempool_snapshot` already own durable accepted-mempool snapshot persistence.
- `RelayServingCache::record_accepted`, `record_replaced`, `record_status`, and `remove_transactions` already map accepted/lifecycle outcomes into serveable or suppressed state.
- `ManagedRelayFanoutState::record_admission_outcome` and `cleanup_transactions` already own queue identity and cleanup state for accepted/replaced/evicted/expired outcomes.
- `ManagedPeerNetwork::apply_connected_block_mempool_lifecycle` and `apply_reorg_mempool_lifecycle` already clean live relay-serving and fanout state through block/reorg boundaries.
- `RelayEvidenceStatus` and support/status renderers already provide a sanitized aggregate projection that can carry recovered-state counters without exposing sensitive transaction or peer material.

### Established Patterns

- Pure replay and lifecycle decisions belong in `open-bitcoin-mempool`, `open-bitcoin-network`, or small pure helpers; `open-bitcoin-node` owns Fjall, managed runtime mutation, and message translation.
- Baseline RPC methods stay compatibility-shaped; richer Open Bitcoin relay evidence belongs in `openbitcoinnetworkstatus`, operator status, dashboard, metrics, logs, and support bundle projections.
- Deterministic phase checkers are Bun/TypeScript scripts with companion tests, fixed target files, forbidden positive-claim scanning, and explicit verifier order in `scripts/verify.sh`.
- Default verification remains local and deterministic. Public-network relay review, service-manager operation, wall-clock soak, destructive repair, and production deployment remain outside `bash scripts/verify.sh`.

### Integration Points

- Add a managed recovery method that loads/replays snapshots and, for recovered accepted outcomes, calls the same serving/fanout admission helpers used by live accepted transactions without emitting socket messages during replay.
- Extend lifecycle cleanup tests so recovered records are removed from serving and fanout state after block connect, replacement, eviction, expiry, and reorg reconsideration.
- Extend status/support evidence only through shared sanitized contracts. If new recovery counters are required, add fixed fields and redaction tests.
- Refresh parity roots and checker wiring only after concrete implementation paths exist.

</code_context>

<specifics>
## Specific Ideas

- Prefer reuse of the Phase 104 serving/fanout state instead of adding a parallel recovered-transaction cache.
- Keep restart replay evidence aggregate and non-promissory: recovered state can prove local durable state rejoined relay-serving policy, not public propagation.
- Treat corrupt or incompatible snapshot handling as diagnosis/safe-drop evidence unless a future phase explicitly scopes destructive repair.

</specifics>

<deferred>
## Deferred Ideas

- Public transaction relay by default.
- Public-network relay UAT as a default CI or pre-commit gate.
- Compact block relay, package relay, bloom/filter serving, and broad mempool serving beyond the v2.0 scoped relay boundary.
- Production service operation, production full-node readiness, production-funds wallet safety, packaging, GUI, hosted dashboard, migration apply mode, automatic support-bundle upload, and destructive repair.

</deferred>

*Phase: 108-durable-mempool-relay-state-recovery*
*Context gathered: 2026-07-03*
