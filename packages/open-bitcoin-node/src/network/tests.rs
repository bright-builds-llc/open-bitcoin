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
mod mempool_lifecycle_cases;
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
    let mut datacarrier = vec![0x6a, 80];
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

#[test]
fn phase126_compact_announcement_uses_injected_nonce_once() {
    // Arrange
    let peer_id = 126_101;
    let mut network = compact_relay_enabled_managed_network(peer_id);
    network
        .connect_outbound_peer(peer_id, 1)
        .expect("connect outbound");
    let block = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 500_000_000);
    let nonce_calls = Cell::new(0);
    let expected_nonce = 0x0123_4567_89ab_cdef;

    // Act
    let maybe_message = network
        .announce_block_with_nonce(
            peer_id,
            &block,
            phase126_compact_announcement_decision(),
            || {
                nonce_calls.set(nonce_calls.get() + 1);
                Ok::<u64, ()>(expected_nonce)
            },
        )
        .expect("compact announcement");

    // Assert
    let Some(WireNetworkMessage::CompactBlock(payload)) = maybe_message else {
        panic!("expected compact block announcement");
    };
    assert_eq!(payload.nonce, expected_nonce);
    assert_eq!(nonce_calls.get(), 1);
}

#[test]
fn phase126_compact_announcement_entropy_failure_uses_safe_fallback_without_compact_evidence() {
    // Arrange
    let peer_id = 126_102;
    let mut network = compact_relay_enabled_managed_network(peer_id);
    network
        .connect_outbound_peer(peer_id, 1)
        .expect("connect outbound");
    let block = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 500_000_000);

    // Act
    let maybe_message = network
        .announce_block_with_nonce(
            peer_id,
            &block,
            phase126_compact_announcement_decision(),
            || Err::<u64, ()>(()),
        )
        .expect("safe fallback");

    // Assert
    assert!(matches!(maybe_message, Some(WireNetworkMessage::Inv(_))));
    assert!(
        network
            .peer_manager()
            .peer_state(peer_id)
            .expect("peer")
            .compact_announcements
            .is_empty()
    );
    let encoded =
        serde_json::to_value(network.block_relay_evidence_status()).expect("block relay evidence");
    assert_eq!(
        encoded["announcement"]["value"]["compact_announced_count"],
        0
    );
    assert_eq!(
        encoded["announcement"]["value"]["compact_inventory_fallback_count"],
        0
    );
}

#[test]
fn phase126_non_compact_announcement_actions_do_not_request_entropy() {
    // Arrange
    let peer_id = 126_103;
    let mut network = compact_relay_enabled_managed_network(peer_id);
    network
        .connect_outbound_peer(peer_id, 1)
        .expect("connect outbound");
    let block = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 500_000_000);
    let cases = [
        (
            CompactAnnouncementDecision {
                action: CompactAnnouncementAction::AnnounceHeaders,
                reason: CompactAnnouncementReason::CompactHighBandwidthNotRequested,
                eligibility: CompactAnnouncementEligibility::Ineligible {
                    reason: CompactAnnouncementEligibilityReason::HighBandwidthNotRequested,
                },
            },
            "headers",
        ),
        (
            CompactAnnouncementDecision {
                action: CompactAnnouncementAction::AnnounceInventory,
                reason: CompactAnnouncementReason::CompactRelayDisabled,
                eligibility: CompactAnnouncementEligibility::Ineligible {
                    reason: CompactAnnouncementEligibilityReason::LocalActivationDisabled,
                },
            },
            "inventory",
        ),
        (
            CompactAnnouncementDecision {
                action: CompactAnnouncementAction::Suppress,
                reason: CompactAnnouncementReason::CompactBlockUnavailable,
                eligibility: CompactAnnouncementEligibility::Ineligible {
                    reason: CompactAnnouncementEligibilityReason::BlockUnavailable,
                },
            },
            "suppression",
        ),
    ];

    // Act
    let outcomes: Vec<_> = cases
        .into_iter()
        .map(|(decision, label)| {
            let nonce_calls = Cell::new(0);
            let maybe_message = network
                .announce_block_with_nonce(peer_id, &block, decision, || {
                    nonce_calls.set(nonce_calls.get() + 1);
                    Ok::<u64, ()>(0)
                })
                .expect("non-compact announcement");
            (label, nonce_calls.get(), maybe_message)
        })
        .collect();

    // Assert
    assert!(matches!(
        outcomes.as_slice(),
        [
            ("headers", 0, Some(WireNetworkMessage::Headers(_))),
            ("inventory", 0, Some(WireNetworkMessage::Inv(_))),
            ("suppression", 0, None),
        ]
    ));
}

#[test]
fn phase116_block_relay_evidence_status_defaults_to_unavailable_until_observed() {
    let network = compact_relay_enabled_managed_network(116_001);

    let status = network.block_relay_evidence_status();
    let encoded = serde_json::to_value(status).expect("block relay evidence");

    assert_eq!(
        encoded["block_serving"]["activation"]["state"],
        "unavailable"
    );
}

#[test]
fn phase123_block_acknowledgement_increments_private_served_count() {
    // Arrange
    let mut network = compact_relay_enabled_managed_network(123_201);
    let block = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 500_000_000);
    let message = WireNetworkMessage::Block(block);

    // Act
    network.acknowledge_wire_message_written(&message);
    let snapshot: crate::network::BlockRelayRuntimeEvidenceSnapshot =
        network.block_relay_runtime_evidence_snapshot();

    // Assert
    assert_eq!(network.block_served_write_count(), 1);
    assert_eq!(snapshot.served_count, 1);
    assert!(matches!(
        snapshot.status.block_serving.activation,
        crate::status::FieldAvailability::Available(_)
    ));
}

#[test]
fn phase123_non_block_acknowledgement_does_not_increment_private_served_count() {
    // Arrange
    let mut network = compact_relay_enabled_managed_network(123_202);
    let message = WireNetworkMessage::Verack;

    // Act
    network.acknowledge_wire_message_written(&message);

    // Assert
    assert_eq!(network.block_served_write_count(), 0);
}

#[test]
fn phase123_public_block_relay_status_omits_runtime_served_count() {
    // Arrange
    let mut network = compact_relay_enabled_managed_network(123_203);
    let block = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 500_000_000);
    let message = WireNetworkMessage::Block(block);
    network.acknowledge_wire_message_written(&message);

    // Act
    let encoded = serde_json::to_value(network.block_relay_evidence_status())
        .expect("serialize public block-relay status");

    // Assert
    assert!(!encoded.to_string().contains("served_count"));
    assert_eq!(
        encoded["block_serving"]["eligibility"]["value"]["eligible_peer_count"],
        0
    );
    assert_eq!(
        encoded["block_serving"]["status"]["value"]["validated_count"],
        0
    );
}

#[test]
fn phase116_block_relay_evidence_projects_negotiation_serving_download_and_cleanup() {
    let mut network = compact_relay_enabled_managed_network(116_002);
    let peer_id = 116_002;
    network
        .connect_outbound_peer(peer_id, 1)
        .expect("connect outbound");
    network
        .receive_message(
            peer_id,
            WireNetworkMessage::Version(open_bitcoin_network::VersionMessage::default()),
            1,
            verify_flags(),
            consensus_params(),
        )
        .expect("version");
    network
        .receive_message(
            peer_id,
            WireNetworkMessage::Verack,
            1,
            verify_flags(),
            consensus_params(),
        )
        .expect("verack");
    network
        .receive_message(
            peer_id,
            WireNetworkMessage::SendCompact(SendCompactMessage {
                announce: true,
                version: open_bitcoin_codec::BIP152_COMPACT_BLOCKS_VERSION,
            }),
            2,
            verify_flags(),
            consensus_params(),
        )
        .expect("sendcmpct");

    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 500_000_000);
    network
        .connect_local_block(&genesis, verify_flags(), consensus_params())
        .expect("connect genesis");
    let maybe_announce = network
        .announce_block(peer_id, &genesis)
        .expect("announce block");
    assert!(
        matches!(
            maybe_announce.as_ref(),
            Some(WireNetworkMessage::CompactBlock(_))
        ),
        "HB-eligible announce must emit CompactBlock, got {maybe_announce:?}"
    );
    let emission = crate::network::PeerEmission::new(
        peer_id,
        maybe_announce.expect("compact announcement"),
        block_hash(&genesis.header),
    )
    .expect("supported compact emission");
    let (_, _, receipt) = emission.into_parts();
    network
        .complete_peer_emission(receipt)
        .expect("complete compact write");

    let served = network
        .receive_message(
            peer_id,
            WireNetworkMessage::GetData(block_getdata_inventory(&genesis)),
            3,
            verify_flags(),
            consensus_params(),
        )
        .expect("serve block")
        .outbound;
    assert!(
        served
            .iter()
            .any(|message| matches!(message, WireNetworkMessage::Block(_)))
    );

    let actions = network
        .peer_manager
        .handle_compact_block_download(
            peer_id,
            compact_payload_with_missing_short_id(block_hash(&genesis.header)),
            open_bitcoin_network::CompactBlockReceiveFacts {
                candidates: &[],
                extra: &[],
            },
            4,
        )
        .expect("compact block");
    network.note_block_relay_observed();
    network.record_compact_download_evidence(&actions);
    assert!(actions.iter().any(|action| matches!(
        action,
        open_bitcoin_network::PeerAction::Send(WireNetworkMessage::GetBlockTxn(_))
    )));

    let status = network.block_relay_evidence_status();
    let encoded = serde_json::to_value(&status).expect("block relay evidence");
    assert_eq!(
        encoded["block_serving"]["activation"]["value"]["block_serving_enabled"],
        true
    );
    assert_eq!(
        encoded["negotiation"]["value"]["version2_high_bandwidth_count"],
        1
    );
    assert_eq!(
        encoded["announcement"]["value"]["compact_announced_count"],
        1
    );
    assert_eq!(
        encoded["block_serving"]["eligibility"]["value"]["eligible_peer_count"],
        1
    );
    assert_eq!(
        encoded["missing_transaction"]["value"]["compact_missing_tx_requested_count"],
        1
    );
    assert_eq!(encoded["in_flight"]["value"]["in_flight_count"], 1);
    assert_eq!(
        encoded["in_flight"]["value"]["peers_with_in_flight_count"],
        1
    );

    network.disconnect_peer(peer_id).expect("disconnect");

    let cleaned = serde_json::to_value(network.block_relay_evidence_status())
        .expect("block relay evidence after cleanup");
    assert_eq!(
        cleaned["cleanup"]["value"]["compact_download_peer_disconnect_count"],
        1
    );
}

