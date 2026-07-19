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

use open_bitcoin_network::InboundResourceEvent;
use open_bitcoin_node::{
    logging::{StructuredLogError, writer::append_structured_log_record},
    network::ManagedResourceGovernanceInfo,
    status::{
        FieldAvailability, INBOUND_RESOURCE_DECISION_UNAVAILABLE_REASON,
        InboundResourceGovernanceEvent,
    },
};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::inbound_listener::InboundListenerEvidence;

use super::ManagedRpcContext;

impl ManagedRpcContext {
    pub fn record_inbound_resource_event(&mut self, event: InboundResourceEvent) {
        if self
            .record_inbound_resource_event_at(event, current_unix_seconds())
            .is_err()
        {
            self.resource_governance_log_write_failures = self
                .resource_governance_log_write_failures
                .saturating_add(1);
        }
    }

    pub(crate) fn record_inbound_resource_event_at(
        &mut self,
        event: InboundResourceEvent,
        timestamp_unix_seconds: u64,
    ) -> Result<(), StructuredLogError> {
        if let Some(evidence) = &mut self.maybe_inbound_listener_evidence {
            evidence.record_resource_event(event.clone());
        }
        if self
            .network
            .record_resource_governance_event(event)
            .is_err()
        {
            self.resource_governance_log_write_failures = self
                .resource_governance_log_write_failures
                .saturating_add(1);
            return Ok(());
        }
        let Ok(resource_info) = self.network.resource_governance_info() else {
            self.resource_governance_log_write_failures = self
                .resource_governance_log_write_failures
                .saturating_add(1);
            return Ok(());
        };
        let Some(record) = resource_info.maybe_structured_log_record(timestamp_unix_seconds) else {
            return Ok(());
        };
        let Some(log_dir) = &self.maybe_resource_governance_log_dir else {
            return Ok(());
        };

        append_structured_log_record(log_dir, &record, self.resource_governance_log_retention)?;
        Ok(())
    }
}

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

pub(super) fn current_unix_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0)
}

fn usize_to_u32(value: usize) -> u32 {
    u32::try_from(value).unwrap_or(u32::MAX)
}
