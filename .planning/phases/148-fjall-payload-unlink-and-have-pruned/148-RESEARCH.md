# Phase 148: Fjall Payload Unlink and Have-Pruned - Research

**Researched:** 2026-09-26
**Domain:** Fjall 3.1.4 paired payload deletion, durable have-pruned, and interrupted-prune resume on the existing flush owner
**Confidence:** HIGH

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

### Paired delete unit

- **D-01:** One prune unit is the block payload and the undo for the same
  height. Both Fjall keys are removed together. Deleting only the block
  key, or only the undo key, is not a successful prune of that height.
- **D-02:** No `blk`/`rev` files and no second block store. Keys stay the
  existing `block:<64-hex>` payload key and the existing undo key for that
  block hash.
- **D-03:** A height is done only when both keys are absent. If one mate is
  already absent, delete the remaining mate. Do not invent an empty block
  or undo to fill the gap.
- **D-04:** Resolve each planned height to the active-chain block hash
  already known to chainstate or the block index, then delete those two
  keys. Do not scan unrelated namespaces and do not delete by a height
  prefix the store does not use.

### Have-pruned durability

- **D-05:** Record have-pruned only after a non-empty paired delete batch
  is durable. Prune config, an empty plan, and a candidate list that has
  not yet committed do not set the flag.
- **D-06:** An empty `PrunePlan` is success without a delete. Leave
  have-pruned unchanged. It is not a failure and it is not a reason to
  mark the node pruned.
- **D-07:** Have-pruned becomes visible only when the committed batch
  removed at least one pair and both keys of every height in that batch
  are absent. If the batch fails, have-pruned stays false.

### Interrupted prune

- **D-08:** Restart either finishes the same interrupted height's paired
  delete or refuses closed. Both outcomes are valid. Inventing block or
  undo bytes is not. Automatic reindex and coins repair stay out of scope
  (FUT-23).
- **D-09:** Crash seams to prove: index or presence cleared while keys
  remain; one of the two keys deleted; have-pruned set while either key
  remains; coins best-block advanced in the same batch that deletes undo
  for that height. Each seam fails closed or completes the same height.
  None of them synthesize payloads.
- **D-10:** A refuse-closed restart must not serve or reconstruct the
  missing payload as if the height were still fully stored. It also must
  not report the delete as successful when a key remains.

### Flush order and lock consumption

- **D-11:** Unlink stays on the existing flush lifecycle
  (`persist_ordered_prefix` / `FlushLifecycle`). Persist pending
  block, undo, and index mutations first, clear presence for the pruned
  heights, delete the paired keys, record have-pruned only after that
  delete commits, then flush coins. Do not add a second prune flusher.
- **D-12:** Do not advance coins best-block inside the delete batch. Do
  not delete undo for a height the node still needs to disconnect the
  active tip. The Phase 147 keep window already excludes the last 288
  blocks; unlink must not widen that set.
- **D-13:** Consume `PrunePlan.heights` from `plan_automatic_prune` /
  `plan_manual_prune`. Also skip any height
  `height_forbidden_by_any_lock` still protects, including the 10-block
  buffer. Skipping a protected height does not fail the rest of the plan.
  A protected height's block and undo keys remain after the attempt.

### Cache and later-phase boundary

- **D-14:** After a durable delete, drop the deleted hashes from the
  in-memory block cache before a later presence probe. Do not rewrite a
  full chainstate snapshot as a prune side effect. Leftover snapshot bytes
  stay non-authoritative, as Phase 146 already decided.
- **D-15:** Do not advertise `NODE_NETWORK_LIMITED`, do not emit `Pruned`
  versus `Unavailable`, and do not add RPC, CLI, or dashboard prune
  fields. Phases 149 and 150 own those surfaces. This phase only makes the
  deleted payloads actually gone.
- **D-16:** New first-party Rust source and tests under
  `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests` get
  parity breadcrumbs through `docs/parity/source-breadcrumbs.json`. Cite
  pinned Knots prune-unlink anchors where a defensible source line exists.
  The v2.4 parity-doc pass and no-claim checkers stay in Phase 151.
- **D-17:** Verification remains `bash scripts/verify.sh`. Default
  verification stays deterministic and public-network-free. No new
  production crate or third-party library.

### Claude's Discretion

- Whether have-pruned lives in runtime metadata or a dedicated chainstate
  marker, as long as it is durable, loaded on restart, and never inferred
  from "payload missing" or from prune config.
- The exact Fjall write-batch shape, as long as a successful commit cannot
  leave one mate key behind while reporting the height pruned, and a
  failed commit cannot set have-pruned.
- How the interrupted-prune intent is recorded, as long as restart can
  finish or refuse that same height without reindex.
- Module placement inside the existing node storage and flush lifecycle,
  as long as chainstate policy stays I/O-free and unlink is not a second
  flusher.

### Deferred Ideas (OUT OF SCOPE)

- `NODE_NETWORK_LIMITED` advertisement and limited-window getdata refusal
  — Phase 149.
- Honest `Pruned` versus `Unavailable` labels — Phase 149. This phase must
  not flip inventory to `Pruned` just because a key is missing.
- Prune RPC, CLI, dashboard fields, manual prune invocation, and operator
  prune-lock setters — Phase 150.
