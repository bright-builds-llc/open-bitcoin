# Phase 156: Index-Owned Manual and Automatic Prune Coordination - Discussion Log

> Audit trail only. Downstream agents consume CONTEXT.md.

**Date:** 2026-10-04
**Mode:** Yolo
**Lifecycle:** 156-2026-10-04T20-28-36

Recommended answers were automatically selected under the user-invoked yolo workflow.

| Area                 | Selected answer and rationale                                                                                            | Alternatives considered                                                                                  |
| -------------------- | ------------------------------------------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------------------------------- |
| Reserved ownership   | Guard authoritative CRUD and concrete store replacement/deletion; all callers must preserve owned protection.            | RPC-only checks leave low-level bypasses; ordinary mutable named locks cannot prove ownership.           |
| Apply-time retention | Reload durable protection under actual mutation synchronization and invalidate stale automatic decisions.                | Planner-only snapshots can delete after protection changes; periodic throttling can hide a changed lock. |
| Protection progress  | Recoverable immutable records and coins/metadata-fenced checkpoint precede relaxation.                                   | Filter height or network tip alone can advance beyond recoverable state.                                 |
| Disable/re-enable    | Invalidate work before release and install protection before re-enable work. Preserve saved data.                        | Clearing a named lock alone leaves stale work races; deleting prefix/history exceeds scope.              |
| Verification         | Real-store/authority manual and ordinary automatic tests plus faults/reopen, source review and full native verification. | Memory-only or string-only proof misses concrete deletion/publication ordering.                          |

## Discretion and deferred scope

Researcher/planner choose small internal types and test seams. Public activation, scheduling, RPC, peer serving and production claims remain later/deferred scope. All decisions are in CONTEXT.md.
