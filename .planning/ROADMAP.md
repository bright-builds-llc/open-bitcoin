# Roadmap: Open Bitcoin

## Milestones

- ✅ **v1.0 Headless Parity** - 22 phase entries, including inserted 3.x and 7.x closure phases (shipped 2026-04-26). Archive: [v1.0-ROADMAP.md](milestones/v1.0-ROADMAP.md)
- ✅ **v1.1 Operator Runtime and Real-Network Sync** - Phases 13 through 34 (shipped 2026-04-30). Archive: [v1.1-ROADMAP.md](milestones/v1.1-ROADMAP.md)
- ✅ **v1.2 Full Mainnet Network Syncing** - Phases 35 through 41 (shipped 2026-05-23). Archive: [v1.2-ROADMAP.md](milestones/v1.2-ROADMAP.md)
- ✅ **v1.3 Public Mainnet Sync Proof and Node Hardening** - Phases 42 through 53 (shipped 2026-06-02). Archive: [v1.3-ROADMAP.md](milestones/v1.3-ROADMAP.md)
- 🚧 **v1.4 Mainnet IBD Convergence and Peer Compatibility** - Phases 54 through 59 (active).

## Current Focus

v1.4 turns v1.3's fresh diagnosed-blocker closeout into a stronger opt-in
outbound IBD claim. The milestone focuses on public peer compatibility,
validated header progress, validated block download/connect progress,
same-datadir restart/resume evidence, and truthful operator evidence.

Broad ecosystem research is intentionally skipped for this milestone. Phase
planning should use targeted Knots/protocol comparison and live-smoke evidence
instead of broad new-feature research.

The milestone does not include inbound serving, transaction relay,
production-funds wallet use, migration apply mode, packaging, hosted dashboard,
GUI work, unattended production-node operation, or adding public-network checks
to the default `bash scripts/verify.sh` gate.

## Phases

- [x] **Phase 54: Peer Compatibility Baseline and Diagnostic Harness** - Make handshake and early-protocol failures reproducible and comparable to the Knots baseline. (completed 2026-06-02)
- [x] **Phase 55: Outbound Handshake Compatibility Fixes** - Complete baseline-compatible handshakes with reachable peers and keep incompatible peers diagnosable. (completed 2026-06-03)
- [ ] **Phase 56: Header IBD Convergence** - Prove validated multi-batch header progress with fresh durable daemon status.
- [ ] **Phase 57: Block Download and Connect Progress** - Prove bounded block download and first validated block connection beyond the v1.4 target boundary.
- [ ] **Phase 58: Same-Datadir Restart and Resume Evidence** - Prove durable resume after observed live progress without duplicate connects.
- [ ] **Phase 59: Operator Evidence, Threat Model, and Release Boundaries** - Close v1.4 with coherent support evidence, docs, security analysis, and claim boundaries.

## Phase Details

### Phase 54: Peer Compatibility Baseline and Diagnostic Harness

**Goal**: Make v1.3 handshake and early-protocol failures reproducible, typed, and comparable against the pinned Knots baseline.
**Depends on**: Phase 53
**Requirements**: COMPAT-01, COMPAT-02, COMPAT-04
**Success Criteria** (what must be TRUE):
  1. Reviewer can inspect a baseline comparison for Open Bitcoin versus Knots outbound `version`, `verack`, `sendheaders`, `wtxidrelay`, `getheaders`, and `getdata` behavior.
  2. Operator can run a deterministic compatibility harness or scripted peer check that reproduces the failing step for handshake or early-protocol failures.
  3. Compatibility diagnostics distinguish version rejection, network mismatch, service-bit mismatch, message-order failure, timeout, peer disconnect, malformed payload, and local configuration failure.
  4. The diagnostic harness is hermetic by default and does not require public-network access in `bash scripts/verify.sh`.
**Plans**: 1/1 plans complete

### Phase 55: Outbound Handshake Compatibility Fixes

**Goal**: Make daemon sync complete baseline-compatible outbound handshakes with reachable peers while preserving existing protocol rejection safeguards.
**Depends on**: Phase 54
**Requirements**: COMPAT-03, COMPAT-05
**Success Criteria** (what must be TRUE):
  1. Daemon sync completes the outbound handshake with a reachable manual or DNS peer that accepts a baseline-compatible Knots outbound connection.
  2. Existing duplicate-version, malformed-message, and wrong-network rejections remain covered by deterministic tests.
  3. Incompatible peers are skipped or replaced with typed compatibility outcomes and no useful-progress credit.
  4. Durable state remains coherent when mixed compatible and incompatible peers are observed.
**Plans**: 1/1 plans complete

### Phase 56: Header IBD Convergence

**Goal**: Prove validated public-mainnet-like header progress through multi-batch sync and fresh daemon status.
**Depends on**: Phase 55
**Requirements**: HDR-01, HDR-02, HDR-03, HDR-04
**Success Criteria** (what must be TRUE):
  1. Operator can run an opt-in live-mainnet smoke command that records the first validated header-height increase with endpoint, source, timestamp, and fresh before/after daemon status.
  2. Daemon sync continues locator and `getheaders` rounds across multiple batches until a smoke target, tip estimate, timeout, or typed diagnosed blocker is reached.
  3. Header progress persists durably and remains visible through `openbitcoinsyncstatus` after daemon restart or status polling.
  4. Deterministic tests cover accepted multi-batch headers, rejected headers, and no-progress diagnosis without public-network access.
**Plans**: none yet.

### Phase 57: Block Download and Connect Progress

