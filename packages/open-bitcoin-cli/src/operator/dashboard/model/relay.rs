// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use open_bitcoin_node::status::{
    OpenBitcoinStatusSnapshot,
    relay_evidence::{
        RelayCapabilityEvidence, RelayEvidenceCapability, RelayEvidenceCounters, RelayEvidenceField,
    },
};

use super::{DashboardRow, row, u64_availability, wallet_freshness, wallet_scan_progress};

pub(super) fn mempool_and_wallet_rows(snapshot: &OpenBitcoinStatusSnapshot) -> Vec<DashboardRow> {
    vec![
        row(
            "Mempool",
            u64_availability(&snapshot.mempool.transactions, "transactions"),
        ),
        row(
            "Relay evidence",
            relay_counters_text(&snapshot.mempool.relay.outcome_counters),
        ),
        row(
            "Mempool evidence",
            relay_capability_text(&snapshot.mempool.relay.mempool_admission),
        ),
        row(
            "Relay local submission",
            relay_capability_text(&snapshot.mempool.relay.local_submission),
        ),
        row(
            "Relay fanout",
            relay_capability_text(&snapshot.mempool.relay.fanout),
        ),
        row(
            "Relay serving",
            relay_capability_text(&snapshot.mempool.relay.serving),
        ),
        row(
            "Rebroadcast: deferred",
            relay_capability_text(&snapshot.mempool.relay.rebroadcast),
        ),
        row(
            "Public relay",
            relay_capability_text(&snapshot.mempool.relay.public_relay),
        ),
        row(
            "Wallet",
            u64_availability(&snapshot.wallet.trusted_balance_sats, "trusted sats"),
        ),
        row("Freshness", wallet_freshness(&snapshot.wallet.freshness)),
        row("Scan", wallet_scan_progress(&snapshot.wallet.scan_progress)),
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
