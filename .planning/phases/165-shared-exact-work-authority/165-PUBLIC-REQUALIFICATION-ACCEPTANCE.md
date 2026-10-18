---
phase: 165-shared-exact-work-authority
reviewed: "2026-10-17T13:01:10Z"
generated_by: gsd-code-reviewer
phase_lifecycle_id: 165-2026-10-14T16-09-59Z
lifecycle_mode: yolo
review_type: parent-plan15-public-prerequisite-acceptance
status: passed
verdict: PASS
plan28_summary_sha256: b94d8b56ee68206769da4f6206707372012dac2d76e2425e8d3b02411f61c4c1
plan24_selected_header_body_linkage_prerequisite: resolved
plan29_execution: pending
phase_acceptance: pending
findings:
  critical: 0
  warning: 0
  info: 0
  total: 0
---

# Phase 165: Public Requalification Prerequisite Acceptance

**PASS. Plan 28's completed, executed public proof resolves Plan 24 Task 2's selected-header and checked body-ancestor linkage prerequisite.** The acceptance is based on the new combined post-close proof, its actual nonzero execution, independent source review and complete stable matrices. Plan 29 remains unexecuted, so this is not authorization to skip that separate prerequisite or declare Plan 24 ready.

## Accepted Artifact and Contract

The completed `165-28-SUMMARY.md` was read in full after the executor released its heavy verification slot. Its final executor-produced SHA-256 is `b94d8b56ee68206769da4f6206707372012dac2d76e2425e8d3b02411f61c4c1`, lifecycle `165-2026-10-14T16-09-59Z`, mode yolo, generated at 2026-10-17T12:57:41Z, with `## Self-Check: PASSED`. Root independently reported verify-summary with check-count 100 passing 31/31 with no errors; the reviewer did not rerun that tool. Deferred nested commits are not represented as actual commits.

The governing checked Plan 28 is `a8d6d1158e4d688aa43d1321b2babd2abf1584e290a2ae6df3be03be9bc1e697`. The current amended Plan 24 hashes to `b09f2f9a3d2616542b846b7914f92854376f7b199ace0ed793fefcb66abcbe19`. Its Task 2 requires actual selected-header proof and checked body-ancestor linkage in the new network owner after close/requalification, with UNKNOWN historical order and the first direct extension. Header readmission alone or a genuine core handoff alone cannot satisfy it.

The existing Plan 15 constructor is the permitted public mechanism. Its current source, not just its old summary, independently replays actual retained bodies, checks physical content and source identity, initializes a selected graph, links the unique root and uses cached-body validation for historical nonroots. This review accepts that mechanism only in combination with the new executed Plan 28 path. Old maturity 1 graph tests and Plan 25/19 retained-owner receipts are not substituted.

The earlier Plan 28 source review remains CLEAN at `d79ed20c1c1d7f1200c09e85093b37989003c2b77641f4feb5f24e7426099049`. Its exact source, current policy and reviewed pin delta were checked again against actual completed bytes before this acceptance.

## Public Proof Obligations Satisfied

