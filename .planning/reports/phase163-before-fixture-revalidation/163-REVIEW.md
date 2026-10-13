---
phase: 163-archive-safe-parity-provenance-and-milestone-handoff
generated_by: gsd-code-reviewer
lifecycle_mode: yolo
phase_lifecycle_id: 163-2026-10-12T21-09-41
generated_at: "2026-10-13T02:43:21Z"
reviewed: "2026-10-13T02:43:21Z"
depth: standard
diff_base: d47f6e09b4621808974cc083fcc27ee9d2c1b4c7
source_digest: 34443772f20ccfd57013a53d62ffd32f7fbf9e0b8700bb50be4b8ca7e2175ff8
files_reviewed: 21
files_reviewed_list:
  - scripts/check-phase163-provenance.ts
  - scripts/check-phase163-provenance.test.ts
  - scripts/check-phase158-validated-reorg.ts
  - scripts/check-phase158-validated-reorg.test.ts
  - scripts/check-phase162-integrated-filter-proof.ts
  - scripts/check-phase162-integrated-filter-proof.test.ts
  - scripts/check-phase159-filter-rpcs.ts
  - scripts/check-phase159-filter-rpcs.test.ts
  - scripts/check-phase160-peer-filter-serving.ts
  - scripts/check-phase160-peer-filter-serving.test.ts
  - scripts/check-phase161-operator-index-evidence.ts
  - scripts/check-phase161-operator-index-evidence.test.ts
  - scripts/check-phase163-provenance/native.test.ts
  - scripts/check-phase163-provenance/fixtures.ts
  - scripts/check-phase162-integrated-filter-proof/contracts.ts
  - scripts/verify.sh
  - README.md
  - packages/README.md
  - docs/parity/index.json
  - docs/parity/v2-5-archive-safe-provenance.md
  - docs/metrics/lines-of-code.md
prior_reviewed: "2026-10-13T00:09:51Z"
prior_files_reviewed: 42
prior_files_reviewed_list:
  - scripts/check-phase163-provenance.ts
  - scripts/check-phase163-provenance.test.ts
  - scripts/check-phase158-validated-reorg.ts
  - scripts/check-phase158-validated-reorg.test.ts
  - scripts/check-phase162-integrated-filter-proof.ts
  - scripts/check-phase162-integrated-filter-proof.test.ts
  - scripts/check-phase159-filter-rpcs.ts
  - scripts/check-phase159-filter-rpcs.test.ts
  - scripts/check-phase160-peer-filter-serving.ts
  - scripts/check-phase160-peer-filter-serving.test.ts
  - scripts/check-phase161-operator-index-evidence.ts
  - scripts/check-phase161-operator-index-evidence.test.ts
  - scripts/check-phase163-provenance/native.test.ts
  - scripts/check-phase163-provenance/fixtures.ts
  - scripts/check-phase162-integrated-filter-proof/contracts.ts
  - scripts/verify.sh
  - README.md
  - packages/README.md
  - docs/parity/index.json
  - docs/parity/v2-5-archive-safe-provenance.md
  - .planning/PROJECT.md
  - .planning/REQUIREMENTS.md
  - .planning/ROADMAP.md
  - .planning/phases/162-real-prune-retention-and-integrated-parity-proof/162-UAT.md
  - .planning/debug/phase156-automatic-writers.md
  - .planning/debug/bright-builds-file-lengths-ci.md
  - .planning/phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-CONTEXT.md
  - .planning/phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-RESEARCH.md
  - .planning/phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-GAP-SCOPE.md
  - .planning/phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-CHECKER-REVIEW.md
  - .planning/phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-CHECKER-REBIND.md
  - .planning/phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-TRANSITION-EVIDENCE.md
  - .planning/phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-HANDOFF.md
  - .planning/phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-ORIGINAL-MILESTONE-AUDIT.md
  - .planning/phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-ORIGINAL-INTEGRATION-AUDIT.md
  - .planning/phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-01-PLAN.md
  - .planning/phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-02-PLAN.md
  - .planning/phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-03-PLAN.md
  - .planning/phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-04-PLAN.md
  - .planning/phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-01-SUMMARY.md
  - .planning/phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-02-SUMMARY.md
  - .planning/phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-03-SUMMARY.md
findings:
  critical: 0
  warning: 0
  info: 0
  total: 0
resolved_during_review:
  warning: 3
  info: 1
status: clean
byte_rebind_acceptance: accepted
review_scope: readiness-publication-phase-finalization-pending
phase_finalization_at_review: pending
---

# Phase 163: Code Review Report

**Current acceptance:** 2026-10-13T02:43:21Z; standard depth; 21 explicit source/publication files; clean, zero current findings. The source digest and resolved source findings remain accepted. The actual renewed audit authorizes current readiness; final whole-phase verification, canonical 04/05 completion and global counter reconciliation remain pending. See the dated Wave 5 acceptance appended below. The preceding frontmatter now records this current scope and preserves the earlier 42-file scope separately.

## Historical source rebind assessment | 2026-10-13T00:09:51Z

