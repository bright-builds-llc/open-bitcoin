---
phase: 16-metrics-logs-and-sync-telemetry
slug: metrics-logs-and-sync-telemetry
status: verified
threats_open: 0
asvs_level: 1
created: 2026-05-03
generated_by: gsd-secure-phase
lifecycle_mode: yolo
phase_lifecycle_id: 16-2026-04-26T21-50-05
generated_at: 2026-05-03T13:11:46Z
---

# Phase 16 - Security

Per-phase security contract: threat register, accepted risks, and audit trail.

## Scope

Phase 16 adds bounded metrics retention, typed metrics snapshot persistence,
structured JSONL runtime logging, deterministic log pruning, status-facing
warning and error summaries, and sync telemetry projection into shared metrics,
logs, status, and health signals. The phase threat model is explicit in
`16-01-PLAN.md`, `16-02-PLAN.md`, and `16-03-PLAN.md`, so this audit verified
those declared threats against implementation, verification, and targeted exact
tests only. No unrelated vulnerability discovery pass was performed.

The phase summaries do not include a `## Threat Flags` section. The only
non-mitigated item in the plan is the declared accepted risk that metrics values
are numeric runtime evidence rather than secrets. That accepted disposition is
documented below so `threats_open: 0` reflects the plan rather than hiding an
implementation gap.

## Trust Boundaries

| Boundary | Description | Data Crossing |
|----------|-------------|---------------|
| Runtime collectors -> metrics core | Runtime values become bounded metric samples. | Sync heights, peer counts, mempool counts, wallet summary values, disk usage, RPC health, service restart counters. |
| Metrics core -> Fjall storage | Pure retention decisions become durable snapshot writes. | Typed `MetricSample` history within `MetricsStorageSnapshot`. |
| Runtime event -> structured log record | Local runtime events become serialized status-facing log records. | `StructuredLogRecord { level, source, message, timestamp_unix_seconds }`. |
| Configured log directory -> log reader and pruner | Filesystem contents under the managed log directory are inspected and pruned. | Managed `open-bitcoin-runtime-<unix_day>.jsonl` files only. |
| Remote peer/scripted transport -> sync runtime | Peer outcomes influence metrics, logs, status, and health evidence. | Connection failures, retry counts, stalled-peer state, message/header/block counters. |
| Sync evidence -> later status and dashboard consumers | Later operator surfaces trust Phase 16 evidence without reparsing raw network state. | Sync progress counters, recent warning/error signals, durable metrics history, peer outcomes. |

## Threat Register

