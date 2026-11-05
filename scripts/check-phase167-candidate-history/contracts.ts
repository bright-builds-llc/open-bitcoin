/** Reviewed offline commands, in the exact native verification order. */
export const OFFLINE_COMMANDS = [
  "bun run scripts/phase167-historical-corpus.ts --check",
  "bun test ./scripts/phase167-historical-corpus.test.ts",
  "bun run scripts/phase167-bip30-fixtures.ts --check",
  "bun test ./scripts/phase167-bip30-fixtures.test.ts",
  "bun run scripts/phase167-regtest-corpus.ts --check",
  "bun test ./scripts/phase167-regtest-corpus.test.ts",
  "bun test ./scripts/check-phase167-candidate-history.test.ts",
  "bun run scripts/check-phase167-candidate-history.ts",
] as const;

export const STEP_LABELS = [
  "check Phase 167 historical corpus",
  "test Phase 167 historical corpus",
  "check Phase 167 BIP30 fixtures",
  "test Phase 167 BIP30 fixtures",
  "check Phase 167 Regtest corpus",
  "test Phase 167 Regtest corpus",
  "test Phase 167 candidate history boundary",
  "check Phase 167 candidate history boundary",
] as const;

export const VERIFY_BASE_SHA = "2925344e06e3604fbba1821e49a00b0f2f292448eae205107882d0ed65aacc43";
export const VERIFY_FILE = "scripts/verify.sh";

