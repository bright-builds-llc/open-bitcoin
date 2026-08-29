# Pitfalls Research

**Domain:** Adding chainstate durability to Open Bitcoin
**Researched:** 2026-08-29
**Confidence:** HIGH for current Open Bitcoin snapshot/serving seams and pinned Knots flush/consistency behavior; MEDIUM for the exact Fjall coins-batch crash protocol until requirements name the allowed loss window

## Critical Pitfalls

### Pitfall 1: Flushing Coins Out of Order With Block Files, Index, Undo, and Tip

**What goes wrong:**
A crash or failed write leaves coins, block payloads, undo, and the block index pointing at different tips. Restart then reconnects a tip whose undo is missing, treats a coins best-block as proven history, or credits progress that the payload store cannot rewind. Knots' `FlushStateToDisk` writes in a fixed order: flush block and undo files first, then the block-index database, then unlink prune candidates, then `Flush`/`Sync` the coins cache that may refer to those index entries. A failed coins write is a fatal error, not a retryable "we'll catch up later."

**Why it happens:**
Today Open Bitcoin does not have a coins cache. `ManagedChainstate::persist` dumps one in-memory `ChainstateSnapshot` after every connect, disconnect, and reorg. `DurableSyncRuntime::persist_progress` then writes headers, the full snapshot, and runtime metadata as **separate** Fjall puts. Adding a coins keyspace as a fourth store without a single flush protocol copies the current "write whatever is convenient" habit into a Knots-shaped surface.

**How to avoid:**
Define one shell-owned flush sequence before any incremental coins write lands. The durable coins best-block may advance only after the matching block payload and undo are durable and the block-index row for that height exists. Failed coins writes must fail closed and stop progress credit. Encode this as an explicit consistency gate: coins tip, index tip, undo presence, and payload presence are compared after every flush and after every restart.

**Warning signs:**

- A coins `BatchWrite` helper exists before block-file and index flush helpers share one manager entry point.
- Tests assert only that coins keys exist, not that undo and payload exist for the same hash.
- `persist_progress` grows a third independent `save_*` call instead of one ordered flush.
- Operator status reports a connected tip after a coins write that the index or undo store cannot name.

**Phase to address:**
Mid durability (flush policy and chainstate-manager), after the early coins-store contract exists. Re-verify on every restart fixture in the late closeout.

---

### Pitfall 2: Treating a Cache Hit as Durable Coin Truth

**What goes wrong:**
Connect, mempool spend checks, wallet rescan, RPC, and progress credit all read the in-memory view and call it persisted. A crash then reopens an older coins tip. The opposite failure is also likely: every cache insert is written through to disk, so the node never gains a cache and still behaves like today's full-snapshot dump.

**Why it happens:**
`Chainstate` is a complete `HashMap` of UTXOs. `ManagedChainstate` persists after every mutation. `DurableSyncRuntime::open` hydrates that map from `load_chainstate_snapshot`. Knots distinguishes `HaveCoin` (may fetch into cache) from `HaveCoinInCache`, and `Flush` (write and empty) from `Sync` (write dirty flags, keep the cache). Open Bitcoin has none of those distinctions yet, so "the map has the coin" currently **is** the durability story.

**How to avoid:**
Make cache occupancy, dirty/fresh flags, and durable best-block three different facts. Progress credit, restart/resume, and "durably persisted" operator fields may move only when the coins store reports a committed best-block, never when a cache lookup succeeded. Keep `persist()`-after-every-connect only as a test adapter or an explicit `FlushStateMode::ALWAYS` path. Encode a no-claim: cache hit is not a durability proof.

**Warning signs:**

- Status or tests use `chainstate.utxos().contains` or `HaveCoin` as restart evidence.
- `save_snapshot` remains the only coins write after a "disk-backed coins" phase ships.
- `Flush` and `Sync` are named but both empty the cache, or neither records a best-block.
- A comment says "already in memory, so it is durable."

**Phase to address:**
Early durability (coins store and cache contract, Phase 139+). Mid durability must consume the committed-best-block fact for flush and restart.

---

### Pitfall 3: Mis-applying DIRTY/FRESH so Spentness Never Reaches Disk

**What goes wrong:**
A reorg spends a coin in cache, then reconnects a block that recreates it, or spends a fresh coin and erases the cache entry. The parent store never learns the spend. Restart resurrects a spent outpoint, or a later flush throws the Knots `FRESH flag misapplied` class of invariant.

