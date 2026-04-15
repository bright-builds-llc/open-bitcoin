---
phase: 06-p2p-networking-and-sync
generated_by: gsd-phase-researcher
lifecycle_mode: yolo
phase_lifecycle_id: 06-2026-04-15T00-28-26
generated_at: 2026-04-15T00:28:26Z
---

# Phase 6: P2P Networking and Sync - Research

**Researched:** 2026-04-14  
**Domain:** pure-core peer state, Bitcoin wire messages, header or block sync,
and inventory relay over managed chainstate and mempool adapters  
**Confidence:** HIGH

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
All items in this subsection are copied verbatim from `06-CONTEXT.md`.
[VERIFIED: .planning/phases/06-p2p-networking-and-sync/06-CONTEXT.md]
- **D-01:** Add a dedicated pure-core `open-bitcoin-network` crate instead of
  embedding peer state directly in `open-bitcoin-node` or stretching the
  existing primitives or codec crates into a subsystem owner.
- **D-02:** Keep `open-bitcoin-node` as the imperative shell that stores
  connected blocks or transactions and applies synced data to managed
  chainstate or mempool state, but keep handshake policy, header-tree
  bookkeeping, locator construction, and inventory request decisions inside the
  pure core.
- **D-03:** The initial parity slice should cover `version`, `verack`,
  `wtxidrelay`, `sendheaders`, `ping`, `pong`, `getheaders`, `headers`, `inv`,
  `getdata`, `tx`, `block`, and `notfound`, plus strict message-header checksum
  handling.
- **D-04:** Defer `addr`, `addrv2`, compact blocks, filters, encrypted
  transport, misbehavior scoring, and peer-eviction policy; Phase 6 should
  call those out explicitly in parity docs rather than implying they already
  exist.
- **D-05:** Header sync should build a pure header tree that derives locators
  from the best known chain and requests full blocks in header order instead of
  pulling blocks directly from inventory announcements.
- **D-06:** Block application should stay deterministic for hermetic fixtures:
  blocks are requested only after headers connect, then applied to the managed
  chainstate in announced order with explicit chain-work progression suitable
  for the controlled low-difficulty test corpus.
- **D-07:** Transaction relay must preserve txid or wtxid distinctions, use
  `wtxidrelay` negotiation to choose requested inventory types, and feed
  received transactions through the managed mempool rather than bypassing
  policy.
- **D-08:** Phase 6 should prove behavior through hermetic in-memory multi-node
  fixtures that exchange real encoded messages between managed nodes rather
  than relying on real sockets or external daemons.
- **D-09:** Parity reporting for the `p2p` surface is a required output of the
  phase, not optional cleanup after code lands.

### the agent's Discretion
- exact internal module split inside `open-bitcoin-network`
- the concrete header-work scoring helper used by low-difficulty fixtures
- whether the node wrapper is named `ManagedPeerNetwork` or similar

### Deferred Ideas (OUT OF SCOPE)
All items in this subsection are copied verbatim from `06-CONTEXT.md`.
[VERIFIED: .planning/phases/06-p2p-networking-and-sync/06-CONTEXT.md]
- address gossip, `addrv2`, DNS seeds, and peer-discovery policy
- BIP324 or other encrypted transport support
- compact blocks, blocktxn, and other bandwidth-optimized block relay
- bloom filters, compact filters, and filter checkpoint protocols
- peer eviction, ban scoring, timeouts, and resource-governance parity beyond
  basic lifecycle handling

