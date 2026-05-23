# Phase 36: Mainnet Peer Discovery and Outbound Lifecycle - Research

**Researched:** 2026-05-01  
**Domain:** Deterministic peer resolution, bounded outbound peer lifecycle management, and sync telemetry expansion for daemon-owned mainnet IBD.  
**Confidence:** HIGH for repo integration seams and plan slicing; MEDIUM for the exact peer-pool policy constants until implementation validates the ergonomics.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

#### Resolver and address sourcing
- **D-01:** Resolve manual peers and DNS seeds through an injected resolver interface owned by the sync shell, never by pure-core crates or direct calls from tests.
- **D-02:** Treat manual peers and DNS seeds as separate sources in the domain model and preserve that source label in telemetry and failure reporting.
- **D-03:** Normalize resolved candidates into explicit socket-address domain records before connection attempts so downstream sync logic does not re-parse host strings.

#### Outbound peer pool
- **D-04:** Replace the current sequential `candidate_peers()` walk with a bounded outbound peer pool that tracks candidate, connecting, connected, stalled, and failed lifecycle states plus retry metadata.
- **D-05:** Keep pool policy deterministic and adapter-driven: target outbound count, connect timeout, read/stall timeout, retry budget, and backoff schedule must all come from typed runtime config.
- **D-06:** When alternatives exist, stalled or invalid peers must be disconnected and deprioritized instead of allowing one bad peer to monopolize the run.

#### Failure semantics and resilience
- **D-07:** Introduce typed peer-lifecycle failure reasons for resolver failure, connect timeout, stall timeout, invalid magic/data, transport I/O, and policy-driven disconnects so `RESUME-02` stays explicit.
- **D-08:** Persist only the durable sync artifacts already owned by the runtime store in this phase; peer pool state itself may stay in-memory for now as long as failure outcomes and health signals are surfaced truthfully.
- **D-09:** Keep later header/block phases unblocked by making the peer layer reusable through transport/session abstractions rather than binding header progress directly to one transport loop.

#### Telemetry and observability
- **D-10:** Extend sync telemetry to record peer source, resolved endpoint, negotiated network identity, service-capability summary, contribution counters, last activity, attempts, and terminal failure reason.
- **D-11:** Keep the telemetry model shell-safe but transport-agnostic so current status/log/metric surfaces can consume it without leaking sockets, DNS, or clocks into pure-core crates.

### the agent's Discretion
- Exact outbound target count and backoff constants.
- Whether resolver output is consumed lazily or batched per round, as long as tests remain deterministic.
- The minimum status/log surface needed now before Phase 39 expands operator presentation.

### Deferred Ideas (OUT OF SCOPE)
- Persistent peer database / addrman parity, peer gossip, eviction, banning, and inbound serving.
- Full operator dashboard and RPC presentation polish for peer telemetry.
- Header/block scheduling and restart recovery beyond the peer-layer boundary.

</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| PEERMAIN-01 | Resolve configured mainnet DNS seeds and manual peers with deterministic tests using injected resolvers. | Split transport resolution from `TcpPeerTransport` into an injected resolver adapter and promote resolved endpoints into typed runtime records. |
| PEERMAIN-02 | Maintain bounded outbound peer lifecycle state with timeout, retry backoff, stall detection, and clean disconnect handling. | Introduce a shell-layer peer-pool state machine around `ManagedPeerNetwork` and `SyncTransport`, keeping protocol decisions in existing pure-core peer logic. |
| PEERMAIN-03 | Prefer healthy peers and rotate away flaky peers so IBD cannot be blocked indefinitely. | Track peer health, retry windows, and deprioritization in the pool manager; only reconnect failed peers after backoff and only when capacity is available. |
| PEERMAIN-04 | Record peer source, negotiated network, service capability summary, sync contribution, failure reason, and last activity without leaking low-level socket details into pure-core crates. | Extend sync summary and peer telemetry contracts in `open-bitcoin-node` rather than adding socket-heavy status types to pure-core crates. |
| RESUME-02 | Invalid data, timeouts, stalls, resolver failures, and storage errors produce typed runtime failures and health signals instead of panics. | Add typed peer lifecycle failure enums and map them to existing `HealthSignal` / structured-log projections. |
| VERMAIN-01 | Keep `bash scripts/verify.sh` hermetic and passing by default. | Use scripted resolvers and transports in tests; keep live DNS/network behavior out of default verification. |
| VERMAIN-02 | Deterministic integration tests cover peer failures, resolver failures, invalid data, restarts, partial stores, bounded resources, and status truthfulness. | Extend `packages/open-bitcoin-node/src/sync/tests.rs` with fake resolver + session scenarios and status/log assertions. |

</phase_requirements>

<research_summary>
## Summary

