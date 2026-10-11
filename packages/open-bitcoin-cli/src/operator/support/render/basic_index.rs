// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use open_bitcoin_node::status::BasicIndexOperatorStatus;

use crate::operator::status::basic_index_entries;

pub(super) fn push_basic_index(output: &mut String, status: &BasicIndexOperatorStatus) {
    output.push_str("\n## BASIC index\n\n");
    for (label, value) in basic_index_entries(status) {
        output.push_str(&format!("- {label}: {value}\n"));
    }
}
