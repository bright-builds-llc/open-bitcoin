// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Multi-day soak run contracts.

use std::{
    fmt,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

pub(crate) mod ledger;
pub(crate) mod outcome;
pub(crate) mod report;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub(crate) struct SoakRunId(String);

impl SoakRunId {
    pub(crate) fn try_new(value: impl Into<String>) -> Result<Self, SoakContractError> {
        let value = value.into();
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(SoakContractError::EmptyRunId);
        }
        if !trimmed
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || matches!(character, '-' | '_'))
        {
            return Err(SoakContractError::InvalidRunId {
                value,
                reason: "run id may only contain ASCII letters, digits, '-' and '_'".to_string(),
            });
        }

        Ok(Self(trimmed.to_string()))
    }

    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for SoakRunId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum SoakPeerPolicy {
    #[serde(rename = "daemon_configured")]
    DaemonConfigured,
    #[serde(rename = "manual_peers_only")]
    ManualPeersOnly,
    #[serde(rename = "no_dns_seeds")]
    NoDnsSeeds,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum SoakStopCondition {
    #[serde(rename = "elapsed_time")]
    ElapsedTime,
    #[serde(rename = "target_height")]
    TargetHeight,
    #[serde(rename = "status_verdict")]
    StatusVerdict,
    #[serde(rename = "operator_stop")]
    OperatorStop,
    #[serde(rename = "resource_stop")]
    ResourceStop,
    #[serde(rename = "recovery_stop")]
    RecoveryStop,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct SoakBounds {
    pub(crate) elapsed_time_seconds: u64,
    pub(crate) checkpoint_interval_seconds: u64,
    pub(crate) maybe_target_height: Option<u64>,
    pub(crate) datadir: PathBuf,
    pub(crate) network: String,
    pub(crate) peer_policy: SoakPeerPolicy,
    pub(crate) disk_budget_bytes: u64,
    pub(crate) stop_conditions: Vec<SoakStopCondition>,
}

impl SoakBounds {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn try_new(
        elapsed_time_seconds: u64,
        checkpoint_interval_seconds: u64,
        maybe_target_height: Option<u64>,
        datadir: PathBuf,
        network: impl Into<String>,
        peer_policy: SoakPeerPolicy,
        disk_budget_bytes: u64,
        stop_conditions: Vec<SoakStopCondition>,
    ) -> Result<Self, SoakContractError> {
        if elapsed_time_seconds == 0 {
            return Err(SoakContractError::ElapsedTimeRequired);
        }
        if checkpoint_interval_seconds == 0 {
            return Err(SoakContractError::CheckpointIntervalRequired);
        }
        if path_is_empty(&datadir) {
            return Err(SoakContractError::DatadirRequired);
        }

        let network = network.into();
        if network.trim().is_empty() {
            return Err(SoakContractError::NetworkRequired);
        }
        if disk_budget_bytes == 0 {
            return Err(SoakContractError::DiskBudgetRequired);
        }
        if stop_conditions.is_empty() {
            return Err(SoakContractError::StopConditionsRequired);
        }

        Ok(Self {
            elapsed_time_seconds,
            checkpoint_interval_seconds,
            maybe_target_height,
            datadir,
            network: network.trim().to_string(),
            peer_policy,
            disk_budget_bytes,
            stop_conditions,
        })
    }
}

fn path_is_empty(path: &Path) -> bool {
    path.as_os_str().is_empty()
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub(crate) enum SoakContractError {
    #[error("soak run id must not be empty")]
    EmptyRunId,

    #[error("invalid soak run id {value}: {reason}")]
    InvalidRunId { value: String, reason: String },

    #[error("elapsed time seconds must be greater than zero")]
    ElapsedTimeRequired,

    #[error("checkpoint interval seconds must be greater than zero")]
    CheckpointIntervalRequired,

    #[error("soak datadir must not be empty")]
    DatadirRequired,

    #[error("soak network must not be empty")]
    NetworkRequired,

    #[error("disk budget bytes must be greater than zero")]
    DiskBudgetRequired,

    #[error("at least one soak stop condition is required")]
    StopConditionsRequired,
}

#[cfg(test)]
mod runtime_tests {
    use std::{
        fs,
        path::{Path, PathBuf},
        time::{SystemTime, UNIX_EPOCH},
    };

    use open_bitcoin_node::{
        LogRetentionPolicy, LogStatus, MetricRetentionPolicy, MetricsStatus,
        status::{
            BestKnownTipSource, BestKnownTipStatus, BuildProvenance, ConfigStatus,
            FieldAvailability, MempoolStatus, NodeRuntimeState, NodeStatus,
            OpenBitcoinStatusSnapshot, PeerStatus, StayCurrentStatus, SyncProgress,
            SyncRecoveryCategory, SyncStatus, TipFreshnessStatus, WalletStatus,
        },
    };

    use super::{
        SoakBounds, SoakPeerPolicy, SoakRunId, SoakStopCondition,
        ledger::{
            SoakCheckpointStatus, SoakLedger, SoakLedgerEvent, SoakLedgerLayout, SoakRunIndex,
            SoakRunIndexEntry,
        },
        outcome::SoakOutcomeLabel,
        report::write_soak_reports,
        run_bounded_soak_loop, validate_resume_plan, write_operator_stop,
        write_report_projection, SoakLoopMode, SoakStatusCollector, SoakTestClock,
    };

    #[derive(Debug)]
    struct TestDirectory {
        path: PathBuf,
    }

    impl TestDirectory {
        fn new(label: &str) -> Self {
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("time after epoch")
                .as_nanos();
            let path = std::env::temp_dir().join(format!(
                "open-bitcoin-soak-runtime-{label}-{}-{timestamp}",
                std::process::id()
            ));
            fs::create_dir_all(&path).expect("test directory");
            Self { path }
        }

        fn path(&self) -> &Path {
            &self.path
        }
    }

    impl Drop for TestDirectory {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    #[derive(Debug)]
    struct ScriptedStatusCollector {
        snapshots: Vec<OpenBitcoinStatusSnapshot>,
        index: usize,
    }

    impl ScriptedStatusCollector {
        fn repeating(snapshot: OpenBitcoinStatusSnapshot) -> Self {
            Self {
                snapshots: vec![snapshot],
                index: 0,
            }
        }

        fn from_snapshots(snapshots: Vec<OpenBitcoinStatusSnapshot>) -> Self {
            Self {
                snapshots,
                index: 0,
            }
        }
    }

    impl SoakStatusCollector for ScriptedStatusCollector {
        fn collect(&mut self) -> OpenBitcoinStatusSnapshot {
            let snapshot = self
                .snapshots
                .get(self.index)
                .or_else(|| self.snapshots.last())
                .expect("scripted status snapshot")
                .clone();
            self.index += 1;
            snapshot
        }
    }

    #[test]
    fn soak_runtime_elapsed_writes_started_checkpoints_stop_and_verdict() {
        // Arrange
        let temp = TestDirectory::new("elapsed");
        let layout = SoakLedgerLayout::for_datadir(temp.path());
        let run_id = SoakRunId::try_new("soak-1700000000-0001").expect("run id");
        let bounds = soak_bounds(
            temp.path(),
            Some(144),
            vec![SoakStopCondition::ElapsedTime],
        );
        let mut ledger = SoakLedger::create(&layout, run_id.clone());
        let mut collector = ScriptedStatusCollector::repeating(clean_status_snapshot(temp.path(), 144));
        let mut clock = SoakTestClock::new(1_700_000_000);

        // Act
        let result = run_bounded_soak_loop(
            &run_id,
            &bounds,
            &layout,
            &mut ledger,
            &mut collector,
            &mut clock,
            SoakLoopMode::Start,
        )
        .expect("bounded soak loop");
        let events = SoakLedger::read_events(&layout.paths_for_run(&run_id).events_path)
            .expect("read events")
            .events;

        // Assert
        assert_eq!(result.final_outcome, SoakOutcomeLabel::CleanCompletion);
        assert_eq!(result.latest_sequence, 8);
        assert!(result.report_paths.json_path.exists());
        assert!(result.report_paths.markdown_path.exists());
        assert!(matches!(events[0].event, SoakLedgerEvent::Started { .. }));
        assert_eq!(
            events
                .iter()
                .filter(|event| matches!(event.event, SoakLedgerEvent::Checkpoint { .. }))
                .count(),
            5
        );
        assert!(matches!(
            events[6].event,
            SoakLedgerEvent::Stop {
                outcome: SoakOutcomeLabel::CleanCompletion
            }
        ));
        assert!(matches!(
            events[7].event,
            SoakLedgerEvent::Verdict {
                outcome: SoakOutcomeLabel::CleanCompletion
            }
        ));
    }

    #[test]
    fn soak_runtime_target_height_resource_recovery_and_status_verdict_stop_conditions() {
        // Arrange
        let temp = TestDirectory::new("stop-conditions");
        let cases = [
            (
                "target",
                vec![SoakStopCondition::TargetHeight],
                clean_status_snapshot(temp.path(), 144),
                Some(144),
                SoakOutcomeLabel::CleanCompletion,
            ),
            (
                "resource",
                vec![SoakStopCondition::ResourceStop],
                resource_status_snapshot(temp.path()),
                None,
                SoakOutcomeLabel::ResourceStop,
            ),
            (
                "recovery",
                vec![SoakStopCondition::RecoveryStop],
                recovery_status_snapshot(temp.path()),
                None,
                SoakOutcomeLabel::RecoveryStop,
            ),
            (
                "verdict",
                vec![SoakStopCondition::StatusVerdict],
                diagnosed_status_snapshot(temp.path()),
                None,
                SoakOutcomeLabel::DiagnosedBlocker,
            ),
        ];

        // Act / Assert
        for (label, stop_conditions, snapshot, maybe_target_height, expected) in cases {
            let run_id = SoakRunId::try_new(format!("soak-1700000000-{label}")).expect("run id");
            let layout = SoakLedgerLayout::for_datadir(temp.path());
            let mut ledger = SoakLedger::create(&layout, run_id.clone());
            let bounds = soak_bounds(temp.path(), maybe_target_height, stop_conditions);
            let mut collector = ScriptedStatusCollector::from_snapshots(vec![snapshot]);
            let mut clock = SoakTestClock::new(1_700_000_000);

            let result = run_bounded_soak_loop(
                &run_id,
                &bounds,
                &layout,
                &mut ledger,
                &mut collector,
                &mut clock,
                SoakLoopMode::Start,
            )
            .expect("bounded stop condition loop");

            assert_eq!(result.final_outcome, expected, "case {label}");
        }
    }

    #[test]
    fn soak_runtime_resume_refuses_clean_completion_and_flags_interrupted_runs() {
        // Arrange
        let temp = TestDirectory::new("resume");
        let layout = SoakLedgerLayout::for_datadir(temp.path());
        let clean_run_id = SoakRunId::try_new("soak-1700000000-clean").expect("run id");
        let interrupted_run_id =
            SoakRunId::try_new("soak-1700000000-interrupted").expect("run id");
        let clean_paths = layout.paths_for_run(&clean_run_id);
        let interrupted_paths = layout.paths_for_run(&interrupted_run_id);
        let mut clean_ledger = SoakLedger::create(&layout, clean_run_id.clone());
        clean_ledger
            .append_event(
                1,
                SoakLedgerEvent::Started {
                    bounds: soak_bounds(temp.path(), None, vec![SoakStopCondition::ElapsedTime]),
                },
            )
            .expect("clean started");
        clean_ledger
            .append_event(
                2,
                SoakLedgerEvent::Stop {
                    outcome: SoakOutcomeLabel::CleanCompletion,
                },
            )
            .expect("clean stop");
        clean_ledger
            .append_event(
                3,
                SoakLedgerEvent::Verdict {
                    outcome: SoakOutcomeLabel::CleanCompletion,
                },
            )
            .expect("clean verdict");
        let mut interrupted_ledger = SoakLedger::create(&layout, interrupted_run_id.clone());
        interrupted_ledger
            .append_event(
                1,
                SoakLedgerEvent::Started {
                    bounds: soak_bounds(temp.path(), None, vec![SoakStopCondition::ElapsedTime]),
                },
            )
            .expect("interrupted started");
        let mut index = SoakRunIndex::empty();
        index.record_run(SoakRunIndexEntry {
            run_id: clean_run_id.clone(),
            ledger_path: clean_paths.events_path.clone(),
            started_at_unix_seconds: 1,
            updated_at_unix_seconds: 3,
            maybe_outcome: Some(SoakOutcomeLabel::CleanCompletion),
        });
        index.record_run(SoakRunIndexEntry {
            run_id: interrupted_run_id.clone(),
            ledger_path: interrupted_paths.events_path.clone(),
            started_at_unix_seconds: 1,
            updated_at_unix_seconds: 1,
            maybe_outcome: None,
        });
        index.write_atomic(&layout).expect("write index");

        // Act
        let clean_result = validate_resume_plan(&layout, &clean_run_id, 15);
        let interrupted = validate_resume_plan(&layout, &interrupted_run_id, 15)
            .expect("interrupted resume plan");

        // Assert
        assert!(clean_result.is_err());
        assert!(interrupted.interrupted_prior_run);
        assert_eq!(interrupted.next_sequence, 2);
    }

    #[test]
    fn soak_runtime_stop_records_operator_stop_verdict() {
        // Arrange
        let temp = TestDirectory::new("stop");
        let layout = SoakLedgerLayout::for_datadir(temp.path());
        let run_id = SoakRunId::try_new("soak-1700000000-stop").expect("run id");
        let mut ledger = SoakLedger::create(&layout, run_id.clone());
        ledger
            .append_event(
                1,
                SoakLedgerEvent::Started {
                    bounds: soak_bounds(temp.path(), None, vec![SoakStopCondition::ElapsedTime]),
                },
            )
            .expect("started");

        // Act
        let result = write_operator_stop(&layout, &run_id, 2).expect("operator stop");
        let events = SoakLedger::read_events(&layout.paths_for_run(&run_id).events_path)
            .expect("read events")
            .events;

        // Assert
        assert_eq!(result.final_outcome, SoakOutcomeLabel::OperatorStop);
        assert!(matches!(
            events[1].event,
            SoakLedgerEvent::Stop {
                outcome: SoakOutcomeLabel::OperatorStop
            }
        ));
        assert!(matches!(
            events[2].event,
            SoakLedgerEvent::Verdict {
                outcome: SoakOutcomeLabel::OperatorStop
            }
        ));
    }

    #[test]
    fn soak_runtime_report_rewrites_projection_without_ledger_append() {
        // Arrange
        let temp = TestDirectory::new("report");
        let layout = SoakLedgerLayout::for_datadir(temp.path());
        let run_id = SoakRunId::try_new("soak-1700000000-report").expect("run id");
        let mut ledger = SoakLedger::create(&layout, run_id.clone());
        ledger
            .append_event(
                1,
                SoakLedgerEvent::Started {
                    bounds: soak_bounds(temp.path(), None, vec![SoakStopCondition::ElapsedTime]),
                },
            )
            .expect("started");
        ledger
            .append_event(
                2,
                SoakLedgerEvent::Checkpoint {
                    status: checkpoint_status(144),
                },
            )
            .expect("checkpoint");
        let events_path = layout.paths_for_run(&run_id).events_path;
        let before_lines = fs::read_to_string(&events_path)
            .expect("ledger before")
            .lines()
            .count();

        // Act
        let result = write_report_projection(&layout, &run_id).expect("report projection");
        let after_lines = fs::read_to_string(&events_path)
            .expect("ledger after")
            .lines()
            .count();

        // Assert
        assert_eq!(before_lines, after_lines);
        assert_eq!(result.latest_sequence, 2);
        assert!(result.report_paths.json_path.exists());
    }

    fn soak_bounds(
        datadir: &Path,
        maybe_target_height: Option<u64>,
        stop_conditions: Vec<SoakStopCondition>,
    ) -> SoakBounds {
        SoakBounds::try_new(
            60,
            15,
            maybe_target_height,
            datadir.to_path_buf(),
            "regtest",
            SoakPeerPolicy::DaemonConfigured,
            1_048_576,
            stop_conditions,
        )
        .expect("soak bounds")
    }

    fn checkpoint_status(height: u64) -> SoakCheckpointStatus {
        SoakCheckpointStatus {
            maybe_network: Some("regtest".to_string()),
            maybe_lifecycle: Some("active".to_string()),
            maybe_latest_stop_reason_label: None,
            maybe_recovery_category_label: None,
            maybe_no_progress_diagnosis_label: None,
            maybe_validated_active_chain_height: Some(height),
            maybe_best_known_tip_height: Some(height),
            maybe_source_status_path: None,
        }
    }

    fn clean_status_snapshot(datadir: &Path, height: u64) -> OpenBitcoinStatusSnapshot {
        let mut snapshot = base_status_snapshot(datadir);
        snapshot.sync.sync_progress = FieldAvailability::available(SyncProgress {
            header_height: height,
            block_height: height,
            downloaded_block_height: height,
            connected_block_height: height,
            validated_active_chain_height: height,
            maybe_downloaded_block_hash: Some("11".repeat(32)),
            maybe_connected_block_hash: Some("11".repeat(32)),
            maybe_validated_active_chain_hash: Some("11".repeat(32)),
            maybe_validated_active_chain_work: Some("work".to_string()),
            progress_ratio: 1.0,
            messages_processed: 1,
            headers_received: 1,
            blocks_received: 1,
        });
        snapshot.sync.best_known_tip = FieldAvailability::available(BestKnownTipStatus {
            source: BestKnownTipSource::HeaderStore,
            height,
            block_hash: "11".repeat(32),
            work: "work".to_string(),
            block_time_unix_seconds: 1_700_000_000,
            observed_at_unix_seconds: 1_700_000_000,
            freshness: TipFreshnessStatus::Fresh,
            peer_agreement: Vec::new(),
        });
        snapshot.sync.stay_current =
            FieldAvailability::available(StayCurrentStatus::CurrentAtBestKnownTip);
        snapshot
    }

    fn resource_status_snapshot(datadir: &Path) -> OpenBitcoinStatusSnapshot {
        let mut snapshot = base_status_snapshot(datadir);
        snapshot.sync.recovery_category =
            FieldAvailability::available(SyncRecoveryCategory::ResourceExhaustion);
        snapshot
    }

    fn recovery_status_snapshot(datadir: &Path) -> OpenBitcoinStatusSnapshot {
        let mut snapshot = base_status_snapshot(datadir);
        snapshot.sync.recovery_category =
            FieldAvailability::available(SyncRecoveryCategory::StoreCorruption);
        snapshot
    }

    fn diagnosed_status_snapshot(datadir: &Path) -> OpenBitcoinStatusSnapshot {
        let mut snapshot = base_status_snapshot(datadir);
        snapshot.sync.no_progress_diagnosis = FieldAvailability::available(
            open_bitcoin_node::status::NoProgressDiagnosis::StorageOrResourceBlocked,
        );
        snapshot
    }

    fn base_status_snapshot(datadir: &Path) -> OpenBitcoinStatusSnapshot {
        OpenBitcoinStatusSnapshot {
            node: NodeStatus {
                state: NodeRuntimeState::Running,
                version: "test".to_string(),
            },
            config: ConfigStatus {
                datadir: FieldAvailability::available(datadir.display().to_string()),
                config_paths: Vec::new(),
            },
            service: open_bitcoin_node::status::ServiceStatus {
                manager: FieldAvailability::unavailable("no service manager"),
                lifecycle: FieldAvailability::unavailable("service lifecycle unavailable"),
                installed: FieldAvailability::unavailable("service installed unavailable"),
                enabled: FieldAvailability::unavailable("service enabled unavailable"),
                running: FieldAvailability::unavailable("service running unavailable"),
                service_file_path: FieldAvailability::unavailable("service file unavailable"),
                log_path: FieldAvailability::unavailable("service log unavailable"),
                diagnostics: FieldAvailability::unavailable("service diagnostics unavailable"),
                restart_resume: FieldAvailability::unavailable(
                    "service restart/resume unavailable",
                ),
            },
            sync: SyncStatus {
                network: FieldAvailability::available("regtest".to_string()),
                chain_tip: FieldAvailability::unavailable("chain tip unavailable"),
                sync_progress: FieldAvailability::unavailable("sync progress unavailable"),
                lifecycle: FieldAvailability::available(
                    open_bitcoin_node::status::SyncLifecycleState::Active,
                ),
                phase: FieldAvailability::available("test".to_string()),
                configured_targets: FieldAvailability::unavailable("targets unavailable"),
                attempt_counters: FieldAvailability::unavailable("attempts unavailable"),
                progress_signal: FieldAvailability::unavailable("signal unavailable"),
                lag: FieldAvailability::unavailable("lag unavailable"),
                last_successful_progress_unix_seconds: FieldAvailability::unavailable(
                    "progress unavailable",
                ),
                latest_stop_reason: FieldAvailability::unavailable("stop reason unavailable"),
                last_error: FieldAvailability::unavailable("last error unavailable"),
                recovery_category: FieldAvailability::unavailable("recovery unavailable"),
                recovery_action: FieldAvailability::unavailable("recovery action unavailable"),
                resource_pressure: FieldAvailability::unavailable("pressure unavailable"),
                best_known_tip: FieldAvailability::unavailable("best tip unavailable"),
                stay_current: FieldAvailability::unavailable("stay current unavailable"),
                stay_current_next_action: FieldAvailability::unavailable(
                    "stay current action unavailable",
                ),
                no_progress_diagnosis: FieldAvailability::unavailable(
                    "no-progress diagnosis unavailable",
                ),
                no_progress_next_action: FieldAvailability::unavailable(
                    "no-progress action unavailable",
                ),
                latest_reorg: FieldAvailability::unavailable("reorg unavailable"),
                reconcile_progress: FieldAvailability::unavailable("reconcile unavailable"),
            },
            peers: PeerStatus {
                peer_counts: FieldAvailability::unavailable("peer counts unavailable"),
                recent_peers: FieldAvailability::unavailable("recent peers unavailable"),
            },
            mempool: MempoolStatus {
                transactions: FieldAvailability::unavailable("mempool unavailable"),
            },
            wallet: WalletStatus {
                trusted_balance_sats: FieldAvailability::unavailable("wallet unavailable"),
                freshness: FieldAvailability::unavailable("wallet freshness unavailable"),
                scan_progress: FieldAvailability::unavailable("wallet scan unavailable"),
            },
            logs: LogStatus::default(),
            metrics: MetricsStatus::unavailable(
                MetricRetentionPolicy::default(),
                "metrics unavailable",
            ),
            health_signals: Vec::new(),
            build: BuildProvenance::unavailable(),
        }
    }
}
