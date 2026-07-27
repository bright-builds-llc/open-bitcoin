use super::*;

#[test]
fn contextual_block_checks_cover_context_mapping_and_nonfinal_rejection() {
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

    assert_eq!(
        validate_block_with_context(&block, &[], &context)
            .expect_err("missing contexts must fail")
            .reject_reason,
        "bad-txns-inputs-missingorspent",
    );

    let mut nonfinal_block = block.clone();
    nonfinal_block.transactions[1].lock_time = 2;
    nonfinal_block.transactions[1].inputs[0].sequence = 0;
    let (merkle_root, _) = block_merkle_root(&nonfinal_block.transactions).expect("merkle root");
    nonfinal_block.header.merkle_root = merkle_root;
    mine_header(&mut nonfinal_block);

    assert_eq!(
        check_block_contextual(&nonfinal_block, &context)
            .expect_err("non-final tx must fail")
            .reject_reason,
        "bad-txns-nonfinal",
    );
}

#[test]
fn witness_commitment_and_coinbase_height_paths_are_exercised() {
    let mut coinbase = coinbase_transaction();
    coinbase.inputs[0].witness = ScriptWitness::new(vec![vec![9_u8; 32]]);
    let coinbase_txid = crate::crypto::transaction_txid(&coinbase).expect("coinbase txid");
    let mut spend = spend_transaction(coinbase_txid);
    spend.inputs[0].witness = ScriptWitness::new(vec![vec![0x01]]);

    let mut block = Block {
        header: BlockHeader {
            version: 1,
            previous_block_hash: BlockHash::from_byte_array([0_u8; 32]),
            merkle_root: MerkleRoot::from_byte_array([0_u8; 32]),
            time: 1_231_006_505,
            bits: EASY_BITS,
            nonce: 0,
        },
        transactions: vec![coinbase.clone(), spend.clone()],
    };

    let witness_root = block_witness_merkle_root(&block).expect("witness root");
    let mut commitment_preimage = [0_u8; 64];
    commitment_preimage[..32].copy_from_slice(witness_root.as_bytes());
    commitment_preimage[32..].copy_from_slice(&coinbase.inputs[0].witness.stack()[0]);
    let commitment = crate::crypto::double_sha256(&commitment_preimage);
    block.transactions[0].outputs.push(TransactionOutput {
        value: Amount::from_sats(0).expect("zero amount"),
        script_pubkey: script(
            &[&[0x6a, 0x24, 0xaa, 0x21, 0xa9, 0xed][..], &commitment[..]].concat(),
        ),
    });
    let (merkle_root, _) = block_merkle_root(&block.transactions).expect("merkle root");
    block.header.merkle_root = merkle_root;
    mine_header(&mut block);

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
        consensus_params: ConsensusParams::default(),
    };

    assert_eq!(witness_commitment_index(&block), Some(1));
    assert_eq!(check_block_contextual(&block, &context), Ok(()));

    let bad_height_context = BlockValidationContext {
        height: 2,
        ..context.clone()
    };
    assert_eq!(
        check_block_contextual(&block, &bad_height_context)
            .expect_err("coinbase height mismatch must fail")
            .reject_reason,
        "bad-cb-height",
    );

    let mut bad_commitment_block = block.clone();
    bad_commitment_block.transactions[0].outputs[1].script_pubkey =
        script(&[&[0x6a, 0x24, 0xaa, 0x21, 0xa9, 0xed][..], &[7_u8; 32][..]].concat());
    let (bad_merkle_root, _) =
        block_merkle_root(&bad_commitment_block.transactions).expect("merkle root");
    bad_commitment_block.header.merkle_root = bad_merkle_root;
    mine_header(&mut bad_commitment_block);
    assert_eq!(
        check_block_contextual(&bad_commitment_block, &context)
            .expect_err("bad witness commitment must fail")
            .reject_reason,
        "bad-witness-merkle-match",
    );

    let mut missing_commitment_block = block.clone();
    missing_commitment_block.transactions[0].outputs.pop();
    let (missing_commitment_merkle_root, _) =
        block_merkle_root(&missing_commitment_block.transactions).expect("merkle root");
    missing_commitment_block.header.merkle_root = missing_commitment_merkle_root;
    mine_header(&mut missing_commitment_block);
    assert_eq!(
        check_block_contextual(&missing_commitment_block, &context)
            .expect_err("missing witness commitment must fail")
            .reject_reason,
        "unexpected-witness",
    );

    let no_witness_context = BlockValidationContext {
        consensus_params: ConsensusParams {
            enforce_segwit: false,
            ..Default::default()
        },
        ..context
    };
    assert_eq!(
        check_block_contextual(&block, &no_witness_context)
            .expect_err("unexpected witness must fail")
            .reject_reason,
        "unexpected-witness",
    );
}

