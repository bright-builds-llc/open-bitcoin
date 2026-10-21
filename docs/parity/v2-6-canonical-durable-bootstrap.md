# Canonical Durable Genesis and BASIC

Surface id: `v2-6-canonical-durable-bootstrap`. This companion records CBOT-02,
CBOT-03 and CWOR-04 implementation evidence. Final phase verification and
requirement/lifecycle closure remain separate gates owned by Plan14/root.
The [Phase164 identity/refusal record](v2-6-canonical-chain-identity.md) and
[Phase165 work record](v2-6-shared-exact-work.md) retain their historical receipts.

## Current Phase 166 canonical genesis boundary

Canonical genesis bootstrap and reopen are supported for all four selectors.

The selectors are `mainnet`, `testnet` (testnet3), `signet` (default challenge) and
`regtest` (default profile). Complete pinned header/body bytes determine genesis
identity. A marker, selected name, decoded work value, raw store or fixture alone
cannot confer lineage. Ordinary history remains refused pending Phase167;
Signet challenge validation is still missing. Public/default network activation
remains refused. Actual pinned Knots-daemon differential qualification remains
deferred to Phase168. The local product evidence below concerns Open Bitcoin.

## Pinned source and proved state

Bitcoin Knots `29.3.knots20260210`, commit
`a9aee730466ac67d35a3c03ee24676be5e045878`, supplies the source anchors:

| Source | Bounded behavior |
| --- | --- |
| [kernel/chainparams.cpp](../../packages/bitcoin-knots/src/kernel/chainparams.cpp) | Full four-network genesis header and transaction bodies |
| [validation.cpp](../../packages/bitcoin-knots/src/validation.cpp) | Genesis activation and coins best-block handling |
| [chain.cpp](../../packages/bitcoin-knots/src/chain.cpp), [arith_uint256.h](../../packages/bitcoin-knots/src/arith_uint256.h) | Exact target proof and modulo-2^256 cumulative arithmetic |
| [blockfilter.cpp](../../packages/bitcoin-knots/src/blockfilter.cpp), [index/blockfilterindex.cpp](../../packages/bitcoin-knots/src/index/blockfilterindex.cpp) | Genesis output-script BASIC input and zero predecessor |

The frozen [genesis/filter vectors](fixtures/phase166-genesis-vectors.json)
contain complete serialized headers/bodies, hashes, all 32 proof bytes, encoded
filters and filter commitments. [Independent source-derived generation](../../scripts/phase166-genesis-vectors.ts)
uses the pinned Python SipHash/message helpers with independently expressed
proof/filter formulas. These numerical/source-derived bytes are expectations,
not an executed Knots-daemon comparison or storage capability.

At height zero the accepted best header, active block, metadata tip and coins
best block `B` all bind the same genesis. No coins `C` rows or incomplete-batch
`H` marker remain. The genesis coinbase output is absent from UTXOs through its
special activation rule, not maturity or an unspendable-script shortcut. Its
serialized output script remains a BASIC input. Explicit BASIC enablement
publishes one genuine genesis record with zero predecessor, exact filter/hash/
header and durable height zero. Disabled mode does not initialize the index;
unspecified mode preserves a proved saved mode.

The sealed opener is consumed by the canonical runtime/RPC factory. Exact work
is derived again from the complete stored header ancestry and proof rules,
then compared across header/index/metadata/coins bindings before authority.
All-handle release includes runtime, RPC context, network/chainstate, store
clones and coins/read/index owners. Same-path independent reopen follows release;
a retained clone or decoded numerical snapshot is not reopen proof.

## Refusal, inspection and recovery limits

The compatible-genesis route differs from Knots' ordinary startup scope: it
requires the bounded complete genesis record set and refuses ordinary
descendants, legacy/synthetic/foreign/forged state, missing bindings and unsafe
paths. It provides no automatic import, conversion, repair or destructive retry.
Incompatible-source refusal preserves physical source bytes, file kinds, Unix
modes, mtimes and link targets in the tested pre-admission controls; atime is
excluded. Bounded diagnostics disclose fixed categories, not private payloads
or backend paths. Successful admitted Fjall opening can recover or write engine
state, so neither compatible reopen nor offline status is physically read-only.

