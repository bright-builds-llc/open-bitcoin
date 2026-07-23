# Phase 130: Resource, Time, and Fee Primitives - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md; this log preserves the alternatives considered.

**Date:** 2026-07-23
**Phase:** 130-resource-time-and-fee-primitives
**Mode:** Yolo
**Areas discussed:** Resource accounting, fee-floor vocabulary, entry metadata and explicit inputs, typed lifecycle outcomes

***

## Resource Accounting

| Option | Description | Selected |
| --- | --- | :---: |
| Typed Rust-owned accounting ledger | Distinct typed vsize, accounted-memory, and capacity values backed by a deterministic documented estimator, cached aggregate, and recomputation oracle. | ✓ |
| Rust container-capacity estimator | Estimate physical Rust allocations from container capacities and reserved storage. |  |
| Projection-time recomputation | Compute usage only when producing node or RPC evidence. |  |

**User's choice:** Automatically selected the recommended typed Rust-owned accounting ledger.
**Notes:** This gives Phase 131 a deterministic primitive without moving enforcement, trimming, or parity-tolerance work into Phase 130.

***

## Fee-Floor Vocabulary

| Option | Description | Selected |
| --- | --- | :---: |
| Named shared `FeeRate` fields | Use descriptive field names while retaining one interchangeable fee-rate type. |  |
| Semantic role newtypes plus applicability contract | Give each fee role compile-time identity and state explicitly which individual or package fee basis may satisfy each floor. | ✓ |
| Generic tagged fee-constraint evaluator | Represent all fee rules through one extensible evaluator. |  |

**User's choice:** Automatically selected semantic fee-role newtypes plus an explicit applicability contract.
**Notes:** Package aggregates may satisfy the rolling floor but not bypass the ordinary static relay floor; incremental relay fee remains a replacement and pressure-bump input.

***

## Entry Metadata and Explicit Inputs

| Option | Description | Selected |
| --- | --- | :---: |
| Canonical metadata plus operation-specific contexts | Store acceptance/provenance metadata on entries and pass narrow explicit contexts to each pure operation. | ✓ |
| Canonical metadata plus one shared `PolicyContext` | Pass one broad structure containing every possible time, block, occupancy, and jitter input. |  |
| Effect traits plus side metadata maps | Inject clock/random traits and keep entry metadata in separate indexes. |  |

**User's choice:** Automatically selected canonical entry metadata plus operation-specific input contexts.
**Notes:** Shell adapters sample time and randomness. Recovery preserves original acceptance time and must not infer missing origin.

***

## Typed Lifecycle Outcomes

| Option | Description | Selected |
| --- | --- | :---: |
| Canonical semantic `MempoolLifecycleDelta` | Separate committed consequences from attempts, record typed reasons and final membership, and expose stable evidence labels. | ✓ |
| Enrich existing `MempoolOutcome` | Add fields and variants to the current admission-oriented enum. |  |
| Defer canonical delta to Phase 134 | Add only narrow reason types now and postpone the authoritative delta. |  |

**User's choice:** Automatically selected a canonical semantic lifecycle delta separate from attempt results.
**Notes:** Phase 130 owns cache-agnostic facts. Phase 134 projects those facts across caches through `ManagedNetworkHandle` without reclassification.

## Claude's Discretion

- Exact type and module names.
- Deterministic accounted-memory formula components and cache layout.
- Narrow operation-context structure.
- Lifecycle-delta collection and deterministic ordering representation.

## Deferred Ideas

- Pressure enforcement and rolling-fee mechanics — Phase 131.
- Package-policy execution — Phase 132.
- Complete cross-cache projection — Phase 134.
- Durable checkpoint implementation — Phase 135.
- Retry scheduling and transport receipt handling — Phase 136.
