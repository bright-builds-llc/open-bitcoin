# Operator Observability Contracts

`OpenBitcoinStatusSnapshot` is the shared source of truth for status, dashboard,
support evidence, RPC-facing blockchain info, metrics projections, structured
logs, and live-smoke snapshots. Observability writers and readers may keep their
own retention policies, but they must not reinterpret header height, downloaded
block height, connected block height, validated_active_chain_height,
maybe_validated_active_chain_hash, maybe_validated_active_chain_work,
best_known_tip, stay_current, no_progress_diagnosis,
no_progress_next_action, latest_reorg, reconcile_progress, resource_pressure,
resource_bounds, peer_contribution, latest_stop_reason, evidence_verdict,
recovery_category, configured_targets, attempt_counters, or latest error
independently from the shared snapshot.

## Default metrics retention

Metrics history defaults to a 30 seconds sampling interval, 2880 samples per series, and a 24 hours maximum age. The intent is to give status and dashboard consumers bounded numeric samples for a day-scale window without creating unbounded runtime storage.
The runtime implementation names this bounded metrics envelope
`MetricRetentionPolicy`; docs and support evidence should describe configured
retention rather than retaining raw metric arrays.

Required metric kinds are sync height, header height, downloaded block height,
connected block height, peer count, mempool transactions, wallet trusted balance
in sats, disk usage bytes, RPC health, and service restarts.

Phase 62 metrics remain bounded numeric samples for `header_height`,
`downloaded_block_height`, `connected_block_height`, `sync_height`, and
`peer_count`. Phase 72 adds `validated_active_chain_height` as a bounded numeric
metric so operators can correlate validated active-chain progress with status
and support evidence. Stop reasons, recovery labels, configured targets,
resource_pressure, peer_contribution, latest_stop_reason, and attempt counters
belong in status or compact structured logs, not as unbounded metric objects.

No metric or log retention contract may require public network access. Default verification must remain hermetic; live-network telemetry belongs behind explicit opt-in tests or operator runtime paths.

## Default log retention

Structured logs default to daily rotation, 14 files, 14 days, and 268435456 bytes of total retained log data. Rolling file creation is not retention pruning. Phase 16 must implement pruning separately from any rolling file writer and must test max-file, max-age, and byte-cap behavior.
The runtime implementation names this bounded log envelope `LogRetentionPolicy`;
operator evidence should report retention configuration and compact sync facts
instead of preserving raw daemon tails.

Managed runtime log files use the `open-bitcoin-runtime-<unix_day>.jsonl` naming scheme, with one structured JSON record per line. The Unix-day bucket provides daily rotation without adding a calendar-formatting dependency; rolling file creation and retention pruning remain separate responsibilities.

Sync summary log records must report the same progress dimensions that status,
dashboard, support evidence, RPC-facing blockchain info, metrics projections,
and live-smoke snapshots use: header height, downloaded block height, connected
block height, progress signal, recovery_category, and last successful progress
timestamp when one is known.

Structured logs carry compact labels for `progress_signal`,
`latest_stop_reason`, `recovery_category`, configured targets, attempt
counters, `resource_pressure`, and `peer_contribution`. Summary messages use
stable labels such as `progress_signal=`, `latest_stop_reason=`,
`recovery_category=`, `target_outbound_peers=`, `target_header_height=`,
`messages_processed`, `headers_received`, `blocks_received`,
`validated_active_chain_height=`, and `maybe_validated_active_chain_work` so
operators can compare logs with status and live-smoke snapshots without parsing
prose.

Status and dashboard consumers must read these contracts instead of inventing renderer-local retention windows.

## Resource and recovery vocabulary

Status, structured logs, metrics projections, live-smoke summaries, and support
evidence use the shared sync vocabulary for `progress_signal`,
`recovery_category`, `resource_pressure`, and recovery guidance. Metrics remain
bounded numeric samples; bounded support evidence remains an allowlisted compact
summary rather than a raw daemon log, raw peer table, or retained report array.

Live-smoke Markdown and JSON use compact snapshots. The live-smoke compact
snapshot contract does not persist raw daemon tails. Reports may include exit
status, signal, observed-output flags, line counts, final status fields, and
bounded snapshot rows, but raw daemon stdout or stderr tails stay out of the
persisted report contract.

