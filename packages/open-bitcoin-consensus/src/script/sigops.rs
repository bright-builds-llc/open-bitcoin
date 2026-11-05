// Parity breadcrumbs:
// - packages/bitcoin-knots/src/script/script.h
// - packages/bitcoin-knots/src/script/script.cpp
// - packages/bitcoin-knots/src/script/interpreter.cpp
// - packages/bitcoin-knots/src/script/script_error.h
// - packages/bitcoin-knots/src/test/data/script_tests.json

use open_bitcoin_primitives::{ScriptBuf, ScriptWitness};

use crate::classify::{ScriptPubKeyType, classify_script_pubkey};
use crate::context::ScriptVerifyFlags;

#[cfg(test)]
use super::ScriptError;
use super::encoding::read_instruction_syntax;
use super::opcodes::{
    MAX_PUBKEYS_PER_MULTISIG, OP_CHECKMULTISIG, OP_CHECKMULTISIGVERIFY, OP_CHECKSIG,
    OP_CHECKSIGVERIFY, decode_small_int_opcode,
};

pub(super) fn count_legacy_sigops(script: &ScriptBuf) -> usize {
    count_sigops(script.as_bytes(), false)
}

pub(super) fn count_p2sh_sigops(script_sig: &ScriptBuf, script_pubkey: &ScriptBuf) -> usize {
    if !matches!(
        classify_script_pubkey(script_pubkey),
        ScriptPubKeyType::PayToScriptHash(_)
    ) {
        return 0;
    }
    let Some(redeem_bytes) = maybe_last_raw_push(script_sig) else {
        return 0;
    };
    count_sigops(&redeem_bytes, true)
}

pub(super) fn count_witness_sigops(
    script_sig: &ScriptBuf,
    script_pubkey: &ScriptBuf,
    witness: &ScriptWitness,
    verify_flags: ScriptVerifyFlags,
) -> usize {
    if !verify_flags.contains(ScriptVerifyFlags::WITNESS) {
        return 0;
    }

    let script_type = classify_script_pubkey(script_pubkey);
    if let Some(sigops) = witness_sigops_for_type_count(&script_type, witness) {
        return sigops;
    }

    if matches!(script_type, ScriptPubKeyType::PayToScriptHash(_)) {
        let Some(redeem_bytes) = maybe_last_raw_push(script_sig) else {
            return 0;
        };
        let redeem_script = ScriptBuf::from_serialized_bytes(redeem_bytes);
        let redeem_type = classify_script_pubkey(&redeem_script);
        if let Some(sigops) = witness_sigops_for_type_count(&redeem_type, witness) {
            return sigops;
        }
    }

    0
}

