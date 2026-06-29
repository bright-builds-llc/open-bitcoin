---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 100-2026-06-29T16-18-03
generated_at: 2026-06-29T16:18:03.921Z
---

# Phase 100: Relay Activation Boundary and Permission Semantics - Context

**Gathered:** 2026-06-29
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Phase 100 defines the transaction-relay activation and peer-eligibility policy before later v2.0 phases wire transaction inventory, download, mempool admission, serving, fanout, RPC, metrics, logs, or support output. It may introduce typed config, parser, policy, status, docs, and deterministic guardrail tests for default-off relay activation and scoped permission effects.

This phase must not implement broad transaction download, orphan handling, mempool lifecycle, relay serving, rebroadcast, compact block relay, bloom/filter serving, package relay, public relay by default, public-network CI, production service operation, production full-node readiness, or production-funds wallet support.

</domain>

<decisions>
## Implementation Decisions

### Activation Contract

- **D-01:** Transaction relay must remain default-off through an explicit Open Bitcoin-owned activation setting. Default config, default daemon startup, and existing inbound listener enablement must not make Open Bitcoin a public transaction-relay participant.
- **D-02:** Activation should be represented as a typed policy decision, not as scattered boolean checks. The planner should prefer a pure data-in/data-out relay activation module that can be unit tested without socket, RPC, mempool, filesystem, or public-network side effects.
- **D-03:** Service bits must not change in Phase 100. If version-message relay preference is touched, it must be a deliberate output of the relay activation policy with matrix tests proving default-off behavior and no accidental public relay claim.
- **D-04:** Operator-facing naming should stay Open Bitcoin-owned. Reuse existing JSONC and `-openbitcoin...` CLI conventions rather than accepting Knots `whitelist` or `whitebind` compatibility inputs as a shortcut.

### Peer Eligibility Matrix

- **D-05:** Add one explicit eligibility matrix for default config, outbound peers, inbound peers, manual/operator-configured peers, protected slots, and permissioned peers. The matrix should be pure and should emit stable machine labels for eligible, disabled, permission_required, activation_required, protected_not_relay, and permission_effect_inactive-style outcomes.
- **D-06:** Outbound and manual peers can become relay-eligible only after explicit relay activation. Inbound peers require both the existing inbound-serving boundary and an explicit v2.0 relay-eligible permission or class signal; ordinary public inbound peers are not relay-eligible by default.
- **D-07:** Protected admission is not transaction-relay eligibility. Existing `forceinbound` and `noban` effects may protect admission, eviction, or misbehavior policy, but they must not activate transaction relay unless paired with a scoped relay-like permission effect.
- **D-08:** Existing `download` and `addr` permission effects remain their current bounded policy inputs. Phase 100 must not reinterpret them as transaction inventory, mempool query, rebroadcast, or public relay permission.

### Scoped Permission Effects

- **D-09:** Promote `relay`, `forcerelay`, and `mempool` from fully inactive labels into explicit v2.0 relay-permission policy effects, but only as eligibility evidence and policy inputs for later v2.0 transaction relay plans. Phase 100 itself should not mutate mempool state or perform socket relay actions.
- **D-10:** `relay` means peer eligibility for normal transaction inventory/request/send paths once Phase 101+ wires those paths. It does not imply compact blocks, package relay, bloom/filter serving, full address relay, public defaults, or production readiness.
- **D-11:** `forcerelay` implies the scoped `relay` eligibility signal and should be modeled as a separate force-relay policy input for later suppression/bypass rules. It must not become unbounded rebroadcast, package relay, or public propagation by itself.
- **D-12:** `mempool` means eligibility for scoped v2.0 mempool-related peer behavior once later phases own the exact message and serving rules. It must not make the current `mempool` P2P command serve arbitrary transactions in Phase 100.
- **D-13:** `bloomfilter`, `blockfilters`, compact-filter-like behavior, and compact-block behavior remain inactive/deferred. Tests and docs must prove that `all` does not activate these surfaces.

### Evidence, Docs, and Guardrails

- **D-14:** Status/support/log/metric evidence should use low-cardinality labels only. Do not expose raw permission class names, raw permission strings, peer ids, endpoints, transaction ids, raw transaction hex, credentials, or dynamic labels.
- **D-15:** Add deterministic no-claim guardrails for Phase 100 if docs, parity roots, or verifier order are updated. The checker should fail on claims that default public relay, compact block relay, bloom/filter serving, package relay, production service operation, production full-node readiness, production-funds wallet use, or public-network relay CI are supported by Phase 100.
- **D-16:** Verification must stay local and deterministic. Public-network relay review, if documented at all, is opt-in UAT evidence outside `bash scripts/verify.sh`.
- **D-17:** New first-party Rust files under `packages/open-bitcoin-*/src` or tests under `packages/open-bitcoin-*/tests` need source-breadcrumb entries unless the file has a defensible `none` breadcrumb.