#[test]
fn phase118_low_bandwidth_announce_does_not_increment_compact_announced_count() {
    // Arrange
    let mut network = compact_relay_enabled_managed_network(118_003);
    let peer_id = 118_003;
    network
        .connect_outbound_peer(peer_id, 1)
        .expect("connect outbound");
    network
        .receive_message(
            peer_id,
            WireNetworkMessage::Version(open_bitcoin_network::VersionMessage::default()),
            1,
            verify_flags(),
            consensus_params(),
        )
        .expect("version");
    network
        .receive_message(
            peer_id,
            WireNetworkMessage::Verack,
            1,
            verify_flags(),
            consensus_params(),
        )
        .expect("verack");
    network
        .receive_message(
            peer_id,
            WireNetworkMessage::SendCompact(SendCompactMessage {
                announce: false,
                version: open_bitcoin_codec::BIP152_COMPACT_BLOCKS_VERSION,
            }),
            2,
            verify_flags(),
            consensus_params(),
        )
        .expect("sendcmpct low-bandwidth");

    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 500_000_000);
    network
        .connect_local_block(&genesis, verify_flags(), consensus_params())
        .expect("connect genesis");

    // Act
    let maybe_announce = network
        .announce_block(peer_id, &genesis)
        .expect("announce block");

    // Assert
    assert!(
        !matches!(maybe_announce, Some(WireNetworkMessage::CompactBlock(_))),
        "low-bandwidth path must not emit CompactBlock, got {maybe_announce:?}"
    );
    assert!(
        matches!(
            maybe_announce,
            Some(WireNetworkMessage::Headers(_)) | Some(WireNetworkMessage::Inv(_))
        ),
        "low-bandwidth path should fall back to Headers or Inv, got {maybe_announce:?}"
    );
    let encoded =
        serde_json::to_value(network.block_relay_evidence_status()).expect("block relay evidence");
    assert_eq!(
        encoded["announcement"]["value"]["compact_announced_count"],
        0
    );
}

fn assert_request_cap_resource_governance(network: &ManagedPeerNetwork<MemoryChainstateStore>) {
    let info = network.resource_governance_info();
    assert_eq!(info.request_cap_events, 1);
    let latest = info
        .maybe_latest_resource_governance_decision
        .as_ref()
        .expect("latest resource-governance decision");
    assert_eq!(latest.label, "request_cap_reached");
    assert_eq!(latest.next_action, "request_cap_reached");
}

#[test]
fn phase111_disabled_block_serving_suppresses_cached_active_block() {
    // Arrange
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(111_201),
        PolicyConfig::default(),
    );
    network
        .connect_outbound_peer(111_201, 1)
        .expect("connect outbound");
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 500_000_000);
    network
        .connect_local_block(&genesis, verify_flags(), consensus_params())
        .expect("connect genesis");

    // Act
    let outbound = network
        .receive_message(
            111_201,
            WireNetworkMessage::GetData(block_getdata_inventory(&genesis)),
            2,
            verify_flags(),
            consensus_params(),
        )
        .expect("block getdata")
        .outbound;

    // Assert
    assert_eq!(
        outbound,
        vec![WireNetworkMessage::NotFound(block_getdata_inventory(
            &genesis
        ))]
    );
}

#[test]
fn phase111_block_getdata_serves_active_validated_block_when_enabled() {
    // Arrange
    let mut network = block_serving_enabled_managed_network(111_202);
    network
        .connect_outbound_peer(111_202, 1)
        .expect("connect outbound");
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 500_000_000);
    network
        .connect_local_block(&genesis, verify_flags(), consensus_params())
        .expect("connect genesis");

    // Act
    let outbound = network
        .receive_message(
            111_202,
            WireNetworkMessage::GetData(block_getdata_inventory(&genesis)),
            2,
            verify_flags(),
            consensus_params(),
        )
        .expect("block getdata")
        .outbound;

    // Assert
    assert_eq!(outbound, vec![WireNetworkMessage::Block(genesis)]);
}

#[test]
fn phase111_witness_block_getdata_preserves_witness_payload() {
    // Arrange
    let mut network = block_serving_enabled_managed_network(111_203);
    network
        .connect_outbound_peer(111_203, 1)
        .expect("connect outbound");
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 500_000_000);
    network
        .connect_local_block(&genesis, verify_flags(), consensus_params())
        .expect("connect genesis");
    let mut cached_genesis = genesis.clone();
    cached_genesis.transactions[0].inputs[0].witness =
        ScriptWitness::new(vec![vec![0x01, 0x02], vec![0x51]]);
    let genesis_hash = block_hash(&genesis.header);
    network.blocks_by_hash.insert(genesis_hash, cached_genesis);
    let inventory = InventoryList::new(vec![InventoryVector {
        inventory_type: InventoryType::WitnessBlock,
        object_hash: genesis_hash.into(),
    }]);

    // Act
    let outbound = network
        .receive_message(
            111_203,
            WireNetworkMessage::GetData(inventory),
            2,
            verify_flags(),
            consensus_params(),
        )
        .expect("witness block getdata")
        .outbound;

    // Assert
    let [WireNetworkMessage::Block(served)] = outbound.as_slice() else {
        panic!("expected served witness block, got {outbound:?}");
    };
    let encoded = encode_block(served).expect("served block should encode");
    let decoded = parse_block(&encoded).expect("served block should decode");
    assert_eq!(
        decoded.transactions[0].inputs[0].witness.stack(),
        &[vec![0x01, 0x02], vec![0x51]]
    );
}

#[test]
fn phase111_compact_block_getdata_is_suppressed_without_block_payload() {
    // Arrange
    let mut network = block_serving_enabled_managed_network(111_204);
    network
        .connect_outbound_peer(111_204, 1)
        .expect("connect outbound");
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 500_000_000);
    network
        .connect_local_block(&genesis, verify_flags(), consensus_params())
        .expect("connect genesis");
    let inventory = InventoryList::new(vec![InventoryVector {
        inventory_type: InventoryType::CompactBlock,
        object_hash: block_hash(&genesis.header).into(),
    }]);

    // Act
    let outbound = network
        .receive_message(
            111_204,
            WireNetworkMessage::GetData(inventory.clone()),
            2,
            verify_flags(),
            consensus_params(),
        )
        .expect("compact block getdata")
        .outbound;

    // Assert
    assert_eq!(outbound, vec![WireNetworkMessage::NotFound(inventory)]);
    assert!(
        !outbound
            .iter()
            .any(|message| matches!(message, WireNetworkMessage::Block(_)))
    );
}

#[test]
fn phase113_compact_getdata_remains_suppressed_after_negotiation_policy() {
    // Arrange
    let mut network = compact_relay_enabled_managed_network(113_301);
    network
        .connect_outbound_peer(113_301, 1)
        .expect("connect outbound");
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 500_000_000);
    network
        .connect_local_block(&genesis, verify_flags(), consensus_params())
        .expect("connect genesis");
    let inventory = InventoryList::new(vec![InventoryVector {
        inventory_type: InventoryType::CompactBlock,
        object_hash: block_hash(&genesis.header).into(),
    }]);

    // Act
    let outbound = network
        .receive_message(
            113_301,
            WireNetworkMessage::GetData(inventory.clone()),
            2,
            verify_flags(),
            consensus_params(),
        )
        .expect("compact block getdata")
        .outbound;

    // Assert
    assert_eq!(outbound, vec![WireNetworkMessage::NotFound(inventory)]);
    assert!(
        !outbound
            .iter()
            .any(|message| matches!(message, WireNetworkMessage::CompactBlock(_)))
    );
    assert!(!outbound.iter().any(|message| {
        matches!(
            message,
            WireNetworkMessage::GetBlockTxn(_) | WireNetworkMessage::BlockTxn(_)
        )
    }));
    assert_eq!(network.mempool_info().transaction_count, 0);
    assert_eq!(network.maybe_chain_tip().expect("tip").height, 0);
}

