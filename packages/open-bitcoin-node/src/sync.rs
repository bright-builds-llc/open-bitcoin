// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! Real-network sync runtime shell.

use std::collections::{BTreeMap, BTreeSet};

mod progress;
mod resolver;
mod tcp;
#[cfg(test)]
mod tests;
mod types;
mod wallet_rescan;

use open_bitcoin_core::consensus::{ConsensusParams, ScriptVerifyFlags};
use open_bitcoin_mempool::PolicyConfig;
use open_bitcoin_network::{PeerId, WireNetworkMessage};

pub use resolver::{SyncPeerResolver, SystemSyncPeerResolver};
pub use tcp::{TcpPeerSession, TcpPeerTransport};
pub use types::{
    PeerCapabilitySummary, PeerContribution, PeerFailureReason, PeerSyncOutcome, PeerSyncState,
    ResolvedSyncPeerAddress, SyncNetwork, SyncPeerAddress, SyncPeerSession, SyncPeerSource,
    SyncRunSummary, SyncRuntimeConfig, SyncRuntimeError, SyncTransport,
};
pub use wallet_rescan::WalletRescanRuntime;

use crate::{
    ChainstateStore, FjallNodeStore, LogRetentionPolicy, ManagedPeerNetwork, MemoryChainstateStore,
    MetricRetentionPolicy, RuntimeMetadata,
    logging::{StructuredLogError, StructuredLogRecord, writer::append_structured_log_record},
};
use progress::{PeerFailure, PeerProgress};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PeerRetryState {
    consecutive_failures: u8,
    next_attempt_unix_seconds: i64,
}

pub struct DurableSyncRuntime {
    store: FjallNodeStore,
    network: ManagedPeerNetwork<MemoryChainstateStore>,
    config: SyncRuntimeConfig,
    verify_flags: ScriptVerifyFlags,
    consensus_params: ConsensusParams,
    next_peer_id: PeerId,
    peer_backoff: BTreeMap<String, PeerRetryState>,
}

impl DurableSyncRuntime {
    pub fn open(
        store: FjallNodeStore,
        config: SyncRuntimeConfig,
    ) -> Result<Self, SyncRuntimeError> {
        let mut memory_store = MemoryChainstateStore::default();
        if let Some(snapshot) = store.load_chainstate_snapshot()? {
            memory_store.save_snapshot(snapshot);
        }

        let local_config = progress::local_peer_config(&config);
        let mut network =
            ManagedPeerNetwork::new(memory_store, local_config, PolicyConfig::default());
        if let Some(header_store) = store.load_header_store()? {
            network.seed_header_store(header_store);
        }

        let consensus_params = config.network.consensus_params();
        Ok(Self {
            store,
            network,
            config,
            verify_flags: ScriptVerifyFlags::P2SH,
            consensus_params,
            next_peer_id: 1,
            peer_backoff: BTreeMap::new(),
        })
    }

    pub fn config(&self) -> &SyncRuntimeConfig {
        &self.config
    }

    pub fn store(&self) -> &FjallNodeStore {
        &self.store
    }

    pub fn snapshot_summary(&self) -> SyncRunSummary {
        let (best_header_height, best_block_height) = self.best_heights();
        SyncRunSummary::empty(best_header_height, best_block_height)
    }

    pub fn sync_once<T: SyncTransport>(
        &mut self,
        transport: &mut T,
        timestamp: i64,
    ) -> Result<SyncRunSummary, SyncRuntimeError> {
        let mut resolver = SystemSyncPeerResolver;
        self.sync_once_with_resolver(transport, &mut resolver, timestamp)
    }

    pub fn sync_once_with_resolver<T: SyncTransport, R: SyncPeerResolver>(
        &mut self,
        transport: &mut T,
        resolver: &mut R,
        timestamp: i64,
    ) -> Result<SyncRunSummary, SyncRuntimeError> {
        let peers = self.config.candidate_peers();
        if peers.is_empty() {
            let error = SyncRuntimeError::NoPeersConfigured;
            self.write_runtime_error_log(&error, timestamp);
            return Err(error);
        }

        let (best_header_height, best_block_height) = self.best_heights();
        let mut summary = SyncRunSummary::empty(best_header_height, best_block_height);
        let resolved_peers = self.resolve_candidates(peers, resolver, &mut summary);
        let mut completed_outbound_slots = 0_usize;
        for peer in resolved_peers {
            if completed_outbound_slots >= self.config.target_outbound_peers {
                break;
            }
            if !self.peer_ready(&peer, timestamp) {
                continue;
            }
            summary.attempted_peers += 1;
            let peer_id = self.allocate_peer_id();
            let outcome = self.sync_peer_with_retries(transport, &peer, peer_id, timestamp);
            if matches!(
                outcome,
                Ok(PeerProgress {
                    state: PeerSyncState::Connected,
                    ..
                })
            ) {
                completed_outbound_slots += 1;
            }
            self.record_outcome(&mut summary, outcome, timestamp);
        }
        if let Err(error) = self.persist_metrics(&summary, timestamp) {
            self.write_runtime_error_log(&error, timestamp);
            return Err(error);
        }
        self.write_summary_logs(&mut summary, timestamp);

        Ok(summary)
    }

