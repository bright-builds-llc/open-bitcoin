---
phase: 163-archive-safe-parity-provenance-and-milestone-handoff
plan: "02"
subsystem: verification
tags: [bun, native, provenance, archive, fixtures, hermeticity]
requires:
  - phase: 163-01
    provides: Reviewed pure milestone provenance and compatible native loaders
provides:
  - Actual CLI/filesystem/map transition and refusal matrix with bounded cleanup
  - Default verifier routes for both Phase163 test files
  - Explicit original-state 159–161 fixture compatibility and measured receipts
affects: [163-03, 163-04, milestone-handoff]
tech-stack:
  added: []
  patterns: [bounded-real-cli-fixtures, synthetic-certificate-attribution, named-refusal-assertions]
key-files:
  created:
    - scripts/check-phase163-provenance/native.test.ts
    - scripts/check-phase163-provenance/fixtures.ts
    - .planning/phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-TRANSITION-EVIDENCE.md
  modified:
    - scripts/check-phase159-filter-rpcs.test.ts
    - scripts/check-phase160-peer-filter-serving.test.ts
    - scripts/check-phase161-operator-index-evidence.test.ts
    - scripts/check-phase162-integrated-filter-proof/contracts.ts
    - scripts/verify.sh
key-decisions:
  - Execute unchanged actual entrypoint/import bytes in finite owned layouts, with separate filesystem and hermetic-map checks.
  - Reconcile exact current unarchived prose at the fixture archive boundary while preserving dated Phase162 delivery proof and certificates.
  - Keep the observed pre-extension receipts distinct from the coordinated source-inventory follow-up and final root gates.
patterns-established:
  - Synthetic completed provenance reports are explicitly labeled test data and cannot activate live requirements.
  - Missing supplied-map inputs refuse despite matching valid disk evidence under the provided root.
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 163-2026-10-12T21-09-41
generated_at: "2026-10-12T22:26:03Z"
status: complete
duration: 16min
completed: 2026-10-12
---

# Phase 163 Plan 02: Native Archive Transition Summary

**Both real historical checkers now have default-routed CLI, filesystem and hermetic-map regressions for the complete v2.5 archive handoff and targeted refusals.**

## Performance and scope

- Started: 2026-10-12T22:10:16Z after the root released the read-only preflight.
- Summary recorded: 2026-10-12T22:26:03Z; approximately 16 minutes including verification and evidence writing.
- Tasks: 2/2 implemented and focused verification passed.
- Files: 7 source/test/routing paths, transition evidence and this summary.
- Root retains STATE/ROADMAP/task-ledger ownership, commits and final full-default verification. No staging, commits, hook bypass, branch/worktree change or push occurred.
- Material guidance: AGENTS Repo-Local Guidance, Bright Builds sidecar, placeholder overrides, verification/testing/code-shape/architecture/local-guidance and TypeScript standards. Active global/repo lessons total 7,188 bytes / 2,397 conservative estimated tokens and were loaded without archives. No project skill index was present.

## Accomplishments

- Added 117 native tests: 10 explicit positive layouts plus the real four-stage closeout sequence, 79 named refusal tests, 26 omitted-map controls against valid disk evidence, and deliberate cleanup on action failure. Together they exercise 186 real CLI subprocesses, 186 filesystem checks and 238 map checks through unmodified actual entrypoint/import bytes.
- Copied only 133 finite evidence/import files, 3,286,342 bytes in the final pre-extension corpus. All 117 unique owned roots were removed; live controls, original context/proof/certificates, six proof roots and frozen source inputs remained unchanged.
- Preserved separate original bootstrap, implementation and formal states; honest linked Phase163 Pending without future receipts; canonical functional closure while audit/readiness/phase remain pending; renewed passing audit before final phase refresh; final closure; equivalent safety-copy overlap; archive-only controls; and a later unrelated active milestone.
- Wired both Phase163 explicit test-file routes exactly once immediately after 162check in the documented and executed verifier order. Both historical command snapshots gained the identical pair, preserving every old route, contiguous 161/162 sequence and full native gates.
- Reproduced and repaired only original-state 159–161 fixture compatibility: 159/160 needed explicit requirements and matching original 162 roadmap ownership; 161 also needed consistent statuses and current claim reconciliation. Existing assertions and mutations remain intact.

## Actual verification receipts

All Bun executions used pinned 1.3.9 with explicit `./` paths. Detailed commands, layouts, controls, SHA256 inventory, diagnostic bounds and attribution are in [163-TRANSITION-EVIDENCE.md](163-TRANSITION-EVIDENCE.md).

