---
phase: 165-shared-exact-work-authority
reviewed: 2026-10-17T04:03:22Z
client_date: 2026-10-16
depth: standard
review_kind: preliminary-source-only
diff_base: d0104a997c0362bf65b81de1c8fec8ddc3012a1b
generated_by: gsd-code-reviewer
lifecycle_mode: yolo
phase_lifecycle_id: 165-2026-10-14T16-09-59Z
files_reviewed: 11
files_reviewed_list:
  - scripts/check-phase165-shared-exact-work.ts
  - scripts/check-phase165-shared-exact-work/contracts.ts
  - scripts/check-phase165-shared-exact-work.test.ts
  - scripts/check-phase164-canonical-identity/contracts.ts
  - scripts/check-phase164-canonical-identity.test.ts
  - scripts/check-phase164-canonical-identity/test-modules.ts
  - scripts/check-phase68-active-chain-persistence.ts
  - scripts/check-phase68-active-chain-persistence.test.ts
  - scripts/verify.sh
  - scripts/check-phase164-canonical-identity.ts
  - scripts/check-phase164-canonical-identity/historical-tooling.ts
findings:
  critical: 0
  warning: 3
  info: 0
  total: 3
status: issues_found
---

# Phase 165: Preliminary Guard Review

**Reviewed:** 2026-10-17T04:03:22Z, client date October 16.
**Depth:** Standard source review with import and historical-attribution cross-references.
**Status:** Three actionable warnings; repair and subsequent review remain pending.

## Summary

Reviewed the nine assigned script paths, including new untracked files, against the fixed base. Two supporting checker/adapter modules were also read and are included in the eleven-file scope above. The implementation largely preserves literal-bearing source owners, separate enabled-test ownership, immutable historical evidence and exact default verification registration. Three gaps affect the claimed input boundary, overclaim rejection and reliability of named mutation controls.

This is source-only preliminary evidence. No Bun tests, checker execution, Cargo, Bazel, builds or runtime probes were run by this reviewer. File reads, Git inspection, hashes, inventory reconstruction and source analysis were read-only. The executor retains the exclusive heavy verification slot. Passing static mutations would establish source-policy behavior, not executed Rust semantics or daemon qualification.

All nine assigned files were byte-identical between their initial hash capture and the closing snapshot. Supporting-module hashes were captured at closure only. The parent has authorized a later scoped repair by Plan 16; those future changes are **not reviewed by this report**. The source identities below define precisely what was reviewed. Final whole-phase review remains required.

## Warnings

### WR-01: An unrelated negative phrase suppresses an affirmative qualification claim

**File:** `/Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase165-shared-exact-work.ts:101`
**Related lines:** 83–88 and 100–108; current appended controls at `scripts/check-phase165-shared-exact-work.test.ts:365` and `:386`.

**Issue:** `checkWorkClaims` skips a complete sentence whenever it contains any `no`, `not`, `without`, or other listed negative word. The deferred-phase loop uses the same sentence-wide exemption. Consequently, appending `Canonical bootstrap and reopen acceptance is complete, with no remaining blockers.` preserves every required disclaimer but skips the affirmative canonical acceptance check. `Phase 168 is complete and qualified, with no outstanding blockers.` similarly escapes the deferred-phase check. Comma-free versions with `with no blockers` have the same outcome. This follows directly from the unconditional exemption preceding claim classification; no execution is claimed.

**Fix:** Scope denial or deferral to the qualification claim itself. Preserve actual forms such as `No ... acceptance is claimed`, `... is not qualified`, and `... remains deferred`, while refusing an affirmative qualification predicate accompanied by an unrelated negative clause. Use a small conservative helper over the targeted claim forms, rather than a sentence-wide negative-word exemption. Add actual loaded-document appended controls that retain existing disclaimers, cover comma and comma-free suffixes, and include an unrelated `not` clause. Keep unchanged-current-document and legitimate-denial positives.

### WR-02: The historical manifest is imported before bounded input loading

**File:** `/Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase164-canonical-identity/historical-tooling.ts:2`
**Related files:** `scripts/check-phase165-shared-exact-work.ts:5`, `:8`, `:24`; `scripts/check-phase164-canonical-identity.ts:10` and `:17`.

