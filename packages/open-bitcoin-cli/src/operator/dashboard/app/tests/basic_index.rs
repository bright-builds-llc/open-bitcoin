// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use open_bitcoin_node::status::{
    BasicIndexFailureCategory, BasicIndexNextAction, BasicIndexOperationalState,
    BasicIndexOperatorStatus, BasicIndexProtection, BasicIndexProtectionRange,
    BasicPeerScopeStatus, BasicRetainedAvailability, BasicRetentionBasis,
    BasicRetentionMeasurement, FieldAvailability, OpenBitcoinStatusSnapshot,
};
use ratatui::{Terminal, backend::TestBackend, buffer::Buffer};

use super::{Rc, RefCell, collect_dashboard_snapshot, running_snapshot, test_context};
use crate::operator::dashboard::{
    app::draw_dashboard, model::DashboardState, render_dashboard_text_snapshot,
};

fn snapshot() -> OpenBitcoinStatusSnapshot {
    let context = test_context(Rc::new(RefCell::new(Vec::new())), running_snapshot());
    let mut snapshot = collect_dashboard_snapshot(&context);
    snapshot.basic_index = BasicIndexOperatorStatus {
        operational_state: FieldAvailability::available(BasicIndexOperationalState::Lagged),
        initial_sync_complete: FieldAvailability::available(true),
        maybe_lag_blocks: FieldAvailability::available(Some(3)),
        maybe_processed_height: FieldAvailability::available(Some(840_021)),
        maybe_safe_durable_height: FieldAvailability::available(Some(840_018)),
        maybe_failure: FieldAvailability::available(Some(BasicIndexFailureCategory::RequiredBody)),
        retained_availability: FieldAvailability::available(BasicRetainedAvailability::Observed),
        retention: FieldAvailability::available(BasicRetentionMeasurement {
            retained_record_count: 7,
            encoded_filter_payload_bytes: 13,
            retained_record_key_value_bytes: 97,
            basis: BasicRetentionBasis::ImmutableRecordKeysAndValuesIncludingStaleBranches,
        }),
        configured_indexing: FieldAvailability::available(true),
        configured_peer_serving: FieldAvailability::available(false),
        peer_scope: FieldAvailability::available(BasicPeerScopeStatus {
            general_sessions: 2,
            explicit_sessions: 3,
            all_sessions: 5,
            implicit_sessions: 0,
            offered_sessions: 11,
        }),
        maybe_accepted_height: FieldAvailability::available(Some(840_024)),
        protection: FieldAvailability::available(BasicIndexProtection::InclusiveRange(
            BasicIndexProtectionRange::new(840_018, 840_024).expect("valid range"),
        )),
        next_action: FieldAvailability::available(BasicIndexNextAction::RequiredBody),
    };
    snapshot
}

fn draw(snapshot: &OpenBitcoinStatusSnapshot, width: u16, height: u16) -> Buffer {
    let state = DashboardState::from_snapshot(snapshot);
    let mut terminal = Terminal::new(TestBackend::new(width, height)).expect("test terminal");
    terminal
        .draw(|frame| draw_dashboard(frame, &state, None, "confirmation test", height < 25))
        .expect("draw real dashboard");
    terminal.backend().buffer().clone()
}

fn region(buffer: &Buffer, x: u16, y: u16, width: u16, height: u16) -> Vec<String> {
    (y..y + height)
        .map(|row| {
            (x..x + width)
                .map(|column| buffer[(column, row)].symbol())
                .collect::<String>()
                .trim_end()
                .into()
        })
        .collect()
}

#[test]
fn phase161_basic_dashboard_cells_normal_shows_all_nineteen_adjacent_facts() {
    // Arrange
    let snapshot = snapshot();
    let expected = [
        "BASIC index",
        "State: lagged",
        "Init sync: earned",
        "Lag: 3 blocks",
        "Processed: 840021",
        "Durable: 840018",
        "Failure: missing body",
        "Retained: observed",
        "Index bytes: 97 B",
        "Filter bytes: 13 B",
        "Records: 7",
        "Indexing: true",
        "Peer serving: false",
        "Peer scope: G=2 E=3 A=5 I=0 O=11",
        "Target: 840024",
        "Protection: heights 840018..840024",
        "Basis: immutable keys/values",
        "Next action: missing body; inspect history",
        "Retention note: filters outside target",
        "Body/wallet note: eligibility unchanged",
    ];
    // Act
    let buffer = draw(&snapshot, 160, 55);
    let right = region(&buffer, 113, 4, 46, 20);
    // Assert
    assert_eq!(right, expected);
    assert_eq!(region(&buffer, 113, 24, 46, 1), ["Logs and Health"]);
    assert_eq!(buffer[(56, 3)].symbol(), "┌");
    assert_eq!(buffer[(112, 3)].symbol(), "┌");
}

