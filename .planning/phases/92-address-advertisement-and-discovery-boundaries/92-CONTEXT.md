---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 92-2026-06-26T03-52-33
generated_at: 2026-06-26T03:55:28.679Z
---

# Phase 92: Address Advertisement and Discovery Boundaries - Context

**Gathered:** 2026-06-26
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Phase 92 adds the first Open Bitcoin-owned address advertisement and discovery-boundary surface for v1.9 inbound peer serving. It should derive local listen address candidates from configured listener endpoints, advertise only reachable and privacy-safe candidates, answer inbound `getaddr` requests through a bounded deterministic policy, and introduce a typed learned-address management contract with freshness, source, routability, and persistence evidence.

This phase must remain narrower than full address relay. It may create local listener advertisement, scoped `getaddr` response behavior, and address-manager contracts that later phases can build on, but it must not claim broad peer discovery, gossip-style address relay, DNS seed governance, public inbound defaults, eviction/ban policy, DoS/resource governance, transaction relay, compact block relay, mempool propagation, or production full-node readiness.

</domain>

<decisions>
## Implementation Decisions

### Local Listener Advertisement

- **D-01:** Address advertisement starts from Open Bitcoin-owned `inbound.listen_addresses` and runtime-bound listener evidence created in Phase 90. Do not infer public advertisements from arbitrary local interfaces, outbound peers, DNS discovery, UPnP/NAT-PMP, external IP probes, or baseline Knots `-externalip`/`-discover` compatibility.
- **D-02:** Candidate derivation must be a pure decision that accepts typed listener endpoints, listener state, service flags, reachability/privacy configuration, and current network boundary inputs. Runtime socket code should only consume the decision output.
- **D-03:** Loopback, private, unspecified, multicast, documentation, and otherwise unroutable addresses are not advertised to public peers. Loopback may be retained only as deterministic local/UAT evidence with a stable reason such as `not_publicly_routable`.
- **D-04:** Privacy-network boundaries are explicit. Onion, I2P, CJDNS, and future non-IP reachability should be represented as deferred or unsupported address networks unless the planner adds a bounded typed placeholder with tests proving it cannot leak or relay unsupported privacy-network addresses.
- **D-05:** Version-message sender address behavior should stay conservative. Do not start sending a routable local address in `version` unless the address passes the same typed candidate policy; otherwise keep the existing zero-address behavior.

### Bounded `getaddr` Response Policy

- **D-06:** Add `getaddr` and `addr` message support only for bounded request/response behavior. This phase should not implement gossip relay, addr rebroadcast scheduling, trickle relay, unsolicited address fanout, or full addr relay peer selection.
- **D-07:** The `getaddr` response policy must be deterministic and permission-aware. Permission decisions from Phase 91 should influence whether a peer is eligible for address responses through the existing `addr`/address-response policy input, but raw class names and raw config strings must stay out of status/support output.
- **D-08:** Responses must be capped by explicit count, age, source, cache, and request-frequency rules. The cap should be small enough for deterministic tests and should not depend on wall-clock network crawling or public peers.
- **D-09:** The response cache should be typed and inspectable. Each returned address must have evidence for source, first-seen or last-seen freshness, routability classification, services, port, and whether it came from local listener advertisement or learned peer-address storage.
- **D-10:** Repeated `getaddr` requests from the same peer must not create unbounded work or change relay state. Use a stable "served once" or deterministic request-window policy, with a reason label when a later request is suppressed.

### Learned Address Management Contract

- **D-11:** Introduce a first-party typed address-management contract before durable persistence details become complicated. Required concepts include network kind, address bytes or endpoint, service flags, source, freshness timestamps, routability class, and persistence eligibility.
- **D-12:** Learned `addr` entries should be accepted only through parser and policy boundaries. Invalid ports, unsupported address networks, unroutable entries, stale timestamps, self/local loopback leakage, and over-cap batches must produce stable rejection or quarantine reasons.
- **D-13:** Persistence may be an in-memory or snapshot-backed contract in this phase, but it must expose deterministic evidence showing what would be persisted and why. Do not imply full Knots `addrman.dat`, anchor persistence, DNS seed rotation, or production peer-discovery parity unless explicitly implemented and tested.
- **D-14:** Learned-address state should integrate with existing pure network/domain crates first, then project bounded status/support evidence through shared node status surfaces. Avoid renderer-local address summaries.

