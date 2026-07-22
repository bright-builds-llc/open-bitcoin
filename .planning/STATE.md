---
gsd_state_version: "1.0"
milestone: v2.2
milestone_name: Package Relay and Long-Lived Mempool Policy
status: Defining requirements
stopped_at: Defining milestone v2.2 requirements
last_updated: "2026-07-22T15:30:00Z"
last_activity: "2026-07-22"
progress:
  total_phases: 0
  completed_phases: 0
  total_plans: 0
  completed_plans: 0
  percent: 0
---

# Project State

## Project Reference

See: `.planning/PROJECT.md` (updated 2026-07-22 after starting milestone v2.2).

**Core value:** When a behavior is in scope, Open Bitcoin must behave like the pinned Knots baseline on the outside while staying simpler and safer on the inside.
**Current focus:** Defining v2.2 package relay and long-lived mempool policy requirements

## Current Position

Milestone: v2.2 Package Relay and Long-Lived Mempool Policy
Phase: Not started (defining requirements)
Plan: —
Status: Defining requirements
Last activity: 2026-07-22

The milestone was initialized through `/gsd-new-milestone` and will close explicit gaps around package admission and relay, rolling minimum-fee behavior, periodic rebroadcast, and sustained mempool pressure by reusing v2.0 mempool and relay foundations plus v2.1 peer transport and observability.

Next action: Define v2.2 requirements and create the roadmap, continuing phase numbering after Phase 129.

## Performance Metrics

| Plan | Duration | Tasks | Files |
| --- | ---: | ---: | ---: |
| Phase 126 P01 | 31m | 2 tasks | 12 files |
| Phase 126 P02 | 21m | 3 tasks | 11 files |
| Phase 126 P03 | 7m | 1 task | 3 files |
| Phase 126 P04 | 64m | 3 tasks | 10 files |

## Latest Milestone Archive

- Roadmap: `.planning/milestones/v2.1-ROADMAP.md`
- Requirements: `.planning/milestones/v2.1-REQUIREMENTS.md`
- Audit: `.planning/milestones/v2.1-MILESTONE-AUDIT.md`

## Recent Trend

- v2.1 shipped bounded, default-off validated block serving and compact-block relay through Phases 110–129.
- Production runtime state, inbound serving, RPC/operator evidence, and compact announcement transport now share authoritative state and successful-write-only evidence.
- v2.2 is now active for package relay, rolling minimum-fee behavior, periodic rebroadcast, and sustained mempool-pressure policy.
- Historical phase directories remain tracked because deterministic verifier scripts still consume selected evidence.

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
- [Phase 129]: Compose the Phase 127/128 checker exports inside the Phase 129 aggregate guard and keep the guard stage-independent of volatile planning artifacts.
- [Phase 129]: D-06 resolved on the fix path: fallback compact_timeout_count counts only real Timeout cleanups; live in-flight getblocktxn facts project solely through the in-flight facet.
- [Phase 129]: Model gaps-open, verified pre-promotion, and reconciled archive-ready as the only legal Phase 129 states with evidence-claimed fail-closed mixture rejection.
- [Phase 129]: Pin HARD-05, OBS-01, and BOUND-02 archive-ready ownership to Phase 129 and keep the legacy Phase 124 final-audit path unreachable via the Phase 125/126 roadmap headings.
- [Phase 129]: Promote OBS-01, BOUND-02, and HARD-05 only after independent lifecycle-valid gsd-verifier evidence, land all six reconciled planning artifacts plus the activation summary in one atomic commit, and rerun the v2.1 audit in place to passed with archive routing.

## Pending Todos

- Keep historical `.planning/phases/` directories tracked because repo verifier scripts still depend on selected phase evidence.
- Keep repo-local Cargo and Bazel command forms in UAT guidance:
  - `cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- ...`
  - `bazel run //packages/open-bitcoin-cli:open_bitcoin -- ...`
- Keep public-network relay exposure opt-in and outside default deterministic verification unless a future phase explicitly changes that contract.
- Keep v2.2 package relay and rebroadcast bounded and evidence-backed without promoting public/default/production relay scope.

## Blockers/Concerns

- No active milestone blockers are recorded.
- Default local verification must remain deterministic; public-network relay review should be opt-in UAT evidence unless deliberately changed.
- Existing outbound sync, inbound serving, full-sync, soak, support-bundle, and release-boundary behavior must not regress when future milestones expand relay or production scope.
- Existing transaction relay, mempool, inbound, sync, support-bundle, and release-boundary behavior must not regress in future work built on v2.1 block-serving and compact-block relay scope.
- Package admission, rolling-fee decay, rebroadcast scheduling, and sustained-pressure behavior need exact Knots anchors and deterministic long-lived tests before any support claim is promoted.

### Quick Tasks Completed

| # | Description | Date | Commit | Directory |
| --- | --- | --- | --- | --- |
| 260719-bbh | Accept the approved v2.1 post-audit gap-planning state in closeout verification | 2026-07-19 | `e02bae71` | [260719-bbh](./quick/260719-bbh-teach-the-phase-124-closeout-checker-to-/) |
| Phase 128 P03 | 51m | 3 tasks | 9 files |
| Phase 128 P04 | 57min | 3 tasks | 25 files |
| Phase 129 P01 | 27m | 3 tasks | 5 files |
| Phase 129 P02 | 28m | 2 tasks | 5 files |
| Phase 129 P03 | 30m | 2 tasks | 6 files |
| Phase 129 P04 | 45m | 3 tasks | 9 files |

## Session Continuity

Last session: 2026-07-22
Stopped at: Defining milestone v2.2 requirements
Resume file: None
