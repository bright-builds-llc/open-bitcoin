# Parity Checklist

This checklist is the human-readable view of [`index.json`](index.json). The
machine-readable source remains `checklist.surfaces`, and this page exists to
make the initial milestone audit surface easy to scan.

## Status Taxonomy

Checklist statuses are exactly `planned`, `in_progress`, `done`, `deferred`, `out_of_scope`.

## Surface Status

| Surface | Status | Requirements | Evidence | Known Gaps | Suspected Unknowns |
| --- | --- | --- | --- | --- | --- |
| `reference-baseline` | `done` | `REF-01`, `REF-02` | [Phase 01 baseline](../../.planning/phases/01-workspace-baseline-and-guardrails/01-01-SUMMARY.md), [Phase 01 ledger](../../.planning/phases/01-workspace-baseline-and-guardrails/01-04-SUMMARY.md), [Phase 01 verification](../../.planning/phases/01-workspace-baseline-and-guardrails/01-VERIFICATION.md) | None recorded. | None recorded. |
| `architecture-workspace` | `done` | `ARCH-01`, `ARCH-02`, `ARCH-03`, `ARCH-04`, `VER-01`, `VER-02` | [Workspace wiring](../../.planning/phases/01-workspace-baseline-and-guardrails/01-02-SUMMARY.md), [guardrails](../../.planning/phases/01-workspace-baseline-and-guardrails/01-03-SUMMARY.md), [verification](../../.planning/phases/01-workspace-baseline-and-guardrails/01-VERIFICATION.md), [verify script](../../scripts/verify.sh) | None recorded. | None recorded. |
| `core-serialization` | `done` | `REF-03`, `CONS-01`, `ARCH-03` | [Core catalog](catalog/core-domain-and-serialization.md), [Phase 02 verification](../../.planning/phases/02-core-domain-and-serialization-foundations/02-VERIFICATION.md) | None recorded. | Deprecated hex acceptance, serializer parameters, and protocol/address fixture boundaries remain audit watch items. |
| `consensus-validation` | `done` | `CONS-02`, `CONS-03` | [Consensus catalog](catalog/consensus%2Dvalidation.md), [Phase 03.4 verification](../../.planning/phases/03.4-consensus-parity-closure/03.4-VERIFICATION.md), [Phase 07.6 verification](../../.planning/phases/07.6-enforce-coinbase-subsidy-plus-fees-limits-on-the-consensus-a/07.6-VERIFICATION.md) | None recorded for the current in-scope surface. | None recorded. |
| `chainstate` | `done` | `CHAIN-01` | [Catalog entry](catalog/chainsta%74e.md), [Phase 04 verification](../../.planning/phases/04-chainsta%74e-and-utxo-engine/04-VERIFICATION.md) | Disk-backed coin databases, cache-flush policy, assumeutxo, disconnected-transaction repair, and full manager behavior remain outside the current slice. | Cache and multi-manager behavior ownership remains an audit watch item. |
| `mempool-policy` | `done` | `MEM-01`, `MEM-02` | [Policy catalog](catalog/mempool%2Dpolicy.md), [Phase 05 verification](../../.planning/phases/05-mempool-and-node-policy/05-VERIFICATION.md) | Package relay, rolling minimum-fee decay, reorg repair, and operator-facing interfaces remain follow-up surfaces. | Long-lived pressure and package-relay parity remain audit watch items. |
| `p2p-networking` | `done` | `P2P-01`, `P2P-02` | [P2P catalog](catalog/p2p.md), [Phase 06 verification](../../.planning/phases/06-p2p%2Dnetworking-and-sync/06-VERIFICATION.md) | Address relay, discovery, compact blocks, encrypted transport, peer eviction, bans, resource governance, and long-running socket orchestration remain follow-up surfaces. | Future peer governance, discovery, and address-relay fidelity remain audit watch items. |
| `wallet` | `done` | `WAL-01`, `WAL-02`, `WAL-03` | [Wallet catalog](catalog/walle%74.md), [Phase 07 verification](../../.planning/phases/07-walle%74-core-and-adapters/07-VERIFICATION.md) | Ranged descriptors, HD derivation, miniscript, multisig, PSBT, encryption, external signers, real-node rescans, and broader RPC semantics remain follow-up surfaces. | Future HD, multisig, PSBT, coin-selection, and metadata ownership remain audit watch items. |
| `rpc-cli-config` | `done` | `RPC-01`, `CLI-01`, `CLI-02` | [Operator catalog](catalog/rpc-cli%2Dconfig.md), [Phase 08 verification](../../.planning/phases/08-rpc-cli-and-config-parity/08-VERIFICATION.md) | Richer send ergonomics, peer-info views, multi-endpoint selection, ACL features, daemon supervision, and broader process-control helpers remain deferred. | Future send, peer-info, and multi-endpoint semantics remain audit watch items. |
| `verification-harnesses-fuzzing` | `done` | `VER-03`, `VER-04`, `PAR-01` | [Harness catalog](catalog/verification-harnesses.md), [Phase 09 verification](../../.planning/phases/09-parity-harnesses-and-fuzzing/09-VERIFICATION.md), [verify script](../../scripts/verify.sh) | Vendored Knots process management, full upstream Python suite translation, and a dedicated fuzz runtime remain follow-up surfaces. | Functional-suite translation versus wrapping, plus dedicated fuzz-runtime scope, remain audit watch items. |
| `benchmarks-audit-readiness` | `in_progress` | `PAR-02`, `AUD-01` | [Benchmark docs](benchmarks.md), [benchmark runner](../../scripts/run-benchmarks.sh), [Plan 10-03 summary](../../.planning/phases/10-benchmarks-and-audit-readiness/10-03-SUMMARY.md) | Release-readiness and milestone handoff documentation remain planned for Plan 10-05. | Final readiness review may surface additional follow-up items before milestone closeout. |

## Evidence Rules

- `done` surfaces must link to concise existing evidence, such as catalog pages,
  verification reports, plan summaries, scripts, or checked-in docs.
- `deferred` and `out_of_scope` surfaces must carry explicit rationale in
  [`index.json`](index.json).
- Known gaps and suspected unknowns are audit signals. They do not imply broad
  follow-on implementation inside Phase 10 unless a later plan says so.
