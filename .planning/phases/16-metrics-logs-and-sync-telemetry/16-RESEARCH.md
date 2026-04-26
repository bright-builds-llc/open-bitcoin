# Phase 16: Metrics, Logs, and Sync Telemetry - Research

**Researched:** 2026-04-26
**Domain:** Rust node-shell observability, bounded retention, structured logs, sync telemetry
**Confidence:** HIGH [VERIFIED: .planning/phases/16-metrics-logs-and-sync-telemetry/16-CONTEXT.md; packages/open-bitcoin-node/src]

<user_constraints>
## User Constraints (from CONTEXT.md)

Copied verbatim from `.planning/phases/16-metrics-logs-and-sync-telemetry/16-CONTEXT.md`. [VERIFIED: .planning/phases/16-metrics-logs-and-sync-telemetry/16-CONTEXT.md]

### Locked Decisions

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

### Deferred Ideas (OUT OF SCOPE)

## Deferred Ideas

- Rich `open-bitcoin status` human/JSON rendering belongs to Phase 17.
- Service restart collection from launchd/systemd belongs primarily to Phase 18; this phase can define and retain the metric kind and storage path.
- Ratatui dashboard graph rendering belongs to Phase 19.
- External observability export belongs to future requirement OBS-06.
- Real-sync benchmark reports belong to Phase 22.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| OBS-03 | The node records bounded historical metrics for sync height, header height, peer counts, mempool size, wallet balance summary, disk usage, RPC health, and service restarts. [VERIFIED: .planning/REQUIREMENTS.md] | Use `MetricKind::ALL`, `MetricRetentionPolicy::default()`, `MetricSample`, and `FjallNodeStore::save_metrics_snapshot/load_metrics_snapshot`; replace current overwrite behavior with append-and-prune snapshot semantics. [VERIFIED: packages/open-bitcoin-node/src/metrics.rs; packages/open-bitcoin-node/src/storage/fjall_store.rs; packages/open-bitcoin-node/src/sync.rs] |
| OBS-04 | The runtime writes structured logs with rotation, retention, and status-visible log locations. [VERIFIED: .planning/REQUIREMENTS.md] | Extend `logging.rs` from contract-only types into serializable `StructuredLogRecord`, writer, reader, and pure retention planner while preserving `LogRetentionPolicy` defaults. [VERIFIED: packages/open-bitcoin-node/src/logging.rs; docs/architecture/operator-observability.md] |
| OBS-05 | Operators can inspect recent warnings or errors without opening raw log files manually. [VERIFIED: .planning/REQUIREMENTS.md] | Implement bounded recent signal queries that deserialize managed log records into `RecentLogSignal`, then project concise `HealthSignal` values through the shared status model. [VERIFIED: packages/open-bitcoin-node/src/logging.rs; packages/open-bitcoin-node/src/status.rs; docs/architecture/status-snapshot.md] |
| SYNC-06 | Sync progress and bottlenecks are visible through status, metrics history, logs, and dashboard panels. [VERIFIED: .planning/REQUIREMENTS.md] | Extend `DurableSyncRuntime`, `SyncRunSummary`, `PeerSyncOutcome`, and scripted sync tests so attempted/connected/failed peers, retry/stall outcomes, headers/blocks/messages, and storage/network failures feed metrics, logs, and status without a sync-only telemetry model. [VERIFIED: packages/open-bitcoin-node/src/sync.rs; packages/open-bitcoin-node/src/sync/types.rs; packages/open-bitcoin-node/src/sync/tests.rs; .planning/phases/16-metrics-logs-and-sync-telemetry/16-CONTEXT.md] |
</phase_requirements>

## Summary

Phase 16 should implement a node-shell evidence layer, not a new observability platform: the required metrics, log retention defaults, status fields, and sync summary types already exist in `open-bitcoin-node`. [VERIFIED: docs/architecture/operator-observability.md; docs/architecture/status-snapshot.md; packages/open-bitcoin-node/src/metrics.rs; packages/open-bitcoin-node/src/logging.rs; packages/open-bitcoin-node/src/status.rs; packages/open-bitcoin-node/src/sync/types.rs]

The primary technical work is to add pure retention/planning functions plus thin side-effect adapters: metrics append/prune via `FjallNodeStore`, structured JSONL log write/read/prune via `std::fs`, and sync-runtime calls that emit the same shared metrics/log/status evidence later CLI and dashboard phases consume. [VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store.rs; packages/open-bitcoin-node/src/storage/snapshot_codec.rs; packages/open-bitcoin-node/src/sync.rs; .planning/phases/16-metrics-logs-and-sync-telemetry/16-CONTEXT.md]

The current sync runtime persists only three metric samples per snapshot write and overwrites the metrics snapshot each time, so OBS-03 requires replacing that with bounded history semantics while preserving restart persistence. [VERIFIED: packages/open-bitcoin-node/src/sync.rs; packages/open-bitcoin-node/src/storage/fjall_store.rs]

**Primary recommendation:** Use snapshot-style metrics history first, line-delimited JSON structured logs, pure retention planners, and existing `SyncRunSummary`/`LogStatus`/`MetricsStatus` contracts; do not add `tracing`, external exporters, or dashboard-specific models in Phase 16. [VERIFIED: .planning/phases/16-metrics-logs-and-sync-telemetry/16-CONTEXT.md; packages/open-bitcoin-node/Cargo.toml; cargo tree --manifest-path packages/Cargo.toml -p open-bitcoin-node --depth 1]

## Project Constraints (from AGENTS.md)

