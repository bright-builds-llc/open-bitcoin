# Architecture Research: v2.2 Package Relay and Long-Lived Mempool Policy

**Domain:** Headless Bitcoin node package admission, transaction relay, and sustained mempool policy
**Researched:** 2026-07-22
**Confidence:** HIGH for current Open Bitcoin and pinned Knots boundaries; MEDIUM for the final production scheduler shape until v2.2 requirements choose exact activation and crash-durability guarantees

## Recommendation

Extend the existing authoritative `ManagedPeerNetwork` instead of creating a package service, mempool worker, or rebroadcast daemon with its own state. The pure `open-bitcoin-mempool` crate should remain the sole owner of accepted-transaction graph state, package admission decisions, rolling minimum-fee state, expiry, and pressure-driven eviction. The pure `open-bitcoin-network` crate should remain the owner of peer download, orphan/reconsiderable, fanout, serving, and per-peer eligibility decisions. `open-bitcoin-node` should compose both behind the existing `ManagedNetworkHandle`, run time-based maintenance, persist complete snapshots, and translate typed actions into the v2.1 peer outboxes.

Package relay must not introduce a new P2P package wire message. The pinned Knots baseline opportunistically constructs a bounded 1-parent/1-child package from ordinary `inv`/`getdata`/`tx` traffic and orphan/reconsiderable state. Direct RPC submission supports the broader child-with-unconfirmed-parents package shape. These are two entry adapters over one package admission engine, not two policy implementations.

Rolling fee and rebroadcast are different kinds of state. Rolling fee is pure mempool policy derived from eviction, block arrival, occupancy, and injected time. Initial-broadcast retry is relay-delivery state derived from local admission, peer requests, and injected time. Knots persists the unbroadcast set but not its rolling fee variables. Open Bitcoin should preserve that restart boundary unless v2.2 explicitly records an intentional deviation.

## Standard Architecture

### System Overview

```text
┌──────────────────────────────────────────────────────────────────────┐
│ Effectful entry points                                               │
│ RPC submitpackage/testmempoolaccept | inbound tx | daemon tick       │
└───────────────────────────────┬──────────────────────────────────────┘
                                │ typed commands + injected time/randomness
┌───────────────────────────────▼──────────────────────────────────────┐
│ One runtime authority: ManagedNetworkHandle                         │
│ ┌──────────────────────────────────────────────────────────────────┐ │
│ │ ManagedPeerNetwork                                               │ │
│ │ ┌──────────────────────┐  ┌───────────────────────────────────┐ │ │
│ │ │ ManagedMempool       │  │ PeerManager + orphanage + relay  │ │ │
│ │ │ accepted DAG         │  │ download/fanout/serving state    │ │ │
│ │ │ rolling fee          │  │ unbroadcast + per-peer queues    │ │ │
│ │ │ expiry/eviction      │  │ package candidates/reject cache  │ │ │
│ │ └──────────────────────┘  └───────────────────────────────────┘ │ │
│ │          │ one mutation delta updates every dependent cache       │ │
│ └──────────┼─────────────────────────────────────────────────────────┘ │
└────────────┼──────────────────────┬───────────────────────────────────┘
             │ owned snapshot       │ bounded peer emissions + receipts
┌────────────▼─────────────┐  ┌─────▼──────────────────────────────────┐
│ Node shell              │  │ v2.1 authoritative peer transport      │
│ maintenance scheduler   │  │ generalized peer outbox registry       │
│ persistence coordinator│  │ inbound/outbound session writers       │
│ metrics/log projection  │  │ successful serve/request evidence      │
└────────────┬─────────────┘  └────────────────────────────────────────┘
             │
┌────────────▼─────────────────────────────────────────────────────────┐
│ Fjall: versioned mempool runtime snapshot                           │
│ transactions + entry times + unbroadcast markers + schema metadata │
└──────────────────────────────────────────────────────────────────────┘
```

### Authority Boundaries