- Full v2.4 parity-doc pass and no-claim checkers — Phase 151.
- `-pruneduringinit` — FUT-27, not this milestone.
- Automatic destructive reindex or coins repair — FUT-23.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| UNLK-01 | A prune of a height removes that height's block payload and undo together. | One `fjall::WriteBatch` removes `block:<64-hex>` in the block-index keyspace and `undo:<64-hex>` in the chainstate keyspace. Resolve the hash from active-chain `ChainPosition`. Skip lock-protected and keep-window heights. |
| UNLK-02 | The node records have-pruned only after that delete is durable. | Insert the `have_pruned` marker in that same `PersistMode::Sync` batch, only when a pre-check saw at least one live mate. Empty plans and failed commits leave the marker unchanged. Do not store it in `RuntimeMetadata`. |
| UNLK-03 | Restart after an interrupted prune finishes or refuses the partial delete without inventing blocks or reindexing. | A prior `prune_intent` key names one height and hash. Resume deletes the remaining mate or returns `StorageError::Corruption` with `StorageRecoveryAction::Repair`, never `Reindex`. Do not synthesize payloads. |
</phase_requirements>

## Summary

Open Bitcoin already stores a block body at `block:<64-hex>` in the block-index keyspace and that block's undo at `undo:<64-hex>` in the chainstate keyspace. Those are different keyspaces, and the existing header save already commits both keyspaces in one `Database::batch()`. Fjall 3.1.4 documents that batch as atomic across keyspaces, and journal recovery discards a batch that never received its terminator. A successful `PersistMode::Sync` commit is the durability bar for a height: both tombstones and the have-pruned marker land together, or the commit error leaves have-pruned false.

There is no separate have-data bit. `has_block` is `contains_key` on the payload key. Clearing presence is the payload tombstone. Header entries stay. The in-memory risks that can put bytes back are `Chainstate::undo_by_block`, which the next flush rewrites through `persist_ordered_prefix`, and `PeerNetwork::blocks_by_hash`, which inventory ORs with durable presence.

**Primary recommendation:** Extend `FlushLifecycle::execute_flush` so that, after `persist_ordered_prefix` and before `cache.flush()`, it commits one Sync batch per still-present height: tombstone both keys, insert `have_pruned` in the block-index keyspace, and clear a previously synced `prune_intent`. Do not start an automatic prune from the sync loop in this phase. Existing flush callers pass an empty plan.

## Discretion Recommendations

These four choices are the ones the planner should lock into tasks.

### Have-pruned marker

Use a dedicated key `have_pruned` with value `1` in the **block-index** keyspace. Load it with `contains_key` on restart. Once written, it stays written.

Do not put the flag on `RuntimeMetadata`. That struct is a single JSON document under the runtime keyspace `snapshot` key, rewritten by `save_runtime_metadata` and `mark_interrupted_write` independently of payload deletes [VERIFIED: packages/open-bitcoin-node/src/storage.rs:145, packages/open-bitcoin-node/src/storage/fjall_store.rs:366]. A field there can flip without the delete batch, and a later metadata rewrite can drop it.

Knots stores the analogous flag with `WriteFlag("prunedblockfiles")` on the block-tree DB before unlink [VERIFIED: packages/bitcoin-knots/src/validation.cpp:3107]. Open Bitcoin's D-05 is stricter: the marker is inserted inside the delete batch, so it becomes durable at the same commit as the tombstones and cannot be set when that commit fails. A second commit after the deletes is the worse crash window (keys gone, flag still false).

### Write-batch shape

For each height whose pre-check saw at least one mate:

1. Sync a one-item batch that inserts `prune_intent` (little-endian `u32` height plus 32-byte hash) into the block-index keyspace. This commit does not set `have_pruned`.
2. Sync a second batch that tombstones the block key and the undo key, inserts `have_pruned` only if the pre-check saw a live mate, and tombstones `prune_intent`.

Tombstoning an already-absent mate is idempotent and does not count as "removed a pair" by itself. If both keys were already absent, skip the height: no intent, no marker change (D-03, D-06, D-07). Do not call `remove_bytes` twice. That helper removes one key and persists [VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store.rs:469], so a crash between the two calls is the one-mate seam the happy path must not create.

The batch must not insert or remove coins best-block or head-blocks keys (D-12). `FjallCoinsView::batch_write_capped` stays the only writer of those markers [VERIFIED: packages/open-bitcoin-node/src/storage/coins_view.rs:225].

### Interrupted-prune intent

`prune_intent` is the resume record for exactly one height. Restart behavior in `FlushLifecycle::initialize`, before `ReadyToFlush`:

- Intent absent: open as today.
- Intent present, hash still on `chain_meta` active chain, height outside the keep window, and not lock-forbidden with the locks the caller has: finish that height's paired delete (including a missing mate) and clear the intent. Do not invent bytes.
- Intent present but the hash is not that height on the active chain, the height is inside the keep window, or a supplied lock forbids it: return `StorageError::Corruption` with `StorageRecoveryAction::Repair`. Do not use `InterruptedWrite` or `StorageRecoveryAction::Reindex`. Replay already maps missing block or undo to `InterruptedWrite`, which suggests reindex [VERIFIED: packages/open-bitcoin-node/src/chainstate/replay.rs:283, packages/open-bitcoin-node/src/storage.rs:103]. The prune path must not reuse that error.
- Refuse means `initialize` fails, so the node never reaches `ReadyToFlush` and does not serve the missing body (D-10). Do not report the delete as successful while a key remains.

