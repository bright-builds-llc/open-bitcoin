// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Identifier-free mempool resource and fee-floor groups for shared status.

use serde::{Deserialize, Serialize};

use crate::network::ManagedMempoolInfo;

use super::{FieldAvailability, MempoolStatus, relay_evidence::RelayEvidenceStatus};

/// Reason used when a mempool group is absent from a legacy snapshot.
pub const MEMPOOL_GROUP_UNAVAILABLE_REASON: &str = "mempool group unavailable";

/// Distinct resource roles: virtual size, accounted usage, and accounted capacity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MempoolResourcesGroup {
    pub virtual_size: u64,
    pub accounted_usage: u64,
    pub accounted_capacity: u64,
    pub transaction_count: u64,
}

/// Distinct fee-floor roles in sat/kvB. Effective admission is not a Knots alias.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MempoolFeeFloorsGroup {
    pub static_relay_floor: i64,
    pub rolling_mempool_floor: i64,
    pub effective_admission_floor: i64,
    pub incremental_relay_fee: i64,
}

/// Maps managed mempool pressure into the shared resource group.
pub fn resources_from_managed_info(info: &ManagedMempoolInfo) -> MempoolResourcesGroup {
    MempoolResourcesGroup {
        virtual_size: info.total_virtual_size as u64,
        accounted_usage: info.accounted_memory as u64,
        accounted_capacity: info.mempool_capacity as u64,
        transaction_count: info.transaction_count as u64,
    }
}

/// Maps managed mempool fee roles 1:1 without swapping minrelay and incremental.
pub fn fee_floors_from_managed_info(info: &ManagedMempoolInfo) -> MempoolFeeFloorsGroup {
    MempoolFeeFloorsGroup {
        static_relay_floor: info.static_relay_fee_rate_sats_per_kvb,
        rolling_mempool_floor: info.rolling_mempool_fee_rate_sats_per_kvb,
        effective_admission_floor: info.effective_admission_fee_rate_sats_per_kvb,
        incremental_relay_fee: info.incremental_relay_fee_rate_sats_per_kvb,
    }
}

pub(super) fn mempool_resources_unavailable() -> FieldAvailability<MempoolResourcesGroup> {
    FieldAvailability::unavailable(MEMPOOL_GROUP_UNAVAILABLE_REASON)
}

pub(super) fn mempool_fee_floors_unavailable() -> FieldAvailability<MempoolFeeFloorsGroup> {
    FieldAvailability::unavailable(MEMPOOL_GROUP_UNAVAILABLE_REASON)
}

impl MempoolStatus {
    pub fn from_transactions_and_relay(
        transactions: FieldAvailability<u64>,
        relay: RelayEvidenceStatus,
    ) -> Self {
        Self {
            transactions,
            relay,
            ..Self::default()
        }
    }
}

impl Default for MempoolStatus {
    fn default() -> Self {
        Self {
            transactions: FieldAvailability::unavailable(MEMPOOL_GROUP_UNAVAILABLE_REASON),
            relay: RelayEvidenceStatus::default(),
            resources: mempool_resources_unavailable(),
            fee_floors: mempool_fee_floors_unavailable(),
        }
    }
}
