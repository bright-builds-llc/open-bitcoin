---
phase: 163-archive-safe-parity-provenance-and-milestone-handoff
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 163-2026-10-12T21-09-41
generated_at: "2026-10-12T22:25:00Z"
status: focused-transition-passed
requirements-completed: []
receipt_scope: plan02-before-source-inventory-extension
---

# Phase 163 Native Transition Evidence

The actual Phase158 and Phase162 Bun entrypoints, exported filesystem adapters and hermetic supplied maps agree across the ten required provenance states. The observed live checkout remains CFGR-01 / Phase163 / Pending. Every completed, passed-audit and archived Phase163 layout below is explicitly synthetic test data, never a live acceptance certificate.

## Receipt boundary

These receipts cover the stable Plan02 snapshot before the coordinated source-inventory extension. The root accepted the results and then assigned the Plan01 owner to extend the sole `PROVENANCE_DIGEST_FILES` with the native fixture/test, 159–161 caller/test, 162 contracts and verifier inputs. That follow-up changes the pure helper and its frozen binding; these earlier runs must not be presented as execution of the later manifest bytes. A dated delta addendum will identify the fresh native/pure rerun. Full default verification, whole-phase review/security and actual closure/audit remain later root / Plan04 gates.

All Bun commands below used `PATH=/tmp/open-bitcoin-bun-1.3.9/bun-darwin-aarch64:$PATH` and explicit `./` test paths. The observed test runner identifies Bun `1.3.9 (cf6cdbbb)`.

## Observed commands and outcomes

| Command / control                                                                                                                                                     | Result                                                        | Duration / evidence                                                                                        |
| --------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------- |
| Initial `bun test ./scripts/check-phase163-provenance/native.test.ts`                                                                                                 | RED: 0 pass, 1 fail, 1 import error; fixture module absent    | 78ms                                                                                                       |
| Three original implementation positives in159–161, before repair                                                                                                      | RED: 0 pass, 3 fail                                           | 242ms;159/160 `requirements.replace` on absent map key; 161 inherited inconsistent owners/status           |
| Same three preserved positives after explicit fixture repair                                                                                                          | GREEN: 3 pass, 0 fail, 6 assertions                           | 1.70s                                                                                                      |
| First positive native matrix, before route insertion                                                                                                                  | 10 pass, 0 fail; 99 filtered                                  | 24.65s;10 roots removed; earlier corpus `561f3fe488f5b46526ff868ce9d4787d00b8ed3036d329d118d75fe359ee7ac8` |
| First complete native matrix                                                                                                                                          | 101 pass, 8 fail, 993 assertions                              | 126.53s; `/tmp/phase163-plan02-native-first.log`; expectation corrections described below                  |
| Corrected/new targeted native controls                                                                                                                                | 12 pass, 0 fail, 125 assertions; 105 filtered                 | 11.90s; `/tmp/phase163-plan02-native-corrections.log`                                                      |
| `bun test ./scripts/check-phase163-provenance.test.ts ./scripts/check-phase163-provenance/native.test.ts`                                                             | **171 pass, 0 fail, 1,225 assertions** = 54 pure + 117 native | **133.42s**; `/tmp/phase163-plan02-native-final.log`, session43773                                         |
| `bun test ./scripts/check-phase158-validated-reorg.test.ts ./scripts/check-phase162-integrated-filter-proof.test.ts`                                                  | **3,203 pass, 0 fail, 10,732 assertions**                     | **338.30s**; `/tmp/phase163-plan02-historical.log`, session92945                                           |
| `bun test ./scripts/check-phase159-filter-rpcs.test.ts ./scripts/check-phase160-peer-filter-serving.test.ts ./scripts/check-phase161-operator-index-evidence.test.ts` | **1,528 pass, 0 fail, 4,187 assertions**                      | **451.77s**; `/tmp/phase163-plan02-legacy.log`, session63151                                               |
| `bun test ./scripts/check-phase163-provenance.test.ts ./scripts/check-active-milestone-verification-traceability.test.ts`                                             | 77 pass, 0 fail,144 assertions                                | 269ms; `/tmp/phase163-plan02-pure-traceability.log`                                                        |
| `bun run scripts/check-phase158-validated-reorg.ts`                                                                                                                   | exit0; actual live Pending                                    | Native source/claim guard passed                                                                           |
| `bun run scripts/check-phase162-integrated-filter-proof.ts`                                                                                                           | exit0;208 original function contracts                         | Both new verifier routes and historical snapshots accepted                                                 |
| `bun run scripts/check-active-milestone-verification-traceability.ts`                                                                                                 | exit0                                                         | Exact canonical verification adapter unchanged                                                             |
| `bun scripts/bright-builds-check.ts all`                                                                                                                              | exit0;0 findings                                              | File lengths 1,489 scanned; active repo lessons 1,958 bytes / 653 estimated tokens                         |
| `bash -n scripts/verify.sh`; `shfmt -i 2 -l -d scripts/verify.sh`; `git diff --check`                                                                                 | exit0                                                         | Existing two-space shell style preserved                                                                   |

