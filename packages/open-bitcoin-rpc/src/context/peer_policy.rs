// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/banman.cpp
// - packages/bitcoin-knots/src/net_permissions.cpp

#[cfg(test)]
use open_bitcoin_node::status::FieldAvailability;
use open_bitcoin_node::{
    logging::{
        StructuredLogError, inbound_peer_policy_log_record, writer::append_structured_log_record,
    },
    status::InboundPeerPolicyEvent,
};

use super::ManagedRpcContext;

impl ManagedRpcContext {
    pub fn record_inbound_peer_policy_event_at(
        &mut self,
        event: InboundPeerPolicyEvent,
        timestamp_unix_seconds: u64,
    ) -> Result<(), StructuredLogError> {
        let Some(log_dir) = &self.maybe_resource_governance_log_dir else {
            return Ok(());
        };
        let record = inbound_peer_policy_log_record(&event, timestamp_unix_seconds);
        append_structured_log_record(log_dir, &record, self.resource_governance_log_retention)?;
        Ok(())
    }

    #[cfg(test)]
    pub(crate) fn record_latest_inbound_peer_policy_event_at(
        &mut self,
        timestamp_unix_seconds: u64,
    ) -> Result<bool, StructuredLogError> {
        let FieldAvailability::Available(inbound) = self.current_inbound_status() else {
            return Ok(false);
        };
        let FieldAvailability::Available(event) = inbound.latest_peer_policy_decision else {
            return Ok(false);
        };
        self.record_inbound_peer_policy_event_at(event, timestamp_unix_seconds)?;
        Ok(true)
    }
}
