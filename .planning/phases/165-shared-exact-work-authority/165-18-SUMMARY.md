---
phase: 165-shared-exact-work-authority
plan: "18"
subsystem: fixture-authority-integration
tags: [earned-work, basic, fault-boundaries, retention, header-checkpoints, source-guards]
requires:
  - phase: 165-17
    provides: Genuine two-block managed fixture adapters and verified current guard layout
provides:
  - Genuine same-owner catch-up, writer and RPC read-fault prerequisites
  - Actual accepted unflushed suffix with preserved durable and safe checkpoint boundaries
  - Genuine empty and normal-maturity indexed header-checkpoint fixtures
  - Eight reviewed source owners and three attributed loaded-source controls
affects: [165-19, 165-20, 165-14]
tech-stack:
  added: []
  patterns: [Genuine source before faults, Same owned core, Actual accepted payload retention]
key-files:
  created: []
  modified:
    - packages/open-bitcoin-node/src/sync/tests/filter_index/catch_up.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/catch_up/failures.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/catch_up/rpc_faults.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/evidence/accepted_faults.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/evidence/retention.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/operator_status.rs
    - packages/open-bitcoin-node/src/sync/runtime_state/tests/header_checkpoint.rs
    - packages/open-bitcoin-node/src/sync/runtime_state/tests/header_checkpoint/faults.rs
    - scripts/check-phase165-shared-exact-work.ts
    - scripts/check-phase165-shared-exact-work/contracts.ts
    - scripts/check-phase165-shared-exact-work.test.ts
key-decisions:
  - Reuse the genuine prior-prefix adapter only when its full parameters match.
  - Keep the distinct RPC Regtest parameter selection through same-core BASIC assembly.
  - Preserve physical safe15 before normal startup acknowledges durable19.
  - Retain later body and undo bytes from actual same-owner acceptance without moving durable B19.
  - Change only the actual later-successor status caller and preserve other status setup.
requirements-completed: []
requirements-contributed: [CWOR-02, CWOR-03, CWOR-05]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 165-2026-10-14T16-09-59Z
generated_at: "2026-10-17T05:36:20Z"
client_date: "2026-10-17"
duration: approximately 30min
completed: 2026-10-17
tasks_completed: 3
files_modified: 11
---

# Phase 165 Plan 18: Genuine Fault and Retention Prerequisites Summary

**Same-owner accepted prefixes now reach the original writer/read faults, unflushed retention boundaries and real header-checkpoint effects without importing raw work authority.**

## Delivered Scope

All three tasks are complete within the fifteen declared implementation paths. Eleven paths changed. The three declared current Phase 164 paths and breadcrumb registry remain unchanged because none of these eight existing Rust owners was previously in that fixed inventory and no new Rust file was needed. Approved Plan 18 identity is cc60412c879cf0e74dddba973b8ebbfff7ae4b046cfee3dc1a356fd286489521.

Catch-up, accepted writer/body faults and the small retention case reuse Plan 17's actual two-block live adapter under its full maturity-one parameters. Genuine managed acceptance, same-core real flush and normal BASIC setup precede every original measured fault. Requested-body setup inspects the already captured two-header graph and confirms the future body is absent; the original handshake, successor Headers, real GetData, intake, error and conservative reopen assertions remain. No missing future body is restored.

RPC faults retain their distinct full Regtest parameters, including their retarget selection. The approved existing Plan 15 factory accepts and flushes exactly the old two-block prefix. Its same source bundle enters the public constructor; temporary factory/store handles are dropped. The pending successor is accepted on that original owner. Existing immediate/final read, publication, MissingTarget, backend distinction, cancellation, poison and actual all-handle observation cases remain intact.

Retention now genuinely accepts and flushes only through height 19. Its original records 0..15 are published against that actual checkpoint before normal BASIC assembly. An added physical safe15 assertion precedes the constructor; an added safe19 assertion follows normal startup acknowledgement. Later 20..400 are real same-owner accepts, with body and undo retention taken from those actual accepts. Original processed400/safe19/B19, paired-delete refusal, Periodic/Always cadence, zero-record release, final paired deletion and reopened B400 assertions are unchanged. The original large automatic-retention case retains its setup and actual volume threshold.

Only the status test that accepts a later successor selects the genuine adapter. The existing opened helper and other disabled, transition and read-only callers retain their original setup. The equal-height status premise remains Plan 20-owned.

The checkpoint-local wrapper transfers genuine empty ownership before header or acceptance effects and gives its own temporary names an Atomic suffix. The shared runtime helper is untouched. The pressure case preserves its initialized zero-cache lifecycle and threshold, transferring genuine empty ownership into that exact target. Indexed stale-header setup accepts actual normal-maturity 100 bodies on the same Fjall source before Plan 15 BASIC assembly; its captured graph is inspected rather than rebound. Original snapshot bytes/counters, skipped-turn fault consumption, typed backend failures, NeverConnected/header-only classification, missing-header refusal and final raw observations remain.

