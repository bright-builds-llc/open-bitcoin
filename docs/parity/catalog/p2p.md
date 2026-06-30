# P2P Networking And Sync

This entry tracks the Phase 6 networking slice implemented in Open Bitcoin.
The behavioral baseline remains Bitcoin Knots `29.3.knots20260210`.

## Coverage

- pure-core peer lifecycle state for inbound and outbound peers
- strict Bitcoin message-envelope handling with command, payload-size, and
  checksum validation
- typed payload support for `version`, `verack`, `wtxidrelay`, `sendheaders`,
  `ping`, `pong`, `getheaders`, `headers`, `inv`, `getdata`, `tx`, `block`,
  and `notfound`
- deterministic header-store tracking, best-chain locator construction, and
  header-first block sync decisions
- block announcements that trigger `getheaders`, then `getdata` for missing
  blocks once headers connect
- txid or wtxid-aware transaction announcements and requests gated by
  `wtxidrelay`
- node-side managed wrapper that feeds received blocks into managed chainstate
  and received transactions into the managed mempool
- hermetic encoded-message fixtures covering handshake, initial sync, and relay
- hermetic compatibility transcript harness for outbound `version`, `verack`,
  `sendheaders`, `wtxidrelay`, `getheaders`, and `getdata` comparison against
  the pinned Knots baseline
- typed compatibility diagnoses including `VersionRejected`, `NetworkMismatch`,
  `ServiceBitMismatch`, `UnsupportedMessageOrder`, `Timeout`, `PeerDisconnect`,
  `MalformedPayload`, and `LocalConfigurationFailure`
- daemon sync treats completed outbound `version`/`verack` handshakes from
  manual or DNS peers as connected even when the peer idles before sending
  headers or blocks
- daemon sync records incompatible peer outcomes as typed failures without
  crediting accepted header or block progress, then continues to replacement
  peers when configured
- daemon sync advances validated headers through bounded multi-batch rounds,
  stops on target header height, no progress, or max-round diagnosis, and keeps
  durable header progress visible through sync status after restart
- bounded daemon block download requests, tracks, and caps in-flight best-chain
  block bodies, then records first validated block connect evidence through
  opt-in live-smoke status snapshots
- opt-in same-datadir restart/resume evidence records
  `result.restartResumeEvidence` with before/after durable header, downloaded
  block, and connected block status after a fresh daemon launch
- service-supervised restart/resume evidence exposes `service.restart_resume`
  with same-datadir durable progress, clean or unclean prior shutdown,
  stale in-flight verdict, recovery category, and next action for opt-in
  launchd/systemd operator review
- v1.4 opt-in outbound IBD evidence keeps outbound peer compatibility, header
  progress, downloaded block progress, connected block progress, and
  restart/resume evidence reviewable without claiming broader P2P service
  readiness
- v1.5 release-boundary evidence keeps unattended mainnet operator review,
  service restart/resume, support evidence, compatibility reports, and recovery
  states auditable without claiming inbound serving, transaction relay, compact
  block relay, or production-node operation
- v1.6 release-boundary evidence keeps explicit opt-in full-sync completion,
  best-known-tip review, stay-current state, and public-mainnet UAT auditable
  without claiming inbound serving, address relay, block serving, transaction
  relay, compact block relay, public-network CI, release-blocking live sync, or
  broad production-node readiness
- Phase 75 `phase75-multi-day-soak-runner-evidence-ledger` evidence keeps
  bounded opt-in full-sync soak commands, same-datadir resume records, typed
  final outcomes, and deterministic synthetic replay auditable without
  claiming inbound serving, relay, public-network CI, or broad production-node
  readiness
- Phase 78 `phase78-progress-guarantees-stall-diagnosis` evidence keeps
  PROG-01, PROG-02, PROG-03, and PROG-04 auditable by separating peer
  contribution, retry, in-flight, and message evidence from credited progress
  while surfacing public-network, incompatible-peer, slow-peer, validation,
  at-tip, storage, operator-stop, and local-shutdown stall labels
- Phase 79 `phase79-diagnostics-support-bundle-forensics` evidence keeps
  DIAG-01, DIAG-02, DIAG-03, and DIAG-04 auditable by projecting local
  `support_forensics`, forensic timeline, checkpoint chain, failure narrative,
  likely cause, evidence basis, next action, confidence, redaction, size
  bounds, timeline ordering, and cross-surface consistency without adding P2P
  serving or relay claims
- Phase 70 no-credit peer responses retain typed attribution, release stale
  in-flight block work, and rotate through endpoint-keyed backoff without
  claiming peer banning, inbound reputation, address-manager governance, or
  relay readiness
- Phase 90 `v1-9-inbound-listener-admission-policy` evidence keeps INB-01,
  INB-02, INB-03, INB-04, and INB-05 auditable through disabled-by-default
  Open Bitcoin-owned listener controls, loopback-first listener UAT, typed
  preflight/admission evidence, managed inbound/outbound counts, shared
  inbound status, RPC status, metrics/log labels, support evidence, and source
  breadcrumbs without adding relay, permission-class, address-relay,
  eviction, ban, broad DoS policy, public default, or production readiness
  claims
- Phase 91 `v1-9-peer-permissions-connection-classes` evidence keeps PERM-01,
  PERM-02, PERM-03, and PERM-04 auditable through Open Bitcoin-owned
  permission-class config, literal-IP matching, bounded connection classes,
  active admission/download policy inputs, inactive relay-like effect labels,
  shared status/support evidence, and negative peer tests without adding Knots
  `whitelist` or `whitebind` compatibility, transaction relay, compact block
  relay, mempool propagation, BIP37 or compact-filter serving, full address
  relay, ban/misbehavior semantics, public inbound defaults, or production
  readiness claims
- Phase 92 `v1-9-address-advertisement-discovery-boundaries` evidence keeps
  ADDR-01, ADDR-02, ADDR-03, and ADDR-04 auditable through local listener
  advertisement policy, direct bounded getaddr handling, typed learned-address
  storage evidence, shared status/support fields, and source breadcrumbs
  without adding peer discovery support, full address relay support, public
  inbound by default, DNS seed discovery, UPnP/NAT-PMP discovery, or
  production full-node readiness
- Phase 94 `v1-9-dos-resource-governance` evidence keeps DOS-01, DOS-02,
  DOS-03, DOS-04, and DOS-05 auditable through bounded message-envelope
  rejection, queue/request pressure, timeout/churn/reconnect decisions, shared
  status/support fields, fixed metrics, structured logs, and loopback UAT
  command forms
