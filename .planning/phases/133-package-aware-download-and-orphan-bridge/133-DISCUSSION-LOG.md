# Phase 133: Package-Aware Download and Orphan Bridge - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-07-26
**Phase:** 133-package-aware-download-and-orphan-bridge
**Mode:** Yolo
**Areas discussed:** Bounded reconsiderable and reject evidence, Same-peer 1P1C candidate assembly and identity, Authoritative admission bridge and outcome projection

***

## Bounded reconsiderable and reject evidence

| Option | Description | Selected |
| --- | --- | --- |
| Two node-global rolling Bloom filters, matching pinned Knots | Separate fixed-memory hard-reject wtxids from reconsiderable wtxids and package fingerprints; rotate by insertion generation and reset on active-tip change. | ✓ |
| Two exact global generational stores | Preserve exact membership and deterministic eviction at the cost of higher adversarial heap overhead and parity drift. | |
| One quota-partitioned exact TTL store | Use exact time-based retention with per-class quotas, adding clock policy and cross-class interference risks. | |

**Agent-selected choice:** Two node-global rolling Bloom filters, matching pinned Knots.
**Notes:** Selected for exact baseline semantics, fixed memory, and cross-peer suppression. False positives are an intentional bounded-state tradeoff; peer identity and candidate bodies stay in the orphanage.

***

## Same-peer 1P1C candidate assembly and identity

| Option | Description | Selected |
| --- | --- | --- |
| Knots-style announcer-qualified, parent-triggered assembly | Pair a reconsiderable parent from peer `P` with the newest eligible orphan child announced by `P`, preserving ordered membership, aligned provenance, and the Phase 132 fingerprint. | ✓ |
| Strict body-delivery same-peer ownership | Require one peer to have delivered both bodies, simplifying attribution but rejecting baseline-valid fallback-announcer cases. | |
| Per-peer retained parent/child caches with eager assembly | Store both sides per peer for symmetric eager assembly, increasing memory, cleanup, and observable-behavior divergence. | |

**Agent-selected choice:** Knots-style announcer-qualified, parent-triggered assembly.
**Notes:** “Same peer” is announcer-qualified, not necessarily first body-delivery ownership. The shared orphan body is not duplicated per announcer, and the candidate remains exactly `[parent, child]`.

***

## Authoritative admission bridge and outcome projection

| Option | Description | Selected |
| --- | --- | --- |
| Network-owned candidate, node-owned refinement, minimal feedback | Keep bounded selection in networking, construct Phase 132 package types and call authoritative admission in the node bridge, then update only candidate-selection state. | ✓ |
| Network emits a fully refined `SubmissionPackage` | Make networking depend directly on mempool package vocabulary, simplifying the bridge but inverting current crate layering. | |
| Node-owned refinement plus full lifecycle projection now | Apply serving, fanout, compact, peer, persistence, and evidence effects immediately, absorbing substantial Phase 134 scope. | |

**Agent-selected choice:** Network-owned candidate, node-owned refinement, minimal feedback.
**Notes:** Phase 133 owns only feedback required for correct future candidate selection. The authoritative package report, fingerprint, and lifecycle delta stay intact for Phase 134 to project across all dependent caches.

## the agent's Discretion

- Exact Rust names, internal module split, rolling-filter hash derivation, and bounded announcer-index representation within the locked invariants.

## Deferred Ideas

- Full lifecycle projection across dependent caches — Phase 134.
- Parent-before-child fanout and transport receipts — Phase 136.
- Package RPC and sanitized operator evidence — Phase 137.
- General package wire relay and arbitrary multi-parent reconstruction — beyond v2.2.
