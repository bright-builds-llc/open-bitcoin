// Parity breadcrumbs:
// - packages/bitcoin-knots/src/key.h
// - packages/bitcoin-knots/src/pubkey.h
// - packages/bitcoin-knots/src/script/interpreter.cpp
// - packages/bitcoin-knots/src/script/sign.cpp

use super::{
    EcdsaVerificationRequest, SignatureError, TransactionSignatureChecker,
    is_strict_public_key_encoding, normalize_hybrid_public_key, parse_ecdsa_signature,
    parse_ecdsa_signature_for_verification, parse_public_key, parse_public_key_for_verification,
    parse_schnorr_signature, parse_schnorr_signature_for_verification,
};
use crate::context::{
    ConsensusParams, ScriptVerifyFlags, SpentOutput, TransactionInputContext,
    TransactionValidationContext,
};
use crate::sighash::SigVersion;
use open_bitcoin_primitives::{
    Amount, ScriptBuf, Transaction, TransactionInput, TransactionOutput, Txid,
};
use secp256k1::{Message, PublicKey, Secp256k1, SecretKey};

fn script(bytes: &[u8]) -> ScriptBuf {
    ScriptBuf::from_bytes(bytes.to_vec()).expect("valid script")
}

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

fn hybrid_public_key_bytes(public_key: &PublicKey) -> [u8; 65] {
    let mut bytes = public_key.serialize_uncompressed();
    bytes[0] = if (bytes[64] & 1) == 1 { 0x07 } else { 0x06 };
    bytes
}

#[test]
fn parse_ecdsa_signature_enforces_low_s_and_hash_type() {
    let secp = Secp256k1::new();
    let secret_key = SecretKey::from_byte_array([3_u8; 32]).expect("32-byte secret key");
    let message = Message::from_digest([1_u8; 32]);
    let signature = secp.sign_ecdsa(message, &secret_key).serialize_der();
    let mut encoded = signature.as_ref().to_vec();
    encoded.push(1);

    let parsed = parse_ecdsa_signature(&encoded).expect("signature should parse");
    assert_eq!(parsed.sighash_type.raw(), 1);

    let mut invalid = encoded.clone();
    *invalid.last_mut().expect("hash type") = 0;
    assert!(matches!(
        parse_ecdsa_signature(&invalid),
        Err(SignatureError::InvalidHashType(0))
    ));

    let high_s = decode_hex(
        "3046022100839c1fbc5304de944f697c9f4b1d01d1faeba32d751c0f7acb21ac8a0f436a72022100e89bd46bb3a5a62adc679f659b7ce876d83ee297c7a5587b2011c4fcc72eab4501",
    );
    assert!(matches!(
        parse_ecdsa_signature(&high_s),
        Err(SignatureError::NonLowS)
    ));
}

#[test]
fn public_key_and_schnorr_parsers_validate_shapes() {
    let secp = Secp256k1::new();
    let secret_key = SecretKey::from_byte_array([5_u8; 32]).expect("32-byte secret key");
    let public_key = PublicKey::from_secret_key(&secp, &secret_key);

    assert!(parse_public_key(&public_key.serialize(), true).is_ok());
    assert!(matches!(
        parse_public_key(&public_key.serialize_uncompressed(), true),
        Err(SignatureError::NonCompressedPublicKey)
    ));

    let schnorr = [7_u8; 64];
    assert!(parse_schnorr_signature(&schnorr).is_ok());
    let mut with_hash_type = schnorr.to_vec();
    with_hash_type.push(1);
    assert!(parse_schnorr_signature(&with_hash_type).is_ok());
    assert!(parse_schnorr_signature(&[1_u8; 63]).is_err());
    let (xonly, _) = public_key.x_only_public_key();
    assert!(super::parse_xonly_public_key(&xonly.serialize()).is_ok());
    assert!(super::parse_xonly_public_key(&[1_u8; 31]).is_err());
}

