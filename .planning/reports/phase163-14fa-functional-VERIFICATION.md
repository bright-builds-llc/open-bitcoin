---
phase: 163-archive-safe-parity-provenance-and-milestone-handoff
verified: 2026-10-13T18:51:46Z
status: passed
score: 4/4 functional must-haves verified
generated_by: gsd-verifier
lifecycle_mode: yolo
phase_lifecycle_id: 163-2026-10-12T21-09-41
generated_at: 2026-10-13T18:51:46Z
lifecycle_validated: true
source_digest: 14fa9cc585c9cf8d9b2c0ae04d7f92e6411b38fb7d19128359126eee2735f977
source_inventory_count: 21
requirements: [CFGR-01]
success_criteria: [SC1, SC2, SC3, SC4]
readiness_gate: enforced
verification_scope: functional-closure
audit_status: pending
readiness: not_asserted
plan04_status: in_progress
phase_status: in_progress
native_run_id: dfbd420c-9a6f-405e-8d24-65c9d9385a08
matrix_receipt_sha256: beb01a4bc4c08cc9571e5dd3be68a12862995f58c38b3719203d17deb45029da
overrides_applied: 0
re_verification:
  previous_status: passed
  previous_score: 5/5
  previous_scope: phase
  previous_verified: 2026-10-13T06:23:00Z
  previous_source_digest: e0c16fb204664274c63ea67d8b7c653dbe08a63bdc4c09d4e18fa8ecf139d72d
  reason: Archive safety hook exposed a test assumption; corrected source requires fresh staged acceptance.
  gaps_closed:
    - Physical active-requirements deletion assertion replaced by explicit logical availability and refusal controls.
  gaps_remaining: []
  regressions: []
---

# Phase 163: Logical-control functional verification

**Phase goal:** Contributors can run ordinary native parity verification during gap closure, after standard v2.5 requirements archival and with a later milestone's active requirements, while preserving original evidence and fail-closed claim rules.

**Result:** Functional PASS for SC1–SC4 and enforcement of SC5's conditional readiness gate. This is not final phase verification. Current CFGR-01 is Pending; canonical04/05 remain in_progress with no completed requirement. Root owns the following requirement/carrier activation transaction. A fresh integration/milestone audit, readiness publication and execution, and final same-path verification remain required before phase completion.

## Method and provenance

Independently inspected actual resolver/loaders, caller wiring, changed test/pin bytes, all five PLAN contracts, current controls/carriers, fresh review/security/native reports and raw execution evidence. SUMMARY claims did not substitute for implementation or receipts. No tests, builds, source/control edits or Git mutations were performed by this verifier; only this canonical report was created.

Material guidance: AGENTS Repo-Local Guidance, Bright Builds sidecar, placeholder-only overrides, standards index and architecture/code-shape/verification/testing/TypeScript guidance, and GSD override/gate/calibration references. Both active lesson files were read completely: 7,188 bytes / 2,397 conservative estimated tokens. No project skills were available.

CONTEXT, all five PLANs, all five SUMMARYs and this report have compliant yolo / `163-2026-10-12T21-09-41` execution provenance; no direct-fallback artifact was found. Original GAP-SCOPE's interactive `163-2026-10-12T21-03-37` identity remains separate approved creation lineage. This generation follows both current SUMMARY generations and the actual matrix completion. Strict lifecycle verification follows this write.

## Goal achievement

### Observable truths

