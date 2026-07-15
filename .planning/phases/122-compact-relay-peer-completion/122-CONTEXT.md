---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 122-2026-07-15T15-22-57
generated_at: 2026-07-15T15:22:57.638Z
---

# Phase 122: Compact Relay Peer Completion - Context

**Gathered:** 2026-07-15
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Complete the peer-facing BIP152 request/response symmetry for compact blocks already announced by this node. A remote peer may request missing transactions only for a matching compact announcement that Open Bitcoin actually produced for that peer, and the node may respond only from currently eligible, validated, available local block data through bounded policy and resource gates. Broader public relay defaults, arbitrary recent-block `getblocktxn` serving, package relay, and production-scale claims remain out of scope.

</domain>

<decisions>
## Implementation Decisions

### Announcement Correlation

- **D-01:** Record compact-announcement provenance only after `ManagedPeerNetwork::announce_block` produces `Some(WireNetworkMessage::CompactBlock(_))`. A policy decision to announce is not proof that a compact payload was constructed.
- **D-02:** Keep a bounded per-peer FIFO or set of announced `BlockHash` tokens beside compact peer state. Do not overload `announcement_eligibility`, which is a policy result rather than proof about a particular block hash.
- **D-03:** Retain each block payload once in node-shell-owned shared storage. Do not duplicate full blocks per peer or move block payload ownership into the pure peer model.
- **D-04:** Use an explicit small bound aligned with the pinned Knots recent `getblocktxn` window; the planning default is 11 hashes per peer for the inclusive 10-deep window. The planner may choose an equivalent typed bound if codebase research shows a clearer existing constant.
- **D-05:** Remove announcement tokens on peer disconnect and runtime restart, and prune tokens when their blocks leave the active/eligible recent window. Cleanup must not delete validated chainstate or durable block data.

### Request Eligibility And Response Outcomes

- **D-06:** Replace the unconditional `GetBlockTxn` peer-dispatch no-op with a typed request decision/action. The pure network layer owns peer-scoped provenance and request-policy decisions; the node shell owns current chain/block lookup and wire effects.
- **D-07:** Before any block lookup, require matching peer-scoped announcement provenance and apply the existing request-pressure/resource-governance path. At serving time, re-evaluate compact/block activation, peer eligibility, validated active-chain status, data availability, and resource capacity instead of trusting announcement-time facts.
- **D-08:** Expand differential indexes exactly once with the existing BIP152 codec helper, preserve request order, and select transactions from the matching local block. A successful response is `WireNetworkMessage::BlockTxn` with the requested block hash and witness-preserving transactions.
- **D-09:** Silently suppress unannounced, unknown, unavailable, pruned, reorged, ineligible, or benign lookup-miss requests. Do not send `notfound` and do not expose whether unavailable block or transaction data exists.
- **D-10:** Keep existing resource semantics: request-cap violations may disconnect through resource governance, while ordinary queue/backpressure outcomes suppress. Any expanded index outside `block.transactions` is typed compact misbehavior and disconnects, matching pinned Knots behavior.
- **D-11:** Do not implement Knots' old-block full-witness-block fallback in this phase. HARD-01 is deliberately narrower: locally announced compact blocks only. Record the scoped difference in parity evidence rather than broadening block serving.

### Protocol Verification And Evidence

