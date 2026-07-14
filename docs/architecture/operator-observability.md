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

Phase 90 inbound admission metrics are low-cardinality numeric counters only.
They may include admitted, rejected, cap-reject, reserved-slot-reject,
duplicate-reject, and self-connection-reject series such as
`InboundReservedSlotRejectCount`, but they must not attach peer ids, raw
endpoints, addresses, user labels, permission classes, ban state, or relay
state as metric labels. Listener endpoints and latest admission details remain
bounded status/support evidence instead of metric dimensions.

Phase 91 permission metrics remain low-cardinality numeric counters only.
Allowed series include `InboundPermissionedAdmitCount`,
`InboundProtectedAdmitCount`, `InboundInactivePermissionEffectCount`, and
`InboundPermissionValidationFailureCount`. They count permissioned admission,
protected admission, inactive effect observations, and validation failures
without dynamic labels, raw class names, peer ids, endpoint strings, or raw
permission specs.

Phase 97 persists inbound admission, permission, peer-policy, and resource
governance counters as retained metric samples from the shared inbound status
projection. InboundPeerServingStatus aggregate counters -> fixed MetricSample values -> FjallNodeStore::append_metric_samples -> dashboard/status/support retained history.
The retained samples use fixed `MetricKind` values only; dashboard, status, and
support consumers must not derive peer labels, endpoint labels, permission-class
dimensions, or raw policy material from retained inbound metrics.

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

Phase 90 inbound listener and admission logs use stable labels including
`inbound_listener_state`, `inbound_preflight_reason`, `bound_endpoint`, and
`admission_reject_reason`. Log records should keep those labels comparable with
`OpenBitcoinStatusSnapshot.peers.inbound` and `openbitcoinnetworkstatus`, while
avoiding raw peer tables or high-cardinality metric-style labels.

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

## Phase 79 support forensics projection

CLI status, dashboard status, RPC status, metrics, structured logs, live-smoke
summaries, soak reports, and support bundles consume the shared typed
status/summary contract. `support_forensics` is a support-bundle projection over
that contract: it adds a forensic timeline, checkpoint chain, failure narrative,
source evidence, redaction facts, size bounds, timeline ordering, and
cross-surface consistency checks without redefining the underlying runtime
state.

Metrics and structured logs project bounded labels and counts, not high-cardinality forensic objects or unbounded timelines. They may expose
compact labels such as verdict, checkpoint status, source kind, redaction
status, and event counts, but full `support_forensics` narratives remain in the
local support bundle JSON and Markdown artifacts. Default verification remains
public-network-free, service-manager-free, short-running, and free of large
disk allocations.

## Phase 90 inbound serving evidence

Inbound serving observability is sourced from
`OpenBitcoinStatusSnapshot.peers.inbound` and the Open Bitcoin RPC extension
`openbitcoinnetworkstatus`. Human status, JSON status, support bundles,
structured logs, and metrics should agree on listener state, preflight reason,
bounded bound-endpoint evidence, admission totals, handshake-state totals,
rejection counters, and the latest admission event. Support bundles render a
bounded and redacted projection so operators can share diagnostics without
copying raw peer tables or unbounded endpoint lists.

This observability surface is limited to opt-in Phase 90 listener/admission
review. It keeps outbound sync evidence separate from inbound admission
evidence and does not add public inbound defaults, relay behavior, permission
classes, address relay, eviction, ban, broad DoS policy, or production readiness
claims.

## Phase 91 peer permission evidence

Peer permission observability is sourced from the shared inbound status fields
on `OpenBitcoinStatusSnapshot.peers.inbound`, `openbitcoinnetworkstatus`,
operator status output, and support bundles. The stable labels are
`permission_class`, `permissioned_inbound_peers`,
`protected_inbound_peers`, `active_permission_effects`,
`inactive_permission_effects`, and `latest_permission_decision`.

