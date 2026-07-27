use super::*;

#[test]
fn resolve_service_daemon_binary_feeds_service_and_dashboard_runtimes() {
    // Arrange
    let source = include_str!("../runtime.rs");

    // Act
    let resolver_call_count = source
        .matches("resolve_service_daemon_binary(&operator_binary_path)")
        .count();

    // Assert
    assert_eq!(
        resolver_call_count, 2,
        "service and dashboard runtimes must both resolve open-bitcoind"
    );
    assert!(source.contains("platform_dashboard_service_runtime(\n                binary_path,"));
}

#[test]
fn status_rejects_removed_watch_flag() {
    // Arrange / Act
    let error = route_cli_invocation("open-bitcoin", &[os("status"), os("--watch")])
        .expect_err("status --watch should be rejected");

    // Assert
    assert!(error.to_string().contains("unexpected argument '--watch'"));
}

#[test]
fn status_attempts_live_rpc_without_implicit_bitcoin_conf_when_cookie_exists() {
    // Arrange
    let sandbox = TestDirectory::new("status-live-no-conf");
    let data_dir = sandbox.child("open-bitcoin");
    fs::create_dir_all(&data_dir).expect("datadir");
    fs::write(data_dir.join(".cookie"), "__cookie__:fixture").expect("cookie");
    let cli = OperatorCli {
        maybe_config_path: None,
        maybe_data_dir: Some(data_dir.clone()),
        maybe_network: Some(NetworkSelection::Regtest),
        format: OperatorOutputFormat::Json,
        no_color: true,
        command: OperatorCommand::Status(StatusArgs {}),
    };

    // Act
    let outcome = super::runtime::execute_operator_cli_with_default_data_dir(cli, data_dir.clone());
    let decoded = decode_operator_json(&outcome);

    // Assert
    assert_eq!(outcome.exit_code, super::runtime::OperatorExitCode::Success);
    assert_eq!(decoded["node"]["state"], "unreachable");
}

#[test]
fn dashboard_reuses_live_rpc_bootstrap_without_implicit_bitcoin_conf() {
    // Arrange
    let sandbox = TestDirectory::new("dashboard-live-no-conf");
    let data_dir = sandbox.child("open-bitcoin");
    fs::create_dir_all(&data_dir).expect("datadir");
    fs::write(data_dir.join(".cookie"), "__cookie__:fixture").expect("cookie");
    let cli = OperatorCli {
        maybe_config_path: None,
        maybe_data_dir: Some(data_dir.clone()),
        maybe_network: Some(NetworkSelection::Regtest),
        format: OperatorOutputFormat::Json,
        no_color: true,
        command: OperatorCommand::Dashboard(DashboardArgs { tick_ms: 1_000 }),
    };

    // Act
    let outcome = super::runtime::execute_operator_cli_with_default_data_dir(cli, data_dir.clone());
    let decoded = decode_operator_json(&outcome);

    // Assert
    assert_eq!(outcome.exit_code, super::runtime::OperatorExitCode::Success);
    assert_eq!(decoded["node"]["state"], "unreachable");
}

#[test]
fn status_stays_stopped_when_configless_bootstrap_has_no_credentials() {
    // Arrange
    let sandbox = TestDirectory::new("status-stopped-no-credentials");
    let data_dir = sandbox.child("open-bitcoin");
    fs::create_dir_all(&data_dir).expect("datadir");
    let cli = OperatorCli {
        maybe_config_path: None,
        maybe_data_dir: Some(data_dir.clone()),
        maybe_network: Some(NetworkSelection::Regtest),
        format: OperatorOutputFormat::Json,
        no_color: true,
        command: OperatorCommand::Status(StatusArgs {}),
    };

    // Act
    let outcome = super::runtime::execute_operator_cli_with_default_data_dir(cli, data_dir.clone());
    let decoded = decode_operator_json(&outcome);

    // Assert
    assert_eq!(outcome.exit_code, super::runtime::OperatorExitCode::Success);
    assert_eq!(decoded["node"]["state"], "stopped");
    let bootstrap_warning = decoded["health_signals"]
        .as_array()
        .expect("health signals")
        .iter()
        .find(|signal| signal["source"] == "live_rpc_bootstrap")
        .expect("bootstrap warning");
    assert!(
        bootstrap_warning["message"]
            .as_str()
            .expect("bootstrap warning")
            .contains("live RPC was not attempted")
    );
    assert!(
        bootstrap_warning["message"]
            .as_str()
            .expect("bootstrap warning")
            .contains("bitcoin.conf")
    );
}

#[test]
fn status_human_output_surfaces_bootstrap_warning_before_status_fields() {
    // Arrange
    let sandbox = TestDirectory::new("status-human-bootstrap-warning");
    let data_dir = sandbox.child("open-bitcoin");
    fs::create_dir_all(&data_dir).expect("datadir");
    let cli = OperatorCli {
        maybe_config_path: None,
        maybe_data_dir: Some(data_dir.clone()),
        maybe_network: Some(NetworkSelection::Regtest),
        format: OperatorOutputFormat::Human,
        no_color: true,
        command: OperatorCommand::Status(StatusArgs {}),
    };

    // Act
    let outcome = super::runtime::execute_operator_cli_with_default_data_dir(cli, data_dir);
    let lines = outcome.stdout.text.lines().collect::<Vec<_>>();

    // Assert
    assert_eq!(outcome.exit_code, super::runtime::OperatorExitCode::Success);
    assert!(
        lines
            .first()
            .expect("warning line")
            .starts_with("Warnings: "),
        "expected warning line first, got {:?}",
        lines.first()
    );
    assert!(lines[0].contains("live_rpc_bootstrap"));
    assert!(lines[0].contains("live RPC was not attempted"));
    assert!(
        lines
            .iter()
            .position(|line| line.starts_with("Warnings: "))
            .expect("warning line")
            < lines
                .iter()
                .position(|line| line.starts_with("Daemon: "))
                .expect("daemon line")
    );
}
