// Parity breadcrumbs:
// - packages/bitcoin-knots/src/primitives/block.h
// - packages/bitcoin-knots/src/consensus/merkle.cpp
// - packages/bitcoin-knots/src/pow.cpp
// - packages/bitcoin-knots/src/validation.cpp

use super::*;

#[test]
fn contextual_helpers_cover_merkle_height_and_weight_edges() {
    let empty_block = Block {
        header: BlockHeader::default(),
        transactions: vec![],
    };
    assert_eq!(
        block_witness_merkle_root(&empty_block)
            .expect("empty witness merkle root")
            .to_byte_array(),
        [0_u8; 32],
    );
    assert!(!coinbase_has_height_prefix(&empty_block, 0));
    assert_eq!(serialized_script_num(0), vec![0x00]);
    assert_eq!(serialized_script_num(128), vec![0x02, 0x80, 0x00]);
    assert_eq!(serialized_script_num(-1), vec![0x01, 0x81]);

    let coinbase = coinbase_transaction();
    let coinbase_txid = crate::crypto::transaction_txid(&coinbase).expect("coinbase txid");
    let odd_block = Block {
        header: BlockHeader::default(),
        transactions: vec![
            coinbase.clone(),
            spend_transaction(coinbase_txid),
            spend_transaction(coinbase_txid),
        ],
    };
    let odd_root = block_witness_merkle_root(&odd_block).expect("odd witness merkle root");
    assert_ne!(odd_root.to_byte_array(), [0_u8; 32]);

    let mut witness_coinbase = coinbase_transaction();
    witness_coinbase.inputs[0].witness = ScriptWitness::new(vec![vec![0_u8; 1]]);
    let witness_coinbase_txid =
        crate::crypto::transaction_txid(&witness_coinbase).expect("coinbase txid");
    let mut witness_spend = spend_transaction(witness_coinbase_txid);
    witness_spend.inputs[0].witness = ScriptWitness::new(vec![vec![0_u8; 4_100_000]]);

    let mut heavy_block = Block {
        header: BlockHeader {
            version: 1,
            previous_block_hash: BlockHash::from_byte_array([0_u8; 32]),
            merkle_root: MerkleRoot::from_byte_array([0_u8; 32]),
            time: 1_231_006_505,
            bits: EASY_BITS,
            nonce: 0,
        },
        transactions: vec![witness_coinbase.clone(), witness_spend],
    };
    heavy_block.transactions[0].inputs[0].witness = ScriptWitness::new(vec![vec![9_u8; 32]]);
    let witness_root = block_witness_merkle_root(&heavy_block).expect("witness root");
    let mut commitment_preimage = [0_u8; 64];
    commitment_preimage[..32].copy_from_slice(witness_root.as_bytes());
    commitment_preimage[32..]
        .copy_from_slice(&heavy_block.transactions[0].inputs[0].witness.stack()[0]);
    let commitment = crate::crypto::double_sha256(&commitment_preimage);
    heavy_block.transactions[0].outputs.push(TransactionOutput {
        value: Amount::from_sats(0).expect("zero amount"),
        script_pubkey: script(
            &[&[0x6a, 0x24, 0xaa, 0x21, 0xa9, 0xed][..], &commitment[..]].concat(),
        ),
    });
    let (heavy_merkle_root, _) = block_merkle_root(&heavy_block.transactions).expect("merkle root");
    heavy_block.header.merkle_root = heavy_merkle_root;
    mine_header(&mut heavy_block);

    let no_mtp_context = BlockValidationContext {
        height: 1,
        previous_header: BlockHeader {
            bits: heavy_block.header.bits,
            time: heavy_block.header.time - 1,
            ..BlockHeader::default()
        },
        maybe_retarget_anchor: None,
        maybe_min_difficulty_recovery_target: Some(MinDifficultyRecoveryTarget {
            bits: heavy_block.header.bits,
        }),
        previous_median_time_past: i64::from(heavy_block.header.time) - 10,
        current_time: i64::from(heavy_block.header.time),
        consensus_params: ConsensusParams {
            enforce_bip113_median_time_past: false,
            enforce_segwit: true,
            ..Default::default()
        },
    };
    assert_eq!(
        check_block_contextual(&heavy_block, &no_mtp_context)
            .expect_err("witness weight must fail")
            .reject_reason,
        "bad-blk-weight",
    );

    let mut bad_nonce_block = heavy_block.clone();
    bad_nonce_block.transactions[0].inputs[0].witness = ScriptWitness::new(vec![vec![0_u8; 1]]);
    assert_eq!(
        check_block_contextual(
            &bad_nonce_block,
            &BlockValidationContext {
                consensus_params: ConsensusParams {
                    enforce_segwit: true,
                    ..Default::default()
                },
                ..no_mtp_context.clone()
            }
        )
        .expect_err("bad witness nonce size must fail")
        .reject_reason,
        "bad-witness-nonce-size",
    );

    let mut missing_nonce_block = heavy_block;
    missing_nonce_block.transactions[0].inputs[0].witness = ScriptWitness::default();
    assert_eq!(
        check_block_contextual(
            &missing_nonce_block,
            &BlockValidationContext {
                consensus_params: ConsensusParams {
                    enforce_segwit: true,
                    ..Default::default()
                },
                ..no_mtp_context
            }
        )
        .expect_err("missing witness nonce must fail")
        .reject_reason,
        "bad-witness-nonce-size",
    );
}

