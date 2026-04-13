---
phase: 05-mempool-and-node-policy
plan: 01
subsystem: mempool-foundation
tags: [mempool, policy, workspace, pure-core]
provides:
  - new pure-core open-bitcoin-mempool crate
  - explicit policy config and fee-rate types
  - admission and replacement APIs wired into open-bitcoin-core
affects: [mempool, node, parity, verification]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 05-2026-04-13T23-15-14
generated_at: 2026-04-13T23:37:42.108Z
duration: ongoing
completed: 2026-04-13
---

# Phase 5 Plan 01 Summary

Established the pure-core mempool foundation and wired it through the
workspace.

## Accomplishments

- Added `open-bitcoin-mempool` as a first-party pure-core crate under
  `packages/` and wired it into Cargo, Bazel, `open-bitcoin-core`, and the
  repo purity checks.
- Defined explicit `PolicyConfig`, `FeeRate`, `RbfPolicy`, `MempoolEntry`, and
  `AdmissionResult` types so policy state is visible instead of implicit.
- Implemented the first mempool engine entrypoint for transaction admission and
  targeted replacement against chainstate-derived prevout context.
