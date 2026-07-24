---
gsd_state_version: 1.0
milestone: v2.2
milestone_name: Package Relay and Long-Lived Mempool Policy
status: executing
stopped_at: Completed 130-11-PLAN.md
last_updated: "2026-07-24T04:01:25.793Z"
last_activity: 2026-07-24
progress:
  total_phases: 9
  completed_phases: 0
  total_plans: 13
  completed_plans: 10
  percent: 77
---

# Project State

## Project Reference

See: `.planning/PROJECT.md` (updated 2026-07-22 after starting milestone v2.2).

**Core value:** When a behavior is in scope, Open Bitcoin must behave like the pinned Knots baseline on the outside while staying simpler and safer on the inside.
**Current focus:** Phase 130 — Resource, Time, and Fee Primitives

## Current Position

Milestone: v2.2 Package Relay and Long-Lived Mempool Policy
Phase: 130 (Resource, Time, and Fee Primitives) — EXECUTING
Plan: 11 of 13 complete; next incomplete is 09
Status: Ready to execute
Last activity: 2026-07-24 -- Completed Plan 130-11 local RPC admission timing

Progress: [████████░░] 77%

Next action: Execute Plan 130-09

## Performance Metrics

**Current milestone:**

- Total plans completed: 10
- Average duration: 42 min
- Total execution time: 6h 58m

| Phase | Plans | Total | Avg/Plan |
| --- | ---: | ---: | ---: |
| 130–138 | 10 | 6h 58m | 42 min |

## Accumulated Context

### Decisions

- [v2.2 milestone]: Initialized the new milestone through `/gsd-new-milestone` after the archived v2.1 closeout.
- [v2.2 roadmap]: Use the research-backed nine-phase dependency order across Phases 130–138 at fine granularity.
- [v2.2 roadmap]: Assign PPKG-04 to Phase 136, where parent-before-child package fanout becomes an achieved transport behavior after the peer bridge and lifecycle authority exist.
- [v2.2 roadmap]: Keep package handling to local package APIs and bounded same-peer 1P1C assembly over ordinary transaction messages; add no general package wire protocol.
- [v2.2 roadmap]: Persist canonical entries, acceptance times, and surviving local unbroadcast membership, but rebuild derived state and reset the rolling fee on restart.
- [v2.2 roadmap]: Keep default verification deterministic and hermetic; public/default/production relay and guaranteed-propagation claims remain deferred.
- [Phase 130]: Use deterministic Rust-owned logical mempool accounting rather than C++ allocator estimates.
- [Phase 130]: Keep Phase 130 trimming exclusively on legacy vsize while reporting distinct accounted usage and capacity.
- [Phase 130]: Map resource arithmetic failures to MempoolError::InternalInvariant at mutation boundaries.
- [Phase 130]: Keep FeeRate role-neutral for wallet arithmetic while requiring semantic wrappers at mempool policy boundaries.
- [Phase 130]: Initialize the rolling floor to zero and derive effective admission from static and rolling values at decision and summary boundaries.
- [Phase 130]: Keep package member-static and eligible aggregate-rolling obligations independent without a generic exception switch.
- [Phase 130]: Classify missing legacy metadata only as LegacyUnknown, RecoveryUnknown, and NotRequested; never infer local origin or current time.
- [Phase 130]: Require local origin, requested relay intent, and current authoritative membership together for retry eligibility.
- [Phase 130]: No-time local outcome adapters are removed; wallet AdmissionResult no-time path remains deprecated separately.
- [Phase 130]: Keep MempoolOutcome as attempt vocabulary and MempoolLifecycleDelta as committed fact vocabulary.
- [Phase 130]: Resolve retry clears with LifecycleRemoval > TransportWritten > EligibleServe precedence.
- [Phase 130]: Keep removal cause independent from direct-versus-descendant role.
- [Phase 130]: Peer admission uses exact receive or reconsideration time with Peer and NotRequested metadata.
- [Phase 130]: Local RPC admission now samples checked shell time with Local origin and activation-resolved relay intent.
- [Phase 130]: Bridge-owned admission cache effects consume lifecycle delta cause, role, identities, and final membership.
- [Phase 130]: Model only the injected variable retry delay in Phase 130; Phase 136 owns scheduling, fanout, receipts, and clearing.
- [Phase 130]: Require fallible 0-to-300-second jitter construction before creating a retry decision context.
- [Phase 130]: Use requested relay intent only for local relay and serving fixtures; non-relay admission setup remains explicitly not requested.
- [Phase 130]: Deterministic fixture time remains authoritative in tests; live RPC clock sampling is owned by Plan 130-11 and is complete.
- [Phase 130]: Use stored-block receive time and connected height while direct local blocks use explicit header time and connected height.
- [Phase 130]: Use one explicit reorg operation time for replacement-block cleanup and disconnected transaction reacceptance.
- [Phase 130]: Apply every reorg admission attempt through its semantic lifecycle delta without expanding Phase 134 cross-cache scope.
- [Phase 130]: Keep SchemaVersion::CURRENT unchanged and encode metadata as three optional mempool-record fields.
- [Phase 130]: All-absent decodes to LegacyUnknown, RecoveryUnknown, and NotRequested; any partial set is StorageError::Corruption in Mempool.
- [Phase 130]: Known capture and recovery pass metadata through AdmissionContext::recovery without substituting restart time or local origin.
- [Phase 130]: Sample SystemTime only in sendrawtransaction with checked conversion; never unwrap_or(0).
- [Phase 130]: Resolve RelayIntent::Requested from relay activation enabled; otherwise NotRequested.
- [Phase 130]: Migrate the final RPC caller and delete both no-time outcome adapters in one commit.

### Pending Todos

- Keep historical `.planning/phases/` directories tracked because repository verifiers consume selected evidence.
- Keep repo-local Cargo and Bazel command forms in UAT guidance.
- Preserve existing explicit relay activation and public-network opt-in boundaries.

### Blockers/Concerns

- Phase 131 planning must define accounted-memory enforcement and parity tolerance against the Plan 130 ledger.
- Phase 132 planning must confirm scoped package RBF, TRUC, and ephemeral-dust prerequisites or narrow unsupported outcomes explicitly.
- Phase 135 planning must choose mempool-local snapshot compatibility, checkpoint cadence/strength, and the advertised crash-loss window.
- Phase 136 planning must specify the exact eligible-serve or successful-write receipt that clears unbroadcast membership.

## Latest Milestone Archive

- Roadmap: `.planning/milestones/v2.1-ROADMAP.md`
- Requirements: `.planning/milestones/v2.1-REQUIREMENTS.md`
- Audit: `.planning/milestones/v2.1-MILESTONE-AUDIT.md`

## Session Continuity

Last session: 2026-07-24T04:01:25.789Z
Stopped at: Completed 130-11-PLAN.md
Resume file: None
