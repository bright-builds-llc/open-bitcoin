use open_bitcoin_primitives::{Hash32, ScriptBuf, Transaction, TransactionOutput};

use crate::context::{PrecomputedTransactionData, TransactionInputContext};
use crate::crypto::{Sha256, double_sha256};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SigVersion {
    Base,
    WitnessV0,
    Taproot,
    Tapscript,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SigHashType(u32);

impl SigHashType {
    pub const DEFAULT: Self = Self(0);
    pub const ALL: Self = Self(1);
    pub const NONE: Self = Self(2);
    pub const SINGLE: Self = Self(3);
    pub const ANYONECANPAY: u32 = 0x80;

    pub const fn from_u32(value: u32) -> Self {
        Self(value)
    }

    pub const fn raw(self) -> u32 {
        self.0
    }

    pub const fn base_type(self) -> u32 {
        self.0 & 0x1f
    }

    pub const fn is_anyone_can_pay(self) -> bool {
        (self.0 & Self::ANYONECANPAY) != 0
    }

    pub const fn is_default(self) -> bool {
        self.0 == 0
    }
}

pub fn legacy_sighash(
    script_code: &ScriptBuf,
    transaction: &Transaction,
    input_index: usize,
    sighash_type: SigHashType,
) -> Hash32 {
    if input_index >= transaction.inputs.len() {
        return Hash32::from_byte_array(one_hash());
    }

    let mut encoded = Vec::new();
    encoded.extend_from_slice(&transaction.version.to_le_bytes());

    let selected_inputs: Vec<usize> = if sighash_type.is_anyone_can_pay() {
        vec![input_index]
    } else {
        (0..transaction.inputs.len()).collect()
    };
    write_compact_size(&mut encoded, selected_inputs.len() as u64);
    for &index in &selected_inputs {
        let input = &transaction.inputs[index];
        encoded.extend_from_slice(input.previous_output.txid.as_bytes());
        encoded.extend_from_slice(&input.previous_output.vout.to_le_bytes());
        let script = if index == input_index {
            remove_codeseparators(script_code)
        } else {
            Vec::new()
        };
        write_script(&mut encoded, &script);
        let sequence = match sighash_type.base_type() {
            2 | 3 if index != input_index => 0,
            _ => input.sequence,
        };
        encoded.extend_from_slice(&sequence.to_le_bytes());
    }

    match sighash_type.base_type() {
        2 => write_compact_size(&mut encoded, 0),
        3 => {
            if input_index >= transaction.outputs.len() {
                return Hash32::from_byte_array(one_hash());
            }
            write_compact_size(&mut encoded, (input_index + 1) as u64);
            for _ in 0..input_index {
                write_null_output(&mut encoded);
            }
            write_output(&mut encoded, &transaction.outputs[input_index]);
        }
        _ => {
            write_compact_size(&mut encoded, transaction.outputs.len() as u64);
            for output in &transaction.outputs {
                write_output(&mut encoded, output);
            }
        }
    }

    encoded.extend_from_slice(&transaction.lock_time.to_le_bytes());
    encoded.extend_from_slice(&sighash_type.raw().to_le_bytes());
    Hash32::from_byte_array(double_sha256(&encoded))
}

pub fn segwit_v0_sighash(
    script_code: &ScriptBuf,
    transaction: &Transaction,
    input_index: usize,
    spent_output: &TransactionInputContext,
    sighash_type: SigHashType,
    precomputed: &PrecomputedTransactionData,
) -> Hash32 {
    let mut encoded = Vec::new();
    encoded.extend_from_slice(&transaction.version.to_le_bytes());

    if sighash_type.is_anyone_can_pay() {
        encoded.extend_from_slice(&[0_u8; 32]);
    } else {
        encoded.extend_from_slice(precomputed.hash_prevouts.as_bytes());
    }

    if sighash_type.is_anyone_can_pay() || matches!(sighash_type.base_type(), 2 | 3) {
        encoded.extend_from_slice(&[0_u8; 32]);
    } else {
        encoded.extend_from_slice(precomputed.hash_sequence.as_bytes());
    }

    let input = &transaction.inputs[input_index];
    encoded.extend_from_slice(input.previous_output.txid.as_bytes());
    encoded.extend_from_slice(&input.previous_output.vout.to_le_bytes());
    write_script(&mut encoded, script_code.as_bytes());
    encoded.extend_from_slice(&spent_output.spent_output.value.to_sats().to_le_bytes());
    encoded.extend_from_slice(&input.sequence.to_le_bytes());

    match sighash_type.base_type() {
        2 => encoded.extend_from_slice(&[0_u8; 32]),
        3 if input_index < transaction.outputs.len() => {
            let mut single_output = Vec::new();
            write_output(&mut single_output, &transaction.outputs[input_index]);
            encoded.extend_from_slice(&double_sha256(&single_output));
        }
        3 => encoded.extend_from_slice(&[0_u8; 32]),
        _ => encoded.extend_from_slice(precomputed.hash_outputs.as_bytes()),
    }

    encoded.extend_from_slice(&transaction.lock_time.to_le_bytes());
    encoded.extend_from_slice(&sighash_type.raw().to_le_bytes());
    Hash32::from_byte_array(double_sha256(&encoded))
}

pub fn taproot_tagged_hash(tag: &str, data: &[u8]) -> Hash32 {
    let tag_hash = Sha256::digest(tag.as_bytes());
    let mut preimage = Vec::with_capacity(tag_hash.len() * 2 + data.len());
    preimage.extend_from_slice(&tag_hash);
    preimage.extend_from_slice(&tag_hash);
    preimage.extend_from_slice(data);
    Hash32::from_byte_array(Sha256::digest(&preimage))
}

fn remove_codeseparators(script: &ScriptBuf) -> Vec<u8> {
    script
        .as_bytes()
        .iter()
        .copied()
        .filter(|byte| *byte != 0xab)
        .collect()
}

fn write_null_output(out: &mut Vec<u8>) {
    out.extend_from_slice(&(-1_i64).to_le_bytes());
    write_compact_size(out, 0);
}

fn write_output(out: &mut Vec<u8>, output: &TransactionOutput) {
    out.extend_from_slice(&output.value.to_sats().to_le_bytes());
    write_script(out, output.script_pubkey.as_bytes());
}

fn write_script(out: &mut Vec<u8>, script: &[u8]) {
    write_compact_size(out, script.len() as u64);
    out.extend_from_slice(script);
}

fn write_compact_size(out: &mut Vec<u8>, value: u64) {
    match value {
        0..=252 => out.push(value as u8),
        253..=0xffff => {
            out.push(0xfd);
            out.extend_from_slice(&(value as u16).to_le_bytes());
        }
        0x1_0000..=0xffff_ffff => {
            out.push(0xfe);
            out.extend_from_slice(&(value as u32).to_le_bytes());
        }
        _ => {
            out.push(0xff);
            out.extend_from_slice(&value.to_le_bytes());
        }
    }
}

fn one_hash() -> [u8; 32] {
    let mut one = [0_u8; 32];
    one[0] = 1;
    one
}

#[cfg(test)]
mod tests {
    use super::{SigHashType, SigVersion, legacy_sighash, segwit_v0_sighash, taproot_tagged_hash};
    use crate::context::{
        ConsensusParams, ScriptVerifyFlags, SpentOutput, TransactionInputContext,
        TransactionValidationContext,
    };
    use open_bitcoin_codec::parse_transaction_without_witness;
    use open_bitcoin_primitives::{
        Amount, ScriptBuf, Transaction, TransactionInput, TransactionOutput, Txid,
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
        let single_hash =
            legacy_sighash(&script(&[0xab, 0x51]), &transaction, 1, SigHashType::SINGLE);
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
}
