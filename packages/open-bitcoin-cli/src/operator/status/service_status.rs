// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Service status projection for the shared operator status snapshot.

use open_bitcoin_node::status::{
    FieldAvailability, ServiceLifecycleStatus, ServiceRestartResumeStatus, ServiceStatus,
};

use super::{StatusCollectorInput, StatusDetectionEvidence, detection};
use crate::operator::config::OperatorConfigResolution;
use crate::operator::service::{ServiceLifecycleState, ServiceStateSnapshot};

const PROBE_ONLY_RUNTIME_METADATA_REASON: &str =
    "service restart/resume evidence unavailable: probe-only status does not open Fjall stores";

pub(super) fn service_lifecycle_from_snapshot(
    snapshot: &crate::operator::service::ServiceStateSnapshot,
) -> ServiceLifecycleStatus {
    match (snapshot.state, snapshot.maybe_enabled) {
        (ServiceLifecycleState::Unmanaged, _) => ServiceLifecycleStatus::Unmanaged,
        (ServiceLifecycleState::Running, _) => ServiceLifecycleStatus::Running,
        (ServiceLifecycleState::Failed, _) => ServiceLifecycleStatus::Failed,
        (_, Some(false)) => ServiceLifecycleStatus::Disabled,
        _ => ServiceLifecycleStatus::InstalledStopped,
    }
}

pub(super) fn collect_service_status(input: &StatusCollectorInput) -> ServiceStatus {
    if let Some(manager) = input.maybe_service_manager.as_ref() {
        match manager.status() {
            Ok(snapshot) => {
                let restart_resume =
                    service_restart_resume_status(&input.config_resolution, Some(&snapshot));
                #[cfg(target_os = "macos")]
                let manager_name = "launchd";
                #[cfg(target_os = "linux")]
                let manager_name = "systemd";
                #[cfg(not(any(target_os = "macos", target_os = "linux")))]
                let manager_name = "unknown";

                let installed = !matches!(snapshot.state, ServiceLifecycleState::Unmanaged);
                let running = matches!(snapshot.state, ServiceLifecycleState::Running);

                return ServiceStatus {
                    manager: FieldAvailability::available(manager_name.to_string()),
                    lifecycle: FieldAvailability::available(service_lifecycle_from_snapshot(
                        &snapshot,
                    )),
                    installed: FieldAvailability::available(installed),
                    enabled: bool_from_maybe(
                        snapshot.maybe_enabled,
                        "service manager did not report enablement",
                    ),
                    running: FieldAvailability::available(running),
                    service_file_path: path_availability(
                        snapshot.maybe_service_file_path.as_deref(),
                        "service file path unavailable",
                    ),
                    log_path: log_path_availability(&snapshot),
                    diagnostics: diagnostics_availability(
                        snapshot.maybe_manager_diagnostics.as_deref(),
                    ),
                    restart_resume,
                };
            }
            Err(error) => {
                let restart_resume = service_restart_resume_status(&input.config_resolution, None);
                let unavailable_reason = format!("service manager unavailable: {error}");
                return ServiceStatus {
                    manager: FieldAvailability::unavailable(unavailable_reason.clone()),
                    lifecycle: FieldAvailability::available(
                        ServiceLifecycleStatus::UnavailableManager,
                    ),
                    installed: FieldAvailability::unavailable(unavailable_reason.clone()),
                    enabled: FieldAvailability::unavailable(unavailable_reason.clone()),
                    running: FieldAvailability::unavailable(unavailable_reason.clone()),
                    service_file_path: FieldAvailability::unavailable(unavailable_reason.clone()),
                    log_path: FieldAvailability::unavailable(unavailable_reason),
                    diagnostics: FieldAvailability::available(error.to_string()),
                    restart_resume,
                };
            }
        }
    }

    let restart_resume = service_restart_resume_status(&input.config_resolution, None);
    detection_service_status(&input.detection_evidence, restart_resume)
}

