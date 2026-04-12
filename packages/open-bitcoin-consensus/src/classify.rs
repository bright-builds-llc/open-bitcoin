use open_bitcoin_primitives::ScriptBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScriptPubKeyType {
    PayToPubKey {
        compressed: bool,
        pubkey: Vec<u8>,
    },
    PayToPubKeyHash([u8; 20]),
    PayToScriptHash([u8; 20]),
    WitnessV0KeyHash([u8; 20]),
    WitnessV0ScriptHash([u8; 32]),
    WitnessV1Taproot([u8; 32]),
    WitnessUnknown {
        version: u8,
        program: Vec<u8>,
    },
    Multisig {
        required_signatures: usize,
        pubkeys: Vec<Vec<u8>>,
    },
    NonStandard,
}

pub fn classify_script_pubkey(script: &ScriptBuf) -> ScriptPubKeyType {
    let bytes = script.as_bytes();
    if let Some(classified) = classify_pay_to_pubkey(bytes) {
        return classified;
    }
    if let Some(hash) = extract_p2pkh_hash(bytes) {
        return ScriptPubKeyType::PayToPubKeyHash(hash);
    }
    if let Some(hash) = extract_p2sh_hash(bytes) {
        return ScriptPubKeyType::PayToScriptHash(hash);
    }
    if let Some(classified) = classify_witness_program(bytes) {
        return classified;
    }
    if let Some(classified) = classify_multisig(bytes) {
        return classified;
    }

    ScriptPubKeyType::NonStandard
}

pub fn is_push_only(script: &ScriptBuf) -> bool {
    let bytes = script.as_bytes();
    let mut pc = 0;
    while pc < bytes.len() {
        let opcode = bytes[pc];
        pc += 1;
        let maybe_push_len = match opcode {
            0x00..=0x4b => Some(opcode as usize),
            0x4c => {
                let Some(length) = bytes.get(pc) else {
                    return false;
                };
                pc += 1;
                Some(usize::from(*length))
            }
            0x4d => {
                let Some(length_bytes) = bytes.get(pc..pc + 2) else {
                    return false;
                };
                pc += 2;
                Some(usize::from(u16::from_le_bytes([
                    length_bytes[0],
                    length_bytes[1],
                ])))
            }
            0x4e => {
                let Some(length_bytes) = bytes.get(pc..pc + 4) else {
                    return false;
                };
                pc += 4;
                Some(u32::from_le_bytes([
                    length_bytes[0],
                    length_bytes[1],
                    length_bytes[2],
                    length_bytes[3],
                ]) as usize)
            }
            0x4f | 0x51..=0x60 => None,
            _ => return false,
        };

        if let Some(push_len) = maybe_push_len {
            let end = pc + push_len;
            if end > bytes.len() {
                return false;
            }
            pc = end;
        }
    }

    true
}

pub fn extract_script_sig_pushes(script: &ScriptBuf) -> Option<Vec<Vec<u8>>> {
    let bytes = script.as_bytes();
    let mut pushes = Vec::new();
    let mut pc = 0;

    while pc < bytes.len() {
        let opcode = *bytes.get(pc)?;
        pc += 1;
        match opcode {
            0x00..=0x4b => {
                let end = pc.checked_add(opcode as usize)?;
                pushes.push(bytes.get(pc..end)?.to_vec());
                pc = end;
            }
            0x4c => {
                let length = usize::from(*bytes.get(pc)?);
                pc += 1;
                let end = pc + length;
                pushes.push(bytes.get(pc..end)?.to_vec());
                pc = end;
            }
            0x4d => {
                let length_bytes = bytes.get(pc..pc + 2)?;
                pc += 2;
                let length = usize::from(u16::from_le_bytes([length_bytes[0], length_bytes[1]]));
                let end = pc + length;
                pushes.push(bytes.get(pc..end)?.to_vec());
                pc = end;
            }
            0x4e => {
                let length_bytes = bytes.get(pc..pc + 4)?;
                pc += 4;
                let length = u32::from_le_bytes([
                    length_bytes[0],
                    length_bytes[1],
                    length_bytes[2],
                    length_bytes[3],
                ]) as usize;
                let end = pc + length;
                pushes.push(bytes.get(pc..end)?.to_vec());
                pc = end;
            }
            0x4f => pushes.push(vec![0x81]),
            0x51..=0x60 => pushes.push(vec![opcode - 0x50]),
            _ => return None,
        }
    }

    Some(pushes)
}

pub fn extract_redeem_script(script_sig: &ScriptBuf) -> Option<ScriptBuf> {
    let pushes = extract_script_sig_pushes(script_sig)?;
    let redeem_script = pushes.last()?.clone();
    ScriptBuf::from_bytes(redeem_script).ok()
}