**Issue:** A static JSON import loads the historical manifest during module evaluation. Phase 165 imports this adapter both directly and through the older checker. The manifest is therefore read by the module loader before `assertInputInventory`, `lstatSync`, `O_NOFOLLOW`, descriptor validation or the 512,000-byte limit. The same manifest is a declared bounded input, so an oversized or symlinked manifest is consumed before the advertised bounded-loader refusal. The inventory pin does prevent manifest-controlled rows from broadening the subsequent explicit source read set, but it cannot undo this earlier data read. Existing isolated-loader shape controls alter only the first source input and do not exercise the module-import boundary.

**Fix:** Replace the data import with the exact fixed ordered reviewed current-file/snapshot pairs. There are **twelve** actual manifest bindings. Derive read inventories from those code literals. In `historicalToolingView`, authenticate the manifest string already supplied by the bounded input map against its unchanged expected SHA before locally parsing it, and require parsed binding paths to match the fixed pairs. Preserve all original manifest, snapshot, corpus and receipt bytes. Renew only the reviewed current adapter identities. Add fresh-process isolated-copy controls proving that importing the actual checker modules does not read a malformed, symlinked or oversized manifest; then require manifest-specific bounded-loader refusal for unsafe shapes and pure history-policy refusal for in-bound malicious contents. Do not mutate the checkout to construct these controls.

**Independent design check:** The parent proposed precisely this bounded separation; it is feasible without another parser or read surface. The CLI's inspected static dependency graph contains thirteen TypeScript modules and exactly one data import, this manifest. Source-module imports are allowed; external corpus data must use the bounded shell. Test-only `historical.test.ts` separately imports the manifest but is outside this CLI dependency chain and must not be relabeled as an independently bounded entrypoint. An in-bound malicious manifest should be inert data in `loadInputs` and fail the pure history policy; it need not broaden the loader's responsibilities.

### WR-03: Named test mutations repeatedly alter the first matching attribute span

**File:** `/Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase165-shared-exact-work.test.ts:244`
**Related lines:** 154–159 and 230–247.

**Issue:** The loop selects a particular `fn` and calculates its preceding attribute text, but passes that text to `mutation`, which calls `source.replace(before, after)`. Ordinary test declarations share the same `#[test]\n` span. Later cases therefore mutate the first matching declaration in the file, rather than the test named in the generated label. Source identity checks still reject that changed file, so the matrix can pass while never exercising disablement of the advertised later case. This is a test-reliability finding, not a production Rust semantic failure.

**Fix:** Retain the calculated attribute start and `fn.start`, then splice exactly that range with `source.slice(0, attributeStart) + replacement + source.slice(fn.start)`. Assert that the named function's attribute region changed and earlier functions remain byte-identical. Keep the actual loaded positive baseline and intended owner/rule assertion. Apply this to removal, ignore and disabling mutations without reducing the corpus or weakening assertions.

## Preserved Boundaries and Static Evidence

