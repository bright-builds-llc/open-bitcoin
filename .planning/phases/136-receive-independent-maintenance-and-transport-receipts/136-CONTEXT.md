---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 136-2026-08-15T21-24-17
generated_at: 2026-08-15T21:27:40.831Z
---

# Phase 136: Receive-Independent Maintenance and Transport Receipts - Context

**Gathered:** 2026-08-15
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Idle and active nodes perform bounded initial-broadcast retry and ordinary
topological package fanout through existing relay activation, peer eligibility,
queue, serving, rate, outbox, and achieved-effect transport paths.

This phase delivers PPKG-04 and IBR-01 through IBR-04. It does not add a package
wire protocol, arbitrary multi-parent assembly, whole-mempool rebroadcast,
public/default/production relay, guaranteed propagation, public-network CI,
broad RPC/dashboard/support presentation (Phase 137), or production-readiness
claims (Phase 138).

</domain>

<decisions>
## Implementation Decisions

### Unbroadcast-clear receipt

- **D-01:** Never clear unbroadcast membership on an inventory write, inventory
  queue, or INV successful-write receipt. An INV is announcement, not
  acknowledgement.
- **D-02:** The applying receipt that may clear a still-present local
  unbroadcast member is a successful write of the transaction response
  (`WireNetworkMessage::Tx`), recorded as `MempoolRetryClearCause::TransportWritten`
  through the existing Phase 134 `acknowledge_write` / `CompletePeerEmission`
  path. Failed encode, reject, disconnect, or write abort leaves the member
  retry-eligible.
- **D-03:** `EligibleServe` remains the documented Knots-equivalent semantic
  (eligible GETDATA found the transaction and a TX serve was classified). It
  must not emit a membership-clearing retry-clear before the TX write succeeds.
  Existing precedence stays `LifecycleRemoval > TransportWritten > EligibleServe`.
- **D-04:** Authoritative lifecycle removal (confirmation, replacement, expiry,
  eviction, conflict, or absence from the mempool) still clears unbroadcast
  immediately via `LifecycleRemoval`. A successful TX write never claims
  network-wide or guaranteed propagation.

### Retry cycle timing

- **D-05:** Use one process-global retry cycle, not per-member due times. The
  first cycle is a fresh injected 10-to-15-minute window from node start or
  recovery install. Each completed tick schedules the next cycle the same way.
- **D-06:** Restart and recovery remint a fresh injected cycle. Do not persist
  or restore leftover due times; derived retry timers are not durable source
  state.
- **D-07:** First announcement of a newly accepted local transaction remains
  the existing immediate fanout path. The retry timer never replaces that first
  hop. A member admitted just after a tick waits until the next cycle for retry.
- **D-08:** Cycle length is the existing Phase 130 contract: 10-minute base plus
  injected `RetryJitterSeconds` in `0..=300`. The shell supplies
  `RetryDecisionContext { observed_at_unix_seconds, jitter }`; pure policy never
  samples time or randomness.

### Per-tick work caps

- **D-09:** The unbroadcast set itself remains the IBR-01 membership bound
  (existing 5,000-member persisted-input / runtime cap). That cap is not the
  per-tick work budget.
- **D-10:** Each receive-independent maintenance tick has a separate inspect
  budget (max identities considered) and prepare budget (max emissions prepared).
  Prepared identities then enter the existing Phase 104 fanout path and still
  honor `PHASE104_MAX_TX_FANOUT_QUEUE_PER_PEER` and
  `PHASE104_MAX_TX_FANOUT_DRAIN_PER_PEER`.
- **D-11:** Identities past the inspect or prepare budget spill to the next
  injected cycle as still-due and unattempted. Queue-cap, rate-limit, and
  suppression labels must not be recorded as attempts for members the tick
  never prepared.
- **D-12:** Walk leftover members with a deterministic cursor so the tail is
  not starved across cycles. Do not add a parallel fanout path.

### Parent-before-child package fanout

- **D-13:** Accepted, still-present members that were admitted together as one
  package enter the existing per-peer fanout queue in parent-before-child order.
  Drain, rate, identity (txid/wtxid), activation, and peer-eligibility controls
  stay on the existing path.
