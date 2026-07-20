// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! Real-network sync runtime shell.

use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};

mod block_reconcile;
mod block_response;
mod metrics;
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
mod waiting;
mod wallet_rescan;

use open_bitcoin_core::{
    consensus::{ConsensusParams, ScriptVerifyFlags},
    primitives::{Block, BlockHash},
};
use open_bitcoin_mempool::PolicyConfig;
use open_bitcoin_network::{BlockRelayActivationPolicy, RelayActivationConfig};

pub use resolver::{SyncPeerResolver, SystemSyncPeerResolver};
pub use session::{AnnouncementOutboxNotification, AnnouncementOutboxRegistry};
pub use tcp::{TcpPeerSession, TcpPeerTransport};
pub use types::{
    PeerCapabilitySummary, PeerContribution, PeerFailureReason, PeerIdentityAuthority,
    PeerSyncOutcome, PeerSyncState, ResolvedSyncPeerAddress, SyncNetwork, SyncPeerAddress,
    SyncPeerReceiveOutcome, SyncPeerSession, SyncPeerSource, SyncRunSummary, SyncRuntimeConfig,
    SyncRuntimeError, SyncStopReason, SyncTransport,
};
pub use wallet_rescan::WalletRescanRuntime;

use crate::{
    ChainstateStore, FieldAvailability, FjallNodeStore, InboundPeerServingStatus,
    ManagedNetworkHandle, ManagedPeerNetwork, MemoryChainstateStore, SyncLifecycleState,
    network::{BlockConnectDisposition, BlockRelayRuntimeEvidenceSnapshot},
};
use progress::{PeerFailure, PeerProgress};
use types::SyncReconcileProgress;

/// A validated best-tip block whose body and chainstate are durably available.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DurableTipAdvanced {
    block: Block,
}

impl DurableTipAdvanced {
    fn new(block: Block) -> Self {
        Self { block }
    }

    pub const fn block(&self) -> &Block {
        &self.block
    }
}

type DurableTipAnnouncementSink =
    Arc<dyn Fn(DurableTipAdvanced) -> Result<(), SyncRuntimeError> + Send + Sync>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PeerRetryState {
    consecutive_failures: u8,
    next_attempt_unix_seconds: i64,
}

pub struct DurableSyncRuntime {
    store: FjallNodeStore,
    network: ManagedNetworkHandle,
    config: SyncRuntimeConfig,
    verify_flags: ScriptVerifyFlags,
    consensus_params: ConsensusParams,
    peer_identity_authority: PeerIdentityAuthority,
    peer_backoff: BTreeMap<String, PeerRetryState>,
    inflight_blocks: BTreeSet<BlockHash>,
    maybe_reconcile_progress: Option<SyncReconcileProgress>,
    maybe_pending_durable_tip: Option<DurableTipAdvanced>,
    maybe_durable_tip_announcement_sink: Option<DurableTipAnnouncementSink>,
    announcement_outboxes: AnnouncementOutboxRegistry,
    maybe_inbound_metric_status_provider:
        Option<Arc<dyn Fn() -> FieldAvailability<InboundPeerServingStatus> + Send + Sync>>,
}

impl DurableSyncRuntime {
    pub fn open(
        store: FjallNodeStore,
        config: SyncRuntimeConfig,
    ) -> Result<Self, SyncRuntimeError> {
        Self::open_with_block_relay_activation(store, config, BlockRelayActivationPolicy::default())
    }

    /// Opens a durable runtime with the resolved block-relay activation policy.
    pub fn open_with_block_relay_activation(
        store: FjallNodeStore,
        config: SyncRuntimeConfig,
        block_relay_activation: BlockRelayActivationPolicy,
    ) -> Result<Self, SyncRuntimeError> {
        Self::open_with_runtime_activation(
            store,
            config,
            RelayActivationConfig::default(),
            block_relay_activation,
            false,
        )
    }

