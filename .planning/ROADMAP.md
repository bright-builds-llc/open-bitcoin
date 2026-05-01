# Roadmap: Open Bitcoin

## Milestones

- ✅ **v1.0 Headless Parity** — Phases 1 through 12 (shipped 2026-04-26). Archive: [v1.0-ROADMAP.md](milestones/v1.0-ROADMAP.md)
- ✅ **v1.1 Operator Runtime and Real-Network Sync** — Phases 13 through 34 (shipped 2026-04-30). Archive: [v1.1-ROADMAP.md](milestones/v1.1-ROADMAP.md)
- 🚧 **v1.2 Full Mainnet Network Syncing** — Phases 35 through 40 (active). Requirements: [REQUIREMENTS.md](REQUIREMENTS.md)

## Current Focus

v1.2 makes public-mainnet initial block download an explicit,
operator-ready `open-bitcoind` workflow. The milestone wires the existing
durable sync foundations into the daemon runtime so operators can opt in to
mainnet sync, connect to public outbound peers, validate and persist headers
and blocks, resume after restart, and inspect progress through current status,
dashboard, metrics, logs, and RPC-facing surfaces.

The current milestone does not claim broad production-node readiness,
production-funds wallet safety, full P2P policy parity, inbound peer serving,
transaction relay, or unattended packaged-service hardening.

## Phases

- [ ] **Phase 35: Daemon Mainnet Sync Activation** — Add the explicit daemon runtime boundary, config, flags, startup/shutdown semantics, and safety copy for opt-in public-mainnet sync.
- [ ] **Phase 36: Mainnet Peer Discovery and Outbound Lifecycle** — Resolve DNS/manual peers, maintain bounded outbound peer state, rotate unhealthy peers, and expose peer lifecycle telemetry.
- [ ] **Phase 37: Header-First Mainnet Sync Integration** — Drive validated header synchronization from durable state to the best known mainnet header chain through the daemon sync task.
- [ ] **Phase 38: Block Download, Connect, and Restart Recovery** — Download, validate, persist, and connect blocks with bounded in-flight work, reorg-aware state transitions, and restart recovery.
- [ ] **Phase 39: Operator Sync Observability and Control** — Make mainnet sync progress, health, stop/resume state, resource pressure, and support evidence truthful across status, dashboard, metrics, logs, and RPC surfaces.
- [ ] **Phase 40: Live Mainnet Smoke, Docs, and Parity Closeout** — Add opt-in live mainnet smoke/benchmark commands, refresh operator and parity docs, and close the milestone with auditable evidence.

## Phase Details

### Phase 35: Daemon Mainnet Sync Activation

**Goal:** Make `open-bitcoind` own an explicit, opt-in public-mainnet sync mode without changing default hermetic local workflows.
**Depends on:** v1.1 archive and current durable sync foundations
**Requirements:** SYNCMAIN-01, SYNCMAIN-02, SYNCMAIN-03, SYNCMAIN-04, RESUME-01, RESUME-04, VERMAIN-01

**Success Criteria:**

1. `open-bitcoind` has documented config and CLI flags that enable mainnet sync only when explicitly requested.
2. Daemon startup opens the durable store, constructs the sync runtime, and reports a coherent pre-sync status snapshot.
3. Startup, cancellation, graceful shutdown, and restart paths do not corrupt durable chainstate or leave misleading status.
4. Default tests and docs examples do not join public mainnet accidentally.
5. Operator-facing copy states the v1.2 support boundary and deferred production claims.

**Plans:** TBD by `/gsd-plan-phase 35`

### Phase 36: Mainnet Peer Discovery and Outbound Lifecycle

**Goal:** Give daemon sync a bounded, observable, and testable public-peer connection layer for initial block download.
**Depends on:** Phase 35
**Requirements:** PEERMAIN-01, PEERMAIN-02, PEERMAIN-03, PEERMAIN-04, RESUME-02, VERMAIN-01, VERMAIN-02

**Success Criteria:**

1. Mainnet DNS seed and manual-peer resolution use injectable resolvers for deterministic tests.
2. The runtime maintains bounded outbound peer state with timeout, retry, backoff, stall detection, and clean disconnect handling.
3. Flaky, stalled, or invalid-data peers cannot block IBD indefinitely when alternatives are available.
4. Peer telemetry records address source, negotiated network, capabilities, contribution, failure reason, and last activity.
5. Pure-core crates remain free of direct socket, DNS, clock, and filesystem effects.

**Plans:** TBD by `/gsd-plan-phase 36`

