// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use open_bitcoin_core::primitives::NetworkAddress;
use open_bitcoin_network::{LocalPeerConfig, ServiceFlags};

use crate::{
    logging::{StructuredLogError, StructuredLogLevel},
    status::{HealthSignal, HealthSignalLevel},
};

use super::{
    PeerCapabilitySummary, PeerContribution, PeerFailureReason, PeerSyncOutcome, PeerSyncState,
    ResolvedSyncPeerAddress, SyncNetwork, SyncRunSummary, SyncRuntimeConfig,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct PeerProgress {
    pub(super) peer: ResolvedSyncPeerAddress,
    pub(super) network: SyncNetwork,
    pub(super) state: PeerSyncState,
    pub(super) attempts: u8,
    pub(super) messages_processed: usize,
    pub(super) headers_received: usize,
    pub(super) blocks_received: usize,
    pub(super) maybe_last_activity_unix_seconds: Option<u64>,
    pub(super) maybe_capabilities: Option<PeerCapabilitySummary>,
    pub(super) maybe_failure_reason: Option<PeerFailureReason>,
}

#[derive(Debug)]
pub(super) struct PeerFailure {
    pub(super) peer: ResolvedSyncPeerAddress,
    pub(super) error: super::SyncRuntimeError,
    pub(super) attempts: u8,
    pub(super) reason: PeerFailureReason,
    pub(super) maybe_progress: Option<PeerProgress>,
}

impl PeerProgress {
    pub(super) fn new(peer: ResolvedSyncPeerAddress, network: SyncNetwork, attempts: u8) -> Self {
        Self {
            peer,
            network,
            state: PeerSyncState::Connected,
            attempts,
            messages_processed: 0,
            headers_received: 0,
            blocks_received: 0,
            maybe_last_activity_unix_seconds: None,
            maybe_capabilities: None,
            maybe_failure_reason: None,
        }
    }

    pub(super) fn record_activity(&mut self, timestamp: i64) {
        self.messages_processed += 1;
        self.maybe_last_activity_unix_seconds = Some(u64::try_from(timestamp).unwrap_or(0));
    }

    pub(super) fn record_validated_headers(&mut self, count: usize) {
        self.headers_received += count;
    }

    pub(super) fn record_accepted_block(&mut self) {
        self.blocks_received += 1;
    }

    pub(super) fn into_outcome(self, maybe_error: Option<String>) -> PeerSyncOutcome {
        PeerSyncOutcome {
            peer: self.peer.peer,
            maybe_resolved_endpoint: Some(self.peer.endpoint.to_string()),
            network: self.network,
            state: self.state,
            attempts: self.attempts,
            contribution: PeerContribution {
                messages_processed: self.messages_processed,
                headers_received: self.headers_received,
                blocks_received: self.blocks_received,
            },
            maybe_last_activity_unix_seconds: self.maybe_last_activity_unix_seconds,
            maybe_capabilities: self.maybe_capabilities,
            maybe_failure_reason: self.maybe_failure_reason,
            maybe_error,
        }
    }

    pub(super) fn into_failed_outcome(
        mut self,
        reason: PeerFailureReason,
        maybe_error: Option<String>,
    ) -> PeerSyncOutcome {
        self.state = PeerSyncState::Failed;
        self.maybe_failure_reason = Some(reason);
        self.into_outcome(maybe_error)
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

pub(super) fn waiting_peer_signal() -> HealthSignal {
    HealthSignal {
        level: HealthSignalLevel::Warn,
        source: "sync".to_string(),
        message: "peer waiting for retry backoff before next attempt".to_string(),
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
