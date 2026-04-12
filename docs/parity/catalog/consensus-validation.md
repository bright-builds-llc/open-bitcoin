# Consensus Validation Engine

This entry tracks the Phase 3 consensus foundation currently implemented in
Open Bitcoin. The behavioral baseline remains Bitcoin Knots
`29.3.knots20260210`.

## Coverage

- compact-target parsing and proof-of-work comparison
- transaction txid and wtxid hashing from Phase 2 codecs
- merkle-root construction and duplicate-transaction malleation detection
- deterministic non-signature script evaluation for stack, equality, numeric,
  and SHA-256 or HASH256 operations
- context-free transaction and block validation with typed reject reasons
- explicit transaction and block validation contexts for finality, sequence
  locks, coinbase maturity, coinbase-height, witness commitment, and block
  weight checks
- script classification, legacy/segwit sighash scaffolding, signature parsing,
  full legacy `CHECKSIG` or `CHECKMULTISIG` execution, `P2PKH`, and the first
  repo-owned `HASH160` support needed by later script paths
- canonical `P2SH` redeem-script execution
- native and nested segwit-v0 witness-program execution for `P2WPKH`, `P2WSH`,
  `P2SH-P2WPKH`, and `P2SH-P2WSH`
- taproot key-path verification with repo-owned taproot sighash support and
  Schnorr validation
- tapscript script-path execution with control-block or tapleaf validation,
  annex handling, CODESEPARATOR tracking, and `CHECKSIGADD`
- taproot discouragement or upgrade-flag enforcement for supported witness-v1
  execution points
- imported upstream `sighash.json` coverage through the first-party legacy
  signature-hash implementation
- imported upstream `script_tests.json` anchors for witness mismatch,
  malleation, wrong-length, compressed-key, and witness-multisig behavior
- repo-owned deterministic fixtures for coinbase maturity, absolute locktime,
  sequence locks, witness commitment success or failure, and unexpected witness
  data
- split sigop accounting across legacy, `P2SH`, and witness paths in the
  contextual validation surface

## Knots sources

- [`packages/bitcoin-knots/src/script/script.h`](../../../packages/bitcoin-knots/src/script/script.h)
- [`packages/bitcoin-knots/src/script/interpreter.cpp`](../../../packages/bitcoin-knots/src/script/interpreter.cpp)
- [`packages/bitcoin-knots/src/consensus/tx_check.cpp`](../../../packages/bitcoin-knots/src/consensus/tx_check.cpp)
- [`packages/bitcoin-knots/src/consensus/validation.h`](../../../packages/bitcoin-knots/src/consensus/validation.h)
- [`packages/bitcoin-knots/src/validation.cpp`](../../../packages/bitcoin-knots/src/validation.cpp)
- [`packages/bitcoin-knots/src/pow.cpp`](../../../packages/bitcoin-knots/src/pow.cpp)

## Knots tests and vectors

- [`packages/bitcoin-knots/src/test/data/script_tests.json`](../../../packages/bitcoin-knots/src/test/data/script_tests.json)
- [`packages/open-bitcoin-codec/testdata/block_header.hex`](../../../packages/open-bitcoin-codec/testdata/block_header.hex)

## Implemented quirks to preserve

- Compact targets are decoded with the same sign, zero, and overflow rejection
  shape as Knots' proof-of-work helpers.
- Merkle validation distinguishes plain root mismatch from duplicate-transaction
  malleation.
- Transaction and block validation expose stable reject reasons instead of
  silent booleans.
- Contextual transaction validation takes explicit spend-height and
  median-time-past inputs instead of reaching into chainstate directly.
- Witness commitment validation uses the coinbase reserved value plus the
  witness merkle root and rejects unexpected witness data when segwit is not
  expected.

## Known gaps

- None for the in-scope Phase 3 consensus surface. Later phases still own
  chainstate, mempool, networking, wallet, and interface parity.

## Follow-up triggers

Update this entry when later phases change consensus behavior or extend the
parity corpus beyond the current Phase 3 surface.
