# Verification Harnesses And Fuzzing

This entry tracks the Phase 9 verification infrastructure for Open Bitcoin.
The behavioral baseline remains Bitcoin Knots `29.3.knots20260210`.

## Coverage

- reusable Rust black-box harness crate under `open-bitcoin-test-harness`
- target-neutral functional cases that can run against Open Bitcoin and a
  Knots-compatible JSON-RPC endpoint without rewriting the suite
- default Open Bitcoin RPC parity suite over the real JSON-RPC HTTP router
- optional Knots-compatible target selected with:
  - `OPEN_BITCOIN_KNOTS_RPC_ADDR`
  - `OPEN_BITCOIN_KNOTS_RPC_USER`
  - `OPEN_BITCOIN_KNOTS_RPC_PASSWORD`
- parallel-safe sandbox, port-reservation, and process-guard helpers
- deterministic property-style codec and protocol tests under normal
  `cargo test`
- JSON and Markdown parity reports written when
  `OPEN_BITCOIN_PARITY_REPORT_DIR` is set
- CI artifact upload for generated parity reports

## Knots sources

- [`packages/bitcoin-knots/test/functional/test_framework`](../../../packages/bitcoin-knots/test/functional/test_framework)
- [`packages/bitcoin-knots/test/functional/interface_rpc.py`](../../../packages/bitcoin-knots/test/functional/interface_rpc.py)
- [`packages/bitcoin-knots/test/functional/interface_bitcoin_cli.py`](../../../packages/bitcoin-knots/test/functional/interface_bitcoin_cli.py)
- [`packages/bitcoin-knots/doc/fuzzing.md`](../../../packages/bitcoin-knots/doc/fuzzing.md)
- [`packages/bitcoin-knots/src/test/fuzz/deserialize.cpp`](../../../packages/bitcoin-knots/src/test/fuzz/deserialize.cpp)
- [`packages/bitcoin-knots/src/test/fuzz/protocol.cpp`](../../../packages/bitcoin-knots/src/test/fuzz/protocol.cpp)
- [`packages/bitcoin-knots/src/test/fuzz/primitives_transaction.cpp`](../../../packages/bitcoin-knots/src/test/fuzz/primitives_transaction.cpp)

## First-party implementation

- [`packages/open-bitcoin-test-harness/src/case.rs`](../../../packages/open-bitcoin-test-harness/src/case.rs)
- [`packages/open-bitcoin-test-harness/src/target.rs`](../../../packages/open-bitcoin-test-harness/src/target.rs)
- [`packages/open-bitcoin-test-harness/src/isolation.rs`](../../../packages/open-bitcoin-test-harness/src/isolation.rs)
- [`packages/open-bitcoin-test-harness/src/report.rs`](../../../packages/open-bitcoin-test-harness/src/report.rs)
- [`packages/open-bitcoin-rpc/tests/black_box_parity.rs`](../../../packages/open-bitcoin-rpc/tests/black_box_parity.rs)
- [`packages/open-bitcoin-codec/tests/properties.rs`](../../../packages/open-bitcoin-codec/tests/properties.rs)
- [`packages/open-bitcoin-network/tests/properties.rs`](../../../packages/open-bitcoin-network/tests/properties.rs)
- [`scripts/verify.sh`](../../../scripts/verify.sh)
- [`.github/workflows/ci.yml`](../../../.github/workflows/ci.yml)

## Knots behaviors mirrored here

- JSON-RPC parity tests operate at the authenticated HTTP request/response
  boundary instead of calling Open Bitcoin dispatch helpers directly.
- The same functional cases can target Open Bitcoin or a configured external
  Knots-compatible endpoint.
- Parser, serializer, and wire-message property tests exercise generated cases
  and checksum failures with deterministic reproduction.
- Integration helpers avoid hard-coded ports and shared data directories so
  tests can run in parallel.

## Known gaps

- The harness does not yet build or spawn the vendored Knots daemon itself.
- The upstream Python functional suite has not been translated wholesale.
- Property-style tests are deterministic generated tests under `cargo test`,
  not a full cargo-fuzz/libFuzzer runtime.

## Follow-up triggers

Update this entry when the harness starts managing a built Knots process, when
additional upstream functional cases move into the shared suite, or when the
project adds a dedicated fuzzing runtime.
