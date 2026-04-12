# Consensus Validation Engine

This entry tracks the Phase 3 consensus surface currently implemented in Open
Bitcoin. The behavioral baseline remains Bitcoin Knots `29.3.knots20260210`.

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

- `CHECKSIG`, `CHECKMULTISIG`, P2SH, and witness-program execution are not
  implemented yet.
- Taproot key-path, script-path, and tapscript execution are not implemented
  yet.
- The current deterministic parity fixtures cover the implemented slice only,
  not the full in-scope consensus surface.

## Follow-up triggers

Update this entry when Phase 3 or later work:

- adds signature or witness-program execution
- expands contextual validation beyond the current explicit-height and
  explicit-time inputs
- expands the parity fixture corpus beyond the current deterministic script and
  block-header checks
