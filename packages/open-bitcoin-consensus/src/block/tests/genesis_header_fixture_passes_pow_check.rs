// Parity breadcrumbs:
// - packages/bitcoin-knots/src/primitives/block.h
// - packages/bitcoin-knots/src/consensus/merkle.cpp
// - packages/bitcoin-knots/src/pow.cpp
// - packages/bitcoin-knots/src/validation.cpp

use super::*;

#[test]
fn genesis_header_fixture_passes_pow_check() {
    let header = parse_block_header(&decode_hex(GENESIS_BLOCK_HEADER_HEX))
        .expect("genesis header fixture should parse");

    assert_eq!(check_block_header(&header), Ok(()));
}

#[test]
fn check_block_accepts_mined_block() {
    let (block, _) = valid_block();

    assert_eq!(check_block(&block), Ok(()));
}

#[test]
fn validate_block_accepts_matching_spent_outputs() {
    let (block, spent_outputs) = valid_block();

    assert_eq!(validate_block(&block, &spent_outputs), Ok(()));
}

#[test]
fn check_block_rejects_bad_merkle_root() {
    let (mut block, _) = valid_block();
    block.header.merkle_root = MerkleRoot::from_byte_array([9_u8; 32]);
    mine_header(&mut block);

    let error = check_block(&block).expect_err("bad merkle root must fail");

    assert_eq!(error.reject_reason, "bad-txnmrklroot");
}

#[test]
fn check_block_rejects_missing_coinbase() {
    let (mut block, spent_outputs) = valid_block();
    block.transactions.swap(0, 1);
    let (merkle_root, _) = block_merkle_root(&block.transactions).expect("merkle root");
    block.header.merkle_root = merkle_root;
    mine_header(&mut block);

    let error = check_block(&block).expect_err("missing coinbase must fail");

    assert_eq!(error.reject_reason, "bad-cb-missing");
    assert_eq!(spent_outputs.len(), 1);
}

#[test]
fn check_block_rejects_multiple_coinbases() {
    let mut second_coinbase = coinbase_transaction();
    second_coinbase.inputs[0].script_sig = script(&[0x01, 0x02, 0x52]);
    let mut block = Block {
        header: BlockHeader {
            version: 1,
            previous_block_hash: BlockHash::from_byte_array([0_u8; 32]),
            merkle_root: MerkleRoot::from_byte_array([0_u8; 32]),
            time: 1_231_006_505,
            bits: EASY_BITS,
            nonce: 0,
        },
        transactions: vec![coinbase_transaction(), second_coinbase],
    };
    let (merkle_root, _) = block_merkle_root(&block.transactions).expect("merkle root");
    block.header.merkle_root = merkle_root;
    mine_header(&mut block);

    let error = check_block(&block).expect_err("multiple coinbases must fail");

    assert_eq!(error.reject_reason, "bad-cb-multiple");
}

#[test]
fn check_block_rejects_duplicate_transactions_even_with_matching_root() {
    let coinbase = coinbase_transaction();
    let coinbase_txid = crate::crypto::transaction_txid(&coinbase).expect("coinbase txid");
    let spend = spend_transaction(coinbase_txid);
    let mut block = Block {
        header: BlockHeader {
            version: 1,
            previous_block_hash: BlockHash::from_byte_array([0_u8; 32]),
            merkle_root: MerkleRoot::from_byte_array([0_u8; 32]),
            time: 1_231_006_505,
            bits: EASY_BITS,
            nonce: 0,
        },
        transactions: vec![coinbase, spend.clone(), spend],
    };
    let (merkle_root, maybe_mutated) = block_merkle_root(&block.transactions).expect("merkle root");
    assert!(maybe_mutated);
    block.header.merkle_root = merkle_root;
    mine_header(&mut block);

    let error = check_block(&block).expect_err("mutated merkle tree must fail");

    assert_eq!(error.reject_reason, "bad-txns-duplicate");
}