**Reviewed:** 2026-10-13T00:09:51Z
**Depth:** standard
**Files reviewed:** 42
**Status:** clean — renewed acceptance of the repaired source

## Summary and limits

This renewed independent source review accepts only source digest `34443772f20ccfd57013a53d62ffd32f7fbf9e0b8700bb50be4b8ca7e2175ff8` and the dated document inventory below. There are no current open findings. Three warnings and one documentation item were repaired by their owners and independently rechecked.

The earlier `f3e9a247…` clean acceptance at 2026-10-12T22:58:18Z was withdrawn at 23:55:37Z after WR-03 exposed unclassifiable active controls borrowing archive authority. The intermediate `f8713880…` classifier candidate was not accepted because the shared text adapter still dropped present empty strings. Both causes are now resolved. Earlier scoped review and execution receipts retain their own source boundaries; none is silently relabeled as covering current bytes.

Current exact byte rebind is accepted, including the strict classifier, presence-aware adapters, transitive helper, all 20 binding inputs, native fixtures and existing contracts. Clean source review does not establish a completed default native run on these repaired bytes, security certification, canonical functional closure, renewed audit, final phase completion, archival, tag, shipment, production capability or funds safety. Root's full3/default verification and later publication/Git gates remain separate. No future or running result is credited as passed.

Material guidance: repo-local AGENTS/Repo-Local Guidance, Bright Builds sidecar, placeholder overrides, architecture, code-shape, verification, testing, local-guidance and TypeScript standards; current CONTEXT, RESEARCH, all four plans, Plans 01–03 summaries, scoped review/rebind, transition evidence and handoff. Both active lesson files were read completely: 5,230 + 1,958 bytes, 1,744 + 653 conservative estimated tokens. No lesson archives or unrelated managed/history changes were reviewed. No project skills were present. The two named ignored debug handoffs were inspected only because this task explicitly scoped them. The reviewer changed only this report.

## Finding resolved during this renewed review

### WR-03: Unclassifiable and empty active controls borrowed archived authority

**Files:** `/Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase163-provenance.ts:87` (`activeControlDisposition`), `:154` (`select`), and `/Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase158-validated-reorg.ts:37` / `:135` (`sourceText`, `resolveTextProvenance`).

**Issue:** The selector treats failure to identify v2.5 as proof that an active document belongs to an unrelated later milestone. With valid fixed archives present, active REQUIREMENTS or ROADMAP text with no usable identity, a blank/newline identity, or duplicate/contradictory foreign markers can be silently ignored. The pure policy and both checker maps can then accept with no failures. The intended exception requires affirmative unique later-milestone identity; unknown is not unrelated. The cooperating native matrix captured 15 genuine RED cases: 13 wrongly accepted, two refused without the required active-path diagnostic.

**Fix:** Classify active candidates explicitly as target v2.5, uniquely identified later milestone, or invalid; reject present invalid documents with their fixed active path. Count marker declarations before parsing values, constrain values to the same physical line, reject duplicate/contradictory markers, and allow the archive-only roadmap index only as the exact whole document pointing to its existing fixed archive. Preserve absent-active, equivalent-overlap and affirmative later-milestone positives. Rebind the helper after implementation, rerun pure and actual CLI/filesystem/map regressions, recompute the sole 20-input digest and obtain renewed independent review before clean publication.

**Independent repaired-candidate probe:** The `f87138805bc70557f3bde7acedef9159b5fea86185484f65fa46988ffc6fdc6a` pure classifier refuses present zero-byte REQUIREMENTS/ROADMAP inputs correctly, but both checker maps still accept them beside valid archives. `scripts/check-phase158-validated-reorg.ts:130` rebuilds the policy map with a truthy text filter, silently dropping present empty strings. Presence must be preserved through the shared adapter, with native/filesystem/map empty-file refusals and absent-file positives. This was part of WR-03 and prevented acceptance of `f871…`.

**Current repair and independent resolution:** The classifier now returns explicit validate/use-archive/invalid dispositions. It counts primary and secondary marker declarations before extracting same-line values, rejects empty/malformed/duplicate/contradictory identity, accepts only canonical safe integer milestone versions, and ignores a uniquely identified milestone only when it is later than v2.5. A roadmap with no declared milestone routes to the archive solely when it equals the exact fixed whole-document archive index; a missing fixed archive still refuses.

The shared `sourceText(ReadonlyMap)` factory preserves the original string-reader API and carries a typed optional `maybeHasSource` callback using `map.has`. All five actual checker adapters use the same provider for initial state, repeated claim views and excluded-claim resolution. Present empty entries therefore remain present and invalid; absent entries remain absent. Legacy string-only callbacks remain compatible and explicitly cannot express this distinction; actual map/filesystem adapters use the presence-aware provider. No content sentinel or filesystem fallback is introduced.

Independent checks on the final frozen candidate pass: 121 pure cases; 21 real CLI/filesystem/map tests covering all 15 identity refusals, both empty-file refusals, equivalent overlap, archive-only, next-active and the sequential closeout; 11 original-state/live/empty compatibility tests; and 24 explicit pure/map refusal-and-positive assertions from a separate probe. All 21 owned native roots were removed. WR-03 is resolved on the current digest only.

