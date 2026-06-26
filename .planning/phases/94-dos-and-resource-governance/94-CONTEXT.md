---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 94-2026-06-26T15-47-23
generated_at: 2026-06-26T15:47:23.352Z
---

# Phase 94: DoS and Resource Governance - Context

**Gathered:** 2026-06-26
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Phase 94 adds deterministic inbound DoS and resource-governance controls for the v1.9 opt-in inbound serving surface. It should bound message-envelope parsing, payload allocation, per-peer and aggregate request/queue pressure, slow handshakes, idle peers, churn, repeated failures, and banned or discouraged reconnect attempts, while making resource pressure visible through shared operator evidence.

This phase extends Phase 90 listener/admission, Phase 91 permission classes, Phase 92 address-boundary behavior, and Phase 93 eviction/ban/misbehavior policy. It must not enable transaction relay, compact block relay, mempool propagation, broad address relay, public inbound defaults, public-network CI, production-service support, or production full-node readiness.

</domain>

<decisions>
## Implementation Decisions

### Message Envelope And Payload Allocation

- **D-01:** Add a typed resource-governance gate before allocation-heavy inbound message handling. The gate must reject wrong network magic, malformed headers, unsupported commands, oversized payloads, checksum failures, malformed payloads, and trailing data through stable labels before creating unbounded buffers or peer-side work.
- **D-02:** Preserve and centralize existing hard caps such as `MAX_SIZE`, `MAX_HEADERS_RESULTS`, `MAX_INV_SIZE`, `DEFAULT_MAX_BLOCKS_IN_FLIGHT_PER_PEER`, and `PHASE92_ADDR_BATCH_LIMIT`. If Phase 94 adds new caps, they should be named constants in a focused policy module with tests proving the boundary.
- **D-03:** Unsupported command handling should remain bounded evidence, not a feature expansion. Repeated unsupported commands may feed Phase 93 misbehavior decisions, but Phase 94 must not start handling mempool, compact-block, filter, or relay commands.
- **D-04:** Parser errors should map to low-cardinality resource or violation labels such as `wrong_network_magic`, `malformed_header`, `payload_oversized`, `invalid_checksum`, `unsupported_command`, `malformed_payload`, and `trailing_payload`. These labels should be usable by tests, metrics, logs, status, support bundles, and docs.

### Request, Queue, And Backpressure Bounds

- **D-05:** Model request and queue governance as pure data-in/data-out policy before runtime effects. Suggested inputs include peer role, handshake state, permission effects, current per-peer queued reads/writes, aggregate queued reads/writes, requested inventory counts, block/header/transaction request counts, and resource-pressure observations.
- **D-06:** Enforce explicit per-peer and aggregate read/write queue limits with stable outcomes. The runtime may apply socket backpressure or disconnects, but it should consume a policy output rather than recalculating queue pressure in the accept loop.
- **D-07:** Bound inventory and request surfaces without enabling transaction relay. `inv`, `getdata`, `headers`, `getheaders`, block, and transaction request tracking should have caps for inbound peers, but transaction relay, mempool propagation, compact blocks, BIP37, and compact-filter serving remain inactive or deferred.
- **D-08:** Permissioned and protected peers may receive scoped policy treatment, but they still count toward resource evidence. `download`, `addr`, `noban`, and `forceinbound` effects can influence bounded policy decisions; relay-like inactive effects must not grant extra queues, request capacity, or serving behavior.

### Timeouts, Churn, Idle Peers, And Reconnects

- **D-09:** Slow handshakes, idle peers, churn, repeated failures, and reconnect suppression should be represented by typed policy decisions evaluated from injected timestamps and counters. Runtime clocks belong in shell adapters; pure policy accepts `now` as data.
- **D-10:** Phase 94 should define deterministic labels for timeout and churn outcomes, such as `slow_handshake`, `idle_peer`, `connection_churn_limited`, `repeated_failure_limited`, `reconnect_suppressed_banned`, and `reconnect_suppressed_discouraged`.
- **D-11:** Banned and discouraged reconnect attempts should use the Phase 93 ban/discourage model as an input and produce explicit evidence. Do not hide broad bans in the listener runtime, and do not silently drop protected-peer violations.
- **D-12:** Tests must avoid wall-clock sleeps. Use injected timestamps, synthetic peer records, loopback-safe fixtures, and deterministic counters to prove timeout, idle, churn, and reconnect behavior.

### Operator Evidence, Metrics, Logs, And Support