Active bounded effects are admission protection, eviction-policy input,
misbehavior-policy input, address-response policy input, download-serving
policy input, and diagnostics. Inactive/deferred effects are relay,
forcerelay, mempool, bloomfilter, and blockfilters; support bundles should
render them as inactive labels such as `inactive_relay`, `inactive_mempool`,
`inactive_bloomfilter`, and `inactive_blockfilters`. Those inactive labels are
not relay support, mempool propagation, BIP37 serving, compact-filter serving,
compact-block behavior, full address relay, ban/misbehavior enforcement, public
inbound defaults, or production readiness claims.

Support evidence keeps permission output shareable by sanitizing unknown
permission classes or effect-like strings to bounded redaction labels. It may
preserve safe machine labels and aggregate counts, but it must not copy raw
class names, raw `in,noban,...` config strings, peer ids, raw endpoint tables,
RPC password values, cookie contents, or unbounded peer details.

## Phase 100 relay activation evidence

Phase 100 relay activation evidence uses only low-cardinality policy labels:
`transaction_relay_policy_input`, `force_relay_policy_input`,
`mempool_policy_input`, `inactive_bloomfilter`, `inactive_blockfilters`,
`permission_effect_inactive`, `eligible`, `disabled`,
`activation_required`, `inbound_serving_required`, `permission_required`, and
`protected_not_relay`. These labels explain the scoped v2.0 eligibility policy
created by `relay.enabled` and `-openbitcoinrelay`; they are not transaction,
mempool, or public-network telemetry.

Observability surfaces must not expose raw permission class names, raw
permission strings, peer ids, endpoints, transaction ids, raw transaction hex,
credentials, or dynamic labels for Phase 100. Metrics, structured logs, status,
dashboard, RPC, and support evidence should preserve only bounded aggregate
counts and the fixed policy labels above.

Phase 100 evidence does not claim transaction download scheduling, orphan
handling, mempool admission, relay serving/fanout, rebroadcast, compact block
relay, bloom/filter serving, package relay, public relay by default,
public-network relay CI, production service operation, production full-node
readiness, or production-funds wallet use. Public-network relay review stays
explicit opt-in UAT outside `bash scripts/verify.sh`.

## Phase 105 relay and mempool evidence

Phase 105 observability uses one shared relay evidence contract instead of
surface-specific summaries. `OpenBitcoinStatusSnapshot.mempool.relay`,
`openbitcoinnetworkstatus.relay`, operator status, dashboard rows, support
bundles, metrics, and structured logs should agree on the same classified field
states: `implemented`, `unavailable`, `deferred`, and `intentionally_different`.

The stable counter vocabulary is `accepted_count`, `rejected_count`,
`orphaned_count`, `requested_count`, `served_count`, `announced_count`,
`suppressed_count`, `evicted_count`, `expired_count`, and
`rebroadcast_deferred_count`. Metric kinds remain fixed aggregate counters such
as `relay_accepted_count`, `relay_rejected_count`, and
`relay_rebroadcast_deferred_count`. Structured logs use the
`relay_mempool` source and the same aggregate count values.

Support-bundle evidence is shareable only after redaction. It may preserve safe
states, counter values, and bounded capability labels such as
`mempool_admission`, `local_submission_relay`, `relay_fanout`,
`relay_serving`, `rebroadcast`, and `public_relay_readiness`. It must redact
raw transaction hex, txids, wtxids, endpoints, socket-address shapes, peer
identifiers, permission strings, credentials, cookies, secrets, suspicious long
hex, and dynamic labels.

Phase 105 does not claim public propagation, compact block relay, package
relay, bloom/filter serving, public relay defaults, public-network relay CI,
production service operation, production full-node readiness,
production-service proof, production full-node readiness proof, or
production-funds wallet safety proof. Those remain future scoped surfaces.

## Phase 107 runtime activation and download eligibility evidence

