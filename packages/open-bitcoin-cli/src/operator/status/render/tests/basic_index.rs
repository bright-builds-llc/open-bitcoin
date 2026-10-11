// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use open_bitcoin_node::status::{
    BasicIndexFailureCategory, BasicIndexNextAction, BasicIndexOperationalState,
    BasicIndexOperatorStatus, BasicIndexProtection, BasicIndexProtectionRange,
    BasicIndexUnavailableReason, BasicPeerScopeStatus, BasicRetainedAvailability,
    BasicRetentionBasis, BasicRetentionMeasurement, FieldAvailability,
};

use super::{StatusRenderMode, render_status, shared_sync_truth_snapshot};

const LABELS: [&str; 19] = [
    "State",
    "Init sync",
    "Lag",
    "Processed",
    "Durable",
    "Failure",
    "Retained",
    "Index bytes",
    "Filter bytes",
    "Records",
    "Indexing",
    "Peer serving",
    "Peer scope",
    "Target",
    "Protection",
    "Basis",
    "Next action",
    "Retention note",
    "Body/wallet note",
];

fn render(status: BasicIndexOperatorStatus) -> String {
    let mut snapshot = shared_sync_truth_snapshot();
    snapshot.basic_index = status;
    render_status(&snapshot, StatusRenderMode::Human).expect("human status")
}

#[test]
fn phase161_basic_render_orders_all_rows_before_prune() {
    // Arrange
    let status = BasicIndexOperatorStatus::default();
    // Act
    let text = render(status);
    let section = text.split("BASIC index\n").nth(1).expect("BASIC heading");
    // Assert
    let lines: Vec<_> = section.lines().take(19).collect();
    for (line, label) in lines.iter().zip(LABELS) {
        assert!(line.starts_with(&format!("{label}: ")), "{line}");
    }
    assert!(
        section
            .lines()
            .nth(19)
            .expect("prune follows")
            .starts_with("Prune")
    );
}

#[test]
fn phase161_basic_render_preserves_earned_lag_and_distinct_endpoints() {
    // Arrange
    let status = BasicIndexOperatorStatus {
        operational_state: FieldAvailability::available(BasicIndexOperationalState::Lagged),
        initial_sync_complete: FieldAvailability::available(true),
        maybe_lag_blocks: FieldAvailability::available(Some(3)),
        maybe_processed_height: FieldAvailability::available(Some(21)),
        maybe_safe_durable_height: FieldAvailability::available(Some(18)),
        maybe_accepted_height: FieldAvailability::available(Some(24)),
        next_action: FieldAvailability::available(BasicIndexNextAction::ProcessedAheadOfDurable),
        ..BasicIndexOperatorStatus::default()
    };
    // Act
    let text = render(status);
    // Assert
    for expected in [
        "State: lagged",
        "Init sync: earned",
        "Lag: 3 blocks",
        "Processed: 21",
        "Durable: 18",
        "Target: 24",
        "Next action: Processed work exceeds the safe durable endpoint.",
    ] {
        assert!(text.contains(expected), "{expected}");
    }
}

#[test]
fn phase161_basic_render_distinguishes_empty_genesis_and_unknown() {
    // Arrange
    let cases = [
        (FieldAvailability::available(None), "none"),
        (FieldAvailability::available(Some(0)), "0"),
        (
            FieldAvailability::unavailable(BasicIndexUnavailableReason::LiveUnavailable.as_str()),
            "Unavailable: live BASIC evidence unavailable",
        ),
    ];
    // Act / Assert
    for (endpoint, expected) in cases {
        let text = render(BasicIndexOperatorStatus {
            maybe_processed_height: endpoint.clone(),
            maybe_safe_durable_height: endpoint.clone(),
            maybe_accepted_height: endpoint,
            ..BasicIndexOperatorStatus::default()
        });
        for label in ["Processed", "Durable", "Target"] {
            assert!(text.contains(&format!("{label}: {expected}\n")));
        }
    }
}

