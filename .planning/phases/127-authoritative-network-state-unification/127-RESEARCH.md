---
phase: 127-authoritative-network-state-unification
generated_by: gsd-phase-researcher
lifecycle_mode: yolo
phase_lifecycle_id: 127-2026-07-19T15-09-40
generated_at: 2026-07-19T15:35:00.000Z
status: complete
---

# Phase 127 Research

## Research Outcome

Phase 127 is a production-composition repair for GAP-01, FLOW-01, and FLOW-04. The required behavior already exists in phase-local pieces, but `open-bitcoind` constructs one blank RPC/inbound `ManagedPeerNetwork<MemoryChainstateStore>` and a separate durable-sync runtime that owns the chainstate, blocks, peers, and evidence that actually advance. Planning must remove that split, make durable block bodies available to the existing bounded serving policy, and prove every operator consumer reads the same authority.

The required plan coverage is `BSRV-03`, `BSRV-04`, `OBS-02`, and `OBS-04`. The `init plan-phase` helper currently reports `phase_req_ids: null`, so planner and checker prompts must carry these four explicit roadmap IDs.

## Runtime Findings

### Split Production Authority

- `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs` constructs `ManagedRpcContext` and its network before separately constructing `DurableSyncRuntime`.
- `packages/open-bitcoin-rpc/src/context/network.rs` builds a fresh `MemoryChainstateStore`; its durable recovery restores mempool state but not the durable sync network and chainstate.
- `packages/open-bitcoin-node/src/sync.rs` restores chainstate into the `ManagedPeerNetwork` owned by `DurableSyncRuntime`.
- RPC status, inbound listener dispatch, CLI/dashboard data, and support evidence therefore observe a different mutable network than durable sync.

Recommended shape:

1. Introduce a small node-owned shared authority around exactly one `ManagedPeerNetwork<MemoryChainstateStore>`.
2. Let `DurableSyncRuntime` create or recover that authority and expose only a cloneable narrow handle.
3. Construct `ManagedRpcContext` and inbound handling from that same handle.
4. Provide typed closure/command methods and owned snapshots rather than public lock guards.
5. Keep synchronous critical sections short and never hold a guard across `.await`, socket I/O, Fjall operations, JSON serialization, logging, or metric persistence.
6. Map poison/authority failures into explicit runtime errors instead of panicking or silently returning stale status.

A dedicated actor/coordinator would provide stronger single ownership but is too broad for this gap: the current blocking sync loop, inbound async path, RPC requests, shutdown ordering, bounded queues, and cancellation semantics would all need redesign. Retaining two networks with versioned snapshots does not meet the phase criterion.

### Durable Block Serving

- `ManagedPeerNetwork` already owns activation, eligibility, validation/status decisions, request caps, queue pressure, compact state, peer cleanup, and block-relay evidence.
- `packages/open-bitcoin-node/src/network/inventory.rs` currently uses `blocks_by_hash` as the body source, so a durable block can become unavailable to inbound serving after restart or cache loss.
- `FjallNodeStore::load_block` and the sync block-response path already provide durable body access.

Recommended shape:

1. Preserve policy and validation classification inside the authoritative network.
2. After the gate passes, return a typed serve intent or requested hash to the node shell.
3. Load the body lazily from a narrow `FjallNodeStore`-backed block-source seam.
4. Complete existing send/cleanup/evidence transitions against the same authoritative network after the durable lookup and transport outcome.
5. Treat `blocks_by_hash` as a cache or focused-test source, not production durable truth.
6. Map missing/corrupt/store failures to the existing unavailable or suppressed vocabulary and preserve caps, cleanup, and achieved-effect ordering.

Do not preload the durable block corpus or expand claims to archive-node behavior.

### Operator Projection

- Existing `BlockRelayEvidenceStatus`, RPC method response types, CLI/dashboard models, metrics/log labels, and support redaction already satisfy the intended public contract.
- Phase 127 should change provenance, not schema or presentation.
- Snapshot the authoritative network once into an owned aggregate value, release the guard, then serialize/render/persist outside the critical section.
- Retain availability gates, fixed low-cardinality labels, and redaction of endpoints, permissions, credentials, transactions, and dynamic peer labels.

## Security And Concurrency Threat Model

| Threat | Severity | Mitigation |
| --- | --- | --- |
| Deadlock or starvation from a network guard held across I/O or `.await` | High | Hide guards, use short synchronous closure methods, and add lock-boundary review/tests. |
| Split-brain regression through a second production constructor | High | Centralize production construction and add a deterministic source mutation checker. |
| TOCTOU between eligibility and durable lookup | Medium | Carry a typed serve decision and re-enter the same authority only for completion/evidence; fail closed on lookup failure. |
| Memory amplification from eager durable hydration | Medium | Use lazy request-scoped `load_block`; keep the in-memory map cache-only. |
| Sensitive material leaking through unified status | High | Reuse existing aggregate contracts and redaction; add regression assertions for forbidden endpoint/permission/transaction content. |
| Poisoned shared authority causing panic or silent stale data | High | Propagate a typed runtime error and render unavailable status; never `unwrap`. |

