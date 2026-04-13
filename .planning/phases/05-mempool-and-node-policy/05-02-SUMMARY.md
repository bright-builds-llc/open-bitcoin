---
phase: 05-mempool-and-node-policy
plan: 02
subsystem: mempool-accounting-and-eviction
tags: [mempool, ancestors, descendants, eviction, fees]
provides:
  - deterministic parent/child and ancestor/descendant metrics
  - explicit limit enforcement for ancestor and descendant policy
  - size-limit trimming via descendant-score ordering
affects: [mempool, networking, parity]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 05-2026-04-13T23-15-14
generated_at: 2026-04-13T23:37:42.108Z
duration: ongoing
completed: 2026-04-13
---

# Phase 5 Plan 02 Summary

Turned the admission engine into a deterministic policy state machine.

## Accomplishments

- Added explicit parent/child relationships and aggregate ancestor/descendant
  counts, virtual sizes, and fee totals to the public entry state.
- Enforced ancestor and descendant count or virtual-size limits through the
  recomputed relationship graph rather than hidden caches.
- Implemented size-limit trimming that removes the weakest descendant-score
  package and reports the evicted txids explicitly.
