---
phase: 136-receive-independent-maintenance-and-transport-receipts
reviewed: 2026-08-16T06:15:00Z
depth: standard
files_reviewed: 51
files_reviewed_list:
  - packages/open-bitcoin-network/src/lib.rs
  - packages/open-bitcoin-network/src/peer.rs
  - packages/open-bitcoin-network/src/peer/transaction_relay.rs
  - packages/open-bitcoin-network/src/peer/transaction_relay/fanout.rs
  - packages/open-bitcoin-network/src/peer/transaction_relay/retry.rs
  - packages/open-bitcoin-network/src/peer/transaction_relay/tests/fanout_cases.rs
  - packages/open-bitcoin-node/src/network.rs
  - packages/open-bitcoin-node/src/network/announcement_transport.rs
  - packages/open-bitcoin-node/src/network/block_relay_evidence.rs
  - packages/open-bitcoin-node/src/network/inventory.rs
  - packages/open-bitcoin-node/src/network/lifecycle_projection/authority.rs
  - packages/open-bitcoin-node/src/network/lifecycle_projection/reconciliation.rs
  - packages/open-bitcoin-node/src/network/relay_fanout.rs
  - packages/open-bitcoin-node/src/network/relay_fanout/action_info.rs
  - packages/open-bitcoin-node/src/network/relay_fanout/lifecycle.rs
  - packages/open-bitcoin-node/src/network/relay_serving.rs
  - packages/open-bitcoin-node/src/network/runtime_authority.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/lifecycle.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/maintenance.rs
  - packages/open-bitcoin-node/src/network/tests.rs
  - packages/open-bitcoin-node/src/network/tests/announcement_transport_cases.rs
  - packages/open-bitcoin-node/src/network/tests/getdata_tx_receipt_cases.rs
  - packages/open-bitcoin-node/src/network/tests/lifecycle_projection_cases/reconciliation.rs
  - packages/open-bitcoin-node/src/network/tests/maintenance_tick_cases.rs
  - packages/open-bitcoin-node/src/network/tests/package_fanout_cases.rs
  - packages/open-bitcoin-node/src/network/tests/relay_fanout_cases.rs
  - packages/open-bitcoin-node/src/network/tests/relay_local_submission_cases.rs
  - packages/open-bitcoin-node/src/network/tests/relay_serving_cases.rs
  - packages/open-bitcoin-node/src/network/tests/unbroadcast_projection_cases.rs
  - packages/open-bitcoin-node/src/network/types.rs
  - packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/retry.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/retry.rs
  - packages/open-bitcoin-rpc/src/context.rs
  - packages/open-bitcoin-rpc/src/context/inbound_wire.rs
  - packages/open-bitcoin-rpc/src/context/network.rs
  - packages/open-bitcoin-rpc/src/dispatch.rs
  - packages/open-bitcoin-rpc/src/inbound_listener/connection_runtime.rs
  - packages/open-bitcoin-rpc/src/inbound_listener/tests.rs
  - packages/open-bitcoin-rpc/src/inbound_listener/tests/block_serving.rs
  - scripts/check-phase122-compact-relay-peer-completion.test.ts
  - scripts/check-phase122-compact-relay-peer-completion.ts
  - scripts/check-phase123-runtime-timing-evidence-integrity.test.ts
  - scripts/check-phase123-runtime-timing-evidence-integrity/constants.ts
  - scripts/check-phase123-runtime-timing-evidence-integrity/evidence.ts
  - scripts/check-phase126-compact-relay-residual-hardening.test.ts
  - scripts/check-phase126-compact-relay-residual-hardening.ts
  - scripts/check-phase127-authoritative-network-state-unification.test.ts
  - scripts/check-phase127-authoritative-network-state-unification.ts
  - scripts/check-phase128-production-compact-announcement-transport.ts
findings:
  critical: 0
  warning: 2
  info: 0
  total: 2
status: issues
---

# Phase 136: Code Review Report

**Reviewed:** 2026-08-16T06:15:00Z
**Depth:** standard
**Files Reviewed:** 51
**Status:** issues

## Summary

Phase 136 source from `7f618e7c` through `ef6206ee` was reviewed against the locked receipt, timer, and coupling rules. The membership-clear contracts hold: INV completion does not clear unbroadcast, classify-time EligibleServe does not mutate membership, `TransportWritten` is recorded only on a fresh Applied `TransactionResponse`, and a stale receipt does not clear a newer unbroadcast set. Pure `open-bitcoin-network` policy does not sample time or randomness, and the retry worker starts from `open-bitcoind` main beside the checkpoint worker rather than from `DurableSyncRuntime`. Production files in this range do not `unwrap()`.

