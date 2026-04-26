---
phase: 06
slug: p2p-networking-and-sync
status: verified
threats_open: 0
asvs_level: 1
created: 2026-04-26
updated: 2026-04-26
---

# Phase 06 - Security

Per-phase security contract: threat register, accepted risks, and audit trail.

## Scope

Phase 06 delivers the first P2P networking and sync slice: a pure-core
networking crate for Bitcoin wire messages, peer lifecycle state, header-first
sync decisions, txid or wtxid-aware relay decisions, and a node-side managed
adapter that routes received blocks and transactions through existing
chainstate and mempool adapters.

No explicit `<threat_model>` block or `## Threat Flags` entries were present in
the Phase 06 plan and summary artifacts. This report therefore records an
empty declared threat register and verifies the security-relevant controls
implied by the phase artifacts and repo guardrails.

## Trust Boundaries

| Boundary | Description | Data Crossing |
|----------|-------------|---------------|
| Peer wire bytes to pure networking core | Encoded Bitcoin P2P messages enter through the Phase 6 wire decoder before becoming typed peer actions. | Network magic, command names, payload sizes, checksums, version metadata, inventory, headers, transactions, and blocks. |
| Peer lifecycle state to sync decisions | The pure-core peer manager turns peer messages into explicit actions instead of performing hidden runtime I/O. | Handshake state, wtxidrelay negotiation, sendheaders preference, ping/pong nonce state, requested inventory, and header-sync state. |
| Header sync state to block download requests | Header batches update a deterministic header store before missing blocks are requested. | Block headers, parent hashes, best-tip state, locator hashes, stop hashes, and requested block hashes. |
| Managed networking adapter to validation cores | The node shell applies peer-provided blocks and transactions through managed chainstate and mempool adapters. | Blocks, transactions, script flags, consensus parameters, message timestamps, and local block/transaction stores. |
| Parity evidence to reviewers | Phase 6 parity docs, UAT, and tests document implemented scope and deferred networking capabilities. | Human-readable catalog entries, machine-readable parity status, verification reports, and UAT results. |

## Threat Register

| Threat ID | Category | Component | Disposition | Mitigation | Status |
|-----------|----------|-----------|-------------|------------|--------|
| _none declared_ | - | Phase 06 artifacts | - | No explicit Phase 06 threat model or summary threat flags were found. Security-relevant controls verified below. | closed |

Status: open, closed.
Disposition: mitigate (implementation required), accept (documented risk), transfer (third-party).

## Controls Verified

| Control | Evidence | Status |
|---------|----------|--------|
| Pure-core networking isolation | `scripts/pure-core-crates.txt` includes `open-bitcoin-network`; `open-bitcoin-network` depends only on first-party pure-core crates; `scripts/check-pure-core-deps.sh` rejects forbidden runtime, network, filesystem, process, thread, random, and TLS dependencies or imports in pure-core crates. | verified |
| Unsafe-code and production panic-site hardening | `packages/open-bitcoin-network/src/lib.rs` forbids unsafe code and denies unwrap, expect, panic, unreachable, todo, unimplemented, and panic-in-result functions outside tests. `scripts/check-panic-sites.sh` found no unclassified production panic-like sites. | verified |
| Strict wire envelope validation | `ParsedNetworkMessage::decode_wire` requires the 24-byte header, exact payload length, matching double-SHA256 checksum prefix, and typed command decoding before any peer action is produced. | verified |
| Payload size and shape limits | Header payload decoding rejects more than `MAX_HEADERS_RESULTS`, inventory payload decoding rejects more than `MAX_INV_SIZE`, cursors reject trailing data and unexpected EOF, and `headers` payloads reject non-zero transaction counts. | verified |
| Unknown or malformed command handling | Unknown network commands become typed `NetworkError::UnknownCommand`, invalid user-agent UTF-8 becomes `NetworkError::InvalidUserAgentEncoding`, and message command validation stays in the primitive command type. | verified |
| Duplicate and inconsistent peer lifecycle handling | `PeerManager` tracks version and verack state explicitly, rejects duplicate peer IDs, and turns duplicate `version` messages into `PeerAction::Disconnect(DisconnectReason::DuplicateVersion)`. | verified |
| Header-first block sync | Unknown block inventory triggers `getheaders`; accepted headers are validated with `check_block_header`, inserted through `HeaderStore`, and only then converted into explicit `getdata` requests for missing blocks. | verified |
| Deterministic header-store behavior | `HeaderStore` requires known ancestors, records parent relationships, selects the best tip by chain work, height, and block hash tie-breaker, and builds deterministic Bitcoin-style locators. | verified |
| Requested-inventory bookkeeping | The peer manager records requested blocks, txids, and wtxids, clears them on matching `tx`, `block`, or `notfound` responses, and ignores unsupported inventory types. | verified |
| Managed validation boundary | `ManagedPeerNetwork::receive_wire_message` decodes bytes into typed messages; received transactions go through `ManagedMempool::submit_transaction`; received blocks go through `ManagedChainstate::connect_block_with_current_time`; missing local inventory is answered with `notfound`. | verified |
| Wtxidrelay-aware transaction identity | Transaction announcements and requests switch between txid and wtxid inventory types only when the peer negotiated `wtxidrelay`, with txid and wtxid stores maintained separately. | verified |
| Honest known-gap tracking | `docs/parity/catalog/p2p.md` documents address relay, peer discovery, encrypted transport, compact blocks, filters, peer eviction, bans, resource-governance scoring, timeouts, and socket persistence as deferred capabilities. | verified |

## Accepted Risks Log

No accepted risks.

## Security Audit Trail

| Audit Date | Threats Total | Closed | Open | Run By |
|------------|---------------|--------|------|--------|
| 2026-04-26 | 0 | 0 | 0 | Codex |

## Verification Evidence

| Command | Result |
|---------|--------|
| Targeted `rg` scan for threat-model and threat-flag terms across Phase 06 plan and summary artifacts. | No explicit Phase 06 threat model or summary threat flags found. |
| Targeted `rg` scan for forbidden runtime imports and panic-like production code across Phase 06 network files and guard scripts. | Production controls and dependency surfaces reviewed; panic-like findings are test-only or governed by the production guard script. |
| `bash scripts/check-pure-core-deps.sh` | Passed: pure-core dependency and import checks passed. |
| `bash scripts/check-panic-sites.sh` | Passed: no unclassified production panic-like sites. |
| `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network --all-targets` | Passed: 21 unit tests, 2 parity tests, 2 property tests, and doc-tests passed. |
| `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node network` | Passed: 4 matching node network tests passed. |

## Sign-Off

- [x] All threats have a disposition (mitigate / accept / transfer)
- [x] Accepted risks documented in Accepted Risks Log
- [x] `threats_open: 0` confirmed
- [x] `status: verified` set in frontmatter

Approval: verified 2026-04-26
