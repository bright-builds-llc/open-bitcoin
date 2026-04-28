// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    fs, io,
    path::{Path, PathBuf},
    rc::Rc,
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

use open_bitcoin_network::{HeaderEntry, HeadersMessage, VersionMessage, WireNetworkMessage};
use open_bitcoin_node::{
    LogStatus, MetricKind, MetricRetentionPolicy, MetricSample, MetricsStatus,
    MetricsStorageSnapshot, PersistMode, RuntimeMetadata, SyncNetwork, SyncPeerAddress,
    SyncPeerSession, SyncRuntimeConfig, SyncRuntimeError, SyncTransport,
    core::{
        chainstate::{ChainPosition, ChainstateSnapshot, Coin},
        consensus::{block_hash, block_merkle_root, check_block_header},
        primitives::{
            Amount, Block, BlockHash, BlockHeader, MerkleRoot, OutPoint, ScriptBuf, ScriptWitness,
            Transaction, TransactionInput, TransactionOutput, Txid,
        },
    },
    status::{
        BuildProvenance, ChainTipStatus, ConfigStatus, FieldAvailability, HealthSignal,
        HealthSignalLevel, MempoolStatus, NodeRuntimeState, NodeStatus, OpenBitcoinStatusSnapshot,
        PeerCounts, PeerStatus, ServiceStatus, SyncProgress, SyncStatus, WalletFreshness,
        WalletScanProgress, WalletStatus,
    },
};
use open_bitcoin_wallet::{AddressNetwork, DescriptorRole, Wallet};

use crate::error::BenchError;

const EASY_BITS: u32 = 0x207f_ffff;

#[derive(Debug)]
pub(crate) struct TempStoreDir {
    path: PathBuf,
}

