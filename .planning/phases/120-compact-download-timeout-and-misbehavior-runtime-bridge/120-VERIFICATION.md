---
phase: 120-compact-download-timeout-and-misbehavior-runtime-bridge
verified: 2026-07-14T00:45:24Z
status: passed
score: 4/4 must-haves verified
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: 120-2026-07-13T20-01-34
generated_at: 2026-07-14T00:45:24Z
lifecycle_validated: true
overrides_applied: 0
---

# Phase 120: Compact Download Timeout and Misbehavior Runtime Bridge Verification Report

**Phase Goal:** Schedule compact-download timeout expiration from the node runtime and escalate typed compact misbehavior beyond silent suppress.
**Verified:** 2026-07-14T00:45:24Z
**Status:** passed
**Re-verification:** No — initial verification

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
| --- | --- | --- | --- |
| 1 | `expire_compact_download_timeouts` is called from the node/sync runtime on a deterministic tick | ✓ VERIFIED | `ManagedPeerNetwork::receive_message` and `receive_sync_message` both call `self.expire_compact_download_timeouts(timestamp)` then `merge_compact_timeout_outbound` (`packages/open-bitcoin-node/src/network.rs`). Clock is caller-supplied message timestamp — no Tokio timer. |
| 2 | Timeout expiration produces full-block fallback or suppression `PeerAction`s on the live path | ✓ VERIFIED | Shell forwarder keeps `PeerAction::Send` → wire `GetData(Block)` (`action_translation.rs`). Live-path tests `receive_sync_message_past_timeout_emits_getdata_and_timeout_evidence` and `receive_message_preserves_other_peer_timeout_getdata` pass. |
| 3 | Disconnect/timeout/reorg cleanup still remove only volatile compact-relay state | ✓ VERIFIED | Disconnect removes peer `compact_download_states`; timeout clears expired `in_flight`; ReceivedBlock calls `on_compact_download_block_connected`; reorg/restart use `cleanup_all_compact_downloads`. `compact_cleanup_cases` + `compact_timeout_cases` prove durable chain tip / store unchanged. |
| 4 | Typed compact misbehavior maps to Knots-aligned disconnect, score, or suppression decisions rather than empty-action silence only | ✓ VERIFIED | `compact_block_txn_actions` maps `Misbehavior`/`UnexpectedBlockHash` → `PeerAction::Disconnect(CompactBlockMisbehavior)`; Invalid init → `CompactBlockHeaderViolation`; `NoMatchingInFlight` stays `Vec::new()`. Shell records peer-policy via `compact_misbehavior_decision`. Six `compact_misbehavior_cases` tests pass. |

**Score:** 4/4 truths verified

### Required Artifacts

| Artifact | Expected | Status | Details |
| -------- | -------- | ------ | ------- |
| `packages/open-bitcoin-node/src/network/action_translation.rs` | Expire forwarder + ReceivedBlock cleanup + misbehavior policy | ✓ VERIFIED | `expire_compact_download_timeouts`, `on_compact_download_block_connected`, `compact_misbehavior_decision` present and wired |
| `packages/open-bitcoin-node/src/network.rs` | receive_* piggyback tick | ✓ VERIFIED | Both receive paths call expire + merge |
| `packages/open-bitcoin-network/src/peer/compact_download_state.rs` | Escalating txn/init actions + peer-scoped expire | ✓ VERIFIED | Disconnect arms + `NoMatchingInFlight` suppress; expire returns `(PeerId, PeerAction)` |
| `packages/open-bitcoin-network/src/compact_download.rs` | Invalid → Misbehavior (not Fallback-only) | ✓ VERIFIED | `CompactReconstructionOutcome::Invalid` → `CompactBlockInitOutcome::Misbehavior` |
| `packages/open-bitcoin-network/src/error.rs` | Compact DisconnectReason / NetworkError | ✓ VERIFIED | `CompactBlockMisbehavior` / header-violation variants present |
| `packages/open-bitcoin-node/src/network/tests/compact_timeout_cases.rs` | Live-path timeout→GetData proofs | ✓ VERIFIED | 4 tests pass |
| `packages/open-bitcoin-node/src/network/tests/compact_misbehavior_cases.rs` | GOV-02 runtime proofs | ✓ VERIFIED | 6 tests pass |
| `packages/open-bitcoin-node/src/network/tests/compact_cleanup_cases.rs` | GOV-03 + Phase 121 isolation | ✓ VERIFIED | 6 tests pass |
| `docs/parity/source-breadcrumbs.json` | Phase 120 breadcrumb registry | ✓ VERIFIED | Entries for timeout/misbehavior/cleanup cases + action_translation |

### Key Link Verification