- The explicit Phase 165 loader calls `assertInputInventory` before its own corpus reads. Independent reconstruction produced **216 ordered paths**, SHA-256 **d53e4c64edcb651547895005295f7a269841c52aaecbd84d47eb2d72141e00c7**, matching the current pin. All paths exist. WR-02 is the separate earlier import defect.
- Explicit reads reject nonregular final components, open with `O_NOFOLLOW`, recheck regularity on the descriptor, consume at most 512,001 bytes, and reject lengths above 512,000. Cleanup closes descriptors in `finally`. No fallback checkout reads, corpus traversal, network calls or writes are present in that shell. This is not a guarantee against symlinked ancestor directories.
- The current contracts contain **84 literal-bearing source-owner rows** and **28 enabled-test-owner files**. Source identity includes code, literals and attributes after leading masked commentary. Existing lexical helpers inspect actual function attributes, file attributes and enclosing inline ownership. Module-registration owners are separately pinned. WR-03 limits the matrix's per-named-case attribution; it does not remove those production policy checks.
- Against the fixed base, the older edge registry changes from **246 to 463** rows: twenty existing identities change, 218 rows are added, and one old path row is removed. That removed `transient_runtime` row relocates to `network/canonical_admission.rs` with the **same digest**. None of the twenty renewed rows belongs to a byte-unchanged source file. This rejects the obvious unchanged-owner rebaseline concern; full Rust semantics remain the other source reviewers' responsibility.
- All **fourteen** current historical-identity hashes match their constants. The historical manifest is unchanged versus the base, SHA-256 **9783fa03fdd2df1395445b7752e6c31d84985c351ef44de516656af7f82fc7c0**. Every one of its twelve original snapshot hashes matches. No historical receipt identity was renewed by this review.
- The independent corpus SHA-256 remains **3261c2d2c73cbd6977a30691b40514e4d2940abfbfc500b145ccdcc6b10d7451**: source pin **a9aee730466ac67d35a3c03ee24676be5e045878**, evidence class `source-derived-arithmetic`, eleven proof and six addition vectors. Exact numerical expectations are hash-pinned, rather than regenerated with the production helper.
- Current preservation source remains **20,688 bytes**, SHA-256 **e6cb5740cb5c83655aed14c43b1206847d264431d944741a365771c3e8d76f5b**. Its protected original 20,556-byte source/blob attribution remains separate. This review does not reinterpret current numerical carriers as historical validation provenance.
- `verify.sh` differs from the base by exactly **six added lines**, each occurring exactly once: Phase 165 test/check after Phase 164 in both declared and executed surfaces, and Phase 68 test before its checker in both. Removing only those six exact lines reproduces the base byte-for-byte, SHA-256 **2925344e06e3604fbba1821e49a00b0f2f292448eae205107882d0ed65aacc43**.
- `verificationHistoricalBase` permits only exact anchored groups and preserves unrelated bytes. The older mandatory checker independently requires all six additions. The actual all-six-removal control invokes only `checkPhase164`, preserves historical-view validity and rules out missing-input failure; it cannot pass merely because the newly added checker detected its own removal.
- The mutation matrix uses real loaded source maps and checks anchors. Source-owner tests cover arithmetic width/wrap, selected admission, strict header preference, provenance/stages, body order/linkage/epochs, owner-before-effects, fallback/error propagation, legacy refusal and fixture boundaries. Corpus, receipt and verifier controls remain distinct. The new fixture controls require an unchanged positive baseline and owning path/rule; none is evidence that mutated Rust was compiled or executed.
- Phase 68 now requires an actual `ProofWork` carrier and lossless work projection, while masking Rust comments/string literals for that policy. Its six tests include narrowing, height substitution and comment-decoy regressions. Imported test execution and checker main execution are separated by `import.meta.main`.

## Evidence Attribution and Remaining Gates

Plan 15 records 34 private plus fourteen external executed Rust cases, 781 current Phase 164 guard cases and 1,069 current Phase 165 cases, with 221 stable saved input/checker identities. These are **executor-reported receipts read for intent and context**, not independently executed results of this review. The prior 837/843 counts remain predecessor receipts rather than current matrix totals.

The actual Plan 14 workspace diagnostic remains **249 failures** and is not erased by focused checker success. Later integration repairs, complete default native verification, full source/security review, formal goal verification, normal hooks and Git finalization remain pending parent gates. Canonical bootstrap, exact all-handle reopen and genesis BASIC remain Phase 166 work; contextual completeness remains Phase 167; actual pinned-Knots daemon qualification remains Phase 168. The 32 local Open Bitcoin refusal children are not pinned-Knots differential evidence.

Guidance materially applied: root `AGENTS.md` and repo-local native verification/timing/breadcrumb rules; `AGENTS.bright-builds.md`; placeholder-only `standards-overrides.md`; architecture, code-shape, verification, testing and TypeScript/JavaScript standards. Both active lesson files were fully read: 7,188 bytes and 2,397 conservative estimated tokens, with no archive loading. No `.claude/skills/` or `.agents/skills/` directory was present. None of the nine assigned paths is Git-ignored; no `.claudeignore` was found. Only this report was created; source, shared state, Git index and other agents' artifacts were preserved.

## Initial and Closing Full-File Identities

Initial identities for the nine assigned paths were captured before the source analysis. The closing snapshot at 2026-10-17T04:03:22Z matched those nine rows. The final two supporting-module rows are closing-only identities, without a separately captured initial hash. Hashes are full-file SHA-256, distinct from lexical function/source identities.

