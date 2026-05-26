// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp

mod projection;
mod summary;

use std::{fmt, net::SocketAddr, path::PathBuf};

use open_bitcoin_core::{
    chainstate::ChainstateError, consensus::ConsensusParams, primitives::NetworkMagic,
};
use open_bitcoin_network::{NetworkError, WireNetworkMessage};

use crate::{
    ManagedNetworkError, PersistMode, StorageError,
    status::{HealthSignal, HealthSignalLevel},
};
use projection::storage_health_message;

const DEFAULT_CONNECT_TIMEOUT_MS: u64 = 5_000;
const DEFAULT_READ_TIMEOUT_MS: u64 = 5_000;
const DEFAULT_MAX_MESSAGES_PER_PEER: usize = 64;
const DEFAULT_MAX_SYNC_ROUNDS: usize = 8;
const DEFAULT_MAX_PEER_RETRIES: u8 = 1;
const DEFAULT_TARGET_OUTBOUND_PEERS: usize = 4;
const DEFAULT_RETRY_BACKOFF_MS: u64 = 1_000;
const DEFAULT_MAX_BLOCKS_IN_FLIGHT_PER_PEER: usize = 16;
const DEFAULT_MAX_BLOCKS_IN_FLIGHT_TOTAL: usize = 64;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyncNetwork {
    Mainnet,
    Testnet,
    Signet,
    Regtest,
}

impl SyncNetwork {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Mainnet => "mainnet",
            Self::Testnet => "testnet",
            Self::Signet => "signet",
            Self::Regtest => "regtest",
        }
    }

    pub const fn magic(self) -> NetworkMagic {
        match self {
            Self::Mainnet => NetworkMagic::MAINNET,
            Self::Testnet => NetworkMagic::from_bytes([0x0b, 0x11, 0x09, 0x07]),
            Self::Signet => NetworkMagic::from_bytes([0x0a, 0x03, 0xcf, 0x40]),
            Self::Regtest => NetworkMagic::from_bytes([0xfa, 0xbf, 0xb5, 0xda]),
        }
    }

    pub const fn default_port(self) -> u16 {
        match self {
            Self::Mainnet => 8333,
            Self::Testnet => 18_333,
            Self::Signet => 38_333,
            Self::Regtest => 18_444,
        }
    }

    pub const fn default_dns_seeds(self) -> &'static [&'static str] {
        match self {
            Self::Mainnet => &[
                "seed.bitcoin.sipa.be",
                "dnsseed.bluematt.me",
                "dnsseed.bitcoin.dashjr-list-of-p2p-nodes.us",
                "seed.bitcoinstats.com",
                "seed.bitcoin.jonasschnelli.ch",
            ],
            Self::Testnet => &[
                "testnet-seed.bitcoin.jonasschnelli.ch",
                "seed.tbtc.petertodd.net",
                "testnet-seed.bluematt.me",
            ],
            Self::Signet => &["seed.signet.bitcoin.sprovoost.nl"],
            Self::Regtest => &[],
        }
    }

    pub const fn consensus_params(self) -> ConsensusParams {
        match self {
            Self::Mainnet => network_consensus_params(0x1d00_ffff, false, false),
            Self::Testnet => network_consensus_params(0x1d00_ffff, true, false),
            Self::Signet => network_consensus_params(0x1e03_77ae, false, false),
            Self::Regtest => network_consensus_params(0x207f_ffff, true, true),
        }
    }
}

