## task-v24-milestone-audit | 2026-10-02 17:12 CDT | Audit v2.4 before archival

- [x] Resolve the active milestone and read all six phase verification reports.
- [x] Cross-reference all 17 requirements against traceability, verification, and all 27 summary frontmatters.
- [x] Independently check production integration and end-to-end prune, recovery, wallet, serving, and operator flows.
- [x] Run the repository verification contract and Bright Builds checks; distinguish fresh results from historical evidence.
- [x] Save `.planning/v2.4-MILESTONE-AUDIT.md`, review the artifact diff, and route by the audit result.

Workflow: `/gsd-audit-milestone`; audit only, with milestone archival left to its own command.

Completion review (2026-10-02 17:34 CDT): `gaps_found`, with 14/17 requirements fully satisfied, 14/16 integration seams connected, and 8/10 flows traced complete. Closure needs automatic production retention and creating-payload eligibility for post-prune midrange wallet scans. Fresh guardrail/traceability checks, 70 focused tests, Bright Builds checks, and audit YAML/scope checks passed. Full `verify.sh` was attempted and stopped after sampled Clippy processes remained blocked in macOS dynamic-library loading; build/tests/Bazel/benchmarks/coverage were not reached. No implementation changes, commit, push, or archival.

## task-v24-gap-closure-phases | 2026-10-02 17:40 CDT | Plan milestone audit gap closure

- [x] Load the latest structured audit and prioritize all three requirement, two integration, and two flow gaps.
- [x] Prepare and validate a concrete two-phase proposal with dependencies, tasks, and runtime verification criteria.
- [x] Obtain the confirmation required by the invoked skill before roadmap/ownership changes.
- [x] Append phases, reopen all three affected requirement rows, and create phase directories.
- [x] Validate pending-state checker compatibility and planning consistency with focused tests, live checkers, and the required Rust checks.

Proposal: `.planning/reports/v2.4-GAP-CLOSURE-PROPOSAL.md`. Current state: approved, created, and checked. The Phase 151 gate permits only audited/mapped Pending closure and requires new-owner lifecycle evidence before Complete. The full repo-native verifier remains the mandatory commit gate; Git history records finalization.

Completion review: Phases 152 and 153 have goals, dependencies, four planning tasks each, success criteria, and tracked directories. All three affected requirements are Pending under their new owners. The 82 affected checker/documentation tests, live checkers, Bright Builds checks, and ordered Rust format/Clippy/build/test checks passed. No execution plans or product fixes were created; INT-02 and INT-01 remain the work of the new phases. Next: `/gsd-plan-phase 152`, then re-audit after both closure phases pass.

## task-phase152-wallet-eligibility | 2026-10-02 19:28 CDT | Close post-prune wallet rescan eligibility

- [x] Load active lessons and standards, sync remote state, and resolve Phase 152.
- [x] Capture yolo decisions with lifecycle provenance and scope boundaries.
- [x] Research and check executable plans for SNAP-01 and INT-02.
- [x] Implement shared replacement eligibility and node chunk/resume refusal handling.
- [x] Apply the same replacement eligibility contract to durable RPC entry points.
- [x] Prove real paired-prune refusal, retained controls, resume/reopen, and preserved wallet state.
- [x] Review/simplify changes, refresh parity/docs, and pass default repo verification.
- [x] Record lifecycle-valid phase verification and progress.
- [x] Review and stage the verified change set for commit and push on main.

Workflow: `/gsd-yolo-discuss-plan-execute-commit-and-push` (no arguments;
selected Phase 152). Lifecycle: `152-2026-10-03T00-26-52`.
Planning, implementation and finalization follow the strict clean-verification
gate; no commits until that gate passes. Phase 153 stays pending.

Git finalization is confirmed by the phase-scoped commit and current
branch/upstream synchronization. It is derived from Git evidence rather
than a self-updating push checkbox that would need another commit solely
to record its own push.

Failure signal: The real paired-prune node regression
`post_prune_midrange_rescan_preserves_wallet_and_fails_job` failed against
the unchanged adapter (exit 101): deleting creating height 1 while retaining
the coin still let a scan starting at height 2 return Complete/Fresh through
height 3. This establishes INT-02 behavior before the shared gate change.

