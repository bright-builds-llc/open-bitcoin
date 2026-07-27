// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::*;

#[test]
fn collect_status_snapshot_with_fake_running_manager_sets_service_fields_to_available_true() {
    // Arrange
    let fake = FakeServiceManager::new(ServiceStateSnapshot {
        state: ServiceLifecycleState::Running,
        maybe_enabled: Some(true),
        maybe_service_file_path: Some(PathBuf::from("/tmp/test.plist")),
        maybe_manager_diagnostics: None,
        maybe_log_path: None,
        maybe_log_path_unavailable_reason: Some("service log path unavailable".to_string()),
        maybe_data_dir: Some(PathBuf::from("/tmp/open-bitcoin")),
        maybe_data_dir_unavailable_reason: None,
    });
    let input = StatusCollectorInput {
        request: StatusRequest {
            render_mode: StatusRenderMode::Human,
            maybe_config_path: None,
            maybe_data_dir: None,
            maybe_network: None,
            include_live_rpc: false,
            no_color: false,
        },
        config_resolution: config_resolution(),
        detection_evidence: StatusDetectionEvidence {
            detected_installations: Vec::new(),
            service_candidates: Vec::new(),
        },
        maybe_live_rpc: None,
        maybe_service_manager: Some(Box::new(fake)),
        wallet_rpc_access: StatusWalletRpcAccess::Root,
    };

    // Act
    let snapshot = collect_status_snapshot(&input, None);

    // Assert
    assert!(
        matches!(
            &snapshot.service.manager,
            open_bitcoin_node::status::FieldAvailability::Available(_)
        ),
        "service.manager should be available when running manager injected"
    );
    assert_eq!(
        snapshot.service.installed,
        open_bitcoin_node::status::FieldAvailability::available(true),
        "service.installed should be true when state is Running"
    );
    assert_eq!(
        snapshot.service.enabled,
        open_bitcoin_node::status::FieldAvailability::available(true),
        "service.enabled should be true when state is Running"
    );
    assert_eq!(
        snapshot.service.running,
        open_bitcoin_node::status::FieldAvailability::available(true),
        "service.running should be true when state is Running"
    );
}

#[test]
fn collect_status_snapshot_with_fake_installed_manager_sets_installed_true_enabled_false() {
    // Arrange
    let fake = FakeServiceManager::new(ServiceStateSnapshot {
        state: ServiceLifecycleState::Installed,
        maybe_enabled: Some(false),
        maybe_service_file_path: Some(PathBuf::from("/tmp/test.plist")),
        maybe_manager_diagnostics: None,
        maybe_log_path: None,
        maybe_log_path_unavailable_reason: Some("service log path unavailable".to_string()),
        maybe_data_dir: Some(PathBuf::from("/tmp/open-bitcoin")),
        maybe_data_dir_unavailable_reason: None,
    });
    let input = StatusCollectorInput {
        request: StatusRequest {
            render_mode: StatusRenderMode::Human,
            maybe_config_path: None,
            maybe_data_dir: None,
            maybe_network: None,
            include_live_rpc: false,
            no_color: false,
        },
        config_resolution: config_resolution(),
        detection_evidence: StatusDetectionEvidence {
            detected_installations: Vec::new(),
            service_candidates: Vec::new(),
        },
        maybe_live_rpc: None,
        maybe_service_manager: Some(Box::new(fake)),
        wallet_rpc_access: StatusWalletRpcAccess::Root,
    };

    // Act
    let snapshot = collect_status_snapshot(&input, None);

    // Assert
    assert_eq!(
        snapshot.service.installed,
        open_bitcoin_node::status::FieldAvailability::available(true),
        "service.installed should be true when state is Installed"
    );
    assert_eq!(
        snapshot.service.enabled,
        open_bitcoin_node::status::FieldAvailability::available(false),
        "service.enabled should be false when state is Installed (not Enabled/Running)"
    );
    assert_eq!(
        snapshot.service.running,
        open_bitcoin_node::status::FieldAvailability::available(false),
        "service.running should be false when state is Installed"
    );
}

