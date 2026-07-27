use super::*;

#[test]
fn status_command_parses_json_output() {
    // Arrange
    let argv = ["open-bitcoin", "status", "--format", "json"];

    // Act
    let parsed = OperatorCli::try_parse_from(argv).expect("operator cli should parse");

    // Assert
    assert_eq!(parsed.format, OperatorOutputFormat::Json);
    assert!(matches!(parsed.command, OperatorCommand::Status(_)));
}

#[test]
fn open_bitcoin_cli_routes_to_compat() {
    // Arrange
    let args = vec![os("-named"), os("getnetworkinfo")];

    // Act
    let route = route_cli_invocation("open-bitcoin-cli", &args).expect("route");

    // Assert
    assert_eq!(route, CliRoute::BitcoinCliCompat(args));
}

#[test]
fn open_bitcoin_routes_to_operator() {
    // Arrange
    let args = vec![os("config"), os("paths")];

    // Act
    let route = route_cli_invocation("open-bitcoin", &args).expect("route");

    // Assert
    let CliRoute::Operator(cli) = route else {
        panic!("expected operator route");
    };
    let OperatorCommand::Config(config) = cli.command else {
        panic!("expected config command");
    };
    assert_eq!(config.command, ConfigCommand::Paths);
}

#[test]
fn open_bitcoin_status_routes_to_operator_status() {
    // Arrange
    let args = vec![os("status")];

    // Act
    let route = route_cli_invocation("open-bitcoin", &args).expect("route");

    // Assert
    let CliRoute::Operator(cli) = route else {
        panic!("expected operator route");
    };
    assert!(matches!(cli.command, OperatorCommand::Status(_)));
}

#[test]
fn open_bitcoin_sync_pause_routes_to_operator_sync() {
    // Arrange
    let args = vec![os("sync"), os("pause")];

    // Act
    let route = route_cli_invocation("open-bitcoin", &args).expect("route");

    // Assert
    let CliRoute::Operator(cli) = route else {
        panic!("expected operator route");
    };
    let OperatorCommand::Sync(sync) = cli.command else {
        panic!("expected sync command");
    };
    assert_eq!(sync.command, SyncCommand::Pause);
}

#[test]
fn soak_cli_start_parses_bounded_operator_contract() {
    // Arrange
    let argv = [
        "open-bitcoin",
        "--datadir",
        "/tmp/node",
        "--network",
        "regtest",
        "soak",
        "start",
        "--elapsed-time-seconds",
        "60",
        "--checkpoint-interval-seconds",
        "15",
        "--target-height",
        "144",
        "--peer-policy",
        "daemon-configured",
        "--disk-budget-bytes",
        "1048576",
        "--stop-condition",
        "elapsed-time",
    ];

    // Act
    let parsed = OperatorCli::try_parse_from(argv).expect("soak start cli");

    // Assert
    assert_eq!(
        parsed.maybe_data_dir.as_deref(),
        Some(std::path::Path::new("/tmp/node"))
    );
    assert_eq!(parsed.maybe_network, Some(NetworkSelection::Regtest));
    let OperatorCommand::Soak(soak) = parsed.command else {
        panic!("expected soak command");
    };
    let SoakCommand::Start(start) = soak.command else {
        panic!("expected soak start command");
    };
    assert_eq!(start.elapsed_time_seconds, 60);
    assert_eq!(start.checkpoint_interval_seconds, 15);
    assert_eq!(start.maybe_target_height, Some(144));
    assert_eq!(start.peer_policy, SoakPeerPolicyArg::DaemonConfigured);
    assert_eq!(start.disk_budget_bytes, 1_048_576);
    assert_eq!(start.stop_condition, SoakStopConditionArg::ElapsedTime);
}

#[test]
fn soak_cli_rejects_zero_bounds_and_missing_stop_condition() {
    // Arrange
    let zero_elapsed = [
        "open-bitcoin",
        "soak",
        "start",
        "--elapsed-time-seconds",
        "0",
        "--checkpoint-interval-seconds",
        "15",
        "--peer-policy",
        "daemon-configured",
        "--disk-budget-bytes",
        "1048576",
        "--stop-condition",
        "elapsed-time",
    ];
    let zero_checkpoint = [
        "open-bitcoin",
        "soak",
        "start",
        "--elapsed-time-seconds",
        "60",
        "--checkpoint-interval-seconds",
        "0",
        "--peer-policy",
        "daemon-configured",
        "--disk-budget-bytes",
        "1048576",
        "--stop-condition",
        "elapsed-time",
    ];
    let zero_disk = [
        "open-bitcoin",
        "soak",
        "start",
        "--elapsed-time-seconds",
        "60",
        "--checkpoint-interval-seconds",
        "15",
        "--peer-policy",
        "daemon-configured",
        "--disk-budget-bytes",
        "0",
        "--stop-condition",
        "elapsed-time",
    ];
    let missing_stop_condition = [
        "open-bitcoin",
        "soak",
        "start",
        "--elapsed-time-seconds",
        "60",
        "--checkpoint-interval-seconds",
        "15",
        "--peer-policy",
        "daemon-configured",
        "--disk-budget-bytes",
        "1048576",
    ];

    // Act
    let results = [
        OperatorCli::try_parse_from(zero_elapsed),
        OperatorCli::try_parse_from(zero_checkpoint),
        OperatorCli::try_parse_from(zero_disk),
        OperatorCli::try_parse_from(missing_stop_condition),
    ];

    // Assert
    assert!(results.into_iter().all(|result| result.is_err()));
}

