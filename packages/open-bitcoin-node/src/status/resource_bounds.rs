// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Pure resource-bound status contracts and classification.

use serde::{Deserialize, Serialize};

use super::FieldAvailability;

pub const RESOURCE_BOUND_WARNING_PERCENT: u8 = 80;
pub const RESOURCE_BOUND_STOP_PERCENT: u8 = 95;
pub const REQUIRED_RESOURCE_MEASUREMENTS_UNAVAILABLE: &str =
    "required resource measurements unavailable";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResourcePressureState {
    Normal,
    Warning,
    StopRequired,
}

impl ResourcePressureState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::Warning => "warning",
            Self::StopRequired => "stop_required",
        }
    }
}

pub type ResourcePressureLevel = ResourcePressureState;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResourceBoundKind {
    Disk,
    File,
    Cache,
    Queue,
    Peer,
    InFlight,
    Log,
    Metric,
    SupportBundle,
}

impl ResourceBoundKind {
    pub const ALL: [Self; 9] = [
        Self::Disk,
        Self::File,
        Self::Cache,
        Self::Queue,
        Self::Peer,
        Self::InFlight,
        Self::Log,
        Self::Metric,
        Self::SupportBundle,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Disk => "disk",
            Self::File => "file",
            Self::Cache => "cache",
            Self::Queue => "queue",
            Self::Peer => "peer",
            Self::InFlight => "in_flight",
            Self::Log => "log",
            Self::Metric => "metric",
            Self::SupportBundle => "support_bundle",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResourceBoundUnit {
    Bytes,
    Files,
    Items,
    Peers,
    Requests,
}

impl ResourceBoundUnit {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Bytes => "bytes",
            Self::Files => "files",
            Self::Items => "items",
            Self::Peers => "peers",
            Self::Requests => "requests",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceBoundUsage {
    pub current: u64,
    pub unit: ResourceBoundUnit,
    pub state: ResourcePressureState,
    pub maybe_limit: Option<u64>,
    pub maybe_warning_threshold: Option<u64>,
    pub maybe_stop_threshold: Option<u64>,
    pub next_action: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceBoundEntry {
    pub kind: ResourceBoundKind,
    pub label: String,
    pub usage: FieldAvailability<ResourceBoundUsage>,
}

impl ResourceBoundEntry {
    pub fn available(
        kind: ResourceBoundKind,
        label: impl Into<String>,
        usage: ResourceBoundUsage,
    ) -> Self {
        Self {
            kind,
            label: label.into(),
            usage: FieldAvailability::available(usage),
        }
    }

    pub fn unavailable(
        kind: ResourceBoundKind,
        label: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            label: label.into(),
            usage: FieldAvailability::unavailable(reason),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceBoundSnapshot {
    pub overall_level: ResourcePressureState,
    pub entries: Vec<ResourceBoundEntry>,
}

impl ResourceBoundSnapshot {
    pub fn new(entries: Vec<ResourceBoundEntry>) -> Self {
        let overall_level = classify_resource_bound_entries(&entries);
        Self {
            overall_level,
            entries,
        }
    }

    pub fn missing_required_measurements() -> Self {
        Self::new(
            ResourceBoundKind::ALL
                .into_iter()
                .map(|kind| {
                    ResourceBoundEntry::unavailable(
                        kind,
                        kind.as_str(),
                        REQUIRED_RESOURCE_MEASUREMENTS_UNAVAILABLE,
                    )
                })
                .collect(),
        )
    }

    pub fn has_unavailable_required_measurements(&self) -> bool {
        if ResourceBoundKind::ALL
            .iter()
            .any(|kind| !self.entries.iter().any(|entry| entry.kind == *kind))
        {
            return true;
        }
        self.entries
            .iter()
            .any(|entry| matches!(entry.usage, FieldAvailability::Unavailable { .. }))
    }
}

impl Default for FieldAvailability<ResourceBoundSnapshot> {
    fn default() -> Self {
        Self::unavailable("resource bounds unavailable")
    }
}

pub fn classify_budget_pressure(current: u64, limit: u64) -> ResourcePressureState {
    if limit == 0 {
        return ResourcePressureState::StopRequired;
    }
    let warning_threshold = percentage_threshold(limit, RESOURCE_BOUND_WARNING_PERCENT);
    let stop_threshold = percentage_threshold(limit, RESOURCE_BOUND_STOP_PERCENT);
    if current >= stop_threshold {
        ResourcePressureState::StopRequired
    } else if current >= warning_threshold {
        ResourcePressureState::Warning
    } else {
        ResourcePressureState::Normal
    }
}

pub fn usage_against_budget(
    current: u64,
    limit: u64,
    unit: ResourceBoundUnit,
    next_action: impl Into<String>,
) -> ResourceBoundUsage {
    ResourceBoundUsage {
        current,
        unit,
        state: classify_budget_pressure(current, limit),
        maybe_limit: Some(limit),
        maybe_warning_threshold: Some(percentage_threshold(limit, RESOURCE_BOUND_WARNING_PERCENT)),
        maybe_stop_threshold: Some(percentage_threshold(limit, RESOURCE_BOUND_STOP_PERCENT)),
        next_action: next_action.into(),
    }
}

pub fn classify_snapshot_against_disk_budget(
    snapshot: &ResourceBoundSnapshot,
    disk_budget_bytes: u64,
) -> ResourcePressureState {
    snapshot
        .entries
        .iter()
        .filter(|entry| entry.kind == ResourceBoundKind::Disk)
        .map(|entry| match &entry.usage {
            FieldAvailability::Available(usage) => {
                classify_budget_pressure(usage.current, disk_budget_bytes)
            }
            FieldAvailability::Unavailable { .. } => ResourcePressureState::StopRequired,
        })
        .max()
        .unwrap_or(ResourcePressureState::StopRequired)
}

pub fn classify_resource_bound_entries(entries: &[ResourceBoundEntry]) -> ResourcePressureState {
    entries
        .iter()
        .filter_map(|entry| match &entry.usage {
            FieldAvailability::Available(usage) => Some(usage.state),
            FieldAvailability::Unavailable { .. } => None,
        })
        .max()
        .unwrap_or(ResourcePressureState::Normal)
}

const fn percentage_threshold(limit: u64, percent: u8) -> u64 {
    let numerator = limit.saturating_mul(percent as u64);
    numerator.saturating_add(99) / 100
}
