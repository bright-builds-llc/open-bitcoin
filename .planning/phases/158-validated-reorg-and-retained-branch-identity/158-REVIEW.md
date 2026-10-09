---
phase: 158-validated-reorg-and-retained-branch-identity
reviewed: "2026-10-09T04:32:57.858546Z"
depth: standard
diff_base: 2f21ac2052208c7e7a084ed006d908c5ccb714aa
files_reviewed: 67
files_reviewed_list:
  - .codex/tasks/todo.md
  - .planning/PROJECT.md
  - .planning/REQUIREMENTS.md
  - .planning/ROADMAP.md
  - .planning/STATE.md
  - .planning/phases/158-validated-reorg-and-retained-branch-identity/158-07-SUMMARY.md
  - .planning/phases/158-validated-reorg-and-retained-branch-identity/158-NATIVE-EVIDENCE.md
  - .planning/phases/158-validated-reorg-and-retained-branch-identity/158-UAT.md
  - README.md
  - docs/parity/index.json
  - docs/parity/source-breadcrumbs.json
  - docs/parity/v2-5-validated-reorg.md
  - packages/README.md
  - packages/open-bitcoin-chainstate/src/engine.rs
  - packages/open-bitcoin-chainstate/src/engine/stage.rs
  - packages/open-bitcoin-chainstate/src/engine/tests/apply_non_coinbase_transaction_returns_fee_and_records_undo/staged_connect_and_reorg.rs
  - packages/open-bitcoin-chainstate/src/filter_index/catch_up.rs
  - packages/open-bitcoin-chainstate/src/filter_index/catch_up/reorg.rs
  - packages/open-bitcoin-chainstate/src/filter_index/catch_up/tests.rs
  - packages/open-bitcoin-chainstate/src/filter_index/catch_up/tests/reorg.rs
  - packages/open-bitcoin-chainstate/src/lib.rs
  - packages/open-bitcoin-node/src/chainstate.rs
  - packages/open-bitcoin-node/src/chainstate/filter_index.rs
  - packages/open-bitcoin-node/src/chainstate/filter_reorg.rs
  - packages/open-bitcoin-node/src/chainstate/filter_reorg/preflight.rs
  - packages/open-bitcoin-node/src/chainstate/filter_reorg/tests.rs
  - packages/open-bitcoin-node/src/chainstate/filter_reorg/tests/faults.rs
  - packages/open-bitcoin-node/src/chainstate/fjall_store.rs
  - packages/open-bitcoin-node/src/chainstate/fjall_store/reorg.rs
  - packages/open-bitcoin-node/src/chainstate/fjall_store/tests.rs
  - packages/open-bitcoin-node/src/chainstate/fjall_store/tests/fixture.rs
  - packages/open-bitcoin-node/src/network/mempool_lifecycle.rs
  - packages/open-bitcoin-node/src/network/runtime_authority.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/catch_up.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/catch_up/tests.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/catch_up/tests/faults.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/reorg.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/coins.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/append.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/ownership.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/ownership/proofs.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/publication.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/reorg.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/reorg.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/reorg/faults.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/turn_inputs.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/turn_inputs/undo.rs
  - packages/open-bitcoin-node/src/sync/block_reconcile.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/catch_up.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/reorg.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/branches.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/failures.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/fixtures.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/measurements.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/protection.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/retention.rs
  - scripts/check-phase103-mempool-lifecycle.test.ts
  - scripts/check-phase157-index-catch-up.test.ts
  - scripts/check-phase157-index-catch-up.ts
  - scripts/check-phase157-index-catch-up/contracts.ts
  - scripts/check-phase158-validated-reorg.test.ts
  - scripts/check-phase158-validated-reorg.ts
  - scripts/verify.sh
findings:
  critical: 0
  warning: 0
  info: 0
  total: 0
resolved_findings: [WR-158-01, WR-158-02]
status: clean
---

# Phase 158 consolidated source review

