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
| `sync` | Sync/runtime collector | `network`, `chain tip`, `sync progress`, lifecycle, phase, progress signal, estimated lag, last successful progress, bounded reorg/reconcile evidence, no-progress diagnosis, resource pressure, recovery guidance, and last error |
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

## Phase 90 inbound serving status

`OpenBitcoinStatusSnapshot.peers.inbound` is the shared Phase 90 contract for
inbound listener and admission evidence. It is a child of the existing peer
status, not a renderer-local summary. CLI status, JSON status, dashboard
status, RPC-facing status, metrics projections, structured logs, and support
bundles should preserve the same listener state, preflight reason, bounded
endpoint summary, admission counters, handshake state counts, rejection
counters, latest admission event, and unavailable reason.

The baseline-shaped `getnetworkinfo` response remains responsible for
`connections`, `connections_in`, and `connections_out`. Detailed Phase 90
listener and admission evidence belongs to Open Bitcoin-owned status surfaces,
including the `openbitcoinnetworkstatus` RPC extension and the shared
`OpenBitcoinStatusSnapshot.peers.inbound` projection. A missing older-daemon RPC
extension should render inbound evidence as unavailable with a reason while
preserving baseline peer counts when `getnetworkinfo` is available.

Supported Phase 90 status labels include disabled listener state, ready
listener state, stable preflight reasons such as `disabled`,
`no_listen_addresses`, `invalid_endpoint`, `unsafe_endpoint`,
`bind_unavailable`, `already_bound`, and `ready`, plus admission reject labels
for cap, duplicate endpoint, duplicate peer id, self-connection, reserved slot,
and shutdown outcomes. Endpoint evidence stays bounded and may be redacted in
support bundles; raw peer tables and unbounded endpoint lists are not status
requirements.

These fields document opt-in listener/admission evidence only. They do not
promote public inbound defaults, transaction relay, compact block relay,
mempool forwarding, address propagation, permission classes, eviction, ban
policy, broader DoS/resource governance, or production full-node readiness.

## Phase 91 peer permission status

Phase 91 extends `OpenBitcoinStatusSnapshot.peers.inbound` with bounded
permission evidence. The shared fields are:

- `permission_class`: a low-cardinality machine class such as
  `ordinary_inbound`, `permissioned_inbound`, or `protected_inbound`.
- `permissioned_inbound_peers` and `protected_inbound_peers`: aggregate counts
  only, not peer tables.
- `active_permission_effects`: bounded labels for admission protection,
  eviction-policy input, misbehavior-policy input, address-response policy
  input, download-serving policy input, and diagnostics.
- `inactive_permission_effects`: bounded labels for deferred behavior such as
  `inactive_relay`, `inactive_forcerelay`, `inactive_mempool`,
  `inactive_bloomfilter`, and `inactive_blockfilters`.
- `latest_permission_decision`: the latest bounded decision reason, machine
  class, active effects, inactive effects, and sanitized message.

Support bundles and status renderers must preserve these labels from the shared
snapshot. They must not expose raw permission class names, raw permission
strings, peer ids, unbounded endpoints, or credential material. Inactive relay,
forcerelay, mempool, bloomfilter, and blockfilters labels are diagnostic
evidence only; they do not claim transaction relay, mempool propagation, BIP37
bloom serving, compact-filter serving, compact block relay, full address relay,
public inbound defaults, or production node readiness.

## Phase 92 address advertisement and discovery status

Phase 92 extends `OpenBitcoinStatusSnapshot.peers.inbound` with bounded address
evidence sourced from the network/domain address-boundary policy and projected
through managed networking. The shared fields are:

- `local_advertisement_candidates`: low-cardinality evidence entries for local
  listener-derived candidates accepted by the advertisement policy.
- `suppressed_advertisements`: bounded decision events for local listener
  candidates rejected with reasons such as `not_publicly_routable`.
- `getaddr_responses_served` and `getaddr_requests_suppressed`: counters for
  bounded getaddr direct response handling.
- `learned_address_entries` and `learned_address_rejections`: aggregate counts
  for typed inbound `addr` intake and learned-address policy decisions.
- `latest_address_decision`: the latest bounded address-boundary event with
  outcome, reason, label, source, and sanitized message.

Status consumers should preserve these field names in JSON and render them from
the shared snapshot in human output. They should not derive address summaries in
CLI, support, dashboard, RPC, log, or metric layers. The fields separate local
listener advertisement, direct bounded getaddr handling, and learned-address
storage evidence from broader peer discovery and relay claims. They do not
claim peer discovery support, full address relay support, public inbound by
default, DNS seed discovery, UPnP/NAT-PMP discovery, public-network CI, or
production full-node readiness. Use `full_relay_deferred` for the deferred
relay boundary when a no-claim label is needed.

## Reorg and no-progress semantics