Phase 36 should be implemented as a shell-layer peer orchestration refactor, not as a rewrite of the existing protocol state machine. The current `DurableSyncRuntime` already has the right ownership boundaries: it owns the store, transport abstraction, metric/log persistence, and `ManagedPeerNetwork` integration. The main problem is that it still treats peers as a flat sequential list of host strings and leaves DNS resolution embedded in `sync/tcp.rs`. That makes deterministic tests difficult, blocks a bounded outbound pool, and prevents the runtime from distinguishing candidate selection, connection attempts, active peers, and terminal failures.

The cleanest path is to split Phase 36 into three technical layers. First, expand the sync config and shell-domain types so peer resolution is explicit and injectable: manual peers and seed hostnames should resolve into typed endpoint candidates carrying source metadata, with a resolver trait that fake tests can implement. Second, add a peer-pool coordinator in `open-bitcoin-node` that maintains lifecycle state, retry/backoff windows, last activity, contribution counters, and clean disconnect reasons while continuing to delegate handshake and message processing to `ManagedPeerNetwork`/`PeerManager`. Third, project the richer peer outcomes into summary/status/log contracts so later phases can surface the same truth without re-deriving it from ad hoc strings or socket errors.

The existing pure-core crates already contain the right reusable primitives. `PeerManager` understands version/verack, headers, inventory, and bounded block requests. `ManagedPeerNetwork` already mediates between peer actions and chainstate/mempool effects. `SyncRuntimeError`, `SyncRunSummary`, and the Phase 16 metrics/log/status work provide a ready-made projection path for peer lifecycle evidence. Phase 36 should therefore avoid adding DNS or transport details to `open-bitcoin-network`; instead, it should create a small shell-owned peer coordinator plus richer shell-owned telemetry records in `open-bitcoin-node`.

**Primary recommendation:** keep `PeerManager` and `ManagedPeerNetwork` as the functional core, split resolver logic out of `sync/tcp.rs`, add a shell-owned bounded peer pool with typed failure/backoff state in `open-bitcoin-node`, and extend Phase 16 telemetry contracts to carry peer lifecycle evidence.

</research_summary>

<reusable_assets>
## Reusable Assets and Constraints

### Existing seams that should stay

| Asset | Current Role | Reuse Guidance |
|---|---|---|
| `packages/open-bitcoin-node/src/sync.rs` | Owns store, transport/session abstraction, persistence, and summary generation. | Keep it as the imperative shell entrypoint; move peer orchestration into a sibling module rather than bloating `sync.rs`. |
| `packages/open-bitcoin-node/src/sync/types.rs` | Holds sync config, peer-address types, summary/status/log projections, and runtime errors. | Expand it with resolved endpoint, peer lifecycle, and backoff/timeout config types. |
| `packages/open-bitcoin-node/src/sync/tcp.rs` | Owns DNS + socket effects for real TCP sessions. | Split it into resolver + transport helpers; keep `TcpPeerSession` small and effectful. |
| `packages/open-bitcoin-network/src/peer.rs` | Pure-core handshake, header, inventory, and request state machine. | Do not move DNS/socket/pool policy here. Reuse it per connected peer. |
| `packages/open-bitcoin-node/src/status.rs` | Shared status contracts used by CLI/dashboard/RPC. | Extend peer status carefully so later phases can expose richer peer truth without incompatible rewrites. |
| `packages/open-bitcoin-node/src/sync/tests.rs` | Hermetic scripted-transport sync tests. | Add a fake resolver and peer-pool scenarios here instead of live-network defaults. |

### Existing constraints that matter

- `SyncRuntimeConfig` currently carries `manual_peers`, `dns_seeds`, timeouts, retry count, and persist mode, but it does not model a bounded pool or resolved endpoints.
- `TcpPeerTransport::connect` currently calls `to_socket_addrs()` directly; this is the main deterministic-testing and layering problem for `PEERMAIN-01`.
- `SyncRunSummary` and `PeerSyncOutcome` only track coarse connected/stalled/failed results, not lifecycle history or negotiated-capability details.
- `PeerStatus` only reports counts today, so richer telemetry must not break existing consumers while still giving later phases truthful peer evidence.

</reusable_assets>

<recommended_structure>
## Recommended Structure

