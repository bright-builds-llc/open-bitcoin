# Phase 110: Block Serving Activation and Eligibility Boundary - Research

**Researched:** 2026-07-04
**Domain:** Rust P2P policy boundary, Bitcoin Knots block-serving parity anchors, Open Bitcoin status/verification guardrails
**Confidence:** HIGH

<user_constraints>
## User Constraints (from CONTEXT.md)

Copied verbatim from `.planning/phases/110-block-serving-activation-and-eligibility-boundary/110-CONTEXT.md`. [VERIFIED: 110-CONTEXT.md]

### Locked Decisions

#### Activation Contract

- **D-01:** Block serving and compact-block relay must stay default-off through explicit Open Bitcoin-owned activation settings. Default daemon startup, default inbound serving, and existing transaction-relay activation must not make the node a public block-serving participant.
- **D-02:** Model block-serving activation as a typed pure policy decision, not scattered booleans in node runtime code. The policy should be unit-testable without sockets, durable storage, RPC, filesystem, public-network peers, or service-manager effects.
- **D-03:** Keep block-serving activation separate from transaction relay activation. Existing `RelayActivationConfig` and status patterns are reusable design references, but the planner should avoid overloading transaction-relay types when a block-serving-specific type prevents ambiguous states.
- **D-04:** Service bits, public defaults, inbound listener defaults, and transaction-relay behavior must not change in this phase. Any version-message or service-advertisement output must be an explicit policy output with matrix tests proving no accidental public-serving claim.

#### Peer Eligibility Matrix

- **D-05:** Add one explicit block-serving eligibility matrix for outbound, inbound, manual/operator-configured, protected, and permissioned peers. The matrix should emit stable machine labels for eligible, disabled, activation_required, inbound_serving_required, permission_required, protected_not_serving, status_unavailable, and permission_effect_inactive-style outcomes.
- **D-06:** Outbound and manual peers may become block-serving-eligible only after explicit block-serving activation. Ordinary inbound peers remain ineligible by default. Permissioned inbound peers require inbound serving plus a scoped block-serving/download-style permission input before later phases may read or send block data.
- **D-07:** Protected admission is not block-serving eligibility. Existing `forceinbound` and `noban` effects may protect admission, eviction, or misbehavior policy, but they must not activate block serving or compact relay by themselves.
- **D-08:** The existing `download` permission may be a policy input for bounded block-serving eligibility, but it must not imply archive-node behavior, unbounded historical serving, compact-block relay, transaction relay, package relay, bloom filters, compact filters, or production readiness.

#### Block Status Classification

- **D-09:** Introduce a pure block-serving status classifier before any storage read or socket response. It should distinguish validated, available, stale, side-chain, pruned, unavailable, unvalidated, unknown, and suppressed outcomes with stable typed labels.
- **D-10:** The classifier should accept current chain/header/block facts as data and return a decision that later adapters can consume. It should not perform durable storage reads, mutate chainstate, touch mempool state, or inspect runtime sockets directly.
- **D-11:** Pruned, unavailable, side-chain, and stale outcomes must be truthful but sanitized. Operator evidence and support bundles may expose stable labels and aggregate counters, but not prune-height details, raw peer endpoints, raw permission strings, credentials, dynamic labels, or raw block/transaction payloads.
- **D-12:** Classification should keep the v2.1 claim bounded to validated and available blocks inside the documented active-chain or recent-valid boundary. Unknown or unvalidated data must not be served optimistically.

#### Resource Governance

- **D-13:** Full block serving and compact-relay activation gates must participate in the existing Phase 94 resource-governance model before later phases add serving effects. Request caps, backpressure, timeouts, churn, ban/discourage, and cleanup labels should be policy inputs or outputs, not runtime-only side effects.
- **D-14:** Permissioned or protected peers may receive scoped policy treatment, but they still count toward per-peer and aggregate resource evidence. Scoped block-serving permissions must not grant unbounded queues, request capacity, or serving behavior.
- **D-15:** Use injected timestamps and synthetic peer/resource records for tests. Do not add wall-clock sleeps, public-network peers, service-manager operations, or long-running default verification.
- **D-16:** Reuse existing low-cardinality labels where they fit, and add block-serving-specific labels only when they remove ambiguity. Suggested labels include `block_serving_disabled`, `block_serving_eligible`, `block_serving_suppressed`, `block_status_unavailable`, `block_status_pruned`, `block_status_unvalidated`, and `block_request_cap_reached`.

#### Evidence, Docs, And Guardrails

