---
phase: 165-shared-exact-work-authority
reviewed: "2026-10-17T12:27:35Z"
generated_by: gsd-code-reviewer
phase_lifecycle_id: 165-2026-10-14T16-09-59Z
lifecycle_mode: yolo
depth: deep
scope: Plan28-public-requalification-before-pin-renewal
files_reviewed: 6
files_reviewed_list:
  - packages/open-bitcoin-node/tests/phase165_live_fixture.rs
  - packages/open-bitcoin-node/tests/phase165_live_fixture/requalification.rs
  - docs/parity/source-breadcrumbs.json
  - scripts/check-phase165-shared-exact-work.ts
  - scripts/check-phase165-shared-exact-work/contracts.ts
  - scripts/check-phase165-shared-exact-work.test.ts
findings:
  critical: 0
  warning: 0
  info: 0
  total: 0
status: clean
---

# Phase 165 Plan 28: Independent Preliminary Source Review

**CLEAN before the bounded source/input pin renewal.** The six frozen owners implement the checked public-only requalification proof without weakening the existing fourteen public cases. No actionable issue was found. Approval is limited to the exact proposal below; source-policy matrices, summary, parent prerequisite resolution and final phase acceptance remain separate.

## Context and Authentic Scope

The actual checked Plan 28 hashes to `a8d6d1158e4d688aa43d1321b2babd2abf1584e290a2ae6df3be03be9bc1e697`. The authentic six-owner before capture hashes to `92d4e1b2348a1ea88dad432c0c441a8136cbae74895766a0ab0311fef839da32` and records the new child as absent.

The frozen pre-pin packet is `.local/open-bitcoin-dev/phase165-p28-prepin-review.json`, SHA-256 `52c9400e3b8062a2dbcd0ddcb4aae5b74fbad4681f554c88a0ae50a86d3922b5`. The independent reviewer verified every captured complete source against the working bytes, each captured before identity against the authentic snapshot, and all eight public dependency source/hash records at 2026-10-17T12:27:35.250903Z. No packet field was accepted as proof without comparison.

Repo instructions, Bright Builds sidecar, placeholder-only overrides, relevant architecture/code-shape/testing/verification/Rust/TypeScript guidance and both active lessons informed review. The active lesson set was fully loaded within 7,188 bytes/2,397 conservative estimated tokens; no archives or project skills were loaded. This is an assigned review within the active GSD workflow.

Only this report was written by the reviewer. No source, shared state, Git, hook or pin was changed, and no Bun/project import/test/Cargo/Bazel/build was executed. Reported runtime results below are from independently read executor logs and timing records.

## Public Lifecycle and Authority

The existing root differs from its authenticated before source by exactly the three-line blank/path/module insertion. Removing that exact insertion reconstructs the entire original source byte-for-byte. All fourteen original cases, helpers, maturity 1 wrappers, four canonical-root refusals and the unknown-equal refusal remain unchanged. The explicit path registers the child within the same external integration target and normally compiled Node library.

The child uses only public Node/core observations and transfer APIs. The existing parent `genuine` helper receives the complete Regtest parameters explicitly, validates both model prefix blocks using P2SH, and performs a real flush of their bodies, undo and active positions. The child checks full selected parameter equality and maturity 100. Its populated saved snapshot has physical confirmation counts None, a durable nonroot B marker, empty interrupted heads, exact retained bodies/undo, and complete coins.

Only owned data survives `drop((source, store))`: block values, snapshots, positions and complete coins. No original sink, core, coins view, runtime or network handle is retained. The fresh Fjall open occurs after this drop. The temporary raw `from_parent` uses hydrated metadata only to observe unearned origin, asserts exact UntrustedWorkLineage and is dropped before the genuine replay.

Every observed active-prefix position loads its actual body from the reopened store and requires the expected body, rather than substituting original in-memory data. Fresh `Chainstate::new()` validates root through nonroot with ZERO caller work, actual body time, P2SH and complete selected parameters. Assertions cover length, height/hash/header/parent/work, exact derived active/undo and complete Memory coins/B/heads equality with physical state.

The new Fjall target receives active/undo/logical Some confirmation counts exclusively from this genuine replay snapshot, while its actual coins parent comes from the reopened store. It starts unearned. The Memory owner is consumed by strict `rebind_into`; subsequent origin and full parameter binding checks succeed, while a maturity 1 parameter alteration is explicitly refused. No raw hydrate metadata supplies those target arguments, no imported snapshot acquires authority, and no physical counts writer is called.

The exact same transferred target is wrapped by public ManagedChainstate construction and passed to the existing public seven-argument constructor with Disabled BASIC and full selected parameters. There is no inserted count normalization or source replay shortcut. After construction, public hydrate/complete_snapshot/load_block/load_undo observations compare the entire persisted prefix/coins/B/heads/undo/bodies with the saved physical identity, including counts None.

