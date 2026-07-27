use super::*;

#[test]
fn phase63_service_lifecycle_rendering_direct_status_labels() {
    // Arrange
    let cases = [
        (
            ServiceStateSnapshot {
                state: ServiceLifecycleState::Unmanaged,
                maybe_enabled: None,
                maybe_service_file_path: None,
                maybe_manager_diagnostics: None,
                maybe_log_path: None,
                maybe_log_path_unavailable_reason: Some("service not installed".to_string()),
                maybe_data_dir: None,
                maybe_data_dir_unavailable_reason: Some("service not installed".to_string()),
            },
            "service: unmanaged",
        ),
        (
            ServiceStateSnapshot {
                state: ServiceLifecycleState::Installed,
                maybe_enabled: Some(true),
                maybe_service_file_path: Some(PathBuf::from("/tmp/open-bitcoin-node.service")),
                maybe_manager_diagnostics: None,
                maybe_log_path: Some(PathBuf::from("/tmp/logs/open-bitcoin.log")),
                maybe_log_path_unavailable_reason: None,
                maybe_data_dir: Some(PathBuf::from("/tmp/open-bitcoin")),
                maybe_data_dir_unavailable_reason: None,
            },
            "service: installed-stopped",
        ),
        (
            ServiceStateSnapshot {
                state: ServiceLifecycleState::Running,
                maybe_enabled: Some(false),
                maybe_service_file_path: Some(PathBuf::from("/tmp/open-bitcoin-node.service")),
                maybe_manager_diagnostics: Some("running while startup is disabled".to_string()),
                maybe_log_path: Some(PathBuf::from("/tmp/logs/open-bitcoin.log")),
                maybe_log_path_unavailable_reason: None,
                maybe_data_dir: Some(PathBuf::from("/tmp/open-bitcoin")),
                maybe_data_dir_unavailable_reason: None,
            },
            "service: running",
        ),
        (
            ServiceStateSnapshot {
                state: ServiceLifecycleState::Failed,
                maybe_enabled: Some(true),
                maybe_service_file_path: Some(PathBuf::from("/tmp/open-bitcoin-node.service")),
                maybe_manager_diagnostics: Some("systemctl is-active=failed".to_string()),
                maybe_log_path: Some(PathBuf::from("/tmp/logs/open-bitcoin.log")),
                maybe_log_path_unavailable_reason: None,
                maybe_data_dir: Some(PathBuf::from("/tmp/open-bitcoin")),
                maybe_data_dir_unavailable_reason: None,
            },
            "service: failed",
        ),
        (
            ServiceStateSnapshot {
                state: ServiceLifecycleState::Stopped,
                maybe_enabled: Some(false),
                maybe_service_file_path: Some(PathBuf::from("/tmp/open-bitcoin-node.service")),
                maybe_manager_diagnostics: None,
                maybe_log_path: Some(PathBuf::from("/tmp/logs/open-bitcoin.log")),
                maybe_log_path_unavailable_reason: None,
                maybe_data_dir: Some(PathBuf::from("/tmp/open-bitcoin")),
                maybe_data_dir_unavailable_reason: None,
            },
            "service: disabled",
        ),
    ];

    // Act
    let actual = cases
        .iter()
        .map(|(snapshot, _)| first_status_line(snapshot.clone()))
        .collect::<Vec<_>>();

    // Assert
    assert_eq!(
        actual,
        cases
            .iter()
            .map(|(_, expected)| expected.to_string())
            .collect::<Vec<_>>()
    );

    struct ErrorStatusManager;
    impl ServiceManager for ErrorStatusManager {
        fn install(
            &self,
            _request: &ServiceInstallRequest,
        ) -> Result<ServiceCommandOutcome, ServiceError> {
            Err(ServiceError::UnsupportedPlatform {
                reason: "not used by this test".to_string(),
            })
        }

        fn uninstall(
            &self,
            _request: &crate::operator::service::ServiceUninstallRequest,
        ) -> Result<ServiceCommandOutcome, ServiceError> {
            Err(ServiceError::UnsupportedPlatform {
                reason: "not used by this test".to_string(),
            })
        }

        fn enable(
            &self,
            _request: &crate::operator::service::ServiceEnableRequest,
        ) -> Result<ServiceCommandOutcome, ServiceError> {
            Err(ServiceError::UnsupportedPlatform {
                reason: "not used by this test".to_string(),
            })
        }

        fn disable(
            &self,
            _request: &crate::operator::service::ServiceDisableRequest,
        ) -> Result<ServiceCommandOutcome, ServiceError> {
            Err(ServiceError::UnsupportedPlatform {
                reason: "not used by this test".to_string(),
            })
        }

        fn status(&self) -> Result<ServiceStateSnapshot, ServiceError> {
            Err(ServiceError::UnsupportedPlatform {
                reason: "system service manager unavailable".to_string(),
            })
        }
    }

    let manager = ErrorStatusManager;
    let cli = OperatorCli::try_parse_from(["open-bitcoin", "service", "status"])
        .expect("service status should parse");
    let crate::operator::OperatorCommand::Service(service_args) = &cli.command else {
        panic!("expected Service command");
    };

    let unavailable = execute_service_command(
        service_args,
        PathBuf::from("/fake/bin/open-bitcoind"),
        PathBuf::from("/fake/datadir"),
        None,
        None,
        &manager,
    );
    assert_eq!(
        unavailable.exit_code,
        crate::operator::runtime::OperatorExitCode::Success
    );
    assert!(
        unavailable
            .stdout
            .text
            .starts_with("service: unavailable-manager")
    );
    assert!(
        unavailable
            .stdout
            .text
            .contains("diagnostics: unsupported platform: system service manager unavailable")
    );
}