Phase 70 keeps branch competition, reorg recovery, peer recovery, and
no-progress diagnosis in the shared `sync` status object. Consumers must render
these fields from `OpenBitcoinStatusSnapshot` rather than reclassifying them in
CLI, dashboard, RPC, log, metric, or support-bundle layers.

`sync.latest_reorg` is a `FieldAvailability<SyncReorgEvidence>`. When
available, it carries bounded evidence only:

- `sync.latest_reorg.common_ancestor_height`
- `sync.latest_reorg.common_ancestor_hash`
- `sync.latest_reorg.disconnected_count`
- `sync.latest_reorg.connected_count`
- `sync.latest_reorg.final_active_height`
- `sync.latest_reorg.final_active_hash`
- `sync.latest_reorg.fully_persisted`

`sync.reconcile_progress` is a `FieldAvailability<SyncReconcileProgress>`.
Current labels include `branch_competition_awaiting_bodies` when a better
branch has been selected by cumulative work but required replacement block
bodies are not yet durable, and `reorg_persisted` when a reorg completed and
bounded latest evidence is available. A missing active-chain block body, missing
undo record, malformed stored chainstate, or storage write failure remains a
storage/recovery blocker rather than peer retry advice.

`sync.no_progress_diagnosis` is a `FieldAvailability<NoProgressDiagnosis>`.
Current labels are:

- `current_at_best_known_tip`
- `behind_awaiting_headers`
- `awaiting_block_bodies`
- `stale_inflight_cleanup`
- `peer_backoff`
- `peer_stalled`
- `peer_failures_exhausted`
- `branch_competition_awaiting_bodies`
- `recovering_from_reorg_or_storage`
- `storage_or_resource_blocked`

`sync.no_progress_next_action` is the bounded human guidance paired with the
diagnosis. Storage and resource blockers take precedence over peer guidance;
branch competition waiting for replacement bodies stays distinct from current
at-tip evidence; and stale in-flight work remains visible until cleanup or
reassignment occurs. These fields are deterministic status evidence, not a
public-network or production-node readiness claim.

## Sync resource pressure

`sync.resource_pressure` is the rendered `SyncResourcePressure` contract. It
reports observed pressure and configured bounds
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

## Phase 76 resource bounds

`resource_bounds` is the top-level `FieldAvailability<ResourceBoundSnapshot>`
for disk and local runtime resource evidence. It is adjacent to
`sync.resource_pressure`: `sync.resource_pressure` remains the sync/network
in-flight envelope, while `resource_bounds` carries the disk, file, cache,
queue, peer, in-flight, log, metric, and support-bundle resource set used by
status, dashboard, soak, and support surfaces.

Each resource-bound entry preserves its `kind`, label, current usage, optional
limit, unit, warning threshold, stop threshold, pressure state, and next action.
Pressure states are `normal`, `warning`, and `stop_required`; default thresholds
are `RESOURCE_BOUND_WARNING_PERCENT = 80` and
`RESOURCE_BOUND_STOP_PERCENT = 95`. Missing required measurements remain
unavailable evidence with a reason, and soak preflight refuses missing,
unassessable, or stop-required resource evidence before writing a ledger.

Soak reports keep resource-bound state, labels, next action, and source status
evidence in checkpoint projections. Support evidence keeps the same data as a
compact summary and projected support-bundle footprint; raw logs, stores,
status snapshots, peer tables, and live-smoke inputs are not embedded.

## Phase 77 corruption and lock recovery hardening

`recovery_evidence: FieldAvailability<RecoveryEvidenceSnapshot>` is the
top-level Phase 77 contract for lock, corruption, schema, partial-write,
unreadable-store, backend-open, and resource-pressure diagnosis. It is adjacent
to `sync.recovery_category` and `sync.recovery_action`: those legacy sync
fields remain compatibility summaries, while `recovery_evidence` carries the
shared status, support, dashboard, and soak evidence used for typed recovery
guidance even when durable sync state is unavailable.

The stable Phase 77 action classes are:

- `safe_retry`
- `read_only_inspection`
- `backup_then_rebuild`
- `stop_and_escalate`

The stable causes are:

- `schema_mismatch`
- `corruption_marker`
- `corrupt_record`
- `partial_write`
- `unreadable_namespace`
- `backend_open_failure`
- `active_lock`
- `stale_lock_evidence`
- `concurrent_datadir_use`
- `resource_pressure`

The compatibility categories remain:

- `incompatible_schema`
- `store_corruption`
- `storage_lock_contention`
- `storage_backend_failure`
- `resource_exhaustion`

Unavailable recovery evidence must remain explicit through
`FieldAvailability::Unavailable` and its reason. Consumers must not infer a
healthy store, clear a lock, delete a recovery marker, compact, reindex,
relocate, repair, or mutate a source datadir from missing evidence.

## Phase 78 progress guarantees and stall diagnosis

