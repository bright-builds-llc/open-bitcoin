# Architecture Research

**Domain:** Open Bitcoin v2.4 prune-mode product behavior (single chainstate)
**Researched:** 2026-09-21
**Confidence:** HIGH for Open Bitcoin core/shell seams and pinned Knots prune/serving APIs read in-tree; MEDIUM for exact Fjall delete-batch atomicity and height-to-key grouping until a later phase pins those contracts

## Recommendation

Keep prune **decisions** I/O-free. Keep Fjall key deletes, durable `have_pruned` flag writes, and any filesystem effects in `open-bitcoin-node` adapters. Do not move I/O into `open-bitcoin-chainstate`. Do not add a second chainstate, assumeutxo, or archive mode.

Cut wallet leftover-snapshot reads **first** so prune cannot resurrect snapshot bytes as chain truth. Then add typed height-window / lock / `m_have_pruned` policy, then execute deletes, then project `NODE_NETWORK_LIMITED` serving limits and honest `Pruned` labels, then close with operator evidence and no-claim guardrails.

Open Bitcoin stores block payloads as per-hash Fjall keys (`FjallNodeStore::save_block` / `has_block`), not Knots `blk?????.dat` flat files. Behavioral parity is height windows, locks, have-pruned, serving limits, and honest labels — not a line-for-line block-file layout.

## Standard Architecture

### System Overview

```
┌──────────────────────────────────────────────────────────────────────────────┐
│ Effectful entry points                                                        │
│ open-bitcoind | DurableSyncRuntime | inbound getdata/getblocks | RPC/CLI     │
└──────────────────────────────────┬───────────────────────────────────────────┘
                                   │ typed prune + serve commands
┌──────────────────────────────────▼───────────────────────────────────────────┐
│ Node shell: ManagedChainstate + ManagedPeerNetwork + FjallNodeStore          │
│ ┌─────────────────────────┐  ┌────────────────────────┐  ┌─────────────────┐ │
│ │ FlushLifecycle + prune  │  │ Unlink adapter         │  │ Serving project │ │
│ │ orchestrate decide →    │  │ delete block/undo keys │  │ NETWORK_LIMITED │ │
│ │ persist flag → unlink   │  │ set have_pruned        │  │ Pruned label    │ │
│ └───────────┬─────────────┘  └───────────┬────────────┘  └────────┬────────┘ │
└─────────────┼────────────────────────────┼────────────────────────┼──────────┘
              │ decisions (pure)           │ has_block / delete     │ facts
┌─────────────▼────────────────────────────┼────────────────────────┼──────────┐
│ Pure core                                                                  │
│ open-bitcoin-chainstate: height window, prune locks, decide-what-to-prune  │
│ open-bitcoin-network: BlockServingDataAvailability, ServiceFlags, gates    │
│ (no Fjall, no fs, no unlink)                                               │
└─────────────┬────────────────────────────┼────────────────────────┼──────────┘
              │ coins best-block           │                        │
┌─────────────▼────────────────────────────▼────────────────────────▼──────────┐
│ Fjall                                                                        │
│ coins: per-outpoint + best-block   block_index: block:<hex> payloads         │
│ chainstate: undo keys + leftover snapshot blob (non-authoritative)           │
│ prune meta: durable have_pruned (+ lock records as needed)                   │
└──────────────────────────────────────────────────────────────────────────────┘
```

### Component Responsibilities

| Component | Responsibility | Typical implementation |
|-----------|----------------|------------------------|
| `WalletRescanRuntime` | Rescan from coins/headers/blocks, never leftover snapshot | Modify `packages/open-bitcoin-node/src/sync/wallet_rescan.rs` |
| Pure prune policy | Height window, target, locks → set of deletable hashes/ranges | New I/O-free types + functions (prefer `open-bitcoin-chainstate` prune module) |
| `ManagedChainstate` / `FlushLifecycle` | Single-chainstate orchestration: flush coins, then apply prune decision | Modify `packages/open-bitcoin-node/src/chainstate.rs` + `flush_lifecycle.rs` |
| `FjallNodeStore` | Payload probe (`has_block`), save/load, **delete** block/undo keys, persist `have_pruned` | Modify `storage/fjall_store.rs` + `blocks.rs` + coins undo helpers |
| Availability classifier | `Available` iff bytes present; `Pruned` only after prune deleted; else `Unavailable` | Modify `ManagedPeerNetwork::managed_block_serve_input` (`network/inventory.rs`) + pure `BlockServingDataAvailability` |
| Service-flag projector | Advertise `NETWORK` vs `NETWORK_LIMITED` from prune mode | Modify `ServiceFlags` in `open-bitcoin-network/src/message.rs` + local peer config |
| Serving gates | Tip-distance limit + stop `getblocks` when pruned/too-old | Modify block-serving / peer inventory path; mirror Knots `net_processing.cpp` |
| Operator evidence | Sanitized prune/have-bytes/serving evidence; last-gate no-claim | Modify status/metrics/CLI + claim checkers |

