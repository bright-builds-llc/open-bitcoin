# Open Bitcoin CLI

This crate supplies the baseline RPC client `open-bitcoin-cli` and the
Open Bitcoin operator binary `open-bitcoin`. The operator binary provides status,
sync controls, configuration inspection and other workflows documented in the
[runtime guide](../../docs/operator/runtime-guide.md).

## Work diagnostics and current support

Live validated work renders as exactly 64 lowercase hexadecimal digits, preserving
all 256 bits. Header work and downloaded height do not prove connected history.
Unknown fields remain unavailable; a number or decoded snapshot grants no
validation lineage. See [exact work and earned authority](../../docs/parity/v2-6-shared-exact-work.md).

## Current Phase 166 canonical genesis boundary

Canonical genesis bootstrap and reopen are supported for all four selectors.

The local genesis route proves the complete pinned header/body, height-zero
header/block/coins best block and all 32 bytes of internally derived work.
Genesis coinbase outputs stay absent from UTXOs. Explicit BASIC indexing uses
the genesis transaction's output script and the zero predecessor commitment.
The selector `testnet` means testnet3; Signet and regtest use their default profiles.
The genesis RPC context has no wallet authority: wallet operations refuse.

Generic ordinary-history opening remains refused. Phase 167 adds only bounded
default-Regtest history through a separate complete replay proof; Mainnet,
testnet3 and Signet descendants remain outside this route. Signet challenge
validation is unavailable. Networking remains default off, and a selected
network or local implementation fact never authorizes a generic store opener.
Actual pinned Knots-daemon differential qualification remains deferred to Phase 168.
Local Cargo/Bazel process evidence concerns Open Bitcoin's own products.

Use the [bounded genesis evidence and exact operator commands](../../docs/parity/v2-6-canonical-durable-bootstrap.md).
Daemon startup requires an existing explicit datadir and an existing durable
trigger such as `-blockfilterindex=1`; a missing configured datadir still fails
the parser. After Ctrl-C/SIGINT shutdown, offline `sync status` reopens existing
proved genesis or complete default-Regtest history and preserves the saved BASIC mode. It creates no saved
status/progress record. An admitted Fjall open can have filesystem effects;
physical preservation is established by tested refusal/pre-admission controls.

Cooperating application leases cover the supported Unix route; actual process
evidence is macOS. Finite private-copy inspection, child completion and cleanup
limits are documented with the evidence. No repair, conversion, import or
automatic history acquisition is offered. Whole-chain parity, public defaults,
production service and production-funds readiness remain outside the claim.

## Reproduce offline genesis status and missing-storage refusal

Run the [complete disposable genesis/BASIC startup procedure](../../docs/parity/v2-6-canonical-durable-bootstrap.md#operator-reproduction)
first. The same shell retains `genesis_review_dir`; its configuration file exists
and its proved store is closed before either command below opens it.

```bash
bun run scripts/command-timings.ts run --key phase166-docs-cargo-status -- cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --config "$genesis_review_dir/open-bitcoin.jsonc" --datadir "$genesis_review_dir/data" --network regtest --format json sync status
bun run scripts/command-timings.ts run --key phase166-docs-bazel-status -- bazel run //packages/open-bitcoin-cli:open_bitcoin -- --config "$genesis_review_dir/open-bitcoin.jsonc" --datadir "$genesis_review_dir/data" --network regtest --format json sync status
```

Expected facts are Regtest, validated height 0, exact work ending in `0002`, BASIC
configured true, safe durable height 0 and one retained record. Sync-status hash
strings use raw byte order; `getblockchaininfo.bestblockhash` uses display order
and roundtrips into `getblockfilter` through the existing RPC normalizer.

Use the same valid config with a missing storage path as a separate negative
control. Each command intentionally returns nonzero compatibility refusal and
leaves that path absent; it is not a repair workflow.

```bash
bun run scripts/command-timings.ts run --key phase166-docs-cargo-missing -- cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --config "$genesis_review_dir/open-bitcoin.jsonc" --datadir "$genesis_review_dir/missing" --network regtest --format json sync status
bun run scripts/command-timings.ts run --key phase166-docs-bazel-missing -- bazel run //packages/open-bitcoin-cli:open_bitcoin -- --config "$genesis_review_dir/open-bitcoin.jsonc" --datadir "$genesis_review_dir/missing" --network regtest --format json sync status
```

Unproved/unsafe sources retain fixed compatibility diagnostics and unavailable
observed work. Missing/empty sources do not initialize through offline status.
Successful existing-only observation saves no synthesized progress, but an
admitted Fjall open is not physically read-only. Managed-wallet adapters and sync
pause/resume keep ordinary-history gates. Use `bash scripts/verify.sh` for the
native contract; final verification/LOC/hooks remain Plan20/root-owned.

## Bounded default-Regtest descendant status

On a closed, physically prepared Phase 167 default-Regtest datadir with 301
accepted descendants, offline `sync status` consumes the complete copied and
original history proof. It reports the actual accepted height, raw-order tip
hash, full work, and saved BASIC endpoint. The `support` object separately
describes observed proof class, selected identity, bounded local
implementation, and unavailable
pinned Knots-daemon differential qualification. Other selectors retain their
proved-genesis route; a mismatched or missing history source refuses.

Set `phase167_dir` to the absolute path of a disposable prepared directory
containing the existing `open-bitcoin.jsonc` and closed `data` store. These
commands do not create or acquire history:

```bash
phase167_dir=/absolute/path/to/prepared-phase167-regtest
test -f "$phase167_dir/open-bitcoin.jsonc"
test -d "$phase167_dir/data"
bun run scripts/command-timings.ts run --key phase167-docs-cargo-status -- cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --config "$phase167_dir/open-bitcoin.jsonc" --datadir "$phase167_dir/data" --network regtest --format json sync status
bun run scripts/command-timings.ts run --key phase167-docs-bazel-status -- bazel run //packages/open-bitcoin-cli:open_bitcoin -- --config "$phase167_dir/open-bitcoin.jsonc" --datadir "$phase167_dir/data" --network regtest --format json sync status
```

For a correctly proved 301-descendant fixture, expect accepted height 301,
full work `000000000000000000000000000000000000000000000000000000000000025c`,
and a safe BASIC height of 301. Confirm these values in the actual status
output for the prepared store. These fields describe local Open Bitcoin
behavior; no actual pinned-daemon qualification, automatic sync activation, wallet
availability, migration, or repair is implied.