RPC failure signal: With the durable registry variant asserted and real
height-1 paired deletion, have-pruned and surviving coins confirmed, the
unchanged range adapter returned Fresh through height 3 for start/stop 3.
The refusal regression failed (exit 101); the retained-payload control
passed. The earlier local-adapter fixture run is excluded from this evidence.

Completion review (2026-10-03 UTC): Phase 152 passed 11/11 goal checks and
the required lifecycle validator. The full native verifier exited 0 in
34m 46.545s, including lint, build, tests, benchmark smoke, coverage and
Bazel; all 37 new shared/node/RPC cases passed. Review found and resolved
the reused-job freshness bug; the final review is clean and all 17 planned
threat dispositions are closed. Parity/docs and canonical SNAP-01 ownership
are reconciled. Residual scope: Phase 153 automatic retention remains
pending, and completed-prune/resume eligibility does not claim global
atomicity for concurrent direct-library callers. Final Git evidence is
derived as described above.

## task-phase153-automatic-retention | 2026-10-02 23:01 CDT | Automatic Prune Retention Integration

- [x] Capture yolo discussion decisions and lifecycle provenance for Phase 153.
- [x] Research authoritative payload accounting and production flush integration; create and check execution plans.
- [x] Execute plans with measured usage, automatic planner invocation, paired deletion, and cache cleanup.
- [x] Validate target/gate/lock/window/error/reopen behavior and operator/wallet regressions.
- [x] Review code, simplify touched seams, refresh parity and contributor evidence.
- [x] Run default `bash scripts/verify.sh`, verify phase/lifecycle, and prepare deterministic current-`main` finalization under the required commit hook.

### Completion Review

Phase 153 passed 22/22 formal truths and the full native contract in
45m47.953s, including 3,114 workspace/doc-test passes, the genuine legal
target, benchmark smoke, six Bazel targets and pure-core coverage. Review
is clean; all 14 declared threats are closed. The milestone re-audit has
17/17 requirements, 8/8 phases, 34/34 plans, 20/20 seams and 10/10 flows
with no blocker. Residual limits are the three inherited advisories,
logical-versus-physical accounting, sparse codec-valid fixture scope and
nontransactional wallet probe/save. Milestone archival is separate.
Git finalization evidence is derived from the saving commit and upstream
refs, avoiding a self-referential stored commit hash.

Progress: All four Phase 153 plans are complete. The genuine legal-target
scenario passed with 578,359,864 measured initial bytes and real paired
deletion, protected survivors, wallet/serving/operator checks, error cleanup
and reopen. Related storage, owner, flush, RPC and wallet suites plus scoped
strict Clippy passed. The default native verifier exited 0 in 45m47.953s
with benchmark, Bazel and pure-core coverage; source review is clean and
all 14 declared threats are closed. Static integration passed 20/20 seams
and 10/10 flows. Formal lifecycle, canonical activation, current-document
freshness and final audit aggregation are complete. The required hook
and upstream refs provide final Git evidence.
Source paths use root-owned intent-to-add entries only
to satisfy tracked-file discovery; no commit has been created.


## task-v24-archive-push | 2026-10-03 12:35 CDT | Archive v2.4 and push all

- [x] Verify scoped readiness, collect summary/task/Git statistics, and retain audit advisories.
- [x] Add archive-aware resolution to the affected Phase 151 guard with regressions.
- [x] Archive roadmap, requirements and audit; evolve project, milestone, retrospective and current docs.
- [x] Verify archive links, scope, metadata, and all repo-native checks; safety-commit archives before removing active requirements.
- [x] Prepare final closeout, annotated v2.4 tagging and current-main/tag publication under the required hook; final evidence is derived from Git refs.

Completion review: Archived 8 phases, 34 plans, 69 normalized completed task entries and all 17 requirements, preserving historical phase directories and the three accepted audit advisories. Updated project/current docs, decision outcomes, milestones and retrospective; next milestone remains unselected and starts with fresh requirements at phase 154. The archive-aware Phase 151 guard passed 58 positive/negative tests; documentation/traceability/integration checks pass. Safety commit `759abdbd` saved all archive controls before active requirements were removed and passed default native verification in 1h04m10.859s. Final hook, annotated tag and upstream publication evidence are derived from the saving commit/tag and remote refs rather than predicted hashes or outcomes. Repo-local/Bright Builds guidance and the existing native formatter/hook contract informed the closeout.

