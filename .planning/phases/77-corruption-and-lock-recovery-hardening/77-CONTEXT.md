---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 77-2026-06-15T18-33-03
generated_at: 2026-06-15T18:39:45.451Z
---

# Phase 77: Corruption and Lock Recovery Hardening - Context

**Gathered:** 2026-06-15
**Status:** Ready for planning
**Mode:** Yolo

<domain>

## Phase Boundary

Phase 77 makes store lock, stale-lock, corruption-marker, schema-mismatch,
partial-write, unreadable-store, and storage-open failures safe to diagnose
from operator surfaces without hidden datadir mutation. It owns REC-05 through
REC-08: non-mutating lock and concurrent datadir evidence, typed recovery
categories for corruption-style failures, recovery evidence that separates safe
retry, read-only inspection, backup-then-rebuild, and stop-and-escalate
guidance, and deterministic recovery fixtures.

This phase builds on the existing durable Fjall storage contracts,
`SyncRecoveryCategory`, `StorageRecoveryAction`, stopped-node status,
service restart/resume evidence, support evidence, and soak outcome mapping.
It must not silently delete, repair, compact, reindex, relocate, or otherwise
mutate a source datadir. Automatic destructive repair remains explicitly out of
scope; Phase 77 may diagnose and guide recovery only. It also must keep default
verification deterministic, public-network-free, service-manager-free, and free
of large disk allocations.

</domain>

<decisions>

## Implementation Decisions

### Lock Contention And Stale-Lock Evidence

- **D-01:** Add a probe-only lock evidence path that does not call the normal
  store-open path when the operator is only asking for status, support, or
  recovery inspection. Probe-only collection must not create schema records,
  write recovery markers, clear stale artifacts, repair stores, delete lock
  files, or otherwise mutate the selected datadir.
- **D-02:** Distinguish at least three lock cases in typed evidence: active lock
  contention, stale-lock evidence, and concurrent datadir use. Concurrent use
  may be inferred by combining lock evidence with service status, same-datadir
  service evidence, live RPC availability, or other already-collected bounded
  operator evidence; do not depend on non-portable process scans as the core
  classification.
- **D-03:** Backend open failures should still map through `StorageError` and
  `SyncRecoveryCategory::StorageLockContention` when the real storage adapter
  reports a lock/locked/contention condition. The new read-only evidence path
  complements backend-open classification; it does not replace the storage
  adapter's typed error mapping.
- **D-04:** Treat owner heartbeat or PID sentinels as optional future evidence,
  not the default Phase 77 mechanism. A sentinel adds a new mutation surface and
  cleanup/redaction policy that is not needed to satisfy the phase unless
  planning proves lock evidence cannot be made actionable without it.

### Recovery Taxonomy And Action Classes

- **D-05:** Preserve the stable `SyncRecoveryCategory` labels whenever possible:
  `incompatible_schema`, `store_corruption`, `storage_lock_contention`,
  `storage_backend_failure`, and `resource_exhaustion` remain the compatibility
  summary labels for storage and resource failures.
- **D-06:** Add typed recovery evidence beside the existing category/action
  summary instead of expanding `SyncRecoveryCategory` for every root cause. The
  evidence should carry cause details such as schema mismatch, corruption
  marker, partial write, unreadable namespace, backend open failure, active
  lock, stale lock, and concurrent datadir evidence.
- **D-07:** REC-07 guidance should use explicit action classes:
  `safe_retry`, `read_only_inspection`, `backup_then_rebuild`, and
  `stop_and_escalate`. Existing `StorageRecoveryAction` values may continue to
  feed compatibility text, but the new action class must be the durable
  operator-facing safety contract for this phase.
- **D-08:** Centralize mapping from storage errors, lock evidence, recovery
  markers, and status collectors into one pure classifier. Renderers and
  reports should consume the classifier output instead of parsing strings or
  independently remapping categories.
- **D-09:** `storage_backend_failure` remains the safe fallback for unreadable
  or unavailable stores when no more precise schema, corruption, lock, or
  resource pressure signal is available. Storage/resource pressure keeps the
  Phase 76 and Phase 71 precedence and continues to map to
  `resource_exhaustion`/`FreeDisk`.

### Shared Operator Evidence Surfaces

- **D-10:** Expose Phase 77 recovery evidence through a shared status contract,
  preferably a top-level `recovery_evidence:
  FieldAvailability<RecoveryEvidenceSnapshot>` on `OpenBitcoinStatusSnapshot`.
  Store-open and stopped-node failures can happen before `sync` state is
  available, so this should not be modeled as renderer-local prose or only as a
  nested `sync` field.
