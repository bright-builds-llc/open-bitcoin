# Phase 111: Full Block Serving Request Path - Research

**Researched:** 2026-07-04 [VERIFIED: current_date]
**Domain:** Rust P2P node-shell request handling for bounded full and witness block serving [VERIFIED: .planning/phases/111-full-block-serving-request-path/111-CONTEXT.md]
**Confidence:** HIGH [VERIFIED: local code, Phase 110 artifacts, pinned Bitcoin Knots sources]

<user_constraints>
## User Constraints (from CONTEXT.md)

All content in this section is copied from `.planning/phases/111-full-block-serving-request-path/111-CONTEXT.md`. [VERIFIED: .planning/phases/111-full-block-serving-request-path/111-CONTEXT.md]

### Locked Decisions

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

### Deferred Ideas (OUT OF SCOPE)

BIP152 wire codecs, `sendcmpct`, compact-block response payloads, compact-block reconstruction, `getblocktxn`, `blocktxn`, missing compact transaction round trips, fallback/validation handoff, broad operator evidence rollout, package relay, bloom/filter serving, compact filter serving, public serving defaults, public-network CI, archive-node claims, production full-node readiness, production-service operation, and production-funds wallet use remain outside Phase 111.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
| --- | --- | --- |
| BSRV-04 | Node handles block, witness block, and compact block `getdata` requests with bounded request caps, queue backpressure, and peer cleanup. [VERIFIED: .planning/REQUIREMENTS.md] | Use `PeerManager::handle_getdata` request-pressure gating, route `Block` and `WitnessBlock` through the Phase 110 block-serving gate, and classify `CompactBlock` as bounded non-served evidence. [VERIFIED: packages/open-bitcoin-network/src/peer/inventory_state.rs, packages/open-bitcoin-network/src/block_serving.rs, .planning/phases/111-full-block-serving-request-path/111-CONTEXT.md] |
| GOV-01 | Full block serving, compact block serving, partial compact-block state, missing transaction requests, and fallback all participate in existing request, queue, and in-flight resource limits. [VERIFIED: .planning/REQUIREMENTS.md] | Phase 111 only owns full block serving and compact-block request classification; it must reuse `ResourceGovernancePolicy`, `RequestPressureInput`, and Phase 110 cleanup labels rather than adding new caps. [VERIFIED: packages/open-bitcoin-network/src/block_serving.rs, .planning/phases/110-block-serving-activation-and-eligibility-boundary/110-03-SUMMARY.md] |
| GOV-05 | Historical, pruned, stale, side-chain, and unavailable block serving remains bounded by documented eligibility rules and does not imply archive-node behavior. [VERIFIED: .planning/REQUIREMENTS.md] | Use Phase 110 status labels and Knots `BlockRequestAllowed`/pruned-data anchors to test suppress or unavailable outcomes before local block reads. [VERIFIED: packages/open-bitcoin-network/src/block_serving.rs, packages/bitcoin-knots/src/net_processing.cpp, packages/bitcoin-knots/src/node/blockstorage.cpp] |
</phase_requirements>

## Project Constraints (from AGENTS.md)

