---
gsd_state_version: 1.0
milestone: v2.2
milestone_name: Package Relay and Long-Lived Mempool Policy
status: executing
stopped_at: Phase 131 context gathered
last_updated: "2026-07-25T10:04:43.405Z"
last_activity: 2026-07-25
progress:
  total_phases: 9
  completed_phases: 2
  total_plans: 18
  completed_plans: 18
  percent: 100
---

# Project State

## Project Reference

See: `.planning/PROJECT.md` (updated 2026-07-22 after starting milestone v2.2).

**Core value:** When a behavior is in scope, Open Bitcoin must behave like the pinned Knots baseline on the outside while staying simpler and safer on the inside.
**Current focus:** Phase 131 — Rolling Fee, Expiry, and Descendant Eviction Core

## Current Position

Milestone: v2.2 Package Relay and Long-Lived Mempool Policy
Phase: 132
Plan: Not started
Status: Ready to execute
Last activity: 2026-07-25

Progress: [█░░░░░░░░░] 11%

Next action: `/gsd-discuss-phase 131` (or YOLO chain for Phase 131)

## Performance Metrics

**Current milestone:**

- Total plans completed: 31
- Average duration: 42 min
- Total execution time: 9h 4m

| Phase | Plans | Total | Avg/Plan |
| --- | ---: | ---: | ---: |
| 130–138 | 13 | 9h 4m | 42 min |
| 130 | 13 | - | - |
| 131 | 5 | - | - |

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
- [Phase 130]: Keep getmempoolinfo.bytes=vsize, usage=accounted memory, maxmempool=accounted capacity, and mempoolminfee=effective max(static, rolling).
- [Phase 130]: Serialize capacityenforcement as fixed legacy_vsize during Phase 130 without claiming accounted-capacity enforcement.
- [Phase 130]: Expose rollingmempoolfee, effectiveadmissionfee, and incrementalrelayfee as distinct exact fields so incremental never contaminates mempoolminfee.
- [Phase 130]: Register unique FEEP-01 through FEEP-05 ownership under v2-2-resource-time-fee-primitives with exact later-phase boundaries.
- [Phase 130]: Document intentional Rust-owned accounting difference from C++ allocator estimates while preserving Knots RPC meanings.
- [Phase 130]: Align documentation reconciliation with active v2.2 README truth instead of requiring /gsd-new-milestone in the root status block.
- [Phase 130]: Reuse the Phase 129 string[] failure-list contract with no separate result alias.
- [Phase 130]: Validate README freshness through three independent readTarget calls and dedicated stale-wording failures.
- [Phase 130]: Keep FEEP requirements Pending until Phase 130 VERIFICATION.md exists for milestone traceability.

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

Last session: 2026-07-24T22:35:28.240Z
Stopped at: Phase 131 context gathered
Resume file: .planning/phases/131-rolling-fee-expiry-and-descendant-eviction-core/131-CONTEXT.md
