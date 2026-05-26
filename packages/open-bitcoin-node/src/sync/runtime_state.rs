// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use open_bitcoin_network::{MAX_HEADERS_RESULTS, PeerId};

use crate::{
    LogRetentionPolicy, MetricRetentionPolicy, RuntimeMetadata,
    logging::{StructuredLogError, StructuredLogRecord, writer::append_structured_log_record},
    status::{
        DurableSyncState, FieldAvailability, SyncControlState, SyncLifecycleState,
        SyncResourcePressure,
    },
};

use super::{
    DurableSyncRuntime, PeerCapabilitySummary, PeerRetryState, ResolvedSyncPeerAddress,
    SyncPeerAddress, SyncPeerResolver, SyncRunSummary, SyncRuntimeError,
};

const MAX_HEADER_REQUESTS_IN_FLIGHT_PER_PEER: u64 = 1;

impl DurableSyncRuntime {
    pub fn load_sync_control(&self) -> Result<SyncControlState, SyncRuntimeError> {
        Ok(self.load_runtime_metadata()?.sync_control)
    }

    pub fn set_sync_paused(&self, paused: bool) -> Result<(), SyncRuntimeError> {
        let mut metadata = self.load_runtime_metadata()?;
        metadata.sync_control.paused = paused;
        self.store
            .save_runtime_metadata(&metadata, self.config.persist_mode)?;
        Ok(())
    }

    pub fn durable_sync_state(
        &self,
        lifecycle: SyncLifecycleState,
        maybe_last_error: Option<String>,
        timestamp: i64,
    ) -> Result<DurableSyncState, SyncRuntimeError> {
        let summary = self.snapshot_summary();
        self.durable_sync_state_from_summary(&summary, lifecycle, maybe_last_error, timestamp)
    }

    pub fn durable_sync_state_for_summary(
        &self,
        summary: &SyncRunSummary,
        lifecycle: SyncLifecycleState,
        maybe_last_error: Option<String>,
        timestamp: i64,
    ) -> Result<DurableSyncState, SyncRuntimeError> {
        self.durable_sync_state_from_summary(summary, lifecycle, maybe_last_error, timestamp)
    }

    pub fn persist_durable_sync_state(
        &self,
        state: DurableSyncState,
    ) -> Result<(), SyncRuntimeError> {
        let mut metadata = self.load_runtime_metadata()?;
        metadata.last_clean_shutdown = false;
        metadata.maybe_sync_state = Some(state);
        self.store
            .save_runtime_metadata(&metadata, self.config.persist_mode)?;
        Ok(())
    }

    pub(super) fn persist_progress(&self) -> Result<(), SyncRuntimeError> {
        self.store
            .save_header_entries(&self.network.header_entries(), self.config.persist_mode)?;
        self.store.save_chainstate_snapshot(
            &self.network.chainstate_snapshot(),
            self.config.persist_mode,
        )?;
        let mut metadata = self.load_runtime_metadata()?;
        metadata.last_clean_shutdown = false;
        self.store
            .save_runtime_metadata(&metadata, self.config.persist_mode)?;

        Ok(())
    }

