---
phase: 162-real-prune-retention-and-integrated-parity-proof
reviewed: 2026-10-12T02:45:58Z
depth: standard
files_reviewed: 45
files_reviewed_list:
  - packages/open-bitcoin-chainstate/src/coins/tests/flush.rs
  - packages/open-bitcoin-node/src/chainstate.rs
  - packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs
  - packages/open-bitcoin-node/src/chainstate/flush_lifecycle/tests/execute_flush.rs
  - packages/open-bitcoin-node/src/chainstate/flush_lifecycle/tests/execute_flush/header_checkpoint.rs
  - packages/open-bitcoin-node/src/chainstate/flush_lifecycle/tests/recovery_and_persist.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/prune_flush.rs
  - packages/open-bitcoin-node/src/network/validation_history.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/tests/coins_migration.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/validation_history.rs
  - packages/open-bitcoin-node/src/storage/snapshot_codec/tests/chainstate_wallet_and_headers.rs
  - packages/open-bitcoin-node/src/sync/runtime_state/tests.rs
  - packages/open-bitcoin-node/src/sync/runtime_state/tests/header_checkpoint.rs
  - packages/open-bitcoin-node/src/sync/runtime_state/tests/header_checkpoint/faults.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/phase162.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/phase162/corruption.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/phase162/missing_history.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/phase162/publication_faults.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/phase162/recovery.rs
  - packages/open-bitcoin-node/src/sync/tests/peer_filter_transport.rs
  - packages/open-bitcoin-node/src/sync/tests/peer_filter_transport/phase162.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush/tests.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush/tests/phase162.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/automatic_prune/fixtures.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/daemon_sync.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/fixtures.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/inbound.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/lifecycle.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/outbound.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/restrictions.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/retention.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/rpc.rs
  - packages/open-bitcoin-rpc/src/config/loader/open_bitcoin_runtime.rs
  - packages/open-bitcoin-rpc/src/config/tests/blockfilter.rs
  - packages/open-bitcoin-rpc/src/config/tests/sync_configuration.rs
  - packages/open-bitcoin-rpc/src/inbound_listener/tests/peer_filters.rs
  - packages/open-bitcoin-rpc/src/inbound_listener/tests/peer_filters/phase162.rs
  - packages/open-bitcoin-wallet/src/descriptor/format.rs
  - packages/open-bitcoin-wallet/src/descriptor/tests/key_source_and_storage_helpers_cover_extended_branches.rs
  - packages/testdata/phase162/process.rs
  - packages/testdata/phase162/recipe.rs
  - scripts/basic-filter-oracle.py
findings:
  critical: 0
  warning: 0
  info: 0
  total: 0
status: clean
generated_by: gsd-code-reviewer
lifecycle_mode: yolo
phase_lifecycle_id: 162-2026-10-11T16-25-37
generated_at: 2026-10-12T02:45:58Z
source_digest: a4b0d71d01def301ba406d6b46c351ed8944d8d4508182ffb0dfd21960043575
review_scope_manifest: /tmp/phase162-review-runtime-scope.json
review_scope_manifest_sha256: ee221cfb38042ae17977b75268b9a5f1cc5b724226c6bf510d26ab78991e83b6
manifest_review_digest: 1adc17e8332c1805647a4675f029bdf9f0457bdb33ff479acae3854139deb407
implementation_digest: 3750ddc7d23753bd1cc1d016cee81b68fab34583072178088fc4e3ec5940f27a
reviewed_scope_digest: 87ca026afde741a2a8574e9a81d5910f876cbad725706131d20ddf869224ec08
diff_base: a77edab276a9daa086055aa61e6143ce86bc1218
prior_44_reviewed: 2026-10-12T02:11:10Z
prior_44_report_sha256: db2ebc747be7a45d2c8c68e5988212ad890839c64111a1a00b9a07817ab03926
---

# Phase 162: Current 45-Path Runtime Review

**Reviewed:** 2026-10-12T02:45:58Z
**Depth:** standard
**Files reviewed:** 45
**Status:** clean; zero open findings

## Existing Automatic-Prune Fixture Supplement

Read the entire added existing fixture file and actual diff, current 162-NATIVE-PRUNE-DIAGNOSIS.md, existing ordinary retention/reopen test and relevant public/strict HeaderStore and Fjall storage APIs. The prior44runtime/test paths are byte/hash unchanged. No actionable correctness or test-reliability issue remains in the corrected source. Production runtime, strict reader/writer, transport, Phase162 continuous recipe and six publication anchors are unchanged.

The old ordinary-prune fixture provided six disconnected nonzero-height headers with null parents. Its live legacy seed tolerated those codec facts, but the already-approved lazy checkpoint faithfully serialized them and the strict cold reader correctly refused the absent parent. The fixture-only correction supplies finite1002parent-linked codec headers at0..1001 through public HeaderStore::insert_header and saves their complete entries through the existing public save_header_entries path. The actual serialized height-zero root has a computed nonzero canonical hash and is the only null-parent entry. The same six sparse heights1/500/510/713/714/1001 select their actual header identities/heights/work for the existing single coins snapshot seed; original UTXO/wallet setup, six sparse payload/undo pairs, pressure construction and fault helpers remain unchanged.

Independent API inspection confirms seed_coins_from_snapshot and raw save_header_entries both invalidate validation coverage. No accepted-validation batch/publisher, extra genesis/coins authority seed, public visibility change or production status fabrication is introduced. UnknownLegacy/unvalidated provenance is a source-derived compatibility boundary here, not a direct public-provenance assertion in the new test. Coherent header metadata does not turn sparse codec-volume evidence into a continuous consensus-accepted history or prove ScriptsValid.

