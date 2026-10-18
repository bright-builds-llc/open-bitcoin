---
phase: 165-shared-exact-work-authority
plan: "16"
subsystem: fixture-integration
tags: [earned-work, basic, benchmark, fixture-eligibility, evidence-guards]
requires:
  - phase: 165-15
    provides: Genuine same-core Fjall fixture constructor with explicit parameters
provides:
  - Empty-source benchmark transport and actual first-body connection
  - Original-owner CLI BASIC consumers and genuine earlier-body equal-work replacement
  - Exact raw reopen observations and unchanged-state work refusal
  - Repaired claim, bounded-manifest and named-test source guards
affects: [165-17, 165-24, 165-14]
tech-stack:
  added: []
  patterns: [Explicit fixture parameters, Genuine pending-body release, Separate raw observation]
key-files:
  created:
    - scripts/check-phase164-canonical-identity/source-files.ts
    - scripts/check-phase165-shared-exact-work/test-support.ts
  modified:
    - packages/open-bitcoin-bench/src/cases/sync_runtime.rs
    - packages/open-bitcoin-bench/src/cases/storage_recovery.rs
    - packages/open-bitcoin-cli/tests/operator_flows/domain_fixtures.rs
    - packages/open-bitcoin-cli/tests/operator_flows/basic_index.rs
    - scripts/check-phase164-canonical-identity.ts
    - scripts/check-phase164-canonical-identity/contracts.ts
    - scripts/check-phase164-canonical-identity/historical-tooling.ts
    - scripts/check-phase164-canonical-identity.test.ts
    - scripts/check-phase165-shared-exact-work.ts
    - scripts/check-phase165-shared-exact-work/contracts.ts
    - scripts/check-phase165-shared-exact-work.test.ts
key-decisions:
  - Keep benchmark core empty until actual scripted body intake.
  - Retain the same live CLI owner through all connects, pruning and equal-work replacement.
  - Earn B400 preference through actual pending-body release before A400 acceptance.
  - Authenticate bounded manifest bytes before parsing and evaluate claim predicates within their own subject scope.
requirements-completed: []
requirements-contributed: [CWOR-02, CWOR-03, CWOR-05]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 165-2026-10-14T16-09-59Z
generated_at: "2026-10-17T04:34:32Z"
client_date: "2026-10-16"
duration: approximately 34min
completed: 2026-10-16
tasks_completed: 3
files_modified: 13
---

# Phase 165 Plan 16: Genuine Benchmark and CLI Fixtures Summary

**Actual empty-source transport and same-owner BASIC consumers now exercise derived work, real earlier-body eligibility, equal-height replacement and raw reopen refusal.**

## Delivered Scope

All three tasks are complete. The original eight paths expanded to thirteen through explicit parent approvals recorded in 165-PLAN-CHECK.md. The approved plan identity remains 23457ba216ffca1733c82f7e8b6b91ca4c153ffb235a018a9603a5025ab425d2. No shared state, requirements, roadmap, configuration, tracker, hooks or Git index were changed by this executor.

The benchmark transfers a genuine empty core into Fjall and uses Plan 15's public constructor with BASIC Disabled and explicit default parameters. Root admission occurs outside P2P. Header scripts deliver child, grandchild and duplicate nonroot grandchild in three actual rounds, retaining max_rounds=4 and height 2. The summary reports the final round: one header and three messages. Exactly two GetHeaders requests and three Version handshakes prove the actual rounds; the duplicate peer advertises the already reached height and correctly triggers no third GetHeaders. The body case begins with no active tip, receives the first actual genesis Block after handshake, and checks actual height/hash/derived proof, one received body and exact durable body equality.

CLI fixtures preserve maturity 1 and their historical and same-block spend construction. Actual managed genesis acceptance and an empty-header real flush feed the SAME Fjall core into normal BASIC Enabled assembly. The existing two-argument helper remains compatible with all three captured-export callers; serving policy uses existing explicit arguments. Future headers enter through modeled Version/Verack and two separate contiguous messages: A1–A400, then B400. No root P2P header or second fixture binding is used.

