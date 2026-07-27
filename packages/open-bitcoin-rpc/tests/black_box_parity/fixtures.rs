// Parity breadcrumbs:
// - packages/bitcoin-knots/src/bitcoind.cpp
// - packages/bitcoin-knots/src/rpc/protocol.h
// - packages/bitcoin-knots/src/rpc/request.cpp
// - packages/bitcoin-knots/src/rpc/server.cpp
// - packages/bitcoin-knots/src/rpc/blockchain.cpp
// - packages/bitcoin-knots/src/rpc/mempool.cpp
// - packages/bitcoin-knots/src/rpc/net.cpp
// - packages/bitcoin-knots/src/rpc/rawtransaction.cpp
// - packages/bitcoin-knots/src/node/context.h
// - packages/bitcoin-knots/src/rpc/server_util.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/test/functional/interface_rpc.py

use super::*;

pub(super) const RPC_USERNAME: &str = "alice";
pub(super) const RPC_PASSWORD: &str = "secret";
pub(super) const SUITE_NAME: &str = "rpc-black-box-parity";
pub(super) const PHASE127_EASY_BITS: u32 = 0x207f_ffff;
pub(super) const PHASE127_RPC_USERNAME: &str = "phase127-rpc-user";
pub(super) const PHASE127_RPC_PASSWORD: &str = "phase127-secret";
pub(super) const PHASE127_FORBIDDEN_PERMISSION: &str = "phase127-private-permission";
pub(super) const WIRE_HEADER_LENGTH: usize = 24;
pub(super) static NEXT_PHASE127_DIR: AtomicU64 = AtomicU64::new(0);

#[derive(Debug)]
pub(super) struct Phase127ScriptedTransport {
    inbound: VecDeque<WireNetworkMessage>,
}

#[derive(Debug)]
pub(super) struct Phase127ScriptedSession {
    inbound: VecDeque<WireNetworkMessage>,
}

pub(super) struct Phase127WirePeer {
    stream: TcpStream,
    buffered: Vec<u8>,
}

impl SyncTransport for Phase127ScriptedTransport {
    type Session = Phase127ScriptedSession;

    fn connect(
        &mut self,
        _peer: &ResolvedSyncPeerAddress,
        _config: &SyncRuntimeConfig,
    ) -> Result<Self::Session, SyncRuntimeError> {
        Ok(Phase127ScriptedSession {
            inbound: core::mem::take(&mut self.inbound),
        })
    }
}

impl SyncPeerSession for Phase127ScriptedSession {
    fn send(
        &mut self,
        _message: &WireNetworkMessage,
        _magic: NetworkMagic,
    ) -> Result<(), SyncRuntimeError> {
        Ok(())
    }

    fn receive(
        &mut self,
        _magic: NetworkMagic,
    ) -> Result<SyncPeerReceiveOutcome, SyncRuntimeError> {
        Ok(self.inbound.pop_front().map_or(
            SyncPeerReceiveOutcome::Closed,
            SyncPeerReceiveOutcome::Message,
        ))
    }
}

impl Phase127WirePeer {
    pub(super) async fn connect(endpoint: &str) -> Self {
        let stream = TcpStream::connect(endpoint)
            .await
            .expect("phase 127 loopback peer should connect");
        Self {
            stream,
            buffered: Vec::new(),
        }
    }

    pub(super) async fn send(&self, message: WireNetworkMessage, magic: NetworkMagic) {
        let bytes = message
            .encode_wire(magic)
            .expect("phase 127 message should encode");
        let mut written = 0;
        while written < bytes.len() {
            self.stream
                .writable()
                .await
                .expect("phase 127 peer should become writable");
            match self.stream.try_write(&bytes[written..]) {
                Ok(0) => panic!("phase 127 peer write made no progress"),
                Ok(count) => written += count,
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {}
                Err(error) => panic!("phase 127 peer write failed: {error}"),
            }
        }
    }

