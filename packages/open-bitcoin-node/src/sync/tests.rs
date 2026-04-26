// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use std::{
    cell::RefCell,
    collections::VecDeque,
    fs, io,
    path::{Path, PathBuf},
    rc::Rc,
    time::{SystemTime, UNIX_EPOCH},
};

use open_bitcoin_core::{
    consensus::{block_hash, block_merkle_root, check_block_header},
    primitives::{
        Amount, Block, BlockHash, BlockHeader, MerkleRoot, OutPoint, ScriptBuf, ScriptWitness,
        Transaction, TransactionInput, TransactionOutput,
    },
};
use open_bitcoin_network::{HeadersMessage, InventoryList, VersionMessage, WireNetworkMessage};

use super::{
    DurableSyncRuntime, PeerSyncOutcome, PeerSyncState, SyncNetwork, SyncPeerAddress,
    SyncPeerSession, SyncRunSummary, SyncRuntimeConfig, SyncRuntimeError, SyncTransport,
    TcpPeerTransport,
};
use crate::{
    FieldAvailability, FjallNodeStore, MetricKind, MetricSample, PersistMode, StorageError,
    StorageNamespace,
    logging::StructuredLogLevel,
    status::{HealthSignal, HealthSignalLevel, SyncProgress},
};

const EASY_BITS: u32 = 0x207f_ffff;

#[derive(Debug, Clone)]
struct ScriptedTransport {
    scripts: VecDeque<Vec<WireNetworkMessage>>,
    sent: Rc<RefCell<Vec<WireNetworkMessage>>>,
    fail_connect: bool,
}

impl ScriptedTransport {
    fn new(scripts: Vec<Vec<WireNetworkMessage>>) -> Self {
        Self {
            scripts: scripts.into(),
            sent: Rc::new(RefCell::new(Vec::new())),
            fail_connect: false,
        }
    }

    fn failing() -> Self {
        Self {
            scripts: VecDeque::new(),
            sent: Rc::new(RefCell::new(Vec::new())),
            fail_connect: true,
        }
    }

    fn sent_messages(&self) -> Vec<WireNetworkMessage> {
        self.sent.borrow().clone()
    }
}

#[derive(Debug, Clone)]
struct ScriptedSession {
    inbound: VecDeque<WireNetworkMessage>,
    sent: Rc<RefCell<Vec<WireNetworkMessage>>>,
}

impl SyncTransport for ScriptedTransport {
    type Session = ScriptedSession;

    fn connect(
        &mut self,
        peer: &SyncPeerAddress,
        _config: &SyncRuntimeConfig,
    ) -> Result<Self::Session, SyncRuntimeError> {
        if self.fail_connect {
            return Err(SyncRuntimeError::Io {
                peer: format!("{}:{}", peer.host, peer.port),
                message: "scripted connect failure".to_string(),
            });
        }

        Ok(ScriptedSession {
            inbound: self.scripts.pop_front().unwrap_or_default().into(),
            sent: Rc::clone(&self.sent),
        })
    }
}

impl SyncPeerSession for ScriptedSession {
    fn send(
        &mut self,
        message: &WireNetworkMessage,
        _magic: open_bitcoin_core::primitives::NetworkMagic,
    ) -> Result<(), SyncRuntimeError> {
        self.sent.borrow_mut().push(message.clone());
        Ok(())
    }

    fn receive(
        &mut self,
        _magic: open_bitcoin_core::primitives::NetworkMagic,
    ) -> Result<Option<WireNetworkMessage>, SyncRuntimeError> {
        Ok(self.inbound.pop_front())
    }
}

fn temp_store_path(test_name: &str) -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time after unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "open-bitcoin-sync-{test_name}-{}-{timestamp}",
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

fn sync_config() -> SyncRuntimeConfig {
    SyncRuntimeConfig {
        network: SyncNetwork::Regtest,
        manual_peers: vec![SyncPeerAddress::manual("127.0.0.1", 18_444)],
        dns_seeds: Vec::new(),
        max_messages_per_peer: 16,
        persist_mode: PersistMode::Sync,
        ..SyncRuntimeConfig::default()
    }
}