| State or effect | Sole authority | Persistence | Notes |
| --- | --- | --- | --- |
| Accepted transactions, parent/child graph, aggregate stats | `open_bitcoin_mempool::Mempool` inside `ManagedPeerNetwork` | Reconstruct from transaction records | Never duplicate the graph in a scheduler or RPC context. |
| Rolling minimum fee and decay eligibility | Pure mempool state | Reset on restart for Knots parity | It changes only through typed eviction, block-connected, and time-sampled transitions. |
| Package candidates and reconsiderable rejects | `PeerManager`/transaction relay state | Volatile unless a later parity requirement proves otherwise | P2P auto-packaging is bounded 1-parent/1-child state, not a general package pool. |
| Relay-serving transaction bodies and txid/wtxid indexes | `ManagedPeerNetwork` | Rebuilt from recovered mempool entries | Lifecycle delta must remove evicted/replaced/confirmed/expired entries everywhere. |
| Unbroadcast locally submitted transactions | Relay-delivery state under `ManagedPeerNetwork` | Persist txids with mempool snapshot | Clear only when the Knots-equivalent initial-broadcast acknowledgement boundary is reached, not when an `inv` is merely queued. |
| Per-peer fanout, known inventory, request state, and outboxes | Network authority plus live transport registry | Volatile | Rebuilt from active sessions after restart; no stale per-peer queue restoration. |
| Clock, jitter, storage I/O, sockets, metrics, and logs | `open-bitcoin-node`/daemon shell | As applicable | These adapters supply facts and execute decisions; they do not decide policy. |

### Component Responsibilities

| Component | Change | Responsibility |
| --- | --- | --- |
| `open-bitcoin-mempool::package` | New | Package identity, context-free bounds, topological/consistency checks, child-with-parents classification, and typed package-wide errors. |
| `Mempool` staged admission | Modify | Evaluate individual transactions first, then the eligible subpackage; return ordered per-transaction results and apply one coherent graph mutation for each accepted subpackage. |
| `Mempool` rolling-fee/pressure state | New/modify | Track effective rolling floor, eviction bump, block-since-bump gate, decay, expiry, accounted usage, and descendant-package eviction. |
| `open-bitcoin-network` transaction relay | Modify | Distinguish hard rejects from reconsiderable fee rejects, pair a parent with a same-peer orphan child, cache rejected package identity, and emit typed package candidates. |
| `ManagedPeerNetwork` admission bridge | Modify | Invoke the package engine, apply each admitted result to serving/fanout/compact-candidate state, and process all removals through one lifecycle delta. |
| Relay fanout and serving | Modify | Queue every still-admitted package transaction using existing txid/wtxid inventory, peer filters, topology-aware ordering, bounds, and activation policy. |
| Mempool persistence coordinator | New in node shell | Capture a complete snapshot under the authority, coalesce dirty generations, checkpoint periodically and on clean shutdown, and report failures without inventing a second in-memory truth. |
| Daemon maintenance scheduler | New or generalized shell adapter | Wake independently of message receives, inject time and randomized delay, call one authority maintenance command, enqueue resulting emissions, and request persistence checkpoints. |
| v2.1 peer outbox transport | Generalize | Carry transaction inventory as well as block announcements through the same bounded live-session registry and return typed completion/serve receipts. |
| RPC/status/metrics/log/support projection | Modify | Add package results, effective mempool floor, pressure/eviction, unbroadcast/retry, persistence, and recovery fields from one authoritative snapshot. |

## Recommended Project Structure

