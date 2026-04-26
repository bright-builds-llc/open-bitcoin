# Phase 16: Metrics, Logs, and Sync Telemetry - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md - this log preserves the alternatives considered.

**Date:** 2026-04-26T21:50:05.423Z
**Phase:** 16-Metrics, Logs, and Sync Telemetry
**Mode:** Yolo
**Areas discussed:** Metrics history, Structured logs and retention, Status-facing warning and error access, Sync telemetry

---

## Metrics History

| Option | Description | Selected |
|--------|-------------|----------|
| Reuse Phase 13 metric contract | Implement bounded history around `MetricKind`, `MetricRetentionPolicy`, and `MetricSample` defaults already defined in `open-bitcoin-node`. | x |
| Add renderer-local metrics | Let future CLI/dashboard features define their own historical windows. | |
| Add external exporter first | Prioritize Prometheus/OpenTelemetry-style export before local status history. | |

**User's choice:** Reuse Phase 13 metric contract.
**Notes:** Auto-selected because Phase 13 explicitly locked the metric kinds and retention defaults, and OBS-03 requires bounded local history before dashboard rendering.

---

## Structured Logs and Retention

| Option | Description | Selected |
|--------|-------------|----------|
| Repo-owned structured writer and pruner | Keep structured records simple, implement deterministic retention pruning, and avoid broad dependency churn. | x |
| Add a logging stack dependency now | Pull in a full subscriber/appender stack before the runtime needs cross-process logging. | |
| Keep log retention as docs only | Leave implementation to later status/dashboard phases. | |

**User's choice:** Repo-owned structured writer and pruner.
**Notes:** Auto-selected because Phase 16 explicitly owns runtime writers/readers and `docs/architecture/operator-observability.md` says rolling file creation and retention pruning are separate obligations.

---

## Status-Facing Warning and Error Access

| Option | Description | Selected |
|--------|-------------|----------|
| Query bounded recent signals through status models | Feed `LogStatus`, `RecentLogSignal`, and `HealthSignal` so operators do not need raw log files. | x |
| Parse raw log files in each renderer | Let status/dashboard consumers implement their own parsing. | |
| Only expose a log file path | Defer warning/error summaries even though OBS-05 is in this phase. | |

**User's choice:** Query bounded recent signals through status models.
**Notes:** Auto-selected because OBS-05 requires recent warnings/errors without raw log inspection and Phase 13 established the shared status model.

---

## Sync Telemetry

| Option | Description | Selected |
|--------|-------------|----------|
| Extend existing sync summary/status evidence | Record peer outcomes, progress, retries, stalls, and typed failure signals through existing metrics/log/status contracts. | x |
| Add a separate sync telemetry subsystem | Create a parallel model just for sync details. | |
| Only keep current height samples | Preserve Phase 15's minimal metrics and defer bottleneck visibility. | |

**User's choice:** Extend existing sync summary/status evidence.
**Notes:** Auto-selected because SYNC-06 requires sync progress and bottlenecks to be visible through status, metrics history, logs, and dashboard panels without changing consensus or network behavior.

---

## the agent's Discretion

- Exact helper names, file splits, and DTO ordering are discretionary.
- The metrics persistence shape may be snapshot-oriented or append/prune-oriented if bounded retention and restart persistence are verified.
- The structured log file format may be line-delimited JSON or another simple deterministic format if status-facing queries remain stable.

## Deferred Ideas

- CLI status rendering belongs to Phase 17.
- Service lifecycle integration belongs to Phase 18.
- Dashboard graph rendering belongs to Phase 19.
- External observability export belongs to future OBS-06.

---

*Discussion log generated: 2026-04-26*
