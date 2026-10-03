---
phase: 152-post-prune-wallet-rescan-eligibility
plan: "03"
subsystem: docs
tags: [wallet, rescan, prune, parity, evidence]
requires:
  - phase: 152-01
    provides: Shared full-replacement gate, node runtime and real paired-prune evidence
  - phase: 152-02
    provides: Durable RPC eligibility and checkpoint/refusal regressions
provides:
  - Stable SNAP-01 parity owner with Phase 152 closure and historical Phase 146 foundations
  - Pinned wallet anchors and executable node/RPC evidence map
  - Truthful contributor status pending root verification and Phase 153 retention
affects: [SNAP-01, phase-152-verification, phase-153]
tech-stack:
  added: []
  patterns: [behavioral evidence before lifecycle closure claims]
key-files:
  created: []
  modified:
    - docs/parity/index.json
    - docs/parity/checklist.md
    - docs/parity/catalog/chainstate.md
    - README.md
key-decisions:
  - "Preserve stable SNAP-01 surface and historical done foundation; canonical requirement remains Pending until root report."
  - "Distinguish Open Bitcoin full replacement and every-candidate creating gate from Knots range/incremental scanning."
  - "Document actual checkpoint, truthful freshness/target, sanitized Failed evidence and scoped completed-prune concurrency limits."
  - "Root owns lifecycle completion, tracking, full verification and all git finalization."
patterns-established:
  - "Link actual behavioral fixtures and observed executor runs rather than source-string closure assertions."
requirements-completed: [SNAP-01]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 152-2026-10-03T00-26-52
generated_at: 2026-10-03T01:46:30Z
duration: approximately 12 min
completed: 2026-10-03
---

# Phase 152 Plan 03: Post-Prune Wallet Eligibility Documentation Summary

**The existing SNAP-01 surface now maps the shared full-replacement eligibility/refusal contract to real paired-prune node/RPC fixtures and pinned wallet roots, with closure pending root verification.**

## Accomplishments

- Updated exactly the four planned documentation paths. The stable `v2-4-wallet-leftover-snapshot-cutover` owner remains unique; Phase 146 evidence stays historical and Phase 152 Post-Prune Wallet Rescan Eligibility owns INT-02 closure.
- Added pinned `wallet.cpp::ScanForWalletTransactions` and `wallet/rpc/transactions.cpp::rescanblockchain` roots. The catalog explicitly distinguishes Knots requested-range transaction scanning from Open Bitcoin's project-specific staged full UTXO replacement and stricter candidate-creating gate.
- Linked shared helper, node eligibility, durable RPC eligibility and paired-unlink fixture sources. Recorded requested/older creating eligibility, retained/unrelated controls, checkpoint preservation, safe Failed evidence, missing-target refusal, direct-helper bypass closure and fresh resume/reopen probes.
- Distinguished node physical H=[new,old]/missing-B fixtures from RPC injected typed authority failures. Kept leftover bytes non-authoritative without deleting historical artifacts or implying repair.
- README edits are confined to the existing audit status paragraph and evidence links. Its exact v2.4 claim sentence and separate no-claim boundaries are unchanged; Phase 153 retention and both its requirements remain pending.

## Verification Evidence

All scoped checks used ignored `packages/target/phase152/tooling/bun-darwin-aarch64` first on PATH; `bun --version` returned **1.3.9**. This documentation executor ran no Cargo/Bazel jobs and did not overlap the RPC review fix.

| Check | Observed result |
| --- | --- |
| `bun run scripts/check-phase151-parity-uat-release-boundary.ts` | Passed; canonical Pending gap closure accepted and unique owner retained. |
| `bun run scripts/check-parity-breadcrumbs.ts --check` | Passed for 911 first-party Rust files. |
| `bun scripts/bright-builds-check.ts all` | 1,222 files scanned, zero findings; active repo lessons passed. |
| `git diff --check` | Passed. |
| JSON/YAML self-check | Exactly one stable SNAP-01 owner, matching lifecycle, empty completion array, exactly two frontmatter delimiters and all four modified paths verified. |
| Relative-link check | All 24 Phase 152, eligibility fixture and pinned-wallet links resolve. |
| Scoped diff/claim review | Four owned paths only; no runtime-guide mutation, dependency, checker, source-string flow assertion or artifact deletion. Exact README v2.4 claim and deferred paragraph preserved. |

