---
phase: 164-canonical-chain-identity-and-compatibility-gates
reviewed: 2026-10-14T11:08:45.170266Z
depth: standard
status: clean
files_reviewed: 146
findings:
  critical: 0
  warning: 0
  info: 0
  total: 0
source_snapshot_sha256: d9738977df3ea3c28f802db21fadcf6707b6717b7824118ab132ae6d6340274f
native_verification_source_snapshot_sha256: da4b07c93d86bdc74aa93efb2ff62ebfa290c8e790cca66bd08656df6c0340c0
closure_changes: documentation-and-eof-formatting-only
source_sha256:
  packages/open-bitcoin-consensus/src/lib.rs: 3eb75ad506bb6eef7aa0001dd2ec2ba9dea5a88722aaff0ab7e0c038fe6a3cfe
  packages/open-bitcoin-node/src/network.rs: 99ad30449677ee40a9569642bf42d52a743176061f11b41f1bdcaaa0afed116f
  packages/open-bitcoin-node/src/network/action_translation.rs: 4bb07a11761b5c84ef6903c8643a82d775988f1df0ed0d730a3b202bedeb1e62
  packages/open-bitcoin-node/src/network/header_sync.rs: 6b546e406c8ff851c2c089da4445e5f999230e7695cf80170ff6bd64df386731
  packages/open-bitcoin-node/src/network/mempool_lifecycle.rs: e71ff25d0facf5c109a87e9f04de6513e5a1a41d933203e7183332e0d4df2496
  packages/open-bitcoin-node/src/network/peer_network_clone.rs: 5073bd88917a73cb17ea66a6b2584b308d02c097a4aa658e174d2bb6b1fb1dbf
  packages/open-bitcoin-node/src/network/relay_serving.rs: 3f53591281799ba040289f9b20bac53f656861cb0352464d113f32d7c8c8916a
  packages/open-bitcoin-node/src/network/runtime_authority.rs: 3923b0e316fbd66c049d236b0843558950ebdffc968901d3622a996dcc8561e5
  packages/open-bitcoin-node/src/network/runtime_authority/tests.rs: 47ecf6975c17d50becd69a3c2d5f1d5cd55097027497aed5a77f771198ab16cb
  packages/open-bitcoin-node/src/network/validation_history.rs: 65681c65fd7e84b3acd5ebd6e8076254ecca9586cd0f827a6d1f59eab34c3b49
  packages/open-bitcoin-node/src/storage.rs: 9794e1aa3aae44819ccfd2685d992caf4c82d8d6145df58c162aedce1ec26afa
  packages/open-bitcoin-node/src/storage/fjall_store.rs: a5910d96a5fcfc136c89cb16cb57ab2c98b91d56dc0ee14524fc80f2a094e51a
  packages/open-bitcoin-node/src/sync/open_runtime.rs: 80207b6e70bdd9f1f8ccbfb2adcdd6787f560cca64e33455a0faa704efc0dfe1
  packages/open-bitcoin-consensus/src/chain_spec.rs: 80e301612d039afa72125c16a895f248a155305410348acce09376325634302e
  packages/open-bitcoin-consensus/src/chain_spec/genesis.rs: 6203adbae928028fbad72732374afd422a371180021b65045afa4273dfc2f1e9
  packages/open-bitcoin-consensus/src/chain_spec/tests.rs: 262d8d89930115c3e2ed5cd60e654225edaf4713880f63f2352a39d13e9f2bdb
  packages/open-bitcoin-node/src/network/canonical_admission.rs: 07f69af9a17270538194859fb73089088db28f96b8a5bdc1d8f4e1a8a2374b83
  packages/open-bitcoin-node/src/network/canonical_admission/tests.rs: 4977020a4421500c9c629d4787abdd421e0bce28f777f1c56a9cc2b18a461e00
  packages/open-bitcoin-node/src/network/canonical_admission/tests/fixtures.rs: 3b0d4a20790a7d299fa7a3723e618cb65dc0e6197a7da4a4b7f0cc2b8928d69a
  packages/open-bitcoin-node/src/storage/chain_compatibility.rs: a40d4663343c9937329873461ae0721a8acd758ffc0468b0a18c2fdea539598a
  packages/open-bitcoin-node/src/storage/chain_compatibility/tests.rs: a127cd9cd6cecdadda5e0d998fec7ff6948615d1dbe4b35c06f82f8b5826f9cf
  packages/open-bitcoin-bench/src/cases/rpc_cli.rs: 174e4f86174f22866191060f5a31b853d7952f9ec0df9b0ef25ca7d9c3e3d60e
  packages/open-bitcoin-cli/src/operator/runtime.rs: 3ad11d0d3b9bfbfd7f739e63182119c777d26837ddfaab2515caacea98f28868
  packages/open-bitcoin-cli/src/operator/runtime/support.rs: 34222b50b913e30247ce0d24095957f0e77d18e2584a1f630439f5d6ca1be04e
  packages/open-bitcoin-cli/src/operator/soak/runtime.rs: 5cf958492b573dc9cf4bf15d005311840c23586ce0f118dee2ae963ba2c9c702
  packages/open-bitcoin-cli/src/operator/soak/tests/recovery_resume_bounds.rs: 93bd3b57464ed6b43a43cead277f007a8de99c7393bea4f5384a7420620694f0
  packages/open-bitcoin-cli/src/operator/wallet.rs: b4d29ed57e7b54a98b97605d24f5408849442c2531e3d520ba4511310dbcae49
  packages/open-bitcoin-cli/src/operator/wallet/tests.rs: 9e15eb95c7a7582454d1c043c1a4a8b661bcb71674bc15472aa2501d3cb51239
  packages/open-bitcoin-cli/tests/operator_binary/compatibility_wallet.rs: 59340236ee3091de69ca9093aeb16a0095c148f4c73cb8640dd47594d39e3e1c
  packages/open-bitcoin-cli/tests/operator_binary/status_sync_soak.rs: 01e02c3ebf19aa0d2d2bcaac17c95a0060b3953499dab4b13e14be72462bda50
  packages/open-bitcoin-cli/tests/operator_binary/support_sync_soak.rs: 5ce0e414f57476d83bc27a09d5a42cead3c6ac3b0912bc6fe661e7ebac97ac05
  packages/open-bitcoin-cli/tests/operator_flows/basic_index.rs: 1b95457de881b2b68b880b4b8ca4e19e38ca05a3f87b231e1437f93622223f63
  packages/open-bitcoin-cli/tests/operator_flows/basic_index/captured.rs: a79856c1007567d6431a9c14f076d130d66330825aeec4532c20673575b7aca2
  packages/open-bitcoin-cli/tests/operator_flows/rpc_harness.rs: 88e5ac86f36dd3fc059ef46146d3d889ee7bbb2807fb44624f0d04c6ff84877e
  packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs: adaa5772a3fccce0d92b2a23a455e7fc567f1a8416fb2f30c942200111743152
  packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush.rs: c927b7c71b99190cbf3196a0c97ceff51dd86ae45514e53bd674051a928d3165
  packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush/tests/filter_index.rs: 2a96ea915292bce360e3a917ac22dc6359a05ee19dff7b1e75eadb1ad11fd905
  packages/open-bitcoin-rpc/src/bin/open_bitcoind/inbound_metrics.rs: a02c299d8ce54bec707ab9ad651a54bcf94d3d142ae3938f1ed0cf7d71f6dd59
  packages/open-bitcoin-rpc/src/bin/open_bitcoind/inbound_startup.rs: 2021fa7f0c8851c67335c77254d1aadea3edf98d7e63930b4c2b6a6ab1b1d0f3
  packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests.rs: 2ed1e3b1804c24fbee16af54542a6ba40cd2a5237229d8372bfab8c515e67c69
  packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/automatic_prune.rs: 304b9ae10f6829ba0c88afb542341cf8902446555ea6fc07af55072e17cb2d12
  packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/daemon_sync.rs: a6b6ccc7a3520f54da3495b959f9b7963d6121de0207b53f8a49a6812ec809fc
  packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index.rs: cb99a5d7ff54e05bdb8affccf6364c01c2183aa7dacfe77ad86105a01b475e27
  packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/fixtures.rs: c5f89846a95989bf32a235f40e27a2edfcc511b1ff11a166b7e87bb6e107aa2b
  packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/peer_serving.rs: 2a0896c2a034469758154de22e7dc53a942aa22cef4cb21c591e23c64cdc12e5
  packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/fixtures.rs: 933da5cfb57612c9e93a231967a17da3cc0163e1badcd5f7d44debde78868b18
  packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/inbound_runtime.rs: 22558f5497cc4c7919d0b4b8864519589fc28e547357a31c8718c791f4fce683
  packages/open-bitcoin-rpc/src/config.rs: 18fca8b57111efc693b74073bf9af7e5bf97c79b7c071cf45fdc9ce0ae0e8cec
  packages/open-bitcoin-rpc/src/config/tests/prune_config.rs: e327b2f56e96b71cd7d177a5f2eb4203d92358c544e04e0cb08de3572afd2871
  packages/open-bitcoin-rpc/src/context.rs: b5bb4dad46ad7d5d2d92c795a3cf8c68f783695ba01e276c128183e645756f91
  packages/open-bitcoin-rpc/src/context/mempool_recovery.rs: c583939a2ef9639cd0f494ef4bae05eb489a2d437e95af0d29596d3552b604e8
  packages/open-bitcoin-rpc/src/context/network.rs: df9fde08f21ecaea6d403424a64a35170eaff0a9974d99f1357a8475f27462d0
  packages/open-bitcoin-rpc/src/context/tests.rs: 2c7915f8e95d39f992c706afba9e120fe79ae5233d9a0c4011c94ca8419553bd
  packages/open-bitcoin-rpc/src/context/tests/construction.rs: a828fbb79ae1eea2385108b4ad9045a777aedd15a9955cf3edeb71460ffc3ad6
  packages/open-bitcoin-rpc/src/context/tests/rescan_eligibility.rs: 2fd9c844ef75121d5d64ab7485c2a5b3934fdce1bcd3d9605e44652ff982ddb2
  packages/open-bitcoin-rpc/src/context/tests/rescan_eligibility/fixtures.rs: 4eae28c6444da6fa4a943064f2157c9e508b525331c5b0731cdf30daff89a948
  packages/open-bitcoin-rpc/src/context/wallet_state.rs: 8b641e69c5a1bbb70a58590f65b8f96210d778919122de0932c41153934788ba
  packages/open-bitcoin-rpc/src/dispatch/filter_index.rs: 08287da62a225b8b0dbccb2a7ab23c883ac6c4afc13241733de76935f2064b3c
  packages/open-bitcoin-rpc/src/dispatch/filter_index/tests.rs: 6788329f9606e72f5d18e6c9012b0128cc8c98fe1b4f5223d0a7c3081acf66ed
  packages/open-bitcoin-rpc/src/dispatch/prune/tests.rs: f048c4089311fd08dd8976fedb5c7a70c14dca34724ee67b12082667da6a7ea5
  packages/open-bitcoin-rpc/src/dispatch/prune/tests/ownership.rs: ddb88d3f8b42c1d4ff1ded1ba2b64e400afbb05851d5ecbdcb4cc58ca67b2982
  packages/open-bitcoin-rpc/src/dispatch/tests/chain_fixtures.rs: be0d8f26818adc787a35625824c63682f59c883837dee826700438162b381e48
  packages/open-bitcoin-rpc/src/dispatch/tests/network_fixtures.rs: 7e413392913f01bb94f533af52a889414c0cfd5c4ed8fd17dceff0c4fd029e30
  packages/open-bitcoin-rpc/src/dispatch/tests/permissions.rs: 531e449e628ed6d6e253aa799496bb5fb1167046aa1fc3924cade96bdc8e00e9
  packages/open-bitcoin-rpc/src/dispatch/tests/storage_fixtures.rs: 1e6a23fec63a9483c918e3643e97a9a7da707aee692bf6f428dc33500986509a
  packages/open-bitcoin-rpc/src/dispatch/tests/sync_status.rs: ae712454ea90a7e34cf35fc943018f512859430ef52bb4da80e4d8cf3407fecf
  packages/open-bitcoin-rpc/src/http/filter_index.rs: c5e12fb042f15981b887dd0b5b5040d4853697558425bc3b90851e37acde3b27
  packages/open-bitcoin-rpc/src/http/tests.rs: 328ea4bf642998133b40bbd74c6bddc6df1c8f5dbaab4e785899e5454649dccb
  packages/open-bitcoin-rpc/src/http/tests/filter_index.rs: ac652090e20f6342c254cdc634e04fb10673268b00feac58472f0b4cfa996489
  packages/open-bitcoin-rpc/src/inbound_listener/tests.rs: 85efa76208b1a98941fc7bf104784b9aaf6132e8a7e24cffc5555fed83360912
  packages/open-bitcoin-rpc/src/inbound_listener/tests/announcement_successful_prefix.rs: 5812930dbe8324c5a057ca941d1a314d6b0453ce00225b1c15034e458314e0da
  packages/open-bitcoin-rpc/src/inbound_listener/tests/block_serving.rs: 756a409773da87e5479275b550fe9b9ea559774dcefab44dd1983f2dfb581221
  packages/open-bitcoin-rpc/src/inbound_listener/tests/envelope_and_resource.rs: c5febdc1cceb813b7688516206f4ca2ae9e89546880e8babaef0e51f9a52611e
  packages/open-bitcoin-rpc/src/inbound_listener/tests/listener_fixtures.rs: 761ce8c01424dc77a5e9708db70440250a7f778bc94fc2b11acecb2acc090add
  packages/open-bitcoin-rpc/src/inbound_listener/tests/peer_filters/phase162.rs: abe2651ff16c3d872695e8bde1222de63c2b5a28c519ffa9da299ac9166ab8b6
  packages/open-bitcoin-rpc/src/inbound_listener/tests/preflight_and_advertisement.rs: 458a2ce54b0afc9fa854f471ddb1d6ca6bf2c7a50f2f71c612e810ea24551499
  packages/open-bitcoin-rpc/src/inbound_listener/tests/reconnect_policy.rs: b4a69adeb5943f287fb0240f82917a538152128caad5db5f4e6dc4ee937509e8
  packages/open-bitcoin-rpc/tests/black_box_parity/open_bitcoin_rpc.rs: 85e511be49aebd7aa410fb1fab841e40a8930f8dc4d8b06e7f49436b74ba88f5
  packages/open-bitcoin-rpc/tests/black_box_parity/phase127_composition.rs: 4fc0a4dfef6b352a527b2e4dec2d65722241da30d2b70a16c86d91b83a58cbf8
  packages/open-bitcoin-cli/src/operator/runtime/chain_compatibility.rs: 5c3fd9c11dd2e4dd8f3c063c2f424a2ba0aabc26d456d2e7c8e3c65eb30f8004
  packages/open-bitcoin-cli/src/operator/runtime/chain_compatibility/tests.rs: ea4b8eb383d26c747f53597d1b46834a771529afbaa09c4ce544da24e6bae31f
  packages/open-bitcoin-cli/src/operator/soak/runtime/chain_compatibility.rs: 6a03585ccf8e64a71b93f0a7dd23fc86e6a9be3f0a17c15706200942d328c4ad
  packages/open-bitcoin-cli/src/operator/wallet/name_resolution.rs: 6fbad107498412e86e595bda2d53fc90dd108ad45e3b663124e9eccf3e7b8500
  packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/canonical_chain.rs: e8154aa11974399fdb80ed08c77bf20bf9d1b3e4b7740a77dbbbe14b55eb4ac6
  packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/fixture_runtime.rs: 610bf1d251458e361115ac3ea96b1cef3dcb8a3d270fafc6d8b656372943129e
  packages/open-bitcoin-rpc/src/config/canonical_chain.rs: 4e3f77b42c594156a87918190cfaa5ab441c89d08747692b421496a46b5a95aa
  packages/open-bitcoin-rpc/src/context/canonical_chain.rs: 6e5aeca818133c28f90e739919edf02fc42f1bc4bad094cede9c12ebb643fa5f
  packages/open-bitcoin-rpc/src/context/canonical_chain/tests.rs: 4a29bc4bcec59e8dd26e55992d47d2f08b5bc7ab87ab2ac6dd6818aeb4b078bb
  packages/open-bitcoin-rpc/src/context/canonical_chain/tests/fixtures.rs: 427e52e21816a6345a7cb8fd53938e0b790ba83adef9e03fb675ea1bd8c4243f
  README.md: f2f851749937089b8e2217dac95be0b0f96eb88a7b46b48741a8833658caa1b7
  docs/parity/index.json: cf4542f441ac00716f49390074d4edfd6763669b5630ecc2e65155fb5ba76536
  docs/parity/source-breadcrumbs.json: cc62a8ebe914e388ea65cc873008a1b6ee2558bcab4a87b912f0c0f763db8ad8
  docs/parity/support-matrix.md: 93cb19cb9ee474f78ee9472934846cbb0b7d00c349b5603e5d4703394d7234bf
  scripts/check-phase123-runtime-timing-evidence-integrity.test.ts: 7cac9ea665fb7497ac45d98c5f896aa6b44ec6050389dcae6e6acece42d66254
  scripts/check-phase123-runtime-timing-evidence-integrity/checks.ts: 0e244a82de3f37253f27daae236f1977b83f78b2c8c531d77521d9e27a59c2ea
  scripts/check-phase123-runtime-timing-evidence-integrity/constants.ts: 8449503f442af7cd7b190062a42ed7d85ae659f6d9d5bd0800c9834add19e6b2
  scripts/check-phase158-validated-reorg.test.ts: bebdd0c7f2cc6e787f34502bb82f6ae6fd27c7a5ce5ac9bdd14be73c054193a3
  scripts/check-phase158-validated-reorg.ts: 385b6c6d62067ba13e8e4d29d0ea1005252614830430f800b0824ec7ba954fdd
  scripts/check-phase162-integrated-filter-proof.test.ts: 784e29775202503f892ee7b005a5595f77ac8d0f8bdb42ca19818c138a86f6d5
  scripts/check-phase162-integrated-filter-proof.ts: caf9147ecd5b4ac189486bca0d2e81298ff0cfc0ffc218769a5f690a20ca8350
  scripts/check-phase162-integrated-filter-proof/contracts.ts: 57c5de29c641a4b2e947cd9e0b25b270cb07dfac018320d7f1961580b2e6b10b
  scripts/check-phase163-provenance.ts: e42bd6a3f1660d46ac35c2bb98fe4df44766ac84ee80d7ffae2a766c22dbaa03
  scripts/check-phase163-provenance/fixtures.ts: 24bf75c09f40d0008bf528a7bd77fa9edf409e15ae967005cae02f6083bb25ae
  scripts/check-phase163-provenance/native.test.ts: 5297898680a15de12f79b7e6c9c8a814fdd0276522bfd071b96cd7b3a6f7ac16
  scripts/verify.sh: 2925344e06e3604fbba1821e49a00b0f2f292448eae205107882d0ed65aacc43
  docs/parity/v2-6-canonical-chain-identity.md: d527c791550bc4873545bc6b123763a18431a62ce2cfaa85def0a27eaa0bf1c9
  packages/testdata/phase164/historical-tooling/manifest.json: 9783fa03fdd2df1395445b7752e6c31d84985c351ef44de516656af7f82fc7c0
  packages/testdata/phase164/historical-tooling/phase158-checker.txt: 1df027aca2deb9827f135d9a558ef2377d7a2ebb18b830ba76f41698376834de
  packages/testdata/phase164/historical-tooling/phase158-tests.txt: efa71956ef2e4916a94790f9f9855a418e8f6d4b84c953db126654e89fbd85dc
  packages/testdata/phase164/historical-tooling/phase162-checker.txt: 61c1919cc35f388b8d462ed28d8042cf33d5e5752afcc5d43fea79817d46ad97
  packages/testdata/phase164/historical-tooling/phase162-contracts.txt: b5e5df1f4dc8705cdbe1ca85f9d9049a6e0188653311e00f2cacabb9f88daa00
  packages/testdata/phase164/historical-tooling/phase162-tests.txt: 01e84e45886a44d020ffabfebf32d8c898affc3afe1e35613478d38bd870cbc5
  packages/testdata/phase164/historical-tooling/phase163-fixtures.txt: 40a76924ac64135f08f3482c50f8aafd1b5d98fa6d4dfd7052b87e0b6a492615
  packages/testdata/phase164/historical-tooling/phase163-native-tests.txt: f3750b40ca00a3e2faa38df1e0af24536ba152bcd7141e686d67d1c1a0a5bae9
  packages/testdata/phase164/historical-tooling/phase163-provenance.txt: cf22edda320107ce0dfda9a8692e8c29aaa436ad6ca1f82c48f66b1d778c67d7
  packages/testdata/phase164/historical-tooling/verify.txt: cd051fd967934b85a7489f75ad8793cc4880ef918ced776b179a8ff7fb3165bf
  scripts/check-phase164-canonical-identity.test.ts: d071cab6ea1d5c603b7d7a5c8b235878f3c0d00e47eee5820d57f31408441ade
  scripts/check-phase164-canonical-identity.ts: b82b62af8334d1262a2c5bcbc3fec646827efd22931b166fbf151488fedb176b
  scripts/check-phase164-canonical-identity/claims.ts: d6747a44772111b06a8cc0561e42d742f7efb0130ec06e4590289735ce47ad30
  scripts/check-phase164-canonical-identity/contracts.ts: 0a3f44db805a87f4aa53e170aebdc2b0d57930e101719461c46406087a77fda9
  scripts/check-phase164-canonical-identity/fixture-source.ts: 24224f0dea4e405d3a148a9fe148255884bc7c575092ad8b0c67507d157dbff2
  scripts/check-phase164-canonical-identity/historical-tooling.ts: 8922542b5721d2cc966312bbee416cf1418ea0151b1eecec01cb34dbc06fa85f
  scripts/check-phase164-canonical-identity/historical.test.ts: f03bdd98d4b69a7a0be71b278685e497b4fee5dc5f02e2b492ab1c2aee85ecd4
  scripts/check-phase164-canonical-identity/regions.ts: 4ad5ec9321033f01635079df96f90726e15f5c955d900a803a39a14ed288437c
  scripts/check-phase75-soak-runner.ts: 364b244e6e11326a9ec0724d4e687a1831f2099453d0af26cc0bb16fa2fd4d14
  scripts/check-phase75-soak-runner.test.ts: ef4ce1a7fe418378b029dfed7fa4ae68145932de91355e0bb31b249df660745d
  scripts/check-phase107-runtime-relay-activation-download-eligibility.test.ts: 2961c643e2f784fa18b8c97d9cd1cc36f78f7fef8ac4eb35a81db44116ce0168
  scripts/check-phase107-runtime-relay-activation-download-eligibility/checks.ts: a4c39733c686d75b3902c41bd5f9f3eb86855ba22a8f32b7618b756597947098
  scripts/check-phase107-runtime-relay-activation-download-eligibility/constants.ts: c1460fe29db0568339ba39a9bb9f9859c8584a2d090db2de087b5251e3d27e49
  scripts/check-phase127-authoritative-network-state-unification.ts: 7ac427147c31edd6099ef493679e4fc660a58c8ebad316a7dd2da22eafd14ee5
  scripts/check-phase135-snapshot-recovery.test.ts: 1ddba9516105b047df48028044f41518dd065e723d4371dd9a5fa05dba8c4eb2
  scripts/check-phase135-snapshot-recovery.ts: b52d6036e8a9da3aa162378ba1ba79240133ef6e04c94b95c26305425a97b8c4
  scripts/check-phase135-snapshot-recovery/source.ts: 6c3d16835d5a31c435fe9c557b477be9119e02deacc2c53408dabe4746457ad7
  scripts/check-phase157-index-catch-up.test.ts: b01ae7c68ea6e0c39e802f6550d6d34b172b22d54166a0255bf120e08f526924
  scripts/check-phase157-index-catch-up.ts: 2658f8af4d5d3d29634064a2c9398ce21b9600aeb988f528e95d0471c95398f7
  scripts/check-phase157-index-catch-up/contracts.ts: 826174bd0362b210dd3c9dd00a07a3cf4b22487277f0d2c4d1a26a5ae2eecde0
  scripts/check-phase97-inbound-metrics.test.ts: 6c0586efa85ad6518eb81de109b07e9c6c4a496fb6e1ed7005ad75bb7024b372
  scripts/check-phase97-inbound-metrics.ts: 3c0df84ed39dfe64a3ee24daf3d22bedace47082d94188eb8d9afa9bf80c915c
  scripts/check-phase159-filter-rpcs.test.ts: a4762b5de5ca3c79d1ee2a77d2e48ee77cd29424c6c6a1b44dae1c15944dd59d
  scripts/check-phase160-peer-filter-serving.test.ts: 7994fb29f277138305152f0c46ce7a99c96ab1a740ca4dd6876bb0203eb591bf
  scripts/check-phase161-operator-index-evidence.test.ts: 114dd2bb65d95ad3efcf51273e7d8198b46f31c8ad75600435ccb52988280e8a
  packages/testdata/phase164/historical-tooling/phase159-tests.txt: 8c3591fcd8f4d1e523d77fbd85e4c0d9d50a27e14925ed8876879d1c380451b3
  packages/testdata/phase164/historical-tooling/phase160-tests.txt: 38b3fd04eb9bbafb1ca2d3b75eb5880ec1f75f147f71836ccb1840680edfbee8
  packages/testdata/phase164/historical-tooling/phase161-tests.txt: 05a59352ca6c1641690c94ece985644fd5ae1f72d8cb32cfdf455bdaf85e61dc
  packages/open-bitcoin-node/src/storage/chain_compatibility/tests/preservation.rs: 3d004f91bd72a0f3902588a36fca0438b04b5ac034bc9e227893664496835630
  scripts/panic-sites.allowlist: 27f62337fd9b4ba340d4170f1860f417b5c5b8307eab34af1c80b3c9cb5c5233