fn count_sigops(bytes: &[u8], accurate: bool) -> usize {
    let mut pc = 0;
    let mut sigops = 0;
    let mut last_opcode = None;
    while pc < bytes.len() {
        let Ok(instruction) = read_instruction_syntax(bytes, &mut pc) else {
            break;
        };
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
    sigops
}

// GetOp clears raw data for every opcode; small integer opcodes do not produce
// execution-stack bytes here. OP_RESERVED is syntactically push-only in the pin.
fn maybe_last_raw_push(script: &ScriptBuf) -> Option<Vec<u8>> {
    let mut pc = 0;
    let mut data = Vec::new();
    while pc < script.as_bytes().len() {
        let instruction = read_instruction_syntax(script.as_bytes(), &mut pc).ok()?;
        if instruction.opcode > 0x60 {
            return None;
        }
        data = instruction.maybe_data.unwrap_or_default();
    }
    Some(data)
}

fn witness_sigops_for_type_count(
    script_type: &ScriptPubKeyType,
    witness: &ScriptWitness,
) -> Option<usize> {
    match script_type {
        ScriptPubKeyType::WitnessV0KeyHash(_) => Some(1),
        ScriptPubKeyType::WitnessV0ScriptHash(_) => {
            let script_bytes = witness.stack().last()?;
            Some(count_sigops(script_bytes, true))
        }
        _ => None,
    }
}

#[cfg(test)]
pub(super) fn witness_sigops_for_type(
    script_type: &ScriptPubKeyType,
    witness: &ScriptWitness,
) -> Result<Option<usize>, ScriptError> {
    Ok(witness_sigops_for_type_count(script_type, witness))
}

#[cfg(test)]
mod serialized_tests {
    use super::*;

    #[test]
    fn phase167_serialized_script_paths_nested_wide_redeem_is_non_witness() {
        // Arrange
        let mut encoded = vec![0x4d, 0x11, 0x27];
        encoded.extend(vec![0xac; 10_001]);
        let script_sig = ScriptBuf::from_serialized_bytes(encoded);
        let mut p2sh = vec![0xa9, 20];
        p2sh.extend_from_slice(&[0; 20]);
        p2sh.push(0x87);
        let script_pubkey = ScriptBuf::from_bytes(p2sh).expect("fixed p2sh");
        // Act
        let witness = count_witness_sigops(
            &script_sig,
            &script_pubkey,
            &ScriptWitness::default(),
            ScriptVerifyFlags::WITNESS,
        );
        let redeem = count_p2sh_sigops(&script_sig, &script_pubkey);
        // Assert
        assert_eq!(witness, 0);
        assert_eq!(redeem, 10_001);
        assert_eq!(
            super::super::count_witness_sigops(
                &script_sig,
                &script_pubkey,
                &ScriptWitness::default(),
                ScriptVerifyFlags::WITNESS,
            ),
            Ok(witness)
        );
        assert_eq!(
            super::super::witness_sigops_count(
                &script_sig,
                &script_pubkey,
                &ScriptWitness::default(),
                ScriptVerifyFlags::WITNESS,
            ),
            witness
        );
        assert_eq!(
            super::super::count_p2sh_sigops(&script_sig, &script_pubkey),
            Ok(redeem)
        );
        assert_eq!(
            super::super::p2sh_sigops_count(&script_sig, &script_pubkey),
            redeem
        );
    }

    #[test]
    fn phase167_serialized_script_paths_wide_legacy_counter_skips_raw_pushes() {
        // Arrange
        let mut bytes = vec![0x4d, 0x10, 0x27];
        bytes.extend(vec![0xac; 10_000]);
        bytes.extend_from_slice(&[0xac, 0xae]);
        let script = ScriptBuf::from_serialized_bytes(bytes);
        // Act
        let result = count_legacy_sigops(&script);
        // Assert
        assert_eq!(result, 21);
        assert_eq!(super::super::count_legacy_sigops(&script), Ok(result));
        assert_eq!(super::super::legacy_sigops_count(&script), result);
    }

    #[test]
    fn malformed_legacy_syntax_has_the_same_zero_count_on_both_public_paths() {
        // Arrange
        let malformed = ScriptBuf::from_serialized_bytes(vec![0x4c]);

        // Act / Assert: counting stops at the truncated push without executing it.
        assert_eq!(super::super::count_legacy_sigops(&malformed), Ok(0));
        assert_eq!(super::super::legacy_sigops_count(&malformed), 0);
    }

    #[test]
    fn p2sh_count_paths_agree_for_push_only_missing_and_malformed_redeem() {
        // Arrange
        let mut p2sh = vec![0xa9, 20];
        p2sh.extend_from_slice(&[0; 20]);
        p2sh.push(0x87);
        let script_pubkey = ScriptBuf::from_serialized_bytes(p2sh);
        let cases = [
            (ScriptBuf::from_serialized_bytes(vec![1, 0xac]), 1),
            (ScriptBuf::default(), 0),
            (ScriptBuf::from_serialized_bytes(vec![0x4c]), 0),
        ];

        // Act / Assert
        for (script_sig, expected) in cases {
            assert_eq!(
                super::super::count_p2sh_sigops(&script_sig, &script_pubkey),
                Ok(expected)
            );
            assert_eq!(
                super::super::p2sh_sigops_count(&script_sig, &script_pubkey),
                expected
            );
        }
    }

    #[test]
    fn witness_count_paths_agree_for_direct_nested_and_disabled_flags() {
        // Arrange
        let mut witness_program = vec![0x00, 20];
        witness_program.extend_from_slice(&[0; 20]);
        let direct = ScriptBuf::from_serialized_bytes(witness_program.clone());
        let mut p2sh = vec![0xa9, 20];
        p2sh.extend_from_slice(&[0; 20]);
        p2sh.push(0x87);
        let nested = ScriptBuf::from_serialized_bytes(p2sh);
        let mut push_program = vec![22];
        push_program.extend_from_slice(&witness_program);
        let nested_sig = ScriptBuf::from_serialized_bytes(push_program);
        let empty_sig = ScriptBuf::default();
        let witness = ScriptWitness::default();
        let cases = [
            (&empty_sig, &direct, ScriptVerifyFlags::WITNESS, 1),
            (&nested_sig, &nested, ScriptVerifyFlags::WITNESS, 1),
            (&empty_sig, &direct, ScriptVerifyFlags::NONE, 0),
        ];

        // Act / Assert
        for (script_sig, script_pubkey, flags, expected) in cases {
            assert_eq!(
                super::super::count_witness_sigops(script_sig, script_pubkey, &witness, flags),
                Ok(expected)
            );
            assert_eq!(
                super::super::witness_sigops_count(script_sig, script_pubkey, &witness, flags),
                expected
            );
        }
    }
}
