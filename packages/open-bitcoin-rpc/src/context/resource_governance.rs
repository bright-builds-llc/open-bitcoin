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

use open_bitcoin_node::{
    network::ManagedResourceGovernanceInfo,
    status::{
        FieldAvailability, INBOUND_RESOURCE_DECISION_UNAVAILABLE_REASON,
        InboundResourceGovernanceEvent,
    },
};

use crate::inbound_listener::InboundListenerEvidence;

pub(super) fn resource_governance_info(
    mut info: ManagedResourceGovernanceInfo,
    maybe_listener_evidence: Option<&InboundListenerEvidence>,
) -> ManagedResourceGovernanceInfo {
    if !info.is_empty() {
        return info;
    }

    let Some(listener_evidence) = maybe_listener_evidence else {
        return info;
    };

    info.payload_rejections = usize_to_u32(listener_evidence.resource_rejections);
    info.timeout_disconnects = usize_to_u32(listener_evidence.timeout_disconnects);
    info.churn_rejections = usize_to_u32(listener_evidence.churn_rejections);
    info.reconnect_suppressions = usize_to_u32(listener_evidence.reconnect_suppressions);

    let Some(event) = listener_evidence.maybe_latest_resource_event.clone() else {
        return info;
    };

    if info.is_empty() {
        info.record_event(event);
        return info;
    }

    info.maybe_latest_resource_governance_decision = Some(InboundResourceGovernanceEvent {
        outcome: event.outcome,
        reason: event.reason,
        label: event.label,
        source: event.source,
        message: event.message,
        next_action: event.next_action,
    });
    info
}

pub(super) fn latest_resource_governance_decision(
    info: &ManagedResourceGovernanceInfo,
) -> FieldAvailability<InboundResourceGovernanceEvent> {
    info.maybe_latest_resource_governance_decision
        .clone()
        .map(FieldAvailability::available)
        .unwrap_or_else(|| {
            FieldAvailability::unavailable(INBOUND_RESOURCE_DECISION_UNAVAILABLE_REASON)
        })
}

fn usize_to_u32(value: usize) -> u32 {
    u32::try_from(value).unwrap_or(u32::MAX)
}