- **D-11:** Keep existing `sync.recovery_category` and `sync.recovery_action`
  as compatibility summaries. The richer evidence should preserve the category,
  action class, evidence basis, affected namespace or path, unavailable reason,
  and next action from one shared source of truth.
- **D-12:** CLI status, dashboard status, stopped-node status, support evidence,
  soak checkpoint/report projections, live-smoke summaries, and docs should all
  consume the same recovery evidence fields. Missing evidence must remain
  `Unavailable: {reason}` or the equivalent machine-readable unavailable
  reason; do not silently omit it.
- **D-13:** Soak outcomes should continue to classify schema, corruption, lock,
  and backend storage categories as `recovery_stop`, while resource exhaustion
  remains `resource_stop`. Phase 77 should make the recovery-stop evidence more
  precise without redefining the Phase 75 outcome vocabulary.

### Deterministic Verification

- **D-14:** REC-08 should be proven primarily with Rust tests using tiny temp
  datadirs and real Fjall behavior where practical. Schema mismatch, malformed
  records, recovery markers, partial-write markers, unreadable namespace
  behavior, and path-as-file/open-failure cases should follow the existing
  `fjall_store` and snapshot codec fixture style.
- **D-15:** Lock contention should use a deterministic lock-holder helper or
  subprocess only if the normal in-process fixture cannot prove real backend
  locking reliably. Test cleanup must be explicit and must not leak child
  processes or locks.
- **D-16:** Add a test-only storage opener or failure seam only where real OS or
  backend lock/open behavior is too platform-sensitive for default verification.
  Pure classifier tests should still cover the full recovery matrix so the
  safety contract is deterministic even if one adapter fixture needs a narrow
  portability guard.
- **D-17:** Bun checkers may supplement Rust tests for docs, phase artifact
  anchors, parity breadcrumbs, status/report field names, and default
  verification boundaries. They must not become the only proof of storage
  locks, schema bytes, corruption markers, or open failures.
- **D-18:** If new Rust source or test files are added under
  `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests`, add parity
  breadcrumbs through `docs/parity/source-breadcrumbs.json` and keep
  `scripts/check-parity-breadcrumbs.ts` green.

### the agent's Discretion

- The planner may split Phase 77 across recovery evidence domain types,
  probe-only lock collection, storage error/classifier mapping, status/support
  projection, soak outcome/report integration, docs, and deterministic
  checkers.
- The executor may add compact typed structs/enums for recovery cause, action
  class, lock evidence, and evidence basis when they make illegal states
  unrepresentable.
- The executor may keep existing `StorageRecoveryAction` messages as
  compatibility guidance while introducing safer action-class wording for
  Phase 77. Avoid broad enum churn unless an existing stable label cannot
  truthfully represent the behavior.

</decisions>

<canonical_refs>

## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase Scope

- `.planning/ROADMAP.md` - Phase 77 goal, dependency on Phase 76, success
  criteria, and Phase 78 through Phase 80 boundaries.
- `.planning/REQUIREMENTS.md` - REC-05 through REC-08, explicit out-of-scope
  automatic destructive repair, default verification exclusions, and
  traceability.
- `.planning/PROJECT.md` - v1.7 milestone goal, current state, pinned Knots
  baseline, functional-core boundary, and production-claim limits.
- `.planning/STATE.md` - current focus, accumulated v1.7 decisions, and
  default-verification constraints.
- `AGENTS.md` - repo-local GSD workflow, Rust, parity breadcrumb, UAT command,
  generated artifact, and verification requirements.
- `AGENTS.bright-builds.md` - Bright Builds workflow and standards routing.
- `standards-overrides.md` - local standards override registry.
- `standards/core/architecture.md` - functional-core, parse-at-boundaries, and
  illegal-state modeling rules.
- `standards/core/code-shape.md` - early-return, optional naming, script, and
  file-size guidance.
- `standards/core/verification.md` - sync-first and repo-native verification
  requirements.
- `standards/core/testing.md` - unit-test and Arrange/Act/Assert expectations.
- `standards/languages/rust.md` - Rust module, option naming, invariant, and
  verification guidance.
- `standards/languages/typescript-javascript.md` - Bun/TS automation and
  nullish naming guidance.

### Prior Phase Decisions And Evidence