**Why it happens:**
Pinned `CCoinsViewCache::AddCoin` and `SpendCoin` are explicit: a spent DIRTY coin that is re-added during a reorg must not become FRESH, because a later spend would erase it and never flush spentness. FRESH+spent may be dropped from the child cache; FRESH+spent against a FRESH parent may delete the parent instead of writing a tombstone. Open Bitcoin's current engine applies full snapshots, so it never needed these flags.

**How to avoid:**
Port the dirty/fresh transitions as typed cache-entry state in the **pure** coins-cache model, with Knots comments turned into unit tests: reorg re-add of a dirty spent coin, fresh spend erase, child-fresh against live parent, and parent-fresh plus child-spent delete. Do not invent a "just write every mutation" shortcut and then claim cache-flush parity.

**Warning signs:**

- Cache entries are `Option<Coin>` with no dirty/fresh flags.
- Reorg tests only check the final in-memory UTXO set, never a flushed parent view after crash.
- A flush implementation skips spent keys because "spent coins are not in the snapshot today."

**Phase to address:**
Early durability (cache contract). Mid durability must flush those flags through the disk adapter and replay them after an interrupted write.

---

### Pitfall 4: Incremental Coins Writes Without an Interrupted-Flush Protocol

**What goes wrong:**
A large coins flush writes some keys, crashes, and restarts with a silent mix of old and new coins under a single best-block. Validation then accepts or rejects the wrong spends. Knots writes `DB_HEAD_BLOCKS = [new, old]` first, streams batches, then replaces that marker with `DB_BEST_BLOCK`. `ReplayBlocks` rolls the old branch back and the new branch forward when two heads remain. Two heads of any other count is `unknown inconsistent state`.

**Why it happens:**
Today's snapshot codec writes one blob under one key. That is crash-atomic at the snapshot grain even when headers and chainstate can already diverge across `persist_progress` puts. Incremental coins keys remove that accident. Copying LevelDB batching without the two-head marker, or treating Fjall `PersistMode::Sync` as sufficient, looks complete until the first mid-flush crash.

**How to avoid:**
Require an explicit interrupted-flush record (best-block vs in-flight heads) before any multi-batch coins write. Recovery must either finish the transition from durable undo/block bodies or refuse to start and surface a typed storage-recovery blocker. Do not auto-repair. Tests must inject a crash after the first partial batch and after the final best-block write.

**Warning signs:**

- Coins keys are written in a loop of `put_bytes` with no heads marker.
- Restart tests only cover clean shutdown.
- Recovery copies Knots `-reindex-chainstate` language into operator docs as if it were implemented.

**Phase to address:**
Mid durability (flush policy and restart). Late durability must keep the crash-window claim honest.

---

### Pitfall 5: Dual Truth — Snapshot Blob Plus Coins DB, or Cache Plus Snapshot

**What goes wrong:**
The node keeps writing `ChainstateSnapshot` (active chain, full UTXO map, undo map) **and** a new coins keyspace. Restart loads the snapshot into `MemoryChainstateStore` (as `DurableSyncRuntime::open` does today) and ignores the coins DB, or the reverse. Wallet rescan and confirmation migration still call `load_chainstate_snapshot`. After one successful new-format write, an old process or a partial migration reopens the stale blob.

**Why it happens:**
Every current restart, rescan, and progress path is snapshot-shaped. The fastest "disk-backed coins" demo is to add a store beside the blob and leave `save_snapshot` / `from_snapshot` in place. That preserves compile-time callers and silently keeps snapshot-only coin truth.

**How to avoid:**
Pick one durable coin source of truth before the first production write. A transition phase may read legacy snapshots and write coins+index+undo, but reopen must not hydrate `ManagedChainstate` from a blob once coins exist. Wallet rescan, confirmation migration, `persist_progress`, and tests must move together. Encode a consistency gate: after the cutover generation, `load_chainstate_snapshot` is either gone, a derived projection, or a hard error.

**Warning signs:**

- `save_chainstate_snapshot` and `save_coins_batch` both appear on the connect path.
- `DurableSyncRuntime::open` still does `MemoryChainstateStore::save_snapshot(loaded_blob)`.
- Tests pass by loading the blob while the coins keyspace is empty.

