# Project Research Summary

**Project:** Open Bitcoin — v2.4 Prune-Mode Product Behavior
**Domain:** Bitcoin Knots-aligned prune-mode product behavior (height windows, payload unlink, `m_have_pruned`, prune locks, `NODE_NETWORK_LIMITED`)
**Researched:** 2026-09-21
**Confidence:** HIGH

## Executive Summary

v2.4 is a **bounded-disk full-node** milestone on top of shipped v2.3 durable coins and honest payload-present availability. Experts build prune as three distinct Knots facts—prune *mode* (startup config), *have-ever-pruned* (durable flag set only after real deletes), and *payload absence*—not as “missing bytes ⇒ pruned.” Open Bitcoin must keep those facts separate while mapping Knots file unlink onto height-selected Fjall `block:` / undo key deletion (document that storage-granularity difference in parity docs).

The recommended approach adds **no new production crates and no new third-party libraries**. Extend `open-bitcoin-chainstate` (pure height-window / lock / have-pruned decisions), `open-bitcoin-network` (`NETWORK_LIMITED` + serve window + honest `Pruned` facts), and `open-bitcoin-node` (Fjall delete, durable `have_pruned`, flush orchestration, wallet-rescan cutover). First phase cuts wallet rescan off leftover `ChainstateSnapshot` bytes before any unlink. Then: prune policy/state → unlink → serving limits and `Pruned` labels → operator evidence and no-claim guardrails. Roadmapper assigns exact phase numbers starting at 146.

Key risks are label lies (emitting `Pruned` without delete), snapshot/cache resurrection after unlink, deleting inside the tip − 288 keep window, setting `have_pruned` from config alone, flush-order / block-without-undo inconsistency, advertising full `NETWORK` after prune, and claim creep into archive / assumeutxo / BIP37 / LevelDB import / auto-reindex / production. Mitigate with cutover-first ordering, Knots `IsBlockPruned`-shaped predicates, paired payload+undo units, prune locks before production unlink, and retargeted (not deleted) claim checkers.

## Key Findings

### Recommended Stack

Detailed findings: [STACK.md](./STACK.md)

**No stack additions.** Reuse Rust `1.94.1`, Fjall `3.1.4`, existing serde/clap/jsonc, and first-party crates only. Do not add rust-bitcoin, LevelDB, RocksDB, `bitflags`, a FlatFileSeq/`blk*.dat` layer, a second chainstate, or Tokio in the functional core.

**Core technologies:**
- **Rust `1.94.1` / edition 2024:** Pure prune-range, lock, flush-for-prune, and serve-window state machines — already pinned; enums make eligible vs forbidden outcomes explicit without I/O.
- **Existing workspace crates (`0.1.0`):** Own coins, payloads, availability labels, service flags, wallet rescan — extend; do not invent a prune crate.
- **Bitcoin Knots `29.3.knots20260210`:** External prune behavior contract — vendored anchors are parity roots.
- **Fjall `3.1.4`:** Durable delete of block/undo payloads and persist of `have_pruned` / prune-lock records — maps Knots unlink + flag write without a second database.
- **serde / serde_json (existing):** Versioned prune-lock and operator/evidence shapes — metadata only; not coin or block truth.

**First-party extensions:** `open-bitcoin-chainstate` prune/flush policy; `open-bitcoin-network` `ServiceFlags::NETWORK_LIMITED` + serve gates; `open-bitcoin-node` `FjallNodeStore` deletes, `ManagedChainstate` / flush lifecycle, inventory facts, `WalletRescanRuntime` cutover; thin RPC/CLI operator surfaces.

### Expected Features

Detailed findings: [FEATURES.md](./FEATURES.md)

**Must have (table stakes):**
- Wallet leftover-snapshot cutover before first unlink — rescan must not resurrect snapshot-as-truth
- `-prune` product modes (`0` / `1` / `>=550` MiB) and height windows (`MIN_BLOCKS_TO_KEEP` = 288, `PruneAfterHeight`)
- Height-selected block/undo unlink + persist `m_have_pruned` / `prunedblockfiles` analogue
- Prune locks (`PRUNE_LOCK_BUFFER` = 10) that forbid deleting locked ranges
- `NODE_NETWORK_LIMITED` advertisement (no full `NETWORK` in prune mode) and tip-distance serve limits (288 + 2)
- Emit `Pruned` only when durable `have_pruned` is set and payload is gone; missing-without-prune stays `Unavailable`
- Operator prune surfaces (mode / pruneheight / manual prune) + parity roots and no-claim guardrails

**Should have (competitive):**
- Typed prune decision state machine in pure core
- Explicit `Pruned` vs `Unavailable` vs `Available` facts + sanitized prune-event evidence
- Fail-closed snapshot cutover as phase-0 gate; claim taxonomy in operator copy

