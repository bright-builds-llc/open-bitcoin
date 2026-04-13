---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 05-2026-04-13T23-15-14
generated_at: 2026-04-13T23:15:14.250Z
---

# Phase 5: Mempool and Node Policy - Context

**Gathered:** 2026-04-13  
**Status:** Ready for planning and execution  
**Mode:** Yolo

<domain>
## Phase Boundary

Phase 5 owns the first baseline-compatible mempool slice: transaction admission,
replacement, ancestor or descendant accounting, and size-based eviction behavior
over the existing chainstate core. The pure core must own mempool state and
policy decisions. Runtime orchestration stays in `open-bitcoin-node`. Networking,
package relay, RPC, wallet policy, and reorg-driven mempool repair stay out of
scope.

</domain>

<decisions>
## Implementation Decisions

### Crate and boundary shape
- **D-01:** Add a dedicated pure-core `open-bitcoin-mempool` crate instead of
  burying mempool state inside `open-bitcoin-node` or expanding
  `open-bitcoin-chainstate` beyond its phase boundary.
- **D-02:** Keep node-facing orchestration thin in `open-bitcoin-node`; the
  shell may submit transactions against chainstate snapshots, but admission,
  replacement, ancestor or descendant relationships, and eviction decisions
  must remain pure data transformations.

### Admission and validation scope
- **D-03:** Admission must derive prevout context from the active chainstate
  snapshot first and then from already-accepted mempool parents so policy
  checks can reason over confirmed and in-mempool spends without hidden global
  lookups.
- **D-04:** The initial parity slice should cover standardness, min-relay
  feerate, transaction weight and sigop limits, explicit conflict detection,
  opt-in or full-RBF gating, ancestor or descendant limits, and mempool
  size-limit eviction.

### Replacement and accounting
- **D-05:** Replacement must remove the direct conflicts plus their descendants
  only when the replacement pays a higher absolute fee, beats the conflicting
  feerates, satisfies the incremental relay bump, and does not add new
  unconfirmed inputs outside the replaced set.
- **D-06:** Prefer deterministic recomputation of parent, child, ancestor, and
  descendant accounting after each mutation instead of trying to port Knots'
  cache-heavy incremental bookkeeping in the first slice.

### Parity and audit boundary
- **D-07:** Phase 5 should use targeted repo-owned policy fixtures over the
  public mempool API plus a thin node wrapper rather than a full network or RPC
  harness.
- **D-08:** Any still-missing policy surfaces, such as package relay, rolling
  minimum-fee updates, or reorg-driven mempool repair, must be called out
  explicitly in parity docs instead of being implied as complete.

</decisions>

<specifics>
## Specific Ideas

- Reuse the easy-difficulty chainstate fixtures from Phase 4 so mempool tests
  can build real chain views without inventing a second fake UTXO model.
- Prefer standard P2SH `OP_TRUE` fixtures for policy tests so transactions stay
  consensus-valid while keeping scripts simple and deterministic.
- Keep the public mempool API narrow: submit one transaction, inspect entries,
  inspect accounting, and report replacements or evictions explicitly.

</specifics>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Project and phase scope
- `.planning/PROJECT.md` — functional-core boundary, parity goal, and headless
  scope for the milestone
- `.planning/REQUIREMENTS.md` — `MEM-01` and `MEM-02` acceptance scope
- `.planning/ROADMAP.md` § Phase 5 — mempool policy goal, success criteria, and
  roadmap plan inventory
- `.planning/phases/04-chainstate-and-utxo-engine/04-CONTEXT.md` — prior
  decision that chainstate exposes reusable active-chain and UTXO views for
  mempool consumers

### Existing Rust implementation
- `packages/open-bitcoin-chainstate/src/engine.rs` — active-chain, UTXO, and
  snapshot surfaces the mempool must read
- `packages/open-bitcoin-chainstate/src/types.rs` — coin metadata and snapshot
  shapes for policy context
- `packages/open-bitcoin-consensus/src/context.rs` — transaction input context,
  fee calculation, and sequence-lock helpers
- `packages/open-bitcoin-consensus/src/transaction.rs` — transaction validation
  entry points
- `packages/open-bitcoin-consensus/src/classify.rs` — script classification and
  push-only helpers for policy standardness
- `packages/open-bitcoin-consensus/src/script.rs` — sigop counting helpers used
  by standard transaction policy

### Knots mempool and policy baseline
- `packages/bitcoin-knots/src/txmempool.h` — mempool entry bookkeeping,
  ancestor or descendant metrics, and eviction ordering concepts
- `packages/bitcoin-knots/src/txmempool.cpp` — ancestor or descendant updates,
  package-limit checks, and size-limit trimming behavior
- `packages/bitcoin-knots/src/policy/policy.h` — standardness, relay-fee, and
  ancestor or descendant policy constants
- `packages/bitcoin-knots/src/policy/rbf.h` — replacement policy helpers and
  conflict topology checks
- `packages/bitcoin-knots/src/test/rbf_tests.cpp` — targeted replacement
  expectations
- `packages/bitcoin-knots/src/test/txpackage_tests.cpp` — ancestor or
  descendant and package-policy fixture patterns

### Repo-native verification and parity tracking
- `scripts/verify.sh` — verification contract the phase must keep green
- `docs/parity/index.json` — surface-level parity status for `mempool-policy`

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `open-bitcoin-chainstate::ChainstateSnapshot` already exposes the active UTXO
  set and tip metadata needed for mempool admission.
- `open-bitcoin-consensus::validate_transaction_with_context` and
  `check_tx_inputs` already compute consensus-valid fees and contextual checks
  once prevout metadata is assembled.
- Phase 4 parity tests already show how to build deterministic low-difficulty
  blocks and chainstate snapshots inside repo-owned tests.

### Established Patterns
- New pure-core subsystems land as dedicated crates under `packages/` and are
  re-exported through `open-bitcoin-core`.
- Shell/runtime crates own orchestration and adapters, not pure state
  transitions.
- Parity docs and the phase ledger move from `planned` to `done` only after the
  repo-native verifier is green.

### Integration Points
- `open-bitcoin-core` should re-export `open-bitcoin-mempool` for downstream
  users.
- `open-bitcoin-node` should expose a thin managed wrapper that accepts
  transactions against a chainstate snapshot and delegates the policy decision
  to the pure core.
- Phase 6 networking work will need the mempool's txid/wtxid visibility,
  conflict semantics, and ancestor-aware relay view to stay inspectable.

</code_context>

<deferred>
## Deferred Ideas

- package relay and multi-transaction package admission beyond single-tx policy
- rolling minimum-fee decay and long-lived dynamic relay-fee behavior
- reorg-driven mempool repair and disconnected-transaction staging
- RPC, CLI, and networking relay surfaces over the mempool

</deferred>

---

*Phase: 05-mempool-and-node-policy*  
*Context gathered: 2026-04-13*