---

# Phase 164 Aggregate Code Review

Three independent scoped reviews cover all 146 changed implementation, test, automation, parity and historical-snapshot paths. No unresolved findings remain. The transient-authority comment was corrected without implementation changes; exact literal-byte pins were strengthened before final review.

- [Core/node review](164-REVIEW-CORE.md): 22 paths, clean.
- [RPC/CLI/fixture review](164-REVIEW-SHELL.md): 68 paths, clean.
- [Automation/parity review](164-REVIEW-AUTOMATION.md): 56 paths, clean, including the Phase 75 repair and twelve current constructor/ownership contracts exposed by the subsequent normal-checker sweep.

All nine historical snapshots and 123 historical artifacts remain byte-identical to the fixed initial revision. Current and historical tooling digests remain distinct; this review generates no ordinary canonical qualification receipt.

Scoped evidence includes 4,774 checker tests, current Rust behavior regressions and 30 closed declared security controls. The first full native attempt stopped after 2.979 seconds at the obsolete Phase 75 anchor; its original failure remains in /tmp/open-bitcoin-phase164-native.log. The repair normal check and 21 mutation tests passed; adjacent Phase 76 and 80 checks passed. Renewed full native acceptance, goal verification, requirement activation and normal-hook git finalization remain pending at this capture.

