// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use open_bitcoin_node::status::{CacheSizeLabel, ChainstateDurabilityEvidence, FieldAvailability};

const COINS_BEST_BLOCK_UNAVAILABLE_REASON: &str = "coins best-block unavailable";
const NEXT_ACTION: &str = "Treat flush, coins recovery, cache-size, and have-bytes versus do-not as bounded local operator status. This is not prune-mode, archive-node serving, public-default historical serving, or production readiness.";

pub(super) fn push_chainstate_durability(
    output: &mut String,
    value: &FieldAvailability<ChainstateDurabilityEvidence>,
) {
    output.push_str("\n## Chainstate Durability\n\n");
    for line in durability_bullets(value) {
        output.push_str(&format!("- {line}\n"));
    }
    output.push_str(&format!("- Next action: {NEXT_ACTION}\n"));
}

fn durability_bullets(value: &FieldAvailability<ChainstateDurabilityEvidence>) -> [String; 6] {
    match value {
        FieldAvailability::Available(evidence) => [
            format!("Chainstate durability: {}", durability_summary(evidence)),
            format!(
                "Cache occupancy: cache_bytes={} cache_byte_limit={}",
                evidence.cache_bytes, evidence.cache_byte_limit
            ),
            format!("Coins best-block: {}", coins_best_block_text(evidence)),
            format!("Coins recovery: {}", evidence.recovery_outcome.as_str()),
            format!(
                "Have-bytes: {} payload_present={} index_known={} validated_on_active_chain={}",
                evidence.last_serving_status.as_str(),
                evidence.last_payload_present,
                evidence.last_index_known,
                evidence.last_validated_on_active_chain
            ),
            format!(
                "Have-bytes counts: available_count={} unavailable_count={} index_known_without_payload_count={}",
                evidence.available_count,
                evidence.unavailable_count,
                evidence.index_known_without_payload_count
            ),
        ],
        FieldAvailability::Unavailable { reason } => [
            format!("Chainstate durability: Unavailable: {reason}"),
            format!("Cache occupancy: Unavailable: {reason}"),
            format!("Coins best-block: Unavailable: {reason}"),
            format!("Coins recovery: Unavailable: {reason}"),
            format!("Have-bytes: Unavailable: {reason}"),
            format!("Have-bytes counts: Unavailable: {reason}"),
        ],
    }
}

fn durability_summary(evidence: &ChainstateDurabilityEvidence) -> String {
    format!(
        "cache_size={} last_flush_reason={} write_kind={} readiness={}",
        cache_size_token(evidence.cache_size),
        evidence.last_flush_reason.as_str(),
        evidence.write_kind.as_str(),
        evidence.readiness.as_str()
    )
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