**Phase to address:**
Early durability must name the source of truth. Mid durability owns the cutover and same-datadir reopen. Late durability forbids dual-write language in operator docs.

---

### Pitfall 6: Serving or Labeling a Block Whose Payload Is Not in the Store

**What goes wrong:**
Index presence, coins best-block, or the existing `durable_availability` flag is treated as "we can serve this block." A peer gets a body the node no longer has, or status says `available` / `pruned` in a way that implies prune-mode or archive history. The v2.1 path already refuses with `NotFound` when `lookup_block` is `None`, and already labels non-tip missing local data `Pruned` and missing tip data `Unavailable`. Those labels are **local payload facts**, not product modes.

**Why it happens:**
`gate_inventory_for_durable_serving` currently passes `durable_availability: true` so the gate may allow a later disk read. `managed_block_serve_input` then ORs `has_local_data || durable_availability`. If durability work starts setting that flag from coins or block-index rows, the gate will classify `Available` before the payload probe. Adding disk-backed coins does not add archive-scale history; the payload store is still the only serve truth.

**How to avoid:**
Serve or report stored only after `load_block` (or the equivalent payload probe) returns the body. Index-only or coins-only presence is `Unavailable` (tip) or the existing non-tip missing-payload label — never `Available`. Do not introduce prune-mode deletion or archive-node serving under this milestone. Encode no-claim gates for `archive-node`, `prune mode`, and production-scale historical serving.

**Warning signs:**

- `durable_availability` is set from coins tip or header index without `load_block`.
- Docs say "pruned node" because the `Pruned` status label already exists.
- A serve path constructs a block from undo + coins instead of the payload store.
- README or status implies historical serving because coins are now durable.

**Phase to address:**
Late durability (honest availability and claim guardrails). Early/mid work must not change the flag's meaning without a payload probe.

---

### Pitfall 7: Smuggling assumeutxo, assumevalid, Prune Mode, or LevelDB/rust-bitcoin Under "Durability"

**What goes wrong:**
The catalog still lists "disk-backed coins databases, cache-flush policy, and assumeutxo flows" as one known-gap sentence. `FlushStateToDisk` contains prune-file unlinks. `ChainstateManager` snapshot activation resizes caches, writes a snapshot base hash, and talks about LevelDB datadirs. A durability phase copies those hooks, adds `rusty-leveldb` "because Knots uses LevelDB," or pulls `rust-bitcoin` types for `Coin`. v1.7 already forbade assumeutxo/assumevalid/pruning shortcuts as **sync claims**. v2.3 is not a license to reverse that.

**Why it happens:**
The shortest path to "fuller chainstate-manager behavior" is to follow `validation.cpp` past the flush function into snapshot chainstates and prune. The project dependency policy and Fjall adapter already exist; LevelDB looks like parity rather than a new production database.

**How to avoid:**
Keep assumeutxo, assumevalid, IBD snapshot shortcuts, prune-mode, and archive-mode as explicit deferred no-claims in requirements, parity roots, and the late checker. Implement disk-backed coins on the existing Fjall shell. Keep first-party `Coin` / outpoint types. Do not add LevelDB or rust-bitcoin to the production path. Historical `.planning/phases/` stay tracked; do not delete them to "clean up" after a storage rewrite.

**Warning signs:**

- A phase plan cites `ActivateSnapshot` or `PopulateAndValidateSnapshot` as in-scope.
- `Cargo.toml` gains `bitcoin`, `rust-bitcoin`, or a LevelDB crate for node/chainstate.
- Prune-file unlink or `nManualPruneHeight` appears in the flush manager.
- A PR deletes `.planning/phases/70-*` or other verifier-referenced history because "snapshots are gone."

**Phase to address:**
State the no-claim in early durability. Enforce it in late durability claim guardrails. Mid flush work may read Knots prune comments as **out of scope**, not as a checklist.

---

### Pitfall 8: Putting Coins I/O or Flush Policy in the Pure Chainstate Engine

**What goes wrong:**
`open-bitcoin-chainstate` starts opening Fjall, reading clocks for `m_next_write`, or calling `save` inside `connect_block`. Tests then need a datadir. Architecture policy fails. Alternatively, the shell clones the entire `HashMap` on every persist (`snapshot()` today) while a cache exists, so IBD memory doubles and the authority lock is held across a multi-minute flush.

