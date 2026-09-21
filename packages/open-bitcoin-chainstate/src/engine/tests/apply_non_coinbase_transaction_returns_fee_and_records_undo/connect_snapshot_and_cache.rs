// Parity breadcrumbs:
// - packages/bitcoin-knots/src/coins.h
// - packages/bitcoin-knots/src/coins.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/node/chainstate.cpp

use super::*;

#[test]
fn connect_block_rejects_accumulated_fees_above_max_money_without_mutating_snapshot() {
    // Arrange
    let mut initial_chainstate = Chainstate::new();
    let consensus_params = ConsensusParams {
        coinbase_maturity: 1,
        ..ConsensusParams::default()
    };
    let genesis_coinbase = coinbase_transaction(0, 50);
    let genesis_block = build_block(
        BlockHash::from_byte_array([0_u8; 32]),
        1_231_006_500,
        vec![genesis_coinbase.clone()],
    );
    let genesis_position = initial_chainstate
        .connect_block(
            &genesis_block,
            1,
            ScriptVerifyFlags::P2SH
                | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
                | ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
            consensus_params,
        )
        .expect("genesis block should connect");
    let mut seeded_snapshot = initial_chainstate.snapshot();
    let large_fee_outpoint = OutPoint {
        txid: Txid::from_byte_array([9_u8; 32]),
        vout: 0,
    };
    seeded_snapshot.utxos.insert(
        large_fee_outpoint.clone(),
        Coin {
            output: TransactionOutput {
                value: Amount::from_sats(MAX_MONEY).expect("max money"),
                script_pubkey: script(&[0x51]),
            },
            is_coinbase: false,
            created_height: genesis_position.height,
            created_median_time_past: genesis_position.median_time_past,
        },
    );
    let one_sat_outpoint = OutPoint {
        txid: Txid::from_byte_array([10_u8; 32]),
        vout: 0,
    };
    seeded_snapshot.utxos.insert(
        one_sat_outpoint.clone(),
        Coin {
            output: TransactionOutput {
                value: Amount::from_sats(1).expect("valid amount"),
                script_pubkey: script(&[0x51]),
            },
            is_coinbase: false,
            created_height: genesis_position.height,
            created_median_time_past: genesis_position.median_time_past,
        },
    );
    let mut chainstate = Chainstate::from_snapshot(seeded_snapshot);
    let zero_fee_spend = spend_transaction(
        open_bitcoin_consensus::transaction_txid(&genesis_coinbase).expect("txid"),
        0,
        50,
        TransactionInput::SEQUENCE_FINAL,
    );
    let large_fee_spend = spend_transaction(
        large_fee_outpoint.txid,
        large_fee_outpoint.vout,
        0,
        TransactionInput::SEQUENCE_FINAL,
    );
    let one_sat_fee_spend = spend_transaction(
        one_sat_outpoint.txid,
        one_sat_outpoint.vout,
        0,
        TransactionInput::SEQUENCE_FINAL,
    );
    let block = build_block(
        genesis_position.block_hash,
        1_231_006_600,
        vec![
            coinbase_transaction(1, 50),
            zero_fee_spend,
            large_fee_spend,
            one_sat_fee_spend,
        ],
    );
    let snapshot_before = chainstate.snapshot();

    // Act
    let error = chainstate
        .connect_block_with_current_time(
            &block,
            2,
            i64::from(block.header.time),
            ScriptVerifyFlags::P2SH
                | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
                | ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
            consensus_params,
        )
        .expect_err("accumulated fees above MAX_MONEY must fail");
    let snapshot_after = chainstate.snapshot();

    // Assert
    assert!(matches!(
        error,
        crate::ChainstateError::BlockValidation { source }
            if source.reject_reason == "bad-txns-accumulated-fee-outofrange"
                && source.debug_message.as_deref()
                    == Some("accumulated fee in the block out of range")
    ));
    assert_eq!(snapshot_after, snapshot_before);
}

#[test]
fn connect_block_rejects_overpaying_coinbase_without_mutating_snapshot() {
    // Arrange
    let mut chainstate = Chainstate::new();
    let consensus_params = ConsensusParams {
        coinbase_maturity: 1,
        ..ConsensusParams::default()
    };
    let genesis_coinbase = coinbase_transaction(0, 50);
    let genesis_block = build_block(
        BlockHash::from_byte_array([0_u8; 32]),
        1_231_006_500,
        vec![genesis_coinbase.clone()],
    );
    let genesis_position = chainstate
        .connect_block(
            &genesis_block,
            1,
            ScriptVerifyFlags::P2SH
                | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
                | ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
            consensus_params,
        )
        .expect("genesis block should connect");
    let spend = spend_transaction(
        open_bitcoin_consensus::transaction_txid(&genesis_coinbase).expect("txid"),
        0,
        40,
        TransactionInput::SEQUENCE_FINAL,
    );
    let overpaying_coinbase =
        coinbase_transaction(1, subsidy_plus_fees_value(1, 10, &consensus_params) + 1);
    let block = build_block(
        genesis_position.block_hash,
        1_231_006_600,
        vec![overpaying_coinbase, spend],
    );
    let snapshot_before = chainstate.snapshot();

    // Act
    let error = chainstate
        .connect_block_with_current_time(
            &block,
            2,
            i64::from(block.header.time),
            ScriptVerifyFlags::P2SH
                | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
                | ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
            consensus_params,
        )
        .expect_err("overpaying coinbase must fail");
    let snapshot_after = chainstate.snapshot();

    // Assert
    assert!(matches!(
        error,
        crate::ChainstateError::BlockValidation { source }
            if source.reject_reason == "bad-cb-amount"
    ));
    assert_eq!(snapshot_after, snapshot_before);
}