- Read and follow `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md`, and relevant pages under `standards/` before planning or implementation. [VERIFIED: AGENTS.md, AGENTS.bright-builds.md, standards/index.md]
- Use `git submodule update --init --recursive` if the pinned Knots baseline under `packages/bitcoin-knots` is not materialized. [VERIFIED: AGENTS.md]
- Treat Rust `1.94.1` from `rust-toolchain.toml` as the source of truth for Cargo, CI, and Bazel. [VERIFIED: AGENTS.md, rust-toolchain.toml, MODULE.bazel]
- Use `bash scripts/verify.sh` as the repo-native verification contract; `--fast` is only for local iteration. [VERIFIED: AGENTS.md, scripts/verify.sh]
- Use explicit repo-local Cargo and Bazel command forms in UAT/operator docs instead of a bare installed alias. [VERIFIED: AGENTS.md, .codex/tasks/lessons.md]
- Use Bun for repo-owned higher-level automation scripts and TypeScript for substantial checker logic. [VERIFIED: AGENTS.md, standards/languages/typescript-javascript.md, .bun-version]
- Treat `docs/metrics/lines-of-code.md` as a tracked generated artifact that may require freshness updates when verification regenerates it. [VERIFIED: AGENTS.md, scripts/verify.sh]
- Record intentional behavior differences from Bitcoin Knots in `docs/parity/index.json` and companion docs under `docs/parity/`. [VERIFIED: AGENTS.md]
- Add parity breadcrumbs for new or touched first-party Rust source/test files under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests`. [VERIFIED: AGENTS.md, scripts/check-parity-breadcrumbs.ts]
- No project-local skills were found under `.claude/skills` or `.agents/skills`. [VERIFIED: project skill directory scan]
- Keep functional-core decisions in pure modules and filesystem, network, durable-storage, logs, and process effects in node/RPC/CLI adapters. [VERIFIED: AGENTS.md, standards/core/architecture.md, standards/languages/rust.md]
- Unit tests for pure/business logic must use focused Arrange, Act, Assert structure unless trivially clear. [VERIFIED: AGENTS.md, standards/core/testing.md, standards/languages/rust.md]
- New Rust module layouts should prefer `foo.rs` plus `foo/` over `foo/mod.rs`; internal optional Rust names should use `maybe_` when they represent `Option<T>`. [VERIFIED: standards/languages/rust.md]
- Avoid adding existing Rust Bitcoin libraries to the production path; the project owns the Bitcoin domain model. [VERIFIED: AGENTS.md]
- The `Validation Architecture` section is omitted because `workflow.nyquist_validation` is `false`. [VERIFIED: .planning/config.json]

## Summary

Phase 111 should be planned as a narrow bridge from already-implemented Phase 110 policy to the existing node-shell inventory serving path. `PeerManager::handle_getdata` already performs request-pressure checks before emitting `PeerAction::ServeInventory`, and `ManagedPeerNetwork::serve_inventory` is the current effectful adapter that directly serves cached blocks for `InventoryType::Block` and `InventoryType::WitnessBlock`. [VERIFIED: packages/open-bitcoin-network/src/peer/inventory_state.rs, packages/open-bitcoin-node/src/network/inventory.rs]

The main implementation change is to replace the direct `blocks_by_hash` block branch with a named block-serving adapter seam that first computes peer eligibility, block-serving status facts, and resource-gate decisions, then reads local validated block data only when all gates return serveable. [VERIFIED: packages/open-bitcoin-network/src/block_serving.rs, packages/open-bitcoin-node/src/network/inventory.rs, .planning/phases/111-full-block-serving-request-path/111-CONTEXT.md]

Witness serving can reuse `WireNetworkMessage::Block` only if the stored `Block` contains witness data; the current codec parses block transactions with witness enabled and encodes outbound blocks with witness encoding, so add a focused regression around the serving path instead of adding a new wire message. [VERIFIED: packages/open-bitcoin-codec/src/block.rs, packages/open-bitcoin-network/src/message.rs]

**Primary recommendation:** Add a small `open-bitcoin-node` block-serving adapter around Phase 110 policy, keep transaction serving on `RelayServingCache`, return compact-block requests as deterministic non-served evidence, and cover served/suppressed/unavailable/request-cap/no-archive cases with focused Rust tests plus a Phase 111 checker if docs/parity/verifier files change. [VERIFIED: .planning/phases/111-full-block-serving-request-path/111-CONTEXT.md, packages/open-bitcoin-node/src/network/relay_serving.rs, scripts/check-phase110-block-serving-boundary.ts]

## Standard Stack

### Core

| Library | Version | Purpose | Why Standard |
| --- | --- | --- | --- |
| Rust workspace crates | `0.1.0` workspace package version [VERIFIED: packages/Cargo.toml] | Implement first-party P2P policy, codec, chainstate, node shell, and tests. [VERIFIED: packages/Cargo.toml] | Project policy requires first-party Bitcoin domain models and no production dependency on existing Rust Bitcoin libraries. [VERIFIED: AGENTS.md] |
| Rust toolchain | `1.94.1`, edition 2024 [VERIFIED: rust-toolchain.toml, packages/Cargo.toml] | Compile, lint, and test Phase 111 Rust changes. [VERIFIED: scripts/verify.sh] | Repo pins the same Rust version for local Cargo, CI, and Bazel. [VERIFIED: AGENTS.md, MODULE.bazel] |
| `open-bitcoin-network` | workspace `0.1.0` [VERIFIED: packages/open-bitcoin-network/Cargo.toml] | Own pure block-serving eligibility, status, resource gate, cleanup labels, and peer request routing. [VERIFIED: packages/open-bitcoin-network/src/block_serving.rs, packages/open-bitcoin-network/src/peer/inventory_state.rs] | Repo architecture keeps pure network policy out of node-shell storage/socket adapters. [VERIFIED: AGENTS.md, standards/core/architecture.md] |
| `open-bitcoin-node` | workspace `0.1.0` [VERIFIED: packages/open-bitcoin-node/Cargo.toml] | Own `ManagedPeerNetwork`, local block cache, durable-storage adapters, and socket-message translation. [VERIFIED: packages/open-bitcoin-node/src/network.rs, packages/open-bitcoin-node/src/network/inventory.rs, packages/open-bitcoin-node/src/storage/fjall_store.rs] | This crate is the existing imperative shell for storage and outbound wire messages. [VERIFIED: AGENTS.md, packages/open-bitcoin-node/src/lib.rs] |
| `open-bitcoin-codec` | workspace `0.1.0` [VERIFIED: packages/open-bitcoin-codec/Cargo.toml] | Encode and parse block payloads, including witness transaction data. [VERIFIED: packages/open-bitcoin-codec/src/block.rs] | Existing `WireNetworkMessage::Block` delegates payload encoding to this codec. [VERIFIED: packages/open-bitcoin-network/src/message.rs] |
| `open-bitcoin-primitives` | workspace `0.1.0` [VERIFIED: packages/open-bitcoin-primitives/Cargo.toml] | Provide `InventoryType::Block`, `InventoryType::WitnessBlock`, `InventoryType::CompactBlock`, hashes, blocks, and transactions. [VERIFIED: packages/open-bitcoin-primitives/src/network.rs] | The inventory constants already match the pinned Knots protocol anchors for block, compact block, and witness block. [VERIFIED: packages/bitcoin-knots/src/protocol.h] |

### Supporting

| Library | Version | Purpose | When to Use |
| --- | --- | --- | --- |
| Fjall | `3.1.4` [VERIFIED: packages/open-bitcoin-node/Cargo.toml] | Durable node storage, including `save_block` and `load_block` under the block-index namespace. [VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store.rs] | Use only behind a thin node-shell adapter if Phase 111 must read durable local block data beyond `blocks_by_hash`. [VERIFIED: .planning/phases/111-full-block-serving-request-path/111-CONTEXT.md] |
| Bun | `1.3.9` [VERIFIED: .bun-version, `bun --version`] | Run deterministic TypeScript checker scripts and checker tests. [VERIFIED: scripts/verify.sh] | Use if Phase 111 changes docs, parity roots, release-boundary text, or verifier wiring. [VERIFIED: .planning/phases/111-full-block-serving-request-path/111-CONTEXT.md, scripts/check-phase110-block-serving-boundary.ts] |
| Bazel/rules_rust | Bazel `8.6.0`, `rules_rust` `0.69.0` [VERIFIED: `bazel --version`, MODULE.bazel] | Run the repo smoke build through the full verifier. [VERIFIED: scripts/verify.sh] | Use as part of `bash scripts/verify.sh`; do not create a separate Phase 111 build system. [VERIFIED: AGENTS.md, scripts/verify.sh] |
| `serde`/`serde_json` | `serde 1.0.228`, `serde_json 1.0.149` [VERIFIED: packages/open-bitcoin-node/Cargo.toml] | Serialize shared status/evidence contracts if Phase 111 extends them. [VERIFIED: packages/open-bitcoin-node/src/status/block_serving.rs] | Use only when extending shared evidence; avoid renderer-local evidence formats. [VERIFIED: .planning/phases/111-full-block-serving-request-path/111-CONTEXT.md] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
| --- | --- | --- |
| Phase 110 `open-bitcoin-network` policy plus a node-shell adapter [VERIFIED: packages/open-bitcoin-network/src/block_serving.rs] | Inline all checks inside `ManagedPeerNetwork::serve_inventory` [VERIFIED: packages/open-bitcoin-node/src/network/inventory.rs] | Inline checks would blur pure policy with storage/socket effects and contradict the functional-core boundary. [VERIFIED: standards/core/architecture.md, .planning/phases/111-full-block-serving-request-path/111-CONTEXT.md] |
| Existing first-party block codec [VERIFIED: packages/open-bitcoin-codec/src/block.rs] | Add or adopt an external Bitcoin serialization library [VERIFIED: AGENTS.md] | External Rust Bitcoin libraries are prohibited in the production path and would weaken parity ownership. [VERIFIED: AGENTS.md] |
| Deterministic local tests and optional Bun checker [VERIFIED: scripts/verify.sh] | Public-network or archive-node UAT as a default gate [VERIFIED: .planning/REQUIREMENTS.md] | Public-network and archive-node claims are explicitly out of scope for v2.1 default verification. [VERIFIED: .planning/REQUIREMENTS.md, .planning/phases/111-full-block-serving-request-path/111-CONTEXT.md] |

**Installation:** No new package installation is recommended. [VERIFIED: packages/Cargo.toml, .planning/phases/111-full-block-serving-request-path/111-CONTEXT.md]

```bash
# No new dependencies for Phase 111.
```

**Version verification:** Use local pinned versions instead of `npm view`, because Phase 111 should not add npm packages. [VERIFIED: packages/Cargo.toml, .bun-version, MODULE.bazel]

```bash
rustc --version
cargo --version
bun --version
bazel --version
cargo llvm-cov --version
```

## Architecture Patterns

### Recommended Project Structure

```text
packages/
  open-bitcoin-network/src/
    block_serving.rs                 # Existing pure policy and labels. [VERIFIED]
    peer/inventory_state.rs          # Existing getdata pressure gate and ServeInventory action. [VERIFIED]
  open-bitcoin-node/src/network/
    inventory.rs                     # Existing managed inventory adapter to tighten. [VERIFIED]
    block_serving.rs                 # Recommended new seam if logic exceeds a small helper. [VERIFIED: architecture recommendation from standards]
  open-bitcoin-codec/src/
    block.rs                         # Existing block/witness serialization. [VERIFIED]
