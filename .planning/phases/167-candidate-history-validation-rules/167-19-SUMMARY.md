---
phase: 167-candidate-history-validation-rules
plan: "19"
baseline_commit: 1b472fb225a290df5cdb06eeea2dc67c624c648e
subsystem: candidate-history-parity-and-contributor-scope
tags: [parity, breadcrumbs, documentation, claims, rpc]
requires:
  - phase: 167-18
    provides: Accepted source guards and offline verifier registration
provides:
  - Complete first-party Rust source breadcrumb mapping and unique bounded Phase 167 parity owner
  - Current contributor documentation for the local default-Regtest route and its evidence limits
  - Current submitblock RPC catalog and documentation reconciliation
affects: [167-25, 167-20, CVLD]
tech-stack:
  added: []
  patterns: [source-derived parity attribution, current-versus-historical claims, exact catalog grouping]
key-files:
  created:
    - docs/parity/v2-6-candidate-history-validation.md
  modified:
    - docs/parity/source-breadcrumbs.json
    - docs/parity/index.json
    - docs/parity/support-matrix.md
    - docs/parity/v2-6-canonical-chain-identity.md
    - docs/parity/catalog/rpc-cli-config.md
    - scripts/check-phase164-canonical-identity/claims.ts
    - scripts/check-current-documentation-reconciliation.ts
    - README.md
    - packages/README.md
    - packages/open-bitcoin-node/README.md
    - .planning/ARCHITECTURE.md
key-decisions:
  - Keep actual bounded default-Regtest descendants and retained reopen distinct from seeded historical script, BIP30 and flag fixtures.
  - Record the intentional complete prior-Coin restoration difference from pinned Knots disconnect behavior.
  - Validate every current Phase 167 section and retain the valid duplicate-section positive while rejecting malformed appended sections.
  - Catalog submitblock as the 22nd baseline-backed method within a 30-method current registry, bounded to supported local use.
requirements-completed: [CVLD-01, CVLD-02, CVLD-03, CVLD-04, CVLD-06]
requirements_scope: bounded-plan19-parity-contribution-only
global_cvld_acceptance: open
generated_by: gsd-execute-plan
generated_at: "2026-10-31T06:55:12Z"
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
commits: false
status: canonical-summary-pending-root-final19
---

# Phase 167 Plan 19: Parity and current contributor scope

Plan 19 adds a unique `in_progress` parity surface for CVLD-01/02/03/04/06 and documents the observed local route: proved genesis followed by 301 default-Regtest descendants, selected rule context and complete retained-history reopen. It records seven pinned historical block identities and eight source/corpus hashes. Independently seeded Mainnet/Testnet3 script and Mainnet BIP30 cases, along with Regtest flag fixtures, are evidence for their exact cases and do not create earned canonical product authority. The page states the intentional BIP30 difference: Open Bitcoin retains and restores the complete prior Coin on disconnect, while the pinned Knots path does not restore the overwritten coinbase in the same way. The finite 305-known/303-active observation and separate 306-position envelope are distinct from general performance or whole-chain qualification.

Current README, support, identity and architecture sections reflect this bounded route. They preserve historical Phase 164–166 scopes and explicitly defer Mainnet/testnet3/default Signet ordinary product routes, actual pinned-daemon differential, arbitrary whole-chain and public/default operation, production service or funds claims, and canonical-history wallet availability. UAT text gives repo-local Cargo and Bazel commands. ROOT still owns the AGENTS and PROJECT excerpts in Plan 25.

## Breadcrumb and catalog reconciliation

The initial pinned breadcrumb check failed on 109 unmapped current Rust files. An independently reviewed 79-group preview mapped all 109 exactly once with existing source comments and Knots anchors. The full checker then reached four older comment/group mismatches previously hidden by the early missing-file failure. A reviewed manifest-only split corrected those four without Rust edits. The final manifest has **352 unique groups** and maps all **1,356 scoped first-party Rust files** exactly once. Ten `none` files retain the original generic source comment; their source-specific reasons are recorded in the [worker addendum](../../../.local/open-bitcoin-dev/phase167/plan19/execution-v1/SUMMARY-ADDENDUM.md) and independent preflight.

