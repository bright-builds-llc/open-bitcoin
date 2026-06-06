---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 61-2026-06-06T03-43-41
generated_at: 2026-06-06T03:45:06.553Z
---

# Phase 61: Resource Bounds and Recovery Taxonomy - Context

**Gathered:** 2026-06-06
**Status:** Ready for planning
**Mode:** Yolo

<domain>

## Phase Boundary

Phase 61 makes unattended sync resource bounds and recovery states trustworthy
across long runs. It owns typed recovery taxonomy, bounded resource and
evidence reporting, deterministic assertions that the existing sync loop
preserves those bounds, and operator-facing guidance for recovery next actions.

This phase does not add new public-network execution to default verification,
service-supervisor lifecycle behavior, same-datadir service restart evidence,
support-bundle collection expansion, compatibility-harness operator wrapping, or
a broad production-node claim. Those remain Phase 62 through Phase 67 or future
milestone work.

</domain>

<decisions>

## Implementation Decisions

### Bounded Resource Envelope

- **D-01:** Treat `SyncResourcePressure` as the shared status contract for
  active sync bounds. It must report observed in-flight block pressure plus
  configured limits for header requests, protocol header batch size, per-peer
  and total block in-flight limits, messages per peer, sync rounds, outbound
  peers, and target outbound peers.
- **D-02:** Do not introduce unbounded queues or retained report arrays for
  Phase 61. Retry state, peer outcomes, metrics samples, structured logs, and
  support evidence summaries must stay bounded by existing config and retention
  policies or by explicit compact summaries.
- **D-03:** Make long-run bound preservation deterministic. Add tests around
  scripted sync outcomes and projection/rendering surfaces rather than relying
  on public-network long-run tests.

### Recovery Taxonomy

- **D-04:** Normalize recovery states into a single operator-facing taxonomy:
  clean shutdown, unclean shutdown, incompatible schema, store corruption,
  storage lock contention or backend failure, resource exhaustion, invalid peer
  data, public-network unreachability, and operator cancellation.
- **D-05:** Storage incompatibility and corruption continue to outrank peer or
  network guidance. If durable metadata exposes a storage recovery action, status
  and support surfaces should present that before recommending network retries.
- **D-06:** Map existing low-level signals into typed recovery categories rather
  than adding ad hoc strings at each renderer. `StorageError`,
  `StorageRecoveryAction`, `PeerFailureReason`, `SyncStopReason`,
  `SyncRuntimeError`, live-smoke `maybeNoProgressCause`, and durable
  `last_clean_shutdown` are the input facts.

### Operator Truth Surfaces

- **D-07:** Status, dashboard, RPC sync status, structured logs, metrics, support
  evidence, and docs should use the same names for recovery categories, progress
  signals, resource pressure, and next action guidance. A renderer may choose
  human wording, but the underlying category labels must remain stable.
- **D-08:** Phase 61 should add compact support-evidence fields only where needed
  to expose bounds and recovery taxonomy. It must preserve the allowlist and
  redaction posture from Phase 59 and avoid embedding raw live-smoke reports,
  daemon tails, peer endpoint tables, secrets, wallet material, or unbounded log
  samples.
- **D-09:** Operator docs should explain how to inspect the active bounds and how
  to interpret recovery categories with copy-pasteable repo-local Cargo and
  Bazel commands. Public-network review commands remain clearly opt-in UAT.

### Verification Posture

- **D-10:** Default verification stays deterministic. Phase verification should
  include targeted Rust tests, Bun fixture checks if live-smoke/support scripts
  change, documentation/release-boundary checks where relevant, and the
  repo-native `bash scripts/verify.sh`.
- **D-11:** Public-network long-run UAT may be documented as an optional operator
  review path, but it must not become part of `bash scripts/verify.sh` or phase
  completion proof.

### the agent's Discretion

- The planner may introduce a small domain enum or projection helper for
  recovery categories if it reduces string duplication across status, support,
  live-smoke, and docs.
