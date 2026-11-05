// Parity breadcrumbs:
// - packages/bitcoin-knots/src/pubkey.cpp
// - packages/bitcoin-knots/src/script/interpreter.cpp
// - packages/bitcoin-knots/src/consensus/tx_verify.cpp

use super::*;
use crate::sighash::{SigHashType, legacy_sighash, segwit_v0_sighash};
use secp256k1::ecdsa::Signature;

fn high_s(low: Signature) -> Signature {
    let order = decode_hex("fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141");
    let mut compact = low.serialize_compact();
    let mut borrow = 0_i16;
    for index in (0..32).rev() {
        let difference = i16::from(order[index]) - i16::from(compact[index + 32]) - borrow;
        compact[index + 32] = difference.rem_euclid(256) as u8;
        borrow = i16::from(difference < 0);
    }
    Signature::from_compact(&compact).expect("equivalent high-S signature")
}

fn encoded(signature: Signature) -> Vec<u8> {
    let mut bytes = signature.serialize_der().as_ref().to_vec();
    bytes.push(1);
    bytes
}

fn fixture(
    multisig: bool,
) -> (
    Transaction,
    Transaction,
    TransactionValidationContext,
    PublicKey,
) {
    let secp = Secp256k1::new();
    let key = SecretKey::from_byte_array([9; 32]).expect("fixed test key");
    let public_key = PublicKey::from_secret_key(&secp, &key);
    let mut bytes = if multisig { vec![0x51, 33] } else { vec![33] };
    bytes.extend_from_slice(&public_key.serialize());
    bytes.extend_from_slice(if multisig { &[0x51, 0xae] } else { &[0xac] });
    let script_code = script(&bytes);
    let funding = Transaction {
        version: 1,
        inputs: vec![TransactionInput {
            previous_output: open_bitcoin_primitives::OutPoint::null(),
            script_sig: script(&[0x51, 0x51]),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: Default::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(100_000).expect("amount"),
            script_pubkey: script_code.clone(),
        }],
        lock_time: 0,
    };
    let transaction = Transaction {
        version: 2,
        inputs: vec![TransactionInput {
            previous_output: open_bitcoin_primitives::OutPoint {
                txid: crate::transaction_txid(&funding).expect("funding txid"),
                vout: 0,
            },
            script_sig: script(&[0x51]),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: Default::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(90_000).expect("amount"),
            script_pubkey: script(&[0x51]),
        }],
        lock_time: 0,
    };
    let context = TransactionValidationContext {
        inputs: vec![TransactionInputContext {
            spent_output: SpentOutput {
                value: funding.outputs[0].value,
                script_pubkey: script_code,
                is_coinbase: true,
            },
            created_height: 1,
            created_median_time_past: 1_296_688_602,
        }],
        spend_height: 101,
        block_time: 1_296_749_202,
        median_time_past: 1_296_745_602,
        verify_flags: ScriptVerifyFlags::DERSIG,
        consensus_params: ConsensusParams::default(),
    };
    (funding, transaction, context, public_key)
}

