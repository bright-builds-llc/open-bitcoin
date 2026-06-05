---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 58-2026-06-05T12-58-05
generated_at: 2026-06-05T12:58:05.451Z
---

# Phase 58: Same-Datadir Restart and Resume Evidence - Context

**Gathered:** 2026-06-05
**Status:** Ready for planning
**Mode:** Yolo

<domain>

## Phase Boundary

Phase 58 proves that an operator can interrupt and restart the same v1.4
public-mainnet datadir after observed durable header or block progress and see
Open Bitcoin resume from that durable state without duplicate block connects.
It builds on Phase 57's downloaded/connected block status, first-block
live-smoke evidence, and peer-attributed no-credit outcomes.

This phase owns same-datadir restart/resume evidence, restart-aware live-smoke
reporting, recovery diagnosis, deterministic durable resume tests, and
operator runbook guidance. It does not own support bundle allowlisting,
threat-model closeout, release-boundary copy, service-manager restart policy,
unattended production-node claims, inbound serving, transaction relay, wallet
production use, migration apply mode, packaging, hosted dashboards, or GUI
work.

</domain>

<decisions>

## Implementation Decisions

### Same-Datadir Restart Flow

- **D-01:** Add a script-managed two-session live-smoke restart flow as the
  operator-facing public UAT proof. The flow should use the same selected
  datadir, capture progress before restart, terminate the first daemon
  intentionally, relaunch the daemon, and capture resume evidence from fresh
  `openbitcoinsyncstatus` snapshots.
- **D-02:** Treat deterministic store-reopen tests as the mandatory regression
  guard for RESUME-01. They must prove durable headers, block bodies,
  chainstate, downloaded height/hash, and connected height/hash survive reopen
  and that already connected blocks are not requested or connected again.
- **D-03:** Do not require fresh post-restart public-network progress when
  peers stall after the restart. The restart claim is durable resume from the
  same datadir plus a typed diagnosis of any post-restart blocker. Fresh
  progress after restart is stronger evidence when available, not the only
  acceptable evidence path.
- **D-04:** Keep service-manager and unattended restart-policy behavior out of
  scope. Phase 58 may relaunch a daemon process for explicit smoke evidence,
  but it must not imply launchd/systemd supervision or production-node
  operation.

### Restart Evidence Report Schema

- **D-05:** Add a compact schema v2 result object named
  `result.restartResumeEvidence` rather than raw top-level restart attempts.
  The object should prove the restart boundary and same-datadir resume using
  allowlist-friendly summary fields.
- **D-06:** `restartResumeEvidence` should include same-datadir confirmation,
  restart status, before-restart and after-restart durable heights and hashes,
  runtime phase/lifecycle, latest successful progress timestamp, peer outcome
  summary, duplicate-connect verdict, and optional post-restart progress delta.
- **D-07:** Preserve Phase 57 `firstHeaderProgress` and `firstBlockProgress`
  as local report evidence, but do not use them alone as restart proof because
  they cannot distinguish same-process progress from post-relaunch resume.
- **D-08:** Keep raw daemon stdout/stderr tails, raw status snapshots, raw
  options, raw endpoint tables, and high-volume peer rows out of the compact
  restart evidence object. Phase 59 can decide support-bundle allowlisting.

### Recovery Diagnosis Taxonomy

- **D-09:** Prefer a layered recovery diagnosis model for Phase 58 evidence.
  The user-facing category should be one of:
  `peer_incompatibility`, `public_network_unreachable`, `invalid_peer_data`,
  `store_corruption`, `store_incompatibility`, `resource_exhaustion`, or
  `intentional_cancellation`.
- **D-10:** Preserve underlying causes alongside the Phase 58 category when
  available, including existing live-smoke `NoProgressCause`, durable
  `PeerFailureReason`, storage recovery action, and last-error detail.
- **D-11:** Storage health outranks peer retry guidance. Store corruption or
  store incompatibility should classify before peer incompatibility,
  public-network unreachability, invalid peer data, or timeout-style guidance.
- **D-12:** Operator guidance should distinguish cancellation from failure:
  intentional interruption used for restart evidence is part of the flow, while
  cancellation before enough evidence is captured remains a typed
  `intentional_cancellation` diagnosis.

### Deterministic Test Strategy

