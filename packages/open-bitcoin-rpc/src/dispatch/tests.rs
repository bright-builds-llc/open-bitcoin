// Parity breadcrumbs:
// - packages/bitcoin-knots/src/bitcoind.cpp
// - packages/bitcoin-knots/src/rpc/protocol.h
// - packages/bitcoin-knots/src/rpc/request.cpp
// - packages/bitcoin-knots/src/rpc/server.cpp
// - packages/bitcoin-knots/src/rpc/blockchain.cpp
// - packages/bitcoin-knots/src/rpc/mempool.cpp
// - packages/bitcoin-knots/src/rpc/net.cpp
// - packages/bitcoin-knots/src/rpc/rawtransaction.cpp
// - packages/bitcoin-knots/test/functional/interface_rpc.py

use std::{
    collections::HashMap,
    fs, io,
    path::{Path, PathBuf},
    thread,
    time::{SystemTime, UNIX_EPOCH},
};

use serde_json::json;

use open_bitcoin_network::{
    AddressAnnouncement, AddressList, InboundAdmissionSlotClass, InboundListenerConfig,
    ParsedPeerPermissionClass, PeerConnectionClass, PermissionEffectLabel, RelayActivationConfig,
    RelayPermissionEffectLabel, VersionMessage,
};
use open_bitcoin_node::{
    DurableSyncState, FjallNodeStore, ManagedPeerNetwork, ManagedWallet, MemoryChainstateStore,
    MemoryWalletStore, PersistMode, RuntimeMetadata, WalletRegistry,
    core::{
        chainstate::{ChainPosition, ChainstateSnapshot, Coin},
        codec::{TransactionEncoding, encode_transaction, parse_transaction},
        consensus::{
            ConsensusParams, ScriptVerifyFlags, block_hash, block_merkle_root, check_block_header,
            crypto::hash160, transaction_txid,
        },
        mempool::PolicyConfig,
        network::{LocalPeerConfig, ServiceFlags, WireNetworkMessage},
        primitives::{
            Amount, Block, BlockHash, BlockHeader, NetworkAddress, NetworkMagic, OutPoint,
            ScriptBuf, ScriptWitness, Transaction, TransactionInput, TransactionOutput, Txid,
        },
        wallet::{AddressNetwork, DescriptorRole, SingleKeyDescriptor, Wallet},
    },
    status::{
        BestKnownTipSource, BestKnownTipStatus, ChainTipStatus, FieldAvailability,
        INBOUND_STATUS_UNAVAILABLE_REASON, InboundPeerServingStatus, NoProgressDiagnosis,
        PeerCounts, PeerStatus, StayCurrentStatus, SyncAttemptCounters, SyncConfiguredTargets,
        SyncLagStatus, SyncLifecycleState, SyncProgress, SyncProgressSignal,
        SyncReconcileProgressStatus, SyncRecoveryCategory, SyncReorgEvidence, SyncResourcePressure,
        SyncStatus, SyncStopReasonStatus, TipFreshnessStatus,
    },
};

use crate::{
    DaemonSyncControl, DaemonSyncControlAction, DaemonSyncControlReceiver, ManagedRpcContext,
    RpcErrorCode, RpcFailureKind,
    config::{RuntimeConfig, WalletRuntimeConfig},
    dispatch::dispatch,
    inbound_listener::InboundListenerEvidence,
    method::{
        BuildAndSignTransactionRequest, DeriveAddressesRequest, GetBalancesRequest,
        GetBlockchainInfoRequest, GetMempoolInfoRequest, GetNetworkInfoRequest,
        GetWalletInfoRequest, ImportDescriptorsRequest, ListUnspentRequest, MethodCall,
        OpenBitcoinNetworkStatusRequest, OpenBitcoinSyncPauseRequest, OpenBitcoinSyncResumeRequest,
        OpenBitcoinSyncStatusRequest, RescanBlockchainRequest, SendRawTransactionRequest,
        SendToAddressRequest, TransactionRecipient,
    },
};

const EASY_BITS: u32 = 0x207f_ffff;
const RANGED_TPRV: &str = "tprv8ZgxMBicQKsPd7Uf69XL1XwhmjHopUGep8GuEiJDZmbQz6o58LninorQAfcKZWARbtRtfnLcJ5MQ2AtHcQJCCRUcMRvmDUjyEmNUWwx8UbK";
const RANGED_TPUB: &str = "tpubD6NzVbkrYhZ4WaWSyoBvQwbpLkojyoTZPRsgXELWz3Popb3qkjcJyJUGLnL4qHHoQvao8ESaAstxYSnhyswJ76uZPStJRJCTKvosUCJZL5B";

fn script(bytes: &[u8]) -> ScriptBuf {
    ScriptBuf::from_bytes(bytes.to_vec()).expect("script")
}

fn redeem_script() -> ScriptBuf {
    script(&[0x51])
}

fn p2sh_script() -> ScriptBuf {
    let redeem_hash = hash160(redeem_script().as_bytes());
    let mut bytes = vec![0xa9, 20];
    bytes.extend_from_slice(&redeem_hash);
    bytes.push(0x87);
    script(&bytes)
}

fn sample_tip(height: u32) -> ChainPosition {
    ChainPosition::new(
        BlockHeader {
            version: 1,
            previous_block_hash: BlockHash::from_byte_array([0_u8; 32]),
            merkle_root: Default::default(),
            time: 1_700_000_000 + height,
            bits: EASY_BITS,
            nonce: 1,
        },
        height,
        1,
        i64::from(1_700_000_000 + height),
    )
}

fn wallet_with_descriptors() -> Wallet {
    let mut wallet = Wallet::new(AddressNetwork::Regtest);
    wallet
        .import_descriptor(
            "receive",
            DescriptorRole::External,
            "wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi)",
        )
        .expect("receive descriptor");
    wallet
        .import_descriptor(
            "change",
            DescriptorRole::Internal,
            "sh(wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi))",
        )
        .expect("change descriptor");
    wallet
}

fn funded_snapshot(wallet: &Wallet) -> ChainstateSnapshot {
    let receive_script = wallet
        .default_receive_address()
        .expect("receive address")
        .script_pubkey;
    let mut utxos = HashMap::new();
    utxos.insert(
        OutPoint {
            txid: Txid::from_byte_array([7_u8; 32]),
            vout: 0,
        },
        Coin {
            output: TransactionOutput {
                value: Amount::from_sats(75_000).expect("amount"),
                script_pubkey: receive_script,
            },
            is_coinbase: false,
            created_height: 9,
            created_median_time_past: 1_700_000_009,
        },
    );

    ChainstateSnapshot::new(vec![sample_tip(10)], utxos, Default::default())
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

fn coinbase_transaction(height: u32, value: i64, script_pubkey: ScriptBuf) -> Transaction {
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
            value: Amount::from_sats(value).expect("amount"),
            script_pubkey,
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
        .expect("nonce");
}

fn build_block(
    previous_block_hash: BlockHash,
    height: u32,
    value: i64,
    script_pubkey: ScriptBuf,
) -> Block {
    let transactions = vec![coinbase_transaction(height, value, script_pubkey)];
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

fn spend_transaction(previous_txid: Txid, value: i64) -> Transaction {
    Transaction {
        version: 2,
        inputs: vec![TransactionInput {
            previous_output: OutPoint {
                txid: previous_txid,
                vout: 0,
            },
            script_sig: script(&[0x01, 0x51]),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(value).expect("amount"),
            script_pubkey: p2sh_script(),
        }],
        lock_time: 0,
    }
}

fn encode_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(HEX[(byte >> 4) as usize] as char);
        output.push(HEX[(byte & 0x0f) as usize] as char);
    }
    output
}

fn decode_hex(text: &str) -> Vec<u8> {
    let trimmed = text.trim();
    trimmed
        .as_bytes()
        .chunks_exact(2)
        .map(|pair| {
            let high = char::from(pair[0]).to_digit(16).expect("hex") as u8;
            let low = char::from(pair[1]).to_digit(16).expect("hex") as u8;
            (high << 4) | low
        })
        .collect()
}

fn empty_context() -> ManagedRpcContext {
    ManagedRpcContext::from_runtime_config(&RuntimeConfig {
        chain: AddressNetwork::Regtest,
        wallet: WalletRuntimeConfig {
            coinbase_maturity: 1,
            ..WalletRuntimeConfig::default()
        },
        ..RuntimeConfig::default()
    })
}

fn relay_enabled_context(nonce: u64) -> ManagedRpcContext {
    let local_config = LocalPeerConfig {
        magic: NetworkMagic::MAINNET,
        services: ServiceFlags::NETWORK | ServiceFlags::WITNESS,
        address: NetworkAddress {
            services: 0,
            address_bytes: [0_u8; 16],
            port: 18_444,
        },
        nonce,
        relay: true,
        user_agent: "/open-bitcoin:rpc-test/".to_string(),
    };
    let network = ManagedPeerNetwork::new_with_relay_activation(
        MemoryChainstateStore::default(),
        local_config,
        PolicyConfig::default(),
        RelayActivationConfig { enabled: true },
        true,
    );
    let wallet = ManagedWallet::from_store(
        MemoryWalletStore::default(),
        Wallet::new(AddressNetwork::Regtest),
    );
    ManagedRpcContext::new(
        AddressNetwork::Regtest,
        ConsensusParams {
            coinbase_maturity: 1,
            ..ConsensusParams::default()
        },
        rpc_verify_flags(),
        network,
        wallet,
    )
}

fn rpc_verify_flags() -> ScriptVerifyFlags {
    ScriptVerifyFlags::P2SH
        | ScriptVerifyFlags::STRICTENC
        | ScriptVerifyFlags::DERSIG
        | ScriptVerifyFlags::LOW_S
        | ScriptVerifyFlags::NULLDUMMY
        | ScriptVerifyFlags::SIGPUSHONLY
        | ScriptVerifyFlags::MINIMALDATA
        | ScriptVerifyFlags::CLEANSTACK
        | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
        | ScriptVerifyFlags::CHECKSEQUENCEVERIFY
        | ScriptVerifyFlags::WITNESS
        | ScriptVerifyFlags::MINIMALIF
        | ScriptVerifyFlags::NULLFAIL
        | ScriptVerifyFlags::WITNESS_PUBKEYTYPE
        | ScriptVerifyFlags::TAPROOT
}