- **D-17:** Project block-serving activation, eligibility, status classification, and resource decisions through shared status/evidence contracts before CLI, dashboard, RPC, metrics, logs, or support renderers format them. Avoid renderer-local summaries.
- **D-18:** Add deterministic guardrails if docs, parity roots, or release-boundary text change. The checker should reject claims that v2.1 enables public serving by default, archive-node behavior, package relay, bloom/filter serving, compact filter serving, public-network CI, production full-node readiness, production service operation, or production-funds wallet use.
- **D-19:** Verification remains `bash scripts/verify.sh`, deterministic, local, and public-network-free. Any public-network block-serving or compact-relay review belongs in opt-in UAT guidance, not pre-commit or default CI.
- **D-20:** New first-party Rust source or test files under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests` need parity breadcrumbs in file comments and `docs/parity/source-breadcrumbs.json` unless an explicit `none` breadcrumb is defensible.

### the agent's Discretion

The planner may choose exact config key names, Rust type names, module boundaries, status field names, and checker filenames. Prefer the smallest pure API that keeps block-serving policy separate from transaction relay policy, keeps runtime adapters thin, and leaves Phase 111+ to perform actual block reads and responses.

### Deferred Ideas (OUT OF SCOPE)

Full block and witness block response handling, BIP152 wire codecs, compact relay negotiation, compact-block reconstruction, missing transaction round trips, fallback/validation handoff, operator evidence rollout, parity/UAT closeout, package relay, bloom/filter serving, compact filter serving, public serving defaults, public-network CI, archive-node claims, production full-node readiness, production service operation, and production-funds wallet use remain outside Phase 110.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
| --- | --- | --- |
| BSRV-01 | Explicit activation settings; public serving off by default. [VERIFIED: .planning/REQUIREMENTS.md] | Use Open Bitcoin-owned JSONC/CLI config patterns from `RelayConfig` while adding block-specific activation types and default-off tests. [VERIFIED: packages/open-bitcoin-rpc/src/config/open_bitcoin.rs; packages/open-bitcoin-network/src/relay.rs] |
| BSRV-02 | Peer eligibility across outbound, inbound, manual, protected, and permissioned peers without accidental service-bit/default changes. [VERIFIED: .planning/REQUIREMENTS.md] | Reuse `PeerConnectionClass`, permission-effect labels, and the pure `classify_relay_eligibility` shape, but create block-serving-specific reasons. [VERIFIED: packages/open-bitcoin-network/src/inbound/permissions.rs; packages/open-bitcoin-network/src/relay.rs] |
| BSRV-03 | Serve only validated and available blocks inside documented active-chain or recent-valid boundary. [VERIFIED: .planning/REQUIREMENTS.md] | Put a pure status classifier before storage reads; feed it chain/header/block facts gathered by adapters. [VERIFIED: packages/open-bitcoin-chainstate/src/engine.rs; packages/open-bitcoin-node/src/network/inventory.rs; packages/bitcoin-knots/src/net_processing.cpp] |
| BSRV-05 | Report unknown, stale, side-chain, pruned, unavailable, unvalidated, and suppressed outcomes without leaking sensitive details. [VERIFIED: .planning/REQUIREMENTS.md] | Mirror existing shared status/evidence and redaction patterns, and keep labels fixed and aggregate. [VERIFIED: packages/open-bitcoin-node/src/status/relay_evidence.rs; docs/architecture/status-snapshot.md; docs/architecture/operator-observability.md] |
| BSRV-06 | Preserve block download, inbound governance, timeouts, churn, ban/discourage, and in-flight cleanup under adversarial block-serving bursts. [VERIFIED: .planning/REQUIREMENTS.md] | Reuse `ResourceGovernancePolicy` request/queue/timeout/churn/reconnect decisions and add tests proving block-serving gates do not bypass caps. [VERIFIED: packages/open-bitcoin-network/src/resource.rs; packages/open-bitcoin-network/src/peer/inventory_state.rs] |
</phase_requirements>

## Summary

Phase 110 should add a pure `open-bitcoin-network` policy boundary for block-serving activation, compact-relay activation, peer eligibility, block status classification, and resource-governance decisions; adapters should only pass facts into that boundary and later consume its typed outputs. [VERIFIED: 110-CONTEXT.md; standards/core/architecture.md] The closest existing pattern is transaction relay activation in `relay.rs`, but Phase 110 must define separate block-serving types so `relay.enabled`, transaction download, mempool relay, and public-service claims do not bleed into block serving. [VERIFIED: packages/open-bitcoin-network/src/relay.rs; 110-CONTEXT.md]

The current runtime already has a risk point: `serve_inventory` can send `Block` responses from `blocks_by_hash` for `InventoryType::Block` and `InventoryType::WitnessBlock` without a block-serving eligibility/status classifier. [VERIFIED: packages/open-bitcoin-node/src/network/inventory.rs] Phase 110 should not implement new block reads or BIP152 responses; it should put a planning-safe, testable decision boundary in place before Phase 111+ add actual serving effects. [VERIFIED: 110-CONTEXT.md; .planning/ROADMAP.md]

**Primary recommendation:** implement `open-bitcoin-network/src/block_serving.rs` as the pure core, add Open Bitcoin-owned default-off config in RPC config loading, project sanitized shared evidence in node status, and wire deterministic Bun guardrails only if docs/parity/release text changes. [VERIFIED: AGENTS.md; packages/open-bitcoin-rpc/src/config/open_bitcoin.rs; packages/open-bitcoin-node/src/status/relay_evidence.rs; scripts/check-phase100-relay-activation-boundary.ts]

## Project Constraints (from AGENTS.md)

- Use the root `AGENTS.md` plus `AGENTS.bright-builds.md`, `standards-overrides.md`, and relevant `standards/` pages before planning or implementation. [VERIFIED: AGENTS.md]
- Use `git submodule update --init --recursive` to materialize the pinned Bitcoin Knots baseline under `packages/bitcoin-knots`. [VERIFIED: AGENTS.md]
- Treat Bitcoin Knots `29.3.knots20260210` as the externally observable behavior baseline for in-scope parity claims. [VERIFIED: AGENTS.md; git submodule status packages/bitcoin-knots]
- Use `rust-toolchain.toml` as the Rust source of truth; the pinned toolchain is Rust `1.94.1`. [VERIFIED: AGENTS.md; rust-toolchain.toml]
- Use `bash scripts/verify.sh` as the repo-native verification contract; `--fast` is local iteration only. [VERIFIED: AGENTS.md; scripts/verify.sh]
- Keep pure business logic in functional-core crates and isolate filesystem, process, network, terminal, RPC, service-manager, and durable-storage effects in shell adapters. [VERIFIED: AGENTS.md; standards/core/architecture.md]
- Do not use existing Rust Bitcoin libraries in the production path. [VERIFIED: AGENTS.md]
- New first-party Rust source or test files under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests` need parity breadcrumb comments and `docs/parity/source-breadcrumbs.json` entries unless an explicit `none` breadcrumb is defensible. [VERIFIED: AGENTS.md; docs/parity/source-breadcrumbs.json]
- Rust tests should test one concern and use Arrange/Act/Assert comments when that improves clarity. [VERIFIED: AGENTS.md; standards/core/testing.md]
- Rust code should prefer `foo.rs` plus `foo/` over `foo/mod.rs`, avoid `unwrap()`, use `let...else` for early returns, and prefix internal `Option` names with `maybe_`. [VERIFIED: AGENTS.md; standards/languages/rust.md]
- Treat `docs/metrics/lines-of-code.md` as a tracked generated artifact that may be refreshed by verification. [VERIFIED: AGENTS.md; scripts/verify.sh]
- No project-specific skills exist under `.claude/skills/` or `.agents/skills/`. [VERIFIED: find .claude/skills .agents/skills]

