---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 121-2026-07-14T04-25-57
generated_at: 2026-07-14T04:28:00.000Z
---

# Phase 121: Block Relay Metrics and Log Runtime Projection - Context

**Gathered:** 2026-07-14
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Phase 121 closes the OBS-03 runtime seam: project already-built `block_relay_metric_samples` and `block_relay_log_record` helpers through `DurableSyncRuntime` persist and structured-logging paths so retained metrics and sync logs carry fixed low-cardinality block-relay series beyond helper-only unit coverage.

This phase must not add new P2P behavior, change activation defaults, invent new metric kinds or dynamic labels, expand dashboard/RPC/CLI product surfaces, or claim package/filter/public-default/production readiness. Helper vocabulary and sanitization already exist from Phase 116; this phase only wires runtime projection.

</domain>

<decisions>
## Implementation Decisions

### Metric Source And Provider Wiring

- **D-01:** Treat `BlockRelayEvidenceStatus` from managed network evidence (`ManagedPeerNetwork::block_relay_evidence_status` / equivalent shared status projection) as the canonical source for block-relay metric samples. Do not duplicate counters.
- **D-02:** Mirror the Phase 97 inbound pattern: add a `DurableSyncRuntime` provider setter (for example `set_block_relay_metric_status_provider`) that returns `FieldAvailability<BlockRelayEvidenceStatus>` (or an equivalent available/unavailable wrapper), and call `block_relay_metric_samples` only when status is available.
- **D-03:** Extend `DurableSyncRuntime::persist_metrics` to append `block_relay_metric_samples(...)` alongside existing sync and inbound samples through `FjallNodeStore::append_metric_samples` and the existing retention policy. Do not create a parallel metrics store.

### Persist Omission Semantics

- **D-04:** When block-relay status is unavailable, emit no block-relay metric samples (same posture as inbound D-03 in Phase 97). Do not manufacture zero-valued availability evidence that would imply runtime projection occurred.
- **D-05:** Reuse existing fixed `MetricKind` variants and the Phase 116 helper mapping unchanged. No new kinds, no peer ids, endpoints, permission strings, credentials, transaction payloads, or dynamic label dimensions.

### Structured Log Emission Path

- **D-06:** Emit `block_relay_log_record` through the sync runtime structured-log path (same effectful append used by `write_summary_logs` / `append_structured_record`), not through a new log writer or by parsing log text.
- **D-07:** Emit the block-relay log record when the same availability condition as metrics is met (status available). Reuse the existing `block_relay_log_record` helper and its fixed low-cardinality `outcome`/`cause`/`label` vocabulary without adding sensitive or dynamic fields.
- **D-08:** Keep pure helpers side-effect-free. Filesystem append stays in the sync runtime shell adapter.

### Verification And Leakage Guardrails

- **D-09:** Add runtime-level tests (DurableSyncRuntime persist/log path) that prove samples and log records appear when a provider returns available status, and are omitted when unavailable — beyond Phase 116 helper-only unit coverage.
- **D-10:** Prove no raw peer, permission, credential, or transaction payload leakage in persisted metric sample kinds/messages and emitted structured log records (reuse existing sanitization/redaction assertions patterns).
- **D-11:** Add a deterministic Phase 121 checker (Bun/TypeScript under `scripts/`) proving production-callable wiring into `persist_metrics` and structured-log emission, helper reuse, and verifier inclusion; wire it into `bash scripts/verify.sh`.
- **D-12:** Default verification remains `bash scripts/verify.sh` — deterministic, local, public-network-free.

### Folded Todos

No pending todos matched this phase.

### Claude's Discretion

Exact provider type wrapper naming, whether log emission shares the metrics provider or a twin setter, module placement within `sync/`, checker script naming, and fixture construction are agent discretion — provided the Phase 97 mirror, availability-gated emission, helper reuse, and leakage guardrails hold.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase Scope And Gap Evidence