The eight first full-matrix failures were fixture expectation errors, not policy acceptance defects: wrong scope correctly refused at the earlier 04-SUMMARY stage; six mandatory filesystem source omissions emitted `ENOENT` with the exact source path rather than the map source-hash reason; stale closure prose had initially been placed only in README, outside 162's scanned surface. The corrections retain named reasons, require the exact missing path for native 162 reads, and mutate the shared parity index for the two-checker prose refusal. No Wave 1 code or frozen binding was changed by Plan02.

## Positive transition matrix

| Explicit layout                 | Evidence and permitted state                                                                                                           | Both actual CLIs / filesystem / maps |
| ------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------ |
| Original bootstrap              | Phase162 owns all three Pending rows; proof/formal receipts absent                                                                     | pass                                 |
| Original implementation         | Phase162/Pending rows; source-bound implementation proof; formal receipts absent                                                       | pass                                 |
| Original formal                 | Phase162/Complete rows; original six-source proof and four original reports                                                            | pass                                 |
| Linked Phase163 Pending         | Historical162 formal proof preserved; original gap audit/scope/current lifecycle; future163 receipts absent                            | pass                                 |
| Functional closure              | Synthetic canonical SC1–4 certificate and SUMMARY activate CFGR-01; audit pending/readiness unasserted/Plan04 and Phase163 in progress | pass                                 |
| Audit passed, phase in progress | Functional certificate retained; renewed synthetic passing audit earns readiness without phase completion                              | pass                                 |
| Final phase                     | Synthetic SC1–5 canonical certificate, completed SUMMARY and renewed passing audit                                                     | pass                                 |
| Equivalent overlap              | Standard archived requirements safety copy, copied v2.5 roadmap, moved audit; active requirements still present                        | pass                                 |
| Archive-only                    | Active requirements unlinked only inside fixture; active roadmap reorganized after its safety copy; archived current prose reconciled  | pass                                 |
| Next active milestone           | Archived v2.5 plus unrelated active v2.6 requirements/roadmap                                                                          | pass                                 |

One additional test executes the real sequence in a single owned directory: final → overlap/audit move → archive-only → next active milestone. The standard requirements archive envelope contains the actual `Requirements Archive` heading, `Archived` date, `Status: SHIPPED`, link to current requirements and original body. SHIPPED metadata alone is separately refused as authority. Active roadmap reorganization follows the installed complete-milestone workflow; no real archive workflow was invoked.

At archive-only publication, fixture README/package README/current parity page/index replace only exact current `CLOSURE_CLAIM`/`READINESS_CLAIM` sentences with `ARCHIVED_CLAIM`. This is normal closeout prose reconciliation, separately disclosed from filesystem selection. Both stale current unarchived sentences refuse in the shared parity surface. The dated Phase162 proof sentence, implementation proof bytes, six proof roots and original certificates remain unchanged through the sequential closeout.

## Refusal and hermeticity coverage