    pub(super) async fn receive(&mut self) -> WireNetworkMessage {
        loop {
            if self.buffered.len() >= WIRE_HEADER_LENGTH {
                let header = parse_message_header(&self.buffered[..WIRE_HEADER_LENGTH])
                    .expect("phase 127 response header should decode");
                let frame_length = WIRE_HEADER_LENGTH + header.payload_size as usize;
                if self.buffered.len() >= frame_length {
                    let frame = self.buffered.drain(..frame_length).collect::<Vec<_>>();
                    return ParsedNetworkMessage::decode_wire(&frame)
                        .expect("phase 127 response should decode")
                        .message;
                }
            }

            self.stream
                .readable()
                .await
                .expect("phase 127 peer should become readable");
            let mut bytes = [0_u8; 4_096];
            match self.stream.try_read(&mut bytes) {
                Ok(0) => panic!("phase 127 listener closed before a complete response"),
                Ok(count) => self.buffered.extend_from_slice(&bytes[..count]),
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {}
                Err(error) => panic!("phase 127 peer read failed: {error}"),
            }
        }
    }
}

pub(super) fn functional_cases() -> Vec<FunctionalCase> {
    vec![
        FunctionalCase {
            name: "getblockchaininfo shape",
            method: "getblockchaininfo",
            params: json!([]),
            expected: ExpectedOutcome::ResultHasKeys(vec![
                "chain",
                "blocks",
                "headers",
                "initialblockdownload",
            ]),
        },
        FunctionalCase {
            name: "getnetworkinfo shape",
            method: "getnetworkinfo",
            params: json!([]),
            expected: ExpectedOutcome::ResultHasKeys(vec![
                "version",
                "subversion",
                "protocolversion",
                "connections",
            ]),
        },
        FunctionalCase {
            name: "getmempoolinfo shape",
            method: "getmempoolinfo",
            params: json!([]),
            expected: ExpectedOutcome::ResultHasKeys(vec!["size", "bytes", "loaded"]),
        },
        FunctionalCase {
            name: "unknown method error shape",
            method: "openbitcoin_does_not_exist",
            params: json!([]),
            expected: ExpectedOutcome::ErrorCode(-32601),
        },
    ]
}

pub(super) fn phase127_data_dir() -> PathBuf {
    std::env::temp_dir().join(format!(
        "open-bitcoin-phase127-black-box-{}-{}",
        process::id(),
        NEXT_PHASE127_DIR.fetch_add(1, Ordering::SeqCst),
    ))
}

pub(super) fn phase127_runtime_config(data_dir: PathBuf) -> RuntimeConfig {
    let permission_classes = PeerPermissionClassRegistry::new([ParsedPeerPermissionClass::parse(
        PHASE127_FORBIDDEN_PERMISSION,
        ["127.0.0.1"],
        ["in", "download", "relay"],
    )
    .expect("phase 127 loopback permission should parse")]);
    RuntimeConfig {
        chain: AddressNetwork::Regtest,
        maybe_data_dir: Some(data_dir),
        inbound: InboundListenerConfig {
            enabled: true,
            listen_addresses: vec!["127.0.0.1:0".to_string()],
            max_peers: 2,
            reserved_slots: 1,
            allow_public: false,
            permission_classes,
        },
        block_serving: BlockRelayActivationPolicy {
            block_serving: BlockServingActivationConfig { enabled: true },
            compact_relay: CompactRelayActivationConfig { enabled: true },
        },
        relay: RelayActivationConfig { enabled: true },
        ..RuntimeConfig::default()
    }
}

pub(super) fn phase127_sync_config() -> SyncRuntimeConfig {
    SyncRuntimeConfig {
        network: SyncNetwork::Regtest,
        manual_peers: vec![SyncPeerAddress::manual("127.0.0.1", 18_444)],
        dns_seeds: Vec::new(),
        target_outbound_peers: 1,
        max_peer_retries: 0,
        max_rounds: 1,
        ..SyncRuntimeConfig::default()
    }
}

