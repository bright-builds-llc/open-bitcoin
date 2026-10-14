---
phase: 164-canonical-chain-identity-and-compatibility-gates
plan: "08"
subsystem: verification
tags: [bun, canonical-identity, immutable-provenance, mutation-tests]
requires:
  - phase: 164-01-through-07-and-09-through-13
    provides: Actual canonical identity/refusal wiring and explicitly classified fixture callers
provides:
  - Offline fixed-input identity, opener, admission, fixture and claim boundary checker
  - Default native registration and adversarial current-source mutation evidence
  - Versioned original tooling snapshots with distinct current and historical attribution
affects: [165, 166, 167, 168]
tech-stack:
  added: []
  patterns: [Exact-byte source contracts, bounded read-only shell, strict historical tooling view]
key-files:
  created:
    - packages/open-bitcoin-node/src/storage/chain_compatibility/tests/preservation.rs
    - scripts/check-phase164-canonical-identity.ts
    - scripts/check-phase164-canonical-identity.test.ts
    - scripts/check-phase164-canonical-identity/contracts.ts
    - scripts/check-phase164-canonical-identity/regions.ts
    - scripts/check-phase164-canonical-identity/claims.ts
    - scripts/check-phase164-canonical-identity/fixture-source.ts
    - scripts/check-phase164-canonical-identity/historical-tooling.ts
    - scripts/check-phase164-canonical-identity/historical.test.ts
    - packages/testdata/phase164/historical-tooling/manifest.json
  modified:
    - packages/open-bitcoin-consensus/src/chain_spec/tests.rs
    - packages/open-bitcoin-rpc/src/context/canonical_chain/tests.rs
    - packages/open-bitcoin-node/src/storage/chain_compatibility.rs
    - scripts/panic-sites.allowlist
    - docs/parity/source-breadcrumbs.json
    - scripts/check-phase159-filter-rpcs.test.ts
    - scripts/check-phase160-peer-filter-serving.test.ts
    - scripts/check-phase161-operator-index-evidence.test.ts
    - scripts/check-phase97-inbound-metrics.ts
    - scripts/check-phase97-inbound-metrics.test.ts
    - scripts/check-phase107-runtime-relay-activation-download-eligibility/checks.ts
    - scripts/check-phase107-runtime-relay-activation-download-eligibility/constants.ts
    - scripts/check-phase107-runtime-relay-activation-download-eligibility.test.ts
    - scripts/check-phase127-authoritative-network-state-unification.ts
    - scripts/check-phase135-snapshot-recovery.ts
    - scripts/check-phase135-snapshot-recovery/source.ts
    - scripts/check-phase135-snapshot-recovery.test.ts
    - scripts/check-phase157-index-catch-up.ts
    - scripts/check-phase157-index-catch-up/contracts.ts
    - scripts/check-phase157-index-catch-up.test.ts
    - scripts/check-phase75-soak-runner.ts
    - scripts/check-phase75-soak-runner.test.ts
    - scripts/verify.sh
    - scripts/check-phase123-runtime-timing-evidence-integrity/checks.ts
    - scripts/check-phase123-runtime-timing-evidence-integrity/constants.ts
    - scripts/check-phase123-runtime-timing-evidence-integrity.test.ts
    - scripts/check-phase158-validated-reorg.ts
    - scripts/check-phase158-validated-reorg.test.ts
    - scripts/check-phase162-integrated-filter-proof.ts
    - scripts/check-phase162-integrated-filter-proof/contracts.ts
    - scripts/check-phase162-integrated-filter-proof.test.ts
    - scripts/check-phase163-provenance.ts
    - scripts/check-phase163-provenance/fixtures.ts
    - scripts/check-phase163-provenance/native.test.ts
key-decisions:
  - Historical receipt qualification uses exact original source bytes; current executable guards inspect raw current inputs.
  - Unknown current tooling, original snapshots, manifest or adapter bytes refuse until explicitly reviewed.
  - Exact-byte region fingerprints preserve literal whitespace and conservatively require renewal after formatting changes.
requirements-completed: []
requirements-addressed: [CBOT-01, CBOT-04, CVLD-05]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 164-2026-10-14T01-59-33Z
generated_at: 2026-10-14T06:24:20Z
duration: approximately 1h
completed: 2026-10-14
---