## Recommended Project Structure

```
packages/
├── open-bitcoin-chainstate/src/
│   ├── coins/flush.rs              # existing I/O-free flush policy (keep)
│   └── prune/                      # NEW: height window, locks, prune decision types
│       ├── policy.rs               # decide files/hashes to prune (pure)
│       ├── locks.rs                # PruneLockInfo-shaped facts + forbid check
│       └── have_pruned.rs          # typed flag transition (decision only)
├── open-bitcoin-network/src/
│   ├── message.rs                  # MODIFY: ServiceFlags::NETWORK_LIMITED
│   └── block_serving.rs            # MODIFY: wire Pruned into live classification
├── open-bitcoin-node/src/
│   ├── sync/wallet_rescan.rs       # MODIFY FIRST: drop load_chainstate_snapshot truth
│   ├── chainstate.rs               # MODIFY: ManagedChainstate prune orchestration
│   ├── chainstate/flush_lifecycle.rs  # MODIFY: flush-then-prune hook points
│   ├── storage/fjall_store/
│   │   ├── blocks.rs               # MODIFY: has_block + delete_block
│   │   └── coins.rs                # MODIFY: delete_undo alongside block prune
│   ├── network/inventory.rs        # MODIFY: Pruned vs Unavailable facts
│   ├── network/block_serving.rs    # MODIFY: gates + NETWORK_LIMITED distance
│   └── status/                     # MODIFY: prune evidence surfaces
└── bitcoin-knots/src/              # pinned baseline only (read, do not port wholesale)
    ├── node/blockstorage.{h,cpp}
    ├── validation.cpp / validation.h
    ├── net_processing.cpp
    └── protocol.h
```

### Structure Rationale

- **`open-bitcoin-chainstate/prune/`:** Mirrors the v2.3 flush-policy pattern — decisions stay unit-testable and free of Fjall. Knots puts `FindFilesToPrune` on `BlockManager`; Open Bitcoin adapts that to height→hash/key selection without importing filesystem APIs into the core crate.
- **`open-bitcoin-node` adapters:** Own all deletes and durable flag writes, same as `FlushLifecycle` executes `decide_flush` today.
- **`open-bitcoin-network`:** Already owns `BlockServingDataAvailability::{Available, Pruned, Unavailable}` and serving gates; v2.4 makes `Pruned` live instead of reserved.
- **Do not add** a second chainstate package path, LevelDB importer, or archive-serving crate.

## Architectural Patterns

### Pattern 1: Decide → Persist Flag → Unlink (Knots FlushStateToDisk shape)

**What:** Pure policy returns a prune plan; shell writes `have_pruned` when the plan is non-empty; shell deletes payloads last.
**When to use:** Every automatic or manual prune event on the single active chainstate.
**Trade-offs:** Matches Knots ordering in `Chainstate::FlushStateToDisk` (find files → set `m_have_pruned` → write index → `UnlinkPrunedFiles`). Slightly more ceremony than delete-first, but prevents “bytes gone, flag never set” and keeps crash stories auditable.

**Example:**
```rust
// Pure (open-bitcoin-chainstate)
let plan = decide_prune(PrunePolicyInput { tip_height, target_bytes, locks, candidates });
// Shell (ManagedChainstate / FlushLifecycle)
if !plan.is_empty() {
    store.set_have_pruned(true)?;
    store.delete_block_payloads(&plan.hashes_to_delete)?;
}
```

### Pattern 2: Payload Facts Drive Labels (extend v2.3 honesty)

**What:** Serving labels remain a pure function of typed facts. Today `managed_block_serve_input` maps `payload_present → Available else Unavailable` and never emits `Pruned`. After prune product behavior, facts must include `have_pruned` / “deleted by prune” so missing-without-prune stays `Unavailable`.
**When to use:** Every `getdata` / compact serve / inventory eligibility path.
**Trade-offs:** Requires durable prune metadata; cannot infer `Pruned` from absence alone (v2.3 deliberately forbids that).