**Defer (v2.4.x / later milestones):**
- Full `listprunelocks` / `setprunelock` RPC parity if MVP ships locks via adapters only; `-pruneduringinit`; crash mid-unlink cleanup
- Compact-filter serving, assumeutxo / dual chainstate, archive-node, BIP37, LevelDB import, auto-reindex, public defaults / public-network CI, production readiness, txindex+prune

### Architecture Approach

Detailed findings: [ARCHITECTURE.md](./ARCHITECTURE.md)

Keep prune **decisions** I/O-free in `open-bitcoin-chainstate`; keep Fjall deletes, durable `have_pruned` writes, and filesystem effects in `open-bitcoin-node`. Behavioral parity is height windows, locks, have-pruned, serving limits, and honest labels—not a line-for-line `blk?????.dat` layout. Ordering: decide → persist flag on non-empty plan → unlink → project `NETWORK_LIMITED` and `Pruned` labels.

**Major components:**
1. **`WalletRescanRuntime`** — Rescan from coins/headers/present payloads; never leftover snapshot (cut over first)
2. **Pure prune policy (`open-bitcoin-chainstate/prune/`)** — Height window, target, locks → `PrunePlan` (no Fjall)
3. **`ManagedChainstate` / `FlushLifecycle`** — Single-chainstate orchestration mirroring Knots `FlushStateToDisk` prune branch
4. **`FjallNodeStore`** — `has_block`, paired `block:` + undo deletes, durable `have_pruned`
5. **Availability + service-flag projectors (`open-bitcoin-network` + node adapters)** — `Pruned` only after earned facts; `NETWORK_LIMITED` + tip-distance gates
6. **Operator evidence / claim checkers** — Sanitized prune evidence; scoped claims only

### Critical Pitfalls

Detailed findings: [PITFALLS.md](./PITFALLS.md)

1. **Wallet rescan still treats leftover snapshot as truth** — Cut rescan off `load_chainstate_snapshot` before any unlink; drive from coins/headers/payload-present reads.
2. **Emitting `Pruned` for any missing payload** — Gate on durable `have_pruned` ∧ intentional delete; keep `Unavailable` otherwise (Knots `IsBlockPruned`).
3. **Setting have-pruned before files are deleted (or never after)** — Flip flag only after a non-empty durable delete batch through ordered flush; never from config parse alone.
4. **Deleting inside tip − 288 / leaving block without undo** — Hard prune ceiling at keep window; treat payload+undo as one atomic unit; clear both presence facts together.
5. **Wrong service flags / claim creep** — Add `NETWORK_LIMITED`, enforce depth gate, clear full `NETWORK` when Knots would; retarget claim checkers—do not gut them for archive/assumeutxo/auto-reindex/production.

Also watch: unlink without prune locks beside rescan/serve; flush-order breaking index↔payload↔coins; warm cache resurrecting deleted bodies after unlink.

## Implications for Roadmap

Based on research, suggested phase structure (roles starting at **146**; roadmapper assigns exact numbers—do not treat these as a written ROADMAP):

### Phase 146: Wallet leftover-snapshot cutover
**Rationale:** v2.3 left rescan on leftover snapshot while coins restart already ignores it; unlink without cutover resurrects snapshot-as-truth.
**Delivers:** Coins/headers/payload-backed rescan; leftover `SNAPSHOT_KEY` non-authoritative on wallet path.
**Addresses:** Wallet leftover-snapshot cutover (table stake / first MVP gate)
**Avoids:** Pitfall 1 (snapshot-as-truth); Pitfall 9 resurrection channel on wallet path

### Phase 147: Prune policy and state
**Rationale:** Pure height windows, targets, locks, and typed have-pruned *decisions* must exist before shell deletes.
**Delivers:** I/O-free `PruneRange` / lock forbid / flush-for-prune facts; `-prune` mode parsing shapes (no unlink yet).
**Addresses:** `-prune` modes + height windows; prune-lock policy types
**Avoids:** Pitfall 4 (delete inside keep window)—encoded as hard ceiling before candidates

### Phase 148: File/key unlinking
**Rationale:** Disk savings require real deletes; have-pruned is a side effect of successful non-empty unlink, not policy alone.
**Delivers:** Height-selected Fjall `block:` + undo deletion; durable `have_pruned`; prune locks gate candidates; flush-lifecycle order (index → unlink → coins).
**Addresses:** Block/undo unlink + `m_have_pruned`; prune locks at execution time
**Avoids:** Pitfalls 3, 5, 7, 8 (flag-before-delete, locks, flush order, unpaired deletes); document Fjall-vs-`blk*.dat` granularity in `docs/parity/`

### Phase 149: Serving limits and `Pruned` labels
**Rationale:** Labels and peer advertisement must follow real deletes; wiring `Pruned` before unlink invents the claim.
**Delivers:** `ServiceFlags::NETWORK_LIMITED`; tip-distance getdata / getblocks stop; inventory emits `Pruned` only when earned; missing-without-prune stays `Unavailable`.
**Addresses:** `NODE_NETWORK_LIMITED` + honest `Pruned`/`Unavailable` split (HAVL-02 fulfillment)
**Avoids:** Pitfalls 2 and 6 (fake Pruned; NETWORK / LIMITED mistakes)