#[test]
fn difficulty_helpers_cover_contextual_work_branches() {
    // Arrange
    let base_params = ConsensusParams::default();
    let header = BlockHeader {
        time: 10_000,
        bits: base_params.pow_limit_bits,
        ..BlockHeader::default()
    };
    let previous_header = BlockHeader {
        bits: 0x1f00_ffff,
        time: header.time - 1,
        ..BlockHeader::default()
    };

    // Act / Assert
    assert_eq!(
        difficulty_adjustment_interval(&ConsensusParams {
            pow_target_spacing_seconds: 0,
            ..base_params
        }),
        1,
    );
    assert_eq!(
        next_work_required(
            &header,
            &BlockValidationContext {
                height: 0,
                previous_header: previous_header.clone(),
                maybe_retarget_anchor: None,
                maybe_min_difficulty_recovery_target: None,
                previous_median_time_past: 0,
                current_time: i64::from(header.time),
                consensus_params: base_params,
            },
        )
        .expect("genesis work should compute"),
        base_params.pow_limit_bits,
    );
    assert_eq!(
        next_work_required(
            &header,
            &BlockValidationContext {
                height: 1,
                previous_header: previous_header.clone(),
                maybe_retarget_anchor: None,
                maybe_min_difficulty_recovery_target: Some(MinDifficultyRecoveryTarget {
                    bits: previous_header.bits,
                }),
                previous_median_time_past: 0,
                current_time: i64::from(header.time),
                consensus_params: base_params,
            },
        )
        .expect("non-boundary work should compute"),
        previous_header.bits,
    );
    assert_eq!(
        next_work_required(
            &header,
            &BlockValidationContext {
                height: 1,
                previous_header: BlockHeader {
                    bits: previous_header.bits,
                    time: header.time - 1_201,
                    ..BlockHeader::default()
                },
                maybe_retarget_anchor: None,
                maybe_min_difficulty_recovery_target: None,
                previous_median_time_past: 0,
                current_time: i64::from(header.time),
                consensus_params: base_params,
            },
        )
        .expect("min-difficulty work should compute"),
        base_params.pow_limit_bits,
    );

    let retarget_params = ConsensusParams {
        allow_min_difficulty_blocks: false,
        no_pow_retargeting: false,
        pow_target_spacing_seconds: 10,
        pow_target_timespan_seconds: 20,
        ..base_params
    };
    let retarget_height = difficulty_adjustment_interval(&retarget_params) as u32;
    let retarget_previous_header = BlockHeader {
        bits: base_params.pow_limit_bits,
        time: 110,
        ..BlockHeader::default()
    };
    let retarget_header = BlockHeader {
        time: 120,
        bits: retarget_previous_header.bits,
        ..BlockHeader::default()
    };

    assert_eq!(
        next_work_required(
            &retarget_header,
            &BlockValidationContext {
                height: retarget_height,
                previous_header: previous_header.clone(),
                maybe_retarget_anchor: None,
                maybe_min_difficulty_recovery_target: Some(MinDifficultyRecoveryTarget {
                    bits: previous_header.bits,
                }),
                previous_median_time_past: 0,
                current_time: i64::from(retarget_header.time),
                consensus_params: base_params,
            },
        )
        .expect("retarget-disabled boundary work should compute"),
        previous_header.bits,
    );
    let retarget_bits = next_work_required(
        &retarget_header,
        &BlockValidationContext {
            height: retarget_height,
            previous_header: retarget_previous_header.clone(),
            maybe_retarget_anchor: Some(RetargetAnchor {
                first_block_time: 100,
            }),
            maybe_min_difficulty_recovery_target: None,
            previous_median_time_past: 0,
            current_time: i64::from(retarget_header.time),
            consensus_params: retarget_params,
        },
    )
    .expect("retarget-enabled boundary work should compute");
    assert_ne!(retarget_bits, retarget_previous_header.bits,);
}

#[test]
fn contextual_header_rejects_previous_bits_at_retarget_boundary() {
    // Arrange
    let consensus_params = ConsensusParams {
        allow_min_difficulty_blocks: false,
        no_pow_retargeting: false,
        pow_target_spacing_seconds: 10,
        pow_target_timespan_seconds: 20,
        ..ConsensusParams::default()
    };
    let previous_header = BlockHeader {
        bits: consensus_params.pow_limit_bits,
        time: 110,
        ..BlockHeader::default()
    };
    let header = BlockHeader {
        time: 120,
        bits: previous_header.bits,
        ..BlockHeader::default()
    };
    let context = BlockValidationContext {
        height: difficulty_adjustment_interval(&consensus_params) as u32,
        previous_header,
        maybe_retarget_anchor: Some(RetargetAnchor {
            first_block_time: 100,
        }),
        maybe_min_difficulty_recovery_target: None,
        previous_median_time_past: 109,
        current_time: i64::from(header.time),
        consensus_params,
    };

    // Act
    let error = check_block_header_contextual(&header, &context)
        .expect_err("stale previous bits must fail at a retarget boundary");

    // Assert
    assert_eq!(error.result, BlockValidationResult::InvalidHeader);
    assert_eq!(error.reject_reason, "bad-diffbits");
}

#[test]
fn contextual_header_rejects_previous_bits_after_special_min_difficulty_block() {
    // Arrange
    let consensus_params = ConsensusParams {
        allow_min_difficulty_blocks: true,
        no_pow_retargeting: false,
        pow_target_spacing_seconds: 10,
        pow_target_timespan_seconds: 20,
        ..ConsensusParams::default()
    };
    let recovered_bits = 0x207e_ffff;
    let recovered_header = BlockHeader {
        time: 140,
        bits: recovered_bits,
        ..BlockHeader::default()
    };
    let stale_header = BlockHeader {
        bits: consensus_params.pow_limit_bits,
        ..recovered_header.clone()
    };
    let context = BlockValidationContext {
        height: 3,
        previous_header: BlockHeader {
            bits: consensus_params.pow_limit_bits,
            time: 131,
            ..BlockHeader::default()
        },
        maybe_retarget_anchor: None,
        maybe_min_difficulty_recovery_target: Some(MinDifficultyRecoveryTarget {
            bits: recovered_bits,
        }),
        previous_median_time_past: 130,
        current_time: i64::from(recovered_header.time),
        consensus_params,
    };

    // Act
    let ok_result = check_block_header_contextual(&recovered_header, &context);
    let error = check_block_header_contextual(&stale_header, &context)
        .expect_err("previous special bits must fail after recovery");

    // Assert
    assert_eq!(ok_result, Ok(()));
    assert_eq!(error.result, BlockValidationResult::InvalidHeader);
    assert_eq!(error.reject_reason, "bad-diffbits");
}
