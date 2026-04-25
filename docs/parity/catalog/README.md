# Parity Catalog

This directory is the human-readable companion to
[`../index.json`](../index.json). It records the subsystem-level evidence,
Knots source anchors, quirks, gaps, and suspected unknowns behind the
machine-readable parity ledger.

## Conventions

- Keep [`../index.json`](../index.json) as the machine-readable root.
- Add one Markdown entry per subsystem-sized surface or phase-sized audit slice.
- For each entry, record:
  - the in-scope features and boundaries being tracked
  - the concrete Knots source files and tests or vectors that anchor the note
  - notable quirks that Open Bitcoin must preserve intentionally
  - confirmed bugs, if any are known
  - suspected unknowns that later phases still need to audit
- Treat catalog pages as evidence and review maps, not as replacement
  specifications for the Rust implementation.

## Current entries

| Entry | Status | Scope |
| --- | --- | --- |
| [`core-domain-and-serialization.md`](core-domain-and-serialization.md) | `done` | Amounts, hashes, serialization primitives, scripts, transactions, blocks, and protocol framing reused across the workspace |
| [`consensus-validation.md`](consensus-validation.md) | `done` | Script execution, proof-of-work, merkle roots, and typed transaction or block validation outcomes |
| [`chainstate.md`](chainstate.md) | `done` | UTXO state, connect/disconnect, reorg selection, and adapter-owned persistence boundaries |
| [`mempool-policy.md`](mempool-policy.md) | `done` | Admission, replacement, accounting, eviction, and node-side policy orchestration |
| [`p2p.md`](p2p.md) | `done` | Peer lifecycle, wire handling, header/block sync, and txid/wtxid-aware relay |
| [`wallet.md`](wallet.md) | `done` | Descriptor wallets, addresses, balances, coin selection, signing, and adapter-owned persistence |
| [`rpc-cli-config.md`](rpc-cli-config.md) | `done` | Supported JSON-RPC, `bitcoin-cli`-style behavior, config, auth, and deferred operator surfaces |
| [`verification-harnesses.md`](verification-harnesses.md) | `done` | Cross-implementation parity harnesses, parallel-safe integration isolation, property-style tests, and CI report output |

## Maintenance

Update a catalog entry when a parity surface gains new behavior, a deferred
surface becomes supported, a suspected unknown is resolved, or an intentional
in-scope deviation is added to [`../index.json`](../index.json).