Checked duplicate genesis preserves the active snapshot and supplies linkage only. The exact deterministic B400 body is persisted and retained as NonExtending before any A child. A399 releases it before later A400 intake; actual A400 acceptance captures the live binding. Replacement loads the retained precomputed bytes, passes ZERO as the untrusted carrier and proves equal derived work and height after the genuine reorg. All connects, index turns, pruning and replacement occur on the original owner before every handle closes.

Consumer checks retain catching/current/automatic-target/lag/ahead/durable transitions; physically present body serving; actual prune at height 20 body and undo absence; wallet refusal; NotFound and no DurableBlock; displaced immutable filter rows; and exact BASIC observations after real all-handle reopening. Raw reopened cores refuse a later connect with UntrustedWorkLineage while complete work and physical snapshots remain equal and no refused body appears.

## Executed Evidence

All Cargo commands used pinned Bun1.3.9 command-timings wrappers and Rust1.94.1, serially. Ignored logs use the prefix .local/open-bitcoin-dev/phase165-plan16-. First timed execution began 2026-10-17T04:00:03.622Z.

| Gate                                                | Actual result                                                                                                                                           |
| --------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Initial benchmark repeatability                     | RED: 0 passed / 1 failed, exit101                                                                                                                       |
| Initial CLI BASIC consumers                         | RED: 1 passed / 5 failed, actual UntrustedWorkLineage                                                                                                   |
| Discovered repeatability target                     | One exact named test listed                                                                                                                             |
| Final repeatability                                 | 1 passed, 7.88s                                                                                                                                         |
| Complete benchmark library                          | 16 passed / 0 failed, 12.66s                                                                                                                            |
| Actual benchmark binary smoke                       | Exit0; 15 registered cases across 11 groups; JSON/Markdown reports; no Knots source supplied                                                            |
| Discovered CLI BASIC consumers                      | Six actual cases listed, including three captured-export callers                                                                                        |
| Final CLI BASIC consumers                           | 6 passed / 0 failed / 0 ignored, 14.49s                                                                                                                 |
| Workspace format check                              | Exit0, 1.738s timed wrapper                                                                                                                             |
| Node/bench/CLI strict all-target all-feature clippy | Exit0, 17.205s timed wrapper                                                                                                                            |
| Node/bench/CLI all-target all-feature build         | Exit0, 12.245s timed wrapper                                                                                                                            |
| Downstream source mutations                         | 45 real owner/anchor mutations plus unchanged baseline in each checker; focused Phase 164: 46/0 and Phase 165: 81/0 before later claim/helper additions |
| First complete164 matrix                            | 841 passed / 0 failed; 1,399 expectations; 341.99s                                                                                                      |
| First complete 165 matrix                           | 1,165 passed / 0 failed; 4,369 expectations; 294.77s                                                                                                    |
| First frozen layout                                 | All 233 input/checker/module hashes unchanged                                                                                                           |
| Post-helper focused controls                        | 69 passed / 0 failed; 122 expectations; 14.79s                                                                                                          |
| Final complete 165 matrix                           | 1,165 passed / 0 failed; 4,369 expectations; 293.93s                                                                                                    |
| Final frozen layout                                 | All 234 input/checker/module/helper hashes unchanged                                                                                                    |
| Exact checker entrypoints                           | 164: 135 Rust inputs/477 edges; 165: 89 owners/30 enabled-test owners; both exit0                                                                       |
| Breadcrumbs / managed / diff                        | 1,201 Rust breadcrumbs; zero managed findings/exemptions; diff clean                                                                                    |

The smoke output is local benchmark evidence, not a pinned-Knots comparison. No complete workspace/default native verifier was run by this plan; final Plan 14 owns that gate.

## Approved Deviations and Preserved Diagnostics

