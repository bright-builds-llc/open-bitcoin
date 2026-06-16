// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Local resource-bound probes for operator status.

use std::{collections::VecDeque, fs, path::Path};

use open_bitcoin_node::{
    DurableSyncState, FieldAvailability, LogRetentionPolicy, MetricRetentionPolicy,
    status::{
        ResourceBoundEntry, ResourceBoundKind, ResourceBoundSnapshot, ResourceBoundUnit,
        ResourceBoundUsage, ResourcePressureState, usage_against_budget,
    },
};

use super::super::config::OperatorConfigResolution;

const MAX_RESOURCE_WALK_ENTRIES: usize = 2_048;
const MAX_DATADIR_FILES: u64 = 100_000;
const CACHE_BUDGET_BYTES: u64 = 512 * 1024 * 1024;
const SUPPORT_BUNDLE_BUDGET_BYTES: u64 = 16 * 1024 * 1024;

pub(crate) fn collect_resource_bounds(
    resolution: &OperatorConfigResolution,
    maybe_durable_sync_state: Option<&DurableSyncState>,
) -> FieldAvailability<ResourceBoundSnapshot> {
    let entries = vec![
        disk_entry(resolution),
        file_entry(resolution),
        cache_entry(resolution),
        queue_entry(maybe_durable_sync_state),
        peer_entry(maybe_durable_sync_state),
        in_flight_entry(maybe_durable_sync_state),
        log_entry(resolution),
        metric_entry(resolution),
        support_bundle_entry(resolution),
    ];
    FieldAvailability::available(ResourceBoundSnapshot::new(entries))
}

fn disk_entry(resolution: &OperatorConfigResolution) -> ResourceBoundEntry {
    let Some(datadir) = resolution.maybe_data_dir.as_deref() else {
        return unavailable(
            ResourceBoundKind::Disk,
            "selected datadir filesystem",
            "datadir unavailable for disk resource bounds",
        );
    };
    if !datadir.is_dir() {
        return unavailable(
            ResourceBoundKind::Disk,
            "selected datadir filesystem",
            format!(
                "datadir {} is missing or not a directory",
                datadir.display()
            ),
        );
    }

    let available_space = fs4::available_space(datadir);
    let free_space = fs4::free_space(datadir);
    let total_space = fs4::total_space(datadir);
    let (Ok(_available_space), Ok(_free_space), Ok(total_space)) =
        (available_space, free_space, total_space)
    else {
        return unavailable(
            ResourceBoundKind::Disk,
            "selected datadir filesystem",
            "selected filesystem capacity unavailable",
        );
    };
    let footprint = match directory_footprint(datadir) {
        Ok(footprint) => footprint,
        Err(reason) => {
            return unavailable(ResourceBoundKind::Disk, "selected datadir disk", reason);
        }
    };
    ResourceBoundEntry::available(
        ResourceBoundKind::Disk,
        "selected datadir disk",
        usage_against_budget(
            footprint.bytes,
            total_space.max(1),
            ResourceBoundUnit::Bytes,
            "Free disk space for the selected datadir before continuing.",
        ),
    )
}

fn file_entry(resolution: &OperatorConfigResolution) -> ResourceBoundEntry {
    let Some(datadir) = resolution.maybe_data_dir.as_deref() else {
        return unavailable(
            ResourceBoundKind::File,
            "datadir file count",
            "datadir unavailable for file-count bounds",
        );
    };
    match directory_footprint(datadir) {
        Ok(footprint) => ResourceBoundEntry::available(
            ResourceBoundKind::File,
            "datadir file count",
            usage_against_budget(
                footprint.files,
                MAX_DATADIR_FILES,
                ResourceBoundUnit::Files,
                "Review datadir growth and retention before continuing.",
            ),
        ),
        Err(reason) => unavailable(ResourceBoundKind::File, "datadir file count", reason),
    }
}

fn cache_entry(resolution: &OperatorConfigResolution) -> ResourceBoundEntry {
    let Some(datadir) = resolution.maybe_data_dir.as_deref() else {
        return unavailable(
            ResourceBoundKind::Cache,
            "cache footprint",
            "datadir unavailable for cache bounds",
        );
    };
    let cache_dir = datadir.join("cache");
    let footprint = directory_footprint_or_zero(&cache_dir);
    ResourceBoundEntry::available(
        ResourceBoundKind::Cache,
        "cache footprint",
        usage_against_budget(
            footprint.bytes,
            CACHE_BUDGET_BYTES,
            ResourceBoundUnit::Bytes,
            "Review cache growth before continuing.",
        ),
    )
}

