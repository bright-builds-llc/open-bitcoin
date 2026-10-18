---
phase: 165-shared-exact-work-authority
reviewed: 2026-10-17T12:07:29Z
depth: deep
review_scope: preliminary-production-rust-only
base_commit: d0104a997c0362bf65b81de1c8fec8ddc3012a1b
knots_commit: a9aee730466ac67d35a3c03ee24676be5e045878
files_reviewed: 53
files_reviewed_list:
  - packages/open-bitcoin-chainstate/src/coins.rs
  - packages/open-bitcoin-chainstate/src/coins/cache.rs
  - packages/open-bitcoin-chainstate/src/coins/memory.rs
  - packages/open-bitcoin-chainstate/src/engine.rs
  - packages/open-bitcoin-chainstate/src/engine/overlay_apply.rs
  - packages/open-bitcoin-chainstate/src/engine/stage.rs
  - packages/open-bitcoin-chainstate/src/engine/work_authority.rs
  - packages/open-bitcoin-chainstate/src/error.rs
  - packages/open-bitcoin-chainstate/src/lib.rs
  - packages/open-bitcoin-chainstate/src/types.rs
  - packages/open-bitcoin-consensus/src/block.rs
  - packages/open-bitcoin-consensus/src/chain_proof.rs
  - packages/open-bitcoin-consensus/src/crypto.rs
  - packages/open-bitcoin-network/src/header_store.rs
  - packages/open-bitcoin-network/src/header_store/eligibility.rs
  - packages/open-bitcoin-network/src/header_store/proof.rs
  - packages/open-bitcoin-network/src/lib.rs
  - packages/open-bitcoin-network/src/peer.rs
  - packages/open-bitcoin-network/src/peer/eligibility.rs
  - packages/open-bitcoin-network/src/peer/message_dispatch.rs
  - packages/open-bitcoin-node/src/chainstate.rs
  - packages/open-bitcoin-node/src/chainstate/filter_index.rs
  - packages/open-bitcoin-node/src/chainstate/fjall_store.rs
  - packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs
  - packages/open-bitcoin-node/src/chainstate/replay.rs
  - packages/open-bitcoin-node/src/chainstate/validation_history.rs
  - packages/open-bitcoin-node/src/chainstate/work_authority.rs
  - packages/open-bitcoin-node/src/network.rs
  - packages/open-bitcoin-node/src/network/action_translation.rs
  - packages/open-bitcoin-node/src/network/candidate_eligibility.rs
  - packages/open-bitcoin-node/src/network/canonical_admission.rs
  - packages/open-bitcoin-node/src/network/header_sync.rs
  - packages/open-bitcoin-node/src/network/inventory.rs
  - packages/open-bitcoin-node/src/network/mempool_lifecycle.rs
  - packages/open-bitcoin-node/src/network/peer_network_clone.rs
  - packages/open-bitcoin-node/src/network/relay_serving.rs
  - packages/open-bitcoin-node/src/network/runtime_authority.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/catch_up.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/reorg.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/prune_flush.rs
  - packages/open-bitcoin-node/src/network/validation_history.rs
  - packages/open-bitcoin-node/src/network/work_authority.rs
  - packages/open-bitcoin-node/src/storage/chain_compatibility.rs
  - packages/open-bitcoin-node/src/storage/coins_view.rs
  - packages/open-bitcoin-node/src/storage/snapshot_codec.rs
  - packages/open-bitcoin-node/src/storage/snapshot_codec/work.rs
  - packages/open-bitcoin-node/src/sync.rs
  - packages/open-bitcoin-node/src/sync/block_reconcile.rs
  - packages/open-bitcoin-node/src/sync/block_response.rs
  - packages/open-bitcoin-node/src/sync/live_fixture.rs
  - packages/open-bitcoin-node/src/sync/open_runtime.rs
  - packages/open-bitcoin-node/src/sync/runtime_state.rs
  - packages/open-bitcoin-node/src/sync/types/error.rs
