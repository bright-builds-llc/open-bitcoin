---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 96-2026-06-28T02-38-04
generated_at: 2026-06-28T02:43:32.273Z
---

# Phase 96: Peer Policy Runtime Bridge - Context

**Gathered:** 2026-06-28
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Phase 96 closes the peer-policy runtime bridge gaps found by the v1.9 milestone audit. It connects Phase 93 ban, unban, discourage, and misbehavior policy decisions into live managed runtime state, scoped reconnect suppression, shared status/RPC/CLI/support/log evidence, and deterministic local verification.

This phase must close `INT-01-peer-policy-runtime-bridge` and `FLOW-01-peer-policy-to-runtime` without creating a public banlist surface, enabling transaction relay, enabling compact block relay, enabling mempool propagation, changing public inbound defaults, adding public-network verification, or claiming production-service or production full-node readiness.

</domain>

<decisions>
## Implementation Decisions

### Managed Peer-Policy Runtime State

- **D-01:** Prefer a pure `PeerManager`-owned peer-policy state, or an equivalent pure `open-bitcoin-network` state reached through `PeerManager`, as the source of truth for live ban, unban, discourage, and misbehavior decisions. Use `open-bitcoin-node` only as a thin managed projection and clock/runtime adapter.
- **D-02:** Do not solve the audit gap by appending only a bounded decision history in `ManagedPeerNetwork`. A history can support evidence, but scoped reconnect checks must query authoritative typed state rather than reconstructing policy from recent events.
- **D-03:** Do not make a Fjall-backed event ledger the first runtime authority unless execution proves it is necessary. The phase is a bridge and deterministic local behavior closure; durable storage should not imply production banlist parity or cross-restart public-network governance.
- **D-04:** Extend or wrap `PeerBanBook` so callers can ask whether a remote `IpAddr` matches an active address or subnet ban at an injected `now_unix_seconds`. Expiry evaluation must remain deterministic and shell clocks must be passed in as data.
- **D-05:** Add explicit scoped discourage state if the existing Phase 93 model does not already preserve it. Discouraged reconnects must be distinguishable from active bans in evidence and policy output.
- **D-06:** `ManagedPeerNetwork::peer_policy_info()` must no longer call `ManagedPeerPolicyInfo::from_policy_decisions(..., &[], &[], &[])` for active runtime paths. It should project actual bounded ban, unban, discourage, and misbehavior decisions from the policy state.
- **D-07:** Keep public evidence bounded. Expose aggregate counters and the latest sanitized policy event, not raw ban tables, raw endpoints, peer IDs, permission class names, raw config strings, payloads, credentials, or unbounded ledgers.

### Reconnect Suppression And Admission Boundary

- **D-08:** `ManagedRpcContext::reconnect_suppression_input_for_remote_addr` must use `remote_addr.ip()` and an injected timestamp. It must not derive `banned` or `discouraged` from aggregate counters such as `active_bans > 0`.
- **D-09:** Reconnect suppression should query exact address/subnet ban state and explicit discourage state before translating the scoped result into the existing Phase 94 resource-governance labels, such as `reconnect_suppressed_banned` and `reconnect_suppressed_discouraged`.
- **D-10:** Scoped suppression belongs beside the runtime listener decision path, but it should not be folded into the Phase 90 admission cap model unless implementation shows a clear need. Preserve the distinction between admission-cap rejection, peer-policy suppression, and resource-governance evidence.
- **D-11:** Permissioned and protected peers must still produce bounded evidence when they hit ban, discourage, or misbehavior policy. `noban` or protected connection classes may prevent a ban or eviction action, but they must not hide observations, protected-no-action outcomes, or reconnect-policy reasons.
- **D-12:** Avoid hidden broad bans. A ban or discourage entry for one address or subnet must not suppress unrelated remote addresses, and tests must prove this with at least one non-matching remote.

### Status, RPC, CLI, Support, Logs, And Checkers

