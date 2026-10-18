---
phase: 165-shared-exact-work-authority
plan: "13"
subsystem: parity-and-operator-work-evidence
tags: [proof-work, provenance, documentation, refusal, mutation-controls]
requires:
  - phase: 165-12
    provides: Pinned source/test evidence owners and bounded ordered input loader
provides:
  - Auditable exact-work parity page and current catalog entry
  - Fixed current-page input and six actual overclaim mutations
  - Contributor/operator scope, package entrypoints and real Cargo/Bazel command forms
  - Complete audit of new Rust source breadcrumbs
affects: [165-14, 166, 167, 168]
tech-stack:
  added: []
  patterns:
    - Separate exact numerical facts, earned authority and runtime qualification
    - Check replacement and appended overclaims against the actual loaded document
key-files:
  created:
    - docs/parity/v2-6-shared-exact-work.md
    - packages/open-bitcoin-node/README.md
    - packages/open-bitcoin-cli/README.md
  modified:
    - scripts/check-phase165-shared-exact-work.ts
    - scripts/check-phase165-shared-exact-work/contracts.ts
    - scripts/check-phase165-shared-exact-work.test.ts
    - docs/parity/index.json
    - docs/parity/v2-6-canonical-chain-identity.md
    - README.md
    - docs/operator/runtime-guide.md
    - docs/architecture/status-snapshot.md
key-decisions:
  - Keep the catalog in_progress until full phase acceptance; contributed requirements are not independently closed here.
  - Ordinary profiles still report exact_work missing despite the delivered numeric/live-owner foundation.
  - Preserve historical source identities and receipts separately from current exact carriers and local refusal processes.
requirements-completed: []
requirements-contributed: [CWOR-01, CWOR-02, CWOR-03, CWOR-05]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 165-2026-10-14T16-09-59Z
generated_at: "2026-10-17T02:23:01Z"
duration: approximately 6min recorded implementation/verification; context loading not separately timed
completed: 2026-10-16
tasks_completed: 2
files_modified: 11
---

# Phase 165 Plan 13: Exact-Work Parity and Operator Scope Summary

**A source-linked exact-work catalog, fixed claim-input guard and concrete operator guidance distinguish numeric correctness, earned live authority and non-opening ordinary refusal.**

## Delivered Scope

Task 1 adds the substantive work parity page and a unique current catalog entry for CWOR-01/02/03/05. The page links the exact Knots pin, formula/compact/wrapping behavior, independent 11-proof/six-addition corpus, real source/test owners and predecessor execution evidence. It distinguishes strict first-accepted header ties from body/ancestor insertion-FIFO order, sticky cached unknown order, immutable linkage seals from revocable eligibility epochs, private source/tip/generation stages, full-identity consuming rebind and checks before actual effects. Typed consensus-only fallback, fatal local failures, fresh candidate authorization, accepted-prefix turn termination and actual-active BASIC identity retain their limits.

The legacy codec remains numerical: strict compact 512-byte records, historical u128 classification and exact bytes never grant lineage. Fixed guidance precedes the explicit work decision; all observations retain unavailable stored work. Six new physical/effect controls plus thirteen reused original controls are accurately described as nineteen module cases, not nineteen independent additions. One normal local Open Bitcoin daemon integration case executes thirty-two children, separate from three factory tests and six CLI cases; none is pinned-Knots differential qualification.

Original preservation attribution remains the 20,556-byte Git blob `0345bab1bfbd76be3078453a6b9201fdf73941cc`, SHA-256 `3d004f91bd72a0f3902588a36fca0438b04b5ac034bc9e227893664496835630`. The mechanically migrated current source remains separately attributable at 20,688 bytes, SHA-256 `e6cb5740cb5c83655aed14c43b1206847d264431d944741a365771c3e8d76f5b`. Neither historical manifests nor receipt bytes/source IDs were rebaselined.

Only the new parity page was appended to the actual fixed ordered input inventory; its pin was renewed after review. All source-owner digests, historical identities, inventory traversal/extra/missing/reordered controls, enabled test ownership and bounded regular-descriptor behavior remain intact. Three real sentence replacements and three appended positive assertions must produce an attributed claim finding for daemon qualification, deterministic restart ties and canonical bootstrap/reopen. Appended controls preserve the required disclaimers, so missing-text checks alone cannot satisfy them.

Task 2 updates the current root README, operator guide and status contract, and creates concise package READMEs at the two declared paths that were absent. The examples use actual existing global options and status/sync status definitions from the operator parser and network-resolution owner, with fully copy-pasteable timed Cargo and Bazel forms. No public networking is started. Exact live work renders as sixty-four lowercase hexadecimal digits; unavailable observations are not invented work. All four selectors retain identity-only ordinary refusal and default-off networking. The ordinary profile's missing exact_work prerequisite remains explicit despite the delivered foundation.

The complete inventory found thirty-eight new first-party Rust source/test paths, each already mapped to defensible Knots source anchors. The existing mapping was retained unchanged by this plan; the full breadcrumb checker covers 1,198 Rust files. Eleven owned implementation/documentation paths changed, plus this summary; the twelfth declared path was audited without unnecessary edits.

## Executed Evidence

All Bun verification used the command-local pinned Bun 1.3.9 PATH. The full current matrix ran serially, with no Cargo/Bazel or other heavy-suite overlap. The recorded implementation/verification clock begins at 2026-10-17T02:16:15Z. CLI definitions were inspected; the documentation examples were not presented as executed operator commands. No Rust change or Cargo build was needed for this documentation/checker plan.