1. **[Rule1 — raw benchmark contract]** After sync cases advanced, repeatability and actual smoke exposed the previously masked storage-recovery.restart-reopen assertion. Raw stored numeric entries correctly have no accepted best tip. The added storage_recovery.rs ownership now checks exact count and full entry equality, including numeric carriers, absent accepted tip, height -1 and empty accepted chain. Other persistence assertions remain intact; no recovery authority is created.
1. **[Rule1 — WR-01 claim scope]** Eight actual appended-claim controls initially escaped, preserving all disclaimers. The source rule now evaluates every status predicate within its own known subject span, preserves compound phase/No scope, separates JSON string leaves and reports malformed JSON. Mixed subjects, unrelated negation/conditions, comma-free tails, legitimate denials and nested catalog claims are controlled. Two later achieved/available regressions also produced actual RED before bounded vocabulary repair. Parent and independent reviewer checked the staged design.
1. **[Rule1 — WR-02 import boundary]** The current historical-tooling adapter previously imported JSON before bounded loading. Two extra current-adapter paths were approved. Twelve ordered literal path pairs now define the inventory; bounded original manifest SHA authentication precedes local parsing and pair comparison. Fresh isolated imports copy all 14 real CLI modules and prove zero implicit data import, unchanged inventory, manifest-specific missing/symlink/size refusal and policy rejection of malformed/redirected/extra/reordered bytes. Original historical data and its expected SHA remain unchanged; only the reviewed current adapter hash was renewed.
1. **[Rule1 — WR-03 named attributes]** An actual second-test control demonstrated first-occurrence replacement could mutate the wrong test. Mutations now splice the computed attribute range, assert earlier bytes are unchanged and verify the intended named owner. The combined initial offset/import RED had three actual errors.
1. **[Rule3 — static inventory capacity]** Parent approved source-files.ts extraction before full matrices. Serialized SOURCE_FILES 135, EDGES 477, TEST_MODULES 26 and ordered INPUT_FILES 219 were independently identical before/after. Extraction alone did not renew the inventory pin.
1. **[Rule3 — test helper capacity]** Parent approved test-support.ts extraction only after both first matrices and their 233-hash closure. Its 14-module list and normalized callable body remain identical; it performs no import-time I/O. The first focused extraction run was 62 passed/7 errors due to a missing dirname import; the corrected run was 69/0. There are 33 actual line-anchored test declarations before/after. The earlier receipt's 34 counted all textual test( occurrences and has an appended correction. Dynamic complete 165 execution remains 1,165 cases.

The first stable layout and final layout are separate receipts. Comparison proved the only changed existing path after the first full run was the 165 test file, plus the new helper. Therefore only the affected complete 165 matrix was repeated. Its final full-source hash map is .local/open-bitcoin-dev/phase165-plan16-final-stable-inputs.json, file SHA 210f266fc9699a3d1792c6355e38f62752508aed49647c62fed962998d6a70ea. Final comparison records 234 checked paths and zero drift.

The current ordered 219-input pin is 9656243abbd04bbb6e3ea6d78c8bcbb78a4312e434b8667e7edef25178fd2472. New current-owner inputs/edges were added only for the actual reviewed fixtures; original historical 162/163 receipts, preservation source, manifest, corpus and all six native registrations remain intact.

## Review and Simplification

The parent independently approved plan15/16 design and actual predecessor GREEN. The preliminary integration/source review and final guard review are recorded in 165-PRE-REVIEW-INTEGRATION16.md and 165-PRE-REVIEW-GUARDS.md; earlier findings and hashes remain historical attribution. The guard reviewer performed source review, not the executed tests reported here.

The simplification pass keeps one genuine returned core, one explicit parameter helper, one deterministic replacement builder and the existing public handshake/source/flush APIs. Small test-only source assembly avoids duplicating runtime policy. Static inventories/helpers moved without changing values or tests; no new dependency, public production capability, validator, options hierarchy or exemption was added.

