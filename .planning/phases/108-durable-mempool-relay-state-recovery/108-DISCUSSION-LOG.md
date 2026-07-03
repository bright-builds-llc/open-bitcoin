# Phase 108: Durable Mempool Relay State Recovery - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md - this log preserves the alternatives considered.

**Date:** 2026-07-03T14:09:06.388Z
**Phase:** 108-durable-mempool-relay-state-recovery
**Mode:** Yolo
**Areas discussed:** Recovery replay into relay state, lifecycle coherence after restart, operator evidence and redaction, storage corruption and repair, tests/docs/guardrails

## Recovery Replay Into Relay State

| Option | Description | Selected |
| --- | --- | --- |
| Reuse live relay-serving/fanout paths | Replay accepted recovered records through the same managed serving/fanout evidence paths used for live accepted outcomes. | yes |
| Keep recovery mempool-only | Restore pure mempool entries but leave serving/fanout state untouched until a later live event. | |
| Add a separate recovered cache | Add recovery-only relay indexes separate from existing serving/fanout state. | |

**User's choice:** Auto-selected reuse of live relay-serving/fanout paths.
**Notes:** This is the only option that satisfies Phase 108's requirement that recovered accepted transactions become serveable or suppressible through the same typed relay-serving policy as live accepted transactions.

## Lifecycle Coherence After Restart

| Option | Description | Selected |
| --- | --- | --- |
| Shared cleanup path | Use existing block/reorg/replacement/eviction/expiry cleanup helpers for recovered and live records. | yes |
| Recovery-specific cleanup | Add separate cleanup code paths for recovered records. | |
| Partial cleanup only | Clear mempool entries while leaving serving/fanout state to expire naturally. | |

**User's choice:** Auto-selected shared cleanup path.
**Notes:** This follows Phase 103 and Phase 104 decisions and avoids stale serving entries after restart.

## Operator Evidence And Redaction

| Option | Description | Selected |
| --- | --- | --- |
| Shared sanitized status projection | Extend or reuse `RelayEvidenceStatus` and support redaction helpers with fixed aggregate labels. | yes |
| Raw recovery diagnostics | Expose transaction identifiers or free-form recovery reasons for debugging. | |
| Docs-only evidence | Explain recovery behavior without status/support/test-visible counters. | |

**User's choice:** Auto-selected shared sanitized status projection.
**Notes:** This preserves Phase 105's low-cardinality operator evidence boundary and keeps recovered relay state non-promissory.

## Storage Corruption And Repair

| Option | Description | Selected |
| --- | --- | --- |
| Typed safe-drop/diagnosis | Map corrupt or incompatible snapshots to typed recovery evidence and safe non-destructive handling. | yes |
| Destructive repair | Mutate or rebuild stores automatically when corruption is detected. | |
| Silent ignore | Drop bad records without surfaced recovery evidence. | |

**User's choice:** Auto-selected typed safe-drop/diagnosis.
**Notes:** Destructive repair and source datadir mutation remain out of scope.

## Tests, Docs, And Guardrails

| Option | Description | Selected |
| --- | --- | --- |
| Deterministic local coverage | Add pure replay, managed restart, lifecycle cleanup, redaction, checker, and verifier coverage as needed. | yes |
| Public-network relay UAT gate | Add live relay review as a default verifier or CI gate. | |
| Minimal tests only | Rely on existing Phase 103/104 tests without recovery-specific guardrails. | |

**User's choice:** Auto-selected deterministic local coverage.
**Notes:** The final gate remains `bash scripts/verify.sh`; public-network relay remains opt-in UAT only.

## the agent's Discretion

- Exact type names, module split, recovery evidence counter names, and plan granularity.
- Whether recovery replay is implemented beside `mempool_snapshot`, inside `ManagedPeerNetwork`, or through a focused managed recovery helper.
- Whether new checker coverage is added by extending existing Phase 103/104/105/107 checkers or creating a Phase 108 checker, as long as verifier order and evidence are deterministic.

## Deferred Ideas

- Public transaction relay by default.
- Public-network relay UAT as a default CI or pre-commit gate.
- Compact block relay, package relay, bloom/filter serving, production service operation, production full-node readiness, production-funds wallet safety, packaging, GUI, hosted dashboards, migration apply mode, automatic support-bundle upload, and destructive repair.
