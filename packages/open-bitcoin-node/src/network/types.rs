// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/node/txdownloadman.h
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/protocol.h
// - packages/bitcoin-knots/src/txorphanage.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/test/functional/mempool_persist.py
// - packages/bitcoin-knots/test/functional/p2p_getdata.py
// - packages/bitcoin-knots/test/functional/p2p_orphan_handling.py
// - packages/bitcoin-knots/test/functional/p2p_tx_download.py
// - packages/bitcoin-knots/test/functional/mempool_accept.py

use open_bitcoin_core::{
    chainstate::{ChainPosition, ChainstateError},
    codec::CodecError,
    primitives::{BlockHash, NetworkMagic},
};
use open_bitcoin_mempool::{
    MempoolCapacityEnforcement, MempoolCapacityStatus, MempoolError, PackageShapeError,
    RollingFeeParityStatus,
};
use open_bitcoin_network::{NetworkError, PeerId, WireNetworkMessage};

use crate::status::{BlockRelayEvidenceStatus, relay_evidence::RelayEvidenceStatus};

use super::{
    ManagedAddressBoundaryInfo, ManagedBlockServeIntent, ManagedInboundAdmissionInfo,
    ManagedPeerPolicyInfo, ManagedResourceGovernanceInfo,
};

/// One response step in the exact peer-request order chosen under the network authority.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ManagedInboundResponsePlanItem {
    Immediate(WireNetworkMessage),
    DurableBlock(ManagedBlockServeIntent),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManagedBlockSerializationMode {
    Block,
    WitnessBlock,
    CompactBlock,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManagedBlockServeCompletionOutcome {
    LookupUnavailable,
    TransportFailed,
    Written,
}

#[derive(Debug)]
pub enum ManagedNetworkError {
    Network(NetworkError),
    Chainstate(ChainstateError),
    Mempool(MempoolError),
    PackageShape(PackageShapeError),
    LifecycleEffect(&'static str),
}

impl core::fmt::Display for ManagedNetworkError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Network(error) => error.fmt(f),
            Self::Chainstate(error) => error.fmt(f),
            Self::Mempool(error) => error.fmt(f),
            Self::PackageShape(error) => error.fmt(f),
            Self::LifecycleEffect(message) => f.write_str(message),
        }
    }
}

impl std::error::Error for ManagedNetworkError {}

impl From<NetworkError> for ManagedNetworkError {
    fn from(value: NetworkError) -> Self {
        Self::Network(value)
    }
}

impl From<ChainstateError> for ManagedNetworkError {
    fn from(value: ChainstateError) -> Self {
        Self::Chainstate(value)
    }
}

impl From<MempoolError> for ManagedNetworkError {
    fn from(value: MempoolError) -> Self {
        Self::Mempool(value)
    }
}

impl From<PackageShapeError> for ManagedNetworkError {
    fn from(value: PackageShapeError) -> Self {
        Self::PackageShape(value)
    }
}

impl From<CodecError> for ManagedNetworkError {
    fn from(value: CodecError) -> Self {
        Self::Network(NetworkError::from(value))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManagedMempoolInfo {
    pub transaction_count: usize,
    pub total_virtual_size: usize,
    pub accounted_memory: usize,
    pub mempool_capacity: usize,
    pub total_fee_sats: i64,
    pub static_relay_fee_rate_sats_per_kvb: i64,
    pub incremental_relay_fee_rate_sats_per_kvb: i64,
    pub rolling_mempool_fee_rate_sats_per_kvb: i64,
    pub effective_admission_fee_rate_sats_per_kvb: i64,
    pub capacity_status: MempoolCapacityStatus,
    pub capacity_enforcement: MempoolCapacityEnforcement,
    pub rolling_fee_parity: RollingFeeParityStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManagedNetworkInfo {
    pub network_magic: NetworkMagic,
    pub protocol_version: i32,
    pub user_agent: String,
    pub local_services_bits: u64,
    pub relay: bool,
    pub connected_peers: usize,
    pub inbound_peers: usize,
    pub outbound_peers: usize,
    pub wtxidrelay_peers: usize,
    pub header_preferring_peers: usize,
}

/// Owned, sanitized network state captured under one authoritative read guard.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManagedNetworkOperatorSnapshot {
    pub(super) network: ManagedNetworkInfo,
    pub(super) mempool: ManagedMempoolInfo,
    pub(super) relay: RelayEvidenceStatus,
    pub(super) block_relay: BlockRelayEvidenceStatus,
    pub(super) block_served_count: u64,
    pub(super) inbound_admission: ManagedInboundAdmissionInfo,
    pub(super) address_boundary: ManagedAddressBoundaryInfo,
    pub(super) peer_policy: ManagedPeerPolicyInfo,
    pub(super) resource_governance: ManagedResourceGovernanceInfo,
}

impl ManagedNetworkOperatorSnapshot {
    pub fn network(&self) -> &ManagedNetworkInfo {
        &self.network
    }

    pub fn mempool(&self) -> &ManagedMempoolInfo {
        &self.mempool
    }

    pub fn relay(&self) -> &RelayEvidenceStatus {
        &self.relay
    }

    pub fn block_relay(&self) -> &BlockRelayEvidenceStatus {
        &self.block_relay
    }

    pub const fn block_served_count(&self) -> u64 {
        self.block_served_count
    }

    pub(crate) fn block_relay_runtime_snapshot(&self) -> super::BlockRelayRuntimeEvidenceSnapshot {
        super::BlockRelayRuntimeEvidenceSnapshot {
            status: self.block_relay.clone(),
            served_count: self.block_served_count,
        }
    }

    pub fn inbound_admission(&self) -> &ManagedInboundAdmissionInfo {
        &self.inbound_admission
    }

    pub fn address_boundary(&self) -> &ManagedAddressBoundaryInfo {
        &self.address_boundary
    }

    pub fn peer_policy(&self) -> &ManagedPeerPolicyInfo {
        &self.peer_policy
    }

    pub fn resource_governance(&self) -> &ManagedResourceGovernanceInfo {
        &self.resource_governance
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockConnectDisposition {
    Connected(ChainPosition),
    Duplicate(BlockHash),
    NonExtending {
        block_hash: BlockHash,
        previous_block_hash: BlockHash,
    },
    Disconnected {
        block_hash: BlockHash,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManagedSyncMessageResult {
    pub outbound: Vec<WireNetworkMessage>,
    pub targeted_outbound: Vec<(PeerId, WireNetworkMessage)>,
    pub maybe_block_disposition: Option<BlockConnectDisposition>,
    pub inbound_response_plan: Vec<ManagedInboundResponsePlanItem>,
    pub(super) package_admissions: Vec<super::admission_bridge::ManagedPeerPackageAdmission>,
}