**Why it happens:**
Knots `CCoinsViewCache` and `CCoinsViewDB` sit in one inheritance tree. The current Rust engine already owns UTXOs and undo. The tempting port is to make `Chainstate` the database. `FlushStateToDisk` also does disk-space checks and randomized next-write times — easy to drop into the core crate.

**How to avoid:**
Keep connect/disconnect/reorg and dirty/fresh cache algebra I/O-free. The node shell owns Fjall coins batches, flush triggers, disk-space probes, and clocks. Capture an owned flush delta under the authority lock; persist after release; apply a short receipt. Do not hold `ManagedNetworkHandle` across a full UTXO flush. Encode the architecture gate the way other milestones did for mempool snapshots.

**Warning signs:**

- `open-bitcoin-chainstate` depends on `fjall`, `std::fs`, or `SystemTime`.
- `connect_block` returns only after a disk write.
- `snapshot()` still clones the full UTXO map on every block after coins are incremental.
- Flush benchmarks are missing before any IBD-scale claim.

**Phase to address:**
Early durability (cache is pure; store is a shell trait). Mid durability owns lock-release flush. Late durability keeps the architecture checker honest.

---

### Pitfall 9: Breaking Same-Datadir Restart, Rescan, and Progress Credit Mid-Cutover

**What goes wrong:**
Existing v1.4–v1.7 restart/resume tests, wallet rescan chunks, and confirmation migration assume a complete `ChainstateSnapshot` blob. A coins-only write leaves `active_chain` empty, drops undo, or resets confirmation counts. Progress credit (Phase 78 contract) still fires from in-memory connect because `persist_progress` used to dump the whole map. Operators reopen a datadir and reconnect genesis.

**Why it happens:**
`persist_progress` is the real durable write; `ManagedChainstate::persist` only updates the in-memory store. It is easy to replace the Fjall blob and forget that rescan, migration, and reopen all call the old loader. Periodic flush also means "connected" is no longer "persisted."

**How to avoid:**
Keep same-datadir reopen as a required fixture from the first coins write: headers, block payloads, undo, coins best-block, and active tip must round-trip. Wallet rescan must read the new source of truth or a maintained projection. Progress credit stays gated on durable persist, not cache apply. Legacy snapshot fixtures remain readable until an explicit migration generation.

**Warning signs:**

- `restart_chainstate` tests are rewritten to skip UTXO equality.
- `load_chainstate_snapshot_with_confirmation_migration` is deleted with no replacement.
- `progress_credit` increments in a test that never flushes coins.
- Undo lives only in the old snapshot DTO after coins flush.

**Phase to address:**
Mid durability (restart/resume and flush policy). Late durability re-runs the existing restart and rescan fixtures as no-regression gates.

---

### Pitfall 10: Letting "Chainstate Durability" Expand the Product Claim

**What goes wrong:**
README, status, RPC help, or release notes imply production full-node readiness, public/default serving, archive-node or production-scale historical serving, prune-mode product behavior, assumeutxo sync, or that Open Bitcoin now matches Knots' entire `ChainstateManager`. Past milestones repeatedly smuggled those claims. The previous package-relay pitfalls file was right: a short milestone name is broader than the shipped contract.

**Why it happens:**
"Fuller chainstate-manager behavior" and "disk-backed coins" sound like a complete node. Successful local flush tests look like IBD durability at archive scale. Existing no-claim checkers still talk about deferred coins/assumeutxo in one breath and will need **narrow replacement wording**, not deletion.

**How to avoid:**
Adopt a claim taxonomy: `disk_backed_coins`, `cache_flush_policy`, `chainstate_manager_flush_lifecycle`, and `honest_payload_availability` are in scope. `assumeutxo`, `assumevalid`, `prune_mode`, `archive_node`, public/default serving, public-network CI, production readiness, and production-funds wallet use stay deferred. Positive checkers require the bounded claim; negative fixtures fail on archive/prune/assumeutxo/production/public-default language. Keep historical phase directories tracked.

**Warning signs:**

- "assumeutxo deferred" or "archive-node deferred" guard text is deleted without a narrower contract.
- Docs say "full chainstate manager" with no flush/availability qualifier.
- A live mainnet flush or archive-serving test enters default `verify.sh`.
- Benchmarks of a few thousand coins are used to claim mainnet UTXO-set durability.

