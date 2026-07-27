// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::*;

#[test]
fn fake_rpc_server_responses_declare_connection_close() {
    // Arrange
    let server = FakeRpcServer::start();
    let request_body = r#"{"jsonrpc":"2.0","method":"getblockchaininfo","params":[],"id":1}"#;
    let request = format!(
        "POST / HTTP/1.1\r\nHost: 127.0.0.1\r\nContent-Length: {}\r\n\r\n{}",
        request_body.len(),
        request_body
    );
    let mut stream = TcpStream::connect(server.address).expect("connect fake rpc");
    stream
        .set_read_timeout(Some(Duration::from_secs(2)))
        .expect("read timeout");

    // Act
    stream.write_all(request.as_bytes()).expect("write request");
    let mut response = String::new();
    stream
        .read_to_string(&mut response)
        .expect("read fake rpc response");

    // Assert
    assert!(
        response.contains("\r\nConnection: close\r\n"),
        "response did not declare connection close: {response}"
    );
}

#[test]
fn open_bitcoin_status_human_no_color_is_support_oriented() {
    // Arrange
    let sandbox = TestSandbox::new("human");
    let data_dir = sandbox.child("open-data");
    fs::create_dir_all(&data_dir).expect("open datadir");

    // Act
    let output = run_open_bitcoin(
        &sandbox,
        [
            "--datadir",
            data_dir.to_str().expect("datadir"),
            "status",
            "--format",
            "human",
            "--no-color",
        ],
    );

    // Assert
    assert_success(&output);
    let stdout = String::from_utf8(output.stdout).expect("stdout utf8");
    for label in [
        "Daemon:", "Version:", "Datadir:", "Config:", "Network:", "Chain:", "Sync:", "Peers:",
        "Mempool:", "Wallet:", "Service:", "Logs:", "Metrics:", "Health:",
    ] {
        assert!(stdout.contains(label), "missing {label}");
    }
    assert!(!stdout.contains("\u{1b}["));
}

#[test]
fn open_bitcoin_dashboard_json_is_snapshot_and_ansi_free() {
    // Arrange
    let sandbox = TestSandbox::new("dashboard-json");
    let data_dir = sandbox.child("open-data");
    fs::create_dir_all(&data_dir).expect("open datadir");

    // Act
    let output = run_open_bitcoin(
        &sandbox,
        [
            "--datadir",
            data_dir.to_str().expect("datadir"),
            "--format",
            "json",
            "dashboard",
        ],
    );

    // Assert
    assert_success(&output);
    let decoded: Value = serde_json::from_slice(&output.stdout).expect("dashboard json");
    assert_eq!(decoded["node"]["state"], "stopped");
    assert_eq!(decoded["metrics"]["samples"], json!([]));
    let rendered = String::from_utf8(output.stdout).expect("stdout utf8");
    assert!(!rendered.contains("\u{1b}["));
    assert!(!rendered.contains("dashboard command is deferred"));
}

#[test]
fn open_bitcoin_dashboard_human_non_tty_uses_snapshot_sections() {
    // Arrange
    let sandbox = TestSandbox::new("dashboard-human");
    let data_dir = sandbox.child("open-data");
    fs::create_dir_all(&data_dir).expect("open datadir");

    // Act
    let output = run_open_bitcoin(
        &sandbox,
        [
            "--datadir",
            data_dir.to_str().expect("datadir"),
            "--format",
            "human",
            "--no-color",
            "dashboard",
        ],
    );

    // Assert
    assert_success(&output);
    let stdout = String::from_utf8(output.stdout).expect("stdout utf8");
    for label in [
        "Open Bitcoin Dashboard",
        "## Node",
        "## Sync and Peers",
        "## Mempool and Wallet",
        "## Service",
        "## Logs and Health",
        "## Charts",
        "## Actions",
    ] {
        assert!(stdout.contains(label), "missing {label}");
    }
    assert!(!stdout.contains("\u{1b}["));
    assert!(!stdout.contains("dashboard command is deferred"));
}