| Command or check               | Actual result                                                                                                                                                                                                                                                                             |
| ------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Breadcrumb check               | bun scripts/check-parity-breadcrumbs.ts --check: passed for 1,198 Rust files. All thirty-eight new Rust paths were separately inventoried against existing mappings.                                                                                                                      |
| Focused current-page controls  | bun test ./scripts/check-phase165-shared-exact-work.test.ts --test-name-pattern 'actual current source\|work document overclaim': seven passed, 836 filtered out, zero failed, forty-two expectations; 1.37s.                                                                             |
| Final declared Phase165 matrix | bun test ./scripts/check-phase165-shared-exact-work.test.ts: **843 passed, zero failed, 3,098 expectations, 159.90s**, session `98387`, exit zero. This includes all six new actual-document overclaim controls and preserved inventory/test-owner controls.                              |
| Current Phase165 checker       | bun run scripts/check-phase165-shared-exact-work.ts: passed at both task gates, eighty reviewed owners and twenty-six executable test-owner files.                                                                                                                                        |
| Older current-source checker   | bun run scripts/check-phase164-canonical-identity.ts: passed, 131 fixed Rust inputs and 452 reviewed source edges.                                                                                                                                                                        |
| Stable matrix inputs           | All 216 saved input/checker SHA-256s matched unchanged after the full matrix and both task gates. The fixed loader now has 213 inputs. One actual final full matrix serves both gates; no duplicate full suite or source-only substitute is claimed.                                      |
| Markdown                       | Initial scoped check correctly reported four unformatted files; scoped write and final check with mdformat --extensions gfm --extensions frontmatter passed for all seven changed Markdown files. Existing table padding/list markers were normalized without changing historical claims. |
| Managed checks                 | bun scripts/bright-builds-check.ts all: 1,521 inputs, zero exceptions/findings; active lesson check passed.                                                                                                                                                                               |
| Links/catalog                  | All forty-one local parity-page links and every new catalog evidence/upstream path exist; unique in-progress surface and exact four requirement IDs verified.                                                                                                                             |
| Caps/diff/index                | Checker/contracts/test files are 181/147/300 lines, each below 628 with no exception. git diff --check passed at both task gates; Git index remains empty.                                                                                                                                |

The full matrix log and stability map are retained in ignored `.local/open-bitcoin-dev/phase165-plan13-final-guard.log` and `.local/open-bitcoin-dev/phase165-plan13-input-hashes.json`. The passing predecessor matrix remains a distinct historical source receipt, not a replacement for this actual affected run. No product behavioral RED is claimed for documentation; the initial current-page checks were honestly GREEN with adversarial loaded-text controls.

## Deviations and Review

No scope expansion was needed. The two package README paths were absent, so Task 2 created useful entrypoints at those exact declared paths, as confirmed by the parent. Task 1 breadcrumb mapping required no new row because all new Rust paths were already covered. Both documentation tasks were completed before the single final unchanged-source matrix, which serves both gates with explicit hash proof.

The simplification pass retained the existing loader and claim-policy structure, added one bounded document and a small claim helper, and avoided a new dependency or alternative authority model. No configured TS formatter/build project exists; adjacent style, actual Bun execution, managed checks and caps provide the applicable TS checks. Markdown used the available GFM/frontmatter plugins. AGENTS.md, the Bright Builds sidecar, placeholder-only overrides and architecture/code-shape/verification/testing/Rust/TypeScript standards materially informed the scope. Both active lesson files were read completely: 7,188 bytes / 2,397 conservative estimated tokens.

No blocking stub or new production endpoint, authentication, schema or file-access authority was introduced. The only added checker input is the declared fixed parity document under the existing bounded loader. No authentication gate occurred.

The loose summary helper initially classified the self-check's explicit zero-failure wording as a failure. Its presentation was clarified without changing the actual receipt. Its known non-task-hash warning remains: the protected original Git blob above is a source blob, not an executor commit, and no task commit exists by design.

## Integration and Finalization Handoff

The catalog remains in_progress. Phase 166 retains canonical bootstrap, exact checkpoint/all-handle reopen and genesis BASIC; Phase 167 retains contextual completeness; Phase 168 retains actual pinned-Knots differential qualification. No deterministic restart tie, ordinary canonical acceptance, whole-chain, production-service or production-funds claim is made. Historical synthetic/u128 evidence retains its original scope. Whole-crate complete coverage is not inferred from changed-owner coverage; Plan 14 owns the actual native acceptance policy.

All per-task and metadata commits are deferred to the parent strict wrapper. This executor created no worktree, staged/committed/pushed nothing, changed no hooks and did not mutate shared STATE/ROADMAP/REQUIREMENTS/config/todo. Parent Plan 14 owns full default native/workspace verification, independent review/security/formal verification, shared state and normal-hook Git finalization. No commit hash was invented.

## Self-Check: PASSED

All twelve declared paths and this summary exist. The actual final matrix log confirms 843/843 passed and 3,098 expectations; the process exited zero. All 216 input/checker hashes remain unchanged across that matrix and both live checker gates. Complete breadcrumbs, catalog/link audit, scoped Markdown, managed checks, file caps and whitespace checks passed. No blocking stub remains; the Git index is empty and no executor commit is expected. Shared phase acceptance and finalization remain parent-owned.
