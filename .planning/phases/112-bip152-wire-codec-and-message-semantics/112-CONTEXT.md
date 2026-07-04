---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 112-2026-07-04T19-37-55
generated_at: 2026-07-04T19:37:55.303Z
---

# Phase 112: BIP152 Wire Codec and Message Semantics - Context

**Gathered:** 2026-07-04
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Phase 112 adds first-party BIP152 wire payload support for `sendcmpct`, `cmpctblock`, `getblocktxn`, and `blocktxn`, plus malformed-input semantics that reject invalid compact-block payloads before any partial reconstruction state exists.

This phase owns pure payload types, encode/decode helpers, network-message command mapping, focused round-trip fixtures, malformed payload tests, and parity breadcrumbs for BIP152 message serialization. It may validate header, nonce, six-byte short ID, prefilled transaction differential index, block transaction request, and witness transaction serialization shapes.

This phase must not implement compact relay negotiation policy, compact-block announcement eligibility, mempool-based short ID reconstruction, missing transaction scheduling, `blocktxn` in-flight matching, fallback to full blocks, validation/connect handoff, operator evidence rollout, package relay, bloom/filter serving, compact filter serving, public-serving defaults, public-network CI, archive-node behavior, production full-node readiness, production-service operation, or production-funds wallet use.
</domain>

<decisions>
## Implementation Decisions

### Message Surface

- **D-01:** Add explicit BIP152 message variants to `WireNetworkMessage` for `sendcmpct`, `cmpctblock`, `getblocktxn`, and `blocktxn`; unknown BIP152 command strings should stop being `UnknownCommand` only for these four payloads.
- **D-02:** `sendcmpct` should be represented as a pure payload type with an announce/preference boolean and a `u64` version. Version 2 is the in-scope supported semantic, while unsupported versions should still decode as data so Phase 113 can ignore or suppress them through peer policy.
- **D-03:** Message command names must stay anchored to Knots `NetMsgType::{SENDCMPCT,CMPCTBLOCK,GETBLOCKTXN,BLOCKTXN}` and BIP152. Do not overload existing `GetData`, `Block`, or transaction relay message branches for these commands.

### Compact Block Payloads

- **D-04:** Model compact block payloads with a block header, nonce, six-byte short IDs, and prefilled transactions. Six-byte short IDs should be stored in a type that cannot accidentally accept or emit eight-byte values.
- **D-05:** Prefilled transaction indexes are differential indexes on the wire. The decoded model may expose the wire deltas directly for Phase 112, but helpers/tests must prove malformed overflow, non-canonical compact sizes, out-of-order expansion, and excessive transaction counts fail before reconstruction.
- **D-06:** `cmpctblock` parsing should validate the structural boundary only: header presence, nonce presence, short ID width, prefilled transaction differential-index shape, witness transaction serialization, no trailing data, and count bounds. Header-chain validity, short ID matching, mempool lookup, and block mutation checks belong to later phases.

### Block Transaction Round Trips

- **D-07:** `getblocktxn` should carry a block hash plus differential transaction indexes matching Knots `BlockTransactionsRequest`. Indexes should be bounded to `u16`-compatible values because Knots stores the request indexes as `std::vector<uint16_t>`.
- **D-08:** `blocktxn` should carry a block hash plus transactions serialized with witness data using the existing `TransactionEncoding::WithWitness` codec. Do not add a no-witness `blocktxn` runtime path in Phase 112.
- **D-09:** Round-trip tests should cover multi-index differential encoding, empty vectors where permitted by the wire shape, witness-carrying transactions, and byte-preserving decode/encode for representative payloads.

### Malformed Payload Boundary

- **D-10:** Malformed compact-block payloads should fail at decode with stable `CodecError` or `NetworkError` values before any partial compact-block state can be initialized. Prefer extending `CodecError` with precise BIP152 structural labels over using generic string errors.
- **D-11:** Reject trailing bytes, unexpected EOF, non-canonical compact sizes, six-byte short ID truncation/overflow, differential value overflow, indexes above the representable range, prefilled positions beyond the implied transaction count, null/structurally invalid transactions, and superfluous witness records.
- **D-12:** Keep misbehavior, disconnect, ignore, fallback, and peer-state consequences out of the codec. The codec returns typed decode results or errors; Phase 113+ maps those into peer behavior.

