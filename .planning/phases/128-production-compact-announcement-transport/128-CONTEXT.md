---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 128-2026-07-20T01-54-33
generated_at: 2026-07-20T02:02:55.970Z
---

# Phase 128: Production Compact Announcement Transport - Context

**Gathered:** 2026-07-19
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Phase 128 closes GAP-02, GAP-03, FLOW-02, and FLOW-03 by completing bilateral BIP152 negotiation and carrying newly validated blocks from the authoritative production runtime through compact/header/inventory policy, real peer transport writes, and achieved-effect evidence.

The phase owns local version-2 `sendcmpct` emission, retained remote negotiation state, a production validated-block announcement trigger, live per-peer header/resource facts, bounded peer-targeted transport, and post-write-only compact announcement provenance, metrics, and logs. It does not add adaptive high-bandwidth inbound peer selection, package relay, bloom/filter or compact-filter serving, public relay defaults, public-network CI, archive-node claims, production full-node readiness, or production-funds wallet use.

</domain>

<decisions>
## Implementation Decisions

### Bilateral Compact Negotiation

- **D-01:** After a production peer reaches established handshake state, emit a local BIP152 version-2 `SendCompactMessage { announce: false, version: 2 }` offer when compact relay is explicitly active and the remote protocol version supports the message.
- **D-02:** Keep the local offer state directionally distinct from the existing remote-derived `CompactRelayPeerState`. A locally queued or written low-bandwidth offer must not imply that the remote peer requested high-bandwidth announcements.
- **D-03:** Continue to treat the remote peer's version-2 `sendcmpct` message as the authoritative input for outbound compact-announcement eligibility: `announce: true` permits the high-bandwidth compact path when all other policy gates pass, while `announce: false` retains low-bandwidth capability without granting unsolicited compact announcements.
- **D-04:** Combine the post-Verack `sendcmpct` offer with existing handshake actions in a deterministic order and make duplicate Verack handling idempotent. Negotiation remains independent from transaction relay, package relay, bloom/filter permissions, compact filters, and public serving defaults.

### Validated-Block Announcement Trigger

- **D-05:** Trigger production announcement planning from a typed post-durable tip-advance event. Emit it only after a newly connected best-chain block has validated and its durable block body is available; do not announce historical IBD connects, side-branch storage, failed persistence, or mere receipt intent.
- **D-06:** For reconciliation that activates multiple blocks, announce only the final newly active durable tip unless pinned Knots behavior or existing runtime invariants require a bounded ordered set. Never emit from a pre-persistence connect decision.
- **D-07:** Under a short `NetworkRuntimeAuthority` mutation/snapshot boundary, collect the active peer IDs plus each peer's remote compact negotiation state, best-known/best-header-sent ancestry, eligibility, and queue/resource pressure. Do not substitute global header presence, constant header booleans, or a second network instance for live per-peer facts.
- **D-08:** Load or retain the authoritative validated block outside the network lock, prepare compact/header/inventory decisions outside socket effects, and enqueue peer-targeted emissions into bounded per-session outboxes. No authority lock may cross a socket read/write, `.await`, Fjall access, encoding, metric/log persistence, or RPC serialization.
- **D-09:** Preserve existing typed Phase 113/118 fallback and suppression policy. Queue pressure, disconnect, unavailable block data, payload construction failure, or lost eligibility must fail closed to the established header/inventory/suppress vocabulary without activating deferred relay surfaces.

### Transport And Achieved-Effect Evidence

- **D-10:** Represent an outbound announcement as an owned peer-targeted carrier such as `PeerEmission`, preserving `PeerId`, typed `WireNetworkMessage`, block identity, and evidence intent through transport without exposing raw peer or block identifiers to metrics or logs.
- **D-11:** Transport writes the emission outside the authority lock and returns or consumes a non-replayable success receipt. Only a successful `CompactBlock`, `Headers`, or `Inv` write may commit the matching announcement outcome and peer/header provenance.
- **D-12:** Failed encoding, failed writes, disconnected peers, suppressed decisions, and unsent batch suffixes receive no achieved-effect credit. In a partially successful batch, acknowledge each successful prefix emission exactly once before returning the later failure.
- **D-13:** `compact_announced_count` and compact-announcement provenance advance only from a successfully written `CompactBlock`. Successfully written header/inventory fallbacks advance only their corresponding fixed outcomes; suppression remains a decision outcome because it has no wire effect.
- **D-14:** Feed existing `BlockRelayEvidenceStatus`, fixed `MetricKind` series, and structured-log helpers from the authoritative post-write snapshot. Preserve availability gating and low cardinality; raw peer IDs, endpoints, block hashes, permission strings, credentials, and transaction payloads remain internal or redacted.