#[test]
fn phase111_side_chain_cached_block_is_not_served() {
    // Arrange
    let mut network = block_serving_enabled_managed_network(111_206);
    network
        .connect_outbound_peer(111_206, 1)
        .expect("connect outbound");
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 500_000_000);
    let side_block = build_block(BlockHash::from_byte_array([206_u8; 32]), 1, 500_000_000);
    network
        .connect_local_block(&genesis, verify_flags(), consensus_params())
        .expect("connect genesis");
    assert!(matches!(
        network
            .connect_stored_block(
                &side_block,
                2,
                i64::from(side_block.header.time),
                verify_flags(),
                consensus_params(),
            )
            .expect("cache side block"),
        BlockConnectDisposition::NonExtending { .. }
    ));
    let inventory = block_getdata_inventory(&side_block);

    // Act
    let outbound = network
        .receive_message(
            111_206,
            WireNetworkMessage::GetData(inventory.clone()),
            2,
            verify_flags(),
            consensus_params(),
        )
        .expect("side-chain getdata")
        .outbound;

    // Assert
    assert_eq!(outbound, vec![WireNetworkMessage::NotFound(inventory)]);
}

#[test]
fn phase111_active_chain_non_tip_missing_local_block_returns_pruned_notfound() {
    // Arrange
    let mut network = block_serving_enabled_managed_network(111_207);
    network
        .connect_outbound_peer(111_207, 1)
        .expect("connect outbound");
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 500_000_000);
    let child = build_block(block_hash(&genesis.header), 1, 500_000_000);
    network
        .connect_local_block(&genesis, verify_flags(), consensus_params())
        .expect("connect genesis");
    network
        .connect_local_block(&child, verify_flags(), consensus_params())
        .expect("connect child");
    let inventory = block_getdata_inventory(&genesis);
    network.blocks_by_hash.remove(&block_hash(&genesis.header));

    // Act
    let outbound = network
        .receive_message(
            111_207,
            WireNetworkMessage::GetData(inventory.clone()),
            2,
            verify_flags(),
            consensus_params(),
        )
        .expect("pruned non-tip getdata")
        .outbound;

    // Assert
    assert_eq!(outbound, vec![WireNetworkMessage::NotFound(inventory)]);
}

#[test]
fn phase111_active_tip_missing_local_block_returns_unavailable_notfound() {
    // Arrange
    let mut network = block_serving_enabled_managed_network(111_208);
    network
        .connect_outbound_peer(111_208, 1)
        .expect("connect outbound");
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 500_000_000);
    network
        .connect_local_block(&genesis, verify_flags(), consensus_params())
        .expect("connect genesis");
    let inventory = block_getdata_inventory(&genesis);
    network.blocks_by_hash.remove(&block_hash(&genesis.header));

    // Act
    let outbound = network
        .receive_message(
            111_208,
            WireNetworkMessage::GetData(inventory.clone()),
            2,
            verify_flags(),
            consensus_params(),
        )
        .expect("unavailable tip getdata")
        .outbound;

    // Assert
    assert_eq!(outbound, vec![WireNetworkMessage::NotFound(inventory)]);
}

#[test]
fn phase111_cached_old_block_outside_active_chain_is_not_archive_served() {
    // Arrange
    let mut network = block_serving_enabled_managed_network(111_209);
    network
        .connect_outbound_peer(111_209, 1)
        .expect("connect outbound");
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 500_000_000);
    let old_disconnected = build_block(BlockHash::from_byte_array([209_u8; 32]), 1, 500_000_000);
    assert!(matches!(
        network
            .connect_stored_block(
                &old_disconnected,
                1,
                i64::from(old_disconnected.header.time),
                verify_flags(),
                consensus_params(),
            )
            .expect("cache disconnected block"),
        BlockConnectDisposition::Disconnected { .. }
    ));
    network
        .connect_local_block(&genesis, verify_flags(), consensus_params())
        .expect("connect genesis");
    let inventory = block_getdata_inventory(&old_disconnected);

    // Act
    let outbound = network
        .receive_message(
            111_209,
            WireNetworkMessage::GetData(inventory.clone()),
            2,
            verify_flags(),
            consensus_params(),
        )
        .expect("old disconnected block getdata")
        .outbound;

    // Assert
    assert_eq!(outbound, vec![WireNetworkMessage::NotFound(inventory)]);
}

#[test]
fn phase111_managed_getdata_over_request_cap_disconnects_without_block_payload() {
    // Arrange
    let mut network = block_serving_enabled_managed_network(111_210);
    network.add_inbound_peer(111_210).expect("inbound peer");

    // Act
    let error = network
        .receive_message(
            111_210,
            WireNetworkMessage::GetData(block_inventory(
                PHASE94_MAX_INBOUND_REQUEST_INVENTORY_ITEMS + 1,
            )),
            1,
            verify_flags(),
            consensus_params(),
        )
        .expect_err("over-cap getdata should disconnect");

    // Assert
    assert!(matches!(
        error,
        crate::network::ManagedNetworkError::Network(NetworkError::ResourceLimit(111_210))
    ));
    assert_eq!(network.network_info().inbound_peers, 0);
    assert_request_cap_resource_governance(&network);
}

#[test]
fn phase111_permissioned_block_getdata_still_hits_request_cap() {
    // Arrange
    let mut network = block_serving_enabled_managed_network(111_211);
    let decision = network.admit_inbound_peer(permissioned_inbound_request(
        111_211,
        "203.0.113.7:18444",
        &["in", "download"],
    ));
    assert!(matches!(decision, InboundAdmissionDecision::Admit(_)));

    // Act
    let error = network
        .receive_message(
            111_211,
            WireNetworkMessage::GetData(block_inventory(
                PHASE94_MAX_INBOUND_REQUEST_INVENTORY_ITEMS + 1,
            )),
            1,
            verify_flags(),
            consensus_params(),
        )
        .expect_err("permissioned over-cap getdata should disconnect");

    // Assert
    assert!(matches!(
        error,
        crate::network::ManagedNetworkError::Network(NetworkError::ResourceLimit(111_211))
    ));
    assert_eq!(network.network_info().inbound_peers, 0);
    assert_request_cap_resource_governance(&network);
}

#[test]
fn phase111_mixed_getdata_preserves_transaction_relay_serving() {
    // Arrange
    let mut network = block_serving_enabled_managed_network(111_205);
    network
        .connect_outbound_peer(111_205, 1)
        .expect("connect outbound");
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 500_000_000);
    network
        .connect_local_block(&genesis, verify_flags(), consensus_params())
        .expect("connect genesis");
    let transaction = Transaction::default();
    network
        .store_transaction(transaction.clone())
        .expect("store transaction");
    let inventory = InventoryList::new(vec![
        InventoryVector {
            inventory_type: InventoryType::Block,
            object_hash: block_hash(&genesis.header).into(),
        },
        InventoryVector {
            inventory_type: InventoryType::Transaction,
            object_hash: transaction_txid(&transaction).expect("txid").into(),
        },
    ]);

    // Act
    let outbound = network
        .receive_message(
            111_205,
            WireNetworkMessage::GetData(inventory),
            2,
            verify_flags(),
            consensus_params(),
        )
        .expect("mixed getdata")
        .outbound;

    // Assert
    assert_eq!(
        outbound,
        vec![
            WireNetworkMessage::Block(genesis),
            WireNetworkMessage::Tx(transaction),
        ]
    );
}

#[test]
fn managed_network_transaction_relay_default_constructor_suppresses_getdata() {
    // Arrange
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(501),
        PolicyConfig::default(),
    );
    let transaction = Transaction::default();
    let inventory = transaction_relay_inventory(&transaction);
    network
        .connect_outbound_peer(501, 1)
        .expect("outbound peer");

    // Act
    let network_info = network.network_info();
    let outbound = network
        .receive_message(
            501,
            WireNetworkMessage::Inv(inventory),
            2,
            verify_flags(),
            consensus_params(),
        )
        .expect("default-off transaction inventory")
        .outbound;

    // Assert
    assert!(!network_info.relay);
    assert!(outbound.is_empty());
}

#[test]
fn managed_network_transaction_relay_enabled_outbound_translates_request_action_to_getdata() {
    // Arrange
    let mut network = relay_enabled_managed_network(502);
    network
        .connect_outbound_peer(502, 1)
        .expect("txid outbound peer");
    network
        .connect_outbound_peer(503, 1)
        .expect("wtxid outbound peer");
    network
        .receive_message(
            503,
            WireNetworkMessage::WtxidRelay,
            1,
            verify_flags(),
            consensus_params(),
        )
        .expect("wtxidrelay");
    let transaction = Transaction::default();
    let txid_inventory = transaction_relay_inventory(&transaction);
    let wtxid_inventory = witness_transaction_relay_inventory(&transaction);

    // Act
    let network_info = network.network_info();
    let txid_outbound = network
        .receive_message(
            502,
            WireNetworkMessage::Inv(txid_inventory.clone()),
            2,
            verify_flags(),
            consensus_params(),
        )
        .expect("txid inventory")
        .outbound;
    let wtxid_outbound = network
        .receive_message(
            503,
            WireNetworkMessage::Inv(wtxid_inventory.clone()),
            3,
            verify_flags(),
            consensus_params(),
        )
        .expect("wtxid inventory")
        .outbound;

    // Assert
    assert!(network_info.relay);
    assert_getdata(&txid_outbound, txid_inventory);
    assert_getdata(&wtxid_outbound, wtxid_inventory);
}