## Renewed Native Repair Binding

The second full native attempt stopped after 8.055 seconds at the stale Phase 97 constructor contract. Both original failure logs remain preserved. All 82 registered normal checks and 822 affected mutation tests passed after the narrow twelve-path repair; the resolved Phase 157 comment/attribute-decoy regressions remain documented in the automation review. All 139 current file fingerprints match the clean scoped reviews. The third full native run is executing in session 29227, with `/tmp/open-bitcoin-phase164-native-third.log`; no passing native acceptance is inferred until its actual exit zero.

## Native Attempt 3 — Historical Factory Reconciliation

The third default verifier exited 1 after 12m14.140s at Phase159: 636 tests passed and one original-proof factory failed because it hashed current migrated fixture bytes. The strict historical reader refused that mismatch. The actual log remains `/tmp/open-bitcoin-phase164-native-third.log`. Three affected frozen test factories (Phases159–161) are being reconciled through exact original snapshots and separately reviewed current fingerprints. The 139-file table above is the last reviewed source snapshot; no new source acceptance is inferred until expanded review/security and full native success. Original receipt bytes and raw physical preservation assertions remain authoritative.

## Final Historical Factory Review Binding

All 145 current hashes match their clean scoped reports. The three repaired test factories change exactly one import and one original-proof hash call each; raw current inputs, assertions and physical preservation maps remain unchanged. All twelve original snapshots match the fixed initial revision; the original nine manifest bindings and all 123 historical evidence artifacts are preserved. Current tooling digest is `6e1e19fd449f2bf77f4511cdd1cec8599fb45148ac87808d8be9575f48ba7570`; original historical digest remains `14fa9cc585c9cf8d9b2c0ae04d7f92e6411b38fb7d19128359126eee2735f977`, with no current ordinary qualification. New guard suite 352, provenance 124, selected native 14 and affected factory positives 1/2/1 passed; these targeted results do not assert whole legacy replay. Full default native attempt 4 is running in session 16533, log `/tmp/open-bitcoin-phase164-native-fourth.log`. All three earlier actual failures remain recorded.

