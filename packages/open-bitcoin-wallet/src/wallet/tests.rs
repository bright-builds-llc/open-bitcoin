// Parity breadcrumbs:
// - packages/bitcoin-knots/src/wallet/wallet.cpp
// - packages/bitcoin-knots/src/wallet/spend.cpp
// - packages/bitcoin-knots/src/wallet/coinselection.cpp
// - packages/bitcoin-knots/src/wallet/transaction.cpp
// - packages/bitcoin-knots/test/functional/wallet_descriptor.py
// - packages/bitcoin-knots/test/functional/feature_segwit.py

use open_bitcoin_chainstate::{ChainPosition, ChainstateSnapshot, Coin};
use open_bitcoin_consensus::{TransactionValidationContext, validate_transaction_with_context};
use open_bitcoin_mempool::validate_standard_transaction;
use open_bitcoin_primitives::{
    BlockHash, BlockHeader, OutPoint, ScriptBuf, ScriptWitness, Transaction, TransactionInput,
    TransactionOutput, Txid,
};

use super::{
    BuildRequest, BuiltTransaction, Recipient, Wallet, WalletSnapshot, WalletUtxo,
    amount_from_sats, standard_wallet_verify_flags,
};
use crate::WalletError;
use crate::address::AddressNetwork;
use crate::descriptor::DescriptorRole;

fn sample_tip(height: u32) -> ChainPosition {
    ChainPosition::new(
        BlockHeader {
            version: 1,
            previous_block_hash: BlockHash::from_byte_array([0_u8; 32]),
            merkle_root: Default::default(),
            time: 1_700_000_000 + height,
            bits: 0x207f_ffff,
            nonce: 1,
        },
        height,
        1,
        i64::from(1_700_000_000 + height),
    )
}

fn script(bytes: &[u8]) -> ScriptBuf {
    ScriptBuf::from_bytes(bytes.to_vec()).expect("script")
}

fn wallet_with_descriptors() -> Wallet {
    let mut wallet = Wallet::new(AddressNetwork::Regtest);
    wallet
        .import_descriptor(
            "receive",
            DescriptorRole::External,
            "wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi)",
        )
        .expect("receive descriptor");
    wallet
        .import_descriptor(
            "change",
            DescriptorRole::Internal,
            "sh(wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi))",
        )
        .expect("change descriptor");
    wallet
}

fn funded_snapshot(wallet: &Wallet) -> ChainstateSnapshot {
    let receive_script = wallet
        .default_receive_address()
        .expect("receive address")
        .script_pubkey;
    let mut utxos = std::collections::HashMap::new();
    utxos.insert(
        OutPoint {
            txid: Txid::from_byte_array([7_u8; 32]),
            vout: 0,
        },
        Coin {
            output: TransactionOutput {
                value: amount_from_sats(75_000).expect("amount"),
                script_pubkey: receive_script,
            },
            is_coinbase: false,
            created_height: 9,
            created_median_time_past: 1_700_000_009,
        },
    );

    ChainstateSnapshot::new(vec![sample_tip(10)], utxos, Default::default())
}

#[test]
fn signing_helper_errors_remain_typed() {
    // Arrange
    let mut wallet = Wallet::new(AddressNetwork::Regtest);
    let descriptor_id = wallet
        .import_descriptor(
            "legacy",
            DescriptorRole::External,
            "pkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi)",
        )
        .expect("legacy descriptor");
    let descriptor = wallet.descriptor(descriptor_id).expect("descriptor");

    // Act
    let redeem_error = super::build::required_redeem_script(descriptor)
        .expect_err("legacy descriptor has no redeem script");
    let taproot_error = super::sign::taproot_sighash_unavailable_error();

    // Assert
    assert!(matches!(
        redeem_error,
        WalletError::UnsupportedSigningDescriptor(_)
    ));
    assert!(matches!(
        taproot_error,
        WalletError::UnsupportedSigningDescriptor(_)
    ));
}

#[test]
fn rescan_populates_wallet_balance_from_matching_chainstate_outputs() {
    let mut wallet = wallet_with_descriptors();
    wallet
        .rescan_chainstate(&funded_snapshot(&wallet))
        .expect("rescan");
    let balance = wallet.balance(100).expect("balance");

    assert_eq!(wallet.utxos().len(), 1);
    assert_eq!(balance.total.to_sats(), 75_000);
    assert_eq!(balance.spendable.to_sats(), 75_000);
    assert_eq!(balance.immature.to_sats(), 0);
}