    pub fn sync_until_idle<T: SyncTransport>(
        &mut self,
        transport: &mut T,
        timestamp: i64,
    ) -> Result<SyncRunSummary, SyncRuntimeError> {
        let mut resolver = SystemSyncPeerResolver;
        self.sync_until_idle_with_resolver(transport, &mut resolver, timestamp)
    }

    pub fn sync_until_idle_with_resolver<T: SyncTransport, R: SyncPeerResolver>(
        &mut self,
        transport: &mut T,
        resolver: &mut R,
        timestamp: i64,
    ) -> Result<SyncRunSummary, SyncRuntimeError> {
        let mut current_timestamp = timestamp;
        let mut last_summary =
            self.sync_once_with_resolver(transport, resolver, current_timestamp)?;
        let mut previous_progress = progress::sync_progress_marker(&last_summary);
        for _ in 1..self.config.max_rounds {
            current_timestamp = current_timestamp
                .saturating_add(i64::try_from(self.config.retry_backoff_ms).unwrap_or(i64::MAX));
            let current_summary =
                self.sync_once_with_resolver(transport, resolver, current_timestamp)?;
            let current_progress = progress::sync_progress_marker(&current_summary);
            let is_idle = current_progress == previous_progress;
            last_summary = current_summary;
            if is_idle {
                break;
            }
            previous_progress = current_progress;
        }

        Ok(last_summary)
    }

    fn sync_peer_with_retries<T: SyncTransport>(
        &mut self,
        transport: &mut T,
        peer: &ResolvedSyncPeerAddress,
        peer_id: PeerId,
        timestamp: i64,
    ) -> Result<PeerProgress, Box<PeerFailure>> {
        let mut attempts = 0_u8;
        let max_attempts = self.config.max_peer_retries.saturating_add(1);
        loop {
            attempts = attempts.saturating_add(1);
            match transport.connect(peer, &self.config) {
                Ok(session) => {
                    return self
                        .sync_connected_peer(session, peer, peer_id, attempts, timestamp)
                        .map_err(|error| {
                            Box::new(PeerFailure {
                                peer: peer.clone(),
                                reason: peer_failure_reason_for_error(&error),
                                error,
                                attempts,
                            })
                        });
                }
                Err(error) if attempts < max_attempts => {
                    let _ = error;
                }
                Err(error) => {
                    return Err(Box::new(PeerFailure {
                        peer: peer.clone(),
                        reason: peer_failure_reason_for_error(&error),
                        error,
                        attempts,
                    }));
                }
            }
        }
    }

    fn sync_connected_peer<S: SyncPeerSession>(
        &mut self,
        mut session: S,
        peer: &ResolvedSyncPeerAddress,
        peer_id: PeerId,
        attempts: u8,
        timestamp: i64,
    ) -> Result<PeerProgress, SyncRuntimeError> {
        let result = (|| -> Result<PeerProgress, SyncRuntimeError> {
            let outbound = self.network.connect_outbound_peer(peer_id, timestamp)?;
            self.send_all(&mut session, &outbound)?;

            let mut progress = PeerProgress::new(peer.clone(), self.config.network, attempts);
            for _ in 0..self.config.max_messages_per_peer {
                let Some(message) = session.receive(self.config.network.magic())? else {
                    progress.state = PeerSyncState::Stalled;
                    progress.maybe_failure_reason = Some(PeerFailureReason::Stall);
                    progress.maybe_capabilities = self.peer_capabilities(peer_id);
                    return Ok(progress);
                };
                progress.messages_processed += 1;
                progress.maybe_last_activity_unix_seconds =
                    Some(u64::try_from(timestamp).unwrap_or(0));
                progress.record_message(&message);

                let maybe_block = match &message {
                    WireNetworkMessage::Block(block) => Some(block.clone()),
                    _ => None,
                };
                let outbound = self.network.receive_sync_message(
                    peer_id,
                    message,
                    timestamp,
                    self.verify_flags,
                    self.consensus_params,
                )?;
                if let Some(block) = maybe_block {
                    self.store.save_block(&block, self.config.persist_mode)?;
                }
                self.persist_progress()?;
                self.send_all(&mut session, &outbound)?;
            }
            progress.state = PeerSyncState::Connected;
            progress.maybe_capabilities = self.peer_capabilities(peer_id);

            Ok(progress)
        })();
        let disconnect_result = self.network.disconnect_peer(peer_id);
        match (result, disconnect_result) {
            (Ok(progress), Ok(())) => Ok(progress),
            (Ok(_), Err(error)) => Err(SyncRuntimeError::from(error)),
            (Err(error), _) => Err(error),
        }
    }

