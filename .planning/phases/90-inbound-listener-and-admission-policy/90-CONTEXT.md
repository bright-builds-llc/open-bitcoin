---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 90-2026-06-25T04-23-47
generated_at: 2026-06-25T04:23:47.878Z
---

# Phase 90: Inbound Listener and Admission Policy - Context

**Gathered:** 2026-06-25
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Phase 90 introduces the first explicit opt-in inbound listener and admission path for Open Bitcoin. It must let operators enable or disable inbound serving through Open Bitcoin-owned configuration or daemon CLI controls, bind configured listener endpoints only after deterministic preflight, admit inbound peers as typed peer records, run the existing version/verack handshake lifecycle, enforce inbound caps and reserved slots without starving outbound sync, and expose operator evidence for listener/admission outcomes.

This phase must not expand transaction relay, compact block relay, mempool propagation, full address relay, eviction/ban policy, permission-class policy, public inbound defaults, production service support, or production full-node readiness. Those are later v1.9 or future-milestone surfaces.

</domain>

<decisions>
## Implementation Decisions

### Activation And Listener Preflight

- **D-01:** Inbound serving is disabled by default. A disabled runtime must not bind any P2P listener, create accept-loop tasks, or report listener success.
- **D-02:** Phase 90 should use Open Bitcoin-owned controls rather than silently accepting baseline `bitcoin.conf` listener keys. Add JSONC-owned config under an `inbound` section, with at least `enabled`, `listen_addresses`, `max_peers`, and `reserved_slots`. Add daemon CLI overrides with an Open Bitcoin prefix, such as `-openbitcoininbound=1` and `-openbitcoinlisten=<host:port>`, so this phase does not imply full Knots `-listen` or `-bind` compatibility.
- **D-03:** Listener preflight must be a typed, deterministic result before any socket side effect. Required outcomes include `disabled`, `no_listen_addresses`, `invalid_endpoint`, `unsafe_endpoint`, `bind_unavailable`, `already_bound`, and `ready`.
- **D-04:** Loopback endpoints are the default deterministic test/UAT target. Wildcard or public interfaces require an explicit public-exposure acknowledgement field, for example `inbound.allow_public = true`, and are never part of `bash scripts/verify.sh`.
- **D-05:** Preflight diagnostics must include the endpoint, stable reason code, human message, and next action. Error messages should name the exact config or CLI field that needs correction.

### Admission And Handshake Lifecycle

- **D-06:** Keep admission decisions in pure domain types before runtime socket effects. Introduce first-party types such as `InboundListenerConfig`, `InboundAdmissionPolicy`, `InboundAdmissionDecision`, and `InboundPeerRecord` in the network/node boundary rather than burying policy inside the Tokio accept loop.
- **D-07:** Reuse and extend the existing `PeerManager` and `ManagedPeerNetwork` inbound role support. The current `add_inbound_peer`, `ConnectionRole::Inbound`, `PeerState`, and `network_info` count paths are the starting point; Phase 90 should add enough metadata to distinguish accepted, rejected, handshaking, established, duplicate, self-connection, and disconnected inbound peers.
- **D-08:** The inbound handshake should reuse the existing message-driven version/verack path. A newly accepted inbound peer starts without `local_version_sent`, then sends local `version`, `wtxidrelay`, `verack`, and `sendheaders` only through the same `PeerAction` flow used today.
- **D-09:** Duplicate and self-connection protection is required before a peer is counted as admitted. Use stable connection keys based on remote endpoint and handshake nonce where available, reject duplicate peer IDs, and reject a remote nonce matching the local nonce as a self-connection signal.
- **D-10:** Phase 90 may parse ordinary P2P messages already supported by the core, but it must not use inbound serving as a way to claim transaction relay, compact block relay, mempool propagation, full address relay, or production network participation. Any relay-related capability should stay explicitly deferred or inert.

### Caps, Reserved Slots, And Outbound Sync Safety