#[test]
fn build_and_sign_produces_a_standard_spendable_transaction() {
    let mut wallet = wallet_with_descriptors();
    wallet
        .rescan_chainstate(&funded_snapshot(&wallet))
        .expect("rescan");
    let recipient = Recipient::from_address(
        &wallet
            .default_change_address()
            .expect("standard change address"),
        amount_from_sats(30_000).expect("amount"),
    );
    let built = wallet
        .build_and_sign(
            &BuildRequest {
                recipients: vec![recipient],
                fee_rate: open_bitcoin_mempool::FeeRate::from_sats_per_kvb(2000),
                maybe_change_descriptor_id: None,
                maybe_lock_time: None,
                enable_rbf: true,
            },
            100,
        )
        .expect("build and sign");
    let input_contexts = wallet.input_contexts_for(&built).expect("input contexts");
    let validation_context = TransactionValidationContext {
        inputs: input_contexts.clone(),
        spend_height: 11,
        block_time: 1_700_000_010,
        median_time_past: 1_700_000_010,
        verify_flags: standard_wallet_verify_flags(),
        consensus_params: open_bitcoin_consensus::ConsensusParams::default(),
    };

    validate_transaction_with_context(&built.transaction, &validation_context)
        .expect("signed transaction should validate");
    validate_standard_transaction(
        &built.transaction,
        &input_contexts,
        &open_bitcoin_mempool::PolicyConfig::default(),
        open_bitcoin_mempool::transaction_weight_and_virtual_size(&built.transaction)
            .expect("weight")
            .0,
        open_bitcoin_mempool::transaction_sigops_cost(&built.transaction, &input_contexts)
            .expect("sigops"),
    )
    .expect("standard policy");
}

#[test]
fn legacy_descriptor_signing_populates_script_sig() {
    let mut wallet = Wallet::new(AddressNetwork::Regtest);
    let receive_id = wallet
        .import_descriptor(
            "legacy",
            DescriptorRole::External,
            "pkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi)",
        )
        .expect("legacy descriptor");
    wallet
        .import_descriptor(
            "legacy-change",
            DescriptorRole::Internal,
            "wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi)",
        )
        .expect("legacy change descriptor");
    let receive_script = wallet
        .address_for_descriptor(receive_id)
        .expect("address")
        .script_pubkey;
    let mut utxos = std::collections::HashMap::new();
    utxos.insert(
        OutPoint {
            txid: Txid::from_byte_array([8_u8; 32]),
            vout: 0,
        },
        Coin {
            output: TransactionOutput {
                value: amount_from_sats(60_000).expect("amount"),
                script_pubkey: receive_script.clone(),
            },
            is_coinbase: false,
            created_height: 5,
            created_median_time_past: 1_700_000_005,
        },
    );
    wallet
        .rescan_chainstate(&ChainstateSnapshot::new(
            vec![sample_tip(6)],
            utxos,
            Default::default(),
        ))
        .expect("rescan");
    let built = wallet
        .build_and_sign(
            &BuildRequest {
                recipients: vec![Recipient {
                    script_pubkey: script(&[0x51]),
                    value: amount_from_sats(20_000).expect("amount"),
                }],
                fee_rate: open_bitcoin_mempool::FeeRate::from_sats_per_kvb(1500),
                maybe_change_descriptor_id: None,
                maybe_lock_time: None,
                enable_rbf: true,
            },
            100,
        )
        .expect("legacy spend");

    assert!(!built.transaction.inputs[0].script_sig.is_empty());
    assert!(built.transaction.inputs[0].witness.is_empty());
}

#[test]
fn watch_only_outputs_do_not_count_as_spendable() {
    let mut wallet = Wallet::new(AddressNetwork::Regtest);
    wallet
        .import_descriptor(
            "watch",
            DescriptorRole::External,
            "wpkh(03a34b99f22c790c95f2ef4b20e6d4544e4c53aaf2fcd7d7f0ed6f246d1063f404)",
        )
        .expect("watch-only descriptor");
    let snapshot = funded_snapshot(&wallet_with_descriptors());
    wallet.rescan_chainstate(&snapshot).expect("rescan");
    let error = wallet
        .build_transaction(
            &BuildRequest {
                recipients: vec![Recipient {
                    script_pubkey: script(&[0x51]),
                    value: amount_from_sats(10_000).expect("amount"),
                }],
                fee_rate: open_bitcoin_mempool::FeeRate::from_sats_per_kvb(1000),
                maybe_change_descriptor_id: None,
                maybe_lock_time: None,
                enable_rbf: true,
            },
            100,
        )
        .expect_err("watch-only wallet cannot spend");

    assert_eq!(error, WalletError::NoSpendableCoins);
}

