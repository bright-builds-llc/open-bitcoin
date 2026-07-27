// Parity breadcrumbs:
// - packages/bitcoin-knots/src/coins.h
// - packages/bitcoin-knots/src/coins.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/node/chainstate.cpp

use super::*;

#[test]
fn disconnect_tip_skips_unspendable_outputs_and_reports_missing_created_outputs() {
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
    let op_return_block = build_block(
        genesis_position.block_hash,
        1_231_006_700,
        vec![coinbase_transaction(1, 50), op_return],
    );
    let op_return_position = connect_block(&mut chainstate, &op_return_block, 2);

    let disconnected = chainstate
        .disconnect_tip(&op_return_block)
        .expect("disconnect should ignore unspendable outputs");
    assert_eq!(disconnected, op_return_position);

    let spend_block = build_block(
        genesis_position.block_hash,
        1_231_006_600,
        vec![
            coinbase_transaction(1, 50),
            spend_transaction(
                open_bitcoin_consensus::transaction_txid(&genesis_coinbase).expect("txid"),
                0,
                40,
                TransactionInput::SEQUENCE_FINAL,
            ),
        ],
    );
    let missing_created_output = Chainstate {
        active_chain: vec![ChainPosition::new(spend_block.header.clone(), 1, 2, 1)],
        utxos: HashMap::new(),
        undo_by_block: HashMap::from([(
            open_bitcoin_consensus::block_hash(&spend_block.header),
            BlockUndo {
                transactions: vec![TxUndo {
                    restored_inputs: vec![Coin {
                        output: genesis_block.transactions[0].outputs[0].clone(),
                        is_coinbase: true,
                        created_height: 0,
                        created_median_time_past: 0,
                    }],
                }],
            },
        )]),
    }
    .disconnect_tip(&spend_block)
    .expect_err("missing created spendable outputs should fail");
    assert!(matches!(
        missing_created_output,
        crate::ChainstateError::DisconnectSpentOutputMismatch { .. }
    ));
}

#[test]
fn restore_non_coinbase_inputs_rejects_undo_shape_mismatch() {
    // Arrange
    let transaction = spend_transaction(
        Txid::from_byte_array([9_u8; 32]),
        0,
        40,
        TransactionInput::SEQUENCE_FINAL,
    );
    let mut utxos = HashMap::new();

    // Act
    let error = restore_non_coinbase_inputs(&mut utxos, &transaction, &TxUndo::default())
        .expect_err("missing restored inputs should fail");

    // Assert
    assert!(matches!(
        error,
        crate::ChainstateError::UndoMismatch {
            expected_transactions: 1,
            actual_transactions: 0,
        }
    ));
}

#[test]
fn median_time_past_uses_the_last_window_of_times() {
    // Arrange
    let positions = (0..12_u32)
        .map(|index| {
            ChainPosition::new(
                BlockHeader {
                    version: 1,
                    previous_block_hash: BlockHash::from_byte_array([index as u8; 32]),
                    merkle_root: Default::default(),
                    time: index + 10,
                    bits: EASY_BITS,
                    nonce: 0,
                },
                index,
                u128::from(index),
                i64::from(index + 10),
            )
        })
        .collect::<Vec<_>>();

    // Act
    let median = compute_median_time_past(&positions, None);

    // Assert
    assert_eq!(median, 16);
}

#[test]
fn median_time_past_returns_zero_for_an_empty_chain() {
    assert_eq!(compute_median_time_past(&[], None), 0);
}

#[test]
fn snapshot_round_trip_preserves_accessors() {
    let mut chainstate = Chainstate::new();
    let genesis_block = build_block(
        BlockHash::from_byte_array([0_u8; 32]),
        1_231_006_500,
        vec![coinbase_transaction(0, 50)],
    );
    let genesis_position = connect_block(&mut chainstate, &genesis_block, 1);

    let snapshot = chainstate.snapshot();
    let restored = Chainstate::from_snapshot(snapshot.clone());

    assert_eq!(snapshot.tip(), Some(&genesis_position));
    assert_eq!(restored.tip(), Some(&genesis_position));
    assert_eq!(restored.utxos(), chainstate.utxos());
}

#[test]
fn connect_block_rejects_invalid_tip_extensions() {
    let mut chainstate = Chainstate::new();
    let block = build_block(
        BlockHash::from_byte_array([1_u8; 32]),
        1_231_006_500,
        vec![coinbase_transaction(0, 50)],
    );

    let error = chainstate
        .connect_block(
            &block,
            1,
            ScriptVerifyFlags::P2SH,
            ConsensusParams::default(),
        )
        .expect_err("wrong parent hash must fail");

    assert!(matches!(
        error,
        crate::ChainstateError::InvalidTipExtension { .. }
    ));
}

