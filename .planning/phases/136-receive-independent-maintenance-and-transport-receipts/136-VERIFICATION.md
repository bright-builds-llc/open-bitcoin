---
phase: 136-receive-independent-maintenance-and-transport-receipts
verified: 2026-08-16T05:55:00Z
status: passed
score: 4/4 must-haves verified
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: 136-2026-08-15T21-24-17
generated_at: 2026-08-16T21:44:33.000Z
lifecycle_validated: true
overrides_applied: 0
---

# Phase 136: Receive-Independent Maintenance and Transport Receipts Verification Report

**Phase Goal:** Idle and active nodes perform bounded initial broadcast retry and ordinary topological package fanout through existing activation, peer-policy, queue, serving, and transport controls.
**Verified:** 2026-08-16T05:55:00Z
**Status:** passed
**Re-verification:** No — initial verification

## Goal Achievement

The roadmap contract is four success criteria (PPKG-04, IBR-01, IBR-02, IBR-03, IBR-04). Plan frontmatter truths add detail but do not subtract those criteria. Locked decisions (TransportWritten-only clear, 10–15 minute cycle with inspect=256 / prepare=32 leftover cursor, parent-before-child enqueue, open-bitcoind shell timer, injected now+jitter) hold in the live code.

### Observable Truths

| #   | Truth   | Status     | Evidence       |
| --- | ------- | ---------- | -------------- |
| 1   | Only bounded, locally submitted, relay-requested, still-present transactions enter initial broadcast retry; the whole mempool never becomes a retry set. | ✓ VERIFIED | Insert gate is `delta().admitted ∩ is_retry_eligible(true)` (`local` + `Requested` + current). `select_maintenance_identities` and `maintenance_tick` walk `unbroadcast_members` only. No `mempool.entries()` walk in retry policy or the tick. Cap remains `MAX_UNBROADCAST_MEMBERS` (5,000). Tests: `local_requested_admission_inserts_unbroadcast_member`, `not_requested_or_peer_admission_does_not_insert_unbroadcast_member`, `maintenance_tick_walks_only_unbroadcast_members`. |
| 2   | Receive-independent maintenance schedules a fresh injected 10-to-15-minute retry cycle and caps decisions and emissions per tick. | ✓ VERIFIED | Cycle is `600 + injected jitter` (`0..=300`). Production inspect/prepare are 256/32, independent of PHASE104 16/1024. Cursor leftover walk remints due time from `RetryDecisionContext`. No `SystemTime`/`getrandom` in `open-bitcoin-network` or `maintenance.rs`. Shell timer lives in `open-bitcoind` (`start_initial_broadcast_retry_worker` beside checkpoint; not `DurableSyncRuntime`). Recovery and handle construction remint due/cursor to `None`. Tests: cycle/budget/cursor unit suite, seven `maintenance_tick_*` tests, four `retry_worker_*` fake-clock tests. |
| 3   | Surviving package members fan out parent-before-child and retries use the existing relay activation, peer eligibility, txid/wtxid selection, rate, outbox, serving, and successful-receipt path. | ✓ VERIFIED | `prepare_fanout_projection` enqueues `facts.final_present()` (admitted topological order). AlreadyPresent parents are not in that list. `enqueue_retry_admissions` calls `record_prepared_admission` / `TxFanoutQueue::enqueue_admission` in caller order. No `rebroadcast.rs`. GETDATA TX is `PreparedTxServe` → encode → `complete_peer_emission`. Tests: four `package_fanout_*` tests, three `enqueue_retry_admissions_*` tests, `local_accept_plus_drain_prepares_first_hop_inv_without_advancing_ten_minutes`. |
| 4   | Unbroadcast membership clears only at the documented eligible serve or successful-write receipt, or on authoritative lifecycle removal, and supported restart behavior never claims guaranteed propagation. | ✓ VERIFIED | Applying receipt is fresh `EffectCompletion::Applied` + `is_transaction_response()` → `MempoolRetryClearCause::TransportWritten` beside `unbroadcast_members.remove`. INV (`TransactionInventory`) does not clear. `EligibleServe` is semantic only and is not written into `retry_clears`. Teardown still removes. Cleared still-present members are not re-inserted and are not reconciliation mismatches. Rustdoc forbids public/default relay and guaranteed propagation. Tests: five `getdata_tx_receipt_*` tests, insert-gate and subset-oracle suites. |

