---
phase: 165-shared-exact-work-authority
reviewed: 2026-10-17T02:28:25Z
generated_at: 2026-10-17T02:28:25Z
generated_by: gsd-code-reviewer
lifecycle_mode: yolo
phase_lifecycle_id: 165-2026-10-14T16-09-59Z
depth: standard
review_base: d0104a997c0362bf65b81de1c8fec8ddc3012a1b
preliminary: true
source_only: true
files_reviewed: 45
files_reviewed_list:
  - packages/open-bitcoin-node/src/chainstate.rs
  - packages/open-bitcoin-node/src/chainstate/filter_index.rs
  - packages/open-bitcoin-node/src/chainstate/filter_reorg/preflight.rs
  - packages/open-bitcoin-node/src/chainstate/filter_reorg/tests.rs
  - packages/open-bitcoin-node/src/chainstate/filter_reorg/tests/faults.rs
  - packages/open-bitcoin-node/src/chainstate/fjall_store.rs
  - packages/open-bitcoin-node/src/chainstate/fjall_store/tests/fixture.rs
  - packages/open-bitcoin-node/src/chainstate/fjall_store/tests/fixture/managed.rs
  - packages/open-bitcoin-node/src/chainstate/live_fixture.rs
  - packages/open-bitcoin-node/src/chainstate/validation_history.rs
  - packages/open-bitcoin-node/src/chainstate/validation_history/tests/phase165_work_authority.rs
  - packages/open-bitcoin-node/src/chainstate/work_authority.rs
  - packages/open-bitcoin-node/src/network/candidate_eligibility.rs
  - packages/open-bitcoin-node/src/network/header_sync.rs
  - packages/open-bitcoin-node/src/network/mempool_lifecycle.rs
  - packages/open-bitcoin-node/src/network/runtime_authority.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/catch_up.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/catch_up/tests.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/reorg.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/live_fixture.rs
  - packages/open-bitcoin-node/src/network/tests/phase165_eligibility.rs
  - packages/open-bitcoin-node/src/network/tests/phase165_headers.rs
  - packages/open-bitcoin-node/src/network/tests/phase165_rebind.rs
  - packages/open-bitcoin-node/src/network/tests/phase165_work_authority.rs
  - packages/open-bitcoin-node/src/network/tests/phase165_work_authority/fixtures.rs
  - packages/open-bitcoin-node/src/network/tests/phase165_work_authority/freshness.rs
  - packages/open-bitcoin-node/src/network/tests/phase165_work_authority/history.rs
  - packages/open-bitcoin-node/src/network/work_authority.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/ownership.rs
  - packages/open-bitcoin-node/src/sync/block_reconcile.rs
  - packages/open-bitcoin-node/src/sync/block_response.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/branches.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/branches/manual_lifecycle.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/failures.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/fixtures.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/fixtures/live.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/fixtures/return_branch.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/protection.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/retention.rs
  - packages/open-bitcoin-node/src/sync/tests/phase158_preflight.rs
  - packages/open-bitcoin-node/src/sync/tests/phase165_candidates.rs
  - packages/open-bitcoin-node/src/sync/tests/phase165_candidates/fixtures.rs
  - packages/open-bitcoin-node/src/sync/tests/support_runtime.rs
  - packages/open-bitcoin-node/src/sync/types/error.rs
findings:
  critical: 0
  warning: 0
  info: 0
  total: 0
status: clean
source_delta_detected: false
---

# Phase 165: Preliminary Node Source Review

**Reviewed:** 2026-10-17T02:28:25Z
**Depth:** standard, source only
**Files reviewed:** 45
**Status:** clean within the listed scope

## Summary

No actionable correctness or security defect was identified in the listed Node live-work authority, candidate reconciliation, BASIC integration and relevant regression/fixture sources. This is a preliminary independent review of the current checkout against `d0104a99`, including untracked files. It is not phase acceptance, runtime test proof, a coverage claim or pinned-daemon qualification. All files in `files_reviewed_list` were read in full. Supporting pure-core/network interfaces were consulted only to understand Node contracts; their separate review is not duplicated here.

## Review Plan and Completion

- [x] Load repository instructions, applicable standards, active lessons and Phase 165 context/07–09 summaries.
- [x] Identify changed and untracked sources; capture per-file SHA-256 before reviewing.
- [x] Trace actual Node ownership, generation, parameters and preference checks through preparation, intake, preview and commit.
- [x] Inspect candidate fallback, accepted-prefix progress, selected-header request projection and BASIC target/fixture guards.
- [x] Review relevant tests for behavioral assertions and authority shortcuts.
- [x] Rehash the exact reviewed scope before report creation.