fn script(bytes: &[u8]) -> ScriptBuf {
    ScriptBuf::from_bytes(bytes.to_vec()).expect("valid script")
}

fn serialized_script_num(value: i64) -> Vec<u8> {
    if value == 0 {
        return vec![0x00];
    }

    let mut magnitude = value as u64;
    let mut encoded = Vec::new();
    while magnitude > 0 {
        encoded.push((magnitude & 0xff) as u8);
        magnitude >>= 8;
    }

    let mut script = Vec::with_capacity(encoded.len() + 2);
    script.push(encoded.len() as u8);
    script.extend(encoded);
    script.push(0x51);
    script
}

fn coinbase_transaction(height: u32, value: i64) -> Transaction {
    let mut script_sig = serialized_script_num(i64::from(height));
    script_sig.push(0x51);
    Transaction {
        version: 1,
        inputs: vec![TransactionInput {
            previous_output: OutPoint::null(),
            script_sig: script(&script_sig),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(value).expect("valid amount"),
            script_pubkey: script(&[0x51]),
        }],
        lock_time: 0,
    }
}

fn mine_header(block: &mut Block) {
    block.header.nonce = (0..=u32::MAX)
        .find(|nonce| {
            block.header.nonce = *nonce;
            check_block_header(&block.header).is_ok()
        })
        .expect("expected nonce at easy target");
}

fn build_block(previous_block_hash: BlockHash, height: u32) -> Block {
    let transactions = vec![coinbase_transaction(height, 50)];
    let (merkle_root, maybe_mutated) = block_merkle_root(&transactions).expect("merkle root");
    assert!(!maybe_mutated);

    let mut block = Block {
        header: BlockHeader {
            version: 1,
            previous_block_hash,
            merkle_root,
            time: 1_231_006_500 + height,
            bits: EASY_BITS,
            nonce: 0,
        },
        transactions,
    };
    mine_header(&mut block);
    block
}

fn header(previous_block_hash: BlockHash, nonce: u32) -> BlockHeader {
    let mut header = BlockHeader {
        version: 1,
        previous_block_hash,
        merkle_root: MerkleRoot::from_byte_array([nonce as u8; 32]),
        time: 1_231_006_500 + nonce,
        bits: EASY_BITS,
        nonce,
    };
    let nonce = (0..=u32::MAX)
        .find(|candidate| {
            header.nonce = *candidate;
            check_block_header(&header).is_ok()
        })
        .expect("expected nonce at easy target");
    header.nonce = nonce;
    header
}

#[test]
fn sync_summary_projects_metric_samples() {
    // Arrange
    let summary = SyncRunSummary {
        attempted_peers: 2,
        connected_peers: 1,
        failed_peers: 1,
        messages_processed: 7,
        headers_received: 3,
        blocks_received: 2,
        best_header_height: 42,
        best_block_height: 40,
        peer_outcomes: Vec::new(),
        health_signals: Vec::new(),
    };

    // Act
    let samples = summary.metric_samples(1_777_225_022);

    // Assert
    assert_eq!(
        samples,
        vec![
            MetricSample::new(MetricKind::HeaderHeight, 42.0, 1_777_225_022),
            MetricSample::new(MetricKind::SyncHeight, 40.0, 1_777_225_022),
            MetricSample::new(MetricKind::PeerCount, 1.0, 1_777_225_022),
        ]
    );
}

