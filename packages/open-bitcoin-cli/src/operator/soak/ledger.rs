// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Datadir-owned soak ledger.

use std::{
    fs::{self, File, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use super::{SoakBounds, SoakRunId, outcome::SoakOutcomeLabel};

pub(crate) const SOAK_LEDGER_SCHEMA_VERSION: u16 = 1;
pub(crate) const MAX_SOAK_EVENT_BYTES: usize = 16 * 1024;
pub(crate) const MAX_SOAK_RUNS_IN_INDEX: usize = 32;

const SOAK_DIR: &str = "soak";
const RUNS_DIR: &str = "runs";
const RUN_INDEX_FILE: &str = "run-index.json";
const RUN_INDEX_TMP_FILE: &str = "run-index.json.tmp";
const EVENTS_FILE: &str = "events.jsonl";
const REPORT_JSON_FILE: &str = "report.json";
const REPORT_MARKDOWN_FILE: &str = "report.md";

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SoakLedgerLayout {
    datadir: PathBuf,
}

impl SoakLedgerLayout {
    pub(crate) fn for_datadir(datadir: impl AsRef<Path>) -> Self {
        Self {
            datadir: datadir.as_ref().to_path_buf(),
        }
    }

    pub(crate) fn datadir(&self) -> PathBuf {
        self.datadir.clone()
    }

    pub(crate) fn run_index_path(&self) -> PathBuf {
        self.soak_dir().join(RUN_INDEX_FILE)
    }

    pub(crate) fn run_index_tmp_path(&self) -> PathBuf {
        self.soak_dir().join(RUN_INDEX_TMP_FILE)
    }

    pub(crate) fn paths_for_run(&self, run_id: &SoakRunId) -> SoakRunPaths {
        let run_dir = self.runs_dir().join(run_id.as_str());
        SoakRunPaths {
            events_path: run_dir.join(EVENTS_FILE),
            report_json_path: run_dir.join(REPORT_JSON_FILE),
            report_markdown_path: run_dir.join(REPORT_MARKDOWN_FILE),
            run_dir,
        }
    }

    fn soak_dir(&self) -> PathBuf {
        self.datadir.join(SOAK_DIR)
    }

    fn runs_dir(&self) -> PathBuf {
        self.soak_dir().join(RUNS_DIR)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SoakRunPaths {
    pub(crate) run_dir: PathBuf,
    pub(crate) events_path: PathBuf,
    pub(crate) report_json_path: PathBuf,
    pub(crate) report_markdown_path: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct SoakRunIndex {
    pub(crate) schema_version: u16,
    pub(crate) runs: Vec<SoakRunIndexEntry>,
}

impl SoakRunIndex {
    pub(crate) fn empty() -> Self {
        Self {
            schema_version: SOAK_LEDGER_SCHEMA_VERSION,
            runs: Vec::new(),
        }
    }

    pub(crate) fn record_run(&mut self, entry: SoakRunIndexEntry) {
        let run_id = entry.run_id.clone();
        self.runs.retain(|existing| existing.run_id != run_id);
        self.runs.push(entry);
        self.runs.sort_by(|left, right| {
            right
                .updated_at_unix_seconds
                .cmp(&left.updated_at_unix_seconds)
                .then_with(|| {
                    right
                        .started_at_unix_seconds
                        .cmp(&left.started_at_unix_seconds)
                })
                .then_with(|| right.run_id.as_str().cmp(left.run_id.as_str()))
        });
        self.runs.truncate(MAX_SOAK_RUNS_IN_INDEX);
    }

    pub(crate) fn write_atomic(&self, layout: &SoakLedgerLayout) -> Result<(), SoakLedgerError> {
        let index_path = layout.run_index_path();
        let tmp_path = layout.run_index_tmp_path();
        let Some(parent) = index_path.parent() else {
            return Err(SoakLedgerError::MissingParent {
                path: index_path,
                action: "write run index",
            });
        };
        fs::create_dir_all(parent).map_err(|source| SoakLedgerError::Io {
            path: parent.to_path_buf(),
            action: "create run index directory",
            source,
        })?;

        let mut encoded = serde_json::to_vec_pretty(self).map_err(SoakLedgerError::Encode)?;
        encoded.push(b'\n');
        let mut file = File::create(&tmp_path).map_err(|source| SoakLedgerError::Io {
            path: tmp_path.clone(),
            action: "create run index tmp file",
            source,
        })?;
        file.write_all(&encoded)
            .and_then(|()| file.flush())
            .and_then(|()| file.sync_all())
            .map_err(|source| SoakLedgerError::Io {
                path: tmp_path.clone(),
                action: "write run index tmp file",
                source,
            })?;
        fs::rename(&tmp_path, &index_path).map_err(|source| SoakLedgerError::Io {
            path: index_path.clone(),
            action: "rename run index tmp file",
            source,
        })?;
        sync_parent_dir(&index_path)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct SoakRunIndexEntry {
    pub(crate) run_id: SoakRunId,
    pub(crate) ledger_path: PathBuf,
    pub(crate) started_at_unix_seconds: u64,
    pub(crate) updated_at_unix_seconds: u64,
    pub(crate) maybe_outcome: Option<SoakOutcomeLabel>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct SoakLedgerEventEnvelope {
    pub(crate) schema_version: u16,
    pub(crate) run_id: SoakRunId,
    pub(crate) sequence: u64,
    pub(crate) recorded_at_unix_seconds: u64,
    pub(crate) event: SoakLedgerEvent,
}

impl SoakLedgerEventEnvelope {
    pub(crate) fn new(
        run_id: SoakRunId,
        sequence: u64,
        recorded_at_unix_seconds: u64,
        event: SoakLedgerEvent,
    ) -> Self {
        Self {
            schema_version: SOAK_LEDGER_SCHEMA_VERSION,
            run_id,
            sequence,
            recorded_at_unix_seconds,
            event,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub(crate) enum SoakLedgerEvent {
    Started { bounds: SoakBounds },
    Checkpoint { status: SoakCheckpointStatus },
    Resume { interrupted_prior_run: bool },
    Stop { outcome: SoakOutcomeLabel },
    Verdict { outcome: SoakOutcomeLabel },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct SoakCheckpointStatus {
    pub(crate) maybe_network: Option<String>,
    pub(crate) maybe_lifecycle: Option<String>,
    pub(crate) maybe_latest_stop_reason_label: Option<String>,
    pub(crate) maybe_recovery_category_label: Option<String>,
    pub(crate) maybe_no_progress_diagnosis_label: Option<String>,
    pub(crate) maybe_validated_active_chain_height: Option<u64>,
    pub(crate) maybe_best_known_tip_height: Option<u64>,
    pub(crate) maybe_source_status_path: Option<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SoakLedger {
    run_id: SoakRunId,
    events_path: PathBuf,
    next_sequence: u64,
}

impl SoakLedger {
    pub(crate) fn create(layout: &SoakLedgerLayout, run_id: SoakRunId) -> Self {
        let paths = layout.paths_for_run(&run_id);
        Self {
            run_id,
            events_path: paths.events_path,
            next_sequence: 1,
        }
    }

    pub(crate) fn resume(layout: &SoakLedgerLayout, run_id: SoakRunId, next_sequence: u64) -> Self {
        let paths = layout.paths_for_run(&run_id);
        Self {
            run_id,
            events_path: paths.events_path,
            next_sequence: next_sequence.max(1),
        }
    }

    pub(crate) fn append_event(
        &mut self,
        recorded_at_unix_seconds: u64,
        event: SoakLedgerEvent,
    ) -> Result<SoakLedgerEventEnvelope, SoakLedgerError> {
        let envelope = SoakLedgerEventEnvelope::new(
            self.run_id.clone(),
            self.next_sequence,
            recorded_at_unix_seconds,
            event,
        );
        let mut encoded = serde_json::to_vec(&envelope).map_err(SoakLedgerError::Encode)?;
        encoded.push(b'\n');
        if encoded.len() > MAX_SOAK_EVENT_BYTES {
            return Err(SoakLedgerError::EventTooLarge {
                bytes: encoded.len(),
                max: MAX_SOAK_EVENT_BYTES,
            });
        }

        let Some(parent) = self.events_path.parent() else {
            return Err(SoakLedgerError::MissingParent {
                path: self.events_path.clone(),
                action: "append ledger event",
            });
        };
        fs::create_dir_all(parent).map_err(|source| SoakLedgerError::Io {
            path: parent.to_path_buf(),
            action: "create soak run directory",
            source,
        })?;
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.events_path)
            .map_err(|source| SoakLedgerError::Io {
                path: self.events_path.clone(),
                action: "open soak ledger",
                source,
            })?;
        file.write_all(&encoded)
            .and_then(|()| file.flush())
            .and_then(|()| file.sync_all())
            .map_err(|source| SoakLedgerError::Io {
                path: self.events_path.clone(),
                action: "append soak ledger event",
                source,
            })?;
        self.next_sequence += 1;
        Ok(envelope)
    }

    pub(crate) fn read_events(path: &Path) -> Result<SoakLedgerReadResult, SoakLedgerError> {
        let bytes = fs::read(path).map_err(|source| SoakLedgerError::Io {
            path: path.to_path_buf(),
            action: "read soak ledger",
            source,
        })?;
        let ignored_trailing_bytes = trailing_partial_line_bytes(&bytes);
        let complete_len = bytes.len().saturating_sub(ignored_trailing_bytes);
        let complete = &bytes[..complete_len];
        let mut events = Vec::new();
        for (index, line) in complete.split(|byte| *byte == b'\n').enumerate() {
            if line.is_empty() {
                continue;
            }
            let envelope = serde_json::from_slice(line).map_err(|source| {
                SoakLedgerError::MalformedCompleteLine {
                    path: path.to_path_buf(),
                    line_number: index + 1,
                    source,
                }
            })?;
            events.push(envelope);
        }

        Ok(SoakLedgerReadResult {
            events,
            ignored_trailing_bytes,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SoakLedgerReadResult {
    pub(crate) events: Vec<SoakLedgerEventEnvelope>,
    pub(crate) ignored_trailing_bytes: usize,
}

fn trailing_partial_line_bytes(bytes: &[u8]) -> usize {
    if bytes.is_empty() || bytes.ends_with(b"\n") {
        return 0;
    }
    bytes
        .iter()
        .rposition(|byte| *byte == b'\n')
        .map_or(bytes.len(), |index| bytes.len() - index - 1)
}

fn sync_parent_dir(path: &Path) -> Result<(), SoakLedgerError> {
    let Some(parent) = path.parent() else {
        return Ok(());
    };
    File::open(parent)
        .and_then(|file| file.sync_all())
        .map_err(|source| SoakLedgerError::Io {
            path: parent.to_path_buf(),
            action: "sync parent directory",
            source,
        })
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum SoakLedgerError {
    #[error("missing parent while attempting to {action}: {path:?}")]
    MissingParent { path: PathBuf, action: &'static str },

    #[error("I/O error while attempting to {action} at {path:?}: {source}")]
    Io {
        path: PathBuf,
        action: &'static str,
        #[source]
        source: std::io::Error,
    },

    #[error("could not encode soak ledger JSON: {0}")]
    Encode(#[source] serde_json::Error),

    #[error("malformed complete soak ledger line {line_number} at {path:?}: {source}")]
    MalformedCompleteLine {
        path: PathBuf,
        line_number: usize,
        #[source]
        source: serde_json::Error,
    },

    #[error("soak ledger event is {bytes} bytes, exceeding max {max}")]
    EventTooLarge { bytes: usize, max: usize },
}