#[test]
fn coinbase_outputs_stay_immature_until_the_maturity_window_passes() {
    let mut wallet = wallet_with_descriptors();
    let receive_script = wallet
        .default_receive_address()
        .expect("receive")
        .script_pubkey;
    let mut utxos = std::collections::HashMap::new();
    utxos.insert(
        OutPoint {
            txid: Txid::from_byte_array([9_u8; 32]),
            vout: 0,
        },
        Coin {
            output: TransactionOutput {
                value: amount_from_sats(50_000).expect("amount"),
                script_pubkey: receive_script,
            },
            is_coinbase: true,
            created_height: 15,
            created_median_time_past: 1_700_000_015,
        },
    );
    wallet
        .rescan_chainstate(&ChainstateSnapshot::new(
            vec![sample_tip(20)],
            utxos,
            Default::default(),
        ))
        .expect("rescan");

    let balance = wallet.balance(100).expect("balance");

    assert_eq!(balance.spendable.to_sats(), 0);
    assert_eq!(balance.immature.to_sats(), 50_000);
}

#[test]
fn build_transaction_requires_change_descriptor_for_changeful_spends() {
    let mut wallet = Wallet::new(AddressNetwork::Regtest);
    wallet
        .import_descriptor(
            "receive",
            DescriptorRole::External,
            "wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi)",
        )
        .expect("receive descriptor");
    wallet
        .rescan_chainstate(&funded_snapshot(&wallet_with_descriptors()))
        .expect("rescan");
    let error = wallet
        .build_transaction(
            &BuildRequest {
                recipients: vec![Recipient {
                    script_pubkey: script(&[0x51]),
                    value: amount_from_sats(10_000).expect("amount"),
                }],
                fee_rate: open_bitcoin_mempool::FeeRate::from_sats_per_kvb(1000),
                maybe_change_descriptor_id: None,
                maybe_lock_time: None,
                enable_rbf: true,
            },
            100,
        )
        .expect_err("change descriptor is required");

    assert_eq!(error, WalletError::ChangeDescriptorRequired);
}