- The planner may keep resource-bound proof in existing sync/status tests if no
  new module boundary is justified. If new first-party Rust files are added,
  parity breadcrumbs must be updated.
- The executor may defer Phase 62-only truth-surface expansion if Phase 61 can
  prove typed states and bounds without broad status/dashboard/RPC rewrites.

</decisions>

<canonical_refs>

## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase Scope

- `.planning/ROADMAP.md` - Phase 61 goal, success criteria, dependencies, and
  deferred Phase 62 through Phase 67 boundaries.
- `.planning/REQUIREMENTS.md` - RR-01, RR-02, RR-04 and v1.5 out-of-scope
  boundaries.
- `.planning/PROJECT.md` - v1.5 milestone goal, current state, and release
  boundary constraints.
- `.planning/STATE.md` - Current milestone state and prior decisions affecting
  deterministic verification and operator evidence.

### Prior Phase Decisions

- `.planning/phases/60-unattended-sync-loop-control/60-CONTEXT.md` - Loop
  activation, stop-reason persistence, control surface, and deterministic
  verification decisions.
- `.planning/phases/60-unattended-sync-loop-control/60-01-SUMMARY.md` -
  Completed daemon-loop policy and stop-reason evidence.
- `.planning/phases/59-operator-evidence-threat-model-and-release-boundaries/59-CONTEXT.md`
  - Support evidence allowlist, cross-surface truth, and release-boundary
  decisions.
- `.planning/phases/59-operator-evidence-threat-model-and-release-boundaries/59-VERIFICATION.md`
  - Passed v1.4 evidence and residual release-boundary notes.
- `.planning/phases/58-same-datadir-restart-and-resume-evidence/58-CONTEXT.md`
  - Same-datadir restart/resume recovery diagnosis boundaries.
- `.planning/phases/57-block-download-and-connect-progress/57-CONTEXT.md` -
  Block in-flight bounds, invalid data, and no-credit peer outcome decisions.
- `.planning/phases/56-header-ibd-convergence/56-CONTEXT.md` - Header progress,
  no-progress diagnosis, and target-height stop decisions.
- `.planning/phases/55-outbound-handshake-compatibility-fixes/55-CONTEXT.md` -
  Peer failure, retry, backoff, and compatibility no-credit behavior.

### Implementation Surfaces

- `packages/open-bitcoin-node/src/status.rs` - Shared `SyncStatus`,
  `SyncResourcePressure`, lifecycle, progress signal, and recovery action
  status contracts.
- `packages/open-bitcoin-node/src/storage.rs` - `RuntimeMetadata`,
  `RecoveryMarker`, `StorageError`, `StorageNamespace`, and
  `StorageRecoveryAction` taxonomy inputs.
- `packages/open-bitcoin-node/src/storage/fjall_store.rs` - Durable metadata,
  recovery marker, clean-shutdown, schema mismatch, backend failure, and store
  corruption integration.
- `packages/open-bitcoin-node/src/sync.rs` - `DurableSyncRuntime` sync loop,
  peer retry/backoff, and stop-reason behavior.
- `packages/open-bitcoin-node/src/sync/types.rs` - `SyncRuntimeConfig`,
  `PeerFailureReason`, `SyncRunSummary`, `SyncStopReason`, and
  `SyncRuntimeError`.
- `packages/open-bitcoin-node/src/sync/runtime_state.rs` - Durable sync state
  projection, resource pressure projection, recovery action precedence, metrics
  persistence, and structured-log writing.
- `packages/open-bitcoin-node/src/sync/types/summary.rs` - Status, metrics, and
  structured-log projection of progress, stop reasons, recovery actions, and
  resource pressure.
- `packages/open-bitcoin-node/src/sync/types/projection.rs` - Sync phase names,
  storage health messages, and peer/status projection helpers.
- `packages/open-bitcoin-node/src/sync/tests.rs` - Scripted transport/resolver
  fixtures for deterministic bound and recovery tests.
- `packages/open-bitcoin-cli/src/operator/status/render.rs` - Human and JSON
  operator status rendering for pressure and recovery action.
