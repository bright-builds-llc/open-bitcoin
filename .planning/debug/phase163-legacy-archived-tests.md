---
status: resolved
trigger: "GSD-debug bounded archive testhelper repair for Phase158-160 historical test fixtures"
created: 2026-10-13T04:07:00Z
updated: 2026-10-13T06:32:25Z
---

## Current Focus

hypothesis: Historical test builders assume active v2.5 requirements and roadmap, although standard archival removes or replaces those controls.
test: Frozen test sources pass selected historical cases and invalid-state controls; after parent pin refresh, strict production boundaries pass in all three layouts.
expecting: Parent full native verification and independent acceptance bind the final shared candidate bytes.
next_action: Hand back frozen test sources and receipts; parent owns full native verification, independent review/security, report rebinding and actual-workflow confirmation.

## Symptoms

expected: Historical implementation, formal and Phase163 Pending fixture tests run from all supported v2.5 control layouts without changing live controls.
actual: Archive-only selected legacy tests fail before production checks because active REQUIREMENTS is absent; next-active content cannot supply the historical v2.5 baseline.
errors: ENOENT active .planning/REQUIREMENTS.md; undefined active map entry .replace.
reproduction: /tmp/open-bitcoin-archive25-legacy-preflight.ts uses isolated archive-only tree and selected Bun test-name patterns.
started: Standard v2.5 archive removes active requirements and replaces active roadmap with an index stub.

## Eliminated

- hypothesis: Runtime guards lack archive-only support.
  evidence: Existing preflight records all six Phase158/162 map, filesystem and CLI archive boundaries passing on the same isolated tree.
  timestamp: 2026-10-13T04:07:00Z

## Evidence

- timestamp: 2026-10-13T04:07:00Z
  checked: .local/open-bitcoin-dev/phase163/archive25-legacy-test-preflight.md and complete historical fixture builders in Phase158-160 tests.
  found: Phase158 and Phase160 transitionSources and Phase159 inline transition directly read active controls; Phase158 gapPendingSources dereferences an absent active requirements map entry. Raw optional-source loaders already support archived controls.
  implication: Normalize test-only historical controls before restoration, without weakening production selectors or raw negative maps.

- timestamp: 2026-10-13T04:10:00Z
  checked: Same selected implementation, formal, filesystem-loader, original-owner and gap-Pending cases using /tmp/open-bitcoin-legacy158-160-layouts.ts on pinned Bun 1.3.9.
  found: Final frozen-source run passes 27 cases and 66 assertions across gap-pending active, archive-only and next-active layouts. Each of nine child subprocesses exits zero without a signal or timeout. Archive-only retains absent active REQUIREMENTS, next-active retains v2.6 controls, and fixture-owned cleanup/live-byte assertions pass. Corpora are bounded to 260/266/267 files and 13 test imports.
  implication: Historical builders now select archived v2.5 controls without mutating real controls or accidentally using v2.6 data.

- timestamp: 2026-10-13T04:12:00Z
  checked: Adjacent source/hash/lifecycle/formal-gate refusals, absent and empty supplied evidence, unauthorized Pending claims and empty-active-control presence on pinned Bun 1.3.9.
  found: 54 pass, zero fail, 176 assertions, 8.35 seconds. Scoped diff whitespace check passes.
  implication: Repair retains strict missing-source and invalid-control refusal behavior while preserving historical fixture source isolation.

- timestamp: 2026-10-13T04:13:00Z
  checked: /tmp/open-bitcoin-legacy158-160-boundaries.ts after parent refreshed the strict Phase158 test and Phase163 policy pins in the Phase162 checker.
  found: All 18 production checks pass: Phase158 and Phase162 map, filesystem and actual CLI boundaries on active gap-pending, archive-only and next-active layouts. Corpora contain 142/148/149 files respectively; owned fixture cleanup and live-byte guards pass. Final source hashes remain unchanged.
  implication: The test-only repair is compatible with strict runtime provenance and frozen source identity; no production guards were relaxed by this owner.

## Resolution

root_cause: Historical test fixtures require active-only control files after supported milestone archival.
fix: Use pure test-only v25FixtureSources before fixed historical filtering and owner restoration. Normalize only MD/JSON claim views so raw bound source bytes survive; use the fixed optional inventory for exact loader key equality; preserve exact supplied-map materialization and raw runtime defaults.
verification: Prior archive RED receipt demonstrates active-only setup failures with six passing real archive boundaries. Frozen source passes 27 selected three-layout cases, 54 adjacent refusal cases and all 18 strict production boundaries after parent pin refresh. Diff whitespace and Markdown checks pass. Full native verification and independent acceptance remain parent-owned.
files_changed: [scripts/check-phase158-validated-reorg.test.ts, scripts/check-phase159-filter-rpcs.test.ts, scripts/check-phase160-peer-filter-serving.test.ts]

