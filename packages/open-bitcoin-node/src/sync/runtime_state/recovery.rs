// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use crate::{
    RuntimeMetadata,
    status::{SyncLifecycleState, SyncRecoveryCategory},
};

use super::super::{SyncRunSummary, types::recovery::recovery_category_from_error_detail};

pub(super) fn recovery_category_for_durable_state(
    metadata: &RuntimeMetadata,
    summary: &SyncRunSummary,
    lifecycle: SyncLifecycleState,
    maybe_last_error: Option<&str>,
) -> Option<SyncRecoveryCategory> {
    if let Some(category) = metadata
        .maybe_last_recovery_action
        .map(|action| action.recovery_category())
    {
        return Some(category);
    }
    if let Some(category) = maybe_last_error.and_then(recovery_category_from_error_detail) {
        return Some(category);
    }
    if let Some(category) = summary
        .maybe_stop_reason
        .and_then(|reason| reason.recovery_category())
    {
        return Some(category);
    }
    if let Some(category) = summary.latest_recovery_category() {
        return Some(category);
    }
    if lifecycle == SyncLifecycleState::Stopped && metadata.last_clean_shutdown {
        return Some(SyncRecoveryCategory::CleanShutdown);
    }
    if lifecycle == SyncLifecycleState::Recovering && !metadata.last_clean_shutdown {
        return Some(SyncRecoveryCategory::UncleanShutdown);
    }
    None
}
