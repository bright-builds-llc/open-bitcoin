---
status: resolved
trigger: "Find and fix Phase163 final normal-hook failure using GSD-debug."
created: 2026-10-13T03:53:08Z
updated: 2026-10-13T06:32:25Z
---

## Current Focus

hypothesis: Historical Phase162 fixtures retain later live milestone audits and Phase163 reports that conflict with restored original Phase162 ownership.
test: Parent runs fresh full verification and independently rebinds formal evidence to the changed source digest before retrying the normal hook.
expecting: Complete source-bound native verification and final normal hook pass on the new source bytes.
next_action: Parent owns full verification, report reconciliation, staging and commit/push; this debug session remains verifying pending that real workflow evidence.

## Symptoms

expected: Original Phase162 implementation and formal historical fixture states pass strict provenance checks independently of later live Phase163 closure.
actual: The final normal pre-commit hook rejected three Phase158 fixture tests while the implementation source remained unchanged.
errors: T-162-10: v2.5 provenance: archive readiness requires new-owner closure; consequent unsupported claim diagnostics in README, packages/README and parity surface.
reproduction: Pinned Bun 1.3.9 test scripts/check-phase158-validated-reorg.test.ts filtered to filesystem-loader historical states and original implementation fixture ownership restoration.
started: The live v2.5 milestone audit became passed and Phase163 formal reports became present after historical fixture tests were introduced.

## Eliminated

- hypothesis: This is an unrecognized recurring debug knowledge-base pattern.
  evidence: The only stored entry concerns CLI stdin/network watchdog behavior, with no matching fixture/provenance keywords.
  timestamp: 2026-10-13T03:53:08Z

- hypothesis: The failures require the full-hook runtime or an intermittent scheduler/network condition.
  evidence: All three cases reproduce in the isolated pinned-Bun command in 286 ms; the identical command passes after changing only test fixture construction.
  timestamp: 2026-10-13T03:55:15Z

- hypothesis: Strict production provenance guards need weaker readiness exemptions.
  evidence: Unmodified production guards accept all three corrected historical fixtures and 77 adjacent valid/refusal controls.
  timestamp: 2026-10-13T03:55:15Z

## Evidence

- timestamp: 2026-10-13T03:53:08Z
  checked: final-commit-hook-3.log and complete historical fixture construction in scripts/check-phase158-validated-reorg.test.ts.
  found: fixture() copies all live optional provenance roots before the historical tests overlay a smaller normalized map; transitionSources(maybeBase) also clones every supplied live root before restoring Phase162 ownership.
  implication: Both construction paths can retain a passed milestone audit claiming later closure while requirements are restored to original Phase162 ownership.

- timestamp: 2026-10-13T03:53:08Z
  checked: Focused pinned Bun 1.3.9 reproduction, saved in .local/open-bitcoin-dev/phase163/fixture-leak-red.log.
  found: Exit 1, zero passes, three failures, 604 filtered tests, three assertions, 286 ms. Both filesystem cases produced the exact archive-readiness/new-owner-closure rejection; the supplied-map case returned invalid instead of implementation.
  implication: The failure reproduces without the full hook or concurrent source mutations.

- timestamp: 2026-10-13T03:54:12Z
  checked: The same focused command after test-only fixture isolation, saved in .local/open-bitcoin-dev/phase163/fixture-leak-green.log.
  found: Exit 0, three passes, zero failures, 604 filtered tests, eleven assertions, 446 ms. Optional-loader key equality and restored-map key equality assertions also pass. Test-file SHA256 is c5581d74952b15e2dea98d817e4364362256dabc8b135bfcbb5a5f579c4566b2; owned-file whitespace diff check passes.
  implication: Removing later optional metadata from historical fixture construction resolves both filesystem and supplied-map failures without changing production checks.

