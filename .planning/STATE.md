---
gsd_state_version: "1.0"
milestone: v2.2
milestone_name: Package Relay and Long-Lived Mempool Policy
status: executing
stopped_at: Completed 133.1-05-PLAN.md
last_updated: "2026-07-27T19:40:52.812Z"
last_activity: "2026-07-27"
progress:
  total_phases: 10
  completed_phases: 4
  total_plans: 36
  completed_plans: 35
  percent: 97
---

# Project State

## Project Reference

See: `.planning/PROJECT.md` (updated 2026-07-22 after starting milestone v2.2).

**Core value:** When a behavior is in scope, Open Bitcoin must behave like the pinned Knots baseline on the outside while staying simpler and safer on the inside.
**Current focus:** Phase 133.1 — Bright Builds Verification Baseline Cleanup

## Current Position

Milestone: v2.2 Package Relay and Long-Lived Mempool Policy
Phase: 133.1 (Bright Builds Verification Baseline Cleanup) — EXECUTING
Plan: 6 of 6
Status: Ready to execute
Last activity: 2026-07-27

Progress: [██████████] 100%

Next action: `/gsd-discuss-phase 134`

## Performance Metrics

**Current milestone:**

- Total plans completed: 51
- Average duration: 42 min
- Total execution time: 9h 4m

| Phase | Plans | Total | Avg/Plan |
| --- | ---: | ---: | ---: |
| 130–138 | 13 | 9h 4m | 42 min |
| 130 | 13 | - | - |
| 131 | 5 | - | - |
| 132 | 8 | - | - |
| 133 | 4 | - | - |

### Plan Execution History

| Plan | Duration | Tasks | Files |
| --- | --- | --- | --- |
| Phase 133 P01 | 1h 13m | 3 tasks | 16 files |
| Phase 133 P02 | 1h 10m | 3 tasks | 28 files |
| Phase 133 P03 | 1h 4m | 3 tasks | 20 files |
| Phase 133 P04 | 38min | 3 tasks | 14 files |
| Phase 133.1 P01 | 10min | 2 tasks | 48 files |
| Phase 133.1 P02 | 13min | 3 tasks | 62 files |
| Phase 133.1 P03 | 51min | 3 tasks | 263 files |
| Phase 133.1 P04 | 27min | 3 tasks | 137 files |
| Phase 133.1 P05 | 21min | 3 tasks | 21 files |

## Accumulated Context

### Roadmap Evolution

- Phase 133.1 inserted after Phase 133: Bright Builds Verification Baseline Cleanup (URGENT)

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
- [Phase 133]: Reject evidence accepts only Wtxid or typed package fingerprints; txid-only inventory requires an authoritative txid-to-wtxid mapping.
- [Phase 133]: Ordinary inventory consults hard and reconsiderable evidence, while orphan-parent requests bypass reconsiderable evidence and still honor hard rejects.
- [Phase 133]: Both reject evidence domains reset together immediately after successful authoritative chainstate connect or reorg mutation.
- [Phase 133]: Production tweak entropy is derived in the node shell with RandomState while network constructors retain fixed-tweak deterministic seams.
- [Phase 133]: Capture receipt provenance before request cleanup, deterministically unioning bounded txid/wtxid announcers while retaining the delivering peer.
- [Phase 133]: Retain one orphan body with a policy-bounded announcer set; late inventory changes ownership evidence only and never replaces the body or refreshes TTL.
- [Phase 133]: Use an opaque consume-only same-peer 1P1C candidate as the proof of provenance and bounded eligibility.
- [Phase 133]: Co-locate scheduler, orphanage, reject evidence, and disconnect mutation under PeerManager.
- [Phase 133]: Classify peer singletons through typed package reports and preserve ordinary RBF only for the exact typed one-member package-replacement shape.
- [Phase 133]: Apply only bounded orphan and reject-evidence feedback in Phase 133; defer package lifecycle projection to Phase 134.
- [Phase 133]: Claim only bounded opportunistic same-peer 1P1C assembly over ordinary transaction messages; broader package relay surfaces remain deferred.
- [Phase 133]: Guard the exact node-owned Phase 132 handoff and exhaustive feedback boundary with a filesystem-only checker and 22 independent mutations.
- [Phase 133]: Treat probabilistic reject evidence as suppression-only, with active-tip reset and no peer punishment.
- [Phase 133.1]: Anchor all cleanup comparisons to phase-start commit 3e35678a9e3d623aad27893f9594a8ded152a722.
- [Phase 133.1]: Compare Rust and Bun test behavior with sorted multisets plus independently parsed counts.
- [Phase 133.1]: Group contiguous Rust test leaves by behavior while keeping shared fixtures in thin test roots.
- [Phase 133.1]: Preserve moved tests original super:: resolution through private test-root imports rather than rewriting test bodies.
- [Phase 133.1]: Assign every new Wave A Rust path explicitly to the parity breadcrumb group that owned its source offender.
- [Phase 133.1]: Use semantic behavior and phase families while retaining shared fixtures in thin test-only roots.
- [Phase 133.1]: Expose moved shared helpers only with pub(super), using absolute crate paths where module depth changes super resolution.
- [Phase 133.1]: Assign each new child to the parity group of its nearest original source offender.
- [Phase 133.1]: Preserve each oversized TypeScript root as the stable entrypoint and move concerns into same-named directories.
- [Phase 133.1]: Use explicit child-source maps when parity checkers follow Rust sources decomposed by earlier plans.
- [Phase 133.1]: Compare exact XML-escaped JUnit title multisets plus independent test counts for TypeScript decomposition evidence.
- [Phase 133.1]: Preserve the live-smoke TypeScript and shell roots as stable entrypoints backed by same-named concern directories.
- [Phase 133.1]: Keep extracted networking inert until parsed explicit CLI opt-in and successful local preflight.
- [Phase 133.1]: Confine all hermetic live-smoke fixtures, reports, and cleanup to one guarded temporary root.

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

Last session: 2026-07-27T19:40:52.809Z
Stopped at: Completed 133.1-05-PLAN.md
Resume file: None
