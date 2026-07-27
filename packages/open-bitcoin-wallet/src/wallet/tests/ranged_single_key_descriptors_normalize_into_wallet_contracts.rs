// Parity breadcrumbs:
// - packages/bitcoin-knots/src/wallet/wallet.cpp
// - packages/bitcoin-knots/src/wallet/spend.cpp
// - packages/bitcoin-knots/src/wallet/coinselection.cpp
// - packages/bitcoin-knots/src/wallet/transaction.cpp
// - packages/bitcoin-knots/test/functional/wallet_descriptor.py
// - packages/bitcoin-knots/test/functional/feature_segwit.py

use super::*;

#[test]
fn ranged_single_key_descriptors_normalize_into_wallet_contracts() {
    // Arrange
    let mut wallet = Wallet::new(AddressNetwork::Regtest);

    // Act
    wallet
        .import_descriptor(
            "receive-ranged",
            DescriptorRole::External,
            &format!("wpkh({RANGED_TPRV}/1/1/*)"),
        )
        .expect("ranged external descriptor");
    wallet
        .import_descriptor(
            "change-ranged",
            DescriptorRole::Internal,
            &format!("sh(wpkh([{}/84h/1h/0h]{}/1/*))", "d34db33f", RANGED_TPUB),
        )
        .expect("ranged internal descriptor");
    let taproot = crate::descriptor::SingleKeyDescriptor::parse(
        &format!("tr({TAPROOT_TPRV}/0/*)"),
        AddressNetwork::Regtest,
    )
    .expect("ranged taproot descriptor");
    let multipath_error = crate::descriptor::SingleKeyDescriptor::parse(
        &format!("wpkh({RANGED_TPUB}/<0;1>/*)"),
        AddressNetwork::Regtest,
    )
    .expect_err("multipath should stay deferred");
    let miniscript_error = crate::descriptor::SingleKeyDescriptor::parse(
        &format!("wsh(multi(2,{RANGED_TPUB}/0/*,{RANGED_TPUB}/1/*))"),
        AddressNetwork::Regtest,
    )
    .expect_err("multisig should stay deferred");

    // Assert
    let descriptors = wallet.descriptors();
    assert_eq!(descriptors.len(), 2);
    assert!(descriptors[0].descriptor.is_ranged());
    assert_eq!(descriptors[0].descriptor.range_start(), Some(0));
    assert_eq!(descriptors[0].descriptor.range_end(), Some(1000));
    assert_eq!(descriptors[0].descriptor.next_index(), Some(0));
    assert!(descriptors[1].descriptor.is_ranged());
    assert_eq!(descriptors[1].descriptor.range_start(), Some(0));
    assert_eq!(descriptors[1].descriptor.range_end(), Some(1000));
    assert_eq!(descriptors[1].descriptor.next_index(), Some(0));
    assert!(taproot.is_ranged());
    assert_eq!(taproot.range_start(), Some(0));
    assert_eq!(taproot.range_end(), Some(1000));
    assert_eq!(
        multipath_error.to_string(),
        "unsupported descriptor: multipath descriptors remain deferred"
    );
    assert_eq!(
        miniscript_error.to_string(),
        "unsupported descriptor: miniscript and multisig descriptors remain deferred"
    );
}

#[test]
fn address_allocation_advances_descriptor_cursors_once_per_success() {
    // Arrange
    let mut wallet = Wallet::new(AddressNetwork::Regtest);
    wallet
        .import_descriptor(
            "receive-ranged",
            DescriptorRole::External,
            &format!("wpkh({RANGED_TPRV}/1/1/*)"),
        )
        .expect("ranged receive descriptor");
    wallet
        .import_descriptor(
            "change-ranged",
            DescriptorRole::Internal,
            &format!("sh(wpkh({RANGED_TPUB}/1/*))"),
        )
        .expect("ranged change descriptor");

    // Act
    let first_receive = wallet
        .allocate_receive_address()
        .expect("first receive address");
    let second_receive = wallet
        .allocate_receive_address()
        .expect("second receive address");
    let first_change = wallet
        .allocate_change_address()
        .expect("first change address");

    // Assert
    assert_ne!(first_receive.as_str(), second_receive.as_str());
    assert_ne!(first_receive.as_str(), first_change.as_str());
    assert_eq!(wallet.descriptors()[0].descriptor.next_index(), Some(2));
    assert_eq!(wallet.descriptors()[1].descriptor.next_index(), Some(1));
}

#[test]
fn send_intent_contract_captures_fee_selection_and_change_policy() {
    // Arrange
    let recipient = Recipient {
        script_pubkey: script(&[0x51]),
        value: amount_from_sats(12_000).expect("amount"),
    };

    // Act
    let explicit = SendIntent::new(
        vec![recipient.clone()],
        FeeSelection::Explicit(open_bitcoin_mempool::FeeRate::from_sats_per_kvb(1500)),
        ChangePolicy::FixedDescriptor(9),
        Some(33),
        true,
        Some(750),
    )
    .expect("explicit send intent");
    let resolved = explicit
        .into_build_request(None)
        .expect("explicit fee selection resolves directly");
    let estimated = SendIntent::new(
        vec![recipient.clone()],
        FeeSelection::Estimate(FeeEstimateRequest {
            conf_target: 6,
            mode: FeeEstimateMode::Conservative,
        }),
        ChangePolicy::Automatic,
        None,
        false,
        Some(1000),
    )
    .expect("estimate request");
    let invalid_estimate = SendIntent::new(
        vec![recipient],
        FeeSelection::Estimate(FeeEstimateRequest {
            conf_target: 0,
            mode: FeeEstimateMode::Economical,
        }),
        ChangePolicy::Automatic,
        None,
        true,
        Some(1000),
    )
    .expect_err("conf_target must be positive");
    let unresolved_estimate = estimated
        .into_build_request(None)
        .expect_err("estimate request requires shell-side resolution");

    // Assert
    assert_eq!(resolved.maybe_change_descriptor_id, Some(9));
    assert_eq!(resolved.maybe_lock_time, Some(33));
    assert!(resolved.enable_rbf);
    assert_eq!(
        invalid_estimate,
        WalletError::InvalidEstimateRequest("conf_target must be at least 1".to_string())
    );
    assert_eq!(
        unresolved_estimate,
        WalletError::EstimatorUnavailable("estimate_mode requires a resolved fee rate".to_string())
    );
}

#[test]
fn rescan_progress_contract_distinguishes_fresh_partial_and_scanning() {
    // Arrange
    let fresh =
        WalletRescanState::from_progress(Some(100), Some(100), None, false).expect("fresh state");
    let partial =
        WalletRescanState::from_progress(Some(75), Some(100), None, false).expect("partial state");
    let scanning = WalletRescanState::from_progress(Some(75), Some(100), Some(76), true)
        .expect("scanning state");

    // Act
    let states = [fresh, partial, scanning];

    // Assert
    assert!(matches!(
        states[0],
        WalletRescanState::Fresh {
            scanned_through_height: 100,
            target_height: 100,
        }
    ));
    assert!(matches!(
        states[1],
        WalletRescanState::Partial {
            scanned_through_height: 75,
            target_height: 100,
        }
    ));
    assert!(matches!(
        states[2],
        WalletRescanState::Scanning {
            next_height: 76,
            target_height: 100,
        }
    ));
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
