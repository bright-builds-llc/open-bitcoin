// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use open_bitcoin_node::status::{
    BasicIndexOperatorStatus, BasicIndexUnavailableReason, FieldAvailability,
};

pub(super) fn redact_basic_index(status: &mut BasicIndexOperatorStatus) {
    sanitize(&mut status.configured_indexing);
    sanitize(&mut status.configured_peer_serving);
    sanitize(&mut status.peer_scope);
    sanitize(&mut status.operational_state);
    sanitize(&mut status.initial_sync_complete);
    sanitize(&mut status.maybe_accepted_height);
    sanitize(&mut status.maybe_processed_height);
    sanitize(&mut status.maybe_safe_durable_height);
    sanitize(&mut status.maybe_lag_blocks);
    sanitize(&mut status.maybe_failure);
    sanitize(&mut status.retained_availability);
    sanitize(&mut status.retention);
    sanitize(&mut status.protection);
    sanitize(&mut status.next_action);
}

fn sanitize<T>(value: &mut FieldAvailability<T>) {
    if let FieldAvailability::Unavailable { reason } = value {
        *reason = BasicIndexUnavailableReason::normalize(reason).into();
    }
}