## Standard Stack

### Core

| Library/Module | Version | Purpose | Why Standard |
| --- | --- | --- | --- |
| Rust workspace | Rust `1.94.1`, edition 2024, workspace package `0.1.0` | First-party implementation language and crate workspace. | The repo pins Rust and workspace metadata locally. [VERIFIED: rust-toolchain.toml; packages/Cargo.toml] |
| `open-bitcoin-network` | workspace `0.1.0` | Pure P2P policy types for activation, eligibility, permissions, inventory pressure, and resource governance. | Existing relay, permission, resource, and peer policy modules already live here. [VERIFIED: packages/Cargo.toml; packages/open-bitcoin-network/src/lib.rs] |
| `open-bitcoin-rpc` config loader | workspace `0.1.0` | Open Bitcoin-owned JSONC and CLI activation settings. | Existing `RelayConfig` is default-off, `deny_unknown_fields`, and has CLI override plumbing. [VERIFIED: packages/open-bitcoin-rpc/src/config/open_bitcoin.rs; packages/open-bitcoin-rpc/src/config/loader.rs] |
| `open-bitcoin-node` status/runtime adapter | workspace `0.1.0` | Shared status/evidence projection and later runtime consumption. | Existing relay evidence and managed network code separate adapter facts from pure decisions. [VERIFIED: packages/open-bitcoin-node/src/status/relay_evidence.rs; packages/open-bitcoin-node/src/network/relay_serving.rs] |
| Bitcoin Knots submodule | `v29.3.knots20260210` | Parity anchors for permissions, getdata, block availability, service flags, and compact-block commands. | The submodule is materialized at the pinned baseline. [VERIFIED: git submodule status packages/bitcoin-knots] |

### Supporting

| Library/Tool | Version | Purpose | When to Use |
| --- | --- | --- | --- |
| Bun | `1.3.9` | Deterministic TypeScript guardrail scripts and checker tests. | Use if docs/parity/release text changes need no-claim guardrails. [VERIFIED: .bun-version; bun --version; scripts/check-phase100-relay-activation-boundary.ts] |
| Bazel/Bazelisk command surface | `bazel 8.6.0` on PATH | Repo smoke build and UAT command parity. | Include in default verifier and copy-pasteable UAT commands when operator docs change. [VERIFIED: bazel --version; AGENTS.md; scripts/verify.sh] |
| `cargo-llvm-cov` | `0.8.5` | Full verifier coverage gate for pure-core crates. | Needed by `bash scripts/verify.sh` in full mode. [VERIFIED: cargo llvm-cov --version; scripts/verify.sh] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
| --- | --- | --- |
| New `block_serving.rs` pure module | Add fields to transaction `relay.rs` | Reusing `relay.rs` risks ambiguous transaction/block activation semantics; context explicitly requires separate block-serving activation. [VERIFIED: 110-CONTEXT.md; packages/open-bitcoin-network/src/relay.rs] |
| Shared status/evidence first | Renderer-local CLI/dashboard fields | Renderer-local summaries conflict with existing status ownership and redaction rules. [VERIFIED: docs/architecture/status-snapshot.md; docs/architecture/operator-observability.md] |
| Pure status classifier | Direct storage read in `serve_inventory` | Direct reads before classification can bypass eligibility, sanitization, and resource decisions. [VERIFIED: packages/open-bitcoin-node/src/network/inventory.rs; 110-CONTEXT.md] |
| Config through Open Bitcoin JSONC/CLI | Baseline `bitcoin.conf` or Knots `whitebind` shortcuts | Existing docs and config code reject Open Bitcoin-only keys in `bitcoin.conf` and do not treat whitelist/whitebind as activation aliases. [VERIFIED: docs/architecture/config-precedence.md; packages/open-bitcoin-rpc/src/config/tests.rs] |

**Installation:** no new package installation is recommended for Phase 110; use the existing workspace and run `git submodule update --init --recursive` if `packages/bitcoin-knots` is missing. [VERIFIED: AGENTS.md; packages/Cargo.toml]

**Version verification:** package versions were verified from local pins and installed tools rather than `npm view`, because Phase 110 should not add npm packages. [VERIFIED: rust-toolchain.toml; .bun-version; cargo --version; bun --version]

## Architecture Patterns

### Recommended Project Structure

```text
packages/open-bitcoin-network/src/
  block_serving.rs                 # Pure activation, eligibility, status, and resource boundary.
  block_serving/tests.rs           # Matrix tests with synthetic facts and injected timestamps.
  lib.rs                           # Re-export public policy types.

packages/open-bitcoin-rpc/src/config/
  open_bitcoin.rs                  # Default-off JSONC config structs.
  loader.rs                        # CLI override parsing if new flags are added.
  loader/open_bitcoin_runtime.rs   # Runtime resolution if config feeds daemon context.

packages/open-bitcoin-node/src/status/
  block_serving.rs                 # Shared sanitized evidence if status is in Phase 110 scope.
  relay_evidence.rs or status.rs   # Export integration point if status remains compact.

scripts/
  check-phase110-block-serving-boundary.ts
  check-phase110-block-serving-boundary.test.ts

docs/parity/source-breadcrumbs.json
```

