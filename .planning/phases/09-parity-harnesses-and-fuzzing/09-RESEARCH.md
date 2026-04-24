---
phase: 09-parity-harnesses-and-fuzzing
generated_by: gsd-phase-researcher
lifecycle_mode: yolo
phase_lifecycle_id: 9-2026-04-24T10-06-16
generated_at: 2026-04-24T10:08:00Z
---

# Phase 9 Research: Parity Harnesses and Fuzzing

## RESEARCH COMPLETE

## Planning-Relevant Findings

### Cross-implementation functional harness

The highest-value first slice is a small Rust harness crate that models
externally observable JSON-RPC behavior as reusable `FunctionalCase` values.
Open Bitcoin can run these cases against its real HTTP router in-process, while
Knots-compatible behavior can be exercised against an already-running reference
RPC endpoint configured by environment variables. This proves the same suite can
target both implementations without forcing the repository to build and spawn
Knots during normal verification.

The target boundary should be request/response shaped, not direct dispatcher
calls, so authentication, HTTP status handling, JSON-RPC envelopes, and method
normalization stay in the tested path.

### Parallel-safe integration runs

The existing `open-bitcoin-cli` operator-flow tests already use unique temp
directories and localhost ephemeral ports. Phase 9 should move that pattern into
a reusable harness with:

- unique sandbox directories under `std::env::temp_dir()`
- `127.0.0.1:0` port reservation helpers
- best-effort child process cleanup on `Drop`
- tests proving sibling sandboxes and port reservations differ

These helpers belong in test/adaptor code, not in pure-core crates.

### Property-style tests

The repo can satisfy the first `PAR-01` slice without adding cargo-fuzz or
libFuzzer by adding deterministic generated-case tests under normal `cargo
test`. Good first targets are:

- transaction encode/decode round trips with and without witness data
- network address, inventory, and message-header codec round trips
- wire-message encode/decode invariants for handshake/control/inventory flows
- malformed payload handling that returns typed errors instead of panics

Generated values should be bounded and reproducible so failures are stable in
CI and local reproduction.

### CI and audit reporting

`scripts/verify.sh` is the repo-native verification contract and should remain
the single local and CI entrypoint. The harness can write JSON and Markdown
reports when `OPEN_BITCOIN_PARITY_REPORT_DIR` is set. CI can set that variable
and upload the directory as an artifact while the test process itself blocks
regressions through normal failures.

### Dependency posture

Avoid new runtime dependencies. The harness can reuse existing `serde_json` and
`base64` workspace dependencies and the standard library. Deterministic
property-style tests can use a tiny local generator instead of adding `proptest`
or `quickcheck` in this phase.

## Recommended Plan Shape

1. Create `open-bitcoin-test-harness` with cases, targets, isolation, and report
   helpers, plus an Open Bitcoin RPC parity suite and optional external Knots
   target.
2. Prove process/port/data-dir isolation through harness tests and wire report
   generation into the black-box suite.
3. Add deterministic property-style codec and network tests.
4. Update verify/CI/parity docs and run the full repo verification contract.

