# Canonical Regtest paired pruning

`v2-6-cint-01` supplies `local_product` evidence for `CINT-01`. The
[native reproduction](../../packages/open-bitcoin-node/src/sync/tests/canonical_pruning.rs)
opens a disposable Fjall store at proved default Regtest genesis with BASIC
enabled. It admits and connects the 301 descendants from the
[canonical corpus](../../packages/testdata/phase167/regtest-history.json),
then mines and connects deterministic descendants through height 1001. Each
descendant earns durable `ScriptsValid` provenance. A normal managed flush
persists undo and coins before pruning.

The [pinned Regtest chain parameters](../../packages/bitcoin-knots/src/kernel/chainparams.cpp)
set the default pruning threshold at height 1000. The
[manual RPC threshold](../../packages/bitcoin-knots/src/rpc/blockchain.cpp)
allows pruning at that height, and the
[keep-window constant](../../packages/bitcoin-knots/src/validation.h)
protects the trailing 288 heights. The test runs above the threshold and
checks refusal below it and inside the keep window. Knots applies its
[index locks](../../packages/bitcoin-knots/src/index/base.cpp) to
[whole block and undo file pairs](../../packages/bitcoin-knots/src/node/blockstorage.cpp).
Open Bitcoin applies its owned BASIC protection to Fjall block and undo keys.

| Observed operation | Result |
| --- | --- |
| Apply a plan for heights 1 through 713 while BASIC is behind | Zero deletions. Durable BASIC protection keeps all candidate pairs. |
| Drive bounded BASIC turns until the accepted tip is indexed, then apply the same plan | Both keys are deleted for each of the 713 heights. Heights 714 through 1001 retain both keys. |
| Measure logical block and undo bytes | 1,357,686 before deletion and 390,339 afterward. The drop equals the measured size of the deleted pairs. |
| Flush again with the same runtime | Deleted keys stay absent. The prune commit leaves the durable coins tip and saved BASIC checkpoint intact. |

Genesis is outside the requested descendant pairs. This reproduction uses
the native managed-runtime admission and manual prune planner and flush.
The ordinary daemon submission and complete-history reopen bound remains 306
positions. The byte counts measure logical keys. Automatic size-triggered
pruning and Knots daemon pruning require separate evidence. Post-prune reopen
and BASIC client service are tracked by `CINT-02`.

## Reproduce

Run from the repository root with the repo-pinned Bun and Rust toolchains.
The test uses the system temporary directory. A RAM-backed `TMPDIR` speeds
up the durable writes on macOS.

```sh
git submodule update --init --recursive
bun run scripts/command-timings.ts run --key cint-01-canonical-regtest-pruning -- \
  cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib \
  canonical_regtest_pruning_deletes_pairs_after_basic_catch_up -- --nocapture
bun run scripts/check-parity-ledger.ts
```

## Retained BASIC clients after reopen

`v2-6-cint-02` supplies `local_product` evidence for `CINT-02`. The
[client test](../../packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/cint02.rs)
builds two independently validated instances of the same default Regtest
history. A stronger branch replaces heights 999–1001 and brings the active
tip to height 1002. The active and displaced height-1000 blocks have
different BASIC checkpoint headers. Manual pruning deletes both payload
records at heights 1–714. Each database is fully closed before reopen.

The [archive-backed opener](../../packages/open-bitcoin-node/src/storage/fjall_store/pruned_regtest.rs)
takes separate paths to the original closed pruned source and a complete
unpruned archive of its accepted history, including the displaced branch.
Isolated workers inspect detached copies. Replay from genesis independently
rebuilds coins, exact work, undo, and BASIC. Before opening the original
backend, the proof compares chain and index authority rows, allowing eligible
paired deletion and prune bookkeeping. Proven rows and a mutation stamp
bind authority to the original store through runtime construction. A
correct archive supplies validation inputs during startup. The service
test removes that archive before any client makes a request.

| Client | Observed after reopen |
| --- | --- |
| Authenticated `getblockfilter` | Exact filters and headers for pruned height 714, retained height 715, and both height-1000 blocks. A request without credentials returns HTTP 401. |
| Authenticated `getindexinfo` | BASIC is synchronized at height 1002. |
| Inbound and outbound TCP peers | Five BASIC frames per direction. `cfilter` and `cfheaders` cross the prune boundary, and `cfcheckpt` returns each branch’s own checkpoint. |
| Inbound block requests | A permissioned peer receives block 715 and `notfound` for block 714 on the same session. |
| Wallet RPC | `getwalletinfo` and `rescanblockchain` refuse with the canonical wallet-unavailable error even after the filter queries succeed. |

The [RPC context](../../packages/open-bitcoin-rpc/src/context/canonical_history.rs)
accepts explicit Regtest manual pruning and numeric loopback service. The
outbound session accepts handshake and BASIC requests and refuses an
unsolicited header without advancing the chain. The proof envelope permits
up to 1024 replay positions and 8192 logical inspection entries. This
exercise uses the authenticated HTTP handler and physical TCP peer adapters.
It covers the archive-backed managed startup API and its bounded service
route. CLI configuration, automatic pruning, and Knots daemon pruning are
outside this evidence.

```sh
bun run scripts/command-timings.ts run --key cint-02-client-service -- \
  cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind \
  cint02_authenticated_rpc_and_both_peer_directions_after_canonical_pruning -- --nocapture
```

## Canonical operator status

`v2-6-cint-03` supplies `local_product` evidence for `CINT-03`. The
[operator test](../../packages/open-bitcoin-cli/tests/operator_flows/basic_index/canonical_status.rs)
runs the repo-local CLI against a loopback listener dispatching RPC on the
same archive-backed pruned Regtest owner. The saved runtime metadata still
describes the pre-reorg tip. `getblockchaininfo` reports the replacement tip
at height 1002 and its exact cumulative work, `0x7d6`, as a 64-digit
`chainwork` field. The CLI copies a well-formed work value paired with its
reported block hash into the shared status model.

`openbitcoinnetworkstatus` supplies BASIC progress and live pruning facts.
The test requests `pruneblockchain 715` and observes the keep-window refusal.
CLI `status`, the dashboard, and the support bundle agree on the tip, work,
BASIC progress, prune height 715, 714 removed pairs, and `keep_window`.
The support bundle retains those facts and clears private prune-lock names
from its JSON and Markdown. The measurements use a stable local snapshot
of the bounded Regtest role.

```sh
bun run scripts/command-timings.ts run --key cint-03-operator-status -- \
  cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --test operator_flows \
  cint03_operator_exports_observe_the_same_canonical_tip_and_refusal -- --nocapture
```