This structure follows existing `resource.rs` plus `resource/tests.rs` and transaction relay checker patterns. [VERIFIED: packages/open-bitcoin-network/src/resource.rs; packages/open-bitcoin-network/src/resource/tests.rs; scripts/check-phase107-runtime-relay-activation-download-eligibility.ts]

### Pattern 1: Typed Pure Activation And Eligibility

**What:** define block-specific `BlockServingActivationConfig`, `CompactRelayActivationConfig`, `BlockServingEligibilityInput`, `BlockServingEligibilityDecision`, and `BlockServingEligibilityReason` with stable `as_str()` labels. [VERIFIED: packages/open-bitcoin-network/src/relay.rs; 110-CONTEXT.md]

**When to use:** use for every block-serving or compact-relay request path before storage reads, socket writes, version/service advertisement changes, or request-state mutation. [VERIFIED: 110-CONTEXT.md; packages/open-bitcoin-node/src/network/inventory.rs]

**Planning shape:**

```rust
// Source pattern: packages/open-bitcoin-network/src/relay.rs
pub fn classify_block_serving_eligibility(
    input: &BlockServingEligibilityInput,
) -> BlockServingEligibilityDecision {
    // Return a typed reason and stable label; do not touch sockets or storage.
}
```

### Pattern 2: Status Classification Before Storage Read

**What:** adapters gather facts such as activation, peer eligibility, resource suppression, header known, active-chain membership, recent-valid eligibility, validation state, block-data availability, and prune/unavailable indicators; the classifier returns a typed status and sanitized label. [VERIFIED: 110-CONTEXT.md; packages/open-bitcoin-chainstate/src/engine.rs; packages/open-bitcoin-node/src/storage/fjall_store.rs]

**When to use:** call before `serve_inventory` looks up or loads a block. [VERIFIED: packages/open-bitcoin-node/src/network/inventory.rs]

**Planning shape:**

```rust
// Source anchors: packages/bitcoin-knots/src/net_processing.cpp and
// packages/bitcoin-knots/src/node/blockstorage.cpp.
pub fn classify_block_serving_status(
    facts: &BlockServingStatusFacts,
) -> BlockServingStatusDecision {
    // Return one of: validated, available, stale, side_chain, pruned,
    // unavailable, unvalidated, unknown, suppressed.
}
```

### Pattern 3: Resource Governance As Policy Input/Output

**What:** keep request caps, queue pressure, timeout, churn, reconnect, ban/discourage, and cleanup as explicit facts or outputs that the block-serving boundary can observe. [VERIFIED: packages/open-bitcoin-network/src/resource.rs; 110-CONTEXT.md]

**When to use:** use before block-serving work is queued and when testing adversarial getdata/compact-relay request bursts. [VERIFIED: packages/open-bitcoin-network/src/peer/inventory_state.rs; packages/open-bitcoin-network/src/resource/tests.rs]

### Anti-Patterns to Avoid

- **Overload transaction relay types:** `RelayActivationConfig` is a pattern, not the block-serving activation type. [VERIFIED: 110-CONTEXT.md; packages/open-bitcoin-network/src/relay.rs]
- **Treat `download`, `noban`, `forceinbound`, or `all` as activation:** Knots-style permission flags expand into protection/download effects, but Phase 110 requires explicit block-serving activation. [VERIFIED: packages/open-bitcoin-network/src/inbound/permissions.rs; packages/bitcoin-knots/src/net_permissions.h; 110-CONTEXT.md]
- **Read storage before classification:** Knots checks request allowance and data availability before disk reads, and Phase 110 requires a pure classifier before storage reads. [VERIFIED: packages/bitcoin-knots/src/net_processing.cpp; 110-CONTEXT.md]
- **Change service bits or public defaults:** current tests prove transaction relay activation does not alter service flags; Phase 110 should add equivalent block-serving matrix tests. [VERIFIED: packages/open-bitcoin-network/src/relay.rs; packages/open-bitcoin-network/src/peer/tests.rs; 110-CONTEXT.md]
- **Emit dynamic labels or sensitive support data:** existing status docs prohibit peer ids, endpoints, permission strings, credentials, raw payloads, and dynamic labels in public/support evidence. [VERIFIED: docs/architecture/status-snapshot.md; docs/architecture/operator-observability.md]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
| --- | --- | --- | --- |
| Permission parsing and effects | New ad hoc string parser for block-serving permissions | Existing `PeerPermissionToken`, `PeerPermissionSet`, active/inactive effect labels, and `PeerConnectionClass` | The existing parser already models `download`, `noban`, `forceinbound`, `all`, relay-like inactive effects, and protected/permissioned classes. [VERIFIED: packages/open-bitcoin-network/src/inbound/permissions.rs] |
| Request caps and backpressure | Separate block-serving queue/cap model | Existing `ResourceGovernancePolicy` and `RequestPressureInput` | Phase 94 already owns caps, queue pressure, timeout, churn, repeated failure, and reconnect suppression labels. [VERIFIED: packages/open-bitcoin-network/src/resource.rs] |
| Block/witness/compact inventory constants | New numeric constants | `InventoryType::{Block, CompactBlock, WitnessBlock}` | Inventory codes already exist in primitives. [VERIFIED: packages/open-bitcoin-primitives/src/network.rs] |
| BIP152 command names and codecs | Compact-block protocol implementation in Phase 110 | Knots anchors plus deferred Phase 111+ implementation | BIP152 wire messages are explicitly deferred from Phase 110. [VERIFIED: 110-CONTEXT.md; packages/bitcoin-knots/src/protocol.h] |
| Status renderers | One-off CLI/dashboard/support fields | Shared node status/evidence contracts first | Existing status architecture requires shared ownership before renderers format evidence. [VERIFIED: docs/architecture/status-snapshot.md; packages/open-bitcoin-node/src/status/relay_evidence.rs] |
| Service-bit policy | Direct bit twiddling in runtime adapters | Explicit policy output plus matrix tests | Phase 110 requires no accidental service-bit or public-default changes. [VERIFIED: 110-CONTEXT.md; packages/open-bitcoin-primitives/src/network.rs] |

