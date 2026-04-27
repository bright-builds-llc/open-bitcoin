// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! Real-network sync runtime shell.

mod tcp;
mod types;

use open_bitcoin_core::{
    consensus::{ConsensusParams, ScriptVerifyFlags},
    primitives::NetworkAddress,
};
use open_bitcoin_mempool::PolicyConfig;
use open_bitcoin_network::{LocalPeerConfig, PeerId, ServiceFlags, WireNetworkMessage};
use open_bitcoin_wallet::wallet::WalletRescanState;

pub use tcp::{TcpPeerSession, TcpPeerTransport};
pub use types::{
    PeerSyncOutcome, PeerSyncState, SyncNetwork, SyncPeerAddress, SyncPeerSession, SyncPeerSource,
    SyncRunSummary, SyncRuntimeConfig, SyncRuntimeError, SyncTransport,
};

use crate::{
    ChainstateStore, FjallNodeStore, LogRetentionPolicy, ManagedPeerNetwork, MemoryChainstateStore,
    MetricRetentionPolicy, PersistMode, RuntimeMetadata, StorageError, StorageNamespace,
    logging::{
        StructuredLogError, StructuredLogLevel, StructuredLogRecord,
        writer::append_structured_log_record,
    },
    status::{HealthSignal, HealthSignalLevel},
    wallet_registry::{
        WalletRegistry, WalletRegistryError, WalletRescanFreshness, WalletRescanJob,
        WalletRescanJobState,
    },
};