- `.planning/phases/61-resource-bounds-and-recovery-taxonomy/61-CONTEXT.md` -
  earlier resource pressure, recovery category, and support-evidence compactness
  decisions.
- `.planning/phases/71-resource-bounds-and-durable-restart-resume/71-CONTEXT.md`
  - storage-first recovery precedence, `StorageRecoveryAction::FreeDisk`,
  restart/resume evidence, and deterministic long-chain fixtures.
- `.planning/phases/72-operator-observability-and-support-evidence/72-CONTEXT.md`
  - shared status truth contract, cross-surface alignment, and redacted support
  evidence boundaries.
- `.planning/phases/73-opt-in-uat-and-deterministic-verification/73-CONTEXT.md`
  - opt-in UAT command matrix and default-verification exclusions.
- `.planning/phases/74-release-boundaries-parity-and-documentation/74-CONTEXT.md`
  - scoped release-claim language and release-boundary checker posture.
- `.planning/phases/75-multi-day-soak-runner-and-evidence-ledger/75-CONTEXT.md`
  - `open-bitcoin soak`, durable ledger/report shape, outcome taxonomy,
  support summary projection, and deferred resource/recovery scope.
- `.planning/phases/76-disk-and-resource-bound-enforcement/76-CONTEXT.md` -
  resource-bound status surfaces, storage/resource precedence, support evidence,
  and deterministic fixture policy.
- `.planning/phases/76-disk-and-resource-bound-enforcement/76-VERIFICATION.md`
  - passed Phase 76 evidence and current readiness for Phase 77.

### Implementation And Verification Surfaces

- `packages/open-bitcoin-node/src/storage.rs` - `StorageNamespace`,
  `StorageRecoveryAction`, `RuntimeMetadata`, `RecoveryMarker`,
  `StorageError`, and storage recovery category/action mapping.
- `packages/open-bitcoin-node/src/storage/fjall_store.rs` - durable Fjall store
  open, schema validation, runtime metadata, recovery marker, backend failure,
  and corruption helpers.
- `packages/open-bitcoin-node/src/storage/fjall_store/tests.rs` - existing
  schema mismatch, malformed snapshot, recovery marker, and corruption tests.
- `packages/open-bitcoin-node/src/storage/snapshot_codec.rs` and
  `packages/open-bitcoin-node/src/storage/snapshot_codec/tests.rs` - snapshot
  decoding, corruption mapping, and deterministic malformed-data fixtures.
- `packages/open-bitcoin-node/src/status.rs` - `OpenBitcoinStatusSnapshot`,
  `SyncStatus`, `FieldAvailability`, resource bounds, service-facing status,
  and current recovery fields.
- `packages/open-bitcoin-node/src/status/recovery.rs` - stable
  `SyncRecoveryCategory` labels.
- `packages/open-bitcoin-node/src/sync/types/recovery.rs` - runtime, peer,
  stop-reason, storage detail, and string-detail recovery category mapping.
- `packages/open-bitcoin-node/src/sync/types/projection.rs` - storage health
  messages derived from `StorageError`.
- `packages/open-bitcoin-node/src/sync/runtime_state.rs` - durable runtime
  metadata persistence and sync state projection.
- `packages/open-bitcoin-node/src/sync/tests.rs` and
  `packages/open-bitcoin-node/src/sync/tests/soak.rs` - deterministic sync,
  restart/resume, storage, resource, and soak fixtures.
- `packages/open-bitcoin-cli/src/operator/status/service_status.rs` -
  stopped-node service restart/resume evidence and recovery action projection.
- `packages/open-bitcoin-cli/src/operator/status/render.rs` and
  `packages/open-bitcoin-cli/src/operator/dashboard/model.rs` - human status
  and dashboard recovery rendering.
- `packages/open-bitcoin-cli/src/operator/support/evidence.rs` - support
  evidence recovery summaries and diagnosed-blocker justification.
- `packages/open-bitcoin-cli/src/operator/soak/outcome.rs` - soak outcome
  classification for resource and recovery stops.
- `packages/open-bitcoin-cli/src/operator/soak/runtime.rs`,
  `packages/open-bitcoin-cli/src/operator/soak/ledger.rs`, and
  `packages/open-bitcoin-cli/src/operator/soak/report.rs` - soak runtime,
  checkpoint, ledger, and report projections.
- `packages/open-bitcoin-cli/tests/operator_binary.rs` - operator binary
  contract coverage for status/report JSON.
- `docs/architecture/status-snapshot.md` - shared status snapshot contract,
  recovery category/action semantics, stopped-node status, resource bounds, and
  soak outcome vocabulary.
