# Phase 149: Limited Serving and Honest Pruned Labels - Research

**Researched:** 2026-09-27
**Domain:** Prune-mode service advertisement, limited block-body serving, honest `Pruned` versus `Unavailable`
**Confidence:** HIGH

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

### Service advertisement

- **D-01:** Prune mode means `PruneMode` is manual-only or automatic. Disabled
  stays the current full-history advertisement. Advertisement follows the
  configured mode at startup, not the first successful delete. An empty
  prune plan still advertises limited service.
- **D-02:** In prune mode, local services are `NETWORK_LIMITED | WITNESS`
  and must not include `NETWORK`. `NETWORK_LIMITED` is service bit 10
  (`1 << 10`), added on the existing `ServiceFlags` newtype. Do not add
  the `bitflags` crate, bloom, compact-filter, or other new service bits.
- **D-03:** When prune is disabled, keep today's `NETWORK | WITNESS`
  default. Do not add `NETWORK_LIMITED` to the non-prune default in this
  phase. Knots full nodes also set the limited bit; that co-advertisement
  is a Phase 151 parity note, not a Phase 149 behavior change.
- **D-04:** Only our advertised services change. Do not build a new
  historical-download or peer-selection policy for remote limited peers.

### Limited serve window

- **D-05:** The limited window reuses `MIN_BLOCKS_TO_KEEP` (288). A block
  request is outside the window when its tip distance is greater than
  `288 + 2`. The `+2` is the Knots `net_processing` race buffer from
  milestone research, not a second keep window for deletion. Phase 147's
  prune keep window stays exactly 288.
- **D-06:** The window gate applies only while prune mode is on. Disabled
  prune still serves any block whose payload is present, including blocks
  older than 288.
- **D-07:** Outside the window, do not send block bytes even when the
  payload is still on disk. That is the BIP 159 fingerprinting rule:
  peers must not learn the real prune depth. Inside the window, serve
  only when the existing payload-present fact is true.
- **D-08:** A request whose payload prune already removed is not served,
  whether or not it also falls outside the window. Do not reconstruct
  the block from undo, coins, or leftover snapshot bytes.

### Honest Pruned versus Unavailable

- **D-09:** Emit `Pruned` / `BlockServingDataAvailability::Pruned` only
  when the durable have-pruned flag is set and the payload is absent.
  Have-pruned is the Phase 148 flag, never inferred from prune config or
  from "bytes missing".
- **D-10:** A missing payload with have-pruned false stays `Unavailable`,
  including when prune mode is configured but no delete has committed.
- **D-11:** Payload present stays `Available`, even if have-pruned is
  already true for older heights. `Pruned` is not a chain-wide status.
- **D-12:** `Pruned` applies to index-known or active-chain blocks that
  would otherwise be a missing-payload result. An unknown hash stays the
  existing not-found path. Do not relabel `Unknown`, `Stale`, `SideChain`,
  or `Unvalidated` as `Pruned`.
- **D-13:** Update the reserved-label comments on
  `BlockServingDataAvailability::Pruned` and
  `BlockServingStatusLabel::Pruned` so they describe this earned label.
  Classifier tests may keep injecting `Pruned` directly. Production code
  may inject it only under D-09.

### Wire refusal

- **D-14:** Out-of-window and pruned-payload requests do not get a new
  wire message. Reuse the Phase 143 missing-inventory / `NotFound` refusal
  for "not served".
- **D-15:** An ordinary peer that requests a block body outside the limited
  window is disconnected, matching Knots historical-block serving. A peer
  that already has the existing download-serving permission is refused
  without that disconnect. Do not invent a new permission flag. Map the
  exemption onto `PermissionEffectLabel::DownloadServingPolicyInput` if
  that is the live download permission; otherwise the closest existing
  download permission, recorded in research.
- **D-16:** In-window missing payload does not disconnect by itself. It
  stays the Phase 143 unavailable or, when D-09 holds, pruned refusal.

### What the window covers

- **D-17:** The window and pruned-payload checks cover block-body
  `getdata` and compact-block / block-txn body serving. Those paths
  already share `managed_block_serve_input`; extend that input rather
  than adding a second probe.
- **D-18:** Header serving and header sync are unchanged. This phase does
  not apply the block-body window to `getheaders`.
- **D-19:** In prune mode, do not offer historical block-body inventory
  outside the limited window. Recent-window inventory stays. Do not add
  BIP37 merkle-block serving or compact-filter serving.

### Boundaries carried forward

- **D-20:** No prune RPC, CLI, or dashboard fields, no manual prune
  command, no prune-lock operator surface, and no support-bundle prune
  counts. Phase 150 owns those.
- **D-21:** New first-party Rust source and tests under
  `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests` get
  parity breadcrumbs through `docs/parity/source-breadcrumbs.json`. Cite
  pinned Knots service-bit and limited-serve anchors where a defensible
  source line exists. The v2.4 parity-doc pass and no-claim checkers stay
  in Phase 151.
