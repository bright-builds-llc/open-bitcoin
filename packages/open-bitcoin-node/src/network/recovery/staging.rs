// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/test/functional/mempool_persist.py

//! Side-effect-free mempool snapshot replay into a complete startup candidate.

// The staged candidate is installed by the atomic startup cutover in Plan 135-03.
#![allow(dead_code)]

use std::collections::{BTreeMap, BTreeSet};

use open_bitcoin_core::chainstate::{ChainPosition, ChainstateSnapshot};
use open_bitcoin_core::consensus::{ConsensusParams, ScriptVerifyFlags};
use open_bitcoin_mempool::{
    AdmissionContext, Mempool, MempoolEntryMetadata, MempoolMemberIdentity, MempoolOrigin,
    MempoolOutcome, MempoolRemovalCause, PolicyConfig, PolicyTime, RelayIntent,
};

use crate::storage::mempool_snapshot::CapturedMempoolGeneration;
use crate::storage::{MempoolRecoveryRecord, MempoolRecoveryStatus, MempoolSnapshot};

use super::ManagedNetworkError;
use super::topology::{RecoveryTopologyLimits, TopologyRecord, prepare_recovery_topology};
use crate::network::lifecycle_projection::AuthorityEpoch;

#[derive(Debug)]
pub struct PreparedMempoolRecovery {
    pub(in crate::network) authority_epoch: AuthorityEpoch,
    pub(in crate::network) maybe_chainstate_tip: Option<ChainPosition>,
    pub(crate) staged_mempool: Mempool,
    pub(crate) recovery_records: Vec<MempoolRecoveryRecord>,
    pub(crate) unbroadcast_members: BTreeSet<MempoolMemberIdentity>,
    #[allow(dead_code)] // Consumed by the atomic projection install in Plan 135-03.
    pub(crate) ordered_survivors: Vec<MempoolMemberIdentity>,
    pub(crate) captured_generation: Option<CapturedMempoolGeneration>,
    pub(crate) captured_at: Option<PolicyTime>,
    pub(crate) startup_at: PolicyTime,
}

impl PreparedMempoolRecovery {
    pub(crate) const fn staged_mempool(&self) -> &Mempool {
        &self.staged_mempool
    }

    pub(crate) fn recovery_records(&self) -> &[MempoolRecoveryRecord] {
        &self.recovery_records
    }

    pub(crate) const fn unbroadcast_members(&self) -> &BTreeSet<MempoolMemberIdentity> {
        &self.unbroadcast_members
    }

    #[allow(dead_code)] // Consumed by the atomic projection install in Plan 135-03.
    pub(crate) fn ordered_survivors(&self) -> &[MempoolMemberIdentity] {
        &self.ordered_survivors
    }

    pub(crate) const fn captured_generation(&self) -> Option<CapturedMempoolGeneration> {
        self.captured_generation
    }

    pub(crate) const fn captured_at(&self) -> Option<PolicyTime> {
        self.captured_at
    }

    pub(crate) const fn startup_at(&self) -> PolicyTime {
        self.startup_at
    }
}

