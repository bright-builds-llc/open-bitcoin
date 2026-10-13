---
phase: 163-archive-safe-parity-provenance-and-milestone-handoff
generated_by: gsd-security-auditor
lifecycle_mode: yolo
phase_lifecycle_id: 163-2026-10-12T21-09-41
generated_at: "2026-10-13T17:30:55Z"
status: verified
asvs_level: 1
threats_total: 16
threats_closed: 16
threats_open: 0
unregistered_flags: []
verification_scope: declared-mitigations-after-logical-control-test-repair
source_digest: 14fa9cc585c9cf8d9b2c0ae04d7f92e6411b38fb7d19128359126eee2735f977
source_inventory_count: 21
candidate_revalidation: pending
prior_source_digest: e0c16fb204664274c63ea67d8b7c653dbe08a63bdc4c09d4e18fa8ecf139d72d
prior_security_generated_at: "2026-10-13T04:18:03Z"
---

# Phase 163 security assessment after logical requirements repair

**16 declared threats CLOSED; 0 OPEN; ASVS level 1.** This independent assessment verifies the declared mitigation implementations on the exact 21-input source inventory below. It does not certify the pending whole-layout matrix, default native run, formal closure, renewed audit, readiness or final normal-hook Git outcome. Every registered disposition is `mitigate`; no risk acceptance or transfer is used.

Material guidance: repo AGENTS and Bright Builds sidecar, placeholder overrides, standards index and architecture/testing/verification/TypeScript guidance, GSD secure-phase instructions and all five PLAN threat registers. Both active lesson files were read completely (7,188 bytes; 2,397 conservative estimated tokens); no lesson archive was loaded. This auditor owns only this report and changes no implementation, publication, control, tracker or Git record.

## Source comparison and trust boundary

All 21 manifest files were loaded and raw-byte hashes independently recomputed; the LF-joined ordered `path:sha256` digest is `14fa9cc585c9cf8d9b2c0ae04d7f92e6411b38fb7d19128359126eee2735f977`. Comparison with pushed source `2a7920757ca93a1debee9ba1bdcee51fd808210c` proves exactly two bound paths changed:

- `scripts/check-phase158-validated-reorg.test.ts:424–467`: removes requirements from the physical CHECK_FILES deletion loop and adds four explicit logical requirements controls using the existing synthetic historical transition builder. Valid archive fallback with absent active requirements passes; both missing candidates, empty active and malformed active `v2.x` fail with targeted diagnostics. All physical fixed-evidence tests remain.
- `scripts/check-phase162-integrated-filter-proof.ts:378–386`: only the expected SHA256 literal for the corrected Phase158 test changes to `efa71956ef2e4916a94790f9f9855a418e8f6d4b84c953db126654e89fbd85dc`. Exact string comparison confirms the file otherwise equals its accepted predecessor; no frozen comparison or failure branch was removed.

The other 19 bound inputs remain byte-identical, including the production resolver, shared Phase158 checker, helper/native fixtures, all 159–161 checkers/tests, verifier routes, contracts, approved context/scope and both original audit snapshots. No new runtime, network, credential or production-datadir effect is introduced. The added test surface maps to existing T-163-01/02/05/06/07/10; no unregistered threat is required.

Logical availability and publication authority remain distinct. In the exact real safety-index topology, deleting the retained active requirements can select all archived controls and legitimately refuse stale unarchived publication wording. The new positive uses a synthetic historical transition with active roadmap retained, isolates requirements availability, and does not exempt such stale claims. Production `select()` still distinguishes absent from invalid-present active evidence, compares all relevant candidates, and selects only fixed paths.

## Preserved evidence and current stage

The [prior e0 security report](../../reports/phase163-e0c-before-logical-control-revalidation/163-SECURITY.md) remains byte-exact: 19,628 bytes, SHA256 `1762177298de89abce4588a2d718cd9c8407812515560928e074eb1ffb8b3e05`, generated 2026-10-13T04:18:03Z. Its prior tests, native/audit chronology and 16-threat result belong to its original source and observation. No prior report or receipt is retimed into this assessment.

