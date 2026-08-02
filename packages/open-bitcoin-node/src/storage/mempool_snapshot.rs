// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.h
// - packages/bitcoin-knots/test/functional/mempool_persist.py

use std::collections::BTreeSet;
use std::fmt;

use open_bitcoin_core::{
    chainstate::ChainstateSnapshot,
    consensus::{ConsensusParams, ScriptVerifyFlags, transaction_txid, transaction_wtxid},
    primitives::{OutPoint, Transaction, Txid, Wtxid},
};
use open_bitcoin_mempool::{
    AdmissionContext, Mempool, MempoolAcceptanceTime, MempoolEntryMetadata, MempoolMemberIdentity,
    MempoolOrigin, MempoolOutcome, PolicyTime, RelayIntent, transaction_weight_and_virtual_size,
};

pub(crate) const MAX_MEMPOOL_SNAPSHOT_RECORDS: usize = 50_000;
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
    /// Deprecated Wave 1 compatibility identity; removed by Plan 135-02.
    pub txid: Txid,
    /// Deprecated Wave 1 compatibility identity; removed by Plan 135-02.
    pub wtxid: Wtxid,
    /// Deprecated Wave 1 compatibility input; never encoded by v2.
    pub fee_sats: i64,
    /// Deprecated Wave 1 compatibility input; never encoded by v2.
    pub virtual_size: usize,
    /// Deprecated Wave 1 recovery input; never encoded by v2.
    pub metadata: MempoolEntryMetadata,
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
        if records.len() > MAX_MEMPOOL_SNAPSHOT_RECORDS
            || unbroadcast_members.len() > MAX_MEMPOOL_SNAPSHOT_UNBROADCAST_MEMBERS
        {
            return Err(MempoolSnapshotError::ResourceBoundExceeded);
        }
        if records.iter().any(|record| {
            !matches!(record.acceptance_time, MempoolAcceptanceTime::Known(_))
                || record.metadata.accepted_at != record.acceptance_time
        }) {
            return Err(MempoolSnapshotError::StructuralCorruption);
        }

        let record_members = records
            .iter()
            .map(MempoolSnapshotRecord::member_identity)
            .collect::<BTreeSet<_>>();
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

    pub fn from_mempool(mempool: &Mempool) -> Self {
        let mut records = mempool
            .entries()
            .values()
            .map(|entry| MempoolSnapshotRecord {
                transaction: entry.transaction.clone(),
                acceptance_time: entry.metadata.accepted_at,
                txid: entry.txid,
                wtxid: entry.wtxid,
                fee_sats: entry.fee_sats(),
                virtual_size: entry.virtual_size.as_usize(),
                metadata: entry.metadata,
            })
            .collect::<Vec<_>>();
        records.sort_by_key(|record| record.txid);

        Self::from_legacy_v1(records)
    }

    pub fn replay_into_mempool(
        &self,
        mempool: &mut Mempool,
        chainstate: &ChainstateSnapshot,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Vec<MempoolRecoveryRecord> {
        self.records
            .iter()
            .map(|record| {
                let status = if transaction_is_confirmed(record, chainstate) {
                    MempoolRecoveryStatus::DroppedConfirmed
                } else {
                    recovery_status_from_outcome(
                        mempool
                            .accept_transaction_transition_with_context(
                                record.transaction.clone(),
                                chainstate,
                                verify_flags,
                                consensus_params,
                                AdmissionContext::recovery(record.metadata),
                            )
                            .map(|transition| transition.outcome),
                    )
                };
                MempoolRecoveryRecord {
                    txid: record.txid,
                    status,
                }
            })
            .collect()
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
            txid,
            wtxid,
            fee_sats,
            virtual_size,
            metadata,
        })
    }

    pub fn try_from_canonical(
        transaction: Transaction,
        acceptance_time: MempoolAcceptanceTime,
    ) -> Result<Self, MempoolSnapshotError> {
        if !matches!(acceptance_time, MempoolAcceptanceTime::Known(_)) {
            return Err(MempoolSnapshotError::StructuralCorruption);
        }
        let txid = transaction_txid(&transaction)
            .map_err(|_| MempoolSnapshotError::StructuralCorruption)?;
        let wtxid = transaction_wtxid(&transaction)
            .map_err(|_| MempoolSnapshotError::StructuralCorruption)?;
        let (_, virtual_size) = transaction_weight_and_virtual_size(&transaction)
            .map_err(|_| MempoolSnapshotError::StructuralCorruption)?;
        let metadata = MempoolEntryMetadata::new(
            acceptance_time,
            MempoolOrigin::RecoveryUnknown,
            RelayIntent::NotRequested,
        );

        Ok(Self {
            transaction,
            acceptance_time,
            txid,
            wtxid,
            fee_sats: 0,
            virtual_size,
            metadata,
        })
    }

    pub const fn member_identity(&self) -> MempoolMemberIdentity {
        MempoolMemberIdentity {
            txid: self.txid,
            wtxid: self.wtxid,
        }
    }
}

pub(crate) fn transaction_is_confirmed(
    record: &MempoolSnapshotRecord,
    chainstate: &ChainstateSnapshot,
) -> bool {
    (0..record.transaction.outputs.len()).any(|index| {
        let Ok(vout) = u32::try_from(index) else {
            return false;
        };
        chainstate.utxos.contains_key(&OutPoint {
            txid: record.txid,
            vout,
        })
    })
}

pub(crate) fn recovery_status_from_outcome(
    outcome: Result<MempoolOutcome, open_bitcoin_mempool::MempoolError>,
) -> MempoolRecoveryStatus {
    match outcome {
        Ok(MempoolOutcome::Accepted { .. }) | Ok(MempoolOutcome::Replaced { .. }) => {
            MempoolRecoveryStatus::Recovered
        }
        Ok(MempoolOutcome::Duplicate { .. }) => MempoolRecoveryStatus::DroppedDuplicate,
        Ok(MempoolOutcome::Orphaned { .. }) => MempoolRecoveryStatus::DroppedMissingParent,
        Ok(MempoolOutcome::Rejected { .. }) | Err(_) => {
            MempoolRecoveryStatus::DroppedPolicyIncompatible
        }
        Ok(MempoolOutcome::Evicted { .. }) | Ok(MempoolOutcome::Expired { .. }) => {
            MempoolRecoveryStatus::DroppedEvicted
        }
    }
}

#[cfg(test)]
#[path = "mempool_snapshot/tests.rs"]
mod tests;