# Phase 164 Plan 08: Current Identity and Historical Evidence Guards Summary

**A bounded offline checker protects 93 fixed Rust inputs and 246 reviewed function edges while nine exact original tooling snapshots preserve historical receipt authority without qualifying current ordinary operation.**

## Completed Work

Both tasks completed. `checkPhase164(ReadonlyMap<string,string>)` reports typed findings; its thin Bun CLI reads only the explicit local corpus, rejects nonregular/missing inputs, caps each read at 512,000 bytes and performs no network or datadir operation. Mixed product/test files are classified through executable function regions, reviewed ownership and exact test-module registrations. New raw opens, function aliases, fixture-like product names, disabled guards, late effects, swallowed refusal, supplied-owner bypasses, changed identity/support facts and overstated current claims refuse. Daemon, RPC context, CLI wallet/control/soak paths and lower-level fixture construction retain their separate meaning.

The default verifier runs the new mutation suite and checker after existing provenance gates. Both verifier command representations and the existing exact ordered historical contracts were extended without dropping or disabling a stage. No runtime dependency or Rust source was changed by this plan.

## Historical Attribution and Ownership Extension

Initial Phase 162 failures concerned the explicit fixture constructor/import substitutions and their source-bound historical digest. Adding the planned verifier/tooling changes also exposed the shared Phase 163 digest's tooling inputs. Parent explicitly authorized the minimal central historical-view design and affected fixture-factory repairs.

`fixture-source.ts` reverses exactly one approved import substitution and one constructor rename for the single daemon Phase 162 fixture, and accepts the result only when its original SHA is `faa009380301d6e6acbf032fd814f5cf9e31ddcbdb629787477fe5fbc5d62615`. Raw current SHA remains `933da5cfb57612c9e93a231967a17da3cc0163e1badcd5f7d44debde78868b18`. Unknown edits, including whitespace outside the substitutions, cannot earn that mapping.

The version-1 manifest and nine nonexecuting `.txt` snapshots under `packages/testdata/phase164/historical-tooling/` preserve these original files: Phase 158 checker/tests; Phase 162 checker/tests/contracts; Phase 163 provenance helper/native tests/native fixture copier; and `verify.sh`. Every snapshot was byte-compared against fixed original commit `6cf34f1b96cdb7e719752395b2c52c8f531b559c`. Runtime checks use checked-in data and exact SHA validation, requiring no Git history or network. A supplied corpus cannot borrow absent snapshots from the real checkout.

Only the historical claim reader in Phase 158 and historical digest seam in Phase 163 use the view. Current structural/source checks remain raw. Phase 162's reviewed current helper fingerprints retain all checkpoint, lineage, IfNeeded and no-snapshot controls; original helper bytes remain independently bound by the unchanged historical receipts. The Phase 163 native copier permits only the exact manifest JSON import already present in fixed sources, preserving its path, import-tree and evidence bounds. Phase 158/162/native synthetic factories use the historical view only for rebuilt proof rows/digests; original negative assertions and raw filesystem/source-preservation comparisons remain intact. Phase 123 separately checks the current ordinary three-argument opener, historical explicit fixture six-argument activation and the actual split context constructor.

Actual full provenance attribution is `historical/source-derived-regression`: raw current digest `d6e2abc2024d8cfeef440e0b7b352b39a2775535bd6f1dab2282dcb0c19ba269`; historical digest `14fa9cc585c9cf8d9b2c0ae04d7f92e6411b38fb7d19128359126eee2735f977`, exactly matching the original Phase 163 verification receipt. `currentOrdinaryQualification` is false. Original Phase 162/163 reports, receipt bytes, digests, dates, counts, source IDs, synthetic-root/work/maturity limits and companion documents remain unchanged; scoped `git diff --exit-code` verified this.

## Actual Verification

