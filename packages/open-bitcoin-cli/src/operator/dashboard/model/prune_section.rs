// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Read-only Prune dashboard rows from the shared status snapshot.

use open_bitcoin_node::status::{
    FieldAvailability, ManualPruneRefusalCode, ManualPruneSurface, PruneLockRow,
    PruneOperatorStatus,
};

use super::{DashboardRow, DashboardSection, row};

/// Append-only Prune section. It lists facts and does not delete payloads or edit locks.
pub(crate) fn prune_section(status: &PruneOperatorStatus) -> DashboardSection {
    DashboardSection {
        title: "Prune".to_string(),
        rows: prune_rows(status),
    }
}

/// Quartet, lock list, and manual-prune outcome. Absent facts are omitted.
pub(crate) fn prune_rows(status: &PruneOperatorStatus) -> Vec<DashboardRow> {
    let mut rows = vec![row("Prune mode", bool_text(status.pruned))];
    if let Some(height) = prune_height_value(status) {
        rows.push(row("Prune height", height));
    }
    if let Some(automatic) = status.maybe_automatic_pruning {
        rows.push(row("Automatic pruning", bool_text(automatic)));
    }
    if let Some(target) = status.maybe_prune_target_size {
        rows.push(row("Prune target", target.to_string()));
    }
    rows.extend(lock_rows(&status.locks));
    rows.push(manual_prune_row(&status.manual_prune));
    rows
}

fn prune_height_value(status: &PruneOperatorStatus) -> Option<String> {
    match &status.pruneheight {
        FieldAvailability::Available(Some(height)) => Some(height.to_string()),
        FieldAvailability::Available(None) => None,
        FieldAvailability::Unavailable { reason } if status.pruned => {
            Some(format!("Unavailable: {reason}"))
        }
        FieldAvailability::Unavailable { .. } => None,
    }
}

fn lock_rows(locks: &FieldAvailability<Vec<PruneLockRow>>) -> Vec<DashboardRow> {
    match locks {
        FieldAvailability::Available(locks) if locks.is_empty() => {
            vec![row("Prune locks", "none")]
        }
        FieldAvailability::Available(locks) => locks
            .iter()
            .map(|lock| {
                row(
                    "Prune lock",
                    format!(
                        "name={} height_first={} height_last={}",
                        lock.name, lock.height_first, lock.height_last
                    ),
                )
            })
            .collect(),
        FieldAvailability::Unavailable { reason } => {
            vec![row("Prune locks", format!("Unavailable: {reason}"))]
        }
    }
}

fn manual_prune_row(manual_prune: &FieldAvailability<ManualPruneSurface>) -> DashboardRow {
    let value = match manual_prune {
        FieldAvailability::Available(ManualPruneSurface::None) => "none".to_string(),
        FieldAvailability::Available(ManualPruneSurface::Height { height }) => {
            format!("height={height}")
        }
        FieldAvailability::Available(ManualPruneSurface::Refused { reason }) => {
            refusal_text(*reason).to_string()
        }
        FieldAvailability::Unavailable { reason } => format!("Unavailable: {reason}"),
    };
    row("Manual prune", value)
}

fn refusal_text(reason: ManualPruneRefusalCode) -> &'static str {
    match reason {
        ManualPruneRefusalCode::Disabled => "refused node is not in prune mode",
        ManualPruneRefusalCode::ChainTooShort => "refused blockchain is too short for pruning",
        ManualPruneRefusalCode::TargetAboveTip => {
            "refused blockchain is shorter than the attempted prune height"
        }
        ManualPruneRefusalCode::KeepWindow => "refused target is inside the 288-block keep window",
        ManualPruneRefusalCode::NegativeHeight => "refused negative block height",
        ManualPruneRefusalCode::TimestampNotFound => {
            "refused block at the requested timestamp was not found"
        }
    }
}

fn bool_text(value: bool) -> &'static str {
    if value { "true" } else { "false" }
}