The 117 native tests comprise 11 positive tests, 79 native refusal tests, 26 map-only omissions against valid disk layouts, and one cleanup-on-exception test. The six-boundary helper invokes both real CLIs, both exported filesystem adapters and both map checkers; the sequential positive test invokes it four times. Thus the all-green run exercised 186 actual CLI subprocesses, 186 filesystem checks and 238 supplied-map checks, derived directly from these passing test paths.

- 38 named mutations cover milestone/baseline/owner, checkbox-only completion, duplicate checkbox/roadmap/metadata, malformed YAML, scope/dependency/current and creation lifecycle, current gap linkage, immutable audit lineage, report generator/scope/lifecycle/digest/date, premature readiness/phase completion, native/security/activation gates, audit gaps/digest/date, stale archived prose, shipment/production exclusion, original proof/certificate identity and frozen helper/test inputs.
- 23 selected active control, original snapshot, scope/context, proof root and original/current report keys are individually removed from actual disk layouts and maps. The same 23 keys are independently omitted from supplied maps while valid corresponding files remain under the provided root.
- Three selected archive keys are individually removed, and independently omitted against valid next-active disk layouts.
- Both equivalent-candidate conflicts and malformed candidates alongside valid copies refuse for requirements, roadmap and audit.
- Plan-only completion, SHIPPED without earned final closure/audit, internal-report substitution and old-owner certificate substitution refuse.
- Source-hash tampering, paired-undo/action tampering with rebound proof hashes, executable assertion removal with rebound hashes and actual exclusion-guard removal retain targeted refusals.

Every string mutation asserts changed bytes. Refusals require the intended named reason; mandatory native source omissions additionally require `ENOENT` and the exact omitted source path. Diagnostic capture is bounded: 30-second subprocess timeout, 256KiB maximum buffer and 24KiB retained error text. Positive/per-refusal cases retain 30-second bounds; the sequential transition uses 60 seconds. No fake CLI runner, import replacement, loader replacement, disk rescue, network, Git archive, Cargo or recursive checkout copy is used.

## Finite corpus, ownership and preserved hashes

The final pre-extension fixture corpus contains **133 files / 3,286,342 bytes**. It is the union of the exported 158/162 required evidence, the loader's exported finite provenance universe, and the entrypoints' actual bounded relative TypeScript import tree. The helper refuses more than 32 import modules or 400 evidence files. It writes regular owned files, never symlinks/hardlinks. `CORPUS_MANIFEST` exports the sorted `path:sha256` input rows; its SHA256 is **`41ba4b322755eea105547d5377fa526df7bee7409cc7e8002e4517ab56f30030`**. This is a fixture-input receipt, not a second policy-binding manifest.

All 117 owned roots were removed. Cleanup is verified after success and deliberate action failure through `try/finally`, with `afterEach` fallback and final absence assertions. Each fixture exit verifies the live requirements, roadmap, audit, immutable snapshots, original 162 context/proof/reports, six proof roots and frozen checker/helper inputs against start hashes.

The ordered six-source proof digest independently remains `a4b0d71d01def301ba406d6b46c351ed8944d8d4508182ffb0dfd21960043575`. Original audit SHA256 values remain `f14fa21dff9e340c9794f6190925fc42075f5624d930e551f0d5666b702697d8` and `f60ef2fa6eea08b3fe00b241a5f2ae47c9f7ea864f1649ff865d7476d43b2709`.

