---
phase: 153-automatic-prune-retention-integration
plan: "04"
subsystem: docs
tags: [pruning, parity, operator, verification, fjall]
requires:
  - phase: 153-03
    provides: Genuine legal-target ordinary retention and production checkpoint/reopen evidence
provides:
  - Scoped automatic-retention parity and operator evidence on the existing stable surface
  - Correct production-helper corpus boundary for nested daemon test fixtures
  - Passing default native verification including genuine retention, coverage and Bazel
affects: [PRUN-01, PRUN-02, INT-01, v2.4-integration-audit]
tech-stack:
  added: []
  patterns: [historical surface plus pending canonical closure, exact test-directory corpus exclusion]
key-files:
  created: []
  modified:
    - docs/parity/index.json
    - docs/parity/checklist.md
    - docs/parity/catalog/chainstate.md
    - README.md
    - docs/operator/runtime-guide.md
    - docs/metrics/lines-of-code.md
    - scripts/check-phase127-authoritative-network-state-unification.ts
    - scripts/check-phase127-authoritative-network-state-unification.test.ts
key-decisions:
  - Preserve the unique historical prune surface while leaving canonical PRUN-01/PRUN-02 Pending for root lifecycle verification.
  - Exclude exact tests directory components from the production-helper scanner while retaining production fixtures and duplicate-authority guards.
  - Use the repo-native default verifier and record all failed attempts before the final actual exit-zero result.
patterns-established:
  - Documentation distinguishes production durable checkpoint proof from delegated metadata-fault fixtures using MemoryCoinsView.
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 153-2026-10-03T04-01-54
generated_at: 2026-10-03T07:32:03Z
duration: 77 min
completed: 2026-10-03
---

# Phase 153 Plan 04: Automatic Retention Evidence and Native Verification Summary

**Scoped parity/operator evidence now describes actual legal-target retention, and the default full verifier passes with the genuine ordinary-cycle fixture, strict workspace checks, pure-core coverage and Bazel.**

## Performance

- Execution record window: 2026-10-03T06:15:31Z through 2026-10-03T07:32:03Z; initial context loading preceded the recorded init timestamp.
- Tasks: 2/2.
- Files modified: eight documentation/generated/checker paths plus this summary. Root authorized and implemented the two checker paths after the full gate exposed their corpus-boundary bug.
- Successful default verifier: inner footer 45m47.953s (2,747,953ms), actual exit 0, exec session 43990. Timing record starts 2026-10-03T06:42:51.983Z and ends 2026-10-03T07:28:40.830Z; outer elapsed 2,748,847ms. The executor observed completion on its 2026-10-03T07:28:54Z poll, which is distinct from the recorded end time.
- Prior attempts remain recorded: stale LOC preflight failed after 187ms; the next default attempt failed at Phase 127 after 11m01.170s (661,170ms).

## Accomplishments

- Amended the existing `v2-4-pure-prune-policy-and-lock-windows` surface in place. Historical Phase 147 foundations and PRUN-03/LOCK-01 evidence remain intact. Phase 153 Automatic Prune Retention Integration is the current PRUN-01/PRUN-02 production-consumer closure owner; canonical requirements remain Pending for root lifecycle validation and integration re-audit. The historical surface's checker-required `done` status is explicitly distinguished from canonical requirement completion.
- Catalog, README and operator guidance link real accounting/owner/daemon fixture paths and pinned Knots `CalculateCurrentUsage`, `FindFilesToPrune` and `FlushStateToDisk` roots. They describe complete logical live block/encoded-undo accounting, nonactive/protected total bytes, active-only candidates, soft target, preserved keep/lock/threshold rules, ordinary cadence, serialization, full checkpoint and receipt-owned cleanup.
- Documented explicit offline prune-with-datadir recovery without network activation, existing Cargo/Bazel status/support command forms and sanitized support evidence. Both wallet adapters and in-memory serving/operator observations remain bounded to the actual tested behavior.
- Published exact genuine fixture results from Plan 03: legal 550 MiB = 576,716,800 bytes; initial actual retained total 578,359,864; 235 large nonactive pairs contribute 578,358,970 bytes, while six small active pairs contribute 894. First ordinary deletion removes heights 1/500/713 and 447 bytes, retains 510/714/1001 and leaves 578,359,417 bytes above target. Protected/nonactive bytes explain the unreachable soft target. The final focused scenario took 17.891221375s; this is prior observed focused evidence, not a duration extracted from the default run.
- Kept the first production durable checkpoint/reopen proof separate from the later `MetadataFaultStore` stage. That later stage delegates real accounting, locks, payload mutations, paired unlink and counters while injecting metadata persistence refusal and using a fixture owner with `MemoryCoinsView`. It is not hardware failure or a second production durable-coins checkpoint proof. Sparse codec-valid history establishes retention integration, not continuous consensus-chain acceptance, physical disk caps or public-network/funds readiness.
- Refreshed the intentionally tracked LOC report to 355,534 counted lines. Its final worktree freshness check passes.