#[test]
fn phase161_basic_render_failed_state_preserves_independent_growth_and_protection() {
    // Arrange
    let status = BasicIndexOperatorStatus {
        operational_state: FieldAvailability::available(BasicIndexOperationalState::Failed),
        maybe_failure: FieldAvailability::available(Some(BasicIndexFailureCategory::RequiredUndo)),
        retained_availability: FieldAvailability::available(BasicRetainedAvailability::Observed),
        retention: FieldAvailability::available(BasicRetentionMeasurement {
            retained_record_count: 7,
            encoded_filter_payload_bytes: 13,
            retained_record_key_value_bytes: 97,
            basis: BasicRetentionBasis::ImmutableRecordKeysAndValuesIncludingStaleBranches,
        }),
        protection: FieldAvailability::available(BasicIndexProtection::InclusiveRange(
            BasicIndexProtectionRange::new(18, 24).expect("valid bounds"),
        )),
        ..BasicIndexOperatorStatus::default()
    };
    // Act
    let text = render(status);
    // Assert
    for expected in [
        "State: failed",
        "Failure: missing undo",
        "Retained: observed",
        "Index bytes: 97 B",
        "Filter bytes: 13 B",
        "Records: 7",
        "Protection: heights 18..24",
        "Basis: immutable record keys/values; includes stale branches; excludes Fjall overhead",
        "Filter growth is separate from the soft block/undo target; protection can retain more history.",
        "Retained filters do not restore deleted bodies or wallet eligibility.",
    ] {
        assert!(text.contains(expected), "{expected}");
    }
    assert!(!text.contains("110 B"));
}

#[test]
fn phase161_basic_render_normalizes_hostile_direct_rust_reasons() {
    // Arrange
    let status = BasicIndexOperatorStatus {
        operational_state: FieldAvailability::unavailable("/private/secret\n\u{1b}[31m"),
        ..BasicIndexOperatorStatus::default()
    };
    // Act
    let text = render(status);
    // Assert
    assert!(text.contains("State: Unavailable: BASIC evidence not reported"));
    assert!(!text.contains("/private/secret"));
    assert!(!text.contains('\u{1b}'));
}

#[test]
fn phase161_basic_render_maps_owner_states_without_deriving_from_heights() {
    // Arrange
    use BasicIndexOperationalState::*;
    let cases = [
        (Disabled, "disabled"),
        (CatchingUp, "catching up"),
        (Current, "current"),
        (Lagged, "lagged"),
        (Paused, "paused"),
        (Failed, "failed"),
        (Invalidated, "invalidated"),
    ];
    // Act / Assert
    for (state, expected) in cases {
        let text = render(BasicIndexOperatorStatus {
            operational_state: FieldAvailability::available(state),
            initial_sync_complete: FieldAvailability::available(false),
            maybe_lag_blocks: FieldAvailability::available(Some(0)),
            maybe_processed_height: FieldAvailability::available(Some(1)),
            maybe_safe_durable_height: FieldAvailability::available(Some(1)),
            ..BasicIndexOperatorStatus::default()
        });
        assert!(text.contains(&format!("State: {expected}\n")));
        assert!(text.contains("Init sync: not earned\n"));
    }
}

#[test]
fn phase161_basic_render_distinguishes_each_failure_and_observed_none() {
    // Arrange
    use BasicIndexFailureCategory::*;
    let cases = [
        (Some(RequiredBody), "missing body"),
        (Some(RequiredUndo), "missing undo"),
        (Some(Backend), "backend"),
        (Some(IntegrityOrInvalidFacts), "integrity"),
        (Some(ResourceBound), "resource"),
        (Some(Invalidated), "invalidated"),
        (None, "none"),
    ];
    // Act / Assert
    for (maybe_failure, expected) in cases {
        let text = render(BasicIndexOperatorStatus {
            maybe_failure: FieldAvailability::available(maybe_failure),
            retained_availability: FieldAvailability::available(
                BasicRetainedAvailability::Observed,
            ),
            ..BasicIndexOperatorStatus::default()
        });
        assert!(text.contains(&format!("Failure: {expected}\n")));
        assert!(text.contains("Retained: observed\n"));
    }
}

#[test]
fn phase161_basic_render_keeps_unknown_measurements_separate_from_observed_records() {
    // Arrange
    let reasons = [
        BasicIndexUnavailableReason::RetentionIncomplete,
        BasicIndexUnavailableReason::RetentionFailed,
        BasicIndexUnavailableReason::RetentionBudgetExceeded,
    ];
    // Act / Assert
    for reason in reasons {
        let text = render(BasicIndexOperatorStatus {
            retained_availability: FieldAvailability::available(
                BasicRetainedAvailability::Observed,
            ),
            retention: FieldAvailability::unavailable(reason.as_str()),
            ..BasicIndexOperatorStatus::default()
        });
        assert!(text.contains("Retained: observed\n"));
        for label in ["Index bytes", "Filter bytes", "Records", "Basis"] {
            assert!(text.contains(&format!("{label}: Unavailable: {}\n", reason.as_str())));
        }
    }
}