- Read `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md`, and relevant canonical standards before planning or implementation. [VERIFIED: AGENTS.md; AGENTS.bright-builds.md; standards-overrides.md]
- Keep functional core / imperative shell boundaries: pure business logic stays data-in/data-out, while filesystem, database, clocks, sockets, and other effects stay in adapters. [VERIFIED: AGENTS.md; ../coding-and-architecture-requirements/standards/core/architecture.md]
- Keep pure consensus, chainstate, mempool, wallet, and protocol crates free of filesystem/database/logging side effects. [VERIFIED: AGENTS.md; .planning/phases/16-metrics-logs-and-sync-telemetry/16-CONTEXT.md]
- Use `rust-toolchain.toml` as the Rust source of truth; the pinned local toolchain is Rust 1.94.1. [VERIFIED: AGENTS.md; rust-toolchain.toml; rustc --version]
- Use `bash scripts/verify.sh` as the repo-native verification contract for first-party code, including the Bazel smoke build. [VERIFIED: AGENTS.md; scripts/verify.sh]
- Use Bun for repo-owned higher-level automation scripts and keep Bash for thin orchestration wrappers. [VERIFIED: AGENTS.md]
- New first-party Rust source or test files under `packages/open-bitcoin-*/src` or `packages/open-bitcoin-*/tests` need parity breadcrumb blocks and `docs/parity/source-breadcrumbs.json` entries; `none` is allowed only when no defensible Knots anchor exists. [VERIFIED: AGENTS.md; docs/parity/source-breadcrumbs.json]
- Prefer `foo.rs` plus `foo/` over `foo/mod.rs` for new or restructured multi-file Rust modules. [VERIFIED: ../coding-and-architecture-requirements/standards/languages/rust.md]
- Use early returns, `let...else` for guard-style extraction, and `maybe_` names for optional internal values. [VERIFIED: ../coding-and-architecture-requirements/standards/core/code-shape.md; ../coding-and-architecture-requirements/standards/languages/rust.md]
- Unit tests for pure/business logic must be focused on one concern and should clearly show Arrange, Act, Assert. [VERIFIED: ../coding-and-architecture-requirements/standards/core/testing.md]
- The root project skill directories `.claude/skills/` and `.agents/skills/` were not present in this checkout, so no repo-specific skill patterns apply. [VERIFIED: find .claude/skills .agents/skills -maxdepth 2 -name SKILL.md]

## Standard Stack

### Core

| Library / Component | Version | Purpose | Why Standard |
|---------------------|---------|---------|--------------|
| `open-bitcoin-node` | 0.1.0 | Node-shell crate that owns storage, sync, metrics, logging, and status adapters. | Phase 13/15 contracts and current exports place these side effects in this crate. [VERIFIED: packages/open-bitcoin-node/src/lib.rs; .planning/phases/16-metrics-logs-and-sync-telemetry/16-CONTEXT.md] |
| `fjall` | 3.1.4; published 2026-04-14; crates.io max version 3.1.4 | Durable key-value storage for metrics snapshots in the existing metrics namespace. | Phase 13 selected `fjall`, Phase 14 implemented `FjallNodeStore`, and crates.io confirms the repo uses the current published version. [VERIFIED: .planning/phases/13-operator-runtime-foundations/13-CONTEXT.md; packages/open-bitcoin-node/Cargo.toml; crates.io API] |
| `serde` | 1.0.228; published 2025-09-27; crates.io max version 1.0.228 | Derive serialization for metric, log, status, and storage DTOs. | Existing contracts derive serde and crates.io confirms the repo uses the current published version. [VERIFIED: packages/open-bitcoin-node/Cargo.toml; packages/open-bitcoin-node/src/metrics.rs; packages/open-bitcoin-node/src/logging.rs; crates.io API] |
| `serde_json` | 1.0.149; published 2026-01-06; crates.io max version 1.0.149 | JSON storage DTOs and line-delimited structured log record encoding/decoding. | Existing storage snapshots already use `serde_json::to_vec_pretty/from_slice`, and crates.io confirms the repo uses the current published version. [VERIFIED: packages/open-bitcoin-node/src/storage/snapshot_codec.rs; serde_json 1.0.149 local source; crates.io API] |
| Rust standard library filesystem APIs | Rust 1.94.1 | Managed log file append, directory listing, metadata byte sizes, and deletion. | Phase 16 needs local file effects, and the locked decision permits a small repo-owned writer/pruner without new logging dependency churn. [VERIFIED: rust-toolchain.toml; .planning/phases/16-metrics-logs-and-sync-telemetry/16-CONTEXT.md] |

### Supporting

| Component | Version | Purpose | When to Use |
|-----------|---------|---------|-------------|
| `ScriptedTransport` test pattern | First-party test helper | Hermetic sync telemetry tests without public-network access. | Extend `packages/open-bitcoin-node/src/sync/tests.rs` for retry, stall, metrics, and log-signal assertions. [VERIFIED: packages/open-bitcoin-node/src/sync/tests.rs] |
| `MetricRetentionPolicy::default()` | 30s interval, 2880 samples, 86400s max age | Canonical bounded history policy. | Use in append/prune logic and `MetricsStatus` projections. [VERIFIED: packages/open-bitcoin-node/src/metrics.rs; docs/architecture/operator-observability.md] |
| `LogRetentionPolicy::default()` | daily rotation, 14 files, 14 days, 268435456 bytes | Canonical structured log retention policy. | Use in log file naming, retention planning, pruning tests, and status projections. [VERIFIED: packages/open-bitcoin-node/src/logging.rs; docs/architecture/operator-observability.md] |
| `OpenBitcoinStatusSnapshot` models | First-party data contracts | Shared status, logs, metrics, sync, peers, wallet, mempool, service, and health signals. | Map collectors into these models instead of inventing CLI/dashboard local state. [VERIFIED: packages/open-bitcoin-node/src/status.rs; docs/architecture/status-snapshot.md] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Repo-owned JSONL writer/pruner | `tracing`, `tracing-subscriber`, `tracing-appender` | Do not add for Phase 16 because the repo has no current `tracing` dependency in `open-bitcoin-node`, and D-07 explicitly says to avoid broad logging dependency churn unless needed. [VERIFIED: packages/open-bitcoin-node/Cargo.toml; cargo tree --manifest-path packages/Cargo.toml -p open-bitcoin-node --depth 1; .planning/phases/16-metrics-logs-and-sync-telemetry/16-CONTEXT.md] |
| Snapshot-style metric history | Per-sample Fjall keys and range scans | Use snapshot-style first because D-03/D-13 allow it, the bounded maximum is 8 series * 2880 samples, and Fjall docs warn that full/unbounded iterators may scan many items. [VERIFIED: .planning/phases/16-metrics-logs-and-sync-telemetry/16-CONTEXT.md; packages/open-bitcoin-node/src/metrics.rs; fjall 3.1.4 local source] |
| Unix-day rotation bucket filenames | `time`, `chrono`, or `jiff` | Use UTC Unix day buckets for Phase 16 unless human calendar filenames become a hard requirement; this avoids a new dependency and avoids hand-rolling Gregorian date formatting. [VERIFIED: AGENTS.md dependency policy; .planning/phases/16-metrics-logs-and-sync-telemetry/16-CONTEXT.md] |
| Shared status/log/metric contracts | Sync-only telemetry DTOs | Do not create a separate sync telemetry model because D-12 requires sync evidence to flow through shared metrics/log/status contracts. [VERIFIED: .planning/phases/16-metrics-logs-and-sync-telemetry/16-CONTEXT.md; packages/open-bitcoin-node/src/status.rs] |

