// Parity breadcrumbs:
// - packages/bitcoin-knots/src/protocol.h
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_invalid_messages.py

use super::*;

#[test]
fn read_queue_pressure_returns_stable_label_and_action() {
    // Arrange
    let policy = ResourceGovernancePolicy::default();
    let input = QueuePressureInput {
        peer_read_queue_bytes: PHASE94_MAX_PEER_READ_QUEUE_BYTES + 1,
        ..empty_queue_input()
    };

    // Act
    let decision = policy.decide_queue(input);

    // Assert
    assert_resource_event(
        decision,
        ResourcePressureLabel::ReadQueuePressure,
        "read_queue_pressure",
    );
}

#[test]
fn write_queue_pressure_returns_stable_label_and_action() {
    // Arrange
    let policy = ResourceGovernancePolicy::default();
    let input = QueuePressureInput {
        peer_write_queue_bytes: PHASE94_MAX_PEER_WRITE_QUEUE_BYTES + 1,
        ..empty_queue_input()
    };

    // Act
    let decision = policy.decide_queue(input);

    // Assert
    assert_resource_event(
        decision,
        ResourcePressureLabel::WriteQueuePressure,
        "write_queue_pressure",
    );
}

#[test]
fn aggregate_write_queue_pressure_returns_stable_label_and_action() {
    // Arrange
    let policy = ResourceGovernancePolicy::default();
    let input = QueuePressureInput {
        aggregate_write_queue_bytes: PHASE94_MAX_AGGREGATE_WRITE_QUEUE_BYTES + 1,
        ..empty_queue_input()
    };

    // Act
    let decision = policy.decide_queue(input);

    // Assert
    assert_resource_event(
        decision,
        ResourcePressureLabel::WriteQueuePressure,
        "write_queue_pressure",
    );
}

#[test]
fn queued_message_pressure_returns_resource_pressure_active() {
    // Arrange
    let policy = ResourceGovernancePolicy::default();
    let input = QueuePressureInput {
        peer_queued_messages: PHASE94_MAX_PEER_QUEUED_MESSAGES + 1,
        ..empty_queue_input()
    };

    // Act
    let decision = policy.decide_queue(input);

    // Assert
    assert_resource_event(
        decision,
        ResourcePressureLabel::ResourcePressureActive,
        "resource_pressure_active",
    );
}

#[test]
fn inbound_getdata_inventory_above_cap_returns_request_cap_label() {
    // Arrange
    let policy = ResourceGovernancePolicy::default();
    let input = RequestPressureInput {
        getdata_items: PHASE94_MAX_INBOUND_REQUEST_INVENTORY_ITEMS + 1,
        ..empty_request_input()
    };

    // Act
    let decision = policy.decide_request(input);

    // Assert
    assert_resource_event(
        decision,
        ResourcePressureLabel::RequestCapReached,
        "request_cap_reached",
    );
}

#[test]
fn inventory_count_above_cap_returns_request_cap_label() {
    // Arrange
    let policy = ResourceGovernancePolicy::default();
    let input = RequestPressureInput {
        inventory_items: PHASE94_MAX_INBOUND_REQUEST_INVENTORY_ITEMS + 1,
        ..empty_request_input()
    };

    // Act
    let decision = policy.decide_request(input);

    // Assert
    assert_resource_event(
        decision,
        ResourcePressureLabel::RequestCapReached,
        "request_cap_reached",
    );
}

#[test]
fn header_locator_above_cap_returns_request_cap_label() {
    // Arrange
    let policy = ResourceGovernancePolicy::default();
    let input = RequestPressureInput {
        header_locator_hashes: PHASE94_MAX_HEADER_LOCATOR_HASHES + 1,
        ..empty_request_input()
    };

    // Act
    let decision = policy.decide_request(input);

    // Assert
    assert_resource_event(
        decision,
        ResourcePressureLabel::RequestCapReached,
        "request_cap_reached",
    );
}

