---
phase: 158-validated-reorg-and-retained-branch-identity
plan: "07"
subsystem: testing
tags: [bun, parity, validated-reorg, mutation-testing, native-verification]
requires:
  - phase: 158-06
    provides: Genuine reorg/recovery/protection tests and actual 54-configuration work evidence
provides:
  - Scoped parity, contributor README and executable local UAT evidence
  - Deterministic current-root guard with 449 passing mutation tests
  - Default complete native verification including coverage and Bazel provenance
affects: [159, 160, 161, 162]
tech-stack:
  added: []
  patterns: [fixed current evidence roots, exact sealed-field exception, actual native receipts]
key-files:
  created:
    - docs/parity/v2-5-validated-reorg.md
    - scripts/check-phase158-validated-reorg.ts
    - scripts/check-phase158-validated-reorg.test.ts
    - .planning/phases/158-validated-reorg-and-retained-branch-identity/158-UAT.md
    - .planning/phases/158-validated-reorg-and-retained-branch-identity/158-NATIVE-EVIDENCE.md
  modified:
    - docs/parity/index.json
    - README.md
    - packages/README.md
    - scripts/verify.sh
    - scripts/check-phase157-index-catch-up.ts
    - scripts/check-phase157-index-catch-up.test.ts
    - scripts/check-phase157-index-catch-up/contracts.ts
    - scripts/check-phase103-mempool-lifecycle.test.ts
    - docs/metrics/lines-of-code.md
key-decisions:
  - Preserve fixed current source roots and actual caller/callee contracts without future verification-file prerequisites
  - Restrict the visibility exception to BasicFilterAppendProof.identity pub(super)
  - Split historical grouped mutation cases without changing assertions, fresh fixtures or timeouts
  - Preserve pending root activation and explicit local resource-policy differences
requirements-addressed: [CFIX-03]
requirements-completed: [CFIX-03]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 158-2026-10-08T02-17-15
generated_at: "2026-10-09T04:22:09.129Z"
duration: approximately 45min
completed: 2026-10-09
---

# Phase 158 Plan 07: Scoped Parity and Native Guard Summary

**449 mutation controls now enforce genuine accepted-position, retained-source,
recovered-projection and own-flush contracts; the complete default verifier passed.**

## Execution outcome

Tasks **3/3 implemented and verified**. Execution took approximately 45 minutes
including context loading, independent fix-review waits and the final
19m24.571s native run. This completes Plan 07's implementation/evidence handoff;
root security/formal lifecycle closure, requirement activation and Git remain
separate gates.

The [native evidence](158-NATIVE-EVIDENCE.md) records exact timestamps, versions,
failed attempts, artifacts and limits. Final full default execution was
2026-10-09T03:59:49Z–04:19:15Z, exit 0. Normal Cargo unit/integration/doctest
summaries total 3,735 passed / 0 failed / 3 optional exclusions; the node library
passed 1,353 with zero failures. Coverage rerun is excluded from that aggregate.
Benchmark smoke passed 11 groups/15 cases; all six Bazel targets and provenance
passed; the pure-core no-uncovered-lines gate passed.

## Task 1: Scoped contributor and parity evidence

The new parity page and unique in-progress CFIX-03 manifest surface describe
actual internal acceptance, equal/longer/lagged/repeated branches, immutable
displaced hash lookup after actual reopen, physical suffix masking, exact
genuine undo and required-body refusal, ordinary coins cadence and reserved
prune protection. README links and UAT provide exact timed repo-local Cargo/Bazel
forms. Existing compiled CLI help confirmed the published help/status forms;
there is no invented reorg command or external filter API.

The intentional finite local resource policy can refuse even fully retained
deep/shared-gap forks before effects. The 54-preparation/149-turn report remains
the current measurement owner; historical Phase 157 reports were not rewritten.
The default-maturity-100 case stays below the runtime 210,000 versus Knots
regtest 150 halving difference; compact maturity one and actual 401/1,025-height
prune evidence retain their explicit scope. RPC/peer/operator/integrated serving
159–162 remains pending/nonshipped.

## Task 2: Deterministic current-source guard

The exported `checkPhase158ValidatedReorg(maybeRoot?) -> string[]` reads only its
fixed `CHECK_FILES` roots. `CONTRACTS` follows actual production callers and
guarded callees, with a narrow impl selector where sealed types share a method
name. It reuses the existing Rust comment/literal/test-only masking helpers;
it is supplementary structural evidence, not an AST simulator or runtime proof.

Checks cover private stage/receipt fields, absorption before persistence,
preview suspension, exact store/generation/branch/revision/frontier bindings,
live accepted positions on all nonempty turns, RecoveredPrefix projection
restrictions, predecessor commitments, immutable equality, physical native
body/undo admission before decode, exact historical undo equality, displaced
coins fencing, ordinary own-flush receipts, prune consumers, active versus hash
lookup, executable named tests, breadcrumbs, lifecycle summaries, parity,
bounded diagnostics and independent unsupported-claim clauses.

Both documented and executed verifier sections run the explicit new Bun test
file and checker after Phase 157 and before Phase 121. Existing final Phase 138
ordering is preserved; no future Phase 158 verification artifact is required
by the native checker.

Actual TDD/review evidence:

| Control | Result |
| --- | --- |
| Initial checker tests before implementation | RED: missing checker module, 0 passed / 1 failed / 1 module error; `/tmp/phase158-guard-red.log` |
| Initial completed guard | 282 passed / 0 failed |
| Extended current-root/claim controls | 409 passed / 0 failed |
| WR-158-02 actual false-negative reproduction | 7 failing pub(super) visibility mutations / 33 other controls passed; `/tmp/phase158-visibility-red.log` |
| Exact field-visibility correction | 449 passed / 0 failed, 849 assertions, 34.48 seconds; `/tmp/phase158-visibility-green.log` |
| Final default verifier | Same 449 tests and live checker passed |

