// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/test/functional/p2p_getdata.py

use open_bitcoin_codec::BlockTransactions;
use open_bitcoin_core::primitives::{Block, BlockHash, InventoryType, InventoryVector};
use open_bitcoin_network::{
    BlockRelayActivationPolicy, BlockServingChainPosition, BlockServingDataAvailability,
    BlockServingEligibilityInput, BlockServingEligibilityReason, BlockServingOutcomeLabel,
    BlockServingResourceGateInput, BlockServingStatusFacts, BlockServingStatusLabel,
    BlockServingValidationState, InactivePermissionEffectLabel, PeerConnectionClass,
    PermissionEffectLabel, QueuePressureInput, ReconnectSuppressionInput, RequestPressureInput,
    ResourceGovernancePolicy, classify_block_serving_eligibility, classify_block_serving_status,
    evaluate_block_serving_resource_gate,
};

use super::{ManagedBlockSerializationMode, ManagedBlockServeCompletionOutcome};

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
    pub presence: BlockServingPresenceFacts,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct BlockServingPresenceFacts {
    pub payload_present: bool,
    pub index_known: bool,
    pub validated_on_active_chain: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ManagedBlockServeDecision {
    pub label: BlockServingOutcomeLabel,
    pub status_label: BlockServingStatusLabel,
    pub eligibility_reason: BlockServingEligibilityReason,
    pub maybe_block: Option<Block>,
    pub missing_inventory: bool,
    pub presence: BlockServingPresenceFacts,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManagedBlockServeIntent {
    request: InventoryVector,
    block_hash: BlockHash,
    serialization_mode: ManagedBlockSerializationMode,
    eligible_decision: ManagedBlockServeDecision,
}

impl ManagedBlockServeIntent {
    pub const fn request(&self) -> &InventoryVector {
        &self.request
    }

    pub const fn block_hash(&self) -> BlockHash {
        self.block_hash
    }

    pub const fn serialization_mode(&self) -> ManagedBlockSerializationMode {
        self.serialization_mode
    }

    pub fn completion(
        &self,
        outcome: ManagedBlockServeCompletionOutcome,
    ) -> ManagedBlockServeCompletion {
        let decision = match outcome {
            ManagedBlockServeCompletionOutcome::LookupUnavailable => missing(
                BlockServingOutcomeLabel::BlockStatusUnavailable,
                BlockServingStatusLabel::Unavailable,
                self.eligible_decision.eligibility_reason,
                BlockServingPresenceFacts {
                    payload_present: false,
                    index_known: self.eligible_decision.presence.index_known,
                    validated_on_active_chain: self
                        .eligible_decision
                        .presence
                        .validated_on_active_chain,
                },
            ),
            ManagedBlockServeCompletionOutcome::TransportFailed
            | ManagedBlockServeCompletionOutcome::Written => self.eligible_decision.clone(),
        };
        ManagedBlockServeCompletion {
            request: self.request.clone(),
            outcome,
            decision,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManagedBlockServeCompletion {
    request: InventoryVector,
    outcome: ManagedBlockServeCompletionOutcome,
    decision: ManagedBlockServeDecision,
}

impl ManagedBlockServeCompletion {
    pub(super) const fn request(&self) -> &InventoryVector {
        &self.request
    }

    pub(super) const fn decision(&self) -> &ManagedBlockServeDecision {
        &self.decision
    }

    pub const fn records_served_effect(&self) -> bool {
        matches!(self.outcome, ManagedBlockServeCompletionOutcome::Written)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum ManagedBlockServeGateDecision {
    Serve(ManagedBlockServeIntent),
    Deny(ManagedBlockServeDecision),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum CompactBlockTxnServeOutcome {
    Served,
    Suppressed(CompactBlockTxnServeCause),
    Malformed(CompactBlockTxnServeCause),
}

impl CompactBlockTxnServeOutcome {
    pub(super) const fn label(self) -> &'static str {
        match self {
            Self::Served => "compact_missing_tx_served",
            Self::Suppressed(_) => "compact_missing_tx_serve_suppressed",
            Self::Malformed(_) => "compact_missing_tx_malformed",
        }
    }

    pub(super) const fn cause(self) -> &'static str {
        match self {
            Self::Served => "compact_getblocktxn_served",
            Self::Suppressed(cause) | Self::Malformed(cause) => cause.as_str(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum CompactBlockTxnServeCause {
    Ineligible,
    Unavailable,
    RequestLimited,
    IndexOutOfBounds,
}

impl CompactBlockTxnServeCause {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Ineligible => "compact_getblocktxn_ineligible",
            Self::Unavailable => "compact_getblocktxn_unavailable",
            Self::RequestLimited => "compact_getblocktxn_request_limited",
            Self::IndexOutOfBounds => "compact_getblocktxn_index_out_of_bounds",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum ManagedCompactBlockTxnServeDecision {
    Served(BlockTransactions),
    Suppressed(CompactBlockTxnServeCause),
    Malformed(CompactBlockTxnServeCause),
}

impl ManagedCompactBlockTxnServeDecision {
    pub(super) const fn outcome(&self) -> CompactBlockTxnServeOutcome {
        match self {
            Self::Served(_) => CompactBlockTxnServeOutcome::Served,
            Self::Suppressed(cause) => CompactBlockTxnServeOutcome::Suppressed(*cause),
            Self::Malformed(cause) => CompactBlockTxnServeOutcome::Malformed(*cause),
        }
    }
}

pub(super) fn serve_managed_compact_block_transactions(
    input: ManagedBlockServeInput,
    indexes: &[u16],
    lookup_block: impl FnOnce(BlockHash) -> Option<Block>,
) -> ManagedCompactBlockTxnServeDecision {
    if !input.activation.compact_relay.enabled {
        return compact_block_txn_missing(CompactBlockTxnServeCause::Ineligible);
    }

    let block_hash = input.block_hash;
    let decision = serve_managed_block_request(input, lookup_block);
    let Some(block) = decision.maybe_block else {
        let cause = match decision.label {
            BlockServingOutcomeLabel::BlockStatusUnavailable => {
                CompactBlockTxnServeCause::Unavailable
            }
            BlockServingOutcomeLabel::BlockRequestCapReached
            | BlockServingOutcomeLabel::BlockInFlightLimitStillReached => {
                CompactBlockTxnServeCause::RequestLimited
            }
            _ => CompactBlockTxnServeCause::Ineligible,
        };
        return compact_block_txn_missing(cause);
    };

    let mut transactions = Vec::with_capacity(indexes.len());
    for index in indexes {
        let Some(transaction) = block.transactions.get(usize::from(*index)) else {
            return ManagedCompactBlockTxnServeDecision::Malformed(
                CompactBlockTxnServeCause::IndexOutOfBounds,
            );
        };
        transactions.push(transaction.clone());
    }

    ManagedCompactBlockTxnServeDecision::Served(BlockTransactions {
        block_hash,
        transactions,
    })
}

fn compact_block_txn_missing(
    cause: CompactBlockTxnServeCause,
) -> ManagedCompactBlockTxnServeDecision {
    ManagedCompactBlockTxnServeDecision::Suppressed(cause)
}

pub(super) fn serve_managed_block_request(
    input: ManagedBlockServeInput,
    lookup_block: impl FnOnce(BlockHash) -> Option<Block>,
) -> ManagedBlockServeDecision {
    let intent = match gate_managed_block_request(input) {
        ManagedBlockServeGateDecision::Serve(intent) => intent,
        ManagedBlockServeGateDecision::Deny(decision) => return decision,
    };

    let Some(block) = lookup_block(intent.block_hash()) else {
        return intent
            .completion(ManagedBlockServeCompletionOutcome::LookupUnavailable)
            .decision;
    };

    ManagedBlockServeDecision {
        maybe_block: Some(block),
        ..intent.eligible_decision
    }
}

pub(super) fn gate_managed_block_request(
    input: ManagedBlockServeInput,
) -> ManagedBlockServeGateDecision {
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
            eligibility: eligibility.clone(),
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

    let maybe_serialization_mode = match input.inventory_type {
        InventoryType::Block => Some(ManagedBlockSerializationMode::Block),
        InventoryType::WitnessBlock => Some(ManagedBlockSerializationMode::WitnessBlock),
        InventoryType::CompactBlock if input.activation.compact_relay.enabled => {
            Some(ManagedBlockSerializationMode::CompactBlock)
        }
        InventoryType::CompactBlock => {
            return ManagedBlockServeGateDecision::Deny(missing(
                BlockServingOutcomeLabel::BlockServingSuppressed,
                status.label,
                eligibility.reason,
                input.presence,
            ));
        }
        _ => None,
    };
    let Some(serialization_mode) = maybe_serialization_mode else {
        return ManagedBlockServeGateDecision::Deny(missing(
            BlockServingOutcomeLabel::BlockStatusUnavailable,
            status.label,
            eligibility.reason,
            input.presence,
        ));
    };

    if !gate.allow_storage_read || !gate.may_serve_block {
        return ManagedBlockServeGateDecision::Deny(missing(
            gate.label,
            status.label,
            eligibility.reason,
            input.presence,
        ));
    }

    let eligible_decision = ManagedBlockServeDecision {
        label: BlockServingOutcomeLabel::BlockServingEligible,
        status_label: status.label,
        eligibility_reason: eligibility.reason,
        maybe_block: None,
        missing_inventory: false,
        presence: input.presence,
    };
    ManagedBlockServeGateDecision::Serve(ManagedBlockServeIntent {
        request: InventoryVector {
            inventory_type: input.inventory_type,
            object_hash: input.block_hash.into(),
        },
        block_hash: input.block_hash,
        serialization_mode,
        eligible_decision,
    })
}

fn missing(
    label: BlockServingOutcomeLabel,
    status_label: BlockServingStatusLabel,
    eligibility_reason: BlockServingEligibilityReason,
    presence: BlockServingPresenceFacts,
) -> ManagedBlockServeDecision {
    ManagedBlockServeDecision {
        label,
        status_label,
        eligibility_reason,
        maybe_block: None,
        missing_inventory: true,
        presence,
    }
}

#[cfg(test)]
mod tests;