The corrected fast regression loads the actual saved strict header store, compares full reconstruction equality, checks all1002canonical identities, height-zero root, each parent edge and the same six sparse heights/actual work. It preserves original unknown confirmation-count metadata. Removing the actual h500 entry must produce exact MissingHeaderAncestor for that hash. The main ordinary-prune test remains SHA25687ee951bac32baaf010756ee603af702e24ae3b574358d95c02c551b20290ded, so ordered deletes, accounting, retained bodies, wallet refusal, injected metadata errors and all-handle reopen assertions are unchanged.

Current fixture: `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/automatic_prune/fixtures.rs`, 466lines, SHA256 `209b83454e17dfdbc3b63134ed2bc8262b021f8c70b8278c2d3f5fcd754e88dd`. The initial f0a6879e95675d700ebc0dc1368bde29dcc8f2b45a0d9af2712622dcf43380e7 candidate failed compilation before test execution due to a private provenance call and a missing borrow. The correction stays within this fixture, uses public APIs/observed metadata only and changes no visibility or production API. That intermediate hash is superseded and is not represented as reviewed final coverage.

## Actual Evidence and Pending Verification

The parent reported the corrected fast regression actually passed1/0,104filtered,in1.09s, after the compilation correction. This reviewer inspected the final source and hashes and ran no heavy Rust test.

The prior tail preflight remains **failed**: its node stage passed1695/0 with4ignored, RPC library passed375/0 with1ignored, and the complete phase162_integrated_lifecycle passed; the daemon stage had102passed/one old ordinary-retention cold-reopen failure. Focused RED reproduced that same failure in11.98s. These facts neither invalidate the separate continuous Phase162 scenario nor convert the failed tail/full-native attempt into a pass.

Pressure GREEN, all automatic-prune cases and strict affected lint were still pending in the debugger's serial run at this review handoff. Parent retains remaining workspace/tail and mandatory full Native5. Native1–4 failures, all three Pending requirements, owner in_progress and unaudited/unshipped milestone boundaries remain unchanged. No native/formal/Git completion is claimed.

## Current Exact 45-Path Coverage

All45paths match the refreshed manifest in bytes and SHA256. Runtime coverage is SHA256 of ordered newline-joined path:sha256 rows with no trailing newline: `87ca026afde741a2a8574e9a81d5910f876cbad725706131d20ddf869224ec08`. Inherited full70review digest is `1adc17e8332c1805647a4675f029bdf9f0457bdb33ff479acae3854139deb407`; the separate six-anchor source digest `a4b0d71d01def301ba406d6b46c351ed8944d8d4508182ffb0dfd21960043575` remains unchanged and does not substitute for full coverage.

