---
phase: 38
phase_name: "Block Download, Connect, and Restart Recovery"
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: "38-2026-05-02T01-10-10"
generated_at: "2026-05-02T01:10:10Z"
---

# Phase 38 Context: Block Download, Connect, and Restart Recovery

**Gathered:** 2026-05-02
**Status:** Ready for planning

<domain>
## Phase Boundary

Turn the Phase 37 header-first daemon sync path into a durable block pipeline. This phase owns bounded block request issuance, downloaded-block persistence, block-to-chainstate reconciliation after restart, and replay-safe chain advancement when the best header chain overtakes the current active chain. It does not own richer operator-facing sync controls or final public-mainnet smoke/docs closeout; those stay in Phases 39 and 40.

</domain>

<decisions>
## Implementation Decisions

### Block request and runtime shape
- **D-01:** Keep block download inside the existing `DurableSyncRuntime` plus `ManagedPeerNetwork` seams instead of introducing a second block-sync service.
- **D-02:** Centralize Phase 38 block scheduling in the runtime, not in the generic peer-manager header handler, so the same bounded request path can serve both fresh headers and restart recovery when headers are already durable.
- **D-03:** Add explicit per-peer and global block in-flight limits to the runtime surface and keep request issuance bounded even when the durable header chain is far ahead of chainstate.

### Restart and recovery
- **D-04:** Treat downloaded blocks as durable local evidence and reconcile them against the durable best header chain on open and after each new block, rather than assuming every stored block is already connected.
- **D-05:** Reconstruct block-connect work from durable headers plus stored blocks, so interrupted writes or restarts can reconnect already-downloaded blocks without replaying header work or re-downloading every block.
- **D-06:** Use typed recovery markers and actionable storage/runtime errors when durable state is inconsistent, and keep operator guidance on the existing restart/repair/reindex vocabulary.

### Reorg-like branch handling
- **D-07:** Use the existing chainstate undo-backed `reorg` path when the best durable header branch outranks the current active chain and the required replacement blocks are available locally.
- **D-08:** Keep branch selection driven by the durable header store so block connect follows the same deterministic best-tip choice that Phase 37 established.

### the agent's Discretion
- Whether the best-chain reconciliation helper lives under `sync.rs` or a small child module, as long as the runtime boundary stays clear.
- The smallest public helper surface needed from `HeaderStore` and `PeerManager` to avoid duplicating ancestry or request-tracking logic.
- Whether partial branch availability reconnects only the contiguous locally available prefix or waits for a full best-header branch, provided replay safety and deterministic advancement stay intact.

</decisions>

<specifics>
## Specific Ideas

- Add a header-store helper that exposes bounded best-chain traversal for runtime reconciliation and request planning.
- Let the runtime inspect durable stored blocks before issuing new `getdata`, so restart recovery prefers reconnecting already-downloaded blocks.
- Top up block requests after handshake, after headers import, and after block receipt so progress does not stall once the first request batch drains.
- Reuse `Chainstate::reorg` through `ManagedChainstate` instead of inventing a second branch-application mechanism.

</specifics>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Milestone scope
- `.planning/REQUIREMENTS.md` — Phase 38 requirements `SYNCMAIN-03`, `CHAINMAIN-02`, `CHAINMAIN-03`, `CHAINMAIN-04`, `CHAINMAIN-05`, `RESUME-01`, `RESUME-02`, `RESUME-03`, `VERMAIN-01`, and `VERMAIN-02`.
- `.planning/ROADMAP.md` — Phase 38 goal, success criteria, and boundaries relative to Phases 37, 39, and 40.
- `.planning/STATE.md` — Current milestone state and carry-forward constraints from completed Phases 35 through 37.

### Architecture and workflow rules
- `AGENTS.md` — Repo-local GSD, verification, parity breadcrumb, and README-update expectations.
- `AGENTS.bright-builds.md` — Bright Builds workflow defaults, sync-before-edit, and verification rules.
- `standards-overrides.md` — Local overrides status.
- Bright Builds `standards/index.md` — Canonical standards entrypoint.
- Bright Builds `standards/core/architecture.md` — Functional core / imperative shell expectations.
- Bright Builds `standards/core/code-shape.md` — Early-return and readability guidance.
- Bright Builds `standards/core/testing.md` — Deterministic behavior-focused test expectations.
- Bright Builds `standards/core/verification.md` — Repo-native verification requirements.
- Bright Builds `standards/languages/rust.md` — Rust-specific structure and quality rules.

### Existing code seams
- `packages/open-bitcoin-node/src/sync.rs` — Current durable runtime loop, per-message persistence, and peer/session control flow.
- `packages/open-bitcoin-node/src/network.rs` — Header-only sync message path and current eager block connect behavior for received blocks.
- `packages/open-bitcoin-node/src/chainstate.rs` — Managed `connect_block`, `disconnect_tip`, and `reorg` persistence boundary.
- `packages/open-bitcoin-node/src/storage/fjall_store.rs` — Durable block storage, runtime metadata, and recovery-marker APIs.
- `packages/open-bitcoin-node/src/storage.rs` — Operator-facing recovery-action vocabulary.
- `packages/open-bitcoin-network/src/header_store.rs` — Deterministic best-header-chain selection and ancestry helpers.
- `packages/open-bitcoin-network/src/peer.rs` — Peer handshake state, requested-block tracking, and inventory/message actions.
- `packages/open-bitcoin-node/src/sync/tests.rs` and `packages/open-bitcoin-network/src/peer/tests.rs` — Existing deterministic sync and request-cap regression coverage.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `FjallNodeStore::save_block` and `load_block` already persist canonical block payloads under block hash keys.
- `ManagedChainstate::reorg` already exposes an undo-backed replay-safe branch swap for connected blocks.
- `PeerManager` already tracks per-peer requested blocks and caps in-flight requests, which means Phase 38 can extend rather than replace the request bookkeeping.

### Established Patterns
- `DurableSyncRuntime` already persists durable progress after each inbound sync message and projects typed failures into peer outcomes, health signals, metrics, and structured logs.
- The runtime test harness already supports scripted header and block sessions, store reopen flows, and deterministic restart assertions.
- The repo already treats recovery actions as typed operator guidance instead of silent best-effort repair.

### Integration Points
- `ManagedPeerNetwork::handle_headers_message` currently forces `HeaderSyncPolicy::HeadersOnly`, so the daemon sync path never drains the durable header chain into block requests.
- `DurableSyncRuntime::open` seeds headers and chainstate but does not reconcile already-persisted blocks against the best header chain after restart.
- `ManagedPeerNetwork::process_actions` can connect a received block immediately, but it does not reconcile branch takeovers or recover stored blocks that were persisted before chainstate advanced.

</code_context>

<deferred>
## Deferred Ideas

- Rich operator-facing sync-phase status, controls, metrics wording, and RPC/dashboard truthfulness beyond the minimum truthful block/header counters remain Phase 39 work.
- Live public-mainnet smoke commands, operator runbooks, and parity closeout remain Phase 40 work.
- Full production-node hardening for memory/disk sizing, inbound serving, and long-term policy parity stays outside v1.2 scope.

</deferred>

---

*Phase: 38-block-download-connect-and-restart-recovery*
*Context gathered: 2026-05-02*