The remaining defect is on the announcement write path. First-hop and retry INV are drained into `PeerEmission` values and then aborted, which removes them from the existing Phase 104 fanout queue and never writes them. A mid-drain error can also drop reserved peer-effect capabilities without abort.

## Warnings

### WR-01: Drained TX INV emissions are aborted instead of written

**File:** `packages/open-bitcoin-rpc/src/context/network.rs:496-499`
**Issue:** Production `sendrawtransaction` (`submit_local_transaction_with_relay_evidence_at`) drains first-hop fanout into owned `PeerEmission` values and immediately aborts the write capabilities. The retry worker does the same after every successful `maintenance_tick`. `drain_tx_fanout_emissions` consumes `drain_relay_fanout`, so those INV messages are removed from the Phase 104 queue and never reach `action_translation`'s existing receive-loop write path. D-07 says first announcement stays on the immediate fanout path; D-19 says retry uses existing outbox and successful-write receipts. After this change, local unbroadcast members stay retry-eligible forever unless a peer independently GETDATAs the transaction.
**Fix:** Deliver drained INV emissions through the existing announcement/socket write path, and abort only after a failed write. Until that path exists, do not drain-and-abort on the production submit or worker success path — leave queued INV in `TxFanoutQueue` for `drain_relay_fanout` in `action_translation.rs`.

```rust
// Keep enqueue-only on accept. Do not drain here.
let outcome = self.network.submit_local_transaction_outcome_at(
    transaction,
    self.verify_flags,
    self.consensus_params,
    now_unix_seconds,
    relay_intent,
)?;
Ok(outcome)
```

```rust
// Worker: write each emission, then complete or abort.
match handle.maintenance_tick(context) {
    Ok(outcome) => {
        for emission in outcome.emissions {
            // write INV bytes; complete_peer_emission on success,
            // abort_peer_emission on failure
        }
        next_wait = next_wait_duration(
            observed_at_unix_seconds,
            outcome.maybe_next_due_unix_seconds,
        );
    }
    Err(error) => {
        eprintln!("open-bitcoind initial-broadcast retry tick failed: {error}");
    }
}
```

Same abort loop: `packages/open-bitcoin-rpc/src/bin/open_bitcoind/retry.rs:122-125`.

### WR-02: Mid-drain PeerEmission drop leaks pending write reservations

**File:** `packages/open-bitcoin-node/src/network/runtime_authority/maintenance.rs:182-200`
**Issue:** `drain_tx_fanout_emissions_locked` reserves a `PeerEffectCapability` per drained INV. If `try_new_tx_inventory` returns `None`, the capability is dropped. `PeerEffectCapability` has no `Drop` abort, so the reservation stays in `PeerEffectLedger` until capacity (`MAX_PENDING_PEER_EFFECTS`) is exhausted. Returning `Err` after some emissions were collected drops those `PeerEmission` values the same way. Later GETDATA `PreparedTxServe` and compact announcement reserves then fail closed.
**Fix:** Abort any capability that does not become a returned emission, and abort already-collected emissions before returning an error.

```rust
if let Some(emission) =
    PeerEmission::try_new_tx_inventory(peer_id, message, member, capability)
{
    emissions.push(emission);
} else {
    apply_lifecycle_command(network, LifecycleCommand::AbortPeerEffect(capability))
        .map_err(ManagedNetworkAuthorityError::from)?;
}
```

On `PrepareRelay` / abort-mismatch `Err`, abort every emission already in `emissions` before returning.

## Focus-area results

| Check | Result |
| --- | --- |
| Unbroadcast clear on INV | Pass. `apply_fresh_tx_response_transport_written` returns unless `is_transaction_response()`. INV completion test keeps membership. |
| EligibleServe pre-write clear | Pass. Durable GETDATA prepare stores `PreparedTxServe` and does not remove members. Projection rustdoc forbids `EligibleServe` rows. |
| Timer in pure crates | Pass. No `SystemTime` / `getrandom` in `open-bitcoin-network`. Cycle math uses injected `RetryDecisionContext`. |
| DurableSyncRuntime coupling | Pass. `start_initial_broadcast_retry_worker` is called from `open-bitcoind` main beside the checkpoint worker. |
| `unwrap` in production | Pass. No `unwrap()` / `expect()` in the production receipt, timer, inventory, or retry files. |
| Missing TransportWritten record | Pass on the durable GETDATA path. Fresh Applied TX-response writes record `MempoolRetryClearCause::TransportWritten` beside `unbroadcast_members.remove`. |
| Stale receipt vs newer unbroadcast | Pass. `is_fresh` requires matching authority epoch, lifecycle generation, and peer session. Stale completion is `AchievedButStale` and does not clear. |

---

_Reviewed: 2026-08-16T06:15:00Z_
_Reviewer: Claude (gsd-code-reviewer)_
_Depth: standard_
