# Bounded offline Mainnet parity

`MAINNET-PREFIX-01` covers one local product route. Starting from canonical
Mainnet genesis, both daemons replay the exact serialized blocks at heights
1–200 from empty datadirs. At every height, authenticated `submitblock` must
succeed and `getblockchaininfo` must show the expected tip. At height 200,
chainwork and the complete UTXO projection must match. After clean
shutdown, Open Bitcoin must reopen the same datadir and report the same
tip and work. The closed-state observer compares the recovered coins.
The separately supplied height-201 block exercises the local admission bound.

## Operator route and limits

Use `-chain=main -server=1 -rpcbind=127.0.0.1 -blockfilterindex=0` with an
explicit isolated datadir and authenticated RPC. The route requires
loopback RPC, disabled pruning, and disabled network services. It accepts
only a direct extension of the current active tip, applies ordinary
Mainnet block validation, and caps descendant admission at height 200.
The generic ordinary-history and peer activation gates remain closed.
Canonical-history wallet authority, Mainnet reorgs, retargets, public
sync, and whole-chain validation are outside this scope.

The pinned Knots anchors are
[Mainnet chain parameters](../../packages/bitcoin-knots/src/kernel/chainparams.cpp),
[block validation](../../packages/bitcoin-knots/src/validation.cpp),
[target and work rules](../../packages/bitcoin-knots/src/pow.cpp),
and [chainstate RPC](../../packages/bitcoin-knots/src/rpc/blockchain.cpp).
The height cap and direct-extension restriction are intentional local
admission differences.

## Reproduction and evidence

The tracked hex lines are raw Mainnet block bytes. They were obtained from
Blockstream's Esplora API. The height-200 hash was corroborated with the
mempool.space API. The reader verifies sequential header linkage from the
pinned genesis, the fixed endpoint identities, and compact target bits.
Knots and Open Bitcoin each independently validate the full block bodies.
No network access is needed to read the fixture or submit the blocks.

```sh
bun run scripts/run-knots-mainnet-prefix.ts
bun run scripts/check-mainnet-prefix.ts
```

The runner builds Knots from its pinned source archive and Open Bitcoin
from the committed checkout. It disables Knots networking and assumevalid.
The receipt binds both executable hashes, the agreed active-tip sequence,
the corpus, the Open Bitcoin restart, and its recovered coins. The default
offline verifier checks the recorded observations and qualification input
freshness. The reproduction command runs the actual daemons separately.

The coin projection uses display-order transaction ID, output index,
integer satoshis, raw script bytes, coinbase status, and creation height.
Knots supplies it through `dumptxoutset`. The test-only Open Bitcoin observer
opens the closed datadir through the complete retained-history proof and reads
its detached snapshot. The digest covers every projected row.
