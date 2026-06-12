// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp

mod error;
mod projection;
pub(super) mod recovery;
mod summary;
pub use error::SyncRuntimeError;

use std::{fmt, net::SocketAddr, path::PathBuf};

use open_bitcoin_core::{consensus::ConsensusParams, primitives::NetworkMagic};
use open_bitcoin_network::WireNetworkMessage;

use crate::{
    PersistMode,
    status::{HealthSignal, HealthSignalLevel, SyncReorgEvidence},
};

const DEFAULT_CONNECT_TIMEOUT_MS: u64 = 5_000;
const DEFAULT_READ_TIMEOUT_MS: u64 = 5_000;
const DEFAULT_MAX_MESSAGES_PER_PEER: usize = 64;
const DEFAULT_MAX_SYNC_ROUNDS: usize = 8;
const DEFAULT_MAX_PEER_RETRIES: u8 = 1;
const DEFAULT_TARGET_OUTBOUND_PEERS: usize = 4;
const DEFAULT_RETRY_BACKOFF_MS: u64 = 1_000;
const DEFAULT_MAX_BLOCKS_IN_FLIGHT_PER_PEER: usize = 16;
const DEFAULT_MAX_BLOCKS_IN_FLIGHT_TOTAL: usize = 64;
const DEFAULT_TIP_FRESHNESS_THRESHOLD_SECONDS: u64 = 1_200;
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
    pub maybe_target_header_height: Option<u64>,
    pub connect_timeout_ms: u64,
    pub read_timeout_ms: u64,
    pub retry_backoff_ms: u64,
    pub max_messages_per_peer: usize,
    pub max_rounds: usize,
    pub max_peer_retries: u8,
    pub max_blocks_in_flight_per_peer: usize,
    pub max_blocks_in_flight_total: usize,
    pub tip_freshness_threshold_seconds: u64,
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
            maybe_target_header_height: None,
            connect_timeout_ms: DEFAULT_CONNECT_TIMEOUT_MS,
            read_timeout_ms: DEFAULT_READ_TIMEOUT_MS,
            retry_backoff_ms: DEFAULT_RETRY_BACKOFF_MS,
            max_messages_per_peer: DEFAULT_MAX_MESSAGES_PER_PEER,
            max_rounds: DEFAULT_MAX_SYNC_ROUNDS,
            max_peer_retries: DEFAULT_MAX_PEER_RETRIES,
            max_blocks_in_flight_per_peer: DEFAULT_MAX_BLOCKS_IN_FLIGHT_PER_PEER,
            max_blocks_in_flight_total: DEFAULT_MAX_BLOCKS_IN_FLIGHT_TOTAL,
            tip_freshness_threshold_seconds: DEFAULT_TIP_FRESHNESS_THRESHOLD_SECONDS,
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
    Compatibility,
    Connect,
    Stall,
    RetryBackoff,
    InvalidData,
    InvalidMagic,
    BlockNotFound,
    MalformedBlock,
    InvalidBlock,
    DuplicateBlock,
    DisconnectedBlock,
    NonExtendingBlock,
    Network,
    ResourceLimit,
    Storage,
}

impl fmt::Display for PeerFailureReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AddressResolution => write!(f, "address_resolution"),
            Self::Compatibility => write!(f, "compatibility"),
            Self::Connect => write!(f, "connect"),
            Self::Stall => write!(f, "stall"),
            Self::RetryBackoff => write!(f, "retry_backoff"),
            Self::InvalidData => write!(f, "invalid_data"),
            Self::InvalidMagic => write!(f, "invalid_magic"),
            Self::BlockNotFound => write!(f, "block_notfound"),
            Self::MalformedBlock => write!(f, "malformed_block"),
            Self::InvalidBlock => write!(f, "invalid_block"),
            Self::DuplicateBlock => write!(f, "duplicate_block"),
            Self::DisconnectedBlock => write!(f, "disconnected_block"),
            Self::NonExtendingBlock => write!(f, "non_extending_block"),
            Self::Network => write!(f, "network"),
            Self::ResourceLimit => write!(f, "resource_limit"),
            Self::Storage => write!(f, "storage"),
        }
    }
}