fn classify_pay_to_pubkey(bytes: &[u8]) -> Option<ScriptPubKeyType> {
    match bytes {
        [33, pubkey @ .., 0xac] if pubkey.len() == 33 => Some(ScriptPubKeyType::PayToPubKey {
            compressed: true,
            pubkey: pubkey.to_vec(),
        }),
        [65, pubkey @ .., 0xac] if pubkey.len() == 65 => Some(ScriptPubKeyType::PayToPubKey {
            compressed: false,
            pubkey: pubkey.to_vec(),
        }),
        _ => None,
    }
}

fn extract_p2pkh_hash(bytes: &[u8]) -> Option<[u8; 20]> {
    match bytes {
        [0x76, 0xa9, 20, hash @ .., 0x88, 0xac] if hash.len() == 20 => Some(hash.try_into().ok()?),
        _ => None,
    }
}

fn extract_p2sh_hash(bytes: &[u8]) -> Option<[u8; 20]> {
    match bytes {
        [0xa9, 20, hash @ .., 0x87] if hash.len() == 20 => Some(hash.try_into().ok()?),
        _ => None,
    }
}

fn classify_witness_program(bytes: &[u8]) -> Option<ScriptPubKeyType> {
    if !(4..=42).contains(&bytes.len()) {
        return None;
    }
    let version = match bytes[0] {
        0x00 => 0,
        0x51..=0x60 => bytes[0] - 0x50,
        _ => return None,
    };
    let program_len = usize::from(bytes[1]);
    if program_len + 2 != bytes.len() || !(2..=40).contains(&program_len) {
        return None;
    }
    let program = &bytes[2..];

    Some(match (version, program_len) {
        (0, 20) => ScriptPubKeyType::WitnessV0KeyHash(program.try_into().ok()?),
        (0, 32) => ScriptPubKeyType::WitnessV0ScriptHash(program.try_into().ok()?),
        (1, 32) => ScriptPubKeyType::WitnessV1Taproot(program.try_into().ok()?),
        _ => ScriptPubKeyType::WitnessUnknown {
            version,
            program: program.to_vec(),
        },
    })
}

fn classify_multisig(bytes: &[u8]) -> Option<ScriptPubKeyType> {
    let (&last_opcode, body) = bytes.split_last()?;
    if last_opcode != 0xae || body.len() < 2 {
        return None;
    }

    let required = decode_small_int(body[0])?;
    let total = decode_small_int(*body.last()?)?;
    let mut pc = 1;
    let mut pubkeys = Vec::new();
    while pc < body.len() - 1 {
        let key_len = usize::from(*body.get(pc)?);
        pc += 1;
        if !(key_len == 33 || key_len == 65) {
            return None;
        }
        let end = pc.checked_add(key_len)?;
        pubkeys.push(body.get(pc..end)?.to_vec());
        pc = end;
    }
    if pubkeys.len() != total || required > total {
        return None;
    }

    Some(ScriptPubKeyType::Multisig {
        required_signatures: required,
        pubkeys,
    })
}

