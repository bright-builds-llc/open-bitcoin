---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 93-2026-06-26T13-15-10
generated_at: 2026-06-26T13:15:10.369Z
---

# Phase 93: Eviction, Ban, and Misbehavior Policy - Context

**Gathered:** 2026-06-26
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Phase 93 adds deterministic peer eviction, disconnect, discourage, ban, expiry, unban, and misbehavior handling for the v1.9 opt-in inbound serving surface. It should extend the Phase 90 listener/admission model, Phase 91 permission and connection-class model, and Phase 92 address-boundary evidence with pure policy decisions and operator-visible outcomes.

This phase may score peers for eviction, disconnect or evict peers when caps or abuse policy require it, persist scoped discourage/ban state with expiry and manual unban, and map protocol violations to bounded misbehavior responses. It must not expand transaction relay, compact block relay, mempool propagation, public inbound defaults, public-network CI, production full-node readiness, or the broader DoS/resource-governance controls reserved for Phase 94.

</domain>

<decisions>
## Implementation Decisions

### Eviction Scoring And Disconnect Reasons

- **D-01:** Eviction scoring must be pure and deterministic. Inputs should be typed peer records, connection class, inbound slot class, handshake state, activity evidence, address/netgroup or diversity evidence when available, and permission effects from Phase 91.
- **D-02:** Protected peers must be hard to evict accidentally. `forceinbound` and `noban` style effects may make peers ineligible for ordinary eviction or lower their score, but the decision must remain explicit and visible rather than hidden in runtime socket code.
- **D-03:** Admission-cap pressure should first produce a stable candidate selection result with reason labels before runtime side effects. Runtime adapters may then disconnect the selected peer, but they should consume a policy output rather than recalculate the decision.
- **D-04:** Disconnect and eviction outcomes need stable low-cardinality reason codes suitable for tests, metrics, logs, status, support bundles, and docs. Suggested labels include `cap_pressure`, `handshake_stalled`, `duplicate_identity`, `misbehavior_threshold`, `manual_disconnect`, `protected_peer`, and `no_candidate`.

### Discourage, Ban, Expiry, And Unban

- **D-05:** Discourage/ban state should be a first-party domain model, not a loose map of strings. Required fields include address or subnet scope, reason, source, created time, expiry time, active/expired status, and manual unban evidence.
- **D-06:** Bans must be scoped and auditable. Avoid hidden broad-ban behavior; any subnet behavior must be explicit in the type and in operator evidence.
- **D-07:** Expiry evaluation should be deterministic from an injected timestamp. Runtime clocks belong at the shell boundary, while pure policy functions should accept `now` as data.
- **D-08:** Manual unban should be modeled as a reversible state transition with stable outcomes such as `unbanned`, `not_found`, and `already_expired`. A future operator command or RPC surface may call the policy, but this phase should keep mutation explicit and testable.

### Misbehavior Accounting And Protected Peer Handling

- **D-09:** Misbehavior accounting should map named protocol violations to bounded responses. It may cover already-supported inbound/message violations such as malformed or unexpected payloads, duplicate/self-connection signals, repeated invalid address submissions, unsupported command abuse, or handshake failures, but it must not pull Phase 94 queue, timeout, churn, or payload-size governance forward.
- **D-10:** Misbehavior scores, thresholds, and responses must be typed and capped. Responses should include observe-only, disconnect, discourage, ban, and protected/no-action outcomes where appropriate.
- **D-11:** Permissioned and protected peers still produce evidence when they violate policy. `noban` or protected connection classes may prevent ban/eviction actions, but they must not hide the violation, raw decision, or next action from support evidence.
- **D-12:** Misbehavior policy must not enable transaction relay, mempool propagation, compact block relay, force-relay, bloom-filter serving, or compact-filter serving. Relay-like permission labels from Phase 91 remain inactive evidence.

### Operator Evidence, Persistence, And Boundaries

- **D-13:** Project eviction, ban, and misbehavior evidence through the shared status/support model before CLI/support renderers. Avoid renderer-local peer-policy summaries.
- **D-14:** Evidence must be useful but bounded and redacted. Do not expose raw peer IDs, raw endpoint tables, raw permission class names, raw config strings, credentials, or unbounded ban tables in status/support output.
- **D-15:** Metrics and logs should use fixed low-cardinality fields such as eviction candidate count, disconnect count, discourage count, active ban count, expired ban count, unban count, misbehavior observation count, protected-no-action count, and latest stable reason.
- **D-16:** Durable state should use the existing node storage style if persistence is implemented in this phase. Persistence tests may use temporary storage fixtures; default verification must stay deterministic and public-network-free.

### Verification And UAT

