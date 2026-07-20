// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use std::{
    cell::RefCell,
    collections::VecDeque,
    fs, io,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::{Path, PathBuf},
    rc::Rc,
    sync::{Arc, Mutex},
    time::{SystemTime, UNIX_EPOCH},
};

use open_bitcoin_core::{
    chainstate::{ChainPosition, ChainstateSnapshot},
    consensus::{block_hash, block_merkle_root, check_block_header},
    primitives::{
        Amount, Block, BlockHash, BlockHeader, InventoryType, InventoryVector, MerkleRoot,
        OutPoint, ScriptBuf, ScriptWitness, Transaction, TransactionInput, TransactionOutput,
    },
};
use open_bitcoin_network::{
    HeaderEntry, HeadersMessage, InventoryList, PeerId, VersionMessage, WireNetworkMessage,
};

use super::types::SyncReconcileProgress;
use super::{
    DurableSyncRuntime, PeerContribution, PeerFailureReason, PeerSyncOutcome, PeerSyncState,
    ResolvedSyncPeerAddress, SyncNetwork, SyncPeerAddress, SyncPeerReceiveOutcome,
    SyncPeerResolver, SyncPeerSession, SyncPeerSource, SyncRunSummary, SyncRuntimeConfig,
    SyncRuntimeError, SyncStopReason, SyncTransport, TcpPeerTransport,
};
use crate::{
    FieldAvailability, FjallNodeStore, LogRetentionPolicy, MetricKind, MetricRetentionPolicy,
    MetricSample, PersistMode, RuntimeMetadata, StorageError, StorageNamespace,
    StorageRecoveryAction,
    logging::{
        BLOCK_RELAY_LOG_SOURCE, StructuredLogLevel, StructuredLogRecord, writer::load_log_status,
    },
    status::{
        BestKnownTipSource, BestKnownTipStatus, BlockRelayEvidenceStatus,
        BlockServingActivationEvidence, BlockServingEligibilityCounters,
        BlockServingEvidenceStatus, BlockServingStatusCounters, CompactRelayAnnouncementCounters,
        CompactRelayCleanupCounters, CompactRelayFallbackCounters, CompactRelayInFlightCounters,
        CompactRelayMissingTransactionCounters, CompactRelayNegotiationCounters,
        CompactRelayReconstructionCounters, DurableSyncState, HealthSignal, HealthSignalLevel,
        InboundHandshakeStatusCounts, InboundPeerServingStatus, NoProgressDiagnosis,
        PeerContributionEvidence, PeerContributionKind, PeerTipAgreement, PeerTipAgreementStatus,
        ProgressCreditEvidence, ProgressCreditKind, RejectedProgressActivityKind,
        StallDiagnosisEvidence, StalledSubsystem, StayCurrentStatus, SyncLifecycleState,
        SyncProgress, SyncProgressSignal, SyncReconcileProgressStatus, SyncRecoveryCategory,
        SyncReorgEvidence, SyncResourcePressure, SyncStatus, TipFreshnessStatus,
        inbound_status_unavailable,
    },
};

mod production_announcement_transport_cases;
mod runtime_projection_cases;
mod runtime_timing_cases;
mod runtime_write_evidence_cases;
mod soak;

const EASY_BITS: u32 = 0x207f_ffff;

#[derive(Debug, Clone)]
struct ScriptedTransport {
    scripts: VecDeque<Result<Vec<WireNetworkMessage>, SyncRuntimeError>>,
    sent: Rc<RefCell<Vec<WireNetworkMessage>>>,
    fail_connect: bool,
}

impl ScriptedTransport {
    fn new(scripts: Vec<Vec<WireNetworkMessage>>) -> Self {
        Self {
            scripts: scripts.into_iter().map(Ok).collect(),
            sent: Rc::new(RefCell::new(Vec::new())),
            fail_connect: false,
        }
    }

    fn with_connect_results(
        scripts: Vec<Result<Vec<WireNetworkMessage>, SyncRuntimeError>>,
    ) -> Self {
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

#[derive(Debug, Clone)]
struct ErrorAfterMessagesTransport {
    scripts: VecDeque<Vec<WireNetworkMessage>>,
    sent: Rc<RefCell<Vec<WireNetworkMessage>>>,
    error: SyncRuntimeError,
    errors_remaining: Rc<RefCell<usize>>,
}

#[derive(Debug, Clone)]
struct ErrorAfterMessagesSession {
    inbound: VecDeque<WireNetworkMessage>,
    sent: Rc<RefCell<Vec<WireNetworkMessage>>>,
    maybe_error: Option<SyncRuntimeError>,
}

#[derive(Debug, Clone)]
struct ScriptedResolver {
    results: VecDeque<Result<Vec<ResolvedSyncPeerAddress>, SyncRuntimeError>>,
}

impl ScriptedResolver {
    fn new(results: Vec<Result<Vec<ResolvedSyncPeerAddress>, SyncRuntimeError>>) -> Self {
        Self {
            results: results.into(),
        }
    }
}

impl SyncPeerResolver for ScriptedResolver {
    fn resolve(
        &mut self,
        peer: &SyncPeerAddress,
        _config: &SyncRuntimeConfig,
    ) -> Result<Vec<ResolvedSyncPeerAddress>, SyncRuntimeError> {
        self.results.pop_front().unwrap_or_else(|| {
            Ok(vec![ResolvedSyncPeerAddress::new(
                peer.clone(),
                SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), peer.port),
            )])
        })
    }
}

impl SyncTransport for ScriptedTransport {
    type Session = ScriptedSession;

    fn connect(
        &mut self,
        peer: &ResolvedSyncPeerAddress,
        _config: &SyncRuntimeConfig,
    ) -> Result<Self::Session, SyncRuntimeError> {
        if self.fail_connect {
            return Err(SyncRuntimeError::Io {
                peer: peer.label(),
                message: "scripted connect failure".to_string(),
            });
        }

        let inbound = self.scripts.pop_front().unwrap_or_else(|| Ok(Vec::new()))?;
        Ok(ScriptedSession {
            inbound: inbound.into(),
            sent: Rc::clone(&self.sent),
        })
    }
}

impl ErrorAfterMessagesTransport {
    fn new(
        scripts: Vec<Vec<WireNetworkMessage>>,
        error: SyncRuntimeError,
        errors_remaining: usize,
    ) -> Self {
        Self {
            scripts: scripts.into(),
            sent: Rc::new(RefCell::new(Vec::new())),
            error,
            errors_remaining: Rc::new(RefCell::new(errors_remaining)),
        }
    }

    fn sent_messages(&self) -> Vec<WireNetworkMessage> {
        self.sent.borrow().clone()
    }
}

impl SyncTransport for ErrorAfterMessagesTransport {
    type Session = ErrorAfterMessagesSession;

    fn connect(
        &mut self,
        _peer: &ResolvedSyncPeerAddress,
        _config: &SyncRuntimeConfig,
    ) -> Result<Self::Session, SyncRuntimeError> {
        let inbound = self.scripts.pop_front().unwrap_or_default();
        let mut errors_remaining = self.errors_remaining.borrow_mut();
        let maybe_error = if *errors_remaining == 0 {
            None
        } else {
            *errors_remaining -= 1;
            Some(self.error.clone())
        };
        Ok(ErrorAfterMessagesSession {
            inbound: inbound.into(),
            sent: Rc::clone(&self.sent),
            maybe_error,
        })
    }
}

fn resolved_manual_peer(host: &str, port: u16) -> ResolvedSyncPeerAddress {
    ResolvedSyncPeerAddress::new(
        SyncPeerAddress::manual(host, port),
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), port),
    )
}

fn peer_outcome(
    peer: SyncPeerAddress,
    state: PeerSyncState,
    attempts: u8,
    maybe_failure_reason: Option<PeerFailureReason>,
    maybe_error: Option<String>,
) -> PeerSyncOutcome {
    PeerSyncOutcome {
        maybe_resolved_endpoint: Some(format!("127.0.0.1:{}", peer.port)),
        network: SyncNetwork::Regtest,
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
        peer,
        state,
        attempts,
        maybe_failure_reason,
        maybe_error,
    }
}

fn peer_outcome_with_contribution(
    peer: SyncPeerAddress,
    state: PeerSyncState,
    attempts: u8,
    maybe_failure_reason: Option<PeerFailureReason>,
    contribution: PeerContribution,
) -> PeerSyncOutcome {
    let mut outcome = peer_outcome(peer, state, attempts, maybe_failure_reason, None);
    outcome.contribution = contribution;
    outcome
}

fn summary_with_peer_failure(reason: PeerFailureReason, error: &str) -> SyncRunSummary {
    let mut summary = SyncRunSummary::empty(0, 0, 1);
    summary.failed_peers = 1;
    summary.peer_outcomes.push(peer_outcome(
        SyncPeerAddress::manual("127.0.0.1", 18_444),
        PeerSyncState::Failed,
        1,
        Some(reason),
        Some(error.to_string()),
    ));
    summary
}

fn assert_no_progress_status(
    state: &DurableSyncState,
    diagnosis: NoProgressDiagnosis,
    next_action: &str,
) {
    assert_eq!(
        state.sync.no_progress_diagnosis,
        FieldAvailability::available(diagnosis)
    );
    assert_eq!(
        state.sync.no_progress_next_action,
        FieldAvailability::available(next_action.to_string())
    );
}

fn available_progress_credit(state: &DurableSyncState) -> &ProgressCreditEvidence {
    let FieldAvailability::Available(credit) = &state.sync.progress_credit else {
        panic!("progress credit should be available");
    };
    credit
}

fn available_last_useful_work(state: &DurableSyncState) -> &ProgressCreditEvidence {
    let FieldAvailability::Available(credit) = &state.sync.last_useful_work else {
        panic!("last useful work should be available");
    };
    credit
}

fn available_last_peer_contribution(state: &DurableSyncState) -> &PeerContributionEvidence {
    let FieldAvailability::Available(contribution) = &state.sync.last_peer_contribution else {
        panic!("last_peer_contribution should be available");
    };
    contribution
}

fn available_stall_diagnosis(state: &DurableSyncState) -> &StallDiagnosisEvidence {
    let FieldAvailability::Available(diagnosis) = &state.sync.stall_diagnosis else {
        panic!("stall diagnosis should be available");
    };
    diagnosis
}

fn assert_progress_credit_unavailable(state: &DurableSyncState) {
    assert!(matches!(
        state.sync.progress_credit,
        FieldAvailability::Unavailable { .. }
    ));
}

fn assert_rejected_activity(credit: &ProgressCreditEvidence, kind: RejectedProgressActivityKind) {
    assert!(
        credit
            .rejected_activity
            .iter()
            .any(|activity| activity.kind == kind),
        "missing rejected activity {kind:?} in {credit:?}"
    );
}

fn serialized_label<T>(value: T) -> String
where
    T: serde::Serialize,
{
    serde_json::to_value(value)
        .expect("status label serializes")
        .as_str()
        .expect("status label is a string")
        .to_string()
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
    ) -> Result<SyncPeerReceiveOutcome, SyncRuntimeError> {
        Ok(self.inbound.pop_front().map_or(
            SyncPeerReceiveOutcome::Closed,
            SyncPeerReceiveOutcome::Message,
        ))
    }
}

impl SyncPeerSession for ErrorAfterMessagesSession {
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
    ) -> Result<SyncPeerReceiveOutcome, SyncRuntimeError> {
        let maybe_message = self.inbound.pop_front();
        if let Some(message) = maybe_message {
            return Ok(SyncPeerReceiveOutcome::Message(message));
        }
        if let Some(error) = self.maybe_error.take() {
            return Err(error);
        }
        Ok(SyncPeerReceiveOutcome::Closed)
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

fn sync_config_with_log_dir(log_dir: &Path) -> SyncRuntimeConfig {
    SyncRuntimeConfig {
        maybe_log_dir: Some(log_dir.to_path_buf()),
        ..sync_config()
    }
}

fn two_peer_sync_config() -> SyncRuntimeConfig {
    SyncRuntimeConfig {
        manual_peers: vec![
            SyncPeerAddress::manual("127.0.0.1", 18_444),
            SyncPeerAddress::manual("127.0.0.1", 18_445),
        ],
        target_outbound_peers: 2,
        max_peer_retries: 0,
        ..sync_config()
    }
}

fn connect_runtime_peer(runtime: &mut DurableSyncRuntime, peer_id: PeerId, start_height: i32) {
    runtime
        .network
        .connect_outbound_peer(peer_id, 1_777_225_210)
        .expect("connect peer");
    runtime
        .network
        .receive_sync_message(
            peer_id,
            WireNetworkMessage::Version(VersionMessage {
                start_height,
                ..VersionMessage::default()
            }),
            1_777_225_210,
            runtime.verify_flags,
            runtime.consensus_params,
        )
        .expect("receive version");
    runtime
        .network
        .receive_sync_message(
            peer_id,
            WireNetworkMessage::Verack,
            1_777_225_210,
            runtime.verify_flags,
            runtime.consensus_params,
        )
        .expect("receive verack");
}

fn durable_tip_capture(runtime: &mut DurableSyncRuntime) -> Arc<Mutex<Vec<BlockHash>>> {
    let captured = Arc::new(Mutex::new(Vec::new()));
    let captured_for_sink = Arc::clone(&captured);
    runtime.set_durable_tip_announcement_sink(move |event| {
        captured_for_sink
            .lock()
            .expect("durable tip capture lock")
            .push(block_hash(&event.block().header));
        Ok(())
    });
    captured
}

#[test]
fn durable_tip_direct_sync_emits_only_final_durable_best_tip() {
    // Arrange
    let path = temp_store_path("durable-tip-direct-final-only");
    remove_dir_if_exists(&path);
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let child = build_block(block_hash(&genesis.header), 1);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    let captured = durable_tip_capture(&mut runtime);
    let mut transport = ScriptedTransport::new(vec![vec![
        WireNetworkMessage::Version(VersionMessage {
            start_height: 1,
            ..VersionMessage::default()
        }),
        WireNetworkMessage::Verack,
        WireNetworkMessage::Headers(HeadersMessage {
            headers: vec![genesis.header.clone(), child.header.clone()],
        }),
        WireNetworkMessage::Block(genesis),
        WireNetworkMessage::Block(child.clone()),
        WireNetworkMessage::Block(child.clone()),
    ]]);

    // Act
    runtime
        .sync_once(&mut transport, i64::from(child.header.time))
        .expect("direct durable tip sync");

    // Assert
    assert_eq!(
        *captured.lock().expect("durable tip capture lock"),
        vec![block_hash(&child.header)]
    );
    assert!(
        runtime
            .store()
            .load_block(block_hash(&child.header))
            .is_ok_and(|block| block.is_some())
    );
    remove_dir_if_exists(&path);
}

#[test]
fn durable_tip_live_reconcile_collapses_multiple_blocks_to_final_tip() {
    // Arrange
    let path = temp_store_path("durable-tip-reconcile-final-only");
    remove_dir_if_exists(&path);
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let child = build_block(block_hash(&genesis.header), 1);
    let grandchild = build_block(block_hash(&child.header), 2);
    save_chain_headers_snapshot_and_blocks(
        &path,
        &[(&genesis, 0), (&child, 1), (&grandchild, 2)],
        &[(&genesis, 0)],
        &[(&genesis, 0), (&child, 1), (&grandchild, 2)],
    );
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    let captured = durable_tip_capture(&mut runtime);

    // Act
    let progress = super::block_reconcile::reconcile_best_chain_for_live_session(
        &mut runtime,
        i64::from(grandchild.header.time),
    )
    .expect("live reconciliation");
    runtime
        .persist_progress_and_dispatch_tip()
        .expect("persist reconciled tip");

    // Assert
    assert_eq!(
        progress,
        SyncReconcileProgress::ExtendedActiveChain { connected_count: 2 }
    );
    assert_eq!(
        *captured.lock().expect("durable tip capture lock"),
        vec![block_hash(&grandchild.header)]
    );
    remove_dir_if_exists(&path);
}

fn version_verack_script(start_height: i32) -> Vec<WireNetworkMessage> {
    vec![
        WireNetworkMessage::Version(VersionMessage {
            start_height,
            ..VersionMessage::default()
        }),
        WireNetworkMessage::Verack,
    ]
}

fn headers_script(start_height: i32, headers: Vec<BlockHeader>) -> Vec<WireNetworkMessage> {
    vec![
        WireNetworkMessage::Version(VersionMessage {
            start_height,
            ..VersionMessage::default()
        }),
        WireNetworkMessage::Verack,
        WireNetworkMessage::Headers(HeadersMessage { headers }),
    ]
}

fn load_structured_log_records(log_dir: &Path) -> Vec<StructuredLogRecord> {
    let mut records = Vec::new();
    for entry in fs::read_dir(log_dir).expect("read log directory") {
        let path = entry.expect("read log entry").path();
        if !path.is_file() {
            continue;
        }
        let contents = fs::read_to_string(&path).expect("read structured log file");
        for line in contents.lines().filter(|line| !line.trim().is_empty()) {
            records.push(serde_json::from_str(line).expect("structured log record"));
        }
    }
    records
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

fn build_branch_block(previous_block_hash: BlockHash, height: u32, time_offset: u32) -> Block {
    let mut block = build_block(previous_block_hash, height);
    block.header.time = block.header.time.saturating_add(time_offset);
    mine_header(&mut block);
    block
}

fn getdata_block_hashes(messages: &[WireNetworkMessage]) -> Vec<BlockHash> {
    let mut hashes = Vec::new();
    for message in messages {
        let WireNetworkMessage::GetData(inventory) = message else {
            continue;
        };
        for item in &inventory.inventory {
            if matches!(
                item.inventory_type,
                InventoryType::Block | InventoryType::WitnessBlock
            ) {
                hashes.push(BlockHash::from(item.object_hash));
            }
        }
    }
    hashes
}

fn notfound_for_block(block_hash: BlockHash) -> WireNetworkMessage {
    WireNetworkMessage::NotFound(InventoryList::new(vec![InventoryVector {
        inventory_type: InventoryType::Block,
        object_hash: block_hash.into(),
    }]))
}

fn block_hash_hex(block_hash: BlockHash) -> String {
    encode_hex(block_hash.as_bytes())
}

fn encode_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        encoded.push(HEX[(byte >> 4) as usize] as char);
        encoded.push(HEX[(byte & 0x0f) as usize] as char);
    }
    encoded
}

fn save_best_chain_with_active_blocks(
    path: &Path,
    best_chain: &[(&Block, u32)],
    active_chain: &[(&Block, u32)],
) {
    save_chain_headers_snapshot_and_blocks(path, best_chain, active_chain, active_chain);
}

fn save_chain_headers_snapshot_and_blocks(
    path: &Path,
    best_chain: &[(&Block, u32)],
    active_chain: &[(&Block, u32)],
    stored_blocks: &[(&Block, u32)],
) {
    let store = FjallNodeStore::open(path).expect("store");
    let header_entries = best_chain
        .iter()
        .map(|(block, height)| HeaderEntry {
            block_hash: block_hash(&block.header),
            header: block.header.clone(),
            height: *height,
            chain_work: u128::from(*height).saturating_add(1),
        })
        .collect::<Vec<_>>();
    store
        .save_header_entries(&header_entries, PersistMode::Sync)
        .expect("save best-chain headers");
    let active_positions = active_chain
        .iter()
        .map(|(block, height)| {
            ChainPosition::new(
                block.header.clone(),
                *height,
                u128::from(*height).saturating_add(1),
                i64::from(block.header.time),
            )
        })
        .collect::<Vec<_>>();
    store
        .save_chainstate_snapshot(
            &ChainstateSnapshot::new(active_positions, Default::default(), Default::default()),
            PersistMode::Sync,
        )
        .expect("save active chain snapshot");
    for (block, _) in stored_blocks {
        store
            .save_block(block, PersistMode::Sync)
            .expect("save stored block");
    }
}

fn phase70_branch_blocks() -> (Block, Block, Block, Block, Block, Block) {
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let branch_a_one = build_block(block_hash(&genesis.header), 1);
    let branch_a_two = build_block(block_hash(&branch_a_one.header), 2);
    let branch_b_one = build_branch_block(block_hash(&genesis.header), 1, 100);
    let branch_b_two = build_branch_block(block_hash(&branch_b_one.header), 2, 100);
    let branch_b_three = build_branch_block(block_hash(&branch_b_two.header), 3, 100);

    (
        genesis,
        branch_a_one,
        branch_a_two,
        branch_b_one,
        branch_b_two,
        branch_b_three,
    )
}

