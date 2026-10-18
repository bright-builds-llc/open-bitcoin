---
phase: 165-shared-exact-work-authority
plan: "37"
subsystem: testing
tags: [bun, typescript, durability, authorization, source-guards, provenance]
requires:
  - phase: 165-36
    provides: Accepted current source guards, 168 ownership rows and 287-input inventory
provides:
  - Requested-body persistence and actual active-tip source evidence
  - Capability acquisition and consumption evidence for both connected-block seams
  - Independently reviewed 173-source ownership table and 292-input ordered pin
  - Accepted complete 5096-entry raw generation and eighteen focused commands
affects: [165-38, 165-39, phase165-final14]
tech-stack:
  added: []
  patterns: [Pure owning-function source predicates, independent changed-anchor regressions, source review before ownership activation]
key-files:
  created:
    - scripts/check-phase128-production-compact-announcement-transport/durable-trigger.ts
    - .planning/phases/165-shared-exact-work-authority/165-37-SUMMARY.md
  modified:
    - scripts/check-phase128-production-compact-announcement-transport.ts
    - scripts/check-phase128-production-compact-announcement-transport.test.ts
    - scripts/check-phase134-apply-boundaries/aggregate-roots.ts
    - scripts/check-phase134-authoritative-lifecycle.test/apply-helpers.ts
    - scripts/check-phase164-canonical-identity/test-modules.ts
    - scripts/check-phase165-shared-exact-work.ts
key-decisions:
  - Preserve requested pending and side bodies before disposition and prove both storage and notification failure exits.
  - Require the actual active-tip helper under the same ordinary runtime implementation, including live chain-tip hash equality.
  - Bind acquired authorization to authorized chainstate preparation before existing lifecycle sealing and transaction steps.
  - Accept complete source bodies before five ordinary ownership appends and the actual ordered inventory pin.
patterns-established:
  - Independent review findings receive real changed-anchor RED evidence before a narrow same-owner repair.
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 165-2026-10-14T16-09-59Z
generated_at: "2026-10-17T23:28:22Z"
duration: 24min
completed: 2026-10-17
---

# Phase 165 Plan 37: Durable Triggers and Authorized Connect Seams Summary

**Current guards prove requested-body persistence, actual active-tip identity and consumed connect authorization, with 404 full-suite passes and independently accepted ownership activation.**

## Performance and scope

- Authentic preparation started at 2026-10-17T23:04:35.156Z; this record was prepared at 2026-10-17T23:28:22Z.
- Two tasks, seven declared source owners including one new pure helper; no production Rust or Phase129 source edit.
- Final full suites: Phase128 99 passes, Phase134 293 passes and Phase129 12 passes; four associated CLIs exit 0.
- Final eighteen-command child time totals 39.974844 seconds, excluding complete generation checks around every command.
- Root owns metadata, independent acceptance, external verification tooling and all Git. This executor changed only the seven declared owners and this summary.

Repo-local AGENTS guidance, the Bright Builds sidecar, architecture/code-shape/testing/verification/TypeScript standards and both complete active lesson inputs informed execution. No applicable override or project skill was required. Existing scanners, fixtures and parameterized tests preserve pure predicates and readable physical limits.

## Task 1: Requested bodies, active-tip identity and authorization

The Phase128 checker imports the extracted pure `hasDurableSaveBeforeTip` predicate. All other checker bytes remain exact apart from that approved extraction and its import/dependency relocation. The ordinary owning implementation and production function must persist every requested body before matching disposition, preserving side and pending bodies. Balanced save-error inspection requires the BASIC failure notification and both terminating error paths before local-hash or accepted-progress effects. The successful continuation requires local-hash recording, requested Connected disposition, accepted progress, actual active-tip qualification and queued announcement, preserving duplicate/disconnected/nonextending handling.

Independent review found that the first GREEN predicate proved only the active-tip call name. Three real mutants changed the actual helper to constant true, inverted hash equality or any-present-tip credit; each was incorrectly accepted. All three tests first failed at their intended expected-finding assertion after a passing baseline and a unique actual changed anchor. The narrow repair requires the production active-tip function under the same ordinary Runtime implementation as the disposition function. Its complete executable body derives the candidate header hash, propagates the live chain-tip query error and compares the actual active-tip hash. Additional negatives cover ignored lookup errors, test-only helpers, unrelated implementation owners and comment-only decoys.

