// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/node/txdownloadman.h
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/protocol.h
// - packages/bitcoin-knots/src/txorphanage.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/test/functional/mempool_persist.py
// - packages/bitcoin-knots/test/functional/p2p_getdata.py
// - packages/bitcoin-knots/test/functional/p2p_orphan_handling.py
// - packages/bitcoin-knots/test/functional/p2p_tx_download.py
// - packages/bitcoin-knots/test/functional/mempool_accept.py

use std::{cell::Cell, net::IpAddr};

use open_bitcoin_codec::{CompactBlockPayload, PrefilledTransaction, SendCompactMessage};
use open_bitcoin_core::consensus::crypto::hash160;
use open_bitcoin_core::{
    codec::{encode_block, parse_block},
    consensus::{
        ConsensusParams, ScriptVerifyFlags, block_hash, block_merkle_root, check_block_header,
        transaction_txid, transaction_wtxid,
    },
    primitives::{
        Amount, Block, BlockHash, BlockHeader, Hash32, InventoryType, InventoryVector,
        NetworkAddress, NetworkMagic, OutPoint, ScriptBuf, ScriptWitness, Transaction,
        TransactionInput, TransactionOutput,
    },
};
use open_bitcoin_mempool::{
    FeeRate, IncrementalRelayFeeRate, MempoolCapacity, MempoolCapacityEnforcement, PolicyConfig,
    RelayIntent, RollingMempoolFeeRate, StaticRelayFeeRate,
};
use open_bitcoin_network::{
    AddressAnnouncement, AddressDecisionLabel, AddressDecisionReason, AddressList,
    AddressNetworkKind, AddressSourceKind, BanReason, BanScope, BlockRelayActivationPolicy,
    BlockServingActivationConfig, CompactAnnouncementAction, CompactAnnouncementDecision,
    CompactAnnouncementEligibility, CompactAnnouncementEligibilityReason,
    CompactAnnouncementReason, CompactRelayActivationConfig, InboundAdmissionDecision,
    InboundAdmissionPolicy, InboundAdmissionRejectionReason, InboundAdmissionRequest,
    InboundAdmissionSlotClass, InboundPermissionDecision, InventoryList, LearnedAddressDecision,
    LearnedAddressEntry, LocalAdvertisementDecision, LocalPeerConfig, MisbehaviorDecision,
    MisbehaviorKind, MisbehaviorResponse, NetworkError, PHASE92_LEARNED_ADDR_BATCH_LIMIT,
    PHASE94_MAX_HEADER_LOCATOR_HASHES, PHASE94_MAX_INBOUND_REQUEST_INVENTORY_ITEMS,
    PHASE94_MAX_INBOUND_TX_REQUESTS_PER_PEER, PHASE101_GETDATA_TX_INTERVAL_SECONDS,
    ParsedPeerPermissionClass, PeerAddressBoundaryDecision, PeerAddressBoundaryEvidence,
    PeerBanEntry, PeerConnectionClass, PeerPermissionClassRegistry, RelayActivationConfig,
    RoutabilityClass, ServiceFlags, WireNetworkMessage,
};

use crate::{
    ManagedAddressBoundaryInfo, ManagedPeerNetwork, MemoryChainstateStore,
    network::BlockConnectDisposition,
};
use open_bitcoin_core::primitives::Txid;

mod admission_bridge_cases;
mod announcement_transport_cases;
mod compact_cleanup_cases;
mod compact_misbehavior_cases;
mod compact_receive_cases;
mod compact_timeout_cases;
mod lifecycle_projection_target_cases;
mod mempool_lifecycle_cases;
mod package_bridge_cases;
mod recovery_cases;
mod relay_fanout_cases;
mod relay_local_submission_cases;
mod relay_serving_cases;

const EASY_BITS: u32 = 0x207f_ffff;

