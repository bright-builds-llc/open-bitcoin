// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Shared inbound peer serving status contracts.

use super::FieldAvailability;
use serde::{Deserialize, Serialize};

/// Default unavailable reason when inbound listener evidence has not been projected.
pub const INBOUND_STATUS_UNAVAILABLE_REASON: &str = "inbound listener evidence unavailable";

/// Conservative default for peer status snapshots without inbound serving evidence.
pub fn inbound_status_unavailable() -> FieldAvailability<InboundPeerServingStatus> {
    FieldAvailability::unavailable(INBOUND_STATUS_UNAVAILABLE_REASON)
}

/// Bounded handshake lifecycle counts for inbound peers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct InboundHandshakeStatusCounts {
    pub awaiting_version: u32,
    pub awaiting_verack: u32,
    pub established: u32,
    pub disconnected: u32,
}

/// Latest bounded inbound admission event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InboundAdmissionEvent {
    pub outcome: String,
    pub reason: String,
    pub slot_class: String,
    pub message: String,
}

/// Shared inbound listener and admission evidence under peer status.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InboundPeerServingStatus {
    pub listener_state: String,
    pub bound_endpoints: Vec<String>,
    pub preflight_reason: String,
    pub admitted_inbound_peers: u32,
    pub rejected_inbound_peers: u32,
    pub handshake: InboundHandshakeStatusCounts,
    pub duplicate_rejects: u32,
    pub self_connection_rejects: u32,
    pub cap_rejects: u32,
    pub reserved_slot_rejects: u32,
    pub latest_admission_event: FieldAvailability<InboundAdmissionEvent>,
}

#[cfg(test)]
mod tests;
