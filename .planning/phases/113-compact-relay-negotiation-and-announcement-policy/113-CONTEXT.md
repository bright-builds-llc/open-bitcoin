---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 113-2026-07-04T22-53-48
generated_at: 2026-07-04T22:53:48.000Z
---

# Phase 113: Compact Relay Negotiation and Announcement Policy - Context

**Gathered:** 2026-07-04
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Phase 113 adds pure compact-block relay negotiation state and compact-block announcement policy on top of the Phase 112 BIP152 wire messages. The phase decides when a peer is compact-block capable, whether the peer prefers high-bandwidth or low-bandwidth compact relay, and whether a newly available validated block may be announced with `cmpctblock` or must fall back to headers or inventory behavior.

This phase may consume `sendcmpct` messages, store per-peer compact relay state, add compact-announcement decision types, and prove announcement eligibility through deterministic tests. It must not reconstruct compact blocks from mempool state, request missing transactions, accept `blocktxn` responses, mutate chainstate from partial compact-block state, add broad operator evidence rollout, enable package relay, enable bloom/filter or compact-filter serving, change public defaults, add public-network CI gates, claim archive-node behavior, claim production full-node readiness, or claim production-funds wallet safety.

</domain>

<decisions>
## Implementation Decisions

### Negotiation State

- **D-01:** Per-peer compact relay state must be explicit and typed: capability known/unknown, supported version, high-bandwidth preference, low-bandwidth preference, and compact-announcement eligibility must not be inferred from ad hoc booleans.
- **D-02:** `sendcmpct` version 2 is the only in-scope positive capability signal. Unsupported versions should decode as Phase 112 data but map to a stable unsupported/suppressed policy outcome instead of disconnecting by default in this phase.
- **D-03:** High-bandwidth and low-bandwidth preferences are negotiated peer state, not global activation by themselves. A peer can express a preference while still being ineligible because local activation, block-serving eligibility, header state, block availability, or resource limits fail.
- **D-04:** Negotiation state should live in pure `open-bitcoin-network` peer policy/state surfaces, with node-shell adapters only passing messages and consuming actions.

### Announcement Policy

- **D-05:** Compact block announcements are allowed only when all gates pass: local compact-relay activation, peer compact capability, high-bandwidth preference when announcing `cmpctblock`, known header continuity or acceptable tip context, validated local block availability, and resource capacity.
- **D-06:** When any compact gate fails, the policy should choose an explicit fallback action such as headers, inventory, or suppress, with a stable low-cardinality reason. Fallback must be a typed outcome so later operator evidence can summarize it without renderer-local inference.
- **D-07:** `cmpctblock` announcements should remain announcement-only in this phase. Full compact-block reconstruction, missing transaction scheduling, `getblocktxn`, `blocktxn`, and validation/connect handoff remain deferred to Phases 114 and 115.
- **D-08:** Resource gates should reuse Phase 110/111 block-serving request and in-flight policy concepts where they fit, but compact-announcement decisions should have their own labels when that avoids mixing full-block serving and compact-relay evidence.

### Scope Isolation

- **D-09:** Compact relay negotiation must remain independent from transaction relay activation, package relay, bloom/filter permissions, compact filters, public serving defaults, production-service operation, and production full-node readiness.
- **D-10:** Transaction relay or mempool participation may provide future reconstruction inputs, but in Phase 113 they must not be prerequisites for negotiation state or accidental activators for compact announcements.
- **D-11:** Peer permissions such as `download`, protected admission, inbound serving, and transaction relay eligibility may be policy inputs only where prior phases already made them scoped and bounded. They must not grant compact relay, package relay, archive serving, or public defaults by implication.

### Verification And Parity

