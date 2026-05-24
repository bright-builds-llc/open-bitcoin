---
gsd_state_version: 1.0
milestone: v1.3
milestone_name: Public Mainnet Sync Proof and Node Hardening
status: defining-requirements
stopped_at: v1.3 Public Mainnet Sync Proof and Node Hardening started; defining requirements
last_updated: "2026-05-24T00:51:00-05:00"
last_activity: 2026-05-24 -- Milestone v1.3 started
progress:
  total_phases: 0
  completed_phases: 0
  total_plans: 0
  completed_plans: 0
  percent: 0
---

# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-05-24)

**Core value:** When a behavior is in scope, Open Bitcoin must behave like the pinned Knots baseline on the outside while staying simpler and safer on the inside.
**Current focus:** v1.3 Public Mainnet Sync Proof and Node Hardening

## Current Position

Phase: Not started (defining requirements)
Plan: -
Status: Defining requirements
Last activity: 2026-05-24 -- Milestone v1.3 started

Progress: requirements definition in progress

## Archive Layout

- v1.0 raw phase execution history remains in `.planning/phases/`.
- v1.1 raw phase execution history is archived under `.planning/milestones/v1.1-phases/`.
- v1.2 raw phase execution history is archived under `.planning/milestones/v1.2-phases/`.
- Completed milestone roadmap and requirements archives live under `.planning/milestones/`.
- The v1.3 `/gsd-new-milestone` run should create fresh active requirements and roadmap scope.

## Accumulated Context

### Decisions

Decisions are logged in PROJECT.md Key Decisions table.
Recent decisions affecting next work:

- [v1.0]: Use Bitcoin Knots `29.3.knots20260210` as the pinned behavioral baseline.
- [v1.0]: Keep the initial milestone headless and defer GUI work.
- [v1.0]: Use functional core / imperative shell boundaries with explicit I/O adapters.
- [v1.1]: Build a terminal dashboard and rich status output before any desktop GUI work.
- [v1.1]: Treat migration from Bitcoin Core or Bitcoin Knots as explicit, dry-run-first, and backup-aware.
- [v1.2]: Scope full mainnet network syncing to opt-in `open-bitcoind` initial block download with validated headers/blocks, durable restart/resume, and operator observability before broader P2P, wallet, or production-node claims.
- [v1.2]: Phase 41 found no `needs-phase` security follow-up before archive; future production surfaces still need fresh threat models when scoped.

### Roadmap Evolution

- v1.0 Headless Parity archived on 2026-04-26.
- v1.1 Operator Runtime and Real-Network Sync archived on 2026-04-30.
- v1.2 Full Mainnet Network Syncing archived on 2026-05-23.
- v1.3 Public Mainnet Sync Proof and Node Hardening started on 2026-05-24.

### Pending Todos

- 0 pending.

### Blockers/Concerns

- Automatic or destructive migration remains out of scope; later phases must preserve the current dry-run-first safety boundary until an apply-mode design is explicitly planned.
- Public-network testing must stay opt-in unless a future milestone explicitly changes the verification contract.
- Future production-node, production-funds wallet, inbound-serving, address-relay, compact-block, mempool-relay, and packaged-service claims need fresh scope and threat models.

## Session Continuity

Last session: 2026-05-24
Stopped at: defining v1.3 requirements
Resume file: .planning/REQUIREMENTS.md
