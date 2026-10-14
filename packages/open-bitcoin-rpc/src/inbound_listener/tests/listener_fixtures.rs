// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp

use super::*;

pub(super) fn loopback_config(max_peers: usize) -> InboundListenerConfig {
    loopback_config_with_permission_classes(max_peers, 0, PeerPermissionClassRegistry::default())
}

pub(super) fn loopback_config_with_permission_classes(
    max_peers: usize,
    reserved_slots: usize,
    permission_classes: PeerPermissionClassRegistry,
) -> InboundListenerConfig {
    InboundListenerConfig {
        enabled: true,
        listen_addresses: vec!["127.0.0.1:0".to_string()],
        max_peers,
        reserved_slots,
        allow_public: false,
        permission_classes,
    }
}

pub(super) async fn running_loopback_listener(
    max_peers: usize,
) -> (
    Arc<tokio::sync::Mutex<ManagedRpcContext>>,
    super::InboundListenerWorker,
    String,
) {
    running_loopback_listener_with_config(loopback_config(max_peers)).await
}

pub(super) async fn running_loopback_listener_with_config(
    inbound: InboundListenerConfig,
) -> (
    Arc<tokio::sync::Mutex<ManagedRpcContext>>,
    super::InboundListenerWorker,
    String,
) {
    let runtime = RuntimeConfig {
        inbound,
        ..RuntimeConfig::default()
    };
    let context = Arc::new(tokio::sync::Mutex::new(
        ManagedRpcContext::from_fixture_runtime_config(&runtime),
    ));
    let activation = activate_inbound_listener(&runtime.inbound).await;
    let endpoint = activation
        .bound_endpoints()
        .first()
        .expect("bound loopback endpoint")
        .bound_endpoint
        .clone();
    let worker = start_inbound_accept_loop(activation, Arc::clone(&context))
        .expect("listener worker should start");
    (context, worker, endpoint)
}

pub(super) async fn running_loopback_listener_with_announcements() -> (
    Arc<tokio::sync::Mutex<ManagedRpcContext>>,
    super::InboundListenerWorker,
    String,
    AnnouncementOutboxRegistry,
    ManagedNetworkHandle,
) {
    let runtime = RuntimeConfig {
        inbound: loopback_config(2),
        block_serving: BlockRelayActivationPolicy {
            block_serving: BlockServingActivationConfig { enabled: true },
            compact_relay: CompactRelayActivationConfig { enabled: true },
        },
        ..RuntimeConfig::default()
    };
    let network = ManagedNetworkHandle::transient_runtime(
        NetworkMagic::MAINNET,
        8_333,
        runtime.relay,
        runtime.block_serving,
        true,
    );
    let context = Arc::new(tokio::sync::Mutex::new(
        ManagedRpcContext::from_fixture_runtime_config_with_network_handle(
            &runtime,
            network.clone(),
            None,
        )
        .expect("compose announcement listener context"),
    ));
    let outboxes = AnnouncementOutboxRegistry::default();
    let activation = activate_inbound_listener(&runtime.inbound).await;
    let endpoint = activation
        .bound_endpoints()
        .first()
        .expect("bound announcement loopback endpoint")
        .bound_endpoint
        .clone();
    let worker = start_inbound_accept_loop_with_announcements(
        activation,
        Arc::clone(&context),
        PeerIdentityAuthority::default(),
        outboxes.clone(),
        network.clone(),
    )
    .expect("announcement listener worker should start");
    (context, worker, endpoint, outboxes, network)
}

pub(super) fn loopback_permission_registry(permissions: &[&str]) -> PeerPermissionClassRegistry {
    PeerPermissionClassRegistry::new([ParsedPeerPermissionClass::parse(
        "loopback-permission",
        ["127.0.0.1"],
        permissions.iter().copied(),
    )
    .expect("loopback permission class should parse")])
}