scripts/
  check-phase111-block-serving-request-path.ts       # Add only if docs/parity/verifier change. [VERIFIED: Phase 110 checker pattern]
  check-phase111-block-serving-request-path.test.ts  # Mutation tests for checker. [VERIFIED: Phase 110 checker pattern]
```

### Pattern 1: Gate Before Local Block Read

**What:** Convert peer, inventory, chain/status, and local availability facts into Phase 110 decisions before touching `blocks_by_hash` or durable block data. [VERIFIED: packages/open-bitcoin-network/src/block_serving.rs, .planning/phases/111-full-block-serving-request-path/111-CONTEXT.md]

**When to use:** Every `InventoryType::Block` and `InventoryType::WitnessBlock` request in `ManagedPeerNetwork::serve_inventory`. [VERIFIED: packages/open-bitcoin-node/src/network/inventory.rs]

**Example:**

```rust
// Source: packages/open-bitcoin-network/src/block_serving.rs and
// packages/open-bitcoin-node/src/network/inventory.rs. [VERIFIED]
let status = classify_block_serving_status(&facts);
let eligibility = classify_block_serving_eligibility(&eligibility_input);
let gate = evaluate_block_serving_resource_gate(&policy, gate_input);

if !eligibility.eligible || !status.allow_storage_read || !gate.allow_storage_read {
    missing.push(request);
    continue;
}

let Some(block) = block_source.maybe_block(block_hash) else {
    missing.push(request);
    continue;
};