- **D-13:** Prioritize `DurableSyncRuntime` two-pass same-datadir fixtures that
  use real Fjall reopen and `ScriptedTransport`. These tests should cover
  header-only resume, partial downloaded block resume, connected block resume,
  no duplicate `getdata` for already connected blocks, and best-chain block
  reconciliation after reopen.
- **D-14:** Add mocked live-smoke fixture tests for restart-report semantics:
  before/restart/after snapshots, same datadir, runtime phase, peer summaries,
  latest progress timestamp, duplicate-connect verdict, and recovery diagnosis.
- **D-15:** Add a narrow recovery diagnosis matrix for RESUME-03. Avoid a
  broad process-level local peer harness unless review evidence shows that the
  existing Rust and script fixtures cannot prove the phase claim.
- **D-16:** Public-network live-smoke commands remain opt-in UAT evidence and
  must not be added to `bash scripts/verify.sh`.

### Documentation and UAT Boundaries

- **D-17:** Update operator docs with copy-pasteable repo-local Cargo and Bazel
  commands for same-datadir restart/resume review, status checks, and pass/fail
  interpretation.
- **D-18:** Document that Phase 58 evidence proves explicit opt-in restart and
  resume review only. It does not claim unattended production operation,
  packaged-service restart policy, inbound serving, transaction relay, or
  broad production-node readiness.

### the agent's Discretion

- The planner may choose the smallest robust internal representation for
  `restartResumeEvidence` and recovery diagnosis as long as the externally
  observable schema remains additive, typed, and deterministic to test.
- The planner may split work across Rust runtime tests, live-smoke TypeScript
  changes, shell fixtures, docs, and parity evidence according to existing
  module boundaries.
- The executor may reuse existing restart tests where they already satisfy a
  requirement, but summaries and verification must make the evidence explicit
  for Phase 58.

</decisions>

<canonical_refs>

## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase Scope

- `.planning/ROADMAP.md` - Phase 58 goal, requirements, and success criteria.
- `.planning/REQUIREMENTS.md` - RESUME-01 through RESUME-03 and v1.4 scope
  exclusions.
- `.planning/PROJECT.md` - Open Bitcoin parity, functional-core, and
  production-claim boundaries.
- `.planning/STATE.md` - Phase 57 completion state and prior milestone
  decisions.

### Prior Phase Evidence

- `.planning/phases/57-block-download-and-connect-progress/57-CONTEXT.md` -
  Phase 57 block progress decisions and deferred Phase 58 boundary.
- `.planning/phases/57-block-download-and-connect-progress/57-04-SUMMARY.md` -
  Completed first-block live-smoke evidence and block-specific diagnoses.
- `.planning/phases/56-header-ibd-convergence/56-CONTEXT.md` - Header
  convergence and first-header-progress decisions Phase 58 builds on.
- `.planning/phases/56-header-ibd-convergence/56-01-SUMMARY.md` - Completed
  durable header progress and fresh status evidence.
- `.planning/phases/51-live-smoke-fresh-status-integration/51-01-SUMMARY.md` -
  Fresh daemon `openbitcoinsyncstatus` live-smoke polling behavior.

### Implementation Surfaces

- `packages/open-bitcoin-node/src/sync.rs` - `DurableSyncRuntime` reopen,
  sync loop, peer outcomes, progress persistence, and durable status writes.
- `packages/open-bitcoin-node/src/sync/tests.rs` - Existing deterministic
  restart, block reconnect, status, and peer-failure fixtures.
- `packages/open-bitcoin-node/src/sync/types/projection.rs` - Durable status
  projection, phase naming, and storage health message mapping.
- `packages/open-bitcoin-node/src/sync/types/summary.rs` - Summary projection,
  health signals, logs, metrics, and recovery output.
- `packages/open-bitcoin-node/src/storage.rs` - Storage recovery actions,
  schema errors, recovery markers, and runtime metadata.
- `packages/open-bitcoin-node/src/status.rs` - Shared operator sync status,
  lifecycle, progress signal, resource pressure, and durable sync state.
- `packages/open-bitcoin-rpc/src/context.rs` - `openbitcoinsyncstatus`,
  pause, and resume durable control behavior.
- `packages/open-bitcoin-cli/src/operator/status/sync_state.rs` - Operator
  status projection from durable daemon sync metadata.
- `scripts/run-live-mainnet-smoke.ts` - Opt-in live-smoke process orchestration,
  status polling, report schema, Markdown rendering, and no-progress
  classification.
