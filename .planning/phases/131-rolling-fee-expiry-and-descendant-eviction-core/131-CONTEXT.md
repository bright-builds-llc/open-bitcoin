---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 131-2026-07-24T22-07-47
generated_at: 2026-07-24T22:09:03.816Z
---

# Phase 131: Rolling Fee, Expiry, and Descendant Eviction Core - Context

**Gathered:** 2026-07-24
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Enforce sustained-pressure capacity, descendant-package eviction, rolling-fee bump and block-gated decay, and acceptance-time expiry so the mempool stays bounded and internally consistent while matching pinned Knots trim / trackPackageRemoved / GetMinFee / Expire behavior. Phase 131 owns pure mempool pressure and rolling-fee mechanics plus the minimum shell wiring to inject time and surface truthful evidence. Package admission stays in Phase 132; complete cross-cache lifecycle projection stays in Phase 134; durable rolling-fee persistence stays out (restart resets rolling fee per milestone contract).

</domain>

<decisions>
## Implementation Decisions

### Capacity enforcement

- **D-01:** Enforce configured mempool capacity from accounted memory usage against `MempoolCapacity`. Virtual size remains a distinct fee and reporting measure and must not drive trim.
- **D-02:** Retire `PolicyConfig.legacy_vsize_trim_limit` as the active trim limiter. Update capacity-enforcement evidence from `legacy_vsize` to accounted-memory enforcement.
- **D-03:** Trim continues until accounted usage is within capacity. Over-capacity classification and ledger aggregates stay consistent with the Phase 130 recomputation oracle.

### Descendant-package eviction and rolling bump

- **D-04:** Pressure selects the lowest Knots-compatible descendant-score package (existing `descendant_score` ordering, then txid tie-break) and removes that victim plus all descendants as one package.
- **D-05:** On each pressure package removal, raise `RollingMempoolFeeRate` from the actual evicted package feerate plus incremental relay fee, matching Knots `trackPackageRemoved` / `TrimToSize` bump semantics. Clear any block-since-last-bump gate so decay cannot start until a later block connect.
- **D-06:** Preserve Phase 130 fee-role boundaries: incremental remains a replacement and pressure-bump input; effective admission stays `max(static, rolling)`; do not store effective as mutable state; do not let package aggregates bypass the wrong floor.

### Block-gated rolling decay

- **D-07:** Rolling-floor decay is block-gated only. No wall-clock decay without a connected-block lifecycle context. Pure policy uses `BlockLifecycleContext.connected_at` (and occupancy) rather than reading clocks.
- **D-08:** Match pinned Knots half-life and rounding behavior: 12-hour default half-life, shortened to 6-hour when usage is below half capacity and 3-hour when below quarter capacity; floor rolling to zero below the incremental/2 boundary per Knots `GetMinFee` semantics where those rules affect the rolling state.
- **D-09:** Operator evidence continues to expose static, incremental, rolling, and effective roles separately. Rolling-fee parity status must leave `Deferred` once bump and decay are live.

### Expiry and index cleanup

- **D-10:** Add a pure mempool expiry API that takes explicit `PolicyTime` (or a narrow expiry context) and removes entries whose acceptance age exceeds policy, emitting `MempoolRemovalCause::Expiry` with Direct/Descendant roles.
- **D-11:** Expiry and pressure removals must leave no stale descendants or derived indexes; always remove through existing topology helpers and `recompute_state` / resource-ledger recompute so graph and fee-aggregate invariants hold.
- **D-12:** Shell adapters sample current time and invoke expiry through `ManagedNetworkHandle` / managed network authority. Pure core never reads wall-clock time. Do not invent acceptance times for `LegacyUnknown` entries—fail closed or skip per Phase 130 metadata rules.

### Determinism, oracle, and bounds

- **D-13:** Deterministic fill, trim, block, decay, expiry, refill, and reorg scenarios must agree with recomputation oracles for membership, accounted usage, and rolling fee after each committed transition.
- **D-14:** Document resource and performance bounds for sustained-pressure sequences and enforce them with hermetic tests in the default verifier. No public-network or non-deterministic soak gates.
- **D-15:** Rolling fee remains non-durable for this phase: restart baseline stays zero unless a later durability phase redesigns persistence (MPDUR / Phase 135 territory).