AGENTS.md, Bright Builds guidance and relevant architecture, code-shape, verification, testing, Rust/TypeScript rules informed execution. Both active lesson files were loaded completely (7,188 bytes / 2,397 estimated tokens); no project skills or new lesson trigger applied. Every owned Rust/TS file is within 628 lines. Stub scan found no TODO/FIXME/placeholder or unfinished implementation. No new undeclared endpoint, auth, schema or production file-access surface was introduced; the approved guard changes cover their bounded input boundary. No authentication gate occurred.

## Final Source Identities

| Owned path                                                        | SHA-256                                                          |
| ----------------------------------------------------------------- | ---------------------------------------------------------------- |
| packages/open-bitcoin-bench/src/cases/sync_runtime.rs             | 62668b6f6627ef7b7d7ebb376c16d453748afbeeea78cc02bb8e6d0fdf3902cd |
| packages/open-bitcoin-bench/src/cases/storage_recovery.rs         | 43eb1347f17f63431f98a542355a8352f2d51e5b34c7293f8057fe9c5eed116b |
| packages/open-bitcoin-cli/tests/operator_flows/domain_fixtures.rs | fd0934bb72ed28362477092f6216b1aa4a78b093cce453d29f756e22f3276525 |
| packages/open-bitcoin-cli/tests/operator_flows/basic_index.rs     | d3b6c196c8ca8cca19d2ba64ae778e71b7207843bab7923a04781c1f008a99bc |
| scripts/check-phase164-canonical-identity.ts                      | 1d2990c348e4e9ef367f38320475266e300cbdab98de38693b67abc9d3e5e0f8 |
| scripts/check-phase164-canonical-identity/contracts.ts            | d0d547888bc3ff98bae7b3f0b643c4fe48fad6275bb5a7f70e86227c2207afd1 |
| scripts/check-phase164-canonical-identity/historical-tooling.ts   | bf7dfd86a92628b70de4e6099573786f56c9dc7a1f611c3ac8ca10fe26be8f9e |
| scripts/check-phase164-canonical-identity/source-files.ts         | 5cd178af07ea7f27204a2586d94bf8b1731dc6fd6b32c7b1ded0e07b6fc32832 |
| scripts/check-phase164-canonical-identity.test.ts                 | fb0a15cd8e318e2d1d89d80abee594e48068f1604496c484d1b3d8dbb27ccb23 |
| scripts/check-phase165-shared-exact-work.ts                       | 570f4ff3bd5b3611295b379113e85af28877ff61be1bec43cd81d2b9e0f8be75 |
| scripts/check-phase165-shared-exact-work/contracts.ts             | 0d298a49c5d09a231dc938cbaf2cbb483a612f86e27850c0fd381530cada54ee |
| scripts/check-phase165-shared-exact-work/test-support.ts          | 19cfa9b55b5a46be045fa6a7d6a24e79ffdc558290ed5a7227b5d512c04e6f6a |
| scripts/check-phase165-shared-exact-work.test.ts                  | 3ce93d309a534e5ad10cfdff6f058ef69026e898b817c918cd6644b8b3b4fd26 |

## Task Commits and Parent Gates

All RED/GREEN/task/metadata commits are deferred to the parent's strict single-phase transaction after plans 17–24, final 14 native closure and whole-phase review/security/formal gates with normal hooks. This executor staged, committed and pushed nothing; no commit hashes are invented. Base remains d0104a997c0362bf65b81de1c8fec8ddc3012a1b.

Plan 16's targets are GREEN. The original Plan 14 diagnostic remains 249 failures in 46 shared fixture roots; no current remaining workspace failure count is inferred from these focused repairs. Later repair plans and default native verification remain required. Phase 166 retains canonical bootstrap, exact all-handle authority and genesis BASIC; 167 retains contextual completeness; 168 retains actual pinned-Knots qualification. No later phase was executed.

## Self-Check: PASSED

All thirteen owned implementation paths and this summary exist. Both complete guard layouts, final 234-path identity, Rust behavior/strict compile checks and final policy checks are verified. All owned files satisfy the cap and no blocking stub remains. Shared phase state and final Git saving remain parent-owned; no executor commit is expected.
