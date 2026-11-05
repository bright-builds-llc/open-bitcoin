# Phase 167: Candidate-History Validation Rules - Discussion Log

> Audit trail only. CONTEXT.md is the downstream decision input.

**Date:** 2026-10-21
**Mode:** Yolo, one recommendation pass; no interactive answers were requested.
**Lifecycle:** 167-2026-10-21T16-38-23

## Network authority and support

| Question | Selected recommended answer | Alternative considered | Rationale |
| --- | --- | --- | --- |
| Which rules authorize ordinary validation? | Immutable selected-network facts plus proved candidate history | Generic caller ConsensusParams | Caller flags cannot establish canonical authority. |
| What constitutes default-regtest evidence? | Canonical genesis, maturity100, halving150 and serialized accepted histories | Mature-by-override synthetic histories | Required by CVLD-01 and future genuine qualification. |
| How is incomplete support exposed? | Explicit bounded refusal with missing prerequisites | Broad enabled selector | Preserve Phase164 classification and Signet challenge refusal. |

## Candidate context and historical exceptions

| Question | Selected recommended answer | Alternative considered | Rationale |
| --- | --- | --- | --- |
| Which ancestry drives activation and time/difficulty? | Actual candidate branch with checked prerequisites | Current active/best-header shortcuts | Competing candidates must not borrow unrelated history. |
| How are script flags selected? | Exact pinned baseline, hash exceptions and deployment additions through actual connect | Always-on modern flags | CVLD-03 requires observable historical behavior. |
| How are duplicates/overwrites supported? | Pinned exact facts and restoration-capable undo | Global duplicate allowance | CVLD-06 requires safe bounded historical semantics. |

## Integration, verification and ownership

| Question | Selected recommended answer | Alternative considered | Rationale |
| --- | --- | --- | --- |
| Which path proves behavior? | Ordinary header admission and durable owner after proved genesis | Disconnected pure helpers only | Approved criteria require actual integration. |
| What does rejection prove? | Unchanged accepted tip/coins/work/index/durable authority | Error string alone | Failed validation must not publish state. |
| What parity claim is earned? | Source-bound finite behavioral evidence | Actual-daemon or whole-chain claim | Actual qualification belongs to Phase168. |
| When is Git finalized? | After full native checks, independent reviews, goal/lifecycle pass and normal hooks | Intermediate nested commits | Strict wrapper defers all saving until clean phase acceptance. |

## Agent Discretion

Typed module seams, bounded representation choices, plan decomposition and focused verification commands within the approved scope. Existing dependencies and pure-core boundaries remain required.

## Deferred Ideas

Actual differential runner, fork/recovery qualification, canonical prune integration and historical parity closure retain Phases168–171 ownership. No additional features were folded into scope.