#[test]
fn collect_status_snapshot_uses_manager_enabled_state_over_state_inference() {
    // Arrange
    let fake = FakeServiceManager::new(ServiceStateSnapshot {
        state: ServiceLifecycleState::Failed,
        maybe_enabled: Some(true),
        maybe_service_file_path: Some(PathBuf::from("/tmp/test.plist")),
        maybe_manager_diagnostics: Some("systemctl is-active=failed".to_string()),
        maybe_log_path: None,
        maybe_log_path_unavailable_reason: Some("service log path unavailable".to_string()),
        maybe_data_dir: Some(PathBuf::from("/tmp/open-bitcoin")),
        maybe_data_dir_unavailable_reason: None,
    });
    let input = StatusCollectorInput {
        request: StatusRequest {
            render_mode: StatusRenderMode::Human,
            maybe_config_path: None,
            maybe_data_dir: None,
            maybe_network: None,
            include_live_rpc: false,
            no_color: false,
        },
        config_resolution: config_resolution(),
        detection_evidence: StatusDetectionEvidence {
            detected_installations: Vec::new(),
            service_candidates: Vec::new(),
        },
        maybe_live_rpc: None,
        maybe_service_manager: Some(Box::new(fake)),
        wallet_rpc_access: StatusWalletRpcAccess::Root,
    };

    // Act
    let snapshot = collect_status_snapshot(&input, None);

    // Assert
    assert_eq!(
        snapshot.service.enabled,
        open_bitcoin_node::status::FieldAvailability::available(true),
        "service.enabled should preserve manager evidence even when state is Failed"
    );
    assert_eq!(
        snapshot.service.running,
        open_bitcoin_node::status::FieldAvailability::available(false),
        "service.running should remain false when state is not Running"
    );
}

#[test]
fn collect_status_snapshot_preserves_running_when_startup_is_not_enabled() {
    // Arrange
    let fake = FakeServiceManager::new(ServiceStateSnapshot {
        state: ServiceLifecycleState::Running,
        maybe_enabled: Some(false),
        maybe_service_file_path: Some(PathBuf::from("/tmp/test.plist")),
        maybe_manager_diagnostics: Some("launchctl service is running but disabled".to_string()),
        maybe_log_path: None,
        maybe_log_path_unavailable_reason: Some("service log path unavailable".to_string()),
        maybe_data_dir: Some(PathBuf::from("/tmp/open-bitcoin")),
        maybe_data_dir_unavailable_reason: None,
    });
    let input = StatusCollectorInput {
        request: StatusRequest {
            render_mode: StatusRenderMode::Human,
            maybe_config_path: None,
            maybe_data_dir: None,
            maybe_network: None,
            include_live_rpc: false,
            no_color: false,
        },
        config_resolution: config_resolution(),
        detection_evidence: StatusDetectionEvidence {
            detected_installations: Vec::new(),
            service_candidates: Vec::new(),
        },
        maybe_live_rpc: None,
        maybe_service_manager: Some(Box::new(fake)),
        wallet_rpc_access: StatusWalletRpcAccess::Root,
    };

    // Act
    let snapshot = collect_status_snapshot(&input, None);

    // Assert
    assert_eq!(
        snapshot.service.enabled,
        open_bitcoin_node::status::FieldAvailability::available(false),
        "service.enabled should come from manager evidence instead of Running inference"
    );
    assert_eq!(
        snapshot.service.running,
        open_bitcoin_node::status::FieldAvailability::available(true),
        "service.running should still be true when the manager reports Running"
    );
}

