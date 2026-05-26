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
        SyncResourcePressure,
    },
};

use super::{
    PeerFailureReason, PeerSyncState, SyncNetwork, SyncRunSummary,
    projection::{
        health_signal_log_record, peer_outcome_log_records, peer_telemetry, progress_ratio,
        sync_phase_name,
    },
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
            peer_outcomes: Vec::new(),
            health_signals: Vec::new(),
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
            last_error: FieldAvailability::unavailable("no sync error recorded"),
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

    pub(crate) fn latest_error_message(&self) -> Option<String> {
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
