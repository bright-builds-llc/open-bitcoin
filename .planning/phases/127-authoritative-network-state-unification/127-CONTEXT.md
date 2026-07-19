---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 127-2026-07-19T15-09-40
generated_at: 2026-07-19T15:09:40.422Z
---

# Phase 127: Authoritative Network State Unification - Context

**Gathered:** 2026-07-19
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Close GAP-01, FLOW-01, and FLOW-04 by making durable sync, inbound peer handling, RPC, CLI/dashboard, and support evidence use one authoritative production network, chainstate, durable block source, and block-relay evidence source.

This phase repairs production composition and data provenance. It does not add compact-announcement negotiation or transport, broaden relay or serving defaults, add public-network verification, redesign operator presentation, or perform the final v2.1 milestone reconciliation. Those responsibilities remain with Phases 128 and 129.

</domain>

<decisions>
## Implementation Decisions

### Authoritative Runtime Ownership

- **D-01:** Production `open-bitcoind` must construct exactly one authoritative `ManagedPeerNetwork` allocation for durable sync, inbound peer handling, and RPC status. Remove the current independently constructed RPC/inbound network.
- **D-02:** Share that allocation through a narrow node-owned runtime handle rather than exposing raw network clones. The handle may use `Arc` plus a synchronous mutex or an equivalently bounded mechanism, but its public API must express typed mutations and owned read-only snapshots.
- **D-03:** Keep critical sections short. No network lock may be held across socket reads or writes, `.await`, durable Fjall operations, RPC serialization, or other potentially blocking effects.
- **D-04:** Prefer this proportional shared-handle repair over a new actor/coordinator architecture. A future actor conversion remains possible, but Phase 127 must not absorb the broader command protocol, queue, cancellation, and shutdown redesign.
- **D-05:** Preserve the functional-core/imperative-shell boundary: network and chainstate rules remain in the existing typed core, while the daemon and node runtime own synchronization, storage access, inbound transport, and RPC projection.

### Durable Block Serving Source

- **D-06:** Use the shared authoritative chainstate to decide active/recent-valid eligibility and validation. Only after those existing policy and resource gates pass may the node shell load a block body.
- **D-07:** Production block-body availability must come from a narrow durable block-source seam backed by `FjallNodeStore::load_block`. The in-memory `blocks_by_hash` collection may remain a cache or focused-test source, but it must not be the production authority for blocks already persisted by durable sync.
- **D-08:** Durable lookup is lazy and request-scoped. Do not hydrate the full durable block corpus into memory at startup or imply archive-node behavior.
- **D-09:** Missing, corrupt, or failed durable reads fail closed through the existing unavailable/suppressed serving vocabulary. They must not bypass request caps, queue backpressure, peer cleanup, or evidence ordering.
- **D-10:** Full-block, witness-block, and compact-block request paths must reuse the same authoritative eligibility, block source, bounded request, and cleanup boundary so restart or cache loss cannot make validated durable blocks invisible to inbound peers.

### Shared Operator Truth

- **D-11:** Preserve the existing RPC response schemas, `BlockRelayEvidenceStatus`, CLI/dashboard models, metrics/log labels, and support-bundle formats. Phase 127 changes their provenance, not their presentation.
- **D-12:** Produce one owned aggregate snapshot from the authoritative network for RPC, CLI/dashboard, metrics/logs, and support consumers. Do not retain a second mutable projection network or treat eventual snapshot agreement as equivalent to shared authority.
- **D-13:** Retain availability gating, fixed low-cardinality aggregation, and current redaction rules. Raw endpoints, permission strings, credentials, transaction payloads, and dynamic peer labels remain excluded.
- **D-14:** Snapshot and serialization work must occur outside the network critical section whenever practical so operator polling cannot stall sync or inbound transport.

### Production-Path Guardrails

- **D-15:** Add focused production-path integration tests that prove durable sync mutations are visible to inbound serving and RPC status through the same authoritative handle, including a same-datadir durable-block serving case after cache loss or restart-shaped recovery.
- **D-16:** Add a narrow deterministic Phase 127 checker with mutation coverage that rejects duplicate production network construction, a fresh `MemoryChainstateStore` in the RPC production path, cache-only production serving, and non-authoritative block-relay projection.
- **D-17:** Keep Phase 127 guards scoped to GAP-01 and its two broken flows. Phase 129 still owns the aggregate four-flow integration guard, requirement promotion reconciliation, and final milestone audit decision.
- **D-18:** Default verification remains deterministic and public-network-free. No production readiness, public serving default, archive-node, package relay, filter-serving, or production-funds wallet claim may be inferred from this repair.

