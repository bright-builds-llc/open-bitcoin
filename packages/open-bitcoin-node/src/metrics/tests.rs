// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::{
    MetricKind, MetricRetentionPolicy, MetricSample, MetricsAvailability, MetricsStatus,
    append_and_prune_metric_samples, block_relay_metric_samples, inbound_metric_samples,
    relay_metric_samples,
};
use crate::status::{
    BlockRelayEvidenceStatus, FieldAvailability, InboundHandshakeStatusCounts,
    InboundPeerServingStatus,
    relay_evidence::{RelayEvidenceCounters, RelayEvidenceStatus},
    relay_evidence::{RelayEvidenceField, RelayRecoveryCounters},
};

fn inbound_status_fixture() -> InboundPeerServingStatus {
    InboundPeerServingStatus {
        listener_state: "ready".to_string(),
        bound_endpoints: Vec::new(),
        preflight_reason: "ready".to_string(),
        admitted_inbound_peers: 1,
        rejected_inbound_peers: 2,
        handshake: InboundHandshakeStatusCounts::default(),
        duplicate_rejects: 5,
        self_connection_rejects: 6,
        cap_rejects: 3,
        reserved_slot_rejects: 4,
        latest_admission_event: FieldAvailability::unavailable("no admission event"),
        permissioned_inbound_peers: 7,
        protected_inbound_peers: 8,
        permission_class: "ordinary_inbound".to_string(),
        active_permission_effects: Vec::new(),
        inactive_permission_effects: Vec::new(),
        inactive_permission_effect_observations: 9,
        permission_validation_failures: 10,
        latest_permission_decision: FieldAvailability::unavailable("no permission decision"),
        local_advertisement_candidates: Vec::new(),
        suppressed_advertisements: Vec::new(),
        getaddr_responses_served: 0,
        getaddr_requests_suppressed: 0,
        learned_address_entries: 0,
        learned_address_rejections: 0,
        latest_address_decision: FieldAvailability::unavailable("no address decision"),
        eviction_candidates_evaluated: 11,
        disconnects_requested: 12,
        discouraged_peers: 0,
        active_bans: 13,
        expired_bans: 0,
        manual_unbans: 0,
        misbehavior_observations: 14,
        protected_no_actions: 15,
        latest_peer_policy_decision: FieldAvailability::unavailable("no peer policy decision"),
        resource_pressure_events: 16,
        read_queue_pressure_events: 17,
        write_queue_pressure_events: 18,
        request_cap_events: 19,
        payload_rejections: 20,
        timeout_disconnects: 21,
        churn_rejections: 22,
        reconnect_suppressions: 23,
        latest_resource_governance_decision: FieldAvailability::unavailable(
            "no resource governance decision",
        ),
    }
}

mod contracts;
mod inbound_projection;
mod relay_projection;
mod retention_and_availability;
mod sample_status;
