---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 148-2026-09-27T02-46-06
generated_at: 2026-09-27T14:46:28.789Z
---

# Phase 148: Fjall Payload Unlink and Have-Pruned - Context

**Gathered:** 2026-09-27
**Status:** Ready for planning
**Mode:** Yolo
**Gap update:** 2026-09-27 — error-path cache eviction only. D-01 through D-17 are unchanged.

<domain>
## Phase Boundary

Eligible heights lose paired block and undo payloads on disk, and
have-pruned is recorded only after that delete is durable.

This phase delivers UNLK-01, UNLK-02, and UNLK-03. It consumes the Phase
147 pure `PrunePlan` and deletes that height's Fjall block payload and
undo together. It records have-pruned only after a non-empty durable
delete succeeds. Restart after an interrupted prune finishes the partial
delete or refuses closed without inventing blocks or triggering reindex.
Heights covered by the Phase 147 lock buffer stay present.

This phase does not introduce a `blk`/`rev` flat-file store, a new crate,
assumeutxo, a second chainstate, BIP37, `-pruneduringinit`, txindex, or
automatic destructive reindex. It does not advertise
`NODE_NETWORK_LIMITED`, emit the `Pruned` label, or add RPC, CLI, or
dashboard prune commands. Those belong to Phases 149–151.

</domain>

<decisions>
## Implementation Decisions

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

### Error-path cache eviction

- **D-18:** If a paired delete has already committed and the flush then
  returns an error, those hashes are removed from the in-memory block
  cache before the caller observes the error. Success-path eviction is
  not enough. Inventory treats cache presence as payload presence, so a
  committed delete that stays cached is still servable for the rest of
  the process.
- **D-19:** The call that committed the delete is the eviction. A later
  attempt that finds both mates already absent does not report those
  hashes as deleted and must not put the cache entry back. Do not treat
  that later attempt as a second chance to evict, and do not undo the
  committed delete to clear the cache.
- **D-20:** This gap does not reopen disk pairing, have-pruned timing,
  restart finish-or-refuse, lock-buffer skips, or the keep window.
  Labels, `NODE_NETWORK_LIMITED`, and operator commands stay deferred.

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

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase contract

- `.planning/ROADMAP.md` — Phase 148 goal, UNLK-01, UNLK-02, UNLK-03,
  success criteria, and the research flag for Fjall delete-batch
  atomicity, height-to-key indexing, and crash seams
- `.planning/REQUIREMENTS.md` — UNLK-01..03 wording; FUT-23 automatic
  reindex stays deferred; `blk`/`rev`, assumeutxo, txindex+prune, and
  public defaults stay out of scope
- `.planning/PROJECT.md` — functional core stays I/O-free; externally
  observable Knots behavior stays on the outside
- `.planning/research/PITFALLS.md` — Pitfalls 2, 3, 7, 8, and 9: have-pruned
  timing, unlink order, paired undo, and cache resurrection
- `.planning/phases/147-pure-prune-policy-and-lock-windows/147-CONTEXT.md`
  — pure `PrunePlan`, keep window, lock buffer, and the explicit handoff
  that Phase 148 owns deletion and have-pruned
- `.planning/phases/146-wallet-leftover-snapshot-cutover/146-CONTEXT.md`
  — leftover snapshot bytes are not chain truth

### Pinned Knots anchors

- `packages/bitcoin-knots/src/validation.cpp` — prune flush order: non-empty
  candidate set, have-pruned, index write, unlink, then coins
- `packages/bitcoin-knots/src/node/blockstorage.cpp` —
  `PruneOneBlockFile`, `FindFilesToPrune`, paired block and undo removal,
  `DoPruneLocksForbidPruning`
- `packages/bitcoin-knots/src/node/blockstorage.h` — `PruneLockInfo` and
  have-data / have-undo presence flags
- `packages/bitcoin-knots/src/validation.h` — `MIN_BLOCKS_TO_KEEP` (288)

### Existing Open Bitcoin seams

- `packages/open-bitcoin-chainstate/src/prune/plan.rs` — `PrunePlan`,
  `plan_automatic_prune`, `plan_manual_prune`; this phase consumes heights
  and does not re-decide policy
- `packages/open-bitcoin-chainstate/src/prune/locks.rs` —
  `height_forbidden_by_any_lock` and the 10-block buffer
- `packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs` —
  `persist_ordered_prefix`; the single flush owner to extend
- `packages/open-bitcoin-node/src/storage/fjall_store/blocks.rs` —
  `has_block` on `block:<hash>`
- `packages/open-bitcoin-node/src/storage/fjall_store/coins.rs` —
  `load_undo` and `undo_key`
- `packages/open-bitcoin-node/src/storage/fjall_store.rs` — `block_key`
  and `load_block`
- `packages/open-bitcoin-node/src/storage/coins_view.rs` — existing Fjall
  `OwnedWriteBatch` commit path
- `docs/parity/source-breadcrumbs.json` — required breadcrumb registry for
  new first-party Rust files

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- `PrunePlan.heights` is already the candidate list. Unlink should take
  that list plus lock facts, not reimplement the 288-block window or the
  550 MiB target.
- `height_forbidden_by_any_lock` already encodes the Phase 147 buffer.
  Unlink can call it again so a stale plan cannot delete a protected
  height.
- `FjallNodeStore::has_block`, `load_block`, and `load_undo` already
  address the two keys. The delete path should use those same key helpers.
- `coins_view` already commits `fjall::OwnedWriteBatch`. Paired removal
  should reuse that durability path rather than a new engine.

### Established Patterns

- Chainstate crate stays I/O-free. Policy was decided in Phase 147;
  Fjall effects belong in `open-bitcoin-node` storage and flush lifecycle.
- Flush already orders block payloads, undo, then header/index entries,
  then coins. Unlink extends that owner; it does not run beside it.
- Phase 146 wallet rescan fails closed when payload bytes are missing and
  must not invent have-pruned. This phase is the first one allowed to set
  that fact, and only after a real delete.

### Integration Points

- Automatic and manual planners in
  `packages/open-bitcoin-chainstate/src/prune/plan.rs`.
- Ordered flush in
  `packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs`.
- Block and undo key helpers in `fjall_store.rs` and
  `fjall_store/coins.rs`.

</code_context>

<specifics>
## Specific Ideas

Have-pruned follows the v2.4 success criterion, which is stricter than
setting the Knots flag before the files are unlinked: the flag is recorded
only after the non-empty delete batch is durable. Knots still supplies the
paired block-and-undo rule, the lock skip, and the flush-owner order.

No specific UI or operator copy — those surfaces are later phases.

</specifics>

<deferred>
## Deferred Ideas

- `NODE_NETWORK_LIMITED` advertisement and limited-window getdata refusal
  — Phase 149.
- Honest `Pruned` versus `Unavailable` labels — Phase 149. This phase must
  not flip inventory to `Pruned` just because a key is missing.
- Prune RPC, CLI, dashboard fields, manual prune invocation, and operator
  prune-lock setters — Phase 150.
- Full v2.4 parity-doc pass and no-claim checkers — Phase 151.
- `-pruneduringinit` — FUT-27, not this milestone.
- Automatic destructive reindex or coins repair — FUT-23.

</deferred>

---

*Phase: 148-fjall-payload-unlink-and-have-pruned*
*Context gathered: 2026-09-27*