#[test]
fn schnorr_verification_parsers_reject_default_suffix_and_invalid_hash_types() {
    let default_with_suffix = [0_u8; 65];
    assert!(matches!(
        parse_schnorr_signature_for_verification(&default_with_suffix),
        Err(SignatureError::InvalidHashType(0))
    ));

    let mut invalid_hash_type = [0_u8; 65];
    invalid_hash_type[64] = 0x84;
    assert!(matches!(
        parse_schnorr_signature_for_verification(&invalid_hash_type),
        Err(SignatureError::InvalidHashType(0x84))
    ));
}

#[test]
fn verification_helpers_follow_flag_gated_legacy_rules() {
    let secp = Secp256k1::new();
    let secret_key = SecretKey::from_byte_array([6_u8; 32]).expect("secret key");
    let message = Message::from_digest([2_u8; 32]);
    let mut signature = secp.sign_ecdsa(message, &secret_key);
    signature.normalize_s();
    let serialized = signature.serialize_der();
    let mut encoded_signature = serialized.as_ref().to_vec();
    encoded_signature.push(4);

    let parsed =
        parse_ecdsa_signature_for_verification(&encoded_signature, ScriptVerifyFlags::NONE)
            .expect("non-strict undefined hashtype should parse");
    assert!(parsed.is_some());
    assert!(matches!(
        parse_ecdsa_signature_for_verification(&encoded_signature, ScriptVerifyFlags::STRICTENC),
        Err(SignatureError::InvalidHashType(4))
    ));

    assert_eq!(
        parse_ecdsa_signature_for_verification(&[], ScriptVerifyFlags::NONE)
            .expect("empty signatures are allowed as false"),
        None
    );
    assert_eq!(
        parse_ecdsa_signature_for_verification(&[1_u8], ScriptVerifyFlags::NONE)
            .expect("non-strict invalid DER becomes false"),
        None
    );
    assert!(matches!(
        parse_ecdsa_signature_for_verification(&[1_u8], ScriptVerifyFlags::DERSIG),
        Err(SignatureError::InvalidDer)
    ));

    let high_s = decode_hex(
        "3046022100839c1fbc5304de944f697c9f4b1d01d1faeba32d751c0f7acb21ac8a0f436a72022100e89bd46bb3a5a62adc679f659b7ce876d83ee297c7a5587b2011c4fcc72eab4501",
    );
    assert!(
        parse_ecdsa_signature_for_verification(&high_s, ScriptVerifyFlags::NONE)
            .expect("high-S is accepted without LOW_S")
            .is_some()
    );
    assert!(matches!(
        parse_ecdsa_signature_for_verification(&high_s, ScriptVerifyFlags::LOW_S),
        Err(SignatureError::NonLowS)
    ));

    let public_key = PublicKey::from_secret_key(&secp, &secret_key);
    let hybrid_public_key = hybrid_public_key_bytes(&public_key);
    assert!(
        parse_public_key_for_verification(&hybrid_public_key, false, ScriptVerifyFlags::NONE)
            .is_ok()
    );
    assert!(matches!(
        parse_public_key_for_verification(&hybrid_public_key, false, ScriptVerifyFlags::STRICTENC),
        Err(SignatureError::InvalidPublicKey)
    ));
    assert!(matches!(
        parse_public_key_for_verification(&[1_u8; 10], false, ScriptVerifyFlags::NONE),
        Err(SignatureError::IncorrectSignature)
    ));
    assert!(matches!(
        parse_public_key_for_verification(
            &public_key.serialize_uncompressed(),
            true,
            ScriptVerifyFlags::NONE
        ),
        Err(SignatureError::NonCompressedPublicKey)
    ));

    let invalid_strict_key = {
        let mut bytes = [0_u8; 33];
        bytes[0] = 0x02;
        bytes
    };
    assert!(matches!(
        parse_public_key_for_verification(&invalid_strict_key, false, ScriptVerifyFlags::STRICTENC),
        Err(SignatureError::InvalidPublicKey)
    ));
    assert!(is_strict_public_key_encoding(&public_key.serialize()));
    assert!(is_strict_public_key_encoding(
        &public_key.serialize_uncompressed()
    ));

    let mut invalid_hybrid = public_key.serialize_uncompressed();
    invalid_hybrid[0] = 0x06 | ((invalid_hybrid[64] ^ 1) & 1);
    assert!(normalize_hybrid_public_key(&[1_u8; 10]).is_none());
    assert!(normalize_hybrid_public_key(&invalid_hybrid).is_none());
    assert_eq!(
        normalize_hybrid_public_key(&hybrid_public_key).expect("hybrid key should normalize")[0],
        0x04
    );
}

