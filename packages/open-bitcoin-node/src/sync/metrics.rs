// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use std::sync::Arc;

use crate::{
    FieldAvailability, InboundPeerServingStatus, MetricRetentionPolicy, block_relay_metric_samples,
    inbound_metric_samples, network::BlockRelayRuntimeEvidenceSnapshot,
};

use super::{DurableSyncRuntime, SyncRunSummary, SyncRuntimeError};

impl DurableSyncRuntime {
    pub fn set_inbound_metric_status_provider<F>(&mut self, provider: F)
    where
        F: Fn() -> FieldAvailability<InboundPeerServingStatus> + Send + Sync + 'static,
    {
        self.maybe_inbound_metric_status_provider = Some(Arc::new(provider));
    }

    pub(super) fn persist_metrics(
        &self,
        summary: &SyncRunSummary,
        maybe_block_relay_snapshot: Option<&BlockRelayRuntimeEvidenceSnapshot>,
        timestamp: i64,
    ) -> Result<(), SyncRuntimeError> {
        let timestamp = u64::try_from(timestamp).unwrap_or(0);
        let summary = self.summary_with_configured_targets(summary);
        let mut samples = summary.metric_samples(timestamp);
        if let Some(provider) = self.maybe_inbound_metric_status_provider.as_ref() {
            samples.extend(inbound_metric_samples(&provider(), timestamp));
        }
        if let Some(snapshot) = maybe_block_relay_snapshot {
            samples.extend(block_relay_metric_samples(
                &snapshot.status,
                snapshot.served_count,
                timestamp,
            ));
        }
        self.store.append_metric_samples(
            &samples,
            MetricRetentionPolicy::default(),
            timestamp,
            self.config.persist_mode,
        )?;

        Ok(())
    }
}