**Example:**
```rust
let data_availability = match (payload_present, deleted_by_prune) {
    (true, _) => BlockServingDataAvailability::Available,
    (false, true) => BlockServingDataAvailability::Pruned,
    (false, false) => BlockServingDataAvailability::Unavailable,
};
```

### Pattern 3: Service-Flag Projection Separate From Storage

**What:** Prune mode projects `ServiceFlags::NETWORK_LIMITED` (and clears full `NETWORK` when Knots would) independently of per-block availability. Tip-distance gates then refuse deep historical `getdata` even if a payload still exists inside the limited window policy.
**When to use:** Version handshake / local advertisement and block-serve resource gates.
**Trade-offs:** Decouples “we deleted old files” from “we advertise limited history,” matching BIP159 / Knots `NODE_NETWORK_LIMITED` (bit `1 << 10` in `protocol.h`).

## Data Flow

### Prune Decision → Unlink → Availability → Service Flags

```
Tip height + usage + prune target + prune locks
    ↓  (pure decide_prune)
PrunePlan { hashes/ranges, should_set_have_pruned }
    ↓  (shell)
Persist have_pruned flag (if first delete)
    ↓
Delete Fjall block:<hex> (+ undo) keys   ≈ Knots UnlinkPrunedFiles
    ↓
has_block(hash) == false
    ↓  (inventory / block_serving facts)
BlockServingDataAvailability::Pruned   (only if have_pruned + prune deleted)
    ↓  (peer config projection)
ServiceFlags::NETWORK_LIMITED  (+ tip-distance refuse on getdata/getblocks)
```

### Key Data Flows

1. **Wallet rescan cutover:** `WalletRescanRuntime` stops calling `FjallNodeStore::load_chainstate_snapshot()` as authoritative UTXO/chain truth; builds rescan inputs from coins best-block, headers, and present block payloads (or fails closed when payload missing).
2. **Automatic prune on flush:** `FlushLifecycle` / `ManagedChainstate::flush_with_mode` — after coins flush points analogous to Knots `FlushStateToDisk` — asks pure policy for a plan, then shell unlinks.
3. **Manual prune:** RPC/operator command supplies a height; same pure `GetPruneRange`-shaped window (`tip - MIN_BLOCKS_TO_KEEP`, locks) into the same unlink adapter.
4. **Serve path:** `has_block` + prune metadata → `BlockServingStatusLabel`; `NETWORK_LIMITED` distance check (`NODE_NETWORK_LIMITED_MIN_BLOCKS = 288` in Knots `net_processing.cpp`) may disconnect/refuse before storage read.
5. **Restart:** Load durable `have_pruned`; do not resurrect deleted payloads from leftover snapshot blobs.

### State Management

```
Durable: coins best-block + per-outpoint coins + block payloads + have_pruned (+ locks)
In-memory: ManagedChainstate cache, ManagedPeerNetwork blocks_by_hash, serving counters
Leftover snapshot blob: non-authoritative; must not feed wallet rescan or prune honesty
```

## Scaling Considerations

| Scale | Architecture adjustments |
|-------|--------------------------|
| Dev / short chains | Height-window policy + in-memory tests; Fjall deletes are per-hash |
| Mainnet pruned node | Batch deletes; avoid scanning entire keyspace on every flush — maintain height-indexed candidates or file/group metadata if needed |
| Archive / dual chainstate | Out of scope — do not add assumeutxo background chain or archive product mode |

### Scaling Priorities

1. **First bottleneck:** Naïve “scan all block keys every prune” — mitigate with height-indexed candidate sets derived at connect time.
2. **Second bottleneck:** Serving/CPU on tip-window traffic under `NETWORK_LIMITED` — reuse existing request caps in `evaluate_block_serving_resource_gate`.

## Anti-Patterns

### Anti-Pattern 1: Infer Pruned From Missing Bytes

**What people do:** Map `!has_block` → `Pruned`.
**Why it's wrong:** v2.3 honesty and Knots `IsBlockPruned` require `m_have_pruned && !HAVE_DATA && nTx > 0`. Corruption or never-downloaded blocks are `Unavailable`, not pruned.
**Do this instead:** Emit `Pruned` only when prune product behavior deleted the payload (and have_pruned is set).

### Anti-Pattern 2: Unlink Inside `open-bitcoin-chainstate`

**What people do:** Call Fjall/fs from the chainstate crate during `decide_prune`.
**Why it's wrong:** Breaks functional-core policy and the repo architecture verifier contract.
**Do this instead:** Return a `PrunePlan`; `FjallNodeStore` / flush lifecycle executes deletes.