- **D-12:** Rename `phase112_bip152_wire_messages_are_peer_noops` to `phase112_bip152_baseline_dispatch_emits_no_unconditional_actions`, or an equally precise name, so the test continues to describe its default-disabled/stateless fixture without contradicting the later live shell path.
- **D-13:** Add focused pure tests for request resolution, transaction ordering and witness preservation, peer-scoped provenance, bounded announcement state, cleanup, suppression labels, and out-of-bounds misbehavior. Keep each unit test focused and structured as Arrange, Act, Assert.
- **D-14:** Add live `ManagedPeerNetwork` protocol tests proving: a real compact announcement followed by matching `getblocktxn` emits `blocktxn`; a request from another peer is silent; unavailable or ineligible requests are silent; out-of-bounds indexes disconnect without sensitive evidence; and disconnect/pruning cleanup removes authorization.
- **D-15:** Use stable low-cardinality internal outcomes that distinguish inbound serving from existing outbound missing-request evidence. Preferred labels are `compact_missing_tx_served`, `compact_missing_tx_serve_suppressed`, and `compact_missing_tx_malformed`, with fixed causes such as `compact_getblocktxn_not_announced`, `compact_getblocktxn_ineligible`, `compact_getblocktxn_unavailable`, `compact_getblocktxn_request_limited`, and `compact_getblocktxn_index_out_of_bounds`.
- **D-16:** Evidence and logs must not include peer IDs, endpoints, block hashes, transaction IDs or payloads, permission strings, credentials, or runtime-created labels. Transaction payloads appear only in the intended `blocktxn` wire response.
- **D-17:** Add a deterministic Phase 122 Bun checker and mutation tests, wire them into `bash scripts/verify.sh`, reject the exact unconditional no-op and stale test name, and require the production route, live tests, stable labels, HARD-01 parity evidence, and verifier integration.
- **D-18:** Update `docs/parity/index.json`, `docs/parity/catalog/p2p.md`, and `docs/parity/checklist.md` with a Phase 122 evidence root. Update `docs/parity/source-breadcrumbs.json` for every new first-party Rust source or test file, using an explicit `none` breadcrumb only when no defensible Knots source anchor exists.

### the agent's Discretion

The planner may choose exact Rust type and action names, whether the bounded token collection is a FIFO, deque, or insertion-ordered set, and the smallest module boundary that keeps the pure decision model separate from node-shell block lookup. Prefer reusing existing action translation, block-serving classification, request governance, codec, and parity-checker patterns over introducing parallel policy or storage systems.

</decisions>

<canonical-refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase Contract And Audit Gap

- `.planning/ROADMAP.md` § Phase 122 — fixed goal, dependency, HARD-01 ownership, and success criteria.
- `.planning/REQUIREMENTS.md` § HARD-01 — requires bounded serving of eligible inbound `getblocktxn` for locally announced compact blocks.
- `.planning/v2.1-MILESTONE-AUDIT.md` § Phase 112 / Phase 118 — identifies the inbound no-op and stale test vocabulary.
- `.planning/phases/110-block-serving-activation-and-eligibility-boundary/110-CONTEXT.md` — default-off activation, peer eligibility, status classification, resource, and evidence decisions.
- `.planning/phases/111-full-block-serving-request-path/111-CONTEXT.md` — storage-read gates, current block availability, cleanup, and bounded historical-serving decisions.
- `.planning/phases/112-bip152-wire-codec-and-message-semantics/112-CONTEXT.md` — BIP152 payload, differential-index, witness, malformed-input, and message-surface decisions.
- `.planning/phases/113-compact-relay-negotiation-and-announcement-policy/113-CONTEXT.md` — per-peer compact negotiation and announcement eligibility decisions.
- `.planning/phases/115-missing-transaction-round-trip-fallback-and-validation-handoff/115-CONTEXT.md` — missing-transaction response and compact state cleanup vocabulary.
- `.planning/phases/118-outbound-compact-block-announcement-wiring/118-CONTEXT.md` — truthful wire-emission boundary for compact announcements.
- `.planning/phases/120-compact-download-timeout-and-misbehavior-runtime-bridge/120-CONTEXT.md` — typed compact misbehavior, disconnect, and cleanup behavior.

### Open Bitcoin Integration Points

