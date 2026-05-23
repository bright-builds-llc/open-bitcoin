---
phase: 36
phase_name: "Mainnet Peer Discovery and Outbound Lifecycle"
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: "36-2026-05-01T22-57-33"
generated_at: "2026-05-01T22:57:33.102Z"
---

# Phase 36 Context: Mainnet Peer Discovery and Outbound Lifecycle

**Gathered:** 2026-05-01
**Status:** Ready for planning

<domain>
## Phase Boundary

Add the daemon-owned outbound peer layer that Phase 35 intentionally deferred. This phase owns deterministic seed and manual-peer resolution, bounded outbound peer lifecycle management, unhealthy peer rotation, and peer telemetry that later header/block sync phases can reuse. It does not claim inbound serving, addrman parity, transaction relay, or final operator dashboard/RPC presentation polish that later phases own.

</domain>

<decisions>
## Implementation Decisions

### Resolver and address sourcing
- **D-01:** Resolve manual peers and DNS seeds through an injected resolver interface owned by the sync shell, never by pure-core crates or direct calls from tests.
- **D-02:** Treat manual peers and DNS seeds as separate sources in the domain model and preserve that source label in telemetry and failure reporting.
- **D-03:** Normalize resolved candidates into explicit socket-address domain records before connection attempts so downstream sync logic does not re-parse host strings.

### Outbound peer pool
- **D-04:** Replace the current sequential `candidate_peers()` walk with a bounded outbound peer pool that tracks candidate, connecting, connected, stalled, and failed lifecycle states plus retry metadata.
- **D-05:** Keep pool policy deterministic and adapter-driven: target outbound count, connect timeout, read/stall timeout, retry budget, and backoff schedule must all come from typed runtime config.
- **D-06:** When alternatives exist, stalled or invalid peers must be disconnected and deprioritized instead of allowing one bad peer to monopolize the run.

### Failure semantics and resilience
- **D-07:** Introduce typed peer-lifecycle failure reasons for resolver failure, connect timeout, stall timeout, invalid magic/data, transport I/O, and policy-driven disconnects so `RESUME-02` stays explicit.
- **D-08:** Persist only the durable sync artifacts already owned by the runtime store in this phase; peer pool state itself may stay in-memory for now as long as failure outcomes and health signals are surfaced truthfully.
- **D-09:** Keep later header/block phases unblocked by making the peer layer reusable through transport/session abstractions rather than binding header progress directly to one transport loop.

### Telemetry and observability
- **D-10:** Extend sync telemetry to record peer source, resolved endpoint, negotiated network identity, service-capability summary, contribution counters, last activity, attempts, and terminal failure reason.
- **D-11:** Keep the telemetry model shell-safe but transport-agnostic so current status/log/metric surfaces can consume it without leaking sockets, DNS, or clocks into pure-core crates.

### the agent's Discretion
- Exact target outbound peer count and backoff constants.
- Whether resolver output is consumed lazily or batched per round, as long as deterministic tests stay easy.
- The narrowest status/log surface needed in Phase 36 before broader observability lands in Phase 39.

</decisions>

<specifics>
## Specific Ideas

- Preserve the current `SyncTransport` seam and evolve it instead of hard-wiring `TcpStream` deeper into the runtime.
- Use deterministic scripted transports and resolvers in tests so `bash scripts/verify.sh` remains hermetic.
- Prefer a small explicit peer lifecycle state machine over ad hoc booleans spread across the runtime.

</specifics>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Milestone scope
- `.planning/REQUIREMENTS.md` — Phase 36 requirements `PEERMAIN-01` through `PEERMAIN-04`, `RESUME-02`, `VERMAIN-01`, and `VERMAIN-02`.
- `.planning/ROADMAP.md` — Phase 36 goal, success criteria, and boundaries relative to Phases 35 and 37-40.
- `.planning/STATE.md` — Current milestone state and carry-forward constraints from Phase 35.

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
- `packages/open-bitcoin-node/src/sync.rs` — Current durable sync runtime loop, persistence points, and transport integration.
- `packages/open-bitcoin-node/src/sync/types.rs` — Existing sync config, peer address/source types, summary telemetry, and runtime errors.
- `packages/open-bitcoin-node/src/sync/tcp.rs` — Current resolver/transport behavior and the boundary that still embeds DNS/socket resolution.
- `packages/open-bitcoin-network/src/peer.rs` — Existing pure-core peer manager and header/block message flow.
- `packages/open-bitcoin-node/src/status.rs` — Current sync and peer status contracts.
- `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs` — Phase 35 daemon preflight boundary that Phase 36 extends.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `DurableSyncRuntime` already owns durable store access, sync metrics/log writing, and the `SyncTransport` abstraction.
- `SyncPeerAddress`, `SyncPeerSource`, `SyncRunSummary`, and `SyncRuntimeError` provide a starting domain model for Phase 36 instead of inventing a separate peer subsystem.
- `ManagedPeerNetwork` and `open-bitcoin-network::PeerManager` already manage handshake/message progression once a peer session is established.

### Established Patterns
- Shell adapters (`sync/tcp.rs`) currently own network and DNS effects; that boundary should remain thin and injectable.
- Sync status/logging flows already convert runtime errors into `HealthSignal` plus structured logs, so peer lifecycle failures should plug into the same pattern.
- Hermetic tests use scripted transports in `packages/open-bitcoin-node/src/sync/tests.rs`; Phase 36 should extend that style rather than adding live-network defaults.

### Integration Points
- `SyncRuntimeConfig` needs new peer-pool and timeout/backoff knobs.
- `sync/tcp.rs` likely needs a resolver abstraction split from session transport so DNS resolution is injectable.
- `SyncRunSummary` and/or adjacent status contracts need richer peer telemetry fields that later phases can expose.
- Phase 37 header sync should be able to consume the new outbound peer lifecycle without reworking the daemon/runtime boundary again.

</code_context>

<deferred>
## Deferred Ideas

- Persistent peer database / addrman parity, peer gossip, eviction, banning, and inbound serving remain future work outside v1.2 Phase 36.
- Operator-facing dashboard/RPC truth polish for peer telemetry remains Phase 39 work, beyond the minimum telemetry surface this phase needs.
- Block-download scheduling, restart recovery for partial block/header work, and reorg handling remain Phases 37-38.

</deferred>

---

*Phase: 36-mainnet-peer-discovery-and-outbound-lifecycle*
*Context gathered: 2026-05-01*