    /// Opens a durable runtime with all resolved network activation policies.
    pub fn open_with_runtime_activation(
        store: FjallNodeStore,
        config: SyncRuntimeConfig,
        relay_activation: RelayActivationConfig,
        block_relay_activation: BlockRelayActivationPolicy,
        inbound_enabled: bool,
    ) -> Result<Self, SyncRuntimeError> {
        let mut memory_store = MemoryChainstateStore::default();
        if let Some(snapshot) = store.load_chainstate_snapshot()? {
            memory_store.save_snapshot(snapshot);
        }

        let local_config = progress::local_peer_config(&config);
        let mut network = ManagedPeerNetwork::with_sync_limits_and_block_relay_activation(
            memory_store,
            local_config,
            PolicyConfig::default(),
            config.max_blocks_in_flight_per_peer,
            relay_activation,
            block_relay_activation,
            inbound_enabled,
        );
        if let Some(header_store) = store.load_header_store()? {
            network.seed_header_store(header_store);
        }
        let network = ManagedNetworkHandle::new(network);
        let announcement_outboxes = AnnouncementOutboxRegistry::default();
        let announcement_network = network.clone();
        let announcement_outboxes_for_sink = announcement_outboxes.clone();
        let durable_tip_announcement_sink: DurableTipAnnouncementSink = Arc::new(move |event| {
            let outboxes = announcement_outboxes_for_sink.snapshots()?;
            let outcomes =
                announcement_network.prepare_block_announcements(event.block(), &outboxes)?;
            announcement_outboxes_for_sink.enqueue_prepared(outcomes)
        });

        let consensus_params = config.network.consensus_params();
        Ok(Self {
            store,
            network,
            config,
            verify_flags: ScriptVerifyFlags::P2SH,
            consensus_params,
            peer_identity_authority: PeerIdentityAuthority::default(),
            peer_backoff: BTreeMap::new(),
            inflight_blocks: BTreeSet::new(),
            maybe_reconcile_progress: None,
            maybe_pending_durable_tip: None,
            maybe_durable_tip_announcement_sink: Some(durable_tip_announcement_sink),
            announcement_outboxes,
            maybe_inbound_metric_status_provider: None,
        })
    }

    pub fn config(&self) -> &SyncRuntimeConfig {
        &self.config
    }

    pub fn store(&self) -> &FjallNodeStore {
        &self.store
    }

    pub fn network_handle(&self) -> ManagedNetworkHandle {
        self.network.clone()
    }

    /// Returns the process-wide bounded announcement outboxes shared by live sessions.
    pub fn announcement_outboxes(&self) -> AnnouncementOutboxRegistry {
        self.announcement_outboxes.clone()
    }

    /// Returns the process-wide identity authority shared by inbound and outbound sessions.
    pub fn peer_identity_authority(&self) -> PeerIdentityAuthority {
        self.peer_identity_authority.clone()
    }

    /// Installs the production sink that routes post-durable tip events to peer outboxes.
    pub fn set_durable_tip_announcement_sink<F>(&mut self, sink: F)
    where
        F: Fn(DurableTipAdvanced) -> Result<(), SyncRuntimeError> + Send + Sync + 'static,
    {
        self.maybe_durable_tip_announcement_sink = Some(Arc::new(sink));
    }

    pub fn snapshot_summary(&self) -> SyncRunSummary {
        match self.try_snapshot_summary() {
            Ok(summary) => summary,
            Err(error) => {
                let mut summary = SyncRunSummary::empty(0, 0, self.config.target_outbound_peers);
                summary.health_signals.push(error.health_signal());
                summary
            }
        }
    }

    fn try_snapshot_summary(&self) -> Result<SyncRunSummary, SyncRuntimeError> {
        let (best_header_height, best_block_height) = self.best_heights()?;
        let mut summary = SyncRunSummary::empty(
            best_header_height,
            best_block_height,
            self.config.target_outbound_peers,
        );
        if let Some(downloaded_block) = self.downloaded_block()? {
            summary.downloaded_block_height = downloaded_block.height;
            summary.maybe_downloaded_block_hash =
                Some(tip::block_hash_hex(downloaded_block.block_hash));
        }
        if let Some(connected_block) = self.connected_block()? {
            summary.maybe_connected_block_hash =
                Some(tip::block_hash_hex(connected_block.block_hash));
            summary.maybe_validated_active_chain_work =
                Some(connected_block.chain_work.to_string());
        }
        summary.maybe_reconcile_progress = self.maybe_reconcile_progress.clone();
        Ok(summary)
    }