The legitimate current Phase 167 heading exposed a guard that checked only the first such section. Two existing malformed appended-section tests genuinely failed, 14 pass/2 fail. The checked claims repair validates every current section: both negatives pass while the existing valid duplicate positive remains accepted. A separate direct documentation checker produced three genuine `submitblock` drift findings, and its live-corpus test failed before repair. The two-file checked refinement aligns the current registry and catalog to **30 methods: 22 baseline-backed, including submitblock, and eight extensions**. Pinned Knots mining source anchors the method; the catalog limits current use to the local default-Regtest route. The direct checker, its 20-test suite and missing/misgrouped/wrong-count scratch mutations pass. The first broad claims run was stopped at 322 passing cases for that confirmed source defect and earns no final credit.

## Final frozen verification

Both original literal Plan 19 commands passed on the final twelve-owner frame:

| Command | Final result |
| --- | --- |
| <code>bun run scripts/check-parity-breadcrumbs.ts --check</code> | exit 0; 1,356 Rust files mapped |
| <code>bun run scripts/check-phase167-candidate-history.ts</code> | exit 0 |

The affected Phase 164/165/167 combined suite passed **3,455/3,455** tests with 10,985 expects, including the malformed and valid duplicate-section controls. The current documentation reconciliation suite passed **20/20**; scoped Phase 164/165 claims passed **16/16** and **34/34**, and Phase 167 checker tests passed **49/49**. Direct Phase 158/162/164/165 checks, file lengths and `git diff --check` passed. All final Bun runs used pinned 1.3.9, exited zero and recorded unchanged owner hashes. The [CORE packet](../../../.local/open-bitcoin-dev/phase167/plan19/execution-v1/CORE.json), [QA packet](../../../.local/open-bitcoin-dev/phase167/plan19/execution-v1/QA.json) and [ROOT command audit](../../../.local/open-bitcoin-dev/phase167/plan19/root-final-evidence-audit-v1/audit.json) retain literal arguments, streams, discovery, timings, exits and source frames.

The final frame contains **1,826 current source and document files**, twelve Plan 19 owners and **1,814 protected original** files. Eleven complete original owner bodies changed; the new parity page was truly absent. Strict original-before BSD patch inverse and forward restore all eleven bodies and the true absence, then reproduce final bytes with no offset, fuzz or reject. The [full source frame](../../../.local/open-bitcoin-dev/phase167/plan19/root-candidate-v2/full-frame.json), [inverse proof](../../../.local/open-bitcoin-dev/phase167/plan19/execution-v1/inverse-v2/summary.json), [independent whole-source review](../../../.local/open-bitcoin-dev/phase167/plan19/root-whole-source-review-v2/independent-review.json) and [independent final packet review](../../../.local/open-bitcoin-dev/phase167/plan19/root-final-packet-review-v1/independent-review.json) bind exact hashes and claims.

## Remaining boundary

Plan 25 must reconcile the two root-owned AGENTS and PROJECT files. Plan 20 still owns the complete default native verifier, fresh coverage/Bazel, independent source/security/goal review and phase acceptance. The five global CVLD requirements remain open. Actual pinned-daemon differential and arbitrary whole-chain/public/production use remain future work. No Git staging, commit or push has occurred; Phase 168 has not started.

## Self-Check: PASSED

- Both original Plan 19 commands and affected direct/broad suites have final pinned zero-exit, source-stable receipts.
- All scoped Rust breadcrumbs, twelve-owner conservation and strict original-before inverse pass.
- Independent whole-source, final packet and ROOT command audits pass without claiming phase-wide acceptance.
