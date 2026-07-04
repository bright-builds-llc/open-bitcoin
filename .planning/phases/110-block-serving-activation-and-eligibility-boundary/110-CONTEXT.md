---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 110-2026-07-04T02-39-48
generated_at: 2026-07-04T02:39:48.367Z
---

# Phase 110: Block Serving Activation and Eligibility Boundary - Context

**Gathered:** 2026-07-04
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Phase 110 creates the pure activation, peer eligibility, block status, and resource-governance boundary that every v2.1 block-serving effect must pass before storage reads or socket responses. It may introduce Open Bitcoin-owned config, typed policy, status labels, deterministic tests, parity breadcrumbs, docs, and guardrails for default-off block serving and compact-relay activation.

This phase must not serve full blocks, encode or decode BIP152 messages, reconstruct compact blocks, request missing compact-block transactions, mutate chainstate from compact-block state, claim archive-node behavior, enable public serving by default, enable package relay or filter serving, add public-network CI gates, or claim production full-node readiness.

</domain>

<decisions>
## Implementation Decisions

### Activation Contract

- **D-01:** Block serving and compact-block relay must stay default-off through explicit Open Bitcoin-owned activation settings. Default daemon startup, default inbound serving, and existing transaction-relay activation must not make the node a public block-serving participant.
- **D-02:** Model block-serving activation as a typed pure policy decision, not scattered booleans in node runtime code. The policy should be unit-testable without sockets, durable storage, RPC, filesystem, public-network peers, or service-manager effects.
- **D-03:** Keep block-serving activation separate from transaction relay activation. Existing `RelayActivationConfig` and status patterns are reusable design references, but the planner should avoid overloading transaction-relay types when a block-serving-specific type prevents ambiguous states.
- **D-04:** Service bits, public defaults, inbound listener defaults, and transaction-relay behavior must not change in this phase. Any version-message or service-advertisement output must be an explicit policy output with matrix tests proving no accidental public-serving claim.

### Peer Eligibility Matrix

- **D-05:** Add one explicit block-serving eligibility matrix for outbound, inbound, manual/operator-configured, protected, and permissioned peers. The matrix should emit stable machine labels for eligible, disabled, activation_required, inbound_serving_required, permission_required, protected_not_serving, status_unavailable, and permission_effect_inactive-style outcomes.
- **D-06:** Outbound and manual peers may become block-serving-eligible only after explicit block-serving activation. Ordinary inbound peers remain ineligible by default. Permissioned inbound peers require inbound serving plus a scoped block-serving/download-style permission input before later phases may read or send block data.
- **D-07:** Protected admission is not block-serving eligibility. Existing `forceinbound` and `noban` effects may protect admission, eviction, or misbehavior policy, but they must not activate block serving or compact relay by themselves.
- **D-08:** The existing `download` permission may be a policy input for bounded block-serving eligibility, but it must not imply archive-node behavior, unbounded historical serving, compact-block relay, transaction relay, package relay, bloom filters, compact filters, or production readiness.

### Block Status Classification

- **D-09:** Introduce a pure block-serving status classifier before any storage read or socket response. It should distinguish validated, available, stale, side-chain, pruned, unavailable, unvalidated, unknown, and suppressed outcomes with stable typed labels.
- **D-10:** The classifier should accept current chain/header/block facts as data and return a decision that later adapters can consume. It should not perform durable storage reads, mutate chainstate, touch mempool state, or inspect runtime sockets directly.
- **D-11:** Pruned, unavailable, side-chain, and stale outcomes must be truthful but sanitized. Operator evidence and support bundles may expose stable labels and aggregate counters, but not prune-height details, raw peer endpoints, raw permission strings, credentials, dynamic labels, or raw block/transaction payloads.
- **D-12:** Classification should keep the v2.1 claim bounded to validated and available blocks inside the documented active-chain or recent-valid boundary. Unknown or unvalidated data must not be served optimistically.

### Resource Governance

- **D-13:** Full block serving and compact-relay activation gates must participate in the existing Phase 94 resource-governance model before later phases add serving effects. Request caps, backpressure, timeouts, churn, ban/discourage, and cleanup labels should be policy inputs or outputs, not runtime-only side effects.
- **D-14:** Permissioned or protected peers may receive scoped policy treatment, but they still count toward per-peer and aggregate resource evidence. Scoped block-serving permissions must not grant unbounded queues, request capacity, or serving behavior.
- **D-15:** Use injected timestamps and synthetic peer/resource records for tests. Do not add wall-clock sleeps, public-network peers, service-manager operations, or long-running default verification.
- **D-16:** Reuse existing low-cardinality labels where they fit, and add block-serving-specific labels only when they remove ambiguity. Suggested labels include `block_serving_disabled`, `block_serving_eligible`, `block_serving_suppressed`, `block_status_unavailable`, `block_status_pruned`, `block_status_unvalidated`, and `block_request_cap_reached`.

### Evidence, Docs, And Guardrails

