// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Complete shared BASIC rows; interactive compaction belongs to the renderer.

use open_bitcoin_node::status::BasicIndexOperatorStatus;

use super::{DashboardSection, row};
use crate::operator::status::basic_index_entries;

pub(super) fn basic_index_section(status: &BasicIndexOperatorStatus) -> DashboardSection {
    DashboardSection {
        title: "BASIC index".into(),
        rows: basic_index_entries(status)
            .into_iter()
            .map(|(label, value)| row(label, value))
            .collect(),
    }
}