| Required property                                          | Actual source and execution evidence                                                                                                                                                                                                                                                                       | Result |
| ---------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------ |
| Normally compiled public path                              | Same external Cargo integration target; child uses public Node/core APIs and parent public helpers, with no production inclusion or private accessor.                                                                                                                                                      | PASS   |
| Real populated source and all original handles closed      | Genuine model root plus nonroot are accepted and really flushed. Only owned block/snapshot/coin data survives `drop((source, store))`; a fresh Fjall open follows.                                                                                                                                         | PASS   |
| Raw reopened observations stay unearned                    | Temporary raw `from_parent` is checked for !has_earned_work and exact UntrustedWorkLineage, then dropped.                                                                                                                                                                                                  | PASS   |
| Every actual retained body validates under full parameters | Fresh Memory Chainstate loads and validates every saved prefix body in order, root through endpoint, with actual body time/P2SH/full Regtest parameters and maturity 100. Positions/header/parent/work/undo are checked.                                                                                   | PASS   |
| Genuine metadata and complete physical identity            | Target active/undo/logical Some counts derive only from the new genuine Memory snapshot. Complete coins/B/heads match the real Fjall view; the target is initially unearned and strict consuming rebind establishes its live origin and full parameter binding.                                            | PASS   |
| Physical None counts remain None                           | No count writer or normalization is used. Public hydrate and complete_snapshot compare complete saved prefix/undo/coins/B/heads, with all required body/undo records, before and after construction. Saved counts remain None after extension.                                                             | PASS   |
| Selected checked prefix and UNKNOWN nonroot order          | Public header entries and available/nonfailed body facts match the entire genuine prefix. Historical nonroot sequence remainsNone both before and after the new event.                                                                                                                                     | PASS   |
| First future event is direct actual extension              | Child header, body fact and stored body are absent immediately beforehand. The first future operation is public connect_local_block with full selected parameters, without separate Headers/ingress/header writer/cache admission. Accepted height/hash/header/work and unchanged old prefix are asserted. | PASS   |
| Original refusal/ownership controls retained               | Exact registration-only inverse recovers the whole original root. All fourteen original test/helper bodies, four actual canonical refusals and unknown-equal refusal remain byte-identical and run in the full 15-case target.                                                                             | PASS   |
| Independent source controls and stable closure             | 22 unique loaded-source controls, enabled child/root ownership, exact reviewed pin renewal, focused 36-case controls, both complete matrices and all 491 frozen entries pass.                                                                                                                              | PASS   |

This bounded proof uses a model prefix at heights 0/1 and its first direct child at height 2 under the complete Regtest maturity 100 parameters. It does not replace Plan 24's required actual mature-spend suffix, original socket/RPC/transaction assertions, or the new missing-body/physical-identity negatives.

## Actual Nonzero Runtime Receipts

The actual exact name is `requalification::phase165_live_fixture_requalified_prefix_extends_with_normal_maturity`. The reviewer previously read its discovery/result logs and timing records, and rechecked the unchanged public source and full external result for this acceptance.

| Execution                   | Actual result                           | Harness duration | Complete log SHA-256                                               |
| --------------------------- | --------------------------------------- | ---------------- | ------------------------------------------------------------------ |
| Original external target    | 14 passed/0 failed/0 ignored            | 5.93s            | `f4594f0e94bf691bc2b57397a5a3f5e8a4ac39c5075030624d53fed5c697853f` |
| Exact discovery             | One test, zero benchmarks               | Discovery only   | `14da283308cf67c2b94a8912490a02912cf64f8a1059854f494760009532cf94` |
| FIRST new runtime execution | 1 passed/0 failed/0 ignored;14 filtered | 0.82s            | `98bd47adc6b70b740d03b0452a4df66c62bb532b88015be0f564ebeb2aac5ca1` |
| Full external target        | 15 passed/0 failed/0 ignored;0 filtered | 5.66s            | `d75df4f728acae381e77387130d96fca687a665f696ddee3810d7c462629a219` |

These are the authentic `.local/open-bitcoin-dev/phase165-p28-{baseline-external,list,named,external}.log` receipts, not new reviewer execution. The new runtime case was GREEN on its first execution. Initial E0616 discovery compilation is preserved as compile-only history; the exact one-line correction removed the illegal private-field observation without changing production or weakening public binding checks.

Affected formatting, strict all-target/all-feature Node Clippy and build exited 0, with actual command durations 1.936/2.972/0.320s. The focused guard log `.local/open-bitcoin-dev/phase165-p28-targeted-controls.log` records 36 passed/0 failed/235 expectations in 23.88s, SHA `df8162ff4f300f20f1c0c97132e3949d59e483470f016f3e292d69bbc36b1e70`. Loaded-source mutation policy is not mislabeled as execution of mutated Rust binaries.

## Independent Completed Pin and Matrix Checks

The actual post-pin packet `.local/open-bitcoin-dev/phase165-p28-postpin-review.json` hashes to `c8d3beb572454bcd9fb68670d271bf14319916fa2faf850fa04fd85f8c5f0bbb`; its actual array receipt hashes to `76615dcc4aafddcba38982658abdf562a20abf0d080556169b130aae09788173`.