#[test]
fn duplicate_labels_and_unspendable_snapshots_are_rejected() {
    let mut wallet = Wallet::new(AddressNetwork::Regtest);
    wallet
        .import_descriptor(
            "receive",
            DescriptorRole::External,
            "wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi)",
        )
        .expect("receive descriptor");
    assert_eq!(
        wallet
            .import_descriptor(
                "receive",
                DescriptorRole::Internal,
                "wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi)",
            )
            .expect_err("duplicate label"),
        WalletError::DuplicateLabel("receive".to_string())
    );

    let watch_only = Wallet::from_snapshot(WalletSnapshot {
        network: AddressNetwork::Regtest,
        descriptors: vec![crate::descriptor::DescriptorRecord {
            id: 0,
            label: "watch".to_string(),
            role: DescriptorRole::External,
            original_text:
                "wpkh(024d4b6cd1361032ca9bd2aeb9d900aa4d45d9ead80ac9423374c451a7254d0766)"
                    .to_string(),
            descriptor: crate::descriptor::SingleKeyDescriptor::parse(
                "wpkh(024d4b6cd1361032ca9bd2aeb9d900aa4d45d9ead80ac9423374c451a7254d0766)",
                AddressNetwork::Regtest,
            )
            .expect("watch descriptor"),
        }],
        utxos: vec![WalletUtxo {
            descriptor_id: 0,
            outpoint: OutPoint {
                txid: Txid::from_byte_array([3_u8; 32]),
                vout: 0,
            },
            output: TransactionOutput {
                value: amount_from_sats(5_000).expect("amount"),
                script_pubkey: crate::descriptor::SingleKeyDescriptor::parse(
                    "wpkh(024d4b6cd1361032ca9bd2aeb9d900aa4d45d9ead80ac9423374c451a7254d0766)",
                    AddressNetwork::Regtest,
                )
                .expect("descriptor")
                .script_pubkey()
                .expect("script"),
            },
            created_height: 1,
            created_median_time_past: 1,
            is_coinbase: false,
        }],
        next_descriptor_id: 1,
        maybe_tip_height: Some(2),
        maybe_tip_median_time_past: Some(2),
    });
    let unknown_descriptor = Wallet::from_snapshot(WalletSnapshot {
        network: AddressNetwork::Regtest,
        descriptors: Vec::new(),
        utxos: vec![WalletUtxo {
            descriptor_id: 99,
            outpoint: OutPoint {
                txid: Txid::from_byte_array([4_u8; 32]),
                vout: 0,
            },
            output: TransactionOutput {
                value: amount_from_sats(5_000).expect("amount"),
                script_pubkey: script(&[0x51]),
            },
            created_height: 1,
            created_median_time_past: 1,
            is_coinbase: false,
        }],
        next_descriptor_id: 0,
        maybe_tip_height: Some(2),
        maybe_tip_median_time_past: Some(2),
    });
    let immature_coinbase = Wallet::from_snapshot(WalletSnapshot {
        network: AddressNetwork::Regtest,
        descriptors: wallet.descriptors().to_vec(),
        utxos: vec![WalletUtxo {
            descriptor_id: 0,
            outpoint: OutPoint {
                txid: Txid::from_byte_array([5_u8; 32]),
                vout: 0,
            },
            output: TransactionOutput {
                value: amount_from_sats(5_000).expect("amount"),
                script_pubkey: wallet
                    .default_receive_address()
                    .expect("receive")
                    .script_pubkey,
            },
            created_height: 10,
            created_median_time_past: 10,
            is_coinbase: true,
        }],
        next_descriptor_id: 1,
        maybe_tip_height: Some(10),
        maybe_tip_median_time_past: Some(10),
    });

    for wallet in [watch_only, unknown_descriptor, immature_coinbase] {
        assert_eq!(
            wallet
                .build_transaction(
                    &BuildRequest {
                        recipients: vec![Recipient {
                            script_pubkey: script(&[0x51]),
                            value: amount_from_sats(1_000).expect("amount"),
                        }],
                        fee_rate: open_bitcoin_mempool::FeeRate::from_sats_per_kvb(1000),
                        maybe_change_descriptor_id: None,
                        maybe_lock_time: None,
                        enable_rbf: true,
                    },
                    100,
                )
                .expect_err("unspendable snapshot"),
            WalletError::NoSpendableCoins
        );
    }
}

#[test]
fn snapshot_round_trips_state_and_address_accessors() {
    let mut wallet = wallet_with_descriptors();
    wallet
        .rescan_chainstate(&funded_snapshot(&wallet))
        .expect("rescan");
    let snapshot = wallet.snapshot();
    let restored = Wallet::from_snapshot(snapshot.clone());

    assert_eq!(wallet.network(), AddressNetwork::Regtest);
    assert_eq!(restored.network(), AddressNetwork::Regtest);
    assert_eq!(snapshot.descriptors.len(), 2);
    assert_eq!(restored.descriptors().len(), 2);
    assert_eq!(restored.utxos().len(), 1);
    assert!(restored.address_for_descriptor(0).is_ok());
    assert!(restored.default_receive_address().is_ok());
    assert!(restored.default_change_address().is_ok());
    assert_eq!(
        restored
            .address_for_descriptor(42)
            .expect_err("missing descriptor"),
        WalletError::UnknownDescriptor(42)
    );
}

#[test]
fn wallet_reports_missing_roles_and_basic_build_errors() {
    let wallet = Wallet::new(AddressNetwork::Regtest);
    assert_eq!(
        wallet
            .default_receive_address()
            .expect_err("missing external"),
        WalletError::ChangeDescriptorRequired
    );
    assert_eq!(
        wallet
            .default_change_address()
            .expect_err("missing internal"),
        WalletError::ChangeDescriptorRequired
    );
    assert_eq!(
        wallet
            .build_transaction(
                &BuildRequest {
                    recipients: Vec::new(),
                    fee_rate: open_bitcoin_mempool::FeeRate::from_sats_per_kvb(1000),
                    maybe_change_descriptor_id: None,
                    maybe_lock_time: None,
                    enable_rbf: true,
                },
                100,
            )
            .expect_err("no recipients"),
        WalletError::NoRecipients
    );
}