#[test]
fn sync_summary_projects_structured_log_records() {
    // Arrange
    let summary = SyncRunSummary {
        attempted_peers: 3,
        connected_peers: 2,
        failed_peers: 1,
        messages_processed: 9,
        headers_received: 4,
        blocks_received: 2,
        best_header_height: 44,
        best_block_height: 43,
        peer_outcomes: vec![
            PeerSyncOutcome {
                peer: SyncPeerAddress::manual("127.0.0.1", 18_444),
                state: PeerSyncState::Stalled,
                attempts: 1,
                maybe_error: None,
            },
            PeerSyncOutcome {
                peer: SyncPeerAddress::manual("203.0.113.10", 18_444),
                state: PeerSyncState::Failed,
                attempts: 3,
                maybe_error: Some("scripted network failure".to_string()),
            },
            PeerSyncOutcome {
                peer: SyncPeerAddress::manual("198.51.100.9", 18_444),
                state: PeerSyncState::Connected,
                attempts: 2,
                maybe_error: None,
            },
        ],
        health_signals: vec![
            HealthSignal {
                level: HealthSignalLevel::Warn,
                source: "sync".to_string(),
                message: "headers stalled".to_string(),
            },
            HealthSignal {
                level: HealthSignalLevel::Error,
                source: "storage".to_string(),
                message: "metrics persistence unavailable".to_string(),
            },
        ],
    };

    // Act
    let records = summary.structured_log_records(1_777_225_099);

    // Assert
    let summary_record = records
        .iter()
        .find(|record| {
            record.level == StructuredLogLevel::Info
                && record.source == "sync"
                && record.message.contains("messages_processed=9")
        })
        .expect("sync summary log record");
    assert!(summary_record.message.contains("headers_received=4"));
    assert!(summary_record.message.contains("blocks_received=2"));
    assert!(summary_record.message.contains("best_header_height=44"));
    assert!(summary_record.message.contains("best_block_height=43"));
    assert!(records.iter().any(|record| {
        record.level == StructuredLogLevel::Warn
            && record.source == "sync"
            && record.message.contains("peer stalled")
    }));
    assert!(records.iter().any(|record| {
        record.level == StructuredLogLevel::Error
            && record.source == "sync"
            && record.message.contains("peer failed")
    }));
    assert!(records.iter().any(|record| {
        record.level == StructuredLogLevel::Warn
            && record.source == "sync"
            && record.message.contains("retry attempts=2")
    }));
    assert!(records.iter().any(|record| {
        record.level == StructuredLogLevel::Error
            && record.source == "storage"
            && record.message == "metrics persistence unavailable"
    }));
    assert!(records.iter().all(|record| record.message.len() <= 160));
    assert!(records.iter().all(|record| {
        !record.message.contains("127.0.0.1")
            && !record.message.contains("203.0.113")
            && !record.message.contains("cookie")
            && !record.message.contains("/tmp/")
    }));
}

#[test]
fn sync_summary_status_projections_include_counters() {
    // Arrange
    let summary = SyncRunSummary {
        attempted_peers: 4,
        connected_peers: 3,
        failed_peers: 1,
        messages_processed: 12,
        headers_received: 7,
        blocks_received: 5,
        best_header_height: 100,
        best_block_height: 25,
        peer_outcomes: Vec::new(),
        health_signals: Vec::new(),
    };

    // Act
    let sync_status = summary.sync_status(SyncNetwork::Regtest);
    let peer_status = summary.peer_status();

    // Assert
    assert_eq!(
        sync_status.sync_progress,
        FieldAvailability::available(SyncProgress {
            header_height: 100,
            block_height: 25,
            progress_ratio: 0.25,
            messages_processed: 12,
            headers_received: 7,
            blocks_received: 5,
        })
    );
    assert_eq!(
        peer_status.peer_counts,
        FieldAvailability::available(crate::status::PeerCounts {
            inbound: 0,
            outbound: 3,
        })
    );
}

#[test]
fn sync_runtime_errors_project_storage_and_network_health_signals() {
    // Arrange
    let network_error = SyncRuntimeError::Network {
        message: "connection reset".to_string(),
    };
    let storage_error = SyncRuntimeError::Storage(StorageError::UnavailableNamespace {
        namespace: StorageNamespace::Metrics,
    });

    // Act
    let network_signal = network_error.health_signal();
    let storage_signal = storage_error.health_signal();

    // Assert
    assert_eq!(network_signal.level, HealthSignalLevel::Error);
    assert_eq!(network_signal.source, "network");
    assert!(network_signal.message.contains("sync network failure"));
    assert_eq!(storage_signal.level, HealthSignalLevel::Error);
    assert_eq!(storage_signal.source, "storage");
    assert!(
        storage_signal
            .message
            .contains("storage namespace unavailable")
    );
    assert!(network_signal.message.len() <= 160);
    assert!(storage_signal.message.len() <= 160);
}