#[test]
fn phase161_basic_render_preserves_configured_scope_without_readiness_claim() {
    // Arrange
    let status = BasicIndexOperatorStatus {
        configured_indexing: FieldAvailability::available(false),
        configured_peer_serving: FieldAvailability::available(false),
        peer_scope: FieldAvailability::available(BasicPeerScopeStatus {
            general_sessions: 2,
            explicit_sessions: 3,
            all_sessions: 5,
            implicit_sessions: 0,
            offered_sessions: 11,
        }),
        ..BasicIndexOperatorStatus::unavailable(BasicIndexUnavailableReason::LiveUnavailable)
    };
    // Act
    let text = render(status);
    // Assert
    for expected in [
        "Indexing: false",
        "Peer serving: false",
        "Peer scope: general=2 explicit=3 all=5 implicit=0 offered=11",
        "counts cover modeled origins",
        "implicit origins are not modeled and count zero",
        "ordinary peers are not relabeled implicit",
        "implicit alone grants no BASIC privilege",
        "all may advertise without a backend",
        "State: Unavailable: live BASIC evidence unavailable",
        "Durable: Unavailable: live BASIC evidence unavailable",
    ] {
        assert!(text.contains(expected), "{expected}");
    }
}

#[test]
fn phase161_basic_render_none_seen_and_zero_measurement_are_observed_facts() {
    // Arrange
    let status = BasicIndexOperatorStatus {
        retained_availability: FieldAvailability::available(BasicRetainedAvailability::NoneSeen),
        retention: FieldAvailability::available(BasicRetentionMeasurement {
            retained_record_count: 0,
            encoded_filter_payload_bytes: 0,
            retained_record_key_value_bytes: 0,
            basis: BasicRetentionBasis::ImmutableRecordKeysAndValuesIncludingStaleBranches,
        }),
        protection: FieldAvailability::available(BasicIndexProtection::None),
        ..BasicIndexOperatorStatus::default()
    };
    // Act
    let text = render(status);
    // Assert
    for expected in [
        "Retained: none seen\n",
        "Records: 0\n",
        "Index bytes: 0 B\n",
        "Filter bytes: 0 B\n",
        "Protection: none\n",
    ] {
        assert!(text.contains(expected), "{expected}");
    }
}

#[test]
fn phase161_basic_render_protection_exhaustion_stays_distinct() {
    // Arrange
    let status = BasicIndexOperatorStatus {
        protection: FieldAvailability::available(BasicIndexProtection::HeightSpaceExhausted),
        ..BasicIndexOperatorStatus::default()
    };
    // Act
    let text = render(status);
    // Assert
    assert!(text.contains("Protection: height space exhausted\n"));
}

#[test]
fn phase161_basic_render_guidance_uses_each_supplied_action_without_inference() {
    // Arrange
    use BasicIndexNextAction::*;
    let cases = [
        (
            Disabled,
            "BASIC indexing is disabled. Capability and retained evidence are separate facts.",
        ),
        (
            EmptyPrefix,
            "No BASIC prefix observed. Check index configuration and required history.",
        ),
        (
            InitialCatchUp,
            "Initial catch-up is incomplete. Read lag and safe durable progress.",
        ),
        (
            Current,
            "Read safe durable progress and retained availability separately.",
        ),
        (
            Lagged,
            "Initial sync was earned; current work remains behind the accepted target.",
        ),
        (
            ProcessedAheadOfDurable,
            "Processed work exceeds the safe durable endpoint.",
        ),
        (
            RequiredBody,
            "Required block body is missing. Inspect history evidence; no automatic repair or download.",
        ),
        (
            RequiredUndo,
            "Required undo is missing. Inspect history evidence; no automatic repair or download.",
        ),
        (
            Backend,
            "Index backend failed. Inspect bounded support evidence; read safe durable progress separately.",
        ),
        (
            IntegrityOrInvalidFacts,
            "Index integrity check failed. Inspect bounded support evidence; do not treat failed lookup as absence.",
        ),
        (
            ResourceBound,
            "Index work is paused at a resource bound. Inspect bounded resource evidence before continuing.",
        ),
        (
            Invalidated,
            "Index lifecycle changed. Collect current evidence; stale work cannot earn progress.",
        ),
        (
            CollectLiveEvidence,
            "Check daemon status, then collect live BASIC evidence.",
        ),
        (
            InspectSupportEvidence,
            "Collect status again and inspect bounded support evidence.",
        ),
    ];
    // Act / Assert
    for (action, expected) in cases {
        let text = render(BasicIndexOperatorStatus {
            next_action: FieldAvailability::available(action),
            ..BasicIndexOperatorStatus::default()
        });
        assert!(text.contains(&format!("Next action: {expected}\n")));
    }
}
