---
gsd_state_version: "1.0"
milestone: v2.1
milestone_name: Block Serving and Compact Block Relay Boundary
status: planning
stopped_at: Completed 128-04-PLAN.md
last_updated: "2026-07-20T10:04:16.346Z"
last_activity: 2026-07-20 -- Phase 128 production compact-announcement transport verified and complete
progress:
  total_phases: 20
  completed_phases: 19
  total_plans: 66
  completed_plans: 66
  percent: 100
---

# Project State

## Project Reference

See: `.planning/PROJECT.md` (updated 2026-07-20 after Phase 127 authoritative-state verification).

**Core value:** When a behavior is in scope, Open Bitcoin must behave like the pinned Knots baseline on the outside while staying simpler and safer on the inside.
**Current focus:** Plan Phase 129 integration guardrails and milestone reconciliation

## Current Position

Milestone: v2.1 Block Serving and Compact Block Relay Boundary
Phase: 129
Plan: Not started
Status: Ready to plan
Last activity: 2026-07-20 -- Phase 128 production compact-announcement transport verified and complete

v2.0 Transaction Relay and Mempool Participation Boundary shipped on 2026-07-03. The archived audit reports 32/32 requirements, 10/10 phases, 8/8 integration checks, 8/8 cross-phase flows, and no tracked tech debt.

Phases 110 through 128 are locally complete at 66/66 plans. Phase 128 completed production compact-announcement transport, aggregate guardrails, and parity evidence, promoting three additional requirements to 36/39 complete. Phase 129 remains to add final integration guardrails and reconcile the milestone before a fresh archival decision.

Next action: Run `/gsd-plan-phase 129`.

## Performance Metrics

| Plan | Duration | Tasks | Files |
| --- | ---: | ---: | ---: |
| Phase 126 P01 | 31m | 2 tasks | 12 files |
| Phase 126 P02 | 21m | 3 tasks | 11 files |
| Phase 126 P03 | 7m | 1 task | 3 files |
| Phase 126 P04 | 64m | 3 tasks | 10 files |

## Latest Milestone Archive

- Roadmap: `.planning/milestones/v2.0-ROADMAP.md`
- Requirements: `.planning/milestones/v2.0-REQUIREMENTS.md`
- Audit: `.planning/milestones/v2.0-MILESTONE-AUDIT.md`

## Recent Trend

- v1.6 completed explicit opt-in mainnet full-sync completion evidence.
- v1.7 hardened multi-day full-sync soak, bounded resources, recovery diagnosis, progress guarantees, and support-bundle forensics.
- v1.8 defined production-readiness claim gates, support/update/runbook/service policies, release-readiness evidence, and deterministic no-claim guardrails.
- v1.9 shipped opt-in inbound peer serving while keeping transaction relay, compact blocks, mempool propagation, production-funds wallet use, migration apply mode, packaging, hosted dashboard, GUI, public-network CI, and production full-node readiness deferred.
- Phase 98 canonical ownership remains archived for INB-01, INB-02, INB-03, INB-04, BOUND-06.
- v2.0 shipped bounded transaction relay and mempool participation through explicit activation, txid/wtxid inventory/download, orphan handling, mempool admission and durable recovery, relay serving/fanout, sanitized operator evidence, parity roots, UAT guidance, and deterministic no-claim guardrails.
- v2.1 completed 19 local phases after Phase 128 closed production compact transport; final integration and reconciliation remain.

## Decisions

Recent decisions are logged in `PROJECT.md`. The latest milestone-level decisions are:

- [v2.0]: Scope transaction relay and mempool participation to explicit activation, txid/wtxid inventory, bounded download/orphan handling, mempool lifecycle, relay serving/fanout, operator evidence, and release guardrails.
- [v2.0]: Keep compact block relay, bloom/filter serving, package relay, public relay defaults, public-network CI, production service operation, production full-node readiness, and production-funds wallet use deferred.
- [v2.0]: Keep public-network relay review opt-in and outside default deterministic verification unless a future phase explicitly changes that contract.
- [v2.0]: Keep Phase 106 as the original BOUND-01 through BOUND-05 release-boundary closeout checker, while Phase 107 and Phase 108 extension checkers provide supplemental runtime activation/download and durable recovery coverage.
- [Phase 113-compact-relay-negotiation-and-announcement-policy]: Low-bandwidth sendcmpct remains compact relay capability evidence but never authorizes direct compact block announcements.
- [Phase 113-compact-relay-negotiation-and-announcement-policy]: Adjacent relay, permission, inbound protection, and block-serving state cannot activate compact announcement policy by implication.
- [Phase 113-compact-relay-negotiation-and-announcement-policy]: Compact getdata stays in the suppressed/missing path; compact reconstruction and missing-transaction behavior remain deferred to later phases.
- [Phase 113-compact-relay-negotiation-and-announcement-policy]: No git commits were created because the parent wrapper reserves final git mutation for verification-first orchestration.
- [Phase 117-parity-traceability-uat-and-release-guardrails]: The canonical v2.1 claim is bounded, explicit, and default-off; package/filter/public-default/production surfaces remain deferred and public-network review remains optional UAT.
- [Phase 117-parity-traceability-uat-and-release-guardrails]: Five code-review warnings in the aggregate guardrails were fixed before final verification; the post-review repository contract passed.
- [Phase 118]: Builder lives in open-bitcoin-consensus beside short-ID helpers (D-03)
- [Phase 118]: Empty transactions return CodecError::CompactBlockEmpty; no unwrap/panic
- [Phase 118]: New announce_block_with_action API; legacy announce_block signature unchanged (D-02)
- [Phase 118]: announce_block delegates to Headers/Inv actions for DRY without compact path
- [Phase 118]: CMP-05 left Pending until Plan 03 closes evidence-after-emit
- [Phase 118]: Evidence recorded after emission from actual message (D-05)
- [Phase 118]: Hash-derived deterministic nonce: first 8 LE bytes of block hash
- [Phase 118]: CMP-05 satisfied by Plan 03 closing the runtime seam
- [Phase 119]: PeerManager forwarder walks compact_download_states by wtxid only; no mempool coupling
- [Phase 119]: Empty-facts CompactBlock dispatch kept for tests; production inject via ManagedPeerNetwork shell
- [Phase 119]: CompactExtraTxnBuffer uses virtual size for Knots-aligned byte budget approximation
- [Phase 119]: Live CompactBlock intercepts in receive_* call handle_compact_block_download with shell-built mempool+extra facts
- [Phase 119]: Admission orphan/reject/replaced-victim bodies feed CompactExtraTxnBuffer; admitted Replaced wtxid is not an extras-removal feed
- [Phase 119]: Forward removal.wtxid before TxServing demotion on connected-block lifecycle (D-07)
- [Phase 119]: Evicted/Expired and replaced victims forward wtxid; never hook admitted Replaced wtxid as removal
- [Phase 119]: Explicit duplicate short-id typed-failure test on injected receive path (D-09.2)
- [Phase 120]: PeerManager expire returns peer-scoped Vec<(PeerId, PeerAction)> like TX expire
- [Phase 120]: ManagedPeerNetwork compact expire keeps PeerAction::Send; never copies TX TransactionRelay filter
- [Phase 120]: receive_message returns ManagedSyncMessageResult so other-peer timeout GetData is preserved
- [Phase 120]: Timeout tick piggybacks on receive_* message timestamps; no Tokio timer
- [Phase 121]: One shared block-relay provider for metrics and structured logs
- [Phase 121]: Available-gate block_relay_metric_samples at call site (never on Unavailable)
- [Phase 121]: Production block-relay metrics/logs use ManagedRpcContext evidence with activation outer Available gate
- [Phase 121]: No sync-disabled twin block-relay metrics worker in Phase 121
- [Phase 126]: Factless generic CompactBlock dispatch is a peer-neutral adapter routing error before reconstruction.
- [Phase 126]: Compact announcement entropy is acquired lazily only after compact selection, with peer-safe fallback on failure.
- [Phase 126]: Compact provenance and achieved-effect evidence follow only an actually emitted CompactBlock.
- [Phase 126]: Guard compact receive facts, nonce entropy, achieved-effect evidence, dependency agreement, and parity roots with a fixed local corpus.
- [Phase 126]: Run the Phase 126 guard after Phase 124 plus active traceability and before the unchanged Phase 117 final no-claim gate.
- [Phase 126]: Model candidate, verified pre-promotion, promoted pre-summary, and archive-ready as the only legal Phase 126 closeout states.
- [Phase 126]: Keep executor candidate evidence distinct from independent gsd-verifier ownership.
- [Phase 126]: Promote exactly the six Phase 126 requirements only after the independent verifier and lifecycle gates pass.
- [Milestone gap planning]: Use Phase 127 for authoritative runtime state, Phase 128 for production compact transport, and Phase 129 for integration guardrails plus final reconciliation.
- [Phase 128]: Collapse multi-block reconciliation to one final DurableTipAdvanced event after persistence.
- [Phase 128]: Share bounded volatile outboxes across durable sync and inbound sessions without authority locks crossing I/O.
- [Phase 128]: Credit each successful FIFO prefix immediately and drop failed or unsent suffixes without implicit retry.
- [Phase 128]: Guard the production compact-announcement path from bilateral negotiation through durable trigger, real writes, consuming receipts, and fixed observability.
- [Phase 128]: Preserve default-off and deferred public/production claims while recording production composition evidence.
- [Phase 128]: Model Phase 128 Plan 04 execution and completed routing to Phase 129 as distinct fail-closed lifecycle states.
- [Phase 128]: Extract focused RPC connection and runtime-control modules to preserve the 628-line production source limit.

## Pending Todos

- Plan and execute Phase 129 before rerunning the v2.1 milestone audit.
- Keep historical `.planning/phases/` directories tracked because repo verifier scripts still depend on selected phase evidence.
- Keep repo-local Cargo and Bazel command forms in UAT guidance:
  - `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- ...`
  - `bazel run //packages/open-bitcoin-cli:open_bitcoin -- ...`
- Keep public-network relay exposure opt-in and outside default deterministic verification unless a future phase explicitly changes that contract.

## Blockers/Concerns

- No active milestone blockers are recorded.
- Default local verification must remain deterministic; public-network relay review should be opt-in UAT evidence unless deliberately changed.
- Existing outbound sync, inbound serving, full-sync, soak, support-bundle, and release-boundary behavior must not regress when future milestones expand relay or production scope.
- Existing transaction relay, mempool, inbound, sync, support-bundle, and release-boundary behavior must not regress in future work built on v2.1 block-serving and compact-block relay scope.

### Quick Tasks Completed

| # | Description | Date | Commit | Directory |
| --- | --- | --- | --- | --- |
| 260719-bbh | Accept the approved v2.1 post-audit gap-planning state in closeout verification | 2026-07-19 | `e02bae71` | [260719-bbh](./quick/260719-bbh-teach-the-phase-124-closeout-checker-to-/) |
| Phase 128 P03 | 51m | 3 tasks | 9 files |
| Phase 128 P04 | 57min | 3 tasks | 25 files |

## Session Continuity

Last session: 2026-07-20T06:48:12.890Z
Stopped at: Completed 128-04-PLAN.md
Resume file: None
