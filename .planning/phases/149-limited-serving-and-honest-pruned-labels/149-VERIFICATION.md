---
phase: 149-limited-serving-and-honest-pruned-labels
verified: 2026-09-28T05:25:00Z
status: passed
score: 11/11 must-haves verified
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: 149-2026-09-27T19-48-11
generated_at: 2026-09-28T05:25:00Z
lifecycle_validated: true
overrides_applied: 0
---

# Phase 149: Limited Serving and Honest Pruned Labels Verification Report

**Phase Goal:** Peers and status see limited-network serving and honest `Pruned` versus `Unavailable` labels only after real deletes.
**Verified:** 2026-09-28T05:25:00Z
**Status:** passed
**Re-verification:** No — initial verification

Lifecycle provenance matches across `149-CONTEXT.md`, all four `149-0*-PLAN.md` files, all four `149-0*-SUMMARY.md` files, and this report: `lifecycle_mode: yolo` and `phase_lifecycle_id: 149-2026-09-27T19-48-11`. None of those artifacts are marked `direct-fallback`.

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
| --- | --- | --- | --- |
| 1 | In prune mode the node advertises `NODE_NETWORK_LIMITED` and does not advertise full `NODE_NETWORK`. | ✓ VERIFIED | `advertised_service_flags` returns `NETWORK_LIMITED \| WITNESS` for every non-disabled `PruneMode`. `set_prune_mode` writes those bits onto `local_config.services` and `PeerManager::set_local_services` without reading `serving_have_pruned`. Advertisement tests cover manual and automatic mode while the flag is false, and the outbound version message. |
| 2 | Disabled advertisement stays `NETWORK \| WITNESS` and does not include `NETWORK_LIMITED`. | ✓ VERIFIED | Disabled `PruneMode` returns early to `NETWORK \| WITNESS`. `LocalPeerConfig::default` and `VersionMessage::default` are unchanged. The disabled-mode version-message test asserts the full-history bits. |
| 3 | A block is outside the limited serve window exactly when tip distance is greater than `288 + 2`, and deletion still keeps 288. | ✓ VERIFIED | `block_request_exceeds_limited_serve_window` is `tip_height.saturating_sub(block_height) > MIN_BLOCKS_TO_KEEP + LIMITED_SERVE_RACE_BUFFER` with the buffer equal to 2. `MIN_BLOCKS_TO_KEEP` remains `288` in chainstate prune range. |
| 4 | Production local-peer builders call `advertised_service_flags(PruneMode::Disabled)` until a later phase parses a prune setting. | ✓ VERIFIED | `ManagedNetworkHandle::transient_runtime` and `sync/progress.rs` `local_peer_config` both pass `PruneMode::Disabled`. New `ManagedPeerNetwork` constructors start at `PruneMode::Disabled` and `serving_have_pruned: false`. |
| 5 | A peer request for a block older than the limited serve window is refused. | ✓ VERIFIED | `limited_window_refused` is set only when prune mode is on and the active-chain distance exceeds the window. `gate_managed_block_request` returns `Deny` with `maybe_block: None` before `lookup_block`. Getdata and getblocktxn tests assert `NotFound` and no block or `BlockTxn` bytes, including when the cache still holds the block. |
| 6 | An ordinary out-of-window peer is removed. A download-only peer is refused without removal. `noban` is not an exemption. | ✓ VERIFIED | `disconnect_for_limited_window_request` disconnects when the window is refused unless `DownloadServingPolicyInput` is present without `MisbehaviorPolicyProtected`. Getdata, durable serving, and `ServeCompactBlockTransactions` call it after queueing `NotFound` and still return `Ok`. Download-permission and noban tests match that split. Announcements do not call the disconnect helper. |
| 7 | A peer request for a block whose payload prune removed is not served. | ✓ VERIFIED | Missing cache and durable payload take the `NotFound` path. `inventory.rs` does not call `load_undo` or rebuild from `ChainstateSnapshot`. In-window gap tests assert the outbound message is only `NotFound`. |
| 8 | Status and RPC report `Pruned` only when have-pruned is set and the payload is gone; a missing payload without prune stays `Unavailable`. | ✓ VERIFIED | Production `BlockServingDataAvailability::Pruned` is assigned only from `serving_have_pruned && validated_on_active_chain` after a missing payload. A present payload stays `Available`. Classifier order still returns `Unknown`, `Unvalidated`, `Stale`, and `SideChain` before `Pruned`. `record_block_serving_evidence` increments `pruned_count` only for that label. RPC `open_bitcoin_network_status` clones the block-relay snapshot that serializes `pruned_count`. Counter tests lock unavailable, pruned, present-payload available, manual mode without the flag, and unknown hash. |
| 9 | In-window misses and unknown hashes do not disconnect the peer. Disabled prune still serves an old present payload. | ✓ VERIFIED | Disconnect requires `limited_window_refused`. An unknown hash is not on the active chain, so the window flag stays false and availability stays off `Pruned`. The disabled-mode height-0 test returns a `Block` and keeps the peer. |
| 10 | In prune mode, an out-of-window historical block-body inventory offer is withheld, a recent-window announcement is still offered, and neither announcement disconnects the peer. | ✓ VERIFIED | `prepare_peer_announcement` returns `Suppressed` with `CompactBlockUnavailable` before `decide_compact_announcement_for_peer` when the window is refused. The tip test is a ready `Inv` of `InventoryType::Block`. Both tests keep `peer_state`. `announcement_transport.rs` does not disconnect. Header sync does not call the window predicate, and `WireNetworkMessage` has no `GetBlocks`. |
| 11 | Durability JSON and `getblockchaininfo` gain no prune fields. | ✓ VERIFIED | Chainstate durability tests still forbid the key `pruned`. `pruned_count` is absent from `status/chainstate_durability.rs`. The projection remains the existing block-serving counter. No operator prune command or prune-height field was added on this phase's production paths. |