The actual failed archive safety hook `672cd52d-2d0b-4a88-a9f7-1a60bda170bc` remains a failure: **606 pass, 1 fail, 607 Phase158 tests**. Its log at `.local/open-bitcoin-dev/phase163/archive25-package/safety-commit-hook.log` was independently read and hashed: 210,180 bytes, SHA256 `baa218a834d4fccaa3f9f229550de6d056e7dce0f5662c527112be987ff064a4`. The failed physical-requirements assertion remains visible at log lines 2412/2455. The [new debug record](../../debug/phase163-logical-control-mutations.md) preserves the root cause, the first incorrect v2.6 negative sample and corrected focused result. A canonical later milestone remains valid; malformed `v2.x` is the refusal case.

At this observation CFGR-01 is Pending, canonical 04/05 are in_progress, native acceptance is pending, current formal verification is withdrawn and the live audit is gaps_found. Old completion cannot activate the new source. Root owns subsequent full verification, formal activation, independent integration/audit, publication and Git. No actual archive/tag/shipment is certified here. [Phase156's retained human-confirmation condition](../../debug/phase156-automatic-writers.md:104) remains unsatisfied.

The two original audit hashes remain `f14fa21dff9e340c9794f6190925fc42075f5624d930e551f0d5666b702697d8` and `f60ef2fa6eea08b3fe00b241a5f2ae47c9f7ea864f1649ff865d7476d43b2709`. Independent recomputation of the six historical Phase162 source rows still yields `a4b0d71d01def301ba406d6b46c351ed8944d8d4508182ffb0dfd21960043575`.

## Threat verification

Aliases: `policy` = `scripts/check-phase163-provenance.ts`; `shared` = `scripts/check-phase158-validated-reorg.ts`; `guard162` = `scripts/check-phase162-integrated-filter-proof.ts`; `fixtures`, `helper`, `native` = corresponding files under `scripts/check-phase163-provenance/`. Each row is classified by its PLAN disposition before verification. Plans04/05 share T-163-16; there are 16 unique threats.

| Threat ID | Category | Disposition | Status | Current evidence                                                                                                                                                                                                                                                                                                       |
| --------- | -------- | ----------- | ------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T-163-01  | S/E      | mitigate    | CLOSED | policy:87–173,252–286 retains fixed v2.5 identity, 22 unique requirement/roadmap owners, consistency and conflict refusal. New158test:437–466 distinguishes valid absent-active fallback from missing-all/invalid-present refusal.                                                                                     |
| T-163-02  | T        | mitigate    | CLOSED | policy:12–22,60–62 binds all 21 inputs; guard162:378–386 retains all three frozen hashes. Independent inventory, exact one-literal pin delta and original six-root digest checks pass. native:199–200 retains source tamper mutations.                                                                                 |
| T-163-03  | R        | mitigate    | CLOSED | policy:10–11,177–199 preserves exact immutable original audit lineage, CFGR-01/Phase162/AG01/AF01 and approved distinct lifecycle identities; both snapshot hashes independently match.                                                                                                                                |
| T-163-04  | D        | mitigate    | CLOSED | policy:7–9,66–84,155–173,287–289 retains finite candidates, first-frontmatter strict duplicate/shape checks and 300-character diagnostic bounds. fixtures:38–59 bounds 32 imports/400 evidence files. No discovery fallback is added.                                                                                  |
| T-163-05  | S/T      | mitigate    | CLOSED | fixtures:95–104 labels synthetic receipts; :201–211 executes actual finite entrypoint bytes through map/filesystem/CLI boundaries with 30s/256KiB bounds. native:26–51 retains all 10 layouts and nested archive/next-active controls. New logical tests use explicit transitionSources, not live completion receipts. |
| T-163-06  | I/E      | mitigate    | CLOSED | shared:37–41,135–138,250–256 preserves supplied-map key presence with no disk rescue. native:243–273,426–438 retains omitted-map-key checks while valid disk evidence exists. Invalid-present active requirements still refuse before archive fallback.                                                                |
| T-163-07  | T/D      | mitigate    | CLOSED | fixtures:24–35,164–194 witnesses all six control presences/bytes, requires regular mandatory proof, constrains owned paths and cleans in finally. native:54–80 and:458–467 retain mutation-witness and throwing-action cleanup checks. New158fixtures use the existing owned temporary-root/afterEach cleanup.         |
| T-163-08  | R        | mitigate    | CLOSED | verify.sh:155–156,351–352 retains exactly one documented and executed route for each 163 suite. guard162 contracts and tests:553–575 retain native removal/duplication/replacement/order refusals. Full default route and normal hook remain required, never replaced by focused tests.                                |
| T-163-09  | R/S      | mitigate    | CLOSED | Prior e0 report and failed hook log hashes independently verified; new debug record retains first failed negative sample and corrected execution. Prior source passes remain historical. HANDOFF:28 and Phase156 debug:104 preserve the unperformed human condition.                                                   |
| T-163-10  | E        | mitigate    | CLOSED | policy:293–299 permits only exact state-authorized claims; shared:141–155 scans unsupported/excluded claims independently. native:127–148,371–378 retains premature/excluded/stale publication and SHIPPED-envelope refusals. Logical availability does not bypass publication authorization.                          |
| T-163-11  | T        | mitigate    | CLOSED | Approved root-only shared ownership remains in Plans04/05 execution contracts. Current CFGR-01 Pending, 04/05 in_progress, withdrawn formal report and gaps_found audit preserve staged reopening; originals and other source bytes remain intact. Auditor modifies only SECURITY.md.                                  |
| T-163-12  | S/R      | mitigate    | CLOSED | policy:207–234 checks canonical report paths, independent generators, source/lifecycle, CFGR-01 criteria, functional versus final scope, actual full-native/security and 04 activation. New source cannot reuse old digest-bound completion.                                                                           |
| T-163-13  | T/E      | mitigate    | CLOSED | Independent two-file comparison and single pin-literal equivalence pass. Plans04/05 preserve summary-before-checkbox, independent byte acceptance and renewed native/formal guards. All original proof hashes remain valid; this report supplies no substitute native/activation result.                               |
| T-163-14  | R/T      | mitigate    | CLOSED | policy:177–186,237–246 preserves original gaps and new 22/22/zero-blocking-gap/source-bound audit gate. Plan05Task1 still requires independent integration and explicit AG01/AF01/TD01/TD02 dispositions. Current gaps audit cannot authorize readiness.                                                               |
| T-163-15  | E        | mitigate    | CLOSED | policy:280–284,293–299 separates passed-audit readiness from final archival authorization and excludes inferred capability. Normal .githooks/pre-commit still generates index LOC and invokes literal default scripts/verify.sh. This audit performs no archival or hook bypass.                                       |
| T-163-16  | S/E      | mitigate    | CLOSED | policy:218–234 preserves canonical04 execution stage and distinct functional/final criteria. Plans04/05 require actual Task1/Task2 receipts and final reconciliation at the existing canonical paths. Current 04/05 remain in_progress; report presence alone grants no advancement.                                   |

## Actual lightweight checks and remaining gates

Pinned Bun1.3.9 (`cf6cdbbb`) was used for two independent auditor runs:

- `bun test ./scripts/check-phase163-provenance.test.ts`: **124 pass, 0 fail, 260 assertions,464ms**. Includes fixed archive selection, invalid-present active evidence, conflict, missing-map input, stale-source/generator/lifecycle, open-security, failed-native and premature publication refusals.
- `bun test ./scripts/check-phase158-validated-reorg.test.ts --test-name-pattern '^logical requirements'`: **4 pass, 0 fail, 606 filtered, 4 assertions,920ms**. Explicit fallback, missing-all, empty-active and malformed-active cases pass their targeted assertions.

All five SUMMARYs were inspected for dedicated `## Threat Flags`/`threat_flag` entries; none exists. Their bounded local process, fixture and publication surfaces remain covered by the 16 registered threats. Accepted risks: none. Transferred risks: none. Unregistered flags: none.

No Cargo, Bazel, whole legacy layout matrix, full default verifier, fresh formal verifier or milestone audit was run by this auditor. Root's four-layout matrix and default native work remain in progress at this assessment; no future result is asserted. Fresh source-bound full/formal/audit/readiness and normal full commit-hook acceptance remain necessary before completion.

## Independently verified 21-input inventory

The digest hashes ordered LF-joined rows without a trailing LF; all SHA256 values are raw bytes. Mutable reports, live audits and publication records remain excluded to avoid circular binding.

```text
scripts/check-phase163-provenance.ts:cf22edda320107ce0dfda9a8692e8c29aaa436ad6ca1f82c48f66b1d778c67d7
scripts/check-phase163-provenance.test.ts:cdbffaf4922bdcd70a40d49f32552085519812a3c06b9ebda113054f3580cb5f
scripts/check-phase158-validated-reorg.ts:1df027aca2deb9827f135d9a558ef2377d7a2ebb18b830ba76f41698376834de
scripts/check-phase158-validated-reorg.test.ts:efa71956ef2e4916a94790f9f9855a418e8f6d4b84c953db126654e89fbd85dc
scripts/check-phase162-integrated-filter-proof.ts:61c1919cc35f388b8d462ed28d8042cf33d5e5752afcc5d43fea79817d46ad97
scripts/check-phase162-integrated-filter-proof.test.ts:01e84e45886a44d020ffabfebf32d8c898affc3afe1e35613478d38bd870cbc5
scripts/check-phase159-filter-rpcs.ts:563ec1eb3694e9c3961f705f250596ea68292fc67158f8c85375f07f08184a32
scripts/check-phase159-filter-rpcs.test.ts:8c3591fcd8f4d1e523d77fbd85e4c0d9d50a27e14925ed8876879d1c380451b3
scripts/check-phase160-peer-filter-serving.ts:41f9054550790bf2f0263cedb4f0dd74c964a29a6fc4b8a492b1f17fa6cdbc05
scripts/check-phase160-peer-filter-serving.test.ts:38b3fd04eb9bbafb1ca2d3b75eb5880ec1f75f147f71836ccb1840680edfbee8
scripts/check-phase161-operator-index-evidence.ts:c3447ae4c4cc07c8a9d2de712b663520aab545f990730ec77e3e93ca7e93ce23
scripts/check-phase161-operator-index-evidence.test.ts:05a59352ca6c1641690c94ece985644fd5ae1f72d8cb32cfdf455bdaf85e61dc
scripts/check-phase163-provenance/native.test.ts:f3750b40ca00a3e2faa38df1e0af24536ba152bcd7141e686d67d1c1a0a5bae9
scripts/check-phase163-provenance/fixtures.ts:40a76924ac64135f08f3482c50f8aafd1b5d98fa6d4dfd7052b87e0b6a492615
scripts/check-phase163-provenance/fixture-controls.ts:0395ada68a3f154dacfeaea41dbd444bdf1864ee908931be5f4c3588ef761968
scripts/check-phase162-integrated-filter-proof/contracts.ts:b5e5df1f4dc8705cdbe1ca85f9d9049a6e0188653311e00f2cacabb9f88daa00
scripts/verify.sh:cd051fd967934b85a7489f75ad8793cc4880ef918ced776b179a8ff7fb3165bf
.planning/phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-CONTEXT.md:a766eda3b17389bdfd390775c04085ee756c402d00a918be3fd6505c287d7956
.planning/phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-GAP-SCOPE.md:26b0ac69e42cf3eeeef175ae49f698c3a373b933b933249e5752ad942d801274
.planning/phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-ORIGINAL-MILESTONE-AUDIT.md:f14fa21dff9e340c9794f6190925fc42075f5624d930e551f0d5666b702697d8
.planning/phases/163-archive-safe-parity-provenance-and-milestone-handoff/163-ORIGINAL-INTEGRATION-AUDIT.md:f60ef2fa6eea08b3fe00b241a5f2ae47c9f7ea864f1649ff865d7476d43b2709
```
