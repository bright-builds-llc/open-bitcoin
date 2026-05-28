# Roadmap: Open Bitcoin

## Milestones

- ✅ **v1.0 Headless Parity** - Phases 1 through 12 (shipped 2026-04-26). Archive: [v1.0-ROADMAP.md](milestones/v1.0-ROADMAP.md)
- ✅ **v1.1 Operator Runtime and Real-Network Sync** - Phases 13 through 34 (shipped 2026-04-30). Archive: [v1.1-ROADMAP.md](milestones/v1.1-ROADMAP.md)
- ✅ **v1.2 Full Mainnet Network Syncing** - Phases 35 through 41 (shipped 2026-05-23). Archive: [v1.2-ROADMAP.md](milestones/v1.2-ROADMAP.md)
- 🚧 **v1.3 Public Mainnet Sync Proof and Node Hardening** - Phases 42 through 50 (active).

## Current Focus

v1.3 proves real public-mainnet sync progress through the explicit opt-in
daemon workflow and hardens the node/runtime surfaces needed before any broader
production-node claim. The milestone focuses on durable sync proof, peer
resilience, data integrity, operator observability, and auditable release
boundaries.

The milestone does not include inbound serving, transaction relay,
production-funds wallet use, migration apply mode, packaging, hosted dashboard,
GUI work, or adding public-network checks to the default `bash scripts/verify.sh`
gate.

## Phases

- [x] **Phase 42: Live Smoke Entry and Network Preflight** - Make opt-in live smoke prerequisites and endpoint outcomes explicit before sync.
- [x] **Phase 43: Outbound Peer Resilience** - Keep daemon sync alive and truthful while public peers fail, stall, disconnect, or provide bad data.
- [x] **Phase 44: Peer Contribution Attribution** - Attribute header and block progress to useful peers instead of idle or failing peers. (completed 2026-05-25)
- [x] **Phase 45: Runtime Resource Bounds and Store Coordination** - Keep long public-network runs bounded and single-writer safe. (completed 2026-05-26)
- [x] **Phase 46: Durable Recovery and Invalid Data Handling** - Resume after partial work and reject invalid peer data without advancing chainstate. (completed 2026-05-26)
- [x] **Phase 47: Operator Sync Truth Surfaces** - Keep status, dashboard, metrics, logs, and RPC surfaces consistent during sync. (completed 2026-05-26)
- [x] **Phase 48: Support Evidence and Operator Runbooks** - Produce redacted support evidence and repo-local operator instructions. (completed 2026-05-27)
- [x] **Phase 49: Threat Model and Release Boundaries** - Refresh v1.3 security analysis and scoped parity/release claims. (completed 2026-05-27)
- [x] **Phase 50: Public Mainnet Progress Evidence Closeout** - Capture header, block, and restart/resume proof or diagnose the blocker. (completed 2026-05-28)

## Phase Details

### Phase 42: Live Smoke Entry and Network Preflight
**Goal**: Operators can start an opt-in live-mainnet smoke run with explicit inputs and immediately understand prerequisite or peer-connection failures.
**Depends on**: Phase 41
**Requirements**: PROOF-01, PROOF-02, PEER-01
**Success Criteria** (what must be TRUE):
  1. Operator can run the live smoke command with explicit datadir, timeout, polling interval, and optional manual peers, and it refuses missing prerequisites before starting network work.
  2. Operator can preview DNS seed and manual-peer outcomes showing resolved, connected, handshook, failed, or skipped endpoints.
  3. Live smoke reports identify no-progress causes such as DNS resolution, TCP connection, handshake, unsupported capability, validation, storage, timeout, and operator cancellation.
  4. Operator cancellation is reported as a distinct outcome rather than a crash or generic timeout.
**Plans**: 42-01 completed - Live smoke network preflight and typed no-progress evidence.

### Phase 43: Outbound Peer Resilience
**Goal**: Daemon sync stays alive and truthful while public peers fail, stall, disconnect, or provide bad data.
**Depends on**: Phase 42
**Requirements**: PEER-02, PEER-04
**Success Criteria** (what must be TRUE):
  1. Operator can see bounded outbound peer counts during sync and stable retry, backoff, stall, and replacement reasons when peers are unhealthy.
  2. Daemon sync rotates unhealthy peers and replaces failed connections without exceeding configured peer limits.
  3. Mixed peer failures, disconnects, timeouts, and invalid data do not corrupt durable state or exit the sync runtime unexpectedly.
  4. Failure states show whether sync is still retrying, waiting for peers, or stopped.