Fifteen independent current-body negatives supplement all eighty-four previous Phase128 cases. They cover Connected-only or best-header-gated persistence, absent local hashes, unrequested or non-Connected announcements, absent/substituted tip qualification, BASIC notification fallthrough and the seven helper controls. The old conditional-save mutation now targets the actual sixteen-space local-hash continuation; all prior transport, write/acknowledgment, ordinary-owner, attribute and decoy cases remain registered.

Phase134 changes only the two connected preparation selectors and `inspectConnectedBlockSeam`. Both seams must acquire executable `self.authorize_connect(block, consensus_params)`, consume the same capability once through the exact chainstate receiver and authorized preparation arguments, then retain lifecycle preparation, sealing and the existing transaction order. Actual direct/stored timestamp and lifecycle-context wiring remains covered by the predicate. Existing aggregate roots, callback/persistence handling, transitive effects, scanner/discovery and post-seal checks remain exact outside that narrow seam change.

The existing prepared-chainstate mutant now changes the actual authorized preparation call. Fourteen new parameterized mutations cover both seams' absent acquisition, wrong acquisition receiver/block, substituted capability/preparation receiver, authorization after preparation and effects before transaction. Required function-scoped replacements reject duplicate or unchanged anchors. All 279 previous Phase134 cases remain; Phase129's twelve existing integration cases pass through its unchanged imports.

## Actual test-first evidence

All artifacts with a `phase165-plan37-` basename are under `.local/open-bitcoin-dev/`. Initial two CLI failures and two positive test failures establish real source-policy RED before predicate edits; import, syntax and absent-anchor failures are not counted. Earlier successful generations and all twenty-three Task1 command logs remain immutable.

| Actual run                   | Cases / assertions              | Exit / seconds | Full log SHA256                                                    |
| ---------------------------- | ------------------------------- | -------------- | ------------------------------------------------------------------ |
| Initial128 CLI               | Durable-trigger diagnostic      | 1 / 0.105      | `0111772dabe88e55aa16f6dc7431dfe5aebdb1b593a1e7270a89bc83db70094c` |
| Initial134-apply CLI         | Apply-boundary diagnostic       | 1 / 0.133      | `3beb4fd4f0e1b9ab2c6eac343c6037e387464af30e3073398d85448350ba32cc` |
| Initial128 positive RED      | 0 pass / 1 fail / 1 assertion   | 1 / 0.040      | `68fa07a60d4d45586767576eb45602347fbc7f76352f318e6b5223da4ad71ac7` |
| Initial134 positive RED      | 0 pass / 1 fail / 1 assertion   | 1 / 0.228      | `d880d5e9965afb1e3e8ec6e47a8c357471051d1b374f80c42b6c5a4f05c76c80` |
| First full128 GREEN          | 92 pass / 167 assertions        | 0 / 1.642      | `9c7530d8ae28db7f2c31aa38a3aecb63a77ac8d62b0dd489734d1ce89abde00e` |
| First full134 GREEN          | 293 pass / 438 assertions       | 0 / 15.574     | `23a0369958e3b3e7cc5c3d262ac6918e501e0354fb12d1eefa75f5ce5f07e30f` |
| First full129 GREEN          | 12 pass / 12 assertions         | 0 / 1.127      | `18ff7afdcb4d3b1fe3537de9afdea45690f64a7ddceafc271ae2b0091052d49f` |
| Actual active-tip helper RED | 0 pass / 3 fail / 12 assertions | 1 / 0.100      | `b5c8953695134fde9878ef3c9db02e5737474ad67cd3ebb389006531b4882a37` |
| Final full128 GREEN          | 99 pass / 195 assertions        | 0 / 2.068      | `4a1eef63325f38e407787f9c66c0aadc929b7bc581ac2827a294bc7c4ff9a1e1` |
| Final full134 GREEN          | 293 pass / 438 assertions       | 0 / 17.151     | `907db8b7aa09103b0488bad4139590d6fa3de6b3d7432954707c41c9418fe355` |
| Final full129 GREEN          | 12 pass / 12 assertions         | 0 / 1.360      | `9db276dfc8ab3451b098115f0113537c629d7a4f5edc995add8588f955a84c63` |

The four final Task1 CLI exits are 0 for128,134-apply,134-authoritative and129. Their command receipts and full logs are authenticated in the complete final source packet. All three full final roots execute without skips or ignored cases; their total is404 passes /645 assertions.

## Complete source acceptance before identities