### Anti-Pattern 3: Second Chainstate Or Snapshot Shortcut

**What people do:** Port Knots assumeutxo `GetPruneRange` dual-chainstart logic or keep wallet on leftover snapshots “until prune lands.”
**Why it's wrong:** Out of scope; leftover snapshot as rescan truth lets prune resurrect stale UTXO maps.
**Do this instead:** Single `ManagedChainstate`; wallet cutover phase first.

### Anti-Pattern 4: Advertise Full `NETWORK` While Pruned

**What people do:** Leave `ServiceFlags::NETWORK | WITNESS` defaults after deletes.
**Why it's wrong:** Peers expect historical serving; Knots switches limited peers to `NODE_NETWORK_LIMITED`.
**Do this instead:** Project limited flags and enforce tip-distance on `getdata` / stop `getblocks` when pruned or too old.

## New vs Modified

### New

| Piece | Lives in | Why new |
|-------|----------|---------|
| Prune policy / height-window / target types | `open-bitcoin-chainstate` (pure) | No existing decide-what-to-prune API; Knots `FindFilesToPrune` / `GetPruneRange` |
| `PruneLockInfo`-shaped lock table + forbid check | pure + durable shell records | Knots `m_prune_locks` / `DoPruneLocksForbidPruning` / `PRUNE_LOCK_BUFFER` |
| Durable `have_pruned` (`prunedblockfiles` flag analogue) | `FjallNodeStore` meta | Required for honest `IsBlockPruned` and restart |
| `delete_block` / batch unlink adapter | `FjallNodeStore` | Today only `save_block` / `load_block` / `has_block` |
| `ServiceFlags::NETWORK_LIMITED` | `open-bitcoin-network` | Missing today (`NETWORK`, `WITNESS`, `REPLACE_BY_FEE` only) |
| Tip-distance / getblocks-stop serving policy | network + node serving path | Knots `NODE_NETWORK_LIMITED_MIN_BLOCKS` / prune stop in `getblocks` |

### Modified

| Piece | Path / type | Change |
|-------|-------------|--------|
| `WalletRescanRuntime` | `sync/wallet_rescan.rs` | Stop `required_chainstate_snapshot` → `load_chainstate_snapshot`; use coins + headers + payloads |
| RPC rescan helper | `open-bitcoin-rpc/src/context/rescan.rs` | Same cutover for `partial_chainstate_snapshot` consumers |
| `ManagedChainstate` / `FlushLifecycle` | `chainstate.rs`, `flush_lifecycle.rs` | Hook prune plan execution after flush (Knots `FlushStateToDisk` order) |
| Availability assembly | `network/inventory.rs` `managed_block_serve_input` | Pass prune facts; may emit `Pruned` |
| `BlockServingDataAvailability` / labels | `block_serving.rs` | Activate reserved `Pruned` in live paths (already classified in pure status gates) |
| Compact / getdata serving | `network/block_serving.rs`, announcement transport | Respect pruned + limited-window refusals |
| Local peer / version services | `LocalPeerConfig`, runtime authority defaults | Project limited flags when prune mode active |
| Status / metrics / CLI evidence | `status/`, `HaveBytesAccumulator`, claim checkers | Prune evidence without overclaiming archive/public defaults |
| Tests asserting “no pruned injection” | e.g. inventory / durability tests | Flip from “must not emit Pruned” to “emit only when earned” |

### Explicitly Out (do not build)

| Piece | Why |
|-------|-----|
| Second chainstate / assumeutxo prune range split | Milestone out of scope |
| Literal `blk?????.dat` layout + LevelDB block tree | Open Bitcoin Fjall per-hash payloads; parity is behavioral |
| Archive mode / BIP37 / public defaults / auto destructive reindex | Milestone out of scope |

## Integration Points

### Internal Boundaries

| Boundary | Communication | Notes |
|----------|---------------|-------|
| `decide_prune` ↔ `FlushLifecycle` | Pure plan → shell execute | Same pattern as `decide_flush` → persist |
| `FjallNodeStore::has_block` ↔ inventory | `bool` payload probe | Already the v2.3 honesty seam (`blocks.rs`) |
| Prune meta ↔ `BlockServingDataAvailability` | Facts struct, not globals | Never derive Pruned from probe alone |
| Prune mode ↔ `ServiceFlags` | Config/projection at handshake | Bit `1 << 10` per Knots `protocol.h` |
| `ManagedChainstate` ↔ wallet rescan | Coins best-block + headers + blocks | Snapshot blob is leftover only |
| Serving gates ↔ Knots anchors | Parity breadcrumbs | `net_processing.cpp` getdata limited threshold; getblocks prune stop |

