# Pitfalls Research

**Domain:** Adding Knots-aligned prune-mode onto Open Bitcoin's v2.3 honest-availability and durable-coins node
**Milestone:** Open Bitcoin v2.4 Prune-Mode Product Behavior
**Baseline:** Bitcoin Knots `29.3.knots20260210`
**Researched:** 2026-09-21
**Confidence:** HIGH for current Open Bitcoin availability, wallet-rescan, and flush-order seams plus pinned Knots prune locks / `m_have_pruned` / `MIN_BLOCKS_TO_KEEP` / `NODE_NETWORK_LIMITED` behavior

## Critical Pitfalls

### Pitfall 1: Wallet Rescan Still Treats Leftover Snapshot Bytes as Chain Truth

**What goes wrong:**
Prune deletes historical block payloads from Fjall, but wallet rescan still hydrates a `ChainstateSnapshot` from the leftover `SNAPSHOT_KEY` blob and walks `active_chain` / `undo_by_block` from that blob. Rescan then "sees" heights the node no longer stores, credits wallet progress through deleted history, or fails closed on a snapshot that chainstate restart already treats as non-authoritative. The honesty contract survives on the serve path and breaks on the wallet path.

**Why it happens:**
v2.3 made leftover snapshots non-authoritative for chainstate open (`hydrate_chainstate_for_open` / schema-2 leftover checks in `packages/open-bitcoin-node/src/storage/fjall_store/coins.rs`), but `WalletRescanRuntime::required_chainstate_snapshot` still calls `store.load_chainstate_snapshot()` in `packages/open-bitcoin-node/src/sync/wallet_rescan.rs`. The NEXT-MILESTONE note names this as explicit leftover debt. A prune implementation that only unlinks blocks leaves that reader intact.

**How to avoid:**
Cut wallet rescan off leftover snapshot bytes before any unlink ships. Drive rescan from durable coins best-block, header/chain meta, and payload-present block reads (or fail closed when the height window lacks bytes). Do not invent a second truth by decoding `SNAPSHOT_KEY` after prune. Keep leftover blobs unread on both restart and rescan.

**Warning signs:**

- `wallet_rescan.rs` still contains `load_chainstate_snapshot` after the first v2.4 phase.
- Rescan succeeds through heights that `has_block` reports false.
- Tests plant a leftover snapshot with fake `active_chain` entries and assert wallet progress without writing matching block keys.
- Operator status shows coins best-block tip while a rescan job advances on snapshot-only heights.

**Phase to address:**
snapshot cutover phase (first; before any delete)

---

### Pitfall 2: Emitting `Pruned` for Any Missing Payload

**What goes wrong:**
A missing payload, failed read, or index-without-bytes case is labeled `Pruned` / `block_status_pruned`. Operators and peers treat that as "this node ran prune," which is a lie when prune never deleted anything. The v2.3 honesty contract collapses: `Unavailable` and `Pruned` become synonyms again.

**Why it happens:**
v2.3 reserved `BlockServingDataAvailability::Pruned` and `BlockServingStatusLabel::Pruned` in `packages/open-bitcoin-network/src/block_serving.rs` and currently forces missing payloads to `Unavailable` in `packages/open-bitcoin-node/src/network/inventory.rs`. Production inventory tests forbid injecting `::Pruned`. The reserved enum is one match-arm away from reuse. Knots `IsBlockPruned` requires `m_have_pruned && !(BLOCK_HAVE_DATA) && nTx > 0` (`packages/bitcoin-knots/src/node/blockstorage.cpp`); missing-without-prune is not pruned.

**How to avoid:**
Gate `Pruned` on a durable have-pruned fact set only after a real delete batch. Keep inventory assembly on `Available` vs `Unavailable` until that fact exists. Map Knots' three-part predicate explicitly; do not treat "payload_present == false" as pruned. Refuse remains refuse either way; the label is the claim.

**Warning signs:**

- `inventory.rs` or serve adapters assign `BlockServingDataAvailability::Pruned` without reading a have-pruned flag.
- Docs or status say "pruned" for corruption, never-downloaded, or side-chain misses.
- Tests assert `block_status_pruned` on a node that never unlinked a key.
- `m_have_pruned` / equivalent is set at prune-config parse time, not after unlink.

