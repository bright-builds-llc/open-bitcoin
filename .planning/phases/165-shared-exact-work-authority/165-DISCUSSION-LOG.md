# Phase 165: Shared Exact Work Authority - Discussion Log

> Audit trail only. Decisions are captured in CONTEXT.md; this file is not downstream planning input.

**Date:** 2026-10-14
**Mode:** Yolo; all recommendations auto-selected in one pass.
**Areas:** Exact arithmetic, header and candidate ordering, legacy compatibility.

## Exact arithmetic

| Option                                   | Pros                                               | Cons                                                 | Complexity                    | Selection |
| ---------------------------------------- | -------------------------------------------------- | ---------------------------------------------------- | ----------------------------- | --------- |
| Extend existing ProofWork                | Reuses first-party exact arithmetic; no dependency | Requires lossless boundaries and separate provenance | Consensus helpers and callers | Selected  |
| Extract Uint256 plus proof/work wrappers | More role separation                               | Broader conversion refactor                          | New types and consumers       |           |

**Auto-selected recommendation:** Existing ProofWork with pinned modulo accumulation, compact-target normalization and independent full-width vectors. No new overflow refusal. Exact numbers alone do not prove chain validity.

## Header and connected selection

| Option                                                | Pros                                              | Cons                                                  | Complexity                     | Selection |
| ----------------------------------------------------- | ------------------------------------------------- | ----------------------------------------------------- | ------------------------------ | --------- |
| Shared pure derivation with managed eligibility owner | Reuses staged validation and serialized lifecycle | Must establish parent provenance and body eligibility | Header, chainstate, node seams | Selected  |
| Private accepted-header capability                    | Strong explicit bindings                          | Extra lifecycle/interface changes                     | Capability plus all consumers  |           |

**Auto-selected recommendation:** Strictly greater header work; connected eligibility sequence reflects body plus ancestor linkage. Enforce preference before connect/reorg mutation and exercise body-ready side branches. Research may use existing restricted receipts where they simplify this design. Truly ambiguous recovered ties remain bounded, without claimed pointer/hash parity.

## Legacy compatibility

| Option                                                 | Pros                                             | Cons                                          | Complexity                         | Selection   |
| ------------------------------------------------------ | ------------------------------------------------ | --------------------------------------------- | ---------------------------------- | ----------- |
| Extend filesystem-only compatibility refusal           | Preserves closed bytes before recovery/migration | Actual stored work remains unavailable        | Refusal types, routes and controls | Selected    |
| Add pure legacy snapshot classification where required | Keeps legacy numbers explicitly separate         | Supplied bytes do not prove datadir authority | Codec and fixture adjustments      | Conditional |

**Auto-selected recommendation:** Retain the read-only ordinary gate and explicitly refuse unprovable work. Add legacy codec classification only if exact-work migration requires it to preserve fixtures; never reinterpret u128 as verified authority. Exact durable reopen acceptance belongs to Phase 166.

## Advisor evidence

Three read-only GSD advisor researchers checked pinned arithmetic, separate equal-work ordering, existing caller-work and best-header-only seams, and non-opening compatibility constraints. Their source findings are captured as full repository paths in CONTEXT.md. No pending todos matched; no new capability was folded into scope.

## Deferred ideas

Canonical durable bootstrap/recovery, contextual history consensus and actual-daemon differential qualification retain Phases 166–168 ownership. Nested commits wait for the wrapper's clean final gate.