| Verification                                               | Result                                                   | Duration                                         |
| ---------------------------------------------------------- | -------------------------------------------------------- | ------------------------------------------------ |
| Initial new native test RED                                | 0 pass/1 fail/1 import error: missing fixture module     | 78ms                                             |
| Existing159–161 original positives RED                     | 0 pass/3 fail: absent map key and inherited owner/status | 242ms                                            |
| Same preserved positives GREEN                             | 3 pass/0 fail/6 assertions                               | 1.70s                                            |
| First native matrix                                        | 101 pass/8 fail; expectation corrections only            | 126.53s                                          |
| Corrected/new native controls                              | 12 pass/0 fail/125 assertions                            | 11.90s                                           |
| Final pure + native                                        | **171 pass/0 fail/1,225 assertions**                     | **133.42s**                                      |
| Full historical158/162 suites                              | **3,203 pass/0 fail/10,732 assertions**                  | **338.30s**                                      |
| Full159–161 suites                                         | **1,528 pass/0 fail/4,187 assertions**                   | **451.77s**                                      |
| Pure + active milestone traceability                       | 77 pass/0 fail/144 assertions                            | 269ms                                            |
| Actual live158/162/active traceability guards              | exit0 each                                               | Live Pending;208 original 162 function contracts |
| Managed all; bash syntax; two-space shfmt; diff whitespace | exit0; 0 managed findings                                | Focused source/routing checks                    |

The first eight native failures corrected the fixture's diagnostic/surface expectations without changing policy: canonical scope refused at the earlier 04-SUMMARY gate; mandatory native 162 source omissions emitted exact-path ENOENT; README-only stale prose was outside 162's intended input, so the common parity index now carries that mutation. Every final refusal requires its intended reason; native mandatory-source omissions require their exact path as well.

## Evidence identity and coordinated follow-up

The receipts above belong to the pre-extension corpus `41ba4b322755eea105547d5377fa526df7bee7409cc7e8002e4517ab56f30030` and provenance helper `7a768155c4a73687677e3065ce7054408c4442db297a3226e1a0b83928b6211b`. Owned source hashes are recorded in the transition evidence. The original six-source digest remains `a4b0d71d01def301ba406d6b46c351ed8944d8d4508182ffb0dfd21960043575`.

After all these runs finished, security preflight identified that the sole provenance digest list also needed the new native test/helper, 159–161 caller/test, 162 contracts and verifier inputs. Root assigned the Plan01 owner to extend that one list and refresh the frozen helper binding; Plan02 source files remain frozen. That coordinated follow-up is underway with fresh imports and a new pure/native run. Its result will be appended separately to the transition evidence and this summary; the earlier runs are not relabeled as execution of the later helper/manifest bytes. No secondary policy-binding manifest is introduced.

## Files created/modified

- `scripts/check-phase163-provenance/{fixtures.ts,native.test.ts}` — bounded finite regular-file corpus, explicitly synthetic report constructors, real CLI invocation, named positive/refusal/omission and cleanup controls.
- `scripts/check-phase159-filter-rpcs.test.ts`, `scripts/check-phase160-peer-filter-serving.test.ts`, `scripts/check-phase161-operator-index-evidence.test.ts` — explicit original requirements/roadmap/status inputs, preserving assertions.
- `scripts/verify.sh`, `scripts/check-phase162-integrated-filter-proof/contracts.ts` — two exact test routes and corresponding complete historical snapshots.
- `163-TRANSITION-EVIDENCE.md` — measured pre-extension receipts, actual sequence, corpus and source hashes, refusal and proof limits.

## Decisions and simplification pass

The fixture helper owns one finite corpus and one six-boundary runner. It imports no `.test.ts` helper that would register another test suite, creates no fake executable runner and introduces no dependencies. Construction, mutation, input hashing and owned cleanup remain in the small fixture module; table-driven tests retain one concern each. The code stays below the repository's file/function-size triggers.

Archive-only selection is separately disclosed from publication wording reconciliation. Only exact current closure/readiness strings transition to the exact archived-control sentence; original Phase162 dated delivery evidence stays intact. Synthetic future certificates are clearly test data, with no requirement activation.

## Deviations from plan

No execution outside approved ownership. The planned narrow legacy fixture repair followed actual RED evidence. The root-coordinated manifest completeness follow-up implements the existing source-binding requirement and retains its own exact byte/receipt boundary; it is not an additional runtime or generic lifecycle adapter change.

## Task commits

None. Both task commits and metadata finalization are intentionally deferred to the root strict wrapper with normal hooks. Commit existence is not claimed. This executor did not update shared STATE/ROADMAP/requirements/task ledgers.