#[test]
fn scripted_headers_sync_persists_progress_and_status() {
    // Arrange
    let path = temp_store_path("headers");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let genesis = header(BlockHash::from_byte_array([0_u8; 32]), 1);
    let child = header(block_hash(&genesis), 2);
    let script = vec![
        WireNetworkMessage::Version(VersionMessage {
            start_height: 1,
            ..VersionMessage::default()
        }),
        WireNetworkMessage::Verack,
        WireNetworkMessage::Headers(HeadersMessage {
            headers: vec![genesis.clone(), child.clone()],
        }),
    ];
    let mut transport = ScriptedTransport::new(vec![script]);
    let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");

    // Act
    let summary = runtime
        .sync_once(&mut transport, 1_777_225_022)
        .expect("sync");

    // Assert
    assert_eq!(summary.connected_peers, 1);
    assert_eq!(summary.headers_received, 2);
    assert_eq!(summary.best_header_height, 1);
    assert_eq!(summary.best_block_height, 0);
    assert_eq!(
        summary
            .sync_status(SyncNetwork::Regtest)
            .sync_progress
            .clone(),
        crate::FieldAvailability::available(SyncProgress {
            header_height: 1,
            block_height: 0,
            progress_ratio: 0.0,
            messages_processed: 3,
            headers_received: 2,
            blocks_received: 0,
        })
    );
    assert_eq!(
        runtime
            .store()
            .load_header_entries()
            .expect("load headers")
            .expect("headers")
            .entries
            .len(),
        2
    );
    assert!(
        transport
            .sent_messages()
            .iter()
            .any(|message| { matches!(message, WireNetworkMessage::GetHeaders { .. }) })
    );
    assert!(transport.sent_messages().iter().any(|message| {
        matches!(message, WireNetworkMessage::GetData(InventoryList { inventory }) if inventory.len() == 2)
    }));

    remove_dir_if_exists(&path);
}

#[test]
fn scripted_block_download_connects_and_persists_block() {
    // Arrange
    let path = temp_store_path("block");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let genesis_hash = block_hash(&genesis.header);
    let script = vec![
        WireNetworkMessage::Version(VersionMessage {
            start_height: 0,
            ..VersionMessage::default()
        }),
        WireNetworkMessage::Verack,
        WireNetworkMessage::Headers(HeadersMessage {
            headers: vec![genesis.header.clone()],
        }),
        WireNetworkMessage::Block(genesis.clone()),
    ];
    let mut transport = ScriptedTransport::new(vec![script]);
    let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");

    // Act
    let summary = runtime
        .sync_once(&mut transport, i64::from(genesis.header.time))
        .expect("sync");

    // Assert
    assert_eq!(summary.blocks_received, 1);
    assert_eq!(summary.best_block_height, 0);
    assert_eq!(
        runtime
            .store()
            .load_block(genesis_hash)
            .expect("load block"),
        Some(genesis)
    );
    assert_eq!(
        runtime
            .store()
            .load_chainstate_snapshot()
            .expect("load chainstate")
            .expect("chainstate")
            .tip()
            .expect("tip")
            .height,
        0
    );
    let metrics = runtime
        .store()
        .load_metrics_snapshot()
        .expect("metrics")
        .expect("metrics");
    assert!(
        metrics
            .samples
            .iter()
            .any(|sample| sample.kind == MetricKind::SyncHeight)
    );

    remove_dir_if_exists(&path);
}