**Score:** 11/11 truths verified

Roadmap success criteria 1–4 are truths 1, 5, 7, and 8. Plan must-haves that restate those criteria are covered there. The remaining rows are the plan constraints that add a distinct behavior.

### Required Artifacts

| Artifact | Expected | Status | Details |
| --- | --- | --- | --- |
| `packages/open-bitcoin-network/src/message.rs` | `ServiceFlags::NETWORK_LIMITED` bit 10 | ✓ VERIFIED | `pub const NETWORK_LIMITED: Self = Self(1 << 10);` |
| `packages/open-bitcoin-network/src/limited_serve.rs` | Advertisement and window predicates | ✓ VERIFIED | Both functions are implemented and exported from `lib.rs`. The file does not mention `have_pruned`, block loads, or disconnect. |
| `packages/open-bitcoin-node/src/network/limited_serve.rs` | `set_prune_mode` and ordinary-peer disconnect predicate | ✓ VERIFIED | Mode chooses services. `set_serving_have_pruned` only stores the bool. Disconnect exempts download and still removes `noban`. |
| `packages/open-bitcoin-node/src/network.rs` | `prune_mode` and `serving_have_pruned` | ✓ VERIFIED | Both fields exist and `mod limited_serve` is registered. Constructors initialize them. Clone copies them. |
| `packages/open-bitcoin-node/src/network/inventory.rs` | Window flag and earned `Pruned` availability | ✓ VERIFIED | Shared input computes both facts. Request paths use that input. |
| `packages/open-bitcoin-node/src/network/block_serving.rs` | Gate deny before lookup | ✓ VERIFIED | `limited_window_refused` returns `Deny` with `maybe_block: None` before `lookup_block`. Compact block-txn serving goes through that gate. |
| `packages/open-bitcoin-node/src/storage/fjall_store/prune.rs` | Public have-pruned load | ✓ VERIFIED | `pub fn load_have_pruned` reads whether `HAVE_PRUNED_KEY` is present. It does not invent the flag. |
| `packages/open-bitcoin-network/src/block_serving.rs` | Earned-label comments | ✓ VERIFIED | Both `Pruned` variants contain the earned-label sentence. `Reserved for a future prune-mode` is gone. |
| `packages/open-bitcoin-node/src/network/tests/pruned_label.rs` | Counter projection tests | ✓ VERIFIED | Separate tests for unavailable, pruned, present available, manual mode, and unknown hash. |
| `docs/parity/source-breadcrumbs.json` | `network-limited-serve` and `node-limited-serve` | ✓ VERIFIED | Both groups exist once. Node group lists the four new node files and cites `blockstorage.cpp`. |

### Key Link Verification

| From | To | Via | Status | Details |
| --- | --- | --- | --- | --- |
| `limited_serve.rs` (network) | `MIN_BLOCKS_TO_KEEP` | `> MIN_BLOCKS_TO_KEEP + LIMITED_SERVE_RACE_BUFFER` | ✓ WIRED | Imports the chainstate constant. Does not copy `288`. |
| `lib.rs` (network) | `limited_serve.rs` | `mod limited_serve` and `pub use` | ✓ WIRED | Exports the buffer and both functions. |
| `network/limited_serve.rs` | `advertised_service_flags` | `set_prune_mode` assignment | ✓ WIRED | Also calls `peer_manager.set_local_services`. |
| `peer.rs` | version message | `set_local_services` writes `local_config.services` | ✓ WIRED | Outbound version construction reads that config. |
| `inventory.rs` | window predicate | active-chain tip and block height while prune is on | ✓ WIRED | Disabled mode leaves the flag false. |
| `block_serving.rs` | lookup closure | deny before lookup when the window is refused | ✓ WIRED | `serve_managed_block_request` returns the deny without calling the closure. |
| `inbound_wire.rs` | `FjallNodeStore::load_have_pruned` | `DurableBlockSource` delegates to the public method | ✓ WIRED | Trait and Fjall impl exist. Scripted test source returns `Ok(false)`. |
| `context.rs` | `ManagedNetworkHandle::set_serving_have_pruned` | load, `?` on the handle result, then durable receive | ✓ WIRED | Load errors become `false` through `unwrap_or(false)`. |
| `announcement_transport.rs` | `limited_window_refused` | `Suppressed` with `CompactBlockUnavailable` before the compact decision | ✓ WIRED | In-window blocks continue to the decision. |
| `block_relay_evidence.rs` | `BlockServingStatusLabel::Pruned` | `pruned_count += 1` | ✓ WIRED | RPC clones that block-relay snapshot. |
| `pruned_label.rs` | serve input availability | getdata after `set_serving_have_pruned(true)` | ✓ WIRED | Counter is read from block-relay evidence after the request. |