fn inbound_context(max_peers: usize, reserved_slots: usize) -> ManagedRpcContext {
    ManagedRpcContext::from_runtime_config(&RuntimeConfig {
        chain: AddressNetwork::Regtest,
        inbound: InboundListenerConfig {
            enabled: true,
            listen_addresses: vec!["127.0.0.1:18444".to_string()],
            max_peers,
            reserved_slots,
            allow_public: false,
            permission_classes: Default::default(),
        },
        wallet: WalletRuntimeConfig {
            coinbase_maturity: 1,
            ..WalletRuntimeConfig::default()
        },
        ..RuntimeConfig::default()
    })
}

fn permission_context(classes: Vec<ParsedPeerPermissionClass>) -> ManagedRpcContext {
    permission_context_with_limits(classes, 8, 1)
}

fn permission_context_with_limits(
    classes: Vec<ParsedPeerPermissionClass>,
    max_peers: usize,
    reserved_slots: usize,
) -> ManagedRpcContext {
    ManagedRpcContext::from_runtime_config(&RuntimeConfig {
        chain: AddressNetwork::Regtest,
        inbound: InboundListenerConfig {
            enabled: true,
            listen_addresses: vec!["127.0.0.1:18444".to_string()],
            max_peers,
            reserved_slots,
            allow_public: false,
            permission_classes: open_bitcoin_network::PeerPermissionClassRegistry::new(classes),
        },
        wallet: WalletRuntimeConfig {
            coinbase_maturity: 1,
            ..WalletRuntimeConfig::default()
        },
        ..RuntimeConfig::default()
    })
}

fn parsed_permission_class(
    name: &str,
    address: &str,
    permissions: &[&str],
) -> ParsedPeerPermissionClass {
    ParsedPeerPermissionClass::parse(name, [address], permissions.iter().copied())
        .expect("permission class should parse")
}

fn address_boundary_context() -> ManagedRpcContext {
    ManagedRpcContext::from_runtime_config(&RuntimeConfig {
        chain: AddressNetwork::Regtest,
        inbound: InboundListenerConfig {
            enabled: true,
            listen_addresses: vec!["8.8.8.8:18444".to_string(), "127.0.0.1:18445".to_string()],
            max_peers: 8,
            reserved_slots: 1,
            allow_public: true,
            permission_classes: open_bitcoin_network::PeerPermissionClassRegistry::new(vec![
                parsed_permission_class(
                    "operator-private-addr-secret",
                    "127.0.0.1",
                    &["in", "addr"],
                ),
            ]),
        },
        wallet: WalletRuntimeConfig {
            coinbase_maturity: 1,
            ..WalletRuntimeConfig::default()
        },
        ..RuntimeConfig::default()
    })
}

fn address_announcement(time_unix_seconds: u64, address: NetworkAddress) -> AddressAnnouncement {
    AddressAnnouncement {
        time_unix_seconds: time_unix_seconds as u32,
        address,
    }
}

fn public_ipv4_network_address(a: u8, b: u8, c: u8, d: u8, port: u16) -> NetworkAddress {
    NetworkAddress {
        services: ServiceFlags::NETWORK.bits(),
        address_bytes: ipv4_mapped_address_bytes([a, b, c, d]),
        port,
    }
}

fn ipv4_mapped_address_bytes(octets: [u8; 4]) -> [u8; 16] {
    let mut bytes = [0_u8; 16];
    bytes[10] = 0xff;
    bytes[11] = 0xff;
    bytes[12..].copy_from_slice(&octets);
    bytes
}

fn funded_wallet_context() -> ManagedRpcContext {
    let mut context = empty_context();
    context
        .import_descriptor(
            "receive",
            DescriptorRole::External,
            "wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi)",
        )
        .expect("receive");
    context
        .import_descriptor(
            "change",
            DescriptorRole::Internal,
            "sh(wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi))",
        )
        .expect("change");
    let snapshot = funded_snapshot(&wallet_with_descriptors());
    context.rescan_wallet(&snapshot).expect("rescan");
    context
}

fn spendable_send_context() -> ManagedRpcContext {
    let mut context = empty_context();
    context
        .import_descriptor(
            "receive",
            DescriptorRole::External,
            "wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi)",
        )
        .expect("receive");
    context
        .import_descriptor(
            "change",
            DescriptorRole::Internal,
            "sh(wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi))",
        )
        .expect("change");
    let receive_script = context
        .descriptor_address(0)
        .expect("receive address")
        .script_pubkey;
    let genesis = build_block(
        BlockHash::from_byte_array([0_u8; 32]),
        0,
        500_000_000,
        p2sh_script(),
    );
    let funding = build_block(block_hash(&genesis.header), 1, 75_000, receive_script);
    context.connect_local_block(&genesis).expect("genesis");
    context.connect_local_block(&funding).expect("funding");
    let snapshot = context
        .blockchain_snapshot()
        .expect("authoritative chainstate snapshot");
    context.rescan_wallet(&snapshot).expect("rescan");
    context
}

fn temp_store_path(test_name: &str) -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time after unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "open-bitcoin-rpc-dispatch-{test_name}-{}-{timestamp}",
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

fn durable_wallet_context(test_name: &str, wallet_name: &str) -> ManagedRpcContext {
    let path = temp_store_path(test_name);
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut registry = WalletRegistry::default();
    let mut wallet = Wallet::new(AddressNetwork::Regtest);
    wallet
        .import_descriptor(
            "receive-ranged",
            DescriptorRole::External,
            &format!("wpkh({RANGED_TPRV}/1/1/*)"),
        )
        .expect("receive descriptor");
    wallet
        .import_descriptor(
            "change-ranged",
            DescriptorRole::Internal,
            &format!("sh(wpkh({RANGED_TPUB}/1/*))"),
        )
        .expect("change descriptor");
    registry
        .create_wallet(&store, wallet_name.to_string(), wallet, PersistMode::Sync)
        .expect("create wallet");
    drop(store);

    let mut context = ManagedRpcContext::from_runtime_config(&RuntimeConfig {
        chain: AddressNetwork::Regtest,
        maybe_data_dir: Some(path),
        wallet: WalletRuntimeConfig {
            coinbase_maturity: 1,
            ..WalletRuntimeConfig::default()
        },
        ..RuntimeConfig::default()
    });
    context.set_request_wallet_name(Some(wallet_name.to_string()));
    context
}

fn node_context_with_chain_and_mempool() -> ManagedRpcContext {
    let mut context = empty_context();
    let genesis = build_block(
        BlockHash::from_byte_array([0_u8; 32]),
        0,
        500_000_000,
        p2sh_script(),
    );
    let spendable = build_block(block_hash(&genesis.header), 1, 500_000_000, p2sh_script());
    context.connect_local_block(&genesis).expect("genesis");
    context.connect_local_block(&spendable).expect("spendable");
    context.add_inbound_peer(7).expect("peer");
    context
        .receive_network_message(7, WireNetworkMessage::WtxidRelay, 1)
        .expect("wtxidrelay");
    context
        .receive_network_message(7, WireNetworkMessage::SendHeaders, 1)
        .expect("sendheaders");
    context.connect_outbound_peer(8, 2).expect("outbound");
    let transaction = spend_transaction(
        transaction_txid(&genesis.transactions[0]).expect("txid"),
        499_999_000,
    );
    context
        .submit_local_transaction(transaction)
        .expect("submit");
    context
}

#[test]
fn permission_context_defaults_to_ordinary_decision_without_configured_classes() {
    // Arrange
    let context = ManagedRpcContext::for_local_operator(AddressNetwork::Regtest);
    let remote_addr = "127.0.0.1:50000".parse().expect("remote address");

    // Act
    let decision = context.permission_decision_for_remote_addr(remote_addr);

    // Assert
    assert_eq!(
        decision.connection_class(),
        PeerConnectionClass::OrdinaryInbound
    );
    assert!(decision.active_effects().is_empty());
    assert!(decision.inactive_effects().is_empty());
}

#[test]
fn permission_context_resolves_permissioned_literal_ip_from_runtime_config() {
    // Arrange
    let context = permission_context(vec![parsed_permission_class(
        "loopback-download",
        "127.0.0.1",
        &["in", "download", "addr", "relay"],
    )]);
    let remote_addr = "127.0.0.1:50000".parse().expect("remote address");

    // Act
    let decision = context.permission_decision_for_remote_addr(remote_addr);

    // Assert
    assert_eq!(
        decision.connection_class(),
        PeerConnectionClass::PermissionedInbound
    );
    assert_eq!(decision.slot_class(), InboundAdmissionSlotClass::Ordinary);
    assert!(
        decision
            .active_effects()
            .contains(&PermissionEffectLabel::DownloadServingPolicyInput)
    );
    assert!(
        decision
            .active_effects()
            .contains(&PermissionEffectLabel::AddressResponsePolicyInput)
    );
    assert!(
        decision
            .relay_permission_effects()
            .contains(&RelayPermissionEffectLabel::TransactionRelayPolicyInput)
    );
    assert!(decision.inactive_effects().is_empty());
}

#[test]
fn permission_context_resolves_protected_literal_ip_without_raw_class_leak() {
    // Arrange
    let context = permission_context(vec![parsed_permission_class(
        "operator-loopback-secret-name",
        "127.0.0.1",
        &["in", "noban", "forceinbound"],
    )]);
    let matched_addr = "127.0.0.1:50000".parse().expect("matched address");
    let unmatched_addr = "127.0.0.2:50000".parse().expect("unmatched address");

    // Act
    let matched = context.permission_decision_for_remote_addr(matched_addr);
    let unmatched = context.permission_decision_for_remote_addr(unmatched_addr);
    let debug = format!("{context:?}");

    // Assert
    assert_eq!(
        matched.connection_class(),
        PeerConnectionClass::ProtectedInbound
    );
    assert_eq!(matched.slot_class(), InboundAdmissionSlotClass::Reserved);
    assert!(
        matched
            .active_effects()
            .contains(&PermissionEffectLabel::AdmissionProtected)
    );
    assert!(
        matched
            .active_effects()
            .contains(&PermissionEffectLabel::EvictionPolicyProtected)
    );
    assert_eq!(
        unmatched.connection_class(),
        PeerConnectionClass::OrdinaryInbound
    );
    assert!(!debug.contains("operator-loopback-secret-name"));
}

