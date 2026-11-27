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

## Best block, chainwork, UTXOs and BASIC

`v2-6-cdif-04` compares observed state on the same accepted chain through
height 301. Both daemons supply the live active tip and all 302 BASIC
filter and header pairs through RPC, including the genesis filter.

Knots supplies exact chainwork through `getblockchaininfo` and the complete
UTXO set through its ASCII `dumptxoutset` RPC at the active tip. Open Bitcoin
supplies exact chainwork and all coins after its daemon exits and a
[test-only observer](../../packages/open-bitcoin-rpc/tests/cdif04_closed_state.rs)
reopens the same datadir through the ordinary proved canonical-history path.
The observer checks the closed tip against the live tip. This is an offline
evidence reader.

The [state receipt](receipts/cdif-04-regtest.json) records exact work,
the complete sorted UTXO projection digest, and the height-ordered BASIC
projection digest for each process. The common coin fields are display-order
txid, output index, integer satoshis, raw script bytes, coinbase status,
and creation height. The runner checks both observations against the
[independent corpus expectations](fixtures/phase167-regtest-expectations.json).
The raw projections remain in the ignored run directory for diagnosis.

Knots and Open Bitcoin use different storage formats and observation
interfaces. Open Bitcoin's creating-parent median time is store metadata
outside the common coin projection. The comparison treats a difference in a
common field as a mismatch.

## Case-level qualification

`v2-6-cdif-05` runs both of the bounded Regtest histories from fresh
datadirs on each daemon. The [qualification receipt](receipts/cdif-05-regtest.json)
records a hash for every successful submit and active-tip observation.
Within each isolated run, the position in `accepted` is the block height
minus one. The two runs accept 201 and 301 blocks per daemon. Each also
records its named rejections, serialized input digests, raw RPC outcomes,
and before-and-after active state. The height-301 run includes the complete
final coin and BASIC projections and exact work.

Each passing receipt binds the pinned Knots source and build, both daemon
binaries, the Open Bitcoin product commit, the Regtest corpus, the independent
expectations, and the offline observer. The runner validates the exact case
set, expected outcomes and clean exits before publishing. Receipt validation
is also available through the offline parity ledger checker.

Each build command has a 20-minute deadline, each daemon run has 15 minutes,
and the offline state observer has three minutes. RPC requests have a
five-second deadline. Startup and shutdown have separate 30-second bounds.
The runner stops its owned subprocesses on failure and keeps a `failure.json`
with the stage and outcome in the per-run evidence directory. Missing or
incorrect oracle, timeout, incomplete execution, or a mismatch leaves the
run without a passing receipt. Tests exercise all four failure classes.

## Branch order and restart ties

`v2-6-crec-01` compares valid branches sharing canonical genesis and a common
height-one block. The runner deterministically generates and mines sibling
blocks under the existing Regtest target. Two mirrored arrival schedules
observe the active tip after every submission. The first eligible branch
remains active at equal work. An extension of the other branch switches the
tip when its work becomes strictly greater. Later equal arrivals still
preserve the incumbent, and a stronger extension can switch back.

Both daemons retain the previously active tip across a clean restart when
the competing branches have equal work. Knots reloads the persisted active
chain with sequence ID zero and the other disk candidates with sequence ID
one. The runner then uses a Knots-only invalidation probe in its disposable
datadir to exclude that active branch. Either of the two remaining equal-work,
equal-sequence candidates is a valid outcome for the pointer-address fallback.
The receipt records the observed winner and the allowed set. The Open Bitcoin
restart observation covers its ordinary submission surface.

Open Bitcoin derives the replacement from held headers and exact retained
bodies. It stages the coins, undo and BASIC replacements, checks prospective
rows and then compares the achieved checkpoint before returning acceptance.
Equal-work side bodies remain durable with KnownOnly validation status
until the branch actually activates.

The [branch receipt](receipts/crec-01-regtest.json) records the generated
branch input digest, exact RPC results, observed tips and all process exits.
Pinned anchors include
[the comparator](../../packages/bitcoin-knots/src/node/blockstorage.cpp),
[disk sequence defaults](../../packages/bitcoin-knots/src/chain.h),
[body link and restart rules](../../packages/bitcoin-knots/src/validation.cpp),
and [submitblock](../../packages/bitcoin-knots/src/rpc/mining.cpp).

## Replacement spends and reconnect

`v2-6-crec-02` builds both branches on the canonical positive prefix
through height 100. Each branch spends the same height-one coinbase output
at maturity, with a distinct spending transaction and resulting output.
A stronger candidate on the side branch attempts to spend that input a
second time. The runner then submits a valid stronger replacement, followed
by an equal-work extension and then a stronger extension of the original
branch to exercise reconnect.

The [spend receipt](receipts/crec-02-regtest.json) compares four checkpoints
for each actual daemon. The original and post-rejection checkpoints must
agree on active tip, exact work, all UTXOs, and the tip's BASIC filter and
header. The later checkpoints must match complete coin sets independently
derived from the replacement and reconnected branches. Each process exits
cleanly between checkpoints.

Knots uses its active-tip UTXO dump and chainwork RPC. Open Bitcoin uses a
[test observer](../../packages/open-bitcoin-rpc/tests/crec02_closed_state.rs)
that reopens the same closed datadir through the proved canonical-history
path. Both report live tip and BASIC data, and the comparison uses
consensus UTXOs. Mempool admission and resurrection behavior have distinct
evidence.

## Canonical stop and reopen

`v2-6-crec-03` follows the ordinary Open Bitcoin daemon and operator CLI on
the bounded default-Regtest replacement branch through height 102. The
runner exercises `sync status` against the live owner's datadir and records
the compatibility refusal. After the daemon exits, the source-built CLI
reports the earned tip, exact work, and BASIC fence from that same datadir.
The existing closed-state observer independently checks all consensus
UTXOs through the proved canonical-history reopen.

