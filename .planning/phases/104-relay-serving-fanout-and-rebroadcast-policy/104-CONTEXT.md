---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 104-2026-07-01T14-38-26
generated_at: 2026-07-01T14:38:26.627Z
---

# Phase 104: Relay Serving, Fanout, and Rebroadcast Policy - Context

**Gathered:** 2026-07-01
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Phase 104 lets relay-eligible peers request and hear about accepted relay-eligible
transactions without over-serving stale data or implying guaranteed public
propagation. It owns peer `getdata` transaction serving, accepted-transaction
fanout queues, local `sendrawtransaction` relay evidence, and a truthful
rebroadcast boundary for REL-01 through REL-04.

This phase builds on the Phase 100 default-off relay activation policy, Phase
101 txid/wtxid inventory scheduler, Phase 102 mempool outcome and orphan bridge,
and Phase 103 mempool lifecycle/cache cleanup. It does not add compact block
relay, package relay, bloom/filter serving, public relay defaults, public-network
CI, production full-node readiness, production service operation, or
production-funds wallet safety.

</domain>

<decisions>
## Implementation Decisions

### Relay-Eligible Transaction Serving

- **D-01:** Peer `getdata` transaction serving should consult a typed relay
  serving cache derived from accepted local or peer mempool outcomes, not a loose
  "known transaction" map. `MSG_TX` and `MSG_WTX` requests must resolve through
  txid/wtxid-aware identity before the managed shell emits `tx` or `notfound`.
- **D-02:** Serve transactions only when the transaction is currently
  relay-eligible and present in accepted mempool-backed runtime state. Unknown,
  stale, confirmed, rejected, replaced, evicted, expired, identity-mismatched,
  and non-transaction inventory requests should emit stable typed outcomes and
  peer-facing `notfound` where appropriate.
- **D-03:** Block serving and transaction serving should remain separate
  branches. Preserve existing block `getdata` behavior while deepening
  transaction serving around mempool lifecycle state and relay eligibility.
- **D-04:** Serving evidence must be low-cardinality, for example `served`,
  `unknown`, `stale`, `confirmed`, `rejected`, `replaced`, `evicted`,
  `expired`, `identity_mismatch`, and `not_relay_eligible`. Do not expose raw
  transaction hex, txids, wtxids, peer ids, endpoints, permission strings,
  class names, credentials, or dynamic labels in shared evidence.

### Accepted-Transaction Fanout

- **D-05:** Accepted or replaced `MempoolOutcome` values should enqueue relay
  announcements to eligible peers through a pure fanout policy. The policy
  should emit typed actions for announce, suppress, queue-cap, rate-limit, and
  cleanup decisions; adapters translate those actions into `inv` messages later.
- **D-06:** Fanout eligibility must reuse Phase 100 relay activation and peer
  eligibility decisions. Outbound and manual peers require explicit relay
  activation; inbound peers require inbound serving plus scoped relay permission
  effects. Protected admission alone must not make a peer eligible for
  transaction relay.
- **D-07:** Announcements must honor each peer's negotiated identity mode:
  txid-only peers receive `InventoryType::Transaction`; wtxidrelay peers receive
  `InventoryType::WitnessTransaction`. Do not announce a transaction in an
  identity form that contradicts the peer's negotiated mode.
- **D-08:** Suppression rules should cover the origin/requesting peer,
  already-have state, recent rejects, in-flight/requested state, mempool-known
  state, relay-disabled peers, non-eligible inbound peers, queue caps, and rate
  caps. Suppression should be observable through fixed labels rather than
  dynamic transaction or peer material.
- **D-09:** Per-peer fanout queues must be bounded and fake-clock testable. Tests
  should prove cap enforcement, deterministic draining, rate limits, identity
  negotiation, and cleanup after disconnect or mempool lifecycle removal without
  sleeps or public-network behavior.

### Local Submission Relay Evidence

- **D-10:** Local `sendrawtransaction` submissions should continue to enter
  mempool admission through the shared outcome contract. When accepted or
  replaced, the managed runtime should store the transaction for serving and
  enqueue relay fanout evidence, but RPC success must not imply public
  propagation is guaranteed.
- **D-11:** Local submission evidence should distinguish accepted, queued,
  suppressed, not eligible, relay disabled, and deferred rebroadcast cases. Keep
  detailed operator/RPC/metrics/log/support presentation for Phase 105, but make
  the internal outcome and tests available here so later surfaces share one
  contract.
- **D-12:** Rejected, duplicate, orphaned, evicted, and expired local outcomes
  must not enqueue public fanout. If a duplicate accepted transaction is already
  stored, serving state may remain unchanged, but the relay evidence should not
  claim a new announcement was broadcast.