## task-v25-new-milestone | 2026-10-03 | Define v2.5 Prune-Aware Compact-Filter Serving

- [x] Record accepted scope in PROJECT and reset STATE without clearing historical evidence.
- [x] Research stack, features, architecture and pitfalls against pinned Knots and BIP157/158; synthesize findings.
- [x] Define atomic testable requirements, explicit missing-history behavior and retained exclusions.
- [x] Create roadmap from Phase 154 with every requirement mapped exactly once and observable success criteria.
- [x] Validate planning metadata, links and claim boundaries; prepare the final roadmap commit under the mandatory full native hook.

Verification includes active-milestone traceability, current-document truth, Phase 151 archive fallback, managed standards checks, scoped diff review and the full repo-native pre-commit contract. Completion evidence is derived from actual commands and saving commits. No phase implementation or public-network action is authorized by this initialization alone.

Completion review: Defined 22 Pending requirements and nine phases (154–162), with every requirement recognized by the native parser and mapped exactly once. Four topic reports and their synthesis cite pinned Knots/BIP sources; startup protection, chainstate-fenced progress, reserved lock ownership and continuous validated-chain proof drive the order. Kept all 135 historical phase directories. Simplification retained one BASIC index, existing crates/Fjall and no new production dependencies; letters-only CFNET IDs avoid the existing parser's silent omission. Prior workflow commits passed their full native hooks (milestone-start: `c0ffc5ea`; research: `d977742c`; requirements: `6dd46e4d`); final verification is evidenced by the saving roadmap commit. Schema/recovery details, budgets and known V0 outcomes remain phase-specific design work. Phase 154 is ready for context; compact-filter serving is planned, not shipped.

## task-phase154-basic-filters | 2026-10-03 23:40 CDT | BASIC Generation and Commitment Parity

- [x] Resolve Phase 154, synchronize main and submodule, load active lessons and relevant standards.
- [x] Capture yolo context with complete historical-script and independent parity decisions.
- [x] Research and check executable plans covering CFIL-01 and CFIL-02.
- [x] Implement exact BASIC generation, commitments and validated historical-input projection.
- [x] Prove pinned vectors, edge cases, historical/same-block spends and missing-evidence refusal.
- [x] Review source, security mitigations and simplification opportunities.
- [x] Run full `bash scripts/verify.sh`, Bright Builds checks and lifecycle validation.
- [x] Prepare verified tracking and changes for commit/push; derive final transport evidence from Git history and remote sync.

Completion review: four plans implemented; 11/11 must-haves and all three roadmap criteria passed, source review is clean, and 15/15 declared threats are closed. Full default native verification passed in 31m 19.275s, including Rust, benchmark, Bazel/provenance and pure-core coverage gates. Git history and remote synchronization are the authoritative commit/push record for this pre-commit snapshot. Residual boundary: index/storage, activation, RPC, peers and production claims remain later scope.

## task-phase155-recoverable-index | 2026-10-04 11:01 CDT | Recoverable Index and Pre-Prune Startup Protection

- [x] Resolve Phase 155, sync main/submodule and load active lessons plus repo standards.
- [x] Capture yolo storage, durable-fence and pre-prune startup decisions.
- [x] Research concrete recovery/fault seams for CFIX-02, CFIX-04 and CFPR-03.
- [x] Check executable plans and threat models before implementation.
- [x] Implement bounded BASIC validation and pure recovery/protection contracts (155-01).
- [x] Implement additive immutable Fjall records and atomic publication (155-02).
- [x] Integrate pre-prune startup protection in production reopen (155-03).
- [x] Complete recovery/fault evidence and native parity guardrails (155-04).
- [x] Prove real Fjall faults/reopen and production interrupted-prune refusal/safety.
- [x] Review source and simplification opportunities; independent review is clean across 36 files.
- [x] Close the final security disposition with full native evidence (17/17 closed).
- [x] Run full native verification, Bright Builds checks and lifecycle validation.
- [x] Prepare verified changes for commit/push; derive actual finalization from Git history and remote synchronization.

