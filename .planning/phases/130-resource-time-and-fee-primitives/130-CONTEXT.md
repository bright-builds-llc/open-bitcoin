---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 130-2026-07-23T14-26-46
generated_at: 2026-07-23T14:36:08.077Z
---

# Phase 130: Resource, Time, and Fee Primitives - Context

**Gathered:** 2026-07-23
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Establish explicit, non-overloaded contracts for mempool resource accounting, fee roles, acceptance and relay metadata, effect inputs, and committed lifecycle outcomes. Phase 130 makes these primitives deterministic and observable; Phase 131 owns pressure enforcement and rolling-fee behavior, while Phase 134 owns complete cross-cache projection through the runtime authority.

</domain>

<decisions>
## Implementation Decisions

### Resource accounting

- **D-01:** Represent transaction virtual size, accounted mempool memory, and configured capacity as distinct domain values; no numeric field may stand for more than one concept.
- **D-02:** Add a deterministic Rust-owned accounted-memory ledger with a documented accounting formula, cached aggregate, and recomputation oracle. It estimates owned mempool structures rather than imitating C++ allocator behavior.
- **D-03:** Keep Phase 130 limited to the accounting contract and truthful evidence. Phase 131 will use accounted memory for capacity enforcement, trimming, parity tolerances, and performance thresholds.
- **D-04:** Preserve Knots-compatible RPC meaning: `getmempoolinfo.bytes` is total transaction vsize, `usage` is accounted memory, and `maxmempool` is configured accounted-memory capacity.

### Fee-floor vocabulary

- **D-05:** Wrap the shared fee-rate representation in semantic role types for the static relay floor, incremental relay fee, and rolling mempool floor. Derive the effective admission floor rather than storing another mutable fee state.
- **D-06:** For individual admission and operator reporting, effective admission is the maximum of the static relay floor and rolling mempool floor.
- **D-07:** Eligible package aggregates may satisfy the rolling mempool floor, but ordinary members must still satisfy the static relay floor individually. Preserve only explicitly pinned Knots exceptions, including enforced-TRUC behavior selected by later package planning.
- **D-08:** Incremental relay fee remains a replacement and pressure-bump input; it is not an independent ordinary admission threshold.

### Entry metadata and explicit inputs

- **D-09:** Canonical mempool entries carry a typed acceptance timestamp plus typed origin and relay-intent metadata. Retry eligibility requires local origin, relay requested, and continued authoritative membership.
- **D-10:** Live admission samples acceptance time in the shell, recovery restores the persisted original acceptance time, and genuine reorg reacceptance receives the event's explicit current time. Recovery must not guess missing origin.
- **D-11:** Use operation-specific immutable contexts for admission, pressure, block, reorg, and retry decisions. Each context carries only the explicit time, block, occupancy, or jitter values relevant to that operation.
- **D-12:** Clocks and randomness stay in imperative-shell adapters. Pure mempool and network policy never reads wall-clock time or randomness directly.

### Typed lifecycle outcomes

- **D-13:** Define one cache-agnostic semantic `MempoolLifecycleDelta` for committed consequences, separate from validation or admission attempt results.
- **D-14:** The delta records admitted members, final post-transition membership, and typed removals that distinguish cause from direct-versus-descendant role. Causes cover replacement, expiry, pressure, block confirmation or conflict, reorg consequences, and retry-state clearing where applicable.
- **D-15:** Stable enum-derived labels are the only shared metrics, log, and support-evidence projection. Transaction identities and detailed member results remain confined to authenticated direct responses.
- **D-16:** Phase 130 defines semantic facts and ordering/deduplication invariants. Phase 134 projects those facts through `ManagedNetworkHandle` into serving, fanout, retry, persistence, compact-reconstruction, and evidence state without reclassifying outcomes.

### Claude's Discretion

- Exact Rust type and module names, provided the semantic roles remain compile-time distinct.
- The documented components of the deterministic accounted-memory formula and its internal cache layout.
- Whether operation contexts are structs or newtypes, provided they remain narrow and prevent irrelevant or invalid input combinations.
- Exact lifecycle-delta collection types and ordering representation, provided final membership, deterministic ordering, and deduplication are explicit.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Active milestone contract

- `.planning/ROADMAP.md` — Phase 130 boundary, success criteria, and separation from Phases 131 and 134.
- `.planning/REQUIREMENTS.md` — FEEP-01 through FEEP-05 and the v2.2 scope exclusions.
- `.planning/research/ARCHITECTURE.md` — Recommended package, mempool, authority, and effect-boundary architecture.
- `.planning/research/PITFALLS.md` — Resource-accounting, fee-floor, lifecycle, recovery, and parity hazards.
- `.planning/research/SUMMARY.md` — Synthesized v2.2 research conclusions and sequencing.

### Prior locked decisions