- **D-13:** Resource-governance evidence belongs in the shared inbound status/support contract first, then CLI status, RPC/Open Bitcoin status, metrics, logs, and support renderers project the same fields. Avoid renderer-local resource summaries.
- **D-14:** Evidence must stay low-cardinality, bounded, and redacted. Status/support output may include aggregate counters, latest stable event, reason, source, and next action, but not raw peer ids, raw endpoint tables, raw message payloads, raw permission config strings, credentials, or unbounded queue contents.
- **D-15:** Resource-pressure evidence should include useful next actions. Suggested labels include `resource_pressure_active`, `read_queue_pressure`, `write_queue_pressure`, `request_cap_reached`, `payload_rejected`, `timeout_disconnect`, `churn_rejected`, and `reconnect_suppressed`.
- **D-16:** Metrics should remain fixed `MetricKind` variants or equivalent aggregate counters. Do not introduce dynamic labels for peer id, endpoint, command payload, permission class name, ban scope, or raw address.

### Verification, UAT, And Boundaries

- **D-17:** Default verification remains `bash scripts/verify.sh`, deterministic, local, public-network-free, service-manager-free, and short-running. Use pure policy tests, synthetic wire fixtures, and loopback-safe checks instead of public inbound exposure.
- **D-18:** Add unit tests for the pure policy and parser boundaries using Arrange, Act, Assert. Cover wrong magic, malformed header, oversized payload, unsupported command, malformed payload, queue pressure, request caps, backpressure, slow handshake, idle peer, churn, reconnect suppression, protected-peer evidence, and no relay side effects.
- **D-19:** Add deterministic checker coverage if docs/parity evidence is updated. The checker should follow Phase 90-93 patterns and reject positive claims for transaction relay, compact blocks, mempool propagation, public inbound defaults, public-network readiness, production service, or production full-node readiness.
- **D-20:** Any operator UAT text must include repo-local Cargo and Bazel command forms, not an installed alias alone.

### the agent's Discretion

The planner may choose exact cap values, type names, and module splits. Prefer focused pure policy modules over expanding already-large files such as `message.rs`, `peer.rs`, `inbound.rs`, or `metrics.rs`; use small integration points in those files only when that preserves a clear public API. Keep runtime socket/backpressure behavior thin and driven by policy outputs.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Project And Workflow Rules

- `AGENTS.md` - repo-local verification, GSD workflow, parity breadcrumb, and repo-local UAT command rules.
- `AGENTS.bright-builds.md` - Bright Builds sync, verification, testing, architecture, and task artifact rules.
- `standards/core/architecture.md` - functional core / imperative shell and domain-type rules.
- `standards/core/code-shape.md` - early-return, optional-name, script, and file/function shape rules.
- `standards/core/testing.md` - unit test behavior and Arrange/Act/Assert requirements.
- `standards/core/verification.md` - repo-native verification and commit gate expectations.
- `standards/languages/rust.md` - Rust module, optional naming, invariant, and verification guidance.
- `standards/languages/typescript-javascript.md` - Bun-backed TypeScript checker and automation guidance when scripts are touched.

### Phase Scope And Requirements

- `.planning/PROJECT.md` - active v1.9 inbound-serving scope, deferred relay/production boundaries, and Knots anchor expectations.
- `.planning/REQUIREMENTS.md` - DOS-01 through DOS-05 plus v1.9 future/out-of-scope relay and production boundaries.
- `.planning/ROADMAP.md` - Phase 94 goal, success criteria, and requirement mapping.
- `.planning/STATE.md` - current milestone position and carry-forward v1.9 workflow notes.
- `.planning/phases/90-inbound-listener-and-admission-policy/90-CONTEXT.md` - locked listener/admission decisions that Phase 94 must extend.
- `.planning/phases/91-peer-permissions-and-connection-classes/91-CONTEXT.md` - locked permission and protected-peer decisions that Phase 94 must respect.
- `.planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md` - locked address request and learned-address caps that Phase 94 should preserve.
- `.planning/phases/93-eviction-ban-and-misbehavior-policy/93-CONTEXT.md` - locked eviction, ban, discourage, and misbehavior decisions that Phase 94 should consume, not replace.

### Existing Code Integration Points