- **D-11:** Inbound caps are separate from outbound sync targets. `target_outbound_peers` and existing durable sync behavior must not be reduced or starved by inbound peers.
- **D-12:** Admission policy should expose `max_inbound_peers`, `reserved_slots`, current inbound count, and current outbound count as pure inputs and outputs. If the cap is reached, the rejection reason must be stable and operator-visible.
- **D-13:** Reserved slots are an admission primitive in Phase 90, not the full permission system. They can be modeled and tested now, but Phase 91 owns Knots-aligned permission classes and richer protected-peer policy.
- **D-14:** The listener/accept loop must have a bounded shutdown path tied to `open-bitcoind` graceful shutdown. Dropping or disabling the listener should stop accepting new peers without disturbing existing outbound sync unless an explicit shutdown occurs.

### Operator Evidence, RPC, Metrics, Logs, And Support

- **D-15:** Extend operator evidence from the shared status model. Inbound listener state and admission outcomes should surface through `OpenBitcoinStatusSnapshot` or a clearly owned child contract, then render consistently in CLI status, dashboard/status JSON, support bundles, metrics, structured logs, and RPC-facing status.
- **D-16:** `getnetworkinfo` already exposes `connections`, `connections_in`, and `connections_out`; Phase 90 should keep those fields accurate and add Open Bitcoin-specific status evidence for listener/preflight/admission rather than changing baseline-shaped fields in surprising ways.
- **D-17:** Evidence labels must separate inbound serving from outbound sync. Suggested stable labels include listener state, bound endpoints, preflight reason, admitted inbound peers, rejected inbound peers, handshake state counts, duplicate/self-connection rejects, cap rejects, and latest admission event.
- **D-18:** Support bundles must preserve diagnostic usefulness without copying raw unbounded peer tables. Peer endpoint evidence should be bounded and redacted where needed, following existing support-bundle redaction patterns.

### Verification And UAT

- **D-19:** Default verification must remain deterministic, local, short-running, public-network-free, and real-service-manager-free. Use loopback listeners, injected transports, synthetic peers, and hermetic handshake fixtures for `bash scripts/verify.sh`.
- **D-20:** Unit tests should focus on pure admission policy, preflight classification, cap accounting, duplicate/self-connection rejection, and peer-state transitions with Arrange/Act/Assert structure.
- **D-21:** Integration tests may bind `127.0.0.1:0` using the existing test-harness listener pattern. They should assert that disabled config does not bind, invalid endpoints produce stable diagnostics, and enabled loopback admission increments inbound counts without changing outbound counts.
- **D-22:** Any operator UAT text must include repo-local Cargo and Bazel forms, not only an installed alias. Use commands such as `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind -- ...` and `bazel run //packages/open-bitcoin-rpc:open_bitcoind -- ...`, plus the repo-local `open-bitcoin-cli` status forms.

### the agent's Discretion

The planner may choose exact module splits and naming if they preserve the locked boundaries above. Prefer a small pure policy module plus a thin runtime adapter over a large listener file. Prefer extending existing status/support contracts only where it keeps one shared source of truth; avoid renderer-local inbound summaries.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Project And Workflow Rules

- `AGENTS.md` - repo-local verification, parity breadcrumb, GSD workflow, and UAT command rules.
- `AGENTS.bright-builds.md` - Bright Builds workflow, sync, verification, testing, and architecture defaults.
- `standards/core/architecture.md` - functional core / imperative shell and domain-type rules.
- `standards/core/code-shape.md` - early-return and optional-name conventions.
- `standards/core/testing.md` - unit test structure and focus.
- `standards/core/verification.md` - repo-native verification and commit gate rules.
- `standards/languages/rust.md` - Rust module, optional naming, invariant, and verification guidance.

### Phase Scope And Requirements

- `.planning/PROJECT.md` - active v1.9 inbound-serving scope, deferred relay/production boundaries, and Knots anchor expectations.
- `.planning/REQUIREMENTS.md` - INB-01 through INB-05 and v1.9 future/out-of-scope requirements.
- `.planning/ROADMAP.md` - Phase 90 goal, success criteria, and requirement mapping.
- `.planning/STATE.md` - current milestone position and pending v1.9 workflow notes.

### Existing Code Integration Points