- `packages/open-bitcoin-network/src/peer/message_dispatch.rs` — current inbound `GetBlockTxn` no-op and peer dispatch seam.
- `packages/open-bitcoin-network/src/peer/compact_relay.rs` — typed per-peer compact negotiation and announcement state.
- `packages/open-bitcoin-network/src/peer.rs` — peer state, actions, and compact announcement message construction.
- `packages/open-bitcoin-codec/src/compact_block.rs` — `BlockTransactionsRequest`, `BlockTransactions`, differential index expansion, and witness-preserving codecs.
- `packages/open-bitcoin-node/src/network.rs` — authoritative `ManagedPeerNetwork` receive and successful compact-announcement seams.
- `packages/open-bitcoin-node/src/network/action_translation.rs` — existing pure-action to node-shell effect translation pattern.
- `packages/open-bitcoin-node/src/network/inventory.rs` — current block-serving context, storage lookup, and peer eligibility integration.
- `packages/open-bitcoin-node/src/network/block_serving.rs` — current activation, status, resource, and storage-read gate.
- `packages/open-bitcoin-node/src/network/block_relay_evidence.rs` — post-emission compact announcement evidence and sanitized labels.

### Pinned Bitcoin Knots Parity Anchors

- `packages/bitcoin-knots/src/net_processing.cpp` — `MAX_BLOCKTXN_DEPTH`, `SendBlockTransactions`, inbound `GETBLOCKTXN`, recent-block lookup, silent unavailable handling, old-block fallback, and out-of-bounds misbehavior.
- `packages/bitcoin-knots/src/blockencodings.h` — `BlockTransactionsRequest` and `BlockTransactions` response structure.
- `packages/bitcoin-knots/test/functional/p2p_compactblocks.py` § `test_getblocktxn_handler` — requested transaction/witness equality, recent-window behavior, old-block fallback, out-of-bounds disconnect, and silent unavailable-block behavior.

</canonical-refs>

<code-context>
## Existing Code Insights

### Reusable Assets

- `CompactRelayPeerState`: established typed per-peer compact policy state; a separate bounded announcement-token field can live beside it.
- `PeerAction` and node action translation: existing functional-core to imperative-shell bridge for serving work.
- `serve_managed_block_request` and `managed_block_serve_input`: current activation, eligibility, status, resource, and storage-read ordering.
- `expand_block_transaction_indexes`: existing differential-index expansion with typed overflow behavior.
- `blocks_by_hash`: current shared in-memory block payload source; avoids per-peer duplication.

### Established Patterns

- Record compact-announcement evidence only after a real compact wire message is produced.
- Keep pure peer and policy state free of block storage and socket effects.
- Use silent suppression for unavailable compact-block data to reduce fingerprinting.
- Escalate typed malformed compact behavior through the existing disconnect bridge.
- Add focused deterministic phase checkers and mutation tests to the repo-native verifier.

### Integration Points

- `PeerManager::handle_message` must translate inbound `GetBlockTxn` into a typed decision/action rather than an unconditional empty action list.
- `ManagedPeerNetwork::announce_block` must record the matching peer/hash token at the truthful post-construction seam.
- `ManagedPeerNetwork` receive/action translation must perform current-state policy gating, shared block lookup, transaction selection, and outbound `BlockTxn` emission.
- Peer removal and active-chain/reorg cleanup paths must prune announcement tokens without mutating validated block storage.

</code-context>

<specifics>
## Specific Ideas

- Use a bounded per-peer authorization token rather than a global recent-block cache as the HARD-01 hardening boundary.
- Keep the response path witness-preserving and request-order-preserving.
- Preserve Knots' silent handling for unavailable data and disconnect behavior for out-of-bounds indexes, while documenting the deliberate omission of old-block full-block fallback.

</specifics>

<deferred>
## Deferred Ideas

- Knots-style full-witness-block fallback for `getblocktxn` requests older than the recent-depth window belongs to a future block-serving parity phase.
- New public status, metrics, structured logs, CLI, dashboard, and support-bundle schemas for inbound `getblocktxn` outcomes are not required for HARD-01 and remain deferred.

</deferred>

***

*Phase: 122-compact-relay-peer-completion*
*Context gathered: 2026-07-15*