The retry-state bound is: peer retry state is keyed by resolved endpoint and bounded by candidate peers/outbound target per cycle. The storage-write bound is: durable storage writes are synchronous adapter calls with no queued write backlog. Phase 71 keeps `SyncResourcePressure`, `SyncRecoveryCategory::ResourceExhaustion`, and `StorageRecoveryAction::FreeDisk` in this same compact vocabulary so support evidence can explain storage pressure without expanding into raw logs, peer tables, or report archives.

## Phase 75 soak evidence ledger

The durable source of truth is <datadir>/soak/run-index.json plus <datadir>/soak/runs/<run_id>/events.jsonl.
Reports, support summaries, and operator output are projections from that
datadir-owned run index and append-only event ledger. They should name source
paths and latest sequence numbers when available rather than becoming a second
state store.

The soak ledger event kinds are `started`, `checkpoint`, `resume`, `stop`, and
`verdict`. Final outcomes are `clean_completion`, `diagnosed_blocker`,
`operator_stop`, `resource_stop`, `recovery_stop`, and
`unexpected_termination`. Observability surfaces should render these as soak
outcomes only; detailed sync stop, recovery, resource, and no-progress evidence
continues to come from the shared status snapshot.

## Phase 76 resource-bound evidence

Phase 76 makes `resource_bounds` the shared disk/resource contract for status,
dashboard, soak, support bundles, and report projections. The explicit RES-05
kind list is disk, file, cache, queue, peer, in-flight, log, metric, and
support-bundle; pressure states use 80% warning and 95% stop-required
thresholds against explicit budgets.

`soak start` evaluates resource-bound preflight before ledger mutation and
refuses missing datadir evidence, unavailable required measurements, invalid
disk budget evidence, or stop-required pressure. Checkpoints and reports retain
resource-bound labels, next action, and source status evidence so
`resource_stop` can be diagnosed without copying raw logs, raw stores, raw
status snapshots, live-smoke inputs, or unbounded peer tables. Support bundles render the same compact summary under `## Resource Bound Evidence`.

## Phase 77 corruption and lock recovery hardening

Phase 77 makes `recovery_evidence` the shared status, dashboard, support, and
soak recovery contract. The top-level field is
`recovery_evidence: FieldAvailability<RecoveryEvidenceSnapshot>`, and renderers
should preserve action class, cause, compatibility category, evidence basis,
affected namespace or path when present, unavailable reason, and next action
without reclassifying strings locally.

The stable action classes are `safe_retry`, `read_only_inspection`,
`backup_then_rebuild`, and `stop_and_escalate`. The stable causes are
`schema_mismatch`, `corruption_marker`, `corrupt_record`, `partial_write`,
`unreadable_namespace`, `backend_open_failure`, `active_lock`,
`stale_lock_evidence`, `concurrent_datadir_use`, and `resource_pressure`. The
compatibility categories are `incompatible_schema`, `store_corruption`,
`storage_lock_contention`, `storage_backend_failure`, and
`resource_exhaustion`.

Observability surfaces may report lock and corruption evidence, but they must
not imply automatic destructive repair, lock cleanup, source datadir mutation,
process scanning, public-network default verification, or production-node
readiness.

## Phase 78 progress-guarantee evidence

Phase 78 keeps progress guarantees and stall diagnosis in the shared sync
status contract. Status, dashboard, soak checkpoint/report, support, metrics,
structured-log, and live-smoke projections should preserve `progress_credit`,
`expected_progress_window`, `no_progress_threshold`, `last_useful_work`,
`last_peer_contribution`, and `stall_diagnosis` without deriving local verdicts
from renderer text.

The compact observability rule is that checkpoints, reports, support summaries,
and live-smoke rows may expose rejected activity and peer contribution evidence,
but only `validated_durable_active_chain` or `current_at_best_known_tip`
advances the credited progress watermark. Missing fields remain unavailable
evidence with their reason, and `stall_diagnosis` carries the subsystem and next
action rather than asking each surface to infer one.

## Phase Boundaries

Phase 13 defines serializable contracts only. It must not install a tracing subscriber, create a file appender, write metric samples, prune log files, or render dashboard graphs. Runtime writers and readers are Phase 16 responsibilities.