Phase 107 observability keeps runtime relay activation and transaction download
eligibility in the shared relay evidence contract. The managed runtime reports
resolved activation and aggregate eligibility counts through
`OpenBitcoinStatusSnapshot.mempool.relay`, `openbitcoinnetworkstatus.relay`,
operator status, dashboard rows, support bundles, metrics, and structured logs
only when those surfaces already consume the shared contract. It does not create
per-peer public status.

The public/operator vocabulary is aggregate, sanitized, and fixed-label only:
`activation`, `download_eligibility`, and
`RelayDownloadEligibilityCounters`. Granular scheduler labels including
`relay_disabled`, `not_relay_eligible`, `inbound_serving_required`,
`permission_required`, and `protected_not_relay` remain internal typed
scheduler/test evidence unless reduced to aggregate counters. Observability
surfaces must not expose peer ids, endpoints, permission strings, raw class
names, txids, wtxids, raw transaction hex, credentials, or dynamic labels.

The deterministic inbound-serving input for Phase 107 evidence is resolved
`config.inbound.enabled`. Live listener/public-network relay proof remains
explicit opt-in UAT outside default verification. `sendrawtransaction` success
does not guarantee public propagation, and Phase 107 does not claim public
relay by default, compact block relay, package relay, bloom/filter serving,
public-network relay CI, production service operation, production full-node
readiness, production-funds wallet safety, production-funds wallet use, or
durable mempool recovery.

## Phase 108 durable mempool relay recovery evidence

Phase 108 adds recovered relay-state observability to the existing relay and
mempool evidence surfaces. `RelayRecoveryCounters` are projected as the
operator label `Relay recovery` with fixed fields `recovered_count`,
`dropped_confirmed_count`, `dropped_duplicate_count`,
`dropped_missing_parent_count`, `dropped_policy_incompatible_count`, and
`dropped_evicted_count`.

Recovered accepted records rehydrate managed mempool, relay-serving, and
fanout identity state without socket I/O or public fanout during startup.
Metrics use fixed names such as `relay_recovery_recovered_count`; structured
logs use fixed keys such as `recovered`, `dropped_confirmed`,
`dropped_duplicate`, `dropped_missing_parent`,
`dropped_policy_incompatible`, and `dropped_evicted`. Support bundles redact
sensitive recovery reasons to `redacted_relay_mempool_evidence`.

Phase 108 does not claim public relay by default, guaranteed public
propagation, compact block relay, package relay, bloom/filter serving,
public-network relay CI, production-service operation, production full-node
readiness, production-funds wallet safety/use, destructive repair, source
datadir mutation, compaction, reindexing, store surgery, or automatic support
upload.

## Phase 110 block-serving boundary evidence

Phase 110 observability is a default-off policy, status, resource, and cleanup
boundary only. `BlockServingEvidenceStatus` carries shared aggregate evidence
for activation, eligibility counters, and status counters. Operator status,
dashboard, RPC, metrics, logs, and support surfaces should consume that shared
contract when later phases render it; they must not create renderer-local
block-serving truth.

The fixed vocabulary includes config keys `block_serving.enabled` and
`block_serving.compact_relay_enabled`, CLI flags
`-openbitcoinblockserving` and `-openbitcoincompactrelay`, status labels
`validated`, `available`, `stale`, `side_chain`, `pruned`, `unavailable`,
`unvalidated`, `unknown`, and `suppressed`, resource label
`block_request_cap_reached`, and cleanup labels
`block_inflight_cleanup_released`,
`block_inflight_cleanup_peer_removed`, `block_inflight_cleanup_timeout`,
`block_inflight_cleanup_restart`, and
`block_inflight_limit_still_reached`.

Observability surfaces must not expose raw peer ids, endpoints, permission
strings, prune heights, credentials, raw block payloads, transaction payloads,
or dynamic labels. Public-network block-serving or compact-relay review remains
explicit opt-in UAT outside `bash scripts/verify.sh`. Phase 110 does not claim
full block serving responses, BIP152 implementation, compact reconstruction,
`getblocktxn`, `blocktxn`, archive-node behavior, package relay,
bloom/filter serving, compact filter serving, public block serving by default,
public-network CI, production-service operation, production full-node
readiness, or production-funds wallet use.