#[test]
fn collect_status_snapshot_with_error_manager_falls_back_to_unavailable() {
    // Arrange
    struct ErrorServiceManager;
    impl crate::operator::service::ServiceManager for ErrorServiceManager {
        fn install(
            &self,
            _request: &crate::operator::service::ServiceInstallRequest,
        ) -> Result<crate::operator::service::ServiceCommandOutcome, ServiceError> {
            Err(ServiceError::UnsupportedPlatform {
                reason: "test".to_string(),
            })
        }
        fn uninstall(
            &self,
            _request: &crate::operator::service::ServiceUninstallRequest,
        ) -> Result<crate::operator::service::ServiceCommandOutcome, ServiceError> {
            Err(ServiceError::UnsupportedPlatform {
                reason: "test".to_string(),
            })
        }
        fn enable(
            &self,
            _request: &crate::operator::service::ServiceEnableRequest,
        ) -> Result<crate::operator::service::ServiceCommandOutcome, ServiceError> {
            Err(ServiceError::UnsupportedPlatform {
                reason: "test".to_string(),
            })
        }
        fn disable(
            &self,
            _request: &crate::operator::service::ServiceDisableRequest,
        ) -> Result<crate::operator::service::ServiceCommandOutcome, ServiceError> {
            Err(ServiceError::UnsupportedPlatform {
                reason: "test".to_string(),
            })
        }
        fn status(&self) -> Result<ServiceStateSnapshot, ServiceError> {
            Err(ServiceError::UnsupportedPlatform {
                reason: "platform not supported in test".to_string(),
            })
        }
    }

    let path = temp_path("phase63-manager-error-probe-only");
    remove_dir_if_exists(&path);
    let _guard = TempDirGuard { path: path.clone() };

    let mut resolution = config_resolution();
    resolution.maybe_data_dir = Some(path.clone());
    let input = status_input_with_manager(Box::new(ErrorServiceManager), resolution);

    // Act
    let snapshot = collect_status_snapshot(&input, None);

    // Assert
    assert_eq!(
        snapshot.service.lifecycle,
        FieldAvailability::available(ServiceLifecycleStatus::UnavailableManager)
    );
    assert_eq!(
        snapshot.service.manager,
        FieldAvailability::unavailable(
            "service manager unavailable: unsupported platform: platform not supported in test"
        )
    );
    assert_eq!(
        snapshot.service.installed,
        FieldAvailability::unavailable(
            "service manager unavailable: unsupported platform: platform not supported in test"
        )
    );
    assert_eq!(
        snapshot.service.enabled,
        FieldAvailability::unavailable(
            "service manager unavailable: unsupported platform: platform not supported in test"
        )
    );
    assert_eq!(
        snapshot.service.running,
        FieldAvailability::unavailable(
            "service manager unavailable: unsupported platform: platform not supported in test"
        )
    );
    assert_eq!(
        snapshot.service.service_file_path,
        FieldAvailability::unavailable(
            "service manager unavailable: unsupported platform: platform not supported in test"
        )
    );
    assert_eq!(
        snapshot.service.log_path,
        FieldAvailability::unavailable(
            "service manager unavailable: unsupported platform: platform not supported in test"
        )
    );
    assert_eq!(
        snapshot.service.diagnostics,
        FieldAvailability::available(
            "unsupported platform: platform not supported in test".to_string()
        )
    );

    assert!(matches!(
        snapshot.sync.configured_targets,
        FieldAvailability::Unavailable { .. }
    ));
    assert!(matches!(
        snapshot.sync.attempt_counters,
        FieldAvailability::Unavailable { .. }
    ));
    assert!(matches!(
        snapshot.sync.latest_stop_reason,
        FieldAvailability::Unavailable { .. }
    ));
    assert_eq!(
        snapshot.sync.recovery_category,
        FieldAvailability::unavailable("no recovery category recorded")
    );
}

