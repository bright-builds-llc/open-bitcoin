// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::*;

#[test]
fn phase63_service_lifecycle_status_contract_serializes_labels() {
    // Arrange
    let installed_stopped = ServiceLifecycleStatus::InstalledStopped;
    let unavailable_manager = ServiceLifecycleStatus::UnavailableManager;

    // Act
    let installed_stopped_json =
        serde_json::to_value(installed_stopped).expect("installed-stopped status json");
    let unavailable_manager_json =
        serde_json::to_value(unavailable_manager).expect("unavailable-manager status json");

    // Assert
    assert_eq!(installed_stopped.as_str(), "installed-stopped");
    assert_eq!(unavailable_manager.as_str(), "unavailable-manager");
    assert_eq!(installed_stopped_json, "installed-stopped");
    assert_eq!(unavailable_manager_json, "unavailable-manager");
}

#[test]
fn phase63_service_lifecycle_status_contract_defaults_legacy_json() {
    // Arrange
    let legacy_json = serde_json::json!({
        "manager": { "state": "available", "value": "launchd" },
        "installed": { "state": "available", "value": true },
        "enabled": { "state": "available", "value": true },
        "running": { "state": "available", "value": false }
    });

    // Act
    let service: ServiceStatus =
        serde_json::from_value(legacy_json).expect("legacy service status json");

    // Assert
    assert_eq!(
        service.lifecycle,
        FieldAvailability::unavailable("service lifecycle unavailable")
    );
    assert_eq!(
        service.service_file_path,
        FieldAvailability::unavailable("service file path unavailable")
    );
    assert_eq!(
        service.log_path,
        FieldAvailability::unavailable("service log path unavailable")
    );
    assert_eq!(
        service.diagnostics,
        FieldAvailability::unavailable("service diagnostics unavailable")
    );
}

#[test]
fn service_restart_resume_status_contract_serializes_labels() {
    // Arrange
    let clean_shutdown = ServicePriorShutdownStatus::Clean;
    let unclean_shutdown = ServicePriorShutdownStatus::Unclean;
    let stale_inflight = ServiceStaleInflightStatus::StaleRequestsRecorded;

    // Act
    let clean_json = serde_json::to_value(clean_shutdown).expect("clean shutdown json");
    let unclean_json = serde_json::to_value(unclean_shutdown).expect("unclean shutdown json");
    let stale_json = serde_json::to_value(stale_inflight).expect("stale in-flight json");

    // Assert
    assert_eq!(clean_shutdown.as_str(), "clean");
    assert_eq!(unclean_shutdown.as_str(), "unclean");
    assert_eq!(stale_inflight.as_str(), "stale_requests_recorded");
    assert_eq!(clean_json, "clean");
    assert_eq!(unclean_json, "unclean");
    assert_eq!(stale_json, "stale_requests_recorded");
}

#[test]
fn service_restart_resume_status_contract_defaults_legacy_json() {
    // Arrange
    let legacy_json = serde_json::json!({
        "manager": { "state": "available", "value": "launchd" },
        "lifecycle": { "state": "available", "value": "running" },
        "installed": { "state": "available", "value": true },
        "enabled": { "state": "available", "value": true },
        "running": { "state": "available", "value": true },
        "service_file_path": { "state": "available", "value": "/tmp/open-bitcoin-node.service" },
        "log_path": { "state": "available", "value": "/tmp/logs/open-bitcoin.log" },
        "diagnostics": { "state": "unavailable", "value": { "reason": "service diagnostics unavailable" } }
    });

    // Act
    let service: ServiceStatus =
        serde_json::from_value(legacy_json).expect("legacy service status json");

    // Assert
    assert_eq!(
        service.restart_resume,
        FieldAvailability::<ServiceRestartResumeStatus>::unavailable(
            "service restart/resume evidence unavailable"
        )
    );
}
