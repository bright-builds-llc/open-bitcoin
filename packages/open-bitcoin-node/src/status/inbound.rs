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

/// Default unavailable reason when address-boundary evidence has not been projected.
pub const INBOUND_ADDRESS_DECISION_UNAVAILABLE_REASON: &str =
    "inbound address boundary evidence unavailable";

/// Default unavailable reason when peer-policy evidence has not been projected.
pub const INBOUND_PEER_POLICY_DECISION_UNAVAILABLE_REASON: &str =
    "inbound peer policy evidence unavailable";

/// Default unavailable reason when resource-governance evidence has not been projected.
pub const INBOUND_RESOURCE_DECISION_UNAVAILABLE_REASON: &str =
    "inbound resource governance evidence unavailable";

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

/// Bounded address evidence safe for shared status and support surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InboundAddressEvidenceEntry {
    pub source: String,
    pub network_kind: String,
    pub routability: String,
    pub freshness: String,
    pub services_bits: u64,
    pub port: u16,
    pub persistence_eligible: bool,
}

/// Latest bounded address-boundary decision event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InboundAddressDecisionEvent {
    pub outcome: String,
    pub reason: String,
    pub label: String,
    pub source: String,
    pub message: String,
}

/// Latest bounded eviction, ban, unban, or misbehavior policy decision event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InboundPeerPolicyEvent {
    pub outcome: String,
    pub reason: String,
    pub label: String,
    pub source: String,
    pub message: String,
}

/// Latest bounded resource-governance decision event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InboundResourceGovernanceEvent {
    pub outcome: String,
    pub reason: String,
    pub label: String,
    pub source: String,
    pub message: String,
    pub next_action: String,
}

type InboundResourceGovernanceAvailability = FieldAvailability<InboundResourceGovernanceEvent>;

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
    #[serde(default)]
    pub inactive_permission_effect_observations: u32,
    #[serde(default)]
    pub permission_validation_failures: u32,
    #[serde(default = "latest_permission_decision_unavailable")]
    pub latest_permission_decision: FieldAvailability<InboundPermissionDecisionEvent>,
    #[serde(default)]
    pub local_advertisement_candidates: Vec<InboundAddressEvidenceEntry>,
    #[serde(default)]
    pub suppressed_advertisements: Vec<InboundAddressDecisionEvent>,
    #[serde(default)]
    pub getaddr_responses_served: u32,
    #[serde(default)]
    pub getaddr_requests_suppressed: u32,
    #[serde(default)]
    pub learned_address_entries: u32,
    #[serde(default)]
    pub learned_address_rejections: u32,
    #[serde(default = "latest_address_decision_unavailable")]
    pub latest_address_decision: FieldAvailability<InboundAddressDecisionEvent>,
    #[serde(default)]
    pub eviction_candidates_evaluated: u32,
    #[serde(default)]
    pub disconnects_requested: u32,
    #[serde(default)]
    pub discouraged_peers: u32,
    #[serde(default)]
    pub active_bans: u32,
    #[serde(default)]
    pub expired_bans: u32,
    #[serde(default)]
    pub manual_unbans: u32,
    #[serde(default)]
    pub misbehavior_observations: u32,
    #[serde(default)]
    pub protected_no_actions: u32,
    #[serde(default = "latest_peer_policy_decision_unavailable")]
    pub latest_peer_policy_decision: FieldAvailability<InboundPeerPolicyEvent>,
    #[serde(default)]
    pub resource_pressure_events: u32,
    #[serde(default)]
    pub read_queue_pressure_events: u32,
    #[serde(default)]
    pub write_queue_pressure_events: u32,
    #[serde(default)]
    pub request_cap_events: u32,
    #[serde(default)]
    pub payload_rejections: u32,
    #[serde(default)]
    pub timeout_disconnects: u32,
    #[serde(default)]
    pub churn_rejections: u32,
    #[serde(default)]
    pub reconnect_suppressions: u32,
    #[serde(default = "latest_resource_governance_decision_unavailable")]
    pub latest_resource_governance_decision: FieldAvailability<InboundResourceGovernanceEvent>,
}

fn ordinary_permission_class() -> String {
    InboundPermissionEvidence::ordinary().permission_class
}

fn latest_permission_decision_unavailable() -> FieldAvailability<InboundPermissionDecisionEvent> {
    FieldAvailability::unavailable(INBOUND_PERMISSION_DECISION_UNAVAILABLE_REASON)
}

fn latest_address_decision_unavailable() -> FieldAvailability<InboundAddressDecisionEvent> {
    FieldAvailability::unavailable(INBOUND_ADDRESS_DECISION_UNAVAILABLE_REASON)
}

fn latest_peer_policy_decision_unavailable() -> FieldAvailability<InboundPeerPolicyEvent> {
    FieldAvailability::unavailable(INBOUND_PEER_POLICY_DECISION_UNAVAILABLE_REASON)
}

pub fn latest_resource_governance_decision_unavailable() -> InboundResourceGovernanceAvailability {
    FieldAvailability::unavailable(INBOUND_RESOURCE_DECISION_UNAVAILABLE_REASON)
}

#[cfg(test)]
mod tests;
