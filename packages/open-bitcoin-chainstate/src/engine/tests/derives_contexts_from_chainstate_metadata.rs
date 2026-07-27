use super::*;

#[test]
fn derives_contexts_from_chainstate_metadata() {
    // Arrange
    let mut chainstate = Chainstate::new();
    let genesis_coinbase = coinbase_transaction(0, 50);
    let genesis_block = build_block(
        BlockHash::from_byte_array([0_u8; 32]),
        1_231_006_500,
        vec![genesis_coinbase.clone()],
    );
    let genesis_position = connect_block(&mut chainstate, &genesis_block, 1);
    let spend = spend_transaction(
        Txid::from_byte_array(
            open_bitcoin_consensus::transaction_txid(&genesis_coinbase)
                .expect("txid")
                .to_byte_array(),
        ),
        0,
        40,
        1,
    );
    let block = build_block(
        genesis_position.block_hash,
        1_231_006_600,
        vec![coinbase_transaction(1, 50), spend],
    );

    // Act
    let next_position = connect_block(&mut chainstate, &block, 2);

    // Assert
    assert_eq!(next_position.height, 1);
    let spendable = chainstate
        .utxos()
        .values()
        .find(|coin| !coin.is_coinbase)
        .expect("expected transaction output to be added");
    assert_eq!(spendable.created_height, 1);
    assert_eq!(
        spendable.created_median_time_past,
        genesis_position.median_time_past
    );
}

#[test]
fn connect_and_disconnect_round_trip_utxos_and_tip() {
    // Arrange
    let mut chainstate = Chainstate::new();
    let genesis_coinbase = coinbase_transaction(0, 50);
    let genesis_block = build_block(
        BlockHash::from_byte_array([0_u8; 32]),
        1_231_006_500,
        vec![genesis_coinbase.clone()],
    );
    let genesis_position = connect_block(&mut chainstate, &genesis_block, 1);
    let spend = spend_transaction(
        open_bitcoin_consensus::transaction_txid(&genesis_coinbase).expect("txid"),
        0,
        40,
        TransactionInput::SEQUENCE_FINAL,
    );
    let block = build_block(
        genesis_position.block_hash,
        1_231_006_600,
        vec![coinbase_transaction(1, 50), spend],
    );
    let connected_position = connect_block(&mut chainstate, &block, 2);

    // Act
    let disconnected = chainstate
        .disconnect_tip(&block)
        .expect("block should disconnect cleanly");

    // Assert
    assert_eq!(disconnected, connected_position);
    assert_active_tip(&chainstate, &genesis_position);
    assert_eq!(chainstate.utxos().len(), 1);
}

#[test]
fn reorg_prefers_heavier_branch_and_preserves_expected_utxos() {
    // Arrange
    let mut chainstate = Chainstate::new();
    let genesis_coinbase = coinbase_transaction(0, 50);
    let genesis_block = build_block(
        BlockHash::from_byte_array([0_u8; 32]),
        1_231_006_500,
        vec![genesis_coinbase.clone()],
    );
    let genesis_position = connect_block(&mut chainstate, &genesis_block, 1);

    let branch_a_coinbase = coinbase_transaction(1, 50);
    let branch_a = build_block(
        genesis_position.block_hash,
        1_231_006_600,
        vec![branch_a_coinbase.clone()],
    );
    let branch_a_position = connect_block(&mut chainstate, &branch_a, 2);

    let branch_b_spend = spend_transaction(
        open_bitcoin_consensus::transaction_txid(&genesis_coinbase).expect("txid"),
        0,
        30,
        TransactionInput::SEQUENCE_FINAL,
    );
    let branch_b = build_block(
        genesis_position.block_hash,
        1_231_006_650,
        vec![coinbase_transaction(1, 50), branch_b_spend],
    );
    let branch_b_tip = ChainPosition::new(branch_b.header.clone(), 1, 3, 1_231_006_650);
    assert!(prefer_candidate_tip(&branch_a_position, &branch_b_tip));

    // Act
    let transition = chainstate
        .reorg(
            std::slice::from_ref(&branch_a),
            &[AnchoredBlock {
                block: branch_b.clone(),
                chain_work: 3,
            }],
            ScriptVerifyFlags::P2SH
                | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
                | ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
            ConsensusParams {
                coinbase_maturity: 1,
                ..ConsensusParams::default()
            },
        )
        .expect("reorg should succeed");

    // Assert
    assert_eq!(transition.disconnected, vec![branch_a_position]);
    assert_eq!(transition.connected.len(), 1);
    assert_eq!(chainstate.tip(), Some(&transition.connected[0]));
    assert_eq!(chainstate.utxos().len(), 2);
}