**Phase to address:**
Late durability (parity, availability, and release guardrails). Write the taxonomy in early requirements so mid implementation cannot drift.

## Technical Debt Patterns

| Shortcut | Immediate Benefit | Long-term Cost | When Acceptable |
| --- | --- | --- | --- |
| Keep writing the full `ChainstateSnapshot` blob after every connect | Restart tests keep passing | Write amplification, dual truth, no cache-flush claim | Only as a temporary dual-write with a kill date and one source of truth on reopen |
| Write-through every cache mutation | No dirty/fresh algebra | Not a cache; still snapshot-shaped; cannot claim flush policy | Never if the milestone claims cache-flush |
| Empty the cache on every flush (`Flush` only) | Simpler adapter | IBD thrashes; Knots `Sync` path missing | Test-only `ALWAYS` flushes |
| Port `FlushStateToDisk` including prune unlinks | Looks like Knots | Implements prune-mode under a durability label | Never in v2.3 |
| Use LevelDB because Knots `txdb.cpp` does | Familiar batch API | Violates Fjall/shell policy and dependency rules | Never in the production path |
| Hydrate `MemoryChainstateStore` from a blob forever | Minimal `open()` change | Coins DB is dead on restart | Never after cutover |
| Set `durable_availability: true` from index/coins | Gate allows later I/O | Available/pruned labels lie | Only if a payload probe still decides serve/status |
| Bump global `SchemaVersion` for coins keys | Easy incompatibility | Invalidates wallet/mempool/runtime snapshots | Only with a planned multi-namespace migration |
| Delete historical `.planning/phases/` after storage rewrite | Cleaner tree | Breaks verifier-referenced evidence | Never |
| Hold the authority lock across a full UTXO flush | One atomic in-memory view | Peer/RPC stall; Knots already warns multi-minute flushes | Never; capture delta, persist, receipt |

## Integration Gotchas

| Integration | Common Mistake | Correct Approach |
| --- | --- | --- |
| Pure `Chainstate` → coins cache | Replace the `HashMap` with Fjall inside the core crate | Keep connect/disconnect/reorg and dirty/fresh algebra pure; trait out reads/writes |
| Cache → Fjall coins | `put` each dirty coin independently | One batch protocol with in-flight heads, then committed best-block |
| Coins flush → block index | Flush coins first because "UTXOs are the point" | Knots order: block+undo files, then index, then coins |
| Coins flush → undo | Leave undo in the snapshot blob only | Undo must be durable for the coins tip before that tip is committed |
| `persist_progress` → new stores | Add `save_coins` beside header and snapshot puts | One manager flush; do not widen the existing non-atomic window without a protocol |
| `DurableSyncRuntime::open` → coins | Keep `load_chainstate_snapshot` into `MemoryChainstateStore` | Reopen from coins+index+undo; treat legacy blobs as migration input only |
| Wallet rescan → coins | Keep `required_chainstate_snapshot()` as truth | Point rescan at the durable coins/active-chain projection |
| Confirmation migration → coins | Rewrite the old blob in place | Migrate source records into the new stores; do not leave two writers |
| Progress credit → flush | Credit on `commit_prepared_connect` | Credit only after the flush receipt matches the connected tip |
| Block serving → coins/index | `durable_availability` from coins best-block | Payload `load_block` is the only serve/available proof |
| Mempool / package spend view → cache | Treat `HaveCoin` cache fill as a persist | Same spend view; durability unchanged until flush |
| Architecture checker → chainstate | Allow `std::fs` in `open-bitcoin-chainstate` for "the DB is the engine" | Fail the crate; I/O stays in `open-bitcoin-node` |
| Default verifier → flush | Sleep, live disk soak, or mainnet UTXO replay in `verify.sh` | Deterministic crash-injection, fake clocks, temp Fjall, fixture chains |

## Performance Traps