#[test]
fn disconnect_tip_rejects_missing_tip_and_missing_undo() {
    let mut empty = Chainstate::new();
    let genesis_block = build_block(
        BlockHash::from_byte_array([0_u8; 32]),
        1_231_006_500,
        vec![coinbase_transaction(0, 50)],
    );
    let missing_tip = empty
        .disconnect_tip(&genesis_block)
        .expect_err("empty chain should reject disconnect");
    assert!(matches!(missing_tip, crate::ChainstateError::MissingTip));

    let tip = ChainPosition::new(genesis_block.header.clone(), 0, 1, 1);
    let mut chainstate = Chainstate {
        active_chain: vec![tip.clone()],
        utxos: HashMap::new(),
        undo_by_block: HashMap::new(),
    };
    let missing_undo = chainstate
        .disconnect_tip(&genesis_block)
        .expect_err("missing undo should fail");

    assert!(matches!(
        missing_undo,
        crate::ChainstateError::MissingUndo { block_hash } if block_hash == tip.block_hash
    ));
}

#[test]
fn disconnect_tip_detects_mismatches_and_corrupt_undo_shapes() {
    let genesis_coinbase = coinbase_transaction(0, 50);
    let genesis_block = build_block(
        BlockHash::from_byte_array([0_u8; 32]),
        1_231_006_500,
        vec![genesis_coinbase.clone()],
    );
    let spend = spend_transaction(
        open_bitcoin_consensus::transaction_txid(&genesis_coinbase).expect("txid"),
        0,
        40,
        TransactionInput::SEQUENCE_FINAL,
    );
    let block = build_block(
        open_bitcoin_consensus::block_hash(&genesis_block.header),
        1_231_006_600,
        vec![coinbase_transaction(1, 50), spend],
    );
    let tip = ChainPosition::new(block.header.clone(), 1, 2, 1);

    let mismatch = Chainstate {
        active_chain: vec![tip.clone()],
        utxos: HashMap::new(),
        undo_by_block: HashMap::new(),
    }
    .disconnect_tip(&genesis_block)
    .expect_err("wrong block should fail");
    assert!(matches!(
        mismatch,
        crate::ChainstateError::DisconnectBlockMismatch { .. }
    ));

    let undo_shape = Chainstate {
        active_chain: vec![tip.clone()],
        utxos: HashMap::new(),
        undo_by_block: HashMap::from([(tip.block_hash, BlockUndo::default())]),
    }
    .disconnect_tip(&block)
    .expect_err("corrupt top-level undo shape should fail");
    assert!(matches!(
        undo_shape,
        crate::ChainstateError::UndoMismatch { .. }
    ));

    let inner_undo_shape = Chainstate {
        active_chain: vec![tip.clone()],
        utxos: HashMap::from([
            (
                OutPoint {
                    txid: open_bitcoin_consensus::transaction_txid(&block.transactions[0])
                        .expect("txid"),
                    vout: 0,
                },
                Coin {
                    output: block.transactions[0].outputs[0].clone(),
                    is_coinbase: true,
                    created_height: 1,
                    created_median_time_past: 1,
                },
            ),
            (
                OutPoint {
                    txid: open_bitcoin_consensus::transaction_txid(&block.transactions[1])
                        .expect("txid"),
                    vout: 0,
                },
                Coin {
                    output: block.transactions[1].outputs[0].clone(),
                    is_coinbase: false,
                    created_height: 1,
                    created_median_time_past: 1,
                },
            ),
        ]),
        undo_by_block: HashMap::from([(
            tip.block_hash,
            BlockUndo {
                transactions: vec![TxUndo::default()],
            },
        )]),
    }
    .disconnect_tip(&block)
    .expect_err("corrupt inner undo shape should fail");
    assert!(matches!(
        inner_undo_shape,
        crate::ChainstateError::UndoMismatch { .. }
    ));
}

