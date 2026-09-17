// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use std::fmt;

use crate::status::{ChainstateDurabilityEvidence, FieldAvailability};

use super::{StructuredLogLevel, StructuredLogRecord, sanitized_resource_log_field};

/// Allowlisted structured-log source for chainstate durability evidence.
pub const CHAINSTATE_DURABILITY_LOG_SOURCE: &str = "chainstate_durability";

/// Project chainstate-durability evidence into a closed cause/outcome/label record.
///
/// Available messages carry machine labels, counts, and optional height only.
/// They never include a coins-best-block hash. Unavailable reasons pass
/// through the existing hex/peer/credential sanitizer.
pub fn chainstate_durability_log_record(
    durability: &FieldAvailability<ChainstateDurabilityEvidence>,
    timestamp_unix_seconds: u64,
) -> StructuredLogRecord {
    let message = match durability {
        FieldAvailability::Available(evidence) => available_message(evidence),
        FieldAvailability::Unavailable { reason } => format!(
            "outcome=unavailable cause=status_projection label=chainstate_durability reason={}",
            sanitized_resource_log_field(reason)
        ),
    };

    StructuredLogRecord::new(
        StructuredLogLevel::Info,
        CHAINSTATE_DURABILITY_LOG_SOURCE,
        message,
        timestamp_unix_seconds,
    )
}

/// Map a fail-closed `StorageError` Display into a hash-free recovery label.
pub fn chainstate_durability_fail_closed_log_record(
    error: &impl fmt::Display,
    timestamp_unix_seconds: u64,
) -> StructuredLogRecord {
    let error_display = error.to_string();
    let sanitized_reason = sanitized_resource_log_field(&error_display);
    StructuredLogRecord::new(
        StructuredLogLevel::Warn,
        CHAINSTATE_DURABILITY_LOG_SOURCE,
        format!(
            "outcome=unavailable cause=status_projection label=chainstate_durability recovery_outcome=fail_closed reason={sanitized_reason}"
        ),
        timestamp_unix_seconds,
    )
}

fn available_message(evidence: &ChainstateDurabilityEvidence) -> String {
    let mut message = format!(
        "outcome=projected cause=status_projection label=chainstate_durability cache_size={} last_flush_reason={} write_kind={} recovery_outcome={} last_serving_status={} available_count={} unavailable_count={} index_known_without_payload_count={}",
        evidence.cache_size.as_str(),
        evidence.last_flush_reason.as_str(),
        evidence.write_kind.as_str(),
        evidence.recovery_outcome.as_str(),
        evidence.last_serving_status.as_str(),
        evidence.available_count,
        evidence.unavailable_count,
        evidence.index_known_without_payload_count
    );
    if let Some(height) = evidence.maybe_coins_best_block_height {
        message.push_str(&format!(" height={height}"));
    }
    message
}
