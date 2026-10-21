# Open Bitcoin Node

This crate owns effectful node orchestration, durable storage, network/runtime
coordination, sync, status and BASIC index lifecycle around the pure core crates.
Use the repository's [architecture](../../.planning/ARCHITECTURE.md) and
[operator runtime guide](../../docs/operator/runtime-guide.md) for ownership and
workflow details.

## Current work and support boundary

[Shared exact work](../../docs/parity/v2-6-shared-exact-work.md) uses the existing
256-bit `ProofWork`. Accepted headers derive cumulative target proof; strict
greater work replaces the best header and equal work keeps its first accepted
winner. Connected candidates additionally require checked body/ancestor linkage
and current eligibility. Known insertion-FIFO body order differs from header
arrival; imported unknown order cannot win an equal-work activation.
Private owner/tip/generation-bound stages and revocable eligibility checks precede
managed mutation; a numerical carrier or copied snapshot cannot authorize it.

Explicit fixture callers can use
[from_live_fixture_source](src/sync/live_fixture.rs) with the same genuine,
flushed managed Fjall source and complete selected parameters. The constructor
independently validates all retained prefix bodies and exact physical identity;
it refuses every canonical root. After all handles close, raw observations stay
unearned. An explicit complete-prefix Memory validation and strict consuming
same-store rebind may establish a new genuine fixture owner without repairing
missing inputs or writing physical None counts. Checked historical nonroot body
order stays UNKNOWN; later real events do not recreate old arrival order.
[Public normal100 proof](tests/phase165_live_fixture/requalification.rs) and
[consumer evidence](../../.planning/phases/165-shared-exact-work-authority/165-31-SUMMARY.md)
record the bounded path. This is not automatic raw reopen or canonical recovery.

## Current Phase 166 canonical genesis boundary

Canonical genesis bootstrap and reopen are supported for all four selectors.

The local genesis route proves the complete pinned header/body, height-zero
header/block/coins best block and all 32 bytes of internally derived work.
Genesis coinbase outputs stay absent from UTXOs. Explicit BASIC indexing uses
the genesis transaction's output script and the zero predecessor commitment.
The selector `testnet` means testnet3; Signet and regtest use their default profiles.
The genesis RPC context has no wallet authority: wallet operations refuse.

Ordinary history remains refused pending Phase 167, including Signet challenge
validation. Sync, inbound, transaction relay, block serving and compact relay
activation remain refused before storage opening. Networking remains default off.
Actual pinned Knots-daemon differential qualification remains deferred to Phase 168.
Local Cargo/Bazel process evidence concerns Open Bitcoin's own products.

Use the [bounded genesis evidence and exact operator commands](../../docs/parity/v2-6-canonical-durable-bootstrap.md).
Daemon startup requires an existing explicit datadir and an existing durable
trigger such as `-blockfilterindex=1`; a missing configured datadir still fails
the parser. After Ctrl-C/SIGINT shutdown, offline `sync status` only reopens
existing proved genesis and preserves the saved BASIC mode. It creates no saved
status/progress record. An admitted Fjall open can have filesystem effects;
physical preservation is established by tested refusal/pre-admission controls.

Cooperating application leases cover the supported Unix route; actual process
evidence is macOS. Finite private-copy inspection, child completion and cleanup
limits are documented with the evidence. No repair, conversion, import or
automatic history acquisition is offered. Whole-chain parity, public defaults,
production service and production-funds readiness remain outside the claim.

## Local observation and verification

Follow the [complete disposable startup/reopen procedure](../../docs/parity/v2-6-canonical-durable-bootstrap.md#operator-reproduction)
from the repository root. It creates an existing `data` directory, a valid
`open-bitcoin.jsonc`, and a local daemon with explicit `-blockfilterindex=1`.
After Ctrl-C/SIGINT and complete daemon exit, use either form serially in the
same shell where `genesis_review_dir` was created:

```bash
bun run scripts/command-timings.ts run --key phase166-docs-cargo-status -- cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --config "$genesis_review_dir/open-bitcoin.jsonc" --datadir "$genesis_review_dir/data" --network regtest --format json sync status
bun run scripts/command-timings.ts run --key phase166-docs-bazel-status -- bazel run //packages/open-bitcoin-cli:open_bitcoin -- --config "$genesis_review_dir/open-bitcoin.jsonc" --datadir "$genesis_review_dir/data" --network regtest --format json sync status
```

Offline status is existing-only and does not save synthesized status/progress.
An admitted backend open can recover/update filesystem state. Pre-admission
inspection and incompatible-source refusal preserve the source within the
documented bounds. Fixture constructors above retain their separate authority.
Use `bash scripts/verify.sh` for first-party pre-commit/release verification.
Phase166 final native verification and generated LOC freshness belong to Plan14
and normal hooks; focused results alone do not close those gates.

Historical Phase165 fixture-source and guard-repair receipts are linked from the
[work parity record](../../docs/parity/v2-6-shared-exact-work.md). The
[acceptance record](../../.planning/phases/165-shared-exact-work-authority/165-ACCEPTANCE.md)
separately records actual native attempts and final verification gates.
Complete-prefix fixture requalification does not make raw reopen or ordinary
canonical startup eligible.

The [Phase165 Plan41 source checkpoint](../../.planning/phases/165-shared-exact-work-authority/165-41-SUMMARY.md)
preserves refusal diagnostics for its original source. The Phase165 acceptance
record above records its full default verifier exit 0, fresh-coverage evidence,
original postprocessing error and separate acceptance decisions. Those results
retain their original scope; consult the Phase166 verification/acceptance
records for current gate outcomes.