Phase 78 adds compact progress-guarantee fields to `sync` so status,
dashboard, soak checkpoint/report, support, and live-smoke projections share
one contract for PROG-01 through PROG-04:

- `progress_credit`: credited work from `validated_durable_active_chain` or
  `current_at_best_known_tip` only.
- `expected_progress_window`: retry, round, and freshness evidence for the
  current progress window.
- `no_progress_threshold`: threshold state and elapsed time since
  `last_useful_work`.
- `last_useful_work`: the most recent credited active-chain or stay-current
  evidence.
- `last_peer_contribution`: the latest bounded peer contribution, kept
  separate from credited work.
- `stall_diagnosis`: stalled subsystem, confidence, evidence basis, and next
  action.

Headers, downloaded block bodies, peer messages, in-flight requests, retries,
and report generation are rejected activity for progress-credit purposes. They
can explain `last_peer_contribution` or the `stall_diagnosis`, but they do not
advance `progress_credit`.

`stall_diagnosis.stalled_subsystem` uses stable labels including
`public_network_reachability`, `incompatible_peers`, `slow_or_stalled_peers`,
`validation`, `storage_or_resource_pressure`, `at_tip_waiting`, `operator_stop`,
and `local_shutdown`. Storage/resource and recovery evidence outrank peer retry
guidance so status does not tell an operator to rotate peers when the selected
datadir or configured resource envelope is the blocker.

## Phase 79 shared diagnostic contract and support-forensics sidecar

Live and runtime truth remains in `OpenBitcoinStatusSnapshot`. CLI status,
dashboard status, RPC status, metrics, structured logs, soak reports,
live-smoke summaries, and support bundles must continue to render lifecycle,
progress, resource, recovery, and stall facts from that shared snapshot rather
than from renderer-local interpretations.

The `support_forensics` sidecar owns bundle-specific provenance only: source
ledger and report paths, timeline event counts, redaction summary, bundle
size/projection facts through
`resource_bound_evidence.maybe_projected_bundle_size_bytes`,
checkpoint-chain validation, and comparison metadata. Those sidecar facts can
explain how the bundle was assembled and whether its local timeline projection
is ordered and complete, but they do not replace `OpenBitcoinStatusSnapshot` as
runtime truth.

Checkpoint-chain validation is local ordering and truncation evidence. It is not
an authenticity proof, signing scheme, public-network check, or external trust
root. Missing runtime facts still stay `Unavailable` with reasons; a
`support_forensics` narrative must not infer soak stability from artifact
existence, elapsed time, daemon startup, peer reachability, raw logs, or stale
reports.

## Phase 72 evidence comparison fields

Phase 72 keeps operator observability and support evidence as projections of the
shared status snapshot. Cross-surface comparison uses these exact machine field
names when a surface exposes the corresponding fact:

- `validated_active_chain_height`
- `maybe_validated_active_chain_hash`
- `maybe_validated_active_chain_work`
- `best_known_tip`
- `stay_current`
- `no_progress_diagnosis`
- `no_progress_next_action`
- `latest_reorg`
- `reconcile_progress`
- `resource_pressure`
- `peer_contribution`
- `latest_stop_reason`
- `evidence_verdict`

Unavailable fields remain evidence. Human renderers should preserve
`Unavailable: {reason}`, while JSON/report surfaces should preserve the
equivalent unavailable-reason path instead of silently dropping unsupported
facts. Support verdicts and live-smoke summaries must compare exposed values to
the same snapshot values rather than treating field-name presence as proof.

## Phase 75 soak ledger vocabulary

Phase 75 soak evidence is a ledger/report layer over the shared status
snapshot, not a replacement for sync status. The ledger event kinds are
`started`, `checkpoint`, `resume`, `stop`, and `verdict`. Consumers may render
them with local labels, but JSON and report contracts should preserve those
machine names.

The final soak outcome vocabulary is `clean_completion`, `diagnosed_blocker`,
`operator_stop`, `resource_stop`, `recovery_stop`, and
`unexpected_termination`. These outcomes summarize the soak run and may point
to `latest_stop_reason`, `recovery_category`, `no_progress_diagnosis`,
`resource_pressure`, and `evidence_verdict` as source evidence. They must not
redefine or overload lower-level sync stop reasons or recovery categories.

Phase 71 binds this status contract to deterministic restart/resume evidence:
`phase71_same_datadir_resume_matrix_covers_clean_unclean_mid_download_mid_connect_and_stale_inflight`
covers clean shutdown, unclean shutdown, mid-download interruption,
mid-connect interruption, and stale in-flight cleanup for one selected datadir,
while
`phase71_synthetic_long_chain_exercises_resource_bounds_without_public_network`
exercises bounded long-chain progress without public-network peers. Low-disk
and storage-pressure backend failures surface as
`SyncRecoveryCategory::ResourceExhaustion` with
`StorageRecoveryAction::FreeDisk`; consumers must keep that storage/resource
blocker distinct from peer retry guidance.

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
