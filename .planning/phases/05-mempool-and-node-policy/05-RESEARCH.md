---
phase: 05-mempool-and-node-policy
generated_by: gsd-phase-researcher
lifecycle_mode: yolo
phase_lifecycle_id: 05-2026-04-13T23-15-14
generated_at: 2026-04-13T23:15:14.250Z
---

# Phase 5: Mempool and Node Policy - Research

**Researched:** 2026-04-13
**Domain:** pure-core mempool state, transaction policy admission, replacement, ancestor or descendant accounting, and size-limit eviction
**Confidence:** HIGH

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
All items in this subsection are copied verbatim from `05-CONTEXT.md`. [VERIFIED: .planning/phases/05-mempool-and-node-policy/05-CONTEXT.md]
- **D-01:** Add a dedicated pure-core `open-bitcoin-mempool` crate instead of
  burying mempool state inside `open-bitcoin-node` or expanding
  `open-bitcoin-chainstate` beyond its phase boundary.
- **D-02:** Keep node-facing orchestration thin in `open-bitcoin-node`; the
  shell may submit transactions against chainstate snapshots, but admission,
  replacement, ancestor or descendant relationships, and eviction decisions
  must remain pure data transformations.
- **D-03:** Admission must derive prevout context from the active chainstate
  snapshot first and then from already-accepted mempool parents so policy
  checks can reason over confirmed and in-mempool spends without hidden global
  lookups.
- **D-04:** The initial parity slice should cover standardness, min-relay
  feerate, transaction weight and sigop limits, explicit conflict detection,
  opt-in or full-RBF gating, ancestor or descendant limits, and mempool
  size-limit eviction.
- **D-05:** Replacement must remove the direct conflicts plus their descendants
  only when the replacement pays a higher absolute fee, beats the conflicting
  feerates, satisfies the incremental relay bump, and does not add new
  unconfirmed inputs outside the replaced set.
- **D-06:** Prefer deterministic recomputation of parent, child, ancestor, and
  descendant accounting after each mutation instead of trying to port Knots'
  cache-heavy incremental bookkeeping in the first slice.
- **D-07:** Phase 5 should use targeted repo-owned policy fixtures over the
  public mempool API plus a thin node wrapper rather than a full network or RPC
  harness.
- **D-08:** Any still-missing policy surfaces, such as package relay, rolling
  minimum-fee updates, or reorg-driven mempool repair, must be called out
  explicitly in parity docs instead of being implied as complete.

### the agent's Discretion
- Exact module split inside `open-bitcoin-mempool`
- Whether the thin node wrapper is named `ManagedMempool` or similar
- Exact fixture values for relay fee, size-limit eviction, and ancestor-chain tests

### Deferred Ideas (OUT OF SCOPE)
All items in this subsection are copied verbatim from `05-CONTEXT.md`. [VERIFIED: .planning/phases/05-mempool-and-node-policy/05-CONTEXT.md]
- package relay and multi-transaction package admission beyond single-tx policy
- rolling minimum-fee decay and long-lived dynamic relay-fee behavior
- reorg-driven mempool repair and disconnected-transaction staging
- RPC, CLI, and networking relay surfaces over the mempool

</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| `MEM-01` | The node enforces mempool admission, replacement, and eviction policy compatibly with the baseline. [VERIFIED: .planning/REQUIREMENTS.md] | The minimum Phase 5 slice must cover standardness, relay fee, conflict replacement, and size-limit eviction against deterministic fixtures. [VERIFIED: .planning/ROADMAP.md, .planning/phases/05-mempool-and-node-policy/05-CONTEXT.md, packages/bitcoin-knots/src/txmempool.h, packages/bitcoin-knots/src/txmempool.cpp, packages/bitcoin-knots/src/policy/policy.h, packages/bitcoin-knots/src/policy/rbf.h] |
| `MEM-02` | Policy-related deviations are explicit, tested, and recorded instead of drifting silently. [VERIFIED: .planning/REQUIREMENTS.md] | The parity catalog and index update are required outputs, not optional docs cleanup. [VERIFIED: docs/parity/index.json, .planning/phases/05-mempool-and-node-policy/05-CONTEXT.md] |