| Check | Final result |
| --- | --- |
| Phase 164 new/actual-source mutations | 331 passed, 0 failed; 67.56s; session 79007; `/tmp/phase164-08-new-final.log` |
| Phase 123 affected mutations | 90 passed, 0 failed; 2.19s; `/tmp/phase164-08-p123-final.log` |
| Phase 158 affected guards | 610 passed, 0 failed; 297.78s; session 94432; `/tmp/phase164-08-p158-final.log` |
| Phase 162 integrated proof mutations | 2,616 passed, 0 failed; 185.76s; session 94432; `/tmp/phase164-08-p162-final.log` |
| Phase 163 provenance mutations | 124 passed, 0 failed; 0.317s; `/tmp/phase164-08-p163-final.log` |
| Phase 163 native CLI/filesystem/map fixtures | 160 passed, 0 failed; 211.44s; session 94432; 160 owned roots cleaned, live control/proof hashes unchanged; `/tmp/phase164-08-p163-native-final.log` |
| Normal offline checkers | Phase 123, 158, 159, 160, 161, 162 and 164 all exit 0 |
| Source/syntax/shape | Eight new TS modules transpile; all new script lengths remain below the repo bound; `bash -n scripts/verify.sh` and `git diff --check` pass |
| Repository guards | Managed checks: zero findings; breadcrumbs: 1,160 Rust paths verified |

The six focused suites total 3,931 passing tests. Explicit `./` Bun paths selected the named suites. Initial Task 1 RED was the missing new module. A later semantic mutation exposed whitespace normalization hiding a changed genesis coinbase literal; RED returned no finding, then exact-byte fingerprints and the added regression produced GREEN. Diff review also restored one inadvertently changed raw preservation premise before final legacy verification; session 16185 was deliberately stopped for that identified source error, not a quiet command or elapsed estimate. Initial reconciliation failures are superseded by the final nonzero-discovery passes above.

## Decisions, Limits and Simplification

Covered D-01–08 and T-164-20–23 through exact identity/support facts, actual opener/admission ownership, explicit fixture separation, strict historical attribution and mandatory default registration. Kept one current-source checker, one narrowly reversible fixture mapping and one versioned historical tooling view, avoiding duplicated source mirrors, dynamic HEAD authority, broad filesystem overrides and receipt regeneration. Local AGENTS guidance, the Bright Builds sidecar, relevant managed standards, overrides and active lessons informed the work.

This is a conservative fixed-corpus lexical/fingerprint guard, supplementing the executed Rust behavior, source review and parent full verifier. It is not a Rust semantic parser or a new execution/qualification receipt. Safe formatting or future source changes require explicit contract renewal. All four ordinary histories remain refused pending Phases 165–167; actual Knots qualification belongs to 168. No canonical bootstrap/reopen, whole-chain, production or funds-safety claim follows. No goal-blocking stubs, new endpoint/auth/schema surface or authentication gate was introduced.

## Deviations and Parent Finalization

**[Rule 3 - Blocking historical integration]** Parent-authorized Phase 123 split producer/input repairs and the central Phase 158/163 historical reader/digest seam, original snapshots and fixture-factory adjustments were necessary to preserve the existing historical authority while wiring current guards. All original exclusion, timing, provenance, resource and ordered-check controls remain exercised.

**[Rule 1 - Checker bug]** Exact-byte region/fact hashes replace whitespace-compacted hashes so literal bytes cannot change unnoticed. No production source or historical fingerprint input changed in that final repair.

No staging, commits, pushes, worktrees, hook changes, STATE/ROADMAP/REQUIREMENTS/todo edits or full `bash scripts/verify.sh` run were performed by this executor. Parent owns final catalog evidence registration, independent review/security, full native verification, lifecycle-valid requirement closure and normal-hook Git finalization. No fabricated commit hash or completed requirement is recorded.

## Self-Check: PASSED

All new checker modules, versioned manifest, nine original snapshots, modified historical adapters and this summary exist. Positive normal checkers and all six focused suites substantiate the result; original receipt equality and exact snapshot comparisons passed. Commit checks are intentionally inapplicable under parent-owned strict finalization. Frontmatter uses only its two leading delimiters.

## Parent Native Gate Follow-Up: Phase 75 Current Soak Evidence

Parent's first full-native attempt, session 63399, stopped after 2.979s because Phase 75 still required the obsolete binary-test name claiming ordinary startup wrote a ledger. Parent authorized only the existing `scripts/check-phase75-soak-runner.ts` and `.test.ts` contract repair. The checker now names the actual ordinary refusal test and checks five exact executable function regions in `operator_binary/status_sync_soak.rs`: bounded nonmutating refusal, explicitly synthetic ledger seeding, terminal stop rejection, report projection without append and clean-completion resume refusal. Missing/disabled/spoofed source or changed assertions cannot borrow a matching token elsewhere.

