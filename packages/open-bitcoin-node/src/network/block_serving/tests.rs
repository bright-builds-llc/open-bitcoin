// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/test/functional/p2p_getdata.py

use core::cell::Cell;

use open_bitcoin_core::primitives::Block;
use open_bitcoin_network::{
    BlockRelayActivationPolicy, BlockServingActivationConfig, BlockServingChainPosition,
    BlockServingDataAvailability, BlockServingOutcomeLabel, BlockServingValidationState,
    CompactRelayActivationConfig, PHASE94_MAX_INBOUND_BLOCK_REQUESTS_PER_PEER, PeerConnectionClass,
};

use super::{
    BlockServingPresenceFacts, ManagedBlockSerializationMode, ManagedBlockServeCompletionOutcome,
    ManagedBlockServeGateDecision, ManagedBlockServeInput, gate_managed_block_request,
    serve_managed_block_request,
};

fn enabled_input() -> ManagedBlockServeInput {
    ManagedBlockServeInput {
        inventory_type: open_bitcoin_core::primitives::InventoryType::Block,
        block_hash: Default::default(),
        activation: BlockRelayActivationPolicy {
            block_serving: BlockServingActivationConfig { enabled: true },
            compact_relay: Default::default(),
        },
        inbound_serving_enabled: false,
        connection_class: PeerConnectionClass::Outbound,
        active_permission_effects: Vec::new(),
        inactive_permission_effects: Vec::new(),
        requested_blocks_in_flight: 0,
        requested_txids_in_flight: 0,
        requested_wtxids_in_flight: 0,
        chain_position: BlockServingChainPosition::Active,
        validation_state: BlockServingValidationState::Validated,
        data_availability: BlockServingDataAvailability::Available,
        suppressed: false,
        presence: BlockServingPresenceFacts {
            payload_present: true,
            index_known: true,
            validated_on_active_chain: true,
        },
    }
}

#[test]
fn phase111_block_serving_adapter_invokes_lookup_only_after_policy_allows_storage_read() {
    // Arrange
    let lookup_called = Cell::new(false);

    // Act
    let decision = serve_managed_block_request(enabled_input(), |_| {
        lookup_called.set(true);
        Some(Block::default())
    });

    // Assert
    assert_eq!(
        decision.label,
        BlockServingOutcomeLabel::BlockServingEligible
    );
    assert!(decision.maybe_block.is_some());
    assert!(!decision.missing_inventory);
    assert!(lookup_called.get());
}

#[test]
fn phase111_block_serving_disabled_returns_before_payload_lookup() {
    // Arrange
    let mut input = enabled_input();
    input.activation.block_serving.enabled = false;
    let lookup_called = Cell::new(false);

    // Act
    let decision = serve_managed_block_request(input, |_| {
        lookup_called.set(true);
        Some(Block::default())
    });

    // Assert
    assert_eq!(
        decision.label,
        BlockServingOutcomeLabel::BlockServingDisabled
    );
    assert!(decision.maybe_block.is_none());
    assert!(decision.missing_inventory);
    assert!(!lookup_called.get());
}

#[test]
fn phase111_block_serving_recent_valid_available_block_is_served_after_policy_gate() {
    // Arrange
    let mut input = enabled_input();
    input.chain_position = BlockServingChainPosition::RecentValid;
    let lookup_called = Cell::new(false);

    // Act
    let decision = serve_managed_block_request(input, |_| {
        lookup_called.set(true);
        Some(Block::default())
    });

    // Assert
    assert_eq!(
        decision.label,
        BlockServingOutcomeLabel::BlockServingEligible
    );
    assert!(decision.maybe_block.is_some());
    assert!(!decision.missing_inventory);
    assert!(lookup_called.get());
}

#[test]
fn phase111_recent_valid_available_block_is_served_after_policy_gate() {
    // Arrange
    let mut input = enabled_input();
    input.chain_position = BlockServingChainPosition::RecentValid;
    let lookup_called = Cell::new(false);

    // Act
    let decision = serve_managed_block_request(input, |_| {
        lookup_called.set(true);
        Some(Block::default())
    });

    // Assert
    assert_eq!(
        decision.label,
        BlockServingOutcomeLabel::BlockServingEligible
    );
    assert!(decision.maybe_block.is_some());
    assert!(!decision.missing_inventory);
    assert!(lookup_called.get());
}