Independent complete-source comparison confirms exactly the approved root identity renewal, one child SOURCE row, one child TEST row and one inventory-literal renewal. The other four owners and all eight public dependencies remain identical to the reviewed packet. SOURCE 135/TEST 71/INPUT 264 retain every old row/order apart from the reviewed root identity; root count 14 is unchanged, child count 1 and input index 250 are exact. Inventory SHA is `967b67cdf5a41019a931d9016b32dfc3c42dff8b07713cdab2d8abee48b1868b`. The real stale-pin diagnostic refused before any source read. No old 164/historical/production renewal occurred.

At 2026-10-17T13:00:29.460237Z, the reviewer independently recalculated every 491 frozen canonical path, presence, byte length and raw hash from the pre-run freeze:489 present and the same 2 permitted absences, all exact. All seven actual result exits and log hashes were independently compared with their complete files and summary lines. The boundary covers fixed checker/factory/corpus/module reads and explicit critical Rust dependencies; it does not claim the entire Cargo compiler dependency closure.

| Complete matrix | Pass/fail | Expectations | Actual wall seconds | Log SHA-256                                                        |
| --------------- | --------- | ------------ | ------------------- | ------------------------------------------------------------------ |
| 164             | 868/0     | 1565         | 347.738682541       | `eb84bb9ad89120eb5d4c4b6a7edf90dfa09239cf65c3d8708c35b39c20e9edd6` |
| 165             | 1719/0    | 6223         | 850.802037083       | `3eb7ac3d43f28e71990d9d9494f24cbdbecfaf234e5c3ed6db58a934ce48b46e` |

Exact164, exact165, breadcrumbs, managed checks and whitespace also exited 0. Managed checks report 1521 scanned files and 0 exceptions/findings. No new ignore/skip is introduced.

| Closure artifact                             | SHA-256                                                            |
| -------------------------------------------- | ------------------------------------------------------------------ |
| `phase165-plan28-full-freeze.json`           | `9a04354b88e1f989b19ea1a6c5d69ec28e881847ade56240ffb34abd045f3a30` |
| `phase165-plan28-matrix-matrix-results.json` | `6533332a6e7fb7457a0e02a4eb9e443722227baa09d6b48c98dae0536e849f62` |
| `phase165-plan28-full-freeze-closed.json`    | `fb8874a9fa3ba8276376b78580217c174b8595233bde820ad716828c4df2c72f` |
| `phase165-p28-root-final-closure.json`       | `799abf6f8f966531e8a9a44967d706b47c120ac03df08b559e8aa6b4c34e20f5` |
| `phase165-p28-root-pin-proof.json`           | `137a7ca1ba880fa671766da8486061856cb8a9f2c9d4b746762dc5b952d2f873` |
| `phase165-p28-root-freeze-proof.json`        | `dae3c526164499cac0c35a94981f204bb37ae285c9d003ac34a77eb1abd0959a` |

All paths in this table are under `.local/open-bitcoin-dev/`. The closing record is dated 2026-10-17T12:56:46.842Z. Its fixed raw corpus 168 entries/4,406,607 bytes has derived SHA `0e4f8440afef4c0c274a0d2ed7452e1fecb2f9ad8d82bf1f3645338733152656`; immutable 162/163 evidence remains separate.

## Current Public and Dependency Identities

All these complete raw files match the final post-pin packet and the 491-entry closure. Current hashes were checked again at 2026-10-17T13:01:10Z.

