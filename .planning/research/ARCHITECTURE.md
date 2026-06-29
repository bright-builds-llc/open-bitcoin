# Architecture Research

**Domain:** v2.0 transaction relay and mempool participation boundary for Open Bitcoin
**Researched:** 2026-06-29
**Confidence:** HIGH for integration architecture, MEDIUM for exact parity depth of orphan/package/rolling-filter behavior

## Executive Summary

v2.0 should integrate transaction relay as a pure decision layer between the existing peer lifecycle model and the existing mempool admission engine. Do not make the `open-bitcoind` socket loop own transaction-relay policy. The pinned Knots baseline has the same architectural split in spirit: `net_processing.cpp` owns peer message orchestration, while `node/txdownloadman.*` owns transaction announcement, request, orphan, accepted/rejected, and peer connect/disconnect decisions.

Open Bitcoin already has most of the raw ingredients: `open-bitcoin-network` parses `inv`, `getdata`, `notfound`, `tx`, and `wtxidrelay`; `PeerManager` tracks requested txids/wtxids and emits pure `PeerAction`s; `open-bitcoin-mempool` admits transactions against a chainstate snapshot with standardness, fee, RBF, ancestor/descendant, and trim policy; `open-bitcoin-node::ManagedPeerNetwork` bridges peer actions to managed mempool and transaction storage; RPC/status surfaces already expose `getmempoolinfo`, `sendrawtransaction`, `getnetworkinfo`, and `openbitcoinnetworkstatus`.

The missing boundary is not a new stack. It is a first-class transaction-relay subsystem that can be unit-tested without sockets, Fjall, clocks, or public peers. Build it as `open-bitcoin-network::tx_relay` plus small mempool outcome improvements, then let `open-bitcoin-node` perform storage, status, metrics, logs, and actual message writes as thin effects.

Keep the v2.0 claim bounded. This milestone should activate scoped relay and mempool participation, not compact blocks, BIP37 bloom serving, compact filters, broad address relay, public relay defaults, package relay as a broad public claim, production full-node readiness, or production-funds wallet safety.

## Inputs And Standards

Material repo inputs loaded:

- `.planning/PROJECT.md`
- `.planning/ARCHITECTURE.md`
- `.planning/milestones/v1.9-ROADMAP.md`
- `packages/open-bitcoin-mempool/src/lib.rs`
- `packages/open-bitcoin-network/src/lib.rs`
- `packages/open-bitcoin-node/src/lib.rs`
- `packages/open-bitcoin-rpc/src/lib.rs`
- Relevant implementation modules behind those crate roots, parity catalogs, storage/status/RPC code, and pinned Knots transaction-relay sources.

Repo-local guidance and Bright Builds standards materially affected this recommendation: preserve functional-core / imperative-shell boundaries, parse raw inputs into domain types, make illegal states unrepresentable, keep pure business logic unit-testable, use repo-owned verification, keep public-network checks outside default verification, and maintain auditable Knots breadcrumbs in `docs/parity/`. `standards-overrides.md` has no meaningful active override entries.

One active requirements file mentioned by the project text was not present at `.planning/REQUIREMENTS.md` during research. The architecture below is therefore grounded in the milestone context, shipped v1.9 artifacts, current code, and pinned Knots sources.

## Standard Architecture

### System Overview

```text
+--------------------------------------------------------------------------------+
| Shell adapters: open-bitcoind, RPC HTTP, CLI/status/support, Fjall, sockets     |
|                                                                                |
|  inbound/outbound TCP   RPC submit/status   metrics/logs/support   Fjall store |
+--------------------+------------------+----------------------+-----------------+
                     |                  |                      |
                     v                  v                      v
+--------------------------------------------------------------------------------+
| Node runtime shell: open-bitcoin-node                                           |
|                                                                                |
|  ManagedPeerNetwork                                                            |
|    - owns ManagedMempool, TransactionRelayRuntime bridge, tx/block stores       |
|    - translates pure PeerAction/RelayAction into storage, status, and messages  |
|                                                                                |
|  ManagedMempool                                                                |
|    - feeds chainstate snapshots into pure mempool admission                     |
|    - emits admission/removal outcomes for relay and status                      |
+--------------------+------------------+----------------------+-----------------+
                     |                  |                      |
                     v                  v                      v
+--------------------------------------------------------------------------------+
| Pure core crates                                                               |
|                                                                                |
|  open-bitcoin-network        open-bitcoin-mempool       open-bitcoin-chainstate |
|    PeerManager                 Mempool admission          ChainstateSnapshot     |
|    tx_relay manager            policy/replacement         UTXO context          |
|    resource policy             ancestor/descendant                              |
|    permission classes          trim/removal decisions                           |
+--------------------------------------------------------------------------------+
```

