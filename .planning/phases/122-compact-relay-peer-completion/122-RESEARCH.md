---
phase: 122-compact-relay-peer-completion
generated_by: gsd-phase-researcher
lifecycle_mode: yolo
phase_lifecycle_id: 122-2026-07-15T15-22-57
generated_at: 2026-07-15T15:22:57Z
status: complete
requirements:
  - HARD-01
---

# Phase 122 Research

## Goal and Requirement

Phase 122 closes `HARD-01` by completing the live per-peer BIP152 compact-relay response path. A peer may receive `blocktxn` only for a block Open Bitcoin actually announced to that peer as `cmpctblock`, while current peer eligibility, block availability, resource policy, witness preservation, and malformed-index handling remain explicit and testable.

## Current Gap

- `packages/open-bitcoin-network/src/peer/message_dispatch.rs` currently treats `WireNetworkMessage::GetBlockTxn(_)` as a peer no-op.
- `packages/open-bitcoin-network/src/peer/tests.rs` still contains `phase112_bip152_wire_messages_are_peer_noops`, which encodes the obsolete behavior and must be replaced or renamed.
- `packages/open-bitcoin-node/src/network.rs` constructs outbound compact announcements, but no bounded per-peer provenance token is retained after successful construction.
- Existing compact-block codec support in `packages/open-bitcoin-codec/src/compact_block.rs`, including `expand_block_transaction_indexes`, should remain the canonical index decoder.

## Bitcoin Knots Anchors

- `src/net_processing.cpp`: compact-block announcement depth bound (`MAX_CMPCTBLOCK_DEPTH`), `GETBLOCKTXN` dispatch, and `SendBlockTransactions` malformed-index handling.
- `src/blockencodings.h`: `BlockTransactionsRequest` / `BlockTransactions` wire semantics.
- `test/functional/p2p_compactblocks.py`: successful subset response and silent unservable-request coverage.

The pinned Knots baseline can fall back to a full block for sufficiently old requests. Phase 122 intentionally does not add that behavior: unavailable, ineligible, stale, or unannounced requests are silently suppressed and the parity deviation must be documented.

## Recommended Design

### Functional core

Extend the peer compact-relay state with a bounded FIFO plus membership set of hashes that were actually announced to that peer as compact blocks. Retain shared block data in the existing node inventory/storage surfaces; store only provenance hashes in peer state. Use an inclusive bound matching Knots' ten-block depth window (at most eleven hashes), with deterministic eviction and idempotent insertion.

Add a typed peer action for a syntactically valid `getblocktxn` request. The core should:

1. Reject out-of-bounds or overflowing differential indexes as typed peer misbehavior that disconnects.
2. Emit a serving request only when the hash is present in that peer's bounded announcement provenance.
3. Preserve request order and witness-bearing transaction bytes when forming `BlockTxn`.

Disconnect and peer teardown naturally discard the per-peer state. No persistent provenance is required across process restart; a restarted node has not announced a compact block in the new peer session.

### Imperative shell

Record provenance only after the node has successfully constructed the outbound `CompactBlock`; do not record intent before construction. The node shell should resolve the typed action against current activation/eligibility, block availability, and resource-governance policy, then either return `WireNetworkMessage::BlockTxn`, silently suppress the request, or disconnect on a typed abuse/malformed condition.

Keep benign suppression distinct from malformed-index abuse in action labels and tests. Do not duplicate transactions or full blocks inside peer state.

## Implementation Map

| Concern | Primary files / symbols | Guidance |
| --- | --- | --- |
| Per-peer provenance | `packages/open-bitcoin-network/src/peer/compact_relay.rs`, `peer.rs` | Add bounded FIFO/set state and pure record/query behavior. |
| Wire dispatch | `packages/open-bitcoin-network/src/peer/message_dispatch.rs` | Replace the `GetBlockTxn` no-op with typed validation/action production. |
| Peer behavior tests | `packages/open-bitcoin-network/src/peer/tests.rs` | Cover announced success, unannounced suppression, eviction, disconnect cleanup, ordered indexes, and malformed indexes. |
| Live node seam | `packages/open-bitcoin-node/src/network.rs`, `network/action_translation.rs` | Record only successful compact announcements and translate the typed serving action. |
| Block lookup/response | `packages/open-bitcoin-node/src/network/inventory.rs`, `network/block_serving.rs`, `network/block_relay_evidence.rs` | Reuse existing shared block availability and policy boundaries; build witness-preserving `BlockTxn`. |
| Codec | `packages/open-bitcoin-codec/src/compact_block.rs` | Reuse differential-index expansion and existing BIP152 wire types. |
| Deterministic verifier | `scripts/check-phase122-compact-relay-peer-completion.ts`, matching `.test.ts`, `scripts/verify.sh` | Fixed-file checker with mutation tests and explicit verifier-order wiring. |
| Parity evidence | `docs/parity/index.json`, `docs/parity/checklist.md`, relevant companion doc | Record Knots anchors, live response proof, silent suppression, and the old-block fallback deviation. |

If implementation creates new first-party Rust source or test files, update `docs/parity/source-breadcrumbs.json` and satisfy `scripts/check-parity-breadcrumbs.ts`. Prefer focused edits to existing modules unless a file would otherwise exceed the repository's code-shape limits.

## Verification Strategy

1. Pure state tests: FIFO/set consistency, duplicate announcement behavior, inclusive bound/eviction, peer isolation, and cleanup.
2. Dispatch tests: only peer-announced hashes produce a typed serving action; unannounced requests are silent; malformed differential indexes disconnect with a typed reason.
3. Live node tests: successful compact announcement followed by `getblocktxn` yields ordered witness-preserving `blocktxn`; current ineligibility, unavailable block, and benign resource denial remain silent.
4. Checker tests: valid fixture passes; mutations removing provenance, live translation, malformed-index handling, parity deviation, or `scripts/verify.sh` wiring fail with stable messages.
5. Run focused Cargo tests through `bun run scripts/command-timings.ts run --key <stable-key> -- <command>` during iteration, then `bash scripts/verify.sh` as the final repo-native contract.

## Planning Pitfalls

- Do not mark an announcement before compact-block construction succeeds.
- Do not authorize a request from global block availability alone; provenance is per peer.
- Do not collapse malformed indexes into benign silence.
- Do not let provenance retain block or transaction payloads.
- Do not claim parity for Knots' old-block full-block fallback; document the scoped deviation.
- Do not leave the Phase 112 peer-no-op test name or assertion intact.

## RESEARCH COMPLETE