## Executed Evidence

Pinned Bun 1.3.9 PATH was supplied to every Bun/Cargo command. Rust 1.94.1 Cargo commands ran serially through command-timings. Substantial Bun matrices began after Cargo finished. Resumable sessions were polled within 60s; CPU liveness was observed during the large retention cases. No process was suspended or terminated. Ignored logs use the prefix .local/open-bitcoin-dev/phase165-plan18-.

| Selected Rust corpus                                 | Final actual result          | Libtest duration |
| ---------------------------------------------------- | ---------------------------- | ---------------- |
| catch_up direct phase157_turn cases                  | 12 pass / 0 fail / 0 ignored | 25.11s           |
| catch_up failures                                    | 3 pass / 0 fail / 0 ignored  | 6.17s            |
| RPC fault subtree, including existing captures child | 14 pass / 0 fail / 0 ignored | 14.11s           |
| accepted_faults                                      | 4 pass / 0 fail / 0 ignored  | 9.38s            |
| full retention, including original large-volume case | 3 pass / 0 fail / 0 ignored  | 190.85s          |
| exact later-successor operator status                | 1 pass / 0 fail / 0 ignored  | 0.69s            |
| full header_checkpoint subtree                       | 12 pass / 0 fail / 0 ignored | 7.27s            |

Final Rust session 33030 recovered exit 0 with 49 selected cases. After narrowing the status change to its one intended caller, that queued exact case compiled and passed on the narrowed source; final strict session 71691 also recovered exit 0. Workspace format check, Node all-target/all-feature clippy with warnings denied, and Node all-target/all-feature build passed. Final clippy reports 6.63s (6.752s timing wrapper); build reports 0.08s (0.280s wrapper).

| Source/policy gate                              | Actual result                                                                    |
| ----------------------------------------------- | -------------------------------------------------------------------------------- |
| Fresh baseline seven Rust children              | Each exit 101;18 pass /31 fail /0 ignored,49 selected                            |
| Initial unchanged volume case                   | Pass,185.65s within the full retention baseline                                  |
| Fixed inventory RED after exact owner additions | Exit1 before source reads                                                        |
| Focused current fixture controls and baseline   | 15 pass /0 fail;132 expectations;9.65s                                           |
| Complete current Phase 164 matrix               | 841 pass /0 fail;1,399 expectations;345.41s                                      |
| Complete current Phase 165 matrix               | 1,281 pass /0 fail;4,725 expectations;482.70s                                    |
| Exact current checkers                          | 164:135 Rust inputs/477 edges;165:106 owners/47 enabled-test owners; both exit 0 |
| Breadcrumbs                                     | 1,201 existing Rust paths; exit 0                                                |
| Managed policy                                  | 1,521 inputs;zero findings/exemptions; active lesson check passed                |
| Whitespace and stable closure                   | Exit0;all251 saved hashes unchanged                                              |

Frozen aggregate session 50339 recovered exit 0. Its complete matrices and policy checks ran from 2026-10-17T05:22:23.142Z through closing comparison 2026-10-17T05:36:12.657Z. The map at .local/open-bitcoin-dev/phase165-plan18-stable-inputs.json has SHA-256 16cf9d1e8b0a5a1f4431725d7dc3c12a19936a58953e35e4b8b25f7e98f4c9a3. Its separate closure receipt records 251 checked paths and zero drift. Every current input, both test files, CLI modules and existing test-support helper are included.

The final Rust logs are final-0 through final-6 under the stated prefix. Original RED logs, first repaired execution, retention setup diagnostics, final strict logs, full164/full165, checker, breadcrumb, managed and closing receipts remain separate. Timing evidence starts 2026-10-17T05:06:52.925Z. No complete workspace/default native, coverage or Bazel run is claimed here.

## Source Renewal and Historical Preservation

Exactly eight reviewed current Phase 165 owner identities and their 57 statically recognized enabled test declarations were appended. The 57 declarations are distinct from 49 selected Rust executions: nine other status cases were not selected, while the existing RPC captures child contributes one selected case. Full operator_status and inherited readiness closure remain Plan 20 gates.

Three new actual loaded-source controls replace unique present anchors: restoring a raw all-handle reopen before RPC successor acceptance, arming the publication fault before normal constructor setup, and omitting the pressure target's genuine empty-source transfer. Each checks an unchanged passing baseline and an attributed work-owner finding. Their literal table remains in already-owned contracts.ts; the existing executable loop has a generic current-fixture label. These source-policy mutations are not relabeled executed Rust semantic mutations.

