---
phase: 119-compact-receive-mempool-candidate-injection
verified: 2026-07-13T19:04:34Z
status: passed
score: 9/9 must-haves verified
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: 119-2026-07-13T16-08-52
generated_at: 2026-07-13T19:04:34Z
lifecycle_validated: true
overrides_applied: 0
---

# Phase 119: Compact Receive Mempool Candidate Injection Verification Report

**Phase Goal:** Feed live mempool and bounded extra candidates into compact-block receive so reconstruction outcomes and mempool lifecycle hooks work on the runtime path.
**Verified:** 2026-07-13T19:04:34Z
**Status:** passed
**Re-verification:** No — initial verification

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
| --- | ------- | ---------- | -------------- |
| 1 | Inbound `CompactBlock` dispatch no longer always uses empty `CompactBlockReceiveFacts::default()` | ✓ VERIFIED | `ManagedPeerNetwork::receive_message` / `receive_sync_message` match `CompactBlock` and call `handle_compact_block_receive` before `handle_message`; empty-facts path remains only in `message_dispatch` with D-03 non-production annotation |
| 2 | Live receive supplies mempool candidates and bounded extras into `handle_compact_block_download` | ✓ VERIFIED | `collect_compact_receive_owned` → `mempool_compact_candidate_owned` + `compact_extra_owned`; `handle_compact_block_receive` builds `CompactBlockReceiveFacts` and calls `PeerManager::handle_compact_block_download` |
| 3 | `on_mempool_transaction_removed` is hooked from mempool lifecycle without activating package relay or filters | ✓ VERIFIED | `apply_connected_block_mempool_lifecycle` forwards `removal.wtxid`; admission Evicted/Expired + replaced/evicted victims also forward; `phase119_package_filter_surfaces_untouched` asserts defaults stay off |
| 4 | Runtime tests cover reconstruction, collision, duplicate, missing, and lifecycle cleanup outcomes | ✓ VERIFIED | `compact_receive_cases`: reconstruct/missing indexes, ShortIdCollision, duplicate short-ids typed, GetBlockTxn missing, lifecycle slot clear — all 8 tests pass |
| 5 | PeerManager can clear matching volatile partial compact slots by wtxid without importing `open-bitcoin-mempool` | ✓ VERIFIED | `PeerManager::on_mempool_transaction_removed` walks `compact_download_states[*].in_flight[*].partial`; no mempool dep in `open-bitcoin-network/Cargo.toml`; network unit test passes |
| 6 | `CompactExtraTxnBuffer` enforces Knots-aligned slot and byte bounds with FIFO overwrite | ✓ VERIFIED | Constants 32768 / 10_000_000 / 100_000; ring overwrite + byte-budget eviction + `push_gated`; 6 unit tests pass |
| 7 | Empty-facts CompactBlock in `message_dispatch` is documented as non-production for live shell receive | ✓ VERIFIED | Comment block above `CompactBlockReceiveFacts::default()` cites D-03 / `ManagedPeerNetwork::receive_*` |
| 8 | Orphaned, rejected (size-gated), and replaced-victim bodies are pushed into the extra buffer when available | ✓ VERIFIED | `admission_bridge` orphan `push`, reject `push_gated`, replaced-victim `push` before demotion; three admission_bridge_cases prove feeds |
| 9 | Evict/expire and replaced/evicted victim exits with available wtxid also clear matching volatile partial slots | ✓ VERIFIED | `remove_evicted_outcome` + `forward_mempool_removal_wtxids_for_txids`; connected-block lifecycle case clears matched slot |

**Score:** 9/9 truths verified

### Required Artifacts

| Artifact | Expected | Status | Details |
| -------- | ----------- | ------ | ------- |
| `packages/open-bitcoin-network/src/peer/compact_download_state.rs` | PeerManager forwarder | ✓ VERIFIED | `on_mempool_transaction_removed` + `handle_compact_block_download` |
| `packages/open-bitcoin-node/src/network/compact_receive_candidates.rs` | Extra buffer + snapshot helpers + live receive helper | ✓ VERIFIED | `CompactExtraTxnBuffer`, `mempool_compact_candidate_owned`, `handle_compact_block_receive` |
| `packages/open-bitcoin-node/src/network.rs` | CompactBlock intercept on both receive paths | ✓ VERIFIED | Calls `handle_compact_block_receive` (literal `handle_compact_block_download` lives in helper module — intentional) |
| `packages/open-bitcoin-node/src/network/admission_bridge.rs` | Extra buffer feeds + removal forwarders | ✓ VERIFIED | Orphan/reject/replaced feeds; Evicted/Expired/victim wtxid hooks |
| `packages/open-bitcoin-node/src/network/mempool_lifecycle.rs` | Connected-block lifecycle hook | ✓ VERIFIED | Forwards each `removal.wtxid` before TxServing demotion |
| `packages/open-bitcoin-node/src/network/tests/compact_receive_cases.rs` | Injected-path runtime proofs | ✓ VERIFIED | 8 tests covering RCN-02/RCN-03/GOV-04 + sync path |
| `packages/open-bitcoin-network/src/peer/message_dispatch.rs` | Non-production empty-facts annotation | ✓ VERIFIED | D-03 comment retained; path still callable for PeerManager-only tests |