Locks are not durable yet (`PruneLockInfo` says persistence is out of scope) [VERIFIED: packages/open-bitcoin-chainstate/src/prune/locks.rs:11]. Startup resume in this phase has whatever lock slice the caller passes. `initialize` today has no lock argument [VERIFIED: packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs:112], so resume passes an empty lock list until Phase 150 stores locks. The in-process unlink path still re-checks the locks the caller supplies.

### Module placement

| Piece | Location | Why |
|-------|----------|-----|
| Batch, intent, `have_pruned` load/store | New `packages/open-bitcoin-node/src/storage/fjall_store/prune.rs` | `fjall_store.rs` is already 625 lines and must stay a parent module. Child modules can see private `block_key`. |
| `has_undo` plus `pub(super) fn undo_key` | `fjall_store/coins.rs` | `undo_key` is private to `coins.rs` today, so a sibling cannot call it [VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store/coins.rs:404]. |
| Filter of plan heights | Node flush glue calling existing chainstate helpers | Do not reimplement 288 or the 10-block buffer. |
| `Chainstate::forget_undo` | `open-bitcoin-chainstate` | Pure map removal so the next flush cannot rewrite undo. |
| Call after prefix, before coins | `FlushLifecycle::execute_flush` | Single flush owner. |
| Cache eviction | `flush_coins` mutate closure | That closure already owns `blocks_by_hash`. |

Do not add a production crate. `fjall` 3.1.4 is already the node dependency [VERIFIED: packages/open-bitcoin-node/Cargo.toml:12].

## Project Constraints

`.cursor/rules/` is absent [VERIFIED: glob]. Constraints that bind this phase:

- Functional core stays I/O-free. Fjall effects stay in `open-bitcoin-node` [VERIFIED: AGENTS.md repo-local architecture notes].
- New Rust source and tests need `docs/parity/source-breadcrumbs.json` entries [VERIFIED: AGENTS.md].
- Verification gate is `bash scripts/verify.sh` [VERIFIED: AGENTS.md].
- No `unwrap()` in new Rust; prefer `let...else`; prefix new `Option` bindings with `maybe_`; Arrange/Act/Assert in unit tests [VERIFIED: standards/languages/rust.md, standards/core/testing.md].
- New modules use `foo.rs` plus `foo/`, which `fjall_store.rs` and `flush_lifecycle.rs` already do [VERIFIED: standards/languages/rust.md].
- Do not rewrite illegal combinations as comments. A height outcome should be an enum (deleted a live mate, already absent, skipped lock, skipped keep window, unresolved) so "pruned" cannot mean "skipped" [VERIFIED: standards/core/architecture.md].

## Standard Stack

### Core

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| `fjall` | 3.1.4 | Atomic cross-keyspace tombstones and the have-pruned marker | Already the node store. `WriteBatch` is documented to write atomically across keyspaces [VERIFIED: packages/open-bitcoin-node/Cargo.toml:12, ~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fjall-3.1.4/src/batch/mod.rs:12]. |
| Rust | 1.94.1 | Existing workspace toolchain | Pinned by `rust-toolchain.toml` [VERIFIED: rust-toolchain.toml]. |

### Supporting

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `open-bitcoin-chainstate` prune helpers | in-tree | `PrunePlan`, `height_forbidden_by_any_lock`, `height_inside_keep_window` | Consume. Do not replan 550 MiB or the keep window inside the node. |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Dedicated `have_pruned` key in the block-index keyspace | A bool field on `RuntimeMetadata` | Rejected. Metadata rewrites are independent of the delete batch. |
| Same-batch marker | A follow-up commit after deletes | Rejected. Crash between them leaves keys gone and the flag false. |
| `prune_intent` key | Reusing `RecoveryMarker` | Rejected. Interrupted markers are an operator recovery surface and `mark_interrupted_write` also rewrites runtime metadata. The prune intent must not suggest reindex. |
| One batch per height | One batch for the whole plan | A whole-plan batch is also atomic, but D-08 asks to finish the same interrupted height. One height per Sync pair matches that resume unit. Prune is infrequent; the extra fsyncs are the crash-seam boundary. |

**Installation:** none. Do not add a crate.

## Architecture Patterns

### Recommended Project Structure

```text
packages/open-bitcoin-chainstate/src/engine.rs
  # add forget_undo; no Fjall

packages/open-bitcoin-node/src/storage/fjall_store.rs
  # mod prune; do not grow the key helpers here

packages/open-bitcoin-node/src/storage/fjall_store/prune.rs
  # intent, paired delete batch, have_pruned load, resume

packages/open-bitcoin-node/src/storage/fjall_store/coins.rs
  # pub(super) undo_key and has_undo

packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs
  # execute_flush order and initialize resume

packages/open-bitcoin-node/src/chainstate.rs
  # flush_with_mode threads the plan, then forget_undo

packages/open-bitcoin-node/src/network/runtime_authority.rs
  # flush_coins evicts deleted hashes from blocks_by_hash
```

### Pattern 1: Height to the two keys

**What:** `PrunePlan.heights` are `u32` values, not hashes [VERIFIED: packages/open-bitcoin-chainstate/src/prune/plan.rs:18]. The payload key is `block:` plus 64 lowercase hex chars of the block hash, in the block-index keyspace [VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store.rs:146, packages/open-bitcoin-node/src/storage/fjall_store.rs:561, packages/open-bitcoin-node/src/storage/fjall_store/blocks.rs:13]. The undo key is `undo:` plus the same hex, in the chainstate keyspace [VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store/coins.rs:32, packages/open-bitcoin-node/src/storage/fjall_store/coins.rs:404].