| From | To | Via | Status | Details |
| ---- | -- | --- | ------ | ------- |
| `ManagedPeerNetwork::expire_compact_download_timeouts` | `PeerManager::expire_compact_download_timeouts` | Thin shell; keeps `Send` (no TX `TransactionRelay` filter) | ✓ WIRED | `action_translation.rs:91-109` |
| `receive_message` / `receive_sync_message` | `expire_compact_download_timeouts` | Message timestamp as `now_unix_seconds` | ✓ WIRED | `network.rs:251-252`, `291-292` |
| Expire path | `record_compact_cleanup(Timeout, …)` | Expired pair count | ✓ WIRED | `action_translation.rs:106-108` |
| `compact_block_txn_actions` | `PeerAction::Disconnect` | Misbehavior + UnexpectedBlockHash | ✓ WIRED | `compact_download_state.rs:215-220` |
| Init Invalid | Disconnect / Misbehavior outcome | Not Fallback-only | ✓ WIRED | `compact_download.rs:396-398` + init actions |
| `NoMatchingInFlight` | `Vec::new()` | Benign suppress | ✓ WIRED | `compact_download_state.rs:221` |
| `process_actions` `ReceivedBlock` | `on_compact_download_block_connected` | Before `connect_stored_block` | ✓ WIRED | `action_translation.rs:186-205` |
| ReceivedBlock cleanup | `record_compact_cleanup(BlockConnected, …)` | When `removed_count > 0` | ✓ WIRED | `action_translation.rs:193-198` |

Note: `gsd-tools verify key-links` reported "Source file not found" for symbolic `from` labels; links were verified by direct source inspection.

### Data-Flow Trace (Level 4)

| Artifact | Data Variable | Source | Produces Real Data | Status |
| -------- | ------------- | ------ | ------------------ | ------ |
| Expire forwarder | `expired_pairs` / outbound GetData | `PeerManager::expire_compact_download_timeouts` → `expire_stale_compact_downloads` | Yes — real in-flight hashes → `GetData(Block)` | ✓ FLOWING |
| receive_* tick | `timestamp` → expire → `merge_compact_timeout_outbound` | Live message clock into result.outbound / targeted_outbound | Yes — proven by live-path tests | ✓ FLOWING |
| Misbehavior bridge | `PeerAction::Disconnect` → `compact_misbehavior_decision` | Typed outcomes → DisconnectReason → MisbehaviorDecision | Yes — score + Disconnect response recorded | ✓ FLOWING |
| ReceivedBlock cleanup | `removed_count` | `on_compact_download_block_connected` across peers | Yes — multi-peer clear + BlockConnected evidence | ✓ FLOWING |

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
| -------- | ------- | ------ | ------ |
| Compact timeout live path | `cargo test -p open-bitcoin-node --lib compact_timeout` | 4 passed | ✓ PASS |
| Compact misbehavior live path | `cargo test -p open-bitcoin-node --lib compact_misbehavior` | 6 passed | ✓ PASS |
| Compact cleanup / isolation | `cargo test -p open-bitcoin-node --lib compact_cleanup` | 6 passed | ✓ PASS |
| PeerManager expire unit | `cargo test -p open-bitcoin-network --lib expire_compact_download` | 1 passed | ✓ PASS |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
| ----------- | ---------- | ----------- | ------ | -------- |
| RCN-07 | 120-01, 120-03 | Fallback/suppression on reconstruction failure, timeout, ineligibility | ✓ SATISFIED | Live timeout→GetData; collision Failed stays Fallback; suppression paths preserved |
| GOV-02 | 120-02, 120-03 | Malformed/invalid/duplicate/unexpected/OOB → disconnect/score/suppress | ✓ SATISFIED | Disconnect + peer-policy for typed misbehavior; NoMatchingInFlight suppress; invalid header disconnect |
| GOV-03 | 120-01, 120-03 | Cleanup removes only volatile compact-relay state | ✓ SATISFIED | Timeout/disconnect/reorg/ReceivedBlock proofs leave durable store unchanged |

Note: `.planning/REQUIREMENTS.md` still lists GOV-02 checkbox / traceability as Pending — tracking lag only; implementation evidence satisfies the requirement.

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
| ---- | ---- | ------- | -------- | ------ |
| `compact_download_state.rs` | 221 | `NoMatchingInFlight => Vec::new()` | ℹ️ Info | Intentional Knots-aligned benign suppress (D-06) — not silence of typed misbehavior |
| `ROADMAP.md` Phase 120 Plans | — | Plans 02/03 still unchecked in roadmap text | ℹ️ Info | Closeout metadata; does not block goal achievement |
| `REQUIREMENTS.md` | GOV-02 | Still marked Pending in checklist | ℹ️ Info | Should flip Complete on phase closeout |

No blocker stubs: Misbehavior no longer maps to empty-action silence; Phase 121 `persist_metrics` / `block_relay_log_record` projection remains absent from `sync/metrics.rs` (inbound-only samples) and is asserted untouched by `phase120_package_filter_and_phase121_surfaces_untouched`.

### Human Verification Required

None. All Phase 120 success criteria are covered by deterministic local unit/runtime tests. Public-network compact-relay UAT remains explicitly opt-in and out of this phase's gate.

### Gaps Summary

No gaps. Phase goal achieved: timeout tick is live on receive_*, timeout emits GetData fallbacks with Timeout evidence, volatile-only cleanup holds including ReceivedBlock multi-peer clear, and typed misbehavior escalates to disconnect/score rather than empty-action silence.

---

_Verified: 2026-07-14T00:45:24Z_
_Verifier: Claude (gsd-verifier)_