### Component Responsibilities

| Component | Responsibility | Typical implementation |
| --- | --- | --- |
| `open-bitcoin-network::PeerManager` | Handshake, peer state, wire-message interpretation, resource gates, and peer-scoped action emission | Pure state machine returning `PeerAction` values |
| `open-bitcoin-network::tx_relay` | Transaction announcement scheduling, txid/wtxid request selection, orphan tracking, recent accept/reject/confirmed memory, peer relay eligibility | New pure module inspired by Knots `TxDownloadManager` |
| `open-bitcoin-mempool::Mempool` | Transaction admission, standardness, fee/RBF/ancestor/descendant/trim decisions | Existing pure domain engine with richer typed outcomes |
| `open-bitcoin-node::ManagedPeerNetwork` | Runtime composition across peer state, mempool, chainstate, transaction store, and outbound messages | Imperative shell over pure decisions |
| `open-bitcoin-node::FjallNodeStore` | Durable mempool records, relay evidence snapshots, metrics, runtime metadata, and recovery markers | Existing Fjall adapter extended with a `mempool` namespace |
| `open-bitcoin-rpc::ManagedRpcContext` | RPC-facing access to managed network, local submit, inbound listener, metrics, durable status | Thin adapter methods over `ManagedPeerNetwork` |
| `open-bitcoin-rpc` method/dispatch modules | Compatibility-shaped `getmempoolinfo`, `getnetworkinfo`, `sendrawtransaction`, and Open Bitcoin relay/status extensions | Serde DTOs plus dispatch glue |
| `open-bitcoin-cli` operator status/support/dashboard | Human and JSON rendering from `OpenBitcoinStatusSnapshot` only | No independent relay truth model |
| `docs/parity/*` | Auditable Knots anchors, intentional deviations, release boundaries, source breadcrumbs | Update after each component addition |

## Recommended Project Structure

```text
packages/open-bitcoin-network/src/
├── tx_relay.rs                 # Pure transaction relay manager facade
├── tx_relay/
│   ├── announcements.rs        # Inv intake, txid/wtxid request eligibility, delays
│   ├── orphanage.rs            # Bounded orphan pool and parent request decisions
│   ├── recent.rs               # Recent rejected/confirmed/reconsiderable memory
│   ├── permissions.rs          # Relay/mempool/forcerelay effect decisions
│   ├── rebroadcast.rs          # Pure rebroadcast candidate selection
│   └── tests.rs                # Deterministic relay policy tests
├── peer.rs                     # Integrates tx relay actions, keeps peer lifecycle owner
├── peer/inventory_state.rs     # Thinned to delegate tx-specific request decisions
├── inbound/permissions.rs      # Converts inactive relay-like labels into scoped active effects
└── resource.rs                 # Adds relay/orphan/rebroadcast request caps

packages/open-bitcoin-mempool/src/
├── types.rs                    # Add stable admission/rejection/removal DTOs
├── error.rs                    # Preserve Display, add machine-readable reason classes
├── pool.rs                     # Emit richer outcomes; add block/reorg removal hooks
└── policy.rs                   # Keep standardness and fee decisions pure

packages/open-bitcoin-node/src/
├── network.rs                  # Orchestrates tx relay, mempool, store, outbound messages
├── network/tx_relay.rs         # Node-shell bridge from pure RelayAction to effects
├── mempool.rs                  # Managed mempool outcome bridge and durable snapshot loading
├── storage.rs                  # Add StorageNamespace::Mempool
├── storage/fjall_store.rs      # Durable tx records, mempool metadata, relay evidence
├── status.rs                   # Shared mempool and relay status fields
├── metrics.rs                  # Low-cardinality relay/mempool metric kinds
└── logging.rs                  # Sanitized relay/mempool structured log records

packages/open-bitcoin-rpc/src/
├── config/open_bitcoin.rs      # Explicit relay config, disabled/public-safe defaults
├── config/loader/              # CLI/config/env resolution for relay controls
├── context/network.rs          # Submit/relay/status adapter methods
├── method/node.rs              # DTOs for relay/mempool RPC/status surfaces
└── dispatch/node.rs            # Thin dispatch to context methods

docs/parity/
├── catalog/mempool-policy.md   # Update mempool admission, durability, relay hooks
├── catalog/p2p.md              # Add v2.0 tx relay and permission activation boundary
├── index.json                  # Add/adjust v2.0 evidence and deviations
└── source-breadcrumbs.json     # Breadcrumb every new first-party relay source/test file
```