#[test]
fn managed_network_transaction_relay_enabled_ordinary_inbound_suppresses_getdata() {
    // Arrange
    let mut network = relay_enabled_managed_network(504);
    network
        .add_inbound_peer(504)
        .expect("ordinary inbound peer");
    let inventory = transaction_relay_inventory(&Transaction::default());

    // Act
    let outbound = network
        .receive_message(
            504,
            WireNetworkMessage::Inv(inventory),
            1,
            verify_flags(),
            consensus_params(),
        )
        .expect("ordinary inbound inventory")
        .outbound;

    // Assert
    assert!(outbound.is_empty());
}

#[test]
fn managed_network_transaction_relay_enabled_protected_only_inbound_suppresses_getdata() {
    // Arrange
    let mut network = relay_enabled_managed_network(505);
    network.set_inbound_admission_policy(InboundAdmissionPolicy::new(2, 1));
    let decision = network.admit_inbound_peer(inbound_request(
        505,
        "127.0.0.1:18444",
        InboundAdmissionSlotClass::Reserved,
    ));
    assert!(matches!(decision, InboundAdmissionDecision::Admit(_)));
    let inventory = transaction_relay_inventory(&Transaction::default());

    // Act
    let outbound = network
        .receive_message(
            505,
            WireNetworkMessage::Inv(inventory),
            1,
            verify_flags(),
            consensus_params(),
        )
        .expect("protected-only inbound inventory")
        .outbound;

    // Assert
    assert!(outbound.is_empty());
}

#[test]
fn managed_network_transaction_relay_duplicate_suppression_emits_no_extra_getdata() {
    // Arrange
    let mut network = relay_enabled_managed_network(506);
    network.connect_outbound_peer(506, 1).expect("first peer");
    network
        .connect_outbound_peer(507, 1)
        .expect("duplicate peer");
    let inventory = transaction_relay_inventory(&Transaction::default());

    // Act
    let first_outbound = network
        .receive_message(
            506,
            WireNetworkMessage::Inv(inventory.clone()),
            1,
            verify_flags(),
            consensus_params(),
        )
        .expect("first inventory")
        .outbound;
    let duplicate_outbound = network
        .receive_message(
            507,
            WireNetworkMessage::Inv(inventory.clone()),
            2,
            verify_flags(),
            consensus_params(),
        )
        .expect("duplicate inventory")
        .outbound;

    // Assert
    assert_getdata(&first_outbound, inventory);
    assert!(duplicate_outbound.is_empty());
}

#[test]
fn managed_network_transaction_relay_timeout_fallback_returns_getdata_for_alternate_peer() {
    // Arrange
    let mut network = relay_enabled_managed_network(508);
    network.connect_outbound_peer(508, 1).expect("first peer");
    network
        .connect_outbound_peer(509, 1)
        .expect("fallback peer");
    let inventory = transaction_relay_inventory(&Transaction::default());
    network
        .receive_message(
            508,
            WireNetworkMessage::Inv(inventory.clone()),
            1,
            verify_flags(),
            consensus_params(),
        )
        .expect("first inventory");
    network
        .receive_message(
            509,
            WireNetworkMessage::Inv(inventory.clone()),
            2,
            verify_flags(),
            consensus_params(),
        )
        .expect("fallback inventory");

    // Act
    let fallback_messages = network
        .expire_transaction_requests(1 + PHASE101_GETDATA_TX_INTERVAL_SECONDS)
        .expect("expire requests");

    // Assert
    assert_targeted_getdata(&fallback_messages, 509, inventory);
}

#[test]
fn managed_network_transaction_relay_notfound_fallback_returns_getdata_for_alternate_peer() {
    // Arrange
    let mut network = relay_enabled_managed_network(510);
    network.connect_outbound_peer(510, 1).expect("first peer");
    network
        .connect_outbound_peer(511, 1)
        .expect("fallback peer");
    let inventory = transaction_relay_inventory(&Transaction::default());
    network
        .receive_message(
            510,
            WireNetworkMessage::Inv(inventory.clone()),
            10,
            verify_flags(),
            consensus_params(),
        )
        .expect("first inventory");
    network
        .receive_message(
            511,
            WireNetworkMessage::Inv(inventory.clone()),
            11,
            verify_flags(),
            consensus_params(),
        )
        .expect("fallback inventory");

    // Act
    let result = network
        .receive_sync_message(
            510,
            WireNetworkMessage::NotFound(inventory.clone()),
            12,
            verify_flags(),
            consensus_params(),
        )
        .expect("notfound");

    // Assert
    assert!(result.outbound.is_empty());
    assert_targeted_getdata(&result.targeted_outbound, 511, inventory);
}

#[test]
fn managed_network_transaction_relay_disconnect_fallback_returns_getdata_for_alternate_peer() {
    // Arrange
    let mut network = relay_enabled_managed_network(512);
    network.connect_outbound_peer(512, 1).expect("first peer");
    network
        .connect_outbound_peer(513, 1)
        .expect("fallback peer");
    let inventory = transaction_relay_inventory(&Transaction::default());
    network
        .receive_message(
            512,
            WireNetworkMessage::Inv(inventory.clone()),
            20,
            verify_flags(),
            consensus_params(),
        )
        .expect("first inventory");
    network
        .receive_message(
            513,
            WireNetworkMessage::Inv(inventory.clone()),
            21,
            verify_flags(),
            consensus_params(),
        )
        .expect("fallback inventory");

    // Act
    let fallback_messages = network
        .disconnect_peer_with_transaction_cleanup(512, 22)
        .expect("disconnect cleanup");

    // Assert
    assert_targeted_getdata(&fallback_messages, 513, inventory);
}

#[test]
fn managed_inbound_admission_increments_inbound_counts() {
    // Arrange
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(201),
        PolicyConfig::default(),
    );
    network.set_inbound_admission_policy(InboundAdmissionPolicy::new(2, 0));

    // Act
    let decision = network.admit_inbound_peer(inbound_request(
        201,
        "127.0.0.1:18444",
        InboundAdmissionSlotClass::Ordinary,
    ));

    // Assert
    assert!(matches!(decision, InboundAdmissionDecision::Admit(_)));
    let info = network.network_info();
    assert_eq!(info.connected_peers, 1);
    assert_eq!(info.inbound_peers, 1);
    assert_eq!(info.outbound_peers, 0);
    let admission = network.inbound_admission_info();
    assert_eq!(admission.admitted_inbound_peers, 1);
    assert_eq!(admission.ordinary_inbound_admits, 1);
    assert_eq!(admission.permissioned_inbound_admits, 0);
    assert_eq!(admission.protected_inbound_admits, 0);
    assert_eq!(admission.active_permission_effect_observations, 0);
    assert_eq!(admission.inactive_permission_effect_observations, 0);
    assert_eq!(admission.rejected_inbound_peers, 0);
}

#[test]
fn permissioned_inbound_admission_counts_effects_without_reserved_capacity() {
    // Arrange
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(211),
        PolicyConfig::default(),
    );
    network.set_inbound_admission_policy(InboundAdmissionPolicy::new(2, 1));

    // Act
    let decision = network.admit_inbound_peer(permissioned_inbound_request(
        211,
        "127.0.0.1:18446",
        &["in", "download", "addr", "relay", "mempool"],
    ));

    // Assert
    let InboundAdmissionDecision::Admit(record) = decision else {
        panic!("expected permissioned inbound admission");
    };
    assert_eq!(
        record.connection_class,
        PeerConnectionClass::PermissionedInbound,
    );
    assert_eq!(record.slot_class, InboundAdmissionSlotClass::Ordinary);
    let admission = network.inbound_admission_info();
    assert_eq!(admission.admitted_inbound_peers, 1);
    assert_eq!(admission.ordinary_inbound_admits, 0);
    assert_eq!(admission.permissioned_inbound_admits, 1);
    assert_eq!(admission.protected_inbound_admits, 0);
    assert_eq!(admission.reserved_inbound_admits, 0);
    assert_eq!(admission.active_permission_effect_observations, 2);
    assert_eq!(admission.inactive_permission_effect_observations, 0);
}

