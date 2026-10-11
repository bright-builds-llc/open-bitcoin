// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use open_bitcoin_node::status::{
    BasicIndexFailureCategory, BasicIndexOperationalState, BasicIndexOperatorStatus,
    BasicIndexUnavailableReason, BasicRetainedAvailability, FieldAvailability,
};

use super::{DashboardState, test_snapshot};
use crate::operator::{dashboard::render_dashboard_text_snapshot, status::basic_index_entries};

fn assert_projection(status: BasicIndexOperatorStatus) {
    let mut snapshot = test_snapshot();
    snapshot.basic_index = status;
    let before = snapshot.clone();
    let state = DashboardState::from_snapshot(&snapshot);
    let section = state
        .sections
        .iter()
        .find(|section| section.title == "BASIC index")
        .expect("BASIC section");
    let expected = basic_index_entries(&snapshot.basic_index);
    assert_eq!(section.rows.len(), 19);
    assert_eq!(
        section
            .rows
            .iter()
            .map(|row| (row.label.clone(), row.value.clone()))
            .collect::<Vec<_>>(),
        expected
    );
    let text = render_dashboard_text_snapshot(&snapshot);
    let lines = text
        .split("## BASIC index\n")
        .nth(1)
        .expect("text heading")
        .lines()
        .take(19)
        .collect::<Vec<_>>();
    assert_eq!(
        lines,
        expected
            .iter()
            .map(|(label, value)| format!("{label}: {value}"))
            .collect::<Vec<_>>()
    );
    assert!(state.charts.len() <= 8);
    assert_eq!(snapshot, before);
}

#[test]
fn phase161_basic_dashboard_model_orders_sections_and_full_rows() {
    // Arrange
    let snapshot = test_snapshot();
    // Act
    let state = DashboardState::from_snapshot(&snapshot);
    // Assert
    assert_eq!(
        state
            .sections
            .iter()
            .map(|section| section.title.as_str())
            .collect::<Vec<_>>(),
        [
            "Node",
            "Sync and Peers",
            "Mempool and Wallet",
            "Service",
            "BASIC index",
            "Logs and Health",
            "Prune"
        ]
    );
    assert_projection(snapshot.basic_index);
}

#[test]
fn phase161_basic_dashboard_model_preserves_supplied_states_latch_and_endpoints() {
    // Arrange
    use BasicIndexOperationalState::*;
    let cases = [
        (CatchingUp, false, 7, Some(21), Some(18)),
        (Current, true, 0, Some(21), Some(21)),
        (Lagged, true, 3, Some(21), Some(18)),
        (Disabled, false, 0, None, None),
        (CatchingUp, false, 24, None, None),
        (CatchingUp, false, 24, Some(0), Some(0)),
    ];
    // Act / Assert
    for (state, earned, lag, processed, durable) in cases {
        assert_projection(BasicIndexOperatorStatus {
            operational_state: FieldAvailability::available(state),
            initial_sync_complete: FieldAvailability::available(earned),
            maybe_lag_blocks: FieldAvailability::available(Some(lag)),
            maybe_processed_height: FieldAvailability::available(processed),
            maybe_safe_durable_height: FieldAvailability::available(durable),
            ..BasicIndexOperatorStatus::default()
        });
    }
}

#[test]
fn phase161_basic_dashboard_model_preserves_every_failure_with_retained_evidence() {
    // Arrange
    use BasicIndexFailureCategory::*;
    let cases = [
        RequiredBody,
        RequiredUndo,
        Backend,
        IntegrityOrInvalidFacts,
        ResourceBound,
        Invalidated,
    ];
    // Act / Assert
    for failure in cases {
        assert_projection(BasicIndexOperatorStatus {
            operational_state: FieldAvailability::available(BasicIndexOperationalState::Failed),
            maybe_failure: FieldAvailability::available(Some(failure)),
            retained_availability: FieldAvailability::available(
                BasicRetainedAvailability::Observed,
            ),
            ..BasicIndexOperatorStatus::default()
        });
    }
}

#[test]
fn phase161_basic_dashboard_model_preserves_stopped_config_and_legacy_unknowns() {
    // Arrange
    let stopped = BasicIndexOperatorStatus {
        configured_indexing: FieldAvailability::available(true),
        configured_peer_serving: FieldAvailability::available(false),
        ..BasicIndexOperatorStatus::unavailable(BasicIndexUnavailableReason::LiveUnavailable)
    };
    // Act / Assert
    assert_projection(stopped);
    assert_projection(BasicIndexOperatorStatus::default());
}