## Final Panic Policy Repair Binding

Native attempt 4 passed all Bun suites, then exited 1 after 24m49.836s at the panic policy; its log remains `/tmp/open-bitcoin-phase164-native-fourth.log`. The preservation helper is byte-identical at its conventional test path and compiler-valid explicit cfg-test registration. The initial focused compile path failure is preserved in Plan08. Four exact allowlist entries document a lint-attribute false positive and three proven fixed genesis bounds; genesis production bytes are unchanged. The final 355 guard tests, 24 node compatibility tests and 31 consensus identity tests pass, alongside direct panic, breadcrumb, formatting and managed checks. All 146 file hashes match clean CORE22/SHELL68/AUTOMATION56 snapshots. Full native attempt 5 is running in session 8391, log `/tmp/open-bitcoin-phase164-native-fifth.log`; actual exit zero remains required.

## Final Typed Test Assertion Review Binding

Native attempt 5 passed all Bun, panic and formatting stages, then exited 101 after 24m47.226s on three test-only Clippy err-expect findings; its original log remains `/tmp/open-bitcoin-phase164-native-fifth.log`. Exactly three map-to-unit/expect-err replacements retain the same typed errors, calls and assertions. No production Debug implementation, lint suppression, source-corpus expansion or runtime behavior change is introduced. Final SHELL68 review and all 146 current file hashes match. Serial direct Rust preflight passed format, full Clippy, all-target build, all 4,449 workspace tests with six existing ignored, doc-tests and seven explicit RPC-client tests. The 355 guard tests, six Bazel smoke targets/build provenance, benchmark smoke/report validation also passed. Pure coverage preflight is running; whole default native acceptance and normal-hook Git saving remain pending.