pub(super) fn listener_evidence(bound_endpoints: &[&str]) -> InboundListenerEvidence {
    InboundListenerEvidence {
        listener_state: "listening".to_string(),
        preflight_reason: "ready".to_string(),
        bound_endpoints: bound_endpoints
            .iter()
            .map(|endpoint| (*endpoint).to_string())
            .collect(),
        admitted_inbound_peers: 0,
        rejected_inbound_peers: 0,
        resource_rejections: 0,
        timeout_disconnects: 0,
        churn_rejections: 0,
        reconnect_suppressions: 0,
        maybe_admission_reject_reason: None,
        maybe_latest_admission_event: Some("ready".to_string()),
        maybe_latest_resource_event: None,
    }
}

pub(super) fn peer_policy_entry(scope: BanScope, expires_at_unix_seconds: i64) -> PeerBanEntry {
    PeerBanEntry {
        scope,
        reason: BanReason::Manual,
        created_at_unix_seconds: 100,
        expires_at_unix_seconds,
        source: "runtime_test",
    }
}

pub(super) fn inbound_status(context: &ManagedRpcContext) -> InboundPeerServingStatus {
    match context.current_inbound_status() {
        FieldAvailability::Available(status) => status,
        FieldAvailability::Unavailable { reason } => {
            panic!("expected inbound status to be available, got {reason}")
        }
    }
}

pub(super) async fn send_message(stream: &TcpStream, message: WireNetworkMessage) {
    let encoded = message
        .encode_wire(NetworkMagic::MAINNET)
        .expect("encode wire message");
    super::write_all(stream, &encoded)
        .await
        .expect("write wire message");
}

pub(super) async fn receive_message(stream: &TcpStream) -> WireNetworkMessage {
    let policy = InboundEnvelopePolicy::new(NetworkMagic::MAINNET);
    let outcome = super::read_wire_message(stream, &policy)
        .await
        .expect("read wire message");
    match outcome {
        ReadWireMessageOutcome::Message(parsed) => parsed.message,
        ReadWireMessageOutcome::Rejected(event) => {
            panic!("expected inbound response message, got {}", event.label)
        }
    }
}

pub(super) async fn receive_any_message(stream: &TcpStream) -> WireNetworkMessage {
    let mut buffered = Vec::new();
    loop {
        if buffered.len() >= INBOUND_MESSAGE_HEADER_LEN {
            let header = parse_message_header(&buffered[..INBOUND_MESSAGE_HEADER_LEN])
                .expect("response header should decode");
            let frame_len = INBOUND_MESSAGE_HEADER_LEN + header.payload_size as usize;
            if buffered.len() >= frame_len {
                return ParsedNetworkMessage::decode_wire(&buffered[..frame_len])
                    .expect("response should decode")
                    .message;
            }
        }

        stream
            .readable()
            .await
            .expect("response stream should become readable");
        let mut bytes = [0_u8; 4_096];
        match stream.try_read(&mut bytes) {
            Ok(0) => panic!("listener closed before a complete response"),
            Ok(count) => buffered.extend_from_slice(&bytes[..count]),
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => {}
            Err(error) => panic!("response read failed: {error}"),
        }
    }
}

pub(super) async fn tcp_pair() -> (TcpStream, TcpStream) {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind loopback test listener");
    let endpoint = listener.local_addr().expect("loopback listener address");
    let client = TcpStream::connect(endpoint)
        .await
        .expect("connect loopback test client");
    let (server, _) = listener.accept().await.expect("accept loopback test peer");
    (client, server)
}

pub(super) fn verack_header(magic: NetworkMagic) -> [u8; INBOUND_MESSAGE_HEADER_LEN] {
    let encoded = WireNetworkMessage::Verack
        .encode_wire(magic)
        .expect("encode verack message");
    encoded[..INBOUND_MESSAGE_HEADER_LEN]
        .try_into()
        .expect("encoded message should include header")
}

