# v2.1 Stack Research - Block Serving and Compact Block Relay Boundary

**Defined:** 2026-07-03
**Milestone:** v2.1 Block Serving and Compact Block Relay Boundary

## Recommendation

v2.1 should not add external production dependencies. The milestone can be built with the existing first-party Rust crates plus small owned protocol helpers for BIP152 compact block short IDs and payload encoding.

The only likely new primitive is a first-party SipHash-2-4 implementation or tightly scoped helper for BIP152 short IDs. The repo currently has no SipHash helper, and the dependency policy rules out using a Bitcoin library in the production path. Keep that helper small, audited, fixture-tested, and isolated from generic hashing APIs.

## First-Party Crates To Extend

### `open-bitcoin-codec`

- Add compact block payload codecs for `sendcmpct`, `cmpctblock`, `getblocktxn`, and `blocktxn`.
- Reuse existing compact-size, block, transaction, inventory, and message-header codecs.
- Add 6-byte short ID encoding/decoding and differential index encoding for `BlockTransactionsRequest`.
- Keep malformed payload handling typed through existing `CodecError` and `NetworkError` paths.

### `open-bitcoin-primitives`

- Add value types only if the codec and network crates need shared shapes, such as compact block short IDs, prefilled transactions, or block transaction requests.
- Avoid exposing peer negotiation or runtime state from primitives.

### `open-bitcoin-network`

- Add pure block-serving and compact-relay policy modules beside the existing peer, relay download, transaction relay, resource, and message modules.
- Track per-peer compact-block negotiation flags without storage, sockets, metrics, or logs.
- Emit typed actions for full block serving, compact block serving, missing-transaction requests, fallback, and malformed-message outcomes.

### `open-bitcoin-node`

- Own block storage reads, mempool snapshots, reconstruction execution, fallback scheduling, status projection, metrics, logs, and support evidence.
- Keep compact-block partial state in the node shell or a pure state object controlled by the shell; do not write partial reconstruction state into durable chainstate.
- Reuse the current `blocks_by_hash`, chainstate snapshot, mempool, relay-serving, and resource-governance foundations where they fit.

### `open-bitcoin-rpc` and `open-bitcoin-cli`

- Extend the shared status contract rather than creating compact-relay-only operator truth.
- Render activation, eligible peer counts, negotiation counts, reconstruction outcomes, fallback counts, and serving counts with fixed labels.

### `scripts/` and `docs/parity/`

- Extend deterministic checkers for parity anchors, no-claim language, UAT command forms, and default verifier boundaries.
- Keep substantial checker logic in Bun/TypeScript if logic grows beyond thin shell orchestration.

## Knots Anchors Inspected

- `packages/bitcoin-knots/src/protocol.h` for message names and inventory types.
- `packages/bitcoin-knots/src/blockencodings.h` and `packages/bitcoin-knots/src/blockencodings.cpp` for `BlockTransactionsRequest`, `BlockTransactions`, prefilled transactions, compact block short IDs, and partial reconstruction behavior.
- `packages/bitcoin-knots/src/net_processing.cpp` for block request eligibility, block serving, `sendcmpct` negotiation, compact block announcements, `getblocktxn`, `blocktxn`, reconstruction fallback, and in-flight cleanup.
- `packages/bitcoin-knots/test/functional/p2p_compactblocks.py`, `p2p_compactblocks_extratxs.py`, `p2p_compactblocks_blocksonly.py`, `p2p_compactblocks_hb.py`, and `p2p_mutated_blocks.py` for table-stakes behavioral fixtures.

## What Not To Add

- No Rust Bitcoin production-path dependency.
- No generic async framework changes; Tokio/Axum already exist where runtime/RPC need them.
- No database change for partial compact blocks.
- No public-network test dependency in the default verifier.
- No package relay, bloom filters, compact filters, GUI, installer, or service-manager expansion as part of v2.1.

## Verification Implications

- Unit tests should cover BIP152 codecs, short ID calculation, differential indexes, negotiation decisions, reconstruction statuses, and fallback decisions.
- Node-shell tests should cover storage reads, mempool reconstruction input, in-flight cleanup, validation handoff, metrics/status/log/support projection, and restart cleanup.
- Deterministic checkers should catch unsupported public-default claims and missing parity breadcrumbs.
- `bash scripts/verify.sh` remains the pre-commit contract.