- `.planning/ROADMAP.md` — Phase 121 goal and success criteria
- `.planning/REQUIREMENTS.md` — OBS-03 requirement wording and Phase 121 ownership
- `.planning/v2.1-MILESTONE-AUDIT.md` — OBS-03 unsatisfied evidence: helpers exist but `persist_metrics` / sync logs omit block-relay projection
- `.planning/phases/116-operator-evidence-metrics-logs-and-support-boundary/116-CONTEXT.md` — locked OBS-03 label vocabulary and shared `BlockRelayEvidenceStatus` contract
- `.planning/phases/97-inbound-metrics-sample-production/97-CONTEXT.md` — canonical inbound metrics provider + persist_metrics extension pattern
- `.planning/phases/99-peer-policy-structured-log-emission/99-CONTEXT.md` — structured-log emission + sanitization verification posture

### Metrics, Logging, And Runtime Persistence

- `packages/open-bitcoin-node/src/metrics/block_relay.rs` — `block_relay_metric_samples`
- `packages/open-bitcoin-node/src/metrics.rs` — `MetricKind`, retention, inbound sample pattern
- `packages/open-bitcoin-node/src/logging.rs` — `block_relay_log_record` and fixed source labels
- `packages/open-bitcoin-node/src/sync/metrics.rs` — `DurableSyncRuntime::persist_metrics` and inbound provider
- `packages/open-bitcoin-node/src/sync/runtime_state.rs` — `write_summary_logs` / `append_structured_record`
- `packages/open-bitcoin-node/src/network/block_relay_evidence.rs` — managed evidence projection into `BlockRelayEvidenceStatus`
- `packages/open-bitcoin-node/src/status/block_relay_evidence.rs` — shared status contract
- `docs/architecture/operator-observability.md` — low-cardinality observability constraints
- `scripts/verify.sh` — repo-native verification contract

### Project Rules

- `AGENTS.md`, `AGENTS.bright-builds.md`
- `standards/core/architecture.md`, `standards/core/testing.md`, `standards/core/verification.md`
- `standards/languages/rust.md`, `standards/languages/typescript-javascript.md`

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- `block_relay_metric_samples` and `block_relay_log_record` already map `BlockRelayEvidenceStatus` into fixed MetricKind samples and sanitized structured logs with unit coverage.
- `DurableSyncRuntime::set_inbound_metric_status_provider` + `persist_metrics` already show the exact extension point for appending an additional sample family.
- `write_summary_logs` already loops summary records into `append_structured_record` on the datadir log directory.
- `ManagedPeerNetwork::block_relay_evidence_status` already produces the shared evidence snapshot operators see elsewhere.

### Established Patterns

- Pure mapping helpers in `metrics/` and `logging/`; effectful append only in sync/runtime shell.
- Unavailable status ⇒ omit that family's samples rather than inventing zero availability evidence (Phase 97 inbound).
- Phase checkers under `scripts/check-phaseNN-*.ts` wired into `scripts/verify.sh`.

### Integration Points

- Extend `packages/open-bitcoin-node/src/sync/metrics.rs` for provider + persist append.
- Extend sync structured-log emission near `write_summary_logs` / runtime tick that already persists metrics.
- Add DurableSyncRuntime tests beside existing `persist_metrics_appends_inbound_status_samples_*` coverage.
- Add Phase 121 checker + verify wiring; keep Phase 120 isolation negative assertions intact.

</code_context>

<specifics>
## Specific Ideas

Mirror Phase 97 literally for metrics: provider → availability gate → `block_relay_metric_samples` → `append_metric_samples` in the same `persist_metrics` call as sync/inbound samples.

For logs, emit `block_relay_log_record` on the same runtime tick/availability condition so metrics and logs stay coherent, without expanding CLI/dashboard/RPC surfaces (already closed in Phase 116).

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope. Package relay, bloom/filter serving, public defaults, production readiness, and new operator UI remain out of scope.

</deferred>

---

*Phase: 121-block-relay-metrics-and-log-runtime-projection*
*Context gathered: 2026-07-14*