| Reviewed path | UTF-8 bytes | SHA256 |
| --- | ---: | --- |
| `packages/open-bitcoin-chainstate/src/coins/tests/flush.rs` | 7005 | `f36c460d84b7eefdac849d4afff474b38fbdccde1585cdccd5971a5496b73c4e` |
| `packages/open-bitcoin-node/src/chainstate.rs` | 20425 | `61f9b1fb5d5f3e0643ffeaa12a93f2651cd2a82ee90a5e81694ed551d3ae4a7f` |
| `packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs` | 21417 | `3b31f7b4b0489992e547079e549b808e56d1f945ffd8ae70e9a73bfd3fe89324` |
| `packages/open-bitcoin-node/src/chainstate/flush_lifecycle/tests/execute_flush.rs` | 17008 | `b48c83efb66841ecf9dff4ce35fae6810045d83227f9275a025b6fb5693095db` |
| `packages/open-bitcoin-node/src/chainstate/flush_lifecycle/tests/execute_flush/header_checkpoint.rs` | 9162 | `70ee8bad13671a574472ee93e0fde9d51fbd3a98acaa6de5dd9479c7a9a66eb6` |
| `packages/open-bitcoin-node/src/chainstate/flush_lifecycle/tests/recovery_and_persist.rs` | 5695 | `bf7d12d4a795c90e2b1448d5446f5dc170e9945e3463a4c781c00dfc45a3337b` |
| `packages/open-bitcoin-node/src/network/runtime_authority/prune_flush.rs` | 7422 | `8deb3e561d9948a4a00c35032e7b9f9e6078ac9eddcca2747c761d564782b814` |
| `packages/open-bitcoin-node/src/network/validation_history.rs` | 6371 | `9083b2b04f8198d6a8c76c3af2cd86177248036ecd7ec2c8faf3cd77c251cbf0` |
| `packages/open-bitcoin-node/src/storage/fjall_store/tests/coins_migration.rs` | 19667 | `1144bdd64916361ba1f523c9c30e6842a194dddf07a0aa3100479d0fa4d5f843` |
| `packages/open-bitcoin-node/src/storage/fjall_store/validation_history.rs` | 17962 | `23b7cff93e0d0c1f38a73cdcb8211c8aa0e2c45b102c022f6d5fb44ae23ac471` |
| `packages/open-bitcoin-node/src/storage/snapshot_codec/tests/chainstate_wallet_and_headers.rs` | 9470 | `65a3fddef450930be7e0ecd96f8fc3efb940d2eb195d542a2ea633152633e0db` |
| `packages/open-bitcoin-node/src/sync/runtime_state/tests.rs` | 5295 | `e50301dd41811c8d9b2afcecc070eac32b07183a8f3ffe3622bcf40d45ba7f28` |
| `packages/open-bitcoin-node/src/sync/runtime_state/tests/header_checkpoint.rs` | 20234 | `831e8741d8e4f7d1a746a85a5f359b9b18e00ff96a069d832a4a6517715185cb` |
| `packages/open-bitcoin-node/src/sync/runtime_state/tests/header_checkpoint/faults.rs` | 8635 | `708606fc9c75ea37cfc92c16e886233dcb343b3c2d79c80acd202beee1361678` |
| `packages/open-bitcoin-node/src/sync/tests/filter_index.rs` | 13115 | `a4b4f52714115027da7ea47c6719222d6d9ce99bc0abb65f8c304f39cc8d99f4` |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/phase162.rs` | 14611 | `623b910dba79389972fcbaf4c22c69d6af827b1531035133a95b729fee384580` |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/phase162/corruption.rs` | 12167 | `6ba3fc4cce7df60cdd0eefd9191250848fc02b5feff4a86e1d14cf100547941e` |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/phase162/missing_history.rs` | 11533 | `43b233fa1b7f9685f3ab8f24c508f84bd50b2731739b733801d57e4eca5c021a` |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/phase162/publication_faults.rs` | 18628 | `a0d01c2dd4f6e4d61fd004e8c01aaf2fc6890f0a2a8d5d1e075636f72dbe108e` |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/phase162/recovery.rs` | 15159 | `9deca92df80b1259c1dd7d04a592fd31058af6795127cb77315318a364d210b7` |
| `packages/open-bitcoin-node/src/sync/tests/peer_filter_transport.rs` | 19310 | `c1b2a60097ef7af50290f4511073d6bed2182e4ab61e1f13c11fe8532fffbd7a` |
| `packages/open-bitcoin-node/src/sync/tests/peer_filter_transport/phase162.rs` | 11654 | `3610463d888bb31d2319d435270bc59f71448c20d536f7bb0d242ac6f6cd7020` |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush/tests.rs` | 3600 | `e19289ba0fbef0d0b7252cb79de7c370bfb34d6c90fe367feab6677ff957696d` |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush/tests/phase162.rs` | 15138 | `550e8f4ad1045c2eb7c05abcd774856c0ed66bbb232f0c5555d542dfafabb7e8` |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/automatic_prune/fixtures.rs` | 16218 | `209b83454e17dfdbc3b63134ed2bc8262b021f8c70b8278c2d3f5fcd754e88dd` |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/daemon_sync.rs` | 19867 | `74992724f705facdd800b84f2622ebd66342873c33ae995292cb8a653924a203` |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index.rs` | 12741 | `e7ce05075e3f5df520cbaf4375a9267f07d60b5ef3d4ca5a7d613f352284283d` |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162.rs` | 5181 | `f14b8f96862608730126bc2deb3ff60c4ec8004ee25f155f4666f3dbeb529e7b` |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/fixtures.rs` | 15190 | `faa009380301d6e6acbf032fd814f5cf9e31ddcbdb629787477fe5fbc5d62615` |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/inbound.rs` | 23886 | `1d991eeabfe62e1722828a9cdeb79fbf9ebc122b7a9e0fdb3b7d6ac0b85e7c60` |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/lifecycle.rs` | 7857 | `0dc0300982e189db1d739b0730ff93d032d95981534c5a0fcd231fe51d3353af` |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/outbound.rs` | 21943 | `06966508a66d211a3ee21e547464902cb1572780362031426778c3df76d29ab4` |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/restrictions.rs` | 21994 | `e81ac2e1c4f7d459813b2e50c40dae9709571f765b94e7170e277642d9d232c6` |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/retention.rs` | 13433 | `9b562a9715db42366decde5f91a0feccd10d453e728afa3e741953d82011a4ff` |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/rpc.rs` | 5185 | `3ef67bdf9dde29a7dabba37ec03c7325519d2a22f9c077d81869ffd8e18259a4` |
| `packages/open-bitcoin-rpc/src/config/loader/open_bitcoin_runtime.rs` | 17301 | `506a67537f37a3f6d3ce2f33b64abf063bd692b4e9e330d258a30967ce2202f0` |
| `packages/open-bitcoin-rpc/src/config/tests/blockfilter.rs` | 20683 | `62a15ac95ff0142aefbb4af01df6e8c08fe0dfc9360103c19a668a60be1f948d` |
| `packages/open-bitcoin-rpc/src/config/tests/sync_configuration.rs` | 20448 | `632b87a5559b597a8c56ec18a9afcdff9ee6cd61a0346efeaf391447bb93e079` |
| `packages/open-bitcoin-rpc/src/inbound_listener/tests/peer_filters.rs` | 21290 | `5f173ac212b0b6845ffe64c919d8c8f68c735e283e4c23d64eacd9b96aa6b667` |
| `packages/open-bitcoin-rpc/src/inbound_listener/tests/peer_filters/phase162.rs` | 22174 | `6134ec09df47b94a485915294cf46599b762ac5b99c16cd67cc079b822ae47a7` |
| `packages/open-bitcoin-wallet/src/descriptor/format.rs` | 19585 | `03260f29ce116c49c43b0d04869bb4205b4dc73783bd579a8edb7f89e04ad417` |
| `packages/open-bitcoin-wallet/src/descriptor/tests/key_source_and_storage_helpers_cover_extended_branches.rs` | 23626 | `371f37b62660fa770747ae9ec33edddea43df66a6b745f2498e9456fd57c9207` |
| `packages/testdata/phase162/process.rs` | 18232 | `628204b796e112a94f1ca0d24fb17c3fedfce85a655567df17c074634bbc0c26` |
| `packages/testdata/phase162/recipe.rs` | 7833 | `82c73bc4dbd2990bf5b19947defa5b1b90aada230363b44978a691c7dac21497` |
| `scripts/basic-filter-oracle.py` | 12906 | `103ce6a1e4cf3af6c8ca4f3a6ac7ef5ee3a11bd16b0db4135cb5a655c783b993` |

## Preserved Prior 44-Path Runtime Review — 2026-10-12T02:11:10Z