#[test]
fn phase167_signature_high_s_actual_signed_same_digest_and_policy() {
    // Arrange: a real transaction digest, not the earlier parser-only fixture.
    let (_, transaction, context, public_key) = fixture(false);
    let secp = Secp256k1::new();
    let key = SecretKey::from_byte_array([9; 32]).expect("key");
    let script_code = &context.inputs[0].spent_output.script_pubkey;
    let precomputed = context.precompute(&transaction).expect("precompute");
    let checker = TransactionSignatureChecker::new(&secp, &context, &precomputed);
    for version in [SigVersion::Base, SigVersion::WitnessV0] {
        let digest = match version {
            SigVersion::Base => legacy_sighash(script_code, &transaction, 0, SigHashType::ALL),
            _ => segwit_v0_sighash(
                script_code,
                &transaction,
                0,
                &context.inputs[0],
                SigHashType::ALL,
                &precomputed,
            ),
        };
        let low = secp.sign_ecdsa(Message::from_digest(digest.to_byte_array()), &key);
        let high = high_s(low);
        let mut normalized = high;
        normalized.normalize_s();
        assert_eq!(normalized, low);
        let verify = |bytes: &[u8], tx: &Transaction, pubkey: &[u8], flags| {
            checker.verify_ecdsa(
                EcdsaVerificationRequest {
                    script_code,
                    transaction: tx,
                    input_index: 0,
                    spent_input: &context.inputs[0],
                    signature_bytes: bytes,
                    public_key_bytes: pubkey,
                    sig_version: version,
                    require_compressed_pubkey: false,
                },
                flags,
            )
        };
        let low_bytes = encoded(low);
        let high_bytes = encoded(high);
        // Act / Assert: crypto verification is required in both cases.
        assert_eq!(
            verify(
                &high_bytes,
                &transaction,
                &public_key.serialize(),
                ScriptVerifyFlags::NONE
            ),
            Ok(true)
        );
        assert_eq!(
            verify(
                &low_bytes,
                &transaction,
                &public_key.serialize(),
                ScriptVerifyFlags::DERSIG
            ),
            Ok(true)
        );
        assert_eq!(
            verify(
                &high_bytes,
                &transaction,
                &public_key.serialize(),
                ScriptVerifyFlags::DERSIG | ScriptVerifyFlags::LOW_S
            ),
            Err(SignatureError::NonLowS)
        );
        assert_eq!(
            parse_ecdsa_signature(&high_bytes),
            Err(SignatureError::NonLowS)
        );
        assert_eq!(
            verify(
                &low_bytes,
                &transaction,
                &public_key.serialize(),
                ScriptVerifyFlags::LOW_S
            ),
            Ok(true)
        );
        assert_eq!(
            verify(
                &high_bytes,
                &transaction,
                &public_key.serialize(),
                ScriptVerifyFlags::DERSIG
            ),
            Ok(true),
            "valid high-S {version:?}"
        );
        let mut changed = transaction.clone();
        changed.lock_time = 1;
        assert_eq!(
            verify(
                &high_bytes,
                &changed,
                &public_key.serialize(),
                ScriptVerifyFlags::DERSIG
            ),
            Ok(false)
        );
        let wrong_key =
            PublicKey::from_secret_key(&secp, &SecretKey::from_byte_array([8; 32]).expect("key"));
        assert_eq!(
            verify(
                &high_bytes,
                &transaction,
                &wrong_key.serialize(),
                ScriptVerifyFlags::DERSIG
            ),
            Ok(false)
        );
        let invalid = encoded(high_s(secp.sign_ecdsa(Message::from_digest([7; 32]), &key)));
        assert_eq!(
            verify(
                &invalid,
                &transaction,
                &public_key.serialize(),
                ScriptVerifyFlags::DERSIG
            ),
            Ok(false)
        );
        assert_eq!(high_bytes, encoded(high));
    }
}

#[test]
fn phase167_signature_high_s_generate_digest_bound_canonical_vectors() {
    // Arrange: same accepted height1 funding and height101 spend as chainstate.
    let secp = Secp256k1::new();
    let key = SecretKey::from_byte_array([9; 32]).expect("key");
    for multisig in [false, true] {
        let (funding, transaction, context, public_key) = fixture(multisig);
        let script_code = &context.inputs[0].spent_output.script_pubkey;
        let digest = legacy_sighash(script_code, &transaction, 0, SigHashType::ALL);
        // Act
        let low = secp.sign_ecdsa(Message::from_digest(digest.to_byte_array()), &key);
        let high = high_s(low);
        let mut normalized = high;
        normalized.normalize_s();
        // Assert: independent libsecp check before publishing immutable vector bytes.
        assert_eq!(normalized, low);
        assert!(
            secp.verify_ecdsa(
                Message::from_digest(digest.to_byte_array()),
                &normalized,
                &public_key
            )
            .is_ok()
        );
        std::println!(
            "multisig={multisig} funding={:?} script={:02x?} digest={:02x?} public={:02x?} low={:02x?} high={:02x?}",
            crate::transaction_txid(&funding).expect("txid"),
            script_code.as_bytes(),
            digest.to_byte_array(),
            public_key.serialize(),
            encoded(low),
            encoded(high)
        );
    }
}
