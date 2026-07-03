---
phase: 107-runtime-relay-activation-and-download-eligibility-integration
plan: 05
subsystem: deterministic-guardrails
tags:
  - relay
  - runtime-activation
  - download-eligibility
  - verifier
  - bun

requires:
  - phase: 107-runtime-relay-activation-and-download-eligibility-integration
    provides: Plan 107-01 pure relay download eligibility and scheduler suppression labels
  - phase: 107-runtime-relay-activation-and-download-eligibility-integration
    provides: Plan 107-02 runtime relay activation propagation through RPC context construction
  - phase: 107-runtime-relay-activation-and-download-eligibility-integration
    provides: Plan 107-03 sanitized activation and download eligibility evidence
  - phase: 107-runtime-relay-activation-and-download-eligibility-integration
    provides: Plan 107-04 parity roots, docs, and UAT command evidence
  - phase: 106-parity-traceability-uat-and-release-boundary-guardrails
    provides: Existing deterministic v2.0 release-boundary checker pattern and verifier position
provides:
  - Deterministic Phase 107 checker for runtime activation and download eligibility drift
  - Mutation tests for activation propagation, scheduler gate ordering, status evidence, docs/UAT roots, verifier wiring, forbidden claims, and public-network verifier gates
  - Default verifier wiring immediately after Phase 106
  - Refreshed tracked LOC report
affects:
  - scripts/verify.sh
  - docs/metrics/lines-of-code.md
  - Phase 107 closeout verification

tech-stack:
  added: []
  patterns:
    - Fixed-corpus Bun checker with mutation fixture tests
    - Verifier order checked in both visible command block and executable run_step order
    - Whitespace-normalized Markdown evidence matching for stable doc guardrails

key-files:
  created:
    - scripts/check-phase107-runtime-relay-activation-download-eligibility.ts
    - scripts/check-phase107-runtime-relay-activation-download-eligibility.test.ts
    - .planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-05-SUMMARY.md
  modified:
    - scripts/verify.sh
    - docs/metrics/lines-of-code.md

key-decisions:
  - "Check the exact runtime handoff through new_with_relay_activation, config.relay, and config.inbound.enabled."
  - "Require relay eligibility suppression before scheduler candidate or in-flight mutation for both announcements and orphan parent requests."
  - "Validate Phase 107 operator evidence as aggregate, sanitized, fixed-label status evidence rather than peer, permission, endpoint, or transaction material."
  - "Wire Phase 107 immediately after Phase 106 without adding public-network, wall-clock soak, service-manager, production-deployment, or production-funds gates."

patterns-established:
  - "Phase 107 checker reads the same fixed corpus registered by Plan 107-04 parity roots."
  - "Mutation fixtures create temporary repo roots from the real corpus and remove or alter one contract at a time."
  - "Default verifier additions are guarded by the checker itself, preventing visible-only or executable-only wiring drift."

requirements-completed:
  - ACT-01
  - ACT-02
  - INV-02
  - INV-03
  - DL-01
  - DL-02
  - REL-03
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 107-2026-07-03T02-54-20
generated_at: 2026-07-03T05:05:26Z

duration: 12m
completed: 2026-07-03
---

# Phase 107 Plan 05: Runtime Relay Activation and Download Eligibility Integration Summary

**Deterministic Phase 107 guardrails now fail on dropped runtime activation, missing download eligibility gates, missing sanitized evidence, stale docs/UAT roots, forbidden relay claims, and verifier drift.**

## Performance

- **Duration:** 12m
- **Started:** 2026-07-03T04:53:55Z
- **Completed:** 2026-07-03T05:05:26Z
- **Tasks:** 2
- **Files modified/created:** 5, including this summary

## Accomplishments

- Added `checkPhase107RuntimeRelayActivationDownloadEligibility` as a fixed-corpus Bun checker over runtime construction, managed relay policy wiring, scheduler ordering, status evidence, docs/parity roots, UAT commands, and verifier order.
- Added 15 mutation tests covering missing `config.relay`, missing `config.inbound.enabled`, default constructor regression, missing suppression labels, scheduler mutation before eligibility, missing status fields, missing docs/UAT roots, missing verifier wiring, forbidden claims, sensitive public evidence, and public-network verifier gates.
- Wired the Phase 107 checker test and checker run into `scripts/verify.sh` immediately after Phase 106 in both visible and executable order.
- Refreshed `docs/metrics/lines-of-code.md` after adding the checker/test pair.