### Data-Flow Trace (Level 4)

| Artifact | Data Variable | Source | Produces Real Data | Status |
| --- | --- | --- | --- | --- |
| Version message services | `local_config.services` | `advertised_service_flags(prune_mode)` through `set_prune_mode` | Yes, from the stored `PruneMode` | ✓ FLOWING |
| `limited_window_refused` | tip and block height | Active-chain positions plus the pure window predicate | Yes | ✓ FLOWING |
| `BlockServingDataAvailability::Pruned` | `serving_have_pruned` | `FjallNodeStore::load_have_pruned` key presence, applied before durable serving | Yes. A missing key or load error stays false | ✓ FLOWING |
| Status and RPC `pruned_count` | `status_label` | Classifier label recorded into block-relay evidence and cloned into network status | Yes | ✓ FLOWING |

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
| --- | --- | --- | --- |
| Limited bits, window boundary, and disabled default | Read `limited_serve.rs` unit tests for distances 290 and 291, bit 10, and both prune modes | Assertions match the predicate | ✓ PASS |
| Version message follows mode with `have_pruned` false | Read `limited_serve_advertisement.rs` | Manual, automatic, disabled, and outbound version cases are present | ✓ PASS |
| Out-of-window refusal, disconnect split, announcements, and getblocktxn | Read `limited_window.rs` | NotFound, cache retained, download stays, noban leaves, tip `Inv`, no `BlockTxn` | ✓ PASS |
| Honest counters | Read `pruned_label.rs` | `unavailable_count == 1` with flag false; `pruned_count == 1` only for the earned gap | ✓ PASS |

Cargo was not re-run. These library suites exceed the verifier's ten-second spot-check budget, and the assertions sit on the same functions the production paths call.

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
| --- | --- | --- | --- | --- |
| SERV-01 | 149-01, 149-02 | Prune mode advertises `NODE_NETWORK_LIMITED` and does not advertise full `NODE_NETWORK`. | ✓ SATISFIED | Truths 1–4. `REQUIREMENTS.md` lists it under Phase 149 and still Pending, which is expected until requirement rows are flipped after verification. |
| SERV-02 | 149-03 | A request for a block older than the limited serve window is refused. | ✓ SATISFIED | Truths 3, 5, 6, and 10. |
| SERV-03 | 149-03 | A request for a block whose payload prune removed is not served. | ✓ SATISFIED | Truths 7 and 9. |
| LABL-01 | 149-04 | Status and RPC report `Pruned` only when have-pruned is set and the payload is gone; a missing payload without prune stays `Unavailable`. | ✓ SATISFIED | Truths 8 and 11. |

`REQUIREMENTS.md` maps exactly these four IDs to Phase 149. Every ID appears in at least one plan `requirements` field. No phase-149 requirement is orphaned.

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
| --- | --- | --- | --- | --- |
| `packages/open-bitcoin-rpc/src/context.rs` | have-pruned load | `unwrap_or(false)` on `load_have_pruned` | ℹ️ Info | Intentional. A failed store read must not become a prune claim. No dedicated failing-load test; the only production `Pruned` assignment still requires the stored true flag. |
| `packages/open-bitcoin-node/src/network/runtime_authority/prune_flush.rs` | `set_prune_mode` | Handle setter lives beside flush | ℹ️ Info | Plan 02's acceptance grep wanted this file free of `set_prune_mode`. The added method delegates to the same advertisement setter and is not called by the flush body. New networks still start disabled. |
| `packages/open-bitcoin-node/src/network/limited_serve.rs` | disconnect predicate | `noban` still disconnects even though it also carries `DownloadServingPolicyInput` | ℹ️ Info | Matches decision D-15 and the plan's noban test. The download-only permission still stays connected. |

No TODO, FIXME, placeholder, or empty serve path was found in the phase files. The disconnect and `Pruned` assignments are used by the request paths.

### Human Verification Required

None. The behaviors are pure predicates, in-memory network requests, and status counters. No visual, live-network, or external-service check is required to decide the goal.

### Gaps Summary

No gaps block the phase goal. Peers can be advertised limited service from prune mode, historical block bodies and removed payloads are answered with `NotFound` instead of bytes, ordinary peers who ask past the window are disconnected, and `Pruned` moves only after the durable have-pruned flag is set for that missing active-chain payload. Operator prune fields remain Phase 150 work, as this phase required.

---

_Verified: 2026-09-28T05:25:00Z_
_Verifier: Claude (gsd-verifier)_
