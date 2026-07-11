---
generated_by: gsd-phase-researcher
lifecycle_mode: yolo
phase_lifecycle_id: 118-2026-07-11T16-07-50
generated_at: 2026-07-11T16:15:00.000Z
researched: 2026-07-11
domain: outbound-compact-block-announcement-wiring
confidence: HIGH
---

# Phase 118: Outbound Compact Block Announcement Wiring - Research

**Researched:** 2026-07-11
**Domain:** BIP152 outbound `cmpctblock` announce wiring (CMP-05 runtime seam)
**Confidence:** HIGH

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

#### Action Honor Path

- **D-01:** `ManagedPeerNetwork::announce_block` must branch on `CompactAnnouncementDecision.action` instead of discarding it. `AnnounceCompactBlock` builds and returns `WireNetworkMessage::CompactBlock`; `AnnounceHeaders` / `AnnounceInventory` keep the existing headers/inv emission; `Suppress` returns no outbound message.
- **D-02:** Prefer extending the announce path in `open-bitcoin-network` peer surfaces so `PeerManager` (or a focused helper beside it) can emit CompactBlock/Headers/Inv/None from a typed action plus the validated local block. Keep `ManagedPeerNetwork` as the shell that decides, records evidence from what was actually emitted, and forwards the message — do not leave decision→wire branching only in the node adapter if the network crate already owns announce emission.

#### Compact Payload Construction

- **D-03:** Add a production Block→`CompactBlockPayload` builder for the outbound announce path (not test-only fixtures). Use existing codec short-ID / prefilled helpers. For a locally validated block the announcer knows every transaction; prefer Knots-aligned announce shape (header, nonce, short IDs, coinbase/prefilled as required by BIP152 version 2) rather than stuffing every transaction as prefilled unless research shows that is the only correct local path.
- **D-04:** Payload construction failures on an `AnnounceCompactBlock` decision must not emit a false-positive compact announce. Fall back to a typed headers or inventory announce with a stable reason, or suppress with a stable reason — never record `CompactAnnounced` without a CompactBlock message.

#### Evidence Correctness

- **D-05:** `CompactAnnounced` / `compact_announced_count` increments only when a `WireNetworkMessage::CompactBlock` is actually produced for send. Recording evidence from the decision reason alone before emission is the false-positive bug this phase closes.
- **D-06:** Headers fallback, inventory fallback, and suppress reasons continue to update their existing counters only when those outcomes are the path taken after action honor (and any construction fallback).

#### Fallback And Suppression

- **D-07:** When the decision is already `AnnounceHeaders`, `AnnounceInventory`, or `Suppress`, preserve current Headers/Inv/None behavior and reasons. Do not invent new public defaults or couple announcement to transaction relay / package relay / filters.
- **D-08:** Existing Phase 113 policy gates remain authoritative for *when* compact announce is allowed. This phase does not reopen negotiation policy; it only makes the decided action observable on the wire.

#### Verification And Parity