#[test]
fn phase161_basic_dashboard_cells_narrow_preserves_seven_critical_adjacent_values() {
    // Arrange
    let snapshot = snapshot();
    let expected = [
        "BASIC index",
        "State: lagged",
        "Init sync: earned",
        "Lag: 3 blocks",
        "Processed: 840021",
        "Durable: 840018",
        "Failure: missing body",
        "Retained: observed",
    ];
    // Act / Assert
    for height in [30, 25] {
        let buffer = draw(&snapshot, 80, height);
        assert_eq!(region(&buffer, 57, 4, 22, 8), expected);
        assert_eq!(buffer[(28, 3)].symbol(), "┌");
        assert_eq!(buffer[(56, 3)].symbol(), "┌");
    }
}

#[test]
fn phase161_basic_dashboard_cells_narrow_never_clips_any_failure_category() {
    // Arrange
    use BasicIndexFailureCategory::*;
    let cases = [
        (RequiredBody, "missing body"),
        (RequiredUndo, "missing undo"),
        (Backend, "backend"),
        (IntegrityOrInvalidFacts, "integrity"),
        (ResourceBound, "resource"),
        (Invalidated, "invalidated"),
    ];
    // Act / Assert
    for (failure, value) in cases {
        let mut snapshot = snapshot();
        snapshot.basic_index.operational_state =
            FieldAvailability::available(BasicIndexOperationalState::Failed);
        snapshot.basic_index.maybe_failure = FieldAvailability::available(Some(failure));
        for height in [30, 25] {
            let buffer = draw(&snapshot, 80, height);
            assert_eq!(region(&buffer, 57, 5, 22, 1), ["State: failed"]);
            assert_eq!(
                region(&buffer, 57, 10, 22, 2),
                [format!("Failure: {value}"), "Retained: observed".into()]
            );
        }
    }
}

#[test]
fn phase161_basic_dashboard_cells_narrow_preserves_complete_unavailable_word() {
    // Arrange
    let mut snapshot = snapshot();
    snapshot.basic_index = BasicIndexOperatorStatus::default();
    let expected = [
        "BASIC index",
        "State: Unavailable",
        "Init sync: Unavailable",
        "Lag: Unavailable",
        "Processed: Unavailable",
        "Durable: Unavailable",
        "Failure: Unavailable",
        "Retained: Unavailable",
    ];
    // Act / Assert
    for height in [30, 25] {
        let buffer = draw(&snapshot, 80, height);
        assert_eq!(region(&buffer, 57, 4, 22, 8), expected);
    }
}

#[test]
fn phase161_basic_dashboard_cells_preserves_empty_prefix_and_actual_genesis() {
    // Arrange
    let cases = [(None, "none"), (Some(0), "0")];
    // Act / Assert
    for (maybe_height, value) in cases {
        let mut snapshot = snapshot();
        snapshot.basic_index.initial_sync_complete = FieldAvailability::available(false);
        snapshot.basic_index.maybe_processed_height = FieldAvailability::available(maybe_height);
        snapshot.basic_index.maybe_safe_durable_height = FieldAvailability::available(maybe_height);
        snapshot.basic_index.maybe_failure = FieldAvailability::available(None);
        snapshot.basic_index.retained_availability =
            FieldAvailability::available(BasicRetainedAvailability::NoneSeen);
        let buffer = draw(&snapshot, 80, 25);
        assert_eq!(region(&buffer, 57, 6, 22, 1), ["Init sync: not earned"]);
        assert_eq!(
            region(&buffer, 57, 8, 22, 4),
            [
                format!("Processed: {value}"),
                format!("Durable: {value}"),
                "Failure: none".into(),
                "Retained: none seen".into()
            ]
        );
    }
}

#[test]
fn phase161_basic_dashboard_cells_actions_and_eight_charts_keep_their_bands() {
    // Arrange
    let snapshot = snapshot();
    let state = DashboardState::from_snapshot(&snapshot);
    // Act / Assert
    for (width, height, charts_y, actions_y) in
        [(160, 55, 43, 51), (80, 30, 18, 26), (80, 25, 13, 21)]
    {
        let buffer = draw(&snapshot, width, height);
        let chart_top = region(&buffer, 0, charts_y, width, 1).join("");
        assert_eq!(state.charts.len(), 8);
        assert_eq!(chart_top.matches('┌').count(), 8);
        assert!(region(&buffer, 0, actions_y, width, 1)[0].starts_with("┌Actions"));
        assert!(
            region(&buffer, 0, actions_y + 1, width, 1)[0].contains("r refresh | s service status")
        );
        assert!(
            region(&buffer, 0, 3, width, charts_y - 3)
                .iter()
                .all(|line| !line.contains("Actions"))
        );
    }
}

#[test]
fn phase161_basic_dashboard_cells_below_height_threshold_keeps_exact_blocker() {
    // Arrange
    let snapshot = snapshot();
    // Act
    let buffer = draw(&snapshot, 80, 24);
    let text = region(&buffer, 0, 0, 80, 24).join("\n");
    // Assert
    for expected in [
        "Window too small",
        "Interactive dashboard unavailable",
        "Current height: 24 rows. Required height: 25 rows.",
        "Resize the terminal to continue. Press q, Esc, or Ctrl-C to quit.",
    ] {
        assert!(text.contains(expected), "{expected}");
    }
    assert!(!text.contains("BASIC index"));
    assert!(!text.contains("Actions"));
}

