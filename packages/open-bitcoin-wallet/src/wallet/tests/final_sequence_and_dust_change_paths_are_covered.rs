// Parity breadcrumbs:
// - packages/bitcoin-knots/src/wallet/wallet.cpp
// - packages/bitcoin-knots/src/wallet/spend.cpp
// - packages/bitcoin-knots/src/wallet/coinselection.cpp
// - packages/bitcoin-knots/src/wallet/transaction.cpp
// - packages/bitcoin-knots/test/functional/wallet_descriptor.py
// - packages/bitcoin-knots/test/functional/feature_segwit.py

use super::*;

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