### the agent's Discretion

The planner may choose the exact config key names, Rust type names, module split, and status label spelling as long as they preserve the decisions above, stay consistent with existing Open Bitcoin naming, keep pure policy separated from adapters, and keep file/function sizes within repo standards.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Project and v2.0 Scope

- `.planning/PROJECT.md` - Open Bitcoin parity, architecture, dependency, and verification constraints.
- `.planning/REQUIREMENTS.md` - ACT-01 through ACT-04 and the v2.0 deferred/out-of-scope relay boundaries.
- `.planning/ROADMAP.md` - Phase 100 purpose, scope, success criteria, dependencies, and milestone boundaries.
- `.planning/STATE.md` - Current milestone state, recent decisions, pending todos, and local verification caveats.
- `.planning/MILESTONES.md` - v2.0 milestone summary and historical v1.9 boundary context.

### Prior Locked Decisions

- `.planning/phases/90-inbound-listener-and-admission-policy/90-CONTEXT.md` - disabled-by-default inbound listener/admission boundary and relay non-claims.
- `.planning/phases/91-peer-permissions-and-connection-classes/91-CONTEXT.md` - permission vocabulary, active bounded effects, inactive relay-like labels, config surface, status evidence, and redaction rules.
- `.planning/phases/92-address-advertisement-and-discovery-boundaries/92-CONTEXT.md` - permission-aware address-response inputs and full address-relay deferral.
- `.planning/phases/94-dos-and-resource-governance/94-CONTEXT.md` - resource-governance policy stays pure and relay-like permission labels do not raise caps.
- `.planning/phases/95-network-participation-evidence-and-release-boundary/95-CONTEXT.md` - v1.9 no-claim checker strategy and release-boundary wording.
- `.planning/phases/98-traceability-reconciliation/98-CONTEXT.md` - canonical requirement ownership and release-boundary closure pattern.
- `.planning/phases/99-peer-policy-structured-log-emission/99-CONTEXT.md` - low-cardinality structured-log constraints and no raw peer/permission leakage.

### Open Bitcoin Code and Docs

- `packages/open-bitcoin-network/src/inbound/permissions.rs` - current peer permission tokens, active effects, inactive relay-like labels, class matching, and connection classes.
- `packages/open-bitcoin-network/src/inbound.rs` - inbound listener config, preflight policy, admission slot classes, and permission exports.
- `packages/open-bitcoin-network/src/inbound/tests.rs` - current matrix tests for permission parsing, `all` expansion, inactive labels, and permission decisions.
- `packages/open-bitcoin-network/src/peer.rs` - current peer handshake, `wtxidrelay`, inventory, services, and local relay preference surfaces.
- `packages/open-bitcoin-network/src/peer/tests.rs` - current negative tests for deferred relay commands and inactive relay permission labels.
- `packages/open-bitcoin-node/src/network.rs` - managed network wrapper, local services/relay fields, mempool wrapper, and current in-memory transaction paths.
- `packages/open-bitcoin-node/src/network/inbound.rs` - managed inbound permission projection and shared status mapping.
- `packages/open-bitcoin-node/src/status/inbound.rs` - shared inbound status/support contract and permission evidence fields.
- `packages/open-bitcoin-rpc/src/config/loader/inbound.rs` - Open Bitcoin-owned inbound CLI parser pattern.
- `docs/architecture/status-snapshot.md` - shared status ownership and Phase 90/91 inbound permission status boundaries.
- `docs/architecture/operator-observability.md` - low-cardinality metric/log/support constraints for inbound and permission evidence.
- `docs/operator/runtime-guide.md` - repo-local UAT command style and no-production/no-relay claim wording.
- `docs/parity/catalog/p2p.md` - P2P parity catalog, Knots anchors, existing txid/wtxid relay notes, and deferred relay boundaries.
- `docs/parity/index.json` - parity surfaces, audit roots, and existing P2P/mempool traceability.
- `docs/parity/source-breadcrumbs.json` - Rust source breadcrumb registry for first-party files.
- `scripts/check-phase91-peer-permissions.ts` - deterministic checker pattern for permission evidence and no-claim boundaries.
- `scripts/check-phase91-peer-permissions.test.ts` - fixture pattern for failing overclaim text and required evidence labels.
- `scripts/verify.sh` - repo-native verification contract and checker ordering.

