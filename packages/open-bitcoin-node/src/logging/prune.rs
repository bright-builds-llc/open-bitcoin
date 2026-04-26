// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Pure structured log retention planning.

use std::{
    collections::BTreeSet,
    path::{Path, PathBuf},
};

use super::LogRetentionPolicy;

pub const SECONDS_PER_DAY: u64 = 86_400;

const MANAGED_LOG_FILE_PREFIX: &str = "open-bitcoin-runtime-";
const MANAGED_LOG_FILE_SUFFIX: &str = ".jsonl";

/// Filesystem metadata used by the pure retention planner.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogFileMetadata {
    pub path: PathBuf,
    pub size_bytes: u64,
}

impl LogFileMetadata {
    pub fn new(path: impl Into<PathBuf>, size_bytes: u64) -> Self {
        Self {
            path: path.into(),
            size_bytes,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ManagedLogFile {
    path: PathBuf,
    size_bytes: u64,
    unix_day: u64,
}

pub fn plan_log_retention(
    files: &[LogFileMetadata],
    retention: LogRetentionPolicy,
    now_unix_seconds: u64,
) -> Vec<PathBuf> {
    let now_unix_day = now_unix_seconds / SECONDS_PER_DAY;
    let mut managed_files: Vec<ManagedLogFile> =
        files.iter().filter_map(managed_log_file).collect();

    managed_files.sort_by(compare_newest_first);

    let mut selected = BTreeSet::new();
    select_expired_files(&managed_files, retention, now_unix_day, &mut selected);
    select_files_over_count_cap(&managed_files, retention, &mut selected);
    select_files_over_byte_cap(&managed_files, retention, &mut selected);

    selected.into_iter().collect()
}

pub fn maybe_managed_log_unix_day(path: &Path) -> Option<u64> {
    let file_name = path.file_name()?.to_str()?;
    let maybe_day = file_name
        .strip_prefix(MANAGED_LOG_FILE_PREFIX)?
        .strip_suffix(MANAGED_LOG_FILE_SUFFIX)?;

    if maybe_day.is_empty()
        || !maybe_day
            .chars()
            .all(|character| character.is_ascii_digit())
    {
        return None;
    }

    maybe_day.parse().ok()
}

fn managed_log_file(file: &LogFileMetadata) -> Option<ManagedLogFile> {
    let unix_day = maybe_managed_log_unix_day(&file.path)?;
    Some(ManagedLogFile {
        path: file.path.clone(),
        size_bytes: file.size_bytes,
        unix_day,
    })
}

fn select_expired_files(
    managed_files: &[ManagedLogFile],
    retention: LogRetentionPolicy,
    now_unix_day: u64,
    selected: &mut BTreeSet<PathBuf>,
) {
    let max_age_days = u64::from(retention.max_age_days);
    for file in managed_files {
        let age_days = now_unix_day.saturating_sub(file.unix_day);
        if age_days > max_age_days {
            selected.insert(file.path.clone());
        }
    }
}

fn select_files_over_count_cap(
    managed_files: &[ManagedLogFile],
    retention: LogRetentionPolicy,
    selected: &mut BTreeSet<PathBuf>,
) {
    let max_files = usize::from(retention.max_files);
    let retained_files = managed_files
        .iter()
        .filter(|file| !selected.contains(&file.path));
    let over_cap_paths: Vec<PathBuf> = retained_files
        .skip(max_files)
        .map(|file| file.path.clone())
        .collect();

    for path in over_cap_paths {
        selected.insert(path);
    }
}

fn select_files_over_byte_cap(
    managed_files: &[ManagedLogFile],
    retention: LogRetentionPolicy,
    selected: &mut BTreeSet<PathBuf>,
) {
    let mut retained_bytes = 0_u64;
    let retained_files: Vec<&ManagedLogFile> = managed_files
        .iter()
        .filter(|file| !selected.contains(&file.path))
        .collect();
    let mut over_cap_paths = Vec::new();

    for file in retained_files {
        let next_total = retained_bytes.saturating_add(file.size_bytes);
        if next_total <= retention.max_total_bytes {
            retained_bytes = next_total;
            continue;
        }

        over_cap_paths.push(file.path.clone());
    }

    for path in over_cap_paths {
        selected.insert(path);
    }
}

fn compare_newest_first(left: &ManagedLogFile, right: &ManagedLogFile) -> std::cmp::Ordering {
    right
        .unix_day
        .cmp(&left.unix_day)
        .then_with(|| left.path.cmp(&right.path))
}