- `packages/open-bitcoin-codec/src/network.rs` - message-header parsing, `MAX_SIZE`, compact-size handling, and low-level codec errors.
- `packages/open-bitcoin-network/src/message.rs` - wire message enum, command dispatch, payload decoding, checksum validation, `MAX_HEADERS_RESULTS`, `MAX_INV_SIZE`, and `PHASE92_ADDR_BATCH_LIMIT` usage.
- `packages/open-bitcoin-network/src/message/tests.rs` - wire-message parser and unknown-command test style.
- `packages/open-bitcoin-network/src/peer.rs` - pure peer lifecycle, `PeerState`, request tracking, `handle_message`, inventory handling, getaddr/addr handling, and `PeerAction::Disconnect`.
- `packages/open-bitcoin-network/src/peer/tests.rs` - peer lifecycle, request, handshake, message-policy, and Arrange/Act/Assert test patterns.
- `packages/open-bitcoin-network/src/peer_policy.rs` - Phase 93 eviction, ban, unban, and misbehavior policy model that Phase 94 should consume for reconnect and abuse outcomes.
- `packages/open-bitcoin-network/src/inbound.rs` - listener preflight, admission policy, slot classes, handshake state, peer records, permission exports, and stable rejection labels.
- `packages/open-bitcoin-network/src/inbound/tests.rs` - pure inbound preflight/admission test style with stable labels.
- `packages/open-bitcoin-network/src/address.rs` - Phase 92 address caps, getaddr request state, learned-address decisions, and address-boundary evidence.
- `packages/open-bitcoin-node/src/network.rs` - `ManagedPeerNetwork`, node-side message handling, block/tx inventory serving, and shared peer-network projection.
- `packages/open-bitcoin-node/src/network/inbound.rs` - managed inbound admission, permission, address, peer-policy projection, and shared event construction.
- `packages/open-bitcoin-node/src/status/inbound.rs` - shared inbound status contract to extend with resource-governance evidence.
- `packages/open-bitcoin-node/src/metrics.rs` - fixed metric surface and low-cardinality metric constraints.
- `packages/open-bitcoin-cli/src/operator/status/render/inbound.rs` - human status projection for bounded inbound evidence.
- `packages/open-bitcoin-cli/src/operator/support/render/inbound.rs` - support Markdown projection and redaction pattern for inbound evidence.
- `scripts/check-phase90-inbound-listener-admission.ts` - deterministic checker and no-claim pattern for Phase 90.
- `scripts/check-phase91-peer-permissions.ts` - deterministic checker and permission-evidence pattern for Phase 91.
- `scripts/check-phase92-address-boundaries.ts` - deterministic checker and address-boundary/no-claim pattern for Phase 92.
- `scripts/check-phase93-eviction-ban-policy.ts` - deterministic checker and peer-policy/no-claim pattern for Phase 93.

### Docs, Evidence, And Release Boundaries

- `docs/architecture/status-snapshot.md` - shared status ownership, inbound status contract, unavailable-field policy, and evidence placement.
- `docs/architecture/operator-observability.md` - status, metrics, logs, support evidence interpretation, and low-cardinality inbound evidence guidance.
- `docs/operator/runtime-guide.md` - repo-local operator command style, opt-in UAT posture, and no-production/no-relay-claim language.
- `docs/parity/catalog/p2p.md` - existing P2P coverage, Phase 90-93 evidence, and explicit non-claims for relay and production readiness.
- `docs/parity/release-readiness.md` - deterministic verifier/public-network boundary and deferred-surface wording.
- `docs/parity/checklist.md` - parity checklist roots.
- `docs/parity/index.json` - machine-readable parity root.
- `docs/parity/source-breadcrumbs.json` - required breadcrumb registrations for new first-party Rust sources/tests.

### Knots Anchors

