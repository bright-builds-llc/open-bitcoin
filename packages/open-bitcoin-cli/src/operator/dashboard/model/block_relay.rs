// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use open_bitcoin_node::status::{
    BlockServingActivationEvidence, BlockServingEligibilityCounters, BlockServingStatusCounters,
    CompactRelayAnnouncementCounters, CompactRelayCleanupCounters, CompactRelayFallbackCounters,
    CompactRelayInFlightCounters, CompactRelayMissingTransactionCounters,
    CompactRelayNegotiationCounters, CompactRelayReconstructionCounters, FieldAvailability,
    OpenBitcoinStatusSnapshot,
};

use super::{DashboardRow, row};

pub(super) fn block_relay_rows(snapshot: &OpenBitcoinStatusSnapshot) -> Vec<DashboardRow> {
    let status = &snapshot.block_relay;
    vec![
        row(
            "Block relay activation",
            activation_text(&status.block_serving.activation),
        ),
        row(
            "Block relay eligibility",
            block_serving_eligibility_text(&status.block_serving.eligibility),
        ),
        row(
            "Block relay status",
            block_serving_status_text(&status.block_serving.status),
        ),
        row(
            "Compact negotiation",
            compact_negotiation_text(&status.negotiation),
        ),
        row(
            "Compact announcement",
            compact_announcement_text(&status.announcement),
        ),
        row(
            "Compact reconstruction",
            compact_reconstruction_text(&status.reconstruction),
        ),
        row(
            "Compact missing tx",
            compact_missing_transaction_text(&status.missing_transaction),
        ),
        row("Compact fallback", compact_fallback_text(&status.fallback)),
        row(
            "Compact in-flight",
            compact_in_flight_text(&status.in_flight),
        ),
        row("Compact cleanup", compact_cleanup_text(&status.cleanup)),
    ]
}

fn activation_text(value: &FieldAvailability<BlockServingActivationEvidence>) -> String {
    match value {
        FieldAvailability::Available(value) => format!(
            "block_serving_enabled={} compact_relay_enabled={}",
            value.block_serving_enabled, value.compact_relay_enabled
        ),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn block_serving_eligibility_text(
    value: &FieldAvailability<BlockServingEligibilityCounters>,
) -> String {
    match value {
        FieldAvailability::Available(value) => format!(
            "eligible_peer_count={} ineligible_peer_count={} disabled_count={} activation_required_count={} inbound_serving_required_count={} permission_required_count={} protected_not_serving_count={} status_unavailable_count={} permission_effect_inactive_count={}",
            value.eligible_peer_count,
            value.ineligible_peer_count,
            value.disabled_count,
            value.activation_required_count,
            value.inbound_serving_required_count,
            value.permission_required_count,
            value.protected_not_serving_count,
            value.status_unavailable_count,
            value.permission_effect_inactive_count
        ),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn block_serving_status_text(value: &FieldAvailability<BlockServingStatusCounters>) -> String {
    match value {
        FieldAvailability::Available(value) => format!(
            "validated_count={} available_count={} stale_count={} side_chain_count={} pruned_count={} unavailable_count={} unvalidated_count={} unknown_count={} suppressed_count={}",
            value.validated_count,
            value.available_count,
            value.stale_count,
            value.side_chain_count,
            value.pruned_count,
            value.unavailable_count,
            value.unvalidated_count,
            value.unknown_count,
            value.suppressed_count
        ),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn compact_negotiation_text(value: &FieldAvailability<CompactRelayNegotiationCounters>) -> String {
    match value {
        FieldAvailability::Available(value) => format!(
            "version2_high_bandwidth_count={} version2_low_bandwidth_count={} unsupported_version_count={}",
            value.version2_high_bandwidth_count,
            value.version2_low_bandwidth_count,
            value.unsupported_version_count
        ),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn compact_announcement_text(
    value: &FieldAvailability<CompactRelayAnnouncementCounters>,
) -> String {
    match value {
        FieldAvailability::Available(value) => format!(
            "compact_announced_count={} compact_headers_fallback_count={} compact_inventory_fallback_count={} compact_suppressed_count={}",
            value.compact_announced_count,
            value.compact_headers_fallback_count,
            value.compact_inventory_fallback_count,
            value.compact_suppressed_count
        ),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn compact_reconstruction_text(
    value: &FieldAvailability<CompactRelayReconstructionCounters>,
) -> String {
    match value {
        FieldAvailability::Available(value) => format!(
            "compact_reconstructed_count={} compact_reconstruction_failed_count={} compact_malformed_count={}",
            value.compact_reconstructed_count,
            value.compact_reconstruction_failed_count,
            value.compact_malformed_count
        ),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn compact_missing_transaction_text(
    value: &FieldAvailability<CompactRelayMissingTransactionCounters>,
) -> String {
    match value {
        FieldAvailability::Available(value) => format!(
            "compact_missing_tx_requested_count={} compact_missing_tx_suppressed_count={}",
            value.compact_missing_tx_requested_count, value.compact_missing_tx_suppressed_count
        ),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn compact_fallback_text(value: &FieldAvailability<CompactRelayFallbackCounters>) -> String {
    match value {
        FieldAvailability::Available(value) => format!(
            "compact_fallback_count={} compact_timeout_count={}",
            value.compact_fallback_count, value.compact_timeout_count
        ),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn compact_in_flight_text(value: &FieldAvailability<CompactRelayInFlightCounters>) -> String {
    match value {
        FieldAvailability::Available(value) => format!(
            "in_flight_count={} getblocktxn_in_flight_count={} peers_with_in_flight_count={}",
            value.in_flight_count,
            value.getblocktxn_in_flight_count,
            value.peers_with_in_flight_count
        ),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}

fn compact_cleanup_text(value: &FieldAvailability<CompactRelayCleanupCounters>) -> String {
    match value {
        FieldAvailability::Available(value) => format!(
            "compact_cleanup_count={} compact_download_peer_disconnect_count={} compact_download_timeout_count={} compact_download_reorg_count={} compact_download_restart_count={} compact_download_block_connected_count={}",
            value.compact_cleanup_count,
            value.compact_download_peer_disconnect_count,
            value.compact_download_timeout_count,
            value.compact_download_reorg_count,
            value.compact_download_restart_count,
            value.compact_download_block_connected_count
        ),
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    }
}