pub(super) fn phase127_serialized_script_num(value: u32) -> Vec<u8> {
    if value == 0 {
        return vec![0x00];
    }

    let mut magnitude = value;
    let mut encoded = Vec::new();
    while magnitude > 0 {
        encoded.push((magnitude & 0xff) as u8);
        magnitude >>= 8;
    }
    let mut script = Vec::with_capacity(encoded.len() + 1);
    script.push(encoded.len() as u8);
    script.extend(encoded);
    script
}

pub(super) fn phase127_p2sh_script() -> ScriptBuf {
    let redeem_hash = hash160(&[0x51]);
    let mut bytes = vec![0xa9, 20];
    bytes.extend_from_slice(&redeem_hash);
    bytes.push(0x87);
    ScriptBuf::from_bytes(bytes).expect("phase 127 p2sh script")
}

pub(super) fn phase127_mined_block_after(previous_block_hash: BlockHash, height: u32) -> Block {
    let mut coinbase_script = phase127_serialized_script_num(height);
    coinbase_script.push(0x51);
    let transaction = Transaction {
        version: 1,
        inputs: vec![TransactionInput {
            previous_output: OutPoint::null(),
            script_sig: ScriptBuf::from_bytes(coinbase_script).expect("phase 127 coinbase script"),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(5_000_000_000).expect("phase 127 coinbase amount"),
            script_pubkey: phase127_p2sh_script(),
        }],
        lock_time: 0,
    };
    let (merkle_root, maybe_mutated) =
        block_merkle_root(core::slice::from_ref(&transaction)).expect("phase 127 merkle root");
    assert!(!maybe_mutated);
    let mut block = Block {
        header: BlockHeader {
            version: 1,
            previous_block_hash,
            merkle_root,
            time: 1_231_006_500 + height,
            bits: PHASE127_EASY_BITS,
            nonce: 0,
        },
        transactions: vec![transaction],
    };
    block.header.nonce = (0..=u32::MAX)
        .find(|nonce| {
            block.header.nonce = *nonce;
            check_block_header(&block.header).is_ok()
        })
        .expect("phase 127 easy target should be mineable");
    block
}

pub(super) fn phase127_mined_block() -> Block {
    phase127_mined_block_after(BlockHash::default(), 0)
}

pub(super) fn phase127_spend_transaction(previous_txid: Txid) -> Transaction {
    Transaction {
        version: 2,
        inputs: vec![TransactionInput {
            previous_output: OutPoint {
                txid: previous_txid,
                vout: 0,
            },
            script_sig: ScriptBuf::from_bytes(vec![0x01, 0x51]).expect("phase 127 redeem script"),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(4_999_999_000).expect("phase 127 spend amount"),
            script_pubkey: phase127_p2sh_script(),
        }],
        lock_time: 0,
    }
}

pub(super) fn phase127_transport(block: &Block) -> Phase127ScriptedTransport {
    Phase127ScriptedTransport {
        inbound: VecDeque::from([
            WireNetworkMessage::Version(VersionMessage {
                start_height: 0,
                ..VersionMessage::default()
            }),
            WireNetworkMessage::Verack,
            WireNetworkMessage::Headers(HeadersMessage {
                headers: vec![block.header.clone()],
            }),
            WireNetworkMessage::Block(block.clone()),
        ]),
    }
}

pub(super) fn phase127_block_request(block: &Block) -> WireNetworkMessage {
    WireNetworkMessage::GetData(InventoryList::new(vec![InventoryVector {
        inventory_type: InventoryType::Block,
        object_hash: block_hash(&block.header).into(),
    }]))
}

pub(super) fn phase127_mixed_missing_transaction_block_request(
    block: &Block,
) -> WireNetworkMessage {
    WireNetworkMessage::GetData(InventoryList::new(vec![
        InventoryVector {
            inventory_type: InventoryType::Transaction,
            object_hash: Txid::from_byte_array([127_u8; 32]).into(),
        },
        InventoryVector {
            inventory_type: InventoryType::Block,
            object_hash: block_hash(&block.header).into(),
        },
    ]))
}

