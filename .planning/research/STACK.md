# Stack Research

**Domain:** Bitcoin Knots-aligned prune-mode product behavior (height windows, payload unlink, `m_have_pruned`, prune locks, `NODE_NETWORK_LIMITED`)
**Researched:** 2026-09-21
**Confidence:** HIGH

## Recommendation

v2.4 should add **no new production crates and no new third-party libraries**. Prune-mode product behavior is a typed decision-and-effect extension of the shipped v2.3 coins, block-payload, and honest-availability stack.

Open Bitcoin already stores block bodies as Fjall `block:<64-hex>` keys (`FjallNodeStore::save_block` / `has_block` / `load_block`) and coins under `StorageNamespace::Coins`. Knots prune is height-window selection plus durable `m_have_pruned`, then unlink of `blk`/`rev` files. Map that contract onto the existing store: pure height-window / lock / have-pruned decisions in first-party crates; shell-owned Fjall key removal for selected block and undo payloads; `Pruned` only after a real delete sets `have_pruned`.

Do not invent a second block database, port `FlatFileSeq`, or pull LevelDB/RocksDB/rust-bitcoin into the production path. Keep a single active chainstate.

## Recommended Stack

### Core Technologies

| Technology | Version | Purpose | Why Recommended |
|------------|---------|---------|-----------------|
| Rust | `1.94.1`, edition 2024 | Pure prune-range, lock, flush-for-prune, and serving-window state machines | Already pinned by `rust-toolchain.toml` and `packages/Cargo.toml`. Enums can make prune-eligible vs forbidden, have-pruned, and serve-window outcomes explicit without I/O. |
| Open Bitcoin workspace crates | `0.1.0` | Own coins, block payloads, availability labels, service flags, wallet rescan | Preserves production-path ownership and functional-core / imperative-shell. Extend existing crates; do not add a prune crate. |
| Bitcoin Knots | `29.3.knots20260210` | External prune behavior contract | Vendored anchors under `packages/bitcoin-knots` are the parity roots. Do not invent newer Core/Knots prune semantics. |
| Fjall | `3.1.4`, existing, `default-features = false` | Durable delete of block/undo payloads and persist of `have_pruned` / prune-lock records | Already backs blocks, coins, chain meta, and runtime markers. `remove` / batch + `persist` cover Knots unlink + flag write without a second database. |
| serde / serde_json | `1.0.228` / `1.0.149`, existing | Versioned prune-lock and operator/evidence shapes | Keep for small metadata and evidence. Do not encode prune decisions as free-form JSON in the pure core. |

### First-Party Modules to Extend