</phase_requirements>

## Summary

- Phase 5 should add a new pure-core mempool crate that depends on the existing
  chainstate, consensus, codec, and primitives crates. The pure core should own
  entry state, conflict detection, ancestor or descendant accounting, relay-fee
  checks, and eviction decisions. `open-bitcoin-node` should only wrap those
  decisions for runtime callers. [VERIFIED: .planning/phases/05-mempool-and-node-policy/05-CONTEXT.md, packages/open-bitcoin-core/src/lib.rs, packages/open-bitcoin-node/src/lib.rs]
- Knots' mempool code mixes correctness policy with cache-heavy bookkeeping,
  package logic, and long-running node concerns. The first Open Bitcoin slice
  should mirror the externally visible rules that Phase 5 promises, not the
  mutable cache implementation strategy. Deterministic recomputation of
  relationships after each mutation is acceptable for the initial slice because
  the roadmap asks for reproducible behavior, not throughput tuning. [VERIFIED: packages/bitcoin-knots/src/txmempool.h, packages/bitcoin-knots/src/txmempool.cpp, .planning/ROADMAP.md]
- The cleanest admission path is: derive prevout metadata from chainstate or
  in-mempool parents, run existing consensus validation with standard policy
  flags, apply targeted standardness checks from `policy.h`, then mutate the
  mempool state only after the transaction survives replacement and limit
  checks. [VERIFIED: packages/open-bitcoin-chainstate/src/types.rs, packages/open-bitcoin-consensus/src/context.rs, packages/open-bitcoin-consensus/src/transaction.rs, packages/open-bitcoin-consensus/src/classify.rs, packages/bitcoin-knots/src/policy/policy.h, packages/bitcoin-knots/src/policy/rbf.h]

**Primary recommendation:** implement Phase 5 as a pure-core `open-bitcoin-mempool`
engine with explicit policy config, deterministic relationship recomputation,
targeted opt-in/full-RBF support, and parity fixtures over the public API. [VERIFIED:
.planning/PROJECT.md, .planning/ROADMAP.md, .planning/phases/05-mempool-and-node-policy/05-CONTEXT.md]

## Standard Stack

### Core
| Component | Purpose | Why Standard For This Phase | Source |
|-----------|---------|-----------------------------|--------|
| New pure-core mempool crate under `packages/` | Own mempool entry state, conflict rules, ancestor or descendant accounting, and eviction | Phase 5 explicitly chose a dedicated pure-core mempool boundary instead of shell-owned policy. | [VERIFIED: .planning/phases/05-mempool-and-node-policy/05-CONTEXT.md] |
| `open-bitcoin-chainstate` | Active-chain and UTXO snapshot source | Admission and replacement decisions must derive prevouts from the chainstate snapshot first. | [VERIFIED: packages/open-bitcoin-chainstate/src/types.rs, packages/open-bitcoin-chainstate/src/engine.rs] |
| `open-bitcoin-consensus` | Transaction validation, fee calculation, script classification, and sigop counting | Phase 5 should extend the existing pure-core consensus rules instead of duplicating them in a second policy validator. | [VERIFIED: packages/open-bitcoin-consensus/src/context.rs, packages/open-bitcoin-consensus/src/transaction.rs, packages/open-bitcoin-consensus/src/classify.rs, packages/open-bitcoin-consensus/src/script.rs] |
| `open-bitcoin-codec` | Deterministic transaction serialization for weight and vsize calculation | Virtual-size policy decisions depend on exact transaction encoding. | [VERIFIED: packages/open-bitcoin-codec/src/transaction.rs] |

