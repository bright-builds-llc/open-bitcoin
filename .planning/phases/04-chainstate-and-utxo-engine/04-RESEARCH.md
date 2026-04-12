---
phase: 04-chainstate-and-utxo-engine
generated_by: gsd-phase-researcher
lifecycle_mode: yolo
phase_lifecycle_id: 04-2026-04-12T23-38-43
generated_at: 2026-04-12T23:59:00Z
---

# Phase 4: Chainstate and UTXO Engine - Research

**Researched:** 2026-04-12
**Domain:** pure-core chainstate, UTXO semantics, undo data, reorg selection, and adapter-owned persistence
**Confidence:** HIGH

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
All items in this subsection are copied verbatim from `04-CONTEXT.md`. [VERIFIED: .planning/phases/04-chainstate-and-utxo-engine/04-CONTEXT.md]
- **D-01:** Add a new pure-core crate dedicated to chainstate instead of
  burying mutable state inside `open-bitcoin-node` or overloading the existing
  consensus crate.
- **D-02:** Model UTXO entries explicitly with coinbase and creation-height
  metadata so consensus validation contexts can be derived from chainstate
  without hidden global lookups.
- **D-03:** Keep block-index metadata, active-chain bookkeeping, undo payloads,
  and UTXO mutations as first-class Rust types rather than implicit side
  effects on a store.
- **D-04:** Connect logic must derive spend contexts from the current UTXO set,
  validate the block through the existing consensus APIs, then apply spends and
  newly created outputs in the same order Knots uses: spend non-coinbase inputs
  first, then add transaction outputs at the connected height.
- **D-05:** Disconnect logic must remove created outputs, replay undo entries
  in reverse transaction and reverse input order, and restore the previous tip
  hash and height explicitly.
- **D-06:** Undo data must be preserved as a stable artifact of connect so later
  disconnect and reorg flows do not recompute historical spend information.
- **D-07:** The active chain must be chosen by cumulative work first, with
  deterministic tie-breaking on height and block hash so targeted fixtures can
  assert stable outcomes.
- **D-08:** Reorg application should be represented as an explicit disconnect
  path plus connect path rather than hidden mutation, so later networking code
  can inspect what changed.
- **D-09:** Storage stays adapter-owned in `open-bitcoin-node`; Phase 4 only
  needs a repo-owned in-memory adapter and snapshot contract that later disk
  adapters can implement.
- **D-10:** Targeted parity fixtures should mirror Knots concepts from
  `coins.h`, `coins.cpp`, and `validation.cpp` for add, spend, undo, connect,
  disconnect, and best-chain selection without trying to port the entire node.

### Claude's Discretion
- None explicitly delegated in `04-CONTEXT.md`. [VERIFIED: .planning/phases/04-chainstate-and-utxo-engine/04-CONTEXT.md]

### Deferred Ideas (OUT OF SCOPE)
All items in this subsection are copied verbatim from `04-CONTEXT.md`. [VERIFIED: .planning/phases/04-chainstate-and-utxo-engine/04-CONTEXT.md]
- on-disk LevelDB parity and assumeutxo flows
- mempool-aware spend views and package policy integration
- initial block download, checkpoints, and multi-chainstate snapshot handling
- wallet-facing balance or coin-selection projections

</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| `CHAIN-01` | The node maintains chainstate and UTXO state with baseline-compatible connect, disconnect, and reorg behavior. [VERIFIED: .planning/REQUIREMENTS.md] | The state model, undo model, connect or disconnect ordering, and reorg fixture set below are the minimum Phase 4 surface. [VERIFIED: .planning/ROADMAP.md, .planning/phases/04-chainstate-and-utxo-engine/04-CONTEXT.md, packages/bitcoin-knots/src/coins.h, packages/bitcoin-knots/src/coins.cpp, packages/bitcoin-knots/src/validation.cpp] |

</phase_requirements>

## Summary

