---
phase: 162-real-prune-retention-and-integrated-parity-proof
status: fixed-scoped-checks-passed
generated_by: gsd-debugger
phase_lifecycle_id: 162-2026-10-11T16-25-37
lifecycle_mode: yolo
generated_at: 2026-10-12T01:03:30Z
source_digest: a4b0d71d01def301ba406d6b46c351ed8944d8d4508182ffb0dfd21960043575
requirements-completed: []
---

# Phase 162 Native Phase 80 Guard Fix

The default native attempt exposed a Phase 80 path-prefix false positive. The guard now accepts both explicitly required shared-fixture formatting commands while continuing to refuse literal `/proc` directory references and descendants.

## Scope and Guidance

Only `scripts/check-phase80-opt-in-soak-uat-release-boundaries.ts` and its `.test.ts` source changed. This report and the ignored `.planning/debug/phase162-native80-proc-prefix.md` are the owned debugging artifacts. No fixture, oracle, production, shared helper, native script, registry, state, requirement, manifest or dependency changed in this fix.

Actual `AGENTS.md`, the Bright Builds sidecar and overrides, managed architecture/code-shape/testing/verification/local-guidance/TypeScript standards, GSD debug skill, common bug patterns, thinking models and knowledge base informed the work. Phase 162 CONTEXT and Plans 11/12 summaries establish the legitimate documented and executed rustfmt routes. Phase 80 CONTEXT D-03 forbids process scans. Both active lesson files were fully read: global 5,230 bytes/1,744 estimated tokens and repo 1,958 bytes/653 estimated tokens, combined 7,188 bytes/2,397 estimated tokens. The supplied parent synchronized baseline and lesson-audit baseline remain authoritative; no child sync, installation or audit-file mutation occurred.

Every Bun invocation used `PATH=/tmp/open-bitcoin-bun-1.3.9-fresh:$PATH`, actual Bun 1.3.9. Rust remains the pinned 1.94.1; this child ran no Rust, Cargo, Bazel or full-native command.

## Failure and Cause

Parent's actual `/tmp/phase162-native-full-1.log` records `bash scripts/verify.sh` exiting 1 after 6,677 ms before Cargo. The original six Phase 80 fixture tests passed, then the live checker emitted:

```text
scripts/verify.sh must not contain Phase 80 forbidden text: /proc
```

The live checker independently reproduced that exact sole diagnostic and exit 1 before the fix. `FORBIDDEN_VERIFY_STRINGS` contains `/proc`, and the original `requireNotContains` used `String.includes`. Both `scripts/verify.sh` lines 197 and 394 contain the required `packages/testdata/phase162/process.rs`, whose filename prefix includes `/proc`. Neither line performs a process filesystem scan.

Before changing the checker, two parameterized positive regressions used the exact documented rustfmt and executed `run_step` forms. Both failed with that same diagnostic: **0 pass, 2 fail, 2 expectations, 101 ms**, with 51 other cases filtered. These expected RED reproductions are not failed fix attempts.

## Minimal Fix and Retained Policy

Only the `/proc` entry uses this expression:

```text
/\/proc(?![A-Za-z0-9_.-])/
```

Only a safe literal filename continuation character `[A-Za-z0-9_.-]` suppresses the `/proc` match. `/proc`, `/proc/`, `/proc/self`, `/proc/self/status` and `/proc/123/status` remain forbidden, including quoted/redirection/control forms and shell expansion continuations. `/process.rs`, `/processing.rs`, `/proc.rs`, `/proc2`, `/proc_status`, `/proc-status`, `/processor/status`, `/procfile`, `/procfs/status` and `/proc.d/status` are different path components and are accepted. Unknown punctuation is conservatively refused, retaining the original substring guard's refusal outside known safe filename continuations.

The ordered forbidden-string list remains byte-for-byte unchanged, and all other 18 entries retain substring matching and their original diagnostics. Required Phase 75–80 commands/order/Phase 80 adjacency, live-network/service/multi-day/default exclusions, claim restrictions and forbidden-manifest checks remain unchanged. No allowlist for one fixture, blanket waiver, shell execution or new parser was added. This remains a textual policy guard, with explicit tested boundaries, rather than a general shell semantic analyzer.