### Supporting
| Component | Purpose | When To Use | Source |
|-----------|---------|-------------|--------|
| `open-bitcoin-core` | Umbrella pure-core export | Re-export the mempool crate for downstream packages. | [VERIFIED: packages/open-bitcoin-core/src/lib.rs] |
| `open-bitcoin-node` | Thin runtime shell wrapper | Accept transactions against chainstate snapshots without leaking policy state into the shell. | [VERIFIED: packages/open-bitcoin-node/src/lib.rs, .planning/phases/05-mempool-and-node-policy/05-CONTEXT.md] |
| `docs/parity/index.json` and `docs/parity/catalog/*.md` | Explicit parity and deviation tracking | Required for `MEM-02` and for honest subsystem status. | [VERIFIED: docs/parity/index.json] |
| `bash scripts/verify.sh` | Repo-native verification contract | Phase completion requires format, lint, build, tests, Bazel, purity, and coverage checks. | [VERIFIED: AGENTS.md, scripts/verify.sh] |

### Alternatives Considered
| Instead Of | Could Use | Tradeoff | Source |
|------------|-----------|----------|--------|
| Deterministic recomputation | Cache-heavy incremental ancestor or descendant updates | Faster at scale, but much harder to reason about and unnecessary for the first parity slice. | [VERIFIED: packages/bitcoin-knots/src/txmempool.cpp, .planning/phases/05-mempool-and-node-policy/05-CONTEXT.md] |
| Pure-core mempool engine | Shell-owned mempool wrapper only | Quicker to wire, but it violates the repo's architecture boundary and makes policy harder to test. | [VERIFIED: .planning/PROJECT.md, .planning/phases/05-mempool-and-node-policy/05-CONTEXT.md] |
| Targeted single-transaction policy | Full package relay and rolling fee behavior now | Higher fidelity, but outside the current roadmap boundary. | [VERIFIED: .planning/ROADMAP.md, .planning/phases/05-mempool-and-node-policy/05-CONTEXT.md] |

## Architecture Patterns

### Recommended Crate Boundary

- `packages/open-bitcoin-mempool`
  Own pure entry state, policy configuration, standardness checks, conflict or
  replacement rules, relationship recomputation, and eviction decisions.
  [VERIFIED: .planning/phases/05-mempool-and-node-policy/05-CONTEXT.md]
- `packages/open-bitcoin-core`
  Re-export the mempool crate alongside chainstate, consensus, codec, and
  primitives. [VERIFIED: packages/open-bitcoin-core/src/lib.rs]
- `packages/open-bitcoin-node`
  Own a thin managed wrapper that feeds chainstate snapshots and config into the
  pure-core mempool. [VERIFIED: packages/open-bitcoin-node/src/lib.rs]

### Minimum Policy Types

- `PolicyConfig`
  Carries relay fee, incremental relay fee, ancestor or descendant limits,
  size-limit cap, and opt-in/full-RBF configuration. These constants are the
  stable policy knobs exposed by `policy.h`. [VERIFIED: packages/bitcoin-knots/src/policy/policy.h]
- `MempoolEntry`
  Stores the transaction, fee, txid/wtxid identity, virtual size, sigop cost,
  direct parents or children, and aggregate ancestor or descendant metrics for
  admission or eviction decisions. [VERIFIED: packages/bitcoin-knots/src/txmempool.h]
- `AdmissionResult`
  Makes accepted, replaced, and evicted transaction IDs explicit so later node
  layers can react without inspecting hidden side effects. [ASSUMED]
- `MempoolError`
  Captures precise policy rejects such as missing inputs, non-standard scripts,
  feerate below relay minimum, replacement too cheap, or ancestor-limit
  overflow. [VERIFIED: packages/bitcoin-knots/src/policy/policy.h, packages/bitcoin-knots/src/policy/rbf.h]

### Recommended Module Split

```text
packages/open-bitcoin-mempool/src/
├── lib.rs
├── error.rs        # Policy-specific rejection surface
├── policy.rs       # Standardness, feerate, and size helpers
├── types.rs        # Entry state, aggregates, and config
└── pool.rs         # Admission, replacement, accounting, and eviction
```

