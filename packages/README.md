# Packages

This directory holds both the pinned upstream reference baseline and first-party code.

- `bitcoin-knots/` is the vendored upstream behavioral baseline. Treat it as read-only from the perspective of first-party implementation work.
- `open-bitcoin-chainstate/` is the pure-core chainstate crate that owns UTXO state, undo data, and active-chain mutation logic.
- `open-bitcoin-core/` is the first pure-core Rust crate for domain logic that must stay free of direct I/O and runtime side effects.
- `open-bitcoin-consensus/` is the pure-core consensus and validation crate that owns script, transaction, and block checks.
- `open-bitcoin-node/` is the shell/runtime crate that will own adapters, orchestration, and effectful boundaries.

First-party crates should depend on each other intentionally. Shell crates may depend on pure-core crates, but pure-core crates must not depend on shell/runtime crates.
