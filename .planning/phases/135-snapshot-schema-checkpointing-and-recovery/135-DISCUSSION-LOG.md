# Phase 135: Snapshot Schema, Checkpointing, and Recovery - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution
> agents. Decisions are captured in CONTEXT.md; this log preserves the
> alternatives considered by the yolo recommendation engine.

**Date:** 2026-08-02T17:41:48.380Z
**Phase:** 135-snapshot-schema-checkpointing-and-recovery
**Mode:** Yolo
**Areas discussed:** Source-only snapshot contract, checkpoint durability and coalescing, policy-aware staged recovery

## Source-only snapshot contract

| Option | Description | Selected |
| --- | --- | --- |
| Atomic source-only envelope with separate unbroadcast set | Canonical witness transactions, acceptance times, capture metadata, and an explicit authoritative unbroadcast set; rebuild all derived state. | ✓ |
| Atomic envelope with self-contained source records | Store a local-unbroadcast boolean in every record for record-level salvage. | |
| Generation-partitioned records with committed manifest | Store records under generation keys with a manifest/pointer and old-generation cleanup. | |

**Agent choice:** Atomic source-only envelope with separate unbroadcast set.

**Notes:** This most closely matches the pinned Knots boundary, fits the existing
single-value Fjall adapter, and avoids treating historical local admission
metadata as proof that unbroadcast intent still survives.

## Checkpoint durability and coalescing

| Option | Description | Selected |
| --- | --- | --- |
| Synchronous periodic and clean-shutdown checkpoints | One power-loss-durable meaning with a quiesced final checkpoint. | ✓ |
| Flush periodic plus synchronous clean-shutdown checkpoint | Lower periodic sync pressure but only process-crash durability between clean stops. | |
| Operator-selectable periodic strength | Configurable durability with a mandatory synchronous final checkpoint. | |

**Agent choice:** Synchronous periodic and clean-shutdown checkpoints.

**Notes:** A single durability meaning keeps evidence and the documented crash
window precise. Single-flight coalescing, generation-bound stale completion, and
achieved-receipt retention extend Phase 134's existing effect protocol.

## Policy-aware staged recovery

| Option | Description | Selected |
| --- | --- | --- |
| Staged fresh-state replay with one authoritative install | Deterministic dependency ordering, partial independent recovery, final-membership classification, and rebuilt derived state without transient exposure. | ✓ |
| Streaming topological replay with final reconciliation | Reuse current mutation path and reconcile after order-dependent replay. | |
| All-or-nothing snapshot replay | Reject the entire snapshot if any record fails. | |

**Agent choice:** Staged fresh-state replay with one authoritative install.

**Notes:** Startup-only staged partial recovery best preserves valid independent
records while preventing replay-induced rolling-fee state, transient cache
exposure, and premature `recovered` classifications.

## the agent's Discretion

- Exact type/module names, periodic interval, deterministic tie-breaks, bounded
  rejected-snapshot diagnostics, and the smallest coordinator state machine.

## Deferred Ideas

- Runtime imports, configurable persistence strength, incremental journals,
  automatic destructive repair, receive-independent retries, broad operator
  rendering, public/default relay, and production claims.

***

*Discussion completed in one recommendation-engine pass.*