pub(super) fn prepare_mempool_recovery(
    snapshot: &MempoolSnapshot,
    chainstate: &ChainstateSnapshot,
    verify_flags: ScriptVerifyFlags,
    consensus_params: ConsensusParams,
    config: PolicyConfig,
    startup_at: PolicyTime,
    authority_epoch: AuthorityEpoch,
) -> Result<PreparedMempoolRecovery, ManagedNetworkError> {
    let topology = prepare_recovery_topology(&snapshot.records, RecoveryTopologyLimits::standard())
        .map_err(|_| {
            ManagedNetworkError::LifecycleEffect("mempool recovery topology preparation failed")
        })?;
    let (ordered, mut recovery_records) = topology.into_parts();
    let persisted_unbroadcast = snapshot.unbroadcast_members();
    let mut working = Mempool::new(config.clone());
    let mut admitted = BTreeSet::new();
    let mut immediate = BTreeMap::new();

    for topology_record in &ordered {
        let identity = topology_record.identity;
        if transaction_is_confirmed(topology_record, chainstate) {
            immediate.insert(identity.txid, MempoolRecoveryStatus::DroppedConfirmed);
            continue;
        }

        let metadata = recovery_metadata(
            topology_record.record.acceptance_time,
            persisted_unbroadcast.contains(&identity),
        );
        let transition = working.accept_transaction_transition_with_context(
            topology_record.record.transaction.clone(),
            chainstate,
            verify_flags,
            consensus_params,
            AdmissionContext::recovery(metadata),
        )?;
        match transition.outcome {
            MempoolOutcome::Accepted { .. } | MempoolOutcome::Replaced { .. } => {
                admitted.insert(identity.txid);
            }
            MempoolOutcome::Duplicate { .. } => {
                immediate.insert(identity.txid, MempoolRecoveryStatus::DroppedDuplicate);
            }
            MempoolOutcome::Orphaned { .. } => {
                immediate.insert(identity.txid, MempoolRecoveryStatus::DroppedMissingParent);
            }
            MempoolOutcome::Rejected { .. } => {
                immediate.insert(
                    identity.txid,
                    MempoolRecoveryStatus::DroppedPolicyIncompatible,
                );
            }
            MempoolOutcome::Evicted { .. } => {
                immediate.insert(identity.txid, MempoolRecoveryStatus::DroppedEvicted);
            }
            MempoolOutcome::Expired { .. } => {
                immediate.insert(identity.txid, MempoolRecoveryStatus::DroppedExpired);
            }
        }
    }

    let expiry_delta = working.expire(startup_at)?;
    let expired = expiry_delta
        .removed
        .iter()
        .filter(|removal| removal.cause == MempoolRemovalCause::Expiry)
        .map(|removal| removal.member.txid)
        .collect::<BTreeSet<_>>();
    let final_members = working
        .entries()
        .iter()
        .map(|(txid, entry)| MempoolMemberIdentity {
            txid: *txid,
            wtxid: entry.wtxid,
        })
        .collect::<BTreeSet<_>>();
    let final_txids = final_members
        .iter()
        .map(|identity| identity.txid)
        .collect::<BTreeSet<_>>();
    let final_unbroadcast = persisted_unbroadcast
        .intersection(&final_members)
        .copied()
        .collect::<BTreeSet<_>>();

    for topology_record in &ordered {
        let txid = topology_record.identity.txid;
        let status = immediate.get(&txid).copied().unwrap_or_else(|| {
            if expired.contains(&txid) {
                MempoolRecoveryStatus::DroppedExpired
            } else if final_txids.contains(&txid) {
                MempoolRecoveryStatus::Recovered
            } else if admitted.contains(&txid) {
                MempoolRecoveryStatus::DroppedEvicted
            } else {
                MempoolRecoveryStatus::DroppedPolicyIncompatible
            }
        });
        recovery_records.push(MempoolRecoveryRecord { txid, status });
    }
    recovery_records.sort_by_key(|record| (record.txid, status_rank(record.status)));

    let mut staged_mempool = Mempool::new(config);
    let mut ordered_survivors = Vec::with_capacity(final_txids.len());
    for topology_record in ordered {
        let identity = topology_record.identity;
        if !final_txids.contains(&identity.txid) {
            continue;
        }
        let metadata = recovery_metadata(
            topology_record.record.acceptance_time,
            final_unbroadcast.contains(&identity),
        );
        let transition = staged_mempool.accept_transaction_transition_with_context(
            topology_record.record.transaction,
            chainstate,
            verify_flags,
            consensus_params,
            AdmissionContext::recovery(metadata),
        )?;
        if !matches!(transition.outcome, MempoolOutcome::Accepted { .. }) {
            return Err(ManagedNetworkError::LifecycleEffect(
                "final mempool recovery reconstruction diverged",
            ));
        }
        ordered_survivors.push(identity);
    }

    if staged_mempool
        .entries()
        .keys()
        .copied()
        .collect::<BTreeSet<_>>()
        != final_txids
    {
        return Err(ManagedNetworkError::LifecycleEffect(
            "final mempool recovery membership diverged",
        ));
    }

    Ok(PreparedMempoolRecovery {
        authority_epoch,
        maybe_chainstate_tip: chainstate.tip().cloned(),
        staged_mempool,
        recovery_records,
        unbroadcast_members: final_unbroadcast,
        ordered_survivors,
        captured_generation: snapshot.captured_generation(),
        captured_at: snapshot.captured_at(),
        startup_at,
    })
}

fn transaction_is_confirmed(record: &TopologyRecord, chainstate: &ChainstateSnapshot) -> bool {
    chainstate
        .maybe_confirmed_txids
        .as_ref()
        .is_some_and(|confirmed_txids| confirmed_txids.contains(&record.identity.txid))
}

fn recovery_metadata(
    acceptance_time: open_bitcoin_mempool::MempoolAcceptanceTime,
    local_requested: bool,
) -> MempoolEntryMetadata {
    MempoolEntryMetadata::new(
        acceptance_time,
        if local_requested {
            MempoolOrigin::Local
        } else {
            MempoolOrigin::RecoveryUnknown
        },
        if local_requested {
            RelayIntent::Requested
        } else {
            RelayIntent::NotRequested
        },
    )
}

const fn status_rank(status: MempoolRecoveryStatus) -> u8 {
    match status {
        MempoolRecoveryStatus::Recovered => 0,
        MempoolRecoveryStatus::DroppedConfirmed => 1,
        MempoolRecoveryStatus::DroppedDuplicate => 2,
        MempoolRecoveryStatus::DroppedMissingParent => 3,
        MempoolRecoveryStatus::DroppedPolicyIncompatible => 4,
        MempoolRecoveryStatus::DroppedExpired => 5,
        MempoolRecoveryStatus::DroppedEvicted => 6,
    }
}