```text
packages/
├── open-bitcoin-mempool/src/
│   ├── package.rs                    # Package shapes, IDs, bounds, errors
│   ├── pool/package_admission.rs     # Staged individual + package evaluation
│   ├── pool/rolling_fee.rs           # Pure bump/gate/decay state machine
│   └── pool/pressure.rs              # Usage, expiry, descendant eviction delta
├── open-bitcoin-network/src/peer/transaction_relay/
│   ├── package.rs                    # Bounded P2P 1p1c candidate assembly
│   └── rebroadcast.rs                # Pure retry schedule and unbroadcast actions
├── open-bitcoin-node/src/network/
│   ├── package_admission.rs          # Mempool/network outcome bridge
│   ├── mempool_maintenance.rs        # Apply time/block/pressure lifecycle deltas
│   └── relay_rebroadcast.rs          # Prepare retry fanout and acknowledgements
├── open-bitcoin-node/src/storage/
│   └── mempool_snapshot.rs           # Versioned runtime snapshot and recovery
└── open-bitcoin-rpc/src/
    ├── dispatch/node.rs              # submitpackage/testmempoolaccept/getmempoolinfo
    └── bin/open_bitcoind/             # Clock/transport/persistence scheduling shell
```

These are recommended responsibility boundaries, not a requirement to create every file immediately. Extend the current `foo.rs` plus `foo/` layout and split only when a module has a coherent invariant of its own.

## Architectural Patterns

### Staged Pure Transition With One Commit Point

Package acceptance must operate on a staged mempool view. The result needs both package-wide state and per-transaction results because Knots may admit transactions that pass individually even when the remaining package fails. "Atomic package" must therefore mean no half-applied accepted subpackage or cross-cache mutation, not all-or-nothing for the entire RPC call.

```rust
pub struct PackageAdmissionTransition {
    pub package_result: PackageAdmissionResult,
    pub mempool_delta: MempoolDelta,
}

pub fn evaluate_package(
    current: &MempoolState,
    package: Package,
    chainstate: &ChainstateSnapshot,
    now: UnixSeconds,
) -> Result<PackageAdmissionTransition, PackageAdmissionError>;
```

The node bridge applies `MempoolDelta` once, then derives serving-cache, fanout, compact-candidate, unbroadcast, metrics, and persistence-dirty effects from that same delta. Do not call the current single-transaction mutation repeatedly and attempt rollback after a later failure.

### Two Fee Floors, One Effective Admission Decision

Keep the configured minimum relay fee distinct from the rolling mempool minimum:

- Individual transaction admission requires the effective threshold `max(static min relay, rolling mempool floor)`.
- Package aggregate feerate may satisfy the rolling mempool floor for the eligible subpackage.
- Package feerate must not silently bypass the static minimum relay fee; any Knots exception such as a specifically scoped TRUC rule must be explicit and separately evidenced.
- `getmempoolinfo.mempoolminfee` projects `max(rolling floor, min relay)`, while `minrelaytxfee` remains the configured static value.

This separation prevents the existing `min_relay_feerate` field from becoming an overloaded mutable value.

### Injected-Time Rolling Fee State Machine

Rolling-fee logic belongs in the pure mempool crate and accepts `now` as data. From the pinned Knots implementation:

- Descendant-package eviction bumps the rolling floor to the removed package feerate plus the incremental relay fee.
- Decay is disabled until a block has been connected after the latest bump.
- After that gate opens, the floor decays with a 12-hour half-life, accelerated when usage falls below one-half or one-quarter capacity.
- Values below half the incremental relay fee collapse to zero; otherwise the returned rolling floor is at least the incremental relay fee.

Use deterministic integer/fixed-point arithmetic or golden vectors around the Knots floating-point result. Do not read wall-clock time inside `Mempool`.

### Command/Delta/Receipt Across Effects

Time and transport effects should be modeled as a three-step protocol:

1. The shell sends `MaintenanceTick { now, jitter }` to `ManagedNetworkHandle`.
2. The authority returns bounded peer emissions plus a state/persistence delta.
3. The transport returns a typed receipt; the authority records achieved evidence and updates delivery state.

For initial broadcast, an inventory write is not sufficient acknowledgement. The pinned baseline removes an unbroadcast tx when an eligible peer requests it and the node serves the transaction. The Open Bitcoin receipt should bind that serve event; eviction, confirmation, replacement, or expiry also removes the marker.