- Phase 95 `v1-9-network-participation-release-boundary` evidence keeps
  BOUND-01, BOUND-02, BOUND-03, BOUND-04, BOUND-05, and BOUND-06 auditable
  through the existing parity roots, release-readiness matrix, support
  redaction roots, deterministic verification references, and 28/28 v1.9
  requirement traceability
- Phase 100 `v2-0-relay-activation-boundary` evidence keeps ACT-01, ACT-02,
  ACT-03, and ACT-04 auditable through default-off `relay.enabled` and
  `-openbitcoinrelay` activation, bounded peer eligibility reasons,
  `transaction_relay_policy_input`, `force_relay_policy_input`,
  `mempool_policy_input`, `inactive_bloomfilter`, and
  `inactive_blockfilters` without adding transaction download scheduling,
  orphan handling, mempool admission, relay serving/fanout, rebroadcast,
  compact block relay, bloom/filter serving, package relay, public relay by
  default, public-network relay CI, production service operation, production
  full-node readiness, or production-funds wallet use
- Phase 101 `v2-0-transaction-inventory-download-scheduling` evidence keeps
  INV-01, INV-02, INV-03, INV-04, DL-01, and DL-02 auditable through typed
  `TxRelayId` transaction inventory identity, `TxDownloadScheduler` request
  scheduling, `TxDownloadSuppressionReason::MempoolKnown`, bounded
  `request_getdata`, `suppress_duplicate`, `suppress_already_have`,
  `suppress_recent_reject`, `suppress_mempool_known`, `mempool_known`,
  `suppress_identity_mismatch`, `suppress_request_cap`, `fallback_request`,
  `request_expired`, `notfound_cleanup`, `received_tx_cleanup`, and
  `peer_cleanup` labels, plus
  `PHASE101_MAX_TX_REQUESTS_IN_FLIGHT_PER_PEER` and
  `PHASE101_GETDATA_TX_INTERVAL_SECONDS` constants. Phase 101 does not claim
  orphan handling, parent request behavior, mempool admission outcomes,
  standardness or fee policy, RBF, ancestor or descendant policy, mempool
  lifecycle or persistence, block connect/disconnect mempool behavior, relay
  serving/fanout, rebroadcast, RPC/operator/support surfaces, compact block
  relay, package relay, bloom/filter serving, public relay by default,
  public-network relay CI, production service operation, production full-node
  readiness, or production-funds wallet use.

## Knots sources

- [`packages/bitcoin-knots/src/protocol.h`](../../../packages/bitcoin-knots/src/protocol.h)
- [`packages/bitcoin-knots/src/protocol.cpp`](../../../packages/bitcoin-knots/src/protocol.cpp)
- [`packages/bitcoin-knots/src/netaddress.h`](../../../packages/bitcoin-knots/src/netaddress.h)
- [`packages/bitcoin-knots/src/netaddress.cpp`](../../../packages/bitcoin-knots/src/netaddress.cpp)
- [`packages/bitcoin-knots/src/net_permissions.h`](../../../packages/bitcoin-knots/src/net_permissions.h)
- [`packages/bitcoin-knots/src/net_permissions.cpp`](../../../packages/bitcoin-knots/src/net_permissions.cpp)
- [`packages/bitcoin-knots/src/net.cpp`](../../../packages/bitcoin-knots/src/net.cpp)
- [`packages/bitcoin-knots/src/net_processing.cpp`](../../../packages/bitcoin-knots/src/net_processing.cpp)
- [`packages/bitcoin-knots/src/banman.cpp`](../../../packages/bitcoin-knots/src/banman.cpp)
- [`packages/bitcoin-knots/src/addrman.h`](../../../packages/bitcoin-knots/src/addrman.h)
- [`packages/bitcoin-knots/src/addrman.cpp`](../../../packages/bitcoin-knots/src/addrman.cpp)
- [`packages/bitcoin-knots/src/addrdb.h`](../../../packages/bitcoin-knots/src/addrdb.h)
- [`packages/bitcoin-knots/src/addrdb.cpp`](../../../packages/bitcoin-knots/src/addrdb.cpp)
- [`packages/bitcoin-knots/src/headerssync.h`](../../../packages/bitcoin-knots/src/headerssync.h)
- [`packages/bitcoin-knots/src/headerssync.cpp`](../../../packages/bitcoin-knots/src/headerssync.cpp)
- [`packages/bitcoin-knots/src/sync.cpp`](../../../packages/bitcoin-knots/src/sync.cpp)
- [`packages/bitcoin-knots/src/node/txdownloadman.h`](../../../packages/bitcoin-knots/src/node/txdownloadman.h)
- [`packages/bitcoin-knots/src/node/txdownloadman_impl.cpp`](../../../packages/bitcoin-knots/src/node/txdownloadman_impl.cpp)
- [`packages/bitcoin-knots/src/txrequest.h`](../../../packages/bitcoin-knots/src/txrequest.h)
- [`packages/bitcoin-knots/src/txrequest.cpp`](../../../packages/bitcoin-knots/src/txrequest.cpp)
- [`packages/bitcoin-knots/src/test/peerman_tests.cpp`](../../../packages/bitcoin-knots/src/test/peerman_tests.cpp)
- [`packages/bitcoin-knots/test/functional/p2p_handshake.py`](../../../packages/bitcoin-knots/test/functional/p2p_handshake.py)
- [`packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py`](../../../packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py)
- [`packages/bitcoin-knots/test/functional/p2p_tx_download.py`](../../../packages/bitcoin-knots/test/functional/p2p_tx_download.py)
- [`packages/bitcoin-knots/test/functional/p2p_getdata.py`](../../../packages/bitcoin-knots/test/functional/p2p_getdata.py)
- [`packages/bitcoin-knots/test/functional/p2p_permissions.py`](../../../packages/bitcoin-knots/test/functional/p2p_permissions.py)
- [`packages/bitcoin-knots/test/functional/p2p_getaddr_caching.py`](../../../packages/bitcoin-knots/test/functional/p2p_getaddr_caching.py)
- [`packages/bitcoin-knots/test/functional/p2p_invalid_messages.py`](../../../packages/bitcoin-knots/test/functional/p2p_invalid_messages.py)
- [`packages/bitcoin-knots/test/functional/p2p_dos_header_tree.py`](../../../packages/bitcoin-knots/test/functional/p2p_dos_header_tree.py)
- [`packages/bitcoin-knots/test/functional/p2p_timeouts.py`](../../../packages/bitcoin-knots/test/functional/p2p_timeouts.py)
- [`packages/bitcoin-knots/test/functional/p2p_ibd_stalling.py`](../../../packages/bitcoin-knots/test/functional/p2p_ibd_stalling.py)
- [`packages/bitcoin-knots/test/functional/p2p_getdata.py`](../../../packages/bitcoin-knots/test/functional/p2p_getdata.py)
- [`packages/bitcoin-knots/test/functional/p2p_addrfetch.py`](../../../packages/bitcoin-knots/test/functional/p2p_addrfetch.py)
- [`packages/bitcoin-knots/test/functional/p2p_addr_relay.py`](../../../packages/bitcoin-knots/test/functional/p2p_addr_relay.py)
- [`packages/bitcoin-knots/test/functional/p2p_addrv2_relay.py`](../../../packages/bitcoin-knots/test/functional/p2p_addrv2_relay.py)
- [`packages/bitcoin-knots/test/functional/feature_addrman.py`](../../../packages/bitcoin-knots/test/functional/feature_addrman.py)
- [`packages/bitcoin-knots/test/functional/test_framework/messages.py`](../../../packages/bitcoin-knots/test/functional/test_framework/messages.py)