    /* Phase 121 compatibility anchors for the aggregate-first replacement:
    self.network.block_relay_runtime_evidence_snapshot()?
    match snapshot.status.block_serving.activation
    FieldAvailability::Available(_) => Ok(Some(snapshot)) */
    fn maybe_authoritative_block_relay_snapshot(
        &self,
    ) -> Result<Option<BlockRelayRuntimeEvidenceSnapshot>, SyncRuntimeError> {
        let snapshot = self.network.operator_snapshot()?;
        match snapshot.block_relay().block_serving.activation {
            FieldAvailability::Available(_) => Ok(Some(snapshot.block_relay_runtime_snapshot())),
            FieldAvailability::Unavailable { .. } => Ok(None),
        }
    }

    pub fn sync_once<T: SyncTransport>(
        &mut self,
        transport: &mut T,
        timestamp: i64,
    ) -> Result<SyncRunSummary, SyncRuntimeError> {
        let mut resolver = SystemSyncPeerResolver;
        let mut clock = || timestamp;
        self.sync_once_with_resolver_and_clock(transport, &mut resolver, timestamp, &mut clock)
    }

    pub fn sync_once_with_resolver<T: SyncTransport, R: SyncPeerResolver>(
        &mut self,
        transport: &mut T,
        resolver: &mut R,
        timestamp: i64,
    ) -> Result<SyncRunSummary, SyncRuntimeError> {
        let mut clock = || timestamp;
        self.sync_once_with_resolver_and_clock(transport, resolver, timestamp, &mut clock)
    }

    fn sync_once_with_resolver_and_clock<
        T: SyncTransport,
        R: SyncPeerResolver,
        C: FnMut() -> i64,
    >(
        &mut self,
        transport: &mut T,
        resolver: &mut R,
        timestamp: i64,
        clock: &mut C,
    ) -> Result<SyncRunSummary, SyncRuntimeError> {
        let mut never_cancel = || false;
        self.sync_once_with_resolver_clock_and_cancel(
            transport,
            resolver,
            timestamp,
            clock,
            &mut never_cancel,
        )
    }

    fn sync_once_with_resolver_clock_and_cancel<
        T: SyncTransport,
        R: SyncPeerResolver,
        C: FnMut() -> i64,
        K: FnMut() -> bool,
    >(
        &mut self,
        transport: &mut T,
        resolver: &mut R,
        timestamp: i64,
        clock: &mut C,
        should_cancel: &mut K,
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

        let (best_header_height, best_block_height) = self.best_heights()?;
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
            if should_cancel() || completed_outbound_slots >= self.config.target_outbound_peers {
                break;
            }
            if let Some(backoff) = self.maybe_peer_backoff(&peer, timestamp) {
                self.record_waiting_outcome(&mut summary, &peer, backoff, timestamp);
                continue;
            }
            summary.attempted_peers += 1;
            let peer_id = self.allocate_peer_id()?;
            let outcome = self.sync_peer_with_retries(
                transport,
                &peer,
                peer_id,
                timestamp,
                clock,
                should_cancel,
            );
            if let Ok(progress) = &outcome
                && progress.state == PeerSyncState::Connected
                && progress.is_successful_outbound_slot()
            {
                completed_outbound_slots += 1;
            }
            self.record_outcome(&mut summary, outcome, timestamp)?;
        }
        self.refresh_summary_progress(&mut summary)?;
        summary.maybe_reconcile_progress = self.maybe_reconcile_progress.clone();
        let maybe_block_relay_snapshot = self.maybe_authoritative_block_relay_snapshot()?;
        if let Err(error) =
            self.persist_metrics(&summary, maybe_block_relay_snapshot.as_ref(), timestamp)
        {
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
        self.write_block_relay_log(&mut summary, maybe_block_relay_snapshot.as_ref(), timestamp);
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
        let mut clock = || timestamp;
        self.sync_until_idle_with_resolver_and_clock(
            transport,
            &mut resolver,
            timestamp,
            &mut clock,
        )
    }

