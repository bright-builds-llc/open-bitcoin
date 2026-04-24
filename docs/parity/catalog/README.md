# Parity Catalog

This directory is the human-readable companion to [`../index.json`](../index.json). It exists to satisfy `REF-03` without turning parity knowledge into scattered TODOs or tribal memory.

## Conventions

- Keep [`../index.json`](../index.json) as the machine-readable root.
- Add one Markdown entry per subsystem-sized surface or phase-sized audit slice.
- For each entry, record:
  - the in-scope features and boundaries being tracked
  - the concrete Knots source files and tests or vectors that anchor the note
  - notable quirks that Open Bitcoin must preserve intentionally
  - confirmed bugs, if any are known
  - suspected unknowns that later phases still need to audit

## Current entries

| Entry | Scope | Phase |
| --- | --- | --- |
| [`core-domain-and-serialization.md`](core-domain-and-serialization.md) | Amounts, hashes, serialization primitives, scripts, transactions, blocks, and protocol framing reused by later phases | 2 |
| [`consensus-validation.md`](consensus-validation.md) | Script execution, proof-of-work, merkle roots, and typed transaction or block validation outcomes currently implemented in the pure core | 3 |
| [`chainstate.md`](chainstate.md) | UTXO state, connect/disconnect, reorg selection, and adapter-owned persistence boundary | 4 |
| [`mempool-policy.md`](mempool-policy.md) | Admission, replacement, accounting, eviction, and thin node-side policy orchestration | 5 |
| [`p2p.md`](p2p.md) | Peer lifecycle, wire handling, header/block sync, and txid/wtxid-aware relay | 6 |
| [`wallet.md`](wallet.md) | Descriptor wallets, addresses, balances, coin selection, signing, and adapter-owned persistence | 7 |
| [`rpc-cli-config.md`](rpc-cli-config.md) | Supported JSON-RPC, bitcoin-cli, config, auth, and deferred operator surfaces | 8 |
| [`verification-harnesses.md`](verification-harnesses.md) | Cross-implementation parity harnesses, parallel-safe integration isolation, property-style tests, and CI report output | 9 |
