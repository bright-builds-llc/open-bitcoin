// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.h
// - packages/bitcoin-knots/test/functional/mempool_persist.py

use std::collections::BTreeSet;
use std::fmt;

use open_bitcoin_core::{
    consensus::{transaction_txid, transaction_wtxid},
    primitives::{Transaction, Txid, Wtxid},
};
use open_bitcoin_mempool::{
    MempoolAcceptanceTime, MempoolEntryMetadata, MempoolMemberIdentity, PolicyTime,
    transaction_weight_and_virtual_size,
};

pub(crate) const MAX_MEMPOOL_SNAPSHOT_UNBROADCAST_MEMBERS: usize = 5_000;

/// The current format version local to the mempool snapshot payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MempoolSnapshotFormatVersion(u32);

impl MempoolSnapshotFormatVersion {
    pub const CURRENT: Self = Self(2);

    pub const fn get(self) -> u32 {
        self.0
    }
}

impl TryFrom<u32> for MempoolSnapshotFormatVersion {
    type Error = MempoolSnapshotError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value == Self::CURRENT.get() {
            return Ok(Self::CURRENT);
        }
        Err(MempoolSnapshotError::UnsupportedVersion)
    }
}

/// The authoritative lifecycle generation captured by one current snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CapturedMempoolGeneration(u64);

impl CapturedMempoolGeneration {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn try_new(value: u64) -> Result<Self, MempoolSnapshotError> {
        if value == u64::MAX {
            return Err(MempoolSnapshotError::StructuralCorruption);
        }
        Ok(Self(value))
    }

    pub const fn raw(self) -> u64 {
        self.0
    }
}

/// Low-cardinality snapshot failures safe to retain in recovery evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MempoolSnapshotError {
    UnsupportedVersion,
    StructuralCorruption,
    ResourceBoundExceeded,
    IdentityMismatch,
    DecodeFailure,
}

impl fmt::Display for MempoolSnapshotError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::UnsupportedVersion => "unsupported mempool snapshot version",
            Self::StructuralCorruption => "mempool snapshot structure is corrupt",
            Self::ResourceBoundExceeded => "mempool snapshot exceeds a resource bound",
            Self::IdentityMismatch => "mempool snapshot identity mismatch",
            Self::DecodeFailure => "mempool snapshot decode failed",
        })
    }
}

