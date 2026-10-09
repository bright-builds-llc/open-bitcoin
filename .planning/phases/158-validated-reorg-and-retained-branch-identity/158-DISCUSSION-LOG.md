# Phase 158: Validated Reorg and Retained Branch Identity - Discussion Log

> Audit trail only. Decisions are captured in CONTEXT.md; this log is not downstream agent input.

**Date:** 2026-10-07 CDT
**Mode:** Yolo

## Automatically selected recommendations

| Area | Recommended choice | Alternative considered | Rationale |
| --- | --- | --- | --- |
| Branch orchestration | Extend serialized staged reorg and ordered owner | Invalidate and require reopen | Ordinary reorg must resume autonomously while distinguishing preview, acceptance and durable fencing. |
| Retention identity | Preserve immutable hash records; replace active projection | Delete displaced suffix | CFIX-03 requires retained displaced lookup, including after source prune and reopen. |
| Missing inputs | Preflight required body/undo; refuse conservatively | Accept then pause index | Missing reorg history must not create a partial successful branch transition. |
| Regression evidence | Continuous validated spend fork with actual Fjall faults/reopen | Sparse or memory-only fixtures | Roadmap success requires genuine historical facts and durable branch identity. |

All recommended choices were selected under the user's yolo authorization. Technical decomposition and bounded transition design are delegated to research/planning. No deferred todo was folded.