| Criterion                                                                                                                                                      | Status                                           | Implementation and actual evidence                                                                                                                                                                                                                                                                                                                                                                                          |
| -------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| SC1: Original completed implementation remains verifiable during legitimate Pending reassignment; absent, forged or premature evidence refuses.                | VERIFIED                                         | `resolveV25Provenance` validates 22 unique IDs, checkbox/status pairs, unique owners, pinned baseline, approved scope and immutable original gaps. Shared158 retains original six-source proof and formal162 gates. Full/default and whole-layout suites execute original and reassigned states plus forged-owner/completion refusals.                                                                                      |
| SC2: Native checks select milestone-bound active/archive/next-active provenance; conflicting, missing or mismatched evidence refuses with bounded diagnostics. | VERIFIED                                         | Fixed requirements/roadmap/audit candidates distinguish absent from invalid-present sources, require parsed equivalence and bound diagnostics to 300 characters. Four whole-suite worlds include equivalent overlap, exact safety-index topology, archive-only and next-active. The new158 tests independently cover valid archive fallback, both candidates absent, empty active and malformed active.                     |
| SC3: Actual CLI/filesystem and supplied-map checks agree and preserve existing guards without disk rescue.                                                     | VERIFIED                                         | Presence-preserving `sourceText` and supplied-map branches bypass filesystem loaders. Six actual boundaries per world pass; omitted-key/empty/conflict mutations refuse despite valid disk evidence. Source/function/assertion/ownership/baseline/exclusion and exact frozen guards remain wired through158–162. Historical fixture normalization is test-only.                                                             |
| SC4: Current prose and old handoffs preserve actual receipts, historical failures and claim boundaries.                                                        | VERIFIED                                         | Current requirement row/checkbox/lead/footer and roadmap Plan/progress prose correctly show Pending and3/5. Current handoffs distinguish14fa acceptance from preserved e0 history; original debug failures and Phase156's unperformed human-confirmation condition remain visible. Contributor policy links and repo-local Cargo/Bazel commands remain available. No current audit/readiness or archive result is inferred. |
| SC5: Independent/focused/full-native/audit gates must precede readiness; archive layouts are isolated.                                                         | CONDITIONAL GATE VERIFIED; FINAL OUTCOME PENDING | Fresh clean review, zero-open security, complete default verification and whole-layout controls have passed. Production `validateClosure`/`auditPassed` requires current-source canonical evidence and renewed22/22 audit with empty blocking gaps before readiness. Live audit remains gaps_found; readiness is unasserted. Synthetic archived/audit certificates prove refusal policy, not actual audit acceptance.       |

**Functional score:** 4/4. The fifth roadmap criterion is retained explicitly and is not claimed complete or waived. No override applies.

### All five PLAN outcomes

| Plan | Observation                                                                                                                                                                                          |
| ---- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 01   | Pure fixed-milestone policy, honest reassignment, exact original audits and historical proof are substantive and wired.                                                                              |
| 02   | Actual loaders/CLIs/maps, finite fixtures, refusal/cleanup controls and exact ordinary verifier routes are implemented and freshly exercised.                                                        |
| 03   | Current/historical contributor and debug handoffs preserve bounded claims and dated failures. Reopened Pending prose was reconciled after the matrix witness boundary.                               |
| 04   | Independent review/security/full-native and this functional certificate are earned. Requirement/carrier activation is the root's immediate next transaction; canonical04 stays in_progress.          |
| 05   | Renewed actual integration/milestone audit, readiness execution and final phase reconciliation are outstanding. Canonical05 stays in_progress; this functional report does not complete its outcome. |

### Required artifacts and key links

