---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 115-2026-07-06T00-26-47
generated_at: 2026-07-06T00:26:47.054Z
---

# Phase 115: Missing Transaction Round Trip, Fallback, and Validation Handoff - Context

**Gathered:** 2026-07-06
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Phase 115 completes compact-block download by scheduling bounded `getblocktxn` requests for eligible peers and in-flight partial compact blocks, accepting only matching `blocktxn` responses, assembling full blocks from `PartialCompactBlock` state, handing complete blocks to the existing validation/connect path via `PeerAction::ReceivedBlock`, and falling back to full-block fetch or suppression on failure. Volatile compact-relay state must clear on restart, reconnect, disconnect, timeout, and reorg without mutating chainstate or durable block data.

This phase consumes Phase 114 reconstruction outcomes (`Ready { missing_indexes }`), Phase 112 wire codecs, and Phase 113 compact negotiation state. It must not add broad operator RPC/CLI/dashboard evidence (Phase 116), parity/UAT release closeout (Phase 117), package relay, bloom/filter serving, public defaults, or production readiness claims.
</domain>

<decisions>
## Implementation Decisions

### Missing Transaction Scheduling

- **D-01:** `getblocktxn` requests are sent only when local compact relay is activated, the peer is compact-capable, an in-flight partial compact block exists for the block hash, missing indexes remain, and no duplicate request is already in flight for that block.
- **D-02:** Request indexes use BIP152 differential encoding via existing `BlockTransactionsRequest.index_deltas`; absolute missing indexes from Phase 114 convert to deltas before encode.
- **D-03:** Request scheduling stays pure in `open-bitcoin-network` with typed outcomes; node shell adapters only send wire messages from returned actions.

### BlockTxn Response Handling

- **D-04:** `blocktxn` responses must match the expected in-flight block hash, arrive from the peer that owns the in-flight state, contain only still-missing indexes, reject duplicate or unexpected responses, and reject out-of-bounds transaction counts.
- **D-05:** Malformed, duplicate, unexpected, or out-of-bounds `blocktxn` responses produce stable low-cardinality misbehavior/suppression outcomes aligned with Knots resource-governance patterns (GOV-02).
- **D-06:** Successful `blocktxn` application uses a pure `apply_block_transactions` helper on `PartialCompactBlock` before fill/validation handoff.

### Validation Handoff And Fallback

- **D-07:** `fill_block` assembles a `Block` only when every transaction slot is available; partial state never mutates chainstate.
- **D-08:** Complete blocks emit `PeerAction::ReceivedBlock` through the same path as full `block` messages; reconstruction failure, timeout, old/far blocks, ineligible peers, malformed compact blocks, or invalid headers trigger typed fallback to full-block `getdata` or suppression.
- **D-09:** Full-block fallback reuses existing inventory/getdata request patterns where possible instead of inventing parallel download state.

### Volatile State Cleanup

- **D-10:** Per-peer in-flight compact download maps are volatile only; `on_block_connected`, disconnect, timeout, reorg, and explicit restart cleanup clear them without touching validated chainstate or durable storage.
- **D-11:** Cleanup causes use fixed low-cardinality labels suitable for later operator evidence (Phase 116).

### Scope Isolation

- **D-12:** No operator RPC/CLI/dashboard rollout, parity index closeout, package relay, bloom/filter serving, compact filters, public defaults, or production readiness claims in this phase.
- **D-13:** New first-party Rust source/test files require parity breadcrumbs and `docs/parity/source-breadcrumbs.json` updates unless explicit `none` is defensible.

### Claude's Discretion

The planner may choose exact module names (`compact_download` vs extending `compact_reconstruction`), whether in-flight maps live on `PeerState` or a dedicated peer submodule, and test fixture helpers. Prefer small pure APIs and deterministic tests.
</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Project And Phase Scope

- `AGENTS.md`, `AGENTS.bright-builds.md`, `standards/core/architecture.md`, `standards/core/code-shape.md`, `standards/core/testing.md`, `standards/core/verification.md`, `standards/languages/rust.md`
- `.planning/PROJECT.md`, `.planning/REQUIREMENTS.md` (RCN-04 through RCN-07, GOV-02, GOV-03), `.planning/ROADMAP.md`

### Prior Locked Decisions

- `.planning/phases/114-compact-block-reconstruction-from-mempool-state/114-CONTEXT.md`
- `.planning/phases/113-compact-relay-negotiation-and-announcement-policy/113-CONTEXT.md`

### Existing Code Integration Points

- `packages/open-bitcoin-network/src/compact_reconstruction.rs` — `PartialCompactBlock`, `init_partial_compact_block`
- `packages/open-bitcoin-codec/src/compact_block.rs` — `BlockTransactionsRequest`, `BlockTransactions`, differential index helpers
- `packages/open-bitcoin-network/src/peer/compact_relay.rs` — negotiation state
- `packages/open-bitcoin-network/src/peer.rs` — message dispatch and `PeerAction`
- `docs/parity/source-breadcrumbs.json`

### Bitcoin Knots Anchors

- `packages/bitcoin-knots/src/blockencodings.cpp` — `PartiallyDownloadedBlock::ProcessTxns`, `FillBlock`
- `packages/bitcoin-knots/src/net_processing.cpp` — compact block download, getblocktxn scheduling, fallback
- `packages/bitcoin-knots/test/functional/p2p_compactblocks.py`

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- Phase 114 `PartialCompactBlock` with `missing_transaction_indexes`, lifecycle hooks, and `Ready` outcomes.
- Phase 112 getblocktxn/blocktxn codecs and differential index expansion.
- Phase 113 compact relay capability and announcement policy.
- Existing `PeerAction::ReceivedBlock` and block-serving resource gates.

### Established Patterns

- Pure network modules with sibling `tests.rs`.
- Low-cardinality typed outcome enums.
- Peer manager message handlers delegating to pure policy modules.

### Integration Points

- `PeerManager::handle_message` currently no-ops `CompactBlock`, `GetBlockTxn`, and `BlockTxn`; Phase 115 wires these to compact download policy.
- Node shell consumes `PeerAction` variants for send/disconnect/received block effects.

</code_context>

<specifics>
## Specific Ideas

- Treat Phase 114 `Ready { missing_indexes }` as the sole trigger for first `getblocktxn` scheduling.
- Clear all in-flight compact state on block connect to prevent chainstate coupling.
- Use stable fallback labels such as `compact_reconstruction_failed`, `compact_download_timeout`, and `compact_peer_ineligible`.

</specifics>

<deferred>
## Deferred Ideas

Operator RPC/CLI/dashboard evidence, metrics/log label rollout, parity/UAT release guardrails, package relay, bloom/filter serving, compact filters, public serving defaults, and production readiness claims remain outside Phase 115.

</deferred>

---

*Phase: 115-missing-transaction-round-trip-fallback-and-validation-handoff*
*Context gathered: 2026-07-06*