Completion review: four plans implemented; formal verification passed 10/10 distinct truths and all four roadmap criteria, independent source review is clean across 36 files, and 17/17 security dispositions are closed. The full default native verifier passed in 50m 23.935s with Bun 1.3.9 and Rust 1.94.1, including integration, benchmark, Bazel/provenance and zero-uncovered-lines pure-core coverage. CFIX-02/04 and CFPR-03 are Complete. This is the verified pre-commit snapshot; Git history and remote synchronization record the actual consolidated commit/push. Residual boundary: ordinary index-owned prune coordination, activation/catch-up, runtime reorg and serving/operator/integrated-client scope remain Phases 156–162; accepted historical-fence provenance and linear startup work do not claim hardware durability or archive-scale performance.


## task-phase156-owned-prune | 2026-10-04 15:28 CDT | Index-Owned Manual and Automatic Prune Coordination

- [x] Select Phase 156, sync main/submodule and load active lessons and applicable standards.
- [x] Capture yolo reserved-ownership, apply-time protection and disable/re-enable decisions.
- [x] Research concrete publication, deletion, cache and lifecycle fault seams.
- [x] Create and independently check executable CFPR-01 plans with threat models.
- [x] Execute all plans; prove manual/ordinary automatic pruning, owned CRUD, fenced release and disable/re-enable.
- [x] 156-01: compatible lifecycle/owner codec; focused core 42/42 and codec 13/13 passed.
- [x] 156-02: guarded publication/startup; core44, node78 and targeted legacy1 passed.
- [x] 156-03: ordered disable/re-enable; storage39 and filter-index91 passed.
- [x] 156-04: reserved CRUD/map enforcement; node90 and RPC38 passed.
- [x] 156-05: concrete39/filter96/flush52/receipt10 passed; inherited fixture collision fixed and default-parallel gate passed20 writer/3 full-control repetitions.
- [x] 156-06: ownership-aware cache/throttle and protected planning; default-parallel35/35 passed.
- [x] 156-07: legal daemon1/inherited daemon5/node prune129/RPC prune38 passed; backend repeat30/fullfilter102/Bazel gate closed.
- [x] 156-08: scoped parity/docs and default native guard; final mutation84/inherited53, live/provenance/managed checks passed.
- [x] Source67+locks2 review and simplification clean; all 27 security mitigations closed after native proof.
- [x] Default native verifier passed in 32m 37.914s; managed checks and lifecycle-valid formal verification passed.
- [x] Prepare verified current docs and staged changes for consolidated commit/push; derive final transport evidence from Git and upstream refs.

Verification: real Fjall/authority paths, checkpoint/protection faults and reopen; stale deletion/work rejection; unchanged ordinary operator locks and default-disabled legacy behavior. Full `bash scripts/verify.sh` remains the pre-commit contract. Strict user-invoked wrapper defers all workflow commits until clean phase verification; git evidence is derived from actual commands.

### Native restart plan

- [x] Apply and independently review the equivalent Clippy predicate simplification.
- [x] Regenerate and check the LOC source fingerprint after the final source edit.
- [x] Restart the complete default native contract; preserve prior failed attempts in `156-NATIVE-CHECKS.md`.
- [x] Close security/lifecycle evidence, reconcile claims, and prepare the mandatory commit hook and derived Git finalization evidence.

Completion review (verified pre-commit snapshot): Phase 156 passed 28/28 must-haves and all four roadmap criteria, with clean 67-file source review plus two generated lockfiles, 27/27 closed security mitigations and the default native verifier exit zero in 32m 37.914s. Workspace tests/doctests recorded 3,382 passes and one existing opt-in public-network ignored case; benchmark, Bazel/provenance and configured zero-missing-lines pure-core coverage passed. CFPR-01 is Complete and Phase 157 is ready for context. Residual scope preserves codec-valid large-fixture limits, custom-maturity small engine evidence, software versus hardware durability, the three v2.4 advisories and deferred activation/catch-up/reorg/serving products. Actual commit/push and mandatory hook success are derived from the saving commit and upstream refs, avoiding a self-referential post-push edit.

