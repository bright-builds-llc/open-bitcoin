// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Service status projection for the shared operator status snapshot.

use open_bitcoin_node::status::{FieldAvailability, ServiceLifecycleStatus, ServiceStatus};

use super::{StatusCollectorInput, StatusDetectionEvidence, detection};
use crate::operator::service::ServiceLifecycleState;

pub(super) fn collect_service_status(input: &StatusCollectorInput) -> ServiceStatus {
    if let Some(manager) = input.maybe_service_manager.as_ref() {
        match manager.status() {
            Ok(snapshot) => {
                #[cfg(target_os = "macos")]
                let manager_name = "launchd";
                #[cfg(target_os = "linux")]
                let manager_name = "systemd";
                #[cfg(not(any(target_os = "macos", target_os = "linux")))]
                let manager_name = "unknown";

                let installed = !matches!(snapshot.state, ServiceLifecycleState::Unmanaged);
                let enabled = snapshot.maybe_enabled.unwrap_or(matches!(
                    snapshot.state,
                    ServiceLifecycleState::Enabled
                        | ServiceLifecycleState::Running
                        | ServiceLifecycleState::Stopped
                ));
                let running = matches!(snapshot.state, ServiceLifecycleState::Running);

                return ServiceStatus {
                    manager: FieldAvailability::available(manager_name.to_string()),
                    lifecycle: FieldAvailability::available(if running {
                        ServiceLifecycleStatus::Running
                    } else if installed {
                        ServiceLifecycleStatus::InstalledStopped
                    } else {
                        ServiceLifecycleStatus::Unmanaged
                    }),
                    installed: FieldAvailability::available(installed),
                    enabled: FieldAvailability::available(enabled),
                    running: FieldAvailability::available(running),
                    service_file_path: FieldAvailability::unavailable(
                        "service file path unavailable",
                    ),
                    log_path: FieldAvailability::unavailable("service log path unavailable"),
                    diagnostics: FieldAvailability::unavailable("service diagnostics unavailable"),
                };
            }
            Err(_) => {
                return ServiceStatus {
                    manager: FieldAvailability::unavailable("service manager not inspected"),
                    lifecycle: FieldAvailability::unavailable("service manager not inspected"),
                    installed: FieldAvailability::unavailable("service manager not inspected"),
                    enabled: FieldAvailability::unavailable("service manager not inspected"),
                    running: FieldAvailability::unavailable("service manager not inspected"),
                    service_file_path: FieldAvailability::unavailable(
                        "service file path unavailable",
                    ),
                    log_path: FieldAvailability::unavailable("service log path unavailable"),
                    diagnostics: FieldAvailability::unavailable("service diagnostics unavailable"),
                };
            }
        }
    }

    detection_service_status(&input.detection_evidence)
}

fn detection_service_status(evidence: &StatusDetectionEvidence) -> ServiceStatus {
    let maybe_candidate = evidence
        .service_candidates
        .iter()
        .find(|candidate| candidate.present);

    if let Some(candidate) = maybe_candidate {
        return ServiceStatus {
            manager: FieldAvailability::available(detection::service_manager_name(
                candidate.manager,
            )),
            lifecycle: FieldAvailability::unavailable("service manager not inspected"),
            installed: FieldAvailability::available(true),
            enabled: FieldAvailability::unavailable("service manager not inspected"),
            running: FieldAvailability::unavailable("service manager not inspected"),
            service_file_path: FieldAvailability::available(candidate.path.display().to_string()),
            log_path: FieldAvailability::unavailable("service log path unavailable"),
            diagnostics: FieldAvailability::unavailable("service diagnostics unavailable"),
        };
    }

    ServiceStatus {
        manager: FieldAvailability::unavailable("service manager not inspected"),
        lifecycle: FieldAvailability::unavailable("service manager not inspected"),
        installed: FieldAvailability::unavailable("service manager not inspected"),
        enabled: FieldAvailability::unavailable("service manager not inspected"),
        running: FieldAvailability::unavailable("service manager not inspected"),
        service_file_path: FieldAvailability::unavailable("service file path unavailable"),
        log_path: FieldAvailability::unavailable("service log path unavailable"),
        diagnostics: FieldAvailability::unavailable("service diagnostics unavailable"),
    }
}