- Phase 4 should add a new pure-core chainstate crate under `packages/`, re-export it from `open-bitcoin-core`, and keep persistence traits plus in-memory snapshot storage in `open-bitcoin-node`. [VERIFIED: .planning/phases/04-chainstate-and-utxo-engine/04-CONTEXT.md, packages/open-bitcoin-core/src/lib.rs, packages/open-bitcoin-node/src/lib.rs]
- The consensus-relevant UTXO payload is small: Knots `Coin` stores the output, coinbase bit, and creation height; the cache-layer `DIRTY` and `FRESH` flags live outside that payload and should stay out of the pure core. [VERIFIED: packages/bitcoin-knots/src/coins.h, packages/bitcoin-knots/src/coins.cpp]
- Connect semantics that matter for parity are: derive per-input validation context from the current UTXO set, reject missing inputs, spend inputs first, then add new outputs at the connected height, and update the best-block pointer only after the full connect succeeds. [VERIFIED: packages/open-bitcoin-consensus/src/context.rs, packages/open-bitcoin-consensus/src/block.rs, packages/bitcoin-knots/src/validation.cpp]
- Disconnect semantics that matter for parity are: remove created outputs, restore spent inputs from stored undo in reverse transaction order and reverse input order, then rewind the tip to the previous block. [VERIFIED: packages/bitcoin-knots/src/validation.cpp]
- Reorg logic should stay explicit and phase-scoped: find the best usable candidate by cumulative work, disconnect back to the fork point, connect the winning branch forward, and surface the disconnect or connect path as typed transition data for later node layers. [VERIFIED: .planning/phases/04-chainstate-and-utxo-engine/04-CONTEXT.md, packages/bitcoin-knots/src/validation.cpp]

**Primary recommendation:** implement Phase 4 as a pure-core chainstate transition engine with repo-owned `CoinEntry`, block-index, and undo types, plus deterministic reorg planning, while keeping all persistence and cache-write behavior adapter-owned. [VERIFIED: .planning/PROJECT.md, .planning/ROADMAP.md, .planning/phases/04-chainstate-and-utxo-engine/04-CONTEXT.md]

## Standard Stack

### Core
| Component | Purpose | Why Standard For This Phase | Source |
|-----------|---------|-----------------------------|--------|
| New pure-core chainstate crate under `packages/` | Own UTXO state, block-index metadata, connect/disconnect logic, and reorg planning | Phase 4 explicitly chose a dedicated pure-core crate instead of putting mutable state into `open-bitcoin-node` or `open-bitcoin-consensus`. | [VERIFIED: .planning/phases/04-chainstate-and-utxo-engine/04-CONTEXT.md] |
| `open-bitcoin-primitives` | Typed blocks, headers, transactions, outputs, and outpoints | The chainstate engine indexes `OutPoint`, `TransactionOutput`, `Block`, and `BlockHeader` values that already exist in the first-party primitives crate. | [VERIFIED: packages/open-bitcoin-primitives/src/block.rs, packages/open-bitcoin-primitives/src/transaction.rs] |
| `open-bitcoin-consensus` | Contextual block and transaction validation | Phase 4 should derive `TransactionValidationContext` and `BlockValidationContext` from chainstate instead of re-implementing consensus checks. | [VERIFIED: packages/open-bitcoin-consensus/src/context.rs, packages/open-bitcoin-consensus/src/block.rs] |

### Supporting
| Component | Purpose | When To Use | Source |
|-----------|---------|-------------|--------|
| `open-bitcoin-core` | Pure-core umbrella export | Re-export the new chainstate crate for downstream consumers, matching the current workspace pattern. | [VERIFIED: packages/open-bitcoin-core/src/lib.rs] |
| `open-bitcoin-node` | Adapter-owned snapshot persistence and runtime orchestration | Keep the first in-memory store, future disk stores, and runtime-owned flush policy here. | [VERIFIED: packages/open-bitcoin-node/src/lib.rs, .planning/phases/04-chainstate-and-utxo-engine/04-CONTEXT.md] |
| `bash scripts/verify.sh` | Repo-native verification | Run format, clippy, build, tests, pure-core dependency checks, and pure-core coverage before Phase 4 is considered done. | [VERIFIED: AGENTS.md, scripts/verify.sh] |
| `docs/parity/index.json` | Phase status and parity ledger | Update the `chainstate` surface from `planned` when Phase 4 lands. | [VERIFIED: docs/parity/index.json] |