| Artifact / link                                          | Status                            | Evidence                                                                                                                                                                                                                                     |
| -------------------------------------------------------- | --------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `scripts/check-phase163-provenance.ts`                   | VERIFIED                          | 299-line pure supplied-string policy; fixed selection, strict metadata, original lineage, canonical closure/audit and exact claim authorization. Imported/called by158 and directly frozen by162.                                            |
| 158 logical tests →162 frozen test pin                   | VERIFIED                          | Four explicit filesystem availability/refusal cases replace one false physical-path assertion. New raw test hash `efa71956ef2e4916a94790f9f9855a418e8f6d4b84c953db126654e89fbd85dc` matches the exact pin. Production resolver is unchanged. |
| 158–162 native adapters → shared claim state/policy      | WIRED                             | Fixed optional filesystem sources and exclusive supplied maps reach the same resolver; actual CLI/filesystem/map agreement and targeted omission controls execute.                                                                           |
| `fixtures.ts` / `fixture-controls.ts` / `native.test.ts` | VERIFIED                          | Actual entrypoint/import bytes, explicit historical construction, bounded corpus/imports, owned temporary paths and cleanup. Synthetic certificates remain labeled test data.                                                                |
| `scripts/verify.sh` → pure/native163 suites              | WIRED                             | Exact explicit `./` routes appear once in documented and executed order; both actual stages pass in the176-stage run. Existing ordered routes remain.                                                                                        |
| Original audits / six historical proof roots             | VERIFIED                          | Both raw audit hashes match f14fa21d… / f60ef2fa…. All six actual proof-table hashes match and ordered digest remains `a4b0d71d01def301ba406d6b46c351ed8944d8d4508182ffb0dfd21960043575`.                                                    |
| Fresh REVIEW / SECURITY / NATIVE → closure gates         | VERIFIED                          | All bind14fa/21/current lifecycle. Review is clean with zero findings; security16closed/0open; native passes actual full/default execution.                                                                                                  |
| Canonical verification +04 carrier → CFGR-01             | READY FOR ROOT ACTIVATION         | Policy requires this exact canonical generator/scope/criteria/lifecycle/digest and an in_progress04 carrier with CFGR-01 activation. Current carrier has no completed requirement; root must perform that transaction next.                  |
| Renewed audit → readiness/final/archive claim            | ENFORCED; LIVE ACCEPTANCE PENDING | Current-source22/22/zero-gap audit required. Final phase scope and complete04 additionally gate fully archived controls; no filename or SHIPPED envelope substitutes.                                                                        |

**Level4 data flow:** No dynamic UI is introduced. Fixed filesystem sources → presence-preserving map → pure milestone/ownership/audit/certificate state → shared structural/exclusion checks → actual CLI result. Supplied maps use only their own strings. The historical constructor never participates in production selection.

## Actual behavioral evidence

The following commands were executed by root; the verifier independently checked raw receipts/logs rather than rerunning them.

| Boundary                               | Actual result                                                                                                                                                                                                                          |
| -------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `bash scripts/verify.sh`               | `dfbd420c-9a6f-405e-8d24-65c9d9385a08`,17:26:26.639–18:10:46.426Z;2,659,787ms wrapper /2,659,044ms inner; exit0/full/default/no signal. All176 unique ordered stage JSONs equal their raw receipts and lie inside the parent interval. |
| Seven whole test files in full/default | 158610,159637,160742,161149,1622610,163pure124,native160;5,032pass/0fail,16,739assertions.                                                                                                                                             |
| Full native fixture cleanup            | 160owned roots removed;146finite files/3,485,048bytes, corpus `b08539b5ebd94298fc77b615e50df23dbcf82e3625fb9b488f79dc58ee64d6b1`. Canonical verification was absent in this Pending run; do not substitute an older147-file corpus.    |
| Four-world composite                   | 28actual unfiltered whole-file executions;20,128pass/0fail,66,956assertions;24passed raw boundaries. Each world passes the same seven complete files.                                                                                  |
| Other full/default surfaces            | Format, strict all-target/all-feature Clippy, build, workspace tests, CLI tests, benchmarks, six-target Bazel/provenance and pure-core coverage stages all exit0.                                                                      |

Raw full index: `.local/open-bitcoin-dev/phase163/logical-control-full-receipt-index.json`. Actual log: `logical-control-full-verify.log`,1,618,995bytes/16,353lines, independently hashed `e26d1f3216c03480a234cdf03fcf69c903eccdbadfbf1545be510bbf1e47b216`.

Raw composite: `.local/open-bitcoin-dev/phase163/logical-control-matrix-14fa-complete-four-layouts.json`, generated18:48:18.099Z, SHA256 `beb01a4bc4c08cc9571e5dd3be68a12862995f58c38b3719203d17deb45029da`. Independently validated all four raw attempt parents, all28 selected raw test rows/log hashes and unfiltered commands, both manifests per world,24boundary results, and27equal before/after witnesses:21bound inputs plus six fixed active/archive control candidates. Each world's accepted corpora are byte-identical across attempts; different worlds intentionally have different topology.