    pub(super) fn persist_metrics(
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

    pub(super) fn write_summary_logs(&self, summary: &mut SyncRunSummary, timestamp: i64) {
        let timestamp = u64::try_from(timestamp).unwrap_or(0);
        for record in summary.structured_log_records(timestamp) {
            if let Err(error) = self.append_structured_record(&record) {
                summary
                    .health_signals
                    .push(super::progress::log_write_failed_signal(&error));
                break;
            }
        }
    }

    pub(super) fn write_runtime_error_log(&self, error: &SyncRuntimeError, timestamp: i64) {
        let signal = error.health_signal();
        let record = StructuredLogRecord {
            level: super::progress::structured_log_level(signal.level),
            source: signal.source,
            message: signal.message,
            timestamp_unix_seconds: u64::try_from(timestamp).unwrap_or(0),
        };
        let _ = self.append_structured_record(&record);
    }

    pub(super) fn best_heights(&self) -> (u64, u64) {
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

    pub(super) fn refresh_summary_progress(
        &self,
        summary: &mut SyncRunSummary,
    ) -> Result<(), SyncRuntimeError> {
        let (best_header_height, best_block_height) = self.best_heights();
        summary.best_header_height = best_header_height;
        summary.best_block_height = best_block_height;
        summary.downloaded_block_height = self.downloaded_block_height()?;
        Ok(())
    }

    fn downloaded_block_height(&self) -> Result<u64, SyncRuntimeError> {
        let active_chain = self.network.chainstate_snapshot().active_chain;
        let best_chain = self.network.best_chain_entries();
        if best_chain.is_empty() {
            return Ok(active_chain
                .last()
                .map_or(0, |position| u64::from(position.height)));
        }

        let mut common_prefix_len = 0_usize;
        while common_prefix_len < active_chain.len()
            && common_prefix_len < best_chain.len()
            && active_chain[common_prefix_len].block_hash
                == best_chain[common_prefix_len].block_hash
        {
            common_prefix_len += 1;
        }

        let mut downloaded_height = if common_prefix_len == 0 {
            0
        } else {
            u64::from(best_chain[common_prefix_len - 1].height)
        };
        for entry in best_chain.iter().skip(common_prefix_len) {
            if self.store.load_block(entry.block_hash)?.is_none() {
                break;
            }
            downloaded_height = u64::from(entry.height);
        }

        Ok(downloaded_height.max(
            active_chain
                .last()
                .map_or(0, |position| u64::from(position.height)),
        ))
    }

    pub(super) fn allocate_peer_id(&mut self) -> PeerId {
        let peer_id = self.next_peer_id;
        self.next_peer_id = self.next_peer_id.saturating_add(1);
        peer_id
    }

    pub(super) fn resolve_candidates<R: SyncPeerResolver>(
        &self,
        peers: Vec<SyncPeerAddress>,
        resolver: &mut R,
        summary: &mut SyncRunSummary,
    ) -> Vec<ResolvedSyncPeerAddress> {
        let mut resolved = Vec::new();
        let mut seen = std::collections::BTreeSet::new();
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
                    summary.peer_outcomes.push(super::PeerSyncOutcome {
                        peer,
                        maybe_resolved_endpoint: None,
                        network: self.config.network,
                        state: super::PeerSyncState::Failed,
                        attempts: 1,
                        contribution: super::PeerContribution {
                            messages_processed: 0,
                            headers_received: 0,
                            blocks_received: 0,
                        },
                        maybe_last_activity_unix_seconds: None,
                        maybe_capabilities: None,
                        maybe_failure_reason: Some(super::PeerFailureReason::AddressResolution),
                        maybe_error: Some(message),
                    });
                }
            }
        }
        resolved
    }

    pub(super) fn peer_capabilities(&self, peer_id: PeerId) -> Option<PeerCapabilitySummary> {
        let peer = self.network.peer_manager().peer_state(peer_id)?;
        Some(PeerCapabilitySummary {
            services_bits: peer.remote_services_bits,
            user_agent: peer.remote_user_agent.clone(),
            start_height: peer.remote_start_height,
            wtxidrelay: peer.remote_wtxidrelay,
            prefers_headers: peer.remote_prefers_headers,
        })
    }

    pub(super) fn maybe_peer_backoff(
        &self,
        peer: &ResolvedSyncPeerAddress,
        timestamp: i64,
    ) -> Option<PeerRetryState> {
        let key = peer.endpoint.to_string();
        self.peer_backoff
            .get(&key)
            .copied()
            .filter(|state| state.next_attempt_unix_seconds > timestamp)
    }

    pub(super) fn mark_backoff(&mut self, peer: &ResolvedSyncPeerAddress, timestamp: i64) {
        let key = peer.endpoint.to_string();
        let mut state = self
            .peer_backoff
            .get(&key)
            .copied()
            .unwrap_or(super::PeerRetryState {
                consecutive_failures: 0,
                next_attempt_unix_seconds: timestamp,
            });
        state.consecutive_failures = state.consecutive_failures.saturating_add(1);
        let multiplier = i64::from(state.consecutive_failures);
        let backoff = super::retry_backoff_seconds(self.config.retry_backoff_ms);
        state.next_attempt_unix_seconds =
            timestamp.saturating_add(backoff.saturating_mul(multiplier));
        self.peer_backoff.insert(key, state);
    }

    pub(super) fn clear_backoff(&mut self, peer: &ResolvedSyncPeerAddress) {
        self.peer_backoff.remove(&peer.endpoint.to_string());
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

    pub(super) fn durable_sync_state_from_summary(
        &self,
        summary: &SyncRunSummary,
        lifecycle: SyncLifecycleState,
        maybe_last_error: Option<String>,
        timestamp: i64,
    ) -> Result<DurableSyncState, SyncRuntimeError> {
        let metadata = self.load_runtime_metadata()?;
        let mut sync = summary.sync_status(self.config.network);
        if let FieldAvailability::Available(progress) = &mut sync.sync_progress {
            progress.downloaded_block_height = self.downloaded_block_height()?;
            progress.connected_block_height = progress.block_height;
        }
        sync.lifecycle = FieldAvailability::available(lifecycle);
        sync.phase = FieldAvailability::available(match lifecycle {
            SyncLifecycleState::Paused => "paused".to_string(),
            SyncLifecycleState::Recovering => "recovering".to_string(),
            SyncLifecycleState::Failed => "failed".to_string(),
            SyncLifecycleState::Stopped => "stopped".to_string(),
            SyncLifecycleState::Active => match &sync.phase {
                FieldAvailability::Available(value) => value.clone(),
                FieldAvailability::Unavailable { .. } => "steady_state".to_string(),
            },
        });
        sync.last_error = match maybe_last_error {
            Some(value) => FieldAvailability::available(value),
            None => FieldAvailability::unavailable("no sync error recorded"),
        };
        sync.recovery_action = match metadata.maybe_last_recovery_action {
            Some(value) => FieldAvailability::available(value.operator_message().to_string()),
            None => match summary.latest_recovery_action() {
                Some(value) => FieldAvailability::available(value.to_string()),
                None => FieldAvailability::unavailable("no recovery action required"),
            },
        };
        sync.resource_pressure = FieldAvailability::available(SyncResourcePressure {
            blocks_in_flight: self.inflight_blocks.len() as u64,
            max_header_requests_in_flight_per_peer: MAX_HEADER_REQUESTS_IN_FLIGHT_PER_PEER,
            max_headers_per_message: MAX_HEADERS_RESULTS as u64,
            max_blocks_in_flight_per_peer: self.config.max_blocks_in_flight_per_peer as u64,
            max_blocks_in_flight_total: self.config.max_blocks_in_flight_total as u64,
            max_messages_per_peer: self.config.max_messages_per_peer as u64,
            max_sync_rounds: self.config.max_rounds as u64,
            outbound_peers: summary.connected_peers as u32,
            target_outbound_peers: self.config.target_outbound_peers as u32,
        });

        Ok(DurableSyncState {
            sync,
            peers: summary.peer_status(),
            health_signals: summary.health_signals.clone(),
            updated_at_unix_seconds: u64::try_from(timestamp).unwrap_or(0),
        })
    }

    fn load_runtime_metadata(&self) -> Result<RuntimeMetadata, SyncRuntimeError> {
        Ok(self.store.load_runtime_metadata()?.unwrap_or_default())
    }
}