| Trap | Symptoms | Prevention | When It Breaks |
| --- | --- | --- | --- |
| Clone the full UTXO `HashMap` on every persist (`snapshot()` today) | Memory doubles; IBD stalls | Incremental dirty cursor; clone only in tests | Mainnet-scale UTXO count, not fixture chains |
| Write-through every connect | Disk amplification; no cache benefit | Periodic/`IF_NEEDED`/`ALWAYS` modes with dirty flags | Any IBD longer than toy fixtures |
| Hold authority lock during full flush | Peer timeouts, RPC freeze | Capture owned batch under lock; Fjall after release | First flush measured in seconds (Knots warns at GiB scale) |
| Fetch every `HaveCoin` into cache and never `Uncache` | Cache hits CRITICAL during package/tx validation | Knots-style uncache of coins pulled only for failed checks | Hostile invalid-tx floods (Knots comments this in `validation.cpp`) |
| Sync every key instead of dirty-only | Flush time tracks UTXO set, not mutations | Cursor skips non-dirty entries | After the first large cache |
| One Fjall put per coin | Commit overhead dominates | Sized batches plus heads/best-block markers | Tens of thousands of dirty coins |

## Security Mistakes

| Mistake | Risk | Prevention |
| --- | --- | --- |
| Accept a coins DB whose best-block does not match index/undo | Consensus fork or inflation from resurrected coins | Fail closed; typed inconsistency; no auto-reindex |
| Treat interrupted flush as consistent | Mixed old/new spends | Heads marker + replay-or-refuse |
| Serve bodies inferred from coins | Wrong or missing witness data; peer abuse | Payload store only |
| Import LevelDB or rust-bitcoin | Policy bypass; unowned consensus types | Dependency and architecture checkers |
| Silent prune of block files during flush | Data loss framed as "Knots FlushStateToDisk" | No prune-mode in v2.3 |
| Trust a legacy snapshot after coins cutover | Attacker-controlled or stale blob wins | Generation/version gate; one source of truth |
| Leak UTXO or block hashes into metrics/support | High-cardinality and privacy leaks | Aggregate counts; existing redaction contract |
| Destructive repair of a bad coins DB | Operator data loss | Diagnostic recovery only; v1.8 destructive-repair stays deferred |

## UX Pitfalls

| Pitfall | User Impact | Better Approach |
| --- | --- | --- |
| Status says "durable chainstate" after a cache apply | Operators expect crash survival they do not have | Show last committed coins best-block, pending dirty count, flush mode, and crash-loss window |
| `Pruned` label read as prune-mode | Operators think historical data was deliberately dropped | Keep the v2.1 meaning: missing local payload on a non-tip active hash; say so in help text |
| `Unavailable` vs `Pruned` vs index-present | Support tickets and false archive claims | One table: payload present → may serve; else NotFound + exact label |
| "Fuller chainstate manager" in README | Implies assumeutxo and archive | Name flush lifecycle and honest availability only |
| Progress height ahead of flushed coins tip | Restart looks like data loss | Surface both connected tip and flushed tip until they match |
| Recovery text says `-reindex-chainstate` | Operators run a flag that does not exist | Typed Open Bitcoin recovery action; no Knots-flag cosplay |

## "Looks Done But Isn't" Checklist

- [ ] **Coins store:** Keys round-trip in a unit test, but there is no committed best-block, heads marker, or restart comparison to index/undo/payload.
- [ ] **Cache:** Lookups work, but dirty/fresh reorg cases and `HaveCoin` vs in-cache are untested.
- [ ] **Flush:** A `flush()` function exists, but order vs block files/index is unspecified and `Flush`/`Sync` are the same.
- [ ] **Manager:** `ManagedChainstate` still `save_snapshot`s the full map after every connect.
- [ ] **Restart:** Process reopen works for headers, but UTXOs reload from the old blob or from cache-only memory.
- [ ] **Progress:** Tests credit connect without a flush receipt.
- [ ] **Rescan/migration:** Wallet rescan still requires `load_chainstate_snapshot`.
- [ ] **Availability:** Gate uses `durable_availability: true` without a payload probe on the serve path.
- [ ] **Claims:** Catalog gap sentence still groups assumeutxo with coins; no-claim checkers were deleted instead of narrowed.
- [ ] **Architecture:** Core crate gained I/O, or the shell holds the lock across Fjall.
- [ ] **History:** A cleanup commit removes `.planning/phases/` that verifiers still cite.

## Recovery Strategies