### Verification And Scope Guardrails

- **D-15:** Add focused production-path tests for local post-Verack `sendcmpct`, remote high/low preference retention, duplicate handshake handling, post-durable tip triggering, live per-peer header facts, compact/header/inventory writes, backpressure/disconnect failure, partial-batch success, and exactly-once post-write evidence.
- **D-16:** Add a deterministic Bun/TypeScript Phase 128 checker with mutation coverage that rejects missing local `sendcmpct`, constant header facts, absent production announcement callers, pre-write compact evidence, locks held across transport/storage effects, and public/package/filter/production claim expansion.
- **D-17:** New or touched first-party Rust source/test files require parity breadcrumbs and `docs/parity/source-breadcrumbs.json` entries. Default completion verification remains `bash scripts/verify.sh`, local and public-network-free.

### Folded Todos

No pending todos matched Phase 128.

### the agent's Discretion

The planner may choose exact `PeerEmission`, receipt, outbox, wakeup, and tip-event names; whether the bounded outbox is a small existing-session queue or a focused node-owned registry; and the minimum typed state required to make local offer queued/written status explicit. Prefer the smallest design that makes duplicate evidence and cross-effect lock holding difficult to represent. Do not introduce a general network actor or transport-wide receipt framework in this phase.

</decisions>

<canonical-refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Project Rules And Phase Contract

- `AGENTS.md` — repo-local GSD, Rust, parity breadcrumb, verification, generated-artifact, submodule, and command-timing rules.
- `AGENTS.bright-builds.md` — managed Bright Builds workflow and cross-cutting standards.
- `standards-overrides.md` — local exceptions; no substantive active override applies.
- `standards/core/architecture.md` — functional-core/imperative-shell and illegal-state guidance.
- `standards/core/code-shape.md` — early-return, optional-name, and function/file shape guidance.
- `standards/core/testing.md` — focused Arrange/Act/Assert test requirements.
- `standards/core/verification.md` — sync-first and repo-native verification gates.
- `standards/languages/rust.md` — Rust module, invariant, optional-name, adapter, and verification guidance.
- `.planning/ROADMAP.md` § Phase 128 — fixed goal, gap closure, requirements, and success criteria.
- `.planning/REQUIREMENTS.md` — normative `CMP-04`, `CMP-05`, and `OBS-03` requirements.
- `.planning/PROJECT.md` — bounded v2.1 claim and deferred public/production surfaces.
- `.planning/STATE.md` — active milestone route and lifecycle state.
- `.planning/v2.1-MILESTONE-AUDIT.md` — canonical GAP-02, GAP-03, FLOW-02, and FLOW-03 evidence.

### Prior Locked Decisions

- `.planning/phases/113-compact-relay-negotiation-and-announcement-policy/113-CONTEXT.md` — directional peer state, announcement eligibility, typed fallback, and scope isolation.
- `.planning/phases/118-outbound-compact-block-announcement-wiring/118-CONTEXT.md` — action-honor path, compact payload construction, and achieved-emission evidence.
- `.planning/phases/121-block-relay-metrics-and-log-runtime-projection/121-CONTEXT.md` — fixed low-cardinality metric/log projection and availability semantics.
- `.planning/phases/123-runtime-timing-and-evidence-integrity/123-CONTEXT.md` — post-write evidence, successful-prefix acknowledgement, and authoritative runtime sampling.
- `.planning/phases/126-compact-relay-residual-hardening/126-CONTEXT.md` — call-scoped nonce acquisition, explicit receive facts, and fail-closed closeout posture.
- `.planning/phases/127-authoritative-network-state-unification/127-CONTEXT.md` — one production authority, durable block source, short critical sections, and unchanged operator contracts.

### Production Runtime And Transport Seams