/** Frozen accepted FINAL17 product and Phase167 corpus bytes; never refresh at runtime. */
export const CURRENT_PINS: readonly (readonly [string, string, "production" | "native-test" | "corpus-script" | "corpus"])[] = [
  ["packages/open-bitcoin-consensus/src/chain_spec.rs","8eabc09362573add6485def8b966bd82297d6d085094856c45119bd603485351","production"],
  ["packages/open-bitcoin-consensus/src/chain_spec/rules.rs","530f5eb1e2a79c794b85dcf3718cf640e8438acd15282003579290541abf3cac","production"],
  ["packages/open-bitcoin-consensus/src/candidate_history.rs","9949330d3f0af724c3be268755dacc66fde3b222240cf197a0ca164dda240146","production"],
  ["packages/open-bitcoin-consensus/src/candidate_history/rules.rs","3e4b53c81c7da84a2625a629d9ddce801d437bc9fa6f713bce51053585ab2d84","production"],
  ["packages/open-bitcoin-consensus/src/candidate_history/header_rules.rs","d86cccbcb2d83d5112f89d537bc7058a81f49810a54fd4c42fd2cb5fd1fbc3e5","production"],
  ["packages/open-bitcoin-consensus/src/transaction.rs","57922894913bedb8c588be50df05defcacf876be8294c149de803eaa19f19113","production"],
  ["packages/open-bitcoin-consensus/src/block.rs","14d296dee677feec51a928f26224dce0fef03c45ecc0003abef974b04d594bcd","production"],
  ["packages/open-bitcoin-consensus/src/block/sigops.rs","745659a3dda954d4339685be1e46796da204fe56fc248bc6cc94909fad641afa","production"],
  ["packages/open-bitcoin-consensus/src/script.rs","a4aea9b589ce4b065af505d77ddaf705b9b1ec12d307c3d6e5250991a702e49a","production"],
  ["packages/open-bitcoin-consensus/src/script/witness.rs","2a019eb41fd79db73f89431da3ff17c64dd7f3cf3bbd531e4e750289bdd4abcf","production"],
  ["packages/open-bitcoin-consensus/src/script/legacy.rs","2834f0c7c32e2c9215291ebd9543ccd3963dfa75c3933994954f109c2c7d1b0e","production"],
  ["packages/open-bitcoin-consensus/src/script/taproot.rs","b1a529274989881b381f32f3ddb170f1935fbf3a094654aabe588bbea3c50503","production"],
  ["packages/open-bitcoin-consensus/src/script/stack.rs","2e66e2b5d0c134e1cd7591979a817bf463b1ef1cff78a2faf2cbfcf9c2d5a939","production"],
  ["packages/open-bitcoin-consensus/src/script/opcodes.rs","eda3783069cf67cbc2be16475e64c5cb0952aacc94a444215a4e94d7ba6ed980","production"],
  ["packages/open-bitcoin-consensus/src/script/encoding.rs","b9746d6d1354713b56fc8e1f1e550bafbe0e1ee2aefc73c9616956fb95afcf55","production"],
  ["packages/open-bitcoin-consensus/src/script/locktime.rs","cedfbb3c447cefdaa15e6ef3e129a18871c56ea1c2ae6503a615592720619f54","production"],
  ["packages/open-bitcoin-consensus/src/script/sigops.rs","8e3b6bcf043232191e901f9b71809d1108e24a7bc4e3064ad54b650a291ef46a","production"],
  ["packages/open-bitcoin-chainstate/src/engine.rs","f7db4e5a6562b01061ca8a6466246f2e0b0fc5161c0029d1d000e1b20861e465","production"],
  ["packages/open-bitcoin-chainstate/src/engine/candidate_rules.rs","415a958844e6f68f8ef22054aead7498e6c63bca79d0438b74d8a3a5efaa2e72","production"],
  ["packages/open-bitcoin-chainstate/src/engine/overwrite_undo.rs","700ae4628e0fc38652e4dd3376b568069d2274aa1d082fb5628a34648b776373","production"],
  ["packages/open-bitcoin-node/src/sync/canonical_history.rs","8cf0783dbd7ae764eb5c366d6500c4f51bd119c008d9dc9379b57a895e227e64","production"],
  ["packages/open-bitcoin-node/src/storage/fjall_store/canonical_genesis.rs","4a65e4568b06af964e19b919521151694a864d96ed52bcde2789b3386e1556b9","production"],
  ["packages/open-bitcoin-rpc/src/context/canonical_history.rs","b8e797712a1533baddf56fd38c94c24c6c20573d86f0195a5cb7f54c324f90a9","production"],
  ["packages/open-bitcoin-consensus/src/chain_spec/rules/tests.rs","cd8f200ac0e7f5b72dc9e48f1fbf33bd095192c1ace173741507a003184d2a93","native-test"],
  ["packages/open-bitcoin-consensus/src/candidate_history/rules/tests.rs","02db06cfc59ec71a6f537cb2bc6fe5bd6132cf1685a157291d524844ee672e48","native-test"],
  ["packages/open-bitcoin-consensus/src/block/sigops/tests.rs","f167191a10a4ee9cc532b7a2d60f4c50e70e7ba129cbbb4164272bca89ad24b1","native-test"],
  ["packages/open-bitcoin-chainstate/src/engine/tests/phase167_context.rs","eb96b688358bbe332141107b107d514331c9af26a1b43947ac41e41c903d3ba4","native-test"],
  ["packages/open-bitcoin-chainstate/src/engine/tests/phase167_bip30.rs","520359cd2ed7d77976a4a35e24777b807bf042de52ca824a2d8c8a304842f84c","native-test"],
  ["packages/open-bitcoin-node/src/sync/tests/phase167_historical/scripts.rs","75b38753bf3a10b5958c92b65d1e2af8d06163f07a1c03da70e391c5fe4a022b","native-test"],
  ["packages/open-bitcoin-node/src/sync/tests/phase167_ordinary.rs","92aa5b60d244f5e639e0548b22a510aeb39643aaaa31be771cf2625c974a6f41","native-test"],
  ["packages/open-bitcoin-rpc/src/bin/open_bitcoind/phase167_tests.rs","b1739ee41837f7cbae51a6b517c935d00f0d82d987c2b2295691d6bbc5e4fba0","native-test"],
  ["scripts/phase167-historical-corpus.test.ts","2f6a71285e0ccb4731fc4d831d602846cd31b178b8990ee335236419c672e62d","native-test"],
  ["scripts/phase167-bip30-fixtures.test.ts","651d58b781192e0137677a4fbabcba3f92de2311d127b9bf348e08d4af90dc6b","native-test"],
  ["scripts/phase167-regtest-corpus.test.ts","25853da78da10300e6d7915d7a9d2c96394a3b7070d2846671e6e14723e4d93b","native-test"],
  ["scripts/phase167-historical-corpus.ts","5f9031294493bd42be0ff4a29bf6c4501fc9fb0b1ad8c7d0a8b720ffe7d0459b","corpus-script"],
  ["scripts/phase167-bip30-fixtures.ts","e06f7675c50adda042ff64f3e95bf42a17953b77aad457cd13fc1510c36ffb63","corpus-script"],
  ["scripts/phase167-regtest-corpus.ts","f921cac020a7d4f3df84eecdc066b891d62c3d4d0d0caf81d8884462046556b9","corpus-script"],
  ["packages/testdata/phase167/manifest.json","16649d2cee73bfd769632bd8b59a06af50ee2d8bb69a2a71dcaad8be88d53b09","corpus"],
  ["packages/testdata/phase167/blocks.json","6e81f7f06037d8c1e18b75ddef67d16aa6ab09a505ca616db612de1f29e0c335","corpus"],
  ["packages/testdata/phase167/contexts.json","3067e98fb235c97226482a6c1b9fad2484fe58610f5ed1c25e8860a0a970fd5a","corpus"],
  ["packages/testdata/phase167/bip30-fixtures.txt","7b91df3ceee0f702c46f98911f4dc92a85c85a0327697e424810c46a379fff5a","corpus"],
  ["packages/testdata/phase167/regtest-history.json","1104ddd93f61b91ed0e8d531dc7e2df347162b473fa9936fa47668623edd3377","corpus"],
  ["docs/parity/fixtures/phase167-rule-expectations.json","4dddecfbb11a93eac11474d23f80bfff08736713a02941f7f21feb32b7a88c24","corpus"],
  ["docs/parity/fixtures/phase167-historical-results.json","5948da8d8c5776e9bfeecd71202fe99db2125a7ca8f38d8d0b8e4f5b93700cde","corpus"],
  ["docs/parity/fixtures/phase167-regtest-expectations.json","c695b6bf4e3dce3f8f1d9478bb5f711ce6d35b3d5153f9c6e5d9630150b8ce07","corpus"],
];