**When to use:** For every height in the plan, find `ChainPosition { height, block_hash }` on the active chain the flush already holds [VERIFIED: packages/open-bitcoin-chainstate/src/types.rs:44]. Header entries also carry height and hash [VERIFIED: packages/open-bitcoin-network/src/header_store.rs:19], but they include side chains. Delete the active-chain hash only. If the height is missing from the active chain, skip it. Do not scan keyspaces for a height prefix. Neither key is height-prefixed.

`HeaderEntry` has no have-data flag, and the block index is one snapshot blob plus per-hash payload keys [VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store.rs:112]. Do not rewrite that snapshot to "clear presence." The presence bit is the payload key. Leave header rows in place. Knots keeps `CBlockIndex` and only clears `BLOCK_HAVE_DATA` and `BLOCK_HAVE_UNDO` [VERIFIED: packages/bitcoin-knots/src/node/blockstorage.cpp:283].

### Pattern 2: Flush insertion point

**What:** Today's order is prefix (blocks, then undo, then header entries), then coins flush or sync, then `chain_meta` [VERIFIED: packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs:218, packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs:396].

**When to use:** Insert the unlink loop after `persist_ordered_prefix` returns and before `cache.flush()` / `cache.sync()`. Then keep `persist_chain_meta` after coins. Knots flushes block and undo files, writes the block index, unlinks, then flushes coins [VERIFIED: packages/bitcoin-knots/src/validation.cpp:3138]. Open Bitcoin inserts have-pruned with the tombstones, which is the D-05 tightening of Knots setting `m_have_pruned` before `UnlinkPrunedFiles` [VERIFIED: packages/bitcoin-knots/src/validation.cpp:3107, packages/bitcoin-knots/src/node/blockstorage.cpp:896].

`execute_flush` returns early on `FlushDecision::None` and `RefuseDiskSpace` before the prefix [VERIFIED: packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs:248]. A non-empty prune plan must not be dropped on that path. When the plan is non-empty, run the prefix plus unlink even if coins policy says not to flush, and still do not write coins. An empty plan leaves this early return unchanged (D-06).

Thread `plan: &PrunePlan` and `locks: &[PruneLockInfo]` into `execute_flush` and `flush_with_mode`. Existing callers, including `flush_coins` and `persist`, pass `PrunePlan::default()` and `&[]` [VERIFIED: packages/open-bitcoin-node/src/chainstate.rs:391, packages/open-bitcoin-node/src/network/runtime_authority.rs:525]. `PruneMode` is not referenced outside the chainstate crate [VERIFIED: grep of `packages/**/*.rs`]. Do not add a byte scanner or call `plan_automatic_prune` from the sync loop. Phase 150 is the operator invocation. This phase's proof is calling the flush owner with a plan.

Also skip a planned height when `height_inside_keep_window(tip, height)` is true, with `tip` taken from the last active-chain position [VERIFIED: packages/open-bitcoin-chainstate/src/prune/range.rs:8]. D-13 requires the lock re-check. The keep-window re-check stops a stale plan from widening past 288 (D-12, pitfall 4). Skipping does not fail the other heights.

### Pattern 3: Drop memory that can resurrect bytes

**What:** `flush_window` clones every `undo_by_block` entry and the next prefix calls `save_undo` [VERIFIED: packages/open-bitcoin-node/src/chainstate.rs:380, packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs:91]. After a durable delete, `Chainstate::forget_undo` must remove that hash from the map before the function returns. Otherwise the next flush writes the undo key back.

**When to use:** `blocks_by_hash` is inserted on block connect and inventory treats cache presence as payload presence [VERIFIED: packages/open-bitcoin-node/src/network/lifecycle_projection/authority.rs:565, packages/open-bitcoin-node/src/network/inventory.rs:285]. `flush_coins` holds the network inside `mutate`. After `flush_with_mode`, remove each deleted hash from `blocks_by_hash` before the closure returns. Do not change `BlockServingDataAvailability` in this phase (D-15). Do not call `save_chainstate_snapshot` from the prune path (D-14).

### Pattern 4: Resume injected seams without a process kill

**What:** Fjall's journal reader drops a trailing batch that has no terminator, specifically "to keep atomicity" [VERIFIED: ~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fjall-3.1.4/src/journal/batch_reader.rs:66]. A crashed Sync batch therefore does not leave one mate. UNLK-03 still has to complete injected partials. Tests should build those with `write_raw_for_test` / a single `remove_bytes`, then reopen and call resume.

**When to use:** Resume outcomes:

| Injected state | Result |
|----------------|--------|
| `prune_intent` set, both keys present | Finish that height. Flag set only inside the successful delete batch. |
| One mate key absent, intent or flag set | Delete the remaining mate. Do not write an empty body. |
| `have_pruned` set and a key remains, hash still active | Finish the remaining mate. Success only when both keys are absent afterward. |
| Hash not on the active chain, or height locked / inside the keep window | `Corruption` + `Repair`. Keys that remain stay. No reindex. Node does not become ready. |
| Coins best-block key changed by the unlink batch | Must not be produced. Assert `best_block()` is unchanged. If a test injects an undo delete that also moved best-block, resume refuses that state instead of treating it as a successful prune. |