| Plan02 source path                                            | Stable pre-extension SHA256                                        |
| ------------------------------------------------------------- | ------------------------------------------------------------------ |
| `scripts/check-phase163-provenance/native.test.ts`            | `0e7ca3241548a2f2a75d8f3e76d63aec4635e9168953d7e537917c0af9e131b4` |
| `scripts/check-phase163-provenance/fixtures.ts`               | `b98f5409b0b4f2df125587adc3166b8f4102e05ed3c881740fce0f6358cb32ef` |
| `scripts/check-phase159-filter-rpcs.test.ts`                  | `d864652c9dcee057560d05a6bc5bdbae70165e5527678e24fca4ca029de409bc` |
| `scripts/check-phase160-peer-filter-serving.test.ts`          | `a8aaf445914dfdf32aad4d0dec406ad4b6852ce95ad36605b4af786cd60fc315` |
| `scripts/check-phase161-operator-index-evidence.test.ts`      | `4a7a2e855c435eb66ce27c2501a59a21c4c4cb86c7e40491796f6453fcf53cdb` |
| `scripts/check-phase162-integrated-filter-proof/contracts.ts` | `b5e5df1f4dc8705cdbe1ca85f9d9049a6e0188653311e00f2cacabb9f88daa00` |
| `scripts/verify.sh`                                           | `cd051fd967934b85a7489f75ad8793cc4880ef918ced776b179a8ff7fb3165bf` |

The corresponding pre-extension helper hash is `7a768155c4a73687677e3065ce7054408c4442db297a3226e1a0b83928b6211b`; frozen158 checker/test hashes remain `d079ef8ff44ca4e7728841e53a4ff3ea04d3b7266b240df21c5acd623eb3d2e8` / `759870b52a6e2dbe282592a60ab1dd7cd7f9efe35b9c54597481fb9c023b37b4`.

## Verification routing and limits

Exactly one `bun test ./scripts/check-phase163-provenance.test.ts` and one `bun test ./scripts/check-phase163-provenance/native.test.ts` now follow162check in both documented and executed verifier surfaces. Both historical snapshot arrays gained the same pair at the corresponding filtered position. Every original command and the four contiguous 161check →162oracle test →162checker test →162check commands remain. No native gate, skip, exemption or active-traceability lifecycle adapter was added or removed.

No full default verifier, Cargo/Bazel, production runtime test, actual archive/tag/shipment, current CFGR-01 completion, whole-phase security acceptance or Git finalization is claimed here. Commits and shared state remain root-owned and deferred to the normal strict wrapper gate.

## Coordinated source-binding delta — 2026-10-12

After the earlier receipts, the Plan01 owner extended the sole `PROVENANCE_DIGEST_FILES` from 10 to 20 inputs, covering both native files, the 159–161 callers/tests, 162 contracts and `verify.sh`. Plan02's seven source/test/routing files remain byte-identical to the table above. This closes the source-inventory omission without a secondary binding manifest or changes to original proof/certificates.

The extension's missing/empty/tamper controls first produced **30 fail / 0 pass in 74ms**, then the pure suite passed **84 / 84 in 140ms**. A fresh-import run of `bun test ./scripts/check-phase163-provenance.test.ts ./scripts/check-phase163-provenance/native.test.ts` passed **201 tests / 0 failures / 1,285 assertions in 133.07s**: 84 pure plus the unchanged 117 native cases. Receipt: `/tmp/phase163-digest-extension-native-pure.log`, session16434. It again removed all 117 roots and confirmed unchanged live controls/proofs.

The fresh finite corpus contains **140 files / 3,423,775 bytes**, digest **`1bdfaf4011a492ebb35c2d704b9199780aa0736e1a072d4d09f7710dad352c85`**. It expanded automatically through the exported provenance universe; no fixture inventory workaround was necessary.

| Extended binding                          | SHA256 / result                                                    |
| ----------------------------------------- | ------------------------------------------------------------------ |
| Sole 20-input source digest               | `c017d970b9f788eabecde36828534825939551105c1a214ba630e32aced1bd86` |
| Provenance helper                         | `a9da6af99ea418bc7822333493e61b739dfccd857e6e57dfd90cf88957772930` |
| Pure provenance tests                     | `6251e8692d2a21bf7da697ca022d13f3bd59cc8668831fed582962592de630fb` |
| 162 checker with refreshed helper binding | `7583661c4c2ade312f6483fee5f34a2c03edf0ceb8208719235d7bf217168ba0` |
| Affected boundary controls                | 33 pass / 0 fail; 3.31s                                            |
| Actual live 158 / 162                     | exit0 / 0.20s; exit0 / 0.88s                                       |