#[test]
fn open_bitcoin_onboard_non_interactive_is_idempotent() {
    // Arrange
    let sandbox = TestSandbox::new("onboard");
    let data_dir = sandbox.child("open-data");
    let config_path = data_dir.join("open-bitcoin.jsonc");
    let core_dir = sandbox.child(".bitcoin");
    fs::create_dir_all(&core_dir).expect("core datadir");
    fs::write(core_dir.join("bitcoin.conf"), "regtest=1\n").expect("core config");

    // Act
    let first = run_open_bitcoin_vec(&sandbox, onboard_args(&data_dir, &config_path, &[]));
    let first_contents = fs::read_to_string(&config_path).expect("first config");
    let second = run_open_bitcoin_vec(&sandbox, onboard_args(&data_dir, &config_path, &[]));
    let second_contents = fs::read_to_string(&config_path).expect("second config");
    let forced = run_open_bitcoin_vec(
        &sandbox,
        onboard_args(&data_dir, &config_path, &["--force-overwrite"]),
    );

    // Assert
    assert_success(&first);
    assert_success(&second);
    assert_success(&forced);
    assert_eq!(first_contents, second_contents);
    assert!(first_contents.contains("\"onboarding\""));
    assert!(first_contents.contains("\"wizard_answers\""));
    assert!(first_contents.contains("\"network\""));
    assert!(first_contents.contains("\"datadir\""));
    assert!(first_contents.contains("\"metrics\""));
    assert!(first_contents.contains("\"logs\""));
    assert!(first_contents.contains("\"migration\""));
    let first_stdout = String::from_utf8(first.stdout).expect("stdout utf8");
    let second_stdout = String::from_utf8(second.stdout).expect("stdout utf8");
    assert!(first_stdout.contains(core_dir.join("bitcoin.conf").to_str().expect("core path")));
    assert!(first_stdout.contains("confidence="));
    assert!(second_stdout.contains("left unchanged"));
    assert!(!data_dir.join("bitcoin.conf").exists());
}

#[test]
fn open_bitcoin_migrate_plan_is_dry_run_only_for_detected_source_install() {
    // Arrange
    let sandbox = TestSandbox::new("migrate-plan");
    let target_data_dir = sandbox.child("open-data");
    let source_data_dir = sandbox.child(".bitcoin");
    let source_wallet_dir = source_data_dir.join("wallets/main");
    fs::create_dir_all(&source_wallet_dir).expect("source wallet dir");
    fs::write(source_data_dir.join("bitcoin.conf"), "regtest=1\n").expect("source config");
    fs::write(source_data_dir.join(".cookie"), "__cookie__:secret\n").expect("source cookie");
    fs::write(source_wallet_dir.join("wallet.dat"), "legacy wallet bytes").expect("source wallet");

    #[cfg(target_os = "macos")]
    let source_service_path = {
        let path = sandbox.child("Library/LaunchAgents/org.bitcoin.bitcoind.plist");
        fs::create_dir_all(path.parent().expect("launchagents parent")).expect("launchagents");
        fs::write(
            &path,
            format!(
                "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">\n\
<plist version=\"1.0\">\n\
<dict>\n\
    <key>Label</key>\n\
    <string>org.bitcoin.bitcoind</string>\n\
    <key>ProgramArguments</key>\n\
    <array>\n\
        <string>/usr/local/bin/bitcoind</string>\n\
        <string>-conf</string>\n\
        <string>{}</string>\n\
        <string>-datadir</string>\n\
        <string>{}</string>\n\
    </array>\n\
    <key>RunAtLoad</key>\n\
    <true/>\n\
</dict>\n\
</plist>\n",
                source_data_dir.join("bitcoin.conf").display(),
                source_data_dir.display()
            ),
        )
        .expect("launchd service");
        path
    };

    #[cfg(target_os = "linux")]
    let source_service_path = {
        let path = sandbox.child(".config/systemd/user/bitcoind.service");
        fs::create_dir_all(path.parent().expect("systemd parent")).expect("systemd");
        fs::write(
            &path,
            format!(
                "[Service]\nExecStart=/usr/bin/bitcoind -conf={} -datadir={}\n",
                source_data_dir.join("bitcoin.conf").display(),
                source_data_dir.display()
            ),
        )
        .expect("systemd service");
        path
    };

    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    let source_service_path = {
        let path = sandbox.child("services/bitcoind.service");
        fs::create_dir_all(path.parent().expect("service parent")).expect("service dir");
        fs::write(
            &path,
            format!(
                "[Service]\nExecStart=/usr/bin/bitcoind -conf={} -datadir={}\n",
                source_data_dir.join("bitcoin.conf").display(),
                source_data_dir.display()
            ),
        )
        .expect("service file");
        path
    };

    let before_config = fs::read(source_data_dir.join("bitcoin.conf")).expect("before config");
    let before_cookie = fs::read(source_data_dir.join(".cookie")).expect("before cookie");
    let before_wallet = fs::read(source_wallet_dir.join("wallet.dat")).expect("before wallet");

    // Act
    let output = run_open_bitcoin(
        &sandbox,
        [
            "--network",
            "regtest",
            "--no-color",
            "--datadir",
            target_data_dir.to_str().expect("target datadir"),
            "migrate",
            "plan",
            "--source-datadir",
            source_data_dir.to_str().expect("source datadir"),
        ],
    );

    // Assert
    assert_success(&output);
    let stdout = String::from_utf8(output.stdout).expect("stdout utf8");
    assert!(stdout.contains("Migration plan (dry run only)"));
    assert!(stdout.contains("Benefits:"));
    assert!(stdout.contains("Backup requirements:"));
    assert!(
        stdout.contains(
            source_data_dir
                .join("bitcoin.conf")
                .to_str()
                .expect("config path")
        )
    );
    assert!(stdout.contains(source_service_path.to_str().expect("service path")));
    assert!(
        stdout.contains(
            source_wallet_dir
                .join("wallet.dat")
                .to_str()
                .expect("wallet path")
        )
    );
    assert!(stdout.contains("mig-dry-run-only-switch-over"));
    assert!(!stdout.contains("__cookie__:secret"));
    assert!(!stdout.contains("legacy wallet bytes"));
    assert_eq!(
        fs::read(source_data_dir.join("bitcoin.conf")).expect("after config"),
        before_config
    );
    assert_eq!(
        fs::read(source_data_dir.join(".cookie")).expect("after cookie"),
        before_cookie
    );
    assert_eq!(
        fs::read(source_wallet_dir.join("wallet.dat")).expect("after wallet"),
        before_wallet
    );
}

