---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 150-2026-09-28T16-02-35
generated_at: 2026-09-28T16:04:26.673Z
---

# Phase 150: Operator Prune Surfaces and Evidence - Context

**Gathered:** 2026-09-28
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Operators can inspect prune state, request a manual prune, manage prune locks, and read sanitized support evidence. This phase covers OPER-01, OPER-02, OPER-03, and LOCK-02.

The node already has typed `PruneMode`, the 288-block keep window, the 10-block lock buffer, durable paired-payload delete, have-pruned, limited serving, and the earned block-availability `Pruned` label. This phase exposes those facts and the manual-prune / lock controls. It does not add parity catalog claims, archive serving, assumeutxo, BIP37, compact filters, public-network defaults, or a second chainstate. Functional-core crates stay I/O-free.

</domain>

<decisions>
## Implementation Decisions

### Prune status fields

- **D-01:** OPER-01 is the Knots `getblockchaininfo` quartet: `pruned`, `pruneheight`, `automatic_pruning`, and `prune_target_size`. Project the same facts on the Open Bitcoin status snapshot, the CLI status view, and the terminal dashboard.
- **D-02:** `getblockchaininfo.pruned` means prune mode is on (`PruneMode::ManualOnly` or `PruneMode::Automatic`). It is not the Phase 149 block-availability label. Configured mode alone does not set durable have-pruned and does not relabel a missing payload as `Pruned`.
- **D-03:** `automatic_pruning` is true only for `PruneMode::Automatic`. `prune_target_size` is the automatic target in bytes and is present only when automatic pruning is on. Manual-only reports `automatic_pruning: false` and omits `prune_target_size`. Disabled omits the prune-only keys and reports `pruned: false`.
- **D-04:** `pruneheight` is the Knots lowest-height complete block still stored, present only when prune mode is on. Research must quote the pinned Knots `getblockchaininfo` / prune-height helper before planning the integer. Do not substitute the support-evidence last-delete height for this field.
- **D-05:** Keep Phase 149 `pruned_count` and `BlockServingDataAvailability::Pruned` on the earned rule: have-pruned is set, the payload is gone, and the block is validated on the active chain. Status JSON must expose configured-mode fields and the earned count as separate facts.

### Config so those reads are real

- **D-06:** Wire the existing `parse_prune_arg` integer into Open Bitcoin JSONC as `prune`. `0` is disabled, `1` is manual-only, and `N >= 550` is an automatic target in MiB. Values `2..=549` and negatives stay typed refusals. Default remains `0`.
- **D-07:** Do not add a second constructor or a parallel byte-target field. Phase 147 left JSONC for this phase; the integer contract stays the one `parse_prune_arg` already implements.
- **D-08:** The running node, RPC, CLI status, and dashboard all read that resolved mode. Tests may still call `set_prune_mode` directly.

### Manual prune request

- **D-09:** OPER-02 is Knots `pruneblockchain <height>` on RPC, plus a CLI command that requests the same manual prune. Dashboard shows the height or typed refusal and does not offer a destructive prune control.
- **D-10:** Disabled mode returns the existing typed refusal and does not delete payloads or set have-pruned. A target inside the 288-block keep window is refused without clamping. Success returns the resulting prune height after the Phase 148 durable delete path runs.
- **D-11:** Manual prune does not invent an automatic byte-budget plan. Automatic mode may still accept `pruneblockchain` the way Knots does once the manual height check passes; research quotes that Knots rule before planning.

### Prune locks

- **D-12:** LOCK-02 lists and sets named locks through RPC and CLI. A lock is the existing `PruneLockInfo`: name plus inclusive `height_first` and `height_last`. The same name replaces the previous range.
- **D-13:** Setting includes a clear-by-name so a lock is not one-way. Dashboard lists locks and does not edit them.
- **D-14:** Locks persist in node storage and survive restart. They feed the existing pure lock-buffer planner. This phase does not change the 10-block buffer formula.

### Sanitized support evidence

- **D-15:** OPER-03 adds a prune section to the existing support-evidence JSON and Markdown bundle.
- **D-16:** The section reports prune counts and the last prune height: how many successful prune batches have committed, how many heights lost payloads, and the highest height deleted by the last successful batch. Before any successful delete, counts are zero and last prune height is absent.
- **D-17:** The section does not include datadir paths, Fjall key names, filesystem paths, block hashes, or raw lock names that embed paths. Follow the existing support-bundle redaction pattern.

### Boundaries carried forward

- **D-18:** No archive serving, assumeutxo, BIP37, compact-filter serving, public serving or relay defaults, public-network CI, production full-node claims, or production-funds wallet claims.
- **D-19:** Phase 151 owns parity catalog docs and no-claim checkers. This phase still adds parity breadcrumbs for new first-party Rust under `packages/open-bitcoin-*/src` and `packages/open-bitcoin-*/tests`.
- **D-20:** Verification remains `bash scripts/verify.sh`. Default verification stays deterministic and public-network-free. No new production crate or third-party library.