- **D-22:** Verification remains `bash scripts/verify.sh`. Default
  verification stays deterministic and public-network-free. No new
  production crate or third-party library.

### Claude's Discretion

- Module placement for the pure window predicate versus the node-shell
  service-flag and disconnect wiring, as long as the window math stays
  I/O-free and Fjall stays in the node shell.
- The exact tip-distance comparison that implements "greater than 288 + 2",
  as long as it matches the Knots limited-serve check the researcher cites
  and does not widen Phase 147's delete keep window.
- How status and RPC project the existing `Pruned` label, as long as the
  D-09 conditions are the only production source of that label.

### Deferred Ideas (OUT OF SCOPE)

- Operator prune status, manual prune, prune locks, and sanitized support
  evidence stay in Phase 150.
- Parity-doc citations and no-claim checkers stay in Phase 151, including
  whether a non-prune node should also advertise `NETWORK_LIMITED` beside
  `NETWORK` the way Knots full nodes do.
- BIP37 merkle blocks, compact-filter serving, and archive historical
  serving stay out of v2.4.
- A new downloader preference for remote `NODE_NETWORK_LIMITED` peers is
  not part of this serving phase.

None of the discussion widened the phase into those surfaces.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| SERV-01 | Prune mode advertises `NODE_NETWORK_LIMITED` and does not advertise full `NODE_NETWORK`. | `ServiceFlags::NETWORK_LIMITED = 1 << 10` plus `advertised_service_flags(PruneMode)` used by the node version-message constructors. Disabled stays `NETWORK \| WITNESS`. Mode is the signal, not `have_pruned`. |
| SERV-02 | A request for a block older than the limited serve window is refused. | `block_request_exceeds_limited_serve_window`: active-chain `tip.height.saturating_sub(block.height) > MIN_BLOCKS_TO_KEEP + 2`. Deny inside `managed_block_serve_input` / `gate_managed_block_request` before any byte read. Ordinary requesters disconnect; download permission does not. |
| SERV-03 | A request for a block whose payload prune removed is not served. | Same gate. Missing payload never reaches `maybe_block`. Do not rebuild from undo, coins, or snapshot bytes. In-window miss is `NotFound` without a disconnect. |
| LABL-01 | Status and RPC report `Pruned` only when have-pruned is set and the payload is gone; a missing payload without prune stays `Unavailable`. | Production `BlockServingDataAvailability::Pruned` only for an active-chain missing payload with durable `have_pruned`. Existing `pruned_count` on block-serving evidence is the status/RPC projection. Durability JSON and `getblockchaininfo` stay free of new prune fields. |
</phase_requirements>

## Summary

Peers must see `NODE_NETWORK_LIMITED` (service bit 10) without full `NODE_NETWORK` whenever prune mode is manual or automatic, including when the prune plan is empty and `have_pruned` is still false. Block-body requests on the active chain whose tip distance is greater than `288 + 2` are not served in that mode, even when the payload is still on disk. A payload that prune already removed is not served either. Status and RPC grow the existing `pruned_count` only when durable `have_pruned` is set and that active-chain payload is gone. A missing payload with `have_pruned` false stays `Unavailable`.

The pinned Knots tree is checked out at `v29.3.knots20260210`. `ProcessGetBlockData` refuses with `tip->nHeight - pindex->nHeight > NODE_NETWORK_LIMITED_MIN_BLOCKS + 2` and disconnects unless the peer has `NetPermissionFlags::NoBan`. This phase maps that exemption onto the existing download permission, `PermissionEffectLabel::DownloadServingPolicyInput`, and still answers with `NotFound`. Header serving stays as it is. There is no `getblocks` message in Open Bitcoin, so historical inventory work is "do not add a historical block-body inv walk," not a new message.

**Primary recommendation:** Put the pure window and service-flag predicates in a new I/O-free `open-bitcoin-network` module, then have the node shell apply them inside `managed_block_serve_input` so every existing block-body caller shares one fact path. Disconnect only on an actual block-body request that is outside the window, and only when that peer lacks the download permission.

## Project Constraints

No `.cursor/rules/` directory is present. [VERIFIED: glob `.cursor/rules/**/*` returned nothing]

Binding repo constraints for this phase:

- Functional core stays I/O-free. Fjall reads stay in the node shell. [VERIFIED: AGENTS.md repo-local guidance]
- No rust-bitcoin and no new production crate or third-party library on this path. Keep the hand-rolled `ServiceFlags` newtype. [VERIFIED: AGENTS.md; D-02; D-22]
- New first-party Rust files under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests` need a `docs/parity/source-breadcrumbs.json` entry. [VERIFIED: AGENTS.md]
- New multi-file Rust modules use `foo.rs` plus `foo/`, not `foo/mod.rs`. [VERIFIED: `standards/languages/rust.md`]
- Verification contract is `bash scripts/verify.sh`. [VERIFIED: AGENTS.md; D-22]
- Rust toolchain pin is `1.94.1`. [VERIFIED: `rustc --version` this session]

## Standard Stack

### Core

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| `open-bitcoin-network` `ServiceFlags` | in-tree | Advertise bit 10 beside existing `NETWORK` (`1 << 0`) and `WITNESS` (`1 << 3`) | Already the version-message service field. D-02 forbids `bitflags`. [VERIFIED: `packages/open-bitcoin-network/src/message.rs`] |
| `open-bitcoin-chainstate` `PruneMode` / `MIN_BLOCKS_TO_KEEP` | in-tree, `288` | Startup mode and the 288 in `288 + 2` | Delete keep window already uses this constant and must stay exact. [VERIFIED: `packages/open-bitcoin-chainstate/src/prune/range.rs`, `mode.rs`] |
| `open-bitcoin-node` `managed_block_serve_input` | in-tree | Single block-body fact assembly | D-17. Callers already share it. [VERIFIED: `packages/open-bitcoin-node/src/network/inventory.rs`] |
| `FjallNodeStore::load_have_pruned` | Fjall `3.1.4` | Durable ever-pruned flag | Phase 148 inserts the key only inside a committed paired delete. [VERIFIED: `packages/open-bitcoin-node/src/storage/fjall_store/prune.rs`] |

### Supporting

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `PermissionEffectLabel::DownloadServingPolicyInput` | in-tree | D-15 disconnect exemption | Mapped from `PeerPermissionToken::Download` in `PeerPermissionSet::active_effects`. [VERIFIED: `packages/open-bitcoin-network/src/inbound/permissions.rs`] |
| `WireNetworkMessage::NotFound` | in-tree | Phase 143 refusal | Already what `gate_inventory_for_durable_serving` sends on deny. [VERIFIED: `inventory.rs`] |
| `BlockServingStatusCounters.pruned_count` | in-tree | LABL-01 status/RPC projection | Incremented when a serve decision's `status_label` is `Pruned`. [VERIFIED: `packages/open-bitcoin-node/src/network/block_relay_evidence.rs`] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Hand-rolled `ServiceFlags::NETWORK_LIMITED` | `bitflags` crate | Forbidden by D-02 and D-22. |
| Window predicate in `prune/range.rs` | Serving predicate in `open-bitcoin-network` | `range.rs` is the delete keep window. Putting `+2` there invites a later prune to keep 290. Network already depends on chainstate, so it can import `MIN_BLOCKS_TO_KEEP` without a new crate. [VERIFIED: `packages/open-bitcoin-network/Cargo.toml`] |
| Wait for `have_pruned` before flipping services | Flip from `PruneMode` at startup | Milestone pitfall 6 said "flip when have-pruned is durable." D-01 supersedes that. Empty prune plan still advertises limited service. |

**Installation:** none. No new crates.

**Version verification:** `rustc 1.94.1 (e408947bf 2026-03-25)`, `cargo 1.94.1`, `bun 1.4.2`. [VERIFIED: this session] Fjall remains the already-pinned `3.1.4` node dependency. [VERIFIED: `packages/open-bitcoin-node/Cargo.toml`]

## Architecture Patterns

### Recommended module placement

```text
packages/open-bitcoin-network/src/
├── message.rs                 # add NETWORK_LIMITED = 1 << 10 only
├── limited_serve.rs           # advertised_service_flags + window predicate
└── block_serving.rs           # comment update on Pruned variants; classifier order unchanged

packages/open-bitcoin-node/src/network/
├── inventory.rs               # managed_block_serve_input applies the facts
├── block_serving.rs           # gate denies before byte lookup
├── action_translation.rs      # request-path disconnect only
└── runtime_authority.rs       # PruneMode field, default Disabled
```

`open-bitcoin-network` stays I/O-free. It may import `PruneMode` and `MIN_BLOCKS_TO_KEEP` from `open-bitcoin-chainstate`. Fjall `load_have_pruned` stays behind `DurableBlockSource` in the RPC/node shell and is passed in as a `bool`.

### Pattern 1: Advertised services from mode, not from deletes

**What:** One pure function returns the version-message bits.
**When to use:** Every production `LocalPeerConfig` that represents this node's advertisement (`ManagedNetworkHandle::transient_runtime`, `sync/progress.rs` `local_peer_config`). Leave `LocalPeerConfig::default` and `VersionMessage::default` as `NETWORK | WITNESS` so disabled and test fixtures stay D-03.
**Example:**

```rust
// Sources: Knots init.cpp local services; D-01..D-03
pub fn advertised_service_flags(mode: PruneMode) -> ServiceFlags {
    if matches!(mode, PruneMode::Disabled) {
        return ServiceFlags::NETWORK | ServiceFlags::WITNESS;
    }
    ServiceFlags::NETWORK_LIMITED | ServiceFlags::WITNESS
}
```

Store `PruneMode` on the node network object, default `Disabled`. There is no prune key in RPC/CLI config today. [VERIFIED: grep of `packages/open-bitcoin-rpc/src/config` found no `prune`] Do not add JSONC, CLI, dashboard, or `getblockchaininfo` fields (D-20). Tests set `ManualOnly` or `Automatic { target_mib }` before the version message is built. Do not flip the bits again when a delete commits or when a plan is empty.

### Pattern 2: Tip distance greater than 288 + 2

**What:** The Knots expression is a greater-than against the sum, not `(distance > 288) + 2`.
**When to use:** Prune mode, and only for a block that is on the active chain so a height exists.
**Example:**

```rust
// Source: packages/bitcoin-knots/src/net_processing.cpp ProcessGetBlockData
// tip->nHeight - pindex->nHeight > (int)NODE_NETWORK_LIMITED_MIN_BLOCKS + 2
pub const LIMITED_SERVE_RACE_BUFFER: u32 = 2;