### Folded Todos

No pending todos matched Phase 127.

### the agent's Discretion

The planner may choose the exact shared-handle type, poison/error vocabulary, snapshot method names, durable block-source trait shape, cache interaction, module split, and focused test fixtures. Prefer the smallest API surface that makes duplicate authority difficult to represent, preserves existing public contracts, avoids locks across effects, and provides deterministic Arrange/Act/Assert regression evidence.

</decisions>

<canonical-refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Project Rules And Phase Contract

- `AGENTS.md` — repo-local GSD, parity breadcrumb, verification, generated-artifact, and command-timing rules.
- `AGENTS.bright-builds.md` — managed Bright Builds workflow and cross-cutting standards.
- `standards-overrides.md` — local exceptions; no substantive override currently applies.
- `standards/core/architecture.md` — functional-core/imperative-shell, boundary parsing, and illegal-state rules.
- `standards/core/operability.md` — runtime ownership, failure visibility, and operator-facing behavior guidance.
- `standards/core/testing.md` — focused Arrange/Act/Assert requirements.
- `standards/core/verification.md` — sync-first and repo-native verification gates.
- `standards/languages/rust.md` — Rust invariant, module, optional-name, adapter, and verification guidance.
- `.planning/ROADMAP.md` § Phase 127 — fixed goal, dependency, requirements, and success criteria.
- `.planning/REQUIREMENTS.md` — normative `BSRV-03`, `BSRV-04`, `OBS-02`, and `OBS-04` definitions and ownership.
- `.planning/PROJECT.md` — bounded v2.1 claim and active integration-gap boundary.
- `.planning/STATE.md` — current Phase 127 route and milestone continuity.
- `.planning/v2.1-MILESTONE-AUDIT.md` — canonical GAP-01, FLOW-01, FLOW-04, integration-link, and release-blocking evidence.

### Prior Open Bitcoin Decisions

- `.planning/phases/110-block-serving-activation-and-eligibility-boundary/110-CONTEXT.md` — activation, eligibility, validation, availability, and resource-governance decisions.
- `.planning/phases/111-full-block-serving-request-path/111-CONTEXT.md` — policy-before-read, validated local-data serving, request caps, and cleanup decisions.
- `.planning/phases/116-operator-evidence-metrics-logs-and-support-boundary/116-CONTEXT.md` — shared status, redaction, low-cardinality labels, and support evidence contracts.
- `.planning/phases/121-block-relay-metrics-and-log-runtime-projection/121-CONTEXT.md` — authoritative block-relay metric and structured-log projection.
- `.planning/phases/123-runtime-timing-and-evidence-integrity/123-CONTEXT.md` — direct authoritative sampling and achieved-effect evidence.
- `.planning/phases/126-compact-relay-residual-hardening/126-CONTEXT.md` — current production receive, durable evidence, deterministic guard, and deferred-scope decisions.

### Runtime Composition, Serving, And Projection

- `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs` — daemon construction order and the current split RPC/inbound versus durable-sync runtimes.
- `packages/open-bitcoin-rpc/src/context.rs` — `ManagedRpcContext` ownership and shared operator context.
- `packages/open-bitcoin-rpc/src/context/network.rs` — current fresh production `MemoryChainstateStore`, network status, metrics-store, and mempool recovery paths.
- `packages/open-bitcoin-rpc/src/inbound_listener.rs` — inbound peer handling and lock/transport boundary.
- `packages/open-bitcoin-rpc/src/dispatch/node.rs` — RPC node status projection.
- `packages/open-bitcoin-rpc/src/method/node.rs` — stable node RPC response contracts.
- `packages/open-bitcoin-node/src/sync.rs` — `DurableSyncRuntime`, durable chainstate recovery, authoritative network mutations, metrics, and logs.
- `packages/open-bitcoin-node/src/sync/block_response.rs` — persisted block response and durable block-body access behavior.
- `packages/open-bitcoin-node/src/sync/runtime_state.rs` — durable runtime-state projection.
- `packages/open-bitcoin-node/src/network.rs` — `ManagedPeerNetwork`, serving, compact receive, and block-relay evidence ownership.
- `packages/open-bitcoin-node/src/network/inventory.rs` — current in-memory block inventory and serving lookup.
- `packages/open-bitcoin-node/src/network/block_relay_evidence.rs` — authoritative runtime evidence state and snapshots.
- `packages/open-bitcoin-node/src/storage/fjall_store.rs` — clonable Fjall runtime store and durable block loading.
- `packages/open-bitcoin-node/src/status/block_relay_evidence.rs` — stable aggregate operator evidence contract.
- `packages/open-bitcoin-cli/src/operator/dashboard/model/block_relay.rs` — dashboard projection of shared block-relay status.
- `packages/open-bitcoin-cli/src/operator/support/redaction.rs` — support-bundle sanitization boundary.