pub(super) fn unsupported_command_header() -> [u8; INBOUND_MESSAGE_HEADER_LEN] {
    let mut header = verack_header(NetworkMagic::MAINNET);
    header[4..16].fill(0);
    header[4..11].copy_from_slice(b"mempool");
    header
}

pub(super) async fn read_rejected_header(header: [u8; INBOUND_MESSAGE_HEADER_LEN]) -> String {
    // Arrange
    let (client, server) = tcp_pair().await;
    let policy = InboundEnvelopePolicy::new(NetworkMagic::MAINNET);

    // Act
    super::write_all(&client, &header)
        .await
        .expect("write header under test");
    let outcome = super::read_wire_message(&server, &policy)
        .await
        .expect("read rejected header");

    // Assert
    match outcome {
        ReadWireMessageOutcome::Rejected(event) => event.label,
        ReadWireMessageOutcome::Message(_) => {
            panic!("expected inbound envelope policy to reject header")
        }
    }
}

pub(super) struct FilterListenerFixture {
    pub path: PathBuf,
    pub blocks: Vec<Block>,
    pub runtime: DurableSyncRuntime,
    pub context: Arc<tokio::sync::Mutex<ManagedRpcContext<FjallChainstateStore, FjallCoinsView>>>,
    pub config: RuntimeConfig,
}

impl FilterListenerFixture {
    pub fn new(general: bool, tokens: &[&str]) -> Self {
        Self::new_with_peer(general, tokens, None)
    }