#[test]
fn validate_block_with_context_maps_transaction_errors() {
    let (block, spent_outputs) = valid_block();
    let block_context = BlockValidationContext {
        height: 1,
        previous_header: BlockHeader {
            bits: block.header.bits,
            time: block.header.time - 1,
            ..BlockHeader::default()
        },
        maybe_retarget_anchor: None,
        maybe_min_difficulty_recovery_target: Some(MinDifficultyRecoveryTarget {
            bits: block.header.bits,
        }),
        previous_median_time_past: i64::from(block.header.time) - 1,
        current_time: i64::from(block.header.time),
        consensus_params: ConsensusParams {
            enforce_segwit: false,
            ..Default::default()
        },
    };
    let tx_contexts = vec![TransactionValidationContext {
        inputs: vec![TransactionInputContext {
            spent_output: SpentOutput {
                is_coinbase: true,
                ..spent_outputs[0][0].clone()
            },
            created_height: 1,
            created_median_time_past: 0,
        }],
        spend_height: 1,
        block_time: i64::from(block.header.time),
        median_time_past: i64::from(block.header.time) - 1,
        verify_flags: ScriptVerifyFlags::NONE,
        consensus_params: block_context.consensus_params,
    }];

    let error = validate_block_with_context(&block, &tx_contexts, &block_context)
        .expect_err("transaction context error should map to block error");
    assert_eq!(error.reject_reason, "bad-txns-premature-spend-of-coinbase");
    assert!(
        error
            .debug_message
            .expect("debug message")
            .contains("failed validation")
    );

    let no_debug_tx_contexts = vec![TransactionValidationContext {
        inputs: vec![TransactionInputContext {
            spent_output: SpentOutput {
                is_coinbase: false,
                ..spent_outputs[0][0].clone()
            },
            created_height: 0,
            created_median_time_past: 0,
        }],
        spend_height: 0,
        block_time: i64::from(block.header.time),
        median_time_past: i64::from(block.header.time) - 1,
        verify_flags: ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
        consensus_params: block_context.consensus_params,
    }];
    let mut sequence_locked_block = block.clone();
    sequence_locked_block.transactions[1].version = 2;
    sequence_locked_block.transactions[1].inputs[0].sequence = 2;
    let (sequence_merkle_root, _) =
        block_merkle_root(&sequence_locked_block.transactions).expect("merkle root");
    sequence_locked_block.header.merkle_root = sequence_merkle_root;
    mine_header(&mut sequence_locked_block);

    let error = validate_block_with_context(
        &sequence_locked_block,
        &no_debug_tx_contexts,
        &block_context,
    )
    .expect_err("sequence lock failure should map without source debug");
    assert_eq!(error.reject_reason, "non-BIP68-final");
    assert!(
        error
            .debug_message
            .expect("debug message")
            .contains("failed validation")
    );
}