</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| `P2P-01` | The node performs peer handshake, peer lifecycle, and message handling compatibly with the baseline. [VERIFIED: .planning/REQUIREMENTS.md] | The minimum slice must cover version negotiation, verack flow, optional `wtxidrelay` and `sendheaders`, ping or pong, strict message framing, and deterministic peer state transitions. [VERIFIED: .planning/ROADMAP.md, .planning/phases/06-p2p-networking-and-sync/06-CONTEXT.md, packages/bitcoin-knots/src/protocol.h, packages/bitcoin-knots/test/functional/p2p_handshake.py] |
| `P2P-02` | The node syncs headers and blocks and relays inventory and transactions compatibly with the baseline. [VERIFIED: .planning/REQUIREMENTS.md] | The minimum slice must cover locator-based header sync, `headers` handling, block download via `getdata`, block announcements, tx or wtx inventory requests, and mempool submission for inbound transactions. [VERIFIED: .planning/ROADMAP.md, .planning/phases/06-p2p-networking-and-sync/06-CONTEXT.md, packages/bitcoin-knots/src/headerssync.cpp, packages/bitcoin-knots/src/sync.cpp, packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py, packages/bitcoin-knots/test/functional/p2p_tx_download.py] |

</phase_requirements>

## Summary

- Phase 6 should add a new pure-core networking crate that owns message types,
  wire encoding or decoding, peer state, header-tree bookkeeping, locator
  construction, and inventory-request policy. `open-bitcoin-node` should wrap
  that core with managed chainstate, managed mempool, and in-memory block or
  transaction stores for hermetic fixtures. [VERIFIED:
  .planning/phases/06-p2p-networking-and-sync/06-CONTEXT.md,
  packages/open-bitcoin-node/src/lib.rs]
- The repo already has the right substrate for this slice: typed message
  headers, addresses, locators, inventory vectors, block or transaction codecs,
  txid or wtxid helpers, and managed adapters for chainstate and mempool.
  Phase 6 should compose those existing pieces rather than re-parsing Bitcoin
  data structures by hand. [VERIFIED: packages/open-bitcoin-primitives/src/network.rs,
  packages/open-bitcoin-codec/src/network.rs, packages/open-bitcoin-consensus/src/crypto.rs,
  packages/open-bitcoin-node/src/chainstate.rs, packages/open-bitcoin-node/src/mempool.rs]
- Knots' networking surface is far broader than the roadmap slice. The first
  parity pass should mirror the externally visible handshake, sync, and relay
  semantics promised by the roadmap, not peer discovery, compact blocks,
  addrv2, or connection-governance policy. [VERIFIED:
  packages/bitcoin-knots/src/protocol.h, packages/bitcoin-knots/src/headerssync.cpp,
  packages/bitcoin-knots/src/sync.cpp, .planning/ROADMAP.md]

**Primary recommendation:** implement Phase 6 as a pure-core `open-bitcoin-network`
engine plus a thin `open-bitcoin-node` managed adapter, and verify it with
encoded in-memory multi-node fixtures that cover handshake, header sync, block
download, and tx or wtx relay. [VERIFIED: .planning/PROJECT.md,
.planning/ROADMAP.md, .planning/phases/06-p2p-networking-and-sync/06-CONTEXT.md]

## Standard Stack

### Core
| Component | Purpose | Why Standard For This Phase | Source |
|-----------|---------|-----------------------------|--------|
| New pure-core networking crate under `packages/` | Own peer lifecycle, wire payloads, header tree, locator logic, and relay decisions | Phase 6 explicitly chose a dedicated pure-core subsystem boundary. | [VERIFIED: .planning/phases/06-p2p-networking-and-sync/06-CONTEXT.md] |
| `open-bitcoin-primitives` | Typed message-header, locator, inventory, block, and tx domain types | The networking core already has reusable primitive shapes. | [VERIFIED: packages/open-bitcoin-primitives/src/network.rs, packages/open-bitcoin-primitives/src/block.rs, packages/open-bitcoin-primitives/src/transaction.rs] |
| `open-bitcoin-codec` | Existing block, tx, inventory, locator, and header byte codecs | Wire payload implementation should compose existing codecs instead of duplicating them. | [VERIFIED: packages/open-bitcoin-codec/src/lib.rs, packages/open-bitcoin-codec/src/network.rs, packages/open-bitcoin-codec/src/block.rs, packages/open-bitcoin-codec/src/transaction.rs] |
| `open-bitcoin-consensus` | Message checksums, txid or wtxid, block hash, and proof-of-work checks | The networking core needs these exact compatibility primitives. | [VERIFIED: packages/open-bitcoin-consensus/src/crypto.rs, packages/open-bitcoin-consensus/src/block.rs] |