#[test]
fn connect_block_rejects_premature_coinbase_spend() {
    // Arrange
    let mut chainstate = Chainstate::new();
    let genesis_coinbase = coinbase_transaction(0, 50);
    let genesis_block = build_block(
        BlockHash::from_byte_array([0_u8; 32]),
        1_231_006_500,
        vec![genesis_coinbase.clone()],
    );
    let genesis_position = connect_block(&mut chainstate, &genesis_block, 1);
    let premature_spend = spend_transaction(
        open_bitcoin_consensus::transaction_txid(&genesis_coinbase).expect("txid"),
        0,
        40,
        TransactionInput::SEQUENCE_FINAL,
    );
    let block = build_block(
        genesis_position.block_hash,
        1_231_006_600,
        vec![coinbase_transaction(1, 50), premature_spend],
    );

    // Act
    let error = chainstate
        .connect_block(
            &block,
            2,
            ScriptVerifyFlags::P2SH
                | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
                | ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
            ConsensusParams::default(),
        )
        .expect_err("premature coinbase spend must fail");

    // Assert
    assert!(matches!(
        error,
        crate::ChainstateError::TransactionValidation { .. }
    ));
}

#[test]
fn connect_block_rejects_missing_prevouts_from_chainstate() {
    let mut chainstate = Chainstate::new();
    let genesis_block = build_block(
        BlockHash::from_byte_array([0_u8; 32]),
        1_231_006_500,
        vec![coinbase_transaction(0, 50)],
    );
    connect_block(&mut chainstate, &genesis_block, 1);

    let missing_prevout = spend_transaction(
        Txid::from_byte_array([4_u8; 32]),
        0,
        40,
        TransactionInput::SEQUENCE_FINAL,
    );
    let block = build_block(
        open_bitcoin_consensus::block_hash(&genesis_block.header),
        1_231_006_600,
        vec![coinbase_transaction(1, 50), missing_prevout],
    );

    let error = chainstate
        .connect_block(
            &block,
            2,
            ScriptVerifyFlags::P2SH,
            ConsensusParams {
                coinbase_maturity: 1,
                ..ConsensusParams::default()
            },
        )
        .expect_err("missing prevout must fail before mutation");

    assert!(matches!(error, crate::ChainstateError::MissingCoin { .. }));
}

#[test]
fn connect_block_uses_explicit_current_time_for_future_time_rejection() {
    // Arrange
    let mut chainstate = Chainstate::new();
    let genesis_block = build_block(
        BlockHash::from_byte_array([0_u8; 32]),
        1_231_006_500,
        vec![coinbase_transaction(0, 50)],
    );
    connect_block(&mut chainstate, &genesis_block, 1);
    let future_block = build_block(
        open_bitcoin_consensus::block_hash(&genesis_block.header),
        1_231_016_500,
        vec![coinbase_transaction(1, 50)],
    );

    // Act
    let error = chainstate
        .connect_block_with_current_time(
            &future_block,
            2,
            i64::from(future_block.header.time) - 7_201,
            ScriptVerifyFlags::P2SH,
            ConsensusParams {
                coinbase_maturity: 1,
                ..ConsensusParams::default()
            },
        )
        .expect_err("future block must respect the caller-provided time");

    // Assert
    assert!(matches!(
        error,
        crate::ChainstateError::BlockValidation { source }
            if source.reject_reason == "time-too-new"
    ));
}

#[test]
fn connect_block_rejects_wrong_bits_at_retarget_boundary() {
    // Arrange
    let mut chainstate = Chainstate::new();
    let consensus_params = ConsensusParams {
        coinbase_maturity: 1,
        allow_min_difficulty_blocks: false,
        no_pow_retargeting: false,
        pow_target_spacing_seconds: 10,
        pow_target_timespan_seconds: 20,
        ..ConsensusParams::default()
    };
    let genesis_block = build_block(
        BlockHash::from_byte_array([0_u8; 32]),
        100,
        vec![coinbase_transaction(0, 50)],
    );
    connect_block(&mut chainstate, &genesis_block, 1);
    let height_one_block = build_block(
        open_bitcoin_consensus::block_hash(&genesis_block.header),
        110,
        vec![coinbase_transaction(1, 50)],
    );
    let height_one_position = chainstate
        .connect_block(
            &height_one_block,
            2,
            ScriptVerifyFlags::P2SH,
            consensus_params,
        )
        .expect("height-one block should connect");
    let wrong_bits_block = build_block(
        height_one_position.block_hash,
        120,
        vec![coinbase_transaction(2, 50)],
    );

    // Act
    let error = chainstate
        .connect_block(
            &wrong_bits_block,
            3,
            ScriptVerifyFlags::P2SH,
            consensus_params,
        )
        .expect_err("stale retarget-boundary bits must fail");

    // Assert
    assert!(matches!(
        error,
        crate::ChainstateError::BlockValidation { source }
            if source.reject_reason == "bad-diffbits"
                && source.debug_message.as_deref() == Some("incorrect proof of work")
    ));
}