## task-phase157-safe-activation | 2026-10-05 09:50 CDT | Safe Activation and Scheduled Index Catch-Up

- [x] Select Phase 157, sync main/submodule and load active lessons and applicable standards.
- [x] Capture one-pass yolo activation, missing-history and bounded-owner decisions.
- [x] Research exact option semantics, startup ordering, accepted-state faults and total scheduler budgets.
- [x] Create and independently check executable CFAC-01/02 and CFIX-01 plans and threat models.
- [x] Execute plans with real daemon, validated-chain, Fjall reopen and fault evidence.
- [x] 157-01: exact BASIC option resolver/loader; 18 focused tests, 79 config regressions and scoped Clippy passed. CFAC-01 activation remains pending its daemon consumer.
- [x] 157-02: configured pre-prune startup/history refusal; 14 new controls, inherited filter114/initialize23/restart10 and scoped Clippy passed. Historical Phase 155 guard routing migration is explicitly assigned to Plan 10.
- [x] 157-03: pure ordered progress and checked work budgets; 52 new controls, 96 filter-index regressions and scoped Clippy passed. Numeric production budgets remain a measured Plan 07 gate.
- [x] 157-04: sealed recovered/validated managed flush authority; 36 final matrix controls, schema19/filter116/managed11 passed. Two historical Rust source assertions are Plan 10-owned; staged private consumer diagnostics must close with the actual Plan 07 driver. No clean Clippy/full-phase claim yet.
- [x] 157-05: budgeted incremental immutable/projection publication; append35/proof36/fencing116/prune39 passed. Acquisition, preparation and completion costs are cumulative before effects. Style Clippy passed; seven ordinary dormant private-graph diagnostics must close with the real Plan 07 consumer.
- [x] 157-06: accepted-before-persist historical facts and ordered dependent effects; accepted14/filter130/proof36/manager14/unflushed1/mempool7 passed. Style checks passed; actual Plan 07 initialization/driver must close dormant APIs and assess capture resource ceilings.
- [x] 157-07: actual startup/ordinary bounded driver; final filter145/proof36/append35/core99, strict Clippy and formatting passed. Final-source calibration includes prefix16/128/512 and a genuine989871-byte singleton; complete operation counts and admitted defaults are documented, with no hard latency or universal payload claim.
- [x] 157-08: actual offline configured daemon and existing typed maintenance owner; startup9/idle7/daemon59, strict RPC Clippy and formatting passed. Startup first turn is not duplicated; shutdown preserves savedActive protection and retained errors while Always cleanup runs.
- [x] 157-09: real history7/store14 matrix, strict node Clippy and scoped formatting passed. Fresh/saved paired-loss refusals preserve raw namespaces; ordinary checkpoint linkage earns safe release, and automatic retention achieves714 validated paired deletes with actual payload accounting and closed-reopen comparisons.
- [x] 157-10: all556 new source/claim mutations, historical15593/15684, provenance997 Rust files, live/managed/style and self-check passed. Final review then found WR-02 in async test-only masking; its correction and re-review remain active before whole-phase closure.
- [x] Review source, security and simplification opportunities:91 distinct files independently reviewed; shutdown/async/owning-attribute findings and the completed-state fixture were fixed and re-reviewed. All33 declared mitigations and final native proof are closed.
- [x] Close daemon review WR-01: actual bug RED, final4 real-worker controls/daemon63/Phase13588 and strict Clippy passed; independent9-file re-review is clean. Shutdown settles and joins before reporting failure, with clean marker withheld on failure.
- [x] Close guard review WR-02: actual complete-checker RED then exact diagnostic GREEN plus11 modifier/offset controls;568 final mutations/live/style passed, independent15-file re-review clean, T-157-30 closed.
- [x] Run full native verification, Bright Builds checks and lifecycle validation: default contract passed17m22.546s,3604 primary Rust tests/0failures/2intentional ignores, benchmarks/Bazel/provenance and zero uncovered pure-core lines; formal34/34 and lifecycle valid.
- [x] Reconcile current docs and prepare verified changes for consolidated commit/push: all three requirements Complete, scoped ledger done, nine/22requirements complete and four/ninephases verified; final source and fixture closure reviews are clean.

