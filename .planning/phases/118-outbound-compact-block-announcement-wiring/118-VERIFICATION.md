---
phase: 118-outbound-compact-block-announcement-wiring
verified: 2026-07-11T20:10:17Z
status: passed
score: 8/8 must-haves verified
generated_by: gsd-verifier
lifecycle_mode: yolo
lifecycle_id: 118-2026-07-11T16-07-50
phase_lifecycle_id: 118-2026-07-11T16-07-50
generated_at: 2026-07-11T20:10:17Z
lifecycle_validated: true
overrides_applied: 0
re_verification: false
---

# Phase 118: Outbound Compact Block Announcement Wiring Verification Report

**Phase Goal:** Close the CMP-05 runtime seam so compact announcement decisions produce real outbound `cmpctblock` (or headers/inventory fallback) without false-positive announce evidence.
**Verified:** 2026-07-11T20:10:17Z
**Status:** passed
**Re-verification:** No — initial verification

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
| --- | --- | --- | --- |
| 1 | `ManagedPeerNetwork::announce_block` honors `CompactAnnouncementAction` instead of always emitting Headers/Inv | ✓ VERIFIED | `network.rs` calls `announce_block_with_action(..., announcement.action, compact_nonce)` after `decide_compact_announcement_for_peer`; no pre-emission evidence; legacy `peer_manager.announce_block` unused on this path |
| 2 | `AnnounceCompactBlock` builds and sends `WireNetworkMessage::CompactBlock` from the validated local block | ✓ VERIFIED | `PeerManager::announce_block_with_action` → `build_compact_block_payload` → `WireNetworkMessage::CompactBlock`; HB runtime test asserts CompactBlock return |
| 3 | Compact-announced evidence increments only when a compact payload is actually sent | ✓ VERIFIED | Evidence recorded after emit via `compact_announce_evidence_reason`; `CompactAnnounced` only when message is `CompactBlock`; HB test asserts count==1 with CompactBlock; LB test asserts count==0 |
| 4 | Fallback and suppression paths still emit Headers/Inv or no message with stable reasons | ✓ VERIFIED | Action match emits Headers/Inv/None; construction Err falls back via `remote_prefers_headers`; mapper preserves Phase 113 reasons for policy paths; maps construction fallbacks to `CompactHeadersFallback` / `CompactInventoryFallback` |
| 5 | Production Block→`CompactBlockPayload` builder prefills only the coinbase and short-IDs remaining txs by wtxid | ✓ VERIFIED | `build_compact_block_payload` coinbase-only prefill + `compact_short_id_for_wtxid`; unit tests for coinbase-only and multi-tx SipHash equality |
| 6 | Empty-transaction blocks return a typed Err without panic | ✓ VERIFIED | Early `CodecError::CompactBlockEmpty`; test `build_compact_block_payload_rejects_empty_transactions` |
| 7 | Built payloads encode/decode and pass `validate_compact_block_structure` | ✓ VERIFIED | Builder calls validate before Ok; round-trip test passes |
| 8 | `AnnounceCompactBlock` construction failure falls back to Headers or Inv, never CompactBlock | ✓ VERIFIED | Peer match on build `Err` → Headers/Inv; peer tests for both `remote_prefers_headers` branches |

**Score:** 8/8 truths verified

### Required Artifacts

| Artifact | Expected | Status | Details |
| -------- | ----------- | ------ | ------- |
| `packages/open-bitcoin-consensus/src/compact_block_build.rs` | Pure `build_compact_block_payload` | ✓ VERIFIED | Exists, substantive Knots-shaped builder, re-exported from lib |
| `packages/open-bitcoin-consensus/src/compact_block_build/tests.rs` | Builder unit coverage | ✓ VERIFIED | 4 tests: coinbase-only, short IDs, empty Err, round-trip |
| `docs/parity/source-breadcrumbs.json` | Parity registry for builder | ✓ VERIFIED | `consensus-compact-block-build` entry present |
| `packages/open-bitcoin-network/src/peer.rs` | `announce_block_with_action` | ✓ VERIFIED | Action match + CompactBlock emit + construction fallback; legacy `announce_block` preserved |
| `packages/open-bitcoin-network/src/peer/tests.rs` | Action-aware emit tests | ✓ VERIFIED | 7 `announce_block_with_action_*` tests |
| `packages/open-bitcoin-node/src/network.rs` | Action honor + evidence-after-emit | ✓ VERIFIED | Wired to `announce_block_with_action`; evidence after emission |
| `packages/open-bitcoin-node/src/network/block_relay_evidence.rs` | Evidence reason mapper | ✓ VERIFIED | `compact_announce_evidence_reason` + mapper unit tests |
| `packages/open-bitcoin-node/src/network/tests.rs` | D-09 runtime proofs | ✓ VERIFIED | HB CompactBlock+count==1; LB no CompactBlock+count==0 |

