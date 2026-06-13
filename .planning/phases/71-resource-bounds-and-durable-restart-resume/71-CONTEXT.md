---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 71-2026-06-13T10-34-37
generated_at: 2026-06-13T10:36:32.206Z
---

# Phase 71: Resource Bounds and Durable Restart/Resume - Context

**Gathered:** 2026-06-13
**Status:** Ready for planning
**Mode:** Yolo

<domain>

## Phase Boundary

Phase 71 proves that explicit opt-in long full-sync attempts stay inside
documented resource bounds and recover safely from shutdown, interruption, stale
in-flight work, and storage pressure. It covers RES-01 through RES-04 only:
resource bounds, restart/resume safety, typed storage-pressure recovery guidance,
and deterministic synthetic long-chain verification.

This phase must not broaden the milestone into production full-node readiness,
inbound serving, relay, public-network default verification, production-funds
wallet safety, migration apply mode, packaging, GUI, hosted dashboards, or
trusted shortcut sync.

</domain>

<decisions>

## Implementation Decisions

### Resource Bound Contract

- **D-01:** Extend the existing bounded sync contract instead of inventing a
  parallel resource model. `SyncResourcePressure` remains the operator-facing
  resource envelope and should cover peers, in-flight blocks, header request
  limits, message and round caps, and configured outbound targets. If Phase 71
  needs additional proof fields for queues, caches, storage writes, logs,
  metrics, or support evidence, add them as typed bounded facts or compact
  summaries, not renderer-local strings.
- **D-02:** Keep queue and retention behavior explicitly bounded. The runtime
  should avoid unbounded in-memory queues, retained peer outcome arrays, log
  samples, metrics samples, support report material, or durable write backlogs.
  Where a bound is enforced by an existing retention policy or synchronous
  adapter call, document and test that fact.
- **D-03:** Resource-limit blockers should remain typed runtime outcomes.
  Zero or exhausted block budgets, storage pressure, and low disk conditions
  should surface through `SyncRecoveryCategory`, `SyncRuntimeError`, shared
  status, and next-action guidance rather than vague "sync failed" messages.

### Restart And Interruption Matrix

- **D-04:** Same-datadir resume safety must be proven deterministically for
  clean shutdown, unclean shutdown, mid-download interruption, mid-connect
  interruption, and stale in-flight work. Reuse `DurableSyncRuntime`, Fjall
  reopen, `ScriptedTransport`, and existing block reconcile fixtures where they
  already prove behavior.
- **D-05:** Resume evidence should preserve the Phase 58, Phase 64, Phase 68,
  and Phase 70 truth contract: durable headers, downloaded bodies, connected
  active-chain state, UTXO/undo snapshot, runtime metadata, best-known tip
  evidence, stale in-flight cleanup, and typed recovery category. Already
  connected blocks must not be requested or connected again after reopen.
- **D-06:** Stale in-flight work after restart must be cleared, reassigned, or
  diagnosed explicitly. It must not make the daemon look busy when no peer can
  satisfy the work, and it must not hide durable progress that is safe to resume.

### Storage Pressure And Recovery Guidance

- **D-07:** Storage recovery guidance keeps storage-first precedence. Schema
  mismatch, corruption markers, lock contention, low disk, and storage pressure
  outrank peer retry advice and must not trigger hidden data mutation.
- **D-08:** Add or refine typed recovery categories only where the existing
  taxonomy cannot express Phase 71 requirements. `incompatible_schema`,
  `store_corruption`, `storage_lock_contention`, `storage_backend_failure`, and
  `resource_exhaustion` already exist; low-disk and storage-pressure evidence
  may map to `resource_exhaustion` only if operator guidance remains precise.
- **D-09:** Recovery guidance should be actionable and quiet: inspect storage
  health, free disk, close the competing process holding the lock, run the
  explicit repair/reindex path where available, increase a configured bound, or
  retry after peer backoff. Do not imply automatic repair or mutation.

### Deterministic Long-Chain Verification

- **D-10:** RES-04 should be proven through deterministic synthetic long-chain
  tests, not public-mainnet timing. Tests should exercise bounded peer fanout,
  in-flight block caps, request queues, block reconciliation, durable reconnect,
  restart/resume, stale in-flight cleanup, metrics/log retention, and support
  evidence compactness.