#[test]
fn build_transaction_reports_insufficient_funds_and_uses_snapshot_sorting() {
    let mut wallet = wallet_with_descriptors();
    let receive_script = wallet
        .default_receive_address()
        .expect("receive")
        .script_pubkey;
    let mut utxos = std::collections::HashMap::new();
    utxos.insert(
        OutPoint {
            txid: Txid::from_byte_array([2_u8; 32]),
            vout: 1,
        },
        Coin {
            output: TransactionOutput {
                value: amount_from_sats(10_000).expect("amount"),
                script_pubkey: receive_script.clone(),
            },
            is_coinbase: false,
            created_height: 7,
            created_median_time_past: 1_700_000_007,
        },
    );
    utxos.insert(
        OutPoint {
            txid: Txid::from_byte_array([1_u8; 32]),
            vout: 0,
        },
        Coin {
            output: TransactionOutput {
                value: amount_from_sats(40_000).expect("amount"),
                script_pubkey: receive_script,
            },
            is_coinbase: false,
            created_height: 6,
            created_median_time_past: 1_700_000_006,
        },
    );
    wallet
        .rescan_chainstate(&ChainstateSnapshot::new(
            vec![sample_tip(10)],
            utxos,
            Default::default(),
        ))
        .expect("rescan");
    let build = wallet
        .build_transaction(
            &BuildRequest {
                recipients: vec![Recipient {
                    script_pubkey: wallet
                        .default_change_address()
                        .expect("change")
                        .script_pubkey,
                    value: amount_from_sats(20_000).expect("amount"),
                }],
                fee_rate: open_bitcoin_mempool::FeeRate::from_sats_per_kvb(1000),
                maybe_change_descriptor_id: None,
                maybe_lock_time: Some(99),
                enable_rbf: true,
            },
            100,
        )
        .expect("build");
    let insufficient = wallet
        .build_transaction(
            &BuildRequest {
                recipients: vec![Recipient {
                    script_pubkey: script(&[0x51]),
                    value: amount_from_sats(1_000_000).expect("amount"),
                }],
                fee_rate: open_bitcoin_mempool::FeeRate::from_sats_per_kvb(1000),
                maybe_change_descriptor_id: None,
                maybe_lock_time: None,
                enable_rbf: true,
            },
            100,
        )
        .expect_err("insufficient");

    assert_eq!(wallet.utxos()[0].created_height, 6);
    assert_eq!(
        build.selected_inputs[0].outpoint.txid,
        Txid::from_byte_array([1_u8; 32])
    );
    assert_eq!(build.transaction.lock_time, 99);
    assert!(insufficient.to_string().contains("insufficient funds"));
}

#[test]
fn nested_segwit_and_taproot_signing_cover_remaining_descriptor_paths() {
    let mut nested = Wallet::new(AddressNetwork::Regtest);
    nested
        .import_descriptor(
            "nested-receive",
            DescriptorRole::External,
            "sh(wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi))",
        )
        .expect("nested receive");
    nested
        .import_descriptor(
            "nested-change",
            DescriptorRole::Internal,
            "sh(wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi))",
        )
        .expect("nested change");
    nested
        .rescan_chainstate(&funded_snapshot(&nested))
        .expect("nested rescan");
    let nested_spend = nested
        .build_and_sign(
            &BuildRequest {
                recipients: vec![Recipient::from_address(
                    &nested.default_change_address().expect("change"),
                    amount_from_sats(20_000).expect("amount"),
                )],
                fee_rate: open_bitcoin_mempool::FeeRate::from_sats_per_kvb(1200),
                maybe_change_descriptor_id: None,
                maybe_lock_time: None,
                enable_rbf: true,
            },
            100,
        )
        .expect("nested spend");

    assert!(!nested_spend.transaction.inputs[0].script_sig.is_empty());
    assert_eq!(nested_spend.transaction.inputs[0].witness.stack().len(), 2);

    let mut taproot = Wallet::new(AddressNetwork::Regtest);
    taproot
        .import_descriptor(
            "taproot-receive",
            DescriptorRole::External,
            "tr(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi)",
        )
        .expect("taproot receive");
    taproot
        .import_descriptor(
            "taproot-change",
            DescriptorRole::Internal,
            "tr(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi)",
        )
        .expect("taproot change");
    taproot
        .rescan_chainstate(&funded_snapshot(&taproot))
        .expect("taproot rescan");
    let taproot_spend = taproot
        .build_and_sign(
            &BuildRequest {
                recipients: vec![Recipient::from_address(
                    &taproot.default_change_address().expect("change"),
                    amount_from_sats(20_000).expect("amount"),
                )],
                fee_rate: open_bitcoin_mempool::FeeRate::from_sats_per_kvb(1200),
                maybe_change_descriptor_id: None,
                maybe_lock_time: None,
                enable_rbf: true,
            },
            100,
        )
        .expect("taproot spend");

    assert!(taproot_spend.transaction.inputs[0].script_sig.is_empty());
    assert_eq!(taproot_spend.transaction.inputs[0].witness.stack().len(), 1);
}

