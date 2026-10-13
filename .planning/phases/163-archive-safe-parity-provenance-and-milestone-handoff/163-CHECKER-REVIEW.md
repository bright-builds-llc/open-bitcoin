---
phase: 163-archive-safe-parity-provenance-and-milestone-handoff
generated_by: gsd-code-reviewer
lifecycle_mode: yolo
phase_lifecycle_id: 163-2026-10-12T21-09-41
reviewed: 2026-10-12T22:06:21Z
depth: deep
files_reviewed: 9
files_reviewed_list:
  - scripts/check-phase158-validated-reorg.ts
  - scripts/check-phase158-validated-reorg.test.ts
  - scripts/check-phase159-filter-rpcs.ts
  - scripts/check-phase160-peer-filter-serving.ts
  - scripts/check-phase161-operator-index-evidence.ts
  - scripts/check-phase162-integrated-filter-proof.ts
  - scripts/check-phase162-integrated-filter-proof.test.ts
  - scripts/check-phase163-provenance.ts
  - scripts/check-phase163-provenance.test.ts
findings:
  critical: 0
  warning: 0
  info: 0
  total: 0
resolved_first_pass_warnings: 7
status: clean
byte_rebind_acceptance: accepted_scoped_source_review
review_scope_digest: e6bc2761a2d829a053dbaf379229bee8efb5c4f3290eb522ca6434e591cd98ab
---

# Phase 163 Plan 01: Independent Checker Rebind Review

## Scope and disposition

The nine-file Plan01 replacement byte inventory below is accepted for this scoped independent source review. All seven first-pass warnings are resolved in the reviewed snapshot, including the later duplicate-key variants under WR-01. Original probes ran against initial helper SHA256 `228a20913ee317a230c3807532e1276067f06a651cb82ef294431ff0eef14e82`; the accepted helper is `7a768155c4a73687677e3065ce7054408c4442db297a3226e1a0b83928b6211b`. The report preserves the original findings and intermediate repair observations as dated review history rather than presenting them as current open issues.

Review covers the supplied Plan 01 source/caller changes relative to `d47f6e09b4621808974cc083fcc27ee9d2c1b4c7`, their transitive provenance policy, map isolation and preservation of historical proof. It is not whole-phase final review, full native verification, security certification, audit acceptance or readiness publication. No source, shared tracking or Git state was changed by this reviewer.

Material guidance: repository `AGENTS.md`, `AGENTS.bright-builds.md`, placeholder `standards-overrides.md`, architecture/code-shape/testing/verification/TypeScript standards, approved 163 CONTEXT/RESEARCH/01-PLAN and the staged closure order in 04-PLAN. Both active lesson files were read completely: 5,230 + 1,958 bytes, 1,744 + 653 conservative tokens; no lesson archives were loaded. No project skills were present under `.claude/skills` or `.agents/skills`.

## Resolved first-pass warnings

Locations below identify first-pass candidate lines and the reviewed function. Files are under `/Users/peterryszkiewicz/Repos/open-bitcoin/`. Every item below was independently rechecked after repair; current open finding counts are zero for the declared nine-file source scope.

### WR-01: Quoted duplicate YAML keys bypass ambiguity refusal

**File:** `scripts/check-phase163-provenance.ts:61-70` (`provenanceMetadata`).
**Issue:** The key scanner recognizes only bare keys. On pinned Bun 1.3.9, `status: failed` followed by `"status": passed` parses to a passing value without duplicate rejection. A context containing `status: denied` followed by `"status": approved-scope` similarly bypasses the explicit duplicate-metadata contract.
**Fix:** Reject duplicate mappings before YAML overwrites them, including quoted/flow/merge key forms, or narrowly reject unsupported mapping syntax. Preserve valid quoted scalar sequence values in immutable original audit files; a transient broad rejection of every quoted sequence scalar was observed during repair and reported to the implementer. Add direct duplicate-key and genuine original-audit positive controls.