#[test]
fn validate_block_with_context_rejects_split_sigop_overflow() {
    let coinbase = coinbase_transaction();
    let mut transactions = vec![coinbase];
    let mut transaction_contexts = Vec::new();
    for index in 1..=127_u8 {
        let (transaction, context) = p2sh_sigop_heavy_transaction(index, 200);
        transactions.push(transaction);
        transaction_contexts.push(context);
    }

    let (merkle_root, maybe_mutated) = block_merkle_root(&transactions).expect("merkle root");
    assert!(!maybe_mutated);
    let mut block = Block {
        header: BlockHeader {
            version: 1,
            previous_block_hash: BlockHash::from_byte_array([0_u8; 32]),
            merkle_root,
            time: 1_231_006_505,
            bits: EASY_BITS,
            nonce: 0,
        },
        transactions,
    };
    mine_header(&mut block);
    let block_context = BlockValidationContext {
        height: 1,
        previous_header: BlockHeader {
            bits: block.header.bits,
            time: block.header.time - 1,
            ..BlockHeader::default()
        },
        maybe_retarget_anchor: None,
        maybe_min_difficulty_recovery_target: Some(MinDifficultyRecoveryTarget {
            bits: block.header.bits,
        }),
        previous_median_time_past: i64::from(block.header.time) - 1,
        current_time: i64::from(block.header.time),
        consensus_params: ConsensusParams {
            enforce_segwit: false,
            ..ConsensusParams::default()
        },
    };

    let error = validate_block_with_context(&block, &transaction_contexts, &block_context)
        .expect_err("split sigop overflow must fail");

    assert_eq!(error.reject_reason, "bad-blk-sigops");
}

#[test]
fn sigop_helper_functions_are_covered_directly() {
    let (transaction, context) = p2sh_sigop_heavy_transaction(200, 5);

    assert_eq!(legacy_sigop_cost(&transaction).expect("legacy cost"), 0);
    assert_eq!(
        split_sigop_cost(&transaction, &context).expect("split cost"),
        20
    );
    assert_eq!(block_sigop_overflow().reject_reason, "bad-blk-sigops");
    assert_eq!(enforce_sigop_cost_limit(0), Ok(()));
    assert_eq!(
        enforce_sigop_cost_limit(MAX_BLOCK_SIGOPS_COST + 1)
            .expect_err("overflow must fail")
            .reject_reason,
        "bad-blk-sigops"
    );
}

#[test]
fn block_subsidy_follows_halving_schedule_and_zeroes_after_sixty_four_halvings() {
    // Arrange
    let default_params = ConsensusParams::default();
    let tiny_interval_params = ConsensusParams {
        subsidy_halving_interval: 2,
        ..ConsensusParams::default()
    };

    // Act
    let genesis_subsidy = block_subsidy(0, &default_params);
    let first_tiny_halving = block_subsidy(2, &tiny_interval_params);
    let zero_subsidy = block_subsidy(128, &tiny_interval_params);

    // Assert
    assert_eq!(genesis_subsidy.to_sats(), 50 * COIN);
    assert_eq!(first_tiny_halving.to_sats(), 25 * COIN);
    assert_eq!(zero_subsidy, Amount::ZERO);
}

#[test]
fn enforce_coinbase_reward_limit_accepts_exact_limit_and_rejects_overpay() {
    // Arrange
    let mut exact_coinbase = coinbase_transaction();
    exact_coinbase.outputs[0].value = Amount::from_sats((50 * COIN) + 10).expect("valid amount");
    let exact_block = Block {
        header: BlockHeader::default(),
        transactions: vec![exact_coinbase],
    };
    let mut overpay_coinbase = coinbase_transaction();
    overpay_coinbase.outputs[0].value = Amount::from_sats((50 * COIN) + 11).expect("valid amount");
    let overpay_block = Block {
        header: BlockHeader::default(),
        transactions: vec![overpay_coinbase],
    };
    let consensus_params = ConsensusParams::default();

    // Act
    let exact_result = enforce_coinbase_reward_limit(&exact_block, 0, 10, &consensus_params);
    let overpay_error = enforce_coinbase_reward_limit(&overpay_block, 0, 10, &consensus_params)
        .expect_err("coinbase above subsidy plus fees must fail");

    // Assert
    assert_eq!(exact_result, Ok(()));
    assert_eq!(overpay_error.reject_reason, "bad-cb-amount");
    assert_eq!(
        overpay_error.debug_message.as_deref(),
        Some("coinbase pays too much (actual=5000000011 vs limit=5000000010)"),
    );
}

#[test]
fn enforce_coinbase_reward_limit_reports_missing_and_overflowing_coinbase_values() {
    // Arrange
    let empty_block = Block {
        header: BlockHeader::default(),
        transactions: vec![],
    };
    let mut overflowing_coinbase = coinbase_transaction();
    overflowing_coinbase.outputs = (0..5_000)
        .map(|_| TransactionOutput {
            value: Amount::from_sats(MAX_MONEY).expect("max money"),
            script_pubkey: script(&[0x51]),
        })
        .collect();
    let overflowing_block = Block {
        header: BlockHeader::default(),
        transactions: vec![overflowing_coinbase],
    };

    // Act
    let missing_error =
        enforce_coinbase_reward_limit(&empty_block, 0, 0, &ConsensusParams::default())
            .expect_err("missing coinbase should fail");
    let overflow_error =
        enforce_coinbase_reward_limit(&overflowing_block, 0, 0, &ConsensusParams::default())
            .expect_err("overflowing coinbase total should fail");

    // Assert
    assert_eq!(missing_error.reject_reason, "bad-cb-missing");
    assert_eq!(overflow_error.reject_reason, "bad-txns-txouttotal-toolarge");
}