### Alternatives Considered
| Instead Of | Could Use | Tradeoff | Source |
|------------|-----------|----------|--------|
| Dedicated pure-core chainstate crate | Put chainstate inside `open-bitcoin-node` | Faster short-term wiring, but it violates the phase boundary and weakens the pure-core contract. | [VERIFIED: .planning/phases/04-chainstate-and-utxo-engine/04-CONTEXT.md] |
| Minimal UTXO payload plus explicit undo | Port `CCoinsViewCache` flags and writeback behavior directly | Knots cache flags are persistence-cache mechanics, not the consensus-relevant UTXO model. | [VERIFIED: packages/bitcoin-knots/src/coins.h, packages/bitcoin-knots/src/coins.cpp] |
| Adapter-owned persistence | Disk or snapshot I/O inside the chainstate crate | This would cross the repo's functional-core boundary and fight the phase decision to keep storage outside the pure core. | [VERIFIED: .planning/PROJECT.md, .planning/phases/04-chainstate-and-utxo-engine/04-CONTEXT.md] |

## Architecture Patterns

### Recommended Crate Boundary

- `packages/open-bitcoin-chainstate`
  Own pure data types and state transitions: `CoinEntry`, block-index metadata, active-chain bookkeeping, `TxUndo`, `BlockUndo`, connect/disconnect functions, and reorg planning. [VERIFIED: .planning/phases/04-chainstate-and-utxo-engine/04-CONTEXT.md]
- `packages/open-bitcoin-core`
  Re-export the chainstate crate alongside primitives and consensus so later packages keep one pure-core import surface. [VERIFIED: packages/open-bitcoin-core/src/lib.rs]
- `packages/open-bitcoin-node`
  Own snapshot persistence contracts, the first in-memory adapter, and any future disk-backed stores or flush policies. [VERIFIED: packages/open-bitcoin-node/src/lib.rs, .planning/phases/04-chainstate-and-utxo-engine/04-CONTEXT.md]

### Minimum State Types

- `CoinEntry { output, created_height, is_coinbase }`
  This is the minimum UTXO payload Knots keeps in `Coin`, and it matches what consensus already needs for maturity and spend validation. [VERIFIED: packages/bitcoin-knots/src/coins.h, packages/open-bitcoin-consensus/src/context.rs]
- `ChainIndexEntry { block_hash, previous_block_hash, header, height, chain_work, median_time_past }`
  Phase 4 needs enough indexed block metadata to derive `BlockValidationContext`, pick the active tip by work, and build explicit reorg paths. [VERIFIED: .planning/phases/04-chainstate-and-utxo-engine/04-CONTEXT.md, packages/open-bitcoin-consensus/src/context.rs, packages/bitcoin-knots/src/validation.cpp]
- `ActiveChainState { tip, active_path, utxos }`
  The pure core needs a typed representation of the active tip plus the spendable UTXO set; storage-layer caches and flush flags do not belong here. [VERIFIED: .planning/phases/04-chainstate-and-utxo-engine/04-CONTEXT.md, packages/bitcoin-knots/src/coins.h]
- `ChainstateSnapshot`
  The persistence boundary should use serializable structs instead of trait objects so adapters can save and restore pure-core state without leaking I/O into the core. [VERIFIED: .planning/phases/04-chainstate-and-utxo-engine/04-CONTEXT.md]

### Minimum Undo Types

