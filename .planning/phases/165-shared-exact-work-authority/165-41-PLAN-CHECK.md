# Plan 165-41 — Independent Bounded Plan Check

## VERIFICATION PASSED

- **Status:** PASS; 0 blockers, 0 warnings, 0 info findings.
- **Scope:** One inert two-owner repair proposal; static plan analysis only. This is not SOURCE CODE CLEAN, ordinary-row approval, native success, phase acceptance or execution release.
- **Lifecycle:** `yolo`, `165-2026-10-14T16-09-59Z`.
- **Review gate:** First independent review, with the sole canonical scope-reference correction authenticated below. No substantive revision remains outstanding.

## Authenticated proposal and references

Bytes were authenticated before parsing their contents.

| Artifact | Bytes | SHA256 |
| --- | ---: | --- |
| `.local/open-bitcoin-dev/phase165-plan41-proposal-PLAN.md` | 20688 | `4df35e97558c6b1e5f9222dcc3a3f4802f7e57963bc398dc68c56f7bc614a323` |
| `.local/open-bitcoin-dev/phase165-plan41-proposal-SCOPE.md` | 10383 | `6954a622ca0f5361856d84c7673849370fa7688f371c0324d55d043d2fd0e446` |
| `.local/open-bitcoin-dev/phase165-plan41-proposal-reference-correction.json` | 43681 | `e164396979036b9c5dc07cf948bb68d57faad5d02543ee257af16401aa7807f2` |

The correction packet independently reconstructs complete original proposal `3828df2b8cd128fea4527af26a820c3039ad8a11c888870beb3596cabba6a3e6` and complete final proposal in both directions through one unique hunk. Only the context reference changes from the local SCOPE proposal to its intended tracked destination, `.planning/phases/165-shared-exact-work-authority/165-41-HISTORY-DIAGNOSTIC-REPLAN.md`. Both Task XML bodies remain exact. Root must install that scope with the checked plan after the execution preconditions hold.

Authenticated current source is still entry `ac44b9e9fb32ebb4545fc588d03963aeb6694cd00d1b270254728c2a2521a48f` (15166 bytes) and table host `dcdbe558d3db1d7256682593f5b2273c80394ab558254f85037b2525a60ecb0b` (49625 bytes). Existing Plan 40 is `172366e9de847ee6d23cda234895824a2e3945d8c9ba04f4c735fec43a6e65c3`; accepted Summary 40 is `80e36efda7d8a5419d1bb7d85aa2a89b139d480355c76971bad10c59c11cf40b`; Context is `a271033b0827becc15b6fec4b700283a611249f1ec3339cb3d40a30dc3b03ed2`.

## Goal-backward assessment

The phase goal remains one exact target-derived authority across header, connected and reorg selection with safe legacy refusal. Plan 41 repairs the evidence guard that prevents the existing four-owner regression from establishing the required independent refusal diagnostic. Its requirements `CWOR-02`, `CWOR-03`, `CWOR-05` match that bounded purpose. `CWOR-01` remains covered by the original arithmetic plans and final Plan 14; it is not silently removed from phase coverage. The complete existing plan frontmatter union covers all four roadmap requirements. `CWOR-04` remains Phase 166.

| Dimension | Result | Evidence |
| --- | --- | --- |
| Requirement coverage | PASS | Task 1 restores refusal evidence and preserves current qualification; Task 2 establishes actual unchanged full regression/CLI evidence. Original plans and final native retain the broader authority obligations. |
| Task completeness | PASS | Read-only GSD `verify plan-structure` returned valid, 2 tasks, all Files/Action/Verify/Done fields present, errors/warnings empty. Task 1 provides explicit behavior, implementation and intended RED/GREEN outcomes. |
| Dependency correctness | PASS | Proposed complete 41-plan graph is acyclic and every wave equals maximum dependency wave plus one after the Root-owned amendment: 40/wave39 → 41/wave40 → 14/wave41. Plan 41 is not runnable concurrently with active R4. |
| Key links | PASS | The existing imported `historicalToolingView(inputs)` is called independently for its diagnostic; its returned historical Map is discarded. The existing gated `checkCurrentClaims` import really is defined in old164 `claims.ts`; the proposal links that actual owner correctly. Local `checkWorkClaims` is a separate unchanged call. |
| Scope sanity | PASS | 2 implementation tasks, exactly 2 existing source/data owners. One wrapper removal/dedentation and one established ordinary digest value are permitted. No module, test, inverse, Rust, graph-registration or pin changes. Existing caps remain required. |
| Verification derivation | PASS | Must-have truths describe observable diagnostic/refusal behavior and current qualification boundaries, with concrete artifacts and wiring. No artifact-only success criterion replaces behavior. |
| Context compliance | PASS | Scope ledger covers D-01–D-12 by implementation or explicit preservation. D-04/07/09/10/11/12 are directly sustained; original arithmetic/selection behavior remains owned by prior plans. No deferred 166–168 work enters this plan. |
| Scope reduction | PASS | No mocked Map, skipped assertion, new inverse, selected-only completion, daemon qualification or later-phase claim substitutes for the decisions. |
| Nyquist | SKIPPED | `workflow.nyquist_validation` is explicitly false. Both tasks nevertheless contain concrete automated commands. |
| Cross-plan data contracts | PASS | Source CLEAN/Root acceptance precedes new ordinary-row derivation; complete host before/after and inverse precede activation. H, adapters, historical snapshots, manifest and descriptor contracts remain exact. |
| AGENTS compliance | PASS | GSD workflow, pinned Bun/direct-file commands, full native/normal hooks, per-command evidence, generated LOC ownership, no overlapping Cargo/Bazel and stop-before166 rules remain explicit. |
| Research resolution | PASS | Existing research Open Questions each carry RESOLVED markers; this repair requires no new dependency or unresolved design decision. |

