# Phase 69: Tip Tracking and Stay-Current Operation - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution
> agents. Decisions are captured in CONTEXT.md - this log preserves the
> alternatives considered.

**Date:** 2026-06-11T15:13:14.807Z
**Phase:** 69-tip-tracking-and-stay-current-operation
**Mode:** Yolo
**Areas discussed:** Best-known tip evidence, Stay-current state model, Runtime
loop behavior, Operator surface boundaries, Verification posture

---

## Best-Known Tip Evidence

| Option | Description | Selected |
| --- | --- | --- |
| Typed peer-derived evidence | Derive source, height, hash, work, timestamp, freshness, and agreement from validated headers and peer outcomes. | yes |
| Renderer-specific text | Let each CLI/RPC/dashboard surface describe tip state independently. | |
| External trusted oracle | Ask an external service or trusted peer for current mainnet tip truth. | |

**User's choice:** Auto-selected typed peer-derived evidence.
**Notes:** This preserves the v1.6 first-party validation claim and avoids
centralized tip-oracle scope creep.

---

## Stay-Current State Model

| Option | Description | Selected |
| --- | --- | --- |
| Shared typed classification | Add shared states for initial catch-up, current-at-best-known-tip, stale-tip, recovering, and no-progress. | yes |
| Reuse progress_signal only | Keep the existing header/block/waiting signal as the only state machine. | |
| Surface-specific interpretation | Let each operator surface decide how to phrase current/stale/recovering state. | |

**User's choice:** Auto-selected shared typed classification.
**Notes:** The existing `SyncProgressSignal` remains useful low-level progress
evidence, but Phase 69 needs a stay-current truth contract that does not require
renderer-specific inference.

---

## Runtime Loop Behavior

| Option | Description | Selected |
| --- | --- | --- |
| Continue bounded wake cycles after catch-up | Keep `open-bitcoind` polling peers for fresh headers/blocks, validating and connecting new work when observed. | yes |
| Stop once caught up | Treat initial catch-up as the end of daemon sync work. | |
| Expand to production node operation | Add inbound serving, relay, or production full-node behavior. | |

**User's choice:** Auto-selected continued bounded wake cycles after catch-up.
**Notes:** This is the minimum behavior needed for TIP-03 while preserving the
bounded opt-in posture.

---

## Operator Surface Boundaries

| Option | Description | Selected |
| --- | --- | --- |
| Add truth without hiding counters | Keep Phase 68 progress counters visible and add tip/freshness/current-state meaning. | yes |
| Collapse to one human label | Replace detailed progress fields with a single status phrase. | |
| Broaden release claims | Phrase evidence as production-node or broad readiness. | |

**User's choice:** Auto-selected add truth without hiding counters.
**Notes:** Phase 69 should strengthen status semantics without weakening audit
evidence or crossing deferred release boundaries.

---

## Verification Posture

| Option | Description | Selected |
| --- | --- | --- |
| Deterministic tests and checker | Prove tip evidence, peer agreement, stale/current classifications, post-catch-up progress, restart persistence, and docs guardrails without public network. | yes |
| Public-network default checks | Add live mainnet stay-current checks to `bash scripts/verify.sh`. | |
| Documentation only | Describe stay-current behavior without deterministic code coverage. | |

**User's choice:** Auto-selected deterministic tests and checker.
**Notes:** Public-mainnet stay-current review remains opt-in UAT; default
verification stays hermetic and timing-stable.

---

## the agent's Discretion

- Exact helper/module placement for tip freshness and stay-current classification.
- Conservative freshness defaults, as long as docs and tests make them explicit.
- Plan split across status-domain types, runtime projection, daemon behavior, and
  docs/checker coverage.

## Deferred Ideas

- Phase 70: reorg, branch competition, peer rotation, and broader no-progress
  recovery.
- Phase 71: long-run resource bounds and restart/resume resource-pressure proof.
- Phase 72: cross-surface support evidence unification.
- Phase 73: opt-in public-mainnet stay-current UAT command expansion.
