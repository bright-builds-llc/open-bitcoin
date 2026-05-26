---
phase: 47
phase_name: Operator Sync Truth Surfaces
phase_lifecycle_id: 47-2026-05-26T21-36-05
lifecycle_mode: yolo
generated_at: 2026-05-26T21:36:05.164Z
generated_by: gsd-yolo-discuss-plan-execute-commit-and-push
requirements:
  - OBS-01
  - OBS-02
---

# Phase 47 Research

## Status Contract Findings

- `OpenBitcoinStatusSnapshot` is already the shared model for JSON status,
  human status, dashboard snapshots, and support-oriented future consumers.
- `SyncStatus` currently exposes network, chain tip, sync progress, lifecycle,
  phase, lag, last error, recovery action, and resource pressure.
- `SyncProgress` already separates validated header height, downloaded block
  height, connected block height, and compatibility `block_height`.
- The missing OBS-01 fields are an explicit progress signal and last successful
  progress timestamp. Both can be added as `FieldAvailability` fields without
  breaking existing serialized fields.

## Runtime Projection Findings

- `SyncRunSummary` owns status projection, metrics samples, structured log
  records, and peer telemetry. This is the right place to derive the active-run
  progress signal and latest contributing peer activity.
- `DurableSyncRuntime::durable_sync_state_from_summary` already has access to
  previous runtime metadata. It can preserve a prior durable
  `last_successful_progress_unix_seconds` value when the current run records no
  new accepted header or block contribution.
- Existing `lag` is an estimated best-known lag based on validated header and
  connected block heights. Duplicating it under another field would create drift.

## Surface Findings

- Human status already uses `headers`, `downloaded_blocks`, and
  `connected_blocks`. Dashboard progress still uses compact `blocks={}/{}`
  wording and hides recovery/error details.
- Dashboard charts currently include `sync_height`, peers, mempool tx, disk
  bytes, and RPC health. They do not chart header height or downloaded block
  height even though those samples are already status-relevant.
- RPC `getblockchaininfo` correctly maps durable `blocks` to connected height
  and `headers` to validated header height. This should remain conservative so
  downloaded-but-unconnected blocks never look fully validated.

## Implementation Notes

- Add `SyncProgressSignal` and `last_successful_progress_unix_seconds` to
  `SyncStatus`.
- Add `MetricKind::DownloadedBlockHeight` and
  `MetricKind::ConnectedBlockHeight`, while preserving `SyncHeight` as the
  compatibility connected-height metric.
- Add summary helpers for progress signal and last successful progress time.
- Update status, dashboard, RPC fallback, runtime support tests, and docs.
- Keep verification local and deterministic; live mainnet smoke remains out of
  the default gate.