**Phase to address:**
label phase (after unlink proves a delete happened)

---

### Pitfall 3: Setting Have-Pruned Before Files Are Actually Deleted

**What goes wrong:**
Config enables `-prune`, or a candidate set is computed, and the node flips `m_have_pruned` / writes a `prunedblockfiles` flag before any payload is gone. Restart then reports pruned history for bytes still on disk, or advertises limited-network behavior while still holding full history. The opposite failure also hurts: files are unlinked but the durable flag is never set, so labels stay `Unavailable` forever and peers cannot distinguish intentional prune from corruption.

**Why it happens:**
Knots sets `m_have_pruned` inside `FlushStateToDisk` only when `setFilesToPrune` is non-empty, writing the block-tree flag then unlinking after index flush (`packages/bitcoin-knots/src/validation.cpp`). Open Bitcoin has no have-pruned store yet. It is easy to "prepare" the flag in a prune-policy phase because the enum already exists.

**How to avoid:**
Set have-pruned only after a non-empty delete batch is committed through the ordered flush path (index updates that clear have-data, then unlink). Persist the flag with the same durability as block-index mutations. On restart, load the flag the way Knots loads `prunedblockfiles`; never infer it from "payload missing."

**Warning signs:**

- Have-pruned becomes true when prune target is configured or when height is below tip − 288 with no unlink.
- Label tests pass by stubbing have-pruned true without calling a delete helper.
- Restart after a failed unlink still claims pruned.
- Status exposes "prune mode on" as if it were "have pruned."

**Phase to address:**
unlink phase (have-pruned is a side effect of successful delete, not of policy alone)

---

### Pitfall 4: Deleting Inside the Undo / Reorg Keep Window

**What goes wrong:**
Payloads or undo for heights within `MIN_BLOCKS_TO_KEEP` (288) of tip are removed. A reorg, interrupted-flush replay, or disconnect then needs undo or block bytes that no longer exist. Coins tip and index tip diverge; replay fails closed; reconnect cannot rebuild the trailing window. Knots refuses to prune above `Height() - 288` via `GetPruneRange` (`packages/bitcoin-knots/src/validation.h`, `validation.cpp`).

**Why it happens:**
Open Bitcoin stores per-hash block keys and separate undo records (`save_block` / `save_undo` in `packages/open-bitcoin-node/src/storage/fjall_store.rs` and `coins.rs`), not Knots blk/rev file pairs. A naive "delete oldest N hashes" or "delete until disk target" ignores file-span height windows and the tip-relative keep band. Interrupted replay in `packages/open-bitcoin-node/src/chainstate/replay.rs` still loads blocks and undo by hash.

**How to avoid:**
Encode tip − 288 as a hard prune ceiling before any candidate selection. Retain undo for the same window as block payloads. Prefer height-window policy first; size targets only choose among candidates already inside the safe range. Never delete the tip window to "make room."

**Warning signs:**

- Prune helpers take a byte budget without a `last_block_can_prune` argument.
- Tests prune height `tip - 10` and call it success.
- Replay or disconnect fixtures start failing only after prune is enabled.
- Undo keys disappear while header meta still lists those heights as recent active history.

**Phase to address:**
prune policy phase

---

### Pitfall 5: Unlinking While a Peer, Wallet, Index, or Rescan Holds a Prune Lock

**What goes wrong:**
A block file / height range under a Knots-shaped prune lock is deleted. Wallet rescan, an index builder, or a temporary RPC lock expected those bytes. Rescan fails mid-chunk, peers stall on NotFound for heights they were told were available, or locks silently no-op because Open Bitcoin never consulted them.

**Why it happens:**
Knots `DoPruneLocksForbidPruning` skips whole files overlapping `height_first`/`height_last` with `PRUNE_LOCK_BUFFER` (10) (`packages/bitcoin-knots/src/node/blockstorage.cpp`). Locks are updated through `UpdatePruneLock` and used by indexes and RPC. Open Bitcoin has wallet rescan jobs and block serving, but no prune-lock table. Implementing unlink without locks copies only the delete half of Knots.