messages.push(WireNetworkMessage::Block(block));
```

### Pattern 2: Preserve Inventory Branch Ownership

**What:** Keep block, transaction, compact-block, and unknown inventory outcomes in separate typed branches. [VERIFIED: packages/open-bitcoin-node/src/network/inventory.rs, packages/open-bitcoin-node/src/network/relay_serving.rs]

**When to use:** Mixed `getdata` requests that contain block and transaction inventory must not regress transaction relay serving. [VERIFIED: .planning/phases/111-full-block-serving-request-path/111-CONTEXT.md, packages/open-bitcoin-node/src/network/inventory.rs]

**Example:**

```rust
// Source: packages/open-bitcoin-node/src/network/inventory.rs. [VERIFIED]
match request.inventory_type {
    InventoryType::Block | InventoryType::WitnessBlock => serve_block_request(request),
    InventoryType::Transaction | InventoryType::WitnessTransaction => serve_transaction_request(request),
    InventoryType::CompactBlock => record_deferred_compact_block(request),
    _ => missing.push(request),
}
```

### Pattern 3: Treat Compact Block as Classified and Non-Served

**What:** `InventoryType::CompactBlock` must be bounded by the same request path but must not emit `cmpctblock`, `block`, `getblocktxn`, or `blocktxn` payloads in Phase 111. [VERIFIED: .planning/phases/111-full-block-serving-request-path/111-CONTEXT.md, packages/bitcoin-knots/src/protocol.h]

**When to use:** Any `getdata` request with `InventoryType::CompactBlock`. [VERIFIED: packages/open-bitcoin-primitives/src/network.rs]

**Example:**

```rust
// Source: Phase 111 decision D-04 and primitives inventory constants. [VERIFIED]
InventoryType::CompactBlock => {
    latest_block_serving_outcomes.push(BlockServingOutcomeLabel::BlockServingSuppressed);
    missing.push(request);
}
```

### Pattern 4: Add a Witness Serialization Regression

**What:** Prove a served `WitnessBlock` request returns a `WireNetworkMessage::Block` whose encoded payload preserves witness data. [VERIFIED: packages/open-bitcoin-codec/src/block.rs, packages/open-bitcoin-network/src/message.rs]

**When to use:** Before claiming success criterion 1 for witness block inventory. [VERIFIED: .planning/phases/111-full-block-serving-request-path/111-CONTEXT.md]

**Example:**

```rust
// Source: packages/open-bitcoin-codec/src/block.rs and packages/open-bitcoin-network/src/message.rs. [VERIFIED]
let outbound = network.receive_message(peer_id, witness_getdata, now, flags, params)?;
let [WireNetworkMessage::Block(served)] = outbound.as_slice() else {
    panic!("expected witness block response");
};
let encoded = WireNetworkMessage::Block(served.clone()).encode_payload()?;
let decoded = WireNetworkMessage::decode_payload(&MessageCommand::new("block")?, &encoded)?;
assert_eq!(decoded, WireNetworkMessage::Block(original_block_with_witness));
```

### Anti-Patterns to Avoid

- **Direct cache-as-policy:** Reading `blocks_by_hash` before Phase 110 eligibility/status/resource decisions bypasses the locked Phase 111 boundary. [VERIFIED: .planning/phases/111-full-block-serving-request-path/111-CONTEXT.md, packages/open-bitcoin-node/src/network/inventory.rs]
- **Compact-relay scope creep:** Serving `InventoryType::CompactBlock` as `cmpctblock` or fallback `block` in Phase 111 would pre-implement Phase 112+ BIP152 behavior. [VERIFIED: .planning/ROADMAP.md, .planning/phases/111-full-block-serving-request-path/111-CONTEXT.md]
- **Archive-node inference from cache hit:** Serving an old cached block without status gating would violate GOV-05 and the bounded non-archive claim. [VERIFIED: .planning/REQUIREMENTS.md, .planning/phases/111-full-block-serving-request-path/111-CONTEXT.md]
- **Renderer-local labels:** New evidence labels must come from shared status/evidence contracts, not CLI/dashboard/RPC-specific formatting. [VERIFIED: packages/open-bitcoin-node/src/status/block_serving.rs, .planning/phases/111-full-block-serving-request-path/111-CONTEXT.md]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
| --- | --- | --- | --- |
| Request caps and queue pressure | A new block-serving queue or custom cap counter [VERIFIED: .planning/phases/111-full-block-serving-request-path/111-CONTEXT.md] | `ResourceGovernancePolicy`, `RequestPressureInput`, and existing peer-manager pressure checks [VERIFIED: packages/open-bitcoin-network/src/resource.rs, packages/open-bitcoin-network/src/peer/inventory_state.rs] | Phase 110 already maps resource outcomes to stable block-serving labels and keeps permissioned/protected peers bounded. [VERIFIED: packages/open-bitcoin-network/src/block_serving.rs, .planning/phases/110-block-serving-activation-and-eligibility-boundary/110-03-SUMMARY.md] |
| Block-serving policy | Ad hoc booleans inside `serve_inventory` [VERIFIED: packages/open-bitcoin-node/src/network/inventory.rs] | `classify_block_serving_eligibility`, `classify_block_serving_status`, `evaluate_block_serving_resource_gate` [VERIFIED: packages/open-bitcoin-network/src/block_serving.rs] | The pure policy already models eligibility, storage-read permission, serve permission, and stable labels. [VERIFIED: packages/open-bitcoin-network/src/block_serving.rs] |
| Block wire serialization | A second block encoder or special witness response type [VERIFIED: packages/open-bitcoin-network/src/message.rs] | `WireNetworkMessage::Block` plus `encode_block`/`parse_block` [VERIFIED: packages/open-bitcoin-network/src/message.rs, packages/open-bitcoin-codec/src/block.rs] | The existing block codec serializes transactions with witness data and the wire command remains `block`. [VERIFIED: packages/open-bitcoin-codec/src/block.rs, packages/bitcoin-knots/src/protocol.h] |
| Transaction serving in mixed `getdata` | Reimplement txid/wtxid lookup in the block adapter [VERIFIED: packages/open-bitcoin-node/src/network/inventory.rs] | Existing `RelayServingCache::classify_request` and transaction serving branch [VERIFIED: packages/open-bitcoin-node/src/network/relay_serving.rs] | Phase 104 already established typed transaction serving outcomes and lifecycle cleanup. [VERIFIED: .planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-CONTEXT.md, packages/open-bitcoin-node/src/network/relay_serving.rs] |
| BIP152 compact block payloads | `cmpctblock`, `getblocktxn`, `blocktxn`, or compact reconstruction [VERIFIED: .planning/ROADMAP.md] | Deterministic suppressed/unavailable/deferred outcome for `InventoryType::CompactBlock` [VERIFIED: .planning/phases/111-full-block-serving-request-path/111-CONTEXT.md] | Phases 112 through 115 own the BIP152 codec, negotiation, reconstruction, missing transaction round trip, and fallback. [VERIFIED: .planning/ROADMAP.md] |
| Redaction and evidence | Raw peer ids, endpoints, permission strings, prune heights, or payload details in status output [VERIFIED: .planning/phases/111-full-block-serving-request-path/111-CONTEXT.md] | `BlockServingEvidenceStatus` counters and fixed labels [VERIFIED: packages/open-bitcoin-node/src/status/block_serving.rs] | Existing evidence contracts are intentionally low-cardinality and sanitized. [VERIFIED: docs/architecture/operator-observability.md, packages/open-bitcoin-node/src/status/block_serving.rs] |

**Key insight:** The hard part is not block serialization; it is preserving the policy ordering so untrusted `getdata` input cannot force storage reads, payload sends, archive claims, or cap bypasses. [VERIFIED: packages/open-bitcoin-network/src/block_serving.rs, packages/open-bitcoin-network/src/peer/inventory_state.rs, packages/bitcoin-knots/src/net_processing.cpp]

## Common Pitfalls

### Pitfall 1: Direct `blocks_by_hash` Read Before Policy

**What goes wrong:** A peer can receive a cached block even when eligibility, status, or resource gates should suppress it. [VERIFIED: packages/open-bitcoin-node/src/network/inventory.rs, .planning/phases/111-full-block-serving-request-path/111-CONTEXT.md]

**Why it happens:** The current `serve_inventory` block branch reads `blocks_by_hash` directly for `Block` and `WitnessBlock`. [VERIFIED: packages/open-bitcoin-node/src/network/inventory.rs]

**How to avoid:** Add a block-serving adapter that requires `eligible`, status `Available`, `allow_storage_read`, `may_serve_block`, and local block presence before returning `WireNetworkMessage::Block`. [VERIFIED: packages/open-bitcoin-network/src/block_serving.rs]

**Warning signs:** Tests pass for a cached old/side-chain block without checking status labels. [VERIFIED: .planning/phases/111-full-block-serving-request-path/111-CONTEXT.md]

### Pitfall 2: Witness Request Claim Without Witness Data Proof

**What goes wrong:** A `WitnessBlock` request can be treated like `Block` while the test only compares headers or hashes. [VERIFIED: .planning/phases/111-full-block-serving-request-path/111-CONTEXT.md]

**Why it happens:** `WireNetworkMessage::Block` has one variant for both request types. [VERIFIED: packages/open-bitcoin-network/src/message.rs]

**How to avoid:** Build a block fixture with non-empty witness data, serve it through `InventoryType::WitnessBlock`, encode the outbound block payload, decode it, and compare the full block value. [VERIFIED: packages/open-bitcoin-codec/src/block.rs, packages/open-bitcoin-network/src/message.rs]

**Warning signs:** The test only asserts `matches!(WireNetworkMessage::Block(_))`. [VERIFIED: packages/open-bitcoin-node/src/network/tests.rs]

### Pitfall 3: Compact-Block Fallback Leakage

**What goes wrong:** `InventoryType::CompactBlock` starts producing a full block because Knots can fall back from old compact-block requests to full blocks. [VERIFIED: packages/bitcoin-knots/src/net_processing.cpp, packages/bitcoin-knots/test/functional/p2p_compactblocks.py]

**Why it happens:** Knots implements a mature BIP152 path where compact-block requests may produce `cmpctblock` or `block`; Phase 111 explicitly defers that path. [VERIFIED: packages/bitcoin-knots/src/net_processing.cpp, .planning/ROADMAP.md, .planning/phases/111-full-block-serving-request-path/111-CONTEXT.md]

**How to avoid:** Treat `CompactBlock` as classified and bounded but non-served until Phase 112+ owns wire semantics. [VERIFIED: .planning/phases/111-full-block-serving-request-path/111-CONTEXT.md]

**Warning signs:** New tests expect `WireNetworkMessage::Block` for compact-block inventory. [VERIFIED: .planning/phases/111-full-block-serving-request-path/111-CONTEXT.md]

### Pitfall 4: Local Data Presence Confused With Validated Availability

**What goes wrong:** Side-chain, stale, unvalidated, or pruned/unavailable blocks are served because bytes exist in cache or durable storage. [VERIFIED: .planning/REQUIREMENTS.md, .planning/phases/111-full-block-serving-request-path/111-CONTEXT.md]

**Why it happens:** `connect_stored_block` currently inserts duplicate, non-extending, disconnected, and connected blocks into `blocks_by_hash`, while only connected positions update active-chain state. [VERIFIED: packages/open-bitcoin-node/src/network.rs]

**How to avoid:** Build status facts from active-chain/recent-valid status before local data lookup, and make stale, side-chain, unvalidated, unknown, pruned, unavailable, and suppressed labels deny reads. [VERIFIED: packages/open-bitcoin-network/src/block_serving.rs]

**Warning signs:** A non-extending or disconnected block fixture is serveable from `blocks_by_hash`. [VERIFIED: packages/open-bitcoin-node/src/network.rs]

### Pitfall 5: `notfound` Without Internal Evidence

**What goes wrong:** Peer-facing `notfound` is emitted, but internal evidence cannot distinguish unknown, pruned, stale, side-chain, ineligible, suppressed, and request-cap cases. [VERIFIED: .planning/phases/111-full-block-serving-request-path/111-CONTEXT.md]

**Why it happens:** The current adapter only returns `(messages, missing)` from `serve_inventory`. [VERIFIED: packages/open-bitcoin-node/src/network/inventory.rs]

**How to avoid:** Add low-cardinality block-serving outcomes in the adapter or shared evidence path before pushing missing inventory into `NotFound`. [VERIFIED: packages/open-bitcoin-node/src/status/block_serving.rs, packages/open-bitcoin-network/src/block_serving.rs]

**Warning signs:** Tests only inspect outbound `NotFound` vectors and not the stable outcome labels. [VERIFIED: packages/open-bitcoin-node/src/network/tests.rs]

### Pitfall 6: Checker False Positives From Legacy Docs

**What goes wrong:** A new Phase 111 checker fails on historical no-claim text instead of the Phase 111 evidence surface. [VERIFIED: .planning/phases/110-block-serving-activation-and-eligibility-boundary/110-04-SUMMARY.md]

**Why it happens:** Older checkers scan fixed corpora and existing docs contain many deferred-surface phrases. [VERIFIED: scripts/check-phase110-block-serving-boundary.ts, docs/parity/checklist.md]

**How to avoid:** Model the Phase 111 checker on Phase 110 by scanning Phase-specific evidence units and allowing explicit no-claim/deferred wording. [VERIFIED: scripts/check-phase110-block-serving-boundary.ts, scripts/check-phase110-block-serving-boundary.test.ts]

**Warning signs:** The checker rejects a sentence that says Phase 111 does not claim archive-node behavior. [VERIFIED: scripts/check-phase110-block-serving-boundary.test.ts]

## Code Examples

Verified patterns from local sources:

### Existing Peer-Manager Request Pressure Before Serving

```rust
// Source: packages/open-bitcoin-network/src/peer/inventory_state.rs. [VERIFIED]
let input = request_pressure_input(
    peer,
    0,
    inventory.inventory.len(),
    0,
    peer.requested_blocks.len(),
    self.tx_download.peer_snapshot(peer_id).in_flight_count,
    0,
);
if let Some(actions) = resource_limit_disconnect_actions(input) {
    return Ok(actions);
}
Ok(vec![PeerAction::ServeInventory(/* typed inventory */)])
```

### Existing Status Gate Denies Reads Except Available

```rust
// Source: packages/open-bitcoin-network/src/block_serving.rs. [VERIFIED]
pub fn classify_block_serving_status(
    facts: &BlockServingStatusFacts,
) -> BlockServingStatusDecision {
    let label = classify_block_serving_status_label(facts);
    let may_serve_block = label == BlockServingStatusLabel::Available;

    BlockServingStatusDecision {
        label,
        allow_storage_read: may_serve_block,
        may_serve_block,
    }
}
```

### Existing Block Wire Encoding Uses Witness Encoding

```rust
// Source: packages/open-bitcoin-codec/src/block.rs. [VERIFIED]
pub fn encode_block(block: &Block) -> Result<Vec<u8>, CodecError> {
    let mut out = encode_block_header(&block.header);
    write_compact_size(&mut out, block.transactions.len() as u64)?;
    for transaction in &block.transactions {
        let encoded_transaction =
            encode_transaction(transaction, TransactionEncoding::WithWitness)?;
        out.extend_from_slice(&encoded_transaction);
    }
    Ok(out)
}
```

### Existing Transaction Serving Cache Pattern

```rust
// Source: packages/open-bitcoin-node/src/network/relay_serving.rs. [VERIFIED]
let maybe_status = self.status_for_request(request);
let decision =
    classify_tx_serve_request(request, peer_mode, relay_eligibility, maybe_status);
