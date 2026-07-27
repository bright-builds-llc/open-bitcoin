use super::*;

#[test]
fn repo_owned_contextual_consensus_regressions_are_covered() {
    let coinbase = coinbase_transaction_with_height(1);
    let coinbase_txid = open_bitcoin_consensus::transaction_txid(&coinbase).expect("coinbase txid");

    let immature_spend = spend_transaction(coinbase_txid, 0, TransactionInput::SEQUENCE_FINAL);
    let immature_context = TransactionValidationContext {
        inputs: vec![TransactionInputContext {
            spent_output: SpentOutput {
                value: coinbase.outputs[0].value,
                script_pubkey: coinbase.outputs[0].script_pubkey.clone(),
                is_coinbase: true,
            },
            created_height: 1,
            created_median_time_past: 0,
        }],
        spend_height: 10,
        block_time: 0,
        median_time_past: 0,
        verify_flags: ScriptVerifyFlags::NONE,
        consensus_params: ConsensusParams::default(),
    };
    let maturity_error = validate_transaction_with_context(&immature_spend, &immature_context)
        .expect_err("immature coinbase spend must fail");
    assert_eq!(
        maturity_error.reject_reason,
        "bad-txns-premature-spend-of-coinbase"
    );

    let nonfinal_tx = spend_transaction(coinbase_txid, 2, 0);
    let nonfinal_context = TransactionValidationContext {
        inputs: vec![TransactionInputContext {
            spent_output: SpentOutput {
                value: Amount::from_sats(50).expect("valid amount"),
                script_pubkey: script(&[0x51]),
                is_coinbase: false,
            },
            created_height: 0,
            created_median_time_past: 0,
        }],
        spend_height: 1,
        block_time: 0,
        median_time_past: 0,
        verify_flags: ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
        consensus_params: ConsensusParams::default(),
    };
    let nonfinal_error = validate_transaction_with_context(&nonfinal_tx, &nonfinal_context)
        .expect_err("non-final transaction must fail");
    assert_eq!(nonfinal_error.reject_reason, "bad-txns-nonfinal");

    let sequence_locked_tx = spend_transaction(coinbase_txid, 0, 5);
    let sequence_locked_context = TransactionValidationContext {
        inputs: vec![TransactionInputContext {
            spent_output: SpentOutput {
                value: Amount::from_sats(50).expect("valid amount"),
                script_pubkey: script(&[0x51]),
                is_coinbase: false,
            },
            created_height: 0,
            created_median_time_past: 0,
        }],
        spend_height: 1,
        block_time: 0,
        median_time_past: 0,
        verify_flags: ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
        consensus_params: ConsensusParams::default(),
    };
    let sequence_error =
        validate_transaction_with_context(&sequence_locked_tx, &sequence_locked_context)
            .expect_err("sequence-locked transaction must fail");
    assert_eq!(sequence_error.reject_reason, "non-BIP68-final");

    let mut coinbase_with_witness = coinbase_transaction_with_height(1);
    coinbase_with_witness.inputs[0].witness = ScriptWitness::new(vec![vec![9_u8; 32]]);
    let witness_spend_txid =
        open_bitcoin_consensus::transaction_txid(&coinbase_with_witness).expect("coinbase txid");
    let mut witness_spend =
        spend_transaction(witness_spend_txid, 0, TransactionInput::SEQUENCE_FINAL);
    witness_spend.inputs[0].witness = ScriptWitness::new(vec![vec![1_u8]]);
    let mut block = Block {
        header: BlockHeader {
            version: 1,
            previous_block_hash: BlockHash::from_byte_array([0_u8; 32]),
            merkle_root: MerkleRoot::from_byte_array([0_u8; 32]),
            time: 1_231_006_505,
            bits: EASY_BITS,
            nonce: 0,
        },
        transactions: vec![coinbase_with_witness.clone(), witness_spend],
    };
    let witness_root = witness_merkle_root(&block);
    let mut commitment_preimage = [0_u8; 64];
    commitment_preimage[..32].copy_from_slice(witness_root.as_bytes());
    commitment_preimage[32..].copy_from_slice(&coinbase_with_witness.inputs[0].witness.stack()[0]);
    let commitment = open_bitcoin_consensus::crypto::double_sha256(&commitment_preimage);
    block.transactions[0].outputs.push(TransactionOutput {
        value: Amount::from_sats(0).expect("zero amount"),
        script_pubkey: script(
            &[&[0x6a, 0x24, 0xaa, 0x21, 0xa9, 0xed][..], &commitment[..]].concat(),
        ),
    });
    let (merkle_root, _) =
        open_bitcoin_consensus::block_merkle_root(&block.transactions).expect("merkle root");
    block.header.merkle_root = merkle_root;
    mine_header(&mut block);

    let block_context = BlockValidationContext {
        height: 1,
        previous_header: BlockHeader {
            bits: block.header.bits,
            time: block.header.time - 1,
            ..BlockHeader::default()
        },
        maybe_retarget_anchor: None,
        maybe_min_difficulty_recovery_target: Some(
            open_bitcoin_consensus::context::MinDifficultyRecoveryTarget {
                bits: block.header.bits,
            },
        ),
        previous_median_time_past: i64::from(block.header.time) - 1,
        current_time: i64::from(block.header.time),
        consensus_params: ConsensusParams::default(),
    };
    assert!(check_block_contextual(&block, &block_context).is_ok());

    block.transactions[0].outputs[1].script_pubkey =
        script(&[&[0x6a, 0x24, 0xaa, 0x21, 0xa9, 0xed][..], &[0_u8; 32][..]].concat());
    let (bad_merkle_root, _) =
        open_bitcoin_consensus::block_merkle_root(&block.transactions).expect("merkle root");
    block.header.merkle_root = bad_merkle_root;
    mine_header(&mut block);
    let witness_commitment_error = check_block_contextual(&block, &block_context)
        .expect_err("bad witness commitment must fail");
    assert_eq!(
        witness_commitment_error.reject_reason,
        "bad-witness-merkle-match"
    );

    let unexpected_witness_tx = Transaction {
        version: 1,
        inputs: vec![TransactionInput {
            previous_output: OutPoint {
                txid: Txid::from_byte_array([8_u8; 32]),
                vout: 0,
            },
            script_sig: script(&[0x51]),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::new(vec![vec![1_u8]]),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(1).expect("valid amount"),
            script_pubkey: script(&[0x51]),
        }],
        lock_time: 0,
    };
    let unexpected_witness_context = TransactionValidationContext {
        inputs: vec![TransactionInputContext {
            spent_output: SpentOutput {
                value: Amount::from_sats(1).expect("valid amount"),
                script_pubkey: script(&[0x51, 0xac]),
                is_coinbase: false,
            },
            created_height: 0,
            created_median_time_past: 0,
        }],
        spend_height: 1,
        block_time: 0,
        median_time_past: 0,
        verify_flags: ScriptVerifyFlags::P2SH | ScriptVerifyFlags::WITNESS,
        consensus_params: ConsensusParams::default(),
    };
    let unexpected_witness_error =
        validate_transaction_with_context(&unexpected_witness_tx, &unexpected_witness_context)
            .expect_err("unexpected witness must fail");
    assert_eq!(
        unexpected_witness_error.reject_reason,
        "mandatory-script-verify-flag-failed"
    );
}