Phase 75 normal checker passes; all 21 Bun tests pass (1.78s, 43 assertions), including every original requirement/resource/timing/privacy/opt-in/verification-order control and 13 current-source mutation/missing/spoofing controls. Directly adjacent Phase 76 resource and Phase 80 opt-in normal checkers also pass; the obsolete name occurs in no other nearby checker. Managed checks, new script lengths (390/494 lines) and whitespace checks pass. No production source, startup behavior or historical receipt changed. Neither Phase 75 script is a fixed historical provenance digest input: current `d6e2abc2…` and original `14fa9cc5…` aggregate digests remain exactly unchanged, so no manifest/snapshot renewal was performed. Parent owns review and the next full-native attempt. Including this follow-up, focused passing tests total 3,952.

## Parent Native Gate Follow-Up: Complete Current-Contract Inventory

Parent native retry 24793 stopped after 8.055s at two Phase 97 constructor/path assumptions. Parent authorized a sequential inventory of registered normal Bun checkers and offline check-mode generators before another full-native attempt. The first sweep collected every exit status visibly: 74/82 passed; remaining failures were Phase 107's moved context builder, Phase 157's two ordinary daemon opener edges, Phase 127's explicit fixture integration constructor (also propagated through unchanged Phase 129), Phase 135's split startup/recovery call chain, and a shared new-README negation false positive in Phase 138/145/151. Per-checker logs are `/tmp/phase164-normal-<name>.log`.

Minimal current-source repairs now retain the original controls:

- Phase 97 reads `context/canonical_chain.rs`, pins both fallible ordinary delegation/guard functions, and names the explicit fixture metrics caller separately; retained metric kinds, append/worker/provider, public schema, dashboard/privacy and verifier controls remain intact.
- Phase 107 selects the actual executable `build_context_network` region in the new canonical module and retains relay/block-serving/inbound argument checks, scheduler suppression and privacy controls.
- Phase 127 names the actual fixture integration constructor; daemon ordinary construction and all shared authority/restart/frozen-schema/redaction assertions remain guarded. Phase 129's dependency checks pass unchanged.
- Phase 135 reads the canonical module and traces separate ordinary support-before-effects and explicit fixture recovery-before-publication routes through shared network construction. Confirmation migration, durable attachment, private 300-second checkpointing, receipt/affine ownership and shutdown ordering controls remain intact.
- Phase 157 retains guarded ordinary openers plus exactly two named cfg-test fixture startup rows, shared BASIC enable/disable selection and original fixture activation arguments. Current fixture wiring is separately attributed regression evidence, with its existing breadcrumb and real cfg/path ownership; the historical ordinary parity owner was not rewritten. Registration checks select unique executable module declarations and attributes from masked source before checking exact raw path bytes. Four whole-declaration and attribute-only comment-decoy regressions refuse; the independent review finding was resolved.
- Parent clarified only the new Phase 164 README sentence to `does not establish`; existing historical acceptance text was untouched. Phase 138/145/151 and Phase 164 normal checks pass with that wording.

Final sequential sweep: **82/82 passed**, zero suppressed failures, with `/tmp/phase164-normal-sweep-final.json` and `/tmp/phase164-normal-final-<name>.log`. Final affected suites: Phase 97 **11**, Phase 107 **15**, Phase 127 **18**, unchanged dependent Phase 129 **12**, Phase 135 **90**, and Phase 157 **676** tests passed, zero failed. Phase 157 final session 42648 ran 50.08s with 1,898 assertions; other logs are `/tmp/phase164-p107.log`, `-p127.log`, `-p129.log`, `-p135.log` and `-p157-final.log`. Total focused passing tests including both native follow-ups: **4,774**.

The exact twelve expanded checker/input/test paths are listed in frontmatter; their final SHA-256 ledger is `/tmp/phase164-legacy-current-contract-hashes.json`. Managed checks and diff checks pass; touched source files remain within repo limits. None of these repairs is a fixed Phase 163 digest input: current `d6e2abc2024d8cfeef440e0b7b352b39a2775535bd6f1dab2282dcb0c19ba269` and historical `14fa9cc585c9cf8d9b2c0ae04d7f92e6411b38fb7d19128359126eee2735f977` remain unchanged. No historical snapshot, manifest, report/receipt, Rust source, Cargo job, shared tracker, commit or full-native command was changed/run by this executor. Parent retains both native failure receipts and owns the next full-native attempt, final source/security binding and Git finalization.