## Pending Atomic Finalization Records

D-10 overrides ordinary per-task commits. This executor made no commits, pushes, content staging, index mutations, hook bypasses or STATE/ROADMAP/REQUIREMENTS/PROJECT/task-ledger changes. Root owns requirement activation, formal verification, lifecycle validation, integration re-audit, status freshness and finalization. `requirements-completed` remains empty.

| Task | Intended record | Status |
| --- | --- | --- |
| 1 | `docs(153-04): document measured ordinary automatic retention` | Pending root finalization |
| 2 | `fix(153-04): exclude daemon test trees from production authority scanning` plus native evidence and generated LOC freshness | Pending root finalization |

Base HEAD remains `6dd6f18c9b16158b16391fcd288aadc94728cfcd`. No task commit hashes exist under the explicit deferred-finalization rule. Root's existing intent-to-add source preparation was preserved.

## Verification Evidence

The complete stdout/stderr attempt history is retained locally at `/tmp/open-bitcoin-phase153-verify.log`. The final run has an explicit attempt-3 marker after the two failed attempts. Earlier failures are not reported as passes.

| Check | Actual outcome |
| --- | --- |
| Initial timed Cargo formatting | `bun run scripts/command-timings.ts run --key phase153-plan04-format -- cargo fmt --manifest-path packages/Cargo.toml --all` exits 0 |
| Default full attempt 1 | Exit 1, 187ms: stale tracked LOC report; native printed remediation used |
| Default full attempt 2 | Exit 1, 11m01.170s: Phase 127 production-helper corpus included a nested test fixture |
| Root checker RED/GREEN | Meaningful RED 16 pass/1 fail before fix; final 17/17 pass, including production `tests_support/fixtures.rs` rejection; actual checker passes |
| Default full attempt 3 | `bash scripts/verify.sh`, exit 0 in 45m47.953s; no `--fast`, waived checks or hook bypass |
| Strict workspace Clippy | All targets/features with `-D warnings` passes; 55.45s |
| Workspace build | All targets/features passes; 2m50s |
| Workspace tests and doctests | 38 completed suites, 3,114 passed, zero failed; one existing explicitly opt-in public-network test ignored |
| Genuine fixture in default suite | `tests::automatic_prune::automatic_prune_genuine_ordinary_retention_and_reopen ... ok`; daemon suite 42 passed, zero failed/ignored/filtered, 25.57s |
| Node/RPC default suites | Node 984 passed, zero failed, one existing public-network opt-in ignored, 106.16s; RPC 265 passed, zero failed/ignored, 18.87s |
| Benchmark list/smoke/report | Native contract passes; JSON/Markdown smoke reports generated and report validated |
| Bazel smoke | All six top-level targets pass; 44.574s; build-provenance check passes |
| Pure-core coverage | Native `cargo llvm-cov clean` plus configured pure-core report passes; the native missing-lines guard finds no `Uncovered Lines:` section |
| Final claim/doc checks | Phase 151 release boundary and current-documentation reconciliation pass |
| Final managed/breadcrumb checks | Managed all: 1,230 files, zero findings; 919 Rust breadcrumb paths pass |
| Freshness/diff | Worktree LOC `--check` and `git diff --check` pass |

The single ignored test is the pre-existing `sync::tests::errors_and_live::live_network_smoke_is_explicitly_opt_in`, which requires explicit public Bitcoin network activation. The genuine legal-target test is neither ignored nor filtered. No environment exclusion, lowered target or accounting override was introduced.

The available `mdformat` probe had GFM/frontmatter plugins and checked the four touched Markdown files without write mode. All four mismatches also reproduced on untouched HEAD copies. There is no repository Markdown formatter configuration; the native documentation reconciliation contract passes. No generic whole-file rewrite, new formatter dependency or claim of a passing mdformat result was made. One initially guessed documentation-checker filename was missing; using the actual repository reconciliation checker corrected that invocation and passed.

## Safety Review and Simplification

The explicit touched-seam review corroborates an unchanged pure planner, one existing durable paired-unlink owner, metadata-only snapshot accounting, complete clone-shared mutation invalidation and captured-revision publication. Lock order remains authority → automatic state → storage payload guard; nested flush effects reacquire only after the storage guard is released. Durable lock publication, current manual revalidation and status reads use the same authority. Nonempty automatic plans select the existing Always full checkpoint. Current coins best-block carry preserves a newer overlay tip; successful/error deletion receipts clean cached bodies and undo without leftover-wallet fallback.

