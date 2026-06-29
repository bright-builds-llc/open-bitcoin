---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 101-2026-06-29T21-00-59
generated_at: 2026-06-29T21:03:44.720Z
---

# Phase 101: Transaction Inventory Identity and Download Scheduling - Context

**Gathered:** 2026-06-29
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Phase 101 builds the txid/wtxid-aware transaction inventory identity and bounded download scheduler before orphan handling, mempool admission, durable mempool lifecycle, relay serving, fanout, RPC, metrics, logs, support bundles, or release closeout are wired in.

The phase may handle transaction `inv`, `getdata`, `tx`, and `notfound` decisions with typed txid/wtxid identity, track per-peer negotiation/already-have/request state, suppress duplicate and stale requests, and emit pure typed peer actions for request, suppression, fallback, timeout, `notfound`, received-transaction cleanup, and disconnect cleanup.

The phase must not implement compact block relay, package relay, bloom/filter serving, public relay defaults, production service claims, orphan staging/admission behavior owned by Phase 102, mempool persistence owned by Phase 103, relay serving/fanout owned by Phase 104, or operator/RPC/support surfaces owned by Phase 105+ except where a narrow test seam is required.

</domain>

<decisions>
## Implementation Decisions

### Inventory Identity And Negotiation

- **D-01:** Introduce an Open Bitcoin-owned typed transaction relay identity, for example `TxRelayId`, that distinguishes `Txid(Txid)` from `Wtxid(Wtxid)`. Raw `Hash32` plus `InventoryType` should be parsed at the wire boundary and converted into typed identity before request scheduling logic sees it.
- **D-02:** Preserve BIP339-style negotiation behavior already represented by `remote_wtxidrelay`: txid-only peers announce and request `InventoryType::Transaction`; wtxidrelay peers announce and request `InventoryType::WitnessTransaction`.
- **D-03:** Treat inventory identity mismatches as suppression decisions, not best-effort fallback. `MSG_TX` from a wtxidrelay peer and `MSG_WTX` from a txid-only peer should not create stale request state.
- **D-04:** Keep block inventory behavior separate from transaction relay scheduling. Existing header/block request paths may remain in `PeerManager`, but Phase 101 transaction logic should not blur block and transaction request accounting.

### Per-Peer Request State

- **D-05:** Replace the current bare `requested_txids` and `requested_wtxids` sets with richer pure transaction request state that records identity, announcing peer, requested peer, timestamps, expiry, and reason labels needed for duplicate suppression, `notfound`, timeout, fallback, and disconnect cleanup.
- **D-06:** Track already-have and recent-reject suppression as explicit scheduler inputs. The pure scheduler should receive local mempool/known/recent-reject facts as data and must not call mempool, storage, socket, or runtime APIs directly.
- **D-07:** Duplicate announcements for the same typed identity should be retained only when they can help fallback after timeout, `notfound`, or disconnect; they must not emit redundant `getdata` while an equivalent request is in flight.
- **D-08:** Disconnect cleanup must remove that peer's announcements and in-flight requests, then emit fallback actions when another eligible announcing peer exists.

### Download Scheduling

- **D-09:** Add a deterministic scheduler API that takes a fake-clock timestamp and emits typed request actions. Tests should be able to advance time without sleeping or touching wall-clock APIs.
- **D-10:** Keep in-flight request caps bounded and aligned with prior Phase 94 request-governance constraints. The scheduler should expose low-cardinality cap/suppression reasons instead of dynamic peer, txid, wtxid, or raw transaction labels.
- **D-11:** Model Knots-inspired scheduling delays explicitly: non-preferred peer delay, txid delay when wtxid peers exist, overloaded-peer delay, and getdata retry/expiry interval. The planner may choose exact constant names and simplified values when the tests preserve the observable behavior claimed by Phase 101.
- **D-12:** `notfound` for a requested transaction should complete or clear the matching in-flight request immediately and make the identity eligible for fallback to another announcing peer when one exists.
- **D-13:** Timeout should expire stale in-flight requests, clear the requested peer state, and choose a fallback peer if available without leaving stale request state behind.

### Received Transaction Cleanup

- **D-14:** On `tx`, derive both txid and wtxid once, mark both identities already-have, and clear any matching in-flight txid or wtxid request state for that peer.
- **D-15:** If a received transaction does not match the requested identity for that peer, emit a typed mismatch/suppression result and clean up only the state that is safe to clear. Do not treat mismatched data as satisfying an unrelated request.
- **D-16:** Phase 101 may continue returning a received transaction action for the managed network to submit later, but mempool admission semantics remain Phase 102. The new boundary should make the later admission bridge consume a stable typed transaction response instead of inspecting peer internals.