    fn record_outcome(
        &mut self,
        summary: &mut SyncRunSummary,
        outcome: Result<PeerProgress, Box<PeerFailure>>,
        timestamp: i64,
    ) {
        match outcome {
            Ok(progress) => {
                self.clear_backoff(&progress.peer);
                summary.connected_peers += usize::from(progress.state != PeerSyncState::Failed);
                summary.messages_processed += progress.messages_processed;
                summary.headers_received += progress.headers_received;
                summary.blocks_received += progress.blocks_received;
                let (best_header_height, best_block_height) = self.best_heights();
                summary.best_header_height = best_header_height;
                summary.best_block_height = best_block_height;
                if progress.state == PeerSyncState::Stalled {
                    summary.health_signals.push(progress::stalled_peer_signal());
                    self.mark_backoff(&progress.peer, timestamp);
                }
                summary.peer_outcomes.push(progress.into_outcome(None));
            }
            Err(failure) => {
                self.mark_backoff(&failure.peer, timestamp);
                summary.failed_peers += 1;
                let signal = failure.error.health_signal();
                let message = signal.message.clone();
                summary.health_signals.push(signal);
                summary.peer_outcomes.push(PeerSyncOutcome {
                    peer: failure.peer.peer,
                    maybe_resolved_endpoint: Some(failure.peer.endpoint.to_string()),
                    network: self.config.network,
                    state: PeerSyncState::Failed,
                    attempts: failure.attempts,
                    contribution: PeerContribution {
                        messages_processed: 0,
                        headers_received: 0,
                        blocks_received: 0,
                    },
                    maybe_last_activity_unix_seconds: None,
                    maybe_capabilities: None,
                    maybe_failure_reason: Some(failure.reason),
                    maybe_error: Some(message),
                });
            }
        }
    }

    fn send_all<S: SyncPeerSession>(
        &self,
        session: &mut S,
        messages: &[WireNetworkMessage],
    ) -> Result<(), SyncRuntimeError> {
        for message in messages {
            session.send(message, self.config.network.magic())?;
        }
        Ok(())
    }

    fn persist_progress(&self) -> Result<(), SyncRuntimeError> {
        self.store
            .save_header_entries(&self.network.header_entries(), self.config.persist_mode)?;
        self.store.save_chainstate_snapshot(
            &self.network.chainstate_snapshot(),
            self.config.persist_mode,
        )?;
        self.store.save_runtime_metadata(
            &RuntimeMetadata {
                last_clean_shutdown: false,
                ..RuntimeMetadata::default()
            },
            self.config.persist_mode,
        )?;

        Ok(())
    }

    fn persist_metrics(
        &self,
        summary: &SyncRunSummary,
        timestamp: i64,
    ) -> Result<(), SyncRuntimeError> {
        let timestamp = u64::try_from(timestamp).unwrap_or(0);
        self.store.append_metric_samples(
            &summary.metric_samples(timestamp),
            MetricRetentionPolicy::default(),
            timestamp,
            self.config.persist_mode,
        )?;

        Ok(())
    }

    fn write_summary_logs(&self, summary: &mut SyncRunSummary, timestamp: i64) {
        let timestamp = u64::try_from(timestamp).unwrap_or(0);
        for record in summary.structured_log_records(timestamp) {
            if let Err(error) = self.append_structured_record(&record) {
                summary
                    .health_signals
                    .push(progress::log_write_failed_signal(&error));
                break;
            }
        }
    }

    fn write_runtime_error_log(&self, error: &SyncRuntimeError, timestamp: i64) {
        let signal = error.health_signal();
        let record = StructuredLogRecord {
            level: progress::structured_log_level(signal.level),
            source: signal.source,
            message: signal.message,
            timestamp_unix_seconds: u64::try_from(timestamp).unwrap_or(0),
        };
        let _ = self.append_structured_record(&record);
    }