fn phase70_save_reorg_ready_branch(path: &Path) -> (Block, Block, Block, Block, Block, Block) {
    let (genesis, branch_a_one, branch_a_two, branch_b_one, branch_b_two, branch_b_three) =
        phase70_branch_blocks();
    {
        let store = FjallNodeStore::open(path).expect("store");
        let mut transport = ScriptedTransport::new(vec![vec![
            WireNetworkMessage::Version(VersionMessage {
                start_height: 2,
                ..VersionMessage::default()
            }),
            WireNetworkMessage::Verack,
            WireNetworkMessage::Headers(HeadersMessage {
                headers: vec![
                    genesis.header.clone(),
                    branch_a_one.header.clone(),
                    branch_a_two.header.clone(),
                ],
            }),
            WireNetworkMessage::Block(genesis.clone()),
            WireNetworkMessage::Block(branch_a_one.clone()),
            WireNetworkMessage::Block(branch_a_two.clone()),
        ]]);
        let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
        runtime
            .sync_once(&mut transport, i64::from(branch_a_two.header.time))
            .expect("initial branch sync");
    }

    {
        let store = FjallNodeStore::open(path).expect("reopen store for durable branch");
        let mut transport = ScriptedTransport::new(vec![vec![
            WireNetworkMessage::Version(VersionMessage {
                start_height: 3,
                ..VersionMessage::default()
            }),
            WireNetworkMessage::Verack,
            WireNetworkMessage::Headers(HeadersMessage {
                headers: vec![
                    branch_b_one.header.clone(),
                    branch_b_two.header.clone(),
                    branch_b_three.header.clone(),
                ],
            }),
        ]]);
        let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
        runtime
            .sync_once(&mut transport, i64::from(branch_b_three.header.time))
            .expect("persist better branch headers");
        runtime
            .store()
            .save_block(&branch_b_one, PersistMode::Sync)
            .expect("save branch b one");
        runtime
            .store()
            .save_block(&branch_b_two, PersistMode::Sync)
            .expect("save branch b two");
        runtime
            .store()
            .save_block(&branch_b_three, PersistMode::Sync)
            .expect("save branch b three");
    }

    (
        genesis,
        branch_a_one,
        branch_a_two,
        branch_b_one,
        branch_b_two,
        branch_b_three,
    )
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
fn phase70_branch_awaiting_bodies_does_not_disconnect_active_chain() {
    // Arrange
    let path = temp_store_path("phase70-branch-awaiting-bodies");
    remove_dir_if_exists(&path);
    let (genesis, branch_a_one, branch_a_two, branch_b_one, branch_b_two, branch_b_three) =
        phase70_branch_blocks();
    let branch_b_one_hash = block_hash(&branch_b_one.header);
    save_best_chain_with_active_blocks(
        &path,
        &[
            (&genesis, 0),
            (&branch_b_one, 1),
            (&branch_b_two, 2),
            (&branch_b_three, 3),
        ],
        &[(&genesis, 0), (&branch_a_one, 1), (&branch_a_two, 2)],
    );
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");

    // Act
    let progress = super::block_reconcile::reconcile_and_persist_best_chain(
        &mut runtime,
        i64::from(branch_b_three.header.time),
    )
    .expect("reconcile should wait for missing branch bodies");
    let snapshot = runtime.snapshot_summary();
    let state = runtime
        .durable_sync_state_for_summary(
            &snapshot,
            SyncLifecycleState::Active,
            None,
            i64::from(branch_b_three.header.time),
        )
        .expect("durable reconcile status");

    // Assert
    assert_eq!(
        progress,
        SyncReconcileProgress::BranchCompetitionAwaitingBodies {
            missing_count: 3,
            first_missing_height: 1,
            first_missing_hash: block_hash_hex(branch_b_one_hash),
        }
    );
    assert_eq!(snapshot.best_block_height, 2);
    assert_eq!(
        snapshot.maybe_connected_block_hash,
        Some(block_hash_hex(block_hash(&branch_a_two.header)))
    );
    assert_eq!(
        state.sync.reconcile_progress,
        FieldAvailability::available(
            SyncReconcileProgressStatus::BranchCompetitionAwaitingBodies {
                common_ancestor_height: 0,
                common_ancestor_hash: block_hash_hex(block_hash(&genesis.header)),
                branch_tip_height: 3,
                branch_tip_hash: block_hash_hex(block_hash(&branch_b_three.header)),
                missing_block_count: 3,
            }
        )
    );

    remove_dir_if_exists(&path);
}

#[test]
fn phase70_reorg_records_bounded_persisted_evidence() {
    // Arrange
    let path = temp_store_path("phase70-branch-reorg-persisted");
    remove_dir_if_exists(&path);
    let (genesis, _branch_a_one, _branch_a_two, _branch_b_one, _branch_b_two, branch_b_three) =
        phase70_save_reorg_ready_branch(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    let expected_evidence = SyncReorgEvidence {
        common_ancestor_height: 0,
        common_ancestor_hash: block_hash_hex(block_hash(&genesis.header)),
        disconnected_count: 2,
        connected_count: 3,
        final_active_height: 3,
        final_active_hash: block_hash_hex(block_hash(&branch_b_three.header)),
        fully_persisted: true,
    };

    // Act
    let progress = super::block_reconcile::reconcile_and_persist_best_chain(
        &mut runtime,
        i64::from(branch_b_three.header.time),
    )
    .expect("reconcile should reorg to complete better branch");
    let snapshot = runtime.snapshot_summary();
    let state = runtime
        .durable_sync_state_for_summary(
            &snapshot,
            SyncLifecycleState::Active,
            None,
            i64::from(branch_b_three.header.time),
        )
        .expect("durable reorg status");
    runtime
        .persist_durable_sync_state(state.clone())
        .expect("persist reorg status");
    drop(runtime);
    let reopened_store = FjallNodeStore::open(&path).expect("reopen store");
    let reopened_runtime =
        DurableSyncRuntime::open(reopened_store, sync_config()).expect("reopen runtime");
    let reopened_summary = reopened_runtime.snapshot_summary();
    let reopened_state = reopened_runtime
        .durable_sync_state_for_summary(
            &reopened_summary,
            SyncLifecycleState::Active,
            None,
            i64::from(branch_b_three.header.time),
        )
        .expect("reopened durable reorg status");

    // Assert
    assert_eq!(
        progress,
        SyncReconcileProgress::ReorgPersisted(expected_evidence.clone())
    );
    assert_eq!(snapshot.best_block_height, 3);
    assert_eq!(
        snapshot.maybe_connected_block_hash,
        Some(block_hash_hex(block_hash(&branch_b_three.header)))
    );
    assert_eq!(
        state.sync.latest_reorg,
        FieldAvailability::available(expected_evidence.clone())
    );
    assert_eq!(
        state.sync.reconcile_progress,
        FieldAvailability::available(SyncReconcileProgressStatus::ReorgPersisted {
            evidence: expected_evidence.clone(),
        })
    );
    assert_eq!(
        reopened_state.sync.latest_reorg,
        FieldAvailability::available(expected_evidence)
    );

    remove_dir_if_exists(&path);
}

#[test]
fn phase70_equal_or_lower_work_side_branch_does_not_replace_active_tip() {
    // Arrange
    let path = temp_store_path("phase70-branch-side-preserved");
    remove_dir_if_exists(&path);
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let first_branch = build_branch_block(block_hash(&genesis.header), 1, 100);
    let second_branch = build_branch_block(block_hash(&genesis.header), 1, 200);
    let first_hash = block_hash(&first_branch.header);
    let second_hash = block_hash(&second_branch.header);
    let (active_tip, side_tip) = if first_hash > second_hash {
        (first_branch, second_branch)
    } else {
        (second_branch, first_branch)
    };
    let active_tip_hash = block_hash(&active_tip.header);
    let side_tip_hash = block_hash(&side_tip.header);
    save_best_chain_with_active_blocks(
        &path,
        &[(&genesis, 0), (&side_tip, 1)],
        &[(&genesis, 0), (&active_tip, 1)],
    );
    {
        let store = FjallNodeStore::open(&path).expect("store");
        store
            .save_block(&side_tip, PersistMode::Sync)
            .expect("save side branch body");
    }
    let store = FjallNodeStore::open(&path).expect("reopen store");
    let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");

    // Act
    let progress = super::block_reconcile::reconcile_and_persist_best_chain(
        &mut runtime,
        i64::from(side_tip.header.time),
    )
    .expect("reconcile should preserve equal-work side branch");
    let snapshot = runtime.snapshot_summary();
    let state = runtime
        .durable_sync_state_for_summary(
            &snapshot,
            SyncLifecycleState::Active,
            None,
            i64::from(side_tip.header.time),
        )
        .expect("durable side branch status");

    // Assert
    assert_eq!(progress, SyncReconcileProgress::SideBranchPreserved);
    assert_eq!(snapshot.best_block_height, 1);
    assert_eq!(
        snapshot.maybe_connected_block_hash,
        Some(block_hash_hex(active_tip_hash))
    );
    assert_eq!(
        state.sync.reconcile_progress,
        FieldAvailability::available(SyncReconcileProgressStatus::SideBranchPreserved {
            branch_tip_height: 1,
            branch_tip_hash: block_hash_hex(side_tip_hash),
            active_tip_height: 1,
            active_tip_hash: block_hash_hex(active_tip_hash),
        })
    );
    assert!(side_tip_hash < active_tip_hash);

    remove_dir_if_exists(&path);
}

#[test]
fn phase70_missing_active_chain_block_body_is_storage_blocker() {
    // Arrange
    let path = temp_store_path("phase70-missing-active-body");
    remove_dir_if_exists(&path);
    let (genesis, branch_a_one, branch_a_two, branch_b_one, branch_b_two, branch_b_three) =
        phase70_branch_blocks();
    save_chain_headers_snapshot_and_blocks(
        &path,
        &[
            (&genesis, 0),
            (&branch_b_one, 1),
            (&branch_b_two, 2),
            (&branch_b_three, 3),
        ],
        &[(&genesis, 0), (&branch_a_one, 1), (&branch_a_two, 2)],
        &[
            (&genesis, 0),
            (&branch_a_one, 1),
            (&branch_b_one, 1),
            (&branch_b_two, 2),
            (&branch_b_three, 3),
        ],
    );
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");

    // Act
    let error = super::block_reconcile::reconcile_best_chain(
        &mut runtime,
        i64::from(branch_b_three.header.time),
    )
    .expect_err("missing active body should block reorg");

    // Assert
    assert!(matches!(
        error,
        SyncRuntimeError::Storage(StorageError::Corruption {
            namespace: StorageNamespace::BlockIndex,
            action: StorageRecoveryAction::Repair,
            ref detail,
        }) if detail.contains("missing durable block body")
    ));

    remove_dir_if_exists(&path);
}

#[test]
fn phase70_missing_undo_data_is_storage_blocker() {
    // Arrange
    let path = temp_store_path("phase70-missing-undo");
    remove_dir_if_exists(&path);
    let (genesis, branch_a_one, branch_a_two, branch_b_one, branch_b_two, branch_b_three) =
        phase70_branch_blocks();
    save_chain_headers_snapshot_and_blocks(
        &path,
        &[
            (&genesis, 0),
            (&branch_b_one, 1),
            (&branch_b_two, 2),
            (&branch_b_three, 3),
        ],
        &[(&genesis, 0), (&branch_a_one, 1), (&branch_a_two, 2)],
        &[
            (&genesis, 0),
            (&branch_a_one, 1),
            (&branch_a_two, 2),
            (&branch_b_one, 1),
            (&branch_b_two, 2),
            (&branch_b_three, 3),
        ],
    );
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");

    // Act
    let error = super::block_reconcile::reconcile_best_chain(
        &mut runtime,
        i64::from(branch_b_three.header.time),
    )
    .expect_err("missing undo should block reorg");

    // Assert
    assert!(matches!(
        error,
        SyncRuntimeError::Storage(StorageError::Corruption {
            namespace: StorageNamespace::Chainstate,
            action: StorageRecoveryAction::Repair,
            ref detail,
        }) if detail.contains("missing undo data")
    ));

    remove_dir_if_exists(&path);
}

#[test]
fn phase70_malformed_stored_chainstate_is_storage_blocker() {
    // Arrange
    let path = temp_store_path("phase70-malformed-chainstate");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    store
        .write_raw_for_test(
            StorageNamespace::Chainstate,
            "snapshot",
            b"{bad-json".to_vec(),
        )
        .expect("write malformed chainstate snapshot");

    // Act
    let error = match DurableSyncRuntime::open(store, sync_config()) {
        Ok(_) => panic!("malformed chainstate should block runtime open"),
        Err(error) => error,
    };

    // Assert
    assert!(matches!(
        error,
        SyncRuntimeError::Storage(StorageError::Corruption {
            namespace: StorageNamespace::Chainstate,
            action: StorageRecoveryAction::Repair,
            ..
        })
    ));

    remove_dir_if_exists(&path);
}

#[test]
fn bounded_block_requests_use_validated_best_chain_headers_only() {
    // Arrange
    let path = temp_store_path("bounded-best-chain-requests");
    remove_dir_if_exists(&path);
    let active_block = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let requestable_block = build_block(block_hash(&active_block.header), 1);
    let durable_local_block = build_block(block_hash(&requestable_block.header), 2);
    let inflight_block = build_block(block_hash(&durable_local_block.header), 3);
    let unvalidated_block = build_block(BlockHash::from_byte_array([42_u8; 32]), 99);
    let active_hash = block_hash(&active_block.header);
    let requestable_hash = block_hash(&requestable_block.header);
    let durable_local_hash = block_hash(&durable_local_block.header);
    let inflight_hash = block_hash(&inflight_block.header);
    let unvalidated_hash = block_hash(&unvalidated_block.header);
    {
        let store = FjallNodeStore::open(&path).expect("store");
        store
            .save_header_entries(
                &[
                    HeaderEntry {
                        block_hash: active_hash,
                        header: active_block.header.clone(),
                        height: 0,
                        chain_work: 1,
                    },
                    HeaderEntry {
                        block_hash: requestable_hash,
                        header: requestable_block.header.clone(),
                        height: 1,
                        chain_work: 2,
                    },
                    HeaderEntry {
                        block_hash: durable_local_hash,
                        header: durable_local_block.header.clone(),
                        height: 2,
                        chain_work: 3,
                    },
                    HeaderEntry {
                        block_hash: inflight_hash,
                        header: inflight_block.header.clone(),
                        height: 3,
                        chain_work: 4,
                    },
                ],
                PersistMode::Sync,
            )
            .expect("save best-chain headers");
        store
            .save_chainstate_snapshot(
                &ChainstateSnapshot::new(
                    vec![ChainPosition::new(
                        active_block.header.clone(),
                        0,
                        1,
                        i64::from(active_block.header.time),
                    )],
                    Default::default(),
                    Default::default(),
                ),
                PersistMode::Sync,
            )
            .expect("save active chain snapshot");
        store
            .save_block(&durable_local_block, PersistMode::Sync)
            .expect("save durable local block");
    }
    let store = FjallNodeStore::open(&path).expect("reopen store");
    let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    runtime.inflight_blocks.insert(inflight_hash);
    let mut transport = ScriptedTransport::new(vec![vec![
        WireNetworkMessage::Version(VersionMessage {
            start_height: 3,
            ..VersionMessage::default()
        }),
        WireNetworkMessage::Verack,
        WireNetworkMessage::Inv(InventoryList::new(vec![InventoryVector {
            inventory_type: InventoryType::Block,
            object_hash: unvalidated_hash.into(),
        }])),
    ]]);

    // Act
    let summary = runtime
        .sync_once(&mut transport, i64::from(inflight_block.header.time))
        .expect("sync");
    let requested_hashes = getdata_block_hashes(&transport.sent_messages());

    // Assert
    assert_eq!(summary.best_header_height, 3);
    assert_eq!(requested_hashes, vec![requestable_hash]);
    assert!(!requested_hashes.contains(&active_hash));
    assert!(!requested_hashes.contains(&durable_local_hash));
    assert!(!requested_hashes.contains(&inflight_hash));
    assert!(!requested_hashes.contains(&unvalidated_hash));
    assert!(runtime.inflight_blocks.contains(&inflight_hash));

    remove_dir_if_exists(&path);
}

#[test]
fn bounded_block_requests_respect_per_peer_and_total_caps() {
    // Arrange
    let path = temp_store_path("bounded-request-caps");
    remove_dir_if_exists(&path);
    let first = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let second = build_block(block_hash(&first.header), 1);
    let third = build_block(block_hash(&second.header), 2);
    {
        let store = FjallNodeStore::open(&path).expect("store");
        store
            .save_header_entries(
                &[
                    HeaderEntry {
                        block_hash: block_hash(&first.header),
                        header: first.header.clone(),
                        height: 0,
                        chain_work: 1,
                    },
                    HeaderEntry {
                        block_hash: block_hash(&second.header),
                        header: second.header.clone(),
                        height: 1,
                        chain_work: 2,
                    },
                    HeaderEntry {
                        block_hash: block_hash(&third.header),
                        header: third.header.clone(),
                        height: 2,
                        chain_work: 3,
                    },
                ],
                PersistMode::Sync,
            )
            .expect("save headers");
    }
    let store = FjallNodeStore::open(&path).expect("reopen store");
    let mut runtime = DurableSyncRuntime::open(
        store,
        SyncRuntimeConfig {
            max_blocks_in_flight_per_peer: 1,
            max_blocks_in_flight_total: 2,
            ..sync_config()
        },
    )
    .expect("runtime");
    for peer_id in [1, 2, 3] {
        runtime
            .network
            .connect_outbound_peer(peer_id, 1_777_225_210)
            .expect("connect peer");
        runtime
            .network
            .receive_sync_message(
                peer_id,
                WireNetworkMessage::Version(VersionMessage {
                    start_height: 2,
                    ..VersionMessage::default()
                }),
                1_777_225_210,
                runtime.verify_flags,
                runtime.consensus_params,
            )
            .expect("receive version");
        runtime
            .network
            .receive_sync_message(
                peer_id,
                WireNetworkMessage::Verack,
                1_777_225_210,
                runtime.verify_flags,
                runtime.consensus_params,
            )
            .expect("receive verack");
    }

    // Act
    let first_peer_messages =
        super::block_reconcile::request_missing_blocks(&mut runtime, 1).expect("peer one request");
    let second_peer_messages =
        super::block_reconcile::request_missing_blocks(&mut runtime, 2).expect("peer two request");
    let first_peer_retry =
        super::block_reconcile::request_missing_blocks(&mut runtime, 1).expect("peer one retry");
    let third_peer_messages = super::block_reconcile::request_missing_blocks(&mut runtime, 3)
        .expect("peer three request");

    // Assert
    assert_eq!(getdata_block_hashes(&first_peer_messages).len(), 1);
    assert_eq!(getdata_block_hashes(&second_peer_messages).len(), 1);
    assert!(getdata_block_hashes(&first_peer_retry).is_empty());
    assert!(getdata_block_hashes(&third_peer_messages).is_empty());
    assert_eq!(runtime.inflight_blocks.len(), 2);
    assert_eq!(
        runtime
            .network
            .peer_requested_blocks(1)
            .expect("peer one requested blocks")
            .len(),
        1
    );
    assert_eq!(
        runtime
            .network
            .peer_requested_blocks(2)
            .expect("peer two requested blocks")
            .len(),
        1
    );
    assert!(runtime.inflight_blocks.len() <= runtime.config.max_blocks_in_flight_total);

    remove_dir_if_exists(&path);
}

#[test]
fn phase110_block_serving_cleanup_never_exceeds_total_inflight_limit() {
    // Arrange
    let path = temp_store_path("phase110-block-serving-cleanup-total-limit");
    remove_dir_if_exists(&path);
    let first = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let second = build_block(block_hash(&first.header), 1);
    let third = build_block(block_hash(&second.header), 2);
    let fourth = build_block(block_hash(&third.header), 3);
    {
        let store = FjallNodeStore::open(&path).expect("store");
        store
            .save_header_entries(
                &[
                    HeaderEntry {
                        block_hash: block_hash(&first.header),
                        header: first.header.clone(),
                        height: 0,
                        chain_work: 1,
                    },
                    HeaderEntry {
                        block_hash: block_hash(&second.header),
                        header: second.header.clone(),
                        height: 1,
                        chain_work: 2,
                    },
                    HeaderEntry {
                        block_hash: block_hash(&third.header),
                        header: third.header.clone(),
                        height: 2,
                        chain_work: 3,
                    },
                    HeaderEntry {
                        block_hash: block_hash(&fourth.header),
                        header: fourth.header.clone(),
                        height: 3,
                        chain_work: 4,
                    },
                ],
                PersistMode::Sync,
            )
            .expect("save headers");
    }
    let store = FjallNodeStore::open(&path).expect("reopen store");
    let mut runtime = DurableSyncRuntime::open(
        store,
        SyncRuntimeConfig {
            max_blocks_in_flight_per_peer: 2,
            max_blocks_in_flight_total: 3,
            ..sync_config()
        },
    )
    .expect("runtime");
    for peer_id in [110, 111, 112] {
        connect_runtime_peer(&mut runtime, peer_id, 3);
    }

    // Act
    let first_peer_messages =
        super::block_reconcile::request_missing_blocks(&mut runtime, 110).expect("peer one");
    let second_peer_messages =
        super::block_reconcile::request_missing_blocks(&mut runtime, 111).expect("peer two");
    let third_peer_messages =
        super::block_reconcile::request_missing_blocks(&mut runtime, 112).expect("peer three");

    // Assert
    assert_eq!(getdata_block_hashes(&first_peer_messages).len(), 2);
    assert_eq!(getdata_block_hashes(&second_peer_messages).len(), 1);
    assert!(getdata_block_hashes(&third_peer_messages).is_empty());
    assert_eq!(runtime.inflight_blocks.len(), 3);
    assert!(runtime.inflight_blocks.len() <= runtime.config.max_blocks_in_flight_total);

    remove_dir_if_exists(&path);
}

#[test]
fn phase110_block_serving_cleanup_releases_block_and_notfound_inflight() {
    // Arrange
    let path = temp_store_path("phase110-block-serving-cleanup-release-message");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    let block = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let block_hash = block_hash(&block.header);
    let notfound_hash = BlockHash::from_byte_array([110_u8; 32]);

    // Act
    runtime.inflight_blocks.insert(block_hash);
    super::block_reconcile::release_inflight_for_message(
        &mut runtime,
        &WireNetworkMessage::Block(block),
    );
    runtime.inflight_blocks.insert(notfound_hash);
    super::block_reconcile::release_inflight_for_message(
        &mut runtime,
        &notfound_for_block(notfound_hash),
    );

    // Assert
    assert!(runtime.inflight_blocks.is_empty());

    remove_dir_if_exists(&path);
}

#[test]
fn notfound_releases_block_inflight_for_retry() {
    // Arrange
    let path = temp_store_path("block-inflight-notfound");
    remove_dir_if_exists(&path);
    let block = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let block_hash = block_hash(&block.header);
    let first_peer_script = vec![
        WireNetworkMessage::Version(VersionMessage {
            start_height: 0,
            ..VersionMessage::default()
        }),
        WireNetworkMessage::Verack,
        WireNetworkMessage::Headers(HeadersMessage {
            headers: vec![block.header.clone()],
        }),
        notfound_for_block(block_hash),
    ];
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(store, two_peer_sync_config()).expect("runtime");
    let mut transport = ScriptedTransport::new(vec![first_peer_script, version_verack_script(0)]);

    // Act
    let summary = runtime
        .sync_once(&mut transport, i64::from(block.header.time))
        .expect("sync");
    let requested_hashes = getdata_block_hashes(&transport.sent_messages());

    // Assert
    assert!(
        requested_hashes
            .iter()
            .filter(|hash| **hash == block_hash)
            .count()
            >= 2
    );
    assert!(runtime.inflight_blocks.is_empty());
    assert_eq!(summary.downloaded_block_height, 0);
    assert_eq!(summary.best_block_height, 0);
    assert!(
        runtime
            .store()
            .load_block(block_hash)
            .expect("load notfound block")
            .is_none()
    );

    remove_dir_if_exists(&path);
}

#[test]
fn disconnect_clears_runtime_and_peer_block_inflight() {
    // Arrange
    let path = temp_store_path("block-inflight-disconnect");
    remove_dir_if_exists(&path);
    let block = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let block_hash = block_hash(&block.header);
    let first_peer_script = headers_script(0, vec![block.header.clone()]);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(store, two_peer_sync_config()).expect("runtime");
    let mut transport = ScriptedTransport::new(vec![first_peer_script, version_verack_script(0)]);

    // Act
    let summary = runtime
        .sync_once(&mut transport, i64::from(block.header.time))
        .expect("sync");
    let requested_hashes = getdata_block_hashes(&transport.sent_messages());

    // Assert
    assert!(
        requested_hashes
            .iter()
            .filter(|hash| **hash == block_hash)
            .count()
            >= 2
    );
    assert!(runtime.inflight_blocks.is_empty());
    assert!(runtime.network.peer_requested_blocks(1).is_err());
    assert_eq!(summary.downloaded_block_height, 0);
    assert_eq!(summary.best_block_height, 0);

    remove_dir_if_exists(&path);
}

#[test]
fn duplicate_outbox_registration_does_not_disconnect_an_existing_peer() {
    // Arrange
    let path = temp_store_path("duplicate-outbox-registration-cleanup");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    let peer_id = 128_402;
    connect_runtime_peer(&mut runtime, peer_id, 0);
    runtime
        .announcement_outboxes
        .register_peer(peer_id)
        .expect("register existing peer outbox");
    let session = ScriptedSession {
        inbound: VecDeque::new(),
        sent: Rc::new(RefCell::new(Vec::new())),
    };
    let peer = resolved_manual_peer("127.0.0.1", 18_444);
    let mut clock = || 1_777_225_211;

    // Act
    let failure = runtime
        .sync_connected_peer(session, &peer, peer_id, 1, 1_777_225_210, &mut clock)
        .expect_err("duplicate outbox ownership should reject the session");
    let snapshots = runtime
        .announcement_outboxes
        .snapshots()
        .expect("outbox snapshots");

    // Assert
    assert!(matches!(
        failure.error,
        SyncRuntimeError::Network { ref message }
            if message.contains("already registered")
    ));
    assert!(runtime.network.peer_requested_blocks(peer_id).is_ok());
    assert_eq!(snapshots.len(), 1);
    assert_eq!(snapshots[0].peer_id(), peer_id);

    remove_dir_if_exists(&path);
}

#[test]
fn phase110_block_serving_cleanup_disconnect_releases_peer_and_runtime_inflight() {
    // Arrange
    let path = temp_store_path("phase110-block-serving-cleanup-disconnect");
    remove_dir_if_exists(&path);
    let block = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let block_hash = block_hash(&block.header);
    let first_peer_script = headers_script(0, vec![block.header.clone()]);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(store, two_peer_sync_config()).expect("runtime");
    let mut transport = ScriptedTransport::new(vec![first_peer_script, version_verack_script(0)]);

    // Act
    let summary = runtime
        .sync_once(&mut transport, i64::from(block.header.time))
        .expect("sync");
    let requested_hashes = getdata_block_hashes(&transport.sent_messages());

    // Assert
    assert!(
        requested_hashes
            .iter()
            .filter(|hash| **hash == block_hash)
            .count()
            >= 2
    );
    assert!(runtime.inflight_blocks.is_empty());
    assert!(runtime.network.peer_requested_blocks(1).is_err());
    assert_eq!(summary.best_block_height, 0);

    remove_dir_if_exists(&path);
}

#[test]
fn phase110_block_serving_cleanup_reopen_starts_without_stale_inflight() {
    // Arrange
    let path = temp_store_path("phase110-block-serving-cleanup-stale-reopen");
    remove_dir_if_exists(&path);
    let stale_hash = BlockHash::from_byte_array([111_u8; 32]);
    {
        let store = FjallNodeStore::open(&path).expect("store");
        let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
        connect_runtime_peer(&mut runtime, 110, 0);
        runtime.inflight_blocks.insert(stale_hash);
        assert!(runtime.network.peer_requested_blocks(110).is_ok());
    }
    let store = FjallNodeStore::open(&path).expect("reopen store");

    // Act
    let runtime = DurableSyncRuntime::open(store, sync_config()).expect("reopen runtime");

    // Assert
    assert!(runtime.inflight_blocks.is_empty());
    assert!(runtime.network.peer_requested_blocks(110).is_err());

    remove_dir_if_exists(&path);
}

mod block_response {
    use super::*;

    fn assert_peer_reason_without_block_credit(
        summary: &SyncRunSummary,
        reason: PeerFailureReason,
    ) {
        let outcome = summary
            .peer_outcomes
            .iter()
            .find(|outcome| outcome.maybe_failure_reason.as_ref() == Some(&reason))
            .expect("peer outcome with block response failure reason");
        assert_eq!(outcome.contribution.blocks_received, 0);
    }

    #[test]
    fn first_non_genesis_block_connect_advances_downloaded_and_connected_height() {
        // Arrange
        let path = temp_store_path("block-response-first-connect");
        remove_dir_if_exists(&path);
        let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
        let child = build_block(block_hash(&genesis.header), 1);
        let child_hash = block_hash(&child.header);
        save_best_chain_with_active_blocks(&path, &[(&genesis, 0), (&child, 1)], &[(&genesis, 0)]);
        let store = FjallNodeStore::open(&path).expect("reopen store");
        let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
        let mut transport = ScriptedTransport::new(vec![vec![
            WireNetworkMessage::Version(VersionMessage {
                start_height: 1,
                ..VersionMessage::default()
            }),
            WireNetworkMessage::Verack,
            WireNetworkMessage::Block(child.clone()),
        ]]);

        // Act
        let summary = runtime
            .sync_once(&mut transport, i64::from(child.header.time))
            .expect("sync");

        // Assert
        assert_eq!(summary.downloaded_block_height, 1);
        assert_eq!(summary.best_block_height, 1);
        assert_eq!(summary.blocks_received, 1);
        assert_eq!(summary.peer_outcomes.len(), 1);
        assert_eq!(summary.peer_outcomes[0].contribution.blocks_received, 1);
        assert!(
            runtime
                .store()
                .load_block(child_hash)
                .expect("load connected child")
                .is_some()
        );

        remove_dir_if_exists(&path);
    }

    #[test]
    fn connected_active_chain_progress_survives_runtime_reopen() {
        // Arrange
        let path = temp_store_path("block-response-connected-reopen");
        remove_dir_if_exists(&path);
        let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
        let child = build_block(block_hash(&genesis.header), 1);
        let child_hash = block_hash(&child.header);
        let expected_child_hash = block_hash_hex(child_hash);
        save_best_chain_with_active_blocks(&path, &[(&genesis, 0), (&child, 1)], &[(&genesis, 0)]);
        let store = FjallNodeStore::open(&path).expect("store");
        let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
        let mut transport = ScriptedTransport::new(vec![vec![
            WireNetworkMessage::Version(VersionMessage {
                start_height: 1,
                ..VersionMessage::default()
            }),
            WireNetworkMessage::Verack,
            WireNetworkMessage::Block(child.clone()),
        ]]);

        // Act
        let summary = runtime
            .sync_once(&mut transport, i64::from(child.header.time))
            .expect("sync");
        let connected_progress = summary.sync_status(SyncNetwork::Regtest).sync_progress;
        drop(runtime);

        let reopened_store = FjallNodeStore::open(&path).expect("reopen store");
        let reopened_runtime =
            DurableSyncRuntime::open(reopened_store, sync_config()).expect("reopen runtime");
        let reopened_summary = reopened_runtime.snapshot_summary();
        let reopened_state = reopened_runtime
            .durable_sync_state_for_summary(
                &reopened_summary,
                SyncLifecycleState::Active,
                None,
                1_777_225_182,
            )
            .expect("reopened durable status");

        // Assert
        assert_eq!(summary.downloaded_block_height, 1);
        assert_eq!(summary.best_block_height, 1);
        assert_eq!(
            connected_progress,
            FieldAvailability::available(SyncProgress {
                header_height: 1,
                block_height: 1,
                downloaded_block_height: 1,
                connected_block_height: 1,
                validated_active_chain_height: 1,
                maybe_downloaded_block_hash: Some(expected_child_hash.clone()),
                maybe_connected_block_hash: Some(expected_child_hash.clone()),
                maybe_validated_active_chain_hash: Some(expected_child_hash.clone()),
                maybe_validated_active_chain_work: Some("2".to_string()),
                progress_ratio: 1.0,
                messages_processed: 3,
                headers_received: 0,
                blocks_received: 1,
            })
        );
        assert_eq!(reopened_summary.best_block_height, 1);
        assert_eq!(reopened_summary.downloaded_block_height, 1);
        assert_eq!(
            reopened_summary.maybe_connected_block_hash,
            Some(expected_child_hash.clone())
        );
        assert_eq!(
            reopened_summary.maybe_validated_active_chain_work,
            Some("2".to_string())
        );
        assert_eq!(
            reopened_state.sync.sync_progress,
            FieldAvailability::available(SyncProgress {
                header_height: 1,
                block_height: 1,
                downloaded_block_height: 1,
                connected_block_height: 1,
                validated_active_chain_height: 1,
                maybe_downloaded_block_hash: Some(expected_child_hash.clone()),
                maybe_connected_block_hash: Some(expected_child_hash.clone()),
                maybe_validated_active_chain_hash: Some(expected_child_hash),
                maybe_validated_active_chain_work: Some("2".to_string()),
                progress_ratio: 1.0,
                messages_processed: 0,
                headers_received: 0,
                blocks_received: 0,
            })
        );
        assert!(
            reopened_runtime
                .store()
                .load_block(child_hash)
                .expect("load reopened child")
                .is_some()
        );
        let snapshot = reopened_runtime
            .store()
            .load_chainstate_snapshot()
            .expect("load chainstate snapshot")
            .expect("chainstate snapshot");
        let active_tip = snapshot.active_chain.last().expect("active tip");
        assert_eq!(active_tip.height, 1);
        assert_eq!(active_tip.block_hash, child_hash);
        assert_eq!(active_tip.chain_work, 2);

        remove_dir_if_exists(&path);
    }

    #[test]
    fn unrequested_extending_block_response_is_no_credit_and_does_not_mutate_chainstate() {
        // Arrange
        let path = temp_store_path("block-response-unrequested-extending");
        remove_dir_if_exists(&path);
        let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
        let child = build_block(block_hash(&genesis.header), 1);
        let child_hash = block_hash(&child.header);
        save_best_chain_with_active_blocks(&path, &[(&genesis, 0)], &[(&genesis, 0)]);
        let store = FjallNodeStore::open(&path).expect("reopen store");
        let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
        let mut transport = ScriptedTransport::new(vec![vec![
            WireNetworkMessage::Version(VersionMessage {
                start_height: 1,
                ..VersionMessage::default()
            }),
            WireNetworkMessage::Verack,
            WireNetworkMessage::Block(child),
        ]]);

        // Act
        let summary = runtime
            .sync_once(&mut transport, i64::from(genesis.header.time))
            .expect("sync");
        let active_chain = runtime
            .network
            .chainstate_snapshot()
            .expect("authoritative chainstate snapshot")
            .active_chain;

        // Assert
        assert_eq!(summary.blocks_received, 0);
        assert_eq!(summary.downloaded_block_height, 0);
        assert_eq!(summary.best_block_height, 0);
        assert_eq!(active_chain.len(), 1);
        assert_eq!(
            active_chain.last().map(|position| position.block_hash),
            Some(block_hash(&genesis.header))
        );
        assert_peer_reason_without_block_credit(&summary, PeerFailureReason::DisconnectedBlock);
        assert!(
            runtime
                .store()
                .load_block(child_hash)
                .expect("load unrequested child")
                .is_none()
        );

        remove_dir_if_exists(&path);
    }

    #[test]
    fn sync_progress_reports_downloaded_and_connected_block_hashes() {
        // Arrange
        let path = temp_store_path("sync-progress-connected-hashes");
        remove_dir_if_exists(&path);
        let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
        let child = build_block(block_hash(&genesis.header), 1);
        let child_hash = block_hash(&child.header);
        save_best_chain_with_active_blocks(&path, &[(&genesis, 0), (&child, 1)], &[(&genesis, 0)]);
        let store = FjallNodeStore::open(&path).expect("reopen store");
        let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
        let mut transport = ScriptedTransport::new(vec![vec![
            WireNetworkMessage::Version(VersionMessage {
                start_height: 1,
                ..VersionMessage::default()
            }),
            WireNetworkMessage::Verack,
            WireNetworkMessage::Block(child.clone()),
        ]]);

        // Act
        let summary = runtime
            .sync_once(&mut transport, i64::from(child.header.time))
            .expect("sync");
        let sync_progress = summary.sync_status(SyncNetwork::Regtest).sync_progress;

        // Assert
        assert_eq!(
            sync_progress,
            FieldAvailability::available(SyncProgress {
                header_height: 1,
                block_height: 1,
                downloaded_block_height: 1,
                connected_block_height: 1,
                validated_active_chain_height: 1,
                maybe_downloaded_block_hash: Some(block_hash_hex(child_hash)),
                maybe_connected_block_hash: Some(block_hash_hex(child_hash)),
                maybe_validated_active_chain_hash: Some(block_hash_hex(child_hash)),
                maybe_validated_active_chain_work: Some("2".to_string()),
                progress_ratio: 1.0,
                messages_processed: 3,
                headers_received: 0,
                blocks_received: 1,
            })
        );

        remove_dir_if_exists(&path);
    }

    #[test]
    fn sync_progress_reports_downloaded_only_block_hash() {
        // Arrange
        let path = temp_store_path("sync-progress-downloaded-only-hash");
        remove_dir_if_exists(&path);
        let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
        let child = build_block(block_hash(&genesis.header), 1);
        let genesis_hash = block_hash(&genesis.header);
        let child_hash = block_hash(&child.header);
        save_best_chain_with_active_blocks(&path, &[(&genesis, 0), (&child, 1)], &[(&genesis, 0)]);
        {
            let store = FjallNodeStore::open(&path).expect("store");
            store
                .save_block(&child, PersistMode::Sync)
                .expect("save downloaded child");
        }

        let store = FjallNodeStore::open(&path).expect("reopen store");
        let runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");

        // Act
        let summary = runtime.snapshot_summary();
        let status = runtime
            .durable_sync_state_for_summary(
                &summary,
                SyncLifecycleState::Active,
                None,
                1_777_225_180,
            )
            .expect("durable status");

        // Assert
        assert_eq!(
            status.sync.sync_progress,
            FieldAvailability::available(SyncProgress {
                header_height: 1,
                block_height: 0,
                downloaded_block_height: 1,
                connected_block_height: 0,
                validated_active_chain_height: 0,
                maybe_downloaded_block_hash: Some(block_hash_hex(child_hash)),
                maybe_connected_block_hash: Some(block_hash_hex(genesis_hash)),
                maybe_validated_active_chain_hash: Some(block_hash_hex(genesis_hash)),
                maybe_validated_active_chain_work: Some("1".to_string()),
                progress_ratio: 0.0,
                messages_processed: 0,
                headers_received: 0,
                blocks_received: 0,
            })
        );

        remove_dir_if_exists(&path);
    }

    #[test]
    fn phase69_peer_agreement_classifies_agrees_behind_disagrees_and_no_evidence() {
        // Arrange
        let path = temp_store_path("phase69-peer-agreement");
        remove_dir_if_exists(&path);
        let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
        let child = build_block(block_hash(&genesis.header), 1);
        let best_tip = build_block(block_hash(&child.header), 2);
        let child_hash = block_hash(&child.header);
        let best_tip_hash = block_hash(&best_tip.header);
        save_best_chain_with_active_blocks(
            &path,
            &[(&genesis, 0), (&child, 1), (&best_tip, 2)],
            &[(&genesis, 0), (&child, 1), (&best_tip, 2)],
        );
        let store = FjallNodeStore::open(&path).expect("store");
        let runtime = DurableSyncRuntime::open(store, two_peer_sync_config()).expect("runtime");
        let mut agrees = peer_outcome(
            SyncPeerAddress::manual("127.0.0.1", 18_444),
            PeerSyncState::Connected,
            1,
            None,
            None,
        );
        agrees.maybe_tip_height = Some(2);
        agrees.maybe_tip_hash = Some(block_hash_hex(best_tip_hash));
        agrees.maybe_tip_work = Some("3".to_string());
        agrees.maybe_last_activity_unix_seconds = Some(u64::from(best_tip.header.time));
        let mut behind = peer_outcome(
            SyncPeerAddress::manual("127.0.0.1", 18_445),
            PeerSyncState::Connected,
            1,
            None,
            None,
        );
        behind.maybe_tip_height = Some(1);
        behind.maybe_tip_hash = Some(block_hash_hex(child_hash));
        behind.maybe_tip_work = Some("2".to_string());
        behind.maybe_last_activity_unix_seconds = Some(u64::from(best_tip.header.time));
        let mut disagrees = peer_outcome(
            SyncPeerAddress::manual("127.0.0.1", 18_446),
            PeerSyncState::Connected,
            1,
            None,
            None,
        );
        disagrees.maybe_tip_height = Some(2);
        disagrees.maybe_tip_hash = Some("aa".repeat(32));
        disagrees.maybe_tip_work = Some("3".to_string());
        disagrees.maybe_last_activity_unix_seconds = Some(u64::from(best_tip.header.time));
        let no_evidence = peer_outcome(
            SyncPeerAddress::manual("127.0.0.1", 18_447),
            PeerSyncState::Connected,
            1,
            None,
            None,
        );
        let mut summary = SyncRunSummary::empty(2, 2, 4);
        summary.connected_peers = 4;
        summary.peer_outcomes = vec![agrees, behind, disagrees, no_evidence];

        // Act
        let state = runtime
            .durable_sync_state_for_summary(
                &summary,
                SyncLifecycleState::Active,
                None,
                i64::from(best_tip.header.time) + 30,
            )
            .expect("durable status");

        // Assert
        let FieldAvailability::Available(best_known_tip) = state.sync.best_known_tip else {
            panic!("best-known tip should be available");
        };
        assert_eq!(best_known_tip.source, BestKnownTipSource::HeaderStore);
        assert_eq!(best_known_tip.height, 2);
        assert_eq!(best_known_tip.block_hash, block_hash_hex(best_tip_hash));
        assert_eq!(
            best_known_tip
                .peer_agreement
                .iter()
                .map(|row| row.status)
                .collect::<Vec<_>>(),
            vec![
                PeerTipAgreementStatus::Agrees,
                PeerTipAgreementStatus::Behind,
                PeerTipAgreementStatus::Disagrees,
                PeerTipAgreementStatus::NoEvidence,
            ]
        );
        assert_eq!(
            state.sync.stay_current,
            FieldAvailability::available(StayCurrentStatus::CurrentAtBestKnownTip)
        );

        remove_dir_if_exists(&path);
    }

    #[test]
    fn phase69_peer_tip_observation_uses_peer_terminal_header_not_global_best() {
        // Arrange
        let path = temp_store_path("phase69-peer-terminal-header");
        remove_dir_if_exists(&path);
        let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
        let global_child = build_block(block_hash(&genesis.header), 1);
        let global_tip = build_block(block_hash(&global_child.header), 2);
        let peer_terminal = build_branch_block(block_hash(&genesis.header), 1, 200);
        let global_tip_hash = block_hash(&global_tip.header);
        let peer_terminal_hash = block_hash(&peer_terminal.header);
        save_best_chain_with_active_blocks(
            &path,
            &[(&genesis, 0), (&global_child, 1), (&global_tip, 2)],
            &[(&genesis, 0)],
        );
        let store = FjallNodeStore::open(&path).expect("store");
        let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
        assert_eq!(
            runtime
                .network
                .peer_manager_snapshot()
                .expect("authoritative peer-manager snapshot")
                .header_store()
                .best_tip()
                .map(|entry| entry.block_hash),
            Some(global_tip_hash)
        );
        let mut transport =
            ScriptedTransport::new(vec![headers_script(2, vec![peer_terminal.header.clone()])]);

        // Act
        let summary = runtime
            .sync_once(&mut transport, i64::from(peer_terminal.header.time))
            .expect("sync summary");
        let state = runtime
            .durable_sync_state_for_summary(
                &summary,
                SyncLifecycleState::Active,
                None,
                i64::from(peer_terminal.header.time),
            )
            .expect("durable status");

        // Assert
        assert_eq!(summary.headers_received, 1);
        assert_eq!(
            runtime
                .network
                .peer_manager_snapshot()
                .expect("authoritative peer-manager snapshot")
                .header_store()
                .best_tip()
                .map(|entry| entry.block_hash),
            Some(global_tip_hash)
        );
        let outcome = summary.peer_outcomes.first().expect("peer outcome");
        assert_eq!(outcome.maybe_tip_height, Some(1));
        assert_eq!(
            outcome.maybe_tip_hash,
            Some(block_hash_hex(peer_terminal_hash))
        );
        assert_eq!(outcome.maybe_tip_work, Some("2".to_string()));
        let FieldAvailability::Available(best_known_tip) = state.sync.best_known_tip else {
            panic!("best-known tip should be available");
        };
        assert_eq!(best_known_tip.block_hash, block_hash_hex(global_tip_hash));
        assert_eq!(
            best_known_tip.peer_agreement.first().map(|row| row.status),
            Some(PeerTipAgreementStatus::Behind)
        );

        remove_dir_if_exists(&path);
    }

    #[test]
    fn sync_progress_omits_block_hashes_when_unavailable() {
        // Arrange
        let path = temp_store_path("sync-progress-no-hashes");
        remove_dir_if_exists(&path);
        let header_only_block = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
        {
            let store = FjallNodeStore::open(&path).expect("store");
            store
                .save_header_entries(
                    &[HeaderEntry {
                        block_hash: block_hash(&header_only_block.header),
                        header: header_only_block.header.clone(),
                        height: 0,
                        chain_work: 1,
                    }],
                    PersistMode::Sync,
                )
                .expect("save header");
        }
        let store = FjallNodeStore::open(&path).expect("reopen store");
        let runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");

        // Act
        let summary = runtime.snapshot_summary();
        let status = runtime
            .durable_sync_state_for_summary(
                &summary,
                SyncLifecycleState::Active,
                None,
                1_777_225_181,
            )
            .expect("durable status");
        let encoded = serde_json::to_value(&status.sync.sync_progress).expect("sync progress json");

        // Assert
        assert_eq!(
            status.sync.sync_progress,
            FieldAvailability::available(SyncProgress {
                header_height: 0,
                block_height: 0,
                downloaded_block_height: 0,
                connected_block_height: 0,
                validated_active_chain_height: 0,
                maybe_downloaded_block_hash: None,
                maybe_connected_block_hash: None,
                maybe_validated_active_chain_hash: None,
                maybe_validated_active_chain_work: None,
                progress_ratio: 1.0,
                messages_processed: 0,
                headers_received: 0,
                blocks_received: 0,
            })
        );
        assert!(encoded["value"]["maybe_downloaded_block_hash"].is_null());
        assert!(encoded["value"]["maybe_connected_block_hash"].is_null());
        assert!(encoded["value"]["maybe_validated_active_chain_hash"].is_null());
        assert!(encoded["value"]["maybe_validated_active_chain_work"].is_null());

        remove_dir_if_exists(&path);
    }

    #[test]
    fn block_notfound_is_peer_attributed_no_credit() {
        // Arrange
        let path = temp_store_path("block-response-notfound");
        remove_dir_if_exists(&path);
        let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
        let child = build_block(block_hash(&genesis.header), 1);
        let child_hash = block_hash(&child.header);
        save_best_chain_with_active_blocks(&path, &[(&genesis, 0), (&child, 1)], &[(&genesis, 0)]);
        let store = FjallNodeStore::open(&path).expect("reopen store");
        let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
        let mut transport = ScriptedTransport::new(vec![vec![
            WireNetworkMessage::Version(VersionMessage {
                start_height: 1,
                ..VersionMessage::default()
            }),
            WireNetworkMessage::Verack,
            notfound_for_block(child_hash),
        ]]);

        // Act
        let summary = runtime
            .sync_once(&mut transport, i64::from(child.header.time))
            .expect("sync");
        let requested_hashes = getdata_block_hashes(&transport.sent_messages());

        // Assert
        assert!(
            requested_hashes
                .iter()
                .filter(|hash| **hash == child_hash)
                .count()
                >= 2
        );
        assert!(runtime.inflight_blocks.is_empty());
        assert_eq!(summary.blocks_received, 0);
        assert_eq!(summary.downloaded_block_height, 0);
        assert_eq!(summary.best_block_height, 0);
        assert_peer_reason_without_block_credit(&summary, PeerFailureReason::BlockNotFound);
        assert!(
            runtime
                .store()
                .load_block(child_hash)
                .expect("load missing child")
                .is_none()
        );

        remove_dir_if_exists(&path);
    }

    #[test]
    fn duplicate_block_response_is_peer_attributed_no_credit() {
        // Arrange
        let path = temp_store_path("block-response-duplicate");
        remove_dir_if_exists(&path);
        let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
        let child = build_block(block_hash(&genesis.header), 1);
        let child_hash = block_hash(&child.header);
        save_best_chain_with_active_blocks(
            &path,
            &[(&genesis, 0), (&child, 1)],
            &[(&genesis, 0), (&child, 1)],
        );
        let store = FjallNodeStore::open(&path).expect("reopen store");
        let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
        let mut transport = ScriptedTransport::new(vec![vec![
            WireNetworkMessage::Version(VersionMessage {
                start_height: 1,
                ..VersionMessage::default()
            }),
            WireNetworkMessage::Verack,
            WireNetworkMessage::Block(child.clone()),
        ]]);

        // Act
        let summary = runtime
            .sync_once(&mut transport, i64::from(child.header.time))
            .expect("sync");
        let active_chain = runtime
            .network
            .chainstate_snapshot()
            .expect("authoritative chainstate snapshot")
            .active_chain;

        // Assert
        assert_eq!(summary.blocks_received, 0);
        assert_eq!(summary.downloaded_block_height, 1);
        assert_eq!(summary.best_block_height, 1);
        assert_eq!(active_chain.len(), 2);
        assert_eq!(
            active_chain.last().map(|position| position.block_hash),
            Some(child_hash)
        );
        assert_peer_reason_without_block_credit(&summary, PeerFailureReason::DuplicateBlock);

        remove_dir_if_exists(&path);
    }

    #[test]
    fn disconnected_block_response_is_peer_attributed_no_credit() {
        // Arrange
        let path = temp_store_path("block-response-disconnected");
        remove_dir_if_exists(&path);
        let block = build_block(BlockHash::from_byte_array([7_u8; 32]), 1);
        let block_hash = block_hash(&block.header);
        let store = FjallNodeStore::open(&path).expect("store");
        let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
        let mut transport = ScriptedTransport::new(vec![vec![
            WireNetworkMessage::Version(VersionMessage {
                start_height: 1,
                ..VersionMessage::default()
            }),
            WireNetworkMessage::Verack,
            WireNetworkMessage::Block(block.clone()),
        ]]);

        // Act
        let summary = runtime
            .sync_once(&mut transport, i64::from(block.header.time))
            .expect("sync");

        // Assert
        assert_eq!(summary.blocks_received, 0);
        assert_eq!(summary.downloaded_block_height, 0);
        assert_eq!(summary.best_block_height, 0);
        assert!(
            runtime
                .network
                .chainstate_snapshot()
                .expect("authoritative chainstate snapshot")
                .active_chain
                .is_empty()
        );
        assert_peer_reason_without_block_credit(&summary, PeerFailureReason::DisconnectedBlock);
        assert!(
            runtime
                .store()
                .load_block(block_hash)
                .expect("load disconnected block")
                .is_none()
        );

        remove_dir_if_exists(&path);
    }

    #[test]
    fn non_extending_block_response_is_peer_attributed_no_credit() {
        // Arrange
        let path = temp_store_path("block-response-non-extending");
        remove_dir_if_exists(&path);
        let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
        let side_block = build_block(BlockHash::from_byte_array([42_u8; 32]), 1);
        let side_hash = block_hash(&side_block.header);
        save_best_chain_with_active_blocks(&path, &[(&genesis, 0)], &[(&genesis, 0)]);
        let store = FjallNodeStore::open(&path).expect("reopen store");
        let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
        let mut transport = ScriptedTransport::new(vec![vec![
            WireNetworkMessage::Version(VersionMessage {
                start_height: 1,
                ..VersionMessage::default()
            }),
            WireNetworkMessage::Verack,
            WireNetworkMessage::Block(side_block.clone()),
        ]]);

        // Act
        let summary = runtime
            .sync_once(&mut transport, i64::from(side_block.header.time))
            .expect("sync");
        let active_chain = runtime
            .network
            .chainstate_snapshot()
            .expect("authoritative chainstate snapshot")
            .active_chain;

        // Assert
        assert_eq!(summary.blocks_received, 0);
        assert_eq!(summary.downloaded_block_height, 0);
        assert_eq!(summary.best_block_height, 0);
        assert_eq!(active_chain.len(), 1);
        assert_eq!(
            active_chain.last().map(|position| position.block_hash),
            Some(block_hash(&genesis.header))
        );
        assert_peer_reason_without_block_credit(&summary, PeerFailureReason::NonExtendingBlock);
        assert!(
            runtime
                .store()
                .load_block(side_hash)
                .expect("load non-extending block")
                .is_none()
        );

        remove_dir_if_exists(&path);
    }
}

mod phase70_peer {
    use super::*;

    fn rotation_config() -> SyncRuntimeConfig {
        SyncRuntimeConfig {
            manual_peers: vec![
                SyncPeerAddress::manual("127.0.0.1", 18_444),
                SyncPeerAddress::manual("127.0.0.1", 18_445),
            ],
            dns_seeds: Vec::new(),
            target_outbound_peers: 1,
            max_peer_retries: 0,
            retry_backoff_ms: 10_000,
            max_messages_per_peer: 8,
            ..sync_config()
        }
    }

    fn outcome_with_reason(
        summary: &SyncRunSummary,
        reason: PeerFailureReason,
    ) -> &PeerSyncOutcome {
        summary
            .peer_outcomes
            .iter()
            .find(|outcome| outcome.maybe_failure_reason.as_ref() == Some(&reason))
            .expect("peer outcome with expected failure reason")
    }

    fn assert_reason_without_block_credit(summary: &SyncRunSummary, reason: PeerFailureReason) {
        let outcome = outcome_with_reason(summary, reason);
        assert_eq!(outcome.contribution.blocks_received, 0);
    }

    fn assert_first_peer_backoff(runtime: &DurableSyncRuntime) {
        assert!(runtime.peer_backoff.contains_key("127.0.0.1:18444"));
    }

    fn persist_previous_active_chain_credit(
        runtime: &mut DurableSyncRuntime,
        observed_at_unix_seconds: i64,
    ) -> ProgressCreditEvidence {
        let mut previous_summary = runtime.snapshot_summary();
        previous_summary.messages_processed = 3;
        previous_summary.headers_received = 1;
        previous_summary.blocks_received = 1;
        previous_summary
            .peer_outcomes
            .push(peer_outcome_with_contribution(
                SyncPeerAddress::manual("127.0.0.1", 18_444),
                PeerSyncState::Connected,
                1,
                None,
                PeerContribution {
                    messages_processed: 3,
                    headers_received: 1,
                    blocks_received: 1,
                },
            ));
        let previous_state = runtime
            .durable_sync_state_for_summary(
                &previous_summary,
                SyncLifecycleState::Active,
                None,
                observed_at_unix_seconds,
            )
            .expect("previous durable status");
        let previous_credit = available_progress_credit(&previous_state).clone();
        runtime
            .persist_durable_sync_state(previous_state)
            .expect("persist previous status");
        previous_credit
    }

    #[test]
    fn phase70_notfound_releases_inflight_and_rotates_to_second_peer() {
        // Arrange
        let path = temp_store_path("phase70-peer-notfound-rotation");
        remove_dir_if_exists(&path);
        let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
        let child = build_block(block_hash(&genesis.header), 1);
        let child_hash = block_hash(&child.header);
        save_best_chain_with_active_blocks(&path, &[(&genesis, 0), (&child, 1)], &[(&genesis, 0)]);
        let store = FjallNodeStore::open(&path).expect("reopen store");
        let mut runtime = DurableSyncRuntime::open(store, rotation_config()).expect("runtime");
        let mut transport = ScriptedTransport::new(vec![
            vec![
                WireNetworkMessage::Version(VersionMessage {
                    start_height: 1,
                    ..VersionMessage::default()
                }),
                WireNetworkMessage::Verack,
                notfound_for_block(child_hash),
            ],
            version_verack_script(1),
        ]);

        // Act
        let summary = runtime
            .sync_once(&mut transport, i64::from(child.header.time))
            .expect("sync");
        let requested_hashes = getdata_block_hashes(&transport.sent_messages());

        // Assert
        assert_eq!(summary.attempted_peers, 2);
        assert_eq!(summary.connected_peers, 1);
        assert_eq!(summary.peer_outcomes.len(), 2);
        assert_reason_without_block_credit(&summary, PeerFailureReason::BlockNotFound);
        assert_eq!(summary.peer_outcomes[1].state, PeerSyncState::Connected);
        assert!(
            requested_hashes
                .iter()
                .filter(|hash| **hash == child_hash)
                .count()
                >= 2
        );
        assert!(runtime.inflight_blocks.is_empty());
        assert!(runtime.network.peer_requested_blocks(1).is_err());
        assert_first_peer_backoff(&runtime);

        remove_dir_if_exists(&path);
    }

    #[test]
    fn phase78_stale_inflight_cleanup_preserves_prior_credit_and_rotates_peer() {
        // Arrange
        let path = temp_store_path("phase78-stale-inflight-prior-credit");
        remove_dir_if_exists(&path);
        let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
        let child = build_block(block_hash(&genesis.header), 1);
        save_best_chain_with_active_blocks(
            &path,
            &[(&genesis, 0), (&child, 1)],
            &[(&genesis, 0), (&child, 1)],
        );
        let store = FjallNodeStore::open(&path).expect("store");
        let mut runtime = DurableSyncRuntime::open(store, rotation_config()).expect("runtime");
        runtime
            .inflight_blocks
            .insert(BlockHash::from_byte_array([78_u8; 32]));
        let previous_credit =
            persist_previous_active_chain_credit(&mut runtime, i64::from(child.header.time));
        assert_eq!(
            serialized_label(RejectedProgressActivityKind::InFlightRequest),
            "in_flight_request"
        );
        assert_rejected_activity(
            &previous_credit,
            RejectedProgressActivityKind::InFlightRequest,
        );
        let observed_at_unix_seconds = i64::from(child.header.time) + 10_000;
        let mut transport = ScriptedTransport::new(vec![Vec::new(), version_verack_script(1)]);

        // Act
        let summary = runtime
            .sync_once(&mut transport, observed_at_unix_seconds)
            .expect("sync with stale in-flight and replacement peer");
        let state = runtime
            .durable_sync_state_for_summary(
                &summary,
                SyncLifecycleState::Active,
                None,
                observed_at_unix_seconds,
            )
            .expect("durable stale in-flight status");

        // Assert
        assert_eq!(summary.attempted_peers, 2);
        assert_eq!(summary.peer_outcomes[0].state, PeerSyncState::Stalled);
        assert_eq!(summary.peer_outcomes[1].state, PeerSyncState::Connected);
        assert_progress_credit_unavailable(&state);
        let last_work = available_last_useful_work(&state);
        assert_eq!(
            last_work.kind,
            ProgressCreditKind::ValidatedDurableActiveChain
        );
        assert_eq!(last_work.credited_validated_active_chain_height, 1);
        assert_rejected_activity(last_work, RejectedProgressActivityKind::InFlightRequest);
        assert_eq!(
            state.sync.no_progress_diagnosis,
            FieldAvailability::available(NoProgressDiagnosis::StaleInflightCleanup)
        );
        let last_peer_contribution = available_last_peer_contribution(&state);
        assert_eq!(
            last_peer_contribution.kind,
            PeerContributionKind::MessagesOnly
        );
        let stall = available_stall_diagnosis(&state);
        assert_eq!(
            serialized_label(stall.stalled_subsystem),
            "slow_or_stalled_peers"
        );
        assert_eq!(
            stall.stalled_subsystem,
            StalledSubsystem::SlowOrStalledPeers
        );
        assert_first_peer_backoff(&runtime);

        remove_dir_if_exists(&path);
    }

    #[test]
    fn phase78_no_credit_peer_rotation_keeps_last_peer_contribution_without_credit() {
        // Arrange
        let path = temp_store_path("phase78-no-credit-peer-rotation");
        remove_dir_if_exists(&path);
        let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
        let child = build_block(block_hash(&genesis.header), 1);
        let child_hash = block_hash(&child.header);
        save_best_chain_with_active_blocks(&path, &[(&genesis, 0), (&child, 1)], &[(&genesis, 0)]);
        let store = FjallNodeStore::open(&path).expect("store");
        let mut runtime = DurableSyncRuntime::open(store, rotation_config()).expect("runtime");
        let previous_credit =
            persist_previous_active_chain_credit(&mut runtime, i64::from(genesis.header.time));
        assert_eq!(previous_credit.credited_validated_active_chain_height, 0);
        let mut transport = ScriptedTransport::new(vec![
            vec![
                WireNetworkMessage::Version(VersionMessage {
                    start_height: 1,
                    ..VersionMessage::default()
                }),
                WireNetworkMessage::Verack,
                notfound_for_block(child_hash),
            ],
            version_verack_script(1),
        ]);

        // Act
        let summary = runtime
            .sync_once(&mut transport, i64::from(child.header.time))
            .expect("sync with no-credit peer rotation");
        let state = runtime
            .durable_sync_state_for_summary(
                &summary,
                SyncLifecycleState::Active,
                None,
                i64::from(child.header.time),
            )
            .expect("durable no-credit rotation status");

        // Assert
        assert_eq!(summary.attempted_peers, 2);
        assert_eq!(
            summary.peer_outcomes[0].maybe_failure_reason,
            Some(PeerFailureReason::BlockNotFound)
        );
        assert_eq!(summary.peer_outcomes[1].state, PeerSyncState::Connected);
        assert_progress_credit_unavailable(&state);
        assert_eq!(
            available_last_useful_work(&state).credited_validated_active_chain_height,
            0
        );
        let last_peer_contribution = available_last_peer_contribution(&state);
        assert_eq!(
            last_peer_contribution.peer,
            SyncPeerAddress::manual("127.0.0.1", 18_445).label()
        );
        assert_eq!(
            last_peer_contribution.kind,
            PeerContributionKind::MessagesOnly
        );
        let stall = available_stall_diagnosis(&state);
        assert_eq!(
            serialized_label(stall.stalled_subsystem),
            "slow_or_stalled_peers"
        );
        assert_eq!(
            stall.stalled_subsystem,
            StalledSubsystem::SlowOrStalledPeers
        );
        assert_first_peer_backoff(&runtime);

        remove_dir_if_exists(&path);
    }

    #[test]
    fn phase78_validation_stall_classifies_validation_subsystem() {
        // Arrange
        let path = temp_store_path("phase78-validation-stall");
        remove_dir_if_exists(&path);
        let valid_block = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
        let mut invalid_block = valid_block.clone();
        invalid_block.transactions[0].outputs[0].value =
            Amount::from_sats(51).expect("valid amount");
        let first_peer_script = vec![
            WireNetworkMessage::Version(VersionMessage {
                start_height: 0,
                ..VersionMessage::default()
            }),
            WireNetworkMessage::Verack,
            WireNetworkMessage::Headers(HeadersMessage {
                headers: vec![valid_block.header.clone()],
            }),
            WireNetworkMessage::Block(invalid_block),
        ];
        let store = FjallNodeStore::open(&path).expect("store");
        let mut runtime = DurableSyncRuntime::open(store, rotation_config()).expect("runtime");
        let mut transport =
            ScriptedTransport::new(vec![first_peer_script, version_verack_script(0)]);

        // Act
        let summary = runtime
            .sync_once(&mut transport, i64::from(valid_block.header.time))
            .expect("sync with invalid block");
        let state = runtime
            .durable_sync_state_for_summary(
                &summary,
                SyncLifecycleState::Active,
                None,
                i64::from(valid_block.header.time),
            )
            .expect("durable validation stall status");

        // Assert
        assert_eq!(
            summary.peer_outcomes[0].maybe_failure_reason,
            Some(PeerFailureReason::InvalidBlock)
        );
        assert_progress_credit_unavailable(&state);
        let stall = available_stall_diagnosis(&state);
        assert_eq!(serialized_label(stall.stalled_subsystem), "validation");
        assert_eq!(stall.stalled_subsystem, StalledSubsystem::Validation);
        assert_eq!(
            stall.evidence_basis,
            vec![
                "no_progress_diagnosis=BehindAwaitingHeaders".to_string(),
                "recovery_category=invalid_peer_data".to_string(),
                "peer_failure_reason=invalid_block".to_string(),
            ]
        );

        remove_dir_if_exists(&path);
    }

    #[test]
    fn phase70_malformed_block_releases_inflight_and_rotates() {
        // Arrange
        let path = temp_store_path("phase70-peer-malformed-rotation");
        remove_dir_if_exists(&path);
        let block = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
        let block_hash = block_hash(&block.header);
        let first_peer_script = headers_script(0, vec![block.header.clone()]);
        let store = FjallNodeStore::open(&path).expect("store");
        let mut runtime = DurableSyncRuntime::open(store, rotation_config()).expect("runtime");
        let mut transport = ErrorAfterMessagesTransport::new(
            vec![first_peer_script, version_verack_script(0)],
            SyncRuntimeError::Network {
                message: "malformed block payload".to_string(),
            },
            1,
        );

        // Act
        let summary = runtime
            .sync_once(&mut transport, i64::from(block.header.time))
            .expect("sync");
        let requested_hashes = getdata_block_hashes(&transport.sent_messages());

        // Assert
        assert_eq!(summary.attempted_peers, 2);
        assert_eq!(summary.failed_peers, 1);
        assert_eq!(summary.connected_peers, 1);
        assert_reason_without_block_credit(&summary, PeerFailureReason::MalformedBlock);
        assert_eq!(summary.peer_outcomes[1].state, PeerSyncState::Connected);
        assert!(
            requested_hashes
                .iter()
                .filter(|hash| **hash == block_hash)
                .count()
                >= 2
        );
        assert!(runtime.inflight_blocks.is_empty());
        assert!(runtime.network.peer_requested_blocks(1).is_err());
        assert_first_peer_backoff(&runtime);

        remove_dir_if_exists(&path);
    }

    #[test]
    fn phase70_invalid_block_releases_inflight_and_rotates() {
        // Arrange
        let path = temp_store_path("phase70-peer-invalid-rotation");
        remove_dir_if_exists(&path);
        let valid_block = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
        let block_hash = block_hash(&valid_block.header);
        let mut invalid_block = valid_block.clone();
        invalid_block.transactions[0].outputs[0].value =
            Amount::from_sats(51).expect("valid amount");
        let first_peer_script = vec![
            WireNetworkMessage::Version(VersionMessage {
                start_height: 0,
                ..VersionMessage::default()
            }),
            WireNetworkMessage::Verack,
            WireNetworkMessage::Headers(HeadersMessage {
                headers: vec![valid_block.header.clone()],
            }),
            WireNetworkMessage::Block(invalid_block),
        ];
        let store = FjallNodeStore::open(&path).expect("store");
        let mut runtime = DurableSyncRuntime::open(store, rotation_config()).expect("runtime");
        let mut transport =
            ScriptedTransport::new(vec![first_peer_script, version_verack_script(0)]);

        // Act
        let summary = runtime
            .sync_once(&mut transport, i64::from(valid_block.header.time))
            .expect("sync");
        let requested_hashes = getdata_block_hashes(&transport.sent_messages());

        // Assert
        assert_eq!(summary.attempted_peers, 2);
        assert_eq!(summary.failed_peers, 1);
        assert_eq!(summary.connected_peers, 1);
        assert_reason_without_block_credit(&summary, PeerFailureReason::InvalidBlock);
        assert_eq!(summary.peer_outcomes[1].state, PeerSyncState::Connected);
        assert!(
            requested_hashes
                .iter()
                .filter(|hash| **hash == block_hash)
                .count()
                >= 2
        );
        assert!(runtime.inflight_blocks.is_empty());
        assert!(runtime.network.peer_requested_blocks(1).is_err());
        assert_first_peer_backoff(&runtime);

        remove_dir_if_exists(&path);
    }

    #[test]
    fn phase70_duplicate_block_releases_inflight_without_credit() {
        // Arrange
        let path = temp_store_path("phase70-peer-duplicate-no-credit");
        remove_dir_if_exists(&path);
        let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
        let child = build_block(block_hash(&genesis.header), 1);
        let child_hash = block_hash(&child.header);
        save_best_chain_with_active_blocks(
            &path,
            &[(&genesis, 0), (&child, 1)],
            &[(&genesis, 0), (&child, 1)],
        );
        let store = FjallNodeStore::open(&path).expect("reopen store");
        let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
        runtime.inflight_blocks.insert(child_hash);
        let mut transport = ScriptedTransport::new(vec![vec![
            WireNetworkMessage::Version(VersionMessage {
                start_height: 1,
                ..VersionMessage::default()
            }),
            WireNetworkMessage::Verack,
            WireNetworkMessage::Block(child.clone()),
        ]]);

        // Act
        let summary = runtime
            .sync_once(&mut transport, i64::from(child.header.time))
            .expect("sync");

        // Assert
        assert_reason_without_block_credit(&summary, PeerFailureReason::DuplicateBlock);
        assert!(runtime.inflight_blocks.is_empty());
        assert_first_peer_backoff(&runtime);

        remove_dir_if_exists(&path);
    }

    #[test]
    fn phase70_disconnected_block_releases_inflight_without_credit() {
        // Arrange
        let path = temp_store_path("phase70-peer-disconnected-no-credit");
        remove_dir_if_exists(&path);
        let block = build_block(BlockHash::from_byte_array([7_u8; 32]), 1);
        let block_hash = block_hash(&block.header);
        let store = FjallNodeStore::open(&path).expect("store");
        let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
        runtime.inflight_blocks.insert(block_hash);
        let mut transport = ScriptedTransport::new(vec![vec![
            WireNetworkMessage::Version(VersionMessage {
                start_height: 1,
                ..VersionMessage::default()
            }),
            WireNetworkMessage::Verack,
            WireNetworkMessage::Block(block.clone()),
        ]]);

        // Act
        let summary = runtime
            .sync_once(&mut transport, i64::from(block.header.time))
            .expect("sync");

        // Assert
        assert_reason_without_block_credit(&summary, PeerFailureReason::DisconnectedBlock);
        assert!(runtime.inflight_blocks.is_empty());
        assert_first_peer_backoff(&runtime);

        remove_dir_if_exists(&path);
    }

    #[test]
    fn phase70_non_extending_block_releases_inflight_without_credit() {
        // Arrange
        let path = temp_store_path("phase70-peer-non-extending-no-credit");
        remove_dir_if_exists(&path);
        let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
        let side_block = build_block(BlockHash::from_byte_array([42_u8; 32]), 1);
        let side_hash = block_hash(&side_block.header);
        save_best_chain_with_active_blocks(&path, &[(&genesis, 0)], &[(&genesis, 0)]);
        let store = FjallNodeStore::open(&path).expect("reopen store");
        let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
        runtime.inflight_blocks.insert(side_hash);
        let mut transport = ScriptedTransport::new(vec![vec![
            WireNetworkMessage::Version(VersionMessage {
                start_height: 1,
                ..VersionMessage::default()
            }),
            WireNetworkMessage::Verack,
            WireNetworkMessage::Block(side_block.clone()),
        ]]);

        // Act
        let summary = runtime
            .sync_once(&mut transport, i64::from(side_block.header.time))
            .expect("sync");

        // Assert
        assert_reason_without_block_credit(&summary, PeerFailureReason::NonExtendingBlock);
        assert!(runtime.inflight_blocks.is_empty());
        assert_first_peer_backoff(&runtime);

        remove_dir_if_exists(&path);
    }

    #[test]
    fn phase70_stalled_peer_backoff_does_not_consume_rotation_slot() {
        // Arrange
        let path = temp_store_path("phase70-peer-stall-rotation");
        remove_dir_if_exists(&path);
        let store = FjallNodeStore::open(&path).expect("store");
        let mut runtime = DurableSyncRuntime::open(store, rotation_config()).expect("runtime");
        let mut transport = ScriptedTransport::new(vec![
            Vec::new(),
            version_verack_script(0),
            version_verack_script(0),
        ]);

        // Act
        let stalled_summary = runtime
            .sync_once(&mut transport, 1_777_225_300)
            .expect("first sync");
        let waiting_summary = runtime
            .sync_once(&mut transport, 1_777_225_301)
            .expect("second sync");

        // Assert
        assert_eq!(stalled_summary.attempted_peers, 2);
        assert_eq!(stalled_summary.connected_peers, 1);
        assert_eq!(
            stalled_summary.peer_outcomes[0].state,
            PeerSyncState::Stalled
        );
        assert_eq!(
            stalled_summary.peer_outcomes[0].maybe_failure_reason,
            Some(PeerFailureReason::Stall)
        );
        assert_eq!(
            stalled_summary.peer_outcomes[1].state,
            PeerSyncState::Connected
        );
        assert!(
            stalled_summary.health_signals.iter().any(|signal| {
                signal.message == "peer stalled before sending more sync messages"
            })
        );
        assert_eq!(
            waiting_summary.peer_outcomes[0].state,
            PeerSyncState::Waiting
        );
        assert_eq!(
            waiting_summary.peer_outcomes[0].maybe_failure_reason,
            Some(PeerFailureReason::RetryBackoff)
        );
        assert!(waiting_summary.health_signals.iter().any(|signal| {
            signal.message == "peer waiting for retry backoff before next attempt"
        }));

        remove_dir_if_exists(&path);
    }

    #[test]
    fn phase70_incompatible_peer_rotates_with_typed_backoff() {
        // Arrange
        let path = temp_store_path("phase70-peer-incompatible-rotation");
        remove_dir_if_exists(&path);
        let duplicate_version_script = vec![
            WireNetworkMessage::Version(VersionMessage {
                start_height: 0,
                ..VersionMessage::default()
            }),
            WireNetworkMessage::Version(VersionMessage {
                start_height: 0,
                ..VersionMessage::default()
            }),
        ];
        let store = FjallNodeStore::open(&path).expect("store");
        let mut runtime = DurableSyncRuntime::open(store, rotation_config()).expect("runtime");
        let mut transport =
            ScriptedTransport::new(vec![duplicate_version_script, version_verack_script(0)]);

        // Act
        let summary = runtime
            .sync_once(&mut transport, 1_777_225_302)
            .expect("sync");

        // Assert
        assert_eq!(summary.attempted_peers, 2);
        assert_eq!(summary.failed_peers, 1);
        assert_eq!(summary.connected_peers, 1);
        assert_eq!(
            summary.peer_outcomes[0].maybe_failure_reason,
            Some(PeerFailureReason::Compatibility)
        );
        assert_eq!(summary.peer_outcomes[1].state, PeerSyncState::Connected);
        assert_first_peer_backoff(&runtime);

        remove_dir_if_exists(&path);
    }

    #[test]
    fn phase70_disconnect_backoff_reports_waiting_and_tries_other_peer() {
        // Arrange
        let path = temp_store_path("phase70-peer-disconnect-backoff");
        remove_dir_if_exists(&path);
        let store = FjallNodeStore::open(&path).expect("store");
        let mut runtime = DurableSyncRuntime::open(store, rotation_config()).expect("runtime");
        let mut transport = ScriptedTransport::with_connect_results(vec![
            Err(SyncRuntimeError::Network {
                message: "scripted disconnect".to_string(),
            }),
            Ok(version_verack_script(0)),
            Ok(version_verack_script(0)),
        ]);

        // Act
        let failed_summary = runtime
            .sync_once(&mut transport, 1_777_225_303)
            .expect("first sync");
        let waiting_summary = runtime
            .sync_once(&mut transport, 1_777_225_304)
            .expect("second sync");

        // Assert
        assert_eq!(failed_summary.attempted_peers, 2);
        assert_eq!(failed_summary.failed_peers, 1);
        assert_eq!(
            failed_summary.peer_outcomes[0].maybe_failure_reason,
            Some(PeerFailureReason::Network)
        );
        assert_eq!(
            failed_summary.peer_outcomes[1].state,
            PeerSyncState::Connected
        );
        assert_eq!(waiting_summary.attempted_peers, 1);
        assert_eq!(
            waiting_summary.peer_outcomes[0].state,
            PeerSyncState::Waiting
        );
        assert_eq!(
            waiting_summary.peer_outcomes[0].maybe_failure_reason,
            Some(PeerFailureReason::RetryBackoff)
        );
        assert_eq!(
            waiting_summary.peer_outcomes[1].state,
            PeerSyncState::Connected
        );

        remove_dir_if_exists(&path);
    }
}

#[test]
fn block_inflight_invalid_block_releases_runtime_and_peer_inflight_for_retry() {
    // Arrange
    let path = temp_store_path("block-inflight-invalid");
    remove_dir_if_exists(&path);
    let valid_block = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let block_hash = block_hash(&valid_block.header);
    let mut invalid_block = valid_block.clone();
    invalid_block.transactions[0].outputs[0].value = Amount::from_sats(51).expect("valid amount");
    let first_peer_script = vec![
        WireNetworkMessage::Version(VersionMessage {
            start_height: 0,
            ..VersionMessage::default()
        }),
        WireNetworkMessage::Verack,
        WireNetworkMessage::Headers(HeadersMessage {
            headers: vec![valid_block.header.clone()],
        }),
        WireNetworkMessage::Block(invalid_block),
    ];
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(store, two_peer_sync_config()).expect("runtime");
    let mut transport = ScriptedTransport::new(vec![first_peer_script, version_verack_script(0)]);

    // Act
    let summary = runtime
        .sync_once(&mut transport, i64::from(valid_block.header.time))
        .expect("sync");
    let requested_hashes = getdata_block_hashes(&transport.sent_messages());

    // Assert
    assert_eq!(summary.failed_peers, 1);
    assert!(summary.peer_outcomes.iter().any(|outcome| {
        outcome.maybe_failure_reason == Some(PeerFailureReason::InvalidBlock)
            && outcome.contribution.blocks_received == 0
    }));
    assert!(
        requested_hashes
            .iter()
            .filter(|hash| **hash == block_hash)
            .count()
            >= 2
    );
    assert!(runtime.inflight_blocks.is_empty());
    assert!(
        runtime
            .store()
            .load_block(block_hash)
            .expect("load invalid block")
            .is_none()
    );
    assert_eq!(summary.downloaded_block_height, 0);
    assert_eq!(summary.best_block_height, 0);

    remove_dir_if_exists(&path);
}

#[test]
fn block_inflight_malformed_block_releases_runtime_and_peer_inflight_for_retry() {
    // Arrange
    let path = temp_store_path("block-inflight-malformed");
    remove_dir_if_exists(&path);
    let block = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let block_hash = block_hash(&block.header);
    let first_peer_script = headers_script(0, vec![block.header.clone()]);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(store, two_peer_sync_config()).expect("runtime");
    let mut transport = ErrorAfterMessagesTransport::new(
        vec![first_peer_script, version_verack_script(0)],
        SyncRuntimeError::Network {
            message: "malformed block payload".to_string(),
        },
        1,
    );

    // Act
    let summary = runtime
        .sync_once(&mut transport, i64::from(block.header.time))
        .expect("sync");
    let requested_hashes = getdata_block_hashes(&transport.sent_messages());

    // Assert
    assert_eq!(summary.failed_peers, 1);
    assert!(summary.peer_outcomes.iter().any(|outcome| {
        outcome.maybe_failure_reason == Some(PeerFailureReason::MalformedBlock)
            && outcome.contribution.blocks_received == 0
    }));
    assert!(
        requested_hashes
            .iter()
            .filter(|hash| **hash == block_hash)
            .count()
            >= 2
    );
    assert!(runtime.inflight_blocks.is_empty());
    assert!(runtime.network.peer_requested_blocks(1).is_err());
    assert_eq!(summary.downloaded_block_height, 0);
    assert_eq!(summary.best_block_height, 0);

    remove_dir_if_exists(&path);
}

#[test]
fn sync_summary_projects_metric_samples() {
    // Arrange
    let summary = SyncRunSummary {
        target_outbound_peers: 1,
        maybe_target_header_height: None,
        attempted_peers: 2,
        connected_peers: 1,
        failed_peers: 1,
        messages_processed: 7,
        headers_received: 3,
        blocks_received: 2,
        best_header_height: 42,
        downloaded_block_height: 41,
        best_block_height: 40,
        maybe_downloaded_block_hash: None,
        maybe_connected_block_hash: None,
        maybe_validated_active_chain_work: None,
        peer_outcomes: Vec::new(),
        health_signals: Vec::new(),
        maybe_stop_reason: None,
        maybe_reconcile_progress: None,
    };

    // Act
    let samples = summary.metric_samples(1_777_225_022);

    // Assert
    assert_eq!(
        samples,
        vec![
            MetricSample::new(MetricKind::HeaderHeight, 42.0, 1_777_225_022),
            MetricSample::new(MetricKind::DownloadedBlockHeight, 41.0, 1_777_225_022),
            MetricSample::new(MetricKind::ConnectedBlockHeight, 40.0, 1_777_225_022),
            MetricSample::new(MetricKind::ValidatedActiveChainHeight, 40.0, 1_777_225_022,),
            MetricSample::new(MetricKind::SyncHeight, 40.0, 1_777_225_022),
            MetricSample::new(MetricKind::PeerCount, 1.0, 1_777_225_022),
        ]
    );
}

#[test]
fn sync_summary_projects_progress_signal_and_last_successful_timestamp() {
    // Arrange
    let mut outcome = peer_outcome(
        SyncPeerAddress::manual("127.0.0.1", 18_444),
        PeerSyncState::Connected,
        1,
        None,
        None,
    );
    outcome.contribution.headers_received = 2;
    outcome.maybe_last_activity_unix_seconds = Some(1_777_225_099);
    let summary = SyncRunSummary {
        target_outbound_peers: 1,
        maybe_target_header_height: None,
        attempted_peers: 1,
        connected_peers: 1,
        failed_peers: 0,
        messages_processed: 3,
        headers_received: 2,
        blocks_received: 0,
        best_header_height: 42,
        downloaded_block_height: 0,
        best_block_height: 0,
        maybe_downloaded_block_hash: None,
        maybe_connected_block_hash: None,
        maybe_validated_active_chain_work: None,
        peer_outcomes: vec![outcome],
        health_signals: Vec::new(),
        maybe_stop_reason: None,
        maybe_reconcile_progress: None,
    };

    // Act
    let sync_status = summary.sync_status(SyncNetwork::Regtest);
    let records = summary.structured_log_records(1_777_225_100);

    // Assert
    assert_eq!(
        sync_status.progress_signal,
        FieldAvailability::available(SyncProgressSignal::HeaderProgress)
    );
    assert_eq!(
        sync_status.last_successful_progress_unix_seconds,
        FieldAvailability::available(1_777_225_099)
    );
    assert!(records.iter().any(|record| {
        record.message.contains("progress_signal=header_progress")
            && record.message.contains("last_progress=1777225099")
    }));
}

#[test]
fn sync_summary_projects_structured_log_records() {
    // Arrange
    let summary = SyncRunSummary {
        target_outbound_peers: 2,
        maybe_target_header_height: None,
        attempted_peers: 3,
        connected_peers: 2,
        failed_peers: 1,
        messages_processed: 9,
        headers_received: 4,
        blocks_received: 2,
        best_header_height: 44,
        downloaded_block_height: 44,
        best_block_height: 43,
        maybe_downloaded_block_hash: None,
        maybe_connected_block_hash: None,
        maybe_validated_active_chain_work: None,
        peer_outcomes: vec![
            peer_outcome(
                SyncPeerAddress::manual("127.0.0.1", 18_444),
                PeerSyncState::Stalled,
                1,
                Some(PeerFailureReason::Stall),
                None,
            ),
            peer_outcome(
                SyncPeerAddress::manual("203.0.113.10", 18_444),
                PeerSyncState::Failed,
                3,
                Some(PeerFailureReason::Network),
                Some("scripted network failure".to_string()),
            ),
            peer_outcome(
                SyncPeerAddress::manual("198.51.100.9", 18_444),
                PeerSyncState::Connected,
                2,
                None,
                None,
            ),
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
        maybe_stop_reason: None,
        maybe_reconcile_progress: None,
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
    assert!(summary_record.message.contains("header=44"));
    assert!(summary_record.message.contains("downloaded=44"));
    assert!(summary_record.message.contains("connected=43"));
    assert!(
        summary_record
            .message
            .contains("progress_signal=block_progress")
    );
    assert!(summary_record.message.contains("last_progress=unavailable"));
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
    assert!(records.iter().all(|record| record.message.len() <= 192));
    assert!(records.iter().all(|record| {
        !record.message.contains("127.0.0.1")
            && !record.message.contains("203.0.113")
            && !record.message.contains("cookie")
            && !record.message.contains("/tmp/")
    }));
}

#[test]
fn phase62_structured_logs_keep_bounded_cycle_facts() {
    // Arrange
    let mut summary = SyncRunSummary {
        target_outbound_peers: 4,
        maybe_target_header_height: Some(840_123),
        attempted_peers: 3,
        connected_peers: 2,
        failed_peers: 1,
        messages_processed: 9,
        headers_received: 4,
        blocks_received: 2,
        best_header_height: 840_123,
        downloaded_block_height: 840_120,
        best_block_height: 840_119,
        maybe_downloaded_block_hash: None,
        maybe_connected_block_hash: None,
        maybe_validated_active_chain_work: None,
        peer_outcomes: Vec::new(),
        health_signals: Vec::new(),
        maybe_stop_reason: Some(SyncStopReason::NoProgress {
            rounds_completed: 2,
        }),
        maybe_reconcile_progress: None,
    };
    summary.peer_outcomes.push(peer_outcome(
        SyncPeerAddress::manual("198.51.100.44", 18_444),
        PeerSyncState::Connected,
        1,
        None,
        None,
    ));

    // Act
    let records = summary.structured_log_records(1_777_225_100);
    let summary_text = records
        .iter()
        .filter(|record| record.source == "sync" && record.level == StructuredLogLevel::Info)
        .map(|record| record.message.as_str())
        .collect::<Vec<_>>()
        .join(" ");

    // Assert
    for expected in [
        "target_outbound_peers=4",
        "target_header_height=840123",
        "attempted_peers=3",
        "connected_peers=2",
        "failed_peers=1",
        "messages_processed=9",
        "headers_received=4",
        "blocks_received=2",
        "header=840123",
        "downloaded=840120",
        "connected=840119",
        "progress_signal=block_progress",
        "last_progress=unavailable",
        "latest_stop_reason=no_progress",
        "recovery_category=unavailable",
    ] {
        assert!(
            summary_text.contains(expected),
            "missing structured log fact: {expected}"
        );
    }
    assert!(records.iter().all(|record| record.message.len() <= 192));
}

#[test]
fn sync_summary_logs_stop_reason_when_available() {
    // Arrange
    let mut summary = SyncRunSummary::empty(0, 0, 1);
    summary.maybe_stop_reason = Some(SyncStopReason::NoProgress {
        rounds_completed: 2,
    });

    // Act
    let records = summary.structured_log_records(1_777_225_101);

    // Assert
    assert!(
        records
            .iter()
            .any(|record| record.message == "sync stop reason=no_progress")
    );
    assert!(records.iter().all(|record| record.message.len() <= 192));
}

#[test]
fn phase62_status_and_structured_logs_agree_on_configured_targets() {
    // Arrange
    let path = temp_store_path("phase62-target-agreement");
    let log_dir = path.join("logs");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let runtime = DurableSyncRuntime::open(
        store,
        SyncRuntimeConfig {
            maybe_target_header_height: Some(840_123),
            maybe_log_dir: Some(log_dir.clone()),
            max_rounds: 5,
            ..sync_config()
        },
    )
    .expect("runtime");
    let mut summary = SyncRunSummary::empty(840_123, 840_120, 4);
    summary.maybe_stop_reason = Some(SyncStopReason::TargetHeaderReached {
        target_header_height: 840_123,
        best_header_height: 840_123,
    });

    // Act
    runtime.write_summary_logs(&mut summary, 1_777_225_102);
    let state = runtime
        .durable_sync_state_for_summary(&summary, SyncLifecycleState::Active, None, 1_777_225_102)
        .expect("durable status");
    let records = load_structured_log_records(&log_dir);
    let summary_text = records
        .iter()
        .filter(|record| record.source == "sync")
        .map(|record| record.message.as_str())
        .collect::<Vec<_>>()
        .join(" ");

    // Assert
    let FieldAvailability::Available(configured_targets) = state.sync.configured_targets else {
        panic!("configured targets should be available");
    };
    assert_eq!(configured_targets.maybe_target_header_height, Some(840_123));
    assert_eq!(configured_targets.target_outbound_peers, 4);
    let FieldAvailability::Available(stop_reason) = state.sync.latest_stop_reason else {
        panic!("latest stop reason should be available");
    };
    assert_eq!(stop_reason.label, "target_header_reached");
    assert!(summary_text.contains("target_header_height=840123"));
    assert!(summary_text.contains("latest_stop_reason=target_header_reached"));

    remove_dir_if_exists(&path);
}

#[test]
fn sync_summary_status_keeps_connected_height_alias_with_hashes() {
    // Arrange
    let downloaded_hash =
        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string();
    let connected_hash =
        "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb".to_string();
    let summary = SyncRunSummary {
        target_outbound_peers: 4,
        maybe_target_header_height: None,
        attempted_peers: 2,
        connected_peers: 1,
        failed_peers: 0,
        messages_processed: 9,
        headers_received: 7,
        blocks_received: 3,
        best_header_height: 30,
        downloaded_block_height: 27,
        best_block_height: 25,
        maybe_downloaded_block_hash: Some(downloaded_hash.clone()),
        maybe_connected_block_hash: Some(connected_hash.clone()),
        maybe_validated_active_chain_work: Some("26".to_string()),
        peer_outcomes: Vec::new(),
        health_signals: Vec::new(),
        maybe_stop_reason: None,
        maybe_reconcile_progress: None,
    };

    // Act
    let status = summary.sync_status(SyncNetwork::Regtest);

    // Assert
    assert_eq!(
        status.sync_progress,
        crate::FieldAvailability::available(crate::status::SyncProgress {
            header_height: 30,
            block_height: 25,
            downloaded_block_height: 27,
            connected_block_height: 25,
            validated_active_chain_height: 25,
            maybe_downloaded_block_hash: Some(downloaded_hash),
            maybe_connected_block_hash: Some(connected_hash.clone()),
            maybe_validated_active_chain_hash: Some(connected_hash),
            maybe_validated_active_chain_work: Some("26".to_string()),
            progress_ratio: 25.0 / 30.0,
            messages_processed: 9,
            headers_received: 7,
            blocks_received: 3,
        })
    );
}

#[test]
fn sync_summary_status_projections_include_counters() {
    // Arrange
    let summary = SyncRunSummary {
        target_outbound_peers: 4,
        maybe_target_header_height: None,
        attempted_peers: 4,
        connected_peers: 3,
        failed_peers: 1,
        messages_processed: 12,
        headers_received: 7,
        blocks_received: 5,
        best_header_height: 100,
        downloaded_block_height: 75,
        best_block_height: 25,
        maybe_downloaded_block_hash: None,
        maybe_connected_block_hash: None,
        maybe_validated_active_chain_work: None,
        peer_outcomes: Vec::new(),
        health_signals: Vec::new(),
        maybe_stop_reason: None,
        maybe_reconcile_progress: None,
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
            downloaded_block_height: 75,
            connected_block_height: 25,
            validated_active_chain_height: 25,
            maybe_downloaded_block_hash: None,
            maybe_connected_block_hash: None,
            maybe_validated_active_chain_hash: None,
            maybe_validated_active_chain_work: None,
            progress_ratio: 0.25,
            messages_processed: 12,
            headers_received: 7,
            blocks_received: 5,
        })
    );
    assert_eq!(
        sync_status.progress_signal,
        FieldAvailability::available(SyncProgressSignal::BlockProgress)
    );
    assert!(matches!(
        sync_status.last_successful_progress_unix_seconds,
        FieldAvailability::Unavailable { .. }
    ));
    assert_eq!(
        peer_status.peer_counts,
        FieldAvailability::available(crate::status::PeerCounts {
            inbound: 0,
            outbound: 3,
        })
    );
    assert_eq!(
        sync_status.resource_pressure,
        FieldAvailability::available(SyncResourcePressure {
            blocks_in_flight: 0,
            max_header_requests_in_flight_per_peer: 1,
            max_headers_per_message: 2_000,
            max_blocks_in_flight_per_peer: 0,
            max_blocks_in_flight_total: 0,
            max_messages_per_peer: 0,
            max_sync_rounds: 0,
            outbound_peers: 3,
            target_outbound_peers: 4,
        })
    );
}

#[test]
fn phase69_sync_status_defaults_tip_and_stay_current_fields() {
    // Arrange
    let payload = serde_json::json!({
        "network": { "state": "available", "value": "regtest" },
        "chain_tip": {
            "state": "unavailable",
            "value": { "reason": "chain tip unavailable" }
        },
        "sync_progress": {
            "state": "unavailable",
            "value": { "reason": "sync progress unavailable" }
        },
        "lifecycle": { "state": "available", "value": "active" },
        "phase": { "state": "available", "value": "headers" },
        "configured_targets": {
            "state": "available",
            "value": {
                "target_outbound_peers": 1,
                "maybe_target_header_height": null
            }
        },
        "attempt_counters": {
            "state": "available",
            "value": {
                "attempted_peers": 1,
                "connected_peers": 1,
                "failed_peers": 0,
                "max_sync_rounds": 8
            }
        },
        "progress_signal": { "state": "available", "value": "steady" },
        "lag": {
            "state": "available",
            "value": { "headers_remaining": 0, "blocks_remaining": 0 }
        },
        "last_successful_progress_unix_seconds": {
            "state": "unavailable",
            "value": { "reason": "no successful sync progress recorded in this run" }
        },
        "latest_stop_reason": {
            "state": "unavailable",
            "value": { "reason": "no stop reason recorded" }
        },
        "last_error": {
            "state": "unavailable",
            "value": { "reason": "no sync error recorded" }
        },
        "recovery_category": {
            "state": "unavailable",
            "value": { "reason": "no recovery category recorded" }
        },
        "recovery_action": {
            "state": "unavailable",
            "value": { "reason": "no recovery action required" }
        },
        "resource_pressure": {
            "state": "unavailable",
            "value": { "reason": "resource pressure unavailable" }
        }
    });

    // Act
    let sync_status: SyncStatus = serde_json::from_value(payload).expect("sync status decode");

    // Assert
    assert_eq!(
        sync_status.best_known_tip,
        FieldAvailability::unavailable("best-known tip evidence unavailable")
    );
    assert_eq!(
        sync_status.stay_current,
        FieldAvailability::<StayCurrentStatus>::unavailable("stay-current state unavailable")
    );
    assert_eq!(
        sync_status.stay_current_next_action,
        FieldAvailability::<String>::unavailable("stay-current next action unavailable")
    );
}

#[test]
fn phase69_sync_status_serializes_tip_and_stay_current_fields() {
    // Arrange
    let mut sync_status = SyncRunSummary::empty(2, 2, 1).sync_status(SyncNetwork::Regtest);
    sync_status.best_known_tip = FieldAvailability::available(BestKnownTipStatus {
        source: BestKnownTipSource::HeaderStore,
        height: 2,
        block_hash: "aa".to_string(),
        work: "3".to_string(),
        block_time_unix_seconds: 1_777_225_000,
        observed_at_unix_seconds: 1_777_225_010,
        freshness: TipFreshnessStatus::Fresh,
        peer_agreement: vec![PeerTipAgreement {
            peer: "127.0.0.1:18444".to_string(),
            maybe_resolved_endpoint: Some("127.0.0.1:18444".to_string()),
            status: PeerTipAgreementStatus::Agrees,
            maybe_height: Some(2),
            maybe_hash: Some("aa".to_string()),
            maybe_work: Some("3".to_string()),
            maybe_last_activity_unix_seconds: Some(1_777_225_010),
        }],
    });
    sync_status.stay_current =
        FieldAvailability::available(StayCurrentStatus::CurrentAtBestKnownTip);
    sync_status.stay_current_next_action = FieldAvailability::available(
        "No action required; node is current at the best-known validated tip.".to_string(),
    );

    // Act
    let encoded = serde_json::to_string(&sync_status).expect("sync status encode");

    // Assert
    assert!(encoded.contains("best_known_tip"));
    assert!(encoded.contains("header_store"));
    assert!(encoded.contains("fresh"));
    assert!(encoded.contains("agrees"));
    assert!(encoded.contains("current_at_best_known_tip"));
    assert!(
        encoded.contains("No action required; node is current at the best-known validated tip.")
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
fn sync_once_with_resolver_records_resolution_failures() {
    // Arrange
    let path = temp_store_path("resolver-failure");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(
        store,
        SyncRuntimeConfig {
            manual_peers: vec![SyncPeerAddress::manual("seed.invalid", 18_444)],
            dns_seeds: Vec::new(),
            ..sync_config()
        },
    )
    .expect("runtime");
    let mut transport = ScriptedTransport::new(Vec::new());
    let mut resolver = ScriptedResolver::new(vec![Err(SyncRuntimeError::AddressResolution {
        peer: "seed.invalid:18444".to_string(),
        message: "scripted lookup failure".to_string(),
    })]);

    // Act
    let summary = runtime
        .sync_once_with_resolver(&mut transport, &mut resolver, 1_777_225_166)
        .expect("summary");

    // Assert
    assert_eq!(summary.attempted_peers, 1);
    assert_eq!(summary.failed_peers, 1);
    assert_eq!(
        summary.peer_outcomes[0].maybe_failure_reason,
        Some(PeerFailureReason::AddressResolution)
    );
    assert!(
        summary.peer_outcomes[0]
            .maybe_error
            .as_ref()
            .is_some_and(|message| message.contains("address resolution failed"))
    );
}

#[test]
fn sync_once_rotates_to_alternative_peer_after_stall() {
    // Arrange
    let path = temp_store_path("peer-rotation");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(
        store,
        SyncRuntimeConfig {
            manual_peers: vec![
                SyncPeerAddress::manual("198.51.100.10", 18_444),
                SyncPeerAddress::manual("198.51.100.11", 18_444),
            ],
            dns_seeds: Vec::new(),
            target_outbound_peers: 1,
            max_messages_per_peer: 3,
            ..sync_config()
        },
    )
    .expect("runtime");
    let mut transport = ScriptedTransport::new(vec![
        Vec::new(),
        headers_script(1, vec![header(BlockHash::from_byte_array([0_u8; 32]), 2)]),
    ]);
    let mut resolver = ScriptedResolver::new(vec![
        Ok(vec![ResolvedSyncPeerAddress::new(
            SyncPeerAddress::manual("198.51.100.10", 18_444),
            SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 18_444),
        )]),
        Ok(vec![ResolvedSyncPeerAddress::new(
            SyncPeerAddress::manual("198.51.100.11", 18_444),
            SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 2)), 18_444),
        )]),
    ]);

    // Act
    let summary = runtime
        .sync_once_with_resolver(&mut transport, &mut resolver, 1_777_225_177)
        .expect("summary");

    // Assert
    assert_eq!(summary.attempted_peers, 2);
    assert_eq!(summary.peer_outcomes.len(), 2);
    assert_eq!(summary.peer_outcomes[0].state, PeerSyncState::Stalled);
    assert_eq!(
        summary.peer_outcomes[0].maybe_failure_reason,
        Some(PeerFailureReason::Stall)
    );
    assert_eq!(summary.peer_outcomes[1].state, PeerSyncState::Connected);
    assert_eq!(summary.peer_outcomes[1].contribution.headers_received, 1);
}

