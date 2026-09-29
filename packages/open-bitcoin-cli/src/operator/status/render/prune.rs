// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Human status lines for the read-only prune snapshot.

use open_bitcoin_node::status::PruneOperatorStatus;

use crate::operator::dashboard::model::prune_section::prune_rows;

/// Same row values as the dashboard Prune section, formatted `Label: value`.
pub(super) fn prune_status_lines(status: &PruneOperatorStatus) -> Vec<String> {
    prune_rows(status)
        .into_iter()
        .map(|row| format!("{}: {}", row.label, row.value))
        .collect()
}