- **D-11:** The synthetic long-chain path should use first-party fixtures and
  scripted transport rather than new production dependencies. Prefer pure
  helper functions and typed fixtures where possible so the test isolates the
  bound being proven.
- **D-12:** `bash scripts/verify.sh` remains the final deterministic verification
  contract. Public-network full-sync, long-run, service-manager, or
  `--restart-after-progress` commands may be documented as opt-in UAT only.

### Operator Evidence And Documentation

- **D-13:** Operator surfaces should describe what the evidence proves:
  bounded long sync, safe same-datadir resume, diagnosed storage or resource
  blocker, or deferred production-node scope. Keep status, docs, support
  evidence, metrics, logs, and live-smoke reports aligned on field names.
- **D-14:** Update operator docs, architecture docs, parity notes, and focused
  deterministic checkers only where Phase 71 changes the truth contract or
  resource/recovery guidance. Preserve copy-pasteable repo-local Cargo and Bazel
  command forms for opt-in operator workflows.
- **D-15:** If new first-party Rust source or test files are added under
  `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests`, update
  parity breadcrumbs through `docs/parity/source-breadcrumbs.json` and
  `scripts/check-parity-breadcrumbs.ts`.

### the agent's Discretion

- The planner may split work across resource contract/status fields,
  restart/interruption fixtures, storage-pressure recovery guidance,
  synthetic long-chain tests, and docs/checker closeout.
- The executor may add small pure helper types for storage pressure or
  restart/resume classification when they keep illegal states unrepresentable
  and avoid renderer duplication.
- The executor may keep new tests inside existing sync/status/storage test files
  when that is the smallest robust path. If new files are cleaner, parity
  breadcrumbs are mandatory.
- The executor may preserve existing recovery labels and add more precise
  next-action text instead of adding enum variants, provided RES-03 remains
  auditable and typed.

</decisions>

<canonical_refs>

## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase Scope

- `.planning/ROADMAP.md` - Phase 71 goal, dependency on Phase 70, success
  criteria, and deferred Phase 72 through Phase 74 boundaries.
- `.planning/REQUIREMENTS.md` - RES-01 through RES-04, related OBS/VER
  downstream requirements, and v1.6 out-of-scope table.
- `.planning/PROJECT.md` - v1.6 milestone goal, pinned Knots baseline,
  functional-core boundary, and production-claim limits.
- `.planning/STATE.md` - Current milestone state and Phase 71 readiness.
- `AGENTS.md` - Repo-local workflow, Rust, parity breadcrumb, and verification
  requirements.
- `AGENTS.bright-builds.md` - Bright Builds workflow and standards routing.
- `standards-overrides.md` - Current local standards override registry.

### Prior Phase Decisions And Evidence

- `.planning/phases/58-same-datadir-restart-and-resume-evidence/58-CONTEXT.md`
  - Two-session same-datadir restart evidence, duplicate-connect verdicts, and
  restart diagnosis taxonomy.
- `.planning/phases/61-resource-bounds-and-recovery-taxonomy/61-CONTEXT.md` -
  `SyncResourcePressure`, recovery categories, storage-first recovery
  precedence, support evidence compactness, and deterministic verification
  posture.
- `.planning/phases/62-long-run-sync-truth-surfaces/62-CONTEXT.md` - Shared
  truth fields, bounded metrics/logs, explicit unavailable states, and
  cross-surface agreement.
- `.planning/phases/64-service-restart-and-same-datadir-resume-evidence/64-CONTEXT.md`
  - Service-supervised same-datadir resume, stale in-flight verdicts, and
  service-manager UAT boundaries.
- `.planning/phases/68-full-active-chain-validation-and-durable-persistence/68-CONTEXT.md`
  - Validated active-chain progress credit, durable persistence, and no-credit
  downloaded-only or invalid peer outcomes.
- `.planning/phases/69-tip-tracking-and-stay-current-operation/69-CONTEXT.md` -
  Best-known tip evidence, stay-current states, restart persistence of tip
  evidence, and no-progress/stale-tip split.
- `.planning/phases/70-reorg-peer-rotation-and-no-progress-recovery/70-CONTEXT.md`
  - Durable reorg, stale in-flight release, peer rotation, and typed
  no-progress next actions.
- `.planning/phases/70-reorg-peer-rotation-and-no-progress-recovery/70-VERIFICATION.md`
  - Passed Phase 70 evidence and residual Phase 71 resource/restart risks.

### Implementation Surfaces

