// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Pure presentation of captured BASIC facts, without inferring owner state.

use open_bitcoin_node::status::{
    BasicIndexFailureCategory, BasicIndexNextAction, BasicIndexOperationalState,
    BasicIndexOperatorStatus, BasicIndexProtection, BasicIndexUnavailableReason,
    BasicRetainedAvailability, BasicRetentionBasis, FieldAvailability,
};

/// Full rows shared by human status, dashboard text and captured support exports.
pub(crate) fn basic_index_entries(status: &BasicIndexOperatorStatus) -> Vec<(String, String)> {
    let entries = [
        ("State", available(&status.operational_state, |state| state_name(*state).into())),
        ("Init sync", available(&status.initial_sync_complete, |earned| if *earned { "earned" } else { "not earned" }.into())),
        ("Lag", available(&status.maybe_lag_blocks, |maybe_lag| maybe_lag.map_or_else(|| "none".into(), |lag| format!("{lag} blocks")))),
        ("Processed", endpoint(&status.maybe_processed_height)),
        ("Durable", endpoint(&status.maybe_safe_durable_height)),
        ("Failure", available(&status.maybe_failure, |maybe_failure| maybe_failure.map_or("none", failure_name).into())),
        ("Retained", available(&status.retained_availability, |value| match value { BasicRetainedAvailability::Observed => "observed", BasicRetainedAvailability::NoneSeen => "none seen" }.into())),
        ("Index bytes", available(&status.retention, |value| format!("{} B", value.retained_record_key_value_bytes))),
        ("Filter bytes", available(&status.retention, |value| format!("{} B", value.encoded_filter_payload_bytes))),
        ("Records", available(&status.retention, |value| value.retained_record_count.to_string())),
        ("Indexing", available(&status.configured_indexing, ToString::to_string)),
        ("Peer serving", available(&status.configured_peer_serving, ToString::to_string)),
        ("Peer scope", available(&status.peer_scope, |value| format!("general={} explicit={} all={} implicit={} offered={}; counts cover modeled origins; implicit origins are not modeled and count zero; ordinary peers are not relabeled implicit; implicit alone grants no BASIC privilege; all may advertise without a backend", value.general_sessions, value.explicit_sessions, value.all_sessions, value.implicit_sessions, value.offered_sessions))),
        ("Target", endpoint(&status.maybe_accepted_height)),
        ("Protection", available(&status.protection, |value| match value { BasicIndexProtection::None => "none".into(), BasicIndexProtection::InclusiveRange(range) => format!("heights {}..{}", range.height_first(), range.height_last()), BasicIndexProtection::HeightSpaceExhausted => "height space exhausted".into() })),
        ("Basis", available(&status.retention, |value| match value.basis { BasicRetentionBasis::ImmutableRecordKeysAndValuesIncludingStaleBranches => "immutable record keys/values; includes stale branches; excludes Fjall overhead. Excludes active projections, runtime state, owner/checkpoint/protection/lock metadata and physical filesystem allocation.".into() })),
        ("Next action", available(&status.next_action, |value| next_action(*value).into())),
        ("Retention note", "Filter growth is separate from the soft block/undo target; protection can retain more history.".into()),
        ("Body/wallet note", "Retained filters do not restore deleted bodies or wallet eligibility.".into()),
    ];
    entries
        .into_iter()
        .map(|(label, value)| (label.into(), value))
        .collect()
}

pub(super) fn basic_index_lines(status: &BasicIndexOperatorStatus) -> Vec<String> {
    std::iter::once("BASIC index".into())
        .chain(
            basic_index_entries(status)
                .into_iter()
                .map(|(label, value)| format!("{label}: {value}")),
        )
        .collect()
}

fn available<T>(field: &FieldAvailability<T>, render: impl FnOnce(&T) -> String) -> String {
    match field {
        FieldAvailability::Available(value) => render(value),
        FieldAvailability::Unavailable { reason } => format!(
            "Unavailable: {}",
            BasicIndexUnavailableReason::normalize(reason)
        ),
    }
}

fn endpoint(field: &FieldAvailability<Option<u32>>) -> String {
    available(field, |maybe_height| {
        maybe_height.map_or_else(|| "none".into(), |height| height.to_string())
    })
}

fn state_name(state: BasicIndexOperationalState) -> &'static str {
    match state {
        BasicIndexOperationalState::Disabled => "disabled",
        BasicIndexOperationalState::CatchingUp => "catching up",
        BasicIndexOperationalState::Current => "current",
        BasicIndexOperationalState::Lagged => "lagged",
        BasicIndexOperationalState::Paused => "paused",
        BasicIndexOperationalState::Failed => "failed",
        BasicIndexOperationalState::Invalidated => "invalidated",
    }
}

fn failure_name(failure: BasicIndexFailureCategory) -> &'static str {
    match failure {
        BasicIndexFailureCategory::RequiredBody => "missing body",
        BasicIndexFailureCategory::RequiredUndo => "missing undo",
        BasicIndexFailureCategory::Backend => "backend",
        BasicIndexFailureCategory::IntegrityOrInvalidFacts => "integrity",
        BasicIndexFailureCategory::ResourceBound => "resource",
        BasicIndexFailureCategory::Invalidated => "invalidated",
    }
}

fn next_action(action: BasicIndexNextAction) -> &'static str {
    match action {
        BasicIndexNextAction::Disabled => {
            "BASIC indexing is disabled. Capability and retained evidence are separate facts."
        }
        BasicIndexNextAction::EmptyPrefix => {
            "No BASIC prefix observed. Check index configuration and required history."
        }
        BasicIndexNextAction::InitialCatchUp => {
            "Initial catch-up is incomplete. Read lag and safe durable progress."
        }
        BasicIndexNextAction::Current => {
            "Read safe durable progress and retained availability separately."
        }
        BasicIndexNextAction::Lagged => {
            "Initial sync was earned; current work remains behind the accepted target."
        }
        BasicIndexNextAction::ProcessedAheadOfDurable => {
            "Processed work exceeds the safe durable endpoint."
        }
        BasicIndexNextAction::RequiredBody => {
            "Required block body is missing. Inspect history evidence; no automatic repair or download."
        }
        BasicIndexNextAction::RequiredUndo => {
            "Required undo is missing. Inspect history evidence; no automatic repair or download."
        }
        BasicIndexNextAction::Backend => {
            "Index backend failed. Inspect bounded support evidence; read safe durable progress separately."
        }
        BasicIndexNextAction::IntegrityOrInvalidFacts => {
            "Index integrity check failed. Inspect bounded support evidence; do not treat failed lookup as absence."
        }
        BasicIndexNextAction::ResourceBound => {
            "Index work is paused at a resource bound. Inspect bounded resource evidence before continuing."
        }
        BasicIndexNextAction::Invalidated => {
            "Index lifecycle changed. Collect current evidence; stale work cannot earn progress."
        }
        BasicIndexNextAction::CollectLiveEvidence => {
            "Check daemon status, then collect live BASIC evidence."
        }
        BasicIndexNextAction::InspectSupportEvidence => {
            "Collect status again and inspect bounded support evidence."
        }
    }
}