#[test]
fn open_bitcoin_network_status_returns_available_inbound_evidence() {
    // Arrange
    let mut context = inbound_context(4, 0);
    context
        .record_inbound_admission(7, "127.0.0.1:18444".to_string(), false)
        .expect("authoritative inbound admission");
    context
        .record_inbound_admission(8, "127.0.0.1:18444".to_string(), false)
        .expect("authoritative inbound admission");
    context
        .record_inbound_admission(7, "127.0.0.1:18445".to_string(), false)
        .expect("authoritative inbound admission");

    // Act
    let status = dispatch(
        &mut context,
        MethodCall::OpenBitcoinNetworkStatus(OpenBitcoinNetworkStatusRequest::default()),
    )
    .expect("network status");

    // Assert
    let inbound = &status["inbound"];
    assert_eq!(inbound["state"], json!("available"));
    assert_eq!(inbound["value"]["listener_state"], json!("listening"));
    assert_eq!(inbound["value"]["preflight_reason"], json!("ready"));
    assert_eq!(inbound["value"]["admitted_inbound_peers"], json!(1));
    assert_eq!(inbound["value"]["rejected_inbound_peers"], json!(2));
    assert_eq!(inbound["value"]["handshake"]["established"], json!(1));
    assert_eq!(inbound["value"]["duplicate_rejects"], json!(2));
    assert_eq!(inbound["value"]["self_connection_rejects"], json!(0));
    assert_eq!(inbound["value"]["cap_rejects"], json!(0));
    assert_eq!(inbound["value"]["reserved_slot_rejects"], json!(0));
    assert_eq!(
        inbound["value"]["latest_admission_event"]["value"]["reason"],
        json!("duplicate_peer_id")
    );
    assert_eq!(inbound["value"]["permissioned_inbound_peers"], json!(0));
    assert_eq!(inbound["value"]["protected_inbound_peers"], json!(0));
    assert_eq!(
        inbound["value"]["permission_class"],
        json!("ordinary_inbound")
    );
    assert_eq!(inbound["value"]["active_permission_effects"], json!([]));
    assert_eq!(inbound["value"]["inactive_permission_effects"], json!([]));
    assert_eq!(
        inbound["value"]["latest_permission_decision"]["state"],
        json!("unavailable")
    );
    assert_eq!(inbound["value"]["eviction_candidates_evaluated"], json!(1));
    assert_eq!(inbound["value"]["disconnects_requested"], json!(1));
    assert_eq!(
        inbound["value"]["latest_peer_policy_decision"]["state"],
        json!("available")
    );
    assert_eq!(
        inbound["value"]["latest_peer_policy_decision"]["value"]["label"],
        json!("eviction_candidate_selected")
    );
    assert_eq!(
        status["relay"]["outcome_counters"]["state"],
        json!("implemented")
    );
    assert_eq!(
        status["relay"]["outcome_counters"]["value"]["accepted_count"],
        json!(0)
    );
    assert_eq!(
        status["relay"]["mempool_admission"]["state"],
        json!("unavailable")
    );
    assert_eq!(
        status["relay"]["public_relay"]["state"],
        json!("intentionally_different")
    );
    assert_eq!(
        status["block_relay"]["block_serving"]["activation"]["state"],
        json!("unavailable")
    );
    assert_eq!(
        status["block_relay"]["negotiation"]["value"]["version2_high_bandwidth_count"],
        json!(0)
    );
}

#[test]
fn authoritative_operator_snapshot_preserves_network_status_schema_and_provenance() {
    // Arrange
    let mut context = inbound_context(4, 0);
    context
        .record_inbound_admission(7, "127.0.0.1:18444".to_string(), false)
        .expect("authoritative inbound admission");

    // Act
    let snapshot = context
        .authoritative_operator_snapshot()
        .expect("owned authoritative operator snapshot");
    let status = dispatch(
        &mut context,
        MethodCall::OpenBitcoinNetworkStatus(OpenBitcoinNetworkStatusRequest::default()),
    )
    .expect("network status");

    // Assert
    assert_eq!(
        status
            .as_object()
            .expect("status object")
            .keys()
            .collect::<Vec<_>>(),
        vec!["block_relay", "inbound", "metrics", "relay"]
    );
    assert_eq!(
        status["inbound"],
        serde_json::to_value(snapshot.inbound()).expect("inbound snapshot")
    );
    assert_eq!(
        status["relay"],
        serde_json::to_value(snapshot.relay()).expect("relay snapshot")
    );
    assert_eq!(
        status["block_relay"],
        serde_json::to_value(snapshot.block_relay()).expect("block-relay snapshot")
    );
}

#[test]
fn open_bitcoin_network_status_includes_block_relay_projection() {
    let mut context = empty_context();

    let status = dispatch(
        &mut context,
        MethodCall::OpenBitcoinNetworkStatus(OpenBitcoinNetworkStatusRequest::default()),
    )
    .expect("network status");

    assert_eq!(
        status["block_relay"]["block_serving"]["activation"]["state"],
        json!("unavailable")
    );
    assert_eq!(
        status["block_relay"]["cleanup"]["value"]["compact_cleanup_count"],
        json!(0)
    );
}

#[test]
fn open_bitcoin_network_status_projects_listener_activation_before_admissions() {
    // Arrange
    let mut context = empty_context();
    context
        .set_inbound_listener_evidence(InboundListenerEvidence {
            listener_state: "listening".to_string(),
            preflight_reason: "ready".to_string(),
            bound_endpoints: vec!["127.0.0.1:18444".to_string()],
            admitted_inbound_peers: 0,
            rejected_inbound_peers: 0,
            resource_rejections: 0,
            timeout_disconnects: 0,
            churn_rejections: 0,
            reconnect_suppressions: 0,
            maybe_admission_reject_reason: None,
            maybe_latest_admission_event: Some("ready".to_string()),
            maybe_latest_resource_event: None,
        })
        .expect("authoritative listener evidence");

    // Act
    let status = dispatch(
        &mut context,
        MethodCall::OpenBitcoinNetworkStatus(OpenBitcoinNetworkStatusRequest::default()),
    )
    .expect("network status");

    // Assert
    let inbound = &status["inbound"];
    assert_eq!(inbound["state"], json!("available"));
    assert_eq!(inbound["value"]["listener_state"], json!("listening"));
    assert_eq!(inbound["value"]["preflight_reason"], json!("ready"));
    assert_eq!(
        inbound["value"]["bound_endpoints"],
        json!(["127.0.0.1:18444"])
    );
    assert_eq!(inbound["value"]["admitted_inbound_peers"], json!(0));
    assert_eq!(inbound["value"]["rejected_inbound_peers"], json!(0));
    assert_eq!(
        inbound["value"]["latest_admission_event"]["value"]["reason"],
        json!("ready")
    );
}

#[test]
fn open_bitcoin_network_status_projects_address_boundary_evidence_without_raw_details() {
    // Arrange
    let mut context = address_boundary_context();
    let peer_id = 9_206_101;
    let now_unix_seconds = 1_700_000_000;
    context
        .set_inbound_listener_evidence(InboundListenerEvidence {
            listener_state: "listening".to_string(),
            preflight_reason: "ready".to_string(),
            bound_endpoints: vec!["8.8.8.8:18444".to_string(), "127.0.0.1:18445".to_string()],
            admitted_inbound_peers: 0,
            rejected_inbound_peers: 0,
            resource_rejections: 0,
            timeout_disconnects: 0,
            churn_rejections: 0,
            reconnect_suppressions: 0,
            maybe_admission_reject_reason: None,
            maybe_latest_admission_event: Some("ready".to_string()),
            maybe_latest_resource_event: None,
        })
        .expect("authoritative listener evidence");
    context
        .record_inbound_admission_for_remote_addr(
            peer_id,
            "127.0.0.1:52061".parse().expect("permissioned remote"),
            false,
        )
        .expect("authoritative inbound admission");
    context
        .receive_network_message(
            peer_id,
            WireNetworkMessage::Addr(AddressList {
                addresses: vec![
                    address_announcement(
                        now_unix_seconds,
                        public_ipv4_network_address(9, 9, 9, 9, 8333),
                    ),
                    address_announcement(
                        now_unix_seconds,
                        public_ipv4_network_address(10, 0, 0, 1, 8333),
                    ),
                ],
            }),
            now_unix_seconds as i64,
        )
        .expect("addr evidence should be recorded");
    let first_getaddr_response = context
        .receive_network_message(
            peer_id,
            WireNetworkMessage::GetAddr,
            now_unix_seconds as i64 + 1,
        )
        .expect("first getaddr should be served");
    let second_getaddr_response = context
        .receive_network_message(
            peer_id,
            WireNetworkMessage::GetAddr,
            now_unix_seconds as i64 + 2,
        )
        .expect("second getaddr should be suppressed");

    // Act
    let status = dispatch(
        &mut context,
        MethodCall::OpenBitcoinNetworkStatus(OpenBitcoinNetworkStatusRequest::default()),
    )
    .expect("network status");

    // Assert
    assert!(matches!(
        first_getaddr_response.as_slice(),
        [WireNetworkMessage::Addr(addresses)] if !addresses.addresses.is_empty()
    ));
    assert!(second_getaddr_response.is_empty());
    let inbound = &status["inbound"]["value"];
    assert_eq!(
        inbound["local_advertisement_candidates"],
        json!([{
            "source": "source_local_listener",
            "network_kind": "ipv4",
            "routability": "publicly_routable",
            "freshness": "fresh",
            "services_bits": 9,
            "port": 18444,
            "persistence_eligible": false
        }])
    );
    assert_eq!(
        inbound["suppressed_advertisements"][0]["label"],
        json!("advertise_suppressed")
    );
    assert_eq!(
        inbound["suppressed_advertisements"][0]["reason"],
        json!("not_publicly_routable")
    );
    assert_eq!(inbound["getaddr_responses_served"], json!(1));
    assert_eq!(inbound["getaddr_requests_suppressed"], json!(1));
    assert_eq!(inbound["learned_address_entries"], json!(1));
    assert_eq!(inbound["learned_address_rejections"], json!(1));
    assert_eq!(
        inbound["latest_address_decision"]["value"]["label"],
        json!("getaddr_suppressed")
    );
    assert_eq!(
        inbound["latest_address_decision"]["value"]["reason"],
        json!("already_served")
    );
    let address_evidence = json!({
        "local_advertisement_candidates": inbound["local_advertisement_candidates"],
        "suppressed_advertisements": inbound["suppressed_advertisements"],
        "getaddr_responses_served": inbound["getaddr_responses_served"],
        "getaddr_requests_suppressed": inbound["getaddr_requests_suppressed"],
        "learned_address_entries": inbound["learned_address_entries"],
        "learned_address_rejections": inbound["learned_address_rejections"],
        "latest_address_decision": inbound["latest_address_decision"],
    });
    let serialized_address_evidence =
        serde_json::to_string(&address_evidence).expect("serialize address evidence");
    for forbidden in [
        "operator-private-addr-secret",
        "8.8.8.8:18444",
        "127.0.0.1:18445",
        "127.0.0.1",
        "8.8.8.8",
        "9.9.9.9",
        "10.0.0.1",
        "9206101",
        "address_bytes",
        "raw_permission",
        "raw_config",
        "class_name",
        "00000000000000000000ffff08080808",
    ] {
        assert!(
            !serialized_address_evidence.contains(forbidden),
            "address evidence exposed raw detail {forbidden}"
        );
    }
}

