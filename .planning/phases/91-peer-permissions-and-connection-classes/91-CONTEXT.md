---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 91-2026-06-25T13-36-41
generated_at: 2026-06-25T13:36:55.195Z
---

# Phase 91: Peer Permissions and Connection Classes - Context

**Gathered:** 2026-06-25
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Phase 91 models Knots-aligned peer permission concepts and connection classes for Open Bitcoin inbound serving. It should extend the Phase 90 listener/admission surface with explicit permission parsing, typed connection classes, bounded permission effects, and shared operator evidence.

This phase may make permissions observable in admission protection, eviction-policy inputs, address-response policy inputs, download-serving policy inputs, and diagnostics. It must not enable transaction relay, compact block relay, mempool propagation, force-relay behavior, full address relay, broad ban/misbehavior policy, public inbound defaults, or production full-node readiness.

</domain>

<decisions>
## Implementation Decisions

### Permission Vocabulary And Parsing

- **D-01:** Use Knots permission names as the vocabulary anchor: `bloomfilter`, `blockfilters`, `noban`, `forcerelay`, `relay`, `mempool`, `download`, `addr`, `forceinbound`, `in`, `out`, and `all`.
- **D-02:** Open Bitcoin-owned config remains the entry point. Add permissions under the existing JSONC-owned `inbound` surface and Open Bitcoin-prefixed daemon CLI overrides; do not silently accept Knots `-whitelist` or `-whitebind` as full baseline compatibility.
- **D-03:** Parsing must be explicit and stable. Unsupported tokens, direction-only inputs, invalid connection direction combinations, and malformed class definitions return deterministic validation errors that name the offending field and token.
- **D-04:** Permission bundles must be typed, not plain strings. Represent parsed flags, connection direction boundaries, and effective class names with domain types so illegal states are hard to construct after parsing.
- **D-05:** `all` should expand only to an auditable parsed set. Deferred or inactive permissions remain visibly inactive; `all` must not smuggle in relay, mempool, force-relay, compact-filter, or broad block-filter behavior.

### Connection Classes And Admission Effects

- **D-06:** Introduce explicit connection classes such as ordinary inbound, permissioned inbound, protected inbound, outbound, and manual/operator configured peers as needed by the Phase 91 plan. Class names should be stable machine labels suitable for status and support evidence.
- **D-07:** `forceinbound` and `noban`-style protections may influence admission protection or eviction-candidate inputs, but only through bounded pure decisions. They should not mutate peer state hidden inside the runtime accept loop.
- **D-08:** Permissioned peers can consume the reserved admission path created in Phase 90. The planner may replace or extend `InboundAdmissionSlotClass::Reserved`, but must preserve ordinary peers being unable to consume protected capacity.
- **D-09:** Outbound sync safety remains non-negotiable. Permissioned inbound peers must not reduce `target_outbound_peers`, starve outbound sync, or count as outbound compatibility progress.

### Bounded Permission Effects

- **D-10:** v1.9 active permission effects are bounded to admission protection, eviction-policy inputs, address-response policy inputs, download-serving policy inputs, and diagnostics.
- **D-11:** `download` may influence block/header serving or max-upload-style decisions only as a policy input. It must not create a new unattended block-serving claim unless a plan adds explicit bounded tests and documentation.
- **D-12:** `addr` may influence the later Phase 92 bounded `getaddr` response policy, but Phase 91 should expose only typed policy inputs and diagnostics unless it is needed to support a Phase 91 test seam.
- **D-13:** `noban` may mark a peer as protected from eviction/ban/misbehavior responses, but Phase 93 owns actual ban, discourage, disconnect, and misbehavior semantics.
- **D-14:** `relay`, `forcerelay`, `mempool`, `bloomfilter`, `blockfilters`, and compact-filter-like permissions must be rejected, deferred, or parsed as inactive for now. They cannot initialize tx relay state, mempool query handling, force-relay rebroadcasts, compact block relay, BIP37 filtering, or compact-filter serving.

### Operator Evidence And Redaction

- **D-15:** Permission status must project through shared status/support contracts rather than renderer-local summaries. Extend `OpenBitcoinStatusSnapshot.peers.inbound` or a closely owned child contract with low-cardinality labels for permission class, active bounded effects, inactive/deferred effects, and latest permission decision.
- **D-16:** Support bundles may include bounded permission evidence and reasons, but must not leak secrets, raw config values beyond safe labels, raw peer tables, unbounded endpoints, or credential material.
- **D-17:** Structured logs and metrics should stay low-cardinality. Numeric counters may cover permissioned admits, protected peers, inactive relay-like permissions, and permission validation failures; labels must not include peer ids, raw endpoints, user labels, or raw permission config strings.

### Verification And UAT

- **D-18:** Default verification stays deterministic, local, and public-network-free. Use pure parser/policy tests, synthetic peer records, and existing Phase 90 inbound fixtures rather than public peers.
- **D-19:** Tests should prove both positive bounded effects and negative relay safeguards. At minimum, include cases for stable labels, explicit parse errors, `all` expansion boundaries, reserved/protected admission behavior, inactive relay/mempool/force-relay/compact-filter effects, status/support redaction, and no outbound sync starvation.
- **D-20:** Operator UAT docs must use repo-local Cargo and Bazel forms when commands are needed, matching the repo lesson and `AGENTS.md` guidance.