let maybe_transaction = match decision.outcome {
    TxServeOutcomeLabel::Served => decision
        .maybe_relay_id
        .and_then(|relay_id| self.transaction_for_relay_id(relay_id)),
    _ => None,
};
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
| --- | --- | --- | --- |
| Direct cached block serving from `blocks_by_hash` for full and witness block inventory [VERIFIED: packages/open-bitcoin-node/src/network/inventory.rs] | Phase 111 should gate block reads through Phase 110 eligibility/status/resource decisions first. [VERIFIED: packages/open-bitcoin-network/src/block_serving.rs, .planning/phases/111-full-block-serving-request-path/111-CONTEXT.md] | Phase 110 completed on 2026-07-04. [VERIFIED: .planning/phases/110-block-serving-activation-and-eligibility-boundary/110-VERIFICATION.md] | Plans must treat cache presence as local data only, not as serving policy. [VERIFIED: .planning/phases/111-full-block-serving-request-path/111-CONTEXT.md] |
| Transaction and block serving both used local maps in `serve_inventory`. [VERIFIED: packages/open-bitcoin-node/src/network/inventory.rs] | Transaction serving now uses `RelayServingCache::classify_request`; block serving needs a comparable seam. [VERIFIED: packages/open-bitcoin-node/src/network/relay_serving.rs] | Phase 104 established relay serving/fanout patterns in v2.0. [VERIFIED: .planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-CONTEXT.md] | Do not overload transaction relay APIs for blocks. [VERIFIED: .planning/phases/111-full-block-serving-request-path/111-CONTEXT.md] |
| Knots can respond to compact-block `getdata` with `cmpctblock` or fallback full block under BIP152 rules. [VERIFIED: packages/bitcoin-knots/src/net_processing.cpp, packages/bitcoin-knots/test/functional/p2p_compactblocks.py] | Phase 111 classifies `CompactBlock` as bounded non-served evidence and defers payload semantics. [VERIFIED: .planning/phases/111-full-block-serving-request-path/111-CONTEXT.md] | Phase 112+ owns BIP152 wire/relay work in the v2.1 roadmap. [VERIFIED: .planning/ROADMAP.md] | Tests must reject compact-block payload serving in Phase 111. [VERIFIED: .planning/phases/111-full-block-serving-request-path/111-CONTEXT.md] |
| Broad block-serving docs historically stayed deferred. [VERIFIED: docs/parity/checklist.md, docs/parity/index.json] | Phase 111 can claim bounded eligible full/witness block responses only for validated local data. [VERIFIED: .planning/ROADMAP.md, .planning/REQUIREMENTS.md] | Phase 111 is pending in v2.1. [VERIFIED: .planning/ROADMAP.md] | Docs and parity must avoid archive-node, public default, and production-readiness wording. [VERIFIED: .planning/REQUIREMENTS.md, .planning/phases/111-full-block-serving-request-path/111-CONTEXT.md] |

