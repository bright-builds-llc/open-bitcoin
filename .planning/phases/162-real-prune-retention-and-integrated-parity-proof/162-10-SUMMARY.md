---
phase: 162-real-prune-retention-and-integrated-parity-proof
plan: "10"
subsystem: testing
tags: [bun, claims, source-binding, historical-evidence, lifecycle]
requires:
  - phase: 162-09
    provides: Frozen actual private transport receipts and complete/partial-write controls
  - phase: 162-15
    provides: Preserved lineage, IfNeeded, lazy checkpoint and denial mutation guards
provides:
  - Shared lazy Pending, implementation and formal claim validation for historical checkers 158–161
  - Source-hash-bound scoped evidence with exact ownership and requirement consistency
  - Positive filesystem/source-map transitions and independent exclusion, history and formal mutations
affects: [162-11, 162-12, native-verification, formal-activation]
tech-stack:
  added: []
  patterns: [Fixed optional evidence roots, exact approved sentences, GFM-compatible hash tables]
key-files:
  created:
    - .planning/phases/162-real-prune-retention-and-integrated-parity-proof/162-10-SUMMARY.md
  modified:
    - scripts/check-phase158-validated-reorg.ts
    - scripts/check-phase158-validated-reorg.test.ts
    - scripts/check-phase159-filter-rpcs.ts
    - scripts/check-phase159-filter-rpcs.test.ts
    - scripts/check-phase160-peer-filter-serving.ts
    - scripts/check-phase160-peer-filter-serving.test.ts
    - scripts/check-phase161-operator-index-evidence.ts
    - scripts/check-phase161-operator-index-evidence.test.ts
key-decisions:
  - Reuse one validator without broad subject-based completion exemptions.
  - Read optional proof sources only when the proof exists and formal reports only for Complete requirements.
  - Native evidence uses 162-NATIVE-EVIDENCE.md to avoid GSD formal-verification suffix collisions.
  - Preserve all historical source, safety, privacy, polling and finite transport checks.
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 162-2026-10-11T16-25-37
generated_at: 2026-10-11T23:20:57Z
duration: approximately 28 min
completed: 2026-10-11
---

# Phase 162 Plan 10: Evidence-Aware Historical Claim Transitions Summary

**Historical checkers now admit bounded source-linked implementation and formal claims while preserving Pending bootstrap, historical evidence boundaries and existing safety contracts.**

## Performance and Scope

- Tasks: 2/2. Eight owned TypeScript paths and this summary only. No runtime, frozen Plan09 source, historical evidence, registry, requirement, state, roadmap, todo or generated LOC changes.
- Duration is approximate from the parent session handoff marker 22:52:22.992Z; a separate executor-start clock was not captured. The first full-suite log was created at 22:59:32.704926Z. Completion metadata uses the actual observed UTC clock above.
- Material guidance: actual AGENTS.md, Bright Builds sidecar/overrides and local architecture, code-shape, testing, verification and language standards; execute-plan workflow; current STATE and phase context/research/UI contract; Plan10 and Plan09/15 summaries.
- Both active lesson files were read completely: 7,188 bytes and 2,397 conservative estimated tokens. The existing baseline has no new audit trigger. No project skills were found. Parent supplied fetched, synchronized main a77edab276a9daa086055aa61e6143ce86bc1218 and materialized pinned Knots a9aee730466ac67d35a3c03ee24676be5e045878.
- Every child used PATH=/tmp/open-bitcoin-bun-1.3.9-fresh:$PATH and Bun1.3.9. No Cargo/Bazel invocation or dependency installation was necessary for this TypeScript-only change.

## Task 1: Current 158/159 Ownership and Claims

The Phase158 checker exports a single bounded claim validator reused by 159/160/161. Pending requirements and an in_progress unique owner pass without a future proof document or formal output. Only REQUIREMENTS and the current lifecycle CONTEXT are mandatory claim inputs. A present proof causes the six fixed current source roots below to load; all Complete checkboxes cause the four fixed formal reports to load. No directory or archive discovery was introduced.

Implementation claims require the exact owner id and ordered CFPR-02/CFGR-01/CFGR-02 requirements, registered evidence, matching lifecycle metadata, valid generated_at no earlier than CONTEXT, exactly one current SHA256 row for each source, and actual comment-aware function contracts. These inspect raw-block oracle construction, physical paired body/undo reads, legal manual planning, automatic550 effects, consuming owner close and actual integrated caller order. Rehashing mutated raw-input, paired-undo or close code does not bypass those function checks. The detailed future Phase162 source checker and actual executed runtime evidence remain separate obligations.

Checkbox and traceability states must agree across all three requirements. Formal Complete additionally requires done ownership and matching source-bound current-lifecycle formal reports; documentation alone cannot activate anything. Current gap replacements use the exact approved sentence. Existing earlier-phase owners, requirement sets, evidence lists, source checks and intentional differences remain independently enforced.