The cooperating application lease covers supported Unix (macOS/Linux), in
addition to the existing engine lock. Actual process receipts here are macOS.
The lease spans inspection and live backend teardown. This does not protect
against arbitrary direct-Fjall/filesystem writers or hostile lock replacement.
Refusal on unsupported platforms remains explicit.

Private-copy inspection limits are 4,096 entries, 512 MiB cumulative bytes,
directory depth 16 and 4,096 encoded path bytes. Individual authority records are
bounded at 64 KiB. A same-executable child recovers only the private copy;
its result is bounded and non-authorizing until parent validation. The worker's
30-second deadline includes job delivery/result observation, not the entire
inventory/copy/cleanup operation or a hardware crash guarantee. Parent admission
requires confirmed child exit, source recheck and scratch cleanup. When child
termination/reaping is uncertain, admission refuses and scratch is retained
rather than removed under a possibly live process. Live-parent cleanup controls
do not promise cleanup after parent crash, power loss or arbitrary OS failure.

The managed checkpoint preserves already-stored confirmed-transaction counts
only for exactly unchanged full ancestry. Missing/legacy counts stay absent;
changed ancestry discards them, malformed records refuse, and raw public metadata
writes still invalidate authority. Coins-before-metadata failure semantics remain;
there is no whole-flush atomicity or hardware power-loss claim.

## Operator reproduction

From the repository root with pinned tools available, create only disposable
paths. There is no standalone bootstrap subcommand.

```bash
genesis_review_dir=$(mktemp -d /tmp/open-bitcoin-genesis-review.XXXXXX) || exit 1
genesis_review_dir=$(cd -- "$genesis_review_dir" && pwd -P) || exit 1
mkdir "$genesis_review_dir/data" || exit 1
printf '{}\n' > "$genesis_review_dir/open-bitcoin.jsonc" || exit 1
```

An explicit existing datadir and durable trigger are required for actual daemon
startup. Use one build route at a time with disposable loopback-only credentials
and a free local RPC port (18443 below):

```bash
bun run scripts/command-timings.ts run --key phase166-docs-cargo-daemon -- cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind -- -datadir="$genesis_review_dir/data" -chain=regtest -server=1 -rpcbind=127.0.0.1 -rpcport=18443 -rpcuser=preview -rpcpassword=preview -blockfilterindex=1
# Alternative, after the first process has stopped:
bun run scripts/command-timings.ts run --key phase166-docs-bazel-daemon -- bazel run //packages/open-bitcoin-rpc:open_bitcoind -- -datadir="$genesis_review_dir/data" -chain=regtest -server=1 -rpcbind=127.0.0.1 -rpcport=18443 -rpcuser=preview -rpcpassword=preview -blockfilterindex=1
```

Ctrl-C/SIGINT is the existing graceful shutdown contract. Wait for full daemon
exit before offline commands. Repeating startup on that path exercises product
reopen; do not run a concurrent operator open. An absent datadir configuration or
an explicitly nonexistent daemon datadir still refuses; the adapter API's ability
to create missing storage does not relax the actual config parser. A no-trigger
daemon remains storage-free and refuses unsupported history.

After shutdown, run either actual operator form serially:

```bash
bun run scripts/command-timings.ts run --key phase166-docs-cargo-status -- cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --config "$genesis_review_dir/open-bitcoin.jsonc" --datadir "$genesis_review_dir/data" --network regtest --format json sync status
bun run scripts/command-timings.ts run --key phase166-docs-bazel-status -- bazel run //packages/open-bitcoin-cli:open_bitcoin -- --config "$genesis_review_dir/open-bitcoin.jsonc" --datadir "$genesis_review_dir/data" --network regtest --format json sync status
```