impl std::error::Error for MempoolSnapshotError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MempoolSnapshotRecord {
    /// Canonical durable source transaction.
    pub transaction: Transaction,
    /// Trustworthy original acceptance time, or explicit legacy unknown.
    pub acceptance_time: MempoolAcceptanceTime,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MempoolSnapshotSource {
    CurrentV2 {
        format_version: MempoolSnapshotFormatVersion,
        captured_generation: CapturedMempoolGeneration,
        captured_at: PolicyTime,
    },
    LegacyV1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MempoolSnapshot {
    pub records: Vec<MempoolSnapshotRecord>,
    source: MempoolSnapshotSource,
    unbroadcast_members: BTreeSet<MempoolMemberIdentity>,
}

impl Default for MempoolSnapshot {
    fn default() -> Self {
        Self::from_legacy_v1(Vec::new())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MempoolRecoveryStatus {
    Recovered,
    DroppedConfirmed,
    DroppedDuplicate,
    DroppedMissingParent,
    DroppedPolicyIncompatible,
    DroppedExpired,
    DroppedEvicted,
}

impl MempoolRecoveryStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Recovered => "recovered",
            Self::DroppedConfirmed => "dropped_confirmed",
            Self::DroppedDuplicate => "dropped_duplicate",
            Self::DroppedMissingParent => "dropped_missing_parent",
            Self::DroppedPolicyIncompatible => "dropped_policy_incompatible",
            Self::DroppedExpired => "dropped_expired",
            Self::DroppedEvicted => "dropped_evicted",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MempoolRecoveryRecord {
    pub txid: Txid,
    pub status: MempoolRecoveryStatus,
}

impl MempoolSnapshot {
    pub fn try_new_current(
        captured_generation: CapturedMempoolGeneration,
        captured_at: PolicyTime,
        records: Vec<MempoolSnapshotRecord>,
        unbroadcast_members: BTreeSet<MempoolMemberIdentity>,
    ) -> Result<Self, MempoolSnapshotError> {
        if captured_generation.raw() == u64::MAX {
            return Err(MempoolSnapshotError::StructuralCorruption);
        }
        if unbroadcast_members.len() > MAX_MEMPOOL_SNAPSHOT_UNBROADCAST_MEMBERS {
            return Err(MempoolSnapshotError::ResourceBoundExceeded);
        }
        if records.iter().any(|record| {
            matches!(
                record.acceptance_time,
                MempoolAcceptanceTime::Known(accepted_at) if accepted_at > captured_at
            )
        }) {
            return Err(MempoolSnapshotError::StructuralCorruption);
        }

        let record_members = records
            .iter()
            .map(MempoolSnapshotRecord::member_identity)
            .collect::<Result<BTreeSet<_>, _>>()?;
        if record_members.len() != records.len() || !unbroadcast_members.is_subset(&record_members)
        {
            return Err(MempoolSnapshotError::IdentityMismatch);
        }

        Ok(Self {
            records,
            source: MempoolSnapshotSource::CurrentV2 {
                format_version: MempoolSnapshotFormatVersion::CURRENT,
                captured_generation,
                captured_at,
            },
            unbroadcast_members,
        })
    }

    pub fn from_legacy_v1(records: Vec<MempoolSnapshotRecord>) -> Self {
        Self {
            records,
            source: MempoolSnapshotSource::LegacyV1,
            unbroadcast_members: BTreeSet::new(),
        }
    }

    pub const fn format_version(&self) -> Option<MempoolSnapshotFormatVersion> {
        match self.source {
            MempoolSnapshotSource::CurrentV2 { format_version, .. } => Some(format_version),
            MempoolSnapshotSource::LegacyV1 => None,
        }
    }

    pub const fn captured_generation(&self) -> Option<CapturedMempoolGeneration> {
        match self.source {
            MempoolSnapshotSource::CurrentV2 {
                captured_generation,
                ..
            } => Some(captured_generation),
            MempoolSnapshotSource::LegacyV1 => None,
        }
    }

    pub const fn captured_at(&self) -> Option<PolicyTime> {
        match self.source {
            MempoolSnapshotSource::CurrentV2 { captured_at, .. } => Some(captured_at),
            MempoolSnapshotSource::LegacyV1 => None,
        }
    }

    pub fn unbroadcast_members(&self) -> &BTreeSet<MempoolMemberIdentity> {
        &self.unbroadcast_members
    }
}

impl MempoolSnapshotRecord {
    pub fn try_from_compatibility(
        transaction: Transaction,
        txid: Txid,
        wtxid: Wtxid,
        fee_sats: i64,
        virtual_size: usize,
        metadata: MempoolEntryMetadata,
    ) -> Result<Self, MempoolSnapshotError> {
        let actual_txid = transaction_txid(&transaction)
            .map_err(|_| MempoolSnapshotError::StructuralCorruption)?;
        let actual_wtxid = transaction_wtxid(&transaction)
            .map_err(|_| MempoolSnapshotError::StructuralCorruption)?;
        if txid != actual_txid || wtxid != actual_wtxid {
            return Err(MempoolSnapshotError::IdentityMismatch);
        }
        let (_, actual_virtual_size) = transaction_weight_and_virtual_size(&transaction)
            .map_err(|_| MempoolSnapshotError::StructuralCorruption)?;
        if fee_sats < 0 || virtual_size != actual_virtual_size {
            return Err(MempoolSnapshotError::StructuralCorruption);
        }

        Ok(Self {
            transaction,
            acceptance_time: metadata.accepted_at,
        })
    }

    pub fn try_from_canonical(
        transaction: Transaction,
        acceptance_time: MempoolAcceptanceTime,
    ) -> Result<Self, MempoolSnapshotError> {
        Ok(Self {
            transaction,
            acceptance_time,
        })
    }

    pub fn member_identity(&self) -> Result<MempoolMemberIdentity, MempoolSnapshotError> {
        let txid = transaction_txid(&self.transaction)
            .map_err(|_| MempoolSnapshotError::StructuralCorruption)?;
        let wtxid = transaction_wtxid(&self.transaction)
            .map_err(|_| MempoolSnapshotError::StructuralCorruption)?;
        Ok(MempoolMemberIdentity { txid, wtxid })
    }
}

#[cfg(test)]
#[path = "mempool_snapshot/tests.rs"]
mod tests;
