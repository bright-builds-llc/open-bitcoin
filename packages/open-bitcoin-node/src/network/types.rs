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
use open_bitcoin_mempool::{MempoolCapacityStatus, MempoolError, RollingFeeParityStatus};
use open_bitcoin_network::{NetworkError, PeerId, WireNetworkMessage};

use super::ManagedBlockServeIntent;

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
}

impl core::fmt::Display for ManagedNetworkError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Network(error) => error.fmt(f),
            Self::Chainstate(error) => error.fmt(f),
            Self::Mempool(error) => error.fmt(f),
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

impl From<CodecError> for ManagedNetworkError {
    fn from(value: CodecError) -> Self {
        Self::Network(NetworkError::from(value))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManagedMempoolInfo {
    pub transaction_count: usize,
    pub total_virtual_size: usize,
    pub total_fee_sats: i64,
    pub min_relay_feerate_sats_per_kvb: i64,
    pub incremental_relay_feerate_sats_per_kvb: i64,
    pub max_mempool_virtual_size: usize,
    pub capacity_status: MempoolCapacityStatus,
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
    pub block_serve_intents: Vec<ManagedBlockServeIntent>,
}