### Claude's Discretion

- Exact module split for trim/bump/decay/expiry helpers inside `open-bitcoin-mempool`, provided the public semantic contracts stay clear.
- Internal representation of block-since-last-bump and last-rolling-fee-update state, provided decay stays block-gated and occupancy-sensitive.
- Whether pressure trim accepts `PressureDecisionContext` as a required argument or threads occupancy through existing ledger + capacity fields, provided pure code still never samples clocks.
- Exact performance threshold numbers, provided they are documented, hermetic, and fail the default verifier when exceeded.
- Temporary retention of `set_rolling_mempool_fee_rate` for tests until internal bump/decay fully own the state machine.

### Folded Todos

None — no pending todos matched this phase.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Active milestone contract

- `.planning/ROADMAP.md` — Phase 131 goal, success criteria, and separation from Phases 132–135.
- `.planning/REQUIREMENTS.md` — PRESS-01 through PRESS-05 (and completed FEEP-01 through FEEP-05 prerequisites).
- `.planning/research/ARCHITECTURE.md` — Mempool authority, effect-boundary, and package sequencing architecture.
- `.planning/research/PITFALLS.md` — Pressure, rolling-fee, expiry, and parity hazards.
- `.planning/research/SUMMARY.md` — Synthesized v2.2 research conclusions and sequencing.

### Prior locked decisions

- `.planning/phases/130-resource-time-and-fee-primitives/130-CONTEXT.md` — Resource, fee-role, context, and lifecycle primitives this phase activates.
- `.planning/phases/108-durable-mempool-relay-state-recovery/108-CONTEXT.md` — Durable source facts vs rebuilt derived state.
- `.planning/phases/103-mempool-chainstate-lifecycle-and-durable-recovery/103-CONTEXT.md` — Mempool-chainstate lifecycle boundaries.
- `.planning/phases/127-authoritative-network-state-unification/127-CONTEXT.md` — Single `ManagedNetworkHandle` mutation authority.
- `.planning/phases/129-integration-guardrails-and-milestone-reconciliation/129-CONTEXT.md` — Deterministic verifier and claim-boundary guardrails.
- `.planning/phases/05-mempool-and-node-policy/05-CONTEXT.md` — Baseline mempool/policy model.

### Pinned Bitcoin Knots behavior

- `packages/bitcoin-knots/src/txmempool.cpp` — `TrimToSize`, `trackPackageRemoved`, `GetMinFee`, `Expire`, descendant-score eviction, rolling bump/decay.
- `packages/bitcoin-knots/src/txmempool.h` — Rolling-fee state, entry indexes, removal APIs.
- `packages/bitcoin-knots/src/kernel/mempool_removal_reason.h` — EXPIRY / SIZE / etc. removal vocabulary.
- `packages/bitcoin-knots/src/kernel/mempool_entry.h` — Acceptance-time / entry_time contract.
- `packages/bitcoin-knots/src/rpc/mempool.cpp` — `getmempoolinfo` bytes, usage, maxmempool, mempoolminfee / minrelaytxfee surfaces.
- `packages/bitcoin-knots/src/validation.cpp` — Admission and final membership relative to min fee and limits.
- `packages/bitcoin-knots/test/functional/mempool_limit.py` — Mempool limit / trim behavioral anchors.

### Open Bitcoin seams and parity docs

