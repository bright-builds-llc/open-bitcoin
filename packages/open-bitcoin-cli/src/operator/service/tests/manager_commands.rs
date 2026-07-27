// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::*;

#[test]
fn launchd_data_dir_parser_reads_service_datadir_argument() {
    // Arrange
    let plist = generate_plist_content(
        "org.open-bitcoin.node",
        Path::new("/fake/bin/open-bitcoind"),
        Path::new("/fake/datadir"),
        None,
        None,
    );

    // Act
    let maybe_data_dir = parse_launchd_data_dir(&plist);

    // Assert
    assert_eq!(maybe_data_dir, Some(PathBuf::from("/fake/datadir")));
}

#[test]
fn systemd_enabled_state_parser_classifies_common_states() {
    // Arrange / Act / Assert
    assert_eq!(parse_systemd_enabled_state("enabled\n"), Some(true));
    assert_eq!(parse_systemd_enabled_state("disabled\n"), Some(false));
    assert_eq!(parse_systemd_enabled_state("masked\n"), Some(false));
}

#[test]
fn systemd_log_path_parser_reads_append_path() {
    // Arrange
    let unit = "\
[Service]\n\
StandardOutput=append:/fake/logs/open-bitcoin.log\n\
StandardError=append:/fake/logs/open-bitcoin.log\n";

    // Act
    let maybe_log_path = parse_systemd_log_path(unit);

    // Assert
    assert_eq!(
        maybe_log_path,
        Some(PathBuf::from("/fake/logs/open-bitcoin.log"))
    );
}

#[test]
fn systemd_data_dir_parser_reads_service_datadir_argument() {
    // Arrange
    let unit = generate_unit_content(
        Path::new("/fake/bin/open-bitcoind"),
        Path::new("/fake/datadir"),
        None,
        None,
    );

    // Act
    let maybe_data_dir = parse_systemd_data_dir(&unit);

    // Assert
    assert_eq!(maybe_data_dir, Some(PathBuf::from("/fake/datadir")));
}

#[test]
fn launchd_start_stop_restart_commands_are_user_scope() {
    // Arrange
    let plist_path = Path::new("/fake/home/Library/LaunchAgents/org.open-bitcoin.node.plist");
    let uid = 501;

    // Act
    let commands = [
        launchd_start_command(uid, plist_path),
        launchd_stop_command(uid),
        launchd_restart_command(uid),
    ];

    // Assert
    assert_eq!(
        commands[0],
        "launchctl bootstrap gui/501 /fake/home/Library/LaunchAgents/org.open-bitcoin.node.plist"
    );
    assert_eq!(
        commands[1],
        "launchctl bootout gui/501/org.open-bitcoin.node"
    );
    assert_eq!(
        commands[2],
        "launchctl kickstart -k gui/501/org.open-bitcoin.node"
    );
    for command in commands {
        assert!(
            !command.contains("sudo")
                && !command.contains("/Library/LaunchDaemons")
                && !command.contains("bitcoind.service")
                && !command.contains("bitcoin-knots"),
            "launchd lifecycle command must stay user-scope: {command}"
        );
    }
}

#[test]
fn launchd_start_stop_restart_missing_plist_returns_not_installed() {
    // Arrange
    let test_dir = TestDirectory::new("launchd-missing-lifecycle");
    let adapter = LaunchdAdapter::new(test_dir.path.clone());

    // Act
    let start_result = adapter.start(&ServiceStartRequest);
    let stop_result = adapter.stop(&ServiceStopRequest);
    let restart_result = adapter.restart(&ServiceRestartRequest);

    // Assert
    assert!(
        matches!(start_result, Err(ServiceError::NotInstalled)),
        "missing launchd plist should block start: {start_result:?}"
    );
    assert!(
        matches!(stop_result, Err(ServiceError::NotInstalled)),
        "missing launchd plist should block stop: {stop_result:?}"
    );
    assert!(
        matches!(restart_result, Err(ServiceError::NotInstalled)),
        "missing launchd plist should block restart: {restart_result:?}"
    );
}

#[test]
fn systemd_start_stop_restart_commands_are_user_scope() {
    // Arrange / Act
    let commands = [
        systemd_start_command(),
        systemd_stop_command(),
        systemd_restart_command(),
    ];

    // Assert
    assert_eq!(
        commands[0],
        "systemctl --user start open-bitcoin-node.service"
    );
    assert_eq!(
        commands[1],
        "systemctl --user stop open-bitcoin-node.service"
    );
    assert_eq!(
        commands[2],
        "systemctl --user restart open-bitcoin-node.service"
    );
    for command in commands {
        assert!(
            !command.contains("sudo")
                && !command.contains("/etc/systemd/system")
                && !command.contains("bitcoind.service")
                && !command.contains("bitcoin-knots"),
            "systemd lifecycle command must stay user-scope: {command}"
        );
    }
}

#[test]
fn systemd_start_stop_restart_missing_unit_returns_not_installed() {
    // Arrange
    let test_dir = TestDirectory::new("systemd-missing-lifecycle");
    let adapter = SystemdAdapter::new(test_dir.path.clone());

    // Act
    let start_result = adapter.start(&ServiceStartRequest);
    let stop_result = adapter.stop(&ServiceStopRequest);
    let restart_result = adapter.restart(&ServiceRestartRequest);

    // Assert
    assert!(
        matches!(start_result, Err(ServiceError::NotInstalled)),
        "missing systemd unit should block start: {start_result:?}"
    );
    assert!(
        matches!(stop_result, Err(ServiceError::NotInstalled)),
        "missing systemd unit should block stop: {stop_result:?}"
    );
    assert!(
        matches!(restart_result, Err(ServiceError::NotInstalled)),
        "missing systemd unit should block restart: {restart_result:?}"
    );
}