## Data Flows

### P2P Limited Package Relay

```text
ordinary inv → bounded tx request → ordinary tx received
                                   ↓
                         individual admission attempt
                                   ↓ reconsiderable/missing
                reconsiderable filter + bounded orphanage
                                   ↓ eligible same-peer pair
                    PackageCandidate(parent, child, senders)
                                   ↓
                   node package admission bridge
                                   ↓
       per-tx result + one mempool/lifecycle mutation delta
                                   ↓
      existing txid/wtxid fanout → peer outboxes → transport
```

No codec or wire-message change is required for package relay itself. The network layer must keep hard reject, missing-input, and fee-reconsiderable categories distinct so a valid CPFP package is not suppressed by the ordinary recent-reject path.

### Direct Package RPC

```text
submitpackage/testmempoolaccept
    → decode all transactions at RPC boundary
    → context-free package checks
    → child-with-unconfirmed-parents/tree check for submission
    → authoritative package admission command
    → per-wtxid results + package result
    → enqueue each still-admitted transaction through existing relay policy
```

The pinned limits are at most 25 transactions and 404,000 total weight, topologically sorted with no duplicate transactions or internal input conflicts. RPC response ordering and partial-result behavior must remain explicit. A successful local result still does not promise public propagation.

### Pressure, Eviction, and Rolling-Fee Flow

```text
admission/package admission/maintenance tick
    → expire old entries with descendants
    → compare accounted memory usage to configured byte capacity
    → select lowest descendant-package score
    → remove victim + descendants
    → bump rolling fee from removed package + incremental fee
    → emit one MempoolDelta
        ├── remove serving/fanout/unbroadcast state
        ├── clear compact reconstruction candidates/slots
        ├── update package/orphan/reject state where applicable
        ├── mark persistence generation dirty
        └── project fixed-label evidence
```

The current Open Bitcoin cap is expressed as total virtual size, while Knots trims on estimated dynamic memory usage. v2.2 should introduce explicit accounted-byte pressure (or document and test an intentional difference); it must not label a virtual-size-only cap as Knots pressure parity.

### Block Connect and Decay Gate

```text
validated block connect under ManagedNetworkHandle
    → existing confirmed/conflict/descendant removals
    → clear related relay/compact/unbroadcast state
    → record block-since-last-rolling-fee-bump
    → later maintenance tick may decay rolling floor
```

This must stay on the existing authoritative block-connect path. A parallel chain observer would race the mempool and recreate the v2.1 split-authority failure mode.

### Persistence and Restart

```text
authoritative mutation
    → increment dirty generation
    → capture owned snapshot under one authority lock
    → release lock
    → Fjall atomic versioned write
    → checkpoint success/failure evidence

startup
    → load schema-versioned snapshot
    → validate txid/wtxid and timestamps
    → replay records in topological order against authoritative chainstate
    → rebuild graph, serving indexes, and compact candidates
    → restore only surviving unbroadcast markers
    → start rolling fee at the Knots restart baseline
```

Extend the current snapshot with admission time and unbroadcast membership. Sorting records only by txid is insufficient for parent/child recovery. Derived ancestry, descendant scores, relay caches, and peer queues should be rebuilt, not persisted. Clean shutdown should force a final checkpoint; periodic checkpoints may be coalesced, and crash-loss guarantees must be stated rather than implied.

## Integration Points

