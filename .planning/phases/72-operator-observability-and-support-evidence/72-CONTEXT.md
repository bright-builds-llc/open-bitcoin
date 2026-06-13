---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 72-2026-06-13T16-25-04
generated_at: 2026-06-13T16:25:09.129Z
---

# Phase 72: Operator Observability and Support Evidence - Context

**Gathered:** 2026-06-13
**Status:** Ready for planning
**Mode:** Yolo

<domain>

## Phase Boundary

Phase 72 aligns every operator-facing evidence surface around one full-sync
truth contract for the explicit opt-in `open-bitcoind` mainnet sync-to-tip
workflow. Operators should be able to inspect CLI status, dashboard status, RPC
status, bounded metrics, structured logs, live-smoke reports, and redacted
support bundles and see the same connected active-chain progress, best-known
tip freshness, stay-current state, no-progress or reorg diagnosis, peer health,
resource pressure, restart/resume evidence, and next action.

This phase owns cross-surface agreement and redacted support evidence for
OBS-01 through OBS-04. It does not own new consensus validation behavior, new
peer policy, default public-network verification, opt-in UAT command breadth,
release-boundary closeout, inbound serving, address relay, block serving,
transaction relay, compact block relay, production-wallet claims, migration
apply mode, signed packaging, Windows service support, GUI work, hosted
dashboards, or broad production-node claims.

</domain>

<decisions>

## Implementation Decisions

### Shared Truth Contract

- **D-01:** Treat `OpenBitcoinStatusSnapshot` and its nested `SyncStatus` as
  the canonical full-sync truth contract. CLI status, dashboard, RPC
  Open-Bitcoin sync status, metrics/log projections, live-smoke summaries, and
  support bundles should consume or summarize this contract instead of
  reclassifying sync state locally.
- **D-02:** Preserve the Phase 68 progress distinction everywhere: header
  height, downloaded block height, connected block height, validated active-chain
  height, validated active-chain hash, and validated active-chain work are
  separate facts. Headers-only or downloaded-only progress must not be rendered
  as sync-to-tip or current-at-tip proof.
- **D-03:** Carry forward Phase 69 stay-current semantics. `current_at_best_known_tip`
  requires fresh best-known tip evidence and connected active-chain
  height/hash/work matching that best-known validated tip; stale-tip,
  recovering, and no-progress states stay distinct.
- **D-04:** Carry forward Phase 70 and Phase 71 bounded diagnosis fields:
  latest reorg evidence, reconcile progress, no-progress diagnosis, no-progress
  next action, resource pressure, storage/resource recovery category, and
  restart/resume evidence. These fields should appear as shared machine labels
  or explicit `Unavailable` reasons, not renderer-specific prose.

### Cross-Surface Alignment

- **D-05:** Add missing Phase 69-71 fields to human CLI status and dashboard
  projections where they are not already rendered, especially best-known tip,
  stay-current state/action, latest reorg, reconcile progress, no-progress
  diagnosis/action, resource pressure, and validated active-chain work.
- **D-06:** RPC Open-Bitcoin sync status should expose the same durable sync
  state contract when durable metadata is available. Baseline-compatible RPC
  methods such as `getblockchaininfo` may remain scoped to their existing
  Knots-compatible response shape, but Open Bitcoin-specific RPC status must not
  drop durable Phase 68-71 evidence.
- **D-07:** Metrics remain bounded numeric samples and structured logs remain
  compact records, but both should include enough shared labels and progress
  dimensions to correlate with status and support evidence: connected progress,
  tip/stay-current state, recovery/no-progress/reorg labels, peer contribution,
  resource pressure, and latest stop reason where appropriate.
- **D-08:** Add a deterministic cross-surface comparison check. It should verify
  that shared status JSON, CLI human rendering, dashboard projection, RPC
  durable status, live-smoke compact summary, structured-log summary, metrics
  projection, and support evidence agree on the core full-sync truth fields or
  preserve the same unavailable reasons.

### Redacted Support Evidence

- **D-09:** Support evidence should keep an allowlisted, redacted shape. It may
  include the shared status snapshot and compact live-smoke summaries, but must
  not embed raw daemon logs, raw peer transcripts, RPC cookies/passwords, config
  secrets, wallet material, raw live-smoke reports, or unbounded endpoint tables.