**Repair recheck:** On helper candidate `fc31dfa5f6b94ef507de21a53baea9dfdd5273faf9166f54faf21fd974a3d743`, quoted-key repair was present, but `status : approved-scope`, tab-before-colon and `!!str status: approved-scope` still overrode an earlier `status: denied` without refusal. These variants were sent to the implementer under the same finding; independent acceptance remains pending.

The subsequent `cbc036731fe3f1b967e25224e9dab1074e97b1e9e5dd6fd6b52d31c5c1357a94` candidate refused all seven independently probed quoted/spaced/tabbed/tagged/anchored/sequence-key variants and passed 52 pure tests. A final targeted audit probe still exposed a nested flow-map variant: replacing current `gaps.requirements` with an otherwise equivalent JSON flow array containing duplicate `id: CFGR-02` then `id: CFGR-01` returned `gap_pending` with no failures. This is reachable through the valid audit record-array schema and was sent for narrow refusal, without expanding to a general YAML parser.

### WR-02: Invalid context dates bypass report freshness

**File:** `scripts/check-phase163-provenance.ts:139-159` (`validateGap`, `freshReport`), and `:185-186` (`auditPassed`).
**Issue:** Missing or invalid context `generated_at` is accepted. Comparing report time with `Date.parse(invalid)` yields a false comparison, allowing source-digest-matched functional closure reports dated 2026-10-11 to authorize the 2026-10-12 lifecycle. A supplied-map probe returned `gap_complete` with no failures. The final-phase audit timestamp exception also needs a current-lifecycle lower bound rather than permitting any finite old audit date.
**Fix:** Parse and require finite canonical timestamps for scope, context, reports and audit; enforce approved lifecycle lower bounds before chronology comparisons. Retain the valid final-report refresh after an earlier renewed audit. Test missing, invalid and pre-lifecycle dates.

### WR-03: Current gaps audit can disagree with claimed gap lineage

**File:** `scripts/check-phase163-provenance.ts:107-115` (`selectAudit`) and `:196-202` (`resolveV25Provenance`).
**Issue:** Changing the live `gaps_found` audit requirement from CFGR-01 to CFGR-02 still returns `gap_pending` with no failures. The immutable original snapshot remains valid, but current audit linkage is not checked, allowing the exact current AG01/AF01 gap sentence to be authorized from conflicting current evidence.
**Fix:** Require the current gaps audit to retain the approved CFGR-01/Phase162/partial and AG01/AF01 linkage, or equivalent narrowly defined original gap semantics. Add map and claim-view refusal tests.

### WR-04: Conflicting milestone identities in one control pass

**File:** `scripts/check-phase163-provenance.ts:73-74` (`requirements`) and `:92-93` (`roadmap`).
**Issue:** Appending `**Milestone:** v2.6 Future` to otherwise valid active v2.5 requirements is accepted with no failures. Identity and baseline checks look for an acceptable occurrence rather than refusing ambiguous control identity.
**Fix:** Require one coherent milestone identity and reject contradictory identity/baseline declarations. Preserve the legitimate archive envelope with its v2.5 body. Cover both requirements and roadmap controls.

### WR-05: New tests assume the checkout remains Pending forever

**Files:** `scripts/check-phase158-validated-reorg.test.ts:503-532`, `scripts/check-phase162-integrated-filter-proof.test.ts:12-35`, `scripts/check-phase163-provenance.test.ts:9-42`.
**Issue:** Positive/refusal fixtures inherit mutable live controls and require canonical Phase163 verification to be absent. Legitimate Plan04 CFGR-01 completion, renewed audit or canonical report publication would make retained tests fail because the live state advanced. Original-owner fixtures also inherit mutable current audit state.
**Fix:** Explicitly construct Pending/original fixtures, normalize their requirement and roadmap ownership, use the preserved original gap audit where intended, and clear future reports from those maps. Keep actual-live acceptance lifecycle-neutral. Add a fixture-stability control using advanced live publication inputs.

**Repair recheck:** The first normalization repair still matched only `Phase 163 | Pending` in Phase158 `transitionSources`, leaving legitimate live Complete ownership unnormalized. Also identified the inherited Phase160 test fixture reading REQUIREMENTS from CHECK_FILES after that path ceased to be mandatory. Both need narrow fixture compatibility corrections; the latter requires explicit scope/manifest expansion if its test file changes.