#[test]
fn open_bitcoin_migrate_plan_selects_explicit_custom_source_outside_default_roots() {
    // Arrange
    let sandbox = TestSandbox::new("migrate-custom-source");
    let target_data_dir = sandbox.child("open-data");
    let source_data_dir = sandbox.child("custom-source/bitcoin-core-datadir");
    let source_wallet_dir = source_data_dir.join("wallets/main");
    fs::create_dir_all(&source_wallet_dir).expect("source wallet dir");
    fs::write(source_data_dir.join("bitcoin.conf"), "regtest=1\n").expect("source config");
    fs::write(source_data_dir.join(".cookie"), "__cookie__:secret\n").expect("source cookie");
    fs::write(source_wallet_dir.join("wallet.dat"), "legacy wallet bytes").expect("source wallet");

    #[cfg(target_os = "macos")]
    let source_service_path = {
        let path = sandbox.child("Library/LaunchAgents/org.bitcoin.bitcoind.plist");
        fs::create_dir_all(path.parent().expect("launchagents parent")).expect("launchagents");
        fs::write(&path, "<plist></plist>\n").expect("launchd service");
        path
    };

    #[cfg(target_os = "linux")]
    let source_service_path = {
        let path = sandbox.child(".config/systemd/user/bitcoind.service");
        fs::create_dir_all(path.parent().expect("systemd parent")).expect("systemd");
        fs::write(&path, "[Service]\nExecStart=bitcoind\n").expect("systemd service");
        path
    };

    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    let source_service_path = {
        let path = sandbox.child("services/bitcoind.service");
        fs::create_dir_all(path.parent().expect("service parent")).expect("service dir");
        fs::write(&path, "service unsupported\n").expect("service file");
        path
    };

    let before_config = fs::read(source_data_dir.join("bitcoin.conf")).expect("before config");
    let before_cookie = fs::read(source_data_dir.join(".cookie")).expect("before cookie");
    let before_wallet = fs::read(source_wallet_dir.join("wallet.dat")).expect("before wallet");

    // Act
    let output = run_open_bitcoin(
        &sandbox,
        [
            "--network",
            "regtest",
            "--no-color",
            "--datadir",
            target_data_dir.to_str().expect("target datadir"),
            "migrate",
            "plan",
            "--source-datadir",
            source_data_dir.to_str().expect("source datadir"),
        ],
    );

    // Assert
    assert_success(&output);
    let stdout = String::from_utf8(output.stdout).expect("stdout utf8");
    assert!(stdout.contains("Migration plan (dry run only)"));
    assert!(stdout.contains(source_data_dir.to_str().expect("source datadir")));
    assert!(
        stdout.contains(
            source_data_dir
                .join("bitcoin.conf")
                .to_str()
                .expect("config path")
        )
    );
    assert!(!stdout.contains(source_service_path.to_str().expect("service path")));
    assert!(stdout.contains("could not be confidently tied to the selected source install"));
    assert!(
        stdout.contains(
            source_wallet_dir
                .join("wallet.dat")
                .to_str()
                .expect("wallet path")
        )
    );
    assert!(!stdout.contains("__cookie__:secret"));
    assert!(!stdout.contains("legacy wallet bytes"));
    assert_eq!(
        fs::read(source_data_dir.join("bitcoin.conf")).expect("after config"),
        before_config
    );
    assert_eq!(
        fs::read(source_data_dir.join(".cookie")).expect("after cookie"),
        before_cookie
    );
    assert_eq!(
        fs::read(source_wallet_dir.join("wallet.dat")).expect("after wallet"),
        before_wallet
    );
}