**Installation:**

```bash
# No new crates are recommended for Phase 16.
# Existing workspace dependencies are already declared in packages/open-bitcoin-node/Cargo.toml.
```

**Version verification:**

| Package | Current Repo Version | Registry Max | Publish Date | Verification |
|---------|----------------------|--------------|--------------|--------------|
| `fjall` | 3.1.4 | 3.1.4 | 2026-04-14T16:33:32Z | `cargo search fjall --limit 1`; crates.io API. [VERIFIED: crates.io API; cargo search] |
| `serde` | 1.0.228 | 1.0.228 | 2025-09-27T16:51:35Z | `cargo search serde --limit 1`; crates.io API. [VERIFIED: crates.io API; cargo search] |
| `serde_json` | 1.0.149 | 1.0.149 | 2026-01-06T16:23:34Z | `cargo search serde_json --limit 1`; crates.io API. [VERIFIED: crates.io API; cargo search] |

## Architecture Patterns

### Recommended Project Structure

```text
packages/open-bitcoin-node/src/
|-- metrics.rs                  # metric kinds, samples, retention policy, pure append/prune helpers
|-- metrics/
|   `-- tests.rs                # deterministic per-series retention tests if metrics.rs grows
|-- logging.rs                  # log contracts, structured record type, module exports
|-- logging/
|   |-- writer.rs               # thin filesystem append/read adapter for managed JSONL files
|   |-- prune.rs                # pure retention planner plus adapter delete executor
|   `-- tests.rs                # deterministic rotation/query/prune tests
|-- sync.rs                     # calls metric/log adapters from the durable sync runtime shell
|-- sync/types.rs               # sync summary/status DTO extensions
`-- storage/
    |-- snapshot_codec.rs       # metrics snapshot DTO remains serde-versioned
    `-- fjall_store.rs          # append/load metrics history helpers around the metrics keyspace
```

This structure follows the repo's existing `foo.rs` plus `foo/` module style for multi-file Rust modules. [VERIFIED: ../coding-and-architecture-requirements/standards/languages/rust.md; packages/open-bitcoin-node/src/sync.rs; packages/open-bitcoin-node/src/storage.rs]

New files under `packages/open-bitcoin-node/src` must add top-of-file parity breadcrumbs and update `docs/parity/source-breadcrumbs.json`; Open Bitcoin-only metrics/logging files can use the existing `none` breadcrumb group pattern. [VERIFIED: AGENTS.md; docs/parity/source-breadcrumbs.json]

### Pattern 1: Pure Metrics Append and Prune

**What:** Keep retention decisions pure by transforming `(existing samples, new samples, policy, now)` into a bounded sample vector before any Fjall write. [VERIFIED: ../coding-and-architecture-requirements/standards/core/architecture.md; packages/open-bitcoin-node/src/metrics.rs]

**When to use:** Use for OBS-03 history updates from sync, mempool, wallet, disk, RPC, and service collectors. [VERIFIED: .planning/REQUIREMENTS.md; .planning/phases/16-metrics-logs-and-sync-telemetry/16-CONTEXT.md]

**Example:**

```rust
pub fn append_and_prune_metrics(
    mut samples: Vec<MetricSample>,
    new_samples: impl IntoIterator<Item = MetricSample>,
    policy: MetricRetentionPolicy,
    now_unix_seconds: u64,
) -> Vec<MetricSample> {
    let min_timestamp = now_unix_seconds.saturating_sub(policy.max_age_seconds);
    samples.extend(new_samples);
    samples.retain(|sample| sample.timestamp_unix_seconds >= min_timestamp);
    samples.sort_by_key(|sample| (sample.kind.as_str(), sample.timestamp_unix_seconds));

    let mut retained = Vec::with_capacity(samples.len());
    for kind in MetricKind::ALL {
        let mut per_kind = samples
            .iter()
            .filter(|sample| sample.kind == kind)
            .cloned()
            .collect::<Vec<_>>();
        let keep_from = per_kind.len().saturating_sub(policy.max_samples_per_series);
        retained.extend(per_kind.drain(keep_from..));
    }

    retained
}
```

Source: Existing `MetricKind`, `MetricRetentionPolicy`, and `MetricSample` contracts. [VERIFIED: packages/open-bitcoin-node/src/metrics.rs]

### Pattern 2: Snapshot-Style Metrics Persistence

**What:** Load the current `MetricsStorageSnapshot`, call the pure append/prune helper, and save the bounded snapshot through `FjallNodeStore`. [VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store.rs; packages/open-bitcoin-node/src/storage/snapshot_codec.rs]

**When to use:** Use as the first implementation because D-03 allows storage through the existing metrics namespace and D-discretion allows snapshot-style persistence. [VERIFIED: .planning/phases/16-metrics-logs-and-sync-telemetry/16-CONTEXT.md]

**Example:**

