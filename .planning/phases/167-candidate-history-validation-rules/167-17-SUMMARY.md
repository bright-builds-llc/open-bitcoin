---
phase: 167-candidate-history-validation-rules
plan: "17"
baseline_commit: 1b472fb225a290df5cdb06eeea2dc67c624c648e
subsystem: current-source-guards-and-bounded-claims
tags: [bun, source-guards, parity, historical-inverse, evidence]
requires:
  - phase: 167-51
    provides: Accepted full Phase 167 product source and stored-body completion
provides:
  - Current Phase 158/162/164/165/166 and Phase 123 guards bound to reviewed Phase 167 source
  - Narrow default-Regtest product claim separated from seeded historical evidence
  - Exact historical projections and original-before source conservation
affects: [167-18, 167-19, 167-25, 167-20, CVLD]
tech-stack:
  added: []
  patterns: [direct production-body guards, exact source inverses, scoped current claims]
key-files:
  created:
    - scripts/check-phase123-runtime-timing-evidence-integrity/test-fixtures.ts
  modified:
    - scripts/check-phase158-validated-reorg.ts
    - scripts/check-phase162-integrated-filter-proof.ts
    - scripts/check-phase164-canonical-identity.ts
    - scripts/check-phase164-canonical-identity/contracts.ts
    - scripts/check-phase164-canonical-identity/claims.ts
    - scripts/check-phase165-shared-exact-work.ts
    - scripts/check-phase166-canonical-bootstrap.ts
    - scripts/check-phase123-runtime-timing-evidence-integrity/checks.ts
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - Bind current guards to actual bounded parser, durable writer, peer request and canonical startup owners while preserving frozen historical controls.
  - Keep seeded historical fixtures non-authoritative and permit only the proved local default-Regtest product claim.
  - Project renewed files through strict current-to-prior and copied-CLI inverses, with separate named support descriptors where generic mutation assumptions do not fit.
  - Defer the remaining 109 Phase 167 breadcrumb mappings to Plan 19.
requirements-completed: [CVLD-01, CVLD-02, CVLD-03, CVLD-04, CVLD-06]
requirements_scope: bounded-plan17-guard-contribution-only
global_cvld_acceptance: open
generated_by: gsd-execute-plan
generated_at: "2026-10-31T03:51:02Z"
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
commits: false
status: canonical-summary-pending-root-final17
---

# Phase 167 Plan 17: Current guards and evidence claims

Plan 17 renews current source guards against the accepted full Phase 167 product source. Phase 158 now checks the bounded block parser and registered metadata writer, including the counts-support preflight before persistence. Phase 162 checks the real peer-request and bounded header publication owners. Phase 164/165/166 check current identity, selected rules, genesis and durable startup without relaxing frozen historical attribution. Phase 123 checks actual managed handoff, request capacity and cleanup, send/write/ack ordering and sealed startup. Source-derived mutations reject wrong owners, test-only and path redirects, dead calls, reordered effects and unsupported claims.

The current claims contract separates the local ordinary default-Regtest, 301-descendant product route from independently seeded historical script/BIP30 and flag fixtures. Seeded cases confer no canonical authority. Current sections require explicit refusal or deferral for unsupported selected ordinary routes, actual-daemon and whole-chain qualification, public-default and production use, and the canonical-history runtime wallet. Plan 17 adds one truthful breadcrumb group for the moved metadata.rs writer. Plan 19 owns the other 109 presently missing Phase 167 mappings and final parity documentation.

## Test-first history and corrections

The independent pre-edit review authenticated genuine Phase 164 and Phase 123 stale-source RED findings. The Phase 123 suite discovered 129 tests, with 39 pass and 90 fail; only two clean-live positives are genuine behavior RED, while 88 negative failures cascade from that precondition. A mixed-time three-suite run receives no baseline or final credit. Phase 162 had two genuine current source-row REDs with a historical positive/refusal control. Phase 158's later pinned native failures traced to two stale current rows, not a frozen historical change.