**Key insight:** the hard part is not sending a block; it is proving every future send passes explicit activation, deterministic eligibility, truthful status classification, and existing resource caps before it can touch storage or sockets. [VERIFIED: 110-CONTEXT.md; packages/open-bitcoin-node/src/network/inventory.rs; packages/open-bitcoin-network/src/resource.rs]

## Common Pitfalls

### Pitfall 1: `blocks_by_hash` Becomes The De Facto Serving Policy

**What goes wrong:** `serve_inventory` currently sends block data from `blocks_by_hash` when the requested inventory type is `Block` or `WitnessBlock`. [VERIFIED: packages/open-bitcoin-node/src/network/inventory.rs]

**Why it happens:** the adapter has data locally and no block-serving classifier yet. [VERIFIED: packages/open-bitcoin-node/src/network/inventory.rs]

**How to avoid:** Phase 110 should make the adapter call a pure block-serving decision before any later storage/cache lookup or response. [VERIFIED: 110-CONTEXT.md]

**Warning signs:** new tests assert sent blocks without asserting activation, eligibility, status label, and resource decision first. [VERIFIED: 110-CONTEXT.md; packages/open-bitcoin-network/src/relay.rs]

### Pitfall 2: Permission Effects Accidentally Activate Serving

**What goes wrong:** `download`, `noban`, `forceinbound`, or `all` can be mistaken as serving activation. [VERIFIED: packages/open-bitcoin-network/src/inbound/permissions.rs; packages/bitcoin-knots/src/net_permissions.h]

**Why it happens:** Knots permissions combine protection and download-serving concepts, and Open Bitcoin already maps `download` to a bounded active effect. [VERIFIED: packages/open-bitcoin-network/src/inbound/permissions.rs; packages/bitcoin-knots/src/net_permissions.h]

**How to avoid:** require both explicit block-serving activation and the scoped peer eligibility outcome before any serving effect. [VERIFIED: 110-CONTEXT.md]

**Warning signs:** tests with `all`, `download`, `noban`, or `forceinbound` become eligible while activation is false. [VERIFIED: 110-CONTEXT.md; packages/open-bitcoin-network/src/relay.rs]

### Pitfall 3: Public-Service Claims Leak Through Docs

**What goes wrong:** docs or parity roots can imply archive-node behavior, public serving by default, compact-block relay, package/filter support, public-network CI, or production readiness. [VERIFIED: 110-CONTEXT.md; scripts/check-phase100-relay-activation-boundary.ts; scripts/check-phase107-runtime-relay-activation-download-eligibility.ts]

**Why it happens:** block serving is a high-signal public-node feature, but Phase 110 is only a boundary. [VERIFIED: .planning/ROADMAP.md; 110-CONTEXT.md]

**How to avoid:** if docs/parity/release files change, add a Phase 110 checker and test it against forbidden positive claims and default-verifier scope. [VERIFIED: scripts/check-phase100-relay-activation-boundary.ts; scripts/check-phase100-relay-activation-boundary.test.ts]

**Warning signs:** `scripts/verify.sh` gains public-network, wall-clock, service-manager, or production-deployment gates. [VERIFIED: scripts/check-phase107-runtime-relay-activation-download-eligibility.ts; 110-CONTEXT.md]

### Pitfall 4: Status Labels Leak Sensitive Or High-Cardinality Data

**What goes wrong:** support/status evidence exposes prune heights, endpoints, permission strings, peer ids, raw block/transaction payloads, credentials, or dynamic labels. [VERIFIED: 110-CONTEXT.md; docs/architecture/status-snapshot.md; docs/architecture/operator-observability.md]

**Why it happens:** status classifiers often have rich facts that should not all become public evidence. [VERIFIED: docs/architecture/operator-observability.md]

**How to avoid:** return stable labels and aggregate counters from the shared status contract; keep detailed facts adapter-local. [VERIFIED: packages/open-bitcoin-node/src/status/relay_evidence.rs; docs/architecture/status-snapshot.md]

**Warning signs:** labels include block hashes, peer ids, endpoints, permission class names, exact prune heights, or free-form error text. [VERIFIED: docs/architecture/status-snapshot.md; 110-CONTEXT.md]

### Pitfall 5: Resource Governance Is Tested Only On Normal Requests

**What goes wrong:** permissioned/protected peers or compact-relay paths bypass per-peer caps, backpressure, timeouts, churn, ban/discourage, or in-flight cleanup. [VERIFIED: 110-CONTEXT.md; packages/open-bitcoin-network/src/resource.rs]

**Why it happens:** protected admission and serving eligibility can be confused with capacity exemption. [VERIFIED: 110-CONTEXT.md; packages/open-bitcoin-network/src/inbound/permissions.rs]

**How to avoid:** add adversarial matrix tests for ordinary, permissioned, protected, manual, and outbound peers with caps at boundary, over boundary, timeout, churn, repeated failure, and cleanup conditions. [VERIFIED: packages/open-bitcoin-network/src/resource/tests.rs; 110-CONTEXT.md]

**Warning signs:** tests only cover happy-path eligible peers or skip inactive effect labels. [VERIFIED: packages/open-bitcoin-network/src/resource/tests.rs; packages/open-bitcoin-network/src/relay.rs]

## Code Examples

### Existing Pure Eligibility Shape