- **D-10:** Support evidence should include initial and final tip evidence,
  connected active-chain height/hash/work, restart/resume checkpoints,
  stay-current window, peer contribution summary, no-progress or reorg events,
  resource pressure, recovery category/action, and a final verdict.
- **D-11:** The final verdict should be typed and evidence-derived. Prefer a
  small enum such as `sync_to_tip_proven`, `stay_current_proven`,
  `diagnosed_blocker`, and `inconclusive` over freeform support text. The
  verdict must explain which evidence fields justify it.
- **D-12:** Live-smoke report ingestion into support bundles should stay summary
  only. Expand the allowlist for schema v2 fields where needed so support
  evidence can summarize Phase 69-71 fields without copying the raw report.

### Operator Guidance And Scope

- **D-13:** Operator guidance should explain what the evidence proves:
  sync-to-tip, stay-current behavior, diagnosed blocker, restart/resume safety,
  storage/resource pressure, or deferred production-node scope. Avoid broad
  production-node, inbound-serving, relay, production-wallet, migration-apply,
  signed-packaging, GUI, hosted-dashboard, or drop-in readiness language.
- **D-14:** Default verification remains deterministic, public-network-free,
  service-manager-free, timing-stable, and short-running. Public-mainnet
  full-sync and stay-current evidence remains opt-in UAT and should not be
  wired into `bash scripts/verify.sh`.
- **D-15:** When operator docs mention commands, use repo-local Cargo and Bazel
  forms for UAT guidance. Phase 72 may refine evidence interpretation now, while
  Phase 73 owns the broader opt-in UAT command matrix.

### Verification Posture

- **D-16:** Add focused unit and fixture tests for every changed renderer or
  evidence adapter. Tests should assert one concern per test with explicit
  Arrange/Act/Assert comments when setup is non-trivial.
- **D-17:** Add a Phase 72 deterministic checker in the existing Bun checker
  style and wire it into `bash scripts/verify.sh` after the Phase 71 checker.
  The checker should cover plan artifacts, source/test anchors, docs wording,
  support allowlist fields, cross-surface agreement fields, and default
  verification boundaries.
- **D-18:** New first-party Rust source or test files under
  `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests` must receive
  parity breadcrumb coverage through `docs/parity/source-breadcrumbs.json` and
  `scripts/check-parity-breadcrumbs.ts`.

### the agent's Discretion

- The planner may split work across shared status/rendering alignment, support
  evidence schema/verdicts, live-smoke/log/metric projections, cross-surface
  comparison tests, and docs/checker closeout.
- The executor may add small pure projection helpers when they prevent
  renderer-local divergence and keep illegal evidence verdict states
  unrepresentable.
- The executor may keep support evidence compact and additive. Existing JSON
  consumers should continue to decode older bundles through defaults or
  unavailable reasons.

</decisions>

<canonical_refs>

## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase Scope

- `.planning/ROADMAP.md` - Phase 72 goal, dependency on Phase 71, success
  criteria, and deferred Phase 73 through Phase 74 boundaries.
- `.planning/REQUIREMENTS.md` - OBS-01 through OBS-04, VER downstream
  boundaries, and v1.6 out-of-scope table.
- `.planning/PROJECT.md` - v1.6 milestone goal, pinned Knots baseline,
  functional-core boundary, and production-claim limits.
- `.planning/STATE.md` - Current milestone state and Phase 72 readiness.
- `AGENTS.md` - Repo-local GSD workflow, Rust, parity breadcrumb, UAT command,
  generated artifact, and verification requirements.
- `AGENTS.bright-builds.md` - Bright Builds workflow and standards routing.
- `standards-overrides.md` - Current local standards override registry.

### Prior Phase Decisions And Evidence

- `.planning/phases/62-long-run-sync-truth-surfaces/62-CONTEXT.md` - Shared
  long-run truth fields across status, metrics, logs, live-smoke, and support
  evidence.
- `.planning/phases/65-support-bundle-and-operator-review-docs/65-CONTEXT.md` -
  Redacted support bundle boundaries and repo-local operator command guidance.
- `.planning/phases/67-release-boundaries-and-deterministic-verification/67-CONTEXT.md`
  - Release-boundary wording and deterministic checker posture.
