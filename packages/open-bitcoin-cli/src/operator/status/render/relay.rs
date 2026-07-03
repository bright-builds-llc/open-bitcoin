// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use open_bitcoin_node::status::{
    MempoolStatus,
    relay_evidence::{
        RelayCapabilityEvidence, RelayEvidenceCapability, RelayEvidenceCounters,
        RelayEvidenceField, RelayRecoveryCounters,
    },
};

pub(super) fn relay_evidence_lines(mempool: &MempoolStatus) -> Vec<String> {
    vec![
        format!(
            "Relay evidence: {}",
            relay_counters_text(&mempool.relay.outcome_counters)
        ),
        format!(
            "Relay recovery: {}",
            relay_recovery_counters_text(&mempool.relay.recovery_counters)
        ),
        format!(
            "Mempool evidence: {}",
            relay_capability_text(&mempool.relay.mempool_admission)
        ),
        format!(
            "Relay local submission: {}",
            relay_capability_text(&mempool.relay.local_submission)
        ),
        format!(
            "Relay fanout: {}",
            relay_capability_text(&mempool.relay.fanout)
        ),
        format!(
            "Relay serving: {}",
            relay_capability_text(&mempool.relay.serving)
        ),
        format!(
            "Rebroadcast: deferred: {}",
            relay_capability_text(&mempool.relay.rebroadcast)
        ),
        format!(
            "Public relay: {}",
            relay_capability_text(&mempool.relay.public_relay)
        ),
    ]
}

fn relay_counters_text(value: &RelayEvidenceField<RelayEvidenceCounters>) -> String {
    match value {
        RelayEvidenceField::Implemented(counters) => format!(
            "accepted_count={} rejected_count={} orphaned_count={} requested_count={} served_count={} announced_count={} suppressed_count={} evicted_count={} expired_count={} rebroadcast_deferred_count={}",
            counters.accepted_count,
            counters.rejected_count,
            counters.orphaned_count,
            counters.requested_count,
            counters.served_count,
            counters.announced_count,
            counters.suppressed_count,
            counters.evicted_count,
            counters.expired_count,
            counters.rebroadcast_deferred_count
        ),
        RelayEvidenceField::Unavailable { reason } => format!("Unavailable: {reason}"),
        RelayEvidenceField::Deferred { reason } => format!("Deferred: {reason}"),
        RelayEvidenceField::IntentionallyDifferent { reason } => {
            format!("Intentionally different: {reason}")
        }
    }
}

fn relay_recovery_counters_text(value: &RelayEvidenceField<RelayRecoveryCounters>) -> String {
    match value {
        RelayEvidenceField::Implemented(counters) => format!(
            "recovered_count={} dropped_confirmed_count={} dropped_duplicate_count={} dropped_missing_parent_count={} dropped_policy_incompatible_count={} dropped_evicted_count={}",
            counters.recovered_count,
            counters.dropped_confirmed_count,
            counters.dropped_duplicate_count,
            counters.dropped_missing_parent_count,
            counters.dropped_policy_incompatible_count,
            counters.dropped_evicted_count
        ),
        RelayEvidenceField::Unavailable { reason } => format!("Unavailable: {reason}"),
        RelayEvidenceField::Deferred { reason } => format!("Deferred: {reason}"),
        RelayEvidenceField::IntentionallyDifferent { reason } => {
            format!("Intentionally different: {reason}")
        }
    }
}

fn relay_capability_text(value: &RelayEvidenceField<RelayCapabilityEvidence>) -> String {
    match value {
        RelayEvidenceField::Implemented(evidence) => {
            format!(
                "Implemented: {}",
                relay_capability_name(evidence.capability)
            )
        }
        RelayEvidenceField::Unavailable { reason } => format!("Unavailable: {reason}"),
        RelayEvidenceField::Deferred { reason } => format!("Deferred: {reason}"),
        RelayEvidenceField::IntentionallyDifferent { reason } => {
            format!("Intentionally different: {reason}")
        }
    }
}

fn relay_capability_name(capability: RelayEvidenceCapability) -> &'static str {
    match capability {
        RelayEvidenceCapability::MempoolAdmission => "mempool_admission",
        RelayEvidenceCapability::LocalSubmissionRelay => "local_submission_relay",
        RelayEvidenceCapability::RelayFanout => "relay_fanout",
        RelayEvidenceCapability::RelayServing => "relay_serving",
        RelayEvidenceCapability::Rebroadcast => "rebroadcast",
        RelayEvidenceCapability::PublicRelayReadiness => "public_relay_readiness",
    }
}
