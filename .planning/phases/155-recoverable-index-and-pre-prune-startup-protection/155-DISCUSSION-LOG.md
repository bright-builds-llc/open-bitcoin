# Phase 155: Recoverable Index and Pre-Prune Startup Protection - Discussion Log

> Audit trail only. Decisions are captured in CONTEXT.md; downstream planning consumes that file.

**Date:** 2026-10-04
**Mode:** Yolo — recommended choices auto-selected under the invoked skill.

| Area | Recommended choice selected | Alternatives considered | Rationale |
| --- | --- | --- | --- |
| Storage compatibility | Additive versioned records in existing Fjall | New database, global schema reset | Preserve old datadirs and existing atomic batch substrate. |
| Record identity | Immutable hash-keyed rows plus separate active projection | Height-only rows, deleting displaced/ahead rows | Retain valid evidence without mixing branches. |
| Durable progress | Fence by recovered coins plus matching active metadata ancestry | Trust filter cursor, headers or counters | Records can survive ahead of recoverable chainstate. |
| Commit ordering | Records/checkpoint first, protection relaxation last or same atomic batch | Lock-first publication | Failures retain extra history rather than deleting required inputs. |
| Startup guard | Inside recovery before resumed prune deletion | Restore lock after manager construction | Existing initialize deletes before manager construction. |
| Unsafe recovery | Explicit refusal preserving prefix, payloads and intent | Reset index, skip heights, repair/download | Current exclusions forbid implicit mutation/acquisition. |
| Verification | Real Fjall faults/reopen and production runtime path | Memory-only or source-string checks | Concrete durable effects are the phase acceptance boundary. |

The agent may choose exact record layout, error types, bounds and focused module decomposition. Public activation, ordinary prune coordination, reorg runtime, RPC, P2P and operator consumers remain later phases.