- `scripts/test-run-live-mainnet-smoke.sh` - Deterministic live-smoke fixture
  regression suite.
- `docs/operator/runtime-guide.md` - Operator-facing runtime, live-smoke,
  durable recovery, and same-datadir command guidance.
- `docs/parity/catalog/p2p.md` - P2P parity catalog and v1.4 outbound IBD
  claim boundaries.

### Baseline Anchors

- `packages/bitcoin-knots/test/functional/feature_init.py` - Baseline-style
  restart/interruption functional testing posture.
- `packages/bitcoin-knots/doc/man/bitcoind.1` - Daemon option and datadir
  operator behavior anchor.
- `packages/bitcoin-knots/src/init.cpp` - Daemon startup and datadir handling
  anchor.
- `packages/bitcoin-knots/src/net_processing.cpp` - Peer sync, invalid data,
  and block response attribution anchor.
- `packages/bitcoin-knots/src/node/blockstorage.cpp` - Block storage and
  restart/reindex behavior anchor.

</canonical_refs>

<code_context>

## Existing Code Insights

### Reusable Assets

- `DurableSyncRuntime::open` already reloads chainstate snapshots and header
  stores from `FjallNodeStore`, then exposes `snapshot_summary`.
- Existing sync tests already cover header seeding on restart, persisted block
  reconnect before re-request, partial downloaded block status after reopen,
  and reorg to the best available branch after restart.
- `SyncStatus` already separates header height, downloaded block height,
  connected block height, optional downloaded/connected hashes, lifecycle,
  phase, last progress timestamp, recovery action, and resource pressure.
- `scripts/run-live-mainnet-smoke.ts` already captures daemon status snapshots,
  first header progress, first block progress, final status, peer endpoint
  outcomes, cancellation, and typed no-progress causes.
- `scripts/test-run-live-mainnet-smoke.sh` already has mocked status binaries
  and fixtures for connected block progress, downloaded-only progress,
  header-only progress, peer failure mappings, preflight failure, timeout, and
  cancellation.

### Established Patterns

- Public-mainnet checks are opt-in UAT evidence and remain outside
  `bash scripts/verify.sh`.
- Report/schema changes should be additive and tolerate older or unavailable
  evidence.
- Storage recovery metadata takes precedence over peer retry guidance in
  operator-facing status.
- Raw live-smoke reports are local artifacts; support-facing summaries should
  be allowlisted and redacted.
- Tests for non-trivial pure/domain behavior use explicit Arrange, Act, Assert
  comments.

### Integration Points

- Add restart/resume evidence derivation beside existing first-header and
  first-block progress derivation in `scripts/run-live-mainnet-smoke.ts`.
- Extend deterministic live-smoke fixtures to simulate first daemon progress,
  restart, second daemon status polling, and compact restart evidence output.
- Add or tighten `DurableSyncRuntime` restart tests around duplicate block
  connect/request prevention and durable post-reopen summary projection.
- Update docs/parity text to describe the explicit same-datadir review claim
  without broadening v1.4 release scope.

</code_context>

<specifics>

## Specific Ideas

- Add a live-smoke option such as `--restart-after-progress` or equivalent
  that performs the two-session restart review only when explicitly requested.
- Record compact restart evidence under `result.restartResumeEvidence` with
  before/restart/after summaries rather than embedding all raw snapshots.
- Include a duplicate-connect verdict that is derived from deterministic status
  or peer failure evidence and is conservative when unavailable.
- Add a recovery diagnosis helper with storage-first precedence and unit or
  fixture coverage for all seven RESUME-03 categories.
- In UAT docs, provide both Cargo and Bazel commands for `sync status` against
  the same datadir before and after restart.

</specifics>

<deferred>

## Deferred Ideas

- Support bundle allowlisting for `result.restartResumeEvidence` remains
  Phase 59 unless Phase 58 needs a minimal preparatory hook for schema
  compatibility.
- Threat-model updates, release-boundary copy, and final operator evidence
  closeout remain Phase 59.
- Service-manager restart policy, launchd/systemd supervision, unattended
  production-node operation, inbound serving, transaction relay, production
  wallet use, migration apply mode, packaging, hosted dashboard, and GUI work
  remain out of scope for v1.4.

</deferred>

---

*Phase: 58-same-datadir-restart-and-resume-evidence*
*Context gathered: 2026-06-05*
