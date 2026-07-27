// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::{
    BLOCK_RELAY_LOG_SOURCE, INBOUND_PEER_POLICY_LOG_SOURCE, INBOUND_RESOURCE_GOVERNANCE_LOG_SOURCE,
    LogPathStatus, LogRetentionPolicy, LogRotation, LogStatus, RELAY_MEMPOOL_LOG_SOURCE,
    RecentLogSignal, StructuredLogLevel, StructuredLogRecord, block_relay_log_record,
    health_signals_from_recent_logs, inbound_peer_policy_log_record,
    inbound_resource_governance_log_record, recent_log_signals_from_records,
    relay_mempool_log_record,
};
use super::{
    prune::{LogFileMetadata, plan_log_retention},
    writer::{append_structured_log_record, load_log_status},
};
use crate::status::{
    BlockRelayEvidenceStatus, HealthSignalLevel, InboundPeerPolicyEvent,
    InboundResourceGovernanceEvent,
    relay_evidence::{
        RelayEvidenceCounters, RelayEvidenceField, RelayEvidenceStatus, RelayRecoveryCounters,
    },
};
use std::{
    fs,
    path::PathBuf,
    process,
    sync::atomic::{AtomicU64, Ordering},
};

static NEXT_TEMP_DIR: AtomicU64 = AtomicU64::new(0);

fn test_log_dir(name: &str) -> PathBuf {
    let counter = NEXT_TEMP_DIR.fetch_add(1, Ordering::SeqCst);
    let path = std::env::temp_dir().join(format!(
        "open-bitcoin-logging-{name}-{}-{counter}",
        process::id()
    ));
    if path.exists() {
        fs::remove_dir_all(&path).expect("remove stale test directory");
    }
    path
}

fn log_metadata(unix_day: u64, size_bytes: u64) -> LogFileMetadata {
    LogFileMetadata::new(managed_path(unix_day), size_bytes)
}

fn managed_path(unix_day: u64) -> PathBuf {
    PathBuf::from(format!("open-bitcoin-runtime-{unix_day}.jsonl"))
}

mod contracts;
mod persistence;
mod redaction;
mod retention;
mod signals;
