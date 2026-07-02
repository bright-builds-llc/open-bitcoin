---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 106-2026-07-02T03-46-34
generated_at: 2026-07-02T03:48:26.726Z
---

# Phase 106: Parity Traceability, UAT, and Release Boundary Guardrails - Context

**Gathered:** 2026-07-02
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Phase 106 closes v2.0 by making the bounded transaction relay and mempool participation claim auditable. It updates parity traceability, deterministic no-claim checkers, repo-local UAT guidance, contributor-facing docs, runtime/operator docs, and release notes so Open Bitcoin can truthfully describe what v2.0 implements without implying compact block relay, bloom/filter serving, package relay, public relay defaults, public-network relay CI, production service operation, production full-node readiness, or production-funds wallet safety.

This phase should not add new relay behavior, public-network default checks, service-manager gates, long-running soak requirements, packaging claims, GUI work, hosted dashboards, migration apply mode, or production readiness claims. It is a closeout and guardrail phase over the evidence produced by Phases 100 through 105.

</domain>

<decisions>
## Implementation Decisions

### Traceability Ownership

- **D-01:** Treat Phase 106 as the canonical v2.0 traceability closeout for BOUND-01 through BOUND-05, while preserving Phases 100 through 105 as the implementation and surface evidence roots for ACT, INV, DL, MEM, REL, and OBS requirements.
- **D-02:** Every v2.0 requirement must have exactly one roadmap owner and concrete evidence roots in parity docs, `docs/parity/index.json`, checker tests, implementation tests, or phase verification artifacts. The planner should add a deterministic audit rather than relying on prose inspection.
- **D-03:** Parity refs must cite concrete Bitcoin Knots anchors for transaction relay, transaction download, mempool admission, validation, and policy behavior. Prefer existing Knots anchors from prior v2.0 phase contexts and add missing anchors only where Phase 106 evidence requires them.
- **D-04:** Source breadcrumbs remain mandatory for first-party Rust source or test files under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests`. If Phase 106 only touches docs/scripts, it should still verify that existing v2.0 Rust breadcrumbs cover the implemented relay and mempool surfaces.

### Deterministic No-Claim Guardrails

- **D-05:** Add or extend deterministic checkers for compact block relay, bloom/filter serving, package relay, public-relay-default, production-readiness, production-service, public-network relay CI, and production-funds wallet claims across the v2.0 closeout corpus.
- **D-06:** Guardrails should be fixed-corpus and fixture-tested. They should reject positive support language while allowing explicit deferred, out-of-scope, opt-in, or no-claim wording.
- **D-07:** The checker should validate both visible documentation claims and executable verifier wiring when Phase 106 adds a new checker. Documentation-only references to `scripts/verify.sh` are not enough.
- **D-08:** Avoid broad text scans that make ordinary parity anchors impossible to maintain. Use targeted required phrases, required evidence roots, and forbidden positive-claim fixtures so the guardrail is strict but not brittle.

### UAT Guidance

- **D-09:** UAT guidance must use copy-pasteable repo-local Cargo and Bazel forms, not only the installed `open-bitcoin` alias.
- **D-10:** Required local operator command forms are:
  - `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- ...`
  - `bazel run //packages/open-bitcoin-cli:open_bitcoin -- ...`
- **D-11:** Public-network relay review, if documented, must stay explicit opt-in UAT evidence outside default verification. It must not become a CI, pre-commit, release-blocking, wall-clock soak, service-manager, or production-deployment gate.
- **D-12:** UAT copy should distinguish local bounded relay/mempool evidence from public propagation. A successful `sendrawtransaction` or queued relay outcome does not guarantee public relay.

### Docs And Release Boundary Wording

- **D-13:** Refresh README, operator runtime docs, parity docs, and release notes around one bounded v2.0 claim: Open Bitcoin now has explicit, default-off, bounded transaction relay and mempool participation evidence for review and operator testing.
- **D-14:** Docs must list deferred surfaces clearly: compact block relay, bloom/filter serving, package relay, public relay defaults, public-network relay CI, production service operation, production full-node readiness, production-funds wallet use, packaging, GUI, hosted dashboards, migration apply mode, and destructive repair.
- **D-15:** Operator-facing wording should remain quiet and evidence-focused. Avoid marketing claims, broad drop-in replacement language, and production-ready phrasing.
- **D-16:** Parity docs should preserve intentional differences and deferred behavior in `docs/parity/index.json` and companion docs rather than hiding gaps in release prose.

### Default Verification Contract

