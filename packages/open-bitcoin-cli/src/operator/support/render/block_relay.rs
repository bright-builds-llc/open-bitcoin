// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use open_bitcoin_node::status::{
    BlockRelayEvidenceStatus, BlockServingActivationEvidence, BlockServingEligibilityCounters,
    BlockServingStatusCounters, CompactRelayAnnouncementCounters, CompactRelayCleanupCounters,
    CompactRelayFallbackCounters, CompactRelayInFlightCounters,
    CompactRelayMissingTransactionCounters, CompactRelayNegotiationCounters,
    CompactRelayReconstructionCounters, FieldAvailability,
};

const BLOCK_RELAY_NEXT_ACTION: &str = "Treat block-relay evidence as bounded local troubleshooting/parity-review evidence only; do not treat it as public block serving by default, BIP152 production readiness, public-network proof, production-service proof, production full-node readiness proof, production-funds wallet safety proof, or authorization for destructive repair.";

pub(super) fn push_block_relay_evidence(
    output: &mut String,
    block_relay: &BlockRelayEvidenceStatus,
) {
    output.push_str("\n## Block Relay Evidence\n\n");
    output.push_str(&format!(
        "- Block relay activation: {}\n",
        activation_text(&block_relay.block_serving.activation)
    ));
    output.push_str(&format!(
        "- Block relay eligibility: {}\n",
        block_serving_eligibility_text(&block_relay.block_serving.eligibility)
    ));
    output.push_str(&format!(
        "- Block relay status: {}\n",
        block_serving_status_text(&block_relay.block_serving.status)
    ));
    output.push_str(&format!(
        "- Compact negotiation: {}\n",
        compact_negotiation_text(&block_relay.negotiation)
    ));
    output.push_str(&format!(
        "- Compact announcement: {}\n",
        compact_announcement_text(&block_relay.announcement)
    ));
    output.push_str(&format!(
        "- Compact reconstruction: {}\n",
        compact_reconstruction_text(&block_relay.reconstruction)
    ));
    output.push_str(&format!(
        "- Compact missing tx: {}\n",
        compact_missing_transaction_text(&block_relay.missing_transaction)
    ));
    output.push_str(&format!(
        "- Compact fallback: {}\n",
        compact_fallback_text(&block_relay.fallback)
    ));
    output.push_str(&format!(
        "- Compact in-flight: {}\n",
        compact_in_flight_text(&block_relay.in_flight)
    ));
    output.push_str(&format!(
        "- Compact cleanup: {}\n",
        compact_cleanup_text(&block_relay.cleanup)
    ));
    output.push_str(&format!("- Next action: {BLOCK_RELAY_NEXT_ACTION}\n"));
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
