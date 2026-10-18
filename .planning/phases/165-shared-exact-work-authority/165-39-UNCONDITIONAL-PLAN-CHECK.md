# Phase165 Plan39 unconditional-refusal amendment check

## Verdict

**PASS for the bounded amendment, with one nonblocking scope warning.** No implementation acceptance, inverse derivation, current pin activation, test execution, full-native acceptance or phase completion is asserted by this review.

Checked plan SHA256: `17d4abedf0d4ecb8e11cc80627949bc9e17510d6f5c59f9732ba0640eee7e942`.

Checked ledger SHA256: `1f98653f53b88cd7bdc97fb0b828332db2a00cf3147f64e98bc64756a74ef8a7`.

The authorization is the Root coordinator's bounded amendment within the active Phase165 GSD workflow. Review ownership is this report only. Root must renew the executor's release after accepting this report.

## Preservation and structural evidence

Independently authenticated the complete before-amendment capture at `.local/open-bitcoin-dev/phase165-plan39-unconditional-amendment-before.json`, SHA256 `fe63f3e082a5a76d8146083fae0bf87bc649364a3604afa65e6ba680555b1061`. Its embedded original plan and ledger bodies reproduce their recorded SHA256 values. All38 other plan files reproduce their captured whole-file identities. The complete39-plan dependency graph is acyclic, every dependency exists and precedes its dependent wave, Plan39 remains wave38 depending on38, and final14 remains wave39 depending on39.

Final14's complete task XML identities remain:

- Task1: `f73b7127d8992cbd9b10bf362c48f7682c021a355d3febd11b06333765e5b9a4`.
- Task2: `a4706372cbd14638ae042a6675ab637a8b6679709e708b9b9b7ce096d6e69b5e`.

Actual `gsd-tools verify plan-structure` returned valid, three complete tasks, no structural errors and no structural warnings. Actual lifecycle verification returned `valid` for lifecycle165-2026-10-14T16-09-59Z. The amendment changes neither verification command blocks nor dependencies. Its eleven existing owners are explicitly partitioned4/5/2 among the three tasks.

## Why the added owner is necessary and sufficient

Read actual159 main, its contract consumer, existing shared Rust evidence helpers and the ordinary Rust work-authority implementation. The contract loop uses `hasOrderedCode`; ordered inclusion of the refusal expression alone does not establish that the refusal is unconditional. The actual ordinary `prepare_reorg` body is exactly the expression `Err(ChainstateError::PreferenceAuthorizationRequired)` after ordinary-source masking and whitespace compaction.

The amendment requires a separate exact-body comparison in existing159 `checkSource`, reusing existing `maybeBoundary` and imported `compact`. `maybeBoundary` already selects a unique ordinary function and accommodates the semicolon inside an array signature. Existing production ownership validation remains required. Consequently a conditional expression containing the same error token, a success path, or an effect before the error cannot satisfy whole-body equality. No general parser, new helper module, Rust change or new dependency is necessary.159 main remains byte-exact at review time:317lines, SHA256 `563ec1eb3694e9c3961f705f250596ea68292fc67158f8c85375f07f08184a32`.

The current159 test is621lines, SHA256 `321e97583780d4fb3ad1ec760fb67b2e1127b6561ac19b23067dc37a28e0eb70`. Comparing it with the authenticated344-file baseline shows only one added161 import and the appended thirteen-case table/harness. All original case definitions remain intact; the separately authorized no-op repair is still pending. Three additional data rows fit its remaining seven physical lines without packing or deleting assertions. All eleven current owners are below628lines; additions remain subject to actual final readable-cap measurement and a mandatory bounded replan if capacity is insufficient.

The retained semantic RED receipt binds the actual pinned-Bun selector, exit1, log SHA256 `0ba90ba98b14459fd85483405694eaaf43d58078ce80f75c9a8047dcc401ba36`, and generation SHA256 `4fbff81844773a60e81d8405930588521e05dc137c157007e425620ad2761083`. The log reports0pass/13fail,637filtered and27assertions; generation contains5227raw-state rows. These are prior execution evidence, not tests run by this reviewer. The amendment separately requires demonstrating token-only acceptance of the conditional mutant before repair and exact refusal afterward. Setup failures retain their separate status.

## Historical and cross-plan contracts

Actual Phase163 `PROVENANCE_DIGEST_FILES` includes159 main as well as159 test and161 main/test.159 main is not manifest-bound. Changing it therefore requires the third exact inverse; omitting it would change historical attribution even if all manifest-bound snapshots passed.