### Anti-Patterns to Avoid

- **Two `remove_bytes` calls:** Not one prune unit.
- **Setting have-pruned from `PruneMode` or from an empty plan:** Pitfall 3.
- **Inferring have-pruned because `has_block` is false:** Pitfall 2. Phase 149 owns labels; this phase only stores the marker.
- **`StorageRecoveryAction::Reindex` on the prune path:** FUT-23.
- **Deleting by scanning `block:` or `undo:` prefixes:** D-04. Those prefixes are hash suffixes, not heights.
- **Rewriting the header snapshot or the leftover chainstate snapshot as part of prune.**
- **Putting coins best-block in the unlink batch.**
- **A second flusher beside `FlushLifecycle`.**

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Cross-keyspace atomic delete | Two keyspace `remove` calls, or a custom journal | `self.db.batch()` / `fjall::WriteBatch` with `remove` on each keyspace, committed with `PersistMode::Sync` | Fjall already groups them as one journal batch and discards an incomplete trailer [VERIFIED: fjall-3.1.4 batch/mod.rs and journal/batch_reader.rs]. `commit_batch` already applies durability then `commit` [VERIFIED: packages/open-bitcoin-node/src/storage/coins_view.rs:382]. |
| Durability | `PersistMode::Flush` (`Buffer`) or `Buffered` (no persist) | `PersistMode::Sync`, which is `FjallPersistMode::SyncAll` (`fsync`) [VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store.rs:553, fjall journal/writer.rs `PersistMode::SyncAll`] | `Buffer` does not survive power loss. Buffered commit returns before journal persist. |
| Keep window and lock buffer | A second 288/10 implementation in the node | `height_inside_keep_window`, `height_forbidden_by_any_lock` | Phase 147 already encoded Knots `MIN_BLOCKS_TO_KEEP` (288) and `PRUNE_LOCK_BUFFER` (10) [VERIFIED: packages/open-bitcoin-chainstate/src/prune/range.rs:8, packages/open-bitcoin-chainstate/src/prune/locks.rs:9, packages/bitcoin-knots/src/validation.h:71, packages/bitcoin-knots/src/node/blockstorage.h:268]. |
| Candidate selection | Re-reading byte targets inside unlink | `PrunePlan.heights` | Unlink consumes heights. It does not decide the 550 MiB target. |
| Crash atomicity | A test that expects one Fjall Sync batch to tear a single key | Inject the torn key with the existing test raw writer, then resume | A real single-batch crash is discarded whole by journal recovery. |

**Key insight:** `WriteBatch::commit` on an empty batch returns `Ok` without writing [VERIFIED: fjall-3.1.4/src/batch/mod.rs:103]. Never use an empty batch to "set" have-pruned. A failed `persist` poisons the database and returns before memtable apply [VERIFIED: fjall-3.1.4/src/batch/mod.rs:119]. Treat that `Err` as have-pruned unchanged in this process and do not retry the batch after poison.

## Common Pitfalls

### Pitfall 1: Next flush rewrites undo

**What goes wrong:** Unlink deletes `undo:<hash>`, then the next `persist_ordered_prefix` saves `undo_by_block` and the key returns.
**Why it happens:** `flush_window` persists the whole in-memory undo map [VERIFIED: packages/open-bitcoin-node/src/chainstate.rs:380].
**How to avoid:** `forget_undo` after the Sync commit, before `execute_flush` returns.
**Warning signs:** `load_undo` is `Some` again after a second flush with no connect.

### Pitfall 2: Cache presence after a durable delete

**What goes wrong:** `has_block` is false and inventory still reports available.
**Why it happens:** `payload_present = cache_present || durable_payload_present` [VERIFIED: packages/open-bitcoin-node/src/network/inventory.rs:285].
**How to avoid:** `flush_coins` removes the deleted hashes from `blocks_by_hash` before returning.
**Warning signs:** A serve test still finds the hash in `blocks_by_hash` after unlink. Do not "fix" this by assigning `Pruned`.

### Pitfall 3: Have-pruned from config, or flag before the keys are gone

**What goes wrong:** Restart claims pruned history that is still stored, or a crash sets the flag while a mate remains.
**Why it happens:** Knots sets `m_have_pruned` when the candidate set is non-empty, before unlink [VERIFIED: packages/bitcoin-knots/src/validation.cpp:3107].
**How to avoid:** Marker insert only in the Sync batch that tombstones a height whose pre-check found a live key. Empty plan and commit `Err` leave the key absent.
**Warning signs:** Tests set the marker without a delete. `load` returns true while `has_block` is still true and no resume is running.

### Pitfall 4: Deleting the keep window or a locked height

**What goes wrong:** Disconnect or rescan needs undo the node just removed. Replay's `require_block` / `require_undo` then fail closed and suggest reindex [VERIFIED: packages/open-bitcoin-node/src/chainstate/replay.rs:283].
**Why it happens:** Unlink trusts a stale `PrunePlan` and never calls the Phase 147 predicates again.
**How to avoid:** Re-check `height_forbidden_by_any_lock` and `height_inside_keep_window`. Skip, do not fail the rest of the plan. Do not widen the 288 set.
**Warning signs:** A plan containing the tip height removes that height's undo.