## Prefix Graph and First Direct Extension

The existing public constructor independently replays the prefix and installs checked headers/body linkage. The new test observes all prefix header identities/heights/exact work and available/not-failed body facts. Every historical nonroot retains sequence None, both before and after extension.

Immediately before the first new event, the test checks the exact prefix header count, child absence from headers, no child body eligibility, and no stored child body. The first future event is the direct public `connect_local_block(&child, flags, selected)`. There is no separate ingress, Headers message, fixture header admission, cache insertion or raw header write before that call.

The accepted child height/hash/header/work and extended logical length/prefix are asserted. The old prefix's body/undo records and UNKNOWN nonroot order remain, and saved physical confirmation counts are stillNone after actual acceptance. Final cleanup explicitly drops the newly created handle, runtime and reopened store. This proves the bounded model fixture path; it does not establish canonical bootstrap/reopen or historical arrival ordering.

## Actual Execution Evidence

The reviewer read the real result lines and actual command-timing JSON records; no rerun is claimed.

| Execution                                 | Actual result                           | Harness duration        | Command exit |
| ----------------------------------------- | --------------------------------------- | ----------------------- | ------------ |
| Existing external target before edits     | 14 passed/0 failed/0 ignored            | 5.93s                   | 0            |
| Exact new case discovery                  | One named test, zero benchmarks         | discovery only          | 0            |
| FIRST new runtime execution               | 1 passed/0 failed/0 ignored;14 filtered | 0.82s                   | 0            |
| Full external target after change         | 15 passed/0 failed/0 ignored;0 filtered | 5.66s                   | 0            |
| Affected formatting check                 | Passed                                  | 1.936s command duration | 0            |
| Node all-target/all-feature strict Clippy | Passed                                  | 2.972s command duration | 0            |
| Node all-target/all-feature build         | Passed                                  | 0.320s command duration | 0            |

The initial discovery attempt failed compilation with E0616 because it tried to read private `runtime.consensus_params`. Its log and source are preserved. Independent source comparison proves the only correction was removing that illegal private-field assertion; no production accessor or helper change was made. This compile-only failure is not a runtime RED result. The first executable test was legitimately GREEN.

| Log                                                             | Complete SHA-256                                                   |
| --------------------------------------------------------------- | ------------------------------------------------------------------ |
| `.local/open-bitcoin-dev/phase165-p28-baseline-external.log`    | `f4594f0e94bf691bc2b57397a5a3f5e8a4ac39c5075030624d53fed5c697853f` |
| `.local/open-bitcoin-dev/phase165-p28-list-initial-compile.log` | `ee5771bf602e128c619be24695f64151c98b8c54388f9db63a2713aa8a311c91` |
| `.local/open-bitcoin-dev/phase165-p28-list.log`                 | `14da283308cf67c2b94a8912490a02912cf64f8a1059854f494760009532cf94` |
| `.local/open-bitcoin-dev/phase165-p28-named.log`                | `98bd47adc6b70b740d03b0452a4df66c62bb532b88015be0f564ebeb2aac5ca1` |
| `.local/open-bitcoin-dev/phase165-p28-external.log`             | `d75df4f728acae381e77387130d96fca687a665f696ddee3810d7c462629a219` |
| `.local/open-bitcoin-dev/phase165-p28-fmt.log`                  | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| `.local/open-bitcoin-dev/phase165-p28-clippy.log`               | `99e15f9e69dfad1d6f1e40c0313af74c3eb4abbf1b709cff32ee4363075a4d63` |
| `.local/open-bitcoin-dev/phase165-p28-build.log`                | `4cf86278c1e6d659943a46a3d7e3da429ac15be90f0f17742e24ef85228e7d3e` |

## Source Controls, Breadcrumbs and Caps

All 22 new PUBLIC_REQUALIFICATION_MUTATIONS select actual, present, unique anchors in the child. Each replacement changes bytes. The cases cover original handle retention, imported rather than fresh Memory origin, partial-prefix validation, maturity 1/parameter substitution, raw-origin denial, target metadata copied from raw hydrate, omitted consuming transfer, omitted derived Some/physical None/complete coins checks, incomplete retained inputs, invented historical sequence, inserted ingress before direct connect, missing child-absence checks, post-extension counts, changed prefix, full selected equality and post-constructor physical comparison.

The existing owner/peer loop is reused with a same-line imported group and conditional. Every new case first requires one unique loaded anchor and an unchanged passing checkPhase165 baseline, then requires the actual child owner rule. The existing mutation helper proves bytes changed. This is source-policy evidence, not execution of mutated Rust semantics.

The complete root path/module declaration joins TEST_REGISTRATIONS, so existing deletion/comment/cfg/path-redirection controls exercise it. The proposed child TEST_OWNERS row invokes the existing per-owner enabled test, ignore, disabling and file configuration controls. Previous controls and table rows remain intact; no new TS module or broad discovery is introduced.