#[test]
fn non_strict_legacy_verification_accepts_knots_compatible_lax_der_vector() {
    // Arrange
    let lax_der = decode_hex(
        "304502202de8c03fc525285c9c535631019a5f2af7c6454fa9eb392a3756a4917c420edd02210046130bf2baf7cfc065067c8b9e33a066d9c15edcea9feb0ca2d233e3597925b401",
    );
    let high_s = decode_hex(
        "3046022100839c1fbc5304de944f697c9f4b1d01d1faeba32d751c0f7acb21ac8a0f436a72022100e89bd46bb3a5a62adc679f659b7ce876d83ee297c7a5587b2011c4fcc72eab4501",
    );

    // Act
    let maybe_non_strict =
        parse_ecdsa_signature_for_verification(&lax_der, ScriptVerifyFlags::NONE);
    let strict_der = parse_ecdsa_signature_for_verification(&lax_der, ScriptVerifyFlags::DERSIG);
    let low_s = parse_ecdsa_signature_for_verification(&high_s, ScriptVerifyFlags::LOW_S);

    // Assert
    assert!(
        maybe_non_strict
            .expect("non-strict path should accept lax DER")
            .is_some()
    );
    assert!(matches!(strict_der, Err(SignatureError::InvalidDer)));
    assert!(matches!(low_s, Err(SignatureError::NonLowS)));
}

#[test]
fn transaction_signature_checker_covers_false_and_uncompressed_paths() {
    let verification_secp = Secp256k1::verification_only();
    let signing_secp = Secp256k1::new();
    let secret_key = SecretKey::from_byte_array([12_u8; 32]).expect("secret key");
    let public_key = PublicKey::from_secret_key(&signing_secp, &secret_key);
    let transaction = Transaction {
        version: 1,
        inputs: vec![TransactionInput {
            previous_output: open_bitcoin_primitives::OutPoint {
                txid: Txid::from_byte_array([3_u8; 32]),
                vout: 0,
            },
            script_sig: ScriptBuf::default(),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: Default::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(40).expect("valid amount"),
            script_pubkey: script(&[0x51]),
        }],
        lock_time: 0,
    };
    let script_code = {
        let mut bytes = Vec::with_capacity(35);
        bytes.push(33);
        bytes.extend_from_slice(&public_key.serialize());
        bytes.push(0xac);
        script(&bytes)
    };
    let context = TransactionValidationContext {
        inputs: vec![TransactionInputContext {
            spent_output: SpentOutput {
                value: Amount::from_sats(50).expect("valid amount"),
                script_pubkey: script_code.clone(),
                is_coinbase: false,
            },
            created_height: 0,
            created_median_time_past: 0,
        }],
        spend_height: 1,
        block_time: 0,
        median_time_past: 0,
        verify_flags: ScriptVerifyFlags::NONE,
        consensus_params: ConsensusParams::default(),
    };
    let precomputed = context.precompute(&transaction).expect("precompute");
    let checker = TransactionSignatureChecker::new(&verification_secp, &context, &precomputed);

    assert_eq!(
        checker.verify_ecdsa(
            EcdsaVerificationRequest {
                script_code: &script_code,
                transaction: &transaction,
                input_index: 0,
                spent_input: &context.inputs[0],
                signature_bytes: &[1_u8, 2_u8],
                public_key_bytes: &public_key.serialize(),
                sig_version: SigVersion::Base,
                require_compressed_pubkey: false,
            },
            ScriptVerifyFlags::NONE
        ),
        Ok(false)
    );

    let digest = crate::sighash::legacy_sighash(
        &script_code,
        &transaction,
        0,
        crate::sighash::SigHashType::ALL,
    );
    let message = Message::from_digest(digest.to_byte_array());
    let mut signature = signing_secp.sign_ecdsa(message, &secret_key);
    signature.normalize_s();
    let serialized = signature.serialize_der();
    let mut encoded_signature = serialized.as_ref().to_vec();
    encoded_signature.push(1);

    assert_eq!(
        checker.verify_ecdsa(
            EcdsaVerificationRequest {
                script_code: &script_code,
                transaction: &transaction,
                input_index: 0,
                spent_input: &context.inputs[0],
                signature_bytes: &encoded_signature,
                public_key_bytes: &public_key.serialize_uncompressed(),
                sig_version: SigVersion::Base,
                require_compressed_pubkey: false,
            },
            ScriptVerifyFlags::NONE
        ),
        Ok(true)
    );
    assert_eq!(
        checker.verify_ecdsa(
            EcdsaVerificationRequest {
                script_code: &script_code,
                transaction: &transaction,
                input_index: 0,
                spent_input: &context.inputs[0],
                signature_bytes: &encoded_signature,
                public_key_bytes: &[1_u8; 10],
                sig_version: SigVersion::Base,
                require_compressed_pubkey: false,
            },
            ScriptVerifyFlags::NONE
        ),
        Err(SignatureError::IncorrectSignature)
    );

    let parsed =
        parse_ecdsa_signature_for_verification(&encoded_signature, ScriptVerifyFlags::LOW_S)
            .expect("low-S signatures remain valid under LOW_S");
    assert!(parsed.is_some());
}