#[test]
fn open_bitcoin_network_status_preserves_unavailable_reason() {
    // Arrange
    let mut context = empty_context();

    // Act
    let status = dispatch(
        &mut context,
        MethodCall::OpenBitcoinNetworkStatus(OpenBitcoinNetworkStatusRequest::default()),
    )
    .expect("network status");

    // Assert
    assert_eq!(status["inbound"]["state"], json!("unavailable"));
    assert_eq!(
        status["inbound"]["value"]["reason"],
        json!(INBOUND_STATUS_UNAVAILABLE_REASON)
    );
}

#[test]
fn open_bitcoin_network_status_reports_permission_evidence_without_raw_class_names() {
    // Arrange
    let mut context = permission_context(vec![
        parsed_permission_class(
            "operator-loopback-relay-like",
            "127.0.0.1",
            &[
                "in",
                "download",
                "addr",
                "relay",
                "forcerelay",
                "mempool",
                "bloomfilter",
                "blockfilters",
            ],
        ),
        parsed_permission_class(
            "operator-loopback-protected",
            "127.0.0.2",
            &["in", "noban", "forceinbound"],
        ),
    ]);
    context
        .record_inbound_admission_for_remote_addr(
            31,
            "127.0.0.1:50031".parse().expect("permissioned remote"),
            false,
        )
        .expect("authoritative inbound admission");
    context
        .record_inbound_admission_for_remote_addr(
            32,
            "127.0.0.2:50032".parse().expect("protected remote"),
            false,
        )
        .expect("authoritative inbound admission");

    // Act
    let status = dispatch(
        &mut context,
        MethodCall::OpenBitcoinNetworkStatus(OpenBitcoinNetworkStatusRequest::default()),
    )
    .expect("network status");
    let serialized = serde_json::to_string(&status).expect("status json");

    // Assert
    let inbound = &status["inbound"]["value"];
    assert_eq!(inbound["permissioned_inbound_peers"], json!(1));
    assert_eq!(inbound["protected_inbound_peers"], json!(1));
    assert_eq!(inbound["permission_class"], json!("protected_inbound"));
    assert_eq!(
        inbound["active_permission_effects"],
        json!([
            "admission_protected",
            "eviction_policy_protected",
            "misbehavior_policy_protected",
            "address_response_policy_input",
            "download_serving_policy_input"
        ])
    );
    assert_eq!(
        inbound["inactive_permission_effects"],
        json!(["inactive_bloomfilter", "inactive_blockfilters"])
    );
    assert_eq!(
        inbound["latest_permission_decision"]["value"]["permission_class"],
        json!("protected_inbound")
    );
    assert_eq!(
        inbound["latest_admission_event"]["value"]["slot_class"],
        json!("reserved")
    );
    assert_eq!(
        inbound["latest_permission_decision"]["value"]["active_permission_effects"],
        json!([
            "admission_protected",
            "eviction_policy_protected",
            "misbehavior_policy_protected",
            "download_serving_policy_input"
        ])
    );
    assert!(!serialized.contains("operator-loopback-relay-like"));
    assert!(!serialized.contains("operator-loopback-protected"));
}

#[test]
fn open_bitcoin_network_status_reports_cap_and_reserved_slot_rejections() {
    // Arrange
    let mut cap_context = inbound_context(1, 0);
    cap_context
        .record_inbound_admission(11, "127.0.0.1:18444".to_string(), false)
        .expect("authoritative inbound admission");
    cap_context
        .record_inbound_admission(12, "127.0.0.1:18445".to_string(), false)
        .expect("authoritative inbound admission");
    let mut reserved_context = inbound_context(2, 1);
    reserved_context
        .record_inbound_admission(21, "127.0.0.1:18444".to_string(), false)
        .expect("authoritative inbound admission");
    reserved_context
        .record_inbound_admission(22, "127.0.0.1:18445".to_string(), false)
        .expect("authoritative inbound admission");
    let mut protected_reserved_context = permission_context_with_limits(
        vec![parsed_permission_class(
            "operator-loopback-protected",
            "127.0.0.1",
            &["in", "noban", "forceinbound"],
        )],
        1,
        1,
    );
    protected_reserved_context
        .record_inbound_admission_for_remote_addr(
            31,
            "127.0.0.1:50031".parse().expect("first protected peer"),
            false,
        )
        .expect("authoritative inbound admission");
    protected_reserved_context
        .record_inbound_admission_for_remote_addr(
            32,
            "127.0.0.1:50032".parse().expect("second protected peer"),
            false,
        )
        .expect("authoritative inbound admission");

    // Act
    let cap_status = dispatch(
        &mut cap_context,
        MethodCall::OpenBitcoinNetworkStatus(OpenBitcoinNetworkStatusRequest::default()),
    )
    .expect("cap status");
    let reserved_status = dispatch(
        &mut reserved_context,
        MethodCall::OpenBitcoinNetworkStatus(OpenBitcoinNetworkStatusRequest::default()),
    )
    .expect("reserved status");
    let protected_reserved_status = dispatch(
        &mut protected_reserved_context,
        MethodCall::OpenBitcoinNetworkStatus(OpenBitcoinNetworkStatusRequest::default()),
    )
    .expect("protected reserved status");

    // Assert
    assert_eq!(cap_status["inbound"]["value"]["cap_rejects"], json!(1));
    assert_eq!(
        cap_status["inbound"]["value"]["latest_admission_event"]["value"]["reason"],
        json!("cap_reached")
    );
    assert_eq!(
        reserved_status["inbound"]["value"]["reserved_slot_rejects"],
        json!(1)
    );
    assert_eq!(
        reserved_status["inbound"]["value"]["latest_admission_event"]["value"]["reason"],
        json!("reserved_slot_unavailable")
    );
    assert_eq!(
        reserved_status["inbound"]["value"]["latest_admission_event"]["value"]["slot_class"],
        json!("ordinary")
    );
    assert_eq!(
        protected_reserved_status["inbound"]["value"]["latest_admission_event"]["value"]["reason"],
        json!("reserved_slot_unavailable")
    );
    assert_eq!(
        protected_reserved_status["inbound"]["value"]["latest_admission_event"]["value"]["slot_class"],
        json!("reserved")
    );
    assert_eq!(
        protected_reserved_status["inbound"]["value"]["latest_permission_decision"]["state"],
        json!("unavailable")
    );
}

#[test]
fn open_bitcoin_network_status_latest_event_updates_after_rejection_then_admission() {
    // Arrange
    let mut context = inbound_context(2, 0);
    context
        .record_inbound_admission(41, "127.0.0.1:18444".to_string(), false)
        .expect("authoritative inbound admission");
    context
        .record_inbound_admission(42, "127.0.0.1:18444".to_string(), false)
        .expect("authoritative inbound admission");
    context
        .record_inbound_admission(43, "127.0.0.1:18445".to_string(), false)
        .expect("authoritative inbound admission");

    // Act
    let status = dispatch(
        &mut context,
        MethodCall::OpenBitcoinNetworkStatus(OpenBitcoinNetworkStatusRequest::default()),
    )
    .expect("network status");

    // Assert
    let inbound = &status["inbound"]["value"];
    assert_eq!(inbound["admitted_inbound_peers"], json!(2));
    assert_eq!(inbound["rejected_inbound_peers"], json!(1));
    assert_eq!(
        inbound["latest_admission_event"]["value"]["outcome"],
        json!("admitted")
    );
    assert_eq!(
        inbound["latest_admission_event"]["value"]["reason"],
        json!("admitted")
    );
}

#[test]
fn open_bitcoin_network_status_records_runtime_self_connection_rejection() {
    // Arrange
    let mut context = inbound_context(2, 0);
    context
        .record_inbound_admission(51, "127.0.0.1:18451".to_string(), false)
        .expect("authoritative inbound admission");

    // Act
    let error = context
        .receive_network_message(
            51,
            WireNetworkMessage::Version(VersionMessage {
                nonce: 0,
                ..VersionMessage::default()
            }),
            1,
        )
        .expect_err("self-connection should disconnect admitted inbound peer");
    let status = dispatch(
        &mut context,
        MethodCall::OpenBitcoinNetworkStatus(OpenBitcoinNetworkStatusRequest::default()),
    )
    .expect("network status");

    // Assert
    assert_eq!(error.to_string(), "peer 51 connected to self");
    let inbound = &status["inbound"]["value"];
    assert_eq!(inbound["rejected_inbound_peers"], json!(1));
    assert_eq!(inbound["self_connection_rejects"], json!(1));
    assert_eq!(
        inbound["latest_admission_event"]["value"]["outcome"],
        json!("rejected")
    );
    assert_eq!(
        inbound["latest_admission_event"]["value"]["reason"],
        json!("self_connection")
    );
    assert_eq!(
        inbound["latest_admission_event"]["value"]["slot_class"],
        json!("ordinary")
    );
    assert_eq!(
        inbound["latest_permission_decision"]["state"],
        json!("unavailable")
    );
}

#[test]
fn open_bitcoin_network_status_get_network_info_omits_open_bitcoin_inbound_status_details() {
    // Arrange
    let mut context = node_context_with_chain_and_mempool();
    context
        .record_inbound_admission(17, "127.0.0.1:18447".to_string(), false)
        .expect("authoritative inbound admission");
    let regression_scope =
        "getnetworkinfo local_advertisement_candidates latest_address_decision regression";

    // Act
    let network = dispatch(
        &mut context,
        MethodCall::GetNetworkInfo(GetNetworkInfoRequest::default()),
    )
    .expect("network");
    let serialized = serde_json::to_string(&network).expect("serialize network info");

    // Assert
    assert_eq!(network["connections_in"], json!(2));
    for forbidden in [
        "listener_state",
        "preflight_reason",
        "admission",
        "duplicate_rejects",
        "self_connection_rejects",
        "reserved_slot_rejects",
        "cap_rejects",
        "permission_class",
        "permissioned_inbound_peers",
        "protected_inbound_peers",
        "active_permission_effects",
        "inactive_permission_effects",
        "latest_permission_decision",
        "local_advertisement_candidates",
        "suppressed_advertisements",
        "getaddr_responses_served",
        "getaddr_requests_suppressed",
        "learned_address_entries",
        "learned_address_rejections",
        "latest_address_decision",
        "eviction_candidates_evaluated",
        "disconnects_requested",
        "discouraged_peers",
        "active_bans",
        "expired_bans",
        "manual_unbans",
        "misbehavior_observations",
        "protected_no_actions",
        "latest_peer_policy_decision",
        "outcome_counters",
        "accepted_count",
        "rebroadcast_deferred_count",
        "public_relay",
        "mempool_admission",
    ] {
        assert!(
            !serialized.contains(forbidden),
            "{regression_scope}: baseline method exposed {forbidden}"
        );
    }
}