| File                                                                        | Complete raw SHA-256                                               |
| --------------------------------------------------------------------------- | ------------------------------------------------------------------ |
| `packages/open-bitcoin-node/tests/phase165_live_fixture.rs`                 | `1c38f95572bae47ef45753fe3990d1517142d05821498c91bcae1599566f9ca9` |
| `packages/open-bitcoin-node/tests/phase165_live_fixture/requalification.rs` | `5594a7f2ada84b0bda3fab8fb102834d5002b40e64f17f35fe258e6ebef92f22` |
| `docs/parity/source-breadcrumbs.json`                                       | `4fe89200d14b415d2ad4d14e842d7281e1af3d19dd376b0460bb31e1fad35061` |
| `scripts/check-phase165-shared-exact-work.ts`                               | `b37cc5399386eefcf0a749075c5d48b5d07cb030539a20760768c0942d7cf402` |
| `scripts/check-phase165-shared-exact-work/contracts.ts`                     | `502dc79ab6b5a7264aaf4dbaf62b534982a12195af23f46f99d77360022cded4` |
| `scripts/check-phase165-shared-exact-work.test.ts`                          | `d1f595f02d18a630a769135a593ced452c63d0f43608d674433c66c3ce227037` |
| `packages/open-bitcoin-node/src/sync/live_fixture.rs`                       | `4da004e8b12a1b6bd0da8d460dde94b70a7c1119893fbe03ba2d048d707b9b9c` |
| `packages/open-bitcoin-chainstate/src/engine/work_authority.rs`             | `6c3412159f23e761a283adcccbaf45af3b0cf9aca62af607184d5fc612785dbb` |
| `packages/open-bitcoin-node/src/storage/fjall_store/coins.rs`               | `39838f28879656e526093cee21b94ba08541b9a1f774c76da6093e05547e1c65` |
| `packages/open-bitcoin-node/src/storage/coins_view.rs`                      | `f71ff78ac7a7c4e65d657964a7f415e3c11e9684388fb4456c93a974f1c5ee56` |
| `packages/open-bitcoin-node/src/network/work_authority.rs`                  | `a0952b43f0f4e6eb598421abfb6e6c2e8d01d30ebbb224ba80bd8c6fe8f71447` |
| `packages/open-bitcoin-node/src/network/candidate_eligibility.rs`           | `14e3334a388bdde9037fc1521a86e8a42d3ede428fc0fff8b4fe5be078b1ca1a` |
| `packages/open-bitcoin-node/src/network/runtime_authority.rs`               | `65835d0307dd58ee1572efab8a8eb83794cc210c1748ad87f5218bb3cf082212` |
| `packages/open-bitcoin-network/src/header_store/eligibility.rs`             | `751b989eec04f97e280a0eccc2dc234a6fa413bd0804e5a3d1e802b0f120cc9c` |

Source changes after these observations require affected review and receipt validation. A later coordinator append linking this acceptance must preserve the accepted completed Summary's recorded execution/source facts; the hash above identifies the authenticated executor-produced Summary snapshot.

## Disposition and Remaining Blockers

**The Plan 15/Plan 24 selected-header/body-ancestor prerequisite is resolved. There are no open blockers within Plan 28's accepted public proof.**

Plan 24 is still gated on the separate, checked Plan 29 extraction actually executing and producing its reviewed passing Summary. Current29 plan SHA is `4cf40fa4f86a20df17fb4ef67d491f817b0bde4317b7337a0e52e05a523fe898`; `165-29-SUMMARY.md` is absent at this acceptance. No Plan 29 execution, data-move acceptance or Plan 24 consumer readiness is inferred.

Plan 24 must then execute its own full consumer behavior, actual post-reopen mature-spend path and required exact missing-input/identity negatives. Final Plan 14 default native verification, pure-core coverage, workspace/Bazel/benchmark gates, whole-phase code/security/formal/lifecycle acceptance and normal-hook commit/push remain outstanding and parent-owned. No phase or requirement completion is declared. CWOR-04 remains Phase 166.

All four ordinary canonical selectors remain refused, networking remains default-off, and canonical bootstrap/exact reopen/genesis BASIC, contextual completeness, deterministic restart tie parity and actual pinned-Knots daemon differential qualification remain in their later scopes. This local public model proof establishes no such qualification.

## Review Boundaries

This assigned acceptance used read-only source/report/log/Git-object/hash inspection. The reviewer ran no project imports, tests, build, Cargo, Bun or Bazel commands, and changed only this new acceptance report. Repo instructions, Bright Builds sidecar, placeholder-only overrides, applicable architecture/code-shape/testing/verification/language standards and the fully loaded active lessons informed the review. The actual execution remains attributed to the executor, with independent receipt/source authentication here.

_Reviewer: gsd-code-reviewer acting as the independent Parent/Plan 15 prerequisite checker._