## Final Formatter Failure Coverage Binding

Direct coverage preflight session91777 exposed only chain_spec.rs:218, the first fallible diagnostic write. One behavioral failing-writer test was added; all production bytes, existing 93 inputs/246 guard edges, twelve historical snapshots and original receipts remain unchanged. Serial repair session82469 passed format/full Clippy/32 consensus tests/1,706 pure coverage tests/355 guards. The native text report has no Uncovered Lines and no zero-count rendered rows; the former missing line now executes once. The 26,296 plain integer-count rows are not the complete report denominator because LLVM also renders abbreviated positive counts. This direct preflight failure remains distinct from five failed full native attempts. CORE22/SHELL68/AUTOMATION56 and every146 current file hash match the clean reviews. Full native attempt6 runs in session41473 with `/tmp/open-bitcoin-phase164-native-sixth.log`; actual full exit zero, goal verification and normal-hook Git saving remain required.

## Actual Full Native Acceptance

Session41473 exited zero after 1h0m21.973s (3621973ms), with `/tmp/open-bitcoin-phase164-native-sixth.log`. The full default contract passed every registered Bun guard, formatting, full Clippy, all-target build, workspace and doc tests, explicit RPC-client binary, benchmark list/smoke/report, six Bazel targets/provenance and strict pure-core coverage. Rust summaries total 6,163 passing executions with zero failures and six pre-existing ignored cases (4,450 workspace, seven client and 1,706 coverage). Root independently rechecked every146 reviewed source fingerprint after exit zero; the source aggregate remains da4b07c93d86bdc74aa93efb2ff62ebfa290c8e790cca66bd08656df6c0340c0. Five earlier full native failures plus focused compile/coverage preflight failures remain recorded as dated history. Goal report, lifecycle closure and final normal-hook Git transaction remain pending; this acceptance adds no ordinary canonical qualification.