#[test]
fn sync_once_retry_backoff_wait_replaces_peer() {
    // Arrange
    let path = temp_store_path("backoff-replacement");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(
        store,
        SyncRuntimeConfig {
            manual_peers: vec![
                SyncPeerAddress::manual("198.51.100.12", 18_444),
                SyncPeerAddress::manual("198.51.100.13", 18_445),
            ],
            dns_seeds: Vec::new(),
            target_outbound_peers: 1,
            max_messages_per_peer: 2,
            retry_backoff_ms: 10_000,
            ..sync_config()
        },
    )
    .expect("runtime");
    let mut transport = ScriptedTransport::new(vec![
        Vec::new(),
        version_verack_script(0),
        version_verack_script(0),
    ]);
    let mut resolver = ScriptedResolver::new(vec![
        Ok(vec![resolved_manual_peer("198.51.100.12", 18_444)]),
        Ok(vec![resolved_manual_peer("198.51.100.13", 18_445)]),
        Ok(vec![resolved_manual_peer("198.51.100.12", 18_444)]),
        Ok(vec![resolved_manual_peer("198.51.100.13", 18_445)]),
    ]);
    runtime
        .sync_once_with_resolver(&mut transport, &mut resolver, 1_777_225_177)
        .expect("first sync");

    // Act
    let summary = runtime
        .sync_once_with_resolver(&mut transport, &mut resolver, 1_777_225_178)
        .expect("second sync");

    // Assert
    assert_eq!(summary.attempted_peers, 1);
    assert_eq!(summary.connected_peers, 1);
    assert_eq!(summary.failed_peers, 0);
    assert_eq!(summary.peer_outcomes.len(), 2);
    assert_eq!(summary.peer_outcomes[0].state, PeerSyncState::Waiting);
    assert_eq!(
        summary.peer_outcomes[0].maybe_failure_reason,
        Some(PeerFailureReason::RetryBackoff)
    );
    assert!(
        summary.peer_outcomes[0]
            .maybe_error
            .as_ref()
            .is_some_and(|message| message.contains("consecutive_failures=1"))
    );
    assert_eq!(summary.peer_outcomes[1].state, PeerSyncState::Connected);
}

