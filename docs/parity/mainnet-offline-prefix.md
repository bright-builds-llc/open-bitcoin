# Bounded offline Mainnet parity

`MAINNET-PREFIX-01` covered the first 200 descendants.
`MAINNET-PREFIX-02` extends the same local route through the first difficulty retarget. Starting from canonical
Mainnet genesis, both daemons replay the exact serialized blocks at heights
1–2016 from empty datadirs. At every height, authenticated `submitblock` must
succeed and `getblockchaininfo` must show the expected tip. At height 2016,
compact target, chainwork and the complete UTXO projection must match. After clean
shutdown, Open Bitcoin must reopen the same datadir and report the same
tip and work. The closed-state observer compares the recovered coins.
The separately supplied height-2017 block exercises the local admission bound.

## Operator route and limits

Use `-chain=main -server=1 -rpcbind=127.0.0.1 -blockfilterindex=0` with an
explicit isolated datadir and authenticated RPC. The route requires
loopback RPC, disabled pruning, and disabled network services. It accepts
only a direct extension of the current active tip, applies ordinary
Mainnet block validation, and caps descendant admission at height 2016.
The generic ordinary-history and peer activation gates remain closed.
The scope covers the first difficulty retarget. Later retargets, peer sync,
Mainnet reorgs, the canonical-history wallet, and whole-chain validation
require separate evidence.

The pinned Knots anchors are
[Mainnet chain parameters](../../packages/bitcoin-knots/src/kernel/chainparams.cpp),
[block validation](../../packages/bitcoin-knots/src/validation.cpp),
[target and work rules](../../packages/bitcoin-knots/src/pow.cpp),
and [chainstate RPC](../../packages/bitcoin-knots/src/rpc/blockchain.cpp).
The height cap and direct-extension restriction are intentional local
admission differences.

## Reproduction and evidence

The tracked hex lines are raw Mainnet block bytes collected from public block
archive APIs, including Blockstream and BTCScan. The collector checked payload
sizes and header hashes against the linked Mainnet history and corroborated the
height-2016 hash with mempool.space. The offline reader checks linkage from the
pinned genesis, endpoint identities, and the retarget inputs and compact bits.
Knots and Open Bitcoin each independently validate the full block bodies.
The daemon comparison runs entirely offline.
[Esplora API](https://github.com/Blockstream/esplora/blob/master/API.md#blocks).

```sh
bun run scripts/run-knots-mainnet-prefix.ts
bun run scripts/check-mainnet-prefix.ts
```

The runner builds Knots from its pinned source archive and Open Bitcoin
in release mode from the committed checkout. It disables Knots networking and assumevalid.
The receipt binds both executable hashes, the agreed active-tip sequence,
the corpus, both retarget timestamps, the Open Bitcoin restart, and its recovered coins. The default
offline verifier checks the recorded observations and qualification input
freshness. The reproduction command runs the actual daemons separately.

The coin projection uses display-order transaction ID, output index,
integer satoshis, raw script bytes, coinbase status, and creation height.
Knots supplies it through `dumptxoutset`. The test-only Open Bitcoin observer
opens the closed datadir through the complete retained-history proof and reads
its detached snapshot. The digest covers every projected row.

## First retarget

The pinned [proof-of-work implementation](../../packages/bitcoin-knots/src/pow.cpp)
uses the genesis timestamp, 1,231,006,505, and the height-2015 timestamp,
1,233,061,996. The elapsed 2,055,491 seconds falls within the retarget's
time clamp. It exceeds the 1,209,600-second target interval, so the new
target is capped at the Mainnet proof-of-work limit. The compact target
therefore stays at `0x1d00ffff` for height 2016.

The [chainstate tests](../../packages/open-bitcoin-chainstate/tests/mainnet_prefix.rs)
check the height bound and refusal of replacement admission. The
[header-store tests](../../packages/open-bitcoin-network/tests/mainnet_prefix.rs)
check the earned genesis anchor for the retarget and refusal at height 2017.

The original height-200 result remains available at
[commit 807cba4b](https://github.com/bright-builds-llc/open-bitcoin/blob/807cba4bf4ddff7d54272a961c3499912c5caa57/docs/parity/receipts/mainnet-prefix.json).
