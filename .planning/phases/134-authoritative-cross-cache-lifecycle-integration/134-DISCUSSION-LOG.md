---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 134-2026-07-28T01-41-12
generated_at: 2026-07-28T01:49:35.254Z
---

# Phase 134: Authoritative Cross-Cache Lifecycle Integration - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution
> agents. Decisions are captured in `134-CONTEXT.md`; this log preserves the
> alternatives considered.

**Date:** 2026-07-27
**Phase:** 134-authoritative-cross-cache-lifecycle-integration
**Mode:** Yolo
**Areas discussed:** Authoritative mutation boundary, lifecycle projection and
cleanup, effect extraction and typed receipts

## Authoritative Mutation Boundary

| Option | Description | Selected |
| --- | --- | --- |
| Typed lifecycle command family plus single projector | One exhaustive mutation vocabulary and one I/O-free application path, with narrow facades permitted over it. | ✓ |
| Independent narrow per-surface methods | Minimal caller churn, but consequence logic remains distributed unless every method delegates to one projector. | |
| Single-owner actor with typed mailbox | Structural serialization, but requires a broad queue, cancellation, shutdown, fairness, and runtime migration. | |

**Auto-selected choice:** Typed lifecycle command family plus single projector.

**Notes:** This preserves the Phase 127 shared-handle decision while making
projection omissions mechanically visible. Actor conversion remains deferred.

## Lifecycle Projection and Cleanup

| Option | Description | Selected |
| --- | --- | --- |
| Prepared exhaustive projection plan plus infallible aggregate reducer | Preflight all fallible identity/body work, then update every authoritative cache synchronously and completely. | ✓ |
| Typed lifecycle observer bus | Decouples consumers, but hides exhaustiveness and introduces partial-handler or stale-queue failure modes. | |
| Generation-based rebuild and reconciliation | Useful as an oracle and recovery mechanism, but loses removal cause and volatile provenance and is too expensive for the hot path. | |

**Auto-selected choice:** Prepared exhaustive projection plan plus infallible
aggregate reducer.

**Notes:** Final membership governs projection. Admissions remain parent-first;
removals become descendant-first; reorg transitions compose sequentially.

## Effect Extraction and Typed Receipts

| Option | Description | Selected |
| --- | --- | --- |
| Effect-family prepare, execute, and complete capabilities | Owned bounded commands leave the lock; consuming generation-bound receipts report exact achieved effects. | ✓ |
| Closed generic `EffectBatch` | Centralizes dispatch, but couples unrelated effects and weakens family-specific stale semantics. | |
| Single-owner authority actor with I/O workers | Avoids shared-lock I/O structurally, but exceeds the proportional Phase 134 repair. | |
| Durable transactional outbox plus completion journal | Adds replay and deduplication across restart, but pulls Phase 135–136 durability and scheduling scope forward. | |

**Auto-selected choice:** Effect-family prepare, execute, and complete
capabilities.

**Notes:** Complete every successful prefix exactly once. Stale receipts may
record achieved external truth but cannot clear newer authority state.

## the agent's Discretion

- Exact type and module names, bounded batch sizes, generation representation,
  completed-effect ledger representation, and focused test fixtures.
- Exact reverse-index strategy for accepted package fingerprints, provided
  cleanup is complete and bounded.

## Deferred Ideas

- Runtime actor conversion.
- Generic heterogeneous effect bus.
- Durable transactional outbox and completion journal.
- Later-phase snapshot, scheduling, transport, operator-surface, and final
  release-guardrail work.

