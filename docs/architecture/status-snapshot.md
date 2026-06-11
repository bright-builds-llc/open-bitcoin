# Status Snapshot Contract

## OpenBitcoinStatusSnapshot

`OpenBitcoinStatusSnapshot` is the sole shared status model for later CLI status output, JSON automation, service diagnostics, dashboard panels, and support bundles. Live RPC is not the only status source; stopped-node inspection can still report local datadir, config paths, service state, log paths, locally collected health signals, metrics policy, and build provenance when those collectors are available.

For v1.6, `OpenBitcoinStatusSnapshot` is the shared source of truth for status, dashboard, support evidence, RPC-facing blockchain info, metrics projections, structured logs, and live-smoke snapshots. Each consumer may render a different view, but it must preserve the same lifecycle, phase, configured targets, attempt counters, header height, downloaded block height, connected block height, validated active-chain height/hash/work, peer compatibility state, progress signal, latest stop reason, latest error, recovery state, resource pressure, and unavailable-field reasons instead of inventing renderer-local summaries.

## Field Ownership

| Field | Owner | OBS-01 details |
| --- | --- | --- |
| `node` | Runtime/process collector | daemon state and version |
| `config` | Config/datadir collector | `datadir` and `config paths` |
| `service` | Service lifecycle collector | service manager and installed/enabled/running state |
| `sync` | Sync/runtime collector | `network`, `chain tip`, `sync progress`, lifecycle, phase, progress signal, estimated lag, last successful progress, resource pressure, recovery guidance, and last error |
| `peers` | Network collector | `peer counts` plus recent peer telemetry when durable sync state is available |
| `mempool` | Mempool collector | mempool summary |
| `wallet` | Wallet collector | `trusted_balance_sats`, `freshness`, and `scan_progress` so balances never imply completeness by themselves |
| `logs` | Logging collector | log paths and retention |
| `metrics` | Metrics collector | retention, enabled series, and bounded samples when a metrics snapshot exists |
| `health_signals` | Log/status collectors | recent `health signals` |
| `build` | Build/release collector | version, commit, build time, target, and profile |

## Phase 62 sync truth contract

The canonical Phase 62 sync truth projection is the `sync` status object plus
its peer, metric, and structured-log projections. Human renderers may use title
case labels, TypeScript reports may use a single camelCase mapping layer, and
RPC/JSON/log/metric surfaces keep stable snake_case machine labels. Missing
typed fields must render exactly as `Unavailable: {reason}` in human-facing
output or preserve the equivalent unavailable/null reason in machine output.

The shared order is:

1. `lifecycle`
2. `phase`
3. `configured_targets`
4. `attempt_counters`
5. `progress_signal`
6. `last_successful_progress_unix_seconds`
7. `latest_stop_reason`
8. `last_error`
9. `recovery_category`
10. `recovery_action`
11. `resource_pressure`
12. `peer health`
13. `header_height`
14. `downloaded_block_height`
15. `maybe_downloaded_block_hash`
16. `connected_block_height`
17. `maybe_connected_block_hash`
18. `validated_active_chain_height`
19. `maybe_validated_active_chain_hash`
20. `maybe_validated_active_chain_work`
21. `messages_processed`
22. `headers_received`
23. `blocks_received`

## Stopped-node status

Stopped-node status must not omit live fields. Fields that cannot be collected because the daemon is stopped use `Unavailable` with a `reason`. For example, live `network`, `chain tip`, `sync progress`, `peer counts`, mempool, and wallet values can be unavailable while datadir, config paths, service state, logs, metrics policy, health signals, and build provenance remain visible.

`node.state = stopped` can also mean live RPC was not attempted because the
operator side could not rediscover credentials for the selected datadir. That
bootstrap distinction should surface through warning health signals, not a
separate top-level status field.

When durable sync metadata exists, stopped or unreachable-node status may still
surface the last known sync lifecycle, phase, progress signal, estimated lag,
last successful progress, peer telemetry, recovery guidance, and last sync
error from the durable store rather than collapsing those fields back to
renderer-local guesses.

Support bundles embed this same snapshot instead of defining a separate support
DTO. Bundle consumers should therefore preserve `Unavailable` fields and their
reasons verbatim; missing live data is useful diagnostic evidence, not a
serialization error.

## Sync progress semantics

`sync.sync_progress` separates validated header, durable download, and connected
chainstate progress:

- `header_height`: best validated header height.
- `downloaded_block_height`: highest contiguous best-chain block body available
  in the durable store.
- `connected_block_height`: active chainstate height.
- `validated_active_chain_height`: explicit active-chain progress credit. It
  matches connected chainstate height and advances only after consensus validation,
  active-chain connection, and durable persistence.
- `block_height`: compatibility alias for `connected_block_height`.
- `maybe_validated_active_chain_hash`: active chainstate tip hash when a
  connected tip is available.
- `maybe_validated_active_chain_work`: active chainstate tip cumulative work as
  a decimal string when connected chainstate evidence is available.

Consumers should use the explicit downloaded and connected fields for recovery
diagnostics. Downloaded-only block bodies must not be treated as validated
active-chain progress. `last_error`, `recovery_category`, and
`recovery_action` are separate fields so a status snapshot can report active
progress, the stable machine recovery label, the latest recoverable error, and
the human next-action text at the same time.

## Tip and stay-current semantics

`sync.best_known_tip` is the `BestKnownTipStatus` contract. When available, it
contains:

- `source`: currently `header_store`, meaning the durable validated header
  store supplied the best-known tip evidence.
