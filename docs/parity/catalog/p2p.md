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
- Phase 70 no-credit peer responses retain typed attribution, release stale
  in-flight block work, and rotate through endpoint-keyed backoff without
  claiming peer banning, inbound reputation, address-manager governance, or
  relay readiness

## Knots sources

- [`packages/bitcoin-knots/src/protocol.h`](../../../packages/bitcoin-knots/src/protocol.h)
- [`packages/bitcoin-knots/src/headerssync.h`](../../../packages/bitcoin-knots/src/headerssync.h)
- [`packages/bitcoin-knots/src/headerssync.cpp`](../../../packages/bitcoin-knots/src/headerssync.cpp)
- [`packages/bitcoin-knots/src/sync.cpp`](../../../packages/bitcoin-knots/src/sync.cpp)
- [`packages/bitcoin-knots/src/test/peerman_tests.cpp`](../../../packages/bitcoin-knots/src/test/peerman_tests.cpp)
- [`packages/bitcoin-knots/test/functional/p2p_handshake.py`](../../../packages/bitcoin-knots/test/functional/p2p_handshake.py)
- [`packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py`](../../../packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py)
- [`packages/bitcoin-knots/test/functional/p2p_tx_download.py`](../../../packages/bitcoin-knots/test/functional/p2p_tx_download.py)
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

## First-party implementation

- [`packages/open-bitcoin-network/src/message.rs`](../../../packages/open-bitcoin-network/src/message.rs)
- [`packages/open-bitcoin-network/src/compatibility.rs`](../../../packages/open-bitcoin-network/src/compatibility.rs)
- [`packages/open-bitcoin-network/src/header_store.rs`](../../../packages/open-bitcoin-network/src/header_store.rs)
- [`packages/open-bitcoin-network/src/peer.rs`](../../../packages/open-bitcoin-network/src/peer.rs)
- [`packages/open-bitcoin-network/tests/parity.rs`](../../../packages/open-bitcoin-network/tests/parity.rs)
- [`packages/open-bitcoin-node/src/network.rs`](../../../packages/open-bitcoin-node/src/network.rs)
- [`packages/open-bitcoin-node/src/sync.rs`](../../../packages/open-bitcoin-node/src/sync.rs)
- [`scripts/run-live-mainnet-smoke.ts`](../../../scripts/run-live-mainnet-smoke.ts)

## Known gaps

- address relay, `addrv2`, peer discovery policy, and DNS-seed governance
- encrypted transport and other non-v1 wire transports
- compact blocks, blocktxn, filtered blocks, bloom filters, and compact filters
- peer eviction, bans, resource-governance scoring, and timeout parity beyond
  the basic lifecycle surface
- production-grade daemon-integrated full sync guarantees
- automatic public-mainnet recovery loops and broad production-node service
  guarantees
- long-running socket orchestration and transport persistence beyond the current
  sync-runtime foundation

## Follow-up triggers

Update this entry when later phases add discovery, compact-block relay,
transport encryption, daemon-integrated sync orchestration, or
connection-governance behavior that materially changes the externally visible
networking surface.