#[test]
fn sync_once_waiting_backoff_projects_waiting_for_peers_phase() {
    // Arrange
    let path = temp_store_path("backoff-waiting-phase");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(
        store,
        SyncRuntimeConfig {
            manual_peers: vec![SyncPeerAddress::manual("198.51.100.14", 18_444)],
            dns_seeds: Vec::new(),
            retry_backoff_ms: 10_000,
            ..sync_config()
        },
    )
    .expect("runtime");
    let mut transport = ScriptedTransport::new(vec![Vec::new()]);
    let mut resolver = ScriptedResolver::new(vec![
        Ok(vec![resolved_manual_peer("198.51.100.14", 18_444)]),
        Ok(vec![resolved_manual_peer("198.51.100.14", 18_444)]),
    ]);
    runtime
        .sync_once_with_resolver(&mut transport, &mut resolver, 1_777_225_180)
        .expect("first sync");

    // Act
    let summary = runtime
        .sync_once_with_resolver(&mut transport, &mut resolver, 1_777_225_181)
        .expect("second sync");
    let sync_status = summary.sync_status(SyncNetwork::Regtest);
    let peer_status = summary.peer_status();
    let log_records = summary.structured_log_records(1_777_225_181);

    // Assert
    assert_eq!(summary.attempted_peers, 0);
    assert_eq!(summary.connected_peers, 0);
    assert_eq!(summary.failed_peers, 0);
    assert_eq!(
        sync_status.phase,
        FieldAvailability::available("waiting_for_peers".to_string())
    );
    assert!(matches!(
        peer_status.recent_peers,
        FieldAvailability::Available(ref peers)
            if peers.first().is_some_and(|peer| peer.state == "waiting")
    ));
    assert!(log_records.iter().any(|record| {
        record.level == StructuredLogLevel::Warn
            && record.source == "sync"
            && record.message.contains("retry backoff")
    }));
}