| Path                                                              | Initial and closing SHA-256                                      |
| ----------------------------------------------------------------- | ---------------------------------------------------------------- |
| `scripts/check-phase165-shared-exact-work.ts`                     | be3cf992cf6dabadfe1bde05becd8145612911ac03e8d715b699a82446830e76 |
| `scripts/check-phase165-shared-exact-work/contracts.ts`           | ff8ef8d9e6573c1edd7969f470864913ab7b5b1cdfeabd55afb2380454320437 |
| `scripts/check-phase165-shared-exact-work.test.ts`                | 9f22efe3c6b7a1e335b0bce369ab2981a147978e66e05c9ab3f3b06145ec5858 |
| `scripts/check-phase164-canonical-identity/contracts.ts`          | 08df122587582cdb728ea2a699535377b67cac41019c08d7454dba43a5a7f690 |
| `scripts/check-phase164-canonical-identity.test.ts`               | 0bbeb183dcbed3ea1c126e36c4fa4a4fd02e955947521b3e398cb09bac05cffb |
| `scripts/check-phase164-canonical-identity/test-modules.ts`       | be66396b89e593c7493fb8e8a6a0701c0a758566fd31f768b777f4ac796a29b5 |
| `scripts/check-phase68-active-chain-persistence.ts`               | b923b54c80d84dfbf6b2cde2b63fe115e70e73ac46a38bd0fcc5f7b512febf6e |
| `scripts/check-phase68-active-chain-persistence.test.ts`          | 0733297ecdbe5c272fe41cf7df21bb8ccc6f36499a1c49c5b64800907a1bf9e4 |
| `scripts/verify.sh`                                               | e1823a94c0394107ecdfbb194b32d93f290cc7b79195dac5583dd7cc7a811fb7 |
| `scripts/check-phase164-canonical-identity.ts`                    | 66a938e4f5dcf9067c1d3d421a7a001c121e4976d325e29a251f907ab57a7f62 |
| `scripts/check-phase164-canonical-identity/historical-tooling.ts` | aad84f340625cbbfa8bdb9fd5827ba4fdbde90fa98c0fe5a5492d5c7eeb5c9c1 |

______________________________________________________________________

_Reviewer: gsd-code-reviewer. Preliminary source-only report; no phase acceptance or independent execution receipt._

## Fix Review 1 | 2026-10-17T04:08:31Z

**Disposition:** WR-02 and WR-03 are addressed in source; WR-01 remains open. No whole-fix source PASS yet. Original findings, attribution and initial identities above are retained.

This follow-up is still source-only and report-only. No tests, checker CLI, compilation, native verification or heavy commands were run. Current adapter digest renewal and actual executor RED/GREEN evidence are pending and are not inferred from source review.

### WR-02: Source fix accepted; execution and current identity renewal pending

`historical-tooling.ts:4` now contains twelve literal current-file/snapshot pairs. Independent comparison confirms exact equality with the original manifest's pair order. `CURRENT_TOOLING_FILES` and `HISTORICAL_TOOLING_FILES` derive solely from those literals. `historicalToolingView` at lines 60–66 SHA-authenticates the bounded map's manifest string before local JSON parsing and then requires exact parsed pair equality. Its remaining snapshot/current hash and exact verifier projection behavior is retained.

The inspected CLI dependency graph still contains thirteen TypeScript modules and now contains **zero data imports**. The original manifest SHA remains 9783fa03fdd2df1395445b7752e6c31d84985c351ef44de516656af7f82fc7c0; every one of its twelve original snapshot hashes still matches. The original manifest and snapshots were not changed. The older entrypoint's stale current adapter hash and current165 contract hash are intentionally not yet renewed; this is expected before the parent's reviewed renewal gate, not a request to rebaseline historical evidence.

The current test source copies the actual thirteen-module tree and actual fixed inputs into an isolated directory, starts a fresh Bun child to import the copied entrypoint, and asserts clean import with missing/malformed manifest data. Subsequent checks demand a manifest-specific missing error or pure history-policy refusal. Separate symlink/oversized tests populate preceding inputs and require manifest-specific bounded-loader errors. These are substantive controls, but at this snapshot the fresh-import cases themselves do not yet include symlink-invalid, oversized-invalid or valid malicious-inventory JSON. The parent requested those additional shape controls; their future implementation is outside this snapshot.