- `.planning/phases/05-mempool-and-node-policy/05-CONTEXT.md` — Existing mempool and policy model.
- `.planning/phases/103-mempool-chainstate-lifecycle-and-durable-recovery/103-CONTEXT.md` — Mempool-chainstate lifecycle and recovery boundaries.
- `.planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-CONTEXT.md` — Redacted aggregate relay evidence and fanout semantics.
- `.planning/phases/108-durable-mempool-relay-state-recovery/108-CONTEXT.md` — Durable mempool and relay recovery decisions.
- `.planning/phases/127-authoritative-network-state-unification/127-CONTEXT.md` — Single `ManagedNetworkHandle` mutation authority.
- `.planning/phases/129-integration-guardrails-and-milestone-reconciliation/129-CONTEXT.md` — Integration and release-boundary guardrails inherited from v2.1.

### Pinned Bitcoin Knots behavior

- `packages/bitcoin-knots/src/txmempool.h` — Mempool entry metadata, resource totals, and rolling-fee state.
- `packages/bitcoin-knots/src/txmempool.cpp` — Dynamic usage, removal reasons, trimming, rolling-floor bumps, and unbroadcast cleanup.
- `packages/bitcoin-knots/src/rpc/mempool.cpp` — `getmempoolinfo` bytes, usage, maxmempool, and mempoolminfee semantics.
- `packages/bitcoin-knots/src/validation.cpp` — Admission timing, fee checks, package applicability, and final membership.
- `packages/bitcoin-knots/src/validation.h` — Admission result vocabulary.
- `packages/bitcoin-knots/src/kernel/mempool_entry.h` — Canonical entry acceptance-time contract.
- `packages/bitcoin-knots/src/kernel/mempool_removal_reason.h` — Stable removal-cause vocabulary.
- `packages/bitcoin-knots/src/node/mempool_persist.cpp` — Persisted acceptance time and unbroadcast membership.
- `packages/bitcoin-knots/src/net_processing.cpp` — Relay retry scheduling, jitter sampling, serving, and unbroadcast transitions.
- `packages/bitcoin-knots/doc/policy/packages.md` — Package fee aggregation and static relay-floor boundaries.
- `packages/bitcoin-knots/test/functional/mempool_truc.py` — Pinned TRUC package-policy exceptions.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- `packages/open-bitcoin-mempool/src/types.rs` — Existing fee-rate, entry, admission, and limit types are the primary domain seam for typed primitives.
- `packages/open-bitcoin-mempool/src/outcome.rs` — Existing admission outcome vocabulary to separate from committed lifecycle facts.
- `packages/open-bitcoin-mempool/src/pool/lifecycle.rs` — Existing lifecycle snapshot and deferred rolling-fee surface.
- `packages/open-bitcoin-node/src/network/mempool_lifecycle.rs` — Current node-side lifecycle consequence projection.
- `packages/open-bitcoin-node/src/network/relay_fanout.rs` — Existing fixed-label, redacted relay evidence patterns.

### Established Patterns

- Pure mempool and network decisions accept explicit inputs; clocks, randomness, storage, and transport stay in shell adapters.
- `ManagedNetworkHandle` remains the sole runtime mutation authority.
- Shared evidence is bounded, low-cardinality, and redacted; identities stay in authenticated direct responses.
- Durable state preserves canonical source facts and rebuilds derived indexes.

### Integration Points

- `packages/open-bitcoin-mempool/src/pool.rs` — Accounted totals, fee-role use, admission context, and committed delta production.
- `packages/open-bitcoin-node/src/network/admission_bridge.rs` — Shell-sampled acceptance time and typed admission metadata.
- `packages/open-bitcoin-node/src/network/runtime_authority.rs` — Authoritative ownership boundary for future delta projection.
- `packages/open-bitcoin-node/src/storage/mempool_snapshot.rs` — Acceptance-time and local-unbroadcast source-state persistence.
- `packages/open-bitcoin-rpc/src/dispatch/node.rs` — Truthful `getmempoolinfo` resource and fee projections.
- `docs/parity/catalog/mempool-policy.md` — Auditable behavior anchors and intentional differences.

</code_context>

<specifics>
## Specific Ideas

- Prefer compile-time semantic separation over comments or repeated runtime checks.
- Keep exact C++ allocator parity out of scope; use a deterministic, auditable Rust-owned accounting model.
- Preserve observable Knots vocabulary and behavior while allowing simpler Rust internals.

</specifics>

<deferred>
## Deferred Ideas

- Accounted-memory enforcement, descendant-package eviction, rolling-fee bump and decay, and parity tolerance benchmarks — Phase 131.
- Package admission, package RBF, TRUC, and ephemeral-dust execution semantics — Phase 132.
- Complete lifecycle-delta projection across every dependent cache — Phase 134.
- Durable checkpoint schema and recovery implementation — Phase 135.
- Receive-independent retry scheduling and transport-receipt clearing — Phase 136.

</deferred>

***

*Phase: 130-resource-time-and-fee-primitives*
*Context gathered: 2026-07-23*
