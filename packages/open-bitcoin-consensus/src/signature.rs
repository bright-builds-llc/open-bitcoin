use secp256k1::{Message, PublicKey, Secp256k1, Verification, XOnlyPublicKey, ecdsa, schnorr};

use crate::context::{
    PrecomputedTransactionData, ScriptExecutionData, ScriptVerifyFlags,
    TransactionValidationContext,
};
use crate::sighash::{SigHashType, SigVersion, legacy_sighash, segwit_v0_sighash, taproot_sighash};

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
    context: &'a TransactionValidationContext,
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
            context,
            precomputed,
        }
    }

    pub fn verify_ecdsa(
        &self,
        request: EcdsaVerificationRequest<'_>,
        flags: ScriptVerifyFlags,
    ) -> Result<bool, SignatureError> {
        let maybe_parsed_signature =
            parse_ecdsa_signature_for_verification(request.signature_bytes, flags)?;
        let Some(parsed_signature) = maybe_parsed_signature else {
            return Ok(false);
        };
        let public_key = parse_public_key_for_verification(
            request.public_key_bytes,
            request.require_compressed_pubkey,
            flags,
        )?;
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
        Ok(self
            .secp
            .verify_ecdsa(message, &parsed_signature.signature, &public_key)
            .is_ok())
    }

    pub fn verify_schnorr(
        &self,
        signature_bytes: &[u8],
        public_key_bytes: &[u8],
        transaction: &open_bitcoin_primitives::Transaction,
        input_index: usize,
        sig_version: SigVersion,
        execution_data: &ScriptExecutionData,
    ) -> Result<bool, SignatureError> {
        let parsed_signature = parse_schnorr_signature_for_verification(signature_bytes)?;
        let public_key = parse_xonly_public_key(public_key_bytes)?;
        let digest = taproot_sighash(
            execution_data,
            transaction,
            input_index,
            parsed_signature.sighash_type,
            sig_version,
            self.context,
        )
        .ok_or(SignatureError::InvalidHashType(
            parsed_signature.sighash_type.raw(),
        ))?;
        Ok(self
            .secp
            .verify_schnorr(&parsed_signature.signature, digest.as_bytes(), &public_key)
            .is_ok())
    }
}

fn validate_legacy_sighash_type(sighash_type: SigHashType) -> Result<(), SignatureError> {
    match sighash_type.base_type() {
        1..=3 => Ok(()),
        _ => Err(SignatureError::InvalidHashType(sighash_type.raw())),
    }
}

fn parse_schnorr_signature_for_verification(
    bytes: &[u8],
) -> Result<ParsedSchnorrSignature, SignatureError> {
    let parsed = parse_schnorr_signature(bytes)?;
    if bytes.len() == 65 && parsed.sighash_type.is_default() {
        return Err(SignatureError::InvalidHashType(parsed.sighash_type.raw()));
    }
    match parsed.sighash_type.raw() {
        0..=3 | 0x81..=0x83 => Ok(parsed),
        _ => Err(SignatureError::InvalidHashType(parsed.sighash_type.raw())),
    }
}

fn parse_ecdsa_signature_for_verification(
    bytes: &[u8],
    flags: ScriptVerifyFlags,
) -> Result<Option<ParsedEcdsaSignature>, SignatureError> {
    let Some((&hash_type, der)) = bytes.split_last() else {
        return Ok(None);
    };
    let sighash_type = SigHashType::from_u32(u32::from(hash_type));

    if flags.contains(ScriptVerifyFlags::STRICTENC) {
        validate_legacy_sighash_type(sighash_type)?;
    }

    let signature = match ecdsa::Signature::from_der(der) {
        Ok(signature) => signature,
        Err(_) => {
            if flags.contains(ScriptVerifyFlags::DERSIG)
                || flags.contains(ScriptVerifyFlags::LOW_S)
                || flags.contains(ScriptVerifyFlags::STRICTENC)
            {
                return Err(SignatureError::InvalidDer);
            }
            return Ok(None);
        }
    };

    if flags.contains(ScriptVerifyFlags::LOW_S) {
        let mut normalized = signature;
        normalized.normalize_s();
        if normalized != signature {
            return Err(SignatureError::NonLowS);
        }
    }

    Ok(Some(ParsedEcdsaSignature {
        signature,
        sighash_type,
    }))
}

fn parse_public_key_for_verification(
    bytes: &[u8],
    require_compressed: bool,
    flags: ScriptVerifyFlags,
) -> Result<PublicKey, SignatureError> {
    if flags.contains(ScriptVerifyFlags::STRICTENC) && !is_strict_public_key_encoding(bytes) {
        return Err(SignatureError::InvalidPublicKey);
    }
    if require_compressed && bytes.len() != 33 {
        return Err(SignatureError::NonCompressedPublicKey);
    }

    if let Some(normalized) = normalize_hybrid_public_key(bytes)
        && !flags.contains(ScriptVerifyFlags::STRICTENC)
    {
        return PublicKey::from_slice(&normalized).map_err(|_| SignatureError::InvalidPublicKey);
    }

    let Ok(public_key) = PublicKey::from_slice(bytes) else {
        if flags.contains(ScriptVerifyFlags::STRICTENC) {
            return Err(SignatureError::InvalidPublicKey);
        }

        return Err(SignatureError::IncorrectSignature);
    };

    Ok(public_key)
}

fn is_strict_public_key_encoding(bytes: &[u8]) -> bool {
    matches!(bytes, [0x02 | 0x03, ..] if bytes.len() == 33)
        || matches!(bytes, [0x04, ..] if bytes.len() == 65)
}

fn normalize_hybrid_public_key(bytes: &[u8]) -> Option<[u8; 65]> {
    if bytes.len() != 65 || !matches!(bytes[0], 0x06 | 0x07) {
        return None;
    }

    let normalized_prefix = 0x04_u8;
    let y_is_odd = (bytes[64] & 1) == 1;
    let hybrid_is_odd = bytes[0] == 0x07;
    if y_is_odd != hybrid_is_odd {
        return None;
    }

    let mut normalized = [0_u8; 65];
    normalized.copy_from_slice(bytes);
    normalized[0] = normalized_prefix;
    Some(normalized)
}

#[cfg(test)]
mod tests;