### Structure Rationale

- **`open-bitcoin-network::tx_relay`:** Put relay request, orphan, fanout, and permission decisions beside peer state because they are P2P policy, but keep them pure so deterministic tests can drive them without sockets.
- **`open-bitcoin-mempool`:** Keep admission truth where it already lives. Extend outcomes so relay can react to missing inputs, accept/reject/replaced/evicted, and block/reorg removals without parsing display strings.
- **`open-bitcoin-node`:** Keep it as the imperative shell. It should call pure relay/mempool decisions, persist accepted transactions, append metrics/logs, and encode/send wire messages.
- **`open-bitcoin-rpc` and `open-bitcoin-cli`:** Project shared status and compatibility-shaped RPC responses. Do not create separate operator-only relay state.
- **`docs/parity`:** Treat parity docs as a first-class integration point because v2.0 changes a previously deferred claim.

## Architectural Patterns

### Pattern 1: Pure Transaction Relay Manager

**What:** Create a pure relay manager that consumes typed events and returns typed actions.

**When to use:** Every time a peer announces a transaction, sends `notfound`, sends `tx`, completes relay-capable handshake, disconnects, receives a mempool response, or when a block connect/disconnect changes recent transaction memory.

**Trade-offs:** This is more explicit than handling tx relay inside `PeerManager::handle_message`, but it prevents the socket loop and mempool adapter from quietly owning consensus-adjacent policy.

**Example:**

```rust
pub enum TxRelayEvent {
    PeerConnected(TxRelayPeerInfo),
    PeerDisconnected { peer_id: PeerId },
    Announcement { peer_id: PeerId, inventory: InventoryVector, now: RelayTime },
    TransactionReceived { peer_id: PeerId, transaction: Transaction },
    NotFound { peer_id: PeerId, inventory: Vec<InventoryVector> },
    MempoolAccepted(MempoolAdmissionEvent),
    MempoolRejected(MempoolAdmissionEvent),
    BlockConnected(BlockConnectedRelayEvent),
    ActiveTipChanged,
}

pub enum TxRelayAction {
    SendGetData { peer_id: PeerId, inventory: Vec<InventoryVector> },
    ValidateTransaction { peer_id: PeerId, transaction: Transaction },
    StoreOrphan { peer_id: PeerId, transaction: Transaction },
    RelayAcceptedTransaction { txid: Txid, wtxid: Wtxid },
    ForgetInventory { txid: Txid, maybe_wtxid: Option<Wtxid> },
    RecordDecision(TxRelayDecisionEvent),
}
```

The node shell supplies time as a value. The relay manager must not call clocks, sockets, Fjall, tracing, or RPC.

### Pattern 2: Admission Outcome Bridge

**What:** Convert mempool admission results into stable machine-readable events before relay reacts.

**When to use:** `ManagedMempool::submit_transaction`, received peer txs, `sendrawtransaction`, wallet-built tx submission, block connect, reorg repair, replacement, and trim.

**Trade-offs:** Adding stable event types is more work than matching `MempoolError::to_string()`, but it prevents roadmap phases from encoding policy in brittle strings.

**Recommended event shape:**

```rust
pub struct MempoolAdmissionEvent {
    pub txid: Txid,
    pub maybe_wtxid: Option<Wtxid>,
    pub source: MempoolAdmissionSource,
    pub outcome: MempoolAdmissionOutcome,
    pub missing_inputs: Vec<OutPoint>,
    pub replaced: Vec<Txid>,
    pub evicted: Vec<Txid>,
    pub virtual_size: Option<usize>,
    pub fee_sats: Option<i64>,
}

pub enum MempoolAdmissionOutcome {
    Accepted,
    Duplicate,
    MissingInputs,
    NonStandard,
    FeeTooLow,
    ConflictRejected,
    ValidationRejected,
    CandidateEvicted,
    InternalError,
}
```

