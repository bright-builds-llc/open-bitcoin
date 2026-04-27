// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! Real-network sync runtime shell.

mod progress;
mod tcp;
#[cfg(test)]
mod tests;
mod types;
mod wallet_rescan;

use open_bitcoin_core::consensus::{ConsensusParams, ScriptVerifyFlags};
use open_bitcoin_mempool::PolicyConfig;
use open_bitcoin_network::{PeerId, WireNetworkMessage};

pub use tcp::{TcpPeerSession, TcpPeerTransport};
pub use types::{
    PeerSyncOutcome, PeerSyncState, SyncNetwork, SyncPeerAddress, SyncPeerSession, SyncPeerSource,
    SyncRunSummary, SyncRuntimeConfig, SyncRuntimeError, SyncTransport,
};
pub use wallet_rescan::WalletRescanRuntime;

use crate::{
    ChainstateStore, FjallNodeStore, LogRetentionPolicy, ManagedPeerNetwork, MemoryChainstateStore,
    MetricRetentionPolicy, RuntimeMetadata,
    logging::{StructuredLogError, StructuredLogRecord, writer::append_structured_log_record},
};
use progress::{PeerFailure, PeerProgress};

pub struct DurableSyncRuntime {
    store: FjallNodeStore,
    network: ManagedPeerNetwork<MemoryChainstateStore>,
    config: SyncRuntimeConfig,
    verify_flags: ScriptVerifyFlags,
    consensus_params: ConsensusParams,
    next_peer_id: PeerId,
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
        let peers = self.config.candidate_peers();
        if peers.is_empty() {
            let error = SyncRuntimeError::NoPeersConfigured;
            self.write_runtime_error_log(&error, timestamp);
            return Err(error);
        }

        let (best_header_height, best_block_height) = self.best_heights();
        let mut summary = SyncRunSummary::empty(best_header_height, best_block_height);
        for peer in &peers {
            summary.attempted_peers += 1;
            let peer_id = self.allocate_peer_id();
            let outcome = self.sync_peer_with_retries(transport, peer, peer_id, timestamp);
            self.record_outcome(&mut summary, peer, outcome);
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
        let mut last_summary = self.sync_once(transport, timestamp)?;
        let mut previous_progress = progress::sync_progress_marker(&last_summary);
        for _ in 1..self.config.max_rounds {
            let current_summary = self.sync_once(transport, timestamp)?;
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
        peer: &SyncPeerAddress,
        peer_id: PeerId,
        timestamp: i64,
    ) -> Result<PeerProgress, PeerFailure> {
        let mut attempts = 0_u8;
        let max_attempts = self.config.max_peer_retries.saturating_add(1);
        loop {
            attempts = attempts.saturating_add(1);
            match transport.connect(peer, &self.config) {
                Ok(session) => {
                    return self
                        .sync_connected_peer(session, peer, peer_id, attempts, timestamp)
                        .map_err(|error| PeerFailure { error, attempts });
                }
                Err(error) if attempts < max_attempts => {
                    let _ = error;
                }
                Err(error) => return Err(PeerFailure { error, attempts }),
            }
        }
    }

    fn sync_connected_peer<S: SyncPeerSession>(
        &mut self,
        mut session: S,
        peer: &SyncPeerAddress,
        peer_id: PeerId,
        attempts: u8,
        timestamp: i64,
    ) -> Result<PeerProgress, SyncRuntimeError> {
        let outbound = self.network.connect_outbound_peer(peer_id, timestamp)?;
        self.send_all(&mut session, &outbound)?;

        let mut progress = PeerProgress::new(peer.clone(), attempts);
        for _ in 0..self.config.max_messages_per_peer {
            let Some(message) = session.receive(self.config.network.magic())? else {
                progress.state = PeerSyncState::Stalled;
                return Ok(progress);
            };
            progress.messages_processed += 1;
            progress.record_message(&message);

            let maybe_block = match &message {
                WireNetworkMessage::Block(block) => Some(block.clone()),
                _ => None,
            };
            let outbound = self.network.receive_message(
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

        Ok(progress)
    }

    fn record_outcome(
        &self,
        summary: &mut SyncRunSummary,
        peer: &SyncPeerAddress,
        outcome: Result<PeerProgress, PeerFailure>,
    ) {
        match outcome {
            Ok(progress) => {
                summary.connected_peers += usize::from(progress.state != PeerSyncState::Failed);
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
                summary.failed_peers += 1;
                let signal = failure.error.health_signal();
                let message = signal.message.clone();
                summary.health_signals.push(signal);
                summary.peer_outcomes.push(PeerSyncOutcome {
                    peer: peer.clone(),
                    state: PeerSyncState::Failed,
                    attempts: failure.attempts,
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
}