Phase158 UAT, measurements and six historical summaries, and both Phase159 measurement/daemon reports, receive unsupported-claim checks without a current-document exemption. Their original facts and maturity/measurement boundaries were not edited. All Plan15 CHECKPOINT_CONTRACTS, wrapper delegation, actual lineage, IfNeeded/no-save_snapshot, lazy/no-op/order/error checks and positive/negative tests remain present.

## Task 2: Current 160/161 Claims and Preserved Presentation

All six operator documents can replace the old pending-only ownership sentence with the exact evidence-linked current implementation sentence. Milestone pending wording may become the explicit unaudited/unshipped sentence. The stronger v2.4 crash-window limitation and the catalog's exclusions remain mandatory.

Source polling/nonmutation, raw-reason redaction, exact producer captures, nineteen/seven fact rendering, actual authenticated caller edges, resource budgets, transport write acknowledgement and module ownership contracts are unchanged. Exclusions are checked independently after removing only an exact authorized sentence. Positive implementation evidence cannot authorize bare success, BIP37/V0, assumeutxo/dual-chainstate, repair/import/download, archive, public defaults/network CI, production/funds or shipment claims.

Filesystem fixtures copy actual optional evidence when it exists. Synthetic transition fixtures explicitly normalize their own requirement and owner state; they do not inherit a later real Complete state. Phase161's two prose-removal mutations select the active exact old/implementation/formal sentence. This preserves meaningful negatives after Plan12 and formal activation. Synthetic reports exist only in memory or cleaned owned temporary fixture roots; none was written into the real phase directory.

## Executed Verification and Corrections

All full commands used explicit ./ test paths and unchanged Bun default5000ms test timeouts. Commands ran to completion and sessions were polled within60seconds; a quiet yield was not treated as a timeout.

| Command / control                                                 | Actual result                                                                  |    Time |
| ----------------------------------------------------------------- | ------------------------------------------------------------------------------ | ------: |
| Initial Phase158 new API fixture                                  | 0 pass,1 fail,1 missing-export error; expected API RED                         |    85ms |
| First targeted transition controls                                | 10 pass,1 fail: new claim inputs unintentionally expanded old breadcrumb sweep |   906ms |
| Corrected targeted158/159 transition controls                     | 16 pass,0 fail                                                                 |   3.88s |
| Phase160 missing source-map evidence handling                     | 0 pass,1 fail; semantic RED                                                    |   268ms |
| Corrected Phase160 targeted controls                              | 2 pass,0 fail                                                                  |   369ms |
| First full158/159 command                                         | 1,149 pass,1 timeout; no failed assertion                                      | 409.96s |
| Exact unchanged timed-out selector rerun                          | 1 pass,0 fail                                                                  |   994ms |
| Full160/161 required command                                      | 891 pass,0 fail;2,188 expectations                                             |  69.09s |
| Required full158/159 rerun                                        | 1,211 pass,0 fail;2,910 expectations                                           | 444.74s |
| Final source-binding/formal controls before future fixture repair | 35 pass,0 fail                                                                 |   3.51s |
| Future filesystem/source-map handoff controls                     | 41 pass,0 fail                                                                 |   6.40s |
| Final handoff controls including native filename regression       | 42 pass,0 fail;52 expectations                                                 |   6.21s |
| Final full Phase161 suite                                         | 149 pass,0 fail;180 expectations                                               |  14.27s |

The sole incidental full-suite failure was existing WR05 mutation phase159_validation_history_store_rejects_foreign_and_conflicting_batches at5193ms versus5000ms. Its neighbor transiently took4019ms versus normal roughly600ms. Exact unmodified/default-timeout rerun passed878.84ms within the994ms command. The subsequent clean full command retained the same timeout and every assertion. One leftover owned timed-out fixture was identified and removed only after that process exited. Final TMPDIR and /tmp inventories contain zero owned phase158/159/160 fixtures.

Concrete review corrections were verified independently: fast-only native text now fails an exact standalone default-command predicate; GFM-aligned/CRLF table cells preserve exact backticked path/hash identity and duplicate rejection; future filesystem fixtures carry optional sources and explicit intended state; and native evidence no longer shares GSD's formal -VERIFICATION.md suffix. The GFM positive first failed with an actual source-hash refusal, and the suffix regression first exposed two matching formal filenames before correction.

**Source sequencing:** the clean1,211-test command loaded its modules before the final exact-native-line and GFM corrections, later future-fixture repairs and native basename correction. Its result is preserved at that actual source boundary. Parent explicitly accepted final affected controls and all four live guards for scoped closure, while retaining the entire final-source native contract and normal full hook before Git finalization. The final42 controls and149 operator tests run on the final source. No timeout, semantic assertion or default verification requirement was waived.

