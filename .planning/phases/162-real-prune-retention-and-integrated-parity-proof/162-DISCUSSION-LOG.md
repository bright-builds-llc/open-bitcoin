# Phase 162: Real Prune Retention and Integrated Parity Proof - Discussion Log

Audit trail only; downstream decisions are in `162-CONTEXT.md`.

**Date:** 2026-10-11
**Mode:** Yolo
**Lifecycle:** 162-2026-10-11T16-25-37

## Continuous accepted history

Question: Which existing recipe can satisfy the integrated proof?

| Option | Tradeoff | Selected |
| --- | --- | --- |
| Extend normal-maturity accepted spend/fork history past 1000 | Exercises genuine durable history, normal maturity, pruning thresholds and positive checkpoints | Yes |
| Combine sparse or maturity-one constituent tests | Cheap but insufficient for the roadmap acceptance gate | No |

Auto-selected recommended option. Carry actual runtime parameters; auxiliary codec-valid volume supplies storage pressure only. Preserve exact history provenance across caller composition.

## Lifecycle and client evidence

Question: What must survive actual paired deletion and reopen?

| Option | Tradeoff | Selected |
| --- | --- | --- |
| Production startup/idle loop/connect/reorg/prune/reopen with authenticated RPC and both socket adapters/all three peer families | Closes every actual handoff, earns physical write receipts and checks nonempty checkpoints | Yes |
| Source wiring and prepared-response assertions | Useful supplementary evidence but cannot prove integrated clients | No |

Auto-selected recommended option. Keep ordinary body serving and wallet restrictions visible in the same scenario.

## Faults and parity

Question: How should existing fault evidence contribute?

| Option | Tradeoff | Selected |
| --- | --- | --- |
| Concrete durable-coins/Fjall production-boundary faults with explicit scenario matrix and independent pinned oracles | Preserves valid prefixes, safe progress and honest achieved receipts | Yes |
| Simulated counters or MemoryCoinsView-only coverage | Cannot satisfy the concrete durability requirement | No |

Auto-selected recommended option. Reuse narrow seams and adapt historical pending-claim guards with mutation tests; preserve archived reports and all substantive checks.

## Documentation and finalization

Question: What does completion authorize?

| Option | Tradeoff | Selected |
| --- | --- | --- |
| Scoped phase completion after independent review, full native verification and lifecycle validation; mandatory hook then push existing main | Matches strict wrapper and retains all exclusions | Yes |
| Automatically ship/archive v2.5 or broaden readiness claims | Requires separate milestone audit and evidence | No |

Auto-selected recommended option. Provide explicit Cargo/Bazel UAT, reuse current terminal presentation, and defer intermediate commits until clean verification. No matching todos.

## Agent Discretion and Deferred Ideas

Research chooses helper boundaries, bounded auxiliary pressure and test-only fault seams. BIP37/V0, archive-scale, assumeutxo, implicit repair/import/download, public defaults/network CI, new GUI/operator features, production/funds claims and unrelated crash-window cleanup remain deferred.
