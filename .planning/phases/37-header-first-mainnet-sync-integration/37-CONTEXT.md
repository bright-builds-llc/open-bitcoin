---
phase: 37
phase_name: "Header-First Mainnet Sync Integration"
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: "37-2026-05-02T00-08-13"
generated_at: "2026-05-02T00:08:13.525Z"
---

# Phase 37 Context: Header-First Mainnet Sync Integration

**Gathered:** 2026-05-01
**Status:** Ready for planning

<domain>
## Phase Boundary

Advance the daemon-owned sync runtime from the Phase 36 outbound peer layer into a durable, restart-aware, header-first pipeline. This phase owns validated header continuation, active-header-chain selection, restart reuse of persisted headers, and truthful header-versus-block progress reporting. It does not claim block download/connect, block undo/reorg application, or final operator-surface presentation polish that later phases own.

</domain>

<decisions>
## Implementation Decisions

### Header continuation and runtime shape
- **D-01:** Keep header synchronization inside the existing daemon sync runtime and `ManagedPeerNetwork` boundary instead of introducing a second standalone headers service.
- **D-02:** Treat Phase 37 as strictly header-first: after importing headers, continue `getheaders` batching until the peer stops advancing the best known header chain, and do not request blocks in this phase.
- **D-03:** Reuse the existing durable header snapshot as the restart anchor so reopening the node can continue from the best persisted header locator instead of replaying from genesis.

### Validation and active-chain selection
- **D-04:** Validate inbound headers with both stateless proof-of-work checks and contextual consensus rules before they enter the durable header store.
- **D-05:** Build header-validation context from the persisted header tree itself so future batches validate against the same active-header-chain view that restart recovery reloads.
- **D-06:** Keep competing-branch selection deterministic inside the header store and return typed invalid-data failures when a peer sends headers that fail contextual validation or ancestry requirements.

### Progress and observability
- **D-07:** Keep header progress and block progress distinct in the sync summary and status projection so Phase 37 can truthfully report advanced headers while block height still lags behind.
- **D-08:** Surface invalid-header and disconnect outcomes through the existing typed peer outcome and health-signal pipeline rather than silent peer drops.

### the agent's Discretion
- Whether header continuation lives behind an explicit peer-manager policy enum or a narrower runtime flag, as long as the boundary stays testable.
- The smallest reusable helper surface needed for median-time-past and retarget context lookup from the header store.
- Whether the sync summary needs a dedicated header-sync completion hint in this phase, provided block progress remains truthful and restart reuse is auditable.

</decisions>

<specifics>
## Specific Ideas

- Split header import from block inventory requests so later Phase 38 work can re-enable block download deliberately instead of inheriting it accidentally from Phase 36.
- Prefer typed helpers around header-store ancestry and contextual validation over re-parsing raw locators or recomputing parent state ad hoc in the runtime loop.
- Keep tests hermetic with scripted peers that exercise multi-batch headers, restart reuse, invalid headers, and competing branches without public-network access.

</specifics>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Milestone scope
- `.planning/REQUIREMENTS.md` — Phase 37 requirements `SYNCMAIN-03`, `CHAINMAIN-01`, `CHAINMAIN-03`, `RESUME-01`, `RESUME-02`, `VERMAIN-01`, and `VERMAIN-02`.
- `.planning/ROADMAP.md` — Phase 37 goal, success criteria, and boundaries relative to Phases 36 and 38-40.
- `.planning/STATE.md` — Current milestone state and carry-forward constraints from completed Phases 35 and 36.

### Architecture and workflow rules
- `AGENTS.md` — Repo-local GSD, verification, parity breadcrumb, and doc-update expectations.
- `AGENTS.bright-builds.md` — Bright Builds workflow defaults, sync-before-edit, and verification rules.
- `standards-overrides.md` — Local overrides status.
- Bright Builds `standards/index.md` — Canonical standards entrypoint.
- Bright Builds `standards/core/architecture.md` — Functional core / imperative shell expectations.
- Bright Builds `standards/core/code-shape.md` — Early-return and readability guidance.
- Bright Builds `standards/core/testing.md` — Deterministic behavior-focused test expectations.
- Bright Builds `standards/core/verification.md` — Repo-native verification requirements.
- Bright Builds `standards/languages/rust.md` — Rust-specific structure and quality rules.

### Existing code seams
- `packages/open-bitcoin-node/src/sync.rs` — Current durable sync runtime loop, persistence points, and per-peer processing.
- `packages/open-bitcoin-node/src/network.rs` — Adapter-owned boundary that applies peer-manager actions to chainstate and mempool state.
- `packages/open-bitcoin-node/src/sync/types.rs` — Sync summary, peer outcome, and health-signal types that must stay truthful in Phase 37.
- `packages/open-bitcoin-network/src/peer.rs` — Current handshake, `getheaders`, header import, and inventory-request behavior.
- `packages/open-bitcoin-network/src/header_store.rs` — Durable header tree, best-tip selection, and locator building.
- `packages/open-bitcoin-consensus/src/block.rs` and `packages/open-bitcoin-consensus/src/context.rs` — Contextual header validation rules and `BlockValidationContext`.
- `packages/open-bitcoin-node/src/storage/fjall_store.rs` — Header snapshot persistence and restart reload paths.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `DurableSyncRuntime` already seeds the in-memory header store from durable storage and persists header entries after each sync loop.
- `PeerManager` already drives handshake state, locator requests, and header insertion, so Phase 37 can extend that path rather than replacing the peer layer.
- `HeaderStore` already records parent links, best-tip selection, and locator construction, which makes it the right place to centralize ancestry lookups for contextual validation.

### Established Patterns
- Sync runtime shell code already projects typed runtime failures into peer outcomes, health signals, metrics, and structured logs.
- Tests already use `ScriptedTransport` and store reopen flows for deterministic sync behavior.
- Header and chainstate snapshots are already persisted separately, which naturally supports reporting header progress ahead of block progress.

### Integration Points
- `PeerManager::handle_headers` currently imports only one header batch and immediately requests blocks, which is the main Phase 37 gap.
- `HeaderStore` currently accepts headers after only stateless checks; contextual checks and durable branch selection need to sit on that seam.
- `SyncRunSummary::sync_status` already reports header and block heights separately, so Phase 37 can tighten truthfulness without inventing a brand-new status model.

</code_context>

<deferred>
## Deferred Ideas

- Block download, bounded in-flight block requests, connect/restart recovery for partial blocks, and reorg-safe block-state transitions remain Phase 38 work.
- Rich operator-facing status, dashboard, metrics, logs, and RPC wording for sync phases remain Phase 39 work beyond the minimum truthful summary changes this phase needs.
- Live public-mainnet smoke commands and milestone closeout docs remain Phase 40 work.

</deferred>

---

*Phase: 37-header-first-mainnet-sync-integration*
*Context gathered: 2026-05-01*
