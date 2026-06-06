---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 62-2026-06-06T19-46-48
generated_at: 2026-06-06T19:46:48.293Z
---

# Phase 62: Long-Run Sync Truth Surfaces - Context

**Gathered:** 2026-06-06
**Status:** Ready for planning
**Mode:** Yolo

<domain>

## Phase Boundary

Phase 62 makes the unattended sync truth fields agree across operator-facing
status, dashboard, RPC sync warnings, metrics projections, structured logs, and
opt-in live-smoke snapshots. It owns cross-surface consistency for lifecycle,
phase, configured targets, attempt counters, progress evidence, stop reasons,
peer health, recovery category, resource pressure, and downloaded/connected
block evidence.

This phase does not add launchd/systemd supervision lifecycle behavior,
service-supervised restart proof, v1.5 support-bundle expansion, compatibility
harness wrapping, default public-network verification, or a broad production
node claim. Those remain Phase 63 through Phase 67 or future milestone work.

</domain>

<decisions>

## Implementation Decisions

### Shared Truth Contract

- **D-01:** Treat the shared status snapshot and durable sync state as the
  canonical truth source for operator surfaces. Renderers, RPC warnings,
  metrics projection, structured logs, and live-smoke parsing should consume
  already-typed fields instead of independently parsing text or re-inferring
  lifecycle, progress, recovery, resource pressure, or peer health.
- **D-02:** The consistent Phase 62 field set is lifecycle, phase, configured
  targets, attempt counters, latest progress signal, last successful progress
  timestamp, latest stop reason, latest error, recovery category, recovery
  action, resource pressure, peer health, header height, downloaded block height
  and hash, connected block height and hash, and bounded message/header/block
  counters.
- **D-03:** Unavailable data must stay explicit through `FieldAvailability` or
  equivalent report-level unavailable/null summaries with reasons. Do not hide
  missing fields behind zeroes, empty strings, or renderer-local "ok" summaries.

### Bounded Metrics And Structured Logs

- **D-04:** Metrics and structured logs should keep the same progress vocabulary
  as status: `header_height`, `downloaded_block_height`,
  `connected_block_height`, peer count, progress signal, recovery category,
  stop reason, and bounded cycle summary counters where those facts exist.
- **D-05:** Long-run evidence must remain bounded by existing retention policies
  or explicit compact cycle summaries. Phase 62 should not add unbounded arrays
  of snapshots, peer outcomes, log lines, metrics samples, or raw live-smoke
  report material.
- **D-06:** Structured log records should expose compact machine-stable cycle
  facts that can be compared with status and live-smoke snapshots. Human message
  text may remain, but deterministic checks should assert the stable labels or
  fields that downstream operators rely on.

### Live-Smoke Snapshot Compactness

- **D-07:** Opt-in live-smoke reports should use the same field names and
  semantics as status for final status and bounded snapshot tables. The final
  report should preserve enough diagnosis evidence to compare before/after
  progress without embedding raw daemon tails, full endpoint tables, or
  unbounded report history.
- **D-08:** Live-smoke markdown and JSON should let an operator distinguish
  progress, waiting, retry, stop, and recovery states the same way they appear
  in status/dashboard/RPC/logs. Where TypeScript report casing differs for JSON
  ergonomics, keep a single mapping layer and deterministic fixture coverage.
- **D-09:** Public-network live-smoke and long-run checks remain opt-in UAT
  evidence. Default verification may use deterministic fixtures and generated
  sample reports, but must not make public network access part of
  `bash scripts/verify.sh`.

### Verification And Documentation

- **D-10:** Add deterministic cross-surface checks that fail when a Phase 62
  truth field exists in one surface but is missing or renamed in another.
  Prefer focused Rust tests for shared projections/renderers and Bun fixture
  checks for scripts/docs when those surfaces change.
- **D-11:** Refresh operator and architecture docs only where Phase 62 changes
  the truth contract or review workflow. Docs should keep copy-pasteable
  repo-local Cargo and Bazel commands for operator workflows and continue to
  separate deterministic verification from opt-in public-network UAT.
- **D-12:** Preserve Phase 61 recovery labels and resource-pressure fields as
  stable inputs. Phase 62 should extend cross-surface agreement around them, not
  rename the taxonomy or broaden recovery semantics.

### the agent's Discretion

- The planner may introduce a small pure projection helper or checker data set
  if it removes duplication across status, dashboard, RPC, metrics, logs, and
  live-smoke fixtures.