findings:
  critical: 0
  warning: 0
  info: 0
  total: 0
status: clean
phase_qualified: false
native_qualification: false
source_drift_observed: false
---

# Phase 165: Preliminary Production Rust Review

**Result: CLEAN within the production scope below.** No actionable bug, security vulnerability, or integration defect was identified in this fresh independent static review. This is not the canonical phase review, native verification, phase completion, or ordinary/canonical-node qualification.

## Scope and method

The exact base resolves through Git to `d0104a997c0362bf65b81de1c8fec8ddc3012a1b`. Scope discovery combined `git diff --name-only BASE` with `git ls-files --others --exclude-standard`, then selected first-party `packages/open-bitcoin-*/src/**/*.rs` files. Conventional test paths were excluded. Changed files' production sections, new production modules, their call chains, and relevant unchanged API boundaries were inspected. Test sections in mixed files were used only to determine ownership, not audited for reliability or counted as production findings.

This produces 53 changed/new production files and 17 additional source-path identities whose changes are wholly test/benchmark-only. The inventory records full-file SHA-256 values for all 70 paths, including mixed files. Recomputed end-of-review identities matched the initial capture: no additions, removals, or hash drift occurred in this source-path inventory. Git's normal ignore exclusions were honored. No project skill directories were present.

A checkable review sequence was completed:

- [x] Load local guidance, active lessons, locked D-01 through D-12, and relevant plan/summary objectives and authority boundaries.
- [x] Discover tracked and untracked Rust changes and capture full source identities.
- [x] Inspect arithmetic/codec/header/eligibility/core/Node/storage/sync changes and trace activation, invalidation, and effect ordering.
- [x] Compare relevant local primary Knots source and verify its exact pin.
- [x] Recheck source identities, distinguish test-only ownership, and write this scoped report.

The material guidance was `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md` (placeholder only), `standards/index.md`, and the local architecture, code-shape, verification, testing, and Rust standards. The global and repository active lessons were canonically resolved, deduplicated, measured and fully read: 5,230 + 1,958 = 7,188 bytes; summed conservative token estimate 2,397. No archive was loaded. Shared lesson/audit/task metadata remained parent-owned and unchanged by this reviewer.

## Inspection evidence

