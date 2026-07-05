# Phase 114: Compact Block Reconstruction from Mempool State - Discussion Log

> **Audit trail only.** Decisions are captured in CONTEXT.md.

**Date:** 2026-07-05
**Phase:** 114-compact-block-reconstruction-from-mempool-state
**Mode:** Yolo
**Areas discussed:** Short ID surface, reconstruction state, input boundaries, lifecycle hooks, scope isolation

---

## Short ID And Selector Surface

| Option | Description | Selected |
|--------|-------------|----------|
| Selector + SipHash in codec with new crypto dep | Single crate owns all BIP152 math | |
| Selector in codec, SipHash in consensus (recommended) | Preserves dependency direction | ✓ |
| Defer short IDs to Phase 115 | Faster now, blocks reconstruction tests | |

**Auto-selected:** Selector in codec, SipHash in consensus, six-byte match keys for hash maps.

## Reconstruction State Model

| Option | Description | Selected |
|--------|-------------|----------|
| `PartialCompactBlock` in network crate (recommended) | Matches Phase 110–113 pure policy placement | ✓ |
| Reconstruction in node shell only | Harder to unit test without I/O | |
| Shared state inside peer manager only | Couples policy to connection bookkeeping | |

**Auto-selected:** `PartialCompactBlock` with `Ready | Invalid | Failed` outcomes.

## Input Boundaries

| Option | Description | Selected |
|--------|-------------|----------|
| Iterator `(Wtxid, Transaction)` inputs (recommended) | Keeps network crate mempool-independent | ✓ |
| Direct `open-bitcoin-mempool` dependency in network | Creates layering pressure | |

**Auto-selected:** Iterator inputs for mempool and extra transactions.

## Lifecycle Integration

| Option | Description | Selected |
|--------|-------------|----------|
| Pure hooks on partial state (recommended) | Shell wires events later | ✓ |
| Wire-aware lifecycle in peer manager now | Blurs Phase 115 scheduling boundary | |

**Auto-selected:** `on_mempool_transaction_removed` and `on_block_connected` clear volatile slots only.

## Scope Isolation

| Option | Description | Selected |
|--------|-------------|----------|
| Reconstruction only; defer getblocktxn/blocktxn (recommended) | Matches roadmap Phase 115 boundary | ✓ |
| Implement full compact download loop now | Scope creep across phases | |

**Auto-selected:** No getblocktxn/blocktxn, FillBlock, or validation handoff in Phase 114.