**How to avoid:**
Add named prune locks before production unlink. Hold locks across wallet rescan height ranges and any in-flight serve/read that promised bytes. Skip candidates that intersect locks (including the buffer). Temporary locks must clear; durable locks must restart. Do not treat "default-off serving" as permission to delete under an active rescan.

**Warning signs:**

- Unlink API exists with no lock registry.
- Rescan and prune can run concurrently with no shared height fence.
- Manual prune RPC lands without a lockid surface.
- Logs show deletes for heights a pending `WalletRescanJob` still targets.

**Phase to address:**
unlink phase (locks gate candidates; policy alone is insufficient)

---

### Pitfall 6: Advertising `NODE_NETWORK` After Prune, or `NODE_NETWORK_LIMITED` Without Enforcing the Window

**What goes wrong:**
Two failure modes:

1. After have-pruned, the node still advertises `ServiceFlags::NETWORK` (full history). Peers expect old blocks; requests fail or disconnect; the node looks like a lying full node.
2. The node advertises limited service but still serves (or refuses inconsistently) outside the last 288 (+ race buffer) blocks, or continues advertising `NETWORK` alongside limited in a way that claims full service.

**Why it happens:**
Open Bitcoin hard-codes `ServiceFlags::NETWORK | ServiceFlags::WITNESS` in local peer config / runtime authority paths and does not define `NETWORK_LIMITED` (`1 << 10`) in `packages/open-bitcoin-network/src/message.rs`. Knots defaults local services to `NODE_NETWORK_LIMITED | NODE_WITNESS`, clears full `NODE_NETWORK` when pruned, and refuses requests deeper than `NODE_NETWORK_LIMITED_MIN_BLOCKS + 2` when advertising limited-only (`packages/bitcoin-knots/src/init.cpp`, `net_processing.cpp`, `protocol.h`).

**How to avoid:**
Add `NETWORK_LIMITED`, flip advertised services when have-pruned is durable, and enforce the serve-depth gate on the request path independently of payload presence. Keep payload honesty: limited window still requires `has_block` / payload-present. Do not advertise both full `NETWORK` and "we pruned" in operator copy.

**Warning signs:**

- Prune ships without a `ServiceFlags` bit for limited network.
- Version handshake still always ORs `NETWORK` after have-pruned.
- Serve path only checks payload presence, never tip-relative depth under limited services.
- Tests cover labels but not advertised service bits.

**Phase to address:**
serving-flag phase

---

### Pitfall 7: Unlink Order Breaks the Block / Undo → Index → Coins Flush Contract

**What goes wrong:**
Deletes run before index rows clear have-data, or coins advance past heights whose undo was already removed, or payload keys are removed while index still implies presence. Crash mid-prune leaves index, undo, and Fjall block keys disagreeing. Restart then serves lies, fails replay, or treats missing payloads as ordinary `Unavailable` without have-pruned.

**Why it happens:**
Knots flush order for prune is: find candidates → (on non-empty set) set have-pruned → flush block/undo files → write block index → **unlink** → flush/sync coins (`packages/bitcoin-knots/src/validation.cpp`). Open Bitcoin already has `persist_ordered_prefix`: block payloads → undo → header/index entries, then coins (`packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs`). There is no unlink step yet. Bolting `remove_bytes` onto the front or the coins success path skips the Knots seam.

**How to avoid:**
Extend the single flush owner: persist any pending block/undo/index mutations first, clear have-data / presence metadata in the index write, unlink payload (and matching undo) keys only after that index write succeeds, then coins. Failed unlink after index clear must still yield honest `Unavailable`/`Pruned` based on have-pruned, not a half-deleted store. Do not add a second prune flusher beside `FlushLifecycle`.

**Warning signs:**

- A standalone `prune_now()` deletes keys without going through flush lifecycle.
- Index still reports presence after keys are gone (or the reverse).
- Coins best-block moves in the same batch that deletes undo for that height.
- Tests mock unlink without asserting index and have-pruned durability order.

**Phase to address:**
unlink phase (ordered with existing flush lifecycle)

---

### Pitfall 8: Deleting Block Payloads While Leaving Undo (or the Reverse)

**What goes wrong:**
Open Bitcoin stores block bodies under `block:<hash>` in the block-index namespace and undo under separate chainstate undo keys. Prune removes one and not the other. Reorg/replay finds undo without a block or a block without undo. Disk savings look real; consistency is not.