```rust
// Source: packages/open-bitcoin-network/src/relay.rs
pub fn classify_relay_eligibility(
    input: &RelayEligibilityInput,
) -> RelayEligibilityDecision {
    let reason = relay_eligibility_reason(input);
    let eligible = reason == RelayEligibilityReason::Eligible;

    RelayEligibilityDecision {
        eligible,
        reason,
        active_permission_effects: input.relay_permission_effects.clone(),
        inactive_permission_effects: input.inactive_permission_effects.clone(),
        version_message_relay: eligible,
    }
}
```

Use the shape, not the transaction type names, for block serving. [VERIFIED: packages/open-bitcoin-network/src/relay.rs; 110-CONTEXT.md]

### Existing Resource Governance Shape

```rust
// Source: packages/open-bitcoin-network/src/resource.rs
let decision = policy.decide_request(&RequestPressureInput {
    command,
    inventory_items,
    tx_request_items,
    block_request_items,
    header_locator_items,
    active_permission_effects,
    inactive_permission_effects,
});
```

Phase 110 should feed block-serving request facts through the same governance model before later serving effects. [VERIFIED: packages/open-bitcoin-network/src/resource.rs; packages/open-bitcoin-network/src/peer/inventory_state.rs]

### Existing Knots Block-Serving Decision Order

```text
Lookup block index -> reject unknown -> check request allowed ->
apply historical/pruned/resource rules -> check data availability ->
read from disk -> send block or compact-block response.
```

Open Bitcoin should preserve the same ordering principle at the boundary level: decide eligibility/status before read/send effects. [VERIFIED: packages/bitcoin-knots/src/net_processing.cpp; packages/bitcoin-knots/src/node/blockstorage.cpp; 110-CONTEXT.md]

## State of the Art

| Old/Current Approach | Phase 110 Approach | When Changed | Impact |
| --- | --- | --- | --- |
| `serve_inventory` can send cached blocks directly. [VERIFIED: packages/open-bitcoin-node/src/network/inventory.rs] | Pure block-serving classifier gates later reads/responses. [VERIFIED: 110-CONTEXT.md] | Phase 110 | Prevents accidental serving before activation, eligibility, status, and resource checks. [VERIFIED: .planning/ROADMAP.md] |
| Transaction relay has `relay.enabled` and `RelayActivationConfig`. [VERIFIED: packages/open-bitcoin-rpc/src/config/open_bitcoin.rs; packages/open-bitcoin-network/src/relay.rs] | Block serving and compact relay get separate explicit activation facts. [VERIFIED: 110-CONTEXT.md] | Phase 110 | Avoids ambiguous tx-relay/block-serving states. [VERIFIED: 110-CONTEXT.md] |
| Inbound permission classes already classify active and inactive effects. [VERIFIED: packages/open-bitcoin-network/src/inbound/permissions.rs] | Block eligibility consumes scoped download-style effect as one input, not activation. [VERIFIED: 110-CONTEXT.md] | Phase 110 | Keeps protected/permissioned peers bounded. [VERIFIED: packages/open-bitcoin-network/src/resource.rs] |
| Phase 100/107 guardrails reject false public relay claims. [VERIFIED: scripts/check-phase100-relay-activation-boundary.ts; scripts/check-phase107-runtime-relay-activation-download-eligibility.ts] | Phase 110 guardrails should extend that pattern if docs/parity/release text changes. [VERIFIED: 110-CONTEXT.md] | Phase 110 | Prevents v2.1 block-serving boundary from being represented as public/default/production readiness. [VERIFIED: 110-CONTEXT.md] |

**Deprecated/outdated for this phase:**

- Using transaction relay activation as block-serving activation is out of scope and contradicts the locked context. [VERIFIED: 110-CONTEXT.md]
- Implementing BIP152 wire codecs, compact-block reconstruction, or full block responses in Phase 110 is out of scope. [VERIFIED: 110-CONTEXT.md]
- Adding public-network default verification or service-manager checks is out of scope. [VERIFIED: 110-CONTEXT.md; scripts/verify.sh]

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
| --- | --- | --- | --- |
| A1 | The recommended exact names `block_serving.rs`, `BlockServingActivationConfig`, and `CompactRelayActivationConfig` are planning recommendations, not locked project names. [ASSUMED] | Architecture Patterns | Low: the planner can rename while preserving the same boundaries. |
| A2 | Phase 110 status evidence may be implemented as a new `block_serving` status module or as a compact extension of existing status exports depending on implementation size. [ASSUMED] | Recommended Project Structure | Low: the user delegated exact module/status names to the planner. |
| A3 | The exact active-chain versus recent-valid precedence among `stale`, `side_chain`, and `unvalidated` needs planner/user confirmation if implementation needs a single precedence order in this phase. [ASSUMED] | Open Questions | Medium: different precedence can change operator labels while still preserving no-serve behavior. |

## Open Questions (RESOLVED)

1. **What exact numeric recent-valid boundary should Phase 110 encode?**
   - What we know: Knots has a `BlockRequestAllowed` concept for active-chain blocks and recent valid stale blocks. [VERIFIED: packages/bitcoin-knots/src/net_processing.cpp]
   - What's unclear: Phase 110 context does not lock a numeric age/depth/work boundary. [VERIFIED: 110-CONTEXT.md]
   - Recommendation: model `recent_valid_boundary` as an input fact in Phase 110 and defer exact numeric policy unless a plan needs it for deterministic tests. [ASSUMED]
   - RESOLVED: Phase 110 will encode recent-valid as an explicit typed input fact (`BlockServingChainPosition::RecentValid`) rather than a numeric age/depth/work boundary. This preserves D-12's bounded active-chain or recent-valid claim without expanding scope into Phase 111+ adapter policy or Knots numeric serving heuristics.

