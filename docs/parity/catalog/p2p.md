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

## First-party implementation

- [`packages/open-bitcoin-network/src/message.rs`](../../../packages/open-bitcoin-network/src/message.rs)
- [`packages/open-bitcoin-network/src/header_store.rs`](../../../packages/open-bitcoin-network/src/header_store.rs)
- [`packages/open-bitcoin-network/src/peer.rs`](../../../packages/open-bitcoin-network/src/peer.rs)
- [`packages/open-bitcoin-network/tests/parity.rs`](../../../packages/open-bitcoin-network/tests/parity.rs)
- [`packages/open-bitcoin-node/src/network.rs`](../../../packages/open-bitcoin-node/src/network.rs)

## Known gaps

- address relay, `addrv2`, peer discovery, and DNS seeds
- encrypted transport and other non-v1 wire transports
- compact blocks, blocktxn, filtered blocks, bloom filters, and compact filters
- peer eviction, bans, resource-governance scoring, and timeout parity beyond
  the basic lifecycle surface
- long-running socket orchestration and transport persistence outside the
  hermetic in-memory adapter

## Follow-up triggers

Update this entry when later phases add discovery, compact-block relay,
transport encryption, or connection-governance behavior that materially changes
the externally visible networking surface.