## Source Evidence

| Boundary | Source inspection |
| --- | --- |
| Actual live connect owner | `chainstate/work_authority.rs:23` binds genuine selected source, checks actual parent and selected target/hash, and derives parent-plus-target work. `chainstate.rs:329` uses that derivation before staging/history/BASIC preparation. Caller work is ignored. Local network connect performs this preflight before candidate retention. |
| Lower reorg refusal | `chainstate/work_authority.rs:83` unconditionally refuses unqualified lower preparation. A genuine core stage alone does not grant managed branch preference. |
| Sealed reorg decision | `network/work_authority.rs:196` checks actual source, retained disconnect bytes/undo, accepted ancestor and replacement linkage, then derives exact work. Active tie order requires matching immutable accepted linkage; unknown active or candidate order cannot win an equal-work tie. |
| Freshness before effects | `network/work_authority.rs:149` checks source/current preference before consuming and publishing its exact checked header store once. Checks at lower preparation, `chainstate.rs:381` preview and `chainstate/validation_history.rs:244` commit preserve owner/generation/eligibility constraints. Current Weak epoch checks remain separate from retained immutable history. |
| Connect absorption | `chainstate/validation_history.rs:195` checks genuine origin, optional current body grant and source-bound stage before exclusive absorption; persistence failures remain observable after actual acceptance. |
| Cached bodies and selection | `network/candidate_eligibility.rs` uses the typed detailed cached validator and retained suffix checks. `sync/block_reconcile.rs:171` readmits raw cached bytes, enumerates retained candidates, and derives suffixes from actual ancestry rather than following only the best-header branch. Checked cached linkage does not mint known arrival order. |
| Typed fallback and progress | `sync/block_reconcile.rs:242` distinguishes actual consensus causes. Reorg fallback inspects the attributed replacement cause; local/storage/BASIC/stale failures propagate. Invalid candidates trigger a fresh eligibility query. `sync/block_reconcile.rs:384` returns actual accepted-prefix progress and ends the turn before another ancestry decision. |
| Requests and durable notification | Request scheduling delegates to the selected full-parameter proof projection and retains ordinary inflight/body budgets. `sync/block_response.rs` persists requested side/pending bodies without false connection credit; notification follows actual active progress and durable persistence. `sync/types/error.rs:119` preserves nested replacement error conversion. |
| BASIC pre-store identity | `network/runtime_authority/filter_index/catch_up.rs:154` compares an invalidated accepted target with actual active identity before proof acquisition/store work. Matching raw read-only recovery remains separate from live work. |
| Fixture transfer and counts | `chainstate/live_fixture.rs:16` enriches only missing counts from a genuine physically matching flushed owner. `chainstate/live_fixture.rs:54` checks concrete parent, physical metadata/undo/counts and delegates consuming full-state rebind. Runtime delegates require Fixture scope. |
| Same-live BASIC attachment | `chainstate/live_fixture.rs:90` requires genuine live origin, no prior owner/lineage, no pending history and complete physical equality. Proof acquisition precedes the publication guard; the guarded same-store/current verifier precedes field assignment. The route and lineage constructor are test-only. |
| Fixture evidence quality | `sync/tests/support_runtime.rs:52` seeds only a new path from genuinely validated bodies. Its replay helper at line 155 validates the complete retained active prefix before strict adoption. BASIC reorg cases exercise real preferred transitions, actual missing-input failures, ordinary flush and real close/reopen; explicit disconnect cases retain paused BASIC and physical checkpoints. |

The inspected regression sources include foreign/stale owner and grant refusal, epoch revocation without core mutation, clone/reseed history controls, caller MAX resistance, earlier/later equal-work cases, shorter-harder preference, typed invalid-body fallback, fatal missing active input with a viable sibling, accepted-prefix notification, raw unproved request refusal, full physical rebind comparisons and same-live BASIC attachment negatives. These are source assertions inspected, not tests executed by this reviewer.

## Guidance and Limits

`AGENTS.md`, `AGENTS.bright-builds.md`, placeholder-only `standards-overrides.md`, `standards/index.md`, and architecture/code-shape/verification/testing/local-guidance/Rust standards informed this review. No project skills were found in `.claude/skills` or `.agents/skills`. Both canonical active lesson inputs were read completely: global 5,230 bytes, repository 1,958 bytes, combined 7,188 bytes and 2,397 conservative estimated tokens. Existing audit baselines were present; no archive content was loaded.