- **D-17:** `bash scripts/verify.sh` remains the default verification contract and must include the v2.0 guardrails after the prior phase checkers in deterministic order.
- **D-18:** Default verification must remain public-network-free, wall-clock-soak-free, service-manager-free, production-deployment-free, and free of destructive repair or migration mutation gates.
- **D-19:** Phase 106 verification should include parity breadcrumb checks, release-boundary checker tests, docs/UAT command checks, roadmap/requirements traceability audit, and the full repo-native `bash scripts/verify.sh`.
- **D-20:** If local generated Rust test binaries still hang before test execution, preserve the existing repo-local caveat only when it is still true and use the repo verifier as the final contract rather than inventing alternate release criteria.

### the agent's Discretion

The planner may choose exact checker names, fixture layout, audit script boundaries, and doc wording structure. Prefer small Bun-backed TypeScript checkers for substantial logic, thin Bash orchestration only where already established, targeted tests for checker behavior, and minimal scoped doc edits that align with existing v2.0 language.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Project And v2.0 Scope

- `.planning/PROJECT.md` - Project value, active v2.0 scope, deferred surfaces, architecture constraints, and no-production-readiness boundary.
- `.planning/REQUIREMENTS.md` - BOUND-01 through BOUND-05 plus the full 32-requirement v2.0 traceability table.
- `.planning/ROADMAP.md` - Phase 106 purpose, scope, success criteria, dependency on Phase 105, deferred scope, and verification contract.
- `.planning/STATE.md` - Current Phase 106 focus, recent Phase 100 through Phase 105 completion notes, UAT command reminders, and deterministic verification caveats.
- `.planning/CONVENTIONS.md` - Parity evidence, operator output, tooling, and planning artifact conventions.
- `.planning/ARCHITECTURE.md` - Functional-core and imperative-shell boundaries that closeout docs must preserve.
- `.planning/STACK.md` - Rust, Bazel, Bun, and verifier runtime context.

### Prior v2.0 Phase Context

- `.planning/phases/100-relay-activation-boundary-and-permission-semantics/100-CONTEXT.md` - Default-off activation, scoped relay permission effects, low-cardinality evidence, and no-claim guardrails.
- `.planning/phases/101-transaction-inventory-identity-and-download-scheduling/101-CONTEXT.md` - Txid/wtxid identity, bounded download scheduling, typed request actions, and deterministic tests.
- `.planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-CONTEXT.md` - Missing-parent staging, stable mempool outcome contract, and peer/local admission bridge.
- `.planning/phases/103-mempool-chainstate-lifecycle-and-durable-recovery/103-CONTEXT.md` - Mempool pressure truth, block/reorg lifecycle, durable recovery, and lifecycle checker boundaries.
- `.planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-CONTEXT.md` - Relay serving, fanout, local submission evidence, rebroadcast-deferred boundary, and lifecycle cleanup.
- `.planning/phases/105-operator-rpc-metrics-logs-and-support-evidence/105-CONTEXT.md` - Shared status contract, RPC/CLI/dashboard projection, fixed metrics/logs, support redaction, and Phase 105 checker vocabulary.

### Parity, Operator, And Release Docs

- `README.md` - Contributor-facing current-state, setup, verification, and v2.0 claim copy.
- `docs/operator/runtime-guide.md` - Operator UAT commands, runtime guidance, opt-in review boundaries, and production non-claims.
- `docs/architecture/operator-observability.md` - Status, metrics, logs, support evidence, and low-cardinality observability constraints.
- `docs/parity/README.md` - Parity documentation entrypoint and evidence model.
- `docs/parity/checklist.md` - Surface checklist that should reflect v2.0 closeout evidence.
- `docs/parity/index.json` - Machine-readable parity surfaces, evidence roots, Knots anchors, intentional differences, and deferred items.
- `docs/parity/source-breadcrumbs.json` - Required source breadcrumb registry.
- `docs/parity/catalog/p2p.md` - P2P and transaction relay parity roots and deferred relay boundaries.
- `docs/parity/catalog/mempool-policy.md` - Mempool policy, lifecycle, and admission parity roots.
- `docs/parity/catalog/consensus-validation.md` - Validation anchors relevant to mempool admission and block lifecycle claims.
- `docs/parity/catalog/rpc-cli-config.md` - RPC/CLI behavior and Open Bitcoin-specific operator extension boundaries.
- `docs/parity/production-claim-boundary.md` - Production readiness, production service, and no-claim boundary language.
- `docs/parity/release-readiness.md` - Release readiness checks and claim-boundary evidence.
- `docs/parity/operator-runbooks.md` - Operator evidence and UAT wording style.