- `packages/bitcoin-knots/src/protocol.h` - message command names, message size limits, and protocol envelope constants.
- `packages/bitcoin-knots/src/net.cpp` - socket receive/send loops, message envelope rejection, payload-size limits, banned/discouraged connection handling, connection timeouts, and reconnect filtering.
- `packages/bitcoin-knots/src/net_processing.cpp` - request bounds, inventory handling, timeouts, stalling behavior, misbehavior hooks, and DoS response boundaries.
- `packages/bitcoin-knots/src/net_permissions.cpp` - `noban`, `download`, `forceinbound`, and permission effects that resource policy must respect without enabling relay.
- `packages/bitcoin-knots/src/banman.cpp` - banned/discouraged reconnect behavior and expiry handling from Phase 93.
- `packages/bitcoin-knots/test/functional/p2p_invalid_messages.py` - malformed message and disconnect behavior anchor.
- `packages/bitcoin-knots/test/functional/p2p_dos_header_tree.py` - header DoS behavior anchor.
- `packages/bitcoin-knots/test/functional/p2p_timeouts.py` - timeout behavior anchor.
- `packages/bitcoin-knots/test/functional/p2p_ibd_stalling.py` - stalling behavior anchor for bounded sync/request policy; keep public-network behavior out of default verification.
- `packages/bitcoin-knots/test/functional/p2p_getdata.py` - request and inventory behavior anchor; use only for bounded non-relay serving behavior.
- `packages/bitcoin-knots/test/functional/p2p_disconnect_ban.py` - disconnect/ban interaction anchor already scoped by Phase 93.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- `ParsedNetworkMessage::decode_wire` already parses the 24-byte message header, checks exact payload length, validates checksum, and dispatches command payload decoding.
- `WireNetworkMessage::decode_payload` already rejects unknown commands with `NetworkError::UnknownCommand` and enforces empty payloads for `verack`, `wtxidrelay`, `sendheaders`, and `getaddr`.
- `MAX_HEADERS_RESULTS`, `MAX_INV_SIZE`, `PHASE92_ADDR_BATCH_LIMIT`, and `DEFAULT_MAX_BLOCKS_IN_FLIGHT_PER_PEER` already provide focused caps that Phase 94 can preserve or wrap in broader policy evidence.
- `PeerState` already tracks handshake booleans, requested blocks, requested txids/wtxids, getaddr request state, inbound admission record, and inbound rejection reason.
- `PeerAction::Disconnect`, Phase 93 `MisbehaviorPolicy`, `PeerBanBook`, and managed peer-policy projection provide the disconnect/abuse evidence seam.
- `InboundPeerServingStatus` already centralizes listener, permission, address, eviction, ban, and misbehavior evidence for status/support surfaces.

### Established Patterns

- Pure network policy belongs in `open-bitcoin-network`; runtime I/O, clocks, sockets, storage, logs, and process effects stay in node/RPC adapters.
- Shared status owns evidence before CLI/status/support renderers format it.
- Deterministic checker scripts use Bun/TypeScript and fixed-file fixtures to prevent release-boundary drift.
- Default verification avoids public peers, public listener exposure, service-manager operations, sleeps, multi-day timing, DNS/seed crawling, and public-network UAT.
- New Rust sources/tests require parity breadcrumbs in file comments and `docs/parity/source-breadcrumbs.json` entries.
- Several likely touch points are near the file-size trigger: `message.rs`, `peer.rs`, `inbound.rs`, and `metrics.rs`; new focused modules are preferred when adding substantial behavior.

### Integration Points

- Add resource-governance policy near `open-bitcoin-network/src/peer.rs`, `message.rs`, `inbound.rs`, or a new `resource.rs`/`peer_resource.rs` module if that keeps boundaries clear.
- Project resource-governance evidence through `ManagedPeerNetwork`, `ManagedInbound*` projection, `InboundPeerServingStatus`, CLI status, support rendering, and optional metrics.
- Use Phase 93 ban/discourage and misbehavior policy as inputs for reconnect suppression rather than duplicating ban logic.
- Extend docs/parity/catalog entries and source breadcrumbs when new public evidence or first-party source/test files are added.
- Add a deterministic Phase 94 checker only after docs/parity evidence exists, following the Phase 90-93 checker pattern.

</code_context>

<specifics>
## Specific Ideas

- Suggested stable labels include `wrong_network_magic`, `malformed_header`, `payload_oversized`, `invalid_checksum`, `unsupported_command`, `malformed_payload`, `trailing_payload`, `read_queue_pressure`, `write_queue_pressure`, `request_cap_reached`, `slow_handshake`, `idle_peer`, `connection_churn_limited`, `repeated_failure_limited`, `reconnect_suppressed_banned`, `reconnect_suppressed_discouraged`, and `resource_pressure_active`.
- Prefer injected timestamps and synthetic records over sleeps for timeout and churn tests.
- Keep block/header/download permissions bounded to scoped request-serving policy; do not expose mempool inventory, compact blocks, transaction relay, force relay, BIP37, compact filters, or full relay behavior.
- Keep support bundles concise: aggregate counters plus the latest safe event are better than raw peer tables, payload dumps, or queue contents.
- Include repo-local Cargo and Bazel command forms in any Phase 94 operator UAT text.

</specifics>

<deferred>
## Deferred Ideas

- Phase 95 owns v1.9 release-boundary docs, no-claim evidence, final parity traceability, and cross-phase non-regression closure.
- Future milestones own transaction relay, compact block relay, mempool propagation, BIP37/compact-filter serving, full address relay, public inbound defaults, public-network CI, production service packaging, and production full-node readiness.

</deferred>

---

*Phase: 94-dos-and-resource-governance*
*Context gathered: 2026-06-26*