#[test]
fn transaction_signature_checker_verifies_legacy_signatures() {
    let secp = Secp256k1::verification_only();
    let signing_secp = Secp256k1::new();
    let secret_key = SecretKey::from_byte_array([9_u8; 32]).expect("32-byte secret key");
    let public_key = PublicKey::from_secret_key(&signing_secp, &secret_key);

    let transaction = Transaction {
        version: 1,
        inputs: vec![TransactionInput {
            previous_output: open_bitcoin_primitives::OutPoint {
                txid: Txid::from_byte_array([1_u8; 32]),
                vout: 0,
            },
            script_sig: ScriptBuf::default(),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: Default::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(40).expect("valid amount"),
            script_pubkey: script(&[0x51]),
        }],
        lock_time: 0,
    };
    let pubkey_script = {
        let mut bytes = Vec::with_capacity(35);
        bytes.push(33);
        bytes.extend_from_slice(&public_key.serialize());
        bytes.push(0xac);
        script(&bytes)
    };
    let tx_context = TransactionValidationContext {
        inputs: vec![TransactionInputContext {
            spent_output: SpentOutput {
                value: Amount::from_sats(50).expect("valid amount"),
                script_pubkey: pubkey_script.clone(),
                is_coinbase: false,
            },
            created_height: 0,
            created_median_time_past: 0,
        }],
        spend_height: 1,
        block_time: 0,
        median_time_past: 0,
        verify_flags: ScriptVerifyFlags::NONE,
        consensus_params: ConsensusParams::default(),
    };
    let precomputed = tx_context.precompute(&transaction).expect("precompute");
    let checker = TransactionSignatureChecker::new(&secp, &tx_context, &precomputed);

    let digest = crate::sighash::legacy_sighash(
        &pubkey_script,
        &transaction,
        0,
        crate::sighash::SigHashType::ALL,
    );
    let message = Message::from_digest(digest.to_byte_array());
    let mut signature = signing_secp.sign_ecdsa(message, &secret_key);
    signature.normalize_s();
    let serialized = signature.serialize_der();
    let mut encoded_signature = serialized.as_ref().to_vec();
    encoded_signature.push(1);

    let spent_input = &tx_context.inputs[0];
    let script_code = pubkey_script;

    assert!(
        checker.verify_ecdsa(
            EcdsaVerificationRequest {
                script_code: &script_code,
                transaction: &transaction,
                input_index: 0,
                spent_input,
                signature_bytes: &encoded_signature,
                public_key_bytes: &public_key.serialize(),
                sig_version: SigVersion::Base,
                require_compressed_pubkey: true,
            },
            ScriptVerifyFlags::NONE
        ) == Ok(true)
    );
}