### WR-03: Exact-offset source fix accepted; execution pending

`replaceTestAttributes` at `check-phase165-shared-exact-work.test.ts:230` derives the actual preceding `#[test]` offset and splices that offset through the selected function start. The generated removal/ignore/cfg controls now use this helper and assert that all bytes before the target attributes remain identical. They also assert the replacement at the target offset before requiring the intended owner/rule finding. A dedicated second-test regression uses repeated attribute text and preserves the earlier declarations. The suffix splice retains the selected function and all later source. This addresses the original first-match mutation defect without removing existing test owners or controls.

### WR-01: Initial repair still allows unrelated denial or conditional text

At this snapshot, `claimClauses` separates `and <known subject>`, so `Canonical bootstrap is not qualified and Phase 168 is complete and qualified.` is no longer one shared clause. However, `deniesClaim` still uses a whole-clause prefix for `until` and `only after`. It incorrectly exempts `Unrelated status waits until Friday while Phase 168 is complete and qualified.` and `Unrelated work happens only after review while canonical bootstrap is complete.`. These are static control-flow conclusions, not executed probes.

The first status search also includes the adjective `deterministic`, so `Deterministic restart tie parity is not proven.` is falsely rejected: the first match precedes the actual `is not proven` predicate. Restoring `acceptance` and `parity` to the target vocabulary is not itself a fix for local predicate scope. The existing original replacement/appended overclaim assertions remain present, and eight new trailing-negative rejects plus attached-denial positives were added. During this review two more mixed-target controls were appended to the test file; no stable-start test execution is claimed.

The parent and executor replanned a bounded remedy before another implementation: identify explicit known subject occurrences, bound each local span by the next known subject, start status-predicate analysis after the subject phrase, and inspect only immediately governing denial/deferral/conditional markers. Pre-subject text cannot exempt the target except an immediately attached `No` with enumerated modifiers. Evaluate every actual status predicate in the local span: `Phase 168 is not complete and is qualified.` must not inherit the first predicate's denial. Current legitimate required `No ... claimed`, target `is not ...`, and `remain deferred` phrasing must stay valid. This design is accepted pending actual source and controls; it does not require a general natural-language parser.

### Fix Review 1 Closing Identities

| Path                                                              | SHA-256 at 2026-10-17T04:08:31Z                                  |
| ----------------------------------------------------------------- | ---------------------------------------------------------------- |
| `scripts/check-phase165-shared-exact-work.ts`                     | 82bd089fd3dc8e6ae869dad6f6020892a9c3544261860085c59405a29e93b41e |
| `scripts/check-phase165-shared-exact-work.test.ts`                | c079dbd9256c7c35ea47dc2c7ee2e7bc85495daa9c57e863580fedec70662631 |
| `scripts/check-phase164-canonical-identity/historical-tooling.ts` | bf7dfd86a92628b70de4e6099573786f56c9dc7a1f611c3ac8ca10fe26be8f9e |
| `scripts/check-phase164-canonical-identity.ts`                    | 66a938e4f5dcf9067c1d3d421a7a001c121e4976d325e29a251f907ab57a7f62 |

Other initial report scope/limits remain applicable. Further source changes need a new closing snapshot and review; these hashes do not authorize final phase acceptance.

## Fix Review 2 | 2026-10-17T04:09:55Z

The implemented per-subject scanner addresses the earlier mixed-denial and unrelated pre-subject `until`/`only after` probes. Actual status scanning starts after known subject words, so `deterministic` in the subject no longer defeats `is not proven`. Each status predicate receives its own local prefix, keeping `is not complete and is qualified` distinct from `is not complete and is not qualified`. Original daemon/restart/canonical replacement and appended assertions remain present. Execution results reported by the executor are not independently run here.

The isolated fresh-process loop now covers seven actual temporary-manifest shapes: missing, malformed JSON, symlink to invalid JSON, oversized invalid JSON, valid JSON with redirected path, extra binding and reordered bindings. It imports the real copied thirteen-module dependency tree in a fresh child and compares exported `INPUT_FILES` to the parent inventory. It then requires manifest-specific bounded loading refusal for unsafe shapes or pure history-policy refusal for inert changed JSON. This closes the earlier control-shape omission at source-review depth; execution remains executor-owned.

