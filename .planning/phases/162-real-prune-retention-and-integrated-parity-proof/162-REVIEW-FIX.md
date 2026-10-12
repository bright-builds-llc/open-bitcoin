---
phase: 162-real-prune-retention-and-integrated-parity-proof
fixed_at: 2026-10-12T00:48:54Z
review_path: .planning/phases/162-real-prune-retention-and-integrated-parity-proof/162-REVIEW.md
iteration: 1
fix_scope: critical_warning
findings_in_scope: 1
fixed: 1
skipped: 0
status: all_fixed
generated_by: gsd-code-fixer
lifecycle_mode: yolo
phase_lifecycle_id: 162-2026-10-11T16-25-37
generated_at: 2026-10-12T00:50:37Z
source_digest: a4b0d71d01def301ba406d6b46c351ed8944d8d4508182ffb0dfd21960043575
requirements-completed: []
commits: deferred
---

# Phase 162: Code Review Fix Report

**Fixed at:** 2026-10-12T00:48:54Z
**Source review:** 162-REVIEW.md; original standard review covered 65 files and reported one Warning/P2, zero Critical and zero Info.
**Iteration:** 1; critical_warning scope.
**Summary:** One finding fixed; zero skipped. Source repair is verified below. Parent subsequently confirmed clean independent tooling and consolidated 65-file reviews closing WR-01; full default native verification, formal lifecycle verification and Git finalization remain parent gates.

## Fix Plan and Guidance

- [x] Reproduce supplied-map checkout borrowing and add focused regression controls before editing the checker.
- [x] Carry explicit supplied-map inventory into breadcrumb discovery/loading, preserving ordinary filesystem mode.
- [x] Verify focused controls, the complete Phase 159 suite, live 158–162 guards, managed checks and the exact scoped diff.
- [x] Record actual failures, final receipts and hashes; defer all commits under D-14.

Material guidance: actual AGENTS.md/Repo-Local Guidance, AGENTS.bright-builds.md, placeholder-only standards-overrides.md, standards/index.md and architecture/code-shape/verification/testing/local-guidance/TypeScript pages; GSD code-review-fix skill/workflow and fixer role; Phase 162 CONTEXT D-11/D-13/D-14, final 10/11/12 summaries and both primary/tooling reviews. Actual affected checker/tests and their discovery/optional-loader contracts were read before edits.

Both canonical active lesson inputs were read completely: global 5,230 bytes plus repository .codex/tasks/lessons.md 1,958 bytes, total 7,188 bytes and 2,397 conservative estimated tokens. The existing lesson-audit baseline has no new trigger. No prescribed project skill directories were present. Parent supplied the synchronized main/pinned Knots context; no checkout sync, bootstrap or dependency installation was needed for this isolated repair.

## Fixed Issues

### WR-01: Supplied Phase 159 source maps still borrow the real checkout

**Status:** fixed: requires human verification (logic/state-mode finding). Semantic regressions pass; parent subsequently confirmed clean independent re-review closing this finding.
**Files modified:** `scripts/check-phase159-filter-rpcs.ts`, `scripts/check-phase159-filter-rpcs.test.ts`.
**Commit:** Deferred explicitly by CONTEXT D-14 and the invoking strict wrapper; no commit hash is claimed. The report commit is also deferred.
**Applied fix:** Use `maybeSources !== undefined` to preserve explicit map mode, including an empty map. Reject missing and empty CHECK_FILES entries. Pass supplied keys into the existing breadcrumb checker; discover mapped first-party Rust keys plus mandatory NEW_RUST, and read only supplied text in that mode. Missing/empty mandatory or discovered breadcrumb text cannot borrow valid real files. The ordinary mode retains the same worktreePaths discovery, broad node/RPC/CLI scope, exact registration/anchor checks and disk fallback, in the original evaluation order.

Eight added tests cover a complete current map at both default and guaranteed nonexistent roots; explicit empty-map refusal for every required row; separate missing/empty Rust and required-tooling rows against valid real files; extra supplied-only source registration and exact anchors; and refusal of an empty mapped extra source whose real disk file is valid. The existing fixture and shared optional loader preserve current implementation and later Complete evidence handling. Existing untracked, ignored-untracked, tracked-but-ignored and Git-link discovery controls remain unchanged and pass.

The simplification pass kept one explicit mode branch and the existing breadcrumb function. No new source file, global helper, framework, dependency, production behavior, registry, config or frozen checkpoint guard was changed. Reversing only this fix in memory reproduces both exact reviewer baseline hashes, proving the original unrelated source and assertions remain byte-for-byte intact.

## Executed Verification

Every Bun child inherited `PATH=/tmp/open-bitcoin-bun-1.3.9-fresh:$PATH` and used Bun 1.3.9. No Cargo/Bazel jobs ran; Rust 1.94.1 remains the repo pin. Test commands used explicit `./` file paths and the unchanged default 5,000 ms timeout. Sessions were polled within 60 seconds and normal active CPU/log progress was observed; no command was interrupted or timeout increased.