### Key Link Verification

| From | To | Via | Status | Details |
| ---- | --- | --- | ------ | ------- |
| `compact_block_build.rs` | `crypto.rs` | `compact_short_id_for_wtxid` / `transaction_wtxid` | ✓ WIRED | Pattern found |
| `compact_block_build.rs` | codec `compact_block.rs` | payload + validate | ✓ WIRED | Pattern found |
| `peer.rs` | `compact_block_build.rs` | `build_compact_block_payload` | ✓ WIRED | Pattern found |
| `peer.rs` | `compact_relay.rs` | `CompactAnnouncementAction::` | ✓ WIRED | Pattern found |
| `network.rs` | `peer.rs` | `announce_block_with_action` | ✓ WIRED | Pattern found |
| `network.rs` | `block_relay_evidence.rs` | post-emission evidence | ✓ WIRED | Pattern found |

### Data-Flow Trace (Level 4)

| Artifact | Data Variable | Source | Produces Real Data | Status |
| -------- | ------------- | ------ | ------------------ | ------ |
| `ManagedPeerNetwork::announce_block` | `maybe_message` | `announce_block_with_action` ← `build_compact_block_payload(block, nonce)` | Real block header/txs → CompactBlockPayload | ✓ FLOWING |
| Evidence counters | `evidence_reason` | `compact_announce_evidence_reason(decision, maybe_message)` after emit | Counter class follows actual wire message | ✓ FLOWING |

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
| -------- | ------- | ------ | ------ |
| Builder Knots shape + empty Err + round-trip | `cargo test -p open-bitcoin-consensus -- compact_block_build` | 4 passed | ✓ PASS |
| Action-aware emit + construction fallback | `cargo test -p open-bitcoin-network -- announce_block_with_action` | 7 passed | ✓ PASS |
| Evidence mapper + LB non-increment | `cargo test -p open-bitcoin-node -- compact_announce` | 7 passed | ✓ PASS |
| HB CompactBlock + compact_announced_count==1 | `cargo test -p open-bitcoin-node -- phase116_block_relay_evidence` | 2 passed | ✓ PASS |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
| ----------- | ---------- | ----------- | ------ | -------- |
| CMP-05 | 118-01, 118-02, 118-03 | Announce compact blocks only when activation/negotiation/header/availability/resource gates permit; runtime wire honor | ✓ SATISFIED | Policy (113) unchanged; Phase 118 closes decide→wire→evidence seam; REQUIREMENTS.md marks Complete |

No orphaned Phase 118 requirements found.

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
| ---- | ---- | ------- | -------- | ------ |
| — | — | No TODO/FIXME/todo!/unwrap in Phase 118 production paths scanned | — | None |
| `network/tests.rs` | ~552 | Still uses `CompactBlockReceiveFacts::default()` in Phase 116 download portion of HB evidence test | ℹ️ Info | Phase 119 deferred receive injection; does not undermine announce/evidence assertions |

### Scope Isolation (119/120/121)

Feat commits `e4a60f03`, `a3addc1e`, `c8745b06` touch only consensus builder, peer announce API, node announce/evidence, breadcrumbs, and LOC metrics. No mempool candidate injection, no `expire_compact_download_timeouts` scheduling, no DurableSyncRuntime metrics projection, no package/filter/public-default changes.

### Human Verification Required

None. D-09 behaviors are covered by deterministic local unit/runtime tests. Public-network compact-relay review remains opt-in UAT per D-11 / milestone posture and is outside this phase's goal gate.

### Gaps Summary

No gaps. CMP-05 runtime seam is closed: decisions are honored on the wire, CompactBlock is built and emitted on the eligible path, and `compact_announced_count` increments only after a real CompactBlock emission.

### Confirmation-Bias Notes (non-blocking)

1. Construction-failure → no `CompactAnnounced` on the full `ManagedPeerNetwork` path is proven by the pure mapper + peer emit fallback tests (empty-tx blocks cannot pass serving gates to reach `AnnounceCompactBlock` end-to-end — intentional per Plan 03).
2. Hash-derived compact nonce is a documented deterministic stand-in, not Knots `FastRandomContext` parity — acceptable under CONTEXT discretion / RESEARCH A2.

---

_Verified: 2026-07-11T20:10:17Z_
_Verifier: Claude (gsd-verifier)_