| Area                              | Inspected behavior and relevant owners                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| --------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Exact arithmetic                  | `consensus/src/chain_proof.rs` uses the existing four-limb unsigned arithmetic, full-width order, modulo addition and target-derived proof. Byte and hex boundaries preserve every limb and grant numerical representation only. Normalized compact validity and selected powLimit/hash admission share the decoder in `crypto.rs`; tiny normalized zero, sign and overflow handling agree with the pinned formula.                                                                                                                                                                               |
| Codec and migration               | `storage/snapshot_codec/work.rs` separates exact numbers from legacy u128 scores, retains raw JSON for exact legacy parsing, denies incompatible shapes and bounds records. The live DTO decoder refuses legacy classification rather than normalizing it into accepted work. Exact tagged decoding itself grants no lineage.                                                                                                                                                                                                                                                                     |
| Header provenance                 | `header_store.rs` distinguishes admitted headers from imported positions, derives cumulative proof from accepted parents, binds selected parameters, refuses unbound ancestry promotion and uses strictly greater work for best-header replacement. Body request projections exclude imported numbers. Managed header entrypoints perform selected proof/continuity/scope checks before peer handling; contextual prefix admission remains explicit.                                                                                                                                              |
| Body/ancestor order               | `header_store/eligibility.rs` checks actual bytes and selected contextual facts, queues pending children in parent-local arrival order, releases them breadth-first and reserves the full sequence range before registry mutation. Historical unknown order propagates through ancestry and cannot become a known activation tie. Current availability and failed ancestry are separate from immutable historical linkage.                                                                                                                                                                        |
| Core authority                    | `engine/work_authority.rs` and `engine/stage.rs` bind nonclone stages to private owner, generation, complete source tip and parameters. Public raw imports remain untrusted; mechanical fixture boundaries remain unearned. Mutable-coins access taints authority, persistence rotates stage generation, independent clones get independent owners, and only the exact preview continuation can absorb its preview. Exclusive validated capabilities close the check-to-absorb mutation window.                                                                                                   |
| Rebind completeness               | `CoinsView::complete_snapshot` refuses unsupported adapters by default; memory/cache/Fjall implementations provide complete effective coins, B and heads. Cache merging includes masks/tombstones, and Fjall reads coin rows and markers from one snapshot. Consuming rebind compares complete coins, positions, undo, counts and endpoint before installing a fresh target identity; populated logical counts are not normalized.                                                                                                                                                                |
| Node authorization                | `network/work_authority.rs`, `chainstate/work_authority.rs` and managed lifecycle paths derive actual work, require retained checked bodies and accepted ancestry, bind activation preference to actual linkage/epoch, and recheck before core/BASIC preview and commit effects. Managed direct reorg preparation refuses without authorization. Candidate failure marking is limited to actual consensus/coins rejection causes.                                                                                                                                                                 |
| Reconciliation                    | `sync/block_reconcile.rs` considers retained eligible side branches as well as best headers, uses unknown-order cached admission, refreshes candidates after branch failure, distinguishes fatal local/storage/authority errors from invalid candidate bodies, and reports accepted extension prefixes when a later body is rejected. Requested side/pending body retention and active-tip durable announcements were traced through `block_response.rs`.                                                                                                                                         |
| Public consuming fixture assembly | `sync/live_fixture.rs` first checks live origin/full parameters/pending publication/all four actual canonical roots, then concrete store membership, independent full-prefix replay, complete physical/live coins, positions, undo, counts, B/heads, bodies and ScriptsValid records. It carries the same genuine core through normal initialization, builds a checked cached-body graph and repeats captured-content checks before final runtime assembly. Physical saved counts may remain None only with the independent complete proof; this is distinct from strict logical rebind equality. |
| Ordinary integration              | `FjallNodeStore::open_ordinary` still refuses every filesystem observation before backend construction. Typed work policy remains separate from unavailable observed work. Unchanged operator offline-sync/wallet/soak and daemon inbound paths were cross-referenced to confirm continued ordinary refusal routing; the fixture constructor does not open those routes.                                                                                                                                                                                                                          |

The local Knots checkout resolves to `a9aee730466ac67d35a3c03ee24676be5e045878`. Primary comparisons included `arith_uint256.cpp::SetCompact`, `chain.cpp::GetBlockProof`, `node/blockstorage.cpp` cumulative work/strict best-header and candidate comparator, `validation.cpp::ReceivedBlockTransactions` FIFO eligibility release, and `pow.cpp` target admission. The unknown/restart tie limitation is retained; no hash/pointer fallback is promoted into proven activation parity.

## Verification and limits

No Cargo, Bazel, Bun, project import, test, build, formatter-write, or source mutation was run by this reviewer. Parent-owned heavy matrices remained isolated. Read-only Git inspection and `git diff --check` passed. No commit, stage, push, shared tracker/lesson edit, external message, PR, or comment was performed.

The 53-file result is a static source review of changed production behavior, with contextual call-chain inspection. It does not establish test execution, coverage, tool-checker integrity, source-mutation sensitivity, final fixture qualification, daemon differential parity or full-phase acceptance. Test-only changes, documentation and tooling require their separate final gates. The pending public maturity-100 post-close fixture proof is outside this production report.

All four public live-fixture canonical-root refusals are intentional. All four ordinary selectors remain refused and networking defaults remain off. CWOR-01/02/03/05 are the Phase 165 requirements; canonical bootstrap/exact all-handle reopen/genesis BASIC (CWOR-04) belong to Phase 166, contextual completeness to Phase 167, and actual pinned-daemon qualification to Phase 168. These deferred surfaces were not treated as defects.