**WR-01 remains open for compound phase subjects.** At `check-phase165-shared-exact-work.ts:108`, `Phase 168 pinned-Knots daemon qualification is complete.` is split into a Phase target with no status and a Knots target with the `complete` status. The emitted result at line 118 is `phase: false`. Lines 84–86 then ignore that finding in non-work claim files. A similar split affects `Phase 166 canonical bootstrap is complete.`. An appended statement outside the inherited bounded README section must still be rejected by the existing all-claim-file deferred-phase policy; the work-page-only classifier cannot substitute for that policy. This is a static source trace. The parent identified this bounded compound-subject probe and the executor has been asked to preserve phase attribution through a compound subject without merging genuinely separate clauses.

Closing snapshot: current165 entrypoint **8084bb058ff1e2c46d44b89bbf2de5fcc7c08523e32f36c9fc9ac1faf796f7d9**; current165 test **042dee09703fc3f88fa1ec6c93ee483ecc7746624c01bba11b93d0c9b54cf721**; current historical adapter **bf7dfd86a92628b70de4e6099573786f56c9dc7a1f611c3ac8ca10fe26be8f9e**. WR-02/03 source fixes remain accepted. No whole-fix source PASS or digest renewal is authorized by this partial review.

## Fix Review 3 | 2026-10-17T04:14:35Z

**Latest disposition: SOURCE PASS. All three original findings are addressed in the reviewed source delta; zero open findings remain in this bounded fix review.** Original warning counts, rejected intermediate versions and identities remain above for attribution. This closes the preliminary source findings only. The executor still owns actual complete guard execution; full native, final independent review/security and formal phase acceptance remain parent gates.

### WR-01 closed in source

`affirmativeScopeClaims` now scans explicit known targets and their local status predicates rather than exempting a whole sentence for any negative word. Each predicate has its own local prefix, so unrelated trailing negatives and an earlier denied predicate cannot hide a subsequent affirmative predicate. Text before a target cannot supply unrelated `until`/`only after` authority. `deterministic` in the known restart subject is consumed as part of that subject, preserving `is not proven` denial. Phase attribution and immediately attached `No` propagate through compound subjects without a finite-verb connector, preserving the non-work-file phase policy and legitimate compound denial.

The actual original six replacement/appended overclaim assertions remain present. Additional loaded-source controls cover eight trailing-negative cases, mixed targets, unrelated leading No, unrelated pre-subject until/only-after, separate predicates on one subject, compound Phase 166 canonical and Phase 168 pinned-Knots subjects in README outside the inherited bounded section, and legitimate attached denials including the compound No case. Predicate vocabulary adds only `achieved` and `available` for the two concrete prior-policy regressions: `Canonical bootstrap parity is achieved.` and `Canonical bootstrap acceptance is available.`. Their `not achieved`/`not available` controls remain valid. This retains known acceptance/parity rejection semantics without interpreting those subject nouns as affirmative status by themselves or introducing a general language parser.

The pure `claimTexts` helper parses already-loaded bounded JSON, recursively retains every string value leaf separately, and emits an owning `input` finding for malformed JSON without raw-text fallback. Independent fields/rows cannot accidentally combine a Phase 166 note with another status field. Four actual index controls retain the unchanged-index positive, each of the two single-leaf compound overclaims and malformed input. No loader read, dependency or write was added.

### WR-02 and WR-03 remain closed in source

The fixed twelve-pair historical inventory and SHA-before-local-parse behavior remain as reviewed. The actual CLI dependency graph now contains fourteen TypeScript modules following the separately approved static extraction and **zero data imports**. The fixed fresh-process copy list contains exactly those fourteen modules. All seven isolated manifest shapes and exact exported-inventory assertions remain enabled; only bounded loading handles unsafe shape, and pure history policy handles inert changed JSON. Current adapter hashes were narrowly renewed to the reviewed adapter bytes; original manifest, original snapshots, corpus and receipt identities were not renewed.