The sole exception is the existing exact
`BasicFilterAppendProof.identity: pub(super)` adapter seam. Every public or
restricted-public field on the other sealed types, and every other proof field,
is rejected. Independent delta review confirmed all 40 visibility mutations
reject while the required seam passes.

## Task 3: Native verification and root handoff

The default contract passed workspace formatting, strict all-target/all-feature
Clippy, build, tests, benchmark/report, Bazel/provenance, breadcrumbs and pure-core
coverage. No required behavior test is ignored. The three default exclusions are
the opt-in public-network smoke and the Phase 157/158 timing experiments.
Plan 06 separately executed the Phase 158 experiment successfully; no public
network attempt was made by this plan.

The first native attempt refused stale tracked LOC. The second and an isolated
diagnostic reproduced two historical Phase 103 grouped-test timeouts. Root
amended Task 3: two four-case groups became eight isolated named tests, retaining
all exact mutations/assertions, fresh fixtures, unchanged default timeout,
production checker and six other tests. The amended suite passed **14/14 with
the same 16 assertions**; independent delta review passed before LOC refresh
and the successful complete retry. Both failed receipts are preserved separately
and are not labeled passes.

## Deviations and review fixes

1. **[Rule 3 - Source-route freshness, root amendment]** Observed inherited
   Phase 157 failures after proof/budget/append splits. Updated actual moved
   routes, the budget re-export and scoped proof visibility; preserved original
   negative controls and numeric checks. Its complete suite passed **600/600**.
   Phase 155/156 passed and were left unchanged.
2. **[Rule 1 - Guard bug, WR-158-02]** The initial pub(super) exception applied
   too broadly. Seven actual false negatives were captured, then restricted
   to the exact required proof field with 40 isolated visibility controls.
3. **[Rule 3 - Native harness, root amendment]** Split the reproducibly timed-out
   Phase 103 grouped tests without a timeout change or cache.
4. **Required generated freshness:** Regenerated tracked LOC with the repo tool;
   final count 389,085. No handwritten metrics or managed-standard changes.
5. **Root-owned WR-158-01:** Another executor fixed the actual consecutive
   A23→B14→A23 unflushed-return bug. Plan 07 added its finalized regression to
   required evidence roots; that test passed in the full default run. No Rust
   production source was edited by Plan 07.

## Simplification, threats and limits

The simplification pass retained shared lexical helpers, one declarative set of
actual call contracts and existing real-source fixture copying. It introduced
no dependency, crate, production endpoint, authority constructor or duplicate
runtime worker. The historical timeout correction removes grouping rather than
weakening checks. Read-only diff review and whitespace checks were clean.

T-158-07-01/02/03 have mutation/native/scoped-claims, bounded diagnostic and explicit
verifier-wiring evidence. Final security mitigation closure is root-owned.
No unmodeled production trust surface or known product stub was introduced.
Source strings are not treated as sufficient behavior proof. Software faults,
reservations and stage-inclusive timings do not establish hardware crash,
whole-runtime constant-memory/work, isolated publication latency, archive scale,
production readiness or production-funds safety.

Material guidance: repo AGENTS/Repo-Local Guidance, Bright Builds sidecar,
placeholder-only overrides, architecture/code-shape/testing/verification and
Rust/TypeScript standards, canonical phase decisions and actual predecessor
summaries. Both active lessons were fully loaded: 7,188 bytes / 2,397 conservative
estimated tokens. No authentication gate or user-correction lesson trigger
occurred.

## Threat Flags

None unregistered. Plan 07 introduces no production network/authentication,
schema or file-access trust surface. WR-158-01 preserves the existing declared
same-store/displaced-coins-fence mitigation; WR-158-02 strengthens the declared
claim/evidence and verifier controls T-158-07-01 and T-158-07-03. Root owns the
25 declared threat closures; this section does not infer accepted risks or
assert security certification.

Finite-cap refusal, the maturity/halving parameter distinction, software-only
fault evidence and absent hardware-crash guarantees remain material tested
limitations. No unlimited-fork, whole-runtime memory/work, public-network,
external-serving or production-funds guarantee is added.

## Task commits and remaining root work

All staging, commits, push, root STATE/ROADMAP/todo/requirements and CFIX-03
activation remain explicitly deferred to root. No nonexistent commit is claimed.
Root receives this summary and native evidence for final security closure,
formal must-have/lifecycle verification and truthful contributor-status
reconciliation.

## Self-Check: PASSED

All five created implementation/evidence artifacts and this summary exist. The
full default native exit is actually zero; recorded checker/harness counts and
versions match their outputs. Frontmatter preserves the exact originating
lifecycle and requirements-completed remains empty. Final source hashes match
the reviewed frozen checker/test/harness. Commit-existence checks and root state
updates are intentionally deferred under the explicit Git barrier.

## Root closure | 2026-10-09T04:29:44.988632Z

All seven plans are complete. [Formal verification](158-VERIFICATION.md) passed 21/21 truths and the same-attempt lifecycle gate; [security](158-SECURITY.md) closed all 25 mitigations. CFIX-03 is Complete in canonical requirements and parity records. The earlier pending-root statements describe the execution handoff. Actual mandatory commit-hook, commit and push outcomes are derived from the saving commit and upstream refs, avoiding a self-referential stored commit hash. Phases 159–162 remain pending.