- The executor may keep changes in existing modules when that is the least risky
  path. If new first-party Rust files are added under `packages/open-bitcoin-*`,
  update parity breadcrumbs before committing.
- The planner may split work into several small plans by surface cluster, but
  each plan should prove agreement against the same Phase 62 field contract.

</decisions>

<canonical_refs>

## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase Scope

- `.planning/ROADMAP.md` - Phase 62 goal, success criteria, dependency on Phase
  61, and deferred Phase 63 through Phase 67 boundaries.
- `.planning/REQUIREMENTS.md` - OBS-01, OBS-02, v1.5 out-of-scope boundaries,
  and the explicit default-verification public-network exclusion.
- `.planning/PROJECT.md` - v1.5 milestone goal and release-boundary constraints.
- `.planning/STATE.md` - Current milestone state and prior decisions affecting
  deterministic verification, opt-in live evidence, and operator truth surfaces.

### Prior Phase Decisions And Evidence

- `.planning/phases/61-resource-bounds-and-recovery-taxonomy/61-CONTEXT.md` -
  Shared recovery category, resource pressure, support evidence, and
  deterministic verification decisions that Phase 62 must carry forward.
- `.planning/phases/61-resource-bounds-and-recovery-taxonomy/61-VERIFICATION.md`
  - Passed evidence for typed recovery labels, resource bounds, renderers, RPC
  warnings, structured logs, live-smoke categories, and default verification.
- `.planning/phases/61-resource-bounds-and-recovery-taxonomy/61-06-SUMMARY.md`
  - Documentation/checker pattern and Phase 62 readiness notes.
- `.planning/phases/60-unattended-sync-loop-control/60-CONTEXT.md` - Loop
  activation, stop-reason persistence, pause/resume/shutdown behavior, and
  deterministic verification posture.
- `.planning/phases/59-operator-evidence-threat-model-and-release-boundaries/59-CONTEXT.md`
  - Cross-surface truth, support evidence allowlist, and release-boundary
  decisions.
- `.planning/phases/57-block-download-and-connect-progress/57-CONTEXT.md` -
  Downloaded/connected block evidence and no-credit peer outcome decisions.
- `.planning/phases/56-header-ibd-convergence/56-CONTEXT.md` - Header progress,
  no-progress diagnosis, and target-height stop decisions.

### Implementation Surfaces

- `packages/open-bitcoin-node/src/status.rs` - Shared `SyncStatus`,
  `SyncProgress`, `SyncLifecycleState`, `SyncResourcePressure`,
  `FieldAvailability`, peer telemetry, metrics, and durable sync status
  contracts.
- `packages/open-bitcoin-node/src/status/recovery.rs` - Stable
  `SyncRecoveryCategory` labels introduced in Phase 61.
- `packages/open-bitcoin-node/src/metrics.rs` - Bounded metrics retention policy,
  metric kind labels, and status samples.
- `packages/open-bitcoin-node/src/sync/runtime_state.rs` - Durable status
  projection, storage-first recovery category precedence, resource pressure,
  metrics persistence, and structured log writing.
- `packages/open-bitcoin-node/src/sync/types/summary.rs` - `SyncRunSummary`
  status, metric, peer, progress, stop-reason, and structured-log projections.
- `packages/open-bitcoin-node/src/sync/types/projection.rs` - Sync phase names
  and storage health projection helpers.
- `packages/open-bitcoin-node/src/sync/tests.rs` - Scripted deterministic
  transport/resolver fixtures for long-run sync, resource, and recovery tests.
- `packages/open-bitcoin-cli/src/operator/status/render.rs` - Human and JSON
  operator status rendering for progress, pressure, recovery, metrics, and
  peer evidence.
- `packages/open-bitcoin-cli/src/operator/dashboard/model.rs` - Dashboard model
  rows for sync lifecycle, progress, pressure, recovery category, peers, and
  metrics.
- `packages/open-bitcoin-rpc/src/dispatch/node.rs` - RPC-facing sync status and
  durable warning integration.
- `packages/open-bitcoin-cli/src/operator/support/live_smoke.rs` - Allowlisted
  live-smoke final-status and resource-pressure summary extraction.
- `packages/open-bitcoin-cli/src/operator/support/render.rs` - Support markdown
  rendering of compact recovery/resource evidence.
- `scripts/run-live-mainnet-smoke.ts` - Opt-in live-smoke JSON/Markdown report,
  snapshot capture, recovery category parsing, and compact final status.