- `height`: best-known validated header height.
- `block_hash`: best-known validated header hash.
- `work`: cumulative work for the best-known validated header tip, encoded as a
  decimal string.
- `block_time_unix_seconds`: timestamp carried by the best-known tip header.
- `observed_at_unix_seconds`: local observation timestamp used for freshness.
- `freshness`: `fresh` or `stale`.
- `peer_agreement`: bounded per-peer agreement evidence with each peer marked
  `agrees`, `behind`, `disagrees`, or `no_evidence`.

`sync.stay_current` is the `StayCurrentStatus` contract. Current values are:

- `initial_catch_up`
- `current_at_best_known_tip`
- `stale_tip`
- `recovering`
- `no_progress`

`sync.stay_current_next_action` is bounded human guidance derived from
`StayCurrentStatus`. It is available for `initial_catch_up`,
`current_at_best_known_tip`, `stale_tip`, and `no_progress`. It stays
unavailable for `recovering` so existing recovery-category and recovery-action
fields remain the source of recovery guidance.

Current-at-tip requires fresh best-known tip evidence and connected
active-chain height/hash/work matching that best-known validated tip. Headers
without corresponding connected blocks, or downloaded-only block bodies, must
not satisfy `current_at_best_known_tip`.

`sync.recovery_category` is the stable machine label for unattended sync
recovery. Current values are:

- `clean_shutdown`
- `unclean_shutdown`
- `incompatible_schema`
- `store_corruption`
- `storage_lock_contention`
- `storage_backend_failure`
- `resource_exhaustion`
- `invalid_peer_data`
- `public_network_unreachable`
- `operator_cancellation`

`sync.recovery_action` remains human guidance and may change wording without
changing the category label. Storage recovery categories take precedence over
peer or public-network categories when both are available.

`sync.progress_signal` is a coarse machine-readable summary of the latest sync
run. Current values are:

- `header_progress`: at least one peer contributed accepted headers.
- `block_progress`: at least one peer contributed accepted blocks.
- `waiting_for_peers`: sync is waiting for retry backoff or peer availability.
- `peer_failures`: the latest run saw peer failures without useful progress.
- `awaiting_blocks`: validated headers are ahead of connected chainstate.
- `steady`: no immediate sync work or failure signal was observed.

`sync.last_successful_progress_unix_seconds` records the most recent accepted
header or block contribution time when known. Durable status preserves the prior
timestamp across later runs that only wait, fail, or report no new progress.

`sync.configured_targets` reports the configured outbound peer target and the
optional target header height. `sync.attempt_counters` reports bounded peer
attempts, connected peers, failed peers, and configured max sync rounds for the
latest durable cycle. `sync.latest_stop_reason` reports the typed durable stop
reason label and message when a cycle stopped for a known reason.

`sync.lag` is the estimated lag from best known validated work. It reports
header and block counts rather than a wall-clock ETA so deterministic local
status remains truthful even when the public network is unavailable.

## Sync resource pressure

`sync.resource_pressure` reports observed pressure and configured bounds
together. Consumers should treat observed fields and durable progress counters
as observations:

- `blocks_in_flight`
- `outbound_peers`

The remaining resource-pressure fields are the currently configured runtime
envelope:

- `max_header_requests_in_flight_per_peer`
- `max_headers_per_message`
- `max_blocks_in_flight_per_peer`
- `max_blocks_in_flight_total`
- `max_messages_per_peer`
- `max_sync_rounds`
- `target_outbound_peers`

This keeps status, dashboard, RPC-facing blockchain info, support evidence,
metrics projections, structured logs, and live-smoke snapshots aligned on one
source of truth for public-network runtime bounds.

## Metrics and Logs

Sync metrics expose the same progress dimensions as status:

- `header_height`: best validated header height.
- `downloaded_block_height`: highest contiguous best-chain block body available
  in the durable store.
- `connected_block_height`: active chainstate height.
- `sync_height`: compatibility series for connected chainstate height.

Structured sync summary logs use the same header, downloaded, connected,
progress-signal, and last-progress values. Status carries the
`validated_active_chain_height`, `maybe_validated_active_chain_hash`, and
`maybe_validated_active_chain_work` evidence needed to distinguish connected
active-chain progress from downloaded-only bodies. Consumers should prefer the
status snapshot for machine state and use logs as an audit trail of how the
state was observed.

## Build provenance semantics

`build.version` should reflect the workspace package version, and the remaining
`build.*` fields should come from truthful compile-time metadata supplied by the
active build system.

- Cargo builds can surface Cargo `TARGET` and `PROFILE` values.
- Bazel builds can surface Bazel `TARGET_CPU` and `COMPILATION_MODE` values.

Consumers should treat those strings as build-system-specific provenance, not as
one normalized cross-build enum.

## Wallet freshness semantics

`wallet.trusted_balance_sats` remains part of the shared snapshot, but operator-facing consumers must treat it as incomplete unless `wallet.freshness` says otherwise.

- `fresh`: the wallet view has caught up to the durable node tip.
- `stale`: the wallet tip lags the durable node tip and no active scan progress is being reported.
- `partial`: the wallet view is incomplete and only partial scan progress is known.
- `scanning`: an active rescan is in progress and `wallet.scan_progress` reports the current `scanned_through_height` and `target_tip_height`.

When the daemon is stopped or the wallet state cannot be collected, both `wallet.freshness` and `wallet.scan_progress` stay `Unavailable` with a reason instead of silently defaulting to a balance-only summary.

## Non-Goals

This contract does not implement a status command, renderer, dashboard, service manager, RPC collector, filesystem collector, or clock source. Later collectors map their evidence into this data model.