    pub fn new_with_peer(
        general: bool,
        tokens: &[&str],
        maybe_endpoint: Option<std::net::SocketAddr>,
    ) -> Self {
        use open_bitcoin_node::core::{
            chainstate::{FlushMode, FlushPolicyTime},
            consensus::{ConsensusParams, ScriptVerifyFlags},
        };
        let id = NEXT_DURABLE_BLOCK_SERVING_DIR.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "open-bitcoin-inbound-filter-{}-{id}",
            process::id()
        ));
        let store = FjallNodeStore::open(&path).expect("new real filter store");
        let sync = SyncRuntimeConfig {
            network: SyncNetwork::Regtest,
            dns_seeds: Vec::new(),
            manual_peers: maybe_endpoint
                .into_iter()
                .map(|endpoint| {
                    open_bitcoin_node::SyncPeerAddress::manual("127.0.0.1", endpoint.port())
                })
                .collect(),
            max_messages_per_peer: 3,
            max_peer_retries: 0,
            ..Default::default()
        };
        let seed =
            DurableSyncRuntime::open(store.clone(), sync.clone()).expect("new real authority");
        let mut blocks: Vec<Block> = Vec::new();
        for height in 0..40_u32 {
            let mut block = phase123_mined_block();
            block.header.previous_block_hash =
                blocks.last().map_or(BlockHash::default(), |previous| {
                    block_hash(&previous.header)
                });
            block.header.time = 1_000 + height * 100;
            let height_script = if height == 0 {
                vec![0, 0x51]
            } else {
                vec![1, height as u8, 0x51]
            };
            block.transactions[0].inputs[0].script_sig =
                ScriptBuf::from_bytes(height_script).expect("height");
            block.transactions[0].outputs[0].script_pubkey =
                ScriptBuf::from_bytes(vec![1, height as u8, 0x75, 0x51])
                    .expect("distinct true script");
            block.header.merkle_root = block_merkle_root(&block.transactions).expect("merkle").0;
            block.header.nonce = (0..=u32::MAX)
                .find(|nonce| {
                    block.header.nonce = *nonce;
                    check_block_header(&block.header).is_ok()
                })
                .expect("easy PoW");
            seed.network_handle()
                .connect_local_block(&block, ScriptVerifyFlags::P2SH, ConsensusParams::default())
                .expect("actual validated acceptance");
            store
                .save_block(&block, PersistMode::Sync)
                .expect("retained input body");
            blocks.push(block);
        }
        seed.network_handle()
            .flush_coins(FlushMode::Always, FlushPolicyTime::new(1), u64::MAX)
            .expect("actual durable fence");
        drop(seed);
        let runtime = DurableSyncRuntime::open_with_configured_runtime_activation(
            store.clone(),
            sync,
            Default::default(),
            Default::default(),
            true,
            open_bitcoin_node::chainstate::BasicFilterStartupMode::Enabled,
        )
        .expect("configured real BASIC owner");
        runtime
            .network_handle()
            .configure_basic_peer_serving(general)
            .expect("configured general grant");
        let config = RuntimeConfig {
            chain: AddressNetwork::Regtest,
            inbound: loopback_config_with_permission_classes(
                2,
                1,
                if tokens == ["in"] {
                    PeerPermissionClassRegistry::default()
                } else {
                    loopback_permission_registry(tokens)
                },
            ),
            peer_block_filters: general,
            prune_mode: open_bitcoin_node::core::chainstate::PruneMode::ManualOnly,
            ..Default::default()
        };
        let context = Arc::new(tokio::sync::Mutex::new(
            ManagedRpcContext::from_fixture_runtime_config_with_network_handle(
                &config,
                runtime.network_handle(),
                Some(store),
            )
            .expect("shared configured authority"),
        ));
        Self {
            path,
            blocks,
            runtime,
            context,
            config,
        }
    }

    pub fn hash(&self, height: usize) -> BlockHash {
        block_hash(&self.blocks[height].header)
    }

    pub async fn listener(&self) -> (InboundListenerWorker, String) {
        let activation = activate_inbound_listener(&self.config.inbound).await;
        let endpoint = activation.bound_endpoints()[0].bound_endpoint.clone();
        let worker = start_inbound_accept_loop(activation, Arc::clone(&self.context))
            .expect("actual listener");
        (worker, endpoint)
    }

    pub async fn prepared(
        &self,
    ) -> open_bitcoin_node::network::filter_transport::PreparedFilterResponse {
        let mut context = self.context.lock().await;
        context
            .record_inbound_admission_for_remote_addr(
                160_009,
                "127.0.0.1:18444".parse().expect("remote"),
                false,
            )
            .expect("admission");
        for message in [
            WireNetworkMessage::Version(VersionMessage {
                nonce: 160_009,
                ..Default::default()
            }),
            WireNetworkMessage::Verack,
        ] {
            let plan = context
                .prepare_inbound_wire_message(160_009, message, 1)
                .expect("real handshake");
            drop(plan.resolve());
        }
        let plan = context
            .prepare_inbound_wire_message(
                160_009,
                WireNetworkMessage::GetCFilters(open_bitcoin_network::GetCFilters {
                    filter_type: 0,
                    start_height: 0,
                    stop_hash: self.hash(2),
                }),
                2,
            )
            .expect("real preparation");
        let step = plan
            .resolve()
            .responses
            .pop()
            .expect("complete prepared response");
        let InboundWireStep::Filter(response) = step else {
            panic!("request-bound filter carrier");
        };
        response
    }

    pub fn cleanup(self) {
        let path = self.path.clone();
        drop(self);
        fs::remove_dir_all(path).expect("closed real filter store cleanup");
    }
}

pub(super) async fn wait_for_inbound_peers(
    context: &Arc<tokio::sync::Mutex<ManagedRpcContext>>,
    expected: usize,
) {
    for _ in 0..100 {
        if context
            .lock()
            .await
            .network_info()
            .is_ok_and(|info| info.inbound_peers == expected)
        {
            return;
        }
        tokio::task::yield_now().await;
    }
}

pub(super) async fn wait_for_reserved_slot_rejections(
    context: &Arc<tokio::sync::Mutex<ManagedRpcContext>>,
    expected: usize,
) {
    for _ in 0..100 {
        if context
            .lock()
            .await
            .inbound_admission_info()
            .is_ok_and(|info| info.reserved_slot_rejections == expected)
        {
            return;
        }
        tokio::task::yield_now().await;
    }
}