### Typed Actions And Evidence

- **D-17:** Emit typed pure actions for `request_getdata`, `suppress_duplicate`, `suppress_already_have`, `suppress_recent_reject`, `suppress_identity_mismatch`, `fallback_request`, `request_expired`, `notfound_cleanup`, `received_tx_cleanup`, and `peer_cleanup`.
- **D-18:** Adapter code may translate request actions into `WireNetworkMessage::GetData`, but socket I/O and managed runtime mutation must stay outside the scheduler.
- **D-19:** Evidence labels must be fixed and low-cardinality. Do not expose raw transaction hex, txids, wtxids, peer endpoints, permission strings, class names, credentials, or dynamic labels in planning, status, support, or log surfaces.

### Tests, Parity, And Guardrails

- **D-20:** Unit tests must cover txid and wtxid paths separately, duplicate announcements, identity mismatches, already-have suppression, recent-reject suppression, in-flight cap suppression, timeout fallback, `notfound` fallback, disconnect cleanup, and received-transaction cleanup.
- **D-21:** Use deterministic fake-clock tests for expiry and fallback. Do not add public-network relay checks, sleeps, or service-manager behavior to `bash scripts/verify.sh`.
- **D-22:** Add parity breadcrumbs for new first-party Rust source or test files under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests`, citing concrete Knots anchors unless `none` is the only defensible breadcrumb.
- **D-23:** If docs or parity roots are updated, preserve the v2.0 no-claim boundary: transaction relay remains bounded and explicit, while compact block relay, package relay, bloom/filter serving, public relay defaults, public-network CI, production full-node readiness, and production-funds wallet use stay deferred.

### the agent's Discretion

The planner may choose the exact module split, type names, scheduler constants, and whether to keep the scheduler under `peer/inventory_state.rs` or extract a sibling transaction-relay module, as long as the result stays pure, testable, bounded, parity-auditable, and compatible with existing `PeerManager` and `ManagedPeerNetwork` integration points.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Project And v2.0 Scope

- `.planning/PROJECT.md` - Open Bitcoin parity, architecture, dependency, verification, and v2.0 transaction relay boundaries.
- `.planning/REQUIREMENTS.md` - INV-01 through INV-04 and DL-01 through DL-02 are owned by Phase 101; DL-03+ and MEM-* are later phases.
- `.planning/ROADMAP.md` - Phase 101 purpose, scope, success criteria, dependencies, and verification contract.
- `.planning/STATE.md` - Current milestone state, recent Phase 100 completion, local verification caveats, and repo-local UAT command reminders.
- `.planning/phases/100-relay-activation-boundary-and-permission-semantics/100-CONTEXT.md` - Locked relay activation, eligibility, permission-effect, low-cardinality evidence, and no-claim decisions that Phase 101 builds on.

### Open Bitcoin Code And Tests

- `packages/open-bitcoin-network/src/relay.rs` - Phase 100 relay activation and peer eligibility policy.
- `packages/open-bitcoin-network/src/peer.rs` - `PeerManager`, `PeerState`, transaction announcement, message dispatch, `wtxidrelay`, and existing requested transaction sets.
- `packages/open-bitcoin-network/src/peer/inventory_state.rs` - Current `inv`, `getdata`, `notfound`, `tx`, and requested-inventory handling that Phase 101 should deepen.
- `packages/open-bitcoin-network/src/peer/tests.rs` - Current peer tests for txid/wtxid requests, `notfound`, and received transaction cleanup.
- `packages/open-bitcoin-network/src/message.rs` - Wire message enum, `inv`/`getdata`/`notfound`/`tx` payload encoding/decoding, and `MAX_INV_SIZE` handling.
- `packages/open-bitcoin-primitives/src/network.rs` - `InventoryType`, `InventoryVector`, and protocol type tags for txid/wtxid inventory.
- `packages/open-bitcoin-codec/src/network.rs` - Inventory vector parse/encode boundary.
- `packages/open-bitcoin-network/src/resource.rs` - Phase 94 request-governance caps and low-cardinality pressure labels.
- `packages/open-bitcoin-node/src/network/inventory.rs` - Managed serving/storage helper for transaction inventory requests.
- `packages/open-bitcoin-node/src/network.rs` - `ManagedPeerNetwork::process_actions` bridge from pure peer actions to managed mempool/storage behavior.
- `packages/open-bitcoin-node/src/network/tests.rs` - Existing managed network tests for wtxidrelay transaction request and in-memory relay behavior.
- `docs/parity/source-breadcrumbs.json` - Required registry for new or touched first-party Rust source/test files.

### Bitcoin Knots Anchors

- `packages/bitcoin-knots/src/node/txdownloadman.h` - Transaction download manager contract, peer connection info, request caps, delay constants, `AddTxAnnouncement`, `GetRequestsToSend`, `ReceivedNotFound`, `ReceivedTx`, and cleanup responsibilities.
- `packages/bitcoin-knots/src/node/txdownloadman_impl.cpp` - Already-have checks, txid/wtxid handling, request scheduling delays, in-flight expiry, `notfound`, accepted/rejected cleanup, and disconnect cleanup.
- `packages/bitcoin-knots/src/net_processing.cpp` - `wtxidrelay` negotiation, transaction `inv` mismatch handling, `getdata`, `tx`, `notfound`, and request flushing through P2P processing.
- `packages/bitcoin-knots/src/protocol.h` - Inventory type constants and protocol message identity.
- `packages/bitcoin-knots/test/functional/p2p_tx_download.py` - Transaction download behavior covering in-flight caps, expiry fallback, disconnect fallback, `notfound` fallback, txid delay, and wtxidrelay mismatch cases.
- `packages/bitcoin-knots/test/functional/p2p_getdata.py` - Invalid `getdata` behavior and continued message processing.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- `Txid`, `Wtxid`, `Hash32`, `InventoryType`, and `InventoryVector` already provide typed primitives and wire inventory tags.
- `WireNetworkMessage::Inv`, `GetData`, `NotFound`, and `Tx` already parse and encode transaction relay messages.
- `PeerState.remote_wtxidrelay` already records peer negotiation state.
- `PeerManager::announce_transaction` already selects `Transaction` versus `WitnessTransaction` based on `remote_wtxidrelay`.
- `PeerManager::handle_inventory`, `handle_transaction`, and `forget_requested_inventory` already contain the narrow seam for request tracking and cleanup.
- `ResourceGovernancePolicy` already bounds inbound request pressure and has existing tx request cap inputs.
- `ManagedPeerNetwork::serve_inventory` and `store_transaction` already map txid/wtxid to in-memory transactions for the current test harness.

### Established Patterns

- Pure network decisions live in `open-bitcoin-network`; managed runtime and mempool effects live in `open-bitcoin-node`.
- Request, timeout, queue, and resource-governance decisions should stay data-in/data-out and produce typed actions.
- Tests use Arrange, Act, Assert sections when non-trivial and prefer deterministic fake inputs over wall-clock or public-network behavior.
- Evidence and operator surfaces use fixed labels and avoid raw peer, permission, txid/wtxid, transaction hex, credential, and endpoint material.

### Integration Points

- Extend `PeerState` or a child transaction-relay state type to hold typed announcements and in-flight transaction requests.
- Keep `WireNetworkMessage` conversion at the edge: parse inventory vectors into typed relay IDs before scheduling and convert scheduler request actions back into `GetData` messages afterward.
- Keep `ManagedPeerNetwork::process_actions` as the shell bridge. It may receive richer pure actions later, but Phase 101 should avoid moving admission policy into the peer manager.
- Reuse existing resource-governance caps for adversarial inventory bursts and add narrower scheduler caps only where Phase 101 behavior needs them.

</code_context>

<specifics>
## Specific Ideas

- Favor a small `TxRelayId` enum plus a request scheduler over growing ad hoc txid/wtxid sets inside `PeerState`.
- Use explicit scheduler result labels instead of boolean return values so later phases can project metrics/logs without parsing strings.
- Treat `p2p_tx_download.py` as the behavioral checklist for deterministic local tests, especially `test_expiry_fallback`, `test_disconnect_fallback`, `test_notfound_fallback`, `test_txid_inv_delay`, and `test_inv_wtxidrelay_mismatch`.
- Preserve existing managed in-memory relay tests as compatibility coverage while moving scheduling semantics into pure network tests.

</specifics>

<deferred>
## Deferred Ideas

Orphan staging, parent request behavior, transaction admission outcome contracts, standardness/fee/RBF/ancestor policy, mempool pressure/trimming, mempool persistence, block connect/disconnect mempool lifecycle, relay serving/fanout, rebroadcast, RPC/operator/support evidence, release-boundary closeout, compact block relay, package relay, bloom/filter serving, public relay defaults, public-network relay CI, production full-node readiness, and production-funds wallet use remain outside Phase 101.

</deferred>

***

*Phase: 101-transaction-inventory-identity-and-download-scheduling*
*Context gathered: 2026-06-29*