### Phase 150: Operator evidence and no-claim guardrails
**Rationale:** Close the milestone with sanitized evidence and retargeted checkers so scoped prune claims do not open archive/assumeutxo/production doors.
**Delivers:** Operator prune mode / pruneheight / evidence; parity breadcrumbs; fail-closed unprune-without-rebuild stance; no archive / assumeutxo / BIP37 / LevelDB import / auto-reindex / public-default / production claims.
**Addresses:** Operator surfaces + parity/no-claim table stakes
**Avoids:** Pitfall 10 (claim creep)

### Phase Ordering Rationale

- **Cutover → policy → unlink → serve/labels → evidence** matches feature dependencies and Knots flush ordering.
- Serving labels before unlinks would invent `Pruned` without deletes; unlinks before cutover risk rescan reading stale snapshot coins after payloads vanish.
- Group policy (pure) separate from unlink (shell) to preserve functional-core / imperative-shell and architecture verifiers.
- Historical `.planning/phases/` directories stay tracked; phase numbering continues from 146 after v2.3’s 139–145.

### Research Flags

Phases likely needing deeper research during planning:
- **Unlink (≈148):** Exact Fjall delete-batch atomicity, height-to-key candidate indexing (avoid full keyspace scan), crash seams after index clear / after unlink / after coins — Architecture confidence MEDIUM on these contracts.
- **Serving limits + labels (≈149):** Precise inventory/`managed_block_serve_input` fact wiring and Knots getdata disconnect vs silent ignore edge cases under limited-only advertisement.

Phases with standard patterns (skip research-phase unless surprises):
- **Snapshot cutover (≈146):** Clear debt; coins path already models leftover as non-authoritative; primarily retarget `WalletRescanRuntime` (+ RPC rescan helper).
- **Prune policy (≈147):** Deterministic Knots constants and `decide_flush`-shaped pure policy; well-anchored in pinned sources.
- **Operator evidence / no-claim (≈150):** Follows v2.2/v2.3 sanitized-evidence + claim-checker pattern; retarget Phase-145-style guards rather than inventing a new verification runtime.

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| Stack | HIGH | Pinned toolchain, Fjall, and first-party crate seams verified in-repo; explicit “no new crates/libs” |
| Features | HIGH | Knots prune/RPC/net/wallet sources + Open Bitcoin availability and wallet-rescan debt read in-tree |
| Architecture | HIGH / MEDIUM | HIGH on core/shell seams and Knots APIs; MEDIUM on Fjall batch atomicity and height-indexed candidates until a phase pins them |
| Pitfalls | HIGH | Mapped to live OB seams (`wallet_rescan`, inventory, flush lifecycle, ServiceFlags) and Knots predicates |

**Overall confidence:** HIGH

### Gaps to Address

- **Fjall unlink durability / batch atomicity:** Pin crash-recovery contracts during unlink-phase planning (not invent a second flusher).
- **Height-indexed prune candidates:** Avoid naïve full-keyscan on mainnet-scale stores; decide metadata shape when implementing unlink.
- **Manual prune + lock operator surface depth:** MVP may ship locks via adapters first; full `listprunelocks` / `setprunelock` can follow in v2.4.x.
- **Wallet rescan refusal past prune height:** Align Knots strings once have-pruned is live (adjacent to labels/operator work).

## Sources

### Primary (HIGH confidence)
- `rust-toolchain.toml`, `packages/*/Cargo.toml` — Rust `1.94.1`, Fjall `3.1.4`, serde pins
- Open Bitcoin: `fjall_store` blocks/coins, `flush_lifecycle.rs`, `inventory.rs`, `block_serving.rs`, `message.rs` (`ServiceFlags`), `wallet_rescan.rs`
- Pinned Knots `29.3.knots20260210`: `node/blockstorage.cpp/.h`, `validation.cpp/.h`, `net_processing.cpp`, `protocol.h`, `init.cpp`, `rpc/blockchain.cpp`, `node/chainstate.cpp`, `node/blockmanager_args.cpp`, wallet rescan refusal paths
- `.planning/PROJECT.md` v2.4 milestone constraints; `.planning/reports/NEXT-MILESTONE-CANDIDATES.md`

### Secondary (MEDIUM confidence)
- Exact Fjall multi-key delete crash stories and height→key grouping strategies — defer pin to unlink-phase planning
- Optional v2.4.x surfaces (`-pruneduringinit`, full prune-lock RPCs, mid-unlink restart cleanup)

### Tertiary (LOW confidence)
- None material for roadmap structure; anti-features (archive, assumeutxo, BIP37, LevelDB import, auto-reindex, production) stay explicitly out of scope

---
*Research completed: 2026-09-21*
*Milestone: v2.4 Prune-Mode Product Behavior*
*Ready for roadmap: yes*