| Control                                                                 | Actual result                                                                                                                                                |             Time |
| ----------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------ | ---------------: |
| Initial read-only reproduction                                          | Same complete map: default root `[]`; nonexistent root Git fatal plus breadcrumb inventory refusal; empty extra map row incorrectly passed by borrowing disk | 1.66 s tool wall |
| Initial supplied-map RED suite                                          | 1 pass, 7 fail; 129 expectations; actual Git fatal and missing refusal/discovery assertions                                                                  |           5.22 s |
| Focused GREEN, including original discovery and implementation controls | 13 pass, 0 fail; 144 expectations                                                                                                                            |           8.94 s |
| Final-source full Phase 159 required suite                              | **637 pass, 0 fail; 1,999 expectations; exit 0**                                                                                                             |     **364.56 s** |
| Exact reproduction after repair                                         | Default and nonexistent roots both `[]`; empty extra row returns named cannot-read-breadcrumb-evidence refusal; captured stderr exactly 0 bytes              | 1.47 s tool wall |
| Live 158/159/160/161/162                                                | All pass; 159 retains 125 selectors and 162 retains 208 contracts                                                                                            |           Scoped |
| Managed all checks                                                      | 1,463 source files scanned; zero findings                                                                                                                    |           Scoped |
| Owned whitespace/diff review                                            | Pass; ordinary-mode evaluation order and all original unrelated logic/assertions preserved                                                                   |           Scoped |

The focused command was `bun test ./scripts/check-phase159-filter-rpcs.test.ts --test-name-pattern 'supplied-map|WR06|new untracked Rust path|current RPC roots'`. The final full command was:

```bash
bun test ./scripts/check-phase159-filter-rpcs.test.ts
```

Each live checker was executed with `bun run scripts/check-phase158-validated-reorg.ts`, the corresponding 159/160/161 checker paths, and `bun run scripts/check-phase162-integrated-filter-proof.ts`. Managed verification used `bun scripts/bright-builds-check.ts all`; whitespace verification used `git diff --check -- scripts/check-phase159-filter-rpcs.ts scripts/check-phase159-filter-rpcs.test.ts`.

The first RED empty-inventory fixture additionally demanded per-Rust breadcrumb diagnostics despite also omitting the manifest JSON. That redundant diagnostic expectation was removed because the missing manifest stops breadcrumb parsing. The empty-map test still requires an explicit required-evidence refusal for every CHECK_FILES row. Separate missing/empty mandatory-Rust controls retain both required-evidence and breadcrumb refusals with a complete mapped manifest. No original negative assertion was changed or removed. No repair verification failed after implementation, no rerun was needed for a failure, and no two failed repair attempts occurred.

The focused GREEN preceded a placement-only correction restoring the original default-mode source-read position after registration; the clean full 637-test suite and subsequent live/diff checks cover that final source. Minimum verification re-read all edited sections. Bun execution verifies parsing/runtime behavior; no standalone TypeScript formatter, tsconfig, package.json or LSP diagnostics surface is configured. The separate full Phase 162 matrix was not rerun for this isolated 159-mode branch: its live guard passes unchanged, and parent full native verification will execute every required suite.

Temporary receipts: `/tmp/phase162-review-fix-repro.log`, `phase162-review-fix-red.log`, `phase162-review-fix-green.log`, `phase162-review-fix-full159.log`, `phase162-review-fix-repro-green.log`, `phase162-review-fix-repro-green.stderr`, `phase162-review-fix-live.log` and `phase162-review-fix-managed.log`, all under `/tmp/`. These are local receipts, not formal native or parity certification.

## Exact Final Sources and Preserved Publication Boundary

| Source                                       | Final SHA256                                                       |
| -------------------------------------------- | ------------------------------------------------------------------ |
| `scripts/check-phase159-filter-rpcs.ts`      | `1f873f1a1267d6d30724ae88f28d5337d589a05ede09668cd75233ba09f34ce9` |
| `scripts/check-phase159-filter-rpcs.test.ts` | `81b83551ba86011d88f68aa19f4a78e64dec3f4b869a332a311391c9328ba032` |

The in-memory inverse matches original reviewed hashes `e84ed66f7c35dbc3afee8154d581470101785587a5b1407984f595d1a847700a` and `e54fde782d7c818324f185fa7f5afc05a19706195b4e6ea3f1f671344e5d9d1c`, respectively. Final source lengths are 317 and 583 lines.

Frozen Phase 158 checker/test hashes remain `3127623de8de726f60fe03095f49b403bbfabafcb3ba677464cc521dba92cb81` and `5c6e6a63da1f00ae49d9b60d7960d84d28982959d47fd245c9625bd9fce112be`. All six published source anchors were independently rehashed in exported PHASE162_EVIDENCE order. Their newline-joined path:sha256 digest without a trailing newline remains `a4b0d71d01def301ba406d6b46c351ed8944d8d4508182ffb0dfd21960043575`.

CFPR-02, CFGR-01 and CFGR-02 remain Pending in checkboxes and traceability; phase verification remains pending. Source repair does not activate requirements, establish full native/formal completion or ship/audit the milestone. No review findings/status were edited by this fixer. Parent independently completed recheck/consolidation and owns scope/digest/LOC refresh, full default `bash scripts/verify.sh`, formal lifecycle evidence and final normal-hook Git operations.

The first new-report formatter check requested standard table padding; the report alone was then formatted with installed mdformat 1.0.0, GFM 1.0.0 and frontmatter 2.1.2 plugins. Source files were unchanged.

No staging, commit, branch change, push or hook bypass occurred in this checkout. Existing authorized Git operations remain confined to disposable test fixtures. The two source edits and this report intentionally remain uncommitted under D-14. No source rollback or unrelated-file revert occurred.

_Fixer: gsd-code-fixer; iteration 1._