### Bitcoin Knots Anchors

- `packages/bitcoin-knots/src/net_permissions.h` - permission flag vocabulary and implied permission anchors.
- `packages/bitcoin-knots/src/net_permissions.cpp` - permission parsing, `all` expansion, and label behavior.
- `packages/bitcoin-knots/src/net.cpp` - peer connection classes, protected peer behavior, service flags, and connection manager context.
- `packages/bitcoin-knots/src/net_processing.cpp` - transaction relay, permission effects, `mempool`, `relay`, `forcerelay`, and P2P processing hazards.
- `packages/bitcoin-knots/src/node/txdownloadman.h` - transaction download manager contract anchor for later v2.0 phases.
- `packages/bitcoin-knots/src/node/txdownloadman_impl.cpp` - transaction download scheduling and relay interaction anchor for later v2.0 phases.
- `packages/bitcoin-knots/src/protocol.h` - inventory and service protocol constants.
- `packages/bitcoin-knots/test/functional/p2p_permissions.py` - permission behavior and protected peer expectations.
- `packages/bitcoin-knots/test/functional/p2p_tx_download.py` - transaction download behavior to avoid pre-implementing in Phase 100.
- `packages/bitcoin-knots/test/functional/p2p_getdata.py` - getdata behavior to defer to later phases except as a policy hazard.
- `packages/bitcoin-knots/test/functional/mempool_accept.py` - mempool admission behavior to defer to later phases except as a policy hazard.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- `PeerPermissionToken`, `PeerPermissionSet`, `PermissionEffectLabel`, and `InactivePermissionEffectLabel` in `packages/open-bitcoin-network/src/inbound/permissions.rs` already provide the Knots-anchored vocabulary and current active/inactive split.
- `PeerConnectionClass` already models `ordinary_inbound`, `permissioned_inbound`, `protected_inbound`, `outbound`, and `manual_configured` labels that can seed the Phase 100 eligibility matrix.
- `InboundPermissionDecision` and `PeerPermissionClassRegistry::resolve_inbound` already provide a pure permission-decision seam for inbound peers.
- `InboundPeerServingStatus` already centralizes permission evidence for status, support, RPC, CLI, dashboard, metrics, and logs.
- Existing Phase 91 checker/tests show how to guard docs and parity roots against relay/public-default overclaims.

### Established Patterns

- Pure network policy belongs in `open-bitcoin-network`; managed runtime projection belongs in `open-bitcoin-node`; config parsing belongs in `open-bitcoin-rpc`; operator rendering belongs in `open-bitcoin-cli`.
- Permission evidence is low-cardinality and redacted. Raw permission class names, raw config strings, endpoints, peer ids, and credentials do not belong in shared status or support bundles.
- Deterministic checkers are Bun/TypeScript scripts wired through `scripts/verify.sh` and fixed-file reads, with fixture tests for both allowed and forbidden wording.
- Public-network, service-manager, long-running, and production-deployment checks stay out of default verification.

### Integration Points

- Add the new relay activation policy near the existing inbound permission model or a sibling `relay` module if that keeps file size and responsibility clear.
- Add config parsing through Open Bitcoin-owned JSONC and CLI surfaces, following `parse_inbound_cli_arg` conventions.
- Project any Phase 100 evidence through shared status contracts only if the implementation needs operator-visible activation state in this phase.
- Update parity docs and source breadcrumbs only for files actually touched by Phase 100 plans.

</code_context>

<specifics>
## Specific Ideas

- Use stable machine labels for policy outcomes so later phases can branch on typed results without parsing prose.
- Treat `all` as a regression hotspot: tests must prove it does not activate bloom/filter, compact block, package relay, public defaults, or production claims.
- Keep the first implementation narrow: config parsing, pure policy, tests, docs, and guardrails. Transaction message handling and mempool mutation belong to later v2.0 phases.
- Preserve the repo-local UAT command lesson if any operator docs are updated: use explicit Cargo and Bazel command forms, not a bare `open-bitcoin` alias.

</specifics>

<deferred>
## Deferred Ideas

Transaction inventory identity, download scheduling, orphan handling, mempool admission, mempool lifecycle, relay serving, fanout, rebroadcast, RPC/mempool surfaces, support redaction for transaction material, parity closeout, compact block relay, bloom/filter serving, package relay, public relay defaults, public-network relay CI, production service operation, production full-node readiness, and production-funds wallet use all remain outside Phase 100.

</deferred>

***

*Phase: 100-relay-activation-boundary-and-permission-semantics*
*Context gathered: 2026-06-29*