const fn network_consensus_params(
    pow_limit_bits: u32,
    allow_min_difficulty_blocks: bool,
    no_pow_retargeting: bool,
) -> ConsensusParams {
    ConsensusParams {
        coinbase_maturity: 100,
        subsidy_halving_interval: 210_000,
        locktime_threshold: 500_000_000,
        sequence_locktime_granularity: 9,
        pow_limit_bits,
        pow_target_spacing_seconds: 600,
        pow_target_timespan_seconds: 1_209_600,
        allow_min_difficulty_blocks,
        no_pow_retargeting,
        enforce_bip34_height_in_coinbase: true,
        enforce_bip113_median_time_past: true,
        enforce_segwit: true,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyncPeerSource {
    Manual,
    DnsSeed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyncPeerAddress {
    pub host: String,
    pub port: u16,
    pub source: SyncPeerSource,
}

impl SyncPeerAddress {
    pub fn manual(host: impl Into<String>, port: u16) -> Self {
        Self {
            host: host.into(),
            port,
            source: SyncPeerSource::Manual,
        }
    }

    pub fn dns_seed(host: impl Into<String>, port: u16) -> Self {
        Self {
            host: host.into(),
            port,
            source: SyncPeerSource::DnsSeed,
        }
    }

    pub(crate) fn label(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedSyncPeerAddress {
    pub peer: SyncPeerAddress,
    pub endpoint: SocketAddr,
}

impl ResolvedSyncPeerAddress {
    pub fn new(peer: SyncPeerAddress, endpoint: SocketAddr) -> Self {
        Self { peer, endpoint }
    }

    pub fn label(&self) -> String {
        format!("{} -> {}", self.peer.label(), self.endpoint)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyncRuntimeConfig {
    pub network: SyncNetwork,
    pub manual_peers: Vec<SyncPeerAddress>,
    pub dns_seeds: Vec<String>,
    pub target_outbound_peers: usize,
    pub connect_timeout_ms: u64,
    pub read_timeout_ms: u64,
    pub retry_backoff_ms: u64,
    pub max_messages_per_peer: usize,
    pub max_rounds: usize,
    pub max_peer_retries: u8,
    pub max_blocks_in_flight_per_peer: usize,
    pub max_blocks_in_flight_total: usize,
    pub persist_mode: PersistMode,
    pub maybe_log_dir: Option<PathBuf>,
}

impl SyncRuntimeConfig {
    pub fn candidate_peers(&self) -> Vec<SyncPeerAddress> {
        let mut peers = self.manual_peers.clone();
        peers.extend(
            self.dns_seeds
                .iter()
                .cloned()
                .map(|seed| SyncPeerAddress::dns_seed(seed, self.network.default_port())),
        );
        peers
    }
}

impl Default for SyncRuntimeConfig {
    fn default() -> Self {
        Self {
            network: SyncNetwork::Mainnet,
            manual_peers: Vec::new(),
            dns_seeds: SyncNetwork::Mainnet
                .default_dns_seeds()
                .iter()
                .map(|seed| (*seed).to_string())
                .collect(),
            target_outbound_peers: DEFAULT_TARGET_OUTBOUND_PEERS,
            connect_timeout_ms: DEFAULT_CONNECT_TIMEOUT_MS,
            read_timeout_ms: DEFAULT_READ_TIMEOUT_MS,
            retry_backoff_ms: DEFAULT_RETRY_BACKOFF_MS,
            max_messages_per_peer: DEFAULT_MAX_MESSAGES_PER_PEER,
            max_rounds: DEFAULT_MAX_SYNC_ROUNDS,
            max_peer_retries: DEFAULT_MAX_PEER_RETRIES,
            max_blocks_in_flight_per_peer: DEFAULT_MAX_BLOCKS_IN_FLIGHT_PER_PEER,
            max_blocks_in_flight_total: DEFAULT_MAX_BLOCKS_IN_FLIGHT_TOTAL,
            persist_mode: PersistMode::Flush,
            maybe_log_dir: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PeerSyncState {
    Connected,
    Stalled,
    Waiting,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PeerFailureReason {
    AddressResolution,
    Connect,
    Stall,
    RetryBackoff,
    InvalidData,
    InvalidMagic,
    Network,
    Storage,
}

impl fmt::Display for PeerFailureReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AddressResolution => write!(f, "address_resolution"),
            Self::Connect => write!(f, "connect"),
            Self::Stall => write!(f, "stall"),
            Self::RetryBackoff => write!(f, "retry_backoff"),
            Self::InvalidData => write!(f, "invalid_data"),
            Self::InvalidMagic => write!(f, "invalid_magic"),
            Self::Network => write!(f, "network"),
            Self::Storage => write!(f, "storage"),
        }
    }
}

impl PeerFailureReason {
    pub(crate) const fn operator_recovery_action(&self) -> &'static str {
        match self {
            Self::AddressResolution => "Check configured sync peers or DNS seeds, then retry sync.",
            Self::Connect => "Check peer connectivity and retry sync after backoff.",
            Self::Stall => "Retry sync after peer backoff or choose a different peer.",
            Self::RetryBackoff => "Wait for retry backoff to elapse or choose a different peer.",
            Self::InvalidData => {
                "Use a different peer or verify the peer is on the configured network before retrying."
            }
            Self::InvalidMagic => {
                "Check the configured Bitcoin network and peer list before retrying."
            }
            Self::Network => "Inspect network connectivity and retry sync.",
            Self::Storage => "Inspect durable store health before retrying sync.",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PeerCapabilitySummary {
    pub services_bits: u64,
    pub user_agent: String,
    pub start_height: i32,
    pub wtxidrelay: bool,
    pub prefers_headers: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PeerContribution {
    pub messages_processed: usize,
    pub headers_received: usize,
    pub blocks_received: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PeerSyncOutcome {
    pub peer: SyncPeerAddress,
    pub maybe_resolved_endpoint: Option<String>,
    pub network: SyncNetwork,
    pub state: PeerSyncState,
    pub attempts: u8,
    pub contribution: PeerContribution,
    pub maybe_last_activity_unix_seconds: Option<u64>,
    pub maybe_capabilities: Option<PeerCapabilitySummary>,
    pub maybe_failure_reason: Option<PeerFailureReason>,
    pub maybe_error: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SyncRunSummary {
    pub target_outbound_peers: usize,
    pub attempted_peers: usize,
    pub connected_peers: usize,
    pub failed_peers: usize,
    pub messages_processed: usize,
    pub headers_received: usize,
    pub blocks_received: usize,
    pub best_header_height: u64,
    pub downloaded_block_height: u64,
    pub best_block_height: u64,
    pub peer_outcomes: Vec<PeerSyncOutcome>,
    pub health_signals: Vec<HealthSignal>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyncRuntimeError {
    NoPeersConfigured,
    AddressResolution { peer: String, message: String },
    Io { peer: String, message: String },
    InvalidData { message: String },
    InvalidMagic { expected: [u8; 4], actual: [u8; 4] },
    Network { message: String },
    ResourceLimit { message: String },
    Storage(StorageError),
}

impl fmt::Display for SyncRuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoPeersConfigured => write!(f, "no sync peers configured"),
            Self::AddressResolution { peer, message } => {
                write!(f, "failed to resolve sync peer {peer}: {message}")
            }
            Self::Io { peer, message } => write!(f, "sync I/O failure for {peer}: {message}"),
            Self::InvalidData { message } => write!(f, "sync invalid data: {message}"),
            Self::InvalidMagic { expected, actual } => write!(
                f,
                "network magic mismatch: expected {expected:?}, got {actual:?}"
            ),
            Self::Network { message } => write!(f, "sync network failure: {message}"),
            Self::ResourceLimit { message } => write!(f, "sync resource limit: {message}"),
            Self::Storage(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for SyncRuntimeError {}

impl SyncRuntimeError {
    pub fn health_signal(&self) -> HealthSignal {
        match self {
            Self::AddressResolution { .. } => HealthSignal {
                level: HealthSignalLevel::Error,
                source: "network".to_string(),
                message: "sync address resolution failed: inspect peer configuration".to_string(),
            },
            Self::Io { .. } => HealthSignal {
                level: HealthSignalLevel::Error,
                source: "network".to_string(),
                message: "sync I/O failure: inspect peer connectivity".to_string(),
            },
            Self::InvalidData { .. } => HealthSignal {
                level: HealthSignalLevel::Error,
                source: "network".to_string(),
                message: "sync peer sent invalid data: inspect peer compatibility".to_string(),
            },
            Self::InvalidMagic { .. } => HealthSignal {
                level: HealthSignalLevel::Error,
                source: "network".to_string(),
                message: "sync network magic mismatch: inspect peer network".to_string(),
            },
            Self::Network { .. } => HealthSignal {
                level: HealthSignalLevel::Error,
                source: "network".to_string(),
                message: "sync network failure: inspect peer connectivity".to_string(),
            },
            Self::ResourceLimit { message } => HealthSignal {
                level: HealthSignalLevel::Warn,
                source: "sync".to_string(),
                message: format!("sync resource limit reached: {message}"),
            },
            Self::Storage(error) => HealthSignal {
                level: HealthSignalLevel::Error,
                source: "storage".to_string(),
                message: storage_health_message(error),
            },
            Self::NoPeersConfigured => HealthSignal {
                level: HealthSignalLevel::Warn,
                source: "sync".to_string(),
                message: "sync has no configured peers".to_string(),
            },
        }
    }
}

impl From<StorageError> for SyncRuntimeError {
    fn from(value: StorageError) -> Self {
        Self::Storage(value)
    }
}

impl From<ManagedNetworkError> for SyncRuntimeError {
    fn from(value: ManagedNetworkError) -> Self {
        match value {
            ManagedNetworkError::Network(error) => Self::from(error),
            ManagedNetworkError::Chainstate(error) if chainstate_error_is_peer_data(&error) => {
                Self::InvalidData {
                    message: error.to_string(),
                }
            }
            other => Self::Network {
                message: other.to_string(),
            },
        }
    }
}

fn chainstate_error_is_peer_data(error: &ChainstateError) -> bool {
    matches!(
        error,
        ChainstateError::MissingCoin { .. }
            | ChainstateError::InvalidGenesisParent { .. }
            | ChainstateError::InvalidTipExtension { .. }
            | ChainstateError::OutputOverwrite { .. }
            | ChainstateError::BlockValidation { .. }
            | ChainstateError::TransactionValidation { .. }
    )
}

impl From<NetworkError> for SyncRuntimeError {
    fn from(value: NetworkError) -> Self {
        match value {
            NetworkError::InvalidHeader { .. }
            | NetworkError::HeadersIncludeTransactions(_)
            | NetworkError::MissingHeaderAncestor(_) => Self::InvalidData {
                message: value.to_string(),
            },
            _ => Self::Network {
                message: value.to_string(),
            },
        }
    }
}

pub trait SyncPeerSession {
    fn send(
        &mut self,
        message: &WireNetworkMessage,
        magic: NetworkMagic,
    ) -> Result<(), SyncRuntimeError>;

    fn receive(
        &mut self,
        magic: NetworkMagic,
    ) -> Result<Option<WireNetworkMessage>, SyncRuntimeError>;
}

pub trait SyncTransport {
    type Session: SyncPeerSession;

    fn connect(
        &mut self,
        peer: &ResolvedSyncPeerAddress,
        config: &SyncRuntimeConfig,
    ) -> Result<Self::Session, SyncRuntimeError>;
}