## Root-cause and authorization-boundary review

Current `checkHistory` first calls `historicalFixtureAdapter`, catches its refusal, and later suppresses `historicalToolingView` behind `maybeHistoricalAdapter !== undefined`. The unchanged test at line 338 expects a MANIFEST_FILE/history finding naming the mutated copier. Removing only that last wrapper reaches H's independent manifest/hash/binding/snapshot/restoration checks even after adapter refusal.

H's `legacyToolingBase` returns unknown copier source unproved; its binding fingerprint then refuses before any historical view is returned. The new independent call does not qualify the copier or execute imported source. Keeping both `return maybeHistoricalAdapter !== undefined` and caller `if (historyBound) checkCurrentClaims(text, fail)` exact preserves the existing current-classification authorization. No H or historical inverse renewal is justified by this repair.

The retained reproduction result `987904fca7a1716b69393189a815bd09d8fd90aad1201a7d29d50de0fc07ba34` was authenticated before parsing, as were its log `0e0ed05ea2514c29224c9d268e9bd618053d231b8751501477bfc30586c99f1f`, before receipt `08d12f18fbd98c2f00c215a09c328fa48e75fc9667939204fefb529907916777`, and after receipt `cf7725528d3234bf02e7d98aba3426fd42208225479864cfccb5122c95fc0104`. Actual outcome is exit1, 0 pass/1 fail/1965 filtered, 90 assertions, 2.12s. Independently compared all 8322 observations in each receipt, including nested before/after canonical path, presence, bytes and SHA fields; every row is equal and the two complete file maps match. This remains failed focused evidence, not final native acceptance.

## Required execution sequence and final boundary

1. Root must retain R4's actual terminal failed outcome, complete log/output receipts and complete failed-generation Raw closure before installing or editing anything. R1–R4 and earlier failed collection/bootstrap evidence remain immutable. Planning-time active-run observations do not supply final counts or timing.
2. Capture full authentic two-owner before bodies and unchanged H/adapter/test-loop, original14 Task XML, other plan and Root metadata guards. Apply only the permitted source wrapper/dedentation. Obtain independent SOURCE CODE CLEAN and Root acceptance before deriving any new ordinary identity candidate.
3. Retain a genuine real-current CLI stale-row ownership refusal after CLEAN. Then independently review the inert complete-host one-row proposal and unique inverse. Root alone activates it. Preserve the other191 SOURCE rows, all88 TEST owners, all307 ordered inputs and pin `625975bfe720b4d08d05429a31a0c2dd0d76cb044c0675617754ffa4cc1ce6bb`; no host self-row or masked-map GREEN is allowed.
4. Independently review fresh graph/read union, immutable drivers and complete source/metadata deltas before the actual four-owner loop, full165 1966 cases, full164, historical203, direct CLIs, breadcrumbs, managed and whitespace checks. Authenticate complete original identities, actual argv/exits/logs/timings and canonical before/after/output-future closure for every command, including intended RED and failed setup. Finish with the freshly derived complete85 eligible source CLIs.
5. Authenticated existing R4 preflight inventory `17a5210f6dc76d8cbec826ce7697507f6a09023fbb34ebbeb3e5319013043157` contains87 registrations/85 eligible and exactly benchmark-report/Bazel-provenance exclusions. Plan 41 requires independent fresh agreement and exact argv, rather than borrowing those counts.
6. Root accepts the closing generation before Summary41. Installing41 changes progress to39/41 phase and52/54 milestone; accepting Summary41 permits40/41 and53/54. No phase/requirement completion or Git action follows from this check.
7. Root alone amends final14 frontmatter/context to wave41/depends_on165-41. Current14 whole SHA is `3d883d80cae9eb6b64daabf124f0b16085fa7c7255e8f9c33503563e05f1677b`; authenticated original Task XML remains3412 bytes/`f73b7127d8992cbd9b10bf362c48f7682c021a355d3febd11b06333765e5b9a4` and3359 bytes/`a4706372cbd14638ae042a6675ab637a8b6679709e708b9b9b7ce096d6e69b5e`. All other39 plans remain exact.
8. Plan14 still requires an ENTIRE fresh default native R5, including all full suites, Rust/build/test/benchmark, fresh actual eight-crate coverage, Bazel and provenance, followed by canonical code/security/formal/lifecycle and normal-hook finalization. R5 collector/source exceptions require separate independent review; this check approves no tool implementation or broad source exception.

The hardest constraint—independent diagnostic reachability without current-qualification credit—is addressed in Task1 with the original gate preserved. Premature frozen-input writes, early pin renewal and selected/mocked completion are each caught by explicit entry/review/whole-matrix gates. The proposed source change is the smallest direct repair and adds no abstraction or dependency.

Material guidance read: repo AGENTS and Bright Builds sidecar, placeholder-only overrides, standards index, architecture, code-shape, verification, testing and TypeScript pages; complete active global/repository lessons7188 bytes/2397 conservative estimated tokens, no archives. Neither project-skill directory exists. This checker ran no application, test suite, source CLI, Cargo, Bazel or native command; only readonly inspection/GSD structure validation and this owned report write occurred.

## Structured issues

```yaml
status: passed
plan: 165-41
blockers: 0
warnings: 0
info: 0
issues: []
execution_release: false
native_acceptance: false
phase_acceptance: false
```

Plans verified within this bounded scope. Root may install/release only after the terminal R4/full failed-generation closure gate, with the exact checked proposal and all subsequent source/data/tool gates retained.