#[test]
fn node_info_methods_return_documented_phase_8_fields() {
    // Arrange
    let mut context = node_context_with_chain_and_mempool();

    // Act
    let network = dispatch(
        &mut context,
        MethodCall::GetNetworkInfo(GetNetworkInfoRequest::default()),
    )
    .expect("network");
    let blockchain = dispatch(
        &mut context,
        MethodCall::GetBlockchainInfo(GetBlockchainInfoRequest::default()),
    )
    .expect("blockchain");
    let mempool = dispatch(
        &mut context,
        MethodCall::GetMempoolInfo(GetMempoolInfoRequest::default()),
    )
    .expect("mempool");

    // Assert
    assert_eq!(network["connections"], json!(2));
    assert_eq!(network["connections_in"], json!(1));
    assert_eq!(network["connections_out"], json!(1));
    assert_eq!(network["localrelay"], json!(false));
    assert_eq!(blockchain["chain"], json!("regtest"));
    assert_eq!(blockchain["blocks"], json!(1));
    assert_eq!(blockchain["headers"], json!(1));
    assert_eq!(blockchain["mediantime"], json!(1231006501));
    assert_eq!(mempool["size"], json!(1));
    assert_eq!(mempool["total_fee_sats"], json!(1000));
    assert_eq!(mempool["loaded"], json!(true));
    assert!(!network.to_string().contains("outcome_counters"));
    assert!(!mempool.to_string().contains("outcome_counters"));
}

#[test]
fn open_bitcoin_sync_rpc_control_updates_daemon_runtime_metadata() {
    // Arrange
    let (control, receiver) = DaemonSyncControl::channel();
    let join_handle = spawn_test_sync_control_worker(receiver);
    let mut context = empty_context();
    context.set_daemon_sync_control(control);

    // Act
    let status = dispatch(
        &mut context,
        MethodCall::OpenBitcoinSyncStatus(OpenBitcoinSyncStatusRequest::default()),
    )
    .expect("sync status");
    let pause = dispatch(
        &mut context,
        MethodCall::OpenBitcoinSyncPause(OpenBitcoinSyncPauseRequest::default()),
    )
    .expect("sync pause");
    let resume = dispatch(
        &mut context,
        MethodCall::OpenBitcoinSyncResume(OpenBitcoinSyncResumeRequest::default()),
    )
    .expect("sync resume");
    drop(context);
    join_handle.join().expect("sync control worker");

    // Assert
    assert_eq!(status["metadata"]["sync_control"]["paused"], json!(false));
    assert_eq!(pause["metadata"]["sync_control"]["paused"], json!(true));
    assert_eq!(resume["metadata"]["sync_control"]["paused"], json!(false));
}

#[test]
fn open_bitcoin_sync_rpc_control_uses_daemon_store_backend() {
    // Arrange
    let path = temp_store_path("sync-control-store-backend");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    store
        .save_runtime_metadata(&RuntimeMetadata::default(), PersistMode::Sync)
        .expect("save metadata");
    let mut context = empty_context();
    context.set_daemon_sync_control(DaemonSyncControl::store_backed(
        store.clone(),
        PersistMode::Sync,
    ));

    // Act
    let status = dispatch(
        &mut context,
        MethodCall::OpenBitcoinSyncStatus(OpenBitcoinSyncStatusRequest::default()),
    )
    .expect("sync status");
    let pause = dispatch(
        &mut context,
        MethodCall::OpenBitcoinSyncPause(OpenBitcoinSyncPauseRequest::default()),
    )
    .expect("sync pause");
    let resume = dispatch(
        &mut context,
        MethodCall::OpenBitcoinSyncResume(OpenBitcoinSyncResumeRequest::default()),
    )
    .expect("sync resume");

    // Assert
    assert_eq!(status["metadata"]["sync_control"]["paused"], json!(false));
    assert_eq!(pause["metadata"]["sync_control"]["paused"], json!(true));
    assert_eq!(resume["metadata"]["sync_control"]["paused"], json!(false));
    let stored_metadata = store
        .load_runtime_metadata()
        .expect("load metadata")
        .expect("metadata");
    assert!(!stored_metadata.sync_control.paused);
}

#[test]
fn open_bitcoin_sync_rpc_control_requires_daemon_handle() {
    // Arrange
    let mut context = empty_context();

    // Act
    let error = dispatch(
        &mut context,
        MethodCall::OpenBitcoinSyncStatus(OpenBitcoinSyncStatusRequest::default()),
    )
    .expect_err("sync control should require daemon handle");

    // Assert
    assert_eq!(error.kind, RpcFailureKind::ClientNotConnected);
    assert_eq!(
        error
            .maybe_detail
            .expect("sync control error detail")
            .message,
        "daemon sync control is unavailable"
    );
}

fn spawn_test_sync_control_worker(receiver: DaemonSyncControlReceiver) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut metadata = RuntimeMetadata::default();
        while let Ok(request) = receiver.recv_timeout(std::time::Duration::from_secs(1)) {
            match request.action() {
                DaemonSyncControlAction::Status => {}
                DaemonSyncControlAction::Pause => metadata.sync_control.paused = true,
                DaemonSyncControlAction::Resume => metadata.sync_control.paused = false,
            }
            request.respond(Ok(metadata.clone()));
        }
    })
}

fn phase62_runtime_metadata() -> RuntimeMetadata {
    RuntimeMetadata {
        maybe_sync_state: Some(DurableSyncState {
            sync: SyncStatus {
                network: FieldAvailability::available("main".to_string()),
                chain_tip: FieldAvailability::available(ChainTipStatus {
                    height: 840_004,
                    block_hash: "00".repeat(32),
                }),
                sync_progress: FieldAvailability::available(SyncProgress {
                    header_height: 840_200,
                    block_height: 840_004,
                    downloaded_block_height: 840_006,
                    connected_block_height: 840_004,
                    validated_active_chain_height: 840_004,
                    maybe_downloaded_block_hash: Some("11".repeat(32)),
                    maybe_connected_block_hash: Some("00".repeat(32)),
                    maybe_validated_active_chain_hash: Some("00".repeat(32)),
                    maybe_validated_active_chain_work: Some("840005".to_string()),
                    progress_ratio: 840_004.0 / 840_200.0,
                    messages_processed: 42,
                    headers_received: 100,
                    blocks_received: 3,
                }),
                lifecycle: FieldAvailability::available(SyncLifecycleState::Active),
                phase: FieldAvailability::available("block_download".to_string()),
                configured_targets: FieldAvailability::available(SyncConfiguredTargets {
                    target_outbound_peers: 4,
                    maybe_target_header_height: Some(840_200),
                }),
                attempt_counters: FieldAvailability::available(SyncAttemptCounters {
                    attempted_peers: 3,
                    connected_peers: 2,
                    failed_peers: 1,
                    max_sync_rounds: 8,
                }),
                progress_signal: FieldAvailability::available(SyncProgressSignal::HeaderProgress),
                lag: FieldAvailability::available(SyncLagStatus {
                    headers_remaining: 0,
                    blocks_remaining: 100,
                }),
                last_successful_progress_unix_seconds: FieldAvailability::available(1_715_000_000),
                progress_credit: FieldAvailability::unavailable(
                    "progress credit evidence unavailable",
                ),
                expected_progress_window: FieldAvailability::unavailable(
                    "expected progress window unavailable",
                ),
                no_progress_threshold: FieldAvailability::unavailable(
                    "no-progress threshold evidence unavailable",
                ),
                last_useful_work: FieldAvailability::unavailable("last useful work unavailable"),
                last_peer_contribution: FieldAvailability::unavailable(
                    "last peer contribution unavailable",
                ),
                stall_diagnosis: FieldAvailability::unavailable("stall diagnosis unavailable"),
                latest_stop_reason: FieldAvailability::available(SyncStopReasonStatus {
                    label: "target_header_reached".to_string(),
                    message: "sync header target reached".to_string(),
                }),
                last_error: FieldAvailability::available(
                    "peer stalled before block connect".to_string(),
                ),
                recovery_category: FieldAvailability::available(
                    SyncRecoveryCategory::InvalidPeerData,
                ),
                recovery_action: FieldAvailability::available(
                    "Restart the node and retry the storage operation.".to_string(),
                ),
                resource_pressure: FieldAvailability::available(SyncResourcePressure {
                    blocks_in_flight: 8,
                    max_header_requests_in_flight_per_peer: 1,
                    max_headers_per_message: 2_000,
                    max_blocks_in_flight_per_peer: 16,
                    max_blocks_in_flight_total: 64,
                    max_messages_per_peer: 64,
                    max_sync_rounds: 8,
                    outbound_peers: 2,
                    target_outbound_peers: 4,
                }),
                best_known_tip: FieldAvailability::<BestKnownTipStatus>::unavailable(
                    "best-known tip evidence unavailable",
                ),
                stay_current: FieldAvailability::<StayCurrentStatus>::unavailable(
                    "stay-current state unavailable",
                ),
                stay_current_next_action: FieldAvailability::unavailable(
                    "stay-current next action unavailable",
                ),
                no_progress_diagnosis: FieldAvailability::unavailable(
                    "no-progress diagnosis unavailable",
                ),
                no_progress_next_action: FieldAvailability::unavailable(
                    "no-progress next action unavailable",
                ),
                latest_reorg: FieldAvailability::unavailable("no reorg evidence recorded"),
                reconcile_progress: FieldAvailability::unavailable(
                    "reconcile progress unavailable",
                ),
            },
            peers: PeerStatus {
                peer_counts: FieldAvailability::available(PeerCounts {
                    inbound: 0,
                    outbound: 2,
                }),
                recent_peers: FieldAvailability::available(Vec::new()),
                inbound: FieldAvailability::<InboundPeerServingStatus>::unavailable(
                    INBOUND_STATUS_UNAVAILABLE_REASON,
                ),
            },
            health_signals: Vec::new(),
            updated_at_unix_seconds: 1_715_000_000,
        }),
        ..RuntimeMetadata::default()
    }
}