**Why it happens:**
Knots prune clears `BLOCK_HAVE_DATA` and `BLOCK_HAVE_UNDO` together in `PruneOneBlockFile` and unlinks both blk and rev files. Open Bitcoin's natural delete API is per-key. A size-target loop that only counts block payload bytes will prefer deleting blocks and forget undo.

**How to avoid:**
Treat block payload + undo for a pruned height as one atomic prune unit in the candidate model, even though storage keys differ. Clear both presence facts in the same index/metadata update. Size accounting must include undo bytes.

**Warning signs:**

- Delete helper only calls remove on `block_key`.
- Disk metrics drop block namespace usage but undo prefix usage stays flat.
- Replay tests fail only when prune is enabled.
- `has_block` is false while `load_undo` still returns `Some`.

**Phase to address:**
unlink phase

---

### Pitfall 9: Snapshot or Cache Bytes Resurrect History After Unlink

**What goes wrong:**
Prune removes durable block keys, but an in-memory `blocks_by_hash` cache, a leftover chainstate snapshot, mempool/package admission snapshot export, or wallet partial snapshot still contains the body or UTXO history. Serve path reports `payload_present` true from cache (`inventory.rs` ORs cache with durable presence). Wallet or RPC reconstructs "available" history the disk no longer owns.

**Why it happens:**
v2.3 honesty is durable-payload oriented, but cache short-circuits presence. Leftover `SNAPSHOT_KEY` remains on disk for migration/compat. Package/admission paths still call `export_chainstate_snapshot()` in places. After prune, any reader of those surfaces becomes a resurrection channel.

**How to avoid:**
After unlink, drop matching cache entries. Keep snapshot cutover first so leftover blobs cannot refill history. Do not re-save full snapshots as a prune side effect. Presence probes for serving must not treat cache as stronger than the have-pruned + durable absence contract once prune deleted the key—evict then probe.

**Warning signs:**

- Serve succeeds for a hash after `has_block` is false because cache still holds it.
- Prune does not invalidate `blocks_by_hash`.
- Leftover snapshot rewrite appears in persist_progress after prune work.
- Tests delete store keys but leave runtime cache populated and assert Available.

**Phase to address:**
snapshot cutover phase first; re-verify during unlink phase

---

### Pitfall 10: Claim Creep Into Archive, Assumeutxo, BIP37, LevelDB Import, Auto-Reindex, or Production Readiness

**What goes wrong:**
Docs, checkers, or status copy pitch prune as "full historical serving when not pruned," assumeutxo-friendly storage, LevelDB datadir compatibility, automatic reindex repair after prune inconsistency, public defaults, or production-ready pruned full node. The milestone's no-claim boundary erodes the same way earlier milestones guarded D-14/D-16 style overclaims.

**Why it happens:**
Prune sits next to attractive neighbors: archive is the opposite operator story; assumeutxo wants dual chainstate (v2.3 refused); recovery enums already mention `StorageRecoveryAction::Reindex`; v1.8 production gates remain unmet. Phase 145 checkers currently **fail** docs that claim prune/archive/assumeutxo product behavior—those guards must be rewritten carefully to allow scoped prune claims without opening the rest.

**How to avoid:**
Keep a dedicated no-claim guardrail phase: allow only evidenced prune-mode product behavior; keep archive, assumeutxo, BIP37, LevelDB live import/export, automatic destructive reindex, public defaults, and production-funds/production-readiness deferred. Fail closed on inconsistent prune state with operator diagnosis; do not auto-reindex. Single active chainstate stays single.

**Warning signs:**

- README says "archive when prune is off."
- Recovery path spawns reindex without an explicit future milestone.
- Dual-chainstate or assumeutxo types appear in prune PRs.
- Claim checkers are deleted instead of retargeted.
- Status equates have-pruned with production-ready disk management.

**Phase to address:**
no-claim guardrail phase (late; retarget checkers, do not gut them)

---

## Technical Debt Patterns

Shortcuts that seem reasonable but create long-term problems.