- **D-09:** Runtime/unit tests must prove: (1) high-bandwidth eligible path emits `WireNetworkMessage::CompactBlock`, (2) headers/inventory/suppress paths still emit Headers/Inv/None, (3) `compact_announced_count` rises only on real CompactBlock emission, (4) construction failure does not increment compact-announced evidence.
- **D-10:** New or touched first-party Rust source/test files under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests` require parity breadcrumbs and `docs/parity/source-breadcrumbs.json` entries unless an explicit `none` breadcrumb is defensible. Prefer Knots `net_processing.cpp` / BIP152 announce anchors.
- **D-11:** Verification remains deterministic and local through repo-native checks and `bash scripts/verify.sh`. Public-network compact-relay review stays opt-in UAT only.

### Claude's Discretion

The planner/researcher may choose exact helper placement (peer module vs compact_relay vs codec), nonce selection strategy, and whether `PeerManager::announce_block` gains an action parameter versus a new `announce_block_with_action` API — prefer the smallest API change that makes action honor and evidence correctness testable. Prefer pure builders and early returns.

### Deferred Ideas (OUT OF SCOPE)

- Phase 119: mempool/extra candidate injection into compact receive and mempool-remove lifecycle hooks.
- Phase 120: compact-download timeout scheduling and misbehavior escalation beyond silent suppress.
- Phase 121: DurableSyncRuntime metrics/log projection for block-relay series.
- Package relay, bloom/filter serving, compact filters, public serving defaults, public-network CI, production full-node readiness, and production-funds wallet safety remain out of scope.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| CMP-05 | Node announces compact blocks only when activation, peer negotiation, header state, block availability, and resource limits permit it. | Close the runtime seam: honor `CompactAnnouncementAction` on the wire via Block→`CompactBlockPayload` + `WireNetworkMessage::CompactBlock`, with evidence only after actual CompactBlock emission; preserve Headers/Inv/Suppress from Phase 113 policy. |
</phase_requirements>

## Summary

Phase 118 is a narrow wiring fix, not a policy redesign. Phase 113 already decides `AnnounceCompactBlock` / Headers / Inv / Suppress. The node shell currently records `CompactAnnounced` from that decision, then calls `PeerManager::announce_block`, which always emits Headers or Inv. There is no production Block→`CompactBlockPayload` builder and no announce-path emission of `WireNetworkMessage::CompactBlock`.

Knots high-bandwidth announce (`NewPoWValidBlock`) builds `CBlockHeaderAndShortTxIDs` once with a random nonce, prefills only the coinbase, short-IDs every other tx by witness hash, then pushes `cmpctblock` to peers that requested HB compact and have the previous header but not the current one. Open Bitcoin already has the short-ID selector, SipHash short-ID helper, payload types, and encode path — only the production builder and action-honor emit path are missing.

**Primary recommendation:** Add a pure Knots-shaped `build_compact_block_payload(block, nonce)` in `open-bitcoin-consensus` (reusing `compact_short_id_*` + `transaction_wtxid`), extend `PeerManager` with an action-aware announce helper that returns CompactBlock/Headers/Inv/`None`, and move `ManagedPeerNetwork` evidence recording to after emission based on the message actually produced (with construction-failure fallback to Headers/Inv using existing fallback reason counters).

## Project Constraints (from .cursor/rules/)

No `.cursor/rules/` directory is present in this repo. [VERIFIED: filesystem]

Apply repo-local / Bright Builds constraints instead (from required project instructions):

- Functional core / imperative shell — pure builders; shell records evidence and supplies nonce/I/O. [CITED: standards/core/architecture.md]
- No Rust Bitcoin libraries on the production path. [CITED: AGENTS.md / PROJECT constraints]
- No `unwrap()` in production Rust; prefer `?` / early returns. [CITED: user code-styling + standards]
- New first-party Rust source/test files need parity breadcrumbs + `docs/parity/source-breadcrumbs.json`. [CITED: AGENTS.md Repo-Local Guidance]
- Verification: `bash scripts/verify.sh` (deterministic, no public-network gates). [CITED: AGENTS.md]

## Current Code Seam Analysis

### Primary false-positive seam (node shell)

`ManagedPeerNetwork::announce_block` in `packages/open-bitcoin-node/src/network.rs`:

1. Builds block-serving status / eligibility / resource gate inputs (lines ~331–378).
2. Calls `peer_manager.decide_compact_announcement_for_peer(...)` (lines 379–388).
3. **Immediately** calls `record_compact_announcement_evidence(announcement.reason)` (line 389) — increments `compact_announced_count` when reason is `CompactAnnounced` **before any wire message exists**. [VERIFIED: network.rs:325-393]
4. Calls `peer_manager.announce_block(peer_id, block)` and **ignores** `announcement.action` (lines 390–392). [VERIFIED: network.rs:389-392]

Existing node test `packages/open-bitcoin-node/src/network/tests.rs` negotiates HB `sendcmpct`, announces genesis, then asserts `compact_announced_count == 1` without asserting a CompactBlock message — documenting today's false-positive contract. [VERIFIED: network/tests.rs:500-572]

### Headers/Inv-only emission (network peer)

`PeerManager::announce_block` in `packages/open-bitcoin-network/src/peer.rs` (lines 375–395):

- Unknown peer → `NetworkError::UnknownPeer`.
- If `peer.remote_prefers_headers` → `WireNetworkMessage::Headers` with one header.
- Else → `WireNetworkMessage::Inv` with one `InventoryType::Block`.
- Never branches on `CompactAnnouncementAction`; never builds CompactBlock. [VERIFIED: peer.rs:375-395]

Call sites of `PeerManager::announce_block`: node shell + peer unit tests only (Inv/Headers regression tests). [VERIFIED: ripgrep]

### Policy already correct (do not reopen)

`decide_compact_announcement` / `CompactAnnouncementAction` / reasons live in `packages/open-bitcoin-network/src/peer/compact_relay.rs`. Eligible HB path returns `AnnounceCompactBlock` + `CompactAnnounced`. Fallbacks return Headers/Inv/Suppress with stable reasons. [VERIFIED: compact_relay.rs:255-335]

`decide_compact_announcement_for_peer` records peer `announcement_eligibility` from the decision (eligibility bookkeeping only — not the false-positive counter). [VERIFIED: compact_relay.rs:47-49, peer.rs:263-279]

### Evidence counters

`BlockRelayEvidenceCounters::record_announcement` maps:

| Reason class | Counter |
|--------------|---------|
| `CompactAnnounced` | `compact_announced_count` |
| `CompactHeadersFallback` | `compact_headers_fallback_count` |
| `CompactInventoryFallback` | `compact_inventory_fallback_count` |
| Other suppress/gate reasons | `compact_suppressed_count` |

[VERIFIED: block_relay_evidence.rs:139-161]

Note: Phase 113 policy Headers outcomes often use reasons like `CompactHeaderContinuityMissing` (suppressed counter), while `CompactHeadersFallback` / `CompactInventoryFallback` exist but are **not** currently produced by `decide_compact_announcement`. They are the right stable labels for **construction-failure** fallbacks after an `AnnounceCompactBlock` decision. [VERIFIED: compact_relay.rs + block_relay_evidence.rs]

### Missing production builder

No `build_compact*` / `from_block` production API exists. Test helpers manually assemble `CompactBlockPayload` (e.g. `compact_payload_with_short_ids` in reconstruction tests). [VERIFIED: codebase grep]

Codec already owns `CompactBlockPayload`, `PrefilledTransaction`, `short_id_selector_from_header_and_nonce`, encode/decode, `validate_compact_block_structure`. [VERIFIED: compact_block.rs]

Consensus owns `compact_short_id_for_wtxid`, `compact_short_id_selector`, `transaction_wtxid`. [VERIFIED: consensus/crypto.rs:26-33, 69+]

## How Knots Builds Outbound `cmpctblock` (High-Bandwidth Announce)

### Construction — `CBlockHeaderAndShortTxIDs` ctor

Anchor: `packages/bitcoin-knots/src/blockencodings.cpp` lines 20–29. [VERIFIED: blockencodings.cpp]

```cpp
CBlockHeaderAndShortTxIDs::CBlockHeaderAndShortTxIDs(const CBlock& block, const uint64_t nonce) :
        nonce(nonce),
        shorttxids(block.vtx.size() - 1), prefilledtxn(1), header(block) {
    FillShortTxIDSelector();
    //TODO: Use our mempool prior to block acceptance to predictively fill more than just the coinbase
    prefilledtxn[0] = {0, block.vtx[0]};
    for (size_t i = 1; i < block.vtx.size(); i++) {
        const CTransaction& tx = *block.vtx[i];
        shorttxids[i - 1] = GetShortID(tx.GetWitnessHash());
    }
}
```

**Knots-aligned announce shape (use this):**

1. Header = block header.
2. Nonce = caller-supplied `u64` (Knots: `FastRandomContext().rand64()`).
3. Prefill **only coinbase** at absolute index 0 (`index_delta = 0` on the wire for first prefilled).
4. Short-ID every remaining tx via SipHash of **wtxid** with selector from `SHA256(header || nonce)`.
5. Do **not** prefill every transaction for local announce — Knots explicitly leaves predictive extra prefills as TODO.

Selector + GetShortID: `FillShortTxIDSelector` / `GetShortID` in same file (lines 32–46). Matches existing Open Bitcoin codec/consensus helpers. [VERIFIED]

### Fast announce emission — `NewPoWValidBlock`

Anchor: `packages/bitcoin-knots/src/net_processing.cpp` lines 2024–2072. [VERIFIED: net_processing.cpp]

1. Build one shared `CBlockHeaderAndShortTxIDs(*pblock, FastRandomContext().rand64())`.
2. For each peer: if `m_requested_hb_cmpctblocks && !PeerHasHeader(current) && PeerHasHeader(prev)` → push `CMPCTBLOCK`.
3. Headers/inv tip announcements remain on the separate `UpdatedBlockTip` path (not this phase's policy reopen).

Open Bitcoin Phase 113 already encodes the peer gates as `decide_compact_announcement`; Phase 118 only needs construction + emit.

### Functional test anchor

`packages/bitcoin-knots/test/functional/p2p_compactblocks.py` — compact announcement behavior examples for breadcrumbs / parity docs. [CITED: CONTEXT canonical_refs]

## Existing Codec / Consensus Helpers to Reuse

| Helper | Crate | Role |
|--------|-------|------|
| `short_id_selector_from_header_and_nonce` | codec | BIP152 selector from header+nonce |
| `ShortId` / `short_id_from_masked_u64` | codec | 6-byte short ID type |
| `CompactBlockPayload` / `PrefilledTransaction` | codec | wire payload shape |
| `encode_compact_block_payload` / `validate_compact_block_structure` | codec | encode + structural checks |
| `compact_short_id_for_wtxid` | consensus | SipHash short ID from wtxid |
| `transaction_wtxid` | consensus | witness txid for short IDs |
| `block_hash` | consensus | Inv object hash if needed |
| Test fixture `compact_payload_with_short_ids` | network tests | pattern to promote to production (coinbase prefilled + short IDs) |

**Do not** put the builder in `open-bitcoin-codec`: codec has no consensus/wtxid SipHash dependency and must stay primitive. [VERIFIED: codec Cargo.toml]

**Do put** the pure builder in `open-bitcoin-consensus` (next to short-ID helpers) **or** a thin pure wrapper in `open-bitcoin-network` that calls consensus. Prefer consensus for reuse and functional-core purity. [ASSUMED: consensus is the best home; network import of consensus is already allowed]

## Recommended API Shape (Action Honor)

### 1. Pure builder (functional core)

```rust
// Recommended location: packages/open-bitcoin-consensus/src/crypto.rs
// (or new compact_block_build.rs re-exported from consensus lib)
pub fn build_compact_block_payload(
    block: &Block,
    nonce: u64,
) -> Result<CompactBlockPayload, CodecError> {
    // Early return if transactions empty (invalid announce block).
    // Prefill coinbase at index_delta 0.
    // Short-ID txs[1..] via compact_short_id_for_wtxid(selector, wtxid).
    // Optionally validate_compact_block_structure before Ok.
}
```

Knots shape — coinbase-only prefilled. Construction errors (empty txs, wtxid encode failure) become typed `Err` that the announce path maps to Headers/Inv fallback. [VERIFIED: Knots ctor + D-03/D-04]

### 2. Network emit helper (smallest API change)

**Prefer new method** over mutating the existing Headers/Inv-only `announce_block` semantics used by legacy tests:

```rust
pub fn announce_block_with_action(
    &self,
    peer_id: PeerId,
    block: &Block,
    action: CompactAnnouncementAction,
    compact_nonce: u64, // ignored unless AnnounceCompactBlock
) -> Result<Option<WireNetworkMessage>, NetworkError>
```

Behavior:

| Action | Emission |
|--------|----------|
| `AnnounceCompactBlock` | `build_compact_block_payload` → `Some(CompactBlock(...))`; on build `Err` → Headers or Inv using `peer.remote_prefers_headers` (same as today's announce_block), **caller** records fallback reason |
| `AnnounceHeaders` | `Some(Headers([...]))` |
| `AnnounceInventory` | `Some(Inv([Block]))` |
| `Suppress` | `Ok(None)` |

Keep existing `announce_block(peer_id, block)` as Headers/Inv-only helper used when no compact decision is in play, **or** implement it as `announce_block_with_action(..., AnnounceHeaders|AnnounceInventory based on remote_prefers_headers)` for DRY. Smallest change: leave `announce_block` as-is; have ManagedPeerNetwork call `announce_block_with_action`. [ASSUMED: new method is clearer for testability]

Alternative (also acceptable under discretion): change `announce_block` to take `action` + `nonce` and update the few call sites. Prefer `announce_block_with_action` to avoid rewriting Inv/Headers-only peer tests unnecessarily.

### 3. ManagedPeerNetwork shell flow (D-01/D-02/D-05)

```text
decision = decide_compact_announcement_for_peer(...)
nonce = choose_compact_announce_nonce(block)  // shell / deterministic
maybe_msg = peer_manager.announce_block_with_action(peer_id, block, decision.action, nonce)?
evidence_reason = match (&decision, &maybe_msg) {
  (_, Some(CompactBlock(_))) => CompactAnnounced,
  (AnnounceCompactBlock, Some(Headers(_))) => CompactHeadersFallback,  // construction fallback
  (AnnounceCompactBlock, Some(Inv(_))) => CompactInventoryFallback,
  (AnnounceCompactBlock, None) => /* suppress with stable reason — rare */,
  (_, _) => decision.reason,  // preserve Phase 113 reason for headers/inv/suppress decisions
};
record_compact_announcement_evidence(evidence_reason);
Ok(maybe_msg)
```

**Critical:** delete the pre-emission `record_compact_announcement_evidence(announcement.reason)` call. [VERIFIED: D-05]

### Nonce strategy (discretion)

- Builder is pure and takes `nonce: u64`.
- Knots uses true random; this repo has `getrandom` only in `open-bitcoin-rpc` today, not network/node. [VERIFIED: Cargo.toml grep]
- For deterministic verify: derive nonce from first 8 LE bytes of `block_hash(header)` (or inject via test parameter). Document that this is an intentional deterministic stand-in, not a crypto RNG claim. [ASSUMED: acceptable for v2.1 bounded announce]
- Do **not** add a new rand dependency in this phase unless planner explicitly wants Knots-parity randomness.

## Evidence Recording Fix Approach

1. **Remove** evidence recording before emission in `ManagedPeerNetwork::announce_block`.
2. **Record after** `announce_block_with_action` returns, using the mapping above.
3. **CompactAnnounced** only when `maybe_msg` is `Some(WireNetworkMessage::CompactBlock(_))`.
4. **Construction failure:** emit Headers/Inv; record `CompactHeadersFallback` / `CompactInventoryFallback` so dedicated fallback counters move instead of false `compact_announced_count`. Do not invent new public defaults.
5. **Policy-decided Headers/Inv/Suppress:** keep original `decision.reason` so existing suppress vs fallback counter semantics from Phase 113 stay intact (D-06/D-07).
6. Peer-state `record_announcement_decision` may stay on the decision (eligibility only); it does not drive `compact_announced_count`. [VERIFIED]

Update the existing node evidence test that expects `compact_announced_count == 1` after announce: it must either (a) assert CompactBlock was returned **and** count==1, or (b) stop expecting count==1 until HB path actually emits CompactBlock (then assert both). Prefer (a). [VERIFIED: tests.rs false-positive]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Short ID SipHash | Custom hash | `compact_short_id_for_wtxid` | Already Knots-aligned |
| Selector derivation | Reimplement SHA256(header\|\|nonce) | `short_id_selector_from_header_and_nonce` | Codec owns it |
| Prefill differential encoding | Ad-hoc index math | `PrefilledTransaction { index_delta: 0, ... }` + existing encode | Codec validates |
| Announcement policy | New gates | Existing `decide_compact_announcement` | D-08 — do not reopen Phase 113 |
| Wire message type | New enum | `WireNetworkMessage::CompactBlock` | Phase 112 already added |

**Key insight:** CMP-05 gap is emission + evidence timing, not negotiation math or codec.

## Architecture Patterns

### Recommended touch layout

```
packages/open-bitcoin-consensus/src/
  crypto.rs (+ build_compact_block_payload) OR compact_block_build.rs