- `TxUndo { spent_coins }`
  Knots stores one undo record per non-coinbase transaction and fills it as each input is spent during connect. [VERIFIED: packages/bitcoin-knots/src/validation.cpp]
- `BlockUndo { tx_undos }`
  Phase 4 needs a stable block-level undo artifact so disconnect and reorg do not re-query or recompute historical spent outputs. [VERIFIED: .planning/phases/04-chainstate-and-utxo-engine/04-CONTEXT.md, packages/bitcoin-knots/src/validation.cpp]
- `ChainTransition { disconnect_path, connect_path, undo }`
  Reorg application should be explicit transition data rather than hidden mutation so later node layers can react without owning chainstate logic. [VERIFIED: .planning/phases/04-chainstate-and-utxo-engine/04-CONTEXT.md] [ASSUMED: exact type name] 

### Recommended Module Split

```text
packages/open-bitcoin-chainstate/src/
├── lib.rs
├── coin.rs          # CoinEntry and UTXO helpers
├── index.rs         # ChainIndexEntry and active-chain metadata
├── undo.rs          # TxUndo and BlockUndo
├── connect.rs       # connect_block and disconnect_block
├── reorg.rs         # best-chain choice and transition planning
└── snapshot.rs      # serializable snapshot payloads
```

Recommended module names only; the boundary is the important part, not these exact filenames. [ASSUMED]

## Baseline Behaviors To Mirror

- `AddCoin` skips unspendable outputs entirely, so Phase 4 should not keep provably unspendable outputs in the spendable UTXO map. [VERIFIED: packages/bitcoin-knots/src/coins.cpp]
- `UpdateCoins` spends every non-coinbase input before adding that transaction's outputs, which lets later transactions in the same block spend earlier transaction outputs in block order. [VERIFIED: packages/bitcoin-knots/src/validation.cpp]
- `check_tx_inputs` and sequence-lock helpers depend on `is_coinbase`, `created_height`, and `created_median_time_past`, so chainstate must carry that metadata instead of forcing hidden lookups later. [VERIFIED: packages/open-bitcoin-consensus/src/context.rs]
- `ConnectBlock` rejects BIP30-style overwrites when an output being created is still present in the current view. [VERIFIED: packages/bitcoin-knots/src/validation.cpp]
- `DisconnectBlock` removes created outputs before it restores prior coins from undo, and it treats mismatch or missing undo data as a disconnect failure or unclean result. [VERIFIED: packages/bitcoin-knots/src/validation.cpp]
- Knots chooses the best candidate by total chain work, then by `nSequenceId`, then by pointer address; Phase 4 should keep the work-first rule and use the already-decided deterministic tie-break for fixtures instead of pointer identity. [VERIFIED: packages/bitcoin-knots/src/node/blockstorage.cpp, .planning/phases/04-chainstate-and-utxo-engine/04-CONTEXT.md]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why | Source |
|---------|-------------|-------------|-----|--------|
| UTXO payload | A cache-entry clone with `DIRTY` or `FRESH` flags in the pure core | A plain `CoinEntry` plus adapter-owned snapshot or writeback logic | The flags are cache-flush mechanics, not the consensus-relevant coin model. | [VERIFIED: packages/bitcoin-knots/src/coins.h, packages/bitcoin-knots/src/coins.cpp] |
| Disconnect data | Recomputed historical spends during disconnect | `TxUndo` and `BlockUndo` captured during connect | Phase 4 explicitly chose stable undo artifacts, and Knots stores undo as it spends inputs. | [VERIFIED: .planning/phases/04-chainstate-and-utxo-engine/04-CONTEXT.md, packages/bitcoin-knots/src/validation.cpp] |
| Reorg application | Hidden tip mutation inside node orchestration | Explicit disconnect and connect paths emitted by the pure core | Later networking and mempool phases need inspectable transition data. | [VERIFIED: .planning/phases/04-chainstate-and-utxo-engine/04-CONTEXT.md, packages/bitcoin-knots/src/validation.cpp] |
| Persistence boundary | Filesystem or DB writes inside chainstate functions | Serializable snapshots consumed by `open-bitcoin-node` adapters | The repo requires a functional core and imperative shell split. | [VERIFIED: .planning/PROJECT.md, .planning/phases/04-chainstate-and-utxo-engine/04-CONTEXT.md] |