### WR-06: Archived state authorizes false current unarchived wording

**File:** `scripts/check-phase163-provenance.ts:225-227` (`provenanceClaimView`).
**Issue:** Archived publication removes CLOSURE_CLAIM and READINESS_CLAIM, although both exact sentences assert that v2.5 remains unarchived. This grants a current-document exemption to a false lifecycle statement after archive selection.
**Fix:** Restrict current unarchived sentences to valid pre-archive states; authorize ARCHIVED_CLAIM after standard archived controls. Preserve the independent historical Phase162 proof sentence. Test that unarchived current sentences remain subject to refusal in archived state.

### WR-07: Passing audit is rejected before the planned final report refresh

**File:** `scripts/check-phase163-provenance.ts:203-208` (`resolveV25Provenance`).
**Issue:** A passed renewed audit with canonical `functional-closure` verification is rejected until the report already has `phase` scope. Approved 04-PLAN Task3 instead requires audit pass, readiness publication and native guards before the final canonical phase refresh. Tests jump directly from functional/gaps to final/passed and omit the necessary intermediate transition.
**Fix:** Represent functional closure plus a renewed passing audit as a valid earned publication/readiness transition while Plan04 and Phase163 remain in progress. Preserve the distinct later final-phase gates. Add positive and premature-claim controls for this intermediate state; align any ambiguous Plan01 wording with the approved staged execution order.

## Preserved original evidence

Independent byte comparisons against the original HEAD confirmed:

- Phase158 `CONTRACTS`, `CHECKPOINT_CONTRACTS`, `TESTS`, `REGISTRATIONS`, `checkCheckpointEdges` and the entire excluded-subject scanner are unchanged.
- The 47 Phase158 contracts, five checkpoint contracts with anchor counts `[4,2,11,10,11]`, 33 required behavior tests and six registrations remain intact. Phase162 retains 208 contracts and the original checkpoint count matrix, including the five/two occurrences on the third checkpoint edge.
- Original frozen Phase158 checker/test SHA256 values are `b2339c02beebd971432bd53dfc81df5f563a41968b5181ff66112045748169e5` and `68e444d11ae5a3203c6a51c069a00a496db6dd99bbf191c724c2ca496f1855ec`.
- All six runtime proof-source files, original Phase162 CONTEXT, proof document, VERIFICATION/NATIVE-EVIDENCE/REVIEW/SECURITY and all sixteen SUMMARY files are byte-identical to the baseline. The ordered six-source digest independently recomputed to `a4b0d71d01def301ba406d6b46c351ed8944d8d4508182ffb0dfd21960043575`.
- Pure policy imports only built-in crypto. Supplied-map entrypoints retain map-only evidence lookup. The helper is transitively mandatory via PHASE162_FILES and explicitly byte-bound by the Phase162 checker. Caller edits in 159/160/161 only supply the evidence text context to existing claim-view/exclusion calls.

The first-pass findings concern the new policy, its lifecycle fixtures and staging; they do not invalidate the preserved runtime evidence.

## Independent final rechecks

Pinned Bun 1.3.9 was verified against `.bun-version`. On the final `7a768155…` snapshot:

- Pure provenance suite: **54 passed, zero failed**, 114 assertions, 68 ms in the reviewer's run. This includes original/Pending/functional/final/archive states and the exact refusal/claim boundaries.
- Independent current-audit flow-array duplicate reproduction now refuses with a bounded provenance category; ordinary scalar arrays and quoted brace/colon list values still parse.
- Earlier independent reproductions were rechecked through the repair batch: missing/invalid context dates, conflicting milestone identity and altered current gap lineage refuse; quoted, whitespace, tabbed, tagged, anchored and sequence-key duplicate metadata refuse. Functional closure plus renewed passed audit earns readiness while the canonical report still records Phase163 `in_progress`; archived state no longer exempts current unarchived sentences.
- Selected Phase158/162 current-state, missing-map-key and unauthorized-claim checks: **15 passed, zero failed**, 2.43 s on the immediately preceding metadata-only candidate. Advanced completed-owner Phase158 fixture: **one passed**, 177 ms; its file did not change subsequently.
- Final-snapshot Phase162 actual-live, explicit Pending and helper-only tamper checks: **three passed, zero failed**, 1.078 s. The helper-only mutation refuses through the exact frozen binding.
- Both final live native checker commands exited zero. These are source/native-entrypoint checks, not a new Rust or full-default-native receipt.
- All three `checkFrozenGuards` constants match independently hashed raw bytes and the replacement table in `163-CHECKER-REBIND.md`. Diff whitespace checks passed. Preserved runtime/checkpoint/exclusion sections, all six proof roots, all 22 original context/proof/report/SUMMARY files and both original audit snapshot hashes were rechecked unchanged.

