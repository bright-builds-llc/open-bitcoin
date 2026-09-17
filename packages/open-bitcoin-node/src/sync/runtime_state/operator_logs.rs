// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use crate::{
    logging::{block_relay_log_record, chainstate_durability_log_record},
    network::BlockRelayRuntimeEvidenceSnapshot,
};

use super::super::{DurableSyncRuntime, SyncRunSummary};

impl DurableSyncRuntime {
    pub(in crate::sync) fn write_block_relay_log(
        &self,
        summary: &mut SyncRunSummary,
        maybe_block_relay_snapshot: Option<&BlockRelayRuntimeEvidenceSnapshot>,
        timestamp: i64,
    ) {
        let timestamp = u64::try_from(timestamp).unwrap_or(0);
        if let Ok(snapshot) = self.network.operator_snapshot() {
            let record =
                chainstate_durability_log_record(snapshot.chainstate_durability(), timestamp);
            if let Err(error) = self.append_structured_record(&record) {
                summary
                    .health_signals
                    .push(super::super::progress::log_write_failed_signal(&error));
            }
        }
        let Some(snapshot) = maybe_block_relay_snapshot else {
            return;
        };
        let record = block_relay_log_record(&snapshot.status, snapshot.served_count, timestamp);
        if let Err(error) = self.append_structured_record(&record) {
            summary
                .health_signals
                .push(super::super::progress::log_write_failed_signal(&error));
        }
    }
}
