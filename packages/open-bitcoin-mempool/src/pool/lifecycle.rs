// Parity breadcrumbs:
// - packages/bitcoin-knots/src/txmempool.h
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/validation.cpp

use std::collections::{BTreeMap, HashMap};

use open_bitcoin_codec::CodecError;
use open_bitcoin_consensus::transaction_txid;
use open_bitcoin_primitives::{Block, Transaction, Txid, Wtxid};

use super::{
    Mempool, MempoolEntry, MempoolError, collect_conflicts_and_descendants, recompute_state,
    resource_invariant_error,
};
use crate::{AccountedMempoolMemory, MempoolCapacity, TransactionVirtualSize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MempoolLifecycleRemovalReason {
    Confirmed,
    Conflict,
    Descendant,
    Trimmed,
}

impl MempoolLifecycleRemovalReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Confirmed => "confirmed",
            Self::Conflict => "conflict",
            Self::Descendant => "descendant",
            Self::Trimmed => "trimmed",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MempoolCapacityStatus {
    Empty,
    UnderCapacity,
    AtCapacity,
    OverCapacity,
}

impl MempoolCapacityStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Empty => "empty",
            Self::UnderCapacity => "under_capacity",
            Self::AtCapacity => "at_capacity",
            Self::OverCapacity => "over_capacity",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RollingFeeParityStatus {
    Deferred,
}

impl RollingFeeParityStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Deferred => "deferred",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MempoolPressureSummary {
    pub transaction_count: usize,
    pub total_virtual_size: TransactionVirtualSize,
    pub accounted_memory: AccountedMempoolMemory,
    pub mempool_capacity: MempoolCapacity,
    pub min_relay_feerate_sats_per_kvb: i64,
    pub incremental_relay_feerate_sats_per_kvb: i64,
    pub capacity_status: MempoolCapacityStatus,
    pub rolling_fee_parity: RollingFeeParityStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MempoolLifecycleRemoval {
    pub txid: Txid,
    pub wtxid: Wtxid,
    pub reason: MempoolLifecycleRemovalReason,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MempoolLifecycleSummary {
    pub removed: Vec<MempoolLifecycleRemoval>,
    pub pressure: MempoolPressureSummary,
}

impl Mempool {
    pub fn pressure_summary(&self) -> MempoolPressureSummary {
        let capacity_status =
            capacity_status(self.accounted_memory(), self.config.mempool_capacity);
        MempoolPressureSummary {
            transaction_count: self.entries.len(),
            total_virtual_size: self.total_virtual_size(),
            accounted_memory: self.accounted_memory(),
            mempool_capacity: self.config.mempool_capacity,
            min_relay_feerate_sats_per_kvb: self
                .config
                .static_relay_fee_rate
                .fee_rate()
                .sats_per_kvb(),
            incremental_relay_feerate_sats_per_kvb: self
                .config
                .incremental_relay_fee_rate
                .fee_rate()
                .sats_per_kvb(),
            capacity_status,
            rolling_fee_parity: RollingFeeParityStatus::Deferred,
        }
    }

    pub fn remove_for_connected_block(
        &mut self,
        block: &Block,
    ) -> Result<MempoolLifecycleSummary, MempoolError> {
        self.remove_for_connected_transactions(block.transactions.iter())
    }

    pub fn remove_for_connected_transactions<'a>(
        &mut self,
        transactions: impl IntoIterator<Item = &'a Transaction>,
    ) -> Result<MempoolLifecycleSummary, MempoolError> {
        let mut reasons = BTreeMap::new();

        for transaction in transactions {
            let txid = transaction_txid(transaction).map_err(txid_serialization_error)?;
            if self.entries.contains_key(&txid) {
                record_reason(&mut reasons, txid, MempoolLifecycleRemovalReason::Confirmed);
            }

            let mut direct_conflicts = self.direct_conflicts(transaction);
            direct_conflicts.remove(&txid);
            let conflict_package =
                collect_conflicts_and_descendants(&self.entries, &direct_conflicts);
            for conflict_txid in direct_conflicts {
                record_reason(
                    &mut reasons,
                    conflict_txid,
                    MempoolLifecycleRemovalReason::Conflict,
                );
            }
            for package_txid in conflict_package {
                record_reason(
                    &mut reasons,
                    package_txid,
                    MempoolLifecycleRemovalReason::Descendant,
                );
            }
        }

        if reasons.is_empty() {
            return Ok(MempoolLifecycleSummary {
                removed: Vec::new(),
                pressure: self.pressure_summary(),
            });
        }

        let mut removed = Vec::with_capacity(reasons.len());
        for (txid, reason) in reasons {
            removed.extend(remove_lifecycle_entry(&mut self.entries, txid, reason));
        }

        let state =
            recompute_state(std::mem::take(&mut self.entries)).map_err(resource_invariant_error)?;
        self.entries = state.entries;
        self.spent_outpoints = state.spent_outpoints;
        self.resource_ledger = state.resource_ledger;

        Ok(MempoolLifecycleSummary {
            removed,
            pressure: self.pressure_summary(),
        })
    }
}

pub(super) fn capacity_status(
    accounted_memory: AccountedMempoolMemory,
    mempool_capacity: MempoolCapacity,
) -> MempoolCapacityStatus {
    if accounted_memory == AccountedMempoolMemory::ZERO {
        return MempoolCapacityStatus::Empty;
    }
    if accounted_memory.as_usize() < mempool_capacity.as_usize() {
        return MempoolCapacityStatus::UnderCapacity;
    }
    if accounted_memory.as_usize() == mempool_capacity.as_usize() {
        return MempoolCapacityStatus::AtCapacity;
    }

    MempoolCapacityStatus::OverCapacity
}

pub(super) fn record_reason(
    reasons: &mut BTreeMap<Txid, MempoolLifecycleRemovalReason>,
    txid: Txid,
    reason: MempoolLifecycleRemovalReason,
) {
    let maybe_existing = reasons.get(&txid).copied();
    let should_replace =
        maybe_existing.is_none_or(|existing| reason.priority() < existing.priority());
    if should_replace {
        reasons.insert(txid, reason);
    }
}

pub(super) fn txid_serialization_error(source: CodecError) -> MempoolError {
    super::serialization_validation_error("transaction txid", source)
}

impl MempoolLifecycleRemovalReason {
    pub(super) fn priority(self) -> u8 {
        match self {
            Self::Confirmed => 0,
            Self::Conflict => 1,
            Self::Descendant => 2,
            Self::Trimmed => 3,
        }
    }
}

fn remove_lifecycle_entry(
    entries: &mut HashMap<Txid, MempoolEntry>,
    txid: Txid,
    reason: MempoolLifecycleRemovalReason,
) -> Option<MempoolLifecycleRemoval> {
    entries.remove(&txid).map(|entry| MempoolLifecycleRemoval {
        txid,
        wtxid: entry.wtxid,
        reason,
    })
}
