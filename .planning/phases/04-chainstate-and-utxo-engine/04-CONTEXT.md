---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 04-2026-04-12T23-38-43
generated_at: 2026-04-12T23:38:43.326Z
---

# Phase 4: Chainstate and UTXO Engine - Context

**Gathered:** 2026-04-12  
**Status:** Ready for planning and execution  
**Mode:** Yolo

<domain>
## Phase Boundary

Phase 4 owns the first baseline-compatible chainstate slice: typed UTXO state,
active-tip bookkeeping, block connect and disconnect mechanics, and reorg
selection that can drive later mempool, networking, and wallet work. The pure
core must own state transitions and undo data. Persistence, snapshots, and
runtime orchestration stay outside that pure core behind explicit adapters.

</domain>

<decisions>
## Implementation Decisions

### Chainstate core shape
- **D-01:** Add a new pure-core crate dedicated to chainstate instead of
  burying mutable state inside `open-bitcoin-node` or overloading the existing
  consensus crate.
- **D-02:** Model UTXO entries explicitly with coinbase and creation-height
  metadata so consensus validation contexts can be derived from chainstate
  without hidden global lookups.
- **D-03:** Keep block-index metadata, active-chain bookkeeping, undo payloads,
  and UTXO mutations as first-class Rust types rather than implicit side
  effects on a store.

### Connect and disconnect behavior
- **D-04:** Connect logic must derive spend contexts from the current UTXO set,
  validate the block through the existing consensus APIs, then apply spends and
  newly created outputs in the same order Knots uses: spend non-coinbase inputs
  first, then add transaction outputs at the connected height.
- **D-05:** Disconnect logic must remove created outputs, replay undo entries
  in reverse transaction and reverse input order, and restore the previous tip
  hash and height explicitly.
- **D-06:** Undo data must be preserved as a stable artifact of connect so later
  disconnect and reorg flows do not recompute historical spend information.

### Reorg and chain selection
- **D-07:** The active chain must be chosen by cumulative work first, with
  deterministic tie-breaking on height and block hash so targeted fixtures can
  assert stable outcomes.
- **D-08:** Reorg application should be represented as an explicit disconnect
  path plus connect path rather than hidden mutation, so later networking code
  can inspect what changed.

### Adapter and parity boundary
- **D-09:** Storage stays adapter-owned in `open-bitcoin-node`; Phase 4 only
  needs a repo-owned in-memory adapter and snapshot contract that later disk
  adapters can implement.
- **D-10:** Targeted parity fixtures should mirror Knots concepts from
  `coins.h`, `coins.cpp`, and `validation.cpp` for add, spend, undo, connect,
  disconnect, and best-chain selection without trying to port the entire node.

</decisions>

<specifics>
## Specific Ideas

- Reuse the existing easy-difficulty block patterns from the consensus test
  suite so chainstate fixtures can exercise real block validation without
  inventing a second fake block format.
- Keep the public chainstate API small: connect one block, disconnect one
  block, and activate the best chain from known candidates.
- Make snapshots and adapter payloads serializable Rust structs, not trait
  objects, so the persistence boundary is easy to audit.

</specifics>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Project and phase scope
- `.planning/PROJECT.md` — functional core / imperative shell boundary and
  parity goal for the milestone
- `.planning/REQUIREMENTS.md` — `CHAIN-01` acceptance scope
- `.planning/ROADMAP.md` § Phase 4 — chainstate goal, success criteria, and
  roadmap plan inventory
- `.planning/phases/03-consensus-validation-engine/03-CONTEXT.md` — prior
  decision to keep contextual validation explicit instead of coupling consensus
  directly to chainstate

### Existing Rust implementation
- `packages/open-bitcoin-consensus/src/context.rs` — spend metadata and
  validation-context types that chainstate must populate
- `packages/open-bitcoin-consensus/src/block.rs` — block validation entry points
  and easy-difficulty test patterns
- `packages/open-bitcoin-primitives/src/block.rs` — current block and header
  primitives
- `packages/open-bitcoin-primitives/src/transaction.rs` — outpoint, input, and
  output domain types the UTXO engine will index

### Knots chainstate baseline
- `packages/bitcoin-knots/src/coins.h` — coin metadata, spent-state semantics,
  and coins-view responsibilities
- `packages/bitcoin-knots/src/coins.cpp` — add, spend, and cache-write behavior
- `packages/bitcoin-knots/src/validation.cpp` — `UpdateCoins`,
  `DisconnectBlock`, `ConnectBlock`, and best-chain activation behavior

### Repo-native verification and parity tracking
- `scripts/verify.sh` — verification contract the phase must keep green
- `docs/parity/index.json` — surface-level parity status that should move
  chainstate from `planned` when Phase 4 is complete

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `open-bitcoin-consensus::SpentOutput` and `TransactionValidationContext`
  already capture the per-input data chainstate needs to supply.
- `open-bitcoin-consensus::validate_block_with_context` can validate connected
  blocks once chainstate derives per-transaction contexts.
- The existing consensus tests already build valid low-difficulty blocks, so
  Phase 4 can reuse that fixture style for connect and reorg coverage.

### Established Patterns
- Pure-core crates live under `packages/` and are re-exported through
  `open-bitcoin-core`; runtime and adapter glue stays in `open-bitcoin-node`.
- The repo favors explicit typed state over booleans or hidden mutable caches.
- Verification expects repo-owned tests and Bazel/Cargo wiring to land together
  with the implementation.

### Integration Points
- `open-bitcoin-core` should re-export the new chainstate crate for downstream
  callers.
- `open-bitcoin-node` should own the first adapter surface around chainstate
  snapshots and persistence.
- Phase 5 mempool work will consume the same active-chain and UTXO views, so
  the API must stay reusable rather than node-private.

</code_context>

<deferred>
## Deferred Ideas

- on-disk LevelDB parity and assumeutxo flows
- mempool-aware spend views and package policy integration
- initial block download, checkpoints, and multi-chainstate snapshot handling
- wallet-facing balance or coin-selection projections

</deferred>

---

*Phase: 04-chainstate-and-utxo-engine*  
*Context gathered: 2026-04-12*
