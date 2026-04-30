---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 16-2026-04-26T21-50-05
generated_at: 2026-04-26T21:50:05.423Z
---

# Phase 16: Metrics, Logs, and Sync Telemetry - Context

**Gathered:** 2026-04-26
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Implement the runtime evidence layer promised by the Phase 13 observability contracts and needed by Phase 17 status output and Phase 19 dashboard work. This phase records bounded historical metrics, writes and prunes structured logs, exposes recent warnings/errors through status-facing APIs, and makes sync bottlenecks visible through metrics and log-derived health signals. It does not implement rich CLI status rendering, service lifecycle commands, Ratatui dashboard panels, migration flows, or external metrics export.

</domain>

<decisions>
## Implementation Decisions

### Metrics History
- **D-01:** Use the existing `MetricKind` contract as the canonical series list: sync height, header height, peer count, mempool transactions, wallet trusted balance sats, disk usage bytes, RPC health, and service restarts.
- **D-02:** Enforce bounded history in the node shell with the Phase 13 defaults: 30 second sampling interval, 2880 samples per series, and 86400 seconds max age.
- **D-03:** Store metrics through `FjallNodeStore` in the existing metrics namespace so runtime samples survive restart, while keeping pure consensus, chainstate, mempool, wallet, and protocol crates free of filesystem/database dependencies.
- **D-04:** Treat missing collectors as explicit unavailable evidence rather than silently dropping fields. Status-facing metrics projections should preserve retention policy and enabled-series metadata even when live values are unavailable.

### Structured Logs and Retention
- **D-05:** Implement structured runtime log records as serializable Open Bitcoin-owned data, not as renderer-local strings. Records must include at least level, message, timestamp, and source so status/dashboard consumers can query recent warnings/errors without opening raw files.
- **D-06:** Apply the Phase 13 retention contract separately from file creation: daily rotation, 14 files, 14 days, and 268435456 total retained bytes. Retention pruning must have deterministic tests for max-file, max-age, and byte-cap behavior.
- **D-07:** Avoid broad logging dependency churn unless the implementation needs it. A small repo-owned writer/pruner is acceptable when it preserves deterministic tests and avoids public-network or daemon requirements in default verification.

### Status-Facing Warning and Error Access
- **D-08:** Expose recent warning/error signals through the shared status model using the existing `LogStatus`, `RecentLogSignal`, and `HealthSignal` concepts instead of requiring callers to parse raw log files.
- **D-09:** Warning/error queries should be bounded, deterministic, and usable for stopped-node inspection when log files are present. Missing log paths should use explicit unavailable states with reasons.
- **D-10:** Keep health signals concise and operator-actionable. They should name the source (`sync`, `storage`, `logging`, `metrics`, etc.) and avoid marketing or dashboard-specific copy.

### Sync Telemetry
- **D-11:** Extend the Phase 15 sync runtime to record bottleneck evidence without changing consensus or network behavior: attempted/connected/failed peers, messages processed, headers received, blocks received, best header height, best block height, retry/stall/failure outcomes, and storage/network error signals.
- **D-12:** Persist sync telemetry through the same metrics/log/status contracts that later CLI and dashboard work consume. Do not create a separate sync-only telemetry model that status renderers must special-case.
- **D-13:** Default tests must remain hermetic. Sync telemetry coverage should use existing scripted transports, isolated temp stores, and deterministic timestamps; live-network smoke paths stay ignored and opt-in.

### the agent's Discretion
- Exact helper names, file splits, and DTO field ordering are discretionary if the final implementation preserves the contracts above and remains easy for status/dashboard consumers to reuse.
- The implementation may choose snapshot-style metric persistence or append/prune APIs first, as long as restart persistence and bounded retention are both testable.
- The implementation may keep log files as line-delimited JSON or another simple structured format, provided warning/error queries and retention pruning are deterministic and documented in code/tests.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase scope
- `.planning/ROADMAP.md` - Phase 16 goal, requirements, success criteria, and downstream Phase 17/19 dependencies.
- `.planning/REQUIREMENTS.md` - OBS-03, OBS-04, OBS-05, and SYNC-06 definitions.
- `.planning/PROJECT.md` - v1.1 operator runtime direction, parity constraints, functional-core boundary, and local operator-surface tone.
- `.planning/phases/13-operator-runtime-foundations/13-CONTEXT.md` - locked observability defaults, shared status model, and Phase 16 boundary.
- `.planning/phases/15-real-network-sync-loop/15-CONTEXT.md` - sync runtime ownership, hermetic test policy, and sync status/health signal decisions.

