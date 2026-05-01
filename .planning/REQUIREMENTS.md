# Requirements: Open Bitcoin

**Defined:** 2026-05-01
**Core Value:** When a behavior is in scope, Open Bitcoin must behave like the pinned Knots baseline on the outside while staying simpler and safer on the inside.
**Milestone:** v1.2 Full Mainnet Network Syncing

## v1.2 Scope

v1.2 makes full public-mainnet initial block download an explicit, operator-ready
`open-bitcoind` workflow. The milestone wires the shipped durable sync
foundations into the daemon runtime so an operator can opt in to mainnet sync,
connect to public outbound peers, validate and persist headers and blocks,
resume safely after restart, and inspect progress through existing status,
dashboard, metrics, logs, and RPC-facing surfaces.

This milestone does not claim broad production-node readiness, production-funds
wallet safety, full P2P policy parity, inbound peer serving, transaction relay,
or unattended packaged-service hardening.

## v1.2 Requirements

### Daemon Sync Activation and Safety

- [ ] **SYNCMAIN-01**: Operator can start `open-bitcoind` with an explicit mainnet sync mode through documented config and CLI flags, and the daemon opens the durable node store before joining public peers.
- [ ] **SYNCMAIN-02**: Mainnet public-network behavior is opt-in and cannot be triggered accidentally by default local verification, status-only commands, docs examples, or hermetic tests.
- [ ] **SYNCMAIN-03**: The daemon owns a long-lived sync task with bounded startup, graceful shutdown, cancellation, and restart semantics that do not corrupt durable chainstate.
- [ ] **SYNCMAIN-04**: Operator docs and runtime warnings clearly distinguish v1.2 mainnet sync support from deferred production-node, production-wallet, and full drop-in replacement claims.

### Peer Discovery and Connectivity

- [ ] **PEERMAIN-01**: Daemon sync can resolve configured mainnet DNS seeds and manual peers, with deterministic tests using injected resolvers instead of public DNS.
- [ ] **PEERMAIN-02**: The runtime maintains a bounded outbound peer set with per-peer lifecycle state, connection timeout, retry backoff, stall detection, and clean disconnect handling.
- [ ] **PEERMAIN-03**: Peer selection and rotation prefer healthy peers without letting flaky, stalled, or invalid-data peers block initial block download indefinitely.
- [ ] **PEERMAIN-04**: Peer telemetry records address source, negotiated network, service capability summary, sync contribution, failure reason, and last activity without leaking low-level socket details into pure-core crates.

### Headers, Blocks, and Chain Progress

- [ ] **CHAINMAIN-01**: Daemon sync performs header-first mainnet synchronization from genesis, checkpoint, or durable store tip to the best known header chain using existing consensus validation rules.
- [ ] **CHAINMAIN-02**: Daemon sync requests, downloads, validates, persists, and connects mainnet blocks needed to advance the active chain with bounded in-flight work.
- [ ] **CHAINMAIN-03**: Chain progress survives process restart, interrupted writes, peer disconnects, and partial block downloads without replaying verified work unnecessarily.
- [ ] **CHAINMAIN-04**: Competing branches and reorgs are handled through durable undo or replay-safe state transitions, with typed errors for invalid or unrecoverable states.
- [ ] **CHAINMAIN-05**: Initial block download enforces documented memory, disk, peer, and request limits, and surfaces operator guidance when local resources are insufficient.

### Runtime Resilience

- [ ] **RESUME-01**: Restart recovery reconstructs the daemon sync plan from durable headers, block index metadata, chainstate, runtime metadata, and any incomplete download state.
- [ ] **RESUME-02**: Invalid blocks, invalid headers, malformed messages, timeouts, stalls, resolver failures, and storage errors produce typed runtime failures and operator-visible health signals instead of panics.
- [ ] **RESUME-03**: The sync runtime includes reindex, repair, or resync guidance for corrupted or incompatible stores without mutating user data implicitly.
- [ ] **RESUME-04**: Shutdown, restart, and service-manager stop flows leave a coherent status snapshot that explains whether sync is active, paused, stopped, recovering, or failed.

### Operator Observability

- [ ] **OBSMAIN-01**: `open-bitcoin status`, JSON status, and dashboard views expose live daemon-sync state, peer count, best header height, best block height, chainwork or equivalent progress signal, estimated lag, current phase, and last error.
- [ ] **OBSMAIN-02**: Metrics and structured logs capture sync milestones, peer rotation, download throughput, validation/connect rates, restart recovery, resource pressure, and failure causes with bounded retention.
- [ ] **OBSMAIN-03**: RPC and CLI surfaces that report blockchain info stay truthful during IBD and do not imply full sync before validated chainstate reaches the selected tip.
- [ ] **OBSMAIN-04**: Operators can stop and resume sync, inspect recovery guidance, and collect support evidence without manually parsing internal store files.