#[test]
fn phase111_block_serving_stale_block_fact_returns_unavailable_without_payload_lookup() {
    // Arrange
    let mut input = enabled_input();
    input.chain_position = BlockServingChainPosition::Stale;
    let lookup_called = Cell::new(false);

    // Act
    let decision = serve_managed_block_request(input, |_| {
        lookup_called.set(true);
        Some(Block::default())
    });

    // Assert
    assert_eq!(
        decision.label,
        BlockServingOutcomeLabel::BlockStatusUnavailable
    );
    assert!(decision.maybe_block.is_none());
    assert!(decision.missing_inventory);
    assert!(!lookup_called.get());
}

#[test]
fn phase111_stale_block_fact_returns_unavailable_notfound_without_lookup() {
    // Arrange
    let mut input = enabled_input();
    input.chain_position = BlockServingChainPosition::Stale;
    let lookup_called = Cell::new(false);

    // Act
    let decision = serve_managed_block_request(input, |_| {
        lookup_called.set(true);
        Some(Block::default())
    });

    // Assert
    assert_eq!(
        decision.label,
        BlockServingOutcomeLabel::BlockStatusUnavailable
    );
    assert!(decision.maybe_block.is_none());
    assert!(decision.missing_inventory);
    assert!(!lookup_called.get());
}

#[test]
fn phase127_eligible_inventory_types_yield_owned_serve_intents() {
    // Arrange
    let inventory_types = [
        (
            open_bitcoin_core::primitives::InventoryType::Block,
            ManagedBlockSerializationMode::Block,
        ),
        (
            open_bitcoin_core::primitives::InventoryType::WitnessBlock,
            ManagedBlockSerializationMode::WitnessBlock,
        ),
        (
            open_bitcoin_core::primitives::InventoryType::CompactBlock,
            ManagedBlockSerializationMode::CompactBlock,
        ),
    ];

    // Act
    let decisions: Vec<_> = inventory_types
        .into_iter()
        .map(|(inventory_type, expected_mode)| {
            let mut input = enabled_input();
            input.inventory_type = inventory_type;
            input.activation.compact_relay = CompactRelayActivationConfig { enabled: true };
            (gate_managed_block_request(input), expected_mode)
        })
        .collect();

    // Assert
    for (decision, expected_mode) in decisions {
        let ManagedBlockServeGateDecision::Serve(intent) = decision else {
            panic!("eligible inventory should yield a serve intent");
        };
        assert_eq!(intent.serialization_mode(), expected_mode);
    }
}

#[test]
fn phase127_denied_request_never_yields_a_storage_intent() {
    // Arrange
    let mut input = enabled_input();
    input.activation.block_serving.enabled = false;

    // Act
    let decision = gate_managed_block_request(input);

    // Assert
    assert!(matches!(
        decision,
        ManagedBlockServeGateDecision::Deny(decision)
            if decision.label == BlockServingOutcomeLabel::BlockServingDisabled
    ));
}

#[test]
fn phase127_request_cap_denial_precedes_storage_intent() {
    // Arrange
    let mut input = enabled_input();
    input.requested_blocks_in_flight =
        PHASE94_MAX_INBOUND_BLOCK_REQUESTS_PER_PEER.saturating_add(1);

    // Act
    let decision = gate_managed_block_request(input);

    // Assert
    assert!(matches!(
        decision,
        ManagedBlockServeGateDecision::Deny(decision)
            if decision.label == BlockServingOutcomeLabel::BlockRequestCapReached
    ));
}

#[test]
fn phase127_completion_preserves_unavailable_and_success_only_effects() {
    // Arrange
    let ManagedBlockServeGateDecision::Serve(intent) = gate_managed_block_request(enabled_input())
    else {
        panic!("eligible request should yield an intent");
    };

    // Act
    let unavailable = intent.completion(ManagedBlockServeCompletionOutcome::LookupUnavailable);
    let transport_failed = intent.completion(ManagedBlockServeCompletionOutcome::TransportFailed);
    let written = intent.completion(ManagedBlockServeCompletionOutcome::Written);

    // Assert
    assert_eq!(
        unavailable.decision().label,
        BlockServingOutcomeLabel::BlockStatusUnavailable
    );
    assert!(!unavailable.records_served_effect());
    assert!(!transport_failed.records_served_effect());
    assert!(written.records_served_effect());
}
