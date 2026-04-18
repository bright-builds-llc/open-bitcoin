use super::{
    SigHashType, SigVersion, legacy_sighash, segwit_v0_sighash, taproot_sighash,
    taproot_tagged_hash,
};
use crate::context::{
    ConsensusParams, ScriptExecutionData, ScriptVerifyFlags, SpentOutput, TransactionInputContext,
    TransactionValidationContext,
};
use open_bitcoin_codec::parse_transaction_without_witness;
use open_bitcoin_primitives::{
    Amount, Hash32, ScriptBuf, Transaction, TransactionInput, TransactionOutput, Txid,
};

fn decode_hex(input: &str) -> Vec<u8> {
    let trimmed = input.trim();
    let mut bytes = Vec::with_capacity(trimmed.len() / 2);
    let chars: Vec<char> = trimmed.chars().collect();
    for pair in chars.chunks(2) {
        let high = pair[0].to_digit(16).expect("hex fixture");
        let low = pair[1].to_digit(16).expect("hex fixture");
        bytes.push(((high << 4) | low) as u8);
    }
    bytes
}

fn script(bytes: &[u8]) -> ScriptBuf {
    ScriptBuf::from_bytes(bytes.to_vec()).expect("valid script")
}

#[test]
fn legacy_sighash_matches_upstream_vectors() {
    let transaction = parse_transaction_without_witness(&decode_hex("73107cbd025c22ebc8c3e0a47b2a760739216a528de8d4dab5d45cbeb3051cebae73b01ca10200000007ab6353656a636affffffffe26816dffc670841e6a6c8c61c586da401df1261a330a6c6b3dd9f9a0789bc9e000000000800ac6552ac6aac51ffffffff0174a8f0010000000004ac52515100000000")).expect("tx");
    let script_code = script(&decode_hex("5163ac63635151ac"));
    let digest = legacy_sighash(
        &script_code,
        &transaction,
        1,
        SigHashType::from_u32(1_190_874_345),
    );
    let mut expected =
        decode_hex("06e328de263a87b09beabe222a21627a6ea5c7f560030da31610c4611f4a46bc");
    expected.reverse();

    assert_eq!(
        digest.to_byte_array(),
        <[u8; 32]>::try_from(expected).expect("32-byte hash"),
    );
}

#[test]
fn legacy_sighash_single_bug_matches_one_hash() {
    let transaction = Transaction {
        version: 1,
        inputs: vec![TransactionInput {
            previous_output: open_bitcoin_primitives::OutPoint {
                txid: Txid::from_byte_array([2_u8; 32]),
                vout: 0,
            },
            script_sig: ScriptBuf::default(),
            sequence: 1,
            witness: Default::default(),
        }],
        outputs: vec![],
        lock_time: 0,
    };

    let digest = legacy_sighash(&ScriptBuf::default(), &transaction, 0, SigHashType::SINGLE);
    let mut expected = [0_u8; 32];
    expected[0] = 1;
    assert_eq!(digest.to_byte_array(), expected);
}

#[test]
fn segwit_v0_sighash_is_stable_for_same_context() {
    let transaction = Transaction {
        version: 2,
        inputs: vec![TransactionInput {
            previous_output: open_bitcoin_primitives::OutPoint {
                txid: Txid::from_byte_array([7_u8; 32]),
                vout: 0,
            },
            script_sig: ScriptBuf::default(),
            sequence: 1,
            witness: Default::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(40).expect("valid amount"),
            script_pubkey: script(&[0x51]),
        }],
        lock_time: 0,
    };
    let context = TransactionValidationContext {
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
        verify_flags: ScriptVerifyFlags::WITNESS,
        consensus_params: ConsensusParams::default(),
    };

    let precomputed = context.precompute(&transaction).expect("precompute");
    let first = segwit_v0_sighash(
        &script(&[0x51]),
        &transaction,
        0,
        &context.inputs[0],
        SigHashType::ALL,
        &precomputed,
    );
    let second = segwit_v0_sighash(
        &script(&[0x51]),
        &transaction,
        0,
        &context.inputs[0],
        SigHashType::ALL,
        &precomputed,
    );

    assert_eq!(first, second);
}

#[test]
fn taproot_tagged_hash_is_deterministic() {
    let tag = taproot_tagged_hash("TapSighash", b"abc");
    let same = taproot_tagged_hash("TapSighash", b"abc");
    let different = taproot_tagged_hash("TapLeaf", b"abc");

    assert_eq!(tag, same);
    assert_ne!(tag, different);
    assert!(matches!(SigVersion::Taproot, SigVersion::Taproot));
}