- `packages/open-bitcoin-mempool/src/pool.rs` — Current `trim_to_size`, `select_eviction_candidate`, admission trim hook.
- `packages/open-bitcoin-mempool/src/types.rs` — `descendant_score`, `PolicyConfig`, legacy trim limit seam.
- `packages/open-bitcoin-mempool/src/fee.rs` — Fee-role wrappers and effective admission derivation.
- `packages/open-bitcoin-mempool/src/resource.rs` — Accounted ledger and recomputation oracle.
- `packages/open-bitcoin-mempool/src/context.rs` — `PressureDecisionContext`, `BlockLifecycleContext`, acceptance-time metadata.
- `packages/open-bitcoin-mempool/src/pool/lifecycle.rs` — Lifecycle deltas, Pressure/Expiry causes, transitional enforcement/parity labels.
- `packages/open-bitcoin-node/src/network/runtime_authority.rs` — Mutation authority.
- `packages/open-bitcoin-node/src/network/admission_bridge.rs` — Admission trim / Pressure projection seam.
- `packages/open-bitcoin-node/src/network/mempool_lifecycle.rs` — Block/reorg lifecycle seam for decay trigger.
- `docs/parity/catalog/mempool-policy.md` — Documented descendant-score trim and deferred rolling-fee gaps.
- `docs/parity/index.json` — Deferred rolling-min-fee ownership notes.
- `docs/parity/source-breadcrumbs.json` — `mempool-policy`, `mempool-resource-accounting`, `mempool-lifecycle` breadcrumb groups.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- `Mempool::trim_to_size` / `select_eviction_candidate` — Descendant-score package selection already exists; switch limiter and add bump.
- `MempoolEntry::descendant_score` — Pinned package scoring for eviction order.
- `collect_descendants` + `recompute_state` — Package removal and invariant repair.
- `MempoolResourceLedger` / `recompute_resource_ledger` — Accounted usage oracle for capacity enforcement.
- Fee-role types (`StaticRelayFeeRate`, `IncrementalRelayFeeRate`, `RollingMempoolFeeRate`, `EffectiveAdmissionFeeRate`) — Already distinct; bump/decay mutate rolling only.
- `MempoolLifecycleDelta` with `Pressure` / `Expiry` causes and Direct/Descendant roles — Facts vocabulary ready; Expiry not yet produced in production paths.
- `PressureDecisionContext` / `BlockLifecycleContext` — Defined explicit-input seams; not yet consumed by trim/decay.
- Node cause→serving maps in `admission_bridge` and `mempool_lifecycle` — Ready for Expiry/Pressure once pure core emits them.

### Established Patterns

- Functional core: no wall-clock or randomness inside `open-bitcoin-mempool`.
- Imperative shell samples time into typed contexts and mutates only through `ManagedNetworkHandle`.
- Attempt outcomes (`MempoolOutcome`) stay separate from committed lifecycle deltas.
- Shared evidence uses stable low-cardinality labels; identities stay on authenticated paths.
- Phase 130 intentionally left `RollingFeeParityStatus::Deferred` and `MempoolCapacityEnforcement::LegacyVsize` for this phase to replace.

### Integration Points

- Admission path trim inside `commit_transaction_with_context` — Primary pressure bump site.
- `apply_connected_block_mempool_lifecycle` — Natural block-gated decay trigger after connect cleanup.
- Future/maintenance call through managed authority for periodic expiry with shell-sampled `PolicyTime`.
- `ManagedPeerNetwork::mempool_info` / RPC — Must reflect accounted enforcement and live rolling/effective fees.
- Recovery paths must not invent rolling state; restart rolling baseline remains zero.

</code_context>

<specifics>
## Specific Ideas

- Prefer Knots function anchors (`TrimToSize`, `trackPackageRemoved`, `GetMinFee`, `Expire`) as the behavioral source of truth over inventing Open Bitcoin-specific pressure policy.
- Keep manual `set_rolling_mempool_fee_rate` only as a test/inject seam until bump/decay own the state machine.
- Natural core commit order: accounted trim + bump → block-gated decay → expiry cleanup → evidence label flip + remove legacy vsize seam → sustained-pressure oracle/perf tests.

</specifics>

<deferred>
## Deferred Ideas

- Typed package vocabulary, staged admission, TRUC/ephemeral-dust package exceptions — Phase 132.
- Package-aware download / same-peer 1P1C orphan bridge — Phase 133.
- Authoritative cross-cache lifecycle projection for every dependent cache — Phase 134.
- Snapshot schema, checkpointing, and recovery of durable mempool records — Phase 135 (rolling fee remains non-durable).
- Receive-independent maintenance loops and transport receipts beyond the minimum expiry/decay wiring — Phase 136.
- Broader RPC/operator evidence expansion beyond correcting enforcement and rolling-fee labels needed for PRESS — Phase 137.

None — discussion stayed within phase scope

</deferred>

---

*Phase: 131-rolling-fee-expiry-and-descendant-eviction-core*
*Context gathered: 2026-07-24*
