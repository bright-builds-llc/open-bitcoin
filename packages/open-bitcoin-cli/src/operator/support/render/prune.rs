// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use open_bitcoin_node::status::{FieldAvailability, PruneOperatorStatus, PruneSupportCounts};

const NEXT_ACTION: &str = "Read prune batch counts and the last deleted height as local operator evidence. Request a manual prune or a lock change from RPC or the CLI.";

pub(super) fn push_prune(output: &mut String, prune: &PruneOperatorStatus) {
    output.push_str("\n## Prune\n\n");
    for line in prune_fact_lines(&prune.support_counts) {
        output.push_str(&format!("- {line}\n"));
    }
    output.push_str(&format!("- Next action: {NEXT_ACTION}\n"));
}

fn prune_fact_lines(counts: &FieldAvailability<PruneSupportCounts>) -> Vec<String> {
    match counts {
        FieldAvailability::Unavailable { reason } => vec![
            format!("Prune batches: Unavailable: {reason}"),
            format!("Pruned heights: Unavailable: {reason}"),
            format!("Last prune height: Unavailable: {reason}"),
        ],
        FieldAvailability::Available(counts) => {
            let mut lines = vec![
                format!("Prune batches: {}", counts.successful_batch_count),
                format!("Pruned heights: {}", counts.pruned_height_count),
            ];
            if let Some(height) = counts.maybe_last_prune_height {
                lines.push(format!("Last prune height: {height}"));
            }
            lines
        }
    }
}