### Operator Evidence, Docs, And Release Boundaries

- **D-15:** Status/support evidence should distinguish at least four concepts with stable labels: local listener advertisement candidates, suppressed advertisements, bounded `getaddr` responses, and learned address-management entries.
- **D-16:** Documentation must keep local listener advertisement, inbound `getaddr` responses, learned address storage, peer discovery, and full address relay visibly separate. Any future full-relay wording belongs to deferred/future sections.
- **D-17:** Deterministic release checks should guard the boundary by proving docs and parity catalogs mention Phase 92 address behavior without claiming full address relay, broader peer discovery, public-network defaults, or production readiness.
- **D-18:** Operator UAT commands, if added, must include repo-local Cargo and Bazel forms from `AGENTS.md`; do not rely on an installed `open-bitcoin` alias alone.

### Verification And UAT

- **D-19:** Default verification must stay deterministic, local, public-network-free, service-manager-free, and short-running. Use pure policy tests, synthetic `addr`/`getaddr` messages, loopback listener fixtures, and fixed docs/checker fixtures.
- **D-20:** Unit tests should cover local candidate classification, privacy-network suppression, `version` sender-address gating, `getaddr` response caps, permission-aware address responses, duplicate/stale/unroutable learned entries, and no full-relay side effects.
- **D-21:** Add parity breadcrumbs for any new first-party Rust source or test files under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests`, including `docs/parity/source-breadcrumbs.json` entries. Use `none` only for Open Bitcoin-only status/support infrastructure without a defensible Knots anchor.

### the agent's Discretion

The planner may choose exact type names, module splits, and whether the first learned-address store is in-memory or snapshot-backed. Prefer a small pure address-policy/address-manager module in `open-bitcoin-network`, thin projection in `open-bitcoin-node`, config or CLI additions only when needed for scoped behavior, and docs/checkers that make non-claims explicit.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Project And Workflow Rules

- `AGENTS.md` - repo-local verification, parity breadcrumb, GSD workflow, and repo-local UAT command rules.
- `AGENTS.bright-builds.md` - Bright Builds sync, verification, testing, architecture, and task artifact rules.
- `standards/core/architecture.md` - functional core / imperative shell and domain-type rules.
- `standards/core/code-shape.md` - early-return, optional-name, and file/function shape rules.
- `standards/core/testing.md` - unit test behavior and Arrange/Act/Assert requirements.
- `standards/core/verification.md` - repo-native verification and commit gate expectations.
- `standards/languages/rust.md` - Rust module, optional naming, invariant, and verification guidance.
- `standards/languages/typescript-javascript.md` - Bun-backed TypeScript checker and automation guidance.

### Phase Scope And Requirements

- `.planning/PROJECT.md` - active v1.9 address advertisement/discovery boundary, deferred relay/production boundaries, and Knots anchor expectations.
- `.planning/REQUIREMENTS.md` - ADDR-01 through ADDR-04 plus v1.9 future/out-of-scope relay and production boundaries.
- `.planning/ROADMAP.md` - Phase 92 goal, success criteria, and requirement mapping.
- `.planning/STATE.md` - current milestone position and carry-forward v1.9 workflow notes.
- `.planning/phases/90-inbound-listener-and-admission-policy/90-CONTEXT.md` - locked listener/admission decisions that Phase 92 must extend.
- `.planning/phases/91-peer-permissions-and-connection-classes/91-CONTEXT.md` - locked permission and address-response-policy input decisions that Phase 92 must use.

### Existing Code Integration Points

- `packages/open-bitcoin-primitives/src/network.rs` - `NetworkAddress`, message command primitives, and wire-level address data shape.
- `packages/open-bitcoin-codec/src/network.rs` - network-address codec support and existing compact-size/message parsing patterns.
- `packages/open-bitcoin-network/src/message.rs` - `WireNetworkMessage`, version sender/receiver address handling, command dispatch, and payload codec extension point for `getaddr`/`addr`.
- `packages/open-bitcoin-network/src/message/tests.rs` - wire-message round-trip and unknown-command test style.
- `packages/open-bitcoin-network/src/inbound.rs` - Phase 90 listener preflight, listener endpoint, admission policy, and Phase 91 permission exports.
- `packages/open-bitcoin-network/src/inbound/permissions.rs` - `addr` permission token, `AddressResponsePolicyInput`, inactive/deferred permission labels, and literal-IP class matching.
- `packages/open-bitcoin-network/src/peer.rs` - pure peer lifecycle and message-action handling where scoped `getaddr`/`addr` behavior should integrate.
- `packages/open-bitcoin-network/src/peer/tests.rs` - pure peer and message policy tests with existing Arrange/Act/Assert patterns.
- `packages/open-bitcoin-node/src/network.rs` - `ManagedPeerNetwork`, node-side message processing, and shared peer-network projection.
- `packages/open-bitcoin-node/src/network/inbound.rs` - managed inbound admission and permission evidence that Phase 92 should consume for address-response policy.
- `packages/open-bitcoin-node/src/status/inbound.rs` - shared inbound serving status contract to extend with bounded address evidence.
- `packages/open-bitcoin-node/src/metrics.rs` - low-cardinality metric surface if address evidence needs metrics.
- `packages/open-bitcoin-rpc/src/config/open_bitcoin.rs` - JSONC-owned inbound config shape if address-specific knobs are required.
- `packages/open-bitcoin-rpc/src/config/loader/inbound.rs` - Open Bitcoin-prefixed inbound CLI parser if address-specific CLI overrides are required.
- `packages/open-bitcoin-rpc/src/context/network.rs` - RPC context wrapper and network status wiring.
- `packages/open-bitcoin-cli/src/operator/status/render/inbound.rs` - human status projection for inbound evidence.
- `packages/open-bitcoin-cli/src/operator/support/render/inbound.rs` - support Markdown projection and redaction pattern for inbound evidence.
- `scripts/check-phase90-inbound-listener-admission.ts` - deterministic checker/no-claim pattern for Phase 90.
- `scripts/check-phase91-peer-permissions.test.ts` and `scripts/check-phase91-peer-permissions.ts` - deterministic checker/no-claim and permission-evidence pattern for Phase 91.

### Docs, Evidence, And Release Boundaries

- `docs/architecture/config-precedence.md` - Open Bitcoin JSONC ownership, CLI precedence, inbound config boundary, and invalid `bitcoin.conf` key policy.
- `docs/architecture/status-snapshot.md` - shared status ownership, Phase 90/91 inbound contracts, unavailable-field policy, and evidence placement.
- `docs/architecture/operator-observability.md` - status, metrics, logs, support evidence interpretation, and low-cardinality inbound evidence guidance.
- `docs/operator/runtime-guide.md` - repo-local operator command style, opt-in UAT posture, and no-production/no-relay-claim language.
- `docs/parity/catalog/p2p.md` - existing P2P coverage, Phase 90/91 evidence, and explicit non-claims for address relay and production readiness.
- `docs/parity/release-readiness.md` - deterministic verifier/public-network boundary and deferred-surface wording.
- `docs/parity/checklist.md` - parity checklist roots.
- `docs/parity/index.json` - machine-readable parity root.
- `docs/parity/source-breadcrumbs.json` - required breadcrumb registrations for new first-party Rust sources/tests.

### Knots Anchors

- `packages/bitcoin-knots/src/netaddress.h` - address network, routability, reachability, and serialization concepts.
- `packages/bitcoin-knots/src/netaddress.cpp` - routability/reachability logic and address classification details.
- `packages/bitcoin-knots/src/addrman.h` - address manager contract, freshness/source concepts, and persistence boundary anchor.
- `packages/bitcoin-knots/src/addrman.cpp` - address manager behavior, selection, and learned-address handling anchor.
- `packages/bitcoin-knots/src/addrdb.h` - address database/persistence boundary anchor.
- `packages/bitcoin-knots/src/addrdb.cpp` - address database read/write behavior anchor.
- `packages/bitcoin-knots/src/net.cpp` - local address advertisement, reachable-network, and connection-manager address behavior.
- `packages/bitcoin-knots/src/net_processing.cpp` - `getaddr`/`addr` message handling, permission effects, relay hazards, and response boundaries.
- `packages/bitcoin-knots/test/functional/p2p_getaddr_caching.py` - bounded `getaddr` cache behavior anchor.
- `packages/bitcoin-knots/test/functional/p2p_addrfetch.py` - address-fetch behavior anchor.
- `packages/bitcoin-knots/test/functional/p2p_addr_relay.py` - full address relay behavior that Phase 92 must not overclaim.
- `packages/bitcoin-knots/test/functional/p2p_addrv2_relay.py` - `addrv2` relay behavior to defer unless explicitly bounded.
- `packages/bitcoin-knots/test/functional/feature_addrman.py` - address-manager behavior and persistence anchor.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- `NetworkAddress` and `encode_network_address`/`parse_network_address` already cover the legacy 26-byte address shape used by `version` and future `addr` payloads.
- `WireNetworkMessage` is the central enum for P2P commands; it currently lacks `GetAddr`, `Addr`, and `AddrV2`, so Phase 92 can extend command handling deliberately.
- `LocalPeerConfig::version_message` currently uses one local address for both receiver and sender; this is the narrow seam for conservative sender-address gating.
- `InboundListenerConfig` and `InboundListenerEndpoint` already normalize listener endpoints before runtime bind attempts.
- `PeerPermissionSet::active_effects` already exposes `AddressResponsePolicyInput` for the `addr` permission token.
- `ManagedInboundPermissionDecisionInfo` and `ManagedInboundAdmissionInfo` already preserve bounded permission evidence without raw class names.
- `InboundPeerServingStatus` is the shared status contract for inbound listener/admission evidence and can be extended with address-boundary fields if needed.

### Established Patterns

- Pure policy and parser decisions belong in `open-bitcoin-network`; runtime/socket effects stay in node/RPC adapters.
- Shared status owns evidence first, then CLI/support/rendering layers project it.
- Deterministic checker scripts use Bun/TypeScript and fixed-file fixtures to prevent release-boundary drift.
- Default verification avoids public peers, public-network listener exposure, service-manager operations, multi-day timing, and real DNS/seed crawling.
- New Rust sources/tests require parity breadcrumbs and `docs/parity/source-breadcrumbs.json` coverage.

### Integration Points

- Add address policy, learned-address contract, and `getaddr`/`addr` handling near `open-bitcoin-network/src/message.rs`, `peer.rs`, and a new `address.rs`/`address/` module if that keeps files below the local size trigger.
- Project address evidence through `ManagedPeerNetwork`, `InboundPeerServingStatus`, CLI status rendering, support rendering, and optional metrics only after pure policy fields exist.
- Extend docs and parity catalogs with a Phase 92 surface id and deterministic checker similar to Phase 91.
- Keep any `addrv2`, DNS seed, gossip relay, and public-network discovery logic explicitly deferred unless a plan can prove bounded no-claim behavior.

</code_context>

<specifics>
## Specific Ideas

- Suggested stable labels include `advertise_candidate`, `advertise_suppressed`, `not_publicly_routable`, `privacy_network_deferred`, `getaddr_served`, `getaddr_suppressed`, `learned_accepted`, `learned_rejected`, `source_local_listener`, `source_inbound_addr`, and `full_relay_deferred`.
- Prefer a small fixed response cap for deterministic tests, such as a constant in the pure policy module, rather than exposing a broad operator-facing tuning surface in the first pass.
- Treat `addrv2` as an explicit future/deferred surface unless implementing it is necessary for a typed placeholder. Legacy `addr` support is sufficient for the bounded Phase 92 goal.
- Keep release docs explicit that bounded `getaddr` responses and learned-address contracts do not mean Open Bitcoin is a full peer-discovery or address-relay participant.

</specifics>

<deferred>
## Deferred Ideas

- Phase 93 owns eviction, disconnect, discourage, ban, expiry, unban, and misbehavior behavior.
- Phase 94 owns broader inbound DoS/resource governance beyond address response caps.
- Phase 95 owns v1.9 release-boundary docs and no-claim evidence across inbound serving.
- Future milestones own full address relay, addr rebroadcast scheduling, address gossip fanout, `addrv2` relay parity, DNS seed governance, public inbound defaults, public-network CI, and production full-node readiness.

</deferred>

---

*Phase: 92-address-advertisement-and-discovery-boundaries*
*Context gathered: 2026-06-26*