| Crate / module | New responsibility | Integration point |
| --- | --- | --- |
| `open-bitcoin-chainstate` prune policy | Pure height window (`GetPruneRange` analogue), `MIN_BLOCKS_TO_KEEP = 288`, prune-after-height gate, prune-lock forbid check (`PRUNE_LOCK_BUFFER = 10`), and flush-for-prune decision input | Keep I/O-free. Inputs are injected tip height, target/manual height, lock ranges, usage vs prune target. Do not unlink here. Dual-chainstate / snapshot-base prune-start branches stay out of scope. |
| `open-bitcoin-chainstate` flush policy | Extend flush facts so a non-empty prune candidate set can force write (`fFlushForPrune`) before deletes | Mirror Knots `FlushStateToDisk`: find candidates → set `have_pruned` when first non-empty set → write index/coins → then unlink. Keep `FlushMode::{None,IfNeeded,Periodic,Always}`; model prune as an injected `flush_for_prune: bool` fact, not a new Tokio timer. |
| `open-bitcoin-network` `ServiceFlags` | Add `NETWORK_LIMITED = 1 << 10` (Knots `NODE_NETWORK_LIMITED`) | Existing hand-rolled `ServiceFlags` in `message.rs` already has `NETWORK` / `WITNESS`. Do not add `bitflags`. |
| `open-bitcoin-network` block serving | Pure serve-window gate: refuse/disconnect path below limited threshold; map `Pruned` only when injected `have_pruned && !payload_present` (plus known-tx fact when available) | Today `BlockServingDataAvailability::Pruned` is reserved and missing payload is `Unavailable`. Keep that honesty: missing without prune stays `Unavailable`. Knots serve refuse uses tip height − request height `> NODE_NETWORK_LIMITED_MIN_BLOCKS + 2` (288 + 2). |
| `open-bitcoin-node` `FjallNodeStore` | Shell unlink: delete selected `block:` keys and matching undo keys; persist `have_pruned` flag and non-temporary prune locks | Reuse `remove_bytes`, existing `has_block`, and `PersistMode`. Prefer height-selected per-hash deletes over inventing `blk?????.dat` files. Persist `have_pruned` like Knots `prunedblockfiles` flag (Runtime or BlockIndex metadata key). |
| `open-bitcoin-node` `ManagedChainstate` / flush lifecycle | Own the prune orchestration order: decide → mark dirty index / set have_pruned → flush → unlink | Single active chainstate only. Refuse “unprune without reindex” like Knots `node/chainstate.cpp` when `m_have_pruned && !options.prune`. Do not auto-reindex. |
| `open-bitcoin-node` inventory / block serve adapters | Inject `payload_present` from `has_block` and `have_pruned` from durable flag into network classifiers | `inventory.rs` already maps missing payload to `Unavailable`. Wire `Pruned` only after prune deleted payloads and set the flag. |
| `open-bitcoin-wallet` + `WalletRescanRuntime` | First-phase cut: stop rescanning from leftover `ChainstateSnapshot` UTXO bytes | `sync/wallet_rescan.rs` still calls `load_chainstate_snapshot` / `rescan_chainstate(&partial_snapshot)`. Point rescan at durable coins (or an explicit coins-derived view) so prune cannot resurrect snapshot-as-truth. |
| `open-bitcoin-rpc` / `open-bitcoin-cli` | Operator config and evidence for prune target / manual height / limited services; sanitized have-pruned and window facts | Reuse clap/jsonc/serde already present. Keep public prune-by-default and production claims out of scope. |

### Supporting Libraries and Standard-Library Approaches

| Library / facility | Version | Purpose | When to Use |
|--------------------|---------|---------|-------------|
| `std` collections (`BTreeMap` / `HashMap` / `BTreeSet`) | Rust `1.94.1` | Pure candidate sets, lock maps, height ranges | Prefer explicit typed structs (`PruneLockInfo`, `PruneRange`) over stringly maps in the core API. |
| Existing `PersistMode` | node crate | Durability of flag write and deletes | `Sync` for first `have_pruned` and non-temporary lock writes; match flush lifecycle for unlink batches. |
| Existing `FjallNodeStore::has_block` | Fjall `3.1.4` | Honest payload presence after prune | Presence remains `contains_key` on `block:<hex>`; deletes must make this false. |
| Existing `ServiceFlags` bit ops | network crate | Advertise and enforce limited serving | Add only `NETWORK_LIMITED`. Local pruned service advertisement should clear full `NETWORK` and keep limited + witness, matching Knots init defaults for pruned nodes. |
| Existing architecture / parity verifiers | repo scripts | Keep core I/O-free and anchors auditable | Cite Knots prune files in `docs/parity/` breadcrumbs; do not add a new verification runtime. |

### Development Tools

| Tool | Purpose | Notes |
|------|---------|-------|
| `bash scripts/verify.sh` | Repo-native format, lint, build, test, coverage, architecture, parity-breadcrumb, Bazel contract | Remains the deterministic default gate. Do not make public-network prune soak a release gate. |
| Existing Rust unit tests + temp Fjall stores | Height windows, lock buffer, have_pruned persistence, delete-then-Unavailable/Pruned labels, limited serve refuse | Inject tip height and lock tables. Assert missing-without-prune stays `Unavailable`. |
| Pinned Knots sources / tests | Parity cases | `validation.h` (`MIN_BLOCKS_TO_KEEP`), `blockstorage.cpp` (`FindFilesToPrune*`, `UnlinkPrunedFiles`, `IsBlockPruned`, `PRUNE_LOCK_BUFFER`), `net_processing.cpp` (`NODE_NETWORK_LIMITED_*`), `blockmanager_args.cpp` (`nPruneTarget`). |
| Existing `open-bitcoin-bench` | Guard accidental full-history rewrite on prune | Measure delete/flush of a bounded height window; do not add Criterion. |

