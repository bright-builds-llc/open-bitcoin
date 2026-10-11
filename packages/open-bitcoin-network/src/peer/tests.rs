// Parity breadcrumbs:
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/node/txdownloadman.h
// - packages/bitcoin-knots/test/functional/p2p_handshake.py
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use core::net::IpAddr;
use std::collections::BTreeSet;

use open_bitcoin_chainstate::ChainPosition;
use open_bitcoin_codec::{
    BlockTransactions, BlockTransactionsRequest, CompactBlockPayload, PrefilledTransaction,
    SendCompactMessage, ShortId,
};
use open_bitcoin_consensus::{block_hash, check_block_header, transaction_txid, transaction_wtxid};
use open_bitcoin_primitives::{
    Amount, Block, BlockHash, BlockHeader, Hash32, MerkleRoot, MessageCommand, NetworkAddress,
    NetworkMagic, OutPoint, ScriptBuf, ScriptWitness, Transaction, TransactionInput,
    TransactionOutput, Txid, Wtxid,
};

use crate::{
    BanDecision, BanReason, BanScope, BlockInFlightCleanupCause, BlockInFlightCleanupInput,
    BlockRelayActivationPolicy, BlockServingActivationConfig, BlockServingOutcomeLabel,
    BlockServingResourceGateDecision, BlockServingStatusDecision, BlockServingStatusLabel,
    CompactAnnouncementEligibility, CompactAnnouncementEligibilityReason, CompactBlockReceiveFacts,
    CompactDownloadCleanupCause, CompactRelayActivationConfig, CompactRelayCapability,
    CompactRelayPreference, ConnectionRole, DisconnectReason, HeaderStore, HeaderSyncPolicy,
    HeadersMessage, InboundAdmissionRejectionReason, InboundAdmissionSlotClass,
    InboundHandshakeState, InboundPeerRecord, InboundPermissionDecision, InventoryList,
    LocalPeerConfig, NetworkError, OrphanPolicy, OrphanReconsiderationStatus, OrphanStageInput,
    PHASE94_MAX_HEADER_LOCATOR_HASHES, PHASE94_MAX_INBOUND_BLOCK_REQUESTS_PER_PEER,
    PHASE94_MAX_INBOUND_REQUEST_INVENTORY_ITEMS, PHASE94_MAX_INBOUND_TX_REQUESTS_PER_PEER,
    PHASE101_GETDATA_TX_INTERVAL_SECONDS, PHASE101_MAX_TX_REQUESTS_IN_FLIGHT_PER_PEER,
    ParsedPeerPermissionClass, PeerAction, PeerBanEntry, PeerConnectionClass, PeerId, PeerManager,
    PeerPermissionClassRegistry, PermissionEffectLabel, ReceivedTransactionProvenance,
    RejectEvidenceTweak, RelayActivationConfig, RelayDownloadPolicy, RequestPressureInput,
    ResourceGovernanceDecision, ResourceGovernancePolicy, ServiceFlags, TxDownloadAction,
    TxDownloadSuppressionReason, TxRelayId, WireNetworkMessage, classify_block_inflight_cleanup,
};
use open_bitcoin_primitives::{InventoryType, InventoryVector};

use crate::address::{
    AddressAnnouncement, AddressDecisionLabel, AddressDecisionReason, AddressList,
    AddressNetworkKind, AddressSourceKind, GetAddrResponseDecision, LocalAdvertisementDecision,
    PHASE92_GETADDR_RESPONSE_LIMIT, PHASE92_LEARNED_ADDR_BATCH_LIMIT, RoutabilityClass,
};

use super::DEFAULT_MAX_BLOCKS_IN_FLIGHT_PER_PEER;
use super::compact_relay::{
    BIP152_MIN_PROTOCOL_VERSION, CompactAnnouncementAction, LocalCompactRelayOfferState,
    PeerCompactAnnouncementInput, maybe_schedule_local_compact_offer,
};