pub(super) fn phase127_mixed_available_transaction_block_request(
    block: &Block,
    transaction: &Transaction,
) -> WireNetworkMessage {
    WireNetworkMessage::GetData(InventoryList::new(vec![
        InventoryVector {
            inventory_type: InventoryType::Transaction,
            object_hash: transaction_txid(transaction)
                .expect("phase 127 available transaction id")
                .into(),
        },
        InventoryVector {
            inventory_type: InventoryType::Block,
            object_hash: block_hash(&block.header).into(),
        },
    ]))
}

pub(super) fn phase127_mixed_block_available_transaction_request(
    block: &Block,
    transaction: &Transaction,
) -> WireNetworkMessage {
    WireNetworkMessage::GetData(InventoryList::new(vec![
        InventoryVector {
            inventory_type: InventoryType::Block,
            object_hash: block_hash(&block.header).into(),
        },
        InventoryVector {
            inventory_type: InventoryType::Transaction,
            object_hash: transaction_txid(transaction)
                .expect("phase 127 available transaction id")
                .into(),
        },
    ]))
}

pub(super) fn phase127_two_block_request(block: &Block) -> WireNetworkMessage {
    let block_inventory = InventoryVector {
        inventory_type: InventoryType::Block,
        object_hash: block_hash(&block.header).into(),
    };
    WireNetworkMessage::GetData(InventoryList::new(vec![
        block_inventory.clone(),
        block_inventory,
    ]))
}

pub(super) fn phase127_mixed_cycle_request(
    block: &Block,
    available_transaction: &Transaction,
) -> WireNetworkMessage {
    WireNetworkMessage::GetData(InventoryList::new(vec![
        InventoryVector {
            inventory_type: InventoryType::Transaction,
            object_hash: Txid::from_byte_array([126_u8; 32]).into(),
        },
        InventoryVector {
            inventory_type: InventoryType::Block,
            object_hash: block_hash(&block.header).into(),
        },
        InventoryVector {
            inventory_type: InventoryType::Transaction,
            object_hash: transaction_txid(available_transaction)
                .expect("phase 127 available transaction id")
                .into(),
        },
    ]))
}

pub(super) fn phase127_unknown_available_transaction_request(
    available_transaction: &Transaction,
) -> WireNetworkMessage {
    WireNetworkMessage::GetData(InventoryList::new(vec![
        InventoryVector {
            inventory_type: InventoryType::Unknown(127),
            object_hash: Txid::from_byte_array([125_u8; 32]).into(),
        },
        InventoryVector {
            inventory_type: InventoryType::Transaction,
            object_hash: transaction_txid(available_transaction)
                .expect("phase 127 available transaction id")
                .into(),
        },
    ]))
}

pub(super) fn phase127_missing_unknown_available_transaction_request(
    available_transaction: &Transaction,
) -> WireNetworkMessage {
    WireNetworkMessage::GetData(InventoryList::new(vec![
        InventoryVector {
            inventory_type: InventoryType::Transaction,
            object_hash: Txid::from_byte_array([124_u8; 32]).into(),
        },
        InventoryVector {
            inventory_type: InventoryType::Unknown(127),
            object_hash: Txid::from_byte_array([123_u8; 32]).into(),
        },
        InventoryVector {
            inventory_type: InventoryType::Transaction,
            object_hash: transaction_txid(available_transaction)
                .expect("phase 127 available transaction id")
                .into(),
        },
    ]))
}

pub(super) fn sorted_result_keys(response: &serde_json::Value) -> Vec<String> {
    let mut keys = response["result"]
        .as_object()
        .expect("phase 127 RPC result should be an object")
        .keys()
        .cloned()
        .collect::<Vec<_>>();
    keys.sort();
    keys
}

pub(super) fn encoded_hash(block_hash: BlockHash) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(64);
    for byte in block_hash.as_bytes() {
        encoded.push(HEX[(byte >> 4) as usize] as char);
        encoded.push(HEX[(byte & 0x0f) as usize] as char);
    }
    encoded
}