#[test]
fn validate_block_rejects_mismatched_spent_output_scripts() {
    let (block, mut spent_outputs) = valid_block();
    spent_outputs[0][0].script_pubkey = script(&[0x53, 0x87]);

    let error =
        validate_block(&block, &spent_outputs).expect_err("mismatched prevout script must fail");

    assert_eq!(error.reject_reason, "mandatory-script-verify-flag-failed");
}

#[test]
fn mined_block_hash_meets_easy_target() {
    let (block, _) = valid_block();
    let hash = block_hash(&block.header);

    assert_ne!(hash.to_byte_array(), [0_u8; 32]);
}

#[test]
fn check_block_header_rejects_invalid_bits() {
    let mut header = parse_block_header(&decode_hex(GENESIS_BLOCK_HEADER_HEX))
        .expect("genesis header fixture should parse");
    header.bits = 0x0180_0000;

    assert_eq!(
        check_block_header(&header)
            .expect_err("invalid bits must fail")
            .reject_reason,
        "bad-diffbits",
    );
}

#[test]
fn check_block_rejects_empty_blocks_and_oversized_blocks() {
    let mut empty_block = Block {
        header: BlockHeader {
            version: 1,
            previous_block_hash: BlockHash::from_byte_array([0_u8; 32]),
            merkle_root: MerkleRoot::from_byte_array([0_u8; 32]),
            time: 1_231_006_505,
            bits: EASY_BITS,
            nonce: 0,
        },
        transactions: vec![],
    };
    mine_header(&mut empty_block);
    assert_eq!(
        check_block(&empty_block)
            .expect_err("empty block must fail")
            .reject_reason,
        "bad-blk-length",
    );

    let big_script = script(&vec![0x51; 10_000]);
    let mut huge_block = Block {
        header: BlockHeader {
            version: 1,
            previous_block_hash: BlockHash::from_byte_array([0_u8; 32]),
            merkle_root: MerkleRoot::from_byte_array([0_u8; 32]),
            time: 1_231_006_505,
            bits: EASY_BITS,
            nonce: 0,
        },
        transactions: vec![Transaction {
            version: 1,
            inputs: vec![TransactionInput {
                previous_output: OutPoint::null(),
                script_sig: script(&[0x01, 0x01]),
                sequence: TransactionInput::SEQUENCE_FINAL,
                witness: ScriptWitness::default(),
            }],
            outputs: (0..101)
                .map(|_| TransactionOutput {
                    value: Amount::from_sats(1).expect("valid amount"),
                    script_pubkey: big_script.clone(),
                })
                .collect(),
            lock_time: 0,
        }],
    };
    let (merkle_root, _) = block_merkle_root(&huge_block.transactions).expect("merkle root");
    huge_block.header.merkle_root = merkle_root;
    mine_header(&mut huge_block);

    assert_eq!(
        check_block(&huge_block)
            .expect_err("oversized block must fail")
            .reject_reason,
        "bad-blk-length",
    );
}

#[test]
fn check_block_maps_transaction_and_sigop_failures() {
    let coinbase = coinbase_transaction();
    let mut invalid_tx =
        spend_transaction(crate::crypto::transaction_txid(&coinbase).expect("coinbase txid"));
    invalid_tx.inputs.push(invalid_tx.inputs[0].clone());
    let mut block = Block {
        header: BlockHeader {
            version: 1,
            previous_block_hash: BlockHash::from_byte_array([0_u8; 32]),
            merkle_root: MerkleRoot::from_byte_array([0_u8; 32]),
            time: 1_231_006_505,
            bits: EASY_BITS,
            nonce: 0,
        },
        transactions: vec![coinbase.clone(), invalid_tx],
    };
    let (merkle_root, _) = block_merkle_root(&block.transactions).expect("merkle root");
    block.header.merkle_root = merkle_root;
    mine_header(&mut block);

    assert_eq!(
        check_block(&block)
            .expect_err("invalid transaction must fail")
            .reject_reason,
        "bad-txns-inputs-duplicate",
    );

    let mut sigops_block = Block {
        header: BlockHeader {
            version: 1,
            previous_block_hash: BlockHash::from_byte_array([0_u8; 32]),
            merkle_root: MerkleRoot::from_byte_array([0_u8; 32]),
            time: 1_231_006_505,
            bits: EASY_BITS,
            nonce: 0,
        },
        transactions: vec![Transaction {
            version: 1,
            inputs: vec![TransactionInput {
                previous_output: OutPoint::null(),
                script_sig: script(&[0x01, 0x01]),
                sequence: TransactionInput::SEQUENCE_FINAL,
                witness: ScriptWitness::default(),
            }],
            outputs: vec![TransactionOutput {
                value: Amount::from_sats(1).expect("valid amount"),
                script_pubkey: script(&vec![0xae; 1_001]),
            }],
            lock_time: 0,
        }],
    };
    let (merkle_root, _) = block_merkle_root(&sigops_block.transactions).expect("merkle root");
    sigops_block.header.merkle_root = merkle_root;
    mine_header(&mut sigops_block);

    assert_eq!(
        check_block(&sigops_block)
            .expect_err("sigops overflow must fail")
            .reject_reason,
        "bad-blk-sigops",
    );
}