| Threat ID | Category | Component | Disposition | Mitigation | Status |
|-----------|----------|-----------|-------------|------------|--------|
| T-16-01-01 | Denial of Service | `append_and_prune_metric_samples` | mitigate | Bounded retention is enforced before save through [metrics.rs](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/metrics.rs:86), iterating all canonical kinds and pruning by policy, with exact regression coverage in [metrics.rs](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/metrics.rs:350) and durable reopen coverage in [tests.rs](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/storage/fjall_store/tests.rs:358). Exact tests passed: `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node metrics::tests::append_and_prune_metric_samples_enforces_sample_interval_buckets -- --exact` and `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node storage::fjall_store::tests::metrics_history_appends_across_reopen -- --exact`. | closed |
| T-16-01-02 | Tampering | `MetricsStorageSnapshot` | mitigate | Metrics persistence stays typed and versioned: [snapshot_codec.rs](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/storage/snapshot_codec.rs:119) defines `MetricsStorageSnapshot` as `Vec<MetricSample>`, and [snapshot_codec.rs](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/storage/snapshot_codec.rs:200) encodes and decodes it only through the `StorageNamespace::Metrics` codec. Fjall append logic persists that typed snapshot in [fjall_store.rs](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/storage/fjall_store.rs:318). | closed |
| T-16-01-03 | Information Disclosure | Metrics values | accept | Accepted risk: Phase 16 metrics are numeric runtime evidence rather than secrets, and unavailable collectors are represented by explicit status metadata instead of fake samples. Evidence: [metrics.rs](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/metrics.rs:166) keeps unavailable status as a reason string while preserving enabled series. The acceptance is documented in the Accepted Risks Log below. | closed |
| T-16-01-04 | Repudiation | Status-facing metrics availability | mitigate | Missing metrics history returns explicit unavailable status with retention and enabled-series metadata intact via [fjall_store.rs](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/storage/fjall_store.rs:336) and [metrics.rs](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/metrics.rs:150). Exact tests passed: `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node metrics::tests::available_metrics_status_preserves_retention_and_series -- --exact` and `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node storage::fjall_store::tests::missing_metrics_snapshot_reports_unavailable_status -- --exact`. | closed |
| T-16-02-01 | Tampering | JSONL log records | mitigate | Structured log records are encoded and decoded through typed serde paths only: [writer.rs](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/logging/writer.rs:62) uses `serde_json::to_vec(record)`, and [writer.rs](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/logging/writer.rs:155) reads back into `StructuredLogRecord`. Exact tests passed: `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node logging::tests::default_log_retention_matches_operator_contract -- --exact` and `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node logging::tests::load_log_status_reads_bounded_recent_signals -- --exact`. | closed |
| T-16-02-02 | Denial of Service | Log retention | mitigate | Retention is deterministic and bounded in [prune.rs](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/logging/prune.rs:41), enforcing `max_files`, `max_age_days`, and `max_total_bytes`, and the operator contract matches the defaults in [logging.rs](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/logging.rs:128) and [operator-observability.md](/Users/peterryszkiewicz/Repos/open-bitcoin/docs/architecture/operator-observability.md:13). Exact tests passed: `... logging::tests::retention_prunes_by_max_files -- --exact`, `... logging::tests::retention_prunes_by_max_age -- --exact`, and `... logging::tests::retention_prunes_by_total_bytes -- --exact`. | closed |
| T-16-02-03 | Tampering | Log pruning | mitigate | Pruning selects managed files only: [prune.rs](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/logging/prune.rs:15) fixes the managed prefix, [prune.rs](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/logging/prune.rs:60) rejects non-matching names, and [writer.rs](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/logging/writer.rs:127) loads managed paths through that parser. | closed |
| T-16-02-04 | Information Disclosure | Log messages and health signals | mitigate | Phase 16 log records are constrained to `level`, `source`, `message`, and `timestamp_unix_seconds` in [logging.rs](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/logging.rs:28), and the phase summaries and verification report confirm no RPC/auth surface was introduced. Sync log and health messages are concise counter or guidance strings rather than raw payload dumps in [sync/types.rs](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/sync/types.rs:390) and [sync/progress.rs](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/sync/progress.rs:120). | closed |
| T-16-02-05 | Repudiation | Missing log paths | mitigate | Missing log directories return explicit unavailable path status with retained metadata via [writer.rs](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/logging/writer.rs:40) and [writer.rs](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/logging/writer.rs:164). Exact test passed: `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node logging::tests::missing_log_directory_reports_unavailable_status -- --exact`. | closed |
| T-16-03-01 | Denial of Service | Sync metrics persistence | mitigate | Sync runtime appends through bounded history instead of overwriting snapshots: [sync.rs](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/sync.rs:165) routes summary persistence through `persist_metrics`, and [fjall_store.rs](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/storage/fjall_store.rs:318) reloads and appends existing samples before save. Exact tests passed: `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node storage::fjall_store::tests::metrics_history_appends_across_reopen -- --exact` and `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node sync::tests::sync_status_and_log_records_include_message_header_block_counters -- --exact`. | closed |
| T-16-03-02 | Tampering | Peer-derived log records | mitigate | Sync summary logging uses the same typed `StructuredLogRecord` path as the managed logger: [sync/types.rs](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/sync/types.rs:390) builds records structurally, and [writer.rs](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/logging/writer.rs:62) serializes them through serde, not raw string concatenation into files. | closed |
| T-16-03-03 | Information Disclosure | Sync log messages | mitigate | Sync log messages are bounded to counters and operator guidance strings such as the summary line in [sync/types.rs](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/sync/types.rs:395) and stalled-peer warning in [progress.rs](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/sync/progress.rs:120). Health-signal conversions similarly emit concise diagnostics in [sync/types.rs](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/sync/types.rs:453) without exposing raw peer payloads, cookies, or headers. | closed |
| T-16-03-04 | Repudiation | Retry and stall evidence | mitigate | Sync runtime preserves retry and stall evidence in shared status and health signals: [sync.rs](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/sync.rs:331) records counters and stalled-peer signals, [sync/tests.rs](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/sync/tests.rs:815) proves stalled-peer warning propagation, and [sync/tests.rs](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/sync/tests.rs:848) proves retry attempt counts remain visible. Exact tests passed: `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node sync::tests::stalled_peer_emits_warning_health_signal_and_log_record -- --exact` and `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node sync::tests::connect_retries_preserve_attempt_count -- --exact`. | closed |
| T-16-03-05 | Denial of Service | Default verification | mitigate | Telemetry tests remain hermetic by default through [sync/tests.rs](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/sync/tests.rs:39) `ScriptedTransport`, isolated temp stores in existing exact tests, and the live-network smoke path stays explicitly opt-in at [sync/tests.rs](/Users/peterryszkiewicz/Repos/open-bitcoin/packages/open-bitcoin-node/src/sync/tests.rs:1436). Phase 16 UAT also verified that public-network coverage is not part of default verification. | closed |

