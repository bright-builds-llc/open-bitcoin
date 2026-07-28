# Packages

This directory holds both the pinned upstream reference baseline and first-party
Open Bitcoin crates.

- `bitcoin-knots/` is the vendored upstream behavioral baseline. Treat it as read-only from the perspective of first-party implementation work.
- `open-bitcoin-primitives/` owns shared low-level value types used across the workspace.
- `open-bitcoin-codec/` owns Bitcoin byte encoding, decoding, and wire-framing helpers.
- `open-bitcoin-core/` re-exports the first-party pure-core surface for downstream package boundaries.
- `open-bitcoin-consensus/` owns script execution, transaction checks, block checks, proof-of-work, merkle behavior, and typed validation errors.
- `open-bitcoin-chainstate/` owns pure-core UTXO state, undo data, active-chain mutation, and reorg behavior.
- `open-bitcoin-mempool/` owns policy admission, replacement, ancestor/descendant accounting, eviction behavior, typed resource, fee, metadata, and lifecycle contracts, and bounded local pure-core package admission with ordered dry-run and staged-submit results.
- `open-bitcoin-network/` owns peer lifecycle, wire-message handling, sync planning, relay state, injected retry-input contracts, bounded reject evidence, and neutral same-peer one-parent/one-child candidate proofs.
- `open-bitcoin-wallet/` owns descriptor parsing, address derivation, balance tracking, coin selection, transaction building, and signing.
- `open-bitcoin-node/` owns adapter-facing orchestration over chainstate, mempool, networking, and wallet state, including the sole `ManagedNetworkHandle` lifecycle authority, complete cross-cache projection, bounded reconciliation, typed peer/snapshot effects, and the single package-admission call for an eligible peer candidate.
- `open-bitcoin-rpc/` owns JSON-RPC envelopes, config loading, method dispatch, HTTP serving, the `open-bitcoind` binary, and truthful resource and fee RPC projection.
- `open-bitcoin-cli/` owns the `open-bitcoin-cli` command-line client and supported `bitcoin-cli`-style startup behavior.
- `open-bitcoin-test-harness/` owns reusable black-box parity cases, target adapters, isolation helpers, and parity report generation.
- `open-bitcoin-bench/` owns deterministic benchmark groups and JSON/Markdown report generation.

First-party crates should depend on each other intentionally. Pure-core crates
must not depend on shell/runtime crates. Adapter and executable crates may
depend on pure-core crates, but I/O and runtime effects should stay outside the
pure-core packages.

The mempool package surface remains local and effect-free. Phase 133 adds a
bounded network-to-node bridge: ordinary `inv`/`getdata`/`tx` flow may yield a
newest-first same-peer one-parent/one-child candidate, and the node owns one
authoritative Phase 132 package-admission call plus typed feedback. Phase 134
projects committed lifecycle facts through the sole `ManagedNetworkHandle`
authority into serving, ordinary fanout state, peer state, compact inputs,
unbroadcast membership, persistence dirtiness, and bounded evidence. Peer and
current-schema snapshot effects use typed prepare/execute/complete boundaries
with I/O outside the authority lock. Phase 135 snapshot schema/recovery, Phase
136 receive-independent scheduling and package fanout, Phase 137 operator
surfaces, Phase 138 release proof, general package wire relay, whole-mempool
rebroadcast, arbitrary multi-parent peer assembly, public or default relay,
guaranteed propagation, public-network CI or release gates, and production
readiness remain deferred.
