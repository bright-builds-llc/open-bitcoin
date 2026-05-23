---
phase: 39
phase_name: "Operator Sync Observability and Control"
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: "39-2026-05-02T11-46-08"
generated_at: "2026-05-02T12:22:11Z"
---

# Discussion Log 39

## Yolo Decisions

- Durable sync state should be the single source of truth for operator-facing mainnet sync lifecycle, lag, error, recovery, and peer detail.
- Operator control should land as an explicit `open-bitcoin sync` command surface with pause and resume, not as undocumented metadata mutation.
- RPC blockchain-info truth should consume durable sync state when present so header-versus-block progress stays honest during IBD.
- Status and dashboard should remain projections of the shared snapshot contract rather than inventing a daemon-specific parallel model.
- This phase should also reconcile stale planning metadata for completed Phases 37 and 38 so future phase routing is correct.

## Deferred

- Live-mainnet smoke evidence and closeout docs stay in Phase 40.
- Dashboard action-bar pause/resume affordances are optional follow-up depth after the CLI control path.