Verification contract: `bash scripts/verify.sh`; use timed Cargo/Bazel invocations and avoid overlapping shared-target builds. The strict wrapper defers workflow commits until clean verification; actual Git commands establish final commit/push evidence. Residual scope includes Phases 158–162 and all existing default-off/no-production boundaries.

Completion review: ten plans implemented with actual configured daemon, accepted history, real Fjall faults/refusal/reopen, measured bounded turns and normal prune ownership. Full default native passed; source91/declaredsecurity33/formaltruths34 all verified. Enabled storage requires retained validated genesis/history; conservative exact-tip release, representation/admission limits, software rather than hardware faults and deferred158–162/v2.4 advisories remain explicit. Git history and remote synchronization record the consolidated saving commit/push and mandatory hook outcome, avoiding a self-referential post-push edit.

## task-phase158-yolo | 2026-10-07 21:17 CDT | Validated Reorg and Retained Branch Identity

- [x] Resolve Phase 158, load active lessons/standards, synchronize main and pinned Knots.
- [x] Capture one-pass yolo branch transition, immutable retention and missing-input decisions.
- [x] Research accepted/durable branch fencing, bounded rewind and real validated fork evidence; preserve shared safe prefix while old-branch coins remain durable, including equal-height replacements.
- [x] Generate and independently check executable CFIX-03 plans and threat models; seven sequential plans/seventeen tasks passed fresh review after three targeted blockers were resolved.
- [x] Execute each plan; record summaries and behavior evidence.
- [x] 158-01: pure branch replacement and genuine staged/accepted contracts; 17 reorg/328 library/12 doctest controls, node compile, Clippy, formatting and starter checks passed. Same-hash return uses renewed generation; stage method moved to the existing owned module for file-length compliance.
- [x] 158-02: guarded Fjall publication, sealed accepted-position authorization and exact displaced-coins fencing; 37 new/35 affected controls passed. Confirmed reopened masked-suffix stall was reproduced and fixed through distinct trusted recovery rebinding. Formatting/starter/provenance/focused lint passed; seventeen unsuppressed consumer-warning groups remain explicitly assigned to 03/04.
- [x] 158-03: preview store suspension, explicit accepted endpoint and ordinary own-flush lineage; final 51 phase/35 append controls passed. Owned append/legacy work is revoked before preview, accepted persistence errors stay truthful and paused, and later connects advance visible target. Focused lint/style/provenance passed; seven unsuppressed driver/accounting diagnostic groups remain for 04/06.
- [x] 158-04: resumed current-source verification after host launch stalls; 15 new production/66 whole-phase/129 affected controls passed, with one inherited explicit timing experiment ignored. Strict all-target/all-feature Clippy, format, starter and provenance checks passed. Exact ledger expectations refreshed to 98 record/44 projection/7621 checkpoint/56 point-read operations with body/byte/batch/prefix bounds preserved. Historical checkpoint remains evidence; 04-SUMMARY is the completed handoff. No commit or push.
- [x] 158-05: final 82/82 phase controls passed, including 16 new continuous validated fork/sync/retention/reopen/startup-order cases and an unchanged standard-maturity-100 fork. Actual paired pruning and A→B→A reuse retain immutable bytes/headers; strict Clippy/format/policy/provenance checks passed. No production edits or Git finalization.
- [x] 158-06: 98 required phase controls passed; final exact-preservation fault 8/8 and protection 4/4 passed. Explicit measurement harness passed 54 preparations/149 turns with actual component/complete ledgers and stage-inclusive timings. Strict Clippy/format/policy/provenance passed; finite retained-fork/shared-gap refusals are documented for parity, with no arbitrary-fork or whole-runtime constant-cost claim.
- [x] 158-07: parity, README/UAT, 449-case claim checker and actual native evidence; full default verification passed in 19m24.571s with 3,735 primary Rust passes, benchmarks, Bazel/provenance and no uncovered pure-core lines.
- [x] Review source/security, make one simplification pass and resolve findings.
  Review progress: all 60 initial source/test/script/doc paths are independently reviewed. WR-158-01 has a real retained A23→B14→A23 RED, exact-fence repair, positive reopen and three missing/mismatched binding controls; the affected suite passed 102 tests and strict node Clippy, and independent delta review is clean. WR-158-02 has demonstrated visibility false negatives, the exact-field checker repair, 449 passing mutations and a clean independent 40-mutation delta check. Reproduced historical Phase 103 grouped-test timeouts were fixed by isolated named cases with the same eight mutations, 16 assertions and default timeout; 14 tests and independent review passed. Security verifies all 25 mitigations. Full native and formal 21/21 lifecycle verification passed; final completion-document deltas are checked separately before staging.