const DEFAULT_WALLET_RESCAN_CHUNK_SIZE: u32 = 128;

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

        let local_config = local_peer_config(&config);
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
        let mut previous_progress = sync_progress_marker(&last_summary);
        for _ in 1..self.config.max_rounds {
            let current_summary = self.sync_once(transport, timestamp)?;
            let current_progress = sync_progress_marker(&current_summary);
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
                    summary.health_signals.push(HealthSignal {
                        level: HealthSignalLevel::Warn,
                        source: "sync".to_string(),
                        message: "peer stalled before sending more sync messages".to_string(),
                    });
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
                summary.health_signals.push(log_write_failed_signal(&error));
                break;
            }
        }
    }

    fn write_runtime_error_log(&self, error: &SyncRuntimeError, timestamp: i64) {
        let signal = error.health_signal();
        let record = StructuredLogRecord {
            level: structured_log_level(signal.level),
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

#[derive(Debug, Clone, PartialEq, Eq)]
struct PeerProgress {
    peer: SyncPeerAddress,
    state: PeerSyncState,
    attempts: u8,
    messages_processed: usize,
    headers_received: usize,
    blocks_received: usize,
}

#[derive(Debug)]
struct PeerFailure {
    error: SyncRuntimeError,
    attempts: u8,
}

impl PeerProgress {
    fn new(peer: SyncPeerAddress, attempts: u8) -> Self {
        Self {
            peer,
            state: PeerSyncState::Connected,
            attempts,
            messages_processed: 0,
            headers_received: 0,
            blocks_received: 0,
        }
    }

    fn record_message(&mut self, message: &WireNetworkMessage) {
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

    fn into_outcome(self, maybe_error: Option<String>) -> PeerSyncOutcome {
        PeerSyncOutcome {
            peer: self.peer,
            state: self.state,
            attempts: self.attempts,
            maybe_error,
        }
    }
}

fn structured_log_level(level: HealthSignalLevel) -> StructuredLogLevel {
    match level {
        HealthSignalLevel::Info => StructuredLogLevel::Info,
        HealthSignalLevel::Warn => StructuredLogLevel::Warn,
        HealthSignalLevel::Error => StructuredLogLevel::Error,
    }
}

fn sync_progress_marker(summary: &SyncRunSummary) -> (u64, u64) {
    (summary.best_header_height, summary.best_block_height)
}

fn log_write_failed_signal(error: &StructuredLogError) -> HealthSignal {
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

fn local_peer_config(config: &SyncRuntimeConfig) -> LocalPeerConfig {
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

pub struct WalletRescanRuntime {
    store: FjallNodeStore,
    persist_mode: PersistMode,
    chunk_size: u32,
}

impl WalletRescanRuntime {
    pub fn open(
        store: FjallNodeStore,
        persist_mode: PersistMode,
    ) -> Result<Self, WalletRegistryError> {
        Self::open_with_chunk_size(store, persist_mode, DEFAULT_WALLET_RESCAN_CHUNK_SIZE)
    }

    pub(crate) fn open_with_chunk_size(
        store: FjallNodeStore,
        persist_mode: PersistMode,
        chunk_size: u32,
    ) -> Result<Self, WalletRegistryError> {
        let runtime = Self {
            store,
            persist_mode,
            chunk_size: chunk_size.max(1),
        };
        let _ = runtime.resume_pending_jobs()?;
        Ok(runtime)
    }

    pub fn store(&self) -> &FjallNodeStore {
        &self.store
    }

    pub fn enqueue_rescan(
        &self,
        wallet_name: &str,
    ) -> Result<WalletRescanJob, WalletRegistryError> {
        let mut registry = WalletRegistry::load(&self.store)?;
        let wallet_snapshot = registry.wallet_snapshot(wallet_name)?;
        let chainstate = self.required_chainstate_snapshot()?;
        let Some(target_tip) = chainstate.tip() else {
            return Err(WalletRegistryError::Storage(
                StorageError::UnavailableNamespace {
                    namespace: StorageNamespace::Chainstate,
                },
            ));
        };

        let mut job = WalletRescanJob::new(
            wallet_name,
            target_tip.block_hash,
            target_tip.height,
            wallet_snapshot
                .maybe_tip_height
                .map_or(0, |height| height.saturating_add(1)),
            wallet_snapshot.maybe_tip_height,
        )?;
        job.state = WalletRescanJobState::Pending;
        job.freshness = WalletRescanFreshness::from_wallet_state(WalletRescanState::from_progress(
            job.maybe_scanned_through_height,
            Some(job.target_tip_height),
            Some(job.next_height),
            true,
        )?);
        registry.save_rescan_job(&self.store, job, self.persist_mode)?;

        self.advance_wallet_rescan(wallet_name)
    }

    pub fn resume_pending_jobs(&self) -> Result<Vec<WalletRescanJob>, WalletRegistryError> {
        let registry = WalletRegistry::load(&self.store)?;
        let pending_wallet_names = registry
            .rescan_jobs()
            .filter(|job| job.requires_resume())
            .map(|job| job.wallet_name.clone())
            .collect::<Vec<_>>();

        let mut advanced_jobs = Vec::with_capacity(pending_wallet_names.len());
        for wallet_name in pending_wallet_names {
            advanced_jobs.push(self.advance_wallet_rescan(wallet_name.as_str())?);
        }
        Ok(advanced_jobs)
    }

    pub fn advance_wallet_rescan(
        &self,
        wallet_name: &str,
    ) -> Result<WalletRescanJob, WalletRegistryError> {
        let chainstate = self.required_chainstate_snapshot()?;
        let mut registry = WalletRegistry::load(&self.store)?;
        let mut job = registry
            .rescan_job(wallet_name)
            .cloned()
            .ok_or_else(|| WalletRegistryError::UnknownWallet(wallet_name.to_string()))?;
        if !job.requires_resume() {
            return Ok(job);
        }

        let chunk_end_height =
            chunk_end_height(job.next_height, job.target_tip_height, self.chunk_size);
        let partial_snapshot = partial_chainstate_snapshot(&chainstate, chunk_end_height);
        let maybe_tip_median_time_past = partial_snapshot.tip().map(|tip| tip.median_time_past);
        let mut wallet = registry.wallet(wallet_name)?;
        if let Err(error) = wallet.rescan_chainstate(&partial_snapshot) {
            job.mark_failed(error.to_string());
            registry.save_rescan_job(&self.store, job.clone(), self.persist_mode)?;
            return Err(error.into());
        }

        registry.save_wallet(&self.store, wallet_name, &wallet, self.persist_mode)?;
        job.mark_chunk_progress(chunk_end_height, maybe_tip_median_time_past);
        registry.save_rescan_job(&self.store, job.clone(), self.persist_mode)?;
        Ok(job)
    }

    fn required_chainstate_snapshot(
        &self,
    ) -> Result<open_bitcoin_core::chainstate::ChainstateSnapshot, WalletRegistryError> {
        self.store.load_chainstate_snapshot()?.ok_or({
            WalletRegistryError::Storage(StorageError::UnavailableNamespace {
                namespace: StorageNamespace::Chainstate,
            })
        })
    }
}

fn chunk_end_height(next_height: u32, target_tip_height: u32, chunk_size: u32) -> u32 {
    next_height
        .saturating_add(chunk_size.saturating_sub(1))
        .min(target_tip_height)
}

fn partial_chainstate_snapshot(
    snapshot: &open_bitcoin_core::chainstate::ChainstateSnapshot,
    through_height: u32,
) -> open_bitcoin_core::chainstate::ChainstateSnapshot {
    let active_chain = snapshot
        .active_chain
        .iter()
        .filter(|position| position.height <= through_height)
        .cloned()
        .collect::<Vec<_>>();
    let active_hashes = active_chain
        .iter()
        .map(|position| position.block_hash)
        .collect::<std::collections::BTreeSet<_>>();
    let utxos = snapshot
        .utxos
        .iter()
        .filter(|(_, coin)| coin.created_height <= through_height)
        .map(|(outpoint, coin)| (outpoint.clone(), coin.clone()))
        .collect();
    let undo_by_block = snapshot
        .undo_by_block
        .iter()
        .filter(|(block_hash, _)| active_hashes.contains(block_hash))
        .map(|(block_hash, undo)| (*block_hash, undo.clone()))
        .collect();

    open_bitcoin_core::chainstate::ChainstateSnapshot::new(active_chain, utxos, undo_by_block)
}

#[cfg(test)]
mod wallet_rescan_runtime_tests {
    use std::{
        collections::HashMap,
        fs, io,
        path::{Path, PathBuf},
        time::{SystemTime, UNIX_EPOCH},
    };

    use open_bitcoin_core::{
        chainstate::{ChainPosition, ChainstateSnapshot, Coin},
        primitives::{BlockHash, BlockHeader, OutPoint, TransactionOutput, Txid},
        wallet::{AddressNetwork, DescriptorRole, Wallet},
    };

    use super::WalletRescanRuntime;
    use crate::{
        FjallNodeStore, PersistMode, WalletRegistry, WalletRescanFreshness, WalletRescanJobState,
    };

    fn temp_store_path(test_name: &str) -> PathBuf {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time after unix epoch")
            .as_nanos();
        std::env::temp_dir().join(format!(
            "open-bitcoin-wallet-rescan-{test_name}-{}-{timestamp}",
            std::process::id()
        ))
    }

    fn remove_dir_if_exists(path: &Path) {
        match fs::remove_dir_all(path) {
            Ok(()) => {}
            Err(error) if error.kind() == io::ErrorKind::NotFound => {}
            Err(error) => panic!("failed to remove {}: {error}", path.display()),
        }
    }

    fn tip(height: u32) -> ChainPosition {
        ChainPosition::new(
            BlockHeader {
                version: 1,
                previous_block_hash: if height == 0 {
                    BlockHash::from_byte_array([0_u8; 32])
                } else {
                    BlockHash::from_byte_array([height as u8 - 1; 32])
                },
                merkle_root: Default::default(),
                time: 1_700_000_000 + height,
                bits: 0x207f_ffff,
                nonce: height,
            },
            height,
            u128::from(height) + 1,
            i64::from(1_700_000_000 + height),
        )
    }

    fn wallet_with_ranged_descriptor() -> Wallet {
        let mut wallet = Wallet::new(AddressNetwork::Regtest);
        wallet
            .import_descriptor(
                "receive-ranged",
                DescriptorRole::External,
                "wpkh(tprv8ZgxMBicQKsPd7Uf69XL1XwhmjHopUGep8GuEiJDZmbQz6o58LninorQAfcKZWARbtRtfnLcJ5MQ2AtHcQJCCRUcMRvmDUjyEmNUWwx8UbK/1/1/*)",
            )
            .expect("descriptor");
        wallet
    }

    fn funded_chainstate(wallet: &Wallet) -> ChainstateSnapshot {
        let receive_script = wallet
            .default_receive_address()
            .expect("receive")
            .script_pubkey;
        let mut utxos = HashMap::new();
        utxos.insert(
            OutPoint {
                txid: Txid::from_byte_array([1_u8; 32]),
                vout: 0,
            },
            Coin {
                output: TransactionOutput {
                    value: open_bitcoin_core::primitives::Amount::from_sats(25_000)
                        .expect("amount"),
                    script_pubkey: receive_script.clone(),
                },
                is_coinbase: false,
                created_height: 1,
                created_median_time_past: 1_700_000_001,
            },
        );
        utxos.insert(
            OutPoint {
                txid: Txid::from_byte_array([2_u8; 32]),
                vout: 1,
            },
            Coin {
                output: TransactionOutput {
                    value: open_bitcoin_core::primitives::Amount::from_sats(35_000)
                        .expect("amount"),
                    script_pubkey: receive_script,
                },
                is_coinbase: false,
                created_height: 3,
                created_median_time_past: 1_700_000_003,
            },
        );

        ChainstateSnapshot::new(
            vec![tip(0), tip(1), tip(2), tip(3)],
            utxos,
            Default::default(),
        )
    }

    #[test]
    fn restart_resume_advances_pending_rescan_in_bounded_chunks() {
        // Arrange
        let path = temp_store_path("resume-chunks");
        remove_dir_if_exists(&path);
        let store = FjallNodeStore::open(&path).expect("open store");
        let wallet = wallet_with_ranged_descriptor();
        store
            .save_chainstate_snapshot(&funded_chainstate(&wallet), PersistMode::Sync)
            .expect("save chainstate");
        let mut registry = WalletRegistry::default();
        registry
            .create_wallet(&store, "alpha", wallet, PersistMode::Sync)
            .expect("save wallet");

        {
            let runtime = WalletRescanRuntime::open_with_chunk_size(store, PersistMode::Sync, 2)
                .expect("runtime");
            let first_job = runtime.enqueue_rescan("alpha").expect("enqueue");
            assert_eq!(first_job.state, WalletRescanJobState::Scanning);
            assert_eq!(first_job.freshness, WalletRescanFreshness::Partial);
            assert_eq!(first_job.maybe_scanned_through_height, Some(1));
        }

        // Act
        let reopened_store = FjallNodeStore::open(&path).expect("reopen store");
        let reopened_runtime =
            WalletRescanRuntime::open_with_chunk_size(reopened_store, PersistMode::Sync, 2)
                .expect("reopened runtime");
        let resumed_job = reopened_runtime
            .store()
            .load_wallet_rescan_job("alpha")
            .expect("load job")
            .expect("job");
        let resumed_registry = WalletRegistry::load(reopened_runtime.store()).expect("registry");
        let resumed_wallet = resumed_registry
            .wallet_snapshot("alpha")
            .expect("wallet snapshot");

        // Assert
        assert_eq!(resumed_job.state, WalletRescanJobState::Complete);
        assert_eq!(resumed_job.freshness, WalletRescanFreshness::Fresh);
        assert_eq!(resumed_job.maybe_scanned_through_height, Some(3));
        assert_eq!(resumed_wallet.maybe_tip_height, Some(3));
        assert_eq!(resumed_wallet.utxos.len(), 2);

        remove_dir_if_exists(&path);
    }
}

#[cfg(test)]
mod tests;