### Runtime Scope

- **D-13:** Phase 112 must leave `InventoryType::CompactBlock` serving behavior as Phase 111 locked it: bounded and classified, but not served as a compact block response from `getdata`.
- **D-14:** The implementation should live primarily in `open-bitcoin-codec` and `open-bitcoin-network::message`, preserving functional core / imperative shell boundaries and avoiding node-shell storage, mempool, socket, metric, log, status, or support-bundle effects.
- **D-15:** If a new Rust source file is introduced under `packages/open-bitcoin-*/src`, add parity breadcrumbs in the file comment and `docs/parity/source-breadcrumbs.json` unless an explicit `none` breadcrumb is defensible.

### Verification And Parity

- **D-16:** Unit tests should be focused and Arrange/Act/Assert structured for pure BIP152 codec behavior. Add mutation-style malformed payload cases around differential index overflow and short ID width because these are high-risk parsing bugs.
- **D-17:** Use repo-native verification and focused package checks first, then `bash scripts/verify.sh` before final phase completion. Public-network compact-block review remains out of default verification.
- **D-18:** Planning may split the phase into the roadmap's three plans: `sendcmpct` and message enum support, `cmpctblock` codec and fixtures, and `getblocktxn`/`blocktxn` codec plus malformed payload tests.

### Claude's Discretion

The planner may choose exact Rust type names, whether BIP152 payloads live in a new `compact_block.rs` codec module or inside existing codec/network modules, and how many fixtures to generate. Prefer a small pure API that later phases can consume without compatibility shims, and prefer typed malformed outcomes over broad catch-all errors.
</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Project And Phase Scope

- `AGENTS.md` - repo-local verification, submodule, parity breadcrumb, UAT command, and GSD workflow guidance.
- `AGENTS.bright-builds.md` - Bright Builds workflow, functional-core, verification, and testing rules.
- `standards/core/architecture.md` - functional core / imperative shell and domain-type rules.
- `standards/core/code-shape.md` - early-return, optional-name, and file/function shape rules.
- `standards/core/testing.md` - focused unit test and Arrange/Act/Assert expectations.
- `standards/core/verification.md` - sync and repo-native verification expectations.
- `standards/languages/rust.md` - Rust module, invariant, optional naming, and verification guidance.
- `.planning/PROJECT.md` - active v2.1 scope, parity value, architecture constraints, and deferred public/production claims.
- `.planning/REQUIREMENTS.md` - CMP-01, CMP-02, CMP-03, and RCN-01 ownership for Phase 112.
- `.planning/ROADMAP.md` - Phase 112 goal, success criteria, requirement mapping, and plan split.
- `.planning/STATE.md` - current milestone state and deterministic verification caveats.
- `.planning/research/ARCHITECTURE.md` - v2.1 codec layer, pure policy, node shell, and data-flow guidance.

### Prior Locked Decisions

- `.planning/phases/110-block-serving-activation-and-eligibility-boundary/110-CONTEXT.md` - default-off block/compact activation, peer eligibility, status, resource, and no-claim decisions.
- `.planning/phases/111-full-block-serving-request-path/111-CONTEXT.md` - full/witness block serving path and explicit compact-block response deferral.
- `.planning/phases/111-full-block-serving-request-path/111-02-SUMMARY.md` - node-shell adapter decisions proving compact-block inventory remains suppressed in Phase 111.
- `.planning/phases/100-relay-activation-boundary-and-permission-semantics/100-CONTEXT.md` - relay activation separation and no-claim guardrail pattern.

### Existing Code Integration Points

