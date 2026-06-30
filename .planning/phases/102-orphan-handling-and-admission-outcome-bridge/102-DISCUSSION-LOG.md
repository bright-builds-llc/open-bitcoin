# Phase 102: Orphan Handling and Admission Outcome Bridge - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md; this log preserves the alternatives considered.

**Date:** 2026-06-30T14:54:50.926Z
**Phase:** 102-orphan-handling-and-admission-outcome-bridge
**Mode:** Yolo
**Areas discussed:** Missing-parent staging and parent requests, reconsideration flow, admission outcome contract, admission policy scope, managed runtime bridge, resource governance and boundaries

## Missing-Parent Staging And Parent Requests

| Option | Description | Selected |
| --- | --- | --- |
| Typed bounded orphan state | Stage missing-parent peer transactions as typed orphan/candidate state with caps, expiry, parent requests, and low-cardinality evidence. | yes |
| Treat missing parent as ordinary rejection | Return only the existing mempool error and leave parent request behavior to later phases. | |
| Socket-owned orphan handling | Let peer/socket code directly mutate orphan and mempool state. | |

**User's choice:** Auto-selected typed bounded orphan state as the recommended default.
**Notes:** This keeps Phase 101's pure download scheduler boundary intact and satisfies DL-03 and DL-04 without creating direct socket-to-mempool mutation.

## Reconsideration Flow

| Option | Description | Selected |
| --- | --- | --- |
| Deterministic reconsideration after parent acceptance | Parent acceptance returns bounded child reconsideration candidates and evidence for accepted, still-missing, rejected, expired, and evicted paths. | yes |
| Manual-only reconsideration | Stage orphans but do not automatically reconsider children in Phase 102. | |
| Recursive unbounded reconsideration | Walk descendants until exhaustion without a deterministic cap. | |

**User's choice:** Auto-selected deterministic reconsideration after parent acceptance as the recommended default.
**Notes:** Reconsideration needs fake-clock/cap-driven tests and should avoid recursion that can bypass resource-governance limits.

## Admission Outcome Contract

| Option | Description | Selected |
| --- | --- | --- |
| One stable typed outcome contract | Map peer and local submissions through accepted, rejected, duplicate, replaced, orphaned, evicted, and expired outcomes. | yes |
| Keep `MempoolError` as caller contract | Expose only current errors and let callers infer outcome categories. | |
| Add separate peer-only outcome surface | Give peer submissions a different contract from local submissions. | |

**User's choice:** Auto-selected one stable typed outcome contract as the recommended default.
**Notes:** This directly satisfies MEM-01 and creates a later observability/RPC/support seam without implementing those later surfaces now.

## Admission Policy Scope

| Option | Description | Selected |
| --- | --- | --- |
| Deepen pure mempool tests | Cover standardness, fee, RBF, ancestor/descendant, duplicate, and no-partial-mutation behavior in pure mempool tests first. | yes |
| Adapter-first coverage | Prove most behavior through managed runtime integration tests. | |
| Broaden to package relay | Fold package or cluster mempool behavior into this phase. | |

**User's choice:** Auto-selected deepening pure mempool tests as the recommended default.
**Notes:** Adapter tests still matter for the bridge, but policy correctness should stay cheap and pure.

## Managed Runtime Bridge

| Option | Description | Selected |
| --- | --- | --- |
| Shell-owned bridge | Use `ManagedPeerNetwork::process_actions` or a small child module to translate peer/download actions into mempool outcomes. | yes |
| Peer-manager admission | Let `PeerManager` invoke mempool admission directly. | |
| Delay integration | Implement outcomes without proving peer transactions pass through the relay/download boundary. | |

**User's choice:** Auto-selected shell-owned bridge as the recommended default.
**Notes:** This preserves functional-core/imperative-shell boundaries and satisfies the managed runtime success criterion.

## Resource Governance And Boundaries

| Option | Description | Selected |
| --- | --- | --- |
| Preserve existing deterministic caps and no-claim guardrails | Orphan/admission behavior must honor Phase 94/101 limits and keep verification local. | yes |
| Add public-network relay verification | Use live relay behavior as proof for Phase 102. | |
| Expand v2.0 claims | Treat orphan/admission work as broad relay or production-readiness support. | |

**User's choice:** Auto-selected preserving deterministic caps and no-claim guardrails as the recommended default.
**Notes:** Public-network relay review, production claims, compact blocks, package relay, and filters remain deferred.

## the agent's Discretion

- Exact type names, module split, orphan caps, expiry constants, and placement of the orphan staging/coordinator.
- Whether the first bridge is a child module under network runtime or a focused adapter around managed mempool submission.

## Deferred Ideas

- Durable mempool persistence and restart recovery.
- Block connect/disconnect mempool lifecycle.
- Relay serving, fanout, rebroadcast, RPC/operator/support evidence, and release closeout.
- Compact block relay, package relay, bloom/filter serving, public relay defaults, public-network relay CI, production full-node readiness, and production-funds wallet use.
