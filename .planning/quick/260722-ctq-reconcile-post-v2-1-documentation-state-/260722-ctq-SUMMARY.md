---
phase: quick-260722-ctq-reconcile-post-v2-1-documentation-state
plan: "01"
subsystem: documentation
tags: [parity, release-readiness, bun, verification, rpc]
requires:
  - phase: v2.1-milestone-archive
    provides: archived milestone state and final audit evidence
provides:
  - reconciled shipped-and-archived v2.1 current documentation
  - bounded v2.0 transaction-relay preview classification
  - exact SupportedMethod RPC catalog reconciliation
  - deterministic current-documentation checker and mutation suite
affects: [future-milestone-planning, parity-docs, verifier-ordering]
tech-stack:
  added: []
  patterns: [fixed-corpus local checker, section-aware Markdown parsing, exact set reconciliation]
key-files:
  created:
    - scripts/check-current-documentation-reconciliation.ts
    - scripts/check-current-documentation-reconciliation.test.ts
  modified:
    - README.md
    - .planning/ARCHITECTURE.md
    - .planning/CONVENTIONS.md
    - docs/parity/release-readiness.md
    - docs/parity/support-matrix.md
    - docs/parity/production-claim-boundary.md
    - docs/parity/deviations-and-unknowns.md
    - docs/parity/catalog/rpc-cli-config.md
    - scripts/verify.sh
    - docs/metrics/lines-of-code.md
key-decisions:
  - "Classify only bounded, explicit, default-off v2.0 transaction relay as preview; keep public/default/production relay deferred."
  - "Treat SupportedMethod serde names as the exact RPC catalog authority and keep richer send and multiwallet lifecycle semantics deferred."
  - "Retain the Phase 82 legacy broad-claim row label for checker compatibility while explicitly limiting that row to public/default/production claims."
patterns-established:
  - "Current-state documentation checks parse named sections and exact table rows so historical milestone prose remains legal."
  - "Verifier wiring assertions require the documentation test/check pair immediately after the final Phase 117 gate."
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: direct-fallback
phase_lifecycle_id: quick-260722-ctq
generated_at: 2026-07-22T14:47:39Z
duration: 30min
completed: 2026-07-22
---

# Quick Task 260722-ctq: Post-v2.1 Documentation Reconciliation Summary

**Current project documentation now matches the shipped-and-archived v2.1 state, with exact relay/RPC scope protected by a deterministic 16-mutation Bun checker.**

## Performance

- **Duration:** 30 min
- **Started:** 2026-07-22T09:17:00-05:00
- **Completed:** 2026-07-22T09:47:39-05:00
- **Tasks:** 3
- **Implementation files:** 12
- **GSD evidence files:** 3

## Accomplishments

- Reconciled current documentation to the 2026-07-22 v2.1 archive with final 39/39 requirements, 20/20 phases, 13/13 integration links, and 11/11 flows, routing future work only to `/gsd-new-milestone`.
- Classified bounded, explicit, default-off v2.0 transaction relay as `preview` while preserving deferred public/default/production relay and all production non-claims.
- Made both RPC catalog lists equal the 20 `SupportedMethod` serde names and documented the implemented `sendtoaddress` and `-rpcwallet` subset without claiming richer send or multiwallet lifecycle parity.
- Added a local-filesystem-only checker with 16 focused Arrange/Act/Assert mutation tests and wired the pair immediately after the final Phase 117 gate in both verifier lists.
- Refreshed LOC evidence to 577 included files and 248,710 total lines, including both new TypeScript files.

## Task Commit

Tasks 1 through 3 and their plan, summary, and verification evidence landed in the required single local commit, `docs: reconcile post-v2.1 project state`.

No push was performed. `.planning/STATE.md`, `.planning/ROADMAP.md`, and the protected milestone-history artifacts were left unchanged.

## Verification Evidence

- LOC generation and worktree freshness check: passed; report includes the two new TypeScript files.
- Phase 83 checker: 10/10 mutation tests and live command passed.
- Phase 88 checker: 7/7 mutation tests and live command passed.
- Phase 106 checker: 8/8 mutation tests and live command passed.
- Phase 117 checker: 26/26 mutation tests and live command passed.
- Phase 124 checker: 88/88 mutation tests and live command passed.
- Current documentation reconciliation: 16/16 mutation tests and live command passed.
- Final timed `bash scripts/verify.sh`: passed in 6m 36.794s, including Rust format, Clippy, all-target build, all-feature tests, benchmark smoke, Bazel smoke, coverage, parity guards, and architecture checks.
- Commit hook reran the full verification contract successfully in 7m 54.507s.
- `git diff --check` and cached diff checks passed.
- Protected hashes for `.planning/PROJECT.md`, `.planning/MILESTONES.md`, `.planning/STATE.md`, `docs/parity/index.json`, and `docs/parity/checklist.md` were unchanged.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Preserved the Phase 82 legacy broad-claim row key**

- **Found during:** Task 3 full verification.
- **Issue:** The unchanged Phase 82 checker requires the exact statement label `Open Bitcoin supports relay/inbound serving.` and exact legacy status/verification cells.
- **Fix:** Retained the required label and cells while explicitly limiting the row's evidence and residual-risk wording to the broad public/default/production claim; the new bounded v2.0 preview row remains separate and authoritative.
- **Files modified:** `docs/parity/production-claim-boundary.md`
- **Verification:** Phase 82 tests/live command, the new checker pair, both final full verifier runs, and the commit hook passed.
- **Committed in:** the final single local commit

**Total deviations:** 1 auto-fixed blocking compatibility issue.
**Impact on plan:** No scope expansion or runtime change; the narrow relay claim remains accurate and historical guard compatibility is preserved.

## Known Stubs

None.

## Threat Review

No new network, authentication, schema, process-execution, or write-capable trust boundary was introduced. The checker performs synchronous reads of fixed repo-relative files only, emits bounded fact labels, and has no network or subprocess access.

## Residual Risks

- The Phase 82 guard still keys on a legacy broad-claim statement label; the row now explains its restricted meaning, and the new checker separately enforces the bounded-preview versus broader-deferred distinction.
- The checker intentionally validates current named sections and fixed tables rather than historical milestone prose; future heading/table renames must update the checker and mutation suite together.
- No runtime behavior changed.

## Self-Check: PASSED

- The final commit has the exact requested subject and contains exactly 12 authorized implementation files plus the three GSD quick-task evidence files.
- Both created checker files and all modified documentation/verifier/LOC files exist.
- Protected planning/parity roots remain unchanged.
- The worktree is clean and the commit remains local and unpushed.