- `docs/architecture/storage-decision.md` - Fjall storage choice, schema,
  corruption, interrupted write, reindex, and repair expectations.
- `docs/operator/runtime-guide.md` - operator-facing recovery guidance,
  deterministic boundaries, status field descriptions, and UAT command surface.
- `scripts/check-phase61-resource-recovery-boundaries.ts`,
  `scripts/check-phase64-service-restart-resume.ts`,
  `scripts/check-phase72-observability-evidence.ts`,
  `scripts/check-phase75-soak-runner.ts`, and
  `scripts/check-phase76-resource-bounds.ts` - existing deterministic checker
  style for recovery/status/resource/soak boundaries.

</canonical_refs>

<code_context>

## Existing Code Insights

### Reusable Assets

- `StorageError` already models schema mismatch, corruption, unavailable
  namespace, interrupted write, and backend failure, and exposes category/action
  mapping.
- `RecoveryMarker` and `RuntimeMetadata.maybe_last_recovery_action` already
  persist durable recovery hints; Phase 77 can reuse them while adding richer
  evidence that does not imply mutation.
- `SyncRecoveryCategory` already provides stable machine labels consumed across
  status, dashboard, support, live-smoke, and soak surfaces.
- `FieldAvailability<T>` is the established way to preserve unavailable
  evidence and reasons instead of silently omitting fields.
- `service_restart_resume_status` already combines selected datadir, service
  same-datadir evidence, prior shutdown, stale in-flight status, and recovery
  action; this is a useful integration point for concurrent datadir and
  stopped-node recovery evidence.
- Existing `fjall_store` tests already create temp stores, corrupt JSON records,
  force schema mismatch, and round-trip recovery markers.

### Established Patterns

- Shared status contracts are the source of truth. CLI, dashboard, support,
  soak, and docs should not parse strings or invent renderer-local summaries.
- Stable machine labels should be extended carefully. Prefer richer adjacent
  evidence over changing existing labels consumed by downstream artifacts.
- Recovery and resource blockers outrank peer retry guidance. Resource pressure
  remains distinct from storage/corruption recovery stops.
- Default verification stays deterministic and synthetic; public-network,
  service-manager, long-wall-clock, and large-disk tests remain opt-in.

### Integration Points

- Add pure recovery classifier code near storage/status domain types, then feed
  it from storage-open errors, probe-only lock evidence, recovery markers, and
  status collectors.
- Extend `OpenBitcoinStatusSnapshot` and status collection so stopped-node and
  store-open failure cases can expose recovery evidence even when durable sync
  state is unavailable.
- Update CLI status, dashboard model, support evidence, soak checkpoint/report,
  live-smoke summaries, runtime guide, and phase checker surfaces to consume the
  shared recovery evidence.
- Add Rust unit tests for classifier/action classes and adapter fixtures for
  real Fjall schema/corruption/marker/open/lock cases; add Bun checkers only for
  docs and artifact contract coverage that Rust tests do not prove.

</code_context>

<specifics>

## Specific Ideas

- Advisor research recommended a read-only lock probe that separates held lock,
  stale lock evidence, and concurrent datadir evidence without deleting or
  clearing lock artifacts.
- Advisor research recommended preserving `SyncRecoveryCategory` as the stable
  compatibility label and adding typed evidence/action classes beside it.
- Advisor research recommended a top-level recovery evidence field on
  `OpenBitcoinStatusSnapshot` because store-open failures may prevent `sync`
  state from loading.
- Advisor research recommended hybrid real-Fjall temp fixtures plus a narrow
  lock-holder helper or subprocess for lock contention, with a test-only seam
  only if backend/OS lock behavior is too platform-sensitive.

</specifics>

<deferred>

## Deferred Ideas

- Owner heartbeat or PID sentinel metadata for exact process attribution is
  deferred unless Phase 77 planning proves the simpler probe/service/RPC
  evidence cannot satisfy REC-05.
- OS process scanning such as `lsof` is deferred as optional support evidence
  because it is non-portable, permission-sensitive, and privacy-sensitive.
- A separate support or soak forensic recovery ledger is deferred to Phase 79
  unless a compact pointer is needed from Phase 77 evidence.
- Automatic destructive repair, hidden lock cleanup, hidden reindex, hidden
  datadir relocation, and source datadir mutation remain out of scope.

</deferred>

---

*Phase: 77-corruption-and-lock-recovery-hardening*
*Context gathered: 2026-06-15*