impl TempStoreDir {
    pub(crate) fn new(label: &str) -> Result<Self, BenchError> {
        static NEXT_TEMP_ID: AtomicU64 = AtomicU64::new(1);
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
        let temp_id = NEXT_TEMP_ID.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "open-bitcoin-bench-{label}-{}-{timestamp}-{temp_id}",
            std::process::id(),
        ));
        remove_dir_if_exists(&path)?;
        Ok(Self { path })
    }

    pub(crate) fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TempStoreDir {
    fn drop(&mut self) {
        let _ = remove_dir_if_exists(&self.path);
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ScriptedTransport {
    scripts: VecDeque<Vec<WireNetworkMessage>>,
    sent: Rc<RefCell<Vec<WireNetworkMessage>>>,
}

#[derive(Debug, Clone)]
pub(crate) struct ScriptedSession {
    inbound: VecDeque<WireNetworkMessage>,
    sent: Rc<RefCell<Vec<WireNetworkMessage>>>,
}

impl ScriptedTransport {
    pub(crate) fn new(scripts: Vec<Vec<WireNetworkMessage>>) -> Self {
        Self {
            scripts: scripts.into(),
            sent: Rc::new(RefCell::new(Vec::new())),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn sent_messages(&self) -> Vec<WireNetworkMessage> {
        self.sent.borrow().clone()
    }
}

impl SyncTransport for ScriptedTransport {
    type Session = ScriptedSession;

    fn connect(
        &mut self,
        _peer: &SyncPeerAddress,
        _config: &SyncRuntimeConfig,
    ) -> Result<Self::Session, SyncRuntimeError> {
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
        _magic: open_bitcoin_node::core::primitives::NetworkMagic,
    ) -> Result<(), SyncRuntimeError> {
        self.sent.borrow_mut().push(message.clone());
        Ok(())
    }

    fn receive(
        &mut self,
        _magic: open_bitcoin_node::core::primitives::NetworkMagic,
    ) -> Result<Option<WireNetworkMessage>, SyncRuntimeError> {
        Ok(self.inbound.pop_front())
    }
}

pub(crate) fn sync_config() -> SyncRuntimeConfig {
    SyncRuntimeConfig {
        network: SyncNetwork::Regtest,
        manual_peers: vec![SyncPeerAddress::manual("127.0.0.1", 18_444)],
        dns_seeds: Vec::new(),
        max_messages_per_peer: 16,
        persist_mode: PersistMode::Sync,
        ..SyncRuntimeConfig::default()
    }
}

pub(crate) fn headers_script(
    start_height: i32,
    headers: Vec<BlockHeader>,
) -> Vec<WireNetworkMessage> {
    vec![
        WireNetworkMessage::Version(VersionMessage {
            start_height,
            ..VersionMessage::default()
        }),
        WireNetworkMessage::Verack,
        WireNetworkMessage::Headers(HeadersMessage { headers }),
    ]
}

pub(crate) fn header(previous_block_hash: BlockHash, nonce: u32) -> BlockHeader {
    BlockHeader {
        version: 1,
        previous_block_hash,
        merkle_root: MerkleRoot::from_byte_array([nonce as u8; 32]),
        time: 1_700_000_000 + nonce,
        bits: EASY_BITS,
        nonce,
    }
}

pub(crate) fn sample_header_entries() -> Vec<HeaderEntry> {
    let genesis_header = header(BlockHash::from_byte_array([0_u8; 32]), 1);
    let genesis_hash = block_hash(&genesis_header);
    let child_header = header(genesis_hash, 2);
    let child_hash = block_hash(&child_header);

    vec![
        HeaderEntry {
            block_hash: genesis_hash,
            header: genesis_header,
            height: 0,
            chain_work: 1,
        },
        HeaderEntry {
            block_hash: child_hash,
            header: child_header,
            height: 1,
            chain_work: 2,
        },
    ]
}

pub(crate) fn build_block(
    previous_block_hash: BlockHash,
    height: u32,
) -> Result<Block, BenchError> {
    let transactions = vec![coinbase_transaction(height, 50)?];
    let (merkle_root, maybe_mutated) = block_merkle_root(&transactions)
        .map_err(|error| BenchError::case_failed("runtime-fixture", error.to_string()))?;
    if maybe_mutated {
        return Err(BenchError::case_failed(
            "runtime-fixture",
            "fixture block produced a mutated merkle root",
        ));
    }

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
    mine_header(&mut block)?;
    Ok(block)
}

pub(crate) fn sample_runtime_metadata() -> RuntimeMetadata {
    RuntimeMetadata {
        last_clean_shutdown: true,
        ..RuntimeMetadata::default()
    }
}

pub(crate) fn sample_metrics_storage_snapshot() -> MetricsStorageSnapshot {
    MetricsStorageSnapshot {
        samples: vec![
            MetricSample::new(MetricKind::SyncHeight, 3.0, 1_700_000_003),
            MetricSample::new(MetricKind::PeerCount, 8.0, 1_700_000_004),
            MetricSample::new(MetricKind::DiskUsageBytes, 4_096.0, 1_700_000_005),
        ],
    }
}

pub(crate) fn sample_status_snapshot() -> OpenBitcoinStatusSnapshot {
    let retention = MetricRetentionPolicy {
        sample_interval_seconds: 30,
        max_samples_per_series: 60,
        max_age_seconds: 1_800,
    };
    let samples = vec![
        MetricSample::new(MetricKind::SyncHeight, 100.0, 1_700_000_000),
        MetricSample::new(MetricKind::PeerCount, 8.0, 1_700_000_030),
        MetricSample::new(MetricKind::MempoolTransactions, 12.0, 1_700_000_060),
        MetricSample::new(MetricKind::DiskUsageBytes, 8_192.0, 1_700_000_090),
        MetricSample::new(MetricKind::RpcHealth, 1.0, 1_700_000_120),
    ];

    OpenBitcoinStatusSnapshot {
        node: NodeStatus {
            state: NodeRuntimeState::Running,
            version: "0.1.0".to_string(),
        },
        config: ConfigStatus {
            datadir: FieldAvailability::available("/tmp/open-bitcoin".to_string()),
            config_paths: vec![
                "/tmp/open-bitcoin/bitcoin.conf".to_string(),
                "/tmp/open-bitcoin/open-bitcoin.jsonc".to_string(),
            ],
        },
        service: ServiceStatus {
            manager: FieldAvailability::available("launchd".to_string()),
            installed: FieldAvailability::available(true),
            enabled: FieldAvailability::available(true),
            running: FieldAvailability::available(true),
        },
        sync: SyncStatus {
            network: FieldAvailability::available("regtest".to_string()),
            chain_tip: FieldAvailability::available(ChainTipStatus {
                height: 100,
                block_hash: "0000000000000000000000000000000000000000000000000000000000000001"
                    .to_string(),
            }),
            sync_progress: FieldAvailability::available(SyncProgress {
                header_height: 100,
                block_height: 100,
                progress_ratio: 1.0,
                messages_processed: 24,
                headers_received: 4,
                blocks_received: 4,
            }),
        },
        peers: PeerStatus {
            peer_counts: FieldAvailability::available(PeerCounts {
                inbound: 0,
                outbound: 8,
            }),
        },
        mempool: MempoolStatus {
            transactions: FieldAvailability::available(12),
        },
        wallet: WalletStatus {
            trusted_balance_sats: FieldAvailability::available(60_000),
            freshness: FieldAvailability::available(WalletFreshness::Scanning),
            scan_progress: FieldAvailability::available(WalletScanProgress {
                scanned_through_height: 80,
                target_tip_height: 100,
            }),
        },
        logs: LogStatus::default(),
        metrics: MetricsStatus::available_with_samples(retention, samples),
        health_signals: vec![HealthSignal {
            level: HealthSignalLevel::Info,
            source: "status".to_string(),
            message: "node healthy".to_string(),
        }],
        build: BuildProvenance::unavailable(),
    }
}

pub(crate) fn wallet_with_ranged_descriptor() -> Result<Wallet, BenchError> {
    let mut wallet = Wallet::new(AddressNetwork::Regtest);
    wallet
        .import_descriptor(
            "receive-ranged",
            DescriptorRole::External,
            "wpkh(tprv8ZgxMBicQKsPd7Uf69XL1XwhmjHopUGep8GuEiJDZmbQz6o58LninorQAfcKZWARbtRtfnLcJ5MQ2AtHcQJCCRUcMRvmDUjyEmNUWwx8UbK/1/1/*)",
        )
        .map_err(|error| BenchError::case_failed("runtime-fixture", error.to_string()))?;
    Ok(wallet)
}

pub(crate) fn funded_chainstate(wallet: &Wallet) -> Result<ChainstateSnapshot, BenchError> {
    let receive_script = wallet
        .default_receive_address()
        .map_err(|error| BenchError::case_failed("runtime-fixture", error.to_string()))?
        .script_pubkey;
    let mut utxos = HashMap::new();
    utxos.insert(
        OutPoint {
            txid: Txid::from_byte_array([1_u8; 32]),
            vout: 0,
        },
        Coin {
            output: TransactionOutput {
                value: Amount::from_sats(25_000).map_err(|error| {
                    BenchError::case_failed("runtime-fixture", error.to_string())
                })?,
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
                value: Amount::from_sats(35_000).map_err(|error| {
                    BenchError::case_failed("runtime-fixture", error.to_string())
                })?,
                script_pubkey: receive_script,
            },
            is_coinbase: false,
            created_height: 3,
            created_median_time_past: 1_700_000_003,
        },
    );

    Ok(ChainstateSnapshot::new(
        vec![tip(0), tip(1), tip(2), tip(3)],
        utxos,
        Default::default(),
    ))
}

fn remove_dir_if_exists(path: &Path) -> io::Result<()> {
    match fs::remove_dir_all(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error),
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
            bits: EASY_BITS,
            nonce: height,
        },
        height,
        u128::from(height) + 1,
        i64::from(1_700_000_000 + height),
    )
}

fn coinbase_transaction(height: u32, value: i64) -> Result<Transaction, BenchError> {
    let mut script_sig = serialized_script_num(i64::from(height));
    script_sig.push(0x51);
    Ok(Transaction {
        version: 1,
        inputs: vec![TransactionInput {
            previous_output: OutPoint::null(),
            script_sig: ScriptBuf::from_bytes(script_sig)
                .map_err(|error| BenchError::case_failed("runtime-fixture", error.to_string()))?,
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(value)
                .map_err(|error| BenchError::case_failed("runtime-fixture", error.to_string()))?,
            script_pubkey: ScriptBuf::from_bytes(vec![0x51])
                .map_err(|error| BenchError::case_failed("runtime-fixture", error.to_string()))?,
        }],
        lock_time: 0,
    })
}

fn mine_header(block: &mut Block) -> Result<(), BenchError> {
    for nonce in 0..=u32::MAX {
        block.header.nonce = nonce;
        if check_block_header(&block.header).is_ok() {
            return Ok(());
        }
    }

    Err(BenchError::case_failed(
        "runtime-fixture",
        "could not mine fixture block at easy target",
    ))
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
    script
}