## Installation

```bash
# No new crates. Use the existing workspace pin.
# Rust toolchain is already pinned:
#   rust-toolchain.toml → channel = "1.94.1"

# Materialize the behavioral baseline when needed:
git submodule update --init --recursive

# Repo-native verification (unchanged):
bash scripts/verify.sh
```

## Alternatives Considered

| Recommended | Alternative | When to Use Alternative |
|-------------|-------------|-------------------------|
| Fjall per-hash `block:` key delete | Port Knots `FlatFileSeq` (`blk?????.dat` / `rev?????.dat`) | Only if a later milestone deliberately adopts archive/flat-file storage. Not needed for Knots-aligned prune product behavior on the current store. |
| Height-window candidate selection in pure core | Byte-identical Knots file-number prune units | Use if Open Bitcoin later materializes contiguous block files. Today payloads are per-hash; height windows are the honest mapping. Document the granularity difference in `docs/parity/`. |
| Hand-rolled `ServiceFlags::NETWORK_LIMITED` | Add `bitflags` crate | Never for this milestone. Existing flags type already matches Knots bit positions for `NETWORK` / `WITNESS`. |
| Extend single `ManagedChainstate` | Dual / assumeutxo chainstate for prune ranges | Out of scope. Knots snapshot prune-start logic stays deferred with assumeutxo. |
| Durable `have_pruned` flag in Fjall | Infer pruned solely from missing payloads | Never. Knots `IsBlockPruned` requires `m_have_pruned && !HAVE_DATA && nTx > 0`. Missing without ever pruning must stay `Unavailable`. |

## What NOT to Use

| Avoid | Why | Use Instead |
|-------|-----|-------------|
| `rust-bitcoin` / other third-party Rust Bitcoin libs | Violates production-path ownership | First-party primitives, codec, network, chainstate |
| LevelDB / RocksDB / `rusty-leveldb` / `rocksdb` | Second database; splits crash ordering from Fjall coins/blocks | Existing Fjall `3.1.4` keyspaces |
| New block flat-file DB (`FlatFileSeq`, `blk*.dat`) | Duplicates the shipped Fjall block-payload store | Height-selected `block:` / undo key deletion |
| Second / snapshot chainstate | assumeutxo and dual-chainstate are out of scope | Single active chainstate manager from v2.3 |
| Automatic destructive `-reindex` | High-risk datadir mutation; deferred | Fail closed when `have_pruned` conflicts with unprune config; operator-driven recovery later |
| Tokio / filesystem crates in `open-bitcoin-chainstate` | Would break I/O-free core | Inject tip, locks, usage, and disk facts; shell executes deletes |
| Emitting `Pruned` for any missing payload | Label lie vs Knots `IsBlockPruned` | `Unavailable` until prune deleted files and set `have_pruned` |
| Leftover `ChainstateSnapshot` as wallet-rescan truth | Lets prune resurrect snapshot UTXOs after coins are authoritative | Coins-backed rescan view; treat leftover snapshot as non-authoritative |
| Public prune defaults / public-network CI as release gate | Outside milestone boundary | Opt-in operator config and deterministic `verify.sh` |
| Archive-node serving stack | Opposite product problem from prune | Keep archive deferred; honesty about deletes is not archive |

## Stack Patterns by Variant

**If implementing height windows and locks (core of prune):**
- Put pure `PruneRange`, `PruneLockInfo`, and forbid-check in `open-bitcoin-chainstate`.
- Because Knots policy is deterministic from tip height, locks, and targets; adapters only supply facts and execute deletes.

**If implementing payload unlink:**
- Delete Fjall block and undo keys for the selected height set after flush, then rely on `has_block == false`.
- Because Open Bitcoin’s durable body is the Fjall payload key, not a Knots flat file. Still set durable `have_pruned` on first successful prune set.

**If implementing `NODE_NETWORK_LIMITED`:**
- Extend `ServiceFlags` and the existing block-serve / inventory gates in `open-bitcoin-network` + node adapters.
- Because serving limits are peer-policy, not storage. Keep the 288-block window and +2 race buffer from pinned `net_processing.cpp`.