- timestamp: 2026-10-13T03:55:15Z
  checked: Adjacent historical implementation/formal valid states, source binding, premature completion, excluded claims and formal report refusals on pinned Bun 1.3.9.
  found: Exit 0, 77 passes, zero failures, 530 filtered tests, 80 assertions, 7.68 s in .local/open-bitcoin-dev/phase163/fixture-leak-adjacent.log. Markdown check passes. No shared reports, staging, commits or production source files were changed.
  implication: The narrow fixture correction preserves existing invalid-state refusal behavior. Full live verification is deliberately left to the parent after digest/report rebinding.

## Resolution

root_cause: Historical fixtures overlay or clone live optional metadata while restoring original Phase162 ownership, leaving an incompatible later passing milestone audit and Phase163 reports.
fix: Restrict transitionSources supplied maps to the fixed historical source set; have fixture materialize an exact supplied map or its unchanged current live default; replace partial filesystem overlay with direct historical-map materialization and assert exact optional/root key sets.
verification: Exact three failing cases reproduced before the change and pass afterward on pinned Bun 1.3.9. Parent must bind fresh full verification and formal reports to the changed implementation digest.
files_changed: [scripts/check-phase158-validated-reorg.test.ts]

## Plan

- [x] Capture the three original focused failures.
- [x] Restrict historical source maps and filesystem fixtures to their explicit source universe.
- [x] Rerun the same focused cases and inspect the exact owned-file diff.
- [x] Return the changed source hash to the parent for fresh full verification and formal evidence rebinding.

## Receipt Hashes

- RED log SHA256: d59d1d85fca6f8fe1176fcf8414d3cf53e3084d578a6beb4d60a169081e469e4.
- GREEN log SHA256: 8818c8c2d3f875f549670c997bb0a396629f906363e858defb4e7ecd5e856aef.
- Adjacent historical log SHA256: adf8616180d24284a1c884fba7daed85a7f3dc7b4e225a9a318148e66c60b5fc.

## Simplification Review

The fix removes the partial overlay and redundant deletion paths. Both in-memory and filesystem historical fixtures now use an explicit source universe; the default live fixture still reads the same current required and optional files. Production provenance checks are unchanged. The three originally failing tests remain the regression guards and now additionally assert source-set identity.

## Scope and Guidance

Only this debug session and scripts/check-phase158-validated-reorg.test.ts are owned. No production guards, reports, audit publication, staging or commits are authorized here. AGENTS.md, AGENTS.bright-builds.md, standards-overrides.md, testing, verification, code-shape and TypeScript standards informed fixture isolation and verification. Both active lesson files were read fully (7,188 bytes total); archive lessons were excluded. The parent coordinates the shared checkout and all source-bound final evidence.

## Frozen-pin compatibility follow-up | 2026-10-13T03:58:25Z

Independent review identified Phase162 checker’s strict frozen hash for the repaired Phase158 test. Root refreshed exactly that expected SHA256 literal to c5581d74952b15e2dea98d817e4364362256dabc8b135bfcbb5a5f579c4566b2; no production guard logic changed. Final candidate digest is7ed874f5808f89c0a9031ea843023845177d0c22c488650c6e826a00e2ad1987, with exactly two changed bound files against344: historical test isolation and its expected hash pin. The intermediate0285digest was never native-certified. New full/default evidence and formal activation remain pending.

## Actual corrected-source closure | 2026-10-13T06:32:25Z

The final21-inpute0c16fb204664274c63ea67d8b7c653dbe08a63bdc4c09d4e18fa8ecf139d72d is accepted by actual full6run38c0607f/all176stages0, actual activation2ea6/284tests0, renewedsource-bound22/22audit/31seams/18flows, readinesscb2bbcae/284tests0 and freshfinal06:23:00Zcanonical5/5verification SHA1d6ec7db836a006c631664b988cf2a60cd06daab313dce714212edef87c8b73c. Earlier RED/intermediatehash/timeout/full-interruption evidence retains actualscope/dates above. This sourcebug is resolved; rootnormal-hookcommit/push is still a separate savinggate. This file remainsunarchived. OriginalPhase156humancondition is separate/unperformed and unchanged.
