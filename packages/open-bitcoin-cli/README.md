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

Ordinary history is currently refused for all four selectors: `mainnet`, `testnet`
(testnet3), `signet` (default challenge) and `regtest` (default profile). The
[support/refusal matrix](../../docs/parity/v2-6-canonical-chain-identity.md)
retains identity-only scope. Offline sync controls refuse with
`chain_compatibility_refused`; fixed guidance is followed by `work_decision`, and
observed work stays unavailable. Missing/empty paths report
`bootstrap_unavailable`, existing unproved paths `existing_unproved`, unsafe paths
`unsafe_path`, and inspection I/O failures `inspection_io_failure`.

Preserve existing data for separate review. A separate empty datadir is not a
workaround; no automatic repair or reencoding is offered. Networking remains
default off. Phase 166 retains canonical bootstrap/exact all-handle reopen/genesis
BASIC, Phase 167 contextual completeness and Phase 168 actual pinned-Knots
qualification. Actual local Open Bitcoin startup refusal evidence does not
establish canonical acceptance, whole-chain or production/funds readiness.

## Reproduce a bounded offline refusal

From the repository root, create disposable paths in the same shell:

```bash
cli_review_dir=$(mktemp -d /tmp/open-bitcoin-cli-review.XXXXXX)
```

Run either form serially. Each sync command intentionally returns a nonzero
compatibility refusal; do not interpret it as successful sync or repeat it
against valuable data as a repair procedure.

```bash
bun run scripts/command-timings.ts run --key phase165-cli-cargo-refusal -- cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --config "$cli_review_dir/absent.jsonc" --datadir "$cli_review_dir/missing" --network testnet --format json sync status
bun run scripts/command-timings.ts run --key phase165-cli-bazel-refusal -- bazel run //packages/open-bitcoin-cli:open_bitcoin -- --config "$cli_review_dir/absent.jsonc" --datadir "$cli_review_dir/missing" --network testnet --format json sync status
```

Use `bash scripts/verify.sh` for the repository-native verification contract.