### the agent's Discretion

The planner may choose exact type names and module splits. Prefer pure parsing and policy modules in `open-bitcoin-network`, thin projection/wiring in `open-bitcoin-node`, config parsing in `open-bitcoin-rpc`, and renderer-only formatting in CLI/support modules. Keep the first implementation narrow enough to satisfy PERM-01 through PERM-04 without pulling Phase 92 or Phase 93 behavior forward.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Project And Workflow Rules

- `AGENTS.md` - repo-local verification, parity breadcrumb, GSD workflow, and UAT command rules.
- `AGENTS.bright-builds.md` - Bright Builds sync, verification, testing, architecture, and task artifact rules.
- `standards/core/architecture.md` - functional core / imperative shell and domain-type rules.
- `standards/core/code-shape.md` - early-return, optional-name, and file/function shape rules.
- `standards/core/testing.md` - unit test behavior and Arrange/Act/Assert requirements.
- `standards/core/verification.md` - repo-native verification and commit gate expectations.
- `standards/languages/rust.md` - Rust module, optional naming, invariant, and verification guidance.

### Phase Scope And Requirements

- `.planning/PROJECT.md` - active v1.9 inbound-serving scope, deferred relay/production boundaries, and Knots anchor expectations.
- `.planning/REQUIREMENTS.md` - PERM-01 through PERM-04 plus v1.9 future/out-of-scope relay and production boundaries.
- `.planning/ROADMAP.md` - Phase 91 goal, success criteria, and requirement mapping.
- `.planning/STATE.md` - current milestone position and pending v1.9 workflow notes.
- `.planning/phases/90-inbound-listener-and-admission-policy/90-CONTEXT.md` - locked Phase 90 inbound listener/admission decisions that Phase 91 must extend, not reopen.

### Existing Code Integration Points

- `packages/open-bitcoin-network/src/inbound.rs` - Phase 90 listener preflight, admission policy, slot classes, handshake state, peer records, and stable rejection labels.
- `packages/open-bitcoin-network/src/inbound/tests.rs` - pure inbound preflight/admission test style with stable-label coverage.
- `packages/open-bitcoin-network/src/peer.rs` - pure peer lifecycle, inbound/outbound roles, peer state, message actions, inventory serving, and tx/block serving boundaries.
- `packages/open-bitcoin-network/src/peer/inbound_state.rs` - inbound endpoint keys, counters, active inbound records, and self-connection rejection.
- `packages/open-bitcoin-node/src/network.rs` - `ManagedPeerNetwork`, inbound policy storage, inbound/outbound count projection, mempool and block handling.
- `packages/open-bitcoin-node/src/network/inbound.rs` - node-side admission info, `set_inbound_admission_policy`, and admission recording.
- `packages/open-bitcoin-node/src/status/inbound.rs` - shared inbound serving status contract to extend with bounded permission evidence.
- `packages/open-bitcoin-node/src/status/inbound/tests.rs` - status serialization and legacy-default tests.
- `packages/open-bitcoin-rpc/src/config/loader/inbound.rs` - Open Bitcoin-owned inbound CLI flag parser to extend with permission-class inputs.
- `packages/open-bitcoin-rpc/src/config/open_bitcoin.rs` - JSONC-owned config shape and inbound settings precedent.
- `packages/open-bitcoin-rpc/src/context/network.rs` - RPC context network wrapper and inbound admission access points.
- `packages/open-bitcoin-rpc/src/method/node.rs` - `getnetworkinfo` baseline response shape and peer counts.
- `packages/open-bitcoin-cli/src/operator/support/render/inbound.rs` - support Markdown projection for bounded inbound evidence.
- `packages/open-bitcoin-cli/src/operator/status/render/inbound.rs` - human status projection for inbound evidence.
- `packages/open-bitcoin-node/src/metrics.rs` - low-cardinality metric naming and redaction constraints.
- `scripts/check-phase90-inbound-listener-admission.ts` - Phase 90 deterministic evidence checker and deferred-surface guardrail pattern.

### Docs, Evidence, And Release Boundaries

- `docs/architecture/config-precedence.md` - Open Bitcoin JSONC ownership, CLI precedence, inbound config boundary, and invalid `bitcoin.conf` key policy.
- `docs/architecture/status-snapshot.md` - shared status ownership, Phase 90 inbound contract, unavailable-field policy, and deferred permission-class boundary.
- `docs/architecture/operator-observability.md` - status, metrics, logs, and support evidence interpretation and low-cardinality inbound evidence guidance.
- `docs/operator/runtime-guide.md` - repo-local operator command style, opt-in UAT posture, and no-production-claim language.
- `docs/parity/catalog/p2p.md` - existing P2P coverage, Phase 90 evidence, and explicit non-claims for relay and production readiness.
- `docs/parity/release-readiness.md` - deterministic verifier/public-network boundary and deferred-surface wording.
- `docs/parity/checklist.md` - parity checklist roots.
- `docs/parity/index.json` - machine-readable parity root.
- `docs/parity/source-breadcrumbs.json` - required breadcrumb registrations for new first-party Rust sources/tests.