Recommended module names only; the pure-core boundary and public API shape
matter more than exact filenames. [ASSUMED]

## Baseline Behaviors To Mirror

- Transactions entering the mempool must satisfy consensus validation plus
  additional standardness policy such as standard output forms, push-only
  scriptSig behavior, weight limits, and standard sigop limits. [VERIFIED:
  packages/bitcoin-knots/src/policy/policy.h, packages/open-bitcoin-consensus/src/classify.rs, packages/open-bitcoin-consensus/src/script.rs]
- Ancestor or descendant checks matter both at entry time and during updates of
  related transactions. The first slice only needs deterministic limits and
  reproducible aggregated accounting, not Knots' exact caching layout.
  [VERIFIED: packages/bitcoin-knots/src/txmempool.cpp]
- Replacement policy is conflict-scoped: it compares against conflicting
  transactions, enforces a higher absolute fee, a higher feerate, an
  incremental relay bump, and restricts new unconfirmed inputs outside the
  replaced set. [VERIFIED: packages/bitcoin-knots/src/policy/rbf.h,
  packages/bitcoin-knots/src/test/rbf_tests.cpp]
- Size-limit trimming removes the lowest-feerate package according to
  descendant-aware ordering, not just the last inserted transaction. [VERIFIED:
  packages/bitcoin-knots/src/txmempool.h, packages/bitcoin-knots/src/txmempool.cpp]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why | Source |
|---------|-------------|-------------|-----|--------|
| Consensus fee or maturity rules | A second tx validator inside the mempool crate | Existing consensus `validate_transaction_with_context` and `check_tx_inputs` | Policy should extend consensus, not fork it. | [VERIFIED: packages/open-bitcoin-consensus/src/context.rs, packages/open-bitcoin-consensus/src/transaction.rs] |
| Size and vsize calculation | Ad-hoc byte estimates | Exact codec serialization with and without witness | Relay and eviction policy depend on exact encoded size. | [VERIFIED: packages/open-bitcoin-codec/src/transaction.rs, packages/open-bitcoin-consensus/src/transaction.rs] |
| Shell-owned prevout state | Hidden globals in `open-bitcoin-node` | Explicit chainstate snapshots and in-mempool parent lookups | The repo requires pure data-in or data-out policy decisions. | [VERIFIED: .planning/PROJECT.md, packages/open-bitcoin-chainstate/src/types.rs] |
| Implicit parity scope | "Done" docs without a ledger entry | `docs/parity/catalog/mempool-policy.md` plus `docs/parity/index.json` | `MEM-02` requires explicit deviations and status. | [VERIFIED: .planning/REQUIREMENTS.md, docs/parity/index.json] |

## Concrete Parity Fixtures

| Fixture | What It Should Prove | Why It Matters | Source |
|---------|----------------------|----------------|--------|
| `accepts_standard_confirmed_spend` | A standard transaction spending a confirmed output is admitted with stable fee and relationship metadata. | This is the basic `MEM-01` admission truth. | [VERIFIED: .planning/ROADMAP.md, packages/open-bitcoin-consensus/src/transaction.rs] |
| `rejects_non_standard_output_script` | Non-standard outputs fail policy admission even when consensus-valid. | Standardness is the first mempool-only policy layer. | [VERIFIED: packages/bitcoin-knots/src/policy/policy.h, packages/open-bitcoin-consensus/src/classify.rs] |
| `replacement_requires_fee_bump` | A cheaper or equal-feerate conflict fails, but a higher-fee replacement succeeds and removes the conflict set. | Replacement behavior is a named success criterion for Phase 5. | [VERIFIED: packages/bitcoin-knots/src/policy/rbf.h, packages/bitcoin-knots/src/test/rbf_tests.cpp] |
| `ancestor_limits_are_enforced` | A child that would exceed the configured ancestor or descendant count or size is rejected deterministically. | The roadmap explicitly calls out ancestor or descendant outcomes. | [VERIFIED: .planning/ROADMAP.md, packages/bitcoin-knots/src/txmempool.cpp] |
| `size_limit_evicts_lowest_descendant_score_package` | When size exceeds the configured cap, the mempool trims the weakest package instead of silently overfilling. | Phase 5 promises eviction parity, not just admission parity. | [VERIFIED: packages/bitcoin-knots/src/txmempool.h, packages/bitcoin-knots/src/txmempool.cpp] |

