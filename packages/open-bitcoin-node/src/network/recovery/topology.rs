// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/test/functional/mempool_persist.py

//! Pure dependency analysis for staged mempool recovery.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use open_bitcoin_core::consensus::{transaction_txid, transaction_wtxid};
use open_bitcoin_core::primitives::{Txid, Wtxid};
use open_bitcoin_mempool::{MempoolCapacityBounds, MempoolMemberIdentity, PolicyConfig};

use crate::storage::mempool_snapshot::MempoolSnapshotError;
use crate::storage::{MempoolRecoveryRecord, MempoolRecoveryStatus, MempoolSnapshotRecord};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct RecoveryTopologyLimits {
    max_vertices: usize,
    max_edges: usize,
    max_parent_edges_per_record: usize,
}

impl RecoveryTopologyLimits {
    #[cfg(test)]
    pub(crate) fn standard() -> Self {
        Self::from_policy(&PolicyConfig::default())
    }

    pub(crate) fn from_policy(policy: &PolicyConfig) -> Self {
        let bounds = MempoolCapacityBounds::from_capacity(policy.mempool_capacity);
        Self {
            max_vertices: bounds.max_live_entries(),
            max_edges: bounds.max_live_input_edges(),
            max_parent_edges_per_record: bounds.max_live_input_edges(),
        }
    }

    #[cfg(test)]
    pub(crate) const fn new(max_vertices: usize, max_parent_edges_per_record: usize) -> Self {
        Self {
            max_vertices,
            max_edges: max_vertices.saturating_mul(max_parent_edges_per_record),
            max_parent_edges_per_record,
        }
    }
}

#[derive(Debug)]
pub(crate) struct PreparedRecoveryTopology {
    ordered: Vec<TopologyRecord>,
    statuses: Vec<MempoolRecoveryRecord>,
}

#[derive(Debug)]
pub(super) struct TopologyRecord {
    pub(super) record: MempoolSnapshotRecord,
    pub(super) identity: MempoolMemberIdentity,
}

impl PreparedRecoveryTopology {
    #[cfg(test)]
    pub(crate) fn ordered_identities(&self) -> impl Iterator<Item = (Txid, Wtxid)> + '_ {
        self.ordered
            .iter()
            .map(|record| (record.identity.txid, record.identity.wtxid))
    }

    #[cfg(test)]
    pub(crate) fn status_for(&self, txid: Txid) -> Option<MempoolRecoveryStatus> {
        self.statuses
            .iter()
            .find(|record| record.txid == txid)
            .map(|record| record.status)
    }

    pub(super) fn into_parts(self) -> (Vec<TopologyRecord>, Vec<MempoolRecoveryRecord>) {
        (self.ordered, self.statuses)
    }
}