```rust
pub fn append_metric_samples(
    &self,
    new_samples: &[MetricSample],
    policy: MetricRetentionPolicy,
    now_unix_seconds: u64,
    mode: PersistMode,
) -> Result<MetricsStorageSnapshot, StorageError> {
    let existing = self.load_metrics_snapshot()?.unwrap_or(MetricsStorageSnapshot {
        samples: Vec::new(),
    });
    let samples = append_and_prune_metrics(
        existing.samples,
        new_samples.iter().cloned(),
        policy,
        now_unix_seconds,
    );
    let snapshot = MetricsStorageSnapshot { samples };
    self.save_metrics_snapshot(&snapshot, mode)?;
    Ok(snapshot)
}
```

Source: Existing `load_metrics_snapshot` and `save_metrics_snapshot` methods. [VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store.rs]

### Pattern 3: Structured Log Records as Owned Data

**What:** Add a serializable `StructuredLogRecord` with at least level, source, message, and timestamp, then write one JSON object per line. [VERIFIED: .planning/phases/16-metrics-logs-and-sync-telemetry/16-CONTEXT.md; serde_json 1.0.149 local source]

**When to use:** Use for runtime warnings/errors and sync/storage/metrics/logging evidence that later status and dashboard consumers query. [VERIFIED: packages/open-bitcoin-node/src/logging.rs; packages/open-bitcoin-node/src/status.rs]

**Example:**

```rust
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct StructuredLogRecord {
    pub level: StructuredLogLevel,
    pub source: String,
    pub message: String,
    pub timestamp_unix_seconds: u64,
}

impl From<&StructuredLogRecord> for RecentLogSignal {
    fn from(record: &StructuredLogRecord) -> Self {
        Self {
            level: record.level,
            message: record.message.clone(),
            timestamp_unix_seconds: record.timestamp_unix_seconds,
        }
    }
}
```

Source: Existing `StructuredLogLevel`, `RecentLogSignal`, and serde contracts. [VERIFIED: packages/open-bitcoin-node/src/logging.rs; serde_json 1.0.149 local source]

### Pattern 4: Pure Log Retention Planner, Thin Delete Adapter

**What:** Represent each managed log file as data, sort deterministically, and produce prune actions for max-age, max-file, and max-total-byte constraints before calling `std::fs::remove_file`. [VERIFIED: docs/architecture/operator-observability.md; ../coding-and-architecture-requirements/standards/core/architecture.md]

**When to use:** Use after each rotation/write and in stopped-node inspection when pruning is explicitly invoked. [VERIFIED: .planning/phases/16-metrics-logs-and-sync-telemetry/16-CONTEXT.md]

**Example:**

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManagedLogFile {
    pub path: std::path::PathBuf,
    pub unix_day: u64,
    pub bytes: u64,
}

