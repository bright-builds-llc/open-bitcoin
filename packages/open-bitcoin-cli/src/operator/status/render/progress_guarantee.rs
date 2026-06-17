// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Phase 78 human status lines.

use open_bitcoin_node::status::SyncStatus;

use crate::operator::sync_truth_render::{
    last_peer_contribution_text, last_useful_work_text, no_progress_threshold_text,
    progress_credit_text, progress_window_text, stall_diagnosis_text,
};

use super::{SYNC_PROGRESS_CREDIT_PREFIX, SYNC_STALLED_SUBSYSTEM_PREFIX};

pub(super) fn progress_guarantee_lines(sync: &SyncStatus) -> [String; 6] {
    [
        format!(
            "{SYNC_PROGRESS_CREDIT_PREFIX} {}",
            progress_credit_text(&sync.progress_credit)
        ),
        format!(
            "Sync expected progress window: {}",
            progress_window_text(&sync.expected_progress_window)
        ),
        format!(
            "Sync no-progress threshold: {}",
            no_progress_threshold_text(&sync.no_progress_threshold)
        ),
        format!(
            "Sync last useful work: {}",
            last_useful_work_text(&sync.last_useful_work)
        ),
        format!(
            "Sync last peer contribution: {}",
            last_peer_contribution_text(&sync.last_peer_contribution)
        ),
        format!(
            "{SYNC_STALLED_SUBSYSTEM_PREFIX} {}",
            stall_diagnosis_text(&sync.stall_diagnosis)
        ),
    ]
}