**Deprecated/outdated:**

- Treating `blocks_by_hash` as sufficient evidence for serving is outdated after Phase 110 because serving effects now require pure eligibility, status, and resource decisions before reads. [VERIFIED: packages/open-bitcoin-network/src/block_serving.rs, .planning/phases/111-full-block-serving-request-path/111-CONTEXT.md]
- Treating compact-block request support as part of full-block serving is out of scope because BIP152 codecs and relay semantics begin in Phase 112. [VERIFIED: .planning/ROADMAP.md, .planning/phases/111-full-block-serving-request-path/111-CONTEXT.md]

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
| --- | --- | --- | --- |
| None | All planning-critical claims in this research were verified against local project files, Phase artifacts, tool probes, or pinned Knots sources. [VERIFIED: sources listed below] | All sections | No user confirmation is required for assumed technical facts. [VERIFIED: sources listed below] |

## Resolved Questions

1. **Should Phase 111 read durable block storage or only introduce the seam over `blocks_by_hash`?** [VERIFIED: .planning/phases/111-full-block-serving-request-path/111-CONTEXT.md]
   - What we know: `FjallNodeStore` already has `save_block` and `load_block`, but `ManagedPeerNetwork<S>` currently stores chainstate through a `ChainstateStore` trait that does not expose `load_block`. [VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store.rs, packages/open-bitcoin-node/src/chainstate.rs]
   - RESOLVED: Phase 111 should implement a named cache-backed block-serving source seam first and defer durable `load_block` integration to a later storage-boundary task. The seam must separate availability/status facts from payload lookup so eligibility, status, and resource gates run before any block clone or durable read. [VERIFIED: standards/core/architecture.md, .planning/phases/111-full-block-serving-request-path/111-CONTEXT.md]

