// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::*;

#[test]
fn dashboard_recovery_evidence_unavailable_row_preserves_reason() {
    // Arrange
    let mut snapshot = shared_sync_truth_snapshot();
    snapshot.recovery_evidence = FieldAvailability::unavailable("recovery evidence unavailable");

    // Act
    let state = DashboardState::from_snapshot(&snapshot);

    // Assert
    let sync_rows = &state.sections[1].rows;
    let recovery_evidence = sync_rows
        .iter()
        .find(|row| row.label == "Recovery evidence")
        .expect("recovery evidence row");
    assert_eq!(
        recovery_evidence.value,
        "Unavailable: recovery evidence unavailable"
    );
}