## Findings previously resolved during this review

### WR-01: Literal relevance filters silently ignored conflicting active v2.5 controls

**Files:** `/Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase163-provenance.ts:222-223`.

**Issue:** With valid archived copies present, an active roadmap heading ending exactly at `## Active Milestone: v2.5` was ignored because the relevance regex required a following literal space. A conflicting active owner then escaped candidate comparison. An active requirements declaration with two spaces after `**Milestone:**` and an ordinary `## Current Requirements` section likewise escaped comparison despite a conflicting description. Both independent supplied-map probes returned accepted publication with no failures on the previous `63ae29…` snapshot.

**Fix and resolution:** Align relevance with the existing parsers' whitespace and milestone-token rules. Current regexes recognize these v2.5 declarations, preserve unrelated later-milestone selection and compare both candidates. Four pure equivalent/conflicting controls and two real CLI/filesystem/map refusal controls were added. Independent reruns of the original two reproductions now refuse specifically with `conflicting v2.5 candidates` and both paths. The full current pure suite and both new six-boundary native tests pass. No broad archive exception was added.

### WR-02: Original Phase 162 fixtures inherited live formal-completion prose

**File:** `/Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase162-integrated-filter-proof.test.ts:19-28` and the original-state builders.

**Issue:** The full historical run completed with 3,211 passes and two failures in `genuine Pending bootstrap uses current executable roots without future proof` and `hermetic implementation proof passes without future reports`. Their maps reset requirement/owner state but inherited the newly dated PHASE162_COMPLETE sentence from the live parity rationale. That made valid original-state fixtures refuse through the expected exclusion scan; the failure was test reliability, not loss of historical runtime proof.

**Fix and resolution:** A small test-only normalizer reconciles only the four explicit current document/proof inputs to the intended bootstrap, implementation or formal sentence. Both builders call it; original mutation assertions, source bytes and runtime guards remain intact. Independent bootstrap/implementation/formal positives now pass: three tests, four assertions, 1.053s. The failed full matrix remains a failed prior-snapshot receipt; no later full-matrix success is invented here.

### IN-01: Roadmap progress contradicted completed implementation summaries

**File:** `/Users/peterryszkiewicz/Repos/open-bitcoin/.planning/ROADMAP.md:323`.

**Issue and fix:** The progress row still reported `0/0 / Not started` while the approved phase had four plans and Plans 01–03 were complete. Root corrected only that row to `3/4 / In Progress`. Independent inspection confirms the correction, with CFGR-01 still Pending and Phase 163 completion unasserted. This publication-only change does not alter the 20-input digest.

## Independent current-byte rebind acceptance

The following original hashes were independently recomputed from exact parent `d47f6e09b4621808974cc083fcc27ee9d2c1b4c7`; current hashes were recomputed from raw working bytes and matched `checkFrozenGuards`:

| Bound input                 | Original SHA256                                                    | Accepted current SHA256                                            |
| --------------------------- | ------------------------------------------------------------------ | ------------------------------------------------------------------ |
| Phase 158 checker           | `b2339c02beebd971432bd53dfc81df5f563a41968b5181ff66112045748169e5` | `1df027aca2deb9827f135d9a558ef2377d7a2ebb18b830ba76f41698376834de` |
| Phase 158 tests             | `68e444d11ae5a3203c6a51c069a00a496db6dd99bbf191c724c2ca496f1855ec` | `28c5ed07dfb670d439c4859dd8f93ff7f7b2e1624d9b58ba2b9106319778965a` |
| Phase 163 transitive helper | New source                                                         | `cc8d36584e7d0456eb2a6969fa0ce3ee633a79dc1e6a0d45ddfd1c4897e05ad1` |

Before/after comparisons independently establish:

- Phase 158's original CONTRACTS, CHECKPOINT_CONTRACTS, TESTS, REGISTRATIONS and `checkCheckpointEdges` are byte-identical. There remain 47 original contracts, five checkpoint contracts with anchor counts `[4,2,11,10,11]`, 33 named behaviors and six registrations. Persist/IfNeeded, validated lineage completion, checkpoint-before-prune ordering, callback declarations/counts and no-snapshot refusal remain.
- Phase 162's contracts/counts/native historical snapshot file differs from the parent only by the two new test routes in each historical array. All 208 function contracts and exact CHECKPOINT_COUNTS remain; the third edge still requires the original five/two occurrences.
- The old exclusion subject alternatives, positive matcher, clause handling and negation rules are preserved. The only deliberate subject-spelling strengthening accepts repeated whitespace/hyphens before production service/readiness/use/safety. Optional current-page coverage uses the existing shared state-aware scan; it adds no blanket quote, blockquote, historical-paragraph or code-fence exemption.
- All six original proof roots and 22 original Phase 162 context/proof/formal-report/SUMMARY documents are byte-identical to the parent. Their ordered six-source digest remains `a4b0d71d01def301ba406d6b46c351ed8944d8d4508182ffb0dfd21960043575`. Both immutable original audit snapshot hashes match the approved values in the manifest below.
- Every old verifier line remains in its original order: removing only the four added routing lines reconstructs the parent `verify.sh` exactly. Each new route occurs once in documented and executed surfaces immediately after 162check; the original contiguous 161check → oracle test → 162test → 162check sequence and full gates remain.
- Active traceability adapters are unchanged. Source/function/assertion/oracle/ownership/baseline/frozen-helper and missing-map refusal remain enforced.