- **D-14:** Independently admitted parents or children use ordinary single-
  transaction fanout. Do not re-announce an already-present parent merely
  because a later child was accepted.
- **D-15:** Do not require emit/write order to be parent-before-child across
  peers or ticks. Rate and drain may split a pair across ticks. Do not walk
  unrelated mempool ancestors or add a package wire message.

### Maintenance wakeup

- **D-16:** A shell-owned timer in the node/daemon runtime wakes even when no
  peer messages arrive and issues one `MaintenanceTick { now, jitter }` into
  `ManagedNetworkHandle`. Clocks and randomness stay in the shell.
- **D-17:** Do not piggyback retry scheduling on inbound or outbound message
  loops. Do not couple the retry timer to `DurableSyncRuntime` or IBD.
- **D-18:** Production samples `SystemTime` plus a fallible jitter source in
  the shell. Tests inject deterministic `now` and jitter with no sleeps. Default
  verification must stay hermetic: no wall-clock or public-network gates.
- **D-19:** Retry uses existing relay activation, peer eligibility, txid/wtxid
  selection, rate limits, bounded outboxes, serving, and successful-write
  receipts. Relay-disabled or ineligible operation may admit locally while
  emitting no public fanout and clearing unbroadcast only by lifecycle removal
  or a later eligible TX serve.

### Claude's Discretion

- Exact inspect-N and prepare-M constants, provided they are documented,
  fake-clock-assertable, smaller than the 5,000-member set cap, and do not
  silently treat PHASE104 queue/drain caps as the only IBR-02 bound.
- Exact module, command, cursor, and receipt-filter names, provided TX-only
  write completion remains the applying unbroadcast-clear receipt.
- Where the shell timer lives (`open-bitcoind` versus node runtime adapter),
  provided it is shutdown-aware, not a second policy authority, and not a
  public-default relay loop.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Active milestone contract

- `.planning/ROADMAP.md` — Phase 136 goal, success criteria, and separation from Phases 137 and 138.
- `.planning/REQUIREMENTS.md` — PPKG-04, IBR-01, IBR-02, IBR-03, IBR-04, and v2.2 exclusions.
- `.planning/research/ARCHITECTURE.md` — `MaintenanceTick` command/delta/receipt model; inventory write is not acknowledgement; clear on eligible GETDATA serve / TX write, never on INV alone.
- `.planning/research/FEATURES.md` — Bounded local-unbroadcast retry every randomized 10–15 minutes until a peer requests the transaction.
- `.planning/research/STACK.md` — Shell timer, existing fanout reuse, and no rolling-fee or derived-timer persistence.
- `.planning/research/PITFALLS.md` — Unbounded maintenance that starves peer/RPC work after a large recovered or idle wakeup.
- `.planning/research/SUMMARY.md` — Synthesized v2.2 sequencing and retry/fanout conclusions.

### Prior locked decisions

- `.planning/phases/100-relay-activation-boundary-and-permission-semantics/100-CONTEXT.md` — Default-off relay activation and peer eligibility.
- `.planning/phases/104-relay-serving-fanout-and-rebroadcast-policy/104-CONTEXT.md` — Serving cache, fanout queues, and deferred `rebroadcast_deferred` boundary that this phase replaces with scheduled retry.
- `.planning/phases/130-resource-time-and-fee-primitives/130-CONTEXT.md` — Retry eligibility (local + requested + current member), injected jitter, and retry-clear causes.
- `.planning/phases/133-package-aware-download-and-orphan-bridge/133-CONTEXT.md` — Ordinary-message 1P1C only; parent-before-child fanout deferred here.
- `.planning/phases/134-authoritative-cross-cache-lifecycle-integration/134-CONTEXT.md` — Sole `ManagedNetworkHandle` authority, family-specific receipts, successful-write completion.
- `.planning/phases/135-snapshot-schema-checkpointing-and-recovery/135-CONTEXT.md` — Durable unbroadcast membership without derived retry timers.

### Pinned Bitcoin Knots behavior

