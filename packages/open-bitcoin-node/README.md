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

Ordinary history is currently refused for all four selectors: mainnet, testnet
(testnet3), default Signet and default regtest. Their
[identity-only support/refusal matrix](../../docs/parity/v2-6-canonical-chain-identity.md)
remains authoritative. Observed stored work is unavailable; the filesystem-only
gate appends `work_decision` before any backend open, migration, recovery or BASIC
write. Preserve existing data and review it separately; an empty/separate datadir
also refuses. Networking remains default off.

Phase 166 retains canonical bootstrap, exact checkpoint/all-handle reopen and
genesis BASIC; Phase 167 retains contextual completeness; Phase 168 retains actual
pinned-Knots differential qualification. Local Open Bitcoin refusal processes and
explicit synthetic fixture lifecycles do not grant those capabilities or a
whole-chain, production-service or production-funds claim.

## Local inspection and verification

From the repository root, inspect an isolated stopped configuration without
starting public networking. Set up the disposable paths once in the same shell:

```bash
node_review_dir=$(mktemp -d /tmp/open-bitcoin-node-review.XXXXXX)
```

Use either repository-local form, serially:

```bash
bun run scripts/command-timings.ts run --key phase165-node-cargo-status -- cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --config "$node_review_dir/absent.jsonc" --datadir "$node_review_dir/empty" --network regtest --format json status
bun run scripts/command-timings.ts run --key phase165-node-bazel-status -- bazel run //packages/open-bitcoin-cli:open_bitcoin -- --config "$node_review_dir/absent.jsonc" --datadir "$node_review_dir/empty" --network regtest --format json status
```

Status can report compatibility unavailability; it does not open accepted history
or prove indexing/recovery. For first-party pre-commit/release verification use
`bash scripts/verify.sh`. Focused evidence and its limits live in the
[work parity record](../../docs/parity/v2-6-shared-exact-work.md).

Current fixture-source and guard-repair receipts are linked from the
[work parity record](../../docs/parity/v2-6-shared-exact-work.md). The
[acceptance record](../../.planning/phases/165-shared-exact-work-authority/165-ACCEPTANCE.md)
separately records actual native attempts and final verification gates.
Complete-prefix fixture requalification does not make raw reopen or ordinary
canonical startup eligible.

The accepted [Plan41 source checkpoint](../../.planning/phases/165-shared-exact-work-authority/165-41-SUMMARY.md)
preserves independent historical refusal diagnostics while current source
qualification stays gated. These source-policy results retain the support limits
above. Fresh whole-repository native verification and final acceptance remain
separately tracked in the acceptance record above.

The full default repository verifier completed with exit 0. The acceptance record
preserves native and fresh-coverage evidence, the original postprocessing error
and the separate acceptance decisions. Follow its canonical verification links
for phase status and scope. The support boundaries above remain unchanged.