| World              | Files / bytes  | Matched per-world manifest SHA256                                  |
| ------------------ | -------------- | ------------------------------------------------------------------ |
| Equivalent overlap | 296 /7,206,104 | `da2c43e2809e004da16da885171be30872c29aef3e07e446de4fbc60c0f9de1a` |
| Exact safety-index | 296 /7,175,060 | `c19d9c72edd324fade0c39318ba62f9705540aaf48abe4ee6e3d994e00deb099` |
| Archive-only       | 295 /7,162,554 | `0ca9399827cdcd638dd68ca63129d8211bcd453242b22c2d7c2f5e36d6701de4` |
| Next-active        | 296 /7,162,672 | `2227972acdcfd4fcf253c40beb16f8b757d70bd66f6bbb5e20e32d9510f719b8` |

The composite combines successful158/159 whole files from the parallel attempt with successful160–163 whole files from serial quiet recovery18:11:19.243–18:48:11.255Z. It is not one new parent execution. First sequential scheduling interruption, parallel160 default-five-second timeouts and interruption, and the13,761ms failed focused probe remain nonpassing historical attempts. The same unchanged-limit probe later passed1/0 in357ms; it is diagnostic only and contributes no whole-suite acceptance row. All recorded outer/probe roots were removed; older interrupted children may leave unrecorded inner fixtures, so no global cleanup claim is made.

The real safety hook `672cd52d-2d0b-4a88-a9f7-1a60bda170bc` remains606pass/1fail on prior e0, not a current success. Its log SHA256 remains `baa218a834d4fccaa3f9f229550de6d056e7dce0f5662c527112be987ff064a4`. Preserved e0 reports under `.planning/reports/phase163-e0c-before-logical-control-revalidation/` retain their original dates/source. Native report generation18:12:18Z and later matrix update18:49:30Z are distinct observations; neither is retimed.

All21current input hashes independently match `logical-control-candidate-manifest.json` and ordered digest14fa9cc5…. Mutable reports, live audits and publication prose are excluded from that digest and assessed separately. Fresh review SHA256 `8a4cd0129c919db617d4b86c71d39df851372e86934dacb42008d44c3d982b7b`; security `a62f24cebfbdbba6f25dd1b7e19e3df3edeff9c6e71935510c0a0de062afa2cb`; current native `f286215cf1e240d10356eb423ccc0250b0660bb8ae1406e6ea379393232afc0e`.

## Requirements, anti-patterns and remaining acceptance

| Requirement | Plans | Status                                          | Evidence                                                                                                                                                                     |
| ----------- | ----- | ----------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| CFGR-01     | 01–05 | FUNCTIONALLY SATISFIED; ROOT ACTIVATION PENDING | Historical pinned proof plus SC1–SC4, exact current-source independent/native and archive-layout evidence. Renewed milestone readiness/final completion remain Plan05 gates. |

Only CFGR-01 maps to163; all five plans claim it. No orphan requirement exists. AG01/AF01 are linked audit concerns; TD01/TD02 retain truthful historical reconciliation. No later milestone phase is used to defer a defect.

No blocking placeholder, empty handler or disconnected artifact was found in the inspected policy/fixture modules. Empty arrays/maps are populated containers or failure results, not user-visible stubs. Disconfirmation checked the earlier misleading physical-path mutation and synthetic passing certificates: the former now has explicit positive/refusal replacements; the latter cannot replace actual review/full-native/audit evidence. SC5 remains deliberately partial until its real audit/readiness boundary is earned.

### Human verification required

None for this scoped automated provenance repair. Phase156's original debugger human-confirmation archive condition remains unperformed and its record remains unarchived; no human confirmation or new runtime test result is invented.

### Handoff

Root may now activate CFGR-01 with the canonical in_progress04 carrier and exact closure wording, then run actual activation guards. Fresh independent integration/milestone audit, earned readiness publication/execution and a fresh final SC1–SC5 report must follow. Canonical04/05/global completion and normal full-hook commit/push remain root-owned later actions. Actual archive/tag/shipment and stronger runtime/production/funds claims are not established.

_Verified: 2026-10-13T18:51:46Z_
_Verifier: gsd-verifier_
