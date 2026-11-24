# v2.6 parity closeout

All 30 approved v2.6 requirements have completed owners in the parity ledger.
The verification commit on `main` is
[`1e1387d11683aa5abf21759d3416cf136e01362d`](https://github.com/bright-builds-llc/open-bitcoin/commit/1e1387d11683aa5abf21759d3416cf136e01362d).
All 11 local pre-push shards and all 11 CI shards passed for that commit.
[CI run](https://github.com/bright-builds-llc/open-bitcoin/actions/runs/45283281531) and
[Bright Builds Checks](https://github.com/bright-builds-llc/open-bitcoin/actions/runs/45283281560).

The [parity trace](v2-6-parity-traceability.md) ties the requirements and
six intentional differences to source anchors and evidence. The seven
[daemon receipts](v2-6-knots-regtest-differential.md) record actual comparisons
against Bitcoin Knots `29.3.knots20260210` at
`a9aee730466ac67d35a3c03ee24676be5e045878`. The Open Bitcoin binaries used
build commit `b346a29bca16ba25ad1ae4f1fd93646e60b5cf92`. The offline CI
gate verified all seven recorded outcomes against the current shared input
digest. [Qualification freshness](v2-6-knots-regtest-differential.md#qualification-freshness).

## Verified scope

- **Regtest daemon comparison.** Both daemons matched active tips through
  height 301, including coinbase maturity at height 101 and subsidy reductions
  at heights 150 and 300. The receipts also cover invalid inputs, exact
  chainwork, UTXOs, BASIC filters and headers, and defined fork and reconnect
  behavior. [Differential evidence](v2-6-knots-regtest-differential.md).
- **Open Bitcoin runtime.** Canonical genesis and explicit BASIC bootstrap
  are verified for all four existing selectors. Isolated Regtest evidence
  covers durable reopen, bounded checkpoint recovery, and paired pruning at
  height 1001 with archive-backed reopen and retained BASIC queries.
  [Bootstrap](v2-6-canonical-durable-bootstrap.md) · [Prune and reopen](v2-6-canonical-pruning.md).
- **Historical validation.** Six Mainnet blocks have finite, seeded evidence
  with explicit ancestry, spent-output and activation prerequisites. The
  current ordinary product route refuses unsupported descendants on Mainnet,
  testnet3, and default Signet.
  [Candidate-history evidence](v2-6-candidate-history-validation.md#finite-mainnet-fixture-audit).

The qualification covers isolated, bounded Regtest. Whole-chain validation,
public-default operation, and production readiness are outside this evidence.
Public networking defaults to off, and the canonical-history runtime wallet
is unavailable.

## v2.5 ledger reconciliation

The [archived v2.5 audit](../../.planning/milestones/v2.5-MILESTONE-AUDIT.md)
marks `CFIL-01`, `CFIL-02`, `CFIX-02`, `CFIX-04`, and `CFPR-03` satisfied.
The ledger records both owners as `done` and links the original verification
for [Phase 154](../../.planning/phases/154-basic-generation-and-commitment-parity/154-VERIFICATION.md)
and [Phase 155](../../.planning/phases/155-recoverable-index-and-pre-prune-startup-protection/155-VERIFICATION.md).
Those records support the original BASIC generation and pre-prune recovery
scope. [v2.5 evidence limits](v2-5-archive-safe-provenance.md#preserved-product-and-evidence-limits).