## Final Closure Documentation Binding

The accepted native source remains da4b07c9…340c0. Exactly three source-document hashes changed afterward: README report link, companion identity/refusal closure and current catalog status/evidence/rationale. Runtime, tests, scripts and twelve original snapshots are byte-identical to the accepted run. Clean AUTOMATION56 closure audit independently verifies exact reversals and preserves historical catalog data. Three requirements have unique complete owners01/02/06; later27 remain Pending, Phase165 unstarted, current13plans complete, archived69 restored and auto-chain false. All82 normal closure checks and lifecycle/traceability/documentation/diff gates pass. Current146source fingerprints match the renewed source snapshot; the final normal pre-commit hook will execute the full contract against this saving candidate. No ordinary canonical qualification or already-completed Git transaction is inferred.

## Saving-Candidate EOF Whitespace Reconciliation

The staged diff exposed extra EOF blank lines in eight new planning/review/verification artifacts and the current TS contract table. Only terminal LF bytes were trimmed to one; no contents, contract rows, assertions, runtime or historical bytes changed. Adding exactly one LF to current contracts.ts reconstructs its previously reviewed SHA2c9b7f8819534ac29d19836b6f2f7f60c6715e553e19473b6c779375fd182167. Its current SHA0a3f44db805a87f4aa53e170aebdc2b0d57930e101719461c46406087a77fda9 is independently reviewed; current146source fingerprints match. The accepted native source differs only in the three closure docs and this terminal LF. Staged whitespace and normal93/246guard pass; final hook remains the full saving gate.