- `packages/open-bitcoin-node/src/status.rs` - `SyncResourcePressure`,
  `DurableSyncState`, `SyncStatus`, `NoProgressDiagnosis`,
  `SyncReconcileProgressStatus`, service restart/resume status, and field
  availability contracts.
- `packages/open-bitcoin-node/src/status/recovery.rs` - Stable
  `SyncRecoveryCategory` labels and storage/resource recovery categories.
- `packages/open-bitcoin-node/src/storage.rs` - `RuntimeMetadata`,
  `RecoveryMarker`, `StorageError`, `StorageRecoveryAction`, schema mismatch,
  lock/contention, corruption, and repair guidance.
- `packages/open-bitcoin-node/src/storage/fjall_store.rs` - Durable metadata,
  recovery marker, clean-shutdown, schema compatibility, backend error, and
  persisted state integration.
- `packages/open-bitcoin-node/src/storage/snapshot_codec.rs` - Versioned
  persisted DTOs for chainstate, UTXO/undo, runtime metadata, headers, and
  block index.
- `packages/open-bitcoin-node/src/sync.rs` - `DurableSyncRuntime`, bounded sync
  cycles, restart/reopen, peer iteration, and opt-in daemon entrypoints.
- `packages/open-bitcoin-node/src/sync/runtime_state.rs` - Durable status
  projection, resource pressure, metrics/log persistence, best-height helpers,
  recovery category precedence, and persisted runtime state.
- `packages/open-bitcoin-node/src/sync/block_reconcile.rs` - Block in-flight
  caps, missing-block request bounds, stale in-flight release, active-chain
  reconnect, branch replacement, and storage blockers.
- `packages/open-bitcoin-node/src/sync/block_response.rs` - Requested block
  handling, no-credit peer attribution, and in-flight release on block or
  `notfound`.
- `packages/open-bitcoin-node/src/sync/progress.rs` - Peer progress,
  retry/backoff, resource-limit signals, no-progress causes, and next actions.
- `packages/open-bitcoin-node/src/sync/types.rs` - `SyncRuntimeConfig`, sync
  bounds, peer outcomes, stop reasons, runtime errors, and recovery mapping.
- `packages/open-bitcoin-node/src/sync/types/summary.rs` - Shared status,
  metrics, structured logs, peer projection, resource pressure, and summary
  contracts.
- `packages/open-bitcoin-node/src/sync/tests.rs` - Deterministic scripted
  transport, resolver, storage, restart, metrics, logs, peer failures, and
  reorg fixtures.
- `packages/open-bitcoin-cli/src/operator/status/render.rs` - Human and JSON
  status rendering for pressure, recovery, no-progress, and restart evidence.
- `packages/open-bitcoin-cli/src/operator/status/service_status.rs` - Service
  same-datadir restart/resume projection from runtime metadata.
- `packages/open-bitcoin-cli/src/operator/runtime/support.rs` - CLI sync
  status/pause/resume support text and resource-pressure rendering.
- `packages/open-bitcoin-cli/src/operator/support/live_smoke.rs` - Allowlisted
  live-smoke support evidence extraction.
- `packages/open-bitcoin-cli/src/operator/support/render.rs` - Redacted support
  evidence rendering.
- `packages/open-bitcoin-rpc/src/context.rs` - RPC sync control and durable
  metadata fallback.
- `scripts/run-live-mainnet-smoke.ts` - Opt-in live-smoke resource pressure,
  restart/resume, and recovery diagnosis report schema.
- `scripts/test-run-live-mainnet-smoke.sh` - Deterministic live-smoke fixture
  checks.
- `scripts/verify.sh` - Repo-native deterministic verification contract.

### Operator Docs And Parity Roots

- `docs/operator/runtime-guide.md` - Runtime resource bounds, same-datadir
  restart/resume, storage recovery guidance, support evidence, and opt-in UAT
  commands.
- `docs/architecture/status-snapshot.md` - Shared status snapshot truth
  contract, resource-pressure fields, no-progress diagnosis, and recovery
  labels.
- `docs/architecture/operator-observability.md` - Metrics/log retention,
  support evidence compactness, and deterministic verification boundary.
- `docs/architecture/storage-decision.md` - Fjall storage decision, restart,
  recovery, schema, and corruption testing posture.
- `docs/parity/catalog/p2p.md` - P2P/sync parity scope, restart/resume,
  support evidence, and production-node boundary.