### Knots Anchors

- `packages/bitcoin-knots/src/net_permissions.h` - `NetPermissionFlags`, implied flags, defaults, and permission class declarations.
- `packages/bitcoin-knots/src/net_permissions.cpp` - permission parsing, direction handling, invalid-token errors, `all` expansion, `ToStrings`, whitelist, and whitebind parsing.
- `packages/bitcoin-knots/test/functional/p2p_permissions.py` - expected permission labels, legacy interactions, invalid input errors, and relay behavior that Open Bitcoin must explicitly defer where out of scope.
- `packages/bitcoin-knots/src/net.cpp` - connection manager permission use, inbound slot protection, and permission-aware connection behavior.
- `packages/bitcoin-knots/src/net_processing.cpp` - permission effects for download, noban, addr, mempool, relay, and forcerelay; use as parity anchor and as a list of deferred relay hazards.
- `packages/bitcoin-knots/src/addrman.cpp` - address-management anchor for Phase 92; cite only to keep `addr` bounded.
- `packages/bitcoin-knots/src/banman.cpp` - ban/discourage anchor for Phase 93; cite only to keep `noban` bounded.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- `InboundAdmissionPolicy`, `InboundAdmissionRequest`, `InboundAdmissionDecision`, and `InboundAdmissionSlotClass` provide the pure policy seam for permission-aware admission.
- `ManagedPeerNetwork::set_inbound_admission_policy` and `admit_inbound_peer` already wire pure admission decisions into node state.
- `InboundPeerServingStatus` and `InboundAdmissionEvent` are the shared status/support projection point for new permission evidence.
- `parse_inbound_cli_arg` already owns Open Bitcoin-prefixed inbound daemon flags and can be extended without accepting baseline `bitcoin.conf` peer-permission options.
- Existing support/status renderers already render bounded inbound evidence and next actions.

### Established Patterns

- Keep parsing and decisions in pure Rust types before runtime socket or peer-manager side effects.
- Preserve baseline-shaped `getnetworkinfo` fields for peer counts while Open Bitcoin-specific evidence lives in Open Bitcoin-owned status/RPC/support surfaces.
- Public network behavior and broad participation claims stay outside `bash scripts/verify.sh`.
- New first-party Rust source or test files under `packages/open-bitcoin-*/src` or `tests` require parity breadcrumbs in source comments and `docs/parity/source-breadcrumbs.json`.
- Tests in this repo use explicit Arrange, Act, Assert comments for non-trivial Rust unit tests.

### Integration Points

- Add permission domain types near `packages/open-bitcoin-network/src/inbound.rs` or in a new sibling module if the file would become too large.
- Extend node-side admission and status projection in `packages/open-bitcoin-node/src/network/inbound.rs` and `packages/open-bitcoin-node/src/status/inbound.rs`.
- Extend config parsing under `packages/open-bitcoin-rpc/src/config/` and Open Bitcoin JSONC config shape under `config/open_bitcoin.rs`.
- Extend CLI status/support renderers only after shared status fields exist.
- Add a deterministic TypeScript checker if docs, parity catalogs, or release-boundary claims need machine-readable guardrails.

</code_context>

<specifics>
## Specific Ideas

- Treat `relay`, `forcerelay`, `mempool`, `bloomfilter`, `blockfilters`, and compact-filter aliases as explicit "parsed but inactive" or rejected permissions with stable reasons, so users see why they do not do anything yet.
- Keep `download`, `addr`, `noban`, and `forceinbound` as the useful v1.9 bounded effects, but only as inputs to scoped policies and diagnostics.
- Expose permission decisions with stable labels such as `ordinary`, `permissioned`, `protected`, `inactive_relay`, `inactive_mempool`, `inactive_forcerelay`, `inactive_blockfilters`, and `protected_from_eviction_policy`.
- If `all` is supported, tests must prove that relay-like flags do not enable relay behavior and appear as inactive/deferred evidence.

</specifics>

<deferred>
## Deferred Ideas

- Phase 92 owns local address advertisement, bounded `getaddr` response behavior, and address-management contracts.
- Phase 93 owns actual eviction, disconnect, discourage, ban, expiry, unban, and misbehavior behavior.
- Phase 94 owns broader inbound DoS/resource governance beyond Phase 90 caps and Phase 91 permission inputs.
- Phase 95 owns v1.9 release-boundary docs and no-claim evidence across inbound serving.
- Future milestones own transaction relay, compact block relay, mempool propagation, BIP37/compact-filter serving, full address relay, public inbound defaults, public-network CI, and production full-node readiness.

</deferred>

---

*Phase: 91-peer-permissions-and-connection-classes*
*Context gathered: 2026-06-25*