### Pitfall 5: Reporting success on a partial key

**What goes wrong:** One key is gone, the API returns success, and a later reader treats the height as fully stored or fully pruned.
**Why it happens:** Success is defined as "batch commit returned Ok" without a post-check, or a tombstone of an already-absent key is counted as the delete.
**How to avoid:** Pre-check both mates. Success for a height means both `contains_key` probes are false afterward. If either probe is still true, the outcome is an error, not a pruned height (D-07, D-10).
**Warning signs:** `has_block` is false while `load_undo` is `Some` after a "successful" prune.

### Pitfall 6: Intent resume suggests reindex

**What goes wrong:** Open reports `StorageRecoveryAction::Reindex` and an operator, or a later phase, starts the destructive repair FUT-23 forbids.
**Why it happens:** The existing missing-payload path uses `InterruptedWrite` [VERIFIED: packages/open-bitcoin-node/src/chainstate/replay.rs:283].
**How to avoid:** Prune refuse uses `Corruption` and `Repair`. Detail text says the node stopped closed and did not reindex. Do not call `mark_interrupted_write`.
**Warning signs:** Prune tests assert `recovery_action() == Some(Reindex)`.

## Code Examples

Verified signatures from this repo. The planner should extend these, not invent parallel stores.

### Block and undo keys

```rust
// packages/open-bitcoin-node/src/storage/fjall_store.rs:146
pub fn save_block(&self, block: &Block, mode: PersistMode) -> Result<BlockHash, StorageError>

// packages/open-bitcoin-node/src/storage/fjall_store.rs:161
pub fn load_block(&self, block_hash: BlockHash) -> Result<Option<Block>, StorageError>

// packages/open-bitcoin-node/src/storage/fjall_store.rs:561
fn block_key(block_hash: BlockHash) -> String // "block:" + 64 hex

// packages/open-bitcoin-node/src/storage/fjall_store/blocks.rs:13
pub fn has_block(&self, block_hash: BlockHash) -> Result<bool, StorageError>

// packages/open-bitcoin-node/src/storage/fjall_store/coins.rs:50
pub fn save_undo(&self, block_hash: BlockHash, undo: &BlockUndo, mode: PersistMode) -> Result<(), StorageError>

// packages/open-bitcoin-node/src/storage/fjall_store/coins.rs:65
pub fn load_undo(&self, block_hash: BlockHash) -> Result<Option<BlockUndo>, StorageError>

// packages/open-bitcoin-node/src/storage/fjall_store/coins.rs:404
fn undo_key(block_hash: BlockHash) -> String // "undo:" + 64 hex
```

`has_block` is a `contains_key` probe and does not decode the body [VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store/blocks.rs:9]. Add `has_undo` the same way. Do not use `load_undo` as the presence check; a corrupt undo value would turn a delete into a decode error.

### Existing multi-keyspace batch and single-key remove

```rust
// packages/open-bitcoin-node/src/storage/fjall_store.rs:120
let mut batch = self.db.batch();
batch.insert(&self.headers, SNAPSHOT_KEY, header_bytes);
batch.insert(&self.block_index, SNAPSHOT_KEY, block_index_bytes);
batch.commit()

// packages/open-bitcoin-node/src/storage/coins_view.rs:225
let mut batch = self.db.batch();
batch.remove(&self.coins, best_key.as_slice());
batch.insert(&self.coins, heads_key.as_slice(), heads_value.as_slice());
// ...
commit_batch(batch, final_mode)

// packages/open-bitcoin-node/src/storage/coins_view.rs:382
fn commit_batch(batch: fjall::OwnedWriteBatch, mode: PersistMode) -> Result<(), StorageError>
```

The prune batch uses the same `db.batch()` owner, with `remove` on `&self.block_index` and `&self.chainstate`, then `PersistMode::Sync`. It does not call `commit_batch` if that function stays private inside `coins_view.rs`. Duplicate the small durability match locally or make `commit_batch` crate-visible. Do not route prune through `batch_write_capped`.

Fjall operations the batch must use [VERIFIED: fjall-3.1.4/src/batch/mod.rs]:

```rust
pub fn insert<K: Into<UserKey>, V: Into<UserValue>>(&mut self, p: &Keyspace, key: K, value: V)
pub fn remove<K: Into<UserKey>>(&mut self, p: &Keyspace, key: K)
pub fn durability(mut self, mode: Option<PersistMode>) -> Self
pub fn commit(mut self) -> crate::Result<()>
```

### Flush owner

```rust
// packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs:76
pub trait FlushPersistSink {
    fn persist_block(&mut self, block: &Block) -> Result<(), StorageError>;
    fn persist_undo(&mut self, hash: BlockHash, undo: &BlockUndo) -> Result<(), StorageError>;
    fn persist_header_entries(&mut self, entries: &[HeaderEntry]) -> Result<(), StorageError>;
    fn persist_chain_meta(&mut self, active_chain: &[ChainPosition]) -> Result<(), StorageError>;
}

// packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs:396
fn persist_ordered_prefix<S: FlushPersistSink>(
    sink: &mut S,
    block_payloads: &[Block],
    undo_window: &[(BlockHash, BlockUndo)],
    header_entries: &[HeaderEntry],
) -> Result<(), StorageError>
```

Add the unlink method on this trait so the order stays inside `execute_flush`. `FjallNodeStore` performs the real Sync batches. Test sinks record the call so order tests can assert prefix-then-unlink-then-coins without a datadir chmod [the trait exists for that reason: packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs:75].