- `docs/parity/catalog/chainstate.md` - Active-chain, UTXO/undo, reorg, and
  chainstate persistence parity scope.
- `docs/parity/threat-model-v1.5.md` - Public peer input, recovery,
  stale in-flight, and service restart/resume threat boundaries.
- `docs/parity/index.json` and `docs/parity/source-breadcrumbs.json` - Parity
  root registry and required first-party source/test breadcrumbs.

### Baseline Anchors

- `packages/bitcoin-knots/src/init.cpp` - Startup, shutdown, datadir, and
  interruption lifecycle anchor.
- `packages/bitcoin-knots/src/net.cpp` - Peer connection, retry, and resource
  lifecycle anchor.
- `packages/bitcoin-knots/src/net_processing.cpp` - Peer sync, invalid data,
  block request attribution, no-credit behavior, and in-flight work anchor.
- `packages/bitcoin-knots/src/headerssync.cpp` - Header sync progress and
  request-bound behavior anchor.
- `packages/bitcoin-knots/src/node/blockstorage.cpp` - Block storage, restart,
  and recovery anchor.
- `packages/bitcoin-knots/src/validation.cpp` - Active-chain validation and
  connect/reconnect behavior anchor.

</canonical_refs>

<code_context>

## Existing Code Insights

### Reusable Assets

- `SyncResourcePressure` already exposes active in-flight block count and
  configured limits for headers, blocks, messages, rounds, and outbound peers.
  Phase 71 should extend this typed surface only when a missing bound cannot be
  proven from existing fields or docs.
- `SyncRecoveryCategory` already covers clean/unclean shutdown, incompatible
  schema, store corruption, storage lock contention, storage backend failure,
  resource exhaustion, invalid peer data, public-network unreachability, and
  operator cancellation.
- `DurableSyncRuntime` persists header entries, chainstate snapshots, runtime
  metadata, metrics, and structured logs through typed shell methods that can be
  exercised with deterministic fixtures.
- `block_reconcile.rs` already enforces zero-budget blockers and global
  in-flight caps, releases in-flight work on block and `notfound`, and reconnects
  stored best-chain blocks through the active-chain path.
- CLI status, support bundle, live-smoke, and service restart/resume surfaces
  already render typed resource pressure and recovery evidence.

### Established Patterns

- Storage and resource blockers should be modeled as domain/runtime errors and
  projected through shared status fields before rendering.
- Missing operator evidence should use `FieldAvailability::Unavailable` with a
  reason, not zeroes or implicit success.
- Public-network and service-manager checks stay opt-in UAT; deterministic Rust
  tests and Bun fixture checkers cover default verification.
- Docs and support evidence prefer compact summaries over raw daemon tails,
  unbounded peer tables, or raw live-smoke snapshots.

### Integration Points

- Status contract changes start in `packages/open-bitcoin-node/src/status.rs`
  and project through `sync/runtime_state.rs`, CLI renderers, RPC, docs, and
  support evidence.
- Durable restart/resume behavior is primarily tested through
  `packages/open-bitcoin-node/src/sync/tests.rs` with real store reopen and
  scripted transport.
- Storage-pressure and low-disk guidance should map through `StorageError`,
  `StorageRecoveryAction`, `SyncRuntimeError`, and `SyncRecoveryCategory`
  before reaching operator surfaces.
- If docs or checker scripts change, `scripts/verify.sh` and
  `docs/metrics/lines-of-code.md` may need freshness updates through the
  repo-owned verification path.

</code_context>

<specifics>

## Specific Ideas

- Treat Phase 71 as a proof-hardening phase: make the bounds and recovery cases
  impossible to miss in status, docs, and tests rather than adding broad new
  sync features.
- Prefer "bounded by X" evidence over vague claims. For example, list the exact
  in-flight, peer, queue, retention, and storage-write bounds where operators
  inspect status or docs.
- Low disk and storage pressure should be explicit enough for an operator to
  know whether to free disk, stop a competing process, run a repair/reindex
  workflow, or increase a configured bound.
- Synthetic long-chain tests should deliberately keep the public-network path
  out of default verification while still exercising mainnet-scale style
  resource behavior.

</specifics>

<deferred>

## Deferred Ideas

None - discussion stayed within Phase 71 scope.

</deferred>

---

*Phase: 71-resource-bounds-and-durable-restart-resume*
*Context gathered: 2026-06-13*
