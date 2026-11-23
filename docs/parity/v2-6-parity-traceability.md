# v2.6 parity traceability

`v2-6-cpar-01` joins the [approved requirements](requirements.md) to the
[ledger](index.json) entries that own them. An owner links its current
tests or observed results, evidence classes, pinned Knots sources, and
claim limits. The check verifies the original source bytes at each Knots
anchor. The `--print` output also gives the receipt and executable hashes
for the seven actual Regtest comparisons and identifies the v2.5 archive
commit and its preserved files.

```sh
bun run scripts/check-v2-6-traceability.ts --check
bun run scripts/check-v2-6-traceability.ts --print
```

| Requirement group | Evidence entry point |
| --- | --- |
| CBOT | [Canonical identity](v2-6-canonical-chain-identity.md) and [durable bootstrap](v2-6-canonical-durable-bootstrap.md) |
| CVLD | [Candidate-history validation](v2-6-candidate-history-validation.md) and [finite Mainnet audit](v2-6-candidate-history-validation.md#finite-mainnet-fixture-audit) |
| CWOR | [Shared exact work](v2-6-shared-exact-work.md) and [durable checkpoint](v2-6-canonical-durable-bootstrap.md) |
| CDIF | [Pinned Knots build and Regtest receipts](v2-6-knots-regtest-differential.md) |
| CREC | [Branch order](v2-6-knots-regtest-differential.md#branch-order-and-restart-ties), [replacement spends](v2-6-knots-regtest-differential.md#replacement-spends-and-reconnect), and [local recovery](v2-6-knots-regtest-differential.md#canonical-stop-and-reopen) |
| CINT | [Canonical prune and BASIC clients](v2-6-canonical-pruning.md), [operator status](v2-6-canonical-pruning.md#canonical-operator-status), and [finite Mainnet audit](fixtures/cint-04-mainnet-audit.json) |

## Intentional differences

The ledger’s `v2.6` deviation entries carry source anchors, owning surfaces,
and evidence links. A difference has an actual-result link when that link
appears in its evidence. Most storage and refusal differences are scoped
source and local-product comparisons.

| Difference | Evidence |
| --- | --- |
| Proved store ownership and bounded canonical acceptance | [Bootstrap refusal limits](v2-6-canonical-durable-bootstrap.md#refusal-inspection-and-recovery-limits) and the [tested Regtest subset](receipts/cdif-05-regtest.json) |
| Typed invalid-target work errors | [Exact work differences](v2-6-shared-exact-work.md#intentional-internal-differences-and-remaining-ownership) |
| Complete overwritten Coin restoration for the exact seeded BIP30 repeats | [Historical BIP30 evidence and scope](v2-6-candidate-history-validation.md) |
| Paired Fjall deletion and logical byte accounting | [Canonical pruning evidence](v2-6-canonical-pruning.md) |
| Manual prune keep-window refusal | [Observed operator refusal](v2-6-canonical-pruning.md#canonical-operator-status) |
| Replay archive required for the bounded pruned-Regtest startup API | [Retained BASIC after reopen](v2-6-canonical-pruning.md#retained-basic-clients-after-reopen) |

The equal-work, equal-sequence Knots restart fallback is an observed
reference ambiguity. The [branch receipt](receipts/crec-01-regtest.json)
identifies the allowed tied candidates in its Knots-only probe. It also
records both ordinary restart observations in their separate case.

## Archived v2.5 provenance

The archive commit is `5b0b2acfacc8ad47ceeb926c469da05a7712aa57`.
The checker verifies ten archived files against SHA-256 values taken from
that commit, including milestone controls, native and UAT evidence, and the
original integrated proof. The [archive-safe provenance guide](v2-5-archive-safe-provenance.md)
retains the original lifecycles and evidence limits. Those include the
synthetic root, caller-supplied work, the inherited subsidy interval, the
soft logical prune target and missing-history refusal. The v2.6 Regtest
receipts apply to their own source identities and scopes.

CPAR-02 tracks qualification freshness and invalidation. The traceability
check reports recorded results with their existing evidence classes.