#[test]
fn block_requests_above_cap_return_request_cap_label() {
    // Arrange
    let policy = ResourceGovernancePolicy::default();
    let input = RequestPressureInput {
        requested_blocks_in_flight: PHASE94_MAX_INBOUND_BLOCK_REQUESTS_PER_PEER + 1,
        ..empty_request_input()
    };

    // Act
    let decision = policy.decide_request(input);

    // Assert
    assert_resource_event(
        decision,
        ResourcePressureLabel::RequestCapReached,
        "request_cap_reached",
    );
}

#[test]
fn transaction_requests_above_cap_return_request_cap_label() {
    // Arrange
    let policy = ResourceGovernancePolicy::default();
    let input = RequestPressureInput {
        requested_txids_in_flight: PHASE94_MAX_INBOUND_TX_REQUESTS_PER_PEER,
        requested_wtxids_in_flight: 1,
        ..empty_request_input()
    };

    // Act
    let decision = policy.decide_request(input);

    // Assert
    assert_resource_event(
        decision,
        ResourcePressureLabel::RequestCapReached,
        "request_cap_reached",
    );
}

#[test]
fn resource_policy_accepts_inputs_at_configured_caps() {
    // Arrange
    let policy = ResourceGovernancePolicy::default();
    let queue_input = QueuePressureInput {
        peer_read_queue_bytes: PHASE94_MAX_PEER_READ_QUEUE_BYTES,
        peer_write_queue_bytes: PHASE94_MAX_PEER_WRITE_QUEUE_BYTES,
        aggregate_write_queue_bytes: PHASE94_MAX_AGGREGATE_WRITE_QUEUE_BYTES,
        peer_queued_messages: PHASE94_MAX_PEER_QUEUED_MESSAGES,
        ..empty_queue_input()
    };
    let request_input = RequestPressureInput {
        inventory_items: PHASE94_MAX_INBOUND_REQUEST_INVENTORY_ITEMS,
        getdata_items: PHASE94_MAX_INBOUND_REQUEST_INVENTORY_ITEMS,
        header_locator_hashes: PHASE94_MAX_HEADER_LOCATOR_HASHES,
        requested_blocks_in_flight: PHASE94_MAX_INBOUND_BLOCK_REQUESTS_PER_PEER,
        requested_txids_in_flight: PHASE94_MAX_INBOUND_TX_REQUESTS_PER_PEER / 2,
        requested_wtxids_in_flight: PHASE94_MAX_INBOUND_TX_REQUESTS_PER_PEER / 2,
        ..empty_request_input()
    };

    // Act
    let queue_decision = policy.decide_queue(queue_input);
    let request_decision = policy.decide_request(request_input);

    // Assert
    assert_eq!(queue_decision, ResourceGovernanceDecision::Accept);
    assert_eq!(request_decision, ResourceGovernanceDecision::Accept);
}

#[test]
fn inactive_relay_like_effects_do_not_raise_resource_caps() {
    // Arrange
    let policy = ResourceGovernancePolicy::default();
    let inactive_permission_effects = vec![
        InactivePermissionEffectLabel::Relay,
        InactivePermissionEffectLabel::ForceRelay,
        InactivePermissionEffectLabel::Mempool,
        InactivePermissionEffectLabel::BloomFilter,
        InactivePermissionEffectLabel::BlockFilters,
    ];
    let queue_input = QueuePressureInput {
        peer_read_queue_bytes: PHASE94_MAX_PEER_READ_QUEUE_BYTES + 1,
        inactive_permission_effects: inactive_permission_effects.clone(),
        ..empty_queue_input()
    };
    let request_input = RequestPressureInput {
        getdata_items: PHASE94_MAX_INBOUND_REQUEST_INVENTORY_ITEMS + 1,
        active_permission_effects: vec![PermissionEffectLabel::DownloadServingPolicyInput],
        inactive_permission_effects,
        ..empty_request_input()
    };

    // Act
    let queue_decision = policy.decide_queue(queue_input);
    let request_decision = policy.decide_request(request_input);

    // Assert
    assert_resource_event(
        queue_decision,
        ResourcePressureLabel::ReadQueuePressure,
        "read_queue_pressure",
    );
    assert_resource_event(
        request_decision,
        ResourcePressureLabel::RequestCapReached,
        "request_cap_reached",
    );
}