## Knots behaviors mirrored here

- the version handshake remains explicit and message-driven: `version`,
  optional capability messages, then `verack`
- `sendheaders` preference changes block announcement behavior from `inv`
  towards direct header announcements
- unknown block announcements trigger locator-driven header sync before block
  download
- accepted headers produce explicit `getdata` requests for blocks the local
  node still lacks
- tx relay keeps txid or wtxid identity visible and switches request or
  announcement type when `wtxidrelay` is negotiated
- ping or pong preserves the nonce across the round-trip so peer liveness stays
  inspectable
- the compatibility transcript harness remains deterministic by default and
  reports failed peer transcripts without useful-progress credit; public-network
  checks remain opt-in operator evidence outside `bash scripts/verify.sh`
- completed outbound daemon handshakes now fill compatible peer slots while
  duplicate-version, malformed-data, and wrong-network peers remain rejected and
  uncredited
- bounded daemon header sync now records first-header-progress evidence in
  opt-in live-smoke reports while deterministic tests cover accepted batches,
  rejected headers, no-progress diagnosis, and restart-visible status
- bounded daemon block download mirrors Knots-style `getdata`/`notfound`
  attribution while duplicate, invalid, malformed, non-extending,
  and disconnected no-credit block responses stay peer-attributed without
  advancing active chainstate; public-network checks remain opt-in operator
  evidence outside `bash scripts/verify.sh`
- same-datadir restart/resume review remains explicit operator evidence:
  `--restart-after-progress` stops the first daemon after observed progress,
  relaunches with the same selected datadir, and records
  `restartResumeEvidence` without turning the daemon into an unattended
  production full-sync service
- service-supervised restart review keeps the same boundary: launchd/systemd
  operators can inspect `service.restart_resume` after an explicit service
  restart, while real service-manager and public-network checks remain opt-in
  UAT outside default verification
- Phase 70 peer recovery keeps `notfound`, malformed, invalid, duplicate,
  disconnected, and non-extending block responses peer-attributed without
  useful-progress credit. Those responses release stale in-flight work for the
  affected block, preserve typed attribution in peer outcomes, and allow the
  runtime to try another eligible endpoint through endpoint-keyed backoff.
- Phase 71 bounded-resource restart/resume evidence keeps outbound daemon
  review explicit and deterministic: local tests cover resource envelopes,
  same-datadir resume, stale in-flight cleanup, and storage-pressure recovery
  without making public-network checks part of default verification.

## v1.4 release boundary

The shipped v1.4 P2P claim is opt-in outbound IBD evidence only. It is evidence
that a source-built operator can review outbound peer compatibility, validated
header progress, downloaded block progress, connected block progress,
same-datadir restart/resume evidence, or a typed diagnosed blocker through
local reports and support evidence.

The v1.4 P2P catalog does not claim inbound serving, transaction relay,
production-funds wallet use, migration apply mode, packaging, hosted dashboard,
GUI, Windows service support, or unattended production-node operation. Those
surfaces require future scoped phases and fresh parity/threat-model roots before
they can become shipped claims.

## v1.5 service restart boundary

Phase 64 adds service-supervised restart/resume evidence for the opt-in
unattended mainnet review workflow. The evidence is limited to a source-built
operator intentionally installing or running the user-level service, restarting
it, and then reviewing `service.restart_resume`, `sync.recovery_category`, and
downloaded or connected block progress from the same selected Open Bitcoin
datadir.

This service-supervised restart evidence does not make public-network checks,
`systemctl --user restart`, `launchctl kickstart`, or
`--restart-after-progress` part of default verification. It also does not claim
inbound serving, transaction relay, production-funds wallet use, migration apply
mode, packaging, Windows service support, hosted dashboard, GUI, or a broad
production-node service guarantee.

## v1.5 support bundle/operator review evidence

Phase 65 adds redacted local support evidence and operator review docs for the
opt-in unattended mainnet review workflow. The accepted local artifacts are
`support-evidence.json` and `support-evidence.md`, which summarize shared status,
service restart/resume evidence, metrics/log availability, recovery categories,
resource pressure, config sources, and compact live-smoke facts without copying
raw daemon logs, raw peer tables, endpoint tables, credential contents, wallet
material, or raw local report artifacts.

This support bundle/operator review evidence stays opt-in UAT outside default
verification. Bundle existence is not proof of sync success, service readiness,
or a production-node service guarantee; reviewers must inspect the specific
fields and keep public-network and real service-manager checks outside
`bash scripts/verify.sh`.

## v1.5 compatibility harness operator wrapper

Phase 66 adds `open-bitcoin compatibility harness` as an operator-facing wrapper
around the deterministic Phase 54 compatibility transcript harness. The accepted
local artifacts are `compatibility-harness-report.json` and
`compatibility-harness-report.md`, which record the supplied peer endpoint
label, selected network, scenario, negotiated capabilities, failing step,
diagnosis, transcript summary, redaction boundaries, and next action while
delegating diagnosis to `open-bitcoin-network::evaluate_transcript`.

This wrapper evidence stays opt-in local compatibility evidence outside default
verification. Report existence is not proof that a public peer was contacted,
that inbound serving works, that relay behavior is complete, or that Open
Bitcoin is a production-node service guarantee; public-network probing remains
explicit UAT outside `bash scripts/verify.sh`.

## v1.5 unattended operation release boundary

Phase 67 closes the v1.5 P2P release boundary as source-built, explicit opt-in
extended unattended mainnet operator review readiness. P2P evidence may include
bounded daemon loop status, typed resource pressure, recovery category/action,
same-datadir service restart/resume, redacted support summaries, and local
compatibility wrapper reports.

This is not an inbound serving, address advertisement, transaction relay,
compact block relay, public-peer probing, or production-node support claim.
Public-network live-smoke, manual-peer probing, restart-after-progress, and real
launchd/systemd manager calls remain explicit operator UAT outside
`bash scripts/verify.sh`.