#[test]
fn open_bitcoin_config_paths_reports_sources() {
    // Arrange
    let sandbox = TestSandbox::new("config-paths");
    let data_dir = sandbox.child("open-data");
    fs::create_dir_all(&data_dir).expect("open datadir");

    // Act
    let output = run_open_bitcoin(
        &sandbox,
        [
            "--datadir",
            data_dir.to_str().expect("datadir"),
            "config",
            "paths",
            "--format",
            "human",
        ],
    );

    // Assert
    assert_success(&output);
    let stdout = String::from_utf8(output.stdout).expect("stdout utf8");
    assert!(stdout.contains("Config:"));
    assert!(stdout.contains("Bitcoin config:"));
    assert!(stdout.contains("Datadir:"));
    assert!(stdout.contains("Logs:"));
    assert!(stdout.contains("Metrics:"));
    assert!(stdout.contains("cli_flags > environment > open_bitcoin_jsonc"));
}

#[test]
fn open_bitcoin_support_bundle_writes_redacted_json_and_markdown() {
    // Arrange
    let sandbox = TestSandbox::new("support-bundle");
    let data_dir = sandbox.child("open-data");
    let output_dir = sandbox.child("support");
    fs::create_dir_all(&data_dir).expect("open datadir");
    let server = FakeRpcServer::start();
    fs::write(
        data_dir.join("bitcoin.conf"),
        format!(
            "regtest=1\nrpcconnect=127.0.0.1\nrpcport={}\nrpcuser=alice\nrpcpassword=super-secret-password\n",
            server.address.port()
        ),
    )
    .expect("bitcoin.conf");
    fs::write(data_dir.join(".cookie"), "__cookie__:super-secret-cookie\n").expect("cookie");

    // Act
    let output = run_open_bitcoin(
        &sandbox,
        [
            "--network",
            "regtest",
            "--datadir",
            data_dir.to_str().expect("datadir"),
            "support",
            "bundle",
            "--output-dir",
            output_dir.to_str().expect("output dir"),
        ],
    );

    // Assert
    assert_success(&output);
    let stdout = String::from_utf8(output.stdout).expect("stdout utf8");
    assert!(stdout.contains("Support evidence written"));
    assert!(stdout.contains("support-evidence.json"));
    assert!(stdout.contains("support-evidence.md"));
    let json_path = output_dir.join("support-evidence.json");
    let markdown_path = output_dir.join("support-evidence.md");
    let json_text = fs::read_to_string(&json_path).expect("support json");
    let markdown = fs::read_to_string(&markdown_path).expect("support markdown");
    let decoded: Value = serde_json::from_str(&json_text).expect("support json");
    assert_eq!(decoded["status"]["node"]["state"], "running");
    assert!(decoded.get("config").is_some());
    assert!(decoded.get("store_health").is_some());
    assert!(decoded.get("redaction").is_some());
    assert_eq!(decoded["live_smoke"]["state"], "unavailable");
    assert!(markdown.contains("# Open Bitcoin Support Evidence"));
    for rendered in [&stdout, &json_text, &markdown] {
        assert_absent(rendered, "super-secret-password");
        assert_absent(rendered, "super-secret-cookie");
    }
}
