---
generated_by: gsd-phase-researcher
lifecycle_mode: yolo
phase_lifecycle_id: 108-2026-07-03T14-09-06
generated_at: 2026-07-03T14:09:06.388Z
status: complete
recovered_by_orchestrator: true
---

# Phase 108: Durable Mempool Relay State Recovery - Research

## RESEARCH COMPLETE

Phase 108 should be planned as four connected slices: managed recovery replay, lifecycle coherence after restart, sanitized operator evidence, and deterministic parity/guardrail closeout. The key implementation risk is that the existing durable snapshot replay repopulates only the pure mempool; it does not yet rehydrate the relay-serving cache, fanout identity state, or sanitized recovered-state evidence.

This artifact was recovered locally after the delegated research agent stalled without writing `108-RESEARCH.md`. The findings are based on the Phase 108 context, prior Phase 100-107 artifacts, and targeted code inspection.

## Scope Anchors

Phase 108 owns:

- **MEM-04:** block connect removes confirmed and conflicting transactions from mempool and relay-serving caches.
- **MEM-05:** block disconnect or reorg handling reconsiders eligible disconnected transactions within the documented v2.0 boundary.
- **MEM-06:** durable mempool persistence saves accepted transaction state and recovers or repairs stale, corrupt, or incompatible records safely on restart.
- **REL-01:** node serves only relay-eligible transactions in response to peer `getdata` requests and reports unknown, stale, confirmed, rejected, or evicted transactions correctly.
- **REL-02:** node announces accepted transactions to eligible peers using negotiated txid or wtxid identity, per-peer queues, rate limits, and suppression rules.

It must not add public transaction relay by default, compact block relay, package relay, bloom/filter serving, public-network relay CI, production service operation, production full-node readiness, production-funds wallet safety, destructive repair, or public propagation guarantees.

## Existing Open Bitcoin Seams

### Durable Snapshot Replay

- `packages/open-bitcoin-node/src/storage/mempool_snapshot.rs` defines `MempoolSnapshot`, `MempoolSnapshotRecord`, `MempoolRecoveryStatus`, and `MempoolSnapshot::replay_into_mempool`.
- `MempoolRecoveryStatus` already has stable labels: `recovered`, `dropped_confirmed`, `dropped_duplicate`, `dropped_missing_parent`, `dropped_policy_incompatible`, and `dropped_evicted`.
- `replay_into_mempool` accepts records back into `open_bitcoin_mempool::Mempool`, but it does not notify `RelayServingCache`, `ManagedRelayFanoutState`, `RelayEvidenceStatus`, RPC, CLI, dashboard, metrics, logs, or support surfaces.

Planning implication: add a managed replay wrapper around the pure replay result. Accepted `Recovered` records need to call the same managed admission/serving/fanout helpers used by live `MempoolOutcome::Accepted` or `MempoolOutcome::Replaced`.

### Fjall Storage

- `packages/open-bitcoin-node/src/storage/fjall_store/mempool.rs` already exposes `save_mempool_snapshot`, `load_mempool_snapshot`, and `clear_mempool_snapshot`.
- `packages/open-bitcoin-node/src/storage/snapshot_codec.rs` already has versioned DTO conversion for mempool snapshots.
- `packages/open-bitcoin-node/src/storage/fjall_store/tests.rs` already covers save/load/clear/corrupt snapshot behavior.

Planning implication: Phase 108 should not invent a second storage namespace or peer-specific persistence model. Persist accepted transaction state only, then derive runtime relay state on recovery.

### Managed Relay Serving

- `RelayServingCache` in `packages/open-bitcoin-node/src/network/relay_serving.rs` owns `records_by_txid`, txid/wtxid indexes, status indexes, and latest serve outcomes.
- `record_accepted`, `record_replaced`, `record_status`, and `remove_transactions` are the core cache mutation operations.
- `classify_request` already maps `getdata` requests through peer relay mode, relay eligibility, known status, and typed serve outcomes such as `served`, `unknown`, `stale`, `confirmed`, `replaced`, `evicted`, `expired`, `identity_mismatch`, and `not_relay_eligible`.

Planning implication: recovered accepted records should enter `RelayServingCache::record_accepted`; dropped or invalid records should record a non-serveable status only when that evidence is useful and safe.

### Managed Relay Fanout