pub fn block_request_exceeds_limited_serve_window(tip_height: u32, block_height: u32) -> bool {
    tip_height.saturating_sub(block_height) > MIN_BLOCKS_TO_KEEP + LIMITED_SERVE_RACE_BUFFER
}
```

`+` binds tighter than `>` in that C++ expression, and the adjacent comment says "add two blocks buffer extension for possible races." [VERIFIED: `net_processing.cpp` lines 2293–2296 at `v29.3.knots20260210`] `saturating_sub` and Knots signed subtraction both yield "does not exceed" when `block_height > tip_height`.

Boundary the tests must lock, for tip height `1000`:

| Block height | Distance | Outside window? | Bytes |
|--------------|----------|-----------------|-------|
| 710 | 290 | no (`290 > 290` is false) | serve only if payload present |
| 709 | 291 | yes | `NotFound`, no bytes, disconnect unless download permission |
| 1000 | 0 | no | recent inventory and tip announcements stay |

Use `active_chain.last().height` as the tip, not the slice length. [VERIFIED: `prune_apply.rs` documents that tip height is the last active-chain position] An empty active chain has no window and no disconnect. A hash that is not on the active chain has no serving height here: leave it on the existing not-found path and do not invent a historical disconnect for it.

Do not edit `height_inside_keep_window` or `MIN_BLOCKS_TO_KEEP`. That helper remains `height > tip.saturating_sub(288)` for deletion. [VERIFIED: `range.rs`]

### Pattern 3: One input, then a request-only disconnect

**What:** `managed_block_serve_input` computes payload presence, have-pruned availability, and `limited_window_refused`. `gate_managed_block_request` denies before `lookup_block` when the window flag is set, even if availability is `Available`.
**When to use:** All four current callers, because a cache-only path can still leak bytes (D-07):

- `inventory.rs` `serve_inventory` (immediate, cache)
- `inventory.rs` `gate_inventory_for_durable_serving` (durable getdata)
- `action_translation.rs` `ServeCompactBlockTransactions`
- `announcement_transport.rs` `announcement_status_and_gate`

Disconnect is not part of the shared input. Set it only when a peer **requested** a block body (`getdata` block / witness block / compact block, or compact block-txn) and `limited_window_refused` is true and `active_permission_effects` does not contain `DownloadServingPolicyInput`. Then send `NotFound` and disconnect. Announcement uses the same refuse flag to withhold a historical body inv and must not disconnect the peer.

`PeerPermissionToken::Download` already becomes `DownloadServingPolicyInput`. [VERIFIED: `permissions.rs` lines 224–226] Knots exempts `NetPermissionFlags::NoBan` on this specific check, and `NetPermissionFlags::Download` on a different historical-bandwidth disconnect. [VERIFIED: `net_processing.cpp` lines 2283–2300] D-15 locks the download effect. Do not also exempt `NoBan` / `MisbehaviorPolicyProtected`.

In-window missing payload: `NotFound` only (D-16). Do not disconnect because a read failed.

### Pattern 4: Pruned label only on the existing missing-payload arm

**What:** `classify_block_serving_status_label` returns `Pruned` only after it has already passed the `Unknown`, `Unvalidated`, `Stale`, and `SideChain` early returns and the availability is `Pruned`. [VERIFIED: `block_serving.rs` `classify_block_serving_status_label`] The shell today produces `Unavailable` only for an active-chain validated block with no payload. That is the arm to replace with `Pruned` under D-09.

Production availability rule inside `managed_block_serve_input`:

```rust
let data_availability = if payload_present {
    BlockServingDataAvailability::Available
} else if have_pruned && validated_on_active_chain {
    BlockServingDataAvailability::Pruned
} else {
    BlockServingDataAvailability::Unavailable
};
```

`validated_on_active_chain` is the current "would otherwise be Unavailable" fact. An unknown hash stays `Unknown` and `NotFound`. Index-known blocks that are not on the active chain also stay on the `Unknown` path, because the classifier returns `Unknown` before it reads availability. Changing that order would relabel `Unknown`, which D-12 forbids. Do not set chain position to `Active` just to force a `Pruned` label.

`have_pruned` is loaded once per inbound message from `DurableBlockSource`, not once per height and not from `PruneMode`. On a load error, treat the flag as false so a failed read cannot emit `Pruned`. The block is still not served. Add `load_have_pruned` to `DurableBlockSource` and implement it with `FjallNodeStore::load_have_pruned`. Memory runtimes and test sources that have never deleted return false.

Status/RPC projection: keep using `record_block_serving_evidence`, which already increments `BlockServingStatusCounters.pruned_count`. CLI status, dashboard, and support render that counter. Do not add `pruned`, prune height, or prune target to durability JSON, `getblockchaininfo`, or the support bundle (D-20). Existing tests forbid `pruned` on the durability cluster; keep those tests green. [VERIFIED: `network_status_schema.rs` `open_bitcoin_network_status_omits_getblock_and_pruned_on_durability`; `status/chainstate_durability.rs`]

Update the two reserved-label comments so they describe an earned label (D-13). Classifier tests in `block_serving/tests/status_cases.rs` may still inject `Pruned` directly.

### Anti-Patterns to Avoid

- **Advertising from `have_pruned`:** An empty plan in manual or automatic mode must already be `NETWORK_LIMITED | WITNESS`. [D-01]
- **Serving outside the window when the cache still holds the block:** Immediate `serve_inventory` passes `durable_payload_present: false` and reads `blocks_by_hash`. The window flag has to win before that lookup. [VERIFIED: `inventory.rs` `serve_inventory`]
- **Disconnecting announcements or in-window misses:** The disconnect is a request outcome for tip distance `> 290` only.
- **Putting `+2` into `MIN_BLOCKS_TO_KEEP` or `height_inside_keep_window`.**
- **Reconstructing a pruned block** from undo, coins, or `ChainstateSnapshot` leftovers.
- **New wire command** for the refusal. Reuse `NotFound`.
- **New operator prune fields** in RPC, CLI, or dashboard.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Service bit set | `bitflags`, extra service bits, bloom, compact filters | `ServiceFlags(1 << 10)` | D-02. Knots bit is already `(1 << 10)`. [VERIFIED: `protocol.h` line 327] |
| Limited window | A second keep constant of 290 on the delete path | Import `MIN_BLOCKS_TO_KEEP` and add `LIMITED_SERVE_RACE_BUFFER` only in the serve predicate | Delete window stays 288. |
| Pruned detection | "bytes missing" or "prune mode is on" | `load_have_pruned && !payload_present` on the active-chain missing arm | Knots `IsBlockPruned` is `m_have_pruned && !(BLOCK_HAVE_DATA) && nTx > 0`. [VERIFIED: `blockstorage.cpp` lines 711–714] D-09 drops the `nTx > 0` conjunct; `ChainPosition` has no `nTx`. [VERIFIED: `chainstate/src/types.rs`] |
| Historical download policy | `FindNextBlocksToDownload` limited-peer skip | Nothing this phase | That check reads the **remote** peer's services. [VERIFIED: `net_processing.cpp` lines 1492–1494] D-04 forbids it. |
| `getblocks` inv walker | Knots `nPrunedBlocksLikelyToHave` | Do not add `GetBlocks` | Open Bitcoin has no `getblocks` message. [VERIFIED: grep of `WireNetworkMessage` / `"getblocks"`] |
| Permission flag | A new limited-serve permission | `DownloadServingPolicyInput` | D-15. |

**Key insight:** The refusal, the label, and the advertisement are three different facts. Mode chooses the bits. Tip distance chooses whether bytes and a disconnect happen. Durable `have_pruned` plus a missing active-chain payload is the only production source of `Pruned`.

## Common Pitfalls

### Pitfall 1: Flipping services only after the first delete

**What goes wrong:** Prune mode is on, the plan is empty, and the version message still says `NETWORK`. Peers request ancient history.
**Why it happens:** `.planning/research/PITFALLS.md` pitfall 6 says to flip flags when `have_pruned` is durable. That text is older than D-01.
**How to avoid:** `advertised_service_flags` reads `PruneMode` only. Test manual-only and automatic with `have_pruned == false` and an empty plan.
**Warning signs:** Version bits change inside `flush_and_evict_pruned_blocks`.

### Pitfall 2: `288 + 2` parsed as the wrong comparison

**What goes wrong:** The node serves height `tip - 291`, or it refuses height `tip - 290`, or deletion starts keeping 290 blocks.
**Why it happens:** The C++ line is easy to mis-group, and `NODE_NETWORK_LIMITED_MIN_BLOCKS - 2` exists nearby for a different purpose (remote limited peers we download from, line 1493).
**How to avoid:** Lock `tip 1000 / height 710` as inside and `height 709` as outside. Do not use the `>= 288 - 2` download predicate.
**Warning signs:** A test named for "288 blocks" that treats distance 290 as outside.

### Pitfall 3: Cache or announcement leaks bytes or disconnects

**What goes wrong:** Durable getdata is refused, but `serve_inventory` still returns the cached block. Or preparing a tip announcement disconnects the peer.
**Why it happens:** Four call sites share `managed_block_serve_input`, and only the request paths should disconnect.
**How to avoid:** Window deny lives in the shared gate. Disconnect lives in the getdata and block-txn request arms.
**Warning signs:** Disconnects with no inbound `GetData` / `GetBlockTxn`.

### Pitfall 4: Label lie, or a silent label that never increments

**What goes wrong:** Every missing payload becomes `pruned_count`, or a real delete stays `unavailable_count` forever because availability is set after the `Unknown` early return.
**Why it happens:** `Pruned` is already a classifier arm, and the shell currently forces `Unavailable` whenever payload is absent, including unknown hashes.
**How to avoid:** Inject `Pruned` only when `have_pruned && !payload_present && validated_on_active_chain`. Assert `pruned_count` stays 0 when the flag is false. Assert it increments for an active-chain gap after the flag is true, and that a still-present older block stays `Available`.
**Warning signs:** `data_availability = Pruned` assigned without reading `load_have_pruned`. `pruned` appearing in durability JSON.

### Pitfall 5: Treating Knots' getblocks depth as this phase's window

**What goes wrong:** A second threshold of `288 - 6` (`MIN_BLOCKS_TO_KEEP - 3600 / nPowTargetSpacing`, and mainnet spacing is `10 * 60`) ships beside `288 + 2`.
**Why it happens:** Knots `GETBLOCKS` stops inventory at `tip - nPrunedBlocksLikelyToHave` when prune mode is on. [VERIFIED: `net_processing.cpp` lines 4102–4107; `kernel/chainparams.cpp` `nPowTargetSpacing = 10 * 60`]
**How to avoid:** This repo does not implement `getblocks`. D-19 is satisfied by not adding that walk and by letting the shared window refuse a historical **body** offer. Record the `288 - 6` formula as a Phase 151 note if `getblocks` is ever added. Recent tip announcements (distance 0) stay.
**Warning signs:** A new `WireNetworkMessage::GetBlocks` variant in this phase.

### Pitfall 6: Disconnecting pruned reads the way Knots disconnects a failed disk read

**What goes wrong:** An in-window pruned block disconnects the peer, or an out-of-window block with the payload still present is sent.
**Why it happens:** Knots returns quietly when `BLOCK_HAVE_DATA` is unset, then disconnects if a later read fails and `IsBlockPruned` is true. [VERIFIED: `net_processing.cpp` lines 2302–2324] D-16 forbids the in-window disconnect. D-07 forbids the bytes.
**How to avoid:** Window check first, independent of payload presence. Pruned absence is `NotFound` without disconnect.
**Warning signs:** `disconnect_peer` in the `!payload_present` branch with no tip-distance test.

## Code Examples

### Knots limited-serve refusal

```cpp
// Source: packages/bitcoin-knots/src/net_processing.cpp lines 2292-2305
// tag v29.3.knots20260210
// Avoid leaking prune-height by never sending blocks below the NODE_NETWORK_LIMITED threshold
if (!pfrom.HasPermission(NetPermissionFlags::NoBan) && (
        (((peer.m_our_services & NODE_NETWORK_LIMITED) == NODE_NETWORK_LIMITED) &&
         ((peer.m_our_services & NODE_NETWORK) != NODE_NETWORK) &&
         (tip->nHeight - pindex->nHeight > (int)NODE_NETWORK_LIMITED_MIN_BLOCKS + 2))
   )) {
    pfrom.fDisconnect = true;
    return;
}
if (!(pindex->nStatus & BLOCK_HAVE_DATA)) {
    return;
}
```

Map `NoBan` to `DownloadServingPolicyInput` (D-15). Map both the disconnect return and the missing-data return to `NotFound` (D-14). Keep the disconnect only for the first condition.

### Knots prune advertisement

```cpp
// Source: packages/bitcoin-knots/src/init.cpp lines 977 and 2063-2078
// tag v29.3.knots20260210
ServiceFlags g_local_services = ServiceFlags(NODE_NETWORK_LIMITED | NODE_WITNESS);
// ...
if (chainman.m_blockman.IsPruneMode()) {
    // prune; NODE_NETWORK is not added
} else if (!BackgroundSyncInProgress()) {
    g_local_services = ServiceFlags(g_local_services | NODE_NETWORK);
}
```

Phase 149 does **not** copy the non-prune `NETWORK_LIMITED | NETWORK | WITNESS` combination (D-03). Disabled stays `NETWORK | WITNESS` only.

### Knots pruned predicate

```cpp
// Source: packages/bitcoin-knots/src/node/blockstorage.cpp lines 711-714
bool BlockManager::IsBlockPruned(const CBlockIndex& block) const {
    return m_have_pruned && !(block.nStatus & BLOCK_HAVE_DATA) && (block.nTx > 0);
}
```

Implement `m_have_pruned && !payload_present` on the active-chain missing arm. Do not add an `nTx` fact in this phase.

### Header path that this phase leaves alone

Knots `GETHEADERS` does not apply the `288 + 2` body check. [VERIFIED: `net_processing.cpp` lines 4177–4232] Open Bitcoin `GetHeaders` handling stays unchanged (D-18).

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Only `NODE_NETWORK` means "I can serve blocks" | BIP 159 `NODE_NETWORK_LIMITED` bit 10 means at least the last 288 blocks, and pruned nodes must not also claim the full chain | BIP 159, 2017-05-11, status Deployed | Pruned advertisement is `NETWORK_LIMITED \| WITNESS` without `NETWORK`. [CITED: https://github.com/bitcoin/bips/blob/master/bip-0159.mediawiki] |
| Open Bitcoin hard-codes `NETWORK \| WITNESS` and labels every gap `Unavailable` | This phase adds bit 10, the serve window, and an earned `Pruned` label | Phase 149 | Reservation comments in `block_serving.rs` become an earned-label contract. |
| Knots sets `m_have_pruned` before unlink | Open Bitcoin sets `have_pruned` only inside the committed delete batch | Phase 148 | This phase reads that flag and does not move the write earlier. [VERIFIED: `fjall_store/prune.rs` module docs] |

**Deprecated/outdated:**

- Milestone pitfall text that says to clear `NETWORK` only after `have_pruned` is durable. D-01 is the contract.
- Non-prune co-advertisement of `NETWORK_LIMITED` beside `NETWORK`. Phase 151.
- `IsBlockPruned`'s `nTx > 0` conjunct. Out of this phase's type model.

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | No operator config key currently selects `PruneMode`, so the node field defaults to `Disabled` and tests set manual/automatic directly. Production `open-bitcoind` stays full-history advertisement until Phase 150 parses a prune setting. | Architecture Pattern 1 | If a hidden config already selects prune, SERV-01 would not be visible on that binary until the field is wired. Grep of RPC config found no prune key. [VERIFIED: absence in `packages/open-bitcoin-rpc/src/config`] |
| A2 | "Index-known or active-chain" in D-12 means "the blocks that are already a missing-payload result," which today is active-chain validated blocks. Index-known headers that are not on the active chain stay `Unknown` because the classifier short-circuits. | Pattern 4 | If the phase is later required to show `block_status_pruned` for off-chain index-known gaps, the classifier order has to change without relabeling true unknown hashes. |

**If this table is empty:** N/A — A1 and A2 need the planner to follow the recommendation above, not to reopen D-01..D-22.

## Open Questions (RESOLVED)

1. **Where should Phase 150 read the mode this phase stores?** RESOLVED
   - What we know: `PruneMode` and `parse_prune_arg` exist in chainstate and are not stored on the node network object. [VERIFIED]
   - What's unclear: The future JSONC/CLI field name. Out of scope here (D-20).
   - Recommendation: Add `prune_mode: PruneMode` on the network object now, default `Disabled`, so Phase 150 sets one field before the version message is built.
   - **RESOLVED:** Store `PruneMode` on the node network object now, defaulting to `PruneMode::Disabled`. Phase 150 chooses the JSONC and CLI name.

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| `rustc` / `cargo` | Implementation and tests | ✓ | 1.94.1 | — |
| `bun` | `scripts/command-timings.ts` wrappers | ✓ | 1.4.2 | — |
| Bitcoin Knots submodule | Line-level anchors | ✓ | `v29.3.knots20260210` (`git -C packages/bitcoin-knots describe --tags --exact-match`) | — |

**Missing dependencies with no fallback:** none.

**Missing dependencies with fallback:** none.

Step 2.6 external services: not required. The phase is in-process Rust. Default verification stays public-network-free (D-22).

## Security Domain

`security_enforcement` is not set to `false` in `.planning/config.json`, so this section is required. [VERIFIED: `.planning/config.json`] `workflow.nyquist_validation` is `false`, so the Validation Architecture section is omitted.

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V2 Authentication | no | No new login or credential surface |
| V3 Session Management | no | Peer session lifetime is unchanged |
| V4 Access Control | yes | Download permission is the only disconnect exemption; do not add a permission token |
| V5 Input Validation | yes | Inventory type and active-chain height gate the window; unknown hashes stay not-found; `PruneMode` stays the existing parser |
| V6 Cryptography | no | No new hashes, signatures, or key handling |

### Known Threat Patterns for limited serving

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Serve a deep block that is still on disk and reveal true prune depth | Information disclosure | Deny before lookup when distance `> 290` (BIP 159, D-07) |
| Advertise `NODE_NETWORK` while prune mode is on | Spoofing | `advertised_service_flags` omits `NETWORK` for manual and automatic |
| Label a corrupt or never-downloaded gap as `Pruned` | Repudiation | `Pruned` only when durable `have_pruned` and payload absent (D-09, D-10) |
| Disconnect on every missing body, including in-window gaps | Denial of service | Disconnect only for out-of-window block-body requests from peers without download permission (D-15, D-16) |
| Rebuild a deleted block from undo or snapshot bytes | Tampering | Serve only the existing payload-present fact (D-08) |

## Sources

### Primary (HIGH confidence)

- Local checkout `packages/bitcoin-knots` at tag `v29.3.knots20260210` — `src/protocol.h` lines 309–327 (`NODE_NETWORK_LIMITED = (1 << 10)`); `src/net_processing.cpp` lines 122–125, 1492–1494, 1865–1871, 2236–2305, 4102–4107, 4177–4232; `src/init.cpp` lines 977 and 2063–2078; `src/node/blockstorage.cpp` lines 711–714; `src/validation.h` line 71 (`MIN_BLOCKS_TO_KEEP = 288`); `src/kernel/chainparams.cpp` `nPowTargetSpacing = 10 * 60`
- [CITED: https://github.com/bitcoin/bips/blob/master/bip-0159.mediawiki] — bit 10 / `0x400`, minimum last 288 blocks, pruned peers must not signal full-history service, should not serve deeper than 288
- In-tree Open Bitcoin: `message.rs` `ServiceFlags`; `block_serving.rs` classifier; `inventory.rs` `managed_block_serve_input`; `permissions.rs` download effect; `range.rs` `MIN_BLOCKS_TO_KEEP`; `fjall_store/prune.rs` `load_have_pruned`; `block_relay_evidence.rs` `pruned_count`

### Secondary (MEDIUM confidence)

- `.planning/research/STACK.md`, `FEATURES.md`, `SUMMARY.md`, `PITFALLS.md` — milestone description of bit 10, `288 + 2`, and the label split. Where that pitfall text disagrees with D-01, D-01 wins.

### Tertiary (LOW confidence)

- None. Knots line numbers above were read from the pinned checkout in this session, not assumed.

## Metadata

**Confidence breakdown:**

- Standard stack: HIGH — no new libraries; seams are in-tree
- Architecture: HIGH — Knots serve predicate and Open Bitcoin call sites were read; A2 is an interpretation of D-12 against the existing classifier order
- Pitfalls: HIGH — each pitfall is tied to a verified branch or a locked decision

**Research date:** 2026-09-27
**Valid until:** 2026-10-27 (stable protocol bits; re-check if the Knots pin moves)

## Recommended Plan Split

Three plans. `granularity` in `.planning/config.json` is `fine`. [VERIFIED]

1. **Pure limited-serve facts (SERV-01 math, window math).** Add `ServiceFlags::NETWORK_LIMITED`. Add `limited_serve.rs` with `advertised_service_flags` and `block_request_exceeds_limited_serve_window`. Unit-test disabled vs manual vs automatic bits, and the 290/291 boundary. Breadcrumbs for the new file cite `protocol.h`, `init.cpp`, and `net_processing.cpp`. Do not change delete helpers.

2. **Node gate, refusal, and disconnect (SERV-01 wiring, SERV-02, SERV-03).** Hold `PruneMode` defaulting to `Disabled` and use the helper in the production `LocalPeerConfig` builders. Extend `managed_block_serve_input` with the window flag and the have-pruned availability rule. Deny before byte lookup on every shared caller. `NotFound` for out-of-window and for missing pruned payloads. Disconnect only on a block-body request that is outside the window when the peer lacks `DownloadServingPolicyInput`. Do not disconnect announcements, unknown hashes, or in-window misses. Do not add `GetBlocks`.

3. **Honest status projection (LABL-01, D-13, D-20 guard).** Rewrite the two reserved `Pruned` comments. Test that `pruned_count` increments only for an active-chain gap with durable `have_pruned`, that a missing payload with the flag false stays `unavailable_count`, and that a present payload stays `available_count`. Keep durability JSON, `getblockchaininfo`, and support-bundle copy free of new prune fields.

## Planner Notes That Are Easy to Miss

- `ServiceFlags` has `contains` and `BitOr`. There is no `BitAnd`. Assert with `contains(NETWORK_LIMITED)` and `!contains(NETWORK)`. [VERIFIED: `message.rs`]
- Compact block-txn serving currently passes `durable_payload_present: false` and reads only the cache. The window flag still has to deny a cached body outside the window. [VERIFIED: `action_translation.rs`]
- `have_pruned` is crate-visible on `FjallNodeStore` and is not on `ChainstateStore`. Add it to `DurableBlockSource` rather than widening the store trait for memory tests. [VERIFIED: `context/inbound_wire.rs`, `fjall_store/prune.rs`]
- Phase 148 cache eviction already drops deleted hashes from `blocks_by_hash` after a committed unlink. Serving must still treat a durable miss as absent when the flag is set, including if the cache and the store disagree. [VERIFIED: `runtime_authority/prune_flush.rs`]