**Plans**: 43-01 completed - Backoff visibility and mixed peer failure resilience.

### Phase 44: Peer Contribution Attribution
**Goal**: Sync progress reports identify which peers contributed validated headers or blocks and avoid crediting idle peers.
**Depends on**: Phase 43
**Requirements**: PEER-03
**Success Criteria** (what must be TRUE):
  1. Operator can inspect per-peer header and block contribution in sync telemetry or reports.
  2. Idle peers are visible as idle rather than counted as useful sync progress.
  3. Failing peers retain last activity and failure reason separate from contributed progress.
  4. Peer contribution data remains available to live smoke and support evidence flows.
**Plans**: 1/1 plans complete
Plans:
- [x] 44-01-PLAN.md — Validation-gated peer contribution telemetry and live-smoke evidence.

### Phase 45: Runtime Resource Bounds and Store Coordination
**Goal**: Public-network sync stays within documented resource limits and preserves single-writer durable-store coordination during operator controls.
**Depends on**: Phase 44
**Requirements**: NODE-01, NODE-04
**Success Criteria** (what must be TRUE):
  1. Operator can inspect documented bounds for in-flight headers, in-flight blocks, durable writes, metrics retention, and log retention.
  2. Long-running sync uses those bounds without unbounded queue, log, metrics, or write growth.
  3. Pause, resume, stop, and status flows leave coherent durable status.
  4. A second runtime or control action cannot create an undiagnosed second-writer store conflict.
**Plans**:
  - [x] 45-01-PLAN.md — Runtime resource bounds and store coordination.

### Phase 46: Durable Recovery and Invalid Data Handling
**Goal**: Durable sync recovers from partial work and invalid peer data without losing validated progress or advancing a bad chain.
**Depends on**: Phase 45
**Requirements**: NODE-02, NODE-03, NODE-05
**Success Criteria** (what must be TRUE):
  1. Operator can restart after partial downloads, partial validation, or partial connects and see validated progress resume without duplicated block connects.
  2. Invalid headers or blocks are rejected with peer attribution and do not advance active chainstate.
  3. Recovery guidance distinguishes transient network failures, incompatible stores, corrupt stores, resource exhaustion, and intentional cancellation.
  4. Durable status after recovery separates validated header, downloaded block, connected block, and error state.
**Plans**:
  - [x] 46-01-PLAN.md — Durable recovery and invalid data handling.

### Phase 47: Operator Sync Truth Surfaces
**Goal**: Operator-facing status surfaces tell the same truth about sync phase, peer health, progress, lag, and errors.
**Depends on**: Phase 46
**Requirements**: OBS-01, OBS-02
**Success Criteria** (what must be TRUE):
  1. Operator can inspect JSON status with phase, outbound peer count, peer outcomes, best header height, best block height, progress signal, estimated lag, last successful progress, and last error.
  2. Status, dashboard, metrics, structured logs, and RPC blockchain info agree on current progress and failure state.
  3. No operator surface implies full sync until validated chainstate reaches the selected tip.
  4. Status remains useful during active, paused, resumed, stopped, failed, and recovering sync states.
**Plans**:
  - [x] 47-01-PLAN.md — Operator sync truth surfaces.
**UI hint**: yes

### Phase 48: Support Evidence and Operator Runbooks
**Goal**: Operators and reviewers can collect redacted support evidence and follow repo-local runbooks for v1.3 proof.
**Depends on**: Phase 47
**Requirements**: OBS-03, OBS-04
**Success Criteria** (what must be TRUE):
  1. Operator can generate a redacted support evidence bundle or equivalent report with config sources, command versions, sync status, peer outcomes, recent logs, metrics, store health, and live smoke artifacts.
  2. Sensitive data in support evidence is redacted or omitted while preserving enough context for review.
  3. Operator docs include repo-local Cargo and Bazel commands, manual-peer examples, disk and network expectations, troubleshooting, and pass/fail interpretation.
  4. Docs explain how to use local artifacts without requiring hosted services, packaged installs, or destructive migration.
**Plans**:
- [x] 50-01-PLAN.md — Public-mainnet evidence capture, UAT summary, and parity closeout.

