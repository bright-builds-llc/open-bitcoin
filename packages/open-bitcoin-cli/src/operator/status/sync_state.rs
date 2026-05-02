// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use open_bitcoin_node::{
    DurableSyncState, FjallNodeStore,
    status::{ChainTipStatus, FieldAvailability, SyncProgress, SyncStatus},
};
use open_bitcoin_rpc::method::GetBlockchainInfoResponse;

use crate::operator::config::OperatorConfigResolution;

pub(super) fn rpc_sync_status(blockchain_info: &GetBlockchainInfoResponse) -> SyncStatus {
    SyncStatus {
        network: FieldAvailability::available(blockchain_info.chain.clone()),
        chain_tip: FieldAvailability::available(ChainTipStatus {
            height: u64::from(blockchain_info.blocks),
            block_hash: blockchain_info
                .maybe_best_block_hash
                .clone()
                .unwrap_or_else(|| "unknown".to_string()),
        }),
        sync_progress: FieldAvailability::available(SyncProgress {
            header_height: u64::from(blockchain_info.headers),
            block_height: u64::from(blockchain_info.blocks),
            progress_ratio: blockchain_info.verificationprogress,
            messages_processed: 0,
            headers_received: u64::from(blockchain_info.headers),
            blocks_received: u64::from(blockchain_info.blocks),
        }),
        lifecycle: FieldAvailability::unavailable("daemon sync lifecycle unavailable"),
        phase: FieldAvailability::unavailable("daemon sync phase unavailable"),
        lag: FieldAvailability::unavailable("daemon sync lag unavailable"),
        last_error: FieldAvailability::unavailable("daemon sync error unavailable"),
        recovery_action: FieldAvailability::unavailable(
            "daemon sync recovery guidance unavailable",
        ),
        resource_pressure: FieldAvailability::unavailable(
            "daemon sync resource pressure unavailable",
        ),
    }
}

pub(super) fn unavailable_sync_status(reason: &str) -> SyncStatus {
    SyncStatus {
        network: FieldAvailability::unavailable(reason),
        chain_tip: FieldAvailability::unavailable(reason),
        sync_progress: FieldAvailability::unavailable(reason),
        lifecycle: FieldAvailability::unavailable(reason),
        phase: FieldAvailability::unavailable(reason),
        lag: FieldAvailability::unavailable(reason),
        last_error: FieldAvailability::unavailable(reason),
        recovery_action: FieldAvailability::unavailable(reason),
        resource_pressure: FieldAvailability::unavailable(reason),
    }
}

pub(super) fn durable_sync_state(
    resolution: &OperatorConfigResolution,
) -> Option<DurableSyncState> {
    let data_dir = resolution.maybe_data_dir.as_ref()?;
    let store = FjallNodeStore::open(data_dir).ok()?;
    let metadata = store.load_runtime_metadata().ok()??;
    metadata.maybe_sync_state
}