## Concrete Parity Fixtures

| Fixture | What It Should Prove | Why It Matters | Source |
|---------|----------------------|----------------|--------|
| `connect_block_spends_then_creates` | A block with dependent transactions connects only when per-tx input contexts come from the current view, and later transactions can spend outputs from earlier transactions in the same block. | This is the core `UpdateCoins` ordering rule. | [VERIFIED: packages/bitcoin-knots/src/validation.cpp] |
| `connect_block_preserves_coin_metadata` | Connected outputs store `created_height` and `is_coinbase`, and those fields feed coinbase-maturity and sequence-lock validation. | Phase 4 must populate consensus context correctly. | [VERIFIED: packages/open-bitcoin-consensus/src/context.rs, packages/bitcoin-knots/src/coins.h] |
| `connect_block_skips_unspendable_outputs` | `OP_RETURN` or otherwise unspendable outputs never appear in the spendable UTXO set. | Knots `AddCoin` exits early for unspendable scripts. | [VERIFIED: packages/bitcoin-knots/src/coins.cpp] |
| `connect_block_rejects_bip30_overwrite` | A block that tries to overwrite an existing unspent output is rejected before mutation is committed. | This is a chainstate-visible consensus rule in `ConnectBlock`. | [VERIFIED: packages/bitcoin-knots/src/validation.cpp] |
| `disconnect_block_restores_tip_and_utxos` | Disconnect removes created outputs, restores spent inputs from `BlockUndo`, and rewinds the best tip to the parent. | This is the minimum disconnect parity surface for `CHAIN-01`. | [VERIFIED: packages/bitcoin-knots/src/validation.cpp, .planning/ROADMAP.md] |
| `disconnect_block_replays_reverse_order` | A multi-input, multi-transaction block disconnects in reverse transaction order and reverse input order. | Undo ordering bugs here will silently corrupt reorg behavior. | [VERIFIED: packages/bitcoin-knots/src/validation.cpp] |
| `activate_best_chain_prefers_more_work` | Reorg planning disconnects to the fork point and connects the higher-work branch forward. | This is the phase's active-chain selection contract. | [VERIFIED: packages/bitcoin-knots/src/validation.cpp, .planning/phases/04-chainstate-and-utxo-engine/04-CONTEXT.md] |
| `equal_work_choice_is_deterministic` | Equal-work candidates resolve to the same winner every test run under the repo's deterministic tie-break. | The phase context chose stable fixture behavior over pointer-based tie breaks. | [VERIFIED: .planning/phases/04-chainstate-and-utxo-engine/04-CONTEXT.md, packages/bitcoin-knots/src/node/blockstorage.cpp] |

## Common Pitfalls

- Modeling Knots cache flags as consensus state will bloat the pure core and blur the adapter boundary. [VERIFIED: packages/bitcoin-knots/src/coins.h, .planning/PROJECT.md]
- Leaving `created_height` or `is_coinbase` out of the UTXO model will break maturity and sequence-lock context derivation later. [VERIFIED: packages/open-bitcoin-consensus/src/context.rs, .planning/phases/04-chainstate-and-utxo-engine/04-CONTEXT.md]
- Recomputing undo during disconnect instead of storing it during connect will make reorg correctness depend on mutable external state. [VERIFIED: .planning/phases/04-chainstate-and-utxo-engine/04-CONTEXT.md, packages/bitcoin-knots/src/validation.cpp]
- Pulling mempool repair, wallet balance views, or disk flush logic into Phase 4 will violate the roadmap boundary; Knots performs mempool repair around reorgs, but that belongs to later node layers here. [VERIFIED: .planning/ROADMAP.md, .planning/phases/04-chainstate-and-utxo-engine/04-CONTEXT.md, packages/bitcoin-knots/src/validation.cpp]