### Phase 37: Header-First Mainnet Sync Integration

**Goal:** Advance daemon-owned mainnet header synchronization from durable state to the best known validated header chain.
**Depends on:** Phase 36
**Requirements:** SYNCMAIN-03, CHAINMAIN-01, CHAINMAIN-03, RESUME-01, RESUME-02, VERMAIN-01, VERMAIN-02

**Success Criteria:**

1. Header sync starts from genesis, checkpoint, or durable store tip and persists validated progress.
2. Restart recovery avoids replaying verified header work unnecessarily.
3. Competing header branches produce deterministic active-header-chain selection and typed invalid-data errors.
4. Status snapshots distinguish header progress from block and chainstate progress.
5. Hermetic tests cover happy path, restart, invalid headers, peer disconnects, and competing branches.

**Plans:** TBD by `/gsd-plan-phase 37`

### Phase 38: Block Download, Connect, and Restart Recovery

**Goal:** Turn validated headers into durable active-chain progress by downloading, validating, persisting, and connecting mainnet blocks.
**Depends on:** Phase 37
**Requirements:** SYNCMAIN-03, CHAINMAIN-02, CHAINMAIN-03, CHAINMAIN-04, CHAINMAIN-05, RESUME-01, RESUME-02, RESUME-03, VERMAIN-01, VERMAIN-02

**Success Criteria:**

1. The daemon requests blocks needed to advance the active chain with bounded in-flight work per peer and globally.
2. Downloaded blocks are validated, persisted, and connected using existing consensus and chainstate rules.
3. Restart recovery handles partial downloads, interrupted writes, and already-connected blocks without corrupting state.
4. Reorg-like competing branches use durable undo or replay-safe state transitions.
5. Resource limits and insufficient-resource conditions produce actionable operator guidance.

**Plans:** TBD by `/gsd-plan-phase 38`

### Phase 39: Operator Sync Observability and Control

**Goal:** Make daemon mainnet sync understandable and controllable through existing operator surfaces.
**Depends on:** Phase 38
**Requirements:** PEERMAIN-04, CHAINMAIN-05, RESUME-04, OBSMAIN-01, OBSMAIN-02, OBSMAIN-03, OBSMAIN-04, VERMAIN-01, VERMAIN-02

**Success Criteria:**

1. `open-bitcoin status`, JSON status, and dashboard views show live sync state, peer count, best header height, best block height, progress, lag, current phase, resource pressure, and last error.
2. Metrics and logs capture sync milestones, peer rotation, throughput, validation/connect rates, recovery, and failure causes with bounded retention.
3. RPC and CLI blockchain-info surfaces stay truthful during IBD and never imply full sync before validated chainstate reaches the selected tip.
4. Operators can stop, resume, inspect recovery guidance, and collect support evidence without reading internal store files.
5. Non-interactive tests verify status truthfulness for active, paused, stopped, recovering, and failed sync states.

**Plans:** TBD by `/gsd-plan-phase 39`

### Phase 40: Live Mainnet Smoke, Docs, and Parity Closeout

**Goal:** Prove the v1.2 daemon sync workflow with opt-in live evidence and update docs so claims match shipped behavior.
**Depends on:** Phase 39
**Requirements:** SYNCMAIN-04, CHAINMAIN-05, RESUME-03, OBSMAIN-04, VERMAIN-01, VERMAIN-03, VERMAIN-04, VERMAIN-05

**Success Criteria:**

1. Optional live mainnet smoke and benchmark commands can verify real `open-bitcoind` progress and emit reproducible reports.
2. Public-network checks fail clearly when network, disk, time, or configuration prerequisites are missing, and they stay out of default verification.
3. Operator docs cover prerequisites, disk/network expectations, config, startup commands, status interpretation, stop/resume, troubleshooting, and known limitations.
4. Parity docs and machine-readable indexes distinguish v1.2 shipped claims from deferred Knots/Core behavior.
5. `bash scripts/verify.sh`, GSD health, diff checks, and milestone evidence are clean enough for archive/audit handoff.

**Plans:** TBD by `/gsd-plan-phase 40`

## Progress

| Milestone | Phases | Plans | Status | Shipped |
| --- | ---: | ---: | --- | --- |
| v1.0 Headless Parity | 22/22 | 80/80 | Archived | 2026-04-26 |
| v1.1 Operator Runtime and Real-Network Sync | 22/22 | 69/69 | Archived | 2026-04-30 |
| v1.2 Full Mainnet Network Syncing | 0/6 | TBD | Active | - |