Task2 requires independently accepted actual four-owner Task1 source before deriving all three current-to-before inverse values.159 main restores to563ec,159 test to the existing a476 target and161 checker to the existing c344 target.159 and161 checker normalization is present-only, accepts explicitly authenticated old data, rejects unknown present data and never reads absent source. The manifest-bound159 test still requires its source and snapshot. This preserves filtered159 and partial historical projections rather than imposing current runtime qualification on their narrower data-only contract.

The existing strict inverse helper checks actual whole current SHA, exact descriptor shape, nonempty and non-no-op spans, unique matches, increasing nonoverlapping locations and complete restored SHA. The amendment does not relax it. H's separate inverse remains external to H and follows independent H source acceptance; contracts does not import H/E/main, so the added import does not create a cycle or self-hash dependency.

Full164/165 gates must bind current PASSED H/E and all three changed guards when present before imported current classifiers count. Actual full maps contain H/E, not raw old164 main/contracts; no nonexistent input qualification is promised. E remains exact, its cec-to-b92 proof remains one file with three hunks, and the existing Rust-to-2d0 and final FAA historical targets remain immutable. No manifest, snapshot, receipt, capture, fixed-read expansion beyond the already planned work-authority source, or historical destination renewal is authorized by the amendment.

Task3 still requires independent complete source acceptance before ordinary source rows and ordered input pin derivation, inert proposal acceptance before activation, actual old-pin refusal when membership changes, fresh copied164/165 controls and unchanged complete raw read closure. The added owner does not bypass a review or postpone an implementation obligation to a later phase.

## Dimension results

| Dimension                 | Bounded amendment result                                                                                                                     |
| ------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------- |
| Requirement coverage      | CWOR-02/03/05 preserved; exact refusal directly reinforces D-07. CWOR-01 remains covered by unchanged phase plans.                           |
| Task completeness         | Three concrete tasks retain files, actions, runnable verification and measurable done criteria.                                              |
| Dependency correctness    | All39 plans form a valid acyclic graph; final14 and38 are unchanged.                                                                         |
| Key links                 | Exact refusal consumer, third historical inverse, current generation qualification and current pin integration are explicitly wired.         |
| Scope sanity              | Warning: eleven existing owners. Explicit4/5/2 tasks and source checkpoints bound execution; no single task has ten owners.                  |
| Verification derivation   | Conditional/success/pre-effect counterexamples distinguish the required invariant; original637/149 final regressions remain mandatory.       |
| Context compliance        | D-04/07/09/10/11/12 scope retained; no canonical-bootstrap, contextual-consensus or actual-daemon acceptance claim added.                    |
| Scope reduction           | None: unconditional refusal is strengthened rather than reduced to ordered token presence.                                                   |
| Nyquist                   | SKIPPED: workflow.nyquist_validation is false.                                                                                               |
| Cross-plan data contracts | Third present-only inverse preserves actual historical provenance inputs; current gates remain separate from partial data projections.       |
| AGENTS compliance         | Pinned Bun, readable628-line bounds, source evidence, immutable attribution, deferred commits and final native verification remain required. |
| Research resolution       | Relevant research questions are explicitly RESOLVED; amendment introduces no unresolved research decision.                                   |

The five existing threat controls cover the added boundary: T-165-39-01 now requires exact unconditional-body semantics;02 covers the third strict inverse;03 covers current PASSED guard/adapter generations;04 covers absent nonmanifest159 as well as161 without fallback;05 preserves source-before-pin acceptance and evidence closure. No new accepted risk or security waiver is introduced.

## Structured issues

```yaml
issues:
  - plan: "165-39"
    dimension: "scope_sanity"
    severity: "warning"
    description: "The bounded amendment expands Plan39 from ten to eleven existing source/test owners across three tasks of four, five and two owners."
    fix_hint: "Retain the explicit task/source checkpoints and actual readable628-line measurements. Stop for a bounded Root replan before acquiring any further owner, module or capacity exception. No split is required for this narrowly defined additional exact-body check and its necessary historical inverse."
```

## Review limits and guidance

This is static verification of the amendment, not a fresh full review of implementation or a replacement for prior phase-plan checks. The application and tests were not run; only static metadata/structure/lifecycle checks and prior receipt authentication were performed. Further actual source review, semantic RED/GREEN, historical controls, final current rows/pins and final14's entire native/coverage/Bazel/code/security/formal/normal-hook gates remain required.

Material guidance: local AGENTS.md, AGENTS.bright-builds.md, placeholder-only standards overrides, architecture, code-shape, verification, testing and TypeScript standards; GSD revision-gate references; complete active lessons7188bytes/2397estimated tokens. No project skill directories were present and no lesson archives were loaded.
