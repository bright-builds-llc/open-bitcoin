// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use open_bitcoin_node::status::{
    CacheSizeLabel, ChainstateDurabilityEvidence, FieldAvailability, OpenBitcoinStatusSnapshot,
};

use super::{DashboardRow, row};

const COINS_BEST_BLOCK_UNAVAILABLE_REASON: &str = "coins best-block unavailable";

const DURABILITY_LABELS: [&str; 6] = [
    "Chainstate durability",
    "Cache occupancy",
    "Coins best-block",
    "Coins recovery",
    "Have-bytes",
    "Have-bytes counts",
];

pub(super) fn chainstate_durability_rows(
    snapshot: &OpenBitcoinStatusSnapshot,
) -> Vec<DashboardRow> {
    match &snapshot.chainstate_durability {
        FieldAvailability::Available(evidence) => available_rows(evidence),
        FieldAvailability::Unavailable { reason } => DURABILITY_LABELS
            .iter()
            .map(|label| row(*label, format!("Unavailable: {reason}")))
            .collect(),
    }
}

fn available_rows(evidence: &ChainstateDurabilityEvidence) -> Vec<DashboardRow> {
    vec![
        row(
            "Chainstate durability",
            format!(
                "cache_size={} last_flush_reason={} write_kind={} readiness={}",
                cache_size_token(evidence.cache_size),
                evidence.last_flush_reason.as_str(),
                evidence.write_kind.as_str(),
                evidence.readiness.as_str()
            ),
        ),
        row(
            "Cache occupancy",
            format!(
                "cache_bytes={} cache_byte_limit={}",
                evidence.cache_bytes, evidence.cache_byte_limit
            ),
        ),
        row("Coins best-block", coins_best_block_text(evidence)),
        row("Coins recovery", evidence.recovery_outcome.as_str()),
        row(
            "Have-bytes",
            format!(
                "{} payload_present={} index_known={} validated_on_active_chain={}",
                evidence.last_serving_status.as_str(),
                evidence.last_payload_present,
                evidence.last_index_known,
                evidence.last_validated_on_active_chain
            ),
        ),
        row(
            "Have-bytes counts",
            format!(
                "available_count={} unavailable_count={} index_known_without_payload_count={}",
                evidence.available_count,
                evidence.unavailable_count,
                evidence.index_known_without_payload_count
            ),
        ),
    ]
}

fn coins_best_block_text(evidence: &ChainstateDurabilityEvidence) -> String {
    let Some(height) = evidence.maybe_coins_best_block_height else {
        return format!("Unavailable: {COINS_BEST_BLOCK_UNAVAILABLE_REASON}");
    };
    let Some(hash) = evidence.maybe_coins_best_block_hash.as_deref() else {
        return format!("Unavailable: {COINS_BEST_BLOCK_UNAVAILABLE_REASON}");
    };

    format!("height={height} hash={hash}")
}

fn cache_size_token(value: CacheSizeLabel) -> &'static str {
    match value {
        CacheSizeLabel::Ok => "OK",
        CacheSizeLabel::Large => "LARGE",
        CacheSizeLabel::Critical => "CRITICAL",
    }
}