// --- execute_service_command tests ---

#[test]
fn parsing_service_start_selects_service_start_command() {
    // Arrange / Act
    let cli = OperatorCli::try_parse_from(["open-bitcoin", "service", "start"])
        .expect("service start should parse");

    // Assert
    let crate::operator::OperatorCommand::Service(service_args) = &cli.command else {
        panic!("expected Service command");
    };
    assert_eq!(service_args.command, ServiceCommand::Start);
    assert!(
        !service_args.apply,
        "service start must not require --apply to parse"
    );
}

#[test]
fn parsing_service_stop_selects_service_stop_command() {
    // Arrange / Act
    let cli = OperatorCli::try_parse_from(["open-bitcoin", "service", "stop"])
        .expect("service stop should parse");

    // Assert
    let crate::operator::OperatorCommand::Service(service_args) = &cli.command else {
        panic!("expected Service command");
    };
    assert_eq!(service_args.command, ServiceCommand::Stop);
    assert!(
        !service_args.apply,
        "service stop must not require --apply to parse"
    );
}

#[test]
fn parsing_service_restart_selects_service_restart_command() {
    // Arrange / Act
    let cli = OperatorCli::try_parse_from(["open-bitcoin", "service", "restart"])
        .expect("service restart should parse");

    // Assert
    let crate::operator::OperatorCommand::Service(service_args) = &cli.command else {
        panic!("expected Service command");
    };
    assert_eq!(service_args.command, ServiceCommand::Restart);
    assert!(
        !service_args.apply,
        "service restart must not require --apply to parse"
    );
}

#[test]
fn fake_manager_service_start_stop_restart_records_call_order() {
    // Arrange
    let manager = FakeServiceManager::unmanaged();

    // Act
    let start_result = manager.start(&ServiceStartRequest);
    let stop_result = manager.stop(&ServiceStopRequest);
    let restart_result = manager.restart(&ServiceRestartRequest);

    // Assert
    assert!(
        start_result.is_ok(),
        "start should succeed: {start_result:?}"
    );
    assert!(stop_result.is_ok(), "stop should succeed: {stop_result:?}");
    assert!(
        restart_result.is_ok(),
        "restart should succeed: {restart_result:?}"
    );
    assert_eq!(
        manager.recorded_calls.borrow().as_slice(),
        &[
            FakeServiceCall::Start,
            FakeServiceCall::Stop,
            FakeServiceCall::Restart
        ]
    );
}

#[test]
fn execute_service_command_service_start_calls_manager_and_renders_commands() {
    // Arrange
    let mut manager = FakeServiceManager::unmanaged();
    manager.start_commands = vec!["systemctl --user start open-bitcoin-node.service".to_string()];
    let cli = OperatorCli::try_parse_from(["open-bitcoin", "service", "start"])
        .expect("service start should parse");
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
        &[FakeServiceCall::Start]
    );
    assert!(
        outcome
            .stdout
            .text
            .contains("systemctl --user start open-bitcoin-node.service"),
        "stdout should contain start command: {}",
        outcome.stdout.text
    );
    assert!(
        !outcome.stdout.text.contains("Dry run"),
        "start should be effectful rather than a dry run: {}",
        outcome.stdout.text
    );
}

#[test]
fn execute_service_command_service_stop_calls_manager_and_renders_commands() {
    // Arrange
    let mut manager = FakeServiceManager::unmanaged();
    manager.stop_commands = vec!["systemctl --user stop open-bitcoin-node.service".to_string()];
    let cli = OperatorCli::try_parse_from(["open-bitcoin", "service", "stop"])
        .expect("service stop should parse");
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
        &[FakeServiceCall::Stop]
    );
    assert!(
        outcome
            .stdout
            .text
            .contains("systemctl --user stop open-bitcoin-node.service"),
        "stdout should contain stop command: {}",
        outcome.stdout.text
    );
    assert!(
        !outcome.stdout.text.contains("Dry run"),
        "stop should be effectful rather than a dry run: {}",
        outcome.stdout.text
    );
}

#[test]
fn execute_service_command_service_restart_calls_manager_and_renders_commands() {
    // Arrange
    let mut manager = FakeServiceManager::unmanaged();
    manager.restart_commands =
        vec!["systemctl --user restart open-bitcoin-node.service".to_string()];
    let cli = OperatorCli::try_parse_from(["open-bitcoin", "service", "restart"])
        .expect("service restart should parse");
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
        &[FakeServiceCall::Restart]
    );
    assert!(
        outcome
            .stdout
            .text
            .contains("systemctl --user restart open-bitcoin-node.service"),
        "stdout should contain restart command: {}",
        outcome.stdout.text
    );
    assert!(
        !outcome.stdout.text.contains("Dry run"),
        "restart should be effectful rather than a dry run: {}",
        outcome.stdout.text
    );
    assert!(
        outcome.stdout.text.contains(
            "Review restart/resume evidence with open-bitcoin status --format json using the same --datadir"
        ),
        "restart output should point to status evidence: {}",
        outcome.stdout.text
    );
}

#[test]
fn execute_service_command_service_preview_calls_install_dry_run() {
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
    assert_eq!(
        outcome.exit_code,
        crate::operator::runtime::OperatorExitCode::Success
    );
    assert_eq!(
        manager.recorded_calls.borrow().as_slice(),
        &[FakeServiceCall::Install { apply: false }]
    );
}