## Parent Native Gate Follow-Up: Remaining Historical Test Factories

Parent native attempt 3, session 29227, stopped after 734.140s at Phase 159: 636 tests passed and one implementation-transition test generated a proof-table SHA from the raw migrated fixture. A sweep of registered test factories identified only the equivalent Phase 160 and Phase 161 factories still requiring the same adjustment. Each of the three tests changed by exactly one helper import and one historical-only proof-table getter. Their original assertions, resource/provenance controls, unknown/missing evidence checks and raw preservation maps are unchanged. Independent review reversed those two substitutions and reproduced each captured pre-repair source byte/hash exactly. Plain pre-repair diff: `/tmp/phase164-third-factory-repair.diff`.

Because those three tests are original Phase 163 digest inputs, parent explicitly authorized three additional byte-identical, nonexecuting snapshots from fixed commit `6cf34f1b96cdb7e719752395b2c52c8f531b559c`: `phase159-tests.txt`, `phase160-tests.txt` and `phase161-tests.txt`. The current version-1 manifest now binds **twelve** snapshots. The existing nine bindings, original bytes and original hashes are unchanged; only three rows were appended. Helper/checker changes are the necessary reviewed manifest/helper pins, plus per-binding missing-current-input controls. The current raw aggregate is now `6e1e19fd449f2bf77f4511cdd1cec8599fb45148ac87808d8be9575f48ba7570`; the original historical aggregate remains `14fa9cc585c9cf8d9b2c0ae04d7f92e6411b38fb7d19128359126eee2735f977`. Scope remains `historical/source-derived-regression`, with `currentOrdinaryQualification=false`. Actual historical reports/receipt bytes/digests/dates/outcomes were not rewritten. Earlier nine-snapshot descriptions above describe that earlier accepted implementation stage.

Following parent's explicit verification refinement, final pre-native evidence is **targeted**, not a claim of whole legacy-suite acceptance: Phase 159 factory positive **1 passed** (636 filtered), Phase 160 linked-factory/paired-evidence removal **2 passed** (740 filtered), Phase 161 factory positive **1 passed** (148 filtered); whole Phase 163 provenance **124 passed**; whole Phase 164 guards **352 passed**, 415 assertions, 67.99s; selected Phase 163 native entrypoint/layout/raw-preservation/rebound-proof cases **14 passed**, 146 filtered, 70 assertions, 37.19s. Session 77342 completed successfully; selected native fixtures cleaned fourteen owned roots and preserved live control/proof hashes. New missing/unknown current-input and snapshot mutations are exercised for all twelve bindings, with no actual-checkout fallback.

The final repeated normal sweep is **82/82 green**, recorded in `/tmp/phase164-fourth-normal-sweep.json` and `/tmp/phase164-fourth-normal-<name>.log`; Phase 158/162 historical readers and Phase 164 checker are green. Managed and diff checks pass. Exact current/original/new-snapshot hashes and ten-path repair scope are `/tmp/phase164-third-factory-hashes.json`. Final logs are `/tmp/phase164-fourth-guards.log`, `/tmp/phase164-fourth-p163-provenance.log` and `/tmp/phase164-fourth-native-focused.log`. No product source, raw preservation comparison, actual receipt, Cargo/full-native command, Git operation or shared tracker was changed/run by this executor. Source is stable for parent review/security rebinding and native attempt 4, which must replay **every whole legacy suite** before acceptance.

## Parent Native Gate Follow-Up: Panic Policy and Test Registration

Parent native attempt 4, session 16533, completed all Bun suites and stopped before Cargo after 1,489.836s at the production panic-site policy. The scanner treated the preservation helper's filename as production and treated the Rust `#[expect(` lint attribute as an `expect` call. Parent authorized a mechanical test relocation and exactly four locally justified genesis allowlist entries; the panic scanner itself was not weakened.

