use open_bitcoin_primitives::{ScriptBuf, ScriptWitness};

use crate::classify::{
    ScriptPubKeyType, classify_script_pubkey, extract_redeem_script, is_push_only,
};
use crate::context::ScriptVerifyFlags;

use super::ScriptError;
use super::encoding::read_instruction;
use super::opcodes::{
    MAX_PUBKEYS_PER_MULTISIG, OP_CHECKMULTISIG, OP_CHECKMULTISIGVERIFY, OP_CHECKSIG,
    OP_CHECKSIGVERIFY, decode_small_int_opcode,
};

pub(super) fn count_legacy_sigops(script: &ScriptBuf) -> Result<usize, ScriptError> {
    count_sigops(script, false)
}

pub(super) fn count_p2sh_sigops(
    script_sig: &ScriptBuf,
    script_pubkey: &ScriptBuf,
) -> Result<usize, ScriptError> {
    if !matches!(
        classify_script_pubkey(script_pubkey),
        ScriptPubKeyType::PayToScriptHash(_)
    ) {
        return Ok(0);
    }
    if !is_push_only(script_sig) {
        return Ok(0);
    }
    let Some(redeem_script) = extract_redeem_script(script_sig) else {
        return Ok(0);
    };
    count_sigops(&redeem_script, true)
}

pub(super) fn count_witness_sigops(
    script_sig: &ScriptBuf,
    script_pubkey: &ScriptBuf,
    witness: &ScriptWitness,
    verify_flags: ScriptVerifyFlags,
) -> Result<usize, ScriptError> {
    if !verify_flags.contains(ScriptVerifyFlags::WITNESS) {
        return Ok(0);
    }

    let script_type = classify_script_pubkey(script_pubkey);
    if let Some(sigops) = witness_sigops_for_type(&script_type, witness)? {
        return Ok(sigops);
    }

    if matches!(script_type, ScriptPubKeyType::PayToScriptHash(_)) && is_push_only(script_sig) {
        let Some(redeem_script) = extract_redeem_script(script_sig) else {
            return Ok(0);
        };
        let redeem_type = classify_script_pubkey(&redeem_script);
        if let Some(sigops) = witness_sigops_for_type(&redeem_type, witness)? {
            return Ok(sigops);
        }
    }

    Ok(0)
}

fn count_sigops(script: &ScriptBuf, accurate: bool) -> Result<usize, ScriptError> {
    let bytes = script.as_bytes();
    let mut pc = 0;
    let mut sigops = 0;
    let mut last_opcode = None;
    while pc < bytes.len() {
        let instruction = read_instruction(bytes, &mut pc)?;
        match instruction.opcode {
            OP_CHECKSIG | OP_CHECKSIGVERIFY => sigops += 1,
            OP_CHECKMULTISIG | OP_CHECKMULTISIGVERIFY => {
                sigops += if accurate {
                    last_opcode
                        .and_then(decode_small_int_opcode)
                        .unwrap_or(MAX_PUBKEYS_PER_MULTISIG)
                } else {
                    MAX_PUBKEYS_PER_MULTISIG
                };
            }
            _ => {}
        }
        last_opcode = Some(instruction.opcode);
    }
    Ok(sigops)
}

pub(super) fn witness_sigops_for_type(
    script_type: &ScriptPubKeyType,
    witness: &ScriptWitness,
) -> Result<Option<usize>, ScriptError> {
    match script_type {
        ScriptPubKeyType::WitnessV0KeyHash(_) => Ok(Some(1)),
        ScriptPubKeyType::WitnessV0ScriptHash(_) if !witness.stack().is_empty() => {
            let script_bytes = witness
                .stack()
                .last()
                .expect("witness stack is non-empty under the guard above");
            let witness_script = ScriptBuf::from_bytes(script_bytes.clone())
                .map_err(|_| ScriptError::WitnessProgramMismatch)?;
            Ok(Some(count_sigops(&witness_script, true)?))
        }
        _ => Ok(None),
    }
}