## Common Pitfalls

- Re-validating fees or maturity rules separately from consensus will create a
  policy fork that drifts from the already-tested validation core. [VERIFIED:
  packages/open-bitcoin-consensus/src/context.rs, packages/open-bitcoin-consensus/src/transaction.rs]
- Treating every mempool conflict as direct replacement without descendant
  closure will produce incorrect removals and undercount replacement cost.
  [VERIFIED: packages/bitcoin-knots/src/policy/rbf.h, packages/bitcoin-knots/src/test/rbf_tests.cpp]
- Porting the full Knots caching model before the public policy semantics are
  proven will increase complexity without moving the external parity surface.
  [VERIFIED: packages/bitcoin-knots/src/txmempool.cpp, .planning/phases/05-mempool-and-node-policy/05-CONTEXT.md]
- Marking `mempool-policy` as done without explicitly documenting deferred
  surfaces will violate `MEM-02` even if the code is green. [VERIFIED:
  .planning/REQUIREMENTS.md, docs/parity/index.json]

## Assumptions Log

| # | Claim | Section | Risk If Wrong |
|---|-------|---------|---------------|
| A1 | The new pure-core crate should be named `open-bitcoin-mempool`. | Standard Stack / Architecture Patterns | Low; the boundary matters more than the exact package name. |
| A2 | Deterministic relationship recomputation is acceptable for the initial parity slice. | Summary / Alternatives Considered | Medium; performance work may require later incremental bookkeeping. |
| A3 | A thin node wrapper without persistence is enough for the Phase 5 node-side surface. | Architecture Patterns | Low; persistence is not part of the phase goal. |

## Sources

### Primary
- `.planning/PROJECT.md`
- `.planning/REQUIREMENTS.md`
- `.planning/ROADMAP.md`
- `.planning/phases/04-chainstate-and-utxo-engine/04-CONTEXT.md`
- `.planning/phases/05-mempool-and-node-policy/05-CONTEXT.md`
- `packages/open-bitcoin-chainstate/src/engine.rs`
- `packages/open-bitcoin-chainstate/src/types.rs`
- `packages/open-bitcoin-consensus/src/context.rs`
- `packages/open-bitcoin-consensus/src/transaction.rs`
- `packages/open-bitcoin-consensus/src/classify.rs`
- `packages/open-bitcoin-consensus/src/script.rs`
- `packages/open-bitcoin-codec/src/transaction.rs`
- `packages/bitcoin-knots/src/txmempool.h`
- `packages/bitcoin-knots/src/txmempool.cpp`
- `packages/bitcoin-knots/src/policy/policy.h`
- `packages/bitcoin-knots/src/policy/rbf.h`
- `packages/bitcoin-knots/src/test/rbf_tests.cpp`
- `packages/bitcoin-knots/src/test/txpackage_tests.cpp`
- `docs/parity/index.json`
- `scripts/verify.sh`

## Metadata

- Standard stack confidence: HIGH because the phase boundary and dependency
  direction are explicit in the context and existing workspace structure.
- Policy-surface confidence: HIGH because the cited Knots headers and tests
  expose the exact admission, replacement, and eviction rules Phase 5 must
  mirror in targeted fixtures.
- Performance caveat: ACCEPTABLE for this phase because correctness and
  auditability matter more than production-scale caching in the first slice.