#[test]
fn managed_address_boundary_info_projects_peer_manager_evidence() {
    // Arrange
    let services = ServiceFlags::NETWORK | ServiceFlags::WITNESS;
    let local_address = public_ipv4_network_address(8, 8, 8, 8, 18_444, services);
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(301),
        PolicyConfig::default(),
    );
    network.set_local_address_decisions(vec![
        local_advertisement_candidate(local_address),
        suppressed_local_advertisement(18_446, services),
    ]);
    let decision = network.admit_inbound_peer(permissioned_inbound_request(
        301,
        "127.0.0.1:18444",
        &["in", "addr"],
    ));
    assert!(matches!(decision, InboundAdmissionDecision::Admit(_)));
    let served = network
        .receive_message(
            301,
            WireNetworkMessage::GetAddr,
            101,
            verify_flags(),
            consensus_params(),
        )
        .expect("first getaddr should be served")
        .outbound;
    let suppressed = network
        .receive_message(
            301,
            WireNetworkMessage::GetAddr,
            101,
            verify_flags(),
            consensus_params(),
        )
        .expect("second getaddr should be suppressed")
        .outbound;

    // Act
    let info = network.address_boundary_info();

    // Assert
    assert!(matches!(
        served.as_slice(),
        [WireNetworkMessage::Addr(addresses)] if addresses.addresses.len() == 1
    ));
    assert!(suppressed.is_empty());
    assert_eq!(info.local_advertisement_candidates.len(), 1);
    assert_eq!(
        info.local_advertisement_candidates[0].source,
        "source_local_listener"
    );
    assert_eq!(info.local_advertisement_candidates[0].network_kind, "ipv4");
    assert_eq!(
        info.local_advertisement_candidates[0].routability,
        "publicly_routable"
    );
    assert_eq!(info.local_advertisement_candidates[0].freshness, "fresh");
    assert_eq!(
        info.local_advertisement_candidates[0].services_bits,
        services.bits()
    );
    assert_eq!(info.local_advertisement_candidates[0].port, 18_444);
    assert!(!info.local_advertisement_candidates[0].persistence_eligible);
    assert_eq!(info.suppressed_advertisements.len(), 1);
    assert_eq!(
        info.suppressed_advertisements[0].label,
        "advertise_suppressed"
    );
    assert_eq!(
        info.suppressed_advertisements[0].reason,
        "permission_policy_denied"
    );
    assert_eq!(info.getaddr_responses_served, 1);
    assert_eq!(info.getaddr_requests_suppressed, 1);
    assert_eq!(info.learned_address_entries, 0);
    assert_eq!(info.learned_address_rejections, 0);
    let latest = info
        .maybe_latest_address_decision
        .expect("latest address decision");
    assert_eq!(latest.label, "getaddr_suppressed");
    assert_eq!(latest.reason, "already_served");
}

#[test]
fn managed_peer_policy_info_projects_eviction_candidate_evidence() {
    // Arrange
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(401),
        PolicyConfig::default(),
    );
    network.add_inbound_peer(401).expect("peer should be added");

    // Act
    let info = network.peer_policy_info();

    // Assert
    assert_eq!(info.eviction_candidates_evaluated, 1);
    assert_eq!(info.disconnects_requested, 1);
    assert_eq!(info.protected_no_actions, 0);
    let latest = info
        .maybe_latest_peer_policy_decision
        .expect("latest policy decision");
    assert_eq!(latest.label, "eviction_candidate_selected");
    assert_eq!(latest.source, "source_eviction_policy");
    assert!(!latest.message.contains("peer-"));
}

#[test]
fn managed_peer_policy_info_projects_protected_eviction_suppression() {
    // Arrange
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(402),
        PolicyConfig::default(),
    );
    network.set_inbound_admission_policy(InboundAdmissionPolicy::new(2, 1));
    let decision = network.admit_inbound_peer(permissioned_inbound_request(
        402,
        "127.0.0.1:18444",
        &["in", "noban", "forceinbound"],
    ));
    assert!(matches!(decision, InboundAdmissionDecision::Admit(_)));

    // Act
    let info = network.peer_policy_info();

    // Assert
    assert_eq!(info.eviction_candidates_evaluated, 1);
    assert_eq!(info.disconnects_requested, 0);
    assert_eq!(info.protected_no_actions, 1);
    let latest = info
        .maybe_latest_peer_policy_decision
        .expect("latest policy decision");
    assert_eq!(latest.label, "eviction_suppressed");
    assert_eq!(latest.reason, "no_eviction_candidate");
}

#[test]
fn managed_peer_policy_info_projects_active_runtime_bans() {
    // Arrange
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(403),
        PolicyConfig::default(),
    );
    let entry = peer_policy_entry(
        BanScope::Address(IpAddr::from([203, 0, 113, 10])),
        300,
        "manual_ban",
    );

    // Act
    network.record_peer_policy_ban(entry, 150);
    let info = network.peer_policy_info();

    // Assert
    assert_eq!(info.active_bans, 1);
    let latest = info
        .maybe_latest_peer_policy_decision
        .expect("latest policy decision");
    assert_eq!(latest.label, "ban_active");
}

#[test]
fn managed_peer_policy_info_projects_manual_unbans() {
    // Arrange
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(404),
        PolicyConfig::default(),
    );
    let scope = BanScope::Address(IpAddr::from([203, 0, 113, 11]));
    network.record_peer_policy_ban(peer_policy_entry(scope.clone(), 300, "manual_ban"), 150);

    // Act
    network.record_peer_policy_unban(&scope, 160);
    let info = network.peer_policy_info();

    // Assert
    assert_eq!(info.manual_unbans, 1);
    let latest = info
        .maybe_latest_peer_policy_decision
        .expect("latest policy decision");
    assert_eq!(latest.label, "unbanned");
}

#[test]
fn managed_peer_policy_info_projects_runtime_misbehavior() {
    // Arrange
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(405),
        PolicyConfig::default(),
    );
    let decision = MisbehaviorDecision {
        peer_label: "peer-protected".to_string(),
        kind: MisbehaviorKind::MalformedMessage,
        score: 500,
        response: MisbehaviorResponse::ProtectedNoAction,
    };

    // Act
    network.record_peer_policy_misbehavior(decision);
    let info = network.peer_policy_info();

    // Assert
    assert_eq!(info.misbehavior_observations, 1);
    assert_eq!(info.protected_no_actions, 1);
    let latest = info
        .maybe_latest_peer_policy_decision
        .expect("latest policy decision");
    assert_eq!(latest.outcome, "protected_no_action");
}

#[test]
fn managed_address_boundary_info_projects_over_cap_addr_rejections() {
    // Arrange
    let services = ServiceFlags::NETWORK | ServiceFlags::WITNESS;
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(302),
        PolicyConfig::default(),
    );
    network.add_inbound_peer(302).expect("peer should be added");
    let now_unix_seconds = 1_700_000_000;
    let addresses = (0..=PHASE92_LEARNED_ADDR_BATCH_LIMIT)
        .map(|index| AddressAnnouncement {
            time_unix_seconds: now_unix_seconds,
            address: public_ipv4_network_address(9, 9, 9, index as u8, 18_444, services),
        })
        .collect();

    // Act
    let actions = network
        .receive_message(
            302,
            WireNetworkMessage::Addr(AddressList { addresses }),
            now_unix_seconds as i64,
            verify_flags(),
            consensus_params(),
        )
        .expect("over-cap addr batch should be evidence only")
        .outbound;
    let info = network.address_boundary_info();

    // Assert
    assert!(actions.is_empty());
    assert_eq!(info.learned_address_entries, 0);
    assert_eq!(
        info.learned_address_rejections,
        u32::try_from(PHASE92_LEARNED_ADDR_BATCH_LIMIT + 1).expect("phase limit fits"),
    );
    let latest = info
        .maybe_latest_address_decision
        .expect("latest address decision");
    assert_eq!(latest.label, "learned_rejected");
    assert_eq!(latest.reason, "over_cap_batch");
}

#[test]
fn managed_address_boundary_info_projects_learned_counts() {
    // Arrange
    let services = ServiceFlags::NETWORK | ServiceFlags::WITNESS;
    let learned_address = public_ipv4_network_address(9, 9, 9, 9, 18_445, services);
    let evidence = PeerAddressBoundaryEvidence {
        local_advertisement_candidates: Vec::new(),
        suppressed_advertisements: Vec::new(),
        getaddr_responses_served: Vec::new(),
        getaddr_requests_suppressed: Vec::new(),
        learned_address_entries: vec![learned_address_entry(learned_address)],
        learned_address_rejections: vec![learned_address_rejection(18_446, services)],
        learned_address_rejection_count: 1,
        maybe_latest_address_decision: Some(PeerAddressBoundaryDecision {
            label: AddressDecisionLabel::LearnedRejected,
            reason: AddressDecisionReason::DuplicateAddress,
        }),
    };

    // Act
    let info = ManagedAddressBoundaryInfo::from(evidence);

    // Assert
    assert_eq!(info.learned_address_entries, 1);
    assert_eq!(info.learned_address_rejections, 1);
    let latest = info
        .maybe_latest_address_decision
        .expect("latest learned decision");
    assert_eq!(latest.label, "learned_rejected");
    assert_eq!(latest.reason, "duplicate_address");
}