- [x] Run default native verification, lifecycle validation and review final diff.
- [x] Reconcile phase/requirement/project records and prepare scoped strict commit/push; actual hook, commit and publication outcomes are derived from the saving commit and upstream refs.

Verification: `bash scripts/verify.sh` with Rust 1.94.1 and Bun 1.3.9. Ad hoc Cargo/Bazel commands use the timing/lock runner. Defer commits until the strict clean gate. Completion review and residual risks will be recorded in this block.

Tracking compatibility: the installed GSD CLI recognizes `**Plans:**` and `**Requirements:**`, while the Phase 158 headings placed the colon outside bold. Only those two Phase 158 labels were normalized before CLI progress updates. Its progress-table regex also stops at the earlier two-column Phase 158 evidence row; the actual four-column progress row was reconciled explicitly to the CLI-reported 6/7 In Progress result. No completion or requirement activation is implied by this compatibility repair.

Completion review (2026-10-09 UTC): all seven plans and 21/21 formal truths are verified. The full default native contract passed in 19m24.571s with 3,735 primary Rust tests/doctests, zero failures, benchmark smoke, all six Bazel targets/provenance and no uncovered pure-core lines. Both review findings are resolved and all 25 security mitigations are closed. CFIX-03 is Complete; the active milestone has five of nine phases and ten of 22 requirements complete. Existing finite caps may refuse fully retained forks; compact maturity-one and default-maturity-100/halving limits, software versus hardware evidence and the three v2.4 advisories remain explicit. Phases 159–162 are pending. Mandatory hook and push results are derived from Git evidence as described above.

## task-phase159-yolo | 2026-10-09 11:11 CDT | Authenticated BASIC Filter and Index RPCs

- [x] Capture one-pass recommended context and canonical references.
- [x] Research durable read/connected-history authority and exact pinned RPC semantics.
- [x] Create executable plans and pass independent plan/requirements checks.
- [x] Execute source work with positional/named parsing, shared authority and authenticated transport evidence; full default native passed.
- [x] Review source and simplification opportunities; all seven findings closed in the clean 91-path independent review. Security closeout passed27/27 after final08 summary and actual preverification lifecycle receipts.
- [x] Run full native verification, Bright Builds and lifecycle validation; review the final diff.
- [x] Reconcile contributor docs and mark Phase159 complete after formal proof.
- [x] Prepare verified staged changes for consolidated commit/push; derive actual hook and transport evidence from Git and upstream refs.

Verification contract: `bash scripts/verify.sh`, pinned Bun 1.3.9 and Rust 1.94.1, serialized timed Cargo/Bazel commands. Defer all workflow commits to the strict clean-verification gate; no hook bypass. Root owns this task block and STATE/ROADMAP mutations.

Wave 1 review: Plans 01/02 are behavior-complete with 12 history and 18 new RPC tests, scoped RPC lint and Bright Builds clean. Sixteen unsuppressed dormant node diagnostics remain explicit Plan 03/04 production-consumer obligations. The GSD key-link helper cannot infer Rust `mod` paths from file names; actual call/import direction was checked and explicit Rust source patterns now verify all six prior-wave links. The acceptance mint and raw-metadata invalidation ownership deltas are recorded in Plans 01/03 and summaries.

Tracking compatibility: as in Phase 158, the installed GSD updater requires the colon inside `**Plans:**`/`**Requirements:**`. Only those Phase 159 labels were normalized so subsequent CLI-owned progress/completion updates can run; future phase labels and all other roadmap content remain untouched. This is a narrow parser compatibility repair inside the active workflow, not a completion override.

