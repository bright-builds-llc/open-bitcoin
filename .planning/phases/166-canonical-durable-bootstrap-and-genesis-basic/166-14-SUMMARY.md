---
phase: 166-canonical-durable-bootstrap-and-genesis-basic
plan: "14"
subsystem: verification
tags: [canonical-genesis, native-verification, retained-failures, bounded-evidence]
requires:
  - phase: 166-13
    provides: Accepted bounded operator guidance and corrected physical-path setup evidence
provides:
  - Retained actual full-verifier failures and separately bound component qualification
  - Actual complete default native success on the final reviewed source
  - Finite source, log and owned-process closure receipts
affects: [phase166-final-acceptance]
tech-stack:
  added: []
  patterns: [serial native verification, explicit source binding, preserved failed attempts]
key-files:
  created:
    - .planning/phases/166-canonical-durable-bootstrap-and-genesis-basic/166-14-SUMMARY.md
    - scripts/check-panic-sites.ts
    - scripts/check-panic-sites.test.ts
  modified:
    - docs/metrics/lines-of-code.md
    - scripts/check-phase163-provenance/native.test.ts
    - scripts/check-phase165-shared-exact-work/test-support.ts
    - scripts/check-phase164-canonical-identity/claims.ts
    - scripts/check-phase164-canonical-identity/test-modules.ts
    - packages/open-bitcoin-chainstate/src/engine/canonical_genesis.rs
    - packages/open-bitcoin-chainstate/src/engine/canonical_genesis/tests.rs
key-decisions:
  - "Component passes remain bound to their actual source and never replace the complete default native contract."
  - "Preserve failed command outcomes and failed ignored audit outcomes separately."
  - "Root owns independent acceptance records, shared metadata and normal-hook saving."
patterns-established:
  - "Fresh pure-core coverage retains the raw report and distinguishes selected packages from observed report paths."