#[test]
fn sync_status_preserves_configured_target_outbound_peer_count() {
    // Arrange
    let path = temp_store_path("configured-target-outbound");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(
        store,
        SyncRuntimeConfig {
            manual_peers: vec![SyncPeerAddress::manual("198.51.100.15", 18_444)],
            dns_seeds: Vec::new(),
            target_outbound_peers: 3,
            max_messages_per_peer: 2,
            ..sync_config()
        },
    )
    .expect("runtime");
    let mut transport = ScriptedTransport::new(vec![version_verack_script(0)]);
    let mut resolver = ScriptedResolver::new(vec![Ok(vec![resolved_manual_peer(
        "198.51.100.15",
        18_444,
    )])]);

    // Act
    let summary = runtime
        .sync_once_with_resolver(&mut transport, &mut resolver, 1_777_225_182)
        .expect("sync");
    let sync_status = summary.sync_status(SyncNetwork::Regtest);

    // Assert
    assert_eq!(summary.target_outbound_peers, 3);
    assert_eq!(
        sync_status.resource_pressure,
        FieldAvailability::available(SyncResourcePressure {
            blocks_in_flight: 0,
            max_header_requests_in_flight_per_peer: 1,
            max_headers_per_message: 2_000,
            max_blocks_in_flight_per_peer: 0,
            max_blocks_in_flight_total: 0,
            max_messages_per_peer: 0,
            max_sync_rounds: 0,
            outbound_peers: 1,
            target_outbound_peers: 3,
        })
    );
}

#[test]
fn sync_once_stops_after_target_outbound_peer_budget_is_met() {
    // Arrange
    let path = temp_store_path("target-outbound-budget");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(
        store,
        SyncRuntimeConfig {
            manual_peers: vec![
                SyncPeerAddress::manual("198.51.100.20", 18_444),
                SyncPeerAddress::manual("198.51.100.21", 18_444),
            ],
            dns_seeds: Vec::new(),
            target_outbound_peers: 1,
            ..sync_config()
        },
    )
    .expect("runtime");
    let mut transport =
        ScriptedTransport::new(vec![version_verack_script(0), version_verack_script(0)]);
    let mut resolver = ScriptedResolver::new(vec![
        Ok(vec![resolved_manual_peer("198.51.100.20", 18_444)]),
        Ok(vec![resolved_manual_peer("198.51.100.21", 18_444)]),
    ]);

    // Act
    let summary = runtime
        .sync_once_with_resolver(&mut transport, &mut resolver, 1_777_225_188)
        .expect("summary");

    // Assert
    assert_eq!(summary.attempted_peers, 1);
    assert_eq!(summary.peer_outcomes.len(), 1);
}

#[test]
fn manual_peer_completes_handshake_before_idle() {
    // Arrange
    let path = temp_store_path("manual-handshake-idle");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(
        store,
        SyncRuntimeConfig {
            manual_peers: vec![SyncPeerAddress::manual("198.51.100.22", 18_444)],
            dns_seeds: Vec::new(),
            target_outbound_peers: 1,
            max_messages_per_peer: 8,
            ..sync_config()
        },
    )
    .expect("runtime");
    let mut transport = ScriptedTransport::new(vec![version_verack_script(0)]);
    let mut resolver = ScriptedResolver::new(vec![Ok(vec![resolved_manual_peer(
        "198.51.100.22",
        18_444,
    )])]);

    // Act
    let summary = runtime
        .sync_once_with_resolver(&mut transport, &mut resolver, 1_777_225_189)
        .expect("summary");

    // Assert
    assert_eq!(summary.attempted_peers, 1);
    assert_eq!(summary.connected_peers, 1);
    assert_eq!(summary.failed_peers, 0);
    assert!(summary.health_signals.is_empty());
    let outcome = &summary.peer_outcomes[0];
    assert_eq!(outcome.state, PeerSyncState::Connected);
    assert_eq!(outcome.contribution.messages_processed, 2);
    assert_eq!(outcome.contribution.headers_received, 0);
    assert_eq!(outcome.contribution.blocks_received, 0);
    assert_eq!(outcome.maybe_failure_reason, None);
    assert!(outcome.maybe_capabilities.is_some());
    assert_eq!(summary.best_header_height, 0);
    assert_eq!(summary.best_block_height, 0);

    remove_dir_if_exists(&path);
}

#[test]
fn dns_seed_peer_completes_handshake_before_idle() {
    // Arrange
    let path = temp_store_path("dns-handshake-idle");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(
        store,
        SyncRuntimeConfig {
            manual_peers: Vec::new(),
            dns_seeds: vec!["seed.example.invalid".to_string()],
            target_outbound_peers: 1,
            max_messages_per_peer: 8,
            ..sync_config()
        },
    )
    .expect("runtime");
    let mut transport = ScriptedTransport::new(vec![version_verack_script(0)]);
    let mut resolver = ScriptedResolver::new(vec![Ok(vec![ResolvedSyncPeerAddress::new(
        SyncPeerAddress::dns_seed("seed.example.invalid", 18_444),
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 18_444),
    )])]);

    // Act
    let summary = runtime
        .sync_once_with_resolver(&mut transport, &mut resolver, 1_777_225_190)
        .expect("summary");

    // Assert
    assert_eq!(summary.attempted_peers, 1);
    assert_eq!(summary.connected_peers, 1);
    let outcome = &summary.peer_outcomes[0];
    assert_eq!(outcome.state, PeerSyncState::Connected);
    assert_eq!(outcome.peer.source, SyncPeerSource::DnsSeed);
    assert_eq!(outcome.contribution.headers_received, 0);
    assert_eq!(outcome.contribution.blocks_received, 0);
    assert_eq!(outcome.maybe_failure_reason, None);

    remove_dir_if_exists(&path);
}

#[test]
fn duplicate_version_peer_is_failed_and_replaced_without_progress_credit() {
    // Arrange
    let path = temp_store_path("duplicate-version-replacement");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let duplicate_version_script = vec![
        WireNetworkMessage::Version(VersionMessage {
            start_height: 0,
            ..VersionMessage::default()
        }),
        WireNetworkMessage::Version(VersionMessage {
            start_height: 0,
            ..VersionMessage::default()
        }),
    ];
    let mut runtime = DurableSyncRuntime::open(
        store,
        SyncRuntimeConfig {
            manual_peers: vec![
                SyncPeerAddress::manual("198.51.100.23", 18_444),
                SyncPeerAddress::manual("198.51.100.24", 18_445),
            ],
            dns_seeds: Vec::new(),
            max_peer_retries: 0,
            target_outbound_peers: 1,
            max_messages_per_peer: 8,
            ..sync_config()
        },
    )
    .expect("runtime");
    let mut transport =
        ScriptedTransport::new(vec![duplicate_version_script, version_verack_script(0)]);
    let mut resolver = ScriptedResolver::new(vec![
        Ok(vec![resolved_manual_peer("198.51.100.23", 18_444)]),
        Ok(vec![resolved_manual_peer("198.51.100.24", 18_445)]),
    ]);

    // Act
    let summary = runtime
        .sync_once_with_resolver(&mut transport, &mut resolver, 1_777_225_191)
        .expect("summary");
    let metadata = runtime
        .store()
        .load_runtime_metadata()
        .expect("load runtime metadata")
        .expect("runtime metadata");
    let durable_sync_state = metadata.maybe_sync_state.expect("durable sync state");

    // Assert
    assert_eq!(summary.attempted_peers, 2);
    assert_eq!(summary.failed_peers, 1);
    assert_eq!(summary.connected_peers, 1);
    assert_eq!(summary.headers_received, 0);
    assert_eq!(summary.blocks_received, 0);
    let rejected = &summary.peer_outcomes[0];
    assert_eq!(rejected.state, PeerSyncState::Failed);
    assert_eq!(
        rejected.maybe_failure_reason,
        Some(PeerFailureReason::Compatibility)
    );
    assert_eq!(rejected.contribution.headers_received, 0);
    assert_eq!(rejected.contribution.blocks_received, 0);
    assert!(
        rejected
            .maybe_error
            .as_ref()
            .is_some_and(|message| { message.contains("duplicate version") })
    );
    assert_eq!(summary.peer_outcomes[1].state, PeerSyncState::Connected);
    assert_eq!(
        durable_sync_state.sync.lifecycle,
        FieldAvailability::available(SyncLifecycleState::Active)
    );
    assert_eq!(
        durable_sync_state.sync.resource_pressure,
        FieldAvailability::available(SyncResourcePressure {
            blocks_in_flight: 0,
            max_header_requests_in_flight_per_peer: 1,
            max_headers_per_message: 2_000,
            max_blocks_in_flight_per_peer: 16,
            max_blocks_in_flight_total: 64,
            max_messages_per_peer: 8,
            max_sync_rounds: 8,
            outbound_peers: 1,
            target_outbound_peers: 1,
        })
    );

    remove_dir_if_exists(&path);
}

#[test]
fn wrong_network_peer_is_failed_without_progress_credit() {
    // Arrange
    let path = temp_store_path("wrong-network-peer");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(
        store,
        SyncRuntimeConfig {
            manual_peers: vec![SyncPeerAddress::manual("198.51.100.25", 18_444)],
            dns_seeds: Vec::new(),
            max_peer_retries: 0,
            ..sync_config()
        },
    )
    .expect("runtime");
    let mut transport =
        ScriptedTransport::with_connect_results(vec![Err(SyncRuntimeError::InvalidMagic {
            expected: SyncNetwork::Regtest.magic().to_bytes(),
            actual: SyncNetwork::Mainnet.magic().to_bytes(),
        })]);
    let mut resolver = ScriptedResolver::new(vec![Ok(vec![resolved_manual_peer(
        "198.51.100.25",
        18_444,
    )])]);

    // Act
    let summary = runtime
        .sync_once_with_resolver(&mut transport, &mut resolver, 1_777_225_192)
        .expect("summary");

    // Assert
    assert_eq!(summary.connected_peers, 0);
    assert_eq!(summary.failed_peers, 1);
    assert_eq!(summary.headers_received, 0);
    assert_eq!(summary.blocks_received, 0);
    let outcome = &summary.peer_outcomes[0];
    assert_eq!(outcome.state, PeerSyncState::Failed);
    assert_eq!(
        outcome.maybe_failure_reason,
        Some(PeerFailureReason::InvalidMagic)
    );
    assert_eq!(outcome.contribution.messages_processed, 0);
    assert_eq!(outcome.contribution.headers_received, 0);
    assert_eq!(outcome.contribution.blocks_received, 0);

    remove_dir_if_exists(&path);
}

#[test]
fn sync_outcome_captures_peer_capabilities_and_endpoint() {
    // Arrange
    let path = temp_store_path("peer-capabilities");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(
        store,
        SyncRuntimeConfig {
            manual_peers: vec![SyncPeerAddress::manual("198.51.100.30", 18_444)],
            dns_seeds: Vec::new(),
            max_messages_per_peer: 4,
            ..sync_config()
        },
    )
    .expect("runtime");
    let mut transport = ScriptedTransport::new(vec![vec![
        WireNetworkMessage::Version(VersionMessage {
            start_height: 3,
            ..VersionMessage::default()
        }),
        WireNetworkMessage::WtxidRelay,
        WireNetworkMessage::SendHeaders,
        WireNetworkMessage::Verack,
    ]]);
    let mut resolver = ScriptedResolver::new(vec![Ok(vec![resolved_manual_peer(
        "198.51.100.30",
        18_444,
    )])]);

    // Act
    let summary = runtime
        .sync_once_with_resolver(&mut transport, &mut resolver, 1_777_225_199)
        .expect("summary");

    // Assert
    let outcome = &summary.peer_outcomes[0];
    assert_eq!(outcome.state, PeerSyncState::Connected);
    assert_eq!(
        outcome.maybe_resolved_endpoint.as_deref(),
        Some("127.0.0.1:18444")
    );
    let capabilities = outcome
        .maybe_capabilities
        .as_ref()
        .expect("peer capabilities");
    assert!(capabilities.services_bits > 0);
    assert_eq!(capabilities.start_height, 3);
    assert!(capabilities.wtxidrelay);
    assert!(capabilities.prefers_headers);
}

#[test]
fn sync_metrics_history_appends_across_runs() {
    // Arrange
    let path = temp_store_path("metrics-history");
    remove_dir_if_exists(&path);
    {
        let store = FjallNodeStore::open(&path).expect("store");
        let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
        let mut transport =
            ScriptedTransport::new(vec![version_verack_script(0), version_verack_script(1)]);

        // Act
        runtime
            .sync_once(&mut transport, 1_777_225_022)
            .expect("first sync");
        runtime
            .sync_once(&mut transport, 1_777_225_052)
            .expect("second sync");
    }

    // Assert
    let reopened = FjallNodeStore::open(&path).expect("reopen store");
    let metrics = reopened
        .load_metrics_snapshot()
        .expect("load metrics")
        .expect("metrics snapshot");
    let mut sync_height_timestamps = metrics
        .samples
        .iter()
        .filter(|sample| sample.kind == MetricKind::SyncHeight)
        .map(|sample| sample.timestamp_unix_seconds)
        .collect::<Vec<_>>();
    sync_height_timestamps.sort_unstable();
    sync_height_timestamps.dedup();
    assert!(sync_height_timestamps.contains(&1_777_225_022));
    assert!(sync_height_timestamps.contains(&1_777_225_052));
    assert!(sync_height_timestamps.len() >= 2);

    remove_dir_if_exists(&path);
}

#[test]
fn persist_metrics_appends_inbound_status_samples_with_sync_samples() {
    // Arrange
    let path = temp_store_path("metrics-inbound");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    let inbound = inbound_status_for_metrics();
    runtime
        .set_inbound_metric_status_provider(move || FieldAvailability::available(inbound.clone()));
    let summary = runtime.snapshot_summary();

    // Act
    runtime
        .persist_metrics(&summary, None, 1_777_225_022)
        .expect("persist metrics");

    // Assert
    let metrics = runtime
        .store()
        .load_metrics_snapshot()
        .expect("load metrics")
        .expect("metrics snapshot");
    assert!(metrics.samples.iter().any(|sample| {
        sample.kind == MetricKind::SyncHeight && sample.timestamp_unix_seconds == 1_777_225_022
    }));
    assert!(metrics.samples.iter().any(|sample| {
        sample.kind == MetricKind::InboundResourcePressureActiveCount
            && sample.value == 16.0
            && sample.timestamp_unix_seconds == 1_777_225_022
    }));

    remove_dir_if_exists(&path);
}

#[test]
fn persist_metrics_omits_inbound_samples_when_status_unavailable() {
    // Arrange
    let path = temp_store_path("metrics-inbound-unavailable");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    runtime.set_inbound_metric_status_provider(inbound_status_unavailable);
    let summary = runtime.snapshot_summary();

    // Act
    runtime
        .persist_metrics(&summary, None, 1_777_225_022)
        .expect("persist metrics");

    // Assert
    let metrics = runtime
        .store()
        .load_metrics_snapshot()
        .expect("load metrics")
        .expect("metrics snapshot");
    assert!(
        metrics
            .samples
            .iter()
            .any(|sample| sample.kind == MetricKind::SyncHeight)
    );
    assert!(!metrics.samples.iter().any(|sample| matches!(
        sample.kind,
        MetricKind::InboundAdmittedPeerCount
            | MetricKind::InboundRejectedPeerCount
            | MetricKind::InboundCapRejectCount
            | MetricKind::InboundReservedSlotRejectCount
            | MetricKind::InboundDuplicateRejectCount
            | MetricKind::InboundSelfConnectionRejectCount
            | MetricKind::InboundPermissionedAdmitCount
            | MetricKind::InboundProtectedAdmitCount
            | MetricKind::InboundInactivePermissionEffectCount
            | MetricKind::InboundPermissionValidationFailureCount
            | MetricKind::InboundEvictionCandidateCount
            | MetricKind::InboundDisconnectCount
            | MetricKind::InboundActiveBanCount
            | MetricKind::InboundMisbehaviorObservationCount
            | MetricKind::InboundProtectedNoActionCount
            | MetricKind::InboundResourcePressureActiveCount
            | MetricKind::InboundReadQueuePressureCount
            | MetricKind::InboundWriteQueuePressureCount
            | MetricKind::InboundRequestCapReachedCount
            | MetricKind::InboundPayloadRejectedCount
            | MetricKind::InboundTimeoutDisconnectCount
            | MetricKind::InboundChurnRejectedCount
            | MetricKind::InboundReconnectSuppressedCount
    )));

    remove_dir_if_exists(&path);
}

#[test]
fn persist_metrics_appends_block_relay_status_samples_with_sync_samples() {
    // Arrange
    let path = temp_store_path("metrics-block-relay");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    let block_relay = block_relay_status_for_metrics();
    let snapshot = crate::network::BlockRelayRuntimeEvidenceSnapshot {
        status: block_relay,
        served_count: 9,
    };
    let summary = runtime.snapshot_summary();

    // Act
    runtime
        .persist_metrics(&summary, Some(&snapshot), 1_777_225_022)
        .expect("persist metrics");

    // Assert
    let metrics = runtime
        .store()
        .load_metrics_snapshot()
        .expect("load metrics")
        .expect("metrics snapshot");
    assert!(metrics.samples.iter().any(|sample| {
        sample.kind == MetricKind::SyncHeight && sample.timestamp_unix_seconds == 1_777_225_022
    }));
    assert!(metrics.samples.iter().any(|sample| {
        sample.kind == MetricKind::CompactAnnouncedCount
            && sample.value == 6.0
            && sample.timestamp_unix_seconds == 1_777_225_022
    }));
    assert!(metrics.samples.iter().any(|sample| {
        sample.kind == MetricKind::BlockServedCount
            && sample.value == 9.0
            && sample.timestamp_unix_seconds == 1_777_225_022
    }));

    remove_dir_if_exists(&path);
}

#[test]
fn persist_metrics_omits_block_relay_samples_without_snapshot() {
    // Arrange
    let path = temp_store_path("metrics-block-relay-unavailable");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    let summary = runtime.snapshot_summary();

    // Act
    runtime
        .persist_metrics(&summary, None, 1_777_225_022)
        .expect("persist metrics");

    // Assert
    let metrics = runtime
        .store()
        .load_metrics_snapshot()
        .expect("load metrics")
        .expect("metrics snapshot");
    assert!(
        metrics
            .samples
            .iter()
            .any(|sample| sample.kind == MetricKind::SyncHeight)
    );
    assert!(!metrics.samples.iter().any(|sample| matches!(
        sample.kind,
        MetricKind::BlockServedCount
            | MetricKind::BlockServingSuppressedCount
            | MetricKind::CompactAnnouncedCount
            | MetricKind::CompactReconstructedCount
            | MetricKind::CompactMissingTxRequestedCount
            | MetricKind::CompactFallbackCount
            | MetricKind::CompactMalformedCount
            | MetricKind::CompactTimeoutCount
            | MetricKind::CompactCleanupCount
    )));

    remove_dir_if_exists(&path);
}

fn block_relay_status_for_metrics() -> BlockRelayEvidenceStatus {
    BlockRelayEvidenceStatus::with_components(
        BlockServingEvidenceStatus::with_activation_eligibility_and_status(
            BlockServingActivationEvidence {
                block_serving_enabled: true,
                compact_relay_enabled: true,
            },
            BlockServingEligibilityCounters {
                eligible_peer_count: 2,
                ineligible_peer_count: 3,
                disabled_count: 1,
                activation_required_count: 0,
                inbound_serving_required_count: 1,
                permission_required_count: 1,
                protected_not_serving_count: 0,
                status_unavailable_count: 0,
                permission_effect_inactive_count: 1,
            },
            BlockServingStatusCounters {
                validated_count: 5,
                available_count: 4,
                stale_count: 1,
                side_chain_count: 2,
                pruned_count: 1,
                unavailable_count: 3,
                unvalidated_count: 0,
                unknown_count: 1,
                suppressed_count: 2,
            },
        ),
        CompactRelayNegotiationCounters {
            version2_high_bandwidth_count: 3,
            version2_low_bandwidth_count: 1,
            unsupported_version_count: 1,
        },
        CompactRelayAnnouncementCounters {
            compact_announced_count: 6,
            compact_headers_fallback_count: 2,
            compact_inventory_fallback_count: 1,
            compact_suppressed_count: 2,
        },
        CompactRelayReconstructionCounters {
            compact_reconstructed_count: 4,
            compact_reconstruction_failed_count: 1,
            compact_malformed_count: 1,
        },
        CompactRelayMissingTransactionCounters {
            compact_missing_tx_requested_count: 2,
            compact_missing_tx_suppressed_count: 1,
        },
        CompactRelayFallbackCounters {
            compact_fallback_count: 2,
            compact_timeout_count: 1,
        },
        CompactRelayInFlightCounters {
            in_flight_count: 3,
            getblocktxn_in_flight_count: 2,
            peers_with_in_flight_count: 2,
        },
        CompactRelayCleanupCounters {
            compact_cleanup_count: 3,
            compact_download_peer_disconnect_count: 1,
            compact_download_timeout_count: 1,
            compact_download_reorg_count: 0,
            compact_download_restart_count: 0,
            compact_download_block_connected_count: 1,
        },
    )
}

#[test]
fn write_block_relay_log_emits_when_status_available() {
    // Arrange
    let path = temp_store_path("block-relay-log-available");
    let log_dir = path.join("logs");
    remove_dir_if_exists(&path);
    fs::create_dir_all(&log_dir).expect("create log dir");
    let store = FjallNodeStore::open(&path).expect("store");
    let runtime =
        DurableSyncRuntime::open(store, sync_config_with_log_dir(&log_dir)).expect("runtime");
    let block_relay = block_relay_status_for_metrics();
    let snapshot = crate::network::BlockRelayRuntimeEvidenceSnapshot {
        status: block_relay,
        served_count: 9,
    };
    let mut summary = runtime.snapshot_summary();

    // Act
    runtime.write_block_relay_log(&mut summary, Some(&snapshot), 1_777_225_305);

    // Assert
    let records = load_structured_log_records(&log_dir);
    let maybe_block_relay = records
        .iter()
        .find(|record| record.source == BLOCK_RELAY_LOG_SOURCE);
    let record = maybe_block_relay.expect("block_relay log record");
    assert!(record.message.contains("outcome=projected"));
    assert!(record.message.contains("cause=status_projection"));
    assert!(record.message.contains("label=block_relay"));

    remove_dir_if_exists(&path);
}

#[test]
fn write_block_relay_log_omits_when_status_unavailable() {
    // Arrange
    let path = temp_store_path("block-relay-log-unavailable");
    let log_dir = path.join("logs");
    remove_dir_if_exists(&path);
    fs::create_dir_all(&log_dir).expect("create log dir");
    let store = FjallNodeStore::open(&path).expect("store");
    let runtime =
        DurableSyncRuntime::open(store, sync_config_with_log_dir(&log_dir)).expect("runtime");
    let mut summary = runtime.snapshot_summary();

    // Act
    runtime.write_block_relay_log(&mut summary, None, 1_777_225_306);

    // Assert
    let records = load_structured_log_records(&log_dir);
    assert!(
        !records
            .iter()
            .any(|record| record.source == BLOCK_RELAY_LOG_SOURCE)
    );

    remove_dir_if_exists(&path);
}

#[test]
fn write_block_relay_log_omits_sensitive_markers() {
    // Arrange
    let path = temp_store_path("block-relay-log-leakage");
    let log_dir = path.join("logs");
    remove_dir_if_exists(&path);
    fs::create_dir_all(&log_dir).expect("create log dir");
    let store = FjallNodeStore::open(&path).expect("store");
    let runtime =
        DurableSyncRuntime::open(store, sync_config_with_log_dir(&log_dir)).expect("runtime");
    let block_relay = block_relay_status_for_metrics();
    let snapshot = crate::network::BlockRelayRuntimeEvidenceSnapshot {
        status: block_relay,
        served_count: 0,
    };
    let mut summary = runtime.snapshot_summary();

    // Act
    runtime.write_block_relay_log(&mut summary, Some(&snapshot), 1_777_225_307);

    // Assert
    let records = load_structured_log_records(&log_dir);
    let maybe_block_relay = records
        .iter()
        .find(|record| record.source == BLOCK_RELAY_LOG_SOURCE);
    let record = maybe_block_relay.expect("block_relay log record");
    for raw in [
        "127.0.0.1",
        "peer_id",
        "permission_string",
        "credential",
        "cookie",
        "secret",
        "0123456789abcdef",
    ] {
        assert!(!record.message.contains(raw), "leaked {raw}");
    }

    remove_dir_if_exists(&path);
}

fn inbound_status_for_metrics() -> InboundPeerServingStatus {
    InboundPeerServingStatus {
        listener_state: "ready".to_string(),
        bound_endpoints: Vec::new(),
        preflight_reason: "ready".to_string(),
        admitted_inbound_peers: 1,
        rejected_inbound_peers: 2,
        handshake: InboundHandshakeStatusCounts::default(),
        duplicate_rejects: 5,
        self_connection_rejects: 6,
        cap_rejects: 3,
        reserved_slot_rejects: 4,
        latest_admission_event: FieldAvailability::unavailable("no admission event"),
        permissioned_inbound_peers: 7,
        protected_inbound_peers: 8,
        permission_class: "ordinary_inbound".to_string(),
        active_permission_effects: Vec::new(),
        inactive_permission_effects: Vec::new(),
        inactive_permission_effect_observations: 9,
        permission_validation_failures: 10,
        latest_permission_decision: FieldAvailability::unavailable("no permission decision"),
        local_advertisement_candidates: Vec::new(),
        suppressed_advertisements: Vec::new(),
        getaddr_responses_served: 0,
        getaddr_requests_suppressed: 0,
        learned_address_entries: 0,
        learned_address_rejections: 0,
        latest_address_decision: FieldAvailability::unavailable("no address decision"),
        eviction_candidates_evaluated: 11,
        disconnects_requested: 12,
        discouraged_peers: 0,
        active_bans: 13,
        expired_bans: 0,
        manual_unbans: 0,
        misbehavior_observations: 14,
        protected_no_actions: 15,
        latest_peer_policy_decision: FieldAvailability::unavailable("no peer policy decision"),
        resource_pressure_events: 16,
        read_queue_pressure_events: 17,
        write_queue_pressure_events: 18,
        request_cap_events: 19,
        payload_rejections: 20,
        timeout_disconnects: 21,
        churn_rejections: 22,
        reconnect_suppressions: 23,
        latest_resource_governance_decision: FieldAvailability::unavailable(
            "no resource governance decision",
        ),
    }
}

