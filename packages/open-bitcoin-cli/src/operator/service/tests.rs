// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Tests for service lifecycle generators, FakeServiceManager, dry-run safety,
//! and execute_service_command dispatch.

use std::{
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};

use clap::Parser as _;

use crate::operator::{
    OperatorCli, ServiceCommand,
    service::{
        ServiceCommandOutcome, ServiceError, ServiceInstallRequest, ServiceLifecycleState,
        ServiceManager, ServiceRestartRequest, ServiceStartRequest, ServiceStateSnapshot,
        ServiceStopRequest, execute_service_command,
        fake::{FakeServiceCall, FakeServiceManager},
        launchd::{
            LaunchdAdapter, generate_plist_content, launchd_restart_command, launchd_start_command,
            launchd_stop_command, parse_launchd_data_dir, parse_launchd_disabled_services,
            parse_launchd_log_path,
        },
        service_log_path_from_log_dir,
        systemd::{
            SystemdAdapter, generate_unit_content, parse_systemd_data_dir,
            parse_systemd_enabled_state, parse_systemd_log_path, systemd_restart_command,
            systemd_start_command, systemd_stop_command,
        },
    },
};

static NEXT_TEST_DIRECTORY_ID: AtomicU64 = AtomicU64::new(0);

/// RAII temp directory for isolation in service tests.
struct TestDirectory {
    path: PathBuf,
}

impl TestDirectory {
    fn new(label: &str) -> Self {
        let directory = std::env::temp_dir().join(format!(
            "open-bitcoin-service-tests-{label}-{}",
            NEXT_TEST_DIRECTORY_ID.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir_all(&directory).expect("test directory");
        Self { path: directory }
    }
}

impl Drop for TestDirectory {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path);
    }
}

// --- Plist generator tests ---

fn preview_install_outcome() -> ServiceCommandOutcome {
    ServiceCommandOutcome {
        dry_run: true,
        description: "fake preview".to_string(),
        maybe_file_path: Some(PathBuf::from(
            "/fake/home/Library/LaunchAgents/org.open-bitcoin.node.plist",
        )),
        maybe_file_content: Some("fake generated content".to_string()),
        commands_that_would_run: vec![
            "launchctl enable gui/501/org.open-bitcoin.node".to_string(),
            "launchctl bootstrap gui/501 /fake/home/Library/LaunchAgents/org.open-bitcoin.node.plist"
                .to_string(),
        ],
    }
}

fn first_status_line(snapshot: ServiceStateSnapshot) -> String {
    let manager = FakeServiceManager::new(snapshot);
    let cli = OperatorCli::try_parse_from(["open-bitcoin", "service", "status"])
        .expect("service status should parse");
    let crate::operator::OperatorCommand::Service(service_args) = &cli.command else {
        panic!("expected Service command");
    };

    let outcome = execute_service_command(
        service_args,
        PathBuf::from("/fake/bin/open-bitcoind"),
        PathBuf::from("/fake/datadir"),
        None,
        None,
        &manager,
    );
    assert_eq!(
        outcome.exit_code,
        crate::operator::runtime::OperatorExitCode::Success
    );
    outcome
        .stdout
        .text
        .lines()
        .next()
        .expect("service status output should have a first line")
        .to_string()
}

mod install_commands;
mod manager_commands;
mod status_rendering;
mod templates_and_errors;