### Observability contracts
- `docs/architecture/operator-observability.md` - exact metrics and logging retention defaults plus Phase 16 responsibilities.
- `docs/architecture/status-snapshot.md` - shared status snapshot ownership, unavailable-field semantics, metrics/logs/health field ownership.
- `packages/open-bitcoin-node/src/metrics.rs` - `MetricKind`, `MetricRetentionPolicy`, `MetricSample`, and metrics status contracts.
- `packages/open-bitcoin-node/src/logging.rs` - log retention, log path, recent signal, and log status contracts.
- `packages/open-bitcoin-node/src/status.rs` - shared status snapshot, field availability, sync, peer, mempool, wallet, and health signal models.

### Runtime and storage integration
- `packages/open-bitcoin-node/src/sync.rs` - Phase 15 durable sync runtime and existing metric persistence hooks.
- `packages/open-bitcoin-node/src/sync/types.rs` - sync config, run summary, peer outcomes, status projections, and typed sync errors.
- `packages/open-bitcoin-node/src/sync/tests.rs` - deterministic scripted transport tests to extend for telemetry.
- `packages/open-bitcoin-node/src/storage.rs` - node-shell storage namespace and typed storage errors.
- `packages/open-bitcoin-node/src/storage/fjall_store.rs` - durable store metrics namespace and persistence adapter.
- `packages/open-bitcoin-node/src/storage/snapshot_codec.rs` - metrics snapshot DTO encoding/decoding.
- `docs/parity/source-breadcrumbs.json` - required source breadcrumb manifest for any new first-party Rust files under breadcrumb scope.

### Standards
- `AGENTS.md` - Repo-local workflow, parity, verification, and GSD rules.
- `AGENTS.bright-builds.md` - Bright Builds workflow and standards routing.
- `standards-overrides.md` - Local standards exceptions.
- `../coding-and-architecture-requirements/standards/core/architecture.md` - Functional core / imperative shell and typed boundary guidance.
- `../coding-and-architecture-requirements/standards/core/code-shape.md` - Early returns, optional naming, and file/function-size guidance.
- `../coding-and-architecture-requirements/standards/core/testing.md` - Unit test and Arrange/Act/Assert expectations.
- `../coding-and-architecture-requirements/standards/core/verification.md` - Sync-first and repo-native verification guidance.
- `../coding-and-architecture-requirements/standards/languages/rust.md` - Rust module, optional naming, invariant, and test guidance.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `MetricKind`, `MetricRetentionPolicy`, `MetricSample`, and `MetricsStatus` already define the serializable metric contract.
- `LogRetentionPolicy`, `LogPathStatus`, `RecentLogSignal`, and `LogStatus` already define the serializable log/status contract.
- `OpenBitcoinStatusSnapshot`, `FieldAvailability`, `HealthSignal`, `SyncStatus`, `PeerStatus`, `MempoolStatus`, and `WalletStatus` already provide the shared status surface.
- `FjallNodeStore::save_metrics_snapshot` and `load_metrics_snapshot` already persist a metrics placeholder snapshot in the node storage namespace.
- `DurableSyncRuntime` already records header height, sync height, and peer count samples during sync progress.

### Established Patterns
- Node-shell modules own side effects while pure crates stay data-in/data-out.
- Tests use deterministic timestamps, fake transports, and isolated temp directories for runtime behavior.
- New Rust files under first-party source paths need parity breadcrumbs and `docs/parity/source-breadcrumbs.json` entries.
- Existing contracts prefer explicit unavailable states with reasons over omitted live fields.

### Integration Points
- Metrics retention and history pruning should live in `open-bitcoin-node`, likely around `metrics.rs`, `storage/snapshot_codec.rs`, `storage/fjall_store.rs`, and sync runtime call sites.
- Structured log writer/query/pruning behavior should live in `open-bitcoin-node` and feed `LogStatus` and `HealthSignal` without requiring CLI/dashboard renderers to parse raw files.
- Sync telemetry should extend `SyncRunSummary`, `DurableSyncRuntime`, and status projections, then remain reusable by later CLI status and Ratatui dashboard phases.

</code_context>

<specifics>
## Specific Ideas

- Favor simple, bounded, deterministic runtime evidence over exporter-style observability. External metrics export is future scope.
- Keep operator-facing messages quiet and direct: describe what happened, where, and what the operator can inspect next.
- Treat metrics and logs as reusable evidence for later status and dashboard features, not decorative output.

</specifics>

<deferred>
## Deferred Ideas

- Rich `open-bitcoin status` human/JSON rendering belongs to Phase 17.
- Service restart collection from launchd/systemd belongs primarily to Phase 18; this phase can define and retain the metric kind and storage path.
- Ratatui dashboard graph rendering belongs to Phase 19.
- External observability export belongs to future requirement OBS-06.
- Real-sync benchmark reports belong to Phase 22.

</deferred>

---

*Phase: 16-metrics-logs-and-sync-telemetry*
*Context gathered: 2026-04-26*