These are separately attributed Plan01-owner extension receipts, independently visible in the shared log; they do not relabel the earlier 133-file runs. Root retains independent rebind-review refresh, final combined-tree default verification, actual activation/audit and Git finalization.

## Optional-page and selector follow-up — first combined snapshot, 2026-10-12

Security review identified T-163-10's new current parity page as an additional claim surface. The core owner exported optional `PHASE163_DOC` (`docs/parity/v2-5-archive-safe-provenance.md`) and routed its present content through the existing state-aware exclusion scanner. The page is a mutable claim input, not a mandatory/digest input. Plan02 adds it explicitly to current fixture constructors and archive prose reconciliation; every original-state constructor and 159–161 original transition builder explicitly omits it. No quote, blockquote or fenced-code exception is added.

Eighteen added native tests cover absence before page creation, six earned Pending/functional/audit/final/archive/next-active page states, and eleven targeted page refusals. Refusals cover premature closure/readiness/archive, functional closure asserting readiness, shipment, production, quoted/blockquote/fenced claims, and stale archived unarchived wording. All use both actual CLIs, filesystem adapters and supplied-map boundaries, with the exact page path and unsupported-claim reason required.

| Receipt boundary                                                        | Actual result                                                                                                           | Identity                                                                                                                                                            |
| ----------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Focused 18 page cases before the later context/shared-control patch     | 18 pass / 0 fail / 146 assertions / 29.19s; 18 owned roots removed                                                      | `/tmp/phase163-page-native-focused.log`; 141 files / 3,436,947 bytes; corpus `79f19ef224d31f5f844e32d8fd70aadcb110cc7887a17c65e93c2bae38a2d95a`                     |
| Fresh combined pure/native after root's control freeze                  | **220 pass / 0 fail / 1,438 assertions / 166.93s** = 85 pure + 135 native; 135 roots removed; all 18 page cases present | `/tmp/phase163-page-combined-native-pure.log`, session44224; 141 files / 3,438,636 bytes; corpus `fe6f14f236286054f9a11375adf5f717f23641d21d108ba5d614a4675fd6711e` |
| Fresh full 159–161 caller suites on that snapshot                       | **1,528 pass / 0 fail / 4,187 assertions / 449.73s**                                                                    | `/tmp/phase163-page-combined-legacy.log`, session66430                                                                                                              |
| Fresh historical 158/162 suites on that snapshot                        | **3,211 pass / 2 fail / 10,744 assertions / 337.48s**                                                                   | `/tmp/phase163-page-combined-historical.log`, session18085                                                                                                          |
| Two new selector overlap controls against the still-unrepaired selector | **0 pass / 2 fail / 5.42s**; both active conflicts accepted; 2 roots removed                                            | `/tmp/phase163-selector-native-red.log`, session38603                                                                                                               |

The combined control freeze used 20-input source digest `63ae29d96f3016f21cd0dc586fe912fd709bf0ca6cbe03e536ca7b626d2fd09a`. The two historical failures are specifically the 162 synthetic bootstrap and implementation positives: their parity owner retains the newly authored dated formal sentence in `rationale`, although their own intended historical status is Pending/implementation. Actual live, formal, page, mutation and native-order cases pass. The core owner owns the narrow intended-state rationale repair; this receipt is not recorded as a green historical suite.

Independent review also reproduced selector relevance inconsistencies: a two-space active milestone directive plus generic requirement heading can hide a contradictory description, and a bare `## Active Milestone: v2.5` can hide duplicate ownership during overlap. Plan02 added two real native/map controls after all broad witnesses finished, captured genuine RED, and froze the native test source at `c2eb4bf83eb6fee43696b6a9507b2a8242e7c48ad930766de073340ce3b2ff11` (137 native tests). The core owner is repairing/refreezing relevance and original fixture fields. Final repaired receipts will be appended below; no earlier run is relabeled.

## Repaired final candidate receipts — 2026-10-12