### Plan and lock predicates

```rust
// packages/open-bitcoin-chainstate/src/prune/plan.rs:18
pub struct PrunePlan { pub heights: Vec<u32> }

// packages/open-bitcoin-chainstate/src/prune/plan.rs:40
pub fn plan_automatic_prune(input: &AutomaticPruneInput) -> PrunePlan

// packages/open-bitcoin-chainstate/src/prune/plan.rs:119
pub fn plan_manual_prune(input: &ManualPruneInput) -> Result<PrunePlan, ManualPruneRefusal>

// packages/open-bitcoin-chainstate/src/prune/locks.rs:39
pub fn height_forbidden_by_any_lock(height: u32, locks: &[PruneLockInfo]) -> bool

// packages/open-bitcoin-chainstate/src/prune/range.rs:19
pub fn height_inside_keep_window(tip: u32, height: u32) -> bool
```

### Active-chain hash

```rust
// packages/open-bitcoin-chainstate/src/types.rs:44
pub struct ChainPosition {
    pub block_hash: BlockHash,
    pub header: BlockHeader,
    pub height: u32,
    pub chain_work: u128,
    pub median_time_past: i64,
}
```

Lookup is `active_chain.iter().find(|position| position.height == height)`. Do not use a header at that height just because it exists.

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Knots sets `m_have_pruned` when the candidate set is non-empty, writes the block index, then unlinks blk/rev files [VERIFIED: validation.cpp:3107, blockstorage.cpp:896] | Open Bitcoin records `have_pruned` inside the Sync tombstone batch | Phase 148 decision D-05 | Stricter than Knots. Document the difference in code comments. The parity-doc pass is Phase 151. |
| Knots `UnlinkPrunedFiles` deletes blk and rev files as a pair [VERIFIED: blockstorage.cpp:896] | Tombstone `block:<hash>` and `undo:<hash>` in one Fjall batch | v2.4 storage choice, REQUIREMENTS.md out-of-scope table | No flat-file block store. |
| Separate BLOCK_HAVE_DATA flag | Presence is the payload key | v2.3 Fjall layout | "Clear presence" is the block-key tombstone. |

**Deprecated/outdated:**

- `remove_bytes` for a paired prune. It persists one key at a time.
- Inferring prune from a missing payload. `inventory.rs` must stay on Available versus Unavailable until Phase 149.

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | Startup `initialize` can resume `prune_intent` with an empty lock list because prune locks are not durable until Phase 150. | Discretion Recommendations | A crash between intent and delete could finish a height that an in-memory lock would have skipped. In-process unlink still re-checks caller-supplied locks. If Phase 150 lands locks first, pass them into `initialize`. |
| A2 | One Sync batch per height is the right resume grain, rather than one batch for every height in the plan. | Discretion Recommendations | A whole-plan batch would also be atomic. Per-height batches mean a crash leaves a finished prefix plus one intent. Wrong only if tests are written to expect a single fsync for the entire plan. |

## Open Questions

1. **Should `flush_coins` build an automatic plan during this phase?**
   - What we know: `PruneMode` is not wired into the node. `flush_coins` is the production flush entry and it only calls `flush_with_mode` [VERIFIED: packages/open-bitcoin-node/src/network/runtime_authority.rs:525]. Operator manual prune is Phase 150.
   - What's unclear: Nothing. A daemon byte scanner is extra scope.
   - Recommendation: Existing callers pass an empty plan. Tests and a crate-visible flush argument apply a non-empty `PrunePlan`. Phase 150 calls that argument.

2. **Where do injected crash seams get their raw keys?**
   - What we know: `write_raw_for_test` and `remove_bytes` exist and can seed one mate, an intent, or a marker without the paired batch.
   - What's unclear: Nothing that blocks planning.
   - Recommendation: Reopen the datadir and run `initialize` / resume. Do not depend on killing a process inside `WriteBatch::commit`.

## Environment Availability

No new external tool. Fjall is already a workspace dependency.

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| Rust toolchain | Compile and tests | ✓ | 1.94.1 | — |
| `fjall` | Delete batch | ✓ | 3.1.4 | — |
| Bitcoin Knots submodule sources | Parity citations | ✓ | `29.3.knots20260210` anchors present under `packages/bitcoin-knots/src` | — |

**Missing dependencies with no fallback:** none.

**Missing dependencies with fallback:** none.

## Security Domain

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V2 Authentication | no | Headless storage mutation. No new auth surface. |
| V3 Session Management | no | No session. |
| V4 Access Control | yes | Prune locks plus the 288 keep window are the authorization boundary for deletion. Re-check them at unlink time. Startup refuse stays closed instead of serving a half-deleted height. |
| V5 Input Validation | yes | Heights come from `PrunePlan`, then must resolve to an active-chain `ChainPosition`. Unresolved, locked, and keep-window heights are skips, not deletes. Do not treat a raw key prefix as a height. |
| V6 Cryptography | no | Do not hash, invent, or re-encode block or undo bytes. Tombstones only. |