| Shortcut | Immediate Benefit | Long-term Cost | When Acceptable |
|----------|-------------------|----------------|-----------------|
| Reuse `Pruned` for all missing payloads | One label, fewer match arms | Destroys v2.3 honesty; peers cannot tell corruption from prune | Never |
| Set have-pruned at config parse | Easy status bit | Lies after crash before first delete; wrong service flags | Never |
| Delete per-hash without height window | Fast disk reclaim | Breaks reorg/undo and Knots observable window | Never |
| Skip prune locks until "later" | Ships unlink sooner | Wallet/index races; hard to retrofit safely | Never for any unlink that can run beside rescan/serve |
| Keep advertising `NETWORK` after prune | Avoids service-flag work | Peer mis-expectation and stall/disconnect behavior | Never once have-pruned is true |
| Auto-reindex on prune inconsistency | Looks self-healing | Hidden destructive datadir mutation; claim-boundary violation | Never in v2.4 |
| Treat leftover snapshot as rescan input until cutover "someday" | Avoids wallet work | Prune resurrects snapshot-as-truth | Never; cutover is first phase |
| Count only block payload bytes toward prune target | Simpler metrics | Undo retained forever; disk target never met honestly | Only in throwaway prototypes, not milestone code |

## Integration Gotchas

Common mistakes when connecting prune to existing Open Bitcoin seams and Knots observables.

| Integration | Common Mistake | Correct Approach |
|-------------|----------------|------------------|
| Block serving (`inventory.rs` + `block_serving.rs`) | Inject `Pruned` whenever `payload_present` is false | Keep `Unavailable` until have-pruned ∧ intentional delete facts; both labels still refuse serve |
| Durable presence (`has_block`) | Delete keys but leave index/cache implying presence | Clear presence metadata, evict cache, then unlink; probe durable keys on serve |
| Wallet rescan (`wallet_rescan.rs`) | Continue `load_chainstate_snapshot` after prune | Cut over to coins/headers/payload reads before unlink |
| Flush lifecycle (`persist_ordered_prefix`) | Unlink outside ordered flush | Block/undo → index → unlink → coins, one owner |
| Service flags (`ServiceFlags`) | Leave `NETWORK \| WITNESS` forever | Add `NETWORK_LIMITED`; advertise limited after have-pruned; enforce depth gate |
| Knots parity docs | Cite `-prune` without `m_have_pruned` / locks / limited serve | Breadcrumb `blockstorage.cpp`, `validation.cpp` flush prune path, `net_processing.cpp` limited threshold |
| Claim checkers (Phase 145 style) | Delete prune bans wholesale | Retarget: allow scoped prune claims; keep archive/assumeutxo/auto-reindex/production bans |
| Fjall key layout vs Knots blk/rev files | Pretend file-number prune maps 1:1 | Preserve height windows, locks, have-pruned, and atomic payload+undo units; adapt storage granularity |

## Performance Traps

Patterns that work on short chains but fail as mainnet history grows.

| Trap | Symptoms | Prevention | When It Breaks |
|------|----------|------------|----------------|
| Scanning every block hash to pick prune candidates each flush | Flush latency spikes; IBD stalls | Maintain height-ordered candidate metadata; prune on policy triggers like Knots `m_check_for_pruning` | Chains with hundreds of thousands of stored payloads |
| Pruning one hash at a time toward a byte target every block | Constant unlink churn; write amplification | Batch candidates; avoid re-prune storms (Knots buffers under target, especially in IBD) | Sustained IBD with tight prune target |
| Loading full leftover snapshots to decide what to delete | Memory spikes; resurrection risk | Never read leftover snapshot for prune or rescan after cutover | Any datadir that still has a large `SNAPSHOT_KEY` |
| Serving-depth checks that walk full active chain each request | CPU burn on getdata | Compare tip height vs request height with the limited-window constant | Busy pruned peers |

## Security Mistakes

Domain-specific integrity and peer-safety issues for pruned nodes.