#[test]
fn validate_block_rejects_missing_prev_groups_and_uses_default_debug_message() {
    let (block, _) = valid_block();
    assert_eq!(
        validate_block(&block, &[])
            .expect_err("missing prev groups must fail")
            .reject_reason,
        "bad-txns-inputs-missingorspent",
    );

    let coinbase = coinbase_transaction();
    let coinbase_txid = crate::crypto::transaction_txid(&coinbase).expect("coinbase txid");
    let mut spend = spend_transaction(coinbase_txid);
    spend.inputs.push(TransactionInput {
        previous_output: OutPoint {
            txid: Txid::from_byte_array([9_u8; 32]),
            vout: 0,
        },
        script_sig: script(&[0x52]),
        sequence: TransactionInput::SEQUENCE_FINAL,
        witness: ScriptWitness::default(),
    });
    let mut block = Block {
        header: BlockHeader {
            version: 1,
            previous_block_hash: BlockHash::from_byte_array([0_u8; 32]),
            merkle_root: MerkleRoot::from_byte_array([0_u8; 32]),
            time: 1_231_006_505,
            bits: EASY_BITS,
            nonce: 0,
        },
        transactions: vec![coinbase, spend],
    };
    let (merkle_root, _) = block_merkle_root(&block.transactions).expect("merkle root");
    block.header.merkle_root = merkle_root;
    mine_header(&mut block);

    let error = validate_block(
        &block,
        &[vec![SpentOutput {
            value: Amount::from_sats(50).expect("valid amount"),
            script_pubkey: script(&[0x52, 0x87]),
            is_coinbase: true,
        }]],
    )
    .expect_err("missing inputs inside transaction must fail");

    assert_eq!(error.reject_reason, "bad-txns-inputs-missingorspent");
    assert!(
        error
            .debug_message
            .expect("debug message")
            .contains("failed validation")
    );
}

#[test]
fn map_transaction_validation_error_preserves_source_debug_message() {
    // Arrange
    let transaction = spend_transaction(Txid::from_byte_array([7_u8; 32]));
    let txid = format!(
        "{:?}",
        transaction_txid(&transaction)
            .expect("phase-2 typed transactions should serialize for txid logging")
            .to_byte_array()
    );
    let error = tx_error(
        TxValidationResult::Consensus,
        "bad-txns-debug",
        Some("inner details".to_string()),
    );

    // Act
    let mapped = map_transaction_validation_error(&transaction, error);

    // Assert
    assert_eq!(mapped.result, BlockValidationResult::Consensus);
    assert_eq!(mapped.reject_reason, "bad-txns-debug");
    assert_eq!(
        mapped.debug_message,
        Some(format!(
            "transaction {txid} failed validation: inner details"
        ))
    );
}