The new 236-input ordered pin is 63330bb1ac8a01eae6948efb3db1e6bf627210568bca52ac5d474c7f00590228. Removing exactly the eight new owners reconstructs the prior 228-input pin 97d735fbc6761933aeb9a50024dd5433da74afb3a6ca86649982ffb4eb0c8f64 and preserves every prior relative position. The registration receipt verifies that reconstruction and the earlier 219-input order. Existing missing/extra/reordered/redirected inventory controls ran in the complete matrix.

Current 164 contracts/source-files/test remain byte-identical to their completed Plan 16/17 identities. Historical 162/163 recipes, manifests, corpus, original receipts and guarded source pins remain intact. All six default native registrations, claim/JSON/manifest/named-attribute controls and isolated CLI modules remain unchanged. Preservation source stays 20,688 bytes / SHA e6cb5740cb5c83655aed14c43b1206847d264431d944741a365771c3e8d76f5b, with original 20,556-byte / SHA 3d004f91bd72a0f3902588a36fca0438b04b5ac034bc9e227893664496835630 attribution separate. Strict support_runtime.rs remains 5a25857c552bb7dcfdd040ad15e19c4616f52ef8b35df69b4f11a42dcd5b1f05. No count normalization, broad digest renewal, raw promotion, future undo in source state or restoration of removed/pruned history was added.

## Diagnostics, Deviations and Simplification

- **[Rule1 — temporary path collision]** The checkpoint baseline also exposed shared-name temporary-directory collisions. The approved local wrapper adds Atomic uniqueness before calling the unchanged shared helper; the complete checkpoint corpus passes 12/12.
- The new retention helper initially expected a post-constructor cursor15. Actual normal startup advances a genuinely preseeded15 to durable19. The corrected new postcondition expects19, with an added explicit pre-constructor15 observation. The independent review confirmed no original safe15 assertion existed after startup, and the original measured B19/safe19→400 block through cleanup is byte-identical.
- The initial repaired selection reached 47/48 before this new helper precondition was corrected. A subsequent exact retention run passed, followed by the final full 49/49 run. An unused-import warning was removed before strict checks. Two receipt-generation attempts assumed absent prior JSON fields; neither wrote a receipt. The corrected receipt reconstructs the ordered prior inventory and verifies its known pin before writing.
- The status helper initially affected four callers. Scope review restored that helper and changed only the intended successor caller; queued exact status and final strict checks passed after narrowing. No broader status GREEN is inferred.
- The simplification pass reuses Plan 17 and the existing Plan 15 factory where their full parameters match. Local checkpoint assembly remains small because it sits outside that test subtree. No new Rust/helper path, dependency, public capability or production behavior was introduced. All fifteen owned implementation paths fit 628 lines; the largest is 618.

Preliminary source-only review of all eight final Rust owners is clean in165-PRE-REVIEW-INTEGRATION18.md, with zero findings and closing identities. It complements the executed evidence and does not replace whole-phase review. Root AGENTS, Bright Builds sidecar, placeholder-only overrides and architecture/code-shape/verification/testing/Rust/TypeScript standards informed execution. Both active lesson inputs were loaded completely: 7,188 bytes / 2,397 estimated tokens; no project skill or lesson audit trigger applied. No blocking stub or undeclared production endpoint/auth/schema/file-access surface was introduced. No authentication gate occurred.

## Deferred Parent Gates and Commits

The original Plan 14 diagnostic remains historical: 4,461 pass / 249 fail/six old ignored in 46 roots. No remaining workspace count is inferred from this focused repair. Later approved fixture plans and final Plan 14 default native verification, coverage, Bazel, whole-phase source/security/formal review and normal-hook finalization remain required.

All four ordinary selectors remain refused and networking defaults off. Phase 166 retains canonical bootstrap, exact all-handle reopening and genesis BASIC;167 retains contextual completeness;168 retains actual pinned-Knots qualification. Synthetic fixture evidence here qualifies none of those later boundaries. This plan did not start a later phase.

All task/RED/GREEN/metadata commits are deferred to the parent's authorized strict phase transaction. This executor staged, committed and pushed nothing, created no worktree, changed no hooks and edited no shared STATE/ROADMAP/REQUIREMENTS/config/todo or other plan. No commit hashes are invented; HEAD remains d0104a997c0362bf65b81de1c8fec8ddc3012a1b.

## Self-Check: PASSED

All fifteen declared implementation paths and this summary exist. The eleven changed implementation owners have completed scoped behavior/strict compilation and both complete guard matrices, with 251 unchanged checked hashes. Caps, breadcrumbs, managed policy, whitespace and stub scan are verified. Strict support and historical identities remain intact. Shared phase acceptance and final Git saving remain parent-owned; no executor commit is expected.