The implementer's bulk preserved suite in session `73890`, log `/tmp/phase163-plan01-focused-freeze.log`, was still running when this report was finalized. It loaded the preceding candidate before the final sequence-key/flow-array metadata guard and corresponding tests were added. That execution must be reported as its actual loaded snapshot; the final metadata delta is covered by the reviewer's final 54-test suite and focused checker checks above. This report does not claim that running suite passed, nor that the entire default native contract ran.

The inherited Phase159–161 synthetic-fixture compatibility follow-up, including the observed Phase160 fixture reliance on the formerly mandatory active REQUIREMENTS path, is explicitly assigned by root to Plan02. Those future test bytes are outside this nine-file acceptance and must be repaired and verified before whole-phase/native acceptance. The rebind record itself remains a candidate handoff for the later Plan04 complete-manifest review; this focused report is not a substitute for that fresh gate.

Simplification/diff pass: the new pure selector owns evidence decisions; thin fixed-path adapters own reads; the three caller edits only forward source text. No runtime abstraction, dependency, generic traceability adapter, broad claim exemption or certificate rewrite was introduced. Existing historical contracts and mutation intent remain intact.

## Accepted current byte inventory

SHA256 is computed over raw file bytes. The scope digest is SHA256 of the lexically ordered `path:sha256` lines below joined by LF with no trailing LF. This is the stable frozen snapshot observed at 22:06:21Z and independently rechecked. Any later source change requires a new inventory and review of the changed paths.

```text
scripts/check-phase158-validated-reorg.test.ts:759870b52a6e2dbe282592a60ab1dd7cd7f9efe35b9c54597481fb9c023b37b4
scripts/check-phase158-validated-reorg.ts:d079ef8ff44ca4e7728841e53a4ff3ea04d3b7266b240df21c5acd623eb3d2e8
scripts/check-phase159-filter-rpcs.ts:b9629b0968d78f49214bb03e614517d0bc8df0e69e25d41522517c5f69af9a89
scripts/check-phase160-peer-filter-serving.ts:6aba56b08a6063bbe1b7e8b0cad24718b72a9caafdf9d462c27e4fac0d633946
scripts/check-phase161-operator-index-evidence.ts:a4f7e55dfbd607968a00f5ac1d67037f9bf2f6ea2b0e6e60f5fc3304c7c55d8b
scripts/check-phase162-integrated-filter-proof.test.ts:d0c72cb79cf8650eba760a521a010b3aa2517b3df689090f34eab2a9b3f2211d
scripts/check-phase162-integrated-filter-proof.ts:d2fc14442b0fac1d7557f68ac443a4a16484a73ff1a1b49b4fa6cf0254d7e817
scripts/check-phase163-provenance.test.ts:b06e26770aeaa85e0522065520cbb3b1c8566079547c71000045f895dd4a7531
scripts/check-phase163-provenance.ts:7a768155c4a73687677e3065ce7054408c4442db297a3226e1a0b83928b6211b
```

**Disposition:** accepted for the exact scoped source rebind above, with zero current open findings in that scope. Bulk historical-suite completion, Plan02 adapter/fixture proof, full default native verification, fresh whole-phase review/security, canonical closure and renewed audit/readiness publication remain separate actual gates. No archive, shipment, production capability or funds-safety claim is authorized by this report.