#[test]
fn soak_cli_resume_stop_and_report_parse_run_id_contract() {
    // Arrange
    let resume_argv = [
        "open-bitcoin",
        "soak",
        "resume",
        "--run-id",
        "soak-1700000000-0001",
        "--checkpoint-interval-seconds",
        "15",
    ];
    let stop_argv = [
        "open-bitcoin",
        "soak",
        "stop",
        "--run-id",
        "soak-1700000000-0001",
        "--reason",
        "operator-stop",
    ];
    let report_argv = [
        "open-bitcoin",
        "soak",
        "report",
        "--run-id",
        "soak-1700000000-0001",
    ];

    // Act
    let resume = OperatorCli::try_parse_from(resume_argv).expect("soak resume cli");
    let stop = OperatorCli::try_parse_from(stop_argv).expect("soak stop cli");
    let report = OperatorCli::try_parse_from(report_argv).expect("soak report cli");

    // Assert
    let OperatorCommand::Soak(resume) = resume.command else {
        panic!("expected soak resume");
    };
    let SoakCommand::Resume(resume) = resume.command else {
        panic!("expected soak resume command");
    };
    assert_eq!(resume.run_id, "soak-1700000000-0001");
    assert_eq!(resume.checkpoint_interval_seconds, 15);

    let OperatorCommand::Soak(stop) = stop.command else {
        panic!("expected soak stop");
    };
    let SoakCommand::Stop(stop) = stop.command else {
        panic!("expected soak stop command");
    };
    assert_eq!(stop.run_id, "soak-1700000000-0001");
    assert_eq!(stop.reason, SoakStopReasonArg::OperatorStop);

    let OperatorCommand::Soak(report) = report.command else {
        panic!("expected soak report");
    };
    let SoakCommand::Report(report) = report.command else {
        panic!("expected soak report command");
    };
    assert_eq!(report.run_id, "soak-1700000000-0001");
}

#[test]
fn open_bitcoin_dashboard_routes_to_operator_dashboard() {
    // Arrange
    let args = vec![os("dashboard"), os("--tick-ms"), os("500")];

    // Act
    let route = route_cli_invocation("open-bitcoin", &args).expect("route");

    // Assert
    let CliRoute::Operator(cli) = route else {
        panic!("expected operator route");
    };
    let OperatorCommand::Dashboard(dashboard) = cli.command else {
        panic!("expected dashboard command");
    };
    assert_eq!(dashboard.tick_ms, 500);
}

#[test]
fn service_preview_routes_to_operator_service_preview() {
    // Arrange
    let args = vec![os("service"), os("preview")];

    // Act
    let route = route_cli_invocation("open-bitcoin", &args).expect("route");

    // Assert
    let CliRoute::Operator(cli) = route else {
        panic!("expected operator route");
    };
    let OperatorCommand::Service(service) = cli.command else {
        panic!("expected service command");
    };
    assert_eq!(service.command, ServiceCommand::Preview);
    assert!(
        !service.apply,
        "preview should be side-effect-free by default"
    );
}

#[test]
fn open_bitcoin_migrate_plan_routes_to_operator_command() {
    // Arrange
    let args = vec![
        os("migrate"),
        os("plan"),
        os("--source-datadir"),
        os("/tmp/core"),
    ];

    // Act
    let route = route_cli_invocation("open-bitcoin", &args).expect("route");

    // Assert
    let CliRoute::Operator(cli) = route else {
        panic!("expected operator route");
    };
    let OperatorCommand::Migrate(migration) = cli.command else {
        panic!("expected migrate command");
    };
    let MigrationCommand::Plan(plan) = migration.command;
    assert_eq!(
        plan.maybe_source_data_dir.as_deref(),
        Some(std::path::Path::new("/tmp/core"))
    );
}