fn phase72_runtime_metadata() -> RuntimeMetadata {
    let mut metadata = phase62_runtime_metadata();
    let sync_state = metadata
        .maybe_sync_state
        .as_mut()
        .expect("phase62 metadata includes sync state");
    let FieldAvailability::Available(sync_progress) = &mut sync_state.sync.sync_progress else {
        panic!("phase62 metadata includes sync progress");
    };
    sync_progress.maybe_connected_block_hash = Some("11".repeat(32));
    sync_progress.maybe_validated_active_chain_hash = Some("11".repeat(32));
    sync_state.sync.best_known_tip = FieldAvailability::available(BestKnownTipStatus {
        source: BestKnownTipSource::HeaderStore,
        height: 840_004,
        block_hash: "11".repeat(32),
        work: "840005".to_string(),
        block_time_unix_seconds: 1_717_000_010,
        observed_at_unix_seconds: 1_717_000_020,
        freshness: TipFreshnessStatus::Fresh,
        peer_agreement: Vec::new(),
    });
    sync_state.sync.stay_current =
        FieldAvailability::available(StayCurrentStatus::CurrentAtBestKnownTip);
    sync_state.sync.no_progress_diagnosis =
        FieldAvailability::available(NoProgressDiagnosis::PeerBackoff);
    sync_state.sync.latest_reorg = FieldAvailability::available(SyncReorgEvidence {
        common_ancestor_height: 840_000,
        common_ancestor_hash: "11".repeat(32),
        disconnected_count: 2,
        connected_count: 4,
        final_active_height: 840_004,
        final_active_hash: "11".repeat(32),
        fully_persisted: true,
    });
    sync_state.sync.reconcile_progress =
        FieldAvailability::available(SyncReconcileProgressStatus::ExtendedActiveChain {
            connected_count: 4,
            final_active_height: 840_004,
            final_active_hash: "11".repeat(32),
        });
    metadata
}

fn context_with_runtime_metadata(test_name: &str, metadata: RuntimeMetadata) -> ManagedRpcContext {
    let path = temp_store_path(test_name);
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    store
        .save_runtime_metadata(&metadata, PersistMode::Sync)
        .expect("save runtime metadata");
    let mut context = ManagedRpcContext::from_runtime_config(&RuntimeConfig {
        chain: AddressNetwork::Mainnet,
        maybe_data_dir: Some(path),
        ..RuntimeConfig::default()
    });
    context.set_daemon_sync_control(DaemonSyncControl::store_backed(store, PersistMode::Sync));
    context
}

#[test]
fn get_blockchain_info_uses_durable_connected_block_height_not_downloaded_height() {
    // Arrange
    let path = temp_store_path("durable-sync-truth");
    let store = FjallNodeStore::open(&path).expect("store");
    store
        .save_runtime_metadata(
            &RuntimeMetadata {
                maybe_sync_state: Some(DurableSyncState {
                    sync: SyncStatus {
                        network: FieldAvailability::available("main".to_string()),
                        chain_tip: FieldAvailability::available(ChainTipStatus {
                            height: 840_004,
                            block_hash: "00".repeat(32),
                        }),
                        sync_progress: FieldAvailability::available(SyncProgress {
                            header_height: 840_200,
                            block_height: 840_004,
                            downloaded_block_height: 840_006,
                            connected_block_height: 840_004,
                            validated_active_chain_height: 840_004,
                            maybe_downloaded_block_hash: Some("11".repeat(32)),
                            maybe_connected_block_hash: Some("00".repeat(32)),
                            maybe_validated_active_chain_hash: Some("00".repeat(32)),
                            maybe_validated_active_chain_work: Some("840005".to_string()),
                            progress_ratio: 840_004.0 / 840_200.0,
                            messages_processed: 42,
                            headers_received: 100,
                            blocks_received: 3,
                        }),
                        lifecycle: FieldAvailability::available(SyncLifecycleState::Active),
                        phase: FieldAvailability::available("block_download".to_string()),
                        configured_targets: FieldAvailability::available(SyncConfiguredTargets {
                            target_outbound_peers: 4,
                            maybe_target_header_height: Some(840_200),
                        }),
                        attempt_counters: FieldAvailability::available(SyncAttemptCounters {
                            attempted_peers: 3,
                            connected_peers: 2,
                            failed_peers: 1,
                            max_sync_rounds: 8,
                        }),
                        progress_signal: FieldAvailability::available(
                            SyncProgressSignal::HeaderProgress,
                        ),
                        lag: FieldAvailability::available(SyncLagStatus {
                            headers_remaining: 0,
                            blocks_remaining: 100,
                        }),
                        last_successful_progress_unix_seconds: FieldAvailability::available(
                            1_715_000_000,
                        ),
                        progress_credit: FieldAvailability::unavailable(
                            "progress credit evidence unavailable",
                        ),
                        expected_progress_window: FieldAvailability::unavailable(
                            "expected progress window unavailable",
                        ),
                        no_progress_threshold: FieldAvailability::unavailable(
                            "no-progress threshold evidence unavailable",
                        ),
                        last_useful_work: FieldAvailability::unavailable(
                            "last useful work unavailable",
                        ),
                        last_peer_contribution: FieldAvailability::unavailable(
                            "last peer contribution unavailable",
                        ),
                        stall_diagnosis: FieldAvailability::unavailable(
                            "stall diagnosis unavailable",
                        ),
                        latest_stop_reason: FieldAvailability::available(SyncStopReasonStatus {
                            label: "target_header_reached".to_string(),
                            message: "sync header target reached".to_string(),
                        }),
                        last_error: FieldAvailability::available(
                            "peer stalled before block connect".to_string(),
                        ),
                        recovery_category: FieldAvailability::available(
                            SyncRecoveryCategory::InvalidPeerData,
                        ),
                        recovery_action: FieldAvailability::available(
                            "Restart the node and retry the storage operation.".to_string(),
                        ),
                        resource_pressure: FieldAvailability::available(SyncResourcePressure {
                            blocks_in_flight: 8,
                            max_header_requests_in_flight_per_peer: 1,
                            max_headers_per_message: 2_000,
                            max_blocks_in_flight_per_peer: 16,
                            max_blocks_in_flight_total: 64,
                            max_messages_per_peer: 64,
                            max_sync_rounds: 8,
                            outbound_peers: 2,
                            target_outbound_peers: 4,
                        }),
                        best_known_tip: FieldAvailability::<BestKnownTipStatus>::unavailable(
                            "best-known tip evidence unavailable",
                        ),
                        stay_current: FieldAvailability::<StayCurrentStatus>::unavailable(
                            "stay-current state unavailable",
                        ),
                        stay_current_next_action: FieldAvailability::unavailable(
                            "stay-current next action unavailable",
                        ),
                        no_progress_diagnosis: FieldAvailability::unavailable(
                            "no-progress diagnosis unavailable",
                        ),
                        no_progress_next_action: FieldAvailability::unavailable(
                            "no-progress next action unavailable",
                        ),
                        latest_reorg: FieldAvailability::unavailable("no reorg evidence recorded"),
                        reconcile_progress: FieldAvailability::unavailable(
                            "reconcile progress unavailable",
                        ),
                    },
                    peers: PeerStatus {
                        peer_counts: FieldAvailability::available(PeerCounts {
                            inbound: 0,
                            outbound: 2,
                        }),
                        recent_peers: FieldAvailability::available(Vec::new()),
                        inbound: FieldAvailability::<InboundPeerServingStatus>::unavailable(
                            INBOUND_STATUS_UNAVAILABLE_REASON,
                        ),
                    },
                    health_signals: Vec::new(),
                    updated_at_unix_seconds: 1_715_000_000,
                }),
                ..RuntimeMetadata::default()
            },
            PersistMode::Sync,
        )
        .expect("save runtime metadata");
    drop(store);
    let reopened = FjallNodeStore::open(&path).expect("reopen store");
    let reopened_metadata = reopened
        .load_runtime_metadata()
        .expect("load runtime metadata")
        .expect("runtime metadata");
    assert!(reopened_metadata.maybe_sync_state.is_some());
    drop(reopened);
    let mut context = ManagedRpcContext::from_runtime_config(&RuntimeConfig {
        chain: AddressNetwork::Mainnet,
        maybe_data_dir: Some(path),
        ..RuntimeConfig::default()
    });
    assert!(
        context
            .current_durable_sync_state()
            .expect("current durable sync state")
            .is_some()
    );

    // Act
    let blockchain = dispatch(
        &mut context,
        MethodCall::GetBlockchainInfo(GetBlockchainInfoRequest::default()),
    )
    .expect("blockchain");

    // Assert
    assert_eq!(blockchain["headers"], json!(840200));
    assert_eq!(blockchain["blocks"], json!(840004));
    assert_eq!(blockchain["initialblockdownload"], json!(true));
    assert_eq!(
        blockchain["warnings"][0],
        json!("peer stalled before block connect")
    );
    assert_eq!(
        blockchain["warnings"][1],
        json!("progress_signal=header_progress")
    );
    assert_eq!(
        blockchain["warnings"][2],
        json!("latest_stop_reason=target_header_reached")
    );
    assert_eq!(
        blockchain["warnings"][3],
        json!("recovery_category=invalid_peer_data")
    );
    assert_eq!(
        blockchain["warnings"][4],
        json!("Restart the node and retry the storage operation.")
    );
}

#[test]
fn open_bitcoin_sync_status_returns_phase72_durable_truth_contract() {
    // Arrange
    let mut context =
        context_with_runtime_metadata("sync-status-phase72", phase72_runtime_metadata());

    // Act
    let status = dispatch(
        &mut context,
        MethodCall::OpenBitcoinSyncStatus(OpenBitcoinSyncStatusRequest::default()),
    )
    .expect("sync status");

    // Assert
    let sync = &status["metadata"]["maybe_sync_state"]["sync"];
    assert_eq!(
        sync["sync_progress"]["value"]["validated_active_chain_height"],
        json!(840_004)
    );
    assert_eq!(
        sync["sync_progress"]["value"]["maybe_validated_active_chain_hash"],
        json!("11".repeat(32))
    );
    assert_eq!(
        sync["sync_progress"]["value"]["maybe_validated_active_chain_work"],
        json!("840005")
    );
    assert_eq!(sync["best_known_tip"]["state"], json!("available"));
    assert_eq!(sync["best_known_tip"]["value"]["freshness"], json!("fresh"));
    assert_eq!(
        sync["stay_current"]["value"],
        json!("current_at_best_known_tip")
    );
    assert_eq!(
        sync["no_progress_diagnosis"]["value"],
        json!("peer_backoff")
    );
    assert_eq!(
        sync["latest_reorg"]["value"]["final_active_height"],
        json!(840_004)
    );
    assert_eq!(
        sync["reconcile_progress"]["value"]["state"],
        json!("extended_active_chain")
    );
    assert_eq!(sync["resource_pressure"]["state"], json!("available"));
}