- **D-17:** Default verification remains `bash scripts/verify.sh`, with no public-network listener exposure, public peers, service-manager operations, or multi-day timing.
- **D-18:** Unit tests should cover pure eviction candidate ordering, protected-peer immunity, stable reason labels, ban expiry, manual unban, scoped address/subnet matching, bounded misbehavior thresholds, and protected-peer no-action outcomes using Arrange, Act, Assert.
- **D-19:** Integration or node-level tests may use synthetic inbound records and existing loopback-safe fixtures, but should not depend on public peers or wall-clock sleeps.
- **D-20:** Operator UAT documentation, if updated, must include repo-local Cargo and Bazel command forms for daemon startup, network status, operator status, and support bundle review.

### the agent's Discretion

The planner may choose exact module names, score weights, threshold values, and whether the first durable ban store is Fjall-backed or snapshot-backed, as long as the model is typed, deterministic, scoped, and auditable. Prefer pure policy modules in `open-bitcoin-network`, thin managed projection in `open-bitcoin-node`, durable adapter wiring in node/runtime storage only after the pure model exists, and TypeScript checker/docs updates only where they guard real Phase 93 evidence.

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
- `standards/languages/typescript-javascript.md` - Bun-backed TypeScript checker and automation guidance when scripts are touched.

### Phase Scope And Requirements

- `.planning/PROJECT.md` - active v1.9 inbound-serving scope, deferred relay/production boundaries, and Knots anchor expectations.
- `.planning/REQUIREMENTS.md` - EVICT-01 through EVICT-04 plus v1.9 future/out-of-scope relay, DoS/resource, and production boundaries.
- `.planning/ROADMAP.md` - Phase 93 goal, success criteria, and requirement mapping.
- `.planning/STATE.md` - current milestone position and carry-forward v1.9 workflow notes.
- `.planning/phases/90-inbound-listener-and-admission-policy/90-CONTEXT.md` - locked listener/admission decisions that Phase 93 must extend.
- `.planning/phases/91-peer-permissions-and-connection-classes/91-CONTEXT.md` - locked permission and protected-peer decisions that Phase 93 must respect.
- `.planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md` - locked address-boundary and learned-address evidence that Phase 93 may consume for ban/misbehavior scope.

### Existing Code Integration Points

- `packages/open-bitcoin-network/src/inbound.rs` - listener preflight, admission policy, slot classes, handshake state, peer records, and stable rejection labels.
- `packages/open-bitcoin-network/src/inbound/permissions.rs` - `noban`, `forceinbound`, active protection effects, inactive relay-like effects, and literal-IP class matching.
- `packages/open-bitcoin-network/src/peer.rs` - pure peer lifecycle, `PeerState`, `PeerAction::Disconnect`, `remove_peer`, inbound records, `getaddr`/`addr` handling, and message-action integration.
- `packages/open-bitcoin-network/src/address.rs` - learned-address decisions and rejection evidence that may feed scoped ban or misbehavior policy.
- `packages/open-bitcoin-network/src/message.rs` - wire command dispatch and malformed/trailing-payload boundaries for violation accounting.
- `packages/open-bitcoin-node/src/network/inbound.rs` - managed inbound admission, permission evidence, and address-boundary projection.
- `packages/open-bitcoin-node/src/status/inbound.rs` - shared inbound serving status contract to extend with eviction/ban/misbehavior evidence.
- `packages/open-bitcoin-node/src/metrics.rs` - fixed metric enum and low-cardinality metric surface for new peer-policy counters.
- `packages/open-bitcoin-cli/src/operator/status/render/inbound.rs` - human status projection for bounded inbound evidence.
- `packages/open-bitcoin-cli/src/operator/support/render/inbound.rs` - support Markdown projection and redaction pattern for inbound evidence.
- `scripts/check-phase90-inbound-listener-admission.ts` - deterministic checker and no-claim pattern for Phase 90.
- `scripts/check-phase91-peer-permissions.ts` - deterministic checker and permission-evidence pattern for Phase 91.
- `scripts/check-phase92-address-boundaries.ts` - deterministic checker and address-boundary/no-claim pattern for Phase 92.

### Docs, Evidence, And Release Boundaries

- `docs/architecture/status-snapshot.md` - shared status ownership, inbound status contract, unavailable-field policy, and evidence placement.
- `docs/architecture/operator-observability.md` - status, metrics, logs, support evidence interpretation, and low-cardinality inbound evidence guidance.
- `docs/operator/runtime-guide.md` - repo-local operator command style, opt-in UAT posture, and no-production/no-relay-claim language.
- `docs/parity/catalog/p2p.md` - existing P2P coverage, Phase 90/91/92 evidence, and explicit non-claims for relay and production readiness.
- `docs/parity/release-readiness.md` - deterministic verifier/public-network boundary and deferred-surface wording.
- `docs/parity/checklist.md` - parity checklist roots.
- `docs/parity/index.json` - machine-readable parity root.
- `docs/parity/source-breadcrumbs.json` - required breadcrumb registrations for new first-party Rust sources/tests.

### Knots Anchors