Status: open, closed.
Disposition: mitigate (implementation required), accept (documented risk), transfer (third-party).

## Accepted Risks Log

| Risk ID | Threat Ref | Rationale | Accepted By | Date |
|---------|------------|-----------|-------------|------|
| AR-16-01 | T-16-01-03 | Metrics values in scope for Phase 16 are numeric runtime evidence and retention metadata, not secrets. The risk is accepted so unavailable collectors stay explicit through reason strings instead of fake samples. | gsd-secure-phase (per declared plan disposition) | 2026-05-03 |

## Unregistered Flags

No unregistered flags.

| Summary | Threat Flags Result | Mapping |
|---------|---------------------|---------|
| `16-01-SUMMARY.md` | No `## Threat Flags` section present. | None |
| `16-02-SUMMARY.md` | No `## Threat Flags` section present. | None |
| `16-03-SUMMARY.md` | No `## Threat Flags` section present. | None |

## Security Audit Trail

| Audit Date | Threats Total | Closed | Open | Run By |
|------------|---------------|--------|------|--------|
| 2026-05-03 | 14 | 14 | 0 | gsd-secure-phase |

## Verification Evidence

| Command | Result |
|---------|--------|
| `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node metrics::tests::append_and_prune_metric_samples_enforces_sample_interval_buckets -- --exact` | Passed. |
| `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node metrics::tests::available_metrics_status_preserves_retention_and_series -- --exact` | Passed. |
| `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node storage::fjall_store::tests::metrics_history_appends_across_reopen -- --exact` | Passed. |
| `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node storage::fjall_store::tests::missing_metrics_snapshot_reports_unavailable_status -- --exact` | Passed. |
| `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node logging::tests::default_log_retention_matches_operator_contract -- --exact` | Passed. |
| `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node logging::tests::load_log_status_reads_bounded_recent_signals -- --exact` | Passed. |
| `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node logging::tests::missing_log_directory_reports_unavailable_status -- --exact` | Passed. |
| `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node logging::tests::retention_prunes_by_max_files -- --exact` | Passed. |
| `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node logging::tests::retention_prunes_by_max_age -- --exact` | Passed. |
| `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node logging::tests::retention_prunes_by_total_bytes -- --exact` | Passed. |
| `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node sync::tests::sync_status_and_log_records_include_message_header_block_counters -- --exact` | Passed. |
| `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node sync::tests::stalled_peer_emits_warning_health_signal_and_log_record -- --exact` | Passed. |
| `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node sync::tests::connect_retries_preserve_attempt_count -- --exact` | Passed. |

## Standards Inputs

Materially applied local [AGENTS.md](/Users/peterryszkiewicz/Repos/open-bitcoin/AGENTS.md),
[AGENTS.bright-builds.md](/Users/peterryszkiewicz/Repos/open-bitcoin/AGENTS.bright-builds.md),
[standards-overrides.md](/Users/peterryszkiewicz/Repos/open-bitcoin/standards-overrides.md),
the Bright Builds pinned `standards/index.md`, `standards/core/verification.md`,
and `standards/core/testing.md`, the repo-local Phase 16 plan, summary,
verification, and UAT artifacts, and the `gsd-secure-phase` workflow.
ASVS Level: 1.

## Sign-Off

- [x] All threats have a disposition (mitigate / accept / transfer)
- [x] Accepted risks documented in Accepted Risks Log
- [x] `threats_open: 0` confirmed
- [x] `status: verified` set in frontmatter

Approval: verified 2026-05-03
