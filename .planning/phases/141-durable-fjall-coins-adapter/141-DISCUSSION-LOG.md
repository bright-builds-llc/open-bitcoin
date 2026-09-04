# Phase 141: Durable Fjall Coins Adapter - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-09-04
**Phase:** 141-durable-fjall-coins-adapter
**Mode:** Yolo
**Areas discussed:** Per-coin key schema, Dedicated coins keyspace, Dirty-batch accounting and head_blocks, Compact codec home, Undo records vs leftover snapshot DTO, Schema bump and one-way migration, Fail-closed disk reads

---

## Per-coin key schema

| Option | Description | Selected |
|--------|-------------|----------|
| Knots-shaped first-party prefixes | `C`+txid+vout, `B` best-block, `H` head_blocks; compact binary values; not LevelDB-byte-identical | ✓ |
| JSON snapshot keys per outpoint | Pretty-JSON values under `coin:<txid>:<vout>` in the existing chainstate keyspace | |
| Byte-identical LevelDB `chainstate/` | Port `dbwrapper` / rusty-leveldb so keys match Knots files | |

**User's choice:** Knots-shaped first-party prefixes (yolo recommended default)
**Notes:** [auto] Migration apply and Knots datadir file compatibility are out of v2.3. First-party compact records preserve observable Flush/Sync behavior without LevelDB.

---

## Dedicated coins keyspace

| Option | Description | Selected |
|--------|-------------|----------|
| New `StorageNamespace::Coins` | Dedicated `"coins"` keyspace; small tip/undo metadata stays in `chainstate` | ✓ |
| Prefix inside existing `chainstate` | Stuff millions of outpoints next to leftover `"snapshot"` | |
| New production crate | Split Fjall coins into a dedicated crate | |

**User's choice:** New `StorageNamespace::Coins` (yolo recommended default)
**Notes:** [auto] Research STACK prefers a dedicated keyspace. No new crates — extend `FjallNodeStore`.

---

## Dirty-batch accounting and head_blocks

| Option | Description | Selected |
|--------|-------------|----------|
| 64 MiB first-party payload + two-phase `H` | Cap estimated encoded bytes at `64 << 20`; `H=[new,old]` around the write; fail closed on inconsistent markers | ✓ |
| Single atomic Fjall batch only | Skip `head_blocks` because one Fjall batch is atomic | |
| Count-based batches | Use Fjall `len()` as a SizeEstimate substitute | |

**User's choice:** 64 MiB first-party payload + two-phase `H` (yolo recommended default)
**Notes:** [auto] Fjall persist changes durability, not consistency. Replay execution stays Phase 142; this phase persists markers and fails closed.

---

## Compact codec home

| Option | Description | Selected |
|--------|-------------|----------|
| Node `storage/coins_codec.rs` | Parse at the Fjall boundary into domain `Coin`, including MTP | ✓ |
| Pure codec in `open-bitcoin-chainstate` | Put disk encoding next to `Coin` | |
| Reuse snapshot JSON | Keep `encode_chainstate_snapshot` as coin truth | |

**User's choice:** Node `storage/coins_codec.rs` (yolo recommended default)
**Notes:** [auto] Chainstate stays I/O-free. Snapshot JSON is leftover migration source only.

---

## Undo records vs leftover snapshot DTO

| Option | Description | Selected |
|--------|-------------|----------|
| Own undo records in `chainstate` | `save_undo` / `load_undo`; leftover snapshot is not live undo or UTXO truth | ✓ |
| Keep undo in leftover snapshot DTO | Coins keyspace for UTXOs; disconnect still reloads the blob | |
| Defer undo entirely to Phase 142 | Coins only; undo stays blob-shaped until manager work | |

**User's choice:** Own undo records in `chainstate` (yolo recommended default)
**Notes:** [auto] Making the snapshot non-authoritative without an undo home would force Phase 142 to invent undo or remigrate. `save_block` / `load_block` stay unchanged.

---

## Schema bump and one-way migration

| Option | Description | Selected |
|--------|-------------|----------|
| Bump `SchemaVersion` 1→2 + explicit one-way migrate | Fail closed on mismatch; leftover snapshot is a one-shot source then non-authoritative | ✓ |
| Dual-read generation | Serve coins if present else fall back to snapshot blob | |
| Per-namespace schema versions | Version coins independently so wallet/mempool/runtime stay on schema 1 | |

**User's choice:** Bump `SchemaVersion` 1→2 + explicit one-way migrate (yolo recommended default)
**Notes:** [auto] Dual-read is the dual-truth pitfall. One store-level version; other snapshot codecs remain readable under schema 2. persist_progress write-site cutover stays Phase 142.

---

## Fail-closed disk reads

| Option | Description | Selected |
|--------|-------------|----------|
| Typed storage/recovery error | I/O, corruption, and decode failures never become spent or `Ok(None)` | ✓ |
| Treat read errors as missing | Map Fjall errors to `None` so connect can continue | |
| Catcher wrapper that logs and continues | Swallow errors in a `CCoinsViewErrorCatcher`-style adapter | |

**User's choice:** Typed storage/recovery error (yolo recommended default)
**Notes:** [auto] CSOBS-03 is owned here at the first durable-read seam. `Ok(None)` is successful absence only.

---

## Claude's Discretion

- Exact compact value layout (varint vs fixed) as long as MTP/height round-trip
- Exact undo key spelling
- `FjallCoinsView` as standalone type vs module beside `FjallNodeStore`
- How `ChainstateStore` grows without breaking leftover persist write sites

## Deferred Ideas

- Manager init, CanFlush, flush-mode wiring, persist_progress write cutover, replay vs fail-closed — Phase 142
- Honest availability, operator evidence, no-claim guardrails — Phases 143–145
- Assumeutxo, prune/archive, LevelDB, rust-bitcoin — out of v2.3
