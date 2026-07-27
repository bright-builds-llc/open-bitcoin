// Parity breadcrumbs:
// - packages/bitcoin-knots/src/wallet/wallet.cpp
// - packages/bitcoin-knots/src/wallet/spend.cpp
// - packages/bitcoin-knots/src/wallet/coinselection.cpp
// - packages/bitcoin-knots/src/wallet/transaction.cpp
// - packages/bitcoin-knots/test/functional/wallet_descriptor.py
// - packages/bitcoin-knots/test/functional/feature_segwit.py

use super::*;

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
fn send_intent_reports_missing_recipients_and_invalid_ceiling_inputs() {
    // Arrange / Act
    let no_recipients = super::SendIntent::new(
        Vec::new(),
        super::FeeSelection::Explicit(open_bitcoin_mempool::FeeRate::from_sats_per_kvb(1_000)),
        super::ChangePolicy::Automatic,
        None,
        false,
        None,
    )
    .expect_err("at least one recipient is required");
    let invalid_ceiling = super::SendIntent::new(
        vec![Recipient {
            script_pubkey: script(&[0x51]),
            value: Amount::from_sats(1_000).expect("amount"),
        }],
        super::FeeSelection::Explicit(open_bitcoin_mempool::FeeRate::from_sats_per_kvb(1_000)),
        super::ChangePolicy::Automatic,
        None,
        false,
        Some(0),
    )
    .expect_err("non-positive fee ceilings are invalid");

    // Assert
    assert_eq!(no_recipients, WalletError::NoRecipients);
    assert_eq!(
        invalid_ceiling,
        WalletError::FeeCeilingExceeded {
            fee_sats: 0,
            ceiling_sats: 0
        }
    );
}

#[test]
fn send_intent_into_build_request_covers_estimator_and_change_policy_branches() {
    // Arrange
    let recipient = Recipient {
        script_pubkey: script(&[0x51]),
        value: Amount::from_sats(1_000).expect("amount"),
    };
    let estimated = super::SendIntent::new(
        vec![recipient.clone()],
        super::FeeSelection::Estimate(super::FeeEstimateRequest {
            conf_target: 6,
            mode: super::FeeEstimateMode::Economical,
        }),
        super::ChangePolicy::FixedDescriptor(7),
        Some(3),
        true,
        None,
    )
    .expect("estimated intent");
    let forbidden = super::SendIntent::new(
        vec![recipient],
        super::FeeSelection::Explicit(open_bitcoin_mempool::FeeRate::from_sats_per_kvb(1_500)),
        super::ChangePolicy::ChangeForbidden,
        None,
        false,
        None,
    )
    .expect("forbidden intent");
    let automatic = super::SendIntent::new(
        vec![Recipient {
            script_pubkey: script(&[0x51]),
            value: Amount::from_sats(2_000).expect("amount"),
        }],
        super::FeeSelection::Explicit(open_bitcoin_mempool::FeeRate::from_sats_per_kvb(1_750)),
        super::ChangePolicy::Automatic,
        None,
        false,
        None,
    )
    .expect("automatic intent");

    // Act
    let unresolved = estimated
        .into_build_request(None)
        .expect_err("estimate intents need a resolved fee rate");
    let resolved = estimated
        .into_build_request(Some(open_bitcoin_mempool::FeeRate::from_sats_per_kvb(
            2_000,
        )))
        .expect("resolved build request");
    let forbidden_request = forbidden
        .into_build_request(None)
        .expect("explicit fee rate does not need estimator");
    let automatic_request = automatic
        .into_build_request(None)
        .expect("automatic explicit request");

    // Assert
    assert_eq!(
        unresolved,
        WalletError::EstimatorUnavailable("estimate_mode requires a resolved fee rate".to_string())
    );
    assert_eq!(resolved.maybe_change_descriptor_id, Some(7));
    assert_eq!(
        resolved.fee_rate,
        open_bitcoin_mempool::FeeRate::from_sats_per_kvb(2_000)
    );
    assert_eq!(forbidden_request.maybe_change_descriptor_id, None);
    assert_eq!(automatic_request.maybe_change_descriptor_id, None);
}

#[test]
fn address_allocation_reports_missing_role_as_unsupported() {
    // Arrange
    let mut wallet = Wallet::new(AddressNetwork::Regtest);

    // Act
    let receive_error = wallet
        .allocate_receive_address()
        .expect_err("missing external role should be explicit");
    let change_error = wallet
        .allocate_change_address()
        .expect_err("missing internal role should be explicit");

    // Assert
    assert_eq!(
        receive_error,
        WalletError::UnsupportedAddressRole("external".to_string())
    );
    assert_eq!(
        change_error,
        WalletError::UnsupportedAddressRole("internal".to_string())
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