pub(crate) fn prepare_recovery_topology(
    records: &[MempoolSnapshotRecord],
    limits: RecoveryTopologyLimits,
) -> Result<PreparedRecoveryTopology, MempoolSnapshotError> {
    if records.len() > limits.max_vertices {
        return Err(MempoolSnapshotError::ResourceBoundExceeded);
    }

    let mut canonical = records
        .iter()
        .cloned()
        .map(|record| {
            let txid = transaction_txid(&record.transaction)
                .map_err(|_| MempoolSnapshotError::StructuralCorruption)?;
            let wtxid = transaction_wtxid(&record.transaction)
                .map_err(|_| MempoolSnapshotError::StructuralCorruption)?;
            Ok(TopologyRecord {
                record,
                identity: MempoolMemberIdentity { txid, wtxid },
            })
        })
        .collect::<Result<Vec<_>, MempoolSnapshotError>>()?;
    canonical.sort_by_key(|record| (record.identity.txid, record.identity.wtxid));

    let mut primary_by_txid = BTreeMap::<Txid, usize>::new();
    let mut statuses = Vec::<MempoolRecoveryRecord>::new();
    let mut duplicate_indices = BTreeSet::new();
    for (index, record) in canonical.iter().enumerate() {
        if primary_by_txid
            .insert(record.identity.txid, index)
            .is_some()
        {
            duplicate_indices.insert(index);
            statuses.push(MempoolRecoveryRecord {
                txid: record.identity.txid,
                status: MempoolRecoveryStatus::DroppedDuplicate,
            });
        }
    }
    // Retain the lexicographically first witness identity as the deterministic
    // primary for a shared txid.
    primary_by_txid.clear();
    for (index, record) in canonical.iter().enumerate() {
        primary_by_txid.entry(record.identity.txid).or_insert(index);
    }

    let mut indegree = vec![0_usize; canonical.len()];
    let mut children = BTreeMap::<usize, BTreeSet<usize>>::new();
    let mut failed = duplicate_indices;
    let mut edge_count = 0_usize;
    for (index, record) in canonical.iter().enumerate() {
        if failed.contains(&index) {
            continue;
        }
        let parents = record
            .record
            .transaction
            .inputs
            .iter()
            .filter_map(|input| primary_by_txid.get(&input.previous_output.txid).copied())
            .collect::<BTreeSet<_>>();
        if parents.len() > limits.max_parent_edges_per_record {
            failed.insert(index);
            statuses.push(MempoolRecoveryRecord {
                txid: record.identity.txid,
                status: MempoolRecoveryStatus::DroppedMissingParent,
            });
            continue;
        }
        edge_count = edge_count
            .checked_add(parents.len())
            .ok_or(MempoolSnapshotError::ResourceBoundExceeded)?;
        if edge_count > limits.max_edges {
            return Err(MempoolSnapshotError::ResourceBoundExceeded);
        }
        indegree[index] = parents.len();
        for parent in parents {
            children.entry(parent).or_default().insert(index);
        }
    }

    propagate_failed_descendants(&canonical, &children, &mut failed, &mut statuses);

    let mut ready = BTreeSet::<(Txid, Wtxid, usize)>::new();
    for (index, record) in canonical.iter().enumerate() {
        if !failed.contains(&index) && indegree[index] == 0 {
            ready.insert((record.identity.txid, record.identity.wtxid, index));
        }
    }

    let mut ordered_indices = Vec::with_capacity(canonical.len().saturating_sub(failed.len()));
    let mut ordered_set = BTreeSet::new();
    while let Some(next) = ready.pop_first() {
        let index = next.2;
        ordered_indices.push(index);
        ordered_set.insert(index);
        let Some(record_children) = children.get(&index) else {
            continue;
        };
        for child in record_children {
            if failed.contains(child) {
                continue;
            }
            indegree[*child] = indegree[*child]
                .checked_sub(1)
                .ok_or(MempoolSnapshotError::StructuralCorruption)?;
            if indegree[*child] == 0 {
                let identity = canonical[*child].identity;
                ready.insert((identity.txid, identity.wtxid, *child));
            }
        }
    }

    if ordered_indices.len().checked_add(failed.len()) != Some(canonical.len()) {
        for (index, record) in canonical.iter().enumerate() {
            if failed.contains(&index) || ordered_set.contains(&index) {
                continue;
            }
            failed.insert(index);
            statuses.push(MempoolRecoveryRecord {
                txid: record.identity.txid,
                status: MempoolRecoveryStatus::DroppedMissingParent,
            });
        }
        propagate_failed_descendants(&canonical, &children, &mut failed, &mut statuses);
    }

    let mut maybe_records = canonical.into_iter().map(Some).collect::<Vec<_>>();
    let ordered = ordered_indices
        .into_iter()
        .map(|index| {
            maybe_records[index]
                .take()
                .ok_or(MempoolSnapshotError::StructuralCorruption)
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(PreparedRecoveryTopology { ordered, statuses })
}

fn propagate_failed_descendants(
    records: &[TopologyRecord],
    children: &BTreeMap<usize, BTreeSet<usize>>,
    failed: &mut BTreeSet<usize>,
    statuses: &mut Vec<MempoolRecoveryRecord>,
) {
    let mut queue = failed.iter().copied().collect::<VecDeque<_>>();
    while let Some(parent) = queue.pop_front() {
        let Some(record_children) = children.get(&parent) else {
            continue;
        };
        for child in record_children {
            if !failed.insert(*child) {
                continue;
            }
            statuses.push(MempoolRecoveryRecord {
                txid: records[*child].identity.txid,
                status: MempoolRecoveryStatus::DroppedMissingParent,
            });
            queue.push_back(*child);
        }
    }
}