#[test]
fn sync_status_and_log_records_include_message_header_block_counters() {
    // Arrange
    let path = temp_store_path("counter-logs");
    let log_dir = path.join("logs");
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
        WireNetworkMessage::Block(genesis),
    ];
    let mut transport = ScriptedTransport::new(vec![script]);
    let mut runtime =
        DurableSyncRuntime::open(store, sync_config_with_log_dir(&log_dir)).expect("runtime");

    // Act
    let summary = runtime
        .sync_once(&mut transport, 1_777_225_099)
        .expect("sync");

    // Assert
    assert_eq!(summary.messages_processed, 4);
    assert_eq!(summary.headers_received, 1);
    assert_eq!(summary.blocks_received, 1);
    assert_eq!(
        summary.sync_status(SyncNetwork::Regtest).sync_progress,
        FieldAvailability::available(SyncProgress {
            header_height: 0,
            block_height: 0,
            downloaded_block_height: 0,
            connected_block_height: 0,
            validated_active_chain_height: 0,
            maybe_downloaded_block_hash: Some(block_hash_hex(genesis_hash)),
            maybe_connected_block_hash: Some(block_hash_hex(genesis_hash)),
            maybe_validated_active_chain_hash: Some(block_hash_hex(genesis_hash)),
            maybe_validated_active_chain_work: Some("1".to_string()),
            progress_ratio: 1.0,
            messages_processed: 4,
            headers_received: 1,
            blocks_received: 1,
        })
    );
    let records = load_structured_log_records(&log_dir);
    assert!(records.iter().any(|record| {
        record.level == StructuredLogLevel::Info
            && record.source == "sync"
            && record.message.contains("messages_processed=4")
            && record.message.contains("headers_received=1")
            && record.message.contains("blocks_received=1")
            && record.message.contains("header=0")
            && record.message.contains("downloaded=0")
            && record.message.contains("connected=0")
            && record.message.contains("progress_signal=block_progress")
    }));

    remove_dir_if_exists(&path);
}

#[test]
fn peer_contribution_counts_only_accepted_headers_and_blocks() {
    // Arrange
    let path = temp_store_path("peer-contribution-accepted");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let script = vec![
        WireNetworkMessage::Version(VersionMessage {
            start_height: 0,
            ..VersionMessage::default()
        }),
        WireNetworkMessage::Verack,
        WireNetworkMessage::Headers(HeadersMessage {
            headers: vec![genesis.header.clone()],
        }),
        WireNetworkMessage::Block(genesis),
    ];
    let mut transport = ScriptedTransport::new(vec![script]);
    let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");

    // Act
    let summary = runtime
        .sync_once(&mut transport, 1_777_225_101)
        .expect("sync");

    // Assert
    assert_eq!(summary.headers_received, 1);
    assert_eq!(summary.blocks_received, 1);
    assert_eq!(summary.peer_outcomes.len(), 1);
    assert_eq!(summary.peer_outcomes[0].contribution.messages_processed, 4);
    assert_eq!(summary.peer_outcomes[0].contribution.headers_received, 1);
    assert_eq!(summary.peer_outcomes[0].contribution.blocks_received, 1);
    assert_eq!(
        summary.peer_outcomes[0].maybe_last_activity_unix_seconds,
        Some(1_777_225_101)
    );

    remove_dir_if_exists(&path);
}

#[test]
fn stalled_peer_emits_warning_health_signal_and_log_record() {
    // Arrange
    let path = temp_store_path("stalled-peer");
    let log_dir = path.join("logs");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime =
        DurableSyncRuntime::open(store, sync_config_with_log_dir(&log_dir)).expect("runtime");
    let mut transport = ScriptedTransport::new(vec![Vec::new()]);

    // Act
    let summary = runtime
        .sync_once(&mut transport, 1_777_225_111)
        .expect("sync");

    // Assert
    assert_eq!(summary.peer_outcomes[0].state, PeerSyncState::Stalled);
    assert!(summary.health_signals.iter().any(|signal| {
        signal.level == HealthSignalLevel::Warn
            && signal.source == "sync"
            && signal.message.contains("peer stalled")
    }));
    let log_status = load_log_status(&log_dir, LogRetentionPolicy::default(), 10);
    assert!(log_status.recent_signals.iter().any(|signal| {
        signal.level == StructuredLogLevel::Warn
            && signal.source == "sync"
            && signal.message.contains("peer stalled")
    }));

    remove_dir_if_exists(&path);
}

#[test]
fn connect_retries_preserve_attempt_count() {
    // Arrange
    let path = temp_store_path("connect-retries");
    let log_dir = path.join("logs");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(
        store,
        SyncRuntimeConfig {
            max_peer_retries: 2,
            maybe_log_dir: Some(log_dir.clone()),
            ..sync_config()
        },
    )
    .expect("runtime");
    let mut transport = ScriptedTransport::failing();

    // Act
    let summary = runtime
        .sync_once(&mut transport, 1_777_225_122)
        .expect("sync");

    // Assert
    assert_eq!(summary.failed_peers, 1);
    assert_eq!(summary.peer_outcomes[0].state, PeerSyncState::Failed);
    assert_eq!(summary.peer_outcomes[0].attempts, 3);
    assert!(summary.health_signals.iter().any(|signal| {
        signal.source == "network" && signal.message.contains("sync I/O failure")
    }));
    let log_status = load_log_status(&log_dir, LogRetentionPolicy::default(), 10);
    assert!(log_status.recent_signals.iter().any(|signal| {
        signal.source == "network" && signal.message.contains("sync I/O failure")
    }));

    remove_dir_if_exists(&path);
}

#[test]
fn storage_failure_projects_storage_health_signal() {
    // Arrange
    let error = SyncRuntimeError::Storage(StorageError::BackendFailure {
        namespace: StorageNamespace::Metrics,
        message: "/tmp/open-bitcoin/private-store".to_string(),
        action: crate::StorageRecoveryAction::Restart,
    });

    // Act
    let signal = error.health_signal();
    let records = SyncRunSummary {
        target_outbound_peers: 0,
        maybe_target_header_height: None,
        attempted_peers: 0,
        connected_peers: 0,
        failed_peers: 0,
        messages_processed: 0,
        headers_received: 0,
        blocks_received: 0,
        best_header_height: 0,
        downloaded_block_height: 0,
        best_block_height: 0,
        maybe_downloaded_block_hash: None,
        maybe_connected_block_hash: None,
        maybe_validated_active_chain_work: None,
        peer_outcomes: Vec::new(),
        health_signals: vec![signal.clone()],
        maybe_stop_reason: None,
        maybe_reconcile_progress: None,
    }
    .structured_log_records(1_777_225_133);

    // Assert
    assert_eq!(signal.level, HealthSignalLevel::Error);
    assert_eq!(signal.source, "storage");
    assert!(
        signal
            .message
            .contains("storage backend failure in metrics")
    );
    assert!(!signal.message.contains("/tmp/"));
    assert!(records.iter().any(|record| {
        record.level == StructuredLogLevel::Error
            && record.source == "storage"
            && record.message == signal.message
    }));
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
    assert_eq!(summary.peer_outcomes[0].state, PeerSyncState::Connected);
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
            downloaded_block_height: 0,
            connected_block_height: 0,
            validated_active_chain_height: 0,
            maybe_downloaded_block_hash: None,
            maybe_connected_block_hash: None,
            maybe_validated_active_chain_hash: None,
            maybe_validated_active_chain_work: None,
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
    assert!(
        transport
            .sent_messages()
            .iter()
            .any(|message| matches!(message, WireNetworkMessage::GetData(_)))
    );

    remove_dir_if_exists(&path);
}

#[test]
fn sync_until_idle_continues_equal_message_rounds_when_heights_advance() {
    // Arrange
    let path = temp_store_path("until-idle-progress");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let genesis = header(BlockHash::from_byte_array([0_u8; 32]), 21);
    let child = header(block_hash(&genesis), 22);
    let grandchild = header(block_hash(&child), 23);
    let mut runtime = DurableSyncRuntime::open(
        store,
        SyncRuntimeConfig {
            max_rounds: 4,
            ..sync_config()
        },
    )
    .expect("runtime");
    let mut transport = ScriptedTransport::new(vec![
        headers_script(0, vec![genesis]),
        headers_script(1, vec![child]),
        headers_script(2, vec![grandchild]),
        Vec::new(),
    ]);

    // Act
    let summary = runtime
        .sync_until_idle(&mut transport, 1_777_225_155)
        .expect("sync until idle");

    // Assert
    assert_eq!(summary.best_header_height, 2);
    assert_eq!(runtime.snapshot_summary().best_header_height, 2);

    remove_dir_if_exists(&path);
}

#[test]
fn sync_until_idle_stops_at_configured_header_target_after_multiple_batches() {
    // Arrange
    let path = temp_store_path("until-idle-header-target");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let genesis = header(BlockHash::from_byte_array([0_u8; 32]), 31);
    let child = header(block_hash(&genesis), 32);
    let grandchild = header(block_hash(&child), 33);
    let mut runtime = DurableSyncRuntime::open(
        store,
        SyncRuntimeConfig {
            maybe_target_header_height: Some(2),
            max_rounds: 5,
            ..sync_config()
        },
    )
    .expect("runtime");
    let mut transport = ScriptedTransport::new(vec![
        headers_script(0, vec![genesis]),
        headers_script(1, vec![child]),
        headers_script(2, vec![grandchild]),
        headers_script(3, Vec::new()),
    ]);

    // Act
    let summary = runtime
        .sync_until_idle(&mut transport, 1_777_225_156)
        .expect("sync until target");
    let state = runtime
        .durable_sync_state_for_summary(&summary, SyncLifecycleState::Active, None, 1_777_225_156)
        .expect("durable status");

    // Assert
    assert_eq!(summary.best_header_height, 2);
    assert_eq!(
        summary.maybe_stop_reason,
        Some(SyncStopReason::TargetHeaderReached {
            target_header_height: 2,
            best_header_height: 2,
        })
    );
    assert!(summary.health_signals.iter().any(|signal| {
        signal.level == HealthSignalLevel::Info
            && signal.message.contains("sync header target reached")
    }));
    assert_eq!(
        state.sync.phase,
        FieldAvailability::available("header_target_reached".to_string())
    );

    remove_dir_if_exists(&path);
}

#[test]
fn sync_until_idle_records_no_progress_diagnosis_without_public_network() {
    // Arrange
    let path = temp_store_path("until-idle-no-progress");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(
        store,
        SyncRuntimeConfig {
            max_rounds: 4,
            ..sync_config()
        },
    )
    .expect("runtime");
    let mut transport = ScriptedTransport::new(vec![
        version_verack_script(0),
        version_verack_script(0),
        version_verack_script(0),
    ]);

    // Act
    let summary = runtime
        .sync_until_idle(&mut transport, 1_777_225_157)
        .expect("sync until no progress");
    let state = runtime
        .durable_sync_state_for_summary(&summary, SyncLifecycleState::Active, None, 1_777_225_157)
        .expect("durable status");

    // Assert
    assert_eq!(summary.best_header_height, 0);
    assert_eq!(
        summary.maybe_stop_reason,
        Some(SyncStopReason::NoProgress {
            rounds_completed: 2,
        })
    );
    assert!(summary.health_signals.iter().any(|signal| {
        signal.level == HealthSignalLevel::Warn
            && signal.message.contains("no new header or block progress")
    }));
    assert_eq!(
        state.sync.phase,
        FieldAvailability::available("no_progress".to_string())
    );

    remove_dir_if_exists(&path);
}

#[test]
fn phase70_no_progress_status_projects_at_tip() {
    // Arrange
    let path = temp_store_path("phase70-no-progress-at-tip");
    remove_dir_if_exists(&path);
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let child = build_block(block_hash(&genesis.header), 1);
    save_best_chain_with_active_blocks(
        &path,
        &[(&genesis, 0), (&child, 1)],
        &[(&genesis, 0), (&child, 1)],
    );
    let store = FjallNodeStore::open(&path).expect("store");
    let runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    let summary = runtime.snapshot_summary();

    // Act
    let state = runtime
        .durable_sync_state_for_summary(
            &summary,
            SyncLifecycleState::Active,
            None,
            i64::from(child.header.time),
        )
        .expect("durable at-tip status");

    // Assert
    assert_no_progress_status(
        &state,
        NoProgressDiagnosis::CurrentAtBestKnownTip,
        "Confirm current-at-tip evidence; no sync action is required.",
    );

    remove_dir_if_exists(&path);
}

#[test]
fn phase70_no_progress_status_projects_branch_competition_awaiting_bodies() {
    // Arrange
    let path = temp_store_path("phase70-no-progress-branch-competition");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    let mut summary = SyncRunSummary::empty(3, 2, 1);
    summary.maybe_reconcile_progress =
        Some(SyncReconcileProgress::BranchCompetitionAwaitingBodies {
            missing_count: 2,
            first_missing_height: 2,
            first_missing_hash: "11".repeat(32),
        });

    // Act
    let state = runtime
        .durable_sync_state_for_summary(&summary, SyncLifecycleState::Active, None, 1_777_225_158)
        .expect("durable branch competition status");

    // Assert
    assert_no_progress_status(
        &state,
        NoProgressDiagnosis::BranchCompetitionAwaitingBodies,
        "Wait for replacement branch block bodies before reorg.",
    );

    remove_dir_if_exists(&path);
}

#[test]
fn phase70_no_progress_status_projects_peer_backoff() {
    // Arrange
    let path = temp_store_path("phase70-no-progress-peer-backoff");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    let mut summary = SyncRunSummary::empty(0, 0, 1);
    summary.peer_outcomes.push(peer_outcome(
        SyncPeerAddress::manual("127.0.0.1", 18_444),
        PeerSyncState::Waiting,
        2,
        Some(PeerFailureReason::RetryBackoff),
        Some("peer waiting for retry backoff".to_string()),
    ));

    // Act
    let state = runtime
        .durable_sync_state_for_summary(&summary, SyncLifecycleState::Active, None, 1_777_225_159)
        .expect("durable peer backoff status");

    // Assert
    assert_no_progress_status(
        &state,
        NoProgressDiagnosis::PeerBackoff,
        "Wait for retry backoff or try another configured peer.",
    );

    remove_dir_if_exists(&path);
}

#[test]
fn phase70_no_progress_status_projects_stale_inflight_cleanup() {
    // Arrange
    let path = temp_store_path("phase70-no-progress-stale-inflight");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    runtime
        .inflight_blocks
        .insert(BlockHash::from_byte_array([17_u8; 32]));
    let summary = SyncRunSummary::empty(1, 1, 1);

    // Act
    let state = runtime
        .durable_sync_state_for_summary(&summary, SyncLifecycleState::Active, None, 1_777_225_160)
        .expect("durable stale in-flight status");

    // Assert
    assert_no_progress_status(
        &state,
        NoProgressDiagnosis::StaleInflightCleanup,
        "Wait for stale in-flight block cleanup and reassignment.",
    );

    remove_dir_if_exists(&path);
}

#[test]
fn phase70_no_progress_status_projects_storage_or_resource_blocker() {
    // Arrange
    let path = temp_store_path("phase70-no-progress-storage-blocker");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    let summary = SyncRunSummary::empty(0, 0, 1);

    // Act
    let state = runtime
        .durable_sync_state_for_summary(
            &summary,
            SyncLifecycleState::Active,
            Some("database lock contention".to_string()),
            1_777_225_161,
        )
        .expect("durable storage blocker status");

    // Assert
    assert_no_progress_status(
        &state,
        NoProgressDiagnosis::StorageOrResourceBlocked,
        "Inspect storage health, free disk space for the selected datadir, or increase bounded resource limits.",
    );

    remove_dir_if_exists(&path);
}

#[test]
fn phase78_storage_resource_pressure_outranks_peer_retry_advice() {
    // Arrange
    let path = temp_store_path("phase78-storage-outranks-peer-retry");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    let mut summary = SyncRunSummary::empty(0, 0, 1);
    summary.peer_outcomes.push(peer_outcome(
        SyncPeerAddress::manual("127.0.0.1", 18_444),
        PeerSyncState::Waiting,
        2,
        Some(PeerFailureReason::RetryBackoff),
        Some("peer waiting for retry backoff".to_string()),
    ));

    // Act
    let state = runtime
        .durable_sync_state_for_summary(
            &summary,
            SyncLifecycleState::Active,
            Some("resource limit: storage cache exhausted".to_string()),
            1_777_225_162,
        )
        .expect("durable storage-precedence status");

    // Assert
    assert_eq!(
        state.sync.recovery_category,
        FieldAvailability::available(SyncRecoveryCategory::ResourceExhaustion)
    );
    assert_eq!(
        state.sync.no_progress_diagnosis,
        FieldAvailability::available(NoProgressDiagnosis::StorageOrResourceBlocked)
    );
    let stall = available_stall_diagnosis(&state);
    assert_eq!(
        serialized_label(stall.stalled_subsystem),
        "storage_or_resource_pressure"
    );
    assert_eq!(
        stall.stalled_subsystem,
        StalledSubsystem::StorageOrResourcePressure
    );
    assert_eq!(
        stall.maybe_recovery_category,
        Some(SyncRecoveryCategory::ResourceExhaustion)
    );

    remove_dir_if_exists(&path);
}

#[test]
fn phase78_operator_stop_and_shutdown_classify_local_subsystems() {
    // Arrange
    let path = temp_store_path("phase78-local-stop-classification");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    let cases = [
        (
            SyncStopReason::OperatorPaused,
            "operator_stop",
            StalledSubsystem::OperatorStop,
        ),
        (
            SyncStopReason::ShutdownRequested,
            "local_shutdown",
            StalledSubsystem::LocalShutdown,
        ),
    ];

    // Act
    let states = cases
        .iter()
        .enumerate()
        .map(|(index, (stop_reason, _, _))| {
            let mut summary = SyncRunSummary::empty(0, 0, 1);
            summary.maybe_stop_reason = Some(*stop_reason);
            runtime
                .durable_sync_state_for_summary(
                    &summary,
                    SyncLifecycleState::Active,
                    None,
                    1_777_225_163 + i64::try_from(index).expect("index fits i64"),
                )
                .expect("durable local-stop status")
        })
        .collect::<Vec<_>>();

    // Assert
    for (state, (_, expected_label, expected_subsystem)) in states.iter().zip(cases) {
        let stall = available_stall_diagnosis(state);
        assert_eq!(serialized_label(stall.stalled_subsystem), expected_label);
        assert_eq!(stall.stalled_subsystem, expected_subsystem);
    }

    remove_dir_if_exists(&path);
}

#[test]
fn phase69_fresh_idle_cycle_reports_current_at_best_known_tip() {
    // Arrange
    let path = temp_store_path("phase69-fresh-idle-at-tip");
    remove_dir_if_exists(&path);
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let child = build_block(block_hash(&genesis.header), 1);
    let child_hash = block_hash(&child.header);
    save_best_chain_with_active_blocks(
        &path,
        &[(&genesis, 0), (&child, 1)],
        &[(&genesis, 0), (&child, 1)],
    );
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(
        store,
        SyncRuntimeConfig {
            max_rounds: 4,
            retry_backoff_ms: 1_000,
            ..sync_config()
        },
    )
    .expect("runtime");
    let mut transport =
        ScriptedTransport::new(vec![version_verack_script(1), version_verack_script(1)]);

    // Act
    let summary = runtime
        .sync_until_idle(&mut transport, 1_231_006_531)
        .expect("sync until idle at tip");
    let metadata = runtime
        .store()
        .load_runtime_metadata()
        .expect("load runtime metadata")
        .expect("runtime metadata");
    let state = metadata.maybe_sync_state.expect("persisted sync state");

    // Assert
    assert_eq!(summary.best_header_height, 1);
    assert_eq!(summary.best_block_height, 1);
    assert_eq!(
        summary.maybe_stop_reason,
        Some(SyncStopReason::CurrentAtBestKnownTip {
            best_header_height: 1,
            best_block_height: 1,
        })
    );
    assert!(summary.health_signals.iter().any(|signal| {
        signal.level == HealthSignalLevel::Info
            && signal
                .message
                .contains("current at best-known validated tip")
    }));
    assert_eq!(
        state.sync.stay_current,
        FieldAvailability::available(StayCurrentStatus::CurrentAtBestKnownTip)
    );
    assert_eq!(
        state.sync.stay_current_next_action,
        FieldAvailability::available(
            "No action required; node is current at the best-known validated tip.".to_string(),
        )
    );
    let FieldAvailability::Available(best_known_tip) = state.sync.best_known_tip else {
        panic!("best-known tip should be available");
    };
    assert_eq!(best_known_tip.height, 1);
    assert_eq!(best_known_tip.block_hash, block_hash_hex(child_hash));
    assert_eq!(best_known_tip.freshness, TipFreshnessStatus::Fresh);
    let FieldAvailability::Available(stop_reason) = state.sync.latest_stop_reason else {
        panic!("latest stop reason should be available");
    };
    assert_eq!(stop_reason.label, "current_at_best_known_tip");

    remove_dir_if_exists(&path);
}

#[test]
fn phase69_post_catch_up_new_headers_connect_and_report_stay_current_progress() {
    // Arrange
    let path = temp_store_path("phase69-post-catch-up-new-work");
    remove_dir_if_exists(&path);
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let child = build_block(block_hash(&genesis.header), 1);
    let grandchild = build_block(block_hash(&child.header), 2);
    let grandchild_hash = block_hash(&grandchild.header);
    let expected_grandchild_hash = block_hash_hex(grandchild_hash);
    save_best_chain_with_active_blocks(
        &path,
        &[(&genesis, 0), (&child, 1)],
        &[(&genesis, 0), (&child, 1)],
    );
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(
        store,
        SyncRuntimeConfig {
            max_rounds: 1,
            ..sync_config()
        },
    )
    .expect("runtime");
    let mut transport = ScriptedTransport::new(vec![vec![
        WireNetworkMessage::Version(VersionMessage {
            start_height: 2,
            ..VersionMessage::default()
        }),
        WireNetworkMessage::Verack,
        WireNetworkMessage::Headers(HeadersMessage {
            headers: vec![grandchild.header.clone()],
        }),
        WireNetworkMessage::Block(grandchild.clone()),
    ]]);

    // Act
    let summary = runtime
        .sync_until_idle(&mut transport, i64::from(grandchild.header.time))
        .expect("sync post-catch-up work");
    let metadata = runtime
        .store()
        .load_runtime_metadata()
        .expect("load runtime metadata")
        .expect("runtime metadata");
    let state = metadata.maybe_sync_state.expect("persisted sync state");
    let snapshot = runtime
        .store()
        .load_chainstate_snapshot()
        .expect("load chainstate snapshot")
        .expect("chainstate snapshot");
    let active_tip = snapshot.active_chain.last().expect("active tip");

    // Assert
    assert_eq!(summary.headers_received, 1);
    assert_eq!(summary.blocks_received, 1);
    assert_eq!(summary.best_header_height, 2);
    assert_eq!(summary.best_block_height, 2);
    assert_eq!(summary.downloaded_block_height, 2);
    assert_eq!(
        getdata_block_hashes(&transport.sent_messages()),
        vec![grandchild_hash]
    );
    assert_eq!(
        state.sync.stay_current,
        FieldAvailability::available(StayCurrentStatus::CurrentAtBestKnownTip)
    );
    let FieldAvailability::Available(progress) = state.sync.sync_progress else {
        panic!("sync progress should be available");
    };
    assert_eq!(progress.header_height, 2);
    assert_eq!(progress.validated_active_chain_height, 2);
    assert_eq!(
        progress.maybe_validated_active_chain_hash,
        Some(expected_grandchild_hash.clone())
    );
    let FieldAvailability::Available(best_known_tip) = state.sync.best_known_tip else {
        panic!("best-known tip should be available");
    };
    assert_eq!(best_known_tip.height, 2);
    assert_eq!(best_known_tip.block_hash, expected_grandchild_hash.clone());
    assert_eq!(best_known_tip.work, "3");
    assert_eq!(best_known_tip.freshness, TipFreshnessStatus::Fresh);
    assert_eq!(
        best_known_tip.peer_agreement.first().map(|row| row.status),
        Some(PeerTipAgreementStatus::Agrees)
    );
    assert_eq!(active_tip.height, 2);
    assert_eq!(active_tip.block_hash, grandchild_hash);
    assert_eq!(active_tip.chain_work, 3);
    assert!(
        runtime
            .store()
            .load_block(grandchild_hash)
            .expect("load grandchild block")
            .is_some()
    );

    remove_dir_if_exists(&path);
}

#[test]
fn phase69_headers_only_tip_does_not_report_current() {
    // Arrange
    let path = temp_store_path("phase69-headers-only-not-current");
    remove_dir_if_exists(&path);
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let child = build_block(block_hash(&genesis.header), 1);
    let grandchild = build_block(block_hash(&child.header), 2);
    let child_hash = block_hash(&child.header);
    let grandchild_hash = block_hash(&grandchild.header);
    save_best_chain_with_active_blocks(
        &path,
        &[(&genesis, 0), (&child, 1)],
        &[(&genesis, 0), (&child, 1)],
    );
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(
        store,
        SyncRuntimeConfig {
            max_rounds: 1,
            ..sync_config()
        },
    )
    .expect("runtime");
    let mut transport =
        ScriptedTransport::new(vec![headers_script(2, vec![grandchild.header.clone()])]);

    // Act
    let summary = runtime
        .sync_until_idle(&mut transport, i64::from(grandchild.header.time))
        .expect("sync headers-only tip");
    let metadata = runtime
        .store()
        .load_runtime_metadata()
        .expect("load runtime metadata")
        .expect("runtime metadata");
    let state = metadata.maybe_sync_state.expect("persisted sync state");

    // Assert
    assert_eq!(summary.headers_received, 1);
    assert_eq!(summary.blocks_received, 0);
    assert_eq!(summary.best_header_height, 2);
    assert_eq!(summary.best_block_height, 1);
    assert!(!matches!(
        summary.maybe_stop_reason,
        Some(SyncStopReason::CurrentAtBestKnownTip { .. })
    ));
    assert_eq!(
        state.sync.stay_current,
        FieldAvailability::available(StayCurrentStatus::NoProgress)
    );
    assert_eq!(
        state.sync.stay_current_next_action,
        FieldAvailability::available(
            "Retry sync or inspect peer outcomes; no useful stay-current progress was observed."
                .to_string(),
        )
    );
    let FieldAvailability::Available(progress) = state.sync.sync_progress else {
        panic!("sync progress should be available");
    };
    assert_eq!(progress.header_height, 2);
    assert_eq!(progress.validated_active_chain_height, 1);
    assert_eq!(
        progress.maybe_validated_active_chain_hash,
        Some(block_hash_hex(child_hash))
    );
    let FieldAvailability::Available(best_known_tip) = state.sync.best_known_tip else {
        panic!("best-known tip should be available");
    };
    assert_eq!(best_known_tip.height, 2);
    assert_eq!(best_known_tip.block_hash, block_hash_hex(grandchild_hash));
    assert_eq!(best_known_tip.freshness, TipFreshnessStatus::Fresh);

    remove_dir_if_exists(&path);
}

#[test]
fn phase78_progress_guarantee_projection_rejects_headers_only_as_useful_work() {
    // Arrange
    let path = temp_store_path("phase78-headers-only-progress-guarantee");
    let log_dir = path.join("logs");
    remove_dir_if_exists(&path);
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let child = build_block(block_hash(&genesis.header), 1);
    let grandchild = build_block(block_hash(&child.header), 2);
    save_best_chain_with_active_blocks(
        &path,
        &[(&genesis, 0), (&child, 1)],
        &[(&genesis, 0), (&child, 1)],
    );
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime =
        DurableSyncRuntime::open(store, sync_config_with_log_dir(&log_dir)).expect("runtime");
    let previous_summary = runtime.snapshot_summary();
    let previous_timestamp = u64::from(child.header.time);
    let previous_state = runtime
        .durable_sync_state_for_summary(
            &previous_summary,
            SyncLifecycleState::Active,
            None,
            i64::from(child.header.time),
        )
        .expect("previous durable status");
    runtime
        .persist_durable_sync_state(previous_state)
        .expect("persist previous state");
    let mut transport =
        ScriptedTransport::new(vec![headers_script(2, vec![grandchild.header.clone()])]);

    // Act
    let summary = runtime
        .sync_once(&mut transport, i64::from(grandchild.header.time))
        .expect("headers-only sync");
    let state = runtime
        .store()
        .load_runtime_metadata()
        .expect("load runtime metadata")
        .expect("runtime metadata")
        .maybe_sync_state
        .expect("persisted sync state");
    let records = load_structured_log_records(&log_dir);

    // Assert
    assert_eq!(summary.headers_received, 1);
    assert_eq!(summary.blocks_received, 0);
    assert!(matches!(
        state.sync.progress_credit,
        FieldAvailability::Unavailable { .. }
    ));
    let FieldAvailability::Available(last_work) = state.sync.last_useful_work else {
        panic!("previous durable active-chain work should be carried");
    };
    assert_eq!(
        last_work.kind,
        ProgressCreditKind::ValidatedDurableActiveChain
    );
    assert_eq!(last_work.credited_validated_active_chain_height, 1);
    assert_eq!(
        state.sync.last_successful_progress_unix_seconds,
        FieldAvailability::available(previous_timestamp)
    );
    assert_eq!(
        state.sync.stay_current,
        FieldAvailability::available(StayCurrentStatus::NoProgress)
    );
    assert!(records.iter().any(|record| {
        record.message.contains("progress_credit=unavailable")
            && record
                .message
                .contains("last_useful_work=validated_durable_active_chain:1")
            && record.message.contains("stalled_subsystem=")
    }));

    remove_dir_if_exists(&path);
}