#[test]
fn get_blockchain_info_does_not_expose_phase72_support_fields() {
    // Arrange
    let mut context =
        context_with_runtime_metadata("blockchain-info-phase72", phase72_runtime_metadata());

    // Act
    let blockchain = dispatch(
        &mut context,
        MethodCall::GetBlockchainInfo(GetBlockchainInfoRequest::default()),
    )
    .expect("blockchain");
    let serialized = serde_json::to_string(&blockchain).expect("serialize blockchain info");

    // Assert
    for forbidden in [
        "best_known_tip",
        "stay_current",
        "latest_reorg",
        "reconcile_progress",
        "resource_pressure",
        "support_evidence",
        "evidence_verdict",
        "validated_active_chain_work",
    ] {
        assert!(
            !serialized.contains(forbidden),
            "baseline getblockchaininfo exposed {forbidden}"
        );
    }
}

#[test]
fn open_bitcoin_sync_status_returns_phase62_metadata_fields() {
    // Arrange
    let path = temp_store_path("sync-status-phase62");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    store
        .save_runtime_metadata(&phase62_runtime_metadata(), PersistMode::Sync)
        .expect("save metadata");
    let mut context = empty_context();
    context.set_daemon_sync_control(DaemonSyncControl::store_backed(store, PersistMode::Sync));

    // Act
    let status = dispatch(
        &mut context,
        MethodCall::OpenBitcoinSyncStatus(OpenBitcoinSyncStatusRequest::default()),
    )
    .expect("sync status");

    // Assert
    assert_eq!(
        status["metadata"]["maybe_sync_state"]["sync"]["configured_targets"]["value"]["target_outbound_peers"],
        json!(4)
    );
    assert_eq!(
        status["metadata"]["maybe_sync_state"]["sync"]["attempt_counters"]["value"]["attempted_peers"],
        json!(3)
    );
    assert_eq!(
        status["metadata"]["maybe_sync_state"]["sync"]["latest_stop_reason"]["value"]["label"],
        json!("target_header_reached")
    );
}

#[test]
fn deriveaddresses_returns_expected_addresses_for_supported_descriptors() {
    // Arrange
    let mut context = empty_context();
    let request = DeriveAddressesRequest {
        descriptor: "wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi)#8fhd9pwu"
            .to_string(),
        maybe_range: None,
    };

    // Act
    let response = dispatch(&mut context, MethodCall::DeriveAddresses(request)).expect("derive");

    // Assert
    assert_eq!(
        response,
        json!({
            "addresses": ["bcrt1qa0qwuze2h85zw7nqpsj3ga0z9geyrgwpf2m8je"]
        })
    );
}

#[test]
fn getwalletinfo_reports_wallet_identity_and_freshness_fields() {
    // Arrange
    let mut context = funded_wallet_context();

    // Act
    let response = dispatch(
        &mut context,
        MethodCall::GetWalletInfo(GetWalletInfoRequest::default()),
    )
    .expect("wallet info");

    // Assert
    assert_eq!(response["network"], json!("regtest"));
    assert_eq!(response["descriptor_count"], json!(2));
    assert_eq!(response["utxo_count"], json!(1));
    assert_eq!(response["maybe_tip_height"], json!(10));
    assert_eq!(
        response["maybe_tip_median_time_past"],
        json!(1700000010_i64)
    );
    assert_eq!(response["walletname"], json!(null));
    assert_eq!(response["scanning"], json!(false));
    assert_eq!(response["freshness"], json!("fresh"));
}

#[test]
fn wallet_descriptor_and_rescan_methods_update_wallet_views() {
    // Arrange
    let mut context = empty_context();
    let import_request = ImportDescriptorsRequest {
        requests: vec![
            crate::method::DescriptorImportItem {
                descriptor: "wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi)"
                    .to_string(),
                label: "receive".to_string(),
                internal: false,
                maybe_rescan_since_height: Some(0),
            },
            crate::method::DescriptorImportItem {
                descriptor: "sh(wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi))"
                    .to_string(),
                label: "change".to_string(),
                internal: true,
                maybe_rescan_since_height: Some(0),
            },
        ],
    };
    let reference_wallet = wallet_with_descriptors();
    let receive_script = reference_wallet
        .default_receive_address()
        .expect("receive")
        .script_pubkey;
    let genesis = build_block(
        BlockHash::from_byte_array([0_u8; 32]),
        0,
        75_000,
        receive_script,
    );
    context.connect_local_block(&genesis).expect("genesis");

    // Act
    let import_response =
        dispatch(&mut context, MethodCall::ImportDescriptors(import_request)).expect("import");
    let rescan_response = dispatch(
        &mut context,
        MethodCall::RescanBlockchain(RescanBlockchainRequest {
            maybe_start_height: Some(0),
            maybe_stop_height: Some(0),
        }),
    )
    .expect("rescan");
    let balances = dispatch(
        &mut context,
        MethodCall::GetBalances(GetBalancesRequest::default()),
    )
    .expect("balances");
    let unspent = dispatch(
        &mut context,
        MethodCall::ListUnspent(ListUnspentRequest::default()),
    )
    .expect("listunspent");

    // Assert
    assert_eq!(import_response["results"][0]["success"], json!(true));
    assert_eq!(rescan_response["start_height"], json!(0));
    assert_eq!(rescan_response["stop_height"], json!(0));
    assert_eq!(balances["mine"]["trusted_sats"], json!(75_000));
    assert_eq!(balances["mine"]["immature_sats"], json!(0));
    assert_eq!(unspent["entries"][0]["descriptor_id"], json!(0));
    assert_eq!(unspent["entries"][0]["amount_sats"], json!(75_000));
}

#[test]
fn durable_wallet_methods_persist_address_cursors_and_descriptor_metadata() {
    // Arrange
    let mut context = durable_wallet_context("descriptor-cursors", "alpha");

    // Act
    let first_receive = dispatch(&mut context, MethodCall::GetNewAddress(Default::default()))
        .expect("first receive");
    let second_receive = dispatch(&mut context, MethodCall::GetNewAddress(Default::default()))
        .expect("second receive");
    let change = dispatch(
        &mut context,
        MethodCall::GetRawChangeAddress(Default::default()),
    )
    .expect("change");
    let descriptors = dispatch(
        &mut context,
        MethodCall::ListDescriptors(Default::default()),
    )
    .expect("descriptors");
    let wallet_info = dispatch(
        &mut context,
        MethodCall::GetWalletInfo(GetWalletInfoRequest::default()),
    )
    .expect("wallet info");

    // Assert
    assert_ne!(first_receive, second_receive);
    assert_ne!(first_receive, change);
    assert_eq!(descriptors["walletname"], json!("alpha"));
    assert_eq!(descriptors["descriptors"][0]["internal"], json!(false));
    assert_eq!(descriptors["descriptors"][0]["maybe_next_index"], json!(2));
    assert_eq!(descriptors["descriptors"][1]["internal"], json!(true));
    assert_eq!(descriptors["descriptors"][1]["maybe_next_index"], json!(1));
    assert_eq!(wallet_info["walletname"], json!("alpha"));
    assert_eq!(wallet_info["freshness"], json!("fresh"));
}

#[test]
fn rescanblockchain_accepts_ranges_and_records_partial_freshness() {
    // Arrange
    let mut context = durable_wallet_context("range-rescan", "alpha");
    let receive_script = context
        .descriptor_address(0)
        .expect("receive address")
        .script_pubkey;
    let genesis = build_block(
        BlockHash::from_byte_array([0_u8; 32]),
        0,
        75_000,
        receive_script.clone(),
    );
    let block_one = build_block(block_hash(&genesis.header), 1, 75_000, receive_script);
    let block_two = build_block(block_hash(&block_one.header), 2, 75_000, p2sh_script());
    context.connect_local_block(&genesis).expect("genesis");
    context.connect_local_block(&block_one).expect("block one");
    context.connect_local_block(&block_two).expect("block two");

    // Act
    let partial_rescan = dispatch(
        &mut context,
        MethodCall::RescanBlockchain(RescanBlockchainRequest {
            maybe_start_height: Some(1),
            maybe_stop_height: Some(1),
        }),
    )
    .expect("partial range");
    let wallet_info_after_partial = dispatch(
        &mut context,
        MethodCall::GetWalletInfo(GetWalletInfoRequest::default()),
    )
    .expect("wallet info after partial");
    let full_rescan = dispatch(
        &mut context,
        MethodCall::RescanBlockchain(RescanBlockchainRequest {
            maybe_start_height: Some(0),
            maybe_stop_height: Some(2),
        }),
    )
    .expect("full rescan");

    // Assert
    assert_eq!(partial_rescan["start_height"], json!(1));
    assert_eq!(partial_rescan["stop_height"], json!(1));
    assert_eq!(partial_rescan["freshness"], json!("partial"));
    assert_eq!(wallet_info_after_partial["freshness"], json!("partial"));
    assert_eq!(wallet_info_after_partial["walletname"], json!("alpha"));
    assert_eq!(full_rescan["freshness"], json!("fresh"));
    assert_eq!(full_rescan["maybe_scanned_through_height"], json!(2));
}

#[test]
fn sendrawtransaction_returns_txid_and_maps_rejections() {
    // Arrange
    let mut context = empty_context();
    let genesis = build_block(
        BlockHash::from_byte_array([0_u8; 32]),
        0,
        500_000_000,
        p2sh_script(),
    );
    let spendable = build_block(block_hash(&genesis.header), 1, 500_000_000, p2sh_script());
    context.connect_local_block(&genesis).expect("genesis");
    context.connect_local_block(&spendable).expect("spendable");
    let transaction = spend_transaction(
        transaction_txid(&genesis.transactions[0]).expect("txid"),
        499_999_000,
    );
    let transaction_hex = encode_hex(
        &encode_transaction(&transaction, TransactionEncoding::WithWitness).expect("encode"),
    );
    let expected_txid = encode_hex(transaction_txid(&transaction).expect("txid").as_bytes());

    // Act
    let success = dispatch(
        &mut context,
        MethodCall::SendRawTransaction(SendRawTransactionRequest {
            transaction_hex: transaction_hex.clone(),
            maybe_max_fee_rate_sat_per_kvb: None,
            maybe_max_burn_amount_sats: None,
            ignore_rejects: Vec::new(),
        }),
    )
    .expect("submit");
    let failure = dispatch(
        &mut context,
        MethodCall::SendRawTransaction(SendRawTransactionRequest {
            transaction_hex,
            maybe_max_fee_rate_sat_per_kvb: None,
            maybe_max_burn_amount_sats: None,
            ignore_rejects: Vec::new(),
        }),
    )
    .expect_err("duplicate");

    // Assert
    assert_eq!(success["txid_hex"], json!(expected_txid));
    assert_eq!(
        failure.maybe_detail.as_ref().map(|detail| detail.code),
        Some(RpcErrorCode::VerifyRejected),
    );
}

