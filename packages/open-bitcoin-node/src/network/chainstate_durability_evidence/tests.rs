// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/validation.cpp

use open_bitcoin_core::{
    chainstate::{Chainstate, FlushMode, FlushPolicyTime},
    primitives::{InventoryType, NetworkAddress, NetworkMagic},
};
use open_bitcoin_mempool::PolicyConfig;
use open_bitcoin_network::{
    BlockRelayActivationPolicy, BlockServingEligibilityReason, BlockServingOutcomeLabel,
    BlockServingStatusLabel, LocalPeerConfig, RelayActivationConfig, ServiceFlags,
};

use super::HaveBytesAccumulator;
use crate::MemoryChainstateStore;
use crate::chainstate::{FlushLifecycle, ManagedChainstate};
use crate::network::ManagedPeerNetwork;
use crate::network::block_serving::{BlockServingPresenceFacts, ManagedBlockServeDecision};
use crate::status::{
    CHAINSTATE_DURABILITY_UNAVAILABLE_REASON, FieldAvailability, LastFlushReasonLabel,
    ServingStatusLabel, WriteKindLabel,
};

fn absent_payload_known_index() -> BlockServingPresenceFacts {
    BlockServingPresenceFacts {
        payload_present: false,
        index_known: true,
        validated_on_active_chain: true,
    }
}

fn present_payload() -> BlockServingPresenceFacts {
    BlockServingPresenceFacts {
        payload_present: true,
        index_known: true,
        validated_on_active_chain: true,
    }
}

fn serve_decision(presence: BlockServingPresenceFacts) -> ManagedBlockServeDecision {
    ManagedBlockServeDecision {
        label: BlockServingOutcomeLabel::BlockServingEligible,
        status_label: BlockServingStatusLabel::Available,
        eligibility_reason: BlockServingEligibilityReason::Eligible,
        maybe_block: None,
        missing_inventory: false,
        presence,
    }
}

fn test_local_config(nonce: u64) -> LocalPeerConfig {
    LocalPeerConfig {
        magic: NetworkMagic::MAINNET,
        services: ServiceFlags::NETWORK | ServiceFlags::WITNESS,
        address: NetworkAddress {
            services: 0,
            address_bytes: [0_u8; 16],
            port: 8_333,
        },
        nonce,
        relay: true,
        user_agent: "/open-bitcoin:phase144-01/".to_string(),
    }
}

fn unprojected_network() -> ManagedPeerNetwork<MemoryChainstateStore> {
    ManagedPeerNetwork::from_initialized_chainstate(
        ManagedChainstate::from_chainstate(
            MemoryChainstateStore::default(),
            Chainstate::default(),
            FlushLifecycle::not_ready_for_test(),
        ),
        test_local_config(144_301),
        PolicyConfig::default(),
        1,
        RelayActivationConfig::default(),
        BlockRelayActivationPolicy::default(),
        false,
    )
}

#[test]
fn have_bytes_accumulator_sets_unavailable_when_payload_absent() {
    // Arrange
    let mut accumulator = HaveBytesAccumulator::default();

    // Act
    accumulator.record_have_bytes(absent_payload_known_index());
    let snapshot = accumulator.snapshot();

    // Assert
    assert_eq!(
        snapshot.last_serving_status,
        ServingStatusLabel::Unavailable
    );
    assert!(!snapshot.last_payload_present);
    assert!(snapshot.last_index_known);
    assert!(snapshot.last_validated_on_active_chain);
    assert_eq!(snapshot.index_known_without_payload_count, 1);
    assert_eq!(snapshot.unavailable_count, 1);
    assert_eq!(snapshot.available_count, 0);
}

#[test]
fn have_bytes_accumulator_counts_available_when_payload_present() {
    // Arrange
    let mut accumulator = HaveBytesAccumulator::default();

    // Act
    accumulator.record_have_bytes(present_payload());
    let snapshot = accumulator.snapshot();

    // Assert
    assert_eq!(snapshot.last_serving_status, ServingStatusLabel::Available);
    assert!(snapshot.last_payload_present);
    assert_eq!(snapshot.available_count, 1);
    assert_eq!(snapshot.unavailable_count, 0);
    assert_eq!(snapshot.index_known_without_payload_count, 0);
}

#[test]
fn have_bytes_accumulator_has_no_hash_map_or_pruned_counter() {
    // Arrange
    let source = include_str!("../chainstate_durability_evidence.rs");

    // Act
    let mentions_hash_map = source.contains("HashMap");
    let mentions_pruned_count = source.contains("pruned_count");
    let mentions_pruned = source.contains("pruned");
    let mentions_peer_id = source.contains("peer_id");
    let mentions_by_hash = source.contains("by_hash");

    // Assert
    assert!(!mentions_hash_map, "accumulator must not store a HashMap");
    assert!(
        !mentions_pruned_count,
        "accumulator must not count pruned blocks"
    );
    assert!(!mentions_pruned, "accumulator source must omit pruned");
    assert!(!mentions_peer_id, "accumulator source must omit peer_id");
    assert!(!mentions_by_hash, "accumulator source must omit by_hash");
}

#[test]
fn operator_snapshot_copies_chainstate_durability() {
    // Arrange
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        test_local_config(144_302),
        PolicyConfig::default(),
    );
    network
        .chainstate_mut()
        .flush_with_mode(
            FlushMode::Periodic,
            FlushPolicyTime::from_unix_seconds(2),
            u64::MAX,
        )
        .expect("retain periodic flush");
    network.record_block_serving_evidence(InventoryType::Block, &serve_decision(present_payload()));

    // Act
    let snapshot = network.operator_snapshot();

    // Assert
    let FieldAvailability::Available(evidence) = snapshot.chainstate_durability() else {
        panic!("expected available chainstate durability after flush and have-bytes");
    };
    assert_eq!(evidence.last_flush_reason, LastFlushReasonLabel::Periodic);
    assert_eq!(evidence.write_kind, WriteKindLabel::Sync);
    assert_eq!(evidence.last_serving_status, ServingStatusLabel::Available);
    assert!(evidence.last_payload_present);
    assert_eq!(evidence.available_count, 1);
    assert_eq!(evidence.unavailable_count, 0);
    assert_eq!(evidence.index_known_without_payload_count, 0);
}

#[test]
fn operator_snapshot_unprojected_is_unavailable() {
    // Arrange
    let network = unprojected_network();

    // Act
    let snapshot = network.operator_snapshot();

    // Assert
    assert_eq!(
        snapshot.chainstate_durability(),
        &FieldAvailability::unavailable(CHAINSTATE_DURABILITY_UNAVAILABLE_REASON)
    );
}
