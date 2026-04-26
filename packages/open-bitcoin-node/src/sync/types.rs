// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp

use std::fmt;

use open_bitcoin_core::{consensus::ConsensusParams, primitives::NetworkMagic};
use open_bitcoin_network::{NetworkError, WireNetworkMessage};

use crate::{
    FieldAvailability, ManagedNetworkError, PeerStatus, PersistMode, StorageError, SyncStatus,
    status::{HealthSignal, PeerCounts, SyncProgress},
};

const DEFAULT_CONNECT_TIMEOUT_MS: u64 = 5_000;
const DEFAULT_READ_TIMEOUT_MS: u64 = 5_000;
const DEFAULT_MAX_MESSAGES_PER_PEER: usize = 64;
const DEFAULT_MAX_SYNC_ROUNDS: usize = 8;
const DEFAULT_MAX_PEER_RETRIES: u8 = 1;

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
pub struct SyncRuntimeConfig {
    pub network: SyncNetwork,
    pub manual_peers: Vec<SyncPeerAddress>,
    pub dns_seeds: Vec<String>,
    pub connect_timeout_ms: u64,
    pub read_timeout_ms: u64,
    pub max_messages_per_peer: usize,
    pub max_rounds: usize,
    pub max_peer_retries: u8,
    pub persist_mode: PersistMode,
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
            connect_timeout_ms: DEFAULT_CONNECT_TIMEOUT_MS,
            read_timeout_ms: DEFAULT_READ_TIMEOUT_MS,
            max_messages_per_peer: DEFAULT_MAX_MESSAGES_PER_PEER,
            max_rounds: DEFAULT_MAX_SYNC_ROUNDS,
            max_peer_retries: DEFAULT_MAX_PEER_RETRIES,
            persist_mode: PersistMode::Flush,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PeerSyncState {
    Connected,
    Stalled,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PeerSyncOutcome {
    pub peer: SyncPeerAddress,
    pub state: PeerSyncState,
    pub attempts: u8,
    pub maybe_error: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SyncRunSummary {
    pub attempted_peers: usize,
    pub connected_peers: usize,
    pub failed_peers: usize,
    pub messages_processed: usize,
    pub headers_received: usize,
    pub blocks_received: usize,
    pub best_header_height: u64,
    pub best_block_height: u64,
    pub peer_outcomes: Vec<PeerSyncOutcome>,
    pub health_signals: Vec<HealthSignal>,
}

impl SyncRunSummary {
    pub(crate) fn empty(best_header_height: u64, best_block_height: u64) -> Self {
        Self {
            attempted_peers: 0,
            connected_peers: 0,
            failed_peers: 0,
            messages_processed: 0,
            headers_received: 0,
            blocks_received: 0,
            best_header_height,
            best_block_height,
            peer_outcomes: Vec::new(),
            health_signals: Vec::new(),
        }
    }

    pub fn sync_status(&self, network: SyncNetwork) -> SyncStatus {
        SyncStatus {
            network: FieldAvailability::available(network.as_str().to_string()),
            chain_tip: FieldAvailability::unavailable(
                "chain tip hash is unavailable from sync summary alone",
            ),
            sync_progress: FieldAvailability::available(SyncProgress {
                header_height: self.best_header_height,
                block_height: self.best_block_height,
                progress_ratio: progress_ratio(self.best_block_height, self.best_header_height),
            }),
        }
    }

    pub fn peer_status(&self) -> PeerStatus {
        PeerStatus {
            peer_counts: FieldAvailability::available(PeerCounts {
                inbound: 0,
                outbound: self.connected_peers as u32,
            }),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyncRuntimeError {
    NoPeersConfigured,
    AddressResolution { peer: String, message: String },
    Io { peer: String, message: String },
    InvalidMagic { expected: [u8; 4], actual: [u8; 4] },
    Network { message: String },
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
            Self::InvalidMagic { expected, actual } => write!(
                f,
                "network magic mismatch: expected {expected:?}, got {actual:?}"
            ),
            Self::Network { message } => write!(f, "sync network failure: {message}"),
            Self::Storage(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for SyncRuntimeError {}

impl From<StorageError> for SyncRuntimeError {
    fn from(value: StorageError) -> Self {
        Self::Storage(value)
    }
}

impl From<ManagedNetworkError> for SyncRuntimeError {
    fn from(value: ManagedNetworkError) -> Self {
        Self::Network {
            message: value.to_string(),
        }
    }
}

impl From<NetworkError> for SyncRuntimeError {
    fn from(value: NetworkError) -> Self {
        Self::Network {
            message: value.to_string(),
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
        peer: &SyncPeerAddress,
        config: &SyncRuntimeConfig,
    ) -> Result<Self::Session, SyncRuntimeError>;
}

fn progress_ratio(block_height: u64, header_height: u64) -> f64 {
    if header_height == 0 {
        return 1.0;
    }

    (block_height as f64 / header_height as f64).min(1.0)
}
