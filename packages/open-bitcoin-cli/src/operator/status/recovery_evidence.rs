// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Probe-only recovery evidence collection for operator status.

use open_bitcoin_node::{
    RecoveryClassifierInput, RecoveryEvidenceSnapshot, classify_recovery,
    status::{FieldAvailability, ServiceStatus},
    storage::probe_fjall_lock,
};

use super::StatusCollectorInput;

const STATUS_RECOVERY_UNAVAILABLE_REASON: &str =
    "recovery evidence unavailable: no storage, lock, service, or RPC signal";

pub(super) fn collect_status_recovery_evidence(
    input: &StatusCollectorInput,
    service: &ServiceStatus,
    live_rpc_available: FieldAvailability<bool>,
) -> FieldAvailability<RecoveryEvidenceSnapshot> {
    let lock_evidence = input
        .config_resolution
        .maybe_data_dir
        .as_deref()
        .map(probe_fjall_lock)
        .unwrap_or_else(|| {
            FieldAvailability::unavailable("lock probe unavailable: datadir unavailable")
        });

    classify_recovery(RecoveryClassifierInput {
        maybe_storage_error: None,
        maybe_recovery_marker: None,
        lock_evidence,
        service_same_datadir: service_same_datadir(service),
        live_rpc_available,
        resource_bounds: FieldAvailability::unavailable(
            "resource bounds unavailable for status recovery evidence",
        ),
        unavailable_reason: STATUS_RECOVERY_UNAVAILABLE_REASON.to_string(),
    })
}

fn service_same_datadir(service: &ServiceStatus) -> FieldAvailability<bool> {
    match &service.restart_resume {
        FieldAvailability::Available(restart_resume) => restart_resume.same_datadir.clone(),
        FieldAvailability::Unavailable { reason } => FieldAvailability::unavailable(reason.clone()),
    }
}