#[test]
fn open_bitcoin_support_bundle_routes_to_operator_command() {
    // Arrange
    let args = vec![
        os("support"),
        os("bundle"),
        os("--output-dir"),
        os("/tmp/open-bitcoin-support"),
        os("--include-live-smoke-report"),
        os("/tmp/live-smoke.json"),
    ];

    // Act
    let route = route_cli_invocation("open-bitcoin", &args).expect("route");

    // Assert
    let CliRoute::Operator(cli) = route else {
        panic!("expected operator route");
    };
    let OperatorCommand::Support(support) = cli.command else {
        panic!("expected support command");
    };
    let SupportCommand::Bundle(bundle) = support.command;
    assert_eq!(
        bundle.maybe_output_dir.as_deref(),
        Some(std::path::Path::new("/tmp/open-bitcoin-support"))
    );
    assert_eq!(
        bundle.maybe_live_smoke_report.as_deref(),
        Some(std::path::Path::new("/tmp/live-smoke.json"))
    );
}

#[test]
fn operator_config_sources_follow_rpc_precedence_order() {
    // Arrange
    let rpc_sources = ConfigPrecedence::ordered_sources();

    // Act
    let operator_sources: Vec<_> = rpc_sources
        .into_iter()
        .map(operator_source_from_rpc)
        .collect();
    let operator_names: Vec<_> = operator_sources
        .iter()
        .map(|source| source.as_str())
        .collect();

    // Assert
    assert_eq!(operator_sources, OperatorConfigSource::ordered());
    assert_eq!(
        operator_names,
        vec![
            "cli_flags",
            "environment",
            "open_bitcoin_jsonc",
            "bitcoin_conf",
            "cookies",
            "defaults",
        ]
    );
}

#[test]
fn onboarding_write_decision_contract_covers_all_write_states() {
    // Arrange
    let proposed = ProposedConfigWrite {
        path: "/tmp/open-bitcoin.jsonc".into(),
        contents: "{ \"schema_version\": 1 }".to_string(),
        replaces_existing: false,
    };
    let decisions = [
        OnboardingWriteDecision::NoWrite {
            reason: "not approved".to_string(),
        },
        OnboardingWriteDecision::ProposedWrite {
            write: proposed.clone(),
        },
        OnboardingWriteDecision::ApprovedWrite { write: proposed },
    ];

    // Act
    let labels: Vec<_> = decisions
        .iter()
        .map(|decision| match decision {
            OnboardingWriteDecision::NoWrite { .. } => "no_write",
            OnboardingWriteDecision::ProposedWrite { .. } => "proposed_write",
            OnboardingWriteDecision::ApprovedWrite { .. } => "approved_write",
        })
        .collect();

    // Assert
    assert_eq!(labels, vec!["no_write", "proposed_write", "approved_write"]);
}

#[test]
fn status_contract_uses_shared_status_snapshot_without_renderer_dto() {
    // Arrange
    let source = include_str!("../status.rs");

    // Assert
    assert!(!source.contains("StatusJson"));
    assert!(!source.contains("StatusDto"));
    assert!(!source.contains("CliStatusSnapshot"));
    assert!(source.contains("OpenBitcoinStatusSnapshot"));
}

#[test]
fn dashboard_command_is_no_longer_deferred_in_runtime() {
    // Arrange
    let source = include_str!("../runtime.rs");

    // Assert
    assert!(!source.contains("dashboard command is deferred to Phase 19"));
    assert!(source.contains("run_dashboard"));
}

#[test]
fn resolve_service_daemon_binary_uses_materialized_sibling() {
    // Arrange
    let sandbox = TestDirectory::new("service-daemon-sibling");
    let bin_dir = sandbox.child("bin");
    fs::create_dir_all(&bin_dir).expect("bin directory");
    let operator_binary_path = bin_dir.join("open-bitcoin");
    let daemon_binary_path = bin_dir.join("open-bitcoind");
    fs::write(&daemon_binary_path, "").expect("daemon sibling");

    // Act
    let resolved = super::runtime::resolve_service_daemon_binary(&operator_binary_path);

    // Assert
    assert_eq!(resolved, daemon_binary_path);
}

#[test]
fn resolve_service_daemon_binary_falls_back_to_literal_command_without_sibling() {
    // Arrange
    let sandbox = TestDirectory::new("service-daemon-fallback");
    let operator_binary_path = sandbox.child("bin/open-bitcoin");

    // Act
    let resolved = super::runtime::resolve_service_daemon_binary(&operator_binary_path);

    // Assert
    assert_eq!(resolved, PathBuf::from("open-bitcoind"));
}