#[test]
fn final_sequence_and_dust_change_paths_are_covered() {
    let mut wallet = wallet_with_descriptors();
    wallet
        .rescan_chainstate(&funded_snapshot(&wallet))
        .expect("rescan");
    let built = wallet
        .build_transaction(
            &BuildRequest {
                recipients: vec![Recipient::from_address(
                    &wallet.default_change_address().expect("change"),
                    amount_from_sats(74_800).expect("amount"),
                )],
                fee_rate: open_bitcoin_mempool::FeeRate::from_sats_per_kvb(1000),
                maybe_change_descriptor_id: None,
                maybe_lock_time: Some(33),
                enable_rbf: false,
            },
            100,
        )
        .expect("build without change");
    let no_capacity_for_change = wallet
        .build_transaction(
            &BuildRequest {
                recipients: vec![Recipient::from_address(
                    &wallet.default_change_address().expect("change"),
                    amount_from_sats(74_860).expect("amount"),
                )],
                fee_rate: open_bitcoin_mempool::FeeRate::from_sats_per_kvb(1000),
                maybe_change_descriptor_id: None,
                maybe_lock_time: Some(34),
                enable_rbf: false,
            },
            100,
        )
        .expect("build with no room for change");

    assert_eq!(built.change_output_index, None);
    assert_eq!(
        built.transaction.inputs[0].sequence,
        TransactionInput::SEQUENCE_FINAL
    );
    assert_eq!(built.transaction.lock_time, 33);
    assert_eq!(no_capacity_for_change.change_output_index, None);
}

#[test]
fn change_outputs_and_sort_tiebreakers_are_explicit() {
    let mut wallet = wallet_with_descriptors();
    wallet
        .rescan_chainstate(&funded_snapshot(&wallet))
        .expect("rescan");
    let with_change = wallet
        .build_transaction(
            &BuildRequest {
                recipients: vec![Recipient::from_address(
                    &wallet.default_change_address().expect("change"),
                    amount_from_sats(30_000).expect("amount"),
                )],
                fee_rate: open_bitcoin_mempool::FeeRate::from_sats_per_kvb(1000),
                maybe_change_descriptor_id: None,
                maybe_lock_time: None,
                enable_rbf: true,
            },
            100,
        )
        .expect("changeful spend");

    assert!(with_change.change_output_index.is_some());
    assert!(
        wallet
            .estimate_vsize(
                &with_change.selected_inputs,
                &[Recipient::from_address(
                    &wallet.default_change_address().expect("change"),
                    amount_from_sats(30_000).expect("amount"),
                )],
                Some(&TransactionOutput {
                    value: amount_from_sats(1).expect("amount"),
                    script_pubkey: wallet
                        .default_change_address()
                        .expect("change")
                        .script_pubkey,
                }),
                &BuildRequest {
                    recipients: vec![Recipient::from_address(
                        &wallet.default_change_address().expect("change"),
                        amount_from_sats(30_000).expect("amount"),
                    )],
                    fee_rate: open_bitcoin_mempool::FeeRate::from_sats_per_kvb(1000),
                    maybe_change_descriptor_id: None,
                    maybe_lock_time: None,
                    enable_rbf: true,
                },
            )
            .expect("estimate with change")
            > 0
    );

    let receive_script = wallet
        .default_receive_address()
        .expect("receive")
        .script_pubkey;
    let mut equal_snapshot = std::collections::HashMap::new();
    equal_snapshot.insert(
        OutPoint {
            txid: Txid::from_byte_array([9_u8; 32]),
            vout: 0,
        },
        Coin {
            output: TransactionOutput {
                value: amount_from_sats(15_000).expect("amount"),
                script_pubkey: receive_script.clone(),
            },
            is_coinbase: false,
            created_height: 3,
            created_median_time_past: 3,
        },
    );
    equal_snapshot.insert(
        OutPoint {
            txid: Txid::from_byte_array([1_u8; 32]),
            vout: 1,
        },
        Coin {
            output: TransactionOutput {
                value: amount_from_sats(15_000).expect("amount"),
                script_pubkey: receive_script,
            },
            is_coinbase: false,
            created_height: 3,
            created_median_time_past: 3,
        },
    );
    wallet
        .rescan_chainstate(&ChainstateSnapshot::new(
            vec![sample_tip(4)],
            equal_snapshot,
            Default::default(),
        ))
        .expect("rescan equal snapshot");
    let equal_build = wallet
        .build_transaction(
            &BuildRequest {
                recipients: vec![Recipient {
                    script_pubkey: script(&[0x51]),
                    value: amount_from_sats(10_000).expect("amount"),
                }],
                fee_rate: open_bitcoin_mempool::FeeRate::from_sats_per_kvb(1000),
                maybe_change_descriptor_id: None,
                maybe_lock_time: None,
                enable_rbf: true,
            },
            100,
        )
        .expect("equal-value build");

    assert_eq!(
        wallet.utxos()[0].outpoint.txid,
        Txid::from_byte_array([1_u8; 32])
    );
    assert_eq!(
        equal_build.selected_inputs[0].outpoint.txid,
        Txid::from_byte_array([1_u8; 32])
    );
}