- `.planning/phases/68-full-active-chain-validation-and-durable-persistence/68-CONTEXT.md`
  - Validated active-chain progress credit and durable persistence contract.
- `.planning/phases/69-tip-tracking-and-stay-current-operation/69-CONTEXT.md` -
  Best-known tip evidence, stay-current states, and shared status integration.
- `.planning/phases/70-reorg-peer-rotation-and-no-progress-recovery/70-CONTEXT.md`
  - Bounded reorg evidence, no-progress diagnosis, peer rotation, and next
  actions.
- `.planning/phases/71-resource-bounds-and-durable-restart-resume/71-CONTEXT.md`
  - Resource pressure, same-datadir restart/resume, storage pressure recovery,
  and deterministic long-chain proof.
- `.planning/phases/71-resource-bounds-and-durable-restart-resume/71-VERIFICATION.md`
  - Passed Phase 71 evidence and residual Phase 72 cross-surface risks.

### Implementation Surfaces

- `packages/open-bitcoin-node/src/status.rs` - `OpenBitcoinStatusSnapshot`,
  `SyncStatus`, best-known tip, stay-current, no-progress, reorg, reconcile,
  resource pressure, restart/resume, peer, metrics, logs, and build provenance
  contracts.
- `packages/open-bitcoin-node/src/status/recovery.rs` - Stable recovery
  category labels.
- `packages/open-bitcoin-node/src/sync/types/summary.rs` - Runtime summary to
  status, metrics, structured logs, peer status, stop reason, recovery, and
  resource pressure projection.
- `packages/open-bitcoin-node/src/sync/runtime_state.rs` - Durable status
  projection, metadata persistence, best-tip/stay-current/no-progress evidence,
  and recovery precedence.
- `packages/open-bitcoin-node/src/sync/progress.rs` - Peer progress,
  no-progress diagnosis, retry/backoff, and bounded next-action helpers.
- `packages/open-bitcoin-node/src/metrics.rs` - Bounded metric kinds and
  retention contract.
- `packages/open-bitcoin-node/src/logging.rs` and
  `packages/open-bitcoin-node/src/logging/` - Structured log records, retention,
  pruning, and writer behavior.
- `packages/open-bitcoin-cli/src/operator/status/render.rs` - Human and JSON CLI
  status rendering.
- `packages/open-bitcoin-cli/src/operator/status/sync_state.rs` - RPC and
  durable sync state bridge used by operator status.
- `packages/open-bitcoin-cli/src/operator/dashboard/model.rs` - Dashboard
  projection from the shared status snapshot.
- `packages/open-bitcoin-cli/src/operator/runtime/support.rs` - Runtime support
  text for sync status, pause/resume, and resource pressure.
- `packages/open-bitcoin-cli/src/operator/support.rs` - Support bundle command,
  evidence bundle structure, redaction metadata, and store-health collection.
- `packages/open-bitcoin-cli/src/operator/support/live_smoke.rs` - Allowlisted
  live-smoke summary projection used by support bundles.
- `packages/open-bitcoin-cli/src/operator/support/render.rs` - Markdown and JSON
  support bundle rendering.
- `packages/open-bitcoin-cli/src/operator/support/tests.rs` or adjacent support
  test modules - Support bundle redaction and summary regression coverage.
- `packages/open-bitcoin-rpc/src/context.rs` - RPC context and durable metadata
  fallback.
- `packages/open-bitcoin-rpc/src/method/node.rs` - Baseline and Open
  Bitcoin-specific node/sync RPC response shapes.
- `packages/open-bitcoin-rpc/src/dispatch/node.rs` - RPC dispatch for node and
  sync methods.
- `scripts/run-live-mainnet-smoke.ts` - Opt-in live-smoke schema, compact
  snapshots, restart/resume evidence, no-progress causes, and final status.
- `scripts/test-run-live-mainnet-smoke.sh` - Deterministic live-smoke fixture
  checks.
- `scripts/check-phase71-resource-restart.ts` - Prior phase checker style to
  follow for Phase 72.
- `scripts/verify.sh` - Repo-native deterministic verification contract.

### Operator Docs And Parity Roots

- `docs/architecture/status-snapshot.md` - Shared status snapshot truth
  contract, sync progress, tip/stay-current, no-progress, reorg, resource, and
  metrics semantics.
