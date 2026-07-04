---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 111-2026-07-04T14-58-18
generated_at: 2026-07-04T14:58:18.000Z
---

# Phase 111: Full Block Serving Request Path - Context

**Gathered:** 2026-07-04
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Phase 111 adds the node-shell path that serves eligible full and witness block `getdata` requests from validated local block data. The request path must consume the Phase 110 activation, peer eligibility, block status, resource-governance, in-flight cleanup, and sanitized evidence contracts before any storage read or socket response.

This phase may wire eligible full/witness block inventory requests to locally available validated blocks, return deterministic missing/unavailable/suppressed evidence for ineligible or unavailable requests, and add focused tests for request caps, queue pressure, historical/pruned boundaries, and peer cleanup.

This phase must not implement BIP152 compact block serialization, compact-block reconstruction, `getblocktxn`, `blocktxn`, missing compact transaction round trips, package relay, bloom/filter serving, compact filter serving, public serving defaults, public-network CI gates, archive-node behavior, production-service operation, production full-node readiness, or production-funds wallet use. `InventoryType::CompactBlock` requests may be classified and bounded, but actual compact-block payload serving belongs to later compact-relay phases.

</domain>

<decisions>
## Implementation Decisions

### Request Routing

- **D-01:** Full and witness block `getdata` requests must remain inside the existing peer-manager request-pressure path before any node-shell storage lookup. Over-cap `getdata` bursts should keep producing deterministic resource-governance disconnect or suppression behavior instead of falling through to serving.
- **D-02:** The node-shell serving adapter must call the Phase 110 block-serving eligibility, status, and resource gate before reading `blocks_by_hash`, chainstate-backed block data, or any future block-store abstraction. `blocks_by_hash` must not become the serving policy by itself.
- **D-03:** `InventoryType::Block` and `InventoryType::WitnessBlock` are the only inventory types that may produce `WireNetworkMessage::Block` in this phase. Transaction inventory must continue using the existing transaction relay serving cache, and unknown inventory must continue to be missing/suppressed.
- **D-04:** `InventoryType::CompactBlock` requests must be bounded and classified in this phase but must not produce compact-block responses. Treat them as deterministic suppressed/unavailable/deferred outcomes until Phase 112+ owns BIP152 wire semantics.

### Local Block Availability

- **D-05:** Serving requires all three facts: peer eligible, status `Available`, and local validated block data present. Missing any one of those facts returns missing/unavailable/suppressed evidence without serving a block.
- **D-06:** Active-chain and explicit recent-valid blocks are the only positive serving classes. Stale, side-chain, unvalidated, unknown, pruned, unavailable, and suppressed classifications must not attempt optimistic reads or responses.
- **D-07:** The implementation may start from the current `ManagedPeerNetwork` block cache path, but the plan should isolate a named block-serving adapter seam so future durable block storage can replace or extend the cache without changing the policy boundary.
- **D-08:** Witness block requests may reuse the existing `WireNetworkMessage::Block` serialization only if the current block codec preserves witness transaction data for the block value being served. If the existing codec cannot prove witness preservation, the plan must add a focused regression before claiming witness block serving.

### Resource Governance And Cleanup

- **D-09:** Full block serving must participate in existing queue, request, and in-flight limits from `ResourceGovernancePolicy`, including per-peer and aggregate requested-block counters.
- **D-10:** The request path must release or preserve in-flight block state through existing received block, `notfound`, peer disconnect, timeout, and runtime restart cleanup paths. Cleanup evidence should use the Phase 110 block in-flight labels instead of inventing renderer-local labels.
- **D-11:** Permissioned and protected peers remain bounded. A scoped download/block-serving permission can make a peer eligible only when activation and inbound serving facts also permit it; it must not bypass request caps or grant archive-node behavior.

### Historical And Pruned Boundaries