- `packages/open-bitcoin-network/src/message.rs` - `WireNetworkMessage` command, payload encode/decode, and wire checksum integration.
- `packages/open-bitcoin-network/src/message/tests.rs` - existing message test style, sample block/transaction fixtures, and payload error coverage.
- `packages/open-bitcoin-codec/src/network.rs` - message header, inventory vector, block locator, compact-size, and network-address codec helpers.
- `packages/open-bitcoin-codec/src/block.rs` - block header and witness-preserving block codec helpers.
- `packages/open-bitcoin-codec/src/transaction.rs` - witness transaction encode/decode behavior used by `blocktxn` and prefilled transactions.
- `packages/open-bitcoin-codec/src/error.rs` - typed codec error surface to extend for BIP152 structural failures.
- `packages/open-bitcoin-primitives/src/network.rs` - message commands, inventory types, and `InventoryType::CompactBlock`.
- `packages/open-bitcoin-primitives/src/block.rs` - block header and block primitives for compact block payloads.
- `packages/open-bitcoin-primitives/src/transaction.rs` - transaction primitive and witness detection behavior.
- `docs/parity/source-breadcrumbs.json` - required breadcrumb registry for new first-party Rust source/test files.
- `scripts/verify.sh` - repo-native verification contract.

### Bitcoin Knots Anchors

- `packages/bitcoin-knots/src/protocol.h` - BIP152 message command names and inventory constants.
- `packages/bitcoin-knots/src/blockencodings.h` - `DifferenceFormatter`, `BlockTransactionsRequest`, `BlockTransactions`, `PrefilledTransaction`, `CBlockHeaderAndShortTxIDs`, and short ID width.
- `packages/bitcoin-knots/src/blockencodings.cpp` - short ID selector, `GetShortID`, compact-block validity, prefilled index handling, collision, missing transaction, and mutation boundaries for later phases.
- `packages/bitcoin-knots/src/primitives/transaction.h` - witness transaction serialization semantics.
- `packages/bitcoin-knots/test/functional/p2p_compactblocks.py` - BIP152 behavior examples and sendcmpct version expectations.
- `packages/bitcoin-knots/test/functional/test_framework/messages.py` - Python fixture/message shapes for compact-block payloads.
</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets

- `WireNetworkMessage` already owns command names, payload encode/decode, and wire framing for P2P messages.
- `Cursor` in `open-bitcoin-network/src/message.rs` already provides local compact-size and fixed-width read helpers for network payloads.
- `open-bitcoin-codec::transaction` already encodes and decodes witness transactions and rejects superfluous witness records.
- `open-bitcoin-codec::block` already exposes block-header serialization that `cmpctblock` should reuse.
- `InventoryType::CompactBlock` already exists, and Phase 111 already suppresses compact inventory serving from `getdata`.

### Established Patterns

- Pure payload correctness belongs in codec/network modules; runtime effects belong in node-shell adapters.
- Existing message tests use small in-memory fixtures and direct payload/wire round trips.
- Existing codec errors are typed and human-readable, with tests covering display text.
- New source files under first-party Rust packages need parity breadcrumbs and registry entries.

### Integration Points

- Add BIP152 payload helpers to `open-bitcoin-codec` if they are reusable byte-level codecs, then expose them through `open-bitcoin-network::message`.
- Extend `WireNetworkMessage::command_name`, `encode_payload`, and `decode_payload` for the four BIP152 commands.
- Keep `ManagedPeerNetwork::serve_inventory` unchanged except for any compile fallout; runtime compact-relay behavior belongs to later phases.
</code_context>

<specifics>
## Specific Ideas

- Prefer `ShortId([u8; 6])` or an equivalent newtype over raw `u64` for wire short IDs.
- Decode unsupported `sendcmpct` versions as data so Phase 113 can ignore or suppress them without inventing compatibility shims.
- Use witness serialization for prefilled transactions and `blocktxn` responses from the start.
- Tests should prove malformed compact-block inputs fail before reconstruction state can exist.
</specifics>

<deferred>
## Deferred Ideas

Compact relay negotiation, compact-block announcement policy, mempool/recent-block short ID reconstruction, missing transaction scheduling, `blocktxn` in-flight matching, fallback, validation/connect handoff, operator status/RPC/CLI/dashboard/metrics/log/support evidence, parity/UAT release closeout, package relay, bloom/filter serving, compact filter serving, public serving defaults, public-network CI, archive-node claims, production full-node readiness, production-service operation, and production-funds wallet use remain outside Phase 112.
</deferred>

***

*Phase: 112-bip152-wire-codec-and-message-semantics*
*Context gathered: 2026-07-04*