#[test]
fn service_restart_resume_status_surfaces_same_datadir_without_runtime_metadata() {
    // Arrange
    let path = temp_path("service-restart-resume-clean");
    remove_dir_if_exists(&path);
    let _guard = TempDirGuard { path: path.clone() };

    let mut resolution = config_resolution();
    resolution.maybe_data_dir = Some(path.clone());
    let input = status_input_with_manager(
        Box::new(FakeServiceManager::new(service_snapshot(
            ServiceLifecycleState::Running,
            Some(true),
            &path,
        ))),
        resolution,
    );

    // Act
    let snapshot = collect_status_snapshot(&input, None);

    // Assert
    let FieldAvailability::Available(restart_resume) = snapshot.service.restart_resume else {
        panic!("restart/resume evidence should be available");
    };
    assert_eq!(
        restart_resume.datadir,
        FieldAvailability::available(path.display().to_string())
    );
    assert_eq!(
        restart_resume.same_datadir,
        FieldAvailability::available(true)
    );
    assert_eq!(
        restart_resume.prior_shutdown,
        FieldAvailability::unavailable(
            "service restart/resume evidence unavailable: probe-only status does not open Fjall stores"
        )
    );
    assert_eq!(
        restart_resume.stale_inflight,
        FieldAvailability::unavailable(
            "service restart/resume evidence unavailable: probe-only status does not open Fjall stores"
        )
    );
    assert_eq!(
        restart_resume.recovery_category,
        FieldAvailability::unavailable("no recovery category recorded")
    );
    assert_eq!(
        restart_resume.next_action,
        FieldAvailability::unavailable(
            "service restart/resume evidence unavailable: probe-only status does not open Fjall stores"
        )
    );
}

#[test]
fn service_restart_resume_status_does_not_load_unclean_stale_inflight_metadata() {
    // Arrange
    let path = temp_path("service-restart-resume-unclean");
    remove_dir_if_exists(&path);
    let _guard = TempDirGuard { path: path.clone() };

    let mut resolution = config_resolution();
    resolution.maybe_data_dir = Some(path.clone());
    let input = status_input_with_manager(
        Box::new(FakeServiceManager::new(service_snapshot(
            ServiceLifecycleState::Running,
            Some(true),
            &path,
        ))),
        resolution,
    );

    // Act
    let snapshot = collect_status_snapshot(&input, None);

    // Assert
    let FieldAvailability::Available(restart_resume) = snapshot.service.restart_resume else {
        panic!("restart/resume evidence should be available");
    };
    assert_eq!(
        restart_resume.prior_shutdown,
        FieldAvailability::unavailable(
            "service restart/resume evidence unavailable: probe-only status does not open Fjall stores"
        )
    );
    assert_eq!(
        restart_resume.stale_inflight,
        FieldAvailability::unavailable(
            "service restart/resume evidence unavailable: probe-only status does not open Fjall stores"
        )
    );
}

#[test]
fn service_restart_resume_status_reports_datadir_mismatch() {
    // Arrange
    let path = temp_path("service-restart-resume-datadir-mismatch");
    remove_dir_if_exists(&path);
    let _guard = TempDirGuard { path: path.clone() };

    let mut resolution = config_resolution();
    resolution.maybe_data_dir = Some(path);
    let input = status_input_with_manager(
        Box::new(FakeServiceManager::new(service_snapshot(
            ServiceLifecycleState::Running,
            Some(true),
            Path::new("/tmp/different-open-bitcoin"),
        ))),
        resolution,
    );

    // Act
    let snapshot = collect_status_snapshot(&input, None);

    // Assert
    let FieldAvailability::Available(restart_resume) = snapshot.service.restart_resume else {
        panic!("restart/resume evidence should be available");
    };
    assert_eq!(
        restart_resume.same_datadir,
        FieldAvailability::available(false)
    );
}

#[test]
fn service_restart_resume_status_reports_unavailable_selected_datadir() {
    // Arrange
    let input = status_input_with_manager(
        Box::new(FakeServiceManager::new(service_snapshot(
            ServiceLifecycleState::Running,
            Some(true),
            Path::new("/tmp/open-bitcoin"),
        ))),
        OperatorConfigResolution {
            maybe_data_dir: None,
            ..config_resolution()
        },
    );

    // Act
    let snapshot = collect_status_snapshot(&input, None);

    // Assert
    assert_eq!(
        snapshot.service.restart_resume,
        FieldAvailability::unavailable(
            "service restart/resume evidence unavailable: datadir unavailable"
        )
    );
}
