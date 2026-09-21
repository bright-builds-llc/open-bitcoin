// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/net_processing.h
// - packages/bitcoin-knots/test/functional/p2p_compactblocks.py

use crate::network::{
    AnnouncementPreparationOutcome, EffectCompletion, ManagedNetworkHandle, PeerEmission,
    PeerOutboxSnapshot,
    lifecycle_effects::{PeerEffectCapability, PeerEffectId, PeerSessionGeneration},
    lifecycle_projection::{AuthorityEpoch, LifecycleGeneration},
};
use open_bitcoin_mempool::MempoolMemberIdentity;
use open_bitcoin_network::{HeadersMessage, InventoryList, PHASE94_MAX_PEER_QUEUED_MESSAGES};

use super::*;

fn sample_member() -> MempoolMemberIdentity {
    MempoolMemberIdentity {
        txid: Txid::from_byte_array([0x31; 32]),
        wtxid: open_bitcoin_core::primitives::Wtxid::from_byte_array([0x32; 32]),
    }
}

fn sample_effect_capability(peer_id: open_bitcoin_network::PeerId) -> PeerEffectCapability {
    PeerEffectCapability::new(
        AuthorityEpoch::INITIAL,
        LifecycleGeneration::INITIAL,
        PeerEffectId::new(1),
        peer_id,
        PeerSessionGeneration::INITIAL,
    )
}

fn prepared_emission(
    network: &ManagedNetworkHandle,
    peer_id: open_bitcoin_network::PeerId,
    message: WireNetworkMessage,
    block_hash: BlockHash,
) -> PeerEmission {
    let capability = network
        .prepare_peer_relay_effect(peer_id)
        .expect("peer effect capability");
    PeerEmission::new(peer_id, message, block_hash, capability)
        .expect("supported announcement emission")
}

fn announcement_counts(network: &ManagedNetworkHandle) -> serde_json::Value {
    serde_json::to_value(
        network
            .block_relay_evidence_status()
            .expect("block relay evidence"),
    )
    .expect("serialize block relay evidence")
}

fn compact_message() -> WireNetworkMessage {
    WireNetworkMessage::CompactBlock(CompactBlockPayload {
        header: BlockHeader::default(),
        nonce: 7,
        short_ids: Vec::new(),
        prefilled_transactions: Vec::new(),
    })
}

fn headers_message() -> WireNetworkMessage {
    WireNetworkMessage::Headers(HeadersMessage {
        headers: vec![BlockHeader::default()],
    })
}

fn inventory_message(block_hash: BlockHash) -> WireNetworkMessage {
    WireNetworkMessage::Inv(InventoryList::new(vec![InventoryVector {
        inventory_type: InventoryType::Block,
        object_hash: block_hash.into(),
    }]))
}

mod classification_and_outbox;
mod receipt_credit;
mod tx_emission;