### Rebroadcast Boundary

- **D-13:** Treat transaction rebroadcast scheduling as explicitly deferred in
  Phase 104. Implement the `REL-04` route by adding bounded, testable
  `rebroadcast_deferred` evidence across docs, internal status/policy output,
  and tests rather than adding a timer-driven rebroadcast loop.
- **D-14:** The deferred rebroadcast evidence should state that Open Bitcoin can
  serve and announce newly accepted transactions within the scoped relay
  boundary, but it does not yet periodically rebroadcast wallet/local mempool
  transactions or guarantee public propagation.
- **D-15:** Do not introduce wall-clock rebroadcast timers, public-network relay
  UAT, service-manager loops, production deployment gates, wallet production
  safety claims, or compact-block/package-relay behavior while closing REL-04.

### Lifecycle Cleanup And Coherence

- **D-16:** Mempool lifecycle events from Phase 103 must clean relay serving and
  fanout state. Block connect, conflict cleanup, replacement, trimming,
  eviction, expiry, reorg reconsideration, and disconnect cleanup should not
  leave transactions serveable or queued after they are no longer eligible.
- **D-17:** Reuse the Phase 101 scheduler vocabulary and Phase 102
  `MempoolOutcome` vocabulary where possible so request, admission, serving,
  fanout, and cleanup evidence stay compatible.
- **D-18:** Keep pure relay/fanout decisions in `open-bitcoin-network` or another
  pure functional-core surface; keep mempool mutation, transaction storage, and
  message translation in `open-bitcoin-node` managed shell adapters.

### Tests, Parity, And Guardrails

- **D-19:** Tests should lead with pure serving/fanout policy cases, then managed
  network integration cases for `getdata`, accepted peer transactions, local
  `sendrawtransaction`, lifecycle cleanup, and rebroadcast-deferred evidence.
- **D-20:** New first-party Rust source or test files under
  `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests` need parity
  breadcrumbs in `docs/parity/source-breadcrumbs.json`, citing Knots anchors
  where defensible. Use explicit `none` only for Open Bitcoin-only support
  infrastructure.
- **D-21:** If docs, parity roots, or verifier wiring change, add a deterministic
  Phase 104 checker with fixture tests and wire it into `bash scripts/verify.sh`
  after Phase 103. The checker should guard REL-01 through REL-04 evidence and
  reject claims for compact blocks, package relay, bloom/filter serving, public
  relay defaults, public-network CI, production readiness, and production-funds
  wallet use.
- **D-22:** Verification stays local and deterministic. The phase closeout target
  remains `bash scripts/verify.sh`; no public-network relay, service-manager,
  wall-clock soak, destructive repair, or production-deployment gate belongs in
  default verification.

### the agent's Discretion

The planner may choose exact type names, queue constants, rate-limit constants,
module split, and whether serving/fanout policy lives in
`open-bitcoin-network::peer::transaction_relay` or a sibling pure module. Prefer
small pure APIs plus thin managed shell translation. Keep Phase 105-facing
operator/RPC/metrics/log/support presentation out of this phase except where a
minimal shared contract is needed to make REL-04's deferred evidence truthful.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Project And v2.0 Scope

- `.planning/PROJECT.md` - Open Bitcoin parity, architecture, dependency,
  verification, and v2.0 transaction relay boundaries.
- `.planning/REQUIREMENTS.md` - REL-01 through REL-04 are owned by Phase 104;
  OBS-* and BOUND-* remain later phases.
- `.planning/ROADMAP.md` - Phase 104 purpose, scope, success criteria, and
  verification contract.
- `.planning/STATE.md` - Current milestone state, Phase 103 completion notes,
  deterministic verification caveats, and repo-local UAT command reminders.
- `.planning/phases/100-relay-activation-boundary-and-permission-semantics/100-CONTEXT.md`
  - Locked default-off relay activation, permission-effect, low-cardinality
  evidence, and no-claim decisions.
- `.planning/phases/101-transaction-inventory-identity-and-download-scheduling/101-CONTEXT.md`
  - Locked transaction identity, scheduler, request cleanup, and typed action
  decisions.
- `.planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-CONTEXT.md`
  - Locked outcome/orphan bridge and managed admission decisions.
- `.planning/phases/103-mempool-chainstate-lifecycle-and-durable-recovery/103-CONTEXT.md`
  - Locked mempool pressure, block/reorg lifecycle, relay-cache cleanup, and
  durable recovery decisions.

