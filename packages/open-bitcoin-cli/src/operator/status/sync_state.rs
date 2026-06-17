// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use open_bitcoin_node::status::{
    BestKnownTipStatus, ChainTipStatus, FieldAvailability, StayCurrentStatus, SyncAttemptCounters,
    SyncConfiguredTargets, SyncProgress, SyncProgressSignal, SyncStatus, SyncStopReasonStatus,
};
use open_bitcoin_rpc::method::GetBlockchainInfoResponse;

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
            downloaded_block_height: u64::from(blockchain_info.blocks),
            connected_block_height: u64::from(blockchain_info.blocks),
            validated_active_chain_height: u64::from(blockchain_info.blocks),
            maybe_downloaded_block_hash: blockchain_info.maybe_best_block_hash.clone(),
            maybe_connected_block_hash: blockchain_info.maybe_best_block_hash.clone(),
            maybe_validated_active_chain_hash: blockchain_info.maybe_best_block_hash.clone(),
            maybe_validated_active_chain_work: None,
            progress_ratio: blockchain_info.verificationprogress,
            messages_processed: 0,
            headers_received: u64::from(blockchain_info.headers),
            blocks_received: u64::from(blockchain_info.blocks),
        }),
        lifecycle: FieldAvailability::unavailable("daemon sync lifecycle unavailable"),
        phase: FieldAvailability::unavailable("daemon sync phase unavailable"),
        configured_targets: FieldAvailability::<SyncConfiguredTargets>::unavailable(
            "daemon sync configured targets unavailable",
        ),
        attempt_counters: FieldAvailability::<SyncAttemptCounters>::unavailable(
            "daemon sync attempt counters unavailable",
        ),
        progress_signal: FieldAvailability::available(rpc_progress_signal(blockchain_info)),
        lag: FieldAvailability::unavailable("daemon sync lag unavailable"),
        last_successful_progress_unix_seconds: FieldAvailability::unavailable(
            "daemon sync last successful progress unavailable",
        ),
        progress_credit: FieldAvailability::unavailable("progress credit evidence unavailable"),
        expected_progress_window: FieldAvailability::unavailable(
            "expected progress window unavailable",
        ),
        no_progress_threshold: FieldAvailability::unavailable(
            "no-progress threshold evidence unavailable",
        ),
        last_useful_work: FieldAvailability::unavailable("last useful work unavailable"),
        last_peer_contribution: FieldAvailability::unavailable(
            "last peer contribution unavailable",
        ),
        stall_diagnosis: FieldAvailability::unavailable("stall diagnosis unavailable"),
        latest_stop_reason: FieldAvailability::<SyncStopReasonStatus>::unavailable(
            "daemon sync latest stop reason unavailable",
        ),
        last_error: FieldAvailability::unavailable("daemon sync error unavailable"),
        recovery_category: FieldAvailability::unavailable("no recovery category recorded"),
        recovery_action: FieldAvailability::unavailable(
            "daemon sync recovery guidance unavailable",
        ),
        resource_pressure: FieldAvailability::unavailable(
            "daemon sync resource pressure unavailable",
        ),
        best_known_tip: FieldAvailability::<BestKnownTipStatus>::unavailable(
            "daemon sync best-known tip evidence unavailable",
        ),
        stay_current: FieldAvailability::<StayCurrentStatus>::unavailable(
            "daemon sync stay-current state unavailable",
        ),
        stay_current_next_action: FieldAvailability::unavailable(
            "daemon sync stay-current next action unavailable",
        ),
        no_progress_diagnosis: FieldAvailability::unavailable("no-progress diagnosis unavailable"),
        no_progress_next_action: FieldAvailability::unavailable(
            "no-progress next action unavailable",
        ),
        latest_reorg: FieldAvailability::unavailable("no reorg evidence recorded"),
        reconcile_progress: FieldAvailability::unavailable("reconcile progress unavailable"),
    }
}

pub(super) fn unavailable_sync_status(reason: &str) -> SyncStatus {
    SyncStatus {
        network: FieldAvailability::unavailable(reason),
        chain_tip: FieldAvailability::unavailable(reason),
        sync_progress: FieldAvailability::unavailable(reason),
        lifecycle: FieldAvailability::unavailable(reason),
        phase: FieldAvailability::unavailable(reason),
        configured_targets: FieldAvailability::unavailable(reason),
        attempt_counters: FieldAvailability::unavailable(reason),
        progress_signal: FieldAvailability::unavailable(reason),
        lag: FieldAvailability::unavailable(reason),
        last_successful_progress_unix_seconds: FieldAvailability::unavailable(reason),
        progress_credit: FieldAvailability::unavailable("progress credit evidence unavailable"),
        expected_progress_window: FieldAvailability::unavailable(
            "expected progress window unavailable",
        ),
        no_progress_threshold: FieldAvailability::unavailable(
            "no-progress threshold evidence unavailable",
        ),
        last_useful_work: FieldAvailability::unavailable("last useful work unavailable"),
        last_peer_contribution: FieldAvailability::unavailable(
            "last peer contribution unavailable",
        ),
        stall_diagnosis: FieldAvailability::unavailable("stall diagnosis unavailable"),
        latest_stop_reason: FieldAvailability::unavailable(reason),
        last_error: FieldAvailability::unavailable(reason),
        recovery_category: FieldAvailability::unavailable("no recovery category recorded"),
        recovery_action: FieldAvailability::unavailable(reason),
        resource_pressure: FieldAvailability::unavailable(reason),
        best_known_tip: FieldAvailability::<BestKnownTipStatus>::unavailable(reason),
        stay_current: FieldAvailability::<StayCurrentStatus>::unavailable(reason),
        stay_current_next_action: FieldAvailability::unavailable(reason),
        no_progress_diagnosis: FieldAvailability::unavailable("no-progress diagnosis unavailable"),
        no_progress_next_action: FieldAvailability::unavailable(
            "no-progress next action unavailable",
        ),
        latest_reorg: FieldAvailability::unavailable("no reorg evidence recorded"),
        reconcile_progress: FieldAvailability::unavailable("reconcile progress unavailable"),
    }
}

fn rpc_progress_signal(blockchain_info: &GetBlockchainInfoResponse) -> SyncProgressSignal {
    if blockchain_info.headers > blockchain_info.blocks {
        return SyncProgressSignal::AwaitingBlocks;
    }
    SyncProgressSignal::Steady
}