- `docs/architecture/operator-observability.md` - Metrics/log/support evidence,
  retention, compact snapshots, and deterministic verification boundaries.
- `docs/operator/runtime-guide.md` - Operator status interpretation, restart
  evidence, resource pressure, support bundle, live-smoke, and UAT guidance.
- `docs/architecture/storage-decision.md` - Storage recovery and same-datadir
  restart boundaries.
- `docs/parity/catalog/p2p.md` - P2P/sync parity scope, support evidence, and
  deferred production-node boundaries.
- `docs/parity/catalog/chainstate.md` - Active-chain, UTXO/undo, reorg, and
  chainstate persistence parity scope.
- `docs/parity/catalog/operator-runtime-release-hardening.md` - Operator
  runtime and release hardening parity boundaries.
- `docs/parity/index.json` and `docs/parity/source-breadcrumbs.json` - Parity
  root registry and required first-party source/test breadcrumbs.

</canonical_refs>

<code_context>

## Existing Code Insights

### Reusable Assets

- `OpenBitcoinStatusSnapshot` already aggregates node, config, service, sync,
  peer, mempool, wallet, log, metric, health, and build evidence for CLI,
  dashboard, and support consumers.
- `SyncStatus` already carries Phase 68-71 fields for validated active-chain
  progress, best-known tip, stay-current state/action, no-progress
  diagnosis/action, latest reorg, reconcile progress, and resource pressure.
- `SyncRunSummary` already projects status, peer status, metric samples, and
  structured log records from one runtime summary.
- CLI status JSON already serializes the full snapshot. Human CLI and dashboard
  projections are the likely places where newer fields need explicit rendering.
- Support bundles already include redaction metadata, the shared status
  snapshot, store-health availability, and allowlisted live-smoke summaries.
- `support/live_smoke.rs` already enforces summary allowlists for live-smoke
  schema v2 and is the natural place to add Phase 72 support evidence fields.
- The Phase 68, 69, 70, and 71 checker scripts establish the repo-local pattern
  for deterministic source/docs/test/default-verification guards.

### Established Patterns

- Public-network full-sync, service-manager, and long-running smoke review stay
  opt-in UAT and outside `bash scripts/verify.sh`.
- Additive status fields use `FieldAvailability<T>` and serde defaults so old
  status JSON remains decodable.
- Human renderers should show `Unavailable: {reason}` instead of dropping
  fields or inventing local substitutes.
- Support evidence is compact and allowlisted. It intentionally avoids raw
  daemon output, raw logs, raw live-smoke input, secrets, wallet material, and
  unbounded peer tables.
- Phase closeout checkers verify artifacts, source fields, deterministic tests,
  docs wording, and default-verification exclusions together.

### Integration Points

- Fill any missing human/dashboard/RPC/support/live-smoke rendering gaps from
  the existing `SyncStatus` fields before adding new domain concepts.
- Add a small shared projection or comparison helper if it keeps CLI, dashboard,
  support, RPC, logs, metrics, and live-smoke aligned without duplicating field
  selection logic.
- Extend support evidence with a typed final verdict only after deriving it from
  existing status and live-smoke evidence.
- Update docs and checkers as part of the same phase so operators can interpret
  the evidence without reading implementation code.

</code_context>

<specifics>

## Specific Ideas

- Keep the main decision rule simple: if surfaces disagree, the shared status
  snapshot wins and the renderer/projection should be fixed.
- Treat bundle existence, elapsed time, peer reachability, and daemon startup as
  insufficient evidence by themselves; the verdict must cite connected
  active-chain, tip freshness, stay-current, blocker, reorg, restart/resume, or
  resource-pressure fields.
- Prefer compact comparison fixtures over public-network evidence for Phase 72.

</specifics>

<deferred>

## Deferred Ideas

- Full opt-in public-mainnet UAT command matrix is Phase 73 scope.
- Release-readiness matrix, threat model closeout, and final v1.6 claim-boundary
  checks are Phase 74 scope.
- Hosted dashboards, GUI surfaces, inbound serving, relay, production wallets,
  migration apply mode, signed packages, and broad production-node claims remain
  out of scope for v1.6.

</deferred>

---

*Phase: 72-operator-observability-and-support-evidence*
*Context gathered: 2026-06-13*