#[test]
fn sighash_modes_and_helpers_cover_remaining_branches() {
    let transaction = Transaction {
        version: 1,
        inputs: vec![
            TransactionInput {
                previous_output: open_bitcoin_primitives::OutPoint {
                    txid: Txid::from_byte_array([1_u8; 32]),
                    vout: 0,
                },
                script_sig: script(&[0xab, 0x51]),
                sequence: 7,
                witness: Default::default(),
            },
            TransactionInput {
                previous_output: open_bitcoin_primitives::OutPoint {
                    txid: Txid::from_byte_array([2_u8; 32]),
                    vout: 1,
                },
                script_sig: ScriptBuf::default(),
                sequence: 9,
                witness: Default::default(),
            },
        ],
        outputs: vec![
            TransactionOutput {
                value: Amount::from_sats(1).expect("valid amount"),
                script_pubkey: script(&[0x51]),
            },
            TransactionOutput {
                value: Amount::from_sats(2).expect("valid amount"),
                script_pubkey: script(&[0x52]),
            },
        ],
        lock_time: 3,
    };
    let context = TransactionValidationContext {
        inputs: vec![
            TransactionInputContext {
                spent_output: SpentOutput {
                    value: Amount::from_sats(5).expect("valid amount"),
                    script_pubkey: script(&[0x51]),
                    is_coinbase: false,
                },
                created_height: 0,
                created_median_time_past: 0,
            },
            TransactionInputContext {
                spent_output: SpentOutput {
                    value: Amount::from_sats(6).expect("valid amount"),
                    script_pubkey: script(&[0x52]),
                    is_coinbase: false,
                },
                created_height: 0,
                created_median_time_past: 0,
            },
        ],
        spend_height: 1,
        block_time: 0,
        median_time_past: 0,
        verify_flags: ScriptVerifyFlags::WITNESS,
        consensus_params: ConsensusParams::default(),
    };
    let precomputed = context.precompute(&transaction).expect("precompute");

    assert!(SigHashType::DEFAULT.is_default());
    assert!(SigHashType::from_u32(0x81).is_anyone_can_pay());
    assert_eq!(SigHashType::from_u32(0x82).base_type(), 0x02);

    let out_of_range = legacy_sighash(
        &script(&[0x51]),
        &transaction,
        transaction.inputs.len(),
        SigHashType::ALL,
    );
    let mut one = [0_u8; 32];
    one[0] = 1;
    assert_eq!(out_of_range.to_byte_array(), one);

    let none_hash = legacy_sighash(&script(&[0xab, 0x51]), &transaction, 1, SigHashType::NONE);
    let single_hash = legacy_sighash(&script(&[0xab, 0x51]), &transaction, 1, SigHashType::SINGLE);
    let acp_hash = legacy_sighash(
        &script(&[0xab, 0x51]),
        &transaction,
        1,
        SigHashType::from_u32(SigHashType::ALL.raw() | SigHashType::ANYONECANPAY),
    );
    assert_ne!(none_hash, single_hash);
    assert_ne!(single_hash, acp_hash);

    let segwit_none = segwit_v0_sighash(
        &script(&[0x51]),
        &transaction,
        1,
        &context.inputs[1],
        SigHashType::NONE,
        &precomputed,
    );
    let segwit_single = segwit_v0_sighash(
        &script(&[0x51]),
        &transaction,
        1,
        &context.inputs[1],
        SigHashType::SINGLE,
        &precomputed,
    );
    let segwit_anyone_can_pay = segwit_v0_sighash(
        &script(&[0x51]),
        &transaction,
        1,
        &context.inputs[1],
        SigHashType::from_u32(SigHashType::ALL.raw() | SigHashType::ANYONECANPAY),
        &precomputed,
    );
    let segwit_single_out_of_range = segwit_v0_sighash(
        &script(&[0x51]),
        &transaction,
        1,
        &context.inputs[1],
        SigHashType::from_u32(SigHashType::SINGLE.raw() | SigHashType::ANYONECANPAY),
        &precomputed,
    );
    let single_missing_output_transaction = Transaction {
        inputs: transaction.inputs.clone(),
        outputs: vec![transaction.outputs[0].clone()],
        ..transaction.clone()
    };
    let single_missing_output_context = TransactionValidationContext {
        inputs: context.inputs.clone(),
        ..context.clone()
    };
    let single_missing_output_precomputed = single_missing_output_context
        .precompute(&single_missing_output_transaction)
        .expect("precompute");
    let segwit_single_missing_output = segwit_v0_sighash(
        &script(&[0x51]),
        &single_missing_output_transaction,
        1,
        &single_missing_output_context.inputs[1],
        SigHashType::SINGLE,
        &single_missing_output_precomputed,
    );
    assert_ne!(segwit_none, segwit_single);
    assert_ne!(segwit_single, segwit_anyone_can_pay);
    assert_ne!(segwit_single, segwit_single_out_of_range);
    assert_ne!(segwit_single_missing_output, segwit_single);
    assert_ne!(segwit_single_missing_output, segwit_anyone_can_pay);

    let mut compact = Vec::new();
    super::write_compact_size(&mut compact, 253);
    super::write_compact_size(&mut compact, 65_536);
    super::write_compact_size(&mut compact, u64::MAX);
    assert_eq!(compact[0], 0xfd);
    assert_eq!(compact[3], 0xfe);
    assert_eq!(compact[8], 0xff);

    let default_type = SigHashType::DEFAULT;
    assert!(default_type.is_default());
}