#[test]
fn phase161_basic_dashboard_cells_very_narrow_is_bounded_with_full_text_fallback() {
    // Arrange
    let snapshot = snapshot();
    // Act
    let buffer = draw(&snapshot, 40, 30);
    let right = region(&buffer, 29, 4, 10, 13);
    let text = render_dashboard_text_snapshot(&snapshot);
    // Assert
    assert_eq!(right[0], "BASIC");
    assert_eq!(right[1], "Use status");
    assert!(right[2..].iter().all(String::is_empty));
    assert!(region(&buffer, 0, 26, 40, 1)[0].starts_with("┌Actions"));
    assert!(text.contains("Failure: missing body\n"));
    assert!(
        text.contains("Init sync: earned\nLag: 3 blocks\nProcessed: 840021\nDurable: 840018\n")
    );
    assert!(text.contains("implicit origins are not modeled and count zero"));
    assert!(text.contains("Retained filters do not restore deleted bodies or wallet eligibility."));
}

#[test]
fn phase161_basic_dashboard_cells_guidance_compacts_only_the_supplied_action() {
    // Arrange
    use BasicIndexNextAction::*;
    let cases = [
        (Disabled, "indexing disabled"),
        (EmptyPrefix, "no prefix observed"),
        (InitialCatchUp, "initial catch-up incomplete"),
        (Current, "read durable and retained facts"),
        (Lagged, "earned; behind accepted target"),
        (ProcessedAheadOfDurable, "processed exceeds durable"),
        (RequiredBody, "missing body; inspect history"),
        (RequiredUndo, "missing undo; inspect history"),
        (Backend, "backend failed; inspect support"),
        (IntegrityOrInvalidFacts, "integrity failed; inspect support"),
        (ResourceBound, "resource bound; inspect support"),
        (Invalidated, "lifecycle changed; collect status"),
        (CollectLiveEvidence, "collect live BASIC evidence"),
        (InspectSupportEvidence, "inspect bounded support"),
    ];
    // Act / Assert
    for (action, expected) in cases {
        let mut snapshot = snapshot();
        snapshot.basic_index.next_action = FieldAvailability::available(action);
        let buffer = draw(&snapshot, 160, 55);
        assert_eq!(
            region(&buffer, 113, 21, 46, 1),
            [format!("Next action: {expected}")]
        );
        assert_eq!(region(&buffer, 113, 10, 46, 1), ["Failure: missing body"]);
    }
}

#[test]
fn phase161_basic_dashboard_cells_hostile_reasons_stay_unavailable_without_terminal_controls() {
    // Arrange
    let mut snapshot = snapshot();
    snapshot.basic_index.operational_state =
        FieldAvailability::unavailable("/private/raw-key\n\u{1b}[31mcurrent");
    snapshot.basic_index.retention =
        FieldAvailability::unavailable("/private/raw-key\n\u{1b}[31m0");
    // Act
    let buffer = draw(&snapshot, 160, 55);
    let right = region(&buffer, 113, 4, 46, 20);
    let text = render_dashboard_text_snapshot(&snapshot);
    // Assert
    assert_eq!(right[1], "State: Unavailable");
    assert_eq!(right[8], "Index bytes: Unavailable");
    assert_eq!(right[9], "Filter bytes: Unavailable");
    assert_eq!(right[10], "Records: Unavailable");
    assert_eq!(right[16], "Basis: Unavailable");
    assert!(!right.join("\n").contains("/private/raw-key"));
    assert!(!text.contains("/private/raw-key"));
    assert!(!text.contains('\u{1b}'));
    assert!(text.contains("State: Unavailable: BASIC evidence not reported\n"));
}

#[test]
fn phase161_basic_dashboard_cells_oversized_critical_number_uses_truthful_fallback() {
    // Arrange
    let mut snapshot = snapshot();
    snapshot.basic_index.maybe_lag_blocks = FieldAvailability::available(Some(u64::MAX));
    // Act
    let buffer = draw(&snapshot, 80, 25);
    let right = region(&buffer, 57, 4, 22, 8);
    let text = render_dashboard_text_snapshot(&snapshot);
    // Assert
    assert_eq!(right[..2], ["BASIC", "Use status"]);
    assert!(right[2..].iter().all(String::is_empty));
    assert!(text.contains("Lag: 18446744073709551615 blocks\n"));
    assert!(text.contains("Failure: missing body\n"));
}

#[test]
fn phase161_basic_dashboard_cells_normal_overflow_guidance_keeps_all_basic_rows() {
    // Arrange
    let mut snapshot = snapshot();
    snapshot.prune.locks = FieldAvailability::available(
        (0..20)
            .map(|index| open_bitcoin_node::status::PruneLockRow {
                name: format!("fixture-{index}"),
                height_first: 1,
                height_last: 2,
            })
            .collect(),
    );
    // Act
    let buffer = draw(&snapshot, 160, 55);
    // Assert
    assert_eq!(
        region(&buffer, 113, 23, 46, 1),
        ["Body/wallet note: eligibility unchanged"]
    );
    assert_eq!(
        region(&buffer, 113, 41, 46, 1),
        ["Details: use status output"]
    );
}