`open-bitcoin-mempool` should own this classification or a lower-level equivalent. `open-bitcoin-node` can add source and status metadata.

### Pattern 3: Permission Activation With Negative Guards

**What:** Convert Phase 91 inactive relay-like labels into scoped active effects, and keep still-deferred permissions visibly inactive.

**When to use:** v2.0 relay permission work.

**Trade-offs:** It forces an explicit migration from "recorded but inert" to "active but bounded." That is exactly the point: old tests that asserted `inactive_relay`, `inactive_forcerelay`, and `inactive_mempool` must be replaced by tests that prove the new scoped behavior and the remaining non-claims.

**Recommended effect split:**

| Permission token | v2.0 effect | Still not implied |
| --- | --- | --- |
| `relay` | peer may participate in scoped tx announcement/request/serve decisions | public relay default, force relay, compact blocks |
| `forcerelay` | accepted mempool txs from that peer may be queued even when ordinary duplicate logic would skip fanout, subject to bounded checks | relaying non-mempool transactions, bypassing validation |
| `mempool` | peer may request bounded mempool inventory if BIP35-style `mempool` is added | unlimited txid dump, support-bundle txid leakage |
| `bloomfilter` | keep inactive | BIP37 serving |
| `blockfilters` | keep inactive | compact filter serving |

### Pattern 4: One Shared Status Truth

**What:** Extend `OpenBitcoinStatusSnapshot`, `PeerStatus`, `MempoolStatus`, metrics, and Open Bitcoin network status from the managed runtime state.

**When to use:** Every relay or mempool operator/RPC/support/dashboard surface.

**Trade-offs:** Shared DTO changes ripple through CLI tests and snapshots, but the alternative creates contradictory status truth.

**Recommended status additions:**

- `MempoolStatus.transactions`, `virtual_size`, `total_fee_sats`, `min_relay_fee`, `loaded`, `durability`
- relay counts: announcements received, tx requests sent/served, tx accepted, tx rejected, orphan count, orphan evictions, rebroadcast queued/sent, replacement count, trim evictions
- latest low-cardinality relay decision: outcome, reason, source, next action
- peer permission summary: active relay/mempool/forcerelay effect counts, without raw peer ids, endpoints, txids, or class names in support bundles by default

### Pattern 5: Boundary Checkers As Architecture

**What:** Add deterministic v2.0 guardrails that fail docs or code claims for deferred surfaces.

**When to use:** After relay/mempool behavior lands and before roadmap closeout.

**Trade-offs:** Checker work can feel like documentation work, but this project uses parity docs and release boundaries as part of correctness.

The checker should reject accidental claims for compact blocks, BIP37, compact filters, production full-node readiness, production-funds wallet use, public relay defaults, package relay broad support, public-network CI, and hosted/dashboard expansion unless a later phase deliberately scopes them.

## Data Flow

### Incoming Transaction Announcement Flow

```text
peer socket
  -> inbound envelope/resource gate
  -> WireNetworkMessage::Inv
  -> PeerManager records peer-visible inventory context
  -> tx_relay.add_announcement(peer, txid/wtxid, relay peer info, now)
  -> TxRelayAction::SendGetData or RecordDecision
  -> open-bitcoin-node encodes GetData and writes through runtime adapter
  -> status/metrics/logs record bounded request decision
```

Key rule: `tx_relay` decides whether to request. The socket adapter only reads bytes and writes returned messages.

### Received Transaction Flow

```text
peer socket
  -> WireNetworkMessage::Tx
  -> PeerManager emits PeerAction::ReceivedTransaction
  -> tx_relay marks response received and decides whether validation is needed
  -> ManagedMempool submits against ManagedChainstate snapshot
  -> MempoolAdmissionEvent
       accepted:
         -> persist tx record
         -> notify tx_relay accepted
         -> queue relay announcements to eligible peers
       missing inputs:
         -> bounded orphan decision
         -> request missing parents if eligible
       rejected:
         -> recent reject/reconsiderable memory
         -> sanitized metric/log/status event
       replaced/evicted:
         -> update tx store and relay memory
```

