# v2.1 Architecture Research - Block Serving and Compact Block Relay Boundary

**Defined:** 2026-07-03
**Milestone:** v2.1 Block Serving and Compact Block Relay Boundary

## Architecture Direction

Keep v2.1 aligned with the project architecture: pure protocol and relay decisions in first-party core/network crates, storage/mempool/runtime effects in `open-bitcoin-node`, and operator surfaces as projections of shared status contracts.

The milestone should introduce a block relay boundary parallel to the v2.0 transaction relay boundary. Full block serving, compact block negotiation, compact reconstruction, missing transaction requests, fallback, resource governance, and operator evidence should be explicit typed surfaces, not incidental behavior inside socket handling.

## Proposed Module Boundaries

### Codec Layer

Add BIP152 payload support to `open-bitcoin-codec` and expose it through `open-bitcoin-network::message`.

Recommended shapes:

- `SendCmpctMessage { announce: bool, version: u64 }`
- `CompactBlock { header, nonce, short_ids, prefilled_transactions }`
- `PrefilledTransaction { index_delta, transaction }`
- `BlockTransactionsRequest { block_hash, indexes }`
- `BlockTransactions { block_hash, transactions }`

The codec should own payload correctness: compact-size bounds, six-byte short IDs, differential indexes, non-canonical payloads, and trailing data. It should not own peer policy.

### Pure Network Policy

Add focused policy/state modules under `open-bitcoin-network`, likely:

- `peer/block_serving.rs` for serving eligibility, block request classification, and outcome labels.
- `peer/compact_relay.rs` for `sendcmpct` negotiation, high-bandwidth preference, compact announcement decisions, and per-peer compact capability evidence.
- `peer/compact_reconstruction.rs` if reconstruction can stay pure over transaction/hash inputs.

These modules should emit typed actions:

- serve full block
- serve compact block
- request missing block transactions
- process provided block transactions
- fall back to full block
- suppress request
- record misbehavior or disconnect
- clear in-flight partial state

### Node Shell

`open-bitcoin-node` should own:

- reading blocks from in-memory fixtures or durable store adapters
- building compact blocks from full blocks
- building bounded mempool and extra transaction inputs for reconstruction
- holding partial compact-block state
- calling existing block validation/connect behavior
- emitting metrics/log/status/support evidence

This avoids direct storage or mempool mutation from pure peer logic.

### Status And Operator Surfaces

Extend `status/relay_evidence.rs` or add a sibling block-relay status module with stable fields:

- activation state
- eligible and ineligible peer counts
- compact negotiation counts
- block serving outcome counters
- compact reconstruction counters
- fallback counters
- no-claim/public-default boundary

RPC, CLI, dashboard, metrics, logs, and support bundles should all read from this contract.

## Data Flow

### Full Block Serving

1. Peer sends `getdata` for block, witness block, or compact block inventory.
2. Pure policy classifies peer eligibility, inventory type, request limits, and block status.
3. Node shell reads the validated block only after policy permits serving.
4. Node shell sends full block, witness block, compact block, fallback, or suppresses response.
5. Status/metrics/logs record a low-cardinality outcome.

### Compact Block Announcement

1. Handshake reaches compact-block-capable protocol version.
2. Local node sends or accepts `sendcmpct` only within activation rules.
3. Peer compact preferences update per-peer state.
4. New valid blocks are announced as compact blocks only when peer negotiation and header state allow it.
5. Otherwise the node uses the existing headers or inventory announcement path.

### Compact Block Reception

1. Decode `cmpctblock`.
2. Validate header connectivity and work before partial state.
3. Initialize partial reconstruction with mempool and bounded extra transactions.
4. If complete, validate/connect the reconstructed block.
5. If missing transactions, send bounded `getblocktxn`.
6. If reconstruction fails or state is not eligible, fall back to full block fetch or suppress.

### `blocktxn` Response

1. Decode `blocktxn`.
2. Match by block hash to an expected in-flight partial compact block from the same peer.
3. Fill missing transactions and run mutation/merkle/witness checks.
4. On success, pass the complete block to validation.
5. On mismatch, duplicate response, collision, timeout, or malformed payload, clear or retain in-flight state according to the fallback policy.

## State Ownership

Keep volatile compact relay state out of durable stores:

- per-peer compact negotiation flags
- partial compact-block reconstructions
- missing transaction indexes
- compact relay timeout/fallback state
- recent compact block cache

Durable stores should keep validated blocks, chainstate, and existing mempool recovery facts. If a restart happens mid-reconstruction, the node should restart with no stale partial compact block.

## Phase-Friendly Integration Points

- `packages/open-bitcoin-network/src/message.rs` and tests for wire support.
- `packages/open-bitcoin-network/src/peer.rs` for per-peer negotiation and actions.
- `packages/open-bitcoin-node/src/network.rs` for shell integration and block serving cache/store reads.
- `packages/open-bitcoin-node/src/status/` for evidence contracts.
- `packages/open-bitcoin-cli/src/operator/status/`, dashboard, and support renderers for operator evidence.
- `docs/parity/`, `docs/operator/runtime-guide.md`, `README.md`, and verifier scripts for release boundary.

## Architecture Checks

- Pure modules must not read disk, spawn tasks, write logs, or mutate chainstate.
- Codec modules must not encode policy.
- Node shell must not infer eligibility by string matching policy messages.
- Operator surfaces must not maintain their own compact-relay truth.
- Deterministic tests should validate the public contract before any optional public-network review.