    /// Runs sync rounds with a caller clock sampled once for each live-session idle wake.
    pub fn sync_until_idle_with_clock<T: SyncTransport, C: FnMut() -> i64>(
        &mut self,
        transport: &mut T,
        timestamp: i64,
        clock: &mut C,
    ) -> Result<SyncRunSummary, SyncRuntimeError> {
        let mut never_cancel = || false;
        self.sync_until_idle_with_clock_and_cancel(transport, timestamp, clock, &mut never_cancel)
    }

    /// Runs bounded sync rounds while allowing the caller to cancel a live idle session.
    pub fn sync_until_idle_with_clock_and_cancel<
        T: SyncTransport,
        C: FnMut() -> i64,
        K: FnMut() -> bool,
    >(
        &mut self,
        transport: &mut T,
        timestamp: i64,
        clock: &mut C,
        should_cancel: &mut K,
    ) -> Result<SyncRunSummary, SyncRuntimeError> {
        let mut resolver = SystemSyncPeerResolver;
        self.sync_until_idle_with_resolver_clock_and_cancel(
            transport,
            &mut resolver,
            timestamp,
            clock,
            should_cancel,
        )
    }

    pub fn sync_until_idle_with_resolver<T: SyncTransport, R: SyncPeerResolver>(
        &mut self,
        transport: &mut T,
        resolver: &mut R,
        timestamp: i64,
    ) -> Result<SyncRunSummary, SyncRuntimeError> {
        let mut clock = || timestamp;
        self.sync_until_idle_with_resolver_and_clock(transport, resolver, timestamp, &mut clock)
    }

    fn sync_until_idle_with_resolver_and_clock<
        T: SyncTransport,
        R: SyncPeerResolver,
        C: FnMut() -> i64,
    >(
        &mut self,
        transport: &mut T,
        resolver: &mut R,
        timestamp: i64,
        clock: &mut C,
    ) -> Result<SyncRunSummary, SyncRuntimeError> {
        let mut never_cancel = || false;
        self.sync_until_idle_with_resolver_clock_and_cancel(
            transport,
            resolver,
            timestamp,
            clock,
            &mut never_cancel,
        )
    }

    fn sync_until_idle_with_resolver_clock_and_cancel<
        T: SyncTransport,
        R: SyncPeerResolver,
        C: FnMut() -> i64,
        K: FnMut() -> bool,
    >(
        &mut self,
        transport: &mut T,
        resolver: &mut R,
        timestamp: i64,
        clock: &mut C,
        should_cancel: &mut K,
    ) -> Result<SyncRunSummary, SyncRuntimeError> {
        let mut current_timestamp = timestamp;
        let mut last_summary = self.sync_once_with_resolver_clock_and_cancel(
            transport,
            resolver,
            current_timestamp,
            clock,
            should_cancel,
        )?;
        if should_cancel() {
            return Ok(last_summary);
        }
        if let Some(stop_reason) = self.maybe_target_header_stop_reason(&last_summary) {
            self.record_until_idle_stop(&mut last_summary, stop_reason, current_timestamp)?;
            return Ok(last_summary);
        }
        let mut previous_progress = progress::sync_progress_marker(&last_summary);
        let retry_backoff_seconds = progress::retry_backoff_seconds(self.config.retry_backoff_ms);
        let mut rounds_completed = 1_usize;
        for _ in 1..self.config.max_rounds {
            current_timestamp = current_timestamp.saturating_add(retry_backoff_seconds);
            let current_summary = self.sync_once_with_resolver_clock_and_cancel(
                transport,
                resolver,
                current_timestamp,
                clock,
                should_cancel,
            )?;
            if should_cancel() {
                return Ok(current_summary);
            }
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
                    .maybe_current_at_best_known_tip_stop_reason(current_timestamp)?
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

    fn record_outcome(
        &mut self,
        summary: &mut SyncRunSummary,
        outcome: Result<PeerProgress, Box<PeerFailure>>,
        timestamp: i64,
    ) -> Result<(), SyncRuntimeError> {
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
                let (best_header_height, best_block_height) = self.best_heights()?;
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
                    let (best_header_height, best_block_height) = self.best_heights()?;
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
        Ok(())
    }
}