- **D-13:** Keep the shared inbound status snapshot as the public evidence boundary. RPC, CLI status, support rendering, logs, and future metric work should consume shared `InboundPeerServingStatus` or managed inbound evidence rather than computing renderer-local peer-policy summaries.
- **D-14:** A snapshot-first bridge is the preferred public evidence shape: aggregate counts plus a latest sanitized event are enough for Phase 96. A full public event ledger is out of scope unless a later phase deliberately plans it.
- **D-15:** Add bounded structured log evidence only from sanitized policy events. Logs must use low-cardinality labels and safe sources, not raw peer-policy material.
- **D-16:** Add a Phase 96 deterministic checker and fixture test if docs, parity roots, or verifier ordering are updated. The checker should fail on empty decision-slice projection, aggregate-only reconnect suppression, raw peer-policy leaks, public-network verification, and production or relay claim creep.
- **D-17:** Preserve existing status/support fields from Phase 93 and Phase 94 where possible. Extend the shared contract only where the bridge needs a new low-cardinality counter, source, or latest-event label.

### Verification And UAT

- **D-18:** Default verification remains `bash scripts/verify.sh`, deterministic, local-only, public-network-free, service-manager-free, and short-running.
- **D-19:** Unit tests should cover address-scoped bans, subnet-scoped bans, expiry, manual unban, explicit discourage state, misbehavior decision recording, protected-no-action evidence, matching reconnect suppression, and non-matching reconnect admission using Arrange, Act, Assert.
- **D-20:** Integration tests may use synthetic inbound records and loopback-safe listener fixtures. They must not require public peers, DNS/seed crawling, wall-clock sleeps, service-manager operations, or long-running public-network review.
- **D-21:** Operator UAT documentation, if touched, must include repo-local Cargo and Bazel command forms, including `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- ...` and `bazel run //packages/open-bitcoin-cli:open_bitcoin -- ...`, not only an installed alias.

### the agent's Discretion

The planner may choose exact type names, module boundaries, and whether the pure state is named `PeerPolicyState`, `PeerPolicyRuntimeState`, or an extension of `PeerBanBook`. Prefer small pure modules in `open-bitcoin-network`, thin managed adapters in `open-bitcoin-node`/`open-bitcoin-rpc`, and one shared status projection over duplicating evidence in CLI or support renderers.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Project And Workflow Rules

- `AGENTS.md` - repo-local verification, GSD workflow, parity breadcrumb, Rust, TypeScript, and repo-local UAT command rules.
- `AGENTS.bright-builds.md` - Bright Builds sync, verification, testing, architecture, code-shape, and task artifact rules.
- `standards-overrides.md` - active local standards exception surface.
- `standards/core/architecture.md` - functional core / imperative shell and domain-type rules.
- `standards/core/code-shape.md` - early-return, optional-name, script, and file/function shape rules.
- `standards/core/testing.md` - unit test behavior and Arrange/Act/Assert requirements.
- `standards/core/verification.md` - repo-native verification and commit gate expectations.
- `standards/languages/rust.md` - Rust module, optional naming, invariant, and verification guidance.
- `standards/languages/typescript-javascript.md` - Bun-backed TypeScript checker and automation guidance when scripts are touched.

### Phase Scope And Gap Evidence

- `.planning/PROJECT.md` - active v1.9 inbound-serving scope, deferred relay/production boundaries, and Knots anchor expectations.
- `.planning/REQUIREMENTS.md` - EVICT-03, EVICT-04, DOS-03, and v1.9 future/out-of-scope relay and production boundaries.
- `.planning/ROADMAP.md` - Phase 96 goal, success criteria, plan outline, and dependency context.
- `.planning/STATE.md` - current milestone state, carry-forward decisions, and local workflow notes.
- `.planning/v1.9-MILESTONE-AUDIT.md` - `INT-01-peer-policy-runtime-bridge` and `FLOW-01-peer-policy-to-runtime` evidence.

### Prior v1.9 Context