- `packages/open-bitcoin-network/src/peer.rs` - pure peer lifecycle, inbound/outbound roles, version/verack handling, peer state, duplicate peer ID detection, and existing message actions.
- `packages/open-bitcoin-network/src/peer/tests.rs` - existing handshake, inbound peer, request, and error coverage patterns.
- `packages/open-bitcoin-node/src/network.rs` - `ManagedPeerNetwork`, `ManagedNetworkInfo`, inbound/outbound count projection, and node-side action processing.
- `packages/open-bitcoin-node/src/status.rs` - shared status contracts, `PeerCounts`, `PeerTelemetry`, `PeerStatus`, and `OpenBitcoinStatusSnapshot`.
- `packages/open-bitcoin-rpc/src/config.rs` - runtime config root and existing Open Bitcoin-only daemon sync config pattern.
- `packages/open-bitcoin-rpc/src/config/open_bitcoin.rs` - JSONC-owned config shape and `SyncConfig` precedent for Open Bitcoin-owned network settings.
- `packages/open-bitcoin-rpc/src/config/loader.rs` - CLI and config precedence parsing, including `-openbitcoinsync`.
- `packages/open-bitcoin-rpc/src/config/loader/open_bitcoin_runtime.rs` - Open Bitcoin JSONC runtime resolution for daemon sync.
- `packages/open-bitcoin-rpc/src/context/network.rs` - `ManagedRpcContext` network wrapper, default P2P ports, `add_inbound_peer`, and `getnetworkinfo` backing state.
- `packages/open-bitcoin-rpc/src/method/node.rs` - `getnetworkinfo` response shape, including `connections_in` and `connections_out`.
- `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs` - daemon startup, Tokio listener precedent, daemon sync preflight, worker lifecycle, and graceful shutdown.
- `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests.rs` - preflight and daemon-loop test style.
- `packages/open-bitcoin-test-harness/src/isolation.rs` - loopback `TcpListener` allocation for hermetic tests.
- `packages/open-bitcoin-cli/src/operator/status.rs` - live RPC status collection and peer count projection.
- `packages/open-bitcoin-cli/src/operator/status/render.rs` - human and JSON status rendering patterns.
- `packages/open-bitcoin-cli/src/operator/support/` - support evidence, redaction, rendering, resource, and forensic patterns.
- `packages/open-bitcoin-node/src/metrics.rs` - metric naming and existing peer-count metric surface.

### Docs, Evidence, And Release Boundaries

- `docs/architecture/config-precedence.md` - Open Bitcoin JSONC ownership, CLI precedence, and invalid `bitcoin.conf` key boundary.
- `docs/architecture/status-snapshot.md` - shared status ownership and unavailable-field policy.
- `docs/architecture/operator-observability.md` - status, metrics, logs, and support evidence interpretation.
- `docs/operator/runtime-guide.md` - repo-local operator command style, opt-in UAT posture, and no-production-claim language.
- `docs/parity/catalog/p2p.md` - existing outbound P2P coverage and explicit non-claims for inbound, relay, and production readiness.
- `docs/parity/release-readiness.md` - deterministic verifier/public-network boundary and deferred-surface wording.
- `docs/parity/checklist.md` - parity checklist roots.
- `docs/parity/index.json` - machine-readable parity root.
- `docs/parity/source-breadcrumbs.json` - required breadcrumb registrations for new first-party Rust sources/tests.
- `.planning/phases/83-support-matrix-and-issue-evidence/83-CONTEXT.md` - support-term, issue-evidence, repo-local command, and public-network default boundaries.
- `.planning/phases/86-service-operation-expectations/86-CONTEXT.md` - source-built daemon/service command and no-production-service boundaries.
- `.planning/phases/88-deterministic-claim-guardrails/88-CONTEXT.md` - broad deferred-surface and default-verification guardrails.
- `.planning/phases/89-release-readiness-guardrail-closure/89-CONTEXT.md` - latest guardrail corpus and no-claim closure rules.

### Knots Anchors