The complete 20,556-byte `chain_compatibility/preservation.rs` moved unchanged to `chain_compatibility/tests/preservation.rs`, SHA-256 `3d004f91bd72a0f3902588a36fca0438b04b5ac034bc9e227893664496835630`. The module name, every function byte and every physical preservation/control assertion are unchanged. Its parent remains `#[cfg(test)]` and now explicitly registers `#[path = "chain_compatibility/tests/preservation.rs"]`. The initial focused compile exposed Rust's source-file-relative path semantics: `tests/preservation.rs` incorrectly resolved to `src/storage/tests/preservation.rs`; this failed attempt is preserved in `/tmp/phase164-panic-node.log`. The corrected literal resolves the actual moved file. No production behavior/API changed.

The fixed 93-input checker corpus, twelve existing preservation function-edge paths, explicit Unix-test ownership path, breadcrumb registry and current catalog evidence paths were updated. A separate executable declaration/attribute-span guard checks the exact cfg/path literal rather than accepting masked or commented path text. Three adversarial mutations cover an unknown path, disabled cfg plus attribute-only comment decoy, and a commented declaration. Existing ordinary opener, immutable support and historical attribution controls are unchanged.

The four additions to `scripts/panic-sites.allowlist` are restricted to the exact genesis file and one exact needle each: the clippy lint expectation attribute and three constant constructors. `coinbase()` is private and takes no candidate input. Eight fixed script bytes plus the pinned 69-byte ASCII timestamp total 77 bytes; the output is a compile-time 67-byte array; both are strictly below `ScriptBuf::from_bytes`' 10,000-byte bound. The fixed 5,000,000,000-satoshi reward is inside `Amount::from_sats`' inclusive `[0, 2,100,000,000,000,000]` range. Thus those three constructor errors are impossible under the closed constants. The original startup-authority allowlist entry remains unchanged; no generic path, token or checker suppression was added.

Final focused evidence: direct `bash scripts/check-panic-sites.sh` passes; breadcrumbs verify 1,160 paths; scoped rustfmt, managed checks and diff checks pass. Whole Phase 164 guards pass **355 tests**, zero failures, 421 assertions, 66.15s (session 54427, `/tmp/phase164-panic-guards-fixed.log`). Serial cooperative timing-wrapped Cargo passes node `chain_compatibility` **24 tests** (1,712 filtered, 12.13s) and consensus `canonical_chain` **31 tests** (196 filtered); session 70539 logs `/tmp/phase164-panic-node-fixed.log` and `/tmp/phase164-panic-consensus.log`. No Cargo jobs overlapped.

Exact deleted/new/changed paths and final hashes are `/tmp/phase164-panic-reconcile-hashes.json`; the complete pre-move source is `/tmp/phase164-panic-preservation-before.rs.txt`. The Git-visible cumulative delta is `/tmp/phase164-panic-reconcile.diff`; untracked checker source is independently bound by the final ledger/review. Parent changed exactly two current catalog evidence references and owns CORE/automation/security rebinding, aggregate/LOC updates, full native attempt 5 (session 8391) and final hooks/Git. Earlier dated source paths and failure receipts remain attributable. The twelve historical snapshots, manifest, original receipt bytes/digests and production genesis source bytes were not changed; these paths are outside fixed historical digest inputs, so no manifest renewal was performed. Source remains frozen and this executor runs no further tests/Cargo during native attempt 5.

## Parent Native Gate Follow-Up: Clippy-Compatible Refusal Assertions

Parent native attempt 5, session 8391, passed all Bun suites, panic policy and formatting, then exited 101 after 1,487.226s at three `clippy::err_expect` findings in `context/canonical_chain/tests.rs`. Parent authorized only equivalent test assertions: each successful result maps to unit before `expect_err`, retaining the same failure message, typed error checks and preservation assertions. No production `Debug` implementation, API change or lint suppression was added. The fourth unrelated `.err()` use remains unchanged. Independent review reversed only the three substitutions and formatter wrapping to recover the complete original file bytes.

