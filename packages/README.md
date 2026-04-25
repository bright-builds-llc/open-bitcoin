# Packages

This directory holds both the pinned upstream reference baseline and first-party
Open Bitcoin crates.

- `bitcoin-knots/` is the vendored upstream behavioral baseline. Treat it as read-only from the perspective of first-party implementation work.
- `open-bitcoin-primitives/` owns shared low-level value types used across the workspace.
- `open-bitcoin-codec/` owns Bitcoin byte encoding, decoding, and wire-framing helpers.
- `open-bitcoin-core/` re-exports the first-party pure-core surface for downstream package boundaries.
- `open-bitcoin-consensus/` owns script execution, transaction checks, block checks, proof-of-work, merkle behavior, and typed validation errors.
- `open-bitcoin-chainstate/` owns pure-core UTXO state, undo data, active-chain mutation, and reorg behavior.
- `open-bitcoin-mempool/` owns policy admission, replacement, ancestor/descendant accounting, and eviction behavior.
- `open-bitcoin-network/` owns peer lifecycle, wire-message handling, sync planning, and relay state.
- `open-bitcoin-wallet/` owns descriptor parsing, address derivation, balance tracking, coin selection, transaction building, and signing.
- `open-bitcoin-node/` owns adapter-facing orchestration over chainstate, mempool, networking, and wallet state.
- `open-bitcoin-rpc/` owns JSON-RPC envelopes, config loading, method dispatch, HTTP serving, and the `open-bitcoind` binary.
- `open-bitcoin-cli/` owns the `open-bitcoin-cli` command-line client and supported `bitcoin-cli`-style startup behavior.
- `open-bitcoin-test-harness/` owns reusable black-box parity cases, target adapters, isolation helpers, and parity report generation.
- `open-bitcoin-bench/` owns deterministic benchmark groups and JSON/Markdown report generation.

First-party crates should depend on each other intentionally. Pure-core crates
must not depend on shell/runtime crates. Adapter and executable crates may
depend on pure-core crates, but I/O and runtime effects should stay outside the
pure-core packages.