The complete prior report body follows verbatim, including its preserved original42review and Native4test-guard history. Then-current suite/pending status remains historical.


# Phase 162: Current 44-Path Runtime Review

**Reviewed:** 2026-10-12T02:11:10Z
**Depth:** standard
**Files reviewed:** 44
**Status:** clean; zero open findings

## Native4 Test-Guard Supplement

The original 42 runtime paths are byte/hash unchanged. Reviewed both additional existing Rust test files in full, their actual baseline diffs, actual 162-NATIVE-RUST-DIAGNOSIS.md and the unchanged approved Plan 15 recovery_and_persist.rs guard. No actionable correctness or test-reliability finding was identified. These are test-only changes; actual runtime behavior, accepted history, pruning, transport and all six publication anchors remain unchanged.

The core test replaces an obsolete whole-file contiguous-call assertion with the existing Plan 15 whitespace-normalized unique-function/body route assertions. It specifically verifies persist uses IfNeeded, flush_with_mode forwards mode/now/disk to flush_applying_plan, the plan delegate reaches the private checkpoint wrapper and the wrapper invokes the actual lifecycle. The full-file leftover-snapshot prohibition, test name, AAA organization and adjacent tests remain intact. The new guard matches the already reviewed actual four-hop production route. It supplements the existing comment/literal/ownership-aware TypeScript mutation evidence rather than replacing those stronger controls.

The node migration file changes only whitespace normalization of the chainstate include_str input in its final structural test. Every preexisting runtime-state persist_progress/no-save/no-seed assertion and chainstate lifecycle/no-snapshot assertion remains unchanged. The other migration/hydration tests, fixtures, typed error assertions and source-file ownership are unchanged.

The actual new rows are:

| File | Lines | SHA256 |
| --- | ---: | --- |
| `packages/open-bitcoin-chainstate/src/coins/tests/flush.rs` | 220 | `f36c460d84b7eefdac849d4afff474b38fbdccde1585cdccd5971a5496b73c4e` |
| `packages/open-bitcoin-node/src/storage/fjall_store/tests/coins_migration.rs` | 600 | `1144bdd64916361ba1f523c9c30e6842a194dddf07a0aa3100479d0fa4d5f843` |

Both paths already existed with parity breadcrumbs; no new source file was introduced. Root registered both actual existing paths as two additional Phase 162 evidence entries. Independent tooling review owns that JSON registration delta and the final 25-path tooling coverage.

## Verification Boundary

Parent reported actual Native4 failure after 957,941ms, exit101, following passing Bun, formatting, strict Clippy and all-target build stages; core test execution had 327 passed/one stale structural-guard failure before node/RPC tests. That full default native attempt remains **failed**. The debugger reproduced each old assertion failure separately and the parent supplied exact-two focused GREEN outcomes, one passing test per crate. This reviewer ran no Cargo/Bazel or heavy test and does not claim the debugger's still-running strict affected-crate Clippy, full core or related node suites completed.

Parent retains remaining workspace/tail verification and mandatory full Native5. No native/formal/requirement/Git completion is implied by this clean source review. CFPR-02, CFGR-01 and CFGR-02 remain Pending; v2.5 remains unaudited and unshipped.

## Current Exact 44-Path Coverage

Every current path is byte/hash-equal to the refreshed manifest. Current runtime coverage is SHA256 of these ordered path:sha256 rows joined with newline and no trailing newline: `a78b00c5abda840ec6d052e65af4a04e0213cdf026e34090bca4e8d8ecb9ea84`. Inherited full69 parent review digest is `122c75ec5151f79fc654469f5cfb5c1190d5aaaeb9f8cb3b594a2f7e41f5c11a`. The separate six-anchor source_digest `a4b0d71d01def301ba406d6b46c351ed8944d8d4508182ffb0dfd21960043575` is unchanged and is not full-scope coverage. Original 42-path history is preserved below.