- `packages/bitcoin-knots/src/net.cpp` - peer eviction candidate selection, connection manager disconnect behavior, discouragement hooks, and protected peer handling.
- `packages/bitcoin-knots/src/net_processing.cpp` - misbehavior accounting, protocol violation responses, permission effects, and disconnect/ban interaction.
- `packages/bitcoin-knots/src/banman.h` - ban/discourage data model, ban entries, subnet/address scope, expiry, and unban API anchors.
- `packages/bitcoin-knots/src/banman.cpp` - ban persistence, expiry sweep, ban/unban behavior, and serialization anchors.
- `packages/bitcoin-knots/src/net_permissions.cpp` - `noban` and protected permission behavior that Phase 93 must respect.
- `packages/bitcoin-knots/test/functional/p2p_permissions.py` - permission behavior and protected-peer expectations.
- `packages/bitcoin-knots/test/functional/p2p_invalid_messages.py` - protocol violation and disconnect behavior anchor for misbehavior mapping.
- `packages/bitcoin-knots/test/functional/p2p_dos_header_tree.py` - DoS/misbehavior boundary anchor; use only for scoped violation mapping and leave broader DoS governance to Phase 94.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- `InboundAdmissionPolicy`, `InboundAdmissionRequest`, `InboundAdmissionDecision`, and `InboundPeerRecord` already provide typed peer records and admission outcomes for eviction-policy inputs.
- `PeerPermissionSet::active_effects` already marks `noban` as `EvictionPolicyProtected` and `MisbehaviorPolicyProtected`, and `forceinbound` as admission protected.
- `PeerState` already stores role, handshake state, requested inventory, getaddr request state, and inbound admission metadata that can feed deterministic eviction and misbehavior scoring.
- `PeerAction::Disconnect(DisconnectReason)` and `PeerManager::remove_peer` provide a narrow pure-core disconnect seam.
- `LearnedAddressDecision` and address-boundary evidence from Phase 92 provide typed inputs for repeated invalid address submissions or scoped ban decisions.
- `InboundPeerServingStatus` is the shared status/support projection point for inbound listener, permission, and address evidence; Phase 93 should extend it rather than inventing renderer-local structures.

### Established Patterns

- Pure network policy belongs in `open-bitcoin-network`; runtime I/O, clocks, storage, and process side effects stay in `open-bitcoin-node` or `open-bitcoin-rpc` adapters.
- Shared status owns evidence first, then CLI/status/support renderers format it.
- Stable machine labels are used for diagnostics and release-boundary checkers.
- Default verification avoids public peers, public listener exposure, service-manager operations, sleeps, multi-day timing, and public-network UAT.
- New Rust sources/tests require parity breadcrumbs in file comments and `docs/parity/source-breadcrumbs.json` entries.

### Integration Points

- Add eviction, ban, and misbehavior policy near `open-bitcoin-network/src/peer.rs`, `inbound.rs`, or a new `peer_policy.rs`/`ban.rs` module if that keeps file sizes controlled.
- Add managed projection and any durable state adapter in `open-bitcoin-node` after pure domain types exist.
- Extend `InboundPeerServingStatus`, CLI status, support rendering, and metrics only with low-cardinality, redacted fields.
- Add or extend a deterministic TypeScript checker if docs/parity evidence is updated for Phase 93.
- Update docs/parity/catalog entries and source breadcrumbs when new public evidence or first-party source files are added.

</code_context>

<specifics>
## Specific Ideas

- Suggested stable policy labels include `eviction_candidate_selected`, `eviction_suppressed`, `disconnect_requested`, `discouraged`, `ban_active`, `ban_expired`, `unbanned`, `misbehavior_observed`, `misbehavior_threshold_reached`, `protected_no_action`, and `no_eviction_candidate`.
- Keep scoring explainable: expose a small list of score components rather than a single opaque number when recording latest decision evidence.
- Prefer injected timestamps over sleeps in tests for ban expiry and misbehavior decay.
- Treat ban persistence as scoped evidence, not a production peer-governance claim. If durability lands in this phase, document the store boundary and recovery behavior precisely.
- Keep Phase 94 resource controls out of scope: queue pressure, payload-size governance, slow handshakes, churn limits, and reconnect throttling should be deferred unless needed as typed inputs only.

</specifics>

<deferred>
## Deferred Ideas

- Phase 94 owns broader inbound DoS/resource governance, including queues, payload bounds, timeouts, churn, reconnect behavior, and resource pressure.
- Phase 95 owns v1.9 release-boundary docs and no-claim evidence across inbound serving.
- Future milestones own transaction relay, compact block relay, mempool propagation, BIP37/compact-filter serving, full address relay, public inbound defaults, public-network CI, and production full-node readiness.

</deferred>

---

*Phase: 93-eviction-ban-and-misbehavior-policy*
*Context gathered: 2026-06-26*