## Phase 71 resource and restart boundary

Phase 71 remains outbound explicit opt-in full-sync review. It documents and
checks local evidence for bounded peers, in-flight blocks, request queues, retry
maps, cache retention, synchronous storage writes, metrics/log retention,
support evidence compactness, and same-datadir restart/resume behavior.

Phase 71 does not add inbound serving, address relay, block serving,
transaction relay, compact block relay, production-funds wallet claims,
migration apply mode, signed packaging, Windows service support, GUI, hosted
dashboards, or broad production-node readiness. Public-network long-run review,
manual peers, service-manager restarts, and restart-after-progress evidence
remain explicit operator UAT outside `bash scripts/verify.sh`.

## Phase 72 observability and support evidence boundary

Phase 72 adds observability/support evidence only. It aligns CLI status,
dashboard, RPC durable sync status, metrics, structured logs, live-smoke
reports, and support bundles around shared full-sync truth fields such as
validated active-chain progress, best-known tip, stay-current state,
no-progress guidance, reorg/reconcile evidence, resource pressure, peer
contribution, latest stop reason, and typed evidence verdicts.

Phase 72 does not add inbound serving, address relay, block serving,
transaction relay, compact block relay, production-funds wallet claims,
migration apply mode, signed packaging, Windows service support, GUI, hosted
dashboards, or broad production-node readiness. Public-network long-run review,
manual peers, service-manager restarts, and restart-after-progress evidence
remain explicit operator UAT outside `bash scripts/verify.sh`.

## Phase 73 opt-in public-mainnet UAT and deterministic verification

Phase 73 documents public-mainnet full-sync, manual-peer, and
restart-after-progress commands as explicit opt-in UAT only. These commands are
outside `bash scripts/verify.sh`; default verification stays deterministic,
local, and public-network-free.

The accepted local evidence surfaces are `scripts/run-live-mainnet-smoke.ts`,
`scripts/test-run-live-mainnet-smoke.sh`, and the
`open-bitcoin compatibility harness`. They provide auditable live-smoke,
fixture, and compatibility-harness evidence without turning public-network
review into public-network CI or release-blocking live sync.

Phase 73 does not add inbound serving, address relay, block serving,
transaction relay, compact block relay, production-funds wallet claims,
migration apply mode, signed packaging, Windows service support, GUI, hosted
dashboards, broad production-node readiness, public-network CI, or
release-blocking live sync. Those remain deferred or outside-scope claims for
future phases.

## v1.6 full-sync completion release boundary

Phase 74 closes the v1.6 P2P release boundary as source-built, explicit opt-in
full-sync completion evidence. The accepted P2P-facing evidence is outbound
sync review only: validated active-chain progress to the best-known peer tip,
best-known tip freshness, stay-current state, peer-attributed no-progress
diagnosis, reorg evidence, bounded resource behavior, redacted support evidence,
and opt-in UAT commands rooted in the Phase 73 matrix.

This is not an inbound serving, address relay, block serving, transaction
relay, compact block relay, public-network CI, release-blocking live sync, or
broad production-node readiness claim. Public-network full-sync, manual-peer,
restart-after-progress, and real service-manager checks remain explicit
operator UAT outside `bash scripts/verify.sh`.

## Phase 75 multi-day soak runner evidence ledger

The `phase75-multi-day-soak-runner-evidence-ledger` surface is scoped to
operator-controlled outbound full-sync soak evidence. It records started,
checkpoint, resume, stop, and verdict events for the selected datadir and
projects JSON/Markdown reports from that ledger.

This evidence may show bounded opt-in soak behavior, durable resume evidence,
or a diagnosed blocker. It does not add inbound serving, address relay, block
serving, transaction relay, compact block relay, production-funds wallet
safety, migration apply mode, signed packages, GUI readiness, hosted
dashboards, public-network CI, release-blocking live sync, or broad
production-node readiness.

## Phase 78 progress guarantees and stall diagnosis

The `phase78-progress-guarantees-stall-diagnosis` surface is scoped to outbound
sync and soak progress evidence. P2P activity such as headers, block bodies,
peer messages, retries, and in-flight requests remains diagnostic evidence for
PROG-01 through PROG-04, but it does not advance the credited progress
watermark unless the runtime also observes validated durable active-chain
progress or explicit current-at-best-known-tip evidence.

The P2P-facing stall labels distinguish public-network reachability,
incompatible peers, slow or stalled peers, validation failures, at-tip waiting,
storage/resource pressure, operator stop, and local shutdown without adding
peer banning, address-manager governance, relay scope, public-network default
verification, or production-node readiness.

## Phase 79 diagnostics support-bundle forensics boundary

The `phase79-diagnostics-support-bundle-forensics` surface is scoped to local
support-bundle diagnosis for DIAG-01, DIAG-02, DIAG-03, and DIAG-04. P2P-facing
evidence may explain peer reachability, rejected activity, no-progress signals,
and stale report inputs through `support_forensics`, forensic timeline,
checkpoint chain, failure narrative, likely cause, evidence basis, next action,
confidence, redaction, size bounds, timeline ordering, and cross-surface
consistency.

That evidence is local forensics only. It does not add inbound serving, address
relay, block serving, transaction relay, compact block relay, production-funds
wallet use, migration apply mode, packaging, GUI, hosted dashboards,
public-network default checks, multi-day default gates, automatic support-bundle upload, or production-node readiness.

## v1.8 production claim boundary

The v1.8 production claim boundary is
[`docs/parity/production-claim-boundary.md`](../production-claim-boundary.md).
Under that boundary, inbound serving, address relay, block serving,
transaction relay, and compact block relay remain `deferred` until scoped P2P
production gates exist. Historical outbound sync, soak, and diagnostics evidence
does not satisfy those future inbound or relay gates by itself.

The Phase 83 support matrix is
[`docs/parity/support-matrix.md`](../support-matrix.md). It keeps public-network
outbound evidence at `opt-in UAT`, while inbound serving, address relay, block
serving, transaction relay, and compact block relay remain `deferred`.

## Phase 90 inbound listener and admission boundary

The `v1-9-inbound-listener-admission-policy` surface covers INB-01 through
INB-05 for opt-in inbound listener/admission review. Its Knots anchors are
[`packages/bitcoin-knots/src/net.cpp`](../../../packages/bitcoin-knots/src/net.cpp)
for bind/listen and connection-manager behavior,
[`packages/bitcoin-knots/src/net_processing.cpp`](../../../packages/bitcoin-knots/src/net_processing.cpp)
for handshake and peer-processing behavior, and
[`packages/bitcoin-knots/test/functional/p2p_handshake.py`](../../../packages/bitcoin-knots/test/functional/p2p_handshake.py)
for version/verack handshake fixture behavior.