The core owner repaired relevance to match the parser's supported milestone whitespace/token boundaries and archive-title grammar. The same owner normalized only the synthetic original 162 index/proof/current-document controlled sentences to their intended Pending/implementation/formal states. Assertions, historical delivery artifacts and the six proof roots and original certificates remain intact; no claim scanner exception was added.

The final 20-input source digest was independently read from exported `loadSources()`/`provenanceDigest()` as **`f3e9a2475b512202d89fa2973b9fb690cfc68e1511d9452d95097b30bca5575a`**, with no missing digest input. All runs below use fresh imports on that frozen candidate and the same pinned Bun 1.3.9 prefix. Root's literal full/default verification is separate and is not claimed by these focused results.

| Final command / control                                                                                              | Observed result                                                                              | Receipt                                                      |
| -------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------- | ------------------------------------------------------------ |
| Two native selector controls, after genuine 0/2 RED                                                                  | **2 pass / 0 fail / 24 assertions / 3.58s**; both real CLIs/filesystem/maps; 2 roots removed | `/tmp/phase163-selector-native-green.log`, session22911      |
| Original162 bootstrap/implementation/formal positives                                                                | **3 pass / 0 fail / 4 assertions / 1.037s**                                                  | `/tmp/phase163-original-fixtures-green.log`                  |
| `bun test ./scripts/check-phase163-provenance.test.ts ./scripts/check-phase163-provenance/native.test.ts`            | **226 pass / 0 fail / 1,470 assertions / 168.90s** = 89 pure + 137 native                    | `/tmp/phase163-selector-final-native-pure.log`, session91115 |
| `bun test ./scripts/check-phase158-validated-reorg.test.ts ./scripts/check-phase162-integrated-filter-proof.test.ts` | **3,213 pass / 0 fail / 10,744 assertions / 333.97s**                                        | `/tmp/phase163-selector-final-historical.log`, session93763  |
| Targeted 159–161 original/live/map/claim-transition controls                                                         | **8 pass / 0 fail / 13 assertions / 3.50s**; 1,520 filtered                                  | `/tmp/phase163-selector-final-callers.log`, session97353     |

The caller command used the three explicit 159–161 test paths and `-t 'current scoped evidence runs|supplied-map complete evidence|current RPC roots admit|peer guard permits|final scoped production/action/assertion|peer transition loses|all six current operator|current fixed source/document'`. Root explicitly retained the prior full 1,528/0 caller result at its unchanged caller/current-document snapshot and requested these relevant controls after the narrow core/162-fixture repair. No second full caller run is fabricated.

All **137** final native roots were removed. The final finite corpus is **141 files / 3,441,898 bytes**, SHA256 **`ddd993e3c6463e25f586fd0fc1ca8fb2ed8ca7a5c0bd8e1c32708118ad1d2f65`**. Live control/historical-proof witnesses remain unchanged. The 137 native cases comprise 18 positive tests, 92 targeted refusals, 26 disk-backed map omissions and one cleanup failure control. The passing paths execute 226 actual CLI subprocesses, 226 filesystem checks and 278 map checks, including the four-stage closeout sequence. Both selector conflicts now refuse with the intended path/reason at all six boundaries.