Independent standard reviews initially covered 47 distinct Rust source/test files and 13 automation, parity, contributor and UAT files. The final twelve-path closure review adds seven distinct metadata/evidence files, for 67 unique reviewed paths. The reports below preserve the actual pre-fix findings; the two independent delta checks establish their resolution. This aggregate does not rewrite historical findings as though the initial reviews were clean.

## Review evidence

- [Core and storage review](158-REVIEW-STORAGE.md): 20 assigned files, one confirmed warning.
- [Runtime and fixtures review](158-REVIEW-RUNTIME.md): 27 assigned files, the same independently confirmed warning.
- [Automation and claims review](158-REVIEW-AUTOMATION.md): 12 assigned files, one confirmed checker warning.
- [WR-158-01 repair evidence](158-REVIEW-FIX.md) and [independent fix check](158-REVIEW-FIX-CHECK.md).
- [WR-158-02 independent fix check](158-REVIEW-AUTOMATION-FIX-CHECK.md); Plan 07 records the executed checker RED/GREEN receipts.

## Historical verifier harness delta

The default native run and focused historical Phase 103 suite reproduced two grouped-test 5,000 ms timeouts. The authorized one-file harness change splits the same eight requirement/symbol mutations into independent named fresh-fixture tests. The production checker, all expectations, the six other tests, helpers and default timeout are unchanged. Focused verification passed 14 tests and the same 16 assertions in 19.58 seconds, and the live checker passed. Independent [harness delta review](158-REVIEW-HARNESS-CHECK.md) confirms the exact scope and that all 59 previously reviewed fingerprints remain unchanged. The full native retry remains separately gated.

## Resolved findings

WR-158-01: the unconditional durable-height guard rejected a genuine retained A23 → B14 → A23 sequence before coins flush. The production regression failed with the exact displaced-endpoint error. The repair permits taller durable coins only through the existing exact authenticated displaced tuple and preserves the processed-frontier guard. The actual retained-source regression and three missing/wrong-height/wrong-hash binding controls passed; the affected Phase 158 suite passed 102 tests with zero failures and only the previously earned opt-in timing experiment ignored. Strict node Clippy passed. The independent delta review read both changed files in full and confirmed all other original storage hashes were unchanged.

WR-158-02: the sealed-field checker allowed pub(super) on every protected type. Isolated mutations demonstrated the false negatives. The repair permits only the existing exact BasicFilterAppendProof.identity pub(super) field and rejects every remaining public visibility token. All 449 checker tests passed; independent re-review rejected all 40 visibility mutations, preserved the exact required identity seam and confirmed only the two expected checker files changed.

## Simplification and limits

The explicit simplification pass retained the shared append implementation, existing publication/ownership primitives, exact displaced-fence predicate and existing Rust evidence lexer. No duplicate authority path, new dependency, source suppression or production visibility widening was needed. Reviews read actual test bodies and traced capability producers and consumers; lexical mutation checks remain supplementary to executed Rust behavior.

Full native verification, all declared security mitigation dispositions, formal goal/lifecycle verification and the strict commit hook remain separate final gates. The following hashes identify this reviewed snapshot; any subsequent source change requires an affected delta review. Root-owned planning/task/completion metadata and generated LOC freshness are checked separately.

## Final closure delta

[Independent final closure review](158-REVIEW-CLOSURE-CHECK.md) corroborates the successful native result, 25 closed mitigations, 21/21 formal truths and same-attempt lifecycle validity. Exactly five original claim artifacts changed to reflect earned CFIX-03 completion; the other 55 native-tested source/test/script/doc paths are byte-identical. All twelve final claim/tracking artifacts are clean, with five of nine phases, 33/33 created plans and ten of 22 requirements complete. Stale summary counts and the CLI's unintended archived-metric increment were corrected and independently rechecked. Phases 159–162 remain pending. Generated LOC remains separately checked freshness data.

## Reviewed source fingerprints