Open Bitcoin intentionally owns the Phase 90 activation surface:

- listener enablement is disabled by default and uses `open-bitcoin.jsonc`
  `inbound.enabled`, `inbound.listen_addresses`, `inbound.max_peers`,
  `inbound.reserved_slots`, and `inbound.allow_public`
- daemon CLI overrides use Open Bitcoin-prefixed flags such as
  `-openbitcoininbound=1` and `-openbitcoinlisten=127.0.0.1:18444`
- loopback endpoints are the deterministic UAT target; wildcard or public
  endpoints require `inbound.allow_public = true` and remain outside
  `bash scripts/verify.sh`
- baseline `getnetworkinfo` keeps `connections`, `connections_in`, and
  `connections_out`, while `openbitcoinnetworkstatus` and
  `OpenBitcoinStatusSnapshot.peers.inbound` carry detailed listener and
  admission evidence
- metrics and logs expose low-cardinality counters and stable labels such as
  `inbound_listener_state`, `inbound_preflight_reason`, `bound_endpoint`, and
  `admission_reject_reason`
- support bundles render bounded and redacted inbound evidence instead of raw
  peer tables or unbounded endpoint lists

Phase 90 does not claim Phase 91+ peer permission classes, address
advertisement or address relay, eviction, ban, discourage, reputation, broad
DoS/resource governance, transaction relay, compact block relay, public
listener defaults, or production node readiness. Reserved slots are admission
evidence only until the later permission phase deliberately expands that
surface.

## Phase 91 peer permissions and connection-class boundary

The `v1-9-peer-permissions-connection-classes` surface covers PERM-01 through
PERM-04 for bounded peer permission evidence. Its Knots anchors are
[`packages/bitcoin-knots/src/net_permissions.h`](../../../packages/bitcoin-knots/src/net_permissions.h),
[`packages/bitcoin-knots/src/net_permissions.cpp`](../../../packages/bitcoin-knots/src/net_permissions.cpp),
[`packages/bitcoin-knots/test/functional/p2p_permissions.py`](../../../packages/bitcoin-knots/test/functional/p2p_permissions.py),
[`packages/bitcoin-knots/src/net.cpp`](../../../packages/bitcoin-knots/src/net.cpp),
and
[`packages/bitcoin-knots/src/net_processing.cpp`](../../../packages/bitcoin-knots/src/net_processing.cpp).

Open Bitcoin intentionally owns the Phase 91 activation surface:

- config uses `open-bitcoin.jsonc` `inbound.permission_classes` and the
  repeatable CLI override
  `-openbitcoininboundpermissionclass=<name>@<literal_ip>=<tokens>`
- class matching uses literal IP addresses only; ranges, hostnames, and
  endpoint-shaped values are rejected
- Knots `whitelist` and `whitebind` compatibility is not silently accepted
- `permissioned_inbound` and `protected_inbound` are bounded machine classes;
  protected inbound peers are the only permission class that can use reserved
  admission capacity
- active effects are admission protection, eviction-policy input,
  misbehavior-policy input, address-response policy input,
  download-serving policy input, and diagnostics
- inactive effects are `inactive_relay`, `inactive_forcerelay`,
  `inactive_mempool`, `inactive_bloomfilter`, and `inactive_blockfilters`
- `openbitcoinnetworkstatus`, `OpenBitcoinStatusSnapshot.peers.inbound`,
  operator status, metrics, and support bundles expose bounded permission
  counts, labels, and latest decisions without raw class names, raw permission
  strings, peer ids, endpoint tables, or credentials

Phase 91 does not claim Knots `whitelist` or `whitebind` compatibility,
transaction relay, compact block relay, mempool propagation, BIP37 bloom
serving, compact-filter serving, full address relay, ban or misbehavior
semantics, public inbound defaults, broad DoS/resource governance, or
production readiness.

## Phase 92 address advertisement and discovery boundary

The `v1-9-address-advertisement-discovery-boundaries` surface covers ADDR-01
through ADDR-04 for the address advertisement and discovery boundary. Its Knots
anchors are
[`packages/bitcoin-knots/src/protocol.h`](../../../packages/bitcoin-knots/src/protocol.h)
and
[`packages/bitcoin-knots/src/protocol.cpp`](../../../packages/bitcoin-knots/src/protocol.cpp)
for netaddress and legacy `addr` wire representation,
[`packages/bitcoin-knots/src/netaddress.h`](../../../packages/bitcoin-knots/src/netaddress.h)
and
[`packages/bitcoin-knots/src/netaddress.cpp`](../../../packages/bitcoin-knots/src/netaddress.cpp)
for routability and address-network classification,
[`packages/bitcoin-knots/src/net.cpp`](../../../packages/bitcoin-knots/src/net.cpp)
for local address advertisement boundaries,
[`packages/bitcoin-knots/src/net_processing.cpp`](../../../packages/bitcoin-knots/src/net_processing.cpp)
for `getaddr` and `addr` request-response behavior, and
[`packages/bitcoin-knots/src/addrman.h`](../../../packages/bitcoin-knots/src/addrman.h),
[`packages/bitcoin-knots/src/addrman.cpp`](../../../packages/bitcoin-knots/src/addrman.cpp),
[`packages/bitcoin-knots/src/addrdb.h`](../../../packages/bitcoin-knots/src/addrdb.h),
and
[`packages/bitcoin-knots/src/addrdb.cpp`](../../../packages/bitcoin-knots/src/addrdb.cpp)
as comparison anchors for intentionally bounded learned-address storage.

Open Bitcoin intentionally separates these claims:

- local listener advertisement is limited to configured listener endpoints and
  runtime-bound listener evidence. Accepted evidence appears as
  `local_advertisement_candidates`; rejected listener evidence appears as
  `suppressed_advertisements` with stable reasons such as
  `not_publicly_routable`.
- direct `getaddr` handling is bounded getaddr evidence only. The peer manager
  can answer eligible inbound requests from a deterministic local/learned cache
  and records served or suppressed counts without enabling unsolicited relay.
- learned-address storage is an in-memory typed contract with
  `learned_address_entries`, rejection counts, freshness, source, routability,
  service, port, and persistence-eligibility evidence. It does not claim Knots
  `addrman.dat`, `peers.dat`, anchors, bucket randomization, or production peer
  selection parity.
- peer discovery remains outside this surface: no peer discovery support, DNS
  seed discovery, address-fetch crawling, public peer probing, or outbound
  connection selection is claimed.