- `packages/open-bitcoin-network/src/peer.rs` — handshake actions, peer state, and announcement emission.
- `packages/open-bitcoin-network/src/peer/message_dispatch.rs` — Verack and incoming `sendcmpct` dispatch.
- `packages/open-bitcoin-network/src/peer/compact_relay.rs` — typed negotiation and announcement policy.
- `packages/open-bitcoin-node/src/network.rs` — managed announcement decisions, nonce acquisition, and wire-message construction.
- `packages/open-bitcoin-node/src/network/runtime_authority.rs` — Phase 127 shared production authority and lock boundary.
- `packages/open-bitcoin-node/src/network/block_relay_evidence.rs` — aggregate announcement evidence and current write acknowledgement.
- `packages/open-bitcoin-node/src/sync.rs` — durable sync ownership and validated-block runtime.
- `packages/open-bitcoin-node/src/sync/block_response.rs` — durable block validation/persistence disposition.
- `packages/open-bitcoin-node/src/sync/block_reconcile.rs` — active-chain reconciliation and durable tip changes.
- `packages/open-bitcoin-node/src/sync/session.rs` — outbound session send loop and successful-prefix behavior.
- `packages/open-bitcoin-rpc/src/inbound_listener.rs` — inbound peer session and successful wire-write boundary.
- `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs` — production composition and shared runtime construction.
- `docs/parity/index.json` — v2.1 parity evidence root.
- `docs/parity/source-breadcrumbs.json` — source-level parity breadcrumb registry.
- `scripts/verify.sh` — required deterministic repository verification contract.

### Pinned Bitcoin Knots Anchors

- `packages/bitcoin-knots/src/net_processing.cpp` — post-Verack version-2 `sendcmpct(false)`, peer header knowledge, high-bandwidth compact announcements, headers/inventory fallback, and compact send policy.
- `packages/bitcoin-knots/src/net.cpp` — peer connection, send queue, transport lifecycle, and successful effect boundaries.
- `packages/bitcoin-knots/src/validationinterface.cpp` — validated tip/block notification boundary.
- `packages/bitcoin-knots/src/blockencodings.cpp` — compact block construction and nonce-consuming short IDs.
- `packages/bitcoin-knots/test/functional/p2p_compactblocks.py` — bilateral negotiation and compact announcement behavior.

</canonical-refs>

<code-context>
## Existing Code Insights

### Reusable Assets

- `SendCompactMessage` and `BIP152_COMPACT_BLOCKS_VERSION` already define the local offer payload.
- `CompactRelayPeerState`, `CompactAnnouncementDecision`, and `announce_block_with_action` already provide remote-derived policy and typed compact/header/inventory construction.
- `NetworkRuntimeAuthority` already supplies the single shared production network and short mutation/snapshot APIs.
- `compact_announce_evidence_reason`, `BlockRelayEvidenceStatus`, metric samples, and structured-log helpers already define stable aggregate vocabulary.
- `DurableSyncRuntime::send_all` already demonstrates successful-prefix acknowledgement for full-block writes.

### Established Patterns

- Peer policy remains pure in `open-bitcoin-network`; storage, locks, session queues, socket writes, metrics, and logs remain node/RPC shell effects.
- Durable block serving and operator evidence use the Phase 127 authoritative handle rather than an independently constructed network.
- Full-block served evidence and compact-announcement evidence must describe achieved writes, not eligibility, construction, enqueue, or return intent.
- Repo-owned substantial guard automation is Bun/TypeScript and default verification is deterministic and public-network-free.

### Integration Points

- Extend the post-Verack action list with the local low-bandwidth version-2 `sendcmpct` offer and retain remote negotiation state through existing incoming dispatch.
- Emit a post-durable best-tip event from block response/reconciliation into a bounded announcement planner on the shared authority.
- Route resulting peer emissions to the session that owns each peer without holding the authority lock.
- Consume successful receipts back through the shared authority to update peer/header provenance and authoritative aggregate evidence exactly once.

</code-context>

<specifics>
## Specific Ideas

- Match the pinned Knots directional posture: local `sendcmpct(false)` advertises ability to receive compact blocks; the remote peer's `sendcmpct(true)` controls whether Open Bitcoin sends unsolicited compact announcements to that peer.
- Treat runtime identity and evidence identity as construction invariants: one production authority, owned peer-targeted emission, consuming receipt, no replayable generic acknowledgement.
- Prefer bounded per-session outboxes over a new actor protocol, and treat outbox pressure as an existing typed fallback/suppression input.
- Keep peer and block provenance internal while exposing only fixed aggregate outcomes through status, metrics, logs, and support evidence.

</specifics>

<deferred>
## Deferred Ideas

- Adaptive selection, promotion, and demotion of a bounded set of high-bandwidth inbound compact peers.
- A dedicated single-owner network coordinator/actor and its command, cancellation, shutdown, and retry protocol.
- A generalized receipt-bearing outbound emitter for every P2P wire message.
- Package relay, bloom/filter serving, compact filters, public relay defaults, public-network CI, archive-node claims, production full-node readiness, production-funds wallet use, migration apply mode, packaging, hosted services, and GUI work.

</deferred>

***

*Phase: 128-production-compact-announcement-transport*
*Context gathered: 2026-07-19*