- **D-12:** Tests must cover valid version 2 `sendcmpct`, unsupported versions, high-bandwidth toggles, low-bandwidth preference, default-disabled suppression, headers/inventory fallback, missing header or unavailable block suppression, and transaction-relay/package-relay isolation.
- **D-13:** New or touched first-party Rust source/test files under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests` require parity breadcrumbs and `docs/parity/source-breadcrumbs.json` entries unless an explicit `none` breadcrumb is defensible.
- **D-14:** Verification remains deterministic and local through repo-native checks and `bash scripts/verify.sh`. Public-network compact-relay review remains opt-in UAT evidence only.

### Claude's Discretion

The planner may choose exact Rust type names, whether compact negotiation lives in a new `compact_relay` peer module or an existing touched peer module, and how fallback actions are named. Prefer small pure policy APIs, low-cardinality reasons, and tests that make accidental public/default/package/filter coupling impossible.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Project And Phase Scope

- `AGENTS.md` - repo-local verification, submodule, parity breadcrumb, UAT command, and GSD workflow guidance.
- `AGENTS.bright-builds.md` - Bright Builds workflow, functional-core, verification, and testing rules.
- `standards/core/architecture.md` - functional core / imperative shell and domain-type rules.
- `standards/core/code-shape.md` - early-return, optional-name, and file/function shape rules.
- `standards/core/testing.md` - focused unit test and Arrange/Act/Assert expectations.
- `standards/core/verification.md` - repo-native verification and clean commit gate expectations.
- `standards/languages/rust.md` - Rust module, invariant, optional naming, and verification guidance.
- `.planning/PROJECT.md` - active v2.1 scope, parity value, architecture constraints, and deferred public/production claims.
- `.planning/REQUIREMENTS.md` - CMP-04, CMP-05, and CMP-06 ownership for Phase 113.
- `.planning/ROADMAP.md` - Phase 113 goal, success criteria, requirement mapping, and plan split.
- `.planning/STATE.md` - current milestone state and deterministic verification caveats.

### Prior Locked Decisions

- `.planning/phases/110-block-serving-activation-and-eligibility-boundary/110-CONTEXT.md` - default-off block/compact activation, peer eligibility, status, resource, and no-claim decisions.
- `.planning/phases/111-full-block-serving-request-path/111-CONTEXT.md` - full/witness block serving request path, resource gates, and explicit compact-block response deferral.
- `.planning/phases/112-bip152-wire-codec-and-message-semantics/112-CONTEXT.md` - BIP152 message types, unsupported `sendcmpct` decode behavior, and runtime-scope boundaries.
- `.planning/phases/100-relay-activation-boundary-and-permission-semantics/100-CONTEXT.md` - relay activation separation and no-claim guardrail pattern.
- `.planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-CONTEXT.md` - runtime activation propagation, download eligibility gates, suppression evidence, and production construction hazards.

### Existing Code Integration Points

- `packages/open-bitcoin-network/src/message.rs` - `WireNetworkMessage::SendCompact`, BIP152 command mapping, and payload decoding surface.
- `packages/open-bitcoin-network/src/message/tests.rs` - existing BIP152 message round-trip and unsupported-version tests.
- `packages/open-bitcoin-codec/src/compact_block.rs` - Phase 112 BIP152 payload types and codec helpers.
- `packages/open-bitcoin-codec/src/compact_block/tests.rs` - compact-block malformed payload and round-trip test style.
- `packages/open-bitcoin-network/src/block_serving.rs` - Phase 110 activation, eligibility, status, resource, and cleanup contracts to consume before compact announcements.
- `packages/open-bitcoin-network/src/peer.rs` - peer manager state and action ownership.
- `packages/open-bitcoin-network/src/peer/inventory_state.rs` - existing getdata, request-pressure, block in-flight, notfound, and received-block paths that compact announcement policy must not regress.
- `packages/open-bitcoin-network/src/peer/transaction_relay.rs` - transaction relay negotiation and scheduler patterns to keep separate from compact relay.
- `packages/open-bitcoin-node/src/network.rs` - managed network runtime and adapter boundary for later consuming compact announcement actions.
- `packages/open-bitcoin-node/src/network/inventory.rs` - Phase 111 block-serving adapter that must continue suppressing compact inventory responses until later phases.
- `packages/open-bitcoin-node/src/status/block_serving.rs` - shared activation/status fields that currently include compact relay activation.
- `docs/parity/source-breadcrumbs.json` - required breadcrumb registry for new first-party Rust source/test files.
- `scripts/verify.sh` - repo-native verification contract.

### Bitcoin Knots Anchors

- `packages/bitcoin-knots/src/net_processing.cpp` - compact-block negotiation, announcement, peer state, block availability, fallback, and request-bound anchors.
- `packages/bitcoin-knots/src/blockencodings.h` - BIP152 compact block data structures and version expectations.
- `packages/bitcoin-knots/src/blockencodings.cpp` - short ID, compact-block validity, and reconstruction boundaries deferred to later phases.
- `packages/bitcoin-knots/src/protocol.h` - `sendcmpct`, `cmpctblock`, inventory constants, and message command names.
- `packages/bitcoin-knots/test/functional/p2p_compactblocks.py` - compact-block negotiation and announcement behavior examples.
- `packages/bitcoin-knots/test/functional/test_framework/messages.py` - compact-block message fixture shapes.
- `packages/bitcoin-knots/src/net_permissions.h` - permission vocabulary and download/relay permission anchors.
- `packages/bitcoin-knots/src/net.cpp` - peer connection classes, protected peer behavior, and resource policy context.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- `SendCompactMessage` and `BIP152_COMPACT_BLOCKS_VERSION` already exist from Phase 112 and provide the wire input for negotiation.
- `BlockRelayActivationPolicy`, `BlockServingActivationConfig`, and `CompactRelayActivationConfig` already separate block serving and compact relay activation.
- Phase 110 `block_serving` policy already provides a pure gate sequence for activation, peer eligibility, block availability, resource pressure, and cleanup labels.
- Peer transaction-relay scheduler tests provide examples for typed peer modes, per-peer announcement state, request suppression reasons, and Arrange/Act/Assert style.

### Established Patterns

- Pure network policy belongs in `open-bitcoin-network`; managed runtime, durable storage, sockets, logs, metrics, and support rendering remain adapter concerns.
- Existing transaction relay state is explicitly negotiated and should be mirrored only as a design pattern, not coupled to compact relay.
- Compact-block inventory from `getdata` is currently bounded and suppressed; Phase 113 should not start serving compact block payloads from inventory requests.
- New files need parity breadcrumbs and registry updates.

### Integration Points

- Add compact negotiation state near peer state/action logic so `sendcmpct` can update the peer record and later block announcements can consume a pure decision.
- Add compact announcement decisions as pure policy outputs that the node shell can later map to `WireNetworkMessage::CompactBlock`, `Headers`, `Inv`, or suppression.
- Keep Phase 114 reconstruction and Phase 115 missing-transaction/fallback state out of this phase's API except for typed deferral outcomes.

</code_context>

<specifics>
## Specific Ideas

- Treat unsupported `sendcmpct` versions as decoded-but-ineligible peer state so future parity work can decide whether to ignore, discourage, or disconnect without changing the codec.
- Use names that make high-bandwidth compact announcements distinct from low-bandwidth compact capability.
- Include tests that prove enabling transaction relay or mempool behavior does not enable compact announcements.
- Prefer fallback decisions that later operator evidence can report as fixed strings such as `compact_relay_disabled`, `compact_peer_not_negotiated`, `compact_high_bandwidth_not_requested`, `compact_block_unavailable`, `compact_headers_fallback`, and `compact_scope_deferred`.

</specifics>

<deferred>
## Deferred Ideas

Compact-block reconstruction from mempool state, short-ID matching, missing transaction request scheduling, `blocktxn` response matching, fallback to full block fetch, validation/connect handoff, broad operator/RPC/CLI/dashboard/metrics/log/support evidence rollout, parity/UAT release closeout, package relay, bloom/filter serving, compact filter serving, public serving defaults, public-network CI, archive-node claims, production full-node readiness, production-service operation, and production-funds wallet use remain outside Phase 113.

</deferred>

***

*Phase: 113-compact-relay-negotiation-and-announcement-policy*
*Context gathered: 2026-07-04*