**Score:** 4/4 truths verified

Plan-specific truths that restate these criteria (cycle length, 256/32 budgets, leftover cursor, insert-on-admission, TX write kinds, PreparedTxServe, parent-before-child FIFO, shell timer, first-hop prepare) were checked in code and are covered by the rows above.

### Required Artifacts

| Artifact | Expected    | Status | Details |
| -------- | ----------- | ------ | ------- |
| `packages/open-bitcoin-network/src/peer/transaction_relay/retry.rs` | Cycle length, due-time, inspect/prepare budgets, leftover cursor | ✓ VERIFIED | `RETRY_CYCLE_BASE_SECONDS=600`, `MAINTENANCE_INSPECT_BUDGET=256`, `MAINTENANCE_PREPARE_BUDGET=32`, `select_maintenance_identities`. No `SystemTime`/`getrandom`. gsd-tools artifacts: passed. |
| `packages/open-bitcoin-network/src/peer/transaction_relay.rs` | Re-exports of retry policy types | ✓ VERIFIED | Re-exports `select_maintenance_identities` and budget/cycle types. Also re-exported from `peer.rs` and `lib.rs`. |
| `packages/open-bitcoin-node/src/network/lifecycle_projection/authority.rs` | Insert-only-on-new-admission unbroadcast projection | ✓ VERIFIED | Insert loop requires `facts.delta().admitted.contains` and `is_retry_eligible(true)`. Teardown and `retry_clears` remain the exits. Recovery remints timer fields to `None`. |
| `packages/open-bitcoin-node/src/network/lifecycle_projection/reconciliation.rs` | Subset oracle for unbroadcast after TX-write clears | ✓ VERIFIED | `expected_unbroadcast_members` is live set ∩ retry-eligible canonical members. Missing still-present members are not mismatches. |
| `packages/open-bitcoin-node/src/network/announcement_transport.rs` | TX-specific PeerEmission constructors and write kinds | ✓ VERIFIED | `TransactionInventory` / `TransactionResponse`. `try_new_tx_inventory` / `try_new_tx_response`. Compact `for_message` still rejects `Tx`. |
| `packages/open-bitcoin-node/src/network/block_relay_evidence.rs` | Compact evidence only when a compact reason is present | ✓ VERIFIED | `record_peer_emission` calls `record_announcement` only inside `if let Some(reason) = evidence.maybe_evidence_reason()`. |
| `packages/open-bitcoin-node/src/network/inventory.rs` | GETDATA TX promoted to `PreparedTxServe(PeerEmission)` | ✓ VERIFIED | `try_prepare_tx_response_item` builds `try_new_tx_response` and pushes `PreparedTxServe`. No `Immediate(Tx)` in the durable GETDATA path. |
| `packages/open-bitcoin-node/src/network/runtime_authority/lifecycle.rs` | Applied-only TransportWritten membership mutation | ✓ VERIFIED | `apply_fresh_tx_response_transport_written` runs only on fresh Applied TX-response. No `EligibleServe` insert. |
| `packages/open-bitcoin-node/src/network/relay_fanout.rs` | Retry identity enqueue through existing admission path | ✓ VERIFIED | `enqueue_retry_admissions` iterates the supplied slice and calls `record_prepared_admission`. |
| `packages/open-bitcoin-node/src/network/tests/package_fanout_cases.rs` | Parent-before-child and AlreadyPresent-parent proofs | ✓ VERIFIED | Four named tests registered in `network/tests.rs`. All passed. |
| `packages/open-bitcoin-node/src/network/runtime_authority/maintenance.rs` | Handle-owned maintenance tick and TX INV drain | ✓ VERIFIED | Walks `unbroadcast_members` with production 256/32. Stores last prepared identity as cursor. No receive-loop or DurableSync symbols. |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/retry.rs` | Checkpoint-style shutdown-aware retry worker | ✓ VERIFIED | `start_initial_broadcast_retry_worker` + injectible `wait`/`now`/`jitter`. Production jitter uses `getrandom`; failure is `JitterUnavailable`, not silent `0`/`300`. |

gsd-tools `verify artifacts` reported `all_passed: true` for all six plans (12/12 artifacts).

### Key Link Verification

| From | To  | Via | Status | Details |
| ---- | --- | --- | ------ | ------- |
| `transaction_relay.rs` | `retry.rs` | `pub use` of cycle, budget, and cursor types | ✓ WIRED | `select_maintenance_identities` re-exported. |
| `authority.rs` | `mempool/context.rs` | `is_retry_eligible` used only as the insert gate | ✓ WIRED | Insert requires admitted ∩ eligible. |
| `block_relay_evidence.rs` | `announcement_transport.rs` | `record_peer_emission` skips compact counters when reason is `None` | ✓ WIRED | `maybe_evidence_reason` gated. |
| `connection_runtime.rs` | runtime authority effects | `complete_peer_emission` after successful TX write | ✓ WIRED | `acknowledge_inbound_response_write` completes on `Written`, aborts otherwise. `inbound_wire.rs` resolve also carries `maybe_tx_write_capability`. |
| `relay_fanout.rs` | `fanout.rs` | `TxFanoutQueue::enqueue_admission` `push_back` | ✓ WIRED | `record_prepared_admission` → `enqueue_admission`. |
| `open_bitcoind/retry.rs` | `runtime_authority.rs` | `maintenance_tick` with shell-sampled now and jitter | ✓ WIRED | Worker builds `RetryDecisionContext` and calls `handle.maintenance_tick`. |

gsd-tools `verify key-links` reported `all_verified: true` for all six plans (6/6 links).

### Data-Flow Trace (Level 4)

These artifacts are protocol/control-plane, not UI. Trace is the unbroadcast → tick → fanout → receipt path.

| Artifact | Data Variable | Source | Produces Real Data | Status |
| -------- | ------------- | ------ | ------------------ | ------ |
| `maintenance.rs` | `unbroadcast_members` / `selection.prepare` | Live `BTreeSet` on `ManagedPeerNetwork`; `select_maintenance_identities` | Yes — caller-supplied unbroadcast set, not hardcoded | ✓ FLOWING |
| `relay_fanout.rs` | `TxFanoutAdmission` identities | `enqueue_retry_admissions` slice and `facts.final_present()` | Yes — admitted members in topological order | ✓ FLOWING |
| `inventory.rs` / `inbound_wire.rs` | `PreparedTxServe` / `maybe_tx_write_capability` | Served GETDATA TX + `try_new_tx_response` | Yes — real transaction bytes and affine capability | ✓ FLOWING |
| `lifecycle.rs` | `MempoolRetryClearCause::TransportWritten` | Fresh Applied TX-response receipt | Yes — member identity from emission evidence | ✓ FLOWING |

Note: the open-bitcoind worker and the RPC local-submit wrapper abort unused TX INV write capabilities after drain so reservations do not leak. Plan 06 scoped first-hop to **prepare** (`drain_tx_fanout_emissions`), not socket write. GETDATA TX write/receipt is the complete transport path. Existing `action_translation` still drains peer-originated fanout to outbound messages.

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
| -------- | ------- | ------ | ------ |
| Cycle + leftover cursor policy | `cargo test -p open-bitcoin-network --lib` filters `select_maintenance_identities`, `retry_cycle`, `next_retry_due`, `maintenance_budgets`, `production_inspect` | 5 + 2 + 1 + 1 + 1 passed | ✓ PASS |
| Insert gate, subset oracle, GETDATA receipts | `cargo test -p open-bitcoin-node --lib unbroadcast` | 25 passed including five `getdata_tx_receipt_*` and five insert-gate tests | ✓ PASS |
| Parent-before-child FIFO | `cargo test -p open-bitcoin-node --lib package_fanout` | 4 passed | ✓ PASS |
| Retry enqueue on Phase 104 path | `cargo test -p open-bitcoin-node --lib enqueue_retry_admissions` | 3 passed | ✓ PASS |
| Maintenance tick + first-hop prepare | `cargo test -p open-bitcoin-node --lib maintenance_tick` | 7 passed | ✓ PASS |
| TX INV vs compact evidence | `tx_inventory_emission`, `compact_inventory_emission`, `tx_response_emission`, `compact_constructor_rejects` | 4 passed | ✓ PASS |
| Shell timer fake-clock | `cargo test -p open-bitcoin-rpc --bin open-bitcoind retry_worker` | 4 passed | ✓ PASS |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
| ----------- | ---------- | ----------- | ------ | -------- |
| **IBR-01** | 01, 02, 06 | Track only bounded locally submitted, relay-requested, still-present txs; never the whole mempool | ✓ SATISFIED | Insert gate + `unbroadcast_members` walk + 5,000 cap. Plans 01/02/06 claim it. |
| **IBR-02** | 01, 06 | Receive-independent 10-to-15-minute cycles from injected inputs; cap work and emissions per tick | ✓ SATISFIED | Pure cycle/budgets + `maintenance_tick` + open-bitcoind worker. Plans 01/06 claim it. |
| **IBR-03** | 03, 05, 06 | Retry uses existing activation, eligibility, txid/wtxid, rate, outboxes, serving, receipts — no parallel fanout | ✓ SATISFIED | TX PeerEmission kinds, `enqueue_retry_admissions`, `PreparedTxServe`, no `rebroadcast.rs`. Plans 03/05/06 claim it. |
| **IBR-04** | 02, 04, 06 | Clear only at documented serve/write receipt or lifecycle removal; restart remint; no guaranteed-propagation claim | ✓ SATISFIED | TransportWritten on fresh TX write; EligibleServe does not clear; recovery remints timers; rustdoc claim boundary. Plans 02/04/06 claim it. |
| **PPKG-04** | 05 | Accepted still-present package members enter existing serving/fanout parent-before-child under existing controls | ✓ SATISFIED | `final_present()` FIFO + package_fanout tests. Plan 05 claims it. |

No orphaned Phase 136 requirement IDs. REQUIREMENTS.md maps exactly these five IDs to Phase 136. Every ID appears in at least one plan `requirements:` field.

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
| ---- | ---- | ------- | -------- | ------ |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/retry.rs` | 122–125 | Worker aborts drained TX INV `PeerEmission` capabilities after `maintenance_tick` | ℹ️ Info | Plan 06 scoped first-hop to prepare, not socket write. Prevents leaked write reservations. GETDATA TX write path is complete. |
| `packages/open-bitcoin-rpc/src/context/network.rs` | 496–499 | RPC local-submit wrapper drains then aborts unused INV capabilities | ℹ️ Info | Same prepare-only seam. `submit_local_transaction_outcome_at` stays enqueue-only so existing receive-loop drain still works. |
| `packages/open-bitcoin-node/src/network/announcement_transport.rs` | 107 | Stale `#[cfg_attr(not(test), allow(dead_code))]` on `try_new_tx_inventory` | ℹ️ Info | Production `maintenance.rs` now calls it. Allow is leftover from Plan 03, not a stub. |

No TODO/FIXME/placeholder implementations in the phase production files. No second announcer module. No `EligibleServe` membership-clear emission.

### Human Verification Required

None. Behavior is protocol/control-plane and is covered by fake-clock unit tests. No visual, public-network, or wall-clock gate is required for this phase.

### Gaps Summary

No gaps. All four roadmap success criteria hold in the codebase. Locked decisions match the implementation: unbroadcast clears only on successful TX write (`TransportWritten`) or `LifecycleRemoval`; the cycle is process-global 10–15 minutes with inspect=256 / prepare=32 and a leftover cursor; co-admitted packages enqueue parent-before-child; the timer lives in `open-bitcoind` with injected now+jitter.

---

_Verified: 2026-08-16T05:55:00Z_
_Verifier: Claude (gsd-verifier)_