- `scripts/test-run-live-mainnet-smoke.sh` - Deterministic live-smoke fixture
  checks.
- `scripts/check-phase61-resource-recovery-boundaries.ts` - Prior phase checker
  pattern for exact docs/default-verification boundary assertions.

### Operator Docs And Parity Roots

- `docs/operator/runtime-guide.md` - Runtime sync activation, status, metrics,
  logs, live-smoke, and opt-in UAT commands.
- `docs/architecture/status-snapshot.md` - Shared status snapshot contract and
  cross-surface truth expectations.
- `docs/architecture/operator-observability.md` - Metrics/log retention,
  support evidence, and observability boundaries.
- `docs/architecture/config-precedence.md` - Config source ownership and
  credential reporting boundaries.
- `docs/parity/release-readiness.md` - Release claim boundaries and deferred
  public-network/production surfaces.
- `docs/parity/index.json` - Machine-readable parity roots and evidence links.

### Baseline Anchors

- `packages/bitcoin-knots/src/init.cpp` - Startup, shutdown, and datadir
  lifecycle anchor.
- `packages/bitcoin-knots/src/net.cpp` - Peer connection and retry lifecycle
  anchor.
- `packages/bitcoin-knots/src/net_processing.cpp` - Peer sync, invalid data,
  and no-credit progress attribution anchor.
- `packages/bitcoin-knots/src/headerssync.cpp` - Header sync progress and stop
  behavior anchor.

</canonical_refs>

<code_context>

## Existing Code Insights

### Reusable Assets

- `SyncStatus` already carries lifecycle, phase, progress signal, lag, last
  successful progress, last error, recovery category, recovery action, and
  `SyncResourcePressure`.
- `SyncProgress` already carries header height, block height, downloaded block
  height/hash, connected block height/hash, progress ratio, and processed
  message/header/block counters.
- `SyncRunSummary::metric_samples()` already projects bounded metric samples for
  header height, downloaded block height, connected block height, sync height,
  and peer count.
- `SyncRunSummary::structured_log_records()` already emits compact sync summary
  records with progress counters, heights, progress signal, last progress, and
  recovery category.
- Status and dashboard renderers already surface recovery category, pressure,
  progress, peers, and metrics from the shared snapshot.
- `scripts/run-live-mainnet-smoke.ts` already parses status snapshots, carries
  recovery category and resource pressure, and writes compact final-status and
  snapshot markdown sections.

### Established Patterns

- Sync decisions stay in `open-bitcoin-node`; CLI, RPC, dashboard, support, and
  script layers should project typed facts rather than duplicate domain logic.
- Metrics, logs, support evidence, and live-smoke reports are bounded by
  retention, allowlists, or compact summaries.
- Public-network live review stays opt-in UAT and outside `bash scripts/verify.sh`.
- Operator docs use repo-local Cargo and Bazel commands rather than relying only
  on an installed alias.
- Phase-specific Bun checkers are acceptable when they enforce docs/script
  boundary contracts deterministically.

### Integration Points

- Add any shared truth-field helper close to `status.rs`, metrics, or sync
  summary projection so all consumers can reuse it without depending on CLI or
  script code.
- Renderer and RPC tests should assert exact field agreement with the shared
  status snapshot instead of just checking that some text is present.
- Live-smoke fixture tests should validate both JSON and Markdown compact
  snapshots against the Phase 62 field set while keeping public-network runs
  opt-in.
- Documentation updates should point operators to status, dashboard, RPC,
  metrics/logs, and live-smoke review commands as alternate views of the same
  bounded sync facts.

</code_context>

<specifics>

## Specific Ideas

No additional user-specific requests beyond the v1.5 milestone prompt. Use the
standard Open Bitcoin posture: opt-in, bounded, auditable, deterministic by
default, and explicit about deferred production-node scope.

</specifics>

<deferred>

## Deferred Ideas

- Phase 63 owns launchd/systemd service supervision lifecycle behavior.
- Phase 64 owns service-supervised restart and same-datadir resume evidence.
- Phase 65 owns v1.5 support-bundle collection and operator review docs.
- Phase 66 owns the compatibility harness operator wrapper.
- Phase 67 owns v1.5 release-boundary and deterministic verification closeout.
- Production-node, inbound-serving, relay, production-funds wallet, destructive
  migration apply, hosted dashboard, GUI, packaging/distribution, and Windows
  service claims remain future milestones.

</deferred>

---

*Phase: 62-long-run-sync-truth-surfaces*
*Context gathered: 2026-06-06*
