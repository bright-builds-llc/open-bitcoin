---
phase: 167-candidate-history-validation-rules
plan: "18"
baseline_commit: 1b472fb225a290df5cdb06eeea2dc67c624c648e
subsystem: offline-candidate-history-verification
tags: [bun, verifier, source-guard, historical-inverse, evidence]
requires:
  - phase: 167-17
    provides: Accepted current guards and exact historical projections
provides:
  - Focused Phase 167 source, corpus and scope checker with source-derived mutation controls
  - Eight exact read-only offline commands registered once after Phase 166 in the default verifier
  - Strict current verifier projection back to frozen historical evidence
affects: [167-19, 167-25, 167-20, CVLD]
tech-stack:
  added: []
  patterns: [fixed source pins, bounded input loader, exact verifier registration, historical inverse]
key-files:
  created:
    - scripts/check-phase167-candidate-history.ts
    - scripts/check-phase167-candidate-history.test.ts
    - scripts/check-phase167-candidate-history/contracts.ts
  modified:
    - scripts/verify.sh
    - scripts/check-phase164-canonical-identity/historical-tooling.ts
    - scripts/check-phase164-canonical-identity/contracts.ts
    - scripts/check-phase165-shared-exact-work/contracts.ts
key-decisions:
  - Check current source and corpus membership without treating static pins as Rust behavioral evidence.
  - Register only read-only check and test modes; reject write, acquisition and network fetch substitutions.
  - Keep the frozen verifier snapshot and its historical hash while projecting current additions through exact inverse hunks.
requirements-completed: [CVLD-01, CVLD-02, CVLD-03, CVLD-04, CVLD-06]
requirements_scope: bounded-plan18-verifier-contribution-only
global_cvld_acceptance: open
generated_by: gsd-execute-plan
generated_at: "2026-10-31T05:11:15Z"
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
commits: false
status: canonical-summary-pending-root-final18
---

# Phase 167 Plan 18: Offline checker and verifier registration

Plan 18 adds a fixed Phase 167 checker for the accepted source, historical and Regtest corpora, selected rule wiring and bounded support scope. It checks transaction-to-script and script-to-witness execution, all eight direct script evaluator children, sigop budget, candidate context, median time, BIP30 and prior-Coin behavior, sealed history opening, seeded fixture isolation and native test registration. The loader requires regular files and finite per-file and total byte bounds. These source contracts supplement actual Rust tests; they do not create new canonical authority or pinned-daemon evidence.

The default verifier now lists eight Phase 167 offline commands exactly once, after Phase 166, in both its command inventory and executable steps. Each runs in check or test mode. Missing, duplicate, reordered, commented, write-mode, acquisition and network-fetch replacements fail the checker. The Phase 164 current-to-prior verifier projection removes the exact 16 new lines and restores the frozen historical base hash; its separate older verifier snapshot and manifest remain unchanged. Only the affected current Phase 165 hash rows were renewed.

## Test-first evidence and final commands

Before verifier edits, a pinned registration test failed on the eight missing declared and executed entries, and the checker CLI exited nonzero for the same reason. Two separate source tests failed because the skeleton did not yet reject a selected-halving mutation and a removed required historical block. Those are genuine behavioral REDs; the [worker addendum](../../../.local/open-bitcoin-dev/phase167/plan18/execution-v1/SUMMARY-ADDENDUM.md) preserves raw locations and chronology.

The two original literal Plan 18 test commands passed on the final frozen frame:

| Command | Final result |
| --- | --- |
| <code>bun test ./scripts/check-phase167-candidate-history.test.ts</code> | 49/49 pass, 130 expects |
| <code>bun test ./scripts/check-phase167-candidate-history.test.ts ./scripts/check-phase164-canonical-identity.test.ts ./scripts/check-phase165-shared-exact-work.test.ts</code> | 3,455/3,455 pass, 10,985 expects |

All eight registered offline commands also ran directly under pinned Bun 1.3.9, exited zero and kept source unchanged. Their four test commands discovered **55 historical**, **93 BIP30**, **32 Regtest** and **49 checker** tests. The [ROOT command audit](../../../.local/open-bitcoin-dev/phase167/plan18/root-final-evidence-audit-v1/audit.json) binds exact arguments, outputs, exits and hashes. Focused Phase 164/165 current guards, the final historical controls, shell syntax, file lengths and `git diff --check` pass. The full historical 231-test and 24-test tooling runs occurred on the pre-witness frame with their inverse/verifier owners unchanged; they are supporting evidence rather than final v2 command credit.

## Source conservation and review

The final frame has **1,821 current source files**, seven Plan 18 owners and **1,814 protected post-Plan 17** files. Four complete owner bodies changed and three new checker files were truly absent at release. Strict original-before BSD patch inverse and forward restore all four bodies and three absences, then reproduce final bytes with no offset, fuzz or reject. The [full candidate frame](../../../.local/open-bitcoin-dev/phase167/plan18/root-candidate-v2/full-frame.json), [inverse proof](../../../.local/open-bitcoin-dev/phase167/plan18/execution-v1/inverse-v2/summary.json), [independent source review](../../../.local/open-bitcoin-dev/phase167/plan18/root-source-review-v2/independent-review.json), [CORE packet](../../../.local/open-bitcoin-dev/phase167/plan18/execution-v1/CORE.json), [QA packet](../../../.local/open-bitcoin-dev/phase167/plan18/execution-v1/QA.json) and [independent final packet review](../../../.local/open-bitcoin-dev/phase167/plan18/root-final-packet-review-v1/independent-review.json) retain exact evidence. No production Rust, parity doc, frozen historical artifact or unowned source changed.

## Remaining boundary

Plan 19 must finish the remaining 109 source breadcrumbs, unique parity owner and current contributor docs. Plan 25 must reconcile root AGENTS and PROJECT excerpts. Plan 20 owns the default native verifier, fresh coverage/Bazel, independent source/security/goal review and phase acceptance. All five global CVLD requirements remain open. No Git staging, commit or push has occurred, and Phase 168 has not started.

## Self-Check: PASSED

- Both original commands and eight offline commands have pinned zero-exit, source-stable receipts with nonzero named test discovery.
- Seven-owner source conservation and strict historical verifier and original-before inverses pass.
- Independent source, final packet and ROOT command audits pass without claiming phase-wide acceptance.
