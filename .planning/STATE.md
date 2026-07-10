---
gsd_state_version: 1.0
milestone: v2.1
milestone_name: Block Serving and Compact Block Relay Boundary
status: verifying
stopped_at: Phase 117 verification passed
last_updated: "2026-07-10T07:27:00.000Z"
last_activity: 2026-07-10
progress:
  total_phases: 8
  completed_phases: 8
  total_plans: 29
  completed_plans: 29
  percent: 100
---

# Project State

## Project Reference

See: `.planning/PROJECT.md` (updated 2026-07-10 after Phase 117 completed v2.1 implementation and verification).

**Core value:** When a behavior is in scope, Open Bitcoin must behave like the pinned Knots baseline on the outside while staying simpler and safer on the inside.
**Current focus:** v2.1 milestone audit and archive readiness after Phase 117 verification

## Current Position

Milestone: v2.1 Block Serving and Compact Block Relay Boundary
Phase: 117
Plan: 4 of 4 complete
Status: Phase complete and verified — ready for milestone audit
Last activity: 2026-07-10

v2.0 Transaction Relay and Mempool Participation Boundary shipped on 2026-07-03. The archived audit reports 32/32 requirements, 10/10 phases, 8/8 integration checks, 8/8 cross-phase flows, and no tracked tech debt.

v2.1 focuses on bounded, opt-in block-serving and compact-block relay behavior with Bitcoin Knots parity evidence, deterministic local verification, and explicit no-claim guardrails for package relay, public defaults, public-network CI, production full-node readiness, and production-funds wallet use.

The completed v2.1 roadmap contains 8 phases, 29 plans, and 34 mapped requirements:
Phase 110 block-serving activation and eligibility, Phase 111 full block serving, Phase 112 BIP152 wire codecs, Phase 113 compact relay negotiation and announcement policy, Phase 114 compact reconstruction, Phase 115 missing transaction/fallback/validation handoff, Phase 116 operator evidence, and Phase 117 parity/UAT/release guardrails.

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
- v2.1 completed bounded, explicit, default-off validated block serving and compact-block relay with BIP152 codecs, reconstruction/fallback, sanitized operator evidence, exact parity roots, five-test UAT, and hardened deterministic no-claim guardrails.

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

## Pending Todos

- Audit v2.1 before milestone archive.
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

## Session Continuity

Last session: 2026-07-10T07:27:00.000Z
Stopped at: Phase 117 verification passed
Resume file: .planning/phases/117-parity-traceability-uat-and-release-guardrails/117-VERIFICATION.md