#[test]
fn phase78_branch_competition_does_not_credit_replacement_tip_before_connect() {
    // Arrange
    let path = temp_store_path("phase78-branch-competition-no-credit");
    remove_dir_if_exists(&path);
    let (genesis, branch_a_one, branch_a_two, branch_b_one, branch_b_two, branch_b_three) =
        phase70_branch_blocks();
    save_best_chain_with_active_blocks(
        &path,
        &[
            (&genesis, 0),
            (&branch_b_one, 1),
            (&branch_b_two, 2),
            (&branch_b_three, 3),
        ],
        &[(&genesis, 0), (&branch_a_one, 1), (&branch_a_two, 2)],
    );
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    let previous_summary = runtime.snapshot_summary();
    let previous_state = runtime
        .durable_sync_state_for_summary(
            &previous_summary,
            SyncLifecycleState::Active,
            None,
            i64::from(branch_a_two.header.time),
        )
        .expect("previous durable status");
    runtime
        .persist_durable_sync_state(previous_state)
        .expect("persist previous status");

    // Act
    let progress = super::block_reconcile::reconcile_and_persist_best_chain(
        &mut runtime,
        i64::from(branch_b_three.header.time),
    )
    .expect("reconcile should wait for missing branch bodies");
    let snapshot = runtime.snapshot_summary();
    let state = runtime
        .durable_sync_state_for_summary(
            &snapshot,
            SyncLifecycleState::Active,
            None,
            i64::from(branch_b_three.header.time),
        )
        .expect("durable branch competition status");

    // Assert
    assert!(matches!(
        progress,
        SyncReconcileProgress::BranchCompetitionAwaitingBodies { .. }
    ));
    assert_progress_credit_unavailable(&state);
    let last_work = available_last_useful_work(&state);
    assert_eq!(
        last_work.kind,
        ProgressCreditKind::ValidatedDurableActiveChain
    );
    assert_eq!(last_work.credited_validated_active_chain_height, 2);
    assert_eq!(
        state.sync.no_progress_diagnosis,
        FieldAvailability::available(NoProgressDiagnosis::BranchCompetitionAwaitingBodies)
    );
    let stall = available_stall_diagnosis(&state);
    assert_eq!(
        serialized_label(stall.stalled_subsystem),
        "branch_competition_awaiting_bodies"
    );
    assert_eq!(
        stall.stalled_subsystem,
        StalledSubsystem::BranchCompetitionAwaitingBodies
    );

    remove_dir_if_exists(&path);
}

#[test]
fn phase78_current_at_tip_credits_stay_current_useful_work() {
    // Arrange
    let path = temp_store_path("phase78-current-at-tip-credit");
    remove_dir_if_exists(&path);
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let child = build_block(block_hash(&genesis.header), 1);
    let child_hash = block_hash(&child.header);
    save_best_chain_with_active_blocks(
        &path,
        &[(&genesis, 0), (&child, 1)],
        &[(&genesis, 0), (&child, 1)],
    );
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(
        store,
        SyncRuntimeConfig {
            max_rounds: 2,
            retry_backoff_ms: 1_000,
            ..sync_config()
        },
    )
    .expect("runtime");
    let previous_summary = runtime.snapshot_summary();
    let previous_state = runtime
        .durable_sync_state_for_summary(
            &previous_summary,
            SyncLifecycleState::Active,
            None,
            i64::from(child.header.time),
        )
        .expect("previous durable status");
    assert_eq!(
        available_progress_credit(&previous_state).kind,
        ProgressCreditKind::ValidatedDurableActiveChain
    );
    runtime
        .persist_durable_sync_state(previous_state)
        .expect("persist previous status");
    let mut transport =
        ScriptedTransport::new(vec![version_verack_script(1), version_verack_script(1)]);

    // Act
    let summary = runtime
        .sync_until_idle(&mut transport, i64::from(child.header.time) + 1)
        .expect("sync until current at tip");
    let state = runtime
        .store()
        .load_runtime_metadata()
        .expect("load runtime metadata")
        .expect("runtime metadata")
        .maybe_sync_state
        .expect("persisted sync state");

    // Assert
    assert_eq!(
        summary.maybe_stop_reason,
        Some(SyncStopReason::CurrentAtBestKnownTip {
            best_header_height: 1,
            best_block_height: 1,
        })
    );
    let credit = available_progress_credit(&state);
    assert_eq!(credit.kind, ProgressCreditKind::CurrentAtBestKnownTip);
    assert_eq!(credit.credited_validated_active_chain_height, 1);
    assert_eq!(
        credit.credited_validated_active_chain_hash,
        block_hash_hex(child_hash)
    );
    assert_eq!(
        state.sync.stay_current,
        FieldAvailability::available(StayCurrentStatus::CurrentAtBestKnownTip)
    );
    let stall = available_stall_diagnosis(&state);
    assert_eq!(serialized_label(stall.stalled_subsystem), "at_tip_waiting");
    assert_eq!(stall.stalled_subsystem, StalledSubsystem::AtTipWaiting);

    remove_dir_if_exists(&path);
}

#[test]
fn phase69_stale_tip_is_distinct_from_no_progress() {
    // Arrange
    let path = temp_store_path("phase69-stale-tip");
    remove_dir_if_exists(&path);
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let child = build_block(block_hash(&genesis.header), 1);
    let child_hash = block_hash(&child.header);
    save_best_chain_with_active_blocks(
        &path,
        &[(&genesis, 0), (&child, 1)],
        &[(&genesis, 0), (&child, 1)],
    );
    let store = FjallNodeStore::open(&path).expect("store");
    let runtime = DurableSyncRuntime::open(
        store,
        SyncRuntimeConfig {
            tip_freshness_threshold_seconds: 1_200,
            ..sync_config()
        },
    )
    .expect("runtime");
    let summary = runtime.snapshot_summary();

    // Act
    let state = runtime
        .durable_sync_state_for_summary(
            &summary,
            SyncLifecycleState::Active,
            None,
            i64::from(child.header.time) + 1_201,
        )
        .expect("durable stale-tip status");

    // Assert
    assert_ne!(
        state.sync.stay_current,
        FieldAvailability::available(StayCurrentStatus::NoProgress)
    );
    assert_eq!(
        state.sync.stay_current,
        FieldAvailability::available(StayCurrentStatus::StaleTip)
    );
    assert_eq!(
        state.sync.stay_current_next_action,
        FieldAvailability::available(
            "Refresh peers or wait for fresh peer tip evidence before treating the node as current."
                .to_string(),
        )
    );
    let FieldAvailability::Available(best_known_tip) = state.sync.best_known_tip else {
        panic!("best-known tip should be available");
    };
    assert_eq!(best_known_tip.height, 1);
    assert_eq!(best_known_tip.block_hash, block_hash_hex(child_hash));
    assert_eq!(best_known_tip.freshness, TipFreshnessStatus::Stale);

    remove_dir_if_exists(&path);
}

#[test]
fn phase69_tip_evidence_survives_runtime_reopen() {
    // Arrange
    let path = temp_store_path("phase69-tip-evidence-reopen");
    remove_dir_if_exists(&path);
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let child = build_block(block_hash(&genesis.header), 1);
    let child_hash = block_hash(&child.header);
    let expected_child_hash = block_hash_hex(child_hash);
    save_best_chain_with_active_blocks(
        &path,
        &[(&genesis, 0), (&child, 1)],
        &[(&genesis, 0), (&child, 1)],
    );
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(
        store,
        SyncRuntimeConfig {
            max_rounds: 1,
            ..sync_config()
        },
    )
    .expect("runtime");
    let mut transport = ScriptedTransport::new(vec![version_verack_script(1)]);

    // Act
    runtime
        .sync_until_idle(&mut transport, i64::from(child.header.time) + 30)
        .expect("persist tip evidence");
    let persisted_before = runtime
        .store()
        .load_runtime_metadata()
        .expect("load runtime metadata")
        .expect("runtime metadata")
        .maybe_sync_state
        .expect("persisted sync state");
    drop(runtime);
    let reopened_store = FjallNodeStore::open(&path).expect("reopen store");
    let reopened_runtime =
        DurableSyncRuntime::open(reopened_store, sync_config()).expect("reopen runtime");
    let persisted_after = reopened_runtime
        .store()
        .load_runtime_metadata()
        .expect("load reopened runtime metadata")
        .expect("reopened runtime metadata")
        .maybe_sync_state
        .expect("reopened persisted sync state");
    let reopened_summary = reopened_runtime.snapshot_summary();
    let reopened_state = reopened_runtime
        .durable_sync_state_for_summary(
            &reopened_summary,
            SyncLifecycleState::Active,
            None,
            i64::from(child.header.time) + 30,
        )
        .expect("reopened durable status");

    // Assert
    assert_eq!(
        persisted_before.sync.stay_current,
        FieldAvailability::available(StayCurrentStatus::CurrentAtBestKnownTip)
    );
    assert_eq!(
        persisted_after.sync.stay_current,
        persisted_before.sync.stay_current
    );
    let FieldAvailability::Available(persisted_tip) = persisted_after.sync.best_known_tip else {
        panic!("persisted best-known tip should be available after reopen");
    };
    assert_eq!(persisted_tip.height, 1);
    assert_eq!(persisted_tip.block_hash, expected_child_hash.clone());
    assert_eq!(persisted_tip.freshness, TipFreshnessStatus::Fresh);
    assert_eq!(reopened_summary.best_header_height, 1);
    assert_eq!(reopened_summary.best_block_height, 1);
    assert_eq!(
        reopened_state.sync.stay_current,
        FieldAvailability::available(StayCurrentStatus::CurrentAtBestKnownTip)
    );
    let FieldAvailability::Available(reopened_tip) = reopened_state.sync.best_known_tip else {
        panic!("reopened best-known tip should be available");
    };
    assert_eq!(reopened_tip.height, 1);
    assert_eq!(reopened_tip.block_hash, expected_child_hash);
    assert_eq!(reopened_tip.freshness, TipFreshnessStatus::Fresh);

    remove_dir_if_exists(&path);
}

#[test]
fn sync_once_continues_header_batches_when_peer_advertises_more_work() {
    // Arrange
    let path = temp_store_path("header-batches");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let genesis = header(BlockHash::from_byte_array([0_u8; 32]), 1);
    let child = header(block_hash(&genesis), 2);
    let grandchild = header(block_hash(&child), 3);
    let script = vec![
        WireNetworkMessage::Version(VersionMessage {
            start_height: 2,
            ..VersionMessage::default()
        }),
        WireNetworkMessage::Verack,
        WireNetworkMessage::Headers(HeadersMessage {
            headers: vec![genesis],
        }),
        WireNetworkMessage::Headers(HeadersMessage {
            headers: vec![child, grandchild],
        }),
    ];
    let mut transport = ScriptedTransport::new(vec![script]);
    let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");

    // Act
    let summary = runtime
        .sync_once(&mut transport, 1_777_225_166)
        .expect("sync");

    // Assert
    assert_eq!(summary.best_header_height, 2);
    assert_eq!(summary.best_block_height, 0);
    let getheaders_requests = transport
        .sent_messages()
        .into_iter()
        .filter(|message| matches!(message, WireNetworkMessage::GetHeaders { .. }))
        .count();
    assert!(getheaders_requests >= 2);

    remove_dir_if_exists(&path);
}

#[test]
fn same_datadir_reopen_seeds_headers_from_durable_store() {
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
    let summary = runtime.snapshot_summary();
    let status = runtime
        .durable_sync_state_for_summary(&summary, SyncLifecycleState::Active, None, 1_777_225_176)
        .expect("durable status after restart");

    // Assert
    assert_eq!(summary.best_header_height, 1);
    assert!(matches!(
        status.sync.sync_progress,
        FieldAvailability::Available(SyncProgress {
            header_height: 1,
            ..
        })
    ));

    remove_dir_if_exists(&path);
}

#[test]
fn durable_sync_state_projects_storage_first_recovery_category() {
    // Arrange
    let path = temp_store_path("storage-first-recovery-category");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let metadata = RuntimeMetadata {
        maybe_last_recovery_action: Some(StorageRecoveryAction::Repair),
        ..RuntimeMetadata::default()
    };
    store
        .save_runtime_metadata(&metadata, PersistMode::Sync)
        .expect("save runtime metadata");
    let runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    let summary =
        summary_with_peer_failure(PeerFailureReason::Stall, "peer stalled waiting for headers");

    // Act
    let state = runtime
        .durable_sync_state_for_summary(&summary, SyncLifecycleState::Active, None, 1_777_225_182)
        .expect("durable status");

    // Assert
    assert_eq!(
        state.sync.recovery_category,
        FieldAvailability::available(SyncRecoveryCategory::StoreCorruption)
    );
    assert_eq!(
        state.sync.recovery_action,
        FieldAvailability::available(StorageRecoveryAction::Repair.operator_message().to_string())
    );

    remove_dir_if_exists(&path);
}

#[test]
fn durable_sync_state_storage_metadata_beats_peer_network_last_error_detail() {
    // Arrange
    let path = temp_store_path("storage-metadata-beats-peer-error");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let metadata = RuntimeMetadata {
        maybe_last_recovery_action: Some(StorageRecoveryAction::Repair),
        ..RuntimeMetadata::default()
    };
    store
        .save_runtime_metadata(&metadata, PersistMode::Sync)
        .expect("save runtime metadata");
    let runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    let summary = SyncRunSummary::empty(0, 0, 1);

    // Act
    let state = runtime
        .durable_sync_state_for_summary(
            &summary,
            SyncLifecycleState::Active,
            Some("peer stalled waiting for headers".to_string()),
            1_777_225_183,
        )
        .expect("durable status");

    // Assert
    assert_eq!(
        state.sync.recovery_category,
        FieldAvailability::available(SyncRecoveryCategory::StoreCorruption)
    );

    remove_dir_if_exists(&path);
}

#[test]
fn durable_sync_state_projects_storage_lock_category_from_last_error() {
    // Arrange
    let path = temp_store_path("storage-lock-last-error");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    let summary = SyncRunSummary::empty(0, 0, 1);

    // Act
    let state = runtime
        .durable_sync_state_for_summary(
            &summary,
            SyncLifecycleState::Active,
            Some("database lock contention".to_string()),
            1_777_225_184,
        )
        .expect("durable status");

    // Assert
    assert_eq!(
        state.sync.recovery_category,
        FieldAvailability::available(SyncRecoveryCategory::StorageLockContention)
    );

    remove_dir_if_exists(&path);
}

#[test]
fn durable_sync_state_distinguishes_clean_and_unclean_shutdown_category() {
    // Arrange
    let path = temp_store_path("shutdown-recovery-category");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let clean_metadata = RuntimeMetadata {
        last_clean_shutdown: true,
        ..RuntimeMetadata::default()
    };
    store
        .save_runtime_metadata(&clean_metadata, PersistMode::Sync)
        .expect("save clean runtime metadata");
    let runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    let summary = SyncRunSummary::empty(0, 0, 1);

    // Act
    let clean_state = runtime
        .durable_sync_state_for_summary(&summary, SyncLifecycleState::Stopped, None, 1_777_225_185)
        .expect("clean durable status");
    let unclean_metadata = RuntimeMetadata {
        last_clean_shutdown: false,
        ..RuntimeMetadata::default()
    };
    runtime
        .store()
        .save_runtime_metadata(&unclean_metadata, PersistMode::Sync)
        .expect("save unclean runtime metadata");
    let unclean_state = runtime
        .durable_sync_state_for_summary(
            &summary,
            SyncLifecycleState::Recovering,
            None,
            1_777_225_186,
        )
        .expect("unclean durable status");

    // Assert
    assert_eq!(
        clean_state.sync.recovery_category,
        FieldAvailability::available(SyncRecoveryCategory::CleanShutdown)
    );
    assert_eq!(
        unclean_state.sync.recovery_category,
        FieldAvailability::available(SyncRecoveryCategory::UncleanShutdown)
    );

    remove_dir_if_exists(&path);
}

#[test]
fn contextual_invalid_headers_fail_with_typed_invalid_data() {
    // Arrange
    let path = temp_store_path("invalid-header");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let genesis = header(BlockHash::from_byte_array([0_u8; 32]), 1);
    let mut stale_child = header(block_hash(&genesis), 2);
    stale_child.time = genesis.time;
    let script = vec![
        WireNetworkMessage::Version(VersionMessage {
            start_height: 1,
            ..VersionMessage::default()
        }),
        WireNetworkMessage::Verack,
        WireNetworkMessage::Headers(HeadersMessage {
            headers: vec![genesis, stale_child],
        }),
    ];
    let mut transport = ScriptedTransport::new(vec![script]);
    let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");

    // Act
    let summary = runtime
        .sync_once(&mut transport, 1_777_225_177)
        .expect("sync summary");

    // Assert
    assert_eq!(summary.failed_peers, 1);
    assert!(matches!(
        summary.peer_outcomes.as_slice(),
        [PeerSyncOutcome {
            maybe_failure_reason: Some(PeerFailureReason::InvalidData),
            ..
        }]
    ));
    assert!(summary.health_signals.iter().any(|signal| {
        signal.message == "sync peer sent invalid data: inspect peer compatibility"
    }));

    remove_dir_if_exists(&path);
}

#[test]
fn peer_contribution_rejects_invalid_headers_without_credit() {
    // Arrange
    let path = temp_store_path("peer-contribution-invalid-headers");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let genesis = header(BlockHash::from_byte_array([0_u8; 32]), 1);
    let mut stale_child = header(block_hash(&genesis), 2);
    stale_child.time = genesis.time;
    let script = vec![
        WireNetworkMessage::Version(VersionMessage {
            start_height: 1,
            ..VersionMessage::default()
        }),
        WireNetworkMessage::Verack,
        WireNetworkMessage::Headers(HeadersMessage {
            headers: vec![genesis, stale_child],
        }),
    ];
    let mut transport = ScriptedTransport::new(vec![script]);
    let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");

    // Act
    let summary = runtime
        .sync_once(&mut transport, 1_777_225_178)
        .expect("sync summary");

    // Assert
    assert_eq!(summary.failed_peers, 1);
    assert_eq!(summary.best_header_height, 0);
    assert_eq!(summary.headers_received, 0);
    assert_eq!(summary.blocks_received, 0);
    assert!(matches!(
        summary.peer_outcomes.as_slice(),
        [PeerSyncOutcome {
            maybe_failure_reason: Some(PeerFailureReason::InvalidData),
            contribution: PeerContribution {
                messages_processed: 3,
                headers_received: 0,
                blocks_received: 0,
            },
            maybe_last_activity_unix_seconds: Some(1_777_225_178),
            ..
        }]
    ));

    remove_dir_if_exists(&path);
}

#[test]
fn retry_backoff_waiting_and_stalled_peers_remain_uncredited() {
    // Arrange
    let path = temp_store_path("peer-contribution-waiting-stalled");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(
        store,
        SyncRuntimeConfig {
            manual_peers: vec![SyncPeerAddress::manual("198.51.100.45", 18_444)],
            dns_seeds: Vec::new(),
            retry_backoff_ms: 10_000,
            ..sync_config()
        },
    )
    .expect("runtime");
    let mut transport = ScriptedTransport::new(vec![Vec::new()]);
    let mut resolver = ScriptedResolver::new(vec![
        Ok(vec![resolved_manual_peer("198.51.100.45", 18_444)]),
        Ok(vec![resolved_manual_peer("198.51.100.45", 18_444)]),
    ]);

    // Act
    let stalled_summary = runtime
        .sync_once_with_resolver(&mut transport, &mut resolver, 1_777_225_180)
        .expect("first sync");
    let waiting_summary = runtime
        .sync_once_with_resolver(&mut transport, &mut resolver, 1_777_225_181)
        .expect("second sync");

    // Assert
    assert_eq!(
        stalled_summary.peer_outcomes[0].state,
        PeerSyncState::Stalled
    );
    assert_eq!(stalled_summary.connected_peers, 0);
    assert_eq!(
        stalled_summary.peer_outcomes[0].maybe_failure_reason,
        Some(PeerFailureReason::Stall)
    );
    assert_eq!(
        stalled_summary.peer_outcomes[0].contribution,
        PeerContribution {
            messages_processed: 0,
            headers_received: 0,
            blocks_received: 0,
        }
    );
    assert_eq!(
        waiting_summary.peer_outcomes[0].state,
        PeerSyncState::Waiting
    );
    assert_eq!(
        waiting_summary.peer_outcomes[0].maybe_failure_reason,
        Some(PeerFailureReason::RetryBackoff)
    );
    assert!(
        waiting_summary.peer_outcomes[0]
            .maybe_error
            .as_ref()
            .is_some_and(|message| message.contains("retry backoff wait_seconds=9"))
    );
    assert_eq!(
        waiting_summary.peer_outcomes[0].contribution,
        PeerContribution {
            messages_processed: 0,
            headers_received: 0,
            blocks_received: 0,
        }
    );

    remove_dir_if_exists(&path);
}

#[test]
fn mixed_peer_failures_rotate_to_replacement_without_corrupting_state() {
    // Arrange
    let path = temp_store_path("mixed-peer-failures");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let genesis = header(BlockHash::from_byte_array([0_u8; 32]), 1);
    let mut stale_child = header(block_hash(&genesis), 2);
    stale_child.time = genesis.time;
    let invalid_script = vec![
        WireNetworkMessage::Version(VersionMessage {
            start_height: 1,
            ..VersionMessage::default()
        }),
        WireNetworkMessage::Verack,
        WireNetworkMessage::Headers(HeadersMessage {
            headers: vec![genesis, stale_child],
        }),
    ];
    let mut runtime = DurableSyncRuntime::open(
        store,
        SyncRuntimeConfig {
            manual_peers: vec![
                SyncPeerAddress::manual("198.51.100.40", 18_444),
                SyncPeerAddress::manual("198.51.100.41", 18_445),
                SyncPeerAddress::manual("198.51.100.42", 18_446),
            ],
            dns_seeds: Vec::new(),
            max_peer_retries: 0,
            max_messages_per_peer: 3,
            target_outbound_peers: 1,
            ..sync_config()
        },
    )
    .expect("runtime");
    let mut transport = ScriptedTransport::with_connect_results(vec![
        Err(SyncRuntimeError::Io {
            peer: "198.51.100.40:18444".to_string(),
            message: "scripted disconnect".to_string(),
        }),
        Ok(invalid_script),
        Ok(vec![
            WireNetworkMessage::Version(VersionMessage {
                start_height: 0,
                ..VersionMessage::default()
            }),
            WireNetworkMessage::Verack,
            WireNetworkMessage::SendHeaders,
        ]),
    ]);
    let mut resolver = ScriptedResolver::new(vec![
        Ok(vec![resolved_manual_peer("198.51.100.40", 18_444)]),
        Ok(vec![resolved_manual_peer("198.51.100.41", 18_445)]),
        Ok(vec![resolved_manual_peer("198.51.100.42", 18_446)]),
    ]);

    // Act
    let summary = runtime
        .sync_once_with_resolver(&mut transport, &mut resolver, 1_777_225_183)
        .expect("sync");
    let metadata = runtime
        .store()
        .load_runtime_metadata()
        .expect("load runtime metadata")
        .expect("runtime metadata");
    let durable_sync_state = metadata.maybe_sync_state.expect("durable sync state");

    // Assert
    assert_eq!(summary.attempted_peers, 3);
    assert_eq!(summary.failed_peers, 2);
    assert_eq!(summary.connected_peers, 1);
    assert_eq!(summary.peer_outcomes[0].state, PeerSyncState::Failed);
    assert_eq!(
        summary.peer_outcomes[0].maybe_failure_reason,
        Some(PeerFailureReason::Connect)
    );
    assert_eq!(summary.peer_outcomes[1].state, PeerSyncState::Failed);
    assert_eq!(
        summary.peer_outcomes[1].maybe_failure_reason,
        Some(PeerFailureReason::InvalidData)
    );
    assert_eq!(summary.peer_outcomes[2].state, PeerSyncState::Connected);
    assert_eq!(runtime.snapshot_summary().best_block_height, 0);
    assert_eq!(
        durable_sync_state.sync.lifecycle,
        FieldAvailability::available(SyncLifecycleState::Active)
    );
    assert_eq!(
        durable_sync_state.sync.resource_pressure,
        FieldAvailability::available(SyncResourcePressure {
            blocks_in_flight: 0,
            max_header_requests_in_flight_per_peer: 1,
            max_headers_per_message: 2_000,
            max_blocks_in_flight_per_peer: 16,
            max_blocks_in_flight_total: 64,
            max_messages_per_peer: 3,
            max_sync_rounds: 8,
            outbound_peers: 1,
            target_outbound_peers: 1,
        })
    );
}

#[test]
fn bounded_unattended_cycles_preserve_resource_pressure_and_retention() {
    // Arrange
    let path = temp_store_path("bounded-unattended-cycles");
    let log_dir = path.join("logs");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(
        store,
        SyncRuntimeConfig {
            manual_peers: vec![
                SyncPeerAddress::manual("198.51.100.60", 18_444),
                SyncPeerAddress::manual("198.51.100.61", 18_445),
                SyncPeerAddress::manual("198.51.100.62", 18_446),
                SyncPeerAddress::manual("198.51.100.63", 18_447),
                SyncPeerAddress::manual("198.51.100.64", 18_448),
            ],
            dns_seeds: Vec::new(),
            target_outbound_peers: 2,
            max_messages_per_peer: 3,
            max_rounds: 5,
            max_peer_retries: 0,
            retry_backoff_ms: 10_000,
            max_blocks_in_flight_per_peer: 2,
            max_blocks_in_flight_total: 4,
            maybe_log_dir: Some(log_dir.clone()),
            ..sync_config()
        },
    )
    .expect("runtime");
    let invalid_headers_script = |time: u32| {
        let genesis = header(BlockHash::from_byte_array([0_u8; 32]), time);
        let mut stale_child = header(block_hash(&genesis), time.saturating_add(1));
        stale_child.time = genesis.time;
        vec![
            WireNetworkMessage::Version(VersionMessage {
                start_height: 1,
                ..VersionMessage::default()
            }),
            WireNetworkMessage::Verack,
            WireNetworkMessage::Headers(HeadersMessage {
                headers: vec![genesis, stale_child],
            }),
        ]
    };
    let mut transport = ScriptedTransport::with_connect_results(vec![
        Ok(Vec::new()),
        Ok(invalid_headers_script(100)),
        Ok(version_verack_script(0)),
        Ok(version_verack_script(0)),
        Ok(version_verack_script(0)),
        Ok(version_verack_script(0)),
        Ok(version_verack_script(0)),
        Ok(invalid_headers_script(120)),
        Ok(version_verack_script(0)),
    ]);
    let mut resolver = ScriptedResolver::new(Vec::new());
    let load_pressure = |runtime: &DurableSyncRuntime| {
        let metadata = runtime
            .store()
            .load_runtime_metadata()
            .expect("load runtime metadata")
            .expect("runtime metadata");
        let durable_sync_state = metadata.maybe_sync_state.expect("durable sync state");
        match durable_sync_state.sync.resource_pressure {
            FieldAvailability::Available(pressure) => pressure,
            FieldAvailability::Unavailable { reason } => {
                panic!("missing sync resource pressure: {reason}")
            }
        }
    };
    let assert_bounded_pressure = |pressure: &SyncResourcePressure| {
        assert!(pressure.blocks_in_flight <= 4);
        assert_eq!(pressure.max_header_requests_in_flight_per_peer, 1);
        assert_eq!(pressure.max_headers_per_message, 2_000);
        assert_eq!(pressure.max_blocks_in_flight_per_peer, 2);
        assert_eq!(pressure.max_blocks_in_flight_total, 4);
        assert_eq!(pressure.max_messages_per_peer, 3);
        assert_eq!(pressure.max_sync_rounds, 5);
        assert!(pressure.outbound_peers <= 2);
        assert_eq!(pressure.target_outbound_peers, 2);
    };

    // Act
    let first_summary = runtime
        .sync_once_with_resolver(&mut transport, &mut resolver, 1_777_225_200)
        .expect("first sync");
    // durable storage writes are synchronous adapter calls with no queued write backlog.
    let first_pressure = load_pressure(&runtime);
    let first_backoff_keys = runtime.peer_backoff.keys().cloned().collect::<Vec<_>>();
    let second_summary = runtime
        .sync_once_with_resolver(&mut transport, &mut resolver, 1_777_225_201)
        .expect("second sync");
    let second_pressure = load_pressure(&runtime);
    let third_summary = runtime
        .sync_once_with_resolver(&mut transport, &mut resolver, 1_777_225_211)
        .expect("third sync");
    let third_pressure = load_pressure(&runtime);
    let metrics_retention = MetricRetentionPolicy::default();
    let log_retention = LogRetentionPolicy::default();
    let records = load_structured_log_records(&log_dir);

    // Assert
    assert_eq!(first_summary.peer_outcomes[0].state, PeerSyncState::Stalled);
    assert_eq!(first_summary.peer_outcomes[1].state, PeerSyncState::Failed);
    assert_eq!(
        first_summary.peer_outcomes[1].maybe_failure_reason,
        Some(PeerFailureReason::InvalidData)
    );
    assert_eq!(first_summary.connected_peers, 2);
    assert_eq!(first_summary.failed_peers, 1);
    assert_bounded_pressure(&first_pressure);
    assert_eq!(
        first_backoff_keys,
        vec!["127.0.0.1:18444".to_string(), "127.0.0.1:18445".to_string()]
    );
    // peer retry state is keyed by resolved endpoint.
    assert!(
        first_backoff_keys
            .iter()
            .all(|key| key.starts_with("127.0.0.1:"))
    );
    assert!(first_backoff_keys.len() <= runtime.config.target_outbound_peers);
    assert!(first_backoff_keys.len() <= runtime.config.candidate_peers().len());

    assert_eq!(
        second_summary.peer_outcomes[0].state,
        PeerSyncState::Waiting
    );
    assert_eq!(
        second_summary.peer_outcomes[0].maybe_failure_reason,
        Some(PeerFailureReason::RetryBackoff)
    );
    assert_eq!(
        second_summary.peer_outcomes[1].state,
        PeerSyncState::Waiting
    );
    assert_eq!(
        second_summary.peer_outcomes[1].maybe_failure_reason,
        Some(PeerFailureReason::RetryBackoff)
    );
    assert_eq!(second_summary.connected_peers, 2);
    assert_bounded_pressure(&second_pressure);

    assert_eq!(
        third_summary.peer_outcomes[0].state,
        PeerSyncState::Connected
    );
    assert_eq!(third_summary.peer_outcomes[1].state, PeerSyncState::Failed);
    assert_eq!(
        third_summary.peer_outcomes[1].maybe_failure_reason,
        Some(PeerFailureReason::InvalidData)
    );
    assert_eq!(
        third_summary.peer_outcomes[2].state,
        PeerSyncState::Connected
    );
    assert_eq!(third_summary.connected_peers, 2);
    assert_eq!(runtime.peer_backoff.len(), 1);
    assert!(runtime.peer_backoff.contains_key("127.0.0.1:18445"));
    assert!(runtime.peer_backoff.len() <= runtime.config.target_outbound_peers);
    assert_bounded_pressure(&third_pressure);

    assert_eq!(metrics_retention.sample_interval_seconds, 30);
    assert_eq!(metrics_retention.max_samples_per_series, 2_880);
    assert_eq!(metrics_retention.max_age_seconds, 86_400);
    assert_eq!(log_retention.max_files, 14);
    assert_eq!(log_retention.max_age_days, 14);
    assert_eq!(log_retention.max_total_bytes, 268_435_456);
    assert!(!records.is_empty());
    assert!(records.len() <= 32);

    remove_dir_if_exists(&path);
}