### Existing Checkers And Verification

- `scripts/verify.sh` - Repo-native verification contract and phase checker order.
- `scripts/check-phase100-relay-activation.ts` - Phase 100 checker pattern for relay activation and no-claim language.
- `scripts/check-phase101-transaction-inventory-download.ts` - Phase 101 checker pattern for transaction inventory/download traceability.
- `scripts/check-phase102-orphan-admission-bridge.ts` - Phase 102 checker pattern for orphan/admission evidence.
- `scripts/check-phase103-mempool-lifecycle-recovery.ts` - Phase 103 checker pattern for mempool lifecycle and recovery evidence.
- `scripts/check-phase104-relay-serving-fanout.ts` - Phase 104 checker pattern for relay serving/fanout and rebroadcast-deferred evidence.
- `scripts/check-phase105-operator-relay-evidence.ts` - Phase 105 checker pattern for operator/RPC/metrics/log/support evidence and no-claim wording.
- `scripts/check-parity-breadcrumbs.ts` - Source breadcrumb verification.
- `scripts/check-no-claim-drift.ts` - Existing no-claim drift guardrails that Phase 106 may extend or complement.
- `scripts/check-release-readiness.ts` - Release-readiness checker pattern and boundary wording.

### Open Bitcoin Implementation Evidence

- `packages/open-bitcoin-network/src/relay.rs` - Relay activation and peer eligibility policy.
- `packages/open-bitcoin-network/src/peer/transaction_relay.rs` - Txid/wtxid relay identity and transaction relay module exports.
- `packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs` - Bounded transaction download scheduler.
- `packages/open-bitcoin-network/src/peer/transaction_relay/orphanage.rs` - Bounded orphan handling vocabulary.
- `packages/open-bitcoin-network/src/peer/transaction_relay/serving.rs` - Pure relay serving outcomes.
- `packages/open-bitcoin-network/src/peer/transaction_relay/fanout.rs` - Pure fanout actions and `rebroadcast_deferred` evidence.
- `packages/open-bitcoin-mempool/src/outcome.rs` - Stable mempool outcome vocabulary.
- `packages/open-bitcoin-mempool/src/pool.rs` - Mempool admission, replacement, trimming, and lifecycle core.
- `packages/open-bitcoin-mempool/src/pool/lifecycle.rs` - Mempool lifecycle cleanup summaries.
- `packages/open-bitcoin-node/src/network/relay_serving.rs` - Managed relay serving evidence.
- `packages/open-bitcoin-node/src/network/relay_fanout.rs` - Managed fanout and local submission evidence.
- `packages/open-bitcoin-node/src/status.rs` - Shared status snapshot and relay/mempool projection.
- `packages/open-bitcoin-node/src/metrics.rs` - Fixed metric kind surface.
- `packages/open-bitcoin-node/src/logging.rs` - Structured log source and sanitizer.
- `packages/open-bitcoin-rpc/src/dispatch/node.rs` - RPC projection for local submission and network status.
- `packages/open-bitcoin-cli/src/operator/status/render.rs` - Operator status rendering.
- `packages/open-bitcoin-cli/src/operator/support/redaction.rs` - Support evidence redaction.

### Bitcoin Knots Anchors

