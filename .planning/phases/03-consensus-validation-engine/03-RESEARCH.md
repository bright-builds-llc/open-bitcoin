---
phase: 03-consensus-validation-engine
generated_by: gsd-phase-researcher
lifecycle_mode: yolo
phase_lifecycle_id: 3-2026-04-11T21-32-30
generated_at: 2026-04-11T21:34:00.000Z
---

# Phase 3 Research: Consensus Validation Engine

## Baseline anchors

- `consensus/tx_check.cpp` captures the context-free transaction checks that do
  not require UTXO state.
- `validation.cpp` contains the context-free block checks for proof-of-work,
  merkle root integrity, coinbase placement, duplicate-transaction malleation,
  and legacy sigop limits.
- `pow.cpp` defines the compact-target parsing and proof-of-work comparison
  logic that block-header validation depends on.
- `script_tests.json` provides deterministic non-signature script vectors that
  can be mirrored without introducing secp256k1 yet.

## Reusable repo assets

- `open-bitcoin-codec` already serializes block headers and transactions in the
  wire shapes needed for txid, wtxid, merkle root, and proof-of-work hashing.
- `open-bitcoin-primitives` already makes illegal single-output values
  unrepresentable through `Amount`.
- `scripts/verify.sh` already checks formatting, clippy, build, tests, and pure
  core coverage in one repo-native command.

## Chosen execution shape

1. Add a pure-core `open-bitcoin-consensus` crate.
2. Implement a repo-owned SHA-256 so Cargo and Bazel stay aligned without a new
   external dependency layer.
3. Start script parity with deterministic stack, numeric, equality, and hash
   opcodes from upstream vectors.
4. Add typed transaction and block validation APIs around the existing Phase 2
   domain types.
5. Record unsupported signature- and witness-heavy consensus surfaces as
   explicit verification gaps rather than claiming full parity early.