2. **Should compact relay have an independent user-facing setting in Phase 110?**
   - What we know: context requires block serving and compact-block relay default-off through explicit Open Bitcoin-owned activation settings. [VERIFIED: 110-CONTEXT.md]
   - What's unclear: context leaves exact config key names to the planner. [VERIFIED: 110-CONTEXT.md]
   - Recommendation: keep separate typed activation facts even if one config section owns both, so BIP152 can remain deferred without blocking block-serving policy. [ASSUMED]
   - RESOLVED: Phase 110 will add a separate default-off compact-relay activation fact and setting (`block_serving.compact_relay_enabled`) alongside block serving. It remains a policy/config boundary only; BIP152 codecs, negotiation, compact reconstruction, `getblocktxn`, and `blocktxn` stay deferred.

3. **How much status surface belongs in Phase 110 versus Phase 112+?**
   - What we know: D-17 requires shared status/evidence before renderers, and Phase 112 later owns operator evidence and UAT closeout. [VERIFIED: 110-CONTEXT.md; .planning/ROADMAP.md]
   - What's unclear: Phase 110 can satisfy the boundary with shared internal status types, while public renderers may wait. [ASSUMED]
   - Recommendation: implement shared status/evidence types and unit tests now; defer broad CLI/dashboard/support rendering unless docs or requirements force it. [ASSUMED]
   - RESOLVED: Phase 110 owns shared internal status/evidence contracts and tests only. Broad CLI, dashboard, RPC, metrics, logs, support rendering, UAT closeout, and release-boundary expansion remain in later phases unless Phase 110 docs/checker updates need to name the shared contract.

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
| --- | --- | --- | --- | --- |
| Rust `rustc` | Cargo workspace build/test | yes | `rustc 1.94.1` | None needed. [VERIFIED: rustc --version] |
| Cargo | Rust checks/tests | yes | `cargo 1.94.1` | None needed. [VERIFIED: cargo --version] |
| Bun | TypeScript guardrails | yes | `1.3.9` | None needed. [VERIFIED: bun --version] |
| Bash | `scripts/verify.sh` | yes | GNU bash `3.2.57` | None needed. [VERIFIED: bash --version] |
| Bazel/Bazelisk command | Full verifier smoke build | yes | `bazel 8.6.0` | None needed. [VERIFIED: bazel --version] |
| `cargo-llvm-cov` | Full verifier coverage gate | yes | `0.8.5` | None needed. [VERIFIED: cargo llvm-cov --version] |
| Git submodule `packages/bitcoin-knots` | Knots parity anchors | yes | `v29.3.knots20260210` | Run `git submodule update --init --recursive` if missing. [VERIFIED: git submodule status packages/bitcoin-knots; AGENTS.md] |

**Missing dependencies with no fallback:** none found. [VERIFIED: environment audit commands]

**Missing dependencies with fallback:** none found. [VERIFIED: environment audit commands]

## Validation Architecture

`workflow.nyquist_validation` is `false`, so a separate Nyquist validation matrix is not required. [VERIFIED: .planning/config.json] Standard plan verification is sufficient because Phase 110 can be proven through focused unit tests, existing deterministic TypeScript guardrails when docs/parity change, and the repo-native `bash scripts/verify.sh` contract. [VERIFIED: scripts/verify.sh; standards/core/testing.md; 110-CONTEXT.md]

Recommended verification map:

| Behavior | Test Type | Command |
| --- | --- | --- |
| Default-off block serving and compact relay config. [VERIFIED: 110-CONTEXT.md] | Rust config unit tests plus optional checker fixture | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-features` [VERIFIED: packages/open-bitcoin-rpc/src/config/tests.rs] |
| Peer eligibility matrix across outbound, inbound, manual, protected, and permissioned peers. [VERIFIED: 110-CONTEXT.md] | Rust unit tests in `open-bitcoin-network` | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network --all-features block_serving` [ASSUMED] |
| Status classifier labels and no-read decision order. [VERIFIED: 110-CONTEXT.md] | Rust unit tests with synthetic facts | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network --all-features block_serving` [ASSUMED] |
| Resource-governance caps/backpressure/timeouts/churn/cleanup remain active. [VERIFIED: 110-CONTEXT.md] | Rust resource policy tests plus adapter tests only if adapters change | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network --all-features resource` [ASSUMED] |
| Docs/parity no-claim guardrails if docs change. [VERIFIED: 110-CONTEXT.md] | Bun checker and checker tests | `bun test scripts/check-phase110-block-serving-boundary.test.ts && bun run scripts/check-phase110-block-serving-boundary.ts` [ASSUMED] |
| Full repo contract before done. [VERIFIED: AGENTS.md; scripts/verify.sh] | Repo verifier | `bash scripts/verify.sh` [VERIFIED: AGENTS.md; scripts/verify.sh] |

## Security Domain

