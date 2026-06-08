---
generated_by: gsd-plan-phase
lifecycle_mode: yolo
phase_lifecycle_id: 64-2026-06-08T03-22-46
generated_at: 2026-06-08T03:22:46.768Z
---

# Phase 64: Service Restart and Same-Datadir Resume Evidence - Research

## Inputs Read

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/PROJECT.md`
- `.planning/STATE.md`
- `.planning/phases/64-service-restart-and-same-datadir-resume-evidence/64-CONTEXT.md`
- `.planning/phases/63-service-supervision-lifecycle/63-CONTEXT.md`
- `.planning/phases/63-service-supervision-lifecycle/63-01-PLAN.md`
- `.planning/phases/63-service-supervision-lifecycle/63-03-PLAN.md`
- `.planning/phases/58-same-datadir-restart-and-resume-evidence/58-01-PLAN.md`
- `.planning/phases/58-same-datadir-restart-and-resume-evidence/58-VERIFICATION.md`
- `packages/open-bitcoin-node/src/status.rs`
- `packages/open-bitcoin-node/src/sync/runtime_state.rs`
- `packages/open-bitcoin-node/src/sync/tests.rs`
- `packages/open-bitcoin-node/src/storage.rs`
- `packages/open-bitcoin-cli/src/operator/service.rs`
- `packages/open-bitcoin-cli/src/operator/service/fake.rs`
- `packages/open-bitcoin-cli/src/operator/status.rs`
- `packages/open-bitcoin-cli/src/operator/status/service_status.rs`
- `packages/open-bitcoin-cli/src/operator/status/sync_state.rs`
- `packages/open-bitcoin-cli/src/operator/status/render.rs`
- `packages/open-bitcoin-cli/src/operator/dashboard/model.rs`
- `docs/operator/runtime-guide.md`
- `scripts/check-phase63-service-lifecycle.ts`

## Findings

### Pattern 1: Additive Shared Status Contract

`open-bitcoin-node::status::ServiceStatus` is already the shared service status
contract used by CLI JSON, human status, and dashboard projection. Phase 64
should add restart/resume evidence there as an additive field with serde defaults
so older status JSON remains readable.

Useful existing types:

- `FieldAvailability<T>` for explicit unavailable reasons.
- `ServiceLifecycleStatus` for Phase 63 labels.
- `SyncRecoveryCategory` for clean/unclean shutdown and storage-first recovery
  labels.
- `SyncProgress` and `SyncResourcePressure` for durable progress and in-flight
  counters.

### Pattern 2: Durable Metadata Is The Restart Evidence Source

`RuntimeMetadata` already records:

- `last_clean_shutdown`
- `maybe_last_recovery_action`
- `maybe_sync_state`
- `sync_control`

`DurableSyncRuntime::durable_sync_state_from_summary` already maps clean stopped
state to `clean_shutdown` and unclean recovering state to `unclean_shutdown`.
Storage recovery metadata already beats peer/network guidance. The service
restart/resume surface should reuse this data rather than invent a separate
restart ledger.

### Pattern 3: Status Collection Can Load Durable Runtime Metadata Once

`operator/status/sync_state.rs` currently exposes `durable_sync_state()` by
opening the selected datadir and returning `metadata.maybe_sync_state`. Phase 64
needs the same metadata plus `last_clean_shutdown`. Add a small helper returning
`RuntimeMetadata` from the selected datadir, then derive both durable sync state
and service restart/resume evidence from that source.

### Pattern 4: Service Manager Snapshot Does Not Need Real I/O In Tests

`FakeServiceManager` returns deterministic `ServiceStateSnapshot` values and
records start/stop/restart calls. Service restart/resume status tests should use
fake managers plus a temp Fjall datadir with `RuntimeMetadata`; no `launchctl`,
`systemctl`, or public network calls are needed.

### Pattern 5: Renderer And Dashboard Should Stay Thin

`operator/status/render.rs` and `operator/dashboard/model.rs` already render
`ServiceStatus` as compact key/value text. Phase 64 should add a small formatter
for restart/resume evidence and rows in the existing Service section rather than
creating a new dashboard section or a second status vocabulary.

## Pitfalls To Avoid

- Do not invoke real service-manager commands in default tests or `scripts/verify.sh`.
- Do not require fresh public-network progress after restart.
- Do not duplicate Phase 58 live-smoke schema inside status; status should expose
  compact durable evidence, while live-smoke keeps its `result.restartResumeEvidence`.
- Do not infer production readiness or uptime from service restart evidence.
- Do not add new Rust source files unless needed; updating existing files avoids
  new parity breadcrumb work.

## Recommended Plan Split

1. Add shared restart/resume status types and project them from durable metadata.
2. Render restart/resume evidence in human status and dashboard service rows.
3. Document Phase 64 operator review and guard it with a deterministic Bun
   checker wired into `scripts/verify.sh`.