| Final source path                                        | SHA256                                                             |
| -------------------------------------------------------- | ------------------------------------------------------------------ |
| `scripts/check-phase163-provenance/native.test.ts`       | `c2eb4bf83eb6fee43696b6a9507b2a8242e7c48ad930766de073340ce3b2ff11` |
| `scripts/check-phase163-provenance/fixtures.ts`          | `588c41ff2719aec69a53b0799c7596945b697bff7f45162532eca2562c7dea5a` |
| `scripts/check-phase159-filter-rpcs.test.ts`             | `9bfdc8c7a7b927296d57cb03f394412630d042b6a06fd03e943e4b21ce9958f9` |
| `scripts/check-phase160-peer-filter-serving.test.ts`     | `d44b875591976e6e7e959c48c6348ea5c5f291ac2b249d34bf295a60e0d97a65` |
| `scripts/check-phase161-operator-index-evidence.test.ts` | `33449ab72da6b5630349e3b5f47402322feb6576c866c9569069b3ed1a9d3177` |
| `scripts/check-phase163-provenance.ts`                   | `71655a6484ecb57c95bd5913a4f09a30e016ba1ee6d040f4bfe0d71fec963d54` |
| `scripts/check-phase163-provenance.test.ts`              | `921bd1345ff5e534d673b866304f5460e39b7617c75bd953b77c372c661dd7ff` |
| `scripts/check-phase158-validated-reorg.ts`              | `db64d076530cf2c6c4df0be0863bda87935179e8243ed32fef7d55ec9f6fce32` |
| `scripts/check-phase158-validated-reorg.test.ts`         | `b33afce9a639aa86e04a4b04d6388341c5a7541b943e7a13fff14b918a36fa31` |
| `scripts/check-phase162-integrated-filter-proof.test.ts` | `3339fe5353164ad741315a1f29e2cee48a7bcec2e46ca09a2ea32813e9de64a3` |
| `scripts/check-phase162-integrated-filter-proof.ts`      | `74dbf700ac1027d505ed97c62a7a8ecdd50473f3d1a88936b2e3b72ae3909a55` |

The current [REVIEW](163-REVIEW.md) accepts the final bytes at this exact source digest; [SECURITY](163-SECURITY.md) is verified with 15/15 declared mitigations closed, zero open threats and the same 20-input digest. These reports preserve their declared scope and do not stand in for the subsequent full/default native receipt, actual CFGR-01 activation, renewed audit, archive readiness or shipment. No Git/shared-state/full-native mutation was performed by this executor.

## Active-identity and presence repair — 2026-10-12/13 UTC

Subsequent review found that present active controls without a provable milestone identity were silently treated as unrelated during archive overlap. Root reported full2 exit0 on the preceding `f3e9a247…5575a` source snapshot, then authorized the repair; that older verifier receipt does not certify the later source. The exact existing archive-only index remains accepted as affirmative fixed routing: the entire `# Roadmap` document with `v2.5 controls: .planning/milestones/v2.5-ROADMAP.md`, no added control/identity content, and a valid fixed archive. Actual absent controls, equivalent overlap and unique later v2.6 controls remain positive.

Plan02 added 15 native refusal cases covering missing, duplicate, blank/newline, blank-marker-plus-foreign-marker, contradictory primary/fallback, earlier-version identities and wrong/extra/ambiguous archive-index content. RED was **0 pass / 15 fail / 32 assertions / 33.52s** at `/tmp/phase163-active-identity-native-red.log`, session22947. Thirteen controls were silently accepted by the old adapters; two contradictory declaration controls already refused but lacked the newly required active-path/identity diagnostic. All 15 roots were removed. The test source then froze at `cf3e11a16f654b68a39d7fd0746f11b7f432798ef09471049d68f68924bcedd8`.

The core owner implemented line-local, cardinality-aware identity classification and the narrowly accepted exact archive index, preserving later/absent/overlap positives. An intermediate fresh-import combined run passed **273 / 0 / 1,729 assertions / 188.57s** (121 pure + 152 native) at 20-input digest `f87138805bc70557f3bde7acedef9159b5fea86185484f65fa46988ffc6fdc6a`. It removed 152 roots and used 143 files / 3,503,202 bytes, corpus `942d5f2480cb76a3dce3c9ed7dd5bad37547818dea3533ce9dafab466c7f3df5`. Receipt: `/tmp/phase163-active-identity-final-native-pure.log`, session47793. This remains explicitly intermediate.

Fresh adapter review then found the truthy text projection dropped present zero-byte active controls. Plan02 added two actual empty-file tests against valid archive-only layouts: direct pure policy refuses, physical files are verified empty, then both real CLIs/filesystem/map checkers must refuse the same path. Genuine RED was **0 pass / 2 fail / 10 assertions / 5.49s** at `/tmp/phase163-empty-active-native-red.log`, session82603; pure checks refused but actual 158-map accepted. Both roots were removed. The core owner propagated a presence-aware `sourceText(map)` factory through all five 158–162 providers and claim views, retaining `map.has` without filesystem rescue or a sentinel. Child-owned caller tests and fixture helpers were unchanged during this repair.