| Mistake | Risk | Prevention |
|---------|------|------------|
| Serving below limited threshold while advertising limited-only | Peer stall attacks / unexpected disconnects; prune-height leakage patterns Knots avoids | Enforce `NODE_NETWORK_LIMITED_MIN_BLOCKS + 2` style depth gate before read |
| Advertising full `NETWORK` after deletes | Peers rely on absent history; eclipse/stall surface | Flip services with have-pruned |
| Deleting under an active wallet rescan without locks | Wallet state corruption or false balances from partial history | Prune locks across rescan ranges |
| Auto-reindex / silent datadir repair after prune inconsistency | Destructive mutation of operator data | Fail closed; diagnose; explicit future repair milestone only |
| Treating index membership as proof after prune | Serve path lies; validation/replay confusion | Payload-present remains the serve gate; have-pruned only changes the label |

## UX Pitfalls

Operator-facing mistakes specific to adding prune on an honesty-first node.

| Pitfall | User Impact | Better Approach |
|---------|-------------|-----------------|
| Status says "pruned" when bytes were never deleted | Operator misdiagnoses corruption as intentional prune | Reserve pruned for have-pruned deletes; keep unavailable otherwise |
| "Prune enabled" conflated with "have pruned" | Expectation that old blocks are already gone | Separate config/mode from durable have-pruned evidence |
| No tip-relative keep-window explanation | Operator sets aggressive prune and breaks reorg | Document 288-block keep window and locks in operator copy |
| Implying archive completeness when prune is off | Overclaim vs v2.3 honesty-only serving | "We serve what we store" ≠ archive product mode |
| Suggesting automatic reindex to "fix" prune | Encourages destructive recovery | Point to fail-closed diagnosis and safe retry / free-disk actions |

## "Looks Done But Isn't" Checklist

Things that appear complete but are missing critical pieces.

- [ ] **Snapshot cutover:** Wallet rescan no longer calls `load_chainstate_snapshot` — verify `wallet_rescan.rs` and tests fail if leftover snapshot is the only history source
- [ ] **Height window:** Candidates respect tip − 288 and retain undo — verify policy unit tests reject in-window deletes
- [ ] **Prune locks:** Unlink skips locked ranges (with buffer) — verify rescan/serve lock fixtures
- [ ] **Have-pruned:** Flag flips only after non-empty durable delete — verify restart loads flag; config-only does not set it
- [ ] **Labels:** Missing-without-prune stays `Unavailable`; post-delete uses `Pruned` — verify inventory never injects `Pruned` without have-pruned
- [ ] **Service flags:** `NETWORK_LIMITED` advertised and depth-enforced — verify handshake bits and getdata depth cases
- [ ] **Flush order:** Unlink sits after index write and before/with coins correctly — verify crash fixtures at each seam
- [ ] **Cache eviction:** In-memory bodies cannot resurrect deleted hashes — verify serve after unlink with warm cache
- [ ] **No-claim guards:** Archive, assumeutxo, BIP37, LevelDB import, auto-reindex, production readiness still banned — verify retargeted checkers
- [ ] **Atomic payload+undo:** Both keys cleared together — verify size accounting and load APIs

## Recovery Strategies

When pitfalls occur despite prevention, how to recover. Do **not** treat automatic destructive reindex as the default recovery path.

| Pitfall | Recovery Cost | Recovery Steps |
|---------|---------------|----------------|
| Rescan read leftover snapshot after prune | MEDIUM | Stop using snapshot; fail closed pending jobs; rerun rescan from coins/headers/payload-present sources only |
| Labeled Pruned without have-pruned | LOW | Revert label mapping; keep refuse-as-Unavailable; add regression test |
| Deleted inside keep window | HIGH | Fail closed on replay/disconnect; restore from backup if available; do not auto-reindex; operator restores datadir or resyncs explicitly |
| Unlink vs index disagreement | HIGH | Diagnose which store advanced; refuse serve/progress; operator restores backup or plans explicit repair milestone—no silent rewrite |
| Wrong service flags | LOW–MEDIUM | Correct advertised flags; disconnect mis-served peers; document temporary peer churn |
| Claim creep in docs/checkers | LOW | Retarget wording; restore deferred bans; do not widen checkers to "anything storage" |
| Cache resurrected bodies | LOW | Evict cache entries for unlinked hashes; re-probe `has_block` |

## Pitfall-to-Phase Mapping

How roadmap phases should address these pitfalls. Phase numbers continue at 146; roles below are the planning handles.