OWASP ASVS latest stable version is `5.0.0` dated May 2025, and ASVS 5.0 uses version-qualified requirement identifiers because identifiers can change between versions. [CITED: https://github.com/OWASP/ASVS] This phase is a Rust P2P-node boundary rather than a web application, so the relevant security mapping is threat-pattern based while borrowing ASVS category language. [VERIFIED: AGENTS.md; 110-CONTEXT.md]

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
| --- | --- | --- |
| Authentication | no | Phase 110 does not add RPC/admin authentication or credentials. [VERIFIED: 110-CONTEXT.md] |
| Session Management | no | Phase 110 does not add sessions. [VERIFIED: 110-CONTEXT.md] |
| Access Control / Authorization | yes | Typed activation plus peer eligibility matrix gates serving before reads/sends. [VERIFIED: 110-CONTEXT.md; packages/open-bitcoin-network/src/relay.rs] |
| Validation, Sanitization, and Encoding | yes | Inventory/status/request facts should be typed and labels sanitized before public/status projection. [VERIFIED: 110-CONTEXT.md; docs/architecture/operator-observability.md] |
| Cryptography | no | Phase 110 adds no new cryptographic primitive or key handling. [VERIFIED: 110-CONTEXT.md] |
| Business Logic | yes | Active-chain/recent-valid/status precedence is business logic that must be deterministic and unit-tested. [VERIFIED: 110-CONTEXT.md; packages/bitcoin-knots/src/net_processing.cpp] |
| Files and Resources | yes | Existing resource-governance caps/backpressure/timeouts/churn apply to adversarial block-serving requests. [VERIFIED: packages/open-bitcoin-network/src/resource.rs; 110-CONTEXT.md] |

### Known Threat Patterns for Phase 110

| Pattern | STRIDE | Standard Mitigation |
| --- | --- | --- |
| Unauthorized block serving due to implicit activation | Elevation of Privilege | Default-off config plus typed activation and eligibility decisions. [VERIFIED: 110-CONTEXT.md; packages/open-bitcoin-network/src/relay.rs] |
| Resource exhaustion by block/compact-block request bursts | Denial of Service | Reuse `ResourceGovernancePolicy` request caps, queue pressure, timeout, churn, repeated failure, reconnect suppression, and cleanup evidence. [VERIFIED: packages/open-bitcoin-network/src/resource.rs] |
| Sensitive state disclosure through status labels | Information Disclosure | Low-cardinality labels and aggregate counters only; no prune heights, endpoints, permission strings, credentials, dynamic labels, or payloads. [VERIFIED: 110-CONTEXT.md; docs/architecture/status-snapshot.md] |
| Serving stale, side-chain, unvalidated, unknown, or unavailable data optimistically | Tampering / Information Disclosure | Status classifier must distinguish non-serve outcomes before reads/sends. [VERIFIED: 110-CONTEXT.md; packages/bitcoin-knots/src/net_processing.cpp] |
| Service-bit or public-default regression | Spoofing / Repudiation | Explicit policy output and matrix tests proving no accidental public-serving claim. [VERIFIED: 110-CONTEXT.md; packages/open-bitcoin-network/src/relay.rs] |

## Sources

### Primary (HIGH confidence)

- `AGENTS.md` - repo-local verification, Rust, parity breadcrumb, UAT command, and project constraints. [VERIFIED: AGENTS.md]
- `AGENTS.bright-builds.md` - Bright Builds workflow defaults. [VERIFIED: AGENTS.bright-builds.md]
- `standards/core/architecture.md` - functional core / imperative shell and domain-type guidance. [VERIFIED: standards/core/architecture.md]
- `standards/core/code-shape.md` - code shape, optional naming, early returns, and script guidance. [VERIFIED: standards/core/code-shape.md]
- `standards/core/testing.md` - behavior tests and Arrange/Act/Assert guidance. [VERIFIED: standards/core/testing.md]
- `standards/core/verification.md` - repo-native verification and commit gates. [VERIFIED: standards/core/verification.md]
- `standards/languages/rust.md` - Rust module, `Option`, panic, and verification guidance. [VERIFIED: standards/languages/rust.md]
- `.planning/STATE.md`, `.planning/ROADMAP.md`, `.planning/REQUIREMENTS.md`, and `110-CONTEXT.md` - phase scope, decisions, requirements, and deferred boundaries. [VERIFIED: local planning files]
- `packages/open-bitcoin-network/src/relay.rs` - existing pure transaction-relay activation/eligibility pattern. [VERIFIED: local source]
- `packages/open-bitcoin-network/src/inbound/permissions.rs` - permission token/effect/class vocabulary. [VERIFIED: local source]
- `packages/open-bitcoin-network/src/resource.rs` and `resource/tests.rs` - resource-governance policy and tests. [VERIFIED: local source]
- `packages/open-bitcoin-node/src/network/inventory.rs` - current block inventory serving seam. [VERIFIED: local source]
- `packages/open-bitcoin-node/src/status/relay_evidence.rs`, `docs/architecture/status-snapshot.md`, and `docs/architecture/operator-observability.md` - shared status/evidence/redaction patterns. [VERIFIED: local source/docs]
- `packages/bitcoin-knots/src/net_permissions.h`, `net_permissions.cpp`, `net.cpp`, `net_processing.cpp`, `protocol.h`, `node/blockstorage.cpp`, and `validation.cpp` - parity anchors. [VERIFIED: pinned submodule]
- `scripts/verify.sh`, `scripts/check-phase100-relay-activation-boundary.ts`, and `scripts/check-phase107-runtime-relay-activation-download-eligibility.ts` - verification and guardrail patterns. [VERIFIED: local scripts]

### Secondary (MEDIUM confidence)

- OWASP ASVS GitHub README - current ASVS stable version and identifier guidance. [CITED: https://github.com/OWASP/ASVS]

### Tertiary (LOW confidence)

- None.

## Metadata

**Confidence breakdown:**

- Standard stack: HIGH - versions and modules were verified from local pins, installed tools, and source files. [VERIFIED: rust-toolchain.toml; packages/Cargo.toml; environment audit commands]
- Architecture: HIGH - recommended shape follows locked context, existing pure relay/resource patterns, and Bright Builds architecture rules. [VERIFIED: 110-CONTEXT.md; packages/open-bitcoin-network/src/relay.rs; packages/open-bitcoin-network/src/resource.rs; standards/core/architecture.md]
- Pitfalls: HIGH - risks are grounded in current adapter seams, existing permission/resource behavior, and Knots serving anchors. [VERIFIED: packages/open-bitcoin-node/src/network/inventory.rs; packages/open-bitcoin-network/src/inbound/permissions.rs; packages/bitcoin-knots/src/net_processing.cpp]
- Exact names and status module placement: MEDIUM - user delegated names and module boundaries to the planner, so recommendations are intentionally flexible. [VERIFIED: 110-CONTEXT.md]

**Research date:** 2026-07-04
**Valid until:** 2026-08-03 for local architecture and pinned Knots findings; re-check installed tool versions and ASVS references after 30 days. [ASSUMED]