#[test]
fn execute_service_command_status_surfaces_unavailable_log_path_reason() {
    // Arrange
    let snapshot = ServiceStateSnapshot {
        state: ServiceLifecycleState::Installed,
        maybe_enabled: Some(false),
        maybe_service_file_path: Some(PathBuf::from("/tmp/open-bitcoin-node.service")),
        maybe_manager_diagnostics: Some("systemctl is-enabled=false".to_string()),
        maybe_log_path: None,
        maybe_log_path_unavailable_reason: Some(
            "installed unit routes service output to journald".to_string(),
        ),
        maybe_data_dir: Some(PathBuf::from("/tmp/open-bitcoin")),
        maybe_data_dir_unavailable_reason: None,
    };
    let manager = FakeServiceManager::new(snapshot);
    let cli = OperatorCli::try_parse_from(["open-bitcoin", "service", "status"]).unwrap();
    let crate::operator::OperatorCommand::Service(service_args) = &cli.command else {
        panic!("expected Service command");
    };

    // Act
    let outcome = execute_service_command(
        service_args,
        PathBuf::from("/fake/bin/open-bitcoind"),
        PathBuf::from("/fake/datadir"),
        None,
        None,
        &manager,
    );

    // Assert
    assert_eq!(
        outcome.exit_code,
        crate::operator::runtime::OperatorExitCode::Success
    );
    assert!(
        outcome
            .stdout
            .text
            .contains("logs: Unavailable: installed unit routes service output to journald")
    );
}

#[test]
fn execute_service_command_status_unmanaged_surfaces_preview_hint() {
    // Arrange
    let snapshot = ServiceStateSnapshot {
        state: ServiceLifecycleState::Unmanaged,
        maybe_enabled: Some(false),
        maybe_service_file_path: None,
        maybe_manager_diagnostics: Some(
            "unmanaged — run `open-bitcoin service install` to preview what would be created"
                .to_string(),
        ),
        maybe_log_path: None,
        maybe_log_path_unavailable_reason: Some("service not installed".to_string()),
        maybe_data_dir: None,
        maybe_data_dir_unavailable_reason: Some("service not installed".to_string()),
    };
    let manager = FakeServiceManager::new(snapshot);
    let cli = OperatorCli::try_parse_from(["open-bitcoin", "service", "status"]).unwrap();
    let crate::operator::OperatorCommand::Service(service_args) = &cli.command else {
        panic!("expected Service command");
    };

    // Act
    let outcome = execute_service_command(
        service_args,
        PathBuf::from("/fake/bin/open-bitcoind"),
        PathBuf::from("/fake/datadir"),
        None,
        None,
        &manager,
    );

    // Assert
    assert_eq!(
        outcome.exit_code,
        crate::operator::runtime::OperatorExitCode::Success
    );
    assert!(
        outcome
            .stdout
            .text
            .contains("hint: run `open-bitcoin service install` to preview what would be created")
    );
    assert!(!outcome.stdout.text.contains("--dry-run"));
}

#[test]
fn launchd_status_unmanaged_diagnostics_use_preview_contract() {
    // Arrange
    let test_dir = TestDirectory::new("launchd-unmanaged-preview-hint");
    let adapter = LaunchdAdapter::new(test_dir.path.clone());

    // Act
    let snapshot = adapter.status().expect("launchd unmanaged status");

    // Assert
    assert_eq!(snapshot.state, ServiceLifecycleState::Unmanaged);
    let diagnostics = snapshot
        .maybe_manager_diagnostics
        .expect("launchd unmanaged diagnostics");
    assert!(diagnostics.contains("open-bitcoin service install"));
    assert!(diagnostics.contains("preview"));
    assert!(!diagnostics.contains("--dry-run"));
}

#[test]
fn systemd_status_unmanaged_diagnostics_use_preview_contract() {
    // Arrange
    let test_dir = TestDirectory::new("systemd-unmanaged-preview-hint");
    let adapter = SystemdAdapter::new(test_dir.path.clone());

    // Act
    let snapshot = adapter.status().expect("systemd unmanaged status");

    // Assert
    assert_eq!(snapshot.state, ServiceLifecycleState::Unmanaged);
    let diagnostics = snapshot
        .maybe_manager_diagnostics
        .expect("systemd unmanaged diagnostics");
    assert!(diagnostics.contains("open-bitcoin service install"));
    assert!(diagnostics.contains("preview"));
    assert!(!diagnostics.contains("--dry-run"));
}