#[test]
fn sendrawtransaction_queues_internal_relay_evidence_without_propagation_claim() {
    // Arrange
    let mut context = relay_enabled_context(44);
    let genesis = build_block(
        BlockHash::from_byte_array([0_u8; 32]),
        0,
        500_000_000,
        p2sh_script(),
    );
    let spendable = build_block(block_hash(&genesis.header), 1, 500_000_000, p2sh_script());
    context.connect_local_block(&genesis).expect("genesis");
    context.connect_local_block(&spendable).expect("spendable");
    context.connect_outbound_peer(44, 1).expect("outbound");
    let transaction = spend_transaction(
        transaction_txid(&genesis.transactions[0]).expect("txid"),
        499_999_000,
    );
    let transaction_hex = encode_hex(
        &encode_transaction(&transaction, TransactionEncoding::WithWitness).expect("encode"),
    );
    let submitted_transaction_hex = transaction_hex.clone();
    let expected_txid = encode_hex(transaction_txid(&transaction).expect("txid").as_bytes());

    // Act
    let success = dispatch(
        &mut context,
        MethodCall::SendRawTransaction(SendRawTransactionRequest {
            transaction_hex,
            maybe_max_fee_rate_sat_per_kvb: None,
            maybe_max_burn_amount_sats: None,
            ignore_rejects: Vec::new(),
        }),
    )
    .expect("submit");

    // Assert
    let response = success.as_object().expect("response object");
    assert_eq!(response.len(), 3);
    assert!(response.contains_key("txid_hex"));
    assert!(response.contains_key("replaced_txids"));
    assert!(response.contains_key("evicted_txids"));
    for forbidden_key in [
        "propagated",
        "broadcast",
        "public_relay",
        "production_ready",
    ] {
        assert!(!response.contains_key(forbidden_key));
    }
    let response_json = success.to_string();
    for forbidden in ["broadcast", "propagation", "public", "guaranteed"] {
        assert!(!response_json.contains(forbidden));
    }
    let evidence = context
        .latest_local_submission_evidence()
        .expect("authoritative relay evidence")
        .expect("relay evidence");
    assert_eq!(evidence.queued_count, 1);
    assert_eq!(
        evidence
            .labels
            .iter()
            .map(|label| label.as_str())
            .collect::<Vec<_>>(),
        vec!["accepted", "queued", "rebroadcast_deferred"],
    );
    let status = dispatch(
        &mut context,
        MethodCall::OpenBitcoinNetworkStatus(OpenBitcoinNetworkStatusRequest::default()),
    )
    .expect("network status");
    assert_eq!(
        status["relay"]["outcome_counters"]["value"]["accepted_count"],
        json!(1)
    );
    assert_eq!(
        status["relay"]["outcome_counters"]["value"]["rebroadcast_deferred_count"],
        json!(1)
    );
    assert_eq!(
        status["relay"]["activation"]["value"]["enabled"],
        json!(true)
    );
    assert_eq!(
        status["relay"]["download_eligibility"]["value"]["eligible_peer_count"],
        json!(1)
    );
    assert_eq!(
        status["relay"]["local_submission"]["state"],
        json!("implemented")
    );
    assert_eq!(
        status["relay"]["rebroadcast"]["state"],
        json!("implemented")
    );
    let status_json = serde_json::to_string(&status).expect("network status json");
    assert!(!status_json.contains(&submitted_transaction_hex));
    assert!(!status_json.contains(&expected_txid));
}

#[test]
fn sendrawtransaction_duplicate_does_not_queue_new_fanout() {
    // Arrange
    let mut context = relay_enabled_context(45);
    let genesis = build_block(
        BlockHash::from_byte_array([0_u8; 32]),
        0,
        500_000_000,
        p2sh_script(),
    );
    let spendable = build_block(block_hash(&genesis.header), 1, 500_000_000, p2sh_script());
    context.connect_local_block(&genesis).expect("genesis");
    context.connect_local_block(&spendable).expect("spendable");
    context.connect_outbound_peer(45, 1).expect("outbound");
    let transaction = spend_transaction(
        transaction_txid(&genesis.transactions[0]).expect("txid"),
        499_999_000,
    );
    let transaction_hex = encode_hex(
        &encode_transaction(&transaction, TransactionEncoding::WithWitness).expect("encode"),
    );
    dispatch(
        &mut context,
        MethodCall::SendRawTransaction(SendRawTransactionRequest {
            transaction_hex: transaction_hex.clone(),
            maybe_max_fee_rate_sat_per_kvb: None,
            maybe_max_burn_amount_sats: None,
            ignore_rejects: Vec::new(),
        }),
    )
    .expect("initial submit");

    // Act
    let duplicate = dispatch(
        &mut context,
        MethodCall::SendRawTransaction(SendRawTransactionRequest {
            transaction_hex,
            maybe_max_fee_rate_sat_per_kvb: None,
            maybe_max_burn_amount_sats: None,
            ignore_rejects: Vec::new(),
        }),
    )
    .expect_err("duplicate");

    // Assert
    assert_eq!(
        duplicate.maybe_detail.as_ref().map(|detail| detail.code),
        Some(RpcErrorCode::VerifyRejected),
    );
    let evidence = context
        .latest_local_submission_evidence()
        .expect("authoritative relay evidence")
        .expect("relay evidence");
    assert_eq!(evidence.queued_count, 0);
    assert_eq!(
        evidence
            .labels
            .iter()
            .map(|label| label.as_str())
            .collect::<Vec<_>>(),
        vec!["duplicate"],
    );
}

#[test]
fn sendrawtransaction_rejects_unenforced_fee_limits_before_mempool_submission() {
    // Arrange
    let mut context = empty_context();

    // Act
    let fee_limit_failure = dispatch(
        &mut context,
        MethodCall::SendRawTransaction(SendRawTransactionRequest {
            transaction_hex: "not hex".to_string(),
            maybe_max_fee_rate_sat_per_kvb: Some(1),
            maybe_max_burn_amount_sats: None,
            ignore_rejects: Vec::new(),
        }),
    )
    .expect_err("maxfeerate");
    let burn_limit_failure = dispatch(
        &mut context,
        MethodCall::SendRawTransaction(SendRawTransactionRequest {
            transaction_hex: "not hex".to_string(),
            maybe_max_fee_rate_sat_per_kvb: None,
            maybe_max_burn_amount_sats: Some(1),
            ignore_rejects: Vec::new(),
        }),
    )
    .expect_err("maxburnamount");
    let mempool = dispatch(
        &mut context,
        MethodCall::GetMempoolInfo(GetMempoolInfoRequest::default()),
    )
    .expect("mempool");

    // Assert
    let fee_detail = fee_limit_failure.maybe_detail.expect("fee detail");
    assert_eq!(fee_detail.code, RpcErrorCode::InvalidParams);
    assert_eq!(
        fee_detail.message,
        "sendrawtransaction maxfeerate enforcement is not supported in Phase 8; omit maxfeerate",
    );
    let burn_detail = burn_limit_failure.maybe_detail.expect("burn detail");
    assert_eq!(burn_detail.code, RpcErrorCode::InvalidParams);
    assert_eq!(
        burn_detail.message,
        "sendrawtransaction maxburnamount enforcement is not supported in Phase 8; omit maxburnamount",
    );
    assert_eq!(mempool["size"], json!(0));
}

#[test]
fn buildandsigntransaction_returns_deterministic_hex_and_fee() {
    // Arrange
    let mut context = funded_wallet_context();
    let request = BuildAndSignTransactionRequest {
        recipients: vec![TransactionRecipient {
            script_pubkey_hex: "51".to_string(),
            amount_sats: 30_000,
        }],
        fee_rate_sat_per_kvb: 2_000,
        maybe_change_descriptor_id: None,
        maybe_lock_time: None,
        enable_rbf: true,
    };

    // Act
    let first = dispatch(
        &mut context,
        MethodCall::BuildAndSignTransaction(request.clone()),
    )
    .expect("first");
    let second =
        dispatch(&mut context, MethodCall::BuildAndSignTransaction(request)).expect("second");

    // Assert
    assert_eq!(first, second);
    assert_eq!(first["fee_sats"], json!(242));
    assert!(first["transaction_hex"].as_str().expect("hex").len() > 10);
    assert_eq!(first["inputs"][0]["amount_sats"], json!(75_000));
}

#[test]
fn sendtoaddress_reuses_the_build_and_sign_spend_path() {
    // Arrange
    let mut context = spendable_send_context();
    let destination_script = SingleKeyDescriptor::parse(
        "wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi)#8fhd9pwu",
        AddressNetwork::Regtest,
    )
    .expect("descriptor")
    .address(AddressNetwork::Regtest)
    .expect("destination address")
    .script_pubkey;
    let build_request = BuildAndSignTransactionRequest {
        recipients: vec![TransactionRecipient {
            script_pubkey_hex: encode_hex(destination_script.as_bytes()),
            amount_sats: 30_000,
        }],
        fee_rate_sat_per_kvb: 2_000,
        maybe_change_descriptor_id: None,
        maybe_lock_time: None,
        enable_rbf: true,
    };

    // Act
    let built = dispatch(
        &mut context,
        MethodCall::BuildAndSignTransaction(build_request),
    )
    .expect("build and sign");
    let expected_transaction = parse_transaction(&decode_hex(
        built["transaction_hex"].as_str().expect("transaction hex"),
    ))
    .expect("parse built transaction");
    let send = dispatch(
        &mut context,
        MethodCall::SendToAddress(SendToAddressRequest {
            address: "bcrt1qa0qwuze2h85zw7nqpsj3ga0z9geyrgwpf2m8je".to_string(),
            amount_sats: 30_000,
            maybe_fee_rate_sat_per_kvb: Some(2_000),
            maybe_conf_target: None,
            maybe_estimate_mode: None,
            maybe_change_descriptor_id: None,
            maybe_lock_time: None,
            enable_rbf: true,
            maybe_max_tx_fee_sats: Some(1_000),
        }),
    )
    .expect("sendtoaddress");

    // Assert
    assert_eq!(
        send,
        json!(encode_hex(
            transaction_txid(&expected_transaction)
                .expect("expected txid")
                .as_bytes()
        )),
    );
}
