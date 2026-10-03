# Phase 153: Automatic Prune Retention Integration - Discussion Log

> Audit trail only. Decisions are authoritative in CONTEXT.md.

**Date:** 2026-10-02 CDT
**Mode:** Yolo; recommended choices auto-accepted by the invoked workflow.

| Area | Recommended and selected | Alternatives considered | Rationale |
| --- | --- | --- | --- |
| Retained usage | Actual encoded block/undo value bytes, all retained usage, active candidates | Filesystem allocation; decoded estimates | Measures the keys paired unlink can remove without misrepresenting physical disk reclamation. |
| Retention cadence | Existing Periodic/Always durable owner, including no coins write due | New timer; manual-only trigger | Closes the missing production flow with one mutation owner. |
| Lock safety | Serialize current lock updates with plan/apply | Unguarded snapshot | Independent cloned-store RPC mutation otherwise creates a stale-lock race. |
| Offline prune activation | Explicit prune plus datadir selects durable lifecycle | Require network activation | Operator prune configuration must function without changing network defaults. |
| Failure handling | Fail closed; clean caches from committed deletion receipts | Ignore accounting errors; clear cache only on full flush success | Preserves durable truth through partial failures and reopen. |
| Proof | Real storage above legal target, gate/error/reopen regressions and native verification | Synthetic byte override; source-only checks | Runtime behavior closes INT-01. |

The agent chooses accounting API, bounded scan strategy, synchronization
and hermetic fixture design. No todos matched. Existing keep-window,
prune-after, serving and Phase 152 wallet contracts are carried forward.
All commits are deferred to the wrapper's clean phase gate.
