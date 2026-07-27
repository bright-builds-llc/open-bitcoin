use super::*;

#[test]
fn execute_service_command_service_preview_renders_install_preview_output() {
    // Arrange
    let mut manager = FakeServiceManager::unmanaged();
    manager.install_outcome = Some(preview_install_outcome());
    let cli = OperatorCli::try_parse_from(["open-bitcoin", "service", "preview"])
        .expect("service preview should parse");
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
    let stdout = &outcome.stdout.text;
    assert!(stdout.contains("Dry run"), "missing dry-run text: {stdout}");
    assert!(
        stdout.contains("Would write"),
        "missing file path preview: {stdout}"
    );
    assert!(
        stdout.contains("Generated content"),
        "missing generated content: {stdout}"
    );
    assert!(
        stdout.contains("Commands"),
        "missing manager commands: {stdout}"
    );
    assert!(
        stdout.contains("Scope: user-level"),
        "missing user-level scope: {stdout}"
    );
}

#[test]
fn execute_service_command_service_preview_apply_rejects_without_manager_call() {
    // Arrange
    let manager = FakeServiceManager::unmanaged();
    let cli = OperatorCli::try_parse_from(["open-bitcoin", "service", "preview", "--apply"])
        .expect("service preview --apply should parse before dispatcher rejection");
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
        crate::operator::runtime::OperatorExitCode::Failure(1)
    );
    assert_eq!(
        outcome.stderr.text,
        "service preview is always side-effect-free; remove --apply"
    );
    assert!(
        manager.recorded_calls.borrow().is_empty(),
        "preview --apply must not call the manager"
    );
}

#[test]
fn execute_service_command_service_install_without_apply_still_records_dry_run() {
    // Arrange
    let manager = FakeServiceManager::unmanaged();
    let cli = OperatorCli::try_parse_from(["open-bitcoin", "service", "install"])
        .expect("service install should parse");
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
    assert_eq!(
        manager.recorded_calls.borrow().as_slice(),
        &[FakeServiceCall::Install { apply: false }]
    );
}

#[test]
fn execute_service_command_install_dry_run_shows_dry_run_output() {
    // Arrange
    let manager = FakeServiceManager::unmanaged();
    let cli = OperatorCli::try_parse_from(["open-bitcoin", "service", "install"]).unwrap();
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
        crate::operator::runtime::OperatorExitCode::Success,
        "dry-run install should succeed"
    );
    let stdout = &outcome.stdout.text;
    assert!(
        stdout.contains("Dry run") || stdout.contains("dry run") || stdout.contains("dry-run"),
        "stdout should contain dry run indicator: {stdout}"
    );
}

#[test]
fn execute_service_command_install_already_installed_returns_failure() {
    // Arrange
    let mut manager = FakeServiceManager::unmanaged();
    manager.install_error = Some(ServiceError::AlreadyInstalled {
        path: PathBuf::from("/fake/LaunchAgents/org.open-bitcoin.node.plist"),
    });
    let cli =
        OperatorCli::try_parse_from(["open-bitcoin", "service", "install", "--apply"]).unwrap();
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
        crate::operator::runtime::OperatorExitCode::Failure(1),
        "already installed should return failure"
    );
    assert!(
        outcome.stderr.text.contains("already installed"),
        "stderr should contain 'already installed': {}",
        outcome.stderr.text
    );
}

#[test]
fn execute_service_command_enable_returns_success_with_output() {
    // Arrange
    let manager = FakeServiceManager::unmanaged();
    let cli = OperatorCli::try_parse_from(["open-bitcoin", "service", "enable"]).unwrap();
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
        crate::operator::runtime::OperatorExitCode::Success,
        "enable should succeed"
    );
    assert!(
        !outcome.stdout.text.is_empty(),
        "enable should produce output"
    );
}

#[test]
fn execute_service_command_uninstall_dry_run_succeeds() {
    // Arrange
    let manager = FakeServiceManager::unmanaged();
    let cli = OperatorCli::try_parse_from(["open-bitcoin", "service", "uninstall"]).unwrap();
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
        crate::operator::runtime::OperatorExitCode::Success,
        "dry-run uninstall with fake manager should succeed"
    );
}

#[test]
fn parsing_service_install_with_apply_flag_sets_apply_true() {
    // Arrange / Act
    let cli =
        OperatorCli::try_parse_from(["open-bitcoin", "service", "install", "--apply"]).unwrap();

    // Assert
    let crate::operator::OperatorCommand::Service(service_args) = &cli.command else {
        panic!("expected Service command");
    };
    assert!(
        service_args.apply,
        "apply flag should be true when --apply is passed"
    );
}

#[test]
fn parsing_service_install_without_apply_flag_sets_apply_false() {
    // Arrange / Act
    let cli = OperatorCli::try_parse_from(["open-bitcoin", "service", "install"]).unwrap();

    // Assert
    let crate::operator::OperatorCommand::Service(service_args) = &cli.command else {
        panic!("expected Service command");
    };
    assert!(!service_args.apply, "apply flag should be false by default");
}

#[test]
fn execute_service_command_install_dry_run_shows_scope() {
    // Arrange
    let manager = FakeServiceManager::unmanaged();
    let cli = OperatorCli::try_parse_from(["open-bitcoin", "service", "install"]).unwrap();
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
    let stdout = &outcome.stdout.text;
    assert!(
        stdout.contains("user-level") || stdout.contains("Scope"),
        "stdout should mention user-level scope: {stdout}"
    );
}

#[test]
fn execute_service_command_status_surfaces_enabled_and_running_flags() {
    // Arrange
    let snapshot = ServiceStateSnapshot {
        state: ServiceLifecycleState::Failed,
        maybe_enabled: Some(true),
        maybe_service_file_path: Some(PathBuf::from("/tmp/open-bitcoin-node.service")),
        maybe_manager_diagnostics: Some("systemctl is-active=failed".to_string()),
        maybe_log_path: Some(PathBuf::from("/tmp/logs/open-bitcoin.log")),
        maybe_log_path_unavailable_reason: None,
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
    assert!(outcome.stdout.text.contains("service: failed"));
    assert!(outcome.stdout.text.contains("enabled: true"));
    assert!(outcome.stdout.text.contains("running: false"));
    assert!(
        outcome
            .stdout
            .text
            .contains("logs: /tmp/logs/open-bitcoin.log")
    );
}
