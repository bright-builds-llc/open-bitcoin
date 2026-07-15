---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 123-2026-07-15T18-12-00
generated_at: 2026-07-15T18:12:00.855Z
---

# Phase 123: Runtime Timing and Evidence Integrity - Context

**Gathered:** 2026-07-15
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Close the three approved v2.1 runtime-integrity gaps without expanding the public relay boundary: compact-download timeouts must advance while a live peer is idle, served-block evidence must follow successful block wire writes, and sync metrics/logs must sample the exact network instance owned by `DurableSyncRuntime`.

</domain>

<decisions>
## Implementation Decisions

### Deterministic Idle Timeout Scheduling

- **D-01:** Add a caller-clocked maintenance pulse to the live sync session driver. A socket read-idle wake must be distinguishable from EOF/connection close so the peer session remains live while maintenance runs.
- **D-02:** On each idle wake, obtain an explicit `now_unix_seconds` from an injected caller clock, invoke the existing `ManagedPeerNetwork::expire_compact_download_timeouts(now_unix_seconds)` forwarder, and emit the returned same-peer or targeted fallback actions through the session that owns the connection before polling again.
- **D-03:** Preserve deterministic tests with scripted idle outcomes and a fake clock. Idle wakes must not count as received messages, create false progress, or discard peer-targeted `GetData(Block)` fallback.

### Successful Block-Emission Evidence

- **D-04:** Add a dedicated aggregate served-block counter to authoritative managed-network evidence. `BlockServedCount` must project this counter and must no longer derive from `eligible_peer_count` or any other eligibility, lookup, construction, or enqueue proxy.
- **D-05:** Advance the counter through a typed post-write acknowledgement only after a transport successfully writes an actual `WireNetworkMessage::Block`. Wire the acknowledgement at both current block-serving effect seams: `DurableSyncRuntime::send_all` and the inbound listener's successful `WriteWireMessageOutcome::Written` path.
- **D-06:** Preserve typed message identity until the acknowledgement boundary, including where the inbound path currently reduces outbound messages to encoded bytes. Failed encoding, failed writes, non-block messages, and pre-send intent never increment the counter. If a batch partially succeeds, acknowledge each successful block write exactly once before a later failure is returned.

### Authoritative Runtime Projection

- **D-07:** `DurableSyncRuntime` must sample `self.network.block_relay_evidence_status()` directly after runtime peer processing. The same tick-local, availability-gated snapshot feeds both metric persistence and structured-log emission.
- **D-08:** Remove the block-relay provider field/setter and the daemon closure over `ManagedRpcContext`; that context owns a different `ManagedPeerNetwork` and is not authoritative for sync-runtime compact activity.
- **D-09:** Preserve Phase 121 omission semantics: unobserved or unavailable block-relay evidence produces no block-relay metric samples or log record, not manufactured zero-valued evidence.

### Verification, Parity, and Scope

- **D-10:** Focused runtime tests must prove idle expiry without receives, fallback emission through the retained session, post-write-only served counting across success/failure/partial-batch cases, and metrics/log projection from compact activity performed by the sync runtime's own network.
- **D-11:** Add a deterministic Phase 123 Bun checker with mutation coverage, wire it into `bash scripts/verify.sh`, and keep default verification local and public-network-free.
- **D-12:** New or touched first-party Rust source/test files require parity breadcrumbs and `docs/parity/source-breadcrumbs.json` entries. Update the v2.1 parity evidence root for HARD-02 through HARD-04 without broadening block-serving, compact-relay, public-default, or production-readiness claims.

### Agent's Discretion

The planner may choose exact receive-outcome, clock, acknowledgement, and snapshot helper names; whether the maintenance pulse is represented as a small session-driver helper or a focused runtime method; and the smallest typed carrier that preserves outbound message identity in the inbound listener. Prefer existing blocking runtime seams, no new async/timer dependency, one authoritative counter, early returns, and focused modules over a transport-wide redesign.

</decisions>

<canonical-refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase Contract and Audit Gaps

- `.planning/ROADMAP.md` § Phase 123 — fixed goal, dependency, HARD-02 through HARD-04 ownership, and success criteria.
- `.planning/REQUIREMENTS.md` § Milestone Hardening and Closeout — normative HARD-02, HARD-03, and HARD-04 requirements.
- `.planning/v2.1-MILESTONE-AUDIT.md` § Phases 116, 120, and 121 — the three concrete runtime-integrity debt findings this phase closes.
- `.planning/phases/118-outbound-compact-block-announcement-wiring/118-CONTEXT.md` — achieved-effect evidence precedent for compact announcements.
- `.planning/phases/120-compact-download-timeout-and-misbehavior-runtime-bridge/120-CONTEXT.md` — existing caller-clocked timeout forwarder, fallback, and cleanup decisions.
- `.planning/phases/121-block-relay-metrics-and-log-runtime-projection/121-CONTEXT.md` — availability gating, helper reuse, persistence, log, and leakage decisions.
- `.planning/phases/122-compact-relay-peer-completion/122-CONTEXT.md` — current compact serving provenance, request handling, evidence, and parity boundary.