| Existing seam | v2.2 integration |
| --- | --- |
| `open-bitcoin-mempool::Mempool::accept_transaction*` | Refactor shared validation into staged single/package evaluation; keep one graph authority. |
| `Mempool::pressure_summary`, `trim_to_size`, and `remove_for_connected_block` | Replace deferred rolling status with actual state, accounted pressure, expiry, eviction reason, and one lifecycle delta. |
| `TxDownloadScheduler` and `TxOrphanage` | Add reconsiderable-package identity and bounded 1p1c assembly without adding wire messages. |
| `ManagedPeerNetwork::process_actions`/admission bridge | Handle package candidates and atomically propagate admission/removal effects to every cache. |
| `ManagedRelayFanoutState` | Replace `defer_local_rebroadcast` with persisted unbroadcast state and pure due-retry decisions. |
| `RelayServingCache` | Clear unbroadcast only at the served-request acknowledgement boundary; remove every lifecycle victim. |
| `ManagedNetworkHandle` | Add package admission, maintenance tick, snapshot capture, and typed receipt methods; all consumers share cloned handles to the same authority. |
| `AnnouncementOutboxRegistry`/`PeerEmission` | Generalize from block-only metadata to an enum payload/receipt that also carries bounded transaction inventory and transaction-serve acknowledgements. |
| `DurableSyncRuntime` and `open-bitcoind` loop | Reuse its clock, store, metrics, and shared network handle, but schedule mempool maintenance even when sync has no incoming traffic; do not make policy depend on a receive call. |
| `FjallNodeStore::save/load_mempool_snapshot` | Upgrade schema and add the missing production checkpoint coordinator around the existing codec/adapter. |
| `ManagedNetworkOperatorSnapshot` | Add package, rolling floor, pressure, eviction, unbroadcast/retry, checkpoint, and recovery evidence under the same read guard. |
| RPC/CLI/dashboard/support | Consume the shared snapshot; add `submitpackage`/`testmempoolaccept` only as adapters, never as a second admission implementation. |

## Dependency-Aware Build Order

1. **Resource and fee primitives** — split static relay fee from rolling floor, add accounted usage, entry time, typed removal reasons, and deterministic clock types. Everything later depends on these semantics.
2. **Rolling fee, expiry, and eviction core** — implement pure bump/gate/decay and descendant-package eviction with invariant/property tests. Package admission must evaluate against the correct dynamic floor.
3. **Package vocabulary and staged admission** — add context-free limits, topological/tree checks, individual-first evaluation, package feerate, package limits/RBF boundary, and coherent deltas.
4. **Package-aware download/orphan bridge** — distinguish reconsiderable rejects, assemble bounded P2P 1p1c candidates, and feed the staged admission engine. Do not touch transport yet.
5. **Cross-cache lifecycle integration** — apply package acceptance, replacement, pressure eviction, expiry, block connect, and reorg deltas to relay serving, fanout, compact candidates, orphanage, and unbroadcast state.
6. **Snapshot schema and recovery** — persist entry time and unbroadcast markers, topologically replay, rebuild indexes, reset rolling fee per parity, and add clean-shutdown/periodic checkpoint behavior.
7. **Receive-independent maintenance and transport** — generalize v2.1 outboxes, run maintenance on a daemon clock, add randomized 10–15 minute initial-broadcast retries, and wire request/serve receipts.
8. **RPC and operator evidence** — expose package methods and authoritative fee/pressure/retry/checkpoint fields through RPC, CLI, dashboard, metrics, logs, and support bundles.
9. **Parity, adversarial pressure, restart, and release guardrails** — verify Knots vectors, long-run boundedness, no duplicate authority, no public-default expansion, and deterministic default verification.

Phases 1–3 are pure-core work and should land before shell integration. Phase 5 must precede persistence and transport so recovered or relayed state cannot bypass cleanup invariants. Operator projection belongs after achieved effects exist, not before.

## Scaling Considerations

| Load | Architecture response |
| --- | --- |
| Small/regtest | Simple maps and full invariant checks are acceptable; deterministic fake-clock and package vector coverage dominate. |
| Sustained realistic mempool | Avoid cloning and fully recomputing the entire graph per transaction/package. Maintain staged adjacency/aggregate deltas and a bounded eviction index, with benchmarks against current behavior. |
| Adversarial limits | Bound package count/weight, orphanage and reconsiderable filters, per-peer announcements/in-flight requests, maintenance work per tick, persistence generations, and operator evidence cardinality. Apply backpressure rather than spawn more tasks. |