fn detection_service_status(
    evidence: &StatusDetectionEvidence,
    restart_resume: FieldAvailability<ServiceRestartResumeStatus>,
) -> ServiceStatus {
    let maybe_candidate = evidence
        .service_candidates
        .iter()
        .find(|candidate| candidate.present);

    if let Some(candidate) = maybe_candidate {
        return ServiceStatus {
            manager: FieldAvailability::available(detection::service_manager_name(
                candidate.manager,
            )),
            lifecycle: FieldAvailability::available(ServiceLifecycleStatus::UnavailableManager),
            installed: FieldAvailability::available(true),
            enabled: FieldAvailability::unavailable("service manager not inspected"),
            running: FieldAvailability::unavailable("service manager not inspected"),
            service_file_path: FieldAvailability::available(candidate.path.display().to_string()),
            log_path: FieldAvailability::unavailable("service manager not inspected"),
            diagnostics: FieldAvailability::unavailable("service manager not inspected"),
            restart_resume,
        };
    }

    ServiceStatus {
        manager: FieldAvailability::unavailable("service manager not inspected"),
        lifecycle: FieldAvailability::available(ServiceLifecycleStatus::UnavailableManager),
        installed: FieldAvailability::unavailable("service manager not inspected"),
        enabled: FieldAvailability::unavailable("service manager not inspected"),
        running: FieldAvailability::unavailable("service manager not inspected"),
        service_file_path: FieldAvailability::unavailable("service manager not inspected"),
        log_path: FieldAvailability::unavailable("service manager not inspected"),
        diagnostics: FieldAvailability::unavailable("service manager not inspected"),
        restart_resume,
    }
}

fn bool_from_maybe(maybe_value: Option<bool>, unavailable_reason: &str) -> FieldAvailability<bool> {
    match maybe_value {
        Some(value) => FieldAvailability::available(value),
        None => FieldAvailability::unavailable(unavailable_reason),
    }
}

fn path_availability(
    maybe_path: Option<&std::path::Path>,
    unavailable_reason: &str,
) -> FieldAvailability<String> {
    match maybe_path {
        Some(path) => FieldAvailability::available(path.display().to_string()),
        None => FieldAvailability::unavailable(unavailable_reason),
    }
}

fn log_path_availability(
    snapshot: &crate::operator::service::ServiceStateSnapshot,
) -> FieldAvailability<String> {
    if let Some(path) = snapshot.maybe_log_path.as_deref() {
        return FieldAvailability::available(path.display().to_string());
    }
    FieldAvailability::unavailable(
        snapshot
            .maybe_log_path_unavailable_reason
            .as_deref()
            .unwrap_or("service log path unavailable"),
    )
}

fn diagnostics_availability(maybe_diagnostics: Option<&str>) -> FieldAvailability<String> {
    let Some(diagnostics) = maybe_diagnostics
        .map(str::trim)
        .filter(|diagnostics| !diagnostics.is_empty())
    else {
        return FieldAvailability::unavailable("service diagnostics unavailable");
    };
    FieldAvailability::available(diagnostics.to_string())
}

pub(super) fn service_restart_resume_status(
    resolution: &OperatorConfigResolution,
    maybe_snapshot: Option<&ServiceStateSnapshot>,
) -> FieldAvailability<ServiceRestartResumeStatus> {
    let Some(datadir) = resolution.maybe_data_dir.as_ref() else {
        return FieldAvailability::unavailable(
            "service restart/resume evidence unavailable: datadir unavailable",
        );
    };
    let Some(snapshot) = maybe_snapshot else {
        return FieldAvailability::unavailable(
            "service restart/resume evidence unavailable: runtime metadata unavailable",
        );
    };

    FieldAvailability::available(ServiceRestartResumeStatus {
        datadir: FieldAvailability::available(datadir.display().to_string()),
        same_datadir: same_datadir_status(datadir, Some(snapshot)),
        prior_shutdown: FieldAvailability::unavailable(PROBE_ONLY_RUNTIME_METADATA_REASON),
        durable_progress: FieldAvailability::unavailable(PROBE_ONLY_RUNTIME_METADATA_REASON),
        stale_inflight: FieldAvailability::unavailable(PROBE_ONLY_RUNTIME_METADATA_REASON),
        recovery_category: FieldAvailability::unavailable("no recovery category recorded"),
        next_action: FieldAvailability::unavailable(PROBE_ONLY_RUNTIME_METADATA_REASON),
    })
}

fn same_datadir_status(
    selected_datadir: &std::path::Path,
    maybe_snapshot: Option<&ServiceStateSnapshot>,
) -> FieldAvailability<bool> {
    let Some(snapshot) = maybe_snapshot else {
        return FieldAvailability::unavailable("service datadir unavailable");
    };
    if let Some(service_data_dir) = snapshot.maybe_data_dir.as_deref() {
        return FieldAvailability::available(service_data_dir == selected_datadir);
    }
    FieldAvailability::unavailable(
        snapshot
            .maybe_data_dir_unavailable_reason
            .as_deref()
            .unwrap_or("service datadir unavailable"),
    )
}