## Phase 116 block-relay operator evidence

Phase 116 extends the bounded, explicit, default-off block-serving and
compact-relay observability contract with
`OpenBitcoinStatusSnapshot.block_relay` and
`openbitcoinnetworkstatus.block_relay`. Operator status, dashboard rows,
support bundles, metrics, and structured logs should all agree on the same
block-serving activation/eligibility/status facts plus the compact-relay
counter groups `negotiation`, `announcement`, `reconstruction`,
`missing_transaction`, `fallback`, `in_flight`, and `cleanup`.

Metrics remain fixed aggregate counters only:
`block_served_count`, `block_serving_suppressed_count`,
`compact_announced_count`, `compact_reconstructed_count`,
`compact_missing_tx_requested_count`, `compact_fallback_count`,
`compact_malformed_count`, `compact_timeout_count`, and
`compact_cleanup_count`. Structured logs use the `block_relay` source and
fixed labels such as `block_serving_eligible`, `block_serving_suppressed`,
`compact_announced`, `compact_reconstruction_failed`,
`compact_download_timeout`, and `compact_download_peer_disconnect`.

Support bundles may preserve bounded counts and stable unavailable reasons, but
they must redact raw `cmpctblock`, `blocktxn`, `getblocktxn`, block hashes,
peer ids, endpoints, permission strings, credentials, cookies, secrets, and
dynamic labels. This remains local troubleshooting/parity-review evidence only:
it does not claim public block serving by default, BIP152 production readiness,
package relay, public-network CI, production-service operation, production
full-node readiness, or production-funds wallet use.

## Phase 121 block-relay runtime projection

Phase 121 closes the OBS-03 runtime seam by projecting the Phase 116 helpers
through `DurableSyncRuntime`. When a provider returns Available
`BlockRelayEvidenceStatus` (activation-gated from ManagedRpcContext in
`open-bitcoind`), the same sync tick that runs `persist_metrics` and summary
structured logs also appends `block_relay_metric_samples` into retained metrics
history and emits `block_relay_log_record` via `append_structured_record`.

Closed flow:
`BlockRelayEvidenceStatus -> block_relay_metric_samples / block_relay_log_record -> DurableSyncRuntime persist_metrics / structured logs`.

Unavailable status omits the block-relay family entirely (no zero-valued
availability samples). Helpers, MetricKinds, and fixed log labels stay
unchanged. This is retained local observability only: it does not claim public
block serving by default, package relay, public inbound defaults, or production
full-node readiness.

## Phase 117 v2.1 release-boundary evidence

The v2.1 release boundary keeps `block_relay` evidence aggregate-only across
status, the RPC extension, dashboard rows, metrics, logs, and support bundles.
The fixed counters and unavailable reasons may be shared; raw payloads, block
or transaction hashes, peer identifiers, endpoints, permission strings,
credentials, secrets, and dynamic labels must be redacted. No projection may
invent peer-, block-, or transaction-level detail.

This provides bounded, explicit, default-off block-serving and compact-relay
evidence only. Public-network review is optional UAT outside default
verification; public serving or relay defaults, archive-node and
production-scale historical serving, production service/deployment, and
production readiness remain deferred.

## Phase 92 address advertisement and discovery evidence

Address-boundary observability is sourced from the shared inbound status fields
on `OpenBitcoinStatusSnapshot.peers.inbound`, `openbitcoinnetworkstatus`,
operator status output, and redacted support bundles. The stable field and
label vocabulary is `local_advertisement_candidates`,
`suppressed_advertisements`, `not_publicly_routable`, bounded getaddr,
`learned_address_entries`, `latest_address_decision`, and
`full_relay_deferred`.