#[test]
fn managed_address_boundary_info_latest_decision_labels_are_stable() {
    // Arrange
    let cases = [
        (
            AddressDecisionLabel::AdvertiseCandidate,
            AddressDecisionReason::PolicyAccepted,
            "advertise_candidate",
            "source_local_listener",
        ),
        (
            AddressDecisionLabel::AdvertiseSuppressed,
            AddressDecisionReason::PermissionPolicyDenied,
            "advertise_suppressed",
            "source_local_listener",
        ),
        (
            AddressDecisionLabel::GetAddrServed,
            AddressDecisionReason::PolicyAccepted,
            "getaddr_served",
            "source_inbound_addr",
        ),
        (
            AddressDecisionLabel::GetAddrSuppressed,
            AddressDecisionReason::AlreadyServed,
            "getaddr_suppressed",
            "source_inbound_addr",
        ),
        (
            AddressDecisionLabel::LearnedAccepted,
            AddressDecisionReason::PolicyAccepted,
            "learned_accepted",
            "source_inbound_addr",
        ),
        (
            AddressDecisionLabel::LearnedRejected,
            AddressDecisionReason::DuplicateAddress,
            "learned_rejected",
            "source_inbound_addr",
        ),
    ];

    // Act
    let projected: Vec<_> = cases
        .into_iter()
        .map(|(label, reason, expected_label, expected_source)| {
            let info = ManagedAddressBoundaryInfo::from(PeerAddressBoundaryEvidence {
                local_advertisement_candidates: Vec::new(),
                suppressed_advertisements: Vec::new(),
                getaddr_responses_served: Vec::new(),
                getaddr_requests_suppressed: Vec::new(),
                learned_address_entries: Vec::new(),
                learned_address_rejections: Vec::new(),
                learned_address_rejection_count: 0,
                maybe_latest_address_decision: Some(PeerAddressBoundaryDecision { label, reason }),
            });
            let event = info
                .maybe_latest_address_decision
                .expect("latest decision should project");
            (
                event.label,
                event.reason,
                event.source,
                expected_label,
                expected_source,
            )
        })
        .collect();

    // Assert
    for (label, reason, source, expected_label, expected_source) in projected {
        assert_eq!(label, expected_label);
        assert!(!reason.is_empty());
        assert_eq!(source, expected_source);
    }
}

#[test]
fn cap_rejected_inbound_peer_updates_evidence_without_counts() {
    // Arrange
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(202),
        PolicyConfig::default(),
    );
    network.set_inbound_admission_policy(InboundAdmissionPolicy::new(1, 0));
    network.admit_inbound_peer(inbound_request(
        202,
        "127.0.0.1:18444",
        InboundAdmissionSlotClass::Ordinary,
    ));

    // Act
    let decision = network.admit_inbound_peer(inbound_request(
        203,
        "127.0.0.1:18445",
        InboundAdmissionSlotClass::Ordinary,
    ));

    // Assert
    assert!(matches!(
        decision,
        InboundAdmissionDecision::Reject(rejection)
            if rejection.reason == InboundAdmissionRejectionReason::CapReached
    ));
    let info = network.network_info();
    assert_eq!(info.connected_peers, 1);
    assert_eq!(info.inbound_peers, 1);
    assert_eq!(network.inbound_admission_info().rejected_inbound_peers, 1);
    assert_eq!(network.inbound_admission_info().cap_rejections, 1);
}

#[test]
fn managed_network_records_request_cap_event_for_over_cap_inv() {
    // Arrange
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(240),
        PolicyConfig::default(),
    );
    network.add_inbound_peer(240).expect("inbound peer");

    // Act
    let error = network
        .receive_message(
            240,
            WireNetworkMessage::Inv(transaction_inventory(
                PHASE94_MAX_INBOUND_TX_REQUESTS_PER_PEER + 1,
            )),
            1,
            verify_flags(),
            consensus_params(),
        )
        .expect_err("over-cap inv should disconnect");

    // Assert
    assert!(matches!(
        error,
        crate::network::ManagedNetworkError::Network(NetworkError::ResourceLimit(240))
    ));
    assert_eq!(network.network_info().inbound_peers, 0);
    assert_request_cap_resource_governance(&network);
}

#[test]
fn managed_network_records_request_cap_event_for_over_cap_getdata() {
    // Arrange
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(241),
        PolicyConfig::default(),
    );
    network.add_inbound_peer(241).expect("inbound peer");

    // Act
    let error = network
        .receive_message(
            241,
            WireNetworkMessage::GetData(block_inventory(
                PHASE94_MAX_INBOUND_REQUEST_INVENTORY_ITEMS + 1,
            )),
            1,
            verify_flags(),
            consensus_params(),
        )
        .expect_err("over-cap getdata should disconnect");

    // Assert
    assert!(matches!(
        error,
        crate::network::ManagedNetworkError::Network(NetworkError::ResourceLimit(241))
    ));
    assert_eq!(network.network_info().inbound_peers, 0);
    assert_request_cap_resource_governance(&network);
}

#[test]
fn managed_network_records_request_cap_event_for_over_cap_getheaders() {
    // Arrange
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(242),
        PolicyConfig::default(),
    );
    network.add_inbound_peer(242).expect("inbound peer");
    let locator = open_bitcoin_core::primitives::BlockLocator {
        block_hashes: (0..=PHASE94_MAX_HEADER_LOCATOR_HASHES)
            .map(hash_from_index)
            .collect(),
    };

    // Act
    let error = network
        .receive_message(
            242,
            WireNetworkMessage::GetHeaders {
                locator,
                stop_hash: BlockHash::from_byte_array([0_u8; 32]),
            },
            1,
            verify_flags(),
            consensus_params(),
        )
        .expect_err("over-cap getheaders should disconnect");

    // Assert
    assert!(matches!(
        error,
        crate::network::ManagedNetworkError::Network(NetworkError::ResourceLimit(242))
    ));
    assert_eq!(network.network_info().inbound_peers, 0);
    assert_request_cap_resource_governance(&network);
}

#[test]
fn ordinary_inbound_admission_cannot_consume_reserved_capacity() {
    // Arrange
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(204),
        PolicyConfig::default(),
    );
    network.set_inbound_admission_policy(InboundAdmissionPolicy::new(2, 1));
    network.admit_inbound_peer(inbound_request(
        204,
        "127.0.0.1:18444",
        InboundAdmissionSlotClass::Ordinary,
    ));

    // Act
    let decision = network.admit_inbound_peer(inbound_request(
        205,
        "127.0.0.1:18445",
        InboundAdmissionSlotClass::Ordinary,
    ));

    // Assert
    assert!(matches!(
        decision,
        InboundAdmissionDecision::Reject(rejection)
            if rejection.reason == InboundAdmissionRejectionReason::ReservedSlotUnavailable
    ));
    assert_eq!(network.network_info().inbound_peers, 1);
    assert_eq!(network.inbound_admission_info().reserved_slot_rejections, 1,);
}

#[test]
fn reserved_inbound_admission_uses_reserved_capacity_then_rejects_when_exhausted() {
    // Arrange
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(206),
        PolicyConfig::default(),
    );
    network.set_inbound_admission_policy(InboundAdmissionPolicy::new(2, 1));

    // Act
    let admitted = network.admit_inbound_peer(inbound_request(
        206,
        "127.0.0.1:18444",
        InboundAdmissionSlotClass::Reserved,
    ));
    let rejected = network.admit_inbound_peer(inbound_request(
        207,
        "127.0.0.1:18445",
        InboundAdmissionSlotClass::Reserved,
    ));

    // Assert
    assert!(matches!(admitted, InboundAdmissionDecision::Admit(_)));
    assert!(matches!(
        rejected,
        InboundAdmissionDecision::Reject(rejection)
            if rejection.reason == InboundAdmissionRejectionReason::ReservedSlotUnavailable
    ));
    let admission = network.inbound_admission_info();
    assert_eq!(admission.reserved_inbound_admits, 1);
    assert_eq!(admission.permissioned_inbound_admits, 0);
    assert_eq!(admission.protected_inbound_admits, 1);
    assert_eq!(admission.active_permission_effect_observations, 4);
    assert_eq!(admission.inactive_permission_effect_observations, 0);
    assert_eq!(admission.reserved_slot_rejections, 1);
    assert_eq!(network.network_info().inbound_peers, 1);
}

#[test]
fn inbound_admission_preserves_outbound_count_and_observed_outbound_evidence() {
    // Arrange
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(208),
        PolicyConfig::default(),
    );
    network.set_inbound_admission_policy(InboundAdmissionPolicy::new(2, 0));
    network
        .connect_outbound_peer(208, 1)
        .expect("outbound peer");

    // Act
    let decision = network.admit_inbound_peer(inbound_request(
        209,
        "127.0.0.1:18444",
        InboundAdmissionSlotClass::Ordinary,
    ));

    // Assert
    assert!(matches!(decision, InboundAdmissionDecision::Admit(_)));
    let info = network.network_info();
    assert_eq!(info.connected_peers, 2);
    assert_eq!(info.inbound_peers, 1);
    assert_eq!(info.outbound_peers, 1);
    let inbound_record = network
        .peer_manager()
        .peer_state(209)
        .and_then(|peer| peer.maybe_inbound_record.as_ref())
        .expect("inbound record");
    assert_eq!(inbound_record.observed_outbound_peers, 1);
}

