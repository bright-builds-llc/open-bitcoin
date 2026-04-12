use secp256k1::{Message, PublicKey, Secp256k1, Verification, XOnlyPublicKey, ecdsa, schnorr};

use crate::context::{PrecomputedTransactionData, TransactionValidationContext};
use crate::sighash::{SigHashType, SigVersion, legacy_sighash, segwit_v0_sighash};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SignatureError {
    EmptySignature,
    InvalidDer,
    InvalidHashType(u32),
    InvalidPublicKey,
    NonCompressedPublicKey,
    NonLowS,
    IncorrectSignature,
    UnsupportedSigVersion,
}

impl core::fmt::Display for SignatureError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::EmptySignature => write!(f, "empty signature"),
            Self::InvalidDer => write!(f, "invalid DER signature"),
            Self::InvalidHashType(hash_type) => write!(f, "invalid sighash type: {hash_type}"),
            Self::InvalidPublicKey => write!(f, "invalid public key"),
            Self::NonCompressedPublicKey => write!(f, "public key must be compressed"),
            Self::NonLowS => write!(f, "signature is not low-S normalized"),
            Self::IncorrectSignature => write!(f, "incorrect signature"),
            Self::UnsupportedSigVersion => write!(f, "unsupported signature version"),
        }
    }
}

impl std::error::Error for SignatureError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedEcdsaSignature {
    pub signature: ecdsa::Signature,
    pub sighash_type: SigHashType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedSchnorrSignature {
    pub signature: schnorr::Signature,
    pub sighash_type: SigHashType,
}

pub fn parse_ecdsa_signature(bytes: &[u8]) -> Result<ParsedEcdsaSignature, SignatureError> {
    let Some((&hash_type, der)) = bytes.split_last() else {
        return Err(SignatureError::EmptySignature);
    };
    let sighash_type = SigHashType::from_u32(u32::from(hash_type));
    validate_legacy_sighash_type(sighash_type)?;

    let signature = ecdsa::Signature::from_der(der).map_err(|_| SignatureError::InvalidDer)?;
    let mut normalized = signature;
    normalized.normalize_s();
    if normalized != signature {
        return Err(SignatureError::NonLowS);
    }

    Ok(ParsedEcdsaSignature {
        signature,
        sighash_type,
    })
}

pub fn parse_schnorr_signature(bytes: &[u8]) -> Result<ParsedSchnorrSignature, SignatureError> {
    let (signature_bytes, sighash_type) = match bytes.len() {
        64 => (bytes, SigHashType::DEFAULT),
        65 => (&bytes[..64], SigHashType::from_u32(u32::from(bytes[64]))),
        _ => return Err(SignatureError::InvalidDer),
    };

    let signature = schnorr::Signature::from_byte_array(
        signature_bytes
            .try_into()
            .map_err(|_| SignatureError::InvalidDer)?,
    );
    Ok(ParsedSchnorrSignature {
        signature,
        sighash_type,
    })
}

pub fn parse_public_key(
    bytes: &[u8],
    require_compressed: bool,
) -> Result<PublicKey, SignatureError> {
    if require_compressed && bytes.len() != 33 {
        return Err(SignatureError::NonCompressedPublicKey);
    }
    PublicKey::from_slice(bytes).map_err(|_| SignatureError::InvalidPublicKey)
}

pub fn parse_xonly_public_key(bytes: &[u8]) -> Result<XOnlyPublicKey, SignatureError> {
    XOnlyPublicKey::from_byte_array(
        bytes
            .try_into()
            .map_err(|_| SignatureError::InvalidPublicKey)?,
    )
    .map_err(|_| SignatureError::InvalidPublicKey)
}

pub struct TransactionSignatureChecker<'a, C: Verification> {
    secp: &'a Secp256k1<C>,
    _context: &'a TransactionValidationContext,
    precomputed: &'a PrecomputedTransactionData,
}

pub struct EcdsaVerificationRequest<'a> {
    pub script_code: &'a open_bitcoin_primitives::ScriptBuf,
    pub transaction: &'a open_bitcoin_primitives::Transaction,
    pub input_index: usize,
    pub spent_input: &'a crate::context::TransactionInputContext,
    pub signature_bytes: &'a [u8],
    pub public_key_bytes: &'a [u8],
    pub sig_version: SigVersion,
    pub require_compressed_pubkey: bool,
}

impl<'a, C: Verification> TransactionSignatureChecker<'a, C> {
    pub fn new(
        secp: &'a Secp256k1<C>,
        context: &'a TransactionValidationContext,
        precomputed: &'a PrecomputedTransactionData,
    ) -> Self {
        Self {
            secp,
            _context: context,
            precomputed,
        }
    }

    pub fn verify_ecdsa(
        &self,
        request: EcdsaVerificationRequest<'_>,
    ) -> Result<(), SignatureError> {
        let parsed_signature = parse_ecdsa_signature(request.signature_bytes)?;
        let public_key =
            parse_public_key(request.public_key_bytes, request.require_compressed_pubkey)?;
        let digest = match request.sig_version {
            SigVersion::Base => legacy_sighash(
                request.script_code,
                request.transaction,
                request.input_index,
                parsed_signature.sighash_type,
            ),
            SigVersion::WitnessV0 => segwit_v0_sighash(
                request.script_code,
                request.transaction,
                request.input_index,
                request.spent_input,
                parsed_signature.sighash_type,
                self.precomputed,
            ),
            _ => return Err(SignatureError::UnsupportedSigVersion),
        };
        let message = Message::from_digest(digest.to_byte_array());
        self.secp
            .verify_ecdsa(message, &parsed_signature.signature, &public_key)
            .map_err(|_| SignatureError::IncorrectSignature)
    }
}

fn validate_legacy_sighash_type(sighash_type: SigHashType) -> Result<(), SignatureError> {
    match sighash_type.base_type() {
        1..=3 => Ok(()),
        _ => Err(SignatureError::InvalidHashType(sighash_type.raw())),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        EcdsaVerificationRequest, SignatureError, TransactionSignatureChecker,
        parse_ecdsa_signature, parse_public_key, parse_schnorr_signature,
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
            checker
                .verify_ecdsa(EcdsaVerificationRequest {
                    script_code: &script_code,
                    transaction: &transaction,
                    input_index: 0,
                    spent_input,
                    signature_bytes: &encoded_signature,
                    public_key_bytes: &public_key.serialize(),
                    sig_version: SigVersion::Base,
                    require_compressed_pubkey: true,
                })
                .is_ok()
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
            checker
                .verify_ecdsa(EcdsaVerificationRequest {
                    script_code: &script_code,
                    transaction: &transaction,
                    input_index: 0,
                    spent_input: &context.inputs[0],
                    signature_bytes: &encoded_signature,
                    public_key_bytes: &public_key.serialize(),
                    sig_version: SigVersion::WitnessV0,
                    require_compressed_pubkey: true,
                })
                .is_ok()
        );

        assert!(matches!(
            checker.verify_ecdsa(EcdsaVerificationRequest {
                script_code: &script_code,
                transaction: &transaction,
                input_index: 0,
                spent_input: &context.inputs[0],
                signature_bytes: &encoded_signature,
                public_key_bytes: &public_key.serialize(),
                sig_version: SigVersion::Taproot,
                require_compressed_pubkey: true,
            }),
            Err(SignatureError::UnsupportedSigVersion)
        ));
    }
}