| Pitfall | Prevention Phase | Verification |
|---------|------------------|--------------|
| Wallet rescan leftover snapshot as truth | snapshot cutover phase | Rescan fixtures with leftover `SNAPSHOT_KEY` present and block keys absent must not invent history |
| Emitting `Pruned` for any miss | label phase | Inventory/serve tests: missing-without-have-pruned → `Unavailable`; after delete → `Pruned` |
| Have-pruned before real delete | unlink phase | Flag false after config-only; true only after durable non-empty unlink + restart |
| Delete inside undo/reorg window | prune policy phase | Policy rejects tip − &lt;288; undo retained for window |
| Unlink under active locks | unlink phase | Locked height ranges survive prune candidate selection |
| `NETWORK` / `NETWORK_LIMITED` mistakes | serving-flag phase | Handshake bits + depth-gate tests match Knots limited window (+ race buffer) |
| Flush-order / index disagreement | unlink phase | Crash seams: after index clear, after unlink, after coins—each honest and restartable without auto-reindex |
| Block without undo (or reverse) | unlink phase | Paired delete tests; both `has_block` and undo absence |
| Cache/snapshot resurrection | snapshot cutover phase + unlink phase | Warm-cache serve after unlink is not Available; leftover snapshot unread |
| Archive / assumeutxo / auto-reindex claim creep | no-claim guardrail phase | Retargeted checkers allow scoped prune only; deferred surfaces still fail |

**Suggested phase order (dependency):**
1. snapshot cutover phase
2. prune policy phase
3. unlink phase (locks + ordered delete + have-pruned)
4. label phase
5. serving-flag phase
6. no-claim guardrail phase

## Sources

- Open Bitcoin availability and reserved prune labels: `packages/open-bitcoin-network/src/block_serving.rs`, `packages/open-bitcoin-node/src/network/inventory.rs`, `packages/open-bitcoin-node/src/network/tests/block_serving.rs`, `docs/parity/catalog/p2p.md`
- Open Bitcoin durable presence probe: `packages/open-bitcoin-node/src/storage/fjall_store/blocks.rs` (`has_block`)
- Open Bitcoin flush order: `packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs` (`persist_ordered_prefix`)
- Open Bitcoin coins leftover non-authority vs wallet rescan snapshot read: `packages/open-bitcoin-node/src/storage/fjall_store/coins.rs`, `packages/open-bitcoin-node/src/sync/wallet_rescan.rs`
- Open Bitcoin interrupted replay needs blocks/undo: `packages/open-bitcoin-node/src/chainstate/replay.rs`
- Open Bitcoin service flags today: `packages/open-bitcoin-network/src/message.rs` (no `NETWORK_LIMITED` yet)
- Milestone scope and dangerous habits: `.planning/PROJECT.md`, `.planning/reports/NEXT-MILESTONE-CANDIDATES.md`
- Knots keep window and prune range: `packages/bitcoin-knots/src/validation.h` (`MIN_BLOCKS_TO_KEEP = 288`), `packages/bitcoin-knots/src/validation.cpp` (`GetPruneRange`, `FlushStateToDisk` prune/unlink/coins order)
- Knots have-pruned and IsBlockPruned: `packages/bitcoin-knots/src/node/blockstorage.cpp` / `.h` (`m_have_pruned`, `IsBlockPruned`, `PruneOneBlockFile`, `UnlinkPrunedFiles`)
- Knots prune locks: `packages/bitcoin-knots/src/node/blockstorage.cpp` (`DoPruneLocksForbidPruning`, `PRUNE_LOCK_BUFFER`, `UpdatePruneLock`)
- Knots limited network serving: `packages/bitcoin-knots/src/protocol.h` (`NODE_NETWORK_LIMITED`), `packages/bitcoin-knots/src/net_processing.cpp` (`NODE_NETWORK_LIMITED_MIN_BLOCKS`, serve-depth disconnect), `packages/bitcoin-knots/src/init.cpp` (default limited services)
- Claim-boundary pattern to retarget, not delete: `scripts/check-phase145-parity-uat-release-boundary.test.ts`

---
*Pitfalls research for: Adding prune-mode onto Open Bitcoin v2.3 honesty + durable coins*
*Researched: 2026-09-21*