fn decode_small_int(opcode: u8) -> Option<usize> {
    match opcode {
        0x00 => Some(0),
        0x51..=0x60 => Some(usize::from(opcode - 0x50)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ScriptPubKeyType, classify_script_pubkey, extract_redeem_script, extract_script_sig_pushes,
        is_push_only,
    };
    use open_bitcoin_primitives::ScriptBuf;

    fn script(bytes: &[u8]) -> ScriptBuf {
        ScriptBuf::from_bytes(bytes.to_vec()).expect("valid script")
    }

    #[test]
    fn classify_standard_script_pubkeys() {
        let p2pkh = script(&[
            0x76, 0xa9, 20, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0x88, 0xac,
        ]);
        let p2sh = script(&[
            0xa9, 20, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0x87,
        ]);
        let p2wpkh = script(&[
            0x00, 20, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
        ]);
        let p2wsh = script(&[
            0x00, 32, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
            4, 4, 4, 4, 4, 4,
        ]);
        let p2tr = script(&[
            0x51, 32, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
            5, 5, 5, 5, 5, 5,
        ]);

        assert!(matches!(
            classify_script_pubkey(&p2pkh),
            ScriptPubKeyType::PayToPubKeyHash(_)
        ));
        assert!(matches!(
            classify_script_pubkey(&p2sh),
            ScriptPubKeyType::PayToScriptHash(_)
        ));
        assert!(matches!(
            classify_script_pubkey(&p2wpkh),
            ScriptPubKeyType::WitnessV0KeyHash(_)
        ));
        assert!(matches!(
            classify_script_pubkey(&p2wsh),
            ScriptPubKeyType::WitnessV0ScriptHash(_)
        ));
        assert!(matches!(
            classify_script_pubkey(&p2tr),
            ScriptPubKeyType::WitnessV1Taproot(_)
        ));
    }

    #[test]
    fn classify_pay_to_pubkey_and_multisig() {
        let p2pk = script(&[
            33, 2, 6, 110, 125, 137, 102, 181, 197, 85, 175, 88, 5, 152, 157, 169, 251, 248, 219,
            149, 225, 86, 49, 206, 53, 140, 58, 23, 16, 201, 98, 103, 144, 99, 0xac,
        ]);
        let multisig = script(&[
            0x51, 33, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 0x51, 0xae,
        ]);

        assert!(matches!(
            classify_script_pubkey(&p2pk),
            ScriptPubKeyType::PayToPubKey { .. }
        ));
        assert!(matches!(
            classify_script_pubkey(&multisig),
            ScriptPubKeyType::Multisig {
                required_signatures: 1,
                ..
            }
        ));
    }

    #[test]
    fn push_only_helpers_extract_pushes_and_redeem_script() {
        let script_sig = script(&[0x01, 0x51, 0x02, 0x51, 0x21]);
        assert!(is_push_only(&script_sig));
        assert_eq!(
            extract_script_sig_pushes(&script_sig).expect("pushes"),
            vec![vec![0x51], vec![0x51, 0x21]]
        );
        assert_eq!(
            extract_redeem_script(&script_sig)
                .expect("redeem script")
                .as_bytes(),
            &[0x51, 0x21]
        );

        let not_push_only = script(&[0x76]);
        assert!(!is_push_only(&not_push_only));
        assert!(extract_script_sig_pushes(&not_push_only).is_none());
    }

    #[test]
    fn classify_fallback_and_pushdata_variants_are_exercised() {
        let uncompressed_p2pk = {
            let mut bytes = vec![65, 4];
            bytes.extend(vec![1_u8; 64]);
            bytes.push(0xac);
            script(&bytes)
        };
        assert!(matches!(
            classify_script_pubkey(&uncompressed_p2pk),
            ScriptPubKeyType::PayToPubKey {
                compressed: false,
                ..
            }
        ));

        let witness_unknown = script(&[0x52, 0x02, 0xaa, 0xbb]);
        assert!(matches!(
            classify_script_pubkey(&witness_unknown),
            ScriptPubKeyType::WitnessUnknown { version: 2, .. }
        ));

        let non_standard = script(&[0x6a, 0x51]);
        assert!(matches!(
            classify_script_pubkey(&non_standard),
            ScriptPubKeyType::NonStandard
        ));

        let pushdata_script = script(&[
            0x4c, 0x01, 0x51, 0x4d, 0x01, 0x00, 0x52, 0x4e, 0x01, 0x00, 0x00, 0x00, 0x53, 0x4f,
        ]);
        assert!(is_push_only(&pushdata_script));
        assert_eq!(
            extract_script_sig_pushes(&pushdata_script).expect("pushdata pushes"),
            vec![vec![0x51], vec![0x52], vec![0x53], vec![0x81]]
        );

        assert!(!is_push_only(&script(&[0x4c])));
        assert!(!is_push_only(&script(&[0x4d, 0x01])));
        assert!(!is_push_only(&script(&[0x4e, 0x01, 0x00, 0x00])));

        assert!(extract_script_sig_pushes(&script(&[0x4c])).is_none());
        assert!(extract_script_sig_pushes(&script(&[0x4d, 0x01])).is_none());
        assert!(extract_script_sig_pushes(&script(&[0x4e, 0x01, 0x00, 0x00])).is_none());

        let oversized_push = script(&[0x4c, 0xff]);
        assert!(!is_push_only(&oversized_push));
        let overflow_push = script(&[0x4e, 0xff, 0xff, 0xff, 0xff]);
        assert!(!is_push_only(&overflow_push));
        let end_past_len = script(&[0x01]);
        assert!(!is_push_only(&end_past_len));

        let bad_multisig = script(&[0x52, 0x21, 0x02, 0x01, 0x01, 0x51, 0xae]);
        assert!(matches!(
            classify_script_pubkey(&bad_multisig),
            ScriptPubKeyType::NonStandard
        ));

        let invalid_version_witness = script(&[0x50, 0x02, 0xaa, 0xbb]);
        assert!(matches!(
            classify_script_pubkey(&invalid_version_witness),
            ScriptPubKeyType::NonStandard
        ));

        let invalid_key_multisig = script(&[0x51, 0x20, 0x02, 0x01, 0x51, 0xae]);
        assert!(matches!(
            classify_script_pubkey(&invalid_key_multisig),
            ScriptPubKeyType::NonStandard
        ));

        let invalid_count_multisig = script(&[
            0x52, 33, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 0x51, 0xae,
        ]);
        assert!(matches!(
            classify_script_pubkey(&invalid_count_multisig),
            ScriptPubKeyType::NonStandard
        ));

        let invalid_required_multisig = {
            let mut bytes = vec![0x52, 33, 2];
            bytes.extend(vec![1_u8; 32]);
            bytes.push(0x51);
            bytes.push(0xae);
            script(&bytes)
        };
        assert!(matches!(
            classify_script_pubkey(&invalid_required_multisig),
            ScriptPubKeyType::NonStandard
        ));

        assert_eq!(super::decode_small_int(0x00), Some(0));
        assert_eq!(super::decode_small_int(0x62), None);
    }
}
