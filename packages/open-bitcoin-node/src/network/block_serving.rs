// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/test/functional/p2p_getdata.py

use open_bitcoin_core::primitives::{Block, BlockHash, InventoryType};
use open_bitcoin_network::{
    BlockRelayActivationPolicy, BlockServingChainPosition, BlockServingDataAvailability,
    BlockServingEligibilityInput, BlockServingOutcomeLabel, BlockServingResourceGateInput,
    BlockServingStatusFacts, BlockServingValidationState, InactivePermissionEffectLabel,
    PeerConnectionClass, PermissionEffectLabel, QueuePressureInput, ReconnectSuppressionInput,
    RequestPressureInput, ResourceGovernancePolicy, classify_block_serving_eligibility,
    classify_block_serving_status, evaluate_block_serving_resource_gate,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ManagedBlockServeInput {
    pub inventory_type: InventoryType,
    pub block_hash: BlockHash,
    pub activation: BlockRelayActivationPolicy,
    pub inbound_serving_enabled: bool,
    pub connection_class: PeerConnectionClass,
    pub active_permission_effects: Vec<PermissionEffectLabel>,
    pub inactive_permission_effects: Vec<InactivePermissionEffectLabel>,
    pub requested_blocks_in_flight: usize,
    pub requested_txids_in_flight: usize,
    pub requested_wtxids_in_flight: usize,
    pub chain_position: BlockServingChainPosition,
    pub validation_state: BlockServingValidationState,
    pub data_availability: BlockServingDataAvailability,
    pub suppressed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ManagedBlockServeDecision {
    pub label: BlockServingOutcomeLabel,
    pub maybe_block: Option<Block>,
    pub missing_inventory: bool,
}

pub(super) fn serve_managed_block_request(
    input: ManagedBlockServeInput,
    lookup_block: impl FnOnce(BlockHash) -> Option<Block>,
) -> ManagedBlockServeDecision {
    let status = classify_block_serving_status(&BlockServingStatusFacts {
        chain_position: input.chain_position,
        validation_state: input.validation_state,
        data_availability: input.data_availability,
        suppressed: input.suppressed,
    });
    let eligibility = classify_block_serving_eligibility(&BlockServingEligibilityInput {
        activation: input.activation,
        inbound_serving_enabled: input.inbound_serving_enabled,
        connection_class: input.connection_class,
        active_permission_effects: input.active_permission_effects.clone(),
        inactive_permission_effects: input.inactive_permission_effects.clone(),
        status_available: status.may_serve_block,
    });
    let gate = evaluate_block_serving_resource_gate(
        &ResourceGovernancePolicy::default(),
        BlockServingResourceGateInput {
            eligibility,
            status,
            queue_pressure: QueuePressureInput {
                active_permission_effects: input.active_permission_effects.clone(),
                inactive_permission_effects: input.inactive_permission_effects.clone(),
                ..QueuePressureInput::default()
            },
            request_pressure: RequestPressureInput {
                requested_blocks_in_flight: input.requested_blocks_in_flight,
                requested_txids_in_flight: input.requested_txids_in_flight,
                requested_wtxids_in_flight: input.requested_wtxids_in_flight,
                active_permission_effects: input.active_permission_effects,
                inactive_permission_effects: input.inactive_permission_effects,
                ..RequestPressureInput::default()
            },
            maybe_timeout: None,
            maybe_churn: None,
            maybe_repeated_failure: None,
            reconnect: ReconnectSuppressionInput::default(),
            maybe_cleanup: None,
        },
    );

    if input.inventory_type == InventoryType::CompactBlock {
        return missing(BlockServingOutcomeLabel::BlockServingSuppressed);
    }

    if !matches!(
        input.inventory_type,
        InventoryType::Block | InventoryType::WitnessBlock
    ) {
        return missing(BlockServingOutcomeLabel::BlockStatusUnavailable);
    }

    if !gate.allow_storage_read || !gate.may_serve_block {
        return missing(gate.label);
    }

    let Some(block) = lookup_block(input.block_hash) else {
        return missing(BlockServingOutcomeLabel::BlockStatusUnavailable);
    };

    ManagedBlockServeDecision {
        label: BlockServingOutcomeLabel::BlockServingEligible,
        maybe_block: Some(block),
        missing_inventory: false,
    }
}

fn missing(label: BlockServingOutcomeLabel) -> ManagedBlockServeDecision {
    ManagedBlockServeDecision {
        label,
        maybe_block: None,
        missing_inventory: true,
    }
}

#[cfg(test)]
mod tests {
    use core::cell::Cell;

    use open_bitcoin_core::primitives::Block;
    use open_bitcoin_network::{
        BlockRelayActivationPolicy, BlockServingActivationConfig, BlockServingChainPosition,
        BlockServingDataAvailability, BlockServingOutcomeLabel, BlockServingValidationState,
        PeerConnectionClass,
    };

    use super::{ManagedBlockServeInput, serve_managed_block_request};

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
}