### Open Bitcoin Code And Tests

- `packages/open-bitcoin-network/src/relay.rs` - Phase 100 relay activation and
  peer eligibility policy.
- `packages/open-bitcoin-network/src/peer.rs` - `PeerManager`, peer actions, and
  transaction relay exports.
- `packages/open-bitcoin-network/src/peer/inventory_state.rs` - Current `getdata`,
  `inv`, `notfound`, and `tx` peer handling.
- `packages/open-bitcoin-network/src/peer/transaction_relay.rs` - Typed txid/wtxid
  identity, transaction download actions, and scheduler/orphan exports.
- `packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs` -
  Transaction request, already-have, recent-reject, in-flight, timeout, fallback,
  and cleanup scheduler.
- `packages/open-bitcoin-network/src/peer/transaction_relay/orphanage.rs` -
  Bounded orphan state and reconsideration vocabulary.
- `packages/open-bitcoin-network/src/peer/tests.rs` - Existing peer inventory,
  download, identity, `getdata`, and transaction handling tests.
- `packages/open-bitcoin-mempool/src/outcome.rs` - Stable `MempoolOutcome`
  labels and rejection categories.
- `packages/open-bitcoin-mempool/src/pool.rs` - Pure mempool admission,
  replacement, trimming, graph state, and entry indexes.
- `packages/open-bitcoin-mempool/src/pool/lifecycle.rs` - Mempool lifecycle
  cleanup summaries.
- `packages/open-bitcoin-mempool/src/pool/tests/lifecycle_cases.rs` - Block,
  conflict, trim, and lifecycle regression coverage.
- `packages/open-bitcoin-node/src/network.rs` - `ManagedPeerNetwork`, action
  processing, transaction indexes, and managed sync result contract.
- `packages/open-bitcoin-node/src/network/inventory.rs` - Current managed
  inventory serving and transaction storage indexes.
- `packages/open-bitcoin-node/src/network/admission_bridge.rs` - Managed bridge
  between peer/local transactions, mempool outcomes, orphan staging, and parent
  requests.
- `packages/open-bitcoin-node/src/network/action_translation.rs` - Transaction
  request action translation into targeted `getdata`.
- `packages/open-bitcoin-node/src/network/mempool_lifecycle.rs` - Managed
  block-connect and reorg cleanup hooks.
- `packages/open-bitcoin-node/src/network/tests.rs` - Existing managed-network
  block, mempool, transaction relay, and integration tests.
- `packages/open-bitcoin-node/src/network/tests/admission_bridge_cases.rs` -
  Phase 102 peer/local admission and orphan lifecycle integration tests.
- `packages/open-bitcoin-node/src/network/tests/mempool_lifecycle_cases.rs` -
  Phase 103 lifecycle and cache cleanup tests.
- `packages/open-bitcoin-rpc/src/dispatch/node.rs` - Current
  `sendrawtransaction`, `getmempoolinfo`, and `getnetworkinfo` dispatch.
- `packages/open-bitcoin-rpc/src/method/node.rs` - RPC response contracts,
  including local submission and network status shapes.
- `docs/parity/catalog/p2p.md` - P2P relay/orphan parity catalog and v2.0
  deferred-boundary wording.
- `docs/parity/catalog/mempool-policy.md` - Mempool policy parity catalog and
  Knots anchors.
- `docs/parity/index.json` - Machine-readable parity surface registry.
- `docs/parity/source-breadcrumbs.json` - Required source breadcrumb registry
  for new/touched first-party Rust files.
- `scripts/verify.sh` - Repo-native verification contract and checker ordering.
- `scripts/check-phase103-mempool-lifecycle-recovery.ts` - Deterministic
  phase-checker pattern to reuse if Phase 104 docs/checkers change.

### Bitcoin Knots Anchors

- `packages/bitcoin-knots/src/net_processing.cpp` - P2P transaction relay,
  `getdata`, `inv`, `notfound`, mempool interaction, relay suppression, and
  rebroadcast-related behavior anchors.
- `packages/bitcoin-knots/src/node/txdownloadman.h` - Transaction request,
  in-flight, cleanup, and announcement state contract.
- `packages/bitcoin-knots/src/node/txdownloadman_impl.cpp` - Request/fallback,
  already-have/recent-reject, accepted/rejected cleanup, and peer cleanup
  anchors.
- `packages/bitcoin-knots/src/protocol.h` - Inventory type and wire message
  anchors.
- `packages/bitcoin-knots/src/txmempool.h` - Mempool state, entry/index
  ownership, conflict tracking, descendants, and size policy anchors.