- **D-12:** Historical and pruned requests must be truthful but bounded. The result should identify stable low-cardinality outcomes such as pruned, unavailable, stale, side-chain, unknown, suppressed, or request-cap reached, without exposing prune heights, raw peer endpoints, raw permission strings, credentials, or block/transaction payload details.
- **D-13:** The phase must preserve the "bounded block serving, not archive-node availability" claim in docs, parity artifacts, tests, and verifier output. A local cache hit for an old block must not become a broad historical-serving guarantee.
- **D-14:** Public-network review stays opt-in UAT guidance only. Default local verification should prove request routing, resource limits, cleanup, and no-claim boundaries with deterministic unit/integration tests and checker scripts.

### Evidence And Guardrails

- **D-15:** Operator-facing evidence created or extended in this phase should flow through shared status/evidence contracts before CLI, dashboard, RPC, metrics, logs, or support renderers format it.
- **D-16:** If docs, parity files, release boundaries, or verifier wiring change, add a deterministic Phase 111 checker modeled on Phase 110. The checker should require the new request-path evidence while rejecting positive claims for compact relay, package relay, public defaults, archive-node behavior, public-network CI, production readiness, production service operation, and production-funds wallet use.
- **D-17:** New or touched first-party Rust source/test files under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests` need parity breadcrumbs in file comments and `docs/parity/source-breadcrumbs.json`, using `none` only when no defensible Knots anchor exists.

### the agent's Discretion

The planner may choose exact type names, helper boundaries, test fixture names, and whether the Phase 111 checker is a new script or a scoped extension of the Phase 110 checker. Prefer the smallest adapter surface that keeps policy pure, keeps storage/socket effects in the node shell, and leaves compact-block relay, reconstruction, and fallback to their later phases.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Project And Phase Scope

- `AGENTS.md` - repo-local verification, GSD workflow, parity breadcrumb, Rust, Bright Builds, and repo-local UAT command guidance.
- `AGENTS.bright-builds.md` - Bright Builds sync, verification, testing, architecture, and task artifact rules.
- `standards/core/architecture.md` - functional core / imperative shell and domain-type rules.
- `standards/core/code-shape.md` - early-return, optional-name, script, and file/function shape rules.
- `standards/core/testing.md` - unit test behavior and Arrange/Act/Assert requirements.
- `standards/core/verification.md` - repo-native verification and commit gate expectations.
- `standards/languages/rust.md` - Rust module, optional naming, invariant, and verification guidance.
- `standards/languages/typescript-javascript.md` - Bun/TypeScript script guidance for deterministic checker work.
- `.planning/PROJECT.md` - active v2.1 scope, parity value, architecture constraints, and deferred production/public-serving claims.
- `.planning/REQUIREMENTS.md` - BSRV-04, GOV-01, and GOV-05 ownership for Phase 111.
- `.planning/ROADMAP.md` - Phase 111 goal, success criteria, requirement mapping, and milestone boundaries.
- `.planning/STATE.md` - current milestone state, v2.1 pending notes, local verification caveats, and repo-local UAT command reminders.

### Prior Locked Decisions

- `.planning/phases/110-block-serving-activation-and-eligibility-boundary/110-CONTEXT.md` - activation, eligibility, status, resource, evidence, and no-claim decisions that Phase 111 must consume.
- `.planning/phases/110-block-serving-activation-and-eligibility-boundary/110-VERIFICATION.md` - Phase 110 passed scope and the explicit list of behavior deferred to Phase 111+.
- `.planning/phases/110-block-serving-activation-and-eligibility-boundary/110-03-SUMMARY.md` - resource-governance and in-flight cleanup outcomes available for Phase 111.
- `.planning/phases/110-block-serving-activation-and-eligibility-boundary/110-04-SUMMARY.md` - docs, parity, and no-claim checker pattern to extend if Phase 111 changes those surfaces.
- `.planning/phases/94-dos-and-resource-governance/94-CONTEXT.md` - pure resource-governance policy, stable labels, request caps, timeout/churn inputs, and no relay side effects.
- `.planning/phases/100-relay-activation-boundary-and-permission-semantics/100-CONTEXT.md` - default-off activation model, peer eligibility matrix, scoped permission effects, low-cardinality evidence, and no-claim guardrails.
- `.planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-CONTEXT.md` - existing relay serving/fanout pattern to mirror only where it fits full block serving.

### Existing Code Integration Points

- `packages/open-bitcoin-network/src/block_serving.rs` - Phase 110 pure activation, eligibility, status, resource-gate, and cleanup contracts to consume before serving.
- `packages/open-bitcoin-network/src/block_serving/tests.rs` - Phase 110 unit-test style and labels that Phase 111 should preserve.
- `packages/open-bitcoin-network/src/peer/inventory_state.rs` - current `getdata`, request-pressure, block in-flight, `notfound`, and received-block paths.
- `packages/open-bitcoin-network/src/peer/tests.rs` - peer-manager request cap and Phase 110 block request regression coverage.
- `packages/open-bitcoin-node/src/network/inventory.rs` - current node-shell `serve_inventory` adapter that serves cached blocks and transactions.
- `packages/open-bitcoin-node/src/network.rs` - managed network runtime and `PeerAction::ServeInventory`/block receive integration points.
- `packages/open-bitcoin-node/src/network/relay_serving.rs` - existing transaction serving cache/status pattern to use as a reference, not an overloaded block-serving API.
- `packages/open-bitcoin-node/src/status/block_serving.rs` - shared sanitized block-serving evidence contract.
- `packages/open-bitcoin-node/src/sync/tests.rs` - Phase 110 durable-sync in-flight cap and cleanup regression coverage.
- `packages/open-bitcoin-primitives/src/network.rs` - inventory types for block, compact block, and witness block plus message command primitives.
- `packages/open-bitcoin-network/src/message.rs` - `WireNetworkMessage::Block` encoding/decoding surface.
- `packages/open-bitcoin-codec/src/block.rs` - block and witness-preserving block codec surface.
- `packages/open-bitcoin-chainstate/src/engine.rs` - validated block/connect state and active-chain concepts that serving must not mutate.
- `docs/parity/source-breadcrumbs.json` - required breadcrumb registry for new or touched first-party Rust files.
- `scripts/check-phase110-block-serving-boundary.ts` - deterministic checker pattern for Phase 111 docs/parity/verifier guardrails.
- `scripts/check-phase110-block-serving-boundary.test.ts` - mutation-test pattern for required evidence and forbidden claims.
- `scripts/verify.sh` - repo-native verification contract and checker ordering.

### Docs, Parity, And Release Boundaries

- `docs/architecture/status-snapshot.md` - shared status ownership and unavailable-field policy.
- `docs/architecture/operator-observability.md` - low-cardinality status, metrics, logs, and support evidence constraints.
- `docs/operator/runtime-guide.md` - repo-local operator command style and opt-in UAT posture.
- `docs/parity/catalog/p2p.md` - P2P parity catalog and deferred relay/block-serving boundary notes.
- `docs/parity/index.json` - machine-readable parity surface ownership.
- `docs/parity/checklist.md` - parity checklist roots.
- `docs/parity/release-readiness.md` - deterministic verifier/public-network boundary and deferred-surface wording.

### Bitcoin Knots Anchors

- `packages/bitcoin-knots/src/net_processing.cpp` - block `getdata`, block serving, compact-block deferral hazards, request bounds, peer state, and DoS response boundaries.
- `packages/bitcoin-knots/src/protocol.h` - inventory constants and message command names for block, witness block, and compact block inventory.
- `packages/bitcoin-knots/src/net.cpp` - peer connection classes, protected peer behavior, upload/resource policy, and connection manager context.
- `packages/bitcoin-knots/src/net_permissions.h` - permission flag vocabulary and download/relay permission anchors.
- `packages/bitcoin-knots/src/net_permissions.cpp` - permission parsing, `all` expansion, and label behavior.
- `packages/bitcoin-knots/src/validation.cpp` - active-chain, validated block, side-chain, and block-availability anchors.
- `packages/bitcoin-knots/src/node/blockstorage.cpp` - block file availability and pruned/unavailable block anchors.
- `packages/bitcoin-knots/test/functional/p2p_getdata.py` - full block and witness block `getdata` behavior and request boundary anchor.
- `packages/bitcoin-knots/test/functional/p2p_permissions.py` - permission and protected peer behavior expectations.
- `packages/bitcoin-knots/test/functional/p2p_compactblocks.py` - compact-block request behavior to defer beyond Phase 111 except for bounded suppress/unavailable handling.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- `classify_block_serving_eligibility`, `classify_block_serving_status`, `evaluate_block_serving_resource_gate`, and `classify_block_inflight_cleanup` in `open-bitcoin-network/src/block_serving.rs` already express the required pure gate sequence.
- `PeerManager::handle_getdata`, `request_pressure_input`, `resource_limit_disconnect_actions_from_decision`, `handle_notfound`, and `handle_block` in `open-bitcoin-network/src/peer/inventory_state.rs` already carry the request and cleanup hooks for block inventory.
- `ManagedPeerNetwork::serve_inventory` in `open-bitcoin-node/src/network/inventory.rs` is the current effectful serving adapter; it must be tightened to consume Phase 110 decisions before block reads.
- `RelayServingCache` in `open-bitcoin-node/src/network/relay_serving.rs` shows how serving outcomes can be cached and exposed without leaking raw peer or payload detail.
- `BlockServingEvidenceStatus` in `open-bitcoin-node/src/status/block_serving.rs` is the shared sanitized status contract available for evidence projection.

### Established Patterns

- Pure network policy belongs in `open-bitcoin-network`; managed runtime, durable storage, clocks, sockets, logs, and process effects stay in node/RPC/CLI adapters.
- Existing transaction relay serving already separates peer-mode/eligibility classification from the actual transaction lookup; Phase 111 should do the same for block serving rather than duplicating policy inside the adapter.
- Deterministic checker scripts are Bun/TypeScript and fixed-file based; public-network UAT remains opt-in and outside default verification.
- New Rust source/test files need parity breadcrumbs and Arrange/Act/Assert-style tests when non-trivial.

### Integration Points

- Route block and witness block requests through the existing `PeerAction::ServeInventory` path only after peer-manager pressure checks and node-shell Phase 110 gates pass.
- Add a named block-serving adapter/cache seam in `open-bitcoin-node` if direct changes to `network/inventory.rs` would make storage reads, policy decisions, and evidence projection hard to test separately.
- Extend docs/parity/source breadcrumbs and deterministic no-claim checks only for files and docs actually touched by Phase 111 plans.

</code_context>

<specifics>
## Specific Ideas

- Treat compact-block inventory as "bounded and classified, not served" in this phase.
- Preserve existing transaction relay behavior when mixed `getdata` requests include both block and transaction inventory.
- Keep exact Cargo and Bazel command forms in any operator/UAT docs touched by this phase:
  - `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- ...`
  - `bazel run //packages/open-bitcoin-cli:open_bitcoin -- ...`
- Prefer policy outputs and focused tests over broad docs claims. Docs should say what Phase 111 proves and what remains deferred.

</specifics>

<deferred>
## Deferred Ideas

BIP152 wire codecs, `sendcmpct`, compact-block response payloads, compact-block reconstruction, `getblocktxn`, `blocktxn`, missing compact transaction round trips, fallback/validation handoff, broad operator evidence rollout, package relay, bloom/filter serving, compact filter serving, public serving defaults, public-network CI, archive-node claims, production full-node readiness, production-service operation, and production-funds wallet use remain outside Phase 111.

</deferred>

***

*Phase: 111-full-block-serving-request-path*
*Context gathered: 2026-07-04*