#[test]
fn disconnect_tip_detects_restore_and_output_integrity_failures() {
    let genesis_coinbase = coinbase_transaction(0, 50);
    let genesis_block = build_block(
        BlockHash::from_byte_array([0_u8; 32]),
        1_231_006_500,
        vec![genesis_coinbase.clone()],
    );
    let spend = spend_transaction(
        open_bitcoin_consensus::transaction_txid(&genesis_coinbase).expect("txid"),
        0,
        40,
        TransactionInput::SEQUENCE_FINAL,
    );
    let block = build_block(
        open_bitcoin_consensus::block_hash(&genesis_block.header),
        1_231_006_600,
        vec![coinbase_transaction(1, 50), spend.clone()],
    );
    let tip = ChainPosition::new(block.header.clone(), 1, 2, 1);
    let spend_outpoint = spend.inputs[0].previous_output.clone();
    let created_coinbase_outpoint = OutPoint {
        txid: open_bitcoin_consensus::transaction_txid(&block.transactions[0]).expect("txid"),
        vout: 0,
    };
    let created_spend_outpoint = OutPoint {
        txid: open_bitcoin_consensus::transaction_txid(&block.transactions[1]).expect("txid"),
        vout: 0,
    };

    let restore_overwrite = Chainstate {
        active_chain: vec![tip.clone()],
        utxos: HashMap::from([
            (
                created_coinbase_outpoint.clone(),
                Coin {
                    output: block.transactions[0].outputs[0].clone(),
                    is_coinbase: true,
                    created_height: 1,
                    created_median_time_past: 1,
                },
            ),
            (
                created_spend_outpoint.clone(),
                Coin {
                    output: block.transactions[1].outputs[0].clone(),
                    is_coinbase: false,
                    created_height: 1,
                    created_median_time_past: 1,
                },
            ),
            (
                spend_outpoint.clone(),
                Coin {
                    output: block.transactions[1].outputs[0].clone(),
                    is_coinbase: false,
                    created_height: 0,
                    created_median_time_past: 0,
                },
            ),
        ]),
        undo_by_block: HashMap::from([(
            tip.block_hash,
            BlockUndo {
                transactions: vec![TxUndo {
                    restored_inputs: vec![Coin {
                        output: genesis_block.transactions[0].outputs[0].clone(),
                        is_coinbase: true,
                        created_height: 0,
                        created_median_time_past: 0,
                    }],
                }],
            },
        )]),
    }
    .disconnect_tip(&block)
    .expect_err("restoring into an occupied outpoint should fail");
    assert!(matches!(
        restore_overwrite,
        crate::ChainstateError::RestoredCoinOverwrite { .. }
    ));

    let mismatch_block = build_block(
        open_bitcoin_consensus::block_hash(&genesis_block.header),
        1_231_006_600,
        vec![coinbase_transaction(1, 50)],
    );
    let mismatch_tip = ChainPosition::new(mismatch_block.header.clone(), 1, 2, 1);
    let mismatch_coinbase_outpoint = OutPoint {
        txid: open_bitcoin_consensus::transaction_txid(&mismatch_block.transactions[0])
            .expect("txid"),
        vout: 0,
    };
    let output_mismatch = Chainstate {
        active_chain: vec![mismatch_tip],
        utxos: HashMap::from([(
            mismatch_coinbase_outpoint,
            Coin {
                output: mismatch_block.transactions[0].outputs[0].clone(),
                is_coinbase: true,
                created_height: 999,
                created_median_time_past: 1,
            },
        )]),
        undo_by_block: HashMap::from([(
            open_bitcoin_consensus::block_hash(&mismatch_block.header),
            BlockUndo::default(),
        )]),
    }
    .disconnect_tip(&mismatch_block)
    .expect_err("mismatched created output metadata should fail");
    assert!(matches!(
        output_mismatch,
        crate::ChainstateError::DisconnectSpentOutputMismatch { .. }
    ));
}

#[test]
fn reorg_and_tip_preference_cover_remaining_decision_branches() {
    let candidate_same_work_higher_height = ChainPosition::new(
        BlockHeader {
            version: 1,
            previous_block_hash: BlockHash::from_byte_array([0_u8; 32]),
            merkle_root: Default::default(),
            time: 2,
            bits: EASY_BITS,
            nonce: 0,
        },
        2,
        5,
        2,
    );
    let current_same_work_lower_height = ChainPosition::new(
        BlockHeader {
            version: 1,
            previous_block_hash: BlockHash::from_byte_array([0_u8; 32]),
            merkle_root: Default::default(),
            time: 1,
            bits: EASY_BITS,
            nonce: 0,
        },
        1,
        5,
        1,
    );
    assert!(prefer_candidate_tip(
        &current_same_work_lower_height,
        &candidate_same_work_higher_height,
    ));

    let current_same_height = ChainPosition::new(
        BlockHeader {
            version: 1,
            previous_block_hash: BlockHash::from_byte_array([0_u8; 32]),
            merkle_root: Default::default(),
            time: 3,
            bits: EASY_BITS,
            nonce: 0,
        },
        2,
        5,
        3,
    );
    let candidate_same_height = ChainPosition::new(
        BlockHeader {
            version: 1,
            previous_block_hash: BlockHash::from_byte_array([1_u8; 32]),
            merkle_root: Default::default(),
            time: 4,
            bits: EASY_BITS,
            nonce: 0,
        },
        2,
        5,
        4,
    );
    assert_eq!(
        prefer_candidate_tip(&current_same_height, &candidate_same_height),
        candidate_same_height.block_hash > current_same_height.block_hash
    );

    let mut empty = Chainstate::new();
    let error = empty
        .reorg(
            &[build_block(
                BlockHash::from_byte_array([0_u8; 32]),
                1_231_006_500,
                vec![coinbase_transaction(0, 50)],
            )],
            &[],
            ScriptVerifyFlags::P2SH,
            ConsensusParams::default(),
        )
        .expect_err("cannot disconnect past genesis");
    assert!(matches!(
        error,
        crate::ChainstateError::DisconnectPastGenesis { .. }
    ));
}