fn queue_entry(maybe_durable_sync_state: Option<&DurableSyncState>) -> ResourceBoundEntry {
    let Some(sync_state) = maybe_durable_sync_state else {
        return unavailable(
            ResourceBoundKind::Queue,
            "request queue bounds",
            "durable sync state unavailable for request queue bounds",
        );
    };
    match &sync_state.sync.resource_pressure {
        FieldAvailability::Available(pressure) => ResourceBoundEntry::available(
            ResourceBoundKind::Queue,
            "request queue bounds",
            usage_against_budget(
                pressure.blocks_in_flight,
                pressure.max_blocks_in_flight_total.max(1),
                ResourceBoundUnit::Requests,
                "Pause sync or reduce in-flight work before queue pressure grows.",
            ),
        ),
        FieldAvailability::Unavailable { reason } => unavailable(
            ResourceBoundKind::Queue,
            "request queue bounds",
            reason.clone(),
        ),
    }
}

fn peer_entry(maybe_durable_sync_state: Option<&DurableSyncState>) -> ResourceBoundEntry {
    let Some(sync_state) = maybe_durable_sync_state else {
        return unavailable(
            ResourceBoundKind::Peer,
            "peer bounds",
            "durable sync state unavailable for peer bounds",
        );
    };
    match &sync_state.sync.resource_pressure {
        FieldAvailability::Available(pressure) => ResourceBoundEntry::available(
            ResourceBoundKind::Peer,
            "peer bounds",
            peer_bound_usage(
                observed_outbound_peers(sync_state, pressure.outbound_peers),
                u64::from(pressure.target_outbound_peers),
                "Review peer configuration before continuing.",
            ),
        ),
        FieldAvailability::Unavailable { reason } => {
            unavailable(ResourceBoundKind::Peer, "peer bounds", reason.clone())
        }
    }
}

fn observed_outbound_peers(sync_state: &DurableSyncState, fallback: u32) -> u64 {
    match &sync_state.peers.peer_counts {
        FieldAvailability::Available(peer_counts) => u64::from(peer_counts.outbound),
        FieldAvailability::Unavailable { .. } => u64::from(fallback),
    }
}

fn peer_bound_usage(
    current: u64,
    target_outbound_peers: u64,
    next_action: impl Into<String>,
) -> ResourceBoundUsage {
    let maybe_stop_threshold = target_outbound_peers.checked_add(1);
    ResourceBoundUsage {
        current,
        unit: ResourceBoundUnit::Peers,
        state: if current > target_outbound_peers {
            ResourcePressureState::StopRequired
        } else {
            ResourcePressureState::Normal
        },
        maybe_limit: Some(target_outbound_peers),
        maybe_warning_threshold: Some(target_outbound_peers),
        maybe_stop_threshold,
        next_action: next_action.into(),
    }
}

fn in_flight_entry(maybe_durable_sync_state: Option<&DurableSyncState>) -> ResourceBoundEntry {
    let Some(sync_state) = maybe_durable_sync_state else {
        return unavailable(
            ResourceBoundKind::InFlight,
            "in-flight block bounds",
            "durable sync state unavailable for in-flight bounds",
        );
    };
    match &sync_state.sync.resource_pressure {
        FieldAvailability::Available(pressure) => ResourceBoundEntry::available(
            ResourceBoundKind::InFlight,
            "in-flight block bounds",
            usage_against_budget(
                pressure.blocks_in_flight,
                pressure.max_blocks_in_flight_total.max(1),
                ResourceBoundUnit::Requests,
                "Pause sync before in-flight pressure reaches configured bounds.",
            ),
        ),
        FieldAvailability::Unavailable { reason } => unavailable(
            ResourceBoundKind::InFlight,
            "in-flight block bounds",
            reason.clone(),
        ),
    }
}

fn log_entry(resolution: &OperatorConfigResolution) -> ResourceBoundEntry {
    let policy = resolution
        .maybe_open_bitcoin_config
        .as_ref()
        .map(|config| config.logs.max_total_bytes)
        .unwrap_or_else(|| LogRetentionPolicy::default().max_total_bytes);
    let Some(log_dir) = resolution.maybe_log_dir.as_deref() else {
        return unavailable(
            ResourceBoundKind::Log,
            "structured log retention",
            "log directory unavailable for log resource bounds",
        );
    };
    let footprint = directory_footprint_or_zero(log_dir);
    ResourceBoundEntry::available(
        ResourceBoundKind::Log,
        "structured log retention",
        usage_against_budget(
            footprint.bytes,
            policy.max(1),
            ResourceBoundUnit::Bytes,
            "Prune or rotate structured logs before continuing.",
        ),
    )
}

fn metric_entry(resolution: &OperatorConfigResolution) -> ResourceBoundEntry {
    let policy = resolution
        .maybe_open_bitcoin_config
        .as_ref()
        .map(|config| config.metrics.max_samples_per_series.saturating_mul(11))
        .unwrap_or_else(|| MetricRetentionPolicy::default().max_samples_per_series * 11);
    let Some(metrics_path) = resolution.maybe_metrics_store_path.as_deref() else {
        return unavailable(
            ResourceBoundKind::Metric,
            "metrics retention",
            "metrics store unavailable for metric resource bounds",
        );
    };
    let footprint = directory_footprint_or_zero(metrics_path);
    ResourceBoundEntry::available(
        ResourceBoundKind::Metric,
        "metrics retention",
        usage_against_budget(
            footprint.files,
            u64::try_from(policy).unwrap_or(u64::MAX).max(1),
            ResourceBoundUnit::Items,
            "Review metrics retention before continuing.",
        ),
    )
}