## Code Examples

### Consensus Context Shape That Chainstate Must Populate

```rust
TransactionValidationContext {
    inputs: vec![TransactionInputContext {
        spent_output: SpentOutput {
            value,
            script_pubkey,
            is_coinbase,
        },
        created_height,
        created_median_time_past,
    }],
    spend_height,
    block_time,
    median_time_past,
    verify_flags,
    consensus_params,
}
```

This is the existing pure-core validation surface that Phase 4 must fill from chainstate state instead of bypassing consensus APIs. [VERIFIED: packages/open-bitcoin-consensus/src/context.rs]

### Recommended Connect API Shape

```rust
pub fn connect_block(
    state: &ActiveChainState,
    parent: &ChainIndexEntry,
    block: &Block,
) -> Result<(ActiveChainState, BlockUndo), ConnectBlockError>
```

This keeps the transition pure, returns undo as a first-class artifact, and avoids adapter-owned mutation in the core. [ASSUMED]

## Assumptions Log

| # | Claim | Section | Risk If Wrong |
|---|-------|---------|---------------|
| A1 | The exact new crate name should be `open-bitcoin-chainstate`. | Standard Stack / Architecture Patterns | Low; the boundary matters more than the final package name. |
| A2 | `ChainTransition` is the right top-level name for explicit reorg output. | Minimum Undo Types | Low; only the typed transition concept is required. |
| A3 | The recommended module filenames are a good fit for this crate. | Recommended Module Split | Low; planner can rename files without changing the design. |
| A4 | A pure `connect_block` API returning `(ActiveChainState, BlockUndo)` is the clearest first cut. | Code Examples | Medium; planner may choose a diff-style return instead. |

## Sources

### Primary
- `.planning/PROJECT.md`
- `.planning/REQUIREMENTS.md`
- `.planning/ROADMAP.md`
- `.planning/phases/04-chainstate-and-utxo-engine/04-CONTEXT.md`
- `packages/open-bitcoin-consensus/src/context.rs`
- `packages/open-bitcoin-consensus/src/block.rs`
- `packages/open-bitcoin-primitives/src/block.rs`
- `packages/open-bitcoin-primitives/src/transaction.rs`
- `packages/bitcoin-knots/src/coins.h`
- `packages/bitcoin-knots/src/coins.cpp`
- `packages/bitcoin-knots/src/node/blockstorage.cpp`
- `packages/bitcoin-knots/src/validation.cpp`
- `packages/open-bitcoin-core/src/lib.rs`
- `packages/open-bitcoin-node/src/lib.rs`
- `scripts/verify.sh`
- `docs/parity/index.json`

## Metadata

- Standard stack confidence: HIGH because the boundary decisions are explicit in the phase context and align with the existing workspace split. [VERIFIED: .planning/phases/04-chainstate-and-utxo-engine/04-CONTEXT.md, packages/open-bitcoin-core/src/lib.rs, packages/open-bitcoin-node/src/lib.rs]
- Architecture confidence: HIGH because Knots exposes the consensus-relevant `Coin`, undo, connect, disconnect, and best-chain behaviors directly in the cited sources. [VERIFIED: packages/bitcoin-knots/src/coins.h, packages/bitcoin-knots/src/coins.cpp, packages/bitcoin-knots/src/validation.cpp, packages/bitcoin-knots/src/node/blockstorage.cpp]
- Fixture confidence: HIGH because the targeted cases map directly to the roadmap success criteria and the specific upstream state-transition rules Phase 4 is meant to mirror. [VERIFIED: .planning/ROADMAP.md, packages/bitcoin-knots/src/validation.cpp]