### Pinned Bitcoin Knots Anchors

- `packages/bitcoin-knots/src/node/context.h` — centralized node context ownership for chain and peer managers.
- `packages/bitcoin-knots/src/rpc/server_util.cpp` — RPC resolution of authoritative node-context references.
- `packages/bitcoin-knots/src/net_processing.cpp` — validation-aware block serving, peer request handling, and centralized peer-manager state.
- `packages/bitcoin-knots/src/validation.cpp` — active-chain and validation-status authority.
- `packages/bitcoin-knots/src/node/blockstorage.cpp` — durable block-body loading boundary.

### Deterministic Verification

- `scripts/verify.sh` — required full repository verification contract and checker ordering.
- `scripts/check-active-milestone-verification-traceability.ts` — lifecycle-valid requirement verification ownership.
- `scripts/check-phase126-compact-relay-residual-hardening.ts` — current narrow phase guard and mutation-test precedent.

</canonical-refs>

<code-context>
## Existing Code Insights

### Reusable Assets

- `DurableSyncRuntime` already recovers chainstate into its owned `ManagedPeerNetwork` and exposes the canonical `FjallNodeStore`; it is the natural source from which the shared production handle should be derived.
- `ManagedPeerNetwork` already owns block-serving policy, bounded request state, compact reconstruction state, peer cleanup, and block-relay evidence.
- `FjallNodeStore::load_block` and the sync block-response path already provide durable block-body access without a new storage dependency.
- Existing `BlockRelayEvidenceStatus`, RPC method types, CLI/dashboard models, metrics/log projection, and support redaction can remain unchanged.

### Established Patterns

- Pure policy and state transitions remain inside node/network types; daemon, storage, socket, clock, and serialization effects remain in shell adapters.
- Full-block serving applies eligibility and status gates before local-data reads and records achieved-effect evidence only after successful writes.
- Operator consumers use stable availability wrappers and aggregate/redacted contracts.
- Substantial repo-owned guards use Bun/TypeScript with deterministic mutation coverage and run through `scripts/verify.sh`.

### Integration Points

- Reorder `open-bitcoind` startup so durable runtime initialization establishes the one network authority before RPC/inbound consumers are constructed.
- Replace direct network ownership in `ManagedRpcContext` and `DurableSyncRuntime` with the narrow shared authority without leaking lock guards into callers.
- Route inbound serving body lookups through the durable block-source seam while retaining network-owned policy, request, and cleanup state.
- Snapshot authoritative status for RPC and downstream consumers, then serialize and render outside critical sections.
- Wire a Phase 127 production-composition checker into `scripts/verify.sh`; leave aggregate milestone reconciliation to Phase 129.

</code-context>

<specifics>
## Specific Ideas

- Treat runtime identity as a construction invariant: one production network allocation, many narrow handles, no independent production constructors.
- Treat `blocks_by_hash` as a cache or test fixture, never as proof that durable sync does not possess a block.
- Keep lock scope explicit in API shape so no `.await`, socket I/O, Fjall load, or JSON serialization can accidentally occur under the network guard.
- Preserve user-facing status and support output exactly where possible; only the authoritative provenance should change.

</specifics>

<deferred>
## Deferred Ideas

- A dedicated single-owner network actor or typed coordinator may be reconsidered when a broader runtime-concurrency redesign is justified.
- Production compact negotiation, announcement construction, transport emission, and post-write compact evidence remain Phase 128.
- Aggregate four-flow integration guards, requirement promotion, final audit reconciliation, and archive routing remain Phase 129.
- Package relay, bloom/filter serving, compact filters, public serving defaults, public-network CI, archive-node claims, production full-node readiness, production-funds wallet use, migration apply mode, packaging, hosted services, and GUI work remain outside v2.1.

</deferred>

***

*Phase: 127-authoritative-network-state-unification*
*Context gathered: 2026-07-19*