#[test]
fn phase71_same_datadir_resume_matrix_covers_clean_unclean_mid_download_mid_connect_and_stale_inflight()
 {
    // Arrange
    let clean_shutdown = "clean_shutdown";
    let clean_path = temp_store_path(clean_shutdown);
    remove_dir_if_exists(&clean_path);
    let clean_store = FjallNodeStore::open(&clean_path).expect("clean store");
    clean_store
        .save_runtime_metadata(
            &RuntimeMetadata {
                last_clean_shutdown: true,
                ..RuntimeMetadata::default()
            },
            PersistMode::Sync,
        )
        .expect("save clean shutdown metadata");
    let clean_runtime =
        DurableSyncRuntime::open(clean_store, sync_config()).expect("clean runtime");

    let unclean_shutdown = "unclean_shutdown";
    let unclean_path = temp_store_path(unclean_shutdown);
    remove_dir_if_exists(&unclean_path);
    let unclean_store = FjallNodeStore::open(&unclean_path).expect("unclean store");
    unclean_store
        .save_runtime_metadata(
            &RuntimeMetadata {
                last_clean_shutdown: false,
                ..RuntimeMetadata::default()
            },
            PersistMode::Sync,
        )
        .expect("save unclean shutdown metadata");
    let unclean_runtime =
        DurableSyncRuntime::open(unclean_store, sync_config()).expect("unclean runtime");

    let mid_download_interruption = "mid_download_interruption";
    let mid_download_path = temp_store_path(mid_download_interruption);
    remove_dir_if_exists(&mid_download_path);
    let mid_download_genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let mid_download_child_one = build_block(block_hash(&mid_download_genesis.header), 1);
    let mid_download_child_two = build_block(block_hash(&mid_download_child_one.header), 2);
    save_chain_headers_snapshot_and_blocks(
        &mid_download_path,
        &[
            (&mid_download_genesis, 0),
            (&mid_download_child_one, 1),
            (&mid_download_child_two, 2),
        ],
        &[(&mid_download_genesis, 0)],
        &[(&mid_download_genesis, 0), (&mid_download_child_one, 1)],
    );

    let mid_connect_interruption = "mid_connect_interruption";
    let mid_connect_path = temp_store_path(mid_connect_interruption);
    remove_dir_if_exists(&mid_connect_path);
    let mid_connect_genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let mid_connect_child = build_block(block_hash(&mid_connect_genesis.header), 1);
    let mid_connect_missing = build_block(block_hash(&mid_connect_child.header), 2);
    let mid_connect_child_hash = block_hash(&mid_connect_child.header);
    let mid_connect_missing_hash = block_hash(&mid_connect_missing.header);
    save_chain_headers_snapshot_and_blocks(
        &mid_connect_path,
        &[
            (&mid_connect_genesis, 0),
            (&mid_connect_child, 1),
            (&mid_connect_missing, 2),
        ],
        &[(&mid_connect_genesis, 0), (&mid_connect_child, 1)],
        &[(&mid_connect_genesis, 0), (&mid_connect_child, 1)],
    );

    let stale_inflight_after_reopen = "stale_inflight_after_reopen";
    let stale_inflight_path = temp_store_path(stale_inflight_after_reopen);
    remove_dir_if_exists(&stale_inflight_path);
    let stale_genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let stale_child = build_block(block_hash(&stale_genesis.header), 1);
    let stale_missing = build_block(block_hash(&stale_child.header), 2);
    let stale_missing_hash = block_hash(&stale_missing.header);
    save_chain_headers_snapshot_and_blocks(
        &stale_inflight_path,
        &[(&stale_genesis, 0), (&stale_child, 1), (&stale_missing, 2)],
        &[(&stale_genesis, 0), (&stale_child, 1)],
        &[(&stale_genesis, 0), (&stale_child, 1)],
    );

    // Act
    let clean_state = clean_runtime
        .durable_sync_state_for_summary(
            &SyncRunSummary::empty(0, 0, 1),
            SyncLifecycleState::Stopped,
            None,
            1_777_225_220,
        )
        .expect("clean shutdown state");
    let unclean_state = unclean_runtime
        .durable_sync_state_for_summary(
            &SyncRunSummary::empty(0, 0, 1),
            SyncLifecycleState::Recovering,
            None,
            1_777_225_221,
        )
        .expect("unclean shutdown state");

    let mid_download_store = FjallNodeStore::open(&mid_download_path).expect("mid-download store");
    let mid_download_runtime =
        DurableSyncRuntime::open(mid_download_store, sync_config()).expect("mid-download runtime");
    let mid_download_summary = mid_download_runtime.snapshot_summary();
    let mid_download_state = mid_download_runtime
        .durable_sync_state_for_summary(
            &mid_download_summary,
            SyncLifecycleState::Active,
            None,
            i64::from(mid_download_child_two.header.time),
        )
        .expect("mid-download state");

    let mid_connect_store = FjallNodeStore::open(&mid_connect_path).expect("mid-connect store");
    let mut mid_connect_runtime =
        DurableSyncRuntime::open(mid_connect_store, sync_config()).expect("mid-connect runtime");
    let mid_connect_summary_before = mid_connect_runtime.snapshot_summary();
    let mut mid_connect_transport = ScriptedTransport::new(vec![version_verack_script(2)]);
    let mid_connect_summary_after = mid_connect_runtime
        .sync_once(
            &mut mid_connect_transport,
            i64::from(mid_connect_missing.header.time),
        )
        .expect("mid-connect resume sync");
    let mid_connect_requested = getdata_block_hashes(&mid_connect_transport.sent_messages());

    let stale_store = FjallNodeStore::open(&stale_inflight_path).expect("stale in-flight store");
    let mut stale_runtime =
        DurableSyncRuntime::open(stale_store, sync_config()).expect("stale in-flight runtime");
    stale_runtime.inflight_blocks.insert(stale_missing_hash);
    let stale_summary = SyncRunSummary::empty(1, 1, 1);
    let stale_state = stale_runtime
        .durable_sync_state_for_summary(
            &stale_summary,
            SyncLifecycleState::Active,
            None,
            i64::from(stale_missing.header.time),
        )
        .expect("stale in-flight state");
    drop(stale_runtime);
    let reopened_stale_store =
        FjallNodeStore::open(&stale_inflight_path).expect("reopened stale store");
    let reopened_stale_runtime = DurableSyncRuntime::open(reopened_stale_store, sync_config())
        .expect("reopened stale runtime");
    let reopened_stale_summary = reopened_stale_runtime.snapshot_summary();

    // Assert
    assert_eq!(
        clean_state.sync.recovery_category,
        FieldAvailability::available(SyncRecoveryCategory::CleanShutdown),
        "{clean_shutdown}"
    );
    assert_eq!(
        unclean_state.sync.recovery_category,
        FieldAvailability::available(SyncRecoveryCategory::UncleanShutdown),
        "{unclean_shutdown}"
    );

    assert_eq!(mid_download_summary.best_header_height, 2);
    assert_eq!(mid_download_summary.downloaded_block_height, 1);
    assert_eq!(mid_download_summary.best_block_height, 0);
    assert_eq!(
        mid_download_summary.maybe_downloaded_block_hash,
        Some(block_hash_hex(block_hash(&mid_download_child_one.header)))
    );
    let FieldAvailability::Available(mid_download_pressure) =
        mid_download_state.sync.resource_pressure
    else {
        panic!("missing {mid_download_interruption} resource pressure");
    };
    assert_eq!(mid_download_pressure.blocks_in_flight, 0);

    assert_eq!(mid_connect_summary_before.best_header_height, 2);
    assert_eq!(mid_connect_summary_before.best_block_height, 1);
    assert_eq!(
        mid_connect_summary_before.maybe_connected_block_hash,
        Some(block_hash_hex(mid_connect_child_hash))
    );
    assert_eq!(
        mid_connect_summary_before.maybe_validated_active_chain_work,
        Some("2".to_string())
    );
    assert_eq!(mid_connect_summary_after.best_block_height, 1);
    assert!(!mid_connect_requested.contains(&mid_connect_child_hash));
    assert!(mid_connect_requested.contains(&mid_connect_missing_hash));

    assert_no_progress_status(
        &stale_state,
        NoProgressDiagnosis::StaleInflightCleanup,
        "Wait for stale in-flight block cleanup and reassignment.",
    );
    let FieldAvailability::Available(stale_pressure) = stale_state.sync.resource_pressure else {
        panic!("missing {stale_inflight_after_reopen} resource pressure");
    };
    assert_eq!(stale_pressure.blocks_in_flight, 1);
    assert!(reopened_stale_runtime.inflight_blocks.is_empty());
    assert_eq!(reopened_stale_summary.downloaded_block_height, 1);
    assert_eq!(reopened_stale_summary.best_block_height, 1);
    assert_eq!(
        reopened_stale_summary.maybe_connected_block_hash,
        Some(block_hash_hex(block_hash(&stale_child.header)))
    );

    drop(clean_runtime);
    drop(unclean_runtime);
    drop(mid_download_runtime);
    drop(mid_connect_runtime);
    drop(reopened_stale_runtime);
    remove_dir_if_exists(&clean_path);
    remove_dir_if_exists(&unclean_path);
    remove_dir_if_exists(&mid_download_path);
    remove_dir_if_exists(&mid_connect_path);
    remove_dir_if_exists(&stale_inflight_path);
}

#[test]
fn phase71_synthetic_long_chain_exercises_resource_bounds_without_public_network() {
    // Arrange
    const SYNTHETIC_BLOCKS: usize = 48;

    let path = temp_store_path("phase71-synthetic-long-chain");
    let log_dir = path.join("logs");
    remove_dir_if_exists(&path);
    let blocks = {
        let mut blocks = Vec::with_capacity(SYNTHETIC_BLOCKS);
        let mut previous_hash = BlockHash::from_byte_array([0_u8; 32]);
        for height in 0..SYNTHETIC_BLOCKS {
            let block = build_block(previous_hash, height as u32);
            previous_hash = block_hash(&block.header);
            blocks.push(block);
        }
        blocks
    };
    let all_headers = blocks
        .iter()
        .map(|block| block.header.clone())
        .collect::<Vec<_>>();
    let first_peer_blocks = blocks
        .iter()
        .take(9)
        .cloned()
        .map(WireNetworkMessage::Block);
    let second_peer_blocks = blocks
        .iter()
        .skip(9)
        .take(10)
        .cloned()
        .map(WireNetworkMessage::Block);
    let mut first_peer_script = headers_script(47, all_headers);
    first_peer_script.extend(first_peer_blocks);
    let mut second_peer_script = version_verack_script(47);
    second_peer_script.extend(second_peer_blocks);
    let config = SyncRuntimeConfig {
        manual_peers: vec![
            SyncPeerAddress::manual("127.0.0.1", 18_444),
            SyncPeerAddress::manual("127.0.0.1", 18_445),
        ],
        dns_seeds: Vec::new(),
        target_outbound_peers: 2,
        max_blocks_in_flight_per_peer: 2,
        max_blocks_in_flight_total: 4,
        max_messages_per_peer: 12,
        max_rounds: 32,
        max_peer_retries: 0,
        maybe_log_dir: Some(log_dir.clone()),
        ..sync_config()
    };
    assert!(config.dns_seeds.is_empty());
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(store, config.clone()).expect("runtime");
    let mut transport = ScriptedTransport::new(vec![first_peer_script, second_peer_script]);
    let load_pressure = |runtime: &DurableSyncRuntime| {
        let metadata = runtime
            .store()
            .load_runtime_metadata()
            .expect("load runtime metadata")
            .expect("runtime metadata");
        let durable_sync_state = metadata.maybe_sync_state.expect("durable sync state");
        match durable_sync_state.sync.resource_pressure {
            FieldAvailability::Available(pressure) => pressure,
            FieldAvailability::Unavailable { reason } => {
                panic!("missing synthetic long-chain resource pressure: {reason}")
            }
        }
    };

    // Act
    let partial_summary = runtime
        .sync_once(&mut transport, i64::from(blocks[18].header.time))
        .expect("partial synthetic sync");
    let partial_pressure = load_pressure(&runtime);
    let metrics_status = runtime
        .store()
        .load_metrics_status(MetricRetentionPolicy::default())
        .expect("metrics status");
    let metrics_retention = MetricRetentionPolicy::default();
    let log_retention = LogRetentionPolicy::default();
    let log_status = load_log_status(&log_dir, LogRetentionPolicy::default(), 10);
    drop(runtime);

    let reopened_store = FjallNodeStore::open(&path).expect("reopen store");
    let mut reopened_runtime =
        DurableSyncRuntime::open(reopened_store, config).expect("reopened runtime");
    let reopened_summary = reopened_runtime.snapshot_summary();
    let connected_index =
        usize::try_from(reopened_summary.best_block_height).expect("connected height fits usize");
    let connected_hash = block_hash(&blocks[connected_index].header);
    let next_missing_hash = block_hash(&blocks[connected_index + 1].header);
    let mut resume_transport =
        ScriptedTransport::new(vec![version_verack_script(47), version_verack_script(47)]);
    let resume_summary = reopened_runtime
        .sync_once(
            &mut resume_transport,
            i64::from(blocks[connected_index + 1].header.time),
        )
        .expect("resume sync");
    let resume_pressure = load_pressure(&reopened_runtime);
    let resume_requested_hashes = getdata_block_hashes(&resume_transport.sent_messages());

    // Assert
    assert_eq!(blocks.len(), 48);
    assert_eq!(partial_summary.best_header_height, 47);
    assert!(partial_summary.blocks_received > 0);
    assert!(partial_summary.best_block_height < partial_summary.best_header_height);
    assert!(partial_pressure.blocks_in_flight <= 4);
    assert!(partial_pressure.outbound_peers <= 2);
    assert_eq!(partial_pressure.target_outbound_peers, 2);
    assert_eq!(partial_pressure.max_blocks_in_flight_per_peer, 2);
    assert_eq!(partial_pressure.max_blocks_in_flight_total, 4);
    assert_eq!(partial_pressure.max_messages_per_peer, 12);
    assert_eq!(partial_pressure.max_sync_rounds, 32);

    assert_eq!(metrics_status.retention, MetricRetentionPolicy::default());
    assert_eq!(metrics_retention.sample_interval_seconds, 30);
    assert_eq!(metrics_retention.max_samples_per_series, 2_880);
    assert_eq!(metrics_retention.max_age_seconds, 86_400);
    assert_eq!(log_status.retention, LogRetentionPolicy::default());
    assert_eq!(log_retention.max_files, 14);
    assert_eq!(log_retention.max_age_days, 14);
    assert_eq!(log_retention.max_total_bytes, 268_435_456);

    assert!(reopened_runtime.inflight_blocks.is_empty());
    assert_eq!(reopened_summary.best_header_height, 47);
    assert_eq!(
        reopened_summary.downloaded_block_height,
        partial_summary.downloaded_block_height
    );
    assert_eq!(
        reopened_summary.best_block_height,
        partial_summary.best_block_height
    );
    assert_eq!(
        reopened_summary.maybe_connected_block_hash,
        partial_summary.maybe_connected_block_hash
    );
    assert!(!resume_requested_hashes.contains(&connected_hash));
    assert!(resume_requested_hashes.contains(&next_missing_hash));
    assert_eq!(resume_summary.best_header_height, 47);
    assert!(resume_pressure.blocks_in_flight <= 4);
    assert!(resume_pressure.outbound_peers <= 2);
    assert_eq!(resume_pressure.max_messages_per_peer, 12);
    assert_eq!(resume_pressure.max_sync_rounds, 32);

    drop(reopened_runtime);
    remove_dir_if_exists(&path);
}

#[test]
fn competing_header_branch_wins_after_restart_when_it_extends_farther() {
    // Arrange
    let path = temp_store_path("header-fork");
    remove_dir_if_exists(&path);
    let genesis = header(BlockHash::from_byte_array([0_u8; 32]), 1);
    let branch_a_one = header(block_hash(&genesis), 2);
    let branch_a_two = header(block_hash(&branch_a_one), 3);
    {
        let store = FjallNodeStore::open(&path).expect("store");
        let mut transport = ScriptedTransport::new(vec![vec![
            WireNetworkMessage::Version(VersionMessage {
                start_height: 2,
                ..VersionMessage::default()
            }),
            WireNetworkMessage::Verack,
            WireNetworkMessage::Headers(HeadersMessage {
                headers: vec![genesis.clone(), branch_a_one, branch_a_two],
            }),
        ]]);
        let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
        runtime
            .sync_once(&mut transport, 1_777_225_188)
            .expect("initial branch imports");
    }

    // Act
    let store = FjallNodeStore::open(&path).expect("reopen store");
    let branch_b_one = header(block_hash(&genesis), 4);
    let branch_b_two = header(block_hash(&branch_b_one), 5);
    let branch_b_three = header(block_hash(&branch_b_two), 6);
    let mut transport = ScriptedTransport::new(vec![vec![
        WireNetworkMessage::Version(VersionMessage {
            start_height: 3,
            ..VersionMessage::default()
        }),
        WireNetworkMessage::Verack,
        WireNetworkMessage::Headers(HeadersMessage {
            headers: vec![branch_b_one, branch_b_two, branch_b_three],
        }),
    ]]);
    let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    let summary = runtime
        .sync_once(&mut transport, 1_777_225_199)
        .expect("fork extends");

    // Assert
    assert_eq!(summary.best_header_height, 3);
    assert_eq!(runtime.snapshot_summary().best_header_height, 3);
    assert_eq!(
        runtime
            .store()
            .load_header_entries()
            .expect("load headers")
            .expect("headers")
            .entries
            .len(),
        6
    );

    remove_dir_if_exists(&path);
}

#[test]
fn same_datadir_reopen_does_not_duplicate_connected_block_getdata() {
    // Arrange
    let path = temp_store_path("restart-block-reconnect");
    remove_dir_if_exists(&path);
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let genesis_hash = block_hash(&genesis.header);
    {
        let store = FjallNodeStore::open(&path).expect("store");
        store
            .save_header_entries(
                &[open_bitcoin_network::HeaderEntry {
                    block_hash: genesis_hash,
                    header: genesis.header.clone(),
                    height: 0,
                    chain_work: 1,
                }],
                PersistMode::Sync,
            )
            .expect("save headers");
        store
            .save_block(&genesis, PersistMode::Sync)
            .expect("save block");
    }

    let store = FjallNodeStore::open(&path).expect("reopen store");
    let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    let mut transport = ScriptedTransport::new(vec![version_verack_script(0)]);

    // Act
    let summary = runtime
        .sync_once(&mut transport, i64::from(genesis.header.time))
        .expect("sync");

    // Assert
    assert_eq!(summary.best_header_height, 0);
    assert_eq!(summary.best_block_height, 0);
    assert_eq!(summary.blocks_received, 0);
    assert_eq!(runtime.snapshot_summary().best_block_height, 0);
    let requested_hashes = getdata_block_hashes(&transport.sent_messages());
    assert!(!requested_hashes.contains(&genesis_hash));
    assert!(requested_hashes.is_empty());
    let durable_summary = runtime.snapshot_summary();
    assert_eq!(durable_summary.best_block_height, 0);
    assert_eq!(
        durable_summary.maybe_connected_block_hash,
        Some(block_hash_hex(genesis_hash))
    );

    remove_dir_if_exists(&path);
}

#[test]
fn same_datadir_reopen_reports_downloaded_and_connected_block_hashes_after_partial_download() {
    // Arrange
    let path = temp_store_path("restart-partial-download-status");
    remove_dir_if_exists(&path);
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let child_one = build_block(block_hash(&genesis.header), 1);
    let child_two = build_block(block_hash(&child_one.header), 2);
    {
        let store = FjallNodeStore::open(&path).expect("store");
        store
            .save_header_entries(
                &[
                    HeaderEntry {
                        block_hash: block_hash(&genesis.header),
                        header: genesis.header.clone(),
                        height: 0,
                        chain_work: 1,
                    },
                    HeaderEntry {
                        block_hash: block_hash(&child_one.header),
                        header: child_one.header.clone(),
                        height: 1,
                        chain_work: 2,
                    },
                    HeaderEntry {
                        block_hash: block_hash(&child_two.header),
                        header: child_two.header.clone(),
                        height: 2,
                        chain_work: 3,
                    },
                ],
                PersistMode::Sync,
            )
            .expect("save headers");
        store
            .save_block(&genesis, PersistMode::Sync)
            .expect("save genesis");
        store
            .save_block(&child_one, PersistMode::Sync)
            .expect("save child one");
    }

    let store = FjallNodeStore::open(&path).expect("reopen store");
    let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    let mut transport = ScriptedTransport::new(vec![version_verack_script(2)]);

    // Act
    let summary = runtime
        .sync_once(&mut transport, i64::from(child_two.header.time))
        .expect("sync after restart");

    // Assert
    assert_eq!(summary.best_header_height, 2);
    assert_eq!(summary.downloaded_block_height, 1);
    assert_eq!(summary.best_block_height, 1);
    assert_eq!(
        summary.maybe_downloaded_block_hash,
        Some(block_hash_hex(block_hash(&child_one.header)))
    );
    assert_eq!(
        summary.maybe_connected_block_hash,
        Some(block_hash_hex(block_hash(&child_one.header)))
    );
    assert_eq!(
        summary.sync_status(SyncNetwork::Regtest).sync_progress,
        FieldAvailability::available(SyncProgress {
            header_height: 2,
            block_height: 1,
            downloaded_block_height: 1,
            connected_block_height: 1,
            validated_active_chain_height: 1,
            maybe_downloaded_block_hash: Some(block_hash_hex(block_hash(&child_one.header))),
            maybe_connected_block_hash: Some(block_hash_hex(block_hash(&child_one.header))),
            maybe_validated_active_chain_hash: Some(block_hash_hex(block_hash(&child_one.header))),
            maybe_validated_active_chain_work: Some("2".to_string()),
            progress_ratio: 0.5,
            messages_processed: 2,
            headers_received: 0,
            blocks_received: 0,
        })
    );
    assert!(
        transport
            .sent_messages()
            .iter()
            .any(|message| matches!(message, WireNetworkMessage::GetData(_)))
    );
    let metadata = runtime
        .store()
        .load_runtime_metadata()
        .expect("load runtime metadata")
        .expect("runtime metadata");
    let durable_progress = metadata
        .maybe_sync_state
        .expect("durable sync state")
        .sync
        .sync_progress;
    assert_eq!(
        durable_progress,
        FieldAvailability::available(SyncProgress {
            header_height: 2,
            block_height: 1,
            downloaded_block_height: 1,
            connected_block_height: 1,
            validated_active_chain_height: 1,
            maybe_downloaded_block_hash: Some(block_hash_hex(block_hash(&child_one.header))),
            maybe_connected_block_hash: Some(block_hash_hex(block_hash(&child_one.header))),
            maybe_validated_active_chain_hash: Some(block_hash_hex(block_hash(&child_one.header))),
            maybe_validated_active_chain_work: Some("2".to_string()),
            progress_ratio: 0.5,
            messages_processed: 2,
            headers_received: 0,
            blocks_received: 0,
        })
    );

    remove_dir_if_exists(&path);
}

#[test]
fn invalid_block_body_is_peer_attributed_and_not_persisted() {
    // Arrange
    let path = temp_store_path("invalid-block-body");
    remove_dir_if_exists(&path);
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let genesis_hash = block_hash(&genesis.header);
    let mut invalid_genesis = genesis.clone();
    invalid_genesis.transactions[0].outputs[0].value = Amount::from_sats(51).expect("valid amount");
    let script = vec![
        WireNetworkMessage::Version(VersionMessage {
            start_height: 0,
            ..VersionMessage::default()
        }),
        WireNetworkMessage::Verack,
        WireNetworkMessage::Headers(HeadersMessage {
            headers: vec![genesis.header.clone()],
        }),
        WireNetworkMessage::Block(invalid_genesis),
    ];
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    let mut transport = ScriptedTransport::new(vec![script]);

    // Act
    let summary = runtime
        .sync_once(&mut transport, i64::from(genesis.header.time))
        .expect("sync records peer failure");

    // Assert
    assert_eq!(summary.failed_peers, 1);
    assert_eq!(summary.connected_peers, 0);
    assert_eq!(summary.headers_received, 1);
    assert_eq!(summary.blocks_received, 0);
    assert_eq!(summary.downloaded_block_height, 0);
    assert_eq!(summary.best_block_height, 0);
    let outcome = &summary.peer_outcomes[0];
    assert_eq!(outcome.state, PeerSyncState::Failed);
    assert_eq!(
        outcome.maybe_failure_reason,
        Some(PeerFailureReason::InvalidBlock)
    );
    assert_eq!(outcome.contribution.headers_received, 1);
    assert_eq!(outcome.contribution.blocks_received, 0);
    assert!(
        outcome
            .maybe_error
            .as_ref()
            .is_some_and(|message| message.contains("invalid data"))
    );
    assert!(
        runtime
            .store()
            .load_block(genesis_hash)
            .expect("load rejected block")
            .is_none()
    );
    let snapshot = runtime
        .store()
        .load_chainstate_snapshot()
        .expect("load chainstate snapshot")
        .expect("chainstate snapshot");
    assert!(snapshot.active_chain.is_empty());
    let metadata = runtime
        .store()
        .load_runtime_metadata()
        .expect("load runtime metadata")
        .expect("runtime metadata");
    let durable_state = metadata.maybe_sync_state.expect("durable sync state");
    assert_eq!(
        durable_state.sync.lifecycle,
        FieldAvailability::available(SyncLifecycleState::Active)
    );
    assert!(matches!(
        durable_state.sync.last_error,
        FieldAvailability::Available(ref value) if value.contains("invalid data")
    ));
    assert!(matches!(
        durable_state.sync.recovery_action,
        FieldAvailability::Available(ref value) if value.contains("different peer")
    ));

    remove_dir_if_exists(&path);
}

#[test]
fn same_datadir_reopen_connects_best_available_branch_when_blocks_are_already_local() {
    // Arrange
    let path = temp_store_path("restart-branch-reorg");
    remove_dir_if_exists(&path);
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let branch_a_one = build_block(block_hash(&genesis.header), 1);
    let branch_a_two = build_block(block_hash(&branch_a_one.header), 2);
    {
        let store = FjallNodeStore::open(&path).expect("store");
        let mut transport = ScriptedTransport::new(vec![vec![
            WireNetworkMessage::Version(VersionMessage {
                start_height: 2,
                ..VersionMessage::default()
            }),
            WireNetworkMessage::Verack,
            WireNetworkMessage::Headers(HeadersMessage {
                headers: vec![
                    genesis.header.clone(),
                    branch_a_one.header.clone(),
                    branch_a_two.header.clone(),
                ],
            }),
            WireNetworkMessage::Block(genesis.clone()),
            WireNetworkMessage::Block(branch_a_one.clone()),
            WireNetworkMessage::Block(branch_a_two.clone()),
        ]]);
        let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
        runtime
            .sync_once(&mut transport, i64::from(branch_a_two.header.time))
            .expect("initial branch sync");
    }

    let branch_b_one = build_branch_block(block_hash(&genesis.header), 1, 100);
    let branch_b_two = build_branch_block(block_hash(&branch_b_one.header), 2, 100);
    let branch_b_three = build_branch_block(block_hash(&branch_b_two.header), 3, 100);
    {
        let store = FjallNodeStore::open(&path).expect("reopen store for durable branch");
        let mut transport = ScriptedTransport::new(vec![vec![
            WireNetworkMessage::Version(VersionMessage {
                start_height: 3,
                ..VersionMessage::default()
            }),
            WireNetworkMessage::Verack,
            WireNetworkMessage::Headers(HeadersMessage {
                headers: vec![
                    branch_b_one.header.clone(),
                    branch_b_two.header.clone(),
                    branch_b_three.header.clone(),
                ],
            }),
        ]]);
        let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
        runtime
            .sync_once(&mut transport, i64::from(branch_b_three.header.time))
            .expect("persist better branch headers");
        runtime
            .store()
            .save_block(&branch_b_one, PersistMode::Sync)
            .expect("save branch b one");
        runtime
            .store()
            .save_block(&branch_b_two, PersistMode::Sync)
            .expect("save branch b two");
        runtime
            .store()
            .save_block(&branch_b_three, PersistMode::Sync)
            .expect("save branch b three");
    }

    let store = FjallNodeStore::open(&path).expect("reopen store");
    let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    let mut transport = ScriptedTransport::new(vec![version_verack_script(3)]);

    // Act
    let summary = runtime
        .sync_once(&mut transport, i64::from(branch_b_three.header.time))
        .expect("sync after restart");

    // Assert
    assert_eq!(summary.best_header_height, 3);
    assert_eq!(summary.best_block_height, 3);
    assert_eq!(summary.blocks_received, 0);
    assert_eq!(runtime.snapshot_summary().best_block_height, 3);
    let requested_hashes = getdata_block_hashes(&transport.sent_messages());
    assert!(requested_hashes.is_empty());

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

mod wallet_rescan_runtime {
    use std::collections::HashMap;

    use open_bitcoin_core::{
        chainstate::{ChainPosition, ChainstateSnapshot, Coin},
        primitives::{BlockHash, BlockHeader, OutPoint, TransactionOutput, Txid},
        wallet::{AddressNetwork, DescriptorRole, Wallet},
    };

    use super::{PersistMode, remove_dir_if_exists, temp_store_path};
    use crate::{
        FjallNodeStore, WalletRegistry, WalletRescanFreshness, WalletRescanJobState,
        sync::WalletRescanRuntime,
    };

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