Expect `selected_network=regtest`, validated active-chain height 0, work
`0000000000000000000000000000000000000000000000000000000000000002`, BASIC
configured true, safe durable height 0 and one retained record. Offline status
only reopens existing proved genesis, preserves saved BASIC and does not save
synthesized status/progress. An admitted backend open can have filesystem effects.
Sync-status hash strings use raw byte order; RPC `bestblockhash` uses display
order and roundtrips to `getblockfilter` through the existing normalizer.
These are offline storage commands. Separately authenticated live RPC queries
the running daemon's existing context and does not open a second local store.

The same valid config with missing storage must return nonzero compatibility
refusal and leave that path absent:

```bash
bun run scripts/command-timings.ts run --key phase166-docs-cargo-missing -- cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --config "$genesis_review_dir/open-bitcoin.jsonc" --datadir "$genesis_review_dir/missing" --network regtest --format json sync status
bun run scripts/command-timings.ts run --key phase166-docs-bazel-missing -- bazel run //packages/open-bitcoin-cli:open_bitcoin -- --config "$genesis_review_dir/open-bitcoin.jsonc" --datadir "$genesis_review_dir/missing" --network regtest --format json sync status
```

The genesis RPC context carries `WalletState::Unavailable`; wallet queries and
mutations refuse before wallet/registry effects. Offline wallet adapters and
`sync pause|resume` retain ordinary gates. Sync, inbound, transaction relay,
block serving and compact relay activation refuse before store opening. BASIC
indexing alone grants no serving, peer or wallet permission.

## Evidence classes and gate ownership

| Evidence | Actual scope and source |
| --- | --- |
| Frozen bytes/formulas | [Plan01](../../.planning/phases/166-canonical-durable-bootstrap-and-genesis-basic/166-01-SUMMARY.md); source-derived four-network genesis/filter and numerical expectations |
| Durable bootstrap/reopen and software faults | [Plan10](../../.planning/phases/166-canonical-durable-bootstrap-and-genesis-basic/166-10-SUMMARY.md); actual-byte binding corruption, checkpoint/count preservation, failures and all-handle release |
| Child isolation and cleanup controls | [Plan15](../../.planning/phases/166-canonical-durable-bootstrap-and-genesis-basic/166-15-SUMMARY.md); actual private-copy worker and bounded live-parent lifecycle |
| Canonical runtime/RPC and unavailable wallet | [Plan16](../../.planning/phases/166-canonical-durable-bootstrap-and-genesis-basic/166-16-SUMMARY.md); sealed consumption, four-selector observations, physical versus logical refusal boundaries |
| Local Open Bitcoin products | [Plan09](../../.planning/phases/166-canonical-durable-bootstrap-and-genesis-basic/166-09-SUMMARY.md); actual Cargo/Bazel Regtest fresh/reopen, authenticated RPC/BASIC roundtrip, SIGINT exit0 and operator status; Rust process matrix covers all four selectors |
| Documentation command reproduction | Plan13 ignored receipts under `.local/open-bitcoin-dev/phase166/plan13/`; exact Cargo/Bazel existing-only status and missing-storage controls with a provided valid JSONC config |

Original failed/scaffold/interrupted receipts retain their actual outcomes.
Compile/setup errors are separate from behavioral failures. Source guards and
arithmetic above-u128 tests cannot establish durable history or daemon parity;
no above-u128 genesis history is asserted. Actual pinned Knots-daemon differential
qualification remains Phase168-owned. Ordinary descendants, fork recovery and
canonical pruning remain later phase work. Whole-chain, public-default,
production-service and funds-readiness claims remain outside this evidence.

Full default `bash scripts/verify.sh`, generated
`docs/metrics/lines-of-code.md` freshness, independent source/security/goal
verification, lifecycle closure and normal-hook Git saving are Plan14/root
responsibilities. Consult the phase verification and acceptance records for
their recorded outcomes; the focused receipts above retain their cited scope.