| File | SHA-256 |
| --- | --- |
| `.codex/tasks/todo.md` | `25f16abe94b53052683c1377427dc069c4014e2bdd1ef195f783718836faa25c` |
| `.planning/PROJECT.md` | `234efcb4b0e5cd889296b1fa6c86c55c0d61fa689912852733ce427767b203a5` |
| `.planning/REQUIREMENTS.md` | `19f0cbacbb85c9cd94e2962bd2926747a6c0110bc489dd9e58137a7bae1a8ecc` |
| `.planning/ROADMAP.md` | `80627d7345205e052513fb1e7bdc08e75b48132794155b1f9cdd7694c8b0fbac` |
| `.planning/STATE.md` | `f197a653f42ffea106ab6c740418b08800021e7ad28d461d1c3ca76c30755ca1` |
| `.planning/phases/158-validated-reorg-and-retained-branch-identity/158-07-SUMMARY.md` | `16352b50def17ee3d9bb7f4a060e9823be00685b81d25d2d9fd7f0dc97c21add` |
| `.planning/phases/158-validated-reorg-and-retained-branch-identity/158-NATIVE-EVIDENCE.md` | `0562662d673b4a71105c62a39952705970a556beb267ea649aa37cb2038408ee` |
| `.planning/phases/158-validated-reorg-and-retained-branch-identity/158-UAT.md` | `400b57813d74210865458d94c2c7b0b2903b058ac91f6bc8ba4360ce18abd599` |
| `README.md` | `3e6804c35bca8f52b6e28dd15ca2c8b910dfde0de9105a8ef75861e24e6d996a` |
| `docs/parity/index.json` | `3411caff99b127816e897ace1b6a538cd0499d894fafc16a1f19d1d87f483bb4` |
| `docs/parity/source-breadcrumbs.json` | `02c2b000bb46934ed4261185f4ff86551d16d4e634f1079df46bdf077eefb67d` |
| `docs/parity/v2-5-validated-reorg.md` | `80e4ea951a4794fe60a33184fe5922926d4c4bc1356c3f725a79250729c912b5` |
| `packages/README.md` | `3cbbdcb8e07c22c53c122cce0f808c44d919e3be4ae83fe03067c959293af023` |
| `packages/open-bitcoin-chainstate/src/engine.rs` | `ce642b9f3c4f266c09712550f66acc53d8038f11da1463cef65c79effa7b69a7` |
| `packages/open-bitcoin-chainstate/src/engine/stage.rs` | `cc04325a42f7f45ce8d68f5a39300d2af89398842232e3fec7586bcf529c18e8` |
| `packages/open-bitcoin-chainstate/src/engine/tests/apply_non_coinbase_transaction_returns_fee_and_records_undo/staged_connect_and_reorg.rs` | `b918d51b6c6a95857d4e1897ca6828a73529a0639bf7b163e2963fd968fc7518` |
| `packages/open-bitcoin-chainstate/src/filter_index/catch_up.rs` | `332b402ce454f35cc88eba2e0d9b11fa294a53b1d41a9a9f656ba9ea69cef503` |
| `packages/open-bitcoin-chainstate/src/filter_index/catch_up/reorg.rs` | `a37403ada7a4ffcdd0bceee5a4f3f388404211f924c97b780c52ad216da26abf` |
| `packages/open-bitcoin-chainstate/src/filter_index/catch_up/tests.rs` | `8f9e7102c97e66474bcb48e647065c34b77431857167c7921e63195422d96492` |
| `packages/open-bitcoin-chainstate/src/filter_index/catch_up/tests/reorg.rs` | `22d74c633756ba3d91cadaf2f878b7f2fa33a6bf44fde21c4c201de017e8fe73` |
| `packages/open-bitcoin-chainstate/src/lib.rs` | `5f8153f6b948ae2871ecd066ff091f8dd444b3a228eed7c5a00c6f3c0fc5738a` |
| `packages/open-bitcoin-node/src/chainstate.rs` | `3e04ceb45cfc3affdaae01ebd4ab791cd7b82110738e26e910b5242b14eb19f1` |
| `packages/open-bitcoin-node/src/chainstate/filter_index.rs` | `9902f1021730914bfffa4b1b99764dcf8350ebf20d33b3ed777808c98a86ec0d` |
| `packages/open-bitcoin-node/src/chainstate/filter_reorg.rs` | `cbde61693ef860a2210b4a7261b74f095a9c60aae3771cf0775c0f9acd1134c5` |
| `packages/open-bitcoin-node/src/chainstate/filter_reorg/preflight.rs` | `b7adbc9f1ec32d45e36429e2702fde89409050268b22f084bf2f5b5ede33d377` |
| `packages/open-bitcoin-node/src/chainstate/filter_reorg/tests.rs` | `43fd99edb144250f6c10b59781993a4a7189bfc424d92d4b25d540012961b32a` |
| `packages/open-bitcoin-node/src/chainstate/filter_reorg/tests/faults.rs` | `b29db3b5927f81879c2ef90552775ad6ed65f9926d486b339dcc6dee6e5aa357` |
| `packages/open-bitcoin-node/src/chainstate/fjall_store.rs` | `322b973e2f687d6ff4c7b9d06d46682a4723f499115da9d53d4f412d71d2353b` |
| `packages/open-bitcoin-node/src/chainstate/fjall_store/reorg.rs` | `41755114f92096de344dd0e2f987e324d713d1d9a4f3195700c192d1a644268a` |
| `packages/open-bitcoin-node/src/chainstate/fjall_store/tests.rs` | `951ec9688e42561d353dd724540a924dc28d63a0db9ac7d6da26b50d905e1fdd` |
| `packages/open-bitcoin-node/src/chainstate/fjall_store/tests/fixture.rs` | `9f1115b019ae6846d251b71f2543c15e82721db1db782e555d6eca5275e9eecb` |
| `packages/open-bitcoin-node/src/network/mempool_lifecycle.rs` | `d82c02194ec2bd0de54d16e4bb006f43a498c172a3d2a1c15ff79377f8fec16e` |
| `packages/open-bitcoin-node/src/network/runtime_authority.rs` | `051cc7f80d4b937f17086197571abbb61f65f936f184e7c82dbea72296da934c` |
| `packages/open-bitcoin-node/src/network/runtime_authority/filter_index.rs` | `f13b4e896dd803cf640dbdbafba13b86bb5b8c12921c7662563d90b193010f0b` |
| `packages/open-bitcoin-node/src/network/runtime_authority/filter_index/catch_up.rs` | `935a82ecc7838fe0a109180a4a2961dc492f787a06b36537c2d7274bbdafd2c4` |
| `packages/open-bitcoin-node/src/network/runtime_authority/filter_index/catch_up/tests.rs` | `cc6f28cdea3a16517fac0bffa0b7c267ea2765db3163cf80fbd9501c068e4838` |
| `packages/open-bitcoin-node/src/network/runtime_authority/filter_index/catch_up/tests/faults.rs` | `4b065e69f0aeff73a974f6e4c93d1d5a932aeb3a65d74c34b8ec2838f0637497` |
| `packages/open-bitcoin-node/src/network/runtime_authority/filter_index/reorg.rs` | `89104de96b6cca93e3b5321027786ea9570dd610ecf5a1d1e40de51c10f01eb9` |
| `packages/open-bitcoin-node/src/storage/fjall_store/coins.rs` | `a3bad66bbf4d848fd9c9ac0d01a4c556586cb34ef66c3804506f8187cd906ee1` |
| `packages/open-bitcoin-node/src/storage/fjall_store/filters.rs` | `91a61729172f33581dc6b7ff652effc96faf80e4d367c24e4241f5ffcc5ed442` |
| `packages/open-bitcoin-node/src/storage/fjall_store/filters/append.rs` | `83bbaa80175bd9cd05b7a4e9b638faa7e0be81e89723b7793b2041b17883c69e` |
| `packages/open-bitcoin-node/src/storage/fjall_store/filters/ownership.rs` | `379442b8623ad8f517de892ceace6603c01267f9c8460fe0b22b25420b5329d9` |
| `packages/open-bitcoin-node/src/storage/fjall_store/filters/ownership/proofs.rs` | `1298113df10505036770b40ed560853ed7d94513801408e4390b0384fc7a8542` |
| `packages/open-bitcoin-node/src/storage/fjall_store/filters/publication.rs` | `1c98f0b8ad4c7738befecff4dbdb2b76b1ca0f732a606ead5bc70fc51da72f41` |
| `packages/open-bitcoin-node/src/storage/fjall_store/filters/reorg.rs` | `f85e0da039b66b24669b3a085cca2d5c70889d5d28ef5b0098439f37b581e715` |
| `packages/open-bitcoin-node/src/storage/fjall_store/filters/tests.rs` | `6ed895afa1d1ac0126019ef6dca79d97a2bc81b53031cd8f6010c2fb84813d07` |
| `packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/reorg.rs` | `625d2a2932a2a7fe470fcde30f63fe8010355ad661444111d2fe854a10d40b41` |
| `packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/reorg/faults.rs` | `f767df6bd156930b1c625c49916ae6e9f8e014ff99ab514a0f069fe1e2fa43b7` |
| `packages/open-bitcoin-node/src/storage/fjall_store/filters/turn_inputs.rs` | `81c363d712cf6aff21b81f5cbc691acdfc520fd17368c02a2fd4d1e3be45db79` |
| `packages/open-bitcoin-node/src/storage/fjall_store/filters/turn_inputs/undo.rs` | `923371e2e2207190afdd1fbab69e6ae9970d556ea44a4ca3d10b3d66da6e15e7` |
| `packages/open-bitcoin-node/src/sync/block_reconcile.rs` | `1c2088e1d39572e91d28247993f3bf73f234beb8200895969daf597ae1d0319d` |
| `packages/open-bitcoin-node/src/sync/tests/filter_index.rs` | `0a0fd9b0a775c55b8c598f2abc410fcef6f255c6afd431c335405f0b95a18b7d` |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/catch_up.rs` | `5c2a5ba1c93ded830ed969e04ee7682dd3840a38c31652c6afb2a5fc4dd66bc8` |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg.rs` | `597b2276dc5b5719c67fa555fcb5578c2bc49f0ab954e83852c7378127330c49` |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/branches.rs` | `a9bcf1d5e3de9e719f29cbff8e6fac29511dc3571c1326b52cd880497e23654e` |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/failures.rs` | `f450a4741c0da057e7a2e0f941eb8cae2e26515ee4ce49720ed8366e86b8b100` |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/fixtures.rs` | `5dcec3e9bbf7380e82258bd9e1e05e0fc9d60d469104fd33ee11b3803e02b0b0` |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/measurements.rs` | `7a8e1d798bc974fc0123ab2a8355453149e081ad404a5d4f772f1eacfb2c6a74` |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/protection.rs` | `2cdc131e3c7586d523e36924fa8c095e4f95b3da5f6c497cfb5bb5e4fb87810f` |
| `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/retention.rs` | `8a3f4e94741796c8071277c5a2c732204631ff828a0a1d5f2e811baa35e91eae` |
| `scripts/check-phase103-mempool-lifecycle.test.ts` | `05991df40afc4d2d515da813443e343c27de0289ceb38d727796aa71465dd458` |
| `scripts/check-phase157-index-catch-up.test.ts` | `53d897f7720f9a2989399a641956058f3937fa1507f39e02e8e605c182d60e6c` |
| `scripts/check-phase157-index-catch-up.ts` | `0b5890ac87f4d27e0fbeb5f0e28c36119045ade3db92853ca5deacaaca9d7ad0` |
| `scripts/check-phase157-index-catch-up/contracts.ts` | `eeaa568a615924dacdc253da4551203e7c31ebe25f40b0ec5d63936f102256f1` |
| `scripts/check-phase158-validated-reorg.test.ts` | `05d98dcd2abf30dad2df49a2f5ab058fbf60d070d76ea38e88b6c65f033ef109` |
| `scripts/check-phase158-validated-reorg.ts` | `e9b8ae3ddc7a6ccaffa67a0b5bf7b7614eae7938d3cc1829de4cd2ca9aaa64e9` |
| `scripts/verify.sh` | `d9c6fbff98535c283c3adc01726ea18f58e159c541e01bc27a238414520af3fc` |
