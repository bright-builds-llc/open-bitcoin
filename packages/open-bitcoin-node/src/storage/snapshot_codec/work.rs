// Parity breadcrumbs:
// - packages/bitcoin-knots/src/arith_uint256.h

//! Storage work numbers preserve width and representation, never accepted lineage.

use open_bitcoin_core::consensus::chain_proof::ProofWork;
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error, ser::Error as _};
use serde_json::value::RawValue;

use crate::{StorageError, StorageNamespace, StorageRecoveryAction};

const WORK_RECORD_VERSION: u32 = 1;
// Compact 32-byte records need fewer than 256 JSON bytes; bound untrusted parsing.
const MAX_WORK_RECORD_BYTES: usize = 512;

/// A decoded number's representation, with no assertion of validated ancestry or recovery.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClassifiedWorkRecord {
    /// Historic count/synthetic scores must remain visibly distinct from exact work.
    LegacySynthetic(u128),
    /// All 256 bits are present; numerical exactness does not earn runtime authority.
    ExactNumber(ProofWork),
}

#[derive(Deserialize)]
struct RecordKind {
    kind: String,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ExactRecord {
    version: u32,
    kind: String,
    bytes_le: [u8; 32],
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct LegacyRecord {
    version: u32,
    kind: String,
    score: u128,
}

impl Serialize for ClassifiedWorkRecord {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // Keep embedded records compact even inside a pretty-printed snapshot,
        // so our own writes honor the same strict 512-byte read bound.
        let raw = match self {
            Self::LegacySynthetic(score) => serde_json::value::to_raw_value(&LegacyRecord {
                version: WORK_RECORD_VERSION,
                kind: "legacy_synthetic".to_owned(),
                score: *score,
            }),
            Self::ExactNumber(number) => serde_json::value::to_raw_value(&ExactRecord {
                version: WORK_RECORD_VERSION,
                kind: "exact_number".to_owned(),
                bytes_le: number.to_le_bytes(),
            }),
        }
        .map_err(S::Error::custom)?;
        raw.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ClassifiedWorkRecord {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // Raw JSON preserves historic u128 values that serde_json::Value cannot hold exactly.
        let raw = Box::<RawValue>::deserialize(deserializer)?;
        classify(raw.get()).map_err(D::Error::custom)
    }
}

fn classify(json: &str) -> Result<ClassifiedWorkRecord, &'static str> {
    if json.len() > MAX_WORK_RECORD_BYTES {
        return Err("work number record exceeds byte limit");
    }
    if !json.trim_start().starts_with('{') {
        let score = serde_json::from_str::<u128>(json).map_err(|_| "invalid legacy work number")?;
        return Ok(ClassifiedWorkRecord::LegacySynthetic(score));
    }
    let record_kind =
        serde_json::from_str::<RecordKind>(json).map_err(|_| "invalid work record tag")?;
    match record_kind.kind.as_str() {
        "legacy_synthetic" => {
            let record = serde_json::from_str::<LegacyRecord>(json)
                .map_err(|_| "invalid legacy work record shape")?;
            check_version(record.version)?;
            Ok(ClassifiedWorkRecord::LegacySynthetic(record.score))
        }
        "exact_number" => {
            let record = serde_json::from_str::<ExactRecord>(json)
                .map_err(|_| "invalid exact work record shape")?;
            check_version(record.version)?;
            Ok(ClassifiedWorkRecord::ExactNumber(ProofWork::from_le_bytes(
                record.bytes_le,
            )))
        }
        _ => Err("unknown work record tag"),
    }
}

fn check_version(version: u32) -> Result<(), &'static str> {
    if version != WORK_RECORD_VERSION {
        return Err("unsupported work record version");
    }
    Ok(())
}

/// Encode a tagged/versioned number without reading or modifying storage.
pub fn encode_work_record(
    namespace: StorageNamespace,
    record: &ClassifiedWorkRecord,
) -> Result<Vec<u8>, StorageError> {
    serde_json::to_vec(record).map_err(|_| StorageError::BackendFailure {
        namespace,
        message: "failed to encode work number".to_owned(),
        action: StorageRecoveryAction::Restart,
    })
}

/// Decode either an explicit tagged record or a historic bare u128 score.
/// The result is untrusted numeric data, never a checkpoint or acceptance token.
/// Records larger than 512 bytes, including whitespace, are refused before parsing.
pub fn decode_work_record(
    namespace: StorageNamespace,
    bytes: &[u8],
) -> Result<ClassifiedWorkRecord, StorageError> {
    if bytes.len() > MAX_WORK_RECORD_BYTES {
        return Err(super::corruption(
            namespace,
            "work number record exceeds byte limit",
        ));
    }
    serde_json::from_slice(bytes)
        .map_err(|_| super::corruption(namespace, "invalid work number record"))
}

#[cfg(test)]
mod tests;
