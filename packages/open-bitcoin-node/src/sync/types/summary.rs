// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp

use open_bitcoin_network::MAX_HEADERS_RESULTS;

use crate::{
    FieldAvailability, MetricKind, MetricSample, PeerStatus, SyncStatus,
    logging::{StructuredLogLevel, StructuredLogRecord},
    status::{
        PeerCounts, SyncLagStatus, SyncLifecycleState, SyncProgress, SyncProgressSignal,
        SyncRecoveryCategory, SyncResourcePressure,
    },
};

use super::{
    PeerFailureReason, PeerSyncState, SyncNetwork, SyncRunSummary, SyncStopReason,
    projection::{
        health_signal_log_record, peer_outcome_log_records, peer_telemetry, progress_ratio,
        sync_phase_name,
    },
    recovery::recovery_category_from_error_detail,
};

const MAX_HEADER_REQUESTS_IN_FLIGHT_PER_PEER: u64 = 1;

impl SyncRunSummary {
    pub(crate) fn empty(
        best_header_height: u64,
        best_block_height: u64,
        target_outbound_peers: usize,
    ) -> Self {
        Self {
            target_outbound_peers,
            attempted_peers: 0,
            connected_peers: 0,
            failed_peers: 0,
            messages_processed: 0,
            headers_received: 0,
            blocks_received: 0,
            best_header_height,
            downloaded_block_height: best_block_height,
            best_block_height,
            maybe_downloaded_block_hash: None,
            maybe_connected_block_hash: None,
            peer_outcomes: Vec::new(),
            health_signals: Vec::new(),
            maybe_stop_reason: None,
        }
    }

    pub fn sync_status(&self, network: SyncNetwork) -> SyncStatus {
        SyncStatus {
            network: FieldAvailability::available(network.as_str().to_string()),
            chain_tip: FieldAvailability::unavailable(
                "chain tip hash is unavailable from sync summary alone",
            ),
            sync_progress: FieldAvailability::available(SyncProgress {
                header_height: self.best_header_height,
                block_height: self.best_block_height,
                downloaded_block_height: self.downloaded_block_height,
                connected_block_height: self.best_block_height,
                maybe_downloaded_block_hash: self.maybe_downloaded_block_hash.clone(),
                maybe_connected_block_hash: self.maybe_connected_block_hash.clone(),
                progress_ratio: progress_ratio(self.best_block_height, self.best_header_height),
                messages_processed: self.messages_processed as u64,
                headers_received: self.headers_received as u64,
                blocks_received: self.blocks_received as u64,
            }),
            lifecycle: FieldAvailability::available(SyncLifecycleState::Active),
            phase: FieldAvailability::available(sync_phase_name(self).to_string()),
            progress_signal: FieldAvailability::available(self.progress_signal()),
            lag: FieldAvailability::available(SyncLagStatus {
                headers_remaining: 0,
                blocks_remaining: self
                    .best_header_height
                    .saturating_sub(self.best_block_height),
            }),
            last_successful_progress_unix_seconds: match self
                .last_successful_progress_unix_seconds()
            {
                Some(value) => FieldAvailability::available(value),
                None => FieldAvailability::unavailable(
                    "no successful sync progress recorded in this run",
                ),
            },
            last_error: match self.latest_error_message() {
                Some(value) => FieldAvailability::available(value),
                None => FieldAvailability::unavailable("no sync error recorded"),
            },
            recovery_category: match self.recovery_category() {
                Some(value) => FieldAvailability::available(value),
                None => FieldAvailability::unavailable("no recovery category recorded"),
            },
            recovery_action: FieldAvailability::unavailable("no recovery action required"),
            resource_pressure: FieldAvailability::available(SyncResourcePressure {
                blocks_in_flight: 0,
                max_header_requests_in_flight_per_peer: MAX_HEADER_REQUESTS_IN_FLIGHT_PER_PEER,
                max_headers_per_message: MAX_HEADERS_RESULTS as u64,
                max_blocks_in_flight_per_peer: 0,
                max_blocks_in_flight_total: 0,
                max_messages_per_peer: 0,
                max_sync_rounds: 0,
                outbound_peers: self.connected_peers as u32,
                target_outbound_peers: self.target_outbound_peers as u32,
            }),
        }
    }

