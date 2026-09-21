# Phase 146: Wallet Leftover-Snapshot Cutover - Research

**Researched:** 2026-09-21
**Domain:** Wallet rescan authority cutover (durable coins + payload-present blocks vs leftover Fjall `Chainstate` `"snapshot"` blob)
**Confidence:** HIGH

## Summary

Phase 146 is a correctness cutover, not prune product behavior. After Phases 141–142, durable coins (`C`/`B`/`H`) are live UTXO truth for chainstate reopen, and `persist_progress` no longer writes the leftover snapshot. Wallet rescan is the remaining consumer that still treats `FjallNodeStore::load_chainstate_snapshot()` as scan truth via `WalletRescanRuntime::required_chainstate_snapshot`. SNAP-01 closes that seam so a later unlink (Phase 148) cannot resurrect snapshot UTXOs as wallet balances.

The planning-critical implementation shape is already in-tree: `hydrate_chainstate_for_open` builds a `ChainstateSnapshot` from scanned coin records, `chain_meta`, and undo keys without reading leftover UTXOs once coins exist. Keep the chunked `WalletRescanJob` protocol and the pure `open-bitcoin-wallet::rescan_chainstate(&ChainstateSnapshot)` signature; replace only the shell assembly and add a payload-presence fail-closed gate with `has_block`. Do not delete the leftover blob, do not restore snapshot writes, and do not invent `have_pruned` / `Pruned`.

**Primary recommendation:** In `open-bitcoin-node` only, replace `required_chainstate_snapshot` with a coins + `chain_meta` + `has_block` assembly path (reuse hydrate/scan helpers), keep feeding assembled `ChainstateSnapshot` into the existing wallet scan; add a same-datadir reopen fixture where leftover tip/UTXOs disagree with coins; also cut or route the durable RPC `rescanblockchain` path that still seeds memory chainstate from leftover snapshot bytes.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

### Wallet scan authority

- **D-01:** Wallet rescan authority is the durable coins view (coins
  best-block plus the coins UTXO set) together with payload-present block
  bodies. A leftover chainstate `"snapshot"` blob must not supply tip,
  UTXOs, undo, balances, or history on the wallet path.
- **D-02:** The cutover lives in the node shell. `open-bitcoin-wallet`
  stays I/O-free and keeps consuming an assembled scan view. It must not
  open Fjall or read `StorageNamespace::Chainstate` key `"snapshot"`.
  `WalletRescanRuntime` stops using `load_chainstate_snapshot()` as scan
  truth.
- **D-03:** Same-datadir reopen with a planted leftover snapshot that
  disagrees with durable coins must not change wallet balances or history.
  The rescan job protocol stays: bounded height chunks, persisted rescan
  jobs, and checkpointed progress. Only the chain input source changes.
- **D-04:** A wallet UTXO or history entry is admissible only when its
  creating block's payload bytes are present. Coins best-block alone does
  not authorize an entry whose creating payload is missing.

### Leftover blob disposition

- **D-05:** Leave the leftover `"snapshot"` blob on disk. This phase does
  not delete it. Deletion is not required for SNAP-01 and must not be
  presented as prune.
- **D-06:** Do not restore leftover snapshot writes. `persist_progress`
  already does not call `save_chainstate_snapshot`. Schema-1 one-way
  migration may still read the blob once to seed coins (Phase 141). After
  coins exist, that read must not feed wallet rescan.
- **D-07:** `ChainstateSnapshot` may remain a pure hydrate, export, and
  test helper. Wallet rescan must not treat a helper built from the
  leftover blob as live chain truth.

### Missing payload

- **D-08:** If a height inside the current rescan chunk lacks payload
  bytes, fail that chunk closed. Do not skip the height, do not invent
  transactions, and do not fall back to leftover snapshot UTXOs or undo.
- **D-09:** A missing payload stays `Unavailable` under the Phase 143
  rule. This phase does not relabel it `Pruned`.

### Have-pruned and unlink boundary

- **D-10:** Do not delete block or undo payloads. Do not set have-pruned.
  Do not emit `Pruned` or `block_status_pruned` on production paths.
  Contributors must be able to observe that payloads remain and
  have-pruned was not invented by cutover alone.
