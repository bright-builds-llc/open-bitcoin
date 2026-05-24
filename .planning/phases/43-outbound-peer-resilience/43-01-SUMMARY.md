---
phase: 43
phase_name: "Outbound Peer Resilience"
plan_id: "43-01"
generated_by: gsd-execute-phase
lifecycle_mode: yolo
phase_lifecycle_id: "43-2026-05-24T20-38-15"
generated_at: "2026-05-24T20:51:50Z"
status: completed
---

# Summary 43-01: Backoff Visibility And Mixed Peer Failure Resilience

## Completed

- Added configured outbound target tracking to `SyncRunSummary` so direct sync
  status can distinguish observed outbound peers from the configured
  `target_outbound_peers` bound.
- Added `PeerSyncState::Waiting` and `PeerFailureReason::RetryBackoff`, and
  changed the sync peer loop so peers still inside retry backoff are reported as
  waiting outcomes instead of being silently skipped.
- Projected waiting/backoff state through peer telemetry, health signals,
  structured logs, and the direct sync phase name `waiting_for_peers` when no
  peer is eligible for a connection attempt.
- Extended deterministic sync tests for backoff replacement, all-peers-waiting
  status, configured target projection, and mixed connection failure plus
  invalid-data replacement survival.
- Updated operator docs and v1.3 GSD state for Phase 43 completion.

## Tests Added

- `sync_once_reports_backoff_wait_and_replaces_peer`
- `sync_once_waiting_backoff_projects_waiting_for_peers_phase`
- `sync_status_preserves_configured_target_outbound_peer_count`
- `mixed_peer_failures_rotate_to_replacement_without_corrupting_state`

## Residual Risks

- Public-mainnet full-sync proof remains environment-dependent and is still
  deferred to later v1.3 phases.
- Phase 43 makes backoff/replacement visible but does not yet attribute useful
  header/block contribution to individual peers; that remains Phase 44.
- Long-run resource-bound and single-writer store coordination hardening remains
  Phase 45.