- unsolicited addr relay, addr gossip relay, rebroadcast scheduling, trickle
  relay, known-address filters, and `addrv2` relay remain
  `full_relay_deferred`; no full address relay support is claimed.
- DNS seed discovery, UPnP/NAT-PMP discovery, `-discover` and `-externalip`
  parity, public inbound defaults, public inbound by default, public-network CI,
  and production full-node readiness remain outside this surface.

## Phase 93 eviction, ban, and misbehavior policy boundary

The `v1-9-eviction-ban-misbehavior-policy` surface covers `EVICT-01`,
`EVICT-02`, `EVICT-03`, and `EVICT-04` for bounded eviction, ban, unban, and
misbehavior policy evidence.
Its Knots anchors are
[`packages/bitcoin-knots/src/net.cpp`](../../../packages/bitcoin-knots/src/net.cpp)
for connection-manager eviction and protected-peer behavior,
[`packages/bitcoin-knots/src/net_processing.cpp`](../../../packages/bitcoin-knots/src/net_processing.cpp)
for peer-processing and misbehavior comparison points,
[`packages/bitcoin-knots/src/banman.h`](../../../packages/bitcoin-knots/src/banman.h)
and
[`packages/bitcoin-knots/src/banman.cpp`](../../../packages/bitcoin-knots/src/banman.cpp)
for banlist scope and expiry behavior, and
[`packages/bitcoin-knots/src/net_permissions.cpp`](../../../packages/bitcoin-knots/src/net_permissions.cpp)
for protected permission effects.

Open Bitcoin intentionally owns the Phase 93 policy surface:

- pure network code scores inbound eviction candidates using bounded inputs
  and emits `eviction_candidate_selected` or `eviction_suppressed` labels.
- protected peers are excluded from eviction and misbehavior action paths, with
  `protected_no_actions` evidence instead of raw peer identities.
- ban and unban policy evidence uses scoped labels and expiry/manual counters
  such as `active_bans`, `expired_bans`, and `manual_unbans`.
- misbehavior observations use stable labels such as `malformed_message`,
  `duplicate_version`, `invalid_address`, `unsupported_command_abuse`,
  `header_violation`, and `misbehavior_policy_decision`.
- `openbitcoinnetworkstatus`, `OpenBitcoinStatusSnapshot.peers.inbound`,
  operator status, metrics, and support bundles expose bounded counts and the
  latest peer-policy decision without raw peer ids, raw endpoints, raw ban
  scopes, raw permission strings, or credentials.

Phase 93 does not claim production banlist parity, public ban enforcement,
Knots discourage parity, broad DoS/resource governance, resource exhaustion
coverage, transaction relay abuse handling, compact block relay abuse
handling, public inbound defaults, public-network CI, or production full-node
readiness.

## Phase 94 DoS and resource-governance boundary

The `v1-9-dos-resource-governance` surface covers `DOS-01`, `DOS-02`,
`DOS-03`, `DOS-04`, and `DOS-05` for bounded inbound DoS and
resource-governance evidence.
Its Knots anchors are
[`packages/bitcoin-knots/src/protocol.h`](../../../packages/bitcoin-knots/src/protocol.h)
for message command and size constants,
[`packages/bitcoin-knots/src/net.cpp`](../../../packages/bitcoin-knots/src/net.cpp)
for connection-manager resource and timeout comparison points,
[`packages/bitcoin-knots/src/net_processing.cpp`](../../../packages/bitcoin-knots/src/net_processing.cpp)
for request and inventory comparison points,
[`packages/bitcoin-knots/src/banman.cpp`](../../../packages/bitcoin-knots/src/banman.cpp)
for banned or discouraged reconnect comparison points,
[`packages/bitcoin-knots/src/net_permissions.cpp`](../../../packages/bitcoin-knots/src/net_permissions.cpp)
for scoped permission-effect comparison points,
[`packages/bitcoin-knots/test/functional/p2p_invalid_messages.py`](../../../packages/bitcoin-knots/test/functional/p2p_invalid_messages.py),
[`packages/bitcoin-knots/test/functional/p2p_dos_header_tree.py`](../../../packages/bitcoin-knots/test/functional/p2p_dos_header_tree.py),
[`packages/bitcoin-knots/test/functional/p2p_timeouts.py`](../../../packages/bitcoin-knots/test/functional/p2p_timeouts.py),
[`packages/bitcoin-knots/test/functional/p2p_ibd_stalling.py`](../../../packages/bitcoin-knots/test/functional/p2p_ibd_stalling.py),
and
[`packages/bitcoin-knots/test/functional/p2p_getdata.py`](../../../packages/bitcoin-knots/test/functional/p2p_getdata.py).

Open Bitcoin intentionally owns the Phase 94 evidence surface:

- message-envelope policy records stable labels for wrong network magic,
  malformed headers, oversized messages, checksum failures, unsupported
  commands, malformed messages, and trailing data.
- queue and request governance records bounded pressure labels for read queue,
  write queue, queued-message, and request-cap decisions.
- lifecycle governance records slow-handshake, idle-peer, connection-churn,
  repeated-failure, banned-reconnect, and discouraged-reconnect decisions from
  deterministic inputs.
- `openbitcoinnetworkstatus`, `OpenBitcoinStatusSnapshot.peers.inbound`,
  operator status, fixed metrics, structured logs, and support bundles expose
  bounded counts and the latest resource-governance decision.
- default verification remains deterministic, local, loopback/synthetic, and
  outside public-network execution.

Phase 94 does not claim transaction relay, compact block relay, mempool propagation, broad address relay, public inbound defaults, public-network CI, production service operation, or production full-node readiness.

## Phase 95 network participation evidence and release boundary

The `v1-9-network-participation-release-boundary` surface covers `BOUND-01`,
`BOUND-02`, `BOUND-03`, `BOUND-04`, `BOUND-05`, and `BOUND-06` for the v1.9
closeout surface. Its role is to connect the Phase 90 through Phase 94 evidence
roots to release reviewers, not to introduce a new runtime networking surface or
a competing evidence manifest.

Its Knots anchors are
[`packages/bitcoin-knots/src/net.cpp`](../../../packages/bitcoin-knots/src/net.cpp)
for listener, connection-manager, protected-slot, timeout, and resource
comparison points,
[`packages/bitcoin-knots/src/net_processing.cpp`](../../../packages/bitcoin-knots/src/net_processing.cpp)
for peer-processing, handshake, request, inventory, and misbehavior comparison
points,
[`packages/bitcoin-knots/src/addrman.cpp`](../../../packages/bitcoin-knots/src/addrman.cpp)
for address-manager and learned-address comparison points,
[`packages/bitcoin-knots/src/banman.cpp`](../../../packages/bitcoin-knots/src/banman.cpp)
for ban, unban, expiry, and discouraged-reconnect comparison points, and
[`packages/bitcoin-knots/src/net_permissions.cpp`](../../../packages/bitcoin-knots/src/net_permissions.cpp)
for permission-effect comparison points.