#[test]
fn permissioned_and_protected_inbound_admits_do_not_starve_outbound_accounting() {
    // Arrange
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(212),
        PolicyConfig::default(),
    );
    network.set_inbound_admission_policy(InboundAdmissionPolicy::new(3, 1));
    network
        .connect_outbound_peer(212, 1)
        .expect("outbound peer");

    // Act
    let permissioned = network.admit_inbound_peer(permissioned_inbound_request(
        213,
        "127.0.0.1:18447",
        &["in", "download", "addr"],
    ));
    let protected = network.admit_inbound_peer(inbound_request(
        214,
        "127.0.0.1:18448",
        InboundAdmissionSlotClass::Reserved,
    ));

    // Assert
    assert!(matches!(permissioned, InboundAdmissionDecision::Admit(_)));
    assert!(matches!(protected, InboundAdmissionDecision::Admit(_)));
    let info = network.network_info();
    assert_eq!(info.connected_peers, 3);
    assert_eq!(info.inbound_peers, 2);
    assert_eq!(info.outbound_peers, 1);
    let admission = network.inbound_admission_info();
    assert_eq!(admission.permissioned_inbound_admits, 1);
    assert_eq!(admission.protected_inbound_admits, 1);
    assert_eq!(admission.reserved_inbound_admits, 1);
    assert_eq!(admission.active_permission_effect_observations, 6);
    assert_eq!(admission.inactive_permission_effect_observations, 0);
}

#[test]
fn duplicate_inbound_endpoint_or_peer_id_rejects_before_counts_change() {
    // Arrange
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(210),
        PolicyConfig::default(),
    );
    network.set_inbound_admission_policy(InboundAdmissionPolicy::new(4, 0));
    network.admit_inbound_peer(inbound_request(
        210,
        "127.0.0.1:18444",
        InboundAdmissionSlotClass::Ordinary,
    ));

    // Act
    let duplicate_endpoint = network.admit_inbound_peer(inbound_request(
        211,
        "127.0.0.1:18444",
        InboundAdmissionSlotClass::Ordinary,
    ));
    let duplicate_peer_id = network.admit_inbound_peer(inbound_request(
        210,
        "127.0.0.1:18445",
        InboundAdmissionSlotClass::Ordinary,
    ));

    // Assert
    assert!(matches!(
        duplicate_endpoint,
        InboundAdmissionDecision::Reject(rejection)
            if rejection.reason == InboundAdmissionRejectionReason::DuplicateEndpoint
    ));
    assert!(matches!(
        duplicate_peer_id,
        InboundAdmissionDecision::Reject(rejection)
            if rejection.reason == InboundAdmissionRejectionReason::DuplicatePeerId
    ));
    let info = network.network_info();
    assert_eq!(info.connected_peers, 1);
    assert_eq!(info.inbound_peers, 1);
    let admission = network.inbound_admission_info();
    assert_eq!(admission.duplicate_endpoint_rejections, 1);
    assert_eq!(admission.duplicate_identity_rejections, 1);
    assert_eq!(admission.rejected_inbound_peers, 2);
}

mod block_connect_disposition {
    use open_bitcoin_core::consensus::block_hash;

    use super::*;

    #[test]
    fn connect_stored_block_returns_connected_disposition() {
        // Arrange
        let mut network = ManagedPeerNetwork::new(
            MemoryChainstateStore::default(),
            local_config(101),
            PolicyConfig::default(),
        );
        let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 500_000_000);
        let child = build_block(block_hash(&genesis.header), 1, 500_000_000);
        network
            .connect_local_block(&genesis, verify_flags(), consensus_params())
            .expect("connect genesis");

        // Act
        let disposition = network
            .connect_stored_block(
                &child,
                2,
                i64::from(child.header.time),
                verify_flags(),
                consensus_params(),
            )
            .expect("connect stored child");

        // Assert
        assert_eq!(
            disposition,
            BlockConnectDisposition::Connected(
                network
                    .maybe_chain_tip()
                    .expect("child should become active tip")
            )
        );
        assert_eq!(network.maybe_chain_tip().expect("tip").height, 1);
    }

    #[test]
    fn connect_stored_block_returns_duplicate_disposition() {
        // Arrange
        let mut network = ManagedPeerNetwork::new(
            MemoryChainstateStore::default(),
            local_config(102),
            PolicyConfig::default(),
        );
        let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 500_000_000);
        let child = build_block(block_hash(&genesis.header), 1, 500_000_000);
        network
            .connect_local_block(&genesis, verify_flags(), consensus_params())
            .expect("connect genesis");
        network
            .connect_local_block(&child, verify_flags(), consensus_params())
            .expect("connect child");
        let child_hash = block_hash(&child.header);

        // Act
        let disposition = network
            .connect_stored_block(
                &child,
                2,
                i64::from(child.header.time),
                verify_flags(),
                consensus_params(),
            )
            .expect("classify duplicate");

        // Assert
        assert_eq!(disposition, BlockConnectDisposition::Duplicate(child_hash));
        assert_eq!(network.chainstate_snapshot().active_chain.len(), 2);
        assert_eq!(network.maybe_chain_tip().expect("tip").height, 1);
    }

    #[test]
    fn connect_stored_block_returns_non_extending_disposition() {
        // Arrange
        let mut network = ManagedPeerNetwork::new(
            MemoryChainstateStore::default(),
            local_config(103),
            PolicyConfig::default(),
        );
        let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 500_000_000);
        let unknown_parent = BlockHash::from_byte_array([42_u8; 32]);
        let side_block = build_block(unknown_parent, 1, 500_000_000);
        network
            .connect_local_block(&genesis, verify_flags(), consensus_params())
            .expect("connect genesis");
        let side_hash = block_hash(&side_block.header);

        // Act
        let disposition = network
            .connect_stored_block(
                &side_block,
                2,
                i64::from(side_block.header.time),
                verify_flags(),
                consensus_params(),
            )
            .expect("classify non-extending block");

        // Assert
        assert_eq!(
            disposition,
            BlockConnectDisposition::NonExtending {
                block_hash: side_hash,
                previous_block_hash: unknown_parent,
            }
        );
        assert_eq!(network.maybe_chain_tip().expect("tip").height, 0);
    }

    #[test]
    fn connect_stored_block_returns_disconnected_disposition() {
        // Arrange
        let mut network = ManagedPeerNetwork::new(
            MemoryChainstateStore::default(),
            local_config(104),
            PolicyConfig::default(),
        );
        let unknown_parent = BlockHash::from_byte_array([7_u8; 32]);
        let disconnected = build_block(unknown_parent, 1, 500_000_000);
        let disconnected_hash = block_hash(&disconnected.header);

        // Act
        let disposition = network
            .connect_stored_block(
                &disconnected,
                1,
                i64::from(disconnected.header.time),
                verify_flags(),
                consensus_params(),
            )
            .expect("classify disconnected block");

        // Assert
        assert_eq!(
            disposition,
            BlockConnectDisposition::Disconnected {
                block_hash: disconnected_hash,
            }
        );
        assert!(network.maybe_chain_tip().is_none());
    }

    #[test]
    fn receive_sync_message_reports_block_disposition() {
        // Arrange
        let mut network = ManagedPeerNetwork::new(
            MemoryChainstateStore::default(),
            local_config(105),
            PolicyConfig::default(),
        );
        let disconnected = build_block(BlockHash::from_byte_array([9_u8; 32]), 1, 500_000_000);
        network.add_inbound_peer(55).expect("peer");

        // Act
        let result = network
            .receive_sync_message(
                55,
                WireNetworkMessage::Block(disconnected),
                1_231_006_501,
                verify_flags(),
                consensus_params(),
            )
            .expect("receive sync block");

        // Assert
        assert!(result.maybe_block_disposition.is_some());
    }
}

#[test]
fn managed_network_requests_transactions_using_wtxidrelay_when_negotiated() {
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(1),
        PolicyConfig::default(),
    );
    network.add_inbound_peer(1).expect("peer");
    network
        .receive_message(
            1,
            WireNetworkMessage::Version(open_bitcoin_network::VersionMessage::default()),
            1,
            verify_flags(),
            consensus_params(),
        )
        .expect("version");
    network
        .receive_message(
            1,
            WireNetworkMessage::WtxidRelay,
            1,
            verify_flags(),
            consensus_params(),
        )
        .expect("wtxidrelay");

    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 500_000_000);
    let spendable = build_block(
        open_bitcoin_core::consensus::block_hash(&genesis.header),
        1,
        500_000_000,
    );
    network
        .connect_local_block(&genesis, verify_flags(), consensus_params())
        .expect("genesis");
    network
        .connect_local_block(&spendable, verify_flags(), consensus_params())
        .expect("spendable");

    let transaction = spend_transaction(
        transaction_txid(&genesis.transactions[0]).expect("txid"),
        499_999_000,
    );
    network
        .submit_local_transaction_outcome_at(
            transaction.clone(),
            verify_flags(),
            consensus_params(),
            2,
            RelayIntent::Requested,
        )
        .expect("admit");

    let message = network
        .announce_transaction(1, &transaction)
        .expect("announce")
        .expect("message");
    assert!(matches!(
        message,
        WireNetworkMessage::Inv(InventoryList { inventory })
        if inventory[0].inventory_type == InventoryType::WitnessTransaction
    ));
}