packages/open-bitcoin-network/src/peer.rs
  announce_block_with_action(...)
packages/open-bitcoin-network/src/peer/compact_relay.rs  # optional re-export only; no policy reopen
packages/open-bitcoin-node/src/network.rs
  announce_block: action honor + post-emission evidence
packages/open-bitcoin-node/src/network/block_relay_evidence.rs  # only if reason mapping helpers needed
tests: consensus builder unit + network emit + node evidence
docs/parity/source-breadcrumbs.json
```

### Pattern: decide → emit → evidence

**What:** Policy decides; network emits typed wire message; node shell records sanitized counters from actual emission.
**When:** All compact announcement paths in Phase 118.
**Anti-pattern:** Recording evidence from decision reason before emission (today's bug).

### Anti-Patterns to Avoid

- **Stuffing all txs as prefilled:** Not Knots announce shape; larger messages; diverges from BIP152 HB intent. [VERIFIED: Knots TODO comment]
- **Building CompactBlock only in node crate:** Violates D-02; network already owns announce emission.
- **Reopening Phase 113 gates / CMP-06 coupling:** Out of scope.
- **Calling mempool for predictive prefills:** Knots TODO; Phase 119 territory for receive candidates, not announce prefills.
- **`unwrap()` on wtxid/build:** Return `Err` and fall back. [CITED: code-styling]

## Common Pitfalls

### Pitfall 1: False-positive evidence after "fixing" emission only
**What goes wrong:** Emit CompactBlock but still record from decision first (double-count or wrong path).
**How to avoid:** Single evidence call site after emission; assert message type and counter together in tests.
**Warning signs:** `compact_announced_count` increments when returned message is Headers/Inv.

### Pitfall 2: Empty or coinbase-less block construction
**What goes wrong:** `transactions.is_empty()` → invalid payload; `validate_compact_block_structure` rejects empty short_ids+prefilled.
**How to avoid:** Early-return build error → Headers/Inv fallback; tests cover empty-tx construction failure without CompactAnnounced.

### Pitfall 3: Prefill index wrong
**What goes wrong:** Using absolute index 1 or differential wrong for coinbase.
**How to avoid:** Mirror Knots: first prefilled `index_delta = 0` (absolute position 0). Round-trip encode/decode in builder test.

### Pitfall 4: Breaking Headers/Inv-only peer tests
**What goes wrong:** Changing `announce_block` signature without updating peer tests.
**How to avoid:** Add `announce_block_with_action`; leave legacy `announce_block` behavior for Inv/Headers tests.

### Pitfall 5: Scope creep into 119–121
**What goes wrong:** Injecting mempool candidates, timeouts, or DurableSyncRuntime metrics "while we're here."
**How to avoid:** Explicit non-goals below; planner tasks must not touch those files except breadcrumbs if required.

## Code Examples

### Knots-aligned builder sketch

```rust
// Source pattern: packages/bitcoin-knots/src/blockencodings.cpp:20-29
// Helpers: open_bitcoin_consensus::{compact_short_id_for_wtxid, transaction_wtxid}
//          open_bitcoin_codec::{CompactBlockPayload, PrefilledTransaction, short_id_selector_from_header_and_nonce}