### Claude's Discretion

- Exact dashboard section layout, as long as the four status facts and the lock list are visible and no control deletes payloads.
- CLI subcommand spelling, as long as RPC stays `pruneblockchain`, list, set, and clear, and the CLI calls that same behavior.
- Whether last-prune counters live beside the have-pruned marker, as long as they advance only after a durable delete succeeds.
- JSONC file placement for `prune`, as long as it uses `parse_prune_arg` and defaults to disabled.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase contract

- `.planning/ROADMAP.md` — Phase 150 goal, OPER-01, OPER-02, OPER-03, LOCK-02, and success criteria. Phase 151 owns parity roots.
- `.planning/REQUIREMENTS.md` — Operator Evidence and LOCK-02 wording. FUT prune-during-init, txindex+prune, archive, assumeutxo, BIP37, and public defaults stay out.
- `.planning/PROJECT.md` — v2.4 prune milestone. Functional core stays I/O-free.
- `.planning/phases/147-pure-prune-policy-and-lock-windows/147-CONTEXT.md` — D-01 through D-17. Operator surfaces were deferred here. `parse_prune_arg`, keep window, and lock buffer are locked.
- `.planning/phases/149-limited-serving-and-honest-pruned-labels/149-CONTEXT.md` — D-09 through D-12 and D-20. Earned `Pruned` is not the getblockchaininfo mode bit. Operator fields start in this phase.

### Existing Open Bitcoin seams

- `packages/open-bitcoin-chainstate/src/prune/mode.rs` — `PruneMode` and `parse_prune_arg`
- `packages/open-bitcoin-chainstate/src/prune/plan.rs` — automatic and manual planners, including disabled and keep-window refusals
- `packages/open-bitcoin-chainstate/src/prune/locks.rs` — `PruneLockInfo` and the 10-block buffer
- `packages/open-bitcoin-node/src/status.rs` — `OpenBitcoinStatusSnapshot`
- `packages/open-bitcoin-node/src/status/block_serving.rs` — earned `pruned_count`
- `packages/open-bitcoin-cli/src/operator/support.rs` — support-evidence bundle
- `packages/open-bitcoin-cli/src/operator/dashboard/mod.rs` — terminal dashboard
- `packages/open-bitcoin-rpc/src/method.rs` — RPC method registry, including `getblockchaininfo`

### Pinned Knots anchors

- `packages/bitcoin-knots/src/rpc/blockchain.cpp` — `getblockchaininfo` prune keys and `pruneblockchain`
- `packages/bitcoin-knots/src/node/blockstorage.h` — `PruneLockInfo`
- `packages/bitcoin-knots/src/node/blockstorage.cpp` — lock buffer and prune-height helper
- `packages/bitcoin-knots/src/validation.h` — `MIN_BLOCKS_TO_KEEP` (288)
- `packages/bitcoin-knots/src/node/blockmanager_args.cpp` — `ParsePruneOption` (`0` / `1` / `>=550`)

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- `parse_prune_arg` already maps Knots prune integers and refuses `2..=549`.
- `plan_manual_prune` already refuses disabled mode and keep-window targets without clamping.
- `PruneLockInfo` is the lock shape. Persistence and list/set were explicitly out of scope in Phase 147.
- `OpenBitcoinStatusSnapshot` and the Ratatui dashboard already project operator status. `pruned_count` is the earned availability count.
- Support evidence is already a redacted JSON plus Markdown bundle under `packages/open-bitcoin-cli/src/operator/support.rs`.

### Established Patterns

- Baseline RPC methods keep Knots names and shapes. Open Bitcoin-only status stays on the status snapshot and CLI/dashboard.
- Destructive operator actions are requested from RPC and CLI. The dashboard reads status.
- Have-pruned and payload deletion stay in the node storage adapter. Chainstate planners stay pure.

### Integration Points

- JSONC config resolution feeds the node prune mode used by serving and by these new reads.
- `pruneblockchain` must call the manual planner and then the Phase 148 durable unlink.
- Lock storage must be visible to the planner on the next prune, including after restart.
- Support-evidence collection reads status or store facts. It must not print storage paths.

</code_context>

<specifics>
## Specific Ideas

No extra product wording beyond the Knots quartet and the earned-versus-configured split. Research quotes `getblockchaininfo` and `pruneblockchain` before planning integer and return-height details.

</specifics>

<deferred>
## Deferred Ideas

- Parity catalog docs and no-claim checkers — Phase 151 (GRD-01).
- `-pruneduringinit`, txindex-plus-prune, archive serving, assumeutxo, BIP37, compact filters, and public defaults stay outside v2.4.
- A dashboard control that deletes block payloads. Status and last-outcome display only.

</deferred>

---

*Phase: 150-operator-prune-surfaces-and-evidence*
*Context gathered: 2026-09-28*