The final frozen 20-input digest was independently recomputed as **`34443772f20ccfd57013a53d62ffd32f7fbf9e0b8700bb50be4b8ca7e2175ff8`**. One fresh pinned-Bun run of `bun test ./scripts/check-phase163-provenance.test.ts ./scripts/check-phase163-provenance/native.test.ts` passed **275 tests / 0 failures / 1,761 assertions / 185.52s** (121 pure + 154 native). It started 2026-10-13T00:04:26Z and exited0; log `/tmp/phase163-presence-final-native-pure.log`, session36473. All 17 newly added identity/presence cases passed at both real CLI/filesystem/map boundaries. All 154 roots were removed and live control/historical-proof witnesses remained unchanged.

The final input corpus is **143 files / 3,506,326 bytes**, SHA256 **`441419516af5f3996ae05ea92e16505fec70930e179ca69d1eaae49b64786bf9`**. It includes newly present fixed optional report inputs from root's prior verifier activity; constructors still remove/rebuild synthetic stage receipts explicitly. The 154 native cases comprise 18 positive tests, 109 targeted native refusals, 26 disk-backed map omissions and one cleanup failure control. Their passing paths execute 260 actual CLI subprocesses, 260 filesystem checks and 312 checker-map calls, plus the two direct pure empty-input checks.

| Source changed by the last repairs                       | Final SHA256                                                       |
| -------------------------------------------------------- | ------------------------------------------------------------------ |
| `scripts/check-phase163-provenance/native.test.ts`       | `b85d81ee18eba16cc0a5e058c1cf34c7682d67b677a2d0e58e8d84f8062f28f8` |
| `scripts/check-phase163-provenance.ts`                   | `cc8d36584e7d0456eb2a6969fa0ce3ee633a79dc1e6a0d45ddfd1c4897e05ad1` |
| `scripts/check-phase163-provenance.test.ts`              | `3ba274c9124e071e1c1d384f3527d04285f84b4090cb831ce55051dee56103b5` |
| `scripts/check-phase158-validated-reorg.ts`              | `1df027aca2deb9827f135d9a558ef2377d7a2ebb18b830ba76f41698376834de` |
| `scripts/check-phase158-validated-reorg.test.ts`         | `28c5ed07dfb670d439c4859dd8f93ff7f7b2e1624d9b58ba2b9106319778965a` |
| `scripts/check-phase159-filter-rpcs.ts`                  | `563ec1eb3694e9c3961f705f250596ea68292fc67158f8c85375f07f08184a32` |
| `scripts/check-phase160-peer-filter-serving.ts`          | `41f9054550790bf2f0263cedb4f0dd74c964a29a6fc4b8a492b1f17fa6cdbc05` |
| `scripts/check-phase161-operator-index-evidence.ts`      | `c3447ae4c4cc07c8a9d2de712b663520aab545f990730ec77e3e93ca7e93ce23` |
| `scripts/check-phase162-integrated-filter-proof.ts`      | `912df3e04b070833778cfbf03aaaf7dfefe20f107c3497e81d2ed4f5521867a3` |
| `scripts/check-phase162-integrated-filter-proof.test.ts` | `97e69067ea0a291917465d6379c6c4adaba4da4919c00611c27aac82a7fe119c` |

The fixture helper remains `588c41ff2719aec69a53b0799c7596945b697bff7f45162532eca2562c7dea5a`; child-owned 159–161 test files retain their previously recorded page-follow-up hashes. Core-owned provider/validator changes, rebind and compatibility receipts are recorded by the Plan01 owner. Earlier historical/caller suites, failed 3211/2 run, selector REDs, f871 intermediate run and full2 are not relabeled as executions of this final source. Fresh independent acceptance and root's full3/default verifier remain separate gates; this executor ran no Git/shared-control/publication/full-default action.
