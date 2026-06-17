// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Sync-and-peer dashboard section projection.

use open_bitcoin_node::status::OpenBitcoinStatusSnapshot;

use crate::operator::sync_truth_render::{
    best_known_tip_text, last_peer_contribution_text, last_useful_work_text,
    no_progress_diagnosis_text, no_progress_threshold_text, progress_credit_text,
    progress_window_text, stall_diagnosis_text, stay_current_text, sync_progress_text,
    sync_reconcile_text, sync_reorg_text,
};

use super::{
    DashboardSection, PROGRESS_CREDIT_ROW_LABEL, STALLED_SUBSYSTEM_ROW_LABEL,
    peer_counts_availability, recovery, resource_bounds, row, string_availability,
    sync_attempt_counters, sync_configured_targets, sync_lifecycle, sync_pressure,
    sync_progress_signal, sync_stop_reason, u64_availability,
};

pub(super) fn sync_and_peers_section(snapshot: &OpenBitcoinStatusSnapshot) -> DashboardSection {
    DashboardSection {
        title: "Sync and Peers".to_string(),
        rows: vec![
            row("State", sync_lifecycle(&snapshot.sync.lifecycle)),
            row("Phase", string_availability(&snapshot.sync.phase)),
            row(
                "Configured targets",
                sync_configured_targets(&snapshot.sync.configured_targets),
            ),
            row(
                "Attempt counters",
                sync_attempt_counters(&snapshot.sync.attempt_counters),
            ),
            row(
                "Signal",
                sync_progress_signal(&snapshot.sync.progress_signal),
            ),
            row(
                "Best-known tip",
                best_known_tip_text(&snapshot.sync.best_known_tip),
            ),
            row(
                "Stay-current",
                stay_current_text(&snapshot.sync.stay_current),
            ),
            row(
                "Stay-current action",
                string_availability(&snapshot.sync.stay_current_next_action),
            ),
            row(
                "No-progress diagnosis",
                no_progress_diagnosis_text(&snapshot.sync.no_progress_diagnosis),
            ),
            row(
                "No-progress action",
                string_availability(&snapshot.sync.no_progress_next_action),
            ),
            row(
                PROGRESS_CREDIT_ROW_LABEL,
                progress_credit_text(&snapshot.sync.progress_credit),
            ),
            row(
                "Expected progress window",
                progress_window_text(&snapshot.sync.expected_progress_window),
            ),
            row(
                "No-progress threshold",
                no_progress_threshold_text(&snapshot.sync.no_progress_threshold),
            ),
            row(
                "Last useful work",
                last_useful_work_text(&snapshot.sync.last_useful_work),
            ),
            row(
                "Last peer contribution",
                last_peer_contribution_text(&snapshot.sync.last_peer_contribution),
            ),
            row(
                STALLED_SUBSYSTEM_ROW_LABEL,
                stall_diagnosis_text(&snapshot.sync.stall_diagnosis),
            ),
            row(
                "Last progress",
                u64_availability(
                    &snapshot.sync.last_successful_progress_unix_seconds,
                    "unix seconds",
                ),
            ),
            row(
                "Latest stop reason",
                sync_stop_reason(&snapshot.sync.latest_stop_reason),
            ),
            row("Last error", string_availability(&snapshot.sync.last_error)),
            row(
                "Recovery category",
                recovery::recovery_category(&snapshot.sync.recovery_category),
            ),
            row(
                "Recovery",
                string_availability(&snapshot.sync.recovery_action),
            ),
            row(
                "Recovery evidence",
                recovery::recovery_evidence(&snapshot.recovery_evidence),
            ),
            row("Pressure", sync_pressure(&snapshot.sync.resource_pressure)),
            row(
                "Resource bounds",
                resource_bounds::resource_bounds(&snapshot.resource_bounds),
            ),
            row("Latest reorg", sync_reorg_text(&snapshot.sync.latest_reorg)),
            row(
                "Reconcile",
                sync_reconcile_text(&snapshot.sync.reconcile_progress),
            ),
            row(
                "Peers",
                peer_counts_availability(&snapshot.peers.peer_counts),
            ),
            row("Progress", sync_progress_text(&snapshot.sync.sync_progress)),
        ],
    }
}
