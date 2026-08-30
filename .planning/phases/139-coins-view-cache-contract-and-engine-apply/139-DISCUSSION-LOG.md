# Phase 139: Coins-View, Cache Contract, and Engine Apply - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-08-30
**Phase:** 139-coins-view-cache-contract-and-engine-apply
**Mode:** Yolo
**Areas discussed:** View/cache layering contract, DIRTY/FRESH occupancy vs HaveCoin, Prepare/commit without cloning, Existing snapshot/HashMap compatibility

---

[auto-select] Selected all gray areas: View/cache layering contract, DIRTY/FRESH occupancy vs HaveCoin, Prepare/commit without cloning, Existing snapshot/HashMap compatibility.

---

## View/cache layering contract

| Option | Description | Selected |
|--------|-------------|----------|
| Knots-shaped stack | `CoinsView` trait + resident `CoinsCache` + `MemoryCoinsView`; Chainstate keeps chain/undo plus a cache handle | ✓ |
| Injected cache | Engine apply takes `&mut CoinsCache<impl CoinsView>`; Chainstate keeps only chain/undo | |
| Prepare-only cache | `MemoryCoinsView` owns the live map; `CoinsCache` is prepare-only or eager-flushed after every block | |

**User's choice:** Knots-shaped stack (recommended default)
**Notes:** [auto] Yolo selected the advisor recommendation. Fjall parent stays Phase 141. Do not Clone the cache.

---

## DIRTY/FRESH occupancy vs HaveCoin

| Option | Description | Selected |
|--------|-------------|----------|
| Knots-shaped entry + four lookup facts | Occupancy, HaveCoinInCache, HaveCoin/GetCoin, and parent truth stay distinct; spent DIRTY tombstone shadows parent | ✓ |
| Typed five-state cache enum | Make the five valid DIRTY/FRESH/spent combos structurally exclusive | |
| Presence-only overlay | No spent entries; `have_coin = cache.contains \|\| parent.have_coin` | |

**User's choice:** Knots-shaped entry + four lookup facts (recommended)
**Notes:** [auto] `Coin` stays a live output. Spentness lives on the entry. Presence-only overlay fails success criterion 2.

---

## Prepare/commit without cloning

| Option | Description | Selected |
|--------|-------------|----------|
| Peek-only child `CoinsCache`, Flush into parent on commit | Apply on child; Flush dirty/fresh including spent tombstones; drop child on failure | ✓ |
| Extracted `CoinsBatch` patch | Prepare emits a borrow-free delta; commit is `parent.batch_write` | |
| In-place apply + rollback journal | Mutate the live cache and roll back on failure | |

**User's choice:** Peek-only child cache flushed into parent (recommended)
**Notes:** [auto] Child reads must peek, not populate the parent. Child→parent commit is emptying Flush, not Phase 140 disk Sync.

---

## Existing snapshot/HashMap compatibility

| Option | Description | Selected |
|--------|-------------|----------|
| Parallel types only | New `CoinsView` types; `Chainstate.utxos` HashMap stays the apply target | |
| Apply on cache; snapshot helpers remain | Hydrate `from_snapshot` into MemoryCoinsView + cache; keep snapshot persist until 141–142 | ✓ |
| Demote HashMap and snapshot from production APIs in 139 | Delete snapshot persist and production HashMap ownership now | |

**User's choice:** Engine apply on `CoinsCache`; HashMap/snapshot stay hydrate/export/test helpers (recommended)
**Notes:** [auto] Forbidden clone is connect/prepare/reorg apply, not `snapshot()` at persist edges.

---

## Claude's Discretion

- Exact `coins/` module split
- Bitflags vs five-state enum for DIRTY/FRESH
- Optional cursor types in Phase 139
- `utxos()` view-backed convenience implementation

## Deferred Ideas

- Flush policy and cache-size classification — Phase 140
- Fjall coins adapter and leftover-snapshot non-authority — Phase 141
- Manager flush lifecycle and restart — Phase 142
- Honest availability, operator evidence, parity closeout — Phases 143–145
