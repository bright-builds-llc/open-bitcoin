---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 06-2026-04-15T00-28-26
generated_at: 2026-04-15T00:28:26Z
---

# Phase 6: P2P Networking and Sync - Context

**Gathered:** 2026-04-14  
**Status:** Ready for planning and execution  
**Mode:** Yolo

<domain>
## Phase Boundary

Phase 6 owns the first baseline-compatible networking slice: peer lifecycle,
wire-message handling, header or block sync, and transaction or block
inventory relay over the existing chainstate and mempool cores. The pure core
must own message types, peer state, header-sync decisions, and relay
selection. `open-bitcoin-node` stays a thin managed adapter that persists
chainstate, keeps block or transaction stores for connected peers, and feeds
validated data into the pure networking core. Address relay, compact blocks,
filters, encryption, DNS seeds, and long-running socket orchestration stay out
of scope.

</domain>

<decisions>
## Implementation Decisions

### Crate and boundary shape
- **D-01:** Add a dedicated pure-core `open-bitcoin-network` crate instead of
  embedding peer state directly in `open-bitcoin-node` or stretching the
  existing primitives or codec crates into a subsystem owner.
- **D-02:** Keep `open-bitcoin-node` as the imperative shell that stores
  connected blocks or transactions and applies synced data to managed
  chainstate or mempool state, but keep handshake policy, header-tree
  bookkeeping, locator construction, and inventory request decisions inside the
  pure core.

### Message scope and protocol behavior
- **D-03:** The initial parity slice should cover `version`, `verack`,
  `wtxidrelay`, `sendheaders`, `ping`, `pong`, `getheaders`, `headers`, `inv`,
  `getdata`, `tx`, `block`, and `notfound`, plus strict message-header checksum
  handling.
- **D-04:** Defer `addr`, `addrv2`, compact blocks, filters, encrypted
  transport, misbehavior scoring, and peer-eviction policy; Phase 6 should
  call those out explicitly in parity docs rather than implying they already
  exist.

### Sync and relay policy
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

### Verification and fixtures
- **D-08:** Phase 6 should prove behavior through hermetic in-memory multi-node
  fixtures that exchange real encoded messages between managed nodes rather
  than relying on real sockets or external daemons.
- **D-09:** Parity reporting for the `p2p` surface is a required output of the
  phase, not optional cleanup after code lands.

</decisions>

<specifics>
## Specific Ideas

- Reuse the easy-difficulty block-building fixtures from Phase 4 and Phase 5 so
  sync tests can mine deterministic chains without introducing a second fake
  header model.
- Keep the first relay surface narrow and inspectable: outbound version
  handshake, header-first block download, explicit block or tx announcements,
  and ping or pong lifecycle checks.
- Favor a queue-driven in-memory test harness where each managed node drains
  encoded outbound messages into another node's inbound handler so wire
  compatibility stays visible in tests.

</specifics>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Project and phase scope
- `.planning/PROJECT.md` — functional-core boundary, headless scope, and
  parity-first milestone goal
- `.planning/REQUIREMENTS.md` — `P2P-01` and `P2P-02` acceptance scope
- `.planning/ROADMAP.md` § Phase 6 — networking goal, success criteria, and
  plan inventory
- `.planning/phases/04-chainstate-and-utxo-engine/04-CONTEXT.md` — chainstate
  snapshot and reorg outputs that networking now consumes
- `.planning/phases/05-mempool-and-node-policy/05-CONTEXT.md` — mempool relay
  and txid or wtxid visibility that networking must preserve

### Existing Rust implementation
- `packages/open-bitcoin-primitives/src/network.rs` — message header,
  inventory, locator, and network-address types already available
- `packages/open-bitcoin-codec/src/network.rs` — existing wire codecs for
  message headers, addresses, inventory vectors, and locators
- `packages/open-bitcoin-consensus/src/crypto.rs` — block hash, txid, wtxid,
  and checksum primitives needed by wire handling
- `packages/open-bitcoin-chainstate/src/types.rs` — active-chain positions and
  snapshot shapes used to seed local sync state
- `packages/open-bitcoin-node/src/chainstate.rs` — managed chainstate adapter
  the networking shell must drive
- `packages/open-bitcoin-node/src/mempool.rs` — managed mempool adapter used
  for inbound transaction relay

### Knots networking baseline
- `packages/bitcoin-knots/src/protocol.h` — message type catalog, inventory
  semantics, and protocol constants
- `packages/bitcoin-knots/src/headerssync.h` and
  `packages/bitcoin-knots/src/headerssync.cpp` — header-tree and locator
  synchronization concepts
- `packages/bitcoin-knots/src/sync.cpp` — header or block sync behavior and
  block-request flow
- `packages/bitcoin-knots/src/test/peerman_tests.cpp` — peer manager behavior
  and synchronization expectations
- `packages/bitcoin-knots/test/functional/p2p_handshake.py` — handshake and
  peer-lifecycle expectations
- `packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py` —
  initial header-sync behavior
- `packages/bitcoin-knots/test/functional/p2p_tx_download.py` — tx relay and
  wtxidrelay request behavior

### Repo-native verification and parity tracking
- `scripts/verify.sh` — verification contract the phase must keep green
- `docs/parity/index.json` — surface-level parity status for `p2p`

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `open-bitcoin-primitives` already models message headers, network addresses,
  block locators, and inventory vectors, so the new networking crate does not
  need to re-invent those wire primitives.
- `open-bitcoin-codec` already encodes or decodes block headers, blocks,
  transactions, locators, and inventory vectors, so Phase 6 can compose
  message payloads instead of re-serializing Bitcoin objects manually.
- `ManagedChainstate` and `ManagedMempool` already provide the adapter-owned
  shell surfaces that networking can drive after messages are interpreted.

### Established Patterns
- New pure-core subsystems land as dedicated crates under `packages/` and are
  re-exported through `open-bitcoin-core`.
- Shell crates own orchestration and state persistence, not the protocol rules
  themselves.
- Parity docs move from `planned` to `done` only after repo-native verifier
  output is green.

### Integration Points
- `open-bitcoin-core` should re-export `open-bitcoin-network` for downstream
  packages.
- `open-bitcoin-node` should expose a thin managed network adapter that wraps
  peer-manager decisions around managed chainstate and mempool instances.
- Later wallet, RPC, and CLI phases will depend on explicit peer state,
  block-sync status, and relay outcomes rather than hidden runtime side
  effects.

</code_context>

<deferred>
## Deferred Ideas

- address gossip, `addrv2`, DNS seeds, and peer-discovery policy
- BIP324 or other encrypted transport support
- compact blocks, blocktxn, and other bandwidth-optimized block relay
- bloom filters, compact filters, and filter checkpoint protocols
- peer eviction, ban scoring, timeouts, and resource-governance parity beyond
  basic lifecycle handling

</deferred>

---

*Phase: 06-p2p-networking-and-sync*  
*Context gathered: 2026-04-14*