- `packages/bitcoin-knots/src/net_processing.cpp` — `ReattemptInitialBroadcast` (10min + rand 5min, whole unbroadcast walk, `RelayTransaction`) and `RemoveUnbroadcastTx` on eligible GETDATA TX push (~2457).
- `packages/bitcoin-knots/src/node/transaction.cpp` — Local submission unbroadcast marking.
- `packages/bitcoin-knots/src/txmempool.cpp` — Unbroadcast insert/remove and mempool-absent cleanup.
- `packages/bitcoin-knots/src/node/mempool_persist.cpp` — Persisted unbroadcast membership without derived timers.
- `packages/bitcoin-knots/test/functional/mempool_unbroadcast.py` — Local unbroadcast, retry, GETDATA acknowledgement, and restart remint.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- `packages/open-bitcoin-network/src/peer/transaction_relay/retry.rs` — `RetryJitterSeconds` (`0..=300`) and `RetryDecisionContext`.
- `packages/open-bitcoin-network/src/peer/transaction_relay/fanout.rs` — Existing announce/suppress/queue-cap/rate-limit/cleanup actions and PHASE104 bounds.
- `packages/open-bitcoin-mempool/src/context.rs` — `MempoolEntryMetadata::is_retry_eligible`.
- `packages/open-bitcoin-mempool/src/pool/lifecycle.rs` — `MempoolRetryClearCause::{LifecycleRemoval, EligibleServe, TransportWritten}`.
- `packages/open-bitcoin-node/src/network/lifecycle_projection.rs` — `LifecycleCommand::Maintenance` and unbroadcast projection.
- `packages/open-bitcoin-node/src/network/relay_fanout.rs` — Managed fanout and current `rebroadcast_deferred` evidence.
- `packages/open-bitcoin-node/src/network/relay_serving.rs` — GETDATA serving classification used for EligibleServe semantics.

### Established Patterns

- Pure policy receives injected time and jitter; the shell owns clocks, randomness, and transport.
- `ManagedNetworkHandle` is the sole mutation authority. Adapters capture work, release the lock before I/O, and complete typed receipts afterward.
- Shared evidence is fixed-label and redacted. Identities stay in authenticated direct responses.
- Phase 134 receipts are family-specific and non-replayable. Stale receipts must not clear newer unbroadcast intent.

### Integration Points

- `packages/open-bitcoin-node/src/network/runtime_authority/lifecycle.rs` — Dispatch `LifecycleCommand::Maintenance`.
- `packages/open-bitcoin-node/src/network/lifecycle_projection/reconciliation.rs` — Expected unbroadcast membership is the retry-eligible canonical subset.
- `packages/open-bitcoin-rpc/src/bin/open_bitcoind/` — Shell timer / shutdown-aware wakeup candidate.
- `packages/open-bitcoin-node/src/storage/mempool_snapshot.rs` — Recovered unbroadcast membership with no persisted due times.
- `docs/parity/catalog/` — Register retry and parent-before-child fanout anchors without claiming public relay or guaranteed propagation.

</code_context>

<specifics>
## Specific Ideas

- Match Knots observable retry cadence and GETDATA-serve acknowledgement while keeping Open Bitcoin's successful-write receipt integrity (clear only after the TX bytes are written, not after send-buffer push).
- Replace `rebroadcast_deferred` as the live scheduling story. Keep the label only if it remains truthful for relay-disabled or not-yet-due cases; do not imply a timer was added in Phase 104.
- Research architecture sketched `rebroadcast.rs` and `relay_rebroadcast.rs`; reuse those names only if they stay on the existing fanout/receipt path.

</specifics>

<deferred>
## Deferred Ideas

- Broad RPC, dashboard, metrics, logs, and support presentation of retry/fanout evidence — Phase 137.
- Integrated Knots parity catalog closeout, adversarial pressure, and claim guardrails — Phase 138.
- Whole-mempool or wallet-wide periodic rebroadcast — out of v2.2 scope.
- General package wire protocol or helpful ancestor re-announce beyond co-admitted package members — out of v2.2 scope.
- Public/default/production relay or guaranteed-propagation claims — permanently deferred for this milestone.

None — discussion stayed within phase scope.

</deferred>

---

*Phase: 136-receive-independent-maintenance-and-transport-receipts*
*Context gathered: 2026-08-15*
