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
- v1.4 opt-in outbound IBD evidence keeps outbound peer compatibility, header
  progress, downloaded block progress, connected block progress, and
  restart/resume evidence reviewable without claiming broader P2P service
  readiness

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
- daemon-integrated, unattended public-network full sync through `open-bitcoind`
- service-manager restart policy, unattended daemon supervision, and automatic
  public-mainnet recovery loops
- long-running socket orchestration and transport persistence beyond the current
  sync-runtime foundation

## Follow-up triggers

Update this entry when later phases add discovery, compact-block relay,
transport encryption, daemon-integrated sync orchestration, or
connection-governance behavior that materially changes the externally visible
networking surface.