fn script(bytes: &[u8]) -> ScriptBuf {
    ScriptBuf::from_bytes(bytes.to_vec()).expect("valid script")
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
            script_pubkey: p2sh_script(),
        }],
        lock_time: 0,
    }
}

fn spend_transaction(
    previous_txid: open_bitcoin_core::primitives::Txid,
    value: i64,
) -> Transaction {
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
            value: Amount::from_sats(value).expect("valid amount"),
            script_pubkey: p2sh_script(),
        }],
        lock_time: 0,
    }
}

fn script_heavy_spend_transaction(
    previous_txid: open_bitcoin_core::primitives::Txid,
    value: i64,
) -> Transaction {
    let mut datacarrier = vec![0x6a, 0x4c, 80];
    datacarrier.extend(std::iter::repeat_n(0xab_u8, 80));
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
        outputs: vec![
            TransactionOutput {
                value: Amount::from_sats(value).expect("valid amount"),
                script_pubkey: p2sh_script(),
            },
            TransactionOutput {
                value: Amount::from_sats(0).expect("zero amount"),
                script_pubkey: script(&datacarrier),
            },
        ],
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

fn build_block(previous_block_hash: BlockHash, height: u32, value: i64) -> Block {
    let transactions = vec![coinbase_transaction(height, value)];
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

fn compact_payload_with_missing_short_id(previous_block_hash: BlockHash) -> CompactBlockPayload {
    let template = build_block(previous_block_hash, 2, 500_000_000);
    let missing = spend_transaction(Txid::from_byte_array([0x22_u8; 32]), 10_000);
    let wtxid = transaction_wtxid(&missing).expect("wtxid");
    let selector =
        open_bitcoin_codec::short_id_selector_from_header_and_nonce(&template.header, 42);
    let short_id = open_bitcoin_core::consensus::compact_short_id_for_wtxid(selector, &wtxid);

    CompactBlockPayload {
        header: template.header,
        nonce: 42,
        short_ids: vec![short_id],
        prefilled_transactions: vec![PrefilledTransaction {
            index_delta: 0,
            transaction: template.transactions[0].clone(),
        }],
    }
}

fn local_config(nonce: u64) -> LocalPeerConfig {
    LocalPeerConfig {
        magic: NetworkMagic::MAINNET,
        services: ServiceFlags::NETWORK | ServiceFlags::WITNESS,
        address: NetworkAddress {
            services: 0,
            address_bytes: [0_u8; 16],
            port: 8333,
        },
        nonce,
        relay: true,
        user_agent: "/open-bitcoin:test/".to_string(),
    }
}

fn peer_policy_entry(
    scope: BanScope,
    expires_at_unix_seconds: i64,
    source: &'static str,
) -> PeerBanEntry {
    PeerBanEntry {
        scope,
        reason: BanReason::Manual,
        created_at_unix_seconds: 100,
        expires_at_unix_seconds,
        source,
    }
}

fn public_ipv4_network_address(
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    port: u16,
    services: ServiceFlags,
) -> NetworkAddress {
    let mut address_bytes = [0_u8; 16];
    address_bytes[10] = 0xff;
    address_bytes[11] = 0xff;
    address_bytes[12] = a;
    address_bytes[13] = b;
    address_bytes[14] = c;
    address_bytes[15] = d;
    NetworkAddress {
        services: services.bits(),
        address_bytes,
        port,
    }
}

fn local_advertisement_candidate(address: NetworkAddress) -> LocalAdvertisementDecision {
    LocalAdvertisementDecision {
        label: AddressDecisionLabel::AdvertiseCandidate,
        reason: AddressDecisionReason::PolicyAccepted,
        source: AddressSourceKind::LocalListener,
        network_kind: AddressNetworkKind::Ipv4,
        routability: RoutabilityClass::PubliclyRoutable,
        services_bits: address.services,
        port: address.port,
        maybe_wire_address: Some(address),
    }
}

fn suppressed_local_advertisement(port: u16, services: ServiceFlags) -> LocalAdvertisementDecision {
    LocalAdvertisementDecision {
        label: AddressDecisionLabel::AdvertiseSuppressed,
        reason: AddressDecisionReason::PermissionPolicyDenied,
        source: AddressSourceKind::LocalListener,
        network_kind: AddressNetworkKind::Ipv4,
        routability: RoutabilityClass::PubliclyRoutable,
        services_bits: services.bits(),
        port,
        maybe_wire_address: None,
    }
}

fn learned_address_entry(address: NetworkAddress) -> LearnedAddressEntry {
    LearnedAddressEntry {
        network_kind: AddressNetworkKind::Ipv4,
        source: AddressSourceKind::InboundAddr,
        first_seen_unix_seconds: 100,
        last_seen_unix_seconds: 100,
        services_bits: address.services,
        routability: RoutabilityClass::PubliclyRoutable,
        persistence_eligible: true,
        address,
    }
}

fn learned_address_rejection(port: u16, services: ServiceFlags) -> LearnedAddressDecision {
    LearnedAddressDecision {
        label: AddressDecisionLabel::LearnedRejected,
        reason: AddressDecisionReason::DuplicateAddress,
        source: AddressSourceKind::InboundAddr,
        network_kind: AddressNetworkKind::Ipv4,
        routability: RoutabilityClass::PubliclyRoutable,
        services_bits: services.bits(),
        port,
        persistence_eligible: false,
        maybe_entry: None,
    }
}

fn verify_flags() -> ScriptVerifyFlags {
    ScriptVerifyFlags::P2SH
        | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
        | ScriptVerifyFlags::CHECKSEQUENCEVERIFY
}

fn consensus_params() -> ConsensusParams {
    ConsensusParams {
        coinbase_maturity: 1,
        ..ConsensusParams::default()
    }
}

fn inbound_request(
    peer_id: u64,
    remote_endpoint: &str,
    slot_class: InboundAdmissionSlotClass,
) -> InboundAdmissionRequest {
    let permission_decision = match slot_class {
        InboundAdmissionSlotClass::Ordinary => InboundPermissionDecision::ordinary(),
        InboundAdmissionSlotClass::Reserved => protected_permission_decision(),
    };
    InboundAdmissionRequest::from_permission_decision(peer_id, remote_endpoint, permission_decision)
}

fn permissioned_inbound_request(
    peer_id: u64,
    remote_endpoint: &str,
    permissions: &[&str],
) -> InboundAdmissionRequest {
    InboundAdmissionRequest::from_permission_decision(
        peer_id,
        remote_endpoint,
        permission_decision(permissions),
    )
}

fn permission_decision(permissions: &[&str]) -> InboundPermissionDecision {
    let class = ParsedPeerPermissionClass::parse("test-class", ["203.0.113.7"], permissions)
        .expect("permission class");
    let address: IpAddr = "203.0.113.7".parse().expect("test address");
    PeerPermissionClassRegistry::new([class]).resolve_inbound(address)
}

fn protected_permission_decision() -> InboundPermissionDecision {
    permission_decision(&["in", "noban", "forceinbound"])
}

fn deliver(
    sender: &ManagedPeerNetwork<MemoryChainstateStore>,
    receiver: &mut ManagedPeerNetwork<MemoryChainstateStore>,
    peer_id: u64,
    messages: Vec<WireNetworkMessage>,
    timestamp: i64,
) -> Vec<WireNetworkMessage> {
    let mut outbound = Vec::new();
    let encoded = sender.encode_messages(&messages).expect("encode");
    for bytes in encoded {
        outbound.extend(
            receiver
                .receive_wire_message(
                    peer_id,
                    &bytes,
                    timestamp,
                    verify_flags(),
                    consensus_params(),
                )
                .expect("receive")
                .outbound,
        );
    }
    outbound
}

fn hash_from_index(index: usize) -> Hash32 {
    let mut bytes = [0_u8; 32];
    bytes[..8].copy_from_slice(&(index as u64).to_le_bytes());
    Hash32::from_byte_array(bytes)
}

fn transaction_inventory(count: usize) -> InventoryList {
    InventoryList::new(
        (0..count)
            .map(|index| InventoryVector {
                inventory_type: InventoryType::Transaction,
                object_hash: hash_from_index(index),
            })
            .collect(),
    )
}

fn block_inventory(count: usize) -> InventoryList {
    InventoryList::new(
        (0..count)
            .map(|index| InventoryVector {
                inventory_type: InventoryType::Block,
                object_hash: hash_from_index(index),
            })
            .collect(),
    )
}

fn block_getdata_inventory(block: &Block) -> InventoryList {
    InventoryList::new(vec![InventoryVector {
        inventory_type: InventoryType::Block,
        object_hash: block_hash(&block.header).into(),
    }])
}

fn transaction_relay_inventory(transaction: &Transaction) -> InventoryList {
    InventoryList::new(vec![InventoryVector {
        inventory_type: InventoryType::Transaction,
        object_hash: transaction_txid(transaction).expect("txid").into(),
    }])
}

fn witness_transaction_relay_inventory(transaction: &Transaction) -> InventoryList {
    InventoryList::new(vec![InventoryVector {
        inventory_type: InventoryType::WitnessTransaction,
        object_hash: transaction_wtxid(transaction).expect("wtxid").into(),
    }])
}

fn assert_getdata(messages: &[WireNetworkMessage], expected_inventory: InventoryList) {
    assert_eq!(messages, &[WireNetworkMessage::GetData(expected_inventory)],);
}

fn assert_targeted_getdata(
    messages: &[(u64, WireNetworkMessage)],
    expected_peer_id: u64,
    expected_inventory: InventoryList,
) {
    assert_eq!(
        messages,
        &[(
            expected_peer_id,
            WireNetworkMessage::GetData(expected_inventory)
        )],
    );
}

fn relay_enabled_managed_network(nonce: u64) -> ManagedPeerNetwork<MemoryChainstateStore> {
    ManagedPeerNetwork::new_with_relay_activation(
        MemoryChainstateStore::default(),
        local_config(nonce),
        PolicyConfig::default(),
        RelayActivationConfig { enabled: true },
        true,
    )
}

fn block_serving_enabled_managed_network(nonce: u64) -> ManagedPeerNetwork<MemoryChainstateStore> {
    ManagedPeerNetwork::new_with_block_relay_activation(
        MemoryChainstateStore::default(),
        local_config(nonce),
        PolicyConfig::default(),
        RelayActivationConfig { enabled: true },
        BlockRelayActivationPolicy {
            block_serving: BlockServingActivationConfig { enabled: true },
            compact_relay: Default::default(),
        },
        true,
    )
}

fn compact_relay_enabled_managed_network(nonce: u64) -> ManagedPeerNetwork<MemoryChainstateStore> {
    ManagedPeerNetwork::new_with_block_relay_activation(
        MemoryChainstateStore::default(),
        local_config(nonce),
        PolicyConfig::default(),
        RelayActivationConfig { enabled: true },
        BlockRelayActivationPolicy {
            block_serving: BlockServingActivationConfig { enabled: true },
            compact_relay: CompactRelayActivationConfig { enabled: true },
        },
        true,
    )
}

fn phase126_compact_announcement_decision() -> CompactAnnouncementDecision {
    CompactAnnouncementDecision {
        action: CompactAnnouncementAction::AnnounceCompactBlock,
        reason: CompactAnnouncementReason::CompactAnnounced,
        eligibility: CompactAnnouncementEligibility::Eligible,
    }
}

mod block_connect_disposition;

mod address_and_peer_policy;
mod block_serving;
mod compact_announcement;
mod inbound_capacity;
mod inbound_observation;
mod resource_governance;
mod runtime_projection;
mod transaction_relay;