    pub fn latest_error_message(&self) -> Option<String> {
        self.peer_outcomes
            .iter()
            .rev()
            .find_map(|outcome| outcome.maybe_error.clone())
    }

    pub(crate) fn latest_recovery_action(&self) -> Option<&'static str> {
        self.peer_outcomes
            .iter()
            .rev()
            .filter_map(|outcome| outcome.maybe_failure_reason.as_ref())
            .next()
            .map(PeerFailureReason::operator_recovery_action)
    }

    pub(crate) fn recovery_category(&self) -> Option<SyncRecoveryCategory> {
        if let Some(category) = self
            .maybe_stop_reason
            .and_then(SyncStopReason::recovery_category)
        {
            return Some(category);
        }

        let maybe_detail_category = self
            .latest_error_message()
            .and_then(|detail| recovery_category_from_error_detail(&detail));
        if let Some(category) = maybe_detail_category
            && storage_recovery_category(category)
        {
            return Some(category);
        }

        if let Some(category) = self
            .peer_outcomes
            .iter()
            .rev()
            .filter_map(|outcome| outcome.maybe_failure_reason.as_ref())
            .next()
            .map(PeerFailureReason::recovery_category)
        {
            return Some(category);
        }

        maybe_detail_category
    }

    pub(crate) fn progress_signal(&self) -> SyncProgressSignal {
        if self.blocks_received > 0 {
            return SyncProgressSignal::BlockProgress;
        }
        if self.headers_received > 0 {
            return SyncProgressSignal::HeaderProgress;
        }
        if self
            .peer_outcomes
            .iter()
            .any(|outcome| outcome.state == PeerSyncState::Waiting)
        {
            return SyncProgressSignal::WaitingForPeers;
        }
        if self.failed_peers > 0 {
            return SyncProgressSignal::PeerFailures;
        }
        if self.best_block_height < self.best_header_height {
            return SyncProgressSignal::AwaitingBlocks;
        }
        SyncProgressSignal::Steady
    }

    pub(crate) fn last_successful_progress_unix_seconds(&self) -> Option<u64> {
        self.peer_outcomes
            .iter()
            .rev()
            .find(|outcome| {
                outcome.contribution.headers_received > 0
                    || outcome.contribution.blocks_received > 0
            })
            .and_then(|outcome| outcome.maybe_last_activity_unix_seconds)
    }

    pub fn peer_status(&self) -> PeerStatus {
        PeerStatus {
            peer_counts: FieldAvailability::available(PeerCounts {
                inbound: 0,
                outbound: self.connected_peers as u32,
            }),
            recent_peers: FieldAvailability::available(
                self.peer_outcomes
                    .iter()
                    .map(peer_telemetry)
                    .collect::<Vec<_>>(),
            ),
        }
    }

    pub fn metric_samples(&self, timestamp_unix_seconds: u64) -> Vec<MetricSample> {
        vec![
            MetricSample::new(
                MetricKind::HeaderHeight,
                self.best_header_height as f64,
                timestamp_unix_seconds,
            ),
            MetricSample::new(
                MetricKind::DownloadedBlockHeight,
                self.downloaded_block_height as f64,
                timestamp_unix_seconds,
            ),
            MetricSample::new(
                MetricKind::ConnectedBlockHeight,
                self.best_block_height as f64,
                timestamp_unix_seconds,
            ),
            MetricSample::new(
                MetricKind::SyncHeight,
                self.best_block_height as f64,
                timestamp_unix_seconds,
            ),
            MetricSample::new(
                MetricKind::PeerCount,
                self.connected_peers as f64,
                timestamp_unix_seconds,
            ),
        ]
    }

    pub fn structured_log_records(&self, timestamp_unix_seconds: u64) -> Vec<StructuredLogRecord> {
        let mut records = vec![StructuredLogRecord {
            level: StructuredLogLevel::Info,
            source: "sync".to_string(),
            message: format!(
                "sync progress messages_processed={} headers_received={} blocks_received={} header={} downloaded={} connected={} signal={} last_progress={}",
                self.messages_processed,
                self.headers_received,
                self.blocks_received,
                self.best_header_height,
                self.downloaded_block_height,
                self.best_block_height,
                progress_signal_name(self.progress_signal()),
                self.last_successful_progress_unix_seconds()
                    .map_or("unavailable".to_string(), |value| value.to_string())
            ),
            timestamp_unix_seconds,
        }];

        if let Some(stop_reason) = self.maybe_stop_reason {
            records.push(StructuredLogRecord {
                level: StructuredLogLevel::Info,
                source: "sync".to_string(),
                message: format!("sync stop reason={}", stop_reason.label()),
                timestamp_unix_seconds,
            });
        }

        for outcome in &self.peer_outcomes {
            records.extend(peer_outcome_log_records(outcome, timestamp_unix_seconds));
        }

        records.extend(
            self.health_signals
                .iter()
                .map(|signal| health_signal_log_record(signal, timestamp_unix_seconds)),
        );
        records
    }
}

