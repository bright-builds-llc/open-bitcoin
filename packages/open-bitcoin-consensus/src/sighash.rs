use open_bitcoin_primitives::{Hash32, ScriptBuf, Transaction, TransactionOutput};

use crate::context::{
    PrecomputedTransactionData, ScriptExecutionData, TransactionInputContext,
    TransactionValidationContext,
};
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

pub fn taproot_sighash(
    execution_data: &ScriptExecutionData,
    transaction: &Transaction,
    input_index: usize,
    sighash_type: SigHashType,
    sig_version: SigVersion,
    context: &TransactionValidationContext,
) -> Option<Hash32> {
    if input_index >= transaction.inputs.len() || context.inputs.len() != transaction.inputs.len() {
        return None;
    }
    if !is_valid_taproot_sighash_type(sighash_type) {
        return None;
    }

    let ext_flag = match sig_version {
        SigVersion::Taproot => 0_u8,
        SigVersion::Tapscript => 1_u8,
        _ => return None,
    };

    let output_type = if sighash_type.is_default() {
        SigHashType::ALL.raw()
    } else {
        sighash_type.base_type()
    };
    let input_type = sighash_type.raw() & SigHashType::ANYONECANPAY;

    let mut preimage = Vec::new();
    preimage.push(0_u8);
    preimage.push(sighash_type.raw() as u8);
    preimage.extend_from_slice(&transaction.version.to_le_bytes());
    preimage.extend_from_slice(&transaction.lock_time.to_le_bytes());

    if input_type != SigHashType::ANYONECANPAY {
        preimage.extend_from_slice(&sha256_prevouts(transaction).to_byte_array());
        preimage.extend_from_slice(&sha256_spent_amounts(context).to_byte_array());
        preimage.extend_from_slice(&sha256_spent_scripts(context).to_byte_array());
        preimage.extend_from_slice(&sha256_sequences(transaction).to_byte_array());
    }
    if output_type == SigHashType::ALL.raw() {
        preimage.extend_from_slice(&sha256_outputs(transaction).to_byte_array());
    }

    let maybe_annex_hash = execution_data
        .maybe_annex
        .as_ref()
        .map(|annex| sha256_annex(annex));
    let spend_type = (ext_flag << 1) | u8::from(maybe_annex_hash.is_some());
    preimage.push(spend_type);

    if input_type == SigHashType::ANYONECANPAY {
        let input = &transaction.inputs[input_index];
        preimage.extend_from_slice(input.previous_output.txid.as_bytes());
        preimage.extend_from_slice(&input.previous_output.vout.to_le_bytes());
        preimage.extend_from_slice(
            &context.inputs[input_index]
                .spent_output
                .value
                .to_sats()
                .to_le_bytes(),
        );
        write_script(
            &mut preimage,
            context.inputs[input_index]
                .spent_output
                .script_pubkey
                .as_bytes(),
        );
        preimage.extend_from_slice(&input.sequence.to_le_bytes());
    } else {
        preimage.extend_from_slice(&(input_index as u32).to_le_bytes());
    }
    if let Some(annex_hash) = maybe_annex_hash {
        preimage.extend_from_slice(annex_hash.as_bytes());
    }

    if output_type == SigHashType::SINGLE.raw() {
        let output = transaction.outputs.get(input_index)?;
        preimage.extend_from_slice(&sha256_single_output(output).to_byte_array());
    }

    if sig_version == SigVersion::Tapscript {
        let tapleaf_hash = execution_data.maybe_tapleaf_hash?;
        preimage.extend_from_slice(tapleaf_hash.as_bytes());
        preimage.push(0_u8);
        preimage.extend_from_slice(
            &execution_data
                .maybe_codeseparator_position
                .unwrap_or(u32::MAX)
                .to_le_bytes(),
        );
    }

    Some(taproot_tagged_hash("TapSighash", &preimage))
}

fn remove_codeseparators(script: &ScriptBuf) -> Vec<u8> {
    script
        .as_bytes()
        .iter()
        .copied()
        .filter(|byte| *byte != 0xab)
        .collect()
}

fn is_valid_taproot_sighash_type(sighash_type: SigHashType) -> bool {
    matches!(sighash_type.raw(), 0..=3 | 0x81..=0x83)
}

fn sha256_prevouts(transaction: &Transaction) -> Hash32 {
    let mut bytes = Vec::with_capacity(transaction.inputs.len() * 36);
    for input in &transaction.inputs {
        bytes.extend_from_slice(input.previous_output.txid.as_bytes());
        bytes.extend_from_slice(&input.previous_output.vout.to_le_bytes());
    }
    Hash32::from_byte_array(Sha256::digest(&bytes))
}

fn sha256_spent_amounts(context: &TransactionValidationContext) -> Hash32 {
    let mut bytes = Vec::with_capacity(context.inputs.len() * 8);
    for input in &context.inputs {
        bytes.extend_from_slice(&input.spent_output.value.to_sats().to_le_bytes());
    }
    Hash32::from_byte_array(Sha256::digest(&bytes))
}

fn sha256_spent_scripts(context: &TransactionValidationContext) -> Hash32 {
    let mut bytes = Vec::new();
    for input in &context.inputs {
        write_script(&mut bytes, input.spent_output.script_pubkey.as_bytes());
    }
    Hash32::from_byte_array(Sha256::digest(&bytes))
}

fn sha256_sequences(transaction: &Transaction) -> Hash32 {
    let mut bytes = Vec::with_capacity(transaction.inputs.len() * 4);
    for input in &transaction.inputs {
        bytes.extend_from_slice(&input.sequence.to_le_bytes());
    }
    Hash32::from_byte_array(Sha256::digest(&bytes))
}

fn sha256_outputs(transaction: &Transaction) -> Hash32 {
    let mut bytes = Vec::new();
    for output in &transaction.outputs {
        write_output(&mut bytes, output);
    }
    Hash32::from_byte_array(Sha256::digest(&bytes))
}

fn sha256_single_output(output: &TransactionOutput) -> Hash32 {
    let mut bytes = Vec::new();
    write_output(&mut bytes, output);
    Hash32::from_byte_array(Sha256::digest(&bytes))
}

fn sha256_annex(annex: &[u8]) -> Hash32 {
    let mut bytes = Vec::new();
    write_compact_size(&mut bytes, annex.len() as u64);
    bytes.extend_from_slice(annex);
    Hash32::from_byte_array(Sha256::digest(&bytes))
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
mod tests;