#[test]
fn map_transaction_validation_error_uses_default_debug_message_when_absent() {
    // Arrange
    let transaction = spend_transaction(Txid::from_byte_array([8_u8; 32]));
    let txid = format!(
        "{:?}",
        transaction_txid(&transaction)
            .expect("phase-2 typed transactions should serialize for txid logging")
            .to_byte_array()
    );
    let error = tx_error(TxValidationResult::Consensus, "bad-txns-debug", None);

    // Act
    let mapped = map_transaction_validation_error(&transaction, error);

    // Assert
    assert_eq!(mapped.result, BlockValidationResult::Consensus);
    assert_eq!(mapped.reject_reason, "bad-txns-debug");
    assert_eq!(
        mapped.debug_message,
        Some(format!("transaction {txid} failed validation"))
    );
}

#[test]
fn helper_functions_cover_serialization_and_mapping_paths() {
    let (block, _) = valid_block();
    let without_witness = serialized_block_size(&block, false).expect("size without witness");
    let with_witness = serialized_block_size(&block, true).expect("size with witness");

    assert!(with_witness >= without_witness);
    assert_eq!(compact_size_len(252), 1);
    assert_eq!(compact_size_len(253), 3);
    assert_eq!(compact_size_len(65_536), 5);
    assert_eq!(compact_size_len(u64::MAX), 9);
    assert_eq!(
        map_codec_error(open_bitcoin_codec::CodecError::UnexpectedEof {
            needed: 1,
            remaining: 0,
        })
        .reject_reason,
        "bad-blk-serialization",
    );
    assert_eq!(
        map_script_error(crate::script::ScriptError::BadOpcode).reject_reason,
        "bad-blk-script",
    );
}

#[test]
fn contextual_header_parity_covers_diffbits_future_time_and_mtp() {
    let (block, spent_outputs) = valid_block();
    let context = BlockValidationContext {
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

    assert_eq!(
        check_block_header_contextual(&block.header, &context),
        Ok(())
    );
    assert_eq!(check_block_contextual(&block, &context), Ok(()));

    let mut wrong_bits_header = block.header.clone();
    wrong_bits_header.bits = wrong_bits_header.bits.saturating_sub(1);
    let wrong_bits_error = check_block_header_contextual(&wrong_bits_header, &context)
        .expect_err("incorrect contextual bits must fail");
    assert_eq!(
        wrong_bits_error.result,
        BlockValidationResult::InvalidHeader
    );
    assert_eq!(wrong_bits_error.reject_reason, "bad-diffbits");
    assert_eq!(
        wrong_bits_error.debug_message.as_deref(),
        Some("incorrect proof of work"),
    );

    let stale_context = BlockValidationContext {
        previous_median_time_past: i64::from(block.header.time),
        ..context.clone()
    };
    let stale_error = check_block_header_contextual(&block.header, &stale_context)
        .expect_err("time-too-old must fail");
    assert_eq!(stale_error.result, BlockValidationResult::InvalidHeader);
    assert_eq!(stale_error.reject_reason, "time-too-old");

    let future_context = BlockValidationContext {
        current_time: i64::from(block.header.time) - 7_201,
        ..context.clone()
    };
    let future_error = check_block_header_contextual(&block.header, &future_context)
        .expect_err("time-too-new must fail");
    assert_eq!(future_error.result, BlockValidationResult::TimeFuture);
    assert_eq!(future_error.reject_reason, "time-too-new");
    assert_eq!(
        future_error.debug_message.as_deref(),
        Some("block timestamp too far in the future"),
    );

    let tx_contexts = vec![TransactionValidationContext {
        inputs: vec![TransactionInputContext {
            spent_output: SpentOutput {
                is_coinbase: false,
                ..spent_outputs[0][0].clone()
            },
            created_height: 0,
            created_median_time_past: 0,
        }],
        spend_height: 1,
        block_time: i64::from(block.header.time),
        median_time_past: i64::from(block.header.time) - 1,
        verify_flags: ScriptVerifyFlags::NONE,
        consensus_params: context.consensus_params,
    }];
    assert_eq!(
        validate_block_with_context(&block, &tx_contexts, &context),
        Ok(())
    );
}