The enabled-test mutation loop retains the exact-offset helper and earlier-byte preservation assertions, with each named function's own attribute span changed. The dedicated repeated-attribute second-test control remains enabled. Test function bodies and later source are preserved by the suffix splice. No test suppression, scope exclusion or source-policy bypass was added.

### Approved static extraction verified

The parent authorized only the `SOURCE_FILES` extraction for Plan 16. New `scripts/check-phase164-canonical-identity/source-files.ts` contains the literal array and no imports, input reads or other executable behavior. The existing contracts module re-exports it; `EDGES` and `TEST_MODULES` retain their values and order. No test-support extraction was implemented in this snapshot. The new module was fully read and adds one source-review path beyond the original eleven-file list.

An independent attempt to capture before-arrays raced after the executor's extraction and therefore is not described as independent pre-extraction capture. The executor retained its actual pre-extraction arrays in ignored `.local/open-bitcoin-dev/phase165-plan16-extraction-before.json`. This reviewer independently parsed the current three arrays and compared each complete value/order against that receipt: all match exactly. The recorded four-array before/after identity log was also inspected. The 219-path ordered inventory was independently reconstructed before extraction and its digest remains unchanged by extraction.

| Serialized value                 | Count | SHA-256, equal before/after                                      |
| -------------------------------- | ----- | ---------------------------------------------------------------- |
| `SOURCE_FILES`                   | 135   | fa0f5cc9d772ab172fb66d517f5cc99b537eb3bc5b1a2b4e3155a4cf06d39d32 |
| `EDGES`                          | 477   | 966a88d1af0f1a917e4e0ca126f4eb7516e3dcdcb7a986cfe8221000daddb14b |
| `TEST_MODULES`                   | 26    | d47862d042e6376cce6f42e7d737ba5929228ff0a466800fe6ccff29e8d75614 |
| Ordered current165 `INPUT_FILES` | 219   | 9656243abbd04bbb6e3ea6d78c8bcbb78a4312e434b8667e7edef25178fd2472 |

The growth from the initial review's 216 inputs/84 owners to 219 inputs/89 owners is the separately approved Plan 16 benchmark/CLI fixture scope. It predates and is distinct from extraction, which does not broaden reads or justify another inventory pin renewal. This review checks the guard wiring and actual owner-attributed mutations; it does not substitute for a semantic review of those Rust fixture implementations.

### Preserved identities and execution boundary

All fourteen current historical identity rows match their expected hashes after the single approved current adapter renewal. The manifest remains SHA-256 9783fa03fdd2df1395445b7752e6c31d84985c351ef44de516656af7f82fc7c0; all twelve original snapshot hashes remain exact. The six verifier additions remain unique and project to the unchanged original verifier SHA-256 2925344e06e3604fbba1821e49a00b0f2f292448eae205107882d0ed65aacc43. The corpus and protected preservation-source identities retain their initial report values.

The executor reports actual focused RED/GREEN receipts, strict Rust checks and focused source-control success. This reviewer did not execute those commands or independently certify their counts. Full matrices were held pending this source verdict. Passing this source review permits the parent to proceed to actual serial verification; it does not turn static mutations into compiled Rust semantics, resolve the original workspace diagnostic by assertion, or qualify later canonical/differential outcomes.

### Final reviewed source identities

Closing hashes at 2026-10-17T04:14:35Z:

| Path                                                              | Full-file SHA-256                                                |
| ----------------------------------------------------------------- | ---------------------------------------------------------------- |
| `scripts/check-phase165-shared-exact-work.ts`                     | 570f4ff3bd5b3611295b379113e85af28877ff61be1bec43cd81d2b9e0f8be75 |
| `scripts/check-phase165-shared-exact-work/contracts.ts`           | 0d298a49c5d09a231dc938cbaf2cbb483a612f86e27850c0fd381530cada54ee |
| `scripts/check-phase165-shared-exact-work.test.ts`                | 937a72a2ebe64edc0f50668f2525e9cd077134d0c43178c612064c5d62dab468 |
| `scripts/check-phase164-canonical-identity/contracts.ts`          | d0d547888bc3ff98bae7b3f0b643c4fe48fad6275bb5a7f70e86227c2207afd1 |
| `scripts/check-phase164-canonical-identity.test.ts`               | fb0a15cd8e318e2d1d89d80abee594e48068f1604496c484d1b3d8dbb27ccb23 |
| `scripts/check-phase164-canonical-identity/source-files.ts`       | 5cd178af07ea7f27204a2586d94bf8b1731dc6fd6b32c7b1ded0e07b6fc32832 |
| `scripts/check-phase164-canonical-identity.ts`                    | 1d2990c348e4e9ef367f38320475266e300cbdab98de38693b67abc9d3e5e0f8 |
| `scripts/check-phase164-canonical-identity/historical-tooling.ts` | bf7dfd86a92628b70de4e6099573786f56c9dc7a1f611c3ac8ca10fe26be8f9e |