const fn storage_recovery_category(category: SyncRecoveryCategory) -> bool {
    matches!(
        category,
        SyncRecoveryCategory::IncompatibleSchema
            | SyncRecoveryCategory::StoreCorruption
            | SyncRecoveryCategory::StorageLockContention
            | SyncRecoveryCategory::StorageBackendFailure
    )
}

fn progress_signal_name(signal: SyncProgressSignal) -> &'static str {
    match signal {
        SyncProgressSignal::HeaderProgress => "header_progress",
        SyncProgressSignal::BlockProgress => "block_progress",
        SyncProgressSignal::WaitingForPeers => "waiting_for_peers",
        SyncProgressSignal::PeerFailures => "peer_failures",
        SyncProgressSignal::AwaitingBlocks => "awaiting_blocks",
        SyncProgressSignal::Steady => "steady",
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        FieldAvailability, MetricKind, MetricSample, SyncStopReason,
        status::{PeerCounts, SyncProgressSignal, SyncRecoveryCategory},
        sync::{
            PeerContribution, PeerFailureReason, PeerSyncOutcome, PeerSyncState, SyncNetwork,
            SyncPeerAddress, SyncRunSummary,
        },
    };

    #[test]
    fn sync_summary_projects_consistent_operator_evidence_fields() {
        // Arrange
        let latest_error = "peer stalled before block connect";
        let summary = SyncRunSummary {
            target_outbound_peers: 4,
            attempted_peers: 2,
            connected_peers: 2,
            failed_peers: 0,
            messages_processed: 7,
            headers_received: 3,
            blocks_received: 1,
            best_header_height: 840_100,
            downloaded_block_height: 840_006,
            best_block_height: 840_004,
            maybe_downloaded_block_hash: Some("22".repeat(32)),
            maybe_connected_block_hash: Some("11".repeat(32)),
            peer_outcomes: vec![PeerSyncOutcome {
                peer: SyncPeerAddress::manual("seed.bitcoin.sipa.be", 8_333),
                maybe_resolved_endpoint: Some("203.0.113.10:8333".to_string()),
                network: SyncNetwork::Mainnet,
                state: PeerSyncState::Stalled,
                attempts: 1,
                contribution: PeerContribution {
                    messages_processed: 7,
                    headers_received: 3,
                    blocks_received: 1,
                },
                maybe_last_activity_unix_seconds: Some(1_717_000_000),
                maybe_capabilities: None,
                maybe_failure_reason: Some(PeerFailureReason::Stall),
                maybe_error: Some(latest_error.to_string()),
            }],
            health_signals: Vec::new(),
            maybe_stop_reason: None,
        };

        // Act
        let status = summary.sync_status(SyncNetwork::Mainnet);
        let peer_status = summary.peer_status();
        let samples = summary.metric_samples(1_717_000_000);
        let records = summary.structured_log_records(1_717_000_000);

        // Assert
        let FieldAvailability::Available(progress) = status.sync_progress else {
            panic!("sync progress should be available");
        };
        assert_eq!(progress.header_height, 840_100);
        assert_eq!(progress.downloaded_block_height, 840_006);
        assert_eq!(progress.connected_block_height, 840_004);
        assert_eq!(progress.block_height, 840_004);
        assert_eq!(progress.maybe_downloaded_block_hash, Some("22".repeat(32)));
        assert_eq!(progress.maybe_connected_block_hash, Some("11".repeat(32)));
        assert_eq!(
            status.progress_signal,
            FieldAvailability::available(SyncProgressSignal::BlockProgress)
        );
        assert_eq!(
            status.last_error,
            FieldAvailability::available(latest_error.to_string())
        );
        assert_eq!(
            status.recovery_category,
            FieldAvailability::available(SyncRecoveryCategory::PublicNetworkUnreachable)
        );
        assert_eq!(
            peer_status.peer_counts,
            FieldAvailability::available(PeerCounts {
                inbound: 0,
                outbound: 2,
            })
        );
        assert_eq!(
            samples,
            vec![
                MetricSample::new(MetricKind::HeaderHeight, 840_100.0, 1_717_000_000),
                MetricSample::new(MetricKind::DownloadedBlockHeight, 840_006.0, 1_717_000_000,),
                MetricSample::new(MetricKind::ConnectedBlockHeight, 840_004.0, 1_717_000_000,),
                MetricSample::new(MetricKind::SyncHeight, 840_004.0, 1_717_000_000),
                MetricSample::new(MetricKind::PeerCount, 2.0, 1_717_000_000),
            ]
        );
        assert!(records.iter().any(|record| {
            record
                .message
                .contains("header=840100 downloaded=840006 connected=840004 signal=block_progress")
        }));
        assert!(
            records
                .iter()
                .any(|record| record.message.contains("peer stalled"))
        );
    }

    #[test]
    fn stop_reason_projection_includes_operator_pause_and_shutdown_labels() {
        // Arrange
        let mut paused_summary = SyncRunSummary::empty(0, 0, 1);
        paused_summary.maybe_stop_reason = Some(SyncStopReason::OperatorPaused);
        paused_summary
            .health_signals
            .push(SyncStopReason::OperatorPaused.health_signal());
        let mut stopped_summary = SyncRunSummary::empty(0, 0, 1);
        stopped_summary.maybe_stop_reason = Some(SyncStopReason::ShutdownRequested);
        stopped_summary
            .health_signals
            .push(SyncStopReason::ShutdownRequested.health_signal());

        // Act
        let paused_status = paused_summary.sync_status(SyncNetwork::Mainnet);
        let stopped_status = stopped_summary.sync_status(SyncNetwork::Mainnet);
        let paused_records = paused_summary.structured_log_records(1_717_000_001);
        let stopped_records = stopped_summary.structured_log_records(1_717_000_002);

        // Assert
        assert_eq!(
            paused_status.phase,
            FieldAvailability::available("operator_paused".to_string())
        );
        assert_eq!(
            stopped_status.phase,
            FieldAvailability::available("shutdown_requested".to_string())
        );
        assert_eq!(
            paused_status.recovery_category,
            FieldAvailability::available(SyncRecoveryCategory::OperatorCancellation)
        );
        assert_eq!(
            stopped_status.recovery_category,
            FieldAvailability::available(SyncRecoveryCategory::OperatorCancellation)
        );
        assert!(
            paused_records
                .iter()
                .any(|record| { record.message.contains("sync stop reason=operator_paused") })
        );
        assert!(stopped_records.iter().any(|record| {
            record
                .message
                .contains("sync stop reason=shutdown_requested")
        }));
    }
}