#[test]
fn connect_block_recovers_last_non_special_target_after_special_min_difficulty_block() {
    // Arrange
    let mut chainstate = Chainstate::new();
    let consensus_params = ConsensusParams {
        coinbase_maturity: 1,
        allow_min_difficulty_blocks: true,
        no_pow_retargeting: false,
        pow_target_spacing_seconds: 10,
        pow_target_timespan_seconds: 40,
        ..ConsensusParams::default()
    };
    let recovered_bits = 0x205f_ffff;
    let genesis_block = build_block_with_bits(
        BlockHash::from_byte_array([0_u8; 32]),
        100,
        consensus_params.pow_limit_bits,
        vec![coinbase_transaction(0, 50)],
    );
    let genesis_position = chainstate
        .connect_block(&genesis_block, 1, ScriptVerifyFlags::P2SH, consensus_params)
        .expect("genesis block should connect");
    let on_time_block = build_block_with_bits(
        genesis_position.block_hash,
        110,
        consensus_params.pow_limit_bits,
        vec![coinbase_transaction(1, 50)],
    );
    let on_time_position = chainstate
        .connect_block(&on_time_block, 2, ScriptVerifyFlags::P2SH, consensus_params)
        .expect("non-special block should connect");
    let second_on_time_block = build_block_with_bits(
        on_time_position.block_hash,
        120,
        consensus_params.pow_limit_bits,
        vec![coinbase_transaction(2, 50)],
    );
    let second_on_time_position = chainstate
        .connect_block(
            &second_on_time_block,
            3,
            ScriptVerifyFlags::P2SH,
            consensus_params,
        )
        .expect("second non-special block should connect");
    let third_on_time_block = build_block_with_bits(
        second_on_time_position.block_hash,
        130,
        consensus_params.pow_limit_bits,
        vec![coinbase_transaction(3, 50)],
    );
    let third_on_time_position = chainstate
        .connect_block(
            &third_on_time_block,
            4,
            ScriptVerifyFlags::P2SH,
            consensus_params,
        )
        .expect("third non-special block should connect");
    let boundary_block = build_block_with_bits(
        third_on_time_position.block_hash,
        140,
        recovered_bits,
        vec![coinbase_transaction(4, 50)],
    );
    let boundary_position = chainstate
        .connect_block(
            &boundary_block,
            5,
            ScriptVerifyFlags::P2SH,
            consensus_params,
        )
        .expect("boundary block should connect");
    let special_block = build_block_with_bits(
        boundary_position.block_hash,
        161,
        consensus_params.pow_limit_bits,
        vec![coinbase_transaction(5, 50)],
    );
    let special_position = chainstate
        .connect_block(&special_block, 6, ScriptVerifyFlags::P2SH, consensus_params)
        .expect("late special block should connect");
    let wrong_bits_block = build_block_with_bits(
        special_position.block_hash,
        170,
        consensus_params.pow_limit_bits,
        vec![coinbase_transaction(6, 50)],
    );
    let recovered_bits_block = build_block_with_bits(
        special_position.block_hash,
        170,
        recovered_bits,
        vec![coinbase_transaction(6, 50)],
    );

    // Act
    let error = chainstate
        .connect_block(
            &wrong_bits_block,
            7,
            ScriptVerifyFlags::P2SH,
            consensus_params,
        )
        .expect_err("previous special bits must be rejected after recovery");
    let recovered_position = chainstate
        .connect_block(
            &recovered_bits_block,
            7,
            ScriptVerifyFlags::P2SH,
            consensus_params,
        )
        .expect("last non-special target should be accepted");

    // Assert
    assert!(matches!(
        error,
        crate::ChainstateError::BlockValidation { source }
            if source.reject_reason == "bad-diffbits"
                && source.debug_message.as_deref() == Some("incorrect proof of work")
    ));
    assert_eq!(recovered_position.height, 6);
    assert_eq!(recovered_position.header.bits, recovered_bits);
}

#[test]
fn difficulty_interval_helper_clamps_non_positive_spacing() {
    let interval = difficulty_adjustment_interval(&ConsensusParams {
        pow_target_spacing_seconds: 0,
        ..ConsensusParams::default()
    });

    assert_eq!(interval, 1);
}