The initial report's test-modules, Phase 68 checker/test and verifier hashes remain unchanged. Future production/test/helper changes require new review or explicit delta attribution. Only this report was modified by the reviewer.

## Fix Review 4 | 2026-10-17T04:27:18Z

**Latest disposition: SOURCE PASS, zero open findings in this final test-only delta.** The parent subsequently approved the previously deferred test-support extraction within Plan 16. This section supersedes only the earlier statement that no such extraction had yet occurred; all prior findings, reviewed identities and execution boundaries remain preserved.

New `scripts/check-phase165-shared-exact-work/test-support.ts` is a fully reviewed 23-line module. It exports the identical fourteen-entry `CLI_MODULES` list and callable `copiedInputs(root, realInputs)`. Filesystem calls occur only inside that explicit function. Module evaluation declares data/functions and imports Node built-ins; it performs no filesystem reads/writes, checker loading, subprocess work or test registration. The newly added `dirname` import supplies the helper's actual directory creation calls.

The saved `.local/open-bitcoin-dev/phase165-plan16-support-before.json` contains the complete previous test file and extracted block. Independent comparison confirmed that the helper block is identical after only the required explicit `realInputs` parameter and `import.meta.dir` root-depth substitutions. Moving from `scripts/` to its child directory correctly changes the repo-root traversal from `..` to `../..`. Module values and order match exactly. The current test-file diff contains only the new helper import, removal of now-unused `readFileSync` import, removal of the extracted list/function, and the call change to `copiedInputs(root, real)`. All tests, generated case tables, seven fresh-process manifest controls and assertions remain in the original enabled `.test.ts` file unchanged.

One attribution correction: an independent line-anchored count finds **33 actual test registration declarations before and after**. The executor's extraction receipt says 34; that broader textual count is not treated here as the number of declarations. This difference does not represent a removed test: the complete saved/current source diff proves all registration declarations and case tables are preserved. Dynamic generated test totals remain separate executor evidence.

The production CLI dependency graph remains exactly fourteen modules with zero external data imports, matching the moved fixed copy inventory. The test helper is not added to that production graph. The current checker entrypoint hash remains 570f4ff3bd5b3611295b379113e85af28877ff61be1bec43cd81d2b9e0f8be75. The ordered 219-input pin remains 9656243abbd04bbb6e3ea6d78c8bcbb78a4312e434b8667e7edef25178fd2472. All fourteen historical identity rows still match; no historical-data identity renewal follows from this test-only extraction.

Final reviewed delta identities at 2026-10-17T04:27:18Z:

| Path                                                       | Full-file SHA-256                                                |
| ---------------------------------------------------------- | ---------------------------------------------------------------- |
| `scripts/check-phase165-shared-exact-work/test-support.ts` | 19cfa9b55b5a46be045fa6a7d6a24e79ffdc558290ed5a7227b5d512c04e6f6a |
| `scripts/check-phase165-shared-exact-work.test.ts`         | 3ce93d309a534e5ad10cfdff6f058ef69026e898b817c918cd6644b8b3b4fd26 |

The executor reports that a first focused extraction run exposed the missing `dirname` import, then a repaired focused run passed 69 cases. The earlier frozen 841/1,165 complete guard receipts precede this extraction; they are not described as final extracted-source results. This reviewer did not run tests/builds or independently certify those execution counts. The parent may proceed with the affected complete current165 rerun on the extracted source. Final whole-phase/native/security/formal gates remain outstanding; only this report was edited by the reviewer.