| Pitfall | Recovery Cost | Recovery Steps |
| --- | --- | --- |
| Coins/index/undo/payload fork | HIGH | Stop sync and serving; preserve the datadir; classify which store is ahead; refuse mutate/repair; restore from backup or wait for an explicit future repair gate |
| Cache treated as durable | HIGH | Stop progress credit; flush or discard dirty cache; reopen from committed best-block; add cache-vs-disk tests |
| DIRTY/FRESH spentness bug | HIGH | Do not flush the bad cache; reopen from last consistent coins+undo; add the Knots reorg fixture; rewrite the cache algebra |
| Interrupted flush without heads | HIGH | Treat the coins keyspace as inconsistent; do not serve or credit; require restore or scoped rebuild once a repair milestone exists |
| Dual snapshot + coins truth | MEDIUM | Pick one generation; make the other read-only; migrate; fail reopen if both writers still exist |
| Serve without payload | MEDIUM | Fail closed to `NotFound`; fix availability classification; add payload-absent fixtures; retract archive/prune wording |
| I/O in core crate | MEDIUM | Move adapters to `open-bitcoin-node`; restore pure tests; add architecture-checker fixtures |
| Claim creep | MEDIUM | Restore no-claim text; add negative fixtures; do not ship until the bounded taxonomy is in README, parity, and status |
| Broken restart/rescan | HIGH | Keep the last snapshot-capable build's datadir readable; dual-read only during a named migration generation |

## Pitfall-to-Phase Mapping

Map to durability phases starting at **Phase 139**. Do not reuse archived phase numbers (4, 70–78, 111) as if they were the new work.

| Pitfall | Prevention phase | Verification |
| --- | --- | --- |
| Cache hit ≠ durable truth; dirty/fresh; I/O in core; no LevelDB/rust-bitcoin | **Early durability** — coins store and cache contract (Phase 139+) | Pure cache tests; shell trait has no core I/O; dependency checker; cache-vs-disk distinction in types |
| Dual snapshot + coins truth | **Early** names the source of truth; **mid** cutover | Reopen loads coins, not a leftover blob; old snapshot is migration-only |
| Flush order vs index/undo/payload; interrupted heads; Flush vs Sync; lock release | **Mid durability** — flush policy and chainstate-manager | Crash after each flush step; consistency tuple equal; `Sync` keeps cache; lock not held during Fjall |
| Restart/rescan/progress credit break | **Mid durability** | Same-datadir reopen, wallet rescan, confirmation fixtures, progress_credit only after flush receipt |
| Serve/label without payload; pruned≠prune-mode | **Late durability** — honest availability | `load_block` None → NotFound; tip missing → Unavailable; non-tip missing → existing Pruned meaning; no archive serve |
| assumeutxo/prune/archive/production/public-default claim creep; phase-dir deletion | **Late durability** — claim and verifier guardrails | Positive bounded claims; negative fixtures; `.planning/phases/` still tracked |

## Roadmap Ordering Implications

The highest-risk ordering mistake is implementing "chainstate manager" or operator evidence before a single durable coin source of truth and a cache-vs-disk distinction. If flush policy is written against today's `save_snapshot`, every later phase will keep snapshot semantics.

Do not build prune, assumeutxo, or archive serving as scaffolding for flush. Knots' flush function mentions them; copying those branches is how v2.3 becomes a sync-shortcut milestone.

Do not change block-serving labels to "more honest" by inferring availability from coins. Honesty is payload presence. Coins durability and historical serving are different products.

Restart/resume must stay green across the cutover. The current daemon **depends** on snapshot hydrate in `DurableSyncRuntime::open`. A coins write that does not reopen is not durability.

Claim guardrails belong last as enforcement, but the no-claim taxonomy must be written in requirements first so mid-phase PRs cannot smuggle LevelDB, assumeutxo, or archive language.

## Research Flags

- **Fjall interrupted-flush representation:** Knots' `DB_HEAD_BLOCKS` protocol is verified. The equivalent Fjall batch/marker design is a requirements decision. **Confidence: HIGH on the need; MEDIUM on the exact encoding.**
- **Allowed crash-loss window:** Whether v2.3 promises periodic, clean-shutdown, or every-connect durability is a product choice. Today's snapshot dump after in-memory persist is not the same as Knots periodic write. **Confidence: MEDIUM pending requirements.**
- **Undo storage location:** Knots keeps undo with block files; Open Bitcoin keeps `undo_by_block` inside `ChainstateSnapshot`. Moving undo without a flush-order gate is a rewrite risk. **Confidence: HIGH that it must be decided before incremental coins.**
- **`durable_availability` semantics:** The durable gate currently passes `true` to allow a later read. Requirements must say the serve/status decision still requires a payload probe. **Confidence: HIGH on the hazard; MEDIUM on the final flag shape.**