#[test]
fn signing_reports_missing_private_keys_and_watch_only_paths() {
    let watch_descriptor = crate::descriptor::SingleKeyDescriptor::parse(
        "pkh(024d4b6cd1361032ca9bd2aeb9d900aa4d45d9ead80ac9423374c451a7254d0766)",
        AddressNetwork::Regtest,
    )
    .expect("watch descriptor");
    let watch_utxo = WalletUtxo {
        descriptor_id: 0,
        outpoint: OutPoint {
            txid: Txid::from_byte_array([6_u8; 32]),
            vout: 0,
        },
        output: TransactionOutput {
            value: amount_from_sats(5_000).expect("amount"),
            script_pubkey: watch_descriptor.script_pubkey().expect("script"),
        },
        created_height: 1,
        created_median_time_past: 1,
        is_coinbase: false,
    };
    let watch_wallet = Wallet::from_snapshot(WalletSnapshot {
        network: AddressNetwork::Regtest,
        descriptors: vec![crate::descriptor::DescriptorRecord {
            id: 0,
            label: "watch".to_string(),
            role: DescriptorRole::External,
            original_text:
                "pkh(024d4b6cd1361032ca9bd2aeb9d900aa4d45d9ead80ac9423374c451a7254d0766)"
                    .to_string(),
            descriptor: watch_descriptor,
        }],
        utxos: vec![watch_utxo.clone()],
        next_descriptor_id: 1,
        maybe_tip_height: Some(2),
        maybe_tip_median_time_past: Some(2),
    });
    let watch_built = BuiltTransaction {
        transaction: Transaction {
            version: 2,
            inputs: vec![TransactionInput {
                previous_output: watch_utxo.outpoint.clone(),
                script_sig: ScriptBuf::default(),
                sequence: TransactionInput::MAX_SEQUENCE_NONFINAL,
                witness: ScriptWitness::default(),
            }],
            outputs: vec![TransactionOutput {
                value: amount_from_sats(4_000).expect("amount"),
                script_pubkey: script(&[0x51]),
            }],
            lock_time: 0,
        },
        selected_inputs: vec![watch_utxo],
        fee: amount_from_sats(1_000).expect("amount"),
        change_output_index: None,
    };
    assert!(
        watch_wallet
            .sign_transaction(&watch_built)
            .expect_err("missing legacy key")
            .to_string()
            .contains("descriptor cannot sign")
    );

    let witness_watch_descriptor = crate::descriptor::SingleKeyDescriptor::parse(
        "wpkh(024d4b6cd1361032ca9bd2aeb9d900aa4d45d9ead80ac9423374c451a7254d0766)",
        AddressNetwork::Regtest,
    )
    .expect("wpkh watch-only");
    let witness_watch = Wallet::from_snapshot(WalletSnapshot {
        network: AddressNetwork::Regtest,
        descriptors: vec![crate::descriptor::DescriptorRecord {
            id: 0,
            label: "wpkh-watch".to_string(),
            role: DescriptorRole::External,
            original_text:
                "wpkh(024d4b6cd1361032ca9bd2aeb9d900aa4d45d9ead80ac9423374c451a7254d0766)"
                    .to_string(),
            descriptor: witness_watch_descriptor.clone(),
        }],
        utxos: vec![WalletUtxo {
            descriptor_id: 0,
            outpoint: OutPoint {
                txid: Txid::from_byte_array([8_u8; 32]),
                vout: 0,
            },
            output: TransactionOutput {
                value: amount_from_sats(5_000).expect("amount"),
                script_pubkey: witness_watch_descriptor.script_pubkey().expect("script"),
            },
            created_height: 1,
            created_median_time_past: 1,
            is_coinbase: false,
        }],
        next_descriptor_id: 1,
        maybe_tip_height: Some(2),
        maybe_tip_median_time_past: Some(2),
    });
    let witness_watch_built = BuiltTransaction {
        transaction: Transaction {
            version: 2,
            inputs: vec![TransactionInput {
                previous_output: OutPoint {
                    txid: Txid::from_byte_array([8_u8; 32]),
                    vout: 0,
                },
                script_sig: ScriptBuf::default(),
                sequence: TransactionInput::MAX_SEQUENCE_NONFINAL,
                witness: ScriptWitness::default(),
            }],
            outputs: vec![TransactionOutput {
                value: amount_from_sats(4_000).expect("amount"),
                script_pubkey: script(&[0x51]),
            }],
            lock_time: 0,
        },
        selected_inputs: witness_watch.utxos().to_vec(),
        fee: amount_from_sats(1_000).expect("amount"),
        change_output_index: None,
    };
    assert!(
        witness_watch
            .sign_transaction(&witness_watch_built)
            .expect_err("missing segwit key")
            .to_string()
            .contains("descriptor cannot sign")
    );

    let taproot_watch_descriptor = crate::descriptor::SingleKeyDescriptor::parse(
        "tr(4d4b6cd1361032ca9bd2aeb9d900aa4d45d9ead80ac9423374c451a7254d0766)",
        AddressNetwork::Regtest,
    )
    .expect("taproot watch-only descriptor");
    let taproot_utxo = WalletUtxo {
        descriptor_id: 0,
        outpoint: OutPoint {
            txid: Txid::from_byte_array([7_u8; 32]),
            vout: 0,
        },
        output: TransactionOutput {
            value: amount_from_sats(5_000).expect("amount"),
            script_pubkey: taproot_watch_descriptor.script_pubkey().expect("script"),
        },
        created_height: 1,
        created_median_time_past: 1,
        is_coinbase: false,
    };
    let taproot_watch = Wallet::from_snapshot(WalletSnapshot {
        network: AddressNetwork::Regtest,
        descriptors: vec![crate::descriptor::DescriptorRecord {
            id: 0,
            label: "taproot-watch".to_string(),
            role: DescriptorRole::External,
            original_text: "tr(4d4b6cd1361032ca9bd2aeb9d900aa4d45d9ead80ac9423374c451a7254d0766)"
                .to_string(),
            descriptor: taproot_watch_descriptor,
        }],
        utxos: vec![taproot_utxo.clone()],
        next_descriptor_id: 1,
        maybe_tip_height: Some(2),
        maybe_tip_median_time_past: Some(2),
    });
    let taproot_built = BuiltTransaction {
        transaction: Transaction {
            version: 2,
            inputs: vec![TransactionInput {
                previous_output: taproot_utxo.outpoint.clone(),
                script_sig: ScriptBuf::default(),
                sequence: TransactionInput::MAX_SEQUENCE_NONFINAL,
                witness: ScriptWitness::default(),
            }],
            outputs: vec![TransactionOutput {
                value: amount_from_sats(4_000).expect("amount"),
                script_pubkey: script(&[0x51]),
            }],
            lock_time: 0,
        },
        selected_inputs: vec![taproot_utxo],
        fee: amount_from_sats(1_000).expect("amount"),
        change_output_index: None,
    };

    assert!(
        taproot_watch
            .sign_transaction(&taproot_built)
            .expect_err("missing taproot key")
            .to_string()
            .contains("descriptor cannot sign")
    );
}