Key rule: mempool admission stays pure. Orphan/request policy stays pure. Persistence and writes stay in `open-bitcoin-node`.

### Local Submission Flow

```text
sendrawtransaction or wallet-built transaction
  -> ManagedRpcContext::submit_local_transaction
  -> ManagedPeerNetwork::submit_local_transaction
  -> ManagedMempool admission
  -> durable tx record and mempool status update
  -> tx_relay queues announcements for eligible relay peers
  -> RPC returns txid/replaced/evicted; it does not wait for public propagation
```

`sendrawtransaction` should not imply production relay success. It should mean local mempool acceptance plus queued relay when relay is active.

### Peer `getdata` And Mempool Request Flow

```text
peer getdata / optional BIP35 mempool request
  -> PeerManager validates command and resource caps
  -> tx_relay checks relay eligibility, known inventory, fee filter, and bounds
  -> PeerAction::ServeInventory or TxRelayAction::ServeMempoolInventory
  -> open-bitcoin-node looks up transactions by txid/wtxid
  -> WireNetworkMessage::Tx or NotFound
```

If the `mempool` command is added in v2.0, gate it behind explicit relay/mempool permission and count/size caps. Do not turn it into an unbounded txid dump.

### Block Connect And Reorg Flow

```text
block connected or disconnected
  -> ManagedChainstate transition
  -> ManagedMempool removes confirmed/conflicting transactions or stages repair
  -> tx_relay updates recent confirmed/reject memory and forgets stale requests
  -> durable mempool snapshot/records update atomically enough for restart review
  -> status/metrics/logs expose removal and repair counts
```

This flow is important for correctness. A relay-capable node must stop serving confirmed transactions as mempool entries and must not keep requesting transactions that became confirmed.

## State Management

```text
Pure runtime state:
  PeerManager
  TransactionRelayManager
  Mempool
  ChainstateSnapshot

Shell-owned durable state:
  accepted transaction records by txid/wtxid
  mempool metadata and last clean load evidence
  relay evidence snapshots and bounded metrics
  runtime recovery metadata

Never persist as resumable truth:
  live sockets
  dead-peer in-flight timers as if the peer is still connected
  raw peer endpoint labels in support bundles
  unbounded orphan or request queues
```

Durable mempool should be a restart aid and evidence surface, not a hidden promise that live relay state resumes exactly after process death.

## Scaling Considerations

| Scale | Architecture adjustments |
| --- | --- |
| Synthetic and loopback tests | In-memory transaction store is enough; use deterministic time and seeded relay memory; prove pure decisions thoroughly. |
| Opt-in local/public review | Persist accepted transactions and mempool metadata in Fjall; cap orphan/request/rebroadcast queues; expose relay status and recovery evidence. |
| Production-readiness future gate | Add deeper privacy/fanout parity, long-run mempool pressure, rolling fee decay, reorg repair, package relay decisions, and public default policy before any production relay claim. |

### Scaling Priorities

1. **First bottleneck: orphan/request memory.** Add hard caps, per-peer accounting, and deterministic eviction before public relay review.
2. **Second bottleneck: fanout privacy and write queues.** Batch/trickle inventory and honor fee filters before increasing peer counts.
3. **Third bottleneck: durable mempool load/repair.** Store transaction records separately from status snapshots and make corrupted mempool records recoverable without corrupting chainstate.

## Anti-Patterns

### Anti-Pattern 1: Socket-Owned Relay Policy

**What people do:** Add tx relay branches directly inside `inbound_listener.rs` or the daemon worker because that is where bytes arrive.

**Why it is wrong:** It mixes clocks, sockets, queues, peer policy, mempool admission, and status side effects. It also makes relay behavior hard to unit test.

**Do this instead:** Decode bytes in the adapter, call pure `PeerManager` and `TransactionRelayManager`, then execute returned actions in `open-bitcoin-node`.

### Anti-Pattern 2: Stringly Typed Mempool Rejections

**What people do:** Parse `MempoolError::to_string()` to decide whether to orphan, reject, request parents, or record a peer event.

**Why it is wrong:** Display text is for humans and parity debugging, not control flow.

**Do this instead:** Add stable admission outcome types and keep human text as a rendering concern.

### Anti-Pattern 3: Persisting Live Peer State

**What people do:** Save in-flight requests, per-peer timers, and socket-derived peer state as if a restart can continue the same P2P session.