- `.planning/phases/90-inbound-listener-and-admission-policy/90-CONTEXT.md` - listener, admission, runtime, status, UAT, and no-claim boundaries.
- `.planning/phases/91-peer-permissions-and-connection-classes/91-CONTEXT.md` - permission labels, `noban`, `forceinbound`, inactive relay effects, and redaction boundaries.
- `.planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md` - local advertisement, bounded `getaddr`, learned-address evidence, and full address-relay deferral.
- `.planning/phases/93-eviction-ban-and-misbehavior-policy/93-CONTEXT.md` - ban, unban, discourage, misbehavior, protected-peer, and evidence decisions that Phase 96 must wire into runtime.
- `.planning/phases/94-dos-and-resource-governance/94-CONTEXT.md` - reconnect suppression labels, resource-governance event paths, and deterministic local verification boundaries.
- `.planning/phases/95-network-participation-evidence-and-release-boundary/95-CONTEXT.md` - no-claim, UAT, support-redaction, and checker boundary decisions.

### Existing Code Integration Points

- `packages/open-bitcoin-network/src/peer_policy.rs` - Phase 93 pure eviction, ban, unban, and misbehavior policy model.
- `packages/open-bitcoin-network/src/peer.rs` - `PeerManager`, peer lifecycle, eviction inputs, message handling, disconnect seam, and possible home for pure policy-state APIs.
- `packages/open-bitcoin-network/src/resource.rs` - Phase 94 reconnect suppression decisions and stable resource-governance labels.
- `packages/open-bitcoin-node/src/network.rs` - `ManagedPeerNetwork`, current `peer_policy_info()` empty-slice projection gap, and managed resource event recording.
- `packages/open-bitcoin-node/src/network/inbound.rs` - `ManagedPeerPolicyInfo::from_policy_decisions`, status event projection, and peer-policy counter mapping.
- `packages/open-bitcoin-node/src/status/inbound.rs` - shared inbound status snapshot consumed by RPC, CLI, and support surfaces.
- `packages/open-bitcoin-rpc/src/context/network.rs` - current aggregate-only `reconnect_suppression_input_for_remote_addr` gap and `current_inbound_status` projection.
- `packages/open-bitcoin-rpc/src/inbound_listener.rs` - runtime listener path that should consume scoped reconnect suppression and record bounded events.
- `packages/open-bitcoin-rpc/src/inbound_listener/tests.rs` - loopback-safe resource-governance and reconnect suppression test style.
- `packages/open-bitcoin-cli/src/operator/status/render/inbound.rs` - human status renderer for inbound evidence.
- `packages/open-bitcoin-cli/src/operator/support/render/inbound.rs` - support Markdown renderer and redaction boundary for inbound evidence.

### Docs, Evidence, And Checker Patterns

- `docs/architecture/status-snapshot.md` - shared status ownership, inbound status contract, unavailable-field policy, and evidence placement.
- `docs/architecture/operator-observability.md` - status, metrics, logs, support evidence interpretation, and low-cardinality inbound evidence guidance.
- `docs/operator/runtime-guide.md` - repo-local operator command style, opt-in UAT posture, support evidence, and no-production/no-relay wording.
- `docs/parity/catalog/p2p.md` - P2P parity surfaces, v1.9 inbound evidence, Knots anchors, and deferred relay/production wording.
- `docs/parity/index.json` - machine-readable parity root.
- `docs/parity/checklist.md` - human parity checklist and requirement surface table.
- `docs/parity/release-readiness.md` - release-readiness evidence and no-claim review root.
- `docs/parity/source-breadcrumbs.json` - required breadcrumb registrations for new first-party Rust source and test files.
- `scripts/verify.sh` - repo-native verification contract and checker ordering.
- `scripts/check-phase93-peer-policy.ts` - deterministic peer-policy checker and no-claim pattern.
- `scripts/check-phase94-dos-resource-governance.ts` - resource-governance checker, reconnect labels, and verifier-order pattern.
- `scripts/check-phase95-network-boundary.ts` - aggregate release-boundary checker pattern.

### Knots Anchors

