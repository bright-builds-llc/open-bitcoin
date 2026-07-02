// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use open_bitcoin_node::status::{
    FieldAvailability, MempoolStatus,
    relay_evidence::{
        RelayCapabilityEvidence, RelayEvidenceCapability, RelayEvidenceCounters, RelayEvidenceField,
    },
};

const RELAY_MEMPOOL_NEXT_ACTION: &str = "Treat relay/mempool evidence as bounded local status only and local troubleshooting/parity-review evidence only; do not treat it as public propagation, compact-block relay, production-readiness proof, a release validator, public-network proof, production-service proof, production full-node readiness proof, or production-funds wallet safety proof.";

pub(super) fn push_relay_mempool_evidence(output: &mut String, mempool: &MempoolStatus) {
    output.push_str("\n## Relay and Mempool Evidence\n\n");
    output.push_str(&format!(
        "- Mempool: {}\n",
        mempool_transactions_text(&mempool.transactions)
    ));
    output.push_str(&format!(
        "- Relay evidence: {}\n",
        relay_counters_text(&mempool.relay.outcome_counters)
    ));
    output.push_str(&format!(
        "- Mempool evidence: {}\n",
        relay_capability_text(&mempool.relay.mempool_admission)
    ));
    output.push_str(&format!(
        "- Relay local submission: {}\n",
        relay_capability_text(&mempool.relay.local_submission)
    ));
    output.push_str(&format!(
        "- Relay fanout: {}\n",
        relay_capability_text(&mempool.relay.fanout)
    ));
    output.push_str(&format!(
        "- Relay serving: {}\n",
        relay_capability_text(&mempool.relay.serving)
    ));
    output.push_str(&format!(
        "- Rebroadcast: deferred: {}\n",
        relay_capability_text(&mempool.relay.rebroadcast)
    ));
    output.push_str(&format!(
        "- Public relay: {}\n",
        relay_capability_text(&mempool.relay.public_relay)
    ));
    output.push_str(&format!("- Next action: {RELAY_MEMPOOL_NEXT_ACTION}\n"));
}

fn mempool_transactions_text(value: &FieldAvailability<u64>) -> String {
    match value {
        FieldAvailability::Available(transactions) => format!("transactions={transactions}"),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
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
