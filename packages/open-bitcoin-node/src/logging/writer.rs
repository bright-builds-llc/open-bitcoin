// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Managed JSONL structured log writer and status reader.

use std::{
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
};

use super::{
    LogPathStatus, LogRetentionPolicy, LogStatus, StructuredLogError, StructuredLogRecord,
    prune::{LogFileMetadata, SECONDS_PER_DAY, maybe_managed_log_unix_day, plan_log_retention},
    recent_log_signals_from_records,
};

pub fn append_structured_log_record(
    log_dir: &Path,
    record: &StructuredLogRecord,
    retention: LogRetentionPolicy,
) -> Result<PathBuf, StructuredLogError> {
    fs::create_dir_all(log_dir)
        .map_err(|source| StructuredLogError::io("create log directory", log_dir, source))?;

    let path = structured_log_path(log_dir, record.timestamp_unix_seconds);
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .map_err(|source| StructuredLogError::io("open structured log", &path, source))?;
    let encoded = encode_record(record)?;
    file.write_all(&encoded)
        .map_err(|source| StructuredLogError::io("write structured log", &path, source))?;

    prune_log_directory(log_dir, retention, record.timestamp_unix_seconds)?;
    Ok(path)
}

pub fn load_log_status(log_dir: &Path, retention: LogRetentionPolicy, limit: usize) -> LogStatus {
    if !log_dir.is_dir() {
        return unavailable_log_status(log_dir, retention);
    }

    let records = match load_managed_records(log_dir) {
        Ok(records) => records,
        Err(_) => return unavailable_log_status(log_dir, retention),
    };

    LogStatus {
        path: LogPathStatus::available(log_dir.display().to_string()),
        retention,
        recent_signals: recent_log_signals_from_records(&records, limit),
    }
}

fn structured_log_path(log_dir: &Path, timestamp_unix_seconds: u64) -> PathBuf {
    let unix_day = timestamp_unix_seconds / SECONDS_PER_DAY;
    log_dir.join(format!("open-bitcoin-runtime-{unix_day}.jsonl"))
}

fn encode_record(record: &StructuredLogRecord) -> Result<Vec<u8>, StructuredLogError> {
    let mut encoded =
        serde_json::to_vec(record).map_err(|source| StructuredLogError::Json { source })?;
    encoded.push(b'\n');
    Ok(encoded)
}

fn prune_log_directory(
    log_dir: &Path,
    retention: LogRetentionPolicy,
    now_unix_seconds: u64,
) -> Result<(), StructuredLogError> {
    let files = collect_log_file_metadata(log_dir)?;
    for path in plan_log_retention(&files, retention, now_unix_seconds) {
        remove_managed_log_file(&path)?;
    }

    Ok(())
}

fn collect_log_file_metadata(log_dir: &Path) -> Result<Vec<LogFileMetadata>, StructuredLogError> {
    let entries = fs::read_dir(log_dir)
        .map_err(|source| StructuredLogError::io("read log directory", log_dir, source))?;
    let mut files = Vec::new();

    for entry in entries {
        let entry = entry.map_err(|source| {
            StructuredLogError::io("read log directory entry", log_dir, source)
        })?;
        let path = entry.path();
        let metadata = entry
            .metadata()
            .map_err(|source| StructuredLogError::io("read log metadata", &path, source))?;
        if metadata.is_file() {
            files.push(LogFileMetadata::new(path, metadata.len()));
        }
    }

    Ok(files)
}

fn remove_managed_log_file(path: &Path) -> Result<(), StructuredLogError> {
    match fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(source) if source.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(source) => Err(StructuredLogError::io(
            "remove structured log",
            path,
            source,
        )),
    }
}

fn load_managed_records(log_dir: &Path) -> Result<Vec<StructuredLogRecord>, std::io::Error> {
    let mut managed_paths = managed_log_paths(log_dir)?;
    managed_paths.sort_by(|left, right| left.0.cmp(&right.0).then_with(|| left.1.cmp(&right.1)));

    let mut records = Vec::new();
    for (_, path) in managed_paths {
        load_file_records(&path, &mut records)?;
    }

    Ok(records)
}

fn managed_log_paths(log_dir: &Path) -> Result<Vec<(u64, PathBuf)>, std::io::Error> {
    let mut paths = Vec::new();
    for entry in fs::read_dir(log_dir)? {
        let entry = entry?;
        let path = entry.path();
        if !entry.metadata()?.is_file() {
            continue;
        }
        let Some(unix_day) = maybe_managed_log_unix_day(&path) else {
            continue;
        };
        paths.push((unix_day, path));
    }

    Ok(paths)
}

fn load_file_records(
    path: &Path,
    records: &mut Vec<StructuredLogRecord>,
) -> Result<(), std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    for line_result in reader.lines() {
        let line = line_result?;
        if line.trim().is_empty() {
            continue;
        }
        let Ok(record) = serde_json::from_str::<StructuredLogRecord>(&line) else {
            continue;
        };
        records.push(record);
    }

    Ok(())
}

fn unavailable_log_status(log_dir: &Path, retention: LogRetentionPolicy) -> LogStatus {
    LogStatus {
        path: LogPathStatus::unavailable(format!("log path unavailable: {}", log_dir.display())),
        retention,
        recent_signals: Vec::new(),
    }
}