Constructor refusal evidence must retain its timing limits: initial pure/source refusal checks precede initialization; later startup/I/O failures can follow effects and consume the source. The static review claims no transaction rollback, revision-history/ABA detection or arbitrary concurrent-writer guarantee. The final captured-content check is a point-in-time check.

## Source identity inventory and reuse rule

Every SHA-256 below covers the **entire current file**, including any inline test sections. `production` means its changed production sections are within this audit; other rows record the explicit exclusion. Earlier stage results may be reused only while the exact corresponding full-file identity remains unchanged. Any future full-file drift requires renewed scoped review (or an explicit production/test ownership analysis); this report does not preapprove future edits. The contextual unchanged CLI/RPC/Knots sources were inspected for routing/parity and are not added to the changed-production qualification set.

| Path                                                                                | Full-file SHA-256                                                  | Scope                                           |
| ----------------------------------------------------------------------------------- | ------------------------------------------------------------------ | ----------------------------------------------- |
| `packages/open-bitcoin-bench/src/cases/chainstate.rs`                               | `eca138eba3518ceb60fce3cb1e069bf11e6d7582eed92ae9e606eb3d17eacfac` | benchmark fixture/case                          |
| `packages/open-bitcoin-bench/src/cases/network.rs`                                  | `8c690621a3c5e79610f854b286bef4ec469e4a1f9c9eccbf83c7577a9f694617` | benchmark fixture/case                          |
| `packages/open-bitcoin-bench/src/cases/storage_recovery.rs`                         | `43eb1347f17f63431f98a542355a8352f2d51e5b34c7293f8057fe9c5eed116b` | benchmark fixture/case                          |
| `packages/open-bitcoin-bench/src/cases/sync_runtime.rs`                             | `62668b6f6627ef7b7d7ebb376c16d453748afbeeea78cc02bb8e6d0fdf3902cd` | benchmark fixture/case                          |
| `packages/open-bitcoin-bench/src/fixtures.rs`                                       | `8b540d61afe51db5f46ff926f1f715a8ea926c8af53a58dbe1295a4b0e9e7f96` | benchmark fixture/case                          |
| `packages/open-bitcoin-bench/src/runtime_fixtures.rs`                               | `d21b6ca89b05af06a72764f227921798cf30385a5a29e58f0d3607e746478362` | benchmark fixture/case                          |
| `packages/open-bitcoin-chainstate/src/coins.rs`                                     | `cd59da7b3d47af02d06266ef1fcfaaa4e7d9984438d2a28ef36f201284aa54b2` | production                                      |
| `packages/open-bitcoin-chainstate/src/coins/cache.rs`                               | `c4607eb78e30621e1edc093563cc7fb1fe650d72eadf3430a3aae78508222664` | production                                      |
| `packages/open-bitcoin-chainstate/src/coins/memory.rs`                              | `b6919e5204326331f3f61970046b766ed454dec6403b7dadb4f219859edd4003` | production                                      |
| `packages/open-bitcoin-chainstate/src/engine.rs`                                    | `c1d99d5b719d3c04176442f5fe3faadf18dbd8d6bdc71865a92d17a367306156` | production                                      |
| `packages/open-bitcoin-chainstate/src/engine/overlay_apply.rs`                      | `0bce54ac9cbe8024fd80ea704e7f6a7a0813f963d400a971b52e2919bd0908f9` | production                                      |
| `packages/open-bitcoin-chainstate/src/engine/stage.rs`                              | `475ac4c27ac612a61b69805c313d5ca8235b8720608dddf2a9584163a0c2641e` | production                                      |
| `packages/open-bitcoin-chainstate/src/engine/work_authority.rs`                     | `6c3412159f23e761a283adcccbaf45af3b0cf9aca62af607184d5fc612785dbb` | production                                      |
| `packages/open-bitcoin-chainstate/src/error.rs`                                     | `bcebfceedef8d23ab4defa74cfec2b4e59f07d89c8867faa52ac8f8c44b33e4f` | production                                      |
| `packages/open-bitcoin-chainstate/src/lib.rs`                                       | `d6a0d6b678ac45ac9e50bfbe68f23de9f4ffd2d8c476b0d67540b3110c16f189` | production                                      |
| `packages/open-bitcoin-chainstate/src/types.rs`                                     | `6ecbd780c0c0501a28b749a496ca4f7431f8ab89c36a0af1a5c487ca9c5b4f62` | production                                      |
| `packages/open-bitcoin-consensus/src/block.rs`                                      | `a5219e5800c8b247afecbb509f481bc9d0cb8049ccd34fb48f10b07ea2bf8cea` | production                                      |
| `packages/open-bitcoin-consensus/src/chain_proof.rs`                                | `9201f9f7604c66f6d3a514eb9f498ff658ebf7ec07e22fab04e311d7c55b785f` | production                                      |
| `packages/open-bitcoin-consensus/src/crypto.rs`                                     | `631710ae9083ab3a6acd21e923aba9fb052a016b70b2d4380f55530d01fb108d` | production                                      |
| `packages/open-bitcoin-network/src/header_store.rs`                                 | `4955b6481396d6dcec0172921e8f10f1fcf4f4b7b655611a73a70f9fc79d9803` | production                                      |
| `packages/open-bitcoin-network/src/header_store/eligibility.rs`                     | `751b989eec04f97e280a0eccc2dc234a6fa413bd0804e5a3d1e802b0f120cc9c` | production                                      |
| `packages/open-bitcoin-network/src/header_store/proof.rs`                           | `3fd34b9d5f0b48939e50a731a330065d84c427b12c312bb18da41e856b662d27` | production                                      |
| `packages/open-bitcoin-network/src/lib.rs`                                          | `5f5250766167d46197f8f89f9355ac15cb97ae792f6935af1f2c7c5f58620fe0` | production                                      |
| `packages/open-bitcoin-network/src/peer.rs`                                         | `4673cc6c9ab5d1885e304c20c5e0bf9ee67020330eea7d2f17a9177ee6f4a615` | production                                      |
| `packages/open-bitcoin-network/src/peer/eligibility.rs`                             | `893c33ce182e4e9e5813a4a4be16f3e4f4ca1fcd5eb2a4dff59883a39aa78b85` | production                                      |
| `packages/open-bitcoin-network/src/peer/message_dispatch.rs`                        | `53f2e79e42cf69072166e0b511009766d98d8fb83b561691548d7f3dfadf649c` | production                                      |
| `packages/open-bitcoin-node/src/chainstate.rs`                                      | `1d25580fd120dd80c5129d065ae5e31da460392be9112f6c2ef767f2defb2ba8` | production                                      |
| `packages/open-bitcoin-node/src/chainstate/filter_index.rs`                         | `9cf19c472ce8b25d0c3d0c33dc7e2465d689fe813333baeb50e558dced088c48` | production                                      |
| `packages/open-bitcoin-node/src/chainstate/fjall_store.rs`                          | `57d814acd77d5f16741669ee2844f6e437410cd2f6dd999b839326d18f66d429` | production                                      |
| `packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs`                      | `fddb8a4b1e730b29727842cc3ca743176ed24774dbc6b9e557de407aa525b04c` | production                                      |
| `packages/open-bitcoin-node/src/chainstate/flush_lifecycle/fjall_sink.rs`           | `70f657e0b7cdaa5080e364e4878b51aaa7926315860bd666e0313eb5540b3645` | changes confined to cfg(test) module at line 68 |
| `packages/open-bitcoin-node/src/chainstate/live_fixture.rs`                         | `13ac5561c3e40eadc7cbb195c70dfd3efb6446af5509e38035e09e4062289a11` | cfg(test) module                                |
| `packages/open-bitcoin-node/src/chainstate/replay.rs`                               | `2f3764dcb7a16eac00354cf22a1ea9c3c9376cb0b79e91ef0779267b74b3ffd1` | production                                      |
| `packages/open-bitcoin-node/src/chainstate/validation_history.rs`                   | `de507925613e44fdea9cb5b3e8766c8aebc5070302a70db7fd59fd6c1dcbee62` | production                                      |
| `packages/open-bitcoin-node/src/chainstate/work_authority.rs`                       | `18fe2dc025deb1b556aa6570166e93d4951dc45e67b4ead69b99a12e255f9370` | production                                      |
| `packages/open-bitcoin-node/src/mempool.rs`                                         | `4870cacb131adcdf4e9b8382622836deef1381600e07a06cf2f3e4d265541ba5` | changes confined to inline tests                |
| `packages/open-bitcoin-node/src/network.rs`                                         | `2a273fdf3b9278e4477fc0731ac338c2d40446bd45614d7248f46b63f32a89ab` | production                                      |
| `packages/open-bitcoin-node/src/network/action_translation.rs`                      | `92c8139de45c123ca759b6696ba70de41c6141ddec3e80e3f3738f4838cd5c0d` | production                                      |
| `packages/open-bitcoin-node/src/network/candidate_eligibility.rs`                   | `14e3334a388bdde9037fc1521a86e8a42d3ede428fc0fff8b4fe5be078b1ca1a` | production                                      |
| `packages/open-bitcoin-node/src/network/canonical_admission.rs`                     | `deb77a064624d2f0ecee6998849633c503aacf43ebd1106dd6adae7cdc409d19` | production                                      |
| `packages/open-bitcoin-node/src/network/compact_receive_candidates.rs`              | `f28a565dc5895851d962d86b280bd289ebd397e61f72516f20b8b923d2c96a3a` | changes confined to inline tests                |
| `packages/open-bitcoin-node/src/network/header_sync.rs`                             | `dcd625d4dd241fb31f97d67d78ac8cd7886309f1927b91abb6fd4a1074564b8e` | production                                      |
| `packages/open-bitcoin-node/src/network/inventory.rs`                               | `6eae72087d1cbf351576c08462351df2ec47c5ba271a45865b5965b6c684e7fc` | production                                      |
| `packages/open-bitcoin-node/src/network/limited_serve.rs`                           | `3ef1325a9034e9445c5c87dd8522a4480b79f4a7aebc1da55abb7cc3ab335921` | changes confined to cfg(test) module at line 87 |
| `packages/open-bitcoin-node/src/network/mempool_lifecycle.rs`                       | `fa186eeafab8925c40006f7a20671c265af261a364698712b751562b6baa8bbb` | production                                      |
| `packages/open-bitcoin-node/src/network/peer_network_clone.rs`                      | `9afc6882d9ea027287f09513554f5679af12daa64de927a76ecc65352cdbbadb` | production                                      |
| `packages/open-bitcoin-node/src/network/relay_serving.rs`                           | `87c1976f801bc4ddd25757ab608597ace0c574e970e8199f8ea6733274a37325` | production                                      |
| `packages/open-bitcoin-node/src/network/runtime_authority.rs`                       | `65835d0307dd58ee1572efab8a8eb83794cc210c1748ad87f5218bb3cf082212` | production                                      |
| `packages/open-bitcoin-node/src/network/runtime_authority/filter_index.rs`          | `dd05bf16a0e4be7cbff5f4d05f5ba25984ed2fa9e366a1d0e2efa95a190508a6` | changes confined to cfg(test) helpers           |
| `packages/open-bitcoin-node/src/network/runtime_authority/filter_index/catch_up.rs` | `d1afde6b9d01c22a2adf253945f3b5b0f6ed55d9974842aaf73cd9ae9203a621` | production                                      |
| `packages/open-bitcoin-node/src/network/runtime_authority/filter_index/reorg.rs`    | `0919a19fa34d573d21b3d1b4e8b17f10f49353f1f9b93b93f1ed8113fa88b85c` | production                                      |
| `packages/open-bitcoin-node/src/network/runtime_authority/live_fixture.rs`          | `5fb001358e2938b00cf50f89335dcc40f099135f3b6649754c902f779306918a` | cfg(test) module                                |
| `packages/open-bitcoin-node/src/network/runtime_authority/prune_flush.rs`           | `262bbe4a989e96309282ed026cd82ed53d6b664b960024f3fd197d1ff747cecb` | production                                      |
| `packages/open-bitcoin-node/src/network/validation_history.rs`                      | `b2a9e5d489797818dada897d8af885c825b1ffa5cd5dac241ca608901f8f3e89` | production                                      |
| `packages/open-bitcoin-node/src/network/work_authority.rs`                          | `a0952b43f0f4e6eb598421abfb6e6c2e8d01d30ebbb224ba80bd8c6fe8f71447` | production                                      |
| `packages/open-bitcoin-node/src/storage/chain_compatibility.rs`                     | `37b3763d3cdfed99e60d11bf7ac705fcdcfb933934f5640a52033146d029370f` | production                                      |
| `packages/open-bitcoin-node/src/storage/coins_view.rs`                              | `f71ff78ac7a7c4e65d657964a7f415e3c11e9684388fb4456c93a974f1c5ee56` | production                                      |
| `packages/open-bitcoin-node/src/storage/fjall_store.rs`                             | `033e588083230861e17e4fa8ce2aee86198616ecf5a3a23779eb4d296291dba1` | changes confined to cfg(test) observability     |
| `packages/open-bitcoin-node/src/storage/fjall_store/filters/ownership.rs`           | `ae563e872349666dee2a27075806327afe769cbb70d14778dd41124b020eb78d` | changes confined to cfg(test) helper            |
| `packages/open-bitcoin-node/src/storage/fjall_store/filters/reorg.rs`               | `f87ae3fcb97304959b325e9b1b2248848d83da4a7979f2aa54eba5f0724e027d` | changes confined to inline tests at line 409    |
| `packages/open-bitcoin-node/src/storage/snapshot_codec.rs`                          | `2fefee5664d1f447a649b864362aa04f6594195088c6623ef5e94a8bbd0e98f6` | production                                      |
| `packages/open-bitcoin-node/src/storage/snapshot_codec/work.rs`                     | `b7d2fc5e127e523ea05fc3aee33657f3466a1b6f7a234221e209047ad310d5cd` | production                                      |
| `packages/open-bitcoin-node/src/sync.rs`                                            | `88cf2a12bca8a236a4a8d821afe96e17cdd13a63c5ff8f159da5d24e3dfbe035` | production                                      |
| `packages/open-bitcoin-node/src/sync/block_reconcile.rs`                            | `96341ba6541be2f3ed99d7a96745dbd88ce6ed254e4f5c01059fc4c21e2d70f0` | production                                      |
| `packages/open-bitcoin-node/src/sync/block_response.rs`                             | `718f8eb2fa0e552cb26fd43e62cd92ade6145973fc96b957885d54121060d11c` | production                                      |
| `packages/open-bitcoin-node/src/sync/live_fixture.rs`                               | `4da004e8b12a1b6bd0da8d460dde94b70a7c1119893fbe03ba2d048d707b9b9c` | production                                      |
| `packages/open-bitcoin-node/src/sync/open_runtime.rs`                               | `e0d80123609b1f85a9edb06c18f1c1682268215bace22f343a2c420357c338d5` | production                                      |
| `packages/open-bitcoin-node/src/sync/runtime_state.rs`                              | `610ad571f62023920b08666a5a0694aa081af610ebf1e637efd6bfc747007c25` | production                                      |
| `packages/open-bitcoin-node/src/sync/types/error.rs`                                | `18967e0b9b4c774c4d67e9c74712b37135203dd49c448096cabefb6ed340390c` | production                                      |
| `packages/open-bitcoin-node/src/wallet.rs`                                          | `6185397f395ccde53880b31c122b59b614a62596cc015c4d6599cf09111318cf` | changes confined to inline tests                |

______________________________________________________________________

_Reviewer: independent gsd-code-reviewer. Preliminary production audit only._