Runtime evidence comes from [Plan 01](152-01-SUMMARY.md), [Plan 02](152-02-SUMMARY.md) and the root's confirmed final executor results, not a new docs-agent test run. Node shared tests passed 8/8, final eligibility 10/10, coins migration 15/15 and node Clippy passed. RPC's final matrix passed 19/19 and freshness compatibility 4/4; construction passed 5/5, range dispatch 1/1, node wallet 39/39 and final review-fix RPC Clippy passed. Both executors captured actual unchanged-adapter RED after paired deletion before GREEN. Full root verification remains pending at handoff.

The final RPC matrix includes review regressions for a previously Fresh job retargeted from height 1 to height 3 with Failed-save error and persisted Pending/Partial readback, an unknown checkpoint retaining Scanning, and real paired-prune refusal with a checkpoint already meeting its target retaining truthful Fresh. The root confirmed all 19 tests passed. Neither those targeted results nor this summary asserts full phase verification. Root must finish the default verifier and passing lifecycle report before completing SNAP-01.

## Task Commits and Tracking

1. **Task 1: Reconcile stable parity owner and runtime evidence** — prepared; commit deferred.
2. **Task 2: Contributor status and root evidence package** — prepared; commit deferred.

The explicit strict-wrapper override leaves staging, commits, push, branches/worktrees, canonical requirement/roadmap/state updates and the single default `bash scripts/verify.sh` with the root. No executor git mutation was performed. `requirements-completed` is deliberately empty until passing lifecycle-valid Phase 152 closeout.

## Decisions and Simplification

Repo-local AGENTS guidance, Bright Builds sidecar, standards index, architecture and verification pages informed authority wording, documentation scope, exact claims and verification. Both active lesson inputs were loaded completely: 5,230 global plus 1,958 repo bytes, 2,397 estimated tokens; no archives were read. No project skills were found. Plan 03 is already inside the parent GSD lifecycle, with scope and verification provided by its checked plan.

The simplification pass keeps all new operator refusal/progress explanation in the existing chainstate catalog and links it from README/index/checklist, avoiding duplicate contracts or a new runtime guide section. No new source-string checker was added.

## Deviations from Plan

None in documentation scope. Standard executor per-task commit and canonical state steps are explicitly deferred by the invoked strict-wrapper/root ownership instruction.

## Residual Boundaries

Full default verification, coverage/Bazel smoke, lifecycle-valid report and SNAP-01 activation remain pending root work. Historical surface `done` is required by the existing Phase 151 checker; wording distinguishes that foundation from canonical Pending closure rather than changing the checker or adding a duplicate owner.

The completed-prune and fresh resume/reopen evidence does not establish a global atomic payload probe/save transaction for separately exported concurrent node/store callers. New concurrent owners or stronger claims require replanning. Automatic retention is Phase 153; incremental scanning, snapshot deletion, repair, archive serving, assumeutxo, public defaults and production-funds claims remain deferred.

## Stub and Threat Review

No blocking placeholder or unwired implementation was introduced; these are documentation changes. Planned T-152-13 through T-152-16 are addressed by unique historical/closure ownership, truthful observed results and Pending status, sanitized failure facts, concrete fixture links, accurate Knots distinction and preserved scope. No new endpoint, auth, schema or file-access trust surface is introduced.

## User Setup Required

None.

## Self-Check: PASSED

All four owned documentation files and this summary exist. Index JSON and
summary YAML parse, unique ownership and lifecycle metadata match, the
completion array was empty at executor handoff, and exact README claim/deferred text was
compared against HEAD without changes. Scoped checker results are recorded
above. Commit-existence checks are deferred under the explicit no-commit
override; executor handoff did not claim a commit hash or root-verifier success.

## Root Verification Closeout | 2026-10-03

The default `bash scripts/verify.sh` completed with exit 0 in 34m 46.545s,
including workspace lint/build/tests, benchmark smoke, Bazel and pure-core
coverage. The [phase verification](152-VERIFICATION.md) passed 11/11 goals;
the required lifecycle CLI returned valid with no reasons. Clean code
review and 17 closed threat dispositions support the scoped closure.
SNAP-01 is now activated by this completing summary and mapped Complete to
Phase 152. Phase 153's two retention requirements remain Pending.
Git finalization is performed by the strict wrapper after this evidence.