#[test]
fn signature_error_paths_are_covered() {
    for (error, expected) in [
        (SignatureError::EmptySignature, "empty signature"),
        (SignatureError::InvalidDer, "invalid DER signature"),
        (
            SignatureError::InvalidHashType(4),
            "invalid sighash type: 4",
        ),
        (SignatureError::InvalidPublicKey, "invalid public key"),
        (
            SignatureError::NonCompressedPublicKey,
            "public key must be compressed",
        ),
        (SignatureError::NonLowS, "signature is not low-S normalized"),
        (SignatureError::IncorrectSignature, "incorrect signature"),
        (
            SignatureError::UnsupportedSigVersion,
            "unsupported signature version",
        ),
    ] {
        assert_eq!(error.to_string(), expected);
    }

    assert!(matches!(
        parse_ecdsa_signature(&[]),
        Err(SignatureError::EmptySignature)
    ));
    assert!(matches!(
        parse_ecdsa_signature(&[1_u8]),
        Err(SignatureError::InvalidDer)
    ));

    let secp = Secp256k1::new();
    let secret_key = SecretKey::from_byte_array([11_u8; 32]).expect("secret key");
    let public_key = PublicKey::from_secret_key(&secp, &secret_key);
    let transaction = Transaction {
        version: 1,
        inputs: vec![TransactionInput {
            previous_output: open_bitcoin_primitives::OutPoint {
                txid: Txid::from_byte_array([1_u8; 32]),
                vout: 0,
            },
            script_sig: ScriptBuf::default(),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: Default::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(40).expect("valid amount"),
            script_pubkey: script(&[0x51]),
        }],
        lock_time: 0,
    };
    let script_code = {
        let mut bytes = Vec::with_capacity(35);
        bytes.push(33);
        bytes.extend_from_slice(&public_key.serialize());
        bytes.push(0xac);
        script(&bytes)
    };
    let context = TransactionValidationContext {
        inputs: vec![TransactionInputContext {
            spent_output: SpentOutput {
                value: Amount::from_sats(50).expect("valid amount"),
                script_pubkey: script_code.clone(),
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
    let verification_secp = Secp256k1::verification_only();
    let checker = TransactionSignatureChecker::new(&verification_secp, &context, &precomputed);
    let digest = crate::sighash::segwit_v0_sighash(
        &script_code,
        &transaction,
        0,
        &context.inputs[0],
        crate::sighash::SigHashType::ALL,
        &precomputed,
    );
    let message = Message::from_digest(digest.to_byte_array());
    let mut signature = secp.sign_ecdsa(message, &secret_key);
    signature.normalize_s();
    let serialized = signature.serialize_der();
    let mut encoded_signature = serialized.as_ref().to_vec();
    encoded_signature.push(1);

    assert!(
        checker.verify_ecdsa(
            EcdsaVerificationRequest {
                script_code: &script_code,
                transaction: &transaction,
                input_index: 0,
                spent_input: &context.inputs[0],
                signature_bytes: &encoded_signature,
                public_key_bytes: &public_key.serialize(),
                sig_version: SigVersion::WitnessV0,
                require_compressed_pubkey: true,
            },
            ScriptVerifyFlags::WITNESS
        ) == Ok(true)
    );

    assert!(matches!(
        checker.verify_ecdsa(
            EcdsaVerificationRequest {
                script_code: &script_code,
                transaction: &transaction,
                input_index: 0,
                spent_input: &context.inputs[0],
                signature_bytes: &encoded_signature,
                public_key_bytes: &public_key.serialize(),
                sig_version: SigVersion::Taproot,
                require_compressed_pubkey: true,
            },
            ScriptVerifyFlags::WITNESS
        ),
        Err(SignatureError::UnsupportedSigVersion)
    ));
}