Release reviewers should read this surface through
[`docs/parity/index.json`](../index.json),
[`docs/parity/checklist.md`](../checklist.md),
[`docs/parity/release-readiness.md`](../release-readiness.md),
[`docs/parity/production-claim-boundary.md`](../production-claim-boundary.md),
[`docs/parity/support-matrix.md`](../support-matrix.md), and
[`docs/operator/runtime-guide.md`](../../operator/runtime-guide.md). The
deterministic Phase 95 checker paths are
[`scripts/check-phase95-network-participation-release-boundary.ts`](../../../scripts/check-phase95-network-participation-release-boundary.ts)
and
[`scripts/check-phase95-network-participation-release-boundary.test.ts`](../../../scripts/check-phase95-network-participation-release-boundary.test.ts),
with final wiring owned by the Phase 95 aggregate checker plan.

Phase 95 does not claim transaction relay, compact block relay, mempool propagation, full address relay beyond Phase 92, public inbound defaults, public-network CI, production service operation, or production full-node readiness. Those surfaces remain deferred until future milestones add scoped parity, release, support, and verification evidence.

## Phase 96 peer-policy runtime bridge

The `v1-9-peer-policy-runtime-bridge` surface covers `EVICT-03`, `EVICT-04`,
and `DOS-03` for scoped runtime peer-policy bridge evidence. It connects the
pure ban, unban, discourage, and misbehavior policy state to managed status,
bounded reconnect suppression, sanitized structured logs, CLI status, and
redacted support output.

Its Knots anchors are
[`packages/bitcoin-knots/src/net.cpp`](../../../packages/bitcoin-knots/src/net.cpp)
for connection-manager and reconnect comparison points,
[`packages/bitcoin-knots/src/net_processing.cpp`](../../../packages/bitcoin-knots/src/net_processing.cpp)
for peer-processing and misbehavior comparison points,
[`packages/bitcoin-knots/src/banman.h`](../../../packages/bitcoin-knots/src/banman.h)
and
[`packages/bitcoin-knots/src/banman.cpp`](../../../packages/bitcoin-knots/src/banman.cpp)
for ban, unban, expiry, and discouraged-reconnect comparison points, and
[`packages/bitcoin-knots/src/net_permissions.cpp`](../../../packages/bitcoin-knots/src/net_permissions.cpp)
for protected-peer permission comparison points.

Phase 96 evidence is intentionally scoped runtime peer-policy bridge evidence,
not a public banlist or production participation claim. It does not claim
transaction relay, compact block relay, mempool propagation, public inbound
defaults, public-network CI, production service operation, or production
readiness. Those surfaces remain deferred until future milestones add scoped
parity, support, release, and verification evidence.

## Phase 100 relay activation boundary

The `v2-0-relay-activation-boundary` surface covers `ACT-01`, `ACT-02`,
`ACT-03`, and `ACT-04` for explicit relay activation and peer eligibility
semantics. It is a v2.0 activation boundary only; later phases own
transaction inventory identity, transaction download scheduling, orphan
handling, mempool admission, relay serving/fanout, rebroadcast, RPC status,
metrics, logs, support evidence, and milestone release closeout.

Its Knots anchors are
[`packages/bitcoin-knots/src/net_permissions.h`](../../../packages/bitcoin-knots/src/net_permissions.h),
[`packages/bitcoin-knots/src/net_permissions.cpp`](../../../packages/bitcoin-knots/src/net_permissions.cpp),
[`packages/bitcoin-knots/src/net.cpp`](../../../packages/bitcoin-knots/src/net.cpp),
[`packages/bitcoin-knots/src/net_processing.cpp`](../../../packages/bitcoin-knots/src/net_processing.cpp),
and
[`packages/bitcoin-knots/test/functional/p2p_permissions.py`](../../../packages/bitcoin-knots/test/functional/p2p_permissions.py).

Open Bitcoin intentionally owns the Phase 100 activation surface:

- `relay.enabled` and `-openbitcoinrelay` are explicit default-off activation
  controls. They do not change service bits, peer socket behavior, or public
  relay defaults by themselves.
- Relay eligibility classifies peers with bounded reason labels:
  `eligible`, `disabled`, `activation_required`, `inbound_serving_required`,
  `permission_required`, `protected_not_relay`, and
  `permission_effect_inactive`.
- `relay`, `forcerelay`, and `mempool` permission tokens become scoped v2.0
  policy inputs exposed as `transaction_relay_policy_input`,
  `force_relay_policy_input`, and `mempool_policy_input`.
- `download` and `addr` remain their existing bounded non-relay effects, such
  as `download_serving_policy_input` and `address_response_policy_input`.
- Bloom and filter permissions remain inactive labels:
  `inactive_bloomfilter` and `inactive_blockfilters`.
- Public-network relay review remains opt-in UAT outside `bash scripts/verify.sh`.

Phase 100 does not claim compact block relay, bloom/filter serving, package
relay, public relay by default, public-network relay CI, production service
operation, production full-node readiness, production-funds wallet use,
transaction download scheduling, orphan handling, mempool admission, relay
serving/fanout, or rebroadcast. Those surfaces remain outside this phase until
future scoped requirements add implementation and evidence.

## Phase 101 transaction inventory download scheduling

The `v2-0-transaction-inventory-download-scheduling` surface covers `INV-01`,
`INV-02`, `INV-03`, `INV-04`, `DL-01`, and `DL-02` for typed transaction
inventory identity and bounded transaction download scheduling. It is a v2.0
inventory/download scheduling boundary only; later phases own orphan handling,
parent request behavior, mempool admission outcomes, relay serving/fanout,
rebroadcast, RPC/operator/support surfaces, and release closeout.

Its Knots anchors are
[`packages/bitcoin-knots/src/protocol.h`](../../../packages/bitcoin-knots/src/protocol.h),
[`packages/bitcoin-knots/src/net_processing.cpp`](../../../packages/bitcoin-knots/src/net_processing.cpp),
[`packages/bitcoin-knots/src/node/txdownloadman.h`](../../../packages/bitcoin-knots/src/node/txdownloadman.h),
[`packages/bitcoin-knots/src/node/txdownloadman_impl.cpp`](../../../packages/bitcoin-knots/src/node/txdownloadman_impl.cpp),
[`packages/bitcoin-knots/src/txrequest.h`](../../../packages/bitcoin-knots/src/txrequest.h),
[`packages/bitcoin-knots/src/txrequest.cpp`](../../../packages/bitcoin-knots/src/txrequest.cpp),
[`packages/bitcoin-knots/test/functional/p2p_tx_download.py`](../../../packages/bitcoin-knots/test/functional/p2p_tx_download.py),
and
[`packages/bitcoin-knots/test/functional/p2p_getdata.py`](../../../packages/bitcoin-knots/test/functional/p2p_getdata.py).

