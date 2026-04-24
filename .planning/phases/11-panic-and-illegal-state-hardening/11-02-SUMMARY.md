# Phase 11 Plan 02 Summary: Replace Reachable Caller-Facing Crash Paths

## Outcome

Replaced reachable production panic-like paths with typed errors or
non-panicking control flow while preserving external Bitcoin, RPC, CLI, wallet,
mempool, networking, and consensus behavior.

Primary changed areas:

- Mempool: admission validation, weight calculation, replacement state, eviction
  selection, and relation recomputation.
- Consensus: block and contextual transaction validation, taproot witness
  extraction, script helpers, Merkle helpers, difficulty bytes, and witness
  script construction.
- Wallet: transaction vsize estimation, nested segwit script building, taproot
  signing, and oversized push-data handling.
- Adapters and tooling: CLI stdin errors, wire command decoding, codec array
  reads, benchmark argument parsing, and report serialization.

## Tests

Added or updated focused coverage for changed behavior, including mempool
internal invariant reporting and wallet oversized script push handling.

## Verification

- `cargo fmt --manifest-path packages/Cargo.toml --all`
- `cargo clippy --manifest-path packages/Cargo.toml --workspace --all-targets --all-features -- -D warnings`
- `cargo build --manifest-path packages/Cargo.toml --workspace --all-targets --all-features`
- `cargo test --manifest-path packages/Cargo.toml --workspace --all-features`
