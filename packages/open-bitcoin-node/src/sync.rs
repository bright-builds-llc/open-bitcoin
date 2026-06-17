// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! Real-network sync runtime shell.

use std::collections::{BTreeMap, BTreeSet};

mod block_reconcile;
mod block_response;
mod progress;
mod reconcile_status;
mod resolver;
mod runtime_state;
mod session;
mod tcp;
#[cfg(test)]
mod tests;
mod tip;
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
    SyncRunSummary, SyncRuntimeConfig, SyncRuntimeError, SyncStopReason, SyncTransport,
};
pub use wallet_rescan::WalletRescanRuntime;

use crate::{
    ChainstateStore, FjallNodeStore, ManagedPeerNetwork, MemoryChainstateStore, SyncLifecycleState,
    network::BlockConnectDisposition,
};
use progress::{PeerFailure, PeerProgress};
use types::SyncReconcileProgress;

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
    maybe_reconcile_progress: Option<SyncReconcileProgress>,
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
            maybe_reconcile_progress: None,
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
        if let Some(downloaded_block) = self.downloaded_block().ok().flatten() {
            summary.downloaded_block_height = downloaded_block.height;
            summary.maybe_downloaded_block_hash =
                Some(tip::block_hash_hex(downloaded_block.block_hash));
        }
        if let Some(connected_block) = self.connected_block() {
            summary.maybe_connected_block_hash =
                Some(tip::block_hash_hex(connected_block.block_hash));
            summary.maybe_validated_active_chain_work =
                Some(connected_block.chain_work.to_string());
        }
        summary.maybe_reconcile_progress = self.maybe_reconcile_progress.clone();
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
        self.maybe_reconcile_progress = None;
        block_reconcile::validate_block_limits(self)?;
        block_reconcile::reconcile_and_persist_best_chain(self, timestamp)?;

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
        summary.maybe_reconcile_progress = self.maybe_reconcile_progress.clone();
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
            if let Ok(progress) = &outcome
                && progress.state == PeerSyncState::Connected
                && progress.is_successful_outbound_slot()
            {
                completed_outbound_slots += 1;
            }
            self.record_outcome(&mut summary, outcome, timestamp);
        }
        self.refresh_summary_progress(&mut summary)?;
        summary.maybe_reconcile_progress = self.maybe_reconcile_progress.clone();
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
        let mut state = self.durable_sync_state_from_summary(
            &summary,
            SyncLifecycleState::Active,
            summary.latest_error_message(),
            timestamp,
        )?;
        self.write_progress_guarantee_log(&mut state, timestamp);
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
        if let Some(stop_reason) = self.maybe_target_header_stop_reason(&last_summary) {
            self.record_until_idle_stop(&mut last_summary, stop_reason, current_timestamp)?;
            return Ok(last_summary);
        }
        let mut previous_progress = progress::sync_progress_marker(&last_summary);
        let retry_backoff_seconds = progress::retry_backoff_seconds(self.config.retry_backoff_ms);
        let mut rounds_completed = 1_usize;
        for _ in 1..self.config.max_rounds {
            current_timestamp = current_timestamp.saturating_add(retry_backoff_seconds);
            let current_summary =
                self.sync_once_with_resolver(transport, resolver, current_timestamp)?;
            rounds_completed = rounds_completed.saturating_add(1);
            let current_progress = progress::sync_progress_marker(&current_summary);
            let is_idle = current_progress == previous_progress;
            last_summary = current_summary;
            if let Some(stop_reason) = self.maybe_target_header_stop_reason(&last_summary) {
                self.record_until_idle_stop(&mut last_summary, stop_reason, current_timestamp)?;
                return Ok(last_summary);
            }
            if is_idle {
                let stop_reason = self
                    .maybe_current_at_best_known_tip_stop_reason(current_timestamp)
                    .unwrap_or(SyncStopReason::NoProgress { rounds_completed });
                self.record_until_idle_stop(&mut last_summary, stop_reason, current_timestamp)?;
                break;
            }
            previous_progress = current_progress;
        }
        if last_summary.maybe_stop_reason.is_none() {
            self.record_until_idle_stop(
                &mut last_summary,
                SyncStopReason::MaxRoundsReached {
                    max_rounds: self.config.max_rounds,
                },
                current_timestamp,
            )?;
        }

        Ok(last_summary)
    }

    fn maybe_target_header_stop_reason(&self, summary: &SyncRunSummary) -> Option<SyncStopReason> {
        let target_header_height = self.config.maybe_target_header_height?;
        if summary.best_header_height < target_header_height {
            return None;
        }
        Some(SyncStopReason::TargetHeaderReached {
            target_header_height,
            best_header_height: summary.best_header_height,
        })
    }

    fn maybe_current_at_best_known_tip_stop_reason(
        &self,
        timestamp: i64,
    ) -> Option<SyncStopReason> {
        tip::current_at_best_known_tip_stop_reason_from_evidence(
            self.network.peer_manager().header_store().best_tip(),
            self.connected_block(),
            u64::try_from(timestamp).unwrap_or(0),
            self.config.tip_freshness_threshold_seconds,
        )
    }

    fn record_until_idle_stop(
        &self,
        summary: &mut SyncRunSummary,
        stop_reason: SyncStopReason,
        timestamp: i64,
    ) -> Result<(), SyncRuntimeError> {
        summary.maybe_stop_reason = Some(stop_reason);
        summary.health_signals.push(stop_reason.health_signal());
        let state = self.durable_sync_state_from_summary(
            summary,
            SyncLifecycleState::Active,
            summary.latest_error_message(),
            timestamp,
        )?;
        self.persist_durable_sync_state(state)?;
        Ok(())
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
                        reason: progress::peer_failure_reason_for_error(&error),
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
        let mut maybe_failure_reason_override = None;
        let result = (|| -> Result<(), SyncRuntimeError> {
            let mut outbound = self.network.connect_outbound_peer(peer_id, timestamp)?;
            outbound.extend(block_reconcile::request_missing_blocks(self, peer_id)?);
            self.send_all(&mut session, &outbound)?;

            for _ in 0..self.config.max_messages_per_peer {
                let maybe_message = match session.receive(self.config.network.magic()) {
                    Ok(maybe_message) => maybe_message,
                    Err(error) => {
                        let reason = progress::peer_failure_reason_for_error(&error);
                        if reason == PeerFailureReason::MalformedBlock {
                            progress.record_malformed_block();
                            maybe_failure_reason_override = Some(reason);
                        }
                        return Err(error);
                    }
                };
                let Some(message) = maybe_message else {
                    progress.maybe_capabilities = self.peer_capabilities(peer_id);
                    if !self.peer_handshake_complete(peer_id) {
                        progress.state = PeerSyncState::Stalled;
                        progress.maybe_failure_reason = Some(PeerFailureReason::Stall);
                    }
                    return Ok(());
                };
                progress.record_activity(timestamp);
                let maybe_header_count = match &message {
                    WireNetworkMessage::Headers(headers) => Some(headers.headers.len()),
                    _ => None,
                };
                let maybe_terminal_header_hash = match &message {
                    WireNetworkMessage::Headers(headers) => headers.headers.last().map(block_hash),
                    _ => None,
                };
                let maybe_block = match &message {
                    WireNetworkMessage::Block(block) => Some(block.clone()),
                    _ => None,
                };
                let maybe_block_hash = maybe_block.as_ref().map(|block| block_hash(&block.header));
                let block_response_was_requested = maybe_block_hash
                    .as_ref()
                    .is_some_and(|hash| self.peer_requested_block(peer_id, *hash));
                let block_response_is_best_chain = maybe_block_hash
                    .as_ref()
                    .is_some_and(|hash| self.block_has_best_chain_header(*hash));
                let notfound_was_requested =
                    self.message_reports_requested_block_notfound(peer_id, &message);
                block_reconcile::release_inflight_for_message(self, &message);

                if let Some(block) = maybe_block.as_ref()
                    && !block_response_was_requested
                {
                    self.record_unrequested_block_response(
                        &mut progress,
                        block,
                        block_response_is_best_chain,
                    )?;
                    let outbound = block_reconcile::request_missing_blocks(self, peer_id)?;
                    self.send_all(&mut session, &outbound)?;
                    continue;
                }

                let sync_result = match self.network.receive_sync_message(
                    peer_id,
                    message,
                    timestamp,
                    self.verify_flags,
                    self.consensus_params,
                ) {
                    Ok(result) => result,
                    Err(error) => {
                        if maybe_block_hash.is_some() {
                            progress.record_invalid_block();
                            maybe_failure_reason_override = Some(PeerFailureReason::InvalidBlock);
                        }
                        return Err(error.into());
                    }
                };
                let mut outbound = sync_result.outbound;
                if let Some(header_count) = maybe_header_count {
                    progress.record_validated_headers(header_count);
                    tip::record_peer_terminal_tip(
                        &mut progress,
                        self.network.peer_manager().header_store(),
                        maybe_terminal_header_hash,
                    );
                }
                if notfound_was_requested {
                    progress.record_block_notfound();
                }
                let mut should_persist_progress = maybe_header_count.is_some_and(|count| count > 0);
                if let Some(disposition) = sync_result.maybe_block_disposition {
                    should_persist_progress =
                        matches!(disposition, BlockConnectDisposition::Connected(_));
                    self.record_block_disposition(
                        &mut progress,
                        maybe_block.as_ref(),
                        disposition,
                        block_response_was_requested,
                        block_response_is_best_chain,
                    )?;
                }
                let reconcile_progress = block_reconcile::reconcile_best_chain(self, timestamp)?;
                should_persist_progress |= reconcile_progress.should_persist_progress();
                self.record_reconcile_progress(reconcile_progress);
                if should_persist_progress {
                    self.persist_progress()?;
                }
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
                    reason: progress::peer_failure_reason_for_error(&error),
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
                    reason: maybe_failure_reason_override
                        .clone()
                        .unwrap_or_else(|| progress::peer_failure_reason_for_error(&error)),
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
                let is_successful_outbound_slot = progress.is_successful_outbound_slot();
                let should_retry_with_backoff = progress.should_retry_with_backoff();
                if is_successful_outbound_slot {
                    self.clear_backoff(&progress.peer);
                } else if should_retry_with_backoff {
                    self.mark_backoff(&progress.peer, timestamp);
                }
                summary.connected_peers += usize::from(is_successful_outbound_slot);
                summary.messages_processed += progress.messages_processed;
                summary.headers_received += progress.headers_received;
                summary.blocks_received += progress.blocks_received;
                let (best_header_height, best_block_height) = self.best_heights();
                summary.best_header_height = best_header_height;
                summary.best_block_height = best_block_height;
                if progress.state == PeerSyncState::Stalled {
                    summary.health_signals.push(progress::stalled_peer_signal());
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
                        maybe_tip_height: None,
                        maybe_tip_hash: None,
                        maybe_tip_work: None,
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
            maybe_tip_height: None,
            maybe_tip_hash: None,
            maybe_tip_work: None,
            maybe_last_activity_unix_seconds: None,
            maybe_capabilities: None,
            maybe_failure_reason: Some(PeerFailureReason::RetryBackoff),
            maybe_error: Some(format!(
                "retry backoff wait_seconds={wait_seconds} consecutive_failures={} next_attempt_unix_seconds={}",
                backoff.consecutive_failures, backoff.next_attempt_unix_seconds
            )),
        });
    }
}