## Issues encountered and remaining gates

- Current live CFGR-01 remains Pending. Synthetic completed/audit/archive fixture success grants no live completion, archive readiness, runtime capability, production/funds safety or shipment.
- The coordinated source-inventory extension/fresh native-pure receipt will be appended precisely when received. Full default native verification and whole-phase independent review/security remain Plan04/root gates.
- No Cargo/Bazel, real archive/tag/shipment or full native run was executed here. No known implementation stub, placeholder data source or unimplemented fixture branch prevents the transition tests from running.
- The new effects are temporary local evidence-file access and bounded subprocesses already covered by T-163-05–08; no additional network/auth/runtime trust boundary was introduced.

## Next wave readiness

Plan03 can use the transition evidence to reconcile truthful current handoffs. Plan04 must bind and verify the final combined source tree, earn actual native/review/security/canonical closure receipts, renew the milestone audit and refresh final phase publication separately. Existing exact canonical active-traceability behavior remains unchanged.

## Coordinated source-binding delta — 2026-10-12

The Plan01 owner completed the sole digest-list extension to 20 inputs and its frozen-helper refresh without changing Plan02's source files. 30 new missing/empty/tamper controls produced genuine RED, then the fresh pure/native run passed **201 tests / 0 failures / 1,285 assertions / 133.07s** (84 pure + 117 unchanged native). All 117 roots were removed and live proof/control hashes remained unchanged.

The separately identified fresh corpus is 140 files / 3,423,775 bytes, digest `1bdfaf4011a492ebb35c2d704b9199780aa0736e1a072d4d09f7710dad352c85`; its 20-input source digest is `c017d970b9f788eabecde36828534825939551105c1a214ba630e32aced1bd86`. The new helper is `a9da6af99ea418bc7822333493e61b739dfccd857e6e57dfd90cf88957772930`. Exact extension hashes and receipts are appended to the transition evidence; earlier pre-extension runs remain explicitly historical. Independent rebind acceptance and final combined-tree native/review/security gates remain root-owned.

## Self-Check: PASSED

All eight planned implementation/evidence paths and this SUMMARY exist. Both Markdown files parse their first frontmatter, preserve the originating lifecycle, contain exactly two standalone frontmatter delimiters and activate no requirement. The seven Plan02 source hashes match their accepted final pre-extension snapshot and remain unchanged by the coordinated extension. Focused suites, live guards, managed checks, shell syntax/two-space formatting and whitespace diff checks passed. The six-source proof digest independently matches its original value. No stub or new undeclared trust boundary was found. Commit checks are intentionally inapplicable before root's deferred Git gate; no commit is claimed.

## Optional-page and selector follow-up — first combined snapshot, 2026-10-12

Root authorized T-163-10 coverage for the new optional current parity page. The core owner owns its fixed optional source export and shared state-aware scanner. Plan02 initializes the page explicitly for current synthetic states, includes it in archive-only wording reconciliation, removes it from every original-state constructor and 159–161 original transition builder, and adds 18 real native cases for optional absence, six earned states and eleven targeted premature/excluded/stale-page claims, including code/quote controls. No source exception or assertion was relaxed.

The focused 18 cases passed before the later control/context patch. On root's subsequent combined freeze, fresh pure/native passed **220 / 0 / 1,438 assertions / 166.93s** (85 pure + 135 native; all 135 roots removed), and full 159–161 suites passed **1,528 / 0 / 4,187 assertions / 449.73s**. The corresponding corpus is 141 files / 3,438,636 bytes, `fe6f14f236286054f9a11375adf5f717f23641d21d108ba5d614a4675fd6711e`; 20-input source digest `63ae29d96f3016f21cd0dc586fe912fd709bf0ca6cbe03e536ca7b626d2fd09a`.

Historical 158/162 ran **3,211 pass / 2 fail / 10,744 assertions / 337.48s**. The two specific original 162 bootstrap/implementation fixtures inherit the new dated formal sentence in live index `rationale`; the core owner is repairing only their intended-state fields. Separately, review found that whitespace/bare-heading active candidates can be ignored during overlap. Plan02 captured genuine RED for two new real native/map conflict controls (**0 pass / 2 fail / 5.42s**) and froze the native source with 137 cases. Final repaired receipts remain pending; the earlier green receipts and Self-Check retain their earlier snapshot boundary. Exact logs, corpus identities and source ownership are appended to the transition evidence.

## Final repaired follow-up — 2026-10-12

The core owner repaired selector relevance to match accepted parser grammar and repaired the original 162 fixture's controlled intended-state sentences. Plan02's two real selector controls went **0/2 RED → 2/0 GREEN (24 assertions, 3.58s)**. Original162 bootstrap/implementation/formal positives independently passed 3/0. No assertion, exclusion or historical runtime proof was weakened.