- `packages/bitcoin-knots/src/net_processing.cpp` - P2P transaction relay, inv/getdata/tx/notfound processing, mempool interaction, relay suppression, and compact-block boundary hazards.
- `packages/bitcoin-knots/src/node/txdownloadman.h` - Transaction download manager contract.
- `packages/bitcoin-knots/src/node/txdownloadman_impl.cpp` - Transaction announcement, request scheduling, fallback, notfound, cleanup, and accepted/rejected handling.
- `packages/bitcoin-knots/src/protocol.h` - Inventory types and protocol message identifiers.
- `packages/bitcoin-knots/src/txorphanage.h` - Orphan transaction staging contract.
- `packages/bitcoin-knots/src/txorphanage.cpp` - Orphan add, erase, expiry, peer cleanup, and reconsideration behavior.
- `packages/bitcoin-knots/src/txmempool.h` - Mempool state, entry/index ownership, conflicts, descendants, and policy structures.
- `packages/bitcoin-knots/src/txmempool.cpp` - Mempool acceptance, trimming, replacement, removal, and rolling fee behavior.
- `packages/bitcoin-knots/src/validation.cpp` - Validation, block connect/disconnect, mempool removal, and disconnected transaction handling.
- `packages/bitcoin-knots/src/policy/policy.h` - Standardness and relay policy declarations.
- `packages/bitcoin-knots/src/policy/policy.cpp` - Standardness and relay policy implementation.
- `packages/bitcoin-knots/src/policy/rbf.h` - Replacement policy declarations.
- `packages/bitcoin-knots/src/policy/rbf.cpp` - Replacement policy implementation.
- `packages/bitcoin-knots/src/rpc/rawtransaction.cpp` - Baseline `sendrawtransaction` behavior.
- `packages/bitcoin-knots/src/rpc/mempool.cpp` - Baseline `getmempoolinfo` behavior.
- `packages/bitcoin-knots/src/rpc/net.cpp` - Baseline `getnetworkinfo` behavior.
- `packages/bitcoin-knots/test/functional/p2p_tx_download.py` - Transaction announcement, download, fallback, and cleanup behavior.
- `packages/bitcoin-knots/test/functional/p2p_getdata.py` - Peer transaction inventory serving behavior.
- `packages/bitcoin-knots/test/functional/p2p_orphan_handling.py` - Orphan and parent request behavior.
- `packages/bitcoin-knots/test/functional/mempool_accept.py` - Admission policy, validation, and rejection behavior.
- `packages/bitcoin-knots/test/functional/mempool_accept_wtxid.py` - Wtxid-aware admission behavior.
- `packages/bitcoin-knots/test/functional/mempool_reorg.py` - Disconnected block transaction reconsideration behavior.
- `packages/bitcoin-knots/test/functional/mempool_persist.py` - Mempool persistence and restart behavior.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- `scripts/check-phase100-relay-activation.ts` through `scripts/check-phase105-operator-relay-evidence.ts`: Phase checker style for fixed-corpus evidence checks, forbidden overclaim fixtures, required docs, and verifier-order assertions.
- `scripts/check-no-claim-drift.ts` and `scripts/check-release-readiness.ts`: Existing release-boundary and no-claim checker patterns that Phase 106 should align with rather than duplicate blindly.
- `scripts/verify.sh`: The authoritative verifier wiring point for Phase 106 guardrails.
- `docs/parity/index.json`, `docs/parity/checklist.md`, and `docs/parity/catalog/*.md`: Current parity registry and catalog surfaces that should carry the v2.0 closeout traceability.
- `docs/parity/source-breadcrumbs.json` and `scripts/check-parity-breadcrumbs.ts`: Existing source-anchor enforcement for first-party Rust files.

### Established Patterns

- Prior closeout phases add a deterministic TypeScript checker plus checker tests, wire both into `scripts/verify.sh`, then record phase verification.
- Substantial repo-owned automation should be TypeScript run by Bun; Bash remains thin orchestration.
- Operator and support evidence must use fixed low-cardinality labels and sanitize raw transaction, peer, endpoint, permission, credential, and dynamic label material.
- UAT instructions should show repo-local Cargo and Bazel forms directly, with opt-in public-network review outside default verification.

### Integration Points

- Phase 106 should integrate at docs/parity registries, release/runtime docs, checker tests, and `scripts/verify.sh`.
- If implementation evidence paths in docs/parity are stale after Phases 100 through 105, update the registry and companion catalog entries instead of adding a separate unreferenced closeout manifest.
- If a traceability audit is added, it should read `.planning/REQUIREMENTS.md`, `.planning/ROADMAP.md`, parity docs, and phase artifacts deterministically from the repo checkout.

</code_context>

<specifics>
## Specific Ideas

- Use a v2.0 closeout checker/audit to prove all 32 requirements are mapped once and have evidence roots.
- Keep BOUND-03 UAT text anchored on exact Cargo/Bazel commands, matching the repo lesson and `AGENTS.md` guidance.
- Treat public-network relay review as optional operator UAT, never default CI or pre-commit verification.
- Prefer updating existing parity docs and index entries over creating a disconnected release note that can drift.

</specifics>

<deferred>
## Deferred Ideas

- Compact block relay, including `cmpctblock`, `getblocktxn`, and `blocktxn` behavior.
- Bloom/filter serving, compact filters, and BIP37-style filter behavior.
- Broad package relay, cluster mempool policy, and package orphan behavior.
- Public relay by default and public-network relay CI.
- Production full-node readiness, production service operation, production-funds wallet safety, packaging, GUI, hosted dashboards, migration apply mode, destructive repair, and automatic support-bundle upload.

</deferred>

***

*Phase: 106-parity-traceability-uat-and-release-boundary-guardrails*
*Context gathered: 2026-07-02*