fn support_bundle_entry(resolution: &OperatorConfigResolution) -> ResourceBoundEntry {
    let Some(datadir) = resolution.maybe_data_dir.as_deref() else {
        return unavailable(
            ResourceBoundKind::SupportBundle,
            "support-bundle projection",
            "datadir unavailable for support-bundle bounds",
        );
    };
    let support_dir = datadir.join("support");
    let footprint = directory_footprint_or_zero(&support_dir);
    ResourceBoundEntry::available(
        ResourceBoundKind::SupportBundle,
        "support-bundle projection",
        usage_against_budget(
            footprint.bytes,
            SUPPORT_BUNDLE_BUDGET_BYTES,
            ResourceBoundUnit::Bytes,
            "Generate compact support evidence and avoid copying raw logs or stores.",
        ),
    )
}

fn unavailable(
    kind: ResourceBoundKind,
    label: impl Into<String>,
    reason: impl Into<String>,
) -> ResourceBoundEntry {
    ResourceBoundEntry::unavailable(kind, label, reason)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct DirectoryFootprint {
    bytes: u64,
    files: u64,
}

fn directory_footprint_or_zero(path: &Path) -> DirectoryFootprint {
    if !path.exists() {
        return DirectoryFootprint::default();
    }
    directory_footprint(path).unwrap_or_default()
}

fn directory_footprint(path: &Path) -> Result<DirectoryFootprint, String> {
    let mut pending = VecDeque::from([path.to_path_buf()]);
    let mut footprint = DirectoryFootprint::default();
    let mut visited = 0usize;

    while let Some(next_path) = pending.pop_front() {
        visited = visited.saturating_add(1);
        if visited > MAX_RESOURCE_WALK_ENTRIES {
            break;
        }
        let metadata = fs::symlink_metadata(&next_path)
            .map_err(|error| format!("could not inspect {}: {error}", next_path.display()))?;
        if metadata.is_file() {
            footprint.files = footprint.files.saturating_add(1);
            footprint.bytes = footprint.bytes.saturating_add(metadata.len());
            continue;
        }
        if !metadata.is_dir() {
            continue;
        }
        let entries = fs::read_dir(&next_path)
            .map_err(|error| format!("could not read {}: {error}", next_path.display()))?;
        for entry in entries {
            let entry = entry.map_err(|error| {
                format!("could not inspect {} entry: {error}", next_path.display())
            })?;
            pending.push_back(entry.path());
        }
    }

    Ok(footprint)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_resource_bounds_missing_datadir_reports_all_required_kinds() {
        // Arrange
        let resolution = OperatorConfigResolution::default();

        // Act
        let bounds = collect_resource_bounds(&resolution, None);

        // Assert
        let FieldAvailability::Available(snapshot) = bounds else {
            panic!("resource bounds should be available with per-entry reasons");
        };
        assert_eq!(snapshot.entries.len(), ResourceBoundKind::ALL.len());
        assert!(
            snapshot
                .entries
                .iter()
                .any(|entry| entry.kind == ResourceBoundKind::Disk)
        );
        assert!(
            snapshot
                .entries
                .iter()
                .any(|entry| entry.kind == ResourceBoundKind::SupportBundle)
        );
    }

    #[test]
    fn status_resource_bounds_directory_footprint_is_bounded() {
        // Arrange
        let root = std::env::temp_dir().join(format!(
            "open-bitcoin-resource-bounds-{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("nested")).expect("create fixture dir");
        fs::write(root.join("nested").join("sample.log"), b"sample").expect("write fixture");

        // Act
        let footprint = directory_footprint(&root).expect("footprint");

        // Assert
        assert_eq!(footprint.files, 1);
        assert_eq!(footprint.bytes, 6);

        fs::remove_dir_all(root).expect("remove fixture");
    }

    #[test]
    fn status_resource_bounds_peer_target_is_not_stop_required() {
        // Arrange / Act
        let at_target = peer_bound_usage(4, 4, "Review peer configuration before continuing.");
        let over_target = peer_bound_usage(5, 4, "Review peer configuration before continuing.");

        // Assert
        assert_eq!(at_target.state, ResourcePressureState::Normal);
        assert_eq!(at_target.maybe_limit, Some(4));
        assert_eq!(over_target.state, ResourcePressureState::StopRequired);
        assert_eq!(over_target.maybe_stop_threshold, Some(5));
    }
}
