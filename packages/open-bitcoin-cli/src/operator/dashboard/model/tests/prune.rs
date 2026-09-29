// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use open_bitcoin_node::status::{
    FieldAvailability, ManualPruneRefusalCode, ManualPruneSurface, PruneLockRow,
    PruneOperatorStatus, PruneSupportCounts,
};

use super::{DashboardState, test_snapshot};

#[test]
fn disabled_prune_section_shows_mode_false_and_omits_quartet_details() {
    // Arrange
    let snapshot = test_snapshot();

    // Act
    let rows = prune_rows(&snapshot);

    // Assert
    assert_eq!(row_value(&rows, "Prune mode"), Some("false"));
    assert!(row_value(&rows, "Prune height").is_none());
    assert!(row_value(&rows, "Automatic pruning").is_none());
    assert!(row_value(&rows, "Prune target").is_none());
    assert_eq!(
        row_value(&rows, "Prune locks"),
        Some("Unavailable: not collected")
    );
    assert_eq!(
        row_value(&rows, "Manual prune"),
        Some("Unavailable: not collected")
    );
    assert_forbidden_copy(&rows);
}

#[test]
fn stopped_automatic_shows_byte_target_and_does_not_substitute_height() {
    // Arrange
    let mut snapshot = test_snapshot();
    snapshot.prune = PruneOperatorStatus::from_stopped_config(Some(550), "collector stopped");
    snapshot.prune.support_counts = FieldAvailability::available(PruneSupportCounts {
        successful_batch_count: 1,
        pruned_height_count: 4,
        maybe_last_prune_height: Some(40),
    });

    // Act
    let rows = prune_rows(&snapshot);

    // Assert
    assert_eq!(row_value(&rows, "Prune mode"), Some("true"));
    assert_eq!(row_value(&rows, "Automatic pruning"), Some("true"));
    assert_eq!(row_value(&rows, "Prune target"), Some("576716800"));
    assert_eq!(
        row_value(&rows, "Prune height"),
        Some("Unavailable: collector stopped")
    );
    assert!(
        rows.iter().all(|(label, value)| {
            label != "Last prune height" && value != "40" && value != "0"
        })
    );
}

#[test]
fn live_manual_only_height_zero_shows_height_and_omits_target() {
    // Arrange
    let mut snapshot = test_snapshot();
    snapshot.prune = manual_only(Some(0), Vec::new(), ManualPruneSurface::None);

    // Act
    let rows = prune_rows(&snapshot);

    // Assert
    assert_eq!(row_value(&rows, "Prune mode"), Some("true"));
    assert_eq!(row_value(&rows, "Prune height"), Some("0"));
    assert_eq!(row_value(&rows, "Automatic pruning"), Some("false"));
    assert!(row_value(&rows, "Prune target").is_none());
    assert_eq!(row_value(&rows, "Prune locks"), Some("none"));
    assert_eq!(row_value(&rows, "Manual prune"), Some("none"));
}

#[test]
fn prune_section_lists_one_lock_row() {
    // Arrange
    let mut snapshot = test_snapshot();
    snapshot.prune = manual_only(
        Some(0),
        vec![PruneLockRow {
            name: "wallet".to_string(),
            height_first: 10,
            height_last: 20,
        }],
        ManualPruneSurface::None,
    );

    // Act
    let rows = prune_rows(&snapshot);

    // Assert
    assert_eq!(
        row_value(&rows, "Prune lock"),
        Some("name=wallet height_first=10 height_last=20")
    );
    assert!(row_value(&rows, "Prune locks").is_none());
}

#[test]
fn keep_window_refusal_says_refused_and_not_clamp() {
    // Arrange
    let mut snapshot = test_snapshot();
    snapshot.prune = manual_only(
        Some(12),
        Vec::new(),
        ManualPruneSurface::Refused {
            reason: ManualPruneRefusalCode::KeepWindow,
        },
    );

    // Act
    let rows = prune_rows(&snapshot);

    // Assert
    assert_eq!(
        row_value(&rows, "Manual prune"),
        Some("refused target is inside the 288-block keep window")
    );
    let manual = row_value(&rows, "Manual prune").expect("manual prune");
    assert!(manual.contains("refused"));
    assert!(!manual.contains("clamp"));
}

#[test]
fn manual_prune_success_shows_height_negative_one() {
    // Arrange
    let mut snapshot = test_snapshot();
    snapshot.prune = manual_only(
        Some(0),
        Vec::new(),
        ManualPruneSurface::Height { height: -1 },
    );

    // Act
    let rows = prune_rows(&snapshot);

    // Assert
    assert_eq!(row_value(&rows, "Manual prune"), Some("height=-1"));
}

fn prune_rows(
    snapshot: &open_bitcoin_node::status::OpenBitcoinStatusSnapshot,
) -> Vec<(String, String)> {
    let state = DashboardState::from_snapshot(snapshot);
    assert_eq!(state.sections[2].title, "Mempool and Wallet");
    assert_eq!(state.sections[5].title, "Prune");
    assert_eq!(state.charts.len(), 8);
    assert_eq!(
        state
            .actions
            .iter()
            .map(|action| action.key.as_str())
            .collect::<Vec<_>>(),
        ["r", "s", "t", "o", "x", "i", "u", "e", "d", "q"]
    );
    state.sections[5]
        .rows
        .iter()
        .map(|row| (row.label.clone(), row.value.clone()))
        .collect()
}

fn manual_only(
    height: Option<u32>,
    locks: Vec<PruneLockRow>,
    manual: ManualPruneSurface,
) -> PruneOperatorStatus {
    PruneOperatorStatus {
        pruned: true,
        maybe_automatic_pruning: Some(false),
        maybe_prune_target_size: None,
        pruneheight: FieldAvailability::available(height),
        locks: FieldAvailability::available(locks),
        manual_prune: FieldAvailability::available(manual),
        support_counts: FieldAvailability::available(PruneSupportCounts {
            successful_batch_count: 0,
            pruned_height_count: 0,
            maybe_last_prune_height: None,
        }),
    }
}

fn row_value<'a>(rows: &'a [(String, String)], label: &str) -> Option<&'a str> {
    rows.iter()
        .find(|(row_label, _)| row_label == label)
        .map(|(_, value)| value.as_str())
}

fn assert_forbidden_copy(rows: &[(String, String)]) {
    let rendered = rows
        .iter()
        .map(|(label, value)| format!("{label}: {value}"))
        .collect::<Vec<_>>()
        .join("\n");
    for forbidden in [
        "archive-node",
        "archive node",
        "assumeutxo",
        "assumevalid",
        "BIP37",
        "compact filter",
        "public default",
        "production ready",
        "production readiness",
        "production-funds",
    ] {
        assert!(
            !rendered.contains(forbidden),
            "prune rows leaked {forbidden}"
        );
    }
}