- `ManagedRelayFanoutState` in `packages/open-bitcoin-node/src/network/relay_fanout.rs` owns `wtxids_by_txid`, per-peer fanout queue, recent rejects, latest fanout actions, and latest local submission evidence.
- `record_admission_outcome` maps `MempoolOutcome::Accepted` and `MempoolOutcome::Replaced` into fanout admissions and maps eviction/expiry into cleanup actions.
- `cleanup_transactions` removes txid/wtxid identity state and queues cleanup actions.

Planning implication: recovery replay should avoid actual socket I/O and duplicate `inv` fanout at startup. It may need a recovery-specific path that seeds identity state and aggregate evidence without draining announcements, or it may reuse `record_admission_outcome` with a no-drain/no-socket flag if that keeps behavior explicit.

### Lifecycle Cleanup

- `packages/open-bitcoin-node/src/network/mempool_lifecycle.rs` already wires block connect and reorg handling into mempool cleanup plus serving/fanout cleanup.
- `apply_connected_block_mempool_lifecycle` removes confirmed transactions from mempool and calls `remove_stored_transactions_with_status(..., Confirmed)`.
- `apply_reorg_mempool_lifecycle` reconnects replacement branch blocks, reconsiders disconnected transactions, and applies accepted/replaced outcomes through `apply_admitted_outcome`.

Planning implication: tests should prove the same cleanup methods remove recovered records from serving/fanout state after restart.

### Status, RPC, CLI, Dashboard, Metrics, Logs, And Support

- `packages/open-bitcoin-node/src/status/relay_evidence.rs` defines `RelayEvidenceStatus`, `RelayEvidenceCounters`, and fixed capability fields.
- `packages/open-bitcoin-node/src/status.rs` embeds `MempoolStatus { transactions, relay }` inside `OpenBitcoinStatusSnapshot`.
- `packages/open-bitcoin-rpc/src/dispatch/node.rs` projects `openbitcoinnetworkstatus` and keeps baseline `getmempoolinfo`, `getnetworkinfo`, and `sendrawtransaction` narrow.
- `packages/open-bitcoin-cli/src/operator/status/render/relay.rs`, `dashboard/model/relay.rs`, `support/render/relay.rs`, and `support/redaction.rs` already consume and sanitize relay evidence.

Planning implication: if recovered relay evidence needs new fields, add fixed aggregate fields only and extend redaction tests. Do not put txids, wtxids, peer ids, endpoints, permission strings, or dynamic labels in status/support/logs.

## Knots Anchors To Follow Or Deliberately Defer

- `packages/bitcoin-knots/src/node/mempool_persist.cpp` and `.h`: `LoadMempool` / `DumpMempool` style persistence and safe handling of bad persisted data.
- `packages/bitcoin-knots/test/functional/mempool_persist.py`: restart persistence behavior and stale-record expectations.
- `packages/bitcoin-knots/src/txmempool.h` and `.cpp`: mempool entry/index ownership, conflict removal, trimming, replacement, expiry, and descendant cleanup.
- `packages/bitcoin-knots/src/validation.cpp`: block connect/disconnect, mempool removal, disconnected transaction reconsideration, and reorg behavior.
- `packages/bitcoin-knots/src/net_processing.cpp`: transaction serving/fanout interaction with mempool and peer permissions.
- `packages/bitcoin-knots/src/node/txdownloadman.h` and `_impl.cpp`: request/fanout cleanup and peer transaction state.
- Functional tests: `p2p_getdata.py`, `p2p_tx_download.py`, `mempool_accept.py`, and `mempool_reorg.py`.

Important parity boundary: Open Bitcoin can preserve the external bounded v2.0 behavior without claiming full Knots mempool.dat compatibility, package relay, compact block relay, bloom/filter serving, or default public relay.

## Recommended Plan Shape

### Plan 01: Managed Recovery Replay And Relay Cache Rehydration

Build a managed recovery API that:

- loads a `MempoolSnapshot`;
- replays records into the managed mempool against current chainstate/policy;
- records recovered accepted transactions into relay-serving state;
- seeds fanout identity/evidence without socket I/O or duplicate startup `inv` emission;
- returns a low-cardinality recovery summary.

Tests:

- recovered accepted record becomes present in pure mempool and relay-serving cache;
- `getdata` for the recovered txid/wtxid can be served for an eligible peer;
- ordinary/ineligible peers still get `not_relay_eligible`;
- recovery does not emit outbound socket messages or duplicate fanout on startup.