- **D-11:** Prune mode, the 550 MiB target, the 288-block keep window, the
  10-block lock buffer, Fjall key deletion, `NODE_NETWORK_LIMITED`, and
  operator prune surfaces stay in Phases 147–151.

### Operator and parity surface

- **D-12:** Do not add prune RPC, CLI, or dashboard fields. Existing
  wallet rescan status stays. Phase 150 owns prune operator evidence.
- **D-13:** Touched first-party Rust source and tests under
  `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests` get
  parity breadcrumbs through `docs/parity/source-breadcrumbs.json`, using
  explicit `none` only when no defensible Knots anchor exists. Full v2.4
  prune parity roots and the no-claim checker stay in Phase 151 (GRD-01).
- **D-14:** Verification remains `bash scripts/verify.sh`. Default
  verification stays deterministic and public-network-free. Historical
  `.planning/phases/` directories stay tracked.

### Claude's Discretion

- Whether the shell assembles the existing `ChainstateSnapshot` from
  coins plus payload-present blocks, or introduces a narrower scan-input
  type, as long as D-01 through D-04 hold and the wallet crate stays
  I/O-free.
- Which existing block-store and coins-view methods supply best-block and
  payload-present bodies.
- Fixture style for a planted leftover snapshot that disagrees with coins,
  as long as reopen rescan ignores that blob for balances and history.

### Deferred Ideas (OUT OF SCOPE)

- Pure prune policy, 550 MiB target, 288-block keep, and 10-block lock
  buffer — Phase 147
- Fjall block and undo key deletion, have-pruned, and interrupted-prune
  recovery — Phase 148
- `NODE_NETWORK_LIMITED` and honest `Pruned` versus `Unavailable` labels
  — Phase 149
- Prune RPC, CLI, dashboard, locks, and support evidence — Phase 150
- v2.4 prune parity roots and no-claim checker — Phase 151
- Deleting leftover `"snapshot"` blobs
- Temporary IBD prune target (`-pruneduringinit`, FUT-27)
- assumeutxo, archive serving, BIP37, public defaults, and
  production-funds wallet claims
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| SNAP-01 | Wallet rescan reads durable coins and payload-present blocks, and does not treat leftover snapshot bytes as chain truth. | Cut `WalletRescanRuntime` off `load_chainstate_snapshot`; assemble scan view from coins/`chain_meta` + `has_block` gate; prove disagreeing leftover cannot change balances/tip; keep blob on disk; no `have_pruned`/`Pruned`. Also address durable RPC rescan seed path (see Open Questions). |
</phase_requirements>

## Project Constraints (from .cursor/rules/)

No `.cursor/rules/` directory exists in this repo. [VERIFIED: glob `.cursor/rules/**/*`]

Planner must still honor repo-local guidance from `AGENTS.md` / `AGENTS.bright-builds.md` / `standards/core/architecture.md` / `standards/core/testing.md`:

- Functional core stays I/O-free; Fjall lives in the node shell. [CITED: AGENTS.md; standards/core/architecture.md]
- No rust-bitcoin on the production path. [CITED: AGENTS.md]
- Touched first-party Rust under `packages/open-bitcoin-*/src` or `tests` needs parity breadcrumbs. [CITED: AGENTS.md Repo-Local Guidance]
- Verification contract is `bash scripts/verify.sh`. [CITED: AGENTS.md]

## Standard Stack

### Core