requirements-completed: [CBOT-02, CBOT-03, CWOR-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 166-2026-10-18T22-36-40Z
generated_at: "2026-10-21T11:07:47Z"
commits: false
status: complete
tasks: 3
tasks_completed: 3
duration: "121 min 23.397 sec whole native execution; multi-day diagnostic and review evidence retained"
completed: 2026-10-21
---

# Phase 166 Plan 14: Native Verification and Final Acceptance Summary

**The complete default native verifier passed all stages on the final reviewed source, with fresh retained coverage and all earlier failed attempts preserved.**

## Completion Boundary

All three executor tasks are complete: the full native contract passed on the final approved source, independent code/security/goal reviews passed, and this canonical summary returns the verified candidate to the strict wrapper. Root retains shared metadata reconciliation, final lifecycle/schema checks and normal-hook saving. No executor commit, normal-hook outcome, push or phase transition is claimed.

| Task | Actual work retained | Remaining acceptance |
| --- | --- | --- |
| 1. Complete native contract and generated changes | Complete: actual whole manual05 zero across all 184 serial native stages, fresh LOC, workspace/CLI tests, benchmarks, Bazel/provenance and clean retained coverage; four prior failed whole attempts remain retained | Done criteria earned on the final approved source; root owns any retest decision for later material changes |
| 2. Independent code, security and goal verification | Complete: final CODE is clean, SECURITY closes all 42 threat dispositions with zero open findings, and canonical goal verification passes all 54 obligations | Root owns final lifecycle reconciliation after this sixteenth canonical summary exists |
| 3. Truthful summary and wrapper handoff | Complete: this canonical summary retains actual failures, dated passes, residual limits and wrapper ownership, returning a concrete verified candidate | Root performs its closing shared-record/lifecycle/schema checks and separately observes normal-hook saving |

## Source and Environment Boundaries

Latest approved source has 113 paths, aggregate SHA256 `861a298d938014739c0f311a292e270bf268c10e6e635d7ad4520195ef516535`. The explicit basis is `.local/open-bitcoin-dev/phase166/plan14/pure-genesis-coverage01/candidate-source-basis113.json`. Manual04's post-format and terminal audits independently retain every raw digest and show no reviewed source change. Canonical source review was bound to the same list and aggregate before execution.

The earlier complete workspace test pass used a different 113-path snapshot, SHA256 `e0bd2fb6898655fab69032bc0f56a8a1f1c78ebf3e71698b70cfce19dd2eff4e`. It remains historical component evidence after the later two-file pure-core repair. Fresh whole-workspace evidence on the latest snapshot is supplied by manual05 below.

All authorized native attempts use pinned Bun, a physically canonical process TMPDIR, the isolated phase Cargo target and private per-attempt parity/benchmark reports. Full native invocation is exactly bash scripts/verify.sh, without fast/skip/deadline modifications. Ad hoc Cargo/Bazel commands use the timing wrapper and shared cooperative lock serially. Quiet yields were polled; commands were not interrupted merely for duration. No claim of perfect historical polling cadence is made.

## Automated Evidence

### Whole default native attempts: actual failures

| Attempt | Actual terminal result and scope | Retained receipt |
| --- | --- | --- |
| manual01 | Exit 1; Phase163 native 158 passed, two failed, 160 tests. Two grouped historical checkout children reached their existing deadlines. Later phases and Rust/benchmark/Bazel/coverage were not reached. | `.local/open-bitcoin-dev/phase166/plan14/manual01/audit.json` |
| manual02 | Exit 1; repaired Phase163 native 178 passed. Phase164 had 1183 passes and seven copied inverse syntax failures. Later Phase165/Phase166/Rust/benchmark/Bazel/coverage were not reached. | `.local/open-bitcoin-dev/phase166/plan14/manual02/audit.json` |
| manual03 | Exit 1; fresh Phase164 1190, Phase165 2020 and Phase166 490 passed. Later Phase123 baseline produced 39 passes and 63 failures. Its CLI and the remaining suffix were not reached. | `.local/open-bitcoin-dev/phase166/plan14/manual03/audit.json` |
| manual04 | Exit 1, no signal; Phase163 native finished with 176 passes, two failures, 178 tests and 1654 assertions in 518.46 seconds. Later Phase163 CLI, Phase164/Phase165/Phase166, remaining legacy guards, Rust, benchmarks, Bazel and fresh coverage were not reached by this attempt. | `.local/open-bitcoin-dev/phase166/plan14/manual04/terminal-audit.json` |

Manual04 ran from `2026-10-21T08:01:38.768Z` through `2026-10-21T08:41:30.706Z`, lasting 2391.929977 seconds. Its raw native log SHA256 is `f463b0df7b2b2874867ca0a3ce92f05d349a2203f6981a0974ebacedcfd9024f`. Formatting and LOC generation both exited zero before native execution. The default verifier's own footer records 39 minutes, 51.130 seconds.

The two top-level failures are historical native builders executing the audit-in-progress and final layouts from next-active checkout controls. The audit child printed a nested 30000 ms test timeout and a Phase158 checker SIGTERM; its parent failed after 31.77 seconds. The final control printed an outer spawn timeout after 56.21 seconds. The nested agreement failure is not a third top-level failed case. Root accepted two subsequent exact original-budget diagnostic passes without a reproduced cause. The failed attempts remain failed; unchanged-source manual05 later completed successfully. No causal repair is invented for manual04.

Manual04's terminal receipt confirms unchanged approved source and native bindings, all recorded owned PIDs absent, and the exact log-printed temporary root absent. Three other nonrecursively listed temporary prefix entries were recorded without established ownership or deletion. The optional coverage observer was never launched because the run did not reach Rust/coverage.

Root owned the fresh official manual05 run, its polling, process lifecycle and read-only coverage observation. Its authoritative native receipt records actual exit zero; the complete audit is below. Neither targeted diagnostic passes nor manual05 success relabel manual04.

### Earlier RED outcomes and component interruptions remain retained

The earlier complete Phase166 claim-control result with ten failures remains failed evidence; later repairs do not relabel it. The legacy checker baseline batch produced twelve successful exits and five refusals before compatibility repairs. All focused REDs, initial assertion/anchor harness errors and their later reviewed corrections remain in their original ignored receipts and root's PLAN-CHECK history.

| Component attempt | Actual failure boundary | Later qualification boundary |
| --- | --- | --- |
| suffix01 | Original step 32 current-documentation tests: 19 passed, one failed | Root-reviewed one-literal fixture-anchor correction; complete 20-test/45-assertion result passed separately |
| suffix02 | Original step 38 panic policy: exit 1, 192 reported sites across four paths | Root-reviewed scanner/fixture-authority repair and negative controls; later actual shell check passed |
| suffix03 | Full workspace Cargo test exit 101 at checkpoint's static source needle; 24 target footers, including a daemon target with 117 passed, one failed, one ignored | Focused repaired case and whole checkpoint module passed; later full workspace pass is separately bound below |
| remaining01 | Strict artifact gate passed; actual black-box harness had four passes and one failure | Independently reviewed expected display repair; separate whole five-test harness passed |
| remaining02 | Both startup integration harnesses passed; library compile exited zero but test-harness selected executable identity and bytes differed from immutable original map | Strict fallback was applied; neither selected library nor doctests executed through this failed continuation |
| suffix04 ignored scope audit | Actual full Cargo command exited zero, but the audit incorrectly required one footer per doctest package | Corrected read-only attribution proves all actual targets/packages; original failed postprocessor result remains preserved |
| suffix05 fresh coverage | Exact eight-package command exited zero but its retained report contained one uncovered propagation line | Root-reviewed two-file pure-core simplification and fresh clean coverage passed separately on the latest source |

Detailed results remain at `.local/open-bitcoin-dev/phase166/plan14/suffix-preflight/`, `.local/open-bitcoin-dev/phase166/plan14/cargo43-remaining/` and root's named diagnostic receipts. No failed original artifact map, run result, inverse destination or immutable source snapshot was repinned by this executor.

### Actual complete workspace component pass on the earlier source

Suffix04 ran the full native Cargo workspace/all-features test command, with no package exclusions, test-case filters or deadline changes. Actual exit was zero. The log contains 29 whole harnesses and thirteen doctest package headers. Chainstate emits two successful doctest batches, so there are 29 harness footers plus fourteen doctest footers: 43 total. Every raw footer is attributed and successful, totaling 4957 passed, zero failed, six ignored and zero filtered tests. Zero-case doctest packages remain recorded as zero.

The corrected proof is `.local/open-bitcoin-dev/phase166/plan14/suffix-preflight/suffix04/corrected-workspace-scope-and-closure.json`; raw log SHA256 `2fc5562c39e5554d7320ad18c4429034f4b3c6b9d9a264b2df3d34b9a41cfa94`. Independent review and root accepted this attribution. The original postprocessor's failed result is retained. This pass belongs to the earlier source aggregate stated above.

Suffix05 original steps 44 through 50 all exited zero: CLI RPC-client binary tests, benchmark listing, benchmark smoke, benchmark report check, Bazel build, Bazel provenance and fresh coverage clean. Its original step 51 command exited zero but the missing-lines gate failed at `packages/open-bitcoin-chainstate/src/engine/canonical_genesis.rs:44`. The failed report SHA256 is `8ec29f0386f926f609ece96fd1a885053df4b51fff5cebf3472b7cc964bfce81`; `.local/open-bitcoin-dev/phase166/plan14/suffix-preflight/suffix05/finite-audit.json` preserves source/log/report identity and finite closure. The failed coverage gate is not treated as a native pass.

### Latest-source bounded repair and clean coverage component pass

Independent proof analysis and root review approved replacing an impossible-error genesis constructor call with the existing infallible constructor. The sealed canonical proof retains its null-parent guarantee; complete coins-snapshot propagation and all three admission checks remain. An explicit null-parent assertion was added inside the existing all-four-selector test. No coverage exclusion, unsafe proof forging, capability widening or DATA/inverse renewal was introduced by this repair.

`.local/open-bitcoin-dev/phase166/plan14/pure-genesis-coverage01/final-verification.json` records zero exits for fresh formatting, strict workspace Clippy with all targets/features and denied warnings, all-target/all-feature build, the nine chainstate tests, forty consensus chain-spec tests, current phase checkers, managed checks, whitespace check, clean coverage and exact eight-package coverage. The two focused test groups retain their actual filtered counts; they are not whole workspace passes.

Fresh coverage has 1832 passing test cases with no failed/ignored/filtered cases in its sixteen harness footers, no `Uncovered Lines` heading, and a nonempty retained raw report. Report SHA256 is `fd042b00ee47b1649adcb598fe14a7cf06a01cbde613f9c2649cb1cf34c2e600`. Selected package argv and observed source headers are recorded separately in `.local/open-bitcoin-dev/phase166/plan14/pure-genesis-coverage01/coverage-footprint.json`; selected packages alone do not establish executable coverage rows. This bounded pass remains separate from failed manual04.

### Official whole manual05 success on the final source

The original exclusive-log runner performed formatting, fresh worktree LOC generation and exact default bash scripts/verify.sh. All three commands exited zero without a signal. The authoritative native receipt records start `2026-10-21T08:54:10.022Z`, finish `2026-10-21T10:55:33.413Z`, and duration 7283.397 seconds, rounded here from the full-precision JSON receipt. The root command session closed zero. The verifier's own completion footer reports two hours, one minute and 22.565 seconds; the wrapper receipt includes its own surrounding overhead.

`.local/open-bitcoin-dev/phase166/plan14/manual05/terminal-audit.json` reports `passed: true` across all 21 finite checks. All 113 raw source digests and their aggregate match the approved latest source; post-format raw digests and all eight native binding files remain exact. It retains the original command/environment, every log digest, and observations showing all recorded owned PIDs absent. No claim is made that a persistent Bazel service must exit or that every historical descendant was individually observed.

All 184 actual `verify-step` timing receipts are successful, exit zero, signal-free, serial, in full mode and on the isolated target. Their exact receipt paths and digests are retained in `.local/open-bitcoin-dev/phase166/plan14/manual05/native-stage-receipts.json`. The native log SHA256 is `76fabae24b50d48cb7a32cac53b255e3447b87239862f29cc491ea2283ec6db7`. The audit checks the actual ordering of formatting/shared-fixture formatting, strict workspace Clippy with all targets/features and denied warnings, all-target/all-feature build, workspace tests, the extra CLI binary, benchmark checks, Bazel/provenance, coverage clean and fresh pure-core coverage. Earlier guard stages all completed in this same successful run.

| Actual Rust test scope | Retained groups and footers | Actual counts |
| --- | --- | --- |
| Whole workspace with all features | 29 whole harnesses plus thirteen doctest packages; 43 footers because chainstate has two doctest batches | 4957 passed, zero failed, six ignored, zero filtered |
| Additional CLI RPC-client binary | One whole harness and one footer | Seven passed, zero failed, zero ignored, zero filtered |
| Fresh exact eight-package pure-core coverage | Sixteen whole harnesses and sixteen footers | 1832 passed, zero failed, zero ignored, zero filtered |

Every one of the sixty raw Rust result footers is attributed exactly once to the table above. Counts are retained separately; overlapping coverage execution is not presented as additional unique tests. Zero-case doctest packages remain zero.

The read-only observer bound native owner PID 25556 to actual coverage command PID 85250 and retained the exact report descriptor identity. It completed with `captured: true` and `nativeExit: 0`. The observation was bound to this new attempt, not an earlier PID. The retained report is `.local/open-bitcoin-dev/phase166/plan14/manual05/coverage-report.txt`, 2,438,922 bytes, SHA256 `589fbac073c775c7cbd285c964c74a6c27e022006fda9b675408ef0fdbf9db41`, with no `Uncovered Lines` heading. Exact ordered eight-package argv, fresh clean timing, conservative observed source headers and complete raw text are retained separately in the terminal audit; requested packages do not themselves prove executable coverage rows.

Benchmark listing, smoke execution and report validation all succeeded. The actual smoke report contains eleven groups and fifteen cases in one-iteration, threshold-free mode; its SHA256 is `16fea7e0bc92e03533c891919fb2b4dd9343819d458394062cde45ec10de658b`. Bazel build and provenance each have successful timing receipts, build-success markers and the actual `Bazel build provenance check passed.` log marker. These are evidence of this full run, not predictions about the eventual normal hook.

## Reviewed Deviations and Simplification

Root re-planned after concrete failures rather than silently relaxing acceptance. Scoped diagnosis/review covered per-layout native smoke isolation without larger deadlines, literal copied-source replacement, repeated current-heading claim checks, physical-path setup documentation, legacy guard compatibility, old fixture anchors, explicit fixture panic policy, checkpoint source needles and black-box hash display expectations. DATA changes were separately restricted to reviewed existing ownership values, with exact immutable originals/inverse destinations retained.

The pure-core repair removed an unreachable fallible branch at a sealed proof boundary instead of excluding it from coverage. Existing negative admission tests and proof-construction constraints remain. These reviewed source changes belong to their recorded owner; this executor did not make unilateral product or source fixes. Root's simplification review and subsequent scoped reviews remain supporting evidence, not full-native proof.

## Independent Acceptance and Remaining Limits

Canonical `166-REVIEW.md` is clean on the approved 113-path source. Canonical `166-SECURITY.md` is passed: all 42 threat dispositions are closed, comprising 39 verified mitigations and three explicit accepted risks, with zero open or unregistered flags and no unresolved high-severity finding. Canonical `166-VERIFICATION.md` is passed at 54/54 obligations: all fifty PLAN truths and four mandatory roadmap criteria, with requirements CBOT-02, CBOT-03 and CWOR-04 verified. Those reports reconcile actual source and receipts, rather than treating this summary as implementation proof.

The lifecycle remains sixteen plans and 36 tasks. At this summary's write boundary, the goal report intentionally records `lifecycle_validated: false` pending existence and same-origin validation of this sixteenth canonical summary. Root and the independent verifier own that final check; its later result is not predicted here. Functional goal, code and security acceptance are earned, while root's final lifecycle/shared-record reconciliation and normal-hook saving remain next steps.

The feature remains bounded to selected-network proved genesis, explicit BASIC support and unavailable wallet capability. Ordinary history/network activation remains refused; unattended full sync and actual pinned-daemon differential qualification remain deferred. Supported application leases/engine locks do not claim exclusion of arbitrary direct filesystem or Fjall writers. The exact accepted legacy fixture-only panic boundary remains explicit in the plan threat register and standards override.

## Strict-Wrapper Handoff

Task-level acceptance is earned, and this summary records `commits: false` under the strict-wrapper override. No task, RED/GREEN or summary commit was created by this executor. Root alone stages and saves the concrete final candidate.

Root next reconciles the active task block, STATE, roadmap, requirements, catalog and generated metadata; validates all sixteen summaries and the final lifecycle; and checks schemas, artifacts and key links. The independent goal verifier owns the pending lifecycle flag update after the actual sixteenth summary is validated. Any later material source change receives the required retest before saving.

The mandatory normal pre-commit hook still runs the complete default verifier on root's final staged candidate. Root alone observes that actual outcome, commits and pushes through the strict wrapper, verifies the actual remote result, and stops before the next phase. No future hook, commit or push result is claimed.

## Self-Check: PASSED

- This canonical summary exists with all three tasks complete, the three verified requirement IDs, `commits: false` and the originating yolo lifecycle identity.
- Manual05's actual whole default native exit zero is authenticated by all 21 finite audit checks on the unchanged approved 113-path source and all 184 successful stage receipts.
- The sixty Rust result footers are fully attributed to workspace, additional CLI and fresh coverage; the retained fresh coverage report has no uncovered-lines heading.
- Canonical CODE is clean, SECURITY is passed with all 42 threat dispositions closed, and functional goal verification is passed at 54/54 obligations.
- Automated Evidence retains the complete historical receipt record, corrected audit attribution and original source boundaries.
- The handoff records root's remaining lifecycle/shared-record and normal-hook saving responsibilities without claiming their future outcomes.