## Task Commits

No commits were created. The execution request explicitly instructed this executor not to commit or push.

1. **Task 1: Add deterministic Phase 107 checker and mutation tests** - complete, not committed here.
2. **Task 2: Wire Phase 107 checker into default verifier order** - complete, not committed here.

## Files Created/Modified

- `scripts/check-phase107-runtime-relay-activation-download-eligibility.ts` - New deterministic Phase 107 checker.
- `scripts/check-phase107-runtime-relay-activation-download-eligibility.test.ts` - Mutation tests for checker drift categories.
- `scripts/verify.sh` - Adds Phase 107 test/checker commands immediately after Phase 106.
- `docs/metrics/lines-of-code.md` - Regenerated from the current worktree.
- `.planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-05-SUMMARY.md` - Records this execution.

## Decisions Made

- The checker asserts gate ordering statically inside `TxDownloadScheduler` so eligibility remains before candidate or in-flight mutation.
- Runtime propagation is guarded by a direct ordered check for `ManagedPeerNetwork::new_with_relay_activation`, `config.relay`, and `config.inbound.enabled`.
- Documentation checks normalize whitespace so guardrails survive Markdown wrapping while still requiring the Phase 107 UAT aggregate/sanitized wording.
- The verifier remains deterministic and public-network-free; no service manager, wall-clock soak, production deployment, or production-funds gate was added.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

- The TDD RED run failed as expected because the test file imported the not-yet-created checker module.
- The first checker test run exposed an overly brittle line-wrapped documentation assertion; the checker now normalizes Markdown whitespace for required doc wording.
- The LOC freshness check failed after adding the checker/test files; regenerating `docs/metrics/lines-of-code.md` resolved it.

## Known Stubs

None. A targeted scan of the Plan 107-05 files found no `TODO`, `FIXME`, placeholder, coming-soon, not-available text, or hardcoded empty UI/data stubs.

## Threat Flags

None. This plan adds deterministic local TypeScript checks and verifier wiring only; it adds no network endpoint, auth path, file-access trust boundary, schema change, service-bit change, compact block behavior, package relay, bloom/filter serving, public relay default, public-network verifier gate, or durable mempool recovery behavior.

## Verification

- `bun test scripts/check-phase107-runtime-relay-activation-download-eligibility.test.ts` - RED failed before checker creation, then passed with 15 tests.
- `bun run scripts/check-phase107-runtime-relay-activation-download-eligibility.ts` - passed.
- `rg -n "checkPhase107RuntimeRelayActivationDownloadEligibility|config\\.inbound\\.enabled|RelayDownloadEligibilityCounters|v2-0-runtime-relay-activation-download-eligibility" scripts/check-phase107-runtime-relay-activation-download-eligibility.ts scripts/check-phase107-runtime-relay-activation-download-eligibility.test.ts` - passed.
- `rg -n "check-phase106-parity-uat-release-boundary|check-phase107-runtime-relay-activation-download-eligibility" scripts/verify.sh` - passed.
- `bun run scripts/generate-loc-report.ts --source=worktree --output=docs/metrics/lines-of-code.md --check` - failed before refresh, then passed after regeneration.
- `git diff --check` - passed.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Plan 107-06 can rely on the default verifier running Phase 107 guardrails immediately after Phase 106. The remaining closeout should create final Phase 107 verification evidence without changing the deterministic public-network-free default verifier boundary.

## Self-Check: PASSED

- Created summary file: `.planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-05-SUMMARY.md`
- Verified the checker exports `checkPhase107RuntimeRelayActivationDownloadEligibility`.
- Verified `scripts/verify.sh` lists and runs the Phase 107 checker immediately after Phase 106.
- Verified `docs/metrics/lines-of-code.md` is current after checker/test additions.
- No commits were created, matching the execution request.

*Phase: 107-runtime-relay-activation-and-download-eligibility-integration*
*Completed: 2026-07-03*