### Phase 49: Threat Model and Release Boundaries
**Goal**: Reviewers can audit v1.3 security posture and release claims before live evidence closeout.
**Depends on**: Phase 48
**Requirements**: PROOF-06, SEC-01, SEC-02
**Success Criteria** (what must be TRUE):
  1. Reviewer can inspect a v1.3 threat model covering public peer input, resource exhaustion, storage corruption, operator RPC controls, log and report redaction, and live evidence handling.
  2. Parity and release-readiness docs distinguish v1.3 proven public-mainnet sync evidence from deferred inbound serving, transaction relay, production-funds wallet, migration apply mode, packaging, hosted/public web surfaces, graphical app work, and unattended production-node claims.
  3. Live-mainnet evidence acceptance criteria are documented with repo-local commands and do not add public-network checks to `bash scripts/verify.sh`.
  4. Reviewers can trace v1.3 evidence requirements to roadmap phases without expanding the shipped support boundary.
**Plans**: 2 plans
Plans:
- [x] 49-01-PLAN.md — Reviewer-facing v1.3 threat model and release claim boundary.
- [x] 49-02-PLAN.md — Parity-root discoverability and deterministic release-boundary assertion.

### Phase 50: Public Mainnet Progress Evidence Closeout
**Goal**: Produce auditable opt-in public-mainnet evidence for header progress, block progress, restart/resume, or a diagnosed environmental blocker.
**Depends on**: Phase 49
**Requirements**: PROOF-03, PROOF-04, PROOF-05, SEC-03
**Success Criteria** (what must be TRUE):
  1. Reviewer can inspect a live smoke report with the first observed validated header-height increase, peer endpoint, source, timestamp, and before/after durable status.
  2. Reviewer can inspect a live smoke report with the first validated block connection beyond genesis or configured checkpoint, or an explicit diagnosis when block progress was not reached.
  3. Operator can interrupt and restart the same public-mainnet datadir and see durable before/after evidence that header, block, and runtime metadata progress resume coherently.
  4. UAT records successful public-mainnet header and block progress evidence or a diagnosed environment/network blocker with enough detail for the next operator action.
**Plans**: TBD

## Completed Milestones

<details>
<summary>✅ v1.2 Full Mainnet Network Syncing (Phases 35-41) - SHIPPED 2026-05-23</summary>

- [x] **Phase 35: Daemon Mainnet Sync Activation** - Explicit opt-in daemon mainnet sync activation and preflight.
- [x] **Phase 36: Mainnet Peer Discovery and Outbound Lifecycle** - DNS/manual peer resolution, bounded outbound peer lifecycle, rotation, and telemetry.
- [x] **Phase 37: Header-First Mainnet Sync Integration** - Durable validated header synchronization and restart recovery.
- [x] **Phase 38: Block Download, Connect, and Restart Recovery** - Bounded block download, validation, connection, reorg-aware state, and restart recovery.
- [x] **Phase 39: Operator Sync Observability and Control** - Truthful sync status, dashboard, metrics, logs, RPC surfaces, and pause/resume control.
- [x] **Phase 40: Live Mainnet Smoke, Docs, and Parity Closeout** - Opt-in live-mainnet smoke reporting and shipped-claim documentation.
- [x] **Phase 41: Security Analysis Audit and Follow-Up** - Security-analysis closeout with `threats_open: 0` and no new security phase required.

Detailed phase execution history is archived under
[milestones/v1.2-phases/](milestones/v1.2-phases/).

</details>

## Progress

**Execution Order:** Phase 42 -> 43 -> 44 -> 45 -> 46 -> 47 -> 48 -> 49 -> 50

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 42. Live Smoke Entry and Network Preflight | 1/1 | Complete | 2026-05-24 |
| 43. Outbound Peer Resilience | 1/1 | Complete | 2026-05-24 |
| 44. Peer Contribution Attribution | 1/1 | Complete    | 2026-05-25 |
| 45. Runtime Resource Bounds and Store Coordination | 1/1 | Complete | 2026-05-26 |
| 46. Durable Recovery and Invalid Data Handling | 1/1 | Complete | 2026-05-26 |
| 47. Operator Sync Truth Surfaces | 1/1 | Complete    | 2026-05-26 |
| 48. Support Evidence and Operator Runbooks | 1/1 | Complete    | 2026-05-27 |
| 49. Threat Model and Release Boundaries | 2/2 | Complete    | 2026-05-27 |
| 50. Public Mainnet Progress Evidence Closeout | 1/1 | Complete    | 2026-05-28 |

## Next Step

v1.3 phase work is complete. Run a milestone audit or milestone completion
workflow before archiving the milestone.