Do not shard the mempool or create per-peer package pools. Admission, conflicts, ancestry, descendant eviction, and the rolling floor are global policy decisions and require one coherent state.

## Anti-Patterns

### A Second Mempool or Scheduler Authority

**Wrong:** Let RPC, the daemon maintenance worker, or persistence own a copy of mempool/package/retry state.
**Consequence:** Admission, relay, status, and restart diverge exactly as v2.1's authority unification was designed to prevent.
**Instead:** Every mutation and snapshot goes through `ManagedNetworkHandle`; workers own only effect handles and timers.

### Treating Package Relay as a New Protocol Message

**Wrong:** Add `package`, `getpackage`, or package-inventory wire codecs.
**Consequence:** It diverges from the pinned limited-package-relay baseline and creates unsupported interoperability claims.
**Instead:** Assemble bounded candidates from ordinary transaction relay and expose wider submission only through RPC.

### Aggregate Fee Bypasses Every Floor

**Wrong:** Compare package feerate to one overloaded `min_relay_feerate` value.
**Consequence:** Low-fee transactions receive free relay and parity-sensitive exceptions become accidental global behavior.
**Instead:** Model static relay floor and dynamic mempool floor separately, with explicit scoped exceptions.

### Clear Unbroadcast on Inventory Queueing

**Wrong:** Mark initial broadcast complete when an `inv` is queued or written.
**Consequence:** With no interested peer or a failed request path, restart stops retrying a transaction that was never served.
**Instead:** Clear on the pinned-equivalent eligible `getdata`/transaction-serve boundary or lifecycle removal.

### Persist Derived or Volatile State Blindly

**Wrong:** Serialize ancestry maps, peer queues, package candidates, rolling fee, and partial compact state.
**Consequence:** Recovery trusts stale topology/peer facts and changes the Knots restart boundary.
**Instead:** Persist source records, entry time, and unbroadcast markers; validate and rebuild derived indexes.

### Maintenance Only on Incoming Messages

**Wrong:** Decay fees, expire entries, or retry broadcast only inside receive handlers.
**Consequence:** Idle nodes never advance long-lived policy.
**Instead:** Use a receive-independent daemon tick with injected time and bounded work.

### Virtual Size Presented as Memory-Pressure Parity

**Wrong:** Keep only `max_mempool_virtual_size` while claiming Knots `-maxmempool` behavior.
**Consequence:** Pressure, eviction timing, rolling floors, and operator diagnostics diverge under realistic entry overhead.
**Instead:** Add explicit accounted memory usage or document the intentional difference with parity tests.

## Research Flags

- **TRUC/package-RBF scope:** The pinned baseline contains narrowly scoped package RBF and TRUC exceptions. Confirm whether v2.2 includes them before broadening package admission; otherwise return explicit unsupported/deferred outcomes.
- **Crash durability contract:** Knots persists on its mempool dump lifecycle and restores unbroadcast markers, but rolling fee variables are not serialized. Requirements should distinguish clean restart, periodic checkpoint, and sudden-crash expectations.
- **Transport acknowledgement:** Knots clears unbroadcast when an eligible `getdata` is served. Specify whether Open Bitcoin records completion when the tx response is queued or successfully written; never clear on `inv` alone.
- **Memory accounting parity:** Exact `DynamicMemoryUsage()` parity is implementation-specific. Establish an auditable Rust accounting model and acceptable observable tolerance before claiming sustained-pressure parity.

## Sources

All baseline sources below are from the pinned local Bitcoin Knots tag `v29.3.knots20260210`, commit `a9aee730466ac67d35a3c03ee24676be5e045878`.