/** Exact Knots 29.3 pin membership shared by the historical corpus. */
export const KNOTS_PINS: readonly (readonly [string, string])[] = [
  ["packages/bitcoin-knots/src/validation.cpp","5bb31c7417ef3fe06a2afefee8a1c32607aed737fdd623368cdc4be72d23d495"],
  ["packages/bitcoin-knots/src/kernel/chainparams.cpp","17170008ef7abe50f1fbde9a854e443b7d0b1119780c8a58329538e8ee038485"],
  ["packages/bitcoin-knots/src/consensus/tx_verify.cpp","8fbc9bde843c2cfb0577ae7d179ad11f928b09bf8b0fc3e43a9ede7dc34a7370"],
  ["packages/bitcoin-knots/src/pow.cpp","9b27f47410b0906b30f12ead4a95e4d345f7832606838f1440ef35b84c708fe2"],
  ["packages/bitcoin-knots/src/arith_uint256.cpp","33099a41d032d02f49f2d0269f31749c6b9d7cee3fdc8ce0c3cadfe46786d1a8"],
  ["packages/bitcoin-knots/src/coins.cpp","12bce0202076a4dd462f16833a6a8f650fe1f6cb283cb2e9955d31542128a95d"],
  ["packages/bitcoin-knots/src/chain.h","c0188574b418a1b67b66d17b4cdb46eb1738bc6e6155514c3d11cc3235db5b1c"],
  ["packages/bitcoin-knots/src/deploymentstatus.h","7cb50f0350d1a0dfa3d10ecd9c9c74106505662e921c695e6932d69a312dbe09"],
  ["packages/bitcoin-knots/src/script/interpreter.h","885ccdca3743cdbd659ae82cd6b77aca299472c6b0d8deb014e4775d8bfdbed3"],
  ["packages/bitcoin-knots/src/clientversion.h","a4ba67ee83b0debad634e7ea7316c6392a8a35dfa130af126313a059413736fa"],
  ["packages/bitcoin-knots/src/clientversion.cpp","e4c8dab790024c5221faaac423d7fbc42ec9e12e4a660272592babaaf199c568"],
  ["packages/bitcoin-knots/CMakeLists.txt","91e3f514a72f254e1728f4e9c306cb0a1cd669cff08f7d24b3ae99097f08947b"],
];