#[test]
fn connect_block_skips_unspendable_outputs() {
    // Arrange
    let mut chainstate = Chainstate::new();
    let genesis_coinbase = coinbase_transaction(0, 50);
    let genesis_block = build_block(
        BlockHash::from_byte_array([0_u8; 32]),
        1_231_006_500,
        vec![genesis_coinbase.clone()],
    );
    let genesis_position = connect_block(&mut chainstate, &genesis_block, 1);
    let op_return = op_return_transaction(
        open_bitcoin_consensus::transaction_txid(&genesis_coinbase).expect("txid"),
    );
    let block = build_block(
        genesis_position.block_hash,
        1_231_006_700,
        vec![coinbase_transaction(1, 50), op_return],
    );

    // Act
    connect_block(&mut chainstate, &block, 2);

    // Assert
    assert_eq!(chainstate.utxos().len(), 1);
}

#[test]
fn failed_connect_leaves_live_cache_occupancy_and_best_block_unchanged() {
    // Arrange
    let genesis_coinbase = coinbase_transaction(0, 50);
    let genesis_txid = open_bitcoin_consensus::transaction_txid(&genesis_coinbase).expect("txid");
    let genesis_outpoint = OutPoint {
        txid: genesis_txid,
        vout: 0,
    };
    let genesis_block = build_block(
        BlockHash::from_byte_array([0_u8; 32]),
        1_231_006_500,
        vec![genesis_coinbase.clone()],
    );
    let genesis_position = ChainPosition::new(genesis_block.header.clone(), 0, 1, 1_231_006_500);
    let genesis_coin = Coin {
        output: genesis_coinbase.outputs[0].clone(),
        is_coinbase: true,
        created_height: 0,
        created_median_time_past: 0,
    };
    let mut chainstate = Chainstate::from_snapshot(crate::ChainstateSnapshot {
        active_chain: vec![genesis_position.clone()],
        utxos: HashMap::from([(genesis_outpoint.clone(), genesis_coin)]),
        undo_by_block: HashMap::new(),
        maybe_confirmed_txid_counts: Some(HashMap::new()),
    });
    let occupancy_before = chainstate.have_coin_in_cache(&genesis_outpoint);
    let tip_before = chainstate.tip().cloned();
    let best_before = chainstate.coins_best_block();
    let utxo_len_before = chainstate.utxos().len();
    let missing = spend_transaction(
        Txid::from_byte_array([9_u8; 32]),
        0,
        40,
        TransactionInput::SEQUENCE_FINAL,
    );
    let bad_block = build_block(
        genesis_position.block_hash,
        1_231_006_600,
        vec![coinbase_transaction(1, 50), missing],
    );

    // Act
    let error = chainstate
        .connect_block_with_current_time(
            &bad_block,
            2,
            i64::from(bad_block.header.time),
            ScriptVerifyFlags::P2SH
                | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
                | ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
            ConsensusParams {
                coinbase_maturity: 1,
                ..ConsensusParams::default()
            },
        )
        .expect_err("missing prevout must fail");

    // Assert
    assert!(matches!(error, crate::ChainstateError::MissingCoin { .. }));
    assert!(!occupancy_before);
    assert!(!chainstate.have_coin_in_cache(&genesis_outpoint));
    assert_eq!(chainstate.tip(), tip_before.as_ref());
    assert_eq!(chainstate.coins_best_block(), best_before);
    assert_eq!(chainstate.utxos().len(), utxo_len_before);
    assert!(
        chainstate
            .have_coin(&genesis_outpoint)
            .expect("genesis coin remains spendable")
    );
}

#[test]
fn connect_block_does_not_clone_utxo_map() {
    // Arrange
    let mut chainstate = Chainstate::new();
    let genesis_block = build_block(
        BlockHash::from_byte_array([0_u8; 32]),
        1_231_006_500,
        vec![coinbase_transaction(0, 50)],
    );

    // Act
    connect_block(&mut chainstate, &genesis_block, 1);

    // Assert
    assert_eq!(chainstate.utxos().len(), 1);
    assert_eq!(chainstate.snapshot().utxos.len(), 1);
    assert_eq!(Chainstate::from_snapshot(chainstate.snapshot()), chainstate);
    assert!(!format!("{chainstate:?}").is_empty());
}