- [Pinned Bitcoin Knots source tree](https://github.com/bitcoinknots/bitcoin/tree/a9aee730466ac67d35a3c03ee24676be5e045878) — authoritative baseline commit used for every Knots claim below.

### Current Open Bitcoin

- `packages/open-bitcoin-mempool/src/pool.rs` — current single-transaction clone/recompute admission, descendant eviction, and virtual-size cap.
- `packages/open-bitcoin-mempool/src/pool/lifecycle.rs` — current pressure summary and explicitly deferred rolling-fee status.
- `packages/open-bitcoin-node/src/network.rs` and `network/runtime_authority.rs` — one authoritative mempool/network/chainstate aggregate behind `ManagedNetworkHandle`.
- `packages/open-bitcoin-node/src/network/admission_bridge.rs`, `relay_fanout.rs`, `relay_serving.rs`, and `mempool_lifecycle.rs` — current admission, cache, fanout, serving, and compact cleanup seams.
- `packages/open-bitcoin-node/src/storage/mempool_snapshot.rs`, `storage/snapshot_codec.rs`, and `storage/fjall_store/mempool.rs` — current transaction snapshot, schema, load/save adapters, and recovery classifications.
- `packages/open-bitcoin-node/src/sync.rs`, `sync/session.rs`, and `network/announcement_transport.rs` — v2.1 shared network handle, live peer outbox registry, and receipt-based transport evidence.
- `packages/open-bitcoin-rpc/src/context/network.rs`, `dispatch/node.rs`, and `src/bin/open-bitcoind.rs` — authoritative RPC projection, current mempool RPCs, and daemon timing shell.

### Pinned Bitcoin Knots

- `packages/bitcoin-knots/doc/policy/packages.md` — package definitions, individual-first semantics, fee-floor separation, and package policy rationale.
- `packages/bitcoin-knots/src/policy/packages.h` and `packages.cpp` — count/weight, sorting, duplicate, conflict, and child-with-parents checks.
- `packages/bitcoin-knots/src/validation.cpp` — staged package acceptance, package feerate, limits, replacement boundary, trimming, and per-transaction results.
- `packages/bitcoin-knots/src/node/txdownloadman.h`, `txdownloadman_impl.cpp`, and `src/net_processing.cpp` — opportunistic P2P 1p1c construction, package-result relay, per-peer queues, and initial-broadcast retry scheduling.
- `packages/bitcoin-knots/src/txmempool.h` and `txmempool.cpp` — rolling fee state machine, dynamic-memory trim, descendant eviction, block decay gate, expiry, and unbroadcast lifecycle.
- `packages/bitcoin-knots/src/node/mempool_persist.cpp` — transaction entry time, fee delta, and unbroadcast persistence; rolling fee is not serialized.
- `packages/bitcoin-knots/src/node/transaction.cpp` and `test/functional/mempool_unbroadcast.py` — local submission unbroadcast marking, restart recovery, retry, and acknowledgement behavior.
- `packages/bitcoin-knots/src/rpc/mempool.cpp` — `submitpackage`, `testmempoolaccept`, per-wtxid results, `getmempoolinfo`, and unbroadcast count.

## Confidence Assessment

| Area | Confidence | Basis |
| --- | --- | --- |
| Existing Open Bitcoin integration seams | HIGH | Direct inspection of current Rust source and completed v2.0/v2.1 roadmaps. |
| Package admission and limited P2P relay | HIGH | Pinned Knots policy docs, validation, tx download manager, net processing, RPC, and tests agree. |
| Rolling fee and pressure lifecycle | HIGH | Direct pinned `CTxMemPool` and `LimitMempoolSize` implementation. |
| Persistence/restart boundary | HIGH | Direct dump/load format and unbroadcast functional test; no rolling-fee fields are serialized. |
| Exact daemon scheduler/outbox refactor | MEDIUM | The authority and transport constraints are clear, but the smallest implementation depends on v2.2 activation and crash-durability requirements. |

*Architecture research for: Open Bitcoin v2.2 Package Relay and Long-Lived Mempool Policy*
*Researched: 2026-07-22*
