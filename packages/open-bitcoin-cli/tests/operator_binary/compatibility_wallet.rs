use super::*;

#[test]
fn open_bitcoin_compatibility_harness_writes_json_and_markdown_reports() {
    // Arrange
    let sandbox = TestSandbox::new("compatibility-report");
    let data_dir = sandbox.child("open-data");
    let output_dir = sandbox.child("compatibility");
    fs::create_dir_all(&data_dir).expect("open datadir");

    // Act
    let output = run_open_bitcoin(
        &sandbox,
        [
            "--network",
            "regtest",
            "--format",
            "json",
            "--datadir",
            data_dir.to_str().expect("datadir"),
            "compatibility",
            "harness",
            "--peer-endpoint",
            "203.0.113.10:8333",
            "--scenario",
            "service-bit-mismatch",
            "--output-dir",
            output_dir.to_str().expect("output dir"),
        ],
    );

    // Assert
    assert_success(&output);
    let stdout: Value = serde_json::from_slice(&output.stdout).expect("compatibility stdout json");
    assert_eq!(
        stdout["json_path"],
        json!(
            output_dir
                .join("compatibility-harness-report.json")
                .display()
                .to_string()
        )
    );
    assert_eq!(
        stdout["markdown_path"],
        json!(
            output_dir
                .join("compatibility-harness-report.md")
                .display()
                .to_string()
        )
    );
    let json_text = fs::read_to_string(output_dir.join("compatibility-harness-report.json"))
        .expect("compatibility json");
    let markdown = fs::read_to_string(output_dir.join("compatibility-harness-report.md"))
        .expect("compatibility markdown");
    let decoded: Value = serde_json::from_str(&json_text).expect("compatibility report json");
    assert_eq!(decoded["peer_endpoint"], "203.0.113.10:8333");
    assert_eq!(decoded["network"], "regtest");
    assert_eq!(decoded["scenario"], "service_bit_mismatch");
    assert_eq!(decoded["diagnosis"], "service_bit_mismatch");
    assert_eq!(decoded["failing_step"]["diagnosis"], "service_bit_mismatch");
    assert_eq!(decoded["transcript_summary"]["useful_progress"], false);
    assert_eq!(
        decoded["negotiated_capabilities"]["required_services"],
        json!(["NODE_NETWORK", "NODE_WITNESS"])
    );
    assert!(decoded["redaction_boundaries"]["omitted"].is_array());
    assert!(markdown.contains("# Open Bitcoin Compatibility Harness Report"));
    assert!(markdown.contains("Peer endpoint: 203.0.113.10:8333"));
    assert!(markdown.contains("Network: regtest"));
    assert!(markdown.contains("Diagnosis: service_bit_mismatch"));
    assert!(markdown.contains("Failing step"));
    assert!(markdown.contains("Negotiated capabilities"));
    assert!(markdown.contains("Redaction boundaries"));
    assert!(markdown.contains("raw wire payloads"));
    for rendered in [&json_text, &markdown] {
        assert_absent(rendered, "super-secret-password");
        assert_absent(rendered, "__cookie__:compat-secret");
        assert_absent(rendered, "seed phrase compat-secret");
        assert_absent(rendered, "raw-payload-hex-secret");
        assert_absent(rendered, "daemon stdout tail secret");
        assert_absent(rendered, "daemon stderr tail secret");
    }
}

#[test]
fn open_bitcoin_compatibility_harness_covers_required_diagnosis_scenarios() {
    // Arrange
    let sandbox = TestSandbox::new("compatibility-scenarios");
    let data_dir = sandbox.child("open-data");
    fs::create_dir_all(&data_dir).expect("open datadir");
    let cases = [
        ("compatible", "compatible"),
        ("version-rejected", "version_rejected"),
        ("network-mismatch", "network_mismatch"),
        ("service-bit-mismatch", "service_bit_mismatch"),
        ("unsupported-message-order", "unsupported_message_order"),
        ("timeout", "timeout"),
        ("peer-disconnect", "peer_disconnect"),
        ("malformed-payload", "malformed_payload"),
        ("local-configuration-failure", "local_configuration_failure"),
    ];

    for (scenario, expected_diagnosis) in cases {
        let output_dir = sandbox.child(&format!("compatibility-{scenario}"));

        // Act
        let output = run_open_bitcoin(
            &sandbox,
            [
                "--network",
                "regtest",
                "--datadir",
                data_dir.to_str().expect("datadir"),
                "compatibility",
                "harness",
                "--peer-endpoint",
                "198.51.100.20:8333",
                "--scenario",
                scenario,
                "--output-dir",
                output_dir.to_str().expect("output dir"),
            ],
        );

        // Assert
        assert_success(&output);
        let report_path = output_dir.join("compatibility-harness-report.json");
        let json_text = fs::read_to_string(report_path).expect("compatibility report");
        let decoded: Value = serde_json::from_str(&json_text).expect("compatibility json");
        assert_eq!(decoded["scenario"], expected_diagnosis);
        assert_eq!(decoded["diagnosis"], expected_diagnosis);
    }
}