**If cutting wallet snapshot resurrection (first phase):**
- Retarget `WalletRescanRuntime` off `load_chainstate_snapshot`.
- Because leftover snapshot bytes must not become UTXO truth after prune deletes historical payloads.

## Version Compatibility

| Package A | Compatible With | Notes |
|-----------|-----------------|-------|
| Rust `1.94.1` | workspace edition 2024 | Do not bump for prune work. |
| Fjall `3.1.4` | `open-bitcoin-node` storage | Keep `default-features = false`. Delete path must use existing persist modes. |
| serde `1.0.228` / serde_json `1.0.149` | node / rpc / cli evidence | Metadata only; not coin or block truth. |
| Tokio `1.52.1` | `open-bitcoin-rpc` only | Do not add Tokio to chainstate or node for prune. |
| Bitcoin Knots `29.3.knots20260210` | all in-scope prune claims | Cite files below; do not chase newer upstream prune changes. |

## Knots Source Anchors (verified in-tree)

| Anchor | Path | What to mirror |
|--------|------|----------------|
| `MIN_BLOCKS_TO_KEEP = 288` | `packages/bitcoin-knots/src/validation.h` | Trailing height window that must not prune |
| `GetPruneRange` | `packages/bitcoin-knots/src/validation.cpp` | `[prune_start, prune_end]`; ignore dual-chainstate branch |
| `FindFilesToPrune` / `FindFilesToPruneManual` / `PruneOneBlockFile` / `UnlinkPrunedFiles` | `packages/bitcoin-knots/src/node/blockstorage.cpp` | Candidate selection, index clear, then unlink |
| `m_have_pruned` / `IsBlockPruned` / `prunedblockfiles` | `packages/bitcoin-knots/src/node/blockstorage.h`, `blockstorage.cpp` | Durable ever-pruned flag; pruned iff have_pruned ∧ !HAVE_DATA ∧ nTx > 0 |
| `PRUNE_LOCK_BUFFER = 10`, `PruneLockInfo`, `UpdatePruneLock` | `packages/bitcoin-knots/src/node/blockstorage.cpp`, `blockstorage.h` | Named height locks with buffer |
| `fFlushForPrune` ordering | `packages/bitcoin-knots/src/validation.cpp` (`FlushStateToDisk`) | Decide → set flag → write index → unlink → coins write |
| `nPruneTarget` / manual `prune=1` | `packages/bitcoin-knots/src/node/blockmanager_args.cpp` | MiB target vs `PRUNE_TARGET_MANUAL`; min `MIN_DISK_SPACE_FOR_BLOCK_FILES` (550 MiB) |
| Refuse unprune without rebuild | `packages/bitcoin-knots/src/node/chainstate.cpp` | `m_have_pruned && !options.prune` fails closed |
| `NODE_NETWORK_LIMITED` bit and serve window | `packages/bitcoin-knots/src/protocol.h`, `net_processing.cpp` | `1 << 10`; min 288 blocks; refuse below window with +2 buffer |

## Sources

- `rust-toolchain.toml` — Rust `1.94.1` (HIGH)
- `packages/open-bitcoin-node/Cargo.toml` — Fjall `3.1.4`, serde `1.0.228`, serde_json `1.0.149`, getrandom `0.3.4`, fs4 `1.1.0` (HIGH)
- `packages/open-bitcoin-rpc/Cargo.toml` — Tokio `1.52.1` shell-only (HIGH)
- `packages/open-bitcoin-node/src/storage/fjall_store.rs`, `fjall_store/blocks.rs`, `fjall_store/coins.rs` — block keys, `has_block`, undo keys, leftover snapshot handling (HIGH)
- `packages/open-bitcoin-network/src/block_serving.rs`, `message.rs` — reserved `Pruned`, hand-rolled `ServiceFlags` (HIGH)
- `packages/open-bitcoin-node/src/network/inventory.rs`, `sync/wallet_rescan.rs` — Unavailable mapping; snapshot-backed rescan seam (HIGH)
- Pinned Knots files listed above under `packages/bitcoin-knots` (HIGH)

---
*Stack research for: Open Bitcoin v2.4 prune-mode product behavior*
*Researched: 2026-09-21*