## Sources

All decisive behavior claims were verified against local primary sources. The pinned submodule is Bitcoin Knots `29.3.knots20260210`.

### Pinned Bitcoin Knots

- `packages/bitcoin-knots/src/coins.cpp` — `AddCoin` FRESH/DIRTY reorg comments, `SpendCoin` fresh-erase, `Flush` vs `Sync`, `HaveCoin` vs `HaveCoinInCache`, `BatchWrite` parent-cache rules. **HIGH confidence.**
- `packages/bitcoin-knots/src/coins.h` — `Coin` spentness and cache-entry model. **HIGH confidence.**
- `packages/bitcoin-knots/src/validation.cpp` `FlushStateToDisk` — write order (block/undo files, block index, optional prune unlink, coins Flush/Sync), disk-space fatal errors, `IF_NEEDED`/`PERIODIC`/`ALWAYS`. **HIGH confidence.**
- `packages/bitcoin-knots/src/validation.cpp` `ReplayBlocks` / `RollforwardBlock` — two-head inconsistent state, rollback/rollforward from block bodies. **HIGH confidence.**
- `packages/bitcoin-knots/src/txdb.cpp` `CCoinsViewDB::BatchWrite` — `DB_HEAD_BLOCKS` then batched coin writes then `DB_BEST_BLOCK`; crash simulation hook. **HIGH confidence.**
- `packages/bitcoin-knots/src/validation.cpp` assumeutxo cache-resize / `PopulateAndValidateSnapshot` — exists in Knots; **out of scope** for v2.3. **HIGH confidence that it is adjacent and tempting.**

### Current Open Bitcoin

- `packages/open-bitcoin-node/src/chainstate.rs` — `save_snapshot` after every connect/disconnect/reorg. **HIGH confidence.**
- `packages/open-bitcoin-chainstate/src/engine.rs` and `types.rs` — in-memory UTXO + undo snapshot as coin truth. **HIGH confidence.**
- `packages/open-bitcoin-node/src/sync.rs` — reopen hydrates `MemoryChainstateStore` from `load_chainstate_snapshot_with_confirmation_migration`. **HIGH confidence.**
- `packages/open-bitcoin-node/src/sync/runtime_state.rs` — `persist_progress` writes headers, full snapshot, and metadata as separate puts. **HIGH confidence.**
- `packages/open-bitcoin-node/src/storage/fjall_store.rs` — snapshot blob, header/index batch, per-hash `save_block`/`load_block`. **HIGH confidence.**
- `packages/open-bitcoin-node/src/network/inventory.rs` — `has_local_data \|\| durable_availability`; durable gate passes `durable_availability: true`. **HIGH confidence.**
- `packages/open-bitcoin-network/src/block_serving.rs` and `packages/open-bitcoin-node/src/network/tests/block_serving.rs` — Available / Pruned / Unavailable; payload-missing NotFound; no archive serve. **HIGH confidence.**
- `docs/parity/catalog/chainstate.md` — disk-backed coins, cache-flush, and assumeutxo still listed as known gaps. **HIGH confidence.**
- `docs/parity/deviations-and-unknowns.md` and `docs/parity/production-claim-boundary.md` — archive-node and production-scale historical serving deferred. **HIGH confidence.**
- `.planning/PROJECT.md` — v2.3 scope; assumeutxo/assumevalid/prune/archive out of scope; historical phases stay tracked. **HIGH confidence.**

### Still-valid claim-boundary lessons (previous milestone research)

- Short milestone names expand into public-default, production, and protocol claims unless checkers get a **narrower replacement contract**.
- Default `bash scripts/verify.sh` must stay deterministic; live durability soaks stay opt-in UAT.
- Split authority and lock-across-I/O already failed other runtimes; do not reintroduce them for coins flush.

### Local standards

- `standards/core/architecture.md` — functional core stays I/O-free. **HIGH confidence.**
- `AGENTS.md` / `.planning/PROJECT.md` — no rust-bitcoin in the production path; Fjall is the durable adapter. **HIGH confidence.**

---
*Pitfalls research for: Adding chainstate durability to Open Bitcoin*
*Researched: 2026-08-29*