Local advertisement evidence means configured listener endpoints and
runtime-bound listener evidence passed through the Phase 92 policy. Suppressed
advertisement evidence means the policy rejected a local candidate before it
could appear in a version sender address or direct address response. Bounded
getaddr evidence means a direct inbound `getaddr` request was served or
suppressed by count, permission, role, and served-once rules. Learned-address
evidence means typed inbound `addr` entries were accepted or rejected by the
in-memory address contract.

Metrics remain aggregate and low-cardinality; Phase 92 does not add dynamic
address labels, peer ids, raw endpoint tables, raw address bytes, raw permission
class names, or raw config strings as metric or log dimensions. Support bundles
must preserve safe machine labels and counts while redacting raw address
material. These observability fields do not claim peer discovery support, full
address relay support, public inbound by default, unsolicited addr gossip, DNS
seed discovery, UPnP/NAT-PMP discovery, or production full-node readiness.

## Phase 93 eviction, ban, and misbehavior policy evidence

Peer-policy observability is sourced from the shared inbound status fields on
`OpenBitcoinStatusSnapshot.peers.inbound`, `openbitcoinnetworkstatus`,
operator status output, and redacted support bundles. The stable field and
label vocabulary is `eviction_candidates_evaluated`, `disconnects_requested`,
`discouraged_peers`, `active_bans`, `expired_bans`, `manual_unbans`,
`misbehavior_observations`, `protected_no_actions`,
`latest_peer_policy_decision`, `eviction_candidate_selected`,
`eviction_suppressed`, `misbehavior_policy_decision`,
`source_eviction_policy`, and `source_misbehavior_policy`.

Eviction evidence means inbound peers were scored by the deterministic
peer-policy core and either selected as bounded disconnect candidates or
suppressed because no unprotected candidate was eligible. Misbehavior evidence
means typed observations were mapped to observe, disconnect, discourage, ban,
or protected no-action labels. Ban and unban evidence is scoped to aggregate
counter and latest-event labels rather than raw banlist entries.

Metrics remain aggregate and low-cardinality; Phase 93 does not add dynamic
peer ids, raw endpoints, raw ban scopes, raw permission class names, or raw
config strings as metric or log dimensions. Support bundles must preserve safe
machine labels and counts while redacting raw peer-policy material. These
observability fields do not claim production banlist parity, public ban
enforcement, Knots discourage parity, broad DoS/resource governance,
transaction relay abuse handling, public inbound by default, or production
full-node readiness.

## Phase 94 resource-governance evidence

Resource-governance observability is sourced from the shared inbound status
fields on `OpenBitcoinStatusSnapshot.peers.inbound`,
`openbitcoinnetworkstatus`, operator status output, structured logs, fixed
metrics, and redacted support bundles. The stable field vocabulary is
`latest_resource_governance_decision`, `payload_rejections`,
`timeout_disconnects`, `churn_rejections`, and `reconnect_suppressions`.

The structured log source is `inbound_resource_governance`. Structured log
records preserve the allowlisted key/value fields `outcome`, `reason`, `label`,
`source`, `message`, and `next_action`. Structured logs must not include raw peer ids, endpoints, payload bytes, permission strings, credentials, or dynamic labels.

Metrics remain fixed aggregate counters with no runtime-created dimensions.
The Phase 94 metric names are `inbound_resource_pressure_active_count`,
`inbound_read_queue_pressure_count`, `inbound_write_queue_pressure_count`,
`inbound_request_cap_reached_count`, `inbound_payload_rejected_count`,
`inbound_timeout_disconnect_count`, `inbound_churn_rejected_count`, and
`inbound_reconnect_suppressed_count`.

Support evidence should preserve bounded counters and the latest safe decision
while keeping default verification loopback/synthetic and public-network-free.
This evidence documents bounded resource-governance review only and does not
expand listener exposure or release claims.

## Phase Boundaries

Phase 13 defines serializable contracts only. It must not install a tracing subscriber, create a file appender, write metric samples, prune log files, or render dashboard graphs. Runtime writers and readers are Phase 16 responsibilities.
