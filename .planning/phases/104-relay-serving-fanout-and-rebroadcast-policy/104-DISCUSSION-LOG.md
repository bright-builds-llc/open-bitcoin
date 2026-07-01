---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 104-2026-07-01T14-38-26
generated_at: 2026-07-01T14:38:26.627Z
---

# Phase 104: Relay Serving, Fanout, and Rebroadcast Policy - Discussion Log

> Audit trail only. Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md; this log preserves the alternatives considered.

**Date:** 2026-07-01T14:38:26.627Z
**Phase:** 104 - Relay Serving, Fanout, and Rebroadcast Policy
**Mode:** Yolo
**Areas discussed:** Relay-eligible transaction serving, accepted-transaction
fanout, local submission relay evidence, rebroadcast boundary, lifecycle cleanup
and coherence, tests/parity/guardrails

## Relay-Eligible Transaction Serving

| Option | Description | Selected |
| --- | --- | --- |
| Accepted relay cache | Serve only transactions present in accepted mempool-backed relay state and classify missing/stale/lifecycle outcomes. | yes |
| Loose known-transaction map | Serve any transaction found in existing runtime maps without deeper eligibility classification. | |
| Defer transaction serving | Keep `getdata` transaction responses as missing until a later observability phase. | |

**User's choice:** Auto-selected accepted relay cache.
**Notes:** This follows REL-01 while reusing Phase 102/103 outcomes and cleanup.

## Accepted-Transaction Fanout

| Option | Description | Selected |
| --- | --- | --- |
| Bounded pure fanout policy | Enqueue low-cardinality announcement actions per eligible peer with queue/rate/suppression limits. | yes |
| Immediate adapter sends | Translate accepted transactions directly into socket messages in managed code. | |
| Defer fanout | Implement serving only and leave announcements to Phase 105. | |

**User's choice:** Auto-selected bounded pure fanout policy.
**Notes:** This preserves functional-core boundaries and makes fanout fake-clock testable.

## Local Submission Relay Evidence

| Option | Description | Selected |
| --- | --- | --- |
| Outcome plus queued evidence | Accepted local submissions store the transaction and record queued/suppressed relay evidence without propagation guarantees. | yes |
| RPC propagation guarantee | Treat `sendrawtransaction` success as public propagation. | |
| No local relay evidence | Keep local submissions accepted only, with no fanout visibility. | |

**User's choice:** Auto-selected outcome plus queued evidence.
**Notes:** This keeps `sendrawtransaction` truthful and leaves rich presentation to Phase 105.

## Rebroadcast Boundary

| Option | Description | Selected |
| --- | --- | --- |
| Explicit deferral | Mark rebroadcast deferred across docs, internal status/policy evidence, and tests. | yes |
| Bounded scheduler now | Implement a timer-driven rebroadcast scheduler in Phase 104. | |
| Ignore rebroadcast | Do not represent REL-04 in implementation or docs. | |

**User's choice:** Auto-selected explicit deferral.
**Notes:** This satisfies the roadmap's allowed REL-04 route while avoiding a broad public propagation claim.

## Lifecycle Cleanup And Coherence

| Option | Description | Selected |
| --- | --- | --- |
| Shared lifecycle cleanup | Use Phase 103 block/reorg/replacement/eviction cleanup to remove serving and fanout state. | yes |
| Separate relay cleanup | Maintain independent relay cleanup with separate state transitions. | |
| Best-effort cleanup | Let missing serving lookups handle stale data after the fact. | |

**User's choice:** Auto-selected shared lifecycle cleanup.
**Notes:** This prevents stale serveable or queued transactions after mempool lifecycle changes.

## Tests, Parity, And Guardrails

| Option | Description | Selected |
| --- | --- | --- |
| Pure then managed tests | Add pure policy tests first, managed integration second, and checker/docs if artifacts change. | yes |
| Managed-only tests | Rely on end-to-end managed tests for all relay behavior. | |
| Docs-only evidence | Close REL-04 and guardrails without policy/integration tests. | |

**User's choice:** Auto-selected pure then managed tests.
**Notes:** This matches Bright Builds and repo standards for pure-core coverage and deterministic verification.

## the agent's Discretion

- Exact type names, queue constants, rate-limit constants, and module split.
- Whether fanout/serving policy lives inside `peer::transaction_relay` or a sibling pure module.
- Exact internal evidence struct shape, as long as labels stay fixed and low-cardinality.

## Deferred Ideas

- Timer-driven periodic rebroadcast scheduling for local or wallet-originated transactions.
- Rich Phase 105 operator/RPC/metrics/log/support presentation.
- Phase 106 release-boundary and UAT closeout docs.