mod peer_fixtures;
use peer_fixtures::*;
mod transaction_fixtures;
use transaction_fixtures::*;
mod compact_fixtures;
use compact_fixtures::*;
mod block_fixtures;
use block_fixtures::*;
mod address_fixtures;
use address_fixtures::*;
mod address_cases;
mod block_announcement_fallback_cases;
mod block_sync_lifecycle_cases;
mod block_sync_request_cap_cases;
mod block_sync_request_policy_cases;
mod compact_download_fallback_cases;
mod compact_download_lifecycle_cases;
mod compact_protocol_cases;
mod handshake_policy_cases;
mod peer_action_cases;
mod phase113_announcement_fallback_cases;
mod phase113_announcement_gate_cases;
mod phase113_negotiation_cases;
mod phase128_offer_cases;
mod transaction_lifecycle_cases;
mod transaction_lifecycle_independence_cases;
mod transaction_relay_download_cases;
mod transaction_relay_orphan_candidate_cases;
mod transaction_relay_orphan_lifecycle_cases;

fn phase160_frame(command: &str, payload: &[u8]) -> Vec<u8> {
    let digest = open_bitcoin_consensus::crypto::double_sha256(payload);
    let header = open_bitcoin_primitives::MessageHeader {
        magic: NetworkMagic::MAINNET,
        command: MessageCommand::new(command).expect("command"),
        payload_size: payload.len() as u32,
        checksum: [digest[0], digest[1], digest[2], digest[3]],
    };
    let mut frame = open_bitcoin_codec::encode_message_header(&header);
    frame.extend_from_slice(payload);
    frame
}

#[test]
fn phase160_wire_dispatch_short_complete_requests_are_nonfatal() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(160, 0).expect("peer");
    complete_outbound_handshake(&mut manager, 160, 0);
    for (command, len) in [
        ("getcfilters", 37),
        ("getcfheaders", 37),
        ("getcfcheckpt", 33),
    ] {
        for short_len in 0..len {
            let frame = phase160_frame(command, &vec![0; short_len]);
            // Act
            let parsed = crate::ParsedNetworkMessage::decode_wire(&frame).expect("complete frame");
            assert!(matches!(
                parsed.message,
                WireNetworkMessage::MalformedCFRequest(_)
            ));
            assert!(parsed.message.encode_payload().is_err());
            let actions = manager
                .handle_message(160, parsed.message, 13)
                .expect("nonfatal receive");
            // Assert
            assert!(actions.is_empty(), "{command}: {short_len}");
            assert!(
                manager
                    .handle_message(160, WireNetworkMessage::Ping { nonce: 19 }, 14)
                    .is_ok()
            );
        }
    }
}

#[test]
fn phase160_wire_dispatch_request_trailing_bytes_preserve_semantics() {
    // Arrange
    for (command, len) in [
        ("getcfilters", 37),
        ("getcfheaders", 37),
        ("getcfcheckpt", 33),
    ] {
        let payload: Vec<u8> = (0..len).collect();
        let original = crate::ParsedNetworkMessage::decode_wire(&phase160_frame(command, &payload))
            .expect("request");
        let mut trailing = payload;
        trailing.extend_from_slice(&[255, 0, 1, 2]);
        // Act
        let parsed = crate::ParsedNetworkMessage::decode_wire(&phase160_frame(command, &trailing))
            .expect("trailing handler bytes");
        // Assert
        assert_eq!(parsed.message, original.message);
    }
}

#[test]
fn phase160_wire_dispatch_frame_failures_remain_errors() {
    // Arrange
    let frame = phase160_frame("getcfilters", &[0; 37]);
    let mut corrupt_checksum = frame.clone();
    corrupt_checksum[20] ^= 1;
    let mut bad_command = frame.clone();
    bad_command[4] = 0xff;
    // Act / Assert
    for invalid in [
        &frame[..23],
        &frame[..frame.len() - 1],
        &corrupt_checksum,
        &bad_command,
    ] {
        assert!(crate::ParsedNetworkMessage::decode_wire(invalid).is_err());
    }
}

#[test]
fn phase160_wire_dispatch_non_cf_and_responses_remain_strict() {
    // Arrange
    for (command, len) in [
        ("ping", 8),
        ("verack", 0),
        ("cfilter", 34),
        ("cfheaders", 66),
        ("cfcheckpt", 34),
    ] {
        let mut payload = vec![0; len];
        payload.push(42);
        // Act
        let parsed = crate::ParsedNetworkMessage::decode_wire(&phase160_frame(command, &payload));
        // Assert
        assert!(parsed.is_err(), "{command}");
    }
}