**Why it is wrong:** The peer connection died. Reusing live-state facts after restart can create stale requests, misleading status, and privacy bugs.

**Do this instead:** Persist recoverable mempool contents and bounded evidence. Rebuild live relay state from connected peers and fresh announcements.

### Anti-Pattern 4: Public Relay By Configuration Accident

**What people do:** Reuse `localrelay=true`, existing inbound listener flags, or old permission labels to silently enable public transaction propagation.

**Why it is wrong:** v1.9 explicitly left relay-like labels inert. v2.0 must change that boundary deliberately and visibly.

**Do this instead:** Add explicit relay activation config, status, docs, and negative release-boundary checks.

### Anti-Pattern 5: Compact Blocks Sneaking In Through Orphan Handling

**What people do:** Implement package/orphan paths by also touching compact-block extra transaction caches, `cmpctblock`, `blocktxn`, or filter serving.

**Why it is wrong:** Knots integrates these surfaces, but v2.0 scope does not need to claim them. Pulling them in increases parity and DoS risk.

**Do this instead:** Record compact-block-related hooks as deferred. Keep v2.0 orphan/parent request handling focused on ordinary `inv`, `getdata`, `tx`, and `notfound`.

## Integration Points

### External Services

| Service | Integration pattern | Notes |
| --- | --- | --- |
| Pinned Bitcoin Knots source under `packages/bitcoin-knots` | Local source anchors and parity fixtures | Primary authority for relay behavior; no network access needed for the pinned baseline. |
| Public Bitcoin peers | Explicit opt-in UAT only | Keep out of `bash scripts/verify.sh`; generated reports are evidence, not release-blocking CI by default. |

### Internal Boundaries

| Boundary | Communication | Notes |
| --- | --- | --- |
| `open-bitcoin-network` to `open-bitcoin-node` | `PeerAction` and new `TxRelayAction` values | Pure decisions out, shell effects in node. |
| `open-bitcoin-mempool` to `open-bitcoin-node` | `AdmissionResult`, `MempoolError`, and new admission/removal events | Add stable reason classes before relay depends on them. |
| `open-bitcoin-node` to `open-bitcoin-rpc` | `ManagedRpcContext` methods and shared status DTOs | RPC should not inspect peer internals. |
| `open-bitcoin-node` to `FjallNodeStore` | typed storage methods | Add a distinct mempool namespace rather than overloading runtime metadata. |
| `open-bitcoin-rpc` to `open-bitcoin-cli` | baseline RPC plus Open Bitcoin extension status | CLI renders `OpenBitcoinStatusSnapshot`, not bespoke relay state. |
| code to parity docs | breadcrumbs and catalog entries | New Rust source/tests under first-party crates need `source-breadcrumbs.json` entries. |

## Modified Components

### `open-bitcoin-network`

Recommended changes:

- Add `tx_relay` pure module modeled after Knots `TxDownloadManager`.
- Move tx-specific announcement/request logic out of `peer/inventory_state.rs` into `tx_relay`.
- Add `WireNetworkMessage::Mempool` only if v2.0 includes bounded BIP35 mempool inventory serving; keep `cmpctblock`, `blocktxn`, bloom, and compact-filter commands unsupported.
- Add `WireNetworkMessage::FeeFilter` if relay fanout uses peer fee filters in v2.0. Without it, docs must state fee-filter behavior is deferred.
- Replace inactive relay-like permission labels with active scoped labels for `relay`, `forcerelay`, and `mempool`; leave bloom and block-filter labels inactive.
- Extend resource policy with orphan count, per-peer announcement count, tx request in-flight count, mempool inventory response count, rebroadcast queue, and served transaction byte caps.

### `open-bitcoin-mempool`

Recommended changes:

- Add typed admission outcome and rejection classification.
- Return all missing parent outpoints where practical, not just the first missing input.
- Expose block-connected and block-disconnected hooks for mempool removal/repair decisions.
- Keep package relay broad support deferred. Add only the minimum event shape needed to explain "missing parents" and "possibly reconsiderable later" without claiming package relay parity.
- Keep standardness, fee/RBF, ancestor/descendant, and trim decisions in this crate.

### `open-bitcoin-node`

Recommended changes:

- Add a node-shell transaction relay bridge that owns effect execution for `TxRelayAction`.
- Persist accepted transaction records by txid and wtxid and remove replaced, evicted, or confirmed transactions.
- Add `StorageNamespace::Mempool` and versioned mempool DTOs to `FjallNodeStore`.
- Extend `ManagedNetworkInfo`, `ManagedMempoolInfo`, and shared status with relay/mempool participation evidence.
- Produce low-cardinality metric samples and structured logs for accepted, rejected, requested, served, orphaned, evicted, and rebroadcast decisions.
- Keep raw txids, peer ids, endpoint tables, and transaction hex out of support bundles unless a future explicit debug artifact adds redaction rules.

### `open-bitcoin-rpc`

Recommended changes:

- Keep `getmempoolinfo` compatibility-shaped and truthful with runtime/durable mempool state.
- Keep `getnetworkinfo.localrelay` tied to actual configured local relay behavior, not merely protocol support.
- Make `sendrawtransaction` return after local admission and queued relay, not after propagation.
- Add Open Bitcoin-specific relay detail to `openbitcoinnetworkstatus` or a sibling extension method. Prefer extending the existing network status response if the fields are low-cardinality.
- Consider `getrawmempool` and `testmempoolaccept` only after the admission/outcome types are stable. They are useful parity surfaces, but they should not precede the pure relay and status boundary.

### `open-bitcoin-cli`

Recommended changes:

- Render relay/mempool status from `OpenBitcoinStatusSnapshot`.
- Update support-bundle rendering and redaction for relay fields.
- Provide copy-pasteable Cargo and Bazel UAT commands for loopback relay review.
- Replace v1.9 wording that says relay-like permission labels are inactive once the new scoped behavior ships.

### `docs/parity`

Recommended changes:

- Update `catalog/mempool-policy.md` for durable/runtime mempool state, block/reorg hooks, and admission outcome events.
- Update `catalog/p2p.md` with a v2.0 transaction-relay section rooted in `net_processing.cpp`, `node/txdownloadman.*`, `txmempool.cpp`, `validation.cpp`, and relevant tests.
- Update `index.json` with v2.0 evidence and any intentional deviations.
- Update `source-breadcrumbs.json` for every new first-party Rust source/test file.
- Add or extend deterministic release-boundary checks so v2.0 cannot accidentally claim compact blocks, public relay defaults, production readiness, or package relay beyond its scoped evidence.

## Build Order And Phase Boundaries

Recommended roadmap phase order:

1. **Pure relay decision core**
   - Build `open-bitcoin-network::tx_relay`.
   - Inputs: peer relay info, txid/wtxid announcements, notfound, tx received, time value, permission effects.
   - Outputs: typed actions and decision events.
   - Verification: unit tests only; no sockets or storage.

2. **Mempool outcome contract**
   - Add stable admission/removal events to `open-bitcoin-mempool` and `ManagedMempool`.
   - Verification: pure mempool tests for accepted, duplicate, missing input, fee too low, non-standard, replacement, trim, and removal events.

3. **Managed runtime integration**
   - Wire `TransactionRelayManager` into `ManagedPeerNetwork`.
   - Replace ad hoc transaction request/serve paths with relay actions.
   - Verification: in-memory managed-node tests for inv/getdata/tx/notfound, local submit fanout, wtxidrelay, replacement, orphan parent request, and disconnect cleanup.

4. **Durable mempool and recovery evidence**
   - Add `StorageNamespace::Mempool`, tx records, metadata, load/repair behavior, and recovery markers.
   - Verification: Fjall adapter tests for save/load/remove/corruption/restart behavior.

5. **Runtime adapter activation**
   - Add explicit relay config and bounded runtime wiring in `open-bitcoind` and inbound/outbound loops.
   - Verification: loopback-safe UAT and deterministic local integration tests; public network remains opt-in.

6. **RPC, status, metrics, logs, support**
   - Extend shared status, `getmempoolinfo`, `getnetworkinfo`, `sendrawtransaction`, and Open Bitcoin network status.
   - Add metrics/log/support redaction.
   - Verification: RPC dispatch tests, CLI renderer tests, support redaction tests.

7. **Parity docs and release guardrails**
   - Update catalogs, breadcrumbs, release boundaries, and UAT docs.
   - Add deterministic v2.0 checker.
   - Verification: `bash scripts/verify.sh` plus specific checker outputs.