pub fn plan_log_prune(
    mut files: Vec<ManagedLogFile>,
    policy: LogRetentionPolicy,
    now_unix_seconds: u64,
) -> Vec<std::path::PathBuf> {
    let current_day = now_unix_seconds / 86_400;
    let min_day = current_day.saturating_sub(u64::from(policy.max_age_days));
    files.sort_by_key(|file| (file.unix_day, file.path.clone()));

    let mut delete = files
        .iter()
        .filter(|file| file.unix_day < min_day)
        .map(|file| file.path.clone())
        .collect::<Vec<_>>();

    let mut survivors = files
        .into_iter()
        .filter(|file| file.unix_day >= min_day)
        .collect::<Vec<_>>();
    while survivors.len() > usize::from(policy.max_files) {
        delete.push(survivors.remove(0).path);
    }

    while survivors.iter().map(|file| file.bytes).sum::<u64>() > policy.max_total_bytes {
        let oldest = survivors.remove(0);
        delete.push(oldest.path);
    }

    delete
}
```

Source: Log retention defaults and Bright Builds functional-core guidance. [VERIFIED: packages/open-bitcoin-node/src/logging.rs; docs/architecture/operator-observability.md; ../coding-and-architecture-requirements/standards/core/architecture.md]

### Pattern 5: Sync Telemetry Through Existing Contracts

**What:** Extend `SyncRunSummary` and `PeerSyncOutcome` only where needed, then emit metrics/logs/health signals from `DurableSyncRuntime` instead of creating a new sync-only status surface. [VERIFIED: packages/open-bitcoin-node/src/sync.rs; packages/open-bitcoin-node/src/sync/types.rs; .planning/phases/16-metrics-logs-and-sync-telemetry/16-CONTEXT.md]

**When to use:** Use in `record_outcome`, `persist_progress`, and `sync_until_idle` so retry, stall, failure, storage, and network evidence is captured without changing protocol behavior. [VERIFIED: packages/open-bitcoin-node/src/sync.rs]

**Example:**

```rust
fn sync_metric_samples(summary: &SyncRunSummary, timestamp: u64) -> [MetricSample; 3] {
    [
        MetricSample::new(MetricKind::HeaderHeight, summary.best_header_height as f64, timestamp),
        MetricSample::new(MetricKind::SyncHeight, summary.best_block_height as f64, timestamp),
        MetricSample::new(MetricKind::PeerCount, summary.connected_peers as f64, timestamp),
    ]
}
```

Source: Existing sync metrics sample construction. [VERIFIED: packages/open-bitcoin-node/src/sync.rs]

### Anti-Patterns to Avoid

- **Metrics overwrite-only persistence:** Existing `persist_metrics` writes a fresh three-sample snapshot; Phase 16 must append and prune bounded history instead. [VERIFIED: packages/open-bitcoin-node/src/sync.rs]
- **Renderer-local log strings:** Logs must be serializable Open Bitcoin-owned records, not CLI/TUI display strings. [VERIFIED: .planning/phases/16-metrics-logs-and-sync-telemetry/16-CONTEXT.md]
- **Rotation-as-retention:** Creating a daily file does not satisfy max-file, max-age, or byte-cap pruning. [VERIFIED: docs/architecture/operator-observability.md]
- **Sync-only telemetry model:** D-12 explicitly forbids a separate sync-only model that status renderers must special-case. [VERIFIED: .planning/phases/16-metrics-logs-and-sync-telemetry/16-CONTEXT.md]
- **Wall-clock-only tests:** Default sync telemetry tests must use deterministic timestamps, scripted transports, and isolated temp stores. [VERIFIED: .planning/phases/16-metrics-logs-and-sync-telemetry/16-CONTEXT.md; packages/open-bitcoin-node/src/sync/tests.rs]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Durable metrics storage | New embedded database, ad hoc files, or per-renderer cache | `FjallNodeStore` metrics namespace | Phase 13/14 selected and implemented this storage adapter. [VERIFIED: .planning/phases/13-operator-runtime-foundations/13-CONTEXT.md; packages/open-bitcoin-node/src/storage/fjall_store.rs] |
| JSON serialization | Manual escaping, string concatenation, or custom parsers | `serde` derives and `serde_json` encode/decode | Existing DTOs already use serde, and serde_json exposes fallible `to_vec`, `to_vec_pretty`, `from_slice`, and `from_str` APIs. [VERIFIED: packages/open-bitcoin-node/src/storage/snapshot_codec.rs; serde_json 1.0.149 local source] |
| Status model | CLI/dashboard-local status structs | `OpenBitcoinStatusSnapshot`, `LogStatus`, `MetricsStatus`, `HealthSignal` | The architecture docs designate one shared status model for CLI, service, dashboard, and support paths. [VERIFIED: docs/architecture/status-snapshot.md; packages/open-bitcoin-node/src/status.rs] |
| Sync network simulation | Real sockets or public-network tests in default verification | Existing `ScriptedTransport` and ignored live smoke pattern | Default verification must remain hermetic, and sync tests already use scripted transports and temp stores. [VERIFIED: .planning/phases/15-real-network-sync-loop/15-CONTEXT.md; packages/open-bitcoin-node/src/sync/tests.rs] |
| Log retention behavior | Inline deletion loops coupled to file writes | Pure `plan_log_prune` plus a thin delete executor | Pure planning gives deterministic tests for max-file, max-age, and byte-cap behavior. [VERIFIED: docs/architecture/operator-observability.md; ../coding-and-architecture-requirements/standards/core/architecture.md] |
| Calendar date formatting | Home-grown Gregorian calendar conversion | Unix-day rotation buckets for Phase 16 | Daily retention only needs a stable day bucket; adding calendar formatting is unnecessary unless a later operator surface requires it. [VERIFIED: .planning/phases/16-metrics-logs-and-sync-telemetry/16-CONTEXT.md; AGENTS.md dependency policy] |
| External observability export | Prometheus/OpenTelemetry exporters | No exporter in Phase 16 | External observability export is future OBS-06 scope. [VERIFIED: .planning/REQUIREMENTS.md; .planning/phases/16-metrics-logs-and-sync-telemetry/16-CONTEXT.md] |

**Key insight:** The hard part is not collecting a few numbers; it is keeping retention bounded, restart-persistent, queryable when stopped, and shared by later status/dashboard consumers without leaking side effects into pure crates. [VERIFIED: .planning/phases/16-metrics-logs-and-sync-telemetry/16-CONTEXT.md; docs/architecture/status-snapshot.md; ../coding-and-architecture-requirements/standards/core/architecture.md]

## Common Pitfalls

### Pitfall 1: Metrics Snapshot Overwrite Instead of History
**What goes wrong:** Only the latest sync height/header height/peer count samples survive because each write replaces the entire metrics snapshot. [VERIFIED: packages/open-bitcoin-node/src/sync.rs]
**Why it happens:** `DurableSyncRuntime::persist_metrics` currently constructs a fresh `MetricsStorageSnapshot` with three samples and calls `save_metrics_snapshot`. [VERIFIED: packages/open-bitcoin-node/src/sync.rs]
**How to avoid:** Implement `append_metric_samples` that loads existing samples, applies `append_and_prune_metrics`, and saves the bounded snapshot. [VERIFIED: packages/open-bitcoin-node/src/storage/fjall_store.rs; packages/open-bitcoin-node/src/metrics.rs]
**Warning signs:** Tests assert only that a `MetricKind::SyncHeight` sample exists rather than asserting multiple timestamps survive restart. [VERIFIED: packages/open-bitcoin-node/src/sync/tests.rs]

### Pitfall 2: Rotation Without Retention
**What goes wrong:** Daily files accumulate forever or byte caps are ignored. [VERIFIED: docs/architecture/operator-observability.md]
**Why it happens:** File creation and retention pruning are separate responsibilities in the Phase 13 contract. [VERIFIED: docs/architecture/operator-observability.md]
**How to avoid:** Test max-file, max-age, and max-total-byte pruning independently with deterministic file metadata. [VERIFIED: .planning/phases/16-metrics-logs-and-sync-telemetry/16-CONTEXT.md]
**Warning signs:** A test creates two rotated files but never proves old files are removed. [VERIFIED: docs/architecture/operator-observability.md]

### Pitfall 3: Missing Collectors Disappear
**What goes wrong:** Mempool, wallet, disk, RPC, or service metrics are absent with no status-visible reason. [VERIFIED: .planning/phases/16-metrics-logs-and-sync-telemetry/16-CONTEXT.md]
**Why it happens:** `MetricSample` is numeric-only, so unavailable collectors cannot be represented as fake samples without corrupting history. [VERIFIED: packages/open-bitcoin-node/src/metrics.rs]
**How to avoid:** Keep `MetricKind::ALL` in `MetricsStatus.enabled_series`, keep retention visible, and emit explicit unavailable health/status evidence when collectors are not wired yet. [VERIFIED: packages/open-bitcoin-node/src/metrics.rs; packages/open-bitcoin-node/src/status.rs; .planning/phases/16-metrics-logs-and-sync-telemetry/16-CONTEXT.md]
**Warning signs:** A collector returns an empty `Vec<MetricSample>` with no `HealthSignal` or unavailable reason. [VERIFIED: .planning/phases/16-metrics-logs-and-sync-telemetry/16-CONTEXT.md]

### Pitfall 4: Stalled or Retried Peers Look Healthy
**What goes wrong:** A peer that stalls or retries may count as connected without a warning/error path. [VERIFIED: packages/open-bitcoin-node/src/sync.rs; packages/open-bitcoin-node/src/sync/types.rs]
**Why it happens:** Current `record_outcome` increments connected peers for any state except `Failed`, and final connect failures are recorded with `attempts: 1`. [VERIFIED: packages/open-bitcoin-node/src/sync.rs]
**How to avoid:** Preserve actual retry attempts and emit concise log/health evidence for `Stalled` and final `Failed` outcomes. [VERIFIED: .planning/phases/16-metrics-logs-and-sync-telemetry/16-CONTEXT.md]
**Warning signs:** Tests assert `failed_peers` but do not assert retry attempt counts, stall log records, or health signal source. [VERIFIED: packages/open-bitcoin-node/src/sync/tests.rs]

### Pitfall 5: Querying Raw Logs in Later Renderers
**What goes wrong:** Phase 17 or Phase 19 would need to parse log files directly and duplicate retention/query behavior. [VERIFIED: .planning/phases/16-metrics-logs-and-sync-telemetry/16-CONTEXT.md]
**Why it happens:** Log querying is delayed into display code instead of owned by `open-bitcoin-node`. [VERIFIED: docs/architecture/status-snapshot.md]
**How to avoid:** Provide a bounded `recent_log_signals` reader that returns `LogStatus`/`RecentLogSignal` and maps warnings/errors to `HealthSignal`. [VERIFIED: packages/open-bitcoin-node/src/logging.rs; packages/open-bitcoin-node/src/status.rs]
**Warning signs:** CLI or dashboard plans mention opening raw `.jsonl` files directly. [VERIFIED: .planning/phases/16-metrics-logs-and-sync-telemetry/16-CONTEXT.md]

## Code Examples

Verified patterns from project and dependency sources:

### Append Metrics Without Losing Restart History

```rust
let timestamp = u64::try_from(timestamp).unwrap_or(0);
let samples = sync_metric_samples(summary, timestamp);
let _snapshot = self.store.append_metric_samples(
    &samples,
    MetricRetentionPolicy::default(),
    timestamp,
    self.config.persist_mode,
)?;
```

Source: Existing sync timestamp conversion and store persistence pattern. [VERIFIED: packages/open-bitcoin-node/src/sync.rs; packages/open-bitcoin-node/src/storage/fjall_store.rs]

### Encode One Structured Log Record Per Line

```rust
let mut encoded = serde_json::to_vec(record)?;
encoded.push(b'\n');
file.write_all(&encoded)?;
```

Source: `serde_json::to_vec` is a fallible serializer in serde_json 1.0.149, and Phase 16 may use line-delimited JSON if deterministic and documented. [VERIFIED: serde_json 1.0.149 local source; .planning/phases/16-metrics-logs-and-sync-telemetry/16-CONTEXT.md]

### Map Logs to Status-Facing Signals

```rust
fn recent_warning_or_error(record: &StructuredLogRecord) -> Option<RecentLogSignal> {
    if !matches!(record.level, StructuredLogLevel::Warn | StructuredLogLevel::Error) {
        return None;
    }

    Some(RecentLogSignal::from(record))
}
```

Source: Existing `StructuredLogLevel` and `RecentLogSignal` contracts. [VERIFIED: packages/open-bitcoin-node/src/logging.rs]

### Keep Tests Hermetic

```rust
let mut transport = ScriptedTransport::new(vec![script]);
let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
let summary = runtime.sync_once(&mut transport, 1_777_225_022).expect("sync");
```

Source: Existing sync tests use scripted transport, isolated temp store paths, and deterministic timestamps. [VERIFIED: packages/open-bitcoin-node/src/sync/tests.rs]

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Phase 13 contract-only metrics/logging | Phase 16 runtime writers/readers/pruners in `open-bitcoin-node` | Phase 16 scope, 2026-04-26 context | Planner should create implementation tasks, not more contract-only docs. [VERIFIED: docs/architecture/operator-observability.md; .planning/phases/16-metrics-logs-and-sync-telemetry/16-CONTEXT.md] |
| Overwrite metrics snapshot | Append and prune bounded metrics history | Required by OBS-03 in Phase 16 | Tests must prove multiple timestamps survive restart and caps apply per series. [VERIFIED: .planning/REQUIREMENTS.md; packages/open-bitcoin-node/src/sync.rs] |
| Raw log file inspection by humans/renderers | Bounded status-facing `RecentLogSignal` and `HealthSignal` queries | Required by OBS-05 in Phase 16 | CLI/dashboard consumers should not parse raw files. [VERIFIED: .planning/REQUIREMENTS.md; packages/open-bitcoin-node/src/logging.rs; packages/open-bitcoin-node/src/status.rs] |
| Sync summary only | Sync telemetry emitted through metrics/logs/status | Required by SYNC-06 in Phase 16 | Dashboard and status later consume shared evidence without sync-only special cases. [VERIFIED: .planning/REQUIREMENTS.md; .planning/phases/16-metrics-logs-and-sync-telemetry/16-CONTEXT.md] |
| External observability exporter | No exporter in Phase 16 | OBS-06 is future scope | Do not add Prometheus/OpenTelemetry tasks in this phase. [VERIFIED: .planning/REQUIREMENTS.md] |

**Deprecated/outdated:**
- Treating `LogStatus` and `MetricsStatus` as placeholders is outdated for Phase 16; they become runtime-backed status projections in this phase. [VERIFIED: docs/architecture/operator-observability.md; packages/open-bitcoin-node/src/logging.rs; packages/open-bitcoin-node/src/metrics.rs]
- Relying on public Bitcoin network tests for telemetry is out of scope for default verification; the existing live smoke test is ignored and opt-in. [VERIFIED: .planning/phases/15-real-network-sync-loop/15-CONTEXT.md; packages/open-bitcoin-node/src/sync/tests.rs]

## Assumptions Log

All claims in this research were verified or cited in this session; no assumed claims were used. [VERIFIED: this research session tool outputs]

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| - | None | - | - |

## Open Questions (RESOLVED)

1. **RESOLVED: Log filenames use Unix-day buckets in Phase 16.** [VERIFIED: .planning/phases/16-metrics-logs-and-sync-telemetry/16-CONTEXT.md; AGENTS.md]
   - What we know: Phase 16 requires daily rotation but does not require human calendar filenames. [VERIFIED: docs/architecture/operator-observability.md]
   - Resolution: Managed log files must use the `open-bitcoin-runtime-<unix_day>.jsonl` bucket policy in Phase 16. Phase 17 may render friendly calendar dates from timestamps in status output, but Phase 16 must not add a calendar-formatting dependency or hand-roll Gregorian date formatting. [VERIFIED: AGENTS.md dependency policy; .planning/phases/16-metrics-logs-and-sync-telemetry/16-CONTEXT.md]

2. **RESOLVED: Per-series availability stays metadata-plus-health evidence in Phase 16.** [VERIFIED: packages/open-bitcoin-node/src/metrics.rs; packages/open-bitcoin-node/src/status.rs]
   - What we know: `MetricsStatus` has global availability, retention, and enabled-series metadata, while `MetricSample` is numeric-only. [VERIFIED: packages/open-bitcoin-node/src/metrics.rs]
   - Resolution: For Phase 16, keep `MetricKind::ALL` in `MetricsStatus.enabled_series`, avoid fake numeric samples for unavailable collectors, and expose missing collectors through `MetricsStatus::unavailable(...)` plus concise `HealthSignal` reasons where runtime/status projections need operator-visible evidence. Later configurable collectors can add per-series availability fields only in a future phase with a separate status-contract decision. [VERIFIED: packages/open-bitcoin-node/src/metrics.rs; packages/open-bitcoin-node/src/status.rs; .planning/phases/16-metrics-logs-and-sync-telemetry/16-CONTEXT.md]

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|-------------|-----------|---------|----------|
| Rust toolchain | Build, tests, lint, formatting | yes | rustc 1.94.1; cargo 1.94.1 | None needed. [VERIFIED: rustc --version; cargo --version; rust-toolchain.toml] |
| rustfmt | Formatting | yes | rustfmt 1.8.0-stable | None needed. [VERIFIED: rustfmt --version] |
| clippy | Linting | yes | clippy 0.1.94 | None needed. [VERIFIED: cargo clippy --version] |
| cargo-llvm-cov | Repo verify coverage gate | yes | 0.8.5 | None needed. [VERIFIED: cargo llvm-cov --version; scripts/verify.sh] |
| Bazel/Bazelisk command | Repo verify Bazel smoke build | yes | bazel 8.6.0 | None needed. [VERIFIED: bazel --version; scripts/verify.sh] |
| Bun | Repo automation scripts | yes | 1.3.9 | None needed. [VERIFIED: bun --version; scripts/verify.sh] |
| git | Verification scripts and optional doc commit | yes | 2.53.0 | None needed. [VERIFIED: git --version; scripts/verify.sh] |
| jq | Research/version probing only | yes | jq-1.7.1-apple | Not required by implementation. [VERIFIED: jq --version] |
| ripgrep | Research/code search only | yes | ripgrep 15.1.0 | Not required by implementation. [VERIFIED: rg --version] |

**Missing dependencies with no fallback:** None found for Phase 16 implementation and repo-native verification. [VERIFIED: environment availability commands]

**Missing dependencies with fallback:** None found for Phase 16 implementation. [VERIFIED: environment availability commands]

## Security Domain

Security enforcement is enabled by default because `.planning/config.json` does not set `security_enforcement: false`. [VERIFIED: .planning/config.json]

OWASP lists ASVS 5.0.0 as the latest stable version and recommends version-qualified requirement references because identifiers can change. [CITED: https://owasp.org/www-project-application-security-verification-standard/; CITED: https://github.com/OWASP/ASVS]

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|------------------|
| V2 Authentication | No direct auth change in Phase 16 | Do not log RPC cookies, passwords, tokens, or credential-bearing paths; preserve existing auth behavior. [VERIFIED: .planning/phases/16-metrics-logs-and-sync-telemetry/16-CONTEXT.md; packages/open-bitcoin-rpc/src/http.rs] |
| V3 Session Management | No direct session change in Phase 16 | No browser/session state is introduced by metrics/logs/sync telemetry. [VERIFIED: .planning/ROADMAP.md; packages/open-bitcoin-node/src] |
| V4 Access Control | Limited | Log readers/pruners must operate only on configured managed log paths; service/CLI authorization is later scope. [VERIFIED: .planning/ROADMAP.md; .planning/phases/16-metrics-logs-and-sync-telemetry/16-CONTEXT.md] |
| V5 Input Validation | Yes | Parse structured logs with serde, bound query limits, validate managed log filenames before deletion, and use typed metric/log/status models. [VERIFIED: packages/open-bitcoin-node/src/logging.rs; packages/open-bitcoin-node/src/status.rs; serde_json 1.0.149 local source] |
| V6 Cryptography | No new cryptography | Do not add custom crypto for telemetry; existing Bitcoin cryptography remains in pure consensus/core crates. [VERIFIED: AGENTS.md; packages/open-bitcoin-node/Cargo.toml] |

### Known Threat Patterns for Phase 16

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Log injection or malformed records | Tampering | Use serde JSON encoding/decoding for records and skip or surface malformed records as unavailable/corruption evidence instead of string parsing. [VERIFIED: serde_json 1.0.149 local source; packages/open-bitcoin-node/src/logging.rs] |
| Unbounded metrics/log growth | Denial of Service | Enforce `MetricRetentionPolicy` and `LogRetentionPolicy` caps with deterministic tests. [VERIFIED: docs/architecture/operator-observability.md; packages/open-bitcoin-node/src/metrics.rs; packages/open-bitcoin-node/src/logging.rs] |
| Secret leakage in logs or health signals | Information Disclosure | Keep messages concise, operator-actionable, and source-scoped; do not serialize raw credentials, cookie values, or full request payloads. [VERIFIED: .planning/phases/16-metrics-logs-and-sync-telemetry/16-CONTEXT.md; packages/open-bitcoin-rpc/src/http.rs] |
| Path traversal or deleting unmanaged files | Tampering | Build retention from managed log file metadata discovered under the configured log directory and delete only files matching the managed naming scheme. [VERIFIED: .planning/phases/16-metrics-logs-and-sync-telemetry/16-CONTEXT.md] |
| Live-network dependence in default telemetry tests | Denial of Service | Keep live network smoke tests ignored/opt-in and use scripted transports for default verification. [VERIFIED: .planning/phases/15-real-network-sync-loop/15-CONTEXT.md; packages/open-bitcoin-node/src/sync/tests.rs] |

## Sources

### Primary (HIGH confidence)

- `.planning/phases/16-metrics-logs-and-sync-telemetry/16-CONTEXT.md` - locked Phase 16 decisions, scope, deferred ideas, and implementation constraints. [VERIFIED: cat]
- `.planning/REQUIREMENTS.md` - OBS-03, OBS-04, OBS-05, and SYNC-06 requirement definitions. [VERIFIED: cat]
- `.planning/ROADMAP.md` - Phase 16 scope and downstream Phase 17/18/19 boundaries. [VERIFIED: cat]
- `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md` - repo-local workflow, Bright Builds routing, local overrides. [VERIFIED: cat]
- `../coding-and-architecture-requirements/standards/core/architecture.md` - functional core / imperative shell guidance. [VERIFIED: cat]
- `../coding-and-architecture-requirements/standards/core/code-shape.md` - early-return, `maybe_`, and file/function shape guidance. [VERIFIED: cat]
- `../coding-and-architecture-requirements/standards/core/testing.md` - Arrange/Act/Assert and focused unit tests. [VERIFIED: cat]
- `../coding-and-architecture-requirements/standards/core/verification.md` - repo-native verification guidance. [VERIFIED: cat]
- `../coding-and-architecture-requirements/standards/languages/rust.md` - Rust module and testing guidance. [VERIFIED: cat]
- `docs/architecture/operator-observability.md` - exact metrics/log retention defaults and Phase 16 runtime responsibility. [VERIFIED: cat]
- `docs/architecture/status-snapshot.md` - shared status ownership and unavailable-field semantics. [VERIFIED: cat]
- `packages/open-bitcoin-node/src/metrics.rs` - `MetricKind`, `MetricRetentionPolicy`, `MetricSample`, and `MetricsStatus`. [VERIFIED: cat]
- `packages/open-bitcoin-node/src/logging.rs` - `LogRetentionPolicy`, `StructuredLogLevel`, `RecentLogSignal`, and `LogStatus`. [VERIFIED: cat]
- `packages/open-bitcoin-node/src/status.rs` - shared status and health signal contracts. [VERIFIED: cat]
- `packages/open-bitcoin-node/src/storage/fjall_store.rs` and `packages/open-bitcoin-node/src/storage/snapshot_codec.rs` - durable metrics snapshot persistence and serde DTOs. [VERIFIED: cat]
- `packages/open-bitcoin-node/src/sync.rs`, `packages/open-bitcoin-node/src/sync/types.rs`, and `packages/open-bitcoin-node/src/sync/tests.rs` - sync runtime, summaries, outcomes, tests, and current metric persistence hooks. [VERIFIED: cat]
- `packages/open-bitcoin-node/Cargo.toml`, `packages/Cargo.toml`, `packages/open-bitcoin-node/BUILD.bazel`, `scripts/verify.sh`, `rust-toolchain.toml` - workspace dependencies, Bazel deps, verification command, and toolchain pin. [VERIFIED: cat]
- Local cargo registry source for `fjall` 3.1.4 and `serde_json` 1.0.149 - API capabilities for keyspace range/insert/remove and JSON encode/decode. [VERIFIED: rg/sed ~/.cargo/registry/src]
- crates.io API and `cargo search`/`cargo info` - current crate versions and publish dates for `fjall`, `serde`, and `serde_json`. [VERIFIED: cargo search; cargo info; curl crates.io API]
- OWASP ASVS official project page and GitHub README - latest stable ASVS 5.0.0 and version-qualified identifier guidance. [CITED: https://owasp.org/www-project-application-security-verification-standard/; CITED: https://github.com/OWASP/ASVS]

### Secondary (MEDIUM confidence)

- None needed; the phase is constrained by first-party contracts and official package/security sources. [VERIFIED: source review]

### Tertiary (LOW confidence)

- None. [VERIFIED: source review]

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - versions were checked against local manifests, cargo metadata, cargo search/cargo info, and crates.io API. [VERIFIED: packages/open-bitcoin-node/Cargo.toml; cargo tree; crates.io API]
- Architecture: HIGH - the shape follows locked Phase 16 decisions, existing node-shell modules, and Bright Builds functional-core guidance. [VERIFIED: .planning/phases/16-metrics-logs-and-sync-telemetry/16-CONTEXT.md; packages/open-bitcoin-node/src; ../coding-and-architecture-requirements/standards/core/architecture.md]
- Pitfalls: HIGH - pitfalls come directly from current code gaps and locked retention/status requirements. [VERIFIED: packages/open-bitcoin-node/src/sync.rs; docs/architecture/operator-observability.md; .planning/phases/16-metrics-logs-and-sync-telemetry/16-CONTEXT.md]

**Research date:** 2026-04-26
**Valid until:** 2026-05-26 for project architecture; recheck crate versions before adding any dependency or changing pinned versions. [VERIFIED: crates.io API; AGENTS.md dependency policy]
