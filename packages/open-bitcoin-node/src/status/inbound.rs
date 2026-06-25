// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Shared inbound peer serving status contracts.

use super::FieldAvailability;
use serde::{Deserialize, Serialize};

/// Default unavailable reason when inbound listener evidence has not been projected.
pub const INBOUND_STATUS_UNAVAILABLE_REASON: &str = "inbound listener evidence unavailable";

/// Default unavailable reason when no permission decision has been projected.
pub const INBOUND_PERMISSION_DECISION_UNAVAILABLE_REASON: &str =
    "inbound permission decision evidence unavailable";

/// Conservative default for peer status snapshots without inbound serving evidence.
pub fn inbound_status_unavailable() -> FieldAvailability<InboundPeerServingStatus> {
    FieldAvailability::unavailable(INBOUND_STATUS_UNAVAILABLE_REASON)
}

/// Low-cardinality permission labels safe for shared status and support surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InboundPermissionEvidence {
    pub permission_class: String,
    pub active_permission_effects: Vec<String>,
    pub inactive_permission_effects: Vec<String>,
}

impl InboundPermissionEvidence {
    pub fn ordinary() -> Self {
        Self {
            permission_class: "ordinary_inbound".to_string(),
            active_permission_effects: Vec::new(),
            inactive_permission_effects: Vec::new(),
        }
    }
}

impl Default for InboundPermissionEvidence {
    fn default() -> Self {
        Self::ordinary()
    }
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

/// Latest bounded inbound permission decision event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InboundPermissionDecisionEvent {
    pub outcome: String,
    pub reason: String,
    pub permission_class: String,
    pub active_permission_effects: Vec<String>,
    pub inactive_permission_effects: Vec<String>,
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
    #[serde(default)]
    pub permissioned_inbound_peers: u32,
    #[serde(default)]
    pub protected_inbound_peers: u32,
    #[serde(default = "ordinary_permission_class")]
    pub permission_class: String,
    #[serde(default)]
    pub active_permission_effects: Vec<String>,
    #[serde(default)]
    pub inactive_permission_effects: Vec<String>,
    #[serde(default = "latest_permission_decision_unavailable")]
    pub latest_permission_decision: FieldAvailability<InboundPermissionDecisionEvent>,
}

fn ordinary_permission_class() -> String {
    InboundPermissionEvidence::ordinary().permission_class
}

fn latest_permission_decision_unavailable() -> FieldAvailability<InboundPermissionDecisionEvent> {
    FieldAvailability::unavailable(INBOUND_PERMISSION_DECISION_UNAVAILABLE_REASON)
}

#[cfg(test)]
mod tests;