Fresh final imports at 20-input source digest **`f3e9a2475b512202d89fa2973b9fb690cfc68e1511d9452d95097b30bca5575a`** passed:

- **226 provenance/native tests, 0 failures, 1,470 assertions, 168.90s** (89 pure + 137 native, including all 18 page cases and both selector controls).
- **3,213 historical 158/162 tests, 0 failures, 10,744 assertions, 333.97s**. This is the repaired rerun; the preceding 3,211/2 receipt is preserved above.
- **8 targeted 159–161 controls, 0 failures, 13 assertions, 3.50s**, after the already earned full 1,528/0 caller suite on unchanged caller/doc inputs. Root's final default verifier will rerun all final bytes.

All 137 owned native roots were removed; the 141-file / 3,441,898-byte final corpus is `ddd993e3c6463e25f586fd0fc1ca8fb2ed8ca7a5c0bd8e1c32708118ad1d2f65`. Live controls and historical proof witnesses stayed unchanged. The final native source is `c2eb4bf83eb6fee43696b6a9507b2a8242e7c48ad930766de073340ce3b2ff11`; final helper is `71655a6484ecb57c95bd5913a4f09a30e016ba1ee6d040f4bfe0d71fec963d54`. Exact final source inventory and logs are appended to the transition evidence.

Independent final REVIEW accepts this digest and SECURITY verifies 15/15 declared mitigations with zero open threats. The literal full/default native run, actual activation, renewed audit/publication and Git finalization remain root-owned. This addendum closes the two previously documented fixture failures and the selector issue without relabeling any earlier source snapshot.

## Final follow-up Self-Check: PASSED

Final source hashes and the complete 20-input digest were read independently. The final focused suites above passed; both planned Markdown artifacts preserve first frontmatter, the current lifecycle, empty requirement activation and all earlier RED/receipt attribution. No owned source/shared control changed during final witnesses. Final artifact formatting and whitespace checks passed. Root's full/default verification is still separately required; no commit or full-native result is invented here.

## Active-identity and presence follow-up — 2026-10-12/13 UTC

After root's full2 exit0 on the previous f3e9 source, review identified ignored unknown active controls. Root authorized a cardinality/line-local identity repair, preserving the exact existing fixed archive index and valid absent/equivalent/later v2.6 states. Plan02 added 15 actual native controls: RED 0/15 (13 silent acceptances plus two pre-existing refusals lacking the new active-path diagnostic), then intermediate f871 combined GREEN 273/0/1,729 assertions/ 188.57s. These are distinct earlier receipts.

Adapter review next found present empty strings were dropped as absent. Two new physical zero-byte active controls produced genuine RED0/2/ 10 assertions/ 5.49s: pure policy refused, but actual 158-map accepted. The core owner propagated a presence-aware map-backed reader through all 158–162 providers/claim views, preserving key presence without disk rescue. Child-owned fixture/caller-test sources were preserved; native source froze at `b85d81ee18eba16cc0a5e058c1cf34c7682d67b677a2d0e58e8d84f8062f28f8`.

The single final combined run at complete 20-input digest **`34443772f20ccfd57013a53d62ffd32f7fbf9e0b8700bb50be4b8ca7e2175ff8`** passed **275 tests / 0 failures / 1,761 assertions / 185.52s** (121 pure + 154 native), including all17 new identity/presence controls through actual CLI/filesystem/map paths. All 154 roots were removed; final 143-file /3,506,326-byte corpus is **`441419516af5f3996ae05ea92e16505fec70930e179ca69d1eaae49b64786bf9`**. Live controls and historical proof witnesses remained unchanged. Receipt: `/tmp/phase163-presence-final-native-pure.log`, session36473, started 2026-10-13T00:04:26Z.

Exact RED/intermediate/final source and corpus identities are appended to the transition evidence. Fresh independent review/security acceptance and root's full3/default verification remain separate; no older suite or full2 receipt is presented as verification of the repaired source. No source/shared/Git/publication action occurred during the final witness or report-only addenda.

## Presence follow-up Self-Check: PASSED

Final native/source inventory bytes and all 20 required digest inputs were independently checked. Both artifacts retain empty requirement activation, the original lifecycle, exact first-frontmatter boundaries and every prior failure/snapshot attribution. Focused final 275/0 and cleanup witnesses passed. Final report-only Markdown formatting, file existence and whitespace checks passed; no new implementation stub or trust surface was introduced. Root owns the remaining independent/default/publication/Git gates.