### External / Baseline Anchors (read)

| Knots API | File | Role for Open Bitcoin |
|-----------|------|------------------------|
| `FindFilesToPrune` / `FindFilesToPruneManual` / `PruneOneBlockFile` | `node/blockstorage.cpp` | Height/usage selection + clear HAVE_DATA |
| `UnlinkPrunedFiles` / `ScanAndUnlinkAlreadyPrunedFiles` | `node/blockstorage.cpp` | Adapter deletes after flag |
| `m_have_pruned` / `IsBlockPruned` | `blockstorage.h/.cpp` | Honest pruned labeling |
| `PruneLockInfo` / `DoPruneLocksForbidPruning` | `blockstorage.h/.cpp` | Lock windows + buffer 10 |
| `FlushStateToDisk` prune branch | `validation.cpp` | Ordering: find → set flag → unlink |
| `GetPruneRange` / `MIN_BLOCKS_TO_KEEP` (288) | `validation.cpp` / `validation.h` | Tip keep-window |
| `NODE_NETWORK_LIMITED` | `protocol.h` | Service bit `1 << 10` |
| Limited getdata / pruned getblocks stop | `net_processing.cpp` | Serving limits |

### Open Bitcoin modules cited

| Module | Types / functions read |
|--------|-------------------------|
| `open-bitcoin-network/src/block_serving.rs` | `BlockServingDataAvailability`, `BlockServingStatusLabel`, `classify_block_serving_status` |
| `open-bitcoin-node/src/network/inventory.rs` | `managed_block_serve_input` (Available/Unavailable only today) |
| `open-bitcoin-node/src/network/block_serving.rs` | `BlockServingPresenceFacts`, `gate_managed_block_request` |
| `open-bitcoin-node/src/storage/fjall_store/blocks.rs` | `FjallNodeStore::has_block` |
| `open-bitcoin-node/src/storage/fjall_store.rs` | `save_block` / `load_block` / `load_chainstate_snapshot` |
| `open-bitcoin-node/src/chainstate.rs` | `ManagedChainstate`, `ChainstateStore`, `flush_with_mode` |
| `open-bitcoin-node/src/chainstate/flush_lifecycle.rs` | Flush orchestration |
| `open-bitcoin-node/src/sync/wallet_rescan.rs` | `WalletRescanRuntime`, `required_chainstate_snapshot` |
| `open-bitcoin-network/src/message.rs` | `ServiceFlags` (needs `NETWORK_LIMITED`) |
| `open-bitcoin-chainstate/src/coins/flush.rs` | `decide_flush` pattern to mirror |

## Suggested Build Order (from phase 146)

| Phase | Focus | Why this order |
|-------|-------|----------------|
| **146** | Wallet leftover-snapshot cutover | Removes snapshot-as-truth before any delete; prune cannot resurrect UTXO blob |
| **147** | Prune policy + state | Height windows, target, locks, typed `have_pruned` decisions (still I/O-free) |
| **148** | File/key unlinking adapters | Shell executes plans: delete block/undo keys, persist flag |
| **149** | Serving limits + `Pruned` labeling | `NETWORK_LIMITED`, tip-distance getdata, getblocks stop, honest labels |
| **150** | Operator evidence + no-claim guardrails | Sanitized prune evidence; forbid archive/public-default/production overclaims |

**Dependency rationale:** Cutover → policy → unlink → serve projection → evidence. Serving labels before unlinks would invent `Pruned` without deletes; unlinks before cutover risk rescan reading stale snapshot coins after payloads vanish.

## Sources

- Open Bitcoin v2.3 seams: `ManagedChainstate`, `FlushLifecycle`, `FjallNodeStore::has_block`, `managed_block_serve_input`, `WalletRescanRuntime` (read 2026-09-21)
- Pinned Knots `29.3.knots20260210`: `node/blockstorage.cpp/.h`, `validation.cpp` (`FlushStateToDisk`, `GetPruneRange`), `validation.h` (`MIN_BLOCKS_TO_KEEP`), `net_processing.cpp` (`NODE_NETWORK_LIMITED_*`, getdata/getblocks), `protocol.h` (`NODE_NETWORK_LIMITED`)
- Project constraints: `.planning/PROJECT.md` v2.4 milestone scope; `.planning/ARCHITECTURE.md` package boundaries

---
*Architecture research for: Open Bitcoin v2.4 prune-mode product behavior*
*Researched: 2026-09-21*