**Goal**: Prove bounded block download and first validated block connection for the scoped v1.4 IBD claim.
**Depends on**: Phase 56
**Requirements**: BLK-01, BLK-02, BLK-03, BLK-04
**Success Criteria** (what must be TRUE):
  1. Daemon sync requests, tracks, and bounds in-flight block downloads for selected validated headers within documented v1.4 resource limits.
  2. Opt-in live-smoke evidence records the first validated block connection beyond genesis or a configured checkpoint-adjacent target when reachable peers provide the data.
  3. If block progress is not reached, the live-smoke report records a typed diagnosis with peer endpoint, reason, and next operator action.
  4. Missing, `notfound`, malformed, invalid, duplicate, or disconnected block responses remain peer-attributed and do not advance active chainstate.
**Plans**: none yet.

### Phase 58: Same-Datadir Restart and Resume Evidence

**Goal**: Prove that the same public-mainnet datadir resumes from durable header or block progress after interruption.
**Depends on**: Phase 57
**Requirements**: RESUME-01, RESUME-02, RESUME-03
**Success Criteria** (what must be TRUE):
  1. Operator can interrupt and restart the same v1.4 public-mainnet datadir after observed progress and see sync resume without duplicate block connects.
  2. Live-smoke reporting captures before/after restart evidence for header height, block height, runtime phase, peer outcomes, and latest progress timestamp.
  3. Recovery guidance distinguishes peer incompatibility, public-network unreachability, invalid peer data, store corruption, store incompatibility, resource exhaustion, and cancellation.
  4. Deterministic restart/resume tests cover durable state transitions without public-network access.
**Plans**: none yet.

### Phase 59: Operator Evidence, Threat Model, and Release Boundaries

**Goal**: Close v1.4 with coherent operator evidence, support artifacts, docs, security analysis, and scoped release claims.
**Depends on**: Phase 58
**Requirements**: OBS-01, OBS-02, OBS-03, SEC-01, SEC-02, SEC-03
**Success Criteria** (what must be TRUE):
  1. Status, dashboard, metrics, logs, RPC-facing blockchain info, and live-smoke snapshots agree on header height, block height, peer compatibility state, progress signal, and latest error.
  2. Support evidence summarizes compatibility diagnostics, selected live-smoke reports, peer outcomes, status snapshots, metrics, logs, config sources, and store health without raw sensitive data.
  3. Operator docs include repo-local Cargo and Bazel commands for deterministic checks, manual-peer live smoke, restart/resume review, support evidence, and pass/fail interpretation.
  4. Reviewer-facing threat model, parity docs, and release-readiness docs preserve the v1.4 opt-in outbound IBD claim boundary and keep public-network checks outside `bash scripts/verify.sh`.
**Plans**: none yet.

## Completed Milestones

<details>
<summary>✅ v1.3 Public Mainnet Sync Proof and Node Hardening (Phases 42-53) - SHIPPED 2026-06-02</summary>

- [x] Phase 42: Live Smoke Entry and Network Preflight (1/1 plans) - completed 2026-05-24
- [x] Phase 43: Outbound Peer Resilience (1/1 plans) - completed 2026-05-24
- [x] Phase 44: Peer Contribution Attribution (1/1 plans) - completed 2026-05-25
- [x] Phase 45: Runtime Resource Bounds and Store Coordination (1/1 plans) - completed 2026-05-26
- [x] Phase 46: Durable Recovery and Invalid Data Handling (1/1 plans) - completed 2026-05-26
- [x] Phase 47: Operator Sync Truth Surfaces (1/1 plans) - completed 2026-05-26
- [x] Phase 48: Support Evidence and Operator Runbooks (1/1 plans) - completed 2026-05-27
- [x] Phase 49: Threat Model and Release Boundaries (2/2 plans) - completed 2026-05-27
- [x] Phase 50: Public Mainnet Progress Evidence Closeout (1/1 plans) - completed 2026-05-28
- [x] Phase 51: Live Smoke Fresh Status Integration (1/1 plans) - completed 2026-05-31
- [x] Phase 52: Operator Evidence Cleanup (1/1 plans) - completed 2026-06-01
- [x] Phase 53: Live Evidence Refresh (1/1 plans) - completed 2026-06-01

Detailed phase requirements, success criteria, and plan links are archived in
[milestones/v1.3-ROADMAP.md](milestones/v1.3-ROADMAP.md). Raw v1.3 phase
execution artifacts remain in [.planning/phases/](phases/) for parity and UAT
traceability.

</details>

## Progress

**Execution Order:** Phase 54 -> 55 -> 56 -> 57 -> 58 -> 59

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 54. Peer Compatibility Baseline and Diagnostic Harness | 1/1 | Complete    | 2026-06-02 |
| 55. Outbound Handshake Compatibility Fixes | 1/1 | Complete    | 2026-06-03 |
| 56. Header IBD Convergence | 0/0 | Not started | - |
| 57. Block Download and Connect Progress | 0/0 | Not started | - |
| 58. Same-Datadir Restart and Resume Evidence | 0/0 | Not started | - |
| 59. Operator Evidence, Threat Model, and Release Boundaries | 0/0 | Not started | - |

| Milestone | Phases | Plans | Status | Shipped |
| --- | ---: | ---: | --- | --- |
| v1.0 Headless Parity | 22 | 80 | Shipped | 2026-04-26 |
| v1.1 Operator Runtime and Real-Network Sync | 22 | 69 | Shipped | 2026-04-30 |
| v1.2 Full Mainnet Network Syncing | 7 | 13 | Shipped | 2026-05-23 |
| v1.3 Public Mainnet Sync Proof and Node Hardening | 12 | 13 | Shipped | 2026-06-02 |
| v1.4 Mainnet IBD Convergence and Peer Compatibility | 6 | 2 | Active | - |

## Next Step

Start Phase 56:

```bash
/gsd-discuss-phase 56
```