Every plan should include a `<threat-model>` block and block on unresolved high-severity threats.

## Recommended Plan Structure

### Plan 127-01: Shared Authoritative Network Handle

Own the node-side authority abstraction, sync-runtime integration, and focused unit tests. Prove one allocation can be shared safely, poison/errors propagate, and owned snapshots do not expose guards.

Likely files:

- `packages/open-bitcoin-node/src/network.rs`
- a focused new node module only if needed, with parity breadcrumb registration
- `packages/open-bitcoin-node/src/sync.rs`
- related sync/network test modules
- `packages/open-bitcoin-node/src/lib.rs`

### Plan 127-02: Durable Serving Source

Depend on 127-01. Split existing serving into policy/intent and shell-side durable lookup/completion without changing caps or cleanup. Cover full, witness, and compact block requests plus missing/corrupt failure cases and restart-shaped availability.

Likely files:

- `packages/open-bitcoin-node/src/network.rs`
- `packages/open-bitcoin-node/src/network/inventory.rs`
- `packages/open-bitcoin-node/src/network/block_serving.rs`
- `packages/open-bitcoin-node/src/storage/fjall_store.rs`
- focused node tests and any new breadcrumb entries

### Plan 127-03: Daemon Composition And Operator Projection

Depend on 127-01 and 127-02. Reorder daemon startup, construct RPC/inbound from the shared handle and store, preserve response schemas, and add production-path integration tests proving sync mutations and durable bodies reach inbound/RPC consumers.

Likely files:

- `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs`
- `packages/open-bitcoin-rpc/src/context.rs`
- `packages/open-bitcoin-rpc/src/context/network.rs`
- `packages/open-bitcoin-rpc/src/inbound_listener.rs`
- `packages/open-bitcoin-rpc/src/dispatch/node.rs`
- focused RPC/integration tests

### Plan 127-04: Parity And Deterministic Phase Guard

Depend on 127-03. Add Phase 127 parity evidence and a Bun checker with mutation coverage rejecting duplicate production construction, fresh RPC chainstate, cache-only serving, or non-authoritative projection. Wire it into `scripts/verify.sh`. Keep Phase 129 aggregate reconciliation out of scope.

Likely files:

- `docs/parity/index.json`
- `docs/parity/source-breadcrumbs.json` when new Rust files are added
- `docs/parity/catalog/p2p.md`
- `scripts/check-phase127-authoritative-network-state-unification.ts`
- `scripts/check-phase127-authoritative-network-state-unification.test.ts`
- `scripts/verify.sh`
- `docs/metrics/lines-of-code.md`

## Verification Strategy

- Focused Rust tests for the shared authority, durable serve intent/load/completion, failure mapping, redaction, and production composition.
- Run ad-hoc Cargo/Bazel commands through `bun run scripts/command-timings.ts run --key <stable-key> -- <command>`.
- Use deterministic TypeScript mutation tests for structural production wiring.
- Run `cargo fmt --all`, Clippy, all-target build, and all-feature tests through the repo contract; the final required gate is `bash scripts/verify.sh`.
- Default verification must not contact the public Bitcoin network.
- If new first-party Rust source or test files are added, register parity breadcrumbs before the checker runs.

## Parity Anchors

- `packages/bitcoin-knots/src/node/context.h` — centralized node context ownership.
- `packages/bitcoin-knots/src/rpc/server_util.cpp` — RPC access to authoritative node-context managers.
- `packages/bitcoin-knots/src/net_processing.cpp` — validation-aware block serving and peer-manager state.
- `packages/bitcoin-knots/src/validation.cpp` — active-chain and validation authority.
- `packages/bitcoin-knots/src/node/blockstorage.cpp` — durable block-body loading.

## Planning Risks

- Avoid a coarse shared lock that spans the whole sync round or inbound message transport.
- Avoid cloning `ManagedPeerNetwork`; clone only the handle.
- Avoid converting every existing unit test to Fjall. Keep explicit in-memory/test sources while making production composition durable.
- Avoid operator schema churn; it creates unrelated compatibility work without closing GAP-01.
- Avoid promoting requirements or reconciling the milestone in Phase 127. Phase 129 owns aggregate verification and final audit state.
- Preserve Phase 128 ownership of compact negotiation and post-write announcement transport.

## Research Complete

The repository contains the necessary primitives. The plan should sequence shared authority first, durable serving second, production composition/projection third, and deterministic parity guardrails last.