Final source review closed the Phase 158 counts-support ordering gap, Phase 162 `#[path]` redirect gap, current-claim evidence and refusal gaps, copied-CLI direct inverse overlap and Phase 162 support-file generator mismatch. The interrupted broad runs remain diagnostics; their partial pass counts do not supply final acceptance. The [worker addendum](../../../.local/open-bitcoin-dev/phase167/plan17/execution-v1/SUMMARY-ADDENDUM.md) records the bounded sequence and exact evidence locations.

## Final frozen verification

All four original literal Plan 17 commands ran under pinned Bun 1.3.9 on the same source frame and exited zero:

| Command | Final result |
| --- | --- |
| <code>bun run scripts/check-phase164-canonical-identity.ts</code> | pass |
| <code>bun test ./scripts/check-phase164-canonical-identity.test.ts ./scripts/check-phase165-shared-exact-work.test.ts ./scripts/check-phase166-canonical-bootstrap.test.ts</code> | 3,767/3,767 pass, 11,521 expects |
| <code>bun run scripts/check-phase123-runtime-timing-evidence-integrity.ts</code> | pass |
| <code>bun test ./scripts/check-phase123-runtime-timing-evidence-integrity.test.ts</code> | 177/177 pass, 616 expects |

Additional final pinned suites passed: Phase 158 **927/927**, Phase 162 **2,642/2,642**, Phase 163 provenance **124/124**, Phase 163 native **178/178**, and focused historical positive/refusal **24/24**. The complete historical suite passed **231/231** on the same source. Six direct current checkers exit zero. Every final runner records literal arguments, Bun version, raw streams, timing, exit and unchanged owner hashes. The [CORE packet](../../../.local/open-bitcoin-dev/phase167/plan17/execution-v1/CORE.json), [QA packet](../../../.local/open-bitcoin-dev/phase167/plan17/execution-v1/QA.json) and [ROOT command audit](../../../.local/open-bitcoin-dev/phase167/plan17/root-plan17-final-evidence-audit-v1/audit.json) bind the results.

The final frame has **1,818 current source files**, including 24 direct Plan 17 owners. All **1,794 protected original** bodies match the accepted post-Plan 51 frame. Of 24 owners, 22 changed original-present bodies, one new Phase 123 fixture helper was originally absent, and one original-present owner remained byte identical. Strict original-before BSD patch inverse and forward restore all original bodies and true absence, then reproduce final bytes with no offset, fuzz or reject. The [full candidate frame](../../../.local/open-bitcoin-dev/phase167/plan17/root-plan17-final-candidate-v4/full-frame.json), [inverse proof](../../../.local/open-bitcoin-dev/phase167/plan17/execution-v1/inverse-v3/summary.json), [independent source reviews](../../../.local/open-bitcoin-dev/phase167/plan17/root-phase162-support-inverse-review-v2/independent-review.json), and [independent final packet review](../../../.local/open-bitcoin-dev/phase167/plan17/root-final-packet-review-v1/independent-review.json) retain exact hashes. Normal line limits and `git diff --check` pass; Plan 17 structure, lifecycle metadata and summary syntax checks pass without phase-wide acceptance.

## Remaining boundary

This is bounded guard and claim acceptance. Plan 18 must register eight offline verifier commands and preserve its strict inverse; Plan 19 must finish parity, breadcrumbs and contributor docs; Plan 25 must reconcile root excerpts; Plan 20 owns the default native verifier, fresh coverage/Bazel, independent source/security/goal review and phase acceptance. Global CVLD requirements remain open. No Git staging, commit or push has occurred, and Phase 168 has not started.

## Self-Check: PASSED

- All four original commands and all named affected suites have final zero-exit, nonzero-discovery evidence on unchanged source.
- Protected originals, direct owner hashes, historical snapshots and strict original-before inverse match the reviewed frame.
- Independent packet QA and ROOT command/source audit pass; no global phase or Git acceptance is inferred.