```text
packages/open-bitcoin-node/src/
|-- sync.rs
|-- sync/
|   |-- pool.rs          # bounded outbound peer lifecycle coordinator
|   |-- resolver.rs      # injected resolver trait + TCP implementation
|   |-- tcp.rs           # session/socket transport only
|   |-- tests.rs
|   `-- types.rs
```

- `resolver.rs` should own `SyncPeerResolver` and a TCP/system resolver implementation.
- `pool.rs` should own lifecycle state, backoff policy, endpoint selection, and outcome aggregation.
- `sync.rs` should orchestrate rounds using the resolver + pool + session transport and persist summary evidence.
- `types.rs` should own the durable config and telemetry shapes used across these modules.

</recommended_structure>

<implementation_patterns>
## Implementation Patterns

### Pattern 1: Parse at the shell boundary into resolved endpoint types

Use the shell boundary to convert seed/manual peer config into explicit resolved endpoints with source metadata and port information. This follows the architecture rule to parse boundary inputs once, then operate on trusted domain types.

Likely types:
- `ResolvedSyncPeerEndpoint { address: SocketAddr, source: SyncPeerSource, label: String }`
- `SyncPeerCandidate { address: SyncPeerAddress, source: SyncPeerSource }`
- `SyncPeerResolver` trait with a deterministic fake in tests.

### Pattern 2: Bounded pool manager, not sequential candidate walking

The current `for peer in &peers` flow in `sync_once` should become a pool loop that:
- resolves a candidate set
- selects up to `target_outbound_peers`
- drives each active peer until it stalls, fails, or exhausts message budget
- records last activity and contribution counters
- rotates out unhealthy peers when healthier candidates remain

This should stay shell-owned. `PeerManager` remains the protocol engine for a connected peer.

### Pattern 3: Typed peer lifecycle failures projected through existing health/log surfaces

Prefer a dedicated failure enum such as:
- `Resolution`
- `ConnectTimeout`
- `ReadTimeout`
- `Stall`
- `InvalidMagic`
- `InvalidPayload`
- `TransportIo`
- `PolicyDisconnect`

Then map these into:
- peer telemetry records
- `HealthSignal`
- structured log records
- summary-level failure counters

That preserves `RESUME-02` and keeps later phases from parsing free-form error strings.

### Pattern 4: Compatibility-first status expansion

Do not replace existing `PeerStatus.peer_counts`. Append richer fields with `FieldAvailability` wrappers, for example:
- lifecycle summary counts
- a bounded recent-peer/outcome list
- maybe peer activity timestamps/counters

This keeps current status consumers compatible while enabling truthful later UI/RPC surfaces.

</implementation_patterns>

<test_strategy>
## Test Strategy

### Deterministic unit/integration coverage to add

1. Resolver behavior
- manual peers bypass DNS lookup but still normalize into endpoints
- DNS seeds resolve through the injected resolver
- resolver failures become typed peer lifecycle failures and health signals

2. Bounded pool behavior
- pool never exceeds target outbound count
- stalled/failed peers are rotated out when alternatives exist
- backoff prevents immediate reconnect storms
- duplicate endpoints are de-duplicated deterministically

3. Peer lifecycle truthfulness
- negotiated network/capability state is captured after handshake
- last activity and contribution counters update as messages are processed
- invalid data and disconnects produce typed terminal reasons

4. Status/log/report projections
- peer telemetry serializes cleanly through shared status types
- structured logs stay concise and omit low-level socket/path leakage
- summary counters stay truthful when there are mixed success/failure peers

5. Repo-native verification
- keep all tests hermetic in `cargo test`
- no live DNS or public-network dependence in default runs

</test_strategy>

<plan_slices>
## Recommended Plan Slices

### Slice 1: Config and resolver boundary
- Expand `SyncRuntimeConfig` with bounded-pool policy knobs and explicit resolver-facing peer candidate types.
- Add `SyncPeerResolver` plus the real TCP/system resolver implementation.
- Extend JSONC / daemon config loading if Phase 36 needs operator-configured manual peers, DNS seeds, or pool limits now.
- Add deterministic resolver/config tests.

### Slice 2: Bounded outbound lifecycle manager
- Introduce peer-pool state and lifecycle transitions in `open-bitcoin-node`.
- Refactor `DurableSyncRuntime::sync_once` around pool orchestration instead of sequential `candidate_peers()` walking.
- Track retries, backoff windows, last activity, and typed disconnect/failure reasons.
- Add hermetic pool tests with scripted sessions.

### Slice 3: Telemetry, status, and closeout
- Extend summary/status/log projections for richer peer telemetry while preserving compatibility.
- Update parity breadcrumbs/docs for any new first-party Rust files.
- Add final phase verification and planning-state updates after code verification passes.

</plan_slices>

<risks>
## Key Risks

1. **Overloading pure-core crates with shell policy**
- Avoid by keeping resolver, socket, backoff, and lifecycle policy in `open-bitcoin-node`.

2. **Status contract churn**
- Avoid by appending optional peer telemetry fields rather than replacing existing counts.

3. **Pool complexity overwhelming `sync.rs`**
- Avoid by extracting `pool.rs` / `resolver.rs` instead of growing one large runtime file.

4. **Flaky tests from time-dependent backoff**
- Avoid by representing backoff with injected timestamps or deterministic counters instead of sleeping.

5. **Configuration sprawl**
- Avoid by only exposing the minimal operator knobs Phase 36 truly needs; keep policy defaults sensible and typed.

</risks>

## RESEARCH COMPLETE

Strongest recommendations:
- Split DNS resolution out of `packages/open-bitcoin-node/src/sync/tcp.rs` behind an injected shell-layer resolver.
- Replace the sequential peer walk in `DurableSyncRuntime::sync_once` with a bounded shell-owned peer pool that tracks lifecycle, backoff, and failure truth.
- Extend `SyncRunSummary` / peer status contracts with richer peer telemetry instead of inventing a second observability model.