Final live158/159/160/161 guards pass. They retain125 named Phase159 selectors,38 production/46 executable Phase160 contracts and28 function/6 caller Phase161 contracts. Managed all checks pass with zero findings; owned whitespace/diff review passes. No standalone TypeScript formatter, tsconfig, package manifest or LSP diagnostic tool is configured; Bun execution/mutations and managed checks cover these scripts. The summary uses the installed GFM/frontmatter-aware mdformat, scoped to this file. Parent retains full bash scripts/verify.sh.

Local logs are /tmp/phase162-plan10-task1.log, task1-final.log, timeout-rerun.log, task2.log, binding-final.log, handoff-final.log, handoff-native-final.log and operator-final.log, each with the phase162-plan10- prefix. They are temporary receipts, not committed parity evidence.

## Exact Plan11/12 and Parent Handoff

Implementation sentence:

> Phase 162 scoped implementation evidence is available in docs/parity/v2-5-integrated-filter-proof.md; CFPR-02/CFGR-01/CFGR-02 remain Pending; v2.5 remains unaudited and unshipped.

Formal sentence, allowed only after all matching formal gates and activation:

> Phase 162 scoped integrated proof is complete in docs/parity/v2-5-integrated-filter-proof.md; CFPR-02/CFGR-01/CFGR-02 are Complete; v2.5 remains unaudited and unshipped.

Replace the four stale current158/159/160/161 known_gaps predicates with the applicable exact sentence after the proof exists. Earlier scoped completion remains documented under its existing owner; a bare Phase161/CFNET/CFOP success assertion cannot borrow Phase162 proof. Do not edit historical reports. Owner id remains v2-5-integrated-retained-filter-proof and owns exactly the ordered three requirements above.

Proof frontmatter requires phase_lifecycle_id matching162-CONTEXT.md, lifecycle_mode:yolo, implementation_status:proven and valid ISO UTC generated_at no earlier than CONTEXT. Its body includes the applicable exact sentence and one semantic two-cell source table row per root, in this exported PHASE162_EVIDENCE order. GFM alignment is accepted. These are actual final source observations, not fabricated proof output:

| Source                                                                                     | SHA256                                                             |
| ------------------------------------------------------------------------------------------ | ------------------------------------------------------------------ |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162.rs`           | `f14b8f96862608730126bc2deb3ff60c4ec8004ee25f155f4666f3dbeb529e7b` |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/retention.rs` | `9b562a9715db42366decde5f91a0feccd10d453e728afa3e741953d82011a4ff` |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/fixtures.rs`  | `faa009380301d6e6acbf032fd814f5cf9e31ddcbdb629787477fe5fbc5d62615` |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush/tests/phase162.rs`            | `550e8f4ad1045c2eb7c05abcd774856c0ed66bbb232f0c5555d542dfafabb7e8` |
| `packages/testdata/phase162/recipe.rs`                                                     | `82c73bc4dbd2990bf5b19947defa5b1b90aada230363b44978a691c7dac21497` |
| `scripts/phase162-filter-oracle.ts`                                                        | `81cbcc907c6b92ce1b5ec789ebe9653ca7d9e94c65b0a1d3430bf251a7761ce9` |

Formal artifacts use the following fixed basenames under this phase directory. In particular native is162-NATIVE-EVIDENCE.md, avoiding the GSD first-sorted-formal-file collision:

| Artifact               | status   | generated_by         |
| ---------------------- | -------- | -------------------- |
| 162-VERIFICATION.md    | passed   | gsd-verifier         |
| 162-NATIVE-EVIDENCE.md | passed   | gsd-execute-phase    |
| 162-REVIEW.md          | clean    | gsd-code-reviewer    |
| 162-SECURITY.md        | verified | gsd-security-auditor |

Each report needs matching phase_lifecycle_id, lifecycle_mode:yolo, generated_at at least the proof timestamp, source_digest and registration in the exact owner's evidence list. Verification also needs lifecycle_validated:true; security needs threats_open:0; native needs a standalone default command line bash scripts/verify.sh, with no --fast suffix. The aggregate source_digest is SHA256 of newline-joined path:sha256 entries in exported six-root order, currently a4b0d71d01def301ba406d6b46c351ed8944d8d4508182ffb0dfd21960043575. Requirement checkboxes and traceability must agree on all Pending or all Complete; owner status must respectively be in_progress or done. Reports are required only for Complete claims/state.

No new Rust file or breadcrumb group is introduced. Parent should register actual Plan11 checker/contract/test and Plan12 proof/UAT/doc paths only after they exist, and may add this summary as execution evidence. Current registry source roots already exist. Keep existing requirement owners and all pinned Knots/source comments intact. Parent owns source/security reviews, native/formal reports, requirement/state/roadmap/todo, parity registries, generated LOC and strict Git finalization.