#[test]
fn runtime_seeds_headers_from_durable_store_on_restart() {
    // Arrange
    let path = temp_store_path("resume");
    remove_dir_if_exists(&path);
    let genesis = header(BlockHash::from_byte_array([0_u8; 32]), 11);
    let child = header(block_hash(&genesis), 12);
    {
        let store = FjallNodeStore::open(&path).expect("store");
        store
            .save_header_entries(
                &[
                    open_bitcoin_network::HeaderEntry {
                        block_hash: block_hash(&genesis),
                        header: genesis.clone(),
                        height: 0,
                        chain_work: 1,
                    },
                    open_bitcoin_network::HeaderEntry {
                        block_hash: block_hash(&child),
                        header: child,
                        height: 1,
                        chain_work: 2,
                    },
                ],
                PersistMode::Sync,
            )
            .expect("save headers");
    }

    // Act
    let store = FjallNodeStore::open(&path).expect("reopen store");
    let runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");

    // Assert
    assert_eq!(runtime.snapshot_summary().best_header_height, 1);

    remove_dir_if_exists(&path);
}

#[test]
fn no_configured_peers_is_a_typed_error() {
    // Arrange
    let path = temp_store_path("no-peers");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(
        store,
        SyncRuntimeConfig {
            manual_peers: Vec::new(),
            dns_seeds: Vec::new(),
            ..sync_config()
        },
    )
    .expect("runtime");
    let mut transport = ScriptedTransport::new(vec![]);

    // Act
    let error = runtime
        .sync_once(&mut transport, 1)
        .expect_err("no peers configured");

    // Assert
    assert_eq!(error, SyncRuntimeError::NoPeersConfigured);

    remove_dir_if_exists(&path);
}

#[test]
fn connect_failures_are_reported_as_peer_outcomes() {
    // Arrange
    let path = temp_store_path("connect-failure");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(
        store,
        SyncRuntimeConfig {
            max_peer_retries: 0,
            ..sync_config()
        },
    )
    .expect("runtime");
    let mut transport = ScriptedTransport::failing();

    // Act
    let summary = runtime.sync_once(&mut transport, 1).expect("summary");

    // Assert
    assert_eq!(summary.failed_peers, 1);
    assert_eq!(summary.peer_outcomes[0].state, PeerSyncState::Failed);
    assert!(summary.peer_outcomes[0].maybe_error.is_some());
    assert_eq!(summary.health_signals.len(), 1);

    remove_dir_if_exists(&path);
}

#[test]
fn sync_networks_select_matching_consensus_pow_rules() {
    // Arrange
    let mainnet = SyncNetwork::Mainnet.consensus_params();
    let testnet = SyncNetwork::Testnet.consensus_params();
    let signet = SyncNetwork::Signet.consensus_params();
    let regtest = SyncNetwork::Regtest.consensus_params();

    // Act / Assert
    assert_eq!(mainnet.pow_limit_bits, 0x1d00_ffff);
    assert!(!mainnet.allow_min_difficulty_blocks);
    assert!(!mainnet.no_pow_retargeting);
    assert_eq!(testnet.pow_limit_bits, 0x1d00_ffff);
    assert!(testnet.allow_min_difficulty_blocks);
    assert!(!testnet.no_pow_retargeting);
    assert_eq!(signet.pow_limit_bits, 0x1e03_77ae);
    assert!(!signet.allow_min_difficulty_blocks);
    assert_eq!(regtest.pow_limit_bits, EASY_BITS);
    assert!(regtest.allow_min_difficulty_blocks);
    assert!(regtest.no_pow_retargeting);
}

#[test]
#[ignore = "requires public Bitcoin network; set OPEN_BITCOIN_LIVE_SYNC_SMOKE=1 to run"]
fn live_network_smoke_is_explicitly_opt_in() {
    if std::env::var("OPEN_BITCOIN_LIVE_SYNC_SMOKE")
        .ok()
        .as_deref()
        != Some("1")
    {
        return;
    }

    let path = temp_store_path("live");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(
        store,
        SyncRuntimeConfig {
            manual_peers: vec![SyncPeerAddress::dns_seed("seed.bitcoin.sipa.be", 8333)],
            dns_seeds: Vec::new(),
            max_messages_per_peer: 2,
            ..SyncRuntimeConfig::default()
        },
    )
    .expect("runtime");
    let mut transport = TcpPeerTransport;

    let _summary = runtime
        .sync_once(&mut transport, 1_777_225_022)
        .expect("live sync smoke");

    remove_dir_if_exists(&path);
}