### Supporting
| Component | Purpose | When To Use | Source |
|-----------|---------|-------------|--------|
| `open-bitcoin-core` | Umbrella pure-core export | Re-export the new networking crate for downstream consumers. | [VERIFIED: packages/open-bitcoin-core/src/lib.rs] |
| `open-bitcoin-node` | Thin managed adapter over networking, chainstate, and mempool | Keep block or tx stores plus shell-side orchestration outside the pure protocol engine. | [VERIFIED: packages/open-bitcoin-node/src/lib.rs, .planning/phases/06-p2p-networking-and-sync/06-CONTEXT.md] |
| `docs/parity/index.json` and `docs/parity/catalog/*.md` | Explicit parity and deviation tracking | Required for honest completion of the `p2p` surface. | [VERIFIED: docs/parity/index.json] |
| `bash scripts/verify.sh` | Repo-native verification contract | Phase completion requires format, lint, build, tests, Bazel, purity, and coverage checks. | [VERIFIED: AGENTS.md, scripts/verify.sh] |

### Alternatives Considered
| Instead Of | Could Use | Tradeoff | Source |
|------------|-----------|----------|--------|
| Pure-core networking crate | Put peer state only in `open-bitcoin-node` | Faster to wire, but it violates the repo's architecture boundary and weakens testability. | [VERIFIED: .planning/PROJECT.md, .planning/phases/06-p2p-networking-and-sync/06-CONTEXT.md] |
| Header-first sync | Request blocks directly from block invs | Simpler, but it skips the roadmap's explicit header-sync behavior and diverges from Knots' modern flow. | [VERIFIED: packages/bitcoin-knots/src/sync.cpp, packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py] |
| Real sockets in tests | In-memory encoded-message fixtures | Closer to production, but much slower and outside the repo's hermetic pure-core testing style. | [VERIFIED: .planning/phases/06-p2p-networking-and-sync/06-CONTEXT.md] |

## Architecture Patterns

### Recommended Crate Boundary

- `packages/open-bitcoin-network`
  Own protocol constants, wire payloads, header-store state, peer state, and
  inventory or sync decisions. [VERIFIED:
  .planning/phases/06-p2p-networking-and-sync/06-CONTEXT.md]
- `packages/open-bitcoin-core`
  Re-export the networking crate with the existing chainstate, consensus,
  codec, mempool, and primitives crates. [VERIFIED: packages/open-bitcoin-core/src/lib.rs]
- `packages/open-bitcoin-node`
  Own a managed wrapper that feeds blocks into chainstate, transactions into
  the mempool, and local stores into `getheaders` or `getdata` responses.
  [VERIFIED: packages/open-bitcoin-node/src/chainstate.rs,
  packages/open-bitcoin-node/src/mempool.rs]

### Minimum Message Surface

- `VersionMessage`
  Must cover protocol version, service flags, timestamp, sender or receiver
  addresses, nonce, user agent, start height, and relay preference. [VERIFIED:
  packages/bitcoin-knots/test/functional/test_framework/messages.py]
- `NetworkMessage`
  Should model the subset of commands named in `D-03`, including empty payload
  messages such as `verack`, `sendheaders`, and `wtxidrelay`. [VERIFIED:
  packages/bitcoin-knots/src/protocol.h]
- `WireEnvelope`
  Should validate Bitcoin message headers, payload length, and checksum so the
  hermetic fixtures exercise the actual framing rules. [VERIFIED:
  packages/open-bitcoin-primitives/src/network.rs,
  packages/open-bitcoin-codec/src/network.rs]

### Minimum State Types

- `PeerState`
  Tracks handshake progress, negotiated relay features, advertised height, and
  request sets for blocks or transactions. [ASSUMED]
