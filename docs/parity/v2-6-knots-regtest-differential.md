# Pinned Knots Regtest Differential Runner

`v2-6-cdif-01` owns `CDIF-01`. The [tracked receipt](receipts/cdif-01-regtest.json)
records an actual comparison between Knots `29.3.knots20260210` at
`a9aee730466ac67d35a3c03ee24676be5e045878` and ordinary Open Bitcoin
`open-bitcoind`. Both began at canonical Regtest genesis, accepted the same
serialized height-one block through authenticated `submitblock` RPC,
reported best block
`194ad8077ee474b008cc5fb29bade1fd6cda566c44972e2d614f1bf586762d24`
at height 1, and exited cleanly.

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
zero peers. The input is row 1 of the
[canonical Regtest corpus](../../packages/testdata/phase167/regtest-history.json).
The runner verifies its genesis parent and header hash before submitting the
same complete bytes to each process.

The command writes a fresh `run-*/receipt.json` under the ignored
`.local/open-bitcoin-dev/cdif-01/` directory after both successful RPC outcomes
and clean shutdowns. Log files and build artifacts remain in that run
directory. A failed run exits nonzero and leaves its logs. The committed
receipt pins one observed run. A reproduction can yield a different
executable digest due to toolchain and build-environment differences. The
offline ledger checker checks the tracked receipt and corpus. The command
above performs the actual two-daemon comparison.

## Evidence scope

The evidence class is `knots_daemon_differential` for the one-block runner.
`CDIF-02` owns maturity and the height-150 and height-300 halvings.
Subsequent requirements own rejection normalization, UTXO/work/BASIC
comparisons, and qualification invalidation. Public networking, whole-chain
validation, service operation, and wallet readiness require separate evidence.

Pinned Knots source anchors include
[`CMakeLists.txt`](../../packages/bitcoin-knots/CMakeLists.txt) for the build,
[`init.cpp`](../../packages/bitcoin-knots/src/init.cpp) for startup options,
[`httprpc.cpp`](../../packages/bitcoin-knots/src/httprpc.cpp) for RPC,
[`rpc/mining.cpp`](../../packages/bitcoin-knots/src/rpc/mining.cpp) for
`submitblock`, and
[`rpc/blockchain.cpp`](../../packages/bitcoin-knots/src/rpc/blockchain.cpp) for
best-block observations.