- `packages/bitcoin-knots/src/txmempool.cpp` - Mempool acceptance, trimming,
  replacement, removal, and rolling fee behavior anchors.
- `packages/bitcoin-knots/src/validation.cpp` - Block connect/disconnect,
  mempool removal, disconnected transaction handling, and validation/mempool
  interaction anchors.
- `packages/bitcoin-knots/test/functional/p2p_getdata.py` - Peer transaction and
  inventory serving behavior.
- `packages/bitcoin-knots/test/functional/p2p_tx_download.py` - Transaction
  announcement, request, and cleanup behavior.
- `packages/bitcoin-knots/test/functional/mempool_accept.py` - Admission policy,
  validation, and rejection behavior.
- `packages/bitcoin-knots/test/functional/mempool_reorg.py` - Disconnected block
  transaction reconsideration behavior.
- `packages/bitcoin-knots/test/functional/mempool_persist.py` - Mempool
  persistence and restart behavior that informs stale/accepted cache boundaries.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- `RelayEligibilityDecision` in `packages/open-bitcoin-network/src/relay.rs`:
  reuse this policy for fanout eligibility instead of adding separate booleans.
- `TxRelayId`, `TxRelayPeerMode`, `TxDownloadAction`, and
  `TxDownloadScheduler` in `packages/open-bitcoin-network/src/peer/transaction_relay.rs`:
  reuse typed identity and fixed evidence labels for serving/fanout actions.
- `MempoolOutcome` in `packages/open-bitcoin-mempool/src/outcome.rs`: use the
  accepted/rejected/replaced/orphaned/evicted/expired vocabulary to decide
  serving cache and fanout state.
- `ManagedPeerNetwork` transaction indexes in
  `packages/open-bitcoin-node/src/network.rs` and
  `packages/open-bitcoin-node/src/network/inventory.rs`: current storage can be
  deepened into relay-eligible serving state with lifecycle-aware cleanup.
- `process_transaction_relay_action` in
  `packages/open-bitcoin-node/src/network/action_translation.rs`: existing
  adapter pattern for translating pure relay actions into targeted peer
  messages.

### Established Patterns

- Pure-core policy lives under `open-bitcoin-network` or
  `open-bitcoin-mempool`; `open-bitcoin-node` owns managed mutation and message
  translation.
- Tests use deterministic fake-clock timestamps for request expiry and should
  keep avoiding sleeps or public-network behavior.
- Evidence labels are fixed and low-cardinality; support/operator surfaces
  avoid raw txids, wtxids, peer endpoints, permission strings, and credentials.
- Docs/checker changes use a phase-specific TypeScript checker wired into
  `scripts/verify.sh` after the previous phase checker.

### Integration Points

- `PeerManager::handle_getdata` currently returns `PeerAction::ServeInventory`
  with typed vectors; Phase 104 should give the managed shell enough typed
  context to classify served vs missing transaction outcomes.
- `ManagedPeerNetwork::process_actions` already handles `ServeInventory`,
  `ReceivedTransaction`, and `TransactionRelay`; Phase 104 can add fanout and
  serving evidence there without socket I/O in the pure core.
- `ManagedPeerNetwork::submit_local_transaction_outcome` and
  `sendrawtransaction` are the local submission path that needs accepted and
  queued relay evidence without propagation guarantees.
- Phase 103 lifecycle hooks remove stored transaction indexes on block connect,
  replacement, eviction, expiry, and reorg reconsideration; fanout/serving state
  must use the same cleanup path.

</code_context>

<specifics>
## Specific Ideas

- Prefer explicit rebroadcast deferral in Phase 104. This satisfies REL-04
  without adding a timer loop or broad propagation claim before observability and
  release-boundary phases are complete.
- Accepted and queued local submissions must be worded as internal relay
  evidence only; they do not promise public propagation.
- Keep public-network relay review opt-in and outside `bash scripts/verify.sh`.

</specifics>

<deferred>
## Deferred Ideas

- Periodic rebroadcast scheduling for local or wallet-originated transactions is
  deferred beyond Phase 104. Phase 104 should record `rebroadcast_deferred`
  evidence instead of implementing a timer-driven rebroadcast loop.
- Phase 105 owns rich RPC, CLI, dashboard, metrics, structured-log, and support
  bundle presentation for relay and mempool evidence.
- Phase 106 owns final parity traceability, UAT guidance, README/operator docs,
  and release-boundary guardrails across the full v2.0 milestone.

</deferred>

*Phase: 104-relay-serving-fanout-and-rebroadcast-policy*
*Context gathered: 2026-07-01*
