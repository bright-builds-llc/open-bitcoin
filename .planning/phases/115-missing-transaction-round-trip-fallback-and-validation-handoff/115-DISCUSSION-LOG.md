# Phase 115: Missing Transaction Round Trip, Fallback, and Validation Handoff - Discussion Log

> **Audit trail only.** Decisions are captured in CONTEXT.md.

**Date:** 2026-07-06
**Phase:** 115-Missing Transaction Round Trip, Fallback, and Validation Handoff
**Mode:** Yolo
**Areas discussed:** Missing transaction scheduling, blocktxn response handling, validation handoff and fallback, volatile state cleanup

---

## Missing Transaction Scheduling

| Option | Description | Selected |
|--------|-------------|----------|
| Pure scheduler with typed actions (recommended) | Keep getblocktxn scheduling in functional core; shell sends wire messages | ✓ |
| Inline peer manager scheduling | Faster but blurs policy boundaries | |
| Defer to node shell | Violates Phase 115 boundary | |

**User's choice:** Pure scheduler with typed actions
**Notes:** Differential index encoding reuses Phase 112 helpers.

## BlockTxn Response Handling

| Option | Description | Selected |
|--------|-------------|----------|
| Strict in-flight matching (recommended) | Match peer, block hash, and expected missing indexes | ✓ |
| Best-effort fill | Risks cross-block contamination | |

**User's choice:** Strict in-flight matching with GOV-02 misbehavior outcomes

## Validation Handoff And Fallback

| Option | Description | Selected |
|--------|-------------|----------|
| ReceivedBlock on fill success (recommended) | Reuse existing validation/connect path | ✓ |
| New validation entrypoint | Duplicates chain integration | |

**User's choice:** `PeerAction::ReceivedBlock` plus typed full-block fallback/suppression

## Volatile State Cleanup

| Option | Description | Selected |
|--------|-------------|----------|
| Fixed-label cleanup matrix (recommended) | Disconnect, timeout, reorg, restart, block connect | ✓ |
| Lazy expiry only | Insufficient for GOV-03 | |

**User's choice:** Fixed-label cleanup matrix without chainstate mutation

## Claude's Discretion

Module layout, exact enum names, and test fixture structure.

## Deferred Ideas

Operator evidence rollout, parity/UAT closeout, package relay, filter serving, public defaults, production readiness.