#[test]
fn open_bitcoin_wallet_send_requires_confirm_and_uses_preview_path() {
    // Arrange
    let sandbox = TestSandbox::new("wallet-send-preview");
    let data_dir = sandbox.child("open-data");
    let server = FakeRpcServer::start();
    seed_managed_wallet(&data_dir, "alpha");
    write_rpc_conf(&data_dir, server.address.port());

    // Act
    let output = run_open_bitcoin(
        &sandbox,
        [
            "--network",
            "regtest",
            "--datadir",
            data_dir.to_str().expect("datadir"),
            "wallet",
            "--wallet",
            "alpha",
            "send",
            "mipcBbFg9gMiCh81Kj8tqqdgoZub1ZJRfn",
            "12000",
            "--fee-rate-sat-per-kvb",
            "2000",
            "--change-descriptor-id",
            "1",
            "--replaceable",
        ],
    );

    // Assert
    assert_failure(&output);
    let stdout = String::from_utf8(output.stdout).expect("stdout utf8");
    let stderr = String::from_utf8(output.stderr).expect("stderr utf8");
    assert!(stdout.contains("Transaction hex:"));
    assert!(stderr.contains("confirmation required"));
    let requests = server.requests();
    assert!(
        requests
            .iter()
            .any(|request| request.contains("POST /wallet/alpha HTTP/1.1"))
    );
    assert!(
        requests
            .iter()
            .any(|request| request.contains("\"buildandsigntransaction\""))
    );
    assert!(
        !requests
            .iter()
            .any(|request| request.contains("\"sendtoaddress\""))
    );
}

#[test]
fn open_bitcoin_wallet_send_confirm_submits_sendtoaddress() {
    // Arrange
    let sandbox = TestSandbox::new("wallet-send-confirm");
    let data_dir = sandbox.child("open-data");
    let server = FakeRpcServer::start();
    seed_managed_wallet(&data_dir, "alpha");
    write_rpc_conf(&data_dir, server.address.port());

    // Act
    let output = run_open_bitcoin(
        &sandbox,
        [
            "--network",
            "regtest",
            "--format",
            "json",
            "--datadir",
            data_dir.to_str().expect("datadir"),
            "wallet",
            "--wallet",
            "alpha",
            "send",
            "mipcBbFg9gMiCh81Kj8tqqdgoZub1ZJRfn",
            "12000",
            "--fee-rate-sat-per-kvb",
            "2000",
            "--change-descriptor-id",
            "1",
            "--replaceable",
            "--confirm",
        ],
    );

    // Assert
    assert_success(&output);
    let decoded: Value = serde_json::from_slice(&output.stdout).expect("submit json");
    assert_eq!(decoded["wallet"], "alpha");
    assert_eq!(decoded["txid"], json!("bb".repeat(32)));
    let requests = server.requests();
    assert!(
        requests
            .iter()
            .any(|request| request.contains("\"buildandsigntransaction\""))
    );
    assert!(
        requests
            .iter()
            .any(|request| request.contains("\"sendtoaddress\""))
    );
    assert!(
        requests
            .iter()
            .all(|request| request.contains("/wallet/alpha"))
    );
}

#[test]
fn open_bitcoin_wallet_backup_writes_open_bitcoin_export() {
    // Arrange
    let sandbox = TestSandbox::new("wallet-backup-write");
    let data_dir = sandbox.child("open-data");
    let backup_dir = sandbox.child("backups");
    let backup_path = backup_dir.join("alpha-backup.json");
    fs::create_dir_all(&backup_dir).expect("backup dir");
    seed_managed_wallet(&data_dir, "alpha");

    // Act
    let output = run_open_bitcoin(
        &sandbox,
        [
            "--network",
            "regtest",
            "--datadir",
            data_dir.to_str().expect("datadir"),
            "wallet",
            "--wallet",
            "alpha",
            "backup",
            backup_path.to_str().expect("backup path"),
        ],
    );

    // Assert
    assert_success(&output);
    let stdout = String::from_utf8(output.stdout).expect("stdout utf8");
    assert!(stdout.contains("Wrote Open Bitcoin wallet backup for alpha"));
    let contents = fs::read_to_string(&backup_path).expect("backup contents");
    let decoded: Value = serde_json::from_str(&contents).expect("backup json");
    assert_eq!(decoded["format"], "open-bitcoin-wallet-backup");
    assert_eq!(decoded["wallet_name"], "alpha");
    assert_eq!(decoded["snapshot"]["network"], "regtest");
    assert_eq!(decoded["snapshot"]["descriptor_count"], 2);
}

#[test]
fn open_bitcoin_wallet_backup_rejects_external_wallet_candidate_paths() {
    // Arrange
    let sandbox = TestSandbox::new("wallet-backup-unsafe");
    let data_dir = sandbox.child("open-data");
    let unsafe_wallet_dir = sandbox.child(".bitcoin/wallets/external");
    let unsafe_backup_path = unsafe_wallet_dir.join("backup.json");
    fs::create_dir_all(&unsafe_wallet_dir).expect("unsafe wallet dir");
    seed_managed_wallet(&data_dir, "alpha");

    // Act
    let output = run_open_bitcoin(
        &sandbox,
        [
            "--network",
            "regtest",
            "--datadir",
            data_dir.to_str().expect("datadir"),
            "wallet",
            "--wallet",
            "alpha",
            "backup",
            unsafe_backup_path.to_str().expect("backup path"),
        ],
    );

    // Assert
    assert_failure(&output);
    let stderr = String::from_utf8(output.stderr).expect("stderr utf8");
    assert!(stderr.contains("backup destination overlaps detected external wallet candidate"));
}