## Policy, native adapters and publication

The pure resolver imports only built-in crypto and uses supplied strings over a finite fixed universe. Requirements, roadmap and audit candidates are milestone-bound; equivalent overlaps are allowed and conflicting/malformed relevant candidates refuse. Present unclassifiable or empty active controls refuse with their active path. Only affirmative unique later-milestone identity or the exact fixed whole-document roadmap archive index permits ignoring a present active control. Absence and presence are preserved through all five actual adapters and every repeated policy resolution. Maps remain the entire evidence universe: omitted controls, proof, reports or scope cannot borrow valid disk files.

The 20-input exported `PROVENANCE_DIGEST_FILES` covers implementation, tests, callers, native fixtures, contracts, verifier, scope/context and immutable audits. Closure requires each to be present and nonempty. Mutable current publication documents, reports and live audits are intentionally outside that digest; their separate reviewed-byte inventory follows below.

Historical Phase 162 proof is required independently of reopened CFGR-01 ownership. Linked Pending needs no future Phase 163 reports. Earned canonical functional-closure verification and in-progress SUMMARY activation can close CFGR-01 while audit/readiness/Plan04/Phase163 remain outstanding; renewed passing audit then earns readiness; final canonical phase scope and completed SUMMARY follow. Archived controls additionally require final closure and passed audit. Exact current unarchived sentences cease to be exempt after archival. None of the four authorized sentences grants shipment.

The native fixture module copies a finite evidence/import corpus into owned regular files, invokes the unchanged actual entrypoint bytes with pinned Bun, checks both filesystem adapters and both supplied-map paths, and performs the sequential overlap → archive-only → next-active transition. It uses no fake runner, loader replacement, network, Git archive, recursive checkout copy, symlink/hardlink or Cargo. Path ownership, import/evidence bounds, subprocess diagnostic bounds and try/finally cleanup are explicit. Original-state fixtures deliberately omit the new optional page and normalize their intended owner/status/claims; future live lifecycle advancement does not manufacture historical test assumptions.

Current prose distinguishes historical delivery from the reopened archive closure, preserves dated failures and the unperformed Phase 156 human-confirmation condition, and retains BASIC/type 0, single Fjall, default-off behavior, missing-history refusal, body/wallet restrictions, soft/unattained 550 MiB target, point physical observations, synthetic-root/work/halving limits and inherited v2.4 advisories. BIP37/V0, assumeutxo/dual-chainstate, archive-scale, public/default/network-CI, implicit repair/import/download and production/service/funds exclusions remain.

Simplification pass: one pure selector/state policy, finite adapters and one shared six-boundary fixture runner suffice. The repairs align existing parsers and explicitly normalize test states; they introduce no dependency, generic traceability exception or new runtime architecture.

## Current evidence and historical attribution

Independent reviewer checks on the final `34443772…` frozen source:

| Check                                                   | Result                                                                                           |
| ------------------------------------------------------- | ------------------------------------------------------------------------------------------------ |
| Pinned Bun versus .bun-version                          | 1.3.9 / 1.3.9                                                                                    |
| Full pure provenance suite                              | 121 pass, 0 fail, 249 assertions, 217ms                                                          |
| Native identity/empty/transition controls               | 21 pass, 0 fail, 251 assertions, 39.21s; actual CLIs/filesystem/maps; all 21 owned roots removed |
| Original/live/empty compatibility                       | 11 pass, 0 fail, 21 assertions, 2.43s                                                            |
| Independent pure/map probes                             | 24 targeted path-specific refusal and absent/overlap/later-active positive checks pass           |
| Actual live 158 / 159 / 160 / 161 / 162 entrypoints     | All exit0                                                                                        |
| Shell syntax / diff whitespace                          | Both pass                                                                                        |
| README/package/policy/handoff local links               | 125 resolve; none missing                                                                        |
| Exported source digest / frozen literals / preservation | Independently recomputed and match                                                               |

The prior `f3e9…` review's 89 pure, two native conflict and three original positives retain their original boundary. Subsequent `f871…` pure/native execution completed 273/0 with 1,729 assertions in 188.57s and 152 roots removed, but its empty-file adapter gap prevented acceptance. The cooperating owners captured 15 identity REDs (13 wrongly accepted; two lacked active-path diagnostics), then two empty native REDs in 5.49s. Current acceptance relies on the independent final-byte checks above, not those superseded witnesses.