### Known Threat Patterns for this phase

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Prune deletes tip undo and disconnect or replay cannot proceed | Denial of service | Keep-window re-check; do not widen the last 288 blocks; coins best-block stays out of the delete batch. |
| Protected rescan or lock range loses payloads | Tampering | `height_forbidden_by_any_lock` at unlink, not only inside the planner. |
| have-pruned set while bytes remain, so a later phase advertises limited service while full history is still stored | Repudiation | Marker only inside the non-empty Sync delete batch. |
| Missing payload labeled or served as still fully stored | Spoofing | Evict `blocks_by_hash`. Refuse-closed startup does not become `ReadyToFlush`. Do not synthesize bodies. |
| Interrupted prune mapped to automatic reindex | Elevation of privilege / tampering | `Repair` corruption error, never `Reindex`. |
| Snapshot or cache rewrite restores deleted history | Tampering | `forget_undo`; no `save_chainstate_snapshot` on the prune path. |

## Sources

### Primary (HIGH confidence)

- `packages/open-bitcoin-node/src/storage/fjall_store.rs` — `block_key`, `load_block`, `save_block`, multi-keyspace `db.batch()`, `remove_bytes`, runtime metadata key
- `packages/open-bitcoin-node/src/storage/fjall_store/blocks.rs` — `has_block`
- `packages/open-bitcoin-node/src/storage/fjall_store/coins.rs` — `undo_key`, `load_undo`, `save_undo`
- `packages/open-bitcoin-node/src/storage/coins_view.rs` — `OwnedWriteBatch` remove/insert and `commit_batch`
- `packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs` — `execute_flush`, `persist_ordered_prefix`, `initialize`
- `packages/open-bitcoin-chainstate/src/prune/plan.rs`, `locks.rs`, `range.rs` — `PrunePlan` and the predicates unlink must call
- `packages/open-bitcoin-node/src/network/inventory.rs` and `lifecycle_projection/authority.rs` — cache resurrection
- `packages/bitcoin-knots/src/validation.cpp` lines 3070–3181 — flush and have-pruned order
- `packages/bitcoin-knots/src/node/blockstorage.cpp` — `PruneOneBlockFile` (283), `DoPruneLocksForbidPruning` (317), `FindFilesToPrune` (387), `UnlinkPrunedFiles` (896), `IsBlockPruned` (711)
- `packages/bitcoin-knots/src/node/blockstorage.h` — `PruneLockInfo`, `m_have_pruned` (419), `MIN` buffer comment (268)
- `packages/bitcoin-knots/src/validation.h` line 71 — `MIN_BLOCKS_TO_KEEP = 288`
- Fjall 3.1.4 sources in the local cargo registry: `src/lib.rs` (cross-keyspace atomic semantics), `src/batch/mod.rs` (`WriteBatch::commit`), `src/journal/batch_reader.rs` (discard incomplete batch), `src/journal/writer.rs` (`PersistMode::SyncAll`)

### Secondary (MEDIUM confidence)

- `.planning/research/PITFALLS.md` pitfalls 2, 3, 7, 8, and 9 — checked against the code locations named above. Pitfall 7's "clear have-data then unlink" maps to one tombstone here because there is no separate presence flag. Pitfall 3's "persist the flag with the same durability as the delete" is satisfied by putting the marker in the Sync batch, which is stricter than Knots.

### Tertiary (LOW confidence)

- None.

## Metadata

**Confidence breakdown:**

- Standard stack: HIGH — `fjall` 3.1.4 is in `Cargo.toml` and the local crate docs were read.
- Architecture: HIGH — flush order, key helpers, active-chain positions, and the cache OR were read at the cited lines.
- Pitfalls: HIGH — the undo rewrite path and the inventory cache OR are in this repo, not general lore.

**Research date:** 2026-09-26
**Valid until:** 2026-10-26 (stable in-repo seams; re-check if `execute_flush` or the Fjall key helpers move)

## Planning Notes the executor will need

- Add new files to `docs/parity/source-breadcrumbs.json`. Defensible anchors are `packages/bitcoin-knots/src/validation.cpp`, `packages/bitcoin-knots/src/node/blockstorage.cpp`, `packages/bitcoin-knots/src/node/blockstorage.h`, and `packages/bitcoin-knots/src/validation.h`. `fjall_store/blocks.rs` is already in the `node-stored-block-presence` group; `flush_lifecycle.rs` is already in `node-chainstate-adapter` [VERIFIED: docs/parity/source-breadcrumbs.json:1323, docs/parity/source-breadcrumbs.json:1857]. A new `prune.rs` is not covered until it is listed.
- File comment breadcrumbs should name the Knots difference: have-pruned is recorded in the delete batch, which is stricter than setting the flag before unlink.
- Tests are normal crate tests with temp datadirs, one concern each, with Arrange/Act/Assert. Suggested cases: both keys gone after reopen; one mate already absent; empty plan leaves the marker absent; lock-buffered height remains; keep-window height in a stale plan remains; marker absent when the commit does not run; reopen with only `prune_intent` finishes that hash; one missing mate finishes without a synthesized body; marker set while a key remains does not report success until both are gone; hash mismatch refuses with `Repair` and `initialize` is not ready; coins `best_block` unchanged; second flush does not restore undo; `blocks_by_hash` loses the hash. Do not assert `BlockServingDataAvailability::Pruned`.
- `bash scripts/verify.sh` is the completion gate. Ad-hoc Cargo during implementation goes through `bun run scripts/command-timings.ts run --key <stable-key> -- <command>` and must not overlap another Cargo job on the same target directory [VERIFIED: AGENTS.md].