Do not start with daemon socket changes. That would put the riskiest policy work in the least testable layer.

## Research Flags For Later Phases

| Topic | Confidence | Reason |
| --- | --- | --- |
| Basic txid/wtxid announcement/request relay | HIGH | Existing Open Bitcoin code and Knots `TxDownloadManager` anchors are clear. |
| Mempool admission integration | HIGH | Existing `open-bitcoin-mempool` already owns core admission policy. |
| Permission activation | HIGH | v1.9 explicitly modeled relay-like labels as inactive; activation path is clear. |
| Durable mempool storage shape | MEDIUM | Current store has no mempool namespace; exact DTO and schema migration need phase design. |
| Orphan handling depth | MEDIUM | Knots behavior is rich; v2.0 should implement bounded orphan parent requests but defer broad package relay unless requirements expand. |
| Rolling bloom/recent filter parity | MEDIUM | Knots uses probabilistic recent filters; Open Bitcoin should define a deterministic test mode and document any intentional false-positive-profile deviation. |
| Fee filter/trickle fanout privacy | MEDIUM | Knots has nuanced timing/fanout behavior; v2.0 can bound and document a simpler first slice, then deepen with parity fixtures. |
| Compact blocks and package relay | HIGH as deferred | Sources show integration points, but project scope says these are not v2.0 claims. |

## Sources

Repo-local and standards sources:

- `AGENTS.md` repo-local guidance supplied in prompt
- `AGENTS.bright-builds.md`
- `standards/core/architecture.md`
- `standards/core/verification.md`
- `standards/core/testing.md`
- `standards/languages/rust.md`
- `/Users/peterryszkiewicz/.codex/get-shit-done/templates/research-project/ARCHITECTURE.md`

Open Bitcoin sources:

- `.planning/PROJECT.md`
- `.planning/ARCHITECTURE.md`
- `.planning/milestones/v1.9-ROADMAP.md`
- `packages/open-bitcoin-mempool/src/{lib.rs,types.rs,policy.rs,pool.rs,error.rs}`
- `packages/open-bitcoin-network/src/{lib.rs,message.rs,peer.rs,peer/inventory_state.rs,inbound.rs,inbound/permissions.rs,resource.rs}`
- `packages/open-bitcoin-node/src/{lib.rs,network.rs,network/inventory.rs,mempool.rs,status.rs,status/inbound.rs,metrics.rs,storage.rs,storage/fjall_store.rs}`
- `packages/open-bitcoin-rpc/src/{lib.rs,context.rs,context/network.rs,method.rs,method/node.rs,dispatch.rs,dispatch/node.rs,inbound_listener.rs,bin/open-bitcoind.rs}`
- `packages/open-bitcoin-cli/src/operator/status.rs`
- `packages/open-bitcoin-cli/src/operator/status/render.rs`
- `docs/parity/catalog/mempool-policy.md`
- `docs/parity/catalog/p2p.md`
- `docs/parity/index.json`
- `docs/parity/source-breadcrumbs.json`
- `docs/parity/deviations-and-unknowns.md`

Pinned Bitcoin Knots baseline sources:

- `packages/bitcoin-knots/src/node/txdownloadman.h`
- `packages/bitcoin-knots/src/node/txdownloadman_impl.cpp`
- `packages/bitcoin-knots/src/net_processing.cpp`
- `packages/bitcoin-knots/src/txmempool.cpp`
- `packages/bitcoin-knots/src/validation.cpp`
- `packages/bitcoin-knots/src/policy/policy.{h,cpp}`
- `packages/bitcoin-knots/src/policy/rbf.{h,cpp}`
- `packages/bitcoin-knots/src/policy/packages.{h,cpp}`
- `packages/bitcoin-knots/src/rpc/mempool.cpp`
- `packages/bitcoin-knots/src/rpc/rawtransaction.cpp`
- `packages/bitcoin-knots/test/functional/p2p_tx_download.py`
- `packages/bitcoin-knots/test/functional/p2p_permissions.py`
- `packages/bitcoin-knots/src/test/rbf_tests.cpp`
- `packages/bitcoin-knots/src/test/txpackage_tests.cpp`

*Architecture research for: v2.0 transaction relay and mempool participation boundary*
*Researched: 2026-06-29*