The original test SHA is `a0efec593741ffbe8147b00bc841a4bed56f85b27d20fe508ccfc0aae9d368a7`; the final SHA is `4a29bc4bcec59e8dd26e55992d47d2f08b5bc7ab87ab2ac6dd6818aeb4b078bb`. Before bytes, plain three-site diff and individual function hashes are `/tmp/phase164-err-expect-before.rs.txt`, `/tmp/phase164-err-expect-repair.diff` and `/tmp/phase164-err-expect-hashes.json`. Parent explicitly retained the existing **93-input/246-edge** guard corpus: this executed test file is independently review-bound and was not a fixed corpus or historical digest input. No contract rows, tooling manifest pins, twelve original snapshots, raw preservation premises or historical receipts changed.

The complete serial, cooperative timing-wrapped direct Rust preflight, session **29354**, exited **0**: `cargo fmt --all`; workspace Clippy with `--all-targets --all-features -- -D warnings`; workspace build with `--all-targets --all-features`; workspace tests with `--all-features`, including documentation tests; and the explicit CLI RPC-client binary test. Workspace output contains **4,449 passed, 0 failed, 6 existing ignored**, across forty result summaries, with no filtering. This includes all ten canonical RPC tests, node **1,732 passed/4 ignored**, RPC library **386 passed/1 ignored**, daemon **112 passed/1 ignored**, and black-box parity **3 passed**. The separate RPC-client binary passes **7 tests**. Clippy completed in 5.86s and build in 24.80s. Logs are `/tmp/phase164-rust-preflight-{fmt,clippy,build,tests,rpc-client}.log`. No Cargo jobs overlapped; a quiet first documentation target was observed live and completed normally.

Whole Phase 164 guards also pass **355 tests**, zero failures, 421 assertions, 67.88s, in `/tmp/phase164-err-expect-guards.log`. Source is frozen for parent native attempt 6 and independent SHELL68/security rebinding; CORE22 and AUTOMATION56 source bindings remain unchanged. This direct preflight does not replace the required parent full native verifier, normal hooks or final lifecycle acceptance. All five prior native failures and the earlier focused path-resolution failure remain attributed to their actual attempts.

## Parent Coverage Preflight Follow-Up: Formatter Failure Propagation

The separate coverage preflight, session **91777**, exited 1 because `chain_spec.rs:218` was the sole uncovered line: the initial diagnostic write's `?` error propagation. This is a preflight failure, distinct from the five full-native attempts above. One meaningful test was added to `chain_spec/tests.rs`: `canonical_chain_support_refusal_propagates_formatter_failure` formats the actual `UnsupportedValidationScope` into a `std::fmt::Write` sink that returns `fmt::Error`, then asserts the diagnostic write returns that error. Production formatting behavior, error text, prerequisites and coverage exclusions remain unchanged. Independent review confirmed removing just the inserted test restores the complete original file bytes.

The test file's original SHA is `c1eec96c79f26447b1ef2a786f6b1bca51a60eb9a3485af536f519116c8e46c7`; its final SHA is `262d8d89930115c3e2ed5cd60e654225edaf4713880f63f2352a39d13e9f2bdb`. Exact before bytes, diff and final ledger are `/tmp/phase164-coverage-tests-before.rs.txt`, `/tmp/phase164-coverage-repair.diff` and `/tmp/phase164-coverage-hashes.json`. No sensitive opener was added, so the existing **93-input/246-edge** corpus passes unchanged; no new contract row, manifest renewal or historical snapshot modification was necessary.

Serial timing-wrapped verification, session **82469**, exited **0**: formatting; full workspace all-target/all-feature Clippy with warnings denied (2.31s); focused consensus canonical-chain tests (**32 passed**, zero failed); native-contract workspace coverage clean followed by the eight packages listed in `scripts/pure-core-crates.txt`, with `--show-missing-lines --text`; and whole Phase 164 guards (**355 passed**, zero failed, 421 assertions, 65.82s). Coverage ran **1,706 passing tests**, zero failures; the text report has no `Uncovered Lines:` section and no zero-count rendered rows. The previously uncovered line 218 has count **1**. Logs are `/tmp/phase164-coverage-{fmt,clippy,consensus,clean,fixed,guards}.log`. No Cargo jobs overlapped.

Source is frozen for parent CORE/automation/security rebinding and full native attempt 6. The full workspace suite was not repeated after this test-only addition; the prior complete direct preflight remains separately attributed, and parent native verification will replay all required suites. No production bytes, coverage suppression, original receipt, twelve historical snapshots, shared tracker or Git operation changed.