#[test]
fn managed_network_info_exposes_rpc_projection_helpers() {
    // Arrange
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(100),
        PolicyConfig::default(),
    );
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 500_000_000);
    let spendable = build_block(
        open_bitcoin_core::consensus::block_hash(&genesis.header),
        1,
        500_000_000,
    );
    network
        .connect_local_block(&genesis, verify_flags(), consensus_params())
        .expect("genesis");
    network
        .connect_local_block(&spendable, verify_flags(), consensus_params())
        .expect("spendable");
    network.add_inbound_peer(7).expect("inbound peer");
    network
        .receive_message(
            7,
            WireNetworkMessage::WtxidRelay,
            1,
            verify_flags(),
            consensus_params(),
        )
        .expect("wtxidrelay");
    network
        .receive_message(
            7,
            WireNetworkMessage::SendHeaders,
            1,
            verify_flags(),
            consensus_params(),
        )
        .expect("sendheaders");
    network.connect_outbound_peer(8, 2).expect("outbound peer");
    let transaction = spend_transaction(
        transaction_txid(&genesis.transactions[0]).expect("txid"),
        499_999_000,
    );
    let expected_virtual_size =
        open_bitcoin_mempool::transaction_weight_and_virtual_size(&transaction)
            .expect("weight")
            .1;
    network
        .submit_local_transaction_outcome_at(
            transaction,
            verify_flags(),
            consensus_params(),
            3,
            RelayIntent::NotRequested,
        )
        .expect("submit");

    // Act
    let snapshot = network.chainstate_snapshot();
    let maybe_tip = network.maybe_chain_tip();
    let mempool_info = network.mempool_info();
    let network_info = network.network_info();

    // Assert
    assert_eq!(snapshot.active_chain.len(), 2);
    assert_eq!(maybe_tip.expect("tip").height, 1);
    assert_eq!(mempool_info.transaction_count, 1);
    assert_eq!(mempool_info.total_virtual_size, expected_virtual_size);
    assert!(mempool_info.accounted_memory > mempool_info.total_virtual_size);
    assert_eq!(mempool_info.mempool_capacity, 300_000_000);
    assert_eq!(mempool_info.total_fee_sats, 1_000);
    assert_eq!(
        mempool_info.capacity_enforcement,
        MempoolCapacityEnforcement::LegacyVsize
    );
    assert_eq!(network_info.connected_peers, 2);
    assert_eq!(network_info.inbound_peers, 1);
    assert_eq!(network_info.outbound_peers, 1);
    assert_eq!(network_info.wtxidrelay_peers, 1);
    assert_eq!(network_info.header_preferring_peers, 1);
}

#[test]
fn mempool_info_exposes_truthful_resource_and_fee_roles() {
    // Arrange
    let policy = PolicyConfig {
        mempool_capacity: MempoolCapacity::new(12_345_678),
        static_relay_fee_rate: StaticRelayFeeRate::new(FeeRate::from_sats_per_kvb(1_000)),
        incremental_relay_fee_rate: IncrementalRelayFeeRate::new(FeeRate::from_sats_per_kvb(7_000)),
        ..PolicyConfig::default()
    };
    let mut network =
        ManagedPeerNetwork::new(MemoryChainstateStore::default(), local_config(91), policy);
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 500_000_000);
    let spendable = build_block(block_hash(&genesis.header), 1, 500_000_000);
    network
        .connect_local_block(&genesis, verify_flags(), consensus_params())
        .expect("genesis");
    network
        .connect_local_block(&spendable, verify_flags(), consensus_params())
        .expect("spendable");
    let transaction = script_heavy_spend_transaction(
        transaction_txid(&genesis.transactions[0]).expect("txid"),
        499_998_000,
    );
    let expected_virtual_size =
        open_bitcoin_mempool::transaction_weight_and_virtual_size(&transaction)
            .expect("weight")
            .1;
    network
        .submit_local_transaction_outcome_at(
            transaction,
            verify_flags(),
            consensus_params(),
            3,
            RelayIntent::NotRequested,
        )
        .expect("submit");
    network.set_rolling_mempool_fee_rate(RollingMempoolFeeRate::new(FeeRate::from_sats_per_kvb(
        3_000,
    )));

    // Act
    let info = network.mempool_info();
    let serialized = format!("{info:?}");

    // Assert
    assert_eq!(info.transaction_count, 1);
    assert_eq!(info.total_virtual_size, expected_virtual_size);
    assert!(info.accounted_memory > info.total_virtual_size);
    assert_eq!(info.mempool_capacity, 12_345_678);
    assert_ne!(info.total_virtual_size, info.accounted_memory);
    assert_ne!(info.accounted_memory, info.mempool_capacity);
    assert_ne!(info.total_virtual_size, info.mempool_capacity);
    assert_eq!(info.static_relay_fee_rate_sats_per_kvb, 1_000);
    assert_eq!(info.incremental_relay_fee_rate_sats_per_kvb, 7_000);
    assert_eq!(info.rolling_mempool_fee_rate_sats_per_kvb, 3_000);
    assert_eq!(info.effective_admission_fee_rate_sats_per_kvb, 3_000);
    assert_eq!(
        info.effective_admission_fee_rate_sats_per_kvb,
        info.static_relay_fee_rate_sats_per_kvb
            .max(info.rolling_mempool_fee_rate_sats_per_kvb)
    );
    assert_ne!(
        info.effective_admission_fee_rate_sats_per_kvb,
        info.incremental_relay_fee_rate_sats_per_kvb
    );
    assert_eq!(
        info.capacity_enforcement,
        MempoolCapacityEnforcement::LegacyVsize
    );
    assert_eq!(info.capacity_enforcement.as_str(), "legacy_vsize");
    for forbidden in [
        "txid",
        "wtxid",
        "peer_id",
        "127.0.0.1",
        "script_sig",
        "transaction_hex",
    ] {
        assert!(
            !serialized.to_lowercase().contains(forbidden),
            "shared evidence leaked {forbidden}: {serialized}"
        );
    }
}

#[test]
fn managed_nodes_sync_blocks_and_relay_transactions_in_memory() {
    let mut source = block_serving_enabled_managed_network(10);
    let mut sink = relay_enabled_managed_network(20);

    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 500_000_000);
    let spendable = build_block(
        open_bitcoin_core::consensus::block_hash(&genesis.header),
        1,
        500_000_000,
    );
    source
        .connect_local_block(&genesis, verify_flags(), consensus_params())
        .expect("genesis");
    source
        .connect_local_block(&spendable, verify_flags(), consensus_params())
        .expect("spendable");

    let source_admission = source.admit_inbound_peer(permissioned_inbound_request(
        7,
        "127.0.0.1:18447",
        &["in", "download", "relay", "mempool"],
    ));
    assert!(matches!(
        source_admission,
        InboundAdmissionDecision::Admit(_)
    ));
    let sync_timestamp = i64::from(spendable.header.time);
    let mut to_source = sink.connect_outbound_peer(7, 1).expect("connect");
    let mut to_sink = deliver(&sink, &mut source, 7, to_source, sync_timestamp);
    to_source = deliver(&source, &mut sink, 7, to_sink, sync_timestamp);
    to_sink = deliver(&sink, &mut source, 7, to_source, sync_timestamp);
    to_source = deliver(&source, &mut sink, 7, to_sink, sync_timestamp);
    to_sink = deliver(&sink, &mut source, 7, to_source, sync_timestamp);
    let final_outbound = deliver(&source, &mut sink, 7, to_sink, sync_timestamp);
    assert!(final_outbound.is_empty());
    assert_eq!(
        sink.chainstate().chainstate().tip().map(|tip| tip.height),
        Some(1)
    );

    let transaction = spend_transaction(
        transaction_txid(&genesis.transactions[0]).expect("txid"),
        499_999_000,
    );
    source
        .submit_local_transaction_outcome_at(
            transaction.clone(),
            verify_flags(),
            consensus_params(),
            7,
            RelayIntent::Requested,
        )
        .expect("source admit");

    let announced = source
        .announce_transaction(7, &transaction)
        .expect("announce")
        .expect("inv");
    let to_source = deliver(&source, &mut sink, 7, vec![announced], 8);
    let to_sink = deliver(&sink, &mut source, 7, to_source, 9);
    let final_messages = deliver(&source, &mut sink, 7, to_sink, 10);
    assert!(final_messages.is_empty());

    let txid = transaction_txid(&transaction).expect("txid");
    assert!(sink.mempool().mempool().entry(&txid).is_some());
}

#[test]
fn managed_network_rejects_future_block_using_message_timestamp() {
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(30),
        PolicyConfig::default(),
    );
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 500_000_000);
    network
        .connect_local_block(&genesis, verify_flags(), consensus_params())
        .expect("genesis");
    network.add_inbound_peer(9).expect("peer");

    let future_block = build_block(
        open_bitcoin_core::consensus::block_hash(&genesis.header),
        10_000,
        500_000_000,
    );
    let error = network
        .receive_message(
            9,
            WireNetworkMessage::Block(future_block.clone()),
            i64::from(future_block.header.time) - 7_201,
            verify_flags(),
            consensus_params(),
        )
        .expect_err("future block must use the message timestamp");

    assert!(matches!(
        error,
        crate::network::ManagedNetworkError::Chainstate(
            open_bitcoin_core::chainstate::ChainstateError::BlockValidation { source }
        ) if source.reject_reason == "time-too-new"
    ));
}