No tests, build, lint, Bun guard, Cargo or Bazel commands were run; Plan 14 owns native verification. No implementation, checker, tracker, hook, configuration or Git state was edited.

This review does not establish complete contextual consensus, ordinary durable work recovery or actual Knots behavior. The inspected sources/tests preserve all four canonical-selector refusal assertions and do not grant production support through fixture APIs. Default-off networking remains a required final acceptance invariant. Phase 166 retains canonical bootstrap, exact all-handle checkpoint/reopen and genesis BASIC; Phase 167 retains contextual completeness; Phase 168 retains actual pinned-daemon qualification. Final independent review must be repeated after Plan 14 fixes and final source stabilization.

## Exact Source Closure

The following SHA-256 values were captured before reviewing each listed file and matched on the closing rehash at 2026-10-17T02:27:38Z. No source delta was observed in this exact scope. Files outside this list are not represented as reviewed.

```text
1d25580fd120dd80c5129d065ae5e31da460392be9112f6c2ef767f2defb2ba8  packages/open-bitcoin-node/src/chainstate.rs
9cf19c472ce8b25d0c3d0c33dc7e2465d689fe813333baeb50e558dced088c48  packages/open-bitcoin-node/src/chainstate/filter_index.rs
b7adbc9f1ec32d45e36429e2702fde89409050268b22f084bf2f5b5ede33d377  packages/open-bitcoin-node/src/chainstate/filter_reorg/preflight.rs
fdf3249d9f4bfbe5ac9a110675b0676941d2de64c3773cb8e4dd17c532c037a4  packages/open-bitcoin-node/src/chainstate/filter_reorg/tests.rs
6de4474a52d4856a58d94b5001be91c679c3de020309ad7941eb2c1893788c10  packages/open-bitcoin-node/src/chainstate/filter_reorg/tests/faults.rs
57d814acd77d5f16741669ee2844f6e437410cd2f6dd999b839326d18f66d429  packages/open-bitcoin-node/src/chainstate/fjall_store.rs
122c2572f6166783d9506e0042993a76b1d228508879308dc2cdcc4a71bad0c1  packages/open-bitcoin-node/src/chainstate/fjall_store/tests/fixture.rs
b91a18bb9faa231da62fa935a4f307b6a9ecc617f5b71b42172a1900b2abb37f  packages/open-bitcoin-node/src/chainstate/fjall_store/tests/fixture/managed.rs
13ac5561c3e40eadc7cbb195c70dfd3efb6446af5509e38035e09e4062289a11  packages/open-bitcoin-node/src/chainstate/live_fixture.rs
de507925613e44fdea9cb5b3e8766c8aebc5070302a70db7fd59fd6c1dcbee62  packages/open-bitcoin-node/src/chainstate/validation_history.rs
910688f61c3c4641eae86d0ca65d1e33591c5a3f96a748848819370536ab6a0e  packages/open-bitcoin-node/src/chainstate/validation_history/tests/phase165_work_authority.rs
18fe2dc025deb1b556aa6570166e93d4951dc45e67b4ead69b99a12e255f9370  packages/open-bitcoin-node/src/chainstate/work_authority.rs
14e3334a388bdde9037fc1521a86e8a42d3ede428fc0fff8b4fe5be078b1ca1a  packages/open-bitcoin-node/src/network/candidate_eligibility.rs
dcd625d4dd241fb31f97d67d78ac8cd7886309f1927b91abb6fd4a1074564b8e  packages/open-bitcoin-node/src/network/header_sync.rs
fa186eeafab8925c40006f7a20671c265af261a364698712b751562b6baa8bbb  packages/open-bitcoin-node/src/network/mempool_lifecycle.rs
65835d0307dd58ee1572efab8a8eb83794cc210c1748ad87f5218bb3cf082212  packages/open-bitcoin-node/src/network/runtime_authority.rs
dd05bf16a0e4be7cbff5f4d05f5ba25984ed2fa9e366a1d0e2efa95a190508a6  packages/open-bitcoin-node/src/network/runtime_authority/filter_index.rs
d1afde6b9d01c22a2adf253945f3b5b0f6ed55d9974842aaf73cd9ae9203a621  packages/open-bitcoin-node/src/network/runtime_authority/filter_index/catch_up.rs
aa8286cbca8b9336217764562d5be181bd13556d4b7ea1d314d4b03d8d623fd9  packages/open-bitcoin-node/src/network/runtime_authority/filter_index/catch_up/tests.rs
0919a19fa34d573d21b3d1b4e8b17f10f49353f1f9b93b93f1ed8113fa88b85c  packages/open-bitcoin-node/src/network/runtime_authority/filter_index/reorg.rs
5fb001358e2938b00cf50f89335dcc40f099135f3b6649754c902f779306918a  packages/open-bitcoin-node/src/network/runtime_authority/live_fixture.rs
141042a5aa3e705bc329e04db0763350395e769383642df5c0bf6c84dcf4cda4  packages/open-bitcoin-node/src/network/tests/phase165_eligibility.rs
913dc6674074c8ee9f86f10696d294ef0ebf6bd72ca78e8ecd7467faf837b38c  packages/open-bitcoin-node/src/network/tests/phase165_headers.rs
387f0e02c0991efa4dc6351a9a9c471a7109ca073943c6cc5b8acf18dc4a9629  packages/open-bitcoin-node/src/network/tests/phase165_rebind.rs
26cdf05da44af6d10482f6163651ff790fa128f1deedd3330a34cf06852e3979  packages/open-bitcoin-node/src/network/tests/phase165_work_authority.rs
ab0eec5a42af1f5ffe8bb8cfaff9cd011911890a5839a8d4d3b3616731285c61  packages/open-bitcoin-node/src/network/tests/phase165_work_authority/fixtures.rs
f20173e226b2fccf614ed24d7a1a876f0150911a6470134f6446fb239cd7ff25  packages/open-bitcoin-node/src/network/tests/phase165_work_authority/freshness.rs
63a7bd3e74120a3b450cddf5bf2839ab19e743818d8b00fefe98fa7fcc170839  packages/open-bitcoin-node/src/network/tests/phase165_work_authority/history.rs
a0952b43f0f4e6eb598421abfb6e6c2e8d01d30ebbb224ba80bd8c6fe8f71447  packages/open-bitcoin-node/src/network/work_authority.rs
ae563e872349666dee2a27075806327afe769cbb70d14778dd41124b020eb78d  packages/open-bitcoin-node/src/storage/fjall_store/filters/ownership.rs
96341ba6541be2f3ed99d7a96745dbd88ce6ed254e4f5c01059fc4c21e2d70f0  packages/open-bitcoin-node/src/sync/block_reconcile.rs
718f8eb2fa0e552cb26fd43e62cd92ade6145973fc96b957885d54121060d11c  packages/open-bitcoin-node/src/sync/block_response.rs
8e384b3b398d0ad7c10872858ac2a70906e352c253c6194230cff099b0ce55e7  packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/branches.rs
34de4e6cf2de803550da7bd16cd8cae52fd288e176b53987e77ac6aa6725e6c1  packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/branches/manual_lifecycle.rs
ed0f8046f5664b536fc450841f4e0ccc65818884a5e3a975347183009dd21743  packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/failures.rs
fbea03ab6da3782fb49e882c4074ad0cc412c0ebc8ec68f5e8e2cf8dad8b5bf5  packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/fixtures.rs
4e5376a4ff1b654f9532810d59d6d903cbc1e6ec7093e06b15f3108952327dcc  packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/fixtures/live.rs
6d6fa1911ea2263da025f1374d5fe9d1ef8f374eb97dc9313688a944e1099921  packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/fixtures/return_branch.rs
5df429f474195bc120b2fd5ea5db4f4e05bf559956e0416c5c9d4a189bab98e8  packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/protection.rs
2f13173c2aee79bb080adadfb494f59cf691838d10278a85c81c8d0a689a5951  packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/retention.rs
6aae5f2c1e04139a0a464b396ae08430472e781cf7710bd670f070e49151b92c  packages/open-bitcoin-node/src/sync/tests/phase158_preflight.rs
acf61dc2106f647f99dbf950acfa7c446393bd90412646637910f4f5827dcb8b  packages/open-bitcoin-node/src/sync/tests/phase165_candidates.rs
3a18443d0a228843d7f240c0c3db4a2791c01ae64664fd0f9a11032c6d37edd2  packages/open-bitcoin-node/src/sync/tests/phase165_candidates/fixtures.rs
5a25857c552bb7dcfdd040ad15e19c4616f52ef8b35df69b4f11a42dcd5b1f05  packages/open-bitcoin-node/src/sync/tests/support_runtime.rs
18967e0b9b4c774c4d67e9c74712b37135203dd49c448096cabefb6ed340390c  packages/open-bitcoin-node/src/sync/types/error.rs
```

_Reviewer: gsd-code-reviewer. Preliminary source review only._