export const REQUIRED_BLOCKS = [
  {
    "network": "mainnet",
    "hash": "00000000000002dc756eebf4f49723ed8d30cc28a5f108eb94b1ba88ac4f9c22",
    "purpose": "script-p2sh-exception"
  },
  {
    "network": "mainnet",
    "hash": "0000000000000000000f14c35b2d841e986ab5441de8c585d5ffe55ea1e395ad",
    "purpose": "script-taproot-exception"
  },
  {
    "network": "testnet3",
    "hash": "00000000dd30457c001f4095d208cc1296b0eed002427aa599874af7a432b105",
    "purpose": "script-p2sh-exception"
  },
  {
    "network": "mainnet",
    "hash": "00000000000a4d0a398161ffc163c503763b1f4360639393e0e4c8e300e0caec",
    "purpose": "bip30-repeat-91842"
  },
  {
    "network": "mainnet",
    "hash": "00000000000743f190a18c5577a3c2d2a1f610ae9601ac046a38084ccb7cd721",
    "purpose": "bip30-repeat-91880"
  },
  {
    "network": "mainnet",
    "hash": "00000000000271a2dc26e7667f8419f2e15416dc6955e5a6c6cdf3f2574dd08e",
    "purpose": "bip30-original-91722"
  },
  {
    "network": "mainnet",
    "hash": "00000000000af0aed4792b1acee3d966af36cf5def14935db8de83d6f9306f2f",
    "purpose": "bip30-original-91812"
  }
] as const;