| Library / surface | Version | Purpose | Why Standard |
|-------------------|---------|---------|--------------|
| Rust | `1.94.1` | Language / toolchain | Pinned by `rust-toolchain.toml` and workspace. [VERIFIED: rustc --version; rust-toolchain.toml] |
| `open-bitcoin-node` `FjallNodeStore` | first-party | Coins scan, `chain_meta`, leftover load APIs, `has_block` | Only shell allowed to touch Fjall for this cutover. [VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store/*.rs] |
| Fjall | `3.1.4` (`default-features = false`) | Durable keyspaces | Existing store; no new DB. [VERIFIED: packages/open-bitcoin-node/Cargo.toml] |
| `open-bitcoin-wallet` `rescan_chainstate` | first-party | Pure UTXO/tip rebuild from `ChainstateSnapshot` | Keep I/O-free; do not open Fjall. [VERIFIED: packages/open-bitcoin-wallet/src/wallet/scan.rs] |
| `WalletRescanRuntime` | first-party | Chunked jobs, resume, checkpoints | Replace input source only. [VERIFIED: packages/open-bitcoin-node/src/sync/wallet_rescan.rs] |
| `ChainstateSnapshot` | first-party | Assembled scan / hydrate DTO | Remains helper; must not be leftover-backed on wallet path. [VERIFIED: packages/open-bitcoin-chainstate/src/types.rs] |

### Supporting

| Library / surface | Version | Purpose | When to Use |
|-------------------|---------|---------|-------------|
| `FjallCoinsView::best_block` | first-party | Coins tip (`B`) | Target tip for enqueue / target_tip_height. [VERIFIED: packages/open-bitcoin-node/src/storage/coins_view.rs] |
| `hydrate_chainstate_for_open` / `scan_coin_records` | first-party | Coins-backed snapshot assembly | Preferred reuse for wallet scan input. [VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store/coins.rs] |
| `FjallNodeStore::has_block` | first-party | Payload-present probe | Fail-closed chunk gate (D-04/D-08). [VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store/blocks.rs] |
| `seed_coins_from_leftover_for_reopen` | first-party test helper | Plant coins from leftover in fixtures | Test setup only; not production wallet truth. [VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store/coins.rs] |
| Existing wallet rescan status / job types | first-party | Operator-visible freshness | Do not add prune fields (D-12). [VERIFIED: packages/open-bitcoin-rpc/src/context/rescan.rs] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Assemble existing `ChainstateSnapshot` in shell | New narrower wallet scan-input type | Narrower type can encode payload-present invariants, but forces wallet API churn; discretion allows either — **prefer keep `ChainstateSnapshot`** for SNAP-01 speed. [ASSUMED: lower churn is preferred given D-07] |
| Reuse `hydrate_chainstate_for_open` | Hand-roll parallel coins+meta loader inside `wallet_rescan.rs` | Duplicate scan logic; risk leftover meta fallback diverges. Prefer shared helper with explicit no-leftover-for-wallet contract. |
| Fix only `WalletRescanRuntime` | Also cut durable RPC leftover seed | RPC `rescanblockchain` still seeds from leftover today; SNAP-01 wording covers wallet rescan broadly. Include RPC cutover or routing. [VERIFIED: packages/open-bitcoin-rpc/src/context/network.rs] |

**Installation:**

```bash
# No new crates. Existing pins only.
# Rust: rust-toolchain.toml → 1.94.1
# Fjall: packages/open-bitcoin-node/Cargo.toml → 3.1.4

bash scripts/verify.sh
```

**Version verification:** Toolchain and Fjall versions confirmed from local files and `rustc --version` on 2026-09-21. No npm packages for this phase.

## Architecture Patterns

### Recommended Project Structure

```
packages/open-bitcoin-node/src/
├── storage/fjall_store/coins.rs     # reuse hydrate / scan / leftover presence
├── storage/fjall_store/blocks.rs    # has_block payload probe
├── sync/wallet_rescan.rs            # cutover: replace required_chainstate_snapshot
└── sync/tests/wallet_rescan_runtime.rs  # disagreeing leftover + payload-missing cases

packages/open-bitcoin-wallet/src/wallet/scan.rs
└── keep pure rescan_chainstate(&ChainstateSnapshot) — no Fjall

packages/open-bitcoin-rpc/src/context/
├── network.rs   # durable init still loads leftover into MemoryChainstateStore
└── rescan.rs    # rescanblockchain uses blockchain_snapshot() — must not remain leftover-backed
```

### Pattern 1: Shell-assembled scan view (recommended)

**What:** Node shell builds `ChainstateSnapshot` from durable coins + `chain_meta` (+ optional undo), gates chunk heights with `has_block`, then calls pure `wallet.rescan_chainstate`.
**When to use:** Always for SNAP-01; matches D-02/D-07 discretion preference for keeping wallet signature.
**Example:**

```rust
// Source: packages/open-bitcoin-node/src/sync/wallet_rescan.rs (current anti-pattern)
// Replace required_chainstate_snapshot body — do NOT call load_chainstate_snapshot.

fn required_wallet_scan_snapshot(
    &self,
) -> Result<ChainstateSnapshot, WalletRegistryError> {
    // Prefer coins-backed hydrate (UTXOs from C keys, not leftover).
    // Align tip with coins best-block + chain_meta positions.
    // Caller then applies has_block gate for [next_height, chunk_end].
    self.store
        .hydrate_chainstate_for_open()?
        .ok_or(/* UnavailableNamespace::Chainstate */ ...)
}
```

[VERIFIED: packages/open-bitcoin-node/src/sync/wallet_rescan.rs; packages/open-bitcoin-node/src/storage/fjall_store/coins.rs]

### Pattern 2: Payload-present fail-closed chunk gate

**What:** Before `rescan_chainstate`, for every `active_chain` position with `height` in the current chunk range, require `store.has_block(hash)? == true`. On miss: `job.mark_failed(...)`, persist job, return error. Do not call leftover load as fallback.
**When to use:** Every `advance_wallet_rescan` chunk (D-04, D-08, D-09).
**Example:**

```rust
// Source: packages/open-bitcoin-node/src/storage/fjall_store/blocks.rs
pub fn has_block(&self, block_hash: BlockHash) -> Result<bool, StorageError> {
    self.block_index
        .contains_key(super::block_key(block_hash))
        .map_err(|error| super::backend_failure(StorageNamespace::BlockIndex, error))
}
```

[VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store/blocks.rs]

### Pattern 3: Same-datadir disagreeing leftover proof

**What:** Plant durable coins tip/UTXOs A; overwrite leftover `"snapshot"` with tip/UTXOs B that disagree; reopen store; run `WalletRescanRuntime` rescan; assert wallet tip/UTXO set matches A; assert leftover key still present; assert payloads still `has_block`; assert no `have_pruned` / `Pruned` symbols introduced on production paths.
**When to use:** Required success criteria 2 and 3.
**Fixture reuse:** Extend `wallet_rescan_runtime.rs` patterns and the overwrite technique from `schema_two_leftover_utxos_do_not_hydrate_after_coins_exist`. [VERIFIED: packages/open-bitcoin-node/src/sync/tests/wallet_rescan_runtime.rs; packages/open-bitcoin-node/src/storage/fjall_store/tests/coins_migration.rs]

### Anti-Patterns to Avoid

- **Calling `load_chainstate_snapshot` on the wallet path:** Direct SNAP-01 regression. [VERIFIED: packages/open-bitcoin-node/src/sync/wallet_rescan.rs]
- **Deleting leftover `"snapshot"` and calling it prune:** Violates D-05 / deferred list.
- **Emitting `Pruned` or inventing `have_pruned`:** Phases 148–149; missing payload stays Unavailable. [VERIFIED: packages/open-bitcoin-node/src/network/tests/block_serving.rs inventory must not inject Pruned]
- **Moving Fjall into `open-bitcoin-wallet`:** Violates functional-core boundary. [CITED: standards/core/architecture.md]
- **Restoring `save_chainstate_snapshot` in `persist_progress`:** Violates D-06; Phase 142 already cut writes. [CITED: 146-CONTEXT.md; VERIFIED via prior phase docs]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Coins UTXO enumeration | New wallet-side DB walk | `scan_coin_records` / `hydrate_chainstate_for_open` | Already ignores leftover UTXOs when coins exist. [VERIFIED: coins.rs] |
| Payload presence | Decode via `load_block` for every gate | `has_block` (`contains_key`) | Phase 143 honesty; cheaper and matches serve probe. [VERIFIED: blocks.rs; block_presence tests] |
| Chunk scheduling | New rescan job protocol | Existing `WalletRescanJob` + `chunk_end_height` | Locked D-03. |
| Wallet UTXO matching | Rewrite wallet scan | Existing `rescan_chainstate` | Pure and tested. [VERIFIED: scan.rs] |
| Prune deletes / have-pruned | Any unlink | Defer to Phase 148 | Locked D-10/D-11. |

**Key insight:** The hard part is authority wiring, not inventing a new wallet scanner. Coins hydrate already answers “what are live UTXOs?”; this phase stops asking the leftover blob the same question on the wallet path and adds payload admission.

## Runtime State Inventory

This phase is an authority cutover over durable Fjall state (leftover blob remains).

| Category | Items Found | Action Required |
|----------|-------------|------------------|
| Stored data | Leftover `StorageNamespace::Chainstate` key `"snapshot"` may remain on operator datadirs after Phase 141 migration. Coins `C`/`B`/`H`, `chain_meta`, `undo:*`, and `block:<hex>` payloads are live truth. [VERIFIED: fjall_store.rs; coins.rs] | **Code edit only** on wallet/RPC read paths: stop treating leftover as scan truth. **No data migration / no blob delete** (D-05). |
| Live service config | None — verified no external service stores this authority outside the datadir. [ASSUMED: local node datadir only] | none |
| OS-registered state | None — verified no launchd/systemd unit embeds leftover snapshot authority in-repo. | none |
| Secrets/env vars | None — no env keys named for leftover snapshot. | none |
| Build artifacts | Rebuilt `open-bitcoin-node` / `open-bitcoind` binaries pick up code cutover; no separate egg/npm install. | Rebuild via normal Cargo/Bazel verify |

## Common Pitfalls

### Pitfall 1: Fixing WalletRescanRuntime but leaving RPC leftover-seeded

**What goes wrong:** Durable `ManagedRpcContext` still calls `load_chainstate_snapshot_with_confirmation_migration` and builds `MemoryChainstateStore::from_snapshot`; `rescanblockchain` then rescans leftover-backed tip/UTXOs. [VERIFIED: packages/open-bitcoin-rpc/src/context/network.rs:119-126; rescan.rs]
**Why it happens:** CONTEXT claims RPC flows through `WalletRescanRuntime`, but production RPC does not call it today. [VERIFIED: rg WalletRescanRuntime under open-bitcoin-rpc — only job status reads]
**How to avoid:** Plan an explicit task for durable RPC wallet rescan: either route through `WalletRescanRuntime` or assemble coins-backed snapshot for `rescan_wallet_range` / durable init.
**Warning signs:** Tests that only cover `WalletRescanRuntime` while RPC fixtures still `save_chainstate_snapshot` as chain truth.

### Pitfall 2: Using leftover `active_chain` meta as tip after coins exist

**What goes wrong:** `load_hydrate_chain_meta` falls back to leftover `active_chain` when `chain_meta` is missing. [VERIFIED: coins.rs `load_hydrate_chain_meta`]
**Why it happens:** Hydrate helper was written for open/migration compatibility.
**How to avoid:** Wallet assembly must prefer coins `best_block` + present `chain_meta`. If meta missing after coins exist, fail closed rather than read leftover tip for wallet.
**Warning signs:** Disagreeing leftover tip still becomes `wallet.maybe_tip_height`.

### Pitfall 3: Silencing missing payloads instead of failing the chunk

**What goes wrong:** Filtering UTXOs or skipping heights when `has_block` is false, leaving a “successful” partial scan that looks Fresh.
**Why it happens:** Wallet scan currently never looks at payloads; easiest “fix” is filter.
**How to avoid:** Follow D-08: `mark_failed`, persist, return error; no leftover fallback.
**Warning signs:** Job state `Complete` while a height in range lacks payload.

### Pitfall 4: Inventing prune semantics during cutover

**What goes wrong:** Deleting payloads, setting have-pruned, or labeling Unavailable as Pruned “to prepare for prune.”
**Why it happens:** Milestone theme is prune; easy scope creep.
**How to avoid:** Assert payloads remain and production paths still reserve `Pruned`. [VERIFIED: no `have_pruned` symbol in packages today via rg]
**Warning signs:** New prune RPC fields or `block_status_pruned` on evidence.

### Pitfall 5: Stale tests that only plant leftover snapshot

**What goes wrong:** `wallet_rescan_runtime` currently saves leftover then `seed_coins_from_leftover_for_reopen`. After cutover, tests that forget coins plant will fail UnavailableNamespace; tests that keep reading leftover will falsely pass SNAP-01.
**How to avoid:** Update fixtures to plant coins/`chain_meta`/blocks explicitly; use leftover only as the disagreeing poison blob.
**Warning signs:** Test still asserts via `load_chainstate_snapshot` equality for wallet balances.

## Code Examples

### Current wallet authority seam (must change)

```rust
// Source: packages/open-bitcoin-node/src/sync/wallet_rescan.rs
fn required_chainstate_snapshot(
    &self,
) -> Result<open_bitcoin_core::chainstate::ChainstateSnapshot, WalletRegistryError> {
    self.store.load_chainstate_snapshot()?.ok_or({
        WalletRegistryError::Storage(StorageError::UnavailableNamespace {
            namespace: StorageNamespace::Chainstate,
        })
    })
}
```

[VERIFIED: packages/open-bitcoin-node/src/sync/wallet_rescan.rs]

### Coins-backed hydrate already ignores leftover UTXOs

```rust
// Source: packages/open-bitcoin-node/src/storage/fjall_store/coins.rs
let utxos = self.scan_coin_records()?;
let undo_by_block = self.load_all_undo_records()?;
let (active_chain, maybe_confirmed_txid_counts) = self.load_hydrate_chain_meta()?;
let mut snapshot = ChainstateSnapshot::new(active_chain, utxos, undo_by_block);
```

[VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store/coins.rs]

### Pure wallet scan still only needs assembled utxos + tip

```rust
// Source: packages/open-bitcoin-wallet/src/wallet/scan.rs
wallet.utxos = utxos;
wallet.maybe_tip_height = snapshot.tip().map(|tip| tip.height);
wallet.maybe_tip_median_time_past = snapshot.tip().map(|tip| tip.median_time_past);
```

[VERIFIED: packages/open-bitcoin-wallet/src/wallet/scan.rs]

Note: There is no separate wallet transaction-history store today; “history” in success criteria means tip/UTXO-derived wallet state (balances and scanned tip), not BIP32 tx ledger reconstruction. [VERIFIED: rg history under open-bitcoin-wallet/src — no matches]

### Durable RPC leftover seed (second wallet-path consumer)

```rust
// Source: packages/open-bitcoin-rpc/src/context/network.rs
let durable_chainstate = match effective_store.as_ref() {
    Some(store) => store.load_chainstate_snapshot_with_confirmation_migration()?,
    None => None,
};
let chainstate_store = durable_chainstate.map_or_else(
    MemoryChainstateStore::default,
    MemoryChainstateStore::from_snapshot,
);
```

[VERIFIED: packages/open-bitcoin-rpc/src/context/network.rs]

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Leftover snapshot = live UTXO + wallet scan truth | Coins = live UTXO truth; leftover non-authoritative for reopen | Phases 141–142 | Wallet rescan still on old path — Phase 146 closes it |
| `persist_progress` writes leftover snapshot | Headers/runtime only; no `save_chainstate_snapshot` | Phase 142 | Blob may remain unread |
| Missing payload labeled flexibly | Missing without prune = `Unavailable`; `Pruned` reserved | Phase 143 | Phase 146 must keep that honesty |
| Wallet scan from leftover bytes | Wallet scan from coins + payload-present gate | Phase 146 (this) | Enables safe later unlink |

**Deprecated/outdated:**

- Treating `load_chainstate_snapshot()` as wallet/chain tip authority after coins exist. [CITED: .planning/research/FEATURES.md; 141/142 CONTEXT]

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | Prefer keeping `ChainstateSnapshot` wallet API over a narrower scan-input type for SNAP-01. | Standard Stack / Discretion | Minor API churn if planner chooses narrower type instead. |
| A2 | No live external service config embeds leftover snapshot authority outside the datadir. | Runtime State Inventory | If an out-of-repo operator harness caches leftover snapshots, docs would need a note (still no blob delete). |
| A3 | “History” in success criteria means wallet tip/UTXO state, not a separate tx-history DB. | Code Examples | If a hidden history store exists outside wallet crate naming, tests must cover it too — none found in-repo. |

## Open Questions

1. **Does SNAP-01 require cutting durable RPC `rescanblockchain` in the same phase?**
   - What we know: Locked text names `WalletRescanRuntime`; CONTEXT integration notes claim RPC flows through it, but code does not. RPC durable init still loads leftover. [VERIFIED]
   - What's unclear: Whether planner treats RPC as mandatory SNAP-01 scope or a follow-on micro-task inside 146.
   - Recommendation: **Include it in Phase 146** — otherwise operator-facing wallet rescan still violates SNAP-01 wording. Prefer routing durable named-wallet rescan through `WalletRescanRuntime` after that runtime is coins-backed, or change durable RPC snapshot seed to `hydrate_chainstate_for_open`.

2. **Should wallet assembly refuse leftover `chain_meta` fallback entirely?**
   - What we know: `load_hydrate_chain_meta` can read leftover `active_chain` when `chain_meta` missing. [VERIFIED]
   - What's unclear: Whether any supported datadir can have coins without `chain_meta` after Phase 141.
   - Recommendation: On the wallet path, require `chain_meta` (or derive positions only from coins B + headers) and fail closed — do not use leftover meta for tip.

3. **How strictly must payload bytes be “used” vs merely present?**
   - What we know: Current `rescan_chainstate` does not decode block bodies; D-04/D-08 require creating-block payload present. [VERIFIED]
   - What's unclear: Whether planners expect scanning scripts from block bodies in this phase.
   - Recommendation: **Presence gate only** for SNAP-01; do not invent block-body wallet history scanning here (out of scope / not present today).

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| Rust / Cargo | Build + tests | ✓ | 1.94.1 | — |
| Bun | Repo scripts / timings | ✓ | 1.4.2 | — |
| Bazelisk | Verify Bazel smoke | ✓ | present (`/opt/homebrew/bin/bazelisk`) | — |
| Fjall (crate) | Storage | ✓ | 3.1.4 (Cargo.toml) | — |
| Bitcoin Knots submodule | Optional parity citation | not probed this session | — | Use existing breadcrumb anchors / `none` if no defensible wallet-rescan prune-refusal anchor needed until Phase 151 |

**Missing dependencies with no fallback:** None for planned code/config work.

**Missing dependencies with fallback:** Knots submodule content not required to plan; cite existing in-tree breadcrumb style.

Step 2.6: External tools required for verify are present on this machine.

## Security Domain

`workflow.security_enforcement` is unset in `.planning/config.json` (treated as enabled). [VERIFIED: .planning/config.json]

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V2 Authentication | no | N/A for this cutover |
| V3 Session Management | no | N/A |
| V4 Access Control | no | No new operator privilege surfaces (D-12) |
| V5 Input Validation | yes | Fail closed on missing coins tip / missing chunk payloads; do not accept leftover as alternate truth |
| V6 Cryptography | no | No new crypto; do not add rust-bitcoin |

### Known Threat Patterns for leftover-snapshot wallet cutover

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Stale/forged leftover UTXOs resurrected as balances | Tampering / Elevation | Ignore leftover on wallet path; coins + payload gate only |
| Silent skip of missing history after future prune | Information disclosure / Tampering | Fail chunk closed now; later Knots-like refuse past prune height (post-148) |
| Labeling missing data as pruned without delete | Spoofing | Keep Unavailable; no have-pruned (D-09/D-10) |

## Sources

### Primary (HIGH confidence)

- `.planning/phases/146-wallet-leftover-snapshot-cutover/146-CONTEXT.md` — locked decisions
- `.planning/REQUIREMENTS.md` — SNAP-01
- `.planning/ROADMAP.md` — Phase 146 success criteria
- `packages/open-bitcoin-node/src/sync/wallet_rescan.rs` — leftover load seam
- `packages/open-bitcoin-node/src/storage/fjall_store/coins.rs` — hydrate / leftover non-authority
- `packages/open-bitcoin-node/src/storage/fjall_store/blocks.rs` — `has_block`
- `packages/open-bitcoin-wallet/src/wallet/scan.rs` — pure rescan
- `packages/open-bitcoin-rpc/src/context/network.rs` — durable leftover seed
- `AGENTS.md`, `AGENTS.bright-builds.md`, `standards/core/architecture.md`, `standards/core/testing.md`
- `.planning/research/STACK.md` / `FEATURES.md` — milestone research framing for wallet cutover-first

### Secondary (MEDIUM confidence)

- Phase 141/142/143 CONTEXT and verification docs — prior leftover / Unavailable rules
- `.planning/milestones/v2.3-MILESTONE-AUDIT.md` — residual wallet leftover consumer note

### Tertiary (LOW confidence)

- Exact Knots wallet “Can't rescan beyond pruned data” wording — not re-verified against submodule this session; cite in Phase 151, not required to implement SNAP-01 presence gate.

## Metadata

**Confidence breakdown:**

- Standard stack: HIGH — existing first-party crates and pinned Rust/Fjall; no new libraries
- Architecture: HIGH — seams located and prior hydrate path verified in source
- Pitfalls: HIGH — RPC dual path and leftover meta fallback confirmed in code

**Research date:** 2026-09-21
**Valid until:** 2026-10-21 (stable internal cutover; re-check if Phase 147+ lands first)