- `packages/bitcoin-knots/src/net.cpp` - listener, connection manager, bind/listen, and socket lifecycle behavior.
- `packages/bitcoin-knots/src/net_processing.cpp` - peer handshake and message-processing parity anchor.
- `packages/bitcoin-knots/src/net_permissions.cpp` - permission concepts to avoid pre-empting before Phase 91.
- `packages/bitcoin-knots/src/addrman.cpp` - address-management anchor for later address phases; use only to avoid accidental broad address-relay claims.
- `packages/bitcoin-knots/src/banman.cpp` - ban-policy anchor for later phases; Phase 90 should not implement ban semantics.
- `packages/bitcoin-knots/test/functional/p2p_handshake.py` - handshake fixture behavior already cited by current peer tests.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- `PeerManager::add_inbound_peer` and `ConnectionRole::Inbound` already create typed inbound peers in the pure core.
- `PeerManager::handle_version` and `handle_verack` already implement the message-driven handshake state that inbound admission should reuse.
- `ManagedPeerNetwork::network_info` already counts inbound and outbound peers from the peer manager.
- `ManagedRpcContext::add_inbound_peer` already exposes a test-facing inbound entry point.
- `GetNetworkInfoResponse` already has baseline-compatible `connections`, `connections_in`, and `connections_out`.
- `OpenBitcoinStatusSnapshot` and support-bundle code already provide the shared evidence pattern for status, support, dashboard, metrics, and logs.
- `packages/open-bitcoin-test-harness/src/isolation.rs` already has a loopback listener helper for hermetic socket tests.

### Established Patterns

- Open Bitcoin-owned runtime configuration lives in `open-bitcoin.jsonc` and CLI overrides with Open Bitcoin-specific keys; baseline `bitcoin.conf` should not silently accept Open Bitcoin-only settings.
- Pure network and status decisions live below shell adapters, while `open-bitcoind` owns runtime I/O, worker lifecycle, and graceful shutdown.
- Public-network and real-service-manager checks stay opt-in UAT outside `bash scripts/verify.sh`.
- Status/support surfaces preserve unavailable reasons rather than dropping fields.
- New first-party Rust sources/tests under `packages/open-bitcoin-*/src` or `tests` need parity breadcrumbs in file comments and `docs/parity/source-breadcrumbs.json`.

### Integration Points

- Add pure listener/admission policy to `open-bitcoin-network` or a narrow `open-bitcoin-node` policy module, depending on whether it needs chain/runtime state.
- Add socket listener/accept-loop wiring in `open-bitcoind` or a node runtime adapter called from it.
- Extend runtime config parsing in `packages/open-bitcoin-rpc/src/config/` and Open Bitcoin JSONC parsing in `config/open_bitcoin.rs`.
- Project listener/admission state through `ManagedNetworkInfo`, `OpenBitcoinStatusSnapshot`, CLI status renderers, support bundle renderers, and metrics/log emitters.
- Add deterministic checker/docs updates only if Phase 90 introduces new docs or release-boundary assertions.

</code_context>

<specifics>
## Specific Ideas

- Prefer loopback-first UAT examples and tests: `127.0.0.1:0` or a fixed loopback port in manual commands, with public interfaces guarded behind explicit opt-in wording.
- Treat listener preflight as a reusable operator diagnostic, not just startup failure text.
- Keep the first implementation narrow: bind, accept, admit, handshake, count, and report. Do not solve permission classes, address relay, eviction, banning, or DoS policy in this phase.
- Use stable machine labels for diagnostics so docs, tests, metrics, and support bundles do not drift.
- Ensure every operator-facing UAT command includes both Cargo and Bazel forms.

</specifics>

<deferred>
## Deferred Ideas

- Phase 91 owns peer permissions and connection classes.
- Phase 92 owns local address advertisement, `getaddr` response boundaries, and address-management contracts.
- Phase 93 owns eviction, ban, discourage, and misbehavior policy.
- Phase 94 owns inbound DoS/resource governance beyond Phase 90 admission caps.
- Phase 95 owns release-boundary and no-claim evidence across v1.9.
- Future milestones own transaction relay, compact block relay, mempool propagation, public inbound defaults, signed packaging, Windows service support, hosted dashboards, GUI, public-network CI, and production full-node readiness claims.

</deferred>

---

*Phase: 90-inbound-listener-and-admission-policy*
*Context gathered: 2026-06-25*
