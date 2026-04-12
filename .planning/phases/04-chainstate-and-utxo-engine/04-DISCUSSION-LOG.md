# Phase 4: Chainstate and UTXO Engine - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-04-12
**Phase:** 04-chainstate-and-utxo-engine
**Areas discussed:** chainstate core shape, connect/disconnect behavior, reorg policy, adapter boundary, parity fixture strategy

---

## Chainstate Core Shape

| Option | Description | Selected |
|--------|-------------|----------|
| Reuse `open-bitcoin-consensus` | Put mutable chainstate types into the existing consensus crate | |
| New pure-core chainstate crate | Isolate UTXO and active-chain logic in a dedicated pure crate | ✓ |
| Node-owned state only | Keep chainstate logic in `open-bitcoin-node` adapters | |

**User's choice:** New pure-core chainstate crate
**Notes:** Phase 4 needs a reusable pure-core state engine that later phases can consume without dragging runtime adapters into consensus logic.

---

## Connect And Disconnect Behavior

| Option | Description | Selected |
|--------|-------------|----------|
| Store only the latest UTXO snapshot | Recompute undo information on demand | |
| Persist explicit undo records | Capture spend metadata during connect and replay it during disconnect | ✓ |
| Skip disconnect support for now | Treat reorg handling as a later phase | |

**User's choice:** Persist explicit undo records
**Notes:** Phase success criteria already include disconnect and reorg behavior, so undo data has to be part of the first implementation rather than deferred.

---

## Reorg Policy

| Option | Description | Selected |
|--------|-------------|----------|
| Height-only preference | Prefer the longest branch regardless of work | |
| Cumulative-work selection | Prefer the branch with more work and apply deterministic tie-breaking | ✓ |
| Manual branch choice | Leave best-chain selection to adapters | |

**User's choice:** Cumulative-work selection with deterministic tie-breaking
**Notes:** This keeps the pure core aligned with baseline chain selection while still making fixtures deterministic.

---

## Adapter Boundary

| Option | Description | Selected |
|--------|-------------|----------|
| Pure core owns persistence | Put snapshot and storage mutation directly in the chainstate crate | |
| Adapter-owned persistence | Keep pure-core transitions separate from snapshot storage adapters | ✓ |
| No adapter surface yet | Delay persistence abstractions until networking | |

**User's choice:** Adapter-owned persistence
**Notes:** This preserves the repo’s functional-core boundary and leaves room for a future disk-backed adapter without forcing I/O into the chainstate core.

---

## Parity Fixture Strategy

| Option | Description | Selected |
|--------|-------------|----------|
| Full Knots harness port | Mirror the entire chainstate test stack now | |
| Targeted repo-owned fixtures | Cover add, spend, undo, disconnect, and reorg with deterministic fixtures anchored to Knots semantics | ✓ |
| Docs-only parity claim | Rely on reasoning and postpone executable fixtures | |

**User's choice:** Targeted repo-owned fixtures
**Notes:** The project needs auditable parity, but the first chainstate slice should stay bounded and executable inside the current Rust workspace.

---

## the agent's Discretion

- Exact type names for chain entries, snapshots, and undo bundles
- Internal indexing layout for active-chain and side-branch lookup tables
- The minimal adapter trait surface needed to keep storage out of the pure core

## Deferred Ideas

- LevelDB-backed coins views
- assumeutxo and snapshot validation
- mempool-coupled spend views
- wallet balance projections

---

*Phase: 04-chainstate-and-utxo-engine*  
*Discussion log generated: 2026-04-12*