#[test]
fn validate_block_with_context_accepts_exact_coinbase_reward_limit() {
    // Arrange
    let block = reward_limit_block((50 * COIN) + 10);
    let block_context = reward_limit_block_context(&block);
    let transaction_contexts = vec![reward_limit_transaction_context(&block, 50)];

    // Act
    let result = validate_block_with_context(&block, &transaction_contexts, &block_context);

    // Assert
    assert_eq!(result, Ok(()));
}

#[test]
fn validate_block_with_context_rejects_coinbase_overpay_with_bad_cb_amount() {
    // Arrange
    let block = reward_limit_block((50 * COIN) + 11);
    let block_context = reward_limit_block_context(&block);
    let transaction_contexts = vec![reward_limit_transaction_context(&block, 50)];

    // Act
    let error = validate_block_with_context(&block, &transaction_contexts, &block_context)
        .expect_err("coinbase above subsidy plus fees must fail");

    // Assert
    assert_eq!(error.result, BlockValidationResult::Consensus);
    assert_eq!(error.reject_reason, "bad-cb-amount");
    assert_eq!(
        error.debug_message.as_deref(),
        Some("coinbase pays too much (actual=5000000011 vs limit=5000000010)"),
    );
}

#[test]
fn validate_block_with_context_rejects_accumulated_fees_above_max_money() {
    // Arrange
    let mut first_spend = spend_transaction(unique_txid(10));
    first_spend.outputs[0].value = Amount::ZERO;
    let mut second_spend = spend_transaction(unique_txid(11));
    second_spend.outputs[0].value = Amount::ZERO;
    let third_spend = spend_transaction(unique_txid(12));
    let block = mined_block(vec![
        coinbase_transaction(),
        first_spend,
        second_spend,
        third_spend,
    ]);
    let block_context = reward_limit_block_context(&block);
    let transaction_contexts = vec![
        reward_limit_transaction_context(&block, MAX_MONEY),
        reward_limit_transaction_context(&block, 1),
        reward_limit_transaction_context(&block, 40),
    ];

    // Act
    let error = validate_block_with_context(&block, &transaction_contexts, &block_context)
        .expect_err("accumulated fees above MAX_MONEY must fail");

    // Assert
    assert_eq!(error.result, BlockValidationResult::Consensus);
    assert_eq!(error.reject_reason, "bad-txns-accumulated-fee-outofrange");
    assert_eq!(
        error.debug_message.as_deref(),
        Some("accumulated fee in the block out of range"),
    );
}

#[test]
fn validate_block_with_context_rejects_accumulated_fee_overflow() {
    // Arrange
    let mut transactions = vec![coinbase_transaction()];
    let mut transaction_contexts = Vec::with_capacity(8_191);
    for seed in 0..8_191_u64 {
        let mut transaction = spend_transaction(unique_txid(seed + 10));
        transaction.outputs[0].value = Amount::from_sats(1).expect("valid amount");
        transactions.push(transaction);
        transaction_contexts.push(TransactionValidationContext {
            inputs: vec![TransactionInputContext {
                spent_output: SpentOutput {
                    value: Amount::from_sats(MAX_MONEY).expect("max money"),
                    script_pubkey: script(&[0x52]),
                    is_coinbase: false,
                },
                created_height: 0,
                created_median_time_past: 0,
            }],
            spend_height: 1,
            block_time: 1_231_006_505,
            median_time_past: 1_231_006_504,
            verify_flags: ScriptVerifyFlags::NONE,
            consensus_params: ConsensusParams {
                enforce_segwit: false,
                ..Default::default()
            },
        });
    }
    let block = mined_block(transactions);
    let block_context = reward_limit_block_context(&block);

    // Act
    let error = validate_block_with_context(&block, &transaction_contexts, &block_context)
        .expect_err("accumulated fees above i64 must fail");

    // Assert
    assert_eq!(error.result, BlockValidationResult::Consensus);
    assert_eq!(error.reject_reason, "bad-txns-accumulated-fee-outofrange");
    assert_eq!(
        error.debug_message.as_deref(),
        Some("accumulated fee in the block out of range"),
    );
}