### Key Link Verification

| From | To | Via | Status | Details |
| ---- | -- | --- | ------ | ------- |
| `PeerManager::on_mempool_transaction_removed` | `PartialCompactBlock::on_mempool_transaction_removed` | walk all in-flight partials | ✓ WIRED | `partial.on_mempool_transaction_removed(removed_wtxid)` |
| `ManagedPeerNetwork::receive_message` / `receive_sync_message` | `PeerManager::handle_compact_block_download` | shell-built `CompactBlockReceiveFacts` | ✓ WIRED | via `handle_compact_block_receive` |
| admission outcomes | `CompactExtraTxnBuffer::push` / `push_gated` | orphan/reject/replaced victims | ✓ WIRED | feeds before demotion |
| `apply_connected_block_mempool_lifecycle` | `PeerManager::on_mempool_transaction_removed` | `MempoolLifecycleRemoval.wtxid` | ✓ WIRED | loop over `summary.removed` |
| `compact_receive_cases` | `receive_message(CompactBlock)` | populated mempool / colliding payloads | ✓ WIRED | live path, not empty-facts `handle_message` |
| `CompactExtraTxnBuffer` | Knots extra-txn bounds | named constants | ✓ WIRED | 32768 / 10MB / 100KB |

### Data-Flow Trace (Level 4)

| Artifact | Data Variable | Source | Produces Real Data | Status |
| -------- | ------------- | ------ | ------------------ | ------ |
| `handle_compact_block_receive` | `facts.candidates` / `facts.extra` | `Mempool::entries()` + `compact_extra_txn` owned snapshots | Yes — cloned mempool/extra txs at receive time | ✓ FLOWING |
| lifecycle hook | `removal.wtxid` | `remove_for_connected_block` summary | Yes — real removal records | ✓ FLOWING |
| admission extra feeds | orphan/reject/victim bodies | admission outcomes + relay/local tx cache | Yes — pushed into ring | ✓ FLOWING |

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
| -------- | ------- | ------ | ------ |
| PeerManager forwarder clears slots | `cargo test -p open-bitcoin-network --lib on_mempool_transaction_removed` | 1 passed | ✓ PASS |
| Injected-path runtime suite | `cargo test -p open-bitcoin-node --lib compact_receive_cases` | 8 passed | ✓ PASS |
| Extra buffer + mempool snapshot | `cargo test -p open-bitcoin-node --lib compact_receive_candidates` | 6 passed | ✓ PASS |
| Lifecycle + admission feeds | `cargo test -p open-bitcoin-node --lib 'mempool_lifecycle\|admission_bridge'` | 7 + 17 passed | ✓ PASS |
| Parity breadcrumbs | `bun scripts/check-parity-breadcrumbs.ts` | 374 files verified | ✓ PASS |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
| ----------- | ---------- | ----------- | ------ | -------- |
| RCN-02 | 01, 02, 03 | Reconstruct from mempool + bounded extras via witness-hash short IDs | ✓ SATISFIED | Live inject path + fewer missing indexes than empty-facts baseline |
| RCN-03 | 03 | Typed collision, duplicate, missing, failure outcomes | ✓ SATISFIED | Collision/duplicate → GetData Fallback; missing → GetBlockTxn |
| GOV-04 | 01, 03 | Mempool lifecycle integration without package/filter activation | ✓ SATISFIED | Removal wtxid hooks + package/filter untouched guard |

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
| ---- | ---- | ------- | -------- | ------ |
| — | — | None in Phase 119 touched seams | — | No TODO/FIXME/placeholder stubs in inject/lifecycle/extra-buffer paths |

### Deferred Surfaces Guard (Phase 120/121)

| Deferred item | Status | Evidence |
| ------------- | ------ | -------- |
| Compact-download timeout scheduling (`expire_compact_download_timeouts` from node) | ✓ ABSENT from node shell | API exists in network crate (Phase 115); **zero** calls under `open-bitcoin-node/src` |
| DurableSyncRuntime block-relay metrics projection | ✓ ABSENT from Phase 119 | `sync/metrics.rs` does not call `block_relay_metric_samples` / `block_relay_log_record` |

These remain owned by Phase 120 / Phase 121 roadmap success criteria — not gaps for 119.

### Human Verification Required

None. All success criteria are covered by deterministic local unit/runtime tests.

### Gaps Summary

No gaps. Phase 119 goal achieved: live CompactBlock receive injects mempool + bounded extras, mempool-removal lifecycle clears matching volatile partial slots, and injected-path tests prove reconstruction/collision/duplicate/missing/lifecycle outcomes without activating package/filter or Phase 120/121 surfaces.

---

_Verified: 2026-07-13T19:04:34Z_
_Verifier: Claude (gsd-verifier)_
