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
