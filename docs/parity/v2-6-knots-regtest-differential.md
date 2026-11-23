# Pinned Knots Regtest Differential Runner

`v2-6-cdif-01` owns `CDIF-01`. The [tracked receipt](receipts/cdif-01-regtest.json)
records an actual comparison between Knots `29.3.knots20260210` at
`a9aee730466ac67d35a3c03ee24676be5e045878` and ordinary Open Bitcoin
`open-bitcoind`. Both began at canonical Regtest genesis, accepted the same
serialized height-one block through authenticated `submitblock` RPC,
reported best block
`194ad8077ee474b008cc5fb29bade1fd6cda566c44972e2d614f1bf586762d24`
at height 1, and exited cleanly.

## Maturity and halvings

`v2-6-cdif-02` owns `CDIF-02`. The
[history receipt](receipts/cdif-02-regtest.json) records both daemons starting
at canonical Regtest genesis and accepting the same 301 serialized descendants
in order. The runner calls `getblockchaininfo` after each successful
`submitblock` and checks the active height and hash against the submitted
block. Both processes exited cleanly after reaching height 301.

The accepted block at height 101 spends output zero of the height-one
coinbase at the default 100-block maturity boundary. The runner decodes
the checked-in input bytes to derive that outpoint, its spending transaction,
and the following fee-free coinbase rewards.

| Height | Subsidy |
| -- | -- |
| 149 | 50 BTC |
| 150 | 25 BTC |
| 299 | 25 BTC |
| 300 | 12.5 BTC |

Each process receipt records the accepted count, matching checkpoints, and a
SHA-256 of the JSON array of `[height, hash]` pairs observed after all 301 RPC
submissions. The offline ledger checker checks that digest, the checkpoint
tips, and the maturity and subsidy facts against the canonical corpus.

## Invalid blocks and spends

`v2-6-cdif-03` covers the 19 invalid inputs in the canonical Regtest
corpus. Each case submits its exact serialized bytes against its canonical
parent, while positive submissions continue the chain. The lower cases run
through height 201. A second fresh datadir per daemon replays the positive
history through height 301 with the height-299 and height-300 overpayment
cases. The split leaves room for rejected header identities within Open
Bitcoin's bounded retained history.

The [rejection receipt](receipts/cdif-03-regtest.json) identifies each input
by its wire SHA-256 and header hash. The `submitblock-regtest-v1`
normalization preserves specific BIP22 reasons verbatim. An RPC fault, an
ambiguous status, an unexpected reason or an accepted invalid input fails the
comparison. For each rejection, the runner checks that the active height and
hash, the active tip's BASIC header and filter digest, and the empty mempool
are unchanged on each daemon. The subsequent positive acceptance exercises
the still-usable active history, including valid blocks that share a header
with malformed witness variants.

## Reproduce

Use the repo-pinned Bun and Rust toolchains, Git, CMake, Ninja, and Knots'
headless build dependencies (C++20 compiler, Boost, libevent, and pkg-config).
The recorded run used macOS arm64. See the pinned Knots
[macOS build guide](../../packages/bitcoin-knots/doc/build-osx.md) and
[Unix build guide](../../packages/bitcoin-knots/doc/build-unix.md) for system
packages. Run from the Open Bitcoin repository root with a committed runtime
source tree:

```sh
git submodule update --init --recursive
bun run scripts/run-knots-regtest-differential.ts
bun run scripts/run-knots-regtest-differential.ts --cdif-02
bun run scripts/run-knots-regtest-differential.ts --cdif-03
bun run scripts/check-parity-ledger.ts
```

[The runner](../../scripts/run-knots-regtest-differential.ts) checks the
checkout and gitlink against the approved Knots commit. It configures and builds
`bitcoind` from a fresh Git archive of that commit. The receipt records the
archive and tree identities, build options, compiler and dependencies, log
digests, and the SHA-256 of the daemon actually launched. The Open Bitcoin
binary is built by Cargo and copied under the shared target lock. Its commit
and executable SHA-256 appear separately in the receipt.

The runner allocates fresh homes and disposable datadirs, with distinct
credentials and authenticated RPC bound to loopback. Knots starts with
`-assumevalid=0` and P2P networking disabled. On macOS the launcher caps
Knots' process-local soft file-descriptor limit at 1024 to avoid the pinned
daemon's conversion of an unlimited inherited limit to a negative integer.
Open Bitcoin starts through its default-off networking route with an explicit
BASIC index, which selects ordinary durable Regtest validation. Both report
zero peers. The default command submits row 1 of the
[canonical Regtest corpus](../../packages/testdata/phase167/regtest-history.json).
The extended commands check the serialized positive blocks and parent links.
The `--cdif-02` command submits positive rows 1 through 301. The
`--cdif-03` command additionally enables Knots BASIC indexing for the
before-and-after checks. The commands use the ordinary
default Regtest rules and submit only the block bytes.

The runner allocates disposable datadirs under the system temporary
directory and removes them after both daemons exit. Point `TMPDIR` at a
writable RAM-backed volume on macOS to speed up the storage syncs during the
extended run.

The command writes a fresh `run-*/receipt.json` under the corresponding ignored
`.local/open-bitcoin-dev/cdif-01/`, `.local/open-bitcoin-dev/cdif-02/` or
`.local/open-bitcoin-dev/cdif-03/` directory after both successful RPC outcomes and clean shutdowns. Log files and build artifacts remain in that run
directory. A failed run exits nonzero and leaves its logs. The committed
receipt pins one observed run. A reproduction can yield a different
executable digest due to toolchain and build-environment differences. The
offline ledger checker checks the tracked receipt and corpus. The command
above performs the actual two-daemon comparison.

## Evidence scope

The evidence class is `knots_daemon_differential` for one positive block in
CDIF-01, the positive chain through height 301 in CDIF-02, and the
bounded rejection and unchanged-state cases in CDIF-03. Cross-daemon UTXO,
work and BASIC comparisons and qualification invalidation have separate
requirement owners. Public networking, whole-chain
validation, service operation, and wallet readiness require separate evidence.

Pinned Knots source anchors include
[`CMakeLists.txt`](../../packages/bitcoin-knots/CMakeLists.txt) for the build,
[`init.cpp`](../../packages/bitcoin-knots/src/init.cpp) for startup options,
[`httprpc.cpp`](../../packages/bitcoin-knots/src/httprpc.cpp) for RPC,
[`rpc/mining.cpp`](../../packages/bitcoin-knots/src/rpc/mining.cpp) for
`submitblock`, and
[`rpc/blockchain.cpp`](../../packages/bitcoin-knots/src/rpc/blockchain.cpp) for
best-block observations. The monetary rules anchor to
[`kernel/chainparams.cpp`](../../packages/bitcoin-knots/src/kernel/chainparams.cpp)
for the Regtest halving interval,
[`consensus/consensus.h`](../../packages/bitcoin-knots/src/consensus/consensus.h)
and [`consensus/tx_verify.cpp`](../../packages/bitcoin-knots/src/consensus/tx_verify.cpp)
for coinbase maturity, and
[`validation.cpp`](../../packages/bitcoin-knots/src/validation.cpp) for the
subsidy schedule and coinbase reward check.