- `packages/bitcoin-knots/src/net.cpp` - banned/discouraged connection filtering, eviction/disconnect behavior, timeouts, and connection manager hooks.
- `packages/bitcoin-knots/src/net_processing.cpp` - misbehavior accounting, protocol violation responses, permission effects, and disconnect/ban interaction.
- `packages/bitcoin-knots/src/banman.h` - ban/discourage data model, scope, expiry, and API anchors.
- `packages/bitcoin-knots/src/banman.cpp` - ban persistence, expiry sweep, ban/unban behavior, serialization, and scoped matching anchors.
- `packages/bitcoin-knots/src/net_permissions.cpp` - `noban`, protected peers, and permission behavior that Phase 96 must respect without enabling relay.
- `packages/bitcoin-knots/test/functional/p2p_disconnect_ban.py` - disconnect and ban interaction anchor.
- `packages/bitcoin-knots/test/functional/p2p_invalid_messages.py` - protocol violation and disconnect behavior anchor.
- `packages/bitcoin-knots/test/functional/p2p_permissions.py` - permission and protected-peer expectation anchor.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- `PeerBanBook`, `BanScope`, `BanDecision`, `UnbanDecision`, `MisbehaviorDecision`, and `MisbehaviorPolicy` already provide typed pure policy primitives for Phase 96 to wire into live state.
- `ManagedPeerPolicyInfo::from_policy_decisions` already maps policy decisions into aggregate counters and latest safe `InboundPeerPolicyEvent`; the missing piece is feeding it actual runtime decisions.
- `ResourceGovernancePolicy::decide_reconnect` and labels such as `reconnect_suppressed_banned` and `reconnect_suppressed_discouraged` already provide the bounded Phase 94 reconnect evidence path.
- `InboundPeerServingStatus` is already the shared status/support contract for inbound listener, permission, address, peer-policy, and resource-governance evidence.
- `ManagedRpcContext::record_inbound_resource_event` already records managed resource events for listener evidence.

### Established Patterns

- Pure policy belongs in `open-bitcoin-network`; runtime clocks, sockets, storage, logs, and process effects stay in node/RPC adapters.
- Shared status owns evidence first, then RPC, CLI, and support surfaces render that evidence.
- Deterministic checker scripts use Bun/TypeScript, fixed file sets, and explicit no-claim assertions.
- Default verification avoids public peers, public listener exposure, service-manager operations, sleeps, multi-day timing, DNS/seed crawling, and public-network UAT.
- New Rust source or test files under first-party crates need parity breadcrumb blocks and `docs/parity/source-breadcrumbs.json` entries.

### Integration Points

- Add scoped policy-state APIs in `open-bitcoin-network` near `peer_policy.rs` or `PeerManager`, then project them through `ManagedPeerNetwork`.
- Replace aggregate-only reconnect suppression in `ManagedRpcContext::reconnect_suppression_input_for_remote_addr` with scoped lookup by `remote_addr.ip()` and injected `now_unix_seconds`.
- Extend listener/runtime tests to prove matching and non-matching remote addresses, active/expired bans, discourage state, and protected-peer evidence.
- Preserve CLI/support renderer shape unless the shared status contract needs new low-cardinality fields.
- Add a Phase 96 checker only after implementation/docs need one, following Phase 93 through Phase 95 patterns.

</code_context>

<specifics>
## Specific Ideas

- Suggested pure-state API shape: query by `IpAddr` and `now_unix_seconds`, returning typed scoped outcomes such as `banned`, `discouraged`, `allowed`, `expired`, or `protected_no_action`.
- Suggested checker assertions: no `from_policy_decisions(..., &[], &[], &[])` in the runtime projection path, no `_ = (remote_addr, now_unix_seconds)`, no `active_bans > 0` aggregate-only reconnect suppression, no public-network verifier gate, and no positive relay/production claims.
- Keep latest-event evidence intentionally bounded. A public event ledger, support-bundle ban table, or raw ban-scope dump is outside this phase.
- If docs are updated, keep Phase 96 wording as "scoped runtime peer-policy bridge" or equivalent, not "production banlist", "public ban manager", or "full network participation".

</specifics>

<deferred>
## Deferred Ideas

- Durable cross-restart peer-policy event replay and public banlist-style management remain future scope unless a later phase deliberately plans them.
- Phase 97 owns inbound metric sample production and dashboard history; Phase 96 may expose counters but should not own persisted metric-sample production.
- Phase 98 owns final requirements traceability closure and any remaining milestone audit artifact updates.
- Transaction relay, mempool propagation, compact block relay, full address relay, public inbound defaults, public-network CI, production service packaging, and production full-node readiness remain future milestone scope.

</deferred>

---

*Phase: 96-peer-policy-runtime-bridge*
*Context gathered: 2026-06-28*
