---
phase: 102-orphan-handling-and-admission-outcome-bridge
reviewed: 2026-07-01T05:18:15Z
depth: standard
files_reviewed: 34
files_reviewed_list:
  - docs/metrics/lines-of-code.md
  - docs/parity/catalog/p2p.md
  - docs/parity/checklist.md
  - docs/parity/index.json
  - docs/parity/source-breadcrumbs.json
  - packages/open-bitcoin-mempool/src/lib.rs
  - packages/open-bitcoin-mempool/src/outcome.rs
  - packages/open-bitcoin-mempool/src/pool.rs
  - packages/open-bitcoin-mempool/src/pool/admission_outcome.rs
  - packages/open-bitcoin-mempool/src/pool/tests.rs
  - packages/open-bitcoin-mempool/src/pool/tests/outcome_cases.rs
  - packages/open-bitcoin-network/src/lib.rs
  - packages/open-bitcoin-network/src/peer.rs
  - packages/open-bitcoin-network/src/peer/inventory_state.rs
  - packages/open-bitcoin-network/src/peer/tests.rs
  - packages/open-bitcoin-network/src/peer/transaction_relay.rs
  - packages/open-bitcoin-network/src/peer/transaction_relay/orphanage.rs
  - packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs
  - packages/open-bitcoin-network/src/peer/transaction_relay/tests.rs
  - packages/open-bitcoin-network/src/peer/transaction_relay/tests/orphanage_cases.rs
  - packages/open-bitcoin-network/src/peer/transaction_relay/tests/scheduler_cases.rs
  - packages/open-bitcoin-network/src/peer/transaction_relay/tests/scheduler_cases/edge_cases.rs
  - packages/open-bitcoin-network/src/peer/transaction_relay/tests/scheduler_cases/received_cases.rs
  - packages/open-bitcoin-node/src/mempool.rs
  - packages/open-bitcoin-node/src/network.rs
  - packages/open-bitcoin-node/src/network/action_translation.rs
  - packages/open-bitcoin-node/src/network/admission_bridge.rs
  - packages/open-bitcoin-node/src/network/inbound.rs
  - packages/open-bitcoin-node/src/network/inventory.rs
  - packages/open-bitcoin-node/src/network/tests.rs
  - packages/open-bitcoin-node/src/network/tests/admission_bridge_cases.rs
  - scripts/check-phase102-orphan-admission-bridge.test.ts
  - scripts/check-phase102-orphan-admission-bridge.ts
  - scripts/verify.sh
findings:
  critical: 0
  warning: 3
  info: 0
  total: 3
status: issues_found
---

# Phase 102: Code Review Report

**Reviewed:** 2026-07-01T05:18:15Z
**Depth:** standard
**Files Reviewed:** 34
**Status:** issues_found

## Summary

Reviewed Phase 102's mempool outcome contract, bounded orphanage, scheduler-mediated parent requests, managed admission bridge, disconnect cleanup, deterministic checker, and parity evidence. This review applied repo-local guidance from `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md`, and the relevant Bright Builds architecture, code-shape, verification, testing, Rust, and TypeScript standards.

The implementation is well-covered for the intended happy paths, caps, and evidence wiring, but I found three behavioral risks around transaction-known state, orphan parent fallback scheduling, and capped orphan reconsideration.

## Warnings

### WR-01: Received Transactions Can Poison Already-Have State Before Admission

**File:** `packages/open-bitcoin-network/src/peer/inventory_state.rs:260`

**Issue:** `handle_transaction` inserts the txid and wtxid into `known_txids` / `known_wtxids` before the scheduler and managed mempool admission decide whether the transaction is valid, matched to a request, accepted, rejected, or orphaned. The scheduler also marks both identities as already-have in `record_received_transaction` before the managed admission bridge knows the outcome. This means an identity-mismatched transaction can be suppressed from admission while still making future inventory look locally known, and an orphan can remain globally already-have even after `disconnect_peer_at` removes that peer's orphan entry. A later peer announcing the same transaction can be suppressed instead of being requested and re-admitted.

**Fix:**

```rust
// Do not mark global known/already-have state until the managed admission
// outcome proves the transaction should be retained locally.
let transaction_actions = self.tx_download.record_received_transaction(peer_id, txid, wtxid);
let suppress = transaction_actions
    .iter()
    .any(|action| matches!(action, TxDownloadAction::SuppressIdentityMismatch { .. }));
let mut actions = handle_transaction_relay_actions(transaction_actions);
if suppress {
    return Ok(actions);
}

actions.push(PeerAction::ReceivedTransaction(transaction));
Ok(actions)
```

Then move the durable `known_txids` / `known_wtxids` and scheduler already-have transition behind the managed admission outcome, or add an explicit outcome callback that converts `Accepted` / `Replaced` to already-have and clears or avoids already-have for `Orphaned`, `Rejected`, and identity-mismatch paths. Add regressions for mismatch-then-second-peer-inv and orphan-disconnect-then-second-peer-inv.

### WR-02: Duplicate Orphan Parent Requests Lose Fallback Peers

**File:** `packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs:156`

**Issue:** `request_parent` suppresses a duplicate parent request when that relay id is already pending, but it does not retain the duplicate peer as a fallback candidate. Normal transaction announcements do retain fallback candidates before suppressing duplicates. As a result, if peer A is asked for an orphan parent and peer B later provides the same orphan-parent opportunity, peer B is discarded; when peer A times out, returns `notfound`, or disconnects, `schedule_relay` has no alternate peer to request from.

**Fix:**

```rust
if self.has_pending_relay(relay_id) {
    if self.peer_total_count(peer_id) < self.policy.max_announcements_per_peer {
        self.insert_candidate(relay_id, peer_id, now_unix_seconds, true);
    }
    return vec![TxDownloadAction::SuppressDuplicate { peer_id, relay_id }];
}
```

Add scheduler and managed-bridge tests showing duplicate orphan-parent requests from a second peer produce a fallback `GetData` after timeout, `notfound`, and disconnect cleanup.

### WR-03: Reconsideration Cap Can Leave Ready Orphans Without A Drain Path

**File:** `packages/open-bitcoin-network/src/peer/transaction_relay/orphanage.rs:231`

**Issue:** `reconsider_after_parent` inserts every newly ready orphan into `pending_reconsideration`, but only drains `max_reconsiderations_per_parent` entries and then returns. The remaining ready children have empty `missing_parents`, stay staged, and are only reconsidered if a later accepted parent happens to call `reconsider_after_parent` again. A parent with more than the capped number of children can therefore leave valid ready children stuck until expiry.

**Fix:** Add an explicit drain mechanism for pending ready children, or return a continuation action/status that the managed bridge can call on subsequent bounded ticks without requiring another parent acceptance. Cover it with a test where the cap is `2`, three children become ready from one parent, and the third child is reconsidered by the follow-up drain instead of expiring.

***

_Reviewed: 2026-07-01T05:18:15Z_
_Reviewer: the agent (gsd-code-reviewer)_
_Depth: standard_