    fn append_structured_record(
        &self,
        record: &StructuredLogRecord,
    ) -> Result<(), StructuredLogError> {
        let Some(log_dir) = &self.config.maybe_log_dir else {
            return Ok(());
        };

        append_structured_log_record(log_dir, record, LogRetentionPolicy::default())?;
        Ok(())
    }

    fn best_heights(&self) -> (u64, u64) {
        let best_header_height = self
            .network
            .peer_manager()
            .header_store()
            .best_tip()
            .map_or(0, |entry| u64::from(entry.height));
        let best_block_height = self
            .network
            .maybe_chain_tip()
            .map_or(0, |tip| u64::from(tip.height));

        (best_header_height, best_block_height)
    }

    fn allocate_peer_id(&mut self) -> PeerId {
        let peer_id = self.next_peer_id;
        self.next_peer_id = self.next_peer_id.saturating_add(1);
        peer_id
    }

    fn resolve_candidates<R: SyncPeerResolver>(
        &self,
        peers: Vec<SyncPeerAddress>,
        resolver: &mut R,
        summary: &mut SyncRunSummary,
    ) -> Vec<ResolvedSyncPeerAddress> {
        let mut resolved = Vec::new();
        let mut seen = BTreeSet::new();
        for peer in peers {
            match resolver.resolve(&peer, &self.config) {
                Ok(endpoints) => {
                    for endpoint in endpoints {
                        if seen.insert(endpoint.endpoint) {
                            resolved.push(endpoint);
                        }
                    }
                }
                Err(error) => {
                    summary.attempted_peers += 1;
                    summary.failed_peers += 1;
                    let signal = error.health_signal();
                    let message = signal.message.clone();
                    summary.health_signals.push(signal);
                    summary.peer_outcomes.push(PeerSyncOutcome {
                        peer,
                        maybe_resolved_endpoint: None,
                        network: self.config.network,
                        state: PeerSyncState::Failed,
                        attempts: 1,
                        contribution: PeerContribution {
                            messages_processed: 0,
                            headers_received: 0,
                            blocks_received: 0,
                        },
                        maybe_last_activity_unix_seconds: None,
                        maybe_capabilities: None,
                        maybe_failure_reason: Some(PeerFailureReason::AddressResolution),
                        maybe_error: Some(message),
                    });
                }
            }
        }
        resolved
    }

    fn peer_capabilities(&self, peer_id: PeerId) -> Option<PeerCapabilitySummary> {
        let peer = self.network.peer_manager().peer_state(peer_id)?;
        Some(PeerCapabilitySummary {
            services_bits: peer.remote_services_bits,
            user_agent: peer.remote_user_agent.clone(),
            start_height: peer.remote_start_height,
            wtxidrelay: peer.remote_wtxidrelay,
            prefers_headers: peer.remote_prefers_headers,
        })
    }

    fn peer_ready(&self, peer: &ResolvedSyncPeerAddress, timestamp: i64) -> bool {
        let key = peer.endpoint.to_string();
        self.peer_backoff
            .get(&key)
            .is_none_or(|state| state.next_attempt_unix_seconds <= timestamp)
    }

    fn mark_backoff(&mut self, peer: &ResolvedSyncPeerAddress, timestamp: i64) {
        let key = peer.endpoint.to_string();
        let mut state = self
            .peer_backoff
            .get(&key)
            .copied()
            .unwrap_or(PeerRetryState {
                consecutive_failures: 0,
                next_attempt_unix_seconds: timestamp,
            });
        state.consecutive_failures = state.consecutive_failures.saturating_add(1);
        let multiplier = i64::from(state.consecutive_failures);
        let backoff = i64::try_from(self.config.retry_backoff_ms).unwrap_or(i64::MAX);
        state.next_attempt_unix_seconds =
            timestamp.saturating_add(backoff.saturating_mul(multiplier));
        self.peer_backoff.insert(key, state);
    }

    fn clear_backoff(&mut self, peer: &ResolvedSyncPeerAddress) {
        self.peer_backoff.remove(&peer.endpoint.to_string());
    }
}

fn peer_failure_reason_for_error(error: &SyncRuntimeError) -> PeerFailureReason {
    match error {
        SyncRuntimeError::AddressResolution { .. } => PeerFailureReason::AddressResolution,
        SyncRuntimeError::InvalidData { .. } => PeerFailureReason::InvalidData,
        SyncRuntimeError::InvalidMagic { .. } => PeerFailureReason::InvalidMagic,
        SyncRuntimeError::Storage(_) => PeerFailureReason::Storage,
        SyncRuntimeError::Io { .. } => PeerFailureReason::Connect,
        SyncRuntimeError::Network { .. } | SyncRuntimeError::NoPeersConfigured => {
            PeerFailureReason::Network
        }
    }
}
