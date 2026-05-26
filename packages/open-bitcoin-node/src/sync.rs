// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! Real-network sync runtime shell.

use std::collections::{BTreeMap, BTreeSet};

mod block_reconcile;
mod progress;
mod resolver;
mod runtime_state;
mod tcp;
#[cfg(test)]
mod tests;
mod types;
mod wallet_rescan;

use open_bitcoin_core::{
    consensus::{ConsensusParams, ScriptVerifyFlags, block_hash},
    primitives::BlockHash,
};
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
    ChainstateStore, FjallNodeStore, ManagedPeerNetwork, MemoryChainstateStore, SyncLifecycleState,
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
    inflight_blocks: BTreeSet<BlockHash>,
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
        let mut network = ManagedPeerNetwork::with_sync_limits(
            memory_store,
            local_config,
            PolicyConfig::default(),
            config.max_blocks_in_flight_per_peer,
        );
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
            inflight_blocks: BTreeSet::new(),
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
        let mut summary = SyncRunSummary::empty(
            best_header_height,
            best_block_height,
            self.config.target_outbound_peers,
        );
        summary.downloaded_block_height = best_block_height;
        summary
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
        block_reconcile::validate_block_limits(self)?;
        if block_reconcile::reconcile_best_chain(self, timestamp)? {
            self.persist_progress()?;
        }

        let peers = self.config.candidate_peers();
        if peers.is_empty() {
            let error = SyncRuntimeError::NoPeersConfigured;
            self.write_runtime_error_log(&error, timestamp);
            let state = self.durable_sync_state(
                SyncLifecycleState::Failed,
                Some(error.to_string()),
                timestamp,
            )?;
            self.persist_durable_sync_state(state)?;
            return Err(error);
        }

        let (best_header_height, best_block_height) = self.best_heights();
        let mut summary = SyncRunSummary::empty(
            best_header_height,
            best_block_height,
            self.config.target_outbound_peers,
        );
        self.refresh_summary_progress(&mut summary)?;
        let resolved_peers = self.resolve_candidates(peers, resolver, &mut summary);
        let mut completed_outbound_slots = 0_usize;
        for peer in resolved_peers {
            if completed_outbound_slots >= self.config.target_outbound_peers {
                break;
            }
            if let Some(backoff) = self.maybe_peer_backoff(&peer, timestamp) {
                self.record_waiting_outcome(&mut summary, &peer, backoff, timestamp);
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
        self.refresh_summary_progress(&mut summary)?;
        if let Err(error) = self.persist_metrics(&summary, timestamp) {
            self.write_runtime_error_log(&error, timestamp);
            let state = self.durable_sync_state_from_summary(
                &summary,
                SyncLifecycleState::Failed,
                Some(error.to_string()),
                timestamp,
            )?;
            self.persist_durable_sync_state(state)?;
            return Err(error);
        }
        self.write_summary_logs(&mut summary, timestamp);
        let state = self.durable_sync_state_from_summary(
            &summary,
            SyncLifecycleState::Active,
            summary.latest_error_message(),
            timestamp,
        )?;
        self.persist_durable_sync_state(state)?;

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
        let retry_backoff_seconds = retry_backoff_seconds(self.config.retry_backoff_ms);
        for _ in 1..self.config.max_rounds {
            current_timestamp = current_timestamp.saturating_add(retry_backoff_seconds);
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
                    return self.sync_connected_peer(session, peer, peer_id, attempts, timestamp);
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
                        maybe_progress: None,
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
    ) -> Result<PeerProgress, Box<PeerFailure>> {
        let mut progress = PeerProgress::new(peer.clone(), self.config.network, attempts);
        let result = (|| -> Result<(), SyncRuntimeError> {
            let mut outbound = self.network.connect_outbound_peer(peer_id, timestamp)?;
            outbound.extend(block_reconcile::request_missing_blocks(self, peer_id)?);
            self.send_all(&mut session, &outbound)?;

            for _ in 0..self.config.max_messages_per_peer {
                let Some(message) = session.receive(self.config.network.magic())? else {
                    progress.state = PeerSyncState::Stalled;
                    progress.maybe_failure_reason = Some(PeerFailureReason::Stall);
                    progress.maybe_capabilities = self.peer_capabilities(peer_id);
                    return Ok(());
                };
                progress.record_activity(timestamp);
                block_reconcile::release_inflight_for_message(self, &message);

                let maybe_header_count = match &message {
                    WireNetworkMessage::Headers(headers) => Some(headers.headers.len()),
                    _ => None,
                };
                let maybe_block = match &message {
                    WireNetworkMessage::Block(block) => Some(block.clone()),
                    _ => None,
                };
                let mut outbound = self.network.receive_sync_message(
                    peer_id,
                    message,
                    timestamp,
                    self.verify_flags,
                    self.consensus_params,
                )?;
                if let Some(header_count) = maybe_header_count {
                    progress.record_validated_headers(header_count);
                }
                if let Some(block) = maybe_block {
                    self.store.save_block(&block, self.config.persist_mode)?;
                    self.network
                        .note_local_block_hash(block_hash(&block.header));
                    progress.record_accepted_block();
                }
                let _ = block_reconcile::reconcile_best_chain(self, timestamp)?;
                self.persist_progress()?;
                outbound.extend(block_reconcile::request_missing_blocks(self, peer_id)?);
                self.send_all(&mut session, &outbound)?;
            }
            progress.state = PeerSyncState::Connected;
            progress.maybe_capabilities = self.peer_capabilities(peer_id);

            Ok(())
        })();
        let outstanding_blocks = self
            .network
            .peer_requested_blocks(peer_id)
            .unwrap_or_default();
        for block_hash in outstanding_blocks {
            self.inflight_blocks.remove(&block_hash);
        }
        let disconnect_result = self.network.disconnect_peer(peer_id);
        match (result, disconnect_result) {
            (Ok(()), Ok(())) => Ok(progress),
            (Ok(()), Err(error)) => {
                let error = SyncRuntimeError::from(error);
                if progress.maybe_capabilities.is_none() {
                    progress.maybe_capabilities = self.peer_capabilities(peer_id);
                }
                Err(Box::new(PeerFailure {
                    peer: peer.clone(),
                    reason: peer_failure_reason_for_error(&error),
                    error,
                    attempts,
                    maybe_progress: Some(progress),
                }))
            }
            (Err(error), _) => {
                if progress.maybe_capabilities.is_none() {
                    progress.maybe_capabilities = self.peer_capabilities(peer_id);
                }
                Err(Box::new(PeerFailure {
                    peer: peer.clone(),
                    reason: peer_failure_reason_for_error(&error),
                    error,
                    attempts,
                    maybe_progress: Some(progress),
                }))
            }
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
                summary.connected_peers += usize::from(progress.state == PeerSyncState::Connected);
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
                if let Some(progress) = failure.maybe_progress {
                    summary.messages_processed += progress.messages_processed;
                    summary.headers_received += progress.headers_received;
                    summary.blocks_received += progress.blocks_received;
                    let (best_header_height, best_block_height) = self.best_heights();
                    summary.best_header_height = best_header_height;
                    summary.best_block_height = best_block_height;
                    summary
                        .peer_outcomes
                        .push(progress.into_failed_outcome(failure.reason, Some(message)));
                } else {
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
    }

    fn record_waiting_outcome(
        &self,
        summary: &mut SyncRunSummary,
        peer: &ResolvedSyncPeerAddress,
        backoff: PeerRetryState,
        timestamp: i64,
    ) {
        let wait_seconds = backoff
            .next_attempt_unix_seconds
            .saturating_sub(timestamp)
            .max(0);
        summary.health_signals.push(progress::waiting_peer_signal());
        summary.peer_outcomes.push(PeerSyncOutcome {
            peer: peer.peer.clone(),
            maybe_resolved_endpoint: Some(peer.endpoint.to_string()),
            network: self.config.network,
            state: PeerSyncState::Waiting,
            attempts: 0,
            contribution: PeerContribution {
                messages_processed: 0,
                headers_received: 0,
                blocks_received: 0,
            },
            maybe_last_activity_unix_seconds: None,
            maybe_capabilities: None,
            maybe_failure_reason: Some(PeerFailureReason::RetryBackoff),
            maybe_error: Some(format!(
                "retry backoff wait_seconds={wait_seconds} consecutive_failures={} next_attempt_unix_seconds={}",
                backoff.consecutive_failures, backoff.next_attempt_unix_seconds
            )),
        });
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
}

fn peer_failure_reason_for_error(error: &SyncRuntimeError) -> PeerFailureReason {
    match error {
        SyncRuntimeError::AddressResolution { .. } => PeerFailureReason::AddressResolution,
        SyncRuntimeError::InvalidData { .. } => PeerFailureReason::InvalidData,
        SyncRuntimeError::InvalidMagic { .. } => PeerFailureReason::InvalidMagic,
        SyncRuntimeError::Storage(_) => PeerFailureReason::Storage,
        SyncRuntimeError::Io { .. } => PeerFailureReason::Connect,
        SyncRuntimeError::Network { .. }
        | SyncRuntimeError::NoPeersConfigured
        | SyncRuntimeError::ResourceLimit { .. } => PeerFailureReason::Network,
    }
}

fn retry_backoff_seconds(retry_backoff_ms: u64) -> i64 {
    let seconds = retry_backoff_ms.div_ceil(1_000).max(1);
    i64::try_from(seconds).unwrap_or(i64::MAX)
}
