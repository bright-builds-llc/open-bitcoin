use super::*;

#[test]
fn apply_non_coinbase_transaction_returns_fee_and_records_undo() {
    // Arrange
    let genesis_coinbase = coinbase_transaction(0, 50);
    let spent_txid = open_bitcoin_consensus::transaction_txid(&genesis_coinbase).expect("txid");
    let spent_outpoint = OutPoint {
        txid: spent_txid,
        vout: 0,
    };
    let spent_coin = Coin {
        output: genesis_coinbase.outputs[0].clone(),
        is_coinbase: true,
        created_height: 0,
        created_median_time_past: 0,
    };
    let transaction = spend_transaction(spent_txid, 0, 40, TransactionInput::SEQUENCE_FINAL);
    let mut next_utxos = HashMap::from([(spent_outpoint.clone(), spent_coin.clone())]);
    let mut block_undo = BlockUndo::default();

    // Act
    let fee = apply_non_coinbase_transaction(
        &mut next_utxos,
        &mut block_undo,
        &transaction,
        1_231_006_600,
        ScriptVerifyFlags::P2SH
            | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
            | ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
        &BlockValidationContext {
            height: 1,
            previous_header: BlockHeader::default(),
            maybe_retarget_anchor: None,
            maybe_min_difficulty_recovery_target: None,
            previous_median_time_past: 0,
            current_time: 1_231_006_600,
            consensus_params: ConsensusParams {
                coinbase_maturity: 1,
                ..ConsensusParams::default()
            },
        },
    )
    .expect("non-coinbase helper should apply cleanly");

    // Assert
    assert_eq!(fee, Amount::from_sats(10).expect("valid fee"));
    assert!(!next_utxos.contains_key(&spent_outpoint));
    assert_eq!(
        block_undo.transactions,
        vec![TxUndo {
            restored_inputs: vec![spent_coin],
        }]
    );
}

#[test]
fn chainstate_helper_error_paths_return_typed_failures() {
    // Arrange
    let missing_outpoint = OutPoint {
        txid: Txid::from_byte_array([7_u8; 32]),
        vout: 0,
    };
    let transaction = spend_transaction(
        missing_outpoint.txid,
        missing_outpoint.vout,
        40,
        TransactionInput::SEQUENCE_FINAL,
    );
    let mut empty_utxos = HashMap::new();

    // Act
    let remove_error = remove_spent_input(&mut empty_utxos, &transaction.inputs[0])
        .expect_err("missing spent input should fail");
    let context_error = build_transaction_context(
        &transaction,
        &HashMap::new(),
        1,
        1_231_006_600,
        0,
        ScriptVerifyFlags::P2SH,
        ConsensusParams::default(),
    )
    .expect_err("missing context input should fail");
    let serialization_error = txid_serialization_error("encoded txid failure");

    // Assert
    assert_eq!(
        remove_error,
        crate::ChainstateError::MissingCoin {
            outpoint: missing_outpoint.clone(),
        }
    );
    assert_eq!(
        context_error,
        crate::ChainstateError::MissingCoin {
            outpoint: missing_outpoint,
        }
    );
    assert!(matches!(
        serialization_error,
        crate::ChainstateError::Serialization {
            context: "txid derivation",
            ..
        }
    ));
}

#[test]
fn connect_block_maps_coinbase_overpay_to_block_validation_error() {
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

    // Assert
    assert!(matches!(
        error,
        crate::ChainstateError::BlockValidation { source }
            if source.reject_reason == "bad-cb-amount"
    ));
}

#[test]
fn accumulated_fee_out_of_range_maps_to_block_validation_error() {
    // Act
    let error = accumulated_fee_out_of_range();

    // Assert
    assert!(matches!(
        error,
        crate::ChainstateError::BlockValidation { source }
            if source.reject_reason == "bad-txns-accumulated-fee-outofrange"
                && source.debug_message.as_deref()
                    == Some("accumulated fee in the block out of range")
    ));
}

#[test]
fn connect_block_accepts_exact_coinbase_reward_limit() {
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
    let exact_limit_coinbase =
        coinbase_transaction(1, subsidy_plus_fees_value(1, 10, &consensus_params));
    let block = build_block(
        genesis_position.block_hash,
        1_231_006_600,
        vec![exact_limit_coinbase, spend],
    );

    // Act
    let position = chainstate
        .connect_block_with_current_time(
            &block,
            2,
            i64::from(block.header.time),
            ScriptVerifyFlags::P2SH
                | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
                | ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
            consensus_params,
        )
        .expect("exact reward limit should connect");

    // Assert
    assert_eq!(position.height, 1);
    assert_eq!(chainstate.tip(), Some(&position));
}

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
