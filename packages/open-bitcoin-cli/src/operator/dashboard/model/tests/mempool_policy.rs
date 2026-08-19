// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::*;

fn mempool_and_wallet_rows(
    state: &DashboardState,
) -> &[crate::operator::dashboard::model::DashboardRow] {
    &state
        .sections
        .iter()
        .find(|section| section.title == "Mempool and Wallet")
        .expect("Mempool and Wallet section")
        .rows
}

fn row_index(rows: &[crate::operator::dashboard::model::DashboardRow], label: &str) -> usize {
    rows.iter()
        .position(|row| row.label == label)
        .unwrap_or_else(|| panic!("missing {label} row"))
}

fn populated_policy_snapshot() -> OpenBitcoinStatusSnapshot {
    let mut snapshot = test_snapshot();
    snapshot.mempool.transactions = FieldAvailability::available(0);
    snapshot.mempool.resources = FieldAvailability::available(MempoolResourcesGroup {
        virtual_size: 0,
        accounted_usage: 0,
        accounted_capacity: 300_000_000,
        transaction_count: 0,
    });
    snapshot.mempool.fee_floors = FieldAvailability::available(MempoolFeeFloorsGroup {
        static_relay_floor: 1_000,
        rolling_mempool_floor: 1_000,
        effective_admission_floor: 1_000,
        incremental_relay_fee: 1_000,
    });
    snapshot.mempool.pressure = FieldAvailability::available(MempoolPressureGroup {
        pressure_removal_count: 0,
        decay_half_life_label: "not_decaying".to_string(),
    });
    snapshot.mempool.eviction = FieldAvailability::available(MempoolEvictionGroup {
        pressure_removal_count: 0,
    });
    snapshot.mempool.checkpoint = FieldAvailability::available(MempoolCheckpointGroup {
        outcome: "never_attempted".to_string(),
        overdue: false,
        persistence_strength: "unavailable".to_string(),
        age_seconds: None,
        loss_bound_seconds: None,
        dirty_generation_present: false,
    });
    snapshot.mempool.recovery = FieldAvailability::available(MempoolRecoveryGroup::default());
    snapshot.mempool.retry = FieldAvailability::available(MempoolRetryGroup {
        eligible: 0,
        queued: 0,
        attempted: 0,
        emitted: 0,
        requested: 0,
        served: 0,
        suppressed: 0,
        relay_disabled: 1,
        cleared: 0,
    });
    snapshot.mempool.admission = FieldAvailability::available(MempoolAdmissionGroup {
        accepted: 0,
        still_present: 0,
        cleared: 0,
    });
    snapshot
}

#[test]
fn dashboard_rows_place_virtual_size_after_mempool_before_relay_evidence() {
    // Arrange
    let snapshot = populated_policy_snapshot();

    // Act
    let state = DashboardState::from_snapshot(&snapshot);
    let rows = mempool_and_wallet_rows(&state);

    // Assert
    let mempool_idx = row_index(rows, "Mempool");
    let virtual_size_idx = row_index(rows, "Virtual size");
    let relay_evidence_idx = row_index(rows, "Relay evidence");
    assert!(mempool_idx < virtual_size_idx);
    assert!(virtual_size_idx < relay_evidence_idx);
    assert_eq!(rows[virtual_size_idx].value, "0 vbytes");
}

#[test]
fn dashboard_rows_include_admission_states_and_relay_states() {
    // Arrange
    let snapshot = populated_policy_snapshot();

    // Act
    let state = DashboardState::from_snapshot(&snapshot);
    let rows = mempool_and_wallet_rows(&state);
    let admission = rows
        .iter()
        .find(|row| row.label == "Admission states")
        .expect("Admission states row");
    let relay_states = rows
        .iter()
        .find(|row| row.label == "Relay states")
        .expect("Relay states row");

    // Assert
    assert_eq!(admission.value, "accepted=0 still_present=0 cleared=0");
    assert_eq!(
        relay_states.value,
        "eligible=0 queued=0 attempted=0 emitted=0 requested=0 served=0 suppressed=0 relay_disabled=1"
    );
}

#[test]
fn dashboard_does_not_add_a_ninth_chart_kind() {
    // Arrange
    let snapshot = populated_policy_snapshot();
    let metrics_src = include_str!("../metrics.rs");

    // Act
    let state = DashboardState::from_snapshot(&snapshot);

    // Assert
    assert_eq!(MAX_DASHBOARD_CHARTS, 8);
    assert_eq!(state.charts.len(), 8);
    assert!(!metrics_src.contains("fee_floors"));
    assert!(!metrics_src.contains("MempoolRetry"));
    assert!(!metrics_src.contains("retry"));
}

#[test]
fn dashboard_row_labels_are_open_bitcoin_not_knots_aliases() {
    // Arrange
    let snapshot = populated_policy_snapshot();

    // Act
    let state = DashboardState::from_snapshot(&snapshot);
    let labels = mempool_and_wallet_rows(&state)
        .iter()
        .map(|row| row.label.as_str())
        .collect::<Vec<_>>();

    // Assert
    for required in [
        "Virtual size",
        "Accounted usage",
        "Accounted capacity",
        "Static relay floor",
        "Rolling mempool floor",
        "Effective admission floor",
        "Incremental relay fee",
        "Pressure removals",
        "Eviction (relay)",
        "Checkpoint",
        "Recovery",
        "Retry",
        "Admission states",
        "Relay states",
    ] {
        assert!(labels.contains(&required), "missing {required}");
    }
    for forbidden in ["bytes", "usage", "maxmempool"] {
        assert!(
            !labels
                .iter()
                .any(|label| label.eq_ignore_ascii_case(forbidden)),
            "used Knots alias {forbidden}"
        );
    }
    let knots_rolling_floor_alias = ["mempool", "min", "fee"].concat();
    assert!(
        !labels
            .iter()
            .any(|label| label.eq_ignore_ascii_case(&knots_rolling_floor_alias)),
        "used Knots rolling-floor alias"
    );
}