Task 07 fault-placement replan: RPC dependencies cannot access node `cfg(test)` private hooks. The independently checked amendment preserves configured HTTP proof and composes private genuine owner/query fault tests with exact RPC projection evidence. A store-bound, crate-private `cfg(test)` read-failure seam closes actual query-read injection; no public test feature/API, dependency, accepted-fact factory or clean-epoch grant is introduced. Reports distinguish composed proof from single daemon flows and record synthetic genesis/maturity-one/manual owner-plan limits.

Native-gate replan after two failed attempts:

- [x] Preserve both failed default-native receipts and fix the stale Phase 134 insertion marker; focused 257 tests and independent delta review passed.
- [x] Reconcile the current RPC catalog/checker with both methods; 20 controls, exact-set live check, managed checks and independent review passed; no other active count consumer found.
- [x] Run serialized workspace Rust preflight, then restart the entire default native contract; actual native4 exit0 recorded before final security/lifecycle/commit gates.

Workspace preflight follow-up: the host loader wait cleared without intervention, then strict Clippy exposed the missing CLI MethodCall conversions. Plan02 owns typed wire encoding plus positional/named/default controls; Plan08 owns native CLI-bin execution and corresponding claim guards. The actual test-only runtime_state module was moved to canonical tests.rs instead of relaxing production panic policy; its three controls, owning module, parity manifest and evidence paths are preserved.

Native attempt 3 recovery plan:

- [x] Repair legacy/raw metadata compatibility without minting provenance; all four original cases and the new live/reopen regression pass; WR-07 is closed.
- [x] Repair stale proof fixtures using actual recovery and preserve corruption refusal; all original append cases/new negative control pass, and final paired-reopen catchup regression passes exactly (1/0fail,0.87s).
- [x] Update the producer source assertion to actual modules/exact boundaries; its regression passes without weakening no-Always/no-Periodic checks.
- [x] Independently review all deltas, run focused and full workspace Rust verification, then restart the entire default native contract before phase closure.

Evidence: default native3 failed after1h5m5.214s: 1,931 earlier Rust passes, then node1,417pass/10fail/3ignored. Full594 source guard and workspace strictClippy/build passed. This is a real test failure; host startup delays do not waive it. Queued final Cargo formatter exited0 after verifier release. No commit/push or requirement activation.

Recovery2 result: workspace formatting/strictClippy/build passed; node1,428pass/1fail/3ignored. The remaining fixture configured only its store, leaving manager/store branch identities mismatched; authorization correctly refused. Paired actual fixture.reopen() now rebinds both, with original missing-payload/reuse assertions preserved. Recovery3 compiles once, runs that exact regression first, then full workspace and CLI binary tests. Full current guard611 passed (1,816 assertions/312.02s) before this final fixture sequencing edit. Independent review covers91 paths/all7findings closed; final execution/native/security/lifecycle gates still pending.


Native4 final receipt: default verifier exit0 in36m0.155s;3,864primaryRust passes/zero failures/three existingoptinignores,611guardtests/1,816assertions, benchmark smoke, six Bazel targets/provenance and zero-uncovered pure-core coverage passed. Source remains frozen; final08/security/formal/lifecycle records precede requirement activation and mandatory hook/Git finalization. Earlier failed receipts remain historical.


Completion review (verified pre-commit snapshot): Phase159 passed all26 truths/four roadmap criteria, clean91-path review/all seven findings closed and27/27 security mitigations. Default native exited0 in36m0.155s with3,864 primary Rust passes,611 mutation tests, benchmark smoke, six Bazel targets/provenance and zero uncovered pure-core lines. Full lifecycle validation passed before CFRP-01/02 activation. Phase160 is next; CFPR-02/CFNET/CFOP/CFGR and v2.5 remain pending. UnknownLegacy absent rows, numeric read/waiter caps and synthetic genesis/maturity-one/manual-owner-plan/composed private-fault limitations remain documented. Actual hook/commit/push outcomes are derived from the saving commit and upstream refs, avoiding a self-referential post-push edit.