The simplification pass retained one small local predicate in the existing verifier loop. There is no new helper interface, dependency, global policy change or renamed fixture. All six prior tests remain unchanged.

## Independent Review and Concrete Replan

The first correction enumerated following slash/end/whitespace/quote/control delimiters. Its 53 tests passed, but independent tooling review WR-02 (P2) demonstrated that the exported checker returned no failures for `/proc{,/self/status}`, `/proc${tail}` and `/proc*`. Harmless Bash printf independently demonstrated actual process-path expansion. Native attempt 2 remained held. The first correction was therefore insufficient.

The hypothesis was revised before further editing: conservatively allow only known literal filename continuations rather than enumerate Bash metacharacters. Eight exported-checker negative controls for braces, glob star, character class, variable expansion, dollar command substitution, backtick substitution, escaped slash and question glob ran before changing the predicate. Actual RED: **1 pass, 7 fail, 9 expectations, 119 ms**, 54 filtered; backtick substitution was already protected. The one corrected attempt uses the expression above. All prior 53 cases remain, with one additional benign `procfs` case and all eight continuation negatives. No test was weakened or deleted. The new source hashes were frozen for parent independent recheck before any full-native rerun.

## Executed Checks

- Initial focused native-format GREEN: **2 pass, 0 fail, 4 expectations, 24 ms**, 51 filtered. Both exact forms also pass in the final complete run.
- Final shell-continuation GREEN: **8 pass, 0 fail, 16 expectations, 59 ms**, 54 filtered.
- Final complete Phase 80 suite: **62 pass, 0 fail, 125 expectations, 230 ms**. This comprises the original 6 plus 2 exact native formatting forms, 10 neighboring-path positives, 18 process-directory negatives, 8 shell-continuation negatives and 18 other forbidden-string negatives.
- Actual live Phase 80 checker: exit 0, expected validated release-boundary message.
- Actual live Phase 158/159/160/161/162 checkers: all exit 0. Phase 162 still checks 208 fixed function contracts and the existing claim lifecycle.
- Managed `bun scripts/bright-builds-check.ts all`: exit 0; 1,463 files scanned, 0 findings.
- Owned `git diff --check`: passed. Reviewed complete owned diff: 7 net checker lines and 137 added test lines; no unrelated source changes.
- Report GFM/frontmatter formatter check: passed after a scoped format-only correction; no other Markdown path formatted.
- Actual ordered six-source SHA256 digest recomputed from exported `PHASE162_EVIDENCE`: unchanged `a4b0d71d01def301ba406d6b46c351ed8944d8d4508182ffb0dfd21960043575`.

The final independent GREEN/live/managed calls ran concurrently after the corrected fix. Measured orchestration elapsed times were focused 121 ms; live 80:87 ms, 158:199 ms, 159:686 ms, 160:342 ms, 161:205 ms, 162:945 ms; managed 436 ms; hash/diff 95 ms. Those include tool dispatch and are distinct from Bun's reported test durations above.

## Final Source Hashes

- `scripts/check-phase80-opt-in-soak-uat-release-boundaries.ts`: `ba2d27b4f59fe0abb313dafadb4e6761a60dec9bed308ca649aa878da81abeb6`
- `scripts/check-phase80-opt-in-soak-uat-release-boundaries.test.ts`: `f7dde6ee85f300029760846b179869e9f213dd562251893ce8ebb1f82620cf80`

## Parent Gates and Deferred Finalization

All three requirements remain Pending and their owner remains `in_progress`. The independent supplemental review of these two new guard paths and the parent's fresh default native attempt are required next. This report does not claim a successful full-native, formal lifecycle, milestone audit/shipment or Git gate. Parent owns updated complete review hashes, report registration, generated LOC freshness and all later activation.

Phase 162 D-14 overrides generic debug finalization: no staging, commit, push, branch change or hook bypass occurred. No human question or external action is needed for this deterministic scoped fix. The ignored debug session remains pending parent verification; no shared knowledge-base entry was appended and no other session was moved.