- `packages/open-bitcoin-cli/src/operator/dashboard/model.rs` - Dashboard model
  projection of resource pressure and recovery guidance.
- `packages/open-bitcoin-cli/src/operator/support/live_smoke.rs` - Allowlisted
  live-smoke summary extraction and redaction.
- `packages/open-bitcoin-cli/src/operator/support/render.rs` - Markdown support
  evidence rendering.
- `packages/open-bitcoin-rpc/src/dispatch/node.rs` - RPC-facing sync status and
  recovery warning integration.
- `scripts/run-live-mainnet-smoke.ts` - Opt-in live-smoke recovery diagnosis,
  no-progress cause, and next-action taxonomy.
- `scripts/test-run-live-mainnet-smoke.sh` - Deterministic fixture checks for
  live-smoke recovery categories.

### Operator Docs And Parity Roots

- `docs/operator/runtime-guide.md` - Runtime resource bounds, durable recovery
  status, support evidence, opt-in UAT commands, and release-boundary wording.
- `docs/architecture/status-snapshot.md` - Shared status snapshot contract,
  `recovery_action`, and `resource_pressure` semantics.
- `docs/architecture/operator-observability.md` - Metrics/log retention and
  deterministic verification boundary.
- `docs/architecture/config-precedence.md` - Config source ownership and
  credential reporting boundaries.
- `docs/parity/release-readiness.md` - Release claim boundaries and deferred
  public-network/production surfaces.
- `docs/parity/threat-model-v1.4.md` - Existing threat-model coverage for
  resource bounds, support evidence, and operator-facing live evidence.
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

- `SyncResourcePressure` already exposes in-flight block count, configured block
  limits, message limits, sync-round limits, header request bounds, and outbound
  peer counts through the shared status snapshot.
- `DurableSyncRuntime::durable_sync_state_from_summary` already gives storage
  recovery metadata precedence over peer guidance and projects active resource
  pressure from runtime config plus current in-flight blocks.
- `SyncRunSummary` already projects progress metrics and structured logs with
  stable height, progress-signal, stop-reason, and peer-outcome fields.
- `StorageRecoveryAction` already carries operator messages for restart,
  reindex, repair, and restore-from-backup recovery paths.
- `scripts/run-live-mainnet-smoke.ts` already has coarse recovery diagnosis
  categories and deterministic fixture checks for store, resource, peer,
  network, invalid-data, and cancellation cases.

### Established Patterns

- Sync decisions stay in `open-bitcoin-node`; daemon and CLI/RPC layers project
  already-typed facts.
- Public-network live review is opt-in UAT and stays outside
  `bash scripts/verify.sh`.
- Support bundles are allowlist-based summaries. Raw reports, daemon tails,
  endpoint tables, credentials, wallet material, and unbounded logs stay out of
  checked-in evidence.
- Operator docs use repo-local Cargo and Bazel commands rather than relying on
  an installed `open-bitcoin` alias.

### Integration Points

- Recovery taxonomy helpers should sit close to sync/status domain types so CLI,
  support, RPC, and live-smoke fixtures can share stable labels.
- Resource-bound assertions should cover both runtime projection and renderer
  visibility: status JSON/human output, dashboard model text, structured logs,
  and support evidence only where Phase 61 changes those surfaces.
- Documentation should explain active fields and opt-in review commands in
  `docs/operator/runtime-guide.md`, with architecture docs updated if the shared
  status contract changes.

</code_context>

<specifics>

## Specific Ideas

No additional user-specific requests beyond the v1.5 milestone prompt. Use the
standard Open Bitcoin posture: opt-in, bounded, auditable, deterministic by
default, and explicit about deferred production-node scope.

</specifics>

<deferred>

## Deferred Ideas

- Phase 62 owns broader long-run truth consistency across status, dashboard,
  RPC, metrics, logs, and live-smoke snapshots.
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

*Phase: 61-resource-bounds-and-recovery-taxonomy*
*Context gathered: 2026-06-06*