#[test]
fn taproot_sighash_supports_keypath_and_tapscript_inputs() {
    let transaction = Transaction {
        version: 2,
        inputs: vec![TransactionInput {
            previous_output: open_bitcoin_primitives::OutPoint {
                txid: Txid::from_byte_array([9_u8; 32]),
                vout: 0,
            },
            script_sig: ScriptBuf::default(),
            sequence: 3,
            witness: Default::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(40).expect("valid amount"),
            script_pubkey: script(&[0x51]),
        }],
        lock_time: 0,
    };
    let context = TransactionValidationContext {
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
        verify_flags: ScriptVerifyFlags::TAPROOT,
        consensus_params: ConsensusParams::default(),
    };
    let keypath = taproot_sighash(
        &ScriptExecutionData::default(),
        &transaction,
        0,
        SigHashType::DEFAULT,
        SigVersion::Taproot,
        &context,
    )
    .expect("taproot keypath sighash");

    let tapscript_data = ScriptExecutionData {
        maybe_tapleaf_hash: Some(Hash32::from_byte_array([7_u8; 32])),
        maybe_codeseparator_position: Some(5),
        maybe_annex: Some(vec![0x50, 0x01]),
        ..ScriptExecutionData::default()
    };
    let tapscript = taproot_sighash(
        &tapscript_data,
        &transaction,
        0,
        SigHashType::SINGLE,
        SigVersion::Tapscript,
        &context,
    )
    .expect("taproot tapscript sighash");

    assert_ne!(keypath, tapscript);
    assert!(
        taproot_sighash(
            &ScriptExecutionData::default(),
            &transaction,
            0,
            SigHashType::from_u32(0x84),
            SigVersion::Taproot,
            &context,
        )
        .is_none()
    );
    assert!(
        taproot_sighash(
            &ScriptExecutionData::default(),
            &transaction,
            1,
            SigHashType::DEFAULT,
            SigVersion::Taproot,
            &context,
        )
        .is_none()
    );
}

#[test]
fn taproot_sighash_covers_anyone_can_pay_and_invalid_sigversion_paths() {
    let transaction = Transaction {
        version: 2,
        inputs: vec![
            TransactionInput {
                previous_output: open_bitcoin_primitives::OutPoint {
                    txid: Txid::from_byte_array([0x11_u8; 32]),
                    vout: 0,
                },
                script_sig: ScriptBuf::default(),
                sequence: 1,
                witness: Default::default(),
            },
            TransactionInput {
                previous_output: open_bitcoin_primitives::OutPoint {
                    txid: Txid::from_byte_array([0x22_u8; 32]),
                    vout: 1,
                },
                script_sig: ScriptBuf::default(),
                sequence: 2,
                witness: Default::default(),
            },
        ],
        outputs: vec![
            TransactionOutput {
                value: Amount::from_sats(40).expect("valid amount"),
                script_pubkey: script(&[0x51]),
            },
            TransactionOutput {
                value: Amount::from_sats(41).expect("valid amount"),
                script_pubkey: script(&[0x52]),
            },
        ],
        lock_time: 5,
    };
    let context = TransactionValidationContext {
        inputs: vec![
            TransactionInputContext {
                spent_output: SpentOutput {
                    value: Amount::from_sats(50).expect("valid amount"),
                    script_pubkey: script(&[0x51]),
                    is_coinbase: false,
                },
                created_height: 0,
                created_median_time_past: 0,
            },
            TransactionInputContext {
                spent_output: SpentOutput {
                    value: Amount::from_sats(60).expect("valid amount"),
                    script_pubkey: script(&[0x52]),
                    is_coinbase: false,
                },
                created_height: 0,
                created_median_time_past: 0,
            },
        ],
        spend_height: 1,
        block_time: 0,
        median_time_past: 0,
        verify_flags: ScriptVerifyFlags::TAPROOT,
        consensus_params: ConsensusParams::default(),
    };
    let execution_data = ScriptExecutionData {
        maybe_annex: Some(vec![0x50, 0x01]),
        ..ScriptExecutionData::default()
    };

    let default_hash = taproot_sighash(
        &execution_data,
        &transaction,
        1,
        SigHashType::DEFAULT,
        SigVersion::Taproot,
        &context,
    )
    .expect("default taproot sighash");
    let anyone_can_pay_hash = taproot_sighash(
        &execution_data,
        &transaction,
        1,
        SigHashType::from_u32(SigHashType::ALL.raw() | SigHashType::ANYONECANPAY),
        SigVersion::Taproot,
        &context,
    )
    .expect("ANYONECANPAY taproot sighash");

    assert_ne!(default_hash, anyone_can_pay_hash);
    assert!(
        taproot_sighash(
            &execution_data,
            &transaction,
            1,
            SigHashType::DEFAULT,
            SigVersion::Base,
            &context,
        )
        .is_none()
    );
}