- **D-17:** Project block-serving activation, eligibility, status classification, and resource decisions through shared status/evidence contracts before CLI, dashboard, RPC, metrics, logs, or support renderers format them. Avoid renderer-local summaries.
- **D-18:** Add deterministic guardrails if docs, parity roots, or release-boundary text change. The checker should reject claims that v2.1 enables public serving by default, archive-node behavior, package relay, bloom/filter serving, compact filter serving, public-network CI, production full-node readiness, production service operation, or production-funds wallet use.
- **D-19:** Verification remains `bash scripts/verify.sh`, deterministic, local, and public-network-free. Any public-network block-serving or compact-relay review belongs in opt-in UAT guidance, not pre-commit or default CI.
- **D-20:** New first-party Rust source or test files under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests` need parity breadcrumbs in file comments and `docs/parity/source-breadcrumbs.json` unless an explicit `none` breadcrumb is defensible.

### the agent's Discretion

The planner may choose exact config key names, Rust type names, module boundaries, status field names, and checker filenames. Prefer the smallest pure API that keeps block-serving policy separate from transaction relay policy, keeps runtime adapters thin, and leaves Phase 111+ to perform actual block reads and responses.

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
- `.planning/PROJECT.md` - active v2.1 scope, parity value, architecture constraints, and deferred production/public-serving claims.
- `.planning/REQUIREMENTS.md` - BSRV-01, BSRV-02, BSRV-03, BSRV-05, and BSRV-06 ownership for Phase 110.
- `.planning/ROADMAP.md` - Phase 110 goal, success criteria, requirement mapping, and milestone boundaries.
- `.planning/STATE.md` - current milestone state, v2.1 pending notes, local verification caveats, and repo-local UAT command reminders.

### Prior Locked Decisions

- `.planning/phases/91-peer-permissions-and-connection-classes/91-CONTEXT.md` - permission vocabulary, connection classes, bounded active effects, inactive relay-like labels, and redaction rules.
- `.planning/phases/94-dos-and-resource-governance/94-CONTEXT.md` - pure resource-governance policy, stable labels, request caps, timeout/churn inputs, and no relay side effects.
- `.planning/phases/100-relay-activation-boundary-and-permission-semantics/100-CONTEXT.md` - default-off activation model, peer eligibility matrix, scoped permission effects, low-cardinality evidence, and no-claim guardrails.
- `.planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-CONTEXT.md` - runtime activation propagation, download eligibility gates, suppression evidence, and production construction hazards.
- `.planning/phases/108-durable-mempool-relay-state-recovery/108-CONTEXT.md` - durable recovery boundary and relay-state restart patterns to avoid coupling into block-serving activation.

### Existing Code Integration Points

- `packages/open-bitcoin-network/src/relay.rs` - existing pure transaction-relay activation and eligibility policy to use as a pattern, not as an overloaded block-serving API.
- `packages/open-bitcoin-network/src/inbound/permissions.rs` - permission tokens, `download`, relay-like effects, inactive labels, connection classes, and typed permission bundles.
- `packages/open-bitcoin-network/src/resource.rs` - Phase 94 resource-governance caps, labels, queue/request pressure inputs, timeout/churn policy, and redacted events.
- `packages/open-bitcoin-primitives/src/network.rs` - inventory types for block, compact block, and witness block plus message command primitives.
- `packages/open-bitcoin-codec/src/block.rs` - block and block-header codec surface that later serving phases will use.
- `packages/open-bitcoin-chainstate/src/engine.rs` - validated block/connect state and active-chain concepts that status classification must not mutate.
- `packages/open-bitcoin-node/src/network.rs` - managed network runtime and adapter boundary for later block-serving integration.
- `packages/open-bitcoin-node/src/network/relay_serving.rs` - existing transaction serving cache/status pattern and sanitized serving outcomes.
- `packages/open-bitcoin-node/src/status/relay_evidence.rs` - shared low-cardinality relay evidence contract to mirror or extend carefully for block-serving evidence.
- `packages/open-bitcoin-node/src/status/inbound.rs` - inbound status/evidence contract and redaction precedent.
- `docs/parity/source-breadcrumbs.json` - required breadcrumb registry for new or touched first-party Rust files.
- `scripts/verify.sh` - repo-native verification contract and checker ordering.

### Docs, Parity, And Release Boundaries

- `docs/architecture/status-snapshot.md` - shared status ownership and unavailable-field policy.
- `docs/architecture/operator-observability.md` - low-cardinality status, metrics, logs, and support evidence constraints.
- `docs/operator/runtime-guide.md` - repo-local operator command style and opt-in UAT posture.
- `docs/parity/catalog/p2p.md` - P2P parity catalog and deferred relay/block-serving boundary notes.
- `docs/parity/index.json` - machine-readable parity surface ownership.
- `docs/parity/checklist.md` - parity checklist roots.
- `docs/parity/release-readiness.md` - deterministic verifier/public-network boundary and deferred-surface wording.
- `.planning/milestones/v2.0-ROADMAP.md` - prior transaction relay activation, eligibility, and no-claim guardrail pattern.
- `.planning/milestones/v2.0-MILESTONE-AUDIT.md` - v2.0 audit boundary and residual no-debt status that v2.1 should not regress.

### Bitcoin Knots Anchors

- `packages/bitcoin-knots/src/net_permissions.h` - permission flag vocabulary and download/relay permission anchors.
- `packages/bitcoin-knots/src/net_permissions.cpp` - permission parsing, `all` expansion, and label behavior.
- `packages/bitcoin-knots/src/net.cpp` - peer connection classes, protected peer behavior, service flags, upload/resource policy, and connection manager context.
- `packages/bitcoin-knots/src/net_processing.cpp` - block inventory handling, block serving, compact-block negotiation hazards, request bounds, peer state, and DoS response boundaries.
- `packages/bitcoin-knots/src/protocol.h` - inventory constants, message command names, service flags, and BIP152 command anchors.
- `packages/bitcoin-knots/src/validation.cpp` - active-chain, validated block, side-chain, and block-availability anchors.
- `packages/bitcoin-knots/src/node/blockstorage.cpp` - block file availability and pruned/unavailable block anchors.
- `packages/bitcoin-knots/test/functional/p2p_getdata.py` - block `getdata` behavior and request boundary anchor.
- `packages/bitcoin-knots/test/functional/p2p_compactblocks.py` - compact-block activation and BIP152 behavior to defer beyond Phase 110 except as guardrail context.
- `packages/bitcoin-knots/test/functional/p2p_permissions.py` - permission and protected peer behavior expectations.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- `RelayActivationConfig`, `RelayEligibilityInput`, `RelayEligibilityDecision`, and `RelayEligibilityReason` in `open-bitcoin-network/src/relay.rs` show the preferred pure-policy shape for activation and peer eligibility.
- `PeerConnectionClass`, `PeerPermissionSet`, `PermissionEffectLabel`, `RelayPermissionEffectLabel`, and `InactivePermissionEffectLabel` in `open-bitcoin-network/src/inbound/permissions.rs` already encode the connection and permission vocabulary needed for a block-serving eligibility matrix.
- `ResourceGovernancePolicy`, `QueuePressureInput`, `RequestPressureInput`, timeout/churn inputs, and stable resource labels in `open-bitcoin-network/src/resource.rs` provide the existing pure governance seam.
- `InventoryType::Block`, `InventoryType::CompactBlock`, and `InventoryType::WitnessBlock` already exist in `open-bitcoin-primitives/src/network.rs`.
- `RelayEvidenceStatus` and related evidence fields in `open-bitcoin-node/src/status/relay_evidence.rs` demonstrate the shared-status-first pattern for sanitized operator evidence.

### Established Patterns

- Pure network policy belongs in `open-bitcoin-network`; managed runtime, durable storage, clocks, sockets, logs, and process effects stay in node/RPC/CLI adapters.
- Config parsing is Open Bitcoin-owned and should use JSONC/CLI patterns rather than silently accepting Knots compatibility flags as shortcuts.
- Shared status owns evidence before CLI, dashboard, RPC, metrics, logs, or support renderers project it.
- Deterministic checker scripts are Bun/TypeScript and fixed-file based; public-network UAT remains opt-in and outside default verification.
- New Rust source/test files need parity breadcrumbs and Arrange/Act/Assert-style tests when non-trivial.

### Integration Points

- Add a focused block-serving policy module under `open-bitcoin-network` if adding this behavior to `relay.rs`, `resource.rs`, `peer.rs`, or `inbound.rs` would blur responsibilities or push large files further.
- Use `open-bitcoin-node` only to project policy decisions and later consume them in runtime adapters; Phase 110 should not add storage reads or actual block serving.
- Extend docs/parity/source breadcrumbs and deterministic no-claim checks only for files and docs actually touched by Phase 110 plans.

</code_context>

<specifics>
## Specific Ideas

- Treat "block serving" and "compact relay" as separately named activation facts even if one user-facing setting enables both later. Phase 110 should make future separation possible.
- Keep status labels stable and low-cardinality because later phases will need to aggregate served, suppressed, unavailable, pruned, malformed, timeout, and cleanup outcomes.
- Treat `download` and `all` permissions as regression hotspots. Tests must prove they do not enable archive-node, public-default, compact-filter, bloom-filter, package-relay, transaction-relay, or production claims.
- Prefer policy outputs that later phases can consume directly instead of prose-only docs or renderer-local flags.
- Preserve repo-local UAT command style whenever operator docs are touched: use explicit `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- ...` and `bazel run //packages/open-bitcoin-cli:open_bitcoin -- ...` forms.

</specifics>

<deferred>
## Deferred Ideas

Full block and witness block response handling, BIP152 wire codecs, compact relay negotiation, compact-block reconstruction, missing transaction round trips, fallback/validation handoff, operator evidence rollout, parity/UAT closeout, package relay, bloom/filter serving, compact filter serving, public serving defaults, public-network CI, archive-node claims, production full-node readiness, production service operation, and production-funds wallet use remain outside Phase 110.

</deferred>

***

*Phase: 110-block-serving-activation-and-eligibility-boundary*
*Context gathered: 2026-07-04*