/** Executable selected seams supplement the full fixed source identities. */
export const BEHAVIOR_ROWS: readonly (readonly [string, string, string])[] = [
  ["selected-halving", "packages/open-bitcoin-consensus/src/chain_spec/rules.rs", "CanonicalChain::Regtest => 150,"],
  ["coinbase-maturity", "packages/open-bitcoin-consensus/src/chain_spec/rules.rs", "pub const fn coinbase_maturity(&self) -> u32 {\n        100"],
  ["exception-identity", "packages/open-bitcoin-consensus/src/candidate_history/rules.rs", ".maybe_script_flag_exception(facts.candidate_hash)"],
  ["deployment-additions", "packages/open-bitcoin-consensus/src/candidate_history/rules.rs", "flags |= addition;"],
  ["sigop-budget", "packages/open-bitcoin-consensus/src/block.rs", "sigop_budget.add_transaction(coinbase, &[], crate::ScriptVerifyFlags::NONE)?;"],
  ["script-interpreter", "packages/open-bitcoin-consensus/src/transaction.rs", "verify_input_script(ScriptInputVerificationContext {"],
  ["script-parent-bridge", "packages/open-bitcoin-consensus/src/script.rs", "witness::verify_input_script(context)"],
  ["witness-execution", "packages/open-bitcoin-consensus/src/script/witness.rs", "verify_top_stack_true(&stack)?;"],
  ["current-tip-context", "packages/open-bitcoin-chainstate/src/engine.rs", "self.selected_candidate_context(&self.active_chain, &block.header, &[], 0)?;"],
  ["eleven-sample-mtp", "packages/open-bitcoin-chainstate/src/engine.rs", ".take(MEDIAN_TIME_PAST_WINDOW - usize::from(maybe_new_time.is_some()))"],
  ["bip30-recheck", "packages/open-bitcoin-consensus/src/candidate_history/rules.rs", "if facts.height >= facts.network.bip30_recheck_height() {"],
  ["bip30-scan-gate", "packages/open-bitcoin-chainstate/src/engine/overwrite_undo.rs", "&& !history.rules().requires_bip30_scan()"],
  ["lossless-overwrite", "packages/open-bitcoin-chainstate/src/engine/overwrite_undo.rs", "batch.overwritten.push(OverwrittenCoin {"],
  ["prior-coin-dto", "packages/open-bitcoin-chainstate/src/engine/overwrite_undo.rs", "transaction_index: index as u32,\n                outpoint,\n                previous_coin,\n                new_coin: Coin {"],
  ["sealed-history-opener", "packages/open-bitcoin-node/src/sync/canonical_history.rs", "opened\n            .into_parts()"],
  ["signet-refusal", "packages/open-bitcoin-consensus/src/chain_spec.rs", "CanonicalChain::Mainnet | CanonicalChain::Testnet3 | CanonicalChain::Signet => {"],
  ["seeded-origin", "packages/open-bitcoin-consensus/src/candidate_history.rs", "history.origin = HistoryOrigin::IndependentlySeeded;"],
  ["fixture-promotion", "packages/open-bitcoin-chainstate/src/engine/candidate_rules.rs", "if !self.work_authority.is_mechanical_fixture()\n            || history.origin() != HistoryOrigin::IndependentlySeeded"],
  ["wallet-unavailable", "packages/open-bitcoin-rpc/src/context/canonical_history.rs", "WalletState::Unavailable,"],
  ["default-regtest-only", "packages/open-bitcoin-consensus/src/chain_spec.rs", "CanonicalChain::Regtest => LocalOrdinaryImplementation::BoundedDefaultRegtest,"],
  ["ordinary-unqualified", "packages/open-bitcoin-consensus/src/chain_spec.rs", "pub const fn ordinary_history(&self) -> bool {\n        false"],
];

/** Nonempty native behavior registrations remain source-bound; Bun cannot replace Rust execution. */
export const NATIVE_TESTS: readonly (readonly [string, string])[] = [
  ["packages/open-bitcoin-consensus/src/chain_spec/rules/tests.rs", "phase167_network_rules_all_selector_pairs_are_distinct"],
  ["packages/open-bitcoin-consensus/src/candidate_history/rules/tests.rs", "phase167_candidate_rules_exception_replacement_precedes_additions"],
  ["packages/open-bitcoin-consensus/src/block/sigops/tests.rs", "phase167_sigops_exact_eighty_thousand_and_one_over"],
  ["packages/open-bitcoin-chainstate/src/engine/tests/phase167_context.rs", "phase167_context_mtp_includes_current_and_only_ten_parents"],
  ["packages/open-bitcoin-chainstate/src/engine/tests/phase167_bip30.rs", "phase167_bip30_real_repeats_stage_restore_and_reconnect"],
  ["packages/open-bitcoin-node/src/sync/tests/phase167_historical/scripts.rs", "phase167_historical_scripts_exact_exception_bodies_reach_managed_kernel"],
  ["packages/open-bitcoin-node/src/sync/tests/phase167_ordinary.rs", "phase167_ordinary_serialized_301_reopen_150_301_and_next_302"],
  ["packages/open-bitcoin-rpc/src/bin/open_bitcoind/phase167_tests.rs", "phase167_daemon_history_route_uses_one_sealed_runtime_and_same_rpc_context"],
];
