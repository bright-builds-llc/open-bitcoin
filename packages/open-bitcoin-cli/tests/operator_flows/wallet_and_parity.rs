use super::*;

#[test]
fn descriptor_rescan_balance_build_sign_and_send_roundtrip() {
    // Arrange
    let sandbox = TestSandbox::new("roundtrip");
    let server = RpcTestServer::start(operator_context());
    let descriptor_requests = serde_json::to_string(&json!([
        {
            "desc": "wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi)",
            "label": "receive",
            "internal": false,
            "timestamp": 0,
        },
        {
            "desc": "sh(wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi))",
            "label": "change",
            "internal": true,
            "timestamp": 0,
        }
    ]))
    .expect("descriptor requests");
    let recipients = serde_json::to_string(&json!([
        {
            "script_pubkey_hex": encode_hex(p2sh_script().as_bytes()),
            "amount_sats": 30_000,
        }
    ]))
    .expect("recipients");

    // Act
    let import_output = run_cli_with_rpc(
        &server,
        &sandbox,
        &[
            "-named".to_string(),
            "importdescriptors".to_string(),
            format!("requests={descriptor_requests}"),
        ],
    );
    let rescan_output = run_cli_with_rpc(&server, &sandbox, &["rescanblockchain".to_string()]);
    let balances_output = run_cli_with_rpc(&server, &sandbox, &["getbalances".to_string()]);
    let unspent_output = run_cli_with_rpc(&server, &sandbox, &["listunspent".to_string()]);
    let import_json = assert_success_json(&import_output);
    let rescan_json = assert_success_json(&rescan_output);
    let balances_json = assert_success_json(&balances_output);
    let unspent_json = assert_success_json(&unspent_output);
    let build_output = run_cli_with_rpc(
        &server,
        &sandbox,
        &[
            "-named".to_string(),
            "buildandsigntransaction".to_string(),
            format!("recipients={recipients}"),
            "fee_rate_sat_per_kvb=2000".to_string(),
            "replaceable=true".to_string(),
        ],
    );
    let build_json = assert_success_json(&build_output);
    let transaction_hex = build_json["transaction_hex"]
        .as_str()
        .expect("transaction hex")
        .to_string();
    let send_output = run_cli_with_rpc(
        &server,
        &sandbox,
        &[
            "-named".to_string(),
            "sendrawtransaction".to_string(),
            format!(
                "hexstring={}",
                serde_json::to_string(&transaction_hex).expect("quoted hex"),
            ),
        ],
    );

    // Assert
    assert_eq!(import_json["results"][0]["success"], json!(true));
    assert_eq!(import_json["results"][1]["success"], json!(true));

    assert_eq!(rescan_json["start_height"], json!(0));
    assert_eq!(rescan_json["stop_height"], json!(1));

    assert_eq!(balances_json["mine"]["trusted_sats"], json!(75_000));

    assert_eq!(unspent_json["entries"][0]["amount_sats"], json!(75_000));

    assert_eq!(build_json["fee_sats"], json!(286));
    assert_eq!(build_json["inputs"][0]["amount_sats"], json!(75_000));

    let send_json = assert_success_json(&send_output);
    let txid_hex = send_json["txid_hex"].as_str().expect("txid hex");
    assert_eq!(txid_hex.len(), 64);
}

#[test]
fn remaining_deferred_surfaces_fail_explicitly() {
    // Arrange
    let sandbox = TestSandbox::new("deferred");

    // Act
    let netinfo_output = run_raw_cli(&sandbox, &["-netinfo".to_string()]);

    // Assert
    assert_eq!(netinfo_output.status.code(), Some(1));
    assert_eq!(
        stderr_text(&netinfo_output).trim(),
        "-netinfo is deferred until the getpeerinfo-backed network dashboard lands in a later Phase 8 plan.",
    );
}

#[test]
fn open_stdin_does_not_block_cli_when_stdin_flags_are_absent() {
    // Arrange
    let sandbox = TestSandbox::new("open-stdin");
    let cli_binary = env!("CARGO_BIN_EXE_open-bitcoin-cli");
    let warmup_status = Command::new(cli_binary)
        .env("HOME", &sandbox.home)
        .arg("-rpcport=notaport")
        .arg("getnetworkinfo")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .expect("warm cli binary before timing stdin behavior");
    assert_eq!(warmup_status.code(), Some(1));

    let mut command = Command::new(cli_binary);
    command
        .env("HOME", &sandbox.home)
        .arg("-rpcport=notaport")
        .arg("getnetworkinfo")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = command.spawn().expect("spawn cli");
    let _stdin_guard = child.stdin.take().expect("stdin pipe");
    let deadline = Instant::now() + Duration::from_secs(10);

    // Act
    let status = loop {
        if let Some(status) = child.try_wait().expect("poll cli") {
            break status;
        }
        if Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            panic!("open stdin pipe kept no-stdin CLI invocation running");
        }
        thread::sleep(Duration::from_millis(10));
    };
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    child
        .stdout
        .take()
        .expect("stdout pipe")
        .read_to_end(&mut stdout)
        .expect("stdout");
    child
        .stderr
        .take()
        .expect("stderr pipe")
        .read_to_end(&mut stderr)
        .expect("stderr");

    // Assert
    assert_eq!(status.code(), Some(1));
    assert!(stdout.is_empty());
    assert_eq!(
        String::from_utf8(stderr).expect("stderr text"),
        "Invalid port provided in -rpcport: notaport\n",
    );
}

#[test]
fn parity_catalog_entry_is_tracked() {
    // Arrange
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..");

    // Act
    let catalog = fs::read_to_string(repo_root.join("docs/parity/catalog/rpc-cli-config.md"))
        .expect("rpc-cli-config catalog");
    let index = fs::read_to_string(repo_root.join("docs/parity/index.json")).expect("parity index");

    // Assert
    assert!(catalog.contains("buildtransaction"));
    assert!(catalog.contains("buildandsigntransaction"));
    assert!(catalog.contains("sendtoaddress"));
    assert!(catalog.contains("rpcauth"));
    assert!(catalog.contains("rpcwhitelist"));
    assert!(catalog.contains("rpcwallet"));
    assert!(catalog.contains("getpeerinfo"));
    assert!(catalog.contains("-netinfo"));
    assert!(index.contains("rpc-cli-config"));
}

#[test]
fn migration_deviation_notices_match_parity_index() {
    // Arrange
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..");

    // Act
    let index = fs::read_to_string(repo_root.join("docs/parity/index.json")).expect("parity index");
    let notices = migration_deviation_definitions();

    // Assert
    assert!(index.contains("drop-in-audit-migration"));
    for notice in notices {
        assert!(
            index.contains(&format!("\"id\": \"{}\"", notice.id)),
            "missing deviation id {} in parity index",
            notice.id
        );
        assert!(
            index.contains(&notice.summary),
            "missing deviation summary {} in parity index",
            notice.id
        );
    }
}
