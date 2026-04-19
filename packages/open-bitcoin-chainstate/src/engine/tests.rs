use std::collections::HashMap;

use open_bitcoin_consensus::{
    BlockValidationContext, ConsensusParams, ScriptVerifyFlags, block_merkle_root,
    check_block_header,
};
use open_bitcoin_primitives::{
    Amount, Block, BlockHash, BlockHeader, OutPoint, ScriptBuf, ScriptWitness, Transaction,
    TransactionInput, TransactionOutput, Txid,
};

use super::{
    Chainstate, apply_non_coinbase_transaction, compute_median_time_past, prefer_candidate_tip,
    restore_non_coinbase_inputs,
};
use crate::{AnchoredBlock, BlockUndo, ChainPosition, Coin, TxUndo};

const EASY_BITS: u32 = 0x207f_ffff;

fn script(bytes: &[u8]) -> ScriptBuf {
    ScriptBuf::from_bytes(bytes.to_vec()).expect("valid script")
}

fn serialized_script_num(value: i64) -> Vec<u8> {
    if value == 0 {
        return vec![0x00];
    }

    let negative = value < 0;
    let mut magnitude = value.unsigned_abs();
    let mut encoded = Vec::new();
    while magnitude > 0 {
        encoded.push((magnitude & 0xff) as u8);
        magnitude >>= 8;
    }

    if encoded.last().is_some_and(|byte| (byte & 0x80) != 0) {
        encoded.push(if negative { 0x80 } else { 0x00 });
    } else if negative {
        let last = encoded.last_mut().expect("value is non-zero");
        *last |= 0x80;
    }

    let mut script = Vec::with_capacity(encoded.len() + 1);
    script.push(encoded.len() as u8);
    script.extend(encoded);
    script
}

fn coinbase_transaction(height: u32, value: i64) -> Transaction {
    let mut script_sig = serialized_script_num(i64::from(height));
    script_sig.push(0x51);
    Transaction {
        version: 1,
        inputs: vec![TransactionInput {
            previous_output: OutPoint::null(),
            script_sig: script(&script_sig),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(value).expect("valid amount"),
            script_pubkey: script(&[0x51]),
        }],
        lock_time: 0,
    }
}

fn spend_transaction(
    previous_txid: Txid,
    previous_vout: u32,
    value: i64,
    sequence: u32,
) -> Transaction {
    Transaction {
        version: 2,
        inputs: vec![TransactionInput {
            previous_output: OutPoint {
                txid: previous_txid,
                vout: previous_vout,
            },
            script_sig: script(&[0x51]),
            sequence,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(value).expect("valid amount"),
            script_pubkey: script(&[0x51]),
        }],
        lock_time: 0,
    }
}

fn op_return_transaction(previous_txid: Txid) -> Transaction {
    Transaction {
        version: 2,
        inputs: vec![TransactionInput {
            previous_output: OutPoint {
                txid: previous_txid,
                vout: 0,
            },
            script_sig: script(&[0x51]),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(10).expect("valid amount"),
            script_pubkey: script(&[0x6a, 0x01, 0x01]),
        }],
        lock_time: 0,
    }
}

fn mine_header(block: &mut Block) {
    block.header.nonce = (0..=u32::MAX)
        .find(|nonce| {
            block.header.nonce = *nonce;
            check_block_header(&block.header).is_ok()
        })
        .expect("expected to find a nonce for easy regtest target");
}

fn build_block(previous_block_hash: BlockHash, time: u32, transactions: Vec<Transaction>) -> Block {
    let (merkle_root, maybe_mutated) = block_merkle_root(&transactions).expect("merkle root");
    assert!(!maybe_mutated);

    let mut block = Block {
        header: BlockHeader {
            version: 1,
            previous_block_hash,
            merkle_root,
            time,
            bits: EASY_BITS,
            nonce: 0,
        },
        transactions,
    };
    mine_header(&mut block);
    block
}

fn connect_block(chainstate: &mut Chainstate, block: &Block, chain_work: u128) -> ChainPosition {
    chainstate
        .connect_block(
            block,
            chain_work,
            ScriptVerifyFlags::P2SH
                | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
                | ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
            ConsensusParams {
                coinbase_maturity: 1,
                ..ConsensusParams::default()
            },
        )
        .expect("block should connect")
}

fn assert_active_tip(chainstate: &Chainstate, expected: &ChainPosition) {
    assert_eq!(chainstate.tip(), Some(expected));
}

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
fn apply_non_coinbase_transaction_consumes_inputs_and_records_undo() {
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
    apply_non_coinbase_transaction(
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
            previous_median_time_past: 0,
            consensus_params: ConsensusParams {
                coinbase_maturity: 1,
                ..ConsensusParams::default()
            },
        },
    )
    .expect("non-coinbase helper should apply cleanly");

    // Assert
    assert!(!next_utxos.contains_key(&spent_outpoint));
    assert_eq!(
        block_undo.transactions,
        vec![TxUndo {
            restored_inputs: vec![spent_coin],
        }]
    );
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

#[test]
fn script_num_helper_covers_negative_and_high_bit_cases() {
    assert_eq!(serialized_script_num(-1), vec![1, 0x81]);
    assert_eq!(serialized_script_num(128), vec![2, 0x80, 0x00]);
}