Before packet `phase165-plan37-before.json`, SHA256 `59815295052425c09f52a05e545d0bcd0b814a7edb5778a088268e25b74444ef`, authenticates four complete original bodies against the unchanged344-body baseline `a6f23987c5d1ea36909f8f49866f17bd086199014509171d63524c4252e229fd`, the new helper's explicit absence, three held activation dependencies and two read-only Rust dependencies. The Rust block-response and mempool-lifecycle bodies retain respective full hashes `718f8eb2fa0e552cb26fd43e62cd92ade6145973fc96b957885d54121060d11c` and `fa186eeafab8925c40006f7a20671c265af261a364698712b751562b6baa8bbb`.

Final packet `phase165-plan37-task1-final-source-review.json`, SHA256 `bba3bdccc613443e457cbb055b3ac2c2e24887d1a3307f7e112c86913cd884b4`, contains all five complete bodies, four complete raw-gated inverses, new-file absence inverse, old and final generations' actual receipts/logs and unchanged held/Rust bodies. The final diff SHA256 is `517fd8ed637e10ab074ac424c5c0196fed6ffe0f11f77bb98fd004035e9c4066`. The pre-feedback packet remains separately preserved at `1333814d503a581deeef80242296c47898ded13fa3c2969127706c1174b71851`.

| Accepted Task1 owner                                                                  | Lines | Full raw SHA256 and measured sourceIdentity                        |
| ------------------------------------------------------------------------------------- | ----- | ------------------------------------------------------------------ |
| `scripts/check-phase128-production-compact-announcement-transport.ts`                 | 566   | `43d202575c3ed366901a03861f05fb2790ed2b8cdece0fea7e55384bf33341e7` |
| `scripts/check-phase128-production-compact-announcement-transport/durable-trigger.ts` | 81    | `90a131b2707648385d5fc15e329a777bb1bb65619062bcce5b14e6bdab2793a5` |
| `scripts/check-phase128-production-compact-announcement-transport.test.ts`            | 447   | `4ebfcc735f6e9f08b24597709caf85aa27a52ef443b201c15e3cfd03ab6636cc` |
| `scripts/check-phase134-apply-boundaries/aggregate-roots.ts`                          | 357   | `4e8b12f6dda81dd430a4e337d66efb727a256ae6452e40492607ad182f7be9d0` |
| `scripts/check-phase134-authoritative-lifecycle.test/apply-helpers.ts`                | 604   | `74f589b1c048da3363f36ae439910ee59d596670ffff91ef5371011b8ef49ea2` |

Independent CLEAN review `165-PRE-REVIEW-INTEGRATION37.md` has SHA256 `829e02fa25f2a6a0c1305e93680854757135a81f1316c378380b6941581a81c9`. Root's source acceptance, SHA256 `d670ba8e62c303a8b862bfa6bf077ac047aca0f8072c2f815fe872edf7fc7806`, follows independent full-body/inverse/log and carried5018-entry delta review. It precedes the first sourceIdentity derivation and every row/pin change. All five source bodies remain exact afterward.

## Task 2: Five ordinary rows and actual pin refusal

The inert proposal SHA256 is `5f15f54fbc0206d19eb3462278a27a68149e348986e7dfa35b6da39fa03e3525`. Its actual fresh isolated imports retain the fourteen-module CLI graph, all168 prior SOURCE rows and order, the143 SOURCE_FILES /606 EDGES /26 TEST_MODULES /88 TEST_OWNERS arrays, all non-SOURCE contract exports, original388-line host prefix and previous input relative order. Five ordinary ownership rows append in Task1 declaration order, with no renewal or fabricated Rust-test row. Actual imported173 SOURCE /88 TEST /292 INPUT produce ordered JSON pin `c800936dec345110760bf56f4862682ca8ebc1a6f1112383b898383d225116a3`.

Root independently accepted both complete candidates and inverses, actual ordered arrays/pin and ten append/disabled loaded-owner controls before activation: approval SHA256 `20da11267e0123608c1cd6348b935a66fe055f8ea3935fe30296950202573e1e`. Those controls each produce exactly the intended ordinary ownership finding without executing mutated Rust or modules.

Only the approved host candidate was applied first. The real current165 CLI with old `cda7b11af8aa9996d2c84226fc8af2e2cbd963ac7e20d9112b42a8ada70a4a5a` pin exited1 in0.103 seconds, reporting fixed inventory refusal before bounded loader reads. Module import activity is separately attributed. Its full log SHA256 is `23c1e64fed4753c2397625462c6823bedb6e76f5cd34fec20537cd874304a2da`. Only then was the approved entry applied, changing solely its inventory constant.