## Source Freeze, Simplification and Limits

| Owned source                                             | Final SHA256                                                       |
| -------------------------------------------------------- | ------------------------------------------------------------------ |
| `scripts/check-phase158-validated-reorg.ts`              | `3127623de8de726f60fe03095f49b403bbfabafcb3ba677464cc521dba92cb81` |
| `scripts/check-phase158-validated-reorg.test.ts`         | `5c6e6a63da1f00ae49d9b60d7960d84d28982959d47fd245c9625bd9fce112be` |
| `scripts/check-phase159-filter-rpcs.ts`                  | `e84ed66f7c35dbc3afee8154d581470101785587a5b1407984f595d1a847700a` |
| `scripts/check-phase159-filter-rpcs.test.ts`             | `e54fde782d7c818324f185fa7f5afc05a19706195b4e6ea3f1f671344e5d9d1c` |
| `scripts/check-phase160-peer-filter-serving.ts`          | `27bb6fd95d12048b88bbf660f9c0a49aafbff2812b3a29bde1fc8bb217a5fa91` |
| `scripts/check-phase160-peer-filter-serving.test.ts`     | `be3dd6e9c638851018915468daf431bea43b34b468ad4ba440e3aad035021293` |
| `scripts/check-phase161-operator-index-evidence.ts`      | `82c0d1a4cbc514414772ddf7c8c95c09fbd6dc58a37479b927d189eae836a68f` |
| `scripts/check-phase161-operator-index-evidence.test.ts` | `8eae75ac083e60b9852a896273cea90eda42f3ac6dde6d31d61944b522b55db2` |

All four Plan09 frozen Rust hashes independently match its published matrix. The explicit simplification pass keeps one state validator, fixed optional paths, existing function helpers and exact sentence exemptions; no duplicated claim engine, new module, framework or dependency was added. Historical breadcrumb scanning remains at its original source ownership; Phase162 detailed source/breadcrumb checks remain separate. All owned scripts remain below628lines.

## Declared Mitigations and Deviations

| Threat     | Scoped executed evidence                                                                                                                                                                                                    |
| ---------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T-162-10-1 | Prose-only, missing/rehashed raw/paired/reopen roots, duplicate ownership, stale hashes/lifecycle, premature activation and missing/failing formal reports are rejected. Positive Pending/implementation/formal views pass. |
| T-162-10-2 | Original substantive guards and Plan15 mutations remain; full historical and peer/operator suites plus final affected controls/live guards pass at the disclosed source boundaries.                                         |
| T-162-10-3 | Historical reports are untouched, independently reject unearned current/excluded claims and retain their original evidence limits.                                                                                          |

Rule1/3 corrections are the narrow breadcrumb-input interaction, missing source-map handling, exact default native predicate, GFM table compatibility, future-fixture state/copying and the parent-discovered GSD native filename collision, each described and verified above. No architecture change, outside-owned production defect or authentication gate occurred. No two failed attempts at the same repaired fixture step occurred. The single host/fixture timeout was resolved with unchanged controls and a clean complete rerun.

Task1, Task2 and metadata commits are explicitly deferred by D-14. No checkout staging, commit, branch, push or hook bypass occurred. Existing tests' Git operations occur only inside their disposable fixture repositories. No source or requirement is claimed committed or activated.

No blocking stub or new production endpoint/auth/schema/dependency exists. New fixed evidence-file reads are within the declared publication boundary. Lexical checks and hashes supplement execution and provenance review; they cannot prove arbitrary prose truth, complete runtime behavior, review independence or a native pass. Independent source/mitigation review and unresolved-High disposition remain parent gates; no ASVS certification is claimed.

BASIC-only, inherited Open Bitcoin210000 versus Knots Regtest150 halving, codec-only pressure, logical soft target versus physical observation, earlier missing-undo refusal, local resource-policy, modeled implicit-permission and v2.4 advisory limits remain. No BIP37/V0, assumeutxo/dual-chainstate, archive-scale, public defaults/network CI, implicit repair/import/download, production-service/readiness/funds, milestone audit/archive/shipment or whole-chain Knots differential is claimed. Current CFPR-02/CFGR-01/CFGR-02 remain Pending. No user setup is required.

## Self-Check: PASSED

All eight owned scripts and this summary exist; their final source hashes match the table. Both full commands, unchanged timeout rerun, final affected controls and final operator suite have the reported receipts. Four final live guards, managed checks, scoped GFM/frontmatter formatting and diff checks pass. Both owned temporary-fixture inventories are empty. The baseline commit exists. No task commit is claimed under D-14. Frontmatter identity and empty requirements-completed are correct, with exactly two standalone top delimiters. Parent-owned artifacts and frozen runtime sources remain untouched by this executor.