2. **Where should adapter outcomes be stored for Phase 111 evidence?** [VERIFIED: packages/open-bitcoin-node/src/status/block_serving.rs]
   - What we know: `BlockServingEvidenceStatus` currently has activation, eligibility counters, and status counters; it does not yet include per-request latest outcomes. [VERIFIED: packages/open-bitcoin-node/src/status/block_serving.rs]
   - RESOLVED: Phase 111 should keep per-request adapter outcomes local to the adapter/tests unless implementation changes a shared status, CLI, RPC, metrics, logs, or dashboard surface. If shared evidence becomes necessary, extend `BlockServingEvidenceStatus` first and make renderers consume that shared contract rather than formatting renderer-local labels. [VERIFIED: .planning/phases/111-full-block-serving-request-path/111-CONTEXT.md, docs/architecture/status-snapshot.md]

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
| --- | --- | --- | --- | --- |
| Rust compiler | Rust implementation and tests [VERIFIED: packages/Cargo.toml] | yes [VERIFIED: `rustc --version`] | `rustc 1.94.1 (e408947bf 2026-03-25)` [VERIFIED: `rustc --version`] | none needed [VERIFIED: rust-toolchain.toml] |
| Cargo | Workspace build/test/lint [VERIFIED: scripts/verify.sh] | yes [VERIFIED: `cargo --version`] | `cargo 1.94.1 (29ea6fb6a 2026-03-24)` [VERIFIED: `cargo --version`] | none needed [VERIFIED: scripts/verify.sh] |
| Bun | TypeScript checkers and tests [VERIFIED: scripts/verify.sh] | yes [VERIFIED: `bun --version`] | `1.3.9` [VERIFIED: `bun --version`, .bun-version] | Avoid checker changes if Bun becomes unavailable. [VERIFIED: scripts/verify.sh] |
| Bazel | Full verifier smoke build [VERIFIED: scripts/verify.sh] | yes [VERIFIED: `bazel --version`] | `8.6.0` [VERIFIED: `bazel --version`] | `bash scripts/verify.sh --fast` skips Bazel for iteration only. [VERIFIED: scripts/verify.sh] |
| `cargo-llvm-cov` | Full verifier pure-core coverage [VERIFIED: scripts/verify.sh] | yes [VERIFIED: `cargo llvm-cov --version`] | `0.8.5` [VERIFIED: `cargo llvm-cov --version`] | `bash scripts/verify.sh --fast` skips coverage for iteration only. [VERIFIED: scripts/verify.sh] |
| Bitcoin Knots submodule | Parity anchors [VERIFIED: .planning/phases/111-full-block-serving-request-path/111-CONTEXT.md] | yes [VERIFIED: `git submodule status packages/bitcoin-knots`] | `a9aee730466ac67d35a3c03ee24676be5e045878 (v29.3.knots20260210)` [VERIFIED: `git submodule status packages/bitcoin-knots`] | Run `git submodule update --init --recursive` if missing. [VERIFIED: AGENTS.md] |

**Missing dependencies with no fallback:** None found. [VERIFIED: tool probes above]

**Missing dependencies with fallback:** None found. [VERIFIED: tool probes above]

## Security Domain

Security enforcement is enabled because `.planning/config.json` does not set `security_enforcement` to `false`. [VERIFIED: .planning/config.json]

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
| --- | --- | --- |
| V2 Authentication | no [VERIFIED: Phase 111 scope and code scan] | No authentication surface is added by full block `getdata` serving. [VERIFIED: .planning/phases/111-full-block-serving-request-path/111-CONTEXT.md] |
| V3 Session Management | no [VERIFIED: Phase 111 scope and code scan] | No session state is added by peer inventory serving. [VERIFIED: packages/open-bitcoin-network/src/peer/inventory_state.rs] |
| V4 Access Control | yes [VERIFIED: Phase 110 policy and Phase 111 decisions] | Use `classify_block_serving_eligibility` and scoped permission effects before serving. [VERIFIED: packages/open-bitcoin-network/src/block_serving.rs] |
| V5 Input Validation | yes [VERIFIED: `getdata` request path] | Use typed `InventoryType`, request caps, status classification, and `ResourceGovernancePolicy`. [VERIFIED: packages/open-bitcoin-primitives/src/network.rs, packages/open-bitcoin-network/src/peer/inventory_state.rs, packages/open-bitcoin-network/src/block_serving.rs] |
| V6 Cryptography | no new crypto [VERIFIED: Phase 111 scope and code scan] | Use existing block hash and codec call paths; do not add custom cryptography. [VERIFIED: packages/open-bitcoin-node/src/network.rs, packages/open-bitcoin-codec/src/block.rs, packages/open-bitcoin-network/src/message.rs] |

### Known Threat Patterns for Block Serving