Open Bitcoin intentionally owns the Phase 101 inventory/download surface:

- `TxRelayId` is the typed boundary for txid and wtxid inventory.
- Types validated by this surface include `TxRelayPeerMode`,
  `TxDownloadPolicy`, `TxDownloadLocalFacts`, `TxDownloadAction`, and
  `PeerAction::TransactionRelay`.
- `TxDownloadScheduler` owns in-memory transaction request, duplicate,
  fallback, timeout, `notfound`, disconnect, and received-transaction cleanup
  state.
- `TxDownloadSuppressionReason::MempoolKnown` and the `mempool_known` reason
  keep future mempool-known facts typed without adding mempool admission
  outcomes in this phase.
- Fixed action labels are `request_getdata`, `suppress_duplicate`,
  `suppress_already_have`, `suppress_recent_reject`,
  `suppress_mempool_known`, `suppress_identity_mismatch`,
  `suppress_request_cap`, `fallback_request`, `request_expired`,
  `notfound_cleanup`, `received_tx_cleanup`, and `peer_cleanup`.
- Scheduler policy constants include
  `PHASE101_MAX_TX_ANNOUNCEMENTS_PER_PEER`,
  `PHASE101_MAX_TX_REQUESTS_IN_FLIGHT_PER_PEER` and
  `PHASE101_TXID_RELAY_DELAY_SECONDS`,
  `PHASE101_NONPREF_PEER_TX_DELAY_SECONDS`,
  `PHASE101_OVERLOADED_PEER_TX_DELAY_SECONDS`, and
  `PHASE101_GETDATA_TX_INTERVAL_SECONDS`.

Phase 101 does not claim orphan handling, parent request behavior, mempool admission outcomes, standardness or fee policy, RBF, ancestor or descendant policy, mempool lifecycle or persistence, block connect/disconnect mempool behavior, relay serving/fanout, rebroadcast, RPC/operator/support surfaces, compact block relay, package relay, bloom/filter serving, public relay by default, public-network relay CI, production service operation, production full-node readiness, or production-funds wallet use. Those surfaces remain outside this phase until future scoped requirements add implementation and evidence.

## First-party implementation

- [`packages/open-bitcoin-network/src/address.rs`](../../../packages/open-bitcoin-network/src/address.rs)
- [`packages/open-bitcoin-network/src/address/advertisement.rs`](../../../packages/open-bitcoin-network/src/address/advertisement.rs)
- [`packages/open-bitcoin-network/src/address/book.rs`](../../../packages/open-bitcoin-network/src/address/book.rs)
- [`packages/open-bitcoin-network/src/address/response.rs`](../../../packages/open-bitcoin-network/src/address/response.rs)
- [`packages/open-bitcoin-network/src/address/tests.rs`](../../../packages/open-bitcoin-network/src/address/tests.rs)
- [`packages/open-bitcoin-network/src/peer/address_boundary.rs`](../../../packages/open-bitcoin-network/src/peer/address_boundary.rs)
- [`packages/open-bitcoin-network/src/inbound.rs`](../../../packages/open-bitcoin-network/src/inbound.rs)
- [`packages/open-bitcoin-network/src/inbound/permissions.rs`](../../../packages/open-bitcoin-network/src/inbound/permissions.rs)
- [`packages/open-bitcoin-network/src/message.rs`](../../../packages/open-bitcoin-network/src/message.rs)
- [`packages/open-bitcoin-network/src/compatibility.rs`](../../../packages/open-bitcoin-network/src/compatibility.rs)
- [`packages/open-bitcoin-network/src/header_store.rs`](../../../packages/open-bitcoin-network/src/header_store.rs)
- [`packages/open-bitcoin-network/src/peer.rs`](../../../packages/open-bitcoin-network/src/peer.rs)
- [`packages/open-bitcoin-network/src/peer/policy_state.rs`](../../../packages/open-bitcoin-network/src/peer/policy_state.rs)
- [`packages/open-bitcoin-network/src/peer_policy.rs`](../../../packages/open-bitcoin-network/src/peer_policy.rs)
- [`packages/open-bitcoin-network/src/relay.rs`](../../../packages/open-bitcoin-network/src/relay.rs)
- [`packages/open-bitcoin-network/src/resource.rs`](../../../packages/open-bitcoin-network/src/resource.rs)
- [`packages/open-bitcoin-network/src/resource/tests.rs`](../../../packages/open-bitcoin-network/src/resource/tests.rs)
- [`packages/open-bitcoin-network/tests/parity.rs`](../../../packages/open-bitcoin-network/tests/parity.rs)
- [`packages/open-bitcoin-node/src/network.rs`](../../../packages/open-bitcoin-node/src/network.rs)
- [`packages/open-bitcoin-node/src/status/inbound.rs`](../../../packages/open-bitcoin-node/src/status/inbound.rs)
- [`packages/open-bitcoin-rpc/src/inbound_listener.rs`](../../../packages/open-bitcoin-rpc/src/inbound_listener.rs)
- [`packages/open-bitcoin-node/src/sync.rs`](../../../packages/open-bitcoin-node/src/sync.rs)
- [`packages/open-bitcoin-cli/src/operator/status/render/inbound.rs`](../../../packages/open-bitcoin-cli/src/operator/status/render/inbound.rs)
- [`packages/open-bitcoin-cli/src/operator/support/render/inbound.rs`](../../../packages/open-bitcoin-cli/src/operator/support/render/inbound.rs)
- [`scripts/run-live-mainnet-smoke.ts`](../../../scripts/run-live-mainnet-smoke.ts)

## Known gaps

- address relay, `addrv2`, peer discovery policy, and DNS-seed governance
- encrypted transport and other non-v1 wire transports
- compact blocks, blocktxn, filtered blocks, bloom filters, and compact filters
- resource-governance parity beyond the Phase 94 bounded evidence surface
- production daemon-integrated full-sync guarantees
- automatic public-mainnet recovery loops and broad production-node service
  guarantees
- public inbound defaults and transport persistence beyond the current explicit
  listener review surface

## Follow-up triggers

Update this entry when later phases add discovery, compact-block relay,
transport encryption, daemon-integrated sync orchestration, or
connection-governance behavior that materially changes the externally visible
networking surface.