- `HeaderStore`
  Tracks known headers by hash, parent relationship, accumulated test-chain
  work, and best-chain selection for locator construction or `headers`
  responses. [VERIFIED: packages/bitcoin-knots/src/headerssync.cpp] [ASSUMED:
  exact type name]
- `PeerAction`
  Makes outbound messages, data requests, disconnect reasons, and inbound block
  or transaction deliveries explicit instead of hiding them as side effects.
  [ASSUMED]

## Baseline Behaviors To Mirror

- A peer connection begins with `version`, followed by `verack`, with optional
  `wtxidrelay` and `sendheaders` negotiation visible before steady-state relay.
  [VERIFIED: packages/bitcoin-knots/src/protocol.h,
  packages/bitcoin-knots/test/functional/p2p_handshake.py]
- Modern block announcements should trigger locator-driven header sync before
  direct block download, and accepted `headers` batches should lead to
  deterministic `getdata` requests for the announced blocks. [VERIFIED:
  packages/bitcoin-knots/src/sync.cpp, packages/bitcoin-knots/src/headerssync.cpp,
  packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py]
- Transaction inventory relay must keep txid or wtxid identity visible and
  honor `wtxidrelay` negotiation when choosing which inventory type to request
  or announce. [VERIFIED: packages/bitcoin-knots/src/protocol.h,
  packages/bitcoin-knots/test/functional/p2p_tx_download.py]
- Ping or pong should round-trip an explicit nonce so peer lifecycle handling
  can prove that both sides are still responsive. [VERIFIED:
  packages/bitcoin-knots/test/functional/test_framework/messages.py]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why | Source |
|---------|-------------|-------------|-----|--------|
| Transaction or block serialization | Custom byte encoders inside the networking crate | Existing `open-bitcoin-codec` encoders and decoders | Phase 2 already established the codec boundary. | [VERIFIED: packages/open-bitcoin-codec/src/block.rs, packages/open-bitcoin-codec/src/transaction.rs] |
| Checksums and txid or wtxid logic | Another hashing utility in the new crate | Existing `double_sha256`, `block_hash`, `transaction_txid`, and `transaction_wtxid` helpers | Networking must reuse the exact same compatibility primitives as the rest of the core. | [VERIFIED: packages/open-bitcoin-consensus/src/crypto.rs] |
| Node-owned peer rules | Hidden connection-state mutation in `open-bitcoin-node` | Explicit pure-core peer or sync state plus shell-owned stores | The repo's architecture requires pure rules and imperative adapters. | [VERIFIED: .planning/PROJECT.md] |

## Concrete Parity Fixtures

| Fixture | What It Should Prove | Why It Matters | Source |
|---------|----------------------|----------------|--------|
| `wire_message_round_trips_version_and_inventory_payloads` | Message envelopes preserve command names, checksums, and payload encoding. | Phase 6 promises wire-message handling, not just high-level state. | [VERIFIED: packages/bitcoin-knots/src/protocol.h, packages/open-bitcoin-primitives/src/network.rs] |
| `outbound_handshake_negotiates_verack_sendheaders_and_wtxidrelay` | An outbound peer reaches steady-state lifecycle messages in the expected order. | This is the minimum `P2P-01` handshake truth. | [VERIFIED: packages/bitcoin-knots/test/functional/p2p_handshake.py] |
| `block_inventory_triggers_getheaders_then_getdata_for_missing_blocks` | Unknown block announcements request headers first, then specific blocks after the headers connect. | This is the minimum Phase 6 sync behavior. | [VERIFIED: packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py, packages/bitcoin-knots/src/sync.cpp] |
| `wtxidrelay_changes_transaction_request_type` | Relay negotiation changes whether tx invs are requested by txid or wtxid. | The roadmap and Phase 2 catalog both call out txid versus wtxid visibility. | [VERIFIED: packages/bitcoin-knots/test/functional/p2p_tx_download.py, docs/parity/catalog/core-domain-and-serialization.md] |
| `managed_nodes_sync_blocks_and_relay_transactions_in_memory` | Multiple managed nodes can handshake, sync a short chain, and relay a mempool transaction through encoded messages only. | This is the phase's end-to-end proof without real sockets. | [VERIFIED: .planning/ROADMAP.md, .planning/phases/06-p2p-networking-and-sync/06-CONTEXT.md] |