pub fn build_compact_block_payload(
    block: &Block,
    nonce: u64,
) -> Result<CompactBlockPayload, CodecError> {
    let Some(coinbase) = block.transactions.first() else {
        return Err(CodecError::CompactBlockEmpty);
    };
    let selector = short_id_selector_from_header_and_nonce(&block.header, nonce);
    let mut short_ids = Vec::with_capacity(block.transactions.len().saturating_sub(1));
    for transaction in block.transactions.iter().skip(1) {
        let wtxid = transaction_wtxid(transaction)?;
        short_ids.push(compact_short_id_for_wtxid(selector, &wtxid));
    }
    let payload = CompactBlockPayload {
        header: block.header.clone(),
        nonce,
        short_ids,
        prefilled_transactions: vec![PrefilledTransaction {
            index_delta: 0,
            transaction: coinbase.clone(),
        }],
    };
    validate_compact_block_structure(&payload)?;
    Ok(payload)
}
```

### Evidence-after-emit sketch

```rust
// Replace network.rs announce_block tail (today lines 389-392)
let maybe_message = self.peer_manager.announce_block_with_action(
    peer_id,
    block,
    announcement.action,
    compact_announce_nonce(block),
)?;
let evidence_reason = compact_announce_evidence_reason(announcement, maybe_message.as_ref());
self.record_compact_announcement_evidence(evidence_reason);
Ok(maybe_message)
```

## Test Plan Outline (D-09)

### Wave A — Pure builder (consensus)

1. Coinbase-only block → payload with 1 prefilled, 0 short IDs; structure validates.
2. Block with coinbase + N txs → N short IDs; each matches `compact_short_id_for_wtxid`.
3. Empty transactions → `Err`; no panic.
4. Encode/decode round-trip of built payload.

### Wave B — PeerManager emit (`announce_block_with_action`)

1. `AnnounceCompactBlock` + valid block → `Some(CompactBlock(_))`.
2. `AnnounceHeaders` → Headers (ignore compact capability).
3. `AnnounceInventory` → Inv Block.
4. `Suppress` → `None`.
5. `AnnounceCompactBlock` + empty txs → Headers or Inv via `remote_prefers_headers`, not CompactBlock.

### Wave C — ManagedPeerNetwork evidence (CMP-05 close)

1. HB negotiated + eligible → returned message is CompactBlock **and** `compact_announced_count == 1`.
2. Low-bandwidth / disabled / suppress paths → no CompactBlock; `compact_announced_count` unchanged; suppress/fallback counters move as today.
3. Force construction failure (empty tx block) on an otherwise eligible decision → Headers/Inv emitted; `compact_announced_count` stays 0; headers/inventory fallback counter increments.
4. Update existing Phase 116-style evidence test that currently asserts count==1 without CompactBlock assertion.

### Verification gate

- Focused: `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-consensus -p open-bitcoin-network -p open-bitcoin-node -- ...`
- Full: `bash scripts/verify.sh`
- Parity: breadcrumbs + `docs/parity/source-breadcrumbs.json` for new files; Knots anchors `net_processing.cpp` (`NewPoWValidBlock`), `blockencodings.cpp` (ctor).

## Standard Stack

### Core

| Library / crate | Version | Purpose | Why Standard |
|-----------------|---------|---------|--------------|
| `open-bitcoin-consensus` | workspace | Block→CompactBlockPayload builder + wtxid/short IDs | Owns SipHash short-ID helpers already |
| `open-bitcoin-codec` | workspace | Payload types + encode/validate | Phase 112 BIP152 codecs |
| `open-bitcoin-network` | workspace | Action-aware announce emission | Owns PeerManager announce |
| `open-bitcoin-node` | workspace | Decide + evidence-after-emit shell | ManagedPeerNetwork seam |
| Bitcoin Knots baseline | `29.3.knots20260210` | Behavioral anchor | Pinned submodule |

### Supporting

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| Existing unit-test harness | workspace | Arrange/Act/Assert | All D-09 proofs |
| `docs/parity/source-breadcrumbs.json` | tracked | Parity registry | New/touched Rust files |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Consensus builder | Network-only builder | Duplicates consensus short-ID usage; still OK but less reusable |
| `announce_block_with_action` | Mutate `announce_block` signature | Fewer methods but more test churn |
| Hash-derived nonce | `getrandom` in node | Closer to Knots random; adds dep / non-determinism to verify |
| Prefill all txs | Coinbase-only (Knots) | All-prefilled works for local peers but diverges from Knots announce shape (D-03 rejects unless only path) |

**Installation:** none — first-party crates only. No new registry packages required for the recommended path.

**Version verification:** Rust toolchain pinned `1.94.1` via `rust-toolchain.toml`. [VERIFIED: `rustc 1.94.1`, `cargo 1.94.1` on research host]

## State of the Art

| Old Approach (Open Bitcoin today) | Current Approach (Knots / target) | When Changed | Impact |
|-----------------------------------|-----------------------------------|--------------|--------|
| Decide compact, record CompactAnnounced, emit Headers/Inv | Decide, emit cmpctblock when action says so, evidence from emission | Phase 118 | Closes CMP-05 |
| Test-only CompactBlockPayload fixtures | Production Block→payload builder (coinbase prefilled) | Phase 118 | Real announce path |
| Phase 113 policy-only CMP-05 checkmark | Runtime wire honor | Audit remapped to 118 | Milestone gap closure |

**Deprecated/outdated:** Treating Phase 113 policy tests as sufficient CMP-05 runtime proof. [VERIFIED: v2.1-MILESTONE-AUDIT.md]

## Explicit Non-Goals (Phases 119–121 and beyond)

| Phase / topic | Why out of scope |
|---------------|------------------|
| **119** Mempool/extra candidate injection into compact **receive**; `on_mempool_transaction_removed` hooks | Receive reconstruction feed, not outbound announce |
| **120** Compact-download timeout scheduling; misbehavior escalation beyond silent suppress | Runtime bridge for download/gov, not announce |
| **121** DurableSyncRuntime metrics/log projection for block-relay series | OBS-03 projection gap, not CMP-05 |
| Predictive mempool-based extra prefills on announce | Knots TODO; not required for CMP-05 |
| Package relay, bloom/filter, compact filters, public defaults, public-network CI | Milestone deferred |
| Production full-node / production-funds claims | BOUND no-claim posture |

## Risks and Open Questions for Planner

### Risks

1. **Test contract churn:** Node evidence test currently asserts false-positive count — must be rewritten carefully to avoid masking the bug.
2. **Empty-block fixtures in peer tests:** Some `announce_block` tests use `transactions: Vec::new()`; those must not call `AnnounceCompactBlock` without expecting fallback.
3. **Nonce determinism vs Knots random:** Document intentional deterministic nonce; do not claim RNG parity.
4. **Reason mapping subtlety:** Using original Phase 113 reasons for policy Headers vs `CompactHeadersFallback` for construction failure must stay consistent with counter semantics.

### Open Questions

1. **Exact API name:** `announce_block_with_action` vs extending `announce_block`?
   - What we know: both satisfy D-02; few call sites.
   - Recommendation: new method; keep legacy Headers/Inv helper.

2. **Builder home:** consensus vs network?
   - What we know: short-ID helpers live in consensus; network already depends on consensus.
   - Recommendation: consensus pure function + network calls it.

3. **Construction-failure suppress vs Headers/Inv?**
   - What we know: D-04 allows either; peer already has `remote_prefers_headers`.
   - Recommendation: Headers/Inv fallback (never silent suppress on build failure when peer is announce-eligible), reasons `CompactHeadersFallback` / `CompactInventoryFallback`.

4. **Should peer `announcement_eligibility` update only after successful CompactBlock emit?**
   - What we know: today updates from decision; eligibility is policy state, not evidence counter.
   - Recommendation: leave peer eligibility on decision (Phase 113 behavior); only fix block-relay evidence counters.

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | Best builder home is `open-bitcoin-consensus` | API / Architecture | Planner may prefer network-local helper; still correct if pure |
| A2 | Hash-derived / injectable nonce is acceptable vs Knots `FastRandomContext` | Nonce strategy | If user wants true random, add getrandom to node shell in plan |
| A3 | Construction failure should fall back to Headers/Inv (not Suppress) | Evidence / API | If Suppress preferred, adjust evidence mapping |
| A4 | New `announce_block_with_action` is smaller churn than changing `announce_block` | API | Alternative is still valid under discretion |

**If wrong:** none of these reopen locked D-01..D-11; they only affect plan task shape.

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| Rust / Cargo | Build + tests | ✓ | 1.94.1 | — |
| Bun | verify.sh TS checkers | ✓ | 1.3.9 | — |
| Knots submodule | Parity anchors | ✓ | v29.3.knots20260210 | — |
| getrandom / OS RNG | Optional Knots-like nonce | ✗ in node/network | — | Hash-derived nonce (recommended) |

**Missing dependencies with no fallback:** none for recommended path.

**Missing dependencies with fallback:** true random nonce → hash-derived / injectable.

Step 2.6: external deps are toolchain-only; no service/DB requirements.

## Security Domain

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V2 Authentication | no | — |
| V3 Session Management | no | — |
| V4 Access Control | yes (peer eligibility) | Existing Phase 113 compact announcement gates; do not weaken |
| V5 Input Validation | yes | `validate_compact_block_structure`; reject empty/null prefilled |
| V6 Cryptography | yes (short IDs) | Existing SipHash helpers — never hand-roll |

### Known Threat Patterns for compact announce

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| False operator evidence of compact announce | Spoofing / Tampering of observability | Evidence only after CompactBlock emission (D-05) |
| Oversized announce by prefilling all txs | DoS | Knots coinbase-only prefill shape |
| Announcing without negotiation/activation | Elevation | Unchanged Phase 113 policy gates (D-08) |
| Coupling announce to package/filter relay | Scope creep | CMP-06 isolation preserved |

## Sources

### Primary (HIGH confidence)

- `packages/open-bitcoin-node/src/network.rs` — announce_block seam (lines 325–393)
- `packages/open-bitcoin-network/src/peer.rs` — Headers/Inv-only announce_block (lines 375–395)
- `packages/open-bitcoin-network/src/peer/compact_relay.rs` — action/reason policy
- `packages/open-bitcoin-node/src/network/block_relay_evidence.rs` — evidence counters
- `packages/open-bitcoin-codec/src/compact_block.rs` — payload + short-ID selector
- `packages/open-bitcoin-consensus/src/crypto.rs` — compact_short_id_for_wtxid / wtxid
- `packages/bitcoin-knots/src/blockencodings.cpp` — CBlockHeaderAndShortTxIDs ctor
- `packages/bitcoin-knots/src/net_processing.cpp` — NewPoWValidBlock cmpctblock push
- `.planning/phases/118-outbound-compact-block-announcement-wiring/118-CONTEXT.md` — locked decisions
- `.planning/v2.1-MILESTONE-AUDIT.md` — CMP-05 gap evidence

### Secondary (MEDIUM confidence)

- BIP152 high-bandwidth announce behavior as reflected by Knots functional tests (`p2p_compactblocks.py`) — cited as parity anchor, not re-executed in this research session

### Tertiary (LOW confidence)

- None material; nonce randomness preference is discretionary (logged as A2)

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH — first-party crates and Knots anchors verified in-tree
- Architecture: HIGH — seam lines and locked decisions align; API naming is discretionary only
- Pitfalls: HIGH — false-positive evidence and empty-block cases confirmed against current tests

**Research date:** 2026-07-11
**Valid until:** 2026-08-10 (stable internal wiring; re-check if announce path refactored)
**lifecycle_id:** 118-2026-07-11T16-07-50
**lifecycle_mode:** yolo