### Open Bitcoin Runtime Seams

- `packages/open-bitcoin-node/src/network.rs` — receive-driven timeout calls, authoritative `ManagedPeerNetwork`, block serving, and outbound message construction.
- `packages/open-bitcoin-node/src/network/block_relay_evidence.rs` — aggregate managed-network evidence state and projection.
- `packages/open-bitcoin-node/src/network/tests/compact_timeout_cases.rs` — current timeout forwarder and receive-driven regression coverage.
- `packages/open-bitcoin-node/src/metrics/block_relay.rs` — current incorrect `BlockServedCount` proxy mapping.
- `packages/open-bitcoin-node/src/sync.rs` — `DurableSyncRuntime` ownership of the live network and sync session loop.
- `packages/open-bitcoin-node/src/sync/session.rs` — successful session-send effect boundary.
- `packages/open-bitcoin-node/src/sync/metrics.rs` — provider-based block-relay metric persistence seam.
- `packages/open-bitcoin-node/src/sync/tcp.rs` — blocking receive behavior and current idle/EOF conflation.
- `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs` — daemon cycle cadence and current non-authoritative block-relay provider closure.
- `packages/open-bitcoin-rpc/src/inbound_listener.rs` — inbound wire write-success acknowledgement seam.

### Pinned Bitcoin Knots and Evidence Guardrails

- `packages/bitcoin-knots/src/net_processing.cpp` — receive-independent peer send/maintenance cadence, block serving, and compact-download behavior.
- `packages/bitcoin-knots/src/net.cpp` — peer connection and wire-send lifecycle anchors.
- `packages/bitcoin-knots/test/functional/p2p_compactblocks.py` — compact relay timeout/fallback and peer behavior reference coverage.
- `docs/parity/index.json` — machine-readable v2.1 parity evidence root.
- `docs/parity/source-breadcrumbs.json` — source-level parity breadcrumb ownership.
- `scripts/verify.sh` — required deterministic repository verification contract.

</canonical-refs>

<code-context>
## Existing Code Insights

### Reusable Assets

- `ManagedPeerNetwork::expire_compact_download_timeouts`: already returns translated peer actions and records timeout cleanup/fallback evidence from an explicit timestamp.
- `ManagedPeerNetwork::block_relay_evidence_status`: already derives the shared aggregate contract from the authoritative network's evidence state and live `PeerManager`.
- `block_relay_metric_samples` and `block_relay_log_record`: existing fixed low-cardinality projection helpers remain the metric/log mapping boundary.
- `SyncPeerSession::send` and `WriteWireMessageOutcome::Written`: existing effect-completion seams where a block write can be acknowledged truthfully.

### Established Patterns

- Phase 118 records compact-announcement evidence only after a typed outbound message exists; Phase 123 moves served-block evidence one step further to actual write success because HARD-03 requires successful emission.
- Runtime clocks are explicit integer timestamps in core/network APIs, while effectful adapters own wall-clock acquisition.
- Unavailable evidence is omitted from persistence rather than converted into misleading zero samples.

### Integration Points

- `DurableSyncRuntime::sync_connected_peer` owns the live session and must retain it across explicit idle wakes so maintenance actions can still use that connection.
- `DurableSyncRuntime::persist_metrics` and the structured-log path should consume one direct snapshot from `self.network` per tick.
- The daemon should stop bridging block-relay evidence from `ManagedRpcContext`, while inbound status provider behavior remains unchanged.

</code-context>

<specifics>
## Specific Ideas

- Treat evidence as proof of achieved effects: decision is not construction, construction is not enqueue, and enqueue is not a successful wire write.
- Use one authoritative tick-local snapshot for both metrics and logs so the two persisted surfaces cannot disagree about runtime provenance.

</specifics>

<deferred>
## Deferred Ideas

- A Tokio-based async receive/timer redesign is deferred; Phase 123 should close HARD-02 through the existing blocking session driver.
- Sharing the mutable live `ManagedPeerNetwork` across RPC and sync, or publishing a broader cross-surface evidence snapshot, is deferred unless a later phase explicitly unifies all operator status provenance.
- A generalized receipt-bearing wire-emitter abstraction is deferred; use the smallest typed acknowledgement seam needed for truthful served-block evidence.

</deferred>

***

*Phase: 123-runtime-timing-and-evidence-integrity*
*Context gathered: 2026-07-15*