The breadcrumb manifest differs only by appending the exact new child to the existing public fixture source group, with the same validation.cpp and node/blockstorage.cpp anchors that occur immediately at the child's top. No other group was changed. Root 568, child 280, current entry 266, contracts 616 and main 627 physical lines fit 628 without an exception or line-count trick. The proposed two row additions would leave contracts 618.

## Approved Bounded Pin Proposal

Proposal `.local/open-bitcoin-dev/phase165-p28-pin-proposal.json` hashes to `7a6f17ffbfabfd5cbf4bbd0dab2cfba614d18c087a6f1b8dae476dd89e6ac407`. It matches the packet proposal exactly. Independent code-constant reconstruction, without executing project modules, confirms only these changes:

- Renew existing root SOURCE row 8 from `09fe30d34ff7c1dce4719d3d8bbe16a95b6112d4b2f88d53f9d611dd06a744d1` to `a2f3812dd06c6bb485bca9c62dcd0aa9fddd9c1adbe7d56f2018a675b556f0fb`.
- Append child SOURCE row 134 with ordinary ownership rule and identity `afe3d4eef34d032b2f6928ce15e54ad7cac9e4b5ed25db81ab43e964f11e9630`.
- Append child TEST row 70 with exactly 1 enabled test, preserving the existing root 14 and all previous rows.
- Grow SOURCE 134→135, TEST 70→71, INPUT 263→264, with the child at input index 250 and every old relative input position/order retained after removing that child.
- Renew only the current entry's ordered inventory SHA to `967b67cdf5a41019a931d9016b32dfc3c42dff8b07713cdab2d8abee48b1868b`.

Both proposed source identities were independently computed from actual literal-bearing source after leading commentary. The prior entry pin remains `53b26c6f2cd457c1f2df85b22095f192de09b76ca2256c9a3eff9f89d3fd208a` in this pre-pin packet. Old 164 requires no renewal: this external root/child is outside its SOURCE_FILES. No historical manifest, snapshot, report, tooling inverse, legacy bundle or production source change is approved.

**Only these exact rows and one inventory literal may now renew after parent review.** Demonstrate stale pre-read pin refusal and independently reverse the final pin delta before full source matrices. No targeted new guard run is claimed yet because the actual source identities/new child are intentionally not registered until this review gate.

## Frozen Complete-File Identities

| Owner                                                                       | Lines | Pre-pin raw SHA-256                                                |
| --------------------------------------------------------------------------- | ----- | ------------------------------------------------------------------ |
| `packages/open-bitcoin-node/tests/phase165_live_fixture.rs`                 | 568   | `1c38f95572bae47ef45753fe3990d1517142d05821498c91bcae1599566f9ca9` |
| `packages/open-bitcoin-node/tests/phase165_live_fixture/requalification.rs` | 280   | `5594a7f2ada84b0bda3fab8fb102834d5002b40e64f17f35fe258e6ebef92f22` |
| `docs/parity/source-breadcrumbs.json`                                       | 3782  | `4fe89200d14b415d2ad4d14e842d7281e1af3d19dd376b0460bb31e1fad35061` |
| `scripts/check-phase165-shared-exact-work.ts`                               | 266   | `68fb5efa600e55220e819fd8c578c13aac8e877996e6bcdd9c5eb29e874ba503` |
| `scripts/check-phase165-shared-exact-work/contracts.ts`                     | 616   | `71230b7180c561860de035e83084c15c2f0cd920ebaee704260198e7eadad2eb` |
| `scripts/check-phase165-shared-exact-work.test.ts`                          | 627   | `d1f595f02d18a630a769135a593ced452c63d0f43608d674433c66c3ce227037` |

The earlier preliminary tooling review is reusable only for unchanged complete-file identities. Its 18-file inventory includes the three current 165 files; after this plan's final renewal, the other 15 tooling files remain a separate unchanged scope, subject to hash confirmation. Changed contracts/main are reviewed here, and the entry is approved only for its one future inventory-literal renewal.

## Remaining Acceptance

Fresh complete 164/165 matrices, full pre/post fixed-input/module closure, breadcrumbs, managed checks, whitespace and Plan 28 SUMMARY remain required. Parent/Plan 15 checker acceptance of the actual summary and complete receipts must still resolve Plan 24's explicit prerequisite before consumer edits. This report alone does not declare Plan 24 ready.

No whole-phase/native/coverage/security/formal/Git completion or canonical bootstrap/reopen, deterministic restart ties, ordinary full-network acceptance or actual pinned-Knots differential qualification is claimed. CWOR-04 remains Phase 166; current phase requirement scope is CWOR-01/02/03/05.

_Reviewer: gsd-code-reviewer; independent source review before bounded pin renewal._