Separately inspected prior-snapshot logs: pure/native `220/0/1,438 assertions/166.93s` with 135 roots cleaned; caller159–161 `1,528/0/4,187 assertions/449.73s`; historical158/162 `3,211/2/10,744 assertions/337.48s`. These loaded earlier bytes and retain their original result, including the two historical fixture failures. The cooperating owner's completed current-byte log `/tmp/phase163-presence-final-native-pure.log` was independently inspected: **275 pass, 0 fail, 1,761 assertions, 185.52s**, all **154 owned roots removed**, 143 finite files / 3,506,326 bytes and corpus SHA256 `441419516af5f3996ae05ea92e16505fec70930e179ca69d1eaae49b64786bf9`. It belongs to the accepted `34443772…` snapshot. Full3/default verification remains a separate unearned root gate at this publication. Root reports full2 completed on the earlier source at 2026-10-12T23:48:14Z; it preceded these repairs and does not certify this digest.

Actual historical local JSON receipts `6c7c6001-7fbd-42ec-875d-840c0c380707` and `b427fa6a-122a-4c28-9dcb-e58342ec3eac` were independently read: success/exit0/full, 2,137,553ms and 2,064,537ms; both record dirty `a77edab276a9daa086055aa61e6143ce86bc1218` worktrees. Read-only GitHub queries independently returned completed/success at exact `d47f6e09b4621808974cc083fcc27ee9d2c1b4c7` for [CI 38206596321](https://github.com/bright-builds-llc/open-bitcoin/actions/runs/38206596321) and [Bright Builds 38206596267](https://github.com/bright-builds-llc/open-bitcoin/actions/runs/38206596267). Neither boundary certifies the changed Phase 163 runtime tree.

## Exact ordered closure-binding inventory

Independently computed through exported `provenanceDigest` and `PROVENANCE_DIGEST_FILES`: **`34443772f20ccfd57013a53d62ffd32f7fbf9e0b8700bb50be4b8ca7e2175ff8`**. The function hashes ordered repository-relative `path:sha256` rows joined by LF without a trailing LF; full paths below identify the actual inspected files. This is the sole closure-policy digest.

```text
/Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase163-provenance.ts:cc8d36584e7d0456eb2a6969fa0ce3ee633a79dc1e6a0d45ddfd1c4897e05ad1
/Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase163-provenance.test.ts:3ba274c9124e071e1c1d384f3527d04285f84b4090cb831ce55051dee56103b5
/Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase158-validated-reorg.ts:1df027aca2deb9827f135d9a558ef2377d7a2ebb18b830ba76f41698376834de
/Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase158-validated-reorg.test.ts:28c5ed07dfb670d439c4859dd8f93ff7f7b2e1624d9b58ba2b9106319778965a
/Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase162-integrated-filter-proof.ts:912df3e04b070833778cfbf03aaaf7dfefe20f107c3497e81d2ed4f5521867a3
/Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase162-integrated-filter-proof.test.ts:97e69067ea0a291917465d6379c6c4adaba4da4919c00611c27aac82a7fe119c
/Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase159-filter-rpcs.ts:563ec1eb3694e9c3961f705f250596ea68292fc67158f8c85375f07f08184a32
/Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase159-filter-rpcs.test.ts:9bfdc8c7a7b927296d57cb03f394412630d042b6a06fd03e943e4b21ce9958f9
/Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase160-peer-filter-serving.ts:41f9054550790bf2f0263cedb4f0dd74c964a29a6fc4b8a492b1f17fa6cdbc05
/Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase160-peer-filter-serving.test.ts:d44b875591976e6e7e959c48c6348ea5c5f291ac2b249d34bf295a60e0d97a65
/Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase161-operator-index-evidence.ts:c3447ae4c4cc07c8a9d2de712b663520aab545f990730ec77e3e93ca7e93ce23
/Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase161-operator-index-evidence.test.ts:33449ab72da6b5630349e3b5f47402322feb6576c866c9569069b3ed1a9d3177
/Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase163-provenance/native.test.ts:b85d81ee18eba16cc0a5e058c1cf34c7682d67b677a2d0e58e8d84f8062f28f8
/Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase163-provenance/fixtures.ts:588c41ff2719aec69a53b0799c7596945b697bff7f45162532eca2562c7dea5a
/Users/peterryszkiewicz/Repos/open-bitcoin/scripts/check-phase162-integrated-filter-proof/contracts.ts:b5e5df1f4dc8705cdbe1ca85f9d9049a6e0188653311e00f2cacabb9f88daa00
/Users/peterryszkiewicz/Repos/open-bitcoin/scripts/verify.sh:cd051fd967934b85a7489f75ad8793cc4880ef918ced776b179a8ff7fb3165bf
/Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-CONTEXT.md:a766eda3b17389bdfd390775c04085ee756c402d00a918be3fd6505c287d7956
/Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-GAP-SCOPE.md:26b0ac69e42cf3eeeef175ae49f698c3a373b933b933249e5752ad942d801274
/Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-ORIGINAL-MILESTONE-AUDIT.md:f14fa21dff9e340c9794f6190925fc42075f5624d930e551f0d5666b702697d8
/Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-ORIGINAL-INTEGRATION-AUDIT.md:f60ef2fa6eea08b3fe00b241a5f2ae47c9f7ea864f1649ff865d7476d43b2709
```

## Separate reviewed-document byte inventory

These full-path SHA256 values record documents at 2026-10-13T00:09:51Z, including the corrected roadmap and later dated execution/rebind addenda. They are review evidence, not a second closure-policy digest. Later authorized publication or evidence appendices must retain their own attribution and rerun the relevant publication guards; this inventory does not silently roll forward.

```text
/Users/peterryszkiewicz/Repos/open-bitcoin/README.md:ce5d31c65714e7590514c293708abcb31ff56dad339d0cb99a5622254627f09b
/Users/peterryszkiewicz/Repos/open-bitcoin/packages/README.md:5fac5848b902663757846ded940ae977b719b3a3466587d041b0790a0441f4d3
/Users/peterryszkiewicz/Repos/open-bitcoin/docs/parity/index.json:4fe8bd73272159e05949b99f46bb6991e3abf131ce17494816f83e4c979f67da
/Users/peterryszkiewicz/Repos/open-bitcoin/docs/parity/v2-5-archive-safe-provenance.md:2c39b27306a646b4df544d52830de89b93cd1c77220fbc958525d4f43ca9d083
/Users/peterryszkiewicz/Repos/open-bitcoin/.planning/PROJECT.md:45ef3aa2715fb10fae802a6d089b6a48768fd842bc48b8d439aa8e1694a56ab6
/Users/peterryszkiewicz/Repos/open-bitcoin/.planning/REQUIREMENTS.md:baf68697e000189dd02764017b20a800574b5d35617af73547804718ed3e1a3f
/Users/peterryszkiewicz/Repos/open-bitcoin/.planning/ROADMAP.md:aa5b21518a38196426799908628769530fb16511dd553cf930b7430a026dcbd5
/Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/162-real-prune-retention-and-integrated-parity-proof/162-UAT.md:b453aa501b883c2a846eabc1ac03a5a28c31d8ab1b4eab3430b39dc2d32f80ae
/Users/peterryszkiewicz/Repos/open-bitcoin/.planning/debug/phase156-automatic-writers.md:adcf42f92760e60966669bd4587b174ac84e8e2ec6d3591c13533677bdf4cf63
/Users/peterryszkiewicz/Repos/open-bitcoin/.planning/debug/bright-builds-file-lengths-ci.md:96438ad700447030f5e8c39c66ba50a7807086686bfcd51ce45d27e0008ae513
/Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-CONTEXT.md:a766eda3b17389bdfd390775c04085ee756c402d00a918be3fd6505c287d7956
/Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-RESEARCH.md:49fed898191c5ea5b36daee06d88da7173bf4e7f420fb852953c2906026e4d4d
/Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-GAP-SCOPE.md:26b0ac69e42cf3eeeef175ae49f698c3a373b933b933249e5752ad942d801274
/Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-CHECKER-REVIEW.md:b8438f341bdcc0fe54167d045294c839e50819375257296d66f50c5aefcd3154
/Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-CHECKER-REBIND.md:221a68506c1815dc6d16d8dd4303a13457d9fef67b5ffac0296496cb9bd95a9f
/Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-TRANSITION-EVIDENCE.md:f0f8ac326cff32758ff10ae6081059c702af1c4bd0881686f8094182596a4de3
/Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-HANDOFF.md:661d8f41225c3a36333a2bb33937a1739e602e47c031f6073c4e5e1b11d864e4
/Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-ORIGINAL-MILESTONE-AUDIT.md:f14fa21dff9e340c9794f6190925fc42075f5624d930e551f0d5666b702697d8
/Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-ORIGINAL-INTEGRATION-AUDIT.md:f60ef2fa6eea08b3fe00b241a5f2ae47c9f7ea864f1649ff865d7476d43b2709
/Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-01-PLAN.md:77c9df696dbcf993921e8a266e82f61953244f09dcd009330d90d8f84aa0bda7
/Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-02-PLAN.md:8db4e57abdf4ffc7d5a0349cf23676d45426a8bf2dcf159a78d3d074bd3f28f1
/Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-03-PLAN.md:26228cfa552e3230cd3c5b4a646447399c7149c7527c9f26bd2f9db2d7804a12
/Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-04-PLAN.md:80b691df1abe3e6e398860f2c9dddf44601208a4ebcaab21acbc4fe4b121c274
/Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-01-SUMMARY.md:fb2e8d9ee1217abbeab9a261cc730040e79557cb854298db3fb526df89ffb815
/Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-02-SUMMARY.md:7163d5bd5d57b225752e4215bc2795edcb3a778a6b42a5500d6f695854a2aabd
/Users/peterryszkiewicz/Repos/open-bitcoin/.planning/phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-03-SUMMARY.md:dc720b32c05ed78c9fd4b47938c1aae0694e762af8c4dacb423d9de184c37896
```

______________________________________________________________________

_Reviewer: gsd-code-reviewer. Current source rebind accepted at the exact digest above; earlier acceptance and executions retain their historical boundaries. Full3, formal closure, renewed audit and final publication remain separate gates._

## Wave 5 readiness publication acceptance | 2026-10-13T02:43:21Z

**Status:** clean. **Depth:** standard. **Current scope:** 21 files, exactly the 16 non-planning entries of exported `PROVENANCE_DIGEST_FILES`, plus the root README, packages README, parity index, archive-safe provenance page and tracked LOC report listed in current frontmatter. Explicit scope overrides SUMMARY/Git discovery. All scoped paths exist, resolve canonically within this repository and are not ignored. Planning/audit/certificate/handoff records were inspected as supporting context and are not counted as source files.

The earlier 42-file review, checker rebind assessments, repaired WR-01/WR-02/WR-03 and IN-01, complete ordered source manifest and dated document inventory above remain historical evidence. All 20 raw input hashes were independently recomputed and individually matched that accepted manifest, including original audit snapshots `f14fa21d…` and `f60ef2fa…`. Exported `provenanceDigest` independently returns **`34443772f20ccfd57013a53d62ffd32f7fbf9e0b8700bb50be4b8ca7e2175ff8`**. Prior standard-depth per-file source assessments are reused only after this byte-for-byte equality; actual changed-source diff and policy/adapter/frozen/native contracts were also inspected. Publication artifacts received current contextual review.

The actual renewed milestone audit is `passed`, 22/22 requirements, with empty requirement/integration/flow gaps, matching lifecycle/source and audit time **2026-10-12T21:11:09-05:00**. Its integration binding independently matches **`ff909e9297b2d046dbef6a6ee0842a367a119a08ad5c922653a9bb3d6b54bff8`**: 31/31 original seams, 18/18 flows and all 22 requirements wired. Its dated nine-final-plus-one-functional certificate inventory is preserved. The audit follows the canonical functional closure and precedes the earned readiness publication; it does not assert actual archival.

Seven exact `READINESS_CLAIM` occurrences across six current publication files were independently counted. No current exact gap, closure or archived-state sentence remains in those files. The live resolver returns **historical complete / gap_complete / audit_passed**. Both READMEs, the parity rationale/page and current handoff distinguish this earned readiness from pending final whole-phase verification and record reconciliation. Product limits remain BASIC/type 0, single Fjall, default-off behavior, missing-history refusal, body/wallet limits, soft/unattained resource target, synthetic-chain limits and inherited v2.4 advisories. No new runtime capability, public/default/network-CI, production/service/funds safety, actual archive/tag/shipment or current dirty-tree remote CI is asserted.

### Independent evidence checked at this publication

| Check                                                                    | Observed result                                                                                                                                                                                                                                                                                                                                                             |
| ------------------------------------------------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Read-only policy and both actual supplied-map adapters, pinned Bun 1.3.9 | 21 concern-level checks pass in 5.99s: live readiness; exact claim authorization; original bootstrap/implementation/formal; Pending; functional; audited; final; overlap; archive-only; later active; empty active controls; omitted archive input; wrong digest/gaps/chronology; premature final; stale unarchived claims. No fixture writes, subprocesses or test runner. |
| Source/function/frozen/transitive bindings                               | Original proof roots and checkpoint/assertion contracts retained; changed 158 checker/test and transitive 163 helper still match exact frozen literals; all five adapters preserve supplied-map presence and never rescue missing keys from disk.                                                                                                                           |
| Actual full4/default receipt                                             | `a78124d3-c707-4813-ac47-74817889fa75`, success/exit0/full, 2,537,329ms; all 176 unique stage receipts independently checked within its interval and each success/exit0/full. Full log hash independently matches `239f7d2ded4fca466c5787dc18d867f67da524b5d8bd47ff29ce8a80f5888b57`.                                                                                       |
| Actual post-readiness native/pure receipt                                | `7c5bc38e-a22b-410a-a915-5221b7378276`, 02:26:20.661Z–02:29:34.681Z, success/exit0, wrapper 194,020ms; log confirms 275 pass, zero fail, 1,761 assertions, Bun 193.80s, all 154 owned roots removed. Corpus: 146 files / 3,600,957 bytes, SHA256 `6fc7c8d2ff24f51a13c4edf13e28d065a447e5e1bc1d320e88c943d89669643d`.                                                        |
| Tracked generated LOC artifact                                           | Pinned generator `--source=worktree --check` passes; no regeneration performed. Root's later staged/index verification remains its own gate.                                                                                                                                                                                                                                |
| Shell syntax / whitespace                                                | `bash -n scripts/verify.sh` and `git diff --check` pass.                                                                                                                                                                                                                                                                                                                    |

The initial reviewer probe used the 162-only map as though it contained 158's additional mandatory roots and stopped on the combined positive check. The harness was corrected to read the union of both explicit root sets and rerun with pinned Bun; the results above belong to that completed probe. This was no source defect or changed acceptance rule.

Native full4 remains evidence for its dirty working source snapshot atop `d47f6e09b4621808974cc083fcc27ee9d2c1b4c7`; no heavy suite was rerun by this reviewer. Earlier dirty-`a77edab2…` local receipts and exact-`d47f6e09…` remote CI runs keep the historical attribution recorded above. The earlier failed/superseded source receipts remain unchanged.

Material guidance: actual AGENTS/Repo-Local Guidance, Bright Builds sidecar, placeholder overrides, architecture/code-shape/verification/testing/local-guidance/TypeScript standards and GSD code-review workflow. Both canonical active lesson files were read in full, 7,188 bytes / 2,397 conservative estimated tokens, with no archives loaded. No project skills were present. Sole write ownership was this report.

The new acceptance region passes explicit GFM/frontmatter mdformat check. Whole-report check reports inherited historical formatting differences; those dated sections were preserved without broad normalization. Final metadata, current readiness authorization, unchanged source digest and whitespace checks pass.

### Current document/context byte inventory

These hashes record this readiness publication. They do not replace the sole 20-input source digest or roll the preceding dated inventories forward.

```text
README.md:7d02874cf8eaf55faf7a3cb7e8b710c9bda28977d74833d402d64ae43b22787e
packages/README.md:bfaa4e09629a7cd81f57a5666f64ef489cc25bf05b3131798bb83e700f73c153
docs/parity/index.json:d8b9cd3b058accf9b0d8af5fdac30e3f269a56add031a011b6f0d478b40ec0ef
docs/parity/v2-5-archive-safe-provenance.md:58dc30558efaa9de3f51a04eb73fc4bfad9985b42e96ec74dbe133e95124163c
docs/metrics/lines-of-code.md:dff171115741f1daeb8b34c6d838e3f471478f71113afb4b9b146bf40bdf28bf
.planning/PROJECT.md:24a1dc4b77479a0041dc1b2b28fd4787fe5593db540e920ad6fa6d20b10a88fc
.planning/phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-HANDOFF.md:0e6dc5594f272cb0d39474df764039149c8b4f74e6812bb97a7d8fdf2872fc6d
.planning/v2.5-MILESTONE-AUDIT.md:4d8ef787684a10c440adfe4a7ba40583ed1fa45aca17eaa768eeb50c5d13e072
.planning/v2.5-INTEGRATION-AUDIT.md:ff909e9297b2d046dbef6a6ee0842a367a119a08ad5c922653a9bb3d6b54bff8
.planning/phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-04-SUMMARY.md:452061b9f269b454c74c885e8dc988c84959214736a806061dc0a5eb56a62841
.planning/phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-VERIFICATION.md:fff3a80e68a3a48b4c754fc938b4b9ce249e91f996f45ed5b4ba667f1d5d5777
```

**Current findings:** Critical 0; Warning 0; Info 0; Total 0. The current source and readiness publication pass this review. At review time canonical verification is still `functional-closure`, canonical 04 is `in_progress`, 05 completion is pending, and global counters remain 3/5, 88/90 and 9/10. Final canonical certificate, 04/05 records, publication reconciliation, final guards and normal-hook Git finalization remain separate root gates. Any later implementation change requires renewed binding and acceptance; later publication-only changes require a bounded reconciliation against this dated snapshot.

## README legacy-guard compatibility acceptance | 2026-10-13T03:26:00Z

**Status:** clean. **Scope:** only the two current root README sentence replacements relative to the staged README. The existing frontmatter, 02:43 review scope, earlier source review, manifests and execution receipts are preserved unchanged; this appendix records a later documentation compatibility assessment.

Independent comparison proves the entire working README equals the staged README with exactly two `READINESS_CLAIM` sentences replaced by the already-authorized `CLOSURE_CLAIM`. Staged README SHA256 was `2d91cb006159678aeeb235d5f8614d07ef7d2def7bfbf512e2dfb33cda8371a6`; the accepted working README SHA256 is `265cd26265545394cfdd96df2f3def80be6301e0e64c0a80e626417803e29f44`. Adjacent prose still records the actual renewed audit and earned readiness. The one original exact Phase 162 completion sentence, its dated 2026-10-12T03:42:55Z historical context and all other README bytes remain unchanged.

This preserves earned readiness: packages README, PROJECT, parity index, provenance policy and HANDOFF each retain one exact `READINESS_CLAIM`. The live policy independently resolves historical complete / gap_complete / audit_passed; the audit is passed with 22/22 requirements, and the current canonical certificate is phase/passed with SC1–SC5. Those current records are supporting context for this narrow prose review; they do not relabel the earlier review's pending-finalization boundary.

The unchanged Phase 157 scanner rejects the broad v2.5/is-established wording. Using the existing exact closure sentence in the root README restores compatibility without altering any guard, test or exemption. All 20 implementation inputs still independently produce accepted source digest `34443772f20ccfd57013a53d62ffd32f7fbf9e0b8700bb50be4b8ca7e2175ff8`.

Independent pinned-Bun live Phase 157, live Phase 162 and current-document reconciliation commands each exit 0; scoped whitespace checks pass. The failed normal-hook attempt remains a failure. No full suite, hook retry, successful commit or push is claimed by this appendix; root retains the normal-hook retry and its actual receipt.