The host is567 lines /47,063 bytes /SHA256 `4ced68e96f261bdb3e29f19f451dd22785587665d20686614873818c5aa3bcf4`; its full inverse restores `4428312183d12e6ce73cae507b4a6b546c39106a70552a16da040d57344a863a`. The entry is269 lines /14,766 bytes /SHA256 `b8ffdd7d25b10fd251b92e18af5eb99d3358b4d4b98519d83b36a1f59eb5b325`; its inverse restores `92c545cb37f04967349a085517739872f93bc5e2499b23918a88929f37e851c7`. Current contracts remain exact at `c6bbdb06f393d41f00fe15b52f81026802aa9a5bc2bdd4345ab91b2f8f753ef4`. All historical identities, manifests, snapshots, receipts and adapters remain unchanged.

The fresh post-pin audit actually loads292 inputs, passes the pure baseline and all ten owner controls; its full deterministic report hashes to `4a10f08d5b5270539e1b23f4526c4e949f2f1536df62a0011560ae7f1a7b25a9`. The driver later executes a separate fresh ten-control proof into a distinct output path. Actual activation result and complete two-file inverses are recorded at `9c1da03ae42d2118dbc81162c6365861cf239822a0b82dce9e4b6048fb0bab44`.

## Actual read closure and final commands

Fresh read-union receipt `b4f7ca4a7af6c29bc5566a62831f55351c0ea2f7fa0f28ca08d37a9b4fb36608` identifies81 static modules,14 copied CLI modules,340 known scripts and978 raw observations. Its123 additional roots and533 descendants explicitly include the real new128 helper and affected128/129/134 dependencies. The complete supplement `a3d224cb4311bec7eb4d16dc1c7f27057ab9518b46300b3b702050999e75ed1c` adds two archive fallbacks, already present among the carried5018 entries, to reach980 observations. All seven actual non-test daemon helper members are checked before and after every command. These finite repository/provenance observations do not claim OS or compiler closure.

The current factory contains171 files and4,604,749 bytes, digest `618d712831b24bee3ffe66d54ce9cadbbf0c5e816b4436e28f1232d731d782b2`. No historical corpus or identity is renewed. Reviewed external tools have no open findings. Freeze `phase165-plan37-freeze.json`, SHA256 `e184b53a366cf081121e61ba7a3924acd92008c22220fbb68db1bf22425c1e4b`, contains5096 complete canonical presence/bytes/full-SHA states, preserving all5018 accepted predecessor paths and both explicit absences. Exactly nine prior states have approved deltas: four existing Task1 owners, two activation owners and Root's three shared metadata files; the new helper is an added path.

Root freeze acceptance SHA256 is `29e6564989caaef786c7b85cdab1959bb76a1638a506569654d0ee143680cb07`. The external accepted freeze SHA is checked before parsing, and every complete raw state plus dynamic helper membership is checked before and after each of eighteen sequential commands with pinned Bun1.3.9. Every check reports zero mismatches.

| Final command                  | Exit / seconds | Full actual log SHA256                                             |
| ------------------------------ | -------------- | ------------------------------------------------------------------ |
| Phase73 CLI                    | 0 /0.266975    | `8c5c1fde6293da87a10a568ee170403d295faab4b8219466551b27a0882ae8e2` |
| Phase75 CLI                    | 0 /0.442932    | `10c2cc457946806ac156d1546a4b33af75d0aa22d9fc745b49505ec20e4d0589` |
| Phase123 CLI                   | 0 /0.040327    | `56848629a16445393539e060264710c6ff5546333657c07909c2054db8967e1b` |
| Phase127 CLI                   | 0 /0.755430    | `9e20d1087131c5e1e95ce213e9744194a5456654f9015e965faf96d1b9ab0819` |
| Phase128 CLI                   | 0 /0.034915    | `850a0ebe54f2445eec538f2fe66ef40987379a0f8d1f00ac346b73a77001dd17` |
| Phase129 CLI                   | 0 /0.785734    | `f7740916e2d66e4b0e854a8f0ae49c9c4ad27788f949353ceca97576710c6235` |
| Phase134-apply CLI             | 0 /0.146506    | `5f400593c56e20bcc8f3ac6417fee2a4cf8c1ba4bca4d444e9255f02061c3b2a` |
| Unchanged134-authoritative CLI | 0 /0.042324    | `921a79fcf94dec8130088f50924c9a8b47f63025c1552fa89b6e1e271a4c5d56` |
| Fresh ten-owner proof          | 0 /5.954349    | `6a0e6c1bc0c9a774058eb3f121ea0b6190eafaf0dd218b1925b8adbfb50a9fe7` |
| Affected165 selector           | 0 /26.982484   | `e491980a09555876087e9586799ec76be2a115bba9a74a2caaf9483400fbdefd` |
| Live164 CLI                    | 0 /0.591006    | `0d78b33a529f49e707acf1017c7d05661ff87947635206b6eab5729c657ceb5e` |
| Live165 CLI                    | 0 /0.583434    | `28d122938ad64b5371c771f8bc72617d7e31df25e6b1d28e082ec766ba8e6ef2` |
| Fast158 CLI                    | 0 /1.876752    | `767b6db464f7d582a64e48de5e0dcc876f62dd29338976b5de1af04e4d6152dc` |
| Fast162 CLI                    | 0 /1.002647    | `10cf94bc32c825073ad4c98764e738cd6865d709876e2323591ea90252a2df53` |
| Parity breadcrumbs             | 0 /0.208176    | `f98277765cf0be0f6892990cd622d341dfa5bd2966263ec6a0df64a2606cb73f` |
| All managed checks             | 0 /0.166574    | `f6020fc944ea1440798b82442c8121b662350d28252e146d89a4aaa45caf0f9b` |
| Physical line bounds           | 0 /0.003886    | `2445ad32339e56eba8349d922e7badb3ab3f69f02cd71fc8852d277ac52749d3` |
| Whitespace diff                | 0 /0.090393    | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |

The affected selector executes178 cases /490 assertions with zero failures and1788 intentionally filtered cases from the unchanged1966-case root. No skip, ignore or todo is recorded. Actual family subsets are baseline1, ordered inventory1, copied fresh CLI14, exact inverse4, historical child ancestry4 and fixed loader4. All actual names, commands, exits, durations and raw log hashes are in `phase165-plan37-results.json`, SHA256 `7a2f4ddda302a5c467e427bc158691e8f092d0bf419052ef171bd33a37bbc92b`; the eighteen-log close hashes to `f6a5df03f01083c4db5f5873df412f360b6809d7fffbfdf7998056f889366285`.

Complete closing proof `eb0a4de28c9742afa8ac7a4950ca2893734a4fa7afa4538d828edd1a101f8551` independently re-observes all5096 states, with5094 regular files and2 explicit absences, and authenticates all eighteen full logs. Root independently accepted the same complete closure, actual178 names/six families and ten fresh owner controls in `phase165-plan37-root-command-acceptance.json`, SHA256 `d3310d6d13eec0c2cb69774004858c7c3820f32bf5523d93efe253fef56ee3c2`.

## Deviations, threats and simplification

One Rule1 source-evidence bug was fixed during Task1 independent review: the initial predicate accepted weakened active-tip helper implementations. Three genuine negative RED cases, seven helper controls and the final complete suites demonstrate its repair within the same declared owners. No additional architectural change or scope warning applies to37. One focused pure extraction and one narrow existing-seam adaptation avoid a new parser, dependency or production change.

T-165-37-01 is covered by current requested-body/failure isolation and actual active-tip evidence;02 by acquired/consumed authorization and both-seam ordering/effect negatives;03 by real changed anchors and complete old/new test discovery;04 by independent source-before-identities acceptance, exact ordinary rows/inverses/refusal/controls;05 by readable caps, complete raw/log closure and bounded claims. No new trust-boundary surface or threat flag was found. Existing source fixture strings are intentional test evidence; no implementation stub was introduced.

## Task commits and remaining ownership

Tasks1–2 are complete. Per the execution contract, no nested commit, push, hook change or shared metadata edit was performed. All Git and planning metadata remain Root-owned and deferred to strict phase-wide verification and normal-hook finalization. No authentication gate or user setup occurred.

Requirements-completed remains empty. Seven repaired source CLIs73/75/123/127/128/129/134-apply are green; unchanged134-authoritative remains green. Known filter155/156/157/159/161 failures await Plans38–39. Both original full-native failures retain their original failed outcomes; no partial run is presented as full native acceptance.

Final14 retains its original Task XML/commands and the entire fresh default native verifier, complete164/165 matrices, Rust/benchmark/fresh coverage/Bazel, whole-phase code/security/formal/lifecycle, generated LOC and normal-hook strict-main commit/push obligations. None of those later acceptances is claimed here. CWOR-04 remains Phase166-owned; stop before166.

## Self-Check: PASSED

All seven declared source owners, authentic before/source/activation/closure evidence, independent review and this summary exist. Complete source and activation inverses recover their authenticated predecessors; the new helper has an explicit original absence. All twenty-three Task1 logs and eighteen final-driver logs retain their recorded full raw hashes. The final source and activation bodies match the accepted generation.