| Reviewed path | UTF-8 bytes | SHA256 |
| --- | ---: | --- |
| `packages/open-bitcoin-chainstate/src/coins/tests/flush.rs` | 7005 | `f36c460d84b7eefdac849d4afff474b38fbdccde1585cdccd5971a5496b73c4e` |
| `packages/open-bitcoin-node/src/chainstate.rs` | 20425 | `61f9b1fb5d5f3e0643ffeaa12a93f2651cd2a82ee90a5e81694ed551d3ae4a7f` |
| `packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs` | 21417 | `3b31f7b4b0489992e547079e549b808e56d1f945ffd8ae70e9a73bfd3fe89324` |
| `packages/open-bitcoin-node/src/chainstate/flush_lifecycle/tests/execute_flush.rs` | 17008 | `b48c83efb66841ecf9dff4ce35fae6810045d83227f9275a025b6fb5693095db` |
| `packages/open-bitcoin-node/src/chainstate/flush_lifecycle/tests/execute_flush/header_checkpoint.rs` | 9162 | `70ee8bad13671a574472ee93e0fde9d51fbd3a98acaa6de5dd9479c7a9a66eb6` |
| `packages/open-bitcoin-node/src/chainstate/flush_lifecycle/tests/recovery_and_persist.rs` | 5695 | `bf7d12d4a795c90e2b1448d5446f5dc170e9945e3463a4c781c00dfc45a3337b` |
| `packages/open-bitcoin-node/src/network/runtime_authority/prune_flush.rs` | 7422 | `8deb3e561d9948a4a00c35032e7b9f9e6078ac9eddcca2747c761d564782b814` |
| `packages/open-bitcoin-node/src/network/validation_history.rs` | 6371 | `9083b2b04f8198d6a8c76c3af2cd86177248036ecd7ec2c8faf3cd77c251cbf0` |
| `packages/open-bitcoin-node/src/storage/fjall_store/tests/coins_migration.rs` | 19667 | `1144bdd64916361ba1f523c9c30e6842a194dddf07a0aa3100479d0fa4d5f843` |
| `packages/open-bitcoin-node/src/storage/fjall_store/validation_history.rs` | 17962 | `23b7cff93e0d0c1f38a73cdcb8211c8aa0e2c45b102c022f6d5fb44ae23ac471` |
| `packages/open-bitcoin-node/src/storage/snapshot_codec/tests/chainstate_wallet_and_headers.rs` | 9470 | `65a3fddef450930be7e0ecd96f8fc3efb940d2eb195d542a2ea633152633e0db` |
| `packages/open-bitcoin-node/src/sync/runtime_state/tests.rs` | 5295 | `e50301dd41811c8d9b2afcecc070eac32b07183a8f3ffe3622bcf40d45ba7f28` |
| `packages/open-bitcoin-node/src/sync/runtime_state/tests/header_checkpoint.rs` | 20234 | `831e8741d8e4f7d1a746a85a5f359b9b18e00ff96a069d832a4a6517715185cb` |
| `packages/open-bitcoin-node/src/sync/runtime_state/tests/header_checkpoint/faults.rs` | 8635 | `708606fc9c75ea37cfc92c16e886233dcb343b3c2d79c80acd202beee1361678` |
| `packages/open-bitcoin-node/src/sync/tests/filter_index.rs` | 13115 | `a4b4f52714115027da7ea47c6719222d6d9ce99bc0abb65f8c304f39cc8d99f4` |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/phase162.rs` | 14611 | `623b910dba79389972fcbaf4c22c69d6af827b1531035133a95b729fee384580` |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/phase162/corruption.rs` | 12167 | `6ba3fc4cce7df60cdd0eefd9191250848fc02b5feff4a86e1d14cf100547941e` |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/phase162/missing_history.rs` | 11533 | `43b233fa1b7f9685f3ab8f24c508f84bd50b2731739b733801d57e4eca5c021a` |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/phase162/publication_faults.rs` | 18628 | `a0d01c2dd4f6e4d61fd004e8c01aaf2fc6890f0a2a8d5d1e075636f72dbe108e` |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/phase162/recovery.rs` | 15159 | `9deca92df80b1259c1dd7d04a592fd31058af6795127cb77315318a364d210b7` |
| `packages/open-bitcoin-node/src/sync/tests/peer_filter_transport.rs` | 19310 | `c1b2a60097ef7af50290f4511073d6bed2182e4ab61e1f13c11fe8532fffbd7a` |
| `packages/open-bitcoin-node/src/sync/tests/peer_filter_transport/phase162.rs` | 11654 | `3610463d888bb31d2319d435270bc59f71448c20d536f7bb0d242ac6f6cd7020` |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush/tests.rs` | 3600 | `e19289ba0fbef0d0b7252cb79de7c370bfb34d6c90fe367feab6677ff957696d` |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush/tests/phase162.rs` | 15138 | `550e8f4ad1045c2eb7c05abcd774856c0ed66bbb232f0c5555d542dfafabb7e8` |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/daemon_sync.rs` | 19867 | `74992724f705facdd800b84f2622ebd66342873c33ae995292cb8a653924a203` |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index.rs` | 12741 | `e7ce05075e3f5df520cbaf4375a9267f07d60b5ef3d4ca5a7d613f352284283d` |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162.rs` | 5181 | `f14b8f96862608730126bc2deb3ff60c4ec8004ee25f155f4666f3dbeb529e7b` |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/fixtures.rs` | 15190 | `faa009380301d6e6acbf032fd814f5cf9e31ddcbdb629787477fe5fbc5d62615` |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/inbound.rs` | 23886 | `1d991eeabfe62e1722828a9cdeb79fbf9ebc122b7a9e0fdb3b7d6ac0b85e7c60` |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/lifecycle.rs` | 7857 | `0dc0300982e189db1d739b0730ff93d032d95981534c5a0fcd231fe51d3353af` |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/outbound.rs` | 21943 | `06966508a66d211a3ee21e547464902cb1572780362031426778c3df76d29ab4` |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/restrictions.rs` | 21994 | `e81ac2e1c4f7d459813b2e50c40dae9709571f765b94e7170e277642d9d232c6` |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/retention.rs` | 13433 | `9b562a9715db42366decde5f91a0feccd10d453e728afa3e741953d82011a4ff` |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/rpc.rs` | 5185 | `3ef67bdf9dde29a7dabba37ec03c7325519d2a22f9c077d81869ffd8e18259a4` |
| `packages/open-bitcoin-rpc/src/config/loader/open_bitcoin_runtime.rs` | 17301 | `506a67537f37a3f6d3ce2f33b64abf063bd692b4e9e330d258a30967ce2202f0` |
| `packages/open-bitcoin-rpc/src/config/tests/blockfilter.rs` | 20683 | `62a15ac95ff0142aefbb4af01df6e8c08fe0dfc9360103c19a668a60be1f948d` |
| `packages/open-bitcoin-rpc/src/config/tests/sync_configuration.rs` | 20448 | `632b87a5559b597a8c56ec18a9afcdff9ee6cd61a0346efeaf391447bb93e079` |
| `packages/open-bitcoin-rpc/src/inbound_listener/tests/peer_filters.rs` | 21290 | `5f173ac212b0b6845ffe64c919d8c8f68c735e283e4c23d64eacd9b96aa6b667` |
| `packages/open-bitcoin-rpc/src/inbound_listener/tests/peer_filters/phase162.rs` | 22174 | `6134ec09df47b94a485915294cf46599b762ac5b99c16cd67cc079b822ae47a7` |
| `packages/open-bitcoin-wallet/src/descriptor/format.rs` | 19585 | `03260f29ce116c49c43b0d04869bb4205b4dc73783bd579a8edb7f89e04ad417` |
| `packages/open-bitcoin-wallet/src/descriptor/tests/key_source_and_storage_helpers_cover_extended_branches.rs` | 23626 | `371f37b62660fa770747ae9ec33edddea43df66a6b745f2498e9456fd57c9207` |
| `packages/testdata/phase162/process.rs` | 18232 | `628204b796e112a94f1ca0d24fb17c3fedfce85a655567df17c074634bbc0c26` |
| `packages/testdata/phase162/recipe.rs` | 7833 | `82c73bc4dbd2990bf5b19947defa5b1b90aada230363b44978a691c7dac21497` |
| `scripts/basic-filter-oracle.py` | 12906 | `103ce6a1e4cf3af6c8ca4f3a6ac7ef5ee3a11bd16b0db4135cb5a655c783b993` |

## Preserved Original 42-Path Runtime Review — 2026-10-12T00:36:42Z

The original report body follows verbatim. Its then-current parent/tooling/native status and 42-path coverage are historical. The unchanged source hashes preserve its runtime findings; this supplement adds the two actual test paths.


# Phase 162: Runtime Code Review Report

**Reviewed:** 2026-10-12T00:36:42Z
**Depth:** standard
**Files reviewed:** 42
**Status:** clean

## Summary

Read every assigned Rust/Python file in its current form, compared tracked changes with baseline `a77edab276a9daa086055aa61e6143ce86bc1218`, and inspected relevant production callers and effect boundaries. New source files were included in full. No actionable correctness, security or test-reliability finding remains within this assigned 42-file runtime scope. This conclusion does not include the independent 23-file tooling/document/registry scope.

Material guidance was the actual root AGENTS.md and Repo-Local Guidance, AGENTS.bright-builds.md, placeholder-only standards-overrides.md, standards/index.md, architecture, code-shape, testing, verification, local-guidance and Rust standards, the GSD code-review skill/workflow, Phase 162 CONTEXT/UI-SPEC, current integrated parity proof and final Plan 12 summary. Both canonical active lesson files were read completely: 7,188 bytes and 2,397 conservative estimated tokens. The existing audit baseline creates no new audit trigger. No project skills were present in the prescribed directories and none of the 42 paths was Git-ignored; .claudeignore was absent.

## Review Plan and Completion

- [x] Resolve instructions, active lessons, phase scope and final proof boundaries.
- [x] Read all 42 assigned files, their current diffs/new content and relevant imported callers.
- [x] Trace accepted history, durable coins/undo, retention and consuming close/reopen provenance.
- [x] Check actual RPC/peer/body/wallet consumers and complete-frame accounting/fault assertions.
- [x] Recompute each assigned file hash and preserve exact manifest/full-scope coverage.
- [x] Write this runtime-only report without source edits, staging, commits or pushes.

## Evidence and Caller Checks

1. **Accepted history and ownership.** The shared recipe asserts actual runtime maturity 100, continuous raw parentage, historical and same-block spends and actual wallet funding. The configured fixture records genuine connect receipts, checks persisted undo-derived scripts, real durable coins B and chain metadata, and creates the independent raw-block oracle before retention. The daemon maintenance harness drives the actual receive-independent loop, waits for completed turns and joins both successful and failed workers. Extension/reorg preserves stale immutable records and distinguishes processed work from coins-fenced safe progress. Fixture close consumes HTTP state, shared context, opened runtime, store and config; the handoff carries data/provenance only, and actual concurrent-open refusal/cold opens constrain all-handle closure.

2. **Real retention.** The authenticated manual caller enforces the keep window and checks paired body/undo loss through h20. Parsed prune=550 and the ordinary automatic flush/planner delete h21..714 while retaining h715..1002. The 584 nonactive codec pressure rows are not acceptance evidence or deletion candidates. SettledIdentity compares chain metadata/coins, the saved BASIC checkpoint and all 1,006 active/stale immutable records across each consuming reopen. The legal 550 MiB soft target remains unattained; allocated-file observations remain point measurements. No most-work or whole-chain Knots consensus differential is inferred from caller-supplied work or the synthetic root.

3. **Actual client paths.** RPC uses the authenticated HTTP handler and dispatcher, with unauthorized controls before successful dispatch; it is not represented as a physical HTTP-server roundtrip. Inbound sessions compare independent exact physical CF frames, predecessors, ordering and distinct positive h1000 active/stale checkpoints. Three granted sessions provide 60 frames/5,670 bytes; denied sessions require zero bytes and actual disconnect. Outbound uses public sync_once with TcpPeerTransport on the consumed original datadir, an explicit numeric loopback peer, empty DNS, finite message/round/retry limits and joined remote; 10 requests produce 20 frames/1,890 bytes, with peers/outboxes/charges checked after settlement.

4. **Body/wallet restrictions.** The bounded public descriptor import retains the original funded derivation-zero body/script and real h20/25,000-sat funding. WalletRegistry creation and public scan precede protected pruning; the public post-reorg scan captures the complete h1002 job before settled retention. The same parsed, handshake-complete, download-eligible body socket returns exact retained h715 Block before exact known/deleted h714 NotFound, with a Pong fence after each; distances 287/288 are inside the 290 window. The public later-range rescan refuses on deleted h20 creating history. The complete WalletSnapshot remains equal, and normalizing only Failed/error yields exact prior job equality including scanned-through/next/target/median-time/freshness on cold registry reload.

5. **Faults and transport receipts.** Mature focused histories exercise fresh/saved missing body/undo, interrupted required/eligible intents, ahead immutable rows and actual B104/meta101 disagreement. Publication faults sit at real undo/coins/metadata/record/checkpoint/protection/commit boundaries, preserve the allowed atomic effects and refuse fabricated success. Corrupt target/predecessor and missing-predecessor reads cover immediate and genuinely earned final-read authority; legitimate missing/unknown controls remain distinct. Both real writer executors acknowledge only successful full-frame outcomes. Partial first/second prefixes, cancellation and epoch invalidation preserve factual complete-frame counts while denying current-range authority; queue/deadline/Notify/disconnect and pending-future-drop cases check charge lifetime, generic suffix ownership and cleanup. The production transport executors and batch-drop guards were inspected directly.

6. **Production prerequisites and simplification.** Disabled configuration preserves selected network and exact selected DNS defaults while retaining mode/activation guards; daemon durable/transient consumers and CLI/JSONC precedence are covered. The lazy header supplier is reached only after the existing lifecycle selects actual prefix work, after body/undo/raw-header persistence and before paired unlink/coins/metadata. It validates store-bound admitted identities, atomically saves both snapshots and poisons uncertain commit outcomes. No-op four-stage counters and typed fault/retry/reopen cases preserve established behavior; old missing stale-header stores still refuse without backfill. Future descriptor storage removes the extra separator only. The unchanged node normalizer continues accepting legacy #ob:: text for parsing while preserving original_text, key/range/cursor/script and snapshot semantics. The focused production changes reuse the existing serialized owner and avoid a second authority or storage abstraction; no additional simplification finding is warranted.

Relevant cross-file callers inspected beyond the assigned manifest include chainstate/validation_history.rs, chainstate/fjall_store.rs, chainstate/flush_lifecycle/fjall_sink.rs and prune_apply.rs, network/runtime_authority.rs and automatic_prune.rs, sync/open_runtime.rs, sync/session/filter_serving.rs, open_bitcoind/coins_flush.rs, open-bitcoind.rs, inbound_listener/filter_serving.rs, context/rescan.rs and storage/snapshot_codec/wallet.rs. Their inspection supplies context and does not expand the declared 42-file changed-source scope.

## Exact File and Hash Coverage

Every current file hash matched its original manifest row. The manifest SHA256 above binds the original JSON bytes. The reviewed_scope_digest is SHA256 of these ordered `path:sha256` rows joined by newlines with no trailing newline. The required source_digest binds only the proof's six anchors, including the separately reviewed TypeScript oracle; it is not a substitute for this complete runtime coverage.

| Reviewed path | UTF-8 bytes | SHA256 |
| --- | ---: | --- |
| `packages/open-bitcoin-node/src/chainstate.rs` | 20425 | `61f9b1fb5d5f3e0643ffeaa12a93f2651cd2a82ee90a5e81694ed551d3ae4a7f` |
| `packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs` | 21417 | `3b31f7b4b0489992e547079e549b808e56d1f945ffd8ae70e9a73bfd3fe89324` |
| `packages/open-bitcoin-node/src/chainstate/flush_lifecycle/tests/execute_flush.rs` | 17008 | `b48c83efb66841ecf9dff4ce35fae6810045d83227f9275a025b6fb5693095db` |
| `packages/open-bitcoin-node/src/chainstate/flush_lifecycle/tests/execute_flush/header_checkpoint.rs` | 9162 | `70ee8bad13671a574472ee93e0fde9d51fbd3a98acaa6de5dd9479c7a9a66eb6` |
| `packages/open-bitcoin-node/src/chainstate/flush_lifecycle/tests/recovery_and_persist.rs` | 5695 | `bf7d12d4a795c90e2b1448d5446f5dc170e9945e3463a4c781c00dfc45a3337b` |
| `packages/open-bitcoin-node/src/network/runtime_authority/prune_flush.rs` | 7422 | `8deb3e561d9948a4a00c35032e7b9f9e6078ac9eddcca2747c761d564782b814` |
| `packages/open-bitcoin-node/src/network/validation_history.rs` | 6371 | `9083b2b04f8198d6a8c76c3af2cd86177248036ecd7ec2c8faf3cd77c251cbf0` |
| `packages/open-bitcoin-node/src/storage/fjall_store/validation_history.rs` | 17962 | `23b7cff93e0d0c1f38a73cdcb8211c8aa0e2c45b102c022f6d5fb44ae23ac471` |
| `packages/open-bitcoin-node/src/storage/snapshot_codec/tests/chainstate_wallet_and_headers.rs` | 9470 | `65a3fddef450930be7e0ecd96f8fc3efb940d2eb195d542a2ea633152633e0db` |
| `packages/open-bitcoin-node/src/sync/runtime_state/tests.rs` | 5295 | `e50301dd41811c8d9b2afcecc070eac32b07183a8f3ffe3622bcf40d45ba7f28` |
| `packages/open-bitcoin-node/src/sync/runtime_state/tests/header_checkpoint.rs` | 20234 | `831e8741d8e4f7d1a746a85a5f359b9b18e00ff96a069d832a4a6517715185cb` |
| `packages/open-bitcoin-node/src/sync/runtime_state/tests/header_checkpoint/faults.rs` | 8635 | `708606fc9c75ea37cfc92c16e886233dcb343b3c2d79c80acd202beee1361678` |
| `packages/open-bitcoin-node/src/sync/tests/filter_index.rs` | 13115 | `a4b4f52714115027da7ea47c6719222d6d9ce99bc0abb65f8c304f39cc8d99f4` |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/phase162.rs` | 14611 | `623b910dba79389972fcbaf4c22c69d6af827b1531035133a95b729fee384580` |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/phase162/corruption.rs` | 12167 | `6ba3fc4cce7df60cdd0eefd9191250848fc02b5feff4a86e1d14cf100547941e` |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/phase162/missing_history.rs` | 11533 | `43b233fa1b7f9685f3ab8f24c508f84bd50b2731739b733801d57e4eca5c021a` |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/phase162/publication_faults.rs` | 18628 | `a0d01c2dd4f6e4d61fd004e8c01aaf2fc6890f0a2a8d5d1e075636f72dbe108e` |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/phase162/recovery.rs` | 15159 | `9deca92df80b1259c1dd7d04a592fd31058af6795127cb77315318a364d210b7` |
| `packages/open-bitcoin-node/src/sync/tests/peer_filter_transport.rs` | 19310 | `c1b2a60097ef7af50290f4511073d6bed2182e4ab61e1f13c11fe8532fffbd7a` |
| `packages/open-bitcoin-node/src/sync/tests/peer_filter_transport/phase162.rs` | 11654 | `3610463d888bb31d2319d435270bc59f71448c20d536f7bb0d242ac6f6cd7020` |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush/tests.rs` | 3600 | `e19289ba0fbef0d0b7252cb79de7c370bfb34d6c90fe367feab6677ff957696d` |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush/tests/phase162.rs` | 15138 | `550e8f4ad1045c2eb7c05abcd774856c0ed66bbb232f0c5555d542dfafabb7e8` |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/daemon_sync.rs` | 19867 | `74992724f705facdd800b84f2622ebd66342873c33ae995292cb8a653924a203` |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index.rs` | 12741 | `e7ce05075e3f5df520cbaf4375a9267f07d60b5ef3d4ca5a7d613f352284283d` |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162.rs` | 5181 | `f14b8f96862608730126bc2deb3ff60c4ec8004ee25f155f4666f3dbeb529e7b` |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/fixtures.rs` | 15190 | `faa009380301d6e6acbf032fd814f5cf9e31ddcbdb629787477fe5fbc5d62615` |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/inbound.rs` | 23886 | `1d991eeabfe62e1722828a9cdeb79fbf9ebc122b7a9e0fdb3b7d6ac0b85e7c60` |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/lifecycle.rs` | 7857 | `0dc0300982e189db1d739b0730ff93d032d95981534c5a0fcd231fe51d3353af` |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/outbound.rs` | 21943 | `06966508a66d211a3ee21e547464902cb1572780362031426778c3df76d29ab4` |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/restrictions.rs` | 21994 | `e81ac2e1c4f7d459813b2e50c40dae9709571f765b94e7170e277642d9d232c6` |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/retention.rs` | 13433 | `9b562a9715db42366decde5f91a0feccd10d453e728afa3e741953d82011a4ff` |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/rpc.rs` | 5185 | `3ef67bdf9dde29a7dabba37ec03c7325519d2a22f9c077d81869ffd8e18259a4` |
| `packages/open-bitcoin-rpc/src/config/loader/open_bitcoin_runtime.rs` | 17301 | `506a67537f37a3f6d3ce2f33b64abf063bd692b4e9e330d258a30967ce2202f0` |
| `packages/open-bitcoin-rpc/src/config/tests/blockfilter.rs` | 20683 | `62a15ac95ff0142aefbb4af01df6e8c08fe0dfc9360103c19a668a60be1f948d` |
| `packages/open-bitcoin-rpc/src/config/tests/sync_configuration.rs` | 20448 | `632b87a5559b597a8c56ec18a9afcdff9ee6cd61a0346efeaf391447bb93e079` |
| `packages/open-bitcoin-rpc/src/inbound_listener/tests/peer_filters.rs` | 21290 | `5f173ac212b0b6845ffe64c919d8c8f68c735e283e4c23d64eacd9b96aa6b667` |
| `packages/open-bitcoin-rpc/src/inbound_listener/tests/peer_filters/phase162.rs` | 22174 | `6134ec09df47b94a485915294cf46599b762ac5b99c16cd67cc079b822ae47a7` |
| `packages/open-bitcoin-wallet/src/descriptor/format.rs` | 19585 | `03260f29ce116c49c43b0d04869bb4205b4dc73783bd579a8edb7f89e04ad417` |
| `packages/open-bitcoin-wallet/src/descriptor/tests/key_source_and_storage_helpers_cover_extended_branches.rs` | 23626 | `371f37b62660fa770747ae9ec33edddea43df66a6b745f2498e9456fd57c9207` |
| `packages/testdata/phase162/process.rs` | 18232 | `628204b796e112a94f1ca0d24fb17c3fedfce85a655567df17c074634bbc0c26` |
| `packages/testdata/phase162/recipe.rs` | 7833 | `82c73bc4dbd2990bf5b19947defa5b1b90aada230363b44978a691c7dac21497` |
| `scripts/basic-filter-oracle.py` | 12906 | `103ce6a1e4cf3af6c8ca4f3a6ac7ef5ee3a11bd16b0db4135cb5a655c783b993` |

## Verification and Claim Boundary

This review performed read-only source/diff/caller/hash checks. It ran no Cargo, Bazel, heavy tests or full native verifier because the parent owns exclusive build verification. Existing execution receipts in the final proof were reviewed as reported evidence, not represented as newly rerun tests. The parent clarified that the full default native run is held for a separate tooling correction.

All reviewed runtime files meet the review's scoped correctness, security and maintainability criteria; no issues found. The independent tooling report, declared-mitigation/security audit, actual full default native receipt and formal lifecycle verification remain separate gates. CFPR-02, CFGR-01 and CFGR-02 are still Pending, and v2.5 remains unaudited and unshipped. This runtime report does not activate requirements, certify the milestone or authorize Git finalization.

_Reviewer: gsd-code-reviewer. No source files modified._
