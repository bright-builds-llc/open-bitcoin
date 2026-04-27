use open_bitcoin_core::primitives::NetworkAddress;
use open_bitcoin_network::{LocalPeerConfig, ServiceFlags, WireNetworkMessage};

use crate::{
    logging::{StructuredLogError, StructuredLogLevel},
    status::{HealthSignal, HealthSignalLevel},
};

use super::{PeerSyncOutcome, PeerSyncState, SyncPeerAddress, SyncRunSummary, SyncRuntimeConfig};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct PeerProgress {
    pub(super) peer: SyncPeerAddress,
    pub(super) state: PeerSyncState,
    pub(super) attempts: u8,
    pub(super) messages_processed: usize,
    pub(super) headers_received: usize,
    pub(super) blocks_received: usize,
}

#[derive(Debug)]
pub(super) struct PeerFailure {
    pub(super) error: super::SyncRuntimeError,
    pub(super) attempts: u8,
}

impl PeerProgress {
    pub(super) fn new(peer: SyncPeerAddress, attempts: u8) -> Self {
        Self {
            peer,
            state: PeerSyncState::Connected,
            attempts,
            messages_processed: 0,
            headers_received: 0,
            blocks_received: 0,
        }
    }

    pub(super) fn record_message(&mut self, message: &WireNetworkMessage) {
        match message {
            WireNetworkMessage::Headers(headers) => {
                self.headers_received += headers.headers.len();
            }
            WireNetworkMessage::Block(_) => {
                self.blocks_received += 1;
            }
            _ => {}
        }
    }

    pub(super) fn into_outcome(self, maybe_error: Option<String>) -> PeerSyncOutcome {
        PeerSyncOutcome {
            peer: self.peer,
            state: self.state,
            attempts: self.attempts,
            maybe_error,
        }
    }
}

pub(super) fn structured_log_level(level: HealthSignalLevel) -> StructuredLogLevel {
    match level {
        HealthSignalLevel::Info => StructuredLogLevel::Info,
        HealthSignalLevel::Warn => StructuredLogLevel::Warn,
        HealthSignalLevel::Error => StructuredLogLevel::Error,
    }
}

pub(super) fn sync_progress_marker(summary: &SyncRunSummary) -> (u64, u64) {
    (summary.best_header_height, summary.best_block_height)
}

pub(super) fn log_write_failed_signal(error: &StructuredLogError) -> HealthSignal {
    let message = match error {
        StructuredLogError::Io { action, source, .. } => {
            format!("structured log write failed: {action}: {source}")
        }
        StructuredLogError::Json { source } => {
            format!("structured log write failed: JSON encoding: {source}")
        }
    };

    HealthSignal {
        level: HealthSignalLevel::Warn,
        source: "logging".to_string(),
        message,
    }
}

pub(super) fn stalled_peer_signal() -> HealthSignal {
    HealthSignal {
        level: HealthSignalLevel::Warn,
        source: "sync".to_string(),
        message: "peer stalled before sending more sync messages".to_string(),
    }
}

pub(super) fn local_peer_config(config: &SyncRuntimeConfig) -> LocalPeerConfig {
    LocalPeerConfig {
        magic: config.network.magic(),
        services: ServiceFlags::NETWORK | ServiceFlags::WITNESS,
        address: NetworkAddress {
            services: 0,
            address_bytes: [0_u8; 16],
            port: 0,
        },
        nonce: 0,
        relay: true,
        user_agent: format!("/open-bitcoin:{}/", env!("CARGO_PKG_VERSION")),
    }
}