| Pattern | STRIDE | Standard Mitigation |
| --- | --- | --- |
| Over-cap `getdata` bursts | Denial of Service [VERIFIED: Phase 94/110 resource policy] | Keep `PeerManager::handle_getdata` request-pressure checks before `ServeInventory`, and preserve resource-governance disconnect evidence. [VERIFIED: packages/open-bitcoin-network/src/peer/inventory_state.rs, packages/open-bitcoin-network/src/peer/tests.rs] |
| Serving stale, side-chain, pruned, or unvalidated local data | Tampering / Information Disclosure [VERIFIED: Phase 111 decisions] | Require Phase 110 status `Available` before reads and return fixed labels for non-servable states. [VERIFIED: packages/open-bitcoin-network/src/block_serving.rs] |
| Prune height or peer-detail leakage | Information Disclosure [VERIFIED: Phase 111 decisions and status contracts] | Emit low-cardinality labels and counters; avoid raw prune heights, endpoints, permission strings, and payload data. [VERIFIED: packages/open-bitcoin-node/src/status/block_serving.rs, docs/architecture/operator-observability.md] |
| Compact-block request scope escalation | Elevation of Privilege / Tampering [VERIFIED: Phase 111 deferred scope] | Classify `InventoryType::CompactBlock` as suppressed/unavailable/deferred without `cmpctblock`, `getblocktxn`, or `blocktxn` payloads. [VERIFIED: .planning/phases/111-full-block-serving-request-path/111-CONTEXT.md, .planning/ROADMAP.md] |
| Archive/public-default overclaim | Spoofing / Information Disclosure [VERIFIED: Phase 111 decisions] | Add or extend deterministic checker coverage if docs/parity/verifier files change. [VERIFIED: scripts/check-phase110-block-serving-boundary.ts, .planning/phases/111-full-block-serving-request-path/111-CONTEXT.md] |

## Sources

### Primary (HIGH confidence)

- `.planning/phases/111-full-block-serving-request-path/111-CONTEXT.md` - locked Phase 111 decisions, deferred scope, canonical refs, and code context. [VERIFIED]
- `.planning/ROADMAP.md` - Phase 111 goal, plan count, success criteria, and v2.1 phase boundaries. [VERIFIED]
- `.planning/REQUIREMENTS.md` - BSRV-04, GOV-01, GOV-05, deferred requirements, and out-of-scope surfaces. [VERIFIED]
- `.planning/STATE.md` - current milestone state and repo-local UAT command reminder. [VERIFIED]
- `AGENTS.md`, `AGENTS.bright-builds.md`, `standards/core/architecture.md`, `standards/core/code-shape.md`, `standards/core/testing.md`, `standards/core/verification.md`, `standards/languages/rust.md`, `standards/languages/typescript-javascript.md` - project constraints and Bright Builds rules. [VERIFIED]
- `packages/open-bitcoin-network/src/block_serving.rs` and `packages/open-bitcoin-network/src/block_serving/tests.rs` - Phase 110 activation, eligibility, status, resource, cleanup contracts and tests. [VERIFIED]
- `packages/open-bitcoin-network/src/peer/inventory_state.rs` and `packages/open-bitcoin-network/src/peer/tests.rs` - current `getdata`, request-pressure, `ServeInventory`, block in-flight, `notfound`, and cleanup paths. [VERIFIED]
- `packages/open-bitcoin-node/src/network/inventory.rs`, `packages/open-bitcoin-node/src/network.rs`, and `packages/open-bitcoin-node/src/network/relay_serving.rs` - current managed inventory serving, block cache, action translation, and transaction serving pattern. [VERIFIED]
- `packages/open-bitcoin-node/src/status/block_serving.rs` - shared sanitized block-serving evidence contract. [VERIFIED]
- `packages/open-bitcoin-codec/src/block.rs`, `packages/open-bitcoin-network/src/message.rs`, and `packages/open-bitcoin-primitives/src/network.rs` - block/witness serialization and inventory type constants. [VERIFIED]
- `packages/open-bitcoin-node/src/storage/fjall_store.rs` and `packages/open-bitcoin-node/src/chainstate.rs` - durable block storage and current chainstate store trait boundary. [VERIFIED]
- `scripts/check-phase110-block-serving-boundary.ts`, `scripts/check-phase110-block-serving-boundary.test.ts`, and `scripts/verify.sh` - deterministic checker and verifier pattern. [VERIFIED]
- `packages/bitcoin-knots/src/protocol.h`, `packages/bitcoin-knots/src/net_processing.cpp`, `packages/bitcoin-knots/src/node/blockstorage.cpp`, `packages/bitcoin-knots/test/functional/p2p_getdata.py`, and `packages/bitcoin-knots/test/functional/p2p_compactblocks.py` - pinned Knots protocol and behavior anchors. [VERIFIED]

### Secondary (MEDIUM confidence)

- None; no web search was needed because Phase 111 is codebase-local and uses pinned Knots sources. [VERIFIED: research process]

### Tertiary (LOW confidence)

- None. [VERIFIED: research process]

## Metadata

**Confidence breakdown:**

- Standard stack: HIGH - all recommended components are existing repo dependencies or pinned local tools. [VERIFIED: packages/Cargo.toml, rust-toolchain.toml, MODULE.bazel, .bun-version]
- Architecture: HIGH - Phase 110 policy, existing managed node adapter, and Bright Builds functional-core rules all point to the same seam. [VERIFIED: packages/open-bitcoin-network/src/block_serving.rs, packages/open-bitcoin-node/src/network/inventory.rs, standards/core/architecture.md]
- Pitfalls: HIGH - pitfalls are backed by current code, Phase 111 locked decisions, and pinned Knots behavior. [VERIFIED: packages/open-bitcoin-node/src/network/inventory.rs, .planning/phases/111-full-block-serving-request-path/111-CONTEXT.md, packages/bitcoin-knots/src/net_processing.cpp]

**Research date:** 2026-07-04 [VERIFIED: current_date]
**Valid until:** 2026-08-03 for local architecture and toolchain findings unless Phase 111 or Phase 112 changes the same request path first. [VERIFIED: .planning/ROADMAP.md]