## Common Pitfalls

- Letting `open-bitcoin-node` own peer rules instead of adapter wiring will
  recreate the exact functional-core boundary violation the project is trying
  to avoid. [VERIFIED: .planning/PROJECT.md]
- Treating block inventory as immediate block-download permission instead of
  header-sync stimulus will skip the explicit `headers` path promised by Phase
  6. [VERIFIED: packages/bitcoin-knots/src/sync.cpp,
  packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py]
- Collapsing txid and wtxid relay identity into one opaque hash will break the
  `wtxidrelay` semantics that Phase 2 already cataloged. [VERIFIED:
  docs/parity/catalog/core-domain-and-serialization.md,
  packages/open-bitcoin-consensus/src/crypto.rs]
- Marking `p2p` done without documenting deferred networking surfaces will make
  the parity ledger dishonest even if the code is green. [VERIFIED:
  docs/parity/index.json, .planning/REQUIREMENTS.md]

## Assumptions Log

| # | Claim | Section | Risk If Wrong |
|---|-------|---------|---------------|
| A1 | The new pure-core crate should be named `open-bitcoin-network`. | Standard Stack / Architecture Patterns | Low; the boundary matters more than the exact package name. |
| A2 | Hermetic fixtures can use deterministic unit work per accepted header because the initial sync corpus is low-difficulty and fully controlled. | Summary / Architecture Patterns | Medium; a later performance or difficulty-accurate phase may refine the scoring helper. |
| A3 | The node wrapper only needs in-memory block or tx stores in this phase. | Architecture Patterns | Low; on-disk transport state is outside the roadmap slice. |

## Sources

### Primary
- `.planning/PROJECT.md`
- `.planning/REQUIREMENTS.md`
- `.planning/ROADMAP.md`
- `.planning/phases/04-chainstate-and-utxo-engine/04-CONTEXT.md`
- `.planning/phases/05-mempool-and-node-policy/05-CONTEXT.md`
- `.planning/phases/06-p2p-networking-and-sync/06-CONTEXT.md`
- `packages/open-bitcoin-primitives/src/network.rs`
- `packages/open-bitcoin-codec/src/network.rs`
- `packages/open-bitcoin-codec/src/block.rs`
- `packages/open-bitcoin-codec/src/transaction.rs`
- `packages/open-bitcoin-consensus/src/crypto.rs`
- `packages/open-bitcoin-chainstate/src/types.rs`
- `packages/open-bitcoin-node/src/chainstate.rs`
- `packages/open-bitcoin-node/src/mempool.rs`
- `packages/bitcoin-knots/src/protocol.h`
- `packages/bitcoin-knots/src/headerssync.h`
- `packages/bitcoin-knots/src/headerssync.cpp`
- `packages/bitcoin-knots/src/sync.cpp`
- `packages/bitcoin-knots/src/test/peerman_tests.cpp`
- `packages/bitcoin-knots/test/functional/p2p_handshake.py`
- `packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py`
- `packages/bitcoin-knots/test/functional/p2p_tx_download.py`
- `packages/bitcoin-knots/test/functional/test_framework/messages.py`
- `docs/parity/catalog/core-domain-and-serialization.md`
- `docs/parity/index.json`
- `scripts/verify.sh`

## Metadata

- Standard stack confidence: HIGH because the repo already has the typed
  wire primitives, codec surfaces, and shell adapters this phase needs.
- Behavior-surface confidence: HIGH for the chosen message subset because the
  cited Knots protocol definitions and functional tests line up directly with
  the roadmap's handshake, sync, and relay promises.