A new daemon process reopens the same datadir, reports the same tip and
BASIC commitment, and recognizes the last accepted block as a duplicate.
After its clean stop, both offline observations must again match. A final
restart accepts a height-103 spend of the recovered replacement output.
The final offline observation checks the resulting coins and exact work.
The [reopen receipt](receipts/crec-03-regtest.json) binds the clean exits,
CLI and daemon builds, canonical inputs, and offline observer. Its evidence
class is `local_product`.

## Interrupted checkpoint and inconsistent records

`v2-6-crec-04` uses disposable processes that earn a canonical Regtest
checkpoint through the ordinary submission command. At existing injected
software fault points, a fixture process exits with the runtime handles
still in scope so ordinary reopen must use the committed store. The
[checkpoint receipt](receipts/crec-04-regtest.json) records these outcomes.

| Case | Ordinary reopen |
| --- | --- |
| Interrupted before candidate body retention | Proves the earlier height-one prefix |
| Interrupted before chain metadata publication | Explicit compatibility refusal with source bytes preserved |
| Interrupted after the BASIC checkpoint commit | Proves the height-two prefix |
| Closed coins tip inconsistent with chain metadata | Explicit compatibility refusal with source bytes preserved |
| Closed header and index records with inconsistent work | Explicit compatibility refusal with source bytes preserved |

On proved prefixes, the managed runtime checks complete coins and the BASIC
checkpoint. The source-built `sync status` process independently reports the
same tip and exact work, or the structured refusal. Both refusal paths check the
original source file inventory and byte digests. This is `local_product`
evidence for the bounded software interruption and inconsistent-record
fixtures, with the test source, corpus and CLI build bound in the receipt.

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
bun run scripts/run-knots-regtest-differential.ts --cdif-04
bun run scripts/run-knots-regtest-differential.ts --cdif-05
TMPDIR=/tmp bun run scripts/run-knots-regtest-differential.ts --crec-01
bun run scripts/run-knots-regtest-differential.ts --crec-02
bun run scripts/run-knots-regtest-differential.ts --crec-03
TMPDIR=/tmp bun run scripts/run-knots-regtest-differential.ts --crec-04
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
`--cdif-03` through `--cdif-05` and `--crec-02` commands additionally
enable Knots BASIC indexing for the observed-state checks. The commands use the ordinary
default Regtest rules and submit only the block bytes.

The runner allocates disposable datadirs under the system temporary
directory and removes them after both daemons exit. Point `TMPDIR` at a
writable RAM-backed volume on macOS to speed up the storage syncs during the
extended run.

The command writes a fresh `run-*/receipt.json` under the corresponding ignored
`.local/open-bitcoin-dev/cdif-01/` through
`.local/open-bitcoin-dev/cdif-05/`, or `.local/open-bitcoin-dev/crec-01/` through
`.local/open-bitcoin-dev/crec-04/` directories after both successful RPC outcomes and clean shutdowns. Log files and build artifacts remain in that run
directory. A failed run exits nonzero and leaves its logs. The committed
receipt pins one observed run. A reproduction can yield a different
executable digest due to toolchain and build-environment differences. The
offline ledger checker checks the tracked receipt and corpus. The command
above performs the actual two-daemon comparison.

## Qualification freshness

Each of the seven actual comparison commands captures its versioned inputs
before it builds either daemon. The input digest covers the workspace and
toolchain files, the Knots gitlink, the runner and normalization code, and
the checked-in state expectations. The runner rechecks those inputs before
publication and binds the digest to the recorded Open Bitcoin build commit.
All seven comparisons share the input set. Changes to those inputs require
renewed runs for the set. Documentation and receipt publication are outside
that input set, so their commits can preserve qualification.

```sh
bun run scripts/check-v2-6-qualification.ts --status
bun run scripts/check-v2-6-qualification.ts --check
```

`--status` reports the receipts that need renewal. `--check` also validates
the recorded outcomes through the parity ledger and fails on a missing,
invalid, or stale input binding. The default repository verifier runs the
read-only check. The actual commands in [Reproduce](#reproduce) separately
build and launch the pinned daemon and Open Bitcoin. Their receipts bind the
versioned inputs as well as their observed executable hashes and build facts.
The qualification scope is isolated, bounded Regtest execution.

For the v2.6 tag, all seven actual comparisons were renewed from Open Bitcoin build commit
`b346a29bca16ba25ad1ae4f1fd93646e60b5cf92`. The tracked receipts carry the
same input SHA-256, `cbffff9a9b5a9984613445a248a85fe9aeee5481da0bd33b456b12ce781eff91`.
Extending the [offline Mainnet prefix](mainnet-offline-prefix.md) to the first
retarget changed shared qualification inputs. All seven Regtest comparisons passed again from
Open Bitcoin build commit `a181679e105e4abc81a00980511aa83990f42270`.
The current Mainnet and Regtest receipts share input SHA-256
`9ab1eb04ba6d0e6bc31c2f3aba9410ef9496c21fbcc584747a5aae23f45849a5`.

The focused tests cover runtime, runner, corpus and oracle-build changes,
including checkout edits during a run and a mismatched build commit. The
ledger's qualification scope rejects whole-chain, public-default and
production-readiness claims.

## Evidence scope

The evidence class is `knots_daemon_differential` for one positive block in
CDIF-01, the positive chain through height 301 in CDIF-02, and the
bounded rejection and unchanged-state cases in CDIF-03, and the shared
chain-state projection in CDIF-04. CDIF-05 covers the integrated case matrix
and its failure handling. Qualification invalidation has a separate
requirement owner. Public networking, whole-chain
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
