---
generated_by: gsd-plan-checker
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
---

# Phase 167 Plan Check — Iteration 1

## ISSUES FOUND

**Phase:** 167 — Candidate-History Validation Rules
**Plans checked:** 20; 41 tasks; 17 waves
**Issues:** 2 blockers, 0 warnings

The approved fidelity is represented throughout the plans. Revision is required to make Plan13's real daemon startup/reopen evidence executable against the actual product target. Neither finding calls for reducing the required evidence.

## Blockers

1. **Plan13 identifies the wrong daemon root file.** The plan owns and asks Task2 to modify `packages/open-bitcoin-rpc/src/bin/open_bitcoind.rs`. The actual auto-discovered product root is `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs`; `open_bitcoind/` is its helper-module directory. The planned underscore root is a different Cargo binary path. The Task2 instructions therefore do not own the existing entrypoint needed to register the new test module and preserve its worker/startup ordering. Correct the frontmatter and Task2 files/read-first references to the hyphenated root; retain the existing underscore helper-directory names.

2. **Plan13's actual daemon process test lacks an executable-build/test contract.** Task2 requires actual binary startup, authenticated local RPC, submission, stop and restart. Its only automated command is `cargo test ... --bin open-bitcoind phase167`, and its proposed tests live inside that binary's unit-test module tree. This command builds the test harness, not a normal daemon executable for spawning. Existing actual process coverage uses integration tests such as `packages/open-bitcoin-rpc/tests/phase166_canonical_startup.rs`, with `env!("CARGO_BIN_EXE_open-bitcoind")`. Add an owned Phase167 integration-test target and an explicit timing-wrapped `cargo test ... --test <target>` command using that mechanism, or specify an equally deterministic normal-binary build and path handoff before the process test. Do not depend on a stale local executable or count an in-process context/unit harness as the mandated actual daemon process evidence.

## Coverage and semantic checks

| Requirement | Concrete implementation/evidence owners | Assessment |
| --- | --- | --- |
| CVLD-01 / roadmap criterion1 | 01,06,10,12,14,15 | Selected immutable subsidy; genuine default-regtest149/150 and299/300 boundaries; maturity100 with height1 spend at101 versus refusal at100; no genesis UTXOs; actual managed durable owner and physical reopen. |
| CVLD-02 / criterion2 | 03–06,09–13,15 | Candidate-branch ancestry, versions/BIP34, BIP113/BIP68 creating-ancestor MTP, witness commitments; no caller blanket activation; staged replacement blocks derive their own context. |
| CVLD-03 / criterion3 | 02,03,05,06,10,15,16 | Exact baseline/replacement/addition order reaches actual interpreter and aggregate sigops; three exact serialized historical exception bodies remain seeded evidence, with bypass controls. |
| CVLD-04 / criterion4 | 01,03,04,06,09,15,16 | Shared header/body rules, strict/equality time/difficulty edges, regtest144/288 boundaries, independent seeded retarget/recovery cases and required-history refusals. |
| CVLD-06 / criterion5 | 02,03,07,08,11,16,19 | Both exact repeats and earlier coins; BIP34 ancestor/recheck1983702; complete overwrite provenance, both codec directions, physical reopen/disconnect restoration and explicit intentional Knots disconnect difference. |

All five requirement IDs appear in plan frontmatter. D-01 through D-10 have implementing task actions, including truthful support classification, independent source expectations, offline registration and final independent source/security/goal/lifecycle gates. Required finite historical behavior is owned here, rather than silently postponed to171. Actual Knots qualification168, broader differential recovery169, pruning170 and wider parity closure171 remain deferred as approved.

The dependency graph has no missing references or cycles, wave assignments agree with declared dependencies, and no same-wave file ownership overlaps were found. Every task contains files/action/verify/done and concrete acceptance behavior. All plans contain must-haves, handoff wiring and threat-model blocks. Scope is two tasks per plan except three documentation tasks in19; its eleven documentation/metadata paths are low-complexity and include explicit root-owned handoff, so their count alone is not a substantive scope failure.

Canonical authority is distinct from structural CandidateHistory, arithmetic HeaderProofFacts and fixture-origin historical envelopes. Plans04/06 establish earned lineage independently without introducing a chainstate/network dependency cycle;09/10 cover peer/direct/stored/local ingress;07/08 carry complete overwritten Coin metadata into durable undo. Plans11/12 require bounded complete retained-history replay through copied inspection, original-source equality and one live store owner before effects, rather than trusting persisted ScriptsValid labels. Rejected body authority and header-known/body-invalid behavior are correctly separated. Default-regtest support is earned; mainnet/testnet ordinary checkpoint prerequisites and missing Signet challenge remain explicit refusals.

Plan02 makes exact historical acquisition an early finite prerequisite with verified hashes/merkle/prevouts, strict corpus/walk bounds and no offline-verifier HTTP dependency. Missing bytes block rather than authorize a scalar substitute. Plans11–14 require long histories before15 freezes the independent corpus; earlier test-local histories can be generated under their owned test modules and need not consume the later frozen artifact. This is not a dependency cycle, but executors must not import a nonexistent Plan15 output to satisfy an earlier task.

Plan17 requires independent source review before current ownership renewal, and18 preserves the exact historical verifier inverse and frozen originals. Plan19 updates current source breadcrumbs/parity/docs and root-owned excerpts before20 freezes final review basis. Plan20 retains full default native verification, fresh pure-core coverage, Bazel, actual retained outcomes and canonical `167-REVIEW.md` ownership. The report-name correction is present.

**Dimension8:** SKIPPED — nyquist_validation is false. Research planning resolutions answer the identified questions, including stronger BIP30 restoration and bounded historical/reopen scope; no unresolved behavioral choice was identified. Frontend and ORM checks do not apply.

## Evidence and limits

This review used local AGENTS.md, AGENTS.bright-builds.md, standards-overrides.md, standards/index.md, architecture/code-shape/testing/verification and Rust/TypeScript guidance, both complete active lesson files (8856bytes;2953 estimated tokens), current ROADMAP/REQUIREMENTS/PROJECT,167 context/research/decision coverage and all20 plans. Static source probes checked pinned contextual header/script/BIP30 semantics, crate dependency direction, inspection bounds and actual daemon process-test conventions. No implementation, Cargo/Bazel/application/test execution, historical receipt audit, staging, commit or push was performed.

## Structured Issues

```yaml
issues:
  - severity: blocker
    dimension: key_links_planned
    plan: "167-13"
    field: "files_modified; Task2 files/read-first"
    description: "The planned daemon root src/bin/open_bitcoind.rs is not the actual src/bin/open-bitcoind.rs product entrypoint, leaving real startup/test registration outside the task's owned edits."
    fix: "Use packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs in frontmatter and Task2 file/read-first lists; keep open_bitcoind/ helper directory references unchanged."
  - severity: blocker
    dimension: task_completeness
    plan: "167-13"
    field: "Task2 action/files/verify"
    description: "Actual daemon process startup/RPC/restart is required, but only a binary unit-test target is planned; it neither supplies nor deterministically builds a normal daemon executable for the process test."
    fix: "Own a Phase167 integration-test target using CARGO_BIN_EXE_open-bitcoind and verify it with a timing-wrapped cargo test --test command, or explicitly build the normal daemon and pass its proved path to the process test. Preserve the full process evidence requirement."
```

Revise Plan13 and recheck the two affected contracts before execution.

# Phase 167 Plan Check — Iteration 2

## VERIFICATION PASSED

**Phase:** 167 — Candidate-History Validation Rules

**Plans checked:** 20; 41 tasks; 17 waves

**Current issues:** 0 blockers, 0 warnings

**Lifecycle:** yolo; `167-2026-10-21T16-38-23`

This is the current revision-gate result. The iteration 1 findings above remain preserved as review history; both are resolved by the revised Plan13. Unchanged plan bodies retain the prior full-review assessment. This iteration independently reviewed the complete revised Plan13 and checked all20 current plan frontmatter, task structure, decision references, dependency and ownership contracts.

## Resolution evidence

| Original blocker | Revised contract | Assessment |
| --- | --- | --- |
| Incorrect daemon root ownership | Frontmatter, Task2 files/read-first and action own `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs`; helper modules remain under `open_bitcoind/`. | Resolved. This matches the actual Cargo auto-discovered daemon root and its explicit helper-module paths. |
| Missing normal executable contract for actual process evidence | Owns `packages/open-bitcoin-rpc/tests/phase167_canonical_startup.rs`; spawns `Command::new(env!("CARGO_BIN_EXE_open-bitcoind"))`; separately runs timing-wrapped `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --test phase167_canonical_startup -- --test-threads=1`. | Resolved. The integration target supplies the normal product executable through the same existing convention as the Phase166 process test. |

Task2 explicitly retains inspection-worker registration, opener/genesis/worker ordering, binary unit/startup coverage and both four-selector genesis regressions. Static source inspection confirmed the named Phase166 binary-unit test in `src/bin/open_bitcoind/tests/phase165_work.rs`, the named actual-process test in `tests/phase166_canonical_startup.rs`, the normal daemon's inspection-worker entrypoint and its test-only worker macro registration. The new helper is explicitly registered under cfg(test) in the owned daemon root.

The new integration target owns its bounded fixture/process/RPC helpers and serialized default-regtest history before Plan15 freezes the independent corpus. It requires isolated config/datadir, cleared inherited environment, authenticated loopback RPC, external networking disabled, valid descendant submission, invalid-body refusal, graceful stop/reap, complete handle release, same-executable/datadir restart and returned tip/exact-work/BASIC evidence. Direct durable observations occur only after process shutdown and are dropped before restart. Wallet file absence, unchanged public defaults and missing Signet challenge refusal remain required. This fixes the executable/evidence seam without reducing D-01/D-07/D-08 or substituting unit evidence for process evidence.

## Regression checks

- All20 plans pass `gsd-tools verify plan-structure` with no errors or warnings; all41 tasks retain files/action/verify/done. Plan13 remains2tasks/8files and is the sole Wave10 owner.
- All declared dependencies exist, the graph is acyclic, waves equal one plus the maximum dependency wave, and same-wave file ownership is disjoint. Plan13 still follows12 and precedes14; the new integration target introduces no shared-file conflict or dependency on the later Plan15 corpus.
- CVLD-01/02/03/04/06 remain covered in frontmatter and in the unchanged concrete coverage assessment above. Every locked D-01 through D-10 still appears in implementing task actions. Context/research/decision-coverage review found no new contradiction, deferred-scope inclusion or reduced decision contract.
- The specialized consuming RPC factory, sealed runtime handoff, actual submitted-body owner, all-handle restart and process executable are connected by explicit task actions and must-have links. Existing fixture/canonical authority separation and independent evidence classes remain intact.
- Plan19's existing all-new-Rust breadcrumb reconciliation covers the additional integration test. Final offline native verification, fresh pure-core coverage, Bazel and independent source/security/goal/lifecycle acceptance remain Plan20 obligations. Actual pinned-Knots differential qualification remains Phase168.

**Dimension8:** SKIPPED — `workflow.nyquist_validation` is false. Frontend and ORM checks do not apply. Research's planning resolutions contain concrete answers; no new unresolved research choice was introduced.

## Evidence and limits

Material guidance: AGENTS.md Repo-Local Guidance, AGENTS.bright-builds.md, standards-overrides.md, standards/index.md, core architecture/code-shape/testing/verification/local-guidance and Rust/TypeScript guidance. Both complete active lesson files were loaded (8856bytes;2953 conservative estimated tokens); no project skill directories were present. Current roadmap/requirements/project scope,167 context/research/decision coverage, original review, revised13 and all20 frontmatter/structure were checked. Source reads were limited to verifying the changed executable/startup contracts. No implementation, test/build/application execution, staging, commit, push or shared metadata edit was performed.

```yaml
issues: []
```

Plans pass the revision gate and may proceed to the orchestrator's execution/lifecycle checks. This verdict establishes plan sufficiency, not implemented behavior or completed requirements.

# Phase 167 Plan Check — Scheduling Amendment Review 1

## ISSUES FOUND

**Scope:** Bounded scheduling amendment dated 2026-10-21T17:45:00Z and Plan01 key-link correction dated 2026-10-21T17:46:31Z. Original iteration1/2 reviews remain preserved above.

**Plans checked:** All20 frontmatters/structures;41 task blocks; contextual interfaces/actions for03–14; original01/02 plans; coverage ledger, context, amendment and prior review history.

**Issues:** 1 blocker, 0 warnings, 1 informational scheduling advisory.

## Blocking earlier consumer

Plan08 Task2, `Prove physical checkpoint and all-handle restore`, explicitly requires: “permit the two exact repeat bodies only with02 envelope prerequisites, connect, persist, release every handle, decode/reopen in test-seeded class, disconnect and compare exact prior coins.” It executes in wave6 and declares only167-07 as its producer dependency. The required02 envelopes are not an independent local regtest history or scalar source table. Thus02 acceptance in wave11 leaves an earlier historical-byte consumer without its required input, even though the declared DAG is acyclic. The amendment and coverage-ledger claims that03–14 have no required02 corpus consumer, and that16 is the first explicit consumer, are false for the retained task contract.

Preserve08's complete exact-body/UTXO/reopen evidence. The smallest scheduling correction is02 completion/acceptance in wave5 alongside07, whose files are disjoint, before08 wave6. This still allows03–07 to proceed after accepted01 while the original isolated acquisition continues. If acquisition remains incomplete at that barrier, hold08 and subsequent consumers. A wave11 gate instead requires a separately reviewed restructuring of08 and affected downstream scheduling; do not silently replace its exact bodies with synthetic or scalar substitutes. Retain the existing02→15 and02→16 prerequisites in either case.

## Checks that pass within this scope

- All20 `gsd-tools verify plan-structure` calls pass, with41 complete files/action/verify/done task blocks. The sole emitted advisory is02's `Wave > 1 but depends_on is empty`. No task-count increase, missing requirement frontmatter, missing D-01–D-10 action reference or extra standalone frontmatter delimiter was found.
- All declared prerequisite references exist and strictly precede their consumers, establishing an acyclic declared graph. All same-wave `files_modified` sets are disjoint, including02/14 wave11. This mechanical result does not resolve08's undeclared data dependency.
- Plan03 declares only01;03–07 actions do not require02's acquired corpus. They can use their prescribed selected/source facts and owned structural/seeded tests while acquisition remains pending.08 is the first explicit acquired-envelope consumer found in the inspected03–14 actions.
- Plan01's amended link correctly runs from chainstate `canonical_genesis.rs` to consensus `chain_spec/rules.rs`. `gsd-tools verify key-links` verifies its concrete pattern; static source inspection finds `let rules = proof.spec().rules();` in `consensus_params`. No source change is required for that metadata correction. Unimplemented future-plan links were not treated as missing implementation.
- All five CVLD requirements remain represented, and D-01–D-10 retain implementing task references and coverage-ledger acceptance contracts. Existing15/16 acceptance,17 source review,18 offline registration,19 documentation and20 full native/fresh coverage/Bazel/independent source/security/goal/lifecycle gates remain binding. No scope reduction or deferred168–171 work is authorized by this amendment.
- The running acquisition began under original wave1 authorization and remains a pending prefetched prerequisite, as reported by the orchestrator. Task1/raw blocks, elapsed estimates and partial context acquisition do not establish completed02 acceptance. This review did not inspect, restart or mutate the producer, cache, data or logs.

**Informational advisory:** An intentionally delayed independent producer may retain `depends_on: []` when its start/acceptance distinction and actual consumer barrier are explicit. The generic empty-dependency warning alone does not justify inventing an implementation prerequisite. The substantive problem here is the misplaced barrier relative to08, not the lack of a fabricated predecessor.

**Dimension8:** SKIPPED — `workflow.nyquist_validation` is false. The bounded amendment introduces no new research decision; existing research planning resolutions and prior full-review coverage remain applicable.

## Evidence and limits

AGENTS.md Repo-Local Guidance, AGENTS.bright-builds.md, standards-overrides.md, standards/index.md and core architecture/testing/verification/local-guidance plus Rust standards informed ownership, evidence attribution, pure-core boundaries and preserved verifier obligations. Both active lesson inputs were read completely:8856bytes and2953 conservative estimated tokens; no project skill directories were present. Current phase goal/requirement text, context, decision coverage and original review history were consulted. Task byte preservation is a planner-reported claim pending access to an original snapshot; current structural completeness was checked independently. Source reads were limited to the completed01 binding. No application, Cargo/Bazel/build/test execution, data mutation, shared metadata edit, staging, commit or push was performed. Only this appended review section is owned by this checker.

```yaml
issues:
  - plan: "167-08"
    task: 2
    dimension: cross_plan_data_contracts
    severity: blocker
    description: "Wave6 Task2 explicitly consumes exact repeat bodies with02 envelope prerequisites, but amended02 acceptance is delayed until wave11; the amendment incorrectly identifies16 as the first acquired-corpus consumer."
    fix_hint: "Move02 acceptance to wave5 alongside disjoint07 and hold08 until02 fully passes both original tasks/verifications/summary acceptance, or separately review a full scope-preserving restructuring. Correct amendment/ledger consumer claims and retain02->15/16 gates."
  - plan: "167-02"
    dimension: dependency_correctness
    severity: info
    description: "Wave>1 with empty depends_on is an explicit acceptance-delay scheduling exception for an independent producer already prefetched under original wave1."
    fix_hint: "Preserve truthful start provenance and empty implementation prerequisites; place acceptance before the actual first consumer and enforce the barrier."
```

The wave11 scheduling amendment does not pass the revision gate. Return the bounded earlier-consumer finding to the planner; preserve the existing task/evidence scope and completed01 acceptance.

### Dependency-metadata clarification

The08 Task2→02 data prerequisite is inferred from the retained explicit task text and is binding even though absent from `depends_on`. With an enforced whole-wave acceptance barrier,02 accepted in wave5 before08 wave6 is sufficient to preserve the current dependency arrays. Adding167-02 to08's `depends_on` would accurately document a real producer prerequisite and improve future dependency-driven scheduling, but is not required for this bounded wave-order correction if the orchestrator explicitly holds08 for accepted02. This differs from inventing a predecessor for02 itself. Any added edge must be recorded as a separately authorized metadata change, not described as preserved byte-for-byte.


# Phase 167 Plan Check — Scheduling Amendment Review 2

## VERIFICATION PASSED

**Phase:** 167 — Candidate-History Validation Rules

**Scope:** Bounded scheduling revision dated 2026-10-21T17:52:07Z, following Scheduling Amendment Review 1. Earlier reviews and findings remain preserved above.

**Plans checked:** All 20 current frontmatters/structures; 41 complete task blocks; relevant task inputs/actions and acceptance gates.

**Current issues:** 0 blockers, 0 warnings; 1 informational scheduling advisory.

**Lifecycle:** yolo; `167-2026-10-21T16-38-23`.

The prior scheduling blocker is resolved. Plan 02 completion/acceptance is now wave 5 alongside disjoint Plan 07. Plan 08 remains wave 6 and explicitly depends on both `167-02` and `167-07`. Its scheduling context holds the whole plan until both original 02 tasks, both original offline verification commands and the complete attributable 02 summary have passed orchestrator acceptance, and 07 is accepted. The exact-body/UTXO/physical-reopen restoration contract is retained in the current Task 2; missing acquisition cannot be replaced with synthetic bodies or scalar expectations.

## Scheduling and coverage checks

- Plan 03 depends only on accepted 01. Plans 03–07 use selected/source facts, structural context and their owned ordinary/seeded tests; their current actions do not require the acquired 02 corpus. Plan 06 defines the classified historical adapter for later consumption without requiring the acquired corpus to implement that seam. Plan 07's seeded rule/undo cases do not state the acquired-envelope prerequisite that 08 Task 2 explicitly requires.
- Plan 08 is the first explicit acquired-envelope consumer. Both 02 and 07 finish before 08; their wave-5 file ownership is disjoint. Plan 08 and 09 wave-6 ownership is also disjoint. All declared prerequisite references exist and strictly precede their consumers; the graph is acyclic and all same-wave ownership sets are disjoint. Every ordinary wave matches one plus its maximum prerequisite wave; independent 02 has the documented acceptance-delay exception.
- Plan 02 directly gates 08, 15 and 16. The declared graph also carries accepted 02 through 08 to 10–14 and then 15–20. Plan 16 explicitly consumes exact historical script/repeat bytes and envelopes; Plan 18's offline historical checker registration and Plan 20's final verification follow those gates. No later acquired-corpus consumer identified in these contracts precedes acceptance.
- The amendment and decision ledger now identify 08 as the first acquired-envelope consumer and the end of wave 5 as its barrier. Their rejected wave-11 proposal and false earlier consumer claims are clearly labeled superseded history. The added 02→08 edge is explicitly disclosed as a real metadata change; 02's implementation prerequisites remain empty.
- All 20 frontmatter validations pass. All 20 structure validations pass with no errors; all 41 tasks have nonempty files/action/verify/done. The sole tool advisory is 02's intentional `Wave > 1 but depends_on is empty`, classified below as informational. All plans retain the originating lifecycle, and every plan has only its two top-level frontmatter delimiters.
- All five phase requirements, CVLD-01/02/03/04/06, remain in frontmatter and the concrete decision/acceptance matrix. All locked D-01–D-10 retain implementing action references. Current 02/08 tasks and later evidence contracts retain exact historical identities, independent expectations, complete overwrite restoration, classified authority and unchanged-state negatives. No reduced decision scope or deferred 168–171 work is introduced by this schedule revision.
- Plans 17–20 retain independent source review before current guard renewal, offline-only verifier registration, source breadcrumbs/parity/root-owned documentation reconciliation and full default native/fresh pure-core coverage/Bazel plus independent source/security/goal/lifecycle acceptance. The previously checked 01 key-link correction and accepted 01 summary are outside this recheck's source-review scope.

**Informational advisory:** Plan 02 is an independent producer already started under the original wave-1 authorization, as reported by the orchestrator. Wave 5 is its delayed completion/acceptance barrier. Empty `depends_on` truthfully describes its implementation prerequisites; adding an invented predecessor would be incorrect. The new explicit 02→08 edge documents the actual consumer relationship. The active acquisition remains incomplete according to the orchestrator; this plan verdict does not accept its data, Task 2 or summary, or authorize 08 to start early. Elapsed estimates, Task 1 completion and partial cache/data never satisfy the barrier.

**Dimension 8:** SKIPPED — `workflow.nyquist_validation` is false. This bounded revision introduces no new research choice; the existing research planning resolutions and prior full-review assessment remain applicable.

## Evidence and limits

Material guidance: AGENTS.md Repo-Local Guidance, AGENTS.bright-builds.md, standards-overrides.md, standards/index.md and core architecture/testing/verification/local-guidance plus Rust standards. Both active lesson files were read completely: 8,856 bytes and 2,953 conservative estimated tokens; no project skill directories were present. Current context, roadmap goal/criteria, decision coverage, amendment, previous findings and the relevant current plan contracts informed this result.

Historical task preservation is a planner/orchestrator attestation: no independent original pre-amendment snapshot exists. This checker independently verified current semantics and structure; it does not claim historical byte equality or independently reproduce the planner's revision-start hash comparison. Acquisition status and original start provenance are attributed to the orchestrator; no producer, cache, log or data probe was performed. No application, build/test, Cargo/Bazel, Git, source/data or shared metadata operation was run. Only this review section was appended, preserving all earlier report bytes.

```yaml
issues:
  - plan: "167-02"
    dimension: dependency_correctness
    severity: info
    description: "Wave 5 with empty depends_on is an explicit delayed-acceptance exception for the independent producer already prefetched under original wave 1 authorization."
    fix_hint: "No plan revision required. Preserve truthful start provenance and hold 08 until full 02 and 07 orchestrator acceptance; retain direct 02 prerequisites on 15 and 16."
```

The corrected scheduling amendment passes the revision gate. Accepted 01 may feed 03–07 while the original isolated acquisition continues; hold 08 until the full wave-5 producer gates pass. This establishes plan sufficiency, not completed implementation or corpus acceptance.


# Phase 167 Plan Check — Plan 04 Key-Link Metadata Recheck

## VERIFICATION PASSED

**Scope:** Only the two concrete Plan 04 key links amended at `2026-10-21T18:24:56Z`; 0 blockers, 0 warnings. Broader Plan 04 implementation/regression repair and execution acceptance remain pending with the orchestrator.

The links accurately describe the current call edges:

- `packages/open-bitcoin-network/src/header_store.rs:180` calls `self.admit_canonical_header(header, shell_observed_now)`. The registered `canonical_history` child module implements that method at `header_store/canonical_history.rs:109`.
- That method builds earned-parent history and invokes `check_context` before inserting contextual admission. `check_context` calls `history.check_header_rules(now)` at `header_store/canonical_history.rs:226–227`; the `CandidateHistory` method is implemented in `packages/open-bitcoin-consensus/src/candidate_history/header_rules.rs:20`, registered by `candidate_history.rs`.

Independent `gsd-tools verify key-links` reports both links verified (2/2). The plain YAML regex values match the actual source, including the second call's line break; no quoted-regex workaround or source change is needed. Plan 04 frontmatter and structure validations pass without errors/warnings, retaining two complete tasks, wave 3, dependency `167-03`, original `generated_at: 2026-10-21T16:55:59Z` and originating `167-2026-10-21T16-38-23`/yolo lifecycle. The two truths remain the previously reviewed contracts. Historical byte preservation of tasks/actions/dependencies is attributed to root; this check does not claim an independent original-byte comparison.

This evidence establishes that the corrected metadata names executable consumer/provider edges. It does not establish full header-validation correctness, passing regressions, completed Plan 04 or phase acceptance. The initial failed single-quoted-regex attempt is root-reported history; the current unquoted form was independently verified. No build/test/application/Git command was run, and no plan/source/data/shared metadata was edited. Earlier report bytes are preserved; only this section was appended. Previously loaded AGENTS.md/sidecar/overrides and architecture/verification/local-guidance standards informed the evidence and ownership limits.

```yaml
issues: []
```


# Phase 167 Plan Check — Plan 05 Bounded Ownership Amendment Review

## VERIFICATION PASSED

**Phase:** 167 — Candidate-History Validation Rules.

**Scope:** Plan 05 ownership amendment dated `2026-10-21T18:37:27Z`, including the concrete key-link metadata update dated `2026-10-21T18:40:43Z`. This is a new evidence-driven bounded amendment review. Earlier report blocks remain preserved.

**Plans checked:** Plan 05's two tasks and original snapshot; related phase ownership/dependency frontmatter, locked context and decision-coverage contracts.

**Current issues:** 0 blockers, 0 warnings, 0 informational findings.

**Lifecycle:** yolo; `167-2026-10-21T16-38-23`.

## Independent scope comparison

The original snapshot `.local/open-bitcoin-dev/phase167/plan05-amendment/before-PLAN.md` independently hashes to SHA-256 `95100f785e587f65ffd87d91a02f303d447f1ae85b06b917e8bb65f071ef51b2`, matching the supplied original identity. Comparing its complete text with the amended plan confirms that original phase/plan/type, wave 3, dependency `167-03`, CVLD-02/CVLD-03 requirements, autonomy, generated_at and originating lifecycle are unchanged. The original objective, interfaces, threat model, final verification, success criteria and output sections are byte-equal. Both original user-observable truths remain unchanged.

Exactly four file owners are added: consensus `script/sigops.rs`, consensus `script/encoding.rs`, the one existing script counting assertion owner, and mempool `policy.rs` for test assertions only. Task 2 now spells out the shared-counter repair, one shared syntax decoder beneath the existing execution-limit wrapper, exact bounded test corrections and affected-package regressions. Task 1 adds the source-correct conditional use of creating-ancestor MTP and its positive/negative controls. Added must-have artifacts/links describe this same work. No original acceptance obligation is removed or reduced, and no production mempool policy, broad interpreter/classifier refactor, additional parser, crate, dependency, acquisition/data, guard, deadline or Phase 168 work is authorized.

There are two complete tasks and ten declared paths; two of those paths permit only the identified existing test corrections. The additional implementation is confined to the existing shared counter and a small decoding seam, with new focused cases in the already-owned sigop test module. This is a bounded prerequisite repair for the same aggregate-accounting concern, rather than an additional subsystem or an oversized task decomposition. Static ownership inspection finds none of the four additions in another Phase 167 plan. All twenty declared dependency references exist and precede consumers; ordinary dependency waves remain consistent. The independent Plan 02 delayed-acceptance exception and acquired-envelope barrier are unchanged.

## Pinned behavior and preserved controls

- Pinned `script/script.cpp:GetSigOpCount(bool)` stops at failed GetOp and returns the prior count. The amended exact expectations `[ac,4c] -> 1` and `[01] -> 0`, truncated direct/PUSHDATA forms and pushed-opcode exclusion follow that source. Accurate multisig uses only immediate OP_1 through OP_16; current `decode_small_int_opcode` already excludes OP_0. The existing OP_0 count 20 is correctly retained as a passing control, with no invented fix or required RED.
- Pinned `GetSigOpCount(scriptSig)` and `GetScriptOp` clear raw data on each opcode. The plan preserves full-scriptSig scanning, zero for malformed outer suffixes or opcode above OP_16, and final raw push selection for P2SH and nested witness. A trailing OP_1 clears the previous redeem data; a complete pushed malformed redeem script still contributes its count-so-far. Native P2WPKH, accurate P2WSH, WITNESS gating and separate execution refusal remain explicit.
- The existing Rust counter currently uses execution's `read_instruction`, which also rejects elements above 520 bytes. Pinned GetScriptOp performs syntax decoding without that execution limit. The planned shared syntax-only seam resolves the actual mismatch without copying a parser into the block accumulator or relaxing execution. The paired requirement is exact: a complete 521-byte push followed by CHECKSIG counts 1 while the same script still fails eval_script with `PushSize(521)`.
- The existing consensus test combines real eval_script opcode/truncated-push refusals with the source-inaccurate `[01]` counting error assertion. Only that counting expectation may become exact `Ok(0)`; the execution assertions and existing oversized-push rejection remain binding. The existing mempool test contains all four identified malformed legacy-input, legacy-output, P2SH-redeem and P2WSH-script cases; each has zero counted operations under pinned semantics. Requiring exact `Ok(0)` for those preserved cases corrects observation rather than authorizing malformed execution. Input-context length checks, independent standardness limits and all execution/parser controls remain unchanged.
- Pinned `consensus/tx_verify.cpp` and ConnectBlock support the retained legacy scale 4, flag-dependent P2SH scale 4, unscaled witness, coinbase inclusion and aggregate maximum 80000. The amended plan retains exact 80000 acceptance/80001 refusal, transaction aggregation, flags, witness scaling, checked arithmetic and real spent-output failures. Existing owned tests include counter atomicity after budget/overflow failure and missing/extra coin-count refusal; the amendment does not authorize changing those controls.
- Pinned CalculateSequenceLocks consumes creating-ancestor MTP only for enforced version >= 2, non-disabled, time-type sequences. Task 1's clarification therefore removes an unjustified unused-history prerequisite while preserving actual candidate-branch ancestry whenever consumed, same-block parent MTP, required-history refusal and branch-disagreement controls. Caller coin metadata cannot substitute for required history.

## Wiring, coverage and verification

The task actions explicitly connect the shared script implementation through existing public Result-returning wrappers to the checked block accumulator and contextual validator, with the later real coins owner using the same budget before spending inputs. The final metadata refinement removes the inaccurate direct private-module edge and names the concrete current block-to-budget, budget-to-public-wrapper and wrapper-to-shared-counter edges; `gsd-tools verify key-links` independently confirms those three patterns. The fourth entry explicitly plans `read_instruction_syntax(` between the shared counter and encoding owner. The tool currently reports that pattern absent, as expected before the authorized implementation. This is a complete planned wire, not a pre-execution requirement that future code already exist. Root requires the executor to verify all four links after implementation and turn every retained RED GREEN before acceptance. No claim is made that the future syntax seam already exists or that all four links currently pass.

`gsd-tools verify plan-structure` passes Plan 05 with two tasks and no errors or warnings. Both tasks retain nonempty files/action/verify/done. Original focused commands remain, and Task 2 adds the serial timing-wrapper command `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-consensus -p open-bitcoin-mempool`. Pinned Bun PATH, cooperative lock, nonzero named discovery, complete actual failures/retries and attributable RED/GREEN remain mandatory. The original 17-test/7-failure RED and paused newly owned source boundary are orchestrator-reported execution state; this review neither reruns nor accepts that evidence. Already-passing controls need no fabricated RED.

Roadmap goal, relevant PROJECT scope, CVLD-02/CVLD-03 and D-03/D-04/D-08/D-09 are consistent with the amendment. The decision ledger retains the complete five-requirement and D-01–D-10 matrices, downstream real interpreter/durable integration, independent evidence classes and unchanged final native/fresh coverage/Bazel/source/security/goal/lifecycle gates. No deferred work or reduced evidence class is substituted. This amendment introduces no unresolved research choice; the existing research planning resolutions remain applicable.

**Dimension 8:** SKIPPED — `workflow.nyquist_validation` is false.

## Evidence and limits

Material guidance: AGENTS.md Repo-Local Guidance, AGENTS.bright-builds.md, standards-overrides.md, standards/index.md, core architecture/code-shape/testing/verification/local-guidance and Rust standards. Both canonically distinct active lesson files were read completely: 8856 bytes and 2953 conservative estimated tokens. The existing October 19 lesson-audit baseline requires no new audit for this task; no project skill directories were present.

Only this review section was appended. No plan, source, data, summary, acquisition artifact, shared metadata or earlier report block was edited; no application, build/test, Cargo/Bazel, Git operation or agent spawning was performed. Static Node/Python inspections and source reads establish plan sufficiency only. The amended ownership may proceed through root's acceptance gate; completed implementation, passing regressions and phase acceptance remain separate executor/orchestrator obligations.

```yaml
issues: []
```


# Phase 167 Plan Check — Plan 06 Bounded Ownership and Provenance Amendment Review

## VERIFICATION PASSED

**Phase:** 167 — Candidate-History Validation Rules.

**Scope:** Plan 06 amendment dated `2026-10-21T18:57:48Z`, reviewed at `2026-10-21T19:03:51Z`. This bounded review preserves all earlier review blocks and accepted dependency verdicts.

**Plans checked:** Plan 06's two tasks, original snapshot, amendment and targeted decision-coverage/producer-consumer contracts.

**Current issues:** 0 blockers, 0 warnings, 0 informational findings.

**Lifecycle:** yolo; `167-2026-10-21T16-38-23`.

## Independent scope and structure evidence

The complete original snapshot `.local/open-bitcoin-dev/phase167/plan06-amendment/before-PLAN.md` independently hashes to SHA-256 `c11ed18b662e416b8e611f351b030e906881bfa04c04b4643391e6999af16234`. Direct comparison confirms exactly two added production owners: `engine/canonical_genesis.rs` and `engine/work_authority.rs`. No original owner is removed. Task 2 is byte-identical. Wave 4, dependencies `167-04`/`167-05`, CVLD-01/02/03/04 requirements, original generated_at, lifecycle mode/identity and two-task count are unchanged. The two original truths, objective and retained verification/output contracts remain intact.

Frontmatter validation and `gsd-tools verify plan-structure` independently pass with two complete files/action/verify/done tasks and no errors or warnings. Ten declared paths comprise seven Task 1 owners and five Task 2 owners with shared integration files; the addition repairs the same initialization/transfer boundary rather than adding a subsystem. The existing accepted Plan 01 owns the genesis initializer earlier in the dependency chain; no other plan owns the added work-authority file, and Plan 06 has no same-wave peer. Current declared prerequisite references exist and precede consumers. Independent Plan 02 acquisition is neither consumed nor changed by this amendment; its existing later acceptance barrier remains binding.

## Authority and data-contract assessment

Static inspection of actual `engine.rs`, `canonical_genesis.rs`, `work_authority.rs` and `stage.rs` confirms the stated defect and proposed seam. Default/generic roots and sealed-genesis roots currently both use `WorkOrigin::Live`; that label cannot establish selected canonical identity. Existing Clone and consuming rebind reconstruct a fresh WorkAuthority and preserve selected params; the latter checks complete effective coins, best block, active metadata, undo and counts before transfer. StageBinding privately binds owner/generation/tip/seal, with one authorized preview continuation. These existing owners are the necessary locations for this bounded correction.

The revised action initializes private `maybe_canonical_rules` only through successful borrowed `CanonicalGenesisProof` construction after existing empty-coins/root checks. It preserves Plan 01's complete height-zero parameter mapping, subsidy/maturity, exact work and empty genesis UTXOs. Legitimate non-preview canonical Clone/rebind retain immutable selected facts with fresh identities; pending admissions and old bindings cannot transfer. Synthetic Live roots, matching root labels, raw snapshots/config/params/copied contexts and fixture/untrusted origins cannot mint selection. Mutable-coins invalidation removes canonical authority, and acceptance must preserve earned lineage without inferring it from a binding's Live label. Preview suspension/continuation remains subject to the existing private seal rather than creating another authority path.

Chainstate's actual Cargo dependencies are only consensus/primitives. The amendment retains that direction and introduces no public implementable authority trait, duplicate work owner, production dependency or extra owner. The explicit pure admission operation consumes trusted-shell observations as data; it does not claim a copied public context proves HeaderStore membership. Actual HeaderStore ownership/original-clock coordination remains with accepted Plans 09/10.

The planned pending admission is one finite, checked owner/generation-scoped batch, cleared on generation changes, acceptance, invalidation and transfer. It binds exact header bytes/hash, earned parent/branch, selected rules and the original successful shell observation. Replacement admission checks the requested disconnect prefix and each prospective predecessor; body staging independently derives each actual coins branch again after staged disconnect/connect and checks full-width work. Existing HeaderStore admission and CandidateHistory source were inspected to confirm the structural-data versus held-authority distinction. Repeated admission retains the first successful observation, and body checking reuses it rather than candidate.time or a new replay clock. Missing/stale/foreign/replayed evidence must refuse before accepted overlay absorption. No unbounded second header index is authorized.

The seeded-historical adapter retains its explicit fixture origin, exact serialized identity and complete envelope prerequisites while sharing the actual derived-rule/apply kernel. A genesis-shaped prefix cannot promote it into ordinary runtime/proof authority. Missing-context, checkpoint and Signet refusals remain. The corrected current-header-plus-ten-parents MTP window and complete linked-history requirement are explicit.

Task 2 retains real overlay-coin input contexts, candidate creating-ancestor MTP, actual transaction/interpreter calls, every named consensus flag and policy separation, aggregate 80000/80001 sigop behavior, selected rewards plus fees, maturity 100 and unchanged accepted-state controls. No scalar calculator or historical fixture is substituted for those obligations. Concrete must-have links now connect proof selection, checked branch history and actual transaction validation; future CandidateHistory wiring is planned work, not a current-source passing claim.

## Coverage, verification and limits

Roadmap goal, relevant PROJECT scope, CVLD-01/02/03/04 and D-02/D-03/D-04/D-05/D-07/D-08 remain represented by specific task actions and measurable positive/refusal evidence. The targeted decision ledger agrees. Previously accepted phase coverage, including separate CVLD-06 ownership, remains intact; no locked decision is reduced or deferred168–171 work added. Existing research planning resolutions remain applicable and this amendment introduces no unresolved research choice.

**Dimension 8:** SKIPPED — `workflow.nyquist_validation` is false.

Material guidance: AGENTS.md Repo-Local Guidance, AGENTS.bright-builds.md, standards-overrides.md, standards/index.md and core architecture/code-shape/testing/verification/local-guidance plus Rust standards. Both canonically distinct active lesson files were read completely: 8856 bytes and 2953 conservative estimated tokens. No project skill directories were present. Existing review history records the recent lesson-audit baseline; no active-ledger change or new audit trigger was introduced.

The executor's 0/2-task checkpoint and absence of Plan 06 source edits are orchestrator-reported state, not an implementation acceptance claim. Only this review was appended; earlier report bytes remain preserved. No source/plan/data/acquisition/shared metadata edits, tests/builds/application execution, Cargo/Bazel/Git operation or agent spawning were performed. Actual RED/GREEN, nonzero focused test discovery, final owned-source checks and attributable summary remain executor obligations. Full native/fresh coverage/Bazel and independent source/security/goal/lifecycle acceptance remain later orchestrator gates.

```yaml
issues: []
```

The bounded amendment passes the revision gate and may be released by root for execution within its exact ownership. This verdict establishes plan sufficiency, not completed implementation or phase acceptance.


## review-plan06-locktime-amendment | 2026-10-21T19:27:32Z

### VERIFICATION PASSED

**Phase:** 167 — Candidate-History Validation Rules. **Scope:** bounded Plan 06 locktime interpreter amendment dated `2026-10-21T19:20:54Z`, at the executor's 1/2-task continuation checkpoint. **Current issues:** 0 blockers, 0 warnings, 0 informational findings. This verdict checks planning sufficiency; it does not accept unfinished implementation or the phase.

### Independent snapshot and ownership evidence

The complete frozen `.local/open-bitcoin-dev/phase167/plan06-locktime-amendment/before-PLAN.md` independently hashes to `e1dda77d7a31215314d0590a016be4ab0964508e5767365d2cfb28debe0bc778`. Full-text comparison confirms byte-identical Task 1; unchanged phase/plan/type, wave 4, dependencies 167-04/05, CVLD-01/02/03/04, autonomy, generated_by, yolo lifecycle `167-2026-10-21T16-38-23`, generated_at `2026-10-21T16:55:59Z`, both user-observable truths and two-task count. No former owner is removed. The sole new key link is concrete executed `legacy.rs → locktime.rs` through `locktime::`; it is planned wiring, not a current-source completed claim.

Exactly five consensus source owners are added to Task 2: `src/script.rs`, `src/script/legacy.rs`, `src/script/opcodes.rs`, `src/script/stack.rs`, and new `src/script/locktime.rs`. Static source inspection establishes why each is needed: module and exhaustive Display/error registration, instruction dispatch, opcode identity, the existing four-byte signed decoder, and the cohesive pure transaction-lock helper. Existing LegacyExecutionContext already carries actual transaction/input/flags; witness and tapscript execution already reuse eval_script_internal. No context/checker, export, dependency, crate, witness/taproot, external test registrar or additional test-file owner is needed. New tests stay in the existing phase167_scripts.rs owner; Plan 19 retains manifest ownership for the new breadcrumb.

The historical plan union is now 15 paths, with seven unchanged Task 1 owners and ten Task 2 paths sharing existing integration/registrar owners. Scope was assessed at this explicit continuation checkpoint: the new implementation is five tightly bounded owners for the two missing lock opcodes, while Task 1 is already implemented and its text is frozen. This is not authorization to reopen all 15 owners or add a subsystem. A fresh full-plan execution would need renewed scope assessment; this bounded continuation preserves the accepted decomposition and requires a fresh executor context. No added owner overlaps another phase plan's files_modified; Plan 05's related counter/decoder/test owners remain read-only dependencies. Wave/dependency and independent Plan 02 acquisition barriers remain unchanged.

### Pinned semantics and executable evidence contract

Independent local hashes match the amendment: interpreter.cpp `6e7367740f8d8ecc8dca4ec129d66782d09fa5733acc96469f367267fb42cc60`; script.h `ebd3aa6de49ac597957923328bca3abc8697e63f2b0c3b06d260f6ebdd89016c`. Pinned EvalScript 521–590, separate NOP policy 592–600, CheckLockTime/CheckSequence 1747–1830, CScriptNum and BaseSignatureChecker were checked against current Rust seams.

- Absent CLTV/CSV flags immediately act as NOP2/NOP3, without operand/context access. DISCOURAGE_UPGRADABLE_NOPS belongs to separate NOP1/4–10 cases and must not reject these inactive branches. Existing conditional skipping stays intact.
- Active opcodes require one item and peek without popping or replacing it. Signed-magnitude i64 decoding explicitly permits five bytes only here, preserves all existing four-byte callers and errors, rejects six bytes, checks negative before CSV disable, and handles negative zero/nonminimal encoding according to explicit MINIMALDATA. No premature u32 narrowing or unrelated policy retrofit is planned.
- CLTV compares actual transaction lock_time against the fixed 500000000 threshold, requires matching height/time type and operand <= transaction value, and rejects this actual input's final sequence. Equality succeeds; block/MTP/fixture thresholds are not substitutes.
- CSV's nonnegative operand-disable NOP precedes transaction checker access; otherwise version >= 2, input disable-bit refusal, type/mask equality and masked <= comparison match the pin. Other bits are ignored. Existing BIP68 age conversion remains separate. Missing comparison context fails closed rather than fabricating success.
- Typed NegativeLockTime/UnsatisfiedLockTime plus existing stack/size failures, concrete dispatch and OP_DROP-dependent non-consumption are required. Tests cover positive/equality/refusal, signed five-byte boundaries including u32::MAX and above, six-byte rejection, negative/negative zero/minimality, CLTV threshold/final input, CSV version/disable/type/mask/ignored-bit order, inactive flags with DISCOURAGE, skipped branches and retained four-byte arithmetic rejection.

Funded output and transaction execution remain mandatory. Reachable positive/refusal/bypass pairs use ordinary selected canonical stages. Boundary probes that separate BIP113/BIP68 checks would reject earlier instead use existing public verify_input_script with actual funded output and derived input context, explicitly labeled interpreter evidence. Those probes do not prove canonical acceptance or mint stage/product authority. Invalid staged bodies must preserve accepted tip/exact work, best-block/outpoints/undo without absorption. The helper is pure and grants no acceptance authority.

The original task2-red-behavior.log independently shows CLTV/CSV failing at the single-rule omission controls with unsupported opcode 0xb1/0xb2; these are missing-feature RED, not canonical-bypass RED. The remaining seven reported behavioral failures and subsequent 7/9 result are retained separately. The amendment requires fresh successful active-opcode controls and genuine omission/refusal pairs, preserving the earlier failed/setup logs and corrected MTP inverse attribution. The 400-unit/3-integration/23-doctest regression and 21 focused results excluded the two locktime cases and remain partial historical evidence only.

### Coverage, checks and acceptance boundary

CVLD-01/02/03/04 and D-02/03/04/05/07/08 retain specific actions and observable acceptance; existing separate CVLD-06 ownership is unaffected. Rewards plus actual fees at 149/150/299/300, maturity 100 versus depth 99, no genesis outputs, LOW_S/MINIMALDATA/CLEANSTACK separation, consumed-only creating-MTP, same-block/branch metadata, all other interpreter flags and aggregate 80000/80001 remain outstanding wherever not executed. Task 1's original-clock, branch, owner/generation, clone/rebind, fixture taint and finite-admission protections are unchanged. No locked decision is reduced and no deferred 168–171 work is introduced. Existing research resolutions introduce no new unanswered choice for this bounded amendment.

Independent gsd-tools frontmatter validation returned valid:true with no missing fields. verify plan-structure returned valid:true, two complete files/action/verify/done tasks, zero errors/warnings. must_haves was extracted and manually checked against the actions. Exact text comparison and hashes confirmed the preserved fields/delta above and only two standalone frontmatter delimiters. The final planned command explicitly tests both consensus and chainstate with --all-features through the timing/lock wrapper, without skip/exclusion flags; final discovery and a non-skipped affected regression remain executor obligations. Existing numeric/script tests must pass unchanged. No earlier partial run can satisfy that gate.

**Dimension 8:** SKIPPED — workflow.nyquist_validation is false. Project compliance was informed by local AGENTS.md, AGENTS.bright-builds.md, standards-overrides.md, standards/index.md, architecture/code-shape/testing/verification/local-guidance and Rust standards. Both distinct active lesson files were measured and read completely: 6898 + 1958 = 8856 bytes; 2300 + 653 = 2953 conservative tokens. No project skill directories exist. Relevant PROJECT/roadmap/requirements, CONTEXT/RESEARCH/decision coverage, both amendments and executor checkpoint were reviewed. No external library/API choice or online research was needed.

Only this review block was appended, preserving all previous bytes. No source/plan/data/acquisition/shared-metadata changes, Git operation, builds/tests/application execution, Cargo/Bazel or subagents were used. Full native/fresh coverage/Bazel and independent source/security/goal/lifecycle acceptance remain later gates controlled by root; this result grants no completed-source or phase-support claim.

```yaml
issues: []
```

The bounded locktime amendment passes the revision gate for root to release within these exact owners and retained evidence obligations.


## review-plan07-08-constructor-amendment | 2026-10-21T19:33:45Z

### ISSUES FOUND

**Phase:** 167 — Candidate-History Validation Rules. **Scope:** bounded Plan07/08 constructor compatibility amendment dated `2026-10-21T19:26:38Z`, including only its two concrete planned key-link corrections. **Plans checked:** 2. **Issues:** 1 blocker, 0 warnings. The behavior, authority and snapshot-preservation checks pass; Plan08 exceeds the checker role's explicit file-scope blocker threshold. This is a planning verdict, not implementation or phase acceptance.

### Independent original and normalization evidence

Both local original snapshots independently match their supplied SHA256 identities: Plan07 `04dffe61b588938778c24ca43bc968f564870fdb44a0f3a682f4b6ca54c88992`; Plan08 `8ce2d90856e056753b5b4e5a7f81cc248295cb25bbc4764bc8d80dae364d2522`. A full-text normalization removing only added owner paths, constructor-amendment blocks and appended action instructions, and restoring only the original key-link entries, produces byte-identical originals for both plans. This establishes preservation beyond selected metadata comparisons.

Each retains exactly two tasks; every original name, read-first, behavior, acceptance, automated verification and done element is byte-equal, with original actions retained as exact prefixes. Plan08 Task2 is byte-identical. Original requirements `[CVLD-06]`, autonomy, lifecycle `167-2026-10-21T16-38-23`/yolo, generated_by and generated_at `2026-10-21T16:55:59Z`, both truths/artifacts, objectives/interfaces/threat models/final verification/output and scheduling text remain unchanged outside the identified additions. Plan07 stays wave5 after06. Plan08 stays wave6 after02 and07, with the whole-plan barrier requiring both original02 tasks/checks and its attributable accepted summary plus accepted07. Partial acquisition, Task1-only completion and elapsed estimates cannot release08.

Independent `gsd-tools.cjs frontmatter validate --schema plan` and `verify plan-structure` pass both amended plans with two complete tasks each, no errors or warnings. Both have only opening/closing standalone frontmatter delimiters. Current declared phase dependencies exist, are acyclic and precede consumers; Plan02's documented delayed-acceptance exception remains. No added owner is declared by another current167 plan, so the additions create no ownership conflict or new edge.

### Constructor, production and authority assessment

Read-only `rg` and literal-context inspection confirm exactly the amendment's two added chainstate owners and eleven added node fixture owners, at all listed lines. Chainstate has four actual direct literals across its two added files; node has twelve across its eleven added files. The already-owned snapshot_codec/tests.rs:146 literal is separate and correctly not counted as a new owner. `HistoricalBlockUndo` occurrences wrap references. `stage.rs::undo() -> &BlockUndo`, the foreign-undo return signature, the BlockUndo definition and its TryFrom implementation header are not fixture constructors. Safe Default callers need no ownership expansion.

The planned explicit empty overwrite collection is confined to existing ordinary/nonoverwrite or intentionally malformed transaction-undo constructors. The identified genesis-nonempty-undo, missing-created-output, corrupt-inner-undo, restore-collision and both Phase165 changed-undo refusal controls remain binding. All previous inputs, assertions, expected errors and fault injections must remain unchanged. No newly added path authorizes a production adapter, guard/source-ownership renewal, frozen artifact or acquired corpus edit.

Static production inspection confirms BlockUndoDto and its real From/TryFrom conversions reside in the already-owned snapshot_codec.rs; decode_block_undo accepts only bytes and cannot infer candidate ancestry. The amended contract preserves full previous Coin/output/value/script/height/coinbase/MTP, outpoint/new-occurrence/block binding and bounds through both per-block and whole-snapshot routes. Missing legacy overwrite evidence remains unproved and must refuse as StorageError unless specific complete prerequisites genuinely prove nonoverwrite. serde(default), unwrap_or_default, empty collections and caller claims cannot substitute. In-memory Default stays nonauthorizing. Complete positive roundtrips and missing/malformed/duplicate/binding atomic refusals remain mandatory.

Plan07's concrete overlay_apply.rs → engine/overwrite_undo.rs edge requires the actual apply_disconnect_on_overlay call to planned restore_overwritten_coins. Plan08's snapshot_codec.rs → snapshot_codec/overwrite_undo.rs edge requires real encode_overwrite_records/decode_overwrite_records calls from DTO conversion. Both providers are original planned owners; actions explicitly forbid comment-only pattern satisfaction. These are sufficient planned wires, with no claim that unimplemented helpers currently exist or that a future source key-link check passed.

D-06/D-08/D-09 and CVLD-06 retain exact BIP30 admission, both repeats, BIP34-ancestor/recheck boundary, unrelated-duplicate refusal, complete prior-coin restoration, physical checkpoint/all-handle release/reopen/disconnect, unchanged accepted-state negatives and the intentional stronger pinned-disconnect difference. Seeded historical evidence remains noncanonical and non-differential. Coverage notes agree; no original evidence is reduced or fixture authority promoted. Existing research resolutions remain applicable; this amendment introduces no unresolved behavioral choice. Other previously accepted phase requirements/decisions and downstream native/coverage/Bazel/source/security/goal/lifecycle gates remain outside this bounded amendment's changes.

### Scope blocker

Plan07 owns9 files, with task file counts5/6. Plan08 owns18 files, with task file counts15/3. Eleven Plan08 additions are proven mechanical fixture adaptations, but the checker role defines15+ files per plan as a blocker and separately calls out10+ files in one task. Plan08 is unexecuted; unlike the prior Plan06 continuation checkpoint, no completed task reduces its remaining execution scope. Neither the amendment nor current instructions provides an explicit exception to that threshold. The structural validator's zero warnings do not evaluate this scope dimension.

Preserve full behavior, original acquisition barrier, frozen provenance and evidence obligations when resolving scope. A reviewed serialized compatibility subplan/step split can keep constructor adaptation explicit and independently bounded before codec/durable execution; any resulting task/dependency amendment must be reviewed rather than silently changing this constructor amendment. Root must not treat this report as an implementation release for08.

**Dimension8:** SKIPPED — workflow.nyquist_validation is false. AGENTS compliance is informed by AGENTS.md Repo-Local Guidance, AGENTS.bright-builds.md, standards-overrides.md, standards/index.md, core architecture/code-shape/testing/verification/local-guidance and Rust guidance. Both canonically distinct active lesson inputs were read completely:6898+1958=8856 bytes;2300+653=2953 conservative tokens. The October19 lesson-audit baseline remains applicable; no new audit trigger or lesson edit was introduced. No project skill directories were present.

Only this review block was appended, preserving all prior report bytes. No source, plan, shared metadata, guard, acquisition/cache/corpus or frozen-original edit; no Git operation, application execution, tests/builds, Cargo/Bazel or subagent spawning occurred. Frontmatter/structure, hashing, read-only source inventory and exact normalization are the actual checks. Implementation remains unexecuted.

```yaml
issues:
  - plan: "167-08"
    dimension: "scope_sanity"
    severity: "blocker"
    task: 1
    description: "Unexecuted Plan08 declares18 files, exceeding the checker role's explicit15+ blocker threshold; Task1 alone declares15. Eleven are mechanical fixture constructors, but no explicit threshold exception is authorized."
    metrics:
      tasks: 2
      files: 18
      task_1_files: 15
      added_mechanical_fixture_owners: 11
    fix_hint: "Review a scope-preserving serialized compatibility split before08 execution; retain every original codec/durable obligation, complete02+07 gate, lifecycle provenance and frozen evidence. Do not reduce checks or silently alter the current two-task contract."
```


## review-execution-scope-amendment | 2026-10-21T19:53:38Z

### VERIFICATION PASSED

**Phase:** 167 — Candidate-History Validation Rules. **Scope:** bounded execution-scope amendment dated `2026-10-21T19:39:52Z`, including Plans06–25, the schedule/coverage ledger and concrete downstream key-link metadata. **Inventory:** 25 plans, 44 tasks, 22 waves. **Current issues:** 0 blockers, 0 warnings, 0 informational findings. **Lifecycle:** yolo; `167-2026-10-21T16-38-23`.

This revision resolves the prior unexecuted Plan08 scope blocker by separating the eleven mechanical fixture owners into22/24, retaining all production/legacy/durable obligations in08. It also separates completed06 Task1 from the remaining opcode and actual-coins responsibilities, and relocates original19 Task3 intact to root-applied25. This is plan sufficiency, not completed implementation, acquisition, source correctness or phase acceptance.

### Independent preservation and task mapping

All44 entries in `execution-scope-amendment-20261021T193711Z/snapshot-manifest.json` independently match the corresponding `before/` existence, byte count and SHA256. Current accepted01–05 plans, all15 original06 source owners and the complete checkpoint match their recorded revision-start identities. All21 retained execution logs match both original paths and preserved evidence copies:559938bytes. The planned sibling `phase167_locktime.rs` is still absent. No source rollback, test rerun or attribution-only RED was needed.

Independent complete-task comparison finds41 original tasks and44 revised tasks. Exactly36 original task blocks remain byte-identical:01–05 all tasks;06.1;07 both tasks;08.2;09–18 all tasks;19.1;20.1. The five remaining mappings are fully accounted for:

| Original task | Current allocation | Preservation assessment |
| --- | --- | --- |
|06.2|21.1 +23.1|Pure CLTV/CSV and direct funded interpreter boundaries in21; actual canonical coins/scripts/monetary/policy/creating-MTP controls and final complete affected regression in23. Original behavior and acceptance remain jointly mandatory. |
|08.1|08.1 +22.1 +24.1|Original production action, both DTO directions, legacy refusal, per-block/snapshot validation and focused test remain08. Eleven ordinary/malformed literal adaptations divide6+5 into22/24 without dropped assertions;08 adds the paired unfiltered node suite. |
|19.2|19.2|Only the reference to relocated Task3 changes to25; all documentation obligations remain. |
|19.3|25.1|Entire task block is byte-identical after changing only the Task3 label to Task1. |
|20.2|20.2|Only the root excerpt reference changes from19 to25; independent canonical source/security/goal/lifecycle and clean saving gates remain. |

All41 original user-observable truths and all original artifact `provides` descriptions remain present across current plans. Original plan generation timestamps, requirements, autonomy and lifecycle fields remain unchanged; new21–25 use their actual new generation timestamp and originating lifecycle. Threat T-167-06-02 is explicitly carried into23, while the pure interpreter/probe and mechanical handoff threats receive specific controls. Accepted01–05 stay frozen;05's historical10-owner metadata is acknowledged without reopening accepted work solely for that count.

### Executable scope and dependency assessment

| Revised slice | Tasks / owners | Wave | Required handoff |
| --- | --- | --- | --- |
|06|1 /7|4|Completed original Task1 only; attributable summary links21/23 and cannot accept whole original06. |
|21|1 /8|5|Five pure consensus owners plus new direct-probe sibling, existing helper visibility and registrar. |
|23|1 /5|6|Actual apply/overlay/candidate context and canonical script tests; final non-skipped affected regression. |
|07|2 /9|7|Both original exact BIP30/complete undo tasks after06+23. |
|22,24|1 /6;1 /5|8|Disjoint mechanical existing node fixtures after07; explicit pending paired compilation. |
|08|2 /7|9|Complete02+07+22+24; full codec, legacy refusal, physical all-handle restore and unfiltered node compilation/tests. |
|09–19|2 each /5–9|10–20|Preserved behavior through ingress/runtime/reopen/product/corpus/guards/docs in dependency order. |
|25|1 /2|21|Root applies original19 Task3 AGENTS/PROJECT excerpts after19, before20. |
|20|2 /5|22|Final full native/fresh coverage/Bazel and independent source/security/goal/lifecycle acceptance. |

All revised/future06–25 plans own at most9files, with at most8files in any revised task. The single-task slices each have one coherent responsibility; no scaffold padding or artificial ownership shuffle is required. All declared predecessors exist and precede consumers; the graph is acyclic. Ordinary waves equal one plus the maximum predecessor wave. Same-wave owner sets are disjoint and every shared owner is transitively serialized, including06→21→23→07 and08→09. Cargo/Bazel work remains serialized through the timing/cooperative-lock contract.

Frozen02 is independently byte-equal, with its historical wave5/empty dependencies and original prefetched authorization retained. Current amendment, schedule, coverage ledger and08 scheduling context explicitly require its effective complete acceptance after07 wave7 and before08 wave9. Both original02 tasks, both original offline commands and the complete attributable accepted summary are mandatory. Task1, partial acquisition, elapsed estimates or the frozen wave field cannot release08. Explicit02 dependencies on08/15/16 remain. The generic structure advisory for this documented frozen producer is not an unresolved finding; no invented predecessor, producer restart or data/cache/log mutation is warranted.

### Behavioral and evidence fidelity

The latest06 checkpoint was read completely, including19:35Z:Task1 complete10/10; genuine missing-opcode RED2/2; initial canonical/script GREEN11/11; boundary setup error followed by15/15; and a separate actual149/150/299/300 reward+10000sat-fee/+1sat control1/1 added later. No combined16-case or broad final regression is claimed. Earlier imports, MTP-vector/12-versus11 inverse correction, coinbase encoding/setup errors, seven real original Task2 REDs, Clippy failure/fix,400unit+3integration+23doctest partial regression and21focused results with two skips remain attributed as recorded. That history cannot satisfy final affected Clippy or non-skipped regression.

21 preserves signed five-byte parsing, four-byte existing callers, typed errors, operand peek/non-consumption, inactive NOP2/NOP3 including DISCOURAGE, skipped branches, negative/minimality/size boundaries, fixed500000000 types, actual final input, CSV version/input/operand-disable/type/mask/ignored-bit/checker order. Direct funded probes remain interpreter evidence. Canonical positives/omissions stay23, including pending genuine CSV ignored-bit and operand-disable/version1 stages.23 explicitly retains maturity100 versus99-depth, zero genesis outputs, actual LOW_S absence with explicit policy rejection, MINIMALDATA/CLEANSTACK separation, consumed-only distant creating-MTP, missing/inconsistent saved history,4096bounds, same-block locks and replacement-branch disagreement. Actual aggregate80000/80001 and full accepted-state/original-clock/stage protections remain required.

22/24 preserve every existing input/assertion/error/fault and permit only explicit empty new-format collections in the inventoried ordinary/malformed literals. Their scoped rustfmt/diff handoff is deliberately paired with08's unfiltered `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-features`, after production DTO conversion. Neither claims compilation while the DTO is pending.08 retains both original focused commands and complete physical checkpoint/all-handle release/reopen/disconnect evidence. Missing-field serde/default/empty fallbacks cannot prove legacy nonoverwrite; complete previous Coin and occurrence/block bindings remain mandatory.

### Coverage, wiring and final gates

| Requirement | Retained concrete implementation/evidence owners |
| --- | --- |
|CVLD-01|01/06/23/10/12/14/15: selected subsidy, ordinary maturity, real managed301history and physical reopen. |
|CVLD-02|03–06/21/23/09–13/15: candidate-branch contextual versions/height/finality/locks/witness; no caller authority. |
|CVLD-03|02/03/05/21/23/09/10/15/16: exact flags reach interpreter/aggregate budget and actual durable historical bodies. |
|CVLD-04|01/03/04/06/21/23/09/11/12/15/16: shared branch difficulty/time, required anchors and missing-history refusal. |
|CVLD-06|02/03/07/08/22/24/11/16/19/25: exact repeat/ancestor/recheck, complete durable earlier-coin restoration and explicit pinned disconnect difference. |

All five IDs appear in current frontmatter and concrete task obligations. D-01–D-10 retain implementing actions and full coverage; no relevant PROJECT/roadmap requirement or locked decision is reduced. Actual daemon qualification168, wider differential recovery169, pruning170 and broader historical closure171 remain deferred as approved; the required finite historical evidence is retained here.

09–20 key links name real existing or declared owned provider paths and concrete call/import/corpus/report patterns.13 retains the actual process executable `CARGO_BIN_EXE_open-bitcoind` contract. Using GSD's dedicated `parseMustHavesBlock`, all40 key-link patterns across revised/future06–25 parse as plain unquoted regexes and compile; both endpoints are existing or declared owned paths. Actions/interfaces expressly require real wiring rather than comments/dummy text. These are planned edges; this review makes no future source-link pass claim. Frozen02/03 metadata is outside this bounded correction.

17 still requires independent exact-source review before any current ownership-row renewal;18 preserves frozen historical inverse/originals and offline registration.19 covers every new Rust breadcrumb, including21's sibling.25 requires root-applied excerpts before20 freezes its basis.20 retains canonical `167-REVIEW.md`, separate security/goal reports, complete default `bash scripts/verify.sh`, fresh pure-core coverage, Bazel, actual outcomes, originating lifecycle and root-only normal-hook commit/push after clean acceptance. No early guard renewal or nested saving is authorized.

**Actual static checks:**25/25 frontmatter validations and25/25 structure validations pass;44 tasks have nonempty files/action/verify/done/automated commands. Snapshot/log hashes, original task/metadata/truth/artifact preservation, DAG/waves/owner serialization, regex parsing and frontmatter delimiter/whitespace checks pass. Planner check reports were read and their principal counts/preservation claims independently reproduced.

**Dimension8:** SKIPPED — `workflow.nyquist_validation` is false. Existing research's concrete planning resolutions remain applicable; no unresolved new behavioral choice is introduced by this decomposition.

Material guidance:AGENTS.md Repo-Local Guidance, AGENTS.bright-builds.md, standards-overrides.md, standards/index.md, architecture/code-shape/testing/verification/local-guidance and relevant Rust/TypeScript rules. Both canonically distinct active lesson files were measured and read completely:6898+1958=8856bytes;2300+653=2953conservative tokens. The October19 audit baseline remains applicable; no audit trigger or project skill directory was found.

Only this block was appended, preserving every prior report byte. No source/plan/shared-state/Git/build/test/application/acquisition/cache/log operation or subagent was performed. One read-only comparison attempted an original review snapshot that is not part of `before/`; no file changed, and append preservation is checked directly against this report's complete pre-append bytes. Root may proceed through the revised execution release gates; all actual implementation and final acceptance obligations remain pending where unexecuted.

```yaml
issues: []
```


## review-plan23-high-s-prerequisite | 2026-10-21T20:12:42Z

### VERIFICATION PASSED

**Phase:**167 — Candidate-History Validation Rules. **Scope:**only the Plan23 high-S prerequisite amendment dated `2026-10-21T20:09:06Z`, current23 PLAN and targeted decision-coverage delta. **Current issues:**0 blockers,0 warnings,0 informational findings. **Lifecycle:**yolo; `167-2026-10-21T16-38-23`. Previously accepted06/21 and the structural revision are not reopened by this check.

### Independent preservation and executable scope

The pre-edit23 snapshot independently hashes to `d41c43faab0fb179531b73a23ba2e43a46729efa7458f2933e994fb564cf3797` (15416bytes). All9 snapshot-manifest entries match their preserved existence/bytes/SHA256, including absent-file markers and the local secp documentation snapshot. Both existing signature source files, pinned inspected source and local documentation remain byte-identical to their pre-edit snapshots; neither new test owner exists yet.

Current23 retains execute/wave6, dependencies06+21, autonomy, CVLD-01/02/03/04, generated_by, original generated_at, yolo lifecycle, commits:false and its original truth. There is still one task. Its original name/done are exact; original read-first, behavior, action and acceptance are exact prefixes, and all original automated commands remain exact content. No original owner is removed. The four additions are precisely:

| Added path | Bounded responsibility |
| --- | --- |
|consensus/src/signature.rs|Conditional local verification-signature normalization only after genuine signed RED and existing encoding/LOW_S checks. |
|consensus/src/signature/tests.rs|Register the cohesive child; preserve current tests/assertions and keep the560-line owner below629. |
|consensus/src/signature/tests/high_s.rs|Actual signed checker controls and auditable deterministic low/high-S vectors using existing test signing seams. |
|chainstate/src/engine/tests/phase167_policy.rs|Real canonical funded high-S success, same-input explicit policy rejection and invalid-signature/state controls. |

Paths above are under their existing open-bitcoin-consensus/open-bitcoin-chainstate packages. This makes nine owners, with the already-owned engine/tests.rs registering the policy sibling. No other phase plan declares an added path.21→23→07 remains serialized in waves5→6→7; no new edge, parallel ownership conflict or phase inventory change is introduced. Current totals remain25plans/44tasks/22waves. Existing five-owner stop prose is explicitly superseded only for these four paths, without general owner expansion.

### Source finding and required evidence

Read-only inspection confirms that current TransactionSignatureChecker::verify_ecdsa passes the verification parser's unnormalized signature to libsecp, while that parser only normalizes a copy to detect NonLowS when LOW_S is selected. The strict public parser separately rejects NonLowS unconditionally. Pinned pubkey.cpp277–292 normalizes ECDSA before cryptographic verification; interpreter.cpp encoding/LOW_S gates precede the checker. Existing local secp0.31.1 documentation states normalization is required for verification. Existing high-S parser cases establish parsing acceptance/rejection, not validity against an actual key/digest.

This supports the stated source suspicion. No actual signed high-S RED has been captured by this planner/checker, and no runtime failure, source correction or GREEN is asserted. The amendment correctly requires nonzero-discovery actual same-key/same-digest signed tests first, records genuine cryptographic refusal separately from setup/parser failures, and permits the narrow production correction only if that RED confirms need. If verification already passes, it retains the controls and prohibits speculative production edits.

The planned correction preserves strict public parsing, existing errors, original signature/DER/script bytes, sighash byte and signature-removal semantics. Flag-gated encoding/LOW_S checks complete before a local verification copy may normalize immediately before secp.verify_ecdsa. The original high-S value must still produce SignatureError::NonLowS/ScriptError::SigHighS under explicit policy. No input-byte rewrite, flag weakening, unconditional success, duplicated checker, new error/export/dependency or accepted21 dispatcher edit is authorized. Shared CHECKSIG/CHECKMULTISIG execution is explicitly covered, with valid low/high-S verification and wrong digest/transaction/signature/key refusals; existing Base/WitnessV0 and lax/strict DER controls remain mandatory.

Canonical integration requires actual matured accepted funding, exact funding outpoint/value/script/transaction and recomputed first-party sighash identity. Deterministic fixed low/high-S vectors are generated and independently verified through the existing consensus test signer, with exact bytes/digest/source evidence retained in the policy tests. Chainstate receives no secp import, dev-dependency or production signing API. High-S must succeed through selected canonical flags and the real coins stage; the same actual funded input/signature must fail under explicit LOW_S through public input verification with the existing typed policy error. Parser success, an unrelated earlier refusal or a disconnected vector cannot substitute. Accepted tip/hash/full work, best-block/complete relevant coins/undo and no absorption remain required on refusal.

### Retained obligations and final gates

The exact original action prefix retains all monetary/reward+actual-fee boundaries, maturity100 versus99-depth, zero genesis UTXOs, all seven selected flag controls, canonical CLTV/CSV positives and omissions, policy separation, aggregate80000/80001, consumed-only distant creating-MTP, missing/inconsistent saved history,4096bounds, same-block locks, replacement-branch disagreements and accepted-state/original-clock protections. Original focused discovery, strict affected Clippy, actual links and full consensus+chainstate --all-features regression without skips remain; the additional high-S command supplements them. Optional MINIMALDATA/CLEANSTACK relocation preserves exact inputs/assertions and names/evidence mapping rather than deleting controls.

Both new test files require pinned-source breadcrumbs;19 retains shared manifest ownership.17 independent review still precedes any current source-guard renewal.20 retains full native/fresh pure-core coverage/Bazel, canonical167-REVIEW.md, separate independent security/goal reports, originating lifecycle and root-only clean normal-hook saving/push. No frozen artifact, acquisition/cache/log change, accepted evidence reopening or deferred168–171 scope is added.02 remains pending according to root; this review neither inspects nor accepts its follow-up acquisition.

The coverage delta is only23's owner-count clarification plus the explicit high-S prerequisite section. D-04/D-07/D-08/D-09 and CVLD-03 retain full execution evidence; all original23 requirements/decisions and threat dispositions remain. Added T-167-23-3 addresses normalization order, invalid-crypto acceptance and evidence confusion. No locked decision is reduced and no new unresolved behavioral choice is introduced.

### Actual checks and limits

Independent frontmatter validation passes with no missing fields; structure validation passes with one complete task and zero errors/warnings. Snapshot/source preservation, exact-prefix/metadata/command comparisons, nine-owner accounting, current ownership-conflict checks and retained21→23→07 ordering pass. GSD's dedicated key-link parser reads all four23 links; their plain unquoted regexes compile. Changed planning artifacts retain only opening/closing frontmatter delimiters and have no trailing whitespace. Planner-checks.json was consulted and these principal claims independently reproduced.

**Dimension8:**SKIPPED — workflow.nyquist_validation is false. Previously loaded AGENTS.md/sidecar/overrides, architecture/code-shape/testing/verification/local-guidance and Rust guidance materially inform typed/pure boundaries, test-module scope, evidence attribution and preserved verification. Both active lesson inputs were already read completely within8856bytes/2953estimated tokens; no new lesson/audit trigger or project skill applies to this bounded follow-up.

Only this review block was appended; every prior report byte is preserved. No plan/source/shared-state/build/test/Git/acquisition/cache/log changes or subagents were performed. Source inspection and metadata/hash comparisons establish plan sufficiency only. Root may release23 within this exact scope; genuine signed RED, any justified correction and all final actual execution evidence remain executor obligations.

```yaml
issues: []
```


## review-plan02-offline-key-link | 2026-10-21T20:26:36Z

### VERIFICATION PASSED

**Scope:**only Plan02's executable key-link metadata amendment. **Current issues:**0 blockers,0 warnings,0 informational findings. **Lifecycle:**yolo; `167-2026-10-21T16-38-23`.

The complete pre-edit snapshot independently hashes to `85ab9cba41dbb678279ca5f52af12da9b19576a6c2bcdf62617e09e326181f73`. Removing exactly the single added line `pattern: checkContexts\(context, corpus\.blocks\)` from the current plan reproduces every original byte. Both task/action blocks, six owners, requirements, truths/artifacts, wave5/empty dependencies, generated_at, schedule metadata and originating lifecycle are therefore unchanged. No source change, acceptance reduction or additional owner is prescribed.

Read-only inspection confirms an executed offline edge, rather than a comment or unused pattern:DATA is `packages/testdata/phase167`; `--check` dispatches to check(), which loads the composed contexts.json path, validates its hash against the source-bound manifest and calls `checkContexts(context, corpus.blocks)` at line207. The callee performs actual context validation. The complete literal target path need not appear in the producer for this real connection to exist.

Independent `frontmatter validate --schema plan` passes. `verify plan-structure` passes with both complete tasks; its sole generic wave>1/empty-dependency advisory is the previously accepted frozen independent-producer scheduling exception, not an unresolved finding. `verify key-links` now verifies1/1 with all_verified:true. The plain unquoted regex names the actual validation call and requires no fake source text or pattern-only implementation.

Root reports completed55/55 focused tests, passed offline corpus checks, all six source/data files below100MiB and independently seeded evidence classification. Those executed results remain attributed to root and the completed producer summary; this metadata check does not rerun or independently accept their behavioral content, promote seeded evidence to ordinary/daemon qualification, or grant whole-phase completion. The earlier complete02 acceptance and downstream08/15/16 barriers remain binding; root may complete Plan02 acceptance through its remaining orchestration checks.

Previously loaded AGENTS/sidecar/overrides and verification/evidence guidance inform this narrow review. No accepted source or unrelated active23 work was reopened. Only this block was appended with every prior report byte preserved; no plans/source/data/shared-state/build/test/Git/acquisition/cache/log changes or subagents were performed. Actual checks were snapshot hashing/exact one-line comparison, targeted source reading and the three metadata/structure/key-link commands above.

```yaml
issues: []
```


## review-plan26-fixture-projection | 2026-10-21T20:51:41Z

### VERIFICATION PASSED

**Scope:**new Plan26 fixture projection, bounded07/18 deltas, current coverage and amendment; final26 includes the independent-checker key-link correction. **Current issues:**0 blockers,0 warnings,0 informational findings. **Inventory:**26plans/45tasks/22waves. **Lifecycle:**yolo; `167-2026-10-21T16-38-23`. This is a static plan gate, not executed projection or Rust stage acceptance.

### Independent finding and correction

The first26 draft declared a third must-have key link from future07's absent `phase167_bip30.rs` consumer to the projection. Requiring that downstream source link at26completion would invert acceptance:07 owns that source and cannot start until26 is accepted. The implementation DAG alone did not expose this completion dependency.07 already declared the same real consumer edge.

Root returned the precise metadata issue to the planner. The pre-correction26 snapshot independently hashes to `fff6563f318411a0cde31e9db081d29bdbe4c4bab1f11e32e4a5a4c90c835f9e`. Exact byte comparison proves final26 is that snapshot with only the four-line duplicate future-consumer edge removed.07 retains its consumer edge;26 retains both owned-producer links, complete textual consumer handoff, schema, tasks/actions, ownership, dependencies, timestamps/lifecycle and truths. No behavior or evidence was reduced. The original finding and correction are recorded in the amendment rather than represented as a first-pass success.

### Preservation, scope and dependencies

All18 original snapshot-manifest entries independently match preserved existence, byte counts and SHA256. Snapshotted accepted02source/tests/data, checked-history/seeded-stage interfaces and accepted execution-scope/schedule history also match their current bytes. The three new26targets and07's new BIP30 test file remain absent. Original07 SHA256 is `93452d8cb0541c977183b38ae89868124230f2d9d8e672112a6206bbd85e2d9a`; original18 SHA256 is `fb53dff1d38c4f4846471516a08cc49b868eca196e7f5db1341ca04c8b17ac9b`.

07/18 retain original waves, owner sets, requirements, truths/artifacts, lifecycle and generated_at. Both Task1blocks are byte-identical.07 Task2's original action is an exact prefix, with original behavior/acceptance/verify/done preserved; appended instructions explain only the data prerequisite and existing test-owned adapter.18 Task2 normalizes exactly to its original after removing the two inserted commands and appended registration/inverse paragraph. Its six original commands remain in their original relative order. No original task obligation is deleted or deferred.

26 is one coherent task with exactly three owners:standard-library Bun projector, meaningful tests and documented tagged-data fixture. It has nonempty files/read-first/behavior/action/acceptance/automated verify/done plus threat model and bounded output. No source owner is added to07's nine or18's seven. New26generation20:37:04Z and originating lifecycle are explicit; all other plan counts remain unchanged.

26 depends on complete root-accepted02 in wave6 alongside disjoint23.07 retains06+23 and adds26 in wave7.18 retains direct17dependency/wave19 and follows26 transitively. Independent full metadata traversal found no missing/non-earlier predecessor, cycle, same-wave owner overlap or unordered shared owner; totals are26/45/22. Current coverage expressly supersedes the older08-first-consumer/end-of-wave7 release wording:26 is now the earliest acquired-corpus consumer, requiring both02tasks/checks and the complete accepted summary before starting.02 remains frozen; partial acquisition, old wave prose or elapsed estimates cannot release26.

### Genuine data availability and authority boundaries

Read-only accepted JSON inspection independently confirms both correct byte-bound pairs and lengths:

| Repeat | Correct original | Repeat/original bytes | Actual155-row suffix |
| --- | --- | --- | --- |
|91842|91812|214/616|91687..91841|
|91880|91722|472/214|91725..91879|

The corpus contains both required ten-header raw prefixes91677..91686 and91715..91724. Independent fixed-header SHA256d and previous-hash checks confirm the prefix identities and links; both155-row suffixes have complete contiguous parent linkage and the required heights. Original-parent MTP windows each contain11real rows. The real detached227931header has the selected `000000000000024b89b42a942fe0d9fea3bb44ab7bd1b19115dd6a759c0808b8` identity. No acquisition is required to obtain these facts.

91880's envelope contains the actual external input `cae24697a15419b1de45c99a41149101c252c2dbd3fd0bbce06ac5809d4547e3:1`,4480000000sat,P2PKH script `76a914b5635b9c16b17bf34f29fdc999b440e0638c5b9088ac`,noncoinbase,height90644,creatingMTP1289277562.91842 has no external inputs. Plans require complete repeat transactions and every needed external UTXO, complete earlier Coins from paired original coinbases, and exact original/repeat body/header/txid/wtxid/merkle identities. Coinbase-only or reversed-pair evidence cannot satisfy the task.

Raw MTP prefixes stay outside the155stored ChainPositions and checked CandidateHistory.07 computes true eleven-header medians for each stored row using those preceding raw headers; it cannot recursively invent prefix metadata or claim global accumulated work. CandidateHistory receives one parent+154ancestors in parent-first order, while stored fixture positions use ascending height. The independently seeded227931anchor enables explicitly synthetic high-height structural/kernel probes only; missing required anchors refuse as unknown context. Synthetic probes remain non-PoW/noncanonical, and full exact header/body seeded-stage evidence belongs to the two real repeat cases.

Read-only interface checks confirm check_seeded marks IndependentlySeeded and the existing stage requires MechanicalFixture origin and exact stored branch rows before the shared application kernel. Data does not create a private RuleFacts/authority constructor, HeaderStore membership/original-clock seal, caller flags or product origin. Existing selected-rule, script, coins, undo and unchanged-state checks still execute in07. Durable historical checkpoint/reopen evidence remains08/16; no projection result replaces it or claims actual daemon qualification.

### Schema, tests and offline verification

The contract is explicit versioned ASCII/LF tab-tag data, with closed record types/order/arity/counts/indexes, canonical checked decimal fields and lowercase fixed/even hex. Display hashes reverse once into internal wrappers; scripts/witness never reverse. Full typed header/transaction/input/output/witness/Coin fields are projected, including detached anchor,case provenance,source digests,history/raw-prefix/original-MTP rows and END markers. No Rust code,expressions,JSON,escaping/evaluation or copied generated Rust constants hide in the .txt.

The projector calls accepted02check and reuses accepted decodeHeader/decodeBlock/pair helpers. It validates fixed source/data hashes and all pinned facts already bound by02; no duplicate raw Bitcoin/CompactSize parser, new dependency, public production helper or ad-hoc Rust JSON parser is needed.07's already-owned test file performs only the bounded tagged-data adapter and ordinary typed construction, with production hash/merkle helpers checking identities.

Output is bounded to1MiB/1024records/16KiBline/32transactions per body; original128MiB/4096 input limits and native deadlines remain. Pure projection/validation/render code is separated from fixed-path effects. Explicit local --write follows complete validation and changes only the new fixture; failure preserves existing output. Native --check reconstructs expected bytes offline, refuses missing/stale/corrupt output and never writes,repairs,fetches or acquires. Frozen acquisition observation supplies only fixture-clock data, never current randomness or candidate.time authority.

Planned tests use real accepted corpus and independent known pairs/fields, reconstitute80-byte headers through accepted decoding, compare every transaction/input/output/witness field against accepted decoding without another transaction encoder, and check complete coins/windows/anchor. Meaningful reversed/swapped/missing/duplicate/truncated/out-of-range/source-hash/version/network/record mutations, deterministic repeat writes,preserved output on failure and throwing-fetch offline controls are explicit. These are data-adaptation tests;07 still must separately prove actual absorption/disconnect/reconnect and atomic refusals.

18 now requires exactly eight offline entries in both VERIFY_COMMAND_ORDER and actual run_step:the unchanged historical pair, the new projector --check/test pair, then the unchanged regtest and Phase167 checker pairs. It preserves the original six relative positions and frozen verify.txt/VERIFY_BASE_SHA by extending only reviewed current-additions/inverse handling. Missing/duplicate/reorder/check-to-write/acquisition/network mutations are required; --write/--acquire/network cannot enter native verification.17 still independently reviews source before any ownership-row renewal; frozen historical originals/legacy bridge,budgets and deadlines remain unchanged.

D-06/D-08/D-09/D-10 have full new projection/consumer/registration coverage, with unchanged CVLD-06 and all other phase decisions/requirements. New threats cover source tampering, bounded tagged parsing, fixture authority promotion and write/check separation. No scope reduction or deferred168–171work is introduced.20 retains full native/fresh pure-core coverage/Bazel,canonical source/security/goal reports and originating lifecycle/root-only normal-hook saving after clean acceptance.

### Actual checks and limits

Independent07/18/26 frontmatter and structure validations pass with complete2/2/1tasks and no errors/warnings. Snapshot/preservation, task normalization, DAG/owner accounting and targeted JSON/header availability checks pass. After the exact correction,26frontmatter/structure pass again;07/18/26 each have two plain unquoted regex links, all parsed by GSD's dedicated parser and compilable.26's two links belong to its producer;07 owns the later consumer. No future source key-link pass is claimed while projector/fixture/consumer are absent. Planner-check reports were read and principal preservation/count claims independently reproduced.

**Dimension8:**SKIPPED — workflow.nyquist_validation is false. Existing accepted research suffices for this bounded data adaptation. Previously loaded AGENTS/sidecar/overrides, architecture/code-shape/testing/verification/local-guidance and Rust/TypeScript rules informed scope,evidence,typed boundaries and verifier preservation. Both active lesson inputs were fully loaded within8856bytes/2953estimated tokens; no new project skill or audit trigger applies.

Root's accepted02/23 execution outcomes remain attributed to root and their summaries. This check did not rerun55corpus tests or798regressions, execute a projector, generate data or independently accept23source behavior. Only this append-only review was written, preserving every earlier report byte. No plan/source/shared-state/tests/builds/Git/data/acquisition/cache/log mutation or subagent occurred. Root may release26, then07 only after complete26acceptance, within the retained obligations above.

```yaml
issues: []
```


## review-plan27-undo-budget | 2026-10-21T22:35:45Z

### VERIFICATION PASSED

**Scope:** only new Plan27, its complete-undo allocation amendment and Plan10's bounded prerequisite/context delta. **Current issues:** 0 blockers, 0 warnings, 0 informational findings. **Inventory:** 27 plans, 46 tasks, 22 waves. **Lifecycle:** yolo; `167-2026-10-21T16-38-23`. This is a static planning check; source execution remains subject to root's complete acceptance of07/08 and this amendment.

### Independent source and feasibility review

The actual accounting parent matches the retained snapshot exactly: 522 lines, 19810 bytes, SHA256 `9cf19c472ce8b25d0c3d0c33dc7e2465d689fe813333baeb50e558dced088c48`. Current fact_work and borrowed_undo_work charge the BlockUndo header and ordinary transaction/restored-Coin allocations but do not traverse overwrites. Both actual capture paths call fallible fact_work before clone_basic_body and the full undo clone. This independently confirms the source omission and the relevant ordering; it is not an observed failing behavioral test or an executed allocation-limit test.

Actual07 types support the planned exact formula: one BlockUndo header; each TxUndo and restored Coin plus its script; each BlockOverwriteUndo; each OverwrittenCoin plus both previous/new script lengths. The outer and nested Vec headers and inline Coins/outputs are already represented by their containing types. Counting logical vector lengths avoids double counting; equal script contents still occupy two independently cloned sequences. The shared private add_undo_work can reuse the existing checked add_bytes/count_script boundary and existing error conversion without another owner or public interface. Owned cloned_bytes continues to include body, complete undo and ChainPosition once; borrowed cloned_bytes remains zero.

Read-only caller tracing reaches ordinary prepare_basic_filter_facts, filter_reorg capture_reorg, reorg preflight and runtime catch-up borrowed accounting. BasicFilterInputs selects outputs and ordinary restored_inputs only; its actual from_historical checks body/header/merkle and transaction/input correspondence, not overwrite authenticity. Small consistent structural fixtures can therefore exercise capture_reorg and preserve nonempty, captured-empty and unknown undo representations without inventing canonical authority. The plan expressly preserves script_items/script_bytes, BASIC inputs/content, witness elision, consensus/BIP30 restoration and integrity/provenance semantics. Overwrite allocation bytes reach only undo_bytes and the owned clone sum.

Boundary verification is executable within the two owners: initialize the actual private shared accumulator near the unchanged 384MiB ceiling or at u64 overflow, then traverse small real undo values. Independent size_of/known-length expectations, multiple entries and both scripts catch the actual omission. Exact-limit, one-over and newly added overwrite contribution controls need neither large allocations nor budget overrides. Existing three phase157 byte/item/overflow controls remain exact and run under the local module filter. Source-order review complements these executable controls; the plan does not misrepresent them as an instrumented oversized capture test. Actual nonzero behavioral RED must precede the correction, then final GREEN; compilation/setup errors and zero discovery cannot substitute.

### Preservation, scheduling and coverage

Independent snapshot hashing verifies all26 original plan snapshots against before-manifest.json. All45 original task XML blocks and automated commands remain byte-identical. All25 old plans other than10 retain complete original bytes. Removing exactly10's added167-27 dependency and one context paragraph reproduces its entire before snapshot, retaining both original tasks, wave11, owners, requirements, truths and original generation/lifecycle metadata. The accounting source is unchanged. Planner validation receipts were consulted and these principal claims independently reproduced.

New27 is one complete cohesive task with two source owners: the existing parent and registered test-only child. It has concrete files, action, behavioral/acceptance criteria, automated verification and measurable done conditions. Its two plain regex key links compile and name the actual BlockUndo provider and owned child registration. Both implementing sources are27-owned; the child is a planned artifact, not an already verified executed link or a future consumer required before its producer can complete. There is no completion-dependency inversion.

27 depends on07/08 in wave10 and is disjoint from09's source ownership.10 waits for08/09/27 in wave11. Independent all-plan dependency traversal found no missing or non-earlier prerequisite, cycle, same-wave owner overlap or unordered shared owner. All other plan bytes/waves remain. Same-wave source independence does not permit concurrent Cargo target work. Complete root-accepted08 includes both original focused commands and its unfiltered node all-features regression; partial progress cannot release27. Root's reported final08 outcomes remain attributable to08 and are not independently rerun or accepted by this check.

CVLD-06/CVLD-03 frontmatter correctly labels this bounded supporting resource contribution. Original full coverage of CVLD-01/02/03/04/06 and all ten decisions is preserved by exact task retention;27 explicitly supports D-06/D-08/D-09/D-10. No locked decision is reduced or deferred idea implemented. No whole requirement or phase closure follows. Accepted research resolves the relevant complete-undo and evidence boundaries; this internal accounting correction adds no unresolved research/API question. **Dimension8: SKIPPED** because workflow.nyquist_validation is false.

17 still requires independent source review before current ownership renewal;19 retains new-child breadcrumb manifest registration.18 retains all eight exact offline entries, current inverse and frozen-original contracts without a new native command. Existing normal Cargo discovery covers the child.20 still owns full default native verification, fresh pure-core coverage, Bazel, independent source/security/goal reports and originating lifecycle acceptance before root-only normal-hook saving. No source guard, frozen evidence, threshold, timeout, skip or original control is weakened;168 and later ownership remain deferred.

### Actual checks, guidance and limits

All27 independent GSD frontmatter/structure validations pass; the sole generic structure advisory is the previously accepted frozen02 wave>1/empty-dependency producer metadata, not a new issue. Independent artifact receipt records exact task/command/file preservation, DAG/ownership, lifecycle metadata, two top-only frontmatter delimiters in each changed/new artifact, source hash and both regexes. The dedicated lifecycle command also reports valid:true, reasons:[],27 valid plans for the required yolo id. Ignored receipts live under `.local/open-bitcoin-dev/phase167/amend27/check/`.

An initial checker-side report attempt treated the generic GSD getter's lossy key_links strings as dictionaries and failed after completing validations/comparisons. Corrected raw plan regex inspection and the subsequent artifact run passed. A Python metadata probe used the system interpreter outside mdformat's pipx environment and could not locate its package; the actual mdformat command reports1.0.0 with gfm1.0.0/frontmatter2.1.2. These are checker setup observations, not product REDs or plan failures. New review text alone is formatted through those available extensions before append; all historical review bytes remain intact.

AGENTS.md local guidance, AGENTS.bright-builds.md, standards-overrides.md, managed index and architecture/code-shape/testing/verification/local-guidance/Rust standards materially informed private checked accounting, module size, synthetic evidence classification, serial pinned-Bun timing-lock verification and preserved final gates. Both canonical active lesson inputs were read completely:6898+1958=8856 bytes,2953 conservative estimated tokens; no archives loaded, new lesson or audit trigger. No project skills were found. No new dependency requires external API research.

Only this new review section and ignored checker receipts were written. No source edits, Cargo/Bazel/tests/application execution, staging/commits/push, shared state/count edits, source-guard renewal, corpus or historical receipt mutation occurred. All implementation and actual RED/GREEN evidence remain pending executor obligations. Root may accept this bounded plan and release27 only after its separate complete08 acceptance.

```yaml
issues: []
```


## Plan09 managed bridge and body eligibility amendment review | 2026-10-21T22:57:44.396Z

**VERIFICATION PASSED — bounded planning amendment.** No new blocker, warning or scope reduction. This independently checks Plan09's exact eight-owner executable design and Plan10's explanatory handoff, preserving earlier accepted checks. It grants no implementation or phase acceptance. Root alone accepts and releases the fresh09 executor after accepted27;10 still requires both09 and27. The existing frozen02 empty-dependency/wave advisory remains previously accepted historical metadata.

### Exact scope and preservation

The reviewed amendment is20512bytes, SHA256`cf6349ad9a92d7c6a315f113c86a1694ffcda0e5eb5f7cee5b958b608c60bc6e`. Independent receipts are in `.local/open-bitcoin-dev/phase167/amend09bridge/check/independent-validation.json`, with the actual rerunnable checker script alongside it. All27 GSD frontmatter and XML structure validations pass:46 complete tasks,22 waves. Lifecycle validation reports valid:true, reasons:[], originating yolo id167-2026-10-21T16-38-23,27 valid plans and14 valid summaries. Missing final verification remains a future20 obligation, not present phase acceptance.

All25 plans other than09/10 are whole-byte identical to before-manifest snapshots.09 Task1 is whole-byte exact. Task2 retains exact action, behavior, acceptance, verification and done bytes; only its three bounded files/read-first additions differ. Full09 reverses to its original bytes by removing those additions and the authorized owner/metadata/context/interface additions. Full10 reverses by removing its single new context paragraph; both original10 tasks and commands are exact. All46 original automated commands and original task counts are retained. Requirements, must-haves, generated_at, lifecycle, waves and dependencies remain unchanged. All eight09 source owners retain their snapshot bytes/existence, including the still-absent phase167 test child.

The two added provider snapshots independently match node chainstate/validation_history.rs11230bytes/SHA256`de507925613e44fdea9cb5b3e8766c8aebc5070302a70db7fd59fd6c1dcbee62` and network header_store/eligibility.rs17044bytes/SHA256`751b989eec04f97e280a0eccc2dc234a6fa413bd0804e5a3d1e802b0f120cc9c`.27's separately accepted disjoint accounting changes are outside that source-preservation claim. No global-checkout source freeze is asserted.

### Source-grounded executable path

1. Current guard_body callers in mempool_lifecycle.rs, candidate_eligibility.rs and action_translation.rs, and guard_message_admission callers in network.rs/action_translation.rs, have mutable network receivers. Changing only the private guard receivers in owned canonical_admission.rs can reach the owned adapter without an outside caller edit.
1. Actual HeaderStore canonical_header_history and canonical_header_validation_context return owned data. The latter retains the original shell admission clock. The adapter can check exact candidate/header/parent/selected-network facts, end the immutable store borrow, then borrow managed coins mutably. The new crate-private provider delegates to existing Chainstate::admit_canonical_header; core independently derives its own earned branch/rules and current owner/generation. No network authority enters pure chainstate; no public mutable accessor, caller params/flags proof or parallel authority owner is planned. Repeated exact pending admission is idempotent. Any genuinely needed reorg delegation must preserve the existing batch's full order and held clocks.
1. Existing retain_checked_candidate reaches PeerManager body checking and the added eligibility owner before authorize_connect. Its unchanged legacy context/insertion currently refuses canonical descendants; therefore fixing only coins admission cannot complete the positive. The planned descendant branch requires this actual store's canonical exact header, checks immutable params, runs existing check_block_with_candidate_history with the held original clock, and only then reuses existing private record_with_order/parent linkage. Fixture API/error/mismatch behavior remains. The shared helper checks exact header identity and contextual body rules. Body eligibility remains distinct from scripts/coins acceptance.
1. Existing registry linkage releases a child only after its parent's linkage. Pure from_canonical_genesis intentionally installs only the proved header root and no body binding. The plan preserves that constructor and its test, while the already-owned node constructor validates/registers the actual proof.block() through existing eligibility before publishing its header owner. A distinct canonical genesis case checks selected exact full genesis bytes and height-zero facts before existing root recording; no invented CandidateHistory, fake parent, caller-time proof, new seal, genesis UTXO or ScriptsValid promotion is allowed. Wrong-proof/scope/default-off controls remain.
1. Normal authorize_connect → prepare_authorized_connect → actual canonical stage → genuine managed absorption remains the only accepted-body route. Current validation_history absorption seals accepted receipts afterward. The genuine local test setup can use existing Fjall proved-genesis and managed constructors, then a correctly mined small regtest child with real merkle/BIP34/subsidy/parent facts and no genesis spend. Existing module privacy permits tests in09's child. No compile or behavioral execution was run by this checker.

### Coverage, scheduling and verification dimensions

| Dimension                | Bounded result                                                                                                                                                                                                                                                                                                            |
| ------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Requirements/goal        | All five CVLD-01/02/03/04/06 remain present across plan frontmatter and retain original behavioral owners.09 retains02/03/04; the new bridge makes its already-required managed positive feasible. PROJECT and roadmap supported-scope/evidence constraints remain.                                                       |
| Task completeness        | Both09 tasks retain concrete files, actions, behavioral RED/GREEN, automated command and measurable done criteria.10's full direct/stored/local/reorg, historical adapter, persistence/publication and BASIC tasks remain.                                                                                                |
| Dependencies             | Independent DAG walk finds no missing/cyclic/later predecessor, same-wave source overlap or unordered shared owner.09 wave10 precedes10 wave11 through its existing dependency; shared chainstate provider is ordered.27 is disjoint from09 and also precedes10. Root's current serial order is accepted27 → fresh09 →10. |
| Key links/data contracts | Exact header/history/original clock cross the adapter as owned values; core rechecks its own lineage. Canonical eligibility consumes the same held context and checks body bytes before existing linkage. No lossy transformation or transfer of accepted authority.                                                      |
| Scope/must-haves         | 09 remains two tasks/eight owners, within the accepted bound. Observable ingress and refusal truths remain;10 has no task/owner expansion. One core admission owner and one existing body registry keep the design cohesive.                                                                                              |
| Context/scope reduction  | D-01–D-10 retain full original mappings and the amendment's explicit contribution matrix. No fixture/header receipt becomes accepted coins, no blanket support switch, deferred168–171 work or reduced decision.                                                                                                          |
| Nyquist                  | Dimension8: SKIPPED because workflow.nyquist_validation is false; no other verification waiver.                                                                                                                                                                                                                           |
| AGENTS/standards         | Typed fallible private seam, pure core/shell, scoped ownership, behavioral AAA tests, no dependency/crate, serial timing lock and native final gates retained.                                                                                                                                                            |
| Research                 | Existing accepted research resolutions remain applicable; the internal ownership correction introduces no unresolved external API or research question. Earlier accepted research/check history is preserved.                                                                                                             |

Actual future RED must expose each unchanged production barrier before its correction. Progressive RED is explicitly permitted when an earlier refusal masks the later one, with honest chronology; compiler/setup failure, zero discovery, core-only positives or counterfactuals cannot substitute. GREEN must exercise Headers/Block/CompactBlock/BlockTxn, stored completion and received-action managed positives, every original refusal, and no socket/default activation. Partial bodies/header-only facts cannot mint receipts. Refusal compares accepted tip/hash/exact work, relevant complete coins/undo, coins best block, ScriptsValid/provenance and BASIC endpoints. Valid headers may remain known; incompatible-source startup retains stricter original-manifest equality.

08's accepted2077passes/four inherited ignores, post-implementation counterfactual and restored-hash chronology remain historical and are not pre-TDD evidence for09.27's accepted5/5 pre-fix RED,11 focused/14 module GREEN and strict checks remain attributed to27; none is re-executed or promoted by this review.17 requires independent source review before current guard renewal,19 registers new breadcrumbs,20 retains full default native/fresh pure-core coverage/Bazel and separate canonical REVIEW/SECURITY/VERIFICATION plus lifecycle validation. Root alone saves with normal hooks after final acceptance; no nested commits or work beyond167.

### Guidance, checker outcomes and limits

AGENTS.md local/GSD guidance, AGENTS.bright-builds.md, standards-overrides.md, managed index and architecture/code-shape/testing/verification/local-guidance/Rust/TS rules materially informed this check. Both canonically distinct active lessons were fully read:6898+1958=8856bytes,2300+653=2953 conservative tokens; no archives. The Oct19 audit baseline remains applicable with no new trigger. No project skill directory was present.

The initial read-only gsd-tools --help probe returned unsupported-flag exit1; actual documented frontmatter/structure/lifecycle commands subsequently passed. Some large read outputs exceeded display budgets; critical contracts were reloaded in bounded sections. These are checker setup observations, not product RED or plan defects. No application/Cargo/Bazel/tests/Clippy/rustfmt, source edit, source-guard renewal, harness/corpus/frozen receipt mutation, staging/commit/push or shared STATE/ROADMAP/REQUIREMENTS/config/todo edit occurred. Only this appended report and new ignored checker receipts are owned by this pass.

```yaml
issues: []
```


## Plan09 test-helper split amendment review | 2026-10-21T23:19:19.708Z

**VERIFICATION PASSED — one test-only owner addition.** No new blocker, warning or scope reduction. This check covers the current frozen implemented basis and the proposed helper relocation; it does not reaccept the implementation or supersede the earlier eight-owner bridge check's historical source basis. Full09 remains unaccepted. Root alone accepts/releases the source move. Current workflow remains14/27 accepted phase plans and84/97 milestone plans.

### Independently reproduced artifact and source facts

The reviewed09 plan is23774bytes/SHA256`020e12649b056813f669cf1ed27bd696783cdd19b37024e5afaadbb2d183e21c`; the new test-split amendment is16313bytes/SHA256`195cf85c2cc30db0b92087da9e46c2880f0854aff7d066b7b969ed56f58bc6f3`. Independent receipts and the rerunnable checker are under `.local/open-bitcoin-dev/phase167/amend09split/check/`.

All27 independent GSD frontmatter and task-structure validations pass:46 tasks/22 waves.09 Task1 is whole-byte exact. Task2 differs only by its one child files/read-first addition; action, behavior, acceptance, commands and done remain exact. Removing only the authorized ninth owner, amendment timestamp/reference/context paragraph and Task2 list additions reconstructs the complete original09 snapshot. All26 other whole plans, including10, are byte-exact. Every original task/automated command, requirement, must-have, lifecycle, wave and dependency is retained. Both changed planning artifacts have exactly their two top frontmatter delimiters and no trailing whitespace. The managed-bridge amendment remains its exact20512bytes/SHA256`cf6349ad9a92d7c6a315f113c86a1694ffcda0e5eb5f7cee5b958b608c60bc6e` historical record.

The current eight implemented sources match this new amendment's before-manifest and their source snapshot bytes; the proposed fixtures.rs child remains absent. These are current implemented hashes, not the prior bridge review's preimplementation hashes. phase167.rs also equals the worker split-basis file. Independent measurement confirms:

| Frozen region      | Lines | Bytes | SHA256                                                             |
| ------------------ | ----- | ----- | ------------------------------------------------------------------ |
| Entire phase167.rs | 832   | 30554 | `344417890bb9d1eb2560d301c1b4e297a3f9c7a4c0bc3354501bc5e9ffb0c374` |
| Helpers17–240      | 224   | 7744  | `af5d85fbd66aa88bd4db05c22c5f2a8fbbd6444b83e730d3542367aefb516658` |
| All12 tests242–832 | 591   | 22269 | `8108c04766903704010dd9758c02bca7408acd1a013bfcb32aed2cb18e65d77c` |

### Source split feasibility and exact inverse gate

The contiguous helper block is a coherent test module: genuine Fjall proved-genesis owner lifecycle/cleanup, bounded actual regtest mining, actual header admission, typed complete accepted chain/coins/undo/work/BASIC/provenance capture and assertions, and socket-free compact transport. The parent suffix uses these helpers without inspecting AcceptedState fields. Keeping those fields private while making only necessary helper functions/types visible to the immediate parent is feasible. Existing parent imports can be inherited by the child with use super::\*; no production export, mutable accessor, proof seam, dependency or authority class is needed. The child remains below existing cfg(test) ancestry. The unchanged test suffix retains relative action_translation references at the same parent depth, avoiding their accidental rebinding.

An independent in-memory literal relocation, with explicit parent mod/use declarations, child breadcrumb/import preamble and enumerated pub(super) additions, reverses exactly to all30554 original parent bytes and7744 helper bytes. Its untouched suffix equals all22269 original bytes and contains the same12 names/paths. Both strengthened controls are explicitly present: compact mismatch has no block disposition and requests GetData, and P2SH rejection matches the typed TransactionValidation error and exact mandatory-script-verify-flag-failed reason. No statement, assertion, literal, bound, cleanup or comment was dropped by this feasibility simulation.

That simulation gives610 parent and231 child lines before actual rustfmt, consistent with the amendment's rough611/~240 projections. It is not an actual source move, formatted size measurement, compilation result or completed executor inverse. The strict executor gate remains: immediately after only the helper move, retain exact before/postmove sources and a deterministic inverse, before the separate foreign-genesis diagnostic assertion/production ordering fix. Prefer exact restoration; if formatter layout differs, reverse only recorded exact adaptations, compare full reconstructed/original formatter output with identical edition2024/skip_children options, and retain the complete unformatted diff. A lossy tokenizer or arbitrary equivalent-code claim is insufficient. Both actual formatted files must remain strictly below629 lines without compression, dropped assertions or overflow into the614-line outer registrar; further scope needs root review.

The later root-authorized diagnostic assertion must produce actual behavioral RED before its production ordering correction and retain separate source/result attribution. It is outside this helper move's inverse. Final augmented tests must not be claimed byte-identical to the832-line basis merely because the intermediate move was reversible.

### Goal, decisions, schedule and verification

All five phase requirements and D-01–D-10 retain full original owners and obligations.09 remains two tasks with nine union owners; the only new path is test-only network/canonical_admission/tests/phase167/fixtures.rs. No production provider allocation or original test path changes. The implementation claims in the amendment are explicitly pre-split reports, not outcomes independently executed by this checker. The actual stored/six peer-route positives,100-block maturation and selected P2SH control remain in the frozen parent test suffix and complete helper semantics. No supported-network, evidence-class or deferred168–171 scope changes.

Independent DAG/owner traversal finds no missing/cyclic/later predecessor, same-wave overlap or unordered shared owner.09 remains wave10 after06/07/08;10 remains wave11 after08/09/27. Accepted27 and pending09 both precede10. Lifecycle validation reports valid:true, reasons:[], yolo id167-2026-10-21T16-38-23,27 valid plans and14 valid summaries. The only structure advisory is the already accepted frozen02 wave>1/empty-dependency metadata. **Dimension8: SKIPPED** because nyquist_validation is false; no other waiver.

Postmove execution must rerun both original focused commands with all12 named tests, existing relevant canonical-admission/header-store module regressions, affected strict node/network all-target/all-feature Clippy, scoped formatting/diff and final source closure. Earlier12/module28/header-store84/Clippy results stay pre-split evidence, with the stronger assertions' later attribution retained. This reversible relocation requires no new behavior tests of its own; it does require actual new module wiring verification. Further relevant checks remain necessary if a new failure or unresolved concern appears.

17 retains independent source review before current ownership renewal;18 retains its offline registrations;19 retains manifest registration for the child's defensible pinned breadcrumb comments.20 retains full default native/fresh coverage/Bazel, independent canonical source/security/goal reports, lifecycle and root-only normal-hook saving. No threshold, deadline, frozen receipt, source guard, harness, accepted requirement or broader verification contract is weakened. Existing accepted research adds no unresolved question for this internal module relocation.

AGENTS.md local/GSD ownership, Bright Builds sidecar/overrides, managed architecture/code-shape/testing/verification/local-guidance/Rust rules materially inform the cohesive child, limited visibility, full-source inverse and unchanged final gates. Both active lessons were already fully loaded within8856bytes/2953 conservative tokens; the recent Oct19 audit baseline still applies, no archives or new audit trigger. No project skills or external dependency research is needed.

Only this new appended report and ignored checker receipts were written. No source move/edit, Cargo/Bazel/tests/application/Clippy/rustfmt execution, guard/hash renewal, threshold/harness/corpus change, shared state/plan edit, staging/commit/push or historical evidence mutation occurred. All source, format, behavior and exact postmove inverse obligations remain executor gates after root release.

```yaml
issues: []
```

## Plan10 connect-caller amendment — independent bounded check | 2026-10-21T23:37:46Z

**BOUNDED PASS WITH SCOPE WARNING.** Zero blockers; one retained warning. Root must explicitly decide whether to accept this exact eleven-owner exception before releasing10. This report grants no source execution or phase acceptance. Historical report bytes above this append remain unchanged.

The checked change is167-10-CONNECT-CALLERS-AMENDMENT.md and its bounded167-10-PLAN.md delta. It adds only chainstate.rs and network/mempool_lifecycle.rs to original Task1. All five original CVLD requirements, both original tasks and full behavior/acceptance remain. There is no new plan, task, wave, dependency, production dependency, proof class or public mutation accessor. Current inventory remains27plans/46tasks/22waves;15 accepted summaries and85/97 milestone plans are root-owned progress facts, not acceptance created by this review.

### Independent artifact evidence

The independently written ignored checker ran with pinned Bun1.3.9 and invoked all27 GSD frontmatter validators, all27 task-structure validators and the originating lifecycle validator. Every validation passes, errors:[]. Lifecycle remains yolo167-2026-10-21T16-38-23. All original automated commands are exact. Task1 behavior/action/acceptance/verify/done are byte-exact; its files differ only by the two additions and its read-first insertion retains the original prefix/suffix. Whole Task2 XML is byte-exact. Removing only the authorized metadata/context/list additions restores complete original10 bytes. All26 other complete plans match their frozen snapshots.

The11 prospective10 source bytes/existence and nine inspected providers independently match before-manifest.json and the actual before/ bytes, including both absent planned test children. The current09 bridge is306lines/11611bytes with SHA2563ec75ff36811ac40447c28ae48de86b26ade1a1fb5c2c5ae1f82bfb7f2450e22. Recorded protected context/research/coverage/earlier amendments are unchanged. These checks establish limited coordinated input preservation, never a globally frozen checkout or renewed source guard. The independent results and retained checker are under.local/open-bitcoin-dev/phase167/amend10callers/check/.

All referenced predecessors exist, are acyclic and precede consumers. Shared owners are transitively ordered with no same-wave overlap.10 remains wave11 after08/09/27;09's bridge and27's complete undo accounting are accepted prerequisites. Frozen02's wave>1/empty-dependency advisory is unchanged historical scheduling metadata and adds no new issue. Requirement frontmatter still covers CVLD-01/02/03/04/06 across the phase;10's original five remain exact. Context D-01–D-10 coverage and deferred168–171 boundaries are preserved. No decision is silently simplified. Existing accepted research resolutions remain applicable; no new unresolved research/API choice was introduced. **Dimension8: SKIPPED** because workflow.nyquist_validation is explicitly false.

### Actual caller and data-contract trace

Current chainstate.rs::connect_block_with_current_time immediately calls immutable prepare_connect_block_with_current_time, which binds a live source and stages before validation-history preparation. A receipt helper alone cannot establish missing admission. The added small mutable caller hook and already-owned validation_history.rs helper are therefore necessary. New admission uses checked real shell time on the actual managed coins owner before preparation; caller current_time and header.time cannot mint it. Core candidate_rules.rs::admit_canonical_reorg_headers validates actual selected ancestry and retains the first successful clock for an identical pending ordered batch without invalidating generation. selected_candidate_context later uses that held clock. The no-clock convenience must bypass admission creation, refuse missing context, and still permit an exact valid held admission. Mechanical fixture behavior stays compatible. Direct public reorg/prepare_reorg retains PreferenceAuthorizationRequired.

Current mempool_lifecycle.rs::reorg_to_branch guards each replacement singly before binding authorization. A singleton has disconnect=[] and cannot prove a fork or complete multi-header batch. The amendment explicitly replaces that canonical route with actual HeaderStore-owned candidate histories/original observations, an exact ordered disconnect/replacement batch bridge in the existing managed provider, and one core admission before authorize_reorg/retain_authorized_reorg/stage_reorg. No later singleton may overwrite it. Core independently checks the active disconnect branch, every preceding replacement, complete batch, generation and supported selector. Preference, body availability, exact work and stage authorization remain separate checks. network/work_authority.rs's checked-copy body eligibility does not call the singleton bridge and need not change.

The unchanged09 network adapter verifies its actual held history, selected params, exact candidate/hash/parent and original clock before calling the singleton bridge. Deferring only off-tip coins admission inside that bridge allows cached/nonextending eligibility to remain eligibility-only. It must not swallow a failed applicable admission or bypass unsupported/foreign/missing-history controls. Cached bytes, raw work, a generic ScriptsValid label and successful guards never supply a coins receipt. Actual branch activation still requires the exact complete admitted batch and genuine absorption. No necessary change was found in either work_authority provider, canonical_admission, candidate_eligibility, network validation_history or filter_index/reorg.

Fresh live BASIC continuation also has existing sufficient extension points. sync/canonical_genesis.rs consumes the original-store RecoveredCanonicalGenesis before configured initialization. chainstate/fjall_store/canonical_genesis.rs creates the genuine core genesis owner; from_recovered_chainstate binds coins-parent identity, actual append proof, generation/branch and exact durable genesis endpoint into ValidatedChainstateLineage. open_runtime::finish_initialized_runtime initializes that owner and drives the ordinary genesis turn. This conditional live authority is separate from a generic public recovery constructor or raw persisted status.

After the first genuine descendant stage absorbs, validation_history::commit_prepared_connect seals its own accepted identity, observes validated lineage and captured BASIC facts, publishes the accepted batch and persists. The existing bounded catch_up path obtains the actual store append proof, checks generation/branch/frontiers, builds records from accepted facts or exact retained body/undo, and obtains manager-authorized append positions. chainstate/fjall_store/reorg.rs binds those positions to the tracked accepted endpoint and same store/proof. Storage append rechecks capability and exact predecessor chain under its publication guard; safe promotion is limited by the durable coins/metadata fence, with own-flush completion confirming the actual accepted endpoint. Only achieved append results confirm manager processed/safe progress. No additional indispensable owner was demonstrated for this fresh-live path. Task2 must prove this through actual managed positives and failure/foreign controls; this static trace is not a behavioral pass. Full descendant physical reopen proof remains11/12.

### Scope warning and unchanged release obligations

Eleven owners exceed the earlier\<=9future-plan target and the generic10-file warning threshold, while remaining below the15-file hard boundary. The actual risk is coherent execution across admission ordering, clocks, reorg batches, undo persistence and index publication, compounded by596-line chainstate.rs and581-line tests.rs. The two existing callers are concrete missing entry points for the original flow; splitting a new feature or inventing a second proof owner would not remove those dependencies. Small caller hooks, substantive logic in the306-line owned provider, and tests in the already-planned phase167 child make this exception bounded. No twelfth owner, large-file allowlist, compressed formatting or relaxed deadline follows. Stop and report a demonstrated further ownership/size barrier before expanding.

Before each correction, actual nonzero behavioral RED remains required; setup/compiler failures, zero discovery and retrospective inverses do not qualify. Preserve progressive chronology if one barrier masks the next. Final positives must traverse direct/stored/local/authorized multi-header reorg absorption with branch-specific MTP, full08 undo persistence and exact accepted-validation/BASIC publication. Controls retain true future-time rejection, caller spoof/reversal, first-clock reuse, identical-batch idempotence, changed-generation invalidation, exact disconnect/order/count/header mismatches, foreign/stale owners and immutable params/mandatory flags. Refusals compare tip/hash/exact work, coins best block and complete relevant coins/undo, ScriptsValid/durable provenance and BASIC processed/safe state. Pre-effect incompatible startup retains full original-manifest equality.

Accepted09's12focused/28node/84header-store/strict-Clippy evidence stays attributable to09. Its documented pre-existing compact fallback difference is not fixed or waived here and grants no compact owner; root's17/20 scope/closure adjudication remains mandatory. New Rust breadcrumbs/19manifests,17 independent semantic review before current guard renewal,20 full native/fresh coverage/Bazel and independent canonical REVIEW/SECURITY/VERIFICATION/lifecycle, root normal-hook saving and stop-before168 remain unchanged.

AGENTS.md local guidance, Bright Builds sidecar/overrides and managed architecture/code-shape/testing/verification/Rust guidance informed this check: pure core, shell clocks, private fallible boundaries, meaningful observed behavior, small modules and native final verification. Both active lesson files were read completely:8856bytes/2953conservative tokens; recentOctober19 baseline applies, with no new audit trigger or archive load. No project skill directory exists. This checker ran no source edit, formatter on source, Cargo/Bazel/test/Clippy, guard renewal, threshold change, Git saving or shared-state edit. Early combined reads exceeded display limits and were followed by bounded source/contract reads; guessed absent read-only paths were setup observations, not test failures.

```yaml
issues:
  - plan: "167-10"
    dimension: scope_sanity
    severity: warning
    description: "11 source owners exceed the earlier <=9 target; admission, persistence and BASIC integration increase execution breadth and central-file pressure."
    fix_hint: "Root must explicitly accept only these two indispensable caller additions, preserve small hooks/provider/test-child boundaries, and stop before any additional owner or size exception."
```

Recommendation: root may accept this single bounded warning and release10 only after confirming accepted08/09/27 and the current lifecycle. No blocker or required plan revision was found. Execution still owes every original behavioral and final acceptance gate.

## Plan10 legacy-refusal test-owner amendment — independent bounded check | 2026-10-22T00:15:25Z

**BOUNDED PASS WITH SCOPE WARNING.** Zero blockers; one explicit12-owner warning. Root alone decides acceptance and releases this test-only correction. This report accepts no10implementation, precise Regtest diagnostic, whole requirement or phase. The earlier caller amendment and all historical report bytes remain intact.

### Current basis and independent preservation

Reviewed latest167-10-PLAN.md,167-10-LEGACY-REFUSAL-AMENDMENT.md, the complete427-line canonical_genesis/tests.rs, actual failed regression log and current final-source-closure.json. Current protected context/research/coverage/earlier amendments retain their previously reviewed bytes. The basis is the completed11-owner implementation at2026-10-22T00:07:12.466Z, not the preimplementation amend10callers snapshot.

An independently written pinned-Bun checker passes all27 GSD frontmatter validations and27 task-structure validations,46tasks/22waves and expected yolo lifecycle167-2026-10-21T16-38-23. Every predecessor exists and precedes its consumer; no cycle, same-wave source overlap or unordered shared owner is present. Frozen02's existing wave>1/empty-dependency advisory remains historical, not a new finding.10 remains wave11 after08/09/27. All five CVLD-01/02/03/04/06 frontmatter requirements retain coverage; no locked D-01–D-10 decision is reduced and no deferred scope is introduced. Existing accepted research resolutions remain applicable. **Dimension8: SKIPPED** because Nyquist is explicitly disabled.

Every captured before/ file matches its manifest SHA256. All26other complete plan files are byte-exact.10Task1 name/behavior/action/acceptance/verify/done remain exact; its only task changes are the one file addition and exact read-first insertion. Whole Task2 and every original automated command remain exact. Removing only the new frontmatter owner, outside-task context paragraph and Task1 file/read-first additions restores complete original10bytes. Original generated_at, lifecycle, requirements, must-haves, waves and dependencies therefore remain exact. The earlier167-10-CONNECT-CALLERS-AMENDMENT.md and all other recorded protected planning files are unchanged.

All12current source inputs match the new snapshot; all original11also independently match each actual00:07:12.466Zclosure owner hash. The added tests.rs is427lines/14601bytes, SHA2563d56611964cc0be077e230fc6842af33d8fd347199e6ebb274f7ecf6f4247bcc. The exact4802-byte target function has SHA2566e24355454ee0a9ebe4f5cf81df58546c9dfa54d493bac889fe0a8e89bf37a18. Its prefix/suffix and shared SELECTORS hash all match the snapshot, proving all other tests/helpers are presently unchanged. Failed log and original source-closure bytes also match. Tracked167-10-SUMMARY.md is absent; root's15/27accepted and85/97milestone counts remain unchanged. Receipts are under.local/open-bitcoin-dev/phase167/amend10legacy/check/.

### Why the test-only correction is coherent

The retained regression discovers16tests and reports15passed/1failed/0ignored. It fails at tests.rs:211 on the first message-route assertion requiring unsupported-validation-scope. It prints no actual returned Regtest error. Static source inspection confirms that09's require_owned_history deliberately recognizes genuine matching Regtest network, core coins and HeaderStore owners; a blanket unsupported assertion is therefore stale for that qualified setup. No exact deeper category is observed or guessed by this check.

The target test loops over the shared four-entry Mainnet/Testnet3/Signet/Regtest SELECTORS array. Each fixture opens proved genesis, leaves peer1 unconnected, clones genesis and changes only its previous hash, with no admitted child. Its effect guards compare full chainstate snapshot, headers, peer snapshot and every durable row across all enumerated namespaces. All four wire message variants traverse receive_sync_message, receive_message and durable-serving routes; the latter retains a panic-on-payload callback. Stored connect, local connect, fixture-header admission and reorg follow, each with assert_unchanged. Cleanup drops the actual handles before deleting the disposable store. The amendment explicitly preserves each of these facts.

Only the existing named function may change. The amendment requires all four selectors and every route/guard to remain, with original unsupported semantics for Mainnet/Testnet3/Signet and observed precise errors for each qualified Regtest message/entrypoint and other refusal route as applicable. It prohibits skipping Regtest, arbitrary is_err replacement, catch-all category masking, production edits and relabeling an unconnected-peer/unadmitted-child refusal as genuine contextual validation evidence. Exact diagnostic observations must be retained and final strong assertions restored. If diagnostics expose a real production issue or changed effects, the executor must stop at a separate checkpoint. These constraints repair the stale expectation without weakening the authority boundary.

The current13/13managed-connect,6/6BASIC,28/28admission,30/30validation-history and strict-Clippy/style results remain attributable to their frozen11source basis. This review checked recorded source closure and the failed regression; it did not rerun those checks or independently accept the complete implementation. Existing09/10genuine-context positives remain unchanged. The unobserved precise Regtest category is an intentionally bounded diagnostic step after release, not missing feature design or permission to invent the expected error.

### Scope warning and future verification

Union11→12exceeds the earlier\<=9target and generic10-file warning threshold, below15hard boundary. Incremental risk is test expectation weakening during diagnostic changes and mistaken attribution of an unadmitted fixture refusal as contextual validation, in addition to the existing integration breadth. This is one427-line existing test owner; Task1has8files and Task2its unchanged4. Production11are frozen, so a new feature/task/proof pipeline is unnecessary. No thirteenth owner, size allowlist, guard threshold or deadline exception follows. All final source files must remain below629lines; the already-frozen627-line basic_tests.rs is not licensed to grow here.

Required execution is explicit: exact failing test with nonzero discovery and retained diagnostic output, restored route-specific assertions, then the complete16-case canonical-genesis module with no ignore/skip; strict affected node all-target/all-feature Clippy; scoped formatting/diff review; exact equality of the original11source hashes plus the twelfth final hash and complete source closure. The24min47s interval is only the earlier checkpoint; final duration must honestly include subsequent regression closure. Original Plan10commands and all failures remain preserved. No duplicate full run of unchanged passing paths is required solely for this test correction; new failures or source changes require appropriate checks.

17independent source review before current ownership renewal,19breadcrumb/parity/docs reconciliation,20full native/fresh coverage/Bazel and canonical independent REVIEW/SECURITY/VERIFICATION/lifecycle, root normal-hook saving and stop-before168 all remain. The compact fallback gap remains separate mandatory17/20scope/closure adjudication and grants no compact owner or waiver here.

AGENTS.md local guidance, Bright Builds sidecar/overrides, managed architecture/code-shape/testing/verification/Rust guidance and the already fully loaded8856byte/2953token active lessons informed the test-only boundary, observed-error requirements and effect equality. No archive/audit trigger, project skill or new library research applies. This checker changed only ignored receipts and this timestamped report append. It ran no source edits, diagnostic/product tests, Cargo/Bazel/Clippy/rustfmt, guard renewal, shared metadata or Git saving.

```yaml
issues:
  - plan: "167-10"
    dimension: scope_sanity
    severity: warning
    description: "12 source owners exceed the earlier <=9 target; the additional diagnostic test correction risks weakening precise refusal/effect coverage."
    fix_hint: "Root must explicitly accept only tests.rs as the twelfth owner; freeze original11, preserve all selectors/routes/guards, retain observed diagnostics and restore exact assertions before final16-case regression and12-owner closure."
```

Recommendation: root may accept this single bounded warning and release only the specified compatibility-test correction. No blocker or additional metadata revision was found; all diagnostic observations and final acceptance checks remain future work.


## Plan11 durable rows/counts prerequisites — independent bounded check | 2026-10-22T00:57:14.775Z

**ISSUES FOUND — HOLD SOURCE RELEASE.** One blocker and one scope warning. This is a planning-only check of the new167-28/29 prerequisites and the permitted original-plan scheduling/context changes. Root must obtain a checked compatibility amendment before releasing28. This report accepts no new implementation, task, replay proof, requirement or phase. Every earlier report byte above this append is preserved.

### Independent artifact results

The independently written pinned-Bun1.3.9 checker passes all29 GSD frontmatter validators, all29 task-structure validators and the originating lifecycle validator. Inventory is29plans/50tasks/24waves. Every original46task XML block and automated command is byte-exact; independently applying only the permitted inverse restores all27 complete original plans. Sixteen unrelated original plans are byte-identical. All source-basis inputs, current11six-owner existence/hashes, historical phase documents/amendments and31 inventoried prior check receipts match their captured before basis.11still has no summary and remains0/2complete. Exact data and validator outputs are retained in `.local/open-bitcoin-dev/phase167/amend11rows/check/independent-artifact-checks.json`; this checker's independent inverse files are under its `inverse/` child.

All predecessors exist, are acyclic and precede consumers.28is wave12 after10;29is wave13 after28;11is wave14 retaining10 and adding28/29;12is wave15 and the downstream wave-only shifts are correct. Shared owners are transitively ordered and same-wave owners disjoint. All five mandatory CVLD-01/02/03/04/06 requirement IDs retain frontmatter coverage. Original D-01–D-10 planning coverage and deferred168–171 boundaries remain intact. The existing resolved research choices remain applicable; this amendment creates no external API/dependency choice. **Dimension8: SKIPPED** because `workflow.nyquist_validation` is explicitly false. Frozen02's independent wave5/empty-dependency structure advisory remains historical scheduling metadata.

The new plans and amendment pass existing mdformat/GFM/frontmatter check mode and scoped planning whitespace checks. These metadata checks ran no Rust/source test, build, Clippy, Cargo, Bazel, source formatting, guard renewal, staging, commit, push or shared-state mutation. The standalone checker's `PASS-PLANNING-ONLY` receipt means its structural/preservation assertions pass; it does not supersede the semantic blocker below.

### Confirmed contracts and evidence limits

Current source confirms the three real aggregate paths: public `save_header_entries`, trusted `save_trusted_header_snapshot` and metadata writers.28plans one strict data-only manifest/fragment provider and every corresponding production loader, coherent header/metadata method extraction, exact old small/genesis bytes, legacy generic reads and strict canonical oversized-row refusal. Replacement of both header projections stays in the same existing database batch. Its mixed-family, arithmetic/order/generation/digest, fault/poisoning and complete physical/logical/namespace tests retain the exact65536-row,4096entry,512MiB,depth/path and30second worker ceilings. No aggregate duplicate, namespace exclusion, extra budget or proof factory is authorized.

The planned count interface observes the actual complete map on existing Chainstate/CoinsFlush. It preserves multiplicities and spent identities, Some(empty) proved genesis, None generic/seeded origins and the old identical-versus-changed ancestry policy for unknown inputs. The safe default rejects Some. Source confirms the actual lifecycle position for read-only count/capability preflight after existing no-work/disk early return and before `persist_ordered_prefix`, hence before body/undo/header/prune/coins mutation. Releasing that borrow and reborrowing the same exclusive owner's unchanged map after coins flush is viable: current flush/sync invalidate stage generation without replacing the count map. Positions+counts remain one metadata stream/commit; later metadata failure cannot pretend earlier coins writes rolled back or publish false CompletedValidatedFlush/BASIC safety.

Declared extractions address the592/601/627/628line owners without touching624line snapshot_codec.rs. Fixture ancestors and required setup functions are pub/pub(crate); Accepted fields stay private and whole typed equality/public snapshots remain available. Node lib.rs already registers the same-executable worker. The proposed test-only measurement re-export through owned fjall_store.rs can bridge the private inspection ancestor without exposing proof/engine owners. Those future functions are planned contracts, not compiled/source-verified implementations.

The original corrected probe/log genuinely reports302positions/301Coins/301undos and headers591086/index591086/meta768467/bodymax285/undomax1218. It is aggregate serialization evidence, not actual durable resource evidence. The initial ZERO-work attempt is setup only. Known-count loss remains a source diagnosis until29runs its genuine behavioral RED.28's missing-count intermediate cannot claim complete canonical history;29still owes final full-count301/BASIC/all-handle reopen/resource measurements, actual preferred replacement, multiplicity/spent/disconnected identities, unknown-origin and unsupported-sink pre-effect controls.11/12still owe every original independent replay, full-state tamper, actual pre-effect copied-worker/source-equality, immediate original reproof, sealed product reopen and next-body acceptance obligation.

### Blocker: unchanged large positive fixture writes a mixed metadata representation

The repository contains an additional **cfg(test), explicitly synthetic fixture** metadata adapter at `packages/open-bitcoin-node/src/chainstate/live_fixture.rs::persist_missing_live_fixture_counts`. It is excluded from production by chainstate.rs's `#[cfg(test)] mod live_fixture`; the network wrapper also requires `AdmissionScope::Fixture`. It is not a production canonical writer or a reason to broaden canonical count authority.

It is nevertheless used by an unchanged required positive setup. `sync/tests/filter_index/reorg/measurements.rs:253` test `phase158_reorg_measurement_retained_over_budget_gap_refuses_before_preview` calls `ForkFixture::compact_early(400,1,16,390,10,0x80)`. In `sync/tests/filter_index/reorg/fixtures/live.rs:401–478`, this creates a fresh synthetic live runtime, initializes fixture genesis, actually admits/connects the complete401-position prefix, persists its trusted header snapshot, performs `FlushMode::Always`, then calls `persist_missing_live_fixture_counts` before initializing BASIC or reaching the target refusal assertion.

Under29's deliberately correct canonical-only accessor this synthetic owner supplies None. With changed ancestry the old metadata policy writes None. Under28the401-position metadata necessarily segments: each existing ChainPositionDto has three32byte hash arrays, whose JSON alone needs at least195bytes per position;401×195=78195bytes before field names/work/times/counts, already above65536. No speculative exact encoded total or executed failure is claimed.

The fixture helper then reads those positions/None, verifies live/physical coins, encodes Some(actual synthetic counts), and at live_fixture.rs:47–51 calls `write_raw_for_test(Chainstate,"chain_meta",bytes)`. That raw mutation does not remove the new manifest/fragments. The next strict metadata read therefore encounters the root+segment-family mixture which28explicitly must reject. The unchanged positive setup is broken before its intended test behavior. Even smaller root-only fixtures can cross the cap when the missing map is added; the concrete401case suffices without relying on that possibility.

Neither plan owns this adapter. Generalizing raw adversarial writes into automatic family cleanup, accepting mixed roots, omitting the regression or giving generic owners canonical Some would weaken the approved contracts. The planner must add a bounded fixture compatibility provider through the same atomic data-only replacement, preserve every existing origin/coins/positions guard and assertion, retain raw-write semantics for intentional corruption tests, and name the exact owner and focused regressions. Another coherent solution needs the same explicit source-grounded preservation. No owner is silently added by this report.

### Scope warning and retained final gates

29has11owners and Task1has9files across core/shell traits, storage publication and reversible helper/test extraction. This exceeds the earlier<=9plan target and generic10file warning threshold, while staying below15. Its size/effect-ordering risks are real; small helpers, exact reversible moves and one count map/metadata transaction make the proposed slice coherent.28's8owners/2tasks are within ordinary file/task targets; its7file integration still requires the full named writer/read/fault/resource evidence. The compatibility blocker must be corrected before root judges the final union of owners. No additional owner, allowlist, minification, relaxed deadline or broader custom-sink permission follows from this warning.

Original positive301/fullcoin/undo/count/work/status/BASIC and pre-effect manifest obligations remain unchanged.17independent source review must precede current guard renewal;18offline registration,19breadcrumbs/parity/docs,20full default native/fresh pure-core coverage/Bazel and independent canonical REVIEW/SECURITY/VERIFICATION/lifecycle remain mandatory. Root alone reconciles shared metadata and performs normal-hook saving/push after clean final acceptance; stop before168. The inherited compact fallback gap remains mandatory17/20adjudication with no waiver or compact source owner here.

AGENTS.md local guidance, Bright Builds sidecar/overrides and managed architecture/code-shape/testing/verification/Rust standards informed this check. Both active lesson files were read fully:6898global+1958repo=8856bytes/2953conservative tokens. The October19 repository audit baseline applies; no new audit trigger, archive load or lesson change occurred. No project skills are present. Early oversized combined displays were followed by bounded source/contract reads; no truncated output is treated as complete behavioral evidence.

```yaml
issues:
  - plan: "167-28/167-29"
    dimension: cross_plan_data_contracts
    severity: blocker
    description: "Unchanged401-position synthetic positive setup enriches None counts via cfg(test) live_fixture.rs raw chain_meta write after the proposed segmented flush, leaving root plus manifest/fragments that strict reads must reject. The necessary compatibility adapter is outside declared ownership."
    fix_hint: "Plan an exact bounded fixture adapter/provider amendment using the same atomic data-only family replacement, preserving all origin/coins/positions guards and raw corruption-write semantics; name the added owner and existing large-positive regression checks. Do not relax canonical-only counts, strict mixed-family refusal or original assertions."
  - plan: "167-29"
    dimension: scope_sanity
    severity: warning
    description: "11 owners and a9file cross-crate task exceed the earlier preferred plan scope and require careful extraction and first-effect ordering; the final compatibility owner union is not yet checked."
    fix_hint: "After blocker correction, root must explicitly accept only the independently checked final owner set or request a coherent split. Preserve sub629formatted files, exact extraction inverses and all focused/final gates."
```

Recommendation: return the single concrete blocker to the planner, preserve all structural/inverse passes and recheck only the exact proposed compatibility delta plus final owner/DAG/preservation closure. **Do not release28or29on this report.**


## Plan11 durable rows/counts prerequisites — revision1 independent recheck | 2026-10-22T01:10:48.714Z

**BOUNDED PASS WITH TWO SCOPE WARNINGS.** Zero remaining planning blockers. The earlier ISSUES result remains valid for its original draft and is preserved in full above. The corrected plans now explicitly supply its missing fixture adapter; this is planning closure, not an executed compatibility fix or implementation acceptance. Root alone may accept the exact scope warnings and release28after confirming the current prerequisite/lifecycle basis. This report releases no source work and accepts no task, requirement or phase as complete.

### Exact correction and preserved history

Read corrected28/29 and the full durable-rows amendment, the original source-backed failure path, revision1planner receipt and its preserved first metadata failure, and the actual fixture/guard/payload/core coverage providers. An independent pinned-Bun1.3.9 recheck passes all29frontmatter and29task-structure validators, all must-have fields, requirements and originating yolo lifecycle167-2026-10-21T16-38-23. Inventory remains29plans/50tasks/24waves. Dependencies are valid/acyclic/earlier-wave; no same-wave overlap or unordered shared source exists.28→29→11→12and all downstream/final ordering remain intact. Frozen02's independent wave5 advisory remains historical; **Dimension8: SKIPPED** because Nyquist is explicitly disabled.

All original46task XML blocks/automated commands remain byte-exact; independent full inverses restore all27original plans, with sixteen unrelated complete originals byte-identical. Original context/research/decision coverage/amendments/source basis, current11six declared owner hashes/existence, source/log inputs and31inventoried earlier check receipts remain exact.11still has no summary and remains0/2complete. No deferred decision, requirement, evidence class or original verification obligation is reduced.

Every one of the40revision1-before snapshots matches its manifest hash. All prior checker scripts/receipts/inverses and the entire161715byte pre-recheck report match that correction baseline; its SHA256 is0c9bf221c0eba1a44ed14f4e32c48854921b396e3ff2dfac439498dcdf015801. This retains the original149736byte historical prefix and the initial concrete semantic blocker. Pre-correction28/29/amendment and planner script/receipt are retained as exact frozen snapshots. The planner updated its live validation script for the new revision and separate output receipt; the old script snapshot and original planner-validation.json remain exact. This intentional script revision is distinguished from immutable historical receipts. An initial independent recheck overconstrained that live script to remain unchanged; its one metadata assertion failure is retained in revision1/independent-artifact-checks-first.json before correction. It is not behavioral RED or a plan/source failure.

The final independent artifact receipt is `.local/open-bitcoin-dev/phase167/amend11rows/check/revision1/independent-artifact-checks.json`, errors:[]. An additional exact-delta check restores the entire pre-correction29file by removing only the new inline-core-test action paragraph and its automated command.29's complete original frontmatter, interfaces, Task2 and other contracts are therefore exact.28adds exactly one existing source owner, chainstate/live_fixture.rs, for nine owners total; Task1has eight files.29retains eleven owners/Task1nine files. Final mdformat/GFM/frontmatter and scoped planning whitespace checks pass. The correct unescaped `phase165_rebind_fixture_counts` command stem matches all three existing functions; the planner's interim escaped-filter metadata failure remains attributable separately.

### Why the fixture correction closes the planning blocker

The source chain is unchanged: `phase158_reorg_measurement_retained_over_budget_gap_refuses_before_preview` calls `ForkFixture::compact_early(400,1,16,390,10,0x80)`, whose fresh synthetic Live401-position prefix is actually admitted/connected and full-flushed before explicit missing-count enrichment. Generic/seeded/noncanonical owners continue to supply None through29's canonical-only observation. The existing fixture helper's subsequent raw chain_meta insertion would mix an aggregate root with the new segmented family. No executed failure is claimed;28now expressly requires actual post-segmentation/pre-adapter RED and final GREEN against that exact unchanged test and inputs/assertions.

The sole additional149line owner is included only by `#[cfg(test)] mod live_fixture`; its network wrapper still requires AdmissionScope::Fixture. The correction changes only persist_missing_live_fixture_counts's final enrichment call, with inline compatibility tests in that short module. It retains validate_live_origin, exact coins-parent belongs_to, no pending history, complete positions/live/physical Coins, no heads, matching best block, missing-map refusal, Some-equal write-free idempotence, Some-conflict refusal and map_persist behavior. The two unrelated transfer/BASIC-attachment methods must remain byte-exact.

The proposed cfg(test) bridge lives in already-owned chain_meta.rs and calls the shared prepared family replacement in one existing-store batch with original Sync durability. `PublicationControl` is already pub(crate) re-exported through filters.rs; the caller can borrow its existing MutexGuard without a new visibility owner or capability type. It obtains that guard from the exact cloned store used for all checks and retains it through commit. The bridge must not reacquire the publication mutex or invoke the unverified/validated metadata writers. This avoids the deadlock and additional coverage/append invalidation those general writers would cause under the held lock. Foreign-store guard patterns are prohibited in this contained test-only call; no canonical authority follows from the bridge or its map.

Current put_bytes/is_payload_key confirms Chainstate/chain_meta is neither body/undo payload nor BASIC: the old raw enrichment does not advance payload generation or invalidate read integrity/append/publication/coverage state. The new shared-family bridge explicitly preserves those semantics while replacing the complete bytes. Database sequence/representation may change; complete live map, positions/Coins/undo/work, coverage/status/provenance, BASIC pending/completed/revision/generation/branch/checkpoints/protection and payload epoch may not gain new authority or reset. write_raw_for_test itself stays byte-exact and raw, with an intentional root-plus-segments corruption control requiring strict refusal and no repair.

Existing fixture setup is source-grounded: flush_lifecycle.rs has a cfg(test) pub(super) re-export of managed_fixture_with_params, whose function is pub(in crate::chainstate); the newly owned chainstate descendant can use it without editing a provider. Existing public(crate) mining/admission helpers remain available, or local synthetic setup can use already visible APIs. The actual managed fixture enables BASIC and adopts a fully revalidated synthetic Live owner. Inline401 tests now require full real flush, initial segmented None metadata, explicit enrichment, exact map/status/epoch/held-proof comparisons, unchanged refusal cases, write-free repeated enrichment and complete all-handle storage reopen. Raw-write and strict mixed-family controls distinguish this compatibility adapter from automatic repair. Existing400positive setup and all three fixture-count controls remain unchanged and receive explicit nonzero-discovery commands.

No additional source owner is needed for this planned route. The149line module has meaningful inline-test capacity below629formatted lines; an actual overrun or inaccessible required seam still stops for another checked amendment. This is source-grounded viability, not a compile pass or successful future function claim.

### Core coverage and retained data/effect boundaries

The new29paragraph adds direct tests only in its existing178line coins.rs and383line engine/work_authority.rs owners. Local child tests can reach private origin/count fields for corruption controls without a production setter. They cover the actual default CoinsFlush observation on CoinsCache, noncanonical/seeded None, CanonicalGenesisProof-created Some(empty), an actual admitted connect's exact nonempty map, and private invalid-origin/missing-map typed refusal. Full state equality establishes read-only behavior. Each test has one concern and Arrange/Act/Assert; no test-only fake authority or second map owner is introduced. Both files retain ample module capacity under629without an exception.

Source confirms verify.sh reads scripts/pure-core-crates.txt, which selects open-bitcoin-chainstate but excludes open-bitcoin-node. These direct tests are therefore necessary input to final fresh pure-core coverage; node integration alone would not cover the new core branches during that run. The new named chainstate automated command preserves the original node commands; final fresh coverage remains20. Compiler/new-symbol/setup errors are not accepted as actual behavioral RED.

All earlier approved contracts remain: genuine301with both BASIC modes; exact authoritative borrowed occurrence map including multiplicity/spent/disconnected identities; Some(empty) genesis versus None unknown origin; read-only count/capability preflight after legitimate no-work/disk early return and before FIRST prefix/prune/coins mutation; same owner's unchanged map reborrowed after flush; positions+counts one bounded stream/batch; no false rollback of completed coins or CompletedValidatedFlush/BASIC safety. Exact row/entry/byte/depth/path/worker limits, full actual physical/logical measurements, all-handle storage reopen, strict segments and legacy/genesis behavior remain mandatory.28's intermediate missing-count history is not complete canonical success. Manifest hashes, raw DTOs or synthetic fixture enrichment cannot authorize replay/product history.

11/12still own original complete isolated re-admission/replay, full counts/Coins/undo/work/status/BASIC comparison, tamper/private-proof controls, real copied-worker original manifest equality, immediate original reproof, sealed product reopening and next valid body. The original11probe and honest existing RED remain attributable and unsuppressed.17must independently review source before current guard renewal;18offline registration,19breadcrumbs/parity/docs,20full default native/fresh coverage/Bazel and canonical REVIEW/SECURITY/VERIFICATION/lifecycle remain required. Root alone updates shared state and performs normal-hook saving/push after clean acceptance, stopping before168. Compact fallback remains mandatory17/20adjudication without a waiver or owner here.

### Scope warnings requiring root acceptance

28's nine-owner plan/Task1eight-file integration is a bounded extension for one indispensable existing test adapter. It combines strict row formatting, multiple real writer/read transactions and synthetic compatibility; execution breadth and retained epoch/locking behavior deserve explicit acceptance. It remains below the10file generic warning and15file hard boundaries, with two tasks and no new subsystem.29's eleven owners/Task1nine files exceed the earlier<=9plan target and generic10file warning threshold. Reversible metadata/proof-test extractions, first-effect ordering and direct core tests remain the principal risks. Both slices retain sub629formatted files, exact inverses and focused evidence; no additional owner, minification, allowlist, timeout/budget exception or skipped regression follows.

AGENTS.md local guidance, Bright Builds sidecar/overrides, architecture/code-shape/testing/verification/Rust standards and the fully read8856byte/2953token active lessons informed the lock, provenance, pure coverage and verification review. The prior October19 audit baseline still applies; no new lesson/audit trigger or project skill applies. This recheck wrote only ignored checker receipts and this report append. It ran no source edit/formatter, Rust/Cargo/Clippy/Bazel/build/product test, guard renewal, shared metadata, staging, commit, push or execution release.

```yaml
issues:
  - plan: "167-28"
    dimension: scope_sanity
    severity: warning
    description: "Nine owners and an eight-file task combine the bounded format/provider integration with one necessary test-only fixture compatibility adapter; held-lock and unchanged epoch/provenance behavior increase review breadth."
    fix_hint: "Root must explicitly accept this exact ninth owner and its shared under-held-guard replacement contract; preserve existing fixture guards/raw semantics and stop before any further ownership or size expansion."
  - plan: "167-29"
    dimension: scope_sanity
    severity: warning
    description: "Eleven owners and a nine-file cross-crate task exceed the earlier preferred scope; reversible extractions, first-effect preflight and direct core test coverage require the complete planned evidence."
    fix_hint: "Root must explicitly accept only the checked eleven owners or request a coherent split; retain sub629formatted files, exact move inverses, all node/core/resource checks and final lifecycle/native gates."
```

Recommendation: root may accept these two bounded warnings and release28only, with29/11continuation still contingent on complete attributable predecessor evidence and independent root acceptance. The initial semantic blocker is corrected in planning; every actual compatibility/resource/behavior/final acceptance obligation remains future work.


## Plan29 provider expectation amendment — independent planning review | 2026-10-22T03:05:40.403Z

**PASS, PLANNING ONLY, WITH ONE BOUNDED SCOPE WARNING.** No planning blocker found in the narrow amendment. This finding covers the exact three-replacement proposal and ownership/evidence amendment; it supplies no source release, executed test result, task completion or phase acceptance. Root retains adjudication and release. The provider remains frozen at the accepted28 bytes during this review.

### Independent evidence and exact preservation

Read the full current29plan/provider amendment, accepted28summary and original11durable-rows amendment, the captured manifest and planner evidence, and the actual ordinary_301 provider/test module route. Independent checks are retained under `.local/open-bitcoin-dev/phase167/amend29provider/check/`, including `review.ts`, `independent-review.json`, independent whole-plan/source inverses and a separate exact pre-append report snapshot. GSD frontmatter validation and two-task structure validation pass; scoped installed mdformat check passes. The check directory is ignored. No Cargo, source formatter, Clippy, build, Bazel or product execution was run by this checker.

The complete original29file is reconstructed by removing exactly the one added frontmatter owner, the same Task2files entry and the appended context/execution-amendment suffix. The independently retained inverse is26251bytes with SHA256 `6c86d241a72abae17c0c01e7d4fa02c3d395429117dde25c7956f0933cfb5a6c`, byte-identical to its baseline. Every original task action, verify command, behavior, done, name and read-first block is exact; the complete inverse additionally proves all other original metadata, interfaces, objectives, threats, lifecycle and acceptance wording unchanged. Current plan hash is `01e6921506f3af4dff48bb9ae0a3ca9b057a6aa44b24dfe2ed714dbc6cd000fc`; amendment hash is `30b012e6d12c2a29194d8a74d21133198f071d421332610c06b4de2cac9f065f`.

All73baseline snapshot records verify against their captured hashes/existence; excluding the intentionally amended29plan, all72protected live observations match. All other28plans and schedule/context/research/decision/amendment history remain byte-exact. All393historical receipt hashes under amend11rows/plan28/plan11 match. Accepted28summary remains24989bytes/SHA256 `396968e33007a4e64606a71d062ebb99678e68f6622bbf8af5d52dc33111aff3`; original11durable-rows amendment remains41416bytes/SHA256 `f08340333ce129b0823ac5b0a30bfcfe496667d3cc329d9b2cf78b98039f8b44`. The original11six owner states, including absent files, remain exact and there is no11summary. No old measurement or failure receipt was rewritten.

The entire previous canonical report is preserved as an exact175380byte prefix, SHA256 `8098c8495ff76ab034201fe6d302143847f8c3a399ca34d9bfe888596c315c59`. That includes the earlier ISSUES and revision1PASS reports without rewriting their conclusions or warnings. This review is appended after that prefix. Post-append verification and its resulting report hash are recorded separately in `append-closure.json`.

### Exact provider delta and expected behavior

Independently inventoried `packages/open-bitcoin-node/src/storage/fjall_store/bounded_rows/durable_tests.rs::ordinary_301`. Its source remains620lines/22817bytes/SHA256 `c1987be909b8127ef9a3c2c5e6330c65938d5fc618fa2ebefc7e5a1bf38073d9`, matching accepted28. There is exactly one obsolete stored-map is_none assertion, one reopened metadata tuple expecting None and one pending29 diagnostic label. The other None checks describe BASIC progress, append-proof/endpoint or local iteration absence and remain unchanged.

The independently derived candidate makes only these substitutions: stored counts equal the full captured snapshot map; reopened metadata equals the captured snapshot positions plus cloned full map; the label changes to counts_policy=Some_exact_live_map. Its22884bytes/623lines/SHA256 `c5f6f7edbd470dc79012b99c40c98f4a1f00ce1f1c3a350c84c5f89244dc2a95` match the draft proposal exactly. Reversing those three substitutions restores the complete accepted28provider hash above. This proves all other complete setup/input/header/index/work/MTP/Coins/undo/body/status/provenance/BASIC/manifest/physical/logical/resource/reopen assertions and both test bodies/names byte-identical. The separate live Some assertion requiring exactly301map entries is preserved. There is no new case, helper, refactor, production delta, second map owner, default, authority promotion, ignore, dependency or resource expansion in this proposal.

The intended new expectation follows the original29carrier contract: actual proved-genesis canonical ownership maintains Some(exact occurrences), then publishes the same map with positions through the actual flush. The provider's old None assertions correctly described accepted28before that carrier; they would reject the intended complete-count behavior afterward. Full Option/map equality together with the unchanged live Some301assertion establishes every identity/value and prevents None=None from masquerading as complete preservation. Reopened metadata now independently compares positions/counts to the captured live snapshot, while existing wallet/hydration whole-snapshot equality and all-handle reopen remain mandatory. This is data preservation, not replay/product authority. Seeded/generic/unknown None and proved-genesis Some(empty) contracts remain unchanged elsewhere. No behavioral RED or GREEN is inferred from this static finding.

### Named command links and final attribution

Source routes are present from lib.rs storage through storage.rs fjall_store, fjall_store.rs bounded_rows and the cfg(test) durable_tests child. The two existing #[test] functions invoke ordinary_301 with Disabled and Enabled respectively. The appended discovery filter `storage::fjall_store::bounded_rows::durable_tests::phase167_durable_rows_reopen_actual_301_basic_` matches exactly those two source names; each exact command names the existing corresponding full test path and includes --exact --nocapture --test-threads=1. Timing keys are phase167-29-provider301-discovery, phase167-29-provider301-disabled and phase167-29-provider301-enabled. These links are statically verified; actual harness discovery remains required after release.

The amendment requires actual discovery of exactly two cases and each mode execution passing exactly one with zero failed/ignored, in addition to every unchanged original29node/core/module/undo/Clippy/style command. Pinned Bun/Rust, physical process-local TMPDIR, serial timing-lock usage and liveness/polling remain intact. Preserve an attributable old-expectation failure after the carrier separately from the original missing-count RED and setup/compile failures. Final command intervals/argv/exits/log digests must bind all twelve final source hashes and final plan/amendment hashes; earlier eleven-owner runs alone cannot qualify them. Preserve all failed/intermediate receipts.

Fresh full-count measurements for both modes must independently establish all-namespace logical and actual physical/source-copy inventory, authority/BASIC/body/undo maxima, entries/bytes and unchanged source physical manifest. Historical28None measurements remain valid only for their historical source. The new values need not equal them. Limits stay65536bytes per authority value, independently4096physical/logical entries,536870912byte budgets and existing depth/path/30second worker bounds. Failure stops for root adjudication. Final exact provider before/after bytes, three-change delta and whole-file inverse remain required after scoped formatting; no unrelated formatting or minification is authorized.

### Dependencies, scope and remaining gates

Inventory remains29plans/50tasks/24waves.29stays wave13 after28;28→29→11→12and all downstream order remain unchanged. The new shared28→29provider is dependency-ordered. Every dependency is present/earlier-wave, there are no cycles or same-wave owner overlaps.29has two original tasks; Task1retains nine files and Task2now has four, with exactly twelve distinct union owners. The original five CVLD requirement fields and task/decision obligations remain unchanged. No deferred scope or silently reduced decision enters this amendment. **Dimension8: SKIPPED**, nyquist_validation is explicitly false.

Twelve owners exceed the preferred nine-owner scope and generic ten-file warning threshold, while remaining below the fifteen-file hard boundary. The added responsibility is one already-existing helper's exact expectations/label and has no new implementation mechanism. Its candidate is623lines; the eleven current executor-owned files observed in this receipt are each below629lines, but those observations are not final formatted-source/build acceptance. All twelve final formatted files, extraction inverses, complete source attribution and required verification remain executor/root obligations. Any extra owner, helper, assertion change or size/budget overrun requires separate adjudication.

Original11full independent re-admission/replay and exact counts/state comparison remain required; its original RED is unsuppressed.11/12retain complete copied-source/pre-effect equality, immediate original reproof, sealed product reopen and next admission.17source review precedes current guard renewal;18offline registration,19parity/breadcrumb/docs and20full default native/fresh pure-core coverage/Bazel plus independent REVIEW/SECURITY/VERIFICATION/lifecycle remain mandatory. Compact fallback stays17/20adjudication; stop before168. Root owns shared state, final acceptance and normal-hook saving/push.

AGENTS.md local guidance, AGENTS.bright-builds.md, standards-overrides.md and local architecture/code-shape/testing/verification/Rust standards informed the minimal assertion delta, evidence retention, file bound and verification distinction. Both active lesson inputs were read fully:6898global+1958repo=8856bytes/2953conservative tokens; the October19audit baseline has no new trigger. No project skill directory was found. This checker wrote only its new ignored evidence and this report append; it changed no source, summary, historical receipt, STATE/ROADMAP/todo/config, guard or build ownership.

```yaml
issues:
  - plan: "167-29"
    dimension: scope_sanity
    severity: warning
    description: "Twelve source owners exceed the preferred nine-owner and generic ten-file warning threshold; the added twelfth owner is limited to three exact replacements in the existing ordinary_301 helper."
    fix_hint: "Root must adjudicate this exact bounded ownership extension before source release. Preserve the three-change whole-file inverse, every other assertion byte, sub629formatted owners, final twelve-source attribution and all original/final gates; stop before any further expansion."
```

Recommendation: root may accept this bounded warning and separately release only the checked provider delta to the existing executor. This planning PASS supplies no execution release or new source/build result.

## Independent Plan12 sync registrar amendment review | 2026-10-22 | ISSUES FOUND

**Scope:** Planning-only review of Plan12 SHA256 `7b7ed2e5fe9c1f852ebe5afe66e052f22eb3710d8105d643a817f8e6bd44ebd3` and amendment SHA256 `615e7abd78abebbcb34b7236a7ff46fd395c539bec3097999da75c9bf5402114`. **Result: one blocker and two scope warnings.** No source release, provider/task acceptance, guard renewal, production execution result or phase acceptance follows. Root retains warning adjudication and release, after independent acceptance of11.

### Blocking missing copied-worker integration

The conventional sync declaration is necessary and correctly bounded, but it does not make the original full copied-worker validation contract executable within these ten owners. Original12 Task1 routes proved descendant records through11 in the existing isolated copied worker; its key link says “Copied bounded worker validates detached history before original equality/reproof and live assembly.” This availability/isolation obligation cannot silently become an untimed parent callback.

Actual existing flow establishes the gap:

- `storage/fjall_store/inspection.rs:23` calls `inspect_owned_then` with `worker::recover`. In that function the worker reaps, source inventory/path and recovered-tree checks run, scratch cleanup completes, then line124 calls `validate(result?)` in the inspecting parent, retaining the source engine lock. The existing generic/genesis callback contract is deliberate and must remain intact.
- `storage/fjall_store/inspection/worker.rs:54` decodes one `OwnedScratchJob`, checks the copied tree, calls `raw_recover` at line78, then serializes untrusted records with `protocol::write_result` at line87. No history proof or callback runs there. `recover` at line148 and `monitor` at line202 enforce the unchanged30-second child deadline, including input, recovery and child completion. That deadline does not cover a subsequent parent replay.
- None of the ten listed owners provides an insertion point inside this existing child between record recovery and successful result publication. Changing only the inspection parent could run bounded data checks/replay afterward, but cannot honestly claim full copied replay in that child or its deadline. Replacing the worker through a duplicate implementation or new process would breach the retained single-worker contract.

Revise the plan through a separately checked amendment. At least `packages/open-bitcoin-node/src/storage/fjall_store/inspection/worker.rs` needs ownership for an opt-in history path, for example `recover_history(&ScratchGuard, CanonicalChain) -> Completion<InspectedRecords>` using the same launcher/monitor/reaping/result path, with child dispatch that invokes the actual `canonical_bootstrap::validate_history_records` shell entry before publishing success. The existing owned `inspection/canonical_history.rs` can hold the decode/replay adapter. A typed bounded selected-history job mode would additionally need `inspection/protocol.rs`; a smaller transport must be explicitly designed and independently checked, including generic-mode isolation, rather than assumed available. These are proposed signatures, not source changes or a pre-approved transport.

Keep original generic worker arguments/protocol/default behavior and every frozen assertion intact; permit only reviewed opt-in metadata needed for the new mode. Use the existing executable and single worker, with no timeout/cap increase. Require genuine history-mode positive/refusal and replay-stall/deadline/reap/nonmutation evidence, so a recovery-only child plus untimed parent replay cannot satisfy the test. The copied child may return bounded detached records only after full proof; it cannot serialize a public authority token. After successful reaping/source equality/cleanup, preserve compatible original open and **immediate full original reproof** through the real shell entry before recovery/config/index/prune effects; consume that actual original source-bound proof into live owners. A child boolean or DTO never replaces original proof. Root must assess the revised scope and any test-owner needs before source release.

### Independent preservation and formatting checks

New independent receipts are under `.local/open-bitcoin-dev/phase167/amend12registrar/check/`: `review.ts`, `independent-review.json`, all29 independently reconstructed inverse files, isolated registrar candidate/inverse, GSD/formatter logs and the exact pre-append checker snapshot. The receipt status explicitly means static preservation only; this semantic blocker controls the overall verdict.

The entire previous canonical report was independently compared to the captured baseline: exactly186483bytes, SHA256 `cab19158cb1d0fbcc9e7c6576fb7bf9f93af2af414316b482b678b0a63dad8fe`. This append preserves that prefix, including every prior ISSUE/PASS/history byte. Post-append closure records the resulting report hash independently.

Every original task XML byte is preserved after reversing only Task2's new files entry. The complete Plan12 inverse additionally removes exactly the frontmatter owner, tagged integration context and added strict verification paragraph/command, restoring all151lines/16713bytes and SHA256 `e3808ea90486b3b25d7cb6ebfb2863eda3e08f93d1db314759eca9b5090296f3`. Original task actions, behaviors, acceptance, read-first, done, verify commands, threats, interfaces and other metadata are exact. All other28plans are identical to their captured bytes; their complete inverses are identity.

Independent inventory remains29plans/50tasks/24waves. All dependencies exist and precede consumers; shared owners are dependency-ordered, with no cycles or same-wave collision. Plan12 remains wave15 after08/10/11. All29 GSD structures/frontmatters validate; the inherited Plan02 independent-producer wave advisory remains historical. All five CVLD requirements remain in plan frontmatter, all ten locked decisions retain task-action coverage across the unchanged phase, and no deferred phase is added. **Dimension8: SKIPPED**, explicit `workflow.nyquist_validation:false`. Research's four planning questions have concrete inline resolutions; this amendment opens no external research question.

All23 captured source/existence states,38 protected historical documents and270 historical receipts match both live and captured hashes/existence. Live11 close/SUMMARY and root STATE/ROADMAP/todo/config are intentionally excluded; ongoing authorized output is not treated as corruption. No historical attempt or receipt was rewritten.

The ignored proposed `sync.rs` is exactly the one conventional `mod canonical_history;` after `mod canonical_genesis;`:567→568lines,21186→21209bytes, proposed/formatted SHA256 `adb5ee133d03c2c9a942c80912c8f43966dc803a99797406d962a2c236eafada`. Independent stdin rustfmt exits0 and produces identical bytes. Removing that one declaration restores the entire source SHA256 `a3ad8c32886cea6ae778fa5ec79d2775817e57e2f680d79d288ae406eb76e599`; the actual checkout registrar remains unchanged. No inline-path module, visibility change or unrelated formatting is present.

Installed mdformat with GFM/frontmatter independently reproduces the inherited original/amended Plan12 check misses; the original body is not normalized. The new amendment passes check mode, scoped whitespace checks emit no diagnostics, and reversing additions in the separately normalized amended copy restores the full normalized original SHA256 `09d074b669a2d98ec9465d270deeae36ee3fb70534d2032123f7c113c68286d1`. Thus no new formatting residual is found. Only this new ignored appendix is formatted before append.

### Other production API assumptions and retained obligations

`lib.rs:76` already registers the worker in the same unit-test executable. No lib.rs owner is needed. `storage/fjall_store.rs:44` already makes its canonical_genesis module pub(crate), so the already-owned child can define private-field pub(crate) opened/recovered wrappers through that full path. `ProvedHistoryRecords` and its consuming `into_parts(self,&same_records)` remain restricted to storage; the wrapper can return actual public earned genesis/HeaderStore/core owners after checking live-instance, sequence and complete source equality. No proof visibility or parent re-export broadening is needed. This remains a planned wrapper, not evidence that current genesis wrappers already support history.

The checked network assembly is possible with current signatures: crate-visible `from_initialized_chainstate`, canonical scope binding, public `peer_manager_mut().seed_header_store(actual_earned_headers)`, then crate-visible `retain_checked_candidate(actual_tip_body, selected_params, actual_held_context_time)` before `finish_initialized_runtime`. The latter calls canonical guards, linked-parent eligibility and real caching/parameter binding. Its disconnected-parent early success must be excluded by actual parent evidence and checked/cache assertions. Require exact header/core/proof/selected-chain and local network-magic correspondence inside the sealed assembly; `into_canonical_scope` alone proves none of that. Preserve the strict single-genesis constructor. No dummy setter, raw entry import, manufactured clock/context or global support label is approved.

Current earned core `admission_snapshot()` plus the actual durable coins view can form the target; consuming `rebind_into` checks complete coins/heads/bestblock, active positions, active undo and exact counts while transferring private selected work/rules. Keep all independently proved retained orphan undo physically stored. Do not substitute physical304undo rows for the earned core's302active undo. Required actual all-handle **product** reopen covers305known/303active, then next acceptance306known/304active, with complete state/work/coins/counts/provenance/BASIC and physical-resource comparisons. Straight genesis+301 and all inherited refusals remain mandatory; raw-store or fixture reopen does not qualify. Phase169 differential deferral waives none of it.

Real copied history proof and immediate original reproof must call `storage::canonical_bootstrap::validate_history_records`, whose existing fallible SystemTime capture supplies the fresh shell clock and actual re-admitted HeaderStore observations. Bounds remain65536-byte authority values,4096entries,512MiB and30-second worker deadline. No raw original fallback may precede compatibility, and no recovery/config/index/prune effect may precede immediate original reproof.

Read `plan11/root-provider-staging-adjudication.json` and current checkpoint/API notes. The actual24 private-provider dead-code groups are FAILED/staged, not strict success. Plan12 must wire the genuine production consumer and obtain exit0 from the unchanged node all-target/all-feature `cargo clippy ... -- -D warnings` command before acceptance, preserving every unsuccessful attempt. No allow/expect, visibility suppression, dummy consumer or erased diagnostic is permitted. Final attribution must bind accepted11 and all final12 owners/commands, copy-worker resources/source equality, full retained-fork product reopen/next evidence and registrar inverse. Plans17/20 retain compact-fallback adjudication,17 source review before guards, and20 independent source/security/goal, full native/fresh pure-core coverage/Bazel/lifecycle gates. Root owns shared metadata and saving; stop before168.

AGENTS.md local guidance, Bright Builds sidecar/overrides and local architecture/code-shape/testing/verification/Rust standards informed this review. Both active lesson files were read fully,8856bytes/2953conservative tokens, with no archive loading or new audit trigger. No project skills were found. This checker changed only its new ignored evidence and this canonical append, and ran no Cargo/Bazel/application/source test. Source/build evidence remains with the designated executor/root.

```yaml
issues:
  - plan: "167-12"
    task: 1
    dimension: key_links_planned
    severity: blocker
    description: "The original copied-worker full-history validation link cannot be implemented with the ten authorized owners: the existing worker only recovers/serializes records, while the proof callback runs afterward in an untimed parent."
    fix_hint: "Add a separately checked opt-in existing-worker integration scope, at least inspection/worker.rs and any required bounded selected-mode protocol/test owner. Execute actual full replay before child success under the unchanged deadline; preserve generic mode, all original tests, source/reap/cleanup gates and immediate original proof before effects."
  - plan: "167-12"
    dimension: scope_sanity
    severity: warning
    description: "The proposal has ten source owners, above preferred scope; the tenth is exactly one conventional sync declaration."
    fix_hint: "Root must adjudicate the exact revised owner set after the blocker is fixed; no silent expansion or overlong formatted file."
  - plan: "167-12"
    task: 2
    dimension: scope_sanity
    severity: warning
    description: "Task2 now lists six files; the sixth owner is constrained to the one-declaration registrar delta."
    fix_hint: "Preserve the full registrar inverse, cohesive task boundary, final source attribution and every original verification obligation; root assesses revised scope before release."
```

Recommendation: return the concrete worker-boundary blocker to the planner. Preserve these successful static checks and every historical byte; do not release source or accept Plan12 until the revised worker integration is independently checked and11 is accepted by root.

## Independent Plan12 sync registrar and child-worker amendment R1 | 2026-10-22 | PLANNING PASS WITH SCOPE WARNINGS

**Reviewed targets:** Plan12 SHA256 `b0314880472284a50d93ae78f89309d2615d663607ee9e9545c66b96842d1332`; amendment SHA256 `af8ef6afdb91e9d33ba151e0fabb66419c904eaf19f1d49bec6ad8ef239a726b`. **Result: the R0 blocker is resolved in the revised design; zero blockers, three scope warnings.** This is a planning result only. Root must independently adjudicate the thirteen-owner/task scope and release source work. It supplies no source/build/test acceptance, guard renewal or phase acceptance. The prior ISSUES result remains unchanged and accurately describes R0.

### R0 blocker closure: actual child replay under the existing timer

R1 adds the concrete insertion point missing from R0: existing `inspection/worker.rs`, existing `inspection/protocol.rs` and a coherent new `inspection/protocol/history.rs`, all within Task1. The mandatory worker-history contract specifies `recover_history(&ScratchGuard,CanonicalChain) -> Completion<InspectedRecords>`, private typed operation dispatch and one shared launcher/writer/monitor/finish lifecycle. The actual existing `run_child` must execute the real history callback after raw recovery and destruction of scratch DB/keyspace/lease owners, **before** opening the result for output, output-fault seams, successful bytes or child exit. The callback decodes every bounded row family, invokes the actual ROOT `canonical_bootstrap::validate_history_records` shell entry with fallible fresh SystemTime and full11 script/retained-branch replay, then drops all child earned owners.

The source-grounded control-flow projection places that callback immediately after the existing raw_recover scope and before result_file/write_result. Both modes use the same current executable, exact ARGS/root macro and same monitored child. The one existing Instant remains captured immediately before spawn; the unchanged30-second deadline covers input delivery, recovery/destruction, full scripts/replay, serialization/sync and successful exit/reap. It is never reset or replaced by a second stage timer. No second worker, binary or process is introduced. Delivered input, actual successful exit/reap, expected response and unchanged file-identity/length checks are still necessary. A valid-looking result from a live or nonzero child cannot qualify.

This closes the original availability/isolation gap without reinterpreting the untimed parent callback. The history parent callback is explicitly data-only decode/digest. Source manifest/path/engine lock, application lease, scratch security/identity/cleanup and confirmed reaping remain the existing gates. Only after those gates may compatible original open occur, followed **immediately** by complete original namespace/key/value/digest equality, actual full original reproof through the ROOT shell entry and consuming its private proof. Recovery/config/index/prune effects follow that proof. The child sends bounded data, never a serialized actual core/HeaderStore/proof or public compatibility authority. The real live parent owners arise from independent original replay; a child bool or DTO cannot mint them. No raw original fallback precedes compatibility.

### Typed transport preserves the generic contract

R1 keeps original OwnedScratchJob fields, v1 encode/decode/result code, cfg(test) fault layout and old fault meanings, generic callback semantics, generic worker arguments, root registration and frozen tests. The private `WorkerJob::{Generic,History}` dispatch delegates generic operations to exact v1 codecs. Request selection is a private typed operation request, not canonical authority.

History request framing is exact8-byte `OBIJ\x02\0\0\0`, mode1, selector0–3, then the exact v1 job frame. Response framing is exact8-byte `OBIR\x02\0\0\0`, the same mode/selector and exact v1 detached-record frame. Expected-mode/selector decoding plus existing nonce/root device/inode and full root/result metadata binding prevents generic/history downgrade and cross-job success. Unsupported mode/selector/version, truncation, trailing bytes, duplicate/count/length/overflow or identity mismatch refuse without fallback. The complete10-byte envelope counts inside unchanged MAX_JOB/MAX_RESULT; bounded writer arithmetic precedes every write. The envelope reduces available payload capacity rather than increasing any cap. Logical/physical/value/path/depth budgets remain unchanged.

New inline history-codec tests and owned phase167 tests must prove paired valid/script-invalid child behavior, including that the arranged invalid case reaches actual script validation instead of only failing transport/decoding. Unchanged generic data recovery must succeed on that recoverable script-invalid data. New hooks are cfg(test)-only in owned worker/history files and must prove full-replay-entry and post-proof delays share the original timer, valid output followed by live/nonzero child refuses, killed/reaped versus uncertain completion remains distinct, and source/cleanup/lock/effect controls hold. Tests cover wrong bindings, complete retained-fork data and exact outer caps/trailing rules. Frozen worker/tests.rs and original generic assertions remain byte-exact. The original Task1 filter includes the required new `phase167_preeffect_history_worker` names; focused worker and generic phase166 inspection commands additionally require nonzero discovery, zero failures and zero ignores. These are test obligations, not executed results.

### Independent exact preservation and inverses

New checker evidence is solely `.local/open-bitcoin-dev/phase167/amend12registrar/check/r1/`, including `review.ts`, `independent-review.json`, all29 complete inverses, full formatted protocol/worker projection inverses, command logs, this exact pre-append report snapshot and append closure. No R0/planner receipt was rerun into its prior path.

The current entire pre-append report independently equals200153bytes/SHA256 `01d5b3113f92dd574b9fa7e71ae731a52210d9f55dba7e983b9a0ec696d641f4`; its original186483-byte prefix independently retains SHA256 `cab19158cb1d0fbcc9e7c6576fb7bf9f93af2af414316b482b678b0a63dad8fe`. Both prefixes, every earlier ISSUE/PASS/history byte and the R0 conclusion remain exact. Closure records the resulting whole-report hash after this append.

All589 revision0-before snapshot records match their captured hashes/existence, and every corresponding protected live observation remains exact except the two deliberately revised current plan/amendment documents. Their original bytes remain in the R0 snapshot. All original23 captured source/existence states,38 protected historical documents and270 historical receipts independently match both live and snapshot state. Added worker/protocol source and absent history-child state match the R0 source capture. Live root metadata and11 close/SUMMARY are not retroactively frozen historical inputs. This checker reads the current accepted11 summary separately; it does not write it.

The complete original Plan12 inverse removes exactly four frontmatter owners, three Task1 file additions, one Task2 file addition, the two added tagged context blocks and added overall verification text. It restores all151lines/16713bytes and SHA256 `e3808ea90486b3b25d7cb6ebfb2863eda3e08f93d1db314759eca9b5090296f3`. Both complete original task XML blocks are exact after reversing only those file-list additions. Every original action, behavior, read-first, acceptance, done and automated command is preserved, as are all original objective/interface/threat/lifecycle fields. Other28plans have byte-exact identity inverses.

All29 GSD frontmatters/task structures pass. Inventory remains29plans/50tasks/24waves; Plan12 remains wave15 after08/10/11. Every dependency exists and precedes its consumer; dependency-ordered shared owners have no cycle or same-wave collision. The inherited Plan02 wave advisory remains attributable. All five CVLD requirement mappings and all ten locked-decision task obligations remain; the added child contract strengthens D-07/D-08/D-09/D-10 without adding deferred scope. No requirement or original acceptance criterion is reduced. Must-have truths remain observable pre-effect refusal and actual product reopen; the added worker link now names its executable insertion point. **Dimension8: SKIPPED**, explicit nyquist_validation:false. Previously resolved research questions are unaffected.

Independent mdformat GFM/frontmatter checks reproduce the inherited original/current Plan12 misses; the original body is not reformatted. New amendment check passes. Reversing the declared additions in a separately formatter-normalized R1 copy restores the entire normalized original SHA256 `09d074b669a2d98ec9465d270deeae36ee3fb70534d2032123f7c113c68286d1`, with no new residual. Scoped whitespace checks emit no diagnostics. Only this new ignored appendix is formatted before append.

### Source-size feasibility and bounded scope assessment

The exact union is original9+sync.rs+worker.rs+protocol.rs+new protocol/history.rs: thirteen distinct owners. Task1 has seven files for one copied-worker/protocol/pre-effect boundary; Task2 has six for one sealed runtime/core boundary. There are still only two implementation tasks. This is above preferred scope and merits explicit root assessment, while below the15-owner hard boundary. The conventional registrar is exactly one23-byte declaration after canonical_genesis,567→568lines. Removing it restores full source SHA256 `a3ad8c32886cea6ae778fa5ec79d2775817e57e2f680d79d288ae406eb76e599`; its isolated formatted proposal remains SHA256 `adb5ee133d03c2c9a942c80912c8f43966dc803a99797406d962a2c236eafada`. Actual checkout source remains unchanged.

Independently reversed the **entire formatted** protocol-parent projection to actual495-line source SHA256 `0ba8288008cd87dd2e4caaec804d7d170f59a5b9413fcc3871a0e070a22094ae`; original v1 codecs/fields/tests are exact. Its forward formatted projection is567lines/SHA256 `2fd4f5bc80ac45ddbae169784258cd933dacc37c411ee53402db5431e0f0117e`, within the reserved590. Independently reversed every exact mode/callback adaptation in the **entire formatted** worker-parent projection to actual288-line source SHA256 `259c5cd0e07fa800a3feb29d036f25d89d5976c5267eb2289399293937b2f455`. Its formatted312-line projection SHA256 `4778d9e1da6a9e7f6f766978af3f9c8ae3c58180bfe999e2829308d6fe65567f` leaves room within420. Each isolated rustfmt check exits0 with identical bytes.

These projections are not compiled implementation. In particular, projected cfg(test) configure_command still refers to moved `job.fault`; final implementation must use the held operation's existing owned fault accessor within worker.rs. This is a local adaptation covered by its explicit remaining-implementation reserve, not another source owner or an accepted compiler result. The new history sketch is only52interface/schema lines, SHA256 `08d80413540a83016c2f0df54f7885977371b9e493d3be4f81fa4e3d46c273c5`; its actual bounded codec/writer/negative tests remain projected≤500. No implementation fit or behavior pass is inferred from that sketch.

All thirteen **actual final formatted** sources must be below629lines. The stated projections, including phase167_tests≤620, do not authorize compression, missing tests, exemptions or a fourteenth owner. Stop for checked adjudication if final mechanisms/tests exceed those boundaries or context becomes insufficient. These cohesive scoped additions and explicit stops make the present plan feasible with warnings; root alone accepts that risk before release.

### Unchanged actual authority, reopen and final gates

Current11's six source hashes independently match the accepted basis, including proof SHA256 `94761ed26a0adf309eebe10c0206d1be557632ed1914214516103262cc79a477`. Its separately read root-formatted SUMMARY retains SHA256 `f79782106551984fd520f37d269fe07bed5ece9aaba3e90a1ba77ffe98e59f1e`. Root's bounded provider acceptance is acknowledged; its24 actual strict dead-code diagnostic groups remain **FAILED/staged**, not a lint pass. R1 must supply meaningful child and production consumers and then exit0 from the unchanged strict node all-target/all-feature `-D warnings` command before12 acceptance. No allow/expect, visibility suppression, dummy calls or erased diagnostics are permitted.

The prior source-grounded API assessment still holds. Same unit-test worker registration needs no lib.rs edit. The existing pub(crate) canonical_genesis module path can hold private-field opened/recovered wrappers, consume the storage-private proof and return actual earned public genesis/HeaderStore/core without parent re-export or proof visibility broadening. The checked sync assembly can bind real core/selected params/local magic, install the actual earned HeaderStore through peer_manager_mut, and genuinely check/cache the actual proved active-tip body with retain_checked_candidate and actual held context before finish_initialized_runtime. Its disconnected-parent early success must be excluded by real linkage/cache assertions. Preserve the strict genesis-only constructor and unsupported non-regtest guards; selected transport bytes alone confer no canonical support.

Build the durable target from earned active admission_snapshot plus actual coins, then consume rebind_into. Keep all separately proved physical orphan undo stored; do not inject physical304 retained undos into the core's302active undo. Actual all-handle **product** reopen must preserve305known/303active and then accept next306known/304active, alongside straight genesis+301, maturity/halving, exact counts/state/work/provenance/BASIC and every source epoch/manifest/effect/refusal control. Raw-store or fixture reopen does not qualify, and Phase169 differential deferral waives none of it. Root's current active-prefix clarification preserves exact proved durable active ancestry without claiming maximality among all persisted ScriptsValid nodes or adding implicit roll-forward/strict-max refusal; it changes no source or gate.

All authority-row65536, entry4096, byte512MiB and worker30-second caps remain fixed. Actual copied full-proof timing must include scripts/replay/result/exit, not recovery-only measurements. Final summary/receipts must bind accepted11 and all thirteen final12 source hashes, every original/additional command and failed attempt, worker/source/resource closure, original consuming proof and product retained-fork reopen/next evidence. Plans17/20 retain compact-fallback adjudication,17 source review before guard renewal,19 parity/breadcrumb/docs and20 independent source/security/goal/native/fresh pure-core coverage/Bazel/lifecycle. Root owns shared metadata and normal-hook saving; no Phase168 work is authorized.

Local AGENTS, Bright Builds sidecar/overrides and architecture/code-shape/testing/verification/Rust standards informed typed transport, exact frozen-contract preservation, minimal owner boundaries and evidence distinctions. The same complete active lessons remain loaded within8856bytes/2953conservative tokens, with no archive loading or new audit trigger. No project skills are present. This checker changed only its new ignored check/r1 evidence and this append; no source, tests, summaries, shared metadata, guards, Cargo/Bazel/application run, commit or push occurred.

```yaml
issues:
  - plan: "167-12"
    dimension: scope_sanity
    severity: warning
    description: "Thirteen distinct source owners exceed preferred scope while remaining below15; additions resolve the genuine child replay boundary and conventional sync registration."
    fix_hint: "Root must independently adjudicate this exact union before source release. Preserve two cohesive tasks, final attribution, all original gates and strict stop on owner/size/context expansion."
  - plan: "167-12"
    task: 1
    dimension: scope_sanity
    severity: warning
    description: "Task1 owns seven files spanning the cohesive copied-worker, typed transport and pre-effect proof boundary."
    fix_hint: "Keep generic code/tests exact, actual history replay inside the unchanged child deadline, all new meaningful controls and every formatted owner below629lines; stop for adjudication if additional scope is necessary."
  - plan: "167-12"
    task: 2
    dimension: scope_sanity
    severity: warning
    description: "Task2 owns six files for sealed runtime/core integration; the added sync parent is restricted to one declaration."
    fix_hint: "Preserve full registrar inverse, actual earned-owner/network binding and full product retained-fork reopen/next evidence; root assesses scope before release."
```

Recommendation: root may adjudicate these three bounded scope warnings and separately release the checked R1 after its own review. The planning blocker is closed in design only; all actual implementation, compilation, full child timing and final acceptance evidence remain required.

## Independent Plan13 context registrar amendment | 2026-10-22 | CONDITIONAL PLANNING PASS

**Reviewed targets:** Plan13 SHA256 `31d302118652e0616d623e2794ec9d3ba7d05b73648fdb37e7f76bff4e594e8b`; amendment SHA256 `386f2df8dcd0889cd3d37c62479617f3ed06d9615e33dd9e0aa134fa5d028057`. **Result: zero blockers, one planning clarification warning**, under the explicit root14 interpretation below. Nine owners are sufficient conditionally; this is no claim that unfinished12 APIs compile or that13 may execute now. Accepted12 and actual external safe-API verification remain mandatory before ANY13 source release. Root alone owns that release and final acceptance.

### Specialized configuration-only factory preserves the original authority boundary

Original Task1 requires accepting only a fresh/reopened sealed12 runtime and rejects raw store/params/positions, fixture owners, foreign selection and startup activation. The proposed factory takes only immutable RuntimeConfig, internally acquires a genuine selected fresh/reopened producer result and **immediately consumes that actual seal** through the accepted safe node consumer. It then assembles from actual returned runtime/network/store/selected rules and WalletState::Unavailable. This meets the original security condition by moving acquisition inside the factory; it does not replace a seal with a public runtime or label test. Every original action and acceptance byte remains unchanged.

No caller runtime/store/boolean/numeric-work/position/fixture/BASIC/network envelope is a proof input. Keep intermediate helpers private. A public seal in an otherwise unnameable node module can be used only if the actual external producer/consumer call works by inference; this review does not infer such accessibility from proposed names. No node export/visibility owner is permitted. Fresh initialization must use the existing sealed genesis producer/consumer; existing history must consume actual12's same-child full replay, fixed deadline, source/lease/cleanup gates and immediate original reproof. Safe path selection must follow checked producer contracts and immutable configuration, never a broad fallback after proof refusal or a label pretending to establish authority. Invalid configuration and activation must refuse before effects; the sealed owner must bind actual selected chain, params, network and held runtime.

The stable RPC sibling interfaces support this design: context/network.rs has pub(super) assemble_context; wallet_state::WalletState is pub(super) with Unavailable; the existing canonical_genesis factory already demonstrates consumption and concrete durable context assembly. Thus context.rs registration provides conventional sibling reachability without changing those provider files. The new canonical history child and the original owned canonical_genesis/canonical_chain files can hold the narrow factory/preflight logic. This is conditional design feasibility, not a passing compilation result.

### Capability versus authority: explicit root14 condition and warning

Read-only inspection found the public context/network.rs::from_runtime_config_with_network_handle seam: it checks selected configuration, network.maybe_canonical_chain envelope, chain mismatch and require_ordinary_scope, then may configure network/build wallet/recover mempool. Envelope scope alone is not a seal. Changing only owned try_from_runtime_config_with_store would not independently harden this unowned path if the shared qualification gate were actually relaxed.

Root independently clarified and recorded the **unchanged actual14 contract** in `.local/open-bitcoin-dev/phase167/plan14-preflight/root-contract-interpretation.json`, SHA256 `3a4291e8a879461d4d7bba39e3cafcbed0b73f8eef7ee56ca22327daf0ea8fae`. It binds original14 and stable classification/CLI inputs, preserves require_ordinary_history returning Err for every selector and ordinary_history=false/identity-only qualification, and adds only separate immutable bounded default-regtest implementation facts. It grants no14 release. Under that explicit condition, the unowned generic handle/wallet/mempool/activation paths stay barred, so no hypothetical global relaxation requires a tenth13 source owner.

Task1 still must add its **independent** UnprovedOwner guard to owned config/store generic constructors and aliases after preserving selector/prerequisite diagnostics, before wallet/network/store effects. Explicit fixture construction remains synthetic. Do not count current global refusal as proof that this new independent guard exists; meaningful owned tests must verify the distinction. Conversely, do not claim that all unowned public generic constructors now have independent owner guards. Their safety condition here remains the unchanged global qualification refusal.

The warning is wording: “Future14 exposes Regtest … require_ordinary_scope succeeds” in the amendment must be read as a hypothetical regression threat, not as the authorized14 behavior. Minimal clarification is to state that actual14 leaves global qualification refused, separate bounded implementation facts confer no owner authority, and owned config/store routes independently refuse; any later real global relaxation must first review all listed unowned seams. Root's recorded interpretation supplies this precise condition without editing the frozen13 target. This conditional PASS becomes inapplicable if those gates are relaxed or if a caller-labeled runtime is treated as authority.

### Original genesis, worker and real-process evidence remain strict

The existing binary test pins open_authoritative_network_runtime to `fn(&RuntimeConfig,Option<open_bitcoin_node::storage::OpenedCanonicalGenesis>) -> Result<OpenedAuthoritativeRuntime,DaemonSyncPreflightError>`. It and the entire existing test remain unchanged. Use a **distinct** history route inside the already-owned canonical_startup.rs/open-bitcoind.rs; do not widen the old helper or turn None/raw storage into authority. Four-selector genesis contexts/startup/reopen and all old refusal tests remain mandatory.

RPC lib.rs already invokes the node worker registration macro for its unit tests. The binary already runs maybe_run_inspection_worker before run_application/configuration and registers the unit-test macro. No lib/macro/worker source owner or second process implementation is necessary. Preserve this early dispatch and the actual product executable boundary. The process integration target must use `Command::new(env!("CARGO_BIN_EXE_open-bitcoind"))`, with the hyphen, cleared inherited environment, isolated datadir/config, loopback authentication, bounded redacted diagnostics and default-off external networking. The owned serialized default-regtest history must work before15's corpus. Harness-only or stale target/debug execution does not count.

Retain valid authenticated submissions, invalid/context refusals with complete accepted-state comparison, no wallet files and unavailable wallet/rescan behavior, graceful stop/reap/drop of all handles and same executable/datadir restart. Direct durable observations occur only while the daemon is completely stopped and must be dropped before restart; no concurrent second Fjall owner. Compare tip/hash/full work/coins/BASIC/provenance rather than endpoint labels alone. None of this local product evidence qualifies actual Knots differential or later scope.

All five original automated commands are exact: Task1 library filter, Task2 daemon unit filter, new Cargo auto-discovered process target and both original four-selector genesis selections. Require nonzero named discovery, zero ignored tests, all failures/retries and exact final source/API attribution. Before ANY13 source work, root must accept completed12 and verify actual safe producer/consumer signatures, accessibility by inference, selected/owner/network binding and copied-worker/original-proof ordering. Provisional observed names are not accepted API evidence. If unavailable, stop for a checked amendment; no source or visibility expansion is silently authorized.

### Independent preservation, contracts, inverses and formatting

New independent receipts are confined to `.local/open-bitcoin-dev/phase167/amend13registrar/check/`, including review.ts, independent-review.json, all29 complete plan inverses, the isolated registrar candidate/full inverse, formatter/GSD logs and exact pre-append checker snapshot. The current entire canonical report independently matches217153bytes/SHA256 `f734e6f31fe5f2eb2593358eeb75ef42d4fbfdabeb43d53107a10cb765f75ced`. This new result is appended after that exact prefix, preserving every earlier ISSUE/PASS/history byte. Post-append closure records the resulting whole-report hash separately.

The whole original13 inverse removes exactly one frontmatter owner, Task1's context.rs files entry and the new tagged context block. It restores157lines/18071bytes/SHA256 `ee5bbd4be54a0d4f25b5ec2d545f3cf47730720b8775a4b99000c1f3b10c3db1`. Both entire original task XML blocks are exact after reversing only that files addition. All original actions, behavior, acceptance, read-first, done, five commands, threats, interfaces, objective, verification, requirements and lifecycle remain byte-identical. Other28plan inverses are identity.

All29 GSD frontmatters/task structures pass. Inventory stays29plans/50tasks/24waves, Plan13 stays wave16 after12, and every dependency exists and precedes its consumer. Shared owners are dependency-ordered without cycles or same-wave collisions. The two sequential tasks share context/canonical_history/tests.rs deliberately; their union is nine owners, with task file counts5/5. This stays within the preferred nine-owner scope; the new responsibility is one conventional registrar declaration and introduces no additional task. The inherited Plan02 independent-producer wave advisory remains unchanged.

All14 stable RPC source/existence states,40 protected historical phase documents and1735 historical receipts independently match live/captured hashes and existence. This includes current12R1 plan/checker/acceptance history, not active12 source execution. Live12 node sources, plan12 execution receipts/SUMMARY and root live STATE/ROADMAP/todo/config are explicitly excluded. This checker neither freezes nor interferes with them. Additional read-only RPC lib macro inspection is recorded separately; it does not alter the14-state baseline.

All five phase CVLD requirement fields remain covered across the phase. Plan13 keeps its original CVLD-01/02/03/04 contributions; unchanged prerequisite/downstream plans retain CVLD-06 ownership and all D-01–D-10 task obligations. Added context supplies the original consuming-factory and product wiring, without reducing a decision or adding deferred scope. Must-have truths remain observable sealed RPC continuation and real process reopen. **Dimension8: SKIPPED**, explicit nyquist_validation:false. Previously resolved research questions are unchanged; active12 accessibility is an explicit release gate, not an assumed finished API.

The actual checkout context.rs remains501lines/16168bytes/SHA256 `f0fc6351d63d61c87ea17ebaa1080db7143918fafcddb77e3353c242dce6eb15`. The independently derived proposal adds exactly23bytes, `mod canonical_history;` immediately after canonical_genesis, giving502lines/16191bytes/SHA256 `f15169d2d40a55245c6d5f0d50aa5ce07efc19ef461e59a793fb560ebf0dccc9`. Isolated stdin rustfmt exits0 with identical bytes; deleting that declaration restores the **entire** original source. No other formatting, export, inline-path module or refactor occurs. All nine actual final formatted owners must stay below629lines; any needed additional owner or size/context failure stops for checked adjudication without minification or waived controls.

Independent GFM/frontmatter mdformat checks reproduce the inherited original/amended Plan13 misses; the original body is not normalized. New amendment check passes. Removing the declared additions from a separately formatter-normalized amended copy restores the complete normalized original SHA256 `57efa266acc627c3fabf9495b843c37b8e465ccb733a7f56611df919b8cdf3eb`, proving no new residual. Scoped whitespace checks emit no diagnostics. Planner's first case-sensitive validator error and initial formatting/inverse attempts remain attributable; this checker did not rewrite or rerun their output paths. Only this new ignored appendix is formatted before append.

### Remaining acceptance boundaries

Final13 receipts must bind actual accepted12 summary/API/source/evidence hashes, all nine final13 source hashes, every original command/failure, meaningful generic/fixture/config refusals, real authenticated process stop/reopen/next evidence and full registrar inverse. Accepted12 must retain its meaningful timed child plus production consumer, unchanged strict node all-target/all-feature -Dwarnings exit0, complete retained-history/source/resource obligations and private consuming owners; no13 evidence retroactively accepts unfinished12.

Plans17/20 retain compact-fallback adjudication;17 source review precedes guard renewal;19 owns parity/breadcrumb/docs;20 owns full native/fresh pure-core coverage/Bazel plus independent source/security/goal/lifecycle acceptance. Root owns shared metadata and normal-hook saving. No phase/global requirement completion, source-hash renewal or Phase168 work follows here.

Local AGENTS, Bright Builds sidecar/overrides and architecture/code-shape/testing/verification/Rust standards informed conventional module registration, actual typed ownership, generic-refusal analysis and evidence distinctions. The fully loaded active lessons remain8856bytes/2953conservative tokens, with no archive loading or new audit trigger. No project skills are present. This checker changed only its new ignored evidence and this canonical append, and ran no Cargo/Bazel/application/source tests, source formatter in write mode, Git saving or build command.

```yaml
issues:
  - plan: "167-13"
    dimension: context_compliance
    severity: warning
    description: "The amendment's future14 capability wording is ambiguous: actual14 must preserve global qualification refusal, and the unowned public network-handle path does not independently prove ownership."
    fix_hint: "Apply the exact root14 interpretation receipt: global gates stay Err; separate immutable implementation facts grant no authority; owned config/store routes gain independent UnprovedOwner. Clarify the wording when authorized. Any future actual global relaxation requires review of all unowned seams before release."
```

Recommendation: root may accept this bounded planning interpretation and warning. Preserve the frozen targets and historical prefix; withhold13 source release until completed12 is accepted and its genuine external producer/consumer route is verified. This conditional planning PASS grants no implementation or phase acceptance.

## Independent formal submission split review | 2026-10-22 | ISSUES FOUND

**Frozen targets:** Plan13 `8f7c6f799b644d2f517644b55c8a86697c1be9195c70b876229f8d087ccd110d`; Plan30 `8a5650e9b74ed3695cc62da678ab6024be11a3333627a6fabacaef6bd4268552`; amendment `bcfc142f04bca6c6d43fcd1c31e85684162306428c175e77768bddf239b239cb`. **Verdict: one blocker, two scope warnings.** The exact preservation/DAG checks pass; they do not resolve the newly identified wire/domain boundary. No source execution, guard renewal or phase/global requirement acceptance is granted.

### Blocker: the proposed prefix API cannot represent the pinned submission wire boundary

Plan30 Task1 reuses parse_block_from_reader and promises unchanged strict decoding, with new parse_block_prefix returning `(Block,usize)`. Plan13 Task1b maps decoding failures to -22, then sends that domain Block to the factory-earned command for conclusive validation. This misses a concrete distinction between complete wire deserialization and domain validity:

- `packages/open-bitcoin-codec/src/transaction.rs:142` calls `Amount::from_sats(reader.read_i64_le()?)` while parsing each output. `packages/open-bitcoin-primitives/src/amount.rs:28–38` has a private i64 field and rejects every negative or over-MAX_MONEY value. The current parser therefore cannot return a Block containing these perfectly serializable signed values.
- The pinned `src/consensus/amount.h:13–14` defines CAmount as signed int64, explicitly allowing negative representation. `src/primitives/transaction.h:149–162` serializes CTxOut's nValue and scriptPubKey without the Rust domain constructor. `src/core_read.cpp:220–234` completes TX_WITH_WITNESS deserialization before returning success. `src/consensus/tx_check.cpp:11–33` then distinguishes consensus invalidity, including bad-txns-vout-negative, bad-txns-vout-toolarge and total-value overflow. Through `src/rpc/mining.cpp:591–606,1142–1168`, a fully parsed, appropriately arranged invalid-amount block reaches a conclusive BIP22 reason rather than -22.
- The same premature-domain-validation seam exists at `transaction.rs:161–163`: parse_script calls ScriptBuf::from_bytes. `primitives/src/script.rs:34–42` rejects scripts above10000bytes during decoding. Pinned `src/script/script.h:410–466` serializes CScript's byte-vector base, without a10000-byte wire-construction restriction. A larger unexecuted output script is not automatically a block-invalid script; it cannot be silently treated as malformed serialization. The frozen prefix return type cannot carry those raw bytes through the current restricted ScriptBuf either.

These safe domain invariants are not inherently wrong; using them as the complete pinned wire representation is the problem. No source was changed to demonstrate it. This is static source evidence, not an execution claim.

Simply converting CodecError::Amount or CodecError::Script into a rejection string is **not** a fix. The decoder may stop at that field before later script/witness/transaction/block bytes prove complete, so a later truncation could be hidden. Pinned ProcessNewBlock at validation.cpp:4924–4950 first calls intrinsic CheckBlock. CheckBlock at4360–4405 checks header/PoW, merkle/mutation, size and coinbase ordering before transaction amounts; an early Rust constructor error cannot prove the appropriate final reason. Clamping values, substituting an empty script or manufacturing a normalized placeholder Block changes actual transaction bytes/txids/merkle and proves neither syntax nor rejection precedence.

The minimal missing **interface decision** is a complete lossless submission-wire boundary, separate from domain construction and preserving exact original transaction identities, followed by the real pinned validation order. Revise the explicit decode/result/command contracts and owner closure to explain how raw values/scripts survive complete deserialization and ordered validation, including accepted oversized-output cases where applicable. Keeping the current Block-only interface requires representation work beyond its current restricted Amount/ScriptBuf; those primitive owners are not in the six-provider closure. An alternate lossless wire view also needs concrete types, conversion/validation links, full identity/merkle preservation and every required owner declared. This review does not pre-approve weakening existing domain constructors, duplicating a consensus calculator or adding a second approximate parser. The existing six-owner feasibility claim is not established for this missing boundary.

Required regression design includes complete negative and over-limit amounts with correctly arranged PoW/merkle/coinbase context; the same inputs with later truncation/malformed witness; competing earlier PoW/merkle/coinbase failures; complete larger raw scripts and applicable valid output cases; exact raw txid/merkle identity; and both prefix/strict caller behavior. Retain the existing allocation/capacity controls and all old strict tests. No cap increase, -22 approximation, guessed reject string or narrower promised behavior is authorized. Root confirmed this issue and requested a bounded planner revision; the frozen candidate remains untouched.

### Independent preservation and DAG checks passed

New evidence is only `.local/open-bitcoin-dev/phase167/amend13submission/check/`: review.ts, independent-review.json, all29 full inverses, explicit new30 absence inverse, additional-checks.json, source projection inverses, formatter/GSD logs and the exact pre-append report snapshot. The structural receipt is explicitly labeled static preservation only; this blocker controls the overall verdict.

The entire previous canonical report independently equals232108bytes/SHA256 `c9bcf1bdfaa2aff5f9cebd61c8979362be2ee192562d75ae4ab84b83ddf1ed73`. This verdict appends after that exact prefix, preserving all prior ISSUE/PASS/history bytes. Append closure records the new total/hash separately.

Candidate-index hashes independently authenticate all30 plan files, three frozen outputs and indexed planner receipts. Every original29 full-plan inverse reconstructs its captured bytes. Plan13 reverses only five added owners, the new submission context, one middle task, dependency30 and its wave; both old task XML blocks and all five commands are exact. Other original task/action/acceptance/verify/requirement/lifecycle bytes remain unchanged. Only14/15/16/17/18/19/20/25 have their original wave fields increased by one; their complete inverse is otherwise identity. New30's inverse is absence. Previous registrar/amendment/source-history bytes remain attributable.

The independently derived inventory is30plans/53tasks/25waves. Plan30 is wave16 after12;13 is wave17 after12/30. Later waves derive as14→18,15→19,16→20,17→21,18→22,19→23,25→24,20→25, retaining all old dependencies. Every dependency exists/precedes consumers; graph traversal finds no cycle. All23 shared-owner pairs are dependency-ordered, with no same-wave overlap. All30 GSD frontmatter/task structures pass; the inherited Plan02 independent-producer wave advisory is unchanged. All five CVLD requirements and D-01–D-10 original task coverage remain. New tasks name concrete files/actions/verification/done and critical links; the blocker is the semantic wire boundary, not missing task XML. **Dimension8: SKIPPED**, explicit nyquist_validation:false; previously resolved research questions are unaffected.

Twenty source/existence states,41 protected documents,3547 historical receipts and10 pinned read-only references match their captured hashes/existence. The original baseline still excludes active12 source/receipts/SUMMARY and root live metadata. Separate authorized accepted12 inputs independently match acceptance `f7ec6e6455ba1002196062d072d430459e53f19323e8d40d1558266ab8be6268`, SUMMARY `5e72ccfb241a8b3d935bb70dad0fda81d28ed317d3b3479bb3c61d76e476415a` and recorded source freeze `ac5639f959643cada49c16ce40912c9bfc95c6fb1e9ee7ac739b00b8a48c5713`. Root accepted two tasks/thirteen sources with all listed final commands exit0. This checker read the public configured history/genesis consumer/wrapper signatures; it neither recaptured nor renewed the active12 source hashes or execution tree. That dependency acceptance does not release30/13.

New30/amendment and independently extracted new13 sections pass GFM/frontmatter mdformat. All nine changed old plans retain their original formatter statuses/residuals: removing only declared changes from independently normalized candidates returns each entire normalized original. No original body was formatted in place. Scoped whitespace checks emit no diagnostics. All first validation/capacity-wording/section-formatting attempts remain unchanged.

The three full formatted field/default projections independently reverse to actual context.rs/network.rs/canonical_chain.rs sources; proposed line counts are503/580/264. The prefix-interface projection reverses to the entire original codec block file, retaining strict parse_block unchanged. Isolated rustfmt checks pass on those candidates and the normal-format interface sketch. These prove only formatting/control-placement feasibility, not API compilation or semantic completeness. In particular, the strict parser's unchanged behavior is precisely why the new endpoint needs the explicit wire/domain decision above.

### Sound planned authority and runtime links retained for revision

The split correctly supplies a real method-registry/normalizer/dispatcher path instead of a direct Rust call, binary catch-all or fake peer. The dedicated private bridge is initialized None in both actual context literals; only safe paired config/path or genuine-genesis-seal factories may bind a command from that same runtime. Generic/raw/fixture, arbitrary S and a pure core or canonical envelope cannot mint it. No Any/downcast/global registry or unrelated-field token is planned. Plan13 explicitly supersedes the old registrar-only context allowance; both real initializers now have declared ownership.

The node child is already registered; its parent remains unowned at627lines. Existing private authority helpers, concrete Fjall store access, shares_validation_history_store identity, actual header admission/KnownOnly publication and automatic-prune ordered flush helpers support the proposed serialized design. Factory capture must bind the actual store/network/params/incarnation from the genuine returned runtime. No arbitrary-runtime/store/bool constructor, no second lease or reopening a held genesis seal, and no public setter/Default is permitted. Non-Regtest genesis returns no command. Shape chooses a safe prover; it grants no authority. Actual accepted12 existing-Regtest full history and strict fresh/four-selector genesis gates remain.

The plan requires a real peer-free action with shell-owned current time, actual held HeaderStore context/CandidateRules and actual body/Coins/mempool processing. Real normalized body, headers, undo/counts/provenance, measured-disk Always flush and actual BASIC owner/turn/barriers must precede durable success. No raw checkpoint, u64::MAX disk fiction, background-worker assumption or second engine qualifies. Lock ordering must use private helpers under existing authority serialization, avoid public reentrant relocking, retain the command operation mutex across the whole action, release the inner guard for BASIC turns and recheck actual target/generation/receipts against external handle mutation. Failure after absorption remains operational with actual state retained, not a fictional rolled-back consensus rejection.

The planned opaque command/result fold requires actual external no_run doctests for both public-by-inference factories; no parent export is assumed. Those interfaces are uncompiled planning sketches. Full actual store/foreign-incarnation/context/duplicate/rejection/fault/reopen tests remain executor evidence. Full source≤628lines is a hard stop; the newly found representation work must be planned, not hidden in these owners or packed into an overlong file.

The other pin corrections are source-supported and remain mandatory: actual witness_commitment_index plus actual candidate Segwit rules; fill only the applicable missing whole-coinbase witness reserve with one zero32 item; leave existing witness/no-commitment/inactive cases unchanged. Prefix decoding deliberately permits bytes after one complete witness block while strict parse_block keeps Reader::finish. IsHex is nonempty/even ASCII hex with no whitespace. Params follow required count/type -1/-3 and optional ignored null/string dummy with real positional/named/mixed semantics.

Actual mining.cpp has **no -22 coinbase precheck**. Correctly serialized and appropriately PoW/merkle-arranged missing/wrong coinbase yields conclusive bad-blk-length/bad-cb-missing. Malformed hex/witness serialization is -22, accepted is null, true duplicate/inconclusive/conclusive reasons require observed processing, and operational is -25. No transaction -26 or invented duplicate-invalid mapping is allowed. These correct cases do not waive the amount/script wire-representability blocker.

Incremental fallible block/input/output/witness Vec growth is explicitly owned and justified. Keep MAX_SIZE, signatures, strict valid/malformed behavior and budgets unchanged; meaningful bounded4096 pre-fix capacity RED and final tiny-input/MAX_SIZE probes must observe actual Vec capacity, not merely EOF after a huge reserve. No source tests were run by this checker.

### Scope warnings and unchanged final gates

Plan30 has two tasks/six explicit owners: witness251, thin block554, codec block144, transaction296, existing node provider13 and one new substantive test child. The planned readable envelopes and cfg(test) observations are feasibility constraints only; actual implementation must preserve source caps and stop for any extra helper/export/test owner. Six is within scope, but the missing representation boundary may change that closure in revision.

Plan13 has three tasks/fourteen owners, below the15-owner hard boundary but above preferred scope. The middle task actually touches ten files, not just five new owners. Its ten files form one cohesive sealed endpoint integration—registry/normalizer/dispatch/private bridge/two literals/factories/tests—with old Task1 and Task2 preserved. These are two explicit warnings requiring root adjudication after the blocker is corrected. No task, helper, field or source owner may be silently omitted to obtain a lower count.

Preserve config-only acquisition, the exact old genuine OpenedCanonicalGenesis helper signature/test, four-selector genesis commands, early worker-before-config routing and actual hyphenated `CARGO_BIN_EXE_open-bitcoind` normal authenticated process. Keep default-off external networking, unavailable wallet/no files, valid/refused submission, complete tip/work/Coins/BASIC/provenance comparisons, graceful stop/reap/drop/restart, fully stopped direct durable observations and no second Fjall owner. Test-local serialized history must work before15. Rerun original factory commands on final bridge-wired source; intermediate results do not qualify later edits.

Global qualification remains Err;14 adds separate immutable implementation facts only. Original17 source review precedes guards;17/20 retain compact-fallback adjudication;18/19/20/25 and all full native/fresh pure-core coverage/Bazel/security/goal/lifecycle/normal-hook Git gates remain unchanged. Root alone owns shared metadata, saving, planning/source release and final acceptance. No Phase168 or whole-requirement completion follows.

Local AGENTS/sidecar/overrides and architecture/code-shape/testing/verification/Rust guidance informed domain-versus-wire boundaries, private ownership and minimal source scope. Active lessons remain fully loaded within8856bytes/2953conservative tokens; no archive or new audit trigger. This checker wrote only new ignored evidence and this append. It ran no Cargo/Bazel/application/build or source mutation, changed no candidate/shared metadata/guard and rewrote no old report or receipt.

```yaml
issues:
  - plan: "167-30 / 167-13"
    task: "30.1 / 13.1b"
    dimension: cross_plan_data_contracts
    severity: blocker
    description: "The Block-only prefix/command interface performs Amount and ScriptBuf domain validation during decoding, so pin-deserializable values/scripts cannot reach actual ordered consensus validation; blanket decode-to--22 mapping violates the promised pinned boundary."
    fix_hint: "Plan an explicit complete lossless submission-wire/domain seam with exact txid/merkle and error-order preservation, declare every necessary representation/parser/validation owner, and test negative/over-limit amounts, later truncation/witness and earlier PoW/merkle/coinbase precedence plus larger raw script cases. Do not clamp, fabricate a placeholder Block, map constructor errors directly to reasons or silently weaken existing strict/domain contracts."
  - plan: "167-13"
    dimension: scope_sanity
    severity: warning
    description: "Fourteen owners exceed preferred scope while staying below15; the real endpoint requires explicit root assessment."
    fix_hint: "Reassess the exact revised union after the boundary blocker is fixed; preserve all original tasks/commands and formatted628-line cap."
  - plan: "167-13"
    task: "1b"
    dimension: scope_sanity
    severity: warning
    description: "The middle cohesive endpoint wiring task touches ten files, not merely five new owners."
    fix_hint: "Root must adjudicate that full ten-file task; no hidden initializer/helper/export or incomplete final factory/process verification."
```

Recommendation: return the single concrete representation/decode-order blocker for a narrow formal planner revision, retain all passing static checks and frozen history, then independently recheck. No source execution is released.

## Independent formal wire-boundary revision | 2026-10-22 | ISSUES FOUND

**Candidate index:** `d115e19ddfca80400539ec000a1653e901d55b2fdf1682f0604e7fc71524f128`. **Verdict: one blocker, four scope warnings, one inherited metadata observation.** The raw-wire/shared-intrinsic and typed provider designs materially address the earlier representation/order findings, and the static checks pass. Plan33 nevertheless forbids the scanner change needed for its actual wide-output BASIC positive. Root agrees that this requires an explicit same-owner planning correction. No candidate/source/build/guard/shared metadata was changed, and no execution or phase acceptance follows.

Frozen principal targets:

| Artifact                       | SHA256                                                             |
| ------------------------------ | ------------------------------------------------------------------ |
| 167-13-PLAN.md                 | `530152cff2b39f4d9b669349833cd2b68624e62a92ee8c16a3bc2530487b532e` |
| 167-30-PLAN.md                 | `bf139ed5e275d5d5e0bac86b6a44471923e328431c58ed3bfd324b2658588137` |
| 167-31-PLAN.md                 | `01966390fd0de2bc2091a9b4587054f3b15b577f933312ec2361046047f93b97` |
| 167-32-PLAN.md                 | `af71f660f212dbb540df7c231968e9eea9301334ac2e802f36591d3cabf24b07` |
| 167-33-PLAN.md                 | `a27d335a7c4a7c6af2b91e7126f77b6baff0f2731bceaab30ed34787967ea91d` |
| 167-WIRE-BOUNDARY-AMENDMENT.md | `ce7bdc47866d37b6d11bd85ab099033027338ec31df677028d2db44e5ca5f5a2` |

### Blocker: BASIC rejects the wide output before the new decoder

Actual `packages/open-bitcoin-node/src/storage/fjall_store/filters/turn_inputs.rs:261–266` has `Cursor::script`: after reading its CompactSize length, it returns `BASIC body script bound` for len>10000. `body_work` invokes this scanner on input scriptSig and output scriptPubKey before `maybe_basic_filter_turn_body` calls the domain block decoder at line145. Thus switching parse_block to parse_serialized_block alone cannot process the promised valid10001-byte non-OP_RETURN output: body_work refuses first and the new decoder is never reached.

The frozen Plan33 interface says “Preserve every length/work check and charge.” Task1 says “replace only the three bounded body parser call/import seams” and preserve original body_work. Its threat register similarly preserves scanner limits without distinguishing this semantic restriction. Those instructions contradict its required exact wide-output BASIC success and the amendment's statement that wide non-OP_RETURN outputs remain filter elements. The source owner is already declared; a tenth/fourteenth owner or broader parser redesign is not needed for this issue. The task's permitted action and test/threat wording must explicitly cover the scanner correction.

The correction must preserve the **actual resource boundary**, rather than treating every script as executable BASE data:

- `maybe_basic_filter_turn_body:118–141` checks native body length≤4000000bytes and remaining-body budget before scanning or decoded allocation. A first candidate that cannot fit has its existing refusal, while a later candidate returns no work. Its conservative TurnWork reservation covers full body_bytes, cloned_bytes (32×wire bytes plus Block), script_items and script_bytes before calling body_work. Admission refusal must remain before scanner/decoder work.
- `Cursor::size` retains canonical CompactSize decoding, unchanged MAX_SIZE and checked usize conversion. `Cursor::take` retains actual-envelope/truncation checking; count/minimum-element bounds, witness flags, locktime and trailing refusal remain. These bounds and work/admission charges do not need to increase to traverse a10001-byte output.
- The extra10000 condition is a premature script-semantic gate inside this data scanner. Correct it explicitly for serialized body traversal while retaining the native/envelope/count/work limits. Do not filter the wide output or reduce its work charge to make a test pass.
- `body_work` traverses input scriptSig bytes but adds output script_items/script_bytes only for output scripts. Preserve that distinction: a10001-byte output must contribute its complete10001bytes to observed output-script work; input traversal remains bounded/accounted by the complete wire/body/clone reservation and must not be misreported as an additional BASIC output element.

Require an actual named positive through `maybe_basic_filter_turn_body`, strict serialized decode and genuine BASIC processing for a valid10001-byte non-OP_RETURN output—not only a decoder unit test. Verify exact observed output work, independent expected BASIC data/bytes, unchanged native/remaining/conservative-admission/MAX_SIZE/canonical/count/truncation/trailing refusals, and the full safe durable/proof/reopen controls already required by33/30/13. Keep the existing malformed-count test and real resource budgets. The already-owned turn_inputs.rs and declared durable test child can hold this correction and evidence; stop/replan if implementation reveals any additional owner or source cap need. No matching10000 semantic gate was found in the reviewed mempool snapshot scanner path; that observation is not a blanket audit claim about unrelated ingress.

This is a planning contradiction, not a failed implementation/test result. Source execution remains held. A narrow formal revision should change only explicit33 scanner/test/threat instructions and the corresponding amendment paragraph, preserving the complete current candidate inverse and every successful check below.

### Successful semantic closure retained

31 explicitly keeps Amount untouched, adds serialized ScriptBuf bytes while retaining bounded from_bytes, and supplies real BASE/default/v0 size and201-op guards, Tapscript exemptions and syntax-only OP_SUCCESS precedence. Its declared reconstruction owners cover P2WSH/Tapscript/redeem/nested-witness/signature-removal paths. Its central Coins predicate reaches actual connect/disconnect/overwrite, while BASIC and legacy sigops retain their different raw-script rules. This solves the original domain-constructor misuse in design without an unchecked Amount escape.

32 explicitly owns complete raw signed-i64/script/witness decoding, one mode-driven structural parser/serializer, borrowed views, exact raw txid/wtxid/merkle and shared typed/raw intrinsic checks. Old bounded wrappers retain their immediate Amount/Script combined-defect errors; strict serialized records call finish, and only submission prefix accepts a suffix. Incremental fallible block/input/output/witness growth has real capacity-observation RED/probes under unchanged caps. Raw outputs are checked negative→per-output MAX_MONEY→aggregate before safe addition; fallible domain conversion preserves actual scripts/witness. No clamping, placeholder identity, constructor-error-to-reason mapping or parallel consensus stack is authorized.

The root `e8a9b34c6ea97f37425d648639df8345b0b47c3112c470829905eb321e935ff8` refinement is actually incorporated: complete decode→actual owner/store/selected consistency→shared selected raw intrinsic→fallible conversion→genuine accepted duplicate→new-candidate header admission→earned CandidateRules→typed witness normalization→actual contextual body/Coins/mempool/durable/BASIC. No unadmitted candidate preview, generic raw commitment selector, missing-parent return or early duplicate precedes intrinsic checks. The supported Regtest intrinsic computation is witness-independent; before/after header/base txid/merkle/stripped sizes/structure/amount/prevout/sigop and result controls are mandatory before relying on that equivalence. Other selectors retain strict genesis with no command; no Signet/P2P qualification is implied.

Accepted genesis/descendant/stale/witness-only duplicates require actual accepted identity/provenance after supplied raw intrinsic checking, without descendant context first. Corrupt base body/merkle and malformed witness framing cannot duplicate. Actual missing parent is conclusive prev-blk-not-found; genuine cached permanent failure may produce duplicate-invalid. Inconclusive requires real processing evidence, and support/owner failures remain operational. Pin -22 decode, -1/-3 params, dummy null/string, strict hex, suffix, null/duplicate/reject/inconclusive and -25 distinctions remain explicit. The pin has no invented -22 coinbase precheck.

Root's `b84e1b6cf33a32e5283993df16dde5aef039793b84e1c5065459402cf2149c4e` addendum is incorporated with seven30 owners and task allocations2/5. The two network children provide typed genuine contextual header admission and LIVE false-mode body eligibility over the same current algorithms. Legacy lossy wrappers, cached true-mode, mutation/order and dynamic bad-version spelling remain exact. Hidden detailed header errors need actual external inferred fold/accessor compilation; the existing publicly reachable BodyEligibilityError remains typed. No Display parsing or parent627 export is assumed.

Permanent-failure classification must preserve ValidationError.result and stage. Raw merkle and contextual witness mutation/time/context/operational failures cannot become permanent failed state. Existing connect_stored_block automatically calls a broad failure marker; the new provider cannot safely use that path and then retrospectively classify. Necessary lower-level composition is available to the owned child through existing authorize_connect, prepare_authorized_connect, LifecycleProjectionPlan::prepare/validate and commit_connected_block_lifecycle_transaction, so gating before marking does not inherently need an unowned mempool_lifecycle edit. The final implementation must actually use a sound typed route, with main/nested tests and no guessed cached flag. KnownOnly/body availability/Ok NonExtending/Disconnected is not acceptance.

The private command is minted only alongside the actual safe runtime by config/path or genuine sealed-genesis consumption, never arbitrary runtime/store/bool/core/envelope. It binds actual store/params/incarnation with shares_validation_history_store and real serialization, avoids reopening a held seal, and uses actual receipt/provenance after absorption. Real measured-disk Always flush, body/undo/header/count durability and BASIC owner/turn/barrier/target checks remain. Post-absorption publication/flush/BASIC failures are operational with honest advanced-state evidence. External inferred submit/fold/error/doctest gates and all source≤628/stop conditions remain execution requirements, not compiled facts.

13 retains the dedicated private bridge with both literals None, safe paired factory-only binding, exact old genesis helper/test and all four-selector commands, worker-before-config and actual hyphenated normal Cargo binary process. Authenticated local submission, unavailable wallet/no files, default-off networking, test-local corpus before15, stop/reap/drop/no second Fjall owner/restart/next and complete state/BASIC comparisons remain. Its original two tasks/five commands are exact; only the explicitly attributed middle wire task/context changes. The new process matrix carries the raw amount/late syntax/PoW/merkle/coinbase, duplicate and valid-wide-body controls. The scanner blocker prevents claiming that final end-to-end path is fully planned yet.

### Independent static, inverse and preservation results

New ignored evidence is solely `.local/open-bitcoin-dev/phase167/amend13submission/wire-boundary/check/`. The independently authenticated index covers every33 current plan hash, the amendment and all indexed frozen receipts. All33 GSD frontmatter/task structures pass; requirement frontmatter covers the five roadmap CVLD IDs and D-01–D-10 action coverage remains. Source/threat/verification links are concrete, with the33 scanner contradiction isolated above. **Dimension8: SKIPPED**, explicit nyquist_validation:false. No deferred scope, global qualification flip or phase acceptance is added.

The graph independently derives33plans/61tasks/28waves and32 dependency-ordered shared-owner pairs, no cycle/same-wave collision. Pending chain is12(15)→31(16)→32(17)→33(18)→30(19)→13(20)→14(21)→15(22)→16(23)→17(24)→18(25)→19(26)→25(27)→20(28). Exact new scopes are31:11owners/3tasks4+5+2;32:12/3tasks5+3+4;33:6/2tasks4+2;30:7/2tasks2+5;13:14/3tasks5+10+5.

All30 failed-candidate plan baselines reconstruct fully. For each changed plan, an independent in-memory unified-diff applier checks every context byte and hunk count, proves the entire forward result equals the frozen current file, and reverses to the complete captured old file. Unchanged plans have identity inverses. Separate constrained13 reconstruction removes its one new wire context, restores only the middle task and old wave, returning the entire prior file; both old outer task blocks stay exact. Separate independent original29-root reconstructions restore their full earlier files, including the13 two original blocks/five commands. All prior30 commands remain in revised30, whose task/owner changes are explicitly attributed and moved obligations are preserved in31/32/33. New31/32/33/amendment inverse is absence. Old report/failed candidate/draft/first-format receipts remain intact.

Preservation counts are stated precisely:679 protected input rows comprise **649 non-plan live rows** plus **30 old-plan baseline rows**. All649 were exact before this append;30 full before bodies/inverses are authenticated, with10 intentional live plan deltas. Do not call all679 live paths unchanged. After append,648 non-report live rows remain exact and the canonical row is protected as an exact prefix. All50 before snapshots,69 observed source rows and the separately authenticated9 API input rows match; observations are not a new ownership/hash lock. Original active12/root metadata exclusions remain, and accepted12 summary/approval are separate final inputs. No source or guard hash was renewed.

The pre-append canonical file independently equals250119bytes/SHA256 `15fcb653967adcd2c809e2abadb2328171c7270c9b86e84c41410a074f048f98`; its232108-byte historical prefix retains `c9bcf1bdfaa2aff5f9cebd61c8979362be2ee192562d75ae4ab84b83ddf1ed73`. This result appends only after those exact bytes. Final whole-report hash is retained in append-closure.json.

Independent Markdown checks confirm new/revised30/31/32/33 and amendment strict0; independently extracted new13 sections strict0. The26 inherited whole-document formatter residuals remain. Ten changed-old normalized full inverses reconstruct original formatter output; original bodies were not normalized in place. The inherited05 literal owner-union residual consists of its two task strings' test-only expectation annotations versus the same two frontmatter paths. Those annotations/source paths are byte-exact and are not hidden new owners; this review does not normalize the old metadata or pretend the literal union check passes.

Two checker-reader setup failures were preserved separately: the first assumed nested before paths although this receipt stores flat basenames; the second overmatched a historical normalized-inverse filename as a live phase plan. Both stopped read-only before candidate validation, changed no source/candidate and were corrected with exact schema/path filtering. They are neither candidate failures nor test results. Final static reader and supplemental formatting checks have zero errors.

### Scope warnings and retained gates

Eleven31 owners and twelve32 owners exceed preferred plan scope; fourteen13 owners stay below15 but require explicit root assessment; the middle13 task honestly touches ten files. These are four warnings. The seven-owner30 typed API addendum is explicitly authorized for planning, not a new scope failure, and its main/nested tests/readable source/API compilation remain hard execution gates. All actual final sources≤628; no minification, suppression, hidden helper/export or extra owner. The33 scanner fix fits existing ownership but still needs formal wording and resource/positive controls.

17 source review remains before guard renewal;17/20 compact-fallback and18/19/20/25 full source/security/goal/native/fresh pure-core coverage/Bazel/lifecycle/normal-hook Git gates remain. Root alone owns metadata, saving and release. Local AGENTS/sidecar/overrides, core architecture/code-shape/testing/verification/Rust and active lessons informed this review; no archive or lesson changes. No tests, build, source change, Git saving, source-guard renewal or implementation/phase approval occurred.

```yaml
issues:
  - plan: "167-33"
    task: 1
    dimension: key_links_planned
    severity: blocker
    description: "BASIC body_work::Cursor::script rejects scripts over10000 before parse_serialized_block, while Task1 permits only parser/import changes and preserves every scanner length check; the promised valid-wide-output BASIC path remains unreachable."
    fix_hint: "Explicitly authorize the same owned scanner's serialized-script correction while preserving native4M, remaining-body, conservative TurnWork, MAX_SIZE/canonical/envelope/count/trailing checks. Require actual10001-byte non-OP_RETURN BASIC processing and exact output-script work/resource-refusal controls, not decoder-only tests; update33 test/threat wording and amendment with a full frozen-candidate inverse."
  - plan: "167-31"
    dimension: scope_sanity
    severity: warning
    description: "Eleven source owners across three cohesive tasks exceed preferred scope."
    fix_hint: "Root adjudicates the exact union; preserve all version/reconstruction/Coins controls and readable628-line source caps."
  - plan: "167-32"
    dimension: scope_sanity
    severity: warning
    description: "Twelve source owners across three shared-codec/intrinsic tasks exceed preferred scope."
    fix_hint: "Root adjudicates the exact union; no duplicate algorithms, hidden export/test owner or source-cap exemption."
  - plan: "167-13"
    dimension: scope_sanity
    severity: warning
    description: "Fourteen source owners exceed preferred scope while remaining below15."
    fix_hint: "Root assesses the final exact closure after the blocker is corrected; preserve original tasks/commands and actual process gates."
  - plan: "167-13"
    task: "1b"
    dimension: scope_sanity
    severity: warning
    description: "The middle sealed endpoint wiring task touches ten files, not only five newly added owners."
    fix_hint: "Retain explicit cohesive ownership, both initializers, all final factory/process verification and stop on additional scope."
```

Recommendation: preserve these semantic/static successes and all frozen evidence, revise only the identified33 scanner/test/threat scope plus explicit amendment paragraph, then independently recheck. Source execution remains held.

## Independent targeted BASIC scanner recheck | 2026-10-22 | PLANNING PASS WITH RETAINED SCOPE WARNINGS

**Index:** `a889d99a4d10b206fd53a8a7a974decc56619cfb186c0e4c9bab622a32bedbbc`. **Verdict: zero blockers; the same four scope warnings remain.** The same-owner scanner correction resolves the last formal planning contradiction. This closes the prior scanner blocker in the plan only; implementation, actual compile/tests/size measurements, source acceptance and release remain root decisions. No prior ISSUE/report/candidate bytes are rewritten.

| Revised artifact               | Exact SHA256                                                       |
| ------------------------------ | ------------------------------------------------------------------ |
| 167-33-PLAN.md                 | `1a07b25d145cc2915a1dbc363dc2f2c90b9a7755935d671d611f4b91f4e88046` |
| 167-WIRE-BOUNDARY-AMENDMENT.md | `eaa92e2ac25e1a20df8e8a529a909c5c7dfaa7b92059a02add4ed01c926e5bfb` |
| Root live planning release     | `d5ebf8603d68db580e8f55e92bac34d51bbe1f5802cc001be364f8b0d5ebb87b` |

### Targeted semantic closure

33 now explicitly permits removing **only** Cursor::script's premature len>10000 semantic rejection in the already-owned filters/turn_inputs.rs, in addition to the existing three decoder seams. Interfaces, Task1 behavior/action/acceptance/done, verification and threat wording consistently distinguish that data-scanner restriction from the real resource and interpreter/Coins boundaries. The source owner union remains six, with task allocations4/2; no extra source, limit increase or other scanner relaxation is granted.

The revised action preserves native4000000-byte body bound; remaining-body and first-candidate behavior; conservative TurnWork admission before body_work/scanning/decoded allocation; canonical CompactSize and unchanged MAX_SIZE; count-minimum and actual-envelope take checks; witness flags, locktime/trailing framing and all existing allocation/work bounds. Observed script_items/script_bytes must include every complete **output** script byte/item. Input scriptSig traversal remains bounded/accounted by the existing conservative wire/body/clone reservation and is not redefined as another BASIC output charge. The10000-byte semantic gate cannot be retained under a resource-budget name or replaced by filtering the wide output.

The unchanged phase167_serialized_body_durable command now must discover a genuinely valid selected-Regtest10001-byte non-OP_RETURN output through the **actual** maybe_basic_filter_turn_body/scanner and budgeted BASIC construction. It requires exact observed output ledger, body/txid/merkle/witness and independent expected BASIC elements/bytes, save/load, complete copied/original reproof and all-handle durable reopen. Direct serialized decoder success does not qualify. Existing10000/empty/OP_RETURN distinctions and native-oversize/remaining/singleton/pre-scan-admission/count/overflow/noncanonical/MAX_SIZE/truncation/flag/trailing/work-budget refusal controls remain. No fixed resource budget changes, source cap exemptions, second Fjall owner or fixture-to-runtime evidence promotion are authorized.

31 still owns real BASE/v0/Tapscript execution and the distinct Coins predicate; wide non-OP_RETURN bodies remain BASIC facts while excluded from canonical Coins/undo. Bounded Coin/undo/genesis decoders and the full history worker/deadline/source/lease/reproof chain remain exact. Task2's entire mempool XML and all automated commands are unchanged; no matching mempool scanner gate or policy change is introduced. Provider30 and authenticated13 still own actual command/full-flush/BASIC/reopen/next and normal product-process acceptance.

### Exact targeted validation and reused full review

New checker evidence is solely `.local/open-bitcoin-dev/phase167/amend13submission/wire-boundary/scanner-revision/check/`, including independent-review.json, two independently reconstructed whole-before inverses, GSD/formatter logs, exact pre-append report and append closure. Independent checks authenticate the new index, all33 current plan hashes, two revised target hashes, root release and indexed receipts. All32 other plan files match the previous d115 index byte hashes. Thus the comprehensive33/61/28 dependency/owner/semantic/inverse review from the preceding verdict carries forward without repeating unrelated exploration: no metadata, wave, dependency, caller or owner outside this correction changed.

Plan33's raw frontmatter, requirements, lifecycle, six owners, task file lists, all commands and complete Task2 XML remain exact. Independent constrained reversal restores old interfaces, Task1, threat model and verification, returning the **entire** old33 file SHA256 `a27d335a7c4a7c6af2b91e7126f77b6baff0f2731bceaab30ed34787967ea91d`. The new amendment retains the entire old prefix and adds only the explicit scanner-correction section; removing it restores full SHA256 `ce7bdc47866d37b6d11bd85ab099033027338ec31df677028d2db44e5ca5f5a2`. Both independently derived inverses match the saved inverse bodies/hashes. GSD schema/task structure and both whole-document GFM/frontmatter checks pass.

The prior d115 index and all its indexed historical receipts remain exact. All1408 captured prior scanner-revision history receipts,69 observed source rows and9 API input rows match their recorded hashes. The actual scanner is still unchanged SHA256 `81c363d712cf6aff21b81f5cbc691acdfc520fd17368c02a2fd4d1e3be45db79`; this proves no source implementation is being claimed. Root live metadata and active12 source/receipt capture exclusions remain. The earlier precise649-non-plan/30-old-plan preservation accounting, all original29 task/command and failed30 inverse chains,26 inherited formatter residuals and05 annotated literal-union observation remain attributable and unaffected.

The entire pre-append report is269048bytes/SHA256 `863fa39a5068d5fe3024ef56e78a7823ec9f2749e345d8000fe0604c4d15bcb0`. Its250119-byte prefix retains `15fcb653967adcd2c809e2abadb2328171c7270c9b86e84c41410a074f048f98`; its232108-byte prefix retains `c9bcf1bdfaa2aff5f9cebd61c8979362be2ee192562d75ae4ab84b83ddf1ed73`. This verdict is appended only after those exact bytes. Closure records the resulting total/hash separately.

### Retained scope and execution boundaries

Four warnings remain:31 has11 owners;32 has12;13 has14;13's cohesive middle task touches10 files. Root must adjudicate those exact scopes.30's seven-owner typed API addendum and external inferred detailed/live false-mode APIs, permanent-versus-mutation classifications and main/nested tests remain unchanged execution gates. No private-parent627 export, arbitrary runtime/store/boolean command, fake peer, cached true-mode shortcut, guessed result or post-absorption false rollback is allowed. All final actual sources must be readable≤628lines and every required source/privacy/API test must compile and pass; sketches and planning checks do not qualify.

The previously reviewed raw/shared-intrinsic/Amount/script/durable, intrinsic-first normalization, duplicate-before-descendant-context, real missing-parent/cached-failure, exact RPC/hex/prefix/error, private bridge and actual authenticated process controls remain in full. Global qualification remains refused;14 adds separate implementation facts. All17 source-before-guards,17/20 compact and18/19/20/25 native/fresh-coverage/Bazel/security/goal/lifecycle/normal-hook saving gates remain. Nyquist remains explicitly disabled. Root alone owns metadata, source release and final acceptance; this targeted PASS does not execute or release31.

Local guidance and the already-loaded standards/lessons informed this bounded recheck. No source, tests, build, Git, guard or shared metadata changed. Only new ignored checker evidence and this append were written.

```yaml
issues:
  - plan: "167-31"
    dimension: scope_sanity
    severity: warning
    description: "Eleven owners exceed preferred scope; exact union and628-line limits require root assessment."
  - plan: "167-32"
    dimension: scope_sanity
    severity: warning
    description: "Twelve owners exceed preferred scope; shared algorithms and explicit helper/test closure remain mandatory."
  - plan: "167-13"
    dimension: scope_sanity
    severity: warning
    description: "Fourteen owners exceed preferred scope while remaining below15."
  - plan: "167-13"
    task: "1b"
    dimension: scope_sanity
    severity: warning
    description: "The cohesive middle endpoint task touches ten files, not merely five new owners."
```

Recommendation: root may independently accept the completed formal planning revision and retained warnings, then make its separate next-plan release decision. All actual implementation/source/phase acceptance remains outstanding.

## Independent active31 FindAndDelete planning refinement | 2026-10-22 | TARGETED PLANNING PASS

**Index:** `64acb68e9fe336a072197a82b70d163cce34fd442ea6313f93b94d45703d13d7`. **Result: zero blockers and no new scope warning.** The four standing phase scope warnings remain attributable. This authorizes no source correction, build, source/phase acceptance or release; root alone decides the encoding-only continuation after the executor's current command completes. Other authorized31 work is neither read nor frozen by this checker.

| Input/target                | Exact SHA256                                                       |
| --------------------------- | ------------------------------------------------------------------ |
| Frozen preflight            | `5a7eb6f62477738131f925e6e0ad9bb8bd3a21cef77d200590240b54ad2d4a26` |
| Root planning release       | `ad1b737392b11e465e58c4ef7fc1ec47cf3bbbdb9fdf5bb01264337da1ff5c7b` |
| Revised31                   | `417ec1cdd6d2f4918bad1e011bb408ef46509519aa844f92c909aa88bef7840d` |
| New31-FIND-DELETE-AMENDMENT | `60d9eee03ec1dd15338453b2ab4e62317fda1c409230a4197b267d7568a60d95` |

### Pin-derived transformation and bounded exception

The actual free FindAndDelete in pinned script/interpreter.cpp:228–255 compares at the initial cursor, removes adjacent exact matches repeatedly at that cursor, then advances through GetOp to the next instruction boundary. GetScriptOp at script.cpp:383–433 skips complete push length/payload bytes and may partially advance its cursor on malformed syntax. FindAndDelete nevertheless retains the full remaining suffix from its saved pre-instruction cursor when an earlier removal occurred; with no removal it leaves the original script. This is observational transformation, not script execution or minimal-push normalization.

The candidate correctly reuses existing read_instruction_syntax, retains exact original instruction bytes and the complete malformed suffix from saved instruction start, and forbids read_instruction's520-byte execution limit. Thus target0130 inside `03 01 30 99 51` or the nonminimal `4c 01 30 51` remains untouched; real adjacent boundary targets are removed. The unchanged pattern encoder handles empty signature as nonempty byte00: `00 51` becomes51, while `01 00 00 51` becomes`01 00 51`, preserving pushed zero data. The pin's empty-pattern guard does not imply an empty-signature early return. Pattern comparisons do not inspect push payloads or length fields as new boundaries.

The explicit refinement includes all13 source-derived literal controls (the12-row matrix plus DER-plus-sighash-shaped embedded target), nonminimal PUSHDATA1/2/4, large>520 complete pushes containing the target plus a real boundary target, malformed length/payload tails, repeated deletions, retained instructions, empty signatures, wide reconstruction and sequential different-pattern calls. It keeps helper signature, encode_push_data and from_serialized_bytes reconstruction unchanged. No second parser/API/module/dependency or bounded fallback is introduced;183 current lines leave plausible readable test space, with actual formatted≤628 and compilation still mandatory.

This intentionally changes previously wrong small-script output. The new context/amendment expressly supersedes only original Task2's unchanged-small-script wording for these enumerated boundary/repetition/malformed-tail/empty-signature corrections, rather than claiming preservation. Other old behavior/tests and all original31 obligations stay exact. The pin's checksig/multisig call sites are BASE-only; unchanged WitnessV0 bypass, Tapscript dispatch and sequential caller composition remain contractual. No CONST_SCRIPTCODE/removal count, CODESEPARATOR, sighash/checker/NULLFAIL/flags/signature order/crypto/Amount/source-guard or other-owner expansion is allowed. These helper controls claim neither cryptographic spend success nor actual pinned-process/differential qualification.

Tests remain inline in encoding.rs under existing phase167_serialized_script_paths discovery. Require actual behavioral RED then GREEN, all old tests and every original31 automated command rerun on final source, complete failure/setup/discovery/hash/line evidence and final source review. Earlier active31 passes do not qualify the later helper change. The active executor retains sole Cargo/Bazel/build ownership and final SUMMARY; this checker ran none of those commands or source formatting.

### Exact targeted preservation

New evidence is solely `.local/open-bitcoin-dev/phase167/plan31/find-delete/check/`, including independent-review.json, the complete tag-removal inverse, explicit new-amendment absence inverse, GSD/formatter logs and exact pre-append report snapshot. Index hashes authenticate all33 current plans, new amendment and frozen planner receipts. Removing only `<find-delete-refinement>` returns the **entire** old31 SHA256 `01966390fd0de2bc2091a9b4587054f3b15b577f933312ec2361046047f93b97`. Raw frontmatter, all three complete original task XML blocks, eleven source owners and all six automated commands are byte-exact. The separately saved inverse body agrees; the new amendment inverts to absence.

All32 other plans and prior amendments are byte-exact. Inventory remains33plans/61tasks/28waves, with no dependency, task or ownership delta; earlier complete DAG/inverse/semantic successes carry forward. GSD plan schema/task structure and both whole-document GFM/frontmatter formatter checks pass. No old plan body was normalized or prior failed/checker receipt rewritten. The standing11-owner31/12-owner32/14-owner13/ten-file13 middle-task warnings and inherited05 literal annotations remain prior evidence, not new changes.

All1469 captured prior submission-planning history receipts match their recorded hashes. The only observed production source is stable encoding.rs SHA256 `de22fd4f0fa9d2f079eb714d52d1fba1431868b53ae1ff4f76c0b40b03c4fab1`; the other three source-window inputs are pinned interpreter.cpp/script.cpp/script.h. All four match. The actual existing helper remains unfixed, so no source implementation is being inferred. This checker did not read/hash/freeze active31's other ten sources, SUMMARY/execution receipts or root metadata; historical caller observations in the preflight are not converted into an active-source lock.

The entire pre-append canonical report independently equals277853bytes/SHA256 `976fb471b7284316c836689740b18ac4d9d64a6ea608edd922a39ceb35eae1bc`. Its269048-byte prefix retains `863fa39a5068d5fe3024ef56e78a7823ec9f2749e345d8000fe0604c4d15bcb0`, with older prefixes preserved. This verdict appends only after those exact bytes; append-closure.json records the new total/hash. Prior source/typed-wire/scanner BLOCKER reports remain unchanged and accurate for their frozen candidates.

Local guidance and already-loaded standards/lessons informed this narrow same-owner check. All final17/18/19/20/25 source-before-guards/compact/native/fresh-coverage/Bazel/security/goal/lifecycle/normal-hook Git gates remain. No source, build, Git, guard or shared metadata was changed. Root retains every release and acceptance decision.

```yaml
issues: []
```

Recommendation: root may independently accept this targeted formal refinement and decide the one-owner source continuation after current-command completion. The planning PASS supplies no implementation or phase approval.

## Independent Plan34 compact unit A planning check | 2026-10-22

**Verdict: PASS with scope warnings; zero planning blockers.** This is a bounded formal planning review of unit A only. It grants no source/build/Git release, implementation acceptance, requirement closure or phase approval. Root remains the release owner. Units B–E remain mandatory checked contributions before final Plan17 compact adjudication and guard renewal.

### Authenticated candidate and preservation

| Input                               | SHA256                                                                            |
| ----------------------------------- | --------------------------------------------------------------------------------- |
| Candidate index                     | `878b1765349d56d403895528c95c4bc45bf695e62db0d385f5b3b62ac8bb0033`                |
| Root planning release               | `2cc904a61ffb315eb9bcc806976334bfa832f56c683ad0645da0d6c5a4541009`                |
| Plan34                              | `d3d71c035bd00204fdecbd1939430c0afe7cd151c5214c858e933b934442d12f`                |
| Compact provenance amendment        | `0e0c75a5a4134b349c6969525e4f737269ab3bfa9fc5bcf7b5a1303d4fae7f52`                |
| Canonical report before this append | `35ded7165963ab934fb138a2f0762fab281479ec2d2223c63a25137b54b8491c` (285277 bytes) |

The independent checker recomputed 360 static assertions with zero failures, rather than relying on the planner's success booleans. All34 current plan/index hashes and five indexed receipt hashes match. All55 full before-document snapshots authenticate. Every original33 plan's task XML, including actions, acceptance and automated commands, remains exact. The other32 plans are wholly unchanged; Plan17 reverses exactly by removing only the new `167-34` dependency. New34 and the amendment invert to absence. The55 baseline documents include that intentionally amended Plan17 and the append-only canonical report; they are not claimed to be55 unchanged live paths. All other53 document paths remain exact.

The14 owner/existence states, seven nonactive API/pin observations,44 prior design-history receipts and selected release/matrix/consolidation remain exact. The two new child sources are actually absent, not zero-byte manufactured sources. Prior design history includes an older zero-byte projection artifact; preserving that historical artifact does not promote it to a live source baseline. The checker independently derives the full parent inverse by deleting exactly the three projected integration lines and restores all24347 original bytes. The formatted ignored parent projection is626 lines, SHA256 `d676c692eb6cbd48c471fbfead1f94f58a968a23915e9949f3a768748506a0d6`; the actual623-line parent remains unchanged.

The complete285277-byte pre-append report and older277853/269048/250119/232108-byte prefixes authenticate. Every earlier failed verdict remains intact. Active31 source/SUMMARY/execution, subsequently active32 source/execution and root live STATE/ROADMAP/todo/config/metrics/requirements are excluded from immutable preservation claims. Root's Plan31 acceptance and Plan32 execution ownership do not constitute this checker's implementation verification.

### Graph, task structure and ownership

Independent parsing gives **34 plans,65 tasks, maximum wave28**. Every dependency exists, the graph is acyclic, predecessors have earlier waves, and every shared source-owner pair is transitively ordered. New34 has no shared owner with any earlier plan; it depends on31 at wave17. Same-wave32 has a disjoint owner set. Plan17's added dependency fits its existing wave24 because16 at23 remains dominant. Existing deliberate wave gaps and lifecycle `167-2026-10-21T16-38-23` remain inherited, rather than being normalized.

Plan34's exact14-owner union equals its task file union; task allocations are5/5/5/3. All four tasks have concrete files/action/verify/done fields and measurable acceptance. The independent GSD plan-structure check passes. New plan/amendment Markdown checks pass with GFM/frontmatter extensions. Earlier body formatting and existing05 owner-annotation residuals remain inherited through exact preservation; this amendment does not silently normalize them. Nyquist is disabled, so Dimension8 is skipped.

### Source-grounded feasibility and complete bounded wiring

Existing ordinary producers and removals are in owned inventory_state.rs (`request_missing_blocks`, `handle_notfound`, `handle_block`, peer removal), header GetData production is in owned message_dispatch.rs, and compact/GetBlockTxn/full-fallback/timeout production plus compact clearing is in owned compact_download_state.rs. A private manager registry and common helpers can therefore cover every actual request emission and scoped release without another source owner. Registration must precede emission. New records obey configured/Phase94 capacity; existing same-peer/hash attachment or resend at capacity neither consumes another slot nor changes position. Real insertion order, removal/re-enrollment, reverse-ID arrivals and equal/backward clocks have independent expected sequences. No numeric-ID/BTreeMap/timestamp surrogate or untracked legacy emission is accepted.

The declared controls cover explicit/header/compact producer interleavings, initial and timeout fallback, new-at-cap suppression versus existing-at-cap resend, duplicates/reannouncement, NotFound sender/hash scope, disconnect and partial-only clearing. They compare affected order/set/partial state and unrelated peers/hashes. Ordinary-only requests survive compact-only cleanup. Existing strict age>60 timeout, clock saturation, reconstruction bounds, expired-map/status behavior and structural completion remain explicit local policy. ReceivedBlock and the existing volatile compact hook do not prove durable or eligible storage; E must later supply genuine stored-success all-request completion. A does not introduce unused later Consumed/scheduler authority.

Production context derives from the actual manager HeaderStore's exact already-admitted header, CandidateHistory::rules and held canonical_header_validation_context. Those actual public APIs exist and refuse unadmitted candidates; they are not a preview. Existing canonical09 reconstruction coverage admits its candidate before ingress. Missing/foreign/mismatched context must refuse before request/set/order/partial mutation; D still owns genuine new-header admission.

The explicit fixture API can be an inherent public method on the already exported PeerManager from its private owned child. Existing public data return types avoid another reexport/parent owner. The real Node route is owned and, under Rust child-module access, can inspect node admission scope, actual core binding, HeaderStore binding and selected fixture params. ANY canonical binding chooses the canonical path; a missing/mismatched store/context cannot fall through. A truly unbound fixture must supply actual selected fixture facts rather than a version/default activation boolean. The explicit manager fixture API rejects canonical HeaderStore binding. Existing pure completion and CompactDownloadInFlight fields/signatures remain unchanged; privately retained matching rule data binds actual BlockTxn request/partial use without a generic caller boolean.

The actual direct-call inventory matches the declared three peer fixture files, two Node fixture files and Node live ingress. The owned canonical fixtures arm raw state before their callers add canonical scope; migration to an explicit fixture method is therefore feasible without editing their unowned caller parent. The canonical actual09 tests use already-admitted real headers and remain a required regression. Stage1 owns ordinary tests/registrar and tracker; stage2 owns explicit peer fixture API/migration; stage3 atomically switches production context with Node and both remaining fixtures; stage4 closes header producers and final matrix. This avoids requiring a permissive fixture default at acceptance.

These are source-derived planning feasibility findings, not a compilation claim. Actual Node compilation and nonzero external fixture-API doctest discovery remain mandatory. Real RED/GREEN, all final commands on final source, unchanged request/compact/transaction/canonical/announcement regressions and strict network/node all-target/all-feature Clippy remain execution gates. All14 actual normally formatted sources must be\<=628 lines, with substantive modules and no minification, suppression, new dependency or hidden owner. Any extra required owner or629-line file requires a checked partition revision.

### Warnings and remaining gates

```yaml
issues:
  - plan: "167-34"
    dimension: scope_sanity
    severity: warning
    description: "Fourteen source/test owners are a broad cohesive unit below the fifteen-owner hard boundary."
    fix_hint: "Root must explicitly accept this scope; execute the declared stages and stop for checked revision if ownership expands."
  - plan: "167-34"
    dimension: scope_sanity
    severity: warning
    description: "Four tasks exceed the preferred two-to-three; their allocations are honestly five/five/five/three files."
    fix_hint: "Keep each compiling TDD stage bounded and retain final reruns; do not hide caller migrations."
  - plan: "167-34"
    dimension: scope_sanity
    severity: warning
    description: "The formatted peer parent projection has only two lines of headroom at626."
    fix_hint: "Restrict the parent to its three projected lines and enforce the actual628-line stop guard."
```

The four earlier standing scope warnings (31 eleven owners,32 twelve owners,13 fourteen owners and its ten-file middle wiring task) remain preserved and outside this bounded re-adjudication. A's three warnings are additional, not waivers. Root alone accepts them and coordinates compile-graph/build ownership after actual accepted predecessors. No checkout source, build, guard, shared metadata or Git operation was performed for this review.

AGENTS, the Bright Builds sidecar, overrides and architecture/code-shape/testing/verification/local-guidance/Rust standards inform the authority boundaries, normal module layout, actual testing and file caps. Both active lesson files were read fully:8856 combined bytes,2953 conservative tokens, within startup bounds. Evidence lives under `.local/open-bitcoin-dev/phase167/compact-provenance-plan/check/`; the exact final append hashes are recorded separately in append-closure.json to avoid a self-referential digest.

B–E still require actual32/30 interfaces and independent checked ownership/DAG review. Plan17 source-before-guards, compact adjudication,18/19/20/25, full native verification, fresh coverage, Bazel, security, goal/lifecycle checks and normal-hook saving remain mandatory. This PASS is planning-only.

## Independent Plan30 reopen-bound refinement check | 2026-10-22

**Verdict: PASS; zero planning blockers and no new scope-count warning.** This is an independent formal planning review of the frozen existing-bound refinement. No source execution, build, implementation/phase acceptance, guard renewal or Git saving follows. Root alone coordinates accepted33 revalidation,34 production/build and eventual30 release. All prior failed and successful checker history remains intact.

### Exact candidate and preservation

| Input                               | SHA256                                                                            |
| ----------------------------------- | --------------------------------------------------------------------------------- |
| Candidate index                     | `035b3d9bdc40da109bb25154a80264c91cbddb0b7ed2dac1c0138514c12c3556`                |
| Root planning release               | `b24c57dc8ac3fafeffcbf57ffeef43ef11bcd965c33a181a3b7e0e26b6a5796f`                |
| Effective Plan30                    | `fa5dafcbcc3693b82e4a653c8437a6333b0200241b4d974bce39be0ffa09a781`                |
| Reopen-bound amendment              | `3e6fb85a0eb2cf5d8d07e861e04250a2e3b39edbb06b7ce4ffb7bcf429781ff3`                |
| Original Plan30 whole inverse       | `bf139ed5e275d5d5e0bac86b6a44471923e328431c58ed3bfd324b2658588137`                |
| Read-only reopen preparation        | `254c525cf84dac8980e79467916068ed23b714f09f2afefbf50989391a1a5ef0`                |
| Read-only API readiness             | `461ab05ab19124524a81f07e278fa693fc211e189293d22d2961465266bddeb3`                |
| Canonical report before this append | `7ebf67880b34626139061ed2fdf1d5f6f6981a70dff854806812a1897f91e87b` (296152 bytes) |

The checker independently recomputed231 static assertions with zero failures. All34 current plan hashes, the new amendment, all five indexed receipt hashes and both readiness inputs authenticate. Both original Plan30 task XML blocks, including actions/acceptance/done and every original automated command, remain byte-exact. One coherent four-owner Task3 is added. The whole before-plan inverse is authenticated against its original snapshot; independently applying every hunk of plan30.diff to that snapshot reconstructs the complete current plan. The new amendment's inverse is absence, with no manufactured empty before-file.

All57 full before-document snapshots authenticate. Other33 plans are wholly unchanged; all other55 live baseline documents remain exact when intentionally changed Plan30 and append-only PLAN-CHECK are excluded. No statement treats all57 live paths as immutable. All1616 captured prior planning-history files, the seven existing source/existence states and selected release/readiness inputs remain exact. The two future test owners are genuinely absent. Existing observations are preservation evidence, not source acceptance.

The added records.rs owner is **observed only**, not a stable source input: the planner observed320 lines/SHA256 `2fbae9ce008398142c53c3318de0907d244d73e3f3b792223e80e9239328a45a`. This review does not assert that active33 source remains unchanged, freeze its bytes, or treat the observed getter sketch as its accepted API. All active33 six sources/SUMMARY/execution, root live metadata and34 test preparation are excluded from immutable claims. Accepted final33 source/SUMMARY/API and getter placement must be revalidated before future30 source release.

The exact296152-byte pre-append report and all specified232108/250119/269048/277853/285277-byte older prefixes authenticate. Only this independent result is appended; no prior issue/history bytes are rewritten.

### Goal-backward semantic check

**The missing live boundary is real.** Actual records.rs defines the sole MAX_HISTORY_BLOCKS=306 and refuses active-chain/header/index vectors over306. The retained proof uses the same constant for complete accepted traversal and checks header/index identity correspondence. Existing actual canonical header admission inserts earned lineage/parent/header entries without this total bound; KnownOnly publication and body-registry/batch/row limits do not impose it. The existing genuine307 fixture uses real header admission, body storage, managed acceptance and full persistence, then observes UnsupportedHistory. Its fixture flush/disk setup is legacy local evidence, not a substitute for the new command's actual disk/Always/BASIC path.

The new guard therefore belongs before header insertion or KnownOnly publication, rather than only before accepted body connection. A later body rejection can already leave a newly retained genuine header identity. Counting active height, body entries, blocks_by_hash or ScriptsValid alone would miss KnownOnly and stale/fork/failed retained identities. The plan explicitly requires actual held HeaderStore entries().count() and entry(candidate_hash) under the real command operation mutex AND held authority lock, so the count and membership correspond to the owner about to mutate.

**The interface fits the declared eighth owner.** A documented crate-visible inherent `FjallNodeStore::canonical_history_reopen_limit()` can be implemented inside the existing private records module and return its existing constant. FjallNodeStore is the same crate's actual type and is already exported; inherent method lookup does not require a records module import/reexport, parent modification, public limit carrier or ninth owner. Existing constant visibility/value and parser/proof checks remain untouched. The provider must call this associated getter; a copied production306 literal, caller capacity/boolean or unrelated body/batch limit is forbidden. Same-crate compilation and actual call-site use remain required execution evidence.

**Order and exceptions are explicit.** Complete raw decode and actual owner/store/selection consistency precede shared selected raw intrinsic validation and fallible Amount conversion. Genuine accepted identity/receipt duplicate handling follows those checks and precedes the new capacity guard and descendant context. Root genesis, active/stale accepted identities and structurally valid witness-only duplicates retain their actual earlier duplicate path; header/KnownOnly/cache membership alone cannot prove acceptance. Intrinsic-invalid same-header bodies still fail before duplicate/capacity.

The guard then allows a novel identity at305 to reach306, permits an existing genuine KnownOnly identity at306 to continue through unchanged contextual body/scripts/Coins checks, and operationally refuses a new main or side identity at306 or an already unsupported held graph. No insertion, KnownOnly publication, body retention, Coins/mempool absorption or durable effect may precede that decision. Existing membership is not an acceptance receipt. The stated resource policy can produce operational -25 before contextual missing-parent/version/time rejection when capacity is full; the amendment explicitly identifies this as bounded support rather than consensus or pin parity. It must not fabricate BlockValidationError, failed-cache provenance, inconclusive or duplicate-invalid.

The operation mutex does not freeze independent legacy/public handles or raw-store paths. The plan honestly preserves those APIs and the genuine307 refusal producer. Actual post-BASIC live/store/incarnation/target/freshness verification must recheck supported retained identity state before durable success, detecting external advancement. No preview HeaderStore, second Fjall engine, new authority, global networking limit or lifted parser/proof/row/time budget is introduced.

### Required verification and source feasibility

The new task requires actual safe configured/genesis-seal factories and genuine selected held owners. Getter/test compilation is setup, not behavioral RED. RED must show the real unguarded306->307 command admission/publication against expected pre-effect refusal, retaining actual state/reopen evidence. GREEN must prove:

- Real305->306 acceptance through Always persistence/BASIC, drop ALL handles and safe configured reopen with complete state/proof comparison.
- At306, novel main and side identity refusals with full accepted/known/header/body/Coins/count/undo/provenance/BASIC equality and byte-level original-manifest equality.
- Genuine KnownOnly body acceptance at306 without identity growth, full contextual checks, persistence/drop-all/reproof/reopen.
- Accepted root/descendant/stale/witness-only duplicates at306, intrinsic-invalid priority, genesis/KnownOnly/stale/fork counting and actual external-advance support/freshness controls.
- The unchanged real307 source fixture and its UnsupportedHistory refusal, plus all original Task1/Task2 commands rerun on the final eight-source freeze with nonzero named discovery.

The ignored23-line limit/guard and129-line semantic-test projections are formatting/interface feasibility artifacts. Their fixture helpers are planned real-history/store/manifest operations; they are neither compiled implementations nor executed acceptance. The full inherited typed validation/failed-state/live-arrival/authority/flush/BASIC/wide-output/API matrices remain required. No case is waived to make the new tests fit. Substantive helpers may be distributed between the two already-declared test owners; no hidden child, dummy proof, minification or suppression is authorized.

Every actual normally formatted owner must remain\<=628 lines. The provider/main-test/typed-child projections of\<=620/600/620 are tight execution budgets, not measured final closure. The added records getter must fit accepted33's actual final source. Any ninth owner,629-line file or missing actual API requires checked repartition before source. The unchanged original Task2 numerical seven-owner text is explicitly its original contribution; new context and plan-wide success text honestly specify eight owners/three tasks rather than silently rewriting old task contracts.

### Graph, scope and remaining gates

Independent parsing yields **34 plans,66 tasks,maximum wave28**. All dependencies exist, the graph is acyclic, predecessor waves are earlier, and every shared-owner pair is transitively ordered. Plan30's dependency/wave fields are unchanged. The new records owner is already owned by33, and the existing33->30 edge orders it. Other33 plans retain full byte identity, including Plan34 and Plan17's earlier compact dependency. No new same-wave overlap or scheduling waiver is inferred.

Plan30 now has eight owners and three tasks, within the checker's preferred count ranges; Task3 has four files and its task union equals the exact frontmatter union. All task fields remain concrete and complete. Strict GFM/frontmatter Markdown checks pass on current Plan30/amendment. Prior scope warnings and historical formatting/owner-annotation residuals remain inherited through exact preservation; they are not removed by this PASS. Nyquist remains disabled, so Dimension8 is skipped. Real external API compilation, complete TDD/final commands, actual source caps and runtime evidence remain unproved execution gates.

```yaml
issues: []
```

AGENTS, sidecar, overrides and architecture/code-shape/testing/verification/local-guidance/Rust standards informed the single policy owner, real locked state and behavioral proof requirements. The same fully loaded active lessons remain6898+1958=8856 bytes/2953 conservative tokens, within bounds; no archives or lesson edits. New independent receipts live under `.local/open-bitcoin-dev/phase167/reopen-bound-plan/check/`; append-closure.json records final hashes separately.

All17/18/19/20/25 source-before-guards, compact B–E adjudication, security/goal/native/fresh coverage/Bazel/lifecycle and normal-hook Git gates remain. The global qualification/default-network refusals and Plan13 authenticated process evidence remain unchanged. This review performed no source/build/Git/root-metadata changes and grants no execution release.

## Independent Plan34 runtime-consumer refinement check | 2026-10-22

**Verdict: bounded planning PASS with explicit scope warnings; zero remaining planning blockers.** This verdict binds revision4 only. It grants no source/build release, source/SUMMARY acceptance, guard renewal, phase acceptance or Git permission. Root remains the release and acceptance owner. The retained79pass/3fail Node compact result still blocks implementation acceptance until real attributable RED/GREEN, final commands and independent source/evidence review close WR-01.

### Frozen candidate and preservation

| Input                          | SHA256                                                                            |
| ------------------------------ | --------------------------------------------------------------------------------- |
| Revision4 index                | `1272b407af8ff0f43eeedcf81835510c0e81b70aa2633aa30ac283eb7fb571bf`                |
| Root planning release          | `bd76166293f9e36b4c6effc85b38febb2b24d8e65640d8a6fbf67449bf6e8716`                |
| Effective Plan34               | `658f45a1a99eb0134d906abcb44ad755ad01840be355810646157a2b99be0e62`                |
| Runtime refinement             | `f69f7006fbe5cbaccf7442234868c5ee7ddefcc87f8df24d94ded8f3ebfcbc8e`                |
| Original Plan34 whole inverse  | `d3d71c035bd00204fdecbd1939430c0afe7cd151c5214c858e933b934442d12f`                |
| Original14 final-source freeze | `59b229fde31acab5736a0e55f9d1d0873beb01171edfaf195c29d1051e73412f`                |
| Independent source review      | `c0670dddd290df68f0ab180e291454e1da33ee3d4910992f51b9a8582171eb18`                |
| Review source/evidence index   | `d364ab59fcc5879a806700bcda2fc0dcdd81111dbd9aec6503c73708ab62404f`                |
| Canonical report before append | `d50d0077dc004fef23c907dd31a0a3bcfa664bc84503966f0dc180700fe66619` (308164 bytes) |

All257 independent static assertions pass. Every current34-plan hash, new refinement/index receipt, original before manifest, cited superseded candidate/copy/projection receipt and fixed source/review/failure input authenticates. All58 full before-document snapshots authenticate. Other33 plans and the other56 live baseline documents remain wholly exact after excluding intentionally changed Plan34 and the append-only canonical report. All154 captured prior planning-history files, six fixed evidence inputs, six original read-only references and17 current source-basis states remain exact. Root active captures/audits/live metadata, other executors and active preparation are excluded; this check does not freeze their output.

The original four complete task XML blocks and all13 automated commands remain byte-exact. Task5 adds exactly three owners and seven commands, giving20 commands. All five task action fields now close before their acceptance fields; required files/action/verify/done/acceptance fields are paired. Original context/numerical scope references are explicitly superseded for final17-owner/five-task acceptance rather than silently rewriting old task contracts.

The whole inverse chain is independently checked: the original Plan34 snapshot matches revision3's full inverse; applying every original-to-revision3 diff hunk reconstructs its complete preserved plan; revision4 changes that plan only by inserting the missing Task5 closing action tag. Revision4's two full inverses restore the exact revision3 plan/refinement. The refinement's only revision4 change replaces the cyclic local-SUMMARY/global-gates sentence with the explicit local/global distinction. Its complete earlier inverse is absence. Frozen021, intermediate3bc and revision3 indices/documents/receipts remain preserved; neither the earlier receipt ambiguity nor malformed-action/cycle hold is erased.

The exact308164-byte prior report and232108/250119/269048/277853/285277/296152-byte historical prefixes authenticate. This append preserves every prior issue and report byte. New evidence is confined to `.local/open-bitcoin-dev/phase167/compact-runtime-plan/check/`. The checker's initial inverse-reader assumptions (singular filename/original versus immediate inverse) are retained as setup evidence, not candidate failures; the final check validates the complete actual inverse chain.

### Actual cross-component contract

The source-derived failure is concrete: manager timeout expiry retains the real request/order and emits an authorized GetData. Idle session.rs extracts hashes and calls the ordinary fresh producer; correct ordinary dedup returns no packet. Runtime global occupancy remains absent, so pending-response detection yields before the expected block. Message wakes can also carry actual merged timeout/initial-fallback packets, requiring the same runtime handoff. Passing13 selected jobs and694 network cases cannot override the observed supplementary Node compact79pass/3fail.

The final contract repairs the consumer rather than changing manager dedup/origin/order. One private block_reconcile helper preflights the entire actual packet batch for current target, retained same-peer block/WitnessBlock membership and UNIQUE hashes absent from the real runtime global set. It preserves packet/inventory type/hash/order and nonblock messages. Existing runtime hashes can resend at global capacity without another slot. Novel overflow returns existing typed ResourceLimit before any delivery/accounting insertion, with no partial batch delivery or silent filtering. Real error/outbox/disconnect teardown cancels the obligation; already expired partials and earlier genuine network effects are not falsely rolled back. There is no new retry queue, consumed-lifetime policy, ordinary-provenance promotion or cap/time increase.

Both Idle and Message paths consume the actual authorized packets. Message accounting precedes later ordinary missing-block scheduling, so fresh scheduling sees real occupancy rather than stealing its slots. The ordinary producer can reuse the accounting helper only for its actual already-created outbound and retains fresh-registration/dedup semantics. Foreign Idle targets keep explicit operational refusal before accounting/delivery. The actual runtime global unique-hash definition remains unchanged.

### Receipt and teardown closure

The earlier candidate's teardown-only wording was insufficient: the existing pre-manager release_inflight_for_message also erased a global hash on Block/NotFound before examining another peer's retained membership. The final plan explicitly removes that production release. It captures actual requested/progress facts and bounded receipt hashes before manager processing, then reconciles actual remaining membership across ALL peers on BOTH Ok and Err results before fresh scheduling. An unrequested-Block early branch performs no release. Sender NotFound preserves another peer's SAME hash. If manager processing releases sender membership and subsequently errors, post-manager reconciliation still prevents a stale global entry; relying only on a later empty departing-peer list would not work.

Teardown similarly captures actual outstanding hashes fallibly, performs real disconnect and uses a fresh post-disconnect manager snapshot to remove only hashes with no remaining requester. Same-hash other-peer and different-hash pending states require genuine manager registrations, not labels. Disconnect/snapshot failure conservatively retains unresolved accounting and surfaces the actual operational error; both original receive and cleanup errors must remain attributable. Receipt/hash lists are observation/bookkeeping, not a queue, acceptance proof or stored-success hook.

The old void utility has one real phase110 raw-counter fixture test with two unowned calls. Its unchanged low-level logic/signature can truthfully remain under the explicitly root-approved cfg(test) isolation. The test manually seeds raw counters and asserts fixture work unearned; it is not new production-policy evidence. Production uses the fallible shared membership helper. No dummy production call, synthetic outbound packet, ignored Result, broad visibility or allow(dead_code) is needed. Existing unowned caller tests remain exact.

### Verification, scope and layout limits

The17-owner union equals the five-task union, and Task5 lists exactly session.rs, block_reconcile.rs and the already-registered compact_timeout.rs child. Independent parsing gives **34 plans/67 tasks/maximum wave28**, valid dependencies, no cycles, and all shared owners transitively ordered. Plan10's existing block_reconcile owner is an ancestor through31; no new wave/dependency is necessary. Same-wave32 remains disjoint. New code can use existing child access, real manager snapshots/membership and ResourceLimit; no fourth added owner/export/registrar is assumed.

The original14-source failure cannot be retroactively called17-fenced. The new task requires a fresh real rerun of the three unchanged failures with all17 before/after hashes BEFORE the production fix. New tests must use actual transport/runtime/manager registrations and prove intact Idle/Message batches, unique-slot counting across duplicate inventories, existing-hash resend at cap, atomic novel-at-cap error plus actual teardown, same/different-hash preservation, requested Block/NotFound/error-after-release and unrequested-Block controls, target/membership refusal and Message accounting before fresh scheduling. All three original compact_timeout test bodies, unowned idle_sessions and original clock/message/strict-age>60/backward-clock expectations remain exact.

After GREEN, all20 planned commands, complete Node compact/runtime timing and full network regressions must pass on final17-source fences, with nonzero named discovery and renewed independent source/evidence review closing WR-01. Setup/compiler errors do not count as behavioral RED. Current failure logs, assertions and source bases are preserved. No source, Cargo/Bazel or application execution occurred during this review.

The original605/594 layout projections and later629/628/610 attempts are historical, uncompiled artifacts. They are NOT a measured final implementation of the selected helper isolation. The629-line attempt exceeds the actual limit and grants no exception. Reusing one membership-aware hash-list helper and isolating the legacy raw fixture is a source-derived same-owner feasibility path, subject to actual compilation and normal formatting. Every actual source must be\<=628; any fourth new owner or629-line final file requires checked repartition. No packed code, dropped test, dummy use or suppression is authorized.

```yaml
issues:
  - plan: "167-34"
    dimension: scope_sanity
    severity: warning
    description: "Seventeen owners and five tasks exceed the normal checker scope thresholds; root explicitly authorizes only this cohesive consumer refinement as an exception."
    fix_hint: "Keep Task5 at its three declared files, retain all old tasks/tests and stop for checked repartition if the complete readable implementation expands."
  - plan: "167-34"
    dimension: scope_sanity
    severity: warning
    description: "Final helper isolation has no compiled or measured final layout proof; a preserved exploratory attempt reached629 lines."
    fix_hint: "Enforce actual normal-format <=628 and real strict compilation/tests before source acceptance; historical projections are not a waiver."
  - plan: "167-34"
    dimension: key_links_planned
    severity: warning
    description: "Correct source invalidates specific current Phase123 guard needles; their future formal ownership remains mandatory."
    fix_hint: "After independent source adjudication, root must formally assign the exact guard/test paths before Plan17 renewal/full native/phase acceptance. No guard edit is authorized here."
```

### Mandatory later guards and non-cyclic acceptance

The concrete deferred paths are scripts/check-phase123-runtime-timing-evidence-integrity/checks.ts and scripts/check-phase123-runtime-timing-evidence-integrity.test.ts. checks.ts253–275 pins the lossy Idle extraction/re-registration;325 pins raw release before actual receive; the test fixture at575 pins that shape. verify.sh378–379 invokes both. This is a real pending gate, not a blanket stale-guard exemption. Root must formally expand17 or assign a coherent checked guard unit AFTER independent actual17-source review and BEFORE guard renewal/full native/Plan20/phase acceptance. Correct source must not retain dummy/dead needles to satisfy the old shape. No current guard owner is silently added to34.

Revision4 removes the dependency cycle: local Plan34 source/SUMMARY acceptance requires WR-01 closure, all20 planned commands and final independent17-source review. Later PHASE/Git/guard acceptance remains blocked by all mandatory global gates. Thus34's local handoff can precede30/20 without waiving native/fresh coverage/Bazel/security/goal/lifecycle/compact/Git checks. Remaining compact B–E still require actual32/30 reconciliation and separate checked plans; ReceivedBlock or volatile cleanup never proves durable stored success. Local timer/caps/default-off/qualification refusals and every prior evidence boundary remain.

AGENTS, sidecar, overrides and architecture/code-shape/testing/verification/local-guidance/Rust standards informed the real consumer boundary, scoped fallible bookkeeping, test isolation and source caps. Both active lesson files were read fully:8856 bytes/2953 conservative tokens, no archives omitted by an overflow shortcut. Nyquist remains disabled. Prior scope/history residuals are preserved; no source or phase acceptance follows from this planning PASS. append-closure.json records the final report and receipt hashes without a self-referential digest.

## Independent Plans15/16 context evidence allocation check | 2026-10-22

**Verdict: PASS; zero planning blockers.** This is a context-only allocation review, not source/build release, compilation/resource evidence, implementation acceptance or phase approval. All actual kernel negatives remain mandatory16 evidence before phase closure. Root retains release ownership and must revalidate future30/13/14 interfaces and physical adapter/datadir receipt binding.

### Exact targets and preservation

| Input                          | SHA256                                                                            |
| ------------------------------ | --------------------------------------------------------------------------------- |
| Candidate index                | `92f213a66c6b7a3c2c9294407dac2a27a653373cbcb69df980df491aa1d19a17`                |
| Root planning release          | `a662a25af4328eaac556fed75727974ccf808a4e5bd468ff5041f68ff1d5cff9`                |
| Plan15                         | `ed47715a27d868c355aec17d29a74a31bc0f67b5e141d98e948f611ccd281af7`                |
| Plan16                         | `c00a3c985c6808364049bf89bedbf6bfcd1c1a0eca280a46b1e9e53e02e8e821`                |
| Allocation amendment           | `890dcd134ab66d814d549f634611630bfd85252b9d340a0dfcb23638929d8926`                |
| Readiness/schema refresh       | `913ef21df8b661510c2865a1a9c83531bed0abb9c20ae2c237cc75b57c907713`                |
| Readiness basis                | `d78566f2b06d8a62604074b7433c98f38d83e437fdc18a51c3bd9ed4aebd49a3`                |
| Original15 inverse             | `27b06af848a1811e18e6071bc21e23425cc2e8735eeec3483f5305dfdf18f2ef`                |
| Original16 inverse             | `78543c63ccc78867cf1f9d9378e8841835d2b5bdd5a745f20faed1f97324870f`                |
| Canonical report before append | `fe9b521bcee2961c7336939abeb1cae15d59a4825e102e335f0a811994e4e3c6` (321959 bytes) |

All165 independent static assertions pass. All three targets/frozen copies, all34 current plan hashes and13 indexed planning receipt hashes authenticate. Removing exactly one tagged context fragment from each plan restores every original byte; independently applying both context-only diffs reconstructs each complete current plan. Saved full inverses match those originals. The new amendment inverts to genuine absence. No task XML/action/acceptance/verify/done field, any of the original four commands, frontmatter/requirements/owners/dependencies/waves or lifecycle changes. Other32 plans remain wholly exact, including current Plan34 `658f45a1a99eb0134d906abcb44ad755ad01840be355810646157a2b99be0e62`.

The exact321959-byte prior report and all eight captured historical prefixes authenticate; earlier failures/verdicts remain preserved. The nine selected nonactive source observations and immutable readiness/basis inputs remain exact. These are bounded captures, not a claim that all files referenced by old readiness are immutable or that future dependency sources are accepted. No active34 source was captured/read for this review; all17 active34 owners/SUMMARY/execution/root captures and live root metadata are excluded. The checker's strict end-offset assumption initially rejected a fragment ending exactly at the enclosing context close; that reader-only setup receipt is preserved, and the corrected check proves the complete whole-file inverses.

Independent graph parsing retains **34 plans,67 tasks,maximum wave28**, no missing dependencies/cycles or unordered shared owners. Plan15 remains eight owners/two tasks and16 five owners/two tasks. Their shared tests.rs/support ownership is already serialized by16's dependency on15. No task, wave, source owner, new capability or source-limit exception is added. New fragments/amendment pass scoped GFM/frontmatter formatting; original whole-document formatting is deliberately inherited rather than normalized. Nyquist remains disabled.

### Actual seam and authority observations

CandidateHistory::check at candidate_history.rs108 is public, accepts selected NetworkRules, candidate, parent and ancestors, and has no seed parameter. It runs linked branch/window checks before BIP34 context and yields GenesisLinked or CheckedSuffix, never IndependentlySeeded. Honest Plan15 pure missing-envelope constructor controls can therefore use actual source-bound rows and default rules without a fixture owner. Missing internal rows may fail earlier link/height/MTP checks; the report must record the actual reason/precedence rather than invent an isolated MissingRetarget or transaction-kernel result. Complete constructor success proves structure, not canonical authority or actual kernel execution.

The existing public check_seeded always marks IndependentlySeeded. The public Chainstate::from_snapshot_for_mechanical_fixture assigns mechanical work origin. Existing public stage_seeded_historical_block checks that origin, seeded history, bounded branch and exact actual snapshot tip/branch membership before invoking apply_connect_on_overlay and the same apply/transaction-context kernel. The stage's existing has_earned_work and the owner's observation must both remain false. No import/rebinding into a canonical product owner is authorized by matching scalar facts.

The source-derived creating-context boundary is real. transaction_context gathers the actual time-locked input's creating parent height. Live unseeded history extends from actual active ancestry when a distant creating MTP is missing; IndependentlySeeded deliberately skips that extension. At candidate201, parent200 suffix46..200 contains155 rows. It supplies parent MTP and expiry MTP56 (including its46..56 window), plus timely min-difficulty recovery144, while omitting the height1 coin's creating-parent MTP0. Thus a structurally valid seeded envelope can reach the real missing-creating kernel check. A complete-history companion changes only supplied coverage. With complete history, changing the consumed coin's creating-MTP metadata by plus/minus1 reaches the actual `creating-ancestor-mtp-mismatch` check; its current error is candidate-history context serialization, not an invented consensus code. Actual execution/logs remain required to establish these predicted outcomes.

Existing crate-visible genuine helpers and their module chain are callable from new sync tests without widening an unowned production parent. They open actual canonical genesis/Coins, consume its proof, admit actual bytes and expose complete accepted-state observations. Shared support can be declared through the already-owned tests.rs with visibility to crate::sync and substantive new orchestration in the already-owned support module; sibling canonical_history/tests cannot simply call its existing privately scoped drain helper. This is source-derived reachability feasibility, not a compile pass or permission to add an export owner.

### Full-fidelity allocation and evidence ownership

| Required control                       | Execution and observed evidence owner                                          | Necessary result                                                                                                                |
| -------------------------------------- | ------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------- |
| RG15-CREATING-LIVE-201                 | 15 Task2, phase167_ordinary_creating_history_live_extension                    | Actual canonical version2 time-lock spend at201 of a separately reserved height1 output exercises live distant creating lookup. |
| RG15-MISSING-MTP/RETARGET/RECOVERY     | 15 Task2, phase167_ordinary_missing_envelope_context, only pure unseeded check | Honest actual pre-stage constructor refusal with complete structural control; no kernel/durable claim.                          |
| Any seeded/fixture constructor variant | Existing16 Task1, phase167_historical_scripts                                  | Explicit independently-seeded/mechanical evidence, including missing anchor/recovery; never a15 fixture exception.              |
| RG16-CREATING-MISSING/COMPLETE         | 16 Task1, phase167_historical_scripts_creating_missing_complete_actual_kernel  | Actual same-kernel missing-MTP0 refusal and otherwise identical complete-history positive.                                      |
| RG16-CREATING-MTP-MISMATCH/MATCH       | 16 Task1, phase167_historical_scripts_creating_mtp_actual_kernel               | Actual plus/minus1 metadata mismatch refusals and matching same-kernel control.                                                 |
| Genuine branch MTP disagreement        | Existing15 ordinary execution                                                  | Independent branch medians demonstrably differ; actual held branch context and complete refusal state.                          |
| Corrupted durable source               | Existing15 safe-reopen controls                                                | Actual source/refusal class and original-manifest equality, never substitution for the creating-kernel negative.                |

Plan15's no-fixture/no-seeded rule remains strict for ALL ordinary execution and its negatives. Any older readiness suggestion of an actual fixture-context refusal under15 is constrained by the final allocation: a variant needing a fixture constructor/context or check_seeded executes16 instead. Offline mutations of the independent corpus checker are data/schema tests, not product fixture authority. Actual ordinary accepted/admission/connect/reopen evidence begins at canonical genesis/default selected rules.

Plan15 generates independently expected complete serialized0..301 (302 rows) and separate next302, exact funding/ledger/UTXO/work/BASIC/source facts and full positive/negative bytes. It may describe/provide the source-bound RG16 inputs; it cannot report their execution/completion. Existing16 Task1 must actually execute every kernel probe and retain raw logs/source/pin/corpus/context/work-unearned/full accepted-state evidence. Existing16 Task2 must finalize these Task1 observations as well as its original repeat/restoration results into the already-owned historical-results.json;16's SUMMARY owns the observed historical outcomes. No missing, skipped or failed probe becomes a pass through a pure constructor, a corrupted datadir, a scalar calculator or Plan15's completion.

Every original monetary/maturity/default-rule/script/lock/witness/sigops/difficulty/context/branch/refusal/reopen obligation remains in15. Every original16 exact exception, wrong forwarded flags/interpreter control, missing anchor/recovery, both repeated coinbases, overwrite/undo/physical restoration and policy boundary remains. The clarification moves no obligation outside this phase. All new16 names remain in the existing historical-scripts command; its separate historical-bip30 Task2 filter and both15 commands stay exact. Per-scenario observations must remain distinct even when multiple scenarios share one discovered test.

### Limits and remaining gates

The full source-bound live corpus must use independent formulas and complete expected Coins/BASIC/work sets, not product-to-itself comparisons or the existing Rust child generator as its expected oracle. The source-derived schema proposes exact hashes/PoW, subsidy/fees/maturity, MTP/time units and actual scripts; it has not generated or measured a new302-row corpus here. Physical RPC/CLI datadir identity and actual30/13/14 provider/process receipts remain unproved future prerequisites. Existing genuine helper visibility does not qualify those adapters.

The306 limit counts all retained known/header/index identities, including genesis/KnownOnly/stale forks, rather than active height. Isolate branch/body negatives so alternative cases do not accumulate into an unsupported307 source. Independently verify branch median disagreement before claiming it; changing only recent timestamps may leave the median unchanged. Preserve65536-byte row,4096-entry,512MiB physical/logical and30-second worker bounds with actual future measurements; accepted12 headroom is not a new resource pass.

Normal pretty JSON is correct: managed sourceExtensions excludes .json. No compact-row workaround or exception is needed. TypeScript/Rust actual normal formatting remains\<=628. The609-line sync/tests.rs and586-line canonical_history/tests.rs need narrow registration/delegated calls; substantive helpers belong to already-owned support files. No new source owner, copied authority engine or calculator is inferred if actual implementation fails to fit. Actual compilation, worker timing/counts and nonzero final test discovery remain execution gates.

```yaml
issues: []
```

There is no new scope-cardinality warning. Execution cautions remain: tight registrar headroom, unmeasured corpus/resources, actual adapter dependencies and mandatory16 kernel observations. Prior phase scope warnings and failed-review history are unchanged. Local AGENTS/sidecar/overrides and architecture/code-shape/testing/verification/local-guidance/Rust standards informed authority/privacy/test/layout checks; existing source/basis confirms the active8856-byte/2953-token lesson set remains bounded. No lesson/metadata/guard edits occur.

Source-before-guards remains mandatory. The two Phase123 guard/test paths still require later FORMAL checked ownership after actual source adjudication and before Plan17 renewal/full-native/phase acceptance; nothing here assigns or waives them. All B–E/native/fresh-coverage/Bazel/security/goal/lifecycle/compact/Git and existing global qualification/default-network refusals remain. Only root can release later source work. append-closure.json binds the final report/receipts separately, without a self-referential digest.

## Independent Plan17 Phase123 guard ownership check | 2026-10-22

**Verdict: planning PASS with scope/layout cautions; zero blockers.** This assigns later guard work only. It grants no guard code/renewal, source/build release, implementation/phase acceptance or Git permission. Before ANY guard edit or ownership renewal, root must obtain independent review of FINAL full Phase167 implementation source, close findings through their owning plans, complete ALL compact B–E and formally check the future E→17 dependency reconciliation. Accepted bounded34 is insufficient. Active30 source/evidence remains excluded.

### Authenticated candidate and preservation

| Input                          | SHA256                                                                            |
| ------------------------------ | --------------------------------------------------------------------------------- |
| Candidate index                | `db254e114f0aee4cd4bf030cac568b6d0541b51ccb6baa789d60c211026b6ef2`                |
| Root planning release          | `9ef81e13d224b025c93837dfc167c8d50e354482cae8081c33fbc9fc1fad85d2`                |
| Effective Plan17               | `9c9cdd44719ff0fbbabcedfe585d42137fe5ec21383fce092d01de7a717a97da`                |
| Ownership amendment            | `c84fd559450e1e640fe13a1522f26771cd95f52237824ebc5afe8e4a0cb36406`                |
| Original17 full inverse        | `308600d034949c57024be0a4fcbccb5d3c1b9411cab178d167ed0084f5cf8181`                |
| Accepted bounded34 receipt     | `66a6394181cee559802699fba654a2bc3a015f9109617ab257988fe795702c22`                |
| Independent34 CLEAN review     | `61f2c67f6d85b7c8405b2c589b97f1581884afec272d75096231119ac4718ca9`                |
| Canonical report before append | `514116e112ad7c14494df700bd3ee805261fee3cc7c3b7ae2cc8074e00330915` (335891 bytes) |

All206 independent static assertions pass. Both targets/frozen copies, all34 current plan hashes and11 indexed planning receipts authenticate. Removing exactly the three declared owner lines, one tagged context block and appended Task3 independently reconstructs every original Plan17 byte. The full saved inverse matches. Original Tasks1/2, all action/acceptance/verification/done text and both original commands remain exact. All other frontmatter, requirements, dependencies, wave and lifecycle bytes remain exact; other33 plans remain wholly unchanged. New Task3 has balanced fields and its action closes before acceptance. Its exact new commands are the Phase123 checker and explicitly pathed Bun test, giving four Plan17 commands total.

The two existing guard files have complete unchanged before copies. The new test-fixtures.ts is actually absent, including no dangling symlink or manufactured empty file. The amendment's inverse is its original absence. The selected39 before inputs and six nonactive source observations authenticate, with the intentionally amended17 baseline handled separately. The checker's initial absence check confused the newly created amendment with its before state; that reader-only failure receipt is retained and the corrected check verifies its original absence. No candidate/source failure is inferred from that setup mistake.

The exact335891-byte pre-append report and all ten captured prefixes (nine older plus current) authenticate. Every earlier failed verdict/history byte remains. All active30 eight source paths/SUMMARY/execution/evidence and root live metadata/captures are excluded from immutable facts; no active30 code was read or tested. Future13/14 interfaces are not qualified by this preservation check. No Bun, Cargo or native source check ran against changing code.

### Ownership, graph and execution gate

Independent parsing gives **34 plans,68 tasks,maximum wave28**, with valid dependencies, no cycle and every shared source owner transitively ordered. The new three paths have no other plan overlap. Plan17's total is12 owners/three tasks with exact5/4/3 allocations; task union equals frontmatter. Existing dependencies16/34 and wave24 are unchanged in the current declared graph. This is not proof that an unknown future E schedule fits: the mandatory formally checked E→17 metadata correction remains a prerequisite before execution. No comment or current dependency can waive it.

Task1's full final-source review and findings closure, completed B–E and future E→17 reconciliation gate ALL guard edits, including the original Tasks1/2. Task3 follows those original tasks and derives contracts from actual accepted final30/13/14 and compact implementation source. Current bounded34 source/review only explains why existing needles are stale. The plan does not require later Plan20 full-native/Git completion before its own local handoff, so it introduces no local-SUMMARY/global-gate cycle. Native/fresh-coverage/Bazel/security/goal/lifecycle/compact/Git checks remain mandatory later gates.

### Actual semantic scope and no stale-guard waiver

The current checker genuinely pins obsolete Idle hash extraction/re-registration and raw pre-manager release. Accepted34 instead has an intact actual packet handoff, Message-wake accounting before ordinary scheduling, whole-batch target/membership/unique-new-global-hash capacity checks and actual all-peer membership cleanup on receive Ok/Err and disconnect. The source review/accepted receipt do not turn that bounded behavior into first/later retry, consumed-lifetime or stored-success qualification.

The new Task3 explicitly requires actual production function/attribute/argument/call-order inspection and failing counterfactuals for:

- Lost/reconstructed manager packets, late Message accounting, changed inventory/nonblock order and ordinary fresh-registration substitution.
- Partial/per-item preflight, foreign targets, missing actual peer membership, duplicate charging, new-at-cap bypass and refusal of legitimate existing-hash resend at cap.
- Pre-manager raw removal, sender-only/clear-all membership, only-Ok cleanup, unreconciled error-path occupancy, unrequested-Block loss, same-hash other-peer loss and broken real-disconnect cleanup.
- Ignored cleanup results or loss of primary/cleanup/both error evidence; production calls or dummy liveness uses of the legitimate cfg(test)-only raw-counter utility.
- Wrong actual canonical startup factories/arguments/owners, broken sealed store/runtime/BASIC pairing, selector/refusal/worker ordering, wallet/default-off/claim violations and comment/literal/disconnected/test-only decoys.

Actual13/14 startup/source changes must be independently reviewed before their old open_ordinary_configured/genesis-only expectations are reconciled. No future factory signature, provider success or global qualification change is assumed. Only affected obsolete expectations may change; all existing timing, write/ack, outbox, cancellation, byte-progress codec capture, public-status, default-off and anti-overclaim negatives must retain their meaning. Complete synthetic corpus and live reviewed source must both pass. Mutation-test success is separate from actual product/source evidence. Real stale-checker/test RED must be retained before editing needles; digest churn or a source string label cannot replace review or behavior.

The existing entrypoint and verify.sh already wire both Phase123 commands. No verifier/entrypoint/constants/helper production edit is secretly assigned. If actual final interfaces or read-only parser APIs require another real owner, execution stops for checked scope revision. No dummy old call, broad claim exemption, skipped negative or blanket stale-guard waiver is authorized.

### Actual helper closure and source limits

The proposed three-file closure is source-derived. The current checker is381 lines and existing test627. Independently counting the contiguous completeFiles()/breadcrumb data region yields132 lines. Extracting that coherent fixture-only region and adding one import leaves approximately496 test-parent lines, matching the plan's headroom estimate. The region constructs literal corpus/parser data, JSON and breadcrumb arrays; filesystem/liveFiles/writeFixture orchestration and assertions stay in the existing test. The new conventional helper can own the coherent fixture path/type data needed by that builder and be imported by the test only. Production must not import or reexport it. Explicit Rust parser-input strings are TESTDATA, not executed foreign implementation.

Existing read-only source helpers expose non-code masking, balanced function bodies, attributes/offsets, initializers and call arguments/order. They are narrow source-contract utilities rather than a complete Rust compiler/AST proof. They support this existing guard family without an apparent new parser-owner requirement; actual guards must still reject the specified decoys, owner/cfg ambiguities and disconnected calls through meaningful adversarial tests. A function name's presence alone is insufficient.

The extraction is a layout allocation, not a formatted implementation/prototype or compilation result. All three actual source files must format normally and remain\<=628. No test deletion, packing, new generic framework, fourth owner or exception is permitted to make them fit. Every original assertion/negative behavior remains meaningful after justified source-shape reconciliation. Full guard RED/GREEN/final checks with nonzero discovery and reviewed-source digests are future execution requirements, not passes from this report.

```yaml
issues:
  - plan: "167-17"
    dimension: scope_sanity
    severity: warning
    description: "Twelve owners exceed the preferred small-plan scope; the added coherent guard task owns exactly three files."
    fix_hint: "Preserve5/4/3 allocations and stop for checked scope revision if another source/parser/entrypoint owner becomes necessary."
  - plan: "167-17"
    dimension: scope_sanity
    severity: warning
    description: "The627-line test requires the planned coherent fixture extraction; final readable fit and compilation are unproved."
    fix_hint: "Retain every assertion/negative and measure actual normal-format <=628 before accepting execution."
```

The declaration-level DAG passes; the unresolved future E→17 correction and full-source/B–E gates are deliberately mandatory execution prerequisites, not silently counted as completed dependencies. All requirements and original guard/claim obligations remain; source before guards is stronger than bounded34 acceptance. Existing15/16 context allocation and its actual-kernel obligations remain unchanged, with no source15/16 release inferred.

Local AGENTS/sidecar/overrides and architecture/code-shape/testing/verification/TypeScript standards informed scoped imports, testdata extraction, fallible evidence and source caps. The active lesson set remains the previously fully loaded8856 bytes/2953 conservative tokens; no archive or lesson edits. New context/task/amendment pass scoped GFM/frontmatter formatting; original body formatting is inherited. Nyquist is disabled. Only new ignored checker evidence and this canonical append were written; no source, guard, build, root metadata or Git changes/release occurred. append-closure.json records final report and receipt hashes separately.

## Independent Plan35 compact unitB planning check | 2026-10-22

**Verdict: planning PASS with scope warnings; zero blockers.** This verdict binds the corrected revision2 index only. It grants no source/build/guard execution, implementation/phase acceptance or Git saving. Plan35 remains held until root accepts actual30 and its final APIs/layout/behavior are revalidated. Active30's eight sources/SUMMARY/execution are excluded from immutable claims. Later C–E and final source-before-guards obligations remain mandatory.

### Exact candidate, layout correction and preservation

| Input                                | SHA256                                                                            |
| ------------------------------------ | --------------------------------------------------------------------------------- |
| Effective revision2 index            | `736f0fc1029030cb0b21b304c09bb5d1363d0a58f560ef1dc3d6e46db417efaf`                |
| Effective Plan35                     | `afaaf03a5f54fece6d9a55b791805ee5d26d6adaf2c5d8caaf857bb8b6cee6a3`                |
| Unchanged amendment                  | `eae31f0d7e4b35d3d2377e70ff451a91ac815bb48d5262c2c690de531842490a`                |
| Superseded original index            | `b69d693a0390f8927d3e54abb22d598d6b5a29e39ce4208f0db6f0476e287077`                |
| Superseded original35/layout inverse | `39251c1429611493433f4b87f73f5aabedd59f8b40bb0b55020ccf135fc85bad`                |
| Inherited before manifest            | `29be5ed61d1aebbd107d6b9081149324beed047992419c2a414cb93851126a4d`                |
| Canonical report before append       | `a79d99b76bc2e6f5a3b9967316de8355438ddd57b2a4fafd2803fd2db7780985` (347173 bytes) |

All176 independent static assertions pass. Both effective targets/frozen copies, all35 current plan hashes and eight revision receipts authenticate. The original b69 index, its frozen copies and nine original receipts remain exact. All34 earlier plan files match their whole before copies and hashes; every original task/action/acceptance/command/requirement/schedule/lifecycle byte therefore remains intact. No current Plan36 document has been published; ignored parallel resource planning is outside this review.

The original35 closing threat tag was accidentally a GFM table row. Its failed layout remains preserved. Independently applying revision2/exact.diff to the complete saved original reconstructs current35. Outside the STRIDE table bytes are exact; normalized table header/mitigation cells, all four task XML blocks, six task commands and frontmatter are unchanged. The closing threat tag is now standalone and balanced with blank lines, not a table cell. Current plan/amendment pass GFM/frontmatter formatting and task/schema structure. This review does not issue a final PASS for the old b69 identity.

The complete planning inverse removes only new35 PLAN and its amendment, restoring actual absence. The layout inverse separately restores all original35 bytes. Nine stable source/existence entries and12 selected API/pin/planning references authenticate. Three new source files are actually absent, with no empty placeholder or dangling symlink: network/test_block_commitments.rs, consensus/block/mutation.rs and mutation/tests.rs. The two held30 block/witness observations are metadata observations only: they were deliberately NOT compared as immutable live source or promoted to accepted APIs. No active30 source copy/hash renewal occurs here.

The exact347173-byte report and all11 captured prefixes (current plus ten earlier) authenticate. All failed verdict/history bytes remain preserved. New evidence is confined to `.local/open-bitcoin-dev/phase167/compact-unit-b-plan/check/`; append-closure.json binds the final report/receipts separately. Root metadata, active30 source/SUMMARY/execution and future resource planning remain untouched.

### Graph, owner closure and staged commands

Independent parsing yields **35 plans,72 tasks,maximum wave28**. All dependencies exist, the graph is acyclic, every shared source pair is transitively ordered and no same-wave owner overlap is introduced. Plan35's exact dependencies are30/32/34. Wave21 is the explicitly selected gap after current30 wave19, not an invented dependency on unplanned36. A later resource proposal may change metadata only after its separate review/root acceptance; the current34 earlier plans remain exact here.

The exact11-owner union equals the four-task union, with4/1/4/3 file allocations. Task1 owns block.rs/witness.rs plus the real new mutation module AND its tests-child registrar/initial tests; Task2 expands that same child, avoiding an absent child between compiling stages. All task fields are paired and actions close before acceptance. Six focused task commands and three explicit final cooperative checks are present. Predicate/witness, pure compact, peer/node compact and final consensus/network/node format/strict-Clippy/full-test gates must discover intended nonzero tests and run on final owned source. Missing-API compilation is setup evidence, not invented behavioral RED; existing fixture coherence must show actual failed assertions before correction. No command/build was run during this planning review.

### Exact pinned predicate semantics

Pin validation.cpp4277–4355 and4468–4496 supports the plan's precise order:

1. Exact stripped-txid Merkle mismatch or real duplicate-pair/subtree mutation returns true first. Ordinary odd padding is not itself mutation.
1. Empty or first-noncoinbase bodies immediately return whether ANY transaction is exactly64 stripped bytes. With no64 they return false and skip ALL witness checks, including otherwise invalid witness data under selected witness=true. Empty with matching zero Merkle returns false. A64-byte FIRST COINBASE does not use that branch.
1. Only a first-coinbase body reaches selected witness malleation: expected commitment uses the highest matching output, one32-byte reserve and zero coinbase witness-root leaf; missing/mismatched reserve/root or unexpected witness classifies mutation. With witness checking false, commitment presence does not authorize witness data.

The new predicate is not full CheckBlock. Existing32 check_submission_wire_block/check_block_view additionally enforce header/PoW/length/coinbase/transaction/monetary/sigops conditions and cannot substitute. Base/witness facts remain exact serialized data, without Amount conversion, normalization or a new authority owner. The matrix includes bad intrinsic facts as distinct from mutation, bytes/header/hash immutability,63/64/65 and later64 boundaries, first-coinbase64, Merkle priority, raw/domain equivalence, selected witness variants and highest/trailing commitment behavior. Expected classifications are pin-derived; shared hashes may construct fixtures but production result getters cannot supply expected answers.

Actual32 provides the required public BlockView/TransactionView, raw WireBlock/transactions, stripped/witness encoders, txid/wtxid views and duplicate-aware Merkle algorithm. Generic crate-visible is_coinbase is available within consensus. Public wrappers can live in already-public block.rs without a crate-root/reexport owner. The selected witness bool remains factual input, not a canonical seal; laterC must derive it from actual held rules.

The shared witness core must retain separate Encoding(CodecError) and Invalid(BlockValidationError) channels within block-private scope. Ordinary validation maps encoding through its existing mapping and preserves semantic reason/result/debug fields; mutation maps only semantic malleation to true and propagates encoding Err. A catch-all `.is_err()`, string recovery or true/false fallback would violate the contract. The bounded borrowed oversized-output-count test can exercise the existing CompactSize rejection before output iteration or attacker-sized allocation; normal bounded encoder setup allocations are not claimed absent. No giant vector, duplicate decoder/hash tree or lifted resource limit is needed.

Current30's selected normalizer remains a separate existing provider operation under earned CandidateRules. B cannot invoke it to repair an invalid compact body. Generalizing shared witness fact readers must preserve ordinary wrapper diagnostics/order, highest-output selection and30 normalization behavior. Current30 is partial/resource-held: proposed reuse is feasible by actual signatures, but actual accepted30 revalidation and compilation remain mandatory before release.

### Coherent fixture construction and real caller boundaries

The network helper is cfg(test)-only in its owned lib.rs, crate-visible to network unit fixtures and absent from non-test dependency builds. Node cannot import it. Actual node/network/tests.rs currently has only a coinbase-only build_block; factoring its OWN build_block_with_transactions from the existing Merkle/mine path is explicit. The older design's assumption that this parent API already existed is corrected, not silently blessed. Existing unowned child builders are read-only references and grant no edit owner.

Every changed positive must begin with its COMPLETE actual body, including missing/matched transactions and preserved witness stacks. Build reserve/commitment through the ONE shared witness-root fact algorithm and existing double-SHA256; recompute base txid Merkle; mine the final peer header where PoW was claimed; only THEN derive block hash, selector and compact short IDs. Returned missing/matched transaction identities stay actual. Seal-after-selector, incomplete-body Merkle or clearing witness stacks cannot count as success. The new public witness-root fact wrapper avoids a copied test hashing algorithm.

Pure fixtures need no invented PoW/consensus acceptance claim. Existing arbitrary-root mappings, suppression, collision, misbehavior and incomplete-response cases retain their structural/negative class. Unowned pure children consume the corrected common helper or remain deliberately structural; B does not add production classification or change completion signatures. Any genuinely required caller edit outside the11-owner union triggers checked replanning. Full compact regressions are required to discover such gaps.

The network helper/fixture parents and node parent have measured current scope; their future budgets (helper\<=160, mutation\<=180, matrix\<=560, node parent\<=610, thin block wrapper\<=525 and witness\<=550 after actual30 revalidation) are estimates, not compiled fit. All actual normal-formatted files must be\<=628. No minification, hidden helper/export/registrar owner, test/witness deletion, suppression or cap/deadline increase is allowed. New Rust source comments need pinned breadcrumbs;19 alone owns their shared manifest reconciliation.

```yaml
issues:
  - plan: "167-35"
    dimension: scope_sanity
    severity: warning
    description: "Eleven owners exceed the preferred small-plan scope, though each task has at most four files."
    fix_hint: "Root must explicitly accept the cohesive scope and enforce the exact union; stop for checked revision if another caller/API owner is required."
  - plan: "167-35"
    dimension: scope_sanity
    severity: warning
    description: "Four tasks exceed the preferred two-to-three; the contract-first split is deliberate and owns the initial tests registrar."
    fix_hint: "Keep4/1/4/3 stages compiling and retain full focused/final reruns, without hiding an absent child or setup failure as behavioral evidence."
  - plan: "167-35"
    dimension: scope_sanity
    severity: warning
    description: "Final generic witness refactor/layout depends on currently unaccepted30 and has no compiled size proof."
    fix_hint: "Accept and revalidate actual30 first; measure actual normal-format <=628 and preserve ordinary diagnostics/normalizer tests before source acceptance."
```

### Remaining gates

Only B's data/predicate preparation is planned. C must integrate selected classification before ReceivedBlock; D owns first/later retry, consumed lifetime, service-based witness fetch and genuine header admission; E owns actual successful stored-body completion. No primitive/fixture pass can claim those outcomes, scheduler parity, broad networking, daemon differential qualification or funds/service safety. Existing local strict>60 expiry, caps/reconstruction/time/row/physical/logical bounds and default/global qualification refusals remain.

Plan17 guard CODE stays held until ALL B–E, formal future E→17 dependency reconciliation and independent focused FINAL full Phase167 source review/findings closure. The current declaration-level DAG is not that future correction. Native/fresh-coverage/Bazel/security/goal/lifecycle/compact/Git and18/19/20/25 gates remain mandatory. Resource36 stays ignored/unpublished until this closure and root adjudication; this PASS neither formalizes it nor releases35.

AGENTS/sidecar/overrides and architecture/code-shape/testing/verification/Rust standards informed pure factual interfaces, shared error handling, test-only modules and file caps. The previously fully loaded active lessons remain8856 bytes/2953 conservative tokens. Nyquist remains disabled. Only new ignored checker evidence and this append were written; source, builds, guards, metadata and Git were untouched. Root alone accepts warnings and decides later release.


## Independent Plan36 and Plan30 resource projection check — revision3 | 2026-10-22 16:39 UTC

**PASS, planning only.** 497 independent static, semantic and retained-failure evidence checks pass; zero blocking findings. The confirmed revision2 production-consumer blocker is preserved and resolved in this candidate by the real Task36.4 command integration. This verdict grants no source/build release, Plan30 implementation acceptance, phase/requirement acceptance, guard renewal or Git authority. Root alone adjudicates the scope warnings and releases any later work.

The effective publication index is `resource-projection-plan/publication/revision3/publication-index.json`, SHA256 `423c73e913d459b4748cefa85dc789a900c3231edc563ba7dd9b0b0d96c4b72e`. Actual Plan36 is42,628bytes/SHA256 `510e0809e636e4e006541fcc6bb96ca8b16f96abe2929fcedaadf139e86885f3`; resource amendment is28,047bytes/SHA256 `4f493e085b656e6573aa95a18d2d71425d559f304578aa022f61d9cd25b8b5a4`. Actual Plan30 remains49,309bytes/SHA256 `48d9fc2f96886d87a0fc9fea47c77300dd5e76e9a33997103691ddaa0c4d6dfe`. Superseded d162/6e09/ab180/02e1 candidates, failed setup/projection receipts and the immutable blocker `b1303edcc254430cfdf129755d52c92841b4f61434e90889a9cc5d33e2e5d5c9` remain historical evidence. No old failure or verdict was rewritten.

### Preservation, structure and dependency closure

The independent checker authenticated74 frozen ignored references, all1,715 first-party file presence/bytes, all21 declared source/existence observations and all six true absent new source children. All24 accepted plans remain whole-byte exact. All other35 plans remain exact relative to the pre-revision3 candidate; relative to the original35-plan publication baseline there are exactly ten intentional old-plan deltas: bounded30 frontmatter/context plus nine successor wave-only edits. The remaining25 original plans are whole-byte exact. Every original35 task XML block and automated command is unchanged. The full original forward/inverse restores every old35 body and both new planning absences; the independent revision3 forward/inverse restores exact revision2 documents and reconstructs both current documents. No actual source/planning inverse was applied.

Raw actual inventory is36 plans,76 tasks,29 waves. Plan36 is wave19 with12/33/34 dependencies;30 is wave20 with12/33/36;35 remains accepted exact wave21. No dependency cycle, same-wave dependency, unordered shared owner or hidden new registrar was found. Plan36 has15 distinct owners, task allocations5/5/5/4, with3 shared owners ordered before30; the combined source union remains21. Plan30 retains9 direct owners and its explicit five-file effective Task3 allocation. Its original3 task XML blocks and14 commands are byte-exact; both additional resource selections and the full broad matrix remain mandatory. The inherited Plan05 annotated file-field union residual remains unchanged. The literal original30 Task3 records owner is deliberately superseded by the explicit effective allocation; this is documented, not a falsely exact literal union.

Plan36 Task1/Task2 XML is exact. Its original11 commands are retained byte-for-byte; strict Clippy moves from Task3 to Task4 after real consumers exist, and the exact original probe selection plus new guard smoke yields13 final commands. Every original command must rerun on the final15-source freeze. Independent GSD structure checks for36/30 and configured GFM/frontmatter Markdown checks pass. All task/action/verify/done delimiters balance. Nyquist dimension is skipped because workflow.nyquist_validation=false. Local AGENTS/sidecar/overrides, architecture/code-shape/testing/verification/Rust standards, GSD revision-gate guidance and the complete8,856-byte active lesson set informed the review.

### Semantic and actual API closure

The single actual Snapshot/catalog/sequence observation remains bounded to the same nine namespaces and unchanged native policy:4,096 namespace-plus-row units,536,870,912 total name/key/value bytes,4,096-byte keys,65,536-byte nonempty authority values and306 retained identities. Normal cooperating-owner checks do not claim arbitrary raw-writer serializability, future allocation support or a future30-second replay proof. Genuine complete admission Coins plus the real staged overlay/counts/chain/undo feed exact existing codecs; segmented families use actual prepare/owns, retained orphan undo and unrelated rows remain, and no second consensus/Coins calculator is introduced.

BASIC forecast and achieved append share pure checkpoint/lock construction and actual record encoders. Disabled preserves actual rows; Enabled consumes genuine prepared inputs and actual predecessor/progress/protection. Budget-before-buffer, work/proof/fault/owner/retention checks stay in the actual writer. Forecasts cannot create a receipt, fence, append proof or authority. Nominal pub(crate) opaque types and called methods with private fields, external inferred compile gates and the boxed store-bound FnOnce remain mandatory; neither hidden-path inference nor boxing bypasses compiler privacy.

The earlier liveness cycle is resolved in the design by actual production calls: submit → private workflow → inventory/early projection → genuine prepared-stage/BASIC projection → store-bound achieved closure and physical check. Shared row, undo and BASIC builders also serve their real writers. All new crate-visible seams must have meaningful production consumers before unchanged strict normal-library Clippy; cfg(test)-only, dummy, allow/expect suppression or visibility solely for lint are forbidden. Signatures and size sketches are feasibility, not compilation evidence.

Existing publish_admitted_header_identities writes records only. The actual available persist_validation_header_snapshot method captures/persists the held trusted header/index snapshot. Task4 explicitly calls it directly under the existing held network lock after early KnownOnly publication, with no public-handle relock. These two writes can fail separately and operationally; no atomicity/rollback is claimed. Full projection captures a NEW inventory/catalog/sequence baseline after ALL those own writes. Final achieved comparison likewise uses a fresh stable actual capture after Always/BASIC, not an original-sequence equality that would reject every valid write.

Actual authorize_connect requires earned body eligibility AND blocks_by_hash.get(hash)==Some(block). The candidate correctly preserves real volatile normalized body/eligibility registration before prepare. A later resource refusal can retain those enumerated volatile deltas plus actual earlier KnownOnly/header writes, while accepted tip/work/Coins/counts/undo/mempool/receipts/BASIC remain unchanged, no body disk row is written, and safe old-state reopen must succeed. Availability is not Stored/accepted proof. The earlier new-at306 guard still requires its original whole-state/manifest equality. Full native projection precedes save_block/lifecycle commit/absorption; actual achieved logical/physical/freshness checks precede public success; honest late errors preserve/report real advanced state. No new authorization API, fake clearance, second engine or rollback is assumed.

The exact two observed Accepted→refused-reopen probes and both8-source before/after copies/raw log are authenticated. Active shared assertions must become genuine operational-refusal/no-accepted-state-change/all-handle-drop/safe-old-reopen GREENs before36 acceptance, without deleting or changing named bodies/identities/counts. Positive safe-factory command smoke in both BASIC modes must actually Accept, pass complete achieved-map equality, drop all handles, safely reopen and connect next. This prevents blanket stale refusal from masquerading as a fix. Mature oversized undo, full BASIC/near-budget/fork/fault/late-error matrices and all14+2 final commands remain30 obligations;36 does not partially accept30.

### Warnings and retained gates

```yaml
issues:
  - plan: 167-36
    dimension: scope_sanity
    severity: warning
    description: Fifteen unique owners reach the large-scope threshold; root explicitly selected this cohesive real-consumer slice.
    fix_hint: Preserve the declared15-owner boundary and staged5/5/5/4 allocations; root decides release.
  - plan: 167-36
    dimension: scope_sanity
    severity: warning
    description: Four tasks exceed the preferred2-3; Task4 closes the genuine strict-library consumer cycle.
    fix_hint: Complete all4 tasks and rerun all13 commands on the final source freeze before local acceptance.
  - plan: 167-36/30
    dimension: layout_feasibility
    severity: warning
    description: Current627-line normal provider projection is tight; extracted provider/workflow and new children are not compiled fit evidence.
    fix_hint: Every actual normal source must remain<=628, with declared reservations; stop for checked refinement rather than packing or adding an owner.
```

Root additionally requires meaningful pure-chainstate projection/binding unit controls inline in the already-owned engine/stage.rs, within its actual628-line ceiling, plus a serialized Core unit selection before36 local acceptance. Actual full verification cleans coverage and measures only pure-core packages, including chainstate and excluding Node; Node work_authority controls and compile-fail doctests alone cannot prove that coverage. Final fresh pure-core coverage remains mandatory with no lowering, copied targets, test deletion or new owner. This is an execution constraint under existing Task2/global verification scope, not a claim of tests already run or source fit established; an actual layout failure requires checked refinement. The authenticated evidence is pure-core-execution-constraint.json SHA256 `a02e458042601a0424162d9a1b5796f5b15519fd79933b0f62967de67a435b90` (3 additional checks).

Immediate correct pinned comments and owned module-child registration remain required; central breadcrumb manifest/--check stays Plan19 before whole-phase/native acceptance, avoiding a36→30→19 local-acceptance cycle. Complete final source/API/security/goal review, all compactB-E, future formal E→17 dependency correction, source-before-guard ownership/adjudication, full native/fresh coverage/Bazel/lifecycle/Git gates remain unchanged. No guard, source, build, metadata, SUMMARY or Git action was performed here.

Checker evidence under `.local/open-bitcoin-dev/phase167/resource-projection-plan/check/`: static.json SHA256 `887924899fbb95526f2b0cab56ef5cc01df0fd3a7b1ce3b709aad331591d8438` (445 checks), semantic.json SHA256 `0d8dc08e1f28005ae216eb2733acc0bdb9f8d6dd3610cef0ea3a19fe82d54797` (31 checks), and failure-evidence.json SHA256 `d97efb9429127f9375c3dace43bb5c46aeb10a239368dcb0069c94a120c5e589` (18 checks). The first literal-check setup receipt is retained separately; its case-sensitive wording mismatch was corrected without candidate changes. Append closure records the final report identity after checking exact360,560-byte prefix `a17d8748e6671863111883a5594da908b88d1fd5852f70865a7d7489eca56d73` and all12 captured prefixes, including every prior failed verdict.


## Independent Plan30 complete retained-bound test partition check | 2026-10-22

**PASS, planning only, with two scope/layout warnings; zero blockers.** The frozen candidate preserves the complete existing contract and supplies a coherent two-file test partition. This result grants no source/build release, implementation/phase acceptance, guard renewal or Git authority. Root must adjudicate the warnings and separately authenticate/release the actual ten-owner before-state.

### Frozen candidate and independent preservation

| Input                                    | SHA256                                                             |
| ---------------------------------------- | ------------------------------------------------------------------ |
| Root-dispatched candidate index          | `0068f02b2b62f4c8936c6fb1734916f4399c5c62904bd77e4f3de28f3b0798b8` |
| Published Plan30,56500 bytes             | `0b8d8660d72cf3f37b9fa91d32633fd4239615eaa377cd5faaf68b0f6c09e1d4` |
| New test-partition amendment,13428 bytes | `82db4474279a811d48eb548bd22bbdb64500af19444ddbc2b7eba3376d783515` |
| Independent frozen check index           | `f14e39b1dcd305c90b2dd1b89811d53154a3e29b518e88fa2818c877efde8543` |
| Prior canonical report,371753 bytes      | `561a883c19e020d5df963ad76f717ee7aefc14d98ef28e3203fcc799700b28a5` |

The independent review passes **504 assertions:467 static and37 semantic**. Every indexed publication/candidate copy/receipt authenticates. The old resource amendment remains whole-byte exact at4f493e; accepted36/root release and original resumed7359/f077 provenance authenticate separately. All35 other PLANs and all25 accepted PLANs remain exact. The preparation independently captured68 existing phase planning documents before publication; all except the intentionally revised30 remain exact. Source/SUMMARY/execution and live root metadata were excluded from that planning capture.

The first three Plan30 Task XML blocks, including every action/acceptance/done and all fourteen original automated argv strings, remain byte-for-byte. There are now15 XML automated-tag occurrences but14 unique original command strings: Task4 repeats Task3's exact reopen-bound selection. The unchanged resource amendment adds two distinct selections, yielding **16 unique final30 commands**, plus the separately inherited36 guard-smoke requirement. No seventeenth new30 command, zero-discovery shortcut or command waiver is inferred.

The checker independently applied both patches with BSD patch, zero fuzz, inside a fresh ignored tree: forward reconstructs both whole published documents; inverse restores the entire old30 and true new-amendment absence; forward reconstructs them again. No live planning or source inverse was applied. The actual Rust child remains genuinely absent. Current32 presence/byte states and every present full copy match root's c77 frozen-for-partition observation; all22 unowned states match original7359. This verifies a bounded observation, not source acceptance. Original7359 resources.rs absence and the additional reopen_bound.rs absence remain the required whole-Plan30 inverse target; current nine-present c77 cannot replace it.

The entire pre-append371753-byte report matches the independent preparation copy, and all12 historical prefixes authenticate. Every prior failed verdict/history byte is preserved. Checker-only reader setup failures are retained under check/; correcting the historical array key, fenced command extraction, reused scratch-tree assumption and one literal location required no candidate change.

### Structure, graph and goal-backward contract

Independent loading/structure/must_haves parsing covers all36 plans. Raw inventory is **36 plans,77 tasks,29 waves**. Plan30 stays wave20 with12/33/36 dependencies; no dependency/wave field changes, missing node, cycle, same-wave dependency or unordered shared owner appears. The new child adds only one direct owner; the plan union is ten. Task4 owns exactly the new reopen_bound.rs and existing tests.rs. The unchanged old Task3 allocation remains explicitly refined by the accepted resource amendment; its historic literal records.rs allocation is not falsely described as the effective task union.

| Requirement | Preserved coverage and new contribution                                                                                                           |
| ----------- | ------------------------------------------------------------------------------------------------------------------------------------------------- |
| CVLD-01     | Existing subsidy/maturity/real durable path and complete body/undo oracle remain; capacity tests cannot substitute fixture consensus.             |
| CVLD-02     | Existing candidate-context/intrinsic/duplicate/body ordering remains; new known-member and branch controls still require genuine body acceptance. |
| CVLD-03     | Actual durable script/flag checks remain; retained header membership supplies no validation waiver.                                               |
| CVLD-04     | Existing shared selected header/context authority remains; genuine fork headers and unsupported-context refusals retain their provenance.         |
| CVLD-06     | Existing complete Coins/undo/restoration evidence remains; relocation cannot drop the mature-spend measurements.                                  |

All five roadmap IDs remain in30 frontmatter, all original tasks/truths/links remain intact, and the new child is explicitly wired through its owned registrar. D-01 through D-10 retain their full inherited coverage; Task4 expressly implements D-01/D-07/D-08/D-09/D-10 without adding a deferred product or qualification claim. Existing research resolutions are unchanged; this partition introduces no new research dependency. Dimension8 is skipped because workflow.nyquist_validation=false. Existing historical warnings and accepted exceptions remain attributable rather than being erased by this targeted review.

### Complete tests and actual seam feasibility

The new task moves the existing genuine88-line cap test intact, preserving its name/assertions. It adds real stale/reorg/known/failed membership, accepted genesis/descendant/stale/witness-only duplicate and novel main/side controls against306 TOTAL retained identities. Existing KnownOnly bodies must still execute genuine context/scripts/Coins checks. Early no-effect refusal retains complete accepted/header/body/known/Coins/undo/provenance/BASIC and whole-map/manifest equality. Intrinsic-invalid same-header bodies retain actual bad-txnmrklroot precedence before capacity/context; obsolete uncompiled prototype API spellings and the erroneous earlier oracle are not promoted into truth. Existing behavioral RED/failure receipts remain failures; earlier focused typed/pure/resource GREEN remains scoped evidence.

Enabled305→306 is an explicit actual positive obligation. The proposed fixture retains304 genuine KnownOnly fork headers above genesis, then requires the novel available-body candidate to earn actual preference, Always/BASIC acceptance and total306, followed by complete state/maps/records, capacity refusals and all-drop safe configured reopen. This is a planned fixture option, not an executed or universally proved construction. If its real binding rules disagree, retain the failure and refine setup without bypassing authority.

Read-only frozen-for-partition inspection confirms the existing after_basic invocation follows BASIC work and precedes achieved-inventory verification; the parent already implements genuine external307 admission/publication. Task4 reuses that provider invocation unchanged. Its separate private parent-test hook may write one actually supported Runtime/snapshot row to trigger genuine achieved-inventory operational failure, with advanced body/tip/undo/receipt retained, safe reopen at that advanced state and valid next acceptance. This is distinct from external307 capacity failure. Task4 authorizes no provider/workflow production edit, public constructor or fake consumer.

The complete oracle preserves all five Accepted concerns, all seven non-anchor BASIC fields and the separately exact eighth branch anchor, full processed/safe identities, all nine namespace byte maps, full StoredFilterRecord identity/bytes and independently observed actual epoch freshness before next mutation. Genesis→recovered-tip branch anchors are exact constructor-derived expectations; arbitrary difference or normalization-away is forbidden. Owned data/Copy epochs cross all-handle drops, with precise Rust2024 captures where needed. The broader resource matrix remains in resources.rs, including actual mature body/undo and unchanged native budgets. No boolean/fixture authority, copied capacity, rollback hiding, policy increase, ignored data, blanket ancestor walk, assertion deletion or timeout/row cap relaxation is authorized.

### Warnings and remaining execution gates

The measured proposed layout1703+289−65−1884 exceeds its three-file capacity by43 lines even with the stated optimistic savings. The later1788+224−1884 observation exceeds it by128 before further refactoring. These are versioned observations and arithmetic lower bounds for those layouts, not compiled final measurements or proof that every conceivable refactor fails. The prospective400–450-line child and approximately491+8-line parent preserve coherent responsibility; actual fit remains unproved. Several retained owners are already near628, so every final normally formatted owner must be measured without packing or reduced tests.

```yaml
issues:
  - plan: "167-30"
    dimension: scope_sanity
    severity: warning
    description: "Four tasks and ten direct owners exceed preferred scope targets; Task4 is a coherent two-file test partition with unchanged DAG."
    fix_hint: "Root adjudicates this bounded scope before source release; retain Task4's exact two-file allocation."
  - plan: "167-30"
    dimension: scope_sanity
    severity: warning
    description: "The new-child and parent line budgets are uncompiled projections, with several preserved owners already close to628."
    fix_hint: "Measure all normally formatted final owners and preserve complete controls; stop for checked refinement if fit fails."
```

Configured GFM/frontmatter checks pass for both published documents, and all36 GSD structure checks pass. AGENTS/sidecar/overrides and architecture/code-shape/testing/verification/local-guidance/Rust standards informed this review. Active global/repo lessons were fully read within8856bytes/2953 estimated tokens, with no archives or lesson edits.

Root still authenticates the ten-owner before-state/true absence and releases source explicitly. The final runner must extend preserved31-state history to the full32-state fence: ten direct, twelve accepted36 support and ten API states. All16 final30 argv and inherited36 guard smoke must run on that final fence with complete actual old/new named discovery and zero ignored cases, followed by full source/API/security review, original7359-composed inverse and true absences, and root implementation acceptance. All compact/source-before-guard/native/fresh pure-core coverage/Bazel/goal/lifecycle/normal-hook saving gates remain. No source, build, process, root metadata, SUMMARY, guard or Git mutation was performed by this checker. Independent receipts live under `.local/open-bitcoin-dev/phase167/plan30/test-partition-plan/check/`; append closure binds the resulting canonical report separately.


## Independent logical-freshness prerequisite scope revision check | 2026-10-22

**Planning verification passes with three warnings and zero blockers.** The revised carrier37→activation38→inventory39 partition resolves the prior hard scope blocker without reducing the complete writer, canonical, inventory, command or evidence contract. Root must adjudicate the warnings and separately authenticate/release source. This result grants no implementation acceptance, source/build release, guard renewal, whole-phase acceptance or Git authority.

### Frozen candidate, preserved failure and scope resolution

| Input                                        | SHA256                                                             |
| -------------------------------------------- | ------------------------------------------------------------------ |
| Revision3 root-dispatched index,208812 bytes | `1cd1331cc966efc0cba05240f880f464c8545005c788cd13c2a0ae153b8abd2f` |
| Mechanical Plan37                            | `9726057b1491a1d03b9e2f12a52c3146a3bf6efa26393486a53ddc93318d3329` |
| Complete activation Plan38                   | `7e480f783b57669194d754386a71361e9019bd14da831a768263339c9ad14c11` |
| Inventory consumer Plan39                    | `72dcaaff7ea58ffba91d8c277d3f1ff1c3230ddcf43583d11fa468346bbe90fd` |
| New37–39 amendment                           | `e948d350d42cf88b3932eb62e79845f09383b402978a10e8b2121a663f9aeb49` |
| Bounded Plan30 revision                      | `c793c9e635d16221c23f058674b6d16d67aa7aeec2e5b7fc2e40ed825dda0140` |
| Prior canonical report,383304 bytes          | `988b60e01f5e1e141c1dd06e2c2371a9cb83c11b4fad8f77010675ae91de51e9` |

The preceding frozen candidate `8f732820ec60a4c561f0ab1dc2c541bbf9568403d528c95a4d29ccf9e457d4a5` was **SCOPEBLOCKED, not approved**. Independent index `9c20c995667fd286d91df1c95dc59cdd0e3a2e6a0ab652127ffa6d72a6ede0b8` records1762 passing static assertions, no technical semantic gap, one scope blocker and two disclosed execution warnings. The injected gsd-plan-checker role's specific table/Step8/anti-pattern requires splitting at5+ tasks or15+ owners; root accepted that finding instead of waiving or silently downgrading it. Its frozen report and every473 prior indexed artifact/bound tool remain exact.

Revision3 divides the same seven task responsibilities into2/8,3/10 and2/4 task/owner stages. The checker independently verifies all seven mapped task file lists exactly, every actual downstream Cargo argv including harness options, the complete source-owner union and all required behavior. Timing keys/plan numbers change with their new owners; six additional37 final native verification occurrences are added. No old control or actual command is removed.37 is accepted only as a complete, actually used native carrier/reader refactor with unchanged native behavior;38 must replace aliases with complete real facades and canonical consumers before any logical-feature acceptance;39 must adopt the inventory consumer before the actual physical probe can pass.

### Independent identity, preservation and inverses

The new review passes **2542 static assertions**. All511 new indexed artifacts, bound tools, historical receipts, proofs and published copies authenticate. The old473 indexed artifacts/tools and the prior9c20 checker artifact set also authenticate independently. The complete95-document original baseline is preserved; all36 original PLAN Task XML blocks and automated strings remain exact; all25 accepted PLAN/SUMMARY pairs remain whole-byte exact. Existing resource and test-partition amendments remain exact. Only eleven old plans change:30's exact bounded context/dependency/wave block, and ten scheduling-only tail wave lines.

Original30 remains four tasks/ten direct owners,15 automated-tag occurrences and14 unique original argv. Its two distinct resource argv still yield16 unique final commands, plus inherited36 guard smoke. It retains12/33/36 dependencies and adds39; none of its original task bodies, complete305/306 controls, production ordering or evidence obligations is rewritten.

The checker independently applies the15-target baseline forward patch in a fresh ignored tree, matches all whole published targets, then applies the inverse. All95 original documents return byte-exact and all four new planning paths are actually absent afterward. A separate independently applied16-target scope transition reconstructs revision3 from the blocked candidate, including actual removal of only its generated unaccepted old amendment, then restores the entire blocked candidate through the reverse patch. The old amendment's full frozen1736febe copy remains exact. No preexisting95-document source is deleted and no live inverse is applied.

The complete graph independently derives **39 plans,84 tasks,32 waves**. It is acyclic with no missing dependency or same-wave owner collision.37 depends36/wave20;38 depends37/wave21;39 depends38/wave22;30 depends12/33/36/39/wave23. The ten affected tail waves are exactly original+3. Accepted02's wave5/depends[] remains the original exact scheduling exception; it is not a new missing dependency. The preserved initial checker setup receipt over-required its earliest wave; corrected checking permits only unchanged inherited scheduling exceptions and derives every affected wave without changing the candidate.

The prospective first-party source fence remains exactly52 paths:48 present and four true new-source absences. Every present source/copy and all201 separately versioned engine/Cargo.lock records/copies authenticate and remain unchanged across this planning split. The source is held diagnostic source, not an execution freeze or compiled acceptance. Historical32-source3f07 remains historical; the only diagnostic change relative to it is the590-line Node test child. No old31/32 job is credited with a new52-path or engine-source capture.

Before this append the full383304-byte canonical report and all13 historical prefixes authenticate. Every prior failed verdict byte remains. The old8f/9c20 scope failure, never previously appended under root's hold, is recorded honestly above with its actual resolution; it is not retroactively relabeled passed. Independent append closure binds the final report and exact prefix separately.

### Goal-backward contract and dimensions

| Requirement | Preserved phase coverage and prerequisite contribution                                                                                                |
| ----------- | ----------------------------------------------------------------------------------------------------------------------------------------------------- |
| CVLD-01     | Actual subsidy/maturity/durable histories remain inherited obligations; logical freshness cannot substitute fixture consensus or shortened histories. |
| CVLD-02     | Candidate ancestry, intrinsic/duplicate/body ordering and exact deployment context remain; stamps confer no consensus authority.                      |
| CVLD-03     | Actual durable flags/interpreter path remains; carrier/batch interposition preserves writer algorithms.                                               |
| CVLD-04     | Selected header/body difficulty/time authority remains; physical publication is separated from logical source mutation.                               |
| CVLD-06     | Complete Coins/undo/overwrite/restoration evidence remains; true same-value/ABA writes still invalidate observations.                                 |

All five roadmap requirement IDs remain in new plan frontmatter and actual inherited tasks. D-01 through D-10 retain action/rationale coverage;37–39 explicitly preserve D-01/07/08/09/10 and leave D-02..D-06 algorithms intact. No deferred product, public networking, Phase168 daemon qualification or later historical ownership is introduced. All39 GSD task-structure checks pass; new tasks have exact owners, specific actions, meaningful automated commands and measurable done criteria. Truths/artifacts/key links support the intended observable preservation or physical-maintenance success and logical-write refusal. Native carrier statements describe a bounded structural prerequisite without falsely claiming completed tracking. No silent scope reduction is found.

Dimension8 is **SKIPPED because workflow.nyquist_validation=false**; meaningful automated verification is still present. Existing research questions remain substantively resolved, the audited native signatures/writer/catalog policy are concrete, and actual compilation/layout is explicitly unproved. Cross-plan raw/tracked/opaque-stamp data contracts agree. Actual AGENTS.md, AGENTS.bright-builds.md, standards-overrides.md and managed architecture/code-shape/testing/verification/Rust standards informed this review. Both complete active lesson files were read within8856 bytes/2953 estimated tokens; no archives or lesson mutation.

### Complete technical boundary retained

37's8 owners deliver actually used native aliases, all9 held field/accessor/explicit batch/helper carriers and one sealed raw RecordSource reader. Existing physical production fences and the original direct-schema same-value ChangedStore test remain unchanged. Final native format/strict all-target checks must pass while native Readable imports are still used. There is no fake stamp, dummy caller, unfinished tracking method or logical-feature acceptance. Its inverse restores seven preexisting sources and tracked.rs absence.

38's10 owners deliver complete Database/Keyspace/Snapshot/OwnedWriteBatch facades sharing one per-open identity across every returned/cloned handle. Real mutation BEGIN increments generation/active count before effects; RAII Drop closes active count on success, native Err or unwind. Observation/check requires matching identity/generation and active0 at both ends. Overflow/poison fail closed permanently; errors remain visible; no generation mutex spans I/O, callbacks or existing payload/history/publication locks. Same-value, ABA, nonexistent delete, clear, catalog and committed batches count; existing lookup, buffering, snapshot/persist and physical maintenance do not. Foreign batches refuse before generation/native effects.

There is no raw Deref/AsRef/Borrow/getter/into_inner/callback/config/tree escape. Native row-guard into_inner remains a detached data read. All9 ordinary writer families interpose through their actual held fields/carriers, including Coins H/B, metadata, prune, validation, BASIC, recovery and direct schema writes. Arbitrary independently opened raw engines/filesystem writers remain outside the stated boundary.

The SAME separate catalog mutex spans lookup/create/delete and the **entire tracked native-reader delegation**, closing its exists→create-capable lookup recreation race while releasing generation mutex during reads/I/O. Raw scratch keeps its exclusive lease/engine-lock/copy/manifest/source guards. There is one reader/Budget, returning detached records, with no extra inspection.rs registrar or copied policy.

Both genuine canonical genesis/history consumers replace every production physical sequence fence with opaque identity-bound stamps while retaining publication identity, complete actual proof/source and before/after checks. The original direct schema.insert identical-write test body remains exact. Physical batch-count evidence stays physical. Actual real genesis/history maintenance positives and same-value/ABA/clear/catalog/foreign negatives are required; no recovered capability is fabricated.

39 retains exact catalog, all9 native maps, complete digest, original store, every row/time/native/copy/physical bound and projected/achieved full-byte equality. Its original stamp brackets actual read/digest checks, so equal final content cannot forgive intervening writes. Expected early/header/commit/Always/BASIC writes instead use the genuine new baseline and fresh achieved capture; post-absorption failures retain real advanced state without rollback or consensus-invalidity fiction.

The four-owner storage test child uses actual public DurableSyncRuntime factories, runtime.store()/network_handle(), inferred command/outcome types and local genuine helpers. Private command fields/provider-test helpers and a hidden public getter are unavailable. The unchanged actual Node physical probe remains read-only and must naturally turn GREEN after39, not after37/38. Its old RED94aa/rawc2cc and original1448.38s4PASS/1FAIL33ac/raw6af remain failures. The complete genuine305/306 prefix, both BASIC modes, all-handle configured safe reopen/next and actual same-value/ABA/empty/tombstone/clear/catalog/in-flight/native-error/unwind/overflow/poison/foreign controls remain required.

Accepted37 carrier sources outside38's declared ten are **read-only support**, not additional edit permission: any actual extra owner needs checked refinement.39's isolated inherited threat-cell wording “37 stamps” is read with its explicit interfaces/key link/action requiring accepted38's actual MutationStamp;37 aliases cannot satisfy that contract. These clarifications confer no hidden owner or alternate producer.

### Warnings and remaining root gates

```yaml
issues:
  - plan: "167-38"
    dimension: scope_sanity
    severity: warning
    description: "Ten direct owners exceed the preferred file target, while three tasks and all per-task allocations remain below blocker thresholds."
    fix_hint: "Root adjudicates the concrete bounded activation scope; preserve every declared writer/canonical/control obligation."
  - plan: "167-37/38/39"
    dimension: scope_sanity
    severity: warning
    description: "Normal formatted adapter/test/API fit is uncompiled; several preserved owners are close to628 lines."
    fix_hint: "Measure actual normally formatted files, retain full controls and stop for checked refinement on cap or owner failure."
  - plan: "167-38"
    dimension: task_completeness
    severity: warning
    description: "Exactly four audited Readable imports may warn between38.1 activation and38.2 cleanup; strict checks correctly follow cleanup and final38 controls."
    fix_hint: "Root records this staged boundary; no suppression/premature strict claim.37 must pass native strict checks and final38/39 strict checks remain mandatory."
```

All four new planning documents pass independent configured GFM/frontmatter checks with standalone threat-model delimiters and no extra body frontmatter delimiters. No new5-task/15-owner blocker remains. The three warnings require explicit root disposition; this checker does not claim their eventual source fit or test outcomes.

Root must freshly capture actual stage-specific owner/support/API/writer/engine before-state and absences, then release the sole scheduler. Actual37,38 and39 acceptance each requires its real selections/final strict checks, normal628 budgets and independent source/security/API/inverse review; composed inverses retain prior accepted-stage provenance. All16 final30 commands plus inherited36 smoke subsequently rerun on the complete actual expanded fence. None of the new selections substitutes for that final30 matrix.

All C/D/E formal work after actual35, formal E→17 before guard renewal, final-source review before guard renewal and Plan19 central manifest remain. Full default native, fresh pure-core coverage, Bazel, independent source/security/goal, originating167-2026-10-21T16-38-23/yolo lifecycle and normal-hook whole-phase main push remain OPEN. Stop before168. Independent receipts and frozen check index live under `.local/open-bitcoin-dev/phase167/resource-freshness-plan/check-preparation/revision3-check/`; append closure binds the resulting report. This checker ran no Cargo/Bazel/application, edited no source/plan/root metadata/guard/SUMMARY, and performed no Git action.


## Plan40 native journal-pressure prerequisite — root accepted planning revision3

Frozen proposal149ae39e; root planning acceptance below is planning only. All source/39/30/global/native/coverage/Bazel/lifecycle/Git gates remain open. Historical check content is preserved as an exact byte prefix.

## VERIFICATION PASSED

**Phase:** 167 — Candidate-History Validation Rules\
**Review:** New Plan40 and the preservation/dependency changes across the existing39 plans\
**Frozen proposal:** `149ae39e1888e923acf46e1a93b602bbb2c7c63bf979d194dfd809b6e0f1adb8`\
**Lifecycle:** `167-2026-10-21T16-38-23`, yolo, `commits:false`\
**Unresolved issues:** 0 blockers, 0 warnings

This is a planning verdict. Root must independently authenticate, publish and release the checked scope before source execution. It grants no Plan40 implementation acceptance, Plan39/30 acceptance, physical-defect closure, CVLD closure or Git action. No source, live planning, shared metadata, application, build or test was changed or run by this checker.

### Frozen inputs and independent proof

| Input                                             | SHA256                                                             |
| ------------------------------------------------- | ------------------------------------------------------------------ |
| Final revision3 proposal index                    | `149ae39e1888e923acf46e1a93b602bbb2c7c63bf979d194dfd809b6e0f1adb8` |
| Proposed167-40-PLAN.md                            | `2adc35722b9abbdb8d8e2289b9c6d3e3c446b74dd9070f39182105906994ae51` |
| Proposed167-40-PHYSICAL-INVENTORY-AMENDMENT.md    | `45edb2f5b37887f4cffed7b7928f387e09abf7f91caa5860628080e928b05228` |
| Proposed heading-only RESEARCH annotation         | `efd9e42bde665b9a621e9dd286fcc3a541eb4920269db819c0bdfb30e7063c28` |
| Original106-present/two-absence planning baseline | `d4f096ba17a0b78b294260609f10e735d1c2f7ff06eac64c08dcfd4e5c1f1beb` |
| Independent3895-check audit                       | `3916a67a671489a7ef5ec45afd19fd74de7e4481c4f657e1cdfe357bbf3f295f` |
| Independent19-check invariant comparison          | `76a11d176eb59b4f8597275e826c6c13d9b9416ede2d0c8645d1a9988c235f62` |

The independent audit authenticates854 unique file/hash references, including original/proposed planning bodies, current/source-copy inputs and separately versioned engine inputs. All hashed reference paths must exist; contents and declared byte counts must agree. The proposal is108 bodies:106 original present inputs plus two genuine new planning documents. Exactly15 targets differ:12 previously unaccepted plans, one research heading, two new documents.

My own `/usr/bin/patch --batch -E -F0 -p1` sandbox applies the frozen patch and reproduces every108 proposed body. Reverse application restores all106 original bodies and both true new-document absences. This is independent reproduction, not reliance on the planner's reported inverse. No patch was applied to the live checkout.

Every original39 Task XML block and automated command string remains whole-byte exact. All27 accepted PLAN/SUMMARY pairs remain whole-byte exact. Ten downstream plans change only their wave; Plan39 additionally receives the bounded predecessor/context note, and Plan30 additionally receives its bounded future-frame note. I read both exceptional diffs in full. No earlier task or verification is narrowed.

### Requirement and decision coverage

The roadmap goal remains ordinary, selected-network and candidate-history-derived header/body validation. Its five mandatory requirements are **CVLD-01, CVLD-02, CVLD-03, CVLD-04 and CVLD-06**. All remain present in existing plan frontmatter and tasks; their original executable coverage is unchanged. Plan40 repeats those IDs for a shared durable-storage prerequisite and does not claim to deliver their consensus rules by itself. Relevant PROJECT.md requirements remain preserved; later differential, fork/recovery, pruning and historical qualification retain their separate owners.

All ten locked D-01…D-10 references remain represented in task actions/rationale across the frozen plan set. Plan40.2 references D-01; both new tasks explicitly implement D-07…D-10. The amendment separately explains unchanged D-02…D-06 monetary, ancestry, script, difficulty and overwrite obligations. No deferred wallet, public-network default, repair/import, service/funds or Phase168–171 qualification task appears in the new scope.

No scope-reduction substitute is accepted. The production candidate must demonstrate actual automatic journal retirement, unchanged complete physical inspection, complete nine-map restoration, stale original capabilities and safe fresh genesis proof. A configuration getter, decoded equality, fake source, seeded/shortened history or successful small control cannot replace those outcomes. The original305/306 tests and every later requirement remain intact.

### Plan and dependency summary

| Plan                    | Tasks     | Source owners | Wave  | Prerequisite                                  |
| ----------------------- | --------- | ------------- | ----- | --------------------------------------------- |
| 40 new                  | 2         | 2             | 22    | Accepted38, transitively36                    |
| 39 unchanged tasks      | 2         | 4             | 23    | 38 and actual accepted40                      |
| 30 unchanged tasks      | 4         | 10            | 24    | 12,33,36,39                                   |
| 13 and35                | Preserved | Preserved     | 25    | Existing prerequisites; root executes35 first |
| 14→15→16→17→18→19→25→20 | Preserved | Preserved     | 26→33 | Existing dependencies                         |

All40 plan files were loaded for preservation, frontmatter, requirement and graph checks. Every reference exists, the graph is acyclic, and new/changed waves equal the derived predecessor maximum plus one. Previously checked scheduling gaps remain unchanged: for example accepted Plan02 has no explicit dependencies and remains in wave5. That inherited deliberate schedule is not silently reset to wave1. Totals derive to40 plans,86 tasks,33 waves. Accepted work is not reopened by this bounded refinement.

New Plan40 is within the hard scope gate: two tasks and two exact owners, existing56-line `opening.rs` and a genuinely absent conventional `opening/tests.rs` child. No third owner, new dependency, raw authority API, config getter or wrapper redesign is hidden in the task actions. The child targets620 normally formatted lines, with every owner bounded by628. Actual compiled fit remains an execution prerequisite; discovering another owner or exceeding the layout bounds requires checked refinement. The unrelated future C/D/E proposal remains unplanned; its five-task D draft is still blocked and receives no waiver here.

### Verification dimensions

| Dimension                 | Result  | Evidence                                                                                                                            |
| ------------------------- | ------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| 1 Requirement coverage    | PASS    | Five phase IDs and original task coverage retained; no relevant goal requirement dropped                                            |
| 2 Task completeness       | PASS    | Both tasks have substantive files, read-first, behavior, action, verify/automated, done and acceptance-criteria fields              |
| 3 Dependencies            | PASS    | Complete acyclic40-plan graph; exact changed/new waves; immutable accepted predecessors                                             |
| 4 Key links               | PASS    | Actual leased builder call; genuine canonical factory/into_parts; held tracked writes; safe public reopen                           |
| 5 Scope sanity            | PASS    | New2tasks/2owners; no implicit extra implementation owner or packed formatting                                                      |
| 6 Verification derivation | PASS    | Observable automatic retirement and full authority preservation, not an asserted policy number                                      |
| 7 Context compliance      | PASS    | D-01…D-10 retained, deferred scope excluded, ordinary histories and refusal oracles preserved                                       |
| 7b Scope reduction        | PASS    | No shortened305, raised cap/deadline, forced test maintenance, omitted proof or later-wiring substitute                             |
| 8 Nyquist                 | SKIPPED | `workflow.nyquist_validation` is explicitlyfalse; no applicable Validation Architecture gate                                        |
| 9 Cross-plan contracts    | PASS    | Existing writers, codecs, tracked mutation rules, projection and physical policies remain exact; fresh successor fences required    |
| 10 AGENTS.md              | PASS    | GSD scope, pinned tools, command-timings/cooperative lock, normal formatting, breadcrumbs and full native completion gates retained |
| 11 Research resolution    | PASS    | Only the authorized `(RESOLVED)` heading annotation changes; all four answers and every other research byte remain exact            |

Independent `gsd-tools verify plan-structure` reports valid, two complete tasks, zero errors/warnings. Independent compatible `mdformat --check` passes both new documents. Each threat-model tag occurs in the correct order, with a standalone closing tag outside the GFM table. The literal research marker denotes resolved planning choices; it does not assert successful implementation or empirical305 support.

### Native feasibility and hardest constraint

The hardest constraint is scheduled first: a genuine automatic-pressure behavioral RED must precede any production policy change. The actual pinned Fjall3.1.10 `Builder::max_journaling_size(self, bytes:u64)` accepts67108864 as its minimum. Its worker compares sealed-journal bytes during natural flush work and asynchronously requests laggard namespace rotations. The cold schema row is written by the genuine original genesis batch; persisted-table watermarks can keep its original shared journal retained. Active journals are separately preallocated to64MiB, so sealed pressure is not a complete-tree bound.

The new opening child can use existing visible `open_canonical_genesis`, inferred `into_parts`, opaque stamp/check, held tracked keyspace writes, snapshots/iteration and `reopen_canonical_genesis_existing`. Sibling-private seal fields or helpers are unnecessary. Restoring exact maps cannot revive an old mutation stamp or recovered seal. Full-drop safe reopen mints fresh proof through existing copied inspection and original reproof, with no Plan39-only SubmissionInventory dependency and therefore no40→39 API cycle.

The finite producer remains at most4096 same-key inserts, one reused value at most65536bytes and at most268435456bytes cumulative value payload. The observation bound remains30seconds. Task1's completed valid old-policy retention is the expected discriminating RED; incomplete workload, native/setup/cap errors or old-policy retirement are not that RED. Under the new policy, retention/exhaustion is always a real failure and STOP. No scheduler guarantee is invented. The19-check comparison confirms Task1 whole-byte equality with revision2, all eight command strings unchanged, every original full hash retained, correct256MiB decimal literal and unchanged physical/workload/timer values. The accidentally modified unpublished candidate is not the checked input.

Automatic scheduling reliability, actual retirement, complete copied-proof stability and normal compiled fit are unproved until execution. The plan explicitly preserves failures and stops rather than enlarging data/time, forcing maintenance, suppressing source changes or retrying until green. These are bounded execution gates, not ignored findings or proof that this candidate completes305.

### Source frame and successor evidence

The prospective first-party frame is genuinely **55 states:54 present plus one new-child absence**. It preserves all original53 input memberships/bytes, adds the existing read-only `inspection/worker.rs`, and adds the genuinely absent opening child. The worker is12041bytes/SHA`46873afbe18c3921739cc7fdf4fd2746d030a585ee9c064b3bc4414a3b6a7d85`; its independent source copy is authenticated. It supplies the explicit30second deadline and safe-reopen runtime contract. Every named read-first Rust input is classified; the native201 inventory is separately authenticated and remains separate from per-job first-party captures.

There are still only two source owners. Every actual command must capture55 before/after states. The direct40 inverse must later restore the exact opening before body plus true child absence while protecting53 other inputs, then reproduce all55 final states. No source inverse has been performed before implementation; only the planning inverse above has actually run.

After actual40 acceptance, Plan39 requires a new55-input final freeze/51 protected paths and all nine fresh original commands, including the full unchanged six-case retained-bound selection and original305-prefix/306 refusal/reopen. The prior nine actual jobs remain historical; three earlier GREEN selections and five passing cases inside the failed selection are not reused as new final receipts. Original53/54 captures retain their true scopes.

Only after actual39 acceptance may Plan30 derive its new55-input frame/45 protected paths and execute all16 unique planned commands plus the actual36 smoke. Its original7359/f077/0702 inverse provenance remains authoritative; old53/54 prepared scripts cannot be relabeled or blindly executed against changed opening bytes. No40 control replaces either successor's required outcomes.

### Revision history and retained failures

Revision1 index98cb729c remains immutable and rejected. Both tasks lacked explicit read-first/acceptance fields, the threat closing tag was folded into a table, and research lacked the literal resolution marker. My historical findings also requested clear expected-RED versus unexpected-exhaustion classification. Revision2 fixes those points; its declaration-scoped2423 checks/850 references and static-tool PASS remain historical. Root and checker separately held revision2 because its54-state frame omitted the named first-party worker input. Root coverage findingc08578ad and checker revision2/hold.json are retained. Final revision3 explicitly fixes that omission with real55-state evidence; historical scopes are not relabeled.

The actual original39 failure remains c217bcd7/rawb9c3ae9c:5PASS/1FAIL after4812.76test seconds, late accepted-physical-inventory limit refusal. The exact failed height is unknown. The retained539516362byte tree is post-drop corroboration, not a failure-time checksum/reopen proof. No final39 SUMMARY/CLEAN/acceptance or defect-resolution claim follows this planning review.

The checker itself corrected two local static-audit assumptions without changing proposal inputs: inherited accepted scheduling is validated as a preserved minimum rather than reset, and hash extraction accommodates prose's `SHA256` prefix. Those were checker diagnostics, not implementation/test failures. Final auditable results are3895 authentication/preservation/graph/roundtrip checks plus19 invariant comparisons and two independent static-tool commands, all passing.

### Remaining root gates

Root publication/release, actual40 evidence/inverse/review/SUMMARY acceptance, all fresh39 and30 outcomes, actual35 then13 integration, formal C/D/E and E→17 dependency, canonical focused final-source review BEFORE17 guard renewal,18 security/mutants,19 parity/README/central breadcrumbs,25 writing-for-agents/project excerpts,20 complete default native verifier with fresh pure-core coverage/Bazel/source/security/goal/lifecycle checks and normal-hook main commit/push remain mandatory. Stop before168.

Guidance materially applied: actual AGENTS.md, Bright Builds sidecar, narrow overrides, standards index, architecture/code-shape/testing/verification/Rust rules, the Revision Gate and planning calibration references, and both complete active lesson files. Lessons total9838bytes/3280 conservative tokens; no active block was omitted and no new audit trigger was introduced. No lesson, archive or shared metadata was modified.

### Structured issues

```yaml
issues: []
```

Root may independently authenticate and publish this frozen planning scope. Source implementation remains gated by root's separate fresh before capture and bounded execution release.



## Plan41 bounded live physical stability — root accepted planning revision2

Frozen proposal a6720066; root acceptance is planning only. Source implementation,40/39/30 and all global gates remain separate. The entire prior415941-byte checker prefix is preserved.

## VERIFICATION PASSED

\*\*Phase:\*\*167 Candidate-History Validation Rules\
\*\*Scope:\*\*New41 plus preservation/dependency review of all40 existing plans\
**Frozen proposal:**`a6720066cc355e9f211fa080a6b3cce4f6893d5b0bbb4fcecfc4be0c15f0c23c`\
**Lifecycle:**`167-2026-10-21T16-38-23`, yolo, `commits:false`\
\*\*Unresolved issues:\*\*0 blockers,0 warnings

This verdict approves the bounded planning design. Root publication, fresh source-before capture and execution release remain separate. No implementation, compiled fit, runtime success,41/40/39/30 acceptance, physical-defect closure, CVLD closure or Git action is established. This checker changed only its owned ignored preparation directory and ran only static planning tools, authentication and sandbox planning patches; no application, Cargo/Bazel, tests, source, live planning, shared metadata or Git operation was run or changed.

### Exact input and proof identities

| Input                                    | SHA256                                                             |
| ---------------------------------------- | ------------------------------------------------------------------ |
| Immutable revision2 index                | `a6720066cc355e9f211fa080a6b3cce4f6893d5b0bbb4fcecfc4be0c15f0c23c` |
| New167-41-PLAN.md                        | `e5fc7fc724701c0a587562d98255777d142e0353a52e295c0448ceeaf7ecfbb4` |
| New167-41-LIVE-PHYSICAL-AMENDMENT.md     | `7208050d721adf293b6768d148016e644cd90a07fe1bdc9ed1788ea05fe1b68f` |
| Original planning/source baseline        | `d03adf300aba855b7c0e3bc2774c8d56da418c5ca10785a7929ad74750a5b747` |
| Independent4336-check audit              | `2af0375566d5fbfb30cc8772fb81d7371f2f4b88b0a899b14683f3c30b7a1166` |
| Independent29-check invariant comparison | `cc354d40c0ce300a043f0fc66a4265846866c5c9147b32f834efb3dffc6b3285` |

The independent audit authenticates507 actual path/copy/hash references and their byte counts. It loads all110 proposed planning bodies, the108 original bodies/two true absences, all61 source states and the separate201 engine inputs. My own `/usr/bin/patch --batch -E -F0 -p1` applies the frozen patch in an ignored sandbox and reproduces all110 proposed bodies; reversal restores108 exact originals and both true new-document absences. No patch was applied to live planning or source. Source inverse is only planned and must be proved after implementation.

Exactly15 planning targets differ:13 previously unaccepted plans plus two new documents. Every original40 plan's Task XML and automated command strings remain whole-byte exact. All27 accepted PLAN/SUMMARY pairs remain exact. Research, requirements, STATE, ROADMAP, todo, config, canonical checker and all other original planning bodies remain exact. The13 changed old plans are13/14/15/16/17/18/19/20/25/30/35/39/40; only40 gets the new dependency and bounded context,39/30 get bounded61-input continuation notes, and the other ten change only their wave. All current pending40/39 source bytes remain protected, rather than accepted or undone.

### Requirement and decision coverage

The roadmap goal remains pinned selected-network and candidate-history validation through ordinary header admission and durable body connection. **CVLD-01, CVLD-02, CVLD-03, CVLD-04 and CVLD-06** remain present in plan frontmatter and retain their previously checked executable coverage. New41 repeats them for a shared physical-observation prerequisite; it does not claim to implement their consensus rules itself. Relevant PROJECT.md goals and later milestone owners remain preserved.

All D-01…D-10 references remain represented in task actions/rationale across the41-plan set. New41.3 explicitly references D-01; all three new tasks implement D-07…D-10. The amendment retains monetary/maturity, branch-context, script, difficulty/MTP and overwrite obligations D-02…D-06. No deferred actual-daemon qualification, fork/recovery, prune integration, historical closure, wallet/service/import/repair, public-network default or production/funds scope is added. All CVLD requirements remain globally OPEN.

There is no scope-reduction substitute: full hashes, all identity fields, original logical freshness, unchanged per-source caps, real native controls and all later original histories remain required. Neither a successful small control nor observed retirement replaces305, authenticated product integration or whole-phase verification.

### Scope and dependency summary

| Plan                    | Tasks     | Source owners | Wave  | Actual prerequisite                |
| ----------------------- | --------- | ------------- | ----- | ---------------------------------- |
| 41 new                  | 3         | 4             | 22    | Accepted38, transitively36         |
| 40 original tasks       | 2         | 2             | 23    | 38 and new41                       |
| 39 original tasks       | 2         | 4             | 24    | 38/40;41 transitive                |
| 30 original tasks       | 4         | 10            | 25    | 12/33/36/39                        |
| 13 and35                | Preserved | Preserved     | 26    | Existing dependencies; root35FIRST |
| 14→15→16→17→18→19→25→20 | Preserved | Preserved     | 27→34 | Existing dependencies              |

All dependency references exist and the graph is acyclic. Every new/changed wave equals the maximum predecessor wave plus one. Previously checked nonminimal accepted schedules remain unchanged, including dependency-free Plan02 in wave5. Counts derive to41 plans,89 tasks,34 waves. No accepted historical scope is reopened or silently waived.

New41 has three tasks/four exact source owners: existing inspection.rs398lines, existing copy.rs364lines, absent live.rs and absent live/tests.rs. This is below the hard five-task/fifteen-owner blocker thresholds. Every owner must fit normal628-line formatting; the new test child targets620. Private API visibility is limited to inspection and descendants. Concrete compiled/API/test fit remains unproved; discovering another owner/read-first implementation or exceeding normal fit requires checked root refinement. The unrelated future C/D/E D draft with five tasks remains blocked; E→17 must be formalized before guard renewal.

### Verification dimensions

| Dimension                 | Result  | Basis                                                                                                                           |
| ------------------------- | ------- | ------------------------------------------------------------------------------------------------------------------------------- |
| 1 Requirement coverage    | PASS    | Five mandatory IDs and original task coverage preserved; no relevant PROJECT goal dropped                                       |
| 2 Task completeness       | PASS    | All three tasks have substantive files/read-first/behavior/action/automated verify/done/acceptance-criteria                     |
| 3 Dependencies            | PASS    | Complete acyclic41-plan graph; changed/new waves derived; accepted gaps preserved                                               |
| 4 Key links               | PASS    | Existing live gate→private helper→single typed copy core; original stamp and held root surround complete observations           |
| 5 Scope sanity            | PASS    | 3tasks/4owners; normal fit and explicit stop conditions; no hidden owner/API                                                    |
| 6 Verification derivation | PASS    | Real native physical change/stability, stale writes, root security and finite-work behavior—not configuration assertions        |
| 7 Context compliance      | PASS    | D-01…D-10 retained, original histories/strict proofs/deferred scope preserved                                                   |
| 7b Scope reduction        | PASS    | No shortened305, field/hash omission, cap/deadline increase, new stamp, arbitrary retries or fake native success                |
| 8 Nyquist                 | SKIPPED | Explicit workflow.nyquist_validation:false; no applicable Validation Architecture gate                                          |
| 9 Cross-plan contracts    | PASS    | Strict original errors, full Inventory equality, detached rows, stamps and future61-input composition remain compatible         |
| 10 AGENTS.md              | PASS    | GSD ownership, pinned tools, timing lock, formatting, breadcrumbs, full native/fresh coverage/Bazel/final review/hooks retained |
| 11 Research resolution    | PASS    | Existing resolved-heading/answers stay byte-exact; no unresolved planning question is introduced                                |

Independent gsd-tools plan-structure reports valid,3 complete tasks,0errors/0warnings. Independent compatible mdformat check passes both new documents. Opening/closing threat tags are standalone outside GFM tables, and only the two top frontmatter delimiter lines use standalone triple dashes. All ten new automated command occurrences are unchanged between revisions; seven belong to the final immutable matrix. Workloads, caps, lifecycle, source counts and old full hashes are checked separately by29 passing invariant checks.

### Classified source/security contract

The hardest constraint is established early: Task1 refactors ONE complete walker/hash core and records a real physical-maintenance RED through the still-strict live route before Task2 changes routing. Existing strict capture/copy/recheck/Identity wrappers immediately unwrap private Changed(StorageError) or Refused(StorageError) to the same original error value and precedence. Offline worker, locking, copy, cleanup and safe producer paths get no retries or altered semantics. New typed classification is contextual at the actual source branch, never error-string matching or a public StorageError change.

Only safe known physical identity/content changes, already-known descendant NotFound and known-file UnexpectedEof may be Changed. Initial missing/replaced/unsafe root, symlink/hardlink/type/mode/link security, permission, unsupported metadata, path/depth/entry/byte/arithmetic/work exhaustion, destination-copy and nontransient I/O are Refused immediately. Both individual Identity checks and the complete second-inventory comparison prioritize observed common-entry safety differences. A readable chmod between otherwise stable captures must be Refused/no second attempt; complete inequality cannot conceal it as ordinary change.

**Every Inventory field stays intact, including root length and mtime.** Only the separate Unix held-root anchor identity omits those two volatile fields. Its read-only directory File pins the original inode, binding canonicalpath/dev/inode/type/mode/nlink against both held descriptor and fresh path metadata around attempts and Changed transitions. Missing/replaced/unsafe anchor never becomes transient. Root metadata-only churn is a separate actual filesystem control: it must cause full Inventory Changed, then require TWO actual full equal inventories on the second attempt. It is not described as native evidence. Non-Unix keeps the strict single-attempt path plus original stamp checks; portable inode/root-fd guarantees are not invented.

The helper captures ONE opaque original MutationStamp before any filesystem I/O. It validates that stamp before/after each attempt, after Changed before another attempt and before final return. Same-value/ABA/in-flight/foreign/poison/overflow failures cannot be forgiven by a fresh stamp or equal maps. Mutex guards do not span filesystem I/O. Actual blocked writer controls must release/join before result assertions can unwind.

One external, nonrefundable WorkLedger spans both possible attempts. It stays outside Inventory/Eq and per-capture Budget, so equal trees still compare equal. Entry visits are charged before metadata, maximum8192; full requested read_exact bytes are charged before reading—including partial failed reads—maximum1073741824; EOF uses a separate precharged8192 one-byte allowance, never content funding. All per-capture536870912bytes/4096entries/depth16/path4096 checks,64KiB full hash chains, EOF and opened-file/path checks remain. Four possible captures must fit the old maximum two-walk scan/hash dimensions; near-cap sources may refuse. There is no new syscall deadline or stable-scheduling guarantee.

Only Changed permits a second full attempt after original-root/stamp validation; Refused stops. A second Changed returns its original error, never a third attempt or success based on benign-looking change. No sleeps/backoff, deadline growth, physical sequence authority, native quiescence, second engine, metadata-only substitute, raw handle or field normalization is authorized.

Native controls prepare distinct genuinely populated namespaces before the original stamp and rotate each at a narrow cfg(test) observation boundary. Rotating an already-empty memtable twice or inserting an intervening logical write is insufficient. Primary controls observe real physical changes, full9-map/catalog equality, original stamp validity and actual second-attempt hashing. Existing poison/overflow internals remain covered by inherited generation controls. Permission and permanent-I/O controls must exercise real available OS behavior; unsupported fixture behavior is disclosed for root review, rather than replaced with fake errors or silently skipped. Original40 automatic-pressure and305 fixtures remain byte-exact and receive no forced maintenance.

### Source and inverse composition

The prospective61-state frame is genuine: old55 current inputs plus four read-only strict-test/protocol inputs and two true new live source absences—59present/2absent. The additions are inspection/tests.rs, inspection/tests/copy.rs, inspection/protocol.rs and inspection/worker/tests.rs. Every named read-first first-party Rust input is covered; instructions/planning/diagnostic documents are separate context observations. The201 engine/lock inputs remain separately versioned, not per-job claims. Pending40's60-line opening and251-line child and pending39's source stay protected current facts, not accepted prerequisites.

After implementation, direct41 inverse must restore its two original bodies and two true absences while protecting57 other states, then reproduce61 final states. That SOURCE proof has not been performed during planning. Future40 must use new61 captures/protect59 non40 states and retain original30cf opening-before/child-absence provenance while preserving accepted41. Future39 uses61/protect57 and original85e three-before/one-absence provenance, preserving41/40. Future30 uses61/protect51 and original7359/f077/0702 eight-present/two-absence provenance, preserving41/40/39. Historical53/54/55 frames and unrun obsolete scripts cannot be relabeled or blindly executed.

Actual41 acceptance must precede all seven fresh40 final commands; actual40 precedes all nine fresh39 commands; actual39 precedes all17 fresh30 commands. Earlier failed selections and partial passing cases are not substituted. Seven41 final commands must run on one real immutable61-source freeze with complete per-job copies, actual discovery/results and independently reviewed inverse/evidence/SUMMARY.

### Historical findings and outcomes

Revision1 fe9c2742 remains immutable and held. Its declaration-scoped audit passed **4324 checks/504 references**, retained with its own record. The checker found Task2 requesting shared copy-core root-policy work absent from its file list. Root independently identified the more fundamental unselected normalization of Inventory root length/mtime. Revision2 removes that policy entirely, preserving ALL Inventory fields and adding real root-metadata/full-second-attempt evidence; this resolves the task-file mismatch without a new Task2 file or source owner. Both historical findings remain separately recorded. No final PASS was issued for revision1.

Actual old512 pressure RED20a188e7/raw4ebbe12b remains a58.91second behavioral failure with retained original journal,4096writes/268435456payload, restored maps/stale capabilities, strict physical PASS and cleanup before the failing retirement assertion. Actual new64 first final4f8084d8/raw67bc8dc5 remains a2.38second failure AFTER genuine retirement/all-map restore/stale checks, at strict live physical sourcechanged. Safe reopen/explicit cleanup were not reached. Matrix0/7passed,1attempted/6never started;55-source freeze1eb3 and diagnosis6742046b remain authentic. Exact raced entry and capture-versus-recheck stage remain unknown. No source CLEAN is inferred from the reviewer capacity failure, and no40/39/30 SUMMARY/acceptance is granted.

### Remaining gates and guidance

Root must authenticate/publish planning and capture/release the actual source frame. Runtime API/normal-format fit, native discrimination, OS fixtures, cumulative accounting, strict original errors and full stable observations remain execution gates—not claims from static review. Failure/exhaustion preserves evidence and returns for checked refinement; there is no retry-until-green or root waiver.

Actual41→40→39→30 precedes35FIRST then13;14/15 ordinary301/302/16 seeded work remains. Formal C/D/E must use actual35 interfaces, D must be under five tasks and E precede17. Canonical final SOURCE review precedes17 guard renewal including123 timing-checker ownership;18 security/native/mutants,19 parity/README/manifest,25 writing-for-agents excerpts,20 full default native/fresh pure-core coverage/Bazel/source/security/goal/lifecycle and normal-hook main push remain mandatory. Stop before168.

Material guidance: actual AGENTS.md, Bright Builds sidecar/overrides/index, architecture/code-shape/testing/verification/Rust standards, Revision Gate and checker calibration, current CONTEXT/research/requirements/PROJECT/roadmap/state and the diagnosed actual source. Both active lessons were fully read earlier and their current hashes match:9838bytes/3280 conservative tokens, no omission or new audit trigger. No lesson/archive/shared metadata was modified. Media-only routing is irrelevant here.

### Structured issues

```yaml
issues: []
```

Root may independently authenticate and publish this planning scope; source execution remains separately gated.



## Plan41 directory-link and five-owner refinement — root accepted planning revision2

Frozen proposal884303dd; planning-only acceptance preserves all history. Actual source62 execution, independent reviews and41/40/39/30/global acceptance remain separate. The entire prior434535-byte checker prefix is exact.

## VERIFICATION PASSED

\*\*Phase:\*\*167 Candidate-History Validation Rules\
\*\*Review:\*\*Bounded directory-link, fixture-evidence and fifth-owner refinement of existing41\
**Frozen proposal:**`884303dd09b53074d4423142a3ba4e4b2b4d4445e1e1f919e068d000db38f77a`\
**Lifecycle:**`167-2026-10-21T16-38-23`, yolo, `commits:false`\
\*\*Unresolved issues:\*\*0 blockers,0 warnings

This is planning approval only. Root must separately authenticate, publish, capture actual working inputs and release continuation. No revised-policy runtime success, normal compiled fit, source inverse,41/40/39/30 acceptance, CVLD closure or Git action is established. All checker writes are confined to the new ignored directory-link-plan/check-preparation directory; no source, live planning, shared metadata, build/test, engine or Git operation was changed or run.

### Frozen proof

| Input                                 | SHA256                                                             |
| ------------------------------------- | ------------------------------------------------------------------ |
| Revision2 proposal index              | `884303dd09b53074d4423142a3ba4e4b2b4d4445e1e1f919e068d000db38f77a` |
| Proposed41PLAN                        | `b5693799d3a55ec16c3ddb867c5a32ea6f80f0310d49420c5bab336e9cc991d6` |
| Aligned41LIVE-PHYSICAL-AMENDMENT      | `f9da1187ba0d822809f9189cfc66b687c426a1ca858f9ca187fa37e4e83ef8dd` |
| New41DIRECTORY-LINK-AMENDMENT         | `d3929b11e256a8b898dd033dcb1e8eb0354f20cc0f5bc6642899ff9b1b3e4dfc` |
| Fresh planning/working-input baseline | `a5d4cd7310b3a45233caf9e0fae7005d96113ce99c5b5bf58aa353f0056e0cec` |
| Independent7206-check audit           | `f4ff850e4cb1b3edc2ce337b22f5ff96a35116fab26c1e8b5b13143dc786c94f` |
| Independent24-invariant comparison    | `cb1cd0b32d54e8f31cb23f694cb7c817e4b9cf6aa99b0de88348d14638bcd68a` |

The independent audit authenticates1822 actual path/copy/hash references and their byte counts. It checks all111 proposed planning bodies, the110 original bodies/one true absence,62 working source states,201 separately versioned engine inputs, original inverse authority and historical command evidence. My own `/usr/bin/patch --batch -E -F0 -p1` sandbox forward reproduces111 proposed bodies; reverse restores110 originals and the true new amendment absence. No patch touches live source or planning.

Exactly six planning targets differ:41PLAN,41LIVE amendment,new41DIRECTORY amendment and added continuation context in40/39/30. All40 other plans' Task XML and automated commands stay whole-byte exact. All ten41 automated strings stay exact. All27 accepted PLAN/SUMMARY pairs, research, requirements, metadata, waves and dependencies stay exact. Other old plan changes are context-only, not frontmatter/task changes. Root's later publication/metadata work remains outside this proposal.

I read the complete diagnosis and relevant captured core/live/test bodies, the new amendment and exact plan changes. The current captured inspection399 body differs from original398 only by the live registrar/delegation; the rest remains the previously read strict implementation. Current partial41 code is context for this refinement, not accepted implementation evidence.

### Requirements, decisions and scope

The roadmap goal remains pinned selected-network/candidate-history rules through ordinary header admission and durable body connection. CVLD-01/02/03/04/06 remain in plan frontmatter and retain all existing executable coverage. This storage observation refinement is a prerequisite contribution, not independent delivery of the consensus rules. Relevant PROJECT goals and later requirement owners remain unchanged; all global CVLD status stays OPEN.

All D-01…D-10 references remain represented in task actions/rationale. D-01…D-06 rule/support/monetary/ancestry/script/time/undo obligations are unchanged. D-07…D-10 retain genuine owners, accepted-state refusal, exact original commands/evidence, independent review and full native/lifecycle/normal-hook saving gates. No deferred wallet, public-default, service/funds, import/repair or actual-daemon/differential qualification enters scope.

Scope stays **41plans/89tasks/34waves**. Existing41 keeps three tasks and explicitly grows from four to five source owners. Its dependencies/wave22 remain accepted38, transitively36;40→39→30 and all later waves remain unchanged and acyclic. Accepted nonminimal schedules remain intact. The four prior owners are inspection.rs,copy.rs,live.rs,live/tests.rs; the fifth is a genuine absent live/tests/refusals.rs. Task2 now declares copy.rs for the classifier fix; Task3 declares copy.rs,parent tests and the new refusals child. Task1 is explicitly historical achieved work GREEN/native RED under its actual61 frames, not a newly repeated RED.

The fifth owner is justified rather than hidden: existing parent416lines plus250–350 missing refusal/work controls and30–50 directory/root controls exceeds628. The new coherent refusal child may reuse ancestor-private fixtures through ordinary Rust visibility. Every owner must remain normally formatted at most628; new child targets620. No packing, suppressions, new production getter, raw authority API or sixth owner is authorized. Actual normal/API/OS-fixture fit is unproved; if it fails, continuation stops for checked refinement. Three tasks/five owners are below the hard five-task/fifteen-owner blocker thresholds; no waiver is needed. The unrelated future C/D/E D draft with five tasks remains blocked.

### Verification dimensions

| Dimension                 | Result  | Basis                                                                                                                                                 |
| ------------------------- | ------- | ----------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1 Requirement coverage    | PASS    | Five mandatory IDs and original rule/integration coverage retained                                                                                    |
| 2 Task completeness       | PASS    | All three tasks have files/read-first/behavior/action/automated verify/done/acceptance; resumed task ownership matches actions                        |
| 3 Dependencies            | PASS    | All41 references exist, graph acyclic, original waves/dependencies exact                                                                              |
| 4 Key links               | PASS    | Shared safety predicate reaches individual/common-entry comparison; private anchor and actual driver observation are wired through existing live gate |
| 5 Scope sanity            | PASS    | 3tasks/5declared owners; fifth-child absence and fit rationale explicit; normal-fit stop gate                                                         |
| 6 Verification derivation | PASS    | Actual typed Changed, full second pair, per-walk hashes/EOF/ledger and actual attempt starts—not aggregate/Boolean evidence                           |
| 7 Context compliance      | PASS    | Existing decisions/refusal/proof/evidence preserved under an explicitly checked contract change                                                       |
| 7b Scope reduction        | PASS    | No Identity/Eq normalization, smaller history, raised limit/deadline, skipped refusal or fake-native substitute                                       |
| 8 Nyquist                 | SKIPPED | workflow.nyquist_validation remainsfalse; no applicable Validation Architecture gate                                                                  |
| 9 Cross-plan contracts    | PASS    | Strict original errors and original inverse provenance preserved; new62 frame is separately classified                                                |
| 10 AGENTS.md              | PASS    | GSD scope, pinned tools/timing lock, normal formatting, breadcrumbs, full native/review/coverage/Bazel/hooks retained                                 |
| 11 Research resolution    | PASS    | Existing resolved research and answers remain whole-byte exact                                                                                        |

Independent gsd-tools plan-structure reports three complete tasks,zero errors/warnings. Independent compatible mdformat check passes all three41 documents. Threat closing tags are standalone, and41PLAN has only its two top frontmatter delimiter lines. No original command string changes. Revision2 changes only the three41 documents from354aa; its Task1/Task3 XML, scope,62 source records and201 engine records remain exact. All24 numeric/lifecycle/per-walk invariants pass.

### Classified directory and anchor contract

The observed defect is specific: diagnostic3 logs whole-inventory common-entry `keyspaces/9`, directorytrue,same dev16777231/inode625671905/mode16877, links5→7,length160→224,mtimechange. Both complete captures succeed; actual native observation changes72→75entries, original stamp remains valid and all9 maps/catalog agree. The current safety predicate then returns Refused on attempt1. This is direct evidence for that diagnostic snapshot; the earlier nine-case selection's exact entry remains unlogged. No namespace name or root-anchor failure is inferred from numeric path9.

The checked change classifies safe DIRECTORY nlink differences as **Changed, never Ok**, in BOTH direct Identity and complete common-entry safety comparison. Type/mode checks retain priority, including simultaneous readable chmod plus directory-link change. REGULAR FILE nlink!=1, symlink/hardlink, unsafe type, permission, resource/arithmetic and nontransient I/O remain Refused. Current from_metadata's regular-file hardlink rejection remains. Full Identity fields and derived Eq—including directory/root nlink,length,mtime—remain untouched. Strict offline wrappers immediately return the same original StorageError and precedence without retry. No error-string categorization or public error type is introduced.

The separate held-root anchor is explicitly refined, not claimed as diagnostic3's cause. It excludes dynamic DIRECTORY nlink alongside length/mtime; it still binds a read-only held directory fd,canonicalpath/dev/inode/directorytype/mode against both fd and fresh path at every boundary. Missing/replaced/symlink/type/mode/security/permanent-I/O root problems remain immediate refusal. Every full Inventory root entry still retains nlink and must stabilize. Non-Unix strict single-attempt/original-stamp behavior stays unchanged.

One original opaque stamp precedes all filesystem I/O and remains through every attempt, Changed transition and final return. No new stamp forgives same-value/ABA/in-flight/foreign/poison/exhaustion. No generation mutex spans I/O, writer pause, raw handle escape, second engine or physical-counter authority is introduced.

The driver stays at most two complete attempts. One external ledger precharges8192visits,1073741824requested content bytes and8192 separate one-byte EOF probes, including failed partial reads, without reset/refund or EOF-to-content transfer. Every capture keeps536870912bytes/4096entries/depth16/path4096 and the same64KiB full hash chain,EOF,opened-file/path checks. No sleeps/backoff/deadline growth, third attempt, normalization or metadata-only fallback is allowed. The partial unreachable! tail must be replaced with a bounded panic-free first-result/one-permitted-second-result shape, not a fallback error or panic allowance.

### Discriminating controls and resolved verification warning

Unrelated strict captures inside the native hook may fail during legitimate churn before the actual driver. They cannot be mandatory successful assertions or acceptance evidence. The revised fixture retains real originally populated native actions and requires the ACTUAL driver's Changed from its full walker/Identity checks, original stamp validity, all9-map/catalog equality and actual full successful second pair. A completed rotation Boolean or constructed Changed is insufficient. Two distinct populated namespaces prepared before the original stamp must cause two actual driver Changed results and refusal after exactly2 attempts,no3rd. Original40 automatic-pressure and305 fixtures remain whole-byte exact and receive no forced maintenance.

Separate attempt-start observations count actual driver entries, not only completed Point::Captured callbacks. An early initial-capture Refused must show1start/0completedcaptures/no second start; pre-driver stamp/root refusal may show0starts. Type/mode/hardlink/readable-chmod/permission/nontransient/root replacement/logical-write/work negatives remain mandatory. In-flight writers release/join before assertions can unwind. These are narrow cfg(test) data observations within the five owners, never runtime authority.

The only unresolved issue in354aa was a verification-derivation warning: aggregate OBSERVED_WORK>=2\*finalBytes can be satisfied by first-attempt work and hide a skipped second recheck. Revision2 explicitly resolves it for BOTH native-success and root-directory-success controls. Actual second-attempt start,capture completion,recheck/full-Eq completion carry cumulative L0/L1/L2. L1−L0 and L2−L1 must separately match their OWN actual inventory entry/content/regular-file counts. Actual completed hash bytes and EOF reads are observed at the real shared operations; probes equal each walk's regular-file count, with nonzero genuine hash/EOF evidence. Full-Eq completion is observed only after the real comparison. First-attempt work stays in L0 and cannot fund either delta. Aggregate-only counters, fake Ok, constructed success events, skipped hash/recheck and metadata-only walks cannot satisfy this proof. No protocol, budget, owner, frame or argv changed.

New actual descendant directory tests must observe real link deltas and require Changed in individual and common-entry paths, with full Eq unequal and strict first-error mapping intact. The held-root control creates and RETAINS a child DIRECTORY through observation, proving actual root nlink change while fd/path/dev/inode/type/mode/stamp/maps remain bound. Full Inventory Changed then a second fully equal pair is required. No portable link-count formula is assumed. This filesystem evidence stays distinct from native primary controls and from the descendant diagnostic cause.

### History and inverse boundaries

Six actual historical61 jobs retain732 before/after copies and1309 references, all authenticated. They include achieved Task1 work GREEN/native RED, the original nine-case7PASS/2FAIL61.32s selection, E0425 zero-test setup, diagnostic2's unrelated hook-capture failure and diagnostic3's0PASS/1FAIL11.49s branch proof948a827b/raw5fa3821f. The diagnosis-confirmed subset independently authenticates four jobs/488copies; that subset is not relabeled as all six. Root HOLDad874ebb remains; no final7 has run, no source CLEAN or acceptance is inferred, and old41/40/39/30 failures remain attributable.

The NEW working frame is62:61 current partial41 inputs plus true new refusals.rs absence. It is not an original inverse baseline. Original4ee360e5 remains authority for inspection/copy BEFORE bodies plus true live.rs/live/tests.rs absences. Fresh a5d4 contributes only the third refusals.rs absence. Final SOURCE inverse must therefore restore2original bodies+3true absences, protect57 current non41 inputs and reproduce62. That source inverse has not run during planning. Current partial bodies cannot replace original before authority.

Future40 uses62/protect60 and all7 fresh final commands;39 uses62/protect58 and all9 fresh;30 uses62/protect52 and all17 fresh, preserving their original inverse authorities. Old61/55/53 frames and obsolete unrun scripts cannot be relabeled. Every new actual job needs its own62 before/after copies; final7 run fresh on one immutable source freeze. Native201 remains separately authenticated, not per-job capture. Historical Task1 commands remain actual61 evidence rather than a re-created RED.

### Remaining root gates

Root publication/fresh-before/release, normal/API/OS-fixture fit, complete actual62-source final7, source/evidence/inverse/security review,SUMMARY QA and bounded41 acceptance remain open. Failure or missing fixture discrimination preserves evidence and returns for checked refinement; no retry-until-green or skipped control is authorized.

Actual41→40→39→30 precedes35FIRST then13/product14/ordinary301–302corpus15/seeded16. Formal C/D/E use actual35 interfaces, D stays under five tasks and E→17 is formal before guards. Canonical final SOURCE review precedes17 guard renewal including123 timing-checker ownership;18 security/native/mutants,19 parity/README/manifest,25 writing-for-agents excerpts and20 entire default native/fresh pure-core coverage/Bazel/source/security/goal/lifecycle precede normal-hook main commit/push. Stop before168.

Material guidance: actual AGENTS, Bright Builds sidecar/overrides, architecture/code-shape/testing/verification/Rust rules, Revision Gate/calibration, unchanged phase context/research/requirements/PROJECT/roadmap/state and captured diagnosed source. Both active lesson files remain the fully read9838byte/3280estimated-token set; no omission,audit,archive or lesson mutation follows this internal refinement.

### Structured issues

```yaml
issues: []
```

Root may independently authenticate and publish the frozen bounded refinement; source execution remains separately gated.



## Plan42 Work-Bounded Live Observation — Independent Checked Refinement

```yaml
status: verification_passed_planning_only
phase: 167-candidate-history-validation-rules
plan: "42"
phase_lifecycle_id: 167-2026-10-21T16-38-23
lifecycle_mode: yolo
commits: false
source_accepted: false
```

# VERIFICATION PASSED

Revision 2 resolves the single prior warning: **0 blockers, 0 warnings, 0 info findings**. This is planning verification only. Root retains publication and implementation release authority; source acceptance, actual test outcomes and compiled fit are unproved.

Proposal: `draft-revision2/proposal-index.json`, 397409 bytes, SHA256 `4c6b3db84defa83dfbff5ce4c9d24cff5128e6ac2efef6688a83df1522aed002`. Paths below are relative to `.local/open-bitcoin-dev/phase167/work-bounded-live-plan/` unless stated otherwise.

The complete new PLAN (35252 bytes, SHA256 `b2852209ac6e9ef8b17c349c3f20f8dd516b8187649ade4518eb045760dc7280`), amendment (22452 bytes, SHA256 `730d9ed20de1e5ac06633f723be57c9dff221e4b43983bfb78bb79412c48776c`), complete 21028-byte precision patch (`04f49fc95e8ede7ade34652e743e9d9c78c6aa7297fb3a76e6afbdf2154ad310`) and prior CHECK were read. Unchanged semantic/source knowledge is reused only after authenticating the frozen prior proposal and checker artifacts. No application or build was run.

## Resolved warning and revision gate

Task 2 now explicitly covers EVERY legacy public-live positive, including one-native change, root metadata, retained root-directory change and both descendant directory-link branches. It requires auditing incidental Captured/start counts outside the shared helper. Exactly the intended injected actions remain: one native rotation, two distinct native rotations once each, and each defined root/directory sequence once. Each action still requires its ACTUAL associated driver Changed observation, original stamp and full nine maps/catalog where applicable.

Only incidental positive scheduling/ordinal counts may generalize to honestly observed additional natural safe Changes. Every success must bind its actual final successful attempt and two complete walks, each with its OWN footprint, ledger and completed hash/EOF deltas; prior failed work stays charged and cannot fund that proof. No aggregate-only counter, mocked result, extra injected action or removal of a real Changed oracle is permitted.

Refused boundaries remain exact: foreign stamp or missing/unsafe initial root has ZERO starts; in-driver refusals retain ONE start and no continuation. Strict offline original-error/no-retry assertions remain exact. Task 1 additionally makes the existing RED evidence ordering explicit: both action-associated Changes, original stamp and complete maps/catalog are asserted BEFORE the expected failing success assertion. Unreachable post-failure assertions cannot count.

The revision changes only the two new42 documents. All other 110 proposed bodies, all ten new42 automated strings, Task 3's complete XML and new42 frontmatter stay exact. The prior proposal `5c5d0373` and CHECK `7124427d` remain frozen history with their original 0-blocker/1-warning HOLD. Revision iteration 1 of maximum 3 reduces the issue count from 1 to 0; no escalation is needed.

## Goal and coverage

Phase167 must deliver pinned network- and candidate-history-derived decisions through ordinary header admission and durable body connection. All five roadmap requirements retain frontmatter and concrete task coverage in the unchanged phase plans. PROJECT's relevant network/history/subsidy/script goal remains represented; unrelated later-phase requirements are not imported. Plan42 is a bounded availability prerequisite, and global CVLD remains OPEN.

| Requirement | Preserved outcome                                      | Plan42 contribution                         |
| ----------- | ------------------------------------------------------ | ------------------------------------------- |
| CVLD-01     | Ordinary subsidy/maturity and original305/306 evidence | Tasks 1–3 physical-observation prerequisite |
| CVLD-02     | Candidate ancestry/deployments/commitments             | Original authority and refusal preserved    |
| CVLD-03     | Actual durable script-flag wiring                      | No script/core change                       |
| CVLD-04     | Shared difficulty/MTP/history authority                | No time, parameter or history shortcut      |
| CVLD-06     | Scoped duplicate/overwrite/undo evidence               | No writer/codec/undo change                 |

D-01 through D-10 retain phase task-action coverage. New42 directly implements D-01/D-07/D-08/D-09/D-10 while preserving the other decisions and deferred scope. Research's Open Questions and Planning Resolutions section is RESOLVED. No wallet, public-network, import/repair, differential or product scope is added.

## Verification dimensions

| Dimension                 | Result  | Evidence                                                                                                       |
| ------------------------- | ------- | -------------------------------------------------------------------------------------------------------------- |
| Requirement coverage      | PASS    | All five IDs and relevant PROJECT goal retained; global closure remains gated                                  |
| Task completeness         | PASS    | Three concrete tasks with files/actions/verification/done; prior warning explicitly resolved                   |
| Dependency correctness    | PASS    | All42-plan DAG acyclic;42 after41;40 adds42; predecessor waves earlier                                         |
| Key links planned         | PASS    | Same complete attempt_once, protected walker/ledger and actual trace/I/O/Eq proof                              |
| Scope sanity              | PASS    | Three tasks/three existing owners; targets320/590/610, hard628 and STOP/refine before expansion                |
| Verification derivation   | PASS    | Observable later-pair success, budget refusal and preserved authority map to actual controls                   |
| Context compliance        | PASS    | Locked decisions retained; no deferred scope or historical-test weakening                                      |
| Scope reduction           | PASS    | No partial pair, reused baseline/hash, fixture shortening or omitted oracle                                    |
| Nyquist                   | SKIPPED | workflow.nyquist_validation is false; all new tasks still have automated verification                          |
| Cross-plan data contracts | PASS    | Full Inventory/Eq and protected APIs exact; future matrices need actual new source versions                    |
| AGENTS compliance         | PASS    | Native workflow, pinned tools, sole scheduler, normal fit, independent review and full final verifier retained |
| Research resolution       | PASS    | Existing resolved research unchanged                                                                           |

## Finite work and discriminating evidence

The unchanged complete attempt_once is called through a Changed-only loop using ONE original pre-I/O stamp, held root and external ledger. Ok and every Refused end immediately; the entire failed pair is discarded. Every continuing Changed has consumed the existing root visit reservation. At most8192 funded Changed iterations and one final denied-precharge start are possible; failed reservation does not increment. The8193 derived bound is not an added attempt cap, ledger peek or double charge. Existing anchor metadata I/O lies outside walker accounting, so no total-syscall or wall-clock guarantee is claimed.

The unchanged shared limits are8192 visits,1073741824 requested content bytes and8192 SEPARATE EOF probes. Per-capture512MiB/4096entries/depth16/path4096, full64KiB hashing, EOF/identity/security/Eq, immediate logical/root/permanent-error refusal and non-Unix/offline behavior remain. Protected copy.rs and inspection.rs stay whole-byte exact.

The retargeted two-native test runs first on accepted41's unchanged driver and must produce meaningful behavioral RED, followed by GREEN under42 with a real later complete pair. Default-ledger real root FileTimes churn must separately exhaust actual unchanged work through the public gate. A precharged8192 terminal-edge test independently proves one final start, original refusal, unchanged counter and no capture/hash/EOF. Neither control can substitute for the other. All23 accepted41 concerns remain; only the declared two-native outcome changes, with incidental positive scheduling clarified as above.

## Independent authentication and inverse

- Independent audit: **4627 passing checks, 669 unique authenticated references**, including R2 and prior frozen reports/copies. Final375 live-state checks confirm112 original planning states,62 sources and201 native/lock inputs unchanged.
- Original planning inverse: own ignored sandbox, strict BSD `patch --batch -E -F0 -p1` forward/reverse/reforward, all112 states checked at each stage. Reverse restores110 original files and TWO true absences; forward reproduces112 exact bodies. No offset/fuzz.
- Precision inverse: separate own sandbox initialized with all112 prior proposed bodies, same strict forward/reverse/reforward and complete112-state checks. Reverse reproduces v1 exactly; forward reproduces R2 exactly. No live patch.
- Exactly15 original planning targets; all41 old task XML/argv and28 accepted PLAN/SUMMARY pairs remain exact. Phase totals become42 plans/92 tasks/35 waves.
- GSD structure passes:3 complete tasks,0 errors/warnings. Scoped GFM/frontmatter-capable mdformat check of both proposal documents passes. Standalone delimiters remain only the top frontmatter pair; threat closing remains standalone.

New42 SOURCE inverse remains future work: root-before `ecf1f7be` records THREE EXISTING accepted41 bodies and59 protected sources, ZERO source absences. Accepted41's older `4ee` inverse is not42's authority. All62 current source bodies and copies authenticate;201 native/lock inputs authenticate separately, with no per-job engine claim.

## Remaining gates

Actual implementation must fit normally in the three owners or stop for checked refinement. Task 3 requires EIGHT fresh final commands on ONE actual62-source freeze: bounded filter, complete live, strict inspection, canonical genesis, generation, seals, format check and strict Node Clippy. Each job needs actual62 before/after copies and exact environment/argv/raw/discovery/outcome/timing; previous greens cannot substitute. Source/API/security/evidence/inverse review and source-first SUMMARY QA precede bounded root42 acceptance.

Only after actual42 acceptance may unchanged40 ALL7,39 ALL9 including original complete305/306, and30 ALL17 run fresh with new source versions and original inverse authorities. Actual pressure/305/safe-reopen success remains unproved. Later35 FIRST, product/corpus/formalCDE, canonical SOURCE review before17 guards,18 security,19 parity/manifest,25 writing-for-agents,20 full default native/fresh pure-core coverage/Bazel/final audits and normal-hook main commit/push remain required. Stop before168.

Repo AGENTS, Bright Builds sidecar/overrides, architecture/code-shape/testing/verification/local-guidance/Rust standards and both complete active lesson files informed this check. Active lessons total9838 bytes/3280 estimated tokens; none omitted. No project skills found. Writes are confined to this ignored checker directory; no source, build, application test, engine, live planning, metadata or Git mutation occurred.

## Structured issues

```yaml
issues: []
```

Planning verification passed. Return to root for publication authentication and any separately authorized implementation release.

## Root checked43 publication after complete failed35 review

Root planning acceptance d815b4e389f64f956059636048df20a90930e5eafbcafb91ee2ff16ddcf2b630 authorizes literal publication of the independently checked and Markdown-normalized three documents only. Complete failed35 Cargo101 remains failed:3393 pass,4 fail,4 inherited ignored; all20jobs/3272sourcecopies/8040nativecopies and original5b0inverse authenticated. Root9dd02564 and separate failed-reviewf3ccc2a8/0676 preserve all four failures, unreached Node integration/rustdoc and unknown original pressure entry/count/stage. Both metadata guards ended naturally and source/build ownership returned ROOT. Candidate44/45 BASIC/peer corrections remain unaccepted planning work.

Independent v3 closesP43-01/P43-02 with2518checks; precision normalization adds41checks, preserving all Task XML/argv and standalone threat close. Earlier HOLDs and the rejected first normalization remain immutable. All source/runtime/fit proof is pending; actual43 needs a fresh source/true-absence frame and separate root execution release. Global CVLD IDs stay OPEN; no35Summary/acceptance, Git or168 work is released.

# Plan43 revision v3 independent check: PASSED

**Candidate verification PASSED: zero blockers, zero warnings. P43-01 and P43-02 are closed.** This is planning approval only. Publication, application/source execution, Summary, source acceptance, global requirement closure and Git remain unreleased.

Candidate index18734B SHA256 `ec5daf4a471d7e88b1aa58317409e5cec1b585da6b59ffd9982dc717e72a8657`; final plan35610B SHA256 `74ba8b1829ce277fc24c9525c28cc1c5d1db5868e035be384dd24b4a268fb895`; amendment21582B SHA256 `404f839ced5b3142990801f7ea313d7009b5d8281dcb2624255bcad9f1a13b8e`. Generated_at `2026-10-23T18:04:31.755703Z`; lifecycle `167-2026-10-21T16-38-23`, yolo, commits:false. Earlier sealed candidates and independent HOLDs remain immutable historical records.

## Closed findings

| Finding                                                        | Final resolution                                                                                                                                                                                                                                                                                                                                                                               | Verdict |
| -------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------- |
| P43-01: original35/composite source reversal authority         | Original5b0 exclusive direct11 before states remain eight original bodies/three historical absences. Future accepted43 replacements/new children get explicit actual per-path CORE/root source-receipt provenance; conditional84/73 membership is derived. Whole-file scratch inverse preserves accepted43; root reviews manifest/proof before separate35SOURCE release and all nine commands. | Closed  |
| P43-02: pending-zero safety errors replace old strict original | One actual unchanged strict constructor invocation yields the original regular-zero hardlink StorageError. The same value is moved into either Changed(safe) or Refused(hazard), while all guards remain. Other fstat/nonzero/nonregular branches retain old errors. Every unsafe mode/replacement case requires full typed exact original, once and no live continuation.                     | Closed  |

The revision explicitly orders actual43 CORE/root source acceptance → prepare/authenticate composite manifest and complete scratch inverse/forward proof → independent root review → separate fresh35 SOURCE release → all nine fresh byte-exact commands. Preparing that proof needs no35 release. This avoids circular release requirements and does not authorize working-tree reversal.

## Meaningful independent evidence

`checks.json` contains2518 independent checks with zero mechanical failures. The reviewer authenticated the final v3 index/inventory, reused immutable v1/v2 inventories and prior independent HOLDs, before references/copies,82 observed source and201 pinned native states against live protected inputs, original5b0 capture, eight exact copied direct before bodies, three historical absence records, and composite per-path provenance/set union/disjointness. Historical absence does not assert current absence.

All42 old candidate plans are unchanged from v1/v2, preserving92 old task bodies,195 automated command bodies, all fenced bodies and old waves. Only the already approved original35 dependency append differs from its original.43 retains three tasks/four owners, unchanged Task1, all automated argv and fenced interface/final-command blocks.43 plans/95 tasks/maxwave35 are acyclic; every dependency exists and has an earlier wave, with no new same-wave owner conflict.43 depends on accepted40/42 at wave25;35 remains wave27.32 accepted contributions remain historical, not new source acceptance.

The44-target whole-file planning inverse independently reconstructs forward→original reverse including new-plan/amendment absences→forward byte-exact in memory. This is a planning inverse only. Actual43 source inverse and later35 composite source proof remain future execution gates.

## Semantic and security result

The final delta and complete227-line Identity layout were read. The layout now calls `Self::from_metadata(&metadata)` once at the opened boundary and matches only `Err(original)` for actual Unix regular nlink0 into the contextual helper. It never creates or normalizes a zero Identity. Both private outcomes consume that same original error. Same-held regular identity, prior links1, dev/inode/full mode, path validation and strict current-path checks remain; a failed guard selects Refused, and only safe contextual disappearance selects Changed. Actual>1 links, unknown/path zero, symlink/directory/socket, changed mode and permanent I/O refusal obligations remain intact. Nonzero/nonregular/metadata-unavailable branches preserve the old path and original error.

The exact-error expectation is feasible without access growth: StorageError derives PartialEq/Eq; inspection_error constructs BackendFailure with Runtime namespace, exact message and Restart action. The plan requires full typed equality, not Display/Debug string comparison. Fresh typed/strict/live fixtures for mode-plus-unlink and EVERY unsafe replacement assert actual held regular nlink0, Refused, exact original once and no live retry; safe unlink/different-inode cases retain Changed and strict original-once. Guard errors choose category and cannot replace the retained public error or be treated as success.

Task1 still requires a compiling, executed production-path RED at the existing Opened hook after actual initial FD validation, before digest. The four-byte payload must demonstrate current Refused versus intended Changed and exact nonrefundable ledger(2 visits,4 requested content bytes,1 separate EOF). OS-only probes, missing API, compile failure or injected CopyFailure cannot count as behavioral RED. All eight named new tests must execute in both final-unlinked and final-live; names alone are not execution evidence.

One-time live convergence must discard the failed attempt and earn a fresh full capture/recheck/hash/EOF/Eq pair using the original stamp/held root. Repeated unlink must exhaust the unchanged cumulative ledger honestly. All seven Identity fields/full Eq, same original pre-I/O stamp, held readonly root anchor, all nine maps/catalog,4096 entries/512MiB/depth16/path4096 source caps,8192 visits/1GiB requested content/8192 independent EOF counters, no work refund and30s child deadline remain. Completed I/O and denied precharges must be distinct.

Original4096×64KiB pressure source remains byte-exact, with restoration, stale original stamp/seal, strict physical checks and full-drop safe reopen required. A remaining pressure or work-limit failure is a HOLD; no pressure/wait/retry/cap/deadline/fixture waiver is allowed. Nine final commands retain unlinked, aggregate live OSIO/security/work/stamp, strict offline copy/worker/lock/deadline, genesis, generation, logical seals, unchanged pressure, format and strict Node Clippy coverage. Complete outcomes and actual discovery, including every ignored reason, are mandatory.

Conventional private child/re-export preserves inspection-only Identity visibility and copy-only member access; no new public or test-access API or fifth owner is granted. Layout463(copy)/227(identity)/618(registrar) is syntax/normal-format feasibility only. The actual new test child remains unimplemented and\<=620 fit unproved; all files\<=628 and function refactoring around161 remain explicit checked stop gates. No compiler/runtime credit is inferred.

## Verification dimensions

Requirement IDs CVLD-01/02/03/04/06 remain present and globally OPEN. The correction supports retained D-07–D-10 authority/evidence/workflow duties while preserving all other decisions across unchanged plans. No deferred qualification, wallet or public-network scope is introduced. Three tasks have concrete files/actions/verification/done, all critical key links are planned, scope is bounded, and must-haves trace to observed behavior. Cross-plan original/composite contracts are explicit. Repo conventions, full native verification and normal-hook/lifecycle requirements remain. Research open questions remain RESOLVED. Dimension8 SKIPPED: nyquist_validation=false.

Classification diagnostics may be limited to strict constructor refusals before propagation/private classification, using the same already captured metadata. The candidate does not require tracing every successful metadata lookup merely because the inert sketch does so. Refusal-only emission retains stage/path-kind/known/nlink evidence with the same bounded safe escaping and no extra stat, while avoiding unrelated full-suite log growth. This is an implementation choice within the checked contract, not a layout-literal requirement or new waiver.

## Gates retained after this PASS

The actual original full35 failed file/link count/stage remain UNKNOWN. Mechanism proof is not actual occurrence attribution. The prefix is not the closed full-job result. This review does not relabel the pressure failure or credit its unreached safe reopen. Existing opt-in ignores retain their recorded meanings and must not be hidden or enabled to manufacture a clean result.

Publication still requires the complete failed35 command to finish naturally, its full packet to freeze and receive root review, and a separate root publication decision. Actual43 execution then requires fresh before/true-absence evidence and a separate source/build release after current jobs end. Independent actual43 source review/root acceptance and the original/composite proof gate precede any resumed35 release.

All remaining13/14/15/16, formalC/D/E (D\<5 tasks; E→17), final source review before17 including123 ownership,18 security/native/mutants,19 parity/README/manifest,25 agent-writing,20 full native/fresh core coverage/Bazel/independent source/security/goal/lifecycle and normal-hook main commit/push gates remain open. STOP168. No Summary, source acceptance or global CVLD closure follows from this candidate PASS.

Guidance materially used remains repo AGENTS/Bright Builds sidecar/overrides, relevant architecture/code-shape/testing/verification/Rust standards, complete active global/repo lessons, locked context, roadmap/research and GSD revision references. No project skills were found. Only new ignored independent-check/v3 artifacts were created. No application/Cargo/Bazel execution, failed datadir access, active session use, canonical/source edit or Git action occurred.

```yaml
issues: []
resolved:
  - id: P43-01
    status: closed_in_candidate_v3_retained_from_v2
  - id: P43-02
    status: closed_in_candidate_v3
status: verification_passed_candidate_only
publication_released: false
application_source_execution_released: false
```

# Publication-copy precision: PASSED

**PASS: zero blockers/warnings;41 independent precision checks pass.** This checks only the three normalized publication copies bound by root-publication-ready-v2/index.json3018B SHA256 `b9ebfbad44ea10f727cb9efec3e0417f6bee8ca8c49dba5c3ae34ff2fc41810b`. It preserves the independent v3 PASS1906B SHA256 `8d9cf37ec30d0bb4110d1896463017ea74a35467eceaab1edcf35604409ce5aa` and its2518 checks; those checks are not recounted as new execution.

| Publication copy                  | Bytes | SHA256                                                           |
| --------------------------------- | ----- | ---------------------------------------------------------------- |
| 167-43-PLAN.md                    | 36526 | f2b301cb86b0162ea4fd84dbf9650481153c916847411131c783db6c5d589fc7 |
| 167-43-OPENED-UNLINK-AMENDMENT.md | 24417 | ba5b2f37fac1a08ec58369131b1ed30f40700a7aa76516fcc52e59cf1cbc7e8d |
| 167-35-PLAN.md                    | 26691 | fa38bbbdcfc101f980135dc1c1aad50f0c70173b72641bbb374f73512a34a677 |

All three hashes and their sealed v3 source hashes authenticate. An independent text comparison allowing only table separator/alignment whitespace, Markdown escapes of literal< and\*, ordered-list marker normalization, the blank line before the standalone closing threat tag, and the declared amendment status change is exact. Full diffs were read. Amendment frontmatter changes only to checked_prerequisite_pending_execution; other frontmatter is exact. Old35 is byte-exact to the approved dependency-only candidate.

All three43 task XML bodies, automated argv and all fenced code/interface/final commands are byte-exact to approved v3. GSD structure validates all three tasks. Configured mdformat GFM/frontmatter check passes for all three copies. The plan has exactly one standalone closing threat-model tag, with a separate blank line before it; it has not become a GFM table row. Only the two top frontmatter delimiters remain.

P43-01 original5b0 direct eleven-owner/eight-body/three-absence authority, accepted43 protected new-child provenance, conditional84/73 counts, inverse protection and linear root-proof-review-before35SOURCE release remain unchanged. P43-02 single saved strict constructor error for either private category, full safety refusal precedence and exact typed original/once/no-retry assertions remain unchanged. Source84 is prospective conditional membership; this normalization confers no source acceptance or runtime evidence. All caps, identity/fullEq, root/stamp, pressure, timers, real OS RED/GREEN, eight names, nine final commands and fit gates are retained by exact task/text comparison.

The earlier root-publication-ready normalization is explicitly held and excluded; only root-publication-ready-v2 copies above pass. Sealed v1/v2/v3 and their prior HOLD/PASS artifacts are untouched. This check does not independently re-audit the complete failed35 packet: root separately reports that packet closed, independently audited and root-reviewed. The original failure remains failed and its original entry/link count/stage UNKNOWN.

Root may use these literal copies for its separately authorized publication decision. A fresh actual43 before/absence frame and separate source/build execution release still follow publication; no source, Summary, acceptance, CVLD closure or Git action follows from this precision PASS. All later phase gates and STOP168 remain. No canonical file, source, fixture, active session or Git state was touched; only this ignored publication-precision directory was written. Existing repo guidance and the prior independent v3 assessment remain applicable.


## Root publication: checked repair plans44/45, source execution pending

The frozen R3 planning review and corrected publication precision check below passed. Root accepts the four-task/eight-owner warning in principle; actual normal source/test fit, compilation, behavior and separate source/build releases remain mandatory. Plan43 remains held on the fresh fixture NotFound failure. All global CVLD requirements and later Phase167 gates remain open; stop before168.

# VERIFICATION PASSED — frozen R3 candidate repairs 44/45

**Verdict: PASS, planning only. Blockers: 0. Unaccepted warnings: 0. Recorded scope warnings: 1.** Four tasks/eight owners are accepted in principle by root; actual normal-format implementation, test fit, compilation and behavior remain mandatory future gates. No source or execution acceptance follows from this review.

Lifecycle `167-2026-10-21T16-38-23`, yolo, `commits:false`. Frozen R3 index: 95,963 bytes, SHA-256 `a33abf68144456b6967da6001dcb2565654dc691742d7d043e9cdddfb91b51d5`. Plan 45: 46,472 bytes / `57117752b7839f50003ab44b5a303ffc4dd0b09ad36811f520ba684a2fd2ebce`. Shared amendment: 34,523 bytes / `8277c9fab2b35a8119ba48ac9f0eae36f9b2af1663dfbdfe24202cef9cffd7a0`. Plan 44 remains wholly byte-exact to corrected R2: 15,181 bytes / `d56b1f67a93c309f1ae0c3309d66d668e629de4be88abd4a532e72471bb1b5da`.

The complete R3 plans, private interfaces/all four 45 tasks, amendment, revision map, caller audit, handoff, caller inventories, extraction/layout and test budget were reviewed with the prior R2 PASS, later inert HOLD and the original RCAs. Local AGENTS, Bright Builds sidecar, overrides and architecture/code-shape/testing/verification/Rust standards and the previously fully loaded active lessons govern this check. No project skill directories exist. The later inert source HOLD is source-derived, incomplete and uncompiled; it is neither compiler failure evidence nor a source pass. Prior candidate/review history remains intact.

## New source-interface findings addressed in R3

### Concrete private type access — CLOSED IN PLAN

`network.rs` privately declares `runtime_authority` and explicitly re-exports selected types. R2's named lifetime/observation types exported only inside that child are not nameable by sibling sync modules. R3 explicitly owns `network.rs` and requires named `pub(crate)` re-exports through the authority parent and network root. Sync imports `crate::network::{SyncPeerLifetime, SyncPeerObservation, SyncPeerRequestAuthority}` through that legitimate path. The module stays private; no lib.rs, external API, broader mutable accessor, inferred/boxed/type-erasure/TLS workaround or test-access API is permitted.

To fit the actual 628-line network root, the existing raw `ManagedPeerNetwork::connect_outbound_peer` may relocate unchanged into the already owned authority child. Its inherent type, public signature, complete body, existing generation/helper calls and caller access remain required. The child's position is a descendant of the raw owner's module, so ancestor-private helpers need no new visibility. Mechanical relocation is an honest source-layout feasibility check; the completed implementation must still compile in ordinary production consumers.

### Validation through actual request/accounting effects — CLOSED IN PLAN

The existing generic helpers accept a peer ID and acquire separate handle operations. Validating an original dispatch and checking ownership only after a later unbound handoff would permit remove/re-add to change a replacement peer before denial. R3 explicitly owns `block_reconcile.rs` and the conventional child `block_reconcile/peer_requests.rs`. **Every actual Session path**—initial missing requests, Idle global expiry plus handoff, post-dispatch batch handoff, unrequested receipt fallback and final scheduling—must use the shared token-bound entry point.

The named non-Clone request projection has a private constructor/fields and a higher-ranked guard lifetime; the callback return type cannot retain that guard. Under the existing operation mutex, validate exact shared authority/registry/epoch/nonce/current membership before invoking the callback. All actual request/local-known/timeout mutations, target/GetData membership and cap validation and runtime in-flight insertion occur inside that same validated scope. A stale/replacement denial precedes the callback and all such effects. No raw mutable network, arbitrary peer parameter or locking-handle escape is exposed. The projection delegates directly to the held raw owner, avoiding reentrant `ManagedNetworkHandle` calls, an extra lock, I/O, await or transport operations while held. A handle clone before mutable runtime borrowing is allowed; it does not weaken the lock boundary.

The existing generic APIs keep their signatures/effective `crate::sync` scope through child definitions and parent re-exports. Static generic and bound adapters use the same selection/request/batch-validation core, preserving error text/categories, empty batches, order, caps and partial effects. Protected logic may be relocated/shared, not silently duplicated or narrowed. `release_unrequested_hashes` and `request_cleanup_result` remain byte-exact and outside the request scope. Their existing all-peer membership observation is not an arbitrary future-concurrency theorem.

The actual raw compact-expiry operation performs global in-memory partial/request cleanup, targeted sends and cleanup metrics. R3 validates the session before running that tick and explicitly preserves valid-token cross-peer effects, foreign-target rejection and original partial-effect/error order. It does not silently turn global expiry into peer-only expiry or waive an actual UnknownPeer/operation failure. Actual protocol-triggered removal belongs to the separate typed dispatch/observation/finish path.

### Preparation and later storage failure — CLOSED IN PLAN

Chainstate/header/store reads stay outside the token-held callback. One shared selector uses a static known-block callback: the generic adapter still calls the original fallible handle notification at its original per-item point. An actual earlier notification must not disappear merely because a later storage lookup fails.

The bound Session adapter instead carries its actual encountered known-hash prefix plus `Result<selected hashes, original preparation error>`. After a legitimate exact-token commit, replay the earlier raw in-memory notifications in order, then return the actual later preparation error before issuing any request or runtime grant. The raw `note_local_block_hash` is in-memory/infallible; the generic wrapper's authority failure remains at the original callback point. Stale/unavailable commit applies no prefix/request/runtime write and preserves both a genuine preparation error and denial through the unchanged primary/secondary error-join contract. No fabricated successful notification, suppressed first error or storage I/O under lock is permitted.

A required real regression stores a known earlier candidate and a later corrupt body that produces an actual loader error. It must prove the authorized partial effect and exact error, plus replacement-state preservation and both genuine errors on stale denial. This is a future behavioral requirement, not a test executed by this checker.

### Physical charging versus peer attribution — CLOSED IN PLAN

Physical receive/decode/message/byte/resource bounds remain charged at actual I/O, including malformed and stale-denied input. R3 does not move the bounded receive loop behind authorization and create free I/O. Peer `record_activity`, last-activity/contribution, success and capabilities follow the actual original-lifetime dispatch outcome. An authorized inner protocol error retains real received-message activity/error semantics, even if a subsequent cleanup/BASIC result fails; outer denial cannot credit a replacement. Success attribution waits for the required successful authorized processing/cleanup. A prior token observation does not authorize a later denied dispatch. The new real controls distinguish these paths while retaining malformed/resource error visibility and old physical bounds.

## Prior R2 obligations remain complete

- Earned bounded request observations are reconciled after authorized dispatch on inner Ok **and** protocol Err, before replacement and at legitimate finish. A helper Ok can leave a still-owned hash, so the latest set remains retained until authorized replacement after another reconciliation or completed final teardown. Result conversion/BASIC-intent errors cannot discard it. Present finish separately reconciles retained and fresh sets; authorized absence skips repeated UnknownPeer operations. Initial/Idle/post-dispatch accounting issuance is observed before send/cancel/return. Actual issued batch candidates cover legitimate removal between handoff and renewed observation. At most two bounded sets coexist; no historical union or global in-flight reset/scan.
- Stale/foreign/replacement/poison denial grants no pending or receipt cleanup and leaves unresolved runtime slots conservative. Genuine primary/cleanup/both errors remain visible. Legitimate already-absent finish is typed success and preserves the original Compatibility error instead of manufacturing generic Network through a second disconnect.
- All three real admission surfaces, policy direct-add/error-after-membership, failed duplicate admission, preexisting peers, shared authority/registry/epoch binding, checked non-reused nonce/exhaustion, current-membership pruning and authority-first registry lock order remain required. Existing effect generations and wake/error ordering stay unchanged. No lifetime-growing map, public constructor, test bypass or new dependency is introduced.
- Real session regressions retain actual compact-only request → existing timeout → actual GetData handoff/runtime registration → valid all-prefilled CompactBlock **and separate partial/BlockTxn completion** → later dispatch/immediate or absent finish. Other-peer ordinary shared/different requests, slots, order/caps remain exact. Real remove/re-add controls cover each initial/Idle/post-dispatch scheduling boundary, plus physical charging, partial-prefix storage failure, foreign/ABA/poison and all old send/cancel/cap cases. No arbitrary slot insertion, fake ReceivedBlock, copied category or unaccepted 35 fixture helper is a substitute.
- Corrected Plan 44 retains actual loader/type/re-export anchors and empty positions plus `Some(empty counts)` for absence, distinct from unknown counts. Its original 401 positions, intent 20, checkpoint 1, MissingMetadata category and all old payload/index/undo/intent checks remain. Exact complete key/colon family removal preserves unrelated chainstate/undo; actual segmented/legacy-helper boundaries and full nine-map post-tamper versus refusal equality remain required. No production codec/startup/parent or visibility change is allowed.

## Explicit eight-owner scope and feasibility warning

| Owner                                                          | Responsibility                                                                          | Before state                        |
| -------------------------------------------------------------- | --------------------------------------------------------------------------------------- | ----------------------------------- |
| sync/session.rs                                                | Actual bound integration, cohesive receive/authorized-outcome/finish split              | Existing                            |
| sync/block_reconcile.rs                                        | Conventional child and unchanged generic APIs/re-exports                                | Existing, latest accepted writer 34 |
| sync/block_reconcile/peer_requests.rs                          | Shared generic/bound request core and bounded observation/finish logic                  | TRUE absent                         |
| network/runtime_authority.rs                                   | Registry init/Clone, command child and named type exports                               | Existing                            |
| network/runtime_authority/peer_sessions.rs                     | Opaque lifetime, commands and scoped request projection; exact raw admission relocation | TRUE absent                         |
| network.rs                                                     | Legitimate crate-private exports and exact raw admission relocation                     | Existing, HEAD-exact/unowned in 167 |
| sync/tests/peer_resolution/peer_protocol_failures.rs           | Exact original 198-line prefix plus private setup/helpers/registrar                     | Existing                            |
| sync/tests/peer_resolution/peer_protocol_failures/lifecycle.rs | Real lifecycle and scheduling/accounting regressions                                    | TRUE absent                         |

The old proposed `sync/session/peer_requests.rs` was never created and is excluded, not silently removed as an implementation change. No ninth owner or hidden include is permitted. All eight abbreviated paths are under `packages/open-bitcoin-node/src/`.

**Recorded scope warning F35P45-02:** four tasks meet the warning threshold; eight files are at the target file-count boundary. Root explicitly accepts this allocation **in principle only**. Tests/typed authority/shared integration/final verification form the four cohesive tasks. Measured extraction gives network 618, authority parent 589, block_reconcile 502, and extracted generic helper child 122 lines; these are syntax/layout-only, not complete implementation fit or compilation.

The disclosed new-test estimate is 607 lines, exceeding the previous 422-line append budget by 185. The now explicit lifecycle child and private parent helpers split the same work: prospective parent 315/target 350 and child 492/target 580, child hard maximum 620, functions around 161. Original 198-line prefix remains exact. Actual code must preserve every required control, normal whitespace, conventional modules and function shape and fit production 628/test 620/narrower stated targets or stop for checked refinement. No packed source, blank deletion, exception, test API, omitted scenario or artificial task split resolves a failed fit. This warning remains an execution constraint; this PASS does not close actual-fit evidence.

## Independent evidence and plan checks

The renewed checker passed **2,509 assertions over 1,592 unique authenticated paths**. It independently authenticates all 377 R3 inventoried files, all 357 R2 candidate files and all 52 prior independent PASS files, plus original candidate/HOLD history and other referenced evidence. Current observations number 55 and separately identify the true new absences; changing 43 source is excluded. The independently derived **131 total / 120 protected** union is planning-only, disjoint from the 11 original direct 35 owners. Actual accepted membership must be derived again from real accepted source/children/causals.

Independent static source-layout comparison additionally verifies the complete raw outbound method before/after relocation, all three existing generic request helper bodies after only effective-scope syntax substitution, and the full original test prefix. Actual HRTB adapter/registry/session code is not present in that mechanical layout and no compilation or behavioral credit is claimed. GSD structure checks independently pass again; all task file/action/automated verify/done fields are present. The full literal focused/final matrices and selected test discovery requirements remain unchanged.

All 43 old task XML sequences, verification argv and fences are byte-exact. Their before-images match actual published canonical planning, including publication `af513afc…`, root acceptance `d815b4e3…` and normalized 43 plan `f2b301cb…`. Only old 35's added 44/45 prerequisites and explicit composite appendix change. All other old bodies remain exact. The 45-plan graph is acyclic and source-owner chains ordered: 44 depends accepted 28/29 at wave 14; 45's existing 34 prerequisite already covers the latest session/block_reconcile writer and earlier 10 ancestry, and the new network root has no conflicting declared 167 writer. 45 remains wave 18, 35 remains wave 27; neither repair depends on unaccepted 35. No gratuitous dependency is needed. Independent scratch planning inverse/forward restores full before metadata/new candidate absences and complete revised candidates; this is not an application-source inverse.

All five roadmap CVLD IDs and all D-01–D-10 decisions remain fully owned. These corrective safety/evidence prerequisites do not independently complete all global requirements. D-07–D-10 are explicitly implemented here; other full phase owners retain D-01–D-06. No deferred feature or scope reduction was introduced. Research open questions are marked resolved. Nyquist is **SKIPPED** because configuration explicitly disables it; meaningful scoped automated commands, source review and whole-phase verification remain required. Repo-local GSD, ownership, formatter, no-commit and normal-hook contracts remain satisfied at planning scope.

## Release and whole-phase gates still open

The candidate remains frozen/unpublished. Any root publication-status/header normalization belongs in separately reviewed ready copies and must preserve all task XML/argv/fences and technical obligations. This reviewer has not modified the candidate, canonical metadata, application source, STATE, Summary, Git, worker sessions or any native/failed fixture, nor executed Cargo/Bazel/application tests.

Actual 43 source is neither accepted nor invariant here. Its independently running verification status grants no acceptance to this tail. Actual 44/45 source implementation needs fresh exact eight-owner/causal/absence/native frames, separate root execution release, honest original behavioral RED, all retained/new runtime cases, final commands, complete source inverse/forward and independent CORE/root acceptance. Selected GREEN or inert layout cannot substitute.

Before resumed 35, original `5b0b899b…` remains the **exclusive** direct-before authority: eight complete bodies and three TRUE historical absences. Actual accepted 43/44/45 repair/new-child bodies and every causal path require complete per-path copy/bytes/hash and real CORE/root or truthful HEAD provenance. Derive actual union/counts; do not use old 82, conditional 84/73, R2 126/115 or this 131/120 as accepted membership. Keep 201 native inputs separate. The scratch source inverse restores only original direct 35 8/3 states while preserving every accepted repair; root independently authenticates it before a **separate** fresh 35 SOURCE release. All original six focused plus three final commands then run fresh.

Original full 35 remains failed: seven prior targets passed (402/8/3/2/695/2/2), eight total reached including Node 2,279 passed / 4 failed / 4 ignored; later Node integration/rustdoc were unreached. Preserve complete raw outcomes/ignored reasons/parser limits and all historical source frames; no prior pass or selected rerun earns fresh credit.

Observer predecessor, actual 13/14, formal 15 helper refinement/ordinary 301/reopen 150/301/next 302, seeded 16, compact C/D/E with D fewer than five tasks and E→17, final complete source review before 17 including 123, native/security/mutants 18, parity/breadcrumb/README/architecture 19, truthful PROJECT/AGENTS 25, entire default native/fresh pure-core coverage/Bazel/independent source/security/goal/lifecycle 20, normal hooks and final main commit/push all remain mandatory. All five global CVLD requirements stay OPEN. STOP before 168.

## Structured result

```yaml
status: passed_with_recorded_scope_warning
scope: frozen_R3_candidate_planning_only
blockers: []
closed_in_plan:
  - "Named private type access: explicit network.rs ownership and legitimate crate-private concrete exports."
  - "Scheduling lifetime: one token-validated operation scope covers every Session request/expiry/batch check/runtime grant."
  - "Generic partial effects: per-item callback order retained; bound known prefix and genuine later error preserved."
  - "Physical versus peer accounting: actual wire charges retained; peer credit follows real dispatch authorization."
  - "Test ownership: explicit conventional lifecycle child and private parent helpers preserve the original prefix."
issues:
  - id: F35P45-02
    plan: "167-45"
    dimension: scope_sanity
    severity: warning
    disposition: "Root accepts four cohesive tasks/eight explicit owners in principle; actual normal-format source/test/function fit remains mandatory."
    fix_hint: "Stop for checked refinement if actual fit fails; no ninth owner, omitted control, packed source or artificial split."
source_execution_released: false
source_acceptance: false
summary_released: false
all_global_cvld_open: true
phase168_held: true
```

# PASS — corrected R3 publication package v2

Ready-v2 index: 32,574 bytes / SHA-256 `0fde604fa080d1259ee22b426d3cd7342a5c6301e5c1aa0616a3a12b92287d81`. Frozen R3's separate planning PASS remains unchanged (`cde26fa7…`). **Zero new issues; publication precision PASS only.** No source execution or acceptance is granted.

The earlier ready-v1 package (`44ce7a46…`) is **HOLD, superseded and unpublished**. Its preserved independent precision review checked all technical/F0 bodies but formatted only the three new documents, omitting amended Plan 35. Root found an extra appended blank there. This new checker independently reproduces that exact ready-v1 Plan 35 formatter failure and records the earlier review's incomplete formatting scope without rewriting it.

Ready-v2 differs from ready-v1 in exactly one body: Plan 35 loses one extra appended newline immediately before the new composite tag. Its original body and all task XML/argv/fences stay exact. All other 46 ready bodies are byte-identical. The previous exact publication-only status normalizations remain the sole other changes from frozen R3: checked-plan objective sentences, amendment status/title/first sentence, all still requiring a separate root execution release. No technical scope, owner, task, dependency, command or evidence obligation changed.

Independent verification passed **580 assertions over 96 unique authenticated paths**, plus the separately saved ready-v1 failure/delta closure. All 47 metadata bodies, patch/script/result and before/after references authenticate. Exactly four canonical paths change: existing 35 and the three genuinely absent new metadata paths. All 44 existing canonical before bodies remain exact. Independent owned scratch ran actual BSD `patch --batch -E -F0 -p1` forward/inverse/forward, matching every complete file and membership after each step, restoring the three new absences on inverse, with no offset/fuzz/reject/failure.

GFM/frontmatter Markdown check now passes independently for **all four changed ready bodies, including 35**. This closes the concrete publication-format HOLD. The frozen R3 planning PASS and its recorded four-task/eight-owner actual-fit warning remain unchanged. Canonical metadata, source, STATE, Summary, Git, application sessions and native/failed fixtures were not modified or accessed by this audit. Root must refresh before-images if canonical planning changes before application. Actual source acceptance, composite inverse/root review, separate fresh 35 release/all nine gates and every later whole-phase gate remain required. All global CVLD requirements stay OPEN; STOP before 168.


## Root checked Task3 fixture refinement: source release remains separate

Technical review passes; corrected V3 closes PF43-01. Only the existing fixture owner is promoted: original three-body/two-absence authority,79 protected/same84 union/201 native. Original Task1/2, pressure corpus, all nine final commands and every later gate remain. No source/runtime acceptance follows.

```yaml
phase: 167-candidate-history-validation-rules
plan: "43"
reviewed: "2026-10-23T20:20:31Z"
scope: frozen_candidate_independent_plan_check
status: publication_format_hold
technical_verdict: pass
technical_findings: 0
publication_findings: 1
commits: false
source_execution_released: false
runtime_accepted: false
```

# Frozen fixture refinement independent check

The technical plan passes independent review with no technical finding. Publication of this frozen version remains **HOLD** because the configured Markdown formatter rejects the two changed documents. A separately sealed, checked publication copy may resolve that presentation gate; this report does not authorize canonical publication, source changes or applications.

## Exact frozen scope

This verdict applies exclusively to `fixture-observation-refinement/v1/FINAL-INDEX.json`,6132 bytes, SHA256 `257c7d3c9442d26704c12bc94b7d94606460e93002fae7c021dc2211f2aed60c`. Its authoritative43PLAN is43406 bytes/SHA256 `904e288a5d06d7f13cd0a495db18a65e7cf90a2083edc2265c0549251d3ca9ec`; fixture companion28435 bytes/SHA256 `0b65a790a1cec59df0bd8f58e2c2fb70a35472a8564b11665c87106b070696c1`. The24417-byte historical OPENED amendment remains SHA256 `ba5b2f37fac1a08ec58369131b1ed30f40700a7aa76516fcc52e59cf1cbc7e8d` and is unchanged.

Whole diagnosis17272 bytes/SHA256 `03726f5f13fa54aca54829098681508aab01ea6f6c16205996d6ed7263032fba`, its complete11949-byte packet/SHA256 `8e45da6ed09f50ee4ff572f2c0aa009c471fc079deb3222bc75aa59d7e8b9595`, the complete original251-line fixture7bcc body, whole new companion and43Task3 were read. The retained original Task1/Task2 contracts were previously read completely and independently compared byte-exact here. No new source code, compilation, Rust formatting or application result is asserted by this planning review.

## Plan dimensions

| Dimension                         | Result                   | Evidence                                                                                                                                                                             |
| --------------------------------- | ------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Requirement and decision coverage | PASS                     | D-07..10 implementation/verification is explicit, all prior decision/requirement obligations retained; five global CVLD IDs remain open.                                             |
| Task completeness                 | PASS                     | Exactly3 tasks; Task3 explicitly TDD implementation plus verification, with files/behavior/action/verify/done and real compiling RED/GREEN.                                          |
| Dependencies and ownership        | PASS                     | 43 retains wave25 and dependencies40/42; new opening test owner is supplied by40, with no new same-wave conflict. Other44 plans remain exact.                                        |
| Typed failure boundary            | PASS                     | Changed only for proved named-descendant metadata/read_dir NotFound; root, unknown iterator, unsafe and permanent errors stay terminal.                                              |
| Funding and control flow          | PASS                     | Precharges, one closing root check, initial1/producer16 attempts and derived6001 retirement slots under unchanged timer/cadence; no partial success or refund.                       |
| Genuine verification              | PASS                     | Five deterministic observer names plus original native pressure case share unchanged selector; two explicit supplemental RED/GREEN commands; all nine final commands must run fresh. |
| Source and reversal authority     | PASS                     | Original43four records plus only original opening0075 promotion;3 bodies/2 true absences,79 protected,84 union and201 native; separate planning/source inverses.                     |
| Scope and feasibility             | PASS with execution stop | One existing fixture owner only, normal620 target/628 stop, functions around161; no production API/dependency or sixth owner. Actual fit/compile/runtime remain unproved.            |
| Publication verification          | HOLD                     | GFM/frontmatter mdformat check fails changed PLAN and companion; protected old PLAN baseline and unchanged OPENED amendment pass.                                                    |

## Technical rationale

The observed failure is a fixture metadata-walk NotFound before live inspection. The exact failed entry/syscall remains unknown; the plan correctly distinguishes source-supported race mechanism from actual occurrence attribution. It does not convert the fresh failure into classifier evidence or retroactively replace old failures.

Complete and Changed remain distinct types; no partially built Physical, empty success, skipped-entry complete walk or stale journal vector can prove retirement. Only a name actually returned by directory enumeration makes descendant metadata NotFound eligible. Only a descendant directory already proved safe makes its read_dir NotFound eligible. An unknown iterator error is terminal. Closing root validation precedes both returned Complete and Changed, rejecting loss/type/inode/full-mode changes with actual platform metadata. Its one extra bounded metadata operation is explicitly separate from4096 walk visits and remains funded by one outer observation slot. This is fixture-local evidence, without new atomic/ABA or non-Unix identity claims.

Visit/depth/path checks precede walk I/O/descent; file-length allowance is checked before committing metadata to a complete result. Attempted visits and observed metadata lengths are accounted separately from successful entries and from production read/hash bytes. Every incomplete observation retains spent work and discards its whole partial state. Original production Inventory fields, hash/EOF protocol, root/stamp and cumulative8192/1GiB/8192 ledger remain frozen.

The6001-slot proof preserves the original loop's subtle ordering: poll0 has no preceding sleep; every subsequent poll follows the unchanged5ms sleep; monotonic work is nonnegative. At poll6000 elapsed is at least30000ms, but a complete retirement predicate is still allowed before elapsed is checked. Therefore0..6000 inclusive are funded, and no later sleep is possible. Changed must pass through that same elapsed/cadence decision, never an early continue. The plan requires a final-slot/one-denied test and makes Changed-at-bound a visible failure. This bounds funded observations, not blocking syscall duration or scheduler latency.

Initial observation remains one strict complete call. Each of16 producer schedules funds exactly one observation; Changed reports incomplete spent work and continues the original4096 writes, while terminal errors still fail. Only the retirement caller schedules subsequent observations under the original single timer/cadence. No extra hidden retry or larger pressure/wait/cap is introduced.

The five private observer regressions exercise real files and hooks at actual enumeration/metadata boundaries. Missing/removed/replaced root, unsafe symlink/socket, real permanent OS failure, pre-I/O denial and real default-ledger repeated churn are required, with cleanup before assertions/unwind. An unavailable actual OS condition is HOLD rather than fabricated or skipped evidence. The initial RED must compile and fail for actual NotFound versus Changed; it must retain its intended assertion in GREEN. These interfaces can remain private inside the existing test file. Normal formatted fit is an execution risk with an explicit stop, not a planning compiler claim.

The original write/setup/cold-schema/initial-observation/restore/stale-stamp/seal/live-gate/drop/reopen/final-retired sections are frozen by four independently checked text/hash receipts. Only the observer and necessary producer/retirement dispatch are open for change. The unchanged final pressure selector will discover all six original/new cases; the original eight opened-unlink tests retain their previous selections. All nine final commands must run fresh on one final84/201 frame, without credit substitution from old127 passes or current126 passes/pressure failure/unstarted checks.

## Independent metadata and inverse checks

The retained `audit.py` independently authenticated448 reference occurrences, including authoritative target copies, before source/native copies, all45 captured plans and the47-state planning inverse inputs. It reconstructed the final authoritative plan from exactly the allowed original replacements plus the sealed precision and GFM changes. Task1/Task2 XML, all old fences and every old automated command are exact. Only two supplemental commands are added. The original diagnosis is embedded in full, byte-exact, in this frozen companion.

All45 actual canonical plan files still match the after-R3 copied inventory; total tasks remain102. All other44 complete plans/task XML are protected. Dependencies exist at earlier waves; the fifth owner creates no new same-wave conflict. Plan/task/wave/dependency/requirements/lifecycle/generated-at/commits metadata remains invariant as scoped.

The actual47-state copied planning proof starts with46 bodies/one true-absent companion. The recorded strict BSD `-E -F0 -p1` forward/inverse/forward commands all exit0 with no offsets/fuzz, reproduce exact candidate/F0/candidate states, and the actual retained scratch tree has exactly47 final files with matching hashes. This is a planning proof, not source implementation.

Separately, root-promoted-owner-readiness-v1/index.json159284 bytes/SHA256 `c99de4079508d68afbb8940e8841bf490b557e31ea25ffd0c18efb48a285e31d` was authenticated:568 original/current copied source/native bodies, all original four direct records unchanged, only original protected opening0075 promoted, five direct/79 protected disjoint with same84 union, three original bodies/two genuine absences, and201 native exact. The actual retained readiness scratch's84 files match held final-v2. This is current readiness only; no fresh final source, acceptance or source release follows. A later final source inverse must include the changed fixture and preserve all current35 bodies. The resumed35 contract explicitly refreshes accepted opening/tests.rs alongside the other accepted43 protected replacements; original35 eight-body/three-absence authority remains immutable.

## Publication finding PF43-01

**Files:** authoritative candidate43PLAN and FIXTURE-OBSERVATION-AMENDMENT.

**Evidence:** `mdformat --extensions gfm --extensions frontmatter --check` returns1 for both changed documents. The old43 before copy and unchanged OPENED amendment pass. In-memory formatting changes new Task3 presentation/new threat-table layout; the companion also changes prose/list syntax and the embedded diagnosis table/list whitespace. Blind formatting would violate the raw embedded diagnosis contract and can escape the new GREEN selector's trailing underscore.

**Required correction:** Prepare a separately sealed publication copy with checked Task3 presentation/HTML verify-container whitespace, preserve all four Task3 automated strings exactly, format the new outer companion prose, and retain all17272 original diagnosis bytes inside a verbatim fence or equally explicit preservation-aware layout. Preserve original Task1/Task2 XML, all original PLAN fences/automated strings, old OPENED bytes and every technical obligation. Require a new successful configured formatter check, complete precision diff and copied planning inverse before publication. Never mutate this frozen candidate or use its technical PASS as a formatting waiver.

## Limits

No application/native fixture, actual source, failed datadir, session, canonical/shared planning, Summary or Git operation was performed. Prior source-only CLEAN and held audit artifacts remain unchanged. Technical plan PASS is not actual source fit, test success, pressure convergence, CORE/source/root acceptance or global requirement closure. A separately checked publication copy and fresh root source/build release remain mandatory. Stop before168.


```yaml
phase: 167-candidate-history-validation-rules
plan: "43"
scope: independent_publication_precision_check
status: pass_pending_root_publication_and_separate_source_release
technical_findings: 0
publication_findings: 0
closed_finding: PF43-01
commits: false
source_execution_released: false
runtime_accepted: false
```

# Corrected V3 publication precision check

**PASS for root's separate publication decision.** The corrected V3 copy closes PF43-01 without changing the reviewed technical plan. Both changed documents pass the configured GFM/frontmatter Markdown check. This precision check does not publish anything, grant the fifth source owner to an executor, accept runtime behavior or waive any remaining gate.

The frozen candidate's technical PASS and publication-format HOLD remain immutable in independent-fixture-refinement/v1/index.json1392 bytes/SHA256 `d1d146e8ff09933ebe88cbb920334db21e806988e0509a3f8a7d427c1670451e`. This report applies that technical review only to the exact approved presentation/status transformations verified below. No new technical finding was identified.

## Exact V3 inputs and outputs

| Artifact                              | Bytes | SHA256                                                             |
| ------------------------------------- | ----: | ------------------------------------------------------------------ |
| Frozen candidate FINAL-INDEX          |  6132 | `257c7d3c9442d26704c12bc94b7d94606460e93002fae7c021dc2211f2aed60c` |
| Root V3 ready index                   | 41206 | `7db6b71b998a2de47a3e65d988fe573e0f09440ea9feaac3f9d942b8d8eda7ee` |
| Ready43PLAN                           | 43740 | `9d620d79dad157e06d87c11acc3a8727c473bca2c0ae7905d2f322f400813163` |
| Ready fixture companion               | 28518 | `b34302ccc09244df410bda078306b37efdc47b12f5aafbcdff4ec3f6cf9802c0` |
| Unchanged historical OPENED amendment | 24417 | `ba5b2f37fac1a08ec58369131b1ed30f40700a7aa76516fcc52e59cf1cbc7e8d` |
| Ready whole-planning patch            | 82744 | `edf940abbdc7a288cb57fb6c06f85be5fb245be95a9c622d11791c1f0c224b8e` |

All47 before/after planning records were independently authenticated. Before has46 existing bodies and one true-absent new companion; after has47 bodies. Exactly the existing43PLAN and new fixture companion change. The historical OPENED amendment and all other44 complete plans are byte-exact. Actual canonical files still matched the captured before copies during this check, and the new companion remained absent canonically.

## Precision proof

The independent audit reconstructs the exact ready PLAN from the frozen43406-byte candidate using its single announced candidate-status sentence replacement, the explicit Task3 verify-container line layout, and the installed configured formatter. The reconstructed bytes equal all43740 ready bytes. Task1 andTask2 complete XML remain byte-exact. Task3 differs only in whitespace. Every automated string, including the two supplemental RED/GREEN argv and the GREEN selector's trailing underscore, remains byte-exact. Every original PLAN fence remains byte-exact. No `observer\_` substitution is present.

The fixture companion is reconstructed from the frozen28435-byte candidate using only its announced status/first-sentence replacements, the sentence describing the verbatim fence, the Markdown fence around the complete original diagnosis, and configured outer formatting. The reconstructed bytes equal all28518 ready bytes. The one diagnosis fence contains exactly the original17272 UTF-8 bytes, including its ending newline; the original diagnosis is not normalized, abbreviated or paraphrased. Outer numbered-list, multiplication-marker and GFM presentation changes are exactly the formatter's output. No technical guard, diagnostic qualifier, original-source authority, protected assertion section, funding rule, verification obligation or continuation order changes.

The installed command was independently run in check mode against both ready changed documents:

```sh
mdformat --extensions gfm --extensions frontmatter --check READY_43_PLAN READY_FIXTURE_COMPANION
```

Both returned0 with empty stdout/stderr. Only owned review documents were formatted by this reviewer; no authoritative candidate/ready/canonical document was changed. The failed frozen/earlier ready versions remain historical and do not become passing artifacts retroactively.

## Independent actual planning inverse

The retained read-only precision audit authenticated286 reference occurrences and inspected the root preparation script and its recorded three strict patch outcomes. In addition, this reviewer executed an independent **planning-only** filesystem proof in its own new ignored `publication-v3/scratch/`, populated exclusively from the authenticated before planning copies. No live file or native fixture was patched.

| Independent command                    | Exit | Actual resulting files | Verification                                                |
| -------------------------------------- | ---: | ---------------------: | ----------------------------------------------------------- |
| `/usr/bin/patch --batch -E -F0 -p1`    |    0 |                     47 | Exact ready candidate, complete file set and all body bytes |
| `/usr/bin/patch --batch -E -F0 -p1 -R` |    0 |                     46 | Exact original F0, new companion truly absent               |
| `/usr/bin/patch --batch -E -F0 -p1`    |    0 |                     47 | Exact ready candidate again                                 |

All three have empty stderr and no offset/fuzz/reject indication. The final actual scratch contains exactly the47 ready files with matching whole bytes. This is independent copied-planning reversal evidence; it is not the future five-owner source inverse, application behavior, original35 composite proof or a permission to reverse the working tree. The audit script and concise check record are retained beside this report.

## Technical contract remains unchanged

The prior independent technical review continues to bind exactly three tasks, the unchanged Task1/Task2 contracts, the explicit Task3-only fifth existing fixture owner, original three-body/two-absence before authority,79 protected/same84 union/201 native, known-descendant-only typed Changed, complete-only retirement, terminal root/unknown/unsafe/permanent errors, separately funded root closing check, unchanged precharged source bounds, and the original initial1/producer16/6001 retirement-slot policy under one30s timer/5ms cadence. No production policy or original4096-by-64KiB pressure/write/restore/stale/live/drop/reopen assertion is relaxed.

All five real observer concerns remain discoverable with the original native test under the unchanged final-pressure selector. All nine final commands must still run fresh after actual observer RED/GREEN and a new frozen source frame. Normal source fit/compilation, every real OS control, successful pressure/reopen, full source/CORE review, Summary/scoped QA/root acceptance and later35/phase gates remain unproved. The620 target/628 stop and no-sixth-owner boundary remain explicit.

Root must perform its separate publication action and independently revalidate the promoted original-before owner map/current source before granting a fresh single source/build release. Source-only CLEAN, the actual failed07 HOLD and unstarted08/09 remain historical truth until new qualified evidence exists. All global CVLD IDs, whole35/43/phase acceptance, canonical source review before17 and later gates remain open. Stop before168.

Only new ignored review artifacts and copied Markdown planning scratch were created here. No application, Cargo, Bazel, Rust compiler/formatter, native/failed datadir, actual source, session, shared canonical planning/state, Summary or Git action occurred.


## Root checked six-owner observer layout: source release remains separate

Technical review passes and corrected readiness closes OL43-01/02/03. Original three-body/three-absence authority,79 protected/derived85 source states/201 native stay distinct from the current partial core. Exact TaskXML, commands, actual RED and pressure assertions remain. Full records are lossless; actual mode control must derive different readable permissions. No source/runtime acceptance follows.

```yaml
phase: 167-candidate-history-validation-rules
plan: "43"
scope: frozen_plan_and_inert_layout_independent_review
status: format_and_record_precision_hold
technical_plan_and_layout_verdict: pass
technical_policy_findings: 0
precision_findings: 3
uncompiled_fixture_risks: 1
commits: false
source_accepted: false
runtime_credit: false
```

# Observer layout refinement independent review

**Technical plan/layout PASS; frozen publication/readiness HOLD.** The conventional private split is coherent, preserves the observed RED and all intended behavioral controls, and provides substantial file headroom. Three precise publication/readiness corrections remain: new PLAN GFM formatting, one extra inert-parent EOF blank line, and loss of original integer metadata precision. No production policy or observer-control-flow defect was identified. The literal mode-change fixture has an additional uncompiled portability risk that must be corrected within its existing future Task3 contract.

This verdict binds observer-layout-refinement/v1/INDEX.json4680 bytes/SHA256 `c87597e3c4df1bb1a52e1c16d984f1bd2e0840d722e04edff41e5ae4effa47d5`. The candidate PLAN is46639 bytes/SHA256 `04778186ab567fe021f719bc2227a88e17bcf36771e95c1b6b0f47b4542b55b9`; the new8110-byte companion is SHA256 `93582d9f7b402a58dccf61518810f6774001bca2b9dd1d4bcc4a1a626eed31d5`. Both prior companions remain whole-byte exact at fixture`b34302cc` and OPENED`ba5b2f37`. No actual child source file, source release, compiler result, GREEN result or phase acceptance follows from this review.

## Three concrete precision findings

### OL43-01: changed PLAN fails configured Markdown check

`mdformat --extensions gfm --extensions frontmatter --check` returns1 for the frozen proposed43PLAN. The new short layout companion passes. An independent in-memory configured-format diff changes only threat-table padding after the five-owner→six-owner wording change. Task1/Task2 complete XML, every automated argv including supplemental GREEN's trailing underscore, and every old PLAN fence remain exact after that normalization. Prepare a separately sealed formatted/status-accurate publication copy and recheck; do not mutate this frozen candidate or waive the failed check.

### OL43-02: inert parent has one extra EOF blank line

The explicitly authorized readonly probe ran pinned `rustup run 1.94.1 rustfmt --edition 2024 --config skip_children=true --check` on both ignored inert source copies. Exit1 reports only removal of one trailing blank line after the last parent wrapper. The child is unchanged by the check. The supplied parent is253 physical lines; trimming only that final excess blank gives252. No RED header/body, test entry, wrapper, field, method or behavior needs to change. Record the corrected copy/hash/count separately and require the scoped check to pass. No source or ignored candidate copy was modified by the probe, and no type/compile/runtime credit is inferred from rustfmt.

### OL43-03: claimed exact original records contain rounded mtimeNs

original-source-authority-map.json84015 bytes/SHA256 `6c9478d5776757e843cd4c8cc67d0da019bd19e5d6ed7d1d41ba0a76ff9a05f9` retains the correct original receipt, paths, whole-body bytes/SHA, copy references and true absences. However, complete original records are not literally exact:

| Original record          | Original observation.mtimeNs | Candidate observation.mtimeNs |
| ------------------------ | ---------------------------: | ----------------------------: |
| inspection/copy.rs       |          1792729436850580189 |           1792729436850580200 |
| inspection/live/tests.rs |          1792740335258033946 |           1792740335258034000 |
| opening/tests.rs         |          1792721804579653524 |           1792721804579653600 |

This is consistent with a JavaScript Number roundtrip; it is metadata imprecision, not an actual source timestamp change or substituted original body. Preserve the original78b7 full records losslessly in a separately checked map, including the promoted opening record and protected union. Alternatively, an explicit projected-body comparison would need to retain the untouched full origin record separately and label its narrower claim accurately. Root intends a lossless map. Old rounded artifacts remain historical. A future fresh root capture/map/source inverse/release is still necessary before child creation.

## Full inert source assessment

The complete8852-byte/253-line parent (`25a8dd0b51e22eb7536e9607a5c95ff7813d6c723b1fd69f01dd2ea6f184400d`) and17278-byte/476-line child (`76239ce6646baff7c9f8097cb82eac4369e313d7e3a6ef80a5bbf8970f7b854a`) were read. The old normal704-line/25279-byte inert body SHA256 `3b11db98af16a68241dfbde6fc05d27c9cf031aaf34b07365bb61086b406ab3f` was authenticated as their mechanical origin. It exceeds628 by76; the split addresses that concrete hold rather than hiding lines or dropping tests.

The observer core is byte-exact to its old normal chunk after removing only the new private `pub(super)` annotations. The owned tree cleanup helper is byte-exact. All four additional helper-case bodies are byte-exact to their original corresponding704-body tests; only their function names/private visibility and thin parent call sites change. The original RED complete attribute/header/body/ending blank is1201 bytes/SHA256 `e5c2a36e28e04603195808351a3e8f30597f972490c5fe8f2dee3f98406a2867`, byte-exact both to the inert704 source and actual recorded pre-RED body copy. No new RED is manufactured or required by moving it.

### Module and effective visibility

Parent `mod observation;` is conventional and private. The parent retains the native pressure case, all five named observer test entrypoints and the actual RED body. Four other parent entrypoints call cohesive private case helpers. The child contains no `#[test]`, so it creates no duplicate discovery or changed supplemental `--exact` path.

The child imports ancestor-private root/constants/JournalIdentity through its private parent. Physical/ObservationWork/Observation/ObservationPoint/ObservationSlots and only parent-used methods/fields have `pub(super)` visibility. The parent needs journals and RED visits/bytes, which are exposed only within opening::tests; closing/work guards and internal failure/walk/root methods remain private. Parent-private JournalIdentity is valid at this same effective scope. No public/pub(crate) API, new production test-access surface, include/path redirection, hidden module, dependency or seventh owner is introduced. Existing opening.rs gates the parent with cfg(test); the Unix-only unsafe-case helper and wrapper use matching cfg(unix).

### Classification, root and funding

Changed remains a typed whole-discard result only for actual NotFound on an enumerated descendant metadata lookup or a descendant directory read_dir after successful safe metadata. The root's metadata/read_dir NotFound, unknown directory-iterator error, unsafe symlink/special type and non-NotFound I/O remain terminal. No error-string category inference or zero-filled/partial Physical success exists. The selected root is rechecked before returning either Complete or Changed; Unix dev/inode/full mode plus real-directory/no-symlink are compared. Root length/mtime/directory-link churn do not acquire an unjustified atomic identity promise. Non-Unix behavior does not fabricate Unix fields or claim a new ABI/ABA guarantee.

Visit/depth/path checks precede metadata/hooks/descent. Checked file-length allowance precedes committing a complete file result. Successful entries, attempted visits, metadata-length bytes and the one closing metadata operation remain distinct. The private construction guarantees visits4096, metadata lengths512MiB, depth16/path4096; no file contents are read or hashed by this observer. The original production Inventory/hash/EOF/root/stamp ledger is untouched. Known Changed drops all partial Physical state, carries original OS error/spent work and refunds nothing.

Only metadata/read_dir errors pass through the fixed-stage bounded encoded-path logger. Unknown iterator/closing errors propagate as terminal original io::Error; this review does not claim exhaustive syscall-stage tracing for those branches. Path output is bounded to256 encoded bytes and rendered numerically, with no contents or extra logging stat.

Initial remains one strict observation. Producer still performs all4096 same-key64KiB inserts, one attempt at each of16 scheduled reports, explicit incomplete/work-only output on Changed and the unchanged yield. Retirement consumes one6001-slot ledger, checks a Complete original-journal-absence predicate before elapsed, then uses the original single30s Instant/5ms sleep for retained or Changed outcomes. There is no early continue, extra retry/deadline or partial retirement success. Last incomplete observation at the bound returns false. The separate source-derived slot cap is not a syscall-latency theorem.

### Real private filesystem controls

The actual RED hook unlinks a real enumerated four-byte file before metadata, verifies one hook/two spent visits and observes NotFound; cleanup precedes the intended category assertion. The directory helper removes an already observed descendant before read_dir and then obtains an independent fresh complete observation. Unsafe controls create actual missing/removed/replaced/mode-changed roots, a real symlink/socket, and actual PermissionDenied with restoration before assertion. A held root descriptor proves the replaced inode is different. The socket uses a unique short alias pointing to the owned root, removes that alias before handling bind failure, and leaves the actual socket for the observer's type refusal; it does not delete another actor's path.

Limit controls deny visits/depth/path before every hook, test exact limits and one-over, and use sparse metadata at512MiB without reading that content. Default churn funds6001 real create/enumerate/unlink Changed observations, each with `(visits,bytes,closing)=(2,0,1)`, totaling12002 visits. The denied next slot causes no hook/walk. A separate final funded complete observation permits retirement before even invoking elapsed. All of these are reviewed test source, not executed cases or runtime evidence.

**Uncompiled fixture risk:** mode-case child342 sets a constant0o750, which can equal the initial create_dir mode under umask027. That would not prove a mode change and could fail the mandatory refusal control for setup reasons. The already checked contract requires a different readable mode: derive it from observed original permissions while preserving owner read/execute access, rather than assuming the umask. This needs no new owner or behavioral contract and should be handled during the future authorized Task3 setup. The inert mechanical split is not final GREEN correctness evidence. Actual PermissionDenied and socket/OS setup also remain required real checks, with honest HOLD on unavailable setup rather than skips/fake errors.

## Plan and source authority

The full new short companion, prior checked behavioral contracts, complete changed PLAN delta and Task3 were reviewed. Task1/Task2 XML, all automated argv and all old PLAN fences remain whole-byte exact. The prior fixture and OPENED companions remain unchanged. Three tasks,45 plans,102 tasks total and35 distinct waves remain; all other44 actual canonical plans match their captured whole copies. The added owner is one conventional child solely within Task3; no dependency/cycle/same-wave scope change is introduced.

Original body/absence authority is correct despite the metadata-integer finding: retain original78b7 four direct states, promote original opening7bcc, add only the new child's recorded true absence. The six direct states contain3 original bodies/3 original absences;79 other protected inputs are disjoint, giving85 original states=82 present/3 absent. Current recorded source is84 present plus the uncreated child; it equals the earlier84 frame with only the declared partial15623-byte opening body SHA256 `36cd42a2a282622a40da4247a92e12e2028694abc78e1fdcd7cf2b777612d1ba`. All201 native shapes are unchanged. Neither that partial body nor inert253/476 substitutes for original7bcc. A genuinely created final may be85 present only after a fresh release/freeze; no source inverse or final runtime qualification is claimed here.

The continuation correctly requires a future six-owner full-file source inverse restoring3 original bodies/removing exactly3 originally absent children, preserving79/current35all11 and201 native, then reproducing actual final85. Resumed35 still uses its exclusive original5b0 direct8 bodies/3 absences; accepted protected replacements include the fixture and all3 accepted new43 children. Conditional85/74 membership must be derived from actual accepted receipts, not old84/73/71 history. Independent/root review precedes its separate source release/all nine fresh35 commands.

## Independent evidence checks

The retained audit authenticated918 reference occurrences over630 distinct paths, including original/captured source/native copies, authoritative documents, layout/body maps, other44 plans and inverse inputs. It independently reproduced the source-map body projection, absence/set arithmetic, original protected pressure assertion sections, unchanged original test entry paths and RED block, old704→split body correspondence, and all command/XML invariants.

In addition to authenticating the recorded48-state planning proof, this reviewer ran independent copied-planning BSD-F0 forward/inverse/forward in its own ignored scratch:48 candidate files→47 original files/one true-absent layout companion→48 candidate files. Every file set and complete body matches, all exits are0 with empty stderr and no offset/fuzz/reject. This is planning scratch only, not a source inverse or permission to reverse the working tree.

The root FIT-HOLD receipt remains667312 bytes/SHA256 `06a00e213ad1e5255919afc8fb618f70db3f9f83e0a3394af2fe07e6d5ee9be5`, with four closed jobs, actual compiling RED and an uncompiled corrected core/no final-nine run. This review authenticates that receipt and actual RED source lineage; it does not repeat a full runtime/CORE audit or turn inert controls into executed evidence.

## Required next gate

Preserve this frozen review/candidate/map. A separately sealed readiness/publication copy must pass configured Markdown and scoped pinned rustfmt checks and preserve full original integer records losslessly. Require independent precision check/root whole review, then fresh actual source/absence/native capture, source inverse/readiness and a separate root release before actual child creation. Future source implementation must also produce a genuinely different mode fixture and satisfy all real GREEN/final-nine gates. Source/CORE before Summary, scoped QA/root acceptance, separate35 and later whole167 gates remain mandatory. No global requirement closes; stop before168.

Only owned ignored review artifacts and copied Markdown planning scratch were written. No actual source, Cargo/Bazel/compiler/test/native fixture, failed datadir, active session, canonical/shared state, Summary or Git action occurred. The only Rust tool used was the expressly allowed readonly pinned rustfmt check on ignored copies. Repo AGENTS, Bright Builds sidecar/overrides, architecture/code-shape/testing/verification/Rust standards and the loaded active lessons informed the review.


```yaml
phase: 167-candidate-history-validation-rules
plan: "43"
scope: corrected_layout_publication_and_inert_readiness_precision
status: pass_pending_root_publication_fresh_actual_frame_and_source_release
precision_findings: 0
closed_findings: [OL43-01, OL43-02, OL43-03]
commits: false
source_accepted: false
compiler_or_runtime_credit: false
```

# Corrected observer layout precision check

**PASS for root's separate publication/readiness decision.** All three frozen precision findings are closed in the exact corrected copies below. The reviewed technical plan and private layout remain unchanged. Future actual source implementation must still derive a different readable mode for its real mode-refusal control; the inert helper is not a GREEN test result. No source release or runtime acceptance is granted here.

The frozen c875 review remains immutable: independent-observer-layout-refinement/v1/index.json1999 bytes/SHA256 `1a85e25ccd009e96453f7ab96d8b9823c256c6fbed295751794df6bb9a4666c6`, with15500-byte REVIEW/SHA256 `0de5a6364c68475eaa42c88a7f3fc942db20db208d3aa7de78bb6d82501161ba`. Its technical PASS/three precision HOLD findings are preserved as historical truth; this separate check qualifies only the corrected ready packet.

## Exact corrected receipts

| Artifact                       |  Bytes | SHA256                                                             |
| ------------------------------ | -----: | ------------------------------------------------------------------ |
| Root ready index               |  42992 | `b1e7578eb80f07d181c2edf35b957ffe5292a34021ededa4b59c2450b0013298` |
| Ready43PLAN                    |  46630 | `af17f243a1a00a71dee269e7d3a9bd01acf066cf67cbc58908907a8c6f26ec34` |
| Ready layout companion         |  10030 | `eaf60beb393758341483a66dd52840501e6c50d56cd004d9cfb6397a489dc9ce` |
| Lossless original authority    | 196360 | `11c0d763cff0bfdd9fe7f061e540633f66a67baadb9f3d44e45b32978ba0dc3d` |
| Inert source-readiness index   |   4218 | `d6d348d2406c32f649bded77924d8e8183aae7f5a1381502d2ac3f0f9e945ded` |
| Corrected252-line inert parent |   8851 | `986833d9d59e4269460b48fd6afb7d8843ddd03a4ee4d89e6cd51c69c93f1fd4` |
| Unchanged476-line inert child  |  17278 | `76239ce6646baff7c9f8097cb82eac4369e313d7e3a6ef80a5bbf8970f7b854a` |

The prior fixture28518-byte`b34302cc` and OPENED24417-byte`ba5b2f37` companions remain whole-byte exact. All other44 plans remain exact. No canonical file was written by this reviewer or by the corrected preparation receipt.

## OL43-01 closed: configured Markdown and exact contract preservation

The ready PLAN is exactly the configured GFM/frontmatter formatter's output from the frozen46639-byte PLAN. Its only presentation change is threat-table padding. **Every complete TaskXML block, including Task3**, every automated argv including supplemental GREEN's trailing underscore, and every old fence remain byte-exact. No task/action/verification or behavioral obligation is rewritten.

The ready layout companion has only the announced checked-status/first-sentence precision, the word distinguishing original253-line inert measurements, and the explicit new corrected-publication section. That section names the exact lossless map and source-readiness receipts; distinguishes252 corrected parent from253 historical parent; labels the copied inert85-state mechanics as uncompiled/unapplied; and requires future derived readable-mode setup under the existing contract. The complete old companion prefix reconstructs exactly after those announced sentence replacements. No policy, cap, ownership, original-before authority or release boundary is weakened.

Independent configured `mdformat --extensions gfm --extensions frontmatter --check` returns0 for both changed ready documents. No authoritative document was formatted in place by this reviewer.

## OL43-02 closed: one EOF byte only, pinned check passes

The corrected parent is exactly the frozen8852-byte parent minus its one final newline byte. The child is whole-byte exact. The parent is now252 lines and child476. The actual RED attribute/header/body/ending blank block remains1201 bytes/SHA256 `e5c2a36e28e04603195808351a3e8f30597f972490c5fe8f2dee3f98406a2867`; all five named parent paths, original protected native test sections and four helper-case bodies remain unchanged by inheritance from the full frozen review and this exact EOF-only comparison.

The expressly permitted independent readonly command `rustup run 1.94.1 rustfmt --edition 2024 --config skip_children=true --check` returns0 on the two corrected ignored copies, with no output. This is syntax/formatting evidence only. It does not prove type resolution, real OS fixture behavior, GREEN, native pressure/reopen, final frame acceptance or ordinary production compilation.

## OL43-03 closed: complete lossless original records

The corrected authority JSON was read using Python's arbitrary-precision integer JSON parser and authenticated by raw length/SHA256. Independent deep equality confirms:

- All84 original source records equal the complete records from immutable original43 receipt208139 bytes/SHA256 `78b72ccc63a16da8fecf102c10f468ea1411eddebe67d0f80b40923c888397a9`, including every original observation field and integer.
- All201 native records equal the original complete native records and remain separately versioned/copied.
- The original four ownedBefore records equal their complete originals, and the promoted opening0075 record equals its original full record.
- All79 remaining protected records are exactly the original records outside those direct owners; no current partial460 opening body substitutes for original7bcc.
- Only the new child's recorded true absence is added to that original union and direct-before set. There are85 original states,3 original bodies/3 direct absences and79 protected inputs. Future final85 presence is not asserted.

In particular, original nanosecond values1792729436850580189,1792740335258033946 and1792721804579653524 remain exact. The old rounded6c947 projected map is preserved as historical data with correctly qualified body-only authority; it is not silently rewritten. Fresh actual source/absence/native capture and a separate release remain required.

## Independent actual copied-tree proofs

The retained precision audit independently authenticated478 reference occurrences over396 distinct files. Beyond checking the root's recorded successful metadata and source mechanics, this reviewer created two new isolated copied trees in its owned ignored review directory and executed actual strict BSD patch forward/inverse/forward proofs.

| Proof               | Forward                 | Inverse                                                                | Forward again           |
| ------------------- | ----------------------- | ---------------------------------------------------------------------- | ----------------------- |
| Planning metadata   | 48 exact final files    | 47 original files; new layout companion truly absent                   | 48 exact final files    |
| Inert source layout | 85 exact proposed files | 82 exact original bodies; all3 originally absent children truly absent | 85 exact proposed files |

Every command uses `/usr/bin/patch --batch -E -F0 -p1` with `-R` for reverse. All six commands exit0, have empty stderr and no offset/fuzz/reject indication. Every resulting complete file set and every complete byte body equals its independently derived expected map.

The inert source expected-before map contains all82 actual original bodies, including the three complete original direct bodies and79 protected inputs. Expected-after retains those79 exactly, substitutes the four previously frozen43 bodies from their authenticated v2 copies, and uses only the corrected252 parent and unchanged476 child. The3 true-new children are absent on reversal. All201 native copies are authenticated separately and are never patched. This proves copied layout/source-authority mechanics; it is not application of the repair to the repository, a new actual final source frame, accepted compilation or a working-tree reversal permission.

The actual application opening source remains the earlier uncompiled partial core per root's lease/evidence; this reviewer did not inspect or change it. No actual sixth source file was created by this audit. The source copies in the owned scratch are deliberately inert.

## Remaining source and whole-phase gates

The original technical review's known-descendant-only Changed, strict terminal errors, closing root validation, precharges,6001-slot ordering, actual filesystem controls, original4096-by-64KiB write corpus and all restore/stale/live/drop/reopen assertions remain binding. The constant0o750 fixture risk is acknowledged in the corrected companion: the future authorized implementation must derive a different readable mode from actual original permissions. The current inert body remains mechanically unchanged and receives no test-pass credit.

Root must whole-read both reviews/corrected packet, publish separately, fresh-capture and check the actual85-state original-before/current-absence/native map, and issue a separate single source/build release before actual child creation. Genuine focused GREEN, all nine fresh final commands on one final85/201 frame, strict actual source inverse, independent source/CORE before Summary, scoped QA/root acceptance, composite35/root review/separate35 and all later167 gates remain pending. No global CVLD ID, whole43/35/phase completion or Phase168 permission follows.

No Cargo, Bazel, compiler, test, native/failed datadir, actual source, active session, canonical/shared metadata, Summary or Git action occurred. Only owned review artifacts and inert copied planning/source trees were written; the sole Rust tool was the expressly authorized readonly pinned format check. Prior frozen reviews/candidates and projected maps remain unchanged.


## R5 peer authority and queue ownership — root planning acceptance | 2026-10-23T22:26:29.611568+00:00

Frozen3c04e77f and independentfb9a6df3/536e1c0f pass with2,134 assertions/779 authenticated paths, zero planning issues. Both prior R4 findings close IN PLAN only; R4 HOLD and all source failures remain history. Root WHOLE-read both plans/companion/map/full independent review, authenticated361 candidate references/344 files and checked all four formatter targets. Six-owner46 authority precedes eight-owner45 integration through an explicit not-final dependency-ready receipt, with all final joint strict/source/tests before Summary46 then45. Complete14 inert fit/API/code and actual behavioral/diagnostic evidence remain separately gated. No actual46/45 source, runtime, strict pass, Summary/count completion or Git release. All5 CVLD OPEN; STOP168.

# VERIFICATION PASSED — frozen R5 plan check

**PASS for planning only: 0 blockers, 0 warnings.** The frozen R5 addresses both R4 planning findings through explicit tasks, ownership, interfaces and real verification obligations. It does not authorize source application or establish source fit, compilation, runtime behavior, a dependency-ready receipt, final acceptance, Summary, commit or push. Frozen R4 remains HOLD.

Checked input: `revision-v5/index.json`, 85,080 bytes, SHA-256 `3c04e77f75c6711e35cfd5dae8ce3b2c522b10338e0e41a45329fd94c153c888`. Whole plans 45/46, companion, revision map, handoff, caller/control inventory, allocation, stage graph/schema, DAG, source/provenance/input/history/native/direct-before/composite and inverse records were read or independently authenticated as appropriate. The independent metadata proof contains 2,134 successful assertions and 779 unique authenticated paths, including all 344 indexed files.

## Frozen principal references

| Artifact                              |  Bytes | SHA-256                                                          |
| ------------------------------------- | -----: | ---------------------------------------------------------------- |
| 167-46-PLAN.md                        | 31,038 | af4b0e082e567860c122cccc5cff1aa6ec19a89b01d3ed5e018f1fb521cef128 |
| 167-45-PLAN.md                        | 31,054 | 49c7f18f8bce923e14a7a9e83712d521df33bd2eb5571eba5dcd3b156b9002db |
| 167-45-46-PEER-AUTHORITY-AMENDMENT.md | 36,184 | 209ef002e79d1b764d475d1f8c1be4071170828f1f273963c628d326ee9fa238 |
| REVISION-MAP.md                       |  7,267 | 4b3db515b3a3d08dbbcc2ac1641296a0fc49cea16375b20e914c72e250b2be7a |
| HANDOFF.md                            |  3,642 | b0c9717596feb8cb8a935b2f0bd2cc8e12ab0edc09ae990c55902d2cd3424acc |
| Unchanged published 44 whole body     | 15,187 | 0fa6f88215ed7f0d6e2b864bb9742f1ca75db924690af9453697ddcbf318d1a3 |

## R4-PC-01 is closed in the plan

The previous Arc-only proof did not distinguish a legitimate network-only same-ID admission while the outbox registration remained unchanged, including B admitted then removed. R5 now requires actual Sync lifetime proof together with registration identity at the memory effect boundary. See 45 lines 116–124, 46's queue contract and Task 1, and companion lines 80–88.

The factory token owns a private strong Arc cleanup identity as well as shared registry identity, epoch, peer and checked non-reused nonce. The conventional private registry child maintains separate current-membership nonce and live-cleanup-Weak maps. Absence prunes current nonces while preserving a live cleanup Weak. Maintenance prunes only dead Weak entries; it is bounded by live actual Sync identities at maintenance boundaries, with no false instantaneous-drop claim. Every actual created same-ID membership invalidates the prior cleanup identity on all three production admission routes, including an error after creation. Rejected duplicates preserve it. Only the actual Sync factory installs the fresh Weak; no Clone/raw constructor/test hook, resettable effect-generation substitution or lifetime-growing tombstone is introduced.

The narrow memory-only operation locks actual authority, briefly validates registry/epoch/Weak upgrade and Arc identity plus current nonce when present, releases registry, then retains the SAME authority guard through expected-registration-Arc compare and queue take/remove under the outbox mutex. Active selection requires original present membership. Final cleanup permits original presence or fresh absence with a still-matching cleanup identity. Thus A removed/no B can clean its genuinely own nonempty queue, while A→B→B removed denies A. All four real send sites and finish must use this boundary. The caller audit must establish authority→brief registry→outbox order without a reverse held path; every guard drops before transport, encoding, completion, abort, storage or await. Queue operations use the actual operation mutex directly, avoiding `mutate` and its bounded BASIC wakes, and do not broaden the request projection.

Registration remains before connect, preserving its old precedence. A session that never earned a token may remove only its matching EMPTY registration atomically. A nonempty unknown queue produces a genuine ownership failure joined with the real primary, preserving queue/FIFO/readiness/capabilities and granting no abort. R5 explicitly avoids promising Compatibility for that nonempty case. No registration means no cleanup. This is an explicit checked precision, not a blanket category override or admit-first workaround.

The required controls distinguish all three unchanged-Arc admission routes, rejected duplicates, actual error-after-created, B later removed, own A nonempty after protocol removal without B, pre-token empty/nonempty, real callback replacement and all four send sites with success/error/cancel. At least two distinct genuine B producer capabilities, full FIFO/readiness/capability/live-versus-physical accounting, private 128-round/bounds/foreign/epoch/poison/exhaustion controls, and all 22 prior named source controls remain mandatory. Copied names are not test discovery.

The plan limits the claim correctly: authorization governs queue selection/removal at that operation. An ID-only producer may prepare before a transition and enqueue later. Weak ownership is not per-emission lineage or a universal transport-at-send theorem. Existing affine effect settlement and physical handed-write accounting remain governing capabilities. This explicit limitation neither fabricates introspection nor licenses stale queue selection.

## R4-PC-02 is closed in the plan

Independent `mdformat --extensions gfm --extensions frontmatter --check` passes all four actual changed targets: 45, 46, companion and 35. The 35 new appendix uses the corrected hyphenated marker/spacing. Original 35 task XML, fences, six focused and three final argv remain exact. The old frozen R4 formatting failure and its preview are preserved as history.

## Goal-backward checks

| Dimension               | Result and evidence                                                                                                                                                                                                                                                                    |
| ----------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Requirement coverage    | PASS. Both frontmatters retain CVLD-01/02/03/04/06; corrective tasks support these requirements without claiming global completion.                                                                                                                                                    |
| Task completeness       | PASS. Both independent GSD structure checks report valid, no errors/warnings, and all six tasks have files/action/automated verification/done.                                                                                                                                         |
| Dependencies            | PASS. 46 plans / 104 tasks / maximum wave 35; valid acyclic references and shared-owner ancestor ordering. 46 depends on 34 at wave 18; 45 depends on 34/46 at wave 19; 35 remains wave 27.                                                                                            |
| Key links               | PASS. Actual admissions/factory/registry, BASIC shared resolver, typed outcomes, fresh policy, shared request core, queue selection/finalizer and real providers are wired by named owner tasks and verification obligations.                                                          |
| Scope                   | PASS. 46 has three tasks/six owners; 45 has three tasks/eight owners, with five integration files in Task 2. These fit normal plan thresholds. Actual complete fourteen-body fit remains an explicit pre-source gate.                                                                  |
| Must-haves derivation   | PASS. Observable lifetime isolation, preserved true errors/accounting and truthful staged acceptance map to artifacts, wiring and actual behavioral cases.                                                                                                                             |
| Context/scope reduction | PASS. D-07–D-10 map explicitly to tasks; D-01–D-06 retain full original selected-network/ancestry/script/time/overwrite owners. No silent reduced delivery or deferred 168–171 work.                                                                                                   |
| Nyquist                 | SKIPPED: configuration explicitly disables it. Specified automated verification remains mandatory.                                                                                                                                                                                     |
| Cross-plan contracts    | PASS. The nonfinal API receipt, observation/denial ownership, BASIC lease boundaries, request-policy facts, queue capabilities and accepted composite origins remain distinct and compatible.                                                                                          |
| AGENTS                  | PASS for planning. Repo/Bright Builds/overrides and relevant architecture, code-shape, testing, verification and Rust guidance were loaded. Active global/repo lessons were fully loaded within limits; no project skills were found. Full native/precommit requirements are retained. |
| Research                | PASS. Existing research marks open questions resolved. Actual implementation/diagnostics are appropriately future evidence rather than predicted passes.                                                                                                                               |

All five CVLD requirements remain OPEN. Neither corrective plan independently satisfies the complete phase goal. No GUI, wallet-product, public-network default, automatic import/repair or actual differential-qualification expansion is introduced.

## Source interfaces and preserved behavior

The authority plan covers all admission routes, initial/preexisting membership, checked monotonic nonce exhaustion without wrap, error-after-membership invalidation, shared identity/epoch and active bounded maintenance. Named crate-private exports are owned by network.rs, preserving the private authority module boundary. Exact raw outbound body relocation remains protected. Request projection stays synchronous, non-escaping and memory-only under actual authority: validation precedes manager/request/global-expiry/target/GetData/runtime effects, with no storage/transport I/O, await or recursive handle use.

BASIC uses one shared existing algorithm in explicitly owned serving.rs. Sync validation occurs under the same held authority guard BEFORE every semantic/native/reserve/read/bind/raw InvalidDisconnect effect; registry drops before existing bounded native reads. All invalid-disconnect reasons and later bind/continuation effects remain guarded. Existing generic APIs, native/wake/lease/error/per-item partial-effect order are retained. This does not invent a global storage-I/O-free requirement for old resolver/wake boundaries. Post-effect pruning failure must remain an **Authorized inner operation error**, joining the genuine operation primary with pruning error where both exist; it cannot become a fabricated Denied result. The prior complete12 source defect remains historical source HOLD, while the R5 plan expressly requires correction.

Authorized and ContinuationDenied are owned production outcomes. Actual backend/read/semantic failures remain authorized inner errors. On genuine later denial the previous and earned raw observations remain at most two bounded sets, ordinary work stops, and denied finish performs a fresh NO-CAPTURE identity/epoch/actual-membership check. Only verified actual absence grants reconciliation of those existing sets, without disconnect or a third vector. Present same lifetime, replacement, foreign/epoch/poison/error yields visible failure and retains both. Helper Ok while another peer still owns a hash is not permission to discard an unresolved observation. Normal authorized finish retains its separate retained+fresh behavior. Queue permission is separately earned.

Block/NotFound policy requires a fresh actual original-peer manager query AND runtime ledger membership at the old query point AFTER preceding cleanup, under token authorization. Retained observation vectors are not policy facts. The shared pure block_response predicate preserves InventoryType/disposition/error semantics. Real controls include manager-only/no-handoff, handed positives and A canceled while B keeps the global runtime hash. A later raw-dispatch check does not claim to strengthen the legitimate old query→receive interval.

GuardedIgnoredPolicy credits only old ignored-body activity under existing token-held scope, without manager receive, requests/grants, I/O or release; stale denial precedes activity and cleanup while physical receive bounds remain charged. Message-budget Connected exit remains capabilities-only, distinct from close/Idle/cancel completion. Real compact fallback→all-prefilled CompactBlock and separate BlockTxn completion→later dispatch/finish, poison/unavailable, ABA, partial-known-prefix followed by actual later preparation error, order/caps/physical accounting and all error joins remain mandatory.

Only genuinely test-only unbound wrappers/adapters/reexports may use ordinary cfg(test) after full actual caller proof; shared selection/request/batch cores stay production. No suppressions, public widening, artificial uses or hiding new authority APIs are allowed. The parent PeerRequests import is explicit after the exact 198-line/6,709-byte prefix; child imports do not fund parent resolution. The genuine proved-genesis provider/private raw producer and existing long-provider semantic controls avoid broadening Sync-private fixture visibility. Both staged and joint selectors include the actual Sync serving module, not merely authority tests.

## Ownership and fit are explicit gates

46's six owners are authority parent, new peer_sessions, network root, serving, new private BASIC tests and new private registry child. 45's eight are Session, block_reconcile parent, new request child, original protocol-failure test parent, new lifecycle child, block_response, emission_terminal and new outbox child. Total fourteen is eight existing bodies/six true absences. The never-created old Session request child is excluded.

The two new children are cohesive conventional modules. Narrow parent reexports preserve existing public paths/constructors/generic ID APIs and exact effective field/helper privacy. No include/path trick, type erasure, TLS, raw mutable escape, extra owner or new public/test-access API is funded.

Measured thirteen-body history remains FIT-HOLD: normal emitter 660 exceeds 628 before required queue cases; frozen registry is 385, not its earlier preliminary 207. Prospective allocation is expressly not fit proof. BEFORE any actual46 source release, complete fourteen inert API/caller/control bodies, normal scoped Rustfmt, every file≤628/new test child≤620, cohesive functions around161, exact old bodies/privacy, actual required cases and independent/root fit acceptance are mandatory. Excess requires checked refinement; no packing, blank deletion, skipped controls, suppression or undeclared owner. This review did not inspect the mutable fourteen-body proposal and does not approve it.

## Staged gate and final closure

The separate stage DAG is acyclic:

`34 → actual46 implementation → real tests/format/build + unsuppressed no-D diagnostics → independent API/exact-diagnostic/root review → versioned NOT-FINAL ready receipt → separate45 SOURCE release → both original behavioral RED before45 edits → actual integration → one joint final source frame → original45 finals + fresh affected46 tests/build/format/strict → independent CORE/evidence/diagnostic closure → final46 acceptance/Summary/QA → final45 acceptance/Summary/QA → actual35 composite/root review → separate35 SOURCE and nine fresh gates`.

The schema is not an actual receipt. Its strict/final/Summary/commit flags remain false. Only exact actual unused internal API warnings solely awaiting a specific genuine45 consumer may remain after source-explained independent/root review. Other diagnostics/compile/test/panic/API defects must be fixed; no category waiver or no-D-as-strict claim. The explicit source exception does not invent a46 Summary, increment accepted counts or trick a default scheduler. API changes reopen46 and evidence through root.

Final joint strict -D warnings, every original45 final argv and ALL fresh affected46 authority/BASIC/actual Sync-serving/branch/transport/peer-filter/build/format require actual nonzero discovery/outcomes on one final frame. All transitional diagnostics must close honestly. Neither final acceptance/Summary/count/commit occurs earlier. Whole-workspace Rust/native precommit and hooks are unchanged.

## Authentication, originals and composite

All 344 R5 indexed files and nested authenticated input/history/source/direct/native/provenance/caller/control refs were checked. Every recorded current non-43 source observation/absence was independently compared, with process-start/execution-before claims false. Actual43 source and its captured AF17 planning copy are volatile and unaccepted; copied observations do not establish a long-lived current43 invariant or source acceptance.

All old plan task XML/automated argv/fences are exact except the explicitly rewritten45 tasks. Old44 is whole-byte exact to its published body. Only old35's dependency/new appendix changes among other old plan bodies. All original35 six focused/three final commands and all nine original45 final commands are independently preserved. Numeric46 before45 is intentional actual dependency order; neither repair depends on failed/unaccepted35.

Actual owned scratch ran BSD `patch --batch -E -F0 -p1` forward→inverse→forward. All 46-before/48-after metadata bodies and membership match exactly at each step, restoring both new metadata absences on inverse, with no offset/fuzz/reject. This is metadata planning evidence, not a source inverse or application permission. Publication still requires root's fresh unrelated/current beforeimages and exact target-only apply; never bulk-copy the frozen candidate over volatile43 or other metadata. Root's separate publication-ready package has not been independently re-audited by this report.

The original direct35 authority remains the exclusive original5b0 eight whole bodies/three true historical absences, independently matched against its capture. The copied 201 native inputs authenticate without opening any fixture/engine. Expanded membership independently derives to **158 total / 147 protected** at this planning snapshot, including six volatile43 paths; those are provisional, not accepted source membership. Old historical or conditional counts do not fund actual coverage.

Fresh actual accepted43/44/46/45 source CORE/root bodies/new children and truthful unchanged causal origins must determine the final composite. A dependency-ready46 is not a final accepted origin. Complete per-path copies/hashes/provenance, source inverse restoring only original direct35 8/3 while preserving every accepted repair/native input, and root review are required BEFORE a separate35 source release and all original six-plus-three gates. Historical full35 failure remains failed: seven prior targets passed, Node four failures/four inherited ignores, later integration/rustdoc unreached. Old parser limits/reasons/source frames remain evidence, not new acceptance.

The full remaining scope stays open: observer predecessors and actual13/14, formal15/helper/ordinary301/reopens150/301/next302, separately seeded16, compact C/D/E with D\<5 and E→17, final source before17 including123, native/security/mutants18, parity/README/architecture/breadcrumb19, truthful PROJECT/AGENTS25, full default20/native/fresh pure-core coverage/Bazel/source/security/goal/lifecycle and normal-hook clean main commit/push. STOP before168.

## Structured result

```yaml
status: passed
scope: frozen_R5_planning_only
issues: []
prior_findings:
  R4-PC-01: closed_in_plan_not_implementation
  R4-PC-02: closed_all_four_candidate_format_checks
prior_R4_HOLD_preserved: true
complete_fourteen_body_fit_accepted: false
source_release: false
dependency_ready_receipt_earned: false
source_runtime_or_test_acceptance: false
strict_clippy_passed: false
final_46_or_45_acceptance: false
Summary_or_commit_release: false
global_CVLD_status: all_five_OPEN
stop_before_phase: 168
```

No application, compiler, Cargo, Bazel, test, native engine, failed fixture, worker session, actual source, canonical metadata, Summary or Git mutation was used. Only this new ignored independent review directory was written. Root publication and all subsequent source/runtime/final release decisions remain separate.


# R6 independent planning and publication precision — complete reports

Prepared 2026-10-23T23:29:54.761Z for separate root whole-delta review/publication. The following two reports are complete immutable earlier checks: full planning PASS with bounded warnings, then exact ready-v1 PUB-01 closure. Their then-unproved source-readiness statements remain historical. Later independent complete16 review46101c49/root static-readiness3fce4999 records SOURCE_ONLY_CLEAN copied source only; it does not retroactively change either report, supply compiler/runtime proof or release actual46. R6-SCOPE-01 remains accepted only in principle under its complete sixteen-body gates. Counts remain46plans104tasks35waves/32accepted; actual43 rerun is underway. All whole167 gates remain;STOP168.

# R6 independent plan check — PASS with bounded warnings

**Planning PASS: zero semantic blockers, two nonblocking warnings.** Frozen R6 describes concrete production seams, genuine reachable/component controls and the same mandatory source/final gates. Warning R6-PUB-01 requires correcting one stale `measured14-body` phrase to `measured16-body` in a separately authenticated publication-ready copy. Warning R6-SCOPE-01 records the explicitly selected nine-owner integration scope and its mandatory complete-source fit gate. Neither warning permits source application, reduced controls or a premature acceptance claim.

This verdict is exclusively about the frozen metadata contract. It does not close the five source findings, inspect the mutable sixteen-body proposal, establish actual fit/type/privacy/compiler/runtime correctness, earn a dependency-ready receipt, accept source, release a Summary or authorize commit/push. The complete14 static HOLD and all earlier evidence remain unchanged.

## Immutable inputs and scope

Frozen `revision-v6/index.json`: **149,989 bytes / SHA-256 `9f8fb3e7f440f553857ea18e6cd3e354aebd7c7a0a3f9f6d6c197d95c4295172`**, with 606 indexed files. The independent proof authenticates all indexed files plus nested references, 1,060 unique paths total. Whole plans45/46, companion, revision map, handoff, root release, selected seam assessment, prior complete14 review and copied repair resume were read. Stage/schema, DAG, full-current metadata, owner/causal/provenance/native/direct-before/composite/inverse/caller/control records were inspected and independently checked.

| Principal input              |  Bytes | SHA-256                                                          |
| ---------------------------- | -----: | ---------------------------------------------------------------- |
| Root R6 seam release         | 10,849 | 1802b0785fe272bc9224afda0b2e554f892c542b69b6e433c70048b35f793474 |
| Selected assessment          | 24,287 | ddbc4b2cc180a5ac46484388a96f452d2789029fec64acec10297f0ec08d2311 |
| Prior complete14 HOLD review | 30,033 | bc5f85c0f0312e2cab23507b7ef6eb4b4e547961b5c3e481df32be98fb7d112b |
| 167-46-PLAN.md               | 40,896 | 2b058e163d015496230af6090bf2236f487fb5fbaa1b5aa5763c7407ccd6b5ba |
| 167-45-PLAN.md               | 41,517 | 83bdda4f51b2d89d955997258c724fb68248b604085c97ab6bc80aadd8d18e5c |
| 167-45-46 companion          | 46,322 | 7e1ddc04784b42126657e62d679a545793327edb84b5254b813953b0937234ab |
| Revision map                 |  7,452 | 1933d449d8aa38ec01ba52b1b0fd75ee3253d46134c950d7d2500cf6f9880119 |
| Handoff                      |  3,454 | c52518510020295c35c7d591373a13549c7841cb9767a829327308bebea87477 |

R5 is actually published, as recorded by publication `5bb0ef3b…` and root planning acceptance `7261b225…`. R6 starts from those whole current bodies, rather than treating the old candidate as an unpublished predecessor. The R6 patch changes only four EXISTING documents:35,45,46 and their existing companion. No plan, metadata file, task, dependency or wave is added. Source owner declarations expand explicitly; source bodies remain unapplied. Actual43 owns a separate sole lease and remains volatile/unaccepted here.

Repo AGENTS/Bright Builds sidecar/overrides, architecture/code-shape/testing/verification/Rust guidance and active lessons govern this review. Both active lesson files were loaded wholly within startup limits; no archive or new audit was needed. No project skills were found. Research open questions are marked resolved. Nyquist is skipped because configuration disables it, with all specified automated gates retained.

## R6-PUB-01 — isolated publication precision

**Severity:** warning. **Location:** 46 Task2 action, line175: `measured14-body/source/test-fit gate in companion is mandatory`.

The operative R6 interfaces in BOTH plans and companion, allocation, source-owner receipts and explicit source-stage ancestor require complete **sixteen** inert bodies/API/control/lock/caller/privacy/normal-fit independent/root review BEFORE ANY actual46 source release. They expressly supersede historical fourteen-body fit. This single stale phrase therefore does not remove two owners or create a reduced gate, but should not be carried into publication.

Required precise correction: preserve frozen `9f8f…`; use a separate ready copy to replace only `measured14-body` with `measured16-body` at this occurrence, then authenticate the exact delta, unchanged argv/fences/other technical text, all four target formats and fresh whole-metadata inverse/current-before protection. This is not permission to edit the frozen candidate or to claim source findings closed. Root publication remains separate from this verdict.

## R6-SCOPE-01 — explicit nine-owner integration scope

**Severity:** warning, already explicitly selected by root and recorded by R6. 46 has three tasks/seven owners;45 has three tasks/nine owners. Task2 has five files in each plan;45 Task1 has four. The new conventional private request-test child hosts genuine consumer controls and may cohesively relocate only actual existing cfg(test) unbound adapters/wrappers. The private disconnect child owns a real production phase boundary plus defining controls. This is a coherent split rather than a hidden extra owner or artificial task padding.

The 617→627 request-parent observation and source605/612/619/622 constraints justify the ninth owner in principle; they do not prove final normal fit. Before actual46: all sixteen whole inert bodies, every mandatory genuine case, full caller/module/private-export/lock contracts, ordinary scoped formatting, all files≤628/dedicated tests≤620/functions around161, independent review and root acceptance remain unproved mandatory gates. Imports/breadcrumbs count. Any excess returns a checked refinement; no seventeenth owner, packing, blank deletion, shortened case, suppression or visibility/test-hook escape. No further split is required by this metadata check while those boundaries remain enforceable.

## SRC03: concrete origin-bound continuation and real denied consumer

The contract appears in both plans' interfaces and companion lines121–127;46 Task1 implements the factory/API,45 Task1 adds private genuine controls and Task2 integrates the actual consumer.

The named crate-private borrowed `SyncPeerContinuation<'a,S,V>` captures the originating handle reference, original token reference, real PRIVATE raw dispatch and original parameters. The actual begin factory performs real raw dispatch. `resolve(self)` consumes the object and accepts NO handle/token/parameter substitution. Legitimate concrete-type exports pass through private child→authority parent→network root; private modules/lib/external APIs stay unchanged. No Clone/Copy, constructor/raw-result accessor/guard escape/test bridge/TLS/type erasure/Drop I/O is funded. This is an affine production contribution, with must_use explicitly not represented as a linear theorem.

Normal `PeerRequests::dispatch` must call begin→resolve IMMEDIATELY and then the old raw activity/settlement in the SAME operation/error order. All actual callers must consume/settle; dead combined private wrappers are retired only after full caller proof, or genuine tests migrate. New production APIs cannot be cfg-hidden, artificially used, publicly widened or suppressed merely to avoid diagnostics. No guard spans the pause and no callback mutation hook is added.

The new private defining test earns its predecessor H1 through actual manager request/runtime handoff, then renews; actual A NotFound cancellation leaves C's legitimate shared globalH1. It earns/hands off distinct H2 without replacing the predecessor. A real BASIC raw intent with the public proved-genesis provider/original factory token produces the continuation; a legitimate clone removes A/readds same-ID B before bound resolve. This earns a real Replaced denial carrying rawH2. A Ping without BASIC intent cannot stand in for this later-denial boundary.

The SAME private production settlement must retain both distinct earned sets and genuine errors, grant no cleanup and refuse ordinary renew/request/dispatch/classification. Real B-present and foreign-runtime finishes refuse with exact conservation of both runtimes. B removal followed by fresh original-runtime absence authorizes both fallible existing all-peer releases; C's exact shared/different ownership/order/caps/globalH1 remain. A's observations can finalize after authorized absence and successful releases while C still owns globalH1: global zero is not a completion condition. A ownership, real denial or release failure cannot silently retire obligations. No arbitrary inserted slot, copied error, forged outcome/received block, capsule mutation or maturity/fixture shortcut is allowed.

Actual `require_absent_sync_peer(token)->Result<(),AuthorityError>` performs registry identity/epoch/current-membership validation WITHOUT request capture, disconnect or queue permission. Only actual absence returns Ok, and it runs before EITHER fallible known-denied release. Real present-same StillPresent, foreign, epoch and poison cases belong at that real helper. Genuine consumer replacement/foreign-runtime/absence and actual scope-panic poison cases compose with them. R6 expressly rejects invented present-same-after-Denied recovery history or a poison-recovery/test API. This is honest compositional evidence rather than a fabricated Cartesian matrix.

## SRC04: achieved effect, real pruning and guard-free wake completion

Both plans and companion lines129–133 bind this to46 Task2's five explicit owners. `PendingDisconnect` holds the ORIGIN handle reference, SAME actual authority guard and real raw-disconnect result. Its private production factory receives the held guard, checks the exact token SECOND time and performs actual raw disconnect after the shared semantic preparation earned InvalidDisconnect. No arbitrary result or manually forged pending object supplies the achieved effect.

`release(self)` performs REAL pruning, ALWAYS collects all actual wakes even when pruning fails, and explicitly DROPS the actual guard BEFORE constructing the guard-free owned phase. `UnlockedDisconnect` carries only owned real operation/prune results and collected wakers; no guard/reference/pointer/guard-capturing closure/reacquisition hook remains. `finish(self)` invokes existing wake_all, then constructs the real semantic primary and preserves the operation/prune join in old order. A post-effect error is Operation inside Authorized; genuine pre-effect authorization failure remains Denied. Normal production invokes pending→release→finish immediately. No second BASIC algorithm, early `?` skipping wake/unlock, bare arbitrary-error helper or Drop I/O is introduced.

The defining control uses real public proved-genesis startup/factory/handshake, actual held token check and shared complete semantic-invalid preparation, actual pending factory/raw removal, then a joined thread genuinely poisoning the actual registry AFTER the effect. It observes real prune failure and lock/member state BEFORE finish, using actual try_lock and relevant existing probes. It must assert both genuine semantic and cleanup causes classified Operation, retain separate pre-effect Denied and old outer Authorized cases. This is a real shared production component control; the plan does not pretend full public dispatch paused itself between private operations.

Genesis wake collection is explicitly EMPTY. Typed guard-free completion and actual locks-before-finish prove the release boundary. Existing genuine nonempty reentrant/latest/direct-poison readiness cases supply common wake-stage evidence, while old outer semantic controls supply Authorized mapping. No fake waker/frontier/register(NonePending), empty-vector callback claim, new long-history qualification or whole-public post-effect interleaving is required or claimed.

## Actual readiness placement and matrix mapping

Independent current comparisons authenticate all nine readonly readiness/registrar/provenance copies. `sync/tests/filter_index.rs` registers `catch_up`; catch_up.rs lines16–17 path-includes the physical authority readiness tests as `phase159_readiness_tests`. That file path-includes its failures/interleavings children. Therefore the real Rust test prefix is **`sync::tests::filter_index::catch_up::phase159_readiness_tests`**, even though source physically resides under authority. An authority-only selector cannot prove those cases ran.

The copied interleavings source contains actual `ReentrantWake` with lock/publication probes and reentrant summary read, genuine pending accepted history, wake-after-both-guards control, latest-waker-once control and direct-dispatch-poison cases. Query derives Pending only from genuine accepted-but-unprocessed synchronized target and otherwise returns actual Found. Old wake_all invokes the collected wakers. These facts support the selected composition, without executing a test or qualifying a new fixture.

The companion adds an explicit `--all-features` readiness selector in BOTH preconsumer (line147) and final JOINT (line187) matrices, requiring actual nonzero named cases. Matrices also retain lifetime/new-name, authority, real Sync serving-family, branch, transport, peer-filter, format/build and final strict closure. The serving source similarly compiles under the actual Sync fixture selector. All nine original45 finals remain byte-exact, including fullSync and strict all-target/all-feature Clippy. Source matching proves placement, not execution/discovery/pass count.

## SRC02 and preserved R5 boundaries

Active queue scope permits `Some(callback_result)` ONLY for present original membership. Actual original absence with a valid live Weak yields `None` with NO queue callback or manufactured error. The registered sender returns an explicit status; ALL FOUR actual Session sites must consume false immediately as proper absence completion BEFORE further read/request/scheduling. Preserve old genuine lifecycle finish==2/GetData-removal success and all request/accounting assertions. No error-on-legitimate-absence prototype or absent active drain is permitted.

The no-B removal-during-send regression uses TWO genuinely produced A emissions, real transport and exact queue/FIFO/readiness/capability/wire preservation until separate final cleanup. FINAL queue authority remains distinct: absent original A with still-valid Weak can clean its own nonempty queue, while B created-then-removed invalidates A forever. Replaced/foreign/epoch/poison remain actual errors. No false return grants release of replacement facts.

All R5 obligations remain intact: private strong identity plus separate one-Weak map; absence does not prune live Weak; dead-only maintenance and live-identity bounds; every actual created admission on all3 routes, including error-after-created, invalidates prior identity; duplicate preserves; factory installs new identity; checked nonce/epoch/shared registry, no growing tombstones/resettable effect substitution. Memory queue selection uses actual authority→brief registry→expected-Arc compare/take/remove while SAME authority stays held, drops every lock before encode/transport/complete/abort/storage/await, uses no mutate/BASIC wake I/O/request scope expansion and audits actual reverse-held callers.

Preregistration precedes connect. Token-none cleanup is matching EMPTY-only; nonempty gives a genuine ownership error joined with real primary while preserving queue/caps, no Compatibility fiction or abort. Delayed PRE-A producer enqueue is explicitly outside per-emission lineage proof; existing physical affine settlement remains exact, with no universal future-send theorem or fabricated introspection.

Fresh Block/NotFound policy still queries actual current original-peer manager AND runtime ledger AFTER preceding cleanup under token scope; retained vectors remain separate cleanup candidates. Real manager-only/no-handoff and canceledA/B-shared negatives/handed positives remain. Request selection/storage preparation stays outside the synchronous memory projection; token validation, actual manager effects/globalexpiry/GetData/cap checks and runtime insertion share the operation, with old known-prefix/later-error per-item effects retained. No nested handle/I/O/await/raw escape.

GuardedIgnoredPolicy records only old intentionally ignored-Block activity under token scope, no raw receive/request/grant/I/O/release; stale denial precedes credit while physical budgets remain charged. Connected budget exit remains authorized capabilities-only, distinct from close/Idle/cancel completion. Compact fallback→all-prefilled and separate BlockTxn completion→later dispatch/finish, earned observations/at-most-two sets, unavailable/poison, genuine errors and unchanged all-peer release/error join remain required. Original parent198lines/6709bytes and all22 legacy controls stay exact;30/31 source-named observations are not discovery/results. SRC01 production InventoryType import and SRC05 unrelated unused import are explicit correction obligations, never allowed transitional warnings.

## Goals, scope, graph and stage gates

Both frontmatters retain CVLD-01/02/03/04/06 and full must_haves/truths/artifacts/key links. D-07–D-10 map to these actual authority/integration/evidence stages; D-01–D-06 retain complete original selected-network/ancestry/script/time/overwrite owners. No locked decision is reduced and no deferred168–171/GUI/wallet-product/public-network-default/automatic import/repair/differential qualification work is introduced. All five global CVLD IDs stay OPEN.

Independent GSD structure validation passes both three-task plans with no missing files/action/automated/done or structural errors/warnings. The graph remains **46 plans /104 tasks /maxwave35**, with32 existing Summary files in the captured metadata, no new accepted contribution. Every old dependency/wave is exact;46 depends34/w18,45 depends34+46/w19,35 remainsw27. Shared source-owner ancestor chains are valid. Neither repair depends on failed35.

The explicit TWENTY-node stage graph is independently acyclic. Formal R6/root publication AND complete16 inert independent/root readiness precede separate actual46 SOURCE. Then real46 implementation/tests/format/build/UNSUPPRESSED no-D diagnostics→independent source/API/exact actual-warning/root review→versioned NOT-FINAL receipt→separate45 SOURCE despite absent46 Summary→both old genuine RED before45 edits→actual integration→one JOINT final source frame→all45 old finals+fresh ALL46 affected matrix/build/format/strict→independent CORE/evidence/diagnostic closure→root46accept/SummaryQA→root45accept/SummaryQA.

The schema is NOT an actual receipt, strict/final/Summary/commit FALSE. Only exact actual reviewed unused internal APIs solely awaiting a named real45 consumer may be residual at dependency readiness. Other diagnostics/compile/test/panic/API defects must be fixed; no allow/-A/public widening/fake uses/cfg-hiding new API or no-D-as-strict claim. No Summary/count/default-scheduler fiction unlocks source. API rework reopens46 via root and renews evidence/receipt. All residuals must close on fresh combined source before either final acceptance/Summary; final strict has zero warnings. Precommit whole-workspace/native/hook requirements remain separate and mandatory.

## Independent authentication, inverse and original contracts

`proof.json` records **3,171 successful assertions /1,060 authenticated paths**, including all606 indexed files, nested history/input/causal/native/direct/provenance records,92 current non43 observations/absences and honest copied-HEAD equality flags. HEAD blobs are authenticated captured provenance, compared to observed copies; no fresh Git command or retroactive process-start identity is claimed. All owner states match the authorized root release:16 total,8 whole existing bodies/8 TRUE current absences. Never-created Session request child stays excluded.

Actual owned ignored scratch runs BSD `patch --batch -E -F0 -p1` forward→inverse→forward over **all123 current metadata files**. Membership and EACH whole body match candidate/before/candidate exactly, with no offsets/fuzz/reject and no new metadata absence to invent. Only the four existing target bodies differ. This proves planning inverse only; actual source inverse remains future evidence. All four actual changed candidate bodies pass the required GFM/frontmatter formatter independently.

The COMPLETE publishedR5 35 prefix is unchanged before the new context-only appendix, including original task XML/fences/SIX focused+THREE final argv. All other old task XML/argv/fences remain exact except explicitly revised45/46 actions; their task automated argv remain exact too. All nine original45 final argv and old companion history are independently exact. 44's whole published15,187B body remains SHA `0fa6f88215ed7f0d6e2b864bb9742f1ca75db924690af9453697ddcbf318d1a3`. Captured CHECK's entire572,923B prefix is preserved; no new Summary/check acceptance is invented by the candidate.

Current canonical protection was checked for every unrelated captured top-level body, permitting only append-preserved CHECK and explicitly volatile43 snapshots. Actual43 AF17 and six source paths are not long-lived invariants/accepted source here. Root must refresh all current beforeimages and43 at publication and apply only exact four targets, never bulk-copy the candidate. This review wrote no canonical/state/task/lesson/Summary/source/Git file.

## Composite35 and remaining whole phase

Original direct35 before authority remains EXCLUSIVE `5b0b899b3604e5d15e72bc3671d51c518c996ee359c4fe8441886333af8991c7`:8 complete bodies/3 TRUE historical absences independently matched. Current partial/GREEN35 is not an alternate before authority. All201 copied native inputs authenticate separately without opening a native fixture/engine. Expanded membership independently derives to **166 total /155 protected** at this planning snapshot; this includes extra readonly causals/new child paths and volatile43 snapshot paths, and is never accepted source membership. No historical or conditional count substitutes for actual final derivation.

Actual accepted43/44/46/45 per-path CORE/root bodies/new children plus truthful unchanged causal origins, complete copies/hashes/provenance and separate native evidence must define fresh35 composite. Dependency-ready46 is not a final accepted origin. Whole-file inverse restores ONLY original35 direct8/3 while preserving every accepted repair/native/protected input; forward restores final direct bodies exactly. Root reviews actual composite/inverse BEFORE separate35 SOURCE/all original six-plus-three fresh gates. Historical35 remains FAILED: seven prior passed targets, Node2279pass/4fail/4inheritedignored, later integration/rustdoc unreached, original reasons/parser limits/source frames intact.

Full remaining requirements stay mandatory: observer predecessors and actual13/14, formal15 helper/ordinary301/reopens150/301/next302, separately seeded16, compactC/D/E with D\<5 and E→17, full final source BEFORE17 including123, native/security/mutants18, parity/README/architecture/breadcrumb19, truthful PROJECT/AGENTS25, entire default20/native/fresh pure-core coverage/Bazel/independent source/security/goal/lifecycle and normal-hook clean main commit/push. STOP before168. No local planning PASS or interim source receipt closes these gates.

## Structured result

```yaml
status: passed_with_nonblocking_warnings
verdict: PASS_PLANNING_ONLY
semantic_blockers: 0
issues:
  - id: R6-PUB-01
    plan: "167-46"
    task: 2
    dimension: publication_precision
    severity: warning
    description: "One historical measured14-body phrase remains; all operative R6 gates require16."
    fix_hint: "Separate ready copy: replace this single phrase with measured16-body, authenticate delta/formats/inverse before publication. Preserve frozen R6."
  - id: R6-SCOPE-01
    plan: "167-45"
    dimension: scope_sanity
    severity: warning
    description: "Nine explicit owners, three tasks; root-selected private test allocation is cohesive but final16 fit/control/API correctness is unproved."
    fix_hint: "Enforce complete16 independent/root readiness before actual46 SOURCE; stop for checked refinement on excess, without hidden owner/packing/dropped controls."
source_finding_closure: none_claimed
complete16_fit_or_API_readiness: false
actual_source_release: false
dependency_ready_receipt_earned: false
source_compile_test_runtime_or_strict_acceptance: false
final46_or45_acceptance_or_Summary: false
commit_or_push_release: false
all_five_global_CVLD: OPEN
phase168: HELD
```

Only this new ignored independent review/probe directory was written. No mutable sixteen-body proposal, compiler/Cargo/Bazel/application/test/native fixture/failed datadir/session or actual43 source was accessed. The only executed checks were read-only metadata/source authentication, plan structure, Markdown formatting checks and the owned metadata-scratch inverse. Root publication and any later source/readiness/runtime/final release remain separate.


# R6 ready precision — PASS

**R6-PUB-01 is closed for this exact publication-ready packet.** R6-SCOPE-01 remains the explicitly selected nine-owner scope warning, accepted only in principle with complete sixteen-body normal-fit/API/privacy/caller/lock/control independent/root readiness BEFORE actual46 SOURCE. No source finding, fit, compiler/runtime, dependency-ready receipt, final acceptance, Summary or commit/push is approved here.

Checked ready index: **114,706 bytes / SHA-256 `79296ff7bc1c0a7307206a7c486dfac7c84ae0ea5e6a65005c7133483a654425`**. Frozen R6 index `9f8fb3e7…` and independent review index `87f1fe88…` are unchanged. Their full planning review remains authoritative; this is a precision follow-up, not a new broad technical proposal.

## Exact correction

Ready46 is 40,896 bytes / SHA-256 `cc6fef2652ef708cc987f8b80af4eaab675eff81eaae5e1c3761a5351280a715`.

Independent whole-string comparison proves it is frozen46 with exactly ONE substitution:

`measured14-body/source/test-fit gate` → `measured16-body/source/test-fit gate`.

The sealed `precision.diff` authenticates as 6,787 bytes / SHA-256 `b71afafc5d442ba5402733f1437105bb7e1502ac9ddeb4ce4dd4d882c969c554`. All other bytes remain exact. Ready35,45 and companion are whole-byte identical to frozen R6. Frontmatter IDs/requirements/owners/dependencies/waves, every argv/fence and all task XML except this authorized phrase are independently exact. Both ready plans pass GSD structure validation with three complete tasks and no structural error/warning. No scope/API/decision/verification change is smuggled into the correction.

## Whole metadata evidence

The check completes **929 assertions over258 authenticated paths**. It authenticates every ready before/after reference and the root patch/delta/prior review references. All126 BEFORE bodies equal their actual current metadata paths at this check, including123 phase top-level files and the three progress files. All126 AFTER paths are the same membership. Only the four existing technical targets change; progress/current CHECK/accepted Summary and QA/history/44/43 AF17 and other unrelated bodies remain whole-byte exact.

Root's sealed F0 steps authenticate with exit0 and no offset/fuzz/reject. Independent own ignored scratch additionally applies `patch --batch -E -F0 -p1` forward→inverse→forward; EVERY126 whole body and full path membership equals after/before/after exactly. No new metadata file or fictional absence is introduced. This is metadata evidence only, not application-source inverse or release.

Independent GFM/frontmatter Markdown checks pass all FOUR ready targets. Original35 six focused/three final and old45 nine argv are preserved through exact frozen-target identity/only-one-phrase proof and the prior sealed full review. No canonical file was written. Current43 source is outside this check and remains volatile; a later canonical drift still requires root refresh before publication.

## Disposition

```yaml
verdict: PASS_PRECISION_ONLY
R6-PUB-01: CLOSED_FOR_EXACT_READY_79296ff7
R6-SCOPE-01: RETAINED_IN_PRINCIPLE_ONLY
frozen_R6_and_prior_review_preserved: true
new_technical_contract_or_owner: false
actual16_source_readiness: false
source_findings_closed: false
actual_source_or_application_release: false
compiler_test_runtime_or_strict_acceptance: false
dependency_ready_receipt_or_Summary: false
canonical_publication_performed: false
commit_or_push_release: false
all_five_CVLD: OPEN
phase168: HELD
```

Root may use this precise metadata correction in its separate publication action after its final current-before check. All staged46 tests/build/unsuppressed diagnostics/independent API/root NOT-FINAL receipt,45 real original RED before edits, final joint original45+fresh46 strict/tests/CORE and46QA→45QA obligations remain unchanged. Actual per-path composite/inverse/root review precedes separate35 SOURCE/all9; full167 gates/hooks/main push remain mandatory. This review accessed no mutable sixteen-body proposal, actual source, compiler/Cargo/Bazel/application/test/native fixture/failed datadir/session or Git. Only this new ignored precision review/probe directory was written.


## Plan47 independent technical review and concrete publication preparation | 2026-10-24T01:36:42.998Z

The complete independent15875-byte report5ef7bcb7 below is preserved as the review of the earlier ignored proposal. It found no technical blocker and required five publication-promotion steps. Canonical-ready metadata now prepares those steps: lifecycle/generation presentation, exact canonical companion/dependency links, old-task-preserving DAG, full metadata before/TRUEabsences/F0 proof and separate prerequisite/source gates. The306-retained absent-query clarification is outside the old task XML. Concrete publication precision and ROOT acceptance remain required; no source/build/strict/Summary/count/Git permission follows. ROOT alone refreshes volatile progress at publication. See167-47-PLAN.md and167-47-SUBMISSION-OBSERVATION-AMENDMENT.md.

# Plan47 formal proposal check

Status: **technical plan content acceptable for canonical-publication preparation; canonical publication HOLD**. There are no identified technical design blockers. One publication blocker remains: this is an ignored proposed package with proposed-only lifecycle metadata, not an accepted canonical Plan47. This review releases no source, build, Summary or phase acceptance.

Scope: active lifecycle `167-2026-10-21T16-38-23`, yolo, commits:false. Only this assigned ignored review directory was written. No canonical/source/Summary/count/Git changes, application runner imports, product execution, Cargo, Bazel, database opening or process signals occurred. Product sources were read to check feasibility; no fresh source-before capture was made. ROOT's ongoing46 repair and sole build ownership remain untouched.

## Evidence and method

Whole sealed proposal index:20468 bytes, SHA256 `9e64a93bc6cf12ae7feaf11a707a993b4c630fcb153e3ec70c490ffd4550e9e1`. All75 referenced occurrences independently match their recorded bytes and SHA256. Whole assessment:13420 bytes, `19ac136654f98cd650fe9114650c487774409571b397bd2bbce01bb058535760`. Whole proposed47:21407 bytes, `ac140db8a81a4af53a262f5ce15c2b3ec7133c612c8d77411bf21e9f4aa9ef3a`. Handoff:3177 bytes, `30e3ca1fbcf10459bec18e9a2d9c9b8b5ad84d4cfb9505e918d8557dd0505828`.

Original whole readiness:20490 bytes, `4c83efed26d31ba41e6d50d336882482912c6534824e695e3e373c67010ecf95`; complete bindings:11791 bytes, `b88ca2bee64a435e4d0ae0d23d45d3f63fabb083aa1e21153f8e079d702176f3`; ROOT readiness receipt `0906ca6638cb0f8dbe26f832bf8ebf960797fa38fee111bb4404488e90924d4c`. These preserve49 historical source observations/six historical absences. They are not fresh before authority or current35 acceptance. R6 publication `98be80e41b14d0785b7be604437294383bd505d576e3e793e93dc0d9920a99e3` and planning-only ROOT acceptance `62c224ff2bfe732eafe9bbc42050a833a4786ea9679c55817516589a0450d63f` were also read.

AGENTS.md, Bright Builds sidecar, overrides, architecture/code-shape/testing/verification/Rust standards and GSD gates/thinking/calibration references materially informed this review. Both active lesson files were loaded completely:7880+1958=9838 bytes,2627+653=3280 conservative estimated tokens. No archive or lesson ledger was changed. Neither project skill directory exists.

`metadata-check.json` contains independent75-reference authentication, complete old/new DAG, byte-exact old task/automated-command preservation, all nine literal new argv, and successful GSD plan-structure output. Its metadata-only checker imports no application modules. An initial overstrict metadata assertion expected every inherited wave to be minimal; it found existing02 at wave5 with no dependencies. The corrected check records that unchanged R6 scheduling slack, requires every dependency to precede its consumer, and verifies exact minimums for all newly changed waves. That failed metadata attempt is retained explicitly; no product failure is implied.

## Goal-backward findings

The phase goal is pinned network/candidate-history decisions through ordinary header admission and durable body connection.47 supplies complete owner-bound observational evidence required by the later RPC continuation; it does not itself implement or close all phase validation rules.

| Dimension | Result and evidence |
| --- | --- |
| 1 Requirement coverage | All five roadmap requirements01/02/03/04/06 remain in aggregate plan frontmatter.47 correctly contributes to01–04;06 remains assigned elsewhere. No relevant PROJECT requirement is removed by this additive observer. All five global requirements stay OPEN. |
| 2 Task completeness | GSD structure valid, no errors/warnings; two auto/tdd tasks each have concrete files/action/verify/done plus behavior. Missing-API compilation is explicitly not behavioral RED. No unsafe/stub implementation is permitted to manufacture RED. |
| 3 Dependencies |47 depends30+35; actual45/46 are ancestors through35.13 retains12+30 and adds47. Graph acyclic, no missing reference or changed source-stage exception. |
| 4 Key links | Actual command→bounded inventory→consuming row extractor and parent→new test-child registration are explicit. External inference doctest and later same-command Arc/closures are explicitly planned. |
| 5 Scope | Four unique node owners; two tasks touching4/2 files. Three future original bodies and one TRUE new-child absence. No present evidence mandates a split; normal fit remains an execution gate. |
| 6 Derived truths | User-observable complete state, honest freshness/projection refusal and ordinary external consumer access. Artifacts and real call wiring support those truths. |
| 7 Context | D-01/07/08/09/10 authority, refusal/evidence and lifecycle obligations remain. Existing plan task actions preserve phase decision references. No deferred differential/wallet/network/prune product work is introduced. |
| 7b Scope reduction | No silent reduced implementation found. Finite Regtest command/context observation is explicit; old all-four-selector genesis tests remain exact; stronger unrelated live-child/non-Regtest internals are not falsely claimed. |
| 8 Nyquist | SKIPPED: workflow.nyquist_validation=false. No validation-file scaffold gate applies. |
| 9 Shared data | Same exact command and same-open capture; owned complete values pass to RPC closures by inference. No destructive stream transformation or incompatible shared representation. |
| 10 AGENTS | Read-only shell boundary, typed/private authority, standard Rust child layout, ordinary dependency access and normal file caps retained.47 scoped checks do not replace the later full native/Bazel/coverage/parity/security/lifecycle gate. commits:false remains. |
| 11 Research | Existing Open Questions and Planning Resolutions section explicitly RESOLVED. This supplement introduces no new unresolved research decision. |

Proposed waves are47=28,13=29,14=30,15=31,16=32,17=33,18=34,19=35,25=36,20=37. All46 existing plans' whole task XML blocks and ordered automated occurrences are byte-exact in the proposed schedule, totaling104 old tasks; the proposed inventory has47 plans/106 tasks.13 retains all three current tasks/eight commands, including both original tasks/five commands.14 is unchanged except its wave. These are planning calculations, not live progress updates.

## Authority, coherence and actual API feasibility

The proposed observer is on the earned Regtest command, with a private-field owned return capsule and consuming public-data fold. Existing public factory return values support inferred use of hidden command methods without reexports. The ordinary external no_run doctest must consume all five exact public fact types; unit-test-only access is insufficient. No caller-supplied runtime/path/token, Default/setter, authority receipt, mutable engine, guard, lease or storage handle escapes.

The five facts fully retain effective ChainstateSnapshot/live Coins best/durable CompleteCoinsSnapshot, actual optional BASIC progress including separate branch, exact optional stored endpoint/fence-height/fence-hash/protection, all nine actual namespace maps including empty ones, and keyed independently read live/durable provenance. Actual definitions support these reads: manager export at chainstate.rs:588; full concrete Fjall admission-coin traversal at storage/coins_view.rs:385 and complete snapshot at292; live BASIC at chainstate/filter_index.rs:242; stored BASIC at storage/fjall_store/filters.rs:51 with StoredFilterState fields at storage/filter_index.rs:123; live/durable provenance at chainstate/validation_history.rs:303 and storage/fjall_store/validation_history.rs:296. The public provenance enum preserves all three variants. Concrete Fjall admission traversal is complete despite the generic trait's optional hint contract; this assessment does not substitute the empty generic default.

Required order is operation mutex→authority read→real owner/selected chain/parameters/incarnation/same-store check→bounded inventory BEFORE individual facts→all facts→owner recheck→unprojected captured-row correspondence and complete original logical stamp/catalog/current digest/final stamp AFTER facts→unlock authority and operation→owned fold. This matches the actual submit lock order. No consumer callback under locks; no reentering public authority calls, engine escape, confirm_durable, drive, flush, repair or normalization. Legitimate post-error divergence remains visible or yields its real operational refusal.

The projection trap is real in the read source: project_admitted_headers changes self.rows while source_digest returns the saved original source and check_current_source compares that saved source with current storage. Therefore check_current_source alone cannot authorize returning self.rows as actual. The planned extractor's captured-row digest comparison plus original freshness checks closes that hole. MutationStamp retains active-writer, same-value and ABA protection even when final rows/digest match. Tests require actual projected new-child/KnownOnly rows, successful original freshness, failed extraction and unchanged native storage, plus real tracked stale/same-value/ABA writes. No sibling-private projection hook is assumed.

Query input is capped306 before allocation, automatically includes all actual retained headers, and returns a deduplicated union of at most612 identities. Implementation must bound the retained count independently of whether an explicit probe is absent: an absent provenance query at306 retained headers is observational input, not a new candidate admission. Do not reuse the submit-only novel-candidate refusal in a way that rejects this valid query. The planned cap/automatic-retention contract already requires this behavior.

Controls cover enabled/disabled real factories, actual accepted/duplicate/refused child, stable-seven BASIC plus separate branch and stored fields, all maps/provenance, retained/absent hashes, exact/one-over bounds, genuine foreign-open/incarnation, actual post-absorption fault, detached data/full drop, reentrant fold, and genuine reopen. Reopen compares stable fields/maps while checking live-genesis versus recovered-tip branch anchors separately. Contextual invalid bodies may publish header/KnownOnly deltas; pre-effect/intrinsic/arity/decode/foreign all-map equality and post-absorption operational effects remain separate categories.

## Fit, verification and downstream obligations

Historical estimates remain374→524–594 command lines,295→310–320 projection lines,626→627 registrar lines, and300–480 new-child lines. Only ONE conventional registrar declaration is funded. Existing maximum628 and new-child maximum620 apply with normal formatting, no packing/deletion/test removal. If refreshed accepted35 baseline is too large, changed in a material interface/owner way, or final formatting cannot fit, HOLD for an explicit checked fifth-owner refinement such as a cohesive production child or shared test-fixture child. This review supplies no fresh source geometry, fit measurement or source lease.

All nine new commands match their shlex argv: two task observation commands, then seven final format/observation/actual command-module/projection-module/rustdoc/all-target build/strict Clippy commands. Read source confirms the real command and projection test module paths. Nonzero named discovery, raw failures/retries, no_run compilation classification, pinned environment, serial command-timings lock, natural global guard, polling within60seconds and actual final-source attribution remain mandatory. No command was run here. Final whole-file inverse/forward must restore three originals and exact new-child absence without discarding accepted repairs.

The additive13 handoff retains one inferred Arc of the SAME factory command and distinct private submit/observe closures. Generic initializers stay None; all Arc/context/closure/runtime/store handles drop before reopen. The node API is ordinarily compiled; RPC test-only observation state does not rely on dependency cfg(test). No extra RPC owner is added. Plan13's complete help/wire/normalization/result cases, authentication/default-off networking/unavailable wallet, real freshly built CARGO_BIN_EXE child, wide-output/no-Coins/undo/restart-next cases and all original tasks/argv remain obligations.14 retains truthful identity/implementation/qualification separation and actual301 CLI path.

The original13 task/final prose requires runtime BASIC/provenance comparison and real process stop/reap/drop before storage reads.47 and the handoff preserve that division. A parent capsule does not prove live child-process internals, and reconstructed stopped-state observations must be labeled and dropped before restart. All-selector genesis remains strict and unchanged; any later stronger non-Regtest/live-child interpretation must retain its explicit hold for a separately checked contract, never lower the current obligation to make47 appear sufficient.

## Canonical-publication blocker and exact promotion work

The ignored candidate must not be copied verbatim and called canonical acceptance. A ROOT-owned publication preparation must:

1. Produce canonical167-47-PLAN.md with generated_by:gsd-plan-phase and actual generated_at; remove proposal_only or set it false under the accepted convention. Preserve lifecycle_mode:yolo, exact lifecycle ID, commits:false and source_execution_released:false. The historical proposal/seal timestamp is not the canonical generation timestamp.
2. Rewrite the candidate's present-tense ignored/proposed status paragraphs in plan/companion to truthful planning-only publication status, retaining immutable historical evidence and every substantive/source-release hold. Publish the companion as167-47-SUBMISSION-OBSERVATION-AMENDMENT.md and ensure13 references that canonical path.
3. Apply ONLY reviewed dependencies/waves to13–20/25, without old task/argv rewrites or46 source-stage changes. Recompute/check full inventory and reciprocal context/decision/plan-check links. ROOT must own any truthful47-plan/106-task/37-wave planning counts and pending contribution accounting; do not claim another accepted contribution.
4. Prepare exact canonical before/after metadata and TRUE new-file absences, test scratch F0 forward/inverse/forward with exact restoration, authenticate the publication inventory, and obtain independent/ROOT planning acceptance. The current candidate metadata proof is insufficient for an unprepared larger publication delta.
5. Retain separate actual35/30/transitive45/46 acceptance and fresh source/API/registrar/geometry reconciliation before source release. Full final source/evidence, independent CORE/root and SummaryQA acceptance remain later gates. No publication timestamp or planning checker can satisfy them.

```yaml
issues:
  - plan: "167-47"
    dimension: "canonical_publication_readiness"
    severity: "blocker"
    description: "The package is intentionally ignored/proposed; generated_by is independent-readonly-preparation, proposal_only is true, generated_at is absent, and no canonical publication delta/acceptance is prepared. It cannot yet be treated as an executable canonical plan."
    fix_hint: "Perform the five exact ROOT-owned publication preparation steps above, preserving old tasks/argv and all source/acceptance holds; then independently check that concrete metadata delta. No technical design rewrite is required by this review."
```

Recommendation: proceed with canonical-publication preparation, then check that final concrete delta. Do not run47,13 or14 from this ignored package. All five global CVLD requirements, whole167 source/security/native/parity/coverage/Bazel/goal/lifecycle/Git gates remain OPEN; Phase168 remains held.


## Plan47 concrete technical precision and final ROOT publication gate | 2026-10-24T01:48:42.504307+00:00

The complete independent technical precision report below closes the14-target promotion scope. Its remaining final ROOT gate is preserved as a historical review condition: fresh current progress integration, complete metadata before/after and exact F0 restoration precede explicit ROOT planning acceptance. The final publication record is maintained under `.local/open-bitcoin-dev/phase167/plan13/root47-publication-v1/`; it grants no source execution or runtime acceptance. All old decisions/task blocks/commands and source35/45/46 anchors stay unchanged.

# Plan47 concrete technical publication precision review

**Technical promotion precision: PASS. PUB-01's technical promotion scope is closed. Complete canonical publication remains HOLD.** The three volatile ROOT progress bodies are deliberately unchanged snapshots, so this package is not the final publishable130-state metadata frame. No new technical defect was found.

This review is confined to the ignored publication-ready metadata package and this new ignored review directory. The earlier source/API/goal review remains the technical basis; no full old product-source review was repeated. No canonical/source/Summary/count/Git writes, product runner import, product/Cargo/Bazel/database work, source/native capture or other actor polling occurred. Only pure metadata checks and a patch against owned ignored metadata copies ran. Active lifecycle remains167-2026-10-21T16-38-23, yolo, commits:false.

## Exact input and authentication

The whole123560-byte publication-ready index SHA256 is `84b1caa2045d413a9eb47e4f2629f41020ecd9e1fe16acde5338d84c1de23c17`. Its whole8468-byte READINESS SHA256 is `e7b0b81555fc69f194b6f84429890514c5645343ee64e49dfff856b0fafcdc74`. All420 reference occurrences independently match their byte lengths and SHA256; the package has exactly its409 listed artifacts and no unlisted artifact. Whole-byte authentication is distinguished from the semantic review of the14 technical target deltas.

Whole canonical-ready47:22156 bytes, `35b3cc89bd83396a6c8a37b09d37fe549bcaee3219689cd71afc60df2f21187d`. Whole canonical companion:3920 bytes, `67212d0054284197f95c5700f43112ca13d66c655f306106698bfc561e13c78c`. All14 technical target deltas, the complete readiness, promotion diffs, progress proposal and metadata proof were examined. The old formal report remains15875 bytes/SHA256 `5ef7bcb7dd30ab792711579c1fdef9670c1ec6f6fc60fa06aee37e03b13d8b9a`; its original index remains2382 bytes/SHA256 `4677f078baeb1a46ed14111fee3c22256468e0a639b8faccb9805f0521c6b899`.

AGENTS/sidecar/overrides, architecture/code-shape/testing/verification/Rust standards, active lessons and GSD gate rules loaded during the immediately preceding formal review remain applicable. The candidate preserves the required private authority boundaries, bounded evidence and normal fit gates; this precision review adds no source scope or standards exception.

## Technical promotion and conservation

| Check | Independent result |
| --- | --- |
| Header/status | generated_by:gsd-plan-phase; actual preparation generation time2026-10-24T01:36:42.998Z; correct lifecycle/yolo; commits:false; source_execution_released:false; proposal_only removed. Current ignored-draft status language replaced by truthful planning-only status. No source acceptance is implied. |
| Canonical links | New canonical companion name is167-47-SUBMISSION-OBSERVATION-AMENDMENT.md.13 references47 and this exact companion outside every old task.47, companion, Context and Coverage links are reciprocal. |
|47 substantive conservation | All reviewed task XML, ordered automated occurrences, entire interfaces block and bash fences remain byte-exact. Same four owners/two tasks/4+2 files, caps, locks, five facts, normal inference, source gates and nine commands. |
|306 clarification | Separate outside old task/interface blocks; an absent queried hash at306 retained headers remains observational input. Independent306 query/retained bounds and deduplicated union≤612 remain. No new admission authority, behavior reduction or larger bound. |
| Old plans | All46 old plans' complete task XML, ordered automated occurrences and shell fences remain exact.13 retains three tasks/eight occurrences and its two original tasks/five commands.14 remains unchanged except its wave. |
| Schedule |47 depends30+35;13 retains12+30 and adds47;45/46 remain ancestors through35. Graph acyclic; every dependency wave is earlier. Changed waves47=28,13=29,14=30,15=31,16=32,17=33,18=34,19=35,25=36,20=37.47 plans/106 tasks/37 waves are prospective planning shape only. Inherited02 wave5 with no dependencies remains unchanged. |
| Frozen R6 anchors |35 `e00e4186b38d7eb028f5477fc46d966e981a1f1b8101cd54327ae0ad224aa285`,45 `83bdda4f51b2d89d955997258c724fb68248b604085c97ab6bc80aadd8d18e5c`,46 `cc6fef2652ef708cc987f8b80af4eaab675eff81eaae5e1c3761a5351280a715` remain exact. No R6 source-stage exception is altered. |
| Append-only metadata | Old Context/Coverage/603193-byte CHECK prefixes remain exact. CHECK appends the whole original15875-byte review exactly once, framed as historical review of the earlier ignored proposal and its promotion requirement. No raw standalone body frontmatter separator is added. Old findings remain historical, without pretending the old report accepted this later delta. |
| Metadata tools | Independently reran GSD plan-structure on ignored47 and13: valid:true, errors:[], warnings:[], two/three tasks. Authenticated the successful scoped new47/companion formatting record; no existing metadata normalization or product formatter was run by this review. |

The14 changed phase targets are nine existing plans13/14/15/16/17/18/19/20/25, append-only Context/Coverage/CHECK, and the two new47 files. All116 remaining present bodies stay protected. No accepted Summary is changed. Technical preservation and presentation promotion close the corresponding part of PUB-01; they do not constitute actual canonical publication.

## Independent full metadata F0 replay

Independent scratch was created only under this review directory from the sealed metadata before copies. The baseline has125 phase files plus three root progress files:128 complete present bodies and two true new-file absences,130 states. The after has127 phase files plus the three unchanged root progress copies:130 present bodies.

The authenticated1712441-byte technical patch SHA256 `bb8cd278413aa205aa6921a6de4107182953358ebdd07506d0a0a8c04b4c61b7` was applied with `/usr/bin/patch --batch -E -F0 -p1` in forward/inverse/forward order. Each step exited0 with no offset/fuzz. Complete membership and every state matched:130 after bodies on both forwards;128 original bodies plus the exact two new-file absences on inverse. No additional scratch file remained. `f0-1.json` through `f0-3.json` retain the exact argv, streams and exits; `precision-check.json` retains full reference authentication, frame comparisons, task conservation and DAG.

This proves the reviewed frozen technical patch is exactly reversible against its recorded metadata baseline. It does not prove the live canonical baseline is still identical, and it does not authorize applying these frozen volatile root copies.

## Remaining ROOT publication gate

STATE.md, ROADMAP.md and .codex/tasks/todo.md are byte-identical between this package's before/after frames and explicitly marked **not final publication bodies**. The prospective progress proposal correctly performs no live write and no source acceptance increment. Its current planning arithmetic34/47 accepted contributions,47/106/37 planning shape, milestone104/117 rounded89% and3/8 verified phases is a proposal, not an independently accepted runtime/current counter frame. Actual46 is not counted ready or accepted by this review.

Before canonical publication, ROOT must freshly resolve current metadata membership/bodies, integrate targeted truthful progress into those three files while preserving every historical/current46 failure and execution record, and renew the complete final130-state before/after/inverse frame. If current membership has legitimately changed, reconcile that explicitly instead of forcing the historical130 count. ROOT must then whole-review/authenticate the concrete final frame, prove fresh F0 forward/inverse/forward and issue explicit planning publication acceptance. Do not copy the current frozen root snapshots over later ROOT edits. Technical14-target precision passes do not discharge this remaining gate.

After publication, source execution still requires actual30/35 and transitive45/46 acceptance, fresh accepted API/registrar/normal-fit/three-original-body/TRUE-child-absence/native reconciliation, separate sole-actor ROOT source/build release, genuine commands and failures, source/API/privacy/lock/evidence reviews, inverse and CORE/root/SummaryQA.13/14 remain behind actual47. No source geometry or actual observer test evidence was supplied here.

```yaml
issues: []
closed_scope:
  - id: PUB-01
    scope: technical canonical-promotion preparation only
    result: closed against the authenticated concrete14-target delta
remaining_publication_gate:
  id: PUB-01-ROOT-FINAL
  severity: blocker
  description: Three volatile root progress files are unchanged snapshots; final fresh integrated whole-publication frame and ROOT acceptance remain required.
  fix_hint: Freshly integrate current STATE/ROADMAP/TODO, preserve history and actual46 status, reconcile complete membership, renew full before/after/F0 proof and obtain ROOT whole-frame publication acceptance.
```

No technical revision is requested. Proceed to ROOT's fresh final publication preparation; complete publication and source execution remain held. All five global CVLD and whole167 gates remain OPEN; STOP168.



## Plan15 formal serialization ownership review | 2026-10-24T02:13:45.785331+00:00

The complete independent technical review below accepts only the five-target ownership correction: Task1 five files and whole15 nine owners. It preserves all old argv, Task2, other46 whole plans and the existing47/106/37 graph. Actual byte-test/CLI/ordinary/runtime/source fit remain unproved. ROOT final publication requires current metadata/checkpoint integration and exact full-frame inverse; the publication record is under `.local/open-bitcoin-dev/phase167/plan15/root-serialization-publication-v1/`. No source lease, acceptance increment or global closure follows.

# Plan15 formal serialization ownership review

**VERIFICATION PASSED for the concrete technical ownership refinement.** No technical blocker or warning was found. This verdict permits ROOT to prepare the final planning publication; it does not publish the amendment, accept source/data/runtime behavior, release execution, or close any global requirement.

Active lifecycle remains167-2026-10-21T16-38-23, yolo, commits:false. Only this new ignored review directory and its own disposable metadata scratch were written. No canonical/source/Summary/count/CHECK/Git write, product-runner import, Bun generator/test/CLI/helper invocation, Cargo/Bazel/database/native product action, source capture or other actor/session action occurred.

## Inputs and independent metadata evidence

Whole proposal index:122487 bytes, SHA256 `fdbebbec3efd58b0b33971cb823035a7c259876daeabbfc4fc4045f27708ea20`. Whole HANDOFF:8283 bytes, `00174b4d96ef4e0a7aa2c6d31b1aed995aacdd163674b78fbe488cf9411c9875`. All415 reference occurrences independently match length/hash; the package contains exactly396 listed artifacts and no unlisted artifact. The five technical target deltas were read in full, including whole candidate15 and companion.

Candidate15:20244 bytes, `642113f105f0714c02b440ca961b528f052623dd37b5176691d1dfd347c5e6f2`. Canonical baseline15 is18287 bytes, `ed2c197a56fcd923d992d12cf9cf040bdee0d32d4cf0e29d276fe0bee629b852`. New companion:5841 bytes, `6cf2b6e7cab6bf4c27b365475bc965a1482bb643572abf2a3ef8f288808f4883`. Minimal plan diff:7101 bytes, `01b2980fb168372aabcb86063b430c3b3689d1af4b599d5f54b7f0d574230371`.

The sealed v6 index `c79a6a9134bd19fb7f47dc6a702d4dec97c7f6d7ca346ef1033d205af6a65122`, complete independent9081-byte review `24b461e7082f9e46b23993a362db98bcdc70c57dfd691399eb237b2052af94c3`, its6045-byte index `9f4e9b8b24801d32cd159a1034f843e8565f3699e76282d9bbc8f185125880d5`, and ROOT1856-byte inert receipt `c57cf04e766645ef8942cceaf8b7907aeacd8b741531b57727bedf69feeff1e2` were read. Those receipts establish inert readiness only. Their original777-line four-owner HOLD, v5 object-equality finding, negative controls and full source/data identities remain historical evidence. The prior330-frame independent derivation was not rerun or promoted to product behavior.

The current47 planning publication `b14cc655355f3babdc5a977cc7f633aecb9a94adbf3ef9c2f1ed0e07458fb937` and planning acceptance `041560c7de405c0175b68f387c03c3477526f06e53735c0047599638b3d4251b` were read as metadata context. Source contribution count stays34; no46 or15 source acceptance increment follows.

AGENTS/sidecar/overrides, active lessons, architecture/code-shape/testing/verification/Rust and TypeScript guidance informed this review. The prior active lesson loading remains valid; no archive/audit/lesson mutation was introduced. The narrow helper follows the pure-core/shell split and Bun ownership convention. No new production dependency, workspace crate, Bitcoin Rust library, private RuleFacts export or canonical authority is introduced.

## Ownership, task conservation and schedule

Task1 explicitly owns these five files: corpus generator; `scripts/phase167-regtest-serialization.ts`; existing corpus test; history JSON; expectations JSON. Task2 retains its four node owners, giving nine unique Plan15 owners and two tasks. The helper owns pure serialization/mining/factual utilities and the shared byte assertion; source-pin/oracle/filesystem orchestration remains in the generator.

Inside Task1, only `<files>`, `<action>` and `<acceptance-criteria>` change. Replacing those three candidate fields with the original fields independently restores the entire original Task1 XML byte-for-byte. Name/read-first/behavior/verify/done are untouched. Whole Task2 XML is byte-exact. Frontmatter differs only by the exact added helper owner and new serialization_ownership_amended_at2026-10-24T02:00:46.385Z; original generated_at2026-10-21T16:55:59Z and all dependency/wave/requirement/lifecycle fields remain exact.

All47 plans retain every ordered automated command. All46 unrelated whole plan bodies are exact, including47 and35/45/46 R6 stage anchors. Plan15 remains wave31 with dependencies02/12/14; Plan16 remains downstream with its original ownership/control allocation. The full graph remains acyclic with every dependency earlier. Planning inventory remains47 plans/106 task blocks/37 waves. No plan/task/dependency/wave/count change is hidden in the ownership correction.

The other changes are a new canonical167-15-SERIALIZATION-REFINEMENT-AMENDMENT.md and append-only reciprocal links in15/16 allocation, Context and Decision Coverage. Every old prefix remains exact. The append supersedes only the old eight-owner15 union statement; seeded16 ownership and original evidence allocation remain. CHECK and all accepted Summaries remain whole-byte protected. No STATE/ROADMAP/TODO body is part of this candidate.

## Goal-backward checks

The phase goal remains observing pinned network/candidate-history decisions through ordinary header admission and durable body connection. This refinement makes the independently expected corpus and exact frozen bytes reviewable within explicit ownership; it supplies no acceptance proof by itself.

| Dimension | Result |
| --- | --- |
| Requirement coverage |15 retains CVLD-01/02/03/04 and its concrete monetary/contextual/script/difficulty tasks.06 remains in other unchanged plans. All five phase requirements stay globally OPEN. No relevant PROJECT obligation is reduced. |
| Task completeness | Independent GSD plan-structure: valid:true, no errors/warnings, original two tasks with files/action/verify/done. Explicit byte verdict, refusal matrix and future actual evidence make the amendment executable. |
| Dependencies | No changed edge/wave; actual14 acceptance and fresh accepted API/physical-datadir reconciliation still precede15 source. |
| Key links | Real CLI and tests must share the exact helper; frozen corpus still feeds real ordinary node admission/connect/reopen. No isolated calculator substitutes for consumers. |
| Scope | Task1 five files, Task2 four, nine total, two tasks. No hidden sixth Task1 owner. Historical inert fit is plausible but is not final source fit. |
| Derived verification | Both complete frozen files have byte-level acceptance; ordinary runtime truth still requires genuine owner/body/reopen evidence. |
| Context/scope reduction | D-02/03/04/05/07/08/09 remain fully required. Unseeded15 and seeded16 allocation is preserved; no deferred differential/network/wallet capability appears. |
| Nyquist | SKIPPED: workflow.nyquist_validation=false. No scaffold gate applies. |
| Cross-plan data | Complete raw deterministic bytes and source-bound facts preserve downstream corpus shape; same helper shares rendering semantics. No lossy transform or authority promotion is introduced. |
| AGENTS | Explicit owners, pure helper/thin orchestration, no dependencies, normal628-line source limit, actual behavior evidence and repo-native later verification gates remain. No source cap exception or packed JSON workaround. |
| Research | Existing phase questions remain resolved; the v5 byte-verdict issue is addressed by the concrete contract. No unresolved research choice is introduced. |

The complete required corpus remains canonical genesis plus1..301, separate next302, default halving150/maturity100, zero spendable genesis Coins, boundary rewards149/150/299/300 and recorded fees, maturity101 versus depth99-at100, unique coinbase txids, version/difficulty/MTP/BIP34/finality/BIP68/witness/sigops/missing-input negatives, complete source-bound UTXO/work/BASIC facts, and isolated≤306 known identities. Actual canonical runtime admission, scripts/Coins/mempool/BASIC, complete-handle checkpoint/drop/reopen at150/301 and connect302 remain Task2 obligations. Actual RPC/CLI evidence13/14 must reference the same physical datadir.

The height201 live distant creating-ancestor path and genuine branch-MTP disagreement remain15. Pure unseeded missing-envelope constructor controls must report actual precedence; a link/MTP refusal cannot masquerade as a retarget/recovery or transaction-kernel refusal. Missing creating-MTP0 and changed coin-MTP actual kernel controls stay16 with mechanical/noncanonical origin and result finalization. No constructor/durable-source negative replaces them. Labels, JSON data and helper facts mint no production authority.

## Byte contract and honest coverage

The copied v6 helper was read narrowly: `assertFrozenJsonBytes` compares `Buffer.from(frozen).equals(Buffer.from(JSON.stringify(expected,null,2)+"\n","utf8"))`. The actual proposed CLI derives expectations once, reads BOTH complete files through `.bytes()` and calls that same helper; the test imports the same module. This is byte equality of the project's chosen pretty rendering, not general JSON canonicalization or admission proof. No BOM removal, UTF-8 replacement normalization, sorting, selected-field/digest substitution or parsed-object verdict is allowed.

Actual future evidence must include both file kinds and compact/indentation changes, extra/missing final LF, BOM, nonUTF8 bytes and reordered keys. Canonical CLI `--check` must genuinely run through the generator's actual entrypoint on both files; passing pure-helper metadata controls or unit tests alone cannot stand in for that CLI invocation. The two old automated commands remain exact; this obligation is explicitly retained in the action/companion and requires attributable execution evidence later.

The existing source declares nine literal named tests plus seven named loop variants, totaling16 declared cases; this is a static inventory, not registered discovery. All16 prior requirements remain, with no exact16 ceiling. The three appended cases currently provide canonical positives, three history whitespace variants and history reordered keys. Prior BOM/nonUTF8/missing-LF and both-file helper invocations were metadata controls. They supply no registered execution, actual CLI or behavioral RED credit. Additional needed registered cases belong only in the already-owned test file; actual names/counts and final normal fit must be refreshed after additions. The plan explicitly requires filling this future evidence gap rather than claiming it already passed.

Historical generator615/helper245/test260 lines and largest named function150 apply only to the sealed inert copies. TypeScript/Rust retain628 physical lines and cohesion/refactor requirements; pretty JSON is outside the managed source-extension cap. No additional case may silently exceed fit, delete controls, pack lines or add an owner. Missing imports/files, compile failure or inert invocation cannot provide behavioral RED.

## Independent full metadata inverse

The candidate frame has127 complete existing phase-metadata bodies plus one TRUE absence for the new companion:128 states. The after has128 complete bodies. Five targets change;123 old bodies stay exact. Only planning Markdown copies were used; no product/source/native before authority was captured.

An independent scratch was populated under this review directory from authenticated frozen before copies. `/usr/bin/patch --batch -E -F0 -p1` forward/inverse/forward each exited0 with no offset/fuzz. Both forwards reproduced all128 after bodies and full membership; inverse restored all127 whole before bodies and the exact new-companion absence. No extra scratch file remained. Raw argv/streams/exits are retained in f0-1/2/3.json; metadata-check.json records all415 references, ownership/Task1 field restoration, Task2/all-plan conservation, DAG, prefixes and structure output.

An initial metadata-only inventory assertion counted only literal test names and stopped before scratch creation. The corrected static check accounts for the seven declared loop variants without importing/running tests; that failed reviewer assertion and correction are preserved. It is not a producer, application or registered-test failure.

## Publication and execution handoff

ROOT must whole-read this report, append it completely with truthful formal-review framing to CHECK only through a fresh separately prepared publication delta, preserve the entire old CHECK prefix and avoid new raw body frontmatter delimiters. Prepare the fresh ROOT progress/checkpoint language without increasing accepted contributions or changing47/106/37, reconcile current full metadata membership/bodies, and prove the complete final publication forward/inverse/forward before explicit ROOT planning acceptance/publication. The current five-target frozen proof does not include that later report/checkpoint delta. Do not overwrite intervening canonical or ROOT history from a frozen copy.

Actual15 source remains held until actual14 acceptance, fresh nine-owner before/TRUE-absence/API/physical-datadir/normal-fit reconciliation and separate ROOT source/build release. Source inverse, actual tests/CLI/RED/GREEN, real ordinary/runtime/native/reopen/adapter outcomes and independent source/evidence/SummaryQA remain later gates. Existing source-before17, compactC/D/E, security18, parity19, docs25 and full native/fresh coverage/Bazel/goal/lifecycle20 plus normal-hook MAIN Git obligations remain.

```yaml
issues: []
verdict: technical_ownership_refinement_verified
canonical_publication: pending_fresh_complete_ROOT_delta_and_acceptance
source_execution_released: false
actual_source_runtime_acceptance: false
```

All five global CVLD requirements and whole167 gates remain OPEN; STOP168.



## R7 complete independent technical and publication-precision review | 2026-10-24T03:57:45.890943+00:00

The complete independent report follows unchanged. It reviews only the inert source/planning refinement; all actual source, loader and renewed runtime gates remain separate.

# R7 typed owned-payload refinement — independent review

**Technical/source-inert and metadata precision review: PASS against precision revision-v2. No open refinement defect was found.** The original packet's two publication-wording issues are closed by the exact immutable precision revision. This is not canonical publication, source application, API dependency readiness, compilation/Clippy/runtime success, source acceptance or Summary/count/Git authority. Every execution/readiness/acceptance flag remains false.

**Execution blocker remains:** the reviewed actual46v3 loader hardcodes R6/preparation-v2 anchors. A separate checked loader/release revision is required for both actual46v4 and future45 before any actual resume. This review does not approve using the stale R6 loader for R7, editing it silently, or treating current recovery as original-before authority.

Only this assigned ignored review directory and its metadata scratch were written. No actual Rust/canonical file, native database, product Cargo/Bazel/Bun application, source/native/current product capture, Git operation or actor/app/session query was performed. Initial review used only sealed HOLD/proposal evidence; no mutable author preparation was accepted as authority. Final metadata proof waited for the author seals. The root-sent indexes and this report concern active lifecycle167-2026-10-21T16-38-23, commits:false; all five global CVLD and whole167/35 gates remain OPEN, STOP168.

## Whole evidence and authority boundaries

The initial actual46v3 HOLD handoff is2076 bytes, SHA256 `b75518479139b58af444252ffe741f0a8943b86d15254970273aaca90cd6c5a4`. Whole raw Clippy stderr17050 bytes is `2be71bcf1ebdc57e3870941fbc1259109f68fb2c041d261970b495f34d4b1a0b`. The raw findings are genuinely nonconsumer shapes: large_enum_variant at Present, result_large_err at owned denial and both serving results, boxed_local at the private response helper. They cannot receive the staged unused/dead-awaiting45 exception. The zero-exit diagnostic command itself did not close readiness.

Whole narrow index10950 bytes `4fd435c820ad3fb54530246d8abe9d132f970eaf1229cc9bd0f8763a38555e3d`, whole15336-byte review `ee0ce7dee15866473772723e4a8f991cc94382222abe8bdef6b6410729cbf9aa`, whole6646-byte contract `e15621bd91f0f38c7d3f203ab08bf394d52ebba49f4a549c678e7dd257a7f546`, actual3-file diff `82fe2c99a96097cdc71aa3aa1402b0beb83b53a3820052cb80f9fba73871ff8d` and future1-file diff `c5301a5282a4228f634cecd906e415706de65622722a82034325b64e4bdc53d4` were read completely, as were all four complete inert source proposals. ROOT's1968-byte whole-read/authentication receipt is `5af245bfb4a21a80b8e3af1703d16b0039db051f15f03eb41ee78a5fd9daa180`; it grants no publication or source lease.

The new sealed source/metadata packet index185299 bytes is `38583d718ed9db4cf0acedf2af8c96a383f57b5c003405f3b13f5c03c9e09625`; whole HANDOFF6776 bytes is `8cb1e095179b834067ca15b6182916f97491ae7aef519be5ccf2c3e87831eefd`; verification297634 bytes is `94c7b1542af5d5cfedd43f7bfa8d28c831e900d8b962eafe7d1e1d7ae4630481`. All498 reference occurrences, including all406 sealed owned artifacts, independently match complete length/SHA256. The packet remains immutable.

The later publication-wording precision index54557 bytes is `5d6d9e4c7ac3ca637eb8ad0c90fb05e9c7197b1217cd1c28c7e34dbd7a2ba70a`; whole HANDOFF2461 bytes is `59dbfcc02e1027156dc6f0e6ee3c80a83f63aaa82eb3403d190ff902fae0b9b7`; whole17810-byte evidence is `f6424c738b5c2bc21a64952316bcc166f96523a9bae7945595bc2ad67c55ca9d`. All131 reference occurrences, including44 precision artifacts, authenticate. It supersedes only the three metadata candidates, preserving all16 Rust hashes and every original406 artifact.

| Final precision candidate | Bytes | SHA256 |
| --- | ---: | --- |
|45 PLAN |47290|`5fa189ce1f4c334d4d5ccd84c5421ed00ae6b540fd1e8e950a61092d85a01195`|
|46 PLAN |46669|`976468bca87629c87b62b2d0f890b83f9e51847adb86afb9368fd635ad4b82eb`|
|45/46 companion |52095|`a62b868fdaa03d5a79d71a1b25a4081dba512ffd25391d75870c46f3e62eaa64`|

Local AGENTS/sidecar/overrides, active lessons, architecture/code-shape/testing/verification/Rust and GSD gates loaded during the prior reviews remain applicable. No new exception, dependency or authority surface is proposed. Full-byte authentication is distinguished from compilation or actual source acceptance.

## Exact16-source refinement

The original complete16 index remains17093 bytes/SHA256 `4c263ae8f8b1323dc5a8b0d9a46e66ca8a72e7640210eb51d86ffeff9ccba790`. The final path/partition/beforeExists map is identical: seven46 and nine45 owners; original8 whole bodies/8 true absences, with46's3-body/4-absence and45's5-body/4-absence partition. No source path, readonly definition, native causal, owner, cap or control is added.

Twelve source bodies are byte-exact original static16 bodies. Four are byte-exact the sealed narrow proposals and all raw-before-format copies:

| Existing declared owner below node src | Bytes | SHA256 | Physical lines |
| --- | ---: | --- | ---: |
|network/runtime_authority/peer_sessions.rs |17735|`f0e4489e3ed76c1c975c40bec4eb23ecf8af690bd2175d50acd2ddd2b295516b`|537|
|network/runtime_authority/filter_index/serving.rs |20037|`e3e0aaccae533321e1ecfd4e9da71302d2b6dad1fc7d09c5bb069a6286386202`|525|
|network/runtime_authority/peer_sessions/basic_tests.rs |20503|`349c17e7ccbaa2fd856995b4e8ae56c28b964ffe4641ff944286ac2353775cb9`|611|
|sync/block_reconcile/peer_requests.rs, FUTURE45 INERT ONLY |19067|`d88fc580bd1c921b6d5372163ccbda1cb4ce1580e47a95dddb673423c772af71`|546|

Independent whole-body inverse substitutions reproduce exact held inputs0a2ab414/5ab86493/**151def5b** and original futuref7215a92. This is stronger than merely matching changed lines: no unrelated byte in those held bodies is lost. In particular the BASIC candidate retains the current151 input's prior actual two-Vec and public-poison/raw-snapshot oracle corrections; it does not regress to the older static test body. This in-memory comparison does not perform an actual source inverse or substitute held source for de637 originals.

Current correction scope is exactly three already-owned46 files. The future45 move-out is separately inert and inside its existing nine owners. The other four actual46 owners and other eight future45 bodies remain exact. No source-compatible-as-is claim is accepted for old frozen45.

## Representation, errors, locks and physical frames

Present owns `peer: Box<PeerState>` and its producer wraps the same `peer.clone()` once under the same authority/registry observation. The requested vector, compact_pending fact, epoch/nonce/token checks and Present/Absent behavior are unchanged. Concrete owned indirection adds no new lookup, alias to mutable state, shared live token, public field access or guard-bearing capability. It is not boxed type erasure; no Box<dyn ...>, new public type, visibility expansion, Any/callback escape or dependency is introduced.

ContinuationDenied and private SyncContinuationFailure::Denied store concrete boxed denial and optional-primary errors. The serving preparation closure boxes the genuine error at the same map_err sites. The existing continuation match moves those same boxes into the owned denial. Only ContinuationDenied::into_parts moves out/unboxes both errors, preserving exact values and optional-primary precedence. Its outer returned tuple remains `(SyncPeerObservation, ManagedNetworkAuthorityError, Option<ManagedNetworkAuthorityError>)`; SyncPeerDispatchOutcome::into_parts remains `Result<SyncPeerAuthorized, ContinuationDenied>`. All future consumer error joining therefore keeps its original types and order.

Operation remains the original unboxed actual operation error. The actual raw result and earned observation survive; Authorized inner operation error and Denied-no-cleanup remain distinct. No imagined Some-primary producer/test is added to claim execution. The correction does not move any lock, change higher-ranked request callbacks, weaken original-bound borrowed continuation/resolve, alter admission routes/Weak-nonce-Arc authority, turn absence into capture, or add a third request-candidate set.

The unchanged PendingDisconnect/UnlockedDisconnect body was read. Second actual authorization precedes raw disconnect; prune/wake collection occurs under the existing guard; release drops that guard; only guard-free completion wakes and returns the original semantic/prune-joined Operation. No boxed denial change reaches that path. All unchanged release/wake/error assertions remain controls, not newly executed evidence.

The private BASIC completed_frames helper now consumes PreparedFilterResponse by value. Both genuine callers move the same value out of their existing response boxes. The response's owned affine lease, next_frame iteration, actual sink writes, acknowledge_write receipt and complete_basic_filter_frame calls are exact. Releasing the outer allocation sooner does not return the response/lease early or manufacture a receipt. Existing loopback physical-frame/charge/poison assertions, public/generic comparison, counters and cleanup remain; production response representation is untouched. No runtime proof is inferred from that source inspection.

## Mandatory future consumer and copied caller evidence

The only future type adjustment is `self.maybe_peer = Some(peer);`→`self.maybe_peer = Some(*peer);` in PeerRequests::reconcile. Independent restoration yields the complete oldf7215a92 body exactly. The move remains after both original fallible previous/current cleanup results and after `self.retained = requests`; capability and compact_pending updates, absence handling, denial retention and two-set finish are unchanged. It moves the identical snapshot once into existing Option<PeerState>, without clone/requery, changed cleanup order, narrower retained set or new source owner.

All current affected observation matches consume requests/absence or use `..`; they do not require another bare-PeerState edit. The only private Denied producer remains the serving closure; the unchanged continuation transports it, and consumer unboxing occurs only in into_parts. The full changed source bodies and future consumer were read, and affected occurrences in the147 whole-file frozen caller overlay were inspected. The overlay's16 replacements match the final bundle; other caller copies remain frozen observations, not a fresh workspace/source-authority scan.

The11 protected whole boundary snippets authenticate; exact narrow whole-body reversibility plus12 unchanged bodies preserves their origin/resolve/immediate-consumer/absence/denial-finish/outbox/private-guard/disconnect boundaries. All35 old source-control names are present and remain explicitly UNRUN. The original6709-byte protocol source prefix is exact. No test name, old case, retention proof, meaningful physical frame, error category or assertion was removed to satisfy shape limits.

## Copied formatting, static fit and bookkeeping disclosure

The packet records pinned `rustup run 1.94.1 rustfmt --edition 2024 --config skip_children=true --check` on all16 owned copies, exit0, zero byte deltas. All final bodies equal raw-before-format and sealed checker sources. Maximum physical length619 was independently recomputed; the authenticated327-function lexical inventory has maximum145. Revised lengths537/525/611/546 meet the existing628/new-test620 caps without packing, blank deletion, suppression or control removal.

This proves copied formatting/static normal shape only. This review did not run a formatter or compiler. Rust type/privacy/borrow compatibility, actual Clippy diagnostic removal, changed-source tests/runtime, source/native conservation and actual fit after application remain UNPROVED. The old five-warning HOLD is not cleared by an expected layout reduction or successful Rustfmt parsing.

The precision author discloses a create-only README bookkeeping guard interruption after its proof/evidence generation. Inspection of prepare.ts shows evidence saved after both F0 ledgers, then the attempted existing-README checklist write. Its separate seal authenticates those already-completed outcomes and completes documentation/index; it does not rerun source/proof/product work. All six exact raw metadata completion receipts are present and pass; the disclosure is not represented as a successful generator run or product failure/RED. Independent full128 metadata proof below confirms the final delta separately. No source/runtime pass credit follows from this bounded bookkeeping incident.

## Metadata precision and independent full128 F0

The original sealed candidates' two wording concerns were valid: “metadata candidate only” would survive publication, and unframed “by this reviewer”/unformatted-fit prose obscured later copied-format evidence. Precision-v2 makes exactly three substitutions per appended clause: suitable planning-only/separate-source-release status; explicit historical narrow reviewer versus later copied Rustfmt evidence with actual gates unproved; and normative “planning refinement” no-release wording. All16 source hashes, amended_at, pre-clause bytes, old Task XML/argv/fences and authority geometry remain exact. These two publication-precision findings are closed against the new candidate hashes; the earlier packet stays historical.

For all three final metadata bodies, removing only the single amended_at2026-10-24T03:37:45.409Z and appended typed-owned-payload clause restores the entire R6 body byte-for-byte. No old task/action/verify/done, command fence, requirement, owner, dependency, wave, count or existing historical paragraph is rewritten. No standalone body--- is added. Both independent GSD plan-structure checks pass with three tasks/no errors/no warnings. Existing3-file scoped Markdown/GFM/frontmatter check receipts are authenticated; no global metadata normalization was performed.

The author retains whole-file copied8-body/8-absence source F0 and initial3-metadata F0 ledgers. The wording revision separately retains six raw F0 completions for v2↔v1 and v2↔original R6. All are inert proof receipts, not de637/actual46 inverse or runtime TDD.

This reviewer independently used the unchanged current128-file phase metadata base and final three candidates. Own scratch BSD patch --batch -E -F0 -p1 forward/inverse/forward each exited0 without offset/fuzz. Every step matched all128 whole bodies and complete membership: forward changed only45/46/companion; inverse restored all originals; second forward reproduced final candidate bytes. All125 protected phase bodies, including every other plan/Summary/Context/Coverage/CHECK, remained exact. Raw argv/streams/exits and full before references are in verification.json and metadata-F0-1/2/3.json. No canonical body was patched. Current47plans/106tasks/37waves and34 accepted contributions remain unchanged.

## Original authority, loader blocker and future execution

Preserve these distinct authorities: original complete16 static8-body/8-absence geometry; original actual46 de6373-body/4-absence direct-before; expanded498 containing all original271 states plus ONLY the two separately authorized readonly definitions; historical initial9a3 successful for271 only; held actual final52129640; and latest current recovery6273b59f1bdde02f5d779baccab9bf8bab22448828d46bde70623b9e1cb8c92d. Recovery6273 is a current future-input candidate, never an application beforeimage, fake initial success or replacement inverse authority.

The three-file correction window must preserve all270 other273 source states and original201 native paths plus separate Byteview1. Final46 still has seven owners/266 protected states; future actual45 needs its own fresh5-body/4-absence original capture and both original behavioral RED commands before edits. No new writable owner, readonly causal or native cause is required or authorized by the proposed boxes/move-out. Static copied before bodies do not replace actual45 before authority.

The actual46v3 load-authority.ts was read. It selects execution46-preparation-v2/anchors.json with hardcoded4075-byte a07649e1 anchor manifest, checks R6 plan45/46/amendment/static16/review/root-acceptance hashes and geometry, and therefore cannot authenticate this new R7 payload as-is. Preserve it as historical. Before actual46v4 OR future45 resume, prepare and independently review a separate exact loader/validator/release revision binding accepted R7 canonical precision and corrected16 while retaining original/current frame distinctions, geometry, minimum271+readonly2, native201+separate1, raw child completion, environment and source-window constraints. No silent hash override, old R6 receipt relabeling or permissive fallback is approved here.

Before actual correction: ROOT must whole-read/publish the final checked planning delta through fresh metadata/root checkpoint/review/inverse preparation; bind current273 recovery and protections; finish the separate loader/release review; then issue an explicit three-owner source lease with no application queued/running. Actual source remains unchanged by this review. Both Tasks and ALL ten preconsumer commands must run freshly on ONE corrected frozen causal/native frame, preserving all raw outcomes/nonzero discovery, exact selectors, environment/global guard/serial discipline and unsuppressed warning review. Old v3's255 overlapping passes/180 unique names and12 commands are historical only, not changed-source credit.

Only renewed complete exact API/every-warning/source/native/original-seven-owner inverse/independent CORE/ROOT review may support a versioned NOT-FINAL46 dependency receipt. Actual45, genuine original REDs, joint strict-D/fresh18 gates and46 then45 SummaryQA remain later separately released work. No readiness, strict pass, final/source acceptance, Summary, count, commit, actual35 source or C/D/E publication is authorized. Whole167 DEEP source/security/native/parity/docs/coverage/Bazel/goal/lifecycle/normal-hook MAIN gates remain; STOP168.

```yaml
issues: []
refinement_review: technical_and_precision_pass
closed_publication_precision:
  - historical_formatter_attribution
  - planning_only_normative_status
remaining_execution_blockers:
  - separately_checked_R7_anchor_loader_and_release_revision
  - fresh_three_owner_source_lease_and_corrected_full_verification
sourceApplicationPermission: false
runtimeReady: false
apiDependencyReady: false
sourceAccepted: false
strictClippyPassed: false
finalAccepted: false
SummaryReleased: false
sourceCountIncrementAllowed: false
commitAllowed: false
canonicalPublicationApproved: false
allFiveGlobalCVLDOpen: true
phase168Held: true
```



## public-chain-information-split-candidate | 2026-10-24T06:15:23.208247+00:00

Planning-only split proposed for independent/Root review:13 core/registry/public facts11owners/2tasks, new48 original daemon/process5owners/1task after13 and before14; one shared history-test owner is sequential. Full original Task2 XML/four argv move byte-exact; originalTask1/prior amendedTask1b XML and all8 prior13 argv remain exact across13+48.15-owner cross-plan union is unchanged. Proposed inventory48plans106tasks38waves versus current47/106/37; existing pending14–20+25 waves shift+1 and14 keeps13 plus adds48. Public factual interface/corrected witness76–103/directKnots1791 evidence and full no-scope-reduction/global gates are in167-13-PUBLIC-CHAIN-INFORMATION-AMENDMENT.md. Eleven-fileTask1b remains a disclosed scope red flag requiring separate independent/Root adjudication and demonstrated normal fit; no source/acceptance/lease/phase168 claim. Future unpublished compact CDE IDs48/49/50 must become49/50/51 before future publication, outside this proposal. All current accepted source/research/Summary pairs and original35/45/46 bodies stay exact; all five globalCVLD remainOPEN. No actual35/47 API acceptance assumed. Whole167 source-before17/security18/parity19/docs25/ENTIREnative20/freshpureCorecoverage/Bazel/lifecycle/normal-hookMAINpush/STOP168 remain mandatory.

## Root planning allocation and exact Markdown preservation

The checked13/48 planning allocation accepts the disclosed eleven-file coupled wiring warning in principle only. Executable13 has11 owners/two tasks,48 has5 owners/one relocated original task, and all8 original argv remain exact. This resolves the former15-owner plan blocker through task relocation; it grants no source, actual runtime, Summary, accepted-contribution count or Git authority. Earlier candidate-only/not-published descriptions record preparation history. Actual35+47 acceptance and fresh complete source/API/normal-fit/private-boundary/original-inverse/native review and separate Root leases remain mandatory. Planning inventory is48plans106tasks38waves; accepted contributions34, milestone104/11888%, verified3/8 and all five globalCVLD remain OPEN.

Scoped mdformat1.0.0 with GFM1.0.0/frontmatter2.1.2 checks found14 pre-existing formatting failures among15 existing changed documents. Those frozen histories are preserved. The new companion and changed Roadmap receive scoped formatting. New48 prose/tables/frontmatter are formatted, while the relocated original5041-byte Task2 XML remains exactly6999184555bca2f52d1260bd23c1c4e244561da96c6f49af32e2cf82a3a8a137: the formatter would remove its required preserved indentation. This is a narrow documented verbatim-XML exception; formatter output with that one exact task restored is a fixed point outside the frozen task. No arbitrary history cleanup, changed argv, reduced control or source-format exception follows. Root publication receipt and independent final precision review remain separately attributable.

## R8 copied correction candidate — independent and Root review pending

This entry records preparation only. It is not an independent review verdict, canonical publication or actual source acceptance. Required inputs were AGENTS.md, AGENTS.bright-builds.md, standards-overrides.md, architecture/code-shape/testing/verification/Rust standards and the complete active9838-byte lesson set. Root's existing sync/checkpoint provenance is inherited; this worker performs no Git operation.

The copied candidate has12 whole45 owners: original9 plus network/types.rs, network/action_translation.rs and sync/runtime_state.rs. All seven actual46 bodies remain protected, yielding19 combined explicit owners. New Task1b owns exactly the three additions. Original Task1/Task2/Task3 XML remains byte-exact; all33 original ordered commands remain exact, with one separately declared bounded Task1b command added. Proposed task arithmetic is48/107/38, with no plan/dependency/wave or accepted-contribution change. Twelve owners are an explicit scope warning requiring independent and Root review, not automatic approval.

Source changes carry the actual completed Block only after successful connect, classify all three body-response identities through existing fresh scoped membership/handoff facts, preserve old Block-only ignored/error behavior and Block/NotFound receipt cleanup, route actual bodies through existing persistence/BASIC/error/order, admit fixture101/102 through normal real-peer Headers ingress, and remove only proved unused imports/re-exports/method. The old6709-byte parent prefix and every existing assertion/control are retained; no original test is deleted or made ignored. Default100-block maturity and all old compact/shared-peer/finish cases remain. The new pure identity control proves response identity does not widen receipt cleanup; its execution is still UNRUN.

Copied-only Rust1.94.1 rustfmt with skip_children=true and a separate check succeeded; every copied body is normally formatted and below628 lines, with test files below620. These are syntax/shape observations, not compilation, borrow/privacy, actual discovery, runtime, strict diagnostic or source acceptance proof. Root causal/probe review remains required before selecting the correction.

Original73dc remains original nine-owner inverse authority; promoted action_translation comes from its old readonly member. Added types/runtime are explicitly current-only64bef inputs. Fresh current recovery and mixed-provenance eight-body/four-absence inverse are distinct. Prospective source284/protect272/native201+separate1 requires new full capture after probe restoration and checked versioned anchors/loader/helpers/matrix/lease. Both originalREDs, failed19-case GREEN and diagnostic probes stay immutable. Fresh added Task1b, published GREEN, ONE joint frame/all18 strict-zero finals, full scratch original inverse/forward, Root sealing and independent actual evidence review precede46 then45 Summaries. Failed35/whole167 gates and STOP168 remain held.


## Root checked R8 source-scope publication | 2026-10-24T10:55:55.853070+00:00

The selected unchanged source-v1 index93ee3fd6 plus metadata-v2 index23716a88 is checked after independent3839a808/wholeed2abdf3 and Root899e0527 review. The preceding copied-candidate/pending descriptions retain their preparation-time meaning; this current decision publishes their exact operative R8 scope only. The missing durability derivation warning is resolved by one requested-body truth, three artifacts and four actual pipeline links. Whole45 has12 owners/four tasks, combined19 with all seven actual46 bodies unchanged. Original three task XML blocks and33 ordered argv remain exact; Task1b adds one bounded compact-handoff row. Planning is48plans107tasks38waves; source acceptance stays34 and milestone104/11888%,3/8verified/allfiveglobalCVLD remainOPEN.

Root accepted the bounded scope warning and ordered process_actions plus3-statement rationale for this correction only, with every copied body at most619 lines and dedicated tests below620. This is no actual compilation, runtime, strict-warning or source-acceptance proof. Genuine ed54 probe confirmed acceptedtip101 before missingbody and complete original restoration; Rootd58c authenticates the result and current486states. Original per-path compositec664 is284 inputs/direct12/eightwhole4originalabs/protect272, retaining old73dc and later current-only64bef origins honestly, native201 plus separate1. It supplies no fresh recovery or executed inverse.

Fresh complete current284/native201/separate1 capture, independently reviewed forward-only helpers/anchors/loader/matrix, genuine new correction-only lease, exactnine changed buffers/scopedformatter, Task1b and fresh publishedGREEN, ONEnewjointframe/all18freshstrict-zero finals, original12-path scratchinverse/forward/CORE/independent actual review still precede46 then45 SummaryQA/final counts. All oldREDs/failedGREEN/diagnostics retain original verdicts. Failed35 and whole167 source-before17/security18/parity19/docs25/ENTIREnative20/freshcorecoverage/Bazel/lifecycle/normal-hookMAINpush remain mandatory;STOP168. No source application, formatter, product command, Summary, acceptance increment or Git action is granted by this publication.


## Canonical C/D/E planning publication | 2026-10-27T17:56:10.370868+00:00

Plans49/50/51 and the [compact lifecycle companion](167-49-51-COMPACT-LIFECYCLE-AMENDMENT.md) are canonical planning metadata after actual accepted Plan35 source receipt .local/open-bitcoin-dev/phase167/plan35/root-task3-correction-v1/r2-final-source-acceptance.json. The receipt carries accepted45/46 prerequisites. The new dependency chain is 16→49→50→51→17, with 17 retaining 16/34 and adding only51; waves17/18/19/25/20 are 37/38/39/40/41. This publication adds three plans and twelve planned tasks to the live 48-plan/107-task/38-wave frame, yielding 51 plans/119 tasks/41 waves. All existing Task XML and ordered automated commands remain exact. The full before/after/inverse and true-absence proof is under .local/open-bitcoin-dev/phase167/.compact-lifecycle-design/canonical-cde-publication-v1. Source execution remains HELD; all61 new C/D/E task/final commands are UNRUN. Actual source/API/normal-fit/fault reachability, complete source/native frame, original inverse, and separate release remain unproved. Independent whole167 source review precedes17;18 security/native/mutants, 19 parity/breadcrumb/docs,25 goal/writing-for-agents and20 full default native/fresh coverage/Bazel/lifecycle/normal-hook MAIN gates remain. All five global CVLD requirements remain OPEN; STOP168.