No extra abstraction, worker, dependency, schema or public/network default was added. Production modules and isolated fixture seams remain within managed file-size limits. The narrow checker fix follows the exact `tests` path boundary instead of excluding filenames such as `fixtures.rs`; its negative production fixture retains the authority guard's strength.

The refreshed [code review](153-REVIEW.md) covers 26 files and records clean status with zero findings. The refreshed [security report](153-SECURITY.md) records 14/14 declared mitigations closed and zero open, including the scanner regression. Those reports were produced before the final native pass; root owns their execution-status freshness and Summary 04 flag refresh. This executor's own safety/diff review found no additional issue.

AGENTS local guidance, the Bright Builds sidecar and overrides, architecture/code-shape/testing/verification/Rust standards and both active lesson files informed this work. Both lesson inputs fit the startup budget and were read completely. Root had already handled sync/bootstrap and lesson baseline maintenance; no project skills were present. Existing concurrent changes were preserved.

## Deviations from Plan

1. **[Rule 3 - Blocking, root-authorized scope] Phase 127 production corpus included nested tests.** Default verification showed a production-authority guard failure after its original 15 fixture tests passed. `productionDaemonHelperSources` recursively included `open_bitcoind/tests/automatic_prune/fixtures.rs` because it excluded only basename `tests.rs`. Root amended Plan 04 to own two checker paths and implemented exact test-directory exclusion plus behavioral positive/negative regressions. Meaningful RED, 17/17 GREEN, actual checker, renewed review/security and the full default run verify the repair. The production duplicate-authority rule remains enforced. Finalization remains pending, so no commit hash exists.
2. **Required generated freshness:** The first default run stopped before substantive checks because tracked LOC was stale after the earlier source work. Regenerating the explicitly owned worktree artifact with the native remediation resolved this preflight. No source or check relaxation was required.

## Issues Encountered

The first failed full run led to the narrowly assigned corpus repair above; full verification restarted from the default entrypoint afterward. Script and compiled Rust target startup included quiet intervals. Resumable sessions were polled and liveness inspected; no process was terminated, no app-owned build was killed and no security metadata was changed. No unrelated build/test/lint failure was repaired.

## Known Stubs and Threat Scan

No goal-blocking stubs remain in the changed documentation/checker surface. Pending requirement/lifecycle language is deliberate authority tracking, not a product placeholder. Sparse fixtures and controlled later faults are accurately identified as test evidence. No new endpoint, authentication path, trust-boundary schema or unregistered security surface was introduced; the scanner correction and claim/support controls fit T-153-12/13/14. No additional threat flag was found.

## Root Handoff

The final native result and measured production evidence are ready for root's independent formal phase verification. Root must create a lifecycle-valid Phase 153 verification report, activate/reconcile canonical PRUN-01/PRUN-02 only after that report, refresh current documentation and report status, validate lifecycle evidence, re-audit v2.4 integration and rerun required freshness gates before strict git finalization. Do not treat this summary as that independent report. Milestone archival remains separate.

No production-readiness, hardware fault, continuous consensus-chain, immediate physical-reclamation, archive/assumeutxo/BIP37, public-default or production-funds claim is added.

## Self-Check: PASSED

All eight modified paths and this summary exist. The unchanged base commit exists. Summary frontmatter uses the originating yolo lifecycle ID, `requirements-completed: []` and only opening/closing frontmatter delimiters. The final default verifier returned actual exit 0; the genuine default-suite test passed and generated LOC is current. Final managed, claim, documentation, breadcrumb and diff checks pass. Task commits and state/requirement updates are intentionally absent under D-10 and root ownership. Root formal lifecycle verification, current-status reconciliation, integration re-audit and strict finalization remain pending.

## Final Canonical Activation

Root subsequently created [formal Phase 153 verification](153-VERIFICATION.md), passed 22/22 with `lifecycle_validated: true`, validated the originating lifecycle, activated PRUN-01/PRUN-02 through Summary 03 and marked both requirements Complete. This summary's `requirements-completed: []` remains unchanged because Summary 03 owns the activation proof. The [integration re-audit](153-INTEGRATION.md) connects 20/20 selected seams and traces 10/10 scoped flows with zero blocking gaps and three retained advisory items. Five current documentation files now cite that formal closure and the default full native pass while preserving all fixture, accounting and concurrency limits. Earlier Pending language above records the pre-activation handoff; it is historical execution context. Root retains metadata/audit freshness and strict git finalization ownership. No additional Cargo/Bazel test run or source change was performed for this documentation reconciliation.

Final activation checks pass: live Phase 151 boundary, current-documentation reconciliation, 919 Rust breadcrumbs, managed all (1,230 scanned files; zero findings) and `git diff --check`. Only the five owned current-documentation files and this append were changed; no staging, commit, state/roadmap edit or Git index mutation was performed.