impl PeerFailureReason {
    pub(crate) const fn operator_recovery_action(&self) -> &'static str {
        match self {
            Self::AddressResolution => "Check configured sync peers or DNS seeds, then retry sync.",
            Self::Compatibility => {
                "Use a different peer or inspect peer protocol compatibility before retrying."
            }
            Self::Connect => "Check peer connectivity and retry sync after backoff.",
            Self::Stall => "Retry sync after peer backoff or choose a different peer.",
            Self::RetryBackoff => "Wait for retry backoff to elapse or choose a different peer.",
            Self::InvalidData => {
                "Use a different peer or verify the peer is on the configured network before retrying."
            }
            Self::InvalidMagic => {
                "Check the configured Bitcoin network and peer list before retrying."
            }
            Self::BlockNotFound => {
                "Retry the missing block from another peer or wait for peer backoff."
            }
            Self::MalformedBlock => {
                "Use another peer and inspect transport framing if malformed block payloads repeat."
            }
            Self::InvalidBlock => {
                "Use a different peer and verify the peer is serving the configured chain."
            }
            Self::DuplicateBlock => {
                "No operator action is required unless the same peer repeatedly sends duplicate blocks."
            }
            Self::DisconnectedBlock => {
                "Request headers and ancestor blocks before retrying this block response."
            }
            Self::NonExtendingBlock => {
                "Continue sync with peers advertising the validated best chain."
            }
            Self::Network => "Inspect network connectivity and retry sync.",
            Self::ResourceLimit => "Increase block in-flight limits or reduce sync pressure.",
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
    pub maybe_tip_height: Option<u64>,
    pub maybe_tip_hash: Option<String>,
    pub maybe_tip_work: Option<String>,
    pub maybe_last_activity_unix_seconds: Option<u64>,
    pub maybe_capabilities: Option<PeerCapabilitySummary>,
    pub maybe_failure_reason: Option<PeerFailureReason>,
    pub maybe_error: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SyncReconcileProgress {
    NoChange,
    ExtendedActiveChain {
        connected_count: u64,
    },
    BranchCompetitionAwaitingBodies {
        missing_count: u64,
        first_missing_height: u64,
        first_missing_hash: String,
    },
    SideBranchPreserved,
    ReorgPersisted(SyncReorgEvidence),
}

impl SyncReconcileProgress {
    pub(crate) const fn should_persist_progress(&self) -> bool {
        matches!(
            self,
            Self::ExtendedActiveChain { connected_count } if *connected_count > 0
        ) || matches!(self, Self::ReorgPersisted(_))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SyncRunSummary {
    pub target_outbound_peers: usize,
    pub maybe_target_header_height: Option<u64>,
    pub attempted_peers: usize,
    pub connected_peers: usize,
    pub failed_peers: usize,
    pub messages_processed: usize,
    pub headers_received: usize,
    pub blocks_received: usize,
    pub best_header_height: u64,
    pub downloaded_block_height: u64,
    pub best_block_height: u64,
    pub maybe_downloaded_block_hash: Option<String>,
    pub maybe_connected_block_hash: Option<String>,
    pub maybe_validated_active_chain_work: Option<String>,
    pub peer_outcomes: Vec<PeerSyncOutcome>,
    pub health_signals: Vec<HealthSignal>,
    pub maybe_stop_reason: Option<SyncStopReason>,
    pub(crate) maybe_reconcile_progress: Option<SyncReconcileProgress>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyncStopReason {
    TargetHeaderReached {
        target_header_height: u64,
        best_header_height: u64,
    },
    CurrentAtBestKnownTip {
        best_header_height: u64,
        best_block_height: u64,
    },
    NoProgress {
        rounds_completed: usize,
    },
    MaxRoundsReached {
        max_rounds: usize,
    },
    OperatorPaused,
    ShutdownRequested,
}

impl SyncStopReason {
    pub const fn label(self) -> &'static str {
        match self {
            Self::TargetHeaderReached { .. } => "target_header_reached",
            Self::CurrentAtBestKnownTip { .. } => "current_at_best_known_tip",
            Self::NoProgress { .. } => "no_progress",
            Self::MaxRoundsReached { .. } => "max_rounds_reached",
            Self::OperatorPaused => "operator_paused",
            Self::ShutdownRequested => "shutdown_requested",
        }
    }

    pub fn message(self) -> String {
        match self {
            Self::TargetHeaderReached {
                target_header_height,
                best_header_height,
            } => format!(
                "sync header target reached: target_header_height={target_header_height} best_header_height={best_header_height}"
            ),
            Self::CurrentAtBestKnownTip {
                best_header_height,
                best_block_height,
            } => format!(
                "sync current at best-known validated tip: best_header_height={best_header_height} best_block_height={best_block_height}"
            ),
            Self::NoProgress { rounds_completed } => {
                format!(
                    "sync stopped with no new header or block progress after {rounds_completed} rounds"
                )
            }
            Self::MaxRoundsReached { max_rounds } => {
                format!("sync stopped after reaching max_rounds={max_rounds}")
            }
            Self::OperatorPaused => "operator paused unattended sync loop".to_string(),
            Self::ShutdownRequested => {
                "daemon shutdown requested for unattended sync loop".to_string()
            }
        }
    }

    pub fn health_signal(self) -> HealthSignal {
        HealthSignal {
            level: match self {
                Self::TargetHeaderReached { .. } | Self::CurrentAtBestKnownTip { .. } => {
                    HealthSignalLevel::Info
                }
                Self::NoProgress { .. }
                | Self::MaxRoundsReached { .. }
                | Self::OperatorPaused
                | Self::ShutdownRequested => HealthSignalLevel::Warn,
            },
            source: "sync".to_string(),
            message: self.message(),
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