### Plan 02: Restart Lifecycle Cleanup And Reorg Coherence

Extend managed lifecycle tests and code so recovered records:

- are removed from relay-serving cache after block connect confirmation;
- are suppressed/cleaned when replaced, evicted, expired, or policy-incompatible;
- stay coherent across reorg reconsideration and disconnected transaction replay;
- preserve txid/wtxid identity status after restart.

Tests:

- recovered confirmed transaction is no longer serveable after block connect;
- recovered conflict/replacement removes stale serving/fanout entries;
- recovered evicted/expired outcomes clean fanout identity state;
- reorg reconsideration reuses `MempoolOutcome` and does not recurse unboundedly.

### Plan 03: Sanitized Recovery Evidence Across Operator Surfaces

Project recovered relay state into existing evidence surfaces:

- add fixed recovery counters or labels only if current `RelayEvidenceCounters` cannot represent recovery safely;
- keep baseline RPC methods narrow and put Open Bitcoin-specific evidence in `openbitcoinnetworkstatus` / status snapshot paths;
- extend CLI/dashboard/support rendering and redaction tests only through shared status projection;
- keep metrics/logs fixed and low-cardinality.

Tests:

- status JSON contains aggregate recovered/drop counters or existing counters as designed;
- support bundle JSON/Markdown redacts malicious recovery reason text;
- metrics/log projections contain no dynamic labels, txids, wtxids, endpoints, permission strings, credentials, or raw transaction hex.

### Plan 04: Parity Docs, Checker, Verification, And Closeout

If implementation updates docs/parity roots:

- register a Phase 108 parity surface for MEM-04, MEM-05, MEM-06, REL-01, and REL-02;
- update `docs/parity/checklist.md`, `docs/parity/index.json`, and relevant catalog docs;
- add `scripts/check-phase108-durable-mempool-relay-state-recovery.ts` plus tests;
- wire the checker into `scripts/verify.sh` after Phase 107;
- create `108-VERIFICATION.md` only after targeted checks and `bash scripts/verify.sh` pass.

Checker should fail on missing evidence roots, missing requirement IDs, stale verifier order, missing summary artifacts, and positive claims for public relay defaults, compact blocks, package relay, bloom/filter serving, public-network relay CI, production service operation, production full-node readiness, production-funds wallet safety, public propagation guarantees, or destructive repair.

## Risks And Mitigations

- **Duplicate fanout at startup:** replaying accepted transactions through ordinary fanout could emit duplicate `inv` on restart. Mitigation: separate recovery rehydration from message draining and test that replay produces no outbound messages.
- **Stale serveable transactions:** pure mempool replay without serving cleanup can leave confirmed or replaced records serveable. Mitigation: make cleanup go through existing `remove_stored_transactions_with_status` and test recovered records through block connect/reorg paths.
- **Evidence leaks:** corruption/recovery reason strings could expose txids, wtxids, endpoints, or raw hex. Mitigation: use fixed recovery labels and extend support redaction tests with hostile strings.
- **Parallel state models:** a separate recovery cache would diverge from Phase 104 serving/fanout state. Mitigation: reuse `RelayServingCache` and `ManagedRelayFanoutState` or add a thin recovery method inside those modules.
- **Overbroad docs:** Phase 108 could accidentally imply public propagation or production readiness. Mitigation: deterministic checker with forbidden positive-claim fixtures and required deferred-scope text.

## Verification Strategy

Run targeted checks during implementation:

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node mempool_snapshot --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node relay_serving --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node relay_fanout --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc open_bitcoin_network_status --all-features`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli support --all-features`
- `bun test scripts/check-phase108-durable-mempool-relay-state-recovery.test.ts` if a Phase 108 checker is added.
- `bun run scripts/check-phase108-durable-mempool-relay-state-recovery.ts` if a Phase 108 checker is added.

Final verification target remains:

```bash
bash scripts/verify.sh
```

## Open Questions For Planning

- Whether recovered accepted records should seed fanout queue identity only, or also increment a recovered accepted counter in `RelayEvidenceCounters`.
- Whether `MempoolRecoveryStatus` should gain statuses for schema/corruption outcomes, or whether those stay under `StorageError` / recovery marker evidence.
- Whether Phase 108 should create a new checker or extend Phase 103/104/105/107 checkers. A new checker is cleaner if parity roots or verifier order change.