### Verification, Parity Evidence, and Docs

- [ ] **VERMAIN-01**: Each v1.2 phase keeps `bash scripts/verify.sh` hermetic and passing by default; public-network tests remain explicitly opt-in.
- [ ] **VERMAIN-02**: Deterministic integration tests cover peer failures, resolver failures, invalid data, restarts, partial stores, reorg-like competing branches, bounded resources, and status truthfulness without depending on public mainnet.
- [ ] **VERMAIN-03**: Optional live mainnet smoke and benchmark commands verify that `open-bitcoind` can make real mainnet progress, emit reproducible reports, and fail clearly when network or resource prerequisites are missing.
- [ ] **VERMAIN-04**: Operator docs explain prerequisites, disk/network expectations, config, startup commands, status interpretation, stop/resume behavior, troubleshooting, and known limitations for v1.2 mainnet sync.
- [ ] **VERMAIN-05**: Parity docs, checklists, and machine-readable indexes distinguish v1.2 shipped mainnet-sync claims from deferred Knots/Core behaviors.

## Future Requirements

### Deferred P2P and Node-Operation Surfaces

- **P2P-ADDRESS-RELAY**: Address relay, addrman behavior, peer gossip, eviction, discouragement, ban-score compatibility, and long-term peer database parity.
- **P2P-INBOUND**: Inbound peer serving, listening sockets, peer permissioning, whitelisting, and externally reachable node operation.
- **P2P-COMPACT**: Compact block relay, high-bandwidth relay modes, block reconstruction, and advanced bandwidth optimization.
- **MEMPOOL-RELAY**: Mainnet transaction relay, orphan handling, fee policy tuning, mempool expiry, replacement, and package relay behavior beyond already scoped mempool primitives.
- **WALLET-PROD**: Production-funds wallet claims, unattended mainnet wallet use, hardware wallet flows, and migration apply-mode wallet mutation.
- **PKG-SVC-PROD**: Signed packages, OS-native unattended service installation, restart policies, OS upgrade handling, and production runbook certification.

## Out of Scope

| Feature | Reason |
|---------|--------|
| Full production-node claim | v1.2 targets operator-ready mainnet IBD testing, not full production hardening across all P2P and service surfaces. |
| Production-funds wallet support | Chain sync must land before extending wallet safety claims for real funds. |
| Inbound peer serving | Initial block download only needs outbound public peers; serving the network is deferred. |
| Address relay, banning, eviction, and addrman parity | These are large P2P policy surfaces and should not block daemon IBD. |
| Mempool transaction relay | Full mempool relay belongs in a later milestone after chain sync is daemon-owned. |
| Destructive migration apply mode | Existing Core/Knots datadirs and wallets remain dry-run-only migration inputs. |
| Public-network tests in default verification | Default verification must stay hermetic and deterministic. |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| SYNCMAIN-01 | Phase 35 | Planned |
| SYNCMAIN-02 | Phase 35 | Planned |
| SYNCMAIN-03 | Phase 35, Phase 37, Phase 38 | Planned |
| SYNCMAIN-04 | Phase 35, Phase 40 | Planned |
| PEERMAIN-01 | Phase 36 | Planned |
| PEERMAIN-02 | Phase 36 | Planned |
| PEERMAIN-03 | Phase 36 | Planned |
| PEERMAIN-04 | Phase 36, Phase 39 | Planned |
| CHAINMAIN-01 | Phase 37 | Planned |
| CHAINMAIN-02 | Phase 38 | Planned |
| CHAINMAIN-03 | Phase 37, Phase 38 | Planned |
| CHAINMAIN-04 | Phase 38 | Planned |
| CHAINMAIN-05 | Phase 38, Phase 39 | Planned |
| RESUME-01 | Phase 35, Phase 37, Phase 38 | Planned |
| RESUME-02 | Phase 36, Phase 37, Phase 38 | Planned |
| RESUME-03 | Phase 38, Phase 40 | Planned |
| RESUME-04 | Phase 35, Phase 39 | Planned |
| OBSMAIN-01 | Phase 39 | Planned |
| OBSMAIN-02 | Phase 39 | Planned |
| OBSMAIN-03 | Phase 39 | Planned |
| OBSMAIN-04 | Phase 39, Phase 40 | Planned |
| VERMAIN-01 | Phase 35, Phase 36, Phase 37, Phase 38, Phase 39, Phase 40 | Planned |
| VERMAIN-02 | Phase 36, Phase 37, Phase 38, Phase 39 | Planned |
| VERMAIN-03 | Phase 40 | Planned |
| VERMAIN-04 | Phase 40 | Planned |
| VERMAIN-05 | Phase 40 | Planned |

**Coverage:**

- v1.2 requirements: 26 total
- Checked off: 0
- Mapped to phases: 26
- Unmapped: 0

---
*Requirements defined: 2026-05-01*