## Plan

- [x] Load repo guidance, relevant standards and both active lesson inputs; inspect existing RED receipt.
- [x] Coordinate shared pure test-only selector and repair historical Phase158-160 builders.
- [x] Verify selected active, archive-only and next-active cases and meaningful adjacent refusals.
- [x] Review scoped diff, simplify, record file hashes and hand back evidence for parent full verification.

## Source Freeze and Receipts

- Phase158 test SHA256: 6526c6d4c1853b4bbf21e462b8dc292eeb95a6c10e493e9d0cbbf449f43c01cb.
- Phase159 test SHA256: 8c3591fcd8f4d1e523d77fbd85e4c0d9d50a27e14925ed8876879d1c380451b3.
- Phase160 test SHA256: 38b3fd04eb9bbafb1ca2d3b75eb5880ec1f75f147f71836ccb1840680edfbee8.
- Existing RED report .local/open-bitcoin-dev/phase163/archive25-legacy-test-preflight.md SHA256: 56ab8ac9dafed8aba51726d040c160a45bd7215bcbc2b63114a2c23779ee62b1 (unchanged).
- GREEN JSON .local/open-bitcoin-dev/phase163/legacy158-160-layouts-green.json SHA256: b071ba87f0cd8d1a5b8a6dcd5e13a95ac2ec025254a52ffb496051e555f7f40e.
- GREEN log .local/open-bitcoin-dev/phase163/legacy158-160-layouts-green.log SHA256: 5d4305dc954f81696489cd7579df9d0da9fdb9457e5133a48cc9f402e20516d3.
- Adjacent log .local/open-bitcoin-dev/phase163/legacy158-160-adjacent.log SHA256: 169e5eb54773822969b5e1fdec32f8ab2781214976663dcbd7d38707cb11b5e1.
- Production boundary JSON .local/open-bitcoin-dev/phase163/legacy158-160-production-boundaries.json SHA256: 9fedc789e6f8f68dcff78fb120ab548c235549f40d942d3f2d00df2f70968315.
- Production boundary log .local/open-bitcoin-dev/phase163/legacy158-160-production-boundaries.log SHA256: 40c56b1c0da7fe2cf33a9934061306ea1264e3de25fc4af9c4163a5716b1047a.

## Simplification Review

One shared selector replaces active-only reads; historical filtering remains explicit. Removed redundant Pending cleanup now handled by the shared helper. Exact optional-key equality now derives its expected set from the fixed inventory and supplied map presence, eliminating dependence on the live layout. No production guards or raw live fixture loaders changed.

## Scope and Guidance

Own only this file and scripts/check-phase158-validated-reorg.test.ts, scripts/check-phase159-filter-rpcs.test.ts, scripts/check-phase160-peer-filter-serving.test.ts. Preserve prior three-case historical fixture isolation and exact key assertions. No staging, commits, actual archival, shared control/report writes or heavy verification. Parent coordinates final digests, strict pins and full native verification. AGENTS.md, AGENTS.bright-builds.md, standards-overrides.md, code-shape/testing/verification/TypeScript standards inform the fix. Both active lesson files were read completely (7,188 bytes total); archives excluded. Existing knowledge base has no keyword match. The parent already entered the GSD-debug workflow.

## Actual corrected-source closure | 2026-10-13T06:32:25Z

The final21-inpute0c16fb204664274c63ea67d8b7c653dbe08a63bdc4c09d4e18fa8ecf139d72d is accepted by actual full6run38c0607f/all176stages0, actual activation2ea6/284tests0, renewedsource-bound22/22audit/31seams/18flows, readinesscb2bbcae/284tests0 and freshfinal06:23:00Zcanonical5/5verification SHA1d6ec7db836a006c631664b988cf2a60cd06daab313dce714212edef87c8b73c. Earlier RED/intermediatehash/timeout/full-interruption evidence retains actualscope/dates above. This sourcebug is resolved; rootnormal-hookcommit/push is still a separate savinggate. This file remainsunarchived. OriginalPhase156humancondition is separate/unperformed and unchanged.
